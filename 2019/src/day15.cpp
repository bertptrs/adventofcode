#include <iostream>
#include <cassert>
#include <unordered_set>
#include "days.hpp"
#include "utils.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    enum class Tile {
        Wall,
        Empty,
        Oxygen,
    };

    enum class Mark {
        None,
        Temp,
        Permanent,
    };

    const std::unordered_map<point_t, std::int64_t> DIRECTIONS{
            {{0,  -1}, 1},
            {{0,  1},  2},
            {{-1, 0},  3},
            {{1,  0},  4},
    };

    std::unordered_map<point_t, Tile> read_map(std::istream &input) {
        aoc2019::IntCodeComputer computer(input);
        std::deque<std::int64_t> output_buffer;
        computer.connectOutput(output_buffer);

        point_t pos = {0, 0};
        std::deque<point_t> prev;
        std::unordered_map<point_t, Tile> map{{pos, Tile::Empty}};
        std::unordered_map<point_t, Mark> markings{{pos, Mark::Temp}};

        computer.run();

        while (true) {
            std::optional<point_t> next_step;

            for (auto &direction : DIRECTIONS) {
                if (markings[pos + direction.first] == Mark::None) {
                    next_step = direction.first;
                    break;
                }
            }

            if (next_step) {
                const auto next_pos = pos + *next_step;
                computer.sendInput(DIRECTIONS.at(*next_step));
                computer.run();

                assert(!output_buffer.empty());

                switch (output_buffer.front()) {
                    case 0:
                        markings[next_pos] = Mark::Permanent;
                        map[next_pos] = Tile::Wall;
                        break;

                    case 1:
                    case 2:
                        prev.push_front(pos);
                        markings[next_pos] = Mark::Temp;
                        map[next_pos] = static_cast<Tile>(output_buffer.front());
                        pos = next_pos;
                        break;

                    default:
                        throw std::domain_error("Invalid data from remote");
                }
                output_buffer.pop_front();
                assert(output_buffer.empty());
            } else {
                markings[pos] = Mark::Permanent;
                // Nowhere left to go, move back.
                if (prev.empty()) {
                    return map;
                }

                auto prev_pos = prev.front();
                auto step = DIRECTIONS.at(prev_pos - pos);
                prev.pop_front();
                computer.sendInput(step);
                computer.run();
                // We should be able to travel back
                assert(output_buffer.front() == 1);
                output_buffer.pop_front();
                pos = prev_pos;
            }
        }
    }

    template<class Callback>
    int bfs(const std::unordered_map<point_t, Tile> &map, point_t starting_point, Callback callback) {
        std::deque<std::pair<point_t, int>> todo{{starting_point, 0}};
        std::unordered_set<point_t> visited{{0, 0}};

        int max_dist = 0;

        while (!todo.empty()) {
            auto[cur, dist] = todo.front();
            todo.pop_front();

            max_dist = std::max(max_dist, dist);

            for (auto &dir : DIRECTIONS) {
                auto new_pos = cur + dir.first;
                if (!visited.count(new_pos)) {
                    visited.insert(new_pos);

                    if (callback(map.at(new_pos))) {
                        return dist + 1;
                    }

                    switch (map.at(new_pos)) {
                        case Tile::Oxygen:
                        case Tile::Empty:
                            todo.emplace_back(new_pos, dist + 1);
                            break;

                        default:
                            break;
                    }
                }
            }
        }

        return max_dist;
    }
}

void aoc2019::day15_part1(std::istream &input, std::ostream &output) {
    const auto map = read_map(input);

    auto dist = bfs(map, {0, 0}, [](Tile x) { return x == Tile::Oxygen; });

    output << dist << std::endl;
}

void aoc2019::day15_part2(std::istream &input, std::ostream &output) {
    const auto map = read_map(input);

    auto starting_point = std::find_if(map.begin(), map.end(), [](auto x) { return x.second == Tile::Oxygen; })->first;

    auto dist = bfs(map, starting_point, [](Tile x) { return false; });

    output << dist << std::endl;
}
