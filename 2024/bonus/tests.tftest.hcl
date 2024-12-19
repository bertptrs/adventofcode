run "day1" {
  command = plan
  module {
    source = "./day01"
  }

  variables {
    input = file("../tests/samples/01.txt")
  }

  assert {
    condition     = output.part1 == 11
    error_message = "Part1 output is wrong"
  }

  assert {
    condition     = output.part2 == 31
    error_message = "Part2 output is wrong"
  }
}

run "day2" {
  command = plan

  module {
    source = "./day02"
  }

  variables {
    input = file("../tests/samples/02.txt")
  }

  assert {
    condition     = output.part1 == 2
    error_message = "Part1 output is wrong"
  }

  assert {
    condition     = output.part2 == 4
    error_message = "Part2 output is wrong"
  }
}

run "day3_1" {
  command = plan

  module {
    source = "./day03"
  }

  variables {
    input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
  }

  assert {
    condition     = output.part1 == 161
    error_message = "Part1 output is wrong"
  }
}

run "day3_2" {
  command = plan

  module {
    source = "./day03"
  }

  variables {
    input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
  }

  assert {
    condition     = output.part2 == 48
    error_message = "Part2 output is wrong"
  }
}

run "day4_small" {
  command = plan

  module {
    source = "./day04"
  }

  variables {
    input = file("../tests/samples/04.1.txt")
  }

  assert {
    condition     = output.part1 == 4
    error_message = "Part1 output is wrong"
  }
}

run "day4" {
  command = plan

  module {
    source = "./day04"
  }

  variables {
    input = file("../tests/samples/04.2.txt")
  }

  assert {
    condition     = output.part1 == 18
    error_message = "Part1 output is wrong"
  }

  assert {
    condition     = output.part2 == 9
    error_message = "Part2 output is wrong"
  }
}

run "day5_1" {
  command = plan

  module {
    source = "./day05"
  }

  variables {
    input = file("../tests/samples/05.txt")
  }

  assert {
    condition     = output.part1 == 143
    error_message = "Part1 output is wrong"
  }
}

run "day11" {
  command = plan

  module {
    source = "./day11"
  }

  variables {
    input = "125 17"
  }

  assert {
    condition     = output.part1 == 55312
    error_message = "Part1 output is wrong"
  }
}

run "day19" {
  command = plan

  module {
    source = "./day19"
  }

  variables {
    input = file("../tests/samples/19.txt")
  }

  assert {
    condition     = output.part1 == 6
    error_message = "Part1 output is wrong"
  }
}
