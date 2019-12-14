#include <exception>
#include <iostream>
#include <array>
#include <vector>
#include "days.hpp"
#include "utils.hpp"

static int run_program(std::vector<std::int64_t> program) {
    aoc2019::IntCodeComputer computer(std::move(program));
    computer.run();

    return computer[0];
}

void aoc2019::day02_part1(std::istream &input, std::ostream &output) {
	auto program = IntCodeComputer::read_intcode(input);
	program[1] = 12;
	program[2] = 2;
	output << run_program(std::move(program)) << std::endl;
}

void aoc2019::day02_part2(std::istream &input, std::ostream &output) {
	auto program = IntCodeComputer::read_intcode(input);

	for (int noun = 0; noun < 100; ++noun) {
		for (int verb = 0; verb < 100; ++verb) {
			program[1] = noun;
			program[2] = verb;
			if (run_program(program) == 19690720) {
				output << 100 * noun + verb << std::endl;
				return;
			}
		}
	}
	throw std::domain_error("No valid solution.");
}
