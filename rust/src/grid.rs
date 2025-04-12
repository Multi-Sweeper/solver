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

    pub fn get_iter(&self) -> CellsIter<T> {
        CellsIter {
            curr_pos: (0, 0),
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
        }
    }

    pub fn adj_cells(&self, x: u8, y: u8, filter_cells: Option<HashSet<T>>) -> Vec<(u8, u8)> {
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
            let elem = self.get_cell(x as i16 + d.0 as i16, y as i16 + d.1 as i16);

            if let Some(e) = elem {
                if let Some(ref filter) = filter_cells {
                    if filter.contains(&e) {
                        out.push(((x as i16 + d.0 as i16) as u8, (y as i16 + d.1 as i16) as u8));
                    }
                } else {
                    out.push(((x as i16 + d.0 as i16) as u8, (y as i16 + d.1 as i16) as u8));
                }
            }
        }

        out
    }
}

impl Grid<Cell> {
    pub fn adj_bombs(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        let hash_set = HashSet::from([Cell::Bomb]);
        self.adj_cells(x, y, Some(hash_set))
    }

    pub fn adj_flags(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        let hash_set = HashSet::from([Cell::Flag]);
        self.adj_cells(x, y, Some(hash_set))
    }

    pub fn adj_number(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
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

    pub fn valid_flags(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_cell(x.into(), y.into()) {
                    Some(Cell::Number(num)) => {
                        if self
                            .adj_cells(x, y, Some(HashSet::from([Cell::Flag])))
                            .len()
                            != num.into()
                        {
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
        let adj_cells = grid.adj_cells(1, 1, None);
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
        let adj_cells = grid.adj_cells(1, 1, Some(HashSet::from([Cell::Unknown])));
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> =
            HashSet::from_iter([(1, 0), (0, 1), (2, 1), (0, 2), (1, 2)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_3() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_number(1, 1);
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([(2, 0)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_4() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_flags(1, 1);
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([(2, 2)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_5() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_flags(0, 1);
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }

    #[test]
    fn adj_cells_6() {
        let grid = generate_grid();
        println!("{}", grid);
        let adj_cells = grid.adj_bombs(0, 2);
        let adj_cells: HashSet<&(u8, u8), RandomState> = HashSet::from_iter(adj_cells.iter());

        let expected_adj_cells: HashSet<&(u8, u8)> = HashSet::from_iter([(1, 1)].iter());

        assert_eq!(adj_cells, expected_adj_cells);
    }
}
