from typing import List, TextIO


def read_program(data: TextIO) -> List[int]:
    line = next(data)

    return [int(i) for i in line.split(',')]


class Computer:
    program: List[int]
    pointer: int

    def __init__(self, program: List[int], pointer: int = 0) -> None:
        self.program = program
        self.pointer = pointer

    def __getitem__(self, item: int) -> int:
        self._ensure_length(item + 1)
        return self.program[item]

    def __setitem__(self, key: int, value: int) -> None:
        self._ensure_length(key + 1)
        self.program[key] = value

    def _ensure_length(self, length: int) -> None:
        if len(self.program) < length:
            # Double current program size with 0s
            self.program.extend(0 for _ in range(len(self.program)))

    def run(self) -> None:
        """ Run until failure"""
        while self._execute_current():
            pass

    def _execute_current(self) -> bool:
        """
        Execute a single instruction
        :return: True if the program should continue
        """
        pointer = self.pointer
        opcode = self[pointer]

        if opcode == 1:
            # Add
            self[self[pointer + 3]] = self[self[pointer + 1]] + self[self[pointer + 2]]
            self.pointer += 4
        elif opcode == 2:
            # Multiply
            self[self[pointer + 3]] = self[self[pointer + 1]] * self[self[pointer + 2]]
            self.pointer += 4
        elif opcode == 99:
            # Halt
            return False
        else:
            raise ValueError(f'Unknown opcode {opcode} at {pointer}')

        return True

