locals {
  opsgenie_notification_channel = "l_iaPw6nk"
  notifications = (
    var.environment == "prod" ?
    "[{\"uid\": \"${local.opsgenie_notification_channel}\"}]" :
    "[]"
  )
}

resource "grafana_data_source" "prometheus" {
  type = "prometheus"
  name = "${var.environment}-${var.app_name}-amp"
  url  = "https://aps-workspaces.eu-central-1.amazonaws.com/workspaces/${var.prometheus_workspace_id}/"

  json_data_encoded = jsonencode({
    httpMethod    = "GET"
    manageAlerts  = false
    sigV4Auth     = true
    sigV4AuthType = "ec2_iam_role"
    sigV4Region   = "eu-central-1"
  })
}

resource "grafana_data_source" "cloudwatch" {
  type = "cloudwatch"
  name = "${var.environment}-${var.app_name}-cloudwatch"

  json_data_encoded = jsonencode({
    defaultRegion = "eu-central-1"
  })
}

# JSON Dashboard. When exporting from Grafana make sure that all
# variables are replaced properly
resource "grafana_dashboard" "at_a_glance" {
  overwrite = true
  message   = "Updated by Terraform"
  config_json = jsonencode({
    annotations : {
      list : [
        {
          builtIn : 1,
          datasource : "-- Grafana --",
          enable : true,
          hide : true,
          iconColor : "rgba(0, 211, 255, 1)",
          name : "Annotations & Alerts",
          target : {
            limit : 100,
            matchAny : false,
            tags : [],
            type : "dashboard"
          },
          type : "dashboard"
        }
      ]
    },
    editable : true,
    fiscalYearStartMonth : 0,
    graphTooltip : 0,
    id : 19,
    links : [],
    liveNow : false,
    panels : [],

    schemaVersion : 36,
    style : "dark",
    tags : [],
    templating : {
      list : []
    },
    time : {
      from : "now-6h",
      to : "now"
    },
    timepicker : {},
    timezone : "",
    title : "${var.environment}_${var.app_name}",
    uid : "${var.environment}_${var.app_name}",
    version : 1,
    weekStart : ""
  })
}
