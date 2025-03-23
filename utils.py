def colour_str(str: str, rgb: tuple[int, int, int]):
    return f"\N{ESC}[38;2;{rgb[0]};{rgb[1]};{rgb[2]}m{str}\N{ESC}[0m"

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
        "F": (255, 50, 50),
        "?": (150, 150, 150)
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