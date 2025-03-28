from Board import GameBoard
from solvers.basic import basic_solve_step
from solvers.permute import permute_solve_step
import time
import math

# game_board = GameBoard(9, 9, 10) # easy board
game_board = GameBoard(16, 16, 40) # intermediate board
# game_board = GameBoard(30, 16, 99) # advanced board

print("= init =======================================")
# click 0 tile location
init = False
for y in range(game_board._board._height):
    for x in range(game_board._board._width):
        if init:
            continue

        if game_board._solved_board.get_elem(x, y) == 0:
            init = True
            game_board.flood_fill(x, y)

if not init:
    raise Exception("no zero tile")

print(game_board)


i = 0
solved = False
start_solve_time = time.time()
while True:
    print(f"= {i+1} =======================================")
    start_time_ms = math.floor(time.time() * 1000)
    progress = basic_solve_step(game_board)
    if not progress:
        progress = permute_solve_step(game_board)

    end_time_ms = math.floor(time.time() * 1000)
    time_delta_ms = end_time_ms - start_time_ms
    print(game_board)

    print("progress:", progress, "|", "time taken:", f"{time_delta_ms}ms\n")
    
    if (game_board.is_solved()):
        solved = True
        break

    if not progress:
        break
    
    i+=1

if solved:
    print("Solved in:", f"{round(time.time() - start_solve_time, 2)}s")
else:
    print("Could not solve")