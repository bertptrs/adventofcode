#pragma once

#include <array>
#include <cstdlib>
#include "utils.hpp"

namespace aoc2019 {
    template<class T, std::size_t L>
    class Point : public std::array<T, L> {
    public:
        constexpr Point& operator +=(Point other) {
            for (std::size_t i = 0; i < L; ++i) {
                (*this)[i] += other[i];
            }
            return *this;
        }

        constexpr Point operator+(Point other) const {
            auto result = *this;
            result += other;

            return result;
        }

        constexpr Point& operator -=(Point other) {
            for (std::size_t i = 0; i < L; ++i) {
                (*this)[i] -= other[i];
            }

            return *this;
        }

        constexpr Point operator-(Point other) const {
            auto result = *this;
            result -= other;

            return result;
        }

        constexpr T l1() const {
            T result = 0;
            for (auto e : *this) {
                result += std::abs(e);
            }

            return result;
        }
    };

    template<typename ValueType, std::size_t N, typename Ignored>
    std::pair<Point<ValueType, N>, Point<ValueType, N>> bounding_box(const std::unordered_map<Point<ValueType, N>, Ignored> &data) {
        Point<ValueType, N> lower, upper;
        std::fill(lower.begin(), lower.end(), std::numeric_limits<ValueType>::max());
        std::fill(upper.begin(), upper.end(), std::numeric_limits<ValueType>::min());

        for (auto &entry : data) {
            for (int i = 0; i < N; ++i) {
                lower[i] = std::min(entry.first[i], lower[i]);
                upper[i] = std::max(entry.first[i], upper[i]);
            }
        }

        return {lower, upper};
    }
}

namespace std {
    // Make point usable with unordered collections.
    template<class T, std::size_t L> struct hash<aoc2019::Point<T, L>> {
        size_t operator()(const aoc2019::Point<T, L> &o) const {
            size_t seed = 0;
            for (auto i : o) {
                aoc2019::combine_hash(seed, i);
            }
            return seed;
        }
    };
}
