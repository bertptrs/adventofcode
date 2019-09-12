#include <numeric>
#include <iterator>
#include <vector>
#include <unordered_set>
#include "solutions.hpp"

// Currently an implementation of 2018 day 1
void aoc2019::day01_part1(std::istream &input, std::ostream &output) {
    int sum = std::accumulate(std::istream_iterator<int>(input),
                              std::istream_iterator<int>(),
                              0);

    output << sum << std::endl;
}

void aoc2019::day01_part2(std::istream &input, std::ostream &output) {
    std::vector<int> drifts;
    std::copy(std::istream_iterator<int>(input),
              std::istream_iterator<int>(),
              std::back_inserter(drifts));

    int cur = 0;
    std::unordered_set<int> seen{cur};
    while (true) {
        for (auto drift : drifts) {
            cur += drift;
            if (seen.count(cur) == 1) {
                output << cur << std::endl;
                return;
            } else {
                seen.insert(cur);
            }
        }
    }
}
