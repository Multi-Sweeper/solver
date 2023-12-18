from Board import GameBoard

game_board = GameBoard(9, 9, 10) # easy board
# game_board = GameBoard(16, 16, 40) # intermediate board
# game_board = GameBoard(30, 16, 99) # advanced board

print("= init =======================================")
# hard code inital click location
game_board.flood_fill(4,4)
print (game_board)
# hard code 10 iteration steps
for i in range(10):
    print(f"= {i+1}f =======================================")
    game_board.place_all_flags()
    print (game_board)
    print(f"= {i+1}c =======================================")
    game_board.chord_all()
    print (game_board)