#include <iostream>
#include <regex>
#include <queue>
#include <unordered_set>
#include "days.hpp"

namespace {
    std::vector<std::pair<std::string, std::string>> read_orbits(std::istream &input) {
        std::vector<std::pair<std::string, std::string>> result;
        std::string buffer;

        std::regex regex("^([A-Z0-9]+)\\)([A-Z0-9]+)$");
        std::smatch match_results;

        while (std::getline(input, buffer)) {
            if (!std::regex_match(buffer, match_results, regex)) {
                std::string error = "Invalid line: ";
                error += buffer;
                throw std::domain_error(error);
            }

            result.emplace_back(match_results[1], match_results[2]);
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
    std::unordered_map<std::string, std::vector<std::string>> orbits;

    for (auto[a, b] : read_orbits(input)) {
        orbits[a].emplace_back(b);
        orbits[b].emplace_back(a);
    }

    std::deque<std::pair<std::string, int>> todo = {{"YOU", 0}};
    std::unordered_set<std::string> visited = { "YOU" };

    while (!todo.empty()) {
        auto[name, offset] = todo.front();
        todo.pop_front();

        for (const auto& partner : orbits[name]) {
            if (partner == "SAN") {
                output << offset - 1 << std::endl;
                return;
            }
            if (!visited.count(partner)) {
                todo.emplace_back(partner, offset + 1);
                visited.emplace(partner);
            }
        }
    }

    throw std::domain_error("No valid path.");
}
