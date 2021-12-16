use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Rating {
    Oxygen,
    CO2,
}
fn main() {
    let input_vec = read_input();
    let bitcount = 12;
    let oxygen = calculate(input_vec.clone(), bitcount, Rating::Oxygen);
    let co2 = calculate(input_vec.clone(), bitcount, Rating::CO2);

    println!("The Product ist: {}", oxygen * co2);
}

fn calculate(mut input_vec: Vec<u32>, bitcount: u32, rating: Rating) -> u32 {
    for bit in (0..bitcount).rev() {
        let bitmask: u32 = 1 << bit;
        let (count_one, count_zero) = count_bits(&input_vec, bitmask, bit);

        match rating {
            Rating::Oxygen => calculate_oxygen(&mut input_vec, bitmask, bit, count_one, count_zero),
            Rating::CO2 => calculate_co2(&mut input_vec, bitmask, bit, count_one, count_zero),
        }

        if input_vec.len() == 1 {
            return input_vec[0];
        }
    }

    0
}

fn calculate_oxygen(
    input_vec: &mut Vec<u32>,
    bitmask: u32,
    bit: u32,
    count_one: u32,
    count_zero: u32,
) {
    if count_one >= count_zero {
        remove_numbers(input_vec, bitmask, bit, 0);
    } else {
        remove_numbers(input_vec, bitmask, bit, 1);
    }
}

fn calculate_co2(
    input_vec: &mut Vec<u32>,
    bitmask: u32,
    bit: u32,
    count_one: u32,
    count_zero: u32,
) {
    if count_one >= count_zero {
        remove_numbers(input_vec, bitmask, bit, 1);
    } else {
        remove_numbers(input_vec, bitmask, bit, 0);
    }
}

fn count_bits(input_vec: &[u32], bitmask: u32, bit: u32) -> (u32, u32) {
    let mut count_zero = 0;
    let mut count_one = 0;
    for number in input_vec.iter() {
        match (*number & bitmask) >> bit {
            1 => count_one += 1,
            0 => count_zero += 1,
            _ => {}
        }
    }

    (count_one, count_zero)
}

fn remove_numbers(numbers: &mut Vec<u32>, bitmask: u32, shift: u32, bit: u32) {
    let mut numbers_to_delete: Vec<usize> = Vec::new();
    for (i, number) in numbers.iter().enumerate() {
        if (number & bitmask) >> shift == bit {
            numbers_to_delete.push(i);
        }
    }

    numbers_to_delete.sort();

    for i in numbers_to_delete.iter().rev() {
        numbers.remove(*i);
    }
}

fn read_input() -> Vec<u32> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open("input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<u32> = Vec::with_capacity(input_vec.len());

    for number in input_vec.into_iter() {
        let mut new_number: u32 = 0;
        for character in number.chars() {
            new_number |= character.to_digit(10).expect("failed to convert character");
            new_number <<= 1;
        }
        new_number >>= 1;
        convert_vec.push(new_number);
    }

    convert_vec
}
