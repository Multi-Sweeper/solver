from __future__ import annotations
from typing import Any, List, Dict
import random
from utils import coloured_num, unflaten_board
from copy import deepcopy

class Board:
    def __init__(self, width: int, height: int):
        self._width = width
        self._height = height
        self._board = unflaten_board([0]*width*height, width, height)
        self.repr_override: Dict[(int, int), Any] = [] # {(x, y): Any}[]

    def get_board(self) -> list[list]:
        return deepcopy(self._board)
    
    def update_board(self, board: list[list]):
        assert len(board) == self._height and len(board[0]) == self._width, "size mismatch"
        self._board = deepcopy(board)

    def get_elem(self, x: int, y: int) -> (Any | None):
        if x >= self._width or x < 0: return None
        if y >= self._height or y < 0: return None

        return self._board[self._height-1-y][x]
    
    def _set_elem(self, x: int, y: int, val):
        assert not (x >= self._width or x < 0), "x not in range"
        assert not y >= self._height or y < 0, "y not in range"

        self._board[self._height-1-y][x] = val

    def adj_vals(self, x: int, y: int, val: list) -> list[tuple[int, int]]:
        deltas: list[tuple[int, int]] = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)]
        out: list[tuple[int, int]] = []
        for d in deltas:
            elem = self.get_elem(x+d[0], y+d[1])
            if elem in val:
                out.append(d)

        return out

    def adj_bombs(self, x: int, y: int) -> list[tuple[int, int]]:
        return self.adj_vals(x, y, ["B"])

    def adj_flags(self, x: int, y: int) -> list[tuple[int, int]]:
        return self.adj_vals(x, y, ["F"])

    def adj_number(self, x: int, y: int) -> list[tuple[int, int]]:
        return self.adj_vals(x, y, [1, 2, 3, 4, 5, 6, 7, 8])
    
    def cell_position_iter(self):
        for y in reversed(range(self._height)):
            for x in range(self._width):
                yield (x, y)
    
    def __repr__(self) -> str:
        out = ""
        for row in range(len(self._board)):
            for col in range(len(self._board[row])):
                elem = self._board[row][col]
                x, y = col, len(self._board)-row-1

                if (x, y) in self.repr_override:
                    elem = self.repr_override[(x, y)]

                out += f" {coloured_num(str(elem))} "
            out += f" | {self._height-row-1}\n"
        
        out += " -"
        for _ in range(len(self._board[0])):
            out += "---"
         
        out += "\n"

        max_digits = len(str(len(self._board[0])))

        for digit in range(max_digits):
            for col in range(len(self._board[0])):
                out += f" {str(col).rjust(max_digits, '0')[digit]} "
            out += "\n"

        self.repr_override = {}
        return out
    
    def __eq__(self, other: Board) -> bool:
        if self._width != other._width or self._height != other._height:
            return False
        
        for y in range(self._height):
            for x in range(self._width):
                if self.get_elem(x, y) != other.get_elem(x, y):
                    return False
                
        return True
    
class GameBoard:
    def __init__(self, width: int, height: int, num_bombs: int):
        self._solved_board = Board(width, height)
        self._board = Board(width, height)
        self.num_bombs = num_bombs
        self.placed_flags = 0
        self.generate_boards(width, height, num_bombs)

    def is_solved(self):
        solved_board = self._solved_board.get_board()
        player_board = self._board.get_board()
        placed_flags = 0

        for y in range(self._board._height):
            for x in range(self._board._width):
                solved_elem = solved_board[y][x]
                player_elem = player_board[y][x]
                if player_elem == "F":
                    placed_flags += 1
                    if solved_elem != "B":
                        return False
        return placed_flags == self.num_bombs

    def generate_boards(self, width: int, height: int, num_bombs: int):
        assert num_bombs <= width*height, "num_bombs must be less than or equal to width*height"
        bombs = [0]*width*height

        for i in range(num_bombs):
            bombs[i] = "B"
        random.shuffle(bombs)

        self._solved_board.update_board(unflaten_board(bombs, width, height))
        self._board.update_board(unflaten_board(["?"]*width*height, width, height))

        for x in range(width):
            for y in range(height):
                if self._solved_board.get_elem(x, y) != "B":
                    self._solved_board._set_elem(x, y, len(self._solved_board.adj_bombs(x, y)))
    
    def flood_fill(self, x: int, y: int):
        solved_elem = self._solved_board.get_elem(x, y)
        if self._board.get_elem(x, y) != "?":
            return
        if solved_elem == "B":
            return
        if solved_elem == None:
            return

        self._board._set_elem(x, y, solved_elem)

        if solved_elem != 0:
            return
        
        self.flood_fill_all_adj(x, y)
        

    def flood_fill_all_adj(self, x: int, y: int):
        deltas = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)]
        for d in deltas:
            self.flood_fill(x+d[0], y+d[1])

    def chord(self, x: int, y: int):
        elem = self._board.get_elem(x, y)
        if type(elem) != int:
            return
        if elem == 0:
            return
        if (len(self._board.adj_flags(x, y)) != elem):
            return
        
        self.flood_fill_all_adj(x, y)

    def chord_all(self):
        for x in range(self._board._width):
            for y in range(self._board._height):
                self.chord(x, y)

    def place_flags(self, x: int, y: int):
        elem = self._board.get_elem(x, y)
        adj = self._board.adj_vals(x, y, ["?", "F"])
        if len(adj) == elem:
            for d in adj:
                self._board._set_elem(x+d[0], y+d[1], "F")

    def place_all_flags(self):
        for x in range(self._board._width):
            for y in range(self._board._height):
                self.place_flags(x, y)
                

    def __repr__(self) -> str:
        return self._solved_board.__repr__() + "\n\n" + self._board.__repr__()