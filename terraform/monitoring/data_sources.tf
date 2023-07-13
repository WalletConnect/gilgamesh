locals {
  prometheus_url = "https://aps-workspaces.${module.this.environment}.amazonaws.com/workspaces/${aws_prometheus_workspace.prometheus.id}/"
}

resource "grafana_data_source" "prometheus" {
  type = "prometheus"
  name = "${module.this.id}-amp"
  url  = local.prometheus_url

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
