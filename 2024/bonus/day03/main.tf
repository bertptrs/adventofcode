variable "input" {
  type = string
}

locals {
  muls = regexall("mul\\((\\d+),(\\d+)\\)", var.input)
  ops  = regexall("(don't\\(\\)|do\\(\\)|mul\\((\\d+),(\\d+)\\))", var.input)
}

module "should_execute" {
  count  = length(local.ops)
  source = "./should_execute"

  index = count.index
  ops   = local.ops
}

output "part1" {
  value = sum([for mul in local.muls : parseint(mul[1], 10) * parseint(mul[0], 10)])
}

output "part2" {
  value = sum(module.should_execute[*].value)
}
