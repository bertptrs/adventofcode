#include <array>
#include <iostream>
#include <memory>
#include "days.hpp"
#include "utils.hpp"

namespace {
    class IntCodeComputer {
    public:
        explicit IntCodeComputer(std::vector<int> program, std::deque<int> initial_inputs) :
                program{std::move(program)}, inputs{std::move(initial_inputs)} {
        }

        void run() {
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
                            return;
                        }

                        program[program[ip + 1]] = inputs.front();
                        inputs.pop_front();
                        ip += 2;
                        break;

                    case 4:
                        outputSink->sendInput(interpret_value(1));
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
                        halted = true;
                        return;

                    default:
                        char buffer[30];
                        std::snprintf(buffer, sizeof(buffer), "Invalid opcode: %d", program[ip]);

                        throw std::domain_error(buffer);
                }
            }
        }

        void sendInput(int input) {
            inputs.push_back(input);
        }

        void connectOutput(IntCodeComputer &computer) {
            outputSink = &computer;
        }

        [[nodiscard]] bool isTerminated() const {
            return halted;
        }

        [[nodiscard]] const std::deque<int> &currentInputs() const {
            return inputs;
        }

    private:
        std::vector<int> program;
        std::deque<int> inputs = {};
        IntCodeComputer *outputSink = nullptr;
        int ip = 0;
        bool halted = false;

        [[nodiscard]] int interpret_value(int pos) const {
            bool immediate;
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
        }
    };

    int simulate(const std::vector<int> &program, const std::array<int, 5> &phases) {
        int state = 0;
        for (int phase : phases) {
            auto copy = program;
            auto result = aoc2019::run_intcode(copy, {phase, state});
            state = result.front();
        }

        return state;
    }

    int simulate2(const std::vector<int> &program, const std::array<int, 5> &phases) {
        std::vector<IntCodeComputer> computers;
        for (int phase : phases) {
            computers.emplace_back(program, std::deque<int>{phase});
        }

        for (int i = 0; i < computers.size(); ++i) {
            computers[i].connectOutput(computers[(i + 1) % 5]);
        }

        computers[0].sendInput(0);

        while (std::any_of(computers.begin(), computers.end(), [](const auto &c) { return !c.isTerminated();})) {
            for (auto& computer : computers) {
                computer.run();
            }
        }

        return computers[0].currentInputs().back();
    }
}

void aoc2019::day07_part1(std::istream &input, std::ostream &output) {
    const auto program = aoc2019::read_intcode(input);
    std::array<int, 5> phases{0, 1, 2, 3, 4};

    int best = 0;

    do {
        best = std::max(simulate(program, phases), best);
    } while (std::next_permutation(phases.begin(), phases.end()));

    output << best << std::endl;
}

void aoc2019::day07_part2(std::istream &input, std::ostream &output) {
    const auto program = aoc2019::read_intcode(input);
    std::array<int, 5> phases{5, 6, 7, 8, 9};

    int best = 0;

    do {
        best = std::max(simulate2(program, phases), best);
    } while (std::next_permutation(phases.begin(), phases.end()));

    output << best << std::endl;
}
