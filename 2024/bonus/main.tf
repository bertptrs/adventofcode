terraform {

}

module "day01" {
  source = "./day01"
  input  = file("../inputs/01.txt")
}

output "day01_1" {
  value = module.day01.part1
}

output "day01_2" {
  value = module.day01.part2
}

module "day02" {
  source = "./day02"
  input  = file("../inputs/02.txt")
}

output "day02_1" {
  value = module.day02.part1
}

output "day02_2" {
  value = module.day02.part2
}

module "day03" {
  source = "./day03"
  input  = file("../inputs/03.txt")
}

output "day03_1" {
  value = module.day03.part1
}
