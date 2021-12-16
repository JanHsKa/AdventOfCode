use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DIAGRAM_SIZE: usize = 1000;
type Coordinates = (u32, u32);
type Line = (Coordinates, Coordinates);
type Diagram = Vec<Vec<u32>>;
fn main() {
    let mut diagram: Diagram = Vec::with_capacity(DIAGRAM_SIZE);
    for _ in 0..DIAGRAM_SIZE {
        diagram.push(vec![0; DIAGRAM_SIZE]);
    }
    let input = read_input();

    for line in input {
        if (line.0 .0 == line.1 .0) || (line.0 .1 == line.1 .1) {
            mark_line(&line, &mut diagram);
        } else {
            mark_diagonal(&line, &mut diagram);
        }
    }

    let crosses = count_crossings(&diagram);
    println!("Crosses: {}", crosses);
}

#[allow(dead_code)]
fn print_diagram(diagram: &Diagram) {
    for y in 0..diagram.len() {
        for x in 0..diagram.len() {
            if diagram[x][y] == 0 {
                print!(".")
            } else {
                print!("{}", diagram[x][y])
            }
        }
        println!();
    }
}

fn count_crossings(diagram: &Diagram) -> usize {
    let crosses = diagram
        .iter()
        .flat_map(|i| i.iter())
        .filter(|number| **number >= 2)
        .map(|n| *n)
        .collect::<Vec<u32>>()
        .len();

    crosses
}

fn mark_diagonal(line: &Line, diagram: &mut Diagram) {
    let mut start_x = line.0 .0 as i32;
    let mut start_y = line.0 .1 as i32;
    let end_x = line.1 .0 as i32;
    let end_y = line.1 .1 as i32;
    let mut x_increase: i32 = 1;
    let mut y_increase: i32 = 1;
    let len = (start_x - end_x).abs();
    if start_x > end_x {
        x_increase *= -1;
    }

    if start_y > end_y {
        y_increase *= -1;
    }

    for _ in 0..len + 1 {
        diagram[start_x as usize][start_y as usize] += 1;
        start_x += x_increase;
        start_y += y_increase;
    }
}

fn mark_line(line: &Line, diagram: &mut Diagram) {
    let ((start_x, start_y), (end_x, end_y)) = determine_start_coords(line);

    for x in start_x..end_x + 1 {
        for y in start_y..end_y + 1 {
            diagram[x as usize][y as usize] += 1;
        }
    }
}

fn determine_start_coords(line: &Line) -> Line {
    let start_x: u32;
    let end_x: u32;
    let start_y: u32;
    let end_y: u32;

    if line.0 .0 < line.1 .0 {
        start_x = line.0 .0;
        end_x = line.1 .0;
    } else {
        start_x = line.1 .0;
        end_x = line.0 .0;
    }

    if line.0 .1 < line.1 .1 {
        start_y = line.0 .1;
        end_y = line.1 .1;
    } else {
        start_y = line.1 .1;
        end_y = line.0 .1;
    }

    ((start_x, start_y), (end_x, end_y))
}

fn read_input() -> Vec<Line> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open("input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<String>;
    let mut line_vector: Vec<Line> = Vec::new();
    for line in input_vec {
        convert_vec = line
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        line_vector.push(create_line_input(convert_vec));
    }

    line_vector
}

fn create_line_input(input: Vec<String>) -> Line {
    let start: Coordinates;
    let end: Coordinates;
    let mut numbers: Vec<u32>;

    numbers = input[0]
        .split(",")
        .map(|s| {
            s.to_string()
                .parse::<u32>()
                .expect("failed to parse number")
        })
        .collect();

    start = (numbers[0], numbers[1]);

    numbers = input[2]
        .split(",")
        .map(|s| {
            s.to_string()
                .parse::<u32>()
                .expect("failed to parse number")
        })
        .collect();

    end = (numbers[0], numbers[1]);

    (start, end)
}
