import re
import fileinput

dist = {}
visited = set()

def computeMinDistance(startPoint):
    visited.add(startPoint)

    if len(dist) == len(visited):
        maximum = 0
        minimum = 0
    else:
        minimum = None
        maximum = 0

    for i in dist[startPoint]:
        if i in visited:
            continue

        shortest, longest = computeMinDistance(i)

        if shortest is not None:
            shortest += dist[startPoint][i]
            longest += dist[startPoint][i]

            if minimum is None:
                minimum = shortest
                maximum = longest
            else:
                minimum = min(minimum, shortest)
                maximum = max(maximum, longest)

    visited.remove(startPoint)

    return minimum, maximum



linePattern = re.compile(r"(\w+) to (\w+) = (\d+)")

for line in fileinput.input():
    match = linePattern.match(line)
    if match.group(1) not in dist:
        dist[match.group(1)] = {}

    if match.group(2) not in dist:
        dist[match.group(2)] = {}

    dist[match.group(1)][match.group(2)] = int(match.group(3))
    dist[match.group(2)][match.group(1)] = int(match.group(3))

minimum = None
possible = []
for i in dist:
    shortest, longest = computeMinDistance(i)

    if shortest is not None:
        possible.append(shortest)
        possible.append(longest)

print "Shortest path is", min(possible)
print "Longest path is", max(possible)
