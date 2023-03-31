variable "log_region" {
  description = "The region to send Cloudwatch logs to."
  type        = string
}

variable "log_level" {
  description = "The log level to use for the application."
  type        = string
  default     = "INFO"
}

variable "image" {
  description = "The name of the ECR image to use for the container."
  type        = string
}

variable "cpu" {
  description = "The number of CPU units to reserve for the container."
  type        = number
}

variable "memory" {
  description = "The amount of memory (in MiB) to reserve for the container."
  type        = number
}

variable "prometheus_endpoint" {
  description = "The endpoint of the Prometheus instance to collect metrics."
  type        = string
}

variable "vpc_id" {
  description = "The ID of the VPC to deploy the container into."
  type        = string
}

variable "allowed_app_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the application server."
  type        = string
}

variable "allowed_lb_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the load-balancer."
  type        = string
}

variable "public_subnets" {
  description = "A list of public subnets to deploy the load-balancer into."
  type        = set(string)
}

variable "private_subnets" {
  description = "A list of private subnets to deploy the application server into."
  type        = set(string)
}

variable "route53-zone_id" {
  description = "The ID of the Route53 zone to create the LB DNS record in."
  type        = string
}

variable "route53-fqdn" {
  description = "The FQDN of the Route53 record to create for the LB."
  type        = string
}

variable "acm_certificate_arn" {
  description = "The ARN of the ACM certificate to use for the LB."
  type        = string
}

variable "docdb-connection_url" {
  description = "The connection URL of the DocumentDB instance to use."
  type        = string
}
