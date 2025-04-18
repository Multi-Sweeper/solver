use std::{collections::HashMap, fmt::Display, hash::Hash, vec};

use crate::{Cell, colour::Coloured, utils::unflatten};
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

    pub fn from(cells: Vec<Vec<T>>, width: u8, height: u8) -> Result<Self, String> {
        if cells.len() != height as usize {
            return Err("cells.len != height".to_string());
        }
        for row in &cells {
            if row.len() != width as usize {
                return Err("row != height".to_string());
            }
        }

        Ok(Grid {
            cells,
            width,
            height,
        })
    }

    fn assert_bounds(&self, x: i16, y: i16) -> Result<(), String> {
        if x < 0 || x >= self.width.into() {
            Err("x is out of bounds".to_string())
        } else if y < 0 || y >= self.height.into() {
            Err("y is out of bounds".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_cell(&self, x: i16, y: i16) -> Option<T> {
        if self.assert_bounds(x, y).is_err() {
            return None;
        }

        let cell = self
            .cells
            .get(((self.height as i16) - y - 1) as usize)
            .unwrap()
            .get(x as usize)
            .cloned()
            .unwrap();

        Some(cell)
    }

    pub fn set_cell(&mut self, x: i16, y: i16, cell_value: T) -> Result<(), String> {
        self.assert_bounds(x, y)?;

        self.cells[(self.height - (y as u8) - 1) as usize][x as usize] = cell_value;

        Ok(())
    }

    pub fn get_iter(&self) -> CellsIter<T> {
        CellsIter {
            curr_pos: (0, 0),
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
        }
    }

    pub fn adj_cells(
        &self,
        x: u8,
        y: u8,
        filter_cells: Option<HashSet<T>>,
    ) -> Result<Vec<(u8, u8)>, String> {
        let mut out: Vec<(u8, u8)> = Vec::new();
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
            if let Some(elem) = self.get_cell(x as i16 + d.0 as i16, y as i16 + d.1 as i16) {
                if let Some(ref filter) = filter_cells {
                    if filter.contains(&elem) {
                        out.push(((x as i16 + d.0 as i16) as u8, (y as i16 + d.1 as i16) as u8));
                    }
                } else {
                    out.push(((x as i16 + d.0 as i16) as u8, (y as i16 + d.1 as i16) as u8));
                }
            }
        }

        Ok(out)
    }

    pub fn diff(&self, other: &Self) -> Result<Vec<(u8, u8)>, String> {
        let mut out = Vec::new();

        if self.width != other.width {
            return Err("self.width != other.width".to_string());
        } else if self.height != other.height {
            return Err("height != other.height".to_string());
        }

        for cell in self.get_iter() {
            let (x, y) = cell.pos;
            if let Some(other_cell) = other.get_cell(x.into(), y.into()) {
                if cell.val != other_cell {
                    out.push(cell.pos);
                }
            }
        }

        Ok(out)
    }

    pub fn to_string(&self, highlights: Option<HashMap<(u8, u8), (u8, u8, u8)>>) -> String {
        let mut out = String::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let elem = self.cells[row as usize][col as usize].clone();

                let mut bg: Option<(u8, u8, u8)> = None;
                if let Some(ref map) = highlights {
                    bg = map.get(&(col, self.height - row - 1)).copied();
                }

                out.push_str(format!(" {} ", elem.to_coloured(bg)).as_str());
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

        out
    }
}

impl Grid<Cell> {
    pub fn from_str(input_str: &str) -> Result<Self, String> {
        let height = input_str.split("\n").count();
        let width = input_str.split("\n").collect::<Vec<_>>()[0]
            .trim()
            .split(" ")
            .filter(|c| !c.is_empty())
            .count();

        if height > 255 {
            return Err("height > 255".to_string());
        } else if width > 255 {
            return Err("width > 255".to_string());
        }

        let height = height as u8;
        let width = width as u8;

        let mut flat_cells: Vec<Cell> = Vec::new();

        for row in input_str.split("\n") {
            for cell in row.trim().split(" ").filter(|c| !c.is_empty()) {
                flat_cells.push(match cell {
                    "0" => Cell::Number(0),
                    "1" => Cell::Number(1),
                    "2" => Cell::Number(2),
                    "3" => Cell::Number(3),
                    "4" => Cell::Number(4),
                    "5" => Cell::Number(5),
                    "6" => Cell::Number(6),
                    "7" => Cell::Number(7),
                    "8" => Cell::Number(8),
                    "B" => Cell::Bomb,
                    "F" => Cell::Flag,
                    "?" => Cell::Unknown,
                    "*" => Cell::Asterix,
                    _ => return Err(format!("unknown character: `{}`", cell)),
                });
            }
        }

        let cells = unflatten(flat_cells, width, height)?;
        Ok(Grid::from(cells, width, height)?)
    }

    pub fn adj_bombs(&self, x: u8, y: u8) -> Result<Vec<(u8, u8)>, String> {
        let hash_set = HashSet::from([Cell::Bomb]);
        self.adj_cells(x, y, Some(hash_set))
    }

    pub fn adj_flags(&self, x: u8, y: u8) -> Result<Vec<(u8, u8)>, String> {
        let hash_set = HashSet::from([Cell::Flag]);
        self.adj_cells(x, y, Some(hash_set))
    }

    pub fn adj_number(&self, x: u8, y: u8) -> Result<Vec<(u8, u8)>, String> {
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
        self.adj_cells(x, y, Some(hash_set))
    }
}

impl Grid<Option<u8>> {
    pub fn decr(&mut self, x: u8, y: u8) -> Result<(), String> {
        if let Some(cell) = self.get_cell(x.into(), y.into()) {
            if let Some(num) = cell {
                if num <= 0 {
                    return Err(format!(
                        "num <= 0 | attempted to dec value at ({}, {}) = {}",
                        x, y, num
                    ));
                }

                self.set_cell(x.into(), y.into(), Some(num - 1))?;
            }
        }

        Ok(())
    }
}

impl<T: Clone + Coloured + Hash + PartialEq + Eq> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string(None).as_str())
    }
}

