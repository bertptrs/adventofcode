variable "num" {
  type = number
}

locals {
  as_str = tostring(var.num)
  len    = length(local.as_str)
  half   = floor(length(local.as_str) / 2)
  first  = try(tonumber(substr(local.as_str, 0, local.half)), -1)
  second = try(tonumber(substr(local.as_str, local.half, local.half)), -1)
}

output "result" {
  value = var.num == 0 ? [1] : local.len % 2 == 0 ? [local.first, local.second] : [var.num * 2024]
}
