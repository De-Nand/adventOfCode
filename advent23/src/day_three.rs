use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calculate_day_three(file: File, debug: bool) -> u64 {

    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    let list:Vec<String> = file_lines.flatten().collect();

    for line in 0..list.len() {
        //if debug { println!("Looking at: {:?}", &list.get(line)); }

        if line == 0 {
            final_result += calculate_gear_ratio(None, &list[line], Some(&list[line+1]), debug);
        } else if line == list.len() -1 {
            final_result += calculate_gear_ratio(Some(&list[line-1]), &list[line], None, debug);
        } else {
            final_result += calculate_gear_ratio(Some(&list[line-1]), &list[line], Some(&list[line+1]), debug);
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
    let mut num_one: u64 = 1;
    let mut num_two: u64 = 1;
    let mut num_three: u64 = 1;

    //iterate through current until first digit
    let row_chars:Vec<char> = current.chars().collect();

    for char in 0..row_chars.len() {
        symbol_index_start = char;

        if row_chars[char] == '*' {
            // Need to see if it is connected on all sides to a number (is_digit)
            let top = is_number_nearby(previous, symbol_index_start, debug); //top row
            let mid = is_number_nearby(Some(current), symbol_index_start, debug); //current row
            let bot = is_number_nearby(next, symbol_index_start, debug); //bottom row

            if top.0 > 0 {
                if debug { println!("top Found {} numbers that multiply up to {}", top.0, top.1); }
                num_one = num_one * top.1;
                numbers_found += top.0;
            }
            if mid.0 > 0 {
                if debug { println!("mid Found {} numbers that multiply up to {}", mid.0, mid.1); }
                num_two = num_two * mid.1;
                numbers_found += mid.0;
            }
            if bot.0 > 0 {
                if debug { println!("bot Found {} numbers that multiply up to {}", bot.0, bot.1); }
                num_three = num_three * bot.1;
                numbers_found += bot.0;
            }

            if numbers_found >= 2 {
                if debug { println!("Found a gear ratio : {}, {}, {} = {}", num_one, num_two, num_three, num_one * num_two * num_three) }
                row_total += num_one * num_two * num_three;
            }
            num_one = 1;
            num_two = 1;
            num_three = 1;
            numbers_found = 0;
        }
    }

    return row_total;
}

fn is_number_nearby(row: Option<&String>, start: usize, debug: bool) -> (i8, u64) {
    if None == row {
        return (0, 0);
    }

    let row_chars:Vec<char> = row.unwrap().chars().collect();

    if row_chars[start].is_digit(10) {
        // Only 1 number possible flow
        if debug { println!("Single number option"); }
        let individual_res = get_number_in_range(&row_chars, start, debug);
        if individual_res.0 {
            return (1, individual_res.1)
        }
        return (0, 1);
    } else {
        if debug { println!("Multi number option"); }
        // max 2 numbers possible flow
        let mut count: i8 = 0;
        let mut result: u64 = 1;

        if start > 0 {
            let left = get_number_in_range(&row_chars, (start-1), debug);
            if left.0 {
                result = result * left.1;
                count += 1;
            }
        }

        if start < row_chars.len() {
            let right = get_number_in_range(&row_chars, (start+1), debug);
            if right.0 {
                result = result * right.1;
                count += 1;
            }
        }

        return (count, result);
    }
}

fn get_number_in_range(row: &Vec<char>, start: usize, debug: bool) -> (bool, u64) {
    if debug { println!("First number is {}", row[start])}
    if !(row[start].is_digit(10)) {
        return (false, 1)
    }
    let mut new_start: usize = start.clone();
    let mut new_end: usize = start.clone();

    // go to the left
    while row[new_start].is_digit(10) {
        if new_start > 0 && row[new_start-1].is_digit(10) {
            new_start -= 1;
        } else {
            break;
        }
    }

    // go to the right
    while row[new_end].is_digit(10) {
        if new_end < row.len() && row[new_end+1].is_digit(10) {
            new_end += 1;
        } else {
            break;
        }
    }

    // Using the new index, send back the result
    let actual_number_chars = &row[(new_start)..=(new_end)];
    let string_rep: String = actual_number_chars.iter().collect();
    if debug { println!("collection of digits is : {}" , &string_rep) }
    return (true, String::from(string_rep).parse::<u64>().unwrap())
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