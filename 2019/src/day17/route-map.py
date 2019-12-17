import fileinput
import sys


def turn_left(direction):
    x, y = direction
    return (y, -x)


def turn_right(direction):
    x, y = direction
    return (-y, x)


def add(pos, direction):
    return tuple(a + b for a, b in zip(pos, direction))


def main():
    chart = [line.strip() for line in fileinput.input()]

    pos = None

    for y, line in enumerate(chart):
        x = line.find('^')
        if x >= 0:
            pos = (x, y)
            break

    if not pos:
        sys.exit('starting point not found')

    route = ['L']

    direction = (-1, 0)

    def bounds_check(pos):
        x, y = pos

        return x >= 0 and y >= 0 and y < len(chart)

    while True:
        # try to move forward
        next_pos = add(direction, pos)
        dist = 0

        while bounds_check(next_pos) and chart[next_pos[1]][next_pos[0]] == '#':
            dist += 1
            pos = next_pos
            next_pos = add(direction, pos)

        if dist:
            route.append(dist)
        else:
            break

        for move, new_dir in zip(('L', 'R'), (turn_left(direction), turn_right(direction))):
            next_pos = add(pos, new_dir)
            if bounds_check(next_pos) and chart[next_pos[1]][next_pos[0]] == '#':
                route.append(move)
                direction = new_dir
                break

    printable_route = []
    for x in route:
        if x == 'L' or x == 'R':
            printable_route.append(x)
        else:
            printable_route += ['M'] * x

    print(','.join(str(x) for x in route))
    print(','.join(printable_route))


if __name__ == '__main__':
    main()
