#include <iostream>
#include <cstdint>
#include <vector>
#include <map>
#include <unordered_map>
#include <algorithm>
#include <regex>
#include <charconv>
#include "days.hpp"

namespace {
    typedef std::pair<std::string, std::int64_t> requirement_t;
    typedef std::vector<requirement_t> reqlist_t;

    std::map<reqlist_t, reqlist_t> read_recipes(std::istream &input) {
        std::map<reqlist_t, reqlist_t> recipes;

        std::string buffer;
        std::regex listing_regex("(\\d+) ([A-Z]+)");

        std::int64_t amount;

        while (std::getline(input, buffer)) {
            reqlist_t requirements, production;

            std::string_view line = buffer;

            auto split_point = line.find(" => ");

            auto requirements_part = line.substr(0, split_point);
            auto production_part = line.substr(split_point + 4);

            for (auto it = std::regex_token_iterator(requirements_part.begin(), requirements_part.end(), listing_regex,
                                                     {1, 2}); it != std::cregex_token_iterator(); ++it) {
                std::from_chars(it->first, it->second, amount);
                ++it;

                requirements.emplace_back(*it, amount);
            }

            for (auto it = std::regex_token_iterator(production_part.begin(), production_part.end(), listing_regex,
                                                     {1, 2}); it != std::cregex_token_iterator(); ++it) {
                std::from_chars(it->first, it->second, amount);
                ++it;

                production.emplace_back(*it, amount);
            }

            recipes[std::move(production)] = std::move(requirements);
        }

        return recipes;
    }

    template<class Map>
    std::unordered_map<std::string, reqlist_t> element_creators(const Map &map) {
        std::unordered_map<std::string, reqlist_t> inverted;

        for (auto &entry : map) {
            for (auto &x : entry.first) {
                inverted[x.first] = entry.first;
            }
        }

        return inverted;
    }

    std::vector<std::string> topological_order(const std::map<reqlist_t, reqlist_t> &recipes) {
        std::vector<std::string> order;

        std::unordered_map<std::string_view, std::vector<std::string>> edges;
        for (auto &entry : recipes) {
            for (auto &production : entry.first) {
                std::transform(entry.second.begin(), entry.second.end(), std::back_inserter(edges[production.first]),
                               [](const auto &x) {
                                   return x.first;
                               });
            }
        }

        std::unordered_map<std::string_view, int> incoming_edge_count;
        for (const auto &entry : edges) {
            for (const auto &parent : entry.second) {
                incoming_edge_count[parent]++;
            }
        }

        std::deque<std::string_view> childless{"FUEL"};

        while (!childless.empty()) {
            auto current = childless.front();
            childless.pop_front();
            order.emplace_back(current);

            for (const auto &parent : edges[current]) {
                if (--incoming_edge_count[parent] == 0) {
                    childless.push_back(parent);
                }
            }
        }

        return order;
    }

    std::int64_t ore_to_fuel(const std::map<reqlist_t, reqlist_t> &recipes, std::int64_t amount = 1) {
        auto inverted = element_creators(recipes);
        auto order = topological_order(recipes);

        std::unordered_map<std::string_view, std::int64_t> total_requirements;
        total_requirements["FUEL"] = amount;

        for (const auto &element : order) {
            if (element == "ORE") {
                break;
            }

            const auto number_required = total_requirements[element];
            if (number_required <= 0) {
                continue;
            }

            const auto &productions = inverted.at(element);
            const auto &requirements = recipes.at(productions);

            auto number_produced = std::find_if(productions.begin(), productions.end(),
                                                [element](const auto &x) { return x.first == element; })->second;

            auto productions_needed = number_required / number_produced + (number_required % number_produced ? 1 : 0);

            for (auto &requirement : requirements) {
                total_requirements[requirement.first] += requirement.second * productions_needed;
            }

            for (auto &production : productions) {
                total_requirements[production.first] -= productions_needed * production.second;
            }
        }

        return total_requirements["ORE"];
    }
}

void aoc2019::day14_part1(std::istream &input, std::ostream &output) {
    auto recipes = read_recipes(input);

    output << ore_to_fuel(recipes) << std::endl;
}

void aoc2019::day14_part2(std::istream &input, std::ostream &output) {
    auto recipes = read_recipes(input);

    constexpr std::int64_t ore_stock = 1000000000000;

    std::int64_t min = 1, max = ore_stock + 1; // assumption: 1 ore produces < 1 fuel.
    while (max - min > 1) {
        auto cur = (max + min) / 2;

        if (ore_to_fuel(recipes, cur) < ore_stock) {
            min = cur;
        } else {
            max = cur - 1;
        }
    }

    output << (max + min) / 2 << std::endl;
}
