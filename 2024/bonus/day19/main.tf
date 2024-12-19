variable "input" {
  type = string
}

locals {
  parts    = split("\n\n", chomp((var.input)))
  patterns = replace(local.parts[0], ", ", "|")
  valid    = [for line in split("\n", local.parts[1]) : line if length(regexall("^(${local.patterns})+$", line)) > 0]
}

output "part1" {
  value = length(local.valid)
}
