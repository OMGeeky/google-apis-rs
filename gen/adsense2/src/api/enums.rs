use super::*;



// region AccountStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the account.
pub enum AccountStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The account is open and ready to serve ads.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// There are some issues with this account. Publishers should visit AdSense in order to fix the account.
    ///
    /// "NEEDS_ATTENTION"
    #[serde(rename="NEEDS_ATTENTION")]
    NEEDSATTENTION,
    

    /// The account is closed and can't serve ads.
    ///
    /// "CLOSED"
    #[serde(rename="CLOSED")]
    CLOSED,
}

impl AsRef<str> for AccountStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AccountStateEnum::READY => "READY",
            AccountStateEnum::NEEDSATTENTION => "NEEDS_ATTENTION",
            AccountStateEnum::CLOSED => "CLOSED",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AccountStateEnum::STATEUNSPECIFIED),
           "READY" => Ok(AccountStateEnum::READY),
           "NEEDS_ATTENTION" => Ok(AccountStateEnum::NEEDSATTENTION),
           "CLOSED" => Ok(AccountStateEnum::CLOSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdClientStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the ad client.
pub enum AdClientStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The ad client is ready to show ads.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Running some checks on the ad client before it is ready to serve ads.
    ///
    /// "GETTING_READY"
    #[serde(rename="GETTING_READY")]
    GETTINGREADY,
    

    /// The ad client hasn't been checked yet. There are tasks pending before AdSense will start the review.
    ///
    /// "REQUIRES_REVIEW"
    #[serde(rename="REQUIRES_REVIEW")]
    REQUIRESREVIEW,
}

impl AsRef<str> for AdClientStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdClientStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AdClientStateEnum::READY => "READY",
            AdClientStateEnum::GETTINGREADY => "GETTING_READY",
            AdClientStateEnum::REQUIRESREVIEW => "REQUIRES_REVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for AdClientStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AdClientStateEnum::STATEUNSPECIFIED),
           "READY" => Ok(AdClientStateEnum::READY),
           "GETTING_READY" => Ok(AdClientStateEnum::GETTINGREADY),
           "REQUIRES_REVIEW" => Ok(AdClientStateEnum::REQUIRESREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdClientStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdUnitStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. State of the ad unit.
pub enum AdUnitStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Ad unit has been activated by the user.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Ad unit has been archived by the user. Note that archived ad units are only removed from the default view in the UI. Archived ad units can still serve ads.
    ///
    /// "ARCHIVED"
    #[serde(rename="ARCHIVED")]
    ARCHIVED,
}

impl AsRef<str> for AdUnitStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdUnitStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AdUnitStateEnum::ACTIVE => "ACTIVE",
            AdUnitStateEnum::ARCHIVED => "ARCHIVED",
        }
    }
}

impl std::convert::TryFrom< &str> for AdUnitStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AdUnitStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(AdUnitStateEnum::ACTIVE),
           "ARCHIVED" => Ok(AdUnitStateEnum::ARCHIVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdUnitStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AlertSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Severity of this alert.
pub enum AlertSeverityEnum {
    

    /// Unspecified severity.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Info.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Warning.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Severe.
    ///
    /// "SEVERE"
    #[serde(rename="SEVERE")]
    SEVERE,
}

impl AsRef<str> for AlertSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AlertSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            AlertSeverityEnum::INFO => "INFO",
            AlertSeverityEnum::WARNING => "WARNING",
            AlertSeverityEnum::SEVERE => "SEVERE",
        }
    }
}

impl std::convert::TryFrom< &str> for AlertSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(AlertSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(AlertSeverityEnum::INFO),
           "WARNING" => Ok(AlertSeverityEnum::WARNING),
           "SEVERE" => Ok(AlertSeverityEnum::SEVERE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AlertSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentAdsSettingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of the ad unit.
pub enum ContentAdsSettingTypeEnum {
    

    /// Unspecified ad unit type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Display ad unit.
    ///
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    

    /// In-feed ad unit.
    ///
    /// "FEED"
    #[serde(rename="FEED")]
    FEED,
    

    /// In-article ad unit.
    ///
    /// "ARTICLE"
    #[serde(rename="ARTICLE")]
    ARTICLE,
    

    /// Matched content unit.
    ///
    /// "MATCHED_CONTENT"
    #[serde(rename="MATCHED_CONTENT")]
    MATCHEDCONTENT,
    

    /// Link ad unit. Note that link ad units have now been retired, see https://support.google.com/adsense/answer/9987221.
    ///
    /// "LINK"
    #[serde(rename="LINK")]
    LINK,
}

impl AsRef<str> for ContentAdsSettingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentAdsSettingTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ContentAdsSettingTypeEnum::DISPLAY => "DISPLAY",
            ContentAdsSettingTypeEnum::FEED => "FEED",
            ContentAdsSettingTypeEnum::ARTICLE => "ARTICLE",
            ContentAdsSettingTypeEnum::MATCHEDCONTENT => "MATCHED_CONTENT",
            ContentAdsSettingTypeEnum::LINK => "LINK",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentAdsSettingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ContentAdsSettingTypeEnum::TYPEUNSPECIFIED),
           "DISPLAY" => Ok(ContentAdsSettingTypeEnum::DISPLAY),
           "FEED" => Ok(ContentAdsSettingTypeEnum::FEED),
           "ARTICLE" => Ok(ContentAdsSettingTypeEnum::ARTICLE),
           "MATCHED_CONTENT" => Ok(ContentAdsSettingTypeEnum::MATCHEDCONTENT),
           "LINK" => Ok(ContentAdsSettingTypeEnum::LINK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentAdsSettingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HeaderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of the header.
pub enum HeaderTypeEnum {
    

    /// Unspecified header.
    ///
    /// "HEADER_TYPE_UNSPECIFIED"
    #[serde(rename="HEADER_TYPE_UNSPECIFIED")]
    HEADERTYPEUNSPECIFIED,
    

    /// Dimension header type.
    ///
    /// "DIMENSION"
    #[serde(rename="DIMENSION")]
    DIMENSION,
    

    /// Tally header type.
    ///
    /// "METRIC_TALLY"
    #[serde(rename="METRIC_TALLY")]
    METRICTALLY,
    

    /// Ratio header type.
    ///
    /// "METRIC_RATIO"
    #[serde(rename="METRIC_RATIO")]
    METRICRATIO,
    

    /// Currency header type.
    ///
    /// "METRIC_CURRENCY"
    #[serde(rename="METRIC_CURRENCY")]
    METRICCURRENCY,
    

    /// Milliseconds header type.
    ///
    /// "METRIC_MILLISECONDS"
    #[serde(rename="METRIC_MILLISECONDS")]
    METRICMILLISECONDS,
    

    /// Decimal header type.
    ///
    /// "METRIC_DECIMAL"
    #[serde(rename="METRIC_DECIMAL")]
    METRICDECIMAL,
}

impl AsRef<str> for HeaderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HeaderTypeEnum::HEADERTYPEUNSPECIFIED => "HEADER_TYPE_UNSPECIFIED",
            HeaderTypeEnum::DIMENSION => "DIMENSION",
            HeaderTypeEnum::METRICTALLY => "METRIC_TALLY",
            HeaderTypeEnum::METRICRATIO => "METRIC_RATIO",
            HeaderTypeEnum::METRICCURRENCY => "METRIC_CURRENCY",
            HeaderTypeEnum::METRICMILLISECONDS => "METRIC_MILLISECONDS",
            HeaderTypeEnum::METRICDECIMAL => "METRIC_DECIMAL",
        }
    }
}

