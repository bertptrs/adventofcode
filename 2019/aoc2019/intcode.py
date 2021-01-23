import collections
from typing import List, TextIO, Tuple, Union


def read_program(data: TextIO) -> List[int]:
    line = next(data)

    return [int(i) for i in line.split(',')]


class Computer:
    program: List[int]
    pointer: int
    input: collections.deque[int]
    output: collections.deque[int]

    def __init__(self, program: List[int], pointer: int = 0) -> None:
        self.program = program
        self.pointer = pointer
        self.input = collections.deque()
        self.output = collections.deque()

    def _mode_and_key(self, item: Union[int, Tuple[int, int]]) -> Tuple[int, int]:
        if type(item) == int:
            mode = 0
            key = item
        else:
            mode, key = item
            key = self.program[self.pointer + key]

        return mode, key

    def __getitem__(self, item: Union[int, Tuple[int, int]]) -> int:
        mode, key = self._mode_and_key(item)

        if mode == 0:
            self._ensure_length(key + 1)
            return self.program[key]
        elif mode == 1:
            return key
        else:
            raise ValueError(f'Unsupported mode "{mode}"')

    def __setitem__(self, item: Union[int, Tuple[int, int]], value: int) -> None:
        mode, key = self._mode_and_key(item)

        if mode == 0:
            self._ensure_length(key + 1)
            self.program[key] = value
        elif mode == 1:
            raise ValueError('Cannot assign to an immediate')
        else:
            raise ValueError(f'Unsupported mode "{mode}"')

    def _ensure_length(self, length: int) -> None:
        if len(self.program) < length:
            # Double current program size with 0s
            self.program.extend(0 for _ in range(len(self.program)))

    def run(self) -> None:
        """ Run until failure """
        while self._execute_current():
            pass

    def _execute_current(self) -> bool:
        """
        Execute a single instruction
        :return: True if the program should continue
        """
        pointer = self.pointer
        instruction = self[pointer]
        opcode = instruction % 100

        mode = [
            (instruction // 100) % 10,
            (instruction // 1000) % 10,
            (instruction // 10000) % 10,
        ]

        if opcode == 1:
            # Add
            self[mode[2], 3] = self[mode[0], 1] + self[mode[1], 2]
            self.pointer += 4
        elif opcode == 2:
            # Multiply
            self[mode[2], 3] = self[mode[0], 1] * self[mode[1], 2]
            self.pointer += 4
        elif opcode == 3:
            # Input
            self[mode[0], 1] = self.input.popleft()
            self.pointer += 2
        elif opcode == 4:
            self.output.append(self[mode[0], 1])
            self.pointer += 2
        elif opcode == 99:
            # Halt
            return False
        else:
            raise ValueError(f'Unknown opcode {opcode} at {pointer}')

        return True

