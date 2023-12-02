fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let mut lines = input.split('\n');

    let mut part1 = 1;
    let mut part2 = 1;

    let mut p2_time = String::new();
    let mut p2_dist = String::new();

    let mut times_split = lines.next().unwrap().split_whitespace();
    let mut distances_split = lines.next().unwrap().split_whitespace();

    let mut races: Vec<(i32, i32)> = Vec::new();

    while let Some(time) = times_split.next() {
        if time == "Time:" {
            distances_split.next();
            continue;
        }
        let dist = distances_split.next().unwrap();
        races.push((time.parse().unwrap(), dist.parse().unwrap()));
        p2_time = p2_time + time;
        p2_dist = p2_dist + dist;
    }

    let p2_time: f32 = p2_time.parse().unwrap();
    let p2_dist: f32 = p2_dist.parse().unwrap();

    let now = std::time::Instant::now();

    for race in races {
        let time = race.0 as f32;
        let distance = race.1 as f32;

        let x_1 = -0.00001 + (time + (time.powi(2) - (4.0 * distance)).sqrt()) / 2.0;
        let x_2 = 0.00001 + (time - (time.powi(2) - (4.0 * distance)).sqrt()) / 2.0;

        let first = x_2.ceil() as i32;
        let last = x_1.floor() as i32;

        part1 *= last - first + 1;
    }

    let part1_time = now.elapsed();

    let now = std::time::Instant::now();

    let time = p2_time as f32;
    let distance = p2_dist as f32;

    let x_1 = -0.00001 + (time + (time.powi(2) - (4.0 * distance)).sqrt()) / 2.0;
    let x_2 = 0.00001 + (time - (time.powi(2) - (4.0 * distance)).sqrt()) / 2.0;

    let first = x_2.ceil() as i32;
    let last = x_1.floor() as i32;

    part2 *= last - first + 1;

    let part2_time = now.elapsed();

    println!("Part 1: {part1} - took {part1_time:.2?}");
    println!("Part 2: {part2} - took {part2_time:.2?}");
}
