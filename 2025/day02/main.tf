locals {
  input   = file("../inputs/02.txt")
  ranges  = split(",", chomp(local.input))
  min_max = [for r in local.ranges : split("-", r)]
}

module "check_range" {
  source = "./range"
  count  = length(local.min_max)

  min = local.min_max[count.index][0]
  max = local.min_max[count.index][1]
}

module "check_range2" {
  source = "./range2"
  count  = length(local.min_max)

  min = local.min_max[count.index][0]
  max = local.min_max[count.index][1]
}

output "part1" {
  value = sum(module.check_range[*].invalid_sum)
}

output "part2" {
  value = sum(module.check_range2[*].invalid_sum)
}
