use super::*;



// region DefaultSinkConfigModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Determines the behavior to apply to the built-in _Default sink inclusion filter.Exclusions are always appended, as built-in _Default sinks have no exclusions.
pub enum DefaultSinkConfigModeEnum {
    

    /// The filter's write mode is unspecified. This mode must not be used.
    ///
    /// "FILTER_WRITE_MODE_UNSPECIFIED"
    #[serde(rename="FILTER_WRITE_MODE_UNSPECIFIED")]
    FILTERWRITEMODEUNSPECIFIED,
    

    /// The contents of filter will be appended to the built-in _Default sink filter. Using the append mode with an empty filter will keep the sink inclusion filter unchanged.
    ///
    /// "APPEND"
    #[serde(rename="APPEND")]
    APPEND,
    

    /// The contents of filter will overwrite the built-in _Default sink filter.
    ///
    /// "OVERWRITE"
    #[serde(rename="OVERWRITE")]
    OVERWRITE,
}

impl AsRef<str> for DefaultSinkConfigModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DefaultSinkConfigModeEnum::FILTERWRITEMODEUNSPECIFIED => "FILTER_WRITE_MODE_UNSPECIFIED",
            DefaultSinkConfigModeEnum::APPEND => "APPEND",
            DefaultSinkConfigModeEnum::OVERWRITE => "OVERWRITE",
        }
    }
}

impl std::convert::TryFrom< &str> for DefaultSinkConfigModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_WRITE_MODE_UNSPECIFIED" => Ok(DefaultSinkConfigModeEnum::FILTERWRITEMODEUNSPECIFIED),
           "APPEND" => Ok(DefaultSinkConfigModeEnum::APPEND),
           "OVERWRITE" => Ok(DefaultSinkConfigModeEnum::OVERWRITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DefaultSinkConfigModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IndexConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of data in this index.
pub enum IndexConfigTypeEnum {
    

    /// The index's type is unspecified.
    ///
    /// "INDEX_TYPE_UNSPECIFIED"
    #[serde(rename="INDEX_TYPE_UNSPECIFIED")]
    INDEXTYPEUNSPECIFIED,
    

    /// The index is a string-type index.
    ///
    /// "INDEX_TYPE_STRING"
    #[serde(rename="INDEX_TYPE_STRING")]
    INDEXTYPESTRING,
    

    /// The index is a integer-type index.
    ///
    /// "INDEX_TYPE_INTEGER"
    #[serde(rename="INDEX_TYPE_INTEGER")]
    INDEXTYPEINTEGER,
}

impl AsRef<str> for IndexConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IndexConfigTypeEnum::INDEXTYPEUNSPECIFIED => "INDEX_TYPE_UNSPECIFIED",
            IndexConfigTypeEnum::INDEXTYPESTRING => "INDEX_TYPE_STRING",
            IndexConfigTypeEnum::INDEXTYPEINTEGER => "INDEX_TYPE_INTEGER",
        }
    }
}

impl std::convert::TryFrom< &str> for IndexConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INDEX_TYPE_UNSPECIFIED" => Ok(IndexConfigTypeEnum::INDEXTYPEUNSPECIFIED),
           "INDEX_TYPE_STRING" => Ok(IndexConfigTypeEnum::INDEXTYPESTRING),
           "INDEX_TYPE_INTEGER" => Ok(IndexConfigTypeEnum::INDEXTYPEINTEGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IndexConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LabelDescriptorValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of data that can be assigned to the label.
pub enum LabelDescriptorValueTypeEnum {
    

    /// A variable-length string. This is the default.
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


// region LinkLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The resource lifecycle state.
pub enum LinkLifecycleStateEnum {
    

    /// Unspecified state. This is only used/useful for distinguishing unset values.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource has been marked for deletion by the user. For some resources (e.g. buckets), this can be reversed by an un-delete operation.
    ///
    /// "DELETE_REQUESTED"
    #[serde(rename="DELETE_REQUESTED")]
    DELETEREQUESTED,
    

    /// The resource has been marked for an update by the user. It will remain in this state until the update is complete.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource has been marked for creation by the user. It will remain in this state until the creation is complete.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is in an INTERNAL error state.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for LinkLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            LinkLifecycleStateEnum::ACTIVE => "ACTIVE",
            LinkLifecycleStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
            LinkLifecycleStateEnum::UPDATING => "UPDATING",
            LinkLifecycleStateEnum::CREATING => "CREATING",
            LinkLifecycleStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(LinkLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "ACTIVE" => Ok(LinkLifecycleStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(LinkLifecycleStateEnum::DELETEREQUESTED),
           "UPDATING" => Ok(LinkLifecycleStateEnum::UPDATING),
           "CREATING" => Ok(LinkLifecycleStateEnum::CREATING),
           "FAILED" => Ok(LinkLifecycleStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogBucketLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The bucket lifecycle state.
pub enum LogBucketLifecycleStateEnum {
    

    /// Unspecified state. This is only used/useful for distinguishing unset values.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource has been marked for deletion by the user. For some resources (e.g. buckets), this can be reversed by an un-delete operation.
    ///
    /// "DELETE_REQUESTED"
    #[serde(rename="DELETE_REQUESTED")]
    DELETEREQUESTED,
    

    /// The resource has been marked for an update by the user. It will remain in this state until the update is complete.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The resource has been marked for creation by the user. It will remain in this state until the creation is complete.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The resource is in an INTERNAL error state.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for LogBucketLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogBucketLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            LogBucketLifecycleStateEnum::ACTIVE => "ACTIVE",
            LogBucketLifecycleStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
            LogBucketLifecycleStateEnum::UPDATING => "UPDATING",
            LogBucketLifecycleStateEnum::CREATING => "CREATING",
            LogBucketLifecycleStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for LogBucketLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(LogBucketLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "ACTIVE" => Ok(LogBucketLifecycleStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(LogBucketLifecycleStateEnum::DELETEREQUESTED),
           "UPDATING" => Ok(LogBucketLifecycleStateEnum::UPDATING),
           "CREATING" => Ok(LogBucketLifecycleStateEnum::CREATING),
           "FAILED" => Ok(LogBucketLifecycleStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogBucketLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogEntrySeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The severity of the log entry. The default value is LogSeverity.DEFAULT.
pub enum LogEntrySeverityEnum {
    

    /// (0) The log entry has no assigned severity level.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// (100) Debug or trace information.
    ///
    /// "DEBUG"
    #[serde(rename="DEBUG")]
    DEBUG,
    

    /// (200) Routine information, such as ongoing status or performance.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// (300) Normal but significant events, such as start up, shut down, or a configuration change.
    ///
    /// "NOTICE"
    #[serde(rename="NOTICE")]
    NOTICE,
    

    /// (400) Warning events might cause problems.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// (500) Error events are likely to cause problems.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// (600) Critical events cause more severe problems or outages.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
    

    /// (700) A person must take an action immediately.
    ///
    /// "ALERT"
    #[serde(rename="ALERT")]
    ALERT,
    

    /// (800) One or more systems are unusable.
    ///
    /// "EMERGENCY"
    #[serde(rename="EMERGENCY")]
    EMERGENCY,
}

impl AsRef<str> for LogEntrySeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogEntrySeverityEnum::DEFAULT => "DEFAULT",
            LogEntrySeverityEnum::DEBUG => "DEBUG",
            LogEntrySeverityEnum::INFO => "INFO",
            LogEntrySeverityEnum::NOTICE => "NOTICE",
            LogEntrySeverityEnum::WARNING => "WARNING",
            LogEntrySeverityEnum::ERROR => "ERROR",
            LogEntrySeverityEnum::CRITICAL => "CRITICAL",
            LogEntrySeverityEnum::ALERT => "ALERT",
            LogEntrySeverityEnum::EMERGENCY => "EMERGENCY",
        }
    }
}

impl std::convert::TryFrom< &str> for LogEntrySeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(LogEntrySeverityEnum::DEFAULT),
           "DEBUG" => Ok(LogEntrySeverityEnum::DEBUG),
           "INFO" => Ok(LogEntrySeverityEnum::INFO),
           "NOTICE" => Ok(LogEntrySeverityEnum::NOTICE),
           "WARNING" => Ok(LogEntrySeverityEnum::WARNING),
           "ERROR" => Ok(LogEntrySeverityEnum::ERROR),
           "CRITICAL" => Ok(LogEntrySeverityEnum::CRITICAL),
           "ALERT" => Ok(LogEntrySeverityEnum::ALERT),
           "EMERGENCY" => Ok(LogEntrySeverityEnum::EMERGENCY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogEntrySeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogMetricVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. The API version that created or updated this metric. The v2 format is used by default and cannot be changed.
pub enum LogMetricVersionEnum {
    

    /// Logging API v2.
    ///
    /// "V2"
    #[serde(rename="V2")]
    V2,
    

    /// Logging API v1.
    ///
    /// "V1"
    #[serde(rename="V1")]
    V1,
}

impl AsRef<str> for LogMetricVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogMetricVersionEnum::V2 => "V2",
            LogMetricVersionEnum::V1 => "V1",
        }
    }
}

impl std::convert::TryFrom< &str> for LogMetricVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "V2" => Ok(LogMetricVersionEnum::V2),
           "V1" => Ok(LogMetricVersionEnum::V1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogMetricVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogSinkOutputVersionFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. This field is unused.
pub enum LogSinkOutputVersionFormatEnum {
    

    /// An unspecified format version that will default to V2.
    ///
    /// "VERSION_FORMAT_UNSPECIFIED"
    #[serde(rename="VERSION_FORMAT_UNSPECIFIED")]
    VERSIONFORMATUNSPECIFIED,
    

    /// LogEntry version 2 format.
    ///
    /// "V2"
    #[serde(rename="V2")]
    V2,
    

    /// LogEntry version 1 format.
    ///
    /// "V1"
    #[serde(rename="V1")]
    V1,
}

impl AsRef<str> for LogSinkOutputVersionFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogSinkOutputVersionFormatEnum::VERSIONFORMATUNSPECIFIED => "VERSION_FORMAT_UNSPECIFIED",
            LogSinkOutputVersionFormatEnum::V2 => "V2",
            LogSinkOutputVersionFormatEnum::V1 => "V1",
        }
    }
}

impl std::convert::TryFrom< &str> for LogSinkOutputVersionFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_FORMAT_UNSPECIFIED" => Ok(LogSinkOutputVersionFormatEnum::VERSIONFORMATUNSPECIFIED),
           "V2" => Ok(LogSinkOutputVersionFormatEnum::V2),
           "V1" => Ok(LogSinkOutputVersionFormatEnum::V1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogSinkOutputVersionFormatEnum {
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


// region SavedQueryVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The visibility status of this query, which determines its ownership.
pub enum SavedQueryVisibilityEnum {
    

    /// The saved query visibility is unspecified. A CreateSavedQuery request with an unspecified visibility will be rejected.
    ///
    /// "VISIBILITY_UNSPECIFIED"
    #[serde(rename="VISIBILITY_UNSPECIFIED")]
    VISIBILITYUNSPECIFIED,
    

    /// The saved query is only visible to the user that created it.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
    

    /// The saved query is visible to anyone in the project.
    ///
    /// "SHARED"
    #[serde(rename="SHARED")]
    SHARED,
}

impl AsRef<str> for SavedQueryVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SavedQueryVisibilityEnum::VISIBILITYUNSPECIFIED => "VISIBILITY_UNSPECIFIED",
            SavedQueryVisibilityEnum::PRIVATE => "PRIVATE",
            SavedQueryVisibilityEnum::SHARED => "SHARED",
        }
    }
}

impl std::convert::TryFrom< &str> for SavedQueryVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VISIBILITY_UNSPECIFIED" => Ok(SavedQueryVisibilityEnum::VISIBILITYUNSPECIFIED),
           "PRIVATE" => Ok(SavedQueryVisibilityEnum::PRIVATE),
           "SHARED" => Ok(SavedQueryVisibilityEnum::SHARED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SavedQueryVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SuppressionInfoReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that entries were omitted from the session.
pub enum SuppressionInfoReasonEnum {
    

    /// Unexpected default.
    ///
    /// "REASON_UNSPECIFIED"
    #[serde(rename="REASON_UNSPECIFIED")]
    REASONUNSPECIFIED,
    

    /// Indicates suppression occurred due to relevant entries being received in excess of rate limits. For quotas and limits, see Logging API quotas and limits (https://cloud.google.com/logging/quotas#api-limits).
    ///
    /// "RATE_LIMIT"
    #[serde(rename="RATE_LIMIT")]
    RATELIMIT,
    

    /// Indicates suppression occurred due to the client not consuming responses quickly enough.
    ///
    /// "NOT_CONSUMED"
    #[serde(rename="NOT_CONSUMED")]
    NOTCONSUMED,
}

impl AsRef<str> for SuppressionInfoReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SuppressionInfoReasonEnum::REASONUNSPECIFIED => "REASON_UNSPECIFIED",
            SuppressionInfoReasonEnum::RATELIMIT => "RATE_LIMIT",
            SuppressionInfoReasonEnum::NOTCONSUMED => "NOT_CONSUMED",
        }
    }
}

impl std::convert::TryFrom< &str> for SuppressionInfoReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNSPECIFIED" => Ok(SuppressionInfoReasonEnum::REASONUNSPECIFIED),
           "RATE_LIMIT" => Ok(SuppressionInfoReasonEnum::RATELIMIT),
           "NOT_CONSUMED" => Ok(SuppressionInfoReasonEnum::NOTCONSUMED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SuppressionInfoReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


