use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("Hello, world!");

    let puzzle_input: String = String::from("puzzle_one_input.txt");
    let debug: bool = false;

    let read_result = read_file(puzzle_input);

    match read_result {
        Ok(file) => {
            println!("Final result = {}", calculate_result(file, debug));
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}


/**
Day 1 Notes

first digit + last digit in this order to combine new number = String Concat
Additionally, these are ACTUAL digits in the string

Need to load an entire document and find the total sum

Code design:
1. be able to read a file of the given sample
2. print out each line of the sample
3. add a string parser to find the first digit.
4. update the string parser to go backwards, find the first digit it hits
5. return the combined string value as the result
6. While looping for each line, parse the result and add to a running total
7. print out the final result

*/

fn calculate_result(file: File, debug: bool) -> u64 {
    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    for line in file_lines.flatten() {
        //println!("=====");
        if debug {
            println!("Next Line : {:?} = {:?}", &line, find_first_and_last_digit(&line, debug).parse::<u64>().unwrap());
        }
        final_result += find_first_and_last_digit(&line, debug).parse::<u64>().unwrap();
        //break;
        //final_result += usize::try_from(find_first_and_last_digit(line)).unwrap();
    }

    return final_result;
}

fn find_first_and_last_digit(line: &String, debug: bool) -> String {
    let mut first:Option<char> = None;
    let mut last:Option<char> = None;
    let characters_list:Vec<char> = line.chars().collect();

    // Find the first value
    if debug {
        println!("looking for first");
    }
    for i in 0..characters_list.len() {
        first = get_valid_digit(&characters_list, i, debug);
        if let Some(_) = first {
            break;
        }
    }

    // Find the last value
    if debug {
        println!("looking for last");
    }
    for i in 0..characters_list.len() {
        last = get_valid_digit(&characters_list, characters_list.len() - 1 - i, debug);
        if let Some(_) = last {
            break;
        }
    }

    if None == first {
        println!("Nothing was found for the first value, {}", line);
        return "0".to_string();
    }
    if debug {
        println!("First = {:?}", &first);
    }

    if None == last {
        println!("Nothing was found for the last value, {}", line);
        return "0".to_string();
    }
    if debug {
        println!("Last = {:?}", &last);
    }
    return format!("{}{}", first.unwrap(), last.unwrap());
}

pub struct CharSetup<'a> {
    s: &'a str,
    v: char
}

const CHAR_3: [CharSetup; 3] = [
    CharSetup {s: "one", v:'1'},
    CharSetup {s:"two", v:'2'},
    CharSetup {s:"six", v:'6'}];

const CHAR_4: [CharSetup; 3] = [
    CharSetup {s:"four", v:'4'},
    CharSetup {s:"five", v:'5'},
    CharSetup {s:"nine", v:'9'}];

const CHAR_5: [CharSetup; 3] = [
    CharSetup {s:"three", v:'3'},
    CharSetup {s:"seven", v:'7'},
    CharSetup {s:"eight", v:'8'}];

fn get_valid_digit(characters: &Vec<char>, index: usize, debug:bool) -> Option<char> {
    if debug {
        println!("Index: {}", &index);
    }

    if index < characters.len() && characters.get(index).unwrap().is_digit(10) {
        return Some(characters.get(index).unwrap().to_owned());
    }

    if characters.len() >= 3 && index <= characters.len() - 3 {
        if debug {
            println!("{:?}", &characters[index..=(index+2)]);
        }
        for i in (0..3) {
            if CHAR_3[i].s == String::from_iter(&characters[index..=(index+2)]).as_str() {
                return Some(CHAR_3[i].v.to_owned());
            }
        }
    }

    if characters.len() >= 4 && index <= characters.len() - 4 {
        if debug {
            println!("{:?}", &characters[index..=(index+3)]);
        }
        for i in (0..3) {
            if CHAR_4[i].s == String::from_iter(&characters[index..=(index+3)]).as_str() {
                return Some(CHAR_4[i].v.to_owned());
            }
        }
    }

    if characters.len() >= 5 && index <= characters.len() - 5{
        if debug {
            println!("{:?}", &characters[index..=(index+4)]);
        }
        for i in (0..3) {
            if CHAR_5[i].s == String::from_iter(&characters[index..=(index+4)]).as_str() {
                return Some(CHAR_5[i].v.to_owned());
            }
        }
    }

    return None;
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