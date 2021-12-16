use getset::{CopyGetters, Getters, Setters};

#[derive(Getters, Setters, Copy, Clone, Debug, CopyGetters)]
pub struct BingoField {
    #[getset(get_copy = "pub", set = "pub")]
    selected: bool,
    #[getset(get = "pub", set = "pub")]
    number: u32,
}

impl BingoField {
    pub fn new(number: u32) -> BingoField {
        BingoField {
            selected: false,
            number,
        }
    }

    pub fn default() -> Self {
        BingoField {
            selected: false,
            number: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BingoBoard {
    board: [[BingoField; 5]; 5],
}

impl BingoBoard {
    pub fn new(bingoField: [[BingoField; 5]; 5]) -> BingoBoard {
        BingoBoard { board: bingoField }
    }

    pub fn default() -> BingoBoard {
        let fields = [[BingoField::default(); 5]; 5];
        BingoBoard { board: fields }
    }

    pub fn check_win(&self) -> bool {
        for y in 0..5 {
            let mut row_checked = 0;
            for x in 0..5 {
                if self.board[x][y].selected() {
                    row_checked += 1;
                }
            }

            if row_checked == 5 {
                return true;
            }
        }

        for (x, column) in self.board.iter().enumerate() {
            let mut column_checked = 0;
            for (y, field) in column.iter().enumerate() {
                if field.selected() {
                    column_checked += 1;
                }
            }

            if column_checked == 5 {
                return true;
            }
        }
        false
    }

    pub fn set_field(&mut self, number: u32, x: usize, y: usize) {
        self.board[x][y].set_number(number);
    }

    pub fn select_number(&mut self, number: u32) {
        if let Ok((x, y)) = self.get_coordinates(number) {
            self.board[x][y].set_selected(true);
        }
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;

        for column in self.board.iter() {
            for field in column.iter() {
                if !field.selected() {
                    sum += field.number();
                }
            }
        }

        sum
    }

    fn get_coordinates(&self, number: u32) -> Result<(usize, usize), ()> {
        for (x, row) in self.board.iter().enumerate() {
            for (y, field) in row.iter().enumerate() {
                if field.number().eq(&number) {
                    return Ok((x, y));
                }
            }
        }

        Err(())
    }
}
