#include <iostream>
#include <numeric>
#include "days.hpp"
#include "utils.hpp"

namespace {
    enum class Operation {
        Stack,
        Deal,
        Cut
    };

    using Move = std::pair<Operation, int>;

    std::vector<Move> read_moves(std::istream &input) {
        std::string buffer;
        std::vector<Move> moves;

        while (std::getline(input, buffer)) {
            std::string_view line = buffer;
            if (!line.find("deal into new stack")) {
                moves.emplace_back(Operation::Stack, 0);
            } else if (!line.find("deal with increment ")) {
                int new_increment;
                aoc2019::from_chars(line.substr(20), new_increment);
                moves.emplace_back(Operation::Deal, new_increment);
            } else {
                // cut
                int new_offset;
                aoc2019::from_chars(line.substr(4), new_offset);

                moves.emplace_back(Operation::Cut, new_offset);
            }
        }

        return moves;
    }
}

void aoc2019::day22_part1(std::istream &input, std::ostream &output) {
    constexpr int DECK_SIZE = 10007;

    std::vector<int> deck(DECK_SIZE);
    std::iota(deck.begin(), deck.end(), 0);

    auto copy = deck;

    for (auto move : read_moves(input)) {
        int argument = move.second;
        int pos;
        switch (move.first) {
            case Operation::Stack:
                std::reverse(deck.begin(), deck.end());
                break;

            case Operation::Deal:
                pos = 0;
                for (auto value : deck) {
                    copy[pos] = value;
                    pos = (pos + argument) % DECK_SIZE;
                }
                std::swap(deck, copy);
                break;

            case Operation::Cut:
                if (argument < 0) {
                    argument += DECK_SIZE;
                }

                auto it = std::copy(deck.begin() + argument, deck.end(), copy.begin());
                std::copy(deck.begin(), deck.begin() + argument, it);

                std::swap(deck, copy);
                break;
        }
    }

    auto location = std::find(deck.begin(), deck.end(), 2019);

    output << std::distance(deck.begin(), location) << std::endl;
}

void aoc2019::day22_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
