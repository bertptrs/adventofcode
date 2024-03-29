import collections
from typing import List, TextIO, Tuple, Union


def read_program(data: TextIO) -> List[int]:
    line = next(data)

    return [int(i) for i in line.split(',')]


class Computer:
    program: List[int]
    pointer: int
    relative_base: int
    input: collections.deque[int]
    output: collections.deque[int]

    def __init__(self, program: List[int], pointer: int = 0) -> None:
        self.program = program
        self.pointer = pointer
        self.relative_base = 0
        self.input = collections.deque()
        self.output = collections.deque()

    def _mode_and_key(self, item: Union[int, Tuple[int, int]]) -> Tuple[int, int]:
        if isinstance(item, int):
            return 0, item
        else:
            mode, key = item
            return mode, self.program[self.pointer + key]

    def __getitem__(self, item: Union[int, Tuple[int, int]]) -> int:
        mode, key = self._mode_and_key(item)

        if mode == 1:
            return key
        elif mode == 0:
            pass  # Nothing to do here, handled below
        elif mode == 2:
            key += self.relative_base
        else:
            raise ValueError(f'Unsupported mode "{mode}"')

        self._ensure_length(key + 1)
        return self.program[key]

    def __setitem__(self, item: Union[int, Tuple[int, int]], value: int) -> None:
        mode, key = self._mode_and_key(item)

        if mode == 1:
            raise ValueError('Cannot assign to an immediate')
        elif mode == 0:
            pass # Nothing to do here, handled below
        elif mode == 2:
            key += self.relative_base
        else:
            raise ValueError(f'Unsupported mode "{mode}"')

        self._ensure_length(key + 1)
        self.program[key] = value

    def _ensure_length(self, length: int) -> None:
        if len(self.program) < length:
            if 2 * len(self.program) >= length:
                # Double current program size with 0s
                self.program.extend(0 for _ in range(len(self.program)))
            else:
                # Resize until the desired length
                self.program.extend(0 for _ in range(length - len(self.program)))

    def run(self) -> None:
        """ Run until failure """
        while self.execute_current():
            pass

    def get_output(self) -> int:
        return self.output.popleft()

    def send_input(self, data: int):
        self.input.append(data)

    def execute_current(self) -> bool:
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
            # Output
            self.output.append(self[mode[0], 1])
            self.pointer += 2
        elif opcode == 5:
            # Jump if true
            if self[mode[0], 1] != 0:
                self.pointer = self[mode[1], 2]
            else:
                self.pointer += 3
        elif opcode == 6:
            # Jump if false
            if self[mode[0], 1] == 0:
                self.pointer = self[mode[1], 2]
            else:
                self.pointer += 3
        elif opcode == 7:
            # Less than
            if self[mode[0], 1] < self[mode[1], 2]:
                self[mode[2], 3] = 1
            else:
                self[mode[2], 3] = 0
            self.pointer += 4
        elif opcode == 8:
            # Equals
            if self[mode[0], 1] == self[mode[1], 2]:
                self[mode[2], 3] = 1
            else:
                self[mode[2], 3] = 0
            self.pointer += 4
        elif opcode == 9:
            # Adjust relative base
            self.relative_base += self[mode[0], 1]
            self.pointer += 2
        elif opcode == 99:
            # Halt
            return False
        else:
            raise ValueError(f'Unknown opcode {opcode} at {pointer}')

        return True

