#include <iostream>
#include <vector>
#include <algorithm>
#include <set>
#include "days.hpp"

namespace {
    using field_t = std::array<std::array<bool, 5>, 5>;

    field_t read_input(std::istream &input) {
        std::string buffer;
        field_t map;

        int y = 0;

        while (std::getline(input, buffer)) {
            auto &row = map[y++];

            std::transform(buffer.begin(), buffer.end(), row.begin(), [](char c) { return c == '#'; });
        }

        return map;
    }

    void next_gen(const field_t &source, field_t &sink) {
        for (int y = 0; y < source.size(); ++y) {
            for (int x = 0; x < source[y].size(); ++x) {
                int neighbours = source[y][x] ? -1 : 0;
                for (int dy = -1; dy <= 1; ++dy) {
                    if (dy + y < 0 || dy + y >= source.size()) {
                        continue;
                    }
                    for (int dx = -1; dx <= 1; ++dx) {
                        if (dx + x < 0 || dx + x >= source[y].size() || dx * dy) {
                            continue;
                        }
                        neighbours += source[y + dy][x + dx];
                    }
                }

                sink[y][x] = neighbours == 1 || (!source[y][x] && neighbours == 2);
            }
        }
    }
}

void aoc2019::day24_part1(std::istream &input, std::ostream &output) {
    auto map = read_input(input);
    auto copy = map;

    std::set<field_t> seen;
    do {
        seen.insert(map);
        next_gen(map, copy);
        std::swap(map, copy);
    } while (!seen.count(map));

    unsigned int pow = 1;
    unsigned int diversity = 0;
    for (auto &row : map) {
        for (auto b : row) {
            if (b) {
                diversity += pow;
            }

            pow <<= 1u;
        }
    }
    output << diversity << std::endl;
}

void aoc2019::day24_part2(std::istream &input, std::ostream &output) {
    output << "Not implemented\n";
}