#[derive(Debug, PartialEq)]
pub struct CellsIterValue<T: Clone + Coloured + Hash + PartialEq + Eq> {
    pub pos: (u8, u8),
    pub val: T,
}

#[derive(Debug)]
pub struct CellsIter<T: Clone + Coloured + Hash + PartialEq + Eq> {
    pub curr_pos: (u8, u8),
    pub width: u8,
    pub height: u8,
    pub cells: Vec<Vec<T>>,
}

impl<T: Clone + Coloured + Hash + PartialEq + Eq> Iterator for CellsIter<T> {
    type Item = CellsIterValue<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_pos.0 >= self.width {
            self.curr_pos = (0, self.curr_pos.1 + 1)
        }

        if self.curr_pos.1 >= self.height {
            return None;
        }

        let out = CellsIterValue {
            pos: self.curr_pos,
            val: self.cells[(self.height - self.curr_pos.1 - 1) as usize][self.curr_pos.0 as usize]
                .clone(),
        };
        self.curr_pos.0 += 1;

        Some(out)
    }
}

#[cfg(test)]
mod tests {

    use std::hash::RandomState;

    use super::*;

    fn generate_cells() -> Vec<Vec<Cell>> {
        let cells = vec![
            vec![Cell::Unknown, Cell::Unknown, Cell::Flag],
            vec![Cell::Unknown, Cell::Bomb, Cell::Unknown],
            vec![Cell::Number(0), Cell::Unknown, Cell::Number(5)],
        ];

        cells
    }

    fn generate_grid() -> Grid<Cell> {
        Grid::from(generate_cells(), 3, 3).unwrap()
    }

    #[test]
    fn new() {
        let grid: Grid<Cell> = Grid::new(10, 10, Cell::Unknown);

        assert_eq!(grid.cells.len(), grid.height.into());

        for y in 0..grid.height {
            assert_eq!(grid.cells[y as usize].len(), grid.width.into());

            for x in 0..grid.width {
                assert_eq!(grid.cells[y as usize][x as usize], Cell::Unknown);
            }
        }
    }

