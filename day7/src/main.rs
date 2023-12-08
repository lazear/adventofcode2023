use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Card {
    Jack,
    Low(u8),
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    kind: Kind,
    cards: [Card; 5],
    bid: u32,
}

fn parse_hand(line: &str) -> Hand {
    let mut cards = [Card::Low(0); 5];

    for i in 0..5 {
        let card = match line.as_bytes()[i] {
            b'T' => Card::Ten,
            b'J' => Card::Jack,
            b'Q' => Card::Queen,
            b'K' => Card::King,
            b'A' => Card::Ace,
            x if x.is_ascii_digit() => Card::Low(x - b'0'),
            x => panic!("{}", x as char),
        };
        cards[i] = card;
    }
    let bid = line[6..].parse::<u32>().unwrap();
    let mut kind = Kind::High;
    let mut map: HashMap<&Card, u32> = HashMap::new();
    for card in cards.iter() {
        *map.entry(card).or_default() += 1;
    }
    let mut counts = map
        .iter()
        .filter_map(|(c, count)| {
            if Card::Jack == **c {
                None
            } else {
                Some(*count)
            }
        })
        .collect::<Vec<_>>();

    counts.sort_by(|a, b| b.cmp(a));
    if counts.is_empty() {
        counts = map.values().copied().collect();
    } else {
        counts[0] += map.get(&Card::Jack).copied().unwrap_or_default();
    }

    for count in counts {
        kind = match (kind, count) {
            (_, 5) => Kind::Five,
            (_, 4) => Kind::Four,
            (_, 3) => Kind::Three,
            (Kind::Three, 2) => Kind::FullHouse,
            (Kind::OnePair, 2) => Kind::TwoPair,
            (Kind::High, 2) => Kind::OnePair,
            (_, _) => kind,
        }
    }

    Hand { kind, cards, bid }
}

fn main() {
    let mut hands = TEST
        .lines()
        .map(|line| parse_hand(line))
        .collect::<Vec<_>>();
    let mut hands = include_str!("../input")
        .lines()
        .map(|line| parse_hand(line))
        .collect::<Vec<_>>();
    hands.sort_unstable();
    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank as u32 + 1) * hand.bid);

    println!("part1={winnings}");
}

const TEST: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
