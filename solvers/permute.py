from Board import GameBoard
from copy import deepcopy

def permute_solve_step(game_board: GameBoard) -> bool:
    pre_board = deepcopy(game_board._board)

    potential_bombs = []
    for cell in game_board._board.cell_position_iter():
        # elem = 
        if game_board._board.get_elem(cell[0], cell[1]) == "?":
            if len(game_board._board.adj_number(cell[0], cell[1])) > 0:
                potential_bombs.append(cell)

    # print(potential_bombs)

    for cell in potential_bombs:
        game_board._board.repr_override[cell] = "*"

    post_board = deepcopy(game_board._board)
    return pre_board != post_board