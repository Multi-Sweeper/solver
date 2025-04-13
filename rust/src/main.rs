#[macro_use]
extern crate time_test;

use board::GameBoard;
use colour::Coloured;
use std::time::Instant;
use strategy::strategy_simple;

mod board;
mod colour;
mod grid;
mod solve;
mod strategy;
mod utils;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Cell {
    Number(u8),
    Flag,
    Bomb,
    Unknown,
    Asterix,
}

impl Coloured for Cell {
    fn to_coloured(&self, background: Option<(u8, u8, u8)>) -> String {
        match self {
            Cell::Number(num) => num.to_string().to_coloured(background),
            Cell::Bomb => String::from("B").to_coloured(background),
            Cell::Flag => String::from("F").to_coloured(background),
            Cell::Unknown => String::from("?").to_coloured(background),
            Cell::Asterix => String::from("*").to_coloured(background),
        }
    }
}

fn main() {
    // let board: GameBoard = GameBoard::new(9, 9, 10).unwrap();
    let board: GameBoard = GameBoard::new(16, 16, 40).unwrap();
    // let board: GameBoard = GameBoard::new(30, 16, 99).unwrap();
    // let board: GameBoard = GameBoard::new(16, 16, 10).unwrap();

    // determine all possible starting cells
    let mut starting_cells: Vec<(u8, u8)> = Vec::new();
    let mut temp_board = board.clone();
    for cell in temp_board.solved_board.get_iter() {
        let (x, y) = cell.pos;
        if cell.val == Cell::Number(0) {
            if temp_board.board.get_cell(x.into(), y.into()) == Some(Cell::Unknown) {
                temp_board.flood_fill(x.into(), y.into());
                starting_cells.push((x, y));
            }
        }
    }

    let start_solve_time = Instant::now();
    let mut step_summary: Option<Vec<Vec<&str>>> = None;
    for starting_cell in &starting_cells[0..1] {
        step_summary = strategy_simple(board.clone(), starting_cell.to_owned());
    }

    if step_summary.is_some() {
        println!("Solved in: {}ms", start_solve_time.elapsed().as_millis());
        println!("step summary: {:?}", step_summary);
    } else {
        println!("could not solve");
    }
}
