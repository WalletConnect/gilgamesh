locals {
  fqdn                = module.this.stage == "prod" ? var.public_url : "${terraform.workspace}.${var.public_url}"
  latest_release_name = data.github_release.latest_release.name
  version             = coalesce(var.image_version, substr(local.latest_release_name, 1, length(local.latest_release_name)))
}

data "github_release" "latest_release" {
  repository  = "gilgamesh"
  owner       = "walletconnect"
  retrieve_by = "latest"
}

################################################################################
# Networking

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "4.0"

  name = "${module.this.stage}-${module.this.name}"

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
  source  = "app.terraform.io/wallet-connect/dns/aws"
  version = "0.1.0"

  fqdn             = local.fqdn
  hosted_zone_name = var.public_url
}

################################################################################
# Data Stores

module "history_docdb" {
  source     = "./docdb"
  context    = module.this.context
  attributes = ["history-db"]

  default_database            = "history"
  primary_instance_class      = var.docdb_primary_instance_class
  primary_instances           = var.docdb_primary_instances
  vpc_id                      = module.vpc.vpc_id
  private_subnet_ids          = module.vpc.private_subnets
  allowed_ingress_cidr_blocks = [module.vpc.vpc_cidr_block]
  allowed_egress_cidr_blocks  = [module.vpc.vpc_cidr_block]
}


################################################################################
# Application

data "aws_ecr_repository" "repository" {
  name = "gilgamesh"
}

module "ecs" {
  source  = "./ecs"
  context = module.this.context

  prometheus_endpoint             = aws_prometheus_workspace.prometheus.prometheus_endpoint
  image                           = "${data.aws_ecr_repository.repository.repository_url}:${local.version}"
  acm_certificate_arn             = module.dns.certificate_arn
  cpu                             = 1024
  route53-fqdn                    = local.fqdn
  memory                          = 2048
  private_subnets                 = module.vpc.private_subnets
  public_subnets                  = module.vpc.public_subnets
  log_region                      = var.region
  route53-zone_id                 = module.dns.zone_id
  vpc_id                          = module.vpc.vpc_id
  allowed_app_ingress_cidr_blocks = module.vpc.vpc_cidr_block
  allowed_lb_ingress_cidr_blocks  = module.vpc.vpc_cidr_block
  docdb-connection_url            = module.history_docdb.connection_url
}

################################################################################
# Monitoring

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${module.this.id}"
}

module "monitoring" {
  source  = "./monitoring"
  context = module.this.context

  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id

  docdb_cluster_id  = module.history_docdb.cluster_id
  ecs_service_name  = module.ecs.service_name
  load_balancer_arn = module.ecs.load_balancer_arn
  target_group_arn  = module.ecs.target_group_arn
}
