use crate::{Cell, grid::Grid, utils::unflatten};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::Display;

pub struct GameBoard {
    width: u8,
    height: u8,
    solved_board: Grid<Cell>,
    board: Grid<Cell>,
    num_bombs: u16,
    placed_flags: u16,
}

impl GameBoard {
    pub fn new(width: u8, height: u8, num_bombs: u16) -> Result<Self, &'static str> {
        let num_cells = height as u16 * width as u16;
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
                if let Some(cell) = solved_board.get_cell(x.into(), y.into()) {
                    if cell != Cell::Bomb {
                        solved_board.set_cell(
                            x.into(),
                            y.into(),
                            Cell::Number(solved_board.adj_bombs(x, y).len() as u8),
                        )?;
                    }
                }
            }
        }

        Ok(GameBoard {
            width,
            height,
            solved_board,
            board: Grid::new(width, height, Cell::Unknown),
            num_bombs,
            placed_flags: 0,
        })
    }

    pub fn is_solved(&self) -> bool {
        for _y in 0..self.height {
            let y = (self.height - _y - 1) as i16;
            for x in 0..self.width {
                let solved_elem = self.solved_board.get_cell(x as i16, y).unwrap();
                let player_elem = self.board.get_cell(x as i16, y).unwrap();

                if let Cell::Number(_) = solved_elem {
                    if solved_elem == player_elem {
                        continue;
                    } else {
                        return false;
                    }
                } else if solved_elem == Cell::Bomb {
                    if player_elem == Cell::Unknown || player_elem == Cell::Flag {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn flood_fill(&mut self, x: i16, y: i16) {
        match self.board.get_cell(x, y) {
            None => return,
            Some(cell) => {
                // if player cell is already open, stop floodfill
                if cell != Cell::Unknown {
                    return;
                }
            }
        }

        match self.solved_board.get_cell(x, y) {
            None => return,
            Some(cell) => {
                // if solved cell is a bomb, stop floodfill
                if cell == Cell::Bomb {
                    return;
                }

                // set player cell as solved cell value
                self.board.set_cell(x, y, cell).unwrap();

                // if solved cell value not zero, stop floodfill
                // ie. continue floodfill only if cell is zero
                if cell != Cell::Number(0) {
                    return;
                }

                // flood_fill_all_adj;
                self.flood_fill_all_adj(x, y);
            }
        }
    }

    pub fn flood_fill_all_adj(&mut self, x: i16, y: i16) {
        let deltas: [(i16, i16); 8] = [
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        for d in deltas {
            self.flood_fill(x + d.0, y + d.1);
        }
    }

    pub fn chord(&mut self, x: i16, y: i16) {
        if let Some(cell) = self.board.get_cell(x, y) {
            match cell {
                Cell::Number(num) => {
                    if num == 0 {
                        return; // return if cell is 0
                    }
                    if self.board.adj_flags(x as u8, y as u8).len() != num as usize {
                        return; // not enough adjacent flags to chord
                    }
                }
                _ => return, // return if cell is not a number
            }

            self.flood_fill_all_adj(x, y);
        }
    }

    pub fn chord_all(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.chord(x.into(), y.into());
            }
        }
    }

    pub fn place_flags(&mut self, x: u8, y: u8) {
        if let Some(Cell::Number(num)) = self.board.get_cell(x.into(), y.into()) {
            let adj = self
                .board
                .adj_cells(x, y, HashSet::from([Cell::Unknown, Cell::Flag]));
            if adj.len() == num as usize {
                for d in adj {
                    self.board
                        .set_cell(x as i16 + d.0 as i16, y as i16 + d.1 as i16, Cell::Flag)
                        .unwrap();
                }
            }
        }
    }

    pub fn place_all_flags(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.place_flags(x, y);
            }
        }
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}\n\n{}", self.solved_board, self.board).as_str())
    }
}
