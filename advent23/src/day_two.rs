use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct DiceSet {
    r: usize,
    g: usize,
    b: usize,
    power: usize
}

const RED_COUNT: usize = 12;
const GREEN_COUNT: usize = 13;
const BLUE_COUNT: usize = 14;

pub fn calculate_valid_games(file: File, debug: bool) -> u64 {

    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    for line in file_lines.flatten() {
        // final_result += is_game_valid(line, debug);
        final_result += get_total_power_of_mins(line, debug);
    }

    return final_result;
}

fn is_game_valid(line: String, debug: bool) -> u64 {
    if debug {
        println!("Game in review: {}", &line);
    }

    let game_data:Vec<&str> = line.split(":").collect();

    if is_each_subset_valid(String::from(game_data[1]), debug) {
        if debug {
            println!("Valid game: {:?}" , &line);
        }
        let game_name:Vec<&str> = game_data[0].split_whitespace().collect();
        return String::from(game_name[1]).parse::<u64>().unwrap();
    } else {
        if debug {
            println!("invalid game: {:?}" , &line);
        }
    }
    return 0;
}

fn get_total_power_of_mins(line: String, debug: bool) -> u64 {
    let game_data:Vec<&str> = line.split(":").collect();

    let minimums = get_minimum_set(String::from(game_data[1]), debug);

    if debug {
        println!("Minimum set is: {:?}", &minimums );
    }

    return minimums.power.to_owned() as u64;
}

fn is_each_subset_valid(game_data: String, debug: bool) -> bool {
    let games_played: Vec<&str> = game_data.split(";").collect();
    for gp in games_played {
        let sub_game:Vec<&str> = gp.split(",").collect();
        if debug {
            println!("Sub game: {:?}", &sub_game);
        }
        for sg in sub_game {
            let dice_count:Vec<&str> = sg.split_whitespace().collect();

            if debug {
                println!("{:?}", dice_count);
            }

            match dice_count[1] {
                "blue" => {
                    if dice_count[0].parse::<usize>().unwrap() > BLUE_COUNT {
                        return false;
                    }
                },
                "red" => {
                    if dice_count[0].parse::<usize>().unwrap() > RED_COUNT {
                        return false;
                    }
                },
                "green" => {
                    if dice_count[0].parse::<usize>().unwrap() > GREEN_COUNT {
                        return false;
                    }
                },
                _ => println!("Found an unknown color {}", dice_count[1])
            }
        }

    }


    return true;
}

fn get_minimum_set(game_data: String, debug: bool) -> DiceSet {
    let mut result = DiceSet {
        r: 0,
        g: 0,
        b: 0,
        power: 0
    };

    let games_played: Vec<&str> = game_data.split(";").collect();
    for gp in games_played {
        let sub_game:Vec<&str> = gp.split(",").collect();
        if debug {
            println!("Sub game: {:?}", &sub_game);
        }
        for sg in sub_game {
            let dice_count:Vec<&str> = sg.split_whitespace().collect();

            if debug {
                println!("{:?}", dice_count);
            }

            match dice_count[1] {
                "blue" => {
                    if dice_count[0].parse::<usize>().unwrap() > result.b {
                        result.b = dice_count[0].parse::<usize>().unwrap();
                    }
                },
                "red" => {
                    if dice_count[0].parse::<usize>().unwrap() > result.r {
                        result.r = dice_count[0].parse::<usize>().unwrap();
                    }
                },
                "green" => {
                    if dice_count[0].parse::<usize>().unwrap() > result.g {
                        result.g = dice_count[0].parse::<usize>().unwrap();
                    }
                },
                _ => println!("Found an unknown color {}", dice_count[1])
            }
        }

    }

    result.power = result.b * result.g * result.r;

    if debug {
        println!("Counts: {:?}", &result);
    }

    return result;
}