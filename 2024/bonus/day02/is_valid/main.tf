variable "report" {
  type = list(number)
}

locals {
  delta = [for i in range(1, length(var.report)) : var.report[i] - var.report[i - 1]]

  all_negative = alltrue([for d in local.delta : d <= -1 && d >= -3])
  all_positive = alltrue([for d in local.delta : d >= 1 && d <= 3])
}

output "valid" {
  value = local.all_negative || local.all_positive
}
