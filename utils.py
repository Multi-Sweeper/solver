def colour_str(str: str, rgb: tuple[int, int, int]):
    return f"\N{ESC}[38;2;{rgb[0]};{rgb[1]};{rgb[2]}m{str}\N{ESC}[0m"