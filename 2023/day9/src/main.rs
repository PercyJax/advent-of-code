fn main() {
    let input = String::from_utf8(include_bytes!("../test.txt").to_vec()).unwrap();
    let lines = input.split('\n');

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let line_split = line.split_whitespace();

        let mut nums = Vec::new();
        for val in line_split {
            let val: i32 = val.parse().unwrap();
            nums.push(val);
        }
        part1 += get_prev_next_number(nums.clone()).1;
        part2 += get_prev_next_number(nums).0
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn get_prev_next_number(nums: Vec<i32>) -> (i32, i32) {
    let next_row = nums.windows(2).map(|e| e[1] - e[0]);
    if next_row.clone().all(|e| e == 0) {
        return (nums[0], nums[0]);
    }
    let (prev_number, next_number) = get_prev_next_number(next_row.collect());
    return (nums[0] - prev_number, nums.last().unwrap() + next_number);
}
