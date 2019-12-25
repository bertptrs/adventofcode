#include <iostream>
#include <vector>
#include <map>
#include <unordered_set>
#include <queue>
#include "days.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;
    typedef std::vector<std::string> map_t;

    std::array<point_t, 4> DIRECTIONS = {{
                                                 {0, -1},
                                                 {0, 1},
                                                 {-1, 0},
                                                 {1, 0},
                                         }};

    std::vector<std::string> read_map(std::istream &input) {
        std::string buffer;
        std::vector<std::string> map;

        while (std::getline(input, buffer)) {
            map.push_back(buffer);
        }

        return map;
    }

    auto get_portals(const map_t &map) {
        std::unordered_map<point_t, std::string> portals;

        // First find horizontal portals
        for (int y = 0; y < map.size(); ++y) {
            for (int x = 0; x < map[y].size() - 1; ++x) {
                if (std::isalpha(map[y][x]) && std::isalpha(map[y][x + 1])) {
                    // find out the entry point
                    point_t entry_point = {0, y};
                    if (x > 0 && map[y][x - 1] == '.') {
                        entry_point[0] = x - 1;
                    } else {
                        entry_point[0] = x + 2;
                    }

                    portals[entry_point] = map[y].substr(x, 2);
                }
            }
        }

        char name[3] = {0, 0, 0};
        for (int x = 0; x < map[0].size(); ++x) {
            for (int y = 0; y < map.size() - 1; ++y) {
                if (std::isalpha(map[y][x]) && std::isalpha(map[y + 1][x])) {
                    name[0] = map[y][x];
                    name[1] = map[y + 1][x];

                    point_t entry_point = {x, 0};
                    if (y > 0 && map[y - 1][x] == '.') {
                        entry_point[1] = y - 1;
                    } else {
                        entry_point[1] = y + 2;
                    }

                    portals[entry_point] = name;
                }
            }
        }

        return portals;
    }

    std::unordered_map<point_t, std::vector<std::pair<int, point_t>>>
    get_implicit_graph(const map_t &map, const std::unordered_map<point_t, std::string> &portals) {
        std::unordered_map<std::string, point_t> half_links;

        std::unordered_map<point_t, std::vector<std::pair<int, point_t>>> graph;

        for (auto &entry : portals) {
            if (auto it = half_links.find(entry.second); it != half_links.end()) {
                // Connect up the portals
                graph[it->second].emplace_back(1, entry.first);
                graph[entry.first].emplace_back(1, it->second);
            } else {
                half_links[entry.second] = entry.first;
            }

            // Do a BFS from the node to see what we can reach.
            std::deque<std::pair<int, point_t>> todo{{0, entry.first}};
            std::unordered_set<point_t> visited{entry.first};

            while (!todo.empty()) {
                const auto[dist, pos] = todo.front();
                todo.pop_front();

                for (auto &direction : DIRECTIONS) {
                    auto next_pos = pos + direction;

                    if (map[next_pos[1]][next_pos[0]] != '.' || visited.count(next_pos)) {
                        continue;
                    }

                    if (portals.count(next_pos)) {
                        graph[entry.first].emplace_back(dist + 1, next_pos);
                    }

                    todo.emplace_back(dist + 1, next_pos);
                    visited.insert(next_pos);
                }
            }
        }

        return graph;
    }
}

void aoc2019::day20_part1(std::istream &input, std::ostream &output) {
    const auto map = read_map(input);
    const auto portals = get_portals(map);

    const auto starting_point = std::find_if(portals.begin(), portals.end(), [](auto &x) {
        return x.second == "AA";
    })->first;

    auto graph = get_implicit_graph(map, portals);

    std::unordered_set<point_t> visited;
    std::priority_queue<std::pair<int, point_t>, std::vector<std::pair<int, point_t>>, std::greater<>> todo;
    todo.emplace(0, starting_point);

    while (!todo.empty()) {
        const auto[dist, pos] = todo.top();
        todo.pop();

        if (visited.count(pos)) {
            continue;
        }

        visited.insert(pos);

        if (portals.at(pos) == "ZZ") {
            output << dist << std::endl;
            return;
        }

        for (auto &edge : graph[pos]) {
            if (visited.count(edge.second)) {
                continue;
            }

            todo.emplace(dist + edge.first, edge.second);
        }
    }

    throw std::domain_error("No valid route.");
}

void aoc2019::day20_part2(std::istream &input, std::ostream &output) {
    output << "Not implemented\n";
}
