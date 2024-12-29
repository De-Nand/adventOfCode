use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
Day 1 Notes

    smallest on left
    smallest on right

    find the delta

    add all

 */

pub fn calculate_result(file: File, debug: bool) -> u64 {
    let file_lines = BufReader::new(file).lines();
    let mut final_result: u64 = 0;

    // step one, build 2 sorted lists
    let mut left_vec: Vec<u64> = Vec::new();
    let mut right_vec: Vec<u64> = Vec::new();

    for line in file_lines.flatten() {
        if line.len() > 0 {
            let vec: Vec<&str> = line.split(' ').filter(|x| x.len() > 0).collect();
            if debug {
                println!("{:?}", &vec);
            }
            left_vec.push(vec[0].parse().expect("failed to get number for left"));
            right_vec.push(vec[1].parse().expect("failed to get number for right"));
        }
    }
    // Step 2 - Sort the lists
    left_vec.sort();
    right_vec.sort();

    // step two, loop through the list size, adding up the differences. have to account for negative.
    let mut left_val: u64;
    let mut right_val: u64;
    for index in 0..left_vec.len() {
        left_val = left_vec[index];
        right_val = right_vec[index];

        if debug {
            println!("Comparing {:?} and {:?}", &left_val, &right_val);
        }

        if left_val > right_val {
            final_result += left_val - right_val;
        } else {
            final_result += right_val - left_val;
        }
    }

    return final_result;
}
