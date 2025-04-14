use std::time::Instant;

use crate::board::GameBoard;

pub fn strategy_simple(
    board: GameBoard,
    starting_cell: (u8, u8),
) -> Result<Option<Vec<Vec<&'static str>>>, String> {
    let mut step_summary: Vec<Vec<&str>> = Vec::new();

    let mut game_board = board.clone();
    game_board.flood_fill(starting_cell.0.into(), starting_cell.1.into())?;

    println!("= init =======================================");
    println!("{}", game_board);

    step_summary.clear();
    let mut i = 0;
    let mut solved = false;
    let mut progress = true;

    while !solved && progress {
        println!("= {} =======================================", i + 1);
        let start_time = Instant::now();
        step_summary.push(vec!["basic"]);
        progress = game_board.simple_solve_step()?;
        if !progress {
            step_summary.last_mut().unwrap().push("permute");
            progress = game_board.permute_solve_step()?;
        }

        println!("{}", game_board);
        println!(
            "progress: {} |  time taken: {}ms",
            progress,
            start_time.elapsed().as_millis()
        );

        solved = game_board.is_solved()?;
        i += 1;

        // std::io::stdin().read_line(&mut String::new());
    }

    // println!("next starting cell...");
    // std::io::stdin().read_line(&mut String::new());
    if solved {
        Ok(Some(step_summary))
    } else {
        Ok(None)
    }
}
