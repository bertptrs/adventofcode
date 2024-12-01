terraform {
  
}

module "day01" {
  source = "./day01"
  input = file("../inputs/01.txt")
}

output "day01_1" {
  value = module.day01.part1
}
