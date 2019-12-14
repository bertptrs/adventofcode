#include <gtest/gtest.h>
#include "utils.hpp"

using aoc2019::run_intcode;
using aoc2019::IntCodeComputer;

auto run_program(std::vector<int64_t> program, std::deque<int64_t> input) {
    std::deque<std::int64_t> output;
    IntCodeComputer computer(std::move(program), std::move(input));
    computer.connectOutput(output);
    computer.run();
    return output;
}

TEST(Intcode, TestPositionEquality) {
    const std::vector<int64_t> program = {3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8};

    ASSERT_EQ(1, run_program(program, {8}).front());
    ASSERT_EQ(0, run_program(program, {9}).front());
}

TEST(Intcode, TestPositionLess) {
    const std::vector<int64_t> program = {3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8};

    ASSERT_EQ(1, run_program(program, {7}).front());
    ASSERT_EQ(0, run_program(program, {9}).front());
}

TEST(Intcode, TestImmediateEquality) {
    const std::vector<int64_t> program = {3, 3, 1108, -1, 8, 3, 4, 3, 99};

    ASSERT_EQ(1, run_program(program, {8}).front());
    ASSERT_EQ(0, run_program(program, {9}).front());
}

TEST(Intcode, TestImmediateLess) {
    const std::vector<int64_t> program = {3, 3, 1107, -1, 8, 3, 4, 3, 99};

    ASSERT_EQ(1, run_program(program, {7}).front());
    ASSERT_EQ(0, run_program(program, {9}).front());
}

TEST(Intcode, TestComplicatedConditional) {
    const std::vector<std::int64_t> program = {3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                                      1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                                      999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99};
    auto pcopy = program;

    auto output = run_intcode(pcopy, {7});
    ASSERT_EQ(999, output.front());

    pcopy = program;
    output = run_intcode(pcopy, {9});
    ASSERT_EQ(1001, output.front());

    pcopy = program;
    output = run_intcode(pcopy, {8});
    ASSERT_EQ(1000, output.front());
}
