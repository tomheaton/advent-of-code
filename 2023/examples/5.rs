fn main() {
    let day = 5;
    let input = aoc_2023::get_input(day, false);
    println!("Day {}", day);
    part_1(input.clone());
    part_2(input.clone());
}

#[derive(Debug, Clone, Copy)]
struct Seed {
    pub seed: i64,
    pub soil: i64,
}

impl Seed {
    pub fn new(seed: i64) -> Seed {
        return Seed { seed, soil: seed };
    }
}

fn part_1(input: String) {
    let answer;

    let seeds_line = input.lines().next().unwrap();
    let seeds_line_data = seeds_line.split(':').collect::<Vec<&str>>();

    let mut seeds = seeds_line_data[1]
        .trim()
        .split(' ')
        .map(|x| Seed::new(x.parse::<i64>().unwrap()))
        .collect::<Vec<Seed>>();

    // println!("seeds:");
    // for seed in seeds.clone() {
    //     println!("{:?}", seed);
    // }

    let mut sections = Vec::new();
    let mut section = Vec::new();

    for line in input.lines().skip(2) {
        if line.is_empty() {
            // println!("end of section");
            sections.push(section.clone());
            continue;
        }
        if line.ends_with(':') {
            // println!("new section");
            section = Vec::new();
            continue;
        }

        let line_data = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        section.push((line_data[0], line_data[1], line_data[2]));
    }

    // println!("end of section");
    sections.push(section.clone());

    println!("processing...");

    for seed in seeds.iter_mut() {
        for section in sections.clone() {
            // println!("section: {:?}", section);

            for set in section.clone() {
                // println!("set: {:?}", set);

                let (destination, source, range) = set;

                if source <= seed.seed && seed.seed < source + range {
                    seed.soil -= source;
                    seed.soil += destination;

                    // println!("{} -> {}", seed.seed, seed.soil);

                    seed.seed = seed.soil;

                    break;
                }
            }
        }
    }

    // println!("seeds:");
    // for seed in seeds.clone() {
    //     println!("{:?}", seed);
    // }

    answer = seeds
        .iter()
        .min_by(|a, b| a.seed.cmp(&b.seed))
        .unwrap()
        .seed;

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    println!("Part 2: {}", answer);
}
