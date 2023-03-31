locals {
  #  opsgenie_notification_channel = "l_iaPw6nk"
  #  notifications = (
  #    module.this.stage == "prod" ?
  #    "[{\"uid\": \"${local.opsgenie_notification_channel}\"}]" :
  #    "[]"
  #  )
}

resource "grafana_data_source" "prometheus" {
  type = "prometheus"
  name = "${module.this.id}-amp"
  url  = "https://aps-workspaces.${module.this.environment}.amazonaws.com/workspaces/${var.prometheus_workspace_id}/"

  json_data_encoded = jsonencode({
    httpMethod    = "GET"
    manageAlerts  = false
    sigV4Auth     = true
    sigV4AuthType = "ec2_iam_role"
    sigV4Region   = module.this.environment
  })
}

resource "grafana_data_source" "cloudwatch" {
  type = "cloudwatch"
  name = "${module.this.id}-cloudwatch"

  json_data_encoded = jsonencode({
    defaultRegion = module.this.environment
  })
}

data "jsonnet_file" "dashboard" {
  source = "${path.module}/dashboard.jsonnet"

  ext_str = {
    dashboard_title = module.this.id
    prometheus_uid = grafana_data_source.prometheus.uid
  }
}

# JSON Dashboard. When exporting from Grafana make sure that all
# variables are replaced properly
resource "grafana_dashboard" "at_a_glance" {
  overwrite = true
  message   = "Updated by Terraform"
  config_json = data.jsonnet_file.dashboard.rendered
}
