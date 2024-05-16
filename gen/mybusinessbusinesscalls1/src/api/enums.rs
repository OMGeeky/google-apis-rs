use super::*;



// region BusinessCallsInsightMetricTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The metric for which the value applies.
pub enum BusinessCallsInsightMetricTypeEnum {
    

    /// Type of metric is unspecified.
    ///
    /// "METRIC_TYPE_UNSPECIFIED"
    #[serde(rename="METRIC_TYPE_UNSPECIFIED")]
    METRICTYPEUNSPECIFIED,
    

    /// The metrics provided are counts aggregated over the input time_range.
    ///
    /// "AGGREGATE_COUNT"
    #[serde(rename="AGGREGATE_COUNT")]
    AGGREGATECOUNT,
}

impl AsRef<str> for BusinessCallsInsightMetricTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinessCallsInsightMetricTypeEnum::METRICTYPEUNSPECIFIED => "METRIC_TYPE_UNSPECIFIED",
            BusinessCallsInsightMetricTypeEnum::AGGREGATECOUNT => "AGGREGATE_COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinessCallsInsightMetricTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_TYPE_UNSPECIFIED" => Ok(BusinessCallsInsightMetricTypeEnum::METRICTYPEUNSPECIFIED),
           "AGGREGATE_COUNT" => Ok(BusinessCallsInsightMetricTypeEnum::AGGREGATECOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinessCallsInsightMetricTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BusinessCallsSettingCallsStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of this location's enrollment in Business calls.
pub enum BusinessCallsSettingCallsStateEnum {
    

    /// Unspecified.
    ///
    /// "CALLS_STATE_UNSPECIFIED"
    #[serde(rename="CALLS_STATE_UNSPECIFIED")]
    CALLSSTATEUNSPECIFIED,
    

    /// Business calls is enabled for the location.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Business calls is disabled for the location.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for BusinessCallsSettingCallsStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BusinessCallsSettingCallsStateEnum::CALLSSTATEUNSPECIFIED => "CALLS_STATE_UNSPECIFIED",
            BusinessCallsSettingCallsStateEnum::ENABLED => "ENABLED",
            BusinessCallsSettingCallsStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for BusinessCallsSettingCallsStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CALLS_STATE_UNSPECIFIED" => Ok(BusinessCallsSettingCallsStateEnum::CALLSSTATEUNSPECIFIED),
           "ENABLED" => Ok(BusinessCallsSettingCallsStateEnum::ENABLED),
           "DISABLED" => Ok(BusinessCallsSettingCallsStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BusinessCallsSettingCallsStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WeekDayMetricDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Day of the week. Allowed values are Sunday - Saturday.
pub enum WeekDayMetricDayEnum {
    

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

impl AsRef<str> for WeekDayMetricDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WeekDayMetricDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            WeekDayMetricDayEnum::MONDAY => "MONDAY",
            WeekDayMetricDayEnum::TUESDAY => "TUESDAY",
            WeekDayMetricDayEnum::WEDNESDAY => "WEDNESDAY",
            WeekDayMetricDayEnum::THURSDAY => "THURSDAY",
            WeekDayMetricDayEnum::FRIDAY => "FRIDAY",
            WeekDayMetricDayEnum::SATURDAY => "SATURDAY",
            WeekDayMetricDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for WeekDayMetricDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(WeekDayMetricDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(WeekDayMetricDayEnum::MONDAY),
           "TUESDAY" => Ok(WeekDayMetricDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(WeekDayMetricDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(WeekDayMetricDayEnum::THURSDAY),
           "FRIDAY" => Ok(WeekDayMetricDayEnum::FRIDAY),
           "SATURDAY" => Ok(WeekDayMetricDayEnum::SATURDAY),
           "SUNDAY" => Ok(WeekDayMetricDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WeekDayMetricDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


