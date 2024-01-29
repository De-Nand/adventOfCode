use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calculate_day_three(file: File, debug: bool) -> u64 {

    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    for line in file_lines.flatten() {
        // final_result += is_game_valid(line, debug);
        final_result += methodname(line, debug);
    }

    return final_result;
}