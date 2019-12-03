#include <cassert>
#include <charconv>
#include <iostream>
#include <limits>
#include <unordered_map>
#include <utility>
#include <vector>
#include "days.hpp"
#include "point.hpp"
#include "utils.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    const std::unordered_map<char, point_t> DIRECTION_MAP = {
            {'U', {0,  -1}},
            {'D', {0,  1}},
            {'L', {-1, 0}},
            {'R', {1,  0}},
    };

    std::unordered_map<point_t, int> get_points(std::string_view line) {
        std::unordered_map<point_t, int> points{};
        point_t pos = {};

        int steps = 0;
        for (auto entry = aoc2019::strtok(line); !line.empty() || !entry.empty(); entry = aoc2019::strtok(line)) {
            const auto dir = DIRECTION_MAP.at(entry[0]);
            std::size_t amount = 0;
            std::from_chars(entry.data() + 1, entry.data() + entry.size(), amount);
            assert(amount > 0 && "Must have some valid direction");

            for (std::size_t i = 0; i < amount; ++i) {
                ++steps;
                pos += dir;
                if (!points.count(pos)) {
                    points[pos] = steps;
                }
            }
        }

        return points;
    }

    std::pair<std::unordered_map<point_t, int>, std::unordered_map<point_t, int>> read_input(std::istream& input) {
        std::string buffer;
        std::getline(input, buffer);
        auto a = get_points(buffer);
        std::getline(input, buffer);
        auto b = get_points(buffer);

        return { std::move(a), std::move(b) };
    }
}

void aoc2019::day03_part1(std::istream &input, std::ostream &output) {
    auto [a, b] = read_input(input);

    int best = std::numeric_limits<int>::max();

    for (const auto& point : a) {
        if (b.count(point.first) && point.first.l1() < best) {
            best = point.first.l1();
        }
    }

    output << best << std::endl;
}

void aoc2019::day03_part2(std::istream &input, std::ostream &output) {
    auto [a, b] = read_input(input);

    int best = std::numeric_limits<int>::max();

    for (const auto& ap : a) {
        const auto bp = b.find(ap.first);

        if (bp != b.cend() && (ap.second + bp->second) < best) {
            best = ap.second + bp->second;
        }
    }

    output << best << std::endl;
}
