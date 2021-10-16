from typing import TextIO

from aoc2019.intcode import read_program, Computer


def print_output(computer: Computer):
    output = ""

    while len(computer.output):
        output += chr(computer.get_output())

    print(output, end='')


def load_save(save, computer: Computer):
    computer.pointer, computer.relative_base, computer.program = save


def create_save(computer: Computer):
    return computer.pointer, computer.relative_base, computer.program.copy()


def part1(data: TextIO):
    print("This day must use a file as input as it requires the stdin for other things.")

    computer = Computer(read_program(data))

    saves = {}

    while True:
        try:
            computer.run()
            print_output(computer)

            print("detected a death, loading auto save...")
            load_save(saves['auto'], computer)
            print("Command?")
        except IndexError:
            pass

        print_output(computer)

        saves['auto'] = create_save(computer)

        command = input().strip()

        if command == "exit":
            return
        # TODO: add manual save states.

        for c in command:
            computer.send_input(ord(c))

        computer.send_input(10)  # manually send newline

