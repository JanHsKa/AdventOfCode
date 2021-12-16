use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";

fn main() {
    let input = read_input();
    let mut count = 0;

    for line in input.iter() {
        count += decode_line(line);
    }

    println!("There complete sum is {}", count);
}

fn decode_line(line: &Vec<String>) -> u32 {
    let mut digit_codes = vec![String::default(); 10];
    digit_codes.clone_from_slice(&line[0..10]);

    let mut decode_digits: Vec<String> = vec![String::default(); 4];
    decode_digits.clone_from_slice(&line[11..]);
    sort_strings(&mut digit_codes);
    sort_strings(&mut decode_digits);

    let mut decode_map: HashMap<String, u8> = HashMap::new();
    let mut index_to_temove: Vec<usize>;

    index_to_temove = decode_by_count(&digit_codes, &mut decode_map);
    index_to_temove.sort();

    for i in index_to_temove.iter().rev() {
        digit_codes.remove(*i);
    }

    index_to_temove = decode_nine_zero_three_six(&digit_codes, &mut decode_map);
    index_to_temove.sort();

    for i in index_to_temove.iter().rev() {
        digit_codes.remove(*i);
    }

    decode_five_two(&digit_codes, &mut decode_map);

    let mut line_value: u32 = 0;
    let mut factor: u32 = 1000;
    for n in decode_digits {
        line_value += factor * *decode_map.get(&n).unwrap() as u32;
        factor /= 10;
    }

    line_value
}

fn sort_strings(slice: &mut Vec<String>) {
    for i in 0..slice.len() {
        let mut chars = slice[i].chars().collect::<Vec<char>>();
        chars.sort();
        slice[i] = chars.into_iter().collect::<String>();
    }
}

fn decode_five_two(line: &[String], decode_map: &mut HashMap<String, u8>) {
    let mut six = String::new();
    decode_map.keys().for_each(|k| {
        if let Some(value) = decode_map.get(k) {
            if *value == 6 {
                six = k.clone();
            }
        }
    });

    for i in line {
        if calculate_differences(i, &six) == 1 {
            decode_map.insert(i.clone(), 5);
        } else {
            decode_map.insert(i.clone(), 2);
        }
    }
}

fn decode_nine_zero_three_six(line: &[String], decode_map: &mut HashMap<String, u8>) -> Vec<usize> {
    let mut indexes = Vec::new();
    let mut possibles: Vec<String> = Vec::new();
    let mut seven_code: String = String::new();
    let mut three = String::new();
    decode_map.keys().for_each(|k| match k.len() {
        3 => seven_code = k.clone(),
        _ => {}
    });

    for (i, s) in line.iter().enumerate() {
        match s.len() {
            6 => {
                possibles.push(s.clone());
                indexes.push(i)
            }
            5 => {
                if is_in(&seven_code, s) {
                    three = s.clone();
                    indexes.push(i);
                }
            }
            _ => {}
        }
    }

    for s in possibles {
        if is_in(&three, &s) {
            decode_map.insert(s.clone(), 9);
        } else if is_in(&seven_code, &s) {
            decode_map.insert(s.clone(), 0);
        } else {
            decode_map.insert(s.clone(), 6);
        }
    }

    decode_map.insert(three, 3);
    indexes
}

fn is_in(string_one: &String, string_two: &String) -> bool {
    for i in string_one.chars() {
        if !string_two.contains(i) {
            return false;
        }
    }
    true
}

fn calculate_differences(string_one: &String, string_two: &String) -> usize {
    let mut differences = 0;
    let strings = vec![string_one, string_two];
    let first;
    let second;
    if string_one.len() > string_two.len() {
        first = 1;
        second = 0;
    } else {
        first = 0;
        second = 1;
    }

    differences += strings[second].len() - strings[first].len();

    for i in strings[first].chars() {
        if !strings[second].contains(i) {
            differences += 1;
        }
    }

    differences
}

fn decode_by_count(line: &[String], decode_map: &mut HashMap<String, u8>) -> Vec<usize> {
    let mut indexes = Vec::new();
    for (i, s) in line.iter().enumerate() {
        match s.len() {
            2 => {
                decode_map.insert(s.clone(), 1);
                indexes.push(i);
            }
            3 => {
                decode_map.insert(s.clone(), 7);
                indexes.push(i);
            }
            4 => {
                decode_map.insert(s.clone(), 4);
                indexes.push(i);
            }
            7 => {
                decode_map.insert(s.clone(), 8);
                indexes.push(i);
            }
            _ => {}
        }
    }

    indexes
}

fn read_input() -> Vec<Vec<String>> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<Vec<String>> = Vec::new();
    for line in input_vec {
        convert_vec.push(
            line.split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    convert_vec
}
