#pragma once

#include <iosfwd>

namespace aoc2019 {
    typedef void (*solution_t)(std::istream &, std::ostream &);

    solution_t get_implementation(int day, bool part2 = false);

    // Declarations of all implemented days.
    void day01_part1(std::istream &input, std::ostream &output);
    void day01_part2(std::istream &input, std::ostream &output);
}
