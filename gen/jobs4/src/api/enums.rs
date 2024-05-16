use super::*;



// region CommuteFilterCommuteMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The method of transportation to calculate the commute time for.
pub enum CommuteFilterCommuteMethodEnum {
    

    /// Commute method isn't specified.
    ///
    /// "COMMUTE_METHOD_UNSPECIFIED"
    #[serde(rename="COMMUTE_METHOD_UNSPECIFIED")]
    COMMUTEMETHODUNSPECIFIED,
    

    /// Commute time is calculated based on driving time.
    ///
    /// "DRIVING"
    #[serde(rename="DRIVING")]
    DRIVING,
    

    /// Commute time is calculated based on public transit including bus, metro, subway, and so on.
    ///
    /// "TRANSIT"
    #[serde(rename="TRANSIT")]
    TRANSIT,
    

    /// Commute time is calculated based on walking time.
    ///
    /// "WALKING"
    #[serde(rename="WALKING")]
    WALKING,
    

    /// Commute time is calculated based on biking time.
    ///
    /// "CYCLING"
    #[serde(rename="CYCLING")]
    CYCLING,
    

    /// Commute time is calculated based on public transit that is wheelchair accessible.
    ///
    /// "TRANSIT_ACCESSIBLE"
    #[serde(rename="TRANSIT_ACCESSIBLE")]
    TRANSITACCESSIBLE,
}

impl AsRef<str> for CommuteFilterCommuteMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommuteFilterCommuteMethodEnum::COMMUTEMETHODUNSPECIFIED => "COMMUTE_METHOD_UNSPECIFIED",
            CommuteFilterCommuteMethodEnum::DRIVING => "DRIVING",
            CommuteFilterCommuteMethodEnum::TRANSIT => "TRANSIT",
            CommuteFilterCommuteMethodEnum::WALKING => "WALKING",
            CommuteFilterCommuteMethodEnum::CYCLING => "CYCLING",
            CommuteFilterCommuteMethodEnum::TRANSITACCESSIBLE => "TRANSIT_ACCESSIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CommuteFilterCommuteMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMUTE_METHOD_UNSPECIFIED" => Ok(CommuteFilterCommuteMethodEnum::COMMUTEMETHODUNSPECIFIED),
           "DRIVING" => Ok(CommuteFilterCommuteMethodEnum::DRIVING),
           "TRANSIT" => Ok(CommuteFilterCommuteMethodEnum::TRANSIT),
           "WALKING" => Ok(CommuteFilterCommuteMethodEnum::WALKING),
           "CYCLING" => Ok(CommuteFilterCommuteMethodEnum::CYCLING),
           "TRANSIT_ACCESSIBLE" => Ok(CommuteFilterCommuteMethodEnum::TRANSITACCESSIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommuteFilterCommuteMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommuteFilterRoadTrafficEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the traffic density to use when calculating commute time.
pub enum CommuteFilterRoadTrafficEnum {
    

    /// Road traffic situation isn't specified.
    ///
    /// "ROAD_TRAFFIC_UNSPECIFIED"
    #[serde(rename="ROAD_TRAFFIC_UNSPECIFIED")]
    ROADTRAFFICUNSPECIFIED,
    

    /// Optimal commute time without considering any traffic impact.
    ///
    /// "TRAFFIC_FREE"
    #[serde(rename="TRAFFIC_FREE")]
    TRAFFICFREE,
    

    /// Commute time calculation takes in account the peak traffic impact.
    ///
    /// "BUSY_HOUR"
    #[serde(rename="BUSY_HOUR")]
    BUSYHOUR,
}

impl AsRef<str> for CommuteFilterRoadTrafficEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommuteFilterRoadTrafficEnum::ROADTRAFFICUNSPECIFIED => "ROAD_TRAFFIC_UNSPECIFIED",
            CommuteFilterRoadTrafficEnum::TRAFFICFREE => "TRAFFIC_FREE",
            CommuteFilterRoadTrafficEnum::BUSYHOUR => "BUSY_HOUR",
        }
    }
}

impl std::convert::TryFrom< &str> for CommuteFilterRoadTrafficEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROAD_TRAFFIC_UNSPECIFIED" => Ok(CommuteFilterRoadTrafficEnum::ROADTRAFFICUNSPECIFIED),
           "TRAFFIC_FREE" => Ok(CommuteFilterRoadTrafficEnum::TRAFFICFREE),
           "BUSY_HOUR" => Ok(CommuteFilterRoadTrafficEnum::BUSYHOUR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommuteFilterRoadTrafficEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompanySizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The employer's company size.
pub enum CompanySizeEnum {
    

    /// Default value if the size isn't specified.
    ///
    /// "COMPANY_SIZE_UNSPECIFIED"
    #[serde(rename="COMPANY_SIZE_UNSPECIFIED")]
    COMPANYSIZEUNSPECIFIED,
    

    /// The company has less than 50 employees.
    ///
    /// "MINI"
    #[serde(rename="MINI")]
    MINI,
    

    /// The company has between 50 and 99 employees.
    ///
    /// "SMALL"
    #[serde(rename="SMALL")]
    SMALL,
    

    /// The company has between 100 and 499 employees.
    ///
    /// "SMEDIUM"
    #[serde(rename="SMEDIUM")]
    SMEDIUM,
    

    /// The company has between 500 and 999 employees.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// The company has between 1,000 and 4,999 employees.
    ///
    /// "BIG"
    #[serde(rename="BIG")]
    BIG,
    

    /// The company has between 5,000 and 9,999 employees.
    ///
    /// "BIGGER"
    #[serde(rename="BIGGER")]
    BIGGER,
    

    /// The company has 10,000 or more employees.
    ///
    /// "GIANT"
    #[serde(rename="GIANT")]
    GIANT,
}

impl AsRef<str> for CompanySizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompanySizeEnum::COMPANYSIZEUNSPECIFIED => "COMPANY_SIZE_UNSPECIFIED",
            CompanySizeEnum::MINI => "MINI",
            CompanySizeEnum::SMALL => "SMALL",
            CompanySizeEnum::SMEDIUM => "SMEDIUM",
            CompanySizeEnum::MEDIUM => "MEDIUM",
            CompanySizeEnum::BIG => "BIG",
            CompanySizeEnum::BIGGER => "BIGGER",
            CompanySizeEnum::GIANT => "GIANT",
        }
    }
}

impl std::convert::TryFrom< &str> for CompanySizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPANY_SIZE_UNSPECIFIED" => Ok(CompanySizeEnum::COMPANYSIZEUNSPECIFIED),
           "MINI" => Ok(CompanySizeEnum::MINI),
           "SMALL" => Ok(CompanySizeEnum::SMALL),
           "SMEDIUM" => Ok(CompanySizeEnum::SMEDIUM),
           "MEDIUM" => Ok(CompanySizeEnum::MEDIUM),
           "BIG" => Ok(CompanySizeEnum::BIG),
           "BIGGER" => Ok(CompanySizeEnum::BIGGER),
           "GIANT" => Ok(CompanySizeEnum::GIANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompanySizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompensationEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compensation type. Default is CompensationType.COMPENSATION_TYPE_UNSPECIFIED.
pub enum CompensationEntryTypeEnum {
    

    /// Default value.
    ///
    /// "COMPENSATION_TYPE_UNSPECIFIED"
    #[serde(rename="COMPENSATION_TYPE_UNSPECIFIED")]
    COMPENSATIONTYPEUNSPECIFIED,
    

    /// Base compensation: Refers to the fixed amount of money paid to an employee by an employer in return for work performed. Base compensation does not include benefits, bonuses or any other potential compensation from an employer.
    ///
    /// "BASE"
    #[serde(rename="BASE")]
    BASE,
    

    /// Bonus.
    ///
    /// "BONUS"
    #[serde(rename="BONUS")]
    BONUS,
    

    /// Signing bonus.
    ///
    /// "SIGNING_BONUS"
    #[serde(rename="SIGNING_BONUS")]
    SIGNINGBONUS,
    

    /// Equity.
    ///
    /// "EQUITY"
    #[serde(rename="EQUITY")]
    EQUITY,
    

    /// Profit sharing.
    ///
    /// "PROFIT_SHARING"
    #[serde(rename="PROFIT_SHARING")]
    PROFITSHARING,
    

    /// Commission.
    ///
    /// "COMMISSIONS"
    #[serde(rename="COMMISSIONS")]
    COMMISSIONS,
    

    /// Tips.
    ///
    /// "TIPS"
    #[serde(rename="TIPS")]
    TIPS,
    

    /// Other compensation type.
    ///
    /// "OTHER_COMPENSATION_TYPE"
    #[serde(rename="OTHER_COMPENSATION_TYPE")]
    OTHERCOMPENSATIONTYPE,
}

impl AsRef<str> for CompensationEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompensationEntryTypeEnum::COMPENSATIONTYPEUNSPECIFIED => "COMPENSATION_TYPE_UNSPECIFIED",
            CompensationEntryTypeEnum::BASE => "BASE",
            CompensationEntryTypeEnum::BONUS => "BONUS",
            CompensationEntryTypeEnum::SIGNINGBONUS => "SIGNING_BONUS",
            CompensationEntryTypeEnum::EQUITY => "EQUITY",
            CompensationEntryTypeEnum::PROFITSHARING => "PROFIT_SHARING",
            CompensationEntryTypeEnum::COMMISSIONS => "COMMISSIONS",
            CompensationEntryTypeEnum::TIPS => "TIPS",
            CompensationEntryTypeEnum::OTHERCOMPENSATIONTYPE => "OTHER_COMPENSATION_TYPE",
        }
    }
}

impl std::convert::TryFrom< &str> for CompensationEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPENSATION_TYPE_UNSPECIFIED" => Ok(CompensationEntryTypeEnum::COMPENSATIONTYPEUNSPECIFIED),
           "BASE" => Ok(CompensationEntryTypeEnum::BASE),
           "BONUS" => Ok(CompensationEntryTypeEnum::BONUS),
           "SIGNING_BONUS" => Ok(CompensationEntryTypeEnum::SIGNINGBONUS),
           "EQUITY" => Ok(CompensationEntryTypeEnum::EQUITY),
           "PROFIT_SHARING" => Ok(CompensationEntryTypeEnum::PROFITSHARING),
           "COMMISSIONS" => Ok(CompensationEntryTypeEnum::COMMISSIONS),
           "TIPS" => Ok(CompensationEntryTypeEnum::TIPS),
           "OTHER_COMPENSATION_TYPE" => Ok(CompensationEntryTypeEnum::OTHERCOMPENSATIONTYPE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompensationEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompensationEntryUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Frequency of the specified amount. Default is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED.
pub enum CompensationEntryUnitEnum {
    

    /// Default value.
    ///
    /// "COMPENSATION_UNIT_UNSPECIFIED"
    #[serde(rename="COMPENSATION_UNIT_UNSPECIFIED")]
    COMPENSATIONUNITUNSPECIFIED,
    

    /// Hourly.
    ///
    /// "HOURLY"
    #[serde(rename="HOURLY")]
    HOURLY,
    

    /// Daily.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
    

    /// Weekly
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// Monthly.
    ///
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
    

    /// Yearly.
    ///
    /// "YEARLY"
    #[serde(rename="YEARLY")]
    YEARLY,
    

    /// One time.
    ///
    /// "ONE_TIME"
    #[serde(rename="ONE_TIME")]
    ONETIME,
    

    /// Other compensation units.
    ///
    /// "OTHER_COMPENSATION_UNIT"
    #[serde(rename="OTHER_COMPENSATION_UNIT")]
    OTHERCOMPENSATIONUNIT,
}

impl AsRef<str> for CompensationEntryUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompensationEntryUnitEnum::COMPENSATIONUNITUNSPECIFIED => "COMPENSATION_UNIT_UNSPECIFIED",
            CompensationEntryUnitEnum::HOURLY => "HOURLY",
            CompensationEntryUnitEnum::DAILY => "DAILY",
            CompensationEntryUnitEnum::WEEKLY => "WEEKLY",
            CompensationEntryUnitEnum::MONTHLY => "MONTHLY",
            CompensationEntryUnitEnum::YEARLY => "YEARLY",
            CompensationEntryUnitEnum::ONETIME => "ONE_TIME",
            CompensationEntryUnitEnum::OTHERCOMPENSATIONUNIT => "OTHER_COMPENSATION_UNIT",
        }
    }
}

