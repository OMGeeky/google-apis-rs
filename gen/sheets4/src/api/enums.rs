use super::*;



// region AppendDimensionRequestDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether rows or columns should be appended.
pub enum AppendDimensionRequestDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for AppendDimensionRequestDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppendDimensionRequestDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            AppendDimensionRequestDimensionEnum::ROWS => "ROWS",
            AppendDimensionRequestDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for AppendDimensionRequestDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(AppendDimensionRequestDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(AppendDimensionRequestDimensionEnum::ROWS),
           "COLUMNS" => Ok(AppendDimensionRequestDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppendDimensionRequestDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BaselineValueFormatComparisonTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The comparison type of key value with baseline value.
pub enum BaselineValueFormatComparisonTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "COMPARISON_TYPE_UNDEFINED"
    #[serde(rename="COMPARISON_TYPE_UNDEFINED")]
    COMPARISONTYPEUNDEFINED,
    

    /// Use absolute difference between key and baseline value.
    ///
    /// "ABSOLUTE_DIFFERENCE"
    #[serde(rename="ABSOLUTE_DIFFERENCE")]
    ABSOLUTEDIFFERENCE,
    

    /// Use percentage difference between key and baseline value.
    ///
    /// "PERCENTAGE_DIFFERENCE"
    #[serde(rename="PERCENTAGE_DIFFERENCE")]
    PERCENTAGEDIFFERENCE,
}

impl AsRef<str> for BaselineValueFormatComparisonTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BaselineValueFormatComparisonTypeEnum::COMPARISONTYPEUNDEFINED => "COMPARISON_TYPE_UNDEFINED",
            BaselineValueFormatComparisonTypeEnum::ABSOLUTEDIFFERENCE => "ABSOLUTE_DIFFERENCE",
            BaselineValueFormatComparisonTypeEnum::PERCENTAGEDIFFERENCE => "PERCENTAGE_DIFFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for BaselineValueFormatComparisonTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPARISON_TYPE_UNDEFINED" => Ok(BaselineValueFormatComparisonTypeEnum::COMPARISONTYPEUNDEFINED),
           "ABSOLUTE_DIFFERENCE" => Ok(BaselineValueFormatComparisonTypeEnum::ABSOLUTEDIFFERENCE),
           "PERCENTAGE_DIFFERENCE" => Ok(BaselineValueFormatComparisonTypeEnum::PERCENTAGEDIFFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BaselineValueFormatComparisonTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartAxiPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The position of this axis.
pub enum BasicChartAxiPositionEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_AXIS_POSITION_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_AXIS_POSITION_UNSPECIFIED")]
    BASICCHARTAXISPOSITIONUNSPECIFIED,
    

    /// The axis rendered at the bottom of a chart. For most charts, this is the standard major axis. For bar charts, this is a minor axis.
    ///
    /// "BOTTOM_AXIS"
    #[serde(rename="BOTTOM_AXIS")]
    BOTTOMAXIS,
    

    /// The axis rendered at the left of a chart. For most charts, this is a minor axis. For bar charts, this is the standard major axis.
    ///
    /// "LEFT_AXIS"
    #[serde(rename="LEFT_AXIS")]
    LEFTAXIS,
    

    /// The axis rendered at the right of a chart. For most charts, this is a minor axis. For bar charts, this is an unusual major axis.
    ///
    /// "RIGHT_AXIS"
    #[serde(rename="RIGHT_AXIS")]
    RIGHTAXIS,
}

impl AsRef<str> for BasicChartAxiPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartAxiPositionEnum::BASICCHARTAXISPOSITIONUNSPECIFIED => "BASIC_CHART_AXIS_POSITION_UNSPECIFIED",
            BasicChartAxiPositionEnum::BOTTOMAXIS => "BOTTOM_AXIS",
            BasicChartAxiPositionEnum::LEFTAXIS => "LEFT_AXIS",
            BasicChartAxiPositionEnum::RIGHTAXIS => "RIGHT_AXIS",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartAxiPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_AXIS_POSITION_UNSPECIFIED" => Ok(BasicChartAxiPositionEnum::BASICCHARTAXISPOSITIONUNSPECIFIED),
           "BOTTOM_AXIS" => Ok(BasicChartAxiPositionEnum::BOTTOMAXIS),
           "LEFT_AXIS" => Ok(BasicChartAxiPositionEnum::LEFTAXIS),
           "RIGHT_AXIS" => Ok(BasicChartAxiPositionEnum::RIGHTAXIS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartAxiPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartSeryTargetAxisEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minor axis that will specify the range of values for this series. For example, if charting stocks over time, the "Volume" series may want to be pinned to the right with the prices pinned to the left, because the scale of trading volume is different than the scale of prices. It is an error to specify an axis that isn't a valid minor axis for the chart's type.
pub enum BasicChartSeryTargetAxisEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_AXIS_POSITION_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_AXIS_POSITION_UNSPECIFIED")]
    BASICCHARTAXISPOSITIONUNSPECIFIED,
    

    /// The axis rendered at the bottom of a chart. For most charts, this is the standard major axis. For bar charts, this is a minor axis.
    ///
    /// "BOTTOM_AXIS"
    #[serde(rename="BOTTOM_AXIS")]
    BOTTOMAXIS,
    

    /// The axis rendered at the left of a chart. For most charts, this is a minor axis. For bar charts, this is the standard major axis.
    ///
    /// "LEFT_AXIS"
    #[serde(rename="LEFT_AXIS")]
    LEFTAXIS,
    

    /// The axis rendered at the right of a chart. For most charts, this is a minor axis. For bar charts, this is an unusual major axis.
    ///
    /// "RIGHT_AXIS"
    #[serde(rename="RIGHT_AXIS")]
    RIGHTAXIS,
}

impl AsRef<str> for BasicChartSeryTargetAxisEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartSeryTargetAxisEnum::BASICCHARTAXISPOSITIONUNSPECIFIED => "BASIC_CHART_AXIS_POSITION_UNSPECIFIED",
            BasicChartSeryTargetAxisEnum::BOTTOMAXIS => "BOTTOM_AXIS",
            BasicChartSeryTargetAxisEnum::LEFTAXIS => "LEFT_AXIS",
            BasicChartSeryTargetAxisEnum::RIGHTAXIS => "RIGHT_AXIS",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartSeryTargetAxisEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_AXIS_POSITION_UNSPECIFIED" => Ok(BasicChartSeryTargetAxisEnum::BASICCHARTAXISPOSITIONUNSPECIFIED),
           "BOTTOM_AXIS" => Ok(BasicChartSeryTargetAxisEnum::BOTTOMAXIS),
           "LEFT_AXIS" => Ok(BasicChartSeryTargetAxisEnum::LEFTAXIS),
           "RIGHT_AXIS" => Ok(BasicChartSeryTargetAxisEnum::RIGHTAXIS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartSeryTargetAxisEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartSeryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this series. Valid only if the chartType is COMBO. Different types will change the way the series is visualized. Only LINE, AREA, and COLUMN are supported.
pub enum BasicChartSeryTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_TYPE_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_TYPE_UNSPECIFIED")]
    BASICCHARTTYPEUNSPECIFIED,
    

    /// A bar chart.
    ///
    /// "BAR"
    #[serde(rename="BAR")]
    BAR,
    

    /// A line chart.
    ///
    /// "LINE"
    #[serde(rename="LINE")]
    LINE,
    

    /// An area chart.
    ///
    /// "AREA"
    #[serde(rename="AREA")]
    AREA,
    

    /// A column chart.
    ///
    /// "COLUMN"
    #[serde(rename="COLUMN")]
    COLUMN,
    

    /// A scatter chart.
    ///
    /// "SCATTER"
    #[serde(rename="SCATTER")]
    SCATTER,
    

    /// A combo chart.
    ///
    /// "COMBO"
    #[serde(rename="COMBO")]
    COMBO,
    

    /// A stepped area chart.
    ///
    /// "STEPPED_AREA"
    #[serde(rename="STEPPED_AREA")]
    STEPPEDAREA,
}

impl AsRef<str> for BasicChartSeryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartSeryTypeEnum::BASICCHARTTYPEUNSPECIFIED => "BASIC_CHART_TYPE_UNSPECIFIED",
            BasicChartSeryTypeEnum::BAR => "BAR",
            BasicChartSeryTypeEnum::LINE => "LINE",
            BasicChartSeryTypeEnum::AREA => "AREA",
            BasicChartSeryTypeEnum::COLUMN => "COLUMN",
            BasicChartSeryTypeEnum::SCATTER => "SCATTER",
            BasicChartSeryTypeEnum::COMBO => "COMBO",
            BasicChartSeryTypeEnum::STEPPEDAREA => "STEPPED_AREA",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartSeryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_TYPE_UNSPECIFIED" => Ok(BasicChartSeryTypeEnum::BASICCHARTTYPEUNSPECIFIED),
           "BAR" => Ok(BasicChartSeryTypeEnum::BAR),
           "LINE" => Ok(BasicChartSeryTypeEnum::LINE),
           "AREA" => Ok(BasicChartSeryTypeEnum::AREA),
           "COLUMN" => Ok(BasicChartSeryTypeEnum::COLUMN),
           "SCATTER" => Ok(BasicChartSeryTypeEnum::SCATTER),
           "COMBO" => Ok(BasicChartSeryTypeEnum::COMBO),
           "STEPPED_AREA" => Ok(BasicChartSeryTypeEnum::STEPPEDAREA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartSeryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartSpecChartTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the chart.
pub enum BasicChartSpecChartTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_TYPE_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_TYPE_UNSPECIFIED")]
    BASICCHARTTYPEUNSPECIFIED,
    

    /// A bar chart.
    ///
    /// "BAR"
    #[serde(rename="BAR")]
    BAR,
    

    /// A line chart.
    ///
    /// "LINE"
    #[serde(rename="LINE")]
    LINE,
    

    /// An area chart.
    ///
    /// "AREA"
    #[serde(rename="AREA")]
    AREA,
    

    /// A column chart.
    ///
    /// "COLUMN"
    #[serde(rename="COLUMN")]
    COLUMN,
    

    /// A scatter chart.
    ///
    /// "SCATTER"
    #[serde(rename="SCATTER")]
    SCATTER,
    

    /// A combo chart.
    ///
    /// "COMBO"
    #[serde(rename="COMBO")]
    COMBO,
    

    /// A stepped area chart.
    ///
    /// "STEPPED_AREA"
    #[serde(rename="STEPPED_AREA")]
    STEPPEDAREA,
}

impl AsRef<str> for BasicChartSpecChartTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartSpecChartTypeEnum::BASICCHARTTYPEUNSPECIFIED => "BASIC_CHART_TYPE_UNSPECIFIED",
            BasicChartSpecChartTypeEnum::BAR => "BAR",
            BasicChartSpecChartTypeEnum::LINE => "LINE",
            BasicChartSpecChartTypeEnum::AREA => "AREA",
            BasicChartSpecChartTypeEnum::COLUMN => "COLUMN",
            BasicChartSpecChartTypeEnum::SCATTER => "SCATTER",
            BasicChartSpecChartTypeEnum::COMBO => "COMBO",
            BasicChartSpecChartTypeEnum::STEPPEDAREA => "STEPPED_AREA",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartSpecChartTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_TYPE_UNSPECIFIED" => Ok(BasicChartSpecChartTypeEnum::BASICCHARTTYPEUNSPECIFIED),
           "BAR" => Ok(BasicChartSpecChartTypeEnum::BAR),
           "LINE" => Ok(BasicChartSpecChartTypeEnum::LINE),
           "AREA" => Ok(BasicChartSpecChartTypeEnum::AREA),
           "COLUMN" => Ok(BasicChartSpecChartTypeEnum::COLUMN),
           "SCATTER" => Ok(BasicChartSpecChartTypeEnum::SCATTER),
           "COMBO" => Ok(BasicChartSpecChartTypeEnum::COMBO),
           "STEPPED_AREA" => Ok(BasicChartSpecChartTypeEnum::STEPPEDAREA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartSpecChartTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartSpecCompareModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The behavior of tooltips and data highlighting when hovering on data and chart area.
pub enum BasicChartSpecCompareModeEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_COMPARE_MODE_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_COMPARE_MODE_UNSPECIFIED")]
    BASICCHARTCOMPAREMODEUNSPECIFIED,
    

    /// Only the focused data element is highlighted and shown in the tooltip.
    ///
    /// "DATUM"
    #[serde(rename="DATUM")]
    DATUM,
    

    /// All data elements with the same category (e.g., domain value) are highlighted and shown in the tooltip.
    ///
    /// "CATEGORY"
    #[serde(rename="CATEGORY")]
    CATEGORY,
}

impl AsRef<str> for BasicChartSpecCompareModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartSpecCompareModeEnum::BASICCHARTCOMPAREMODEUNSPECIFIED => "BASIC_CHART_COMPARE_MODE_UNSPECIFIED",
            BasicChartSpecCompareModeEnum::DATUM => "DATUM",
            BasicChartSpecCompareModeEnum::CATEGORY => "CATEGORY",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartSpecCompareModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_COMPARE_MODE_UNSPECIFIED" => Ok(BasicChartSpecCompareModeEnum::BASICCHARTCOMPAREMODEUNSPECIFIED),
           "DATUM" => Ok(BasicChartSpecCompareModeEnum::DATUM),
           "CATEGORY" => Ok(BasicChartSpecCompareModeEnum::CATEGORY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartSpecCompareModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartSpecLegendPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The position of the chart legend.
pub enum BasicChartSpecLegendPositionEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_LEGEND_POSITION_UNSPECIFIED")]
    BASICCHARTLEGENDPOSITIONUNSPECIFIED,
    

    /// The legend is rendered on the bottom of the chart.
    ///
    /// "BOTTOM_LEGEND"
    #[serde(rename="BOTTOM_LEGEND")]
    BOTTOMLEGEND,
    

    /// The legend is rendered on the left of the chart.
    ///
    /// "LEFT_LEGEND"
    #[serde(rename="LEFT_LEGEND")]
    LEFTLEGEND,
    

    /// The legend is rendered on the right of the chart.
    ///
    /// "RIGHT_LEGEND"
    #[serde(rename="RIGHT_LEGEND")]
    RIGHTLEGEND,
    

    /// The legend is rendered on the top of the chart.
    ///
    /// "TOP_LEGEND"
    #[serde(rename="TOP_LEGEND")]
    TOPLEGEND,
    

    /// No legend is rendered.
    ///
    /// "NO_LEGEND"
    #[serde(rename="NO_LEGEND")]
    NOLEGEND,
}

impl AsRef<str> for BasicChartSpecLegendPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartSpecLegendPositionEnum::BASICCHARTLEGENDPOSITIONUNSPECIFIED => "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED",
            BasicChartSpecLegendPositionEnum::BOTTOMLEGEND => "BOTTOM_LEGEND",
            BasicChartSpecLegendPositionEnum::LEFTLEGEND => "LEFT_LEGEND",
            BasicChartSpecLegendPositionEnum::RIGHTLEGEND => "RIGHT_LEGEND",
            BasicChartSpecLegendPositionEnum::TOPLEGEND => "TOP_LEGEND",
            BasicChartSpecLegendPositionEnum::NOLEGEND => "NO_LEGEND",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartSpecLegendPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_LEGEND_POSITION_UNSPECIFIED" => Ok(BasicChartSpecLegendPositionEnum::BASICCHARTLEGENDPOSITIONUNSPECIFIED),
           "BOTTOM_LEGEND" => Ok(BasicChartSpecLegendPositionEnum::BOTTOMLEGEND),
           "LEFT_LEGEND" => Ok(BasicChartSpecLegendPositionEnum::LEFTLEGEND),
           "RIGHT_LEGEND" => Ok(BasicChartSpecLegendPositionEnum::RIGHTLEGEND),
           "TOP_LEGEND" => Ok(BasicChartSpecLegendPositionEnum::TOPLEGEND),
           "NO_LEGEND" => Ok(BasicChartSpecLegendPositionEnum::NOLEGEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartSpecLegendPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BasicChartSpecStackedTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The stacked type for charts that support vertical stacking. Applies to Area, Bar, Column, Combo, and Stepped Area charts.
pub enum BasicChartSpecStackedTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "BASIC_CHART_STACKED_TYPE_UNSPECIFIED"
    #[serde(rename="BASIC_CHART_STACKED_TYPE_UNSPECIFIED")]
    BASICCHARTSTACKEDTYPEUNSPECIFIED,
    

    /// Series are not stacked.
    ///
    /// "NOT_STACKED"
    #[serde(rename="NOT_STACKED")]
    NOTSTACKED,
    

    /// Series values are stacked, each value is rendered vertically beginning from the top of the value below it.
    ///
    /// "STACKED"
    #[serde(rename="STACKED")]
    STACKED,
    

    /// Vertical stacks are stretched to reach the top of the chart, with values laid out as percentages of each other.
    ///
    /// "PERCENT_STACKED"
    #[serde(rename="PERCENT_STACKED")]
    PERCENTSTACKED,
}

impl AsRef<str> for BasicChartSpecStackedTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicChartSpecStackedTypeEnum::BASICCHARTSTACKEDTYPEUNSPECIFIED => "BASIC_CHART_STACKED_TYPE_UNSPECIFIED",
            BasicChartSpecStackedTypeEnum::NOTSTACKED => "NOT_STACKED",
            BasicChartSpecStackedTypeEnum::STACKED => "STACKED",
            BasicChartSpecStackedTypeEnum::PERCENTSTACKED => "PERCENT_STACKED",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicChartSpecStackedTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_CHART_STACKED_TYPE_UNSPECIFIED" => Ok(BasicChartSpecStackedTypeEnum::BASICCHARTSTACKEDTYPEUNSPECIFIED),
           "NOT_STACKED" => Ok(BasicChartSpecStackedTypeEnum::NOTSTACKED),
           "STACKED" => Ok(BasicChartSpecStackedTypeEnum::STACKED),
           "PERCENT_STACKED" => Ok(BasicChartSpecStackedTypeEnum::PERCENTSTACKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicChartSpecStackedTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
pub enum BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum {
    

    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    ///
    /// "SERIAL_NUMBER"
    #[serde(rename="SERIAL_NUMBER")]
    SERIALNUMBER,
    

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    ///
    /// "FORMATTED_STRING"
    #[serde(rename="FORMATTED_STRING")]
    FORMATTEDSTRING,
}

impl AsRef<str> for BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum::SERIALNUMBER => "SERIAL_NUMBER",
            BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum::FORMATTEDSTRING => "FORMATTED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERIAL_NUMBER" => Ok(BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum::SERIALNUMBER),
           "FORMATTED_STRING" => Ok(BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum::FORMATTEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchGetValuesByDataFilterRequestDateTimeRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchGetValuesByDataFilterRequestMajorDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then a request that selects that range and sets `majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas a request that sets `majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
pub enum BatchGetValuesByDataFilterRequestMajorDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for BatchGetValuesByDataFilterRequestMajorDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchGetValuesByDataFilterRequestMajorDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            BatchGetValuesByDataFilterRequestMajorDimensionEnum::ROWS => "ROWS",
            BatchGetValuesByDataFilterRequestMajorDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchGetValuesByDataFilterRequestMajorDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(BatchGetValuesByDataFilterRequestMajorDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(BatchGetValuesByDataFilterRequestMajorDimensionEnum::ROWS),
           "COLUMNS" => Ok(BatchGetValuesByDataFilterRequestMajorDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchGetValuesByDataFilterRequestMajorDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchGetValuesByDataFilterRequestValueRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How values should be represented in the output. The default render option is FORMATTED_VALUE.
pub enum BatchGetValuesByDataFilterRequestValueRenderOptionEnum {
    

    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `"$1.23"`.
    ///
    /// "FORMATTED_VALUE"
    #[serde(rename="FORMATTED_VALUE")]
    FORMATTEDVALUE,
    

    /// Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`.
    ///
    /// "UNFORMATTED_VALUE"
    #[serde(rename="UNFORMATTED_VALUE")]
    UNFORMATTEDVALUE,
    

    /// Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `"=A1"`. Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see [About date & time values](https://developers.google.com/sheets/api/guides/formats#about_date_time_values).
    ///
    /// "FORMULA"
    #[serde(rename="FORMULA")]
    FORMULA,
}

impl AsRef<str> for BatchGetValuesByDataFilterRequestValueRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchGetValuesByDataFilterRequestValueRenderOptionEnum::FORMATTEDVALUE => "FORMATTED_VALUE",
            BatchGetValuesByDataFilterRequestValueRenderOptionEnum::UNFORMATTEDVALUE => "UNFORMATTED_VALUE",
            BatchGetValuesByDataFilterRequestValueRenderOptionEnum::FORMULA => "FORMULA",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchGetValuesByDataFilterRequestValueRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMATTED_VALUE" => Ok(BatchGetValuesByDataFilterRequestValueRenderOptionEnum::FORMATTEDVALUE),
           "UNFORMATTED_VALUE" => Ok(BatchGetValuesByDataFilterRequestValueRenderOptionEnum::UNFORMATTEDVALUE),
           "FORMULA" => Ok(BatchGetValuesByDataFilterRequestValueRenderOptionEnum::FORMULA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchGetValuesByDataFilterRequestValueRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
pub enum BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum {
    

    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    ///
    /// "SERIAL_NUMBER"
    #[serde(rename="SERIAL_NUMBER")]
    SERIALNUMBER,
    

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    ///
    /// "FORMATTED_STRING"
    #[serde(rename="FORMATTED_STRING")]
    FORMATTEDSTRING,
}

impl AsRef<str> for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum::SERIALNUMBER => "SERIAL_NUMBER",
            BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum::FORMATTEDSTRING => "FORMATTED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERIAL_NUMBER" => Ok(BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum::SERIALNUMBER),
           "FORMATTED_STRING" => Ok(BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum::FORMATTEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateValuesByDataFilterRequestResponseDateTimeRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
pub enum BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum {
    

    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `"$1.23"`.
    ///
    /// "FORMATTED_VALUE"
    #[serde(rename="FORMATTED_VALUE")]
    FORMATTEDVALUE,
    

    /// Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`.
    ///
    /// "UNFORMATTED_VALUE"
    #[serde(rename="UNFORMATTED_VALUE")]
    UNFORMATTEDVALUE,
    

    /// Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `"=A1"`. Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see [About date & time values](https://developers.google.com/sheets/api/guides/formats#about_date_time_values).
    ///
    /// "FORMULA"
    #[serde(rename="FORMULA")]
    FORMULA,
}

impl AsRef<str> for BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum::FORMATTEDVALUE => "FORMATTED_VALUE",
            BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum::UNFORMATTEDVALUE => "UNFORMATTED_VALUE",
            BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum::FORMULA => "FORMULA",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMATTED_VALUE" => Ok(BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum::FORMATTEDVALUE),
           "UNFORMATTED_VALUE" => Ok(BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum::UNFORMATTEDVALUE),
           "FORMULA" => Ok(BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum::FORMULA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateValuesByDataFilterRequestResponseValueRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateValuesByDataFilterRequestValueInputOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the input data should be interpreted.
pub enum BatchUpdateValuesByDataFilterRequestValueInputOptionEnum {
    

    /// Default input value. This value must not be used.
    ///
    /// "INPUT_VALUE_OPTION_UNSPECIFIED"
    #[serde(rename="INPUT_VALUE_OPTION_UNSPECIFIED")]
    INPUTVALUEOPTIONUNSPECIFIED,
    

    /// The values the user has entered will not be parsed and will be stored as-is.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
    

    /// The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    ///
    /// "USER_ENTERED"
    #[serde(rename="USER_ENTERED")]
    USERENTERED,
}

impl AsRef<str> for BatchUpdateValuesByDataFilterRequestValueInputOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateValuesByDataFilterRequestValueInputOptionEnum::INPUTVALUEOPTIONUNSPECIFIED => "INPUT_VALUE_OPTION_UNSPECIFIED",
            BatchUpdateValuesByDataFilterRequestValueInputOptionEnum::RAW => "RAW",
            BatchUpdateValuesByDataFilterRequestValueInputOptionEnum::USERENTERED => "USER_ENTERED",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateValuesByDataFilterRequestValueInputOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INPUT_VALUE_OPTION_UNSPECIFIED" => Ok(BatchUpdateValuesByDataFilterRequestValueInputOptionEnum::INPUTVALUEOPTIONUNSPECIFIED),
           "RAW" => Ok(BatchUpdateValuesByDataFilterRequestValueInputOptionEnum::RAW),
           "USER_ENTERED" => Ok(BatchUpdateValuesByDataFilterRequestValueInputOptionEnum::USERENTERED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateValuesByDataFilterRequestValueInputOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
pub enum BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum {
    

    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    ///
    /// "SERIAL_NUMBER"
    #[serde(rename="SERIAL_NUMBER")]
    SERIALNUMBER,
    

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    ///
    /// "FORMATTED_STRING"
    #[serde(rename="FORMATTED_STRING")]
    FORMATTEDSTRING,
}

impl AsRef<str> for BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum::SERIALNUMBER => "SERIAL_NUMBER",
            BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum::FORMATTEDSTRING => "FORMATTED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERIAL_NUMBER" => Ok(BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum::SERIALNUMBER),
           "FORMATTED_STRING" => Ok(BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum::FORMATTEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateValuesRequestResponseDateTimeRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateValuesRequestResponseValueRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
pub enum BatchUpdateValuesRequestResponseValueRenderOptionEnum {
    

    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `"$1.23"`.
    ///
    /// "FORMATTED_VALUE"
    #[serde(rename="FORMATTED_VALUE")]
    FORMATTEDVALUE,
    

    /// Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`.
    ///
    /// "UNFORMATTED_VALUE"
    #[serde(rename="UNFORMATTED_VALUE")]
    UNFORMATTEDVALUE,
    

    /// Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `"=A1"`. Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see [About date & time values](https://developers.google.com/sheets/api/guides/formats#about_date_time_values).
    ///
    /// "FORMULA"
    #[serde(rename="FORMULA")]
    FORMULA,
}

impl AsRef<str> for BatchUpdateValuesRequestResponseValueRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateValuesRequestResponseValueRenderOptionEnum::FORMATTEDVALUE => "FORMATTED_VALUE",
            BatchUpdateValuesRequestResponseValueRenderOptionEnum::UNFORMATTEDVALUE => "UNFORMATTED_VALUE",
            BatchUpdateValuesRequestResponseValueRenderOptionEnum::FORMULA => "FORMULA",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateValuesRequestResponseValueRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMATTED_VALUE" => Ok(BatchUpdateValuesRequestResponseValueRenderOptionEnum::FORMATTEDVALUE),
           "UNFORMATTED_VALUE" => Ok(BatchUpdateValuesRequestResponseValueRenderOptionEnum::UNFORMATTEDVALUE),
           "FORMULA" => Ok(BatchUpdateValuesRequestResponseValueRenderOptionEnum::FORMULA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateValuesRequestResponseValueRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BatchUpdateValuesRequestValueInputOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the input data should be interpreted.
pub enum BatchUpdateValuesRequestValueInputOptionEnum {
    

    /// Default input value. This value must not be used.
    ///
    /// "INPUT_VALUE_OPTION_UNSPECIFIED"
    #[serde(rename="INPUT_VALUE_OPTION_UNSPECIFIED")]
    INPUTVALUEOPTIONUNSPECIFIED,
    

    /// The values the user has entered will not be parsed and will be stored as-is.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
    

    /// The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    ///
    /// "USER_ENTERED"
    #[serde(rename="USER_ENTERED")]
    USERENTERED,
}

impl AsRef<str> for BatchUpdateValuesRequestValueInputOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchUpdateValuesRequestValueInputOptionEnum::INPUTVALUEOPTIONUNSPECIFIED => "INPUT_VALUE_OPTION_UNSPECIFIED",
            BatchUpdateValuesRequestValueInputOptionEnum::RAW => "RAW",
            BatchUpdateValuesRequestValueInputOptionEnum::USERENTERED => "USER_ENTERED",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchUpdateValuesRequestValueInputOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INPUT_VALUE_OPTION_UNSPECIFIED" => Ok(BatchUpdateValuesRequestValueInputOptionEnum::INPUTVALUEOPTIONUNSPECIFIED),
           "RAW" => Ok(BatchUpdateValuesRequestValueInputOptionEnum::RAW),
           "USER_ENTERED" => Ok(BatchUpdateValuesRequestValueInputOptionEnum::USERENTERED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchUpdateValuesRequestValueInputOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BooleanConditionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of condition.
pub enum BooleanConditionTypeEnum {
    

    /// The default value, do not use.
    ///
    /// "CONDITION_TYPE_UNSPECIFIED"
    #[serde(rename="CONDITION_TYPE_UNSPECIFIED")]
    CONDITIONTYPEUNSPECIFIED,
    

    /// The cell's value must be greater than the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "NUMBER_GREATER"
    #[serde(rename="NUMBER_GREATER")]
    NUMBERGREATER,
    

    /// The cell's value must be greater than or equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "NUMBER_GREATER_THAN_EQ"
    #[serde(rename="NUMBER_GREATER_THAN_EQ")]
    NUMBERGREATERTHANEQ,
    

    /// The cell's value must be less than the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "NUMBER_LESS"
    #[serde(rename="NUMBER_LESS")]
    NUMBERLESS,
    

    /// The cell's value must be less than or equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "NUMBER_LESS_THAN_EQ"
    #[serde(rename="NUMBER_LESS_THAN_EQ")]
    NUMBERLESSTHANEQ,
    

    /// The cell's value must be equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects.
    ///
    /// "NUMBER_EQ"
    #[serde(rename="NUMBER_EQ")]
    NUMBEREQ,
    

    /// The cell's value must be not equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects.
    ///
    /// "NUMBER_NOT_EQ"
    #[serde(rename="NUMBER_NOT_EQ")]
    NUMBERNOTEQ,
    

    /// The cell's value must be between the two condition values. Supported by data validation, conditional formatting and filters. Requires exactly two ConditionValues.
    ///
    /// "NUMBER_BETWEEN"
    #[serde(rename="NUMBER_BETWEEN")]
    NUMBERBETWEEN,
    

    /// The cell's value must not be between the two condition values. Supported by data validation, conditional formatting and filters. Requires exactly two ConditionValues.
    ///
    /// "NUMBER_NOT_BETWEEN"
    #[serde(rename="NUMBER_NOT_BETWEEN")]
    NUMBERNOTBETWEEN,
    

    /// The cell's value must contain the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "TEXT_CONTAINS"
    #[serde(rename="TEXT_CONTAINS")]
    TEXTCONTAINS,
    

    /// The cell's value must not contain the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "TEXT_NOT_CONTAINS"
    #[serde(rename="TEXT_NOT_CONTAINS")]
    TEXTNOTCONTAINS,
    

    /// The cell's value must start with the condition's value. Supported by conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "TEXT_STARTS_WITH"
    #[serde(rename="TEXT_STARTS_WITH")]
    TEXTSTARTSWITH,
    

    /// The cell's value must end with the condition's value. Supported by conditional formatting and filters. Requires a single ConditionValue.
    ///
    /// "TEXT_ENDS_WITH"
    #[serde(rename="TEXT_ENDS_WITH")]
    TEXTENDSWITH,
    

    /// The cell's value must be exactly the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects.
    ///
    /// "TEXT_EQ"
    #[serde(rename="TEXT_EQ")]
    TEXTEQ,
    

    /// The cell's value must be a valid email address. Supported by data validation. Requires no ConditionValues.
    ///
    /// "TEXT_IS_EMAIL"
    #[serde(rename="TEXT_IS_EMAIL")]
    TEXTISEMAIL,
    

    /// The cell's value must be a valid URL. Supported by data validation. Requires no ConditionValues.
    ///
    /// "TEXT_IS_URL"
    #[serde(rename="TEXT_IS_URL")]
    TEXTISURL,
    

    /// The cell's value must be the same date as the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects.
    ///
    /// "DATE_EQ"
    #[serde(rename="DATE_EQ")]
    DATEEQ,
    

    /// The cell's value must be before the date of the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue that may be a relative date.
    ///
    /// "DATE_BEFORE"
    #[serde(rename="DATE_BEFORE")]
    DATEBEFORE,
    

    /// The cell's value must be after the date of the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue that may be a relative date.
    ///
    /// "DATE_AFTER"
    #[serde(rename="DATE_AFTER")]
    DATEAFTER,
    

    /// The cell's value must be on or before the date of the condition's value. Supported by data validation. Requires a single ConditionValue that may be a relative date.
    ///
    /// "DATE_ON_OR_BEFORE"
    #[serde(rename="DATE_ON_OR_BEFORE")]
    DATEONORBEFORE,
    

    /// The cell's value must be on or after the date of the condition's value. Supported by data validation. Requires a single ConditionValue that may be a relative date.
    ///
    /// "DATE_ON_OR_AFTER"
    #[serde(rename="DATE_ON_OR_AFTER")]
    DATEONORAFTER,
    

    /// The cell's value must be between the dates of the two condition values. Supported by data validation. Requires exactly two ConditionValues.
    ///
    /// "DATE_BETWEEN"
    #[serde(rename="DATE_BETWEEN")]
    DATEBETWEEN,
    

    /// The cell's value must be outside the dates of the two condition values. Supported by data validation. Requires exactly two ConditionValues.
    ///
    /// "DATE_NOT_BETWEEN"
    #[serde(rename="DATE_NOT_BETWEEN")]
    DATENOTBETWEEN,
    

    /// The cell's value must be a date. Supported by data validation. Requires no ConditionValues.
    ///
    /// "DATE_IS_VALID"
    #[serde(rename="DATE_IS_VALID")]
    DATEISVALID,
    

    /// The cell's value must be listed in the grid in condition value's range. Supported by data validation. Requires a single ConditionValue, and the value must be a valid range in A1 notation.
    ///
    /// "ONE_OF_RANGE"
    #[serde(rename="ONE_OF_RANGE")]
    ONEOFRANGE,
    

    /// The cell's value must be in the list of condition values. Supported by data validation. Supports any number of condition values, one per item in the list. Formulas are not supported in the values.
    ///
    /// "ONE_OF_LIST"
    #[serde(rename="ONE_OF_LIST")]
    ONEOFLIST,
    

    /// The cell's value must be empty. Supported by conditional formatting and filters. Requires no ConditionValues.
    ///
    /// "BLANK"
    #[serde(rename="BLANK")]
    BLANK,
    

    /// The cell's value must not be empty. Supported by conditional formatting and filters. Requires no ConditionValues.
    ///
    /// "NOT_BLANK"
    #[serde(rename="NOT_BLANK")]
    NOTBLANK,
    

    /// The condition's formula must evaluate to true. Supported by data validation, conditional formatting and filters. Not supported by data source sheet filters. Requires a single ConditionValue.
    ///
    /// "CUSTOM_FORMULA"
    #[serde(rename="CUSTOM_FORMULA")]
    CUSTOMFORMULA,
    

    /// The cell's value must be TRUE/FALSE or in the list of condition values. Supported by data validation. Renders as a cell checkbox. Supports zero, one or two ConditionValues. No values indicates the cell must be TRUE or FALSE, where TRUE renders as checked and FALSE renders as unchecked. One value indicates the cell will render as checked when it contains that value and unchecked when it is blank. Two values indicate that the cell will render as checked when it contains the first value and unchecked when it contains the second value. For example, ["Yes","No"] indicates that the cell will render a checked box when it has the value "Yes" and an unchecked box when it has the value "No".
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// The cell's value must be exactly not the condition's value. Supported by filters on data source objects. Requires at least one ConditionValue.
    ///
    /// "TEXT_NOT_EQ"
    #[serde(rename="TEXT_NOT_EQ")]
    TEXTNOTEQ,
    

    /// The cell's value must be exactly not the condition's value. Supported by filters on data source objects. Requires at least one ConditionValue.
    ///
    /// "DATE_NOT_EQ"
    #[serde(rename="DATE_NOT_EQ")]
    DATENOTEQ,
    

    /// The cell's value must follow the pattern specified. Requires a single ConditionValue.
    ///
    /// "FILTER_EXPRESSION"
    #[serde(rename="FILTER_EXPRESSION")]
    FILTEREXPRESSION,
}

impl AsRef<str> for BooleanConditionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BooleanConditionTypeEnum::CONDITIONTYPEUNSPECIFIED => "CONDITION_TYPE_UNSPECIFIED",
            BooleanConditionTypeEnum::NUMBERGREATER => "NUMBER_GREATER",
            BooleanConditionTypeEnum::NUMBERGREATERTHANEQ => "NUMBER_GREATER_THAN_EQ",
            BooleanConditionTypeEnum::NUMBERLESS => "NUMBER_LESS",
            BooleanConditionTypeEnum::NUMBERLESSTHANEQ => "NUMBER_LESS_THAN_EQ",
            BooleanConditionTypeEnum::NUMBEREQ => "NUMBER_EQ",
            BooleanConditionTypeEnum::NUMBERNOTEQ => "NUMBER_NOT_EQ",
            BooleanConditionTypeEnum::NUMBERBETWEEN => "NUMBER_BETWEEN",
            BooleanConditionTypeEnum::NUMBERNOTBETWEEN => "NUMBER_NOT_BETWEEN",
            BooleanConditionTypeEnum::TEXTCONTAINS => "TEXT_CONTAINS",
            BooleanConditionTypeEnum::TEXTNOTCONTAINS => "TEXT_NOT_CONTAINS",
            BooleanConditionTypeEnum::TEXTSTARTSWITH => "TEXT_STARTS_WITH",
            BooleanConditionTypeEnum::TEXTENDSWITH => "TEXT_ENDS_WITH",
            BooleanConditionTypeEnum::TEXTEQ => "TEXT_EQ",
            BooleanConditionTypeEnum::TEXTISEMAIL => "TEXT_IS_EMAIL",
            BooleanConditionTypeEnum::TEXTISURL => "TEXT_IS_URL",
            BooleanConditionTypeEnum::DATEEQ => "DATE_EQ",
            BooleanConditionTypeEnum::DATEBEFORE => "DATE_BEFORE",
            BooleanConditionTypeEnum::DATEAFTER => "DATE_AFTER",
            BooleanConditionTypeEnum::DATEONORBEFORE => "DATE_ON_OR_BEFORE",
            BooleanConditionTypeEnum::DATEONORAFTER => "DATE_ON_OR_AFTER",
            BooleanConditionTypeEnum::DATEBETWEEN => "DATE_BETWEEN",
            BooleanConditionTypeEnum::DATENOTBETWEEN => "DATE_NOT_BETWEEN",
            BooleanConditionTypeEnum::DATEISVALID => "DATE_IS_VALID",
            BooleanConditionTypeEnum::ONEOFRANGE => "ONE_OF_RANGE",
            BooleanConditionTypeEnum::ONEOFLIST => "ONE_OF_LIST",
            BooleanConditionTypeEnum::BLANK => "BLANK",
            BooleanConditionTypeEnum::NOTBLANK => "NOT_BLANK",
            BooleanConditionTypeEnum::CUSTOMFORMULA => "CUSTOM_FORMULA",
            BooleanConditionTypeEnum::BOOLEAN => "BOOLEAN",
            BooleanConditionTypeEnum::TEXTNOTEQ => "TEXT_NOT_EQ",
            BooleanConditionTypeEnum::DATENOTEQ => "DATE_NOT_EQ",
            BooleanConditionTypeEnum::FILTEREXPRESSION => "FILTER_EXPRESSION",
        }
    }
}

impl std::convert::TryFrom< &str> for BooleanConditionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONDITION_TYPE_UNSPECIFIED" => Ok(BooleanConditionTypeEnum::CONDITIONTYPEUNSPECIFIED),
           "NUMBER_GREATER" => Ok(BooleanConditionTypeEnum::NUMBERGREATER),
           "NUMBER_GREATER_THAN_EQ" => Ok(BooleanConditionTypeEnum::NUMBERGREATERTHANEQ),
           "NUMBER_LESS" => Ok(BooleanConditionTypeEnum::NUMBERLESS),
           "NUMBER_LESS_THAN_EQ" => Ok(BooleanConditionTypeEnum::NUMBERLESSTHANEQ),
           "NUMBER_EQ" => Ok(BooleanConditionTypeEnum::NUMBEREQ),
           "NUMBER_NOT_EQ" => Ok(BooleanConditionTypeEnum::NUMBERNOTEQ),
           "NUMBER_BETWEEN" => Ok(BooleanConditionTypeEnum::NUMBERBETWEEN),
           "NUMBER_NOT_BETWEEN" => Ok(BooleanConditionTypeEnum::NUMBERNOTBETWEEN),
           "TEXT_CONTAINS" => Ok(BooleanConditionTypeEnum::TEXTCONTAINS),
           "TEXT_NOT_CONTAINS" => Ok(BooleanConditionTypeEnum::TEXTNOTCONTAINS),
           "TEXT_STARTS_WITH" => Ok(BooleanConditionTypeEnum::TEXTSTARTSWITH),
           "TEXT_ENDS_WITH" => Ok(BooleanConditionTypeEnum::TEXTENDSWITH),
           "TEXT_EQ" => Ok(BooleanConditionTypeEnum::TEXTEQ),
           "TEXT_IS_EMAIL" => Ok(BooleanConditionTypeEnum::TEXTISEMAIL),
           "TEXT_IS_URL" => Ok(BooleanConditionTypeEnum::TEXTISURL),
           "DATE_EQ" => Ok(BooleanConditionTypeEnum::DATEEQ),
           "DATE_BEFORE" => Ok(BooleanConditionTypeEnum::DATEBEFORE),
           "DATE_AFTER" => Ok(BooleanConditionTypeEnum::DATEAFTER),
           "DATE_ON_OR_BEFORE" => Ok(BooleanConditionTypeEnum::DATEONORBEFORE),
           "DATE_ON_OR_AFTER" => Ok(BooleanConditionTypeEnum::DATEONORAFTER),
           "DATE_BETWEEN" => Ok(BooleanConditionTypeEnum::DATEBETWEEN),
           "DATE_NOT_BETWEEN" => Ok(BooleanConditionTypeEnum::DATENOTBETWEEN),
           "DATE_IS_VALID" => Ok(BooleanConditionTypeEnum::DATEISVALID),
           "ONE_OF_RANGE" => Ok(BooleanConditionTypeEnum::ONEOFRANGE),
           "ONE_OF_LIST" => Ok(BooleanConditionTypeEnum::ONEOFLIST),
           "BLANK" => Ok(BooleanConditionTypeEnum::BLANK),
           "NOT_BLANK" => Ok(BooleanConditionTypeEnum::NOTBLANK),
           "CUSTOM_FORMULA" => Ok(BooleanConditionTypeEnum::CUSTOMFORMULA),
           "BOOLEAN" => Ok(BooleanConditionTypeEnum::BOOLEAN),
           "TEXT_NOT_EQ" => Ok(BooleanConditionTypeEnum::TEXTNOTEQ),
           "DATE_NOT_EQ" => Ok(BooleanConditionTypeEnum::DATENOTEQ),
           "FILTER_EXPRESSION" => Ok(BooleanConditionTypeEnum::FILTEREXPRESSION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BooleanConditionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BorderStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The style of the border.
pub enum BorderStyleEnum {
    

    /// The style is not specified. Do not use this.
    ///
    /// "STYLE_UNSPECIFIED"
    #[serde(rename="STYLE_UNSPECIFIED")]
    STYLEUNSPECIFIED,
    

    /// The border is dotted.
    ///
    /// "DOTTED"
    #[serde(rename="DOTTED")]
    DOTTED,
    

    /// The border is dashed.
    ///
    /// "DASHED"
    #[serde(rename="DASHED")]
    DASHED,
    

    /// The border is a thin solid line.
    ///
    /// "SOLID"
    #[serde(rename="SOLID")]
    SOLID,
    

    /// The border is a medium solid line.
    ///
    /// "SOLID_MEDIUM"
    #[serde(rename="SOLID_MEDIUM")]
    SOLIDMEDIUM,
    

    /// The border is a thick solid line.
    ///
    /// "SOLID_THICK"
    #[serde(rename="SOLID_THICK")]
    SOLIDTHICK,
    

    /// No border. Used only when updating a border in order to erase it.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// The border is two solid lines.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
}

impl AsRef<str> for BorderStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BorderStyleEnum::STYLEUNSPECIFIED => "STYLE_UNSPECIFIED",
            BorderStyleEnum::DOTTED => "DOTTED",
            BorderStyleEnum::DASHED => "DASHED",
            BorderStyleEnum::SOLID => "SOLID",
            BorderStyleEnum::SOLIDMEDIUM => "SOLID_MEDIUM",
            BorderStyleEnum::SOLIDTHICK => "SOLID_THICK",
            BorderStyleEnum::NONE => "NONE",
            BorderStyleEnum::DOUBLE => "DOUBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for BorderStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STYLE_UNSPECIFIED" => Ok(BorderStyleEnum::STYLEUNSPECIFIED),
           "DOTTED" => Ok(BorderStyleEnum::DOTTED),
           "DASHED" => Ok(BorderStyleEnum::DASHED),
           "SOLID" => Ok(BorderStyleEnum::SOLID),
           "SOLID_MEDIUM" => Ok(BorderStyleEnum::SOLIDMEDIUM),
           "SOLID_THICK" => Ok(BorderStyleEnum::SOLIDTHICK),
           "NONE" => Ok(BorderStyleEnum::NONE),
           "DOUBLE" => Ok(BorderStyleEnum::DOUBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BorderStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BubbleChartSpecLegendPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Where the legend of the chart should be drawn.
pub enum BubbleChartSpecLegendPositionEnum {
    

    /// Default value, do not use.
    ///
    /// "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED"
    #[serde(rename="BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED")]
    BUBBLECHARTLEGENDPOSITIONUNSPECIFIED,
    

    /// The legend is rendered on the bottom of the chart.
    ///
    /// "BOTTOM_LEGEND"
    #[serde(rename="BOTTOM_LEGEND")]
    BOTTOMLEGEND,
    

    /// The legend is rendered on the left of the chart.
    ///
    /// "LEFT_LEGEND"
    #[serde(rename="LEFT_LEGEND")]
    LEFTLEGEND,
    

    /// The legend is rendered on the right of the chart.
    ///
    /// "RIGHT_LEGEND"
    #[serde(rename="RIGHT_LEGEND")]
    RIGHTLEGEND,
    

    /// The legend is rendered on the top of the chart.
    ///
    /// "TOP_LEGEND"
    #[serde(rename="TOP_LEGEND")]
    TOPLEGEND,
    

    /// No legend is rendered.
    ///
    /// "NO_LEGEND"
    #[serde(rename="NO_LEGEND")]
    NOLEGEND,
    

    /// The legend is rendered inside the chart area.
    ///
    /// "INSIDE_LEGEND"
    #[serde(rename="INSIDE_LEGEND")]
    INSIDELEGEND,
}

impl AsRef<str> for BubbleChartSpecLegendPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BubbleChartSpecLegendPositionEnum::BUBBLECHARTLEGENDPOSITIONUNSPECIFIED => "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED",
            BubbleChartSpecLegendPositionEnum::BOTTOMLEGEND => "BOTTOM_LEGEND",
            BubbleChartSpecLegendPositionEnum::LEFTLEGEND => "LEFT_LEGEND",
            BubbleChartSpecLegendPositionEnum::RIGHTLEGEND => "RIGHT_LEGEND",
            BubbleChartSpecLegendPositionEnum::TOPLEGEND => "TOP_LEGEND",
            BubbleChartSpecLegendPositionEnum::NOLEGEND => "NO_LEGEND",
            BubbleChartSpecLegendPositionEnum::INSIDELEGEND => "INSIDE_LEGEND",
        }
    }
}

impl std::convert::TryFrom< &str> for BubbleChartSpecLegendPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUBBLE_CHART_LEGEND_POSITION_UNSPECIFIED" => Ok(BubbleChartSpecLegendPositionEnum::BUBBLECHARTLEGENDPOSITIONUNSPECIFIED),
           "BOTTOM_LEGEND" => Ok(BubbleChartSpecLegendPositionEnum::BOTTOMLEGEND),
           "LEFT_LEGEND" => Ok(BubbleChartSpecLegendPositionEnum::LEFTLEGEND),
           "RIGHT_LEGEND" => Ok(BubbleChartSpecLegendPositionEnum::RIGHTLEGEND),
           "TOP_LEGEND" => Ok(BubbleChartSpecLegendPositionEnum::TOPLEGEND),
           "NO_LEGEND" => Ok(BubbleChartSpecLegendPositionEnum::NOLEGEND),
           "INSIDE_LEGEND" => Ok(BubbleChartSpecLegendPositionEnum::INSIDELEGEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BubbleChartSpecLegendPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CellFormatHorizontalAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The horizontal alignment of the value in the cell.
pub enum CellFormatHorizontalAlignmentEnum {
    

    /// The horizontal alignment is not specified. Do not use this.
    ///
    /// "HORIZONTAL_ALIGN_UNSPECIFIED"
    #[serde(rename="HORIZONTAL_ALIGN_UNSPECIFIED")]
    HORIZONTALALIGNUNSPECIFIED,
    

    /// The text is explicitly aligned to the left of the cell.
    ///
    /// "LEFT"
    #[serde(rename="LEFT")]
    LEFT,
    

    /// The text is explicitly aligned to the center of the cell.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// The text is explicitly aligned to the right of the cell.
    ///
    /// "RIGHT"
    #[serde(rename="RIGHT")]
    RIGHT,
}

impl AsRef<str> for CellFormatHorizontalAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CellFormatHorizontalAlignmentEnum::HORIZONTALALIGNUNSPECIFIED => "HORIZONTAL_ALIGN_UNSPECIFIED",
            CellFormatHorizontalAlignmentEnum::LEFT => "LEFT",
            CellFormatHorizontalAlignmentEnum::CENTER => "CENTER",
            CellFormatHorizontalAlignmentEnum::RIGHT => "RIGHT",
        }
    }
}

impl std::convert::TryFrom< &str> for CellFormatHorizontalAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HORIZONTAL_ALIGN_UNSPECIFIED" => Ok(CellFormatHorizontalAlignmentEnum::HORIZONTALALIGNUNSPECIFIED),
           "LEFT" => Ok(CellFormatHorizontalAlignmentEnum::LEFT),
           "CENTER" => Ok(CellFormatHorizontalAlignmentEnum::CENTER),
           "RIGHT" => Ok(CellFormatHorizontalAlignmentEnum::RIGHT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CellFormatHorizontalAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CellFormatHyperlinkDisplayTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If one exists, how a hyperlink should be displayed in the cell.
pub enum CellFormatHyperlinkDisplayTypeEnum {
    

    /// The default value: the hyperlink is rendered. Do not use this.
    ///
    /// "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED"
    #[serde(rename="HYPERLINK_DISPLAY_TYPE_UNSPECIFIED")]
    HYPERLINKDISPLAYTYPEUNSPECIFIED,
    

    /// A hyperlink should be explicitly rendered.
    ///
    /// "LINKED"
    #[serde(rename="LINKED")]
    LINKED,
    

    /// A hyperlink should not be rendered.
    ///
    /// "PLAIN_TEXT"
    #[serde(rename="PLAIN_TEXT")]
    PLAINTEXT,
}

impl AsRef<str> for CellFormatHyperlinkDisplayTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CellFormatHyperlinkDisplayTypeEnum::HYPERLINKDISPLAYTYPEUNSPECIFIED => "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED",
            CellFormatHyperlinkDisplayTypeEnum::LINKED => "LINKED",
            CellFormatHyperlinkDisplayTypeEnum::PLAINTEXT => "PLAIN_TEXT",
        }
    }
}

impl std::convert::TryFrom< &str> for CellFormatHyperlinkDisplayTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HYPERLINK_DISPLAY_TYPE_UNSPECIFIED" => Ok(CellFormatHyperlinkDisplayTypeEnum::HYPERLINKDISPLAYTYPEUNSPECIFIED),
           "LINKED" => Ok(CellFormatHyperlinkDisplayTypeEnum::LINKED),
           "PLAIN_TEXT" => Ok(CellFormatHyperlinkDisplayTypeEnum::PLAINTEXT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CellFormatHyperlinkDisplayTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CellFormatTextDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The direction of the text in the cell.
pub enum CellFormatTextDirectionEnum {
    

    /// The text direction is not specified. Do not use this.
    ///
    /// "TEXT_DIRECTION_UNSPECIFIED"
    #[serde(rename="TEXT_DIRECTION_UNSPECIFIED")]
    TEXTDIRECTIONUNSPECIFIED,
    

    /// The text direction of left-to-right was set by the user.
    ///
    /// "LEFT_TO_RIGHT"
    #[serde(rename="LEFT_TO_RIGHT")]
    LEFTTORIGHT,
    

    /// The text direction of right-to-left was set by the user.
    ///
    /// "RIGHT_TO_LEFT"
    #[serde(rename="RIGHT_TO_LEFT")]
    RIGHTTOLEFT,
}

impl AsRef<str> for CellFormatTextDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CellFormatTextDirectionEnum::TEXTDIRECTIONUNSPECIFIED => "TEXT_DIRECTION_UNSPECIFIED",
            CellFormatTextDirectionEnum::LEFTTORIGHT => "LEFT_TO_RIGHT",
            CellFormatTextDirectionEnum::RIGHTTOLEFT => "RIGHT_TO_LEFT",
        }
    }
}

impl std::convert::TryFrom< &str> for CellFormatTextDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEXT_DIRECTION_UNSPECIFIED" => Ok(CellFormatTextDirectionEnum::TEXTDIRECTIONUNSPECIFIED),
           "LEFT_TO_RIGHT" => Ok(CellFormatTextDirectionEnum::LEFTTORIGHT),
           "RIGHT_TO_LEFT" => Ok(CellFormatTextDirectionEnum::RIGHTTOLEFT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CellFormatTextDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CellFormatVerticalAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The vertical alignment of the value in the cell.
pub enum CellFormatVerticalAlignmentEnum {
    

    /// The vertical alignment is not specified. Do not use this.
    ///
    /// "VERTICAL_ALIGN_UNSPECIFIED"
    #[serde(rename="VERTICAL_ALIGN_UNSPECIFIED")]
    VERTICALALIGNUNSPECIFIED,
    

    /// The text is explicitly aligned to the top of the cell.
    ///
    /// "TOP"
    #[serde(rename="TOP")]
    TOP,
    

    /// The text is explicitly aligned to the middle of the cell.
    ///
    /// "MIDDLE"
    #[serde(rename="MIDDLE")]
    MIDDLE,
    

    /// The text is explicitly aligned to the bottom of the cell.
    ///
    /// "BOTTOM"
    #[serde(rename="BOTTOM")]
    BOTTOM,
}

impl AsRef<str> for CellFormatVerticalAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CellFormatVerticalAlignmentEnum::VERTICALALIGNUNSPECIFIED => "VERTICAL_ALIGN_UNSPECIFIED",
            CellFormatVerticalAlignmentEnum::TOP => "TOP",
            CellFormatVerticalAlignmentEnum::MIDDLE => "MIDDLE",
            CellFormatVerticalAlignmentEnum::BOTTOM => "BOTTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for CellFormatVerticalAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERTICAL_ALIGN_UNSPECIFIED" => Ok(CellFormatVerticalAlignmentEnum::VERTICALALIGNUNSPECIFIED),
           "TOP" => Ok(CellFormatVerticalAlignmentEnum::TOP),
           "MIDDLE" => Ok(CellFormatVerticalAlignmentEnum::MIDDLE),
           "BOTTOM" => Ok(CellFormatVerticalAlignmentEnum::BOTTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CellFormatVerticalAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CellFormatWrapStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The wrap strategy for the value in the cell.
pub enum CellFormatWrapStrategyEnum {
    

    /// The default value, do not use.
    ///
    /// "WRAP_STRATEGY_UNSPECIFIED"
    #[serde(rename="WRAP_STRATEGY_UNSPECIFIED")]
    WRAPSTRATEGYUNSPECIFIED,
    

    /// Lines that are longer than the cell width will be written in the next cell over, so long as that cell is empty. If the next cell over is non-empty, this behaves the same as `CLIP`. The text will never wrap to the next line unless the user manually inserts a new line. Example: | First sentence. | | Manual newline that is very long. <- Text continues into next cell | Next newline. |
    ///
    /// "OVERFLOW_CELL"
    #[serde(rename="OVERFLOW_CELL")]
    OVERFLOWCELL,
    

    /// This wrap strategy represents the old Google Sheets wrap strategy where words that are longer than a line are clipped rather than broken. This strategy is not supported on all platforms and is being phased out. Example: | Cell has a | | loooooooooo| <- Word is clipped. | word. |
    ///
    /// "LEGACY_WRAP"
    #[serde(rename="LEGACY_WRAP")]
    LEGACYWRAP,
    

    /// Lines that are longer than the cell width will be clipped. The text will never wrap to the next line unless the user manually inserts a new line. Example: | First sentence. | | Manual newline t| <- Text is clipped | Next newline. |
    ///
    /// "CLIP"
    #[serde(rename="CLIP")]
    CLIP,
    

    /// Words that are longer than a line are wrapped at the character level rather than clipped. Example: | Cell has a | | loooooooooo| <- Word is broken. | ong word. |
    ///
    /// "WRAP"
    #[serde(rename="WRAP")]
    WRAP,
}

impl AsRef<str> for CellFormatWrapStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CellFormatWrapStrategyEnum::WRAPSTRATEGYUNSPECIFIED => "WRAP_STRATEGY_UNSPECIFIED",
            CellFormatWrapStrategyEnum::OVERFLOWCELL => "OVERFLOW_CELL",
            CellFormatWrapStrategyEnum::LEGACYWRAP => "LEGACY_WRAP",
            CellFormatWrapStrategyEnum::CLIP => "CLIP",
            CellFormatWrapStrategyEnum::WRAP => "WRAP",
        }
    }
}

impl std::convert::TryFrom< &str> for CellFormatWrapStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRAP_STRATEGY_UNSPECIFIED" => Ok(CellFormatWrapStrategyEnum::WRAPSTRATEGYUNSPECIFIED),
           "OVERFLOW_CELL" => Ok(CellFormatWrapStrategyEnum::OVERFLOWCELL),
           "LEGACY_WRAP" => Ok(CellFormatWrapStrategyEnum::LEGACYWRAP),
           "CLIP" => Ok(CellFormatWrapStrategyEnum::CLIP),
           "WRAP" => Ok(CellFormatWrapStrategyEnum::WRAP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CellFormatWrapStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChartAxisViewWindowOptionViewWindowModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The view window's mode.
pub enum ChartAxisViewWindowOptionViewWindowModeEnum {
    

    /// The default view window mode used in the Sheets editor for this chart type. In most cases, if set, the default mode is equivalent to `PRETTY`.
    ///
    /// "DEFAULT_VIEW_WINDOW_MODE"
    #[serde(rename="DEFAULT_VIEW_WINDOW_MODE")]
    DEFAULTVIEWWINDOWMODE,
    

    /// Do not use. Represents that the currently set mode is not supported by the API.
    ///
    /// "VIEW_WINDOW_MODE_UNSUPPORTED"
    #[serde(rename="VIEW_WINDOW_MODE_UNSUPPORTED")]
    VIEWWINDOWMODEUNSUPPORTED,
    

    /// Follows the min and max exactly if specified. If a value is unspecified, it will fall back to the `PRETTY` value.
    ///
    /// "EXPLICIT"
    #[serde(rename="EXPLICIT")]
    EXPLICIT,
    

    /// Chooses a min and max that make the chart look good. Both min and max are ignored in this mode.
    ///
    /// "PRETTY"
    #[serde(rename="PRETTY")]
    PRETTY,
}

impl AsRef<str> for ChartAxisViewWindowOptionViewWindowModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChartAxisViewWindowOptionViewWindowModeEnum::DEFAULTVIEWWINDOWMODE => "DEFAULT_VIEW_WINDOW_MODE",
            ChartAxisViewWindowOptionViewWindowModeEnum::VIEWWINDOWMODEUNSUPPORTED => "VIEW_WINDOW_MODE_UNSUPPORTED",
            ChartAxisViewWindowOptionViewWindowModeEnum::EXPLICIT => "EXPLICIT",
            ChartAxisViewWindowOptionViewWindowModeEnum::PRETTY => "PRETTY",
        }
    }
}

