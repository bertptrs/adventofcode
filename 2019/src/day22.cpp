#include <iostream>
#include <numeric>
#include "days.hpp"
#include "utils.hpp"

void aoc2019::day22_part1(std::istream &input, std::ostream &output) {
    constexpr int DECK_SIZE = 10007;

    std::vector<int> deck(DECK_SIZE);
    std::iota(deck.begin(), deck.end(), 0);

    auto copy = deck;

    std::string buffer;
    while (std::getline(input, buffer)) {
        std::string_view line = buffer;
        if (!line.find("deal into new stack")) {
            std::reverse(deck.begin(), deck.end());
        } else if (!line.find("deal with increment ")) {
            int new_increment;
            from_chars(line.substr(20), new_increment);
            int pos = 0;
            for (auto value : deck) {
                copy[pos] = value;
                pos = (pos + new_increment) % DECK_SIZE;
            }
            std::swap(deck, copy);
        } else {
            // cut
            int new_offset;
            from_chars(line.substr(4), new_offset);
            if (new_offset < 0) {
                new_offset = DECK_SIZE + new_offset;
            }

            auto it = std::copy(deck.begin() + new_offset, deck.end(), copy.begin());
            std::copy(deck.begin(), deck.begin() + new_offset, it);

            std::swap(deck, copy);
        }
    }

    auto location = std::find(deck.begin(), deck.end(), 2019);

    output << std::distance(deck.begin(), location) << std::endl;
}

void aoc2019::day22_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
