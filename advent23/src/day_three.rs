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

/***
Issues observed.

1. Last elements are not properly handled
 */
fn calculate_row_value(previous: Option<&String>, current: &String, next: Option<&String>, debug: bool) -> u64 {

    let mut row_total: u64 = 0;

    let mut start_index:usize = 0;
    let mut integer_start:usize = 0;
    let mut integer_end:usize = 0;
    let mut end_index:usize = 0;
    let mut skip_count:usize = 0;

    //iterate through current until first digit
    let row_chars:Vec<char> = current.chars().collect();

    for char in 0..row_chars.len() {

        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if row_chars.get(char).unwrap().is_digit(10) {
            // find the length of the number
            integer_start = char;
            if char > 0 {
                start_index = integer_start -1;
            }

            for i in 0..5 {
                if !(row_chars.get(char + i).unwrap().is_digit(10)) {
                    break;
                }
                integer_end = char + i;
                skip_count += 1;
            }
            // find start and end indexes (start-1, end+1) for diagonal
            if integer_end < (row_chars.len() -1) {
                end_index = integer_end + 1;
            }
            // search previous and next for any symbols
            if are_symbols_on_row(previous, start_index, end_index) ||
                are_symbols_on_row(Some(current), start_index, end_index) ||
                are_symbols_on_row(next, start_index, end_index) {
                // Since we have the number, add it to the running total
                let actual_number_chars = &row_chars[(integer_start)..=(integer_end)];
                let string_rep: String = actual_number_chars.iter().collect();
                if debug { println!("Adding: {}", string_rep); }
                row_total += String::from(string_rep).parse::<u64>().unwrap();
            }
            // if no, return 0
            row_total += 0; //Actual number found
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