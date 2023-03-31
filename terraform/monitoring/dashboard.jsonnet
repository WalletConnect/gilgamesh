local layout = import 'layout.libsonnet';
local full_width = layout.full_width;
local half_width = layout.half_width;
local height    = 8;

local grafana = import 'grafonnet/grafana.libsonnet';
local dashboard = grafana.dashboard;
local annotation = grafana.annotation;
local statPanel = grafana.statPanel;
local prometheus = grafana.prometheus;

local ds_prometheus = {
  type: 'prometheus',
  uid: std.extVar('prometheus_uid'),
};

dashboard.new(
  title = "%s - %s" % [std.extVar('stage'), std.extVar('name')],
  uid = "%s_%s" % [std.extVar('stage'), std.extVar('name')],
  schemaVersion = 26,
  editable = true,
  graphTooltip = 'shared_crosshair',
)
.addAnnotation(
  annotation.default {
    iconColor: 'rgba(0, 211, 255, 1)',
  }
)
.addPanels(
  layout.generate_grid([
    statPanel.new(
      title = 'Received Items per Hour',
      description = 'The number of items received from relay',
      datasource = ds_prometheus,
      reducerFunction = 'lastNotNull',
    )
    .addTarget(prometheus.target(
      expr = 'sum(rate(received_items{}[1h]))',
      legendFormat = 'Received Items',
      datasource = ds_prometheus,
    ))
    { gridPos: { w: half_width, h: height } },

    statPanel.new(
      title = 'Stored Items per Hour',
      description = 'The number of items actually stored in the database',
      datasource = ds_prometheus,
      reducerFunction = 'lastNotNull',
    )
    .addTarget(prometheus.target(
      expr = 'sum(rate(stored_items{}[1h]))',
      legendFormat = 'Stored Items',
      datasource = ds_prometheus,
    ))
    { gridPos: { w: half_width, h: height } },

    statPanel.new(
      title = 'Get Queries per Hour',
      description = 'The number of items retrieval queries',
      datasource = ds_prometheus,
      reducerFunction = 'lastNotNull',
    )
    .addTarget(prometheus.target(
      expr = 'sum(rate(get_queries{}[1h]))',
      legendFormat = '"Get" Queries',
      datasource = ds_prometheus,
    ))
    { gridPos: { w: half_width, h: height } },

    statPanel.new(
      title = 'Served Items per Hour',
      description = 'The number of messages served to clients',
      datasource = ds_prometheus,
      reducerFunction = 'lastNotNull',
    )
    .addTarget(prometheus.target(
      expr = 'sum(rate(served_items{}[1h]))',
      legendFormat = '"Get" Queries',
      datasource = ds_prometheus,
    ))
    { gridPos: { w: half_width, h: height } },

    statPanel.new(
      title = 'Registrations per Hour',
      description = 'The number of registrations retrievals',
      datasource = ds_prometheus,
      reducerFunction = 'lastNotNull',
    )
    .addTarget(prometheus.target(
      expr = 'sum(rate(cached_registrations{}[1h]))',
      legendFormat = 'Cache hits',
      datasource = ds_prometheus,
    ))
    .addTarget(prometheus.target(
      expr = 'sum(rate(fetched_registrations{}[1h]))',
      legendFormat = 'Cache misses',
      datasource = ds_prometheus,
    ))
    { gridPos: { w: full_width, h: height } },
  ])
)
