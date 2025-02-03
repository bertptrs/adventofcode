variable "first" {
  type = tuple([number, number])
}

variable "second" {
  type = tuple([number, number])
}

variable "width" {
  type = number
}

variable "height" {
  type = number
}

locals {
  dx = var.second[0] - var.first[0]
  dy = var.second[1] - var.first[1]

  sorted = abs(local.dx) < abs(local.dy) ? [abs(local.dx), abs(local.dy)] : [abs(local.dy), abs(local.dx)]
  gcd0   = local.sorted[0] == 0 ? [0, 1] : local.sorted

  # Do as many iterations as necessary. 
  gcd1  = local.gcd0[0] == 0 ? local.gcd0 : [local.gcd0[1] % local.gcd0[0], local.gcd0[0]]
  gcd2  = local.gcd1[0] == 0 ? local.gcd1 : [local.gcd1[1] % local.gcd1[0], local.gcd1[0]]
  gcd3  = local.gcd2[0] == 0 ? local.gcd2 : [local.gcd2[1] % local.gcd2[0], local.gcd2[0]]
  gcd4  = local.gcd3[0] == 0 ? local.gcd3 : [local.gcd3[1] % local.gcd3[0], local.gcd3[0]]
  gcd5  = local.gcd4[0] == 0 ? local.gcd4 : [local.gcd4[1] % local.gcd4[0], local.gcd4[0]]
  gcd6  = local.gcd5[0] == 0 ? local.gcd5 : [local.gcd5[1] % local.gcd5[0], local.gcd5[0]]
  gcd7  = local.gcd6[0] == 0 ? local.gcd6 : [local.gcd6[1] % local.gcd6[0], local.gcd6[0]]
  gcd8  = local.gcd7[0] == 0 ? local.gcd7 : [local.gcd7[1] % local.gcd7[0], local.gcd7[0]]
  gcd9  = local.gcd8[0] == 0 ? local.gcd8 : [local.gcd8[1] % local.gcd8[0], local.gcd8[0]]
  gcd10 = local.gcd9[0] == 0 ? local.gcd9 : [local.gcd9[1] % local.gcd9[0], local.gcd9[0]]

  # 10 iterations should cover numbers up to 55, which is more than the width/height
  gcd = local.gcd10[1]
}

output "nodes1" {
  value = [
    [var.first[0] - local.dx, var.first[1] - local.dy],
    [var.second[0] + local.dx, var.second[1] + local.dy],
  ]
}

output "nodes2" {
  value = concat(
    [for i in range(max(var.width, var.height)) : [var.first[0] - i * local.dx / local.gcd, var.first[1] - i * local.dy / local.gcd]],
    [for i in range(max(var.width, var.height)) : [var.second[0] + i * local.dx / local.gcd, var.second[1] + i * local.dy / local.gcd]]
  )
}
