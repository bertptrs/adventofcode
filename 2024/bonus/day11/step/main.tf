variable "prev" {
  type = list(number)
}

locals {
  values = [
    for num in var.prev : num == 0 ? [1]
    : length(tostring(num)) % 2 == 0
    ? [tonumber(substr(tostring(num), 0, length(tostring(num)) / 2)), tonumber(substr(tostring(num), length(tostring(num)) / 2, 10))]
    : [num * 2024]
  ]
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
  value = flatten(local.values)
}
