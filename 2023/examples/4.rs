fn main() {
    let day = 4;
    let input = aoc_2023::get_input(day, true);
    println!("Day {}", day);
    part_1(input.clone());
    part_2(input.clone());
}

fn part_1(input: String) {
    let mut answer = 0;

    for line in input.lines() {
        let data = line.split([':', '|']).collect::<Vec<&str>>();

        let winning_numbers = data[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let numbers = data[2]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut score = 0;

        for number in numbers {
            if winning_numbers.contains(&number) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        }

        answer += score;
    }

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    println!("Part 2: {}", answer);
}
