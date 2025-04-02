use std::{fmt::Display, hash::Hash};

use crate::colour::Coloured;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Grid<T: Clone + Coloured + Hash + Eq + PartialEq> {
    cells: Vec<Vec<T>>,
    width: u8,
    height: u8,
}

impl<T: Clone + Coloured + Hash + Eq + PartialEq> Grid<T> {
    pub fn new(width: u8, height: u8, fill_cell: T) -> Self {
        Grid {
            cells: vec![vec![fill_cell; width as usize]; height as usize],
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: i16, y: i16) -> Option<T> {
        if x < 0 || x >= self.width as i16 {
            return None;
        } else if y < 0 || y >= self.height as i16 {
            return None;
        }

        self.cells
            .get(((self.height as i16) - y - 1) as usize)?
            .get(x as usize)
            .cloned()
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell_value: T) {
        self.cells[(self.height - y - 1) as usize][x as usize] = cell_value
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

impl<T: Clone + Coloured + Hash + Eq + PartialEq> Display for Grid<T> {
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
