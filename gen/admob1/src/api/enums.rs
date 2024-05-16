use super::*;



// region AppAppApprovalStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The approval state for the app. The field is read-only.
pub enum AppAppApprovalStateEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "APP_APPROVAL_STATE_UNSPECIFIED"
    #[serde(rename="APP_APPROVAL_STATE_UNSPECIFIED")]
    APPAPPROVALSTATEUNSPECIFIED,
    

    /// The app requires additional user action to be approved. Please refer to https://support.google.com/admob/answer/10564477 for details and next steps.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
    

    /// The app is pending review.
    ///
    /// "IN_REVIEW"
    #[serde(rename="IN_REVIEW")]
    INREVIEW,
    

    /// The app is approved and can serve ads.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
}

impl AsRef<str> for AppAppApprovalStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppAppApprovalStateEnum::APPAPPROVALSTATEUNSPECIFIED => "APP_APPROVAL_STATE_UNSPECIFIED",
            AppAppApprovalStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
            AppAppApprovalStateEnum::INREVIEW => "IN_REVIEW",
            AppAppApprovalStateEnum::APPROVED => "APPROVED",
        }
    }
}

impl std::convert::TryFrom< &str> for AppAppApprovalStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_APPROVAL_STATE_UNSPECIFIED" => Ok(AppAppApprovalStateEnum::APPAPPROVALSTATEUNSPECIFIED),
           "ACTION_REQUIRED" => Ok(AppAppApprovalStateEnum::ACTIONREQUIRED),
           "IN_REVIEW" => Ok(AppAppApprovalStateEnum::INREVIEW),
           "APPROVED" => Ok(AppAppApprovalStateEnum::APPROVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppAppApprovalStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediationReportSpecDimensionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account.
pub enum MediationReportSpecDimensionsEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// A date in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// A month in the YYYYMM format (for example, "202107"). Requests can specify at most one time dimension.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// The date of the first day of a week in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, "5450213213286189855" and "AdMob Network" as label value).
    ///
    /// "AD_SOURCE"
    #[serde(rename="AD_SOURCE")]
    ADSOURCE,
    

    /// The unique ID of the ad source instance (for example, "ca-app-pub-1234:asi:5678" and "AdMob (default)" as label value).
    ///
    /// "AD_SOURCE_INSTANCE"
    #[serde(rename="AD_SOURCE_INSTANCE")]
    ADSOURCEINSTANCE,
    

    /// The unique ID of the ad unit (for example, "ca-app-pub-1234/8790"). If AD_UNIT dimension is specified, then APP is included automatically.
    ///
    /// "AD_UNIT"
    #[serde(rename="AD_UNIT")]
    ADUNIT,
    

    /// The unique ID of the mobile application (for example, "ca-app-pub-1234~1234").
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// The unique ID of the mediation group (for example, "ca-app-pub-1234:mg:1234" and "AdMob (default)" as label value).
    ///
    /// "MEDIATION_GROUP"
    #[serde(rename="MEDIATION_GROUP")]
    MEDIATIONGROUP,
    

    /// CLDR country code of the place where the ad views/clicks occur (for example, "US" or "FR"). This is a geography dimension.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Format of the ad unit (for example, "banner", "native"), an ad delivery dimension.
    ///
    /// "FORMAT"
    #[serde(rename="FORMAT")]
    FORMAT,
    

    /// Mobile OS platform of the app (for example, "Android" or "iOS").
    ///
    /// "PLATFORM"
    #[serde(rename="PLATFORM")]
    PLATFORM,
    

    /// Mobile operating system version, e.g. "iOS 13.5.1".
    ///
    /// "MOBILE_OS_VERSION"
    #[serde(rename="MOBILE_OS_VERSION")]
    MOBILEOSVERSION,
    

    /// GMA SDK version, e.g. "iOS 7.62.0".
    ///
    /// "GMA_SDK_VERSION"
    #[serde(rename="GMA_SDK_VERSION")]
    GMASDKVERSION,
    

    /// For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString.
    ///
    /// "APP_VERSION_NAME"
    #[serde(rename="APP_VERSION_NAME")]
    APPVERSIONNAME,
    

    /// Restriction mode for ads serving (e.g. "Non-personalized ads").
    ///
    /// "SERVING_RESTRICTION"
    #[serde(rename="SERVING_RESTRICTION")]
    SERVINGRESTRICTION,
}

impl AsRef<str> for MediationReportSpecDimensionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediationReportSpecDimensionsEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            MediationReportSpecDimensionsEnum::DATE => "DATE",
            MediationReportSpecDimensionsEnum::MONTH => "MONTH",
            MediationReportSpecDimensionsEnum::WEEK => "WEEK",
            MediationReportSpecDimensionsEnum::ADSOURCE => "AD_SOURCE",
            MediationReportSpecDimensionsEnum::ADSOURCEINSTANCE => "AD_SOURCE_INSTANCE",
            MediationReportSpecDimensionsEnum::ADUNIT => "AD_UNIT",
            MediationReportSpecDimensionsEnum::APP => "APP",
            MediationReportSpecDimensionsEnum::MEDIATIONGROUP => "MEDIATION_GROUP",
            MediationReportSpecDimensionsEnum::COUNTRY => "COUNTRY",
            MediationReportSpecDimensionsEnum::FORMAT => "FORMAT",
            MediationReportSpecDimensionsEnum::PLATFORM => "PLATFORM",
            MediationReportSpecDimensionsEnum::MOBILEOSVERSION => "MOBILE_OS_VERSION",
            MediationReportSpecDimensionsEnum::GMASDKVERSION => "GMA_SDK_VERSION",
            MediationReportSpecDimensionsEnum::APPVERSIONNAME => "APP_VERSION_NAME",
            MediationReportSpecDimensionsEnum::SERVINGRESTRICTION => "SERVING_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for MediationReportSpecDimensionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(MediationReportSpecDimensionsEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(MediationReportSpecDimensionsEnum::DATE),
           "MONTH" => Ok(MediationReportSpecDimensionsEnum::MONTH),
           "WEEK" => Ok(MediationReportSpecDimensionsEnum::WEEK),
           "AD_SOURCE" => Ok(MediationReportSpecDimensionsEnum::ADSOURCE),
           "AD_SOURCE_INSTANCE" => Ok(MediationReportSpecDimensionsEnum::ADSOURCEINSTANCE),
           "AD_UNIT" => Ok(MediationReportSpecDimensionsEnum::ADUNIT),
           "APP" => Ok(MediationReportSpecDimensionsEnum::APP),
           "MEDIATION_GROUP" => Ok(MediationReportSpecDimensionsEnum::MEDIATIONGROUP),
           "COUNTRY" => Ok(MediationReportSpecDimensionsEnum::COUNTRY),
           "FORMAT" => Ok(MediationReportSpecDimensionsEnum::FORMAT),
           "PLATFORM" => Ok(MediationReportSpecDimensionsEnum::PLATFORM),
           "MOBILE_OS_VERSION" => Ok(MediationReportSpecDimensionsEnum::MOBILEOSVERSION),
           "GMA_SDK_VERSION" => Ok(MediationReportSpecDimensionsEnum::GMASDKVERSION),
           "APP_VERSION_NAME" => Ok(MediationReportSpecDimensionsEnum::APPVERSIONNAME),
           "SERVING_RESTRICTION" => Ok(MediationReportSpecDimensionsEnum::SERVINGRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediationReportSpecDimensionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediationReportSpecMetricsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of metrics of the report. A report must specify at least one metric.
pub enum MediationReportSpecMetricsEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// The number of requests. The value is an integer.
    ///
    /// "AD_REQUESTS"
    #[serde(rename="AD_REQUESTS")]
    ADREQUESTS,
    

    /// The number of times a user clicks an ad. The value is an integer.
    ///
    /// "CLICKS"
    #[serde(rename="CLICKS")]
    CLICKS,
    

    /// The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000. Estimated earnings per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated earnings will show 0 for dates prior to October 20, 2019.
    ///
    /// "ESTIMATED_EARNINGS"
    #[serde(rename="ESTIMATED_EARNINGS")]
    ESTIMATEDEARNINGS,
    

    /// The total number of ads shown to users. The value is an integer.
    ///
    /// "IMPRESSIONS"
    #[serde(rename="IMPRESSIONS")]
    IMPRESSIONS,
    

    /// The ratio of clicks over impressions. The value is a double precision (approximate) decimal value.
    ///
    /// "IMPRESSION_CTR"
    #[serde(rename="IMPRESSION_CTR")]
    IMPRESSIONCTR,
    

    /// The number of times ads are returned in response to a request. The value is an integer.
    ///
    /// "MATCHED_REQUESTS"
    #[serde(rename="MATCHED_REQUESTS")]
    MATCHEDREQUESTS,
    

    /// The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value.
    ///
    /// "MATCH_RATE"
    #[serde(rename="MATCH_RATE")]
    MATCHRATE,
    

    /// The third-party ad network's estimated average eCPM. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $2.30 would be represented as 2300000. The estimated average eCPM per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated average eCPM will show 0 for dates prior to October 20, 2019.
    ///
    /// "OBSERVED_ECPM"
    #[serde(rename="OBSERVED_ECPM")]
    OBSERVEDECPM,
}

impl AsRef<str> for MediationReportSpecMetricsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediationReportSpecMetricsEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            MediationReportSpecMetricsEnum::ADREQUESTS => "AD_REQUESTS",
            MediationReportSpecMetricsEnum::CLICKS => "CLICKS",
            MediationReportSpecMetricsEnum::ESTIMATEDEARNINGS => "ESTIMATED_EARNINGS",
            MediationReportSpecMetricsEnum::IMPRESSIONS => "IMPRESSIONS",
            MediationReportSpecMetricsEnum::IMPRESSIONCTR => "IMPRESSION_CTR",
            MediationReportSpecMetricsEnum::MATCHEDREQUESTS => "MATCHED_REQUESTS",
            MediationReportSpecMetricsEnum::MATCHRATE => "MATCH_RATE",
            MediationReportSpecMetricsEnum::OBSERVEDECPM => "OBSERVED_ECPM",
        }
    }
}

impl std::convert::TryFrom< &str> for MediationReportSpecMetricsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(MediationReportSpecMetricsEnum::METRICUNSPECIFIED),
           "AD_REQUESTS" => Ok(MediationReportSpecMetricsEnum::ADREQUESTS),
           "CLICKS" => Ok(MediationReportSpecMetricsEnum::CLICKS),
           "ESTIMATED_EARNINGS" => Ok(MediationReportSpecMetricsEnum::ESTIMATEDEARNINGS),
           "IMPRESSIONS" => Ok(MediationReportSpecMetricsEnum::IMPRESSIONS),
           "IMPRESSION_CTR" => Ok(MediationReportSpecMetricsEnum::IMPRESSIONCTR),
           "MATCHED_REQUESTS" => Ok(MediationReportSpecMetricsEnum::MATCHEDREQUESTS),
           "MATCH_RATE" => Ok(MediationReportSpecMetricsEnum::MATCHRATE),
           "OBSERVED_ECPM" => Ok(MediationReportSpecMetricsEnum::OBSERVEDECPM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediationReportSpecMetricsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediationReportSpecDimensionFilterDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Applies the filter criterion to the specified dimension.
pub enum MediationReportSpecDimensionFilterDimensionEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// A date in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// A month in the YYYYMM format (for example, "202107"). Requests can specify at most one time dimension.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// The date of the first day of a week in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, "5450213213286189855" and "AdMob Network" as label value).
    ///
    /// "AD_SOURCE"
    #[serde(rename="AD_SOURCE")]
    ADSOURCE,
    

    /// The unique ID of the ad source instance (for example, "ca-app-pub-1234:asi:5678" and "AdMob (default)" as label value).
    ///
    /// "AD_SOURCE_INSTANCE"
    #[serde(rename="AD_SOURCE_INSTANCE")]
    ADSOURCEINSTANCE,
    

    /// The unique ID of the ad unit (for example, "ca-app-pub-1234/8790"). If AD_UNIT dimension is specified, then APP is included automatically.
    ///
    /// "AD_UNIT"
    #[serde(rename="AD_UNIT")]
    ADUNIT,
    

    /// The unique ID of the mobile application (for example, "ca-app-pub-1234~1234").
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// The unique ID of the mediation group (for example, "ca-app-pub-1234:mg:1234" and "AdMob (default)" as label value).
    ///
    /// "MEDIATION_GROUP"
    #[serde(rename="MEDIATION_GROUP")]
    MEDIATIONGROUP,
    

    /// CLDR country code of the place where the ad views/clicks occur (for example, "US" or "FR"). This is a geography dimension.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Format of the ad unit (for example, "banner", "native"), an ad delivery dimension.
    ///
    /// "FORMAT"
    #[serde(rename="FORMAT")]
    FORMAT,
    

    /// Mobile OS platform of the app (for example, "Android" or "iOS").
    ///
    /// "PLATFORM"
    #[serde(rename="PLATFORM")]
    PLATFORM,
    

    /// Mobile operating system version, e.g. "iOS 13.5.1".
    ///
    /// "MOBILE_OS_VERSION"
    #[serde(rename="MOBILE_OS_VERSION")]
    MOBILEOSVERSION,
    

    /// GMA SDK version, e.g. "iOS 7.62.0".
    ///
    /// "GMA_SDK_VERSION"
    #[serde(rename="GMA_SDK_VERSION")]
    GMASDKVERSION,
    

    /// For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString.
    ///
    /// "APP_VERSION_NAME"
    #[serde(rename="APP_VERSION_NAME")]
    APPVERSIONNAME,
    

    /// Restriction mode for ads serving (e.g. "Non-personalized ads").
    ///
    /// "SERVING_RESTRICTION"
    #[serde(rename="SERVING_RESTRICTION")]
    SERVINGRESTRICTION,
}

