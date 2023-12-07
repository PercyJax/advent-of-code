use std::collections::HashMap;

fn main() {
    let input = String::from_utf8(include_bytes!("../test.txt").to_vec()).unwrap();
    let mut lines = input.split('\n');

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    // let now = std::time::Instant::now();
    // let part1_time = now.elapsed();
    // let part2_time = now.elapsed();

    let mut hands = Vec::new();
    let mut hands2 = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut line_split = line.split_whitespace();
        let hand = line_split.next().unwrap();
        let bid: i32 = line_split.next().unwrap().parse().unwrap();

        // println!("{:?}", get_hand_strength(hand));

        hands.push((get_hand_strength(hand), bid));
        hands2.push((get_hand_strength2(hand), bid));
    }

    hands.sort_by(|a, b| match b.0 .0.cmp(&a.0 .0) {
        std::cmp::Ordering::Equal => match b.0 .1[0].cmp(&a.0 .1[0]) {
            std::cmp::Ordering::Equal => match b.0 .1[1].cmp(&a.0 .1[1]) {
                std::cmp::Ordering::Equal => match b.0 .1[2].cmp(&a.0 .1[2]) {
                    std::cmp::Ordering::Equal => match b.0 .1[3].cmp(&a.0 .1[3]) {
                        std::cmp::Ordering::Equal => b.0 .1[4].cmp(&a.0 .1[4]),
                        v => v,
                    },
                    v => v,
                },
                v => v,
            },
            v => v,
        },
        v => v,
    });

    hands2.sort_by(|a, b| match b.0 .0.cmp(&a.0 .0) {
        std::cmp::Ordering::Equal => match b.0 .1[0].cmp(&a.0 .1[0]) {
            std::cmp::Ordering::Equal => match b.0 .1[1].cmp(&a.0 .1[1]) {
                std::cmp::Ordering::Equal => match b.0 .1[2].cmp(&a.0 .1[2]) {
                    std::cmp::Ordering::Equal => match b.0 .1[3].cmp(&a.0 .1[3]) {
                        std::cmp::Ordering::Equal => b.0 .1[4].cmp(&a.0 .1[4]),
                        v => v,
                    },
                    v => v,
                },
                v => v,
            },
            v => v,
        },
        v => v,
    });

    for (rank, hand) in hands.iter().enumerate() {
        // println!(
        //     "hand.1 {} * rank {} - {:?}",
        //     hand.1,
        //     (hands.len() - rank),
        //     hand
        // );
        let winnings = hand.1 * (i32::try_from((hands.len() - rank)).unwrap());
        part1 += winnings;
    }

    for (rank, hand) in hands2.iter().enumerate() {
        // println!(
        //     "hand.2 {} * rank {} - {:?}",
        //     hand.1,
        //     (hands.len() - rank),
        //     hand
        // );
        let winnings = hand.1 * (i32::try_from((hands.len() - rank)).unwrap());
        part2 += winnings;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    // println!("Part 1: {part1} - took {part1_time:.2?}");
    // println!("Part 2: {part2} - took {part2_time:.2?}");
}

fn get_hand_strength(hand_str: &str) -> (u32, Vec<u8>) {
    if hand_str.len() != 5 {
        panic!("invalid hand");
    }
    let hand: Vec<u8> = hand_str
        .as_bytes()
        .iter()
        .map(|b| {
            // println!("{b}");
            match *b {
                65 => 14,
                75 => 13,
                81 => 12,
                74 => 11,
                84 => 10,
                57 => 9,
                56 => 8,
                55 => 7,
                54 => 6,
                53 => 5,
                52 => 4,
                51 => 3,
                50 => 2,
                _ => panic!("invalid card"),
            }
        })
        .collect();

    let mut frequency_map = HashMap::new();
    for card in hand.clone() {
        frequency_map
            .entry(card)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    let mut sorted_hand = frequency_map.into_iter().collect::<Vec<(u8, u8)>>();
    sorted_hand.sort_by(|a, b| b.1.cmp(&a.1));

    // println!("{:?}", sorted_hand);

    let hand_type = match sorted_hand[0].1 {
        5 => 7,
        4 => 6,
        3 => match sorted_hand[1].1 {
            2 => 5,
            _ => 4,
        },
        2 => match sorted_hand[1].1 {
            2 => 3,
            _ => 2,
        },
        _ => 1,
    };

    (hand_type, hand)
}
fn get_hand_strength2(hand_str: &str) -> (u32, Vec<u8>) {
    if hand_str.len() != 5 {
        panic!("invalid hand");
    }
    let mut num_joker = 0;
    let hand: Vec<u8> = hand_str
        .as_bytes()
        .iter()
        .map(|b| {
            // println!("{b}");
            match *b {
                65 => 14,
                75 => 13,
                81 => 12,
                74 => {
                    num_joker += 1;
                    1
                }
                84 => 10,
                57 => 9,
                56 => 8,
                55 => 7,
                54 => 6,
                53 => 5,
                52 => 4,
                51 => 3,
                50 => 2,
                _ => panic!("invalid card"),
            }
        })
        .collect();

    let mut frequency_map = HashMap::new();
    for card in hand.clone() {
        frequency_map
            .entry(card)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    let mut sorted_hand = frequency_map.into_iter().collect::<Vec<(u8, u8)>>();
    sorted_hand.sort_by(|a, b| b.1.cmp(&a.1));

    if sorted_hand[0].0 != 1 {
        sorted_hand[0].1 += num_joker;
    } else {
        if sorted_hand.len() > 1 {
            sorted_hand[1].1 += num_joker;
            sorted_hand.remove(0);
        }
    }

    // sorted_hand[0].1 += num_joker;

    // println!("{:?}", sorted_hand);

    let hand_type = match sorted_hand[0].1 {
        5 => 7,
        4 => 6,
        3 => match sorted_hand[1].1 {
            2 => 5,
            _ => 4,
        },
        2 => match sorted_hand[1].1 {
            2 => 3,
            _ => 2,
        },
        _ => 1,
    };

    (hand_type, hand)
}
