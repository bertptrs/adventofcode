run "sample-1" {
  module {
    source = "./range"
  }

  variables {
    min = 11
    max = 22
  }

  assert {
    condition     = output.invalid_sum == 33
    error_message = "Incorrect result"
  }
}

run "sample-2" {
  module {
    source = "./range"
  }

  variables {
    min = 95
    max = 115
  }

  assert {
    condition     = output.invalid_sum == 99
    error_message = "Incorrect result"
  }
}
