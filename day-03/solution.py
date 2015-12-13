import fileinput

def move(pos, c):
    if c == '<':
        pos = (pos[0] + 1, pos[1])
    elif c == '>':
        pos = (pos[0] - 1, pos[1])
    elif c == '^':
        pos = (pos[0], pos[1] + 1)
    elif c == 'v':
        pos = (pos[0], pos[1] - 1)

    return pos

pos = (0, 0)
pos2 = pos
places = set([pos])

for line in fileinput.input():
    for idx,c in enumerate(line):
        if idx % 2 == 0:
            pos = move(pos, c)
            places.add(pos)
        else:
            pos2 = move(pos2, c)
            places.add(pos2)

print("Houses:", len(places))
