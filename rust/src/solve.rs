use crate::{Cell, board::GameBoard};

impl GameBoard {
    pub fn simple_solve_step(&mut self) -> Result<bool, String> {
        let pre_board = self.grid.clone();

        self.place_all_flags()?;
        self.chord_all()?;

        Ok(pre_board != self.grid)
    }

    fn is_valid_bomb_pattern(
        &self,
        potential_bombs: &Vec<(u8, u8)>,
        pattern: u128,
    ) -> Result<bool, String> {
        let mut board = self.grid.clone();

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

        board.valid_flags()
    }

    pub fn permute_solve_step(&mut self) -> Result<bool, String> {
        let pre_board = self.grid.clone();

        let mut potential_bombs: Vec<(u8, u8)> = Vec::new();
        for cell in self.grid.get_iter() {
            let (x, y) = cell.pos;
            if let Some(cell) = self.grid.get_cell(x.into(), y.into()) {
                if cell == Cell::Unknown {
                    if self.grid.adj_number(x.into(), y.into())?.len() > 0 {
                        potential_bombs.push((x, y));
                    }
                }
            }
        }

        if potential_bombs.len() > 127 {
            return Err("too many potential bombs".to_string());
        }

        println!("potential_bombs: {:?}", potential_bombs);

        let mut valid_patterns: Vec<u128> = Vec::new();
        let end_pattern: u128 = 1u128 << potential_bombs.len();

        println!("end_pattern: {:b} ({})", end_pattern, potential_bombs.len());

        // if more than 16 potential bomb locations, do not even attempt
        if potential_bombs.len() > 16 {
            return Err("too complex".to_string());
        }

        for pattern in 1..end_pattern {
            if self.is_valid_bomb_pattern(&potential_bombs, pattern)? {
                valid_patterns.push(pattern);
            }
        }

        if valid_patterns.len() == 0 {
            println!("no valid patterns");
            return Ok(false);
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
                self.grid
                    .set_cell(cell.0.into(), cell.1.into(), Cell::Flag)
                    .unwrap();
            }

            if ((safe_pattern >> i) & 0b1) == 0 {
                self.flood_fill(cell.0.into(), cell.1.into())?;
            }
        }

        // logical AND all valid patterns, if any digit is 1, it is guaranteed to be a bomb
        // logical OR all valid patterns, if any digit is 0, it is guaranteed to be a safe

