use std::collections::HashMap;

fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let lines = input
        .split('\n')
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut symbol_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut row: isize = 0;
    'row: loop {
        if row >= (lines.len() as isize) {
            break 'row;
        }
        let mut col: isize = 0;
        'col: loop {
            if col >= (lines[row as usize].len() as isize) {
                break 'col;
            }
            let mut current_number = 0;
            let mut valid = false;
            let mut closest_symbol = None;
            'number: loop {
                if col >= (lines[row as usize].len() as isize)
                    || !lines[row as usize][col as usize].is_numeric()
                {
                    if let Some((r, c)) = closest_symbol {
                        symbol_map
                            .entry((r, c))
                            .and_modify(|ent| ent.push(current_number))
                            .or_insert(vec![current_number]);
                    }
                    break 'number;
                }
                current_number =
                    current_number * 10 + lines[row as usize][col as usize].to_digit(10).unwrap();
                if !valid {
                    'check: for s_row in [row - 1, row, row + 1] {
                        if let Some(r) = lines.get(s_row as usize) {
                            for s_col in [col - 1, col, col + 1] {
                                if let Some(c) = r.get(s_col as usize) {
                                    if !c.is_numeric() && !(*c == '.') {
                                        valid = true;
                                        if *c == '*' {
                                            closest_symbol = Some((s_row as usize, s_col as usize));
                                        }
                                        break 'check;
                                    }
                                }
                            }
                        }
                    }
                }
                col += 1;
            }
            if valid {
                part1 += current_number;
            }
            col += 1;
        }
        row += 1;
    }

    for entry in symbol_map {
        match entry.1.len() {
            2 => part2 += entry.1[0] * entry.1[1],
            _ => continue,
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
