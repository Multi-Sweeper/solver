from Board import GameBoard
from solvers.basic import basic_solve_step

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
while True:
    print(f"= {i+1} =======================================")
    progress = basic_solve_step(game_board)
    print(game_board)
    print("progress:", progress)
    
    if (game_board.is_solved()):
        print("solved")
        break

    if not progress:
        break
    
    input()
    i+=1

