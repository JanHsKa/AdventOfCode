use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
    sync::Arc,
    thread,
};

use parking_lot::{Mutex, RwLock};

const INPUT: &str = "input.txt";
const STEPS: u32 = 40;
fn main() {
    let (template, rules) = read_input();
    let quantities = count_quantities(&template);
    let reference_quantity = Arc::new(Mutex::new(quantities));
    let reference_rules = Arc::new(RwLock::new(rules));
    let mut handles = Vec::new();
    for i in 0..template.len() - 1 {
        let quant_clone = reference_quantity.clone();
        let rules_clone = reference_rules.clone();
        let input = vec![template[i], template[i + 1]];
        let handle = thread::spawn(move || {
            two_chars(quant_clone, input, rules_clone, 1);
        });

        handles.push(handle);
    }

    for i in handles {
        i.join().unwrap();
    }

    let new_quantities = reference_quantity.lock().deref().clone();
    let (most_common, least_common) = find_highest_and_lowest(&new_quantities);
    let quantity = most_common - least_common;

    println!("{:?}", new_quantities);

    println!(
        "The quantites are {} + {} = {}",
        most_common, least_common, quantity
    );
}

fn two_chars(
    counts: Arc<Mutex<HashMap<char, u64>>>,
    input: Vec<char>,
    rules: Arc<RwLock<HashMap<String, char>>>,
    steps: u32,
) {
    if let Some(result) = rules.read().get(&input.iter().collect::<String>()) {
        insert_count(counts.clone(), result);
        if steps < STEPS {
            two_chars(
                counts.clone(),
                vec![input[0], *result],
                rules.clone(),
                steps + 1,
            );
            two_chars(
                counts.clone(),
                vec![*result, input[1]],
                rules.clone(),
                steps + 1,
            );
        }
    }
}

fn insert_count(counts: Arc<Mutex<HashMap<char, u64>>>, result: &char) {
    let mut guard = counts.lock();
    let value = *guard.get(result).or_else(|| Some(&0)).unwrap();
    guard.insert(*result, value + 1);
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

fn count_quantities(template: &Vec<char>) -> HashMap<char, u64> {
    let mut quantities: HashMap<char, u64> = HashMap::new();
    for i in template {
        if let Some(value) = quantities.get(&i).map(|f| *f) {
            quantities.insert(i.clone(), value + 1);
        } else {
            quantities.insert(i.clone(), 1);
        }
    }

    quantities
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
