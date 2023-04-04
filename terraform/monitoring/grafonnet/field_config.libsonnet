local ThresholdMode = {
  Absolute: 'absolute',
  Percentage: 'percentage',
};
local FieldColorModeId = {
  ContinuousGrYlRd: 'continuous-GrYlRd',
  Fixed: 'fixed',
  PaletteClassic: 'palette-classic',
  PaletteSaturated: 'palette-saturated',
  Thresholds: 'thresholds',
};
local FieldColorSeriesByMode = {
  Min: 'min',
  Max: 'max',
  Last: 'last',
};

{
  /**
   * Creates a default field config.
   *
   * @name field_config.default
   *
   * @param color (optional) Map values to a display color.
   * @param custom (optional) ????.
   * @param decimals (optional) Significant digits (for display).
   * @param description (optional) Human readable field metadata.
   * @param displayName (optional) The display value for this field. This supports template variables blank is auto.
   * @param displayNameFromDS (optional) This can be used by data sources that return and explicit naming structure for values and labels.
   * @param filterable (optional) True if data source field supports ad-hoc filters.
   * @param min (optional)
   * @param max (optional)
   * @param noValue (optional) Alternative to empty string.
   * @param path (optional) An explicit path to the field in the datasource.
   * @param unit (optional) Numeric Options.
   * @param writeable (optional) True if data source can write a value to the path.
   *
   * @method addThreshold(step) Adds a [threshold](https://grafana.com/docs/grafana/latest/panels/thresholds/) step. Argument format: `{ color: 'green', value: 0 }`.
   * @method addThresholds(steps) Adds an array of threshold steps.
   * @method addMapping(mapping) Adds a value mapping.
   * @method addMappings(mappings) Adds an array of value mappings.
   */
  default(
    color=null,
    custom={},
    decimals=null,
    description=null,
    displayName=null,
    displayNameFromDS=null,
    filterable=null,
    min=null,
    max=null,
    noValue=null,
    path=null,
    unit=null,
    thresholds=null,
    writeable=null,
  ):: {
    color: if color != null then color else {
      mode: FieldColorModeId.PaletteClassic
    },
    custom: custom,
    [if decimals != null then 'decimals']: decimals,
    [if description != null then 'description']: description,
    [if displayName != null then 'displayName']: displayName,
    [if displayNameFromDS != null then 'displayNameFromDS']: displayNameFromDS,
    [if filterable != null then 'filterable']: filterable,
    [if min != null then 'min']: min,
    [if max != null then 'max']: max,
    [if noValue != null then 'noValue']: noValue,
    [if path != null then 'path']: path,
    [if unit != null then 'unit']: unit,
    [if writeable != null then 'writeable']: writeable,

    thresholds: if thresholds != null then thresholds else {
      mode: ThresholdMode.Absolute,
      steps: [
        {
          color: "green",
          value: null
        }
      ]
    },
  } +

  {
    // Thresholds
    addThreshold(step):: self {
      thresholds+: {
        steps+: [step],
      }
    },
    addThresholds(steps):: std.foldl(function(p, s) p.addThreshold(s), steps, self),
  } +

  {
    // Mappings
    mappings: [],
    addMapping(mapping):: self {
      mappings+: [mapping],
    },
    addMappings(mappings):: std.foldl(function(p, m) p.addMapping(m), mappings, self),
  },
}
