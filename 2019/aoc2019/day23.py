from typing import TextIO

from aoc2019.intcode import read_program, Computer


def part1(data: TextIO) -> int:
    program = read_program(data)

    computers = [Computer(program.copy()) for _ in range(50)]

    for i, computer in enumerate(computers):
        computer.send_input(i)

    while True:
        for computer in computers:
            try:
                computer.run()
            except IndexError:
                computer.send_input(-1)

            while len(computer.output) >= 3:
                dest = computer.get_output()
                x = computer.get_output()
                y = computer.get_output()

                if dest == 255:
                    return y

                computers[dest].send_input(x)
                computers[dest].send_input(y)


def part2(data: TextIO) -> int:
    program = read_program(data)

    computers = [Computer(program.copy()) for _ in range(50)]

    for i, computer in enumerate(computers):
        computer.send_input(i)

    nat_value = None
    last_sent = None

    while True:
        is_idle = True

        for computer in computers:
            try:
                computer.execute_current()
                is_idle = False
            except IndexError:
                computer.send_input(-1)

            try:
                computer.run()
            except IndexError:
                pass

            while len(computer.output) >= 3:
                dest = computer.get_output()
                x = computer.get_output()
                y = computer.get_output()

                if dest == 255:
                    nat_value = (x, y)
                else:
                    computers[dest].send_input(x)
                    computers[dest].send_input(y)

        if is_idle:
            x, y = nat_value

            if last_sent == nat_value:
                return y
            else:
                computers[0].send_input(x)
                computers[0].send_input(y)
                last_sent = nat_value
