import fileinput
import re
from itertools import permutations

linePattern = re.compile(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.")

ratings = {}


# Read the input by regex
for line in fileinput.input():
    match = linePattern.match(line)
    person = match.group(1)
    if person not in ratings:
        ratings[person] = {}

    value = int(match.group(3))
    if match.group(2) == "lose":
        value *= -1

    target = match.group(4)
    ratings[person][target] = value

maxHappiness = 0
maxWithMe = 0

# Compute happiness with everyone present
for permutation in permutations(list(ratings.keys())):
    happiness = 0
    withMe = 0

    for i in range(len(permutation)):
        target = permutation[i]
        neighbour1 = permutation[(i + 1) % len(permutation)]
        neighbour2 = permutation[(i - 1 + len(permutation)) % len(permutation)]

        happiness += ratings[target][neighbour1]
        if i < len(permutation) - 1:
            withMe += ratings[target][neighbour1]

        happiness += ratings[target][neighbour2]
        if i > 0:
            withMe += ratings[target][neighbour2]

    maxHappiness = max(maxHappiness, happiness)
    maxWithMe = max(maxWithMe, withMe)


print(maxHappiness, maxWithMe)

