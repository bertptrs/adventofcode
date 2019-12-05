#pragma once

#include <charconv>
#include <deque>
#include <functional>
#include <iosfwd>
#include <string_view>
#include <vector>

namespace aoc2019 {

    template<typename T>
    inline std::from_chars_result from_chars(std::string_view str, T &value) {
        return std::from_chars(str.data(), str.data() + str.size(), value);
    }

    template<typename T>
    void combine_hash(std::size_t &seed, const T &o) {
        // Algorithm taken from boost::combine_hash.
        std::hash<T> hash{};
        seed ^= hash(o) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    }

    std::string_view strtok(std::string_view &str, char token = ',');

    std::vector<int> read_intcode(std::istream &input);

    std::vector<int> run_intcode(std::vector<int> &program, std::deque<int> inputs = {});
}
