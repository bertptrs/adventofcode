import fileinput
import re

nice1 = 0
nice2 = 0

doubleletter = re.compile(r"(.)\1")
vowels = re.compile(r"[aeiou]")
forbidden = re.compile(r"ab|cd|pq|xy")

inbetween = re.compile(r"(.).\1")
twodouble = re.compile(r"(.)(.)(.*?)\1\2")

for line in fileinput.input():
    if len(vowels.findall(line)) >= 3 and doubleletter.search(line) and not forbidden.search(line):
        nice1 += 1

    if inbetween.search(line) and twodouble.search(line):
        nice2 += 1

print(nice1, "nice1 strings.")
print(nice2, "nice2 strings.")