impl std::convert::TryFrom< &str> for CompensationEntryUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPENSATION_UNIT_UNSPECIFIED" => Ok(CompensationEntryUnitEnum::COMPENSATIONUNITUNSPECIFIED),
           "HOURLY" => Ok(CompensationEntryUnitEnum::HOURLY),
           "DAILY" => Ok(CompensationEntryUnitEnum::DAILY),
           "WEEKLY" => Ok(CompensationEntryUnitEnum::WEEKLY),
           "MONTHLY" => Ok(CompensationEntryUnitEnum::MONTHLY),
           "YEARLY" => Ok(CompensationEntryUnitEnum::YEARLY),
           "ONE_TIME" => Ok(CompensationEntryUnitEnum::ONETIME),
           "OTHER_COMPENSATION_UNIT" => Ok(CompensationEntryUnitEnum::OTHERCOMPENSATIONUNIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompensationEntryUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompensationFilterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of filter.
pub enum CompensationFilterTypeEnum {
    

    /// Filter type unspecified. Position holder, INVALID, should never be used.
    ///
    /// "FILTER_TYPE_UNSPECIFIED"
    #[serde(rename="FILTER_TYPE_UNSPECIFIED")]
    FILTERTYPEUNSPECIFIED,
    

    /// Filter by `base compensation entry's` unit. A job is a match if and only if the job contains a base CompensationEntry and the base CompensationEntry's unit matches provided units. Populate one or more units. See CompensationInfo.CompensationEntry for definition of base compensation entry.
    ///
    /// "UNIT_ONLY"
    #[serde(rename="UNIT_ONLY")]
    UNITONLY,
    

    /// Filter by `base compensation entry's` unit and amount / range. A job is a match if and only if the job contains a base CompensationEntry, and the base entry's unit matches provided CompensationUnit and amount or range overlaps with provided CompensationRange. See CompensationInfo.CompensationEntry for definition of base compensation entry. Set exactly one units and populate range.
    ///
    /// "UNIT_AND_AMOUNT"
    #[serde(rename="UNIT_AND_AMOUNT")]
    UNITANDAMOUNT,
    

    /// Filter by annualized base compensation amount and `base compensation entry's` unit. Populate range and zero or more units.
    ///
    /// "ANNUALIZED_BASE_AMOUNT"
    #[serde(rename="ANNUALIZED_BASE_AMOUNT")]
    ANNUALIZEDBASEAMOUNT,
    

    /// Filter by annualized total compensation amount and `base compensation entry's` unit . Populate range and zero or more units.
    ///
    /// "ANNUALIZED_TOTAL_AMOUNT"
    #[serde(rename="ANNUALIZED_TOTAL_AMOUNT")]
    ANNUALIZEDTOTALAMOUNT,
}

impl AsRef<str> for CompensationFilterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompensationFilterTypeEnum::FILTERTYPEUNSPECIFIED => "FILTER_TYPE_UNSPECIFIED",
            CompensationFilterTypeEnum::UNITONLY => "UNIT_ONLY",
            CompensationFilterTypeEnum::UNITANDAMOUNT => "UNIT_AND_AMOUNT",
            CompensationFilterTypeEnum::ANNUALIZEDBASEAMOUNT => "ANNUALIZED_BASE_AMOUNT",
            CompensationFilterTypeEnum::ANNUALIZEDTOTALAMOUNT => "ANNUALIZED_TOTAL_AMOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for CompensationFilterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_TYPE_UNSPECIFIED" => Ok(CompensationFilterTypeEnum::FILTERTYPEUNSPECIFIED),
           "UNIT_ONLY" => Ok(CompensationFilterTypeEnum::UNITONLY),
           "UNIT_AND_AMOUNT" => Ok(CompensationFilterTypeEnum::UNITANDAMOUNT),
           "ANNUALIZED_BASE_AMOUNT" => Ok(CompensationFilterTypeEnum::ANNUALIZEDBASEAMOUNT),
           "ANNUALIZED_TOTAL_AMOUNT" => Ok(CompensationFilterTypeEnum::ANNUALIZEDTOTALAMOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompensationFilterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompensationFilterUnitsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specify desired `base compensation entry's` CompensationInfo.CompensationUnit.
pub enum CompensationFilterUnitsEnum {
    

    /// Default value.
    ///
    /// "COMPENSATION_UNIT_UNSPECIFIED"
    #[serde(rename="COMPENSATION_UNIT_UNSPECIFIED")]
    COMPENSATIONUNITUNSPECIFIED,
    

    /// Hourly.
    ///
    /// "HOURLY"
    #[serde(rename="HOURLY")]
    HOURLY,
    

    /// Daily.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
    

    /// Weekly
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// Monthly.
    ///
    /// "MONTHLY"
    #[serde(rename="MONTHLY")]
    MONTHLY,
    

    /// Yearly.
    ///
    /// "YEARLY"
    #[serde(rename="YEARLY")]
    YEARLY,
    

    /// One time.
    ///
    /// "ONE_TIME"
    #[serde(rename="ONE_TIME")]
    ONETIME,
    

    /// Other compensation units.
    ///
    /// "OTHER_COMPENSATION_UNIT"
    #[serde(rename="OTHER_COMPENSATION_UNIT")]
    OTHERCOMPENSATIONUNIT,
}

impl AsRef<str> for CompensationFilterUnitsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompensationFilterUnitsEnum::COMPENSATIONUNITUNSPECIFIED => "COMPENSATION_UNIT_UNSPECIFIED",
            CompensationFilterUnitsEnum::HOURLY => "HOURLY",
            CompensationFilterUnitsEnum::DAILY => "DAILY",
            CompensationFilterUnitsEnum::WEEKLY => "WEEKLY",
            CompensationFilterUnitsEnum::MONTHLY => "MONTHLY",
            CompensationFilterUnitsEnum::YEARLY => "YEARLY",
            CompensationFilterUnitsEnum::ONETIME => "ONE_TIME",
            CompensationFilterUnitsEnum::OTHERCOMPENSATIONUNIT => "OTHER_COMPENSATION_UNIT",
        }
    }
}

impl std::convert::TryFrom< &str> for CompensationFilterUnitsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPENSATION_UNIT_UNSPECIFIED" => Ok(CompensationFilterUnitsEnum::COMPENSATIONUNITUNSPECIFIED),
           "HOURLY" => Ok(CompensationFilterUnitsEnum::HOURLY),
           "DAILY" => Ok(CompensationFilterUnitsEnum::DAILY),
           "WEEKLY" => Ok(CompensationFilterUnitsEnum::WEEKLY),
           "MONTHLY" => Ok(CompensationFilterUnitsEnum::MONTHLY),
           "YEARLY" => Ok(CompensationFilterUnitsEnum::YEARLY),
           "ONE_TIME" => Ok(CompensationFilterUnitsEnum::ONETIME),
           "OTHER_COMPENSATION_UNIT" => Ok(CompensationFilterUnitsEnum::OTHERCOMPENSATIONUNIT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompensationFilterUnitsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompletionResultTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The completion topic.
pub enum CompletionResultTypeEnum {
    

    /// Default value.
    ///
    /// "COMPLETION_TYPE_UNSPECIFIED"
    #[serde(rename="COMPLETION_TYPE_UNSPECIFIED")]
    COMPLETIONTYPEUNSPECIFIED,
    

    /// Suggest job titles for jobs autocomplete. For CompletionType.JOB_TITLE type, only open jobs with the same language_codes are returned.
    ///
    /// "JOB_TITLE"
    #[serde(rename="JOB_TITLE")]
    JOBTITLE,
    

    /// Suggest company names for jobs autocomplete. For CompletionType.COMPANY_NAME type, only companies having open jobs with the same language_codes are returned.
    ///
    /// "COMPANY_NAME"
    #[serde(rename="COMPANY_NAME")]
    COMPANYNAME,
    

    /// Suggest both job titles and company names for jobs autocomplete. For CompletionType.COMBINED type, only open jobs with the same language_codes or companies having open jobs with the same language_codes are returned.
    ///
    /// "COMBINED"
    #[serde(rename="COMBINED")]
    COMBINED,
}

impl AsRef<str> for CompletionResultTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompletionResultTypeEnum::COMPLETIONTYPEUNSPECIFIED => "COMPLETION_TYPE_UNSPECIFIED",
            CompletionResultTypeEnum::JOBTITLE => "JOB_TITLE",
            CompletionResultTypeEnum::COMPANYNAME => "COMPANY_NAME",
            CompletionResultTypeEnum::COMBINED => "COMBINED",
        }
    }
}

impl std::convert::TryFrom< &str> for CompletionResultTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLETION_TYPE_UNSPECIFIED" => Ok(CompletionResultTypeEnum::COMPLETIONTYPEUNSPECIFIED),
           "JOB_TITLE" => Ok(CompletionResultTypeEnum::JOBTITLE),
           "COMPANY_NAME" => Ok(CompletionResultTypeEnum::COMPANYNAME),
           "COMBINED" => Ok(CompletionResultTypeEnum::COMBINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompletionResultTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomRankingInfoImportanceLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Controls over how important the score of CustomRankingInfo.ranking_expression gets applied to job's final ranking position. An error is thrown if not specified.
pub enum CustomRankingInfoImportanceLevelEnum {
    

    /// Default value if the importance level isn't specified.
    ///
    /// "IMPORTANCE_LEVEL_UNSPECIFIED"
    #[serde(rename="IMPORTANCE_LEVEL_UNSPECIFIED")]
    IMPORTANCELEVELUNSPECIFIED,
    

    /// The given ranking expression is of None importance, existing relevance score (determined by API algorithm) dominates job's final ranking position.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// The given ranking expression is of Low importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm).
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// The given ranking expression is of Mild importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm).
    ///
    /// "MILD"
    #[serde(rename="MILD")]
    MILD,
    

    /// The given ranking expression is of Medium importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm).
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// The given ranking expression is of High importance in terms of job's final ranking position compared to existing relevance score (determined by API algorithm).
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// The given ranking expression is of Extreme importance, and dominates job's final ranking position with existing relevance score (determined by API algorithm) ignored.
    ///
    /// "EXTREME"
    #[serde(rename="EXTREME")]
    EXTREME,
}

impl AsRef<str> for CustomRankingInfoImportanceLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomRankingInfoImportanceLevelEnum::IMPORTANCELEVELUNSPECIFIED => "IMPORTANCE_LEVEL_UNSPECIFIED",
            CustomRankingInfoImportanceLevelEnum::NONE => "NONE",
            CustomRankingInfoImportanceLevelEnum::LOW => "LOW",
            CustomRankingInfoImportanceLevelEnum::MILD => "MILD",
            CustomRankingInfoImportanceLevelEnum::MEDIUM => "MEDIUM",
            CustomRankingInfoImportanceLevelEnum::HIGH => "HIGH",
            CustomRankingInfoImportanceLevelEnum::EXTREME => "EXTREME",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomRankingInfoImportanceLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORTANCE_LEVEL_UNSPECIFIED" => Ok(CustomRankingInfoImportanceLevelEnum::IMPORTANCELEVELUNSPECIFIED),
           "NONE" => Ok(CustomRankingInfoImportanceLevelEnum::NONE),
           "LOW" => Ok(CustomRankingInfoImportanceLevelEnum::LOW),
           "MILD" => Ok(CustomRankingInfoImportanceLevelEnum::MILD),
           "MEDIUM" => Ok(CustomRankingInfoImportanceLevelEnum::MEDIUM),
           "HIGH" => Ok(CustomRankingInfoImportanceLevelEnum::HIGH),
           "EXTREME" => Ok(CustomRankingInfoImportanceLevelEnum::EXTREME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomRankingInfoImportanceLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceInfoDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the device.
pub enum DeviceInfoDeviceTypeEnum {
    

    /// The device type isn't specified.
    ///
    /// "DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="DEVICE_TYPE_UNSPECIFIED")]
    DEVICETYPEUNSPECIFIED,
    

    /// A desktop web browser, such as, Chrome, Firefox, Safari, or Internet Explorer)
    ///
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
    

    /// A mobile device web browser, such as a phone or tablet with a Chrome browser.
    ///
    /// "MOBILE_WEB"
    #[serde(rename="MOBILE_WEB")]
    MOBILEWEB,
    

    /// An Android device native application.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// An iOS device native application.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// A bot, as opposed to a device operated by human beings, such as a web crawler.
    ///
    /// "BOT"
    #[serde(rename="BOT")]
    BOT,
    

    /// Other devices types.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for DeviceInfoDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceInfoDeviceTypeEnum::DEVICETYPEUNSPECIFIED => "DEVICE_TYPE_UNSPECIFIED",
            DeviceInfoDeviceTypeEnum::WEB => "WEB",
            DeviceInfoDeviceTypeEnum::MOBILEWEB => "MOBILE_WEB",
            DeviceInfoDeviceTypeEnum::ANDROID => "ANDROID",
            DeviceInfoDeviceTypeEnum::IOS => "IOS",
            DeviceInfoDeviceTypeEnum::BOT => "BOT",
            DeviceInfoDeviceTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceInfoDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_TYPE_UNSPECIFIED" => Ok(DeviceInfoDeviceTypeEnum::DEVICETYPEUNSPECIFIED),
           "WEB" => Ok(DeviceInfoDeviceTypeEnum::WEB),
           "MOBILE_WEB" => Ok(DeviceInfoDeviceTypeEnum::MOBILEWEB),
           "ANDROID" => Ok(DeviceInfoDeviceTypeEnum::ANDROID),
           "IOS" => Ok(DeviceInfoDeviceTypeEnum::IOS),
           "BOT" => Ok(DeviceInfoDeviceTypeEnum::BOT),
           "OTHER" => Ok(DeviceInfoDeviceTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceInfoDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobDegreeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired education degrees for the job, such as Bachelors, Masters.
pub enum JobDegreeTypesEnum {
    

    /// Default value. Represents no degree, or early childhood education. Maps to ISCED code 0. Ex) Kindergarten
    ///
    /// "DEGREE_TYPE_UNSPECIFIED"
    #[serde(rename="DEGREE_TYPE_UNSPECIFIED")]
    DEGREETYPEUNSPECIFIED,
    

    /// Primary education which is typically the first stage of compulsory education. ISCED code 1. Ex) Elementary school
    ///
    /// "PRIMARY_EDUCATION"
    #[serde(rename="PRIMARY_EDUCATION")]
    PRIMARYEDUCATION,
    

    /// Lower secondary education; First stage of secondary education building on primary education, typically with a more subject-oriented curriculum. ISCED code 2. Ex) Middle school
    ///
    /// "LOWER_SECONDARY_EDUCATION"
    #[serde(rename="LOWER_SECONDARY_EDUCATION")]
    LOWERSECONDARYEDUCATION,
    

    /// Middle education; Second/final stage of secondary education preparing for tertiary education and/or providing skills relevant to employment. Usually with an increased range of subject options and streams. ISCED code 3. Ex) High school
    ///
    /// "UPPER_SECONDARY_EDUCATION"
    #[serde(rename="UPPER_SECONDARY_EDUCATION")]
    UPPERSECONDARYEDUCATION,
    

    /// Adult Remedial Education; Programmes providing learning experiences that build on secondary education and prepare for labour market entry and/or tertiary education. The content is broader than secondary but not as complex as tertiary education. ISCED code 4.
    ///
    /// "ADULT_REMEDIAL_EDUCATION"
    #[serde(rename="ADULT_REMEDIAL_EDUCATION")]
    ADULTREMEDIALEDUCATION,
    

    /// Associate's or equivalent; Short first tertiary programmes that are typically practically-based, occupationally-specific and prepare for labour market entry. These programmes may also provide a pathway to other tertiary programmes. ISCED code 5.
    ///
    /// "ASSOCIATES_OR_EQUIVALENT"
    #[serde(rename="ASSOCIATES_OR_EQUIVALENT")]
    ASSOCIATESOREQUIVALENT,
    

    /// Bachelor's or equivalent; Programmes designed to provide intermediate academic and/or professional knowledge, skills and competencies leading to a first tertiary degree or equivalent qualification. ISCED code 6.
    ///
    /// "BACHELORS_OR_EQUIVALENT"
    #[serde(rename="BACHELORS_OR_EQUIVALENT")]
    BACHELORSOREQUIVALENT,
    

    /// Master's or equivalent; Programmes designed to provide advanced academic and/or professional knowledge, skills and competencies leading to a second tertiary degree or equivalent qualification. ISCED code 7.
    ///
    /// "MASTERS_OR_EQUIVALENT"
    #[serde(rename="MASTERS_OR_EQUIVALENT")]
    MASTERSOREQUIVALENT,
    

    /// Doctoral or equivalent; Programmes designed primarily to lead to an advanced research qualification, usually concluding with the submission and defense of a substantive dissertation of publishable quality based on original research. ISCED code 8.
    ///
    /// "DOCTORAL_OR_EQUIVALENT"
    #[serde(rename="DOCTORAL_OR_EQUIVALENT")]
    DOCTORALOREQUIVALENT,
}

impl AsRef<str> for JobDegreeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobDegreeTypesEnum::DEGREETYPEUNSPECIFIED => "DEGREE_TYPE_UNSPECIFIED",
            JobDegreeTypesEnum::PRIMARYEDUCATION => "PRIMARY_EDUCATION",
            JobDegreeTypesEnum::LOWERSECONDARYEDUCATION => "LOWER_SECONDARY_EDUCATION",
            JobDegreeTypesEnum::UPPERSECONDARYEDUCATION => "UPPER_SECONDARY_EDUCATION",
            JobDegreeTypesEnum::ADULTREMEDIALEDUCATION => "ADULT_REMEDIAL_EDUCATION",
            JobDegreeTypesEnum::ASSOCIATESOREQUIVALENT => "ASSOCIATES_OR_EQUIVALENT",
            JobDegreeTypesEnum::BACHELORSOREQUIVALENT => "BACHELORS_OR_EQUIVALENT",
            JobDegreeTypesEnum::MASTERSOREQUIVALENT => "MASTERS_OR_EQUIVALENT",
            JobDegreeTypesEnum::DOCTORALOREQUIVALENT => "DOCTORAL_OR_EQUIVALENT",
        }
    }
}

impl std::convert::TryFrom< &str> for JobDegreeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEGREE_TYPE_UNSPECIFIED" => Ok(JobDegreeTypesEnum::DEGREETYPEUNSPECIFIED),
           "PRIMARY_EDUCATION" => Ok(JobDegreeTypesEnum::PRIMARYEDUCATION),
           "LOWER_SECONDARY_EDUCATION" => Ok(JobDegreeTypesEnum::LOWERSECONDARYEDUCATION),
           "UPPER_SECONDARY_EDUCATION" => Ok(JobDegreeTypesEnum::UPPERSECONDARYEDUCATION),
           "ADULT_REMEDIAL_EDUCATION" => Ok(JobDegreeTypesEnum::ADULTREMEDIALEDUCATION),
           "ASSOCIATES_OR_EQUIVALENT" => Ok(JobDegreeTypesEnum::ASSOCIATESOREQUIVALENT),
           "BACHELORS_OR_EQUIVALENT" => Ok(JobDegreeTypesEnum::BACHELORSOREQUIVALENT),
           "MASTERS_OR_EQUIVALENT" => Ok(JobDegreeTypesEnum::MASTERSOREQUIVALENT),
           "DOCTORAL_OR_EQUIVALENT" => Ok(JobDegreeTypesEnum::DOCTORALOREQUIVALENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobDegreeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobEmploymentTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The employment type(s) of a job, for example, full time or part time.
pub enum JobEmploymentTypesEnum {
    

    /// The default value if the employment type isn't specified.
    ///
    /// "EMPLOYMENT_TYPE_UNSPECIFIED"
    #[serde(rename="EMPLOYMENT_TYPE_UNSPECIFIED")]
    EMPLOYMENTTYPEUNSPECIFIED,
    

    /// The job requires working a number of hours that constitute full time employment, typically 40 or more hours per week.
    ///
    /// "FULL_TIME"
    #[serde(rename="FULL_TIME")]
    FULLTIME,
    

    /// The job entails working fewer hours than a full time job, typically less than 40 hours a week.
    ///
    /// "PART_TIME"
    #[serde(rename="PART_TIME")]
    PARTTIME,
    

    /// The job is offered as a contracted, as opposed to a salaried employee, position.
    ///
    /// "CONTRACTOR"
    #[serde(rename="CONTRACTOR")]
    CONTRACTOR,
    

    /// The job is offered as a contracted position with the understanding that it's converted into a full-time position at the end of the contract. Jobs of this type are also returned by a search for EmploymentType.CONTRACTOR jobs.
    ///
    /// "CONTRACT_TO_HIRE"
    #[serde(rename="CONTRACT_TO_HIRE")]
    CONTRACTTOHIRE,
    

    /// The job is offered as a temporary employment opportunity, usually a short-term engagement.
    ///
    /// "TEMPORARY"
    #[serde(rename="TEMPORARY")]
    TEMPORARY,
    

    /// The job is a fixed-term opportunity for students or entry-level job seekers to obtain on-the-job training, typically offered as a summer position.
    ///
    /// "INTERN"
    #[serde(rename="INTERN")]
    INTERN,
    

    /// The is an opportunity for an individual to volunteer, where there's no expectation of compensation for the provided services.
    ///
    /// "VOLUNTEER"
    #[serde(rename="VOLUNTEER")]
    VOLUNTEER,
    

    /// The job requires an employee to work on an as-needed basis with a flexible schedule.
    ///
    /// "PER_DIEM"
    #[serde(rename="PER_DIEM")]
    PERDIEM,
    

    /// The job involves employing people in remote areas and flying them temporarily to the work site instead of relocating employees and their families permanently.
    ///
    /// "FLY_IN_FLY_OUT"
    #[serde(rename="FLY_IN_FLY_OUT")]
    FLYINFLYOUT,
    

    /// The job does not fit any of the other listed types.
    ///
    /// "OTHER_EMPLOYMENT_TYPE"
    #[serde(rename="OTHER_EMPLOYMENT_TYPE")]
    OTHEREMPLOYMENTTYPE,
}

impl AsRef<str> for JobEmploymentTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobEmploymentTypesEnum::EMPLOYMENTTYPEUNSPECIFIED => "EMPLOYMENT_TYPE_UNSPECIFIED",
            JobEmploymentTypesEnum::FULLTIME => "FULL_TIME",
            JobEmploymentTypesEnum::PARTTIME => "PART_TIME",
            JobEmploymentTypesEnum::CONTRACTOR => "CONTRACTOR",
            JobEmploymentTypesEnum::CONTRACTTOHIRE => "CONTRACT_TO_HIRE",
            JobEmploymentTypesEnum::TEMPORARY => "TEMPORARY",
            JobEmploymentTypesEnum::INTERN => "INTERN",
            JobEmploymentTypesEnum::VOLUNTEER => "VOLUNTEER",
            JobEmploymentTypesEnum::PERDIEM => "PER_DIEM",
            JobEmploymentTypesEnum::FLYINFLYOUT => "FLY_IN_FLY_OUT",
            JobEmploymentTypesEnum::OTHEREMPLOYMENTTYPE => "OTHER_EMPLOYMENT_TYPE",
        }
    }
}

