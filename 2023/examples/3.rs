fn main() {
    let day = 3;
    let input = aoc_2023::get_input(day, true);
    println!("Day {}", day);
    part_1(input.clone());
    part_2(input.clone());
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