impl AsRef<str> for MediationReportSpecDimensionFilterDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediationReportSpecDimensionFilterDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            MediationReportSpecDimensionFilterDimensionEnum::DATE => "DATE",
            MediationReportSpecDimensionFilterDimensionEnum::MONTH => "MONTH",
            MediationReportSpecDimensionFilterDimensionEnum::WEEK => "WEEK",
            MediationReportSpecDimensionFilterDimensionEnum::ADSOURCE => "AD_SOURCE",
            MediationReportSpecDimensionFilterDimensionEnum::ADSOURCEINSTANCE => "AD_SOURCE_INSTANCE",
            MediationReportSpecDimensionFilterDimensionEnum::ADUNIT => "AD_UNIT",
            MediationReportSpecDimensionFilterDimensionEnum::APP => "APP",
            MediationReportSpecDimensionFilterDimensionEnum::MEDIATIONGROUP => "MEDIATION_GROUP",
            MediationReportSpecDimensionFilterDimensionEnum::COUNTRY => "COUNTRY",
            MediationReportSpecDimensionFilterDimensionEnum::FORMAT => "FORMAT",
            MediationReportSpecDimensionFilterDimensionEnum::PLATFORM => "PLATFORM",
            MediationReportSpecDimensionFilterDimensionEnum::MOBILEOSVERSION => "MOBILE_OS_VERSION",
            MediationReportSpecDimensionFilterDimensionEnum::GMASDKVERSION => "GMA_SDK_VERSION",
            MediationReportSpecDimensionFilterDimensionEnum::APPVERSIONNAME => "APP_VERSION_NAME",
            MediationReportSpecDimensionFilterDimensionEnum::SERVINGRESTRICTION => "SERVING_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for MediationReportSpecDimensionFilterDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(MediationReportSpecDimensionFilterDimensionEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(MediationReportSpecDimensionFilterDimensionEnum::DATE),
           "MONTH" => Ok(MediationReportSpecDimensionFilterDimensionEnum::MONTH),
           "WEEK" => Ok(MediationReportSpecDimensionFilterDimensionEnum::WEEK),
           "AD_SOURCE" => Ok(MediationReportSpecDimensionFilterDimensionEnum::ADSOURCE),
           "AD_SOURCE_INSTANCE" => Ok(MediationReportSpecDimensionFilterDimensionEnum::ADSOURCEINSTANCE),
           "AD_UNIT" => Ok(MediationReportSpecDimensionFilterDimensionEnum::ADUNIT),
           "APP" => Ok(MediationReportSpecDimensionFilterDimensionEnum::APP),
           "MEDIATION_GROUP" => Ok(MediationReportSpecDimensionFilterDimensionEnum::MEDIATIONGROUP),
           "COUNTRY" => Ok(MediationReportSpecDimensionFilterDimensionEnum::COUNTRY),
           "FORMAT" => Ok(MediationReportSpecDimensionFilterDimensionEnum::FORMAT),
           "PLATFORM" => Ok(MediationReportSpecDimensionFilterDimensionEnum::PLATFORM),
           "MOBILE_OS_VERSION" => Ok(MediationReportSpecDimensionFilterDimensionEnum::MOBILEOSVERSION),
           "GMA_SDK_VERSION" => Ok(MediationReportSpecDimensionFilterDimensionEnum::GMASDKVERSION),
           "APP_VERSION_NAME" => Ok(MediationReportSpecDimensionFilterDimensionEnum::APPVERSIONNAME),
           "SERVING_RESTRICTION" => Ok(MediationReportSpecDimensionFilterDimensionEnum::SERVINGRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediationReportSpecDimensionFilterDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediationReportSpecSortConditionDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort by the specified dimension.
pub enum MediationReportSpecSortConditionDimensionEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// A date in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// A month in the YYYYMM format (for example, "202107"). Requests can specify at most one time dimension.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// The date of the first day of a week in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, "5450213213286189855" and "AdMob Network" as label value).
    ///
    /// "AD_SOURCE"
    #[serde(rename="AD_SOURCE")]
    ADSOURCE,
    

    /// The unique ID of the ad source instance (for example, "ca-app-pub-1234:asi:5678" and "AdMob (default)" as label value).
    ///
    /// "AD_SOURCE_INSTANCE"
    #[serde(rename="AD_SOURCE_INSTANCE")]
    ADSOURCEINSTANCE,
    

    /// The unique ID of the ad unit (for example, "ca-app-pub-1234/8790"). If AD_UNIT dimension is specified, then APP is included automatically.
    ///
    /// "AD_UNIT"
    #[serde(rename="AD_UNIT")]
    ADUNIT,
    

    /// The unique ID of the mobile application (for example, "ca-app-pub-1234~1234").
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// The unique ID of the mediation group (for example, "ca-app-pub-1234:mg:1234" and "AdMob (default)" as label value).
    ///
    /// "MEDIATION_GROUP"
    #[serde(rename="MEDIATION_GROUP")]
    MEDIATIONGROUP,
    

    /// CLDR country code of the place where the ad views/clicks occur (for example, "US" or "FR"). This is a geography dimension.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Format of the ad unit (for example, "banner", "native"), an ad delivery dimension.
    ///
    /// "FORMAT"
    #[serde(rename="FORMAT")]
    FORMAT,
    

    /// Mobile OS platform of the app (for example, "Android" or "iOS").
    ///
    /// "PLATFORM"
    #[serde(rename="PLATFORM")]
    PLATFORM,
    

    /// Mobile operating system version, e.g. "iOS 13.5.1".
    ///
    /// "MOBILE_OS_VERSION"
    #[serde(rename="MOBILE_OS_VERSION")]
    MOBILEOSVERSION,
    

    /// GMA SDK version, e.g. "iOS 7.62.0".
    ///
    /// "GMA_SDK_VERSION"
    #[serde(rename="GMA_SDK_VERSION")]
    GMASDKVERSION,
    

    /// For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString.
    ///
    /// "APP_VERSION_NAME"
    #[serde(rename="APP_VERSION_NAME")]
    APPVERSIONNAME,
    

    /// Restriction mode for ads serving (e.g. "Non-personalized ads").
    ///
    /// "SERVING_RESTRICTION"
    #[serde(rename="SERVING_RESTRICTION")]
    SERVINGRESTRICTION,
}

