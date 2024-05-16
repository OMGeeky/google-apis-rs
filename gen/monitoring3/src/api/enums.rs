use super::*;



// region AggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum AggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for AggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            AggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            AggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            AggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            AggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            AggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            AggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            AggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            AggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            AggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(AggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(AggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(AggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(AggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(AggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(AggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(AggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(AggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(AggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(AggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(AggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum AggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for AggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            AggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            AggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            AggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            AggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            AggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            AggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            AggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            AggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            AggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            AggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            AggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            AggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            AggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            AggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(AggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(AggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(AggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(AggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(AggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(AggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(AggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(AggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(AggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(AggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AlertPolicyCombinerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How to combine the results of multiple conditions to determine if an incident should be opened. If condition_time_series_query_language is present, this must be COMBINE_UNSPECIFIED.
pub enum AlertPolicyCombinerEnum {
    

    /// An unspecified combiner.
    ///
    /// "COMBINE_UNSPECIFIED"
    #[serde(rename="COMBINE_UNSPECIFIED")]
    COMBINEUNSPECIFIED,
    

    /// Combine conditions using the logical AND operator. An incident is created only if all the conditions are met simultaneously. This combiner is satisfied if all conditions are met, even if they are met on completely different resources.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
    

    /// Combine conditions using the logical OR operator. An incident is created if any of the listed conditions is met.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
    

    /// Combine conditions using logical AND operator, but unlike the regular AND option, an incident is created only if all conditions are met simultaneously on at least one resource.
    ///
    /// "AND_WITH_MATCHING_RESOURCE"
    #[serde(rename="AND_WITH_MATCHING_RESOURCE")]
    ANDWITHMATCHINGRESOURCE,
}

impl AsRef<str> for AlertPolicyCombinerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AlertPolicyCombinerEnum::COMBINEUNSPECIFIED => "COMBINE_UNSPECIFIED",
            AlertPolicyCombinerEnum::AND => "AND",
            AlertPolicyCombinerEnum::OR => "OR",
            AlertPolicyCombinerEnum::ANDWITHMATCHINGRESOURCE => "AND_WITH_MATCHING_RESOURCE",
        }
    }
}

impl std::convert::TryFrom< &str> for AlertPolicyCombinerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMBINE_UNSPECIFIED" => Ok(AlertPolicyCombinerEnum::COMBINEUNSPECIFIED),
           "AND" => Ok(AlertPolicyCombinerEnum::AND),
           "OR" => Ok(AlertPolicyCombinerEnum::OR),
           "AND_WITH_MATCHING_RESOURCE" => Ok(AlertPolicyCombinerEnum::ANDWITHMATCHINGRESOURCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AlertPolicyCombinerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AlertPolicySeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The severity of an alert policy indicates how important incidents generated by that policy are. The severity level will be displayed on the Incident detail page and in notifications.
pub enum AlertPolicySeverityEnum {
    

    /// No severity is specified. This is the default value.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// This is the highest severity level. Use this if the problem could cause significant damage or downtime.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
    

    /// This is the medium severity level. Use this if the problem could cause minor damage or downtime.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// This is the lowest severity level. Use this if the problem is not causing any damage or downtime, but could potentially lead to a problem in the future.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
}

impl AsRef<str> for AlertPolicySeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AlertPolicySeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            AlertPolicySeverityEnum::CRITICAL => "CRITICAL",
            AlertPolicySeverityEnum::ERROR => "ERROR",
            AlertPolicySeverityEnum::WARNING => "WARNING",
        }
    }
}

impl std::convert::TryFrom< &str> for AlertPolicySeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(AlertPolicySeverityEnum::SEVERITYUNSPECIFIED),
           "CRITICAL" => Ok(AlertPolicySeverityEnum::CRITICAL),
           "ERROR" => Ok(AlertPolicySeverityEnum::ERROR),
           "WARNING" => Ok(AlertPolicySeverityEnum::WARNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AlertPolicySeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CollectdValueDataSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of measurement.
pub enum CollectdValueDataSourceTypeEnum {
    

    /// An unspecified data source type. This corresponds to google.api.MetricDescriptor.MetricKind.METRIC_KIND_UNSPECIFIED.
    ///
    /// "UNSPECIFIED_DATA_SOURCE_TYPE"
    #[serde(rename="UNSPECIFIED_DATA_SOURCE_TYPE")]
    UNSPECIFIEDDATASOURCETYPE,
    

    /// An instantaneous measurement of a varying quantity. This corresponds to google.api.MetricDescriptor.MetricKind.GAUGE.
    ///
    /// "GAUGE"
    #[serde(rename="GAUGE")]
    GAUGE,
    

    /// A cumulative value over time. This corresponds to google.api.MetricDescriptor.MetricKind.CUMULATIVE.
    ///
    /// "COUNTER"
    #[serde(rename="COUNTER")]
    COUNTER,
    

    /// A rate of change of the measurement.
    ///
    /// "DERIVE"
    #[serde(rename="DERIVE")]
    DERIVE,
    

    /// An amount of change since the last measurement interval. This corresponds to google.api.MetricDescriptor.MetricKind.DELTA.
    ///
    /// "ABSOLUTE"
    #[serde(rename="ABSOLUTE")]
    ABSOLUTE,
}

impl AsRef<str> for CollectdValueDataSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CollectdValueDataSourceTypeEnum::UNSPECIFIEDDATASOURCETYPE => "UNSPECIFIED_DATA_SOURCE_TYPE",
            CollectdValueDataSourceTypeEnum::GAUGE => "GAUGE",
            CollectdValueDataSourceTypeEnum::COUNTER => "COUNTER",
            CollectdValueDataSourceTypeEnum::DERIVE => "DERIVE",
            CollectdValueDataSourceTypeEnum::ABSOLUTE => "ABSOLUTE",
        }
    }
}

impl std::convert::TryFrom< &str> for CollectdValueDataSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_DATA_SOURCE_TYPE" => Ok(CollectdValueDataSourceTypeEnum::UNSPECIFIEDDATASOURCETYPE),
           "GAUGE" => Ok(CollectdValueDataSourceTypeEnum::GAUGE),
           "COUNTER" => Ok(CollectdValueDataSourceTypeEnum::COUNTER),
           "DERIVE" => Ok(CollectdValueDataSourceTypeEnum::DERIVE),
           "ABSOLUTE" => Ok(CollectdValueDataSourceTypeEnum::ABSOLUTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CollectdValueDataSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentMatcherMatcherEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of content matcher that will be applied to the server output, compared to the content string when the check is run.
pub enum ContentMatcherMatcherEnum {
    

    /// No content matcher type specified (maintained for backward compatibility, but deprecated for future use). Treated as CONTAINS_STRING.
    ///
    /// "CONTENT_MATCHER_OPTION_UNSPECIFIED"
    #[serde(rename="CONTENT_MATCHER_OPTION_UNSPECIFIED")]
    CONTENTMATCHEROPTIONUNSPECIFIED,
    

    /// Selects substring matching. The match succeeds if the output contains the content string. This is the default value for checks without a matcher option, or where the value of matcher is CONTENT_MATCHER_OPTION_UNSPECIFIED.
    ///
    /// "CONTAINS_STRING"
    #[serde(rename="CONTAINS_STRING")]
    CONTAINSSTRING,
    

    /// Selects negation of substring matching. The match succeeds if the output does NOT contain the content string.
    ///
    /// "NOT_CONTAINS_STRING"
    #[serde(rename="NOT_CONTAINS_STRING")]
    NOTCONTAINSSTRING,
    

    /// Selects regular-expression matching. The match succeeds if the output matches the regular expression specified in the content string. Regex matching is only supported for HTTP/HTTPS checks.
    ///
    /// "MATCHES_REGEX"
    #[serde(rename="MATCHES_REGEX")]
    MATCHESREGEX,
    

    /// Selects negation of regular-expression matching. The match succeeds if the output does NOT match the regular expression specified in the content string. Regex matching is only supported for HTTP/HTTPS checks.
    ///
    /// "NOT_MATCHES_REGEX"
    #[serde(rename="NOT_MATCHES_REGEX")]
    NOTMATCHESREGEX,
    

    /// Selects JSONPath matching. See JsonPathMatcher for details on when the match succeeds. JSONPath matching is only supported for HTTP/HTTPS checks.
    ///
    /// "MATCHES_JSON_PATH"
    #[serde(rename="MATCHES_JSON_PATH")]
    MATCHESJSONPATH,
    

    /// Selects JSONPath matching. See JsonPathMatcher for details on when the match succeeds. Succeeds when output does NOT match as specified. JSONPath is only supported for HTTP/HTTPS checks.
    ///
    /// "NOT_MATCHES_JSON_PATH"
    #[serde(rename="NOT_MATCHES_JSON_PATH")]
    NOTMATCHESJSONPATH,
}

impl AsRef<str> for ContentMatcherMatcherEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentMatcherMatcherEnum::CONTENTMATCHEROPTIONUNSPECIFIED => "CONTENT_MATCHER_OPTION_UNSPECIFIED",
            ContentMatcherMatcherEnum::CONTAINSSTRING => "CONTAINS_STRING",
            ContentMatcherMatcherEnum::NOTCONTAINSSTRING => "NOT_CONTAINS_STRING",
            ContentMatcherMatcherEnum::MATCHESREGEX => "MATCHES_REGEX",
            ContentMatcherMatcherEnum::NOTMATCHESREGEX => "NOT_MATCHES_REGEX",
            ContentMatcherMatcherEnum::MATCHESJSONPATH => "MATCHES_JSON_PATH",
            ContentMatcherMatcherEnum::NOTMATCHESJSONPATH => "NOT_MATCHES_JSON_PATH",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentMatcherMatcherEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_MATCHER_OPTION_UNSPECIFIED" => Ok(ContentMatcherMatcherEnum::CONTENTMATCHEROPTIONUNSPECIFIED),
           "CONTAINS_STRING" => Ok(ContentMatcherMatcherEnum::CONTAINSSTRING),
           "NOT_CONTAINS_STRING" => Ok(ContentMatcherMatcherEnum::NOTCONTAINSSTRING),
           "MATCHES_REGEX" => Ok(ContentMatcherMatcherEnum::MATCHESREGEX),
           "NOT_MATCHES_REGEX" => Ok(ContentMatcherMatcherEnum::NOTMATCHESREGEX),
           "MATCHES_JSON_PATH" => Ok(ContentMatcherMatcherEnum::MATCHESJSONPATH),
           "NOT_MATCHES_JSON_PATH" => Ok(ContentMatcherMatcherEnum::NOTMATCHESJSONPATH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentMatcherMatcherEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpCheckContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The content type header to use for the check. The following configurations result in errors: 1. Content type is specified in both the headers field and the content_type field. 2. Request method is GET and content_type is not TYPE_UNSPECIFIED 3. Request method is POST and content_type is TYPE_UNSPECIFIED. 4. Request method is POST and a "Content-Type" header is provided via headers field. The content_type field should be used instead.
pub enum HttpCheckContentTypeEnum {
    

    /// No content type specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// body is in URL-encoded form. Equivalent to setting the Content-Type to application/x-www-form-urlencoded in the HTTP request.
    ///
    /// "URL_ENCODED"
    #[serde(rename="URL_ENCODED")]
    URLENCODED,
    

    /// body is in custom_content_type form. Equivalent to setting the Content-Type to the contents of custom_content_type in the HTTP request.
    ///
    /// "USER_PROVIDED"
    #[serde(rename="USER_PROVIDED")]
    USERPROVIDED,
}

impl AsRef<str> for HttpCheckContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpCheckContentTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            HttpCheckContentTypeEnum::URLENCODED => "URL_ENCODED",
            HttpCheckContentTypeEnum::USERPROVIDED => "USER_PROVIDED",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpCheckContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(HttpCheckContentTypeEnum::TYPEUNSPECIFIED),
           "URL_ENCODED" => Ok(HttpCheckContentTypeEnum::URLENCODED),
           "USER_PROVIDED" => Ok(HttpCheckContentTypeEnum::USERPROVIDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpCheckContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpCheckRequestMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The HTTP request method to use for the check. If set to METHOD_UNSPECIFIED then request_method defaults to GET.
pub enum HttpCheckRequestMethodEnum {
    

    /// No request method specified.
    ///
    /// "METHOD_UNSPECIFIED"
    #[serde(rename="METHOD_UNSPECIFIED")]
    METHODUNSPECIFIED,
    

    /// GET request.
    ///
    /// "GET"
    #[serde(rename="GET")]
    GET,
    

    /// POST request.
    ///
    /// "POST"
    #[serde(rename="POST")]
    POST,
}

impl AsRef<str> for HttpCheckRequestMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpCheckRequestMethodEnum::METHODUNSPECIFIED => "METHOD_UNSPECIFIED",
            HttpCheckRequestMethodEnum::GET => "GET",
            HttpCheckRequestMethodEnum::POST => "POST",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpCheckRequestMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METHOD_UNSPECIFIED" => Ok(HttpCheckRequestMethodEnum::METHODUNSPECIFIED),
           "GET" => Ok(HttpCheckRequestMethodEnum::GET),
           "POST" => Ok(HttpCheckRequestMethodEnum::POST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpCheckRequestMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InternalCheckerStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current operational state of the internal checker.
pub enum InternalCheckerStateEnum {
    

    /// An internal checker should never be in the unspecified state.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// The checker is being created, provisioned, and configured. A checker in this state can be returned by ListInternalCheckers or GetInternalChecker, as well as by examining the long running Operation (https://cloud.google.com/apis/design/design_patterns#long_running_operations) that created it.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The checker is running and available for use. A checker in this state can be returned by ListInternalCheckers or GetInternalChecker as well as by examining the long running Operation (https://cloud.google.com/apis/design/design_patterns#long_running_operations) that created it. If a checker is being torn down, it is neither visible nor usable, so there is no "deleting" or "down" state.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
}

impl AsRef<str> for InternalCheckerStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InternalCheckerStateEnum::UNSPECIFIED => "UNSPECIFIED",
            InternalCheckerStateEnum::CREATING => "CREATING",
            InternalCheckerStateEnum::RUNNING => "RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for InternalCheckerStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(InternalCheckerStateEnum::UNSPECIFIED),
           "CREATING" => Ok(InternalCheckerStateEnum::CREATING),
           "RUNNING" => Ok(InternalCheckerStateEnum::RUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InternalCheckerStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JsonPathMatcherJsonMatcherEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of JSONPath match that will be applied to the JSON output (ContentMatcher.content)
pub enum JsonPathMatcherJsonMatcherEnum {
    

    /// No JSONPath matcher type specified (not valid).
    ///
    /// "JSON_PATH_MATCHER_OPTION_UNSPECIFIED"
    #[serde(rename="JSON_PATH_MATCHER_OPTION_UNSPECIFIED")]
    JSONPATHMATCHEROPTIONUNSPECIFIED,
    

    /// Selects 'exact string' matching. The match succeeds if the content at the json_path within the output is exactly the same as the content string.
    ///
    /// "EXACT_MATCH"
    #[serde(rename="EXACT_MATCH")]
    EXACTMATCH,
    

    /// Selects regular-expression matching. The match succeeds if the content at the json_path within the output matches the regular expression specified in the content string.
    ///
    /// "REGEX_MATCH"
    #[serde(rename="REGEX_MATCH")]
    REGEXMATCH,
}

impl AsRef<str> for JsonPathMatcherJsonMatcherEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JsonPathMatcherJsonMatcherEnum::JSONPATHMATCHEROPTIONUNSPECIFIED => "JSON_PATH_MATCHER_OPTION_UNSPECIFIED",
            JsonPathMatcherJsonMatcherEnum::EXACTMATCH => "EXACT_MATCH",
            JsonPathMatcherJsonMatcherEnum::REGEXMATCH => "REGEX_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for JsonPathMatcherJsonMatcherEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JSON_PATH_MATCHER_OPTION_UNSPECIFIED" => Ok(JsonPathMatcherJsonMatcherEnum::JSONPATHMATCHEROPTIONUNSPECIFIED),
           "EXACT_MATCH" => Ok(JsonPathMatcherJsonMatcherEnum::EXACTMATCH),
           "REGEX_MATCH" => Ok(JsonPathMatcherJsonMatcherEnum::REGEXMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JsonPathMatcherJsonMatcherEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LabelDescriptorValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of data that can be assigned to the label.
pub enum LabelDescriptorValueTypeEnum {
    

    /// A variable-length string, not to exceed 1,024 characters. This is the default value type.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Boolean; true or false.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// A 64-bit signed integer.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
}

impl AsRef<str> for LabelDescriptorValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LabelDescriptorValueTypeEnum::STRING => "STRING",
            LabelDescriptorValueTypeEnum::BOOL => "BOOL",
            LabelDescriptorValueTypeEnum::INT64 => "INT64",
        }
    }
}

impl std::convert::TryFrom< &str> for LabelDescriptorValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STRING" => Ok(LabelDescriptorValueTypeEnum::STRING),
           "BOOL" => Ok(LabelDescriptorValueTypeEnum::BOOL),
           "INT64" => Ok(LabelDescriptorValueTypeEnum::INT64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LabelDescriptorValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricDescriptorLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The launch stage of the metric definition.
pub enum MetricDescriptorLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for MetricDescriptorLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricDescriptorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            MetricDescriptorLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            MetricDescriptorLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            MetricDescriptorLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            MetricDescriptorLaunchStageEnum::ALPHA => "ALPHA",
            MetricDescriptorLaunchStageEnum::BETA => "BETA",
            MetricDescriptorLaunchStageEnum::GA => "GA",
            MetricDescriptorLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricDescriptorLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(MetricDescriptorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(MetricDescriptorLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(MetricDescriptorLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(MetricDescriptorLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(MetricDescriptorLaunchStageEnum::ALPHA),
           "BETA" => Ok(MetricDescriptorLaunchStageEnum::BETA),
           "GA" => Ok(MetricDescriptorLaunchStageEnum::GA),
           "DEPRECATED" => Ok(MetricDescriptorLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricDescriptorLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricDescriptorMetricKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported.
pub enum MetricDescriptorMetricKindEnum {
    

    /// Do not use this default value.
    ///
    /// "METRIC_KIND_UNSPECIFIED"
    #[serde(rename="METRIC_KIND_UNSPECIFIED")]
    METRICKINDUNSPECIFIED,
    

    /// An instantaneous measurement of a value.
    ///
    /// "GAUGE"
    #[serde(rename="GAUGE")]
    GAUGE,
    

    /// The change in a value during a time interval.
    ///
    /// "DELTA"
    #[serde(rename="DELTA")]
    DELTA,
    

    /// A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points.
    ///
    /// "CUMULATIVE"
    #[serde(rename="CUMULATIVE")]
    CUMULATIVE,
}

impl AsRef<str> for MetricDescriptorMetricKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricDescriptorMetricKindEnum::METRICKINDUNSPECIFIED => "METRIC_KIND_UNSPECIFIED",
            MetricDescriptorMetricKindEnum::GAUGE => "GAUGE",
            MetricDescriptorMetricKindEnum::DELTA => "DELTA",
            MetricDescriptorMetricKindEnum::CUMULATIVE => "CUMULATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricDescriptorMetricKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_KIND_UNSPECIFIED" => Ok(MetricDescriptorMetricKindEnum::METRICKINDUNSPECIFIED),
           "GAUGE" => Ok(MetricDescriptorMetricKindEnum::GAUGE),
           "DELTA" => Ok(MetricDescriptorMetricKindEnum::DELTA),
           "CUMULATIVE" => Ok(MetricDescriptorMetricKindEnum::CUMULATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricDescriptorMetricKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricDescriptorValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported.
pub enum MetricDescriptorValueTypeEnum {
    

    /// Do not use this default value.
    ///
    /// "VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="VALUE_TYPE_UNSPECIFIED")]
    VALUETYPEUNSPECIFIED,
    

    /// The value is a boolean. This value type can be used only if the metric kind is GAUGE.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The value is a signed 64-bit integer.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// The value is a double precision floating point number.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// The value is a text string. This value type can be used only if the metric kind is GAUGE.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// The value is a Distribution.
    ///
    /// "DISTRIBUTION"
    #[serde(rename="DISTRIBUTION")]
    DISTRIBUTION,
    

    /// The value is money.
    ///
    /// "MONEY"
    #[serde(rename="MONEY")]
    MONEY,
}

impl AsRef<str> for MetricDescriptorValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricDescriptorValueTypeEnum::VALUETYPEUNSPECIFIED => "VALUE_TYPE_UNSPECIFIED",
            MetricDescriptorValueTypeEnum::BOOL => "BOOL",
            MetricDescriptorValueTypeEnum::INT64 => "INT64",
            MetricDescriptorValueTypeEnum::DOUBLE => "DOUBLE",
            MetricDescriptorValueTypeEnum::STRING => "STRING",
            MetricDescriptorValueTypeEnum::DISTRIBUTION => "DISTRIBUTION",
            MetricDescriptorValueTypeEnum::MONEY => "MONEY",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricDescriptorValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_TYPE_UNSPECIFIED" => Ok(MetricDescriptorValueTypeEnum::VALUETYPEUNSPECIFIED),
           "BOOL" => Ok(MetricDescriptorValueTypeEnum::BOOL),
           "INT64" => Ok(MetricDescriptorValueTypeEnum::INT64),
           "DOUBLE" => Ok(MetricDescriptorValueTypeEnum::DOUBLE),
           "STRING" => Ok(MetricDescriptorValueTypeEnum::STRING),
           "DISTRIBUTION" => Ok(MetricDescriptorValueTypeEnum::DISTRIBUTION),
           "MONEY" => Ok(MetricDescriptorValueTypeEnum::MONEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricDescriptorValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricDescriptorMetadataLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. Must use the MetricDescriptor.launch_stage instead.
pub enum MetricDescriptorMetadataLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for MetricDescriptorMetadataLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricDescriptorMetadataLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            MetricDescriptorMetadataLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            MetricDescriptorMetadataLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            MetricDescriptorMetadataLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            MetricDescriptorMetadataLaunchStageEnum::ALPHA => "ALPHA",
            MetricDescriptorMetadataLaunchStageEnum::BETA => "BETA",
            MetricDescriptorMetadataLaunchStageEnum::GA => "GA",
            MetricDescriptorMetadataLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricDescriptorMetadataLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(MetricDescriptorMetadataLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(MetricDescriptorMetadataLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(MetricDescriptorMetadataLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(MetricDescriptorMetadataLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(MetricDescriptorMetadataLaunchStageEnum::ALPHA),
           "BETA" => Ok(MetricDescriptorMetadataLaunchStageEnum::BETA),
           "GA" => Ok(MetricDescriptorMetadataLaunchStageEnum::GA),
           "DEPRECATED" => Ok(MetricDescriptorMetadataLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricDescriptorMetadataLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricThresholdComparisonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The comparison to apply between the time series (indicated by filter and aggregation) and the threshold (indicated by threshold_value). The comparison is applied on each time series, with the time series on the left-hand side and the threshold on the right-hand side.Only COMPARISON_LT and COMPARISON_GT are supported currently.
pub enum MetricThresholdComparisonEnum {
    

    /// No ordering relationship is specified.
    ///
    /// "COMPARISON_UNSPECIFIED"
    #[serde(rename="COMPARISON_UNSPECIFIED")]
    COMPARISONUNSPECIFIED,
    

    /// True if the left argument is greater than the right argument.
    ///
    /// "COMPARISON_GT"
    #[serde(rename="COMPARISON_GT")]
    COMPARISONGT,
    

    /// True if the left argument is greater than or equal to the right argument.
    ///
    /// "COMPARISON_GE"
    #[serde(rename="COMPARISON_GE")]
    COMPARISONGE,
    

    /// True if the left argument is less than the right argument.
    ///
    /// "COMPARISON_LT"
    #[serde(rename="COMPARISON_LT")]
    COMPARISONLT,
    

    /// True if the left argument is less than or equal to the right argument.
    ///
    /// "COMPARISON_LE"
    #[serde(rename="COMPARISON_LE")]
    COMPARISONLE,
    

    /// True if the left argument is equal to the right argument.
    ///
    /// "COMPARISON_EQ"
    #[serde(rename="COMPARISON_EQ")]
    COMPARISONEQ,
    

    /// True if the left argument is not equal to the right argument.
    ///
    /// "COMPARISON_NE"
    #[serde(rename="COMPARISON_NE")]
    COMPARISONNE,
}

impl AsRef<str> for MetricThresholdComparisonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricThresholdComparisonEnum::COMPARISONUNSPECIFIED => "COMPARISON_UNSPECIFIED",
            MetricThresholdComparisonEnum::COMPARISONGT => "COMPARISON_GT",
            MetricThresholdComparisonEnum::COMPARISONGE => "COMPARISON_GE",
            MetricThresholdComparisonEnum::COMPARISONLT => "COMPARISON_LT",
            MetricThresholdComparisonEnum::COMPARISONLE => "COMPARISON_LE",
            MetricThresholdComparisonEnum::COMPARISONEQ => "COMPARISON_EQ",
            MetricThresholdComparisonEnum::COMPARISONNE => "COMPARISON_NE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricThresholdComparisonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPARISON_UNSPECIFIED" => Ok(MetricThresholdComparisonEnum::COMPARISONUNSPECIFIED),
           "COMPARISON_GT" => Ok(MetricThresholdComparisonEnum::COMPARISONGT),
           "COMPARISON_GE" => Ok(MetricThresholdComparisonEnum::COMPARISONGE),
           "COMPARISON_LT" => Ok(MetricThresholdComparisonEnum::COMPARISONLT),
           "COMPARISON_LE" => Ok(MetricThresholdComparisonEnum::COMPARISONLE),
           "COMPARISON_EQ" => Ok(MetricThresholdComparisonEnum::COMPARISONEQ),
           "COMPARISON_NE" => Ok(MetricThresholdComparisonEnum::COMPARISONNE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricThresholdComparisonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricThresholdEvaluationMissingDataEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A condition control that determines how metric-threshold conditions are evaluated when data stops arriving. To use this control, the value of the duration field must be greater than or equal to 60 seconds.
pub enum MetricThresholdEvaluationMissingDataEnum {
    

    /// An unspecified evaluation missing data option. Equivalent to EVALUATION_MISSING_DATA_NO_OP.
    ///
    /// "EVALUATION_MISSING_DATA_UNSPECIFIED"
    #[serde(rename="EVALUATION_MISSING_DATA_UNSPECIFIED")]
    EVALUATIONMISSINGDATAUNSPECIFIED,
    

    /// If there is no data to evaluate the condition, then evaluate the condition as false.
    ///
    /// "EVALUATION_MISSING_DATA_INACTIVE"
    #[serde(rename="EVALUATION_MISSING_DATA_INACTIVE")]
    EVALUATIONMISSINGDATAINACTIVE,
    

    /// If there is no data to evaluate the condition, then evaluate the condition as true.
    ///
    /// "EVALUATION_MISSING_DATA_ACTIVE"
    #[serde(rename="EVALUATION_MISSING_DATA_ACTIVE")]
    EVALUATIONMISSINGDATAACTIVE,
    

    /// Do not evaluate the condition to any value if there is no data.
    ///
    /// "EVALUATION_MISSING_DATA_NO_OP"
    #[serde(rename="EVALUATION_MISSING_DATA_NO_OP")]
    EVALUATIONMISSINGDATANOOP,
}

impl AsRef<str> for MetricThresholdEvaluationMissingDataEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATAUNSPECIFIED => "EVALUATION_MISSING_DATA_UNSPECIFIED",
            MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATAINACTIVE => "EVALUATION_MISSING_DATA_INACTIVE",
            MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATAACTIVE => "EVALUATION_MISSING_DATA_ACTIVE",
            MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATANOOP => "EVALUATION_MISSING_DATA_NO_OP",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricThresholdEvaluationMissingDataEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVALUATION_MISSING_DATA_UNSPECIFIED" => Ok(MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATAUNSPECIFIED),
           "EVALUATION_MISSING_DATA_INACTIVE" => Ok(MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATAINACTIVE),
           "EVALUATION_MISSING_DATA_ACTIVE" => Ok(MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATAACTIVE),
           "EVALUATION_MISSING_DATA_NO_OP" => Ok(MetricThresholdEvaluationMissingDataEnum::EVALUATIONMISSINGDATANOOP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricThresholdEvaluationMissingDataEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MonitoredResourceDescriptorLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The launch stage of the monitored resource definition.
pub enum MonitoredResourceDescriptorLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for MonitoredResourceDescriptorLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MonitoredResourceDescriptorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            MonitoredResourceDescriptorLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            MonitoredResourceDescriptorLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            MonitoredResourceDescriptorLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            MonitoredResourceDescriptorLaunchStageEnum::ALPHA => "ALPHA",
            MonitoredResourceDescriptorLaunchStageEnum::BETA => "BETA",
            MonitoredResourceDescriptorLaunchStageEnum::GA => "GA",
            MonitoredResourceDescriptorLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for MonitoredResourceDescriptorLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(MonitoredResourceDescriptorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(MonitoredResourceDescriptorLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(MonitoredResourceDescriptorLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(MonitoredResourceDescriptorLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(MonitoredResourceDescriptorLaunchStageEnum::ALPHA),
           "BETA" => Ok(MonitoredResourceDescriptorLaunchStageEnum::BETA),
           "GA" => Ok(MonitoredResourceDescriptorLaunchStageEnum::GA),
           "DEPRECATED" => Ok(MonitoredResourceDescriptorLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MonitoredResourceDescriptorLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MonitoringQueryLanguageConditionEvaluationMissingDataEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A condition control that determines how metric-threshold conditions are evaluated when data stops arriving.
pub enum MonitoringQueryLanguageConditionEvaluationMissingDataEnum {
    

    /// An unspecified evaluation missing data option. Equivalent to EVALUATION_MISSING_DATA_NO_OP.
    ///
    /// "EVALUATION_MISSING_DATA_UNSPECIFIED"
    #[serde(rename="EVALUATION_MISSING_DATA_UNSPECIFIED")]
    EVALUATIONMISSINGDATAUNSPECIFIED,
    

    /// If there is no data to evaluate the condition, then evaluate the condition as false.
    ///
    /// "EVALUATION_MISSING_DATA_INACTIVE"
    #[serde(rename="EVALUATION_MISSING_DATA_INACTIVE")]
    EVALUATIONMISSINGDATAINACTIVE,
    

    /// If there is no data to evaluate the condition, then evaluate the condition as true.
    ///
    /// "EVALUATION_MISSING_DATA_ACTIVE"
    #[serde(rename="EVALUATION_MISSING_DATA_ACTIVE")]
    EVALUATIONMISSINGDATAACTIVE,
    

    /// Do not evaluate the condition to any value if there is no data.
    ///
    /// "EVALUATION_MISSING_DATA_NO_OP"
    #[serde(rename="EVALUATION_MISSING_DATA_NO_OP")]
    EVALUATIONMISSINGDATANOOP,
}

impl AsRef<str> for MonitoringQueryLanguageConditionEvaluationMissingDataEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATAUNSPECIFIED => "EVALUATION_MISSING_DATA_UNSPECIFIED",
            MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATAINACTIVE => "EVALUATION_MISSING_DATA_INACTIVE",
            MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATAACTIVE => "EVALUATION_MISSING_DATA_ACTIVE",
            MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATANOOP => "EVALUATION_MISSING_DATA_NO_OP",
        }
    }
}

impl std::convert::TryFrom< &str> for MonitoringQueryLanguageConditionEvaluationMissingDataEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVALUATION_MISSING_DATA_UNSPECIFIED" => Ok(MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATAUNSPECIFIED),
           "EVALUATION_MISSING_DATA_INACTIVE" => Ok(MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATAINACTIVE),
           "EVALUATION_MISSING_DATA_ACTIVE" => Ok(MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATAACTIVE),
           "EVALUATION_MISSING_DATA_NO_OP" => Ok(MonitoringQueryLanguageConditionEvaluationMissingDataEnum::EVALUATIONMISSINGDATANOOP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MonitoringQueryLanguageConditionEvaluationMissingDataEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationChannelVerificationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel.
pub enum NotificationChannelVerificationStatusEnum {
    

    /// Sentinel value used to indicate that the state is unknown, omitted, or is not applicable (as in the case of channels that neither support nor require verification in order to function).
    ///
    /// "VERIFICATION_STATUS_UNSPECIFIED"
    #[serde(rename="VERIFICATION_STATUS_UNSPECIFIED")]
    VERIFICATIONSTATUSUNSPECIFIED,
    

    /// The channel has yet to be verified and requires verification to function. Note that this state also applies to the case where the verification process has been initiated by sending a verification code but where the verification code has not been submitted to complete the process.
    ///
    /// "UNVERIFIED"
    #[serde(rename="UNVERIFIED")]
    UNVERIFIED,
    

    /// It has been proven that notifications can be received on this notification channel and that someone on the project has access to messages that are delivered to that channel.
    ///
    /// "VERIFIED"
    #[serde(rename="VERIFIED")]
    VERIFIED,
}

impl AsRef<str> for NotificationChannelVerificationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationChannelVerificationStatusEnum::VERIFICATIONSTATUSUNSPECIFIED => "VERIFICATION_STATUS_UNSPECIFIED",
            NotificationChannelVerificationStatusEnum::UNVERIFIED => "UNVERIFIED",
            NotificationChannelVerificationStatusEnum::VERIFIED => "VERIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationChannelVerificationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERIFICATION_STATUS_UNSPECIFIED" => Ok(NotificationChannelVerificationStatusEnum::VERIFICATIONSTATUSUNSPECIFIED),
           "UNVERIFIED" => Ok(NotificationChannelVerificationStatusEnum::UNVERIFIED),
           "VERIFIED" => Ok(NotificationChannelVerificationStatusEnum::VERIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationChannelVerificationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationChannelDescriptorLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The product launch stage for channels of this type.
pub enum NotificationChannelDescriptorLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our Terms of Service (https://cloud.google.com/terms/) and the Google Cloud Platform Subject to the Deprecation Policy (https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for NotificationChannelDescriptorLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationChannelDescriptorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            NotificationChannelDescriptorLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            NotificationChannelDescriptorLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            NotificationChannelDescriptorLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            NotificationChannelDescriptorLaunchStageEnum::ALPHA => "ALPHA",
            NotificationChannelDescriptorLaunchStageEnum::BETA => "BETA",
            NotificationChannelDescriptorLaunchStageEnum::GA => "GA",
            NotificationChannelDescriptorLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationChannelDescriptorLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(NotificationChannelDescriptorLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(NotificationChannelDescriptorLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(NotificationChannelDescriptorLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(NotificationChannelDescriptorLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(NotificationChannelDescriptorLaunchStageEnum::ALPHA),
           "BETA" => Ok(NotificationChannelDescriptorLaunchStageEnum::BETA),
           "GA" => Ok(NotificationChannelDescriptorLaunchStageEnum::GA),
           "DEPRECATED" => Ok(NotificationChannelDescriptorLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationChannelDescriptorLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationChannelDescriptorSupportedTiersEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The tiers that support this notification channel; the project service tier must be one of the supported_tiers.
pub enum NotificationChannelDescriptorSupportedTiersEnum {
    

    /// An invalid sentinel value, used to indicate that a tier has not been provided explicitly.
    ///
    /// "SERVICE_TIER_UNSPECIFIED"
    #[serde(rename="SERVICE_TIER_UNSPECIFIED")]
    SERVICETIERUNSPECIFIED,
    

    /// The Cloud Monitoring Basic tier, a free tier of service that provides basic features, a moderate allotment of logs, and access to built-in metrics. A number of features are not available in this tier. For more details, see the service tiers documentation (https://cloud.google.com/monitoring/workspaces/tiers).
    ///
    /// "SERVICE_TIER_BASIC"
    #[serde(rename="SERVICE_TIER_BASIC")]
    SERVICETIERBASIC,
    

    /// The Cloud Monitoring Premium tier, a higher, more expensive tier of service that provides access to all Cloud Monitoring features, lets you use Cloud Monitoring with AWS accounts, and has a larger allotments for logs and metrics. For more details, see the service tiers documentation (https://cloud.google.com/monitoring/workspaces/tiers).
    ///
    /// "SERVICE_TIER_PREMIUM"
    #[serde(rename="SERVICE_TIER_PREMIUM")]
    SERVICETIERPREMIUM,
}

impl AsRef<str> for NotificationChannelDescriptorSupportedTiersEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationChannelDescriptorSupportedTiersEnum::SERVICETIERUNSPECIFIED => "SERVICE_TIER_UNSPECIFIED",
            NotificationChannelDescriptorSupportedTiersEnum::SERVICETIERBASIC => "SERVICE_TIER_BASIC",
            NotificationChannelDescriptorSupportedTiersEnum::SERVICETIERPREMIUM => "SERVICE_TIER_PREMIUM",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationChannelDescriptorSupportedTiersEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_TIER_UNSPECIFIED" => Ok(NotificationChannelDescriptorSupportedTiersEnum::SERVICETIERUNSPECIFIED),
           "SERVICE_TIER_BASIC" => Ok(NotificationChannelDescriptorSupportedTiersEnum::SERVICETIERBASIC),
           "SERVICE_TIER_PREMIUM" => Ok(NotificationChannelDescriptorSupportedTiersEnum::SERVICETIERPREMIUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationChannelDescriptorSupportedTiersEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceGroupResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The resource type of the group members.
pub enum ResourceGroupResourceTypeEnum {
    

    /// Default value (not valid).
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// A group of instances from Google Cloud Platform (GCP) or Amazon Web Services (AWS).
    ///
    /// "INSTANCE"
    #[serde(rename="INSTANCE")]
    INSTANCE,
    

    /// A group of Amazon ELB load balancers.
    ///
    /// "AWS_ELB_LOAD_BALANCER"
    #[serde(rename="AWS_ELB_LOAD_BALANCER")]
    AWSELBLOADBALANCER,
}

impl AsRef<str> for ResourceGroupResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceGroupResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            ResourceGroupResourceTypeEnum::INSTANCE => "INSTANCE",
            ResourceGroupResourceTypeEnum::AWSELBLOADBALANCER => "AWS_ELB_LOAD_BALANCER",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceGroupResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(ResourceGroupResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "INSTANCE" => Ok(ResourceGroupResourceTypeEnum::INSTANCE),
           "AWS_ELB_LOAD_BALANCER" => Ok(ResourceGroupResourceTypeEnum::AWSELBLOADBALANCER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceGroupResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResponseStatusCodeStatusClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A class of status codes to accept.
pub enum ResponseStatusCodeStatusClassEnum {
    

    /// Default value that matches no status codes.
    ///
    /// "STATUS_CLASS_UNSPECIFIED"
    #[serde(rename="STATUS_CLASS_UNSPECIFIED")]
    STATUSCLASSUNSPECIFIED,
    

    /// The class of status codes between 100 and 199.
    ///
    /// "STATUS_CLASS_1XX"
    #[serde(rename="STATUS_CLASS_1XX")]
    STATUSCLASS1XX,
    

    /// The class of status codes between 200 and 299.
    ///
    /// "STATUS_CLASS_2XX"
    #[serde(rename="STATUS_CLASS_2XX")]
    STATUSCLASS2XX,
    

    /// The class of status codes between 300 and 399.
    ///
    /// "STATUS_CLASS_3XX"
    #[serde(rename="STATUS_CLASS_3XX")]
    STATUSCLASS3XX,
    

    /// The class of status codes between 400 and 499.
    ///
    /// "STATUS_CLASS_4XX"
    #[serde(rename="STATUS_CLASS_4XX")]
    STATUSCLASS4XX,
    

    /// The class of status codes between 500 and 599.
    ///
    /// "STATUS_CLASS_5XX"
    #[serde(rename="STATUS_CLASS_5XX")]
    STATUSCLASS5XX,
    

    /// The class of all status codes.
    ///
    /// "STATUS_CLASS_ANY"
    #[serde(rename="STATUS_CLASS_ANY")]
    STATUSCLASSANY,
}

impl AsRef<str> for ResponseStatusCodeStatusClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResponseStatusCodeStatusClassEnum::STATUSCLASSUNSPECIFIED => "STATUS_CLASS_UNSPECIFIED",
            ResponseStatusCodeStatusClassEnum::STATUSCLASS1XX => "STATUS_CLASS_1XX",
            ResponseStatusCodeStatusClassEnum::STATUSCLASS2XX => "STATUS_CLASS_2XX",
            ResponseStatusCodeStatusClassEnum::STATUSCLASS3XX => "STATUS_CLASS_3XX",
            ResponseStatusCodeStatusClassEnum::STATUSCLASS4XX => "STATUS_CLASS_4XX",
            ResponseStatusCodeStatusClassEnum::STATUSCLASS5XX => "STATUS_CLASS_5XX",
            ResponseStatusCodeStatusClassEnum::STATUSCLASSANY => "STATUS_CLASS_ANY",
        }
    }
}

impl std::convert::TryFrom< &str> for ResponseStatusCodeStatusClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_CLASS_UNSPECIFIED" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASSUNSPECIFIED),
           "STATUS_CLASS_1XX" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASS1XX),
           "STATUS_CLASS_2XX" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASS2XX),
           "STATUS_CLASS_3XX" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASS3XX),
           "STATUS_CLASS_4XX" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASS4XX),
           "STATUS_CLASS_5XX" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASS5XX),
           "STATUS_CLASS_ANY" => Ok(ResponseStatusCodeStatusClassEnum::STATUSCLASSANY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResponseStatusCodeStatusClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAgentAuthenticationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of authentication.
pub enum ServiceAgentAuthenticationTypeEnum {
    

    /// Default value, will result in OIDC Authentication.
    ///
    /// "SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED"
    #[serde(rename="SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED")]
    SERVICEAGENTAUTHENTICATIONTYPEUNSPECIFIED,
    

    /// OIDC Authentication
    ///
    /// "OIDC_TOKEN"
    #[serde(rename="OIDC_TOKEN")]
    OIDCTOKEN,
}

impl AsRef<str> for ServiceAgentAuthenticationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAgentAuthenticationTypeEnum::SERVICEAGENTAUTHENTICATIONTYPEUNSPECIFIED => "SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED",
            ServiceAgentAuthenticationTypeEnum::OIDCTOKEN => "OIDC_TOKEN",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAgentAuthenticationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED" => Ok(ServiceAgentAuthenticationTypeEnum::SERVICEAGENTAUTHENTICATIONTYPEUNSPECIFIED),
           "OIDC_TOKEN" => Ok(ServiceAgentAuthenticationTypeEnum::OIDCTOKEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAgentAuthenticationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceLevelObjectiveCalendarPeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A calendar period, semantically "since the start of the current ". At this time, only DAY, WEEK, FORTNIGHT, and MONTH are supported.
pub enum ServiceLevelObjectiveCalendarPeriodEnum {
    

    /// Undefined period, raises an error.
    ///
    /// "CALENDAR_PERIOD_UNSPECIFIED"
    #[serde(rename="CALENDAR_PERIOD_UNSPECIFIED")]
    CALENDARPERIODUNSPECIFIED,
    

    /// A day.
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// A week. Weeks begin on Monday, following ISO 8601 (https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// A fortnight. The first calendar fortnight of the year begins at the start of week 1 according to ISO 8601 (https://en.wikipedia.org/wiki/ISO_week_date).
    ///
    /// "FORTNIGHT"
    #[serde(rename="FORTNIGHT")]
    FORTNIGHT,
    

    /// A month.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// A quarter. Quarters start on dates 1-Jan, 1-Apr, 1-Jul, and 1-Oct of each year.
    ///
    /// "QUARTER"
    #[serde(rename="QUARTER")]
    QUARTER,
    

    /// A half-year. Half-years start on dates 1-Jan and 1-Jul.
    ///
    /// "HALF"
    #[serde(rename="HALF")]
    HALF,
    

    /// A year.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
}

impl AsRef<str> for ServiceLevelObjectiveCalendarPeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceLevelObjectiveCalendarPeriodEnum::CALENDARPERIODUNSPECIFIED => "CALENDAR_PERIOD_UNSPECIFIED",
            ServiceLevelObjectiveCalendarPeriodEnum::DAY => "DAY",
            ServiceLevelObjectiveCalendarPeriodEnum::WEEK => "WEEK",
            ServiceLevelObjectiveCalendarPeriodEnum::FORTNIGHT => "FORTNIGHT",
            ServiceLevelObjectiveCalendarPeriodEnum::MONTH => "MONTH",
            ServiceLevelObjectiveCalendarPeriodEnum::QUARTER => "QUARTER",
            ServiceLevelObjectiveCalendarPeriodEnum::HALF => "HALF",
            ServiceLevelObjectiveCalendarPeriodEnum::YEAR => "YEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceLevelObjectiveCalendarPeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CALENDAR_PERIOD_UNSPECIFIED" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::CALENDARPERIODUNSPECIFIED),
           "DAY" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::DAY),
           "WEEK" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::WEEK),
           "FORTNIGHT" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::FORTNIGHT),
           "MONTH" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::MONTH),
           "QUARTER" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::QUARTER),
           "HALF" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::HALF),
           "YEAR" => Ok(ServiceLevelObjectiveCalendarPeriodEnum::YEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceLevelObjectiveCalendarPeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimeSeryMetricKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The metric kind of the time series. When listing time series, this metric kind might be different from the metric kind of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the metric kind of the associated metric. If the associated metric's descriptor must be auto-created, then this field specifies the metric kind of the new descriptor and must be either GAUGE (the default) or CUMULATIVE.
pub enum TimeSeryMetricKindEnum {
    

    /// Do not use this default value.
    ///
    /// "METRIC_KIND_UNSPECIFIED"
    #[serde(rename="METRIC_KIND_UNSPECIFIED")]
    METRICKINDUNSPECIFIED,
    

    /// An instantaneous measurement of a value.
    ///
    /// "GAUGE"
    #[serde(rename="GAUGE")]
    GAUGE,
    

    /// The change in a value during a time interval.
    ///
    /// "DELTA"
    #[serde(rename="DELTA")]
    DELTA,
    

    /// A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points.
    ///
    /// "CUMULATIVE"
    #[serde(rename="CUMULATIVE")]
    CUMULATIVE,
}

impl AsRef<str> for TimeSeryMetricKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimeSeryMetricKindEnum::METRICKINDUNSPECIFIED => "METRIC_KIND_UNSPECIFIED",
            TimeSeryMetricKindEnum::GAUGE => "GAUGE",
            TimeSeryMetricKindEnum::DELTA => "DELTA",
            TimeSeryMetricKindEnum::CUMULATIVE => "CUMULATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for TimeSeryMetricKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_KIND_UNSPECIFIED" => Ok(TimeSeryMetricKindEnum::METRICKINDUNSPECIFIED),
           "GAUGE" => Ok(TimeSeryMetricKindEnum::GAUGE),
           "DELTA" => Ok(TimeSeryMetricKindEnum::DELTA),
           "CUMULATIVE" => Ok(TimeSeryMetricKindEnum::CUMULATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimeSeryMetricKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimeSeryValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value type of the time series. When listing time series, this value type might be different from the value type of the associated metric if this time series is an alignment or reduction of other time series.When creating a time series, this field is optional. If present, it must be the same as the type of the data in the points field.
pub enum TimeSeryValueTypeEnum {
    

    /// Do not use this default value.
    ///
    /// "VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="VALUE_TYPE_UNSPECIFIED")]
    VALUETYPEUNSPECIFIED,
    

    /// The value is a boolean. This value type can be used only if the metric kind is GAUGE.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The value is a signed 64-bit integer.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// The value is a double precision floating point number.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// The value is a text string. This value type can be used only if the metric kind is GAUGE.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// The value is a Distribution.
    ///
    /// "DISTRIBUTION"
    #[serde(rename="DISTRIBUTION")]
    DISTRIBUTION,
    

    /// The value is money.
    ///
    /// "MONEY"
    #[serde(rename="MONEY")]
    MONEY,
}

impl AsRef<str> for TimeSeryValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimeSeryValueTypeEnum::VALUETYPEUNSPECIFIED => "VALUE_TYPE_UNSPECIFIED",
            TimeSeryValueTypeEnum::BOOL => "BOOL",
            TimeSeryValueTypeEnum::INT64 => "INT64",
            TimeSeryValueTypeEnum::DOUBLE => "DOUBLE",
            TimeSeryValueTypeEnum::STRING => "STRING",
            TimeSeryValueTypeEnum::DISTRIBUTION => "DISTRIBUTION",
            TimeSeryValueTypeEnum::MONEY => "MONEY",
        }
    }
}

impl std::convert::TryFrom< &str> for TimeSeryValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_TYPE_UNSPECIFIED" => Ok(TimeSeryValueTypeEnum::VALUETYPEUNSPECIFIED),
           "BOOL" => Ok(TimeSeryValueTypeEnum::BOOL),
           "INT64" => Ok(TimeSeryValueTypeEnum::INT64),
           "DOUBLE" => Ok(TimeSeryValueTypeEnum::DOUBLE),
           "STRING" => Ok(TimeSeryValueTypeEnum::STRING),
           "DISTRIBUTION" => Ok(TimeSeryValueTypeEnum::DISTRIBUTION),
           "MONEY" => Ok(TimeSeryValueTypeEnum::MONEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimeSeryValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UptimeCheckConfigCheckerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of checkers to use to execute the Uptime check.
pub enum UptimeCheckConfigCheckerTypeEnum {
    

    /// The default checker type. Currently converted to STATIC_IP_CHECKERS on creation, the default conversion behavior may change in the future.
    ///
    /// "CHECKER_TYPE_UNSPECIFIED"
    #[serde(rename="CHECKER_TYPE_UNSPECIFIED")]
    CHECKERTYPEUNSPECIFIED,
    

    /// STATIC_IP_CHECKERS are used for uptime checks that perform egress across the public internet. STATIC_IP_CHECKERS use the static IP addresses returned by ListUptimeCheckIps.
    ///
    /// "STATIC_IP_CHECKERS"
    #[serde(rename="STATIC_IP_CHECKERS")]
    STATICIPCHECKERS,
    

    /// VPC_CHECKERS are used for uptime checks that perform egress using Service Directory and private network access. When using VPC_CHECKERS, the monitored resource type must be servicedirectory_service.
    ///
    /// "VPC_CHECKERS"
    #[serde(rename="VPC_CHECKERS")]
    VPCCHECKERS,
}

impl AsRef<str> for UptimeCheckConfigCheckerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UptimeCheckConfigCheckerTypeEnum::CHECKERTYPEUNSPECIFIED => "CHECKER_TYPE_UNSPECIFIED",
            UptimeCheckConfigCheckerTypeEnum::STATICIPCHECKERS => "STATIC_IP_CHECKERS",
            UptimeCheckConfigCheckerTypeEnum::VPCCHECKERS => "VPC_CHECKERS",
        }
    }
}

impl std::convert::TryFrom< &str> for UptimeCheckConfigCheckerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHECKER_TYPE_UNSPECIFIED" => Ok(UptimeCheckConfigCheckerTypeEnum::CHECKERTYPEUNSPECIFIED),
           "STATIC_IP_CHECKERS" => Ok(UptimeCheckConfigCheckerTypeEnum::STATICIPCHECKERS),
           "VPC_CHECKERS" => Ok(UptimeCheckConfigCheckerTypeEnum::VPCCHECKERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UptimeCheckConfigCheckerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UptimeCheckConfigSelectedRegionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions must be provided to include a minimum of 3 locations. Not specifying this field will result in Uptime checks running from all available regions.
pub enum UptimeCheckConfigSelectedRegionsEnum {
    

    /// Default value if no region is specified. Will result in Uptime checks running from all regions.
    ///
    /// "REGION_UNSPECIFIED"
    #[serde(rename="REGION_UNSPECIFIED")]
    REGIONUNSPECIFIED,
    

    /// Allows checks to run from locations within the United States of America.
    ///
    /// "USA"
    #[serde(rename="USA")]
    USA,
    

    /// Allows checks to run from locations within the continent of Europe.
    ///
    /// "EUROPE"
    #[serde(rename="EUROPE")]
    EUROPE,
    

    /// Allows checks to run from locations within the continent of South America.
    ///
    /// "SOUTH_AMERICA"
    #[serde(rename="SOUTH_AMERICA")]
    SOUTHAMERICA,
    

    /// Allows checks to run from locations within the Asia Pacific area (ex: Singapore).
    ///
    /// "ASIA_PACIFIC"
    #[serde(rename="ASIA_PACIFIC")]
    ASIAPACIFIC,
    

    /// Allows checks to run from locations within the western United States of America
    ///
    /// "USA_OREGON"
    #[serde(rename="USA_OREGON")]
    USAOREGON,
    

    /// Allows checks to run from locations within the central United States of America
    ///
    /// "USA_IOWA"
    #[serde(rename="USA_IOWA")]
    USAIOWA,
    

    /// Allows checks to run from locations within the eastern United States of America
    ///
    /// "USA_VIRGINIA"
    #[serde(rename="USA_VIRGINIA")]
    USAVIRGINIA,
}

impl AsRef<str> for UptimeCheckConfigSelectedRegionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UptimeCheckConfigSelectedRegionsEnum::REGIONUNSPECIFIED => "REGION_UNSPECIFIED",
            UptimeCheckConfigSelectedRegionsEnum::USA => "USA",
            UptimeCheckConfigSelectedRegionsEnum::EUROPE => "EUROPE",
            UptimeCheckConfigSelectedRegionsEnum::SOUTHAMERICA => "SOUTH_AMERICA",
            UptimeCheckConfigSelectedRegionsEnum::ASIAPACIFIC => "ASIA_PACIFIC",
            UptimeCheckConfigSelectedRegionsEnum::USAOREGON => "USA_OREGON",
            UptimeCheckConfigSelectedRegionsEnum::USAIOWA => "USA_IOWA",
            UptimeCheckConfigSelectedRegionsEnum::USAVIRGINIA => "USA_VIRGINIA",
        }
    }
}

impl std::convert::TryFrom< &str> for UptimeCheckConfigSelectedRegionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REGION_UNSPECIFIED" => Ok(UptimeCheckConfigSelectedRegionsEnum::REGIONUNSPECIFIED),
           "USA" => Ok(UptimeCheckConfigSelectedRegionsEnum::USA),
           "EUROPE" => Ok(UptimeCheckConfigSelectedRegionsEnum::EUROPE),
           "SOUTH_AMERICA" => Ok(UptimeCheckConfigSelectedRegionsEnum::SOUTHAMERICA),
           "ASIA_PACIFIC" => Ok(UptimeCheckConfigSelectedRegionsEnum::ASIAPACIFIC),
           "USA_OREGON" => Ok(UptimeCheckConfigSelectedRegionsEnum::USAOREGON),
           "USA_IOWA" => Ok(UptimeCheckConfigSelectedRegionsEnum::USAIOWA),
           "USA_VIRGINIA" => Ok(UptimeCheckConfigSelectedRegionsEnum::USAVIRGINIA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UptimeCheckConfigSelectedRegionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UptimeCheckIpRegionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A broad region category in which the IP address is located.
pub enum UptimeCheckIpRegionEnum {
    

    /// Default value if no region is specified. Will result in Uptime checks running from all regions.
    ///
    /// "REGION_UNSPECIFIED"
    #[serde(rename="REGION_UNSPECIFIED")]
    REGIONUNSPECIFIED,
    

    /// Allows checks to run from locations within the United States of America.
    ///
    /// "USA"
    #[serde(rename="USA")]
    USA,
    

    /// Allows checks to run from locations within the continent of Europe.
    ///
    /// "EUROPE"
    #[serde(rename="EUROPE")]
    EUROPE,
    

    /// Allows checks to run from locations within the continent of South America.
    ///
    /// "SOUTH_AMERICA"
    #[serde(rename="SOUTH_AMERICA")]
    SOUTHAMERICA,
    

    /// Allows checks to run from locations within the Asia Pacific area (ex: Singapore).
    ///
    /// "ASIA_PACIFIC"
    #[serde(rename="ASIA_PACIFIC")]
    ASIAPACIFIC,
    

    /// Allows checks to run from locations within the western United States of America
    ///
    /// "USA_OREGON"
    #[serde(rename="USA_OREGON")]
    USAOREGON,
    

    /// Allows checks to run from locations within the central United States of America
    ///
    /// "USA_IOWA"
    #[serde(rename="USA_IOWA")]
    USAIOWA,
    

    /// Allows checks to run from locations within the eastern United States of America
    ///
    /// "USA_VIRGINIA"
    #[serde(rename="USA_VIRGINIA")]
    USAVIRGINIA,
}

impl AsRef<str> for UptimeCheckIpRegionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UptimeCheckIpRegionEnum::REGIONUNSPECIFIED => "REGION_UNSPECIFIED",
            UptimeCheckIpRegionEnum::USA => "USA",
            UptimeCheckIpRegionEnum::EUROPE => "EUROPE",
            UptimeCheckIpRegionEnum::SOUTHAMERICA => "SOUTH_AMERICA",
            UptimeCheckIpRegionEnum::ASIAPACIFIC => "ASIA_PACIFIC",
            UptimeCheckIpRegionEnum::USAOREGON => "USA_OREGON",
            UptimeCheckIpRegionEnum::USAIOWA => "USA_IOWA",
            UptimeCheckIpRegionEnum::USAVIRGINIA => "USA_VIRGINIA",
        }
    }
}

impl std::convert::TryFrom< &str> for UptimeCheckIpRegionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REGION_UNSPECIFIED" => Ok(UptimeCheckIpRegionEnum::REGIONUNSPECIFIED),
           "USA" => Ok(UptimeCheckIpRegionEnum::USA),
           "EUROPE" => Ok(UptimeCheckIpRegionEnum::EUROPE),
           "SOUTH_AMERICA" => Ok(UptimeCheckIpRegionEnum::SOUTHAMERICA),
           "ASIA_PACIFIC" => Ok(UptimeCheckIpRegionEnum::ASIAPACIFIC),
           "USA_OREGON" => Ok(UptimeCheckIpRegionEnum::USAOREGON),
           "USA_IOWA" => Ok(UptimeCheckIpRegionEnum::USAIOWA),
           "USA_VIRGINIA" => Ok(UptimeCheckIpRegionEnum::USAVIRGINIA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UptimeCheckIpRegionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValueDescriptorMetricKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value stream kind.
pub enum ValueDescriptorMetricKindEnum {
    

    /// Do not use this default value.
    ///
    /// "METRIC_KIND_UNSPECIFIED"
    #[serde(rename="METRIC_KIND_UNSPECIFIED")]
    METRICKINDUNSPECIFIED,
    

    /// An instantaneous measurement of a value.
    ///
    /// "GAUGE"
    #[serde(rename="GAUGE")]
    GAUGE,
    

    /// The change in a value during a time interval.
    ///
    /// "DELTA"
    #[serde(rename="DELTA")]
    DELTA,
    

    /// A value accumulated over a time interval. Cumulative measurements in a time series should have the same start time and increasing end times, until an event resets the cumulative value to zero and sets a new start time for the following points.
    ///
    /// "CUMULATIVE"
    #[serde(rename="CUMULATIVE")]
    CUMULATIVE,
}

impl AsRef<str> for ValueDescriptorMetricKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValueDescriptorMetricKindEnum::METRICKINDUNSPECIFIED => "METRIC_KIND_UNSPECIFIED",
            ValueDescriptorMetricKindEnum::GAUGE => "GAUGE",
            ValueDescriptorMetricKindEnum::DELTA => "DELTA",
            ValueDescriptorMetricKindEnum::CUMULATIVE => "CUMULATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ValueDescriptorMetricKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_KIND_UNSPECIFIED" => Ok(ValueDescriptorMetricKindEnum::METRICKINDUNSPECIFIED),
           "GAUGE" => Ok(ValueDescriptorMetricKindEnum::GAUGE),
           "DELTA" => Ok(ValueDescriptorMetricKindEnum::DELTA),
           "CUMULATIVE" => Ok(ValueDescriptorMetricKindEnum::CUMULATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValueDescriptorMetricKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValueDescriptorValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value type.
pub enum ValueDescriptorValueTypeEnum {
    

    /// Do not use this default value.
    ///
    /// "VALUE_TYPE_UNSPECIFIED"
    #[serde(rename="VALUE_TYPE_UNSPECIFIED")]
    VALUETYPEUNSPECIFIED,
    

    /// The value is a boolean. This value type can be used only if the metric kind is GAUGE.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// The value is a signed 64-bit integer.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// The value is a double precision floating point number.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// The value is a text string. This value type can be used only if the metric kind is GAUGE.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// The value is a Distribution.
    ///
    /// "DISTRIBUTION"
    #[serde(rename="DISTRIBUTION")]
    DISTRIBUTION,
    

    /// The value is money.
    ///
    /// "MONEY"
    #[serde(rename="MONEY")]
    MONEY,
}

impl AsRef<str> for ValueDescriptorValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValueDescriptorValueTypeEnum::VALUETYPEUNSPECIFIED => "VALUE_TYPE_UNSPECIFIED",
            ValueDescriptorValueTypeEnum::BOOL => "BOOL",
            ValueDescriptorValueTypeEnum::INT64 => "INT64",
            ValueDescriptorValueTypeEnum::DOUBLE => "DOUBLE",
            ValueDescriptorValueTypeEnum::STRING => "STRING",
            ValueDescriptorValueTypeEnum::DISTRIBUTION => "DISTRIBUTION",
            ValueDescriptorValueTypeEnum::MONEY => "MONEY",
        }
    }
}

impl std::convert::TryFrom< &str> for ValueDescriptorValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_TYPE_UNSPECIFIED" => Ok(ValueDescriptorValueTypeEnum::VALUETYPEUNSPECIFIED),
           "BOOL" => Ok(ValueDescriptorValueTypeEnum::BOOL),
           "INT64" => Ok(ValueDescriptorValueTypeEnum::INT64),
           "DOUBLE" => Ok(ValueDescriptorValueTypeEnum::DOUBLE),
           "STRING" => Ok(ValueDescriptorValueTypeEnum::STRING),
           "DISTRIBUTION" => Ok(ValueDescriptorValueTypeEnum::DISTRIBUTION),
           "MONEY" => Ok(ValueDescriptorValueTypeEnum::MONEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValueDescriptorValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderAggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum FolderAggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for FolderAggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderAggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            FolderAggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            FolderAggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            FolderAggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            FolderAggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            FolderAggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            FolderAggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            FolderAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            FolderAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            FolderAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderAggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(FolderAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderAggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderAggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum FolderAggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for FolderAggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderAggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            FolderAggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            FolderAggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            FolderAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            FolderAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            FolderAggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            FolderAggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            FolderAggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            FolderAggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            FolderAggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            FolderAggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            FolderAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            FolderAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            FolderAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderAggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(FolderAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderAggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderSecondaryAggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum FolderSecondaryAggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for FolderSecondaryAggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderSecondaryAggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(FolderSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderSecondaryAggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderSecondaryAggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum FolderSecondaryAggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for FolderSecondaryAggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderSecondaryAggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(FolderSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderSecondaryAggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies which information is returned about the time series.
pub enum FolderViewEnum {
    

    /// Returns the identity of the metric(s), the time series, and the time series data.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Returns the identity of the metric and the time series resource, but not the time series data.
    ///
    /// "HEADERS"
    #[serde(rename="HEADERS")]
    HEADERS,
}

impl AsRef<str> for FolderViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderViewEnum::FULL => "FULL",
            FolderViewEnum::HEADERS => "HEADERS",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FULL" => Ok(FolderViewEnum::FULL),
           "HEADERS" => Ok(FolderViewEnum::HEADERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationAggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum OrganizationAggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for OrganizationAggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationAggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationAggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(OrganizationAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationAggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationAggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum OrganizationAggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for OrganizationAggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationAggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(OrganizationAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationAggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationSecondaryAggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum OrganizationSecondaryAggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for OrganizationSecondaryAggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationSecondaryAggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(OrganizationSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationSecondaryAggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationSecondaryAggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum OrganizationSecondaryAggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for OrganizationSecondaryAggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationSecondaryAggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(OrganizationSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationSecondaryAggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies which information is returned about the time series.
pub enum OrganizationViewEnum {
    

    /// Returns the identity of the metric(s), the time series, and the time series data.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Returns the identity of the metric and the time series resource, but not the time series data.
    ///
    /// "HEADERS"
    #[serde(rename="HEADERS")]
    HEADERS,
}

impl AsRef<str> for OrganizationViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationViewEnum::FULL => "FULL",
            OrganizationViewEnum::HEADERS => "HEADERS",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FULL" => Ok(OrganizationViewEnum::FULL),
           "HEADERS" => Ok(OrganizationViewEnum::HEADERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectAggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum ProjectAggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for ProjectAggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectAggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            ProjectAggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            ProjectAggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            ProjectAggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            ProjectAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            ProjectAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectAggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(ProjectAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectAggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectAggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum ProjectAggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for ProjectAggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectAggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectAggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(ProjectAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectAggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectSecondaryAggregationCrossSeriesReducerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reduction operation to be used to combine time series into a single time series, where the value of each data point in the resulting series is a function of all the already aligned values in the input time series.Not all reducer operations can be applied to all time series. The valid choices depend on the metric_kind and the value_type of the original time series. Reduction can yield a time series with a different metric_kind or value_type than the input time series.Time series data must first be aligned (see per_series_aligner) in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified, and must not be ALIGN_NONE. An alignment_period must also be specified; otherwise, an error is returned.
pub enum ProjectSecondaryAggregationCrossSeriesReducerEnum {
    

    /// No cross-time series reduction. The output of the Aligner is returned.
    ///
    /// "REDUCE_NONE"
    #[serde(rename="REDUCE_NONE")]
    REDUCENONE,
    

    /// Reduce by computing the mean value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_MEAN"
    #[serde(rename="REDUCE_MEAN")]
    REDUCEMEAN,
    

    /// Reduce by computing the minimum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MIN"
    #[serde(rename="REDUCE_MIN")]
    REDUCEMIN,
    

    /// Reduce by computing the maximum value across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_MAX"
    #[serde(rename="REDUCE_MAX")]
    REDUCEMAX,
    

    /// Reduce by computing the sum across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric and distribution values. The value_type of the output is the same as the value_type of the input.
    ///
    /// "REDUCE_SUM"
    #[serde(rename="REDUCE_SUM")]
    REDUCESUM,
    

    /// Reduce by computing the standard deviation across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics with numeric or distribution values. The value_type of the output is DOUBLE.
    ///
    /// "REDUCE_STDDEV"
    #[serde(rename="REDUCE_STDDEV")]
    REDUCESTDDEV,
    

    /// Reduce by computing the number of data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of numeric, Boolean, distribution, and string value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT"
    #[serde(rename="REDUCE_COUNT")]
    REDUCECOUNT,
    

    /// Reduce by computing the number of True-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_TRUE"
    #[serde(rename="REDUCE_COUNT_TRUE")]
    REDUCECOUNTTRUE,
    

    /// Reduce by computing the number of False-valued data points across time series for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The value_type of the output is INT64.
    ///
    /// "REDUCE_COUNT_FALSE"
    #[serde(rename="REDUCE_COUNT_FALSE")]
    REDUCECOUNTFALSE,
    

    /// Reduce by computing the ratio of the number of True-valued data points to the total number of data points for each alignment period. This reducer is valid for DELTA and GAUGE metrics of Boolean value_type. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "REDUCE_FRACTION_TRUE"
    #[serde(rename="REDUCE_FRACTION_TRUE")]
    REDUCEFRACTIONTRUE,
    

    /// Reduce by computing the 99th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_99"
    #[serde(rename="REDUCE_PERCENTILE_99")]
    REDUCEPERCENTILE99,
    

    /// Reduce by computing the 95th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_95"
    #[serde(rename="REDUCE_PERCENTILE_95")]
    REDUCEPERCENTILE95,
    

    /// Reduce by computing the 50th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_50"
    #[serde(rename="REDUCE_PERCENTILE_50")]
    REDUCEPERCENTILE50,
    

    /// Reduce by computing the 5th percentile (https://en.wikipedia.org/wiki/Percentile) of data points across time series for each alignment period. This reducer is valid for GAUGE and DELTA metrics of numeric and distribution type. The value of the output is DOUBLE.
    ///
    /// "REDUCE_PERCENTILE_05"
    #[serde(rename="REDUCE_PERCENTILE_05")]
    REDUCEPERCENTILE05,
}

impl AsRef<str> for ProjectSecondaryAggregationCrossSeriesReducerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCENONE => "REDUCE_NONE",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEMEAN => "REDUCE_MEAN",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEMIN => "REDUCE_MIN",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEMAX => "REDUCE_MAX",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCESUM => "REDUCE_SUM",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCESTDDEV => "REDUCE_STDDEV",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNT => "REDUCE_COUNT",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE => "REDUCE_COUNT_TRUE",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE => "REDUCE_COUNT_FALSE",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE => "REDUCE_FRACTION_TRUE",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99 => "REDUCE_PERCENTILE_99",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95 => "REDUCE_PERCENTILE_95",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50 => "REDUCE_PERCENTILE_50",
            ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05 => "REDUCE_PERCENTILE_05",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectSecondaryAggregationCrossSeriesReducerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDUCE_NONE" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCENONE),
           "REDUCE_MEAN" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEMEAN),
           "REDUCE_MIN" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEMIN),
           "REDUCE_MAX" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEMAX),
           "REDUCE_SUM" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCESUM),
           "REDUCE_STDDEV" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCESTDDEV),
           "REDUCE_COUNT" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNT),
           "REDUCE_COUNT_TRUE" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTTRUE),
           "REDUCE_COUNT_FALSE" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCECOUNTFALSE),
           "REDUCE_FRACTION_TRUE" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEFRACTIONTRUE),
           "REDUCE_PERCENTILE_99" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE99),
           "REDUCE_PERCENTILE_95" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE95),
           "REDUCE_PERCENTILE_50" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE50),
           "REDUCE_PERCENTILE_05" => Ok(ProjectSecondaryAggregationCrossSeriesReducerEnum::REDUCEPERCENTILE05),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectSecondaryAggregationCrossSeriesReducerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectSecondaryAggregationPerSeriesAlignerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An Aligner describes how to bring the data points in a single time series into temporal alignment. Except for ALIGN_NONE, all alignments cause all the data points in an alignment_period to be mathematically grouped together, resulting in a single data point for each alignment_period with end timestamp at the end of the period.Not all alignment operations may be applied to all time series. The valid choices depend on the metric_kind and value_type of the original time series. Alignment can change the metric_kind or the value_type of the time series.Time series data must be aligned in order to perform cross-time series reduction. If cross_series_reducer is specified, then per_series_aligner must be specified and not equal to ALIGN_NONE and alignment_period must be specified; otherwise, an error is returned.
pub enum ProjectSecondaryAggregationPerSeriesAlignerEnum {
    

    /// No alignment. Raw data is returned. Not valid if cross-series reduction is requested. The value_type of the result is the same as the value_type of the input.
    ///
    /// "ALIGN_NONE"
    #[serde(rename="ALIGN_NONE")]
    ALIGNNONE,
    

    /// Align and convert to DELTA. The output is delta = y1 - y0.This alignment is valid for CUMULATIVE and DELTA metrics. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_DELTA"
    #[serde(rename="ALIGN_DELTA")]
    ALIGNDELTA,
    

    /// Align and convert to a rate. The result is computed as rate = (y1 - y0)/(t1 - t0), or "delta over time". Think of this aligner as providing the slope of the line that passes through the value at the start and at the end of the alignment_period.This aligner is valid for CUMULATIVE and DELTA metrics with numeric values. If the selected alignment period results in periods with no data, then the aligned value for such a period is created by interpolation. The output is a GAUGE metric with value_type DOUBLE.If, by "rate", you mean "percentage change", see the ALIGN_PERCENT_CHANGE aligner instead.
    ///
    /// "ALIGN_RATE"
    #[serde(rename="ALIGN_RATE")]
    ALIGNRATE,
    

    /// Align by interpolating between adjacent points around the alignment period boundary. This aligner is valid for GAUGE metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_INTERPOLATE"
    #[serde(rename="ALIGN_INTERPOLATE")]
    ALIGNINTERPOLATE,
    

    /// Align by moving the most recent data point before the end of the alignment period to the boundary at the end of the alignment period. This aligner is valid for GAUGE metrics. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_NEXT_OLDER"
    #[serde(rename="ALIGN_NEXT_OLDER")]
    ALIGNNEXTOLDER,
    

    /// Align the time series by returning the minimum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MIN"
    #[serde(rename="ALIGN_MIN")]
    ALIGNMIN,
    

    /// Align the time series by returning the maximum value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_MAX"
    #[serde(rename="ALIGN_MAX")]
    ALIGNMAX,
    

    /// Align the time series by returning the mean value in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the aligned result is DOUBLE.
    ///
    /// "ALIGN_MEAN"
    #[serde(rename="ALIGN_MEAN")]
    ALIGNMEAN,
    

    /// Align the time series by returning the number of values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric or Boolean values. The value_type of the aligned result is INT64.
    ///
    /// "ALIGN_COUNT"
    #[serde(rename="ALIGN_COUNT")]
    ALIGNCOUNT,
    

    /// Align the time series by returning the sum of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric and distribution values. The value_type of the aligned result is the same as the value_type of the input.
    ///
    /// "ALIGN_SUM"
    #[serde(rename="ALIGN_SUM")]
    ALIGNSUM,
    

    /// Align the time series by returning the standard deviation of the values in each alignment period. This aligner is valid for GAUGE and DELTA metrics with numeric values. The value_type of the output is DOUBLE.
    ///
    /// "ALIGN_STDDEV"
    #[serde(rename="ALIGN_STDDEV")]
    ALIGNSTDDEV,
    

    /// Align the time series by returning the number of True values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_TRUE"
    #[serde(rename="ALIGN_COUNT_TRUE")]
    ALIGNCOUNTTRUE,
    

    /// Align the time series by returning the number of False values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The value_type of the output is INT64.
    ///
    /// "ALIGN_COUNT_FALSE"
    #[serde(rename="ALIGN_COUNT_FALSE")]
    ALIGNCOUNTFALSE,
    

    /// Align the time series by returning the ratio of the number of True values to the total number of values in each alignment period. This aligner is valid for GAUGE metrics with Boolean values. The output value is in the range 0.0, 1.0 and has value_type DOUBLE.
    ///
    /// "ALIGN_FRACTION_TRUE"
    #[serde(rename="ALIGN_FRACTION_TRUE")]
    ALIGNFRACTIONTRUE,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 99th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_99"
    #[serde(rename="ALIGN_PERCENTILE_99")]
    ALIGNPERCENTILE99,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 95th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_95"
    #[serde(rename="ALIGN_PERCENTILE_95")]
    ALIGNPERCENTILE95,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 50th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_50"
    #[serde(rename="ALIGN_PERCENTILE_50")]
    ALIGNPERCENTILE50,
    

    /// Align the time series by using percentile aggregation (https://en.wikipedia.org/wiki/Percentile). The resulting data point in each alignment period is the 5th percentile of all data points in the period. This aligner is valid for GAUGE and DELTA metrics with distribution values. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENTILE_05"
    #[serde(rename="ALIGN_PERCENTILE_05")]
    ALIGNPERCENTILE05,
    

    /// Align and convert to a percentage change. This aligner is valid for GAUGE and DELTA metrics with numeric values. This alignment returns ((current - previous)/previous) * 100, where the value of previous is determined based on the alignment_period.If the values of current and previous are both 0, then the returned value is 0. If only previous is 0, the returned value is infinity.A 10-minute moving mean is computed at each point of the alignment period prior to the above calculation to smooth the metric and prevent false positives from very short-lived spikes. The moving mean is only applicable for data whose values are >= 0. Any values < 0 are treated as a missing datapoint, and are ignored. While DELTA metrics are accepted by this alignment, special care should be taken that the values for the metric will always be positive. The output is a GAUGE metric with value_type DOUBLE.
    ///
    /// "ALIGN_PERCENT_CHANGE"
    #[serde(rename="ALIGN_PERCENT_CHANGE")]
    ALIGNPERCENTCHANGE,
}

impl AsRef<str> for ProjectSecondaryAggregationPerSeriesAlignerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNNONE => "ALIGN_NONE",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNDELTA => "ALIGN_DELTA",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNRATE => "ALIGN_RATE",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE => "ALIGN_INTERPOLATE",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER => "ALIGN_NEXT_OLDER",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNMIN => "ALIGN_MIN",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNMAX => "ALIGN_MAX",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNMEAN => "ALIGN_MEAN",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNT => "ALIGN_COUNT",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNSUM => "ALIGN_SUM",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNSTDDEV => "ALIGN_STDDEV",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE => "ALIGN_COUNT_TRUE",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE => "ALIGN_COUNT_FALSE",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE => "ALIGN_FRACTION_TRUE",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99 => "ALIGN_PERCENTILE_99",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95 => "ALIGN_PERCENTILE_95",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50 => "ALIGN_PERCENTILE_50",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05 => "ALIGN_PERCENTILE_05",
            ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE => "ALIGN_PERCENT_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectSecondaryAggregationPerSeriesAlignerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALIGN_NONE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNNONE),
           "ALIGN_DELTA" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNDELTA),
           "ALIGN_RATE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNRATE),
           "ALIGN_INTERPOLATE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNINTERPOLATE),
           "ALIGN_NEXT_OLDER" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNNEXTOLDER),
           "ALIGN_MIN" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNMIN),
           "ALIGN_MAX" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNMAX),
           "ALIGN_MEAN" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNMEAN),
           "ALIGN_COUNT" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNT),
           "ALIGN_SUM" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNSUM),
           "ALIGN_STDDEV" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNSTDDEV),
           "ALIGN_COUNT_TRUE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTTRUE),
           "ALIGN_COUNT_FALSE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNCOUNTFALSE),
           "ALIGN_FRACTION_TRUE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNFRACTIONTRUE),
           "ALIGN_PERCENTILE_99" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE99),
           "ALIGN_PERCENTILE_95" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE95),
           "ALIGN_PERCENTILE_50" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE50),
           "ALIGN_PERCENTILE_05" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTILE05),
           "ALIGN_PERCENT_CHANGE" => Ok(ProjectSecondaryAggregationPerSeriesAlignerEnum::ALIGNPERCENTCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectSecondaryAggregationPerSeriesAlignerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies which information is returned about the time series.
pub enum ProjectViewEnum {
    

    /// Returns the identity of the metric(s), the time series, and the time series data.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Returns the identity of the metric and the time series resource, but not the time series data.
    ///
    /// "HEADERS"
    #[serde(rename="HEADERS")]
    HEADERS,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::FULL => "FULL",
            ProjectViewEnum::HEADERS => "HEADERS",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FULL" => Ok(ProjectViewEnum::FULL),
           "HEADERS" => Ok(ProjectViewEnum::HEADERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// View of the ServiceLevelObjectives to return. If DEFAULT, return each ServiceLevelObjective as originally defined. If EXPLICIT and the ServiceLevelObjective is defined in terms of a BasicSli, replace the BasicSli with a RequestBasedSli spelling out how the SLI is computed.
pub enum ServiceViewEnum {
    

    /// Same as FULL.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Return the embedded ServiceLevelIndicator in the form in which it was defined. If it was defined using a BasicSli, return that BasicSli.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// For ServiceLevelIndicators using BasicSli articulation, instead return the ServiceLevelIndicator with its mode of computation fully spelled out as a RequestBasedSli. For ServiceLevelIndicators using RequestBasedSli or WindowsBasedSli, return the ServiceLevelIndicator as it was provided.
    ///
    /// "EXPLICIT"
    #[serde(rename="EXPLICIT")]
    EXPLICIT,
}

impl AsRef<str> for ServiceViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            ServiceViewEnum::FULL => "FULL",
            ServiceViewEnum::EXPLICIT => "EXPLICIT",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(ServiceViewEnum::VIEWUNSPECIFIED),
           "FULL" => Ok(ServiceViewEnum::FULL),
           "EXPLICIT" => Ok(ServiceViewEnum::EXPLICIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


