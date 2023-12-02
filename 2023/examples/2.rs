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
        // println!("{:?}", game);

        // TODO: game id is just index + 1 so this is unnecessary
        let game_id_str = game[0].split(" ").collect::<Vec<&str>>()[1];
        let game_id = str::parse::<i32>(&game_id_str).unwrap();
        println!("\nGame: {:?}", game_id);

        let sets = game[1].split("; ").collect::<Vec<&str>>();
        // println!("{:?}", sets);

        let mut impossible = false;

        for set in sets.iter() {
            println!("set: {:?}", set);

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
            println!("Game {} is possible", game_id);
            answer += game_id;
        }
    }

    println!("Part 1: {}", answer);
}

fn part_2() {
    let input = aoc_2023::get_input(2, true);

    let mut answer = 0;

    println!("Part 2: {}", answer);
}
