use std::time::Instant;

fn main() {
    let day = 7;
    let input = aoc_2023::get_input(day, true);
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

            panic!("{} and {} are equal", a.0, b.0);
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

    let mut cards = vec![
        // 'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
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

        println!("card_scores: {:?}", card_scores);

        let mut hand_type = 0;

        if card_scores.contains(&5)
            || card_scores.contains(&4) && hand.contains("J")
        {
            // five of a kind
            hand_type = 7;
            println!("hand: {} has five of a kind", hand);
        } else if card_scores.contains(&4)
            || card_scores.contains(&3) && hand.contains("J")
        {
            // four of a kind
            hand_type = 6;
            println!("hand: {} has four of a kind", hand);
        } else if card_scores.contains(&3) && card_scores.contains(&2)
            || hand.contains("J")
        {
            // full house
            hand_type = 5;
            println!("hand: {} has full house", hand);
        } else if card_scores.contains(&3) && !card_scores.contains(&2)
            || hand.contains("J")
        {
            // three of a kind
            hand_type = 4;
            println!("hand: {} has three of a kind", hand);
        } else if card_scores.iter().filter(|&n| *n == 2).count() == 2
            || hand.contains("J")
        {
            // two pair
            hand_type = 3;
            println!("hand: {} has two pair", hand);
        } else if card_scores.iter().filter(|&n| *n == 2).count() == 1
            || hand.contains("J")
        {
            // one pair
            hand_type = 2;
            println!("hand: {} has one pair", hand);
        } else if hand_type == 0 || hand.contains("J") {
            // high card
            hand_type = 1;
            println!("hand: {} has high card", hand);
        }

        hands.push((hand, bid, hand_type, card_scores));
    }

    hands.sort_by(|a, b| {
        if a.2 == b.2 {
            println!("comparing {} and {}", a.0, b.0);

            for i in 0..a.0.len() {
                let char_a = a.0.chars().nth(i).unwrap();
                let char_b = b.0.chars().nth(i).unwrap();

                println!("comparing {} and {}", char_a, char_b);

                if char_a != char_b {
                    let char_a_index =
                        cards.iter().position(|&c| c == char_a).unwrap();
                    let char_b_index =
                        cards.iter().position(|&c| c == char_b).unwrap();
                    println!("index of char_a: {}", char_a_index);
                    println!("index of char_b: {}", char_b_index);
                    println!(
                        "a is {} bigger than b",
                        char_a_index.cmp(&char_b_index)
                            == std::cmp::Ordering::Greater
                    );
                    return char_a_index.cmp(&char_b_index);
                }
            }

            panic!("{} and {} are equal", a.0, b.0);
        } else {
            a.2.cmp(&b.2)
        }
    });

    println!("hand, bid, hand_type, card_scores");
    for (i, hand) in hands.iter().enumerate() {
        println!("{:?} -> {} x {}", hand, hand.1, i + 1);
        answer += hand.1 * (i + 1);
    }

    println!("Part 2: {}", answer);
}
