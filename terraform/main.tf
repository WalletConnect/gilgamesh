locals {
  app_name            = "history"
  fqdn                = terraform.workspace == "prod" ? var.public_url : "${terraform.workspace}.${var.public_url}"
  latest_release_name = data.github_release.latest_release.name
  version             = coalesce(var.image_version, substr(local.latest_release_name, 1, length(local.latest_release_name)))
}

data "github_release" "latest_release" {
  repository  = "gilgamesh"
  owner       = "walletconnect"
  retrieve_by = "latest"
}

////////////////////////////////////////////////////////////////////////////////
// Networking

module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  name   = "${terraform.workspace}-${local.app_name}"

  cidr = "10.0.0.0/16"

  azs             = var.azs
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.4.0/24", "10.0.5.0/24", "10.0.6.0/24"]

  private_subnet_tags = {
    Visibility = "private"
  }
  public_subnet_tags = {
    Visibility = "public"
  }

  enable_dns_support     = true
  enable_dns_hostnames   = true
  enable_nat_gateway     = true
  single_nat_gateway     = true
  one_nat_gateway_per_az = false
}

module "dns" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/dns"

  hosted_zone_name = var.public_url
  fqdn             = local.fqdn
}

////////////////////////////////////////////////////////////////////////////////
// Data Stores

module "keystore-docdb" {
  source = "./docdb"

  app_name                    = local.app_name
  mongo_name                  = "keystore-docdb"
  environment                 = terraform.workspace
  default_database            = "keystore"
  primary_instance_class      = var.docdb_primary_instance_class
  primary_instances           = var.docdb_primary_instances
  vpc_id                      = module.vpc.vpc_id
  private_subnet_ids          = module.vpc.private_subnets
  allowed_ingress_cidr_blocks = [module.vpc.vpc_cidr_block]
  allowed_egress_cidr_blocks  = [module.vpc.vpc_cidr_block]
}


////////////////////////////////////////////////////////////////////////////////
// Application

data "aws_ecr_repository" "repository" {
  name = "gilgamesh"
}

module "ecs" {
  source = "./ecs"

  app_name            = "${terraform.workspace}-${local.app_name}"
  prometheus_endpoint = aws_prometheus_workspace.prometheus.prometheus_endpoint
  image               = "${data.aws_ecr_repository.repository.repository_url}:${replace(local.version, "v", "")}"
  acm_certificate_arn = module.dns.certificate_arn
  cpu                 = 512
  fqdn                = local.fqdn
  memory              = 1024
  private_subnets     = module.vpc.private_subnets
  public_subnets      = module.vpc.public_subnets
  region              = var.region
  route53_zone_id     = module.dns.zone_id
  vpc_cidr            = module.vpc.vpc_cidr_block
  vpc_id              = module.vpc.vpc_id
  mongo_address       = module.keystore-docdb.connection_url
}

////////////////////////////////////////////////////////////////////////////////
// Monitoring

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${terraform.workspace}-${local.app_name}"
}

module "o11y" {
  source = "./monitoring"

  environment             = terraform.workspace
  app_name                = local.app_name
  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id
}
