use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";
const STEPS: u32 = 40;
fn main() {
    let (template, rules) = read_input();
    let mut quantities: HashMap<char, u64> = HashMap::new();
    let new_map = reworked_algorithm(template, &rules, &mut &mut quantities);
    let (most_common, least_common) = find_highest_and_lowest(&quantities);
    let quantity = most_common - least_common;

    println!("{:?}", quantities);

    println!(
        "The quantites are {} + {} = {}",
        most_common, least_common, quantity
    );
}

fn reworked_algorithm(
    template: Vec<char>,
    rules: &HashMap<String, char>,
    quantities: &mut HashMap<char, u64>,
) -> HashMap<String, u64> {
    let mut pairs: HashMap<String, u64> = HashMap::new();
    for i in rules.keys() {
        pairs.insert(i.clone(), 0);
    }
    let new_rules = rework_rules(rules);

    for i in 0..template.len() - 1 {
        if let Some(value) = quantities.get(&template[i]).cloned() {
            quantities.insert(template[i].clone(), value + 1);
        } else {
            quantities.insert(template[i].clone(), 1);
        }
        let mut new_string = template[i].to_string();
        new_string += &template[i + 1].to_string();
        if let Some(pair) = pairs.get(&new_string).cloned() {
            pairs.insert(new_string, pair + 1);
        }
    }

    if let Some(value) = quantities.get(&template.last().unwrap()).cloned() {
        quantities.insert(template.last().unwrap().clone(), value + 1);
    } else {
        quantities.insert(template.last().unwrap().clone(), 1);
    }

    for _ in 0..STEPS {
        one_step(&new_rules, rules, &mut pairs, quantities);
    }

    pairs
}

fn one_step(
    new_rules: &HashMap<String, Vec<String>>,
    old_rules: &HashMap<String, char>,
    pairs: &mut HashMap<String, u64>,
    quantities: &mut HashMap<char, u64>,
) {
    let mut new_pairs: HashMap<String, u64> = HashMap::new();
    let mut remove_pairs: HashMap<String, u64> = HashMap::new();
    for (pair, count) in pairs.iter() {
        if *count > 0 {
            let symbol = old_rules.get(pair).unwrap();
            if let Some(value) = quantities.get(symbol).cloned() {
                quantities.insert(symbol.clone(), value + count);
            } else {
                quantities.insert(symbol.clone(), *count);
            }

            if let Some(value) = remove_pairs.get(pair).cloned() {
                remove_pairs.insert(pair.clone(), value + count);
            } else {
                remove_pairs.insert(pair.clone(), *count);
            }

            let new = new_rules.get(pair).unwrap();

            for i in new {
                if let Some(value) = new_pairs.get(i).cloned() {
                    new_pairs.insert(i.clone(), value + count);
                } else {
                    new_pairs.insert(i.clone(), *count);
                }
            }
        }
    }

    for (pair, count) in new_pairs {
        if let Some(value) = pairs.get(&pair).cloned() {
            pairs.insert(pair.clone(), value + count);
        }
    }

    for (pair, count) in remove_pairs {
        if let Some(value) = pairs.get(&pair).cloned() {
            pairs.insert(pair.clone(), value - count);
        }
    }
}

fn rework_rules(rules: &HashMap<String, char>) -> HashMap<String, Vec<String>> {
    let mut new_rules = HashMap::new();

    for (pair, c) in rules {
        let mut new_pairs: Vec<String> = Vec::new();
        let chars = pair.chars().collect::<Vec<char>>();
        let mut new_string = "".to_string();
        new_string += &chars[0].to_string();
        new_string += &c.to_string();

        new_pairs.push(new_string);

        new_string = "".to_string();

        new_string += &c.to_string();
        new_string += &chars[1].to_string();
        new_pairs.push(new_string);
        new_rules.insert(pair.clone(), new_pairs);
    }

    new_rules
}

fn find_highest_and_lowest(quantities: &HashMap<char, u64>) -> (u64, u64) {
    let mut most_common = 0;
    let mut least_common = 0;

    for i in quantities.values() {
        if *i > most_common || most_common == 0 {
            most_common = *i;
        }
        if *i < least_common || least_common == 0 {
            least_common = *i;
        }
    }

    (most_common, least_common)
}

fn read_input() -> (Vec<char>, HashMap<String, char>) {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<char> = Vec::new();

    for line in input_vec.iter() {
        if line.is_empty() {
            break;
        }
        convert_vec = line.chars().collect::<Vec<char>>();
    }

    let mut input_map = HashMap::new();
    for i in 2..input_vec.len() {
        let splits: Vec<String> = input_vec[i].split(" -> ").map(|f| f.to_string()).collect();
        input_map.insert(splits[0].clone(), splits[1].as_bytes()[0] as char);
    }

    (convert_vec, input_map)
}
