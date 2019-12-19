#include <iostream>
#include <cassert>
#include <queue>
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

    class Beam {
    private:
        aoc2019::IntCodeComputer computer;
        std::int64_t last_width = 1;
        std::int64_t last_start = 0;
        std::int64_t y = 0;

    public:
        Beam(std::istream &input) : computer(input) {};

        std::pair<std::int64_t, std::int64_t> next() {
            auto x = last_start;

            while (!bounds_check(computer, x, y)) {
                ++x;
            }

            last_start = x;
            x += last_width - 1;

            while (bounds_check(computer, x, y)) {
                ++x;
            }

            last_width = x - last_start;
            ++y;

            return {last_start, last_width};
        }
    };
}

void aoc2019::day19_part1(std::istream &input, std::ostream &output) {
    Beam beam(input);

    std::int64_t covered = 0;
    for (std::int64_t y = 0; y < 50; ++y) {
        const auto[start, width] = beam.next();

        if (start >= 50) break;

        covered += std::min(50 - start, width);
    }

    output << covered << std::endl;
}

void aoc2019::day19_part2(std::istream &input, std::ostream &output) {
    Beam beam(input);
    std::queue<std::int64_t> beam_ends;

    constexpr std::int64_t DIMENSION = 100;

    for (std::int64_t y = 0; true; ++y) {
        const auto[start, width] = beam.next();

        beam_ends.push(start + width);
        if (beam_ends.size() == DIMENSION) {
            auto end = beam_ends.front();
            if (end - start >= DIMENSION) {
                auto result = start * 10000 + y - DIMENSION + 1;
                output << result << std::endl;
                return;
            }
            beam_ends.pop();
        }
    }
}
