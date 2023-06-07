data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = "${module.this.stage} - ${module.this.name}"
    dashboard_uid   = "${module.this.stage}-${module.this.name}"

    prometheus_uid = grafana_data_source.prometheus.uid
    cloudwatch_uid = grafana_data_source.cloudwatch.uid

    environment   = module.this.stage
    notifications = jsonencode(local.notifications)

    ecs_service_name = var.ecs_service_name
    load_balancer    = local.load_balancer
    target_group     = local.target_group
    docdb_cluster_id = var.docdb_cluster_id
  }
}

# JSON Dashboard. When exporting from Grafana make sure that all
# variables are replaced properly
resource "grafana_dashboard" "main" {
  overwrite   = true
  message     = "Updated by Terraform"
  config_json = data.jsonnet_file.dashboard.rendered
}
