#include "solutions.hpp"
#include <chrono>
#include <iostream>
#include <boost/program_options.hpp>

namespace po = boost::program_options;

struct AoCOptions {
    aoc2019::solution_t implementation;
    bool run_timer;
};

static AoCOptions parse_options(const int argc, const char *argv[]) {
    AoCOptions options{};
    int day;
    bool part2;
    po::options_description desc("Allowed options");
    desc.add_options()
            ("day", po::value<int>(&day)->required(), "The day to run.")
            ("part2,2", po::bool_switch(&part2), "Whether to run part 2.")
            ("timer,t", po::bool_switch(&options.run_timer), "Show the execution time.");

    po::positional_options_description positionals;
    positionals.add("day", 1);

    try {
        po::variables_map vm;

        po::store(po::command_line_parser(argc, argv).options(desc).positional(positionals).run(), vm);
        po::notify(vm);

        options.implementation = aoc2019::get_implementation(day, part2);

        return options;
    } catch (po::error &argument_error) {
        std::cerr << argument_error.what() << std::endl;
        std::exit(1);
    } catch (std::out_of_range &) {
        std::cerr << "Invalid day: " << day
                  << ".\n Valid range: [1, 25].\n";
        std::exit(1);
    }
}

int main(int argc, const char *argv[]) {
    const auto options = parse_options(argc, argv);

    if (options.implementation != nullptr) {
        const auto start = std::chrono::high_resolution_clock::now();
        options.implementation(std::cin, std::cout);
        if (options.run_timer) {
            const std::chrono::duration<double> duration = std::chrono::high_resolution_clock::now() - start;
            std::cerr << "Time taken: " << duration.count() << "s\n";
        }
        return 0;
    } else {
        std::cerr << "Unimplemented.\n";
        return 1;
    }
}
