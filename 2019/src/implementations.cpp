#include <array>
#include "days.hpp"
#include "implementations.hpp"

constexpr const std::array<std::array<aoc2019::solution_t, 2>, 25> SOLUTIONS = {{
        {aoc2019::day01_part1, aoc2019::day01_part2},
        {aoc2019::day02_part1, aoc2019::day02_part2},
        {aoc2019::day03_part1, aoc2019::day03_part2},
        {aoc2019::day04_part1, aoc2019::day04_part2},
        {aoc2019::day05_part1, aoc2019::day05_part2},
        {aoc2019::day06_part1, aoc2019::day06_part2},
        {aoc2019::day07_part1, aoc2019::day07_part2},
        {aoc2019::day08_part1, aoc2019::day08_part2},
        {aoc2019::day09_part1, aoc2019::day09_part2},
        {aoc2019::day10_part1, aoc2019::day10_part2},
        {aoc2019::day11_part1, aoc2019::day11_part2},
        {aoc2019::day12_part1, aoc2019::day12_part2},
        {aoc2019::day13_part1, aoc2019::day13_part2},
        {aoc2019::day14_part1, aoc2019::day14_part2},
        {aoc2019::day15_part1, aoc2019::day15_part2},
        {aoc2019::day16_part1, aoc2019::day16_part2},
        {aoc2019::day17_part1, aoc2019::day17_part2},
        {aoc2019::day18_part1, aoc2019::day18_part2},
        {aoc2019::day19_part1, aoc2019::day19_part2},
        {aoc2019::day20_part1, aoc2019::day20_part2},
        {aoc2019::day21_part1, aoc2019::day21_part2},
        {aoc2019::day22_part1, aoc2019::day22_part2},
        {aoc2019::day23_part1, aoc2019::day23_part2},
        {aoc2019::day24_part1, aoc2019::day24_part2},
        {aoc2019::day25_part1, aoc2019::day25_part2},
}};

aoc2019::solution_t aoc2019::get_implementation(int day, bool part2) {
    return SOLUTIONS.at(day - 1).at((int) part2);
}
