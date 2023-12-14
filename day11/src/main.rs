fn parse(s: &str, expand: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut empty_cols = Vec::new();
    let mut empty_rows = Vec::new();
    for (y, line) in s.lines().enumerate() {
        empty_cols.extend(std::iter::repeat(true).take(line.len() - empty_cols.len()));
        let mut row = Vec::new();
        for (x, ch) in line.bytes().enumerate() {
            if ch == b'#' {
                row.push((x, y));
                empty_cols[x] = false;
            }
        }
        empty_rows.push(row.is_empty());
        positions.extend(row);
    }
    let mut column_map = (0..empty_cols.len()).collect::<Vec<_>>();
    for i in 0..empty_cols.len() {
        if empty_cols[i] {
            for j in i..empty_cols.len() {
                column_map[j] += expand;
            }
        }
    }
    dbg!(&column_map);

    let mut row_map = (0..empty_rows.len()).collect::<Vec<_>>();
    for i in 0..empty_rows.len() {
        if empty_rows[i] {
            for j in i..empty_rows.len() {
                row_map[j] += expand;
            }
        }
    }

    positions
        .into_iter()
        .map(|(x, y)| (column_map[x], row_map[y]))
        .collect()
}

fn manhattan(x: (usize, usize), y: (usize, usize)) -> usize {
    (x.0 as isize - y.0 as isize).abs() as usize + (x.1 as isize - y.1 as isize).abs() as usize
}

fn run(p: &[(usize, usize)]) -> usize {
    let mut sum = 0;
    for i in 0..p.len() {
        for j in i + 1..p.len() {
            sum += manhattan(p[i], p[j]);
        }
    }
    sum
}

fn main() {
    println!("Hello, world!");
    let a = parse(include_str!("../input"), 1000000 - 1);
    eprintln!("part1={}", run(&a));
}

#[test]
fn test_expand() {
    let a = parse(EX1, 1);
    let b = parse(EX2, 0);
    assert_eq!(a, b);
}

#[test]
fn test_dist() {
    let a = parse(EX1, 1);
    assert_eq!(manhattan(*&a[4], *&a[8]), 9);
    assert_eq!(manhattan(*&a[0], *&a[6]), 15);
    assert_eq!(manhattan(*&a[2], *&a[5]), 17);
    assert_eq!(manhattan(*&a[7], *&a[8]), 5);
}

const EX1: &'static str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

const EX2: &'static str = "\
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
