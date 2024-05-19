use super::*;



// region UrlProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Additional information to return.
pub enum UrlProjectionEnum {
    

    /// Returns short URL click counts.
    ///
    /// "ANALYTICS_CLICKS"
    #[serde(rename="ANALYTICS_CLICKS")]
    ANALYTICSCLICKS,
    

    /// Returns short URL click counts.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for UrlProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlProjectionEnum::ANALYTICSCLICKS => "ANALYTICS_CLICKS",
            UrlProjectionEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANALYTICS_CLICKS" => Ok(UrlProjectionEnum::ANALYTICSCLICKS),
           "FULL" => Ok(UrlProjectionEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