impl std::convert::TryFrom< &str> for JobEmploymentTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EMPLOYMENT_TYPE_UNSPECIFIED" => Ok(JobEmploymentTypesEnum::EMPLOYMENTTYPEUNSPECIFIED),
           "FULL_TIME" => Ok(JobEmploymentTypesEnum::FULLTIME),
           "PART_TIME" => Ok(JobEmploymentTypesEnum::PARTTIME),
           "CONTRACTOR" => Ok(JobEmploymentTypesEnum::CONTRACTOR),
           "CONTRACT_TO_HIRE" => Ok(JobEmploymentTypesEnum::CONTRACTTOHIRE),
           "TEMPORARY" => Ok(JobEmploymentTypesEnum::TEMPORARY),
           "INTERN" => Ok(JobEmploymentTypesEnum::INTERN),
           "VOLUNTEER" => Ok(JobEmploymentTypesEnum::VOLUNTEER),
           "PER_DIEM" => Ok(JobEmploymentTypesEnum::PERDIEM),
           "FLY_IN_FLY_OUT" => Ok(JobEmploymentTypesEnum::FLYINFLYOUT),
           "OTHER_EMPLOYMENT_TYPE" => Ok(JobEmploymentTypesEnum::OTHEREMPLOYMENTTYPE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobEmploymentTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobJobBenefitsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The benefits included with the job.
pub enum JobJobBenefitsEnum {
    

    /// Default value if the type isn't specified.
    ///
    /// "JOB_BENEFIT_UNSPECIFIED"
    #[serde(rename="JOB_BENEFIT_UNSPECIFIED")]
    JOBBENEFITUNSPECIFIED,
    

    /// The job includes access to programs that support child care, such as daycare.
    ///
    /// "CHILD_CARE"
    #[serde(rename="CHILD_CARE")]
    CHILDCARE,
    

    /// The job includes dental services covered by a dental insurance plan.
    ///
    /// "DENTAL"
    #[serde(rename="DENTAL")]
    DENTAL,
    

    /// The job offers specific benefits to domestic partners.
    ///
    /// "DOMESTIC_PARTNER"
    #[serde(rename="DOMESTIC_PARTNER")]
    DOMESTICPARTNER,
    

    /// The job allows for a flexible work schedule.
    ///
    /// "FLEXIBLE_HOURS"
    #[serde(rename="FLEXIBLE_HOURS")]
    FLEXIBLEHOURS,
    

    /// The job includes health services covered by a medical insurance plan.
    ///
    /// "MEDICAL"
    #[serde(rename="MEDICAL")]
    MEDICAL,
    

    /// The job includes a life insurance plan provided by the employer or available for purchase by the employee.
    ///
    /// "LIFE_INSURANCE"
    #[serde(rename="LIFE_INSURANCE")]
    LIFEINSURANCE,
    

    /// The job allows for a leave of absence to a parent to care for a newborn child.
    ///
    /// "PARENTAL_LEAVE"
    #[serde(rename="PARENTAL_LEAVE")]
    PARENTALLEAVE,
    

    /// The job includes a workplace retirement plan provided by the employer or available for purchase by the employee.
    ///
    /// "RETIREMENT_PLAN"
    #[serde(rename="RETIREMENT_PLAN")]
    RETIREMENTPLAN,
    

    /// The job allows for paid time off due to illness.
    ///
    /// "SICK_DAYS"
    #[serde(rename="SICK_DAYS")]
    SICKDAYS,
    

    /// The job includes paid time off for vacation.
    ///
    /// "VACATION"
    #[serde(rename="VACATION")]
    VACATION,
    

    /// The job includes vision services covered by a vision insurance plan.
    ///
    /// "VISION"
    #[serde(rename="VISION")]
    VISION,
}

impl AsRef<str> for JobJobBenefitsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobJobBenefitsEnum::JOBBENEFITUNSPECIFIED => "JOB_BENEFIT_UNSPECIFIED",
            JobJobBenefitsEnum::CHILDCARE => "CHILD_CARE",
            JobJobBenefitsEnum::DENTAL => "DENTAL",
            JobJobBenefitsEnum::DOMESTICPARTNER => "DOMESTIC_PARTNER",
            JobJobBenefitsEnum::FLEXIBLEHOURS => "FLEXIBLE_HOURS",
            JobJobBenefitsEnum::MEDICAL => "MEDICAL",
            JobJobBenefitsEnum::LIFEINSURANCE => "LIFE_INSURANCE",
            JobJobBenefitsEnum::PARENTALLEAVE => "PARENTAL_LEAVE",
            JobJobBenefitsEnum::RETIREMENTPLAN => "RETIREMENT_PLAN",
            JobJobBenefitsEnum::SICKDAYS => "SICK_DAYS",
            JobJobBenefitsEnum::VACATION => "VACATION",
            JobJobBenefitsEnum::VISION => "VISION",
        }
    }
}

impl std::convert::TryFrom< &str> for JobJobBenefitsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_BENEFIT_UNSPECIFIED" => Ok(JobJobBenefitsEnum::JOBBENEFITUNSPECIFIED),
           "CHILD_CARE" => Ok(JobJobBenefitsEnum::CHILDCARE),
           "DENTAL" => Ok(JobJobBenefitsEnum::DENTAL),
           "DOMESTIC_PARTNER" => Ok(JobJobBenefitsEnum::DOMESTICPARTNER),
           "FLEXIBLE_HOURS" => Ok(JobJobBenefitsEnum::FLEXIBLEHOURS),
           "MEDICAL" => Ok(JobJobBenefitsEnum::MEDICAL),
           "LIFE_INSURANCE" => Ok(JobJobBenefitsEnum::LIFEINSURANCE),
           "PARENTAL_LEAVE" => Ok(JobJobBenefitsEnum::PARENTALLEAVE),
           "RETIREMENT_PLAN" => Ok(JobJobBenefitsEnum::RETIREMENTPLAN),
           "SICK_DAYS" => Ok(JobJobBenefitsEnum::SICKDAYS),
           "VACATION" => Ok(JobJobBenefitsEnum::VACATION),
           "VISION" => Ok(JobJobBenefitsEnum::VISION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobJobBenefitsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobJobLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The experience level associated with the job, such as "Entry Level".
pub enum JobJobLevelEnum {
    

    /// The default value if the level isn't specified.
    ///
    /// "JOB_LEVEL_UNSPECIFIED"
    #[serde(rename="JOB_LEVEL_UNSPECIFIED")]
    JOBLEVELUNSPECIFIED,
    

    /// Entry-level individual contributors, typically with less than 2 years of experience in a similar role. Includes interns.
    ///
    /// "ENTRY_LEVEL"
    #[serde(rename="ENTRY_LEVEL")]
    ENTRYLEVEL,
    

    /// Experienced individual contributors, typically with 2+ years of experience in a similar role.
    ///
    /// "EXPERIENCED"
    #[serde(rename="EXPERIENCED")]
    EXPERIENCED,
    

    /// Entry- to mid-level managers responsible for managing a team of people.
    ///
    /// "MANAGER"
    #[serde(rename="MANAGER")]
    MANAGER,
    

    /// Senior-level managers responsible for managing teams of managers.
    ///
    /// "DIRECTOR"
    #[serde(rename="DIRECTOR")]
    DIRECTOR,
    

    /// Executive-level managers and above, including C-level positions.
    ///
    /// "EXECUTIVE"
    #[serde(rename="EXECUTIVE")]
    EXECUTIVE,
}

impl AsRef<str> for JobJobLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobJobLevelEnum::JOBLEVELUNSPECIFIED => "JOB_LEVEL_UNSPECIFIED",
            JobJobLevelEnum::ENTRYLEVEL => "ENTRY_LEVEL",
            JobJobLevelEnum::EXPERIENCED => "EXPERIENCED",
            JobJobLevelEnum::MANAGER => "MANAGER",
            JobJobLevelEnum::DIRECTOR => "DIRECTOR",
            JobJobLevelEnum::EXECUTIVE => "EXECUTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for JobJobLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_LEVEL_UNSPECIFIED" => Ok(JobJobLevelEnum::JOBLEVELUNSPECIFIED),
           "ENTRY_LEVEL" => Ok(JobJobLevelEnum::ENTRYLEVEL),
           "EXPERIENCED" => Ok(JobJobLevelEnum::EXPERIENCED),
           "MANAGER" => Ok(JobJobLevelEnum::MANAGER),
           "DIRECTOR" => Ok(JobJobLevelEnum::DIRECTOR),
           "EXECUTIVE" => Ok(JobJobLevelEnum::EXECUTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobJobLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobPostingRegionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The job PostingRegion (for example, state, country) throughout which the job is available. If this field is set, a LocationFilter in a search query within the job region finds this job posting if an exact location match isn't specified. If this field is set to PostingRegion.NATION or PostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses to the same location level as this field is strongly recommended.
pub enum JobPostingRegionEnum {
    

    /// If the region is unspecified, the job is only returned if it matches the LocationFilter.
    ///
    /// "POSTING_REGION_UNSPECIFIED"
    #[serde(rename="POSTING_REGION_UNSPECIFIED")]
    POSTINGREGIONUNSPECIFIED,
    

    /// In addition to exact location matching, job posting is returned when the LocationFilter in the search query is in the same administrative area as the returned job posting. For example, if a `ADMINISTRATIVE_AREA` job is posted in "CA, USA", it's returned if LocationFilter has "Mountain View". Administrative area refers to top-level administrative subdivision of this country. For example, US state, IT region, UK constituent nation and JP prefecture.
    ///
    /// "ADMINISTRATIVE_AREA"
    #[serde(rename="ADMINISTRATIVE_AREA")]
    ADMINISTRATIVEAREA,
    

    /// In addition to exact location matching, job is returned when LocationFilter in search query is in the same country as this job. For example, if a `NATION_WIDE` job is posted in "USA", it's returned if LocationFilter has 'Mountain View'.
    ///
    /// "NATION"
    #[serde(rename="NATION")]
    NATION,
    

    /// Job allows employees to work remotely (telecommute). If locations are provided with this value, the job is considered as having a location, but telecommuting is allowed.
    ///
    /// "TELECOMMUTE"
    #[serde(rename="TELECOMMUTE")]
    TELECOMMUTE,
}

impl AsRef<str> for JobPostingRegionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobPostingRegionEnum::POSTINGREGIONUNSPECIFIED => "POSTING_REGION_UNSPECIFIED",
            JobPostingRegionEnum::ADMINISTRATIVEAREA => "ADMINISTRATIVE_AREA",
            JobPostingRegionEnum::NATION => "NATION",
            JobPostingRegionEnum::TELECOMMUTE => "TELECOMMUTE",
        }
    }
}

