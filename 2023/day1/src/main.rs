fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let lines = input.split('\n');

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        // Part 1
        let p1_first_digit = line.chars().find(|c| char::is_numeric(*c)).unwrap();
        let p1_last_digit = line.chars().rev().find(|c| char::is_numeric(*c)).unwrap();
        part1 += p1_first_digit.to_digit(10).unwrap() * 10 + p1_last_digit.to_digit(10).unwrap();

        // Part 2
        let digits = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("0", 0),
        ];

        let mut indices: Vec<(usize, i32)> = Vec::new();

        for digit in digits {
            indices.extend(
                line.match_indices(digit.0)
                    .map(|i| (i.0, digit.1))
                    .collect::<Vec<(usize, i32)>>(),
            );
        }

        let first = indices.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().1;
        let last = indices.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().1;

        part2 += (first * 10) + last;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
