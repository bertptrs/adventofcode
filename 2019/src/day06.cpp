#include <iostream>
#include <regex>
#include "days.hpp"

void aoc2019::day06_part1(std::istream &input, std::ostream &output) {
    std::string buffer;
    std::regex regex("^([A-Z0-9]+)\\)([A-Z0-9]+)$");

    while (std::getline(input, buffer)) {
        std::smatch match_results;

        if (!std::regex_match(buffer, match_results, regex)) {
            std::string error = "Invalid line: ";
            error += buffer;
            throw std::domain_error(error);
        }
    }
	output << "Not implemented\n";
}

void aoc2019::day06_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
