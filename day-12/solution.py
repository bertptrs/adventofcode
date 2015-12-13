import fileinput
import re
import json

def totals(obj):
    if isinstance(obj, int):
        return obj
    if isinstance(obj, list):
        return sum(totals(i) for i in obj)
    if not isinstance(obj, dict) or "red" in list(obj.values()):
        return 0

    return sum(totals(i) for i in list(obj.values()))

fileData = ''.join(line for line in fileinput.input())

# Solve the first part by regex, no parsing needed.
pattern = re.compile(r"-?\d+")
total = sum(int(match) for match in pattern.findall(fileData))

data = json.loads(fileData)

print(total, totals(data))
