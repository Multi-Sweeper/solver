use grid::Grid;

mod colour;
mod grid;

fn main() {
    let grid = Grid::new(16, 16, String::from("0"));

    println!("{}", grid)
}
