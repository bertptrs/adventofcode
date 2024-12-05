variable "grid" {
  type = list(string)
}

variable "x" {
  type = number
}

variable "y" {
  type = number
}

locals {
  found_a = substr(var.grid[var.y], var.x, 1) == "A"

  c1 = substr(var.grid[var.y - 1], var.x - 1, 1)
  c2 = substr(var.grid[var.y - 1], var.x + 1, 1)
  c3 = substr(var.grid[var.y + 1], var.x + 1, 1)
  c4 = substr(var.grid[var.y + 1], var.x - 1, 1)

  d1 = "${local.c1}${local.c3}"
  d2 = "${local.c2}${local.c4}"

  found_d1 = contains(["SM", "MS"], local.d1)
  found_d2 = contains(["SM", "MS"], local.d2)
}

output "found" {
  value = local.found_a && local.found_d1 && local.found_d2
}
