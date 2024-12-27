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

output "day03_2" {
  value = module.day03.part2
}

# Don't run this, it runs forever (6 minutes) and requires a lot of memory (~5.5GB) to execute to
# boot. Trust me, it works.

# module "day04" {
#   source = "./day04"
#   input  = file("../inputs/04.txt")
# }

# output "day04_1" {
#   value = module.day04.part1
# }

# output "day04_2" {
#   value = module.day04.part2
# }

module "day05" {
  source = "./day05"
  input  = file("../inputs/05.txt")
}

output "day05_1" {
  value = module.day05.part1
}

module "day11" {
  source = "./day11"
  input  = file("../inputs/11.txt")
}

output "day11_1" {
  value = module.day11.part1
}

output "day11_2" {
  value = module.day11.part2
}

module "day13" {
  source = "./day13"
  input  = file("../inputs/13.txt")
}

output "day13_1" {
  value = module.day13.part1
}

output "day13_2" {
  value = module.day13.part2
}

module "day14" {
  source = "./day14"
  input  = file("../inputs/14.txt")
}

output "day14_1" {
  value = module.day14.part1
}

module "day19" {
  source = "./day19"
  input  = file("../inputs/19.txt")
}

output "day19_1" {
  value = module.day19.part1
}

module "day25" {
  source = "./day25"
  input  = file("../inputs/25.txt")
}

output "day25_1" {
  value = module.day25.part1
}
