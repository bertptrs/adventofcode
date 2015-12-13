import sys
from hashlib import md5
import fileinput

def ok(digest, zeroes):
    for c in digest[0:zeroes]:
        if c != "0":
            return False

    return True

def solve(word, zeroes):
    number = 0

    while True:
        digester = md5(word.encode("utf-8"))
        digester.update(str(number).encode("utf-8"))

        if ok(digester.hexdigest(), zeroes):
            print(word, number)
            break

        number = number + 1

for line in fileinput.input():
    word, zeroes = line.split("\t")
    zeroes = int(zeroes.strip())

    solve(word, zeroes)
