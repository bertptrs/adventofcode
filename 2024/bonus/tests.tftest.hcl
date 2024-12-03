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
