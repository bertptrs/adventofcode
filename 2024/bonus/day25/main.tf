variable "input" {
  type = string
}

locals {
  blocks = split("\n\n", chomp(var.input))
  heights = [
    for block in local.blocks : [
      for i in range(5) : length([
        for line in split("\n", block) : line if substr(line, i, 1) == "#"
      ])
    ]
  ]

  locks = [for i in range(length(local.blocks)) : local.heights[i] if startswith(local.blocks[i], "#####")]
  keys  = [for i in range(length(local.blocks)) : local.heights[i] if !startswith(local.blocks[i], "#####")]

  combined = concat([for lock in local.locks : [for key in local.keys : [for i in range(5) : lock[i] + key[i] <= 7]]]...)
}

output "part1" {
  value = length([for combination in local.combined : combination if alltrue(combination)])
}
