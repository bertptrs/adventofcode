from __future__ import division, print_function
import fileinput
import re
import itertools
from collections import defaultdict

pattern = re.compile("(\w+) can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds.")

runners = defaultdict(lambda: 0)
runnerData = {}
runnerPoints = defaultdict(lambda: 0)

totalTime = 2503

for line in fileinput.input():
    match = pattern.search(line)
    reindeer, speed, runtime, resttime = match.group(1, 2, 3, 4)
    speed = int(speed)
    runtime = int(runtime)
    resttime = int(resttime)

    runnerData[reindeer] = itertools.cycle([speed] * runtime + [0] * resttime)

for _ in range(totalTime):
    for i in runnerData:
        runners[i] += next(runnerData[i])

    best = max(runners.values())
    for i in runners:
        if runners[i] == best:
            runnerPoints[i] += 1

print("Fastest runner covered", max(runners.values()))
print("Highest score is", max(runnerPoints.values()))
