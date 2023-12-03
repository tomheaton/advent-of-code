fn main() {
    let day = 3;
    let input = aoc_2023::get_input(day, true);
    println!("Day {}", day);
    part_1(input.clone());
    part_2(input.clone());
}

fn part_1(input: String) {
    let mut answer = 0;

    // Debug
    let mut numbers = Vec::new();

    let chars = input
        .lines()
        .flat_map(|line| line.chars())
        .filter(|c| *c != '\n')
        .collect::<Vec<char>>();

    for (y, line) in input.lines().enumerate() {
        let y_val = y as i32;
        println!("{}", line);

        let mut number_str = String::new();
        let mut is_part_number = false;

        for (x, c) in line.chars().enumerate() {
            let x_val = x as i32;
            // println!("{}", c);

            if c.is_numeric() {
                number_str.push(c);

                println!("checking ({}, {})", x_val, y_val);

                let mut coords = vec![
                    // left
                    (x_val - 1, y_val),
                    // right
                    (x_val + 1, y_val),
                    // top
                    (x_val, y_val + 1),
                    // bottom
                    (x_val, y_val - 1),
                    // top right
                    (x_val + 1, y_val + 1),
                    // bottom left
                    (x_val - 1, y_val - 1),
                    // top left
                    (x_val - 1, y_val + 1),
                    // bottom right
                    (x_val + 1, y_val - 1),
                ];

                coords.retain(|(x, y)| {
                    *x >= 0
                        && *x < line.len() as i32
                        && *y >= 0
                        && *y < input.lines().count() as i32
                });

                for coord in coords.iter() {
                    let index =
                        coord.1 as usize * line.len() + coord.0 as usize;
                    let character = chars[index];
                    println!("{:?}: {}", coord, character);

                    if character != '.' && !character.is_numeric() {
                        println!("{} is a part number", number_str);
                        is_part_number = true;
                        break;
                    } else {
                        println!("{} is not a part number", number_str);
                        // is_part_number = false;
                    }
                }
            } else {
                if number_str.len() == 0 {
                    continue;
                }

                if is_part_number {
                    let number = number_str.parse::<i32>().unwrap();
                    println!("adding {}", number);
                    answer += number;

                    // Debug
                    numbers.push(number);
                }

                number_str = String::new();
                println!("resetting number_str");
                is_part_number = false;
            }
        }

        // TODO: process number at end of line
    }

    // Debug
    println!("{:?}", numbers);

    println!("Part 1: {}", answer);
}

struct Number {
    pub string: String,
    pub is_part: bool,
}

fn part_1_test(input: String) {
    let mut answer = 0;

    // Debug
    let mut numbers = Vec::new();

    let chars = input
        .lines()
        .flat_map(|line| line.chars())
        .filter(|c| *c != '\n')
        .collect::<Vec<char>>();

    for (y, line) in input.lines().enumerate() {
        let y_val = y as i32;
        println!("{}", line);

        let mut number_str = String::new();
        let mut is_part_number = false;

        for (x, c) in line.chars().enumerate() {
            let x_val = x as i32;
            // println!("{}", c);

            if c.is_numeric() {
                number_str.push(c);

                println!("checking ({}, {})", x_val, y_val);

                let mut coords = vec![
                    // left
                    (x_val - 1, y_val),
                    // right
                    (x_val + 1, y_val),
                    // top
                    (x_val, y_val + 1),
                    // bottom
                    (x_val, y_val - 1),
                    // top right
                    (x_val + 1, y_val + 1),
                    // bottom left
                    (x_val - 1, y_val - 1),
                    // top left
                    (x_val - 1, y_val + 1),
                    // bottom right
                    (x_val + 1, y_val - 1),
                ];

                coords.retain(|(x, y)| {
                    *x >= 0
                        && *x < line.len() as i32
                        && *y >= 0
                        && *y < input.lines().count() as i32
                });

                for coord in coords.iter() {
                    let index =
                        coord.1 as usize * line.len() + coord.0 as usize;
                    let character = chars[index];
                    println!("{:?}: {}", coord, character);

                    if character != '.' && !character.is_numeric() {
                        println!("{} is a part number", number_str);

                        numbers.push(Number {
                            string: number_str.clone(),
                            is_part: true,
                        });
                        break;
                    } else {
                        println!("{} is not a part number", number_str);
                    }
                }
            } else {
                if number_str.len() == 0 {
                    continue;
                }

                number_str = String::new();
                println!("resetting number_str");
            }
        }
    }

    let numbers_values = numbers
        .iter()
        .filter(|n| n.is_part)
        .map(|n| n.string.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("{:?}", numbers_values);

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    println!("Part 2: {}", answer);
}
