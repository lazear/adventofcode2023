#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Alignment {
    blocks: Vec<State>,
    lengths: Vec<usize>,
}

fn parse_one(line: &str) -> Option<Alignment> {
    let (blocks, lengths) = line.split_once(' ')?;
    let lengths = lengths
        .split(',')
        .map(|s| s.parse::<usize>().ok())
        .collect::<Option<Vec<_>>>()?;
    let blocks = blocks
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'.' => State::Operational,
            b'#' => State::Damaged,
            b'?' => State::Unknown,
            _ => unreachable!(),
        })
        .collect();

    Some(Alignment { blocks, lengths })
}

impl Alignment {
    fn can_fit(&self, index: usize, length: usize) -> bool {
        if index + length > self.blocks.len() {
            return false;
        }
        for i in index..(index + length) {
            if self.blocks[i] == State::Operational {
                return false;
            }
        }

        if index + length < self.blocks.len() && self.blocks[index + length] == State::Damaged {
            // damaged spring groups must be separated
            return false;
        }

        if index > 0 && self.blocks[index - 1] == State::Damaged {
            return false;
        }

        true
    }

    /// check final layout
    fn check(&self, layout: &[usize]) -> bool {
        let total_damaged = self.blocks.iter().filter(|x| **x == State::Damaged).count();
        let mut covered = 0;
        for (&position, &length) in layout.iter().zip(&self.lengths) {
            covered += self.blocks[position..position + length]
                .iter()
                .filter(|x| **x == State::Damaged)
                .count();
        }
        covered == total_damaged
    }

    pub fn solve_recur(&mut self, start: usize, group: usize, history: Vec<usize>) -> usize {
        if group >= self.lengths.len() {
            return 0;
        }
        let mut sum = 0;

        for i in start..self.blocks.len() {
            if self.can_fit(i, self.lengths[group]) {
                let mut h = history.clone();
                h.push(i);
                if group == self.lengths.len() - 1 {
                    if self.check(&h) {
                        sum += 1;
                    }
                } else {
                    sum += self.solve_recur(i + self.lengths[group] + 1, group + 1, h);
                }
            }
        }
        sum
    }
}

fn main() {
    println!("Hello, world!");
    // let a = parse_one("?###???????? 3,2,1").unwrap();
    // let a = parse_one("????.######..#####. 1,6,5").unwrap();
    // let a = parse_one("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
    // let a = parse_one("????#?#???????#????? 6,5").unwrap();
    let mut a = parse_one("??.#???##?#??? 1,5").unwrap();
    dbg!(a.solve_recur(0, 0, Vec::new()));

    let input = include_str!("../input");
    let mut sum = 0;
    for a in input.lines().map(|l| parse_one(l)) {
        let x = a.unwrap().solve_recur(0, 0, vec![]);
        eprintln!("{:?}", x);
        sum += x;
    }
    eprintln!("part1={sum}");
}

const EX1: &'static str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
