variables {
  input = file("../../tests/samples/02.txt")
}

run "run" {

  command = plan

  assert {
    condition     = output.part1 == 2
    error_message = "Part1 output is wrong"
  }

  assert {
    condition     = output.part2 == 4
    error_message = "Part2 output is wrong"
  }

}
