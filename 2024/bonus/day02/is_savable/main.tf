variable "report" {
  type = list(number)
}

module "is_valid" {
  source = "../is_valid"
  count  = length(var.report)

  report = concat(
    count.index > 0 ? slice(var.report, 0, count.index) : [],
    count.index < length(var.report) - 1 ? slice(var.report, count.index + 1, length(var.report)) : []
  )
}

output "valid" {
  value = anytrue(module.is_valid[*].valid)
}
