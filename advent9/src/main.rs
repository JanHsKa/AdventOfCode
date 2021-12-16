use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";
type Coordinate = (usize, usize);
fn main() {
    let input = read_input();
    let mut low_points: Vec<Coordinate> = Vec::new();
    let mut product: u32 = 1;
    let mut basins: Vec<HashSet<Coordinate>> = Vec::new();
    let mut largest: Vec<u32> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let adjacents = get_adjacents(&input, (x, y));
            if is_lower(input[y][x], &adjacents) {
                low_points.push((x, y));
            }
        }
    }

    for i in low_points {
        let mut new_basin: HashSet<Coordinate> = HashSet::new();
        new_basin.insert(i);
        get_basins(i, &mut new_basin, &input);
        basins.push(new_basin);
    }

    for basin in basins {
        if largest.len() < 3 {
            largest.push(basin.len() as u32);
            largest.sort();
        } else if largest[0] < basin.len() as u32 {
            largest[0] = basin.len() as u32;
            largest.sort();
        }
    }

    for i in largest {
        product *= i;
    }

    println!("Product is {}", product);
}

fn get_basins(point: Coordinate, basins: &mut HashSet<Coordinate>, input: &Vec<Vec<u8>>) {
    let width = input[0].len();
    let height = input.len();
    let mut new_point: Coordinate;

    if point.0 > 0 {
        new_point = (point.0 - 1, point.1);
        if input[point.1][point.0 - 1] != 9 && !basins.contains(&new_point) {
            basins.insert((point.0 - 1, point.1));
            get_basins((point.0 - 1, point.1), basins, input);
        }
    }

    if point.0 < width - 1 {
        new_point = (point.0 + 1, point.1);

        if input[new_point.1][new_point.0] != 9 && !basins.contains(&new_point) {
            basins.insert((new_point.0, new_point.1));
            get_basins((new_point.0, new_point.1), basins, input);
        }
    }

    if point.1 > 0 {
        new_point = (point.0, point.1 - 1);

        if input[new_point.1][new_point.0] != 9 && !basins.contains(&new_point) {
            basins.insert((new_point.0, new_point.1));
            get_basins((new_point.0, new_point.1), basins, input);
        }
    }

    if point.1 < height - 1 {
        new_point = (point.0, point.1 + 1);

        if input[new_point.1][new_point.0] != 9 && !basins.contains(&new_point) {
            basins.insert((new_point.0, new_point.1));
            get_basins((new_point.0, new_point.1), basins, input);
        }
    }
}

fn is_lower(point: u8, adjacents: &[u8]) -> bool {
    for i in adjacents {
        if point >= *i {
            return false;
        }
    }
    true
}

fn get_adjacents(input: &Vec<Vec<u8>>, field: Coordinate) -> Vec<u8> {
    let mut adjacents = Vec::new();
    let width = input[0].len();
    let height = input.len();

    if field.0 > 0 {
        adjacents.push(input[field.1][field.0 - 1]);
    }

    if field.0 < width - 1 {
        adjacents.push(input[field.1][field.0 + 1]);
    }

    if field.1 > 0 {
        adjacents.push(input[field.1 - 1][field.0]);
    }

    if field.1 < height - 1 {
        adjacents.push(input[field.1 + 1][field.0]);
    }
    adjacents
}

fn read_input() -> Vec<Vec<u8>> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<Vec<u8>> = Vec::new();
    for line in input_vec {
        convert_vec.push(
            line.chars()
                .map(|s| s.to_string().parse::<u8>().expect("failed to parse"))
                .collect::<Vec<u8>>(),
        );
    }

    convert_vec
}
