fn main() {
    let input = std::str::from_utf8(include_bytes!("../input.txt")).unwrap();
    let rounds = input.split('\n');
    let mut score = 0;
    let mut part_2_score = 0;
    for round in rounds {
        let moves: Vec<&str> = round.split(' ').collect();
        if moves.len() < 2 {
            continue;
        }
        let opponent = match moves[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("invalid opponent move"),
        };
        let own = match moves[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("invalid own move"),
        };
        let part_2_outcome = match own {
            1 => 0,
            2 => 3,
            3 => 6,
            _ => panic!("invalid inputs"),
        };
        match ((own - opponent) + 3) % 3 {
            2 => (),
            0 => score += 3,
            1 => score += 6,
            _ => panic!("invalid inputs"),
        };
        part_2_score += match own {
            1 => (opponent + 1) % 3 + 1,
            2 => (opponent + 2) % 3 + 1,
            3 => opponent % 3 + 1,
            _ => panic!("invalid part_2_outcome"),
        } + part_2_outcome;
        score += own;
    }
    println!("Part 1: {}", score);
    println!("Part 2: {}", part_2_score);
}
