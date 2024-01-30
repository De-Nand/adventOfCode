use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Lines;

pub fn calculate_day_three(file: File, debug: bool) -> u64 {

    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    let list:Vec<String> = file_lines.flatten().collect();

    for line in 0..list.len() {
        println!("Looking at: {:?}", &list.get(line));

        if line == 0 {
            final_result += calculate_row_value(None, &list[line], Some(&list[line+1]), debug);
        } else if line == list.len() -1 {
            final_result += calculate_row_value(Some(&list[line-1]), &list[line], None, debug);
        } else {
            final_result += calculate_row_value(Some(&list[line-1]), &list[line], Some(&list[line+1]), debug);
        }
    }

    return final_result;
}

fn calculate_row_value(previous: Option<&String>, current: &String, next: Option<&String>, debug: bool) -> u64 {

    let mut row_total: u64 = 0;
    let mut start_index = 0;
    let mut end_index = 0;

    //iterate through current until first digit
    let row_chars:Vec<char> = current.chars().collect();

    for char in 0..row_chars.len() {
        if row_chars.get(char).unwrap().is_digit(10) {
            // find the length of the number
            if char > 0 {
                start_index = char -1;
            }
            for i in 0..5 {
                if !(row_chars.get(char + i).unwrap().is_digit(10)) {
                    break;
                }
                end_index = char + i;
            }
            // find start and end indexes (start-1, end+1) for diagonal
            end_index += 1;
            // search previous and next for any symbols
            if are_symbols_on_row(previous, start_index, end_index) || are_symbols_on_row(next, start_index, end_index) {
                row_total += 0; //Actual number found
            }
            // if yes, return the number, else 0

        }
    }


    return row_total;
}

fn are_symbols_on_row(row: Option<&String>, start: usize, end: usize) -> bool {
    if None == row {
        return false;
    }

    let row_chars:Vec<char> = row.unwrap().chars().collect();

    for i in start..=end {
        if !row_chars[i].is_digit(10) && row_chars[i] != '.' {
            return true;
        }
    }

    return false;
}