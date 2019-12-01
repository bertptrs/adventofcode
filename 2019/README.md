# Advent of Code 2019

This project contains my implementations for Advent of Code 2019. The
goal is to create reasonably fast C++ implementations in readable and
ergonomic C++. At the end of the contest, I will probably do a write-
up of some sorts. 


## How to compile

Install the dependencies:

- [GTest](https://github.com/google/googletest) **Note:** this project
  by default tries to dynamically link GTest, and the Ubuntu packages
  only provide a statically linked archive. You may need to compile it
  for yourself.

```
mkdir build && cd build
cmake ..
make
```

You can then use the generated executable `runner`.

## Running tests

Tests can be executed with `make test`. The `tests` folder contains a
`samples` folder. This folder contains pairs of `XX-Y-something.in` and
`XX-Y-something.out`, which will be taken as the expected input and
output of the implementations. You can add your own samples to this mix.
