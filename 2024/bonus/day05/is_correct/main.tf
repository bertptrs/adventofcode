variable "update" {
  type = list(number)
}

variable "disallow_rules" {
  type = map(list(number))
}

locals {
  not_disallowed = alltrue([
    for i in range(1, length(var.update)) :
    !contains(
      flatten([for j in range(i) : lookup(var.disallow_rules, var.update[j], [])]),
      var.update[i]
    )
  ])
}

output "valid" {
  value = local.not_disallowed ? var.update[floor(length(var.update) / 2)] : 0
}
