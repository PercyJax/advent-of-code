fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let mut lines = input.split('\n');

    let mut part1 = 0;
    let mut part2 = 0;

    // let now = std::time::Instant::now();
    // let part1_time = now.elapsed();
    // let part2_time = now.elapsed();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        // println!("{line}");
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    // println!("Part 1: {part1} - took {part1_time:.2?}");
    // println!("Part 2: {part2} - took {part2_time:.2?}");
}
