# `archive-server` deployment

<!-- BEGINNING OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_github"></a> [github](#requirement\_github) | 5.26 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_github"></a> [github](#provider\_github) | 5.26 |

## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_archive_docdb"></a> [archive\_docdb](#module\_archive\_docdb) | ./docdb | n/a |
| <a name="module_dns"></a> [dns](#module\_dns) | app.terraform.io/wallet-connect/dns/aws | 0.1.0 |
| <a name="module_ecs"></a> [ecs](#module\_ecs) | ./ecs | n/a |
| <a name="module_monitoring"></a> [monitoring](#module\_monitoring) | ./monitoring | n/a |
| <a name="module_this"></a> [this](#module\_this) | app.terraform.io/wallet-connect/label/null | 0.2.0 |
| <a name="module_vpc"></a> [vpc](#module\_vpc) | terraform-aws-modules/vpc/aws | 4.0 |

## Resources

| Name | Type |
|------|------|
| [github_release.latest_release](https://registry.terraform.io/providers/integrations/github/5.26/docs/data-sources/release) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_azs"></a> [azs](#input\_azs) | The AWS availability zones to deploy to. | `list(string)` | <pre>[<br>  "eu-central-1a",<br>  "eu-central-1b",<br>  "eu-central-1c"<br>]</pre> | no |
| <a name="input_docdb_primary_instance_class"></a> [docdb\_primary\_instance\_class](#input\_docdb\_primary\_instance\_class) | The instance class of the primary MongoDB server. See https://docs.aws.amazon.com/documentdb/latest/developerguide/db-instance-classes.html#db-instance-class-specs for details. | `string` | n/a | yes |
| <a name="input_docdb_primary_instances"></a> [docdb\_primary\_instances](#input\_docdb\_primary\_instances) | The number of instances in the primary MongoDB server cluster. | `number` | n/a | yes |
| <a name="input_grafana_endpoint"></a> [grafana\_endpoint](#input\_grafana\_endpoint) | The endpoint of the Grafana instance used for monitoring. | `string` | n/a | yes |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | Optional override for the Docker image version to deploy. Default is `latest` | `string` | `""` | no |
| <a name="input_public_url"></a> [public\_url](#input\_public\_url) | The public URL of the service. | `string` | `"archive.walletconnect.com"` | no |
| <a name="input_region"></a> [region](#input\_region) | The AWS region to deploy to. | `string` | `"eu-central-1"` | no |

## Outputs

| Name | Description |
|------|-------------|
| <a name="output_fqdn"></a> [fqdn](#output\_fqdn) | The FQDN of the deployed instance |
| <a name="output_version"></a> [version](#output\_version) | The version of the deployed instance |
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
