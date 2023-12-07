use std::time::Instant;

fn main() {
    let day = 7;
    let input = aoc_2023::get_input(day, false);
    println!("Day {}", day);
    let start = Instant::now();
    part_1(input.clone());
    let duration = start.elapsed();
    println!("Part 1 finished in {}ms", duration.as_millis());
    let start = Instant::now();
    part_2(input.clone());
    let duration = start.elapsed();
    println!("Part 2 finished in {}ms", duration.as_millis());
}

fn part_1(input: String) {
    let mut answer = 0;

    let mut cards = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    cards.reverse();

    let mut hands = Vec::new();

    for line in input.lines() {
        let data = line.split(" ").collect::<Vec<&str>>();
        let (hand, bid) = (data[0], data[1].parse::<usize>().unwrap());

        let card_scores = cards
            .iter()
            .map(|&card| hand.chars().filter(|&c| c == card).count())
            .collect::<Vec<usize>>();

        let mut hand_type = 0;

        if card_scores.contains(&5) {
            // five of a kind
            hand_type = 7;
        } else if card_scores.contains(&4) {
            // four of a kind
            hand_type = 6;
        } else if card_scores.contains(&3) && card_scores.contains(&2) {
            // full house
            hand_type = 5;
        } else if card_scores.contains(&3) && !card_scores.contains(&2) {
            // three of a kind
            hand_type = 4;
        } else if card_scores.iter().filter(|&n| *n == 2).count() == 2 {
            // two pair
            hand_type = 3;
        } else if card_scores.iter().filter(|&n| *n == 2).count() == 1 {
            // one pair
            hand_type = 2;
        } else if hand_type == 0 {
            // high card
            hand_type = 1;
        }

        hands.push((hand, bid, hand_type));
    }

    hands.sort_by(|a, b| {
        if a.2 == b.2 {
            for i in 0..a.0.len() {
                let char_a = a.0.chars().nth(i).unwrap();
                let char_b = b.0.chars().nth(i).unwrap();

                if char_a != char_b {
                    let char_a_index =
                        cards.iter().position(|&c| c == char_a).unwrap();
                    let char_b_index =
                        cards.iter().position(|&c| c == char_b).unwrap();

                    return char_a_index.cmp(&char_b_index);
                }
            }

            return a.1.cmp(&b.1);
        } else {
            a.2.cmp(&b.2)
        }
    });

    for (i, hand) in hands.iter().enumerate() {
        answer += hand.1 * (i + 1);
    }

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    println!("Part 2: {}", answer);
}
