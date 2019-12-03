#pragma once

#include <functional>
#include <string_view>

namespace aoc2019 {

    template<typename T>
    void combine_hash(std::size_t& seed, const T& o) {
        // Algorithm taken from boost::combine_hash.
        std::hash<T> hash{};
        seed ^= hash(o) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    }

    std::string_view strtok(std::string_view &str, char token = ',');
}
