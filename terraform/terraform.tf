# Terraform Configuration
terraform {
  required_version = "~> 1.0"

  backend "remote" {
    hostname     = "app.terraform.io"
    organization = "wallet-connect"
    workspaces {
      prefix = "archive-"
    }
  }

  required_providers {
    github = {
      source  = "integrations/github"
      version = "5.26"
    }
  }
}