impl std::convert::TryFrom< &str> for JobPostingRegionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POSTING_REGION_UNSPECIFIED" => Ok(JobPostingRegionEnum::POSTINGREGIONUNSPECIFIED),
           "ADMINISTRATIVE_AREA" => Ok(JobPostingRegionEnum::ADMINISTRATIVEAREA),
           "NATION" => Ok(JobPostingRegionEnum::NATION),
           "TELECOMMUTE" => Ok(JobPostingRegionEnum::TELECOMMUTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobPostingRegionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. The job is only visible to the owner. The visibility of the job. Defaults to Visibility.ACCOUNT_ONLY if not specified.
pub enum JobVisibilityEnum {
    

    /// Default value.
    ///
    /// "VISIBILITY_UNSPECIFIED"
    #[serde(rename="VISIBILITY_UNSPECIFIED")]
    VISIBILITYUNSPECIFIED,
    

    /// The resource is only visible to the GCP account who owns it.
    ///
    /// "ACCOUNT_ONLY"
    #[serde(rename="ACCOUNT_ONLY")]
    ACCOUNTONLY,
    

    /// The resource is visible to the owner and may be visible to other applications and processes at Google.
    ///
    /// "SHARED_WITH_GOOGLE"
    #[serde(rename="SHARED_WITH_GOOGLE")]
    SHAREDWITHGOOGLE,
    

    /// The resource is visible to the owner and may be visible to all other API clients.
    ///
    /// "SHARED_WITH_PUBLIC"
    #[serde(rename="SHARED_WITH_PUBLIC")]
    SHAREDWITHPUBLIC,
}

impl AsRef<str> for JobVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobVisibilityEnum::VISIBILITYUNSPECIFIED => "VISIBILITY_UNSPECIFIED",
            JobVisibilityEnum::ACCOUNTONLY => "ACCOUNT_ONLY",
            JobVisibilityEnum::SHAREDWITHGOOGLE => "SHARED_WITH_GOOGLE",
            JobVisibilityEnum::SHAREDWITHPUBLIC => "SHARED_WITH_PUBLIC",
        }
    }
}

impl std::convert::TryFrom< &str> for JobVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VISIBILITY_UNSPECIFIED" => Ok(JobVisibilityEnum::VISIBILITYUNSPECIFIED),
           "ACCOUNT_ONLY" => Ok(JobVisibilityEnum::ACCOUNTONLY),
           "SHARED_WITH_GOOGLE" => Ok(JobVisibilityEnum::SHAREDWITHGOOGLE),
           "SHARED_WITH_PUBLIC" => Ok(JobVisibilityEnum::SHAREDWITHPUBLIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobDerivedInfoJobCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Job categories derived from Job.title and Job.description.
pub enum JobDerivedInfoJobCategoriesEnum {
    

    /// The default value if the category isn't specified.
    ///
    /// "JOB_CATEGORY_UNSPECIFIED"
    #[serde(rename="JOB_CATEGORY_UNSPECIFIED")]
    JOBCATEGORYUNSPECIFIED,
    

    /// An accounting and finance job, such as an Accountant.
    ///
    /// "ACCOUNTING_AND_FINANCE"
    #[serde(rename="ACCOUNTING_AND_FINANCE")]
    ACCOUNTINGANDFINANCE,
    

    /// An administrative and office job, such as an Administrative Assistant.
    ///
    /// "ADMINISTRATIVE_AND_OFFICE"
    #[serde(rename="ADMINISTRATIVE_AND_OFFICE")]
    ADMINISTRATIVEANDOFFICE,
    

    /// An advertising and marketing job, such as Marketing Manager.
    ///
    /// "ADVERTISING_AND_MARKETING"
    #[serde(rename="ADVERTISING_AND_MARKETING")]
    ADVERTISINGANDMARKETING,
    

    /// An animal care job, such as Veterinarian.
    ///
    /// "ANIMAL_CARE"
    #[serde(rename="ANIMAL_CARE")]
    ANIMALCARE,
    

    /// An art, fashion, or design job, such as Designer.
    ///
    /// "ART_FASHION_AND_DESIGN"
    #[serde(rename="ART_FASHION_AND_DESIGN")]
    ARTFASHIONANDDESIGN,
    

    /// A business operations job, such as Business Operations Manager.
    ///
    /// "BUSINESS_OPERATIONS"
    #[serde(rename="BUSINESS_OPERATIONS")]
    BUSINESSOPERATIONS,
    

    /// A cleaning and facilities job, such as Custodial Staff.
    ///
    /// "CLEANING_AND_FACILITIES"
    #[serde(rename="CLEANING_AND_FACILITIES")]
    CLEANINGANDFACILITIES,
    

    /// A computer and IT job, such as Systems Administrator.
    ///
    /// "COMPUTER_AND_IT"
    #[serde(rename="COMPUTER_AND_IT")]
    COMPUTERANDIT,
    

    /// A construction job, such as General Laborer.
    ///
    /// "CONSTRUCTION"
    #[serde(rename="CONSTRUCTION")]
    CONSTRUCTION,
    

    /// A customer service job, such s Cashier.
    ///
    /// "CUSTOMER_SERVICE"
    #[serde(rename="CUSTOMER_SERVICE")]
    CUSTOMERSERVICE,
    

    /// An education job, such as School Teacher.
    ///
    /// "EDUCATION"
    #[serde(rename="EDUCATION")]
    EDUCATION,
    

    /// An entertainment and travel job, such as Flight Attendant.
    ///
    /// "ENTERTAINMENT_AND_TRAVEL"
    #[serde(rename="ENTERTAINMENT_AND_TRAVEL")]
    ENTERTAINMENTANDTRAVEL,
    

    /// A farming or outdoor job, such as Park Ranger.
    ///
    /// "FARMING_AND_OUTDOORS"
    #[serde(rename="FARMING_AND_OUTDOORS")]
    FARMINGANDOUTDOORS,
    

    /// A healthcare job, such as Registered Nurse.
    ///
    /// "HEALTHCARE"
    #[serde(rename="HEALTHCARE")]
    HEALTHCARE,
    

    /// A human resources job, such as Human Resources Director.
    ///
    /// "HUMAN_RESOURCES"
    #[serde(rename="HUMAN_RESOURCES")]
    HUMANRESOURCES,
    

    /// An installation, maintenance, or repair job, such as Electrician.
    ///
    /// "INSTALLATION_MAINTENANCE_AND_REPAIR"
    #[serde(rename="INSTALLATION_MAINTENANCE_AND_REPAIR")]
    INSTALLATIONMAINTENANCEANDREPAIR,
    

    /// A legal job, such as Law Clerk.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// A management job, often used in conjunction with another category, such as Store Manager.
    ///
    /// "MANAGEMENT"
    #[serde(rename="MANAGEMENT")]
    MANAGEMENT,
    

    /// A manufacturing or warehouse job, such as Assembly Technician.
    ///
    /// "MANUFACTURING_AND_WAREHOUSE"
    #[serde(rename="MANUFACTURING_AND_WAREHOUSE")]
    MANUFACTURINGANDWAREHOUSE,
    

    /// A media, communications, or writing job, such as Media Relations.
    ///
    /// "MEDIA_COMMUNICATIONS_AND_WRITING"
    #[serde(rename="MEDIA_COMMUNICATIONS_AND_WRITING")]
    MEDIACOMMUNICATIONSANDWRITING,
    

    /// An oil, gas or mining job, such as Offshore Driller.
    ///
    /// "OIL_GAS_AND_MINING"
    #[serde(rename="OIL_GAS_AND_MINING")]
    OILGASANDMINING,
    

    /// A personal care and services job, such as Hair Stylist.
    ///
    /// "PERSONAL_CARE_AND_SERVICES"
    #[serde(rename="PERSONAL_CARE_AND_SERVICES")]
    PERSONALCAREANDSERVICES,
    

    /// A protective services job, such as Security Guard.
    ///
    /// "PROTECTIVE_SERVICES"
    #[serde(rename="PROTECTIVE_SERVICES")]
    PROTECTIVESERVICES,
    

    /// A real estate job, such as Buyer's Agent.
    ///
    /// "REAL_ESTATE"
    #[serde(rename="REAL_ESTATE")]
    REALESTATE,
    

    /// A restaurant and hospitality job, such as Restaurant Server.
    ///
    /// "RESTAURANT_AND_HOSPITALITY"
    #[serde(rename="RESTAURANT_AND_HOSPITALITY")]
    RESTAURANTANDHOSPITALITY,
    

    /// A sales and/or retail job, such Sales Associate.
    ///
    /// "SALES_AND_RETAIL"
    #[serde(rename="SALES_AND_RETAIL")]
    SALESANDRETAIL,
    

    /// A science and engineering job, such as Lab Technician.
    ///
    /// "SCIENCE_AND_ENGINEERING"
    #[serde(rename="SCIENCE_AND_ENGINEERING")]
    SCIENCEANDENGINEERING,
    

    /// A social services or non-profit job, such as Case Worker.
    ///
    /// "SOCIAL_SERVICES_AND_NON_PROFIT"
    #[serde(rename="SOCIAL_SERVICES_AND_NON_PROFIT")]
    SOCIALSERVICESANDNONPROFIT,
    

    /// A sports, fitness, or recreation job, such as Personal Trainer.
    ///
    /// "SPORTS_FITNESS_AND_RECREATION"
    #[serde(rename="SPORTS_FITNESS_AND_RECREATION")]
    SPORTSFITNESSANDRECREATION,
    

    /// A transportation or logistics job, such as Truck Driver.
    ///
    /// "TRANSPORTATION_AND_LOGISTICS"
    #[serde(rename="TRANSPORTATION_AND_LOGISTICS")]
    TRANSPORTATIONANDLOGISTICS,
}

impl AsRef<str> for JobDerivedInfoJobCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobDerivedInfoJobCategoriesEnum::JOBCATEGORYUNSPECIFIED => "JOB_CATEGORY_UNSPECIFIED",
            JobDerivedInfoJobCategoriesEnum::ACCOUNTINGANDFINANCE => "ACCOUNTING_AND_FINANCE",
            JobDerivedInfoJobCategoriesEnum::ADMINISTRATIVEANDOFFICE => "ADMINISTRATIVE_AND_OFFICE",
            JobDerivedInfoJobCategoriesEnum::ADVERTISINGANDMARKETING => "ADVERTISING_AND_MARKETING",
            JobDerivedInfoJobCategoriesEnum::ANIMALCARE => "ANIMAL_CARE",
            JobDerivedInfoJobCategoriesEnum::ARTFASHIONANDDESIGN => "ART_FASHION_AND_DESIGN",
            JobDerivedInfoJobCategoriesEnum::BUSINESSOPERATIONS => "BUSINESS_OPERATIONS",
            JobDerivedInfoJobCategoriesEnum::CLEANINGANDFACILITIES => "CLEANING_AND_FACILITIES",
            JobDerivedInfoJobCategoriesEnum::COMPUTERANDIT => "COMPUTER_AND_IT",
            JobDerivedInfoJobCategoriesEnum::CONSTRUCTION => "CONSTRUCTION",
            JobDerivedInfoJobCategoriesEnum::CUSTOMERSERVICE => "CUSTOMER_SERVICE",
            JobDerivedInfoJobCategoriesEnum::EDUCATION => "EDUCATION",
            JobDerivedInfoJobCategoriesEnum::ENTERTAINMENTANDTRAVEL => "ENTERTAINMENT_AND_TRAVEL",
            JobDerivedInfoJobCategoriesEnum::FARMINGANDOUTDOORS => "FARMING_AND_OUTDOORS",
            JobDerivedInfoJobCategoriesEnum::HEALTHCARE => "HEALTHCARE",
            JobDerivedInfoJobCategoriesEnum::HUMANRESOURCES => "HUMAN_RESOURCES",
            JobDerivedInfoJobCategoriesEnum::INSTALLATIONMAINTENANCEANDREPAIR => "INSTALLATION_MAINTENANCE_AND_REPAIR",
            JobDerivedInfoJobCategoriesEnum::LEGAL => "LEGAL",
            JobDerivedInfoJobCategoriesEnum::MANAGEMENT => "MANAGEMENT",
            JobDerivedInfoJobCategoriesEnum::MANUFACTURINGANDWAREHOUSE => "MANUFACTURING_AND_WAREHOUSE",
            JobDerivedInfoJobCategoriesEnum::MEDIACOMMUNICATIONSANDWRITING => "MEDIA_COMMUNICATIONS_AND_WRITING",
            JobDerivedInfoJobCategoriesEnum::OILGASANDMINING => "OIL_GAS_AND_MINING",
            JobDerivedInfoJobCategoriesEnum::PERSONALCAREANDSERVICES => "PERSONAL_CARE_AND_SERVICES",
            JobDerivedInfoJobCategoriesEnum::PROTECTIVESERVICES => "PROTECTIVE_SERVICES",
            JobDerivedInfoJobCategoriesEnum::REALESTATE => "REAL_ESTATE",
            JobDerivedInfoJobCategoriesEnum::RESTAURANTANDHOSPITALITY => "RESTAURANT_AND_HOSPITALITY",
            JobDerivedInfoJobCategoriesEnum::SALESANDRETAIL => "SALES_AND_RETAIL",
            JobDerivedInfoJobCategoriesEnum::SCIENCEANDENGINEERING => "SCIENCE_AND_ENGINEERING",
            JobDerivedInfoJobCategoriesEnum::SOCIALSERVICESANDNONPROFIT => "SOCIAL_SERVICES_AND_NON_PROFIT",
            JobDerivedInfoJobCategoriesEnum::SPORTSFITNESSANDRECREATION => "SPORTS_FITNESS_AND_RECREATION",
            JobDerivedInfoJobCategoriesEnum::TRANSPORTATIONANDLOGISTICS => "TRANSPORTATION_AND_LOGISTICS",
        }
    }
}

