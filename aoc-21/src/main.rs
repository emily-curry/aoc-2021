mod classical;
mod pawn;
mod quantum;

use crate::classical::classical_game_board::ClassicalGameBoard;
use crate::quantum::quantum_game_board::QuantumGameBoard;
use aoc_core::puzzle_input::PuzzleInput;
use std::time::SystemTime;

fn main() {
    play_classic();

    play_quantum();
}

fn play_classic() {
    let input = PuzzleInput::new("aoc-21/input.txt");
    let mut board = ClassicalGameBoard::from(input.to_lines());
    board.play();
    let loser = match board.get_p1().get_score() > board.get_p2().get_score() {
        true => board.get_p2(),
        false => board.get_p1(),
    };
    let rolls = board.get_die().get_roll_count();
    println!(
        "Product of loser's score and roll count: {}",
        loser.get_score() as u32 * rolls
    );
}

fn play_quantum() {
    let start = SystemTime::now();
    let input = PuzzleInput::new("aoc-21/input.txt");
    let mut board = QuantumGameBoard::from(input.to_lines());
    board.play();
    println!(
        "Quantum game completed in {} seconds!",
        start.elapsed().unwrap().as_secs_f32()
    );
    let counts = board.count_winners();
    let winner = if counts.0 > counts.1 { "1" } else { "2" };
    println!(
        "Player 1 wins in {} universes, Player 2 wins in {} universes. Player {} wins!",
        counts.0, counts.1, winner
    );
}
