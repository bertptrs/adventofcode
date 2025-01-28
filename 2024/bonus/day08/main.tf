variable "input" {
  type    = string
  default = <<-EOT
  ............
  ........0...
  .....0......
  .......0....
  ....0.......
  ......A.....
  ............
  ............
  ........A...
  .........A..
  ............
  ............
  EOT

}

locals {
  lines  = split("\n", chomp(var.input))
  height = length(local.lines)
  width  = length(local.lines[0])

  antennae = concat([
    for y in range(local.height) :
    [
      for x in range(local.width) :
      [substr(local.lines[y], x, 1), x, y]
      if substr(local.lines[y], x, 1) != "."
    ]
  ]...)

  by_freq = {
    for antenna in local.antennae :
    antenna[0] => [antenna[1], antenna[2]]...
  }
}

module "freq" {
  source   = "./freq"
  for_each = local.by_freq
  width    = local.width
  height   = local.height
  antennae = each.value
}

output "part1" {
  value = length(setunion([for _, v in module.freq : v.nodes1]...))
}

output "part2" {
  value = length(setunion([for _, v in module.freq : v.nodes2]...))
}
