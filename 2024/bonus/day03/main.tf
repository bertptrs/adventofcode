variable "input" {
  type = string
}

locals {
  muls = regexall("mul\\((\\d+),(\\d+)\\)", var.input)
}

output "part1" {
  value = sum([for mul in local.muls : parseint(mul[1], 10) * parseint(mul[0], 10)])
}
