use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = read_file("input.txt");

    let mut increase_count: u32 = 0;

    let mut last = input[2] + input[1] + input[0];

    for (i, _) in input.iter().enumerate() {
        if i >= 2 {
            let sum = input[i] + input[i - 1] + input[i - 2];
            if sum > last {
                increase_count += 1;
            }
            last = sum;
        }
    }

    println!("The number increased {} times", increase_count);
}

fn read_file(file_name: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    let file = File::open(file_name).expect("could not find file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        numbers.push(
            line.expect("could not read line")
                .parse()
                .expect("failed to parse line"),
        );
    }
    numbers
}
