variable "prev" {
  type = map(number)
}

locals {
  by_value = flatten([
    for num, count in var.prev : (
      tonumber(num) == 0
      ? [{ number = 1, amount = count }]
      : (
        length(tostring(num)) % 2 == 0
        ? [
          {
            number = tonumber(substr(tostring(num), 0, length(tostring(num)) / 2)),
            amount = count
          },
          {
            number = tonumber(substr(tostring(num), length(tostring(num)) / 2, length(tostring(num)) / 2)),
            amount = count,
          },
        ]
        : [{ number = 2024 * num, amount = count }]
      )
    )
  ])

  grouped = { for kv in local.by_value : kv.number => kv.amount... }
}

output "next" {
  value = { for num, groups in local.grouped : num => sum(groups) }
}
