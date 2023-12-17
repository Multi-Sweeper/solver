import random
from utils import colour_str

class Board:
    def __init__(self, width: int, height: int, num_bombs: int):
        self._width = width
        self._height = height
        # format: _board[y][x]
        self.generate_board(width, height, num_bombs)
        # print(self._board)

    def get_elem(self, x: int, y: int):
        if x >= self._width or x < 0: return None
        if y >= self._height or y < 0: return None

        return self._board[self._height-1-y][x]
    
    def _set_elem(self, x: int, y: int, val):
        assert not (x >= self._width or x < 0), "x not in range"
        assert not y >= self._height or y < 0, "y not in range"

        self._board[self._height-1-y][x] = val

    def num_adj_bombs(self, x: int, y: int) -> int:
        num = 0
        deltas = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)]
        for d in deltas:
            elem = self.get_elem(x+d[0], y+d[1])
            if elem == "B":
                num += 1

        return num


    def generate_board(self, width: int, height: int, num_bombs: int):
        assert num_bombs <= width*height, "num_bombs must be less than or equal to width*height"
        self._board = [0]*width*height

        for i in range(num_bombs):
            self._board[i] = "B"

        random.shuffle(self._board)
        self._board = unflaten_board(self._board, width, height)

        for x in range(width):
            for y in range(height):
                if self.get_elem(x, y) != "B":
                    self._set_elem(x, y, self.num_adj_bombs(x, y))

        # return out
    
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
        "B": (0, 0, 0)
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