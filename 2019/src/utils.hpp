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

    template<typename ValueType, typename OutputIt>
    std::istream &read_line_numbers_and_garbage(std::istream &input, OutputIt output) {
        ValueType v;
        char c;
        while (input && (c = input.peek()) != '\n') {
            if (c == '-' || std::isdigit(c)) {
                input >> v;
                *output = v;
                ++output;
            } else {
                input.ignore();
            }
        }

        input.get();

        return input;
    }

    std::string_view strtok(std::string_view &str, char token = ',');

    std::deque<int64_t> run_intcode(std::vector<std::int64_t> program, std::deque<std::int64_t> inputs = {});

    template<class Node>
    std::vector<Node> topological_sort(const std::unordered_map<Node, std::vector<Node>> &edge_list) {
        std::unordered_map<Node, int> incoming_edges;

        for (auto &entry : edge_list) {
            // Ensure entry for parent exist
            incoming_edges[entry.first] += 0;

            for (auto &node : entry.second) {
                incoming_edges[node]++;
            }
        }

        std::vector<Node> order;
        std::deque<Node> childless;

        for (auto &entry : incoming_edges) {
            if (!entry.second) {
                childless.push_back(entry.first);
            }
        }

        while (!childless.empty()) {
            auto current = childless.front();
            childless.pop_front();
            order.emplace_back(current);

            if (auto it = edge_list.find(current); it != edge_list.end()) {
                for (const auto &parent : it->second) {
                    if (--incoming_edges[parent] == 0) {
                        childless.push_back(parent);
                    }
                }
            }
        }

        if (order.size() != incoming_edges.size()) {
            throw std::domain_error("Not a DAG.");
        }

        return order;
    }

    class IntCodeComputer {
    public:
        typedef std::int64_t value_t;

        explicit IntCodeComputer(std::vector<value_t> program, std::deque<value_t> initial_inputs = {});
        explicit IntCodeComputer(std::istream &program_stream, std::deque<value_t> initial_inputs = {});

        void run();
        void connectOutput(IntCodeComputer &computer);
        void connectOutput(std::deque<value_t> &sink);
        void sendInput(value_t input);

        [[nodiscard]] bool isTerminated() const;

        [[nodiscard]] const std::deque<value_t> &currentInputs() const;

        value_t &operator[](std::size_t index);
        const value_t &operator[](std::size_t index) const;

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
