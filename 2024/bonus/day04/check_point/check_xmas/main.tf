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
  match = [for i in range(4) : var.x + i * var.dx >= 0 && try(substr(var.grid[var.y + i * var.dy], var.x + i * var.dx, 1), "F") == substr("XMAS", i, 1)]
}

output "found" {
  value = alltrue(local.match) ? 1 : 0
}
