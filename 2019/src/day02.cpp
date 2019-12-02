#include <cstdio>
#include <exception>
#include <iostream>
#include <array>
#include <vector>
#include "days.hpp"

static std::vector<int> read_program(std::istream &input) {
	std::vector<int> program{};
	for (int current; input >> current; input.ignore()) {
		program.push_back(current);
	}

	return program;
}

static int run_program(std::vector<int> &program) {
	for (int ip = 0; ip < program.size(); ip += 4) {
		switch (program[ip]) {
			case 1:
				program[program[ip + 3]] = program[program[ip + 1]] + program[program[ip + 2]];
				break;

			case 2:
				program[program[ip + 3]] = program[program[ip + 1]] * program[program[ip + 2]];
				break;

			case 99:
				return program[0];

			default:
				char buffer[30];
				std::snprintf(buffer, sizeof(buffer), "Invalid opcode: %d", program[ip]);

				throw std::domain_error(buffer);
		}
	}
	throw std::out_of_range("Program read out of bounds");
}

void aoc2019::day02_part1(std::istream &input, std::ostream &output) {
	auto program = read_program(input);
	program[1] = 12;
	program[2] = 2;
	output << run_program(program) << std::endl;
}

void aoc2019::day02_part2(std::istream &input, std::ostream &output) {
	const auto program = read_program(input);

	for (int noun = 0; noun < 100; ++noun) {
		for (int verb = 0; verb < 100; ++verb) {
			auto program_copy = program;
			program_copy[1] = noun;
			program_copy[2] = verb;
			if (run_program(program_copy) == 19690720) {
				output << 100 * noun + verb << std::endl;
				return;
			}
		}
	}
	throw std::domain_error("No valid solution.");
}
