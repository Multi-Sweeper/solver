use std::fmt::Display;

use crate::colour::Coloured;

#[derive(Debug)]
pub struct Grid<T: Clone + Coloured> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Clone + Coloured> Grid<T> {
    pub fn new(width: usize, height: usize, fill_cell: T) -> Self {
        Grid {
            cells: vec![vec![fill_cell; width]; height],
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<T> {
        let row = self.cells.get(self.height - y - 1)?;
        row.get(x).cloned()
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell_value: T) {
        self.cells[self.height - y - 1][x] = cell_value
    }
}

impl<T: Clone + Coloured> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let elem = self.cells[row][col].clone();

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
                    format!("{:0^width$}", col.to_string(), width = max_digits)
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
