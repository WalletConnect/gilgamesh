#locals {
#  opsgenie_notification_channel = "l_iaPw6nk"
#  notifications                 = (var.environment == "prod" ? [{ uid : local.opsgenie_notification_channel }] : [])
#}

data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = "${module.this.stage} - ${module.this.name}"
    dashboard_uid   = "${module.this.stage}-${module.this.name}"

    prometheus_uid = grafana_data_source.prometheus.uid
  }
}

# JSON Dashboard. When exporting from Grafana make sure that all
# variables are replaced properly
resource "grafana_dashboard" "main" {
  overwrite   = true
  message     = "Updated by Terraform"
  config_json = data.jsonnet_file.dashboard.rendered
}
