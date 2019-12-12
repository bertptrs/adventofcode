#include <iostream>
#include <numeric>
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

    void simulate_step(std::vector<point_t> &moons, std::vector<point_t> &velocities) {
        update_velocities(moons, velocities);

        for (int j = 0; j < moons.size(); ++j) {
            moons[j] += velocities[j];
        }
    }
}

void aoc2019::day12_part1(std::istream &input, std::ostream &output) {
    auto moons = read_moons(input);
    std::vector<point_t> velocities(moons.size());

    for (int i = 0; i < 1000; ++i) {
        simulate_step(moons, velocities);
    }

    int energy = 0;

    for (int i = 0; i < moons.size(); ++i) {
        energy += moons[i].l1() * velocities[i].l1();
    }

    output << energy << std::endl;
}

void aoc2019::day12_part2(std::istream &input, std::ostream &output) {
    const auto moons = read_moons(input);
    auto moons_mut = moons;
    std::vector<point_t> velocities(moons.size());

    std::array<uint64_t, 3> recurrence = {0, 0, 0};

    std::uint64_t steps = 0;

    while (!std::all_of(recurrence.begin(), recurrence.end(), [](auto x) { return x > 0; })) {
        simulate_step(moons_mut, velocities);
        ++steps;

        for (int i = 0; i < 3; ++i) {
            if (!recurrence[i]) {
                bool back_again =
                        std::all_of(velocities.begin(), velocities.end(), [i](const auto &x) { return !x[i]; })
                        && std::equal(moons_mut.begin(), moons_mut.end(), moons.begin(),
                                      [i](const auto &a, const auto &b) {
                                          return a[i] == b[i];
                                      });

                if (back_again) {
                    recurrence[i] = steps;
                }
            }
        }
    }

    auto result = std::lcm(recurrence[0], std::lcm(recurrence[1], recurrence[2]));
    output << result << std::endl;
}
