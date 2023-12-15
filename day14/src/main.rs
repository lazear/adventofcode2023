use std::collections::{HashMap, HashSet};

fn part1(s: &str) -> usize {
    let mut columns = Vec::new();

    for line in s.lines() {
        for (col, ch) in line.chars().enumerate() {
            if col == columns.len() {
                columns.push(Vec::new());
            }
            columns[col].push(ch);
        }
    }

    // tilt north
    for col in columns.iter_mut() {
        let mut x = 0;
        for i in 0..col.len() {
            if col[i] == 'O' {
                let mut j = i;
                while j > x && col[j - 1] == '.' {
                    col.swap(j, j - 1);
                    j -= 1;
                }
            } else if col[i] == '#' {
                x = i;
            }
        }
    }

    let mut load = 0;
    for col in columns.iter() {
        load +=
            col.iter().rev().enumerate().fold(
                0,
                |acc, (idx, val)| {
                    if *val == 'O' {
                        acc + (idx + 1)
                    } else {
                        acc
                    }
                },
            )
    }
    load
}

fn part2(s: &str) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut map = Vec::new();
    for line in s.lines() {
        for (col, ch) in line.chars().enumerate() {
            map.push(ch);
            x = x.max(col + 1);
        }
        y += 1;
    }

    // Store a hash of state -> next state, cycle #, load
    let mut hashes: HashMap<String, (Vec<char>, _, _)> = HashMap::new();
    let mut cycle_hits = HashSet::new();
    let mut values = Vec::new();
    let mut final_cycle = 0;

    for iter in 0..1_000_000_000 {
        let old = map.iter().copied().collect::<String>();
        if let Some((m, cycle, load)) = hashes.get(&old) {
            // we have hit this before, cycle complete
            if !cycle_hits.insert(*cycle) {
                break;
            }
            // store load so we can find the repeating cycle
            values.push(*load);
            map = m.clone();
            continue;
        }
        // go north
        for col in 0..x {
            let mut rock = col;
            for row in 0..y {
                let mut ptr = row * x + col;
                if map[ptr] == 'O' {
                    while ptr > rock && map[ptr - x] == '.' {
                        map.swap(ptr, ptr - x);
                        ptr -= x;
                    }
                } else if map[ptr] == '#' {
                    rock = ptr;
                }
            }
        }

        // go west
        for row in 0..y {
            let mut rock = row * x;
            for col in 0..x {
                let mut ptr = row * x + col;
                if map[ptr] == 'O' {
                    while ptr > rock && map[ptr - 1] == '.' {
                        map.swap(ptr, ptr - 1);
                        ptr -= 1;
                    }
                } else if map[ptr] == '#' {
                    rock = ptr;
                }
            }
        }

        // go south
        for col in 0..x {
            let mut rock = x * (y - 1) + col;
            for row in 0..y {
                let mut ptr = x * (y - row - 1) + col;
                if map[ptr] == 'O' {
                    while ptr < rock && map[ptr + x] == '.' {
                        map.swap(ptr, ptr + x);
                        ptr += x;
                    }
                } else if map[ptr] == '#' {
                    rock = ptr;
                }
            }
        }

        // go east
        for row in 0..y {
            let mut rock = row * x + x - 1;
            for col in 0..x {
                let mut ptr = row * x + x - col - 1;
                if map[ptr] == 'O' {
                    while ptr < rock && map[ptr + 1] == '.' {
                        map.swap(ptr, ptr + 1);
                        ptr += 1;
                    }
                } else if map[ptr] == '#' {
                    rock = ptr;
                }
            }
        }

        let mut load = 0;
        for row in 0..y {
            for col in 0..x {
                if map[row * x + col] == 'O' {
                    load += y - row;
                }
            }
        }
        hashes.insert(old, (map.clone(), iter, load));
        final_cycle = iter + 1;
    }

    let i = (999_999_999 - final_cycle) % values.len();
    values[i]
}

fn main() {
    // dbg!(part1(EX1));
    let s = include_str!("../input");
    dbg!(part1(s));
    dbg!(part2(s));
}

const EX1: &'static str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
