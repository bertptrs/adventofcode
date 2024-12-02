variable "input" {
  type = string
}

locals {
  reports = [for line in split("\n", trim(var.input, "\n")) : [for num in split(" ", line) : parseint(num, 10)]]
}

module "part1_valid" {
  source = "./is_valid"

  count  = length(local.reports)
  report = local.reports[count.index]
}

module "part2_valid" {
  source = "./is_savable"
  count  = length(local.reports)
  report = local.reports[count.index]
}

output "part1" {
  value = length([for i in range(length(local.reports)) : true if module.part1_valid[i].valid])
}

output "part2" {
  value = length([for i in range(length(local.reports)) : true if module.part2_valid[i].valid])
}