impl AsRef<str> for MediationReportSpecSortConditionDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediationReportSpecSortConditionDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            MediationReportSpecSortConditionDimensionEnum::DATE => "DATE",
            MediationReportSpecSortConditionDimensionEnum::MONTH => "MONTH",
            MediationReportSpecSortConditionDimensionEnum::WEEK => "WEEK",
            MediationReportSpecSortConditionDimensionEnum::ADSOURCE => "AD_SOURCE",
            MediationReportSpecSortConditionDimensionEnum::ADSOURCEINSTANCE => "AD_SOURCE_INSTANCE",
            MediationReportSpecSortConditionDimensionEnum::ADUNIT => "AD_UNIT",
            MediationReportSpecSortConditionDimensionEnum::APP => "APP",
            MediationReportSpecSortConditionDimensionEnum::MEDIATIONGROUP => "MEDIATION_GROUP",
            MediationReportSpecSortConditionDimensionEnum::COUNTRY => "COUNTRY",
            MediationReportSpecSortConditionDimensionEnum::FORMAT => "FORMAT",
            MediationReportSpecSortConditionDimensionEnum::PLATFORM => "PLATFORM",
            MediationReportSpecSortConditionDimensionEnum::MOBILEOSVERSION => "MOBILE_OS_VERSION",
            MediationReportSpecSortConditionDimensionEnum::GMASDKVERSION => "GMA_SDK_VERSION",
            MediationReportSpecSortConditionDimensionEnum::APPVERSIONNAME => "APP_VERSION_NAME",
            MediationReportSpecSortConditionDimensionEnum::SERVINGRESTRICTION => "SERVING_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for MediationReportSpecSortConditionDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(MediationReportSpecSortConditionDimensionEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(MediationReportSpecSortConditionDimensionEnum::DATE),
           "MONTH" => Ok(MediationReportSpecSortConditionDimensionEnum::MONTH),
           "WEEK" => Ok(MediationReportSpecSortConditionDimensionEnum::WEEK),
           "AD_SOURCE" => Ok(MediationReportSpecSortConditionDimensionEnum::ADSOURCE),
           "AD_SOURCE_INSTANCE" => Ok(MediationReportSpecSortConditionDimensionEnum::ADSOURCEINSTANCE),
           "AD_UNIT" => Ok(MediationReportSpecSortConditionDimensionEnum::ADUNIT),
           "APP" => Ok(MediationReportSpecSortConditionDimensionEnum::APP),
           "MEDIATION_GROUP" => Ok(MediationReportSpecSortConditionDimensionEnum::MEDIATIONGROUP),
           "COUNTRY" => Ok(MediationReportSpecSortConditionDimensionEnum::COUNTRY),
           "FORMAT" => Ok(MediationReportSpecSortConditionDimensionEnum::FORMAT),
           "PLATFORM" => Ok(MediationReportSpecSortConditionDimensionEnum::PLATFORM),
           "MOBILE_OS_VERSION" => Ok(MediationReportSpecSortConditionDimensionEnum::MOBILEOSVERSION),
           "GMA_SDK_VERSION" => Ok(MediationReportSpecSortConditionDimensionEnum::GMASDKVERSION),
           "APP_VERSION_NAME" => Ok(MediationReportSpecSortConditionDimensionEnum::APPVERSIONNAME),
           "SERVING_RESTRICTION" => Ok(MediationReportSpecSortConditionDimensionEnum::SERVINGRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediationReportSpecSortConditionDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediationReportSpecSortConditionMetricEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort by the specified metric.
pub enum MediationReportSpecSortConditionMetricEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// The number of requests. The value is an integer.
    ///
    /// "AD_REQUESTS"
    #[serde(rename="AD_REQUESTS")]
    ADREQUESTS,
    

    /// The number of times a user clicks an ad. The value is an integer.
    ///
    /// "CLICKS"
    #[serde(rename="CLICKS")]
    CLICKS,
    

    /// The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000. Estimated earnings per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated earnings will show 0 for dates prior to October 20, 2019.
    ///
    /// "ESTIMATED_EARNINGS"
    #[serde(rename="ESTIMATED_EARNINGS")]
    ESTIMATEDEARNINGS,
    

    /// The total number of ads shown to users. The value is an integer.
    ///
    /// "IMPRESSIONS"
    #[serde(rename="IMPRESSIONS")]
    IMPRESSIONS,
    

    /// The ratio of clicks over impressions. The value is a double precision (approximate) decimal value.
    ///
    /// "IMPRESSION_CTR"
    #[serde(rename="IMPRESSION_CTR")]
    IMPRESSIONCTR,
    

    /// The number of times ads are returned in response to a request. The value is an integer.
    ///
    /// "MATCHED_REQUESTS"
    #[serde(rename="MATCHED_REQUESTS")]
    MATCHEDREQUESTS,
    

    /// The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value.
    ///
    /// "MATCH_RATE"
    #[serde(rename="MATCH_RATE")]
    MATCHRATE,
    

    /// The third-party ad network's estimated average eCPM. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $2.30 would be represented as 2300000. The estimated average eCPM per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated average eCPM will show 0 for dates prior to October 20, 2019.
    ///
    /// "OBSERVED_ECPM"
    #[serde(rename="OBSERVED_ECPM")]
    OBSERVEDECPM,
}

impl AsRef<str> for MediationReportSpecSortConditionMetricEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediationReportSpecSortConditionMetricEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            MediationReportSpecSortConditionMetricEnum::ADREQUESTS => "AD_REQUESTS",
            MediationReportSpecSortConditionMetricEnum::CLICKS => "CLICKS",
            MediationReportSpecSortConditionMetricEnum::ESTIMATEDEARNINGS => "ESTIMATED_EARNINGS",
            MediationReportSpecSortConditionMetricEnum::IMPRESSIONS => "IMPRESSIONS",
            MediationReportSpecSortConditionMetricEnum::IMPRESSIONCTR => "IMPRESSION_CTR",
            MediationReportSpecSortConditionMetricEnum::MATCHEDREQUESTS => "MATCHED_REQUESTS",
            MediationReportSpecSortConditionMetricEnum::MATCHRATE => "MATCH_RATE",
            MediationReportSpecSortConditionMetricEnum::OBSERVEDECPM => "OBSERVED_ECPM",
        }
    }
}

