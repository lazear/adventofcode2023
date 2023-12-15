#[derive(Clone, Copy, Debug)]
struct Lens<'s> {
    label: &'s str,
    power: usize,
    // position: usize,
}

pub fn hash(s: &str) -> usize {
    let mut curr = 0;
    for b in s.as_bytes() {
        curr += (*b as usize);
        curr *= 17;
        curr = curr % 256;
    }
    curr
}

fn main() {
    println!("Hello, world!");

    let s = include_str!("../input").trim();
    // let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    let mut sum = 0;
    for seq in s.split(',') {
        sum += hash(seq);

        let n = seq.chars().filter(|c| c.is_alphabetic()).count();
        let label = &seq[..n];
        let instr = &seq[n..n + 1];
        let idx = hash(label);

        if instr == "=" {
            let power = (&seq[n + 1..]).parse::<usize>().unwrap();
            println!("{label}_{instr}_{power}: {}", hash(label));
            let mut found = false;
            for lens in boxes[idx].iter_mut() {
                if lens.label == label {
                    lens.power = power;
                    found = true;
                }
            }
            if !found {
                boxes[idx].push(Lens { label, power });
            }
        } else {
            boxes[idx] = boxes[idx]
                .drain(..)
                .filter(|lens| lens.label != label)
                .collect();
        }
    }

    let part2 = boxes.iter().enumerate().fold(0, |acc, (bx_ix, bx)| {
        acc + bx.iter().enumerate().fold(0, |acc, (ix, lens)| {
            acc + (1 + bx_ix) * (1 + ix) * lens.power
        })
    });

    println!("part1={sum}");
    println!("part2={part2}");
}
