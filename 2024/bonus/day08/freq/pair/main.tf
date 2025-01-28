variable "first" {
  type = tuple([number, number])
}

variable "second" {
  type = tuple([number, number])
}

variable "width" {
  type = number
}

variable "height" {
  type = number
}

locals {
  dx = var.second[0] - var.first[0]
  dy = var.second[1] - var.first[1]
}

output "nodes1" {
  value = [
    [var.first[0] - local.dx, var.first[1] - local.dy],
    [var.second[0] + local.dx, var.second[1] + local.dy],
  ]
}

output "nodes2" {
  value = concat(
    [for i in range(max(var.width, var.height)) : [var.first[0] - i * local.dx, var.first[1] - i * local.dy]],
    [for i in range(max(var.width, var.height)) : [var.second[0] + i * local.dx, var.second[1] + i * local.dy]]
  )
}
