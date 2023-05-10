local grafana     = import 'grafonnet-lib/grafana.libsonnet';
local panels      = import 'panels/panels.libsonnet';

local dashboard   = grafana.dashboard;

local ds    = {
  prometheus: {
    type: 'prometheus',
    uid:  std.extVar('prometheus_uid'),
  },
};
local vars  = {
};

////////////////////////////////////////////////////////////////////////////////

local height  = 8;
local pos     = grafana.layout.pos(height);

////////////////////////////////////////////////////////////////////////////////

dashboard.new(
  title         = std.extVar('dashboard_title'),
  uid           = std.extVar('dashboard_uid'),
  editable      = true,
  graphTooltip  = dashboard.graphTooltips.sharedCrosshair,
)
.addAnnotation(
  grafana.annotation.new(
    target = {
      limit:    100,
      matchAny: false,
      tags:     [],
      type:     'dashboard',
    },
  )
)
.addPanels(
  grafana.layout.generate_grid([
    panels.received_items(ds, vars) { gridPos: pos._2 },
    panels.stored_items(ds, vars)   { gridPos: pos._2 },

    panels.get_queries(ds, vars)    { gridPos: pos._2 },
    panels.served_items(ds, vars)   { gridPos: pos._2 },

    panels.registrations(ds, vars)  { gridPos: pos._1 },
  ])
)
