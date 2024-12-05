variable "input" {
  type = string
}

locals {
  grid   = split("\n", chomp(var.input))
  height = length(local.grid)
  width  = length(local.grid[0])
}

module "check_point" {
  source = "./check_point"

  count = local.width * local.height

  width  = local.width
  height = local.height
  grid   = local.grid
  index  = count.index
}

output "part1" {
  value = sum(module.check_point[*].xmas)
}

output "part2" {
  value = sum(module.check_point[*].x_mas)
}
