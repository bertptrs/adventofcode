from __future__ import print_function
import re
import fileinput

correctAunt = {
        "children": 3,
        "cats": 7,
        "samoyeds": 2,
        "pomeranians": 3,
        "akitas": 0,
        "vizslas": 0,
        "goldfish": 5,
        "trees": 3,
        "cars": 2,
        "perfumes": 1
        }

def isOK(auntString):
    for thing, value in re.findall(r"([a-z]+): (\d+)", line):
        if correctAunt[thing] != int(value):
            return False

    return True

def isOK2(auntString):
    for thing, value in re.findall(r"([a-z]+): (\d+)", line):
        if thing == "cats" or thing == "trees":
            if int(value) <= correctAunt[thing]:
                return False

        elif thing == "pomeranians" or thing == "goldfish":
            if int(value) >= correctAunt[thing]:
                return False

        elif correctAunt[thing] != int(value):
            return False

    return True


for line in fileinput.input():
    if isOK(line):
        print("Found aunt first:", line)

    if isOK2(line):
        print("Found aunt second:", line)
