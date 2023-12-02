fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = aoc_2023::get_input(2, false);

    let mut answer = 0;

    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    for line in input.lines() {
        let game = line.split(": ").collect::<Vec<&str>>();

        // TODO: game id is just index + 1 so this is unnecessary
        let game_id_str = game[0].split(" ").collect::<Vec<&str>>()[1];
        let game_id = str::parse::<i32>(&game_id_str).unwrap();

        let sets = game[1].split("; ").collect::<Vec<&str>>();

        let mut impossible = false;

        for set in sets.iter() {
            let picks = set.split(", ").collect::<Vec<&str>>();

            for pick in picks.iter() {
                let data = pick.split(" ").collect::<Vec<&str>>();

                let cubes = str::parse::<i32>(&data[0]).unwrap();
                let colour = data[1];

                match colour {
                    "red" => {
                        if cubes > red_max {
                            impossible = true;
                        }
                    }
                    "green" => {
                        if cubes > green_max {
                            impossible = true;
                        }
                    }
                    "blue" => {
                        if cubes > blue_max {
                            impossible = true;
                        }
                    }
                    _ => panic!("Unknown colour"),
                }
            }
        }

        if !impossible {
            answer += game_id;
        }
    }

    println!("Part 1: {}", answer);
}

fn part_2() {
    let input = aoc_2023::get_input(2, false);

    let mut answer = 0;

    for line in input.lines() {
        let game = line.split(": ").collect::<Vec<&str>>();

        let sets = game[1].split("; ").collect::<Vec<&str>>();

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        for set in sets.iter() {
            let picks = set.split(", ").collect::<Vec<&str>>();

            for pick in picks.iter() {
                let data = pick.split(" ").collect::<Vec<&str>>();

                let cubes = str::parse::<i32>(&data[0]).unwrap();
                let colour = data[1];

                match colour {
                    "red" => {
                        if cubes > red_min {
                            red_min = cubes;
                        }
                    }
                    "green" => {
                        if cubes > green_min {
                            green_min = cubes;
                        }
                    }
                    "blue" => {
                        if cubes > blue_min {
                            blue_min = cubes;
                        }
                    }
                    _ => panic!("Unknown colour"),
                }
            }
        }

        answer += red_min * green_min * blue_min;
    }

    println!("Part 2: {}", answer);
}
