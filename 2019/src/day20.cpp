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

    std::pair<std::unordered_map<std::string, point_t>, std::unordered_map<std::string, std::pair<point_t, point_t>>>
    get_portals(const map_t &map) {
        std::unordered_map<std::string, point_t> half_portals;
        std::unordered_map<std::string, std::pair<point_t, point_t>> portals;

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

                    auto name = map[y].substr(x, 2);

                    if (auto it = half_portals.find(name); it != half_portals.end()) {
                        portals[name] = {entry_point, it->second};
                    } else {
                        half_portals[name] = entry_point;
                    }
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

                    if (auto it = half_portals.find(name); it != half_portals.end()) {
                        portals[name] = {entry_point, it->second};
                    } else {
                        half_portals[name] = entry_point;
                    }
                }
            }
        }

        return std::make_pair(half_portals, portals);
    }
}

void aoc2019::day20_part1(std::istream &input, std::ostream &output) {
    const auto map = read_map(input);
    auto[half_portals, portals] = get_portals(map);

    const auto starting_point = half_portals.at("AA");
    const auto goal = half_portals.at("ZZ");

    std::unordered_map<point_t, point_t> links;
    for (auto &link : portals) {
        links[link.second.first] = link.second.second;
        links[link.second.second] = link.second.first;
    }

    std::unordered_set<point_t> visited{starting_point};
    std::queue<std::pair<int, point_t>> todo;
    todo.emplace(0, starting_point);

    while (!todo.empty()) {
        const auto[dist, pos] = todo.front();
        todo.pop();

        if (pos == goal) {
            output << dist << std::endl;
            return;
        }

        auto enqueue_point = [&visited, &todo, dist](point_t p) {
            if (visited.count(p)) {
                return;
            }

            visited.insert(p);
            todo.emplace(dist + 1, p);
        };

        for (auto &direction : DIRECTIONS) {
            auto next_point = pos + direction;
            if (map[next_point[1]][next_point[0]] == '.') {
                enqueue_point(pos + direction);
            }
        }

        if (auto it = links.find(pos); it != links.end()) {
            // take portal
            enqueue_point(it->second);
        }
    }

    output << "Not implemented\n";
}

void aoc2019::day20_part2(std::istream &input, std::ostream &output) {
    output << "Not implemented\n";
}
