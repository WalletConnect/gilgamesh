terraform {
  required_version = "~> 1.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.50"
    }
    grafana = {
      source  = "grafana/grafana"
      version = "~> 2.0"
    }
    jsonnet = {
      source  = "alxrem/jsonnet"
      version = "~> 2.2"
    }
  }
}

provider "aws" {
  region = module.this.environment

  default_tags {
    tags = module.this.tags
  }
}

provider "grafana" {
  url = "https://${var.grafana_endpoint}"
}
