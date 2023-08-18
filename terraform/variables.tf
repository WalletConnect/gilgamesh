variable "region" {
  description = "The AWS region to deploy to."
  type        = string
  default     = "eu-central-1"
}

variable "azs" {
  description = "The AWS availability zones to deploy to."
  type        = list(string)
  default     = ["eu-central-1a", "eu-central-1b", "eu-central-1c"]
}

variable "public_url" {
  description = "The public URL of the service."
  type        = string
  default     = "archive.walletconnect.com"
}

# Expects GRAFANA_AUTH env variable to be set
variable "grafana_endpoint" {
  description = "The endpoint of the Grafana instance used for monitoring."
  type        = string
}

variable "image_version" {
  description = "Optional override for the Docker image version to deploy. Default is `latest`"
  type        = string
  default     = ""
}

variable "docdb_primary_instance_class" {
  description = "The instance class of the primary MongoDB server. See https://docs.aws.amazon.com/documentdb/latest/developerguide/db-instance-classes.html#db-instance-class-specs for details."
  type        = string
}

variable "docdb_primary_instances" {
  description = "The number of instances in the primary MongoDB server cluster."
  type        = number
}
