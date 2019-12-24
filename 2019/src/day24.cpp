#include <iostream>
#include <vector>
#include <algorithm>
#include <set>
#include <numeric>
#include "days.hpp"

namespace {
    using field_t = std::array<std::array<bool, 5>, 5>;

    constexpr int EDGE = 4;
    constexpr int MIDPOINT = 2;

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

    int num_bees(const field_t &field) {
        int total = 0;
        for (auto &row : field) {
            total += std::count(row.begin(), row.end(), true);
        }

        return total;
    }

    std::unordered_map<int, field_t> advance(const std::unordered_map<int, field_t> &state) {
        const auto dimension_range = std::minmax_element(state.begin(), state.end());
        const auto min = dimension_range.first->first - 1;
        const auto max = dimension_range.second->first + 1;

        std::unordered_map<int, field_t> next_gen;

        auto has_bee = [&state](int dimension, int x, int y) {
            if (auto it = state.find(dimension); it != state.end()) {
                return it->second[y][x];
            }

            return false;
        };

        for (int dimension = min; dimension <= max; ++dimension) {
            field_t field{};
            if (auto it = state.find(dimension); it != state.end()) {
                field = it->second;
            }

            auto get_neighbours = [has_bee,dimension](int x, int y) {
                int neighbours = 0;

                // Cell above
                if (y == 0) {
                    neighbours += has_bee(dimension + 1, MIDPOINT, 1);
                } else if (y == 3 && x == MIDPOINT) {
                    for (int sx = 0; sx < 5; ++sx) {
                        neighbours += has_bee(dimension - 1, sx, EDGE);
                    }
                } else {
                    neighbours += has_bee(dimension, x, y - 1);
                }

                // Cell below
                if (y == EDGE) {
                    neighbours += has_bee(dimension + 1, MIDPOINT, 3);
                } else if (y == 1 && x == MIDPOINT) {
                    for (int sx = 0; sx < 5; ++sx) {
                        neighbours += has_bee(dimension - 1, sx, 0);
                    }
                } else {
                    neighbours += has_bee(dimension, x, y + 1);
                }

                // Cell left
                if (x == 0) {
                    neighbours += has_bee(dimension + 1, 1, 2);
                } else if (x == 3 && y == MIDPOINT) {
                    for (int sy = 0; sy < 5; ++sy) {
                        neighbours += has_bee(dimension - 1, EDGE, sy);
                    }
                } else {
                    neighbours += has_bee(dimension, x - 1, y);
                }

                // Cell right
                if (x == EDGE) {
                    neighbours += has_bee(dimension + 1, 3, MIDPOINT);
                } else if (x == 1 && y == MIDPOINT) {
                    for (int sy = 0; sy < 5; ++sy) {
                        neighbours += has_bee(dimension - 1, 0, sy);
                    }
                } else {
                    neighbours += has_bee(dimension, x + 1, y);
                }

                return neighbours;
            };


            for (int y = 0; y < 5; ++y) {
                for (int x = 0; x < 5; ++x) {
                    auto neighbours = get_neighbours(x, y);
                    field[y][x] = neighbours == 1 || (neighbours == 2 && !field[y][x]);
                }
            }

            // Don't evolve the midpoint.
            field[2][2] = false;

            if (num_bees(field) || (dimension != min && dimension != max)) {
                next_gen[dimension] = field;
            }
        }

        return next_gen;
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
    std::unordered_map<int, field_t> fields;
    fields[0] = read_input(input);

    for (int gen = 0; gen < 200; ++gen) {
        fields = advance(fields);
    }

    int total = std::accumulate(fields.begin(), fields.end(), 0, [](auto cur, const auto &it) {
        return cur + num_bees(it.second);
    });

    output << total << std::endl;
}