impl std::convert::TryFrom< &str> for JobDerivedInfoJobCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_CATEGORY_UNSPECIFIED" => Ok(JobDerivedInfoJobCategoriesEnum::JOBCATEGORYUNSPECIFIED),
           "ACCOUNTING_AND_FINANCE" => Ok(JobDerivedInfoJobCategoriesEnum::ACCOUNTINGANDFINANCE),
           "ADMINISTRATIVE_AND_OFFICE" => Ok(JobDerivedInfoJobCategoriesEnum::ADMINISTRATIVEANDOFFICE),
           "ADVERTISING_AND_MARKETING" => Ok(JobDerivedInfoJobCategoriesEnum::ADVERTISINGANDMARKETING),
           "ANIMAL_CARE" => Ok(JobDerivedInfoJobCategoriesEnum::ANIMALCARE),
           "ART_FASHION_AND_DESIGN" => Ok(JobDerivedInfoJobCategoriesEnum::ARTFASHIONANDDESIGN),
           "BUSINESS_OPERATIONS" => Ok(JobDerivedInfoJobCategoriesEnum::BUSINESSOPERATIONS),
           "CLEANING_AND_FACILITIES" => Ok(JobDerivedInfoJobCategoriesEnum::CLEANINGANDFACILITIES),
           "COMPUTER_AND_IT" => Ok(JobDerivedInfoJobCategoriesEnum::COMPUTERANDIT),
           "CONSTRUCTION" => Ok(JobDerivedInfoJobCategoriesEnum::CONSTRUCTION),
           "CUSTOMER_SERVICE" => Ok(JobDerivedInfoJobCategoriesEnum::CUSTOMERSERVICE),
           "EDUCATION" => Ok(JobDerivedInfoJobCategoriesEnum::EDUCATION),
           "ENTERTAINMENT_AND_TRAVEL" => Ok(JobDerivedInfoJobCategoriesEnum::ENTERTAINMENTANDTRAVEL),
           "FARMING_AND_OUTDOORS" => Ok(JobDerivedInfoJobCategoriesEnum::FARMINGANDOUTDOORS),
           "HEALTHCARE" => Ok(JobDerivedInfoJobCategoriesEnum::HEALTHCARE),
           "HUMAN_RESOURCES" => Ok(JobDerivedInfoJobCategoriesEnum::HUMANRESOURCES),
           "INSTALLATION_MAINTENANCE_AND_REPAIR" => Ok(JobDerivedInfoJobCategoriesEnum::INSTALLATIONMAINTENANCEANDREPAIR),
           "LEGAL" => Ok(JobDerivedInfoJobCategoriesEnum::LEGAL),
           "MANAGEMENT" => Ok(JobDerivedInfoJobCategoriesEnum::MANAGEMENT),
           "MANUFACTURING_AND_WAREHOUSE" => Ok(JobDerivedInfoJobCategoriesEnum::MANUFACTURINGANDWAREHOUSE),
           "MEDIA_COMMUNICATIONS_AND_WRITING" => Ok(JobDerivedInfoJobCategoriesEnum::MEDIACOMMUNICATIONSANDWRITING),
           "OIL_GAS_AND_MINING" => Ok(JobDerivedInfoJobCategoriesEnum::OILGASANDMINING),
           "PERSONAL_CARE_AND_SERVICES" => Ok(JobDerivedInfoJobCategoriesEnum::PERSONALCAREANDSERVICES),
           "PROTECTIVE_SERVICES" => Ok(JobDerivedInfoJobCategoriesEnum::PROTECTIVESERVICES),
           "REAL_ESTATE" => Ok(JobDerivedInfoJobCategoriesEnum::REALESTATE),
           "RESTAURANT_AND_HOSPITALITY" => Ok(JobDerivedInfoJobCategoriesEnum::RESTAURANTANDHOSPITALITY),
           "SALES_AND_RETAIL" => Ok(JobDerivedInfoJobCategoriesEnum::SALESANDRETAIL),
           "SCIENCE_AND_ENGINEERING" => Ok(JobDerivedInfoJobCategoriesEnum::SCIENCEANDENGINEERING),
           "SOCIAL_SERVICES_AND_NON_PROFIT" => Ok(JobDerivedInfoJobCategoriesEnum::SOCIALSERVICESANDNONPROFIT),
           "SPORTS_FITNESS_AND_RECREATION" => Ok(JobDerivedInfoJobCategoriesEnum::SPORTSFITNESSANDRECREATION),
           "TRANSPORTATION_AND_LOGISTICS" => Ok(JobDerivedInfoJobCategoriesEnum::TRANSPORTATIONANDLOGISTICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobDerivedInfoJobCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the event (see JobEventType).
pub enum JobEventTypeEnum {
    

    /// The event is unspecified by other provided values.
    ///
    /// "JOB_EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="JOB_EVENT_TYPE_UNSPECIFIED")]
    JOBEVENTTYPEUNSPECIFIED,
    

    /// The job seeker or other entity interacting with the service has had a job rendered in their view, such as in a list of search results in a compressed or clipped format. This event is typically associated with the viewing of a jobs list on a single page by a job seeker.
    ///
    /// "IMPRESSION"
    #[serde(rename="IMPRESSION")]
    IMPRESSION,
    

    /// The job seeker, or other entity interacting with the service, has viewed the details of a job, including the full description. This event doesn't apply to the viewing a snippet of a job appearing as a part of the job search results. Viewing a snippet is associated with an impression).
    ///
    /// "VIEW"
    #[serde(rename="VIEW")]
    VIEW,
    

    /// The job seeker or other entity interacting with the service performed an action to view a job and was redirected to a different website for job.
    ///
    /// "VIEW_REDIRECT"
    #[serde(rename="VIEW_REDIRECT")]
    VIEWREDIRECT,
    

    /// The job seeker or other entity interacting with the service began the process or demonstrated the intention of applying for a job.
    ///
    /// "APPLICATION_START"
    #[serde(rename="APPLICATION_START")]
    APPLICATIONSTART,
    

    /// The job seeker or other entity interacting with the service submitted an application for a job.
    ///
    /// "APPLICATION_FINISH"
    #[serde(rename="APPLICATION_FINISH")]
    APPLICATIONFINISH,
    

    /// The job seeker or other entity interacting with the service submitted an application for a job with a single click without entering information. If a job seeker performs this action, send only this event to the service. Do not also send JobEventType.APPLICATION_START or JobEventType.APPLICATION_FINISH events.
    ///
    /// "APPLICATION_QUICK_SUBMISSION"
    #[serde(rename="APPLICATION_QUICK_SUBMISSION")]
    APPLICATIONQUICKSUBMISSION,
    

    /// The job seeker or other entity interacting with the service performed an action to apply to a job and was redirected to a different website to complete the application.
    ///
    /// "APPLICATION_REDIRECT"
    #[serde(rename="APPLICATION_REDIRECT")]
    APPLICATIONREDIRECT,
    

    /// The job seeker or other entity interacting with the service began the process or demonstrated the intention of applying for a job from the search results page without viewing the details of the job posting. If sending this event, JobEventType.VIEW event shouldn't be sent.
    ///
    /// "APPLICATION_START_FROM_SEARCH"
    #[serde(rename="APPLICATION_START_FROM_SEARCH")]
    APPLICATIONSTARTFROMSEARCH,
    

    /// The job seeker, or other entity interacting with the service, performs an action with a single click from the search results page to apply to a job (without viewing the details of the job posting), and is redirected to a different website to complete the application. If a candidate performs this action, send only this event to the service. Do not also send JobEventType.APPLICATION_START, JobEventType.APPLICATION_FINISH or JobEventType.VIEW events.
    ///
    /// "APPLICATION_REDIRECT_FROM_SEARCH"
    #[serde(rename="APPLICATION_REDIRECT_FROM_SEARCH")]
    APPLICATIONREDIRECTFROMSEARCH,
    

    /// This event should be used when a company submits an application on behalf of a job seeker. This event is intended for use by staffing agencies attempting to place candidates.
    ///
    /// "APPLICATION_COMPANY_SUBMIT"
    #[serde(rename="APPLICATION_COMPANY_SUBMIT")]
    APPLICATIONCOMPANYSUBMIT,
    

    /// The job seeker or other entity interacting with the service demonstrated an interest in a job by bookmarking or saving it.
    ///
    /// "BOOKMARK"
    #[serde(rename="BOOKMARK")]
    BOOKMARK,
    

    /// The job seeker or other entity interacting with the service was sent a notification, such as an email alert or device notification, containing one or more jobs listings generated by the service.
    ///
    /// "NOTIFICATION"
    #[serde(rename="NOTIFICATION")]
    NOTIFICATION,
    

    /// The job seeker or other entity interacting with the service was employed by the hiring entity (employer). Send this event only if the job seeker was hired through an application that was initiated by a search conducted through the Cloud Talent Solution service.
    ///
    /// "HIRED"
    #[serde(rename="HIRED")]
    HIRED,
    

    /// A recruiter or staffing agency submitted an application on behalf of the candidate after interacting with the service to identify a suitable job posting.
    ///
    /// "SENT_CV"
    #[serde(rename="SENT_CV")]
    SENTCV,
    

    /// The entity interacting with the service (for example, the job seeker), was granted an initial interview by the hiring entity (employer). This event should only be sent if the job seeker was granted an interview as part of an application that was initiated by a search conducted through / recommendation provided by the Cloud Talent Solution service.
    ///
    /// "INTERVIEW_GRANTED"
    #[serde(rename="INTERVIEW_GRANTED")]
    INTERVIEWGRANTED,
}

impl AsRef<str> for JobEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobEventTypeEnum::JOBEVENTTYPEUNSPECIFIED => "JOB_EVENT_TYPE_UNSPECIFIED",
            JobEventTypeEnum::IMPRESSION => "IMPRESSION",
            JobEventTypeEnum::VIEW => "VIEW",
            JobEventTypeEnum::VIEWREDIRECT => "VIEW_REDIRECT",
            JobEventTypeEnum::APPLICATIONSTART => "APPLICATION_START",
            JobEventTypeEnum::APPLICATIONFINISH => "APPLICATION_FINISH",
            JobEventTypeEnum::APPLICATIONQUICKSUBMISSION => "APPLICATION_QUICK_SUBMISSION",
            JobEventTypeEnum::APPLICATIONREDIRECT => "APPLICATION_REDIRECT",
            JobEventTypeEnum::APPLICATIONSTARTFROMSEARCH => "APPLICATION_START_FROM_SEARCH",
            JobEventTypeEnum::APPLICATIONREDIRECTFROMSEARCH => "APPLICATION_REDIRECT_FROM_SEARCH",
            JobEventTypeEnum::APPLICATIONCOMPANYSUBMIT => "APPLICATION_COMPANY_SUBMIT",
            JobEventTypeEnum::BOOKMARK => "BOOKMARK",
            JobEventTypeEnum::NOTIFICATION => "NOTIFICATION",
            JobEventTypeEnum::HIRED => "HIRED",
            JobEventTypeEnum::SENTCV => "SENT_CV",
            JobEventTypeEnum::INTERVIEWGRANTED => "INTERVIEW_GRANTED",
        }
    }
}

impl std::convert::TryFrom< &str> for JobEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_EVENT_TYPE_UNSPECIFIED" => Ok(JobEventTypeEnum::JOBEVENTTYPEUNSPECIFIED),
           "IMPRESSION" => Ok(JobEventTypeEnum::IMPRESSION),
           "VIEW" => Ok(JobEventTypeEnum::VIEW),
           "VIEW_REDIRECT" => Ok(JobEventTypeEnum::VIEWREDIRECT),
           "APPLICATION_START" => Ok(JobEventTypeEnum::APPLICATIONSTART),
           "APPLICATION_FINISH" => Ok(JobEventTypeEnum::APPLICATIONFINISH),
           "APPLICATION_QUICK_SUBMISSION" => Ok(JobEventTypeEnum::APPLICATIONQUICKSUBMISSION),
           "APPLICATION_REDIRECT" => Ok(JobEventTypeEnum::APPLICATIONREDIRECT),
           "APPLICATION_START_FROM_SEARCH" => Ok(JobEventTypeEnum::APPLICATIONSTARTFROMSEARCH),
           "APPLICATION_REDIRECT_FROM_SEARCH" => Ok(JobEventTypeEnum::APPLICATIONREDIRECTFROMSEARCH),
           "APPLICATION_COMPANY_SUBMIT" => Ok(JobEventTypeEnum::APPLICATIONCOMPANYSUBMIT),
           "BOOKMARK" => Ok(JobEventTypeEnum::BOOKMARK),
           "NOTIFICATION" => Ok(JobEventTypeEnum::NOTIFICATION),
           "HIRED" => Ok(JobEventTypeEnum::HIRED),
           "SENT_CV" => Ok(JobEventTypeEnum::SENTCV),
           "INTERVIEW_GRANTED" => Ok(JobEventTypeEnum::INTERVIEWGRANTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobQueryEmploymentTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The employment type filter specifies the employment type of jobs to search against, such as EmploymentType.FULL_TIME. If a value isn't specified, jobs in the search results includes any employment type. If multiple values are specified, jobs in the search results include any of the specified employment types.
pub enum JobQueryEmploymentTypesEnum {
    

    /// The default value if the employment type isn't specified.
    ///
    /// "EMPLOYMENT_TYPE_UNSPECIFIED"
    #[serde(rename="EMPLOYMENT_TYPE_UNSPECIFIED")]
    EMPLOYMENTTYPEUNSPECIFIED,
    

    /// The job requires working a number of hours that constitute full time employment, typically 40 or more hours per week.
    ///
    /// "FULL_TIME"
    #[serde(rename="FULL_TIME")]
    FULLTIME,
    

    /// The job entails working fewer hours than a full time job, typically less than 40 hours a week.
    ///
    /// "PART_TIME"
    #[serde(rename="PART_TIME")]
    PARTTIME,
    

    /// The job is offered as a contracted, as opposed to a salaried employee, position.
    ///
    /// "CONTRACTOR"
    #[serde(rename="CONTRACTOR")]
    CONTRACTOR,
    

    /// The job is offered as a contracted position with the understanding that it's converted into a full-time position at the end of the contract. Jobs of this type are also returned by a search for EmploymentType.CONTRACTOR jobs.
    ///
    /// "CONTRACT_TO_HIRE"
    #[serde(rename="CONTRACT_TO_HIRE")]
    CONTRACTTOHIRE,
    

    /// The job is offered as a temporary employment opportunity, usually a short-term engagement.
    ///
    /// "TEMPORARY"
    #[serde(rename="TEMPORARY")]
    TEMPORARY,
    

    /// The job is a fixed-term opportunity for students or entry-level job seekers to obtain on-the-job training, typically offered as a summer position.
    ///
    /// "INTERN"
    #[serde(rename="INTERN")]
    INTERN,
    

    /// The is an opportunity for an individual to volunteer, where there's no expectation of compensation for the provided services.
    ///
    /// "VOLUNTEER"
    #[serde(rename="VOLUNTEER")]
    VOLUNTEER,
    

    /// The job requires an employee to work on an as-needed basis with a flexible schedule.
    ///
    /// "PER_DIEM"
    #[serde(rename="PER_DIEM")]
    PERDIEM,
    

    /// The job involves employing people in remote areas and flying them temporarily to the work site instead of relocating employees and their families permanently.
    ///
    /// "FLY_IN_FLY_OUT"
    #[serde(rename="FLY_IN_FLY_OUT")]
    FLYINFLYOUT,
    

    /// The job does not fit any of the other listed types.
    ///
    /// "OTHER_EMPLOYMENT_TYPE"
    #[serde(rename="OTHER_EMPLOYMENT_TYPE")]
    OTHEREMPLOYMENTTYPE,
}

impl AsRef<str> for JobQueryEmploymentTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobQueryEmploymentTypesEnum::EMPLOYMENTTYPEUNSPECIFIED => "EMPLOYMENT_TYPE_UNSPECIFIED",
            JobQueryEmploymentTypesEnum::FULLTIME => "FULL_TIME",
            JobQueryEmploymentTypesEnum::PARTTIME => "PART_TIME",
            JobQueryEmploymentTypesEnum::CONTRACTOR => "CONTRACTOR",
            JobQueryEmploymentTypesEnum::CONTRACTTOHIRE => "CONTRACT_TO_HIRE",
            JobQueryEmploymentTypesEnum::TEMPORARY => "TEMPORARY",
            JobQueryEmploymentTypesEnum::INTERN => "INTERN",
            JobQueryEmploymentTypesEnum::VOLUNTEER => "VOLUNTEER",
            JobQueryEmploymentTypesEnum::PERDIEM => "PER_DIEM",
            JobQueryEmploymentTypesEnum::FLYINFLYOUT => "FLY_IN_FLY_OUT",
            JobQueryEmploymentTypesEnum::OTHEREMPLOYMENTTYPE => "OTHER_EMPLOYMENT_TYPE",
        }
    }
}

