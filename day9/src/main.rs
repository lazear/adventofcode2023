fn derivative(xs: &[i32]) -> Vec<i32> {
    xs.windows(2).map(|w| w[1] - w[0]).collect()
}

fn difference_engine(ys: &[i32]) -> i32 {
    let mut stack = Vec::new();
    let mut d = derivative(ys);
    while d.iter().any(|y| *y != 0) && !d.is_empty() {
        let nd = derivative(&d);
        stack.push(d);
        d = nd;
    }
    let mut next = 0;
    for d in stack.iter().rev() {
        next = -d.first().copied().unwrap_or_default() - next;
    }
    ys.first().copied().unwrap_or_default() + next
}

fn parse(line: &str) -> Option<Vec<i32>> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<i32>().ok())
        .collect::<Option<Vec<_>>>()
}

fn main() {
    println!("Hello, world!");

    let mut sum = 0;

    for line in include_str!("../input").lines() {
        let ys = parse(line).unwrap();
        let interp = difference_engine(&ys);
        eprintln!("{ys:?} .. {interp}");
        sum += interp;
    }
    eprintln!("part1={sum}");
}

const TEST: &'static str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
