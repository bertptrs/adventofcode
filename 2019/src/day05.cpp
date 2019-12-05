#include <iostream>
#include "days.hpp"
#include "utils.hpp"

void aoc2019::day05_part1(std::istream &input, std::ostream &output) {
    auto program = read_intcode(input);
    auto result = aoc2019::run_intcode(program, { 1 });
	output << result.back() << std::endl;
}

void aoc2019::day05_part2(std::istream &input, std::ostream &output) {
    auto program = read_intcode(input);
    auto result = run_intcode(program, { 5 });
    output << result.back() << std::endl;
}
