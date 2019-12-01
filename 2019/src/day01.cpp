#include <iostream>
#include "days.hpp"

static inline int required(int weight) {
	return weight / 3 - 2;
}

void aoc2019::day01_part1(std::istream &input, std::ostream &output) {
	int total = 0;
	for (int current; input >> current;) {
		total += required(current);
	}

	output << total << std::endl;
}

void aoc2019::day01_part2(std::istream &input, std::ostream &output) {
	int total = 0;
	for (int current; input >> current;) {
		for (int fuel = required(current); fuel > 0; fuel = required(fuel)) {
			total += fuel;
		}
	}

	output << total << std::endl;
}
