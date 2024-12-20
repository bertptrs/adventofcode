variable "prev" {
  type = map(number)
}

locals {
  not_a_module = {
    for num, _ in var.prev : num => (
      tonumber(num) == 0
      ? [1]
      : (
        length(tostring(num)) % 2 == 0
        ? [
          tonumber(substr(tostring(num), 0, length(tostring(num)) / 2)),
          tonumber(substr(tostring(num), length(tostring(num)) / 2, length(tostring(num)) / 2)),
        ]
        : [num * 2024]
      )
    )
  }
  by_value = flatten([
    for key, value in local.not_a_module :
    [for result in value : { num = result, count = var.prev[key] }]
  ])

  grouped = { for kv in local.by_value : kv.num => kv.count... }
}

output "next" {
  value = { for num, groups in local.grouped : num => sum(groups) }
}
