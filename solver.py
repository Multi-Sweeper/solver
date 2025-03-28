from Board import GameBoard
from solvers.basic import basic_solve_step
from solvers.permute import permute_solve_step
from typing import List, Tuple
from copy import deepcopy
import time
import math

# _game_board = GameBoard(9, 9, 10) # easy board
_game_board = GameBoard(16, 16, 40) # intermediate board
# _game_board = GameBoard(30, 16, 99) # advanced board

starting_cells: List[Tuple[int, int]] = []

temp_board = deepcopy(_game_board)
for (x, y) in temp_board._board.cell_position_iter():
    if (temp_board._solved_board.get_elem(x, y) == 0) and (temp_board._board.get_elem(x, y) == "?"):
        temp_board.flood_fill(x, y)
        starting_cells.append((x, y))

solved = False
start_solve_time = time.time()
step_summary: List[list] = []
for starting_cell in starting_cells:
    game_board = deepcopy(_game_board)
    game_board.flood_fill(starting_cell[0], starting_cell[1])
    print("= init =======================================")
    print(game_board)

    step_summary = []
    i = 0
    solved = False
    progress = True
    while (not solved) and progress:
        print(f"= {i+1} =======================================")
        start_time_ms = math.floor(time.time() * 1000)
        step_summary.append(["basic"])
        progress = basic_solve_step(game_board)
        if not progress:
            step_summary[-1].append("permute")
            progress = permute_solve_step(game_board)

        end_time_ms = math.floor(time.time() * 1000)
        time_delta_ms = end_time_ms - start_time_ms
        print(game_board)

        print("progress:", progress, "|", "time taken:", f"{time_delta_ms}ms\n")
        
        solved = game_board.is_solved()
        
        i+=1

    if solved:
        break

if solved:
    print("Solved in:", f"{round(time.time() - start_solve_time, 2)}s")
    print("step summary:", step_summary)
else:
    print("Could not solve")
