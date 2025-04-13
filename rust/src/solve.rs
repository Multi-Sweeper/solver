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

        board.valid_flags()
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
        time_test!();

        let pre = get_pre_str();
        let post = get_post_str();
        let solved = get_solved_str();

        let mut pre_board = GameBoard::from_str(solved, pre).unwrap();
        let post_board = GameBoard::from_str(solved, post).unwrap();

        pre_board.permute_solve_step();

        let diff = pre_board.board.diff(&post_board.board).unwrap();
        let mut diff_map = HashMap::new();
        for pos in diff {
            diff_map.insert(pos, (100, 100, 0));
        }

        println!(
            "\nsolved:\n{}left:\n{}\nright:\n{}",
            pre_board.solved_board,
            pre_board.board,
            post_board.board.to_string(Some(diff_map))
        );

        if pre_board.board != post_board.board {
            assert!(false);
        }
    }
}