impl std::convert::TryFrom< &str> for HeaderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEADER_TYPE_UNSPECIFIED" => Ok(HeaderTypeEnum::HEADERTYPEUNSPECIFIED),
           "DIMENSION" => Ok(HeaderTypeEnum::DIMENSION),
           "METRIC_TALLY" => Ok(HeaderTypeEnum::METRICTALLY),
           "METRIC_RATIO" => Ok(HeaderTypeEnum::METRICRATIO),
           "METRIC_CURRENCY" => Ok(HeaderTypeEnum::METRICCURRENCY),
           "METRIC_MILLISECONDS" => Ok(HeaderTypeEnum::METRICMILLISECONDS),
           "METRIC_DECIMAL" => Ok(HeaderTypeEnum::METRICDECIMAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HeaderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyIssueActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The most severe action taken on the entity over the past seven days.
pub enum PolicyIssueActionEnum {
    

    /// The action is unspecified.
    ///
    /// "ENFORCEMENT_ACTION_UNSPECIFIED"
    #[serde(rename="ENFORCEMENT_ACTION_UNSPECIFIED")]
    ENFORCEMENTACTIONUNSPECIFIED,
    

    /// No ad serving enforcement is currently present, but enforcement will start on the `warning_escalation_date` if the issue is not resolved.
    ///
    /// "WARNED"
    #[serde(rename="WARNED")]
    WARNED,
    

    /// Ad serving demand has been restricted on the entity.
    ///
    /// "AD_SERVING_RESTRICTED"
    #[serde(rename="AD_SERVING_RESTRICTED")]
    ADSERVINGRESTRICTED,
    

    /// Ad serving has been disabled on the entity.
    ///
    /// "AD_SERVING_DISABLED"
    #[serde(rename="AD_SERVING_DISABLED")]
    ADSERVINGDISABLED,
    

    /// Ads are being served for the entity but Confirmed Click is being applied to the ads. See https://support.google.com/adsense/answer/10025624.
    ///
    /// "AD_SERVED_WITH_CLICK_CONFIRMATION"
    #[serde(rename="AD_SERVED_WITH_CLICK_CONFIRMATION")]
    ADSERVEDWITHCLICKCONFIRMATION,
    

    /// Ad personalization is restricted because the ad requests coming from the EEA and UK do not have a TCF string or the Consent Management Platform (CMP) indicated by the TCF string is not Google certified. As a result, basic/limited ads will be served. See https://support.google.com/adsense/answer/13554116
    ///
    /// "AD_PERSONALIZATION_RESTRICTED"
    #[serde(rename="AD_PERSONALIZATION_RESTRICTED")]
    ADPERSONALIZATIONRESTRICTED,
}

impl AsRef<str> for PolicyIssueActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyIssueActionEnum::ENFORCEMENTACTIONUNSPECIFIED => "ENFORCEMENT_ACTION_UNSPECIFIED",
            PolicyIssueActionEnum::WARNED => "WARNED",
            PolicyIssueActionEnum::ADSERVINGRESTRICTED => "AD_SERVING_RESTRICTED",
            PolicyIssueActionEnum::ADSERVINGDISABLED => "AD_SERVING_DISABLED",
            PolicyIssueActionEnum::ADSERVEDWITHCLICKCONFIRMATION => "AD_SERVED_WITH_CLICK_CONFIRMATION",
            PolicyIssueActionEnum::ADPERSONALIZATIONRESTRICTED => "AD_PERSONALIZATION_RESTRICTED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyIssueActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENFORCEMENT_ACTION_UNSPECIFIED" => Ok(PolicyIssueActionEnum::ENFORCEMENTACTIONUNSPECIFIED),
           "WARNED" => Ok(PolicyIssueActionEnum::WARNED),
           "AD_SERVING_RESTRICTED" => Ok(PolicyIssueActionEnum::ADSERVINGRESTRICTED),
           "AD_SERVING_DISABLED" => Ok(PolicyIssueActionEnum::ADSERVINGDISABLED),
           "AD_SERVED_WITH_CLICK_CONFIRMATION" => Ok(PolicyIssueActionEnum::ADSERVEDWITHCLICKCONFIRMATION),
           "AD_PERSONALIZATION_RESTRICTED" => Ok(PolicyIssueActionEnum::ADPERSONALIZATIONRESTRICTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyIssueActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyIssueEntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of the entity indicating if the entity is a site, site-section, or page.
pub enum PolicyIssueEntityTypeEnum {
    

    /// The entity type is unspecified.
    ///
    /// "ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="ENTITY_TYPE_UNSPECIFIED")]
    ENTITYTYPEUNSPECIFIED,
    

    /// The enforced entity is an entire website.
    ///
    /// "SITE"
    #[serde(rename="SITE")]
    SITE,
    

    /// The enforced entity is a particular section of a website. All the pages with this prefix are enforced.
    ///
    /// "SITE_SECTION"
    #[serde(rename="SITE_SECTION")]
    SITESECTION,
    

    /// The enforced entity is a single web page.
    ///
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
}

impl AsRef<str> for PolicyIssueEntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyIssueEntityTypeEnum::ENTITYTYPEUNSPECIFIED => "ENTITY_TYPE_UNSPECIFIED",
            PolicyIssueEntityTypeEnum::SITE => "SITE",
            PolicyIssueEntityTypeEnum::SITESECTION => "SITE_SECTION",
            PolicyIssueEntityTypeEnum::PAGE => "PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyIssueEntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_TYPE_UNSPECIFIED" => Ok(PolicyIssueEntityTypeEnum::ENTITYTYPEUNSPECIFIED),
           "SITE" => Ok(PolicyIssueEntityTypeEnum::SITE),
           "SITE_SECTION" => Ok(PolicyIssueEntityTypeEnum::SITESECTION),
           "PAGE" => Ok(PolicyIssueEntityTypeEnum::PAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyIssueEntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of a site.
pub enum SiteStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Either: * The site hasn't been checked yet. * The site is inactive and needs another review before it can show ads again. Learn how to [request a review for an inactive site](https://support.google.com/adsense/answer/9393996).
    ///
    /// "REQUIRES_REVIEW"
    #[serde(rename="REQUIRES_REVIEW")]
    REQUIRESREVIEW,
    

    /// Google is running some checks on the site. This usually takes a few days, but in some cases it can take two to four weeks.
    ///
    /// "GETTING_READY"
    #[serde(rename="GETTING_READY")]
    GETTINGREADY,
    

    /// The site is ready to show ads. Learn how to [set up ads on the site](https://support.google.com/adsense/answer/7037624).
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Publisher needs to fix some issues before the site is ready to show ads. Learn what to do [if a new site isn't ready](https://support.google.com/adsense/answer/9061852).
    ///
    /// "NEEDS_ATTENTION"
    #[serde(rename="NEEDS_ATTENTION")]
    NEEDSATTENTION,
}

impl AsRef<str> for SiteStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SiteStateEnum::REQUIRESREVIEW => "REQUIRES_REVIEW",
            SiteStateEnum::GETTINGREADY => "GETTING_READY",
            SiteStateEnum::READY => "READY",
            SiteStateEnum::NEEDSATTENTION => "NEEDS_ATTENTION",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SiteStateEnum::STATEUNSPECIFIED),
           "REQUIRES_REVIEW" => Ok(SiteStateEnum::REQUIRESREVIEW),
           "GETTING_READY" => Ok(SiteStateEnum::GETTINGREADY),
           "READY" => Ok(SiteStateEnum::READY),
           "NEEDS_ATTENTION" => Ok(SiteStateEnum::NEEDSATTENTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountDateRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Date range of the report, if unset the range will be considered CUSTOM.
pub enum AccountDateRangeEnum {
    

    /// Unspecified date range.
    ///
    /// "REPORTING_DATE_RANGE_UNSPECIFIED"
    #[serde(rename="REPORTING_DATE_RANGE_UNSPECIFIED")]
    REPORTINGDATERANGEUNSPECIFIED,
    

    /// A custom date range specified using the `start_date` and `end_date` fields. This is the default if no ReportingDateRange is provided.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
    

    /// Current day.
    ///
    /// "TODAY"
    #[serde(rename="TODAY")]
    TODAY,
    

    /// Yesterday.
    ///
    /// "YESTERDAY"
    #[serde(rename="YESTERDAY")]
    YESTERDAY,
    

    /// From the start of the current month to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-03-01, 2020-03-12].
    ///
    /// "MONTH_TO_DATE"
    #[serde(rename="MONTH_TO_DATE")]
    MONTHTODATE,
    

    /// From the start of the current year to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-01-01, 2020-03-12].
    ///
    /// "YEAR_TO_DATE"
    #[serde(rename="YEAR_TO_DATE")]
    YEARTODATE,
    

    /// Last 7 days, excluding current day.
    ///
    /// "LAST_7_DAYS"
    #[serde(rename="LAST_7_DAYS")]
    LAST7DAYS,
    

    /// Last 30 days, excluding current day.
    ///
    /// "LAST_30_DAYS"
    #[serde(rename="LAST_30_DAYS")]
    LAST30DAYS,
}

impl AsRef<str> for AccountDateRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountDateRangeEnum::REPORTINGDATERANGEUNSPECIFIED => "REPORTING_DATE_RANGE_UNSPECIFIED",
            AccountDateRangeEnum::CUSTOM => "CUSTOM",
            AccountDateRangeEnum::TODAY => "TODAY",
            AccountDateRangeEnum::YESTERDAY => "YESTERDAY",
            AccountDateRangeEnum::MONTHTODATE => "MONTH_TO_DATE",
            AccountDateRangeEnum::YEARTODATE => "YEAR_TO_DATE",
            AccountDateRangeEnum::LAST7DAYS => "LAST_7_DAYS",
            AccountDateRangeEnum::LAST30DAYS => "LAST_30_DAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountDateRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPORTING_DATE_RANGE_UNSPECIFIED" => Ok(AccountDateRangeEnum::REPORTINGDATERANGEUNSPECIFIED),
           "CUSTOM" => Ok(AccountDateRangeEnum::CUSTOM),
           "TODAY" => Ok(AccountDateRangeEnum::TODAY),
           "YESTERDAY" => Ok(AccountDateRangeEnum::YESTERDAY),
           "MONTH_TO_DATE" => Ok(AccountDateRangeEnum::MONTHTODATE),
           "YEAR_TO_DATE" => Ok(AccountDateRangeEnum::YEARTODATE),
           "LAST_7_DAYS" => Ok(AccountDateRangeEnum::LAST7DAYS),
           "LAST_30_DAYS" => Ok(AccountDateRangeEnum::LAST30DAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountDateRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountReportingTimeZoneEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Timezone in which to generate the report. If unspecified, this defaults to the account timezone. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725).
pub enum AccountReportingTimeZoneEnum {
    

    /// Unspecified timezone.
    ///
    /// "REPORTING_TIME_ZONE_UNSPECIFIED"
    #[serde(rename="REPORTING_TIME_ZONE_UNSPECIFIED")]
    REPORTINGTIMEZONEUNSPECIFIED,
    

    /// Use the account timezone in the report.
    ///
    /// "ACCOUNT_TIME_ZONE"
    #[serde(rename="ACCOUNT_TIME_ZONE")]
    ACCOUNTTIMEZONE,
    

    /// Use the Google timezone in the report (America/Los_Angeles).
    ///
    /// "GOOGLE_TIME_ZONE"
    #[serde(rename="GOOGLE_TIME_ZONE")]
    GOOGLETIMEZONE,
}

impl AsRef<str> for AccountReportingTimeZoneEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountReportingTimeZoneEnum::REPORTINGTIMEZONEUNSPECIFIED => "REPORTING_TIME_ZONE_UNSPECIFIED",
            AccountReportingTimeZoneEnum::ACCOUNTTIMEZONE => "ACCOUNT_TIME_ZONE",
            AccountReportingTimeZoneEnum::GOOGLETIMEZONE => "GOOGLE_TIME_ZONE",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountReportingTimeZoneEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPORTING_TIME_ZONE_UNSPECIFIED" => Ok(AccountReportingTimeZoneEnum::REPORTINGTIMEZONEUNSPECIFIED),
           "ACCOUNT_TIME_ZONE" => Ok(AccountReportingTimeZoneEnum::ACCOUNTTIMEZONE),
           "GOOGLE_TIME_ZONE" => Ok(AccountReportingTimeZoneEnum::GOOGLETIMEZONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountReportingTimeZoneEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountDimensionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dimensions to base the report on.
pub enum AccountDimensionsEnum {
    

    /// Unspecified dimension.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// Date dimension in YYYY-MM-DD format (e.g. "2010-02-10").
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Week dimension in YYYY-MM-DD format, representing the first day of each week (e.g. "2010-02-08"). The first day of the week is determined by the language_code specified in a report generation request (so e.g. this would be a Monday for "en-GB" or "es", but a Sunday for "en" or "fr-CA").
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// Month dimension in YYYY-MM format (e.g. "2010-02").
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Account name. The members of this dimension match the values from Account.display_name.
    ///
    /// "ACCOUNT_NAME"
    #[serde(rename="ACCOUNT_NAME")]
    ACCOUNTNAME,
    

    /// Unique ID of an ad client. The members of this dimension match the values from AdClient.reporting_dimension_id.
    ///
    /// "AD_CLIENT_ID"
    #[serde(rename="AD_CLIENT_ID")]
    ADCLIENTID,
    

    /// Unique ID of a sub-account's ad client. The members of this dimension match the values from AdClient.reporting_dimension_id (for the sub-account).
    ///
    /// "HOSTED_AD_CLIENT_ID"
    #[serde(rename="HOSTED_AD_CLIENT_ID")]
    HOSTEDADCLIENTID,
    

    /// Localized product name (e.g. "AdSense for Content", "AdSense for Search").
    ///
    /// "PRODUCT_NAME"
    #[serde(rename="PRODUCT_NAME")]
    PRODUCTNAME,
    

    /// Product code (e.g. "AFC", "AFS"). The members of this dimension match the values from AdClient.product_code.
    ///
    /// "PRODUCT_CODE"
    #[serde(rename="PRODUCT_CODE")]
    PRODUCTCODE,
    

    /// Ad unit name (within which an ad was served). The members of this dimension match the values from AdUnit.display_name.
    ///
    /// "AD_UNIT_NAME"
    #[serde(rename="AD_UNIT_NAME")]
    ADUNITNAME,
    

    /// Unique ID of an ad unit (within which an ad was served). The members of this dimension match the values from AdUnit.reporting_dimension_id.
    ///
    /// "AD_UNIT_ID"
    #[serde(rename="AD_UNIT_ID")]
    ADUNITID,
    

    /// Localized size of an ad unit (e.g. "728x90", "Responsive").
    ///
    /// "AD_UNIT_SIZE_NAME"
    #[serde(rename="AD_UNIT_SIZE_NAME")]
    ADUNITSIZENAME,
    

    /// The size code of an ad unit (e.g. "728x90", "responsive").
    ///
    /// "AD_UNIT_SIZE_CODE"
    #[serde(rename="AD_UNIT_SIZE_CODE")]
    ADUNITSIZECODE,
    

    /// Custom channel name. The members of this dimension match the values from CustomChannel.display_name.
    ///
    /// "CUSTOM_CHANNEL_NAME"
    #[serde(rename="CUSTOM_CHANNEL_NAME")]
    CUSTOMCHANNELNAME,
    

    /// Unique ID of a custom channel. The members of this dimension match the values from CustomChannel.reporting_dimension_id.
    ///
    /// "CUSTOM_CHANNEL_ID"
    #[serde(rename="CUSTOM_CHANNEL_ID")]
    CUSTOMCHANNELID,
    

    /// Domain name of a verified site (e.g. "example.com"). The members of this dimension match the values from Site.domain.
    ///
    /// "OWNED_SITE_DOMAIN_NAME"
    #[serde(rename="OWNED_SITE_DOMAIN_NAME")]
    OWNEDSITEDOMAINNAME,
    

    /// Unique ID of a verified site. The members of this dimension match the values from Site.reporting_dimension_id.
    ///
    /// "OWNED_SITE_ID"
    #[serde(rename="OWNED_SITE_ID")]
    OWNEDSITEID,
    

    /// URL of the page upon which the ad was served. This is a complete URL including scheme and query parameters. Note that the URL that appears in this dimension may be a canonicalized version of the one that was used in the original request, and so may not exactly match the URL that a user might have seen. Note that there are also some caveats to be aware of when using this dimension. For more information, see [Page URL breakdown](https://support.google.com/adsense/answer/11988478).
    ///
    /// "PAGE_URL"
    #[serde(rename="PAGE_URL")]
    PAGEURL,
    

    /// Name of a URL channel. The members of this dimension match the values from UrlChannel.uri_pattern.
    ///
    /// "URL_CHANNEL_NAME"
    #[serde(rename="URL_CHANNEL_NAME")]
    URLCHANNELNAME,
    

    /// Unique ID of a URL channel. The members of this dimension match the values from UrlChannel.reporting_dimension_id.
    ///
    /// "URL_CHANNEL_ID"
    #[serde(rename="URL_CHANNEL_ID")]
    URLCHANNELID,
    

    /// Name of an ad network that returned the winning ads for an ad request (e.g. "Google AdWords"). Note that unlike other "NAME" dimensions, the members of this dimensions are not localized.
    ///
    /// "BUYER_NETWORK_NAME"
    #[serde(rename="BUYER_NETWORK_NAME")]
    BUYERNETWORKNAME,
    

    /// Unique (opaque) ID of an ad network that returned the winning ads for an ad request.
    ///
    /// "BUYER_NETWORK_ID"
    #[serde(rename="BUYER_NETWORK_ID")]
    BUYERNETWORKID,
    

    /// Localized bid type name (e.g. "CPC bids", "CPM bids") for a served ad.
    ///
    /// "BID_TYPE_NAME"
    #[serde(rename="BID_TYPE_NAME")]
    BIDTYPENAME,
    

    /// Type of a bid (e.g. "cpc", "cpm") for a served ad.
    ///
    /// "BID_TYPE_CODE"
    #[serde(rename="BID_TYPE_CODE")]
    BIDTYPECODE,
    

    /// Localized creative size name (e.g. "728x90", "Dynamic") of a served ad.
    ///
    /// "CREATIVE_SIZE_NAME"
    #[serde(rename="CREATIVE_SIZE_NAME")]
    CREATIVESIZENAME,
    

    /// Creative size code (e.g. "728x90", "dynamic") of a served ad.
    ///
    /// "CREATIVE_SIZE_CODE"
    #[serde(rename="CREATIVE_SIZE_CODE")]
    CREATIVESIZECODE,
    

    /// Localized name of a host on which an ad was served, after IDNA decoding (e.g. "www.google.com", "Web caches and other", "bücher.example").
    ///
    /// "DOMAIN_NAME"
    #[serde(rename="DOMAIN_NAME")]
    DOMAINNAME,
    

    /// Name of a host on which an ad was served (e.g. "www.google.com", "webcaches", "xn--bcher-kva.example").
    ///
    /// "DOMAIN_CODE"
    #[serde(rename="DOMAIN_CODE")]
    DOMAINCODE,
    

    /// Localized region name of a user viewing an ad (e.g. "United States", "France").
    ///
    /// "COUNTRY_NAME"
    #[serde(rename="COUNTRY_NAME")]
    COUNTRYNAME,
    

    /// CLDR region code of a user viewing an ad (e.g. "US", "FR").
    ///
    /// "COUNTRY_CODE"
    #[serde(rename="COUNTRY_CODE")]
    COUNTRYCODE,
    

    /// Localized platform type name (e.g. "High-end mobile devices", "Desktop").
    ///
    /// "PLATFORM_TYPE_NAME"
    #[serde(rename="PLATFORM_TYPE_NAME")]
    PLATFORMTYPENAME,
    

    /// Platform type code (e.g. "HighEndMobile", "Desktop").
    ///
    /// "PLATFORM_TYPE_CODE"
    #[serde(rename="PLATFORM_TYPE_CODE")]
    PLATFORMTYPECODE,
    

    /// Localized targeting type name (e.g. "Contextual", "Personalized", "Run of Network").
    ///
    /// "TARGETING_TYPE_NAME"
    #[serde(rename="TARGETING_TYPE_NAME")]
    TARGETINGTYPENAME,
    

    /// Targeting type code (e.g. "Keyword", "UserInterest", "RunOfNetwork").
    ///
    /// "TARGETING_TYPE_CODE"
    #[serde(rename="TARGETING_TYPE_CODE")]
    TARGETINGTYPECODE,
    

    /// Localized content platform name an ad request was made from (e.g. "AMP", "Web").
    ///
    /// "CONTENT_PLATFORM_NAME"
    #[serde(rename="CONTENT_PLATFORM_NAME")]
    CONTENTPLATFORMNAME,
    

    /// Content platform code an ad request was made from (e.g. "AMP", "HTML").
    ///
    /// "CONTENT_PLATFORM_CODE"
    #[serde(rename="CONTENT_PLATFORM_CODE")]
    CONTENTPLATFORMCODE,
    

    /// Localized ad placement name (e.g. "Ad unit", "Global settings", "Manual").
    ///
    /// "AD_PLACEMENT_NAME"
    #[serde(rename="AD_PLACEMENT_NAME")]
    ADPLACEMENTNAME,
    

    /// Ad placement code (e.g. "AD_UNIT", "ca-pub-123456:78910", "OTHER").
    ///
    /// "AD_PLACEMENT_CODE"
    #[serde(rename="AD_PLACEMENT_CODE")]
    ADPLACEMENTCODE,
    

    /// Localized requested ad type name (e.g. "Display", "Link unit", "Other").
    ///
    /// "REQUESTED_AD_TYPE_NAME"
    #[serde(rename="REQUESTED_AD_TYPE_NAME")]
    REQUESTEDADTYPENAME,
    

    /// Requested ad type code (e.g. "IMAGE", "RADLINK", "OTHER").
    ///
    /// "REQUESTED_AD_TYPE_CODE"
    #[serde(rename="REQUESTED_AD_TYPE_CODE")]
    REQUESTEDADTYPECODE,
    

    /// Localized served ad type name (e.g. "Display", "Link unit", "Other").
    ///
    /// "SERVED_AD_TYPE_NAME"
    #[serde(rename="SERVED_AD_TYPE_NAME")]
    SERVEDADTYPENAME,
    

    /// Served ad type code (e.g. "IMAGE", "RADLINK", "OTHER").
    ///
    /// "SERVED_AD_TYPE_CODE"
    #[serde(rename="SERVED_AD_TYPE_CODE")]
    SERVEDADTYPECODE,
    

    /// Localized ad format name indicating the way an ad is shown to the users on your site (e.g. "In-page", "Anchor", "Vignette").
    ///
    /// "AD_FORMAT_NAME"
    #[serde(rename="AD_FORMAT_NAME")]
    ADFORMATNAME,
    

    /// Ad format code indicating the way an ad is shown to the users on your site (e.g. "ON_PAGE", "ANCHOR", "INTERSTITIAL").
    ///
    /// "AD_FORMAT_CODE"
    #[serde(rename="AD_FORMAT_CODE")]
    ADFORMATCODE,
    

    /// Custom search style name.
    ///
    /// "CUSTOM_SEARCH_STYLE_NAME"
    #[serde(rename="CUSTOM_SEARCH_STYLE_NAME")]
    CUSTOMSEARCHSTYLENAME,
    

    /// Custom search style id.
    ///
    /// "CUSTOM_SEARCH_STYLE_ID"
    #[serde(rename="CUSTOM_SEARCH_STYLE_ID")]
    CUSTOMSEARCHSTYLEID,
    

    /// Domain registrants.
    ///
    /// "DOMAIN_REGISTRANT"
    #[serde(rename="DOMAIN_REGISTRANT")]
    DOMAINREGISTRANT,
    

    /// Query strings for web searches.
    ///
    /// "WEBSEARCH_QUERY_STRING"
    #[serde(rename="WEBSEARCH_QUERY_STRING")]
    WEBSEARCHQUERYSTRING,
}

impl AsRef<str> for AccountDimensionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountDimensionsEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            AccountDimensionsEnum::DATE => "DATE",
            AccountDimensionsEnum::WEEK => "WEEK",
            AccountDimensionsEnum::MONTH => "MONTH",
            AccountDimensionsEnum::ACCOUNTNAME => "ACCOUNT_NAME",
            AccountDimensionsEnum::ADCLIENTID => "AD_CLIENT_ID",
            AccountDimensionsEnum::HOSTEDADCLIENTID => "HOSTED_AD_CLIENT_ID",
            AccountDimensionsEnum::PRODUCTNAME => "PRODUCT_NAME",
            AccountDimensionsEnum::PRODUCTCODE => "PRODUCT_CODE",
            AccountDimensionsEnum::ADUNITNAME => "AD_UNIT_NAME",
            AccountDimensionsEnum::ADUNITID => "AD_UNIT_ID",
            AccountDimensionsEnum::ADUNITSIZENAME => "AD_UNIT_SIZE_NAME",
            AccountDimensionsEnum::ADUNITSIZECODE => "AD_UNIT_SIZE_CODE",
            AccountDimensionsEnum::CUSTOMCHANNELNAME => "CUSTOM_CHANNEL_NAME",
            AccountDimensionsEnum::CUSTOMCHANNELID => "CUSTOM_CHANNEL_ID",
            AccountDimensionsEnum::OWNEDSITEDOMAINNAME => "OWNED_SITE_DOMAIN_NAME",
            AccountDimensionsEnum::OWNEDSITEID => "OWNED_SITE_ID",
            AccountDimensionsEnum::PAGEURL => "PAGE_URL",
            AccountDimensionsEnum::URLCHANNELNAME => "URL_CHANNEL_NAME",
            AccountDimensionsEnum::URLCHANNELID => "URL_CHANNEL_ID",
            AccountDimensionsEnum::BUYERNETWORKNAME => "BUYER_NETWORK_NAME",
            AccountDimensionsEnum::BUYERNETWORKID => "BUYER_NETWORK_ID",
            AccountDimensionsEnum::BIDTYPENAME => "BID_TYPE_NAME",
            AccountDimensionsEnum::BIDTYPECODE => "BID_TYPE_CODE",
            AccountDimensionsEnum::CREATIVESIZENAME => "CREATIVE_SIZE_NAME",
            AccountDimensionsEnum::CREATIVESIZECODE => "CREATIVE_SIZE_CODE",
            AccountDimensionsEnum::DOMAINNAME => "DOMAIN_NAME",
            AccountDimensionsEnum::DOMAINCODE => "DOMAIN_CODE",
            AccountDimensionsEnum::COUNTRYNAME => "COUNTRY_NAME",
            AccountDimensionsEnum::COUNTRYCODE => "COUNTRY_CODE",
            AccountDimensionsEnum::PLATFORMTYPENAME => "PLATFORM_TYPE_NAME",
            AccountDimensionsEnum::PLATFORMTYPECODE => "PLATFORM_TYPE_CODE",
            AccountDimensionsEnum::TARGETINGTYPENAME => "TARGETING_TYPE_NAME",
            AccountDimensionsEnum::TARGETINGTYPECODE => "TARGETING_TYPE_CODE",
            AccountDimensionsEnum::CONTENTPLATFORMNAME => "CONTENT_PLATFORM_NAME",
            AccountDimensionsEnum::CONTENTPLATFORMCODE => "CONTENT_PLATFORM_CODE",
            AccountDimensionsEnum::ADPLACEMENTNAME => "AD_PLACEMENT_NAME",
            AccountDimensionsEnum::ADPLACEMENTCODE => "AD_PLACEMENT_CODE",
            AccountDimensionsEnum::REQUESTEDADTYPENAME => "REQUESTED_AD_TYPE_NAME",
            AccountDimensionsEnum::REQUESTEDADTYPECODE => "REQUESTED_AD_TYPE_CODE",
            AccountDimensionsEnum::SERVEDADTYPENAME => "SERVED_AD_TYPE_NAME",
            AccountDimensionsEnum::SERVEDADTYPECODE => "SERVED_AD_TYPE_CODE",
            AccountDimensionsEnum::ADFORMATNAME => "AD_FORMAT_NAME",
            AccountDimensionsEnum::ADFORMATCODE => "AD_FORMAT_CODE",
            AccountDimensionsEnum::CUSTOMSEARCHSTYLENAME => "CUSTOM_SEARCH_STYLE_NAME",
            AccountDimensionsEnum::CUSTOMSEARCHSTYLEID => "CUSTOM_SEARCH_STYLE_ID",
            AccountDimensionsEnum::DOMAINREGISTRANT => "DOMAIN_REGISTRANT",
            AccountDimensionsEnum::WEBSEARCHQUERYSTRING => "WEBSEARCH_QUERY_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountDimensionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(AccountDimensionsEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(AccountDimensionsEnum::DATE),
           "WEEK" => Ok(AccountDimensionsEnum::WEEK),
           "MONTH" => Ok(AccountDimensionsEnum::MONTH),
           "ACCOUNT_NAME" => Ok(AccountDimensionsEnum::ACCOUNTNAME),
           "AD_CLIENT_ID" => Ok(AccountDimensionsEnum::ADCLIENTID),
           "HOSTED_AD_CLIENT_ID" => Ok(AccountDimensionsEnum::HOSTEDADCLIENTID),
           "PRODUCT_NAME" => Ok(AccountDimensionsEnum::PRODUCTNAME),
           "PRODUCT_CODE" => Ok(AccountDimensionsEnum::PRODUCTCODE),
           "AD_UNIT_NAME" => Ok(AccountDimensionsEnum::ADUNITNAME),
           "AD_UNIT_ID" => Ok(AccountDimensionsEnum::ADUNITID),
           "AD_UNIT_SIZE_NAME" => Ok(AccountDimensionsEnum::ADUNITSIZENAME),
           "AD_UNIT_SIZE_CODE" => Ok(AccountDimensionsEnum::ADUNITSIZECODE),
           "CUSTOM_CHANNEL_NAME" => Ok(AccountDimensionsEnum::CUSTOMCHANNELNAME),
           "CUSTOM_CHANNEL_ID" => Ok(AccountDimensionsEnum::CUSTOMCHANNELID),
           "OWNED_SITE_DOMAIN_NAME" => Ok(AccountDimensionsEnum::OWNEDSITEDOMAINNAME),
           "OWNED_SITE_ID" => Ok(AccountDimensionsEnum::OWNEDSITEID),
           "PAGE_URL" => Ok(AccountDimensionsEnum::PAGEURL),
           "URL_CHANNEL_NAME" => Ok(AccountDimensionsEnum::URLCHANNELNAME),
           "URL_CHANNEL_ID" => Ok(AccountDimensionsEnum::URLCHANNELID),
           "BUYER_NETWORK_NAME" => Ok(AccountDimensionsEnum::BUYERNETWORKNAME),
           "BUYER_NETWORK_ID" => Ok(AccountDimensionsEnum::BUYERNETWORKID),
           "BID_TYPE_NAME" => Ok(AccountDimensionsEnum::BIDTYPENAME),
           "BID_TYPE_CODE" => Ok(AccountDimensionsEnum::BIDTYPECODE),
           "CREATIVE_SIZE_NAME" => Ok(AccountDimensionsEnum::CREATIVESIZENAME),
           "CREATIVE_SIZE_CODE" => Ok(AccountDimensionsEnum::CREATIVESIZECODE),
           "DOMAIN_NAME" => Ok(AccountDimensionsEnum::DOMAINNAME),
           "DOMAIN_CODE" => Ok(AccountDimensionsEnum::DOMAINCODE),
           "COUNTRY_NAME" => Ok(AccountDimensionsEnum::COUNTRYNAME),
           "COUNTRY_CODE" => Ok(AccountDimensionsEnum::COUNTRYCODE),
           "PLATFORM_TYPE_NAME" => Ok(AccountDimensionsEnum::PLATFORMTYPENAME),
           "PLATFORM_TYPE_CODE" => Ok(AccountDimensionsEnum::PLATFORMTYPECODE),
           "TARGETING_TYPE_NAME" => Ok(AccountDimensionsEnum::TARGETINGTYPENAME),
           "TARGETING_TYPE_CODE" => Ok(AccountDimensionsEnum::TARGETINGTYPECODE),
           "CONTENT_PLATFORM_NAME" => Ok(AccountDimensionsEnum::CONTENTPLATFORMNAME),
           "CONTENT_PLATFORM_CODE" => Ok(AccountDimensionsEnum::CONTENTPLATFORMCODE),
           "AD_PLACEMENT_NAME" => Ok(AccountDimensionsEnum::ADPLACEMENTNAME),
           "AD_PLACEMENT_CODE" => Ok(AccountDimensionsEnum::ADPLACEMENTCODE),
           "REQUESTED_AD_TYPE_NAME" => Ok(AccountDimensionsEnum::REQUESTEDADTYPENAME),
           "REQUESTED_AD_TYPE_CODE" => Ok(AccountDimensionsEnum::REQUESTEDADTYPECODE),
           "SERVED_AD_TYPE_NAME" => Ok(AccountDimensionsEnum::SERVEDADTYPENAME),
           "SERVED_AD_TYPE_CODE" => Ok(AccountDimensionsEnum::SERVEDADTYPECODE),
           "AD_FORMAT_NAME" => Ok(AccountDimensionsEnum::ADFORMATNAME),
           "AD_FORMAT_CODE" => Ok(AccountDimensionsEnum::ADFORMATCODE),
           "CUSTOM_SEARCH_STYLE_NAME" => Ok(AccountDimensionsEnum::CUSTOMSEARCHSTYLENAME),
           "CUSTOM_SEARCH_STYLE_ID" => Ok(AccountDimensionsEnum::CUSTOMSEARCHSTYLEID),
           "DOMAIN_REGISTRANT" => Ok(AccountDimensionsEnum::DOMAINREGISTRANT),
           "WEBSEARCH_QUERY_STRING" => Ok(AccountDimensionsEnum::WEBSEARCHQUERYSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountDimensionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountMetricsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Reporting metrics.
pub enum AccountMetricsEnum {
    

    /// Unspecified metric.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// Number of page views.
    ///
    /// "PAGE_VIEWS"
    #[serde(rename="PAGE_VIEWS")]
    PAGEVIEWS,
    

    /// Number of ad units that requested ads (for content ads) or search queries (for search ads). An ad request may result in zero, one, or multiple individual ad impressions depending on the size of the ad unit and whether any ads were available.
    ///
    /// "AD_REQUESTS"
    #[serde(rename="AD_REQUESTS")]
    ADREQUESTS,
    

    /// Requests that returned at least one ad.
    ///
    /// "MATCHED_AD_REQUESTS"
    #[serde(rename="MATCHED_AD_REQUESTS")]
    MATCHEDADREQUESTS,
    

    /// Impressions. An impression is counted for each ad request where at least one ad has been downloaded to the user’s device and has begun to load. It is the number of ad units (for content ads) or search queries (for search ads) that showed ads.
    ///
    /// "TOTAL_IMPRESSIONS"
    #[serde(rename="TOTAL_IMPRESSIONS")]
    TOTALIMPRESSIONS,
    

    /// Impressions. An impression is counted for each ad request where at least one ad has been downloaded to the user’s device and has begun to load. It is the number of ad units (for content ads) or search queries (for search ads) that showed ads.
    ///
    /// "IMPRESSIONS"
    #[serde(rename="IMPRESSIONS")]
    IMPRESSIONS,
    

    /// Ads shown. Different ad formats will display varying numbers of ads. For example, a vertical banner may consist of 2 or more ads. Also, the number of ads in an ad unit may vary depending on whether the ad unit is displaying standard text ads, expanded text ads or image ads.
    ///
    /// "INDIVIDUAL_AD_IMPRESSIONS"
    #[serde(rename="INDIVIDUAL_AD_IMPRESSIONS")]
    INDIVIDUALADIMPRESSIONS,
    

    /// Number of times a user clicked on a standard content ad.
    ///
    /// "CLICKS"
    #[serde(rename="CLICKS")]
    CLICKS,
    

    /// Fraction of page views considered to be spam. Only available to premium accounts.
    ///
    /// "PAGE_VIEWS_SPAM_RATIO"
    #[serde(rename="PAGE_VIEWS_SPAM_RATIO")]
    PAGEVIEWSSPAMRATIO,
    

    /// Fraction of ad requests considered to be spam. Only available to premium accounts.
    ///
    /// "AD_REQUESTS_SPAM_RATIO"
    #[serde(rename="AD_REQUESTS_SPAM_RATIO")]
    ADREQUESTSSPAMRATIO,
    

    /// Fraction of ad requests that returned ads considered to be spam. Only available to premium accounts.
    ///
    /// "MATCHED_AD_REQUESTS_SPAM_RATIO"
    #[serde(rename="MATCHED_AD_REQUESTS_SPAM_RATIO")]
    MATCHEDADREQUESTSSPAMRATIO,
    

    /// Fraction of impressions considered to be spam. Only available to premium accounts.
    ///
    /// "IMPRESSIONS_SPAM_RATIO"
    #[serde(rename="IMPRESSIONS_SPAM_RATIO")]
    IMPRESSIONSSPAMRATIO,
    

    /// Fraction of ad impressions considered to be spam. Only available to premium accounts.
    ///
    /// "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO"
    #[serde(rename="INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO")]
    INDIVIDUALADIMPRESSIONSSPAMRATIO,
    

    /// Fraction of clicks considered to be spam. Only available to premium accounts.
    ///
    /// "CLICKS_SPAM_RATIO"
    #[serde(rename="CLICKS_SPAM_RATIO")]
    CLICKSSPAMRATIO,
    

    /// Ratio of requested ad units or queries to the number returned to the site.
    ///
    /// "AD_REQUESTS_COVERAGE"
    #[serde(rename="AD_REQUESTS_COVERAGE")]
    ADREQUESTSCOVERAGE,
    

    /// Ratio of individual page views that resulted in a click.
    ///
    /// "PAGE_VIEWS_CTR"
    #[serde(rename="PAGE_VIEWS_CTR")]
    PAGEVIEWSCTR,
    

    /// Ratio of ad requests that resulted in a click.
    ///
    /// "AD_REQUESTS_CTR"
    #[serde(rename="AD_REQUESTS_CTR")]
    ADREQUESTSCTR,
    

    /// Ratio of clicks to matched requests.
    ///
    /// "MATCHED_AD_REQUESTS_CTR"
    #[serde(rename="MATCHED_AD_REQUESTS_CTR")]
    MATCHEDADREQUESTSCTR,
    

    /// Ratio of IMPRESSIONS that resulted in a click.
    ///
    /// "IMPRESSIONS_CTR"
    #[serde(rename="IMPRESSIONS_CTR")]
    IMPRESSIONSCTR,
    

    /// Ratio of individual ad impressions that resulted in a click.
    ///
    /// "INDIVIDUAL_AD_IMPRESSIONS_CTR"
    #[serde(rename="INDIVIDUAL_AD_IMPRESSIONS_CTR")]
    INDIVIDUALADIMPRESSIONSCTR,
    

    /// Ratio of requests that were measurable for viewability.
    ///
    /// "ACTIVE_VIEW_MEASURABILITY"
    #[serde(rename="ACTIVE_VIEW_MEASURABILITY")]
    ACTIVEVIEWMEASURABILITY,
    

    /// Ratio of requests that were viewable.
    ///
    /// "ACTIVE_VIEW_VIEWABILITY"
    #[serde(rename="ACTIVE_VIEW_VIEWABILITY")]
    ACTIVEVIEWVIEWABILITY,
    

    /// Mean time an ad was displayed on screen.
    ///
    /// "ACTIVE_VIEW_TIME"
    #[serde(rename="ACTIVE_VIEW_TIME")]
    ACTIVEVIEWTIME,
    

    /// Estimated earnings of the publisher. Note that earnings up to yesterday are accurate, more recent earnings are estimated due to the possibility of spam, or exchange rate fluctuations.
    ///
    /// "ESTIMATED_EARNINGS"
    #[serde(rename="ESTIMATED_EARNINGS")]
    ESTIMATEDEARNINGS,
    

    /// Revenue per thousand page views. This is calculated by dividing the estimated revenue by the number of page views multiplied by 1000.
    ///
    /// "PAGE_VIEWS_RPM"
    #[serde(rename="PAGE_VIEWS_RPM")]
    PAGEVIEWSRPM,
    

    /// Revenue per thousand ad requests. This is calculated by dividing estimated revenue by the number of ad requests multiplied by 1000.
    ///
    /// "AD_REQUESTS_RPM"
    #[serde(rename="AD_REQUESTS_RPM")]
    ADREQUESTSRPM,
    

    /// Revenue per thousand matched ad requests. This is calculated by dividing estimated revenue by the number of matched ad requests multiplied by 1000.
    ///
    /// "MATCHED_AD_REQUESTS_RPM"
    #[serde(rename="MATCHED_AD_REQUESTS_RPM")]
    MATCHEDADREQUESTSRPM,
    

    /// Revenue per thousand ad impressions. This is calculated by dividing estimated revenue by the number of ad impressions multiplied by 1000.
    ///
    /// "IMPRESSIONS_RPM"
    #[serde(rename="IMPRESSIONS_RPM")]
    IMPRESSIONSRPM,
    

    /// Revenue per thousand individual ad impressions. This is calculated by dividing estimated revenue by the number of individual ad impressions multiplied by 1000.
    ///
    /// "INDIVIDUAL_AD_IMPRESSIONS_RPM"
    #[serde(rename="INDIVIDUAL_AD_IMPRESSIONS_RPM")]
    INDIVIDUALADIMPRESSIONSRPM,
    

    /// Amount the publisher earns each time a user clicks on an ad. CPC is calculated by dividing the estimated revenue by the number of clicks received.
    ///
    /// "COST_PER_CLICK"
    #[serde(rename="COST_PER_CLICK")]
    COSTPERCLICK,
    

    /// Number of ad views per impression.
    ///
    /// "ADS_PER_IMPRESSION"
    #[serde(rename="ADS_PER_IMPRESSION")]
    ADSPERIMPRESSION,
    

    /// Total earnings are the gross estimated earnings from revenue shared traffic before any parent and child account revenue share is applied.
    ///
    /// "TOTAL_EARNINGS"
    #[serde(rename="TOTAL_EARNINGS")]
    TOTALEARNINGS,
    

    /// Number of results pages.
    ///
    /// "WEBSEARCH_RESULT_PAGES"
    #[serde(rename="WEBSEARCH_RESULT_PAGES")]
    WEBSEARCHRESULTPAGES,
    

    /// Number of requests for non-ad units (for example a related search unit). For more information, see [Funnel requests](https://support.google.com/adsense/answer/11586959).
    ///
    /// "FUNNEL_REQUESTS"
    #[serde(rename="FUNNEL_REQUESTS")]
    FUNNELREQUESTS,
    

    /// Number of requests for non-ad units ads that returned content that was shown to the user. For more information, see [Funnel impressions](https://support.google.com/adsense/answer/11585767).
    ///
    /// "FUNNEL_IMPRESSIONS"
    #[serde(rename="FUNNEL_IMPRESSIONS")]
    FUNNELIMPRESSIONS,
    

    /// Number of times a user clicked on a non-ad unit, triggering further ad requests. For more information, see [Funnel clicks](https://support.google.com/adsense/answer/11586382).
    ///
    /// "FUNNEL_CLICKS"
    #[serde(rename="FUNNEL_CLICKS")]
    FUNNELCLICKS,
    

    /// Revenue per thousand funnel impressions. This is calculated by dividing estimated revenue by the number of funnel impressions multiplied by 1000. For more information, see [Funnel RPM](https://support.google.com/adsense/answer/11585979).
    ///
    /// "FUNNEL_RPM"
    #[serde(rename="FUNNEL_RPM")]
    FUNNELRPM,
}

impl AsRef<str> for AccountMetricsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountMetricsEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            AccountMetricsEnum::PAGEVIEWS => "PAGE_VIEWS",
            AccountMetricsEnum::ADREQUESTS => "AD_REQUESTS",
            AccountMetricsEnum::MATCHEDADREQUESTS => "MATCHED_AD_REQUESTS",
            AccountMetricsEnum::TOTALIMPRESSIONS => "TOTAL_IMPRESSIONS",
            AccountMetricsEnum::IMPRESSIONS => "IMPRESSIONS",
            AccountMetricsEnum::INDIVIDUALADIMPRESSIONS => "INDIVIDUAL_AD_IMPRESSIONS",
            AccountMetricsEnum::CLICKS => "CLICKS",
            AccountMetricsEnum::PAGEVIEWSSPAMRATIO => "PAGE_VIEWS_SPAM_RATIO",
            AccountMetricsEnum::ADREQUESTSSPAMRATIO => "AD_REQUESTS_SPAM_RATIO",
            AccountMetricsEnum::MATCHEDADREQUESTSSPAMRATIO => "MATCHED_AD_REQUESTS_SPAM_RATIO",
            AccountMetricsEnum::IMPRESSIONSSPAMRATIO => "IMPRESSIONS_SPAM_RATIO",
            AccountMetricsEnum::INDIVIDUALADIMPRESSIONSSPAMRATIO => "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO",
            AccountMetricsEnum::CLICKSSPAMRATIO => "CLICKS_SPAM_RATIO",
            AccountMetricsEnum::ADREQUESTSCOVERAGE => "AD_REQUESTS_COVERAGE",
            AccountMetricsEnum::PAGEVIEWSCTR => "PAGE_VIEWS_CTR",
            AccountMetricsEnum::ADREQUESTSCTR => "AD_REQUESTS_CTR",
            AccountMetricsEnum::MATCHEDADREQUESTSCTR => "MATCHED_AD_REQUESTS_CTR",
            AccountMetricsEnum::IMPRESSIONSCTR => "IMPRESSIONS_CTR",
            AccountMetricsEnum::INDIVIDUALADIMPRESSIONSCTR => "INDIVIDUAL_AD_IMPRESSIONS_CTR",
            AccountMetricsEnum::ACTIVEVIEWMEASURABILITY => "ACTIVE_VIEW_MEASURABILITY",
            AccountMetricsEnum::ACTIVEVIEWVIEWABILITY => "ACTIVE_VIEW_VIEWABILITY",
            AccountMetricsEnum::ACTIVEVIEWTIME => "ACTIVE_VIEW_TIME",
            AccountMetricsEnum::ESTIMATEDEARNINGS => "ESTIMATED_EARNINGS",
            AccountMetricsEnum::PAGEVIEWSRPM => "PAGE_VIEWS_RPM",
            AccountMetricsEnum::ADREQUESTSRPM => "AD_REQUESTS_RPM",
            AccountMetricsEnum::MATCHEDADREQUESTSRPM => "MATCHED_AD_REQUESTS_RPM",
            AccountMetricsEnum::IMPRESSIONSRPM => "IMPRESSIONS_RPM",
            AccountMetricsEnum::INDIVIDUALADIMPRESSIONSRPM => "INDIVIDUAL_AD_IMPRESSIONS_RPM",
            AccountMetricsEnum::COSTPERCLICK => "COST_PER_CLICK",
            AccountMetricsEnum::ADSPERIMPRESSION => "ADS_PER_IMPRESSION",
            AccountMetricsEnum::TOTALEARNINGS => "TOTAL_EARNINGS",
            AccountMetricsEnum::WEBSEARCHRESULTPAGES => "WEBSEARCH_RESULT_PAGES",
            AccountMetricsEnum::FUNNELREQUESTS => "FUNNEL_REQUESTS",
            AccountMetricsEnum::FUNNELIMPRESSIONS => "FUNNEL_IMPRESSIONS",
            AccountMetricsEnum::FUNNELCLICKS => "FUNNEL_CLICKS",
            AccountMetricsEnum::FUNNELRPM => "FUNNEL_RPM",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountMetricsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(AccountMetricsEnum::METRICUNSPECIFIED),
           "PAGE_VIEWS" => Ok(AccountMetricsEnum::PAGEVIEWS),
           "AD_REQUESTS" => Ok(AccountMetricsEnum::ADREQUESTS),
           "MATCHED_AD_REQUESTS" => Ok(AccountMetricsEnum::MATCHEDADREQUESTS),
           "TOTAL_IMPRESSIONS" => Ok(AccountMetricsEnum::TOTALIMPRESSIONS),
           "IMPRESSIONS" => Ok(AccountMetricsEnum::IMPRESSIONS),
           "INDIVIDUAL_AD_IMPRESSIONS" => Ok(AccountMetricsEnum::INDIVIDUALADIMPRESSIONS),
           "CLICKS" => Ok(AccountMetricsEnum::CLICKS),
           "PAGE_VIEWS_SPAM_RATIO" => Ok(AccountMetricsEnum::PAGEVIEWSSPAMRATIO),
           "AD_REQUESTS_SPAM_RATIO" => Ok(AccountMetricsEnum::ADREQUESTSSPAMRATIO),
           "MATCHED_AD_REQUESTS_SPAM_RATIO" => Ok(AccountMetricsEnum::MATCHEDADREQUESTSSPAMRATIO),
           "IMPRESSIONS_SPAM_RATIO" => Ok(AccountMetricsEnum::IMPRESSIONSSPAMRATIO),
           "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO" => Ok(AccountMetricsEnum::INDIVIDUALADIMPRESSIONSSPAMRATIO),
           "CLICKS_SPAM_RATIO" => Ok(AccountMetricsEnum::CLICKSSPAMRATIO),
           "AD_REQUESTS_COVERAGE" => Ok(AccountMetricsEnum::ADREQUESTSCOVERAGE),
           "PAGE_VIEWS_CTR" => Ok(AccountMetricsEnum::PAGEVIEWSCTR),
           "AD_REQUESTS_CTR" => Ok(AccountMetricsEnum::ADREQUESTSCTR),
           "MATCHED_AD_REQUESTS_CTR" => Ok(AccountMetricsEnum::MATCHEDADREQUESTSCTR),
           "IMPRESSIONS_CTR" => Ok(AccountMetricsEnum::IMPRESSIONSCTR),
           "INDIVIDUAL_AD_IMPRESSIONS_CTR" => Ok(AccountMetricsEnum::INDIVIDUALADIMPRESSIONSCTR),
           "ACTIVE_VIEW_MEASURABILITY" => Ok(AccountMetricsEnum::ACTIVEVIEWMEASURABILITY),
           "ACTIVE_VIEW_VIEWABILITY" => Ok(AccountMetricsEnum::ACTIVEVIEWVIEWABILITY),
           "ACTIVE_VIEW_TIME" => Ok(AccountMetricsEnum::ACTIVEVIEWTIME),
           "ESTIMATED_EARNINGS" => Ok(AccountMetricsEnum::ESTIMATEDEARNINGS),
           "PAGE_VIEWS_RPM" => Ok(AccountMetricsEnum::PAGEVIEWSRPM),
           "AD_REQUESTS_RPM" => Ok(AccountMetricsEnum::ADREQUESTSRPM),
           "MATCHED_AD_REQUESTS_RPM" => Ok(AccountMetricsEnum::MATCHEDADREQUESTSRPM),
           "IMPRESSIONS_RPM" => Ok(AccountMetricsEnum::IMPRESSIONSRPM),
           "INDIVIDUAL_AD_IMPRESSIONS_RPM" => Ok(AccountMetricsEnum::INDIVIDUALADIMPRESSIONSRPM),
           "COST_PER_CLICK" => Ok(AccountMetricsEnum::COSTPERCLICK),
           "ADS_PER_IMPRESSION" => Ok(AccountMetricsEnum::ADSPERIMPRESSION),
           "TOTAL_EARNINGS" => Ok(AccountMetricsEnum::TOTALEARNINGS),
           "WEBSEARCH_RESULT_PAGES" => Ok(AccountMetricsEnum::WEBSEARCHRESULTPAGES),
           "FUNNEL_REQUESTS" => Ok(AccountMetricsEnum::FUNNELREQUESTS),
           "FUNNEL_IMPRESSIONS" => Ok(AccountMetricsEnum::FUNNELIMPRESSIONS),
           "FUNNEL_CLICKS" => Ok(AccountMetricsEnum::FUNNELCLICKS),
           "FUNNEL_RPM" => Ok(AccountMetricsEnum::FUNNELRPM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountMetricsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


