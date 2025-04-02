use colour::Coloured;
use grid::Grid;

mod colour;
mod grid;

#[derive(Clone, Hash, PartialEq, Eq)]
enum Cell {
    Number(u8),
    Flag,
    Bomb,
    Empty,
    Asterix,
}

impl Coloured for Cell {
    fn to_coloured(&self) -> String {
        match self {
            Cell::Number(num) => num.to_string().to_coloured(),
            Cell::Bomb => String::from("B").to_coloured(),
            Cell::Flag => String::from("F").to_coloured(),
            Cell::Empty => String::from("?").to_coloured(),
            Cell::Asterix => String::from("*").to_coloured(),
        }
    }
}

fn main() {
    let grid: Grid<Cell> = Grid::new(16, 16, Cell::Empty);

    println!("{}", grid)
}
