use std::time::Instant;

fn main() {
    let day = 8;
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

    let instructions = input.lines().nth(0).unwrap();

    let mut nodes = Vec::new();

    for line in input.lines().skip(2) {
        let data = line.split(" = (").collect::<Vec<&str>>();
        let head = data[0];
        let tail = data[1];

        let tail_nodes = tail.split(", ").collect::<Vec<&str>>();

        let a = tail_nodes[0].to_string();
        let mut b = tail_nodes[1].to_string();
        b.pop();

        nodes.push((head.to_string(), (a, b)));
    }

    let mut current_node = nodes[answer].clone();
    // println!("Initial node: {:?}", current_node);

    loop {
        // println!("Current node: {:?}", current_node);

        if current_node.0 == "ZZZ" {
            println!("Found ZZZ");
            break;
        }

        let instruction = instructions
            .chars()
            .nth(answer % instructions.chars().count())
            .unwrap();

        match instruction {
            'L' => {
                // println!("Left");

                let id = (current_node.clone().1).0;
                // println!("moving to {}", id);

                'lol: for node in nodes.iter() {
                    if node.0 == id {
                        current_node = node.clone();
                        break 'lol;
                    }
                }
            }
            'R' => {
                // println!("Right");

                let id = (current_node.clone().1).1;
                // println!("moving to {}", id);

                'lol: for node in nodes.iter() {
                    if node.0 == id {
                        current_node = node.clone();
                        break 'lol;
                    }
                }
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }

        answer += 1;
    }

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    println!("Part 2: {}", answer);
}
