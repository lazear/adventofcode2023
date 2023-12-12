use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub fn coord(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Dir::N => (x, y.saturating_sub(1)),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x.saturating_sub(1), y),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    // sparse array of edge indices
    nodes: Vec<Vec<usize>>,
    // dense array of connections between node #s
    edges: Vec<(usize, usize)>,

    stride: usize,
    start: usize,
}

pub fn parse(s: &str) -> Graph {
    let mut start = (0, 0);
    let mut edges = Vec::new();

    let mut stride = 0;
    let mut rows = 0;
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            use Dir::*;
            match ch {
                '|' => {
                    edges.push((N.coord(x, y), (x, y)));
                    edges.push(((x, y), S.coord(x, y)));
                }
                '-' => {
                    edges.push((W.coord(x, y), (x, y)));
                    edges.push(((x, y), E.coord(x, y)));
                }
                'L' => {
                    edges.push((N.coord(x, y), (x, y)));
                    edges.push(((x, y), E.coord(x, y)));
                }
                'J' => {
                    edges.push((N.coord(x, y), (x, y)));
                    edges.push(((x, y), W.coord(x, y)));
                }
                '7' => {
                    edges.push((S.coord(x, y), (x, y)));
                    edges.push(((x, y), W.coord(x, y)));
                }
                'F' => {
                    edges.push((S.coord(x, y), (x, y)));
                    edges.push(((x, y), E.coord(x, y)));
                }
                'S' => start = (x, y),
                '.' => {}
                _ => unreachable!("{}", ch),
            }
            stride = stride.max(x + 1);
        }
        rows += 1;
    }

    let edges = edges
        .into_iter()
        .map(|(start, finish)| (start.0 + (start.1 * stride), finish.0 + (finish.1 * stride)))
        .collect::<Vec<_>>();
    let start = start.0 + (start.1 * stride);

    let mut nodes = vec![Vec::new(); stride * rows];

    for (idx, edge) in edges.iter().enumerate() {
        if edge.0 < nodes.len() && edge.1 < nodes.len() {
            nodes[edge.0].push(idx);
            nodes[edge.1].push(idx);
        }
    }

    Graph {
        nodes,
        edges,
        stride,
        start,
    }
}

impl Graph {
    pub fn dfs(&self) -> usize {
        // let mut visited = vec![false; self.edges.len()];
        // let mut stack = vec![(self.start, 0)];
        let mut stack = VecDeque::new();
        stack.push_back((self.start, 0));
        let mut distance = vec![usize::MAX; self.nodes.len()];

        while let Some((node_idx, depth)) = stack.pop_front() {
            // eprintln!("visiting {node_idx} @ {depth}");
            if depth >= distance[node_idx] {
                // if distance[node_idx] != usize::MAX {
                // eprintln!("visiting {node_idx} @ {depth} - already been here at shorter distance");
                continue;
            }
            distance[node_idx] = distance[node_idx].min(depth);

            for edge_idx in &self.nodes[node_idx] {
                let edge = &self.edges[*edge_idx];
                if edge.0 == node_idx {
                    stack.push_back((edge.1, depth + 1));
                } else {
                    stack.push_back((edge.0, depth + 1));
                }
            }
            // dbg!(&stack);
            dbg!(stack.len());
        }

        for row in distance.chunks(distance.len() / self.stride) {
            let ch = row
                .iter()
                .map(|row| {
                    if *row >= 10 {
                        '.'
                    } else {
                        ((*row as u8).min(9) + b'0') as char
                    }
                })
                .collect::<String>();
            eprintln!("{}", ch)
        }

        distance
            .iter()
            .filter(|x| **x != usize::MAX)
            .copied()
            .max()
            .unwrap_or_default()
    }
}

fn main() {
    let g = dbg!(parse(EX1));
    dbg!(g.dfs());

    let input = include_str!("../input");
    let g = (parse(input));
    dbg!(g.dfs());

    // let m = g.dfs_(g.start, &mut vec![false; g.nodes.len()], 0);
    // dbg!(m);
}

const EX1: &'static str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

const EX2: &'static str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
