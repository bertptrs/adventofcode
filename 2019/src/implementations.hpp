#pragma once

#include <iosfwd>

namespace aoc2019 {
    typedef void (*solution_t)(std::istream &, std::ostream &);

    solution_t get_implementation(int day, bool part2 = false);
}
