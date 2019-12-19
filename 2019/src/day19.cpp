#include <iostream>
#include <cassert>
#include "days.hpp"
#include "utils.hpp"

namespace {
    bool bounds_check(aoc2019::IntCodeComputer computer, std::int64_t x, std::int64_t y) {
        std::deque<std::int64_t> output_buffer;
        computer.connectOutput(output_buffer);

        computer.sendInput(x);
        computer.sendInput(y);

        computer.run();
        assert(computer.isTerminated());
        assert(!output_buffer.empty());

        return output_buffer.front();
    }
}

void aoc2019::day19_part1(std::istream &input, std::ostream &output) {
    IntCodeComputer computer(input);

    std::int64_t covered = 0;
    int last_width = 1;
    int last_start = 0;
    for (std::int64_t y = 0; y < 50; ++y) {
        auto x = last_start;

        while (!bounds_check(computer, x, y) && x < 50) {
            ++x;
        }

        if (x == 50) break;

        last_start = x;
        x += last_width - 1;

        if (!bounds_check(computer, x, y)) {
            std::cerr << x << "," << y << "," << covered << std::endl;
            throw std::logic_error("Assumption false");
        }

        while (bounds_check(computer, x, y) && x < 50) {
            ++x;
        }

        last_width = x - last_start;
        covered += last_width;
    }

    output << covered << std::endl;
}

void aoc2019::day19_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
