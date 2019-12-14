#include <iostream>
#include <deque>
#include "days.hpp"
#include "utils.hpp"

void aoc2019::day09_part1(std::istream &input, std::ostream &output) {
    std::deque<std::int64_t> outputs;

    IntCodeComputer computer(input, { 1 });
    computer.connectOutput(outputs);

    computer.run();

    if (outputs.size() != 1) {
        std::cerr << "Error: " << outputs.size() << std::endl;
        for (auto c : outputs) {
            std::cerr << c << std::endl;
        }
    } else {
        output << outputs.front() << std::endl;
    }
}

void aoc2019::day09_part2(std::istream &input, std::ostream &output) {
    std::deque<std::int64_t> outputs;

    IntCodeComputer computer(input, { 2 });
    computer.connectOutput(outputs);

    computer.run();

    output << outputs.front() << std::endl;
}
