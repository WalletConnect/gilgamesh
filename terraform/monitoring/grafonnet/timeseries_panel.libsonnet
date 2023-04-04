local panel = import 'panel.libsonnet';
local fieldConfig = import 'field_config.libsonnet';

  /**
   * Timeseries panel panel.
   *
   * Options:
   * @name timeseries_panel.new
   * @name axisGridShow (optional) True to always show the axis grid.
   * @name axisLabel (optional) Set a Y-axis text label. If you have more than one Y-axis, then you can assign different labels using an override.
   * @name axisPlacement (default `auto`) Select the placement of the Y-axis. 'auto', 'left', 'right', 'hidden'.
   * @name axisSoftMax (optional)
   * @name axisSoftMin (optional)
   * @name axisWidth (optional) Set a fixed width of the axis. By default, Grafana dynamically calculates the width of an axis.
   * @name barAlignment (default 0) Set the position of the bar relative to a data point
   * @name drawStyle (default `'line'`)
   * @name fillOpacity (default 0)
   * @name gradientMode (default `'none'`) 'none' | 'opacity' | 'hue'
   * @name hideFrom (optional)
   * @name lineInterpolation (default `'linear'`) 'linear' | 'smooth'
   * @name lineStyle (optional)
   * @name lineWidth (default 1)
   * @name pointSize (default 5)
   * @name scaleDistribution (optional)
   * @name showPoints (default `'auto'`) 'auto' | 'always' | 'never'
   * @name spanNulls (default `false`)
   * @name stacking (optional)
   * @name thresholdsStyle (optional)
   *
   * @param title The title of the singlestat panel.


   * @method addTarget(target) Adds a target object.
   * @method addTargets(targets) Adds an array of targets.
   * @method addLink(link) Adds a [panel link](https://grafana.com/docs/grafana/latest/linking/panel-links/).
   * @method addLinks(links) Adds an array of links.
   */
{
  type: 'timeseries',
  fieldConfig: fieldConfig.default() + {
    defaults: {
      custom: {
        axisPlacement: 'auto',
        barAlignment: 0,
        drawStyle: 'line',
        fillOpacity: 0,
        gradientMode: 'none',
        lineInterpolation: 'linear',
        lineWidth: 1,
        pointSize: 5,
        showPoints: 'auto',
        spanNulls: false,
        thresholdsStyle: {
          mode: 'off'
        }
      }
    }
  },

  options(
    axisGridShow=null,
    axisLabel=null,
    axisPlacement=null,
    axisSoftMax=null,
    axisSoftMin=null,
    axisWidth=null,
    barAlignment=null,
    drawStyle=null,
    fillOpacity=null,
    gradientMode=null,
    hideFrom=null,
    lineInterpolation=null,
    lineStyle=null,
    lineWidth=null,
    pointSize=null,
    scaleDistribution=null,
    showPoints=null,
    spanNulls=null,
    stacking=null,
    thresholdsStyle=null,
  ):: self {
    custom+: {
      [if axisGridShow != null      then 'axisGridShow']: axisGridShow,
      [if axisLabel != null         then 'axisLabel']: axisLabel,
      [if axisPlacement != null     then 'axisPlacement']: axisPlacement,
      [if axisSoftMax != null       then 'axisSoftMax']: axisSoftMax,
      [if axisSoftMin != null       then 'axisSoftMin']: axisSoftMin,
      [if axisWidth != null         then 'axisWidth']: axisWidth,
      [if barAlignment != null      then 'barAlignment']: barAlignment,
      [if drawStyle != null         then 'drawStyle']: drawStyle,
      [if fillOpacity != null        then 'fillOpacity']: fillOpacity,
      [if gradientMode != null      then 'gradientMode']: gradientMode,
      [if hideFrom != null          then 'hideFrom']: hideFrom,
      [if hideFrom != null          then 'hideFrom']: hideFrom,
      [if lineStyle != null         then 'lineStyle']: lineStyle,
      [if lineWidth != null         then 'lineWidth']: lineWidth,
      [if pointSize != null         then 'pointSize']: pointSize,
      [if scaleDistribution != null then 'scaleDistribution']: scaleDistribution,
      [if showPoints != null        then 'showPoints']: showPoints,
      [if spanNulls != null         then 'spanNulls']: spanNulls,
      [if stacking != null          then 'stacking']: stacking,
      [if thresholdsStyle != null   then 'thresholdsStyle']: thresholdsStyle,
    },
  },
} + panel
