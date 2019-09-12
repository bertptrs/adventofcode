#include "solutions.hpp"
#include <iostream>
#include <charconv>
#include <cstring>

template<typename Int, int Radix = 10>
Int int_from_cstr(Int &value, const char *begin) {
    const char *end = begin + std::strlen(begin);
    auto result = std::from_chars(begin, end, value, Radix);
    if (result.ec != std::errc()) {
        throw std::invalid_argument("Unparseable integer");
    }

    return value;
}

int main(int argc, const char *argv[]) {
    if (argc < 2) {
        std::cerr << "Specify a day to run.\n";
        return 1;
    }

    int day;
    int_from_cstr(day, argv[1]);

    const aoc2019::solution_t solution = aoc2019::get_implementation(day);
    if (solution != nullptr) {
        solution(std::cin, std::cout);
        return 0;
    } else {
        std::cerr << "Unimplemented.\n";
        return 1;
    }
}
