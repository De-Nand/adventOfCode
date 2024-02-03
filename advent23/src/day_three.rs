use std::f32::DIGITS;
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
 */

fn calculate_gear_ratio(previous: Option<&String>, current: &String, next: Option<&String>, debug: bool) -> u64 {

    let mut row_total: u64 = 0;

    let mut symbol_index_start:usize = 0;
    let mut symbol_index_end:usize = 0;
    let mut numbers_found: i8 = 0;
    let mut num_one: usize = 1;
    let mut num_two: usize = 1;
    let mut num_three: usize = 1;

    //iterate through current until first digit
    let row_chars:Vec<char> = current.chars().collect();

    for char in 0..row_chars.len() {
        if char > 0 {
            symbol_index_start = char -1;
        }

        if row_chars[char] == '*' {
            // Need to see if it is connected on all sides to a number (is_digit)
            if is_number_nearby(previous, symbol_index_start, (char+1)) {
                numbers_found += 1;
            }
            if is_number_nearby(Some(current), symbol_index_start, (char+1)) {
                numbers_found += 1;
            }
            if is_number_nearby(next, symbol_index_start, (char+1)) {} {
                numbers_found += 1;
            }
        }

        if numbers_found >= 2 {
            if debug { println!("Found a gear ratio : {}, {}, {}", num_one, num_two, num_three)}
            row_total += num_one * num_two * num_three;
        }
    }

    return row_total;
}

fn is_number_nearby(row: Option<&String>, start: usize, end: usize) -> bool {
    if None == row {
        return false;
    }

    let row_chars:Vec<char> = row.unwrap().chars().collect();

    for i in start..=end {
        if i < row_chars.len() && row_chars[i].is_digit(10) {
            return true;
        }
    }

    return false;
}


fn calculate_row_value(previous: Option<&String>, current: &String, next: Option<&String>, debug: bool) -> u64 {

    let mut row_total: u64 = 0;

    let mut symbol_index_start:usize = 0;
    let mut integer_start:usize = 0;
    let mut integer_end:usize = 0;
    let mut symbol_index_end:usize = 0;
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
                symbol_index_start = integer_start -1;
            }

            for i in 0..5 {
                if (char + i) >= row_chars.len() { break; }
                if !(row_chars.get(char + i).unwrap().is_digit(10)) { break; }

                skip_count += 1;
                integer_end = char + i;
            }

            // find start and end indexes (start-1, end+1) for diagonal
            if integer_end <= (row_chars.len() -1) {
                symbol_index_end = integer_end + 1;
            }

            if symbol_index_end <= symbol_index_start {
                println!("current attributes: {}, {}", integer_start, integer_end);
                panic!("The end index should not be less than or equal to the start \
                | {}, {}", symbol_index_start, symbol_index_end);
            }

            // search previous and next for any symbols
            if are_symbols_on_row(previous, symbol_index_start, symbol_index_end) ||
                are_symbols_on_row(Some(current), symbol_index_start, symbol_index_end) ||
                are_symbols_on_row(next, symbol_index_start, symbol_index_end) {
                // Since we have the number, add it to the running total
                let actual_number_chars = &row_chars[(integer_start)..=(integer_end)];
                let string_rep: String = actual_number_chars.iter().collect();
                if debug { println!("Adding: {}", string_rep); }
                row_total += String::from(string_rep).parse::<u64>().unwrap();
            } else {
                let actual_number_chars = &row_chars[(integer_start)..=(integer_end)];
                let string_rep: String = actual_number_chars.iter().collect();
                if debug { println!("Found {} to not be next to attributes", string_rep)}
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
        if i < row_chars.len() && !row_chars[i].is_digit(10) && row_chars[i] != '.' {
            return true;
        }
    }

    return false;
}