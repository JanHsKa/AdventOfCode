use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";
type Coordinate = (usize, usize);

#[derive(Debug)]
struct Octopus {
    value: u8,
    flashed: bool,
}

impl Octopus {
    pub fn new(value: u8) -> Octopus {
        Octopus {
            value,
            flashed: false,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn set_flashed(&mut self, flashed: bool) {
        self.flashed = flashed;
    }

    pub fn get_flashed(&self) -> bool {
        self.flashed
    }
}

fn main() {
    let mut input = read_input();
    let steps = 300;
    let mut steps_needed = 0;

    for i in 1..steps {
        if step(&mut input) as usize == input.len() * input[0].len() {
            steps_needed = i;
            break;
        }
    }

    println!("After {} steps all flashed simultan", steps_needed);
}

fn step(octopusses: &mut Vec<Vec<Octopus>>) -> u64 {
    increase(octopusses);
    for y in 0..octopusses.len() {
        for x in 0..octopusses[y].len() {
            if octopusses[y][x].get_value() > 9 {
                flash((x, y), octopusses);
            }
        }
    }

    reset(octopusses)
}

fn reset(octopusses: &mut Vec<Vec<Octopus>>) -> u64 {
    let mut flashes = 0;

    for i in octopusses {
        for j in i {
            if j.get_flashed() {
                j.set_flashed(false);
                j.set_value(0);
                flashes += 1;
            }
        }
    }
    flashes
}

fn flash(position: Coordinate, octopusses: &mut Vec<Vec<Octopus>>) {
    if !octopusses[position.1][position.0].get_flashed() {
        octopusses[position.1][position.0].set_flashed(true);
        increase_adjacent(position, octopusses);
    }
}

fn increase_adjacent(position: Coordinate, octopusses: &mut Vec<Vec<Octopus>>) {
    let adjacent = get_adjacent_coordinates(position, octopusses.len(), octopusses[0].len());
    for i in adjacent.iter() {
        let value = octopusses[i.1][i.0].get_value();
        octopusses[i.1][i.0].set_value(value + 1);
    }

    for i in adjacent {
        if octopusses[i.1][i.0].get_value() > 9 {
            flash(i, octopusses);
        }
    }
}

fn get_adjacent_coordinates(position: Coordinate, height: usize, width: usize) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();

    if position.0 > 0 {
        coordinates.push((position.0 - 1, position.1));
        if position.1 > 0 {
            coordinates.push((position.0 - 1, position.1 - 1));
        }
        if position.1 < height - 1 {
            coordinates.push((position.0 - 1, position.1 + 1));
        }
    }

    if position.1 > 0 {
        coordinates.push((position.0, position.1 - 1));
        if position.0 < width - 1 {
            coordinates.push((position.0 + 1, position.1 - 1));
        }
    }

    if position.0 < width - 1 {
        coordinates.push((position.0 + 1, position.1));
        if position.1 < height - 1 {
            coordinates.push((position.0 + 1, position.1 + 1));
        }
    }

    if position.1 < height - 1 {
        coordinates.push((position.0, position.1 + 1));
    }

    coordinates
}

fn increase(octopusses: &mut Vec<Vec<Octopus>>) {
    for i in octopusses {
        for j in i {
            j.set_value(j.get_value() + 1);
        }
    }
}

fn read_input() -> Vec<Vec<Octopus>> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut convert_vec: Vec<Vec<Octopus>> = Vec::new();
    for line in input_vec {
        convert_vec.push(
            line.chars()
                .map(|s| Octopus::new(s.to_string().parse::<u8>().expect("failed to parse number")))
                .collect::<Vec<Octopus>>(),
        );
    }

    convert_vec
}
