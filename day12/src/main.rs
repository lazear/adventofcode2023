use std::collections::HashMap;

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
    fn expand(&self) -> Alignment {
        let blocks = (0..5)
            .flat_map(|n| {
                self.blocks
                    .iter()
                    .copied()
                    .chain(std::iter::once(State::Unknown).take(4 - n))
            })
            .collect::<Vec<_>>();

        let lengths = (0..5)
            .flat_map(|_| self.lengths.iter().copied())
            .collect::<Vec<_>>();
        Alignment { blocks, lengths }
    }

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

    /// check layout
    fn check(&self, layout: &[usize]) -> bool {
        let mut aligned = vec![State::Unknown; self.blocks.len()];
        let mut max = 0;
        for (&position, &length) in layout.iter().zip(&self.lengths) {
            for i in 0..length {
                aligned[position + i] = State::Damaged;
            }
            max = max.max(position + length);
        }
        if layout.len() < self.lengths.len() {
            aligned.truncate(max + 1);
        }

        for (actual, aligned) in self.blocks.iter().zip(&aligned) {
            if *actual == State::Damaged && *aligned != State::Damaged {
                return false;
            }
        }
        true
    }

    pub fn solve_recur(
        &self,
        start: usize,
        group: usize,
        history: Vec<usize>,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if group >= self.lengths.len() {
            return 0;
        }

        if let Some(sum) = memo.get(&(start, group)) {
            return *sum;
        }

        let mut sum = 0;
        let len = self.lengths[group];
        for i in start..self.blocks.len() {
            if self.can_fit(i, len) {
                let mut h = history.clone();
                h.push(i);
                if self.check(&h) {
                    if group == self.lengths.len() - 1 {
                        sum += 1;
                    } else {
                        let sub = self.solve_recur(i + len + 1, group + 1, h, memo);
                        sum += sub;
                    }
                }
            }
        }
        memo.insert((start, group), sum);
        sum
    }
}

fn main() {
    let input = include_str!("../input");
    let mut sum = 0;
    for a in input.lines().map(|l| parse_one(l)) {
        let a = a.unwrap();
        let x = a.expand().solve_recur(0, 0, vec![], &mut HashMap::new());
        dbg!(x);
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
