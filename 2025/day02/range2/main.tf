variable "min" {
  type = number
}

variable "max" {
  type = number
}

locals {
  digits = length(tostring(var.max))
}

module "range" {
  source      = "../range"
  count       = local.digits
  max         = var.max
  min         = var.min
  repetitions = count.index + 1
}

locals {
  results = setunion(module.range[*].invalid...)
}

output "invalid_sum" {
  value = length(local.results) > 0 ? sum(local.results) : 0
}
