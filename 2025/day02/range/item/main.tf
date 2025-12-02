variable "half" {
  type = number
}

output "full" {
  value = tonumber("${var.half}${var.half}")
}
