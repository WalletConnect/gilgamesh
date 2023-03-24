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

    schemaVersion : 1,
    version : 1,
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
    title : module.this.id,
    uid : module.this.id,
    weekStart : ""

    panels : [
      {
        "id" : 1,
        "title" : "Received Items per Hour",
        "type" : "stat"
        "pluginVersion" : "8.4.7",
        "datasource" : {
          "type" : "prometheus",
          "uid" : grafana_data_source.prometheus.uid
        },
        "description" : "The number of items received from relay",
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "thresholds"
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "x" : 0, "y" : 0
          "h" : 8, "w" : 10,
        },
        "options" : {
          "colorMode" : "value",
          "graphMode" : "area",
          "justifyMode" : "auto",
          "orientation" : "auto",
          "reduceOptions" : {
            "calcs" : [
              "lastNotNull"
            ],
            "fields" : "",
            "values" : false
          },
          "text" : {},
          "textMode" : "auto"
        },
        "targets" : [
          {
            "datasource" : {
              "type" : "prometheus",
              "uid" : grafana_data_source.prometheus.uid
            },
            "exemplar" : true,
            "expr" : "sum(rate(received_items{}[1h]))",
            "interval" : "",
            "legendFormat" : "",
            "refId" : "A"
          }
        ],
      },
      {
        "id" : 2,
        "title" : "Stored Items per Hour",
        "type" : "stat"
        "pluginVersion" : "8.4.7",
        "datasource" : {
          "type" : "prometheus",
          "uid" : grafana_data_source.prometheus.uid
        },
        "description" : "The number of items actually stored in the database",
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "thresholds"
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "x" : 10, "y" : 0
          "h" : 8, "w" : 10,
        },
        "options" : {
          "colorMode" : "value",
          "graphMode" : "area",
          "justifyMode" : "auto",
          "orientation" : "auto",
          "reduceOptions" : {
            "calcs" : [
              "lastNotNull"
            ],
            "fields" : "",
            "values" : false
          },
          "text" : {},
          "textMode" : "auto"
        },
        "targets" : [
          {
            "datasource" : {
              "type" : "prometheus",
              "uid" : grafana_data_source.prometheus.uid
            },
            "exemplar" : true,
            "expr" : "sum(rate(stored_items{}[1h]))",
            "interval" : "",
            "legendFormat" : "",
            "refId" : "A"
          }
        ],
      },

      {
        "id" : 3,
        "title" : "Get Queries per Hour",
        "type" : "stat"
        "pluginVersion" : "8.4.7",
        "datasource" : {
          "type" : "prometheus",
          "uid" : grafana_data_source.prometheus.uid
        },
        "description" : "The number of items retrieval queries",
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "thresholds"
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "x" : 0, "y" : 8
          "h" : 8, "w" : 10,
        },
        "options" : {
          "colorMode" : "value",
          "graphMode" : "area",
          "justifyMode" : "auto",
          "orientation" : "auto",
          "reduceOptions" : {
            "calcs" : [
              "lastNotNull"
            ],
            "fields" : "",
            "values" : false
          },
          "text" : {},
          "textMode" : "auto"
        },
        "targets" : [
          {
            "datasource" : {
              "type" : "prometheus",
              "uid" : grafana_data_source.prometheus.uid
            },
            "exemplar" : true,
            "expr" : "sum(rate(get_queries{}[1h]))",
            "interval" : "",
            "legendFormat" : "",
            "refId" : "A"
          }
        ],
      },
      {
        "id" : 4,
        "title" : "Served Items per Hour",
        "type" : "stat"
        "pluginVersion" : "8.4.7",
        "datasource" : {
          "type" : "prometheus",
          "uid" : grafana_data_source.prometheus.uid
        },
        "description" : "The number of messages served to clients",
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "thresholds"
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "x" : 10, "y" : 8
          "h" : 8, "w" : 10,
        },
        "options" : {
          "colorMode" : "value",
          "graphMode" : "area",
          "justifyMode" : "auto",
          "orientation" : "auto",
          "reduceOptions" : {
            "calcs" : [
              "lastNotNull"
            ],
            "fields" : "",
            "values" : false
          },
          "text" : {},
          "textMode" : "auto"
        },
        "targets" : [
          {
            "datasource" : {
              "type" : "prometheus",
              "uid" : grafana_data_source.prometheus.uid
            },
            "exemplar" : true,
            "expr" : "sum(rate(served_items{}[1h]))",
            "interval" : "",
            "legendFormat" : "",
            "refId" : "A"
          }
        ],
      },

      {
        "id" : 5,
        "title" : "Cached Registrations per Hour",
        "type" : "stat"
        "pluginVersion" : "8.4.7",
        "datasource" : {
          "type" : "prometheus",
          "uid" : grafana_data_source.prometheus.uid
        },
        "description" : "The number of registrations retrieved from the in-memory cache",
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "thresholds"
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "x" : 0, "y" : 8
          "h" : 16, "w" : 10,
        },
        "options" : {
          "colorMode" : "value",
          "graphMode" : "area",
          "justifyMode" : "auto",
          "orientation" : "auto",
          "reduceOptions" : {
            "calcs" : [
              "lastNotNull"
            ],
            "fields" : "",
            "values" : false
          },
          "text" : {},
          "textMode" : "auto"
        },
        "targets" : [
          {
            "datasource" : {
              "type" : "prometheus",
              "uid" : grafana_data_source.prometheus.uid
            },
            "exemplar" : true,
            "expr" : "sum(rate(cached_registrations{}[1h]))",
            "interval" : "",
            "legendFormat" : "",
            "refId" : "A"
          }
        ],
      },
      {
        "id" : 6,
        "title" : "Fetched Registrations Items per Hour",
        "type" : "stat"
        "pluginVersion" : "8.4.7",
        "datasource" : {
          "type" : "prometheus",
          "uid" : grafana_data_source.prometheus.uid
        },
        "description" : "The number of registrations retrieved from the database",
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "thresholds"
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "x" : 10, "y" : 8
          "h" : 16, "w" : 10,
        },
        "options" : {
          "colorMode" : "value",
          "graphMode" : "area",
          "justifyMode" : "auto",
          "orientation" : "auto",
          "reduceOptions" : {
            "calcs" : [
              "lastNotNull"
            ],
            "fields" : "",
            "values" : false
          },
          "text" : {},
          "textMode" : "auto"
        },
        "targets" : [
          {
            "datasource" : {
              "type" : "prometheus",
              "uid" : grafana_data_source.prometheus.uid
            },
            "exemplar" : true,
            "expr" : "sum(rate(fetched_registrations{}[1h]))",
            "interval" : "",
            "legendFormat" : "",
            "refId" : "A"
          }
        ],
      },
    ],
  })
}
