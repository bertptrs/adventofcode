#include "solutions.hpp"
#include <iostream>
#include <boost/program_options.hpp>

namespace po = boost::program_options;

struct AoCOptions {
    int day;
    bool part2;
};

AoCOptions parse_options(const int argc, const char* argv[]) {
    AoCOptions options{};
    po::options_description desc("Allowed options");
    desc.add_options()
            ("day", po::value<int>(&options.day)->required(), "The day to run.")
            ("part2,2", po::bool_switch(&options.part2), "Whether to run part 2.");

    po::positional_options_description positionals;
    positionals.add("day", 1);

    po::variables_map vm;

    po::store(po::command_line_parser(argc, argv).options(desc).positional(positionals).run(), vm);
    po::notify(vm);

    return options;
}

int main(int argc, const char *argv[]) {
    try {
        const auto options = parse_options(argc, argv);

        const aoc2019::solution_t solution = aoc2019::get_implementation(options.day, options.part2);
        if (solution != nullptr) {
            solution(std::cin, std::cout);
            return 0;
        } else {
            std::cerr << "Unimplemented.\n";
            return 1;
        }
    } catch (po::required_option &ignored) {
        return 1;
    } catch (std::out_of_range &ignored) {
        std::cerr << "Invalid day.\n";
        return 1;
    }
}