impl std::convert::TryFrom< &str> for ChartAxisViewWindowOptionViewWindowModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT_VIEW_WINDOW_MODE" => Ok(ChartAxisViewWindowOptionViewWindowModeEnum::DEFAULTVIEWWINDOWMODE),
           "VIEW_WINDOW_MODE_UNSUPPORTED" => Ok(ChartAxisViewWindowOptionViewWindowModeEnum::VIEWWINDOWMODEUNSUPPORTED),
           "EXPLICIT" => Ok(ChartAxisViewWindowOptionViewWindowModeEnum::EXPLICIT),
           "PRETTY" => Ok(ChartAxisViewWindowOptionViewWindowModeEnum::PRETTY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChartAxisViewWindowOptionViewWindowModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChartDataAggregateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The aggregation type for the series of a data source chart. Only supported for data source charts.
pub enum ChartDataAggregateTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "CHART_AGGREGATE_TYPE_UNSPECIFIED"
    #[serde(rename="CHART_AGGREGATE_TYPE_UNSPECIFIED")]
    CHARTAGGREGATETYPEUNSPECIFIED,
    

    /// Average aggregate function.
    ///
    /// "AVERAGE"
    #[serde(rename="AVERAGE")]
    AVERAGE,
    

    /// Count aggregate function.
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
    

    /// Maximum aggregate function.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
    

    /// Median aggregate function.
    ///
    /// "MEDIAN"
    #[serde(rename="MEDIAN")]
    MEDIAN,
    

    /// Minimum aggregate function.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Sum aggregate function.
    ///
    /// "SUM"
    #[serde(rename="SUM")]
    SUM,
}

