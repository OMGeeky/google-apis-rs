use super::*;



// region PagespeedapiCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A Lighthouse category to run; if none are given, only Performance category will be run
pub enum PagespeedapiCategoryEnum {
    

    /// Default UNDEFINED category.
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// Accessibility (a11y), category pertaining to a website's capacity to be accessible to all users.
    ///
    /// "ACCESSIBILITY"
    #[serde(rename="ACCESSIBILITY")]
    ACCESSIBILITY,
    

    /// Best Practices, category pertaining to a website's conformance to web best practice.
    ///
    /// "BEST_PRACTICES"
    #[serde(rename="BEST_PRACTICES")]
    BESTPRACTICES,
    

    /// Performance, category pertaining to a website's performance.
    ///
    /// "PERFORMANCE"
    #[serde(rename="PERFORMANCE")]
    PERFORMANCE,
    

    /// Progressive Web App (PWA), category pertaining to a website's ability to be run as a PWA.
    ///
    /// "PWA"
    #[serde(rename="PWA")]
    PWA,
    

    /// Search Engine Optimization (SEO), category pertaining to a website's ability to be indexed by search engines.
    ///
    /// "SEO"
    #[serde(rename="SEO")]
    SEO,
}

impl AsRef<str> for PagespeedapiCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PagespeedapiCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            PagespeedapiCategoryEnum::ACCESSIBILITY => "ACCESSIBILITY",
            PagespeedapiCategoryEnum::BESTPRACTICES => "BEST_PRACTICES",
            PagespeedapiCategoryEnum::PERFORMANCE => "PERFORMANCE",
            PagespeedapiCategoryEnum::PWA => "PWA",
            PagespeedapiCategoryEnum::SEO => "SEO",
        }
    }
}

impl std::convert::TryFrom< &str> for PagespeedapiCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(PagespeedapiCategoryEnum::CATEGORYUNSPECIFIED),
           "ACCESSIBILITY" => Ok(PagespeedapiCategoryEnum::ACCESSIBILITY),
           "BEST_PRACTICES" => Ok(PagespeedapiCategoryEnum::BESTPRACTICES),
           "PERFORMANCE" => Ok(PagespeedapiCategoryEnum::PERFORMANCE),
           "PWA" => Ok(PagespeedapiCategoryEnum::PWA),
           "SEO" => Ok(PagespeedapiCategoryEnum::SEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PagespeedapiCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PagespeedapiStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The analysis strategy (desktop or mobile) to use, and desktop is the default
pub enum PagespeedapiStrategyEnum {
    

    /// UNDEFINED.
    ///
    /// "STRATEGY_UNSPECIFIED"
    #[serde(rename="STRATEGY_UNSPECIFIED")]
    STRATEGYUNSPECIFIED,
    

    /// Fetch and analyze the URL for desktop browsers.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// Fetch and analyze the URL for mobile devices.
    ///
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
}

impl AsRef<str> for PagespeedapiStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PagespeedapiStrategyEnum::STRATEGYUNSPECIFIED => "STRATEGY_UNSPECIFIED",
            PagespeedapiStrategyEnum::DESKTOP => "DESKTOP",
            PagespeedapiStrategyEnum::MOBILE => "MOBILE",
        }
    }
}

impl std::convert::TryFrom< &str> for PagespeedapiStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STRATEGY_UNSPECIFIED" => Ok(PagespeedapiStrategyEnum::STRATEGYUNSPECIFIED),
           "DESKTOP" => Ok(PagespeedapiStrategyEnum::DESKTOP),
           "MOBILE" => Ok(PagespeedapiStrategyEnum::MOBILE),
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


