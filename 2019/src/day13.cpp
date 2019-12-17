#include <iostream>
#ifdef ANIMATE_DAY13
#include <chrono>
#include <thread>
#endif
#include "days.hpp"
#include "utils.hpp"
#include "point.hpp"

namespace {
    typedef aoc2019::Point<int64_t, 2> point_t;

    enum class Tile {
        EMPTY,
        WALL,
        BLOCK,
        PADDLE,
        BALL,
    };

    typedef std::unordered_map<point_t, Tile> Screen;

    std::optional<std::int64_t> update_screen(std::deque<std::int64_t> &output_buffer, Screen &screen) {
        std::optional<std::int64_t> score;
        while (!output_buffer.empty()) {
            auto x = output_buffer.front(); output_buffer.pop_front();
            auto y = output_buffer.front(); output_buffer.pop_front();
            auto type = output_buffer.front(); output_buffer.pop_front();

            if (x == -1 && y == 0) {
                score = type;
                continue;
            }

            screen[{x, y}] = static_cast<Tile>(type);
        }
        return score;
    }

    void draw_screen(const Screen &screen, std::ostream& output) {
        // Determine bounding box
        using limits = std::numeric_limits<int>;

        const auto [lower, upper] = aoc2019::bounding_box(screen);

        for (auto y = lower[1]; y <= upper[1]; ++y) {
            for (auto x = lower[0]; x <= upper[0]; ++x) {
                char c = ' ';
                if (auto it = screen.find({x, y}); it != screen.end()) {
                    switch (it->second) {
                        case Tile::EMPTY:
                            c = ' ';
                            break;

                        case Tile::BALL:
                            c = '*';
                            break;

                        case Tile::BLOCK:
                            c = '=';
                            break;

                        case Tile::PADDLE:
                            c = '_';
                            break;

                        case Tile::WALL:
                            c = '#';
                            break;
                    }
                }

                output << c;
            }

            output << '\n';
        }
    }

    auto find_pos(const Screen &screen, Tile to_find) {
        return std::find_if(screen.begin(), screen.end(), [to_find](const auto& x) {
            return x.second == to_find;
        });
    }
}

void aoc2019::day13_part1(std::istream &input, std::ostream &output) {
    Screen screen;
    aoc2019::IntCodeComputer computer(aoc2019::IntCodeComputer::read_intcode(input));
    std::deque<std::int64_t> output_buffer;
    computer.connectOutput(output_buffer);
    computer.run();
    update_screen(output_buffer, screen);

    output << std::count_if(screen.begin(), screen.end(), [](const auto &x) { return x.second == Tile::BLOCK; })
           << std::endl;
}

void aoc2019::day13_part2(std::istream &input, std::ostream &output) {
    auto program = aoc2019::IntCodeComputer::read_intcode(input);
    program[0] = 2;

    aoc2019::IntCodeComputer computer(std::move(program));
    std::deque<std::int64_t> output_buffer;
    computer.connectOutput(output_buffer);
    computer.run();

    Screen screen;

    std::int64_t score = 0;

    while (!computer.isTerminated()) {
        computer.run();
        auto new_score = update_screen(output_buffer, screen);
        if (new_score) {
            score = *new_score;
        }

#ifdef ANIMATE_DAY13
        output << "Score: " << score << std::endl;
        draw_screen(screen, output);
        std::this_thread::sleep_for(std::chrono::milliseconds(40));
#endif

        auto ball_pos = find_pos(screen, Tile::BALL)->first;
        auto paddle_pos = find_pos(screen, Tile::PADDLE)->first;

        if (ball_pos[0] < paddle_pos[0]) {
            computer.sendInput(-1);
        } else if (ball_pos[0] > paddle_pos[0]) {
            computer.sendInput(1);
        } else {
            computer.sendInput(0);
        }
    }

    output << score << std::endl;
}
