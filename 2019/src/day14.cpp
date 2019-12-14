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

            for(auto it = std::regex_token_iterator(requirements_part.begin(), requirements_part.end(), listing_regex, {1, 2}); it != std::cregex_token_iterator(); ++it) {
                std::from_chars(it->first, it->second, amount);
                ++it;

                requirements.emplace_back(*it, amount);
            }

            for(auto it = std::regex_token_iterator(production_part.begin(), production_part.end(), listing_regex, {1, 2}); it != std::cregex_token_iterator(); ++it) {
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

    std::int64_t
    ore_required(const std::string &element, std::int64_t amount, std::unordered_map<std::string, std::int64_t> &stock,
                 const std::map<reqlist_t, reqlist_t> &recipes, std::unordered_map<std::string, reqlist_t> inverted) {
        if (element == "ORE") {
            return amount;
        }

        if (stock[element] > 0) {
            auto from_stock = std::min(amount, stock[element]);
            amount -= from_stock;
            stock[element] -= from_stock;
        }

        auto &productions = inverted.at(element);
        auto &requirements = recipes.at(productions);

        auto number_produced = std::find_if(productions.begin(), productions.end(),
                                            [element](const auto &x) { return x.first == element; })->second;

        auto productions_needed = amount / number_produced + (amount % number_produced ? 1 : 0);

        std::int64_t ore_needed = 0;

        for (auto &requirement : requirements) {
            ore_needed += ore_required(requirement.first, requirement.second * productions_needed, stock, recipes,
                                       inverted);
        }

        for (auto &production : productions) {
            stock[production.first] += productions_needed * production.second;
        }

        stock[element] -= amount;

        return ore_needed;
    }
}

void aoc2019::day14_part1(std::istream &input, std::ostream &output) {
    auto recipes = read_recipes(input);
    auto inverted = element_creators(recipes);
    std::unordered_map<std::string, std::int64_t> stock;

    output << ore_required("FUEL", 1, stock, recipes, inverted) << std::endl;
}

void aoc2019::day14_part2(std::istream &input, std::ostream &output) {
    output << "Not implemented\n";
}
