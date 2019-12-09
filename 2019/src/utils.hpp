#pragma once

#include <charconv>
#include <deque>
#include <functional>
#include <iosfwd>
#include <string_view>
#include <vector>

namespace aoc2019 {

    template<typename T>
    inline std::from_chars_result from_chars(std::string_view str, T &value) {
        return std::from_chars(str.data(), str.data() + str.size(), value);
    }

    template<typename T>
    void combine_hash(std::size_t &seed, const T &o) {
        // Algorithm taken from boost::combine_hash.
        std::hash<T> hash{};
        seed ^= hash(o) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    }

    std::string_view strtok(std::string_view &str, char token = ',');

    std::vector<int> read_intcode(std::istream &input);

    std::vector<int> run_intcode(std::vector<int> &program, std::deque<int> inputs = {});

    class IntCodeComputer {
    public:
        typedef std::int64_t value_t;

        explicit IntCodeComputer(std::vector<value_t> program, std::deque<value_t> initial_inputs = {});

        void run();
        void connectOutput(IntCodeComputer &computer);
        void connectOutput(std::deque<value_t> &sink);
        void sendInput(value_t input);

        [[nodiscard]] bool isTerminated() const;

        [[nodiscard]] const std::deque<value_t> &currentInputs() const;

        static std::vector<value_t> read_intcode(std::istream &input);

    private:
        std::vector<value_t> program;
        std::deque<value_t> inputs = {};
        std::deque<value_t> *outputSink = nullptr;
        int ip = 0;
        int relative = 0;
        bool halted = false;

        [[nodiscard]] value_t &interpret_value(int pos);
    };
}
