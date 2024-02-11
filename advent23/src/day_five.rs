use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn calculate_day_five(file: File, debug: bool) -> u64 {
    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;


    file_lines.flatten().for_each(|line| {
        counter = line_result(line, debug);
    });

    for sc in scratch_cards {
        final_result += *sc as u64;
    }

    return final_result;
}

fn line_result(current_line: String, debug: bool) -> usize {
    return 0;
}