    #[test]
    fn cells_iter_1() {
        let grid = generate_grid();

        let out: Vec<_> = grid.get_iter().collect();

        assert_eq!(
            out,
            vec![
                CellsIterValue {
                    pos: (0, 0),
                    val: Cell::Number(0)
                },
                CellsIterValue {
                    pos: (1, 0),
                    val: Cell::Unknown
                },
                CellsIterValue {
                    pos: (2, 0),
                    val: Cell::Number(5)
                },
                CellsIterValue {
                    pos: (0, 1),
                    val: Cell::Unknown
                },
                CellsIterValue {
                    pos: (1, 1),
                    val: Cell::Bomb
                },
                CellsIterValue {
                    pos: (2, 1),
                    val: Cell::Unknown
                },
                CellsIterValue {
                    pos: (0, 2),
                    val: Cell::Unknown
                },
                CellsIterValue {
                    pos: (1, 2),
                    val: Cell::Unknown
                },
                CellsIterValue {
                    pos: (2, 2),
                    val: Cell::Flag
                },
            ]
        );
    }

    #[test]
    fn from_1() {
        let cells = generate_cells();
        let mut grid = Grid::new(3, 3, Cell::Unknown);
        grid.set_cell(0, 0, Cell::Number(0)).unwrap();
        grid.set_cell(2, 0, Cell::Number(5)).unwrap();
        grid.set_cell(1, 1, Cell::Bomb).unwrap();
        grid.set_cell(2, 2, Cell::Flag).unwrap();

        let from_grid = Grid::from(cells, grid.width, grid.height);
        assert!(from_grid.is_ok());

        let from_grid = from_grid.unwrap();

        println!("grid:\n{}\nfrom_grid:\n{}", grid, from_grid);
        assert_eq!(grid, from_grid);
    }

    #[test]
    fn from_2() {
        let cells = generate_cells();
        let grid = Grid::new(3, 3, Cell::Unknown);

        let from_grid = Grid::from(cells, grid.width, grid.height);
        assert!(from_grid.is_ok());

        let from_grid = from_grid.unwrap();

        println!("grid:\n{}\nfrom_grid:\n{}", grid, from_grid);
        assert_ne!(grid, from_grid);
    }

    #[test]
    fn from_3() {
        let cells = generate_cells();

        let from_grid = Grid::from(cells, 2, 2);
        assert!(from_grid.is_err());
    }

    #[test]
    fn from_str_1() {
        let cells = generate_cells();
        let grid = Grid::from(cells, 3, 3).unwrap();

        let grid_from_str = Grid::from_str(
            "?  ?  F
?  B  ?
0  ?  5",
        )
        .unwrap();

        println!("{} {}", grid, grid_from_str);

        assert_eq!(grid, grid_from_str)
    }

    #[test]
    fn get_cell_1() {
        let grid = generate_grid();

        let cell = grid.get_cell(0, 0);
        assert!(cell.is_some());

        let cell = cell.unwrap();
        println!("{}", grid);
        assert_eq!(cell, Cell::Number(0))
    }

    #[test]
    fn get_cell_2() {
        let grid = generate_grid();

        let cell = grid.get_cell(2, 2);
        assert!(cell.is_some());

        let cell = cell.unwrap();
        println!("{}", grid);
        assert_eq!(cell, Cell::Flag)
    }

    #[test]
    fn get_cell_3() {
        let grid = generate_grid();

        let cell = grid.get_cell(4, 2);
        println!("{}", grid);
        assert!(cell.is_none());
    }

    #[test]
    fn set_cell_1() {
        let mut grid = generate_grid();

        println!("before:\n{}", grid);
        let res = grid.set_cell(0, 0, Cell::Bomb);
        assert!(res.is_ok());

        println!("after:\n{}", grid);
        assert_eq!(grid.get_cell(0, 0).unwrap(), Cell::Bomb);
    }

    #[test]
    fn set_cell_2() {
        let mut grid = generate_grid();

        let res = grid.set_cell(4, 0, Cell::Bomb);
        assert!(res.is_err());
    }

    #[test]
    fn adj_cells_1() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_cells(1, 1, None).unwrap();
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter(
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
            ]
            .iter(),
        );

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_2() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid
            .adj_cells(1, 1, Some(HashSet::from([Cell::Unknown])))
            .unwrap();
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> =
            HashSet::from_iter([(1, 0), (0, 1), (2, 1), (0, 2), (1, 2)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_3() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_number(1, 1).unwrap();
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([(2, 0)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_4() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_flags(1, 1).unwrap();
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([(2, 2)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_5() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_flags(0, 1).unwrap();
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_6() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_bombs(0, 2).unwrap();
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([(1, 1)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }
}
