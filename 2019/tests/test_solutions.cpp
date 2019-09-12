#define BOOST_TEST_MODULE solutions_tests

#include <string>
#include <boost/test/included/unit_test.hpp>
#include "solutions.hpp"

static std::string run_day1_part1(std::string_view input) {
    std::stringstream input_stream;
    input_stream.write(input.data(), input.size());

    std::stringstream output_stream;

    aoc2019::day01_part1(input_stream, output_stream);
    return output_stream.str();
}

BOOST_AUTO_TEST_CASE(sample_day1_part1)
{
    BOOST_TEST(run_day1_part1("+1\n-2\n+3\n+1") == "3\n");
    BOOST_TEST(run_day1_part1("+1\n+1\n+1") == "3\n");
    BOOST_TEST(run_day1_part1("+1\n+1\n-2") == "0\n");
}


