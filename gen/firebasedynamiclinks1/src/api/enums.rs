use super::*;



// region DynamicLinkEventStatEventEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Link event.
pub enum DynamicLinkEventStatEventEnum {
    

    /// Unspecified type.
    ///
    /// "DYNAMIC_LINK_EVENT_UNSPECIFIED"
    #[serde(rename="DYNAMIC_LINK_EVENT_UNSPECIFIED")]
    DYNAMICLINKEVENTUNSPECIFIED,
    

    /// Indicates that an FDL is clicked by users.
    ///
    /// "CLICK"
    #[serde(rename="CLICK")]
    CLICK,
    

    /// Indicates that an FDL redirects users to fallback link.
    ///
    /// "REDIRECT"
    #[serde(rename="REDIRECT")]
    REDIRECT,
    

    /// Indicates that an FDL triggers an app install from Play store, currently it's impossible to get stats from App store.
    ///
    /// "APP_INSTALL"
    #[serde(rename="APP_INSTALL")]
    APPINSTALL,
    

    /// Indicates that the app is opened for the first time after an install triggered by FDLs
    ///
    /// "APP_FIRST_OPEN"
    #[serde(rename="APP_FIRST_OPEN")]
    APPFIRSTOPEN,
    

    /// Indicates that the app is opened via an FDL for non-first time.
    ///
    /// "APP_RE_OPEN"
    #[serde(rename="APP_RE_OPEN")]
    APPREOPEN,
}

