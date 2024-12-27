variable "input" {
  type = string
}

variable "width" {
  type    = number
  default = 101
}

variable "height" {
  type    = number
  default = 103
}

locals {
  lines = regexall("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)", var.input)
  positions = [
    for line in local.lines :
    [
      ((line[0] + 100 * line[2]) % var.width + var.width) % var.width,
      ((line[1] + 100 * line[3]) % var.height + var.height) % var.height,
    ]
  ]

  q1 = length([for pos in local.positions : pos if pos[0] < floor(var.width / 2) && pos[1] < floor(var.height / 2)])
  q2 = length([for pos in local.positions : pos if pos[0] > floor(var.width / 2) && pos[1] < floor(var.height / 2)])
  q3 = length([for pos in local.positions : pos if pos[0] < floor(var.width / 2) && pos[1] > floor(var.height / 2)])
  q4 = length([for pos in local.positions : pos if pos[0] > floor(var.width / 2) && pos[1] > floor(var.height / 2)])
}

output "part1" {
  value = local.q1 * local.q2 * local.q3 * local.q4
}