impl AsRef<str> for ChartDataAggregateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChartDataAggregateTypeEnum::CHARTAGGREGATETYPEUNSPECIFIED => "CHART_AGGREGATE_TYPE_UNSPECIFIED",
            ChartDataAggregateTypeEnum::AVERAGE => "AVERAGE",
            ChartDataAggregateTypeEnum::COUNT => "COUNT",
            ChartDataAggregateTypeEnum::MAX => "MAX",
            ChartDataAggregateTypeEnum::MEDIAN => "MEDIAN",
            ChartDataAggregateTypeEnum::MIN => "MIN",
            ChartDataAggregateTypeEnum::SUM => "SUM",
        }
    }
}

impl std::convert::TryFrom< &str> for ChartDataAggregateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHART_AGGREGATE_TYPE_UNSPECIFIED" => Ok(ChartDataAggregateTypeEnum::CHARTAGGREGATETYPEUNSPECIFIED),
           "AVERAGE" => Ok(ChartDataAggregateTypeEnum::AVERAGE),
           "COUNT" => Ok(ChartDataAggregateTypeEnum::COUNT),
           "MAX" => Ok(ChartDataAggregateTypeEnum::MAX),
           "MEDIAN" => Ok(ChartDataAggregateTypeEnum::MEDIAN),
           "MIN" => Ok(ChartDataAggregateTypeEnum::MIN),
           "SUM" => Ok(ChartDataAggregateTypeEnum::SUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChartDataAggregateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChartDateTimeRuleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of date-time grouping to apply.
pub enum ChartDateTimeRuleTypeEnum {
    

    /// The default type, do not use.
    ///
    /// "CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED"
    #[serde(rename="CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED")]
    CHARTDATETIMERULETYPEUNSPECIFIED,
    

    /// Group dates by second, from 0 to 59.
    ///
    /// "SECOND"
    #[serde(rename="SECOND")]
    SECOND,
    

    /// Group dates by minute, from 0 to 59.
    ///
    /// "MINUTE"
    #[serde(rename="MINUTE")]
    MINUTE,
    

    /// Group dates by hour using a 24-hour system, from 0 to 23.
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
    

    /// Group dates by hour and minute using a 24-hour system, for example 19:45.
    ///
    /// "HOUR_MINUTE"
    #[serde(rename="HOUR_MINUTE")]
    HOURMINUTE,
    

    /// Group dates by hour and minute using a 12-hour system, for example 7:45 PM. The AM/PM designation is translated based on the spreadsheet locale.
    ///
    /// "HOUR_MINUTE_AMPM"
    #[serde(rename="HOUR_MINUTE_AMPM")]
    HOURMINUTEAMPM,
    

    /// Group dates by day of week, for example Sunday. The days of the week will be translated based on the spreadsheet locale.
    ///
    /// "DAY_OF_WEEK"
    #[serde(rename="DAY_OF_WEEK")]
    DAYOFWEEK,
    

    /// Group dates by day of year, from 1 to 366. Note that dates after Feb. 29 fall in different buckets in leap years than in non-leap years.
    ///
    /// "DAY_OF_YEAR"
    #[serde(rename="DAY_OF_YEAR")]
    DAYOFYEAR,
    

    /// Group dates by day of month, from 1 to 31.
    ///
    /// "DAY_OF_MONTH"
    #[serde(rename="DAY_OF_MONTH")]
    DAYOFMONTH,
    

    /// Group dates by day and month, for example 22-Nov. The month is translated based on the spreadsheet locale.
    ///
    /// "DAY_MONTH"
    #[serde(rename="DAY_MONTH")]
    DAYMONTH,
    

    /// Group dates by month, for example Nov. The month is translated based on the spreadsheet locale.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Group dates by quarter, for example Q1 (which represents Jan-Mar).
    ///
    /// "QUARTER"
    #[serde(rename="QUARTER")]
    QUARTER,
    

    /// Group dates by year, for example 2008.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
    

    /// Group dates by year and month, for example 2008-Nov. The month is translated based on the spreadsheet locale.
    ///
    /// "YEAR_MONTH"
    #[serde(rename="YEAR_MONTH")]
    YEARMONTH,
    

    /// Group dates by year and quarter, for example 2008 Q4.
    ///
    /// "YEAR_QUARTER"
    #[serde(rename="YEAR_QUARTER")]
    YEARQUARTER,
    

    /// Group dates by year, month, and day, for example 2008-11-22.
    ///
    /// "YEAR_MONTH_DAY"
    #[serde(rename="YEAR_MONTH_DAY")]
    YEARMONTHDAY,
}

impl AsRef<str> for ChartDateTimeRuleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChartDateTimeRuleTypeEnum::CHARTDATETIMERULETYPEUNSPECIFIED => "CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED",
            ChartDateTimeRuleTypeEnum::SECOND => "SECOND",
            ChartDateTimeRuleTypeEnum::MINUTE => "MINUTE",
            ChartDateTimeRuleTypeEnum::HOUR => "HOUR",
            ChartDateTimeRuleTypeEnum::HOURMINUTE => "HOUR_MINUTE",
            ChartDateTimeRuleTypeEnum::HOURMINUTEAMPM => "HOUR_MINUTE_AMPM",
            ChartDateTimeRuleTypeEnum::DAYOFWEEK => "DAY_OF_WEEK",
            ChartDateTimeRuleTypeEnum::DAYOFYEAR => "DAY_OF_YEAR",
            ChartDateTimeRuleTypeEnum::DAYOFMONTH => "DAY_OF_MONTH",
            ChartDateTimeRuleTypeEnum::DAYMONTH => "DAY_MONTH",
            ChartDateTimeRuleTypeEnum::MONTH => "MONTH",
            ChartDateTimeRuleTypeEnum::QUARTER => "QUARTER",
            ChartDateTimeRuleTypeEnum::YEAR => "YEAR",
            ChartDateTimeRuleTypeEnum::YEARMONTH => "YEAR_MONTH",
            ChartDateTimeRuleTypeEnum::YEARQUARTER => "YEAR_QUARTER",
            ChartDateTimeRuleTypeEnum::YEARMONTHDAY => "YEAR_MONTH_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for ChartDateTimeRuleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHART_DATE_TIME_RULE_TYPE_UNSPECIFIED" => Ok(ChartDateTimeRuleTypeEnum::CHARTDATETIMERULETYPEUNSPECIFIED),
           "SECOND" => Ok(ChartDateTimeRuleTypeEnum::SECOND),
           "MINUTE" => Ok(ChartDateTimeRuleTypeEnum::MINUTE),
           "HOUR" => Ok(ChartDateTimeRuleTypeEnum::HOUR),
           "HOUR_MINUTE" => Ok(ChartDateTimeRuleTypeEnum::HOURMINUTE),
           "HOUR_MINUTE_AMPM" => Ok(ChartDateTimeRuleTypeEnum::HOURMINUTEAMPM),
           "DAY_OF_WEEK" => Ok(ChartDateTimeRuleTypeEnum::DAYOFWEEK),
           "DAY_OF_YEAR" => Ok(ChartDateTimeRuleTypeEnum::DAYOFYEAR),
           "DAY_OF_MONTH" => Ok(ChartDateTimeRuleTypeEnum::DAYOFMONTH),
           "DAY_MONTH" => Ok(ChartDateTimeRuleTypeEnum::DAYMONTH),
           "MONTH" => Ok(ChartDateTimeRuleTypeEnum::MONTH),
           "QUARTER" => Ok(ChartDateTimeRuleTypeEnum::QUARTER),
           "YEAR" => Ok(ChartDateTimeRuleTypeEnum::YEAR),
           "YEAR_MONTH" => Ok(ChartDateTimeRuleTypeEnum::YEARMONTH),
           "YEAR_QUARTER" => Ok(ChartDateTimeRuleTypeEnum::YEARQUARTER),
           "YEAR_MONTH_DAY" => Ok(ChartDateTimeRuleTypeEnum::YEARMONTHDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChartDateTimeRuleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChartSpecHiddenDimensionStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how the charts will use hidden rows or columns.
pub enum ChartSpecHiddenDimensionStrategyEnum {
    

    /// Default value, do not use.
    ///
    /// "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED"
    #[serde(rename="CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED")]
    CHARTHIDDENDIMENSIONSTRATEGYUNSPECIFIED,
    

    /// Charts will skip hidden rows and columns.
    ///
    /// "SKIP_HIDDEN_ROWS_AND_COLUMNS"
    #[serde(rename="SKIP_HIDDEN_ROWS_AND_COLUMNS")]
    SKIPHIDDENROWSANDCOLUMNS,
    

    /// Charts will skip hidden rows only.
    ///
    /// "SKIP_HIDDEN_ROWS"
    #[serde(rename="SKIP_HIDDEN_ROWS")]
    SKIPHIDDENROWS,
    

    /// Charts will skip hidden columns only.
    ///
    /// "SKIP_HIDDEN_COLUMNS"
    #[serde(rename="SKIP_HIDDEN_COLUMNS")]
    SKIPHIDDENCOLUMNS,
    

    /// Charts will not skip any hidden rows or columns.
    ///
    /// "SHOW_ALL"
    #[serde(rename="SHOW_ALL")]
    SHOWALL,
}

impl AsRef<str> for ChartSpecHiddenDimensionStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChartSpecHiddenDimensionStrategyEnum::CHARTHIDDENDIMENSIONSTRATEGYUNSPECIFIED => "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED",
            ChartSpecHiddenDimensionStrategyEnum::SKIPHIDDENROWSANDCOLUMNS => "SKIP_HIDDEN_ROWS_AND_COLUMNS",
            ChartSpecHiddenDimensionStrategyEnum::SKIPHIDDENROWS => "SKIP_HIDDEN_ROWS",
            ChartSpecHiddenDimensionStrategyEnum::SKIPHIDDENCOLUMNS => "SKIP_HIDDEN_COLUMNS",
            ChartSpecHiddenDimensionStrategyEnum::SHOWALL => "SHOW_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for ChartSpecHiddenDimensionStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHART_HIDDEN_DIMENSION_STRATEGY_UNSPECIFIED" => Ok(ChartSpecHiddenDimensionStrategyEnum::CHARTHIDDENDIMENSIONSTRATEGYUNSPECIFIED),
           "SKIP_HIDDEN_ROWS_AND_COLUMNS" => Ok(ChartSpecHiddenDimensionStrategyEnum::SKIPHIDDENROWSANDCOLUMNS),
           "SKIP_HIDDEN_ROWS" => Ok(ChartSpecHiddenDimensionStrategyEnum::SKIPHIDDENROWS),
           "SKIP_HIDDEN_COLUMNS" => Ok(ChartSpecHiddenDimensionStrategyEnum::SKIPHIDDENCOLUMNS),
           "SHOW_ALL" => Ok(ChartSpecHiddenDimensionStrategyEnum::SHOWALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChartSpecHiddenDimensionStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ColorStyleThemeColorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Theme color.
pub enum ColorStyleThemeColorEnum {
    

    /// Unspecified theme color
    ///
    /// "THEME_COLOR_TYPE_UNSPECIFIED"
    #[serde(rename="THEME_COLOR_TYPE_UNSPECIFIED")]
    THEMECOLORTYPEUNSPECIFIED,
    

    /// Represents the primary text color
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Represents the primary background color
    ///
    /// "BACKGROUND"
    #[serde(rename="BACKGROUND")]
    BACKGROUND,
    

    /// Represents the first accent color
    ///
    /// "ACCENT1"
    #[serde(rename="ACCENT1")]
    ACCENT1,
    

    /// Represents the second accent color
    ///
    /// "ACCENT2"
    #[serde(rename="ACCENT2")]
    ACCENT2,
    

    /// Represents the third accent color
    ///
    /// "ACCENT3"
    #[serde(rename="ACCENT3")]
    ACCENT3,
    

    /// Represents the fourth accent color
    ///
    /// "ACCENT4"
    #[serde(rename="ACCENT4")]
    ACCENT4,
    

    /// Represents the fifth accent color
    ///
    /// "ACCENT5"
    #[serde(rename="ACCENT5")]
    ACCENT5,
    

    /// Represents the sixth accent color
    ///
    /// "ACCENT6"
    #[serde(rename="ACCENT6")]
    ACCENT6,
    

    /// Represents the color to use for hyperlinks
    ///
    /// "LINK"
    #[serde(rename="LINK")]
    LINK,
}

impl AsRef<str> for ColorStyleThemeColorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ColorStyleThemeColorEnum::THEMECOLORTYPEUNSPECIFIED => "THEME_COLOR_TYPE_UNSPECIFIED",
            ColorStyleThemeColorEnum::TEXT => "TEXT",
            ColorStyleThemeColorEnum::BACKGROUND => "BACKGROUND",
            ColorStyleThemeColorEnum::ACCENT1 => "ACCENT1",
            ColorStyleThemeColorEnum::ACCENT2 => "ACCENT2",
            ColorStyleThemeColorEnum::ACCENT3 => "ACCENT3",
            ColorStyleThemeColorEnum::ACCENT4 => "ACCENT4",
            ColorStyleThemeColorEnum::ACCENT5 => "ACCENT5",
            ColorStyleThemeColorEnum::ACCENT6 => "ACCENT6",
            ColorStyleThemeColorEnum::LINK => "LINK",
        }
    }
}

