#include "implementations.hpp"
#include <charconv>
#include <chrono>
#include <iostream>
#include <string_view>
#include <optional>
#include <fstream>

struct AoCOptions {
    aoc2019::solution_t implementation;
    bool run_timer;
    std::optional<std::ifstream> input_file;
};

static AoCOptions parse_options(const int argc, const char* argv[]) {
    using namespace std::literals;
    AoCOptions options{};

    auto show_help = [argv] (int exit_status = 0) {
        std::cerr << "Usage: " << argv[0] << " [--timer|-t] [--part2|-2] [--help|-h] DAY\n"
                  << "\t--timer|-t: print execution time\n"
                  << "\t--input ARG|-fARG: use given input file as puzzle input"
                  << "\t--part2|-2: run part 2\n"
                  << "\t --help|-h: show this message\n";
        std::exit(exit_status);
    };

    int day = -1;
    bool part2 = false;

    // Here follows a manual implementation of getopt, since getopt doesn't work on windows…
    for (int i = 1; i < argc; ++i) {
        std::string_view arg(argv[i]);
        if (arg[0] == '-') {
            // Handle flag arguments
            if (arg[1] != '-') {
                // Shorthand flags
                for (int j = 1; j < arg.size(); ++j) {
                    switch (arg[j]) {
                        case '2':
                            part2 = true;
                            break;

                        case 't':
                            options.run_timer = true;
                            break;

                        case 'h':
                            show_help();
                            break;

                        case 'f':
                            if (j == arg.size() - 1) {
                                if (i == argc - 1) {
                                    std::cerr << "Option -f requires an argument.";
                                    show_help(1);
                                } else {
                                    options.input_file = std::ifstream(argv[i + 1]);
                                    ++i;
                                }
                            } else {
                                options.input_file = std::ifstream(std::string(arg.substr(j)));
                                j = arg.size();
                            }
                            break;

                        default:
                            std::cerr << "Unknown flag '" << arg[j] << "'.\n\n";
                            show_help(1);
                    }
                }
            } else {
                // Handle long form versions
                if (arg == "--timer"sv) {
                    part2 = true;
                } else if (arg == "--timer"sv) {
                    options.run_timer = true;
                } else if (arg == "--help"sv) {
                    show_help();
                } else if (arg == "--input"sv) {
                    if (i == argc - 1) {
                        std::cerr << "Option -f requires an argument.";
                        show_help(1);
                    } else {
                        options.input_file = std::ifstream(argv[i + 1]);
                        ++i;
                    }
                } else {
                    show_help(1);
                }
            }
        } else {
            if (day != -1) {
                // Double date specification, bail.
                show_help(1);
            }

            // Try to parse the date number
            if (auto res = std::from_chars(arg.data(), arg.data() + arg.size(), day); res.ec != std::errc()) {
                auto error_code = std::make_error_code(res.ec);
                std::cerr << error_code.message() << "\n\n";
                show_help(1);
            }
        }
    }

    if (day == -1) {
        std::cerr << "Argument DAY is required.\n\n";
        show_help(1);
    } else if (day < 1 || day > 25) {
        std::cerr << "Invalid day. Valid range: [1, 25]\n";
        show_help(1);
    }

    options.implementation = aoc2019::get_implementation(day, part2);

    return options;
}

int main(int argc, const char *argv[]) {
    auto options = parse_options(argc, argv);

    if (options.implementation != nullptr) {
        const auto start = std::chrono::high_resolution_clock::now();
        options.implementation(options.input_file ? *options.input_file : std::cin, std::cout);
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
