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
    let mut left_vec: Vec<u32> = Vec::new();
    let mut right_vec: Vec<u32> = Vec::new();
    for line in file_lines.flatten() {
        if line.len() > 0{
            let vec: Vec<&str> = line.split(' ').collect();
            left_vec.push(vec[0].parse().expect("failed to get number for left"));
            right_vec.push(vec[1].parse().expect("failed to get number for right"));
        }
    }
    // step two, loop through the list size, adding up the differences. have to account for negative.

    left_vec.iter

    return final_result;
}
