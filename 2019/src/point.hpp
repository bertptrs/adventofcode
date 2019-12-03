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
