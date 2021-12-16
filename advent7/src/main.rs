use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";

fn main() {
    let mut crabs = read_input();
    crabs.sort();

    let mut smallest_cost = 0;
    let mut fuel_map: HashMap<u32, u32> = HashMap::new();

    for i in crabs[0]..*crabs.last().unwrap() {
        let cost = count_fuel(i, &crabs);
        fuel_map.insert(i, cost);
        if fuel_map.get(&smallest_cost).is_none() || *fuel_map.get(&smallest_cost).unwrap() > cost {
            smallest_cost = i;
        }
    }

    println!(
        "Smallest Cost is {} at position {}",
        fuel_map.get(&smallest_cost).unwrap(),
        smallest_cost
    );
}

fn count_fuel(position: u32, crabs: &Vec<u32>) -> u32 {
    let mut fuel_cost = 0;

    for crab in crabs {
        let diff;
        if *crab < position {
            diff = position - crab;
        } else {
            diff = crab - position;
        }

        fuel_cost += ((diff.pow(2)) + diff) / 2;
    }

    fuel_cost
}

fn read_input() -> Vec<u32> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<u32> = Vec::new();
    for line in input_vec {
        convert_vec.append(
            &mut line
                .split(",")
                .map(|s| s.to_string().parse::<u32>().expect("failed to parse"))
                .collect::<Vec<u32>>(),
        );
    }

    convert_vec
}
