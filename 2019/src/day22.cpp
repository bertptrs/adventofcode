#include <iostream>
#include <cassert>
#include "days.hpp"
#include "utils.hpp"

namespace {
    enum class Operation {
        Stack,
        Deal,
        Cut
    };

    using Move = std::pair<Operation, int>;

    std::vector<Move> read_moves(std::istream &input) {
        std::string buffer;
        std::vector<Move> moves;

        while (std::getline(input, buffer)) {
            std::string_view line = buffer;
            if (!line.find("deal into new stack")) {
                moves.emplace_back(Operation::Stack, 0);
            } else if (!line.find("deal with increment ")) {
                int new_increment;
                aoc2019::from_chars(line.substr(20), new_increment);
                moves.emplace_back(Operation::Deal, new_increment);
            } else {
                // cut
                int new_offset;
                aoc2019::from_chars(line.substr(4), new_offset);

                moves.emplace_back(Operation::Cut, new_offset);
            }
        }

        return moves;
    }

    constexpr std::int64_t mmi(std::int64_t a, std::int64_t n) {
        std::int64_t t = 0, newt = 1, r = n, newr = a;

        while (newr != 0) {
            auto q = r / newr;
            // Poor man's simultaneous assignment
            std::tie(t, newt) = std::make_tuple(newt, t - q * newt);
            std::tie(r, newr) = std::make_tuple(newr, r - q * newr);
        }

        if (r > 1) {
            throw std::invalid_argument("Not invertible.");
        }

        if (t < 0) t += n;

        assert((t * a) % n == 1);

        return t;
    }

    constexpr std::pair<std::int64_t, std::int64_t> pow(std::int64_t a, std::int64_t b, std::int64_t n, const std::int64_t M) {
        __int128 ra = 0, rb = 0;

        while (n > 0) {
            if (n % 2) {
                ra = (ra + a) % M;
                rb = (rb + b) % M;
            }

            // f(x) = ax + b
            // f(f(x)) = a(ax + b) + b
            //         = aax + ab + b
            __int128 na = a * (__int128) a;
            __int128 nb = b * (__int128) a + b;

            a = na % M;
            b = nb % M;

            n /= 2;
        }

        return {ra, rb};
    }
}

void aoc2019::day22_part1(std::istream &input, std::ostream &output) {
    constexpr int DECK_SIZE = 10007;

    int pos = 2019;

    for (auto move : read_moves(input)) {
        int argument = move.second;
        switch (move.first) {
            case Operation::Stack:
                pos = DECK_SIZE - 1 - pos;
                break;

            case Operation::Deal:
                pos = pos * argument % DECK_SIZE;
                break;

            case Operation::Cut:
                pos = (pos - argument) % DECK_SIZE;
                if (pos < 0) pos += DECK_SIZE;
                break;
        }
    }

    output << pos << std::endl;
}

void aoc2019::day22_part2(std::istream &input, std::ostream &output) {
    constexpr std::int64_t DECK_SIZE = 119315717514047;
    constexpr std::int64_t SHUFFLES = 101741582076661;

    assert(mmi(3, 11) == 4);

    std::int64_t a = 1, b = 0;

    for (auto move : read_moves(input)) {
        std::int64_t argument = move.second;
        switch (move.first) {
            case Operation::Stack:
                a = -a;
                b = DECK_SIZE - b - 1;
                break;

            case Operation::Cut:
                b = (b + argument) % DECK_SIZE;
                break;

            case Operation::Deal:
                __int128 inv = mmi(argument, DECK_SIZE);
                a = (a * inv) % DECK_SIZE;
                b = (b * inv) % DECK_SIZE;
                break;
        }
    }

    const auto[ra, rb] = pow(a, b, SHUFFLES, DECK_SIZE);

    output << ra << ',' << rb << std::endl;

    auto result = (2020 * ra + rb) % DECK_SIZE;
    if (result < 0) {
        result += DECK_SIZE;
    }

    output << result << std::endl;
}
