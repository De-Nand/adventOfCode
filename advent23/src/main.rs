use std::fs::File;
use advent23::day_one::calculate_result;
use advent23::day_two::calculate_valid_games;
use advent23::day_three::calculate_day_three;
use advent23::day_four::calculate_day_four;
use advent23::day_five::calculate_day_five;

fn main() {
    println!("Hello, world!");
//seeds: 3429320627 235304036 1147330745 114559245 1684000747 468955901 677937579 96599505 1436970021 26560102 3886049334 159534901 936845926 25265009 3247146679 95841652 3696363517 45808572 2319065313 125950148
    let puzzle_input: String = String::from("puzzle_five_sample.txt");
    let debug: bool = true;
    let puzzle_day = 5;

    let read_result = read_file(puzzle_input);

    match read_result {
        Ok(file) => {
            match puzzle_day {
                1 => println!("Final result = {}", calculate_result(file, debug)),
                2 => println!("Result = {}", calculate_valid_games(file, debug)),
                3 => println!("Result = {}", calculate_day_three(file, debug)),
                4 => println!("Result = {}", calculate_day_four(file, debug)),
                5 => println!("Result = {}", calculate_day_five(file, debug)),
                _ => println!("No valid day selected")
            }


        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn read_file(file_name: String) -> Result<File, String> {
    let location: String = format!("supp_docs/{}", file_name);
    let file = File::open(location);

    match file {
        Ok(f) => {
            return Ok(f);
        },
        Err(e) => {
            return Err(format!("Failed to find the file {:?}", e));
        }
    }
}