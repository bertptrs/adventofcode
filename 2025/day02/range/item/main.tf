variable "part" {
  type = number
}

variable "repetitions" {
  type = number
}

locals {
  repeated = [for _ in range(var.repetitions) : tostring(var.part)]
}

output "full" {
  value = join("", local.repeated)
}
