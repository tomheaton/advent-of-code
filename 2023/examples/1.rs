use std::time::Instant;

fn main() {
    let day = 1;
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
    let mut sum = 0;

    for line in input.lines() {
        let mut num_str = String::new();

        for c in line.chars() {
            if c.is_numeric() {
                num_str.push(c);
            }
        }

        let first = num_str.chars().next().unwrap();
        let last = num_str.chars().last().unwrap();

        let calibration_value_str = format!("{}{}", first, last);
        let calibration_value =
            str::parse::<i32>(&calibration_value_str).unwrap();

        sum += calibration_value;
    }

    println!("Part 1: {}", sum);
}

fn part_2(input: String) {
    let text_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for line in input.lines() {
        let mut num_str = String::new();
        let mut current_word = String::new();

        for c in line.chars() {
            if c.is_numeric() {
                current_word.clear();

                num_str.push(c);
            }

            if c.is_alphabetic() {
                current_word.push(c);

                for (i, word) in text_numbers.iter().enumerate() {
                    if current_word.contains(word) {
                        num_str.push_str(&(i + 1).to_string());
                        current_word =
                            current_word.chars().last().unwrap().to_string();
                    }
                }
            }
        }

        let first = num_str.chars().next().unwrap();
        let last = num_str.chars().last().unwrap();

        let calibration_value_str = format!("{}{}", first, last);
        let calibration_value =
            str::parse::<i32>(&calibration_value_str).unwrap();

        sum += calibration_value;
    }

    println!("Part 2: {}", sum);
}
