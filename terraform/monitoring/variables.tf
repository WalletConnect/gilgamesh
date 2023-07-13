variable "ecs_service_name" {
  description = "The name of the ECS service."
  type        = string
}

variable "target_group_arn" {
  description = "The ARN of the target group."
  type        = string
}

variable "load_balancer_arn" {
  description = "The ARN of the load balancer."
  type        = string
}

variable "docdb_cluster_id" {
  description = "The ID of the DocumentDB cluster."
  type        = string
}

variable "grafana_endpoint" {
  description = "The endpoint of the Grafana instance used for monitoring."
  type        = string
}
