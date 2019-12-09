#include <algorithm>
#include <array>
#include <iostream>
#include "days.hpp"
#include "utils.hpp"

namespace {
    using aoc2019::IntCodeComputer;

    int simulate(const std::vector<int> &program, const std::array<int, 5> &phases) {
        int state = 0;
        for (int phase : phases) {
            auto copy = program;
            auto result = aoc2019::run_intcode(copy, {phase, state});
            state = result.front();
        }

        return state;
    }

    int simulate2(const std::vector<std::int64_t> &program, const std::array<int, 5> &phases) {
        std::vector<IntCodeComputer> computers;
        for (int phase : phases) {
            computers.emplace_back(program, std::deque<int64_t>{phase});
        }

        for (int i = 0; i < computers.size(); ++i) {
            computers[i].connectOutput(computers[(i + 1) % 5]);
        }

        computers[0].sendInput(0);

        while (std::any_of(computers.begin(), computers.end(), [](const auto &c) { return !c.isTerminated();})) {
            for (auto& computer : computers) {
                computer.run();
            }
        }

        return computers[0].currentInputs().back();
    }
}

void aoc2019::day07_part1(std::istream &input, std::ostream &output) {
    const auto program = aoc2019::read_intcode(input);
    std::array<int, 5> phases{0, 1, 2, 3, 4};

    int best = 0;

    do {
        best = std::max(simulate(program, phases), best);
    } while (std::next_permutation(phases.begin(), phases.end()));

    output << best << std::endl;
}

void aoc2019::day07_part2(std::istream &input, std::ostream &output) {
    const auto program = aoc2019::IntCodeComputer::read_intcode(input);
    std::array<int, 5> phases{5, 6, 7, 8, 9};

    int best = 0;

    do {
        best = std::max(simulate2(program, phases), best);
    } while (std::next_permutation(phases.begin(), phases.end()));

    output << best << std::endl;
}
