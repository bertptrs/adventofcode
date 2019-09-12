#define BOOST_TEST_MODULE solutions_tests

#include <regex>
#include <string>
#include <boost/filesystem.hpp>
#include <boost/test/included/unit_test.hpp>
#include <boost/test/data/test_case.hpp>
#include "solutions.hpp"

std::vector<std::string> get_samples() {
    std::vector<std::string> samples;
    for (const auto &entry : boost::filesystem::directory_iterator("./samples")) {
        if (entry.path().filename().extension() == ".in") {
            samples.push_back(entry.path().string());
        }
    }

    return samples;
}

static std::string read_file(const std::string &file_name) {
    std::ifstream file(file_name);
    return std::string(std::istreambuf_iterator<char>(file),
                       std::istreambuf_iterator<char>());
}

static void test_solution_impl(const std::string &input_name) {
    std::regex name_parser("/(\\d{2})-(1|2).*\\.in$");
    std::smatch match;
    // Sanity check, is this a parseable input file?
    BOOST_TEST(std::regex_search(input_name, match, name_parser));

    const auto output_filename = input_name.substr(0, input_name.length() - 3) + ".out";
    const int day = std::atoi(match[1].str().c_str());
    const int part2 = match[2].str() == "2";

    const auto desired_output = read_file(output_filename);
    const auto implementation = aoc2019::get_implementation(day, part2);

    std::stringstream output_buffer;
    std::ifstream input(input_name);

    implementation(input, output_buffer);

    BOOST_TEST(desired_output == output_buffer.str());
}

BOOST_DATA_TEST_CASE(test_solution,
                     boost::unit_test::data::make(get_samples()),
                     input_name) {
    test_solution_impl(input_name);
}
