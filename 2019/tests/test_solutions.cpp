#define BOOST_TEST_MODULE solutions_tests

#include <cassert>
#include <cstdio>
#include <string>
#include <boost/filesystem.hpp>
#include <gtest/gtest.h>
#include "implementations.hpp"

class SolutionsTest : public testing::TestWithParam<std::string> {
protected:
    std::string input_data = "";
    std::string output_data = "";
    aoc2019::solution_t implementation = nullptr;

    // Read input data
    void SetUp() override;

private:
    static void readToString(const std::string &name, std::string &target);
};

void SolutionsTest::SetUp() {
    const auto input_name = GetParam();
    const auto output_name = input_name.substr(0, input_name.length() - 3) + ".out";

    const char *base_name = input_name.c_str();
    if (const auto last_slash = input_name.rfind('/'); last_slash != std::string::npos) {
        base_name += last_slash + 1;
    }

    int day, part;
    const auto read_result = std::sscanf(base_name, "%02d-%1d-", &day, &part); // NOLINT(cert-err34-c)
    // Ensure that we've read the input files.
    assert(read_result != 0);

    const bool part2 = part == 2;
    implementation = aoc2019::get_implementation(day, part2);

    readToString(input_name, input_data);
    readToString(output_name, output_data);
}

void SolutionsTest::readToString(const std::string &name, std::string &target) {
    std::ifstream source(name);

    target.assign(std::istreambuf_iterator<char>(source),
                  std::istreambuf_iterator<char>());
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
    for (const auto &entry : boost::filesystem::directory_iterator(TEST_SAMPLES_DIR)) {
        if (entry.path().filename().extension() == ".in") {
            samples.push_back(entry.path().string());
        }
    }

    // Ensure a consistent order.
    std::sort(samples.begin(), samples.end());

    return samples;
}

INSTANTIATE_TEST_CASE_P(InstantiatedSolutionsTest, SolutionsTest,
                        testing::ValuesIn(get_samples()));
