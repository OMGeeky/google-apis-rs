use super::*;



// region WebfontCapabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the font urls in `Webfont.files`, by default, static ttf fonts are sent.
pub enum WebfontCapabilityEnum {
    

    /// Default means static ttf fonts.
    ///
    /// "CAPABILITY_UNSPECIFIED"
    #[serde(rename="CAPABILITY_UNSPECIFIED")]
    CAPABILITYUNSPECIFIED,
    

    /// Use WOFF2(Compressed)instead of ttf.
    ///
    /// "WOFF2"
    #[serde(rename="WOFF2")]
    WOFF2,
    

    /// Prefer variable font files instead of static fonts instantiated at standard weights.
    ///
    /// "VF"
    #[serde(rename="VF")]
    VF,
}

impl AsRef<str> for WebfontCapabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebfontCapabilityEnum::CAPABILITYUNSPECIFIED => "CAPABILITY_UNSPECIFIED",
            WebfontCapabilityEnum::WOFF2 => "WOFF2",
            WebfontCapabilityEnum::VF => "VF",
        }
    }
}

impl std::convert::TryFrom< &str> for WebfontCapabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAPABILITY_UNSPECIFIED" => Ok(WebfontCapabilityEnum::CAPABILITYUNSPECIFIED),
           "WOFF2" => Ok(WebfontCapabilityEnum::WOFF2),
           "VF" => Ok(WebfontCapabilityEnum::VF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebfontCapabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WebfontSortEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Enables sorting of the list.
pub enum WebfontSortEnum {
    

    /// No sorting specified, use the default sorting method.
    ///
    /// "SORT_UNDEFINED"
    #[serde(rename="SORT_UNDEFINED")]
    SORTUNDEFINED,
    

    /// Sort alphabetically
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Sort by date added
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Sort by popularity
    ///
    /// "POPULARITY"
    #[serde(rename="POPULARITY")]
    POPULARITY,
    

    /// Sort by number of styles
    ///
    /// "STYLE"
    #[serde(rename="STYLE")]
    STYLE,
    

    /// Sort by trending
    ///
    /// "TRENDING"
    #[serde(rename="TRENDING")]
    TRENDING,
}

impl AsRef<str> for WebfontSortEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebfontSortEnum::SORTUNDEFINED => "SORT_UNDEFINED",
            WebfontSortEnum::ALPHA => "ALPHA",
            WebfontSortEnum::DATE => "DATE",
            WebfontSortEnum::POPULARITY => "POPULARITY",
            WebfontSortEnum::STYLE => "STYLE",
            WebfontSortEnum::TRENDING => "TRENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for WebfontSortEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_UNDEFINED" => Ok(WebfontSortEnum::SORTUNDEFINED),
           "ALPHA" => Ok(WebfontSortEnum::ALPHA),
           "DATE" => Ok(WebfontSortEnum::DATE),
           "POPULARITY" => Ok(WebfontSortEnum::POPULARITY),
           "STYLE" => Ok(WebfontSortEnum::STYLE),
           "TRENDING" => Ok(WebfontSortEnum::TRENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebfontSortEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


