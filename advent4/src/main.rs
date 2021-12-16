use core::num;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use structs::BingoBoard;

mod structs;
fn main() {
    let (draw_numbers, mut boards) = read_input();
    let mut winner: BingoBoard = BingoBoard::default();
    let mut winner_number = 0;
    let mut remove_indexes: Vec<usize> = Vec::new();
    for (i, number) in draw_numbers.iter().enumerate() {
        boards
            .iter_mut()
            .for_each(|board| board.select_number(*number));

        if i >= 5 {
            let mut win = false;
            for (index, board) in boards.iter().enumerate() {
                if board.check_win() {
                    if boards.len() == 1 || i == draw_numbers.len() - 1 {
                        winner = board.clone();
                        winner_number = *number;
                        win = true;
                        break;
                    } else {
                        remove_indexes.push(index);
                    }
                }
            }

            remove_indexes.sort();
            for index in remove_indexes.iter().rev() {
                boards.remove(*index);
            }

            remove_indexes = Vec::new();

            if win {
                break;
            }
        }
    }

    let unmarked_sum = winner.sum_unmarked();
    println!("Unmarked sum: {}", unmarked_sum);
    println!("Final score: {}", unmarked_sum * winner_number);
}

fn read_input() -> (Vec<u32>, Vec<BingoBoard>) {
    let mut input_vec: Vec<String> = Vec::new();

    let file = File::open("input.txt").expect("could not open file");
    let mut reader = BufReader::new(file);
    let mut draw_numbers = String::new();
    reader
        .read_line(&mut draw_numbers)
        .expect("could not read first line");

    draw_numbers.split_off(draw_numbers.len() - 2).truncate(0);

    let convert_vector: Vec<u32> = draw_numbers
        .split(",")
        .map(|number| {
            number
                .to_string()
                .parse::<u32>()
                .expect("failed to convert number")
        })
        .collect();

    for line in reader.lines() {
        input_vec.push(line.expect("could not read line"));
    }

    let bingo_boards = create_bingo_boards(input_vec);

    (convert_vector, bingo_boards)
}

fn create_bingo_boards(input: Vec<String>) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut input_slice: [String; 5] = Default::default();
    for i in 0..input.len() {
        if input[i].eq("") {
            input_slice.clone_from_slice(&input[i + 1..i + 6]);
            boards.push(create_single_board(input_slice.clone()));
        }
    }
    boards
}

fn create_single_board(input_slice: [String; 5]) -> BingoBoard {
    let mut board = BingoBoard::default();

    for (y, row) in input_slice.iter().enumerate() {
        let row_numbers: Vec<u32> = row
            .clone()
            .split(' ')
            .filter(|number| !number.is_empty())
            .map(|number| {
                number
                    //.to_string()
                    .parse::<u32>()
                    .expect("failed to convert number")
            })
            .collect();

        for (x, number) in row_numbers.iter().enumerate() {
            board.set_field(*number, x, y);
        }
    }

    board
}
