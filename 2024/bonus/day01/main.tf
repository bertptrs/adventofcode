variable "input" {
  type = string
}

locals {
  cleaned_input = replace(var.input, "/ +/", " ")
  lines         = split("\n", trim(local.cleaned_input, "\n"))
  lines_split   = [for line in local.lines : split(" ", line)]
  left          = [for line in local.lines_split : tonumber(line[0])]
  right         = [for line in local.lines_split : tonumber(line[1])]

  left_sorted  = sort(local.left)
  right_sorted = sort(local.right)

  diffs = [for i in range(length(local.left_sorted)) : abs(local.left_sorted[i] - local.right_sorted[i])]

  counts = { for num in local.right : num => num... }

  matching = [for left in local.left : left * try(length(local.counts[left]), 0)]
}

output "part1" {
  value = sum(local.diffs)
}

output "part2" {
  value = sum(local.matching)
}
