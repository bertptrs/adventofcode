#include <iostream>
#include "utils.hpp"

std::string_view aoc2019::strtok(std::string_view &str, char token) {
    auto next_delim = str.find(token);
    auto next = str.substr(0, next_delim);
    if (next_delim == std::string_view::npos) {
        str = {};
    } else {
        str = str.substr(next_delim + 1);
    }
    return next;
}

std::vector<int> aoc2019::run_intcode(std::vector<int> &program, std::deque<int> inputs) {
    std::vector<int> outputs{};
    int ip = 0;
    auto interpret_value = [&program, &ip](int pos) {
        bool immediate = false;
        switch (pos) {
            case 1:
                immediate = program[ip] / 100 % 10;
                break;
            case 2:
                immediate = program[ip] / 1000 % 10;
                break;

            case 3:
                immediate = program[ip] / 10000 % 10;
                break;

            default:
                throw std::out_of_range("Invalid position");
        }

        if (immediate) {
            return program[ip + pos];
        } else {
            return program[program[ip + pos]];
        }
    };

    while (ip < program.size()) {
        switch (program[ip] % 100) {
            case 1:
                program[program[ip + 3]] = interpret_value(1) + interpret_value(2);
                ip += 4;
                break;

            case 2:
                program[program[ip + 3]] = interpret_value(1) * interpret_value(2);
                ip += 4;
                break;

            case 3:
                if (inputs.empty()) {
                    throw std::out_of_range("No inputs left");
                }
                program[program[ip + 1]] = inputs.front();
                inputs.pop_front();
                ip += 2;
                break;

            case 4:
                outputs.push_back(interpret_value(1));
                ip += 2;
                break;

            case 5: // Jump if non-zero
                if (interpret_value(1)) {
                    ip = interpret_value(2);
                } else {
                    ip += 3;
                }
                break;

            case 6: // Jump if zero
                if (!interpret_value(1)) {
                    ip = interpret_value(2);
                } else {
                    ip += 3;
                }
                break;

            case 7: // equality
                program[program[ip + 3]] = interpret_value(1) < interpret_value(2);
                ip += 4;
                break;

            case 8: // less than
                program[program[ip + 3]] = interpret_value(1) == interpret_value(2) ? 1 : 0;
                ip += 4;
                break;

            case 99:
                return outputs;

            default:
                char buffer[30];
                std::snprintf(buffer, sizeof(buffer), "Invalid opcode: %d", program[ip]);

                throw std::domain_error(buffer);
        }
    }
    throw std::out_of_range("Program read out of bounds");
}

std::vector<int> aoc2019::read_intcode(std::istream &input) {
    std::vector<int> program{};
    for (int current; input >> current; input.ignore()) {
        program.push_back(current);
    }

    return program;
}
