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

    int pos = 2019;

    for (auto move : read_moves(input)) {
        int argument = move.second;
        switch (move.first) {
            case Operation::Stack:
                pos = DECK_SIZE - 1 - pos;
                break;

            case Operation::Deal:
                pos = pos * argument % DECK_SIZE;
                break;

            case Operation::Cut:
                pos = (pos - argument) % DECK_SIZE;
                if (pos < 0) pos += DECK_SIZE;
                break;
        }
    }

    output << pos << std::endl;
}

void aoc2019::day22_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
