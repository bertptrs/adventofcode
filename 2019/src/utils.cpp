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

std::deque<int64_t> aoc2019::run_intcode(std::vector<int64_t> program, std::deque<int64_t> inputs) {
    std::deque<std::int64_t> outputs;
    IntCodeComputer computer(std::move(program), std::move(inputs));
    computer.connectOutput(outputs);

    computer.run();

    return outputs;
}

aoc2019::IntCodeComputer::value_t &aoc2019::IntCodeComputer::interpret_value(int pos) {
    value_t immediate;
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

    value_t index;

    switch (immediate) {
        case 0:
            index = program[ip + pos];
            break;

        case 1:
            index = ip + pos;
            break;

        case 2:
            index = program[ip + pos] + relative;
            break;

        default:
            throw std::out_of_range("Invalid mode");
    }

    if (program.size() <= index) {
        program.resize(index + 1);
    }

    return program[index];
}

void aoc2019::IntCodeComputer::connectOutput(aoc2019::IntCodeComputer &computer) {
    outputSink = &computer.inputs;
}

void aoc2019::IntCodeComputer::connectOutput(std::deque<value_t> &sink) {
    outputSink = &sink;
}

bool aoc2019::IntCodeComputer::isTerminated() const {
    return halted;
}

const std::deque<aoc2019::IntCodeComputer::value_t> &aoc2019::IntCodeComputer::currentInputs() const {
    return inputs;
}

std::vector<aoc2019::IntCodeComputer::value_t> aoc2019::IntCodeComputer::read_intcode(std::istream &input) {
    std::vector<value_t> program;
    for (value_t current; input >> current; input.ignore()) {
        program.push_back(current);
    }

    return program;
}

void aoc2019::IntCodeComputer::run() {
    while (ip < program.size()) {
        switch (program[ip] % 100) {
            case 1:
                interpret_value(3) = interpret_value(1) + interpret_value(2);
                ip += 4;
                break;

            case 2:
                interpret_value(3) = interpret_value(1) * interpret_value(2);
                ip += 4;
                break;

            case 3:
                if (inputs.empty()) {
                    return;
                }

                interpret_value(1) = inputs.front();
                inputs.pop_front();
                ip += 2;
                break;

            case 4:
                outputSink->push_back(interpret_value(1));
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

            case 7: // less than
                interpret_value(3) = interpret_value(1) < interpret_value(2);
                ip += 4;
                break;

            case 8: // equality
                interpret_value(3) = interpret_value(1) == interpret_value(2) ? 1 : 0;
                ip += 4;
                break;

            case 9:
                relative += interpret_value(1);
                ip += 2;
                break;

            case 99:
                halted = true;
                return;

            default:
                char buffer[30];
                std::snprintf(buffer, sizeof(buffer), "Invalid opcode: %d", program[ip]);

                throw std::domain_error(buffer);
        }
    }
}

aoc2019::IntCodeComputer::IntCodeComputer(std::vector<value_t> program, std::deque<value_t> initial_inputs) :
        program{std::move(program)}, inputs{std::move(initial_inputs)} {
}


aoc2019::IntCodeComputer::IntCodeComputer(std::istream &program_stream, std::deque<value_t> initial_inputs) :
        program(read_intcode(program_stream)), inputs(std::move(initial_inputs)) {
}

void aoc2019::IntCodeComputer::sendInput(aoc2019::IntCodeComputer::value_t input) {
    inputs.push_back(input);
}

aoc2019::IntCodeComputer::value_t &aoc2019::IntCodeComputer::operator[](std::size_t index) {
    return program[index];
}

const aoc2019::IntCodeComputer::value_t &aoc2019::IntCodeComputer::operator[](std::size_t index) const {
    return program[index];
}
