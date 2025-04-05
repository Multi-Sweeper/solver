use std::{fmt::Display, hash::Hash, vec};

use crate::{Cell, colour::Coloured};
use std::collections::HashSet;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Grid<T: Clone + Coloured + Hash + PartialEq + Eq> {
    cells: Vec<Vec<T>>,
    pub width: u8,
    pub height: u8,
}

impl<T: Clone + Coloured + Hash + PartialEq + Eq> Grid<T> {
    pub fn new(width: u8, height: u8, fill_cell: T) -> Self {
        Grid {
            cells: vec![vec![fill_cell; width as usize]; height as usize],
            width,
            height,
        }
    }

    pub fn from(cells: Vec<Vec<T>>, width: u8, height: u8) -> Result<Self, &'static str> {
        if cells.len() != height as usize {
            return Err("cells.len != height");
        }
        for row in &cells {
            if row.len() != width as usize {
                return Err("row != height");
            }
        }

        Ok(Grid {
            cells,
            width,
            height,
        })
    }

    pub fn get_cell(&self, x: i16, y: i16) -> Option<T> {
        if x < 0 || x >= self.width.into() {
            return None;
        } else if y < 0 || y >= self.height.into() {
            return None;
        }

        self.cells
            .get(((self.height as i16) - y - 1) as usize)?
            .get(x as usize)
            .cloned()
    }

    pub fn set_cell(&mut self, x: i16, y: i16, cell_value: T) -> Result<(), &'static str> {
        if x < 0 || x >= self.width.into() {
            return Err("x is out of bounds");
        } else if y < 0 || y >= self.height.into() {
            return Err("y is out of bounds");
        }

        self.cells[(self.height - (y as u8) - 1) as usize][x as usize] = cell_value;

        Ok(())
    }

    pub fn adj_cells(&self, x: u8, y: u8, filter_cells: HashSet<T>) -> Vec<(i8, i8)> {
        let mut out = Vec::new();
        let deltas: [(i8, i8); 8] = [
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
            let elem = self.get_cell(x as i16 + d.0 as i16, y as i16 + d.1 as i16);

            if let Some(e) = elem {
                if filter_cells.contains(&e) {
                    out.push(d);
                }
            }
        }

        out
    }
}

impl Grid<Cell> {
    pub fn adj_bombs(&self, x: u8, y: u8) -> Vec<(i8, i8)> {
        let hash_set = HashSet::from([Cell::Bomb]);
        self.adj_cells(x, y, hash_set)
    }

    pub fn adj_flags(&self, x: u8, y: u8) -> Vec<(i8, i8)> {
        let hash_set = HashSet::from([Cell::Flag]);
        self.adj_cells(x, y, hash_set)
    }

    pub fn adj_number(&self, x: u8, y: u8) -> Vec<(i8, i8)> {
        let hash_set = HashSet::from([
            Cell::Number(1),
            Cell::Number(2),
            Cell::Number(3),
            Cell::Number(4),
            Cell::Number(5),
            Cell::Number(6),
            Cell::Number(7),
            Cell::Number(8),
        ]);
        self.adj_cells(x, y, hash_set)
    }

    pub fn valid_flags(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell(x.into(), y.into()) {
                    Some(Cell::Number(num)) => {
                        if self.adj_cells(x, y, HashSet::from([Cell::Flag])).len() != num.into() {
                            return false;
                        }
                    }
                    _ => continue,
                }
            }
        }
        true
    }
}

impl<T: Clone + Coloured + Hash + PartialEq + Eq> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let elem = self.cells[row as usize][col as usize].clone();

                out.push_str(format!(" {} ", elem.to_coloured()).as_str());
            }

            out.push_str(format!(" | {}\n", self.height - row - 1).as_str());
        }

        out.push_str(" -");

        for _ in 0..self.width {
            out.push_str("---");
        }

        out.push('\n');

        let max_digits = self.width.to_string().len();
        for digit in 0..max_digits {
            for col in 0..self.width {
                out.push(' ');
                out.push_str(
                    format!("{:0width$}", col, width = max_digits)
                        .chars()
                        .nth(digit)
                        .unwrap()
                        .to_string()
                        .as_str(),
                );
                out.push(' ');
            }
            out.push('\n');
        }

        f.write_str(out.as_str())
    }
}
