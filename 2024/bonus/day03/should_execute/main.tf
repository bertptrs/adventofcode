variable "ops" {
  type = list(list(string))
}

variable "index" {
  type = number
}

locals {
  is_mul   = startswith(var.ops[var.index][0], "mul")
  subslice = reverse(slice(var.ops[*][0], 0, var.index))

  do_pos   = try(index(local.subslice, "do()"), var.index)
  dont_pos = try(index(local.subslice, "don't()"), var.index + 1)
}

output "value" {
  value = (local.is_mul && local.do_pos < local.dont_pos) ? (parseint(var.ops[var.index][1], 10) * parseint(var.ops[var.index][2], 10)) : 0
}
