variable "input" {
  type = string
}

locals {
  machines = regexall(
    "Button A: X\\+(\\d+), Y\\+(\\d+)\nButton B: X\\+(\\d+), Y\\+(\\d+)\nPrize: X=(\\d+), Y=(\\d+)",
    var.input
  )
}

module "solve1" {
  source   = "./solve"
  machines = local.machines
}

module "solve2" {
  source = "./solve"
  machines = [
    for machine in local.machines :
    [machine[0], machine[1], machine[2], machine[3], 10000000000000 + machine[4], 10000000000000 + machine[5]]
  ]
}

output "part1" {
  value = module.solve1.solutions
}

output "part2" {
  value = module.solve2.solutions
}
