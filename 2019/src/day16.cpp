#include <iostream>
#include <array>
#include <vector>
#include <cassert>
#include <iterator>
#include "days.hpp"

namespace {
    std::array<int, 4> base_pattern{0, 1, 0, -1};

    int get_modifier(int rank, int pos) {
        pos += 1;
        pos /= rank + 1;

        return base_pattern[pos % 4];
    }

    std::vector<int> read_input(std::istream &input) {
        std::vector<int> result;

        for (char c; input >> c;) {
            assert(std::isdigit(c));
            result.push_back(c - '0');
        }

        return result;
    }
}

void aoc2019::day16_part1(std::istream &input, std::ostream &output) {
    auto numbers = read_input(input);

    for (int i = 0; i < 100; ++i) {
        std::vector<int> new_numbers;
        new_numbers.reserve(numbers.size());

        for (int rank = 0; rank < numbers.size(); ++rank) {
            int n = 0;
            for (int pos = 0; pos < numbers.size(); ++pos) {
                n += get_modifier(rank, pos) * numbers[pos];
            }

            n = std::abs(n % 10);

            new_numbers.push_back(n);
        }

        numbers = new_numbers;
    }

    std::copy(numbers.begin(), numbers.begin() + 8, std::ostream_iterator<int>(output));
    output << std::endl;
}

void aoc2019::day16_part2(std::istream &input, std::ostream &output) {
    output << "Not implemented\n";
}
