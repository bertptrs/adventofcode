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

    std::vector<int> advance(const std::vector<int>& numbers) {
        std::vector<int> new_numbers;
        new_numbers.reserve(numbers.size());

        for (int rank = 0; rank < numbers.size(); ++rank) {
            int n = 0;
            for (int pos = rank; pos < numbers.size(); ++pos) {
                n += get_modifier(rank, pos) * numbers[pos];
            }

            n = std::abs(n % 10);

            new_numbers.push_back(n);
        }

        return new_numbers;
    }
}

void aoc2019::day16_part1(std::istream &input, std::ostream &output) {
    auto numbers = read_input(input);

    for (int i = 0; i < 100; ++i) {
        std::vector<int> new_numbers;
        new_numbers.reserve(numbers.size());

        for (int rank = 0; rank < numbers.size(); ++rank) {
            int n = 0;
            for (int pos = rank; pos < numbers.size(); ++pos) {
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
    auto numbers = read_input(input);
    const auto initial_size = numbers.size();

    constexpr auto repetitions = 10000;

    numbers.reserve(repetitions * numbers.size());

    int offset = 0;
    for (int i = 0; i < 7; ++i) {
        offset *= 10;
        offset += numbers[i];
    }

    for (int i = 1; i < repetitions; ++i) {
        std::copy(numbers.begin(), numbers.begin() + initial_size, std::back_inserter(numbers));
    }

    numbers = std::vector(numbers.begin() + offset, numbers.end());

    for (int i = 0; i < 100; ++i) {
        std::vector<int> partial_sums(numbers.size());
        std::vector<int> new_numbers(numbers.size());

        partial_sums[0] = numbers[0];
        for (int j = 1; j < numbers.size(); ++j) {
            partial_sums[j] = partial_sums[j - 1] + numbers[j];
        }

        for (int j = 0; j < numbers.size(); ++j) {
            new_numbers[j] = partial_sums.back() - partial_sums[j] + numbers[j];
        }
    }

    std::copy(numbers.begin(), numbers.begin() + 8, std::ostream_iterator<int>(output));
}
