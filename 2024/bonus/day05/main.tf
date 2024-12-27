variable "input" {
  type = string
}

locals {
  parts          = split("\n\n", chomp(var.input))
  rules          = [for rule_line in split("\n", local.parts[0]) : [for v in split("|", rule_line) : tonumber(v)]]
  disallow_rules = { for rule in local.rules : rule[1] => rule[0]... }

  updates = [for update_line in split("\n", local.parts[1]) : [for v in split(",", update_line) : tonumber(v)]]

  scores = [
    for update in local.updates :
    alltrue([
      for i in range(1, length(update)) :
      !contains(
        flatten([for j in range(i) : lookup(local.disallow_rules, update[j], [])]),
        update[i]
      )
  ]) ? update[floor(length(update) / 2)] : 0]
}

output "part1" {
  value = sum(local.scores[*])
}
