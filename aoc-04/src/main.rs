use aoc_core::bit::bitmap::Bitmap;
use aoc_core::puzzle_input::PuzzleInput;
use std::fmt::{Debug, Display, Formatter};
use std::str::Lines;

fn main() {
    let input = PuzzleInput::new("aoc-04/input.txt");
    let lines = input.to_lines();

    let mut bingo_bot: BingoBot = lines.into();
    let sorted_boards = bingo_bot.sort_boards();
    let first = sorted_boards.first().expect("No winner!");
    println!(
        "First board to win:\n{}\nFinal score: {}\n",
        first,
        first.get_final_score().expect("No final score recorded!")
    );
    let last = sorted_boards.last().expect("No winner!");
    println!(
        "Last board to win:\n{}\nFinal score: {}\n",
        last,
        last.get_final_score().expect("No final score recorded!")
    );
}

const fn new_const(value: u32, bitmap_size: usize) -> Bitmap<u32> {
    Bitmap::new_const(value, bitmap_size)
}

static WINNING_BOARDS: [Bitmap; 12] = {
    let row0 = new_const(31 << 0, 25);
    let col0 = new_const(1_082_401 << 0, 25);
    let row1 = new_const(31 << 5, 25);
    let col1 = new_const(1_082_401 << 1, 25);
    let row2 = new_const(31 << 10, 25);
    let col2 = new_const(1_082_401 << 2, 25);
    let row3 = new_const(31 << 15, 25);
    let col3 = new_const(1_082_401 << 3, 25);
    let row4 = new_const(31 << 20, 25);
    let col4 = new_const(1_082_401 << 4, 25);
    let left_down_diag = new_const(17_043_521u32, 25);
    let right_down_diag = new_const(1_118_480u32, 25);
    let boards: [Bitmap; 12] = [
        row0,
        col0,
        row1,
        col1,
        row2,
        col2,
        row3,
        col3,
        row4,
        col4,
        left_down_diag,
        right_down_diag,
    ];
    boards
};

/// A helpful robot that runs the bingo game, drawing numbers, keeping track of winners, and making sure everyone's marked their boards.
struct BingoBot {
    boards: Vec<BingoBoard>,
    drawings: Vec<u8>,
    current_drawing: usize,
}

impl BingoBot {
    pub fn new(boards: Vec<BingoBoard>, drawings: Vec<u8>) -> Self {
        BingoBot {
            boards,
            drawings,
            current_drawing: 0,
        }
    }

    pub fn sort_boards(&mut self) -> &Vec<BingoBoard> {
        let mut sorted: Vec<BingoBoard> = Vec::new();
        let num_boards_total = self.boards.len();
        while sorted.len() != num_boards_total {
            let drawn_number = self.drawings[self.current_drawing];
            let mut idx = 0usize;
            while idx < self.boards.len() {
                self.boards[idx].mark(drawn_number);
                if self.boards[idx].is_winner() {
                    self.boards[idx].set_final_score(drawn_number);
                    sorted.push(self.boards.remove(idx));
                }
                idx += 1;
            }
            self.current_drawing += 1;
        }
        self.boards = sorted;
        &self.boards
    }
}

impl<'a> From<Lines<'a>> for BingoBot {
    fn from(mut lines: Lines) -> Self {
        // Parse drawings
        let drawings: Vec<u8> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        lines.next();

        // Parse boards
        let mut bingo_board_builders: Vec<String> = vec![String::new()];
        for line in lines {
            match line {
                "" => bingo_board_builders.push(String::new()),
                _ => {
                    let idx = bingo_board_builders.len() - 1;
                    bingo_board_builders[idx].push_str(" ");
                    bingo_board_builders[idx].push_str(line);
                }
            }
        }
        let boards = bingo_board_builders
            .iter()
            .map(|x| x.as_str().into())
            .collect();

        BingoBot::new(boards, drawings)
    }
}

impl Display for BingoBot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, board) in self.boards.iter().enumerate() {
            f.write_str(format!("Board {:02}:\n", index + 1).as_str())?;
            std::fmt::Display::fmt(&board, f)?;
        }
        Ok(())
    }
}

/// A 5x5 bingo board.
#[derive(Debug, Clone)]
struct BingoBoard {
    /// The numbers at each position of the board, from left-to-right, top-to-bottom (left-topmost is index 0, right-bottommost is index 24).
    board: Vec<u8>,
    /// A bitmap where each bit position corresponds to whether the board at that index has been marked.
    marked: Bitmap,
    /// The board's final score.
    final_score: Option<u32>,
}

impl BingoBoard {
    pub fn new(board: Vec<u8>) -> Self {
        if board.len() != 25 {
            panic!("Bingo boards can only be 25 elements long!")
        }
        let marked = Bitmap::new(0, 25);
        BingoBoard {
            board,
            marked,
            final_score: None,
        }
    }

    /// Marks the number on the bingo board, if it exists.
    pub fn mark(&mut self, number: u8) {
        let found = self
            .board
            .iter()
            .enumerate()
            .find(|(_, val)| **val == number);
        if let Some(item) = found {
            self.marked.set(item.0, true);
        }
    }

    /// Checks if the board is a winner against the static list of winning boards.
    /// We detect a board has won if the result of `self.marked & winner` is equal to `winner`.
    pub fn is_winner(&self) -> bool {
        WINNING_BOARDS
            .iter()
            .any(|winner| *winner == *winner & self.marked)
    }

    fn sum_unmarked(&self) -> u32 {
        self.board
            .iter()
            .enumerate()
            .fold(0u32, |acc, (index, val)| match self.marked.get(index) {
                false => acc + *val as u32,
                true => acc,
            })
    }

    pub fn get_final_score(&self) -> Option<u32> {
        self.final_score
    }

    pub fn set_final_score(&mut self, winning_number: u8) {
        self.final_score = Some(self.sum_unmarked() * winning_number as u32)
    }
}

impl From<&str> for BingoBoard {
    fn from(input: &str) -> Self {
        let trimmed = input.replace("  ", " ");
        let board = trimmed
            .trim()
            .split(' ')
            .map(|x| x.parse().expect(format!("{} is not parsable!", x).as_str()))
            .collect();
        BingoBoard::new(board)
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self
            .board
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (index, val)| {
                let mut item = format!("{:^4}", format!("{:02}", val));
                if self.marked.get(index) {
                    item.replace_range(0..1, ">");
                    item.replace_range(3..4, "<");
                }
                if index % 5 == 4 {
                    item.push_str("\n");
                }
                format!("{}{}", acc, item)
            });
        f.write_str(result.as_str())
    }
}
