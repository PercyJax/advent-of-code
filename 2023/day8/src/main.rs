use std::collections::{HashMap, HashSet};
fn main() {
    let input = String::from_utf8(include_bytes!("../input.txt").to_vec()).unwrap();
    let mut lines = input.split('\n');

    let mut part1 = 0;
    let mut part2 = 1;

    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let node = &line[0..=2];
        let left = &line[7..=9];
        let right = &line[12..=14];

        map.insert(node.to_string(), (left.to_string(), right.to_string()));
    }

    let now = std::time::Instant::now();

    let mut current_node = "AAA".to_string();

    'navigate: loop {
        if current_node == "ZZZ" {
            break 'navigate;
        }
        let prev_node = current_node;
        current_node = match directions[part1 % directions.len()] {
            'L' => map.get(&prev_node).unwrap().0.to_string(),
            'R' => map.get(&prev_node).unwrap().1.to_string(),
            _ => panic!("invalid directions"),
        };
        part1 += 1;
        // println!("at {} step {} next {}", prev_node, part1, current_node)
    }
    let part1_time = now.elapsed();
    println!("Part 1: {part1} - took {part1_time:.2?}");

    let now = std::time::Instant::now();

    let p2_nodes: Vec<String> = map
        .iter()
        .filter_map(|m| {
            if m.0.chars().collect::<Vec<char>>()[2] == 'A' {
                Some((*m.0).to_string())
            } else {
                None
            }
        })
        .collect();

    // println!("p2_nodes: {:?}", p2_nodes);

    let mut factors = HashSet::new();

    for node in p2_nodes {
        let period: u128 = get_period(node, &directions, &map).try_into().unwrap();
        factors.extend(prime_factorization::Factorization::run(period).factors)
    }

    for factor in factors {
        part2 *= factor;
    }

    let part2_time = now.elapsed();

    println!("Part 2: {part2} - took {part2_time:.2?}");
}

fn get_period(
    mut node: String,
    directions: &[char],
    map: &HashMap<String, (String, String)>,
) -> usize {
    let mut period = 0;
    let mut steps = 0;
    let mut counter = 0;

    loop {
        let prev_node = node;
        node = match directions[steps % directions.len()] {
            'L' => map.get(&prev_node).unwrap().0.to_string(),
            'R' => map.get(&prev_node).unwrap().1.to_string(),
            _ => panic!("invalid directions"),
        };
        steps += 1;
        counter += 1;
        if node.chars().collect::<Vec<char>>()[2] == 'Z' {
            // println!("node {} found {} at step {}", original_node, node, steps);
            if counter == period {
                return period;
            } else {
                period += counter;
                counter = 0;
            }
        }
    }
}