impl std::convert::TryFrom< &str> for ColorStyleThemeColorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THEME_COLOR_TYPE_UNSPECIFIED" => Ok(ColorStyleThemeColorEnum::THEMECOLORTYPEUNSPECIFIED),
           "TEXT" => Ok(ColorStyleThemeColorEnum::TEXT),
           "BACKGROUND" => Ok(ColorStyleThemeColorEnum::BACKGROUND),
           "ACCENT1" => Ok(ColorStyleThemeColorEnum::ACCENT1),
           "ACCENT2" => Ok(ColorStyleThemeColorEnum::ACCENT2),
           "ACCENT3" => Ok(ColorStyleThemeColorEnum::ACCENT3),
           "ACCENT4" => Ok(ColorStyleThemeColorEnum::ACCENT4),
           "ACCENT5" => Ok(ColorStyleThemeColorEnum::ACCENT5),
           "ACCENT6" => Ok(ColorStyleThemeColorEnum::ACCENT6),
           "LINK" => Ok(ColorStyleThemeColorEnum::LINK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ColorStyleThemeColorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConditionValueRelativeDateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A relative date (based on the current date). Valid only if the type is DATE_BEFORE, DATE_AFTER, DATE_ON_OR_BEFORE or DATE_ON_OR_AFTER. Relative dates are not supported in data validation. They are supported only in conditional formatting and conditional filters.
pub enum ConditionValueRelativeDateEnum {
    

    /// Default value, do not use.
    ///
    /// "RELATIVE_DATE_UNSPECIFIED"
    #[serde(rename="RELATIVE_DATE_UNSPECIFIED")]
    RELATIVEDATEUNSPECIFIED,
    

    /// The value is one year before today.
    ///
    /// "PAST_YEAR"
    #[serde(rename="PAST_YEAR")]
    PASTYEAR,
    

    /// The value is one month before today.
    ///
    /// "PAST_MONTH"
    #[serde(rename="PAST_MONTH")]
    PASTMONTH,
    

    /// The value is one week before today.
    ///
    /// "PAST_WEEK"
    #[serde(rename="PAST_WEEK")]
    PASTWEEK,
    

    /// The value is yesterday.
    ///
    /// "YESTERDAY"
    #[serde(rename="YESTERDAY")]
    YESTERDAY,
    

    /// The value is today.
    ///
    /// "TODAY"
    #[serde(rename="TODAY")]
    TODAY,
    

    /// The value is tomorrow.
    ///
    /// "TOMORROW"
    #[serde(rename="TOMORROW")]
    TOMORROW,
}

impl AsRef<str> for ConditionValueRelativeDateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionValueRelativeDateEnum::RELATIVEDATEUNSPECIFIED => "RELATIVE_DATE_UNSPECIFIED",
            ConditionValueRelativeDateEnum::PASTYEAR => "PAST_YEAR",
            ConditionValueRelativeDateEnum::PASTMONTH => "PAST_MONTH",
            ConditionValueRelativeDateEnum::PASTWEEK => "PAST_WEEK",
            ConditionValueRelativeDateEnum::YESTERDAY => "YESTERDAY",
            ConditionValueRelativeDateEnum::TODAY => "TODAY",
            ConditionValueRelativeDateEnum::TOMORROW => "TOMORROW",
        }
    }
}

impl std::convert::TryFrom< &str> for ConditionValueRelativeDateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATIVE_DATE_UNSPECIFIED" => Ok(ConditionValueRelativeDateEnum::RELATIVEDATEUNSPECIFIED),
           "PAST_YEAR" => Ok(ConditionValueRelativeDateEnum::PASTYEAR),
           "PAST_MONTH" => Ok(ConditionValueRelativeDateEnum::PASTMONTH),
           "PAST_WEEK" => Ok(ConditionValueRelativeDateEnum::PASTWEEK),
           "YESTERDAY" => Ok(ConditionValueRelativeDateEnum::YESTERDAY),
           "TODAY" => Ok(ConditionValueRelativeDateEnum::TODAY),
           "TOMORROW" => Ok(ConditionValueRelativeDateEnum::TOMORROW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConditionValueRelativeDateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CopyPasteRequestPasteOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How that data should be oriented when pasting.
pub enum CopyPasteRequestPasteOrientationEnum {
    

    /// Paste normally.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// Paste transposed, where all rows become columns and vice versa.
    ///
    /// "TRANSPOSE"
    #[serde(rename="TRANSPOSE")]
    TRANSPOSE,
}

impl AsRef<str> for CopyPasteRequestPasteOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CopyPasteRequestPasteOrientationEnum::NORMAL => "NORMAL",
            CopyPasteRequestPasteOrientationEnum::TRANSPOSE => "TRANSPOSE",
        }
    }
}

impl std::convert::TryFrom< &str> for CopyPasteRequestPasteOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NORMAL" => Ok(CopyPasteRequestPasteOrientationEnum::NORMAL),
           "TRANSPOSE" => Ok(CopyPasteRequestPasteOrientationEnum::TRANSPOSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CopyPasteRequestPasteOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CopyPasteRequestPasteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What kind of data to paste.
pub enum CopyPasteRequestPasteTypeEnum {
    

    /// Paste values, formulas, formats, and merges.
    ///
    /// "PASTE_NORMAL"
    #[serde(rename="PASTE_NORMAL")]
    PASTENORMAL,
    

    /// Paste the values ONLY without formats, formulas, or merges.
    ///
    /// "PASTE_VALUES"
    #[serde(rename="PASTE_VALUES")]
    PASTEVALUES,
    

    /// Paste the format and data validation only.
    ///
    /// "PASTE_FORMAT"
    #[serde(rename="PASTE_FORMAT")]
    PASTEFORMAT,
    

    /// Like `PASTE_NORMAL` but without borders.
    ///
    /// "PASTE_NO_BORDERS"
    #[serde(rename="PASTE_NO_BORDERS")]
    PASTENOBORDERS,
    

    /// Paste the formulas only.
    ///
    /// "PASTE_FORMULA"
    #[serde(rename="PASTE_FORMULA")]
    PASTEFORMULA,
    

    /// Paste the data validation only.
    ///
    /// "PASTE_DATA_VALIDATION"
    #[serde(rename="PASTE_DATA_VALIDATION")]
    PASTEDATAVALIDATION,
    

    /// Paste the conditional formatting rules only.
    ///
    /// "PASTE_CONDITIONAL_FORMATTING"
    #[serde(rename="PASTE_CONDITIONAL_FORMATTING")]
    PASTECONDITIONALFORMATTING,
}

impl AsRef<str> for CopyPasteRequestPasteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CopyPasteRequestPasteTypeEnum::PASTENORMAL => "PASTE_NORMAL",
            CopyPasteRequestPasteTypeEnum::PASTEVALUES => "PASTE_VALUES",
            CopyPasteRequestPasteTypeEnum::PASTEFORMAT => "PASTE_FORMAT",
            CopyPasteRequestPasteTypeEnum::PASTENOBORDERS => "PASTE_NO_BORDERS",
            CopyPasteRequestPasteTypeEnum::PASTEFORMULA => "PASTE_FORMULA",
            CopyPasteRequestPasteTypeEnum::PASTEDATAVALIDATION => "PASTE_DATA_VALIDATION",
            CopyPasteRequestPasteTypeEnum::PASTECONDITIONALFORMATTING => "PASTE_CONDITIONAL_FORMATTING",
        }
    }
}

impl std::convert::TryFrom< &str> for CopyPasteRequestPasteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASTE_NORMAL" => Ok(CopyPasteRequestPasteTypeEnum::PASTENORMAL),
           "PASTE_VALUES" => Ok(CopyPasteRequestPasteTypeEnum::PASTEVALUES),
           "PASTE_FORMAT" => Ok(CopyPasteRequestPasteTypeEnum::PASTEFORMAT),
           "PASTE_NO_BORDERS" => Ok(CopyPasteRequestPasteTypeEnum::PASTENOBORDERS),
           "PASTE_FORMULA" => Ok(CopyPasteRequestPasteTypeEnum::PASTEFORMULA),
           "PASTE_DATA_VALIDATION" => Ok(CopyPasteRequestPasteTypeEnum::PASTEDATAVALIDATION),
           "PASTE_CONDITIONAL_FORMATTING" => Ok(CopyPasteRequestPasteTypeEnum::PASTECONDITIONALFORMATTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CopyPasteRequestPasteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CutPasteRequestPasteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What kind of data to paste. All the source data will be cut, regardless of what is pasted.
pub enum CutPasteRequestPasteTypeEnum {
    

    /// Paste values, formulas, formats, and merges.
    ///
    /// "PASTE_NORMAL"
    #[serde(rename="PASTE_NORMAL")]
    PASTENORMAL,
    

    /// Paste the values ONLY without formats, formulas, or merges.
    ///
    /// "PASTE_VALUES"
    #[serde(rename="PASTE_VALUES")]
    PASTEVALUES,
    

    /// Paste the format and data validation only.
    ///
    /// "PASTE_FORMAT"
    #[serde(rename="PASTE_FORMAT")]
    PASTEFORMAT,
    

    /// Like `PASTE_NORMAL` but without borders.
    ///
    /// "PASTE_NO_BORDERS"
    #[serde(rename="PASTE_NO_BORDERS")]
    PASTENOBORDERS,
    

    /// Paste the formulas only.
    ///
    /// "PASTE_FORMULA"
    #[serde(rename="PASTE_FORMULA")]
    PASTEFORMULA,
    

    /// Paste the data validation only.
    ///
    /// "PASTE_DATA_VALIDATION"
    #[serde(rename="PASTE_DATA_VALIDATION")]
    PASTEDATAVALIDATION,
    

    /// Paste the conditional formatting rules only.
    ///
    /// "PASTE_CONDITIONAL_FORMATTING"
    #[serde(rename="PASTE_CONDITIONAL_FORMATTING")]
    PASTECONDITIONALFORMATTING,
}

impl AsRef<str> for CutPasteRequestPasteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CutPasteRequestPasteTypeEnum::PASTENORMAL => "PASTE_NORMAL",
            CutPasteRequestPasteTypeEnum::PASTEVALUES => "PASTE_VALUES",
            CutPasteRequestPasteTypeEnum::PASTEFORMAT => "PASTE_FORMAT",
            CutPasteRequestPasteTypeEnum::PASTENOBORDERS => "PASTE_NO_BORDERS",
            CutPasteRequestPasteTypeEnum::PASTEFORMULA => "PASTE_FORMULA",
            CutPasteRequestPasteTypeEnum::PASTEDATAVALIDATION => "PASTE_DATA_VALIDATION",
            CutPasteRequestPasteTypeEnum::PASTECONDITIONALFORMATTING => "PASTE_CONDITIONAL_FORMATTING",
        }
    }
}

