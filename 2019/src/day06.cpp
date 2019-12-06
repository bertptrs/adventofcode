#include <deque>
#include <iostream>
#include <unordered_map>
#include <vector>
#include "days.hpp"

namespace {
    std::vector<std::pair<std::string, std::string>> read_orbits(std::istream &input) {
        std::vector<std::pair<std::string, std::string>> result;
        std::string name1, name2;

        while (std::getline(input, name1, ')')) {
            std::getline(input, name2);

            result.emplace_back(name1, name2);
        }

        return result;
    }
}

void aoc2019::day06_part1(std::istream &input, std::ostream &output) {
    std::unordered_map<std::string, std::vector<std::string>> orbits;

    for (auto[a, b] : read_orbits(input)) {
        orbits[std::move(a)].emplace_back(std::move(b));
    }

    std::deque<std::pair<std::string, int>> todo = {{"COM", 0}};
    int total_orbits = 0;

    while (!todo.empty()) {
        auto[name, offset] = todo.front();
        todo.pop_front();

        total_orbits += offset;

        for (const auto& partner : orbits[name]) {
            todo.emplace_back(partner, offset + 1);
        }
    }

    output << total_orbits << std::endl;
}

void aoc2019::day06_part2(std::istream &input, std::ostream &output) {
    std::unordered_map<std::string, std::string> ancestors;

    for (auto[a, b] : read_orbits(input)) {
        ancestors[std::move(b)] = std::move(a);
    }

    std::unordered_map<std::string, int> santa_ancestors;

    for (auto current = ancestors["SAN"]; current != "COM"; current = ancestors[current]) {
        santa_ancestors[ancestors[current]] = santa_ancestors[current] + 1;
    }

    int dist = 0;
    for (auto current = ancestors["YOU"]; current != "COM"; current = ancestors[current], ++dist) {
        if (auto it = santa_ancestors.find(current); it != santa_ancestors.end()) {
            output << dist + it->second << std::endl;
            return;
        }
    }

    throw std::domain_error("No valid path.");
}