impl std::convert::TryFrom< &str> for JobQueryEmploymentTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EMPLOYMENT_TYPE_UNSPECIFIED" => Ok(JobQueryEmploymentTypesEnum::EMPLOYMENTTYPEUNSPECIFIED),
           "FULL_TIME" => Ok(JobQueryEmploymentTypesEnum::FULLTIME),
           "PART_TIME" => Ok(JobQueryEmploymentTypesEnum::PARTTIME),
           "CONTRACTOR" => Ok(JobQueryEmploymentTypesEnum::CONTRACTOR),
           "CONTRACT_TO_HIRE" => Ok(JobQueryEmploymentTypesEnum::CONTRACTTOHIRE),
           "TEMPORARY" => Ok(JobQueryEmploymentTypesEnum::TEMPORARY),
           "INTERN" => Ok(JobQueryEmploymentTypesEnum::INTERN),
           "VOLUNTEER" => Ok(JobQueryEmploymentTypesEnum::VOLUNTEER),
           "PER_DIEM" => Ok(JobQueryEmploymentTypesEnum::PERDIEM),
           "FLY_IN_FLY_OUT" => Ok(JobQueryEmploymentTypesEnum::FLYINFLYOUT),
           "OTHER_EMPLOYMENT_TYPE" => Ok(JobQueryEmploymentTypesEnum::OTHEREMPLOYMENTTYPE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobQueryEmploymentTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobQueryJobCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The category filter specifies the categories of jobs to search against. See JobCategory for more information. If a value isn't specified, jobs from any category are searched against. If multiple values are specified, jobs from any of the specified categories are searched against.
pub enum JobQueryJobCategoriesEnum {
    

    /// The default value if the category isn't specified.
    ///
    /// "JOB_CATEGORY_UNSPECIFIED"
    #[serde(rename="JOB_CATEGORY_UNSPECIFIED")]
    JOBCATEGORYUNSPECIFIED,
    

    /// An accounting and finance job, such as an Accountant.
    ///
    /// "ACCOUNTING_AND_FINANCE"
    #[serde(rename="ACCOUNTING_AND_FINANCE")]
    ACCOUNTINGANDFINANCE,
    

    /// An administrative and office job, such as an Administrative Assistant.
    ///
    /// "ADMINISTRATIVE_AND_OFFICE"
    #[serde(rename="ADMINISTRATIVE_AND_OFFICE")]
    ADMINISTRATIVEANDOFFICE,
    

    /// An advertising and marketing job, such as Marketing Manager.
    ///
    /// "ADVERTISING_AND_MARKETING"
    #[serde(rename="ADVERTISING_AND_MARKETING")]
    ADVERTISINGANDMARKETING,
    

    /// An animal care job, such as Veterinarian.
    ///
    /// "ANIMAL_CARE"
    #[serde(rename="ANIMAL_CARE")]
    ANIMALCARE,
    

    /// An art, fashion, or design job, such as Designer.
    ///
    /// "ART_FASHION_AND_DESIGN"
    #[serde(rename="ART_FASHION_AND_DESIGN")]
    ARTFASHIONANDDESIGN,
    

    /// A business operations job, such as Business Operations Manager.
    ///
    /// "BUSINESS_OPERATIONS"
    #[serde(rename="BUSINESS_OPERATIONS")]
    BUSINESSOPERATIONS,
    

    /// A cleaning and facilities job, such as Custodial Staff.
    ///
    /// "CLEANING_AND_FACILITIES"
    #[serde(rename="CLEANING_AND_FACILITIES")]
    CLEANINGANDFACILITIES,
    

    /// A computer and IT job, such as Systems Administrator.
    ///
    /// "COMPUTER_AND_IT"
    #[serde(rename="COMPUTER_AND_IT")]
    COMPUTERANDIT,
    

    /// A construction job, such as General Laborer.
    ///
    /// "CONSTRUCTION"
    #[serde(rename="CONSTRUCTION")]
    CONSTRUCTION,
    

    /// A customer service job, such s Cashier.
    ///
    /// "CUSTOMER_SERVICE"
    #[serde(rename="CUSTOMER_SERVICE")]
    CUSTOMERSERVICE,
    

    /// An education job, such as School Teacher.
    ///
    /// "EDUCATION"
    #[serde(rename="EDUCATION")]
    EDUCATION,
    

    /// An entertainment and travel job, such as Flight Attendant.
    ///
    /// "ENTERTAINMENT_AND_TRAVEL"
    #[serde(rename="ENTERTAINMENT_AND_TRAVEL")]
    ENTERTAINMENTANDTRAVEL,
    

    /// A farming or outdoor job, such as Park Ranger.
    ///
    /// "FARMING_AND_OUTDOORS"
    #[serde(rename="FARMING_AND_OUTDOORS")]
    FARMINGANDOUTDOORS,
    

    /// A healthcare job, such as Registered Nurse.
    ///
    /// "HEALTHCARE"
    #[serde(rename="HEALTHCARE")]
    HEALTHCARE,
    

    /// A human resources job, such as Human Resources Director.
    ///
    /// "HUMAN_RESOURCES"
    #[serde(rename="HUMAN_RESOURCES")]
    HUMANRESOURCES,
    

    /// An installation, maintenance, or repair job, such as Electrician.
    ///
    /// "INSTALLATION_MAINTENANCE_AND_REPAIR"
    #[serde(rename="INSTALLATION_MAINTENANCE_AND_REPAIR")]
    INSTALLATIONMAINTENANCEANDREPAIR,
    

    /// A legal job, such as Law Clerk.
    ///
    /// "LEGAL"
    #[serde(rename="LEGAL")]
    LEGAL,
    

    /// A management job, often used in conjunction with another category, such as Store Manager.
    ///
    /// "MANAGEMENT"
    #[serde(rename="MANAGEMENT")]
    MANAGEMENT,
    

    /// A manufacturing or warehouse job, such as Assembly Technician.
    ///
    /// "MANUFACTURING_AND_WAREHOUSE"
    #[serde(rename="MANUFACTURING_AND_WAREHOUSE")]
    MANUFACTURINGANDWAREHOUSE,
    

    /// A media, communications, or writing job, such as Media Relations.
    ///
    /// "MEDIA_COMMUNICATIONS_AND_WRITING"
    #[serde(rename="MEDIA_COMMUNICATIONS_AND_WRITING")]
    MEDIACOMMUNICATIONSANDWRITING,
    

    /// An oil, gas or mining job, such as Offshore Driller.
    ///
    /// "OIL_GAS_AND_MINING"
    #[serde(rename="OIL_GAS_AND_MINING")]
    OILGASANDMINING,
    

    /// A personal care and services job, such as Hair Stylist.
    ///
    /// "PERSONAL_CARE_AND_SERVICES"
    #[serde(rename="PERSONAL_CARE_AND_SERVICES")]
    PERSONALCAREANDSERVICES,
    

    /// A protective services job, such as Security Guard.
    ///
    /// "PROTECTIVE_SERVICES"
    #[serde(rename="PROTECTIVE_SERVICES")]
    PROTECTIVESERVICES,
    

    /// A real estate job, such as Buyer's Agent.
    ///
    /// "REAL_ESTATE"
    #[serde(rename="REAL_ESTATE")]
    REALESTATE,
    

    /// A restaurant and hospitality job, such as Restaurant Server.
    ///
    /// "RESTAURANT_AND_HOSPITALITY"
    #[serde(rename="RESTAURANT_AND_HOSPITALITY")]
    RESTAURANTANDHOSPITALITY,
    

    /// A sales and/or retail job, such Sales Associate.
    ///
    /// "SALES_AND_RETAIL"
    #[serde(rename="SALES_AND_RETAIL")]
    SALESANDRETAIL,
    

    /// A science and engineering job, such as Lab Technician.
    ///
    /// "SCIENCE_AND_ENGINEERING"
    #[serde(rename="SCIENCE_AND_ENGINEERING")]
    SCIENCEANDENGINEERING,
    

    /// A social services or non-profit job, such as Case Worker.
    ///
    /// "SOCIAL_SERVICES_AND_NON_PROFIT"
    #[serde(rename="SOCIAL_SERVICES_AND_NON_PROFIT")]
    SOCIALSERVICESANDNONPROFIT,
    

    /// A sports, fitness, or recreation job, such as Personal Trainer.
    ///
    /// "SPORTS_FITNESS_AND_RECREATION"
    #[serde(rename="SPORTS_FITNESS_AND_RECREATION")]
    SPORTSFITNESSANDRECREATION,
    

    /// A transportation or logistics job, such as Truck Driver.
    ///
    /// "TRANSPORTATION_AND_LOGISTICS"
    #[serde(rename="TRANSPORTATION_AND_LOGISTICS")]
    TRANSPORTATIONANDLOGISTICS,
}

impl AsRef<str> for JobQueryJobCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobQueryJobCategoriesEnum::JOBCATEGORYUNSPECIFIED => "JOB_CATEGORY_UNSPECIFIED",
            JobQueryJobCategoriesEnum::ACCOUNTINGANDFINANCE => "ACCOUNTING_AND_FINANCE",
            JobQueryJobCategoriesEnum::ADMINISTRATIVEANDOFFICE => "ADMINISTRATIVE_AND_OFFICE",
            JobQueryJobCategoriesEnum::ADVERTISINGANDMARKETING => "ADVERTISING_AND_MARKETING",
            JobQueryJobCategoriesEnum::ANIMALCARE => "ANIMAL_CARE",
            JobQueryJobCategoriesEnum::ARTFASHIONANDDESIGN => "ART_FASHION_AND_DESIGN",
            JobQueryJobCategoriesEnum::BUSINESSOPERATIONS => "BUSINESS_OPERATIONS",
            JobQueryJobCategoriesEnum::CLEANINGANDFACILITIES => "CLEANING_AND_FACILITIES",
            JobQueryJobCategoriesEnum::COMPUTERANDIT => "COMPUTER_AND_IT",
            JobQueryJobCategoriesEnum::CONSTRUCTION => "CONSTRUCTION",
            JobQueryJobCategoriesEnum::CUSTOMERSERVICE => "CUSTOMER_SERVICE",
            JobQueryJobCategoriesEnum::EDUCATION => "EDUCATION",
            JobQueryJobCategoriesEnum::ENTERTAINMENTANDTRAVEL => "ENTERTAINMENT_AND_TRAVEL",
            JobQueryJobCategoriesEnum::FARMINGANDOUTDOORS => "FARMING_AND_OUTDOORS",
            JobQueryJobCategoriesEnum::HEALTHCARE => "HEALTHCARE",
            JobQueryJobCategoriesEnum::HUMANRESOURCES => "HUMAN_RESOURCES",
            JobQueryJobCategoriesEnum::INSTALLATIONMAINTENANCEANDREPAIR => "INSTALLATION_MAINTENANCE_AND_REPAIR",
            JobQueryJobCategoriesEnum::LEGAL => "LEGAL",
            JobQueryJobCategoriesEnum::MANAGEMENT => "MANAGEMENT",
            JobQueryJobCategoriesEnum::MANUFACTURINGANDWAREHOUSE => "MANUFACTURING_AND_WAREHOUSE",
            JobQueryJobCategoriesEnum::MEDIACOMMUNICATIONSANDWRITING => "MEDIA_COMMUNICATIONS_AND_WRITING",
            JobQueryJobCategoriesEnum::OILGASANDMINING => "OIL_GAS_AND_MINING",
            JobQueryJobCategoriesEnum::PERSONALCAREANDSERVICES => "PERSONAL_CARE_AND_SERVICES",
            JobQueryJobCategoriesEnum::PROTECTIVESERVICES => "PROTECTIVE_SERVICES",
            JobQueryJobCategoriesEnum::REALESTATE => "REAL_ESTATE",
            JobQueryJobCategoriesEnum::RESTAURANTANDHOSPITALITY => "RESTAURANT_AND_HOSPITALITY",
            JobQueryJobCategoriesEnum::SALESANDRETAIL => "SALES_AND_RETAIL",
            JobQueryJobCategoriesEnum::SCIENCEANDENGINEERING => "SCIENCE_AND_ENGINEERING",
            JobQueryJobCategoriesEnum::SOCIALSERVICESANDNONPROFIT => "SOCIAL_SERVICES_AND_NON_PROFIT",
            JobQueryJobCategoriesEnum::SPORTSFITNESSANDRECREATION => "SPORTS_FITNESS_AND_RECREATION",
            JobQueryJobCategoriesEnum::TRANSPORTATIONANDLOGISTICS => "TRANSPORTATION_AND_LOGISTICS",
        }
    }
}

