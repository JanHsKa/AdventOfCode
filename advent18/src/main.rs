use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum SnailType {
    Number,
    Pair,
}
struct SnailfishNumber {
    pub left: Option<Box<SnailfishNumber>>,
    pub right: Option<Box<SnailfishNumber>>,
    pub pair_type: SnailType,
    pub value: Option<u8>,
}

const INPUT: &str = "test_input.txt";

fn main() {
    println!("Hello, world!");
}

fn read_input() -> SnailfishNumber {
    let mut input_vec: Vec<String> = Vec::new();
    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<String> = Vec::new();

    for line in input_vec.iter() {
        convert_vec = line.chars().map(|c| c.to_string()).collect();
    }

    let fish = read_snail_fish(&mut convert_vec[0..]);

    SnailfishNumber {
        left: None,
        right: None,
        pair_type: SnailType::Pair,
        value: None,
    }
}

fn read_snail_fish(input: &mut [String]) -> SnailfishNumber {
    let mut left: Option<Box<SnailfishNumber>> = None;
    let mut right: Option<Box<SnailfishNumber>> = None;
    let mut pair_type = SnailType::Pair;
    let mut value = None;
    if input[0] == "[".to_string() {
        left = Some(Box::new(read_snail_fish(&mut input[1..])));
    } else if input[1] == ",".to_string() {
        value = Some(input[0].parse::<u8>().expect("failed to parse number"));
        pair_type = SnailType::Number;
    } else if input[1] == "]".to_string() {
    }

    SnailfishNumber {
        left,
        right,
        pair_type,
        value,
    }
}