        Ok(pre_board != self.grid)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn get_solved_str() -> &'static str {
        "B  1  1  B  3  B  3  2  4  B  B  1  1  B  2  1 
        2  2  1  1  3  B  3  B  B  B  3  2  2  3  B  1 
        B  1  0  0  1  1  2  3  B  3  1  1  B  2  2  2 
        1  1  0  1  1  1  0  2  2  3  1  2  1  1  1  B 
        0  0  0  1  B  3  2  2  B  2  B  1  0  0  1  1 
        0  0  0  1  2  B  B  2  1  2  1  1  1  1  1  0 
        1  2  1  1  1  2  2  1  0  1  1  1  1  B  2  1 
        B  2  B  2  2  2  1  0  0  1  B  1  1  1  2  B 
        1  2  1  2  B  B  1  0  0  1  1  1  0  0  1  1 
        0  0  0  1  2  2  1  0  0  0  1  1  1  0  0  0 
        0  0  0  0  0  0  0  1  1  1  1  B  2  1  1  0 
        1  1  1  0  1  1  2  2  B  2  2  1  3  B  2  0 
        1  B  1  1  2  B  3  B  3  B  1  0  2  B  2  0 
        1  1  1  1  B  4  B  2  2  1  1  1  2  2  1  0 
        0  0  0  1  2  B  2  1  0  0  0  1  B  1  1  1 
        0  0  0  0  1  1  1  0  0  0  0  1  1  1  1  B"
    }

    fn get_pre_str() -> &'static str {
        "?  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  ? 
        2  2  1  1  3  F  3  F  F  F  3  2  2  ?  ?  ? 
        F  1  0  0  1  1  2  3  F  3  1  1  F  2  2  2 
        1  1  0  1  1  1  0  2  2  3  1  2  1  1  1  F 
        0  0  0  1  F  3  2  2  F  2  F  1  0  0  1  1 
        0  0  0  1  2  F  F  2  1  2  1  1  1  1  1  0 
        1  2  1  1  1  2  2  1  0  1  1  1  1  F  2  1 
        F  2  F  2  2  2  1  0  0  1  F  1  1  1  2  F 
        1  2  1  2  F  F  1  0  0  1  1  1  0  0  1  1 
        0  0  0  1  2  2  1  0  0  0  1  1  1  0  0  0 
        0  0  0  0  0  0  0  1  1  1  1  F  2  1  1  0 
        1  1  1  0  1  1  2  2  F  2  2  1  3  F  2  0 
        1  F  1  1  2  F  3  F  3  F  1  0  2  F  2  0 
        1  1  1  1  F  4  F  2  2  1  1  1  2  2  1  0 
        0  0  0  1  2  F  2  1  0  0  0  1  F  1  1  1 
        0  0  0  0  1  1  1  0  0  0  0  1  1  1  1  F"
    }

    fn get_post_str() -> &'static str {
        "?  ?  1  ?  ?  F  3  2  ?  F  ?  ?  1  ?  ?  ?
        2  2  1  1  3  F  3  F  F  F  3  2  2  3  F  1
        F  1  0  0  1  1  2  3  F  3  1  1  F  2  2  2
        1  1  0  1  1  1  0  2  2  3  1  2  1  1  1  F
        0  0  0  1  F  3  2  2  F  2  F  1  0  0  1  1
        0  0  0  1  2  F  F  2  1  2  1  1  1  1  1  0
        1  2  1  1  1  2  2  1  0  1  1  1  1  F  2  1
        F  2  F  2  2  2  1  0  0  1  F  1  1  1  2  F
        1  2  1  2  F  F  1  0  0  1  1  1  0  0  1  1
        0  0  0  1  2  2  1  0  0  0  1  1  1  0  0  0
        0  0  0  0  0  0  0  1  1  1  1  F  2  1  1  0
        1  1  1  0  1  1  2  2  F  2  2  1  3  F  2  0
        1  F  1  1  2  F  3  F  3  F  1  0  2  F  2  0
        1  1  1  1  F  4  F  2  2  1  1  1  2  2  1  0
        0  0  0  1  2  F  2  1  0  0  0  1  F  1  1  1
        0  0  0  0  1  1  1  0  0  0  0  1  1  1  1  F"
    }

    #[test]
    #[ignore]
    fn permute_1() {
        // this takes ~70s for me in debug build
        // this takes ~7s for me in release build ?!?
        //
        // potential_bombs: [(13, 14), (14, 14), (15, 14), (0, 15), (1, 15), (2, 15), (3, 15), (4, 15), (5, 15), (6, 15), (7, 15), (9, 15), (10, 15), (11, 15), (12, 15), (13, 15)]
        // end_pattern: 10000000000000000 (16)
        // flag_pattern: 0000100100000010
        // safe_pattern: 1011100111011010
        time_test!();

        let pre = get_pre_str();
        let post = get_post_str();
        let solved = get_solved_str();

        let mut pre_board = GameBoard::from_str(solved, pre).unwrap();
        let post_board = GameBoard::from_str(solved, post).unwrap();

        pre_board.permute_solve_step().unwrap();

        let diff = pre_board.grid.diff(&post_board.grid).unwrap();
        let mut diff_map = HashMap::new();
        for pos in diff {
            diff_map.insert(pos, (100, 100, 0));
        }

        println!(
            "\nsolved:\n{}\nleft:\n{}\nright:\n{}",
            pre_board.solved_grid,
            pre_board.grid,
            post_board.grid.to_string(Some(diff_map))
        );

        if pre_board.grid != post_board.grid {
            assert!(false, "pre_board.grid != post_board.grid");
        }
    }
}
