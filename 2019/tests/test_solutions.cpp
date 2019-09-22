#define BOOST_TEST_MODULE solutions_tests

#include <cassert>
#include <cctype>
#include <cstring>
#include <charconv>
#include <filesystem>
#include <fstream>
#include <string>
#include <gtest/gtest.h>
#include "implementations.hpp"

class SolutionsTest : public testing::TestWithParam<std::string> {
public:
    static std::string nameInstantiatedTest(const testing::TestParamInfo<SolutionsTest::ParamType> &paramInfo);

protected:
    std::string input_data = "";
    std::string output_data = "";
    aoc2019::solution_t implementation = nullptr;

    // Read input data
    void SetUp() override;

private:
    static void readToString(const std::string &name, std::string &target);

    static std::tuple<int, bool, std::string> parseInputName(const std::string &name);
};

void SolutionsTest::SetUp() {
    const auto input_name = GetParam();
    const auto output_name = input_name.substr(0, input_name.length() - 3) + ".out";

    int day;
    bool part2;
    std::tie(day, part2, std::ignore) = parseInputName(input_name);
    implementation = aoc2019::get_implementation(day, part2);

    readToString(input_name, input_data);
    readToString(output_name, output_data);
}

void SolutionsTest::readToString(const std::string &name, std::string &target) {
    std::ifstream source(name);

    target.assign(std::istreambuf_iterator<char>(source),
                  std::istreambuf_iterator<char>());
}

std::tuple<int, bool, std::string> SolutionsTest::parseInputName(const std::string &name) {
    const char *base_name = name.c_str();
    if (const auto last_slash = name.rfind('/'); last_slash != std::string::npos) {
        base_name += last_slash + 1;
    }
    int day, part;
    auto res = std::from_chars(base_name, base_name + 2, day);
    assert(res.ec == std::errc());
    res = std::from_chars(base_name + 3, base_name + 4, part);
    assert(res.ec == std::errc());

    return {day, part == 2, std::string(base_name + 5, std::strchr(base_name, '.'))};
}

std::string SolutionsTest::nameInstantiatedTest(const testing::TestParamInfo<SolutionsTest::ParamType> &paramInfo) {
    int day;
    bool part2;
    std::string sampleName;

    std::tie(day, part2, sampleName) = parseInputName(paramInfo.param);

    std::stringstream nameBuilder;

    nameBuilder << "Day" << day << "Part" << (part2 ? 2 : 1) << "Sample";

    std::copy_if(sampleName.cbegin(), sampleName.cend(), std::ostream_iterator<char>(nameBuilder), [](char c) {
        return std::isalnum(c);
    });

    return nameBuilder.str();
}

TEST_P(SolutionsTest, TestExpectedOutcome) {
    std::stringstream input_buffer, output_buffer;

    // Sanity check, don't call null implementation
    ASSERT_NE(implementation, nullptr);

    input_buffer.str(input_data);

    implementation(input_buffer, output_buffer);

    ASSERT_EQ(output_data, output_buffer.str());
}

static std::vector<std::string> get_samples() {
    std::vector<std::string> samples;
    for (const auto &entry : std::filesystem::directory_iterator(TEST_SAMPLES_DIR)) {
        if (entry.path().filename().extension() == ".in") {
            samples.push_back(entry.path().string());
        }
    }

    // Ensure a consistent order.
    std::sort(samples.begin(), samples.end());

    return samples;
}

INSTANTIATE_TEST_CASE_P(DaysTest, SolutionsTest,
                        testing::ValuesIn(get_samples()),
                        SolutionsTest::nameInstantiatedTest);
