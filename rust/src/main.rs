use board::GameBoard;
use colour::Coloured;
use std::{io, time::Instant, vec};

mod board;
mod colour;
mod grid;
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
    fn to_coloured(&self) -> String {
        match self {
            Cell::Number(num) => num.to_string().to_coloured(),
            Cell::Bomb => String::from("B").to_coloured(),
            Cell::Flag => String::from("F").to_coloured(),
            Cell::Unknown => String::from("?").to_coloured(),
            Cell::Asterix => String::from("*").to_coloured(),
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

    let mut solved = false;
    let start_solve_time = Instant::now();
    let mut step_summary: Vec<Vec<&str>> = Vec::new();

    for starting_cell in starting_cells {
        let mut game_board = board.clone();
        game_board.flood_fill(starting_cell.0.into(), starting_cell.1.into());

        println!("= init =======================================");
        println!("{}", game_board);

        step_summary.clear();
        let mut i = 0;
        solved = false;
        let mut progress = true;

        while !solved && progress {
            println!("= {} =======================================", i + 1);
            let start_time = Instant::now();
            step_summary.push(vec!["basic"]);
            progress = game_board.simple_solve_step();
            if !progress {
                step_summary.last_mut().unwrap().push("permute");
                progress = game_board.permute_solve_step();
            }

            println!("{}", game_board);
            println!(
                "progress: {} |  time taken: {}ms",
                progress,
                start_time.elapsed().as_millis()
            );

            solved = game_board.is_solved();
            i += 1;

            // std::io::stdin().read_line(&mut String::new());
        }

        if solved {
            break;
        }
    }

    if solved {
        println!("Solved in: {}ms", start_solve_time.elapsed().as_millis());
        println!("step summary: {:?}", step_summary);
    } else {
        println!("could not solve");
    }
}
