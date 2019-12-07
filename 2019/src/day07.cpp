#include <iostream>
#include "days.hpp"
#include "utils.hpp"

namespace  {
    int simulate(const std::vector<int>& program, const std::vector<int> phases) {
        int state = 0;
        for (int phase : phases) {
            auto copy = program;
            auto result = aoc2019::run_intcode(copy, { phase, state });
            state = result.front();
        }

        return state;
    }
}

void aoc2019::day07_part1(std::istream &input, std::ostream &output) {
    const auto program = aoc2019::read_intcode(input);
    std::vector<int> phases{0, 1, 2, 3, 4};

    int best = 0;

    do {
        best = std::max(simulate(program, phases), best);
    } while (std::next_permutation(phases.begin(), phases.end()));

	output << best << std::endl;
}

void aoc2019::day07_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
