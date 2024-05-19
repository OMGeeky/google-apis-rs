use super::*;



// region ErrorGroupResolutionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error group's resolution status. An unspecified resolution status will be interpreted as OPEN
pub enum ErrorGroupResolutionStatusEnum {
    

    /// Status is unknown. When left unspecified in requests, it is treated like OPEN.
    ///
    /// "RESOLUTION_STATUS_UNSPECIFIED"
    #[serde(rename="RESOLUTION_STATUS_UNSPECIFIED")]
    RESOLUTIONSTATUSUNSPECIFIED,
    

    /// The error group is not being addressed. This is the default for new groups. It is also used for errors re-occurring after marked RESOLVED.
    ///
    /// "OPEN"
    #[serde(rename="OPEN")]
    OPEN,
    

    /// Error Group manually acknowledged, it can have an issue link attached.
    ///
    /// "ACKNOWLEDGED"
    #[serde(rename="ACKNOWLEDGED")]
    ACKNOWLEDGED,
    

    /// Error Group manually resolved, more events for this group are not expected to occur.
    ///
    /// "RESOLVED"
    #[serde(rename="RESOLVED")]
    RESOLVED,
    

    /// The error group is muted and excluded by default on group stats requests.
    ///
    /// "MUTED"
    #[serde(rename="MUTED")]
    MUTED,
}

impl AsRef<str> for ErrorGroupResolutionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ErrorGroupResolutionStatusEnum::RESOLUTIONSTATUSUNSPECIFIED => "RESOLUTION_STATUS_UNSPECIFIED",
            ErrorGroupResolutionStatusEnum::OPEN => "OPEN",
            ErrorGroupResolutionStatusEnum::ACKNOWLEDGED => "ACKNOWLEDGED",
            ErrorGroupResolutionStatusEnum::RESOLVED => "RESOLVED",
            ErrorGroupResolutionStatusEnum::MUTED => "MUTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ErrorGroupResolutionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOLUTION_STATUS_UNSPECIFIED" => Ok(ErrorGroupResolutionStatusEnum::RESOLUTIONSTATUSUNSPECIFIED),
           "OPEN" => Ok(ErrorGroupResolutionStatusEnum::OPEN),
           "ACKNOWLEDGED" => Ok(ErrorGroupResolutionStatusEnum::ACKNOWLEDGED),
           "RESOLVED" => Ok(ErrorGroupResolutionStatusEnum::RESOLVED),
           "MUTED" => Ok(ErrorGroupResolutionStatusEnum::MUTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ErrorGroupResolutionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectTimeRangePeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restricts the query to the specified time range.
pub enum ProjectTimeRangePeriodEnum {
    

    /// Do not use.
    ///
    /// "PERIOD_UNSPECIFIED"
    #[serde(rename="PERIOD_UNSPECIFIED")]
    PERIODUNSPECIFIED,
    

    /// Retrieve data for the last hour. Recommended minimum timed count duration: 1 min.
    ///
    /// "PERIOD_1_HOUR"
    #[serde(rename="PERIOD_1_HOUR")]
    PERIOD1HOUR,
    

    /// Retrieve data for the last 6 hours. Recommended minimum timed count duration: 10 min.
    ///
    /// "PERIOD_6_HOURS"
    #[serde(rename="PERIOD_6_HOURS")]
    PERIOD6HOURS,
    

    /// Retrieve data for the last day. Recommended minimum timed count duration: 1 hour.
    ///
    /// "PERIOD_1_DAY"
    #[serde(rename="PERIOD_1_DAY")]
    PERIOD1DAY,
    

    /// Retrieve data for the last week. Recommended minimum timed count duration: 6 hours.
    ///
    /// "PERIOD_1_WEEK"
    #[serde(rename="PERIOD_1_WEEK")]
    PERIOD1WEEK,
    

    /// Retrieve data for the last 30 days. Recommended minimum timed count duration: 1 day.
    ///
    /// "PERIOD_30_DAYS"
    #[serde(rename="PERIOD_30_DAYS")]
    PERIOD30DAYS,
}

impl AsRef<str> for ProjectTimeRangePeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectTimeRangePeriodEnum::PERIODUNSPECIFIED => "PERIOD_UNSPECIFIED",
            ProjectTimeRangePeriodEnum::PERIOD1HOUR => "PERIOD_1_HOUR",
            ProjectTimeRangePeriodEnum::PERIOD6HOURS => "PERIOD_6_HOURS",
            ProjectTimeRangePeriodEnum::PERIOD1DAY => "PERIOD_1_DAY",
            ProjectTimeRangePeriodEnum::PERIOD1WEEK => "PERIOD_1_WEEK",
            ProjectTimeRangePeriodEnum::PERIOD30DAYS => "PERIOD_30_DAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectTimeRangePeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIOD_UNSPECIFIED" => Ok(ProjectTimeRangePeriodEnum::PERIODUNSPECIFIED),
           "PERIOD_1_HOUR" => Ok(ProjectTimeRangePeriodEnum::PERIOD1HOUR),
           "PERIOD_6_HOURS" => Ok(ProjectTimeRangePeriodEnum::PERIOD6HOURS),
           "PERIOD_1_DAY" => Ok(ProjectTimeRangePeriodEnum::PERIOD1DAY),
           "PERIOD_1_WEEK" => Ok(ProjectTimeRangePeriodEnum::PERIOD1WEEK),
           "PERIOD_30_DAYS" => Ok(ProjectTimeRangePeriodEnum::PERIOD30DAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectTimeRangePeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The alignment of the timed counts to be returned. Default is `ALIGNMENT_EQUAL_AT_END`.
pub enum ProjectAlignmentEnum {
    

    /// No alignment specified.
    ///
    /// "ERROR_COUNT_ALIGNMENT_UNSPECIFIED"
    #[serde(rename="ERROR_COUNT_ALIGNMENT_UNSPECIFIED")]
    ERRORCOUNTALIGNMENTUNSPECIFIED,
    

    /// The time periods shall be consecutive, have width equal to the requested duration, and be aligned at the alignment_time provided in the request. The alignment_time does not have to be inside the query period but even if it is outside, only time periods are returned which overlap with the query period. A rounded alignment will typically result in a different size of the first or the last time period.
    ///
    /// "ALIGNMENT_EQUAL_ROUNDED"
    #[serde(rename="ALIGNMENT_EQUAL_ROUNDED")]
    ALIGNMENTEQUALROUNDED,
    

    /// The time periods shall be consecutive, have width equal to the requested duration, and be aligned at the end of the requested time period. This can result in a different size of the first time period.
    ///
    /// "ALIGNMENT_EQUAL_AT_END"
    #[serde(rename="ALIGNMENT_EQUAL_AT_END")]
    ALIGNMENTEQUALATEND,
}

impl AsRef<str> for ProjectAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectAlignmentEnum::ERRORCOUNTALIGNMENTUNSPECIFIED => "ERROR_COUNT_ALIGNMENT_UNSPECIFIED",
            ProjectAlignmentEnum::ALIGNMENTEQUALROUNDED => "ALIGNMENT_EQUAL_ROUNDED",
            ProjectAlignmentEnum::ALIGNMENTEQUALATEND => "ALIGNMENT_EQUAL_AT_END",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_COUNT_ALIGNMENT_UNSPECIFIED" => Ok(ProjectAlignmentEnum::ERRORCOUNTALIGNMENTUNSPECIFIED),
           "ALIGNMENT_EQUAL_ROUNDED" => Ok(ProjectAlignmentEnum::ALIGNMENTEQUALROUNDED),
           "ALIGNMENT_EQUAL_AT_END" => Ok(ProjectAlignmentEnum::ALIGNMENTEQUALATEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The sort order in which the results are returned. Default is `COUNT_DESC`.
pub enum ProjectOrderEnum {
    

    /// No group order specified.
    ///
    /// "GROUP_ORDER_UNSPECIFIED"
    #[serde(rename="GROUP_ORDER_UNSPECIFIED")]
    GROUPORDERUNSPECIFIED,
    

    /// Total count of errors in the given time window in descending order.
    ///
    /// "COUNT_DESC"
    #[serde(rename="COUNT_DESC")]
    COUNTDESC,
    

    /// Timestamp when the group was last seen in the given time window in descending order.
    ///
    /// "LAST_SEEN_DESC"
    #[serde(rename="LAST_SEEN_DESC")]
    LASTSEENDESC,
    

    /// Timestamp when the group was created in descending order.
    ///
    /// "CREATED_DESC"
    #[serde(rename="CREATED_DESC")]
    CREATEDDESC,
    

    /// Number of affected users in the given time window in descending order.
    ///
    /// "AFFECTED_USERS_DESC"
    #[serde(rename="AFFECTED_USERS_DESC")]
    AFFECTEDUSERSDESC,
}

impl AsRef<str> for ProjectOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectOrderEnum::GROUPORDERUNSPECIFIED => "GROUP_ORDER_UNSPECIFIED",
            ProjectOrderEnum::COUNTDESC => "COUNT_DESC",
            ProjectOrderEnum::LASTSEENDESC => "LAST_SEEN_DESC",
            ProjectOrderEnum::CREATEDDESC => "CREATED_DESC",
            ProjectOrderEnum::AFFECTEDUSERSDESC => "AFFECTED_USERS_DESC",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GROUP_ORDER_UNSPECIFIED" => Ok(ProjectOrderEnum::GROUPORDERUNSPECIFIED),
           "COUNT_DESC" => Ok(ProjectOrderEnum::COUNTDESC),
           "LAST_SEEN_DESC" => Ok(ProjectOrderEnum::LASTSEENDESC),
           "CREATED_DESC" => Ok(ProjectOrderEnum::CREATEDDESC),
           "AFFECTED_USERS_DESC" => Ok(ProjectOrderEnum::AFFECTEDUSERSDESC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


