#include <iostream>
#include <numeric>
#include <unordered_set>
#include "days.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    std::vector<point_t> read_points(std::istream& input) {
        std::vector<point_t> result;

        int y = 0;

        for (std::string buffer; std::getline(input, buffer); ++y) {
            std::size_t x = 0;

            while ((x = buffer.find('#', x)) != std::string::npos) {
                result.push_back({(int) x, y});
                x += 1;
            }
        }

        return result;
    }

    point_t simplify(point_t x) {
        auto gcd = std::abs(std::gcd(x[0], x[1]));
        if (gcd > 1) {
            return { x[0] / gcd, x[1] / gcd };
        }

        return x;
    }
}

void aoc2019::day10_part1(std::istream &input, std::ostream &output) {
    const auto points = read_points(input);
    std::size_t best = 0;

    for (auto point : points) {
        std::unordered_set<point_t> visible;

        for (auto asteroid : points) {
            if (asteroid == point) continue;
            visible.insert(simplify(asteroid - point));
        }

        best = std::max(visible.size(), best);
    }

	output << best << std::endl;
}

void aoc2019::day10_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
