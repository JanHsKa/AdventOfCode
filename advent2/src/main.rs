use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_vec = read_input();
    // let input_vec: Vec<&str> = vec![
    //     "forward 5",
    //     "down 5",
    //     "forward 8",
    //     "up 3",
    //     "down 8",
    //     "forward 2",
    // ];

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    let mut aim: i32 = 0;

    for command in input_vec.iter() {
        let parts: Vec<&str> = command.split(" ").collect();
        let converted: i32 = parts[1]
            .to_string()
            .parse()
            .expect("failed to parse number");
        match parts[0] {
            "forward" => {
                horizontal += converted;
                vertical += aim * converted
            }
            "down" => aim += converted,
            "up" => aim -= converted,
            _ => println!("no matching part"),
        }
    }

    println!("The product ist: {}", horizontal * vertical);
}

fn read_input() -> Vec<String> {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open("input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    input_vec
}
