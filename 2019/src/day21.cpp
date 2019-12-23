#include <iostream>
#include "days.hpp"
#include "utils.hpp"

namespace {
    void solve(std::istream &input, std::string_view program, std::ostream &output) {
        aoc2019::IntCodeComputer computer(input);
        std::deque<std::int64_t> output_buffer;
        computer.connectOutput(output_buffer);
        computer.run();
        output_buffer.clear();

        computer.sendInputs(program);

        computer.run();
        if (output_buffer.back() < 127) {
            for (char c : output_buffer) {
                output << c;
            }
        } else {
            output << output_buffer.back() << std::endl;
        }
    }
}

void aoc2019::day21_part1(std::istream &input, std::ostream &output) {
    std::string_view program = "OR A J\n"  // Check if any of the next 3 places is a hole
                               "AND B J\n"
                               "AND C J\n"
                               "NOT J J\n"
                               "AND D J\n" // Jump if the landing space is clear
                               "WALK\n";

    solve(input, program, output);
}

void aoc2019::day21_part2(std::istream &input, std::ostream &output) {
    std::string_view program = "NOT H J\n" // If you can safely jump twice
                               "OR C J\n"  // And either of the next 3 places contains a hole
                               "AND A J\n"
                               "AND B J\n"
                               "NOT J J\n"
                               "AND D J\n" // And we can land our first jump, then jump.
                               "RUN\n";

    solve(input, program, output);
}