impl std::convert::TryFrom< &str> for CutPasteRequestPasteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASTE_NORMAL" => Ok(CutPasteRequestPasteTypeEnum::PASTENORMAL),
           "PASTE_VALUES" => Ok(CutPasteRequestPasteTypeEnum::PASTEVALUES),
           "PASTE_FORMAT" => Ok(CutPasteRequestPasteTypeEnum::PASTEFORMAT),
           "PASTE_NO_BORDERS" => Ok(CutPasteRequestPasteTypeEnum::PASTENOBORDERS),
           "PASTE_FORMULA" => Ok(CutPasteRequestPasteTypeEnum::PASTEFORMULA),
           "PASTE_DATA_VALIDATION" => Ok(CutPasteRequestPasteTypeEnum::PASTEDATAVALIDATION),
           "PASTE_CONDITIONAL_FORMATTING" => Ok(CutPasteRequestPasteTypeEnum::PASTECONDITIONALFORMATTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CutPasteRequestPasteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataExecutionStatusErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The error code.
pub enum DataExecutionStatusErrorCodeEnum {
    

    /// Default value, do not use.
    ///
    /// "DATA_EXECUTION_ERROR_CODE_UNSPECIFIED"
    #[serde(rename="DATA_EXECUTION_ERROR_CODE_UNSPECIFIED")]
    DATAEXECUTIONERRORCODEUNSPECIFIED,
    

    /// The data execution timed out.
    ///
    /// "TIMED_OUT"
    #[serde(rename="TIMED_OUT")]
    TIMEDOUT,
    

    /// The data execution returns more rows than the limit.
    ///
    /// "TOO_MANY_ROWS"
    #[serde(rename="TOO_MANY_ROWS")]
    TOOMANYROWS,
    

    /// The data execution returns more columns than the limit.
    ///
    /// "TOO_MANY_COLUMNS"
    #[serde(rename="TOO_MANY_COLUMNS")]
    TOOMANYCOLUMNS,
    

    /// The data execution returns more cells than the limit.
    ///
    /// "TOO_MANY_CELLS"
    #[serde(rename="TOO_MANY_CELLS")]
    TOOMANYCELLS,
    

    /// Error is received from the backend data execution engine (e.g. BigQuery). Check error_message for details.
    ///
    /// "ENGINE"
    #[serde(rename="ENGINE")]
    ENGINE,
    

    /// One or some of the provided data source parameters are invalid.
    ///
    /// "PARAMETER_INVALID"
    #[serde(rename="PARAMETER_INVALID")]
    PARAMETERINVALID,
    

    /// The data execution returns an unsupported data type.
    ///
    /// "UNSUPPORTED_DATA_TYPE"
    #[serde(rename="UNSUPPORTED_DATA_TYPE")]
    UNSUPPORTEDDATATYPE,
    

    /// The data execution returns duplicate column names or aliases.
    ///
    /// "DUPLICATE_COLUMN_NAMES"
    #[serde(rename="DUPLICATE_COLUMN_NAMES")]
    DUPLICATECOLUMNNAMES,
    

    /// The data execution is interrupted. Please refresh later.
    ///
    /// "INTERRUPTED"
    #[serde(rename="INTERRUPTED")]
    INTERRUPTED,
    

    /// The data execution is currently in progress, can not be refreshed until it completes.
    ///
    /// "CONCURRENT_QUERY"
    #[serde(rename="CONCURRENT_QUERY")]
    CONCURRENTQUERY,
    

    /// Other errors.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    

    /// The data execution returns values that exceed the maximum characters allowed in a single cell.
    ///
    /// "TOO_MANY_CHARS_PER_CELL"
    #[serde(rename="TOO_MANY_CHARS_PER_CELL")]
    TOOMANYCHARSPERCELL,
    

    /// The database referenced by the data source is not found. */
    ///
    /// "DATA_NOT_FOUND"
    #[serde(rename="DATA_NOT_FOUND")]
    DATANOTFOUND,
    

    /// The user does not have access to the database referenced by the data source.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// The data execution returns columns with missing aliases.
    ///
    /// "MISSING_COLUMN_ALIAS"
    #[serde(rename="MISSING_COLUMN_ALIAS")]
    MISSINGCOLUMNALIAS,
    

    /// The data source object does not exist.
    ///
    /// "OBJECT_NOT_FOUND"
    #[serde(rename="OBJECT_NOT_FOUND")]
    OBJECTNOTFOUND,
    

    /// The data source object is currently in error state. To force refresh, set force in RefreshDataSourceRequest.
    ///
    /// "OBJECT_IN_ERROR_STATE"
    #[serde(rename="OBJECT_IN_ERROR_STATE")]
    OBJECTINERRORSTATE,
    

    /// The data source object specification is invalid.
    ///
    /// "OBJECT_SPEC_INVALID"
    #[serde(rename="OBJECT_SPEC_INVALID")]
    OBJECTSPECINVALID,
    

    /// The data execution has been cancelled.
    ///
    /// "DATA_EXECUTION_CANCELLED"
    #[serde(rename="DATA_EXECUTION_CANCELLED")]
    DATAEXECUTIONCANCELLED,
}

impl AsRef<str> for DataExecutionStatusErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataExecutionStatusErrorCodeEnum::DATAEXECUTIONERRORCODEUNSPECIFIED => "DATA_EXECUTION_ERROR_CODE_UNSPECIFIED",
            DataExecutionStatusErrorCodeEnum::TIMEDOUT => "TIMED_OUT",
            DataExecutionStatusErrorCodeEnum::TOOMANYROWS => "TOO_MANY_ROWS",
            DataExecutionStatusErrorCodeEnum::TOOMANYCOLUMNS => "TOO_MANY_COLUMNS",
            DataExecutionStatusErrorCodeEnum::TOOMANYCELLS => "TOO_MANY_CELLS",
            DataExecutionStatusErrorCodeEnum::ENGINE => "ENGINE",
            DataExecutionStatusErrorCodeEnum::PARAMETERINVALID => "PARAMETER_INVALID",
            DataExecutionStatusErrorCodeEnum::UNSUPPORTEDDATATYPE => "UNSUPPORTED_DATA_TYPE",
            DataExecutionStatusErrorCodeEnum::DUPLICATECOLUMNNAMES => "DUPLICATE_COLUMN_NAMES",
            DataExecutionStatusErrorCodeEnum::INTERRUPTED => "INTERRUPTED",
            DataExecutionStatusErrorCodeEnum::CONCURRENTQUERY => "CONCURRENT_QUERY",
            DataExecutionStatusErrorCodeEnum::OTHER => "OTHER",
            DataExecutionStatusErrorCodeEnum::TOOMANYCHARSPERCELL => "TOO_MANY_CHARS_PER_CELL",
            DataExecutionStatusErrorCodeEnum::DATANOTFOUND => "DATA_NOT_FOUND",
            DataExecutionStatusErrorCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            DataExecutionStatusErrorCodeEnum::MISSINGCOLUMNALIAS => "MISSING_COLUMN_ALIAS",
            DataExecutionStatusErrorCodeEnum::OBJECTNOTFOUND => "OBJECT_NOT_FOUND",
            DataExecutionStatusErrorCodeEnum::OBJECTINERRORSTATE => "OBJECT_IN_ERROR_STATE",
            DataExecutionStatusErrorCodeEnum::OBJECTSPECINVALID => "OBJECT_SPEC_INVALID",
            DataExecutionStatusErrorCodeEnum::DATAEXECUTIONCANCELLED => "DATA_EXECUTION_CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for DataExecutionStatusErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_EXECUTION_ERROR_CODE_UNSPECIFIED" => Ok(DataExecutionStatusErrorCodeEnum::DATAEXECUTIONERRORCODEUNSPECIFIED),
           "TIMED_OUT" => Ok(DataExecutionStatusErrorCodeEnum::TIMEDOUT),
           "TOO_MANY_ROWS" => Ok(DataExecutionStatusErrorCodeEnum::TOOMANYROWS),
           "TOO_MANY_COLUMNS" => Ok(DataExecutionStatusErrorCodeEnum::TOOMANYCOLUMNS),
           "TOO_MANY_CELLS" => Ok(DataExecutionStatusErrorCodeEnum::TOOMANYCELLS),
           "ENGINE" => Ok(DataExecutionStatusErrorCodeEnum::ENGINE),
           "PARAMETER_INVALID" => Ok(DataExecutionStatusErrorCodeEnum::PARAMETERINVALID),
           "UNSUPPORTED_DATA_TYPE" => Ok(DataExecutionStatusErrorCodeEnum::UNSUPPORTEDDATATYPE),
           "DUPLICATE_COLUMN_NAMES" => Ok(DataExecutionStatusErrorCodeEnum::DUPLICATECOLUMNNAMES),
           "INTERRUPTED" => Ok(DataExecutionStatusErrorCodeEnum::INTERRUPTED),
           "CONCURRENT_QUERY" => Ok(DataExecutionStatusErrorCodeEnum::CONCURRENTQUERY),
           "OTHER" => Ok(DataExecutionStatusErrorCodeEnum::OTHER),
           "TOO_MANY_CHARS_PER_CELL" => Ok(DataExecutionStatusErrorCodeEnum::TOOMANYCHARSPERCELL),
           "DATA_NOT_FOUND" => Ok(DataExecutionStatusErrorCodeEnum::DATANOTFOUND),
           "PERMISSION_DENIED" => Ok(DataExecutionStatusErrorCodeEnum::PERMISSIONDENIED),
           "MISSING_COLUMN_ALIAS" => Ok(DataExecutionStatusErrorCodeEnum::MISSINGCOLUMNALIAS),
           "OBJECT_NOT_FOUND" => Ok(DataExecutionStatusErrorCodeEnum::OBJECTNOTFOUND),
           "OBJECT_IN_ERROR_STATE" => Ok(DataExecutionStatusErrorCodeEnum::OBJECTINERRORSTATE),
           "OBJECT_SPEC_INVALID" => Ok(DataExecutionStatusErrorCodeEnum::OBJECTSPECINVALID),
           "DATA_EXECUTION_CANCELLED" => Ok(DataExecutionStatusErrorCodeEnum::DATAEXECUTIONCANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataExecutionStatusErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataExecutionStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the data execution.
pub enum DataExecutionStatusStateEnum {
    

    /// Default value, do not use.
    ///
    /// "DATA_EXECUTION_STATE_UNSPECIFIED"
    #[serde(rename="DATA_EXECUTION_STATE_UNSPECIFIED")]
    DATAEXECUTIONSTATEUNSPECIFIED,
    

    /// The data execution has not started.
    ///
    /// "NOT_STARTED"
    #[serde(rename="NOT_STARTED")]
    NOTSTARTED,
    

    /// The data execution has started and is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The data execution is currently being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The data execution has completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The data execution has completed with errors.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for DataExecutionStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataExecutionStatusStateEnum::DATAEXECUTIONSTATEUNSPECIFIED => "DATA_EXECUTION_STATE_UNSPECIFIED",
            DataExecutionStatusStateEnum::NOTSTARTED => "NOT_STARTED",
            DataExecutionStatusStateEnum::RUNNING => "RUNNING",
            DataExecutionStatusStateEnum::CANCELLING => "CANCELLING",
            DataExecutionStatusStateEnum::SUCCEEDED => "SUCCEEDED",
            DataExecutionStatusStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for DataExecutionStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_EXECUTION_STATE_UNSPECIFIED" => Ok(DataExecutionStatusStateEnum::DATAEXECUTIONSTATEUNSPECIFIED),
           "NOT_STARTED" => Ok(DataExecutionStatusStateEnum::NOTSTARTED),
           "RUNNING" => Ok(DataExecutionStatusStateEnum::RUNNING),
           "CANCELLING" => Ok(DataExecutionStatusStateEnum::CANCELLING),
           "SUCCEEDED" => Ok(DataExecutionStatusStateEnum::SUCCEEDED),
           "FAILED" => Ok(DataExecutionStatusStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataExecutionStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataFilterValueRangeMajorDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The major dimension of the values.
pub enum DataFilterValueRangeMajorDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for DataFilterValueRangeMajorDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataFilterValueRangeMajorDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            DataFilterValueRangeMajorDimensionEnum::ROWS => "ROWS",
            DataFilterValueRangeMajorDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for DataFilterValueRangeMajorDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(DataFilterValueRangeMajorDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(DataFilterValueRangeMajorDimensionEnum::ROWS),
           "COLUMNS" => Ok(DataFilterValueRangeMajorDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataFilterValueRangeMajorDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataLabelPlacementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The placement of the data label relative to the labeled data.
pub enum DataLabelPlacementEnum {
    

    /// The positioning is determined automatically by the renderer.
    ///
    /// "DATA_LABEL_PLACEMENT_UNSPECIFIED"
    #[serde(rename="DATA_LABEL_PLACEMENT_UNSPECIFIED")]
    DATALABELPLACEMENTUNSPECIFIED,
    

    /// Center within a bar or column, both horizontally and vertically.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// To the left of a data point.
    ///
    /// "LEFT"
    #[serde(rename="LEFT")]
    LEFT,
    

    /// To the right of a data point.
    ///
    /// "RIGHT"
    #[serde(rename="RIGHT")]
    RIGHT,
    

    /// Above a data point.
    ///
    /// "ABOVE"
    #[serde(rename="ABOVE")]
    ABOVE,
    

    /// Below a data point.
    ///
    /// "BELOW"
    #[serde(rename="BELOW")]
    BELOW,
    

    /// Inside a bar or column at the end (top if positive, bottom if negative).
    ///
    /// "INSIDE_END"
    #[serde(rename="INSIDE_END")]
    INSIDEEND,
    

    /// Inside a bar or column at the base.
    ///
    /// "INSIDE_BASE"
    #[serde(rename="INSIDE_BASE")]
    INSIDEBASE,
    

    /// Outside a bar or column at the end.
    ///
    /// "OUTSIDE_END"
    #[serde(rename="OUTSIDE_END")]
    OUTSIDEEND,
}

impl AsRef<str> for DataLabelPlacementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataLabelPlacementEnum::DATALABELPLACEMENTUNSPECIFIED => "DATA_LABEL_PLACEMENT_UNSPECIFIED",
            DataLabelPlacementEnum::CENTER => "CENTER",
            DataLabelPlacementEnum::LEFT => "LEFT",
            DataLabelPlacementEnum::RIGHT => "RIGHT",
            DataLabelPlacementEnum::ABOVE => "ABOVE",
            DataLabelPlacementEnum::BELOW => "BELOW",
            DataLabelPlacementEnum::INSIDEEND => "INSIDE_END",
            DataLabelPlacementEnum::INSIDEBASE => "INSIDE_BASE",
            DataLabelPlacementEnum::OUTSIDEEND => "OUTSIDE_END",
        }
    }
}

impl std::convert::TryFrom< &str> for DataLabelPlacementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_LABEL_PLACEMENT_UNSPECIFIED" => Ok(DataLabelPlacementEnum::DATALABELPLACEMENTUNSPECIFIED),
           "CENTER" => Ok(DataLabelPlacementEnum::CENTER),
           "LEFT" => Ok(DataLabelPlacementEnum::LEFT),
           "RIGHT" => Ok(DataLabelPlacementEnum::RIGHT),
           "ABOVE" => Ok(DataLabelPlacementEnum::ABOVE),
           "BELOW" => Ok(DataLabelPlacementEnum::BELOW),
           "INSIDE_END" => Ok(DataLabelPlacementEnum::INSIDEEND),
           "INSIDE_BASE" => Ok(DataLabelPlacementEnum::INSIDEBASE),
           "OUTSIDE_END" => Ok(DataLabelPlacementEnum::OUTSIDEEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataLabelPlacementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataLabelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the data label.
pub enum DataLabelTypeEnum {
    

    /// The data label type is not specified and will be interpreted depending on the context of the data label within the chart.
    ///
    /// "DATA_LABEL_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_LABEL_TYPE_UNSPECIFIED")]
    DATALABELTYPEUNSPECIFIED,
    

    /// The data label is not displayed.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// The data label is displayed using values from the series data.
    ///
    /// "DATA"
    #[serde(rename="DATA")]
    DATA,
    

    /// The data label is displayed using values from a custom data source indicated by customLabelData.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for DataLabelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataLabelTypeEnum::DATALABELTYPEUNSPECIFIED => "DATA_LABEL_TYPE_UNSPECIFIED",
            DataLabelTypeEnum::NONE => "NONE",
            DataLabelTypeEnum::DATA => "DATA",
            DataLabelTypeEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for DataLabelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_LABEL_TYPE_UNSPECIFIED" => Ok(DataLabelTypeEnum::DATALABELTYPEUNSPECIFIED),
           "NONE" => Ok(DataLabelTypeEnum::NONE),
           "DATA" => Ok(DataLabelTypeEnum::DATA),
           "CUSTOM" => Ok(DataLabelTypeEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataLabelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceRefreshScheduleRefreshScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope of the refresh. Must be ALL_DATA_SOURCES.
pub enum DataSourceRefreshScheduleRefreshScopeEnum {
    

    /// Default value, do not use.
    ///
    /// "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED"
    #[serde(rename="DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED")]
    DATASOURCEREFRESHSCOPEUNSPECIFIED,
    

    /// Refreshes all data sources and their associated data source objects in the spreadsheet.
    ///
    /// "ALL_DATA_SOURCES"
    #[serde(rename="ALL_DATA_SOURCES")]
    ALLDATASOURCES,
}

impl AsRef<str> for DataSourceRefreshScheduleRefreshScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceRefreshScheduleRefreshScopeEnum::DATASOURCEREFRESHSCOPEUNSPECIFIED => "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED",
            DataSourceRefreshScheduleRefreshScopeEnum::ALLDATASOURCES => "ALL_DATA_SOURCES",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceRefreshScheduleRefreshScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED" => Ok(DataSourceRefreshScheduleRefreshScopeEnum::DATASOURCEREFRESHSCOPEUNSPECIFIED),
           "ALL_DATA_SOURCES" => Ok(DataSourceRefreshScheduleRefreshScopeEnum::ALLDATASOURCES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceRefreshScheduleRefreshScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceRefreshWeeklyScheduleDaysOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Days of the week to refresh. At least one day must be specified.
pub enum DataSourceRefreshWeeklyScheduleDaysOfWeekEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for DataSourceRefreshWeeklyScheduleDaysOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::MONDAY => "MONDAY",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::TUESDAY => "TUESDAY",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::THURSDAY => "THURSDAY",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::FRIDAY => "FRIDAY",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::SATURDAY => "SATURDAY",
            DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceRefreshWeeklyScheduleDaysOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(DataSourceRefreshWeeklyScheduleDaysOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceRefreshWeeklyScheduleDaysOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataSourceTableColumnSelectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type to select columns for the data source table. Defaults to SELECTED.
pub enum DataSourceTableColumnSelectionTypeEnum {
    

    /// The default column selection type, do not use.
    ///
    /// "DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED")]
    DATASOURCETABLECOLUMNSELECTIONTYPEUNSPECIFIED,
    

    /// Select columns specified by columns field.
    ///
    /// "SELECTED"
    #[serde(rename="SELECTED")]
    SELECTED,
    

    /// Sync all current and future columns in the data source. If set, the data source table fetches all the columns in the data source at the time of refresh.
    ///
    /// "SYNC_ALL"
    #[serde(rename="SYNC_ALL")]
    SYNCALL,
}

impl AsRef<str> for DataSourceTableColumnSelectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataSourceTableColumnSelectionTypeEnum::DATASOURCETABLECOLUMNSELECTIONTYPEUNSPECIFIED => "DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED",
            DataSourceTableColumnSelectionTypeEnum::SELECTED => "SELECTED",
            DataSourceTableColumnSelectionTypeEnum::SYNCALL => "SYNC_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for DataSourceTableColumnSelectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSPECIFIED" => Ok(DataSourceTableColumnSelectionTypeEnum::DATASOURCETABLECOLUMNSELECTIONTYPEUNSPECIFIED),
           "SELECTED" => Ok(DataSourceTableColumnSelectionTypeEnum::SELECTED),
           "SYNC_ALL" => Ok(DataSourceTableColumnSelectionTypeEnum::SYNCALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataSourceTableColumnSelectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DateTimeRuleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of date-time grouping to apply.
pub enum DateTimeRuleTypeEnum {
    

    /// The default type, do not use.
    ///
    /// "DATE_TIME_RULE_TYPE_UNSPECIFIED"
    #[serde(rename="DATE_TIME_RULE_TYPE_UNSPECIFIED")]
    DATETIMERULETYPEUNSPECIFIED,
    

    /// Group dates by second, from 0 to 59.
    ///
    /// "SECOND"
    #[serde(rename="SECOND")]
    SECOND,
    

    /// Group dates by minute, from 0 to 59.
    ///
    /// "MINUTE"
    #[serde(rename="MINUTE")]
    MINUTE,
    

    /// Group dates by hour using a 24-hour system, from 0 to 23.
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
    

    /// Group dates by hour and minute using a 24-hour system, for example 19:45.
    ///
    /// "HOUR_MINUTE"
    #[serde(rename="HOUR_MINUTE")]
    HOURMINUTE,
    

    /// Group dates by hour and minute using a 12-hour system, for example 7:45 PM. The AM/PM designation is translated based on the spreadsheet locale.
    ///
    /// "HOUR_MINUTE_AMPM"
    #[serde(rename="HOUR_MINUTE_AMPM")]
    HOURMINUTEAMPM,
    

    /// Group dates by day of week, for example Sunday. The days of the week will be translated based on the spreadsheet locale.
    ///
    /// "DAY_OF_WEEK"
    #[serde(rename="DAY_OF_WEEK")]
    DAYOFWEEK,
    

    /// Group dates by day of year, from 1 to 366. Note that dates after Feb. 29 fall in different buckets in leap years than in non-leap years.
    ///
    /// "DAY_OF_YEAR"
    #[serde(rename="DAY_OF_YEAR")]
    DAYOFYEAR,
    

    /// Group dates by day of month, from 1 to 31.
    ///
    /// "DAY_OF_MONTH"
    #[serde(rename="DAY_OF_MONTH")]
    DAYOFMONTH,
    

    /// Group dates by day and month, for example 22-Nov. The month is translated based on the spreadsheet locale.
    ///
    /// "DAY_MONTH"
    #[serde(rename="DAY_MONTH")]
    DAYMONTH,
    

    /// Group dates by month, for example Nov. The month is translated based on the spreadsheet locale.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Group dates by quarter, for example Q1 (which represents Jan-Mar).
    ///
    /// "QUARTER"
    #[serde(rename="QUARTER")]
    QUARTER,
    

    /// Group dates by year, for example 2008.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
    

    /// Group dates by year and month, for example 2008-Nov. The month is translated based on the spreadsheet locale.
    ///
    /// "YEAR_MONTH"
    #[serde(rename="YEAR_MONTH")]
    YEARMONTH,
    

    /// Group dates by year and quarter, for example 2008 Q4.
    ///
    /// "YEAR_QUARTER"
    #[serde(rename="YEAR_QUARTER")]
    YEARQUARTER,
    

    /// Group dates by year, month, and day, for example 2008-11-22.
    ///
    /// "YEAR_MONTH_DAY"
    #[serde(rename="YEAR_MONTH_DAY")]
    YEARMONTHDAY,
}

impl AsRef<str> for DateTimeRuleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DateTimeRuleTypeEnum::DATETIMERULETYPEUNSPECIFIED => "DATE_TIME_RULE_TYPE_UNSPECIFIED",
            DateTimeRuleTypeEnum::SECOND => "SECOND",
            DateTimeRuleTypeEnum::MINUTE => "MINUTE",
            DateTimeRuleTypeEnum::HOUR => "HOUR",
            DateTimeRuleTypeEnum::HOURMINUTE => "HOUR_MINUTE",
            DateTimeRuleTypeEnum::HOURMINUTEAMPM => "HOUR_MINUTE_AMPM",
            DateTimeRuleTypeEnum::DAYOFWEEK => "DAY_OF_WEEK",
            DateTimeRuleTypeEnum::DAYOFYEAR => "DAY_OF_YEAR",
            DateTimeRuleTypeEnum::DAYOFMONTH => "DAY_OF_MONTH",
            DateTimeRuleTypeEnum::DAYMONTH => "DAY_MONTH",
            DateTimeRuleTypeEnum::MONTH => "MONTH",
            DateTimeRuleTypeEnum::QUARTER => "QUARTER",
            DateTimeRuleTypeEnum::YEAR => "YEAR",
            DateTimeRuleTypeEnum::YEARMONTH => "YEAR_MONTH",
            DateTimeRuleTypeEnum::YEARQUARTER => "YEAR_QUARTER",
            DateTimeRuleTypeEnum::YEARMONTHDAY => "YEAR_MONTH_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for DateTimeRuleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATE_TIME_RULE_TYPE_UNSPECIFIED" => Ok(DateTimeRuleTypeEnum::DATETIMERULETYPEUNSPECIFIED),
           "SECOND" => Ok(DateTimeRuleTypeEnum::SECOND),
           "MINUTE" => Ok(DateTimeRuleTypeEnum::MINUTE),
           "HOUR" => Ok(DateTimeRuleTypeEnum::HOUR),
           "HOUR_MINUTE" => Ok(DateTimeRuleTypeEnum::HOURMINUTE),
           "HOUR_MINUTE_AMPM" => Ok(DateTimeRuleTypeEnum::HOURMINUTEAMPM),
           "DAY_OF_WEEK" => Ok(DateTimeRuleTypeEnum::DAYOFWEEK),
           "DAY_OF_YEAR" => Ok(DateTimeRuleTypeEnum::DAYOFYEAR),
           "DAY_OF_MONTH" => Ok(DateTimeRuleTypeEnum::DAYOFMONTH),
           "DAY_MONTH" => Ok(DateTimeRuleTypeEnum::DAYMONTH),
           "MONTH" => Ok(DateTimeRuleTypeEnum::MONTH),
           "QUARTER" => Ok(DateTimeRuleTypeEnum::QUARTER),
           "YEAR" => Ok(DateTimeRuleTypeEnum::YEAR),
           "YEAR_MONTH" => Ok(DateTimeRuleTypeEnum::YEARMONTH),
           "YEAR_QUARTER" => Ok(DateTimeRuleTypeEnum::YEARQUARTER),
           "YEAR_MONTH_DAY" => Ok(DateTimeRuleTypeEnum::YEARMONTHDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DateTimeRuleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeleteRangeRequestShiftDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dimension from which deleted cells will be replaced with. If ROWS, existing cells will be shifted upward to replace the deleted cells. If COLUMNS, existing cells will be shifted left to replace the deleted cells.
pub enum DeleteRangeRequestShiftDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for DeleteRangeRequestShiftDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeleteRangeRequestShiftDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            DeleteRangeRequestShiftDimensionEnum::ROWS => "ROWS",
            DeleteRangeRequestShiftDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for DeleteRangeRequestShiftDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(DeleteRangeRequestShiftDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(DeleteRangeRequestShiftDimensionEnum::ROWS),
           "COLUMNS" => Ok(DeleteRangeRequestShiftDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeleteRangeRequestShiftDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeveloperMetadataVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The metadata visibility. Developer metadata must always have a visibility specified.
pub enum DeveloperMetadataVisibilityEnum {
    

    /// Default value.
    ///
    /// "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED"
    #[serde(rename="DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED")]
    DEVELOPERMETADATAVISIBILITYUNSPECIFIED,
    

    /// Document-visible metadata is accessible from any developer project with access to the document.
    ///
    /// "DOCUMENT"
    #[serde(rename="DOCUMENT")]
    DOCUMENT,
    

    /// Project-visible metadata is only visible to and accessible by the developer project that created the metadata.
    ///
    /// "PROJECT"
    #[serde(rename="PROJECT")]
    PROJECT,
}

impl AsRef<str> for DeveloperMetadataVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeveloperMetadataVisibilityEnum::DEVELOPERMETADATAVISIBILITYUNSPECIFIED => "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED",
            DeveloperMetadataVisibilityEnum::DOCUMENT => "DOCUMENT",
            DeveloperMetadataVisibilityEnum::PROJECT => "PROJECT",
        }
    }
}

impl std::convert::TryFrom< &str> for DeveloperMetadataVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED" => Ok(DeveloperMetadataVisibilityEnum::DEVELOPERMETADATAVISIBILITYUNSPECIFIED),
           "DOCUMENT" => Ok(DeveloperMetadataVisibilityEnum::DOCUMENT),
           "PROJECT" => Ok(DeveloperMetadataVisibilityEnum::PROJECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeveloperMetadataVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeveloperMetadataLocationLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of location this object represents. This field is read-only.
pub enum DeveloperMetadataLocationLocationTypeEnum {
    

    /// Default value.
    ///
    /// "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED")]
    DEVELOPERMETADATALOCATIONTYPEUNSPECIFIED,
    

    /// Developer metadata associated on an entire row dimension.
    ///
    /// "ROW"
    #[serde(rename="ROW")]
    ROW,
    

    /// Developer metadata associated on an entire column dimension.
    ///
    /// "COLUMN"
    #[serde(rename="COLUMN")]
    COLUMN,
    

    /// Developer metadata associated on an entire sheet.
    ///
    /// "SHEET"
    #[serde(rename="SHEET")]
    SHEET,
    

    /// Developer metadata associated on the entire spreadsheet.
    ///
    /// "SPREADSHEET"
    #[serde(rename="SPREADSHEET")]
    SPREADSHEET,
}

impl AsRef<str> for DeveloperMetadataLocationLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeveloperMetadataLocationLocationTypeEnum::DEVELOPERMETADATALOCATIONTYPEUNSPECIFIED => "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED",
            DeveloperMetadataLocationLocationTypeEnum::ROW => "ROW",
            DeveloperMetadataLocationLocationTypeEnum::COLUMN => "COLUMN",
            DeveloperMetadataLocationLocationTypeEnum::SHEET => "SHEET",
            DeveloperMetadataLocationLocationTypeEnum::SPREADSHEET => "SPREADSHEET",
        }
    }
}

impl std::convert::TryFrom< &str> for DeveloperMetadataLocationLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED" => Ok(DeveloperMetadataLocationLocationTypeEnum::DEVELOPERMETADATALOCATIONTYPEUNSPECIFIED),
           "ROW" => Ok(DeveloperMetadataLocationLocationTypeEnum::ROW),
           "COLUMN" => Ok(DeveloperMetadataLocationLocationTypeEnum::COLUMN),
           "SHEET" => Ok(DeveloperMetadataLocationLocationTypeEnum::SHEET),
           "SPREADSHEET" => Ok(DeveloperMetadataLocationLocationTypeEnum::SPREADSHEET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeveloperMetadataLocationLocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeveloperMetadataLookupLocationMatchingStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how this lookup matches the location. If this field is specified as EXACT, only developer metadata associated on the exact location specified is matched. If this field is specified to INTERSECTING, developer metadata associated on intersecting locations is also matched. If left unspecified, this field assumes a default value of INTERSECTING. If this field is specified, a metadataLocation must also be specified.
pub enum DeveloperMetadataLookupLocationMatchingStrategyEnum {
    

    /// Default value. This value must not be used.
    ///
    /// "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED"
    #[serde(rename="DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED")]
    DEVELOPERMETADATALOCATIONMATCHINGSTRATEGYUNSPECIFIED,
    

    /// Indicates that a specified location should be matched exactly. For example, if row three were specified as a location this matching strategy would only match developer metadata also associated on row three. Metadata associated on other locations would not be considered.
    ///
    /// "EXACT_LOCATION"
    #[serde(rename="EXACT_LOCATION")]
    EXACTLOCATION,
    

    /// Indicates that a specified location should match that exact location as well as any intersecting locations. For example, if row three were specified as a location this matching strategy would match developer metadata associated on row three as well as metadata associated on locations that intersect row three. If, for instance, there was developer metadata associated on column B, this matching strategy would also match that location because column B intersects row three.
    ///
    /// "INTERSECTING_LOCATION"
    #[serde(rename="INTERSECTING_LOCATION")]
    INTERSECTINGLOCATION,
}

impl AsRef<str> for DeveloperMetadataLookupLocationMatchingStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeveloperMetadataLookupLocationMatchingStrategyEnum::DEVELOPERMETADATALOCATIONMATCHINGSTRATEGYUNSPECIFIED => "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED",
            DeveloperMetadataLookupLocationMatchingStrategyEnum::EXACTLOCATION => "EXACT_LOCATION",
            DeveloperMetadataLookupLocationMatchingStrategyEnum::INTERSECTINGLOCATION => "INTERSECTING_LOCATION",
        }
    }
}

impl std::convert::TryFrom< &str> for DeveloperMetadataLookupLocationMatchingStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_METADATA_LOCATION_MATCHING_STRATEGY_UNSPECIFIED" => Ok(DeveloperMetadataLookupLocationMatchingStrategyEnum::DEVELOPERMETADATALOCATIONMATCHINGSTRATEGYUNSPECIFIED),
           "EXACT_LOCATION" => Ok(DeveloperMetadataLookupLocationMatchingStrategyEnum::EXACTLOCATION),
           "INTERSECTING_LOCATION" => Ok(DeveloperMetadataLookupLocationMatchingStrategyEnum::INTERSECTINGLOCATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeveloperMetadataLookupLocationMatchingStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeveloperMetadataLookupLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Limits the selected developer metadata to those entries which are associated with locations of the specified type. For example, when this field is specified as ROW this lookup only considers developer metadata associated on rows. If the field is left unspecified, all location types are considered. This field cannot be specified as SPREADSHEET when the locationMatchingStrategy is specified as INTERSECTING or when the metadataLocation is specified as a non-spreadsheet location: spreadsheet metadata cannot intersect any other developer metadata location. This field also must be left unspecified when the locationMatchingStrategy is specified as EXACT.
pub enum DeveloperMetadataLookupLocationTypeEnum {
    

    /// Default value.
    ///
    /// "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED")]
    DEVELOPERMETADATALOCATIONTYPEUNSPECIFIED,
    

    /// Developer metadata associated on an entire row dimension.
    ///
    /// "ROW"
    #[serde(rename="ROW")]
    ROW,
    

    /// Developer metadata associated on an entire column dimension.
    ///
    /// "COLUMN"
    #[serde(rename="COLUMN")]
    COLUMN,
    

    /// Developer metadata associated on an entire sheet.
    ///
    /// "SHEET"
    #[serde(rename="SHEET")]
    SHEET,
    

    /// Developer metadata associated on the entire spreadsheet.
    ///
    /// "SPREADSHEET"
    #[serde(rename="SPREADSHEET")]
    SPREADSHEET,
}

impl AsRef<str> for DeveloperMetadataLookupLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeveloperMetadataLookupLocationTypeEnum::DEVELOPERMETADATALOCATIONTYPEUNSPECIFIED => "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED",
            DeveloperMetadataLookupLocationTypeEnum::ROW => "ROW",
            DeveloperMetadataLookupLocationTypeEnum::COLUMN => "COLUMN",
            DeveloperMetadataLookupLocationTypeEnum::SHEET => "SHEET",
            DeveloperMetadataLookupLocationTypeEnum::SPREADSHEET => "SPREADSHEET",
        }
    }
}

impl std::convert::TryFrom< &str> for DeveloperMetadataLookupLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_METADATA_LOCATION_TYPE_UNSPECIFIED" => Ok(DeveloperMetadataLookupLocationTypeEnum::DEVELOPERMETADATALOCATIONTYPEUNSPECIFIED),
           "ROW" => Ok(DeveloperMetadataLookupLocationTypeEnum::ROW),
           "COLUMN" => Ok(DeveloperMetadataLookupLocationTypeEnum::COLUMN),
           "SHEET" => Ok(DeveloperMetadataLookupLocationTypeEnum::SHEET),
           "SPREADSHEET" => Ok(DeveloperMetadataLookupLocationTypeEnum::SPREADSHEET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeveloperMetadataLookupLocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeveloperMetadataLookupVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Limits the selected developer metadata to that which has a matching DeveloperMetadata.visibility. If left unspecified, all developer metadata visibile to the requesting project is considered.
pub enum DeveloperMetadataLookupVisibilityEnum {
    

    /// Default value.
    ///
    /// "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED"
    #[serde(rename="DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED")]
    DEVELOPERMETADATAVISIBILITYUNSPECIFIED,
    

    /// Document-visible metadata is accessible from any developer project with access to the document.
    ///
    /// "DOCUMENT"
    #[serde(rename="DOCUMENT")]
    DOCUMENT,
    

    /// Project-visible metadata is only visible to and accessible by the developer project that created the metadata.
    ///
    /// "PROJECT"
    #[serde(rename="PROJECT")]
    PROJECT,
}

impl AsRef<str> for DeveloperMetadataLookupVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeveloperMetadataLookupVisibilityEnum::DEVELOPERMETADATAVISIBILITYUNSPECIFIED => "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED",
            DeveloperMetadataLookupVisibilityEnum::DOCUMENT => "DOCUMENT",
            DeveloperMetadataLookupVisibilityEnum::PROJECT => "PROJECT",
        }
    }
}

impl std::convert::TryFrom< &str> for DeveloperMetadataLookupVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_METADATA_VISIBILITY_UNSPECIFIED" => Ok(DeveloperMetadataLookupVisibilityEnum::DEVELOPERMETADATAVISIBILITYUNSPECIFIED),
           "DOCUMENT" => Ok(DeveloperMetadataLookupVisibilityEnum::DOCUMENT),
           "PROJECT" => Ok(DeveloperMetadataLookupVisibilityEnum::PROJECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeveloperMetadataLookupVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionRangeDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dimension of the span.
pub enum DimensionRangeDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for DimensionRangeDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionRangeDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            DimensionRangeDimensionEnum::ROWS => "ROWS",
            DimensionRangeDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionRangeDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(DimensionRangeDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(DimensionRangeDimensionEnum::ROWS),
           "COLUMNS" => Ok(DimensionRangeDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionRangeDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ErrorValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of error.
pub enum ErrorValueTypeEnum {
    

    /// The default error type, do not use this.
    ///
    /// "ERROR_TYPE_UNSPECIFIED"
    #[serde(rename="ERROR_TYPE_UNSPECIFIED")]
    ERRORTYPEUNSPECIFIED,
    

    /// Corresponds to the `#ERROR!` error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Corresponds to the `#NULL!` error.
    ///
    /// "NULL_VALUE"
    #[serde(rename="NULL_VALUE")]
    NULLVALUE,
    

    /// Corresponds to the `#DIV/0` error.
    ///
    /// "DIVIDE_BY_ZERO"
    #[serde(rename="DIVIDE_BY_ZERO")]
    DIVIDEBYZERO,
    

    /// Corresponds to the `#VALUE!` error.
    ///
    /// "VALUE"
    #[serde(rename="VALUE")]
    VALUE,
    

    /// Corresponds to the `#REF!` error.
    ///
    /// "REF"
    #[serde(rename="REF")]
    REF,
    

    /// Corresponds to the `#NAME?` error.
    ///
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
    

    /// Corresponds to the `#NUM!` error.
    ///
    /// "NUM"
    #[serde(rename="NUM")]
    NUM,
    

    /// Corresponds to the `#N/A` error.
    ///
    /// "N_A"
    #[serde(rename="N_A")]
    NA,
    

    /// Corresponds to the `Loading...` state.
    ///
    /// "LOADING"
    #[serde(rename="LOADING")]
    LOADING,
}

impl AsRef<str> for ErrorValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ErrorValueTypeEnum::ERRORTYPEUNSPECIFIED => "ERROR_TYPE_UNSPECIFIED",
            ErrorValueTypeEnum::ERROR => "ERROR",
            ErrorValueTypeEnum::NULLVALUE => "NULL_VALUE",
            ErrorValueTypeEnum::DIVIDEBYZERO => "DIVIDE_BY_ZERO",
            ErrorValueTypeEnum::VALUE => "VALUE",
            ErrorValueTypeEnum::REF => "REF",
            ErrorValueTypeEnum::NAME => "NAME",
            ErrorValueTypeEnum::NUM => "NUM",
            ErrorValueTypeEnum::NA => "N_A",
            ErrorValueTypeEnum::LOADING => "LOADING",
        }
    }
}

impl std::convert::TryFrom< &str> for ErrorValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_TYPE_UNSPECIFIED" => Ok(ErrorValueTypeEnum::ERRORTYPEUNSPECIFIED),
           "ERROR" => Ok(ErrorValueTypeEnum::ERROR),
           "NULL_VALUE" => Ok(ErrorValueTypeEnum::NULLVALUE),
           "DIVIDE_BY_ZERO" => Ok(ErrorValueTypeEnum::DIVIDEBYZERO),
           "VALUE" => Ok(ErrorValueTypeEnum::VALUE),
           "REF" => Ok(ErrorValueTypeEnum::REF),
           "NAME" => Ok(ErrorValueTypeEnum::NAME),
           "NUM" => Ok(ErrorValueTypeEnum::NUM),
           "N_A" => Ok(ErrorValueTypeEnum::NA),
           "LOADING" => Ok(ErrorValueTypeEnum::LOADING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ErrorValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HistogramChartSpecLegendPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The position of the chart legend.
pub enum HistogramChartSpecLegendPositionEnum {
    

    /// Default value, do not use.
    ///
    /// "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED"
    #[serde(rename="HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED")]
    HISTOGRAMCHARTLEGENDPOSITIONUNSPECIFIED,
    

    /// The legend is rendered on the bottom of the chart.
    ///
    /// "BOTTOM_LEGEND"
    #[serde(rename="BOTTOM_LEGEND")]
    BOTTOMLEGEND,
    

    /// The legend is rendered on the left of the chart.
    ///
    /// "LEFT_LEGEND"
    #[serde(rename="LEFT_LEGEND")]
    LEFTLEGEND,
    

    /// The legend is rendered on the right of the chart.
    ///
    /// "RIGHT_LEGEND"
    #[serde(rename="RIGHT_LEGEND")]
    RIGHTLEGEND,
    

    /// The legend is rendered on the top of the chart.
    ///
    /// "TOP_LEGEND"
    #[serde(rename="TOP_LEGEND")]
    TOPLEGEND,
    

    /// No legend is rendered.
    ///
    /// "NO_LEGEND"
    #[serde(rename="NO_LEGEND")]
    NOLEGEND,
    

    /// The legend is rendered inside the chart area.
    ///
    /// "INSIDE_LEGEND"
    #[serde(rename="INSIDE_LEGEND")]
    INSIDELEGEND,
}

impl AsRef<str> for HistogramChartSpecLegendPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HistogramChartSpecLegendPositionEnum::HISTOGRAMCHARTLEGENDPOSITIONUNSPECIFIED => "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED",
            HistogramChartSpecLegendPositionEnum::BOTTOMLEGEND => "BOTTOM_LEGEND",
            HistogramChartSpecLegendPositionEnum::LEFTLEGEND => "LEFT_LEGEND",
            HistogramChartSpecLegendPositionEnum::RIGHTLEGEND => "RIGHT_LEGEND",
            HistogramChartSpecLegendPositionEnum::TOPLEGEND => "TOP_LEGEND",
            HistogramChartSpecLegendPositionEnum::NOLEGEND => "NO_LEGEND",
            HistogramChartSpecLegendPositionEnum::INSIDELEGEND => "INSIDE_LEGEND",
        }
    }
}

