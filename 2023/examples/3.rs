fn main() {
    let day = 3;
    let input = aoc_2023::get_input(day, false);
    println!("Day {}", day);
    part_1(input.clone());
    part_2(input.clone());
}

// 75741499

struct PartNumber {
    pub number_str: String,
    pub start_x: i32,
    pub start_y: i32,
    pub is_part: bool,
    pub gears: Vec<(i32, i32)>,
}

impl PartNumber {
    fn new() -> PartNumber {
        return PartNumber {
            number_str: String::new(),
            start_x: 0,
            start_y: 0,
            is_part: false,
            gears: Vec::new(),
        };
    }
}

fn part_1(input: String) {
    let mut answer = 0;

    let chars = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();

    let mut part_numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_part_number = PartNumber::new();

        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if current_part_number.number_str.len() == 0 {
                    current_part_number.start_x = x as i32;
                    current_part_number.start_y = y as i32;
                }

                current_part_number.number_str.push(c);

                continue;
            }

            if current_part_number.number_str.len() == 0 {
                continue;
            }

            part_numbers.push(current_part_number);
            current_part_number = PartNumber::new();
        }

        // add the last part number of the line
        if current_part_number.number_str.len() > 0 {
            part_numbers.push(current_part_number);
        }
    }

    let line_width = input.lines().next().unwrap().len() as i32;

    for part_number in part_numbers.iter_mut() {
        let number_str_length = part_number.number_str.len();

        let mut coords = Vec::new();

        // top and bottom
        for x in
            part_number.start_x..part_number.start_x + number_str_length as i32
        {
            coords.push((x, part_number.start_y - 1));
            coords.push((x, part_number.start_y + 1));
        }

        // top left, left, bottom left, top right, right, bottom right
        for y in part_number.start_y - 1..part_number.start_y + 2 as i32 {
            coords.push((part_number.start_x - 1, y));
            coords.push((part_number.start_x + number_str_length as i32, y));
        }

        coords.retain(|(x, y)| {
            *x >= 0
                && *x < line_width
                && *y >= 0
                && *y < input.lines().count() as i32
        });

        for coord in coords {
            let index =
                coord.1 as usize * line_width as usize + coord.0 as usize;
            let character = chars[index];

            if character != '.' && !character.is_numeric() {
                part_number.is_part = true;
                break;
            }
        }
    }

    for part_number in part_numbers.iter() {
        if part_number.is_part {
            let number = part_number.number_str.parse::<i32>().unwrap();
            answer += number;
        }
    }

    println!("Part 1: {}", answer);
}

fn part_2(input: String) {
    let mut answer = 0;

    let chars = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();

    let mut part_numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_part_number = PartNumber::new();

        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if current_part_number.number_str.len() == 0 {
                    current_part_number.start_x = x as i32;
                    current_part_number.start_y = y as i32;
                }

                current_part_number.number_str.push(c);
                continue;
            }

            if current_part_number.number_str.len() == 0 {
                continue;
            }

            part_numbers.push(current_part_number);
            current_part_number = PartNumber::new();
        }

        // add the last part number of the line
        if current_part_number.number_str.len() > 0 {
            part_numbers.push(current_part_number);
        }
    }

    let line_width = input.lines().next().unwrap().len() as i32;

    for part_number in part_numbers.iter_mut() {
        let number_str_length = part_number.number_str.len();

        let mut coords = Vec::new();

        // top and bottom
        for x in
            part_number.start_x..part_number.start_x + number_str_length as i32
        {
            coords.push((x, part_number.start_y - 1));
            coords.push((x, part_number.start_y + 1));
        }

        // top left, left, bottom left, top right, right, bottom right
        for y in part_number.start_y - 1..part_number.start_y + 2 as i32 {
            coords.push((part_number.start_x - 1, y));
            coords.push((part_number.start_x + number_str_length as i32, y));
        }

        coords.retain(|(x, y)| {
            *x >= 0
                && *x < line_width
                && *y >= 0
                && *y < input.lines().count() as i32
        });

        for coord in coords {
            let index =
                coord.1 as usize * line_width as usize + coord.0 as usize;
            let character = chars[index];

            if character != '.' && !character.is_numeric() {
                part_number.is_part = true;
                part_number.gears.push(coord);
            }
        }
    }

    for i in 0..part_numbers.len() {
        for j in 0..part_numbers.len() {
            if i == j {
                continue;
            }

            let has_shared_parts =
                part_numbers[i].gears.iter().any(|&gear_a| {
                    part_numbers[j].gears.iter().any(|&gear_b| gear_a == gear_b)
                });

            if has_shared_parts {
                let number_a =
                    part_numbers[i].number_str.parse::<i32>().unwrap();
                let number_b =
                    part_numbers[j].number_str.parse::<i32>().unwrap();
                answer += number_a * number_b;
            }
        }
    }

    println!("Part 2: {}", answer / 2);
}
