use super::*;



// region ConversionAdUserDataConsentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents consent for core platform services (CPS) preferences in settings. No default value. Acceptable values are: GRANTED: The desired consent status is to grant. Read the CPS preferences from GTE settings. DENIED: The desired consent status is to deny; CPS list is empty.
pub enum ConversionAdUserDataConsentEnum {
    

    /// Not specified.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Granted.
    ///
    /// "GRANTED"
    #[serde(rename="GRANTED")]
    GRANTED,
    

    /// Denied.
    ///
    /// "DENIED"
    #[serde(rename="DENIED")]
    DENIED,
}

impl AsRef<str> for ConversionAdUserDataConsentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConversionAdUserDataConsentEnum::UNKNOWN => "UNKNOWN",
            ConversionAdUserDataConsentEnum::GRANTED => "GRANTED",
            ConversionAdUserDataConsentEnum::DENIED => "DENIED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConversionAdUserDataConsentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(ConversionAdUserDataConsentEnum::UNKNOWN),
           "GRANTED" => Ok(ConversionAdUserDataConsentEnum::GRANTED),
           "DENIED" => Ok(ConversionAdUserDataConsentEnum::DENIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConversionAdUserDataConsentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