impl std::convert::TryFrom< &str> for MediationReportSpecSortConditionMetricEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(MediationReportSpecSortConditionMetricEnum::METRICUNSPECIFIED),
           "AD_REQUESTS" => Ok(MediationReportSpecSortConditionMetricEnum::ADREQUESTS),
           "CLICKS" => Ok(MediationReportSpecSortConditionMetricEnum::CLICKS),
           "ESTIMATED_EARNINGS" => Ok(MediationReportSpecSortConditionMetricEnum::ESTIMATEDEARNINGS),
           "IMPRESSIONS" => Ok(MediationReportSpecSortConditionMetricEnum::IMPRESSIONS),
           "IMPRESSION_CTR" => Ok(MediationReportSpecSortConditionMetricEnum::IMPRESSIONCTR),
           "MATCHED_REQUESTS" => Ok(MediationReportSpecSortConditionMetricEnum::MATCHEDREQUESTS),
           "MATCH_RATE" => Ok(MediationReportSpecSortConditionMetricEnum::MATCHRATE),
           "OBSERVED_ECPM" => Ok(MediationReportSpecSortConditionMetricEnum::OBSERVEDECPM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediationReportSpecSortConditionMetricEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediationReportSpecSortConditionOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sorting order of the dimension or metric.
pub enum MediationReportSpecSortConditionOrderEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "SORT_ORDER_UNSPECIFIED"
    #[serde(rename="SORT_ORDER_UNSPECIFIED")]
    SORTORDERUNSPECIFIED,
    

    /// Sort dimension value or metric value in ascending order.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Sort dimension value or metric value in descending order.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for MediationReportSpecSortConditionOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediationReportSpecSortConditionOrderEnum::SORTORDERUNSPECIFIED => "SORT_ORDER_UNSPECIFIED",
            MediationReportSpecSortConditionOrderEnum::ASCENDING => "ASCENDING",
            MediationReportSpecSortConditionOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for MediationReportSpecSortConditionOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_ORDER_UNSPECIFIED" => Ok(MediationReportSpecSortConditionOrderEnum::SORTORDERUNSPECIFIED),
           "ASCENDING" => Ok(MediationReportSpecSortConditionOrderEnum::ASCENDING),
           "DESCENDING" => Ok(MediationReportSpecSortConditionOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediationReportSpecSortConditionOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkReportSpecDimensionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account.
pub enum NetworkReportSpecDimensionsEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// A date in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// A month in the YYYYMM format (for example, "202107"). Requests can specify at most one time dimension.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// The date of the first day of a week in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// The unique ID of the ad unit (for example, "ca-app-pub-1234/1234"). If AD_UNIT dimension is specified, then APP is included automatically.
    ///
    /// "AD_UNIT"
    #[serde(rename="AD_UNIT")]
    ADUNIT,
    

    /// The unique ID of the mobile application (for example, "ca-app-pub-1234~1234").
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// Type of the ad (for example, "text" or "image"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics.
    ///
    /// "AD_TYPE"
    #[serde(rename="AD_TYPE")]
    ADTYPE,
    

    /// CLDR country code of the place where the ad views/clicks occur (for example, "US" or "FR"). This is a geography dimension.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Format of the ad unit (for example, "banner", "native"), an ad delivery dimension.
    ///
    /// "FORMAT"
    #[serde(rename="FORMAT")]
    FORMAT,
    

    /// Mobile OS platform of the app (for example, "Android" or "iOS").
    ///
    /// "PLATFORM"
    #[serde(rename="PLATFORM")]
    PLATFORM,
    

    /// Mobile operating system version, e.g. "iOS 13.5.1".
    ///
    /// "MOBILE_OS_VERSION"
    #[serde(rename="MOBILE_OS_VERSION")]
    MOBILEOSVERSION,
    

    /// GMA SDK version, e.g. "iOS 7.62.0".
    ///
    /// "GMA_SDK_VERSION"
    #[serde(rename="GMA_SDK_VERSION")]
    GMASDKVERSION,
    

    /// For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString.
    ///
    /// "APP_VERSION_NAME"
    #[serde(rename="APP_VERSION_NAME")]
    APPVERSIONNAME,
    

    /// Restriction mode for ads serving (e.g. "Non-personalized ads").
    ///
    /// "SERVING_RESTRICTION"
    #[serde(rename="SERVING_RESTRICTION")]
    SERVINGRESTRICTION,
}

impl AsRef<str> for NetworkReportSpecDimensionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkReportSpecDimensionsEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            NetworkReportSpecDimensionsEnum::DATE => "DATE",
            NetworkReportSpecDimensionsEnum::MONTH => "MONTH",
            NetworkReportSpecDimensionsEnum::WEEK => "WEEK",
            NetworkReportSpecDimensionsEnum::ADUNIT => "AD_UNIT",
            NetworkReportSpecDimensionsEnum::APP => "APP",
            NetworkReportSpecDimensionsEnum::ADTYPE => "AD_TYPE",
            NetworkReportSpecDimensionsEnum::COUNTRY => "COUNTRY",
            NetworkReportSpecDimensionsEnum::FORMAT => "FORMAT",
            NetworkReportSpecDimensionsEnum::PLATFORM => "PLATFORM",
            NetworkReportSpecDimensionsEnum::MOBILEOSVERSION => "MOBILE_OS_VERSION",
            NetworkReportSpecDimensionsEnum::GMASDKVERSION => "GMA_SDK_VERSION",
            NetworkReportSpecDimensionsEnum::APPVERSIONNAME => "APP_VERSION_NAME",
            NetworkReportSpecDimensionsEnum::SERVINGRESTRICTION => "SERVING_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkReportSpecDimensionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(NetworkReportSpecDimensionsEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(NetworkReportSpecDimensionsEnum::DATE),
           "MONTH" => Ok(NetworkReportSpecDimensionsEnum::MONTH),
           "WEEK" => Ok(NetworkReportSpecDimensionsEnum::WEEK),
           "AD_UNIT" => Ok(NetworkReportSpecDimensionsEnum::ADUNIT),
           "APP" => Ok(NetworkReportSpecDimensionsEnum::APP),
           "AD_TYPE" => Ok(NetworkReportSpecDimensionsEnum::ADTYPE),
           "COUNTRY" => Ok(NetworkReportSpecDimensionsEnum::COUNTRY),
           "FORMAT" => Ok(NetworkReportSpecDimensionsEnum::FORMAT),
           "PLATFORM" => Ok(NetworkReportSpecDimensionsEnum::PLATFORM),
           "MOBILE_OS_VERSION" => Ok(NetworkReportSpecDimensionsEnum::MOBILEOSVERSION),
           "GMA_SDK_VERSION" => Ok(NetworkReportSpecDimensionsEnum::GMASDKVERSION),
           "APP_VERSION_NAME" => Ok(NetworkReportSpecDimensionsEnum::APPVERSIONNAME),
           "SERVING_RESTRICTION" => Ok(NetworkReportSpecDimensionsEnum::SERVINGRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkReportSpecDimensionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkReportSpecMetricsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of metrics of the report. A report must specify at least one metric.
pub enum NetworkReportSpecMetricsEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// The number of ad requests. The value is an integer. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
    ///
    /// "AD_REQUESTS"
    #[serde(rename="AD_REQUESTS")]
    ADREQUESTS,
    

    /// The number of times a user clicks an ad. The value is an integer.
    ///
    /// "CLICKS"
    #[serde(rename="CLICKS")]
    CLICKS,
    

    /// The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000.
    ///
    /// "ESTIMATED_EARNINGS"
    #[serde(rename="ESTIMATED_EARNINGS")]
    ESTIMATEDEARNINGS,
    

    /// The total number of ads shown to users. The value is an integer.
    ///
    /// "IMPRESSIONS"
    #[serde(rename="IMPRESSIONS")]
    IMPRESSIONS,
    

    /// The ratio of clicks over impressions. The value is a double precision (approximate) decimal value.
    ///
    /// "IMPRESSION_CTR"
    #[serde(rename="IMPRESSION_CTR")]
    IMPRESSIONCTR,
    

    /// The estimated earnings per thousand ad impressions. The value is in micros. For example, $1.03 would be represented as 1030000. Equivalent to eCPM in the AdMob UI. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
    ///
    /// "IMPRESSION_RPM"
    #[serde(rename="IMPRESSION_RPM")]
    IMPRESSIONRPM,
    

    /// The number of times ads are returned in response to a request. The value is an integer.
    ///
    /// "MATCHED_REQUESTS"
    #[serde(rename="MATCHED_REQUESTS")]
    MATCHEDREQUESTS,
    

    /// The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
    ///
    /// "MATCH_RATE"
    #[serde(rename="MATCH_RATE")]
    MATCHRATE,
    

    /// The ratio of ads that are displayed over ads that are returned, defined as impressions / matched requests. The value is a double precision (approximate) decimal value.
    ///
    /// "SHOW_RATE"
    #[serde(rename="SHOW_RATE")]
    SHOWRATE,
}

impl AsRef<str> for NetworkReportSpecMetricsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkReportSpecMetricsEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            NetworkReportSpecMetricsEnum::ADREQUESTS => "AD_REQUESTS",
            NetworkReportSpecMetricsEnum::CLICKS => "CLICKS",
            NetworkReportSpecMetricsEnum::ESTIMATEDEARNINGS => "ESTIMATED_EARNINGS",
            NetworkReportSpecMetricsEnum::IMPRESSIONS => "IMPRESSIONS",
            NetworkReportSpecMetricsEnum::IMPRESSIONCTR => "IMPRESSION_CTR",
            NetworkReportSpecMetricsEnum::IMPRESSIONRPM => "IMPRESSION_RPM",
            NetworkReportSpecMetricsEnum::MATCHEDREQUESTS => "MATCHED_REQUESTS",
            NetworkReportSpecMetricsEnum::MATCHRATE => "MATCH_RATE",
            NetworkReportSpecMetricsEnum::SHOWRATE => "SHOW_RATE",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkReportSpecMetricsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(NetworkReportSpecMetricsEnum::METRICUNSPECIFIED),
           "AD_REQUESTS" => Ok(NetworkReportSpecMetricsEnum::ADREQUESTS),
           "CLICKS" => Ok(NetworkReportSpecMetricsEnum::CLICKS),
           "ESTIMATED_EARNINGS" => Ok(NetworkReportSpecMetricsEnum::ESTIMATEDEARNINGS),
           "IMPRESSIONS" => Ok(NetworkReportSpecMetricsEnum::IMPRESSIONS),
           "IMPRESSION_CTR" => Ok(NetworkReportSpecMetricsEnum::IMPRESSIONCTR),
           "IMPRESSION_RPM" => Ok(NetworkReportSpecMetricsEnum::IMPRESSIONRPM),
           "MATCHED_REQUESTS" => Ok(NetworkReportSpecMetricsEnum::MATCHEDREQUESTS),
           "MATCH_RATE" => Ok(NetworkReportSpecMetricsEnum::MATCHRATE),
           "SHOW_RATE" => Ok(NetworkReportSpecMetricsEnum::SHOWRATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkReportSpecMetricsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkReportSpecDimensionFilterDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Applies the filter criterion to the specified dimension.
pub enum NetworkReportSpecDimensionFilterDimensionEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// A date in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// A month in the YYYYMM format (for example, "202107"). Requests can specify at most one time dimension.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// The date of the first day of a week in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// The unique ID of the ad unit (for example, "ca-app-pub-1234/1234"). If AD_UNIT dimension is specified, then APP is included automatically.
    ///
    /// "AD_UNIT"
    #[serde(rename="AD_UNIT")]
    ADUNIT,
    

    /// The unique ID of the mobile application (for example, "ca-app-pub-1234~1234").
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// Type of the ad (for example, "text" or "image"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics.
    ///
    /// "AD_TYPE"
    #[serde(rename="AD_TYPE")]
    ADTYPE,
    

    /// CLDR country code of the place where the ad views/clicks occur (for example, "US" or "FR"). This is a geography dimension.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Format of the ad unit (for example, "banner", "native"), an ad delivery dimension.
    ///
    /// "FORMAT"
    #[serde(rename="FORMAT")]
    FORMAT,
    

    /// Mobile OS platform of the app (for example, "Android" or "iOS").
    ///
    /// "PLATFORM"
    #[serde(rename="PLATFORM")]
    PLATFORM,
    

    /// Mobile operating system version, e.g. "iOS 13.5.1".
    ///
    /// "MOBILE_OS_VERSION"
    #[serde(rename="MOBILE_OS_VERSION")]
    MOBILEOSVERSION,
    

    /// GMA SDK version, e.g. "iOS 7.62.0".
    ///
    /// "GMA_SDK_VERSION"
    #[serde(rename="GMA_SDK_VERSION")]
    GMASDKVERSION,
    

    /// For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString.
    ///
    /// "APP_VERSION_NAME"
    #[serde(rename="APP_VERSION_NAME")]
    APPVERSIONNAME,
    

    /// Restriction mode for ads serving (e.g. "Non-personalized ads").
    ///
    /// "SERVING_RESTRICTION"
    #[serde(rename="SERVING_RESTRICTION")]
    SERVINGRESTRICTION,
}

impl AsRef<str> for NetworkReportSpecDimensionFilterDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkReportSpecDimensionFilterDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            NetworkReportSpecDimensionFilterDimensionEnum::DATE => "DATE",
            NetworkReportSpecDimensionFilterDimensionEnum::MONTH => "MONTH",
            NetworkReportSpecDimensionFilterDimensionEnum::WEEK => "WEEK",
            NetworkReportSpecDimensionFilterDimensionEnum::ADUNIT => "AD_UNIT",
            NetworkReportSpecDimensionFilterDimensionEnum::APP => "APP",
            NetworkReportSpecDimensionFilterDimensionEnum::ADTYPE => "AD_TYPE",
            NetworkReportSpecDimensionFilterDimensionEnum::COUNTRY => "COUNTRY",
            NetworkReportSpecDimensionFilterDimensionEnum::FORMAT => "FORMAT",
            NetworkReportSpecDimensionFilterDimensionEnum::PLATFORM => "PLATFORM",
            NetworkReportSpecDimensionFilterDimensionEnum::MOBILEOSVERSION => "MOBILE_OS_VERSION",
            NetworkReportSpecDimensionFilterDimensionEnum::GMASDKVERSION => "GMA_SDK_VERSION",
            NetworkReportSpecDimensionFilterDimensionEnum::APPVERSIONNAME => "APP_VERSION_NAME",
            NetworkReportSpecDimensionFilterDimensionEnum::SERVINGRESTRICTION => "SERVING_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkReportSpecDimensionFilterDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::DATE),
           "MONTH" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::MONTH),
           "WEEK" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::WEEK),
           "AD_UNIT" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::ADUNIT),
           "APP" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::APP),
           "AD_TYPE" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::ADTYPE),
           "COUNTRY" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::COUNTRY),
           "FORMAT" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::FORMAT),
           "PLATFORM" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::PLATFORM),
           "MOBILE_OS_VERSION" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::MOBILEOSVERSION),
           "GMA_SDK_VERSION" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::GMASDKVERSION),
           "APP_VERSION_NAME" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::APPVERSIONNAME),
           "SERVING_RESTRICTION" => Ok(NetworkReportSpecDimensionFilterDimensionEnum::SERVINGRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkReportSpecDimensionFilterDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkReportSpecSortConditionDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort by the specified dimension.
pub enum NetworkReportSpecSortConditionDimensionEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "DIMENSION_UNSPECIFIED"
    #[serde(rename="DIMENSION_UNSPECIFIED")]
    DIMENSIONUNSPECIFIED,
    

    /// A date in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// A month in the YYYYMM format (for example, "202107"). Requests can specify at most one time dimension.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// The date of the first day of a week in the YYYYMMDD format (for example, "20210701"). Requests can specify at most one time dimension.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// The unique ID of the ad unit (for example, "ca-app-pub-1234/1234"). If AD_UNIT dimension is specified, then APP is included automatically.
    ///
    /// "AD_UNIT"
    #[serde(rename="AD_UNIT")]
    ADUNIT,
    

    /// The unique ID of the mobile application (for example, "ca-app-pub-1234~1234").
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// Type of the ad (for example, "text" or "image"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics.
    ///
    /// "AD_TYPE"
    #[serde(rename="AD_TYPE")]
    ADTYPE,
    

    /// CLDR country code of the place where the ad views/clicks occur (for example, "US" or "FR"). This is a geography dimension.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// Format of the ad unit (for example, "banner", "native"), an ad delivery dimension.
    ///
    /// "FORMAT"
    #[serde(rename="FORMAT")]
    FORMAT,
    

    /// Mobile OS platform of the app (for example, "Android" or "iOS").
    ///
    /// "PLATFORM"
    #[serde(rename="PLATFORM")]
    PLATFORM,
    

    /// Mobile operating system version, e.g. "iOS 13.5.1".
    ///
    /// "MOBILE_OS_VERSION"
    #[serde(rename="MOBILE_OS_VERSION")]
    MOBILEOSVERSION,
    

    /// GMA SDK version, e.g. "iOS 7.62.0".
    ///
    /// "GMA_SDK_VERSION"
    #[serde(rename="GMA_SDK_VERSION")]
    GMASDKVERSION,
    

    /// For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString.
    ///
    /// "APP_VERSION_NAME"
    #[serde(rename="APP_VERSION_NAME")]
    APPVERSIONNAME,
    

    /// Restriction mode for ads serving (e.g. "Non-personalized ads").
    ///
    /// "SERVING_RESTRICTION"
    #[serde(rename="SERVING_RESTRICTION")]
    SERVINGRESTRICTION,
}

