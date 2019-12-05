cmake_minimum_required(VERSION 3.12)

project(aoc2019)

find_package(GTest REQUIRED)

file(GLOB DAYS_FILES src/day*.cpp)
add_library(AoCSolutions src/implementations.cpp "${DAYS_FILES}" src/point.hpp src/utils.cpp src/utils.hpp)
target_compile_features(AoCSolutions PUBLIC cxx_std_17)

add_executable(runner src/runner.cpp)
target_compile_features(runner PUBLIC cxx_std_17)
target_link_libraries(runner AoCSolutions)

add_executable(unit_tests tests/test_solutions.cpp tests/test_intcode.cpp)
target_compile_features(unit_tests PUBLIC cxx_std_17)
target_link_libraries(unit_tests AoCSolutions GTest::GTest GTest::Main)
target_compile_definitions(unit_tests PRIVATE "TEST_SAMPLES_DIR=\"${CMAKE_SOURCE_DIR}/tests/samples\"")
target_include_directories(unit_tests PRIVATE "${CMAKE_SOURCE_DIR}/src")

enable_testing()
gtest_discover_tests(unit_tests NO_PRETTY_VALUES)
