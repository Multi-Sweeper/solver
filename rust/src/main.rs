use board::GameBoard;
use colour::Coloured;

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
    // let mut board: GameBoard = GameBoard::new(9, 9, 10).unwrap();
    // let mut board: GameBoard = GameBoard::new(16, 16, 40).unwrap();
    // let mut board: GameBoard = GameBoard::new(30, 16, 99).unwrap();
    let mut board: GameBoard = GameBoard::new(16, 16, 5).unwrap();

    board.flood_fill(0, 0);
    println!("{}\n{}", board, board.is_solved());

    board.place_all_flags();
    println!("{}\n{}", board, board.is_solved());
}
