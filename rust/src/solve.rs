use crate::{Cell, board::GameBoard};

impl GameBoard {
    pub fn simple_solve_step(&mut self) -> bool {
        let pre_board = self.board.clone();

        self.place_all_flags();
        self.chord_all();

        pre_board != self.board
    }

    fn is_valid_bomb_pattern(&self, potential_bombs: &Vec<(u8, u8)>, pattern: u128) -> bool {
        let mut board = self.board.clone();

        let mut current_pattern = pattern;
        let mut i = 0;

        while current_pattern > 0 {
            let is_bomb = (current_pattern & 0b1) == 1;

            if is_bomb {
                let cell = potential_bombs[i];
                board
                    .set_cell(cell.0.into(), cell.1.into(), Cell::Flag)
                    .unwrap();
            }

            current_pattern >>= 1;
            i += 1;
        }

        // println!("{}", board);
        // for cell in potential_bombs {
        //     let adj_d = self.board.adj_number(cell.0.into(), cell.1.into());
        //     for d in adj_d {
        //         let x = cell.0 as i16 + d.0 as i16;
        //         let y = cell.1 as i16 + d.1 as i16;
        //         match self.board.get_cell(x, y) {
        //             Some(Cell::Number(num)) => {
        //                 let adj_flags =
        //                     self.board
        //                         .adj_cells(x as u8, y as u8, HashSet::from([Cell::Flag]));

        //                 if adj_flags.len() != num.into() {
        //                     println!("{} {:?}", num, adj_flags);

        //                     println!("{} {:b}", false, pattern);
        //                     // std::io::stdin().read_line(&mut String::new());
        //                     return false;
        //                 }
        //             }
        //             _ => continue,
        //         }
        //     }
        // }

        // println!("{} {:b}", board.valid_flags(), pattern);
        board.valid_flags()
        // println!("{} {:b}", true, pattern);
        // std::io::stdin().read_line(&mut String::new());
        // true
    }

    pub fn permute_solve_step(&mut self) -> bool {
        let pre_board = self.board.clone();

        let mut potential_bombs: Vec<(u8, u8)> = Vec::new();
        for cell in self.board.get_iter() {
            let (x, y) = cell.pos;
            if let Some(Cell::Unknown) = self.board.get_cell(x.into(), y.into()) {
                if self.board.adj_number(x.into(), y.into()).len() > 0 {
                    potential_bombs.push((x, y));
                }
            }
        }

        if potential_bombs.len() > 127 {
            println!("too many potential bombs");
            return false;
        }

        println!("potential_bombs: {:?}", potential_bombs);

        let mut valid_patterns: Vec<u128> = Vec::new();
        let end_pattern: u128 = 1u128 << potential_bombs.len();

        println!("end_pattern: {:b} ({})", end_pattern, potential_bombs.len());

        // if more than 16 potential bomb locations, do not even attempt
        if potential_bombs.len() > 16 {
            println!("too complex");
            return false;
        }

        for pattern in 1..end_pattern {
            if self.is_valid_bomb_pattern(&potential_bombs, pattern) {
                valid_patterns.push(pattern);
            }
        }

        if valid_patterns.len() == 0 {
            println!("no valid patterns");
            return false;
        }

        let mut flag_pattern = end_pattern - 1;
        let mut safe_pattern = 0u128;

        for pattern in valid_patterns {
            flag_pattern &= pattern;
            safe_pattern |= pattern;
        }

        println!(
            "flag_pattern: {:0width$b}\nsafe_pattern: {:0width$b}",
            flag_pattern,
            safe_pattern,
            width = potential_bombs.len()
        );

        for i in 0..potential_bombs.len() {
            let cell = potential_bombs[i];

            if ((flag_pattern >> i) & 0b1) == 1 {
                self.board
                    .set_cell(cell.0.into(), cell.1.into(), Cell::Flag)
                    .unwrap();
            }

            if ((safe_pattern >> i) & 0b1) == 0 {
                self.flood_fill(cell.0.into(), cell.1.into());
            }
        }

        // logical AND all valid patterns, if any digit is 1, it is guaranteed to be a bomb
        // logical OR all valid patterns, if any digit is 0, it is guaranteed to be a safe

        pre_board != self.board
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use crate::grid::Grid;

    use super::*;

    fn get_permute_1_grid(solved: bool) -> Grid<Cell> {
        let cells: Vec<Vec<Cell>>;

        if !solved {
            cells = vec![
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(2),
                    Cell::Number(2),
                    Cell::Number(1),
                    Cell::Number(1),
                    Cell::Number(2),
                    Cell::Number(2),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Flag,
                    Cell::Number(1),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(1),
                    Cell::Number(1),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(2),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Number(2),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Number(1),
                    Cell::Number(2),
                    Cell::Flag,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Flag,
                    Cell::Number(2),
                    Cell::Number(2),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Number(0),
                    Cell::Number(0),
                    Cell::Number(1),
                    Cell::Number(1),
                    Cell::Number(1),
                    Cell::Number(1),
                    Cell::Unknown,
                    Cell::Unknown,
                ],
            ];
        } else {
            cells = vec![
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Bomb,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Bomb,
                    Cell::Bomb,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Bomb,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Bomb,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Bomb,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Bomb,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Bomb,
                    Cell::Unknown,
                ],
                vec![
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                    Cell::Unknown,
                ],
            ];
        }

        Grid::from(cells, 8, 11).unwrap()
    }

    #[test]
    #[ignore]
    fn permute_1() {
        let grid = get_permute_1_grid(false);
        let solved_grid = get_permute_1_grid(true);

        let mut board = GameBoard::from(solved_grid).unwrap();
        board.board = grid;

        println!("{}", board);
        let start_time = Instant::now();
        board.permute_solve_step();
        println!("{}\ntime: {}ms", board, start_time.elapsed().as_millis());

        assert!(false);
    }
}
