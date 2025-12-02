variable "min" {
  type = number
}

variable "max" {
  type = number
}

variable "repetitions" {
  type    = number
  default = 2
}

locals {
  digits       = floor(length(tostring(var.max)) / var.repetitions)
  maximum      = substr(tostring(var.max), 0, local.digits)
  real_maximum = length(tostring(var.max)) % var.repetitions == 0 ? tonumber(local.maximum) : pow(10, local.digits)

  min_digits = max(floor(length(tostring(var.min)) / var.repetitions), 1)
  minimum    = tonumber(substr(tostring(var.min), 0, local.min_digits))

  count = max(local.real_maximum - local.minimum + 1, 1)

  can_work = anytrue([for n in range(length(tostring(var.min)), length(tostring(var.max)) + 1) : n % var.repetitions == 0])
}

// This "candidates" module ought really be a list comprehension from range, but Terraform does not
// allow you to create ranges longer than 1024.
module "candidates" {
  source      = "./item"
  count       = local.can_work ? local.count : 0
  part        = count.index + local.minimum
  repetitions = var.repetitions
}

locals {
  invalid = [for n in module.candidates[*].full : n if n >= var.min && n <= var.max]
}

output "invalid_sum" {
  value = length(local.invalid) > 0 ? sum(local.invalid) : 0
}

output "invalid" {
  value = toset(local.invalid)
}
