use super::*;



// region PlatformSummaryBetterAdsStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The site's Ad Experience Report status on this platform.
pub enum PlatformSummaryBetterAdsStatusEnum {
    

    /// Not reviewed.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Passing.
    ///
    /// "PASSING"
    #[serde(rename="PASSING")]
    PASSING,
    

    /// Warning. No longer a possible status.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Failing.
    ///
    /// "FAILING"
    #[serde(rename="FAILING")]
    FAILING,
}

impl AsRef<str> for PlatformSummaryBetterAdsStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlatformSummaryBetterAdsStatusEnum::UNKNOWN => "UNKNOWN",
            PlatformSummaryBetterAdsStatusEnum::PASSING => "PASSING",
            PlatformSummaryBetterAdsStatusEnum::WARNING => "WARNING",
            PlatformSummaryBetterAdsStatusEnum::FAILING => "FAILING",
        }
    }
}

impl std::convert::TryFrom< &str> for PlatformSummaryBetterAdsStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(PlatformSummaryBetterAdsStatusEnum::UNKNOWN),
           "PASSING" => Ok(PlatformSummaryBetterAdsStatusEnum::PASSING),
           "WARNING" => Ok(PlatformSummaryBetterAdsStatusEnum::WARNING),
           "FAILING" => Ok(PlatformSummaryBetterAdsStatusEnum::FAILING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlatformSummaryBetterAdsStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlatformSummaryFilterStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The site's [enforcement status](https://support.google.com/webtools/answer/7308033) on this platform.
pub enum PlatformSummaryFilterStatusEnum {
    

    /// N/A.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Ad filtering is on.
    ///
    /// "ON"
    #[serde(rename="ON")]
    ON,
    

    /// Ad filtering is off.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// Ad filtering is paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Ad filtering is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for PlatformSummaryFilterStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlatformSummaryFilterStatusEnum::UNKNOWN => "UNKNOWN",
            PlatformSummaryFilterStatusEnum::ON => "ON",
            PlatformSummaryFilterStatusEnum::OFF => "OFF",
            PlatformSummaryFilterStatusEnum::PAUSED => "PAUSED",
            PlatformSummaryFilterStatusEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PlatformSummaryFilterStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(PlatformSummaryFilterStatusEnum::UNKNOWN),
           "ON" => Ok(PlatformSummaryFilterStatusEnum::ON),
           "OFF" => Ok(PlatformSummaryFilterStatusEnum::OFF),
           "PAUSED" => Ok(PlatformSummaryFilterStatusEnum::PAUSED),
           "PENDING" => Ok(PlatformSummaryFilterStatusEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlatformSummaryFilterStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlatformSummaryRegionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The site's regions on this platform. No longer populated, because there is no longer any semantic difference between sites in different regions.
pub enum PlatformSummaryRegionEnum {
    

    /// Ad standard not yet defined for your region.
    ///
    /// "REGION_UNKNOWN"
    #[serde(rename="REGION_UNKNOWN")]
    REGIONUNKNOWN,
    

    /// Region A.
    ///
    /// "REGION_A"
    #[serde(rename="REGION_A")]
    REGIONA,
    

    /// Region B.
    ///
    /// "REGION_B"
    #[serde(rename="REGION_B")]
    REGIONB,
    

    /// Region C.
    ///
    /// "REGION_C"
    #[serde(rename="REGION_C")]
    REGIONC,
}

impl AsRef<str> for PlatformSummaryRegionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlatformSummaryRegionEnum::REGIONUNKNOWN => "REGION_UNKNOWN",
            PlatformSummaryRegionEnum::REGIONA => "REGION_A",
            PlatformSummaryRegionEnum::REGIONB => "REGION_B",
            PlatformSummaryRegionEnum::REGIONC => "REGION_C",
        }
    }
}

impl std::convert::TryFrom< &str> for PlatformSummaryRegionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REGION_UNKNOWN" => Ok(PlatformSummaryRegionEnum::REGIONUNKNOWN),
           "REGION_A" => Ok(PlatformSummaryRegionEnum::REGIONA),
           "REGION_B" => Ok(PlatformSummaryRegionEnum::REGIONB),
           "REGION_C" => Ok(PlatformSummaryRegionEnum::REGIONC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlatformSummaryRegionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


