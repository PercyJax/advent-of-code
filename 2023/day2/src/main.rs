fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let lines = input.split('\n');

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut game_info = line.split(':');
        let game_id: u32 = game_info
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let game_grabs_info = game_info.next().unwrap().trim().split(';');

        let mut seen_red = 0;
        let mut seen_green = 0;
        let mut seen_blue = 0;

        for grab_info in game_grabs_info {
            let grab_info = grab_info.trim().split(',');
            for color_info in grab_info {
                let mut info = color_info.trim().split(' ');
                let num: usize = info.next().unwrap().trim().parse().unwrap();
                let color = info.next().unwrap().trim();
                match color {
                    "red" => seen_red = num.max(seen_red),
                    "green" => seen_green = num.max(seen_green),
                    "blue" => seen_blue = num.max(seen_blue),
                    _ => panic!("invalid color"),
                }
            }
        }

        part2 += seen_red * seen_green * seen_blue;

        if seen_red > 12 {
            continue;
        }
        if seen_green > 13 {
            continue;
        }
        if seen_blue > 14 {
            continue;
        }
        part1 += game_id;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
