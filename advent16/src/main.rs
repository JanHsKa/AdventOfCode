use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";

fn main() {
    let input = read_input();
    let (_, value) = decode_package(&input);

    println!("the total value is {}", value);
}

fn decode_package(input: &[u8]) -> (u64, u64) {
    let (_, type_id) = decode_header(&input);
    let mut size = 6;
    let mut value: u64 = 0;
    if type_id == 4 {
        let (new_size, new_value) = get_literal_groups(&input[6..]);
        size += new_size;
        value += new_value;
    } else {
        let (new_size, new_value) = decode_operators(&input[6..], type_id);
        size += new_size;
        value += new_value;
    }

    (size as u64, value as u64)
}

fn decode_operators(input: &[u8], type_id: u8) -> (u64, u64) {
    let length_id = input[0];
    let mut packet_start;
    let mut size = 1;
    let value;
    let mut package_values = Vec::new();
    if length_id == 0 {
        let packet_len = decode_bit_slice(&input[1..16]) as usize;
        size += 15;
        packet_start = 16;

        let mut inner_packet_len = 0;
        while inner_packet_len < packet_len {
            let (new_size, new_value) = decode_package(&input[packet_start..]);
            inner_packet_len += new_size as usize;
            packet_start += new_size as usize;
            size += new_size;
            package_values.push(new_value);
        }
    } else {
        let number_of_packages = decode_bit_slice(&input[1..12]) as usize;
        size += 11;
        packet_start = 12;
        let mut index = 1;
        while index <= number_of_packages {
            let (new_size, new_value) = decode_package(&input[packet_start..]);
            packet_start += new_size as usize;
            size += new_size;
            index += 1;
            package_values.push(new_value);
        }
    }

    value = calculate_value(&mut package_values, type_id);
    (size, value)
}

fn calculate_value(package_values: &mut Vec<u64>, type_id: u8) -> u64 {
    let mut value = 0;
    println!("value: {:?}", package_values);
    println!("type id : {}", type_id);
    match type_id {
        0 => package_values.iter().for_each(|v| value += *v),
        1 => {
            value = 1;
            package_values.iter().for_each(|v| value *= *v);
        }
        2 => {
            package_values.sort();
            value = *package_values.first().expect("package is empty");
        }
        3 => {
            package_values.sort();
            value = *package_values.last().expect("package is empty");
        }
        5 => {
            if package_values[0] > package_values[1] {
                value = 1;
            } else {
                value = 0;
            }
        }
        6 => {
            if package_values[0] > package_values[1] {
                value = 0;
            } else {
                value = 1;
            }
        }
        7 => {
            if package_values[0] == package_values[1] {
                value = 1;
            } else {
                value = 0;
            }
        }
        _ => value = 0,
    };

    println!("value: {}", value);
    println!();
    value
}

fn get_literal_groups(input: &[u8]) -> (u64, u64) {
    let mut groups = Vec::new();
    let mut index = 0;
    let mut continue_loop = true;

    while continue_loop {
        if input[index] == 0 {
            continue_loop = false;
        }

        groups.push(
            input[index + 1..index + 5]
                .iter()
                .cloned()
                .collect::<Vec<u8>>(),
        );

        index += 5;
        if index >= input.len() {
            continue_loop = index < input.len();
        }
    }

    let value = decode_bit_slice(
        &groups
            .iter()
            .flat_map(|g| g.iter())
            .cloned()
            .collect::<Vec<u8>>(),
    );

    (groups.len() as u64 * 5, value)
}

fn decode_header(input: &[u8]) -> (u8, u8) {
    let version;
    let type_id;

    version = decode_bit_slice(&input[0..3]) as u8;
    type_id = decode_bit_slice(&input[3..6]) as u8;

    (version, type_id)
}

fn decode_bit_slice(slice: &[u8]) -> u64 {
    let mut value: u64 = 0;
    slice.iter().for_each(|i| {
        value <<= 1;
        value |= *i as u64;
    });

    value
}

fn read_input() -> Vec<u8> {
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

    let decoder = get_hexa_decoder();
    let mut binary_vec = Vec::new();

    for i in convert_vec.iter() {
        let binary = decoder.get(i).expect("no hexadecimal value");
        binary_vec.append(
            &mut binary
                .chars()
                .map(|c| c.to_string().parse::<u8>().expect("failed to parse"))
                .collect::<Vec<u8>>(),
        );
    }

    binary_vec
}

fn get_hexa_decoder() -> HashMap<String, String> {
    let mut decoder = HashMap::new();

    decoder.insert("0".to_string(), "0000".to_string());
    decoder.insert("1".to_string(), "0001".to_string());
    decoder.insert("2".to_string(), "0010".to_string());
    decoder.insert("3".to_string(), "0011".to_string());
    decoder.insert("4".to_string(), "0100".to_string());
    decoder.insert("5".to_string(), "0101".to_string());
    decoder.insert("6".to_string(), "0110".to_string());
    decoder.insert("7".to_string(), "0111".to_string());
    decoder.insert("8".to_string(), "1000".to_string());
    decoder.insert("9".to_string(), "1001".to_string());
    decoder.insert("A".to_string(), "1010".to_string());
    decoder.insert("B".to_string(), "1011".to_string());
    decoder.insert("C".to_string(), "1100".to_string());
    decoder.insert("D".to_string(), "1101".to_string());
    decoder.insert("E".to_string(), "1110".to_string());
    decoder.insert("F".to_string(), "1111".to_string());

    decoder
}
