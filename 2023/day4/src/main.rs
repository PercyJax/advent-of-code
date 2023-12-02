use std::collections::{HashMap, HashSet};

fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let lines = input.split('\n');

    let mut part1 = 0;
    let mut part2 = 0;

    let mut multipliers: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        // println!("{line}");

        let mut line = line.split(':');
        let card_num = usize::from_str_radix(
            line.next()
                .unwrap()
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .trim(),
            10,
        )
        .unwrap();
        let mut line = line.next().unwrap().split('|');
        let winning_numbers: HashSet<usize> = line
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|n| usize::from_str_radix(n.trim(), 10).unwrap())
            .collect();
        let my_numbers: Vec<usize> = line
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|n| usize::from_str_radix(n.trim(), 10).unwrap())
            .collect();

        let mut wins = 0;

        for num in my_numbers {
            if winning_numbers.contains(&num) {
                wins += 1;
            }
        }

        let multiplier = multipliers.entry(card_num).or_insert(1).clone();

        if wins > 0 {
            part1 += 2_usize.pow(wins - 1);
            for _ in 0..multiplier {
                for offset in 1..=wins {
                    multipliers
                        .entry(card_num + offset as usize)
                        .and_modify(|e| *e += 1)
                        .or_insert(2);
                }
            }
        }
    }

    for elem in multipliers {
        // println!("Card {} has {} instances", elem.0, elem.1);
        part2 += elem.1;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
