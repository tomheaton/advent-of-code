use std::time::Instant;

fn main() {
    let day = 5;
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
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut sections = Vec::new();
    let mut section = Vec::new();

    for line in input.lines().skip(2) {
        if line.is_empty() {
            sections.push(section.clone());
            continue;
        }

        if line.ends_with(':') {
            section = Vec::new();
            continue;
        }

        let line_data = line
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        section.push((line_data[0], line_data[1], line_data[2]));
    }

    sections.push(section.clone());

    let mut min = u32::MAX;

    for seed in seeds {
        let mut seed = seed;
        for section in &sections {
            for &(destination, source, range) in section {
                if source <= seed && seed < source + range {
                    seed = seed - source + destination;
                    break;
                }
            }
        }

        if seed < min {
            min = seed;
        }
    }

    println!("Part 1: {}", min);
}

fn part_2(input: String) {
    let seeds_ranges = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let seed_sets = seeds_ranges.chunks(2).collect::<Vec<&[u32]>>();

    let mut sections = Vec::new();
    let mut section = Vec::new();

    for line in input.lines().skip(2) {
        if line.is_empty() {
            sections.push(section.clone());
            continue;
        }

        if line.ends_with(':') {
            section = Vec::new();
            continue;
        }

        let line_data = line
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        section.push((line_data[0], line_data[1], line_data[2]));
    }

    sections.push(section.clone());

    let mut min = u32::MAX;

    for set in seed_sets {
        let (start, range) = (set[0], set[1]);

        for seed in start..(start + range) {
            let mut seed = seed;

            for section in &sections {
                for &(destination, source, range) in section {
                    if source <= seed && seed < source + range {
                        seed = seed - source + destination;
                        break;
                    }
                }
            }

            if seed < min {
                min = seed;
            }
        }
    }

    println!("Part 2: {}", min);
}
