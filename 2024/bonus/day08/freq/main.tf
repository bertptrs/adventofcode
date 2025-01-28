variable "width" {
  type = number
}

variable "height" {
  type = number
}

variable "antennae" {
  type = list(tuple([number, number]))
}

locals {
  pairs = concat([
    for i in range(length(var.antennae)) :
    [
      for j in range(i + 1, length(var.antennae)) : [var.antennae[i], var.antennae[j]]
    ]
  ]...)
}

module "pair" {
  source = "./pair"
  count  = length(local.pairs)

  first  = local.pairs[count.index][0]
  second = local.pairs[count.index][1]
  width  = var.width
  height = var.height
}

output "nodes1" {
  value = setunion([
    for i in range(length(local.pairs)) :
    [
      for v in module.pair[i].nodes1 :
      v
      if v[0] >= 0 && v[0] < var.width && v[1] >= 0 && v[1] < var.height
    ]
  ]...)
}

output "nodes2" {
  value = setunion([
    for i in range(length(local.pairs)) :
    [
      for v in module.pair[i].nodes2 :
      v
      if v[0] >= 0 && v[0] < var.width && v[1] >= 0 && v[1] < var.height
    ]
  ]...)
}
