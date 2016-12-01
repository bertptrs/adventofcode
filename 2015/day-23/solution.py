from __future__ import division, print_function
import fileinput
import re

def run(instructions, registers):
    instrptr = 0
    while instrptr < len(instructions):
        instruction = instructions[instrptr]
        if "j" in instruction[0]:
            if instruction[0] == "jie":
                doJump = registers[instruction[1]] % 2 == 0
            elif instruction[0] == "jio":
                doJump = registers[instruction[1]] == 1
            else:
                doJump = True

            if doJump:
                instrptr = instrptr + int(instruction[-1])
            else:
                instrptr += 1

            continue

        if instruction[0] == "hlf":
            registers[instruction[1]] //= 2
        elif instruction[0] == "tpl":
            registers[instruction[1]] *= 3
        else:
            registers[instruction[1]] += 1

        instrptr += 1

    return registers

instructions = []

for line in fileinput.input():
    instructions.append(tuple(x.strip() for x in re.match(r"(hlf|tpl|inc|jmp|jie|jio) (a|b)?,? ?(\+?-?[0-9]+)?", line).groups() if x is not None))



print(run(instructions, {'a': 0, 'b': 0})['b'])
print(run(instructions, {'a': 1, 'b': 0})['b'])

