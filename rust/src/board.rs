use crate::{Cell, grid::Grid, utils::unflatten};
use rand::seq::SliceRandom;
use std::fmt::Display;

pub struct GameBoard {
    solved_board: Grid<Cell>,
    board: Grid<Cell>,
    num_bombs: u16,
    placed_flags: u16,
}

impl GameBoard {
    pub fn new(width: u8, height: u8, num_bombs: u16) -> Result<Self, &'static str> {
        let num_cells = (height as u16 * width as u16);
        assert!(
            num_bombs <= num_cells,
            "num_bombs must be less than or equal to width*height"
        );

        let mut solved_board: Vec<Cell> = vec![Cell::Bomb; num_bombs as usize];
        for _ in 0..(num_cells - num_bombs) {
            solved_board.push(Cell::Number(0));
        }

        let mut rng = rand::rng();
        solved_board.shuffle(&mut rng);

        let mut solved_board = Grid::from(unflatten(solved_board, width, height), width, height)?;

        // solve board
        for x in 0..width {
            for y in 0..height {
                if let Some(cell) = solved_board.get_cell(x as i16, y as i16) {
                    if cell != Cell::Bomb {
                        solved_board.set_cell(
                            x,
                            y,
                            Cell::Number(solved_board.adj_bombs(x, y).len() as u8),
                        );
                    }
                }
            }
        }

        Ok(GameBoard {
            solved_board,
            board: Grid::new(width, height, Cell::Unknown),
            num_bombs,
            placed_flags: 0,
        })
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}\n\n{}", self.solved_board, self.board).as_str())
    }
}
