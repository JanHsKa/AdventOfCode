use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    pub position: Coordinate,
    pub risk_level_from_start: Option<u64>,
    pub predecessor: Option<Coordinate>,
}

const INPUT: &str = "input.txt";
type Coordinate = (usize, usize);
const START: Coordinate = (0, 0);

fn main() {
    let input = read_input();
    let bigger_input = create_bigger_field(&input);

    let nodes = create_nodes(&bigger_input);

    let result = dijkstra(&bigger_input, nodes);

    println!("The lowest total risk is {}", result);
}

fn create_bigger_field(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_vec = Vec::with_capacity(input.len() * 5);
    let y_len = input.len();
    let x_len = input[0].len();
    for new_y in 0..5 {
        for _ in 0..y_len {
            new_vec.push(vec![0; x_len * 5]);
        }
        for new_x in 0..5 {
            for y in 0..y_len {
                for x in 0..x_len {
                    let mut value;
                    if new_y != 0 {
                        value = new_vec[(new_y - 1) * x_len + y][new_x * x_len + x] + 1;
                    } else if new_x != 0 {
                        value = new_vec[new_y * x_len + y][(new_x - 1) * x_len + x] + 1;
                    } else {
                        value = input[y][x];
                    }

                    if value > 9 {
                        value = 1;
                    }
                    new_vec[new_y * y_len + y][new_x * x_len + x] = value;
                }
            }
        }
    }

    new_vec
}
fn dijkstra(input: &Vec<Vec<u8>>, mut nodes: HashMap<Coordinate, Node>) -> u64 {
    println!("start");
    let end_coordinate = (input[0].len() - 1, input.len() - 1);
    let mut visited_nodes: HashSet<Coordinate> = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert(START);

    while !queue.is_empty() {
        dijkstra_step(input, &mut nodes, &mut queue, &mut visited_nodes);
    }

    let end_node = nodes.get(&end_coordinate).unwrap();

    println!("finished");
    end_node.risk_level_from_start.unwrap()
}

fn dijkstra_step(
    input: &Vec<Vec<u8>>,
    nodes: &mut HashMap<Coordinate, Node>,
    queue: &mut HashSet<Coordinate>,
    visited_nodes: &mut HashSet<Coordinate>,
) {
    let node_coord = get_lowest(queue, nodes);
    let node = *nodes.get(&node_coord).unwrap();

    let mut adjacent = get_all_adjacent(input, &node_coord);
    if let Some(pre) = node.predecessor {
        adjacent.remove(&pre);
    }

    for i in adjacent {
        let current_node = nodes.get_mut(&i).unwrap();
        let new_risk = node.risk_level_from_start.unwrap() + input[i.1][i.0] as u64;
        if let Some(risk) = current_node.risk_level_from_start {
            if risk > new_risk {
                current_node.risk_level_from_start = Some(new_risk);
                current_node.predecessor = Some(node_coord);
            }
        } else {
            current_node.risk_level_from_start = Some(new_risk);
            current_node.predecessor = Some(node_coord);
        }
        if !visited_nodes.contains(&i) {
            queue.insert(i);
        }
    }

    queue.remove(&node_coord);
    visited_nodes.insert(node_coord);
}

fn get_lowest(queue: &HashSet<Coordinate>, nodes: &HashMap<Coordinate, Node>) -> Coordinate {
    let mut lowest = 0;
    let mut low_node = (0, 0);
    for i in queue {
        let node = nodes.get(i).unwrap();
        if let Some(value) = node.risk_level_from_start {
            if value < lowest || lowest == 0 {
                lowest = value;
                low_node = *i;
            }
        }
    }

    low_node
}

fn get_all_adjacent(input: &Vec<Vec<u8>>, position: &Coordinate) -> HashSet<Coordinate> {
    let mut adjacent = HashSet::new();

    if position.0 > 0 {
        adjacent.insert((position.0 - 1, position.1));
    }

    if position.1 > 0 {
        adjacent.insert((position.0, position.1 - 1));
    }

    if position.0 < input[0].len() - 1 {
        adjacent.insert((position.0 + 1, position.1));
    }

    if position.1 < input.len() - 1 {
        adjacent.insert((position.0, position.1 + 1));
    }

    adjacent
}

fn create_nodes(input: &Vec<Vec<u8>>) -> HashMap<Coordinate, Node> {
    let mut node_map = HashMap::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if (x, y) == START {
                node_map.insert(
                    (x, y),
                    Node {
                        position: (x, y),
                        risk_level_from_start: Some(0),
                        predecessor: None,
                    },
                );
            } else {
                node_map.insert(
                    (x, y),
                    Node {
                        position: (x, y),
                        risk_level_from_start: None,
                        predecessor: None,
                    },
                );
            }
        }
    }

    node_map
}

fn read_input() -> Vec<Vec<u8>> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<Vec<u8>> = Vec::new();

    for line in input_vec.iter() {
        if line.is_empty() {
            break;
        }
        convert_vec.push(
            line.chars()
                .map(|c| c.to_string().parse::<u8>().expect("failed to parse number"))
                .collect::<Vec<u8>>(),
        );
    }

    convert_vec
}
