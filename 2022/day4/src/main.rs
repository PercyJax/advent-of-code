use std::{usize};

fn main() {
    let input = std::str::from_utf8(include_bytes!("../input.txt")).unwrap();
    let lines = input.split('\n');

    let mut part_1: usize = 0;
    let mut part_2: usize = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut sections = line.split(',');
        let mut first_elf = sections.next().unwrap().split('-');
        let mut second_elf = sections.next().unwrap().split('-');
        let first_start = first_elf.next().unwrap().parse::<usize>().unwrap();
        let first_end = first_elf.next().unwrap().parse::<usize>().unwrap();
        let second_start = second_elf.next().unwrap().parse::<usize>().unwrap();
        let second_end = second_elf.next().unwrap().parse::<usize>().unwrap();

        if (first_start <= second_start && second_end <= first_end)
            || (second_start <= first_start && first_end <= second_end)
        {
            part_1 += 1;
        }

        if !(first_end < second_start || second_end < first_start)
        {
            part_2 += 1;
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
