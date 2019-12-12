#include <iostream>
#include <vector>
#include <regex>
#include "days.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 3> point_t;
    using aoc2019::from_chars;

    std::vector<point_t> read_moons(std::istream &input) {
        std::vector<point_t> moons;

        std::regex regex(R"(^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$)");
        std::smatch results;

        for (std::string buffer; std::getline(input, buffer);) {
            if (!std::regex_match(buffer, results, regex)) {
                throw std::domain_error(buffer);
            }

            point_t moon;
            for (int i = 0; i < 3; ++i) from_chars(results[i + 1].str(), moon[i]);

            moons.emplace_back(moon);
        }

        return moons;
    }

    void update_velocity(const point_t &a, point_t &va, const point_t &b, point_t &vb) {
        for (int i = 0; i < a.size(); ++i) {
            if (a[i] < b[i]) {
                va[i]++;
                vb[i]--;
            } else if (a[i] > b[i]) {
                va[i]--;
                vb[i]++;
            }
        }
    }

    void update_velocities(const std::vector<point_t> &positions, std::vector<point_t> &velocities) {
        for (int i = 0; i < positions.size(); ++i) {
            for (int j = i + 1; j < positions.size(); ++j) {
                update_velocity(positions[i], velocities[i], positions[j], velocities[j]);
            }
        }
    }
}

void aoc2019::day12_part1(std::istream &input, std::ostream &output) {
    auto moons = read_moons(input);
    std::vector<point_t> velocities(moons.size());

    for (int i = 0; i < 1000; ++i) {
        update_velocities(moons, velocities);

        for (int j = 0; j < moons.size(); ++j) {
            moons[j] += velocities[j];
        }
    }

    int energy = 0;

    for (int i = 0; i < moons.size(); ++i) {
        energy += moons[i].l1() * velocities[i].l1();
    }

	output << energy << std::endl;
}

void aoc2019::day12_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
