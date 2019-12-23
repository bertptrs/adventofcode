#include <iostream>
#include <vector>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <bit>
#include <map>
#include "days.hpp"
#include "point.hpp"

static_assert(sizeof(int) >= 4, "Int should be at least 32 bits.");

using namespace std::string_view_literals;

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    typedef std::vector<std::string> map_t;

    std::array<point_t, 4> DIRECTIONS = {{
                                                 {0, -1},
                                                 {0, 1},
                                                 {-1, 0},
                                                 {1, 0},
                                         }};

    map_t read_map(std::istream &input) {
        std::string buffer;
        map_t map;

        while (std::getline(input, buffer)) {
            map.push_back(buffer);
        }

        return map;
    }

    point_t find(const std::vector<std::string> &map, char needle) {
        for (int y = 0; y < map.size(); ++y) {
            auto x = map[y].find(needle);
            if (x != std::string::npos) {
                return {(int) x, y};
            }
        }

        throw std::invalid_argument("Can't find it!");
    }

    std::vector<std::pair<char, int>> find_edges(const map_t &map, point_t starting_point) {
        std::vector<std::pair<char, int>> edges;
        std::queue<std::pair<int, point_t>> todo;
        todo.emplace(0, starting_point);

        std::unordered_set<point_t> visited{starting_point};

        while (!todo.empty()) {
            const auto[dist, pos] = todo.front();
            todo.pop();

            for (auto &direction : DIRECTIONS) {
                auto next_pos = pos + direction;
                const char at = map[next_pos[1]][next_pos[0]];

                if (at == '#' || visited.count(next_pos)) {
                    // Wall or already visited, ignore
                    continue;
                }

                visited.insert(next_pos);

                if (std::isalpha(at)) {
                    // Don't walk through stuff
                    edges.emplace_back(at, dist + 1);
                } else {
                    todo.emplace(dist + 1, next_pos);
                }
            }
        }

        return edges;
    }

    auto compute_implied_graph(const map_t &map) {
        std::unordered_map<char, std::vector<std::pair<char, int>>> implied_graph;

        for (int y = 0; y < map.size(); ++y) {
            for (int x = 0; x < map[y].size(); ++x) {
                char at = map[y][x];
                if ("@/^*"sv.find(at) != std::string_view::npos || std::isalpha(at)) {
                    implied_graph[at] = find_edges(map, {x, y});
                }
            }
        }

        return implied_graph;
    }

    inline unsigned int key_index(char c) {
        return 1u << static_cast<unsigned int>(c - 'A');
    }
}

void aoc2019::day18_part1(std::istream &input, std::ostream &output) {
    using state_t = std::tuple<unsigned int, char>;

    const auto map = read_map(input);

    auto implied_graph = compute_implied_graph(map);

    std::priority_queue<std::pair<int, state_t>, std::vector<std::pair<int, state_t>>, std::greater<>> todo;
    std::map<state_t, int> visited;
    todo.emplace(0, std::make_pair(0, '@'));

    auto target_size = std::count_if(implied_graph.cbegin(), implied_graph.cend(),
                                     [](auto &x) { return std::islower(x.first); });

    while (!todo.empty()) {
        const auto[dist, state] = todo.top();
        todo.pop();

        if (visited[state] < dist) {
            continue;
        }

        auto[keys, pos] = state;

        if (std::__popcount(keys) == target_size) {
            output << dist << std::endl;
            return;
        }

        for (const auto &edge : implied_graph.at(pos)) {
            auto next_dist = dist + edge.second;
            auto next_keys = keys;
            if (std::islower(edge.first)) {
                // Add the key to our collection
                next_keys |= key_index(edge.first);;
            } else if (std::isupper(edge.first)) {
                // Check if we have the required key already
                if (!(next_keys & key_index(edge.first))) {
                    continue;
                }
            }

            state_t next_state = {next_keys, edge.first};
            if (auto it = visited.find(next_state); it == visited.end() || it->second > next_dist) {
                visited[next_state] = next_dist;
                todo.emplace(next_dist, next_state);
            }

        }
    }

    throw std::logic_error("Should have terminated by now.");
}

void aoc2019::day18_part2(std::istream &input, std::ostream &output) {
    using state_t = std::tuple<unsigned int, std::array<char, 4>>;

    auto map = read_map(input);

    // problem statement says to duplicate @ but where's the fun in that
    const auto initial_pos = find(map, '@');

    // problem statement says to duplicate @ but where's the fun in that, let's have different starting positions
    std::array<std::string_view, 3> overlay = {
            "@#*",
            "###",
            "^#/",
    };

    for (int y = 0; y < 3; ++y) {
        auto &row = map[initial_pos[1] + y - 1];
        std::copy(overlay[y].begin(), overlay[y].end(), row.data() + initial_pos[0] - 1);
    }

    const auto implied_graph = compute_implied_graph(map);

    std::priority_queue<std::pair<int, state_t>, std::vector<std::pair<int, state_t>>, std::greater<>> todo;
    std::map<state_t, int> visited;
    todo.emplace(0, state_t(0, {'@', '*', '^', '/'}));

    auto target_size = std::count_if(implied_graph.cbegin(), implied_graph.cend(),
                                     [](auto &x) { return std::islower(x.first); });


    while (!todo.empty()) {
        const auto[dist, state] = todo.top();
        todo.pop();

        if (visited[state] < dist) {
            continue;
        }

        auto[keys, pos] = state;

        if (std::__popcount(keys) == target_size) {
            output << dist << std::endl;
            return;
        }

        for (int i = 0; i < 4; ++i) {
            auto next_pos = pos;
            for (const auto &edge : implied_graph.at(pos[i])) {
                auto next_dist = dist + edge.second;
                auto next_keys = keys;
                if (std::islower(edge.first)) {
                    // Add the key to our collection
                    next_keys |= key_index(edge.first);;
                } else if (std::isupper(edge.first)) {
                    // Check if we have the required key already
                    if (!(next_keys & key_index(edge.first))) {
                        continue;
                    }
                }

                next_pos[i] = edge.first;

                state_t next_state = {next_keys, next_pos};
                if (auto it = visited.find(next_state); it == visited.end() || it->second > next_dist) {
                    visited[next_state] = next_dist;
                    todo.emplace(next_dist, next_state);
                }
            }
        }
    }


    output << "Not implemented\n";
}
