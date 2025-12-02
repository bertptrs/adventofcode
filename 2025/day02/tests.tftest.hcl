run "sample-1-1" {
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

run "sample-1-2" {
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

run "sample-2-1" {
  module {
    source = "./range2"
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

run "sample-2-2" {
  module {
    source = "./range2"
  }

  variables {
    min = 95
    max = 115
  }

  assert {
    condition     = output.invalid_sum == 210
    error_message = "Incorrect result"
  }
}

run "sample-2-3" {
  module {
    source = "./range2"
  }

  variables {
    min = 998
    max = 1012
  }

  assert {
    condition     = output.invalid_sum == 2009
    error_message = "Incorrect result"
  }
}

run "sample-2-2-detail" {
  module {
    source = "./range"
  }

  variables {
    min         = 95
    max         = 115
    repetitions = 3
  }

  assert {
    condition     = output.invalid_sum == 111
    error_message = "Incorrect result"
  }
}

run "sample-2-10" {
  module {
    source = "./range2"
  }

  variables {
    min = 824824821
    max = 824824827
  }

  assert {
    condition     = output.invalid_sum == 824824824
    error_message = "Incorrect result"
  }
}

run "sample-2-11" {
  module {
    source = "./range2"
  }

  variables {
    min = 2121212118
    max = 2121212124
  }

  assert {
    condition     = output.invalid_sum == 2121212121
    error_message = "Incorrect result"
  }
}
