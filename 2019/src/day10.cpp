#include <algorithm>
#include <iostream>
#include <numeric>
#include <unordered_set>
#include <cmath>
#include "days.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int, 2> point_t;

    std::vector<point_t> read_points(std::istream &input) {
        std::vector<point_t> result;

        int y = 0;

        for (std::string buffer; std::getline(input, buffer); ++y) {
            std::size_t x = 0;

            while ((x = buffer.find('#', x)) != std::string::npos) {
                result.push_back({(int) x, y});
                x += 1;
            }
        }

        return result;
    }

    point_t simplify(point_t x) {
        auto gcd = std::abs(std::gcd(x[0], x[1]));
        if (gcd > 1) {
            return {x[0] / gcd, x[1] / gcd};
        }

        return x;
    }

    std::pair<std::size_t, std::size_t> part1(const std::vector<point_t> &points) {
        std::size_t best = 0;
        std::size_t best_index = 0;
        std::unordered_set<point_t> visible;

        for (std::size_t i = 0; i < points.size(); ++i) {
            visible.clear();

            const auto point = points[i];

            for (auto asteroid : points) {
                if (asteroid == point) continue;
                visible.insert(simplify(asteroid - point));
            }

            if (visible.size() > best) {
                best = visible.size();
                best_index = i;
            }

            best = std::max(visible.size(), best);
        }

        return {best, best_index};
    }
}

void aoc2019::day10_part1(std::istream &input, std::ostream &output) {
    const auto points = read_points(input);

    auto[best, _] = part1(points);

    output << best << std::endl;
}

void aoc2019::day10_part2(std::istream &input, std::ostream &output) {
    const auto points = read_points(input);
    const auto[_, base] = part1(points);
    const auto base_point = points[base];

    std::unordered_map<point_t, std::vector<point_t>> angle_points;

    for (auto point : points) {
        if (point == base_point) continue;
        auto diff = point - base_point;

        angle_points[simplify(diff)].push_back(diff);
    }

    std::vector<std::pair<float, point_t>> angles;

    for (auto &entry : angle_points) {
        angles.emplace_back(std::atan2(entry.first[1], entry.first[0]), entry.first);
        // Sort entries in descending order of distance so we can pop_back() them
        std::sort(entry.second.begin(), entry.second.end(), [](auto a, auto b) { return a.l1() > b.l1(); });
    }

    std::sort(angles.begin(), angles.end(), std::greater<>{});

    const auto starting_point = std::make_pair(float(0.5 * M_PI),
                                               point_t{std::numeric_limits<int>::max(),
                                                       std::numeric_limits<int>::max()});

    auto it = std::lower_bound(angles.begin(), angles.end(), starting_point, std::greater<>{});

    for (int hits = 0; hits < 199; ++hits) {
        angle_points[it->second].pop_back();

        // Advance it to the next asteroid we can hit.
        while (angle_points[it->second].empty()) {
            ++it;
            if (it == angles.end()) {
                it = angles.begin();
            }
        }
    }

    auto final_asteroid = angle_points[it->second].back() + base_point;

    output << final_asteroid[0] * 100 + final_asteroid[1] << std::endl;
}
