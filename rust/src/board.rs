use crate::{Cell, grid::Grid, utils::unflatten};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Clone)]
pub struct GameBoard {
    pub width: u8,
    pub height: u8,
    pub solved_grid: Grid<Cell>,
    pub grid: Grid<Cell>,
    pub flag_adj_grid: Grid<Option<u8>>,
    num_bombs: u16,
    placed_flags: u16,
}

impl GameBoard {
    pub fn new(width: u8, height: u8, num_bombs: u16) -> Result<Self, String> {
        let num_cells = height as u16 * width as u16;
        assert!(
            num_bombs <= num_cells,
            "num_bombs must be less than or equal to width*height"
        );

        // create vec with n Cell::Bomb's with Cell::Number(0) right padded to fill up the desired board size
        let mut solved_board: Vec<Cell> = vec![Cell::Bomb; num_bombs as usize];
        for _ in 0..(num_cells - num_bombs) {
            solved_board.push(Cell::Number(0));
        }

        // randomize cell positions
        let mut rng = rand::rng();
        solved_board.shuffle(&mut rng);

        // unflatten vec into grid
        let solved_board = Grid::from(unflatten(solved_board, width, height)?, width, height)?;

        GameBoard::from(solved_board)
    }

    pub fn from(mut solved_grid: Grid<Cell>) -> Result<Self, String> {
        // populate grid cell with correct Cell::Number
        for cell in solved_grid.get_iter() {
            let (x, y) = cell.pos;

            if cell.val != Cell::Bomb {
                solved_grid.set_cell(
                    x.into(),
                    y.into(),
                    Cell::Number(solved_grid.adj_bombs(x, y)?.len() as u8),
                )?;
            }
        }

        let num_bombs = solved_grid
            .get_iter()
            .filter(|cell| cell.val == Cell::Bomb)
            .count() as u16;

        let width = solved_grid.width;
        let height = solved_grid.height;

        let mut flag_adj_grid = Grid::new(width, height, None);
        for cell in solved_grid.get_iter() {
            let (x, y) = cell.pos;
            match cell.val {
                Cell::Number(num) => flag_adj_grid.set_cell(x.into(), y.into(), Some(num))?,
                _ => continue,
            }
        }

        Ok(GameBoard {
            width,
            height,
            solved_grid,
            grid: Grid::new(width, height, Cell::Unknown),
            flag_adj_grid,
            num_bombs,
            placed_flags: 0,
        })
    }

    pub fn from_str(solved_grid_str: &str, player_grid_str: &str) -> Result<Self, String> {
        let solved_grid = Grid::from_str(solved_grid_str)?;
        let player_grid = Grid::from_str(player_grid_str)?;

        if solved_grid.width != player_grid.width {
            return Err("solved_grid.width != player_grid.width".to_string());
        } else if solved_grid.height != player_grid.height {
            return Err("solved_grid.height != player_grid.height".to_string());
        }

        let num_bombs = solved_grid
            .get_iter()
            .filter(|c| c.val == Cell::Bomb)
            .count() as u16;

        let placed_flags = player_grid
            .get_iter()
            .filter(|c| c.val == Cell::Flag)
            .count() as u16;

        let mut flag_adj_grid = Grid::new(solved_grid.width, solved_grid.height, None);
        for cell in solved_grid.get_iter() {
            let (x, y) = cell.pos;
            match cell.val {
                Cell::Number(num) => flag_adj_grid.set_cell(x.into(), y.into(), Some(num))?,
                _ => continue,
            }
        }

        Ok(GameBoard {
            width: solved_grid.width,
            height: solved_grid.height,
            solved_grid,
            grid: player_grid,
            flag_adj_grid,
            num_bombs,
            placed_flags,
        })
    }

    pub fn is_solved(&self) -> Result<bool, String> {
        for cell in self.solved_grid.get_iter() {
            let (x, y) = cell.pos;

            let solved_elem = cell.val;
            let player_elem = self.grid.get_cell(x as i16, y as i16)?;

            if let Cell::Number(_) = solved_elem {
                if solved_elem == player_elem {
                    continue;
                } else {
                    return Ok(false);
                }
            } else if solved_elem == Cell::Bomb {
                if player_elem == Cell::Unknown || player_elem == Cell::Flag {
                    continue;
                } else {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub fn flood_fill(&mut self, x: i16, y: i16) -> Result<(), String> {
        match self.grid.get_cell(x, y) {
            Err(_) => return Ok(()),
            Ok(Cell::Unknown) => (),
            _ => return Ok(()),
        }

        if let Ok(cell) = self.solved_grid.get_cell(x, y) {
            if cell == Cell::Bomb {
                return Ok(());
            }
            // set player cell as solved cell value
            self.grid.set_cell(x, y, cell)?;
            // if solved cell value not zero, stop floodfill
            // ie. continue floodfill only if cell is zero
            if cell != Cell::Number(0) {
                return Ok(());
            }
        }

        self.flood_fill_all_adj(x, y)
    }

    pub fn flood_fill_all_adj(&mut self, x: i16, y: i16) -> Result<(), String> {
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
            self.flood_fill(x + d.0, y + d.1)?;
        }

        Ok(())
    }

    pub fn chord(&mut self, x: i16, y: i16) -> Result<(), String> {
        let cell = self.grid.get_cell(x, y)?;

        match cell {
            Cell::Number(num) => {
                if num == 0 {
                    return Ok(()); // return if cell is 0
                }
                if self.grid.adj_flags(x as u8, y as u8)?.len() != num as usize {
                    return Ok(()); // not enough adjacent flags to chord
                }
            }
            _ => return Ok(()), // return if cell is not a number
        }

        self.flood_fill_all_adj(x, y)
    }

    pub fn chord_all(&mut self) -> Result<(), String> {
        for y in 0..self.height {
            for x in 0..self.width {
                self.chord(x.into(), y.into())?;
            }
        }

        Ok(())
    }

    pub fn place_flag(&mut self, x: u8, y: u8) -> Result<(), String> {
        self.grid.set_cell(x.into(), y.into(), Cell::Flag)?;
        // for (x, y) in self.grid.adj_cells(x, y, None) {
        //     self.flag_adj_grid.incr_cell(x.into(), y.into()).unwrap();
        // }

        Ok(())
    }

    pub fn place_flags(&mut self, x: u8, y: u8) -> Result<(), String> {
        let cell = self.grid.get_cell(x.into(), y.into())?;
        let adj = self
            .grid
            .adj_cells(x, y, Some(HashSet::from([Cell::Unknown, Cell::Flag])))?;

        if let Cell::Number(num) = cell {
            if adj.len() == num as usize {
                for (x, y) in adj {
                    if self.grid.get_cell(x.into(), y.into())? != Cell::Flag {
                        self.place_flag(x, y)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn place_all_flags(&mut self) -> Result<(), String> {
        for cell in self.grid.get_iter() {
            let (x, y) = cell.pos;
            self.place_flags(x, y)?;
        }

        Ok(())
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "solved:\n{}\nflag adj:\n{}\ncurrent:\n{}",
                self.solved_grid, self.flag_adj_grid, self.grid
            )
            .as_str(),
        )
    }
}
