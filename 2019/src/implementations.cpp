#include <array>
#include "days.hpp"
#include "implementations.hpp"

constexpr const std::array<std::array<aoc2019::solution_t, 2>, 25> SOLUTIONS = {
        {aoc2019::day01_part1, aoc2019::day01_part2}
};

aoc2019::solution_t aoc2019::get_implementation(int day, bool part2) {
    return SOLUTIONS.at(day - 1).at((int) part2);
}
