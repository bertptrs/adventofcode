#!/usr/bin/python
import fileinput
import re

def read_action(it):
    next(it)
    to_write = 1 if "1" in next(it) else 0
    offset = 1 if "right" in next(it) else -1
    goto = next(it).strip()[-2]

    return (to_write, offset, goto)

def read_input():
    in_iter = fileinput.input()

    initial = next(in_iter).strip()[-2]
    runs = int(next(in_iter).split(" ")[-2])
    states = {}

    for line in in_iter:
        line = line.strip()

        if not line:
            continue

        state = line[-2]
        states[state] = (read_action(in_iter), read_action(in_iter))

    return initial, runs, states

state, steps, states = read_input()

ones = set()
pos = 0

for _ in range(steps):
    if pos in ones:
        instr = states[state][1]
    else:
        instr = states[state][0]

    if instr[0] == 1:
        ones.add(pos)
    else:
        ones.discard(pos)

    pos += instr[1]
    state = instr[2]

print(len(ones))
