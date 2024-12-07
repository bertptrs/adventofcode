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
