#include <iostream>
#include "days.hpp"
#include "utils.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int64_t, 2> point_t;

    enum class Tile {
        EMPTY,
        WALL,
        BLOCK,
        PADDLE,
        BALL,
    };
}

void aoc2019::day13_part1(std::istream &input, std::ostream &output) {
    aoc2019::IntCodeComputer computer(aoc2019::IntCodeComputer::read_intcode(input));
    std::deque<std::int64_t> output_buffer;

    computer.connectOutput(output_buffer);
    computer.run();

    std::unordered_map<point_t, Tile> drawn;

    while (!output_buffer.empty()) {
        auto x = output_buffer.front(); output_buffer.pop_front();
        auto y = output_buffer.front(); output_buffer.pop_front();
        auto type = output_buffer.front(); output_buffer.pop_front();

        drawn[{x, y}] = static_cast<Tile>(type);
    }

	output << std::count_if(drawn.begin(), drawn.end(), [](const auto& x) { return x.second == Tile::BLOCK; })<< std::endl;
}

void aoc2019::day13_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
