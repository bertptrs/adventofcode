from __future__ import print_function
import fileinput
import re
import sys

def compact(lightList):
    last = None
    newList = []
    for streak in sorted(lightList):
        if last is not None:
            if streak[2] == last[2]:
                # Repeating state, merge
                last = (last[0], streak[1], last[2])
                continue
            else:
                newList.append(last)

        last = streak

    newList.append(last)

    return newList

def updateState(sliceList, yStart, yEnd, updateFnc):
    newList = []
    for start, end, state in sliceList:
        if not start <= yStart  <= end and not yStart <= start <= yEnd:
            # Block not in range, skip
            newList.append((start, end, state))
            continue

        if start < yStart:
            # Split the block at the start
            newList.append((start, yStart - 1, state))
            start = yStart

        if end > yEnd:
            # Split the block at the end
            newList.append((yEnd + 1, end, state))
            end = yEnd

        newList.append((start, end, updateFnc(state)))

    return compact(newList)

def colsum(lightList):
    return sum(int(state) * (stop - start + 1) for start, stop, state in lightList)

def getPart1Update(command):
    if "toggle" in command:
        return lambda x: not x
    elif "on" in command:
        return lambda _: True
    else:
        return lambda _: False

def getPart2Update(command):
    if "toggle" in command:
        return lambda x: x + 2
    elif "on" in command:
        return lambda x: x + 1
    else:
        return lambda x: max(0, x - 1)

lights1 = []
lights2 = []
for x in range(1000):
    lights1.append([(0, 999, False)])
    lights2.append([(0, 999, 0)])

for line in fileinput.input():
    match = re.search(r"^(toggle|turn (on|off)) (\d+),(\d+) through (\d+),(\d+)$", line)

    xStart = int(match.group(3))
    yStart = int(match.group(4))

    xEnd = int(match.group(5))
    yEnd = int(match.group(6))

    command = match.group(1)

    for x in range(xStart, xEnd + 1):
        lights1[x] = updateState(lights1[x], yStart, yEnd, getPart1Update(command))
        lights2[x] = updateState(lights2[x], yStart, yEnd, getPart2Update(command))

print("Lights on:", sum(colsum(x) for x in lights1))
print("Lumen:", sum(colsum(x) for x in lights2))
