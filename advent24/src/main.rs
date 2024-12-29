use advent23::day_one::calculate_result;
use std::fs::File;

fn main() {
    let debug: bool = true;
    let puzzle_day: u8 = 1;
    let puzzle_sample: bool = false;

    let read_result = read_file(puzzle_sample, puzzle_day);
    match read_result {
        Ok(file) => match puzzle_day {
            1 => println!("Final result = {}", calculate_result(file, debug)),
            _ => println!("No valid day selected"),
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn read_file(sample: bool, day: u8) -> Result<File, String> {
    let location: String;
    if sample {
        location = format!("supp_docs/day{}/sample.txt", day);
    } else {
        location = format!("supp_docs/day{}/puzzle.txt", day);
    }
    let file = File::open(location);

    match file {
        Ok(f) => {
            return Ok(f);
        }
        Err(e) => {
            return Err(format!("Failed to find the file {:?}", e));
        }
    }
}
