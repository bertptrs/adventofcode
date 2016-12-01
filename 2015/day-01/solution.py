from __future__ import print_function
import fileinput

floor = 0
seenBasement = False

for line in fileinput.input():
    for idx, c in enumerate(line.strip()):
        if c == '(':
            floor += 1
        else:
            floor -= 1

        if not seenBasement and floor == -1:
            seenBasement = True
            print("Basement reached at", idx + 1)

print("Finally arrived at", floor)
