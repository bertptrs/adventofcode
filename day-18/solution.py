from __future__ import print_function
import fileinput


def neighbours(x, y, field):
    alive = 0
    for i in range(max(x - 1, 0), min(x + 2, len(field))):
        for j in range(max(y - 1, 0), min(y + 2, len(field))):
            if i == x and j == y:
                continue

            if field[i][j]:
                alive += 1

    return alive

def newState(x, y, field):
    alive = neighbours(x, y, field)
    if field[x][y]:
        return alive == 2 or alive == 3

    return alive == 3

def update(field):
    newField = []
    for x in range(len(field)):
        newField.append([newState(x, y, field) for y in range(len(field))])

    return newField

def enableCorners(field):
    size = len(field) - 1
    field[0][0] = field[0][size] = field[size][0] = field[size][size] = True


field = []
for line in fileinput.input():
    field.append([c == '#' for c in line.strip()])

original = field

for i_ in range(100):
    field = update(field)

print ("Without corners:", sum([sum(int(y) for y in x) for x in field]))

field = original
enableCorners(field)

for i_ in range(100):
    field = update(field)
    enableCorners(field)

print ("With corners:", sum([sum(int(y) for y in x) for x in field]))
