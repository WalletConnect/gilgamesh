local grafana   = import '../grafonnet-lib/grafana.libsonnet';
local panels    = grafana.panels;
local targets   = grafana.targets;

local defaults  = import 'defaults.libsonnet';

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Stored Items per Hour',
      datasource  = ds.prometheus,
    )
    .configure(defaults.configuration.timeseries)
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      expr          = 'sum(rate(stored_items{}[10m]))',
      legendFormat  = 'stored items',
      exemplar      = true,
    ))
}
