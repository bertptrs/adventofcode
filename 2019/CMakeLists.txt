cmake_minimum_required(VERSION 3.15)

project(aoc2019)

find_package(Boost REQUIRED COMPONENTS program_options)

add_library(AoCSolutions src/solutions.hpp src/solutions.cpp src/day01.cpp)
target_compile_features(AoCSolutions PUBLIC cxx_std_17)

add_executable(runner src/runner.cpp)
target_compile_features(runner PUBLIC cxx_std_17)
target_link_libraries(runner AoCSolutions Boost::program_options)
