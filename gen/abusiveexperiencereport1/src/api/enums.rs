use super::*;



// region SiteSummaryResponseAbusiveStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The site's Abusive Experience Report status.
pub enum SiteSummaryResponseAbusiveStatusEnum {
    

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
    

    /// Failing.
    ///
    /// "FAILING"
    #[serde(rename="FAILING")]
    FAILING,
}

impl AsRef<str> for SiteSummaryResponseAbusiveStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteSummaryResponseAbusiveStatusEnum::UNKNOWN => "UNKNOWN",
            SiteSummaryResponseAbusiveStatusEnum::PASSING => "PASSING",
            SiteSummaryResponseAbusiveStatusEnum::FAILING => "FAILING",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteSummaryResponseAbusiveStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SiteSummaryResponseAbusiveStatusEnum::UNKNOWN),
           "PASSING" => Ok(SiteSummaryResponseAbusiveStatusEnum::PASSING),
           "FAILING" => Ok(SiteSummaryResponseAbusiveStatusEnum::FAILING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteSummaryResponseAbusiveStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteSummaryResponseFilterStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The site's [enforcement status](https://support.google.com/webtools/answer/7538608).
pub enum SiteSummaryResponseFilterStatusEnum {
    

    /// N/A.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Enforcement is on.
    ///
    /// "ON"
    #[serde(rename="ON")]
    ON,
    

    /// Enforcement is off.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// Enforcement is paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Enforcement is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for SiteSummaryResponseFilterStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteSummaryResponseFilterStatusEnum::UNKNOWN => "UNKNOWN",
            SiteSummaryResponseFilterStatusEnum::ON => "ON",
            SiteSummaryResponseFilterStatusEnum::OFF => "OFF",
            SiteSummaryResponseFilterStatusEnum::PAUSED => "PAUSED",
            SiteSummaryResponseFilterStatusEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteSummaryResponseFilterStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SiteSummaryResponseFilterStatusEnum::UNKNOWN),
           "ON" => Ok(SiteSummaryResponseFilterStatusEnum::ON),
           "OFF" => Ok(SiteSummaryResponseFilterStatusEnum::OFF),
           "PAUSED" => Ok(SiteSummaryResponseFilterStatusEnum::PAUSED),
           "PENDING" => Ok(SiteSummaryResponseFilterStatusEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteSummaryResponseFilterStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


