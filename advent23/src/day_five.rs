use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn calculate_day_five(file: File, debug: bool) -> u64 {
    let file_lines = BufReader::new(file).lines();
    let mut final_result: usize = 0;
    let mut current_group: usize = 0;

    let mut groups = Groups{
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    let mut seeds: Vec<usize> = vec![];

    // First build the map
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

        match line.as_str() {
           "seed-to-soil map:" => {
               current_group = 1
           },
           "soil-to-fertilizer map:" => {
               current_group = 2
           },
           "fertilizer-to-water map:" => {
               current_group = 3
           },
           "water-to-light map:" => {
               current_group = 4
           },
           "light-to-temperature map:" => {
               current_group = 5
           },
           "temperature-to-humidity map:" => {
               current_group = 6
           },
           "humidity-to-location map:" => {
               current_group = 7
           },
            _ => {}
        }

        if line.len() > 0 {
            line_result(line, &mut groups, &current_group, debug);
        }

    };
    println!("Finished building the groups");

    if debug { println!("Seeds {:?}", &seeds)}
    if debug { println!("Groups {:?}", groups)}

    // Then calculate the result
    let pairs = (seeds.len()) / 2;
    println!("Pairs: {:?}", &pairs);

    for p in 0..pairs {

        
        for n in seeds[(p * 2)]..seeds[(p * 2)+1] {
            let result = find_route(n, &groups, debug);

            if final_result == 0 || result < final_result {
                final_result = result;
            }

            if n % 1000 == 0 {
                println!("Still finding results: {:?} | current: {:?}", &n, &final_result);
            }
        }
    }

    if debug { println!("Results {:?}", final_result)}

    return final_result as u64;
}

fn get_seeds(options: Vec<&str>, debug: bool) -> Vec<usize> {

    let mut seeds: Vec<usize> = vec![];

    let pairs = (options.len() - 1) / 2;
    println!("Pairs: {:?}", &pairs);

    for p in 0..pairs {
        let r1a:usize = options[1 + (p * 2)].parse().unwrap();
        let r1b:usize = r1a + options[2 + (p* 2)].parse::<usize>().unwrap();
        // if debug { println!("R1: {:?} | {:?}", &r1a, &r1b); }
        // for s in r1a..r1b {
        //     //if !(seeds.contains(&s)) {
        //         seeds.push(s);
        //     //}
        // }
        seeds.push(r1a);
        seeds.push(r1b);
    }

    if debug { println!("Final Seeds: {:?}", &seeds); }
    return seeds;

}

#[derive(Debug)]
struct PlantingMap {
    pub source: usize,
    pub destination: usize,
    pub range: usize
}

#[derive(Debug)]
struct Groups {
    pub seed_to_soil: Vec<PlantingMap>,
    pub soil_to_fertilizer: Vec<PlantingMap>,
    pub fertilizer_to_water: Vec<PlantingMap>,
    pub water_to_light: Vec<PlantingMap>,
    pub light_to_temperature: Vec<PlantingMap>,
    pub temperature_to_humidity: Vec<PlantingMap>,
    pub humidity_to_location: Vec<PlantingMap>
}

fn line_result(current_line: String, groups: &mut Groups , current_group: &usize, debug: bool) -> () {
    match current_group {
        1 => { add_line_to_group(current_line, &mut groups.seed_to_soil, debug); },
        2 => {add_line_to_group(current_line, &mut groups.soil_to_fertilizer, debug);},
        3 => {add_line_to_group(current_line, &mut groups.fertilizer_to_water, debug);},
        4 => {add_line_to_group(current_line, &mut groups.water_to_light, debug);},
        5 => {add_line_to_group(current_line, &mut groups.light_to_temperature, debug);},
        6 => {add_line_to_group(current_line, &mut groups.temperature_to_humidity, debug);},
        7 => {add_line_to_group(current_line, &mut groups.humidity_to_location, debug);},
        _ => {}
    }
}

fn add_line_to_group(line: String, group: &mut Vec<PlantingMap>, debug: bool) {
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

fn find_route(seed: usize, groups: &Groups, debug: bool) -> usize{
    let soil = find_destination(&groups.seed_to_soil, seed);
    let fertilizer = find_destination(&groups.soil_to_fertilizer, soil);
    let water = find_destination(&groups.fertilizer_to_water, fertilizer);
    let light = find_destination(&groups.water_to_light, water);
    let temperature = find_destination(&groups.light_to_temperature, light);
    let humidity = find_destination(&groups.temperature_to_humidity, temperature);
    let location= find_destination(&groups.humidity_to_location, humidity);

    return location;
}

fn find_destination(group: &Vec<PlantingMap>, inbound_source: usize) -> usize {
    // find the planting map that has the incoming source between its source + range
    let matching_map = group.iter().find(|g|
                                             (g.source <= inbound_source) &&
                                                 ( (g.source+g.range) >= inbound_source) );

    match matching_map {
        Some(m) => {
            // Find the delta between inbound source & actual source
            let delta = inbound_source - m.source;
            // return destination + delta
            return m.destination + delta;
        },
        None => {
            // If no match is made, something must have gone wrong. It is the same result
            //panic!("no match was found for {:?} | {:?}", &inbound_source, group);
            return inbound_source;
        }
    }

}