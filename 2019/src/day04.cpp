#include <iostream>
#include <utility>
#include "days.hpp"

namespace {
    constexpr bool is_valid_pass(int num) {
        bool has_double = false;
        int prev = 11;

        for (; num != 0; num /= 10) {
            int digit = num % 10;

            if (digit == prev) {
                has_double = true;
            }

            if (digit > prev) {
                return false;
            }

            prev = digit;
        }

        return has_double;
    }

    constexpr bool is_valid_pass2(int num) {
        int prev = 11;
        bool has_double = false;
        int run = 1;

        for (; num != 0; num /= 10) {
            int digit = num % 10;

            if (digit == prev) {
                ++run;
            } else {
                if (run == 2) {
                    has_double = true;
                }
                run = 1;
            }

            if (digit > prev) {
                return false;
            }

            prev = digit;
        }

        return has_double || run == 2;
    }

    std::pair<int, int> read_input(std::istream& input) {
        int a, b;
        input >> a;
        input.ignore();
        input >> b;

        return {a, b};
    }
}

void aoc2019::day04_part1(std::istream &input, std::ostream &output) {
    auto [start_range, end_range] = read_input(input);

    int num_valid = 0;
    for (; start_range <= end_range; ++start_range) {
        num_valid += is_valid_pass(start_range);
    }

    output << num_valid << std::endl;
}

void aoc2019::day04_part2(std::istream &input, std::ostream &output) {
    auto [start_range, end_range] = read_input(input);

    int num_valid = 0;
    for (; start_range <= end_range; ++start_range) {
        num_valid += is_valid_pass2(start_range);
    }

    output << num_valid << std::endl;
}

// Poor man's unit tests
static_assert(is_valid_pass(122345));
static_assert(is_valid_pass(111111));
static_assert(!is_valid_pass(223450));
static_assert(!is_valid_pass(123678));

static_assert(is_valid_pass2(112233));
static_assert(!is_valid_pass2(123444));
static_assert(is_valid_pass2(111122));
