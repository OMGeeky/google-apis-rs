use super::*;



// region ActiveMetricRestrictionRestrictedMetricTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason for this metric's restriction.
pub enum ActiveMetricRestrictionRestrictedMetricTypesEnum {
    

    /// Unspecified type.
    ///
    /// "RESTRICTED_METRIC_TYPE_UNSPECIFIED"
    #[serde(rename="RESTRICTED_METRIC_TYPE_UNSPECIFIED")]
    RESTRICTEDMETRICTYPEUNSPECIFIED,
    

    /// Cost metrics such as `adCost`.
    ///
    /// "COST_DATA"
    #[serde(rename="COST_DATA")]
    COSTDATA,
    

    /// Revenue metrics such as `purchaseRevenue`.
    ///
    /// "REVENUE_DATA"
    #[serde(rename="REVENUE_DATA")]
    REVENUEDATA,
}

impl AsRef<str> for ActiveMetricRestrictionRestrictedMetricTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActiveMetricRestrictionRestrictedMetricTypesEnum::RESTRICTEDMETRICTYPEUNSPECIFIED => "RESTRICTED_METRIC_TYPE_UNSPECIFIED",
            ActiveMetricRestrictionRestrictedMetricTypesEnum::COSTDATA => "COST_DATA",
            ActiveMetricRestrictionRestrictedMetricTypesEnum::REVENUEDATA => "REVENUE_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for ActiveMetricRestrictionRestrictedMetricTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTRICTED_METRIC_TYPE_UNSPECIFIED" => Ok(ActiveMetricRestrictionRestrictedMetricTypesEnum::RESTRICTEDMETRICTYPEUNSPECIFIED),
           "COST_DATA" => Ok(ActiveMetricRestrictionRestrictedMetricTypesEnum::COSTDATA),
           "REVENUE_DATA" => Ok(ActiveMetricRestrictionRestrictedMetricTypesEnum::REVENUEDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActiveMetricRestrictionRestrictedMetricTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AudienceExportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state for this AudienceExport.
pub enum AudienceExportStateEnum {
    

    /// Unspecified state will never be used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The AudienceExport is currently creating and will be available in the future. Creating occurs immediately after the CreateAudienceExport call.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The AudienceExport is fully created and ready for querying. An AudienceExport is updated to active asynchronously from a request; this occurs some time (for example 15 minutes) after the initial create call.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The AudienceExport failed to be created. It is possible that re-requesting this audience export will succeed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for AudienceExportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AudienceExportStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AudienceExportStateEnum::CREATING => "CREATING",
            AudienceExportStateEnum::ACTIVE => "ACTIVE",
            AudienceExportStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for AudienceExportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AudienceExportStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(AudienceExportStateEnum::CREATING),
           "ACTIVE" => Ok(AudienceExportStateEnum::ACTIVE),
           "FAILED" => Ok(AudienceExportStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AudienceExportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CheckCompatibilityRequestCompatibilityFilterEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters the dimensions and metrics in the response to just this compatibility. Commonly used as `”compatibilityFilter”: “COMPATIBLE”` to only return compatible dimensions & metrics.
pub enum CheckCompatibilityRequestCompatibilityFilterEnum {
    

    /// Unspecified compatibility.
    ///
    /// "COMPATIBILITY_UNSPECIFIED"
    #[serde(rename="COMPATIBILITY_UNSPECIFIED")]
    COMPATIBILITYUNSPECIFIED,
    

    /// The dimension or metric is compatible. This dimension or metric can be successfully added to a report.
    ///
    /// "COMPATIBLE"
    #[serde(rename="COMPATIBLE")]
    COMPATIBLE,
    

    /// The dimension or metric is incompatible. This dimension or metric cannot be successfully added to a report.
    ///
    /// "INCOMPATIBLE"
    #[serde(rename="INCOMPATIBLE")]
    INCOMPATIBLE,
}

impl AsRef<str> for CheckCompatibilityRequestCompatibilityFilterEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CheckCompatibilityRequestCompatibilityFilterEnum::COMPATIBILITYUNSPECIFIED => "COMPATIBILITY_UNSPECIFIED",
            CheckCompatibilityRequestCompatibilityFilterEnum::COMPATIBLE => "COMPATIBLE",
            CheckCompatibilityRequestCompatibilityFilterEnum::INCOMPATIBLE => "INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CheckCompatibilityRequestCompatibilityFilterEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPATIBILITY_UNSPECIFIED" => Ok(CheckCompatibilityRequestCompatibilityFilterEnum::COMPATIBILITYUNSPECIFIED),
           "COMPATIBLE" => Ok(CheckCompatibilityRequestCompatibilityFilterEnum::COMPATIBLE),
           "INCOMPATIBLE" => Ok(CheckCompatibilityRequestCompatibilityFilterEnum::INCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CheckCompatibilityRequestCompatibilityFilterEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CohortsRangeGranularityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The granularity used to interpret the `startOffset` and `endOffset` for the extended reporting date range for a cohort report.
pub enum CohortsRangeGranularityEnum {
    

    /// Should never be specified.
    ///
    /// "GRANULARITY_UNSPECIFIED"
    #[serde(rename="GRANULARITY_UNSPECIFIED")]
    GRANULARITYUNSPECIFIED,
    

    /// Daily granularity. Commonly used if the cohort's `dateRange` is a single day and the request contains `cohortNthDay`.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
    

    /// Weekly granularity. Commonly used if the cohort's `dateRange` is a week in duration (starting on Sunday and ending on Saturday) and the request contains `cohortNthWeek`.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// Monthly granularity. Commonly used if the cohort's `dateRange` is a month in duration and the request contains `cohortNthMonth`.
    ///
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
}

impl AsRef<str> for CohortsRangeGranularityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CohortsRangeGranularityEnum::GRANULARITYUNSPECIFIED => "GRANULARITY_UNSPECIFIED",
            CohortsRangeGranularityEnum::DAILY => "DAILY",
            CohortsRangeGranularityEnum::WEEKLY => "WEEKLY",
            CohortsRangeGranularityEnum::MONTHLY => "MONTHLY",
        }
    }
}

impl std::convert::TryFrom< &str> for CohortsRangeGranularityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GRANULARITY_UNSPECIFIED" => Ok(CohortsRangeGranularityEnum::GRANULARITYUNSPECIFIED),
           "DAILY" => Ok(CohortsRangeGranularityEnum::DAILY),
           "WEEKLY" => Ok(CohortsRangeGranularityEnum::WEEKLY),
           "MONTHLY" => Ok(CohortsRangeGranularityEnum::MONTHLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CohortsRangeGranularityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionCompatibilityCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The compatibility of this dimension. If the compatibility is COMPATIBLE, this dimension can be successfully added to the report.
pub enum DimensionCompatibilityCompatibilityEnum {
    

    /// Unspecified compatibility.
    ///
    /// "COMPATIBILITY_UNSPECIFIED"
    #[serde(rename="COMPATIBILITY_UNSPECIFIED")]
    COMPATIBILITYUNSPECIFIED,
    

    /// The dimension or metric is compatible. This dimension or metric can be successfully added to a report.
    ///
    /// "COMPATIBLE"
    #[serde(rename="COMPATIBLE")]
    COMPATIBLE,
    

    /// The dimension or metric is incompatible. This dimension or metric cannot be successfully added to a report.
    ///
    /// "INCOMPATIBLE"
    #[serde(rename="INCOMPATIBLE")]
    INCOMPATIBLE,
}

impl AsRef<str> for DimensionCompatibilityCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionCompatibilityCompatibilityEnum::COMPATIBILITYUNSPECIFIED => "COMPATIBILITY_UNSPECIFIED",
            DimensionCompatibilityCompatibilityEnum::COMPATIBLE => "COMPATIBLE",
            DimensionCompatibilityCompatibilityEnum::INCOMPATIBLE => "INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionCompatibilityCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPATIBILITY_UNSPECIFIED" => Ok(DimensionCompatibilityCompatibilityEnum::COMPATIBILITYUNSPECIFIED),
           "COMPATIBLE" => Ok(DimensionCompatibilityCompatibilityEnum::COMPATIBLE),
           "INCOMPATIBLE" => Ok(DimensionCompatibilityCompatibilityEnum::INCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionCompatibilityCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionOrderByOrderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the rule for dimension value ordering.
pub enum DimensionOrderByOrderTypeEnum {
    

    /// Unspecified.
    ///
    /// "ORDER_TYPE_UNSPECIFIED"
    #[serde(rename="ORDER_TYPE_UNSPECIFIED")]
    ORDERTYPEUNSPECIFIED,
    

    /// Alphanumeric sort by Unicode code point. For example, "2" < "A" < "X" < "b" < "z".
    ///
    /// "ALPHANUMERIC"
    #[serde(rename="ALPHANUMERIC")]
    ALPHANUMERIC,
    

    /// Case insensitive alphanumeric sort by lower case Unicode code point. For example, "2" < "A" < "b" < "X" < "z".
    ///
    /// "CASE_INSENSITIVE_ALPHANUMERIC"
    #[serde(rename="CASE_INSENSITIVE_ALPHANUMERIC")]
    CASEINSENSITIVEALPHANUMERIC,
    

    /// Dimension values are converted to numbers before sorting. For example in NUMERIC sort, "25" < "100", and in `ALPHANUMERIC` sort, "100" < "25". Non-numeric dimension values all have equal ordering value below all numeric values.
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
}

impl AsRef<str> for DimensionOrderByOrderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionOrderByOrderTypeEnum::ORDERTYPEUNSPECIFIED => "ORDER_TYPE_UNSPECIFIED",
            DimensionOrderByOrderTypeEnum::ALPHANUMERIC => "ALPHANUMERIC",
            DimensionOrderByOrderTypeEnum::CASEINSENSITIVEALPHANUMERIC => "CASE_INSENSITIVE_ALPHANUMERIC",
            DimensionOrderByOrderTypeEnum::NUMERIC => "NUMERIC",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionOrderByOrderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_TYPE_UNSPECIFIED" => Ok(DimensionOrderByOrderTypeEnum::ORDERTYPEUNSPECIFIED),
           "ALPHANUMERIC" => Ok(DimensionOrderByOrderTypeEnum::ALPHANUMERIC),
           "CASE_INSENSITIVE_ALPHANUMERIC" => Ok(DimensionOrderByOrderTypeEnum::CASEINSENSITIVEALPHANUMERIC),
           "NUMERIC" => Ok(DimensionOrderByOrderTypeEnum::NUMERIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionOrderByOrderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricCompatibilityCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The compatibility of this metric. If the compatibility is COMPATIBLE, this metric can be successfully added to the report.
pub enum MetricCompatibilityCompatibilityEnum {
    

    /// Unspecified compatibility.
    ///
    /// "COMPATIBILITY_UNSPECIFIED"
    #[serde(rename="COMPATIBILITY_UNSPECIFIED")]
    COMPATIBILITYUNSPECIFIED,
    

    /// The dimension or metric is compatible. This dimension or metric can be successfully added to a report.
    ///
    /// "COMPATIBLE"
    #[serde(rename="COMPATIBLE")]
    COMPATIBLE,
    

    /// The dimension or metric is incompatible. This dimension or metric cannot be successfully added to a report.
    ///
    /// "INCOMPATIBLE"
    #[serde(rename="INCOMPATIBLE")]
    INCOMPATIBLE,
}

impl AsRef<str> for MetricCompatibilityCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricCompatibilityCompatibilityEnum::COMPATIBILITYUNSPECIFIED => "COMPATIBILITY_UNSPECIFIED",
            MetricCompatibilityCompatibilityEnum::COMPATIBLE => "COMPATIBLE",
            MetricCompatibilityCompatibilityEnum::INCOMPATIBLE => "INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricCompatibilityCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPATIBILITY_UNSPECIFIED" => Ok(MetricCompatibilityCompatibilityEnum::COMPATIBILITYUNSPECIFIED),
           "COMPATIBLE" => Ok(MetricCompatibilityCompatibilityEnum::COMPATIBLE),
           "INCOMPATIBLE" => Ok(MetricCompatibilityCompatibilityEnum::INCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricCompatibilityCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricHeaderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The metric's data type.
pub enum MetricHeaderTypeEnum {
    

    /// Unspecified type.
    ///
    /// "METRIC_TYPE_UNSPECIFIED"
    #[serde(rename="METRIC_TYPE_UNSPECIFIED")]
    METRICTYPEUNSPECIFIED,
    

    /// Integer type.
    ///
    /// "TYPE_INTEGER"
    #[serde(rename="TYPE_INTEGER")]
    TYPEINTEGER,
    

    /// Floating point type.
    ///
    /// "TYPE_FLOAT"
    #[serde(rename="TYPE_FLOAT")]
    TYPEFLOAT,
    

    /// A duration of seconds; a special floating point type.
    ///
    /// "TYPE_SECONDS"
    #[serde(rename="TYPE_SECONDS")]
    TYPESECONDS,
    

    /// A duration in milliseconds; a special floating point type.
    ///
    /// "TYPE_MILLISECONDS"
    #[serde(rename="TYPE_MILLISECONDS")]
    TYPEMILLISECONDS,
    

    /// A duration in minutes; a special floating point type.
    ///
    /// "TYPE_MINUTES"
    #[serde(rename="TYPE_MINUTES")]
    TYPEMINUTES,
    

    /// A duration in hours; a special floating point type.
    ///
    /// "TYPE_HOURS"
    #[serde(rename="TYPE_HOURS")]
    TYPEHOURS,
    

    /// A custom metric of standard type; a special floating point type.
    ///
    /// "TYPE_STANDARD"
    #[serde(rename="TYPE_STANDARD")]
    TYPESTANDARD,
    

    /// An amount of money; a special floating point type.
    ///
    /// "TYPE_CURRENCY"
    #[serde(rename="TYPE_CURRENCY")]
    TYPECURRENCY,
    

    /// A length in feet; a special floating point type.
    ///
    /// "TYPE_FEET"
    #[serde(rename="TYPE_FEET")]
    TYPEFEET,
    

    /// A length in miles; a special floating point type.
    ///
    /// "TYPE_MILES"
    #[serde(rename="TYPE_MILES")]
    TYPEMILES,
    

    /// A length in meters; a special floating point type.
    ///
    /// "TYPE_METERS"
    #[serde(rename="TYPE_METERS")]
    TYPEMETERS,
    

    /// A length in kilometers; a special floating point type.
    ///
    /// "TYPE_KILOMETERS"
    #[serde(rename="TYPE_KILOMETERS")]
    TYPEKILOMETERS,
}

impl AsRef<str> for MetricHeaderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricHeaderTypeEnum::METRICTYPEUNSPECIFIED => "METRIC_TYPE_UNSPECIFIED",
            MetricHeaderTypeEnum::TYPEINTEGER => "TYPE_INTEGER",
            MetricHeaderTypeEnum::TYPEFLOAT => "TYPE_FLOAT",
            MetricHeaderTypeEnum::TYPESECONDS => "TYPE_SECONDS",
            MetricHeaderTypeEnum::TYPEMILLISECONDS => "TYPE_MILLISECONDS",
            MetricHeaderTypeEnum::TYPEMINUTES => "TYPE_MINUTES",
            MetricHeaderTypeEnum::TYPEHOURS => "TYPE_HOURS",
            MetricHeaderTypeEnum::TYPESTANDARD => "TYPE_STANDARD",
            MetricHeaderTypeEnum::TYPECURRENCY => "TYPE_CURRENCY",
            MetricHeaderTypeEnum::TYPEFEET => "TYPE_FEET",
            MetricHeaderTypeEnum::TYPEMILES => "TYPE_MILES",
            MetricHeaderTypeEnum::TYPEMETERS => "TYPE_METERS",
            MetricHeaderTypeEnum::TYPEKILOMETERS => "TYPE_KILOMETERS",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricHeaderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_TYPE_UNSPECIFIED" => Ok(MetricHeaderTypeEnum::METRICTYPEUNSPECIFIED),
           "TYPE_INTEGER" => Ok(MetricHeaderTypeEnum::TYPEINTEGER),
           "TYPE_FLOAT" => Ok(MetricHeaderTypeEnum::TYPEFLOAT),
           "TYPE_SECONDS" => Ok(MetricHeaderTypeEnum::TYPESECONDS),
           "TYPE_MILLISECONDS" => Ok(MetricHeaderTypeEnum::TYPEMILLISECONDS),
           "TYPE_MINUTES" => Ok(MetricHeaderTypeEnum::TYPEMINUTES),
           "TYPE_HOURS" => Ok(MetricHeaderTypeEnum::TYPEHOURS),
           "TYPE_STANDARD" => Ok(MetricHeaderTypeEnum::TYPESTANDARD),
           "TYPE_CURRENCY" => Ok(MetricHeaderTypeEnum::TYPECURRENCY),
           "TYPE_FEET" => Ok(MetricHeaderTypeEnum::TYPEFEET),
           "TYPE_MILES" => Ok(MetricHeaderTypeEnum::TYPEMILES),
           "TYPE_METERS" => Ok(MetricHeaderTypeEnum::TYPEMETERS),
           "TYPE_KILOMETERS" => Ok(MetricHeaderTypeEnum::TYPEKILOMETERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricHeaderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricMetadataBlockedReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If reasons are specified, your access is blocked to this metric for this property. API requests from you to this property for this metric will succeed; however, the report will contain only zeros for this metric. API requests with metric filters on blocked metrics will fail. If reasons are empty, you have access to this metric. To learn more, see [Access and data-restriction management](https://support.google.com/analytics/answer/10851388).
pub enum MetricMetadataBlockedReasonsEnum {
    

    /// Will never be specified in API response.
    ///
    /// "BLOCKED_REASON_UNSPECIFIED"
    #[serde(rename="BLOCKED_REASON_UNSPECIFIED")]
    BLOCKEDREASONUNSPECIFIED,
    

    /// If present, your access is blocked to revenue related metrics for this property, and this metric is revenue related.
    ///
    /// "NO_REVENUE_METRICS"
    #[serde(rename="NO_REVENUE_METRICS")]
    NOREVENUEMETRICS,
    

    /// If present, your access is blocked to cost related metrics for this property, and this metric is cost related.
    ///
    /// "NO_COST_METRICS"
    #[serde(rename="NO_COST_METRICS")]
    NOCOSTMETRICS,
}

impl AsRef<str> for MetricMetadataBlockedReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricMetadataBlockedReasonsEnum::BLOCKEDREASONUNSPECIFIED => "BLOCKED_REASON_UNSPECIFIED",
            MetricMetadataBlockedReasonsEnum::NOREVENUEMETRICS => "NO_REVENUE_METRICS",
            MetricMetadataBlockedReasonsEnum::NOCOSTMETRICS => "NO_COST_METRICS",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricMetadataBlockedReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BLOCKED_REASON_UNSPECIFIED" => Ok(MetricMetadataBlockedReasonsEnum::BLOCKEDREASONUNSPECIFIED),
           "NO_REVENUE_METRICS" => Ok(MetricMetadataBlockedReasonsEnum::NOREVENUEMETRICS),
           "NO_COST_METRICS" => Ok(MetricMetadataBlockedReasonsEnum::NOCOSTMETRICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricMetadataBlockedReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricMetadataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this metric.
pub enum MetricMetadataTypeEnum {
    

    /// Unspecified type.
    ///
    /// "METRIC_TYPE_UNSPECIFIED"
    #[serde(rename="METRIC_TYPE_UNSPECIFIED")]
    METRICTYPEUNSPECIFIED,
    

    /// Integer type.
    ///
    /// "TYPE_INTEGER"
    #[serde(rename="TYPE_INTEGER")]
    TYPEINTEGER,
    

    /// Floating point type.
    ///
    /// "TYPE_FLOAT"
    #[serde(rename="TYPE_FLOAT")]
    TYPEFLOAT,
    

    /// A duration of seconds; a special floating point type.
    ///
    /// "TYPE_SECONDS"
    #[serde(rename="TYPE_SECONDS")]
    TYPESECONDS,
    

    /// A duration in milliseconds; a special floating point type.
    ///
    /// "TYPE_MILLISECONDS"
    #[serde(rename="TYPE_MILLISECONDS")]
    TYPEMILLISECONDS,
    

    /// A duration in minutes; a special floating point type.
    ///
    /// "TYPE_MINUTES"
    #[serde(rename="TYPE_MINUTES")]
    TYPEMINUTES,
    

    /// A duration in hours; a special floating point type.
    ///
    /// "TYPE_HOURS"
    #[serde(rename="TYPE_HOURS")]
    TYPEHOURS,
    

    /// A custom metric of standard type; a special floating point type.
    ///
    /// "TYPE_STANDARD"
    #[serde(rename="TYPE_STANDARD")]
    TYPESTANDARD,
    

    /// An amount of money; a special floating point type.
    ///
    /// "TYPE_CURRENCY"
    #[serde(rename="TYPE_CURRENCY")]
    TYPECURRENCY,
    

    /// A length in feet; a special floating point type.
    ///
    /// "TYPE_FEET"
    #[serde(rename="TYPE_FEET")]
    TYPEFEET,
    

    /// A length in miles; a special floating point type.
    ///
    /// "TYPE_MILES"
    #[serde(rename="TYPE_MILES")]
    TYPEMILES,
    

    /// A length in meters; a special floating point type.
    ///
    /// "TYPE_METERS"
    #[serde(rename="TYPE_METERS")]
    TYPEMETERS,
    

    /// A length in kilometers; a special floating point type.
    ///
    /// "TYPE_KILOMETERS"
    #[serde(rename="TYPE_KILOMETERS")]
    TYPEKILOMETERS,
}

impl AsRef<str> for MetricMetadataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricMetadataTypeEnum::METRICTYPEUNSPECIFIED => "METRIC_TYPE_UNSPECIFIED",
            MetricMetadataTypeEnum::TYPEINTEGER => "TYPE_INTEGER",
            MetricMetadataTypeEnum::TYPEFLOAT => "TYPE_FLOAT",
            MetricMetadataTypeEnum::TYPESECONDS => "TYPE_SECONDS",
            MetricMetadataTypeEnum::TYPEMILLISECONDS => "TYPE_MILLISECONDS",
            MetricMetadataTypeEnum::TYPEMINUTES => "TYPE_MINUTES",
            MetricMetadataTypeEnum::TYPEHOURS => "TYPE_HOURS",
            MetricMetadataTypeEnum::TYPESTANDARD => "TYPE_STANDARD",
            MetricMetadataTypeEnum::TYPECURRENCY => "TYPE_CURRENCY",
            MetricMetadataTypeEnum::TYPEFEET => "TYPE_FEET",
            MetricMetadataTypeEnum::TYPEMILES => "TYPE_MILES",
            MetricMetadataTypeEnum::TYPEMETERS => "TYPE_METERS",
            MetricMetadataTypeEnum::TYPEKILOMETERS => "TYPE_KILOMETERS",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricMetadataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_TYPE_UNSPECIFIED" => Ok(MetricMetadataTypeEnum::METRICTYPEUNSPECIFIED),
           "TYPE_INTEGER" => Ok(MetricMetadataTypeEnum::TYPEINTEGER),
           "TYPE_FLOAT" => Ok(MetricMetadataTypeEnum::TYPEFLOAT),
           "TYPE_SECONDS" => Ok(MetricMetadataTypeEnum::TYPESECONDS),
           "TYPE_MILLISECONDS" => Ok(MetricMetadataTypeEnum::TYPEMILLISECONDS),
           "TYPE_MINUTES" => Ok(MetricMetadataTypeEnum::TYPEMINUTES),
           "TYPE_HOURS" => Ok(MetricMetadataTypeEnum::TYPEHOURS),
           "TYPE_STANDARD" => Ok(MetricMetadataTypeEnum::TYPESTANDARD),
           "TYPE_CURRENCY" => Ok(MetricMetadataTypeEnum::TYPECURRENCY),
           "TYPE_FEET" => Ok(MetricMetadataTypeEnum::TYPEFEET),
           "TYPE_MILES" => Ok(MetricMetadataTypeEnum::TYPEMILES),
           "TYPE_METERS" => Ok(MetricMetadataTypeEnum::TYPEMETERS),
           "TYPE_KILOMETERS" => Ok(MetricMetadataTypeEnum::TYPEKILOMETERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricMetadataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NumericFilterOperationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operation type for this filter.
pub enum NumericFilterOperationEnum {
    

    /// Unspecified.
    ///
    /// "OPERATION_UNSPECIFIED"
    #[serde(rename="OPERATION_UNSPECIFIED")]
    OPERATIONUNSPECIFIED,
    

    /// Equal
    ///
    /// "EQUAL"
    #[serde(rename="EQUAL")]
    EQUAL,
    

    /// Less than
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// Less than or equal
    ///
    /// "LESS_THAN_OR_EQUAL"
    #[serde(rename="LESS_THAN_OR_EQUAL")]
    LESSTHANOREQUAL,
    

    /// Greater than
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// Greater than or equal
    ///
    /// "GREATER_THAN_OR_EQUAL"
    #[serde(rename="GREATER_THAN_OR_EQUAL")]
    GREATERTHANOREQUAL,
}

impl AsRef<str> for NumericFilterOperationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NumericFilterOperationEnum::OPERATIONUNSPECIFIED => "OPERATION_UNSPECIFIED",
            NumericFilterOperationEnum::EQUAL => "EQUAL",
            NumericFilterOperationEnum::LESSTHAN => "LESS_THAN",
            NumericFilterOperationEnum::LESSTHANOREQUAL => "LESS_THAN_OR_EQUAL",
            NumericFilterOperationEnum::GREATERTHAN => "GREATER_THAN",
            NumericFilterOperationEnum::GREATERTHANOREQUAL => "GREATER_THAN_OR_EQUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for NumericFilterOperationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATION_UNSPECIFIED" => Ok(NumericFilterOperationEnum::OPERATIONUNSPECIFIED),
           "EQUAL" => Ok(NumericFilterOperationEnum::EQUAL),
           "LESS_THAN" => Ok(NumericFilterOperationEnum::LESSTHAN),
           "LESS_THAN_OR_EQUAL" => Ok(NumericFilterOperationEnum::LESSTHANOREQUAL),
           "GREATER_THAN" => Ok(NumericFilterOperationEnum::GREATERTHAN),
           "GREATER_THAN_OR_EQUAL" => Ok(NumericFilterOperationEnum::GREATERTHANOREQUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NumericFilterOperationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PivotMetricAggregationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Aggregate the metrics by dimensions in this pivot using the specified metric_aggregations.
pub enum PivotMetricAggregationsEnum {
    

    /// Unspecified operator.
    ///
    /// "METRIC_AGGREGATION_UNSPECIFIED"
    #[serde(rename="METRIC_AGGREGATION_UNSPECIFIED")]
    METRICAGGREGATIONUNSPECIFIED,
    

    /// SUM operator.
    ///
    /// "TOTAL"
    #[serde(rename="TOTAL")]
    TOTAL,
    

    /// Minimum operator.
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
    

    /// Maximum operator.
    ///
    /// "MAXIMUM"
    #[serde(rename="MAXIMUM")]
    MAXIMUM,
    

    /// Count operator.
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
}

impl AsRef<str> for PivotMetricAggregationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PivotMetricAggregationsEnum::METRICAGGREGATIONUNSPECIFIED => "METRIC_AGGREGATION_UNSPECIFIED",
            PivotMetricAggregationsEnum::TOTAL => "TOTAL",
            PivotMetricAggregationsEnum::MINIMUM => "MINIMUM",
            PivotMetricAggregationsEnum::MAXIMUM => "MAXIMUM",
            PivotMetricAggregationsEnum::COUNT => "COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for PivotMetricAggregationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_AGGREGATION_UNSPECIFIED" => Ok(PivotMetricAggregationsEnum::METRICAGGREGATIONUNSPECIFIED),
           "TOTAL" => Ok(PivotMetricAggregationsEnum::TOTAL),
           "MINIMUM" => Ok(PivotMetricAggregationsEnum::MINIMUM),
           "MAXIMUM" => Ok(PivotMetricAggregationsEnum::MAXIMUM),
           "COUNT" => Ok(PivotMetricAggregationsEnum::COUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PivotMetricAggregationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RunRealtimeReportRequestMetricAggregationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)".
pub enum RunRealtimeReportRequestMetricAggregationsEnum {
    

    /// Unspecified operator.
    ///
    /// "METRIC_AGGREGATION_UNSPECIFIED"
    #[serde(rename="METRIC_AGGREGATION_UNSPECIFIED")]
    METRICAGGREGATIONUNSPECIFIED,
    

    /// SUM operator.
    ///
    /// "TOTAL"
    #[serde(rename="TOTAL")]
    TOTAL,
    

    /// Minimum operator.
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
    

    /// Maximum operator.
    ///
    /// "MAXIMUM"
    #[serde(rename="MAXIMUM")]
    MAXIMUM,
    

    /// Count operator.
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
}

impl AsRef<str> for RunRealtimeReportRequestMetricAggregationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RunRealtimeReportRequestMetricAggregationsEnum::METRICAGGREGATIONUNSPECIFIED => "METRIC_AGGREGATION_UNSPECIFIED",
            RunRealtimeReportRequestMetricAggregationsEnum::TOTAL => "TOTAL",
            RunRealtimeReportRequestMetricAggregationsEnum::MINIMUM => "MINIMUM",
            RunRealtimeReportRequestMetricAggregationsEnum::MAXIMUM => "MAXIMUM",
            RunRealtimeReportRequestMetricAggregationsEnum::COUNT => "COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for RunRealtimeReportRequestMetricAggregationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_AGGREGATION_UNSPECIFIED" => Ok(RunRealtimeReportRequestMetricAggregationsEnum::METRICAGGREGATIONUNSPECIFIED),
           "TOTAL" => Ok(RunRealtimeReportRequestMetricAggregationsEnum::TOTAL),
           "MINIMUM" => Ok(RunRealtimeReportRequestMetricAggregationsEnum::MINIMUM),
           "MAXIMUM" => Ok(RunRealtimeReportRequestMetricAggregationsEnum::MAXIMUM),
           "COUNT" => Ok(RunRealtimeReportRequestMetricAggregationsEnum::COUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RunRealtimeReportRequestMetricAggregationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RunReportRequestMetricAggregationsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)".
pub enum RunReportRequestMetricAggregationsEnum {
    

    /// Unspecified operator.
    ///
    /// "METRIC_AGGREGATION_UNSPECIFIED"
    #[serde(rename="METRIC_AGGREGATION_UNSPECIFIED")]
    METRICAGGREGATIONUNSPECIFIED,
    

    /// SUM operator.
    ///
    /// "TOTAL"
    #[serde(rename="TOTAL")]
    TOTAL,
    

    /// Minimum operator.
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
    

    /// Maximum operator.
    ///
    /// "MAXIMUM"
    #[serde(rename="MAXIMUM")]
    MAXIMUM,
    

    /// Count operator.
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
}

impl AsRef<str> for RunReportRequestMetricAggregationsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RunReportRequestMetricAggregationsEnum::METRICAGGREGATIONUNSPECIFIED => "METRIC_AGGREGATION_UNSPECIFIED",
            RunReportRequestMetricAggregationsEnum::TOTAL => "TOTAL",
            RunReportRequestMetricAggregationsEnum::MINIMUM => "MINIMUM",
            RunReportRequestMetricAggregationsEnum::MAXIMUM => "MAXIMUM",
            RunReportRequestMetricAggregationsEnum::COUNT => "COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for RunReportRequestMetricAggregationsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_AGGREGATION_UNSPECIFIED" => Ok(RunReportRequestMetricAggregationsEnum::METRICAGGREGATIONUNSPECIFIED),
           "TOTAL" => Ok(RunReportRequestMetricAggregationsEnum::TOTAL),
           "MINIMUM" => Ok(RunReportRequestMetricAggregationsEnum::MINIMUM),
           "MAXIMUM" => Ok(RunReportRequestMetricAggregationsEnum::MAXIMUM),
           "COUNT" => Ok(RunReportRequestMetricAggregationsEnum::COUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RunReportRequestMetricAggregationsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StringFilterMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The match type for this filter.
pub enum StringFilterMatchTypeEnum {
    

    /// Unspecified
    ///
    /// "MATCH_TYPE_UNSPECIFIED"
    #[serde(rename="MATCH_TYPE_UNSPECIFIED")]
    MATCHTYPEUNSPECIFIED,
    

    /// Exact match of the string value.
    ///
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    

    /// Begins with the string value.
    ///
    /// "BEGINS_WITH"
    #[serde(rename="BEGINS_WITH")]
    BEGINSWITH,
    

    /// Ends with the string value.
    ///
    /// "ENDS_WITH"
    #[serde(rename="ENDS_WITH")]
    ENDSWITH,
    

    /// Contains the string value.
    ///
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    

    /// Full match for the regular expression with the string value.
    ///
    /// "FULL_REGEXP"
    #[serde(rename="FULL_REGEXP")]
    FULLREGEXP,
    

    /// Partial match for the regular expression with the string value.
    ///
    /// "PARTIAL_REGEXP"
    #[serde(rename="PARTIAL_REGEXP")]
    PARTIALREGEXP,
}

impl AsRef<str> for StringFilterMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StringFilterMatchTypeEnum::MATCHTYPEUNSPECIFIED => "MATCH_TYPE_UNSPECIFIED",
            StringFilterMatchTypeEnum::EXACT => "EXACT",
            StringFilterMatchTypeEnum::BEGINSWITH => "BEGINS_WITH",
            StringFilterMatchTypeEnum::ENDSWITH => "ENDS_WITH",
            StringFilterMatchTypeEnum::CONTAINS => "CONTAINS",
            StringFilterMatchTypeEnum::FULLREGEXP => "FULL_REGEXP",
            StringFilterMatchTypeEnum::PARTIALREGEXP => "PARTIAL_REGEXP",
        }
    }
}

impl std::convert::TryFrom< &str> for StringFilterMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCH_TYPE_UNSPECIFIED" => Ok(StringFilterMatchTypeEnum::MATCHTYPEUNSPECIFIED),
           "EXACT" => Ok(StringFilterMatchTypeEnum::EXACT),
           "BEGINS_WITH" => Ok(StringFilterMatchTypeEnum::BEGINSWITH),
           "ENDS_WITH" => Ok(StringFilterMatchTypeEnum::ENDSWITH),
           "CONTAINS" => Ok(StringFilterMatchTypeEnum::CONTAINS),
           "FULL_REGEXP" => Ok(StringFilterMatchTypeEnum::FULLREGEXP),
           "PARTIAL_REGEXP" => Ok(StringFilterMatchTypeEnum::PARTIALREGEXP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StringFilterMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


