#include <iostream>
#include "days.hpp"
#include "utils.hpp"

void aoc2019::day22_part1(std::istream &input, std::ostream &output) {
    std::int64_t increment = 1;
    std::int64_t offset = 0;

    constexpr std::int64_t DECK_SIZE = 10007;

    std::string buffer;
    while (std::getline(input, buffer)) {
        std::string_view line = buffer;
        if (!line.find("deal into new stack")) {
            increment *= -1;
        } else if (!line.find("deal with increment ")) {
            std::int64_t new_increment;
            from_chars(line.substr(20), new_increment);
            increment *= new_increment;
            increment %= DECK_SIZE;
        } else {
            std::int64_t new_offset;
            from_chars(line, new_offset);
            offset += increment * new_offset;
            offset %= DECK_SIZE;
        }
    }
	output << "Not implemented\n";
}

void aoc2019::day22_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
