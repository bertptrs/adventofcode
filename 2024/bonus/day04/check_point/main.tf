variable "grid" {
  type = list(string)
}

variable "index" {
  type = number
}

variable "width" {
  type = number
}

variable "height" {
  type = number
}

locals {
  x = var.index % var.width
  y = floor(var.index / var.width)

  directions = {
    "UL" = [-1, -1]
    "U"  = [0, -1]
    "UR" = [1, -1]
    "DL" = [-1, 1]
    "D"  = [0, 1]
    "DR" = [1, 1]
  }

  should_check_x_mas = local.x >= 1 && local.y >= 1 && local.x < var.width - 1 && local.y < var.height - 1
}

module "check_xmas" {
  source   = "./check_xmas"
  for_each = local.directions

  grid = var.grid

  x = local.x
  y = local.y

  dx = each.value[0]
  dy = each.value[1]
}

module "check_x_mas" {
  source = "./check_x_mas"
  count  = local.should_check_x_mas ? 1 : 0

  grid = var.grid

  y = local.y
  x = local.x
}

output "xmas" {
  value = sum([for _, v in module.check_xmas : v.found])
}

output "x_mas" {
  value = try(module.check_x_mas[0].found, false) ? 1 : 0
}
