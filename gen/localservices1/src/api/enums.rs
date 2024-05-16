use super::*;



// region GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the lead has been charged.
pub enum GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum {
    

    /// Not specified.
    ///
    /// "CHARGE_STATUS_UNSPECIFIED"
    #[serde(rename="CHARGE_STATUS_UNSPECIFIED")]
    CHARGESTATUSUNSPECIFIED,
    

    /// Charged.
    ///
    /// "CHARGED"
    #[serde(rename="CHARGED")]
    CHARGED,
    

    /// Not charged.
    ///
    /// "NOT_CHARGED"
    #[serde(rename="NOT_CHARGED")]
    NOTCHARGED,
}

impl AsRef<str> for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum::CHARGESTATUSUNSPECIFIED => "CHARGE_STATUS_UNSPECIFIED",
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum::CHARGED => "CHARGED",
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum::NOTCHARGED => "NOT_CHARGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHARGE_STATUS_UNSPECIFIED" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum::CHARGESTATUSUNSPECIFIED),
           "CHARGED" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum::CHARGED),
           "NOT_CHARGED" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum::NOTCHARGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Lead type.
pub enum GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum {
    

    /// Not specified.
    ///
    /// "LEAD_TYPE_UNSPECIFIED"
    #[serde(rename="LEAD_TYPE_UNSPECIFIED")]
    LEADTYPEUNSPECIFIED,
    

    /// Message lead.
    ///
    /// "MESSAGE"
    #[serde(rename="MESSAGE")]
    MESSAGE,
    

    /// Phone call lead.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// Booking lead.
    ///
    /// "BOOKING"
    #[serde(rename="BOOKING")]
    BOOKING,
}

impl AsRef<str> for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::LEADTYPEUNSPECIFIED => "LEAD_TYPE_UNSPECIFIED",
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::MESSAGE => "MESSAGE",
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::PHONECALL => "PHONE_CALL",
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::BOOKING => "BOOKING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEAD_TYPE_UNSPECIFIED" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::LEADTYPEUNSPECIFIED),
           "MESSAGE" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::MESSAGE),
           "PHONE_CALL" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::PHONECALL),
           "BOOKING" => Ok(GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum::BOOKING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


