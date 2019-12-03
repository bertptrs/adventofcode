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