impl std::convert::TryFrom< &str> for JobQueryJobCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_CATEGORY_UNSPECIFIED" => Ok(JobQueryJobCategoriesEnum::JOBCATEGORYUNSPECIFIED),
           "ACCOUNTING_AND_FINANCE" => Ok(JobQueryJobCategoriesEnum::ACCOUNTINGANDFINANCE),
           "ADMINISTRATIVE_AND_OFFICE" => Ok(JobQueryJobCategoriesEnum::ADMINISTRATIVEANDOFFICE),
           "ADVERTISING_AND_MARKETING" => Ok(JobQueryJobCategoriesEnum::ADVERTISINGANDMARKETING),
           "ANIMAL_CARE" => Ok(JobQueryJobCategoriesEnum::ANIMALCARE),
           "ART_FASHION_AND_DESIGN" => Ok(JobQueryJobCategoriesEnum::ARTFASHIONANDDESIGN),
           "BUSINESS_OPERATIONS" => Ok(JobQueryJobCategoriesEnum::BUSINESSOPERATIONS),
           "CLEANING_AND_FACILITIES" => Ok(JobQueryJobCategoriesEnum::CLEANINGANDFACILITIES),
           "COMPUTER_AND_IT" => Ok(JobQueryJobCategoriesEnum::COMPUTERANDIT),
           "CONSTRUCTION" => Ok(JobQueryJobCategoriesEnum::CONSTRUCTION),
           "CUSTOMER_SERVICE" => Ok(JobQueryJobCategoriesEnum::CUSTOMERSERVICE),
           "EDUCATION" => Ok(JobQueryJobCategoriesEnum::EDUCATION),
           "ENTERTAINMENT_AND_TRAVEL" => Ok(JobQueryJobCategoriesEnum::ENTERTAINMENTANDTRAVEL),
           "FARMING_AND_OUTDOORS" => Ok(JobQueryJobCategoriesEnum::FARMINGANDOUTDOORS),
           "HEALTHCARE" => Ok(JobQueryJobCategoriesEnum::HEALTHCARE),
           "HUMAN_RESOURCES" => Ok(JobQueryJobCategoriesEnum::HUMANRESOURCES),
           "INSTALLATION_MAINTENANCE_AND_REPAIR" => Ok(JobQueryJobCategoriesEnum::INSTALLATIONMAINTENANCEANDREPAIR),
           "LEGAL" => Ok(JobQueryJobCategoriesEnum::LEGAL),
           "MANAGEMENT" => Ok(JobQueryJobCategoriesEnum::MANAGEMENT),
           "MANUFACTURING_AND_WAREHOUSE" => Ok(JobQueryJobCategoriesEnum::MANUFACTURINGANDWAREHOUSE),
           "MEDIA_COMMUNICATIONS_AND_WRITING" => Ok(JobQueryJobCategoriesEnum::MEDIACOMMUNICATIONSANDWRITING),
           "OIL_GAS_AND_MINING" => Ok(JobQueryJobCategoriesEnum::OILGASANDMINING),
           "PERSONAL_CARE_AND_SERVICES" => Ok(JobQueryJobCategoriesEnum::PERSONALCAREANDSERVICES),
           "PROTECTIVE_SERVICES" => Ok(JobQueryJobCategoriesEnum::PROTECTIVESERVICES),
           "REAL_ESTATE" => Ok(JobQueryJobCategoriesEnum::REALESTATE),
           "RESTAURANT_AND_HOSPITALITY" => Ok(JobQueryJobCategoriesEnum::RESTAURANTANDHOSPITALITY),
           "SALES_AND_RETAIL" => Ok(JobQueryJobCategoriesEnum::SALESANDRETAIL),
           "SCIENCE_AND_ENGINEERING" => Ok(JobQueryJobCategoriesEnum::SCIENCEANDENGINEERING),
           "SOCIAL_SERVICES_AND_NON_PROFIT" => Ok(JobQueryJobCategoriesEnum::SOCIALSERVICESANDNONPROFIT),
           "SPORTS_FITNESS_AND_RECREATION" => Ok(JobQueryJobCategoriesEnum::SPORTSFITNESSANDRECREATION),
           "TRANSPORTATION_AND_LOGISTICS" => Ok(JobQueryJobCategoriesEnum::TRANSPORTATIONANDLOGISTICS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobQueryJobCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocationLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of a location, which corresponds to the address lines field of google.type.PostalAddress. For example, "Downtown, Atlanta, GA, USA" has a type of LocationType.NEIGHBORHOOD, and "Kansas City, KS, USA" has a type of LocationType.LOCALITY.
pub enum LocationLocationTypeEnum {
    

    /// Default value if the type isn't specified.
    ///
    /// "LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="LOCATION_TYPE_UNSPECIFIED")]
    LOCATIONTYPEUNSPECIFIED,
    

    /// A country level location.
    ///
    /// "COUNTRY"
    #[serde(rename="COUNTRY")]
    COUNTRY,
    

    /// A state or equivalent level location.
    ///
    /// "ADMINISTRATIVE_AREA"
    #[serde(rename="ADMINISTRATIVE_AREA")]
    ADMINISTRATIVEAREA,
    

    /// A county or equivalent level location.
    ///
    /// "SUB_ADMINISTRATIVE_AREA"
    #[serde(rename="SUB_ADMINISTRATIVE_AREA")]
    SUBADMINISTRATIVEAREA,
    

    /// A city or equivalent level location.
    ///
    /// "LOCALITY"
    #[serde(rename="LOCALITY")]
    LOCALITY,
    

    /// A postal code level location.
    ///
    /// "POSTAL_CODE"
    #[serde(rename="POSTAL_CODE")]
    POSTALCODE,
    

    /// A sublocality is a subdivision of a locality, for example a city borough, ward, or arrondissement. Sublocalities are usually recognized by a local political authority. For example, Manhattan and Brooklyn are recognized as boroughs by the City of New York, and are therefore modeled as sublocalities.
    ///
    /// "SUB_LOCALITY"
    #[serde(rename="SUB_LOCALITY")]
    SUBLOCALITY,
    

    /// A district or equivalent level location.
    ///
    /// "SUB_LOCALITY_1"
    #[serde(rename="SUB_LOCALITY_1")]
    SUBLOCALITY1,
    

    /// A smaller district or equivalent level display.
    ///
    /// "SUB_LOCALITY_2"
    #[serde(rename="SUB_LOCALITY_2")]
    SUBLOCALITY2,
    

    /// A neighborhood level location.
    ///
    /// "NEIGHBORHOOD"
    #[serde(rename="NEIGHBORHOOD")]
    NEIGHBORHOOD,
    

    /// A street address level location.
    ///
    /// "STREET_ADDRESS"
    #[serde(rename="STREET_ADDRESS")]
    STREETADDRESS,
}

impl AsRef<str> for LocationLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocationLocationTypeEnum::LOCATIONTYPEUNSPECIFIED => "LOCATION_TYPE_UNSPECIFIED",
            LocationLocationTypeEnum::COUNTRY => "COUNTRY",
            LocationLocationTypeEnum::ADMINISTRATIVEAREA => "ADMINISTRATIVE_AREA",
            LocationLocationTypeEnum::SUBADMINISTRATIVEAREA => "SUB_ADMINISTRATIVE_AREA",
            LocationLocationTypeEnum::LOCALITY => "LOCALITY",
            LocationLocationTypeEnum::POSTALCODE => "POSTAL_CODE",
            LocationLocationTypeEnum::SUBLOCALITY => "SUB_LOCALITY",
            LocationLocationTypeEnum::SUBLOCALITY1 => "SUB_LOCALITY_1",
            LocationLocationTypeEnum::SUBLOCALITY2 => "SUB_LOCALITY_2",
            LocationLocationTypeEnum::NEIGHBORHOOD => "NEIGHBORHOOD",
            LocationLocationTypeEnum::STREETADDRESS => "STREET_ADDRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for LocationLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_TYPE_UNSPECIFIED" => Ok(LocationLocationTypeEnum::LOCATIONTYPEUNSPECIFIED),
           "COUNTRY" => Ok(LocationLocationTypeEnum::COUNTRY),
           "ADMINISTRATIVE_AREA" => Ok(LocationLocationTypeEnum::ADMINISTRATIVEAREA),
           "SUB_ADMINISTRATIVE_AREA" => Ok(LocationLocationTypeEnum::SUBADMINISTRATIVEAREA),
           "LOCALITY" => Ok(LocationLocationTypeEnum::LOCALITY),
           "POSTAL_CODE" => Ok(LocationLocationTypeEnum::POSTALCODE),
           "SUB_LOCALITY" => Ok(LocationLocationTypeEnum::SUBLOCALITY),
           "SUB_LOCALITY_1" => Ok(LocationLocationTypeEnum::SUBLOCALITY1),
           "SUB_LOCALITY_2" => Ok(LocationLocationTypeEnum::SUBLOCALITY2),
           "NEIGHBORHOOD" => Ok(LocationLocationTypeEnum::NEIGHBORHOOD),
           "STREET_ADDRESS" => Ok(LocationLocationTypeEnum::STREETADDRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocationLocationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocationFilterTelecommutePreferenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allows the client to return jobs without a set location, specifically, telecommuting jobs (telecommuting is considered by the service as a special location). Job.posting_region indicates if a job permits telecommuting. If this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED, telecommuting jobs are searched, and address and lat_lng are ignored. If not set or set to TelecommutePreference.TELECOMMUTE_EXCLUDED, the telecommute status of the jobs is ignored. Jobs that have PostingRegion.TELECOMMUTE and have additional Job.addresses may still be matched based on other location filters using address or latlng. This filter can be used by itself to search exclusively for telecommuting jobs, or it can be combined with another location filter to search for a combination of job locations, such as "Mountain View" or "telecommuting" jobs. However, when used in combination with other location filters, telecommuting jobs can be treated as less relevant than other jobs in the search response. This field is only used for job search requests.
pub enum LocationFilterTelecommutePreferenceEnum {
    

    /// Default value if the telecommute preference isn't specified.
    ///
    /// "TELECOMMUTE_PREFERENCE_UNSPECIFIED"
    #[serde(rename="TELECOMMUTE_PREFERENCE_UNSPECIFIED")]
    TELECOMMUTEPREFERENCEUNSPECIFIED,
    

    /// Deprecated: Ignore telecommute status of jobs. Use TELECOMMUTE_JOBS_EXCLUDED if want to exclude telecommute jobs.
    ///
    /// "TELECOMMUTE_EXCLUDED"
    #[serde(rename="TELECOMMUTE_EXCLUDED")]
    TELECOMMUTEEXCLUDED,
    

    /// Allow telecommute jobs.
    ///
    /// "TELECOMMUTE_ALLOWED"
    #[serde(rename="TELECOMMUTE_ALLOWED")]
    TELECOMMUTEALLOWED,
    

    /// Exclude telecommute jobs.
    ///
    /// "TELECOMMUTE_JOBS_EXCLUDED"
    #[serde(rename="TELECOMMUTE_JOBS_EXCLUDED")]
    TELECOMMUTEJOBSEXCLUDED,
}

impl AsRef<str> for LocationFilterTelecommutePreferenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocationFilterTelecommutePreferenceEnum::TELECOMMUTEPREFERENCEUNSPECIFIED => "TELECOMMUTE_PREFERENCE_UNSPECIFIED",
            LocationFilterTelecommutePreferenceEnum::TELECOMMUTEEXCLUDED => "TELECOMMUTE_EXCLUDED",
            LocationFilterTelecommutePreferenceEnum::TELECOMMUTEALLOWED => "TELECOMMUTE_ALLOWED",
            LocationFilterTelecommutePreferenceEnum::TELECOMMUTEJOBSEXCLUDED => "TELECOMMUTE_JOBS_EXCLUDED",
        }
    }
}

