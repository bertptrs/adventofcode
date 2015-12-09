import fileinput
import re
import sys

commandExpr = re.compile(r"^(toggle|turn (on|off)) (\d+),(\d+) through (\d+),(\d+)$")

lights = []
for x in range(1000):
    lights.append([])
    for y in range(1000):
        lights[x].append(0)

for line in fileinput.input():
    match = commandExpr.search(line)

    if not match:
        print "Invalid string"
        sys.exit(1)

    xStart = int(match.group(3))
    yStart = int(match.group(4))

    xEnd = int(match.group(5))
    yEnd = int(match.group(6))

    command = match.group(1)

    for x in range(xStart, xEnd + 1):
        for y in range(yStart, yEnd + 1):
            if command == "toggle":
                lights[x][y] += 2
            elif "on" in command:
                lights[x][y] += 1
            else:
                lights[x][y] = max(0, lights[x][y] - 1)

total = 0
for row in lights:
    total += sum(row)

print total