impl std::convert::TryFrom< &str> for HistogramChartSpecLegendPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HISTOGRAM_CHART_LEGEND_POSITION_UNSPECIFIED" => Ok(HistogramChartSpecLegendPositionEnum::HISTOGRAMCHARTLEGENDPOSITIONUNSPECIFIED),
           "BOTTOM_LEGEND" => Ok(HistogramChartSpecLegendPositionEnum::BOTTOMLEGEND),
           "LEFT_LEGEND" => Ok(HistogramChartSpecLegendPositionEnum::LEFTLEGEND),
           "RIGHT_LEGEND" => Ok(HistogramChartSpecLegendPositionEnum::RIGHTLEGEND),
           "TOP_LEGEND" => Ok(HistogramChartSpecLegendPositionEnum::TOPLEGEND),
           "NO_LEGEND" => Ok(HistogramChartSpecLegendPositionEnum::NOLEGEND),
           "INSIDE_LEGEND" => Ok(HistogramChartSpecLegendPositionEnum::INSIDELEGEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HistogramChartSpecLegendPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InsertRangeRequestShiftDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dimension which will be shifted when inserting cells. If ROWS, existing cells will be shifted down. If COLUMNS, existing cells will be shifted right.
pub enum InsertRangeRequestShiftDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for InsertRangeRequestShiftDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InsertRangeRequestShiftDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            InsertRangeRequestShiftDimensionEnum::ROWS => "ROWS",
            InsertRangeRequestShiftDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for InsertRangeRequestShiftDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(InsertRangeRequestShiftDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(InsertRangeRequestShiftDimensionEnum::ROWS),
           "COLUMNS" => Ok(InsertRangeRequestShiftDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InsertRangeRequestShiftDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InterpolationPointTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the value should be interpreted.
pub enum InterpolationPointTypeEnum {
    

    /// The default value, do not use.
    ///
    /// "INTERPOLATION_POINT_TYPE_UNSPECIFIED"
    #[serde(rename="INTERPOLATION_POINT_TYPE_UNSPECIFIED")]
    INTERPOLATIONPOINTTYPEUNSPECIFIED,
    

    /// The interpolation point uses the minimum value in the cells over the range of the conditional format.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// The interpolation point uses the maximum value in the cells over the range of the conditional format.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
    

    /// The interpolation point uses exactly the value in InterpolationPoint.value.
    ///
    /// "NUMBER"
    #[serde(rename="NUMBER")]
    NUMBER,
    

    /// The interpolation point is the given percentage over all the cells in the range of the conditional format. This is equivalent to `NUMBER` if the value was: `=(MAX(FLATTEN(range)) * (value / 100)) + (MIN(FLATTEN(range)) * (1 - (value / 100)))` (where errors in the range are ignored when flattening).
    ///
    /// "PERCENT"
    #[serde(rename="PERCENT")]
    PERCENT,
    

    /// The interpolation point is the given percentile over all the cells in the range of the conditional format. This is equivalent to `NUMBER` if the value was: `=PERCENTILE(FLATTEN(range), value / 100)` (where errors in the range are ignored when flattening).
    ///
    /// "PERCENTILE"
    #[serde(rename="PERCENTILE")]
    PERCENTILE,
}

impl AsRef<str> for InterpolationPointTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InterpolationPointTypeEnum::INTERPOLATIONPOINTTYPEUNSPECIFIED => "INTERPOLATION_POINT_TYPE_UNSPECIFIED",
            InterpolationPointTypeEnum::MIN => "MIN",
            InterpolationPointTypeEnum::MAX => "MAX",
            InterpolationPointTypeEnum::NUMBER => "NUMBER",
            InterpolationPointTypeEnum::PERCENT => "PERCENT",
            InterpolationPointTypeEnum::PERCENTILE => "PERCENTILE",
        }
    }
}

impl std::convert::TryFrom< &str> for InterpolationPointTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTERPOLATION_POINT_TYPE_UNSPECIFIED" => Ok(InterpolationPointTypeEnum::INTERPOLATIONPOINTTYPEUNSPECIFIED),
           "MIN" => Ok(InterpolationPointTypeEnum::MIN),
           "MAX" => Ok(InterpolationPointTypeEnum::MAX),
           "NUMBER" => Ok(InterpolationPointTypeEnum::NUMBER),
           "PERCENT" => Ok(InterpolationPointTypeEnum::PERCENT),
           "PERCENTILE" => Ok(InterpolationPointTypeEnum::PERCENTILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InterpolationPointTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LineStyleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dash type of the line.
pub enum LineStyleTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "LINE_DASH_TYPE_UNSPECIFIED"
    #[serde(rename="LINE_DASH_TYPE_UNSPECIFIED")]
    LINEDASHTYPEUNSPECIFIED,
    

    /// No dash type, which is equivalent to a non-visible line.
    ///
    /// "INVISIBLE"
    #[serde(rename="INVISIBLE")]
    INVISIBLE,
    

    /// A custom dash for a line. Modifying the exact custom dash style is currently unsupported.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
    

    /// A solid line.
    ///
    /// "SOLID"
    #[serde(rename="SOLID")]
    SOLID,
    

    /// A dotted line.
    ///
    /// "DOTTED"
    #[serde(rename="DOTTED")]
    DOTTED,
    

    /// A dashed line where the dashes have "medium" length.
    ///
    /// "MEDIUM_DASHED"
    #[serde(rename="MEDIUM_DASHED")]
    MEDIUMDASHED,
    

    /// A line that alternates between a "medium" dash and a dot.
    ///
    /// "MEDIUM_DASHED_DOTTED"
    #[serde(rename="MEDIUM_DASHED_DOTTED")]
    MEDIUMDASHEDDOTTED,
    

    /// A dashed line where the dashes have "long" length.
    ///
    /// "LONG_DASHED"
    #[serde(rename="LONG_DASHED")]
    LONGDASHED,
    

    /// A line that alternates between a "long" dash and a dot.
    ///
    /// "LONG_DASHED_DOTTED"
    #[serde(rename="LONG_DASHED_DOTTED")]
    LONGDASHEDDOTTED,
}

impl AsRef<str> for LineStyleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LineStyleTypeEnum::LINEDASHTYPEUNSPECIFIED => "LINE_DASH_TYPE_UNSPECIFIED",
            LineStyleTypeEnum::INVISIBLE => "INVISIBLE",
            LineStyleTypeEnum::CUSTOM => "CUSTOM",
            LineStyleTypeEnum::SOLID => "SOLID",
            LineStyleTypeEnum::DOTTED => "DOTTED",
            LineStyleTypeEnum::MEDIUMDASHED => "MEDIUM_DASHED",
            LineStyleTypeEnum::MEDIUMDASHEDDOTTED => "MEDIUM_DASHED_DOTTED",
            LineStyleTypeEnum::LONGDASHED => "LONG_DASHED",
            LineStyleTypeEnum::LONGDASHEDDOTTED => "LONG_DASHED_DOTTED",
        }
    }
}

impl std::convert::TryFrom< &str> for LineStyleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_DASH_TYPE_UNSPECIFIED" => Ok(LineStyleTypeEnum::LINEDASHTYPEUNSPECIFIED),
           "INVISIBLE" => Ok(LineStyleTypeEnum::INVISIBLE),
           "CUSTOM" => Ok(LineStyleTypeEnum::CUSTOM),
           "SOLID" => Ok(LineStyleTypeEnum::SOLID),
           "DOTTED" => Ok(LineStyleTypeEnum::DOTTED),
           "MEDIUM_DASHED" => Ok(LineStyleTypeEnum::MEDIUMDASHED),
           "MEDIUM_DASHED_DOTTED" => Ok(LineStyleTypeEnum::MEDIUMDASHEDDOTTED),
           "LONG_DASHED" => Ok(LineStyleTypeEnum::LONGDASHED),
           "LONG_DASHED_DOTTED" => Ok(LineStyleTypeEnum::LONGDASHEDDOTTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LineStyleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MergeCellsRequestMergeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the cells should be merged.
pub enum MergeCellsRequestMergeTypeEnum {
    

    /// Create a single merge from the range
    ///
    /// "MERGE_ALL"
    #[serde(rename="MERGE_ALL")]
    MERGEALL,
    

    /// Create a merge for each column in the range
    ///
    /// "MERGE_COLUMNS"
    #[serde(rename="MERGE_COLUMNS")]
    MERGECOLUMNS,
    

    /// Create a merge for each row in the range
    ///
    /// "MERGE_ROWS"
    #[serde(rename="MERGE_ROWS")]
    MERGEROWS,
}

impl AsRef<str> for MergeCellsRequestMergeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MergeCellsRequestMergeTypeEnum::MERGEALL => "MERGE_ALL",
            MergeCellsRequestMergeTypeEnum::MERGECOLUMNS => "MERGE_COLUMNS",
            MergeCellsRequestMergeTypeEnum::MERGEROWS => "MERGE_ROWS",
        }
    }
}

impl std::convert::TryFrom< &str> for MergeCellsRequestMergeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MERGE_ALL" => Ok(MergeCellsRequestMergeTypeEnum::MERGEALL),
           "MERGE_COLUMNS" => Ok(MergeCellsRequestMergeTypeEnum::MERGECOLUMNS),
           "MERGE_ROWS" => Ok(MergeCellsRequestMergeTypeEnum::MERGEROWS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MergeCellsRequestMergeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NumberFormatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the number format. When writing, this field must be set.
pub enum NumberFormatTypeEnum {
    

    /// The number format is not specified and is based on the contents of the cell. Do not explicitly use this.
    ///
    /// "NUMBER_FORMAT_TYPE_UNSPECIFIED"
    #[serde(rename="NUMBER_FORMAT_TYPE_UNSPECIFIED")]
    NUMBERFORMATTYPEUNSPECIFIED,
    

    /// Text formatting, e.g `1000.12`
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Number formatting, e.g, `1,000.12`
    ///
    /// "NUMBER"
    #[serde(rename="NUMBER")]
    NUMBER,
    

    /// Percent formatting, e.g `10.12%`
    ///
    /// "PERCENT"
    #[serde(rename="PERCENT")]
    PERCENT,
    

    /// Currency formatting, e.g `$1,000.12`
    ///
    /// "CURRENCY"
    #[serde(rename="CURRENCY")]
    CURRENCY,
    

    /// Date formatting, e.g `9/26/2008`
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Time formatting, e.g `3:59:00 PM`
    ///
    /// "TIME"
    #[serde(rename="TIME")]
    TIME,
    

    /// Date+Time formatting, e.g `9/26/08 15:59:00`
    ///
    /// "DATE_TIME"
    #[serde(rename="DATE_TIME")]
    DATETIME,
    

    /// Scientific number formatting, e.g `1.01E+03`
    ///
    /// "SCIENTIFIC"
    #[serde(rename="SCIENTIFIC")]
    SCIENTIFIC,
}

impl AsRef<str> for NumberFormatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NumberFormatTypeEnum::NUMBERFORMATTYPEUNSPECIFIED => "NUMBER_FORMAT_TYPE_UNSPECIFIED",
            NumberFormatTypeEnum::TEXT => "TEXT",
            NumberFormatTypeEnum::NUMBER => "NUMBER",
            NumberFormatTypeEnum::PERCENT => "PERCENT",
            NumberFormatTypeEnum::CURRENCY => "CURRENCY",
            NumberFormatTypeEnum::DATE => "DATE",
            NumberFormatTypeEnum::TIME => "TIME",
            NumberFormatTypeEnum::DATETIME => "DATE_TIME",
            NumberFormatTypeEnum::SCIENTIFIC => "SCIENTIFIC",
        }
    }
}

impl std::convert::TryFrom< &str> for NumberFormatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NUMBER_FORMAT_TYPE_UNSPECIFIED" => Ok(NumberFormatTypeEnum::NUMBERFORMATTYPEUNSPECIFIED),
           "TEXT" => Ok(NumberFormatTypeEnum::TEXT),
           "NUMBER" => Ok(NumberFormatTypeEnum::NUMBER),
           "PERCENT" => Ok(NumberFormatTypeEnum::PERCENT),
           "CURRENCY" => Ok(NumberFormatTypeEnum::CURRENCY),
           "DATE" => Ok(NumberFormatTypeEnum::DATE),
           "TIME" => Ok(NumberFormatTypeEnum::TIME),
           "DATE_TIME" => Ok(NumberFormatTypeEnum::DATETIME),
           "SCIENTIFIC" => Ok(NumberFormatTypeEnum::SCIENTIFIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NumberFormatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrgChartSpecNodeSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The size of the org chart nodes.
pub enum OrgChartSpecNodeSizeEnum {
    

    /// Default value, do not use.
    ///
    /// "ORG_CHART_LABEL_SIZE_UNSPECIFIED"
    #[serde(rename="ORG_CHART_LABEL_SIZE_UNSPECIFIED")]
    ORGCHARTLABELSIZEUNSPECIFIED,
    

    /// The small org chart node size.
    ///
    /// "SMALL"
    #[serde(rename="SMALL")]
    SMALL,
    

    /// The medium org chart node size.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// The large org chart node size.
    ///
    /// "LARGE"
    #[serde(rename="LARGE")]
    LARGE,
}

impl AsRef<str> for OrgChartSpecNodeSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrgChartSpecNodeSizeEnum::ORGCHARTLABELSIZEUNSPECIFIED => "ORG_CHART_LABEL_SIZE_UNSPECIFIED",
            OrgChartSpecNodeSizeEnum::SMALL => "SMALL",
            OrgChartSpecNodeSizeEnum::MEDIUM => "MEDIUM",
            OrgChartSpecNodeSizeEnum::LARGE => "LARGE",
        }
    }
}

