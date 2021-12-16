use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";

fn main() {
    let mut input = read_input();
    let mut error_lines = Vec::new();
    let mut error_code: u32 = 0;
    let mut incomplete_scores: Vec<u64> = Vec::new();
    let middle_score: u64;

    for (i, line) in input.iter().enumerate() {
        let value = check_line(line);
        if value > 0 {
            error_lines.push(i);
            error_code += check_line(line);
        }
    }

    println!("Syntax error score: {}", error_code);

    for i in error_lines.iter().rev() {
        input.remove(*i);
    }

    for i in input {
        incomplete_scores.push(check_incomplete_line(&i));
    }

    incomplete_scores.sort();
    middle_score = incomplete_scores[incomplete_scores.len() / 2];

    println!("Incomplete score: {}", middle_score);
}

fn check_incomplete_line(input: &[String]) -> u64 {
    let openings: HashSet<String> = create_openings();
    let closures: HashSet<String> = create_closures();
    let open_map = create_open_map();
    let mut open_chunks: Vec<String> = Vec::new();
    let mut closing_chunks: Vec<String> = Vec::new();
    let mut score: u64 = 0;

    for i in input {
        if openings.contains(i) {
            open_chunks.push(i.clone());
        } else if closures.contains(i) {
            open_chunks.pop();
        }
    }

    for i in open_chunks.iter().rev() {
        closing_chunks.push(open_map.get(i).unwrap().clone());
    }

    for i in closing_chunks {
        score *= 5;
        match i.as_str() {
            ")" => score += 1,
            "]" => score += 2,
            "}" => score += 3,
            ">" => score += 4,
            _ => {}
        };
    }

    score
}

fn check_line(input: &[String]) -> u32 {
    let openings: HashSet<String> = create_openings();
    let closures: HashSet<String> = create_closures();
    let closure_map = create_closure_map();
    let mut open_chunks: Vec<String> = Vec::new();

    for i in input {
        if openings.contains(i) {
            open_chunks.push(i.clone());
        } else if closures.contains(i) {
            let opening = closure_map
                .get(i)
                .expect("failed to find opening for closure");

            if open_chunks.len() > 0 && open_chunks.last().unwrap().eq(opening) {
                open_chunks.pop();
            } else {
                return match i.as_str() {
                    ")" => 3,
                    "]" => 57,
                    "}" => 1197,
                    ">" => 25137,
                    _ => 0,
                };
            }
        }
    }

    0
}

fn create_open_map() -> HashMap<String, String> {
    let mut open_map = HashMap::new();

    open_map.insert("<".to_string(), ">".to_string());
    open_map.insert("(".to_string(), ")".to_string());
    open_map.insert("{".to_string(), "}".to_string());
    open_map.insert("[".to_string(), "]".to_string());

    open_map
}

fn create_closure_map() -> HashMap<String, String> {
    let mut closure_map = HashMap::new();

    closure_map.insert(">".to_string(), "<".to_string());
    closure_map.insert(")".to_string(), "(".to_string());
    closure_map.insert("}".to_string(), "{".to_string());
    closure_map.insert("]".to_string(), "[".to_string());

    closure_map
}

fn create_openings() -> HashSet<String> {
    let mut openings: HashSet<String> = HashSet::new();
    openings.insert("(".to_string());
    openings.insert("<".to_string());
    openings.insert("{".to_string());
    openings.insert("[".to_string());

    openings
}

fn create_closures() -> HashSet<String> {
    let mut closures: HashSet<String> = HashSet::new();

    closures.insert(")".to_string());
    closures.insert(">".to_string());
    closures.insert("}".to_string());
    closures.insert("]".to_string());

    closures
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
        convert_vec.push(line.chars().map(|s| s.to_string()).collect::<Vec<String>>());
    }

    convert_vec
}
