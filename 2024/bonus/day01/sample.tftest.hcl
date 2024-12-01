variables {
  input = file("../../tests/samples/01.txt")
}

run "run" {

  command = plan

  assert {
    condition     = output.part1 == 11
    error_message = "Part1 output is wrong"
  }

  assert {
    condition = output.part2 == 31
    error_message = "Part2 output is wrong"
  }

}
