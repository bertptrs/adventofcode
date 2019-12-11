#include <iostream>
#include <cassert>
#include "days.hpp"
#include "utils.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;
    using aoc2019::IntCodeComputer;

    inline point_t turn_right(point_t direction) {
        return {-direction[1], direction[0]};
    }

    inline point_t turn_left(point_t direction) {
        return {direction[1], -direction[0]};
    }

    std::unordered_map<point_t, bool> simulate(std::istream &input, bool background = false) {
        std::unordered_map<point_t, bool> image;

        point_t direction{0, -1};
        point_t pos = {0, 0};

        IntCodeComputer computer(IntCodeComputer::read_intcode(input), {});
        std::deque<std::int64_t> outputs;

        computer.connectOutput(outputs);

        while (!computer.isTerminated()) {
            const auto it = image.find(pos);
            computer.sendInput(it != image.end() ? it->second : background);
            computer.run();

            if (!outputs.empty()) {
                assert(outputs.size() == 2);
                auto color = outputs.front();
                auto turn = outputs.back();
                outputs.clear();

                image[pos] = color;

                if (turn) {
                    direction = turn_right(direction);
                } else {
                    direction = turn_left(direction);
                }

                pos += direction;
            }
        }

        return image;
    }
}

void aoc2019::day11_part1(std::istream &input, std::ostream &output) {
    const auto result = simulate(input);

    output << result.size() << std::endl;
}

void aoc2019::day11_part2(std::istream &input, std::ostream &output) {
    const auto result = simulate(input, true);

    // Determine bounding box
    using limits = std::numeric_limits<int>;
    int left_edge = limits::max(), right_edge = limits::min(), top_edge = limits::max(), bottom_edge = limits::min();
    for (auto& entry : result) {
        left_edge = std::min(entry.first[0], left_edge);
        right_edge = std::max(entry.first[0], right_edge);
        top_edge = std::min(entry.first[1], top_edge);
        bottom_edge = std::max(entry.first[1], bottom_edge);
    }

    for (int y = top_edge; y <= bottom_edge; ++y) {
        for (int x = left_edge; x <= right_edge; ++x) {
            if (auto it = result.find({x, y}); it != result.end() && it->second) {
                output << '#';
            } else {
                output << ' ';
            }
        }

        output << '\n';
    }
}
