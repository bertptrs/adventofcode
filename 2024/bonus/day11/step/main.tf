variable "prev" {
  type = map(number)
}

module "transform" {
  source   = "../transform"
  for_each = var.prev

  num = each.key
}

locals {
  by_value = flatten([
    for key, value in module.transform :
    [for result in value.result : { num = result, count = var.prev[key] }]
  ])

  grouped = { for kv in local.by_value : kv.num => kv.count... }
}

# module "transform" {
#   source = "../transform"

#   count = length(var.prev)
#   num   = var.prev[count.index]
# }

# output "next" {
#   value = flatten(module.transform[*].result)
# }

output "next" {
  value = { for num, groups in local.grouped : num => sum(groups) }
}
