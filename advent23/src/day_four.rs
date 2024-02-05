use std::fs::File;
use std::io::{BufRead, BufReader};

///
/// There are two sets of list per line. winning cards (left of the |) and your cards (right of the |)
/// need to make a match. collection vs collection, how many exist
/// each match after the first is a double of the value. I.e. 2 ^ (match - 1)
/// Finally, add each lines result
pub fn calculate_day_four(file: File, debug: bool) -> u64 {
    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    for line in file_lines.flatten() {
        final_result += line_result(line, debug);
    }

    return final_result;
}

fn line_result(current_line: String, debug: bool) -> u64 {
    let cards_result:Vec<&str> = current_line.split("|").collect();
    let winning_cards:Vec<&str> = cards_result[0].split(":").collect();

    let winning_list:Vec<&str> = winning_cards[1].split_whitespace().collect();
    let elves_list:Vec<&str> = cards_result[1].split_whitespace().collect();

    if debug { println!("Winning cards: {:?}", &winning_list)}
    if debug { println!("Elves cards: {:?}", &elves_list)}

    return compute_line_score(winning_list, elves_list, debug);
}

fn compute_line_score(winning_list:Vec<&str>, elves_list:Vec<&str>, debug: bool) -> u64 {
    let mut total_matches: usize = 0;
    let base:i32 = 2;

    winning_list.iter().into_iter().for_each(|wl| {
        if elves_list.contains(wl) {
            total_matches +=1;
        }
    });

    if debug { println!("Total Matches = {}", total_matches); }
    if total_matches == 0 {
        return 0;
    }

    return base.pow((total_matches - 1) as u32) as u64;
}