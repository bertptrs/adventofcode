from __future__ import print_function
import fileinput
import re

def doReplace(replacement, medicine):
    replacements = []
    for match in re.finditer(replacement[0], medicine):
        pos = match.start()
        sub = medicine[:pos] + replacement[1] + medicine[pos + len(replacement[0]):]
        replacements.append(sub)

    return replacements

def allReplacements(replacements, medicine):
    options = set()
    # Simply attempt all replacements
    for replacement in replacements:
        for option in doReplace(replacement, medicine):
            options.add(option)

    return options


# Exhaustive search trying to work back from the medicine.
#
# This is minimal because we try the larger substitutions first.
def solve(replacements, target, current):
    if current == target:
        return 0

    for org, rep in replacements:
        for option in doReplace((rep, org), current):
            result = solve(replacements, target, option)
            if result is not None:
                return result + 1

    return None

replacements = []
medicine = None

for line in fileinput.input():
    if len(line.strip()) == 0:
        continue

    match = re.match(r"(\w+) => (\w+)", line)
    if match:
        replacements.append(match.group(1, 2))
    else:
        medicine = line.strip()

replacements.sort(key=lambda x: -len(x[1]))


print("Options:", len(allReplacements(replacements, medicine)))

print("Steps:", solve(replacements, 'e', medicine))
