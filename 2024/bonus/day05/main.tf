variable "input" {
  type = string
}

locals {
  parts          = split("\n\n", chomp(var.input))
  rules          = [for rule_line in split("\n", local.parts[0]) : [for v in split("|", rule_line) : tonumber(v)]]
  disallow_rules = { for rule in local.rules : rule[1] => rule[0]... }

  updates = [for update_line in split("\n", local.parts[1]) : [for v in split(",", update_line) : tonumber(v)]]
}

module "is_valid" {
  source = "./is_correct"
  count  = length(local.updates)

  update         = local.updates[count.index]
  disallow_rules = local.disallow_rules
}

output "part1" {
  value = sum(module.is_valid[*].valid)
}
