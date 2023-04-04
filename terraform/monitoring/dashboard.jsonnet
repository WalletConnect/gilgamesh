local layout = import 'layout.libsonnet';
local full_width = layout.full_width;
local half_width = layout.half_width;
local height    = 8;

local grafana         = import 'grafonnet/grafana.libsonnet';
local dashboard       = grafana.dashboard;
local annotation      = grafana.annotation;
local timeseriesPanel = grafana.timeseriesPanel;
local prometheus      = grafana.prometheus;

local ds_prometheus = {
  type: 'prometheus',
  uid: std.extVar('prometheus_uid'),
};

dashboard.new(
  title = "%s - %s" % [std.extVar('stage'), std.extVar('name')],
  uid = "%s_%s" % [std.extVar('stage'), std.extVar('name')],
  schemaVersion = 35,
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
    timeseriesPanel.new(
      title = 'Received Items per Hour',
      datasource = ds_prometheus,
    )
    .addTarget(prometheus.target(
      datasource = ds_prometheus,
      expr = 'sum(rate(received_items{}[10m]))',
      legendFormat = 'received items',
      exemplar = true,
    ))
    { gridPos: { w: half_width, h: height } },

    timeseriesPanel.new(
      title = 'Stored Items per Hour',
      datasource = ds_prometheus,
    )
    .addTarget(prometheus.target(
      datasource = ds_prometheus,
      expr = 'sum(rate(stored_items{}[10m]))',
      legendFormat = 'stored items',
      exemplar = true,
    ))
    { gridPos: { w: half_width, h: height } },

    timeseriesPanel.new(
      title = 'Get Queries per Hour',
      datasource = ds_prometheus,
    )
    .addTarget(prometheus.target(
      datasource = ds_prometheus,
      expr = 'sum(rate(get_queries{}[10m]))',
      legendFormat = 'get queries',
      exemplar = true,
    ))
    { gridPos: { w: half_width, h: height } },

    timeseriesPanel.new(
      title = 'Served Items per Hour',
      datasource = ds_prometheus,
    )
    .addTarget(prometheus.target(
      datasource = ds_prometheus,
      expr = 'sum(rate(served_items{}[10m]))',
      legendFormat = 'served items',
      exemplar = true,
    ))
    { gridPos: { w: half_width, h: height } },

    timeseriesPanel.new(
      title = 'Registrations per Hour',
      datasource = ds_prometheus,
    )
    .addTarget(prometheus.target(
      datasource = ds_prometheus,
      expr = 'sum(rate(cached_registrations{}[10m]))',
      legendFormat = 'cached registrations',
      exemplar = true,
    ))
    .addTarget(prometheus.target(
      datasource = ds_prometheus,
      expr = 'sum(rate(fetched_registrations{}[10m]))',
      legendFormat = 'fetched registrations',
      exemplar = true,
    ))
    { gridPos: { w: full_width, h: height } },
  ])
)
