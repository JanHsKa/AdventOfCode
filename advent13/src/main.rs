use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "input.txt";
type Coordinate = (usize, usize);
type Fold = (u8, u32);

fn main() {
    let input = read_input();
    let mut new_fold = create_field(input.0);

    //println!();

    for i in input.1 {
        new_fold = fold_paper(new_fold, i);
    }

    print_vec(&new_fold);

    let dots = count_dots(new_fold);

    println!("There are {} dots left", dots);
}

fn find_highest(input: &Vec<Vec<usize>>) -> Coordinate {
    let mut high_x = 0;
    let mut highy = 0;

    for i in input {
        if i[0] > high_x {
            high_x = i[0];
        }

        if i[1] > highy {
            highy = i[1];
        }
    }

    (high_x, highy)
}

fn count_dots(field: Vec<Vec<u8>>) -> u32 {
    let mut dots: u32 = 0;

    field.into_iter().flat_map(|f| f.into_iter()).for_each(|f| {
        if f == 1 {
            dots += 1;
        };
    });

    dots
}

fn print_vec(input: &Vec<Vec<u8>>) {
    for i in input {
        println!("{:?}", i);
    }
    println!();
}

fn fold_paper(input: Vec<Vec<u8>>, instruction: Fold) -> Vec<Vec<u8>> {
    let mut new_vec: Vec<Vec<u8>>;
    let mut width = input[0].len();
    let mut height = input.len();

    if instruction.0 == 1 {
        height /= 2;
    } else {
        width /= 2;
    }

    new_vec = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            if input[y][x] == 1 {
                new_vec[y][x] = 1;
            }
        }
    }

    if instruction.0 == 1 {
        let mut new_y = 0;
        for y in (height..input.len()).rev() {
            for x in 0..width {
                if input[y][x] == 1 {
                    new_vec[new_y][x] = 1;
                }
            }
            new_y += 1;
        }
    } else {
        for y in 0..height {
            let mut new_x = 0;
            for x in (width..input[0].len()).rev() {
                if input[y][x] == 1 {
                    new_vec[y][new_x] = 1;
                }
                new_x += 1;
            }
        }
    }

    new_vec
}

fn create_field(input: Vec<Vec<usize>>) -> Vec<Vec<u8>> {
    let (width, height) = find_highest(&input);
    let mut field = vec![vec![0; width + 1]; height + 1];

    for i in input {
        field[i[1]][i[0]] = 1;
    }

    field
}

fn read_input() -> (Vec<Vec<usize>>, Vec<Fold>) {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open(INPUT).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let mut instructions: Vec<Fold> = Vec::new();
    let mut convert_vec: Vec<Vec<usize>> = Vec::new();

    for line in input_vec.iter() {
        if line.is_empty() {
            break;
        }
        convert_vec.push(
            line.split(",")
                .map(|s| s.to_string().parse::<usize>().expect("failed to parse"))
                .collect::<Vec<usize>>(),
        );
    }

    for i in convert_vec.len() + 1..input_vec.len() {
        let splits: Vec<String> = input_vec[i].split(" ").map(|f| f.to_string()).collect();
        let foldable: Vec<String> = splits[2].split("=").map(|f| f.to_string()).collect();
        let x_y: u8;
        if foldable[0].eq("y") {
            x_y = 1;
        } else {
            x_y = 0;
        }

        instructions.push((x_y, foldable[1].parse().expect("failed to parse number")));
    }

    (convert_vec, instructions)
}
