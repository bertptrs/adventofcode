from __future__ import print_function
import fileinput
import re
import sys

commandExpr = re.compile(r"^(toggle|turn (on|off)) (\d+),(\d+) through (\d+),(\d+)$")

lights = []
for x in range(1000):
    lights.append([])
    for y in range(1000):
        lights[x].append(False)

for line in fileinput.input():
    match = commandExpr.search(line)

    if not match:
        print("Invalid string")
        sys.exit(1)

    xStart = int(match.group(3))
    yStart = int(match.group(4))

    xEnd = int(match.group(5))
    yEnd = int(match.group(6))

    command = match.group(1)

    for x in range(xStart, xEnd + 1):
        for y in range(yStart, yEnd + 1):
            if command == "toggle":
                lights[x][y] = not lights[x][y]
            elif "on" in command:
                lights[x][y] = True
            else:
                lights[x][y] = False

total = 0
for row in lights:
    total += sum([int(i) for i in row])

print(total)