impl std::convert::TryFrom< &str> for LocationFilterTelecommutePreferenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TELECOMMUTE_PREFERENCE_UNSPECIFIED" => Ok(LocationFilterTelecommutePreferenceEnum::TELECOMMUTEPREFERENCEUNSPECIFIED),
           "TELECOMMUTE_EXCLUDED" => Ok(LocationFilterTelecommutePreferenceEnum::TELECOMMUTEEXCLUDED),
           "TELECOMMUTE_ALLOWED" => Ok(LocationFilterTelecommutePreferenceEnum::TELECOMMUTEALLOWED),
           "TELECOMMUTE_JOBS_EXCLUDED" => Ok(LocationFilterTelecommutePreferenceEnum::TELECOMMUTEJOBSEXCLUDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocationFilterTelecommutePreferenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProcessingOptionHtmlSanitizationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option for job HTML content sanitization. Applied fields are: * description * applicationInfo.instruction * incentives * qualifications * responsibilities HTML tags in these fields may be stripped if sanitiazation isn't disabled. Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY.
pub enum ProcessingOptionHtmlSanitizationEnum {
    

    /// Default value.
    ///
    /// "HTML_SANITIZATION_UNSPECIFIED"
    #[serde(rename="HTML_SANITIZATION_UNSPECIFIED")]
    HTMLSANITIZATIONUNSPECIFIED,
    

    /// Disables sanitization on HTML input.
    ///
    /// "HTML_SANITIZATION_DISABLED"
    #[serde(rename="HTML_SANITIZATION_DISABLED")]
    HTMLSANITIZATIONDISABLED,
    

    /// Sanitizes HTML input, only accepts bold, italic, ordered list, and unordered list markup tags.
    ///
    /// "SIMPLE_FORMATTING_ONLY"
    #[serde(rename="SIMPLE_FORMATTING_ONLY")]
    SIMPLEFORMATTINGONLY,
}

impl AsRef<str> for ProcessingOptionHtmlSanitizationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProcessingOptionHtmlSanitizationEnum::HTMLSANITIZATIONUNSPECIFIED => "HTML_SANITIZATION_UNSPECIFIED",
            ProcessingOptionHtmlSanitizationEnum::HTMLSANITIZATIONDISABLED => "HTML_SANITIZATION_DISABLED",
            ProcessingOptionHtmlSanitizationEnum::SIMPLEFORMATTINGONLY => "SIMPLE_FORMATTING_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for ProcessingOptionHtmlSanitizationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTML_SANITIZATION_UNSPECIFIED" => Ok(ProcessingOptionHtmlSanitizationEnum::HTMLSANITIZATIONUNSPECIFIED),
           "HTML_SANITIZATION_DISABLED" => Ok(ProcessingOptionHtmlSanitizationEnum::HTMLSANITIZATIONDISABLED),
           "SIMPLE_FORMATTING_ONLY" => Ok(ProcessingOptionHtmlSanitizationEnum::SIMPLEFORMATTINGONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProcessingOptionHtmlSanitizationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchJobsRequestDiversificationLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether highly similar jobs are returned next to each other in the search results. Jobs are identified as highly similar based on their titles, job categories, and locations. Highly similar results are clustered so that only one representative job of the cluster is displayed to the job seeker higher up in the results, with the other jobs being displayed lower down in the results. Defaults to DiversificationLevel.SIMPLE if no value is specified.
pub enum SearchJobsRequestDiversificationLevelEnum {
    

    /// The diversification level isn't specified.
    ///
    /// "DIVERSIFICATION_LEVEL_UNSPECIFIED"
    #[serde(rename="DIVERSIFICATION_LEVEL_UNSPECIFIED")]
    DIVERSIFICATIONLEVELUNSPECIFIED,
    

    /// Disables diversification. Jobs that would normally be pushed to the last page would not have their positions altered. This may result in highly similar jobs appearing in sequence in the search results.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Default diversifying behavior. The result list is ordered so that highly similar results are pushed to the end of the last page of search results.
    ///
    /// "SIMPLE"
    #[serde(rename="SIMPLE")]
    SIMPLE,
    

    /// Only one job from the same company will be shown at once, other jobs under same company are pushed to the end of the last page of search result.
    ///
    /// "ONE_PER_COMPANY"
    #[serde(rename="ONE_PER_COMPANY")]
    ONEPERCOMPANY,
    

    /// Similar to ONE_PER_COMPANY, but it allows at most two jobs in the same company to be shown at once, the other jobs under same company are pushed to the end of the last page of search result.
    ///
    /// "TWO_PER_COMPANY"
    #[serde(rename="TWO_PER_COMPANY")]
    TWOPERCOMPANY,
    

    /// Similar to ONE_PER_COMPANY, but it allows at most three jobs in the same company to be shown at once, the other jobs under same company are dropped.
    ///
    /// "MAX_THREE_PER_COMPANY"
    #[serde(rename="MAX_THREE_PER_COMPANY")]
    MAXTHREEPERCOMPANY,
    

    /// The result list is ordered such that somewhat similar results are pushed to the end of the last page of the search results. This option is recommended if SIMPLE diversification does not diversify enough.
    ///
    /// "DIVERSIFY_BY_LOOSER_SIMILARITY"
    #[serde(rename="DIVERSIFY_BY_LOOSER_SIMILARITY")]
    DIVERSIFYBYLOOSERSIMILARITY,
}

impl AsRef<str> for SearchJobsRequestDiversificationLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchJobsRequestDiversificationLevelEnum::DIVERSIFICATIONLEVELUNSPECIFIED => "DIVERSIFICATION_LEVEL_UNSPECIFIED",
            SearchJobsRequestDiversificationLevelEnum::DISABLED => "DISABLED",
            SearchJobsRequestDiversificationLevelEnum::SIMPLE => "SIMPLE",
            SearchJobsRequestDiversificationLevelEnum::ONEPERCOMPANY => "ONE_PER_COMPANY",
            SearchJobsRequestDiversificationLevelEnum::TWOPERCOMPANY => "TWO_PER_COMPANY",
            SearchJobsRequestDiversificationLevelEnum::MAXTHREEPERCOMPANY => "MAX_THREE_PER_COMPANY",
            SearchJobsRequestDiversificationLevelEnum::DIVERSIFYBYLOOSERSIMILARITY => "DIVERSIFY_BY_LOOSER_SIMILARITY",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchJobsRequestDiversificationLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIVERSIFICATION_LEVEL_UNSPECIFIED" => Ok(SearchJobsRequestDiversificationLevelEnum::DIVERSIFICATIONLEVELUNSPECIFIED),
           "DISABLED" => Ok(SearchJobsRequestDiversificationLevelEnum::DISABLED),
           "SIMPLE" => Ok(SearchJobsRequestDiversificationLevelEnum::SIMPLE),
           "ONE_PER_COMPANY" => Ok(SearchJobsRequestDiversificationLevelEnum::ONEPERCOMPANY),
           "TWO_PER_COMPANY" => Ok(SearchJobsRequestDiversificationLevelEnum::TWOPERCOMPANY),
           "MAX_THREE_PER_COMPANY" => Ok(SearchJobsRequestDiversificationLevelEnum::MAXTHREEPERCOMPANY),
           "DIVERSIFY_BY_LOOSER_SIMILARITY" => Ok(SearchJobsRequestDiversificationLevelEnum::DIVERSIFYBYLOOSERSIMILARITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchJobsRequestDiversificationLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchJobsRequestJobViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired job attributes returned for jobs in the search response. Defaults to JobView.JOB_VIEW_SMALL if no value is specified.
pub enum SearchJobsRequestJobViewEnum {
    

    /// Default value.
    ///
    /// "JOB_VIEW_UNSPECIFIED"
    #[serde(rename="JOB_VIEW_UNSPECIFIED")]
    JOBVIEWUNSPECIFIED,
    

    /// A ID only view of job, with following attributes: Job.name, Job.requisition_id, Job.language_code.
    ///
    /// "JOB_VIEW_ID_ONLY"
    #[serde(rename="JOB_VIEW_ID_ONLY")]
    JOBVIEWIDONLY,
    

    /// A minimal view of the job, with the following attributes: Job.name, Job.requisition_id, Job.title, Job.company, Job.DerivedInfo.locations, Job.language_code.
    ///
    /// "JOB_VIEW_MINIMAL"
    #[serde(rename="JOB_VIEW_MINIMAL")]
    JOBVIEWMINIMAL,
    

    /// A small view of the job, with the following attributes in the search results: Job.name, Job.requisition_id, Job.title, Job.company, Job.DerivedInfo.locations, Job.visibility, Job.language_code, Job.description.
    ///
    /// "JOB_VIEW_SMALL"
    #[serde(rename="JOB_VIEW_SMALL")]
    JOBVIEWSMALL,
    

    /// All available attributes are included in the search results.
    ///
    /// "JOB_VIEW_FULL"
    #[serde(rename="JOB_VIEW_FULL")]
    JOBVIEWFULL,
}

impl AsRef<str> for SearchJobsRequestJobViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchJobsRequestJobViewEnum::JOBVIEWUNSPECIFIED => "JOB_VIEW_UNSPECIFIED",
            SearchJobsRequestJobViewEnum::JOBVIEWIDONLY => "JOB_VIEW_ID_ONLY",
            SearchJobsRequestJobViewEnum::JOBVIEWMINIMAL => "JOB_VIEW_MINIMAL",
            SearchJobsRequestJobViewEnum::JOBVIEWSMALL => "JOB_VIEW_SMALL",
            SearchJobsRequestJobViewEnum::JOBVIEWFULL => "JOB_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchJobsRequestJobViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_VIEW_UNSPECIFIED" => Ok(SearchJobsRequestJobViewEnum::JOBVIEWUNSPECIFIED),
           "JOB_VIEW_ID_ONLY" => Ok(SearchJobsRequestJobViewEnum::JOBVIEWIDONLY),
           "JOB_VIEW_MINIMAL" => Ok(SearchJobsRequestJobViewEnum::JOBVIEWMINIMAL),
           "JOB_VIEW_SMALL" => Ok(SearchJobsRequestJobViewEnum::JOBVIEWSMALL),
           "JOB_VIEW_FULL" => Ok(SearchJobsRequestJobViewEnum::JOBVIEWFULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchJobsRequestJobViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchJobsRequestKeywordMatchModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls what keyword match options to use. If both keyword_match_mode and disable_keyword_match are set, keyword_match_mode will take precedence. Defaults to KeywordMatchMode.KEYWORD_MATCH_ALL if no value is specified.
pub enum SearchJobsRequestKeywordMatchModeEnum {
    

    /// The keyword match option isn't specified. Defaults to KeywordMatchMode.KEYWORD_MATCH_ALL behavior.
    ///
    /// "KEYWORD_MATCH_MODE_UNSPECIFIED"
    #[serde(rename="KEYWORD_MATCH_MODE_UNSPECIFIED")]
    KEYWORDMATCHMODEUNSPECIFIED,
    

    /// Disables keyword matching.
    ///
    /// "KEYWORD_MATCH_DISABLED"
    #[serde(rename="KEYWORD_MATCH_DISABLED")]
    KEYWORDMATCHDISABLED,
    

    /// Enable keyword matching over Job.title, Job.description, Job.company_display_name, Job.addresses, Job.qualifications, and keyword searchable Job.custom_attributes fields.
    ///
    /// "KEYWORD_MATCH_ALL"
    #[serde(rename="KEYWORD_MATCH_ALL")]
    KEYWORDMATCHALL,
    

    /// Only enable keyword matching over Job.title.
    ///
    /// "KEYWORD_MATCH_TITLE_ONLY"
    #[serde(rename="KEYWORD_MATCH_TITLE_ONLY")]
    KEYWORDMATCHTITLEONLY,
}

impl AsRef<str> for SearchJobsRequestKeywordMatchModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHMODEUNSPECIFIED => "KEYWORD_MATCH_MODE_UNSPECIFIED",
            SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHDISABLED => "KEYWORD_MATCH_DISABLED",
            SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHALL => "KEYWORD_MATCH_ALL",
            SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHTITLEONLY => "KEYWORD_MATCH_TITLE_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchJobsRequestKeywordMatchModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEYWORD_MATCH_MODE_UNSPECIFIED" => Ok(SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHMODEUNSPECIFIED),
           "KEYWORD_MATCH_DISABLED" => Ok(SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHDISABLED),
           "KEYWORD_MATCH_ALL" => Ok(SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHALL),
           "KEYWORD_MATCH_TITLE_ONLY" => Ok(SearchJobsRequestKeywordMatchModeEnum::KEYWORDMATCHTITLEONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchJobsRequestKeywordMatchModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchJobsRequestSearchModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode of a search. Defaults to SearchMode.JOB_SEARCH.
pub enum SearchJobsRequestSearchModeEnum {
    

    /// The mode of the search method isn't specified. The default search behavior is identical to JOB_SEARCH search behavior.
    ///
    /// "SEARCH_MODE_UNSPECIFIED"
    #[serde(rename="SEARCH_MODE_UNSPECIFIED")]
    SEARCHMODEUNSPECIFIED,
    

    /// The job search matches against all jobs, and featured jobs (jobs with promotionValue > 0) are not specially handled.
    ///
    /// "JOB_SEARCH"
    #[serde(rename="JOB_SEARCH")]
    JOBSEARCH,
    

    /// The job search matches only against featured jobs (jobs with a promotionValue > 0). This method doesn't return any jobs having a promotionValue <= 0. The search results order is determined by the promotionValue (jobs with a higher promotionValue are returned higher up in the search results), with relevance being used as a tiebreaker.
    ///
    /// "FEATURED_JOB_SEARCH"
    #[serde(rename="FEATURED_JOB_SEARCH")]
    FEATUREDJOBSEARCH,
}

impl AsRef<str> for SearchJobsRequestSearchModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchJobsRequestSearchModeEnum::SEARCHMODEUNSPECIFIED => "SEARCH_MODE_UNSPECIFIED",
            SearchJobsRequestSearchModeEnum::JOBSEARCH => "JOB_SEARCH",
            SearchJobsRequestSearchModeEnum::FEATUREDJOBSEARCH => "FEATURED_JOB_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchJobsRequestSearchModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEARCH_MODE_UNSPECIFIED" => Ok(SearchJobsRequestSearchModeEnum::SEARCHMODEUNSPECIFIED),
           "JOB_SEARCH" => Ok(SearchJobsRequestSearchModeEnum::JOBSEARCH),
           "FEATURED_JOB_SEARCH" => Ok(SearchJobsRequestSearchModeEnum::FEATUREDJOBSEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchJobsRequestSearchModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectJobViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired job attributes returned for jobs in the search response. Defaults to JobView.JOB_VIEW_FULL if no value is specified.
pub enum ProjectJobViewEnum {
    

    /// Default value.
    ///
    /// "JOB_VIEW_UNSPECIFIED"
    #[serde(rename="JOB_VIEW_UNSPECIFIED")]
    JOBVIEWUNSPECIFIED,
    

    /// A ID only view of job, with following attributes: Job.name, Job.requisition_id, Job.language_code.
    ///
    /// "JOB_VIEW_ID_ONLY"
    #[serde(rename="JOB_VIEW_ID_ONLY")]
    JOBVIEWIDONLY,
    

    /// A minimal view of the job, with the following attributes: Job.name, Job.requisition_id, Job.title, Job.company, Job.DerivedInfo.locations, Job.language_code.
    ///
    /// "JOB_VIEW_MINIMAL"
    #[serde(rename="JOB_VIEW_MINIMAL")]
    JOBVIEWMINIMAL,
    

    /// A small view of the job, with the following attributes in the search results: Job.name, Job.requisition_id, Job.title, Job.company, Job.DerivedInfo.locations, Job.visibility, Job.language_code, Job.description.
    ///
    /// "JOB_VIEW_SMALL"
    #[serde(rename="JOB_VIEW_SMALL")]
    JOBVIEWSMALL,
    

    /// All available attributes are included in the search results.
    ///
    /// "JOB_VIEW_FULL"
    #[serde(rename="JOB_VIEW_FULL")]
    JOBVIEWFULL,
}

impl AsRef<str> for ProjectJobViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectJobViewEnum::JOBVIEWUNSPECIFIED => "JOB_VIEW_UNSPECIFIED",
            ProjectJobViewEnum::JOBVIEWIDONLY => "JOB_VIEW_ID_ONLY",
            ProjectJobViewEnum::JOBVIEWMINIMAL => "JOB_VIEW_MINIMAL",
            ProjectJobViewEnum::JOBVIEWSMALL => "JOB_VIEW_SMALL",
            ProjectJobViewEnum::JOBVIEWFULL => "JOB_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectJobViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_VIEW_UNSPECIFIED" => Ok(ProjectJobViewEnum::JOBVIEWUNSPECIFIED),
           "JOB_VIEW_ID_ONLY" => Ok(ProjectJobViewEnum::JOBVIEWIDONLY),
           "JOB_VIEW_MINIMAL" => Ok(ProjectJobViewEnum::JOBVIEWMINIMAL),
           "JOB_VIEW_SMALL" => Ok(ProjectJobViewEnum::JOBVIEWSMALL),
           "JOB_VIEW_FULL" => Ok(ProjectJobViewEnum::JOBVIEWFULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectJobViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope of the completion. The defaults is CompletionScope.PUBLIC.
pub enum ProjectScopeEnum {
    

    /// Default value.
    ///
    /// "COMPLETION_SCOPE_UNSPECIFIED"
    #[serde(rename="COMPLETION_SCOPE_UNSPECIFIED")]
    COMPLETIONSCOPEUNSPECIFIED,
    

    /// Suggestions are based only on the data provided by the client.
    ///
    /// "TENANT"
    #[serde(rename="TENANT")]
    TENANT,
    

    /// Suggestions are based on all jobs data in the system that's visible to the client
    ///
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
}

impl AsRef<str> for ProjectScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectScopeEnum::COMPLETIONSCOPEUNSPECIFIED => "COMPLETION_SCOPE_UNSPECIFIED",
            ProjectScopeEnum::TENANT => "TENANT",
            ProjectScopeEnum::PUBLIC => "PUBLIC",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLETION_SCOPE_UNSPECIFIED" => Ok(ProjectScopeEnum::COMPLETIONSCOPEUNSPECIFIED),
           "TENANT" => Ok(ProjectScopeEnum::TENANT),
           "PUBLIC" => Ok(ProjectScopeEnum::PUBLIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The completion topic. The default is CompletionType.COMBINED.
pub enum ProjectTypeEnum {
    

    /// Default value.
    ///
    /// "COMPLETION_TYPE_UNSPECIFIED"
    #[serde(rename="COMPLETION_TYPE_UNSPECIFIED")]
    COMPLETIONTYPEUNSPECIFIED,
    

    /// Suggest job titles for jobs autocomplete. For CompletionType.JOB_TITLE type, only open jobs with the same language_codes are returned.
    ///
    /// "JOB_TITLE"
    #[serde(rename="JOB_TITLE")]
    JOBTITLE,
    

    /// Suggest company names for jobs autocomplete. For CompletionType.COMPANY_NAME type, only companies having open jobs with the same language_codes are returned.
    ///
    /// "COMPANY_NAME"
    #[serde(rename="COMPANY_NAME")]
    COMPANYNAME,
    

    /// Suggest both job titles and company names for jobs autocomplete. For CompletionType.COMBINED type, only open jobs with the same language_codes or companies having open jobs with the same language_codes are returned.
    ///
    /// "COMBINED"
    #[serde(rename="COMBINED")]
    COMBINED,
}

impl AsRef<str> for ProjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectTypeEnum::COMPLETIONTYPEUNSPECIFIED => "COMPLETION_TYPE_UNSPECIFIED",
            ProjectTypeEnum::JOBTITLE => "JOB_TITLE",
            ProjectTypeEnum::COMPANYNAME => "COMPANY_NAME",
            ProjectTypeEnum::COMBINED => "COMBINED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLETION_TYPE_UNSPECIFIED" => Ok(ProjectTypeEnum::COMPLETIONTYPEUNSPECIFIED),
           "JOB_TITLE" => Ok(ProjectTypeEnum::JOBTITLE),
           "COMPANY_NAME" => Ok(ProjectTypeEnum::COMPANYNAME),
           "COMBINED" => Ok(ProjectTypeEnum::COMBINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


