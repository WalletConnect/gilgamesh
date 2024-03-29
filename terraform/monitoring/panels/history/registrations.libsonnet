local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels    = grafana.panels;
local targets   = grafana.targets;

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Registrations per Hour',
      datasource  = ds.prometheus,
    )
    .configure(defaults.configuration.timeseries)
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      expr          = 'sum(rate(cached_registrations{}[10m]))',
      legendFormat  = 'cached registrations',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      expr          = 'sum(rate(fetched_registrations{}[10m]))',
      legendFormat  = 'fetched registrations',
      exemplar      = true,
    ))
}
