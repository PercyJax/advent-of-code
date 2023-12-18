use std::usize;

fn main() {
    let input = std::str::from_utf8(include_bytes!("../test.txt")).unwrap();
    let mut lines = input.split('\n');

    let mut part_1 = Vec::new();
    let mut part_2 = Vec::new();

    let mut current_line;
    let mut initial_stack_str_vec: Vec<&str> = Vec::new();
    let num_stacks;
    let mut p1_stacks: Vec<Vec<char>> = Vec::new();
    let mut p2_stacks: Vec<Vec<char>> = Vec::new();

    current_line = if let Some(current_line) = lines.next() {
        current_line
    } else {
        panic!("no stack initialization found");
    };
    'initialize_stack: loop {
        if current_line.len() >= 3 && current_line[0..=2] == *" 1 " {
            num_stacks = current_line.split_whitespace().count();
            for _ in 1..=num_stacks {
                p1_stacks.push(Vec::new());
                p2_stacks.push(Vec::new());
            }

            for layer in initial_stack_str_vec.iter().rev() {
                // println!("layer: {}", layer);
                // println!("layer_val: {:?}", layer.as_bytes());
                for stack in 1..=num_stacks {
                    let idx = (stack - 1) * 4 + 1;
                    let layer_chars = layer.chars().collect::<Vec<char>>();
                    // println!("layer_chars: {:?}", layer_chars);
                    match layer_chars[idx] {
                        ' ' => (),
                        c => {
                            p1_stacks[stack - 1].push(c);
                            p2_stacks[stack - 1].push(c);
                        }
                    }
                }
            }

            break 'initialize_stack;
        }
        initial_stack_str_vec.push(current_line);
        current_line = if let Some(current_line) = lines.next() {
            current_line
        } else {
            panic!("no instructions section found");
        };
    }
    'main: loop {
        current_line = if let Some(current_line) = lines.next() {
            current_line
        } else {
            break 'main;
        };

        if current_line.is_empty() {
            continue;
        }
        // println!("instruction: {}", current_line);

        let mut split = current_line.split_whitespace();
        let move_count: i32 = split.nth(1).unwrap().parse().unwrap();
        let move_from: usize = split.nth(1).unwrap().parse().unwrap();
        let move_to: usize = split.nth(1).unwrap().parse().unwrap();

        for _ in 1..=move_count {
            let cargo = p1_stacks[move_from - 1].pop().unwrap();
            p1_stacks[move_to - 1].push(cargo);
        }
        let mut cargo_to_move = Vec::new();
        for _ in 1..=move_count {
            let cargo = p2_stacks[move_from - 1].pop().unwrap();
            cargo_to_move.push(cargo);
        }
        for cargo in cargo_to_move.into_iter().rev() {
            p2_stacks[move_to - 1].push(cargo);
        }
    }

    for stack in p1_stacks {
        part_1.push(stack.clone().pop().unwrap());
    }

    for stack in p2_stacks {
        part_2.push(stack.clone().pop().unwrap());
    }

    println!("Part 1: {}", part_1.iter().cloned().collect::<String>());
    println!("Part 2: {}", part_2.iter().cloned().collect::<String>());
}
