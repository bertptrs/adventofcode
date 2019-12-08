#include <algorithm>
#include <iostream>
#include <limits>
#include <string>
#include <string_view>
#include "days.hpp"

namespace {
    constexpr std::size_t WIDTH = 25;
    constexpr std::size_t HEIGHT = 6;
    constexpr std::size_t TILE_SIZE = WIDTH * HEIGHT;

    enum Color {
        BLACK = '0',
        WHITE = '1',
        TRANSPARENT = '2',
    };
}


void aoc2019::day08_part1(std::istream &input, std::ostream &output) {
    std::string buffer;
    std::getline(input, buffer);

    std::string_view image = buffer;
    auto best = std::numeric_limits<int>::max();
    auto best_score = 0;

    for (std::size_t i = 0; i < buffer.length(); i += TILE_SIZE) {
        auto tile = image.substr(i, TILE_SIZE);

        auto zeros = std::count(tile.begin(), tile.end(), '0');

        if (zeros < best) {
            best = zeros;

            best_score = std::count(tile.begin(), tile.end(), '1') * std::count(tile.begin(), tile.end(), '2');
        }
    }

    output << best_score << std::endl;
}

void aoc2019::day08_part2(std::istream &input, std::ostream &output) {
    std::string buffer;
    std::getline(input, buffer);

    std::string_view image = buffer;

    std::array<Color, TILE_SIZE> final_image;
    std::fill(final_image.begin(), final_image.end(), TRANSPARENT);

    for (std::size_t i = 0; i < buffer.length(); i += TILE_SIZE) {
        auto tile = image.substr(i, TILE_SIZE);

        for (int j = 0; j < TILE_SIZE; ++j) {
            if (final_image[j] == TRANSPARENT) {
                final_image[j] = static_cast<Color>(tile[j]);
            }
        }
    }

    for (std::size_t i = 0; i < final_image.size(); ++i) {
        output << (final_image[i] == WHITE ? '#' : ' ');
        if (i % WIDTH == WIDTH - 1) {
            output << '\n';
        }
    }
}
