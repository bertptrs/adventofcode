variable "grid" {
  type = list(string)
}

variable "x" {
  type = number
}

variable "y" {
  type = number
}

variable "dx" {
  type = number
}

variable "dy" {
  type = number
}

locals {
  word = join("", [for i in range(4) : var.x + i * var.dx >= 0 ? try(substr(var.grid[var.y + i * var.dy], var.x + i * var.dx, 1), "F") : "F"])
}

output "found" {
  value = contains(["SAMX", "XMAS"], local.word) ? 1 : 0
}
