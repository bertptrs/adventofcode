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
