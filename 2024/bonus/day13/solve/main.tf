variable "machines" {
  type = list(list(number))
}

locals {
  a_substitutions = [
    for machine in var.machines :
    [-machine[2] / machine[0], machine[4] / machine[0]]
  ]

  b_equations = [
    for i in range(length(var.machines)) :
    [
      var.machines[i][3] + local.a_substitutions[i][0] * var.machines[i][1],
      var.machines[i][5] - local.a_substitutions[i][1] * var.machines[i][1]
    ]
  ]

  b = [for eq in local.b_equations : floor(eq[1] / eq[0] + 0.5)]

  a = [
    for i in range(length(var.machines)) :
    floor((var.machines[i][4] - local.b[i] * var.machines[i][2]) / var.machines[i][0] + 0.5)
  ]
}

output "solutions" {
  value = sum([
    for i in range(length(var.machines)) :
    3 * local.a[i] + local.b[i]
    if var.machines[i][0] * local.a[i] + var.machines[i][2] * local.b[i] == var.machines[i][4]
    && var.machines[i][1] * local.a[i] + var.machines[i][3] * local.b[i] == var.machines[i][5]
  ])
}
