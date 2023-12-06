fn main() {
    let day = 5;
    let input = aoc_2023::get_input(day, false);
    println!("Day {}", day);
    part_1(input.clone());
    part_2(input.clone());
}

fn part_1(input: String) {
    let answer;

    let mut seeds = input
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

    for seed in &mut seeds {
        for section in &sections {
            for &(destination, source, range) in section {
                if source <= *seed && *seed < source + range {
                    *seed = *seed - source + destination;
                    break;
                }
            }
        }
    }

    answer = seeds.iter().min().unwrap();

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let answer;

    let seeds_ranges = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut seed_sets = seeds_ranges.chunks(2).collect::<Vec<&[u64]>>();

    let mut seeds = Vec::new();

    for seed_set in &mut seed_sets {
        let r =
            (seed_set[0]..(seed_set[0] + seed_set[1])).collect::<Vec<u64>>();
        seeds.append(&mut r.clone());
    }

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
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        section.push((line_data[0], line_data[1], line_data[2]));
    }

    sections.push(section.clone());

    for seed in &mut seeds {
        for section in &sections {
            for &(destination, source, range) in section {
                if source <= *seed && *seed < source + range {
                    *seed = *seed - source + destination;
                    break;
                }
            }
        }
    }

    answer = seeds.iter().min().unwrap();

    println!("Part 2: {}", answer);
}
