variable "input" {
  type = string
}

locals {
  nums = [for s in split(" ", chomp(var.input)) : tonumber(s)]

  grouped = { for num in local.nums : num => 1... }
  total   = { for k, v in local.grouped : k => sum(v) }
}

module "step1" {
  source = "./step"

  prev = local.total
}

output "part1" {
  value = sum(values(module.step25.next))
}

output "part2" {
  value = sum(values(module.step75.next))
}