impl AsRef<str> for DynamicLinkEventStatEventEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicLinkEventStatEventEnum::DYNAMICLINKEVENTUNSPECIFIED => "DYNAMIC_LINK_EVENT_UNSPECIFIED",
            DynamicLinkEventStatEventEnum::CLICK => "CLICK",
            DynamicLinkEventStatEventEnum::REDIRECT => "REDIRECT",
            DynamicLinkEventStatEventEnum::APPINSTALL => "APP_INSTALL",
            DynamicLinkEventStatEventEnum::APPFIRSTOPEN => "APP_FIRST_OPEN",
            DynamicLinkEventStatEventEnum::APPREOPEN => "APP_RE_OPEN",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicLinkEventStatEventEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DYNAMIC_LINK_EVENT_UNSPECIFIED" => Ok(DynamicLinkEventStatEventEnum::DYNAMICLINKEVENTUNSPECIFIED),
           "CLICK" => Ok(DynamicLinkEventStatEventEnum::CLICK),
           "REDIRECT" => Ok(DynamicLinkEventStatEventEnum::REDIRECT),
           "APP_INSTALL" => Ok(DynamicLinkEventStatEventEnum::APPINSTALL),
           "APP_FIRST_OPEN" => Ok(DynamicLinkEventStatEventEnum::APPFIRSTOPEN),
           "APP_RE_OPEN" => Ok(DynamicLinkEventStatEventEnum::APPREOPEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicLinkEventStatEventEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DynamicLinkEventStatPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requested platform.
pub enum DynamicLinkEventStatPlatformEnum {
    

    /// Unspecified platform.
    ///
    /// "DYNAMIC_LINK_PLATFORM_UNSPECIFIED"
    #[serde(rename="DYNAMIC_LINK_PLATFORM_UNSPECIFIED")]
    DYNAMICLINKPLATFORMUNSPECIFIED,
    

    /// Represents Android platform. All apps and browsers on Android are classfied in this category.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Represents iOS platform. All apps and browsers on iOS are classfied in this category.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Represents desktop.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// Platforms are not categorized as Android/iOS/Destop fall into here.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for DynamicLinkEventStatPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicLinkEventStatPlatformEnum::DYNAMICLINKPLATFORMUNSPECIFIED => "DYNAMIC_LINK_PLATFORM_UNSPECIFIED",
            DynamicLinkEventStatPlatformEnum::ANDROID => "ANDROID",
            DynamicLinkEventStatPlatformEnum::IOS => "IOS",
            DynamicLinkEventStatPlatformEnum::DESKTOP => "DESKTOP",
            DynamicLinkEventStatPlatformEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicLinkEventStatPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DYNAMIC_LINK_PLATFORM_UNSPECIFIED" => Ok(DynamicLinkEventStatPlatformEnum::DYNAMICLINKPLATFORMUNSPECIFIED),
           "ANDROID" => Ok(DynamicLinkEventStatPlatformEnum::ANDROID),
           "IOS" => Ok(DynamicLinkEventStatPlatformEnum::IOS),
           "DESKTOP" => Ok(DynamicLinkEventStatPlatformEnum::DESKTOP),
           "OTHER" => Ok(DynamicLinkEventStatPlatformEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicLinkEventStatPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DynamicLinkWarningWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The warning code.
pub enum DynamicLinkWarningWarningCodeEnum {
    

    /// Unknown code.
    ///
    /// "CODE_UNSPECIFIED"
    #[serde(rename="CODE_UNSPECIFIED")]
    CODEUNSPECIFIED,
    

    /// The Android package does not match any in developer's DevConsole project.
    ///
    /// "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME"
    #[serde(rename="NOT_IN_PROJECT_ANDROID_PACKAGE_NAME")]
    NOTINPROJECTANDROIDPACKAGENAME,
    

    /// The Android minimum version code has to be a valid integer.
    ///
    /// "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION"
    #[serde(rename="NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION")]
    NOTINTEGERANDROIDPACKAGEMINVERSION,
    

    /// Android package min version param is not needed, e.g. when 'apn' is missing.
    ///
    /// "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION"
    #[serde(rename="UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION")]
    UNNECESSARYANDROIDPACKAGEMINVERSION,
    

    /// Android link is not a valid URI.
    ///
    /// "NOT_URI_ANDROID_LINK"
    #[serde(rename="NOT_URI_ANDROID_LINK")]
    NOTURIANDROIDLINK,
    

    /// Android link param is not needed, e.g. when param 'al' and 'link' have the same value..
    ///
    /// "UNNECESSARY_ANDROID_LINK"
    #[serde(rename="UNNECESSARY_ANDROID_LINK")]
    UNNECESSARYANDROIDLINK,
    

    /// Android fallback link is not a valid URI.
    ///
    /// "NOT_URI_ANDROID_FALLBACK_LINK"
    #[serde(rename="NOT_URI_ANDROID_FALLBACK_LINK")]
    NOTURIANDROIDFALLBACKLINK,
    

    /// Android fallback link has an invalid (non http/https) URI scheme.
    ///
    /// "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK"
    #[serde(rename="BAD_URI_SCHEME_ANDROID_FALLBACK_LINK")]
    BADURISCHEMEANDROIDFALLBACKLINK,
    

    /// The iOS bundle ID does not match any in developer's DevConsole project.
    ///
    /// "NOT_IN_PROJECT_IOS_BUNDLE_ID"
    #[serde(rename="NOT_IN_PROJECT_IOS_BUNDLE_ID")]
    NOTINPROJECTIOSBUNDLEID,
    

    /// The iPad bundle ID does not match any in developer's DevConsole project.
    ///
    /// "NOT_IN_PROJECT_IPAD_BUNDLE_ID"
    #[serde(rename="NOT_IN_PROJECT_IPAD_BUNDLE_ID")]
    NOTINPROJECTIPADBUNDLEID,
    

    /// iOS URL scheme is not needed, e.g. when 'ibi' are 'ipbi' are all missing.
    ///
    /// "UNNECESSARY_IOS_URL_SCHEME"
    #[serde(rename="UNNECESSARY_IOS_URL_SCHEME")]
    UNNECESSARYIOSURLSCHEME,
    

    /// iOS app store ID format is incorrect, e.g. not numeric.
    ///
    /// "NOT_NUMERIC_IOS_APP_STORE_ID"
    #[serde(rename="NOT_NUMERIC_IOS_APP_STORE_ID")]
    NOTNUMERICIOSAPPSTOREID,
    

    /// iOS app store ID is not needed.
    ///
    /// "UNNECESSARY_IOS_APP_STORE_ID"
    #[serde(rename="UNNECESSARY_IOS_APP_STORE_ID")]
    UNNECESSARYIOSAPPSTOREID,
    

    /// iOS fallback link is not a valid URI.
    ///
    /// "NOT_URI_IOS_FALLBACK_LINK"
    #[serde(rename="NOT_URI_IOS_FALLBACK_LINK")]
    NOTURIIOSFALLBACKLINK,
    

    /// iOS fallback link has an invalid (non http/https) URI scheme.
    ///
    /// "BAD_URI_SCHEME_IOS_FALLBACK_LINK"
    #[serde(rename="BAD_URI_SCHEME_IOS_FALLBACK_LINK")]
    BADURISCHEMEIOSFALLBACKLINK,
    

    /// iPad fallback link is not a valid URI.
    ///
    /// "NOT_URI_IPAD_FALLBACK_LINK"
    #[serde(rename="NOT_URI_IPAD_FALLBACK_LINK")]
    NOTURIIPADFALLBACKLINK,
    

    /// iPad fallback link has an invalid (non http/https) URI scheme.
    ///
    /// "BAD_URI_SCHEME_IPAD_FALLBACK_LINK"
    #[serde(rename="BAD_URI_SCHEME_IPAD_FALLBACK_LINK")]
    BADURISCHEMEIPADFALLBACKLINK,
    

    /// Debug param format is incorrect.
    ///
    /// "BAD_DEBUG_PARAM"
    #[serde(rename="BAD_DEBUG_PARAM")]
    BADDEBUGPARAM,
    

    /// isAd param format is incorrect.
    ///
    /// "BAD_AD_PARAM"
    #[serde(rename="BAD_AD_PARAM")]
    BADADPARAM,
    

    /// Indicates a certain param is deprecated.
    ///
    /// "DEPRECATED_PARAM"
    #[serde(rename="DEPRECATED_PARAM")]
    DEPRECATEDPARAM,
    

    /// Indicates certain paramater is not recognized.
    ///
    /// "UNRECOGNIZED_PARAM"
    #[serde(rename="UNRECOGNIZED_PARAM")]
    UNRECOGNIZEDPARAM,
    

    /// Indicates certain paramater is too long.
    ///
    /// "TOO_LONG_PARAM"
    #[serde(rename="TOO_LONG_PARAM")]
    TOOLONGPARAM,
    

    /// Social meta tag image link is not a valid URI.
    ///
    /// "NOT_URI_SOCIAL_IMAGE_LINK"
    #[serde(rename="NOT_URI_SOCIAL_IMAGE_LINK")]
    NOTURISOCIALIMAGELINK,
    

    /// Social meta tag image link has an invalid (non http/https) URI scheme.
    ///
    /// "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK"
    #[serde(rename="BAD_URI_SCHEME_SOCIAL_IMAGE_LINK")]
    BADURISCHEMESOCIALIMAGELINK,
    
    /// "NOT_URI_SOCIAL_URL"
    #[serde(rename="NOT_URI_SOCIAL_URL")]
    NOTURISOCIALURL,
    
    /// "BAD_URI_SCHEME_SOCIAL_URL"
    #[serde(rename="BAD_URI_SCHEME_SOCIAL_URL")]
    BADURISCHEMESOCIALURL,
    

    /// Dynamic Link URL length is too long.
    ///
    /// "LINK_LENGTH_TOO_LONG"
    #[serde(rename="LINK_LENGTH_TOO_LONG")]
    LINKLENGTHTOOLONG,
    

    /// Dynamic Link URL contains fragments.
    ///
    /// "LINK_WITH_FRAGMENTS"
    #[serde(rename="LINK_WITH_FRAGMENTS")]
    LINKWITHFRAGMENTS,
    

    /// The iOS bundle ID does not match with the given iOS store ID.
    ///
    /// "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID"
    #[serde(rename="NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID")]
    NOTMATCHINGIOSBUNDLEIDANDSTOREID,
}

impl AsRef<str> for DynamicLinkWarningWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicLinkWarningWarningCodeEnum::CODEUNSPECIFIED => "CODE_UNSPECIFIED",
            DynamicLinkWarningWarningCodeEnum::NOTINPROJECTANDROIDPACKAGENAME => "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME",
            DynamicLinkWarningWarningCodeEnum::NOTINTEGERANDROIDPACKAGEMINVERSION => "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION",
            DynamicLinkWarningWarningCodeEnum::UNNECESSARYANDROIDPACKAGEMINVERSION => "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION",
            DynamicLinkWarningWarningCodeEnum::NOTURIANDROIDLINK => "NOT_URI_ANDROID_LINK",
            DynamicLinkWarningWarningCodeEnum::UNNECESSARYANDROIDLINK => "UNNECESSARY_ANDROID_LINK",
            DynamicLinkWarningWarningCodeEnum::NOTURIANDROIDFALLBACKLINK => "NOT_URI_ANDROID_FALLBACK_LINK",
            DynamicLinkWarningWarningCodeEnum::BADURISCHEMEANDROIDFALLBACKLINK => "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK",
            DynamicLinkWarningWarningCodeEnum::NOTINPROJECTIOSBUNDLEID => "NOT_IN_PROJECT_IOS_BUNDLE_ID",
            DynamicLinkWarningWarningCodeEnum::NOTINPROJECTIPADBUNDLEID => "NOT_IN_PROJECT_IPAD_BUNDLE_ID",
            DynamicLinkWarningWarningCodeEnum::UNNECESSARYIOSURLSCHEME => "UNNECESSARY_IOS_URL_SCHEME",
            DynamicLinkWarningWarningCodeEnum::NOTNUMERICIOSAPPSTOREID => "NOT_NUMERIC_IOS_APP_STORE_ID",
            DynamicLinkWarningWarningCodeEnum::UNNECESSARYIOSAPPSTOREID => "UNNECESSARY_IOS_APP_STORE_ID",
            DynamicLinkWarningWarningCodeEnum::NOTURIIOSFALLBACKLINK => "NOT_URI_IOS_FALLBACK_LINK",
            DynamicLinkWarningWarningCodeEnum::BADURISCHEMEIOSFALLBACKLINK => "BAD_URI_SCHEME_IOS_FALLBACK_LINK",
            DynamicLinkWarningWarningCodeEnum::NOTURIIPADFALLBACKLINK => "NOT_URI_IPAD_FALLBACK_LINK",
            DynamicLinkWarningWarningCodeEnum::BADURISCHEMEIPADFALLBACKLINK => "BAD_URI_SCHEME_IPAD_FALLBACK_LINK",
            DynamicLinkWarningWarningCodeEnum::BADDEBUGPARAM => "BAD_DEBUG_PARAM",
            DynamicLinkWarningWarningCodeEnum::BADADPARAM => "BAD_AD_PARAM",
            DynamicLinkWarningWarningCodeEnum::DEPRECATEDPARAM => "DEPRECATED_PARAM",
            DynamicLinkWarningWarningCodeEnum::UNRECOGNIZEDPARAM => "UNRECOGNIZED_PARAM",
            DynamicLinkWarningWarningCodeEnum::TOOLONGPARAM => "TOO_LONG_PARAM",
            DynamicLinkWarningWarningCodeEnum::NOTURISOCIALIMAGELINK => "NOT_URI_SOCIAL_IMAGE_LINK",
            DynamicLinkWarningWarningCodeEnum::BADURISCHEMESOCIALIMAGELINK => "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK",
            DynamicLinkWarningWarningCodeEnum::NOTURISOCIALURL => "NOT_URI_SOCIAL_URL",
            DynamicLinkWarningWarningCodeEnum::BADURISCHEMESOCIALURL => "BAD_URI_SCHEME_SOCIAL_URL",
            DynamicLinkWarningWarningCodeEnum::LINKLENGTHTOOLONG => "LINK_LENGTH_TOO_LONG",
            DynamicLinkWarningWarningCodeEnum::LINKWITHFRAGMENTS => "LINK_WITH_FRAGMENTS",
            DynamicLinkWarningWarningCodeEnum::NOTMATCHINGIOSBUNDLEIDANDSTOREID => "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicLinkWarningWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CODE_UNSPECIFIED" => Ok(DynamicLinkWarningWarningCodeEnum::CODEUNSPECIFIED),
           "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME" => Ok(DynamicLinkWarningWarningCodeEnum::NOTINPROJECTANDROIDPACKAGENAME),
           "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION" => Ok(DynamicLinkWarningWarningCodeEnum::NOTINTEGERANDROIDPACKAGEMINVERSION),
           "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION" => Ok(DynamicLinkWarningWarningCodeEnum::UNNECESSARYANDROIDPACKAGEMINVERSION),
           "NOT_URI_ANDROID_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::NOTURIANDROIDLINK),
           "UNNECESSARY_ANDROID_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::UNNECESSARYANDROIDLINK),
           "NOT_URI_ANDROID_FALLBACK_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::NOTURIANDROIDFALLBACKLINK),
           "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::BADURISCHEMEANDROIDFALLBACKLINK),
           "NOT_IN_PROJECT_IOS_BUNDLE_ID" => Ok(DynamicLinkWarningWarningCodeEnum::NOTINPROJECTIOSBUNDLEID),
           "NOT_IN_PROJECT_IPAD_BUNDLE_ID" => Ok(DynamicLinkWarningWarningCodeEnum::NOTINPROJECTIPADBUNDLEID),
           "UNNECESSARY_IOS_URL_SCHEME" => Ok(DynamicLinkWarningWarningCodeEnum::UNNECESSARYIOSURLSCHEME),
           "NOT_NUMERIC_IOS_APP_STORE_ID" => Ok(DynamicLinkWarningWarningCodeEnum::NOTNUMERICIOSAPPSTOREID),
           "UNNECESSARY_IOS_APP_STORE_ID" => Ok(DynamicLinkWarningWarningCodeEnum::UNNECESSARYIOSAPPSTOREID),
           "NOT_URI_IOS_FALLBACK_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::NOTURIIOSFALLBACKLINK),
           "BAD_URI_SCHEME_IOS_FALLBACK_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::BADURISCHEMEIOSFALLBACKLINK),
           "NOT_URI_IPAD_FALLBACK_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::NOTURIIPADFALLBACKLINK),
           "BAD_URI_SCHEME_IPAD_FALLBACK_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::BADURISCHEMEIPADFALLBACKLINK),
           "BAD_DEBUG_PARAM" => Ok(DynamicLinkWarningWarningCodeEnum::BADDEBUGPARAM),
           "BAD_AD_PARAM" => Ok(DynamicLinkWarningWarningCodeEnum::BADADPARAM),
           "DEPRECATED_PARAM" => Ok(DynamicLinkWarningWarningCodeEnum::DEPRECATEDPARAM),
           "UNRECOGNIZED_PARAM" => Ok(DynamicLinkWarningWarningCodeEnum::UNRECOGNIZEDPARAM),
           "TOO_LONG_PARAM" => Ok(DynamicLinkWarningWarningCodeEnum::TOOLONGPARAM),
           "NOT_URI_SOCIAL_IMAGE_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::NOTURISOCIALIMAGELINK),
           "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK" => Ok(DynamicLinkWarningWarningCodeEnum::BADURISCHEMESOCIALIMAGELINK),
           "NOT_URI_SOCIAL_URL" => Ok(DynamicLinkWarningWarningCodeEnum::NOTURISOCIALURL),
           "BAD_URI_SCHEME_SOCIAL_URL" => Ok(DynamicLinkWarningWarningCodeEnum::BADURISCHEMESOCIALURL),
           "LINK_LENGTH_TOO_LONG" => Ok(DynamicLinkWarningWarningCodeEnum::LINKLENGTHTOOLONG),
           "LINK_WITH_FRAGMENTS" => Ok(DynamicLinkWarningWarningCodeEnum::LINKWITHFRAGMENTS),
           "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID" => Ok(DynamicLinkWarningWarningCodeEnum::NOTMATCHINGIOSBUNDLEIDANDSTOREID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicLinkWarningWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GetIosPostInstallAttributionRequestRetrievalMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// App post install attribution retrieval information. Disambiguates mechanism (iSDK or developer invoked) to retrieve payload from clicked link.
pub enum GetIosPostInstallAttributionRequestRetrievalMethodEnum {
    

    /// Unknown method.
    ///
    /// "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD"
    #[serde(rename="UNKNOWN_PAYLOAD_RETRIEVAL_METHOD")]
    UNKNOWNPAYLOADRETRIEVALMETHOD,
    

    /// iSDK performs a server lookup by device heuristics in the background when app is first-opened; no API called by developer.
    ///
    /// "IMPLICIT_WEAK_MATCH"
    #[serde(rename="IMPLICIT_WEAK_MATCH")]
    IMPLICITWEAKMATCH,
    

    /// iSDK performs a server lookup by device heuristics upon a dev API call.
    ///
    /// "EXPLICIT_WEAK_MATCH"
    #[serde(rename="EXPLICIT_WEAK_MATCH")]
    EXPLICITWEAKMATCH,
    

    /// iSDK performs a strong match only if weak match is found upon a dev API call.
    ///
    /// "EXPLICIT_STRONG_AFTER_WEAK_MATCH"
    #[serde(rename="EXPLICIT_STRONG_AFTER_WEAK_MATCH")]
    EXPLICITSTRONGAFTERWEAKMATCH,
}

impl AsRef<str> for GetIosPostInstallAttributionRequestRetrievalMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GetIosPostInstallAttributionRequestRetrievalMethodEnum::UNKNOWNPAYLOADRETRIEVALMETHOD => "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD",
            GetIosPostInstallAttributionRequestRetrievalMethodEnum::IMPLICITWEAKMATCH => "IMPLICIT_WEAK_MATCH",
            GetIosPostInstallAttributionRequestRetrievalMethodEnum::EXPLICITWEAKMATCH => "EXPLICIT_WEAK_MATCH",
            GetIosPostInstallAttributionRequestRetrievalMethodEnum::EXPLICITSTRONGAFTERWEAKMATCH => "EXPLICIT_STRONG_AFTER_WEAK_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GetIosPostInstallAttributionRequestRetrievalMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD" => Ok(GetIosPostInstallAttributionRequestRetrievalMethodEnum::UNKNOWNPAYLOADRETRIEVALMETHOD),
           "IMPLICIT_WEAK_MATCH" => Ok(GetIosPostInstallAttributionRequestRetrievalMethodEnum::IMPLICITWEAKMATCH),
           "EXPLICIT_WEAK_MATCH" => Ok(GetIosPostInstallAttributionRequestRetrievalMethodEnum::EXPLICITWEAKMATCH),
           "EXPLICIT_STRONG_AFTER_WEAK_MATCH" => Ok(GetIosPostInstallAttributionRequestRetrievalMethodEnum::EXPLICITSTRONGAFTERWEAKMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GetIosPostInstallAttributionRequestRetrievalMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GetIosPostInstallAttributionRequestVisualStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Strong match page information. Disambiguates between default UI and custom page to present when strong match succeeds/fails to find cookie.
pub enum GetIosPostInstallAttributionRequestVisualStyleEnum {
    

    /// Unknown style.
    ///
    /// "UNKNOWN_VISUAL_STYLE"
    #[serde(rename="UNKNOWN_VISUAL_STYLE")]
    UNKNOWNVISUALSTYLE,
    

    /// Default style.
    ///
    /// "DEFAULT_STYLE"
    #[serde(rename="DEFAULT_STYLE")]
    DEFAULTSTYLE,
    

    /// Custom style.
    ///
    /// "CUSTOM_STYLE"
    #[serde(rename="CUSTOM_STYLE")]
    CUSTOMSTYLE,
}

impl AsRef<str> for GetIosPostInstallAttributionRequestVisualStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GetIosPostInstallAttributionRequestVisualStyleEnum::UNKNOWNVISUALSTYLE => "UNKNOWN_VISUAL_STYLE",
            GetIosPostInstallAttributionRequestVisualStyleEnum::DEFAULTSTYLE => "DEFAULT_STYLE",
            GetIosPostInstallAttributionRequestVisualStyleEnum::CUSTOMSTYLE => "CUSTOM_STYLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GetIosPostInstallAttributionRequestVisualStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_VISUAL_STYLE" => Ok(GetIosPostInstallAttributionRequestVisualStyleEnum::UNKNOWNVISUALSTYLE),
           "DEFAULT_STYLE" => Ok(GetIosPostInstallAttributionRequestVisualStyleEnum::DEFAULTSTYLE),
           "CUSTOM_STYLE" => Ok(GetIosPostInstallAttributionRequestVisualStyleEnum::CUSTOMSTYLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GetIosPostInstallAttributionRequestVisualStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GetIosPostInstallAttributionResponseAttributionConfidenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The confidence of the returned attribution.
pub enum GetIosPostInstallAttributionResponseAttributionConfidenceEnum {
    

    /// Unset.
    ///
    /// "UNKNOWN_ATTRIBUTION_CONFIDENCE"
    #[serde(rename="UNKNOWN_ATTRIBUTION_CONFIDENCE")]
    UNKNOWNATTRIBUTIONCONFIDENCE,
    

    /// Weak confidence, more than one matching link found or link suspected to be false positive.
    ///
    /// "WEAK"
    #[serde(rename="WEAK")]
    WEAK,
    

    /// Default confidence, match based on device heuristics.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Unique confidence, match based on "unique match link to check" or other means.
    ///
    /// "UNIQUE"
    #[serde(rename="UNIQUE")]
    UNIQUE,
}

impl AsRef<str> for GetIosPostInstallAttributionResponseAttributionConfidenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GetIosPostInstallAttributionResponseAttributionConfidenceEnum::UNKNOWNATTRIBUTIONCONFIDENCE => "UNKNOWN_ATTRIBUTION_CONFIDENCE",
            GetIosPostInstallAttributionResponseAttributionConfidenceEnum::WEAK => "WEAK",
            GetIosPostInstallAttributionResponseAttributionConfidenceEnum::DEFAULT => "DEFAULT",
            GetIosPostInstallAttributionResponseAttributionConfidenceEnum::UNIQUE => "UNIQUE",
        }
    }
}

impl std::convert::TryFrom< &str> for GetIosPostInstallAttributionResponseAttributionConfidenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_ATTRIBUTION_CONFIDENCE" => Ok(GetIosPostInstallAttributionResponseAttributionConfidenceEnum::UNKNOWNATTRIBUTIONCONFIDENCE),
           "WEAK" => Ok(GetIosPostInstallAttributionResponseAttributionConfidenceEnum::WEAK),
           "DEFAULT" => Ok(GetIosPostInstallAttributionResponseAttributionConfidenceEnum::DEFAULT),
           "UNIQUE" => Ok(GetIosPostInstallAttributionResponseAttributionConfidenceEnum::UNIQUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GetIosPostInstallAttributionResponseAttributionConfidenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GetIosPostInstallAttributionResponseRequestIpVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which IP version the request was made from.
pub enum GetIosPostInstallAttributionResponseRequestIpVersionEnum {
    

    /// Unset.
    ///
    /// "UNKNOWN_IP_VERSION"
    #[serde(rename="UNKNOWN_IP_VERSION")]
    UNKNOWNIPVERSION,
    

    /// Request made from an IPv4 IP address.
    ///
    /// "IP_V4"
    #[serde(rename="IP_V4")]
    IPV4,
    

    /// Request made from an IPv6 IP address.
    ///
    /// "IP_V6"
    #[serde(rename="IP_V6")]
    IPV6,
}

impl AsRef<str> for GetIosPostInstallAttributionResponseRequestIpVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GetIosPostInstallAttributionResponseRequestIpVersionEnum::UNKNOWNIPVERSION => "UNKNOWN_IP_VERSION",
            GetIosPostInstallAttributionResponseRequestIpVersionEnum::IPV4 => "IP_V4",
            GetIosPostInstallAttributionResponseRequestIpVersionEnum::IPV6 => "IP_V6",
        }
    }
}

impl std::convert::TryFrom< &str> for GetIosPostInstallAttributionResponseRequestIpVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_IP_VERSION" => Ok(GetIosPostInstallAttributionResponseRequestIpVersionEnum::UNKNOWNIPVERSION),
           "IP_V4" => Ok(GetIosPostInstallAttributionResponseRequestIpVersionEnum::IPV4),
           "IP_V6" => Ok(GetIosPostInstallAttributionResponseRequestIpVersionEnum::IPV6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GetIosPostInstallAttributionResponseRequestIpVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedShortLinkFlaggedAttributeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Attributes that have been flagged about this short url.
pub enum ManagedShortLinkFlaggedAttributeEnum {
    

    /// Indicates that no attributes were found for this short url.
    ///
    /// "UNSPECIFIED_ATTRIBUTE"
    #[serde(rename="UNSPECIFIED_ATTRIBUTE")]
    UNSPECIFIEDATTRIBUTE,
    

    /// Indicates that short url has been flagged by AbuseIAm team as spam.
    ///
    /// "SPAM"
    #[serde(rename="SPAM")]
    SPAM,
}

impl AsRef<str> for ManagedShortLinkFlaggedAttributeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedShortLinkFlaggedAttributeEnum::UNSPECIFIEDATTRIBUTE => "UNSPECIFIED_ATTRIBUTE",
            ManagedShortLinkFlaggedAttributeEnum::SPAM => "SPAM",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedShortLinkFlaggedAttributeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_ATTRIBUTE" => Ok(ManagedShortLinkFlaggedAttributeEnum::UNSPECIFIEDATTRIBUTE),
           "SPAM" => Ok(ManagedShortLinkFlaggedAttributeEnum::SPAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedShortLinkFlaggedAttributeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedShortLinkVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Visibility status of link.
pub enum ManagedShortLinkVisibilityEnum {
    

    /// Visibility of the link is not specified.
    ///
    /// "UNSPECIFIED_VISIBILITY"
    #[serde(rename="UNSPECIFIED_VISIBILITY")]
    UNSPECIFIEDVISIBILITY,
    

    /// Link created in console and should be shown in console.
    ///
    /// "UNARCHIVED"
    #[serde(rename="UNARCHIVED")]
    UNARCHIVED,
    

    /// Link created in console and should not be shown in console (but can be shown in the console again if it is unarchived).
    ///
    /// "ARCHIVED"
    #[serde(rename="ARCHIVED")]
    ARCHIVED,
    

    /// Link created outside of console and should never be shown in console.
    ///
    /// "NEVER_SHOWN"
    #[serde(rename="NEVER_SHOWN")]
    NEVERSHOWN,
}

impl AsRef<str> for ManagedShortLinkVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedShortLinkVisibilityEnum::UNSPECIFIEDVISIBILITY => "UNSPECIFIED_VISIBILITY",
            ManagedShortLinkVisibilityEnum::UNARCHIVED => "UNARCHIVED",
            ManagedShortLinkVisibilityEnum::ARCHIVED => "ARCHIVED",
            ManagedShortLinkVisibilityEnum::NEVERSHOWN => "NEVER_SHOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedShortLinkVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_VISIBILITY" => Ok(ManagedShortLinkVisibilityEnum::UNSPECIFIEDVISIBILITY),
           "UNARCHIVED" => Ok(ManagedShortLinkVisibilityEnum::UNARCHIVED),
           "ARCHIVED" => Ok(ManagedShortLinkVisibilityEnum::ARCHIVED),
           "NEVER_SHOWN" => Ok(ManagedShortLinkVisibilityEnum::NEVERSHOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedShortLinkVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SuffixOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Suffix option.
pub enum SuffixOptionEnum {
    

    /// The suffix option is not specified, performs as UNGUESSABLE .
    ///
    /// "OPTION_UNSPECIFIED"
    #[serde(rename="OPTION_UNSPECIFIED")]
    OPTIONUNSPECIFIED,
    

    /// Short Dynamic Link suffix is a base62 [0-9A-Za-z] encoded string of a random generated 96 bit random number, which has a length of 17 chars. For example, "nlAR8U4SlKRZw1cb2". It prevents other people from guessing and crawling short Dynamic Links that contain personal identifiable information.
    ///
    /// "UNGUESSABLE"
    #[serde(rename="UNGUESSABLE")]
    UNGUESSABLE,
    

    /// Short Dynamic Link suffix is a base62 [0-9A-Za-z] string starting with a length of 4 chars. the length will increase when all the space is occupied.
    ///
    /// "SHORT"
    #[serde(rename="SHORT")]
    SHORT,
    

    /// Custom DDL suffix is a client specified string, for example, "buy2get1free". NOTE: custom suffix should only be available to managed short link creation
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for SuffixOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SuffixOptionEnum::OPTIONUNSPECIFIED => "OPTION_UNSPECIFIED",
            SuffixOptionEnum::UNGUESSABLE => "UNGUESSABLE",
            SuffixOptionEnum::SHORT => "SHORT",
            SuffixOptionEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for SuffixOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPTION_UNSPECIFIED" => Ok(SuffixOptionEnum::OPTIONUNSPECIFIED),
           "UNGUESSABLE" => Ok(SuffixOptionEnum::UNGUESSABLE),
           "SHORT" => Ok(SuffixOptionEnum::SHORT),
           "CUSTOM" => Ok(SuffixOptionEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SuffixOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


