from __future__ import print_function
import fileinput
import re
import sys

recipe = {}
values = {}

def findValue(node):
    match = re.search(r"\d+", node)
    if match:
        return int(match.group(0))

    if node in values:
        return values[node]

    if node not in recipe:
        sys.exit("Node without recipe '" + node + "'")

    valueA = None
    valueB = None
    if recipe[node][0]:
        valueA = findValue(recipe[node][0].strip())
    if recipe[node][2]:
        valueB = findValue(recipe[node][2])

    action = recipe[node][1]
    value = None
    if action == "RSHIFT":
        value = valueA >> valueB
    elif action == "LSHIFT":
        value = (valueA << valueB) & 0xffff
    elif action == "NOT":
        value = (~valueB) & 0xffff
    elif action == "IS":
        value = valueA & 0xffff
    elif action == "AND":
        value = valueA & valueB
    elif action == "OR":
        value = valueA | valueB
    else:
        sys.exit("Unsupported command " + action)

    values[node] = value
    return value

opPattern = re.compile(r"(\w+ )?(RSHIFT|LSHIFT|NOT|AND|OR) (\w+) -> (\w+)")
valuePattern = re.compile(r"(\w+) -> (\w+)")

for line in fileinput.input():
    match = valuePattern.match(line)
    if match:
        recipe[match.group(2)] = (match.group(1), "IS", "")
    else:
        match = opPattern.match(line)
        if match.group(4) in recipe:
            sys.exit("Node with multiple recipes")

        recipe[match.group(4)] = (match.group(1), match.group(2), match.group(3))

a = findValue("a")

print("Initial a is ", a)
values = {}
values["b"] = a
print("Secondary a is", findValue("a"))
