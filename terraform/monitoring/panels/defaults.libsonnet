local grafana         = import '../grafonnet-lib/grafana.libsonnet';

{
  configuration:: {
    timeseries::
      grafana.panels.timeseries().createConfiguration(
        scaleDistribution = {
          type : 'linear'
        },
        stacking = {
          group:  'A',
          mode:   'none'
        },
        legend  = grafana.common.legend(),
        tooltip = grafana.common.tooltip(),
      ),
  },
}
