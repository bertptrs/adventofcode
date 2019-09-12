cmake_minimum_required(VERSION 3.12)

project(aoc2019)

find_package(Boost REQUIRED COMPONENTS program_options unit_test_framework filesystem)

add_library(AoCSolutions src/solutions.hpp src/solutions.cpp src/day01.cpp)
target_compile_features(AoCSolutions PUBLIC cxx_std_17)

add_executable(runner src/runner.cpp)
target_compile_features(runner PUBLIC cxx_std_17)
target_link_libraries(runner AoCSolutions Boost::program_options)

add_executable(test_solutions tests/test_solutions.cpp)
target_compile_features(test_solutions PUBLIC cxx_std_17)
target_link_libraries(test_solutions AoCSolutions Boost::unit_test_framework Boost::filesystem)
target_include_directories(test_solutions PRIVATE "${CMAKE_SOURCE_DIR}/src")

enable_testing()
add_test(NAME test_solutions
         COMMAND test_solutions
         WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/tests")
