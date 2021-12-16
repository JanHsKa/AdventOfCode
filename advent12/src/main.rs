use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";
const START: &str = "start";
const END: &str = "end";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Debug)]
struct Cave {
    name: String,
    size: CaveSize,
    connected: HashSet<String>,
}

impl Cave {
    pub fn new(name: String, first_cave: String) -> Cave {
        let mut connected = HashSet::new();
        let mut size: CaveSize = CaveSize::Small;
        connected.insert(first_cave);
        if check_upppercase(&name) {
            size = CaveSize::Big;
        }

        Cave {
            name,
            size,
            connected,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_connected(&self) -> &HashSet<String> {
        &self.connected
    }

    pub fn add_connection(&mut self, name: String) {
        self.connected.insert(name);
    }

    pub fn is_big(&self) -> bool {
        self.size.eq(&CaveSize::Big)
    }
}

fn main() {
    let input = read_input();
    let solution = find_paths(&input);

    println!("{}", solution);
}

fn find_paths(input: &HashMap<String, Cave>) -> u32 {
    let mut paths = 0;
    let start = input.get(START).expect("no start cave");
    let mut visited_small_caves: HashSet<String> = HashSet::new();
    visited_small_caves.insert(START.to_string());
    for cave in start.get_connected() {
        paths += search_path(
            input,
            visited_small_caves.clone(),
            cave.clone(),
            vec![start.get_name()],
            false,
        );
    }

    paths
}

fn search_path(
    input: &HashMap<String, Cave>,
    mut visited_caves: HashSet<String>,
    cave_name: String,
    mut path: Vec<String>,
    mut visited_twice: bool,
) -> u32 {
    let mut paths = 0;

    path.push(cave_name.clone());
    if cave_name.eq(END) {
        //println!("{:?}", path);
        return 1;
    }

    let cave = input.get(&cave_name).expect("could not find cave");
    if !cave.is_big() {
        if visited_caves.contains(&cave_name) {
            visited_twice = true;
        } else {
            visited_caves.insert(cave_name.clone());
        }
    }

    for connected in cave.get_connected() {
        let connected_cave = input.get(connected).expect("could not find cave");
        if connected != START
            && (connected_cave.is_big() || !visited_caves.contains(connected) || !visited_twice)
        {
            paths += search_path(
                input,
                visited_caves.clone(),
                connected.clone(),
                path.clone(),
                visited_twice,
            );
        }
    }

    paths
}

fn check_upppercase(name: &String) -> bool {
    for i in name.chars() {
        if i.is_lowercase() {
            return false;
        }
    }

    return true;
}

fn read_input() -> HashMap<String, Cave> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<Vec<String>> = Vec::new();
    for line in input_vec {
        convert_vec.push(
            line.split("-")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    let mut cave_map: HashMap<String, Cave> = HashMap::new();

    for i in convert_vec {
        if let Some(cave) = cave_map.get_mut(&i[0]) {
            cave.add_connection(i[1].clone());
        } else {
            cave_map.insert(i[0].clone(), Cave::new(i[0].clone(), i[1].clone()));
        }

        if let Some(cave) = cave_map.get_mut(&i[1]) {
            cave.add_connection(i[0].clone());
        } else {
            cave_map.insert(i[1].clone(), Cave::new(i[1].clone(), i[0].clone()));
        }
    }

    cave_map
}
