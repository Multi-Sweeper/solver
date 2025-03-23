from Board import GameBoard
from copy import deepcopy

def basic_solve_step(game_board: GameBoard) -> bool:
    pre_board = deepcopy(game_board._board)

    game_board.place_all_flags()
    game_board.chord_all()

    post_board = deepcopy(game_board._board)

    return pre_board != post_board