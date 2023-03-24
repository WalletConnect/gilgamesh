variable "primary_instance_class" {
  description = "The instance class of the primary MongoDB server. See https://docs.aws.amazon.com/documentdb/latest/developerguide/db-instance-classes.html#db-instance-class-specs for details."
  type        = string
}

variable "primary_instances" {
  description = "The number of instances in the primary MongoDB server cluster."
  type        = number
}

variable "vpc_id" {
  description = "The VPC ID to deploy the MongoDB server into."
  type        = string
}

variable "private_subnet_ids" {
  description = "A list of private subnet IDs to deploy the MongoDB server into."
  type        = list(string)
}

variable "allowed_ingress_cidr_blocks" {
  description = "A list of CIDR blocks to allow ingress access to the MongoDB server."
  type        = list(string)
}

variable "allowed_egress_cidr_blocks" {
  description = "A list of CIDR blocks to egress access from the MongoDB server."
  type        = list(string)
}

variable "default_database" {
  description = "The name of the default database in MongoDB."
  type        = string
}

variable "port" {
  description = "The port to use for the MongoDB server."
  type        = number
  default     = 27017
}
