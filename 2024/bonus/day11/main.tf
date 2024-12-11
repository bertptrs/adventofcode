variable "input" {
  type = string
}

locals {
  nums = [for s in split(" ", chomp(var.input)) : tonumber(s)]

  grouped = { for num in local.nums : num => 1... }
  total   = { for k, v in local.grouped : k => sum(v) }
}

module "step1" {
  source = "./step"

  prev = local.total
}

module "step2" {
  source = "./step"

  prev = module.step1.next
}

module "step3" {
  source = "./step"

  prev = module.step2.next
}

module "step4" {
  source = "./step"

  prev = module.step3.next
}

module "step5" {
  source = "./step"

  prev = module.step4.next
}

module "step6" {
  source = "./step"

  prev = module.step5.next
}

module "step7" {
  source = "./step"

  prev = module.step6.next
}

module "step8" {
  source = "./step"

  prev = module.step7.next
}

module "step9" {
  source = "./step"

  prev = module.step8.next
}

module "step10" {
  source = "./step"

  prev = module.step9.next
}

module "step11" {
  source = "./step"

  prev = module.step10.next
}

module "step12" {
  source = "./step"

  prev = module.step11.next
}

module "step13" {
  source = "./step"

  prev = module.step12.next
}

module "step14" {
  source = "./step"

  prev = module.step13.next
}

module "step15" {
  source = "./step"

  prev = module.step14.next
}

module "step16" {
  source = "./step"

  prev = module.step15.next
}

module "step17" {
  source = "./step"

  prev = module.step16.next
}

module "step18" {
  source = "./step"

  prev = module.step17.next
}

module "step19" {
  source = "./step"

  prev = module.step18.next
}

module "step20" {
  source = "./step"

  prev = module.step19.next
}

module "step21" {
  source = "./step"

  prev = module.step20.next
}

module "step22" {
  source = "./step"

  prev = module.step21.next
}

module "step23" {
  source = "./step"

  prev = module.step22.next
}

module "step24" {
  source = "./step"

  prev = module.step23.next
}

module "step25" {
  source = "./step"

  prev = module.step24.next
}

output "part1" {
  value = sum(values(module.step25.next))
}
