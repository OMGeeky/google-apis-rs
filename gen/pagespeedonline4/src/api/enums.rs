use super::*;



// region PagespeedapiStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The analysis strategy (desktop or mobile) to use, and desktop is the default
pub enum PagespeedapiStrategyEnum {
    

    /// Fetch and analyze the URL for desktop browsers
    ///
    /// "desktop"
    #[serde(rename="desktop")]
    Desktop,
    

    /// Fetch and analyze the URL for mobile devices
    ///
    /// "mobile"
    #[serde(rename="mobile")]
    Mobile,
}

impl AsRef<str> for PagespeedapiStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PagespeedapiStrategyEnum::Desktop => "desktop",
            PagespeedapiStrategyEnum::Mobile => "mobile",
        }
    }
}

impl std::convert::TryFrom< &str> for PagespeedapiStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "desktop" => Ok(PagespeedapiStrategyEnum::Desktop),
           "mobile" => Ok(PagespeedapiStrategyEnum::Mobile),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PagespeedapiStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


