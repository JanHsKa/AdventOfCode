use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const START_DAYS: u8 = 6;
const DAYS: u16 = 256;

fn main() {
    let fish = read_input();
    let number_map: HashMap<u8, u128> = calculcate_fish_expansion();
    let mut count: u128 = 0;

    println!("map: {:?}", number_map);
    println!("Inital: {:?}", fish);

    for f in fish {
        count += number_map.get(&f).expect("could not find value in map");
    }

    println!("Fish count: {}", count);
}

fn calculcate_fish_expansion() -> HashMap<u8, u128> {
    let mut fish: Vec<u8> = vec![1];
    let mut fish_to_append: Vec<u8> = Vec::new();
    let mut number_map: HashMap<u8, u128> = HashMap::new();

    for i in 0..DAYS {
        println!("Days: {:#3}", i + 1);
        for n in 0..fish.len() {
            if fish[n] == 0 {
                fish[n] = START_DAYS;
                fish_to_append.push(START_DAYS + 2);
            } else {
                fish[n] -= 1;
            }
        }
        fish.append(&mut fish_to_append);

        fish_to_append = Vec::new();

        if i + 5 >= DAYS {
            match DAYS - i {
                2 => number_map.insert(2, fish.len() as u128),
                3 => number_map.insert(3, fish.len() as u128),
                4 => number_map.insert(4, fish.len() as u128),
                5 => number_map.insert(5, fish.len() as u128),

                _ => None,
            };
        }
    }
    number_map.insert(1, fish.len() as u128);

    number_map
}

fn read_input() -> Vec<u8> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open("input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<u8> = Vec::new();
    for line in input_vec {
        convert_vec.append(
            &mut line
                .split(",")
                .map(|s| s.to_string().parse::<u8>().expect("failed to parse"))
                .collect::<Vec<u8>>(),
        );
    }

    convert_vec
}
