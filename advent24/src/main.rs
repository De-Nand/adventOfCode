use advent23::day_one::calculate_result;
use std::fs::File;

fn main() {
    let debug: bool = true;
    let puzzle_day = 1;
    let puzzle_sample: bool = true;

    let puzzle_input: String = get_file_to_read(puzzle_day, puzzle_sample);

    let read_result = read_file(puzzle_input);

    match read_result {
        Ok(fill) => match puzzle_day {
            1 => println!("Final result = {}", calculate_result(file, debug)),
            _ => println!("No valid day selected"),
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
        }
        Err(e) => {
            return Err(format!("Failed to find the file {:?}", e));
        }
    }
}

fn get_file_to_read(day: number, sample: bool) -> String {
    if sample {
        return format!("day_{}_sample", day);
    }
    return format!("day_{}", day);
}
