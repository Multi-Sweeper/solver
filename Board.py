from __future__ import annotations
import random
from utils import colour_str
from copy import deepcopy

class Board:
    def __init__(self, width: int, height: int):
        self._width = width
        self._height = height
        self._board = unflaten_board([0]*width*height, width, height)

    def get_board(self):
        return deepcopy(self._board)
    
    def update_board(self, board: list[list]):
        assert len(board) == self._height and len(board[0]) == self._width, "size mismatch"
        self._board = deepcopy(board)

    def get_elem(self, x: int, y: int):
        if x >= self._width or x < 0: return None
        if y >= self._height or y < 0: return None

        return self._board[self._height-1-y][x]
    
    def _set_elem(self, x: int, y: int, val):
        assert not (x >= self._width or x < 0), "x not in range"
        assert not y >= self._height or y < 0, "y not in range"

        self._board[self._height-1-y][x] = val

    def adj_vals(self, x: int, y: int, val: list) -> int:
        out = []
        deltas = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)]
        for d in deltas:
            elem = self.get_elem(x+d[0], y+d[1])
            if elem in val:
                out.append(d)

        return out

    def adj_bombs(self, x: int, y: int) -> int:
        return self.adj_vals(x, y, ["B"])

    def adj_flags(self, x: int, y: int) -> int:
        return self.adj_vals(x, y, ["F"])
    
    def __repr__(self) -> str:
        out = ""
        for row in range(len(self._board)):
            out += f"{self._height-row-1}|"
            for elem in self._board[row]:
                out += f" {coloured_num(str(elem))} "
            out += "\n"
        
        out += " -"
        for _ in range(len(self._board[0])):
            out += "---"
         
        out += "\n  "
        for col in range(len(self._board[0])):
            out += f" {col} "
        return out
    
class GameBoard:
    def __init__(self, width: int, height: int, num_bombs: int):
        self._solved_board = Board(width, height)
        self._board = Board(width, height)
        self.num_bombs = num_bombs
        self.placed_flags = 0
        self.generate_boards(width, height, num_bombs)

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
                

    def __repr__(self):
        return self._solved_board.__repr__() + "\n\n" + self._board.__repr__()

def coloured_num(str: str):
    colours ={
        "1": (0, 120, 255),
        "2": (0, 255, 0),
        "3": (255, 0, 0),
        "4": (0, 0, 255),
        "5": (150, 0, 0),
        "6": (0, 130, 130),
        "7": (100, 100, 100),
        "8": (0, 0, 0),
        "B": (0, 0, 0),
        "F": (255, 50, 50)
    }

    if (str in colours):
        return colour_str(str, colours[str])
    
    return str

def unflaten_board(flat_board: list[int], width: int, height: int):
    out = [[]]
    for elem in flat_board:
        if len(out[-1]) >= width:
            out.append([])        
        out[-1].append(elem)

    assert len(out) == height, "error unflatening board"
    return out