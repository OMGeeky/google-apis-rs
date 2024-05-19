use super::*;



// region TimeseryAggregatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The aggregation function that will reduce the data points in each window to a single point. This parameter is only valid for non-cumulative metrics with a value type of INT64 or DOUBLE.
pub enum TimeseryAggregatorEnum {
    
    /// "max"
    #[serde(rename="max")]
    Max,
    
    /// "mean"
    #[serde(rename="mean")]
    Mean,
    
    /// "min"
    #[serde(rename="min")]
    Min,
    
    /// "sum"
    #[serde(rename="sum")]
    Sum,
}

impl AsRef<str> for TimeseryAggregatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimeseryAggregatorEnum::Max => "max",
            TimeseryAggregatorEnum::Mean => "mean",
            TimeseryAggregatorEnum::Min => "min",
            TimeseryAggregatorEnum::Sum => "sum",
        }
    }
}

impl std::convert::TryFrom< &str> for TimeseryAggregatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "max" => Ok(TimeseryAggregatorEnum::Max),
           "mean" => Ok(TimeseryAggregatorEnum::Mean),
           "min" => Ok(TimeseryAggregatorEnum::Min),
           "sum" => Ok(TimeseryAggregatorEnum::Sum),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimeseryAggregatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TimeseriesDescriptorAggregatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The aggregation function that will reduce the data points in each window to a single point. This parameter is only valid for non-cumulative metrics with a value type of INT64 or DOUBLE.
pub enum TimeseriesDescriptorAggregatorEnum {
    
    /// "max"
    #[serde(rename="max")]
    Max,
    
    /// "mean"
    #[serde(rename="mean")]
    Mean,
    
    /// "min"
    #[serde(rename="min")]
    Min,
    
    /// "sum"
    #[serde(rename="sum")]
    Sum,
}

impl AsRef<str> for TimeseriesDescriptorAggregatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TimeseriesDescriptorAggregatorEnum::Max => "max",
            TimeseriesDescriptorAggregatorEnum::Mean => "mean",
            TimeseriesDescriptorAggregatorEnum::Min => "min",
            TimeseriesDescriptorAggregatorEnum::Sum => "sum",
        }
    }
}

impl std::convert::TryFrom< &str> for TimeseriesDescriptorAggregatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "max" => Ok(TimeseriesDescriptorAggregatorEnum::Max),
           "mean" => Ok(TimeseriesDescriptorAggregatorEnum::Mean),
           "min" => Ok(TimeseriesDescriptorAggregatorEnum::Min),
           "sum" => Ok(TimeseriesDescriptorAggregatorEnum::Sum),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TimeseriesDescriptorAggregatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