impl AsRef<str> for NetworkReportSpecSortConditionDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkReportSpecSortConditionDimensionEnum::DIMENSIONUNSPECIFIED => "DIMENSION_UNSPECIFIED",
            NetworkReportSpecSortConditionDimensionEnum::DATE => "DATE",
            NetworkReportSpecSortConditionDimensionEnum::MONTH => "MONTH",
            NetworkReportSpecSortConditionDimensionEnum::WEEK => "WEEK",
            NetworkReportSpecSortConditionDimensionEnum::ADUNIT => "AD_UNIT",
            NetworkReportSpecSortConditionDimensionEnum::APP => "APP",
            NetworkReportSpecSortConditionDimensionEnum::ADTYPE => "AD_TYPE",
            NetworkReportSpecSortConditionDimensionEnum::COUNTRY => "COUNTRY",
            NetworkReportSpecSortConditionDimensionEnum::FORMAT => "FORMAT",
            NetworkReportSpecSortConditionDimensionEnum::PLATFORM => "PLATFORM",
            NetworkReportSpecSortConditionDimensionEnum::MOBILEOSVERSION => "MOBILE_OS_VERSION",
            NetworkReportSpecSortConditionDimensionEnum::GMASDKVERSION => "GMA_SDK_VERSION",
            NetworkReportSpecSortConditionDimensionEnum::APPVERSIONNAME => "APP_VERSION_NAME",
            NetworkReportSpecSortConditionDimensionEnum::SERVINGRESTRICTION => "SERVING_RESTRICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkReportSpecSortConditionDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIMENSION_UNSPECIFIED" => Ok(NetworkReportSpecSortConditionDimensionEnum::DIMENSIONUNSPECIFIED),
           "DATE" => Ok(NetworkReportSpecSortConditionDimensionEnum::DATE),
           "MONTH" => Ok(NetworkReportSpecSortConditionDimensionEnum::MONTH),
           "WEEK" => Ok(NetworkReportSpecSortConditionDimensionEnum::WEEK),
           "AD_UNIT" => Ok(NetworkReportSpecSortConditionDimensionEnum::ADUNIT),
           "APP" => Ok(NetworkReportSpecSortConditionDimensionEnum::APP),
           "AD_TYPE" => Ok(NetworkReportSpecSortConditionDimensionEnum::ADTYPE),
           "COUNTRY" => Ok(NetworkReportSpecSortConditionDimensionEnum::COUNTRY),
           "FORMAT" => Ok(NetworkReportSpecSortConditionDimensionEnum::FORMAT),
           "PLATFORM" => Ok(NetworkReportSpecSortConditionDimensionEnum::PLATFORM),
           "MOBILE_OS_VERSION" => Ok(NetworkReportSpecSortConditionDimensionEnum::MOBILEOSVERSION),
           "GMA_SDK_VERSION" => Ok(NetworkReportSpecSortConditionDimensionEnum::GMASDKVERSION),
           "APP_VERSION_NAME" => Ok(NetworkReportSpecSortConditionDimensionEnum::APPVERSIONNAME),
           "SERVING_RESTRICTION" => Ok(NetworkReportSpecSortConditionDimensionEnum::SERVINGRESTRICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkReportSpecSortConditionDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkReportSpecSortConditionMetricEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort by the specified metric.
pub enum NetworkReportSpecSortConditionMetricEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// The number of ad requests. The value is an integer. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
    ///
    /// "AD_REQUESTS"
    #[serde(rename="AD_REQUESTS")]
    ADREQUESTS,
    

    /// The number of times a user clicks an ad. The value is an integer.
    ///
    /// "CLICKS"
    #[serde(rename="CLICKS")]
    CLICKS,
    

    /// The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000.
    ///
    /// "ESTIMATED_EARNINGS"
    #[serde(rename="ESTIMATED_EARNINGS")]
    ESTIMATEDEARNINGS,
    

    /// The total number of ads shown to users. The value is an integer.
    ///
    /// "IMPRESSIONS"
    #[serde(rename="IMPRESSIONS")]
    IMPRESSIONS,
    

    /// The ratio of clicks over impressions. The value is a double precision (approximate) decimal value.
    ///
    /// "IMPRESSION_CTR"
    #[serde(rename="IMPRESSION_CTR")]
    IMPRESSIONCTR,
    

    /// The estimated earnings per thousand ad impressions. The value is in micros. For example, $1.03 would be represented as 1030000. Equivalent to eCPM in the AdMob UI. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
    ///
    /// "IMPRESSION_RPM"
    #[serde(rename="IMPRESSION_RPM")]
    IMPRESSIONRPM,
    

    /// The number of times ads are returned in response to a request. The value is an integer.
    ///
    /// "MATCHED_REQUESTS"
    #[serde(rename="MATCHED_REQUESTS")]
    MATCHEDREQUESTS,
    

    /// The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension.
    ///
    /// "MATCH_RATE"
    #[serde(rename="MATCH_RATE")]
    MATCHRATE,
    

    /// The ratio of ads that are displayed over ads that are returned, defined as impressions / matched requests. The value is a double precision (approximate) decimal value.
    ///
    /// "SHOW_RATE"
    #[serde(rename="SHOW_RATE")]
    SHOWRATE,
}

