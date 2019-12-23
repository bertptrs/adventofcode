#include <iostream>
#include <string_view>
#include <cassert>
#include "days.hpp"
#include "utils.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    const std::unordered_map<char, point_t> DIRECTIONS{
            {'^', {0,  -1}},
            {'>', {0,  1}},
            {'v', {1,  0}},
            {'<', {-1, 0}},
    };

    std::unordered_map<point_t, char> read_scaffold(const std::deque<std::int64_t> &data) {
        int x = 0;
        int y = 0;
        std::unordered_map<point_t, char> map;
        for (auto n : data) {
            if (n == '\n') {
                if (x == 0) {
                    // Double newline, end of map
                    break;
                }
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
            auto it = map.find(x.second + entry.first);
            return it != map.end() && it->second != '.';
        });

        if (is_intersection) {
            total += entry.first[0] * entry.first[1];
        }
    }

    output << total << std::endl;
}

void aoc2019::day17_part2(std::istream &input, std::ostream &output) {
    using namespace std::literals;

    aoc2019::IntCodeComputer computer(input);
    computer[0] = 2;
    std::deque<std::int64_t> output_buffer;
    computer.connectOutput(output_buffer);

    std::array<std::string_view, 3> programs = {
            "L,6,R,8,L,4,R,8,L,12\n",
            "L,12,R,10,L,4\n",
            "L,12,L,6,L,4,L,4\n",
    };

    auto combined_programs = "A,B,B,C,B,C,B,C,A,A\n"sv;

    computer.sendInputs(combined_programs);

    for (auto program : programs) {
        computer.sendInputs(program);
    }

    // Don't give me output.
    computer.sendInputs("n\n");

    computer.run();

    assert(!output_buffer.empty());

    if (output_buffer.size() == 1) {
        output << output_buffer.front() << std::endl;
    } else {
        std::copy(output_buffer.begin(), output_buffer.end(), std::ostreambuf_iterator<char>(output));
        output << output_buffer.back() << std::endl;
    }
}
