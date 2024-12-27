variable "input" {
  type    = string
  default = <<-EOT
  Button A: X+94, Y+34
  Button B: X+22, Y+67
  Prize: X=8400, Y=5400
  
  Button A: X+26, Y+66
  Button B: X+67, Y+21
  Prize: X=12748, Y=12176
  
  Button A: X+17, Y+86
  Button B: X+84, Y+37
  Prize: X=7870, Y=6450
  
  Button A: X+69, Y+23
  Button B: X+27, Y+71
  Prize: X=18641, Y=10279
  EOT
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
    [machine[0], machine[1], machine[2], machine[3], 10000000000000 + tonumber(machine[4]), 10000000000000 + tonumber(machine[5])]
  ]
}

output "part1" {
  value = module.solve1.solutions
}

output "part2" {
  value = module.solve2.solutions
}
