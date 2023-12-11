use std::time::Instant;

fn main() {
    let day = 11;
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

    for line in input.lines() {
        println!("{}", line);
    }

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    println!("Part 2: {}", answer);
}
