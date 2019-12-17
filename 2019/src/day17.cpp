#include <iostream>
#include "days.hpp"
#include "utils.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    const std::unordered_map<point_t, std::int64_t> DIRECTIONS{
            {{0,  -1}, 1},
            {{0,  1},  2},
            {{-1, 0},  3},
            {{1,  0},  4},
    };

    std::unordered_map<point_t, char> read_scaffold(const std::deque<std::int64_t> &data) {
        int x = 0;
        int y = 0;
        std::unordered_map<point_t, char> map;
        for (auto n : data) {
            if (n == '\n') {
                ++y;
                x = 0;
                continue;
            } else {
                map[{x, y}] = (char) n;
                ++x;
            }
        }

        return map;
    }
}

void aoc2019::day17_part1(std::istream &input, std::ostream &output) {
    IntCodeComputer computer(input);
    std::deque<std::int64_t> output_buffer;
    computer.connectOutput(output_buffer);

    computer.run();

    const auto map = read_scaffold(output_buffer);

    std::int64_t total = 0;

    for (auto &entry : map) {
        if (entry.second == '.') continue;

        bool is_intersection = std::all_of(DIRECTIONS.begin(), DIRECTIONS.end(), [&map, &entry](auto &x) {
            auto it = map.find(x.first + entry.first);
            return it != map.end() && it->second != '.';
        });

        if (is_intersection) {
            total += entry.first[0] * entry.first[1];
        }
    }

    output << total << std::endl;
}

void aoc2019::day17_part2(std::istream &input, std::ostream &output) {
    output << "Not implemented\n";
}
