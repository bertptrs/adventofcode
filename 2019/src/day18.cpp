#include <iostream>
#include <vector>
#include <queue>
#include <unordered_map>
#include <map>
#include <numeric>
#include <algorithm>
#include <cctype>
#include "days.hpp"
#include "point.hpp"

namespace {
    typedef std::tuple<int, int, std::string> state_t;

    std::vector<std::string> read_map(std::istream &input) {
        std::string buffer;
        std::vector<std::string> map;

        while (std::getline(input, buffer)) {
            map.push_back(buffer);
        }

        return map;
    }

    std::pair<int, int> find(const std::vector<std::string> &map, char needle) {
        for (int y = 0; y < map.size(); ++y) {
            auto x = map[y].find(needle);
            if (x != std::string::npos) {
                return {(int) x, y};
            }
        }

        throw std::invalid_argument("Can't find it!");
    }
}

void aoc2019::day18_part1(std::istream &input, std::ostream &output) {
    const auto map = read_map(input);

    std::priority_queue<std::pair<int, state_t>, std::vector<std::pair<int, state_t>>, std::greater<>> todo;
    std::map<state_t, int> visited;

    const auto initial = find(map, '@');
    const state_t initial_state(initial.first, initial.second, "");
    visited[initial_state] = 0;
    todo.emplace(0, initial_state);

    int keys_needed = 0;
    for (auto &row : map) {
        keys_needed += std::count_if(row.begin(), row.end(), isalpha);
    }

    // Don't count keys and locks double
    keys_needed /= 2;

    while (!todo.empty()) {
        const auto[dist, state] = todo.top();
        todo.pop();

        const auto[x, y, keys] = state;

        if (keys.size() == keys_needed) {
            output << dist << std::endl;
            return;
        }

        std::array<std::pair<int, int>, 4> next_points = {{
                {x - 1, y},
                {x + 1, y},
                {x, y - 1},
                {x, y + 1}
        }};

        for (auto point : next_points) {
            const auto [nx, ny] = point;
            auto next_keys = keys;

            if (x < 0 || y < 0 || x >= map[0].size() || y >= map.size()) {
                continue;
            }

            char at_next = map[x][y];

            if (at_next == '#') {
                continue;
            } else if (std::isupper(at_next)) {
                // check if we have the key already
                if (keys.find(at_next) == std::string::npos) {
                    continue;
                }
            } else if (std::islower(at_next)) {
                if (keys.find(std::toupper(at_next)) == std::string::npos) {
                    next_keys += std::toupper(at_next);
                    // Ensure unique representation
                    std::sort(next_keys.begin(), next_keys.end());
                }
            }

            state_t next_state{nx, ny, std::move(next_keys)};
            if (auto it = visited.find(next_state); it == visited.end() || it->second < dist + 1) {
                visited[next_state] = dist + 1;
                todo.emplace(dist + 1, std::move(next_state));
            }
        }
    }

	output << "Not implemented\n";
}

void aoc2019::day18_part2(std::istream &input, std::ostream &output) {
	output << "Not implemented\n";
}
