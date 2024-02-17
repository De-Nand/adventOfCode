use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Copy, Clone)]
struct PlantingMap {
    pub source: usize,
    pub destination: usize,
    pub range: usize
}

#[derive(Debug, Clone)]
struct Groups {
    pub seed_to_soil: Vec<PlantingMap>,
    pub soil_to_fertilizer: Vec<PlantingMap>,
    pub fertilizer_to_water: Vec<PlantingMap>,
    pub water_to_light: Vec<PlantingMap>,
    pub light_to_temperature: Vec<PlantingMap>,
    pub temperature_to_humidity: Vec<PlantingMap>,
    pub humidity_to_location: Vec<PlantingMap>
}

pub fn calculate_day_five(file: File, debug: bool) -> u64 {
    // Get the lines
    let file_lines:Lines<BufReader<File>> = BufReader::new(file).lines();

    // Init all the necessary components
    let mut final_result: usize = 0;
    let mut seeds: Vec<usize> = vec![];
    let mut groups = Groups{
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    // Build out the group mappings
    build_groups(file_lines, &mut groups, &mut seeds, debug);

    // Then calculate the result
    final_result = solution_parse_each_value(groups, seeds, debug);

    if debug { println!("Results {:?}", final_result)}

    return final_result as u64;
}

fn solution_parse_each_value(groups: Groups, seeds: Vec<usize>, debug: bool) -> usize {
    let pairs = (seeds.len()) / 2;
    println!("Pairs: {:?}", &pairs);

    let mut handles = vec![];
    let results = Arc::new(Mutex::new([0,0,0,0,0,0,0,0,0,0]));
    let arc_groups = Arc::new(groups);

    for p in 0..pairs {
        // For each pair, spawn a thread
        let start = seeds[(p * 2)];
        let end = seeds[(p * 2)+1];
        let current_mins = Arc::clone(&results);
        let current_groups = Arc::clone(&arc_groups);
        let handle = thread::spawn(move || {
            println!("Starting pair: {:?}", &p);
            let mut min_found: usize = 0;

            if debug { println!(" Start: {:?}, End: {:?}", start, end);}

            for n in start..end {
                let result = find_route(n, &current_groups, debug);
                if min_found == 0 || min_found > result {
                    min_found = result;
                    if debug { println!("New min: {:?}", &min_found);}
                }
            }

            let mut accessed_mins = current_mins.lock().unwrap();
            accessed_mins[p] = min_found;

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished the multi threading: {:?}", &results);

    let mut finding_min = results.lock().unwrap();
    finding_min.sort();
    println!("Sorted: {:?}", &finding_min);

    let proper_minimums:Vec<&usize>= finding_min.iter().filter(|f| { f > &&0 }).collect();
    return proper_minimums[0].to_owned();
}

fn line_result(current_line: String, groups: &mut Groups , current_group: &usize, debug: bool) -> () {
    match current_group {
        1 => {add_line_to_group(current_line, &mut groups.seed_to_soil, debug); },
        2 => {add_line_to_group(current_line, &mut groups.soil_to_fertilizer, debug);},
        3 => {add_line_to_group(current_line, &mut groups.fertilizer_to_water, debug);},
        4 => {add_line_to_group(current_line, &mut groups.water_to_light, debug);},
        5 => {add_line_to_group(current_line, &mut groups.light_to_temperature, debug);},
        6 => {add_line_to_group(current_line, &mut groups.temperature_to_humidity, debug);},
        7 => {add_line_to_group(current_line, &mut groups.humidity_to_location, debug);},
        _ => {}
    }
}

fn add_line_to_group(line: String, mut group: &mut Vec<PlantingMap>, debug: bool) {
    if debug { println!("Current line: {}", &line);}

    // split source/target ranger
    let line_split: Vec<&str> = line.split_whitespace().collect();

    if line_split.len() == 3 {

        // new planting map object
        let pm = PlantingMap {
            destination: line_split[0].parse().unwrap(),
            source: line_split[1].parse().unwrap(),
            range: line_split[2].parse().unwrap()
        };

        // Add to the list
        group.push(pm);
    }
}

fn find_route(seed: usize, groups: &Arc<Groups>, debug: bool) -> usize{
    return find_destination(&groups.humidity_to_location,
            find_destination(&groups.temperature_to_humidity,
            find_destination(&groups.light_to_temperature,
            find_destination(&groups.water_to_light,
            find_destination(&groups.fertilizer_to_water,
            find_destination(&groups.soil_to_fertilizer,
            find_destination(&groups.seed_to_soil, seed)))))));

}

fn find_destination(group: &Vec<PlantingMap>, inbound_source: usize) -> usize {
    // find the planting map that has the incoming source between its source + range
    let matching_map = group.iter().find(|g|
                                             (g.source <= inbound_source) &&
                                                 ( (g.source+g.range) >= inbound_source) );

    match matching_map {
        Some(m) => {
            // Find the delta between inbound source & actual source
            // return destination + delta
            return m.destination + (inbound_source - m.source);
        },
        None => {
            // If no match is made, something must have gone wrong. It is the same result
            //panic!("no match was found for {:?} | {:?}", &inbound_source, group);
            return inbound_source;
        }
    }

}

fn build_groups(file_lines: Lines<BufReader<File>>, groups: &mut Groups, seeds: &mut Vec<usize>, debug: bool) {
    // First build the map
    let mut current_groups: usize = 0;
    for (i, line) in file_lines.flatten().into_iter().enumerate() {
        if i == 0 {
            println!("Getting seeds");
            let seeds_options: Vec<&str> = line.split_whitespace().collect();
            seeds.append(&mut get_seeds(seeds_options, debug));
            if seeds.len() == 0 {
                panic!("No seeds found");
            }
            println!("Finished getting seeds");
        }

        if line.len() > 0 {
            match line.as_str() {
                "seed-to-soil map:" => { current_groups = 1; },
                "soil-to-fertilizer map:" => { current_groups = 2; },
                "fertilizer-to-water map:" => { current_groups = 3;},
                "water-to-light map:" => { current_groups = 4;},
                "light-to-temperature map:" => { current_groups = 5; },
                "temperature-to-humidity map:" => { current_groups = 6; },
                "humidity-to-location map:" => { current_groups = 7; },
                _ => {}
            }
            line_result(line, groups, &current_groups, debug);
        }
    };

    println!("Finished building the groups");

    if debug { println!("Seeds {:?}", &seeds)}
    if debug { println!("Groups {:?}", &groups)}
}

fn get_seeds(options: Vec<&str>, debug: bool) -> Vec<usize> {

    let mut seeds: Vec<usize> = vec![];

    let pairs = (options.len() - 1) / 2;
    println!("Pairs: {:?}", &pairs);

    for p in 0..pairs {
        let r1a:usize = options[1 + (p * 2)].parse().unwrap();
        let r1b:usize = r1a + options[2 + (p* 2)].parse::<usize>().unwrap();
        seeds.push(r1a);
        seeds.push(r1b);
    }

    if debug { println!("Final Seeds: {:?}", &seeds); }
    return seeds;

}