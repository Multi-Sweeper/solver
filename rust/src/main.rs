use grid::Grid;

mod grid;
mod utils;

fn main() {
    let grid = Grid::new(16, 16, String::from("0"));

    println!("{}", grid)
}
