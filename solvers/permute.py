from Board import GameBoard
from copy import deepcopy
from typing import List, Tuple

def _is_valid_bomb_pattern(game_board: GameBoard, potential_bombs: list[tuple[int, int]], pattern: int) -> bool:

    temp_board = deepcopy(game_board._board)
    temp_board.repr_override = {}
    current_pattern = pattern
    i = 0
    while current_pattern > 0:
        is_bomb = bool(current_pattern & 0b1)

        if is_bomb:
            cell = potential_bombs[i]
            temp_board._set_elem(cell[0], cell[1], "F")

        current_pattern = current_pattern >> 1
        i += 1

    # print(temp_board)
    # print(temp_board.is_valid())
    # input()

    return temp_board.valid_flags()

def permute_solve_step(game_board: GameBoard) -> bool:
    pre_board = deepcopy(game_board._board)

    potential_bombs: List[Tuple[int, int]] = []
    for cell in game_board._board.cell_position_iter():
        if game_board._board.get_elem(cell[0], cell[1]) == "?":
            if len(game_board._board.adj_number(cell[0], cell[1])) > 0:
                potential_bombs.append(cell)

    # for cell in potential_bombs:
    #     game_board._board.repr_override[cell] = "*"

    valid_patterns: List[int] = []
    end_pattern: int = (1 << len(potential_bombs))
    # print("end_pattern:", "{0:b}".format(end_pattern), f"({len(potential_bombs)})")
    
    # if more than 17 potential bomb locations, do not even attempt
    if len(potential_bombs) > 17:
        print("too complex")
        return False

    for pattern in range(1, end_pattern):
        # TODO: this can be made parallel
        if _is_valid_bomb_pattern(game_board, potential_bombs, pattern):
            valid_patterns.append(pattern)

    if (len(valid_patterns) == 0):
        print("no valid patterns")
        return False

    flag_pattern = end_pattern-1
    safe_pattern = 0

    # print("valid_patterns:")
    for pattern in valid_patterns:
        # print("{:b}".format(pattern))
        flag_pattern = flag_pattern & pattern
        safe_pattern = safe_pattern | pattern

    print("flag_pattern:", "{:b}".format(flag_pattern))
    print("safe_pattern:", "{:b}".format(safe_pattern))

    for i in range(len(potential_bombs)):
        # print(i, "{:b}".format(flag_pattern >> i))
        cell = potential_bombs[i]

        if ((flag_pattern >> i) & (0b1)) == 1:
            game_board._board._set_elem(cell[0], cell[1], "F")

        if ((safe_pattern >> i) & (0b1)) == 0:
            game_board.flood_fill(cell[0], cell[1])

    # logical AND all valid patterns, if any digit is 1, it is guaranteed to be a bomb
    # logical OR all valid patterns, if any digit is 0, it is guaranteed to be a safe

    post_board = deepcopy(game_board._board)
    return pre_board != post_board