impl std::convert::TryFrom< &str> for OrgChartSpecNodeSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORG_CHART_LABEL_SIZE_UNSPECIFIED" => Ok(OrgChartSpecNodeSizeEnum::ORGCHARTLABELSIZEUNSPECIFIED),
           "SMALL" => Ok(OrgChartSpecNodeSizeEnum::SMALL),
           "MEDIUM" => Ok(OrgChartSpecNodeSizeEnum::MEDIUM),
           "LARGE" => Ok(OrgChartSpecNodeSizeEnum::LARGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrgChartSpecNodeSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasteDataRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the data should be pasted.
pub enum PasteDataRequestTypeEnum {
    

    /// Paste values, formulas, formats, and merges.
    ///
    /// "PASTE_NORMAL"
    #[serde(rename="PASTE_NORMAL")]
    PASTENORMAL,
    

    /// Paste the values ONLY without formats, formulas, or merges.
    ///
    /// "PASTE_VALUES"
    #[serde(rename="PASTE_VALUES")]
    PASTEVALUES,
    

    /// Paste the format and data validation only.
    ///
    /// "PASTE_FORMAT"
    #[serde(rename="PASTE_FORMAT")]
    PASTEFORMAT,
    

    /// Like `PASTE_NORMAL` but without borders.
    ///
    /// "PASTE_NO_BORDERS"
    #[serde(rename="PASTE_NO_BORDERS")]
    PASTENOBORDERS,
    

    /// Paste the formulas only.
    ///
    /// "PASTE_FORMULA"
    #[serde(rename="PASTE_FORMULA")]
    PASTEFORMULA,
    

    /// Paste the data validation only.
    ///
    /// "PASTE_DATA_VALIDATION"
    #[serde(rename="PASTE_DATA_VALIDATION")]
    PASTEDATAVALIDATION,
    

    /// Paste the conditional formatting rules only.
    ///
    /// "PASTE_CONDITIONAL_FORMATTING"
    #[serde(rename="PASTE_CONDITIONAL_FORMATTING")]
    PASTECONDITIONALFORMATTING,
}

impl AsRef<str> for PasteDataRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasteDataRequestTypeEnum::PASTENORMAL => "PASTE_NORMAL",
            PasteDataRequestTypeEnum::PASTEVALUES => "PASTE_VALUES",
            PasteDataRequestTypeEnum::PASTEFORMAT => "PASTE_FORMAT",
            PasteDataRequestTypeEnum::PASTENOBORDERS => "PASTE_NO_BORDERS",
            PasteDataRequestTypeEnum::PASTEFORMULA => "PASTE_FORMULA",
            PasteDataRequestTypeEnum::PASTEDATAVALIDATION => "PASTE_DATA_VALIDATION",
            PasteDataRequestTypeEnum::PASTECONDITIONALFORMATTING => "PASTE_CONDITIONAL_FORMATTING",
        }
    }
}

impl std::convert::TryFrom< &str> for PasteDataRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASTE_NORMAL" => Ok(PasteDataRequestTypeEnum::PASTENORMAL),
           "PASTE_VALUES" => Ok(PasteDataRequestTypeEnum::PASTEVALUES),
           "PASTE_FORMAT" => Ok(PasteDataRequestTypeEnum::PASTEFORMAT),
           "PASTE_NO_BORDERS" => Ok(PasteDataRequestTypeEnum::PASTENOBORDERS),
           "PASTE_FORMULA" => Ok(PasteDataRequestTypeEnum::PASTEFORMULA),
           "PASTE_DATA_VALIDATION" => Ok(PasteDataRequestTypeEnum::PASTEDATAVALIDATION),
           "PASTE_CONDITIONAL_FORMATTING" => Ok(PasteDataRequestTypeEnum::PASTECONDITIONALFORMATTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasteDataRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PieChartSpecLegendPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Where the legend of the pie chart should be drawn.
pub enum PieChartSpecLegendPositionEnum {
    

    /// Default value, do not use.
    ///
    /// "PIE_CHART_LEGEND_POSITION_UNSPECIFIED"
    #[serde(rename="PIE_CHART_LEGEND_POSITION_UNSPECIFIED")]
    PIECHARTLEGENDPOSITIONUNSPECIFIED,
    

    /// The legend is rendered on the bottom of the chart.
    ///
    /// "BOTTOM_LEGEND"
    #[serde(rename="BOTTOM_LEGEND")]
    BOTTOMLEGEND,
    

    /// The legend is rendered on the left of the chart.
    ///
    /// "LEFT_LEGEND"
    #[serde(rename="LEFT_LEGEND")]
    LEFTLEGEND,
    

    /// The legend is rendered on the right of the chart.
    ///
    /// "RIGHT_LEGEND"
    #[serde(rename="RIGHT_LEGEND")]
    RIGHTLEGEND,
    

    /// The legend is rendered on the top of the chart.
    ///
    /// "TOP_LEGEND"
    #[serde(rename="TOP_LEGEND")]
    TOPLEGEND,
    

    /// No legend is rendered.
    ///
    /// "NO_LEGEND"
    #[serde(rename="NO_LEGEND")]
    NOLEGEND,
    

    /// Each pie slice has a label attached to it.
    ///
    /// "LABELED_LEGEND"
    #[serde(rename="LABELED_LEGEND")]
    LABELEDLEGEND,
}

impl AsRef<str> for PieChartSpecLegendPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PieChartSpecLegendPositionEnum::PIECHARTLEGENDPOSITIONUNSPECIFIED => "PIE_CHART_LEGEND_POSITION_UNSPECIFIED",
            PieChartSpecLegendPositionEnum::BOTTOMLEGEND => "BOTTOM_LEGEND",
            PieChartSpecLegendPositionEnum::LEFTLEGEND => "LEFT_LEGEND",
            PieChartSpecLegendPositionEnum::RIGHTLEGEND => "RIGHT_LEGEND",
            PieChartSpecLegendPositionEnum::TOPLEGEND => "TOP_LEGEND",
            PieChartSpecLegendPositionEnum::NOLEGEND => "NO_LEGEND",
            PieChartSpecLegendPositionEnum::LABELEDLEGEND => "LABELED_LEGEND",
        }
    }
}

impl std::convert::TryFrom< &str> for PieChartSpecLegendPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PIE_CHART_LEGEND_POSITION_UNSPECIFIED" => Ok(PieChartSpecLegendPositionEnum::PIECHARTLEGENDPOSITIONUNSPECIFIED),
           "BOTTOM_LEGEND" => Ok(PieChartSpecLegendPositionEnum::BOTTOMLEGEND),
           "LEFT_LEGEND" => Ok(PieChartSpecLegendPositionEnum::LEFTLEGEND),
           "RIGHT_LEGEND" => Ok(PieChartSpecLegendPositionEnum::RIGHTLEGEND),
           "TOP_LEGEND" => Ok(PieChartSpecLegendPositionEnum::TOPLEGEND),
           "NO_LEGEND" => Ok(PieChartSpecLegendPositionEnum::NOLEGEND),
           "LABELED_LEGEND" => Ok(PieChartSpecLegendPositionEnum::LABELEDLEGEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PieChartSpecLegendPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PivotGroupSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order the values in this group should be sorted.
pub enum PivotGroupSortOrderEnum {
    

    /// Default value, do not use this.
    ///
    /// "SORT_ORDER_UNSPECIFIED"
    #[serde(rename="SORT_ORDER_UNSPECIFIED")]
    SORTORDERUNSPECIFIED,
    

    /// Sort ascending.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Sort descending.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for PivotGroupSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PivotGroupSortOrderEnum::SORTORDERUNSPECIFIED => "SORT_ORDER_UNSPECIFIED",
            PivotGroupSortOrderEnum::ASCENDING => "ASCENDING",
            PivotGroupSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PivotGroupSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_ORDER_UNSPECIFIED" => Ok(PivotGroupSortOrderEnum::SORTORDERUNSPECIFIED),
           "ASCENDING" => Ok(PivotGroupSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(PivotGroupSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PivotGroupSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PivotTableValueLayoutEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether values should be listed horizontally (as columns) or vertically (as rows).
pub enum PivotTableValueLayoutEnum {
    

    /// Values are laid out horizontally (as columns).
    ///
    /// "HORIZONTAL"
    #[serde(rename="HORIZONTAL")]
    HORIZONTAL,
    

    /// Values are laid out vertically (as rows).
    ///
    /// "VERTICAL"
    #[serde(rename="VERTICAL")]
    VERTICAL,
}

impl AsRef<str> for PivotTableValueLayoutEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PivotTableValueLayoutEnum::HORIZONTAL => "HORIZONTAL",
            PivotTableValueLayoutEnum::VERTICAL => "VERTICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for PivotTableValueLayoutEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HORIZONTAL" => Ok(PivotTableValueLayoutEnum::HORIZONTAL),
           "VERTICAL" => Ok(PivotTableValueLayoutEnum::VERTICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PivotTableValueLayoutEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PivotValueCalculatedDisplayTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If specified, indicates that pivot values should be displayed as the result of a calculation with another pivot value. For example, if calculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the pivot values are displayed as the percentage of the grand total. In the Sheets editor, this is referred to as "Show As" in the value section of a pivot table.
pub enum PivotValueCalculatedDisplayTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED"
    #[serde(rename="PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED")]
    PIVOTVALUECALCULATEDDISPLAYTYPEUNSPECIFIED,
    

    /// Shows the pivot values as percentage of the row total values.
    ///
    /// "PERCENT_OF_ROW_TOTAL"
    #[serde(rename="PERCENT_OF_ROW_TOTAL")]
    PERCENTOFROWTOTAL,
    

    /// Shows the pivot values as percentage of the column total values.
    ///
    /// "PERCENT_OF_COLUMN_TOTAL"
    #[serde(rename="PERCENT_OF_COLUMN_TOTAL")]
    PERCENTOFCOLUMNTOTAL,
    

    /// Shows the pivot values as percentage of the grand total values.
    ///
    /// "PERCENT_OF_GRAND_TOTAL"
    #[serde(rename="PERCENT_OF_GRAND_TOTAL")]
    PERCENTOFGRANDTOTAL,
}

impl AsRef<str> for PivotValueCalculatedDisplayTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PivotValueCalculatedDisplayTypeEnum::PIVOTVALUECALCULATEDDISPLAYTYPEUNSPECIFIED => "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED",
            PivotValueCalculatedDisplayTypeEnum::PERCENTOFROWTOTAL => "PERCENT_OF_ROW_TOTAL",
            PivotValueCalculatedDisplayTypeEnum::PERCENTOFCOLUMNTOTAL => "PERCENT_OF_COLUMN_TOTAL",
            PivotValueCalculatedDisplayTypeEnum::PERCENTOFGRANDTOTAL => "PERCENT_OF_GRAND_TOTAL",
        }
    }
}

impl std::convert::TryFrom< &str> for PivotValueCalculatedDisplayTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PIVOT_VALUE_CALCULATED_DISPLAY_TYPE_UNSPECIFIED" => Ok(PivotValueCalculatedDisplayTypeEnum::PIVOTVALUECALCULATEDDISPLAYTYPEUNSPECIFIED),
           "PERCENT_OF_ROW_TOTAL" => Ok(PivotValueCalculatedDisplayTypeEnum::PERCENTOFROWTOTAL),
           "PERCENT_OF_COLUMN_TOTAL" => Ok(PivotValueCalculatedDisplayTypeEnum::PERCENTOFCOLUMNTOTAL),
           "PERCENT_OF_GRAND_TOTAL" => Ok(PivotValueCalculatedDisplayTypeEnum::PERCENTOFGRANDTOTAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PivotValueCalculatedDisplayTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PivotValueSummarizeFunctionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A function to summarize the value. If formula is set, the only supported values are SUM and CUSTOM. If sourceColumnOffset is set, then `CUSTOM` is not supported.
pub enum PivotValueSummarizeFunctionEnum {
    

    /// The default, do not use.
    ///
    /// "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED"
    #[serde(rename="PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED")]
    PIVOTSTANDARDVALUEFUNCTIONUNSPECIFIED,
    

    /// Corresponds to the `SUM` function.
    ///
    /// "SUM"
    #[serde(rename="SUM")]
    SUM,
    

    /// Corresponds to the `COUNTA` function.
    ///
    /// "COUNTA"
    #[serde(rename="COUNTA")]
    COUNTA,
    

    /// Corresponds to the `COUNT` function.
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
    

    /// Corresponds to the `COUNTUNIQUE` function.
    ///
    /// "COUNTUNIQUE"
    #[serde(rename="COUNTUNIQUE")]
    COUNTUNIQUE,
    

    /// Corresponds to the `AVERAGE` function.
    ///
    /// "AVERAGE"
    #[serde(rename="AVERAGE")]
    AVERAGE,
    

    /// Corresponds to the `MAX` function.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
    

    /// Corresponds to the `MIN` function.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Corresponds to the `MEDIAN` function.
    ///
    /// "MEDIAN"
    #[serde(rename="MEDIAN")]
    MEDIAN,
    

    /// Corresponds to the `PRODUCT` function.
    ///
    /// "PRODUCT"
    #[serde(rename="PRODUCT")]
    PRODUCT,
    

    /// Corresponds to the `STDEV` function.
    ///
    /// "STDEV"
    #[serde(rename="STDEV")]
    STDEV,
    

    /// Corresponds to the `STDEVP` function.
    ///
    /// "STDEVP"
    #[serde(rename="STDEVP")]
    STDEVP,
    

    /// Corresponds to the `VAR` function.
    ///
    /// "VAR"
    #[serde(rename="VAR")]
    VAR,
    

    /// Corresponds to the `VARP` function.
    ///
    /// "VARP"
    #[serde(rename="VARP")]
    VARP,
    

    /// Indicates the formula should be used as-is. Only valid if PivotValue.formula was set.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
    

    /// Indicates that the value is already summarized, and the summarization function is not explicitly specified. Used for Looker data source pivot tables where the value is already summarized.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for PivotValueSummarizeFunctionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PivotValueSummarizeFunctionEnum::PIVOTSTANDARDVALUEFUNCTIONUNSPECIFIED => "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED",
            PivotValueSummarizeFunctionEnum::SUM => "SUM",
            PivotValueSummarizeFunctionEnum::COUNTA => "COUNTA",
            PivotValueSummarizeFunctionEnum::COUNT => "COUNT",
            PivotValueSummarizeFunctionEnum::COUNTUNIQUE => "COUNTUNIQUE",
            PivotValueSummarizeFunctionEnum::AVERAGE => "AVERAGE",
            PivotValueSummarizeFunctionEnum::MAX => "MAX",
            PivotValueSummarizeFunctionEnum::MIN => "MIN",
            PivotValueSummarizeFunctionEnum::MEDIAN => "MEDIAN",
            PivotValueSummarizeFunctionEnum::PRODUCT => "PRODUCT",
            PivotValueSummarizeFunctionEnum::STDEV => "STDEV",
            PivotValueSummarizeFunctionEnum::STDEVP => "STDEVP",
            PivotValueSummarizeFunctionEnum::VAR => "VAR",
            PivotValueSummarizeFunctionEnum::VARP => "VARP",
            PivotValueSummarizeFunctionEnum::CUSTOM => "CUSTOM",
            PivotValueSummarizeFunctionEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for PivotValueSummarizeFunctionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PIVOT_STANDARD_VALUE_FUNCTION_UNSPECIFIED" => Ok(PivotValueSummarizeFunctionEnum::PIVOTSTANDARDVALUEFUNCTIONUNSPECIFIED),
           "SUM" => Ok(PivotValueSummarizeFunctionEnum::SUM),
           "COUNTA" => Ok(PivotValueSummarizeFunctionEnum::COUNTA),
           "COUNT" => Ok(PivotValueSummarizeFunctionEnum::COUNT),
           "COUNTUNIQUE" => Ok(PivotValueSummarizeFunctionEnum::COUNTUNIQUE),
           "AVERAGE" => Ok(PivotValueSummarizeFunctionEnum::AVERAGE),
           "MAX" => Ok(PivotValueSummarizeFunctionEnum::MAX),
           "MIN" => Ok(PivotValueSummarizeFunctionEnum::MIN),
           "MEDIAN" => Ok(PivotValueSummarizeFunctionEnum::MEDIAN),
           "PRODUCT" => Ok(PivotValueSummarizeFunctionEnum::PRODUCT),
           "STDEV" => Ok(PivotValueSummarizeFunctionEnum::STDEV),
           "STDEVP" => Ok(PivotValueSummarizeFunctionEnum::STDEVP),
           "VAR" => Ok(PivotValueSummarizeFunctionEnum::VAR),
           "VARP" => Ok(PivotValueSummarizeFunctionEnum::VARP),
           "CUSTOM" => Ok(PivotValueSummarizeFunctionEnum::CUSTOM),
           "NONE" => Ok(PivotValueSummarizeFunctionEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PivotValueSummarizeFunctionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PointStyleShapeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The point shape. If empty or unspecified, a default shape is used.
pub enum PointStyleShapeEnum {
    

    /// Default value.
    ///
    /// "POINT_SHAPE_UNSPECIFIED"
    #[serde(rename="POINT_SHAPE_UNSPECIFIED")]
    POINTSHAPEUNSPECIFIED,
    

    /// A circle shape.
    ///
    /// "CIRCLE"
    #[serde(rename="CIRCLE")]
    CIRCLE,
    

    /// A diamond shape.
    ///
    /// "DIAMOND"
    #[serde(rename="DIAMOND")]
    DIAMOND,
    

    /// A hexagon shape.
    ///
    /// "HEXAGON"
    #[serde(rename="HEXAGON")]
    HEXAGON,
    

    /// A pentagon shape.
    ///
    /// "PENTAGON"
    #[serde(rename="PENTAGON")]
    PENTAGON,
    

    /// A square shape.
    ///
    /// "SQUARE"
    #[serde(rename="SQUARE")]
    SQUARE,
    

    /// A star shape.
    ///
    /// "STAR"
    #[serde(rename="STAR")]
    STAR,
    

    /// A triangle shape.
    ///
    /// "TRIANGLE"
    #[serde(rename="TRIANGLE")]
    TRIANGLE,
    

    /// An x-mark shape.
    ///
    /// "X_MARK"
    #[serde(rename="X_MARK")]
    XMARK,
}

impl AsRef<str> for PointStyleShapeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PointStyleShapeEnum::POINTSHAPEUNSPECIFIED => "POINT_SHAPE_UNSPECIFIED",
            PointStyleShapeEnum::CIRCLE => "CIRCLE",
            PointStyleShapeEnum::DIAMOND => "DIAMOND",
            PointStyleShapeEnum::HEXAGON => "HEXAGON",
            PointStyleShapeEnum::PENTAGON => "PENTAGON",
            PointStyleShapeEnum::SQUARE => "SQUARE",
            PointStyleShapeEnum::STAR => "STAR",
            PointStyleShapeEnum::TRIANGLE => "TRIANGLE",
            PointStyleShapeEnum::XMARK => "X_MARK",
        }
    }
}

impl std::convert::TryFrom< &str> for PointStyleShapeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POINT_SHAPE_UNSPECIFIED" => Ok(PointStyleShapeEnum::POINTSHAPEUNSPECIFIED),
           "CIRCLE" => Ok(PointStyleShapeEnum::CIRCLE),
           "DIAMOND" => Ok(PointStyleShapeEnum::DIAMOND),
           "HEXAGON" => Ok(PointStyleShapeEnum::HEXAGON),
           "PENTAGON" => Ok(PointStyleShapeEnum::PENTAGON),
           "SQUARE" => Ok(PointStyleShapeEnum::SQUARE),
           "STAR" => Ok(PointStyleShapeEnum::STAR),
           "TRIANGLE" => Ok(PointStyleShapeEnum::TRIANGLE),
           "X_MARK" => Ok(PointStyleShapeEnum::XMARK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PointStyleShapeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScorecardChartSpecAggregateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The aggregation type for key and baseline chart data in scorecard chart. This field is not supported for data source charts. Use the ChartData.aggregateType field of the key_value_data or baseline_value_data instead for data source charts. This field is optional.
pub enum ScorecardChartSpecAggregateTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "CHART_AGGREGATE_TYPE_UNSPECIFIED"
    #[serde(rename="CHART_AGGREGATE_TYPE_UNSPECIFIED")]
    CHARTAGGREGATETYPEUNSPECIFIED,
    

    /// Average aggregate function.
    ///
    /// "AVERAGE"
    #[serde(rename="AVERAGE")]
    AVERAGE,
    

    /// Count aggregate function.
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
    

    /// Maximum aggregate function.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
    

    /// Median aggregate function.
    ///
    /// "MEDIAN"
    #[serde(rename="MEDIAN")]
    MEDIAN,
    

    /// Minimum aggregate function.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Sum aggregate function.
    ///
    /// "SUM"
    #[serde(rename="SUM")]
    SUM,
}

impl AsRef<str> for ScorecardChartSpecAggregateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScorecardChartSpecAggregateTypeEnum::CHARTAGGREGATETYPEUNSPECIFIED => "CHART_AGGREGATE_TYPE_UNSPECIFIED",
            ScorecardChartSpecAggregateTypeEnum::AVERAGE => "AVERAGE",
            ScorecardChartSpecAggregateTypeEnum::COUNT => "COUNT",
            ScorecardChartSpecAggregateTypeEnum::MAX => "MAX",
            ScorecardChartSpecAggregateTypeEnum::MEDIAN => "MEDIAN",
            ScorecardChartSpecAggregateTypeEnum::MIN => "MIN",
            ScorecardChartSpecAggregateTypeEnum::SUM => "SUM",
        }
    }
}

impl std::convert::TryFrom< &str> for ScorecardChartSpecAggregateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHART_AGGREGATE_TYPE_UNSPECIFIED" => Ok(ScorecardChartSpecAggregateTypeEnum::CHARTAGGREGATETYPEUNSPECIFIED),
           "AVERAGE" => Ok(ScorecardChartSpecAggregateTypeEnum::AVERAGE),
           "COUNT" => Ok(ScorecardChartSpecAggregateTypeEnum::COUNT),
           "MAX" => Ok(ScorecardChartSpecAggregateTypeEnum::MAX),
           "MEDIAN" => Ok(ScorecardChartSpecAggregateTypeEnum::MEDIAN),
           "MIN" => Ok(ScorecardChartSpecAggregateTypeEnum::MIN),
           "SUM" => Ok(ScorecardChartSpecAggregateTypeEnum::SUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScorecardChartSpecAggregateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScorecardChartSpecNumberFormatSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The number format source used in the scorecard chart. This field is optional.
pub enum ScorecardChartSpecNumberFormatSourceEnum {
    

    /// Default value, do not use.
    ///
    /// "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED"
    #[serde(rename="CHART_NUMBER_FORMAT_SOURCE_UNDEFINED")]
    CHARTNUMBERFORMATSOURCEUNDEFINED,
    

    /// Inherit number formatting from data.
    ///
    /// "FROM_DATA"
    #[serde(rename="FROM_DATA")]
    FROMDATA,
    

    /// Apply custom formatting as specified by ChartCustomNumberFormatOptions.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for ScorecardChartSpecNumberFormatSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScorecardChartSpecNumberFormatSourceEnum::CHARTNUMBERFORMATSOURCEUNDEFINED => "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED",
            ScorecardChartSpecNumberFormatSourceEnum::FROMDATA => "FROM_DATA",
            ScorecardChartSpecNumberFormatSourceEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for ScorecardChartSpecNumberFormatSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHART_NUMBER_FORMAT_SOURCE_UNDEFINED" => Ok(ScorecardChartSpecNumberFormatSourceEnum::CHARTNUMBERFORMATSOURCEUNDEFINED),
           "FROM_DATA" => Ok(ScorecardChartSpecNumberFormatSourceEnum::FROMDATA),
           "CUSTOM" => Ok(ScorecardChartSpecNumberFormatSourceEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScorecardChartSpecNumberFormatSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SheetPropertySheetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of sheet. Defaults to GRID. This field cannot be changed once set.
pub enum SheetPropertySheetTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "SHEET_TYPE_UNSPECIFIED"
    #[serde(rename="SHEET_TYPE_UNSPECIFIED")]
    SHEETTYPEUNSPECIFIED,
    

    /// The sheet is a grid.
    ///
    /// "GRID"
    #[serde(rename="GRID")]
    GRID,
    

    /// The sheet has no grid and instead has an object like a chart or image.
    ///
    /// "OBJECT"
    #[serde(rename="OBJECT")]
    OBJECT,
    

    /// The sheet connects with an external DataSource and shows the preview of data.
    ///
    /// "DATA_SOURCE"
    #[serde(rename="DATA_SOURCE")]
    DATASOURCE,
}

impl AsRef<str> for SheetPropertySheetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SheetPropertySheetTypeEnum::SHEETTYPEUNSPECIFIED => "SHEET_TYPE_UNSPECIFIED",
            SheetPropertySheetTypeEnum::GRID => "GRID",
            SheetPropertySheetTypeEnum::OBJECT => "OBJECT",
            SheetPropertySheetTypeEnum::DATASOURCE => "DATA_SOURCE",
        }
    }
}

