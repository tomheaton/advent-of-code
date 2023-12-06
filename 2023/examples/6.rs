use std::time::Instant;

fn main() {
    let day = 6;
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
    let mut answer = 1;

    let time_data = input.lines().nth(0).unwrap();
    let distance_data = input.lines().nth(1).unwrap();

    let times = time_data.split_whitespace().skip(1).collect::<Vec<&str>>();
    let distances = distance_data
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>();

    for i in 0..times.len() {
        let mut ways_to_beat_record = Vec::new();

        let time = times[i].parse::<i32>().unwrap();
        let distance = distances[i].parse::<i32>().unwrap();

        for ms in 0..time {
            let speed = ms;
            let time_left = time - ms;

            if speed * time_left > distance {
                ways_to_beat_record.push(ms);
            }
        }

        answer *= ways_to_beat_record.len();
    }

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 1;

    println!("Part 2: {}", answer);
}
