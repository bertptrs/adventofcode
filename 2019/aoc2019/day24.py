from typing import TextIO, Iterable, Tuple


def read_board(data: TextIO) -> Tuple[Tuple[bool]]:
    return tuple(
        tuple(c == '#' for c in line.strip())
        for line in data
    )


def flatten(it: Iterable[Iterable]) -> Iterable:
    for item in it:
        yield from item


def neighbours(board: Tuple[Tuple[bool]], x: int, y: int) -> int:
    n = 0

    if x > 0 and board[y][x - 1]:
        n += 1

    if x + 1 < len(board[0]) and board[y][x + 1]:
        n += 1

    if y > 0 and board[y - 1][x]:
        n += 1

    if y + 1 < len(board) and board[y + 1][x]:
        n += 1

    return n


def advance_board(board: Tuple[Tuple[bool]]) -> Tuple[Tuple[bool]]:
    def create_row(y: int, row: Tuple[bool]):
        new_row = []
        for x, live in enumerate(row):
            if live:
                new_row.append(neighbours(board, x, y) == 1)
            else:
                new_row.append(neighbours(board, x, y) in [1, 2])

        return tuple(new_row)

    return tuple(create_row(y, row) for y, row in enumerate(board))


def part1(data: TextIO) -> int:
    board = read_board(data)

    seen = set(board)

    while True:
        board = advance_board(board)
        if board in seen:
            return sum(2 ** i for i, b in enumerate(flatten(board)) if b)

        seen.add(board)
