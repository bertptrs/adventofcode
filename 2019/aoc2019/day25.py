import itertools
from typing import TextIO, Iterable, Tuple, Set

from aoc2019.intcode import read_program, Computer


def powerset(iterable: Iterable) -> Iterable[Tuple]:
    s = list(iterable)
    return itertools.chain.from_iterable(itertools.combinations(s, r) for r in range(len(s) + 1))


def print_output(computer: Computer) -> str:
    output = ""

    while len(computer.output):
        output += chr(computer.get_output())

    print(output, end='')

    return output


def load_save(save, computer: Computer):
    computer.pointer, computer.relative_base, computer.program = save


def create_save(computer: Computer):
    return computer.pointer, computer.relative_base, computer.program.copy()


def send_command(computer: Computer, command: str):
    for c in command:
        computer.send_input(ord(c))

    computer.send_input(10)  # manually send newline


def force(computer: Computer, direction: str, items: Set[str]):
    # First, drop everything
    for item in items:
        send_command(computer, f"drop {item}")

    holding = tuple()

    for combination in powerset(items):
        # drop everything we don't want to keep holding
        for item in holding:
            if item not in combination:
                send_command(computer, f"drop {item}")

        # pick up what we want to pick up
        for item in combination:
            if item not in holding:
                send_command(computer, f"take {item}")

        send_command(computer, direction)

        try:
            computer.run()
            print_output(computer)
            return True
        except IndexError:
            print_output(computer)
            holding = combination

    return False


def part1(data: TextIO):
    print("This day must use a file as input as it requires the stdin for other things.")

    computer = Computer(read_program(data))
    items = set()

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

        try:
            command = input().strip()
        except EOFError:
            return "exiting"

        if command == "exit":
            return "exiting"
        elif command == "items":
            print(items)
            continue
        elif command.startswith("save "):
            save_name = command.removeprefix("save ")
            saves[save_name] = create_save(computer)
            print(f"Saved game state as {save_name}")
            continue
        elif command.startswith("load "):
            save_name = command.removeprefix("load ")
            load_save(saves[save_name], computer)
            print(f"Loaded game state from {save_name}")
            continue
        elif command.startswith("take "):
            items.add(command.removeprefix("take "))
        elif command.startswith("drop "):
            items.remove(command.removeprefix("drop "))
        elif command.startswith("force "):
            direction = command.removeprefix("force ")
            if force(computer, direction, items):
                return "success"
            continue

        send_command(computer, command)

