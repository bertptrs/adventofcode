variable "input" {
  type = string
}

locals {
  filtered  = replace(var.input, "/(?s)don't\\(\\).*?do\\(\\)/", "")
  filtered2 = replace(local.filtered, "/(?s)don't\\(\\).*/", "")

  muls          = regexall("mul\\((\\d+),(\\d+)\\)", var.input)
  filtered_muls = regexall("mul\\((\\d+),(\\d+)\\)", local.filtered2)
}

output "part1" {
  value = sum([for mul in local.muls : tonumber(mul[1]) * tonumber(mul[0])])
}

output "part2" {
  value = sum([for mul in local.filtered_muls : tonumber(mul[1]) * tonumber(mul[0])])
}
