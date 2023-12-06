use indicatif::ProgressIterator;
use rayon::prelude::*;

#[derive(Debug)]
struct Entry {
    src: std::ops::Range<usize>,
    dst: std::ops::Range<usize>,
}

#[derive(Debug)]
struct Map {
    entries: Vec<Entry>,
}

fn parse(s: &str) -> Option<(Vec<usize>, Vec<Map>)> {
    let mut l = s.lines();
    let seeds = l
        .next()?
        .split(":")
        .skip(1)
        .next()?
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut maps = Vec::new();
    let mut tmp = Vec::new();

    for line in l {
        if line.is_empty() {
            continue;
        } else if line.starts_with(char::is_alphabetic) {
            if !tmp.is_empty() {
                maps.push(Map { entries: tmp });
                tmp = Vec::new();
            }
        } else {
            let rng = line
                .split(" ")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();
            tmp.push(Entry {
                dst: rng[0]..rng[0] + rng[2],
                src: rng[1]..rng[1] + rng[2],
            });
        }
    }
    if !tmp.is_empty() {
        maps.push(Map { entries: tmp });
    }
    Some((seeds, maps))
}

fn part1(seeds: &[usize], maps: &[Map]) -> usize {
    let mut lowest_ptr = usize::MAX;
    for seed in seeds {
        let mut ptr = *seed;
        for map in maps {
            for range in &map.entries {
                if range.src.contains(&ptr) {
                    eprintln!(
                        " {} => {:?} .. {}",
                        ptr,
                        range.src,
                        range.dst.start + (ptr - range.src.start)
                    );
                    ptr = range.dst.start + (ptr - range.src.start);
                    break;
                }
            }
        }
        eprintln!("seed {seed} ptr={ptr}");
        lowest_ptr = lowest_ptr.min(ptr);
    }
    lowest_ptr
}

fn main() {
    let (seeds, maps) = parse(include_str!("../../input/day5")).unwrap();

    println!("part1={}", part1(&seeds, &maps));
    let mut lowest_ptr = usize::MAX;
    for s in seeds.chunks(2).progress_count(seeds.len() as u64 / 2) {
        let min = (s[0]..s[0] + s[1]).into_par_iter().fold(
            || usize::MAX,
            |acc, seed| {
                let mut ptr = seed;
                for map in &maps {
                    for range in &map.entries {
                        if range.src.contains(&ptr) {
                            ptr = range.dst.start + (ptr - range.src.start);
                            break;
                        }
                    }
                }
                acc.min(ptr)
            }
        ).reduce(|| usize::MAX, |acc, x| acc.min(x));
        lowest_ptr = lowest_ptr.min(min);
    }
    println!("part2={}", lowest_ptr);
}

const TEST: &'static str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
