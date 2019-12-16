#include <iostream>
#include <array>
#include <vector>
#include <cassert>
#include <iterator>
#include <numeric>
#include "days.hpp"
#include "utils.hpp"

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

    void simulate(std::vector<int> numbers, std::ostream &output) {
        std::vector<int> new_numbers(numbers.size());
        std::vector<int> partial_sums(numbers.size());

        for (int i = 0; i < 100; ++i) {
            for (int rank = 0; rank < numbers.size(); ++rank) {
                std::partial_sum(numbers.begin() + rank, numbers.end(), partial_sums.begin() + rank);
                int n = 0;
                for (int pos = rank; pos < numbers.size(); pos += rank + 1) {
                    int run = std::min(rank + 1, (int) numbers.size() - pos);
                    if (int modifier = get_modifier(rank, pos); modifier) {
                        n += modifier * (partial_sums[pos + run - 1] - partial_sums[pos] + numbers[pos]);
                    }
                }

                n = std::abs(n % 10);

                new_numbers[rank] = n;
            }

            std::swap(numbers, new_numbers);
        }

        std::copy(numbers.begin(), numbers.begin() + 8, std::ostream_iterator<int>(output));
        output << std::endl;
    }

    int get_offset(const std::vector<int> &numbers) {
        int offset = 0;
        for (int i = 0; i < 7; ++i) {
            offset *= 10;
            offset += numbers[i];
        }

        return offset;
    }

    std::vector<int> numbers_from_offset(const std::vector<int> &numbers, unsigned int offset) {
        constexpr auto repetitions = 10000;
        const auto desired_length = repetitions * numbers.size() - offset;

        std::vector<int> numbers_after;
        numbers_after.reserve(desired_length);
        numbers_after.insert(numbers_after.end(), numbers.begin() + (offset % numbers.size()), numbers.end());

        while (numbers_after.size() < desired_length) {
            auto remaining = desired_length - numbers_after.size();
            if (remaining >= numbers.size()) {
                numbers_after.insert(numbers_after.end(), numbers.begin(), numbers.end());
            } else {
                numbers_after.insert(numbers_after.end(), numbers.begin(), numbers.end() + remaining);
            }
        }

        return numbers_after;
    }
}

void aoc2019::day16_part1(std::istream &input, std::ostream &output) {
    auto numbers = read_input(input);

    simulate(std::move(numbers), output);
}

void aoc2019::day16_part2(std::istream &input, std::ostream &output) {
    auto numbers = read_input(input);

    const int offset = get_offset(numbers);

    numbers = numbers_from_offset(numbers, offset);

    std::vector<int> new_numbers(numbers.size());
    std::vector<int> partial_sums(numbers.size());

    for (int i = 0; i < 100; ++i) {
        std::partial_sum(numbers.rbegin(), numbers.rend(), partial_sums.rbegin());

        std::transform(partial_sums.begin(), partial_sums.end(), new_numbers.begin(), [](int x) {
            return x % 10;
        });

        std::swap(numbers, new_numbers);
    }

    std::copy(numbers.begin(), numbers.begin() + 8, std::ostream_iterator<int>(output));
    output << std::endl;
}
