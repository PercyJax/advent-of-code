use std::collections::HashSet;

fn main() {
    let input = std::str::from_utf8(include_bytes!("../test.txt")).unwrap();
    let lines = input.split('\n');

    let mut part_1: isize = 0;

    let mut group_badges = Vec::new();
    let mut working_set = HashSet::new();

    for (rucksack, line) in lines.enumerate() {
        let _group = rucksack / 3;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (first, second) = (
            line[..line.len() / 2].as_bytes(),
            line[line.len() / 2..].as_bytes(),
        );
        let mut first_set = HashSet::new();
        for item in first {
            first_set.insert(item);
        }
        for item in second {
            if first_set.contains(item) {
                let dup: isize = if *item < 97 {
                    *item as isize - 65 + 27
                } else {
                    *item as isize - 97 + 1
                };
                part_1 += dup;
                break;
            }
        }
        // let dup_char = if dup < 27 { dup + 96 } else { dup - 26 + 64 };
        // println!(
        //     "Rucksack {} [{:?} - {:?}] has duplicate item {} ({})",
        //     rucksack + 1,
        //     first,
        //     second,
        //     dup_char,
        //     String::from_utf8_lossy(vec![dup_char as u8].as_slice())
        // );
        match rucksack % 3 {
            0 => {
                working_set.extend(line.as_bytes());
            }
            1 => {
                let mut new_set = HashSet::new();
                for item in line.as_bytes() {
                    if working_set.contains(item) {
                        new_set.insert(item);
                    }
                }
                working_set = new_set;
            }
            2 => {
                for item in line.as_bytes() {
                    if working_set.contains(item) {
                        group_badges.push(item);
                        working_set.clear();
                        break;
                    }
                }
            }
            _ => panic!("modulo failed"),
        }
    }
    let mut part_2: isize = 0;
    for b in group_badges {
        if *b < 97 {
            part_2 += *b as isize - 65 + 27;
        } else {
            part_2 += *b as isize - 97 + 1;
        }
    }
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