impl std::convert::TryFrom< &str> for SheetPropertySheetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SHEET_TYPE_UNSPECIFIED" => Ok(SheetPropertySheetTypeEnum::SHEETTYPEUNSPECIFIED),
           "GRID" => Ok(SheetPropertySheetTypeEnum::GRID),
           "OBJECT" => Ok(SheetPropertySheetTypeEnum::OBJECT),
           "DATA_SOURCE" => Ok(SheetPropertySheetTypeEnum::DATASOURCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SheetPropertySheetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SlicerSpecHorizontalAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The horizontal alignment of title in the slicer. If unspecified, defaults to `LEFT`
pub enum SlicerSpecHorizontalAlignmentEnum {
    

    /// The horizontal alignment is not specified. Do not use this.
    ///
    /// "HORIZONTAL_ALIGN_UNSPECIFIED"
    #[serde(rename="HORIZONTAL_ALIGN_UNSPECIFIED")]
    HORIZONTALALIGNUNSPECIFIED,
    

    /// The text is explicitly aligned to the left of the cell.
    ///
    /// "LEFT"
    #[serde(rename="LEFT")]
    LEFT,
    

    /// The text is explicitly aligned to the center of the cell.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// The text is explicitly aligned to the right of the cell.
    ///
    /// "RIGHT"
    #[serde(rename="RIGHT")]
    RIGHT,
}

impl AsRef<str> for SlicerSpecHorizontalAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SlicerSpecHorizontalAlignmentEnum::HORIZONTALALIGNUNSPECIFIED => "HORIZONTAL_ALIGN_UNSPECIFIED",
            SlicerSpecHorizontalAlignmentEnum::LEFT => "LEFT",
            SlicerSpecHorizontalAlignmentEnum::CENTER => "CENTER",
            SlicerSpecHorizontalAlignmentEnum::RIGHT => "RIGHT",
        }
    }
}

impl std::convert::TryFrom< &str> for SlicerSpecHorizontalAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HORIZONTAL_ALIGN_UNSPECIFIED" => Ok(SlicerSpecHorizontalAlignmentEnum::HORIZONTALALIGNUNSPECIFIED),
           "LEFT" => Ok(SlicerSpecHorizontalAlignmentEnum::LEFT),
           "CENTER" => Ok(SlicerSpecHorizontalAlignmentEnum::CENTER),
           "RIGHT" => Ok(SlicerSpecHorizontalAlignmentEnum::RIGHT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SlicerSpecHorizontalAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SortSpecSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order data should be sorted.
pub enum SortSpecSortOrderEnum {
    

    /// Default value, do not use this.
    ///
    /// "SORT_ORDER_UNSPECIFIED"
    #[serde(rename="SORT_ORDER_UNSPECIFIED")]
    SORTORDERUNSPECIFIED,
    

    /// Sort ascending.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Sort descending.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for SortSpecSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SortSpecSortOrderEnum::SORTORDERUNSPECIFIED => "SORT_ORDER_UNSPECIFIED",
            SortSpecSortOrderEnum::ASCENDING => "ASCENDING",
            SortSpecSortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for SortSpecSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_ORDER_UNSPECIFIED" => Ok(SortSpecSortOrderEnum::SORTORDERUNSPECIFIED),
           "ASCENDING" => Ok(SortSpecSortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(SortSpecSortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SortSpecSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SourceAndDestinationDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The dimension that data should be filled into.
pub enum SourceAndDestinationDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for SourceAndDestinationDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SourceAndDestinationDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            SourceAndDestinationDimensionEnum::ROWS => "ROWS",
            SourceAndDestinationDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for SourceAndDestinationDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(SourceAndDestinationDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(SourceAndDestinationDimensionEnum::ROWS),
           "COLUMNS" => Ok(SourceAndDestinationDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SourceAndDestinationDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetPropertyAutoRecalcEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The amount of time to wait before volatile functions are recalculated.
pub enum SpreadsheetPropertyAutoRecalcEnum {
    

    /// Default value. This value must not be used.
    ///
    /// "RECALCULATION_INTERVAL_UNSPECIFIED"
    #[serde(rename="RECALCULATION_INTERVAL_UNSPECIFIED")]
    RECALCULATIONINTERVALUNSPECIFIED,
    

    /// Volatile functions are updated on every change.
    ///
    /// "ON_CHANGE"
    #[serde(rename="ON_CHANGE")]
    ONCHANGE,
    

    /// Volatile functions are updated on every change and every minute.
    ///
    /// "MINUTE"
    #[serde(rename="MINUTE")]
    MINUTE,
    

    /// Volatile functions are updated on every change and hourly.
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
}

impl AsRef<str> for SpreadsheetPropertyAutoRecalcEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetPropertyAutoRecalcEnum::RECALCULATIONINTERVALUNSPECIFIED => "RECALCULATION_INTERVAL_UNSPECIFIED",
            SpreadsheetPropertyAutoRecalcEnum::ONCHANGE => "ON_CHANGE",
            SpreadsheetPropertyAutoRecalcEnum::MINUTE => "MINUTE",
            SpreadsheetPropertyAutoRecalcEnum::HOUR => "HOUR",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetPropertyAutoRecalcEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECALCULATION_INTERVAL_UNSPECIFIED" => Ok(SpreadsheetPropertyAutoRecalcEnum::RECALCULATIONINTERVALUNSPECIFIED),
           "ON_CHANGE" => Ok(SpreadsheetPropertyAutoRecalcEnum::ONCHANGE),
           "MINUTE" => Ok(SpreadsheetPropertyAutoRecalcEnum::MINUTE),
           "HOUR" => Ok(SpreadsheetPropertyAutoRecalcEnum::HOUR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetPropertyAutoRecalcEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TextPositionHorizontalAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Horizontal alignment setting for the piece of text.
pub enum TextPositionHorizontalAlignmentEnum {
    

    /// The horizontal alignment is not specified. Do not use this.
    ///
    /// "HORIZONTAL_ALIGN_UNSPECIFIED"
    #[serde(rename="HORIZONTAL_ALIGN_UNSPECIFIED")]
    HORIZONTALALIGNUNSPECIFIED,
    

    /// The text is explicitly aligned to the left of the cell.
    ///
    /// "LEFT"
    #[serde(rename="LEFT")]
    LEFT,
    

    /// The text is explicitly aligned to the center of the cell.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// The text is explicitly aligned to the right of the cell.
    ///
    /// "RIGHT"
    #[serde(rename="RIGHT")]
    RIGHT,
}

impl AsRef<str> for TextPositionHorizontalAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TextPositionHorizontalAlignmentEnum::HORIZONTALALIGNUNSPECIFIED => "HORIZONTAL_ALIGN_UNSPECIFIED",
            TextPositionHorizontalAlignmentEnum::LEFT => "LEFT",
            TextPositionHorizontalAlignmentEnum::CENTER => "CENTER",
            TextPositionHorizontalAlignmentEnum::RIGHT => "RIGHT",
        }
    }
}

impl std::convert::TryFrom< &str> for TextPositionHorizontalAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HORIZONTAL_ALIGN_UNSPECIFIED" => Ok(TextPositionHorizontalAlignmentEnum::HORIZONTALALIGNUNSPECIFIED),
           "LEFT" => Ok(TextPositionHorizontalAlignmentEnum::LEFT),
           "CENTER" => Ok(TextPositionHorizontalAlignmentEnum::CENTER),
           "RIGHT" => Ok(TextPositionHorizontalAlignmentEnum::RIGHT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TextPositionHorizontalAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TextToColumnsRequestDelimiterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The delimiter type to use.
pub enum TextToColumnsRequestDelimiterTypeEnum {
    

    /// Default value. This value must not be used.
    ///
    /// "DELIMITER_TYPE_UNSPECIFIED"
    #[serde(rename="DELIMITER_TYPE_UNSPECIFIED")]
    DELIMITERTYPEUNSPECIFIED,
    

    /// ","
    ///
    /// "COMMA"
    #[serde(rename="COMMA")]
    COMMA,
    

    /// ";"
    ///
    /// "SEMICOLON"
    #[serde(rename="SEMICOLON")]
    SEMICOLON,
    

    /// "."
    ///
    /// "PERIOD"
    #[serde(rename="PERIOD")]
    PERIOD,
    

    /// " "
    ///
    /// "SPACE"
    #[serde(rename="SPACE")]
    SPACE,
    

    /// A custom value as defined in delimiter.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
    

    /// Automatically detect columns.
    ///
    /// "AUTODETECT"
    #[serde(rename="AUTODETECT")]
    AUTODETECT,
}

impl AsRef<str> for TextToColumnsRequestDelimiterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TextToColumnsRequestDelimiterTypeEnum::DELIMITERTYPEUNSPECIFIED => "DELIMITER_TYPE_UNSPECIFIED",
            TextToColumnsRequestDelimiterTypeEnum::COMMA => "COMMA",
            TextToColumnsRequestDelimiterTypeEnum::SEMICOLON => "SEMICOLON",
            TextToColumnsRequestDelimiterTypeEnum::PERIOD => "PERIOD",
            TextToColumnsRequestDelimiterTypeEnum::SPACE => "SPACE",
            TextToColumnsRequestDelimiterTypeEnum::CUSTOM => "CUSTOM",
            TextToColumnsRequestDelimiterTypeEnum::AUTODETECT => "AUTODETECT",
        }
    }
}

impl std::convert::TryFrom< &str> for TextToColumnsRequestDelimiterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELIMITER_TYPE_UNSPECIFIED" => Ok(TextToColumnsRequestDelimiterTypeEnum::DELIMITERTYPEUNSPECIFIED),
           "COMMA" => Ok(TextToColumnsRequestDelimiterTypeEnum::COMMA),
           "SEMICOLON" => Ok(TextToColumnsRequestDelimiterTypeEnum::SEMICOLON),
           "PERIOD" => Ok(TextToColumnsRequestDelimiterTypeEnum::PERIOD),
           "SPACE" => Ok(TextToColumnsRequestDelimiterTypeEnum::SPACE),
           "CUSTOM" => Ok(TextToColumnsRequestDelimiterTypeEnum::CUSTOM),
           "AUTODETECT" => Ok(TextToColumnsRequestDelimiterTypeEnum::AUTODETECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TextToColumnsRequestDelimiterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThemeColorPairColorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the spreadsheet theme color.
pub enum ThemeColorPairColorTypeEnum {
    

    /// Unspecified theme color
    ///
    /// "THEME_COLOR_TYPE_UNSPECIFIED"
    #[serde(rename="THEME_COLOR_TYPE_UNSPECIFIED")]
    THEMECOLORTYPEUNSPECIFIED,
    

    /// Represents the primary text color
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Represents the primary background color
    ///
    /// "BACKGROUND"
    #[serde(rename="BACKGROUND")]
    BACKGROUND,
    

    /// Represents the first accent color
    ///
    /// "ACCENT1"
    #[serde(rename="ACCENT1")]
    ACCENT1,
    

    /// Represents the second accent color
    ///
    /// "ACCENT2"
    #[serde(rename="ACCENT2")]
    ACCENT2,
    

    /// Represents the third accent color
    ///
    /// "ACCENT3"
    #[serde(rename="ACCENT3")]
    ACCENT3,
    

    /// Represents the fourth accent color
    ///
    /// "ACCENT4"
    #[serde(rename="ACCENT4")]
    ACCENT4,
    

    /// Represents the fifth accent color
    ///
    /// "ACCENT5"
    #[serde(rename="ACCENT5")]
    ACCENT5,
    

    /// Represents the sixth accent color
    ///
    /// "ACCENT6"
    #[serde(rename="ACCENT6")]
    ACCENT6,
    

    /// Represents the color to use for hyperlinks
    ///
    /// "LINK"
    #[serde(rename="LINK")]
    LINK,
}

impl AsRef<str> for ThemeColorPairColorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThemeColorPairColorTypeEnum::THEMECOLORTYPEUNSPECIFIED => "THEME_COLOR_TYPE_UNSPECIFIED",
            ThemeColorPairColorTypeEnum::TEXT => "TEXT",
            ThemeColorPairColorTypeEnum::BACKGROUND => "BACKGROUND",
            ThemeColorPairColorTypeEnum::ACCENT1 => "ACCENT1",
            ThemeColorPairColorTypeEnum::ACCENT2 => "ACCENT2",
            ThemeColorPairColorTypeEnum::ACCENT3 => "ACCENT3",
            ThemeColorPairColorTypeEnum::ACCENT4 => "ACCENT4",
            ThemeColorPairColorTypeEnum::ACCENT5 => "ACCENT5",
            ThemeColorPairColorTypeEnum::ACCENT6 => "ACCENT6",
            ThemeColorPairColorTypeEnum::LINK => "LINK",
        }
    }
}

impl std::convert::TryFrom< &str> for ThemeColorPairColorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THEME_COLOR_TYPE_UNSPECIFIED" => Ok(ThemeColorPairColorTypeEnum::THEMECOLORTYPEUNSPECIFIED),
           "TEXT" => Ok(ThemeColorPairColorTypeEnum::TEXT),
           "BACKGROUND" => Ok(ThemeColorPairColorTypeEnum::BACKGROUND),
           "ACCENT1" => Ok(ThemeColorPairColorTypeEnum::ACCENT1),
           "ACCENT2" => Ok(ThemeColorPairColorTypeEnum::ACCENT2),
           "ACCENT3" => Ok(ThemeColorPairColorTypeEnum::ACCENT3),
           "ACCENT4" => Ok(ThemeColorPairColorTypeEnum::ACCENT4),
           "ACCENT5" => Ok(ThemeColorPairColorTypeEnum::ACCENT5),
           "ACCENT6" => Ok(ThemeColorPairColorTypeEnum::ACCENT6),
           "LINK" => Ok(ThemeColorPairColorTypeEnum::LINK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThemeColorPairColorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValueRangeMajorDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS.
pub enum ValueRangeMajorDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for ValueRangeMajorDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValueRangeMajorDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            ValueRangeMajorDimensionEnum::ROWS => "ROWS",
            ValueRangeMajorDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for ValueRangeMajorDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(ValueRangeMajorDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(ValueRangeMajorDimensionEnum::ROWS),
           "COLUMNS" => Ok(ValueRangeMajorDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValueRangeMajorDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WaterfallChartSpecStackedTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The stacked type.
pub enum WaterfallChartSpecStackedTypeEnum {
    

    /// Default value, do not use.
    ///
    /// "WATERFALL_STACKED_TYPE_UNSPECIFIED"
    #[serde(rename="WATERFALL_STACKED_TYPE_UNSPECIFIED")]
    WATERFALLSTACKEDTYPEUNSPECIFIED,
    

    /// Values corresponding to the same domain (horizontal axis) value will be stacked vertically.
    ///
    /// "STACKED"
    #[serde(rename="STACKED")]
    STACKED,
    

    /// Series will spread out along the horizontal axis.
    ///
    /// "SEQUENTIAL"
    #[serde(rename="SEQUENTIAL")]
    SEQUENTIAL,
}

impl AsRef<str> for WaterfallChartSpecStackedTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WaterfallChartSpecStackedTypeEnum::WATERFALLSTACKEDTYPEUNSPECIFIED => "WATERFALL_STACKED_TYPE_UNSPECIFIED",
            WaterfallChartSpecStackedTypeEnum::STACKED => "STACKED",
            WaterfallChartSpecStackedTypeEnum::SEQUENTIAL => "SEQUENTIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for WaterfallChartSpecStackedTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WATERFALL_STACKED_TYPE_UNSPECIFIED" => Ok(WaterfallChartSpecStackedTypeEnum::WATERFALLSTACKEDTYPEUNSPECIFIED),
           "STACKED" => Ok(WaterfallChartSpecStackedTypeEnum::STACKED),
           "SEQUENTIAL" => Ok(WaterfallChartSpecStackedTypeEnum::SEQUENTIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WaterfallChartSpecStackedTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetInsertDataOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the input data should be inserted.
pub enum SpreadsheetInsertDataOptionEnum {
    

    /// The new data overwrites existing data in the areas it is written. (Note: adding data to the end of the sheet will still insert new rows or columns so the data can be written.)
    ///
    /// "OVERWRITE"
    #[serde(rename="OVERWRITE")]
    OVERWRITE,
    

    /// Rows are inserted for the new data.
    ///
    /// "INSERT_ROWS"
    #[serde(rename="INSERT_ROWS")]
    INSERTROWS,
}

impl AsRef<str> for SpreadsheetInsertDataOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetInsertDataOptionEnum::OVERWRITE => "OVERWRITE",
            SpreadsheetInsertDataOptionEnum::INSERTROWS => "INSERT_ROWS",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetInsertDataOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OVERWRITE" => Ok(SpreadsheetInsertDataOptionEnum::OVERWRITE),
           "INSERT_ROWS" => Ok(SpreadsheetInsertDataOptionEnum::INSERTROWS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetInsertDataOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetResponseDateTimeRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
pub enum SpreadsheetResponseDateTimeRenderOptionEnum {
    

    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    ///
    /// "SERIAL_NUMBER"
    #[serde(rename="SERIAL_NUMBER")]
    SERIALNUMBER,
    

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    ///
    /// "FORMATTED_STRING"
    #[serde(rename="FORMATTED_STRING")]
    FORMATTEDSTRING,
}

impl AsRef<str> for SpreadsheetResponseDateTimeRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetResponseDateTimeRenderOptionEnum::SERIALNUMBER => "SERIAL_NUMBER",
            SpreadsheetResponseDateTimeRenderOptionEnum::FORMATTEDSTRING => "FORMATTED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetResponseDateTimeRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERIAL_NUMBER" => Ok(SpreadsheetResponseDateTimeRenderOptionEnum::SERIALNUMBER),
           "FORMATTED_STRING" => Ok(SpreadsheetResponseDateTimeRenderOptionEnum::FORMATTEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetResponseDateTimeRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetResponseValueRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
pub enum SpreadsheetResponseValueRenderOptionEnum {
    

    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `"$1.23"`.
    ///
    /// "FORMATTED_VALUE"
    #[serde(rename="FORMATTED_VALUE")]
    FORMATTEDVALUE,
    

    /// Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`.
    ///
    /// "UNFORMATTED_VALUE"
    #[serde(rename="UNFORMATTED_VALUE")]
    UNFORMATTEDVALUE,
    

    /// Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `"=A1"`. Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see [About date & time values](https://developers.google.com/sheets/api/guides/formats#about_date_time_values).
    ///
    /// "FORMULA"
    #[serde(rename="FORMULA")]
    FORMULA,
}

impl AsRef<str> for SpreadsheetResponseValueRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetResponseValueRenderOptionEnum::FORMATTEDVALUE => "FORMATTED_VALUE",
            SpreadsheetResponseValueRenderOptionEnum::UNFORMATTEDVALUE => "UNFORMATTED_VALUE",
            SpreadsheetResponseValueRenderOptionEnum::FORMULA => "FORMULA",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetResponseValueRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMATTED_VALUE" => Ok(SpreadsheetResponseValueRenderOptionEnum::FORMATTEDVALUE),
           "UNFORMATTED_VALUE" => Ok(SpreadsheetResponseValueRenderOptionEnum::UNFORMATTEDVALUE),
           "FORMULA" => Ok(SpreadsheetResponseValueRenderOptionEnum::FORMULA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetResponseValueRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetValueInputOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the input data should be interpreted.
pub enum SpreadsheetValueInputOptionEnum {
    

    /// Default input value. This value must not be used.
    ///
    /// "INPUT_VALUE_OPTION_UNSPECIFIED"
    #[serde(rename="INPUT_VALUE_OPTION_UNSPECIFIED")]
    INPUTVALUEOPTIONUNSPECIFIED,
    

    /// The values the user has entered will not be parsed and will be stored as-is.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
    

    /// The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    ///
    /// "USER_ENTERED"
    #[serde(rename="USER_ENTERED")]
    USERENTERED,
}

impl AsRef<str> for SpreadsheetValueInputOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetValueInputOptionEnum::INPUTVALUEOPTIONUNSPECIFIED => "INPUT_VALUE_OPTION_UNSPECIFIED",
            SpreadsheetValueInputOptionEnum::RAW => "RAW",
            SpreadsheetValueInputOptionEnum::USERENTERED => "USER_ENTERED",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetValueInputOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INPUT_VALUE_OPTION_UNSPECIFIED" => Ok(SpreadsheetValueInputOptionEnum::INPUTVALUEOPTIONUNSPECIFIED),
           "RAW" => Ok(SpreadsheetValueInputOptionEnum::RAW),
           "USER_ENTERED" => Ok(SpreadsheetValueInputOptionEnum::USERENTERED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetValueInputOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetDateTimeRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
pub enum SpreadsheetDateTimeRenderOptionEnum {
    

    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    ///
    /// "SERIAL_NUMBER"
    #[serde(rename="SERIAL_NUMBER")]
    SERIALNUMBER,
    

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    ///
    /// "FORMATTED_STRING"
    #[serde(rename="FORMATTED_STRING")]
    FORMATTEDSTRING,
}

impl AsRef<str> for SpreadsheetDateTimeRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetDateTimeRenderOptionEnum::SERIALNUMBER => "SERIAL_NUMBER",
            SpreadsheetDateTimeRenderOptionEnum::FORMATTEDSTRING => "FORMATTED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetDateTimeRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERIAL_NUMBER" => Ok(SpreadsheetDateTimeRenderOptionEnum::SERIALNUMBER),
           "FORMATTED_STRING" => Ok(SpreadsheetDateTimeRenderOptionEnum::FORMATTEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetDateTimeRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetMajorDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The major dimension that results should use. For example, if the spreadsheet data in Sheet1 is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=Sheet1!A1:B2?majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=Sheet1!A1:B2?majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
pub enum SpreadsheetMajorDimensionEnum {
    

    /// The default value, do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Operates on the rows of a sheet.
    ///
    /// "ROWS"
    #[serde(rename="ROWS")]
    ROWS,
    

    /// Operates on the columns of a sheet.
    ///
    /// "COLUMNS"
    #[serde(rename="COLUMNS")]
    COLUMNS,
}

impl AsRef<str> for SpreadsheetMajorDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetMajorDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            SpreadsheetMajorDimensionEnum::ROWS => "ROWS",
            SpreadsheetMajorDimensionEnum::COLUMNS => "COLUMNS",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetMajorDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(SpreadsheetMajorDimensionEnum::DIMENSIONUNSPECIFIED),
           "ROWS" => Ok(SpreadsheetMajorDimensionEnum::ROWS),
           "COLUMNS" => Ok(SpreadsheetMajorDimensionEnum::COLUMNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetMajorDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpreadsheetValueRenderOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How values should be represented in the output. The default render option is FORMATTED_VALUE.
pub enum SpreadsheetValueRenderOptionEnum {
    

    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return `"$1.23"`.
    ///
    /// "FORMATTED_VALUE"
    #[serde(rename="FORMATTED_VALUE")]
    FORMATTEDVALUE,
    

    /// Values will be calculated, but not formatted in the reply. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then `A2` would return the number `1.23`.
    ///
    /// "UNFORMATTED_VALUE"
    #[serde(rename="UNFORMATTED_VALUE")]
    UNFORMATTEDVALUE,
    

    /// Values will not be calculated. The reply will include the formulas. For example, if `A1` is `1.23` and `A2` is `=A1` and formatted as currency, then A2 would return `"=A1"`. Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see [About date & time values](https://developers.google.com/sheets/api/guides/formats#about_date_time_values).
    ///
    /// "FORMULA"
    #[serde(rename="FORMULA")]
    FORMULA,
}

impl AsRef<str> for SpreadsheetValueRenderOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpreadsheetValueRenderOptionEnum::FORMATTEDVALUE => "FORMATTED_VALUE",
            SpreadsheetValueRenderOptionEnum::UNFORMATTEDVALUE => "UNFORMATTED_VALUE",
            SpreadsheetValueRenderOptionEnum::FORMULA => "FORMULA",
        }
    }
}

impl std::convert::TryFrom< &str> for SpreadsheetValueRenderOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMATTED_VALUE" => Ok(SpreadsheetValueRenderOptionEnum::FORMATTEDVALUE),
           "UNFORMATTED_VALUE" => Ok(SpreadsheetValueRenderOptionEnum::UNFORMATTEDVALUE),
           "FORMULA" => Ok(SpreadsheetValueRenderOptionEnum::FORMULA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpreadsheetValueRenderOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


