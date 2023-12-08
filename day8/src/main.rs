use std::collections::HashMap;

#[derive(Debug)]
struct Node<'s> {
    label: &'s str,
    children: [&'s str; 2],
}

#[derive(Debug)]
struct Program<'s> {
    instructions: Vec<usize>,
    nodes: Vec<Node<'s>>,
}

fn parse_node<'s>(line: &'s str) -> Node<'s> {
    let label = &line[..3];
    let left = &line[7..10];
    let right = &line[12..15];
    Node {
        label: label,
        children: [left, right],
    }
}

fn parse_program<'s>(text: &'s str) -> Option<Program<'s>> {
    let mut lines = text.lines();
    let instructions = lines
        .next()?
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let nodes = lines.skip(1).map(parse_node).collect();

    Some(Program {
        instructions,
        nodes,
    })
}

impl<'s> Program<'s> {
    pub fn run(&self) -> u32 {
        let graph: HashMap<&str, usize> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(idx, n)| (n.label, idx))
            .collect();
        let mut ptr = graph["AAA"];
        let end = graph["ZZZ"];
        let mut steps = 0;
        for idx in self.instructions.iter().cycle() {
            if ptr == end {
                break;
            }
            let to = self.nodes[ptr].children[*idx];
            ptr = graph[to];
            steps += 1;
        }
        steps
    }

    pub fn run_2(&self, start: &str) -> usize {
        let graph: HashMap<&str, usize> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(idx, n)| (n.label, idx))
            .collect();
        let mut ptr = graph[start];
        let mut steps = 0;
        for idx in self.instructions.iter().cycle() {
            let to = self.nodes[ptr].children[*idx];
            ptr = graph[to];
            steps += 1;
            if to.ends_with("Z") {
                break;
            }
        }
        steps
    }

    pub fn ghost_run(&self) -> usize {
        let ghosts = self
            .nodes
            .iter()
            .filter(|n| n.label.ends_with("A"))
            .collect::<Vec<_>>();
        let mut steps = 1;
        for ghost in &ghosts {
            steps *= self.run_2(ghost.label) / self.instructions.len();
        }
        steps * self.instructions.len()
    }
}

fn main() {
    println!("Hello, world!");
    // let p = parse_program(TEST3).unwrap();
    let p = parse_program(include_str!("../input")).unwrap();
    dbg!(p.ghost_run());
}

const TEST: &'static str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

const TEST2: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const TEST3: &'static str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