impl AsRef<str> for NetworkReportSpecSortConditionMetricEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkReportSpecSortConditionMetricEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            NetworkReportSpecSortConditionMetricEnum::ADREQUESTS => "AD_REQUESTS",
            NetworkReportSpecSortConditionMetricEnum::CLICKS => "CLICKS",
            NetworkReportSpecSortConditionMetricEnum::ESTIMATEDEARNINGS => "ESTIMATED_EARNINGS",
            NetworkReportSpecSortConditionMetricEnum::IMPRESSIONS => "IMPRESSIONS",
            NetworkReportSpecSortConditionMetricEnum::IMPRESSIONCTR => "IMPRESSION_CTR",
            NetworkReportSpecSortConditionMetricEnum::IMPRESSIONRPM => "IMPRESSION_RPM",
            NetworkReportSpecSortConditionMetricEnum::MATCHEDREQUESTS => "MATCHED_REQUESTS",
            NetworkReportSpecSortConditionMetricEnum::MATCHRATE => "MATCH_RATE",
            NetworkReportSpecSortConditionMetricEnum::SHOWRATE => "SHOW_RATE",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkReportSpecSortConditionMetricEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(NetworkReportSpecSortConditionMetricEnum::METRICUNSPECIFIED),
           "AD_REQUESTS" => Ok(NetworkReportSpecSortConditionMetricEnum::ADREQUESTS),
           "CLICKS" => Ok(NetworkReportSpecSortConditionMetricEnum::CLICKS),
           "ESTIMATED_EARNINGS" => Ok(NetworkReportSpecSortConditionMetricEnum::ESTIMATEDEARNINGS),
           "IMPRESSIONS" => Ok(NetworkReportSpecSortConditionMetricEnum::IMPRESSIONS),
           "IMPRESSION_CTR" => Ok(NetworkReportSpecSortConditionMetricEnum::IMPRESSIONCTR),
           "IMPRESSION_RPM" => Ok(NetworkReportSpecSortConditionMetricEnum::IMPRESSIONRPM),
           "MATCHED_REQUESTS" => Ok(NetworkReportSpecSortConditionMetricEnum::MATCHEDREQUESTS),
           "MATCH_RATE" => Ok(NetworkReportSpecSortConditionMetricEnum::MATCHRATE),
           "SHOW_RATE" => Ok(NetworkReportSpecSortConditionMetricEnum::SHOWRATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkReportSpecSortConditionMetricEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkReportSpecSortConditionOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sorting order of the dimension or metric.
pub enum NetworkReportSpecSortConditionOrderEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "SORT_ORDER_UNSPECIFIED"
    #[serde(rename="SORT_ORDER_UNSPECIFIED")]
    SORTORDERUNSPECIFIED,
    

    /// Sort dimension value or metric value in ascending order.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Sort dimension value or metric value in descending order.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for NetworkReportSpecSortConditionOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkReportSpecSortConditionOrderEnum::SORTORDERUNSPECIFIED => "SORT_ORDER_UNSPECIFIED",
            NetworkReportSpecSortConditionOrderEnum::ASCENDING => "ASCENDING",
            NetworkReportSpecSortConditionOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkReportSpecSortConditionOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_ORDER_UNSPECIFIED" => Ok(NetworkReportSpecSortConditionOrderEnum::SORTORDERUNSPECIFIED),
           "ASCENDING" => Ok(NetworkReportSpecSortConditionOrderEnum::ASCENDING),
           "DESCENDING" => Ok(NetworkReportSpecSortConditionOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkReportSpecSortConditionOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportWarningTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the warning.
pub enum ReportWarningTypeEnum {
    

    /// Default value for an unset field. Do not use.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Some data in this report is aggregated based on a time zone different from the requested time zone. This could happen if a local time-zone report has the start time before the last time this time zone changed. The description field will contain the date of the last time zone change.
    ///
    /// "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE"
    #[serde(rename="DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE")]
    DATABEFOREACCOUNTTIMEZONECHANGE,
    

    /// There is an unusual delay in processing the source data for the requested date range. The report results might be less up to date than usual. AdMob is aware of the issue and is actively working to resolve it.
    ///
    /// "DATA_DELAYED"
    #[serde(rename="DATA_DELAYED")]
    DATADELAYED,
    

    /// Warnings that are exposed without a specific type. Useful when new warning types are added but the API is not changed yet.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    

    /// The currency being requested is not the account currency. The earning metrics will be based on the requested currency, and thus not a good estimation of the final payment anymore, due to the currency rate fluctuation.
    ///
    /// "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY"
    #[serde(rename="REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY")]
    REPORTCURRENCYNOTACCOUNTCURRENCY,
}

impl AsRef<str> for ReportWarningTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportWarningTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ReportWarningTypeEnum::DATABEFOREACCOUNTTIMEZONECHANGE => "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE",
            ReportWarningTypeEnum::DATADELAYED => "DATA_DELAYED",
            ReportWarningTypeEnum::OTHER => "OTHER",
            ReportWarningTypeEnum::REPORTCURRENCYNOTACCOUNTCURRENCY => "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportWarningTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ReportWarningTypeEnum::TYPEUNSPECIFIED),
           "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE" => Ok(ReportWarningTypeEnum::DATABEFOREACCOUNTTIMEZONECHANGE),
           "DATA_DELAYED" => Ok(ReportWarningTypeEnum::DATADELAYED),
           "OTHER" => Ok(ReportWarningTypeEnum::OTHER),
           "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY" => Ok(ReportWarningTypeEnum::REPORTCURRENCYNOTACCOUNTCURRENCY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportWarningTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


