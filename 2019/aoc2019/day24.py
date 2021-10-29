from typing import TextIO, Iterable, Tuple, List


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


def neighbours2(board: List[Tuple[Tuple[bool]]], x: int, y: int, z: int) -> int:
    existing = range(len(board))

    if z in existing:
        # Normal board count, minus the middle tile if applicable
        n = neighbours(board[z], x, y) - board[z][2][2]
    else:
        n = 0

    if z - 1 in existing:
        if y == 2:
            if x == 1:
                n += sum(board[z - 1][iy][0] for iy in range(5))
            elif x == 3:
                n += sum(board[z - 1][iy][4] for iy in range(5))
        elif x == 2:
            if y == 1:
                n += sum(board[z - 1][0])
            elif y == 3:
                n += sum(board[z - 1][4])

    if z + 1 in existing:
        if y == 0:
            n += board[z + 1][1][2]
        elif y == 4:
            n += board[z + 1][3][2]

        if x == 0:
            n += board[z + 1][2][1]
        elif x == 4:
            n += board[z + 1][2][3]

    return n


def advance_board2(board: List[Tuple[Tuple[bool]]]) -> List[Tuple[Tuple[bool]]]:
    layers = []

    for z in range(-1, len(board) + 1):
        layer = []

        for y in range(5):
            row = []

            for x in range(5):
                if y == 2 and x == 2:
                    row.append(False)
                    continue

                if z in range(len(board)):
                    live = board[z][y][x]
                else:
                    live = False

                if live:
                    row.append(neighbours2(board, x, y, z) == 1)
                else:
                    row.append(neighbours2(board, x, y, z) in [1, 2])

            layer.append(tuple(row))

        layers.append(tuple(layer))

    if sum(flatten(layers[0])) == 0:
        layers = layers[1:]
    if sum(flatten(layers[-1])) == 0:
        layers = layers[:-1]

    return layers


def part1(data: TextIO) -> int:
    board = read_board(data)

    seen = set(board)

    while True:
        board = advance_board(board)
        if board in seen:
            return sum(2 ** i for i, b in enumerate(flatten(board)) if b)

        seen.add(board)


def part2(data: TextIO, rounds: int = 200) -> int:
    board = [read_board(data)]

    for _ in range(rounds):
        board = advance_board2(board)

    return sum(flatten(flatten(board)))
