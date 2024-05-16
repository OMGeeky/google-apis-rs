use super::*;



// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of platform at risk by entries present in the list.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum {
    

    /// Unknown platform.
    ///
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Threat posed to Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Threat posed to Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Threat posed to Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Threat posed to OS X.
    ///
    /// "OSX"
    #[serde(rename="OSX")]
    OSX,
    

    /// Threat posed to iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Threat posed to at least one of the defined platforms.
    ///
    /// "ANY_PLATFORM"
    #[serde(rename="ANY_PLATFORM")]
    ANYPLATFORM,
    

    /// Threat posed to all defined platforms.
    ///
    /// "ALL_PLATFORMS"
    #[serde(rename="ALL_PLATFORMS")]
    ALLPLATFORMS,
    

    /// Threat posed to Chrome.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::WINDOWS => "WINDOWS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::LINUX => "LINUX",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::ANDROID => "ANDROID",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::OSX => "OSX",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::IOS => "IOS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::ANYPLATFORM => "ANY_PLATFORM",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::ALLPLATFORMS => "ALL_PLATFORMS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::CHROME => "CHROME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED),
           "WINDOWS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::WINDOWS),
           "LINUX" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::LINUX),
           "ANDROID" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::ANDROID),
           "OSX" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::OSX),
           "IOS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::IOS),
           "ANY_PLATFORM" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::ANYPLATFORM),
           "ALL_PLATFORMS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::ALLPLATFORMS),
           "CHROME" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum::CHROME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of entries present in the list.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum {
    

    /// Unspecified.
    ///
    /// "THREAT_ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_ENTRY_TYPE_UNSPECIFIED")]
    THREATENTRYTYPEUNSPECIFIED,
    

    /// A URL.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// An executable program.
    ///
    /// "EXECUTABLE"
    #[serde(rename="EXECUTABLE")]
    EXECUTABLE,
    

    /// An IP range.
    ///
    /// "IP_RANGE"
    #[serde(rename="IP_RANGE")]
    IPRANGE,
    

    /// Chrome extension.
    ///
    /// "CHROME_EXTENSION"
    #[serde(rename="CHROME_EXTENSION")]
    CHROMEEXTENSION,
    

    /// Filename.
    ///
    /// "FILENAME"
    #[serde(rename="FILENAME")]
    FILENAME,
    

    /// CERT
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED => "THREAT_ENTRY_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::URL => "URL",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::EXECUTABLE => "EXECUTABLE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::IPRANGE => "IP_RANGE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::CHROMEEXTENSION => "CHROME_EXTENSION",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::FILENAME => "FILENAME",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::CERT => "CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED),
           "URL" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::URL),
           "EXECUTABLE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::EXECUTABLE),
           "IP_RANGE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::IPRANGE),
           "CHROME_EXTENSION" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::CHROMEEXTENSION),
           "FILENAME" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::FILENAME),
           "CERT" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum::CERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of threat posed by entries present in the list.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum {
    

    /// Unknown.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware threat type.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering threat type.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software threat type.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// Potentially harmful application threat type.
    ///
    /// "POTENTIALLY_HARMFUL_APPLICATION"
    #[serde(rename="POTENTIALLY_HARMFUL_APPLICATION")]
    POTENTIALLYHARMFULAPPLICATION,
    

    /// Social engineering threat type for internal use.
    ///
    /// "SOCIAL_ENGINEERING_INTERNAL"
    #[serde(rename="SOCIAL_ENGINEERING_INTERNAL")]
    SOCIALENGINEERINGINTERNAL,
    

    /// API abuse threat type.
    ///
    /// "API_ABUSE"
    #[serde(rename="API_ABUSE")]
    APIABUSE,
    

    /// Malicious binary threat type.
    ///
    /// "MALICIOUS_BINARY"
    #[serde(rename="MALICIOUS_BINARY")]
    MALICIOUSBINARY,
    

    /// Client side detection whitelist threat type.
    ///
    /// "CSD_WHITELIST"
    #[serde(rename="CSD_WHITELIST")]
    CSDWHITELIST,
    

    /// Client side download detection whitelist threat type.
    ///
    /// "CSD_DOWNLOAD_WHITELIST"
    #[serde(rename="CSD_DOWNLOAD_WHITELIST")]
    CSDDOWNLOADWHITELIST,
    

    /// Client incident threat type.
    ///
    /// "CLIENT_INCIDENT"
    #[serde(rename="CLIENT_INCIDENT")]
    CLIENTINCIDENT,
    

    /// Whitelist used when detecting client incident threats.
    ///
    /// "CLIENT_INCIDENT_WHITELIST"
    #[serde(rename="CLIENT_INCIDENT_WHITELIST")]
    CLIENTINCIDENTWHITELIST,
    

    /// List used for offline APK checks in PAM.
    ///
    /// "APK_MALWARE_OFFLINE"
    #[serde(rename="APK_MALWARE_OFFLINE")]
    APKMALWAREOFFLINE,
    

    /// Patterns to be used for activating the subresource filter.
    ///
    /// "SUBRESOURCE_FILTER"
    #[serde(rename="SUBRESOURCE_FILTER")]
    SUBRESOURCEFILTER,
    

    /// Entities that are suspected to present a threat.
    ///
    /// "SUSPICIOUS"
    #[serde(rename="SUSPICIOUS")]
    SUSPICIOUS,
    

    /// Trick-to-bill threat type.
    ///
    /// "TRICK_TO_BILL"
    #[serde(rename="TRICK_TO_BILL")]
    TRICKTOBILL,
    

    /// URL expressions that are very likely to be safe.
    ///
    /// "HIGH_CONFIDENCE_ALLOWLIST"
    #[serde(rename="HIGH_CONFIDENCE_ALLOWLIST")]
    HIGHCONFIDENCEALLOWLIST,
    

    /// An experimental threat type related to Jigsaw. See https://jigsaw.google.com/.
    ///
    /// "ACCURACY_TIPS"
    #[serde(rename="ACCURACY_TIPS")]
    ACCURACYTIPS,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::MALWARE => "MALWARE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION => "POTENTIALLY_HARMFUL_APPLICATION",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SOCIALENGINEERINGINTERNAL => "SOCIAL_ENGINEERING_INTERNAL",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::APIABUSE => "API_ABUSE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::MALICIOUSBINARY => "MALICIOUS_BINARY",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CSDWHITELIST => "CSD_WHITELIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CSDDOWNLOADWHITELIST => "CSD_DOWNLOAD_WHITELIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CLIENTINCIDENT => "CLIENT_INCIDENT",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CLIENTINCIDENTWHITELIST => "CLIENT_INCIDENT_WHITELIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::APKMALWAREOFFLINE => "APK_MALWARE_OFFLINE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SUBRESOURCEFILTER => "SUBRESOURCE_FILTER",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SUSPICIOUS => "SUSPICIOUS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::TRICKTOBILL => "TRICK_TO_BILL",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::HIGHCONFIDENCEALLOWLIST => "HIGH_CONFIDENCE_ALLOWLIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::ACCURACYTIPS => "ACCURACY_TIPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::UNWANTEDSOFTWARE),
           "POTENTIALLY_HARMFUL_APPLICATION" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION),
           "SOCIAL_ENGINEERING_INTERNAL" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SOCIALENGINEERINGINTERNAL),
           "API_ABUSE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::APIABUSE),
           "MALICIOUS_BINARY" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::MALICIOUSBINARY),
           "CSD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CSDWHITELIST),
           "CSD_DOWNLOAD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CSDDOWNLOADWHITELIST),
           "CLIENT_INCIDENT" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CLIENTINCIDENT),
           "CLIENT_INCIDENT_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::CLIENTINCIDENTWHITELIST),
           "APK_MALWARE_OFFLINE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::APKMALWAREOFFLINE),
           "SUBRESOURCE_FILTER" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SUBRESOURCEFILTER),
           "SUSPICIOUS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::SUSPICIOUS),
           "TRICK_TO_BILL" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::TRICKTOBILL),
           "HIGH_CONFIDENCE_ALLOWLIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::HIGHCONFIDENCEALLOWLIST),
           "ACCURACY_TIPS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum::ACCURACYTIPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The compression types supported by the client.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum {
    

    /// Unknown.
    ///
    /// "COMPRESSION_TYPE_UNSPECIFIED"
    #[serde(rename="COMPRESSION_TYPE_UNSPECIFIED")]
    COMPRESSIONTYPEUNSPECIFIED,
    

    /// Raw, uncompressed data.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
    

    /// Rice-Golomb encoded data.
    ///
    /// "RICE"
    #[serde(rename="RICE")]
    RICE,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum::COMPRESSIONTYPEUNSPECIFIED => "COMPRESSION_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum::RAW => "RAW",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum::RICE => "RICE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPRESSION_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum::COMPRESSIONTYPEUNSPECIFIED),
           "RAW" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum::RAW),
           "RICE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum::RICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintSupportedCompressionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform type for which data is returned.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum {
    

    /// Unknown platform.
    ///
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Threat posed to Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Threat posed to Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Threat posed to Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Threat posed to OS X.
    ///
    /// "OSX"
    #[serde(rename="OSX")]
    OSX,
    

    /// Threat posed to iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Threat posed to at least one of the defined platforms.
    ///
    /// "ANY_PLATFORM"
    #[serde(rename="ANY_PLATFORM")]
    ANYPLATFORM,
    

    /// Threat posed to all defined platforms.
    ///
    /// "ALL_PLATFORMS"
    #[serde(rename="ALL_PLATFORMS")]
    ALLPLATFORMS,
    

    /// Threat posed to Chrome.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::WINDOWS => "WINDOWS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::LINUX => "LINUX",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::ANDROID => "ANDROID",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::OSX => "OSX",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::IOS => "IOS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::ANYPLATFORM => "ANY_PLATFORM",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::ALLPLATFORMS => "ALL_PLATFORMS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::CHROME => "CHROME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::PLATFORMTYPEUNSPECIFIED),
           "WINDOWS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::WINDOWS),
           "LINUX" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::LINUX),
           "ANDROID" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::ANDROID),
           "OSX" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::OSX),
           "IOS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::IOS),
           "ANY_PLATFORM" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::ANYPLATFORM),
           "ALL_PLATFORMS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::ALLPLATFORMS),
           "CHROME" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum::CHROME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of response. This may indicate that an action is required by the client when the response is received.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum {
    

    /// Unknown.
    ///
    /// "RESPONSE_TYPE_UNSPECIFIED"
    #[serde(rename="RESPONSE_TYPE_UNSPECIFIED")]
    RESPONSETYPEUNSPECIFIED,
    

    /// Partial updates are applied to the client's existing local database.
    ///
    /// "PARTIAL_UPDATE"
    #[serde(rename="PARTIAL_UPDATE")]
    PARTIALUPDATE,
    

    /// Full updates replace the client's entire local database. This means that either the client was seriously out-of-date or the client is believed to be corrupt.
    ///
    /// "FULL_UPDATE"
    #[serde(rename="FULL_UPDATE")]
    FULLUPDATE,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum::RESPONSETYPEUNSPECIFIED => "RESPONSE_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum::PARTIALUPDATE => "PARTIAL_UPDATE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum::FULLUPDATE => "FULL_UPDATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum::RESPONSETYPEUNSPECIFIED),
           "PARTIAL_UPDATE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum::PARTIALUPDATE),
           "FULL_UPDATE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum::FULLUPDATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of the threats.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum {
    

    /// Unspecified.
    ///
    /// "THREAT_ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_ENTRY_TYPE_UNSPECIFIED")]
    THREATENTRYTYPEUNSPECIFIED,
    

    /// A URL.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// An executable program.
    ///
    /// "EXECUTABLE"
    #[serde(rename="EXECUTABLE")]
    EXECUTABLE,
    

    /// An IP range.
    ///
    /// "IP_RANGE"
    #[serde(rename="IP_RANGE")]
    IPRANGE,
    

    /// Chrome extension.
    ///
    /// "CHROME_EXTENSION"
    #[serde(rename="CHROME_EXTENSION")]
    CHROMEEXTENSION,
    

    /// Filename.
    ///
    /// "FILENAME"
    #[serde(rename="FILENAME")]
    FILENAME,
    

    /// CERT
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED => "THREAT_ENTRY_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::URL => "URL",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::EXECUTABLE => "EXECUTABLE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::IPRANGE => "IP_RANGE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::CHROMEEXTENSION => "CHROME_EXTENSION",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::FILENAME => "FILENAME",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::CERT => "CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED),
           "URL" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::URL),
           "EXECUTABLE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::EXECUTABLE),
           "IP_RANGE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::IPRANGE),
           "CHROME_EXTENSION" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::CHROMEEXTENSION),
           "FILENAME" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::FILENAME),
           "CERT" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum::CERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The threat type for which data is returned.
pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum {
    

    /// Unknown.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware threat type.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering threat type.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software threat type.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// Potentially harmful application threat type.
    ///
    /// "POTENTIALLY_HARMFUL_APPLICATION"
    #[serde(rename="POTENTIALLY_HARMFUL_APPLICATION")]
    POTENTIALLYHARMFULAPPLICATION,
    

    /// Social engineering threat type for internal use.
    ///
    /// "SOCIAL_ENGINEERING_INTERNAL"
    #[serde(rename="SOCIAL_ENGINEERING_INTERNAL")]
    SOCIALENGINEERINGINTERNAL,
    

    /// API abuse threat type.
    ///
    /// "API_ABUSE"
    #[serde(rename="API_ABUSE")]
    APIABUSE,
    

    /// Malicious binary threat type.
    ///
    /// "MALICIOUS_BINARY"
    #[serde(rename="MALICIOUS_BINARY")]
    MALICIOUSBINARY,
    

    /// Client side detection whitelist threat type.
    ///
    /// "CSD_WHITELIST"
    #[serde(rename="CSD_WHITELIST")]
    CSDWHITELIST,
    

    /// Client side download detection whitelist threat type.
    ///
    /// "CSD_DOWNLOAD_WHITELIST"
    #[serde(rename="CSD_DOWNLOAD_WHITELIST")]
    CSDDOWNLOADWHITELIST,
    

    /// Client incident threat type.
    ///
    /// "CLIENT_INCIDENT"
    #[serde(rename="CLIENT_INCIDENT")]
    CLIENTINCIDENT,
    

    /// Whitelist used when detecting client incident threats.
    ///
    /// "CLIENT_INCIDENT_WHITELIST"
    #[serde(rename="CLIENT_INCIDENT_WHITELIST")]
    CLIENTINCIDENTWHITELIST,
    

    /// List used for offline APK checks in PAM.
    ///
    /// "APK_MALWARE_OFFLINE"
    #[serde(rename="APK_MALWARE_OFFLINE")]
    APKMALWAREOFFLINE,
    

    /// Patterns to be used for activating the subresource filter.
    ///
    /// "SUBRESOURCE_FILTER"
    #[serde(rename="SUBRESOURCE_FILTER")]
    SUBRESOURCEFILTER,
    

    /// Entities that are suspected to present a threat.
    ///
    /// "SUSPICIOUS"
    #[serde(rename="SUSPICIOUS")]
    SUSPICIOUS,
    

    /// Trick-to-bill threat type.
    ///
    /// "TRICK_TO_BILL"
    #[serde(rename="TRICK_TO_BILL")]
    TRICKTOBILL,
    

    /// URL expressions that are very likely to be safe.
    ///
    /// "HIGH_CONFIDENCE_ALLOWLIST"
    #[serde(rename="HIGH_CONFIDENCE_ALLOWLIST")]
    HIGHCONFIDENCEALLOWLIST,
    

    /// An experimental threat type related to Jigsaw. See https://jigsaw.google.com/.
    ///
    /// "ACCURACY_TIPS"
    #[serde(rename="ACCURACY_TIPS")]
    ACCURACYTIPS,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::MALWARE => "MALWARE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION => "POTENTIALLY_HARMFUL_APPLICATION",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SOCIALENGINEERINGINTERNAL => "SOCIAL_ENGINEERING_INTERNAL",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::APIABUSE => "API_ABUSE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::MALICIOUSBINARY => "MALICIOUS_BINARY",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CSDWHITELIST => "CSD_WHITELIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CSDDOWNLOADWHITELIST => "CSD_DOWNLOAD_WHITELIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CLIENTINCIDENT => "CLIENT_INCIDENT",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CLIENTINCIDENTWHITELIST => "CLIENT_INCIDENT_WHITELIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::APKMALWAREOFFLINE => "APK_MALWARE_OFFLINE",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SUBRESOURCEFILTER => "SUBRESOURCE_FILTER",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SUSPICIOUS => "SUSPICIOUS",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::TRICKTOBILL => "TRICK_TO_BILL",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::HIGHCONFIDENCEALLOWLIST => "HIGH_CONFIDENCE_ALLOWLIST",
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::ACCURACYTIPS => "ACCURACY_TIPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::UNWANTEDSOFTWARE),
           "POTENTIALLY_HARMFUL_APPLICATION" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION),
           "SOCIAL_ENGINEERING_INTERNAL" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SOCIALENGINEERINGINTERNAL),
           "API_ABUSE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::APIABUSE),
           "MALICIOUS_BINARY" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::MALICIOUSBINARY),
           "CSD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CSDWHITELIST),
           "CSD_DOWNLOAD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CSDDOWNLOADWHITELIST),
           "CLIENT_INCIDENT" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CLIENTINCIDENT),
           "CLIENT_INCIDENT_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::CLIENTINCIDENTWHITELIST),
           "APK_MALWARE_OFFLINE" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::APKMALWAREOFFLINE),
           "SUBRESOURCE_FILTER" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SUBRESOURCEFILTER),
           "SUSPICIOUS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::SUSPICIOUS),
           "TRICK_TO_BILL" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::TRICKTOBILL),
           "HIGH_CONFIDENCE_ALLOWLIST" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::HIGHCONFIDENCEALLOWLIST),
           "ACCURACY_TIPS" => Ok(GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum::ACCURACYTIPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The compression type for the entries in this set.
pub enum GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum {
    

    /// Unknown.
    ///
    /// "COMPRESSION_TYPE_UNSPECIFIED"
    #[serde(rename="COMPRESSION_TYPE_UNSPECIFIED")]
    COMPRESSIONTYPEUNSPECIFIED,
    

    /// Raw, uncompressed data.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
    

    /// Rice-Golomb encoded data.
    ///
    /// "RICE"
    #[serde(rename="RICE")]
    RICE,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum::COMPRESSIONTYPEUNSPECIFIED => "COMPRESSION_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum::RAW => "RAW",
            GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum::RICE => "RICE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPRESSION_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum::COMPRESSIONTYPEUNSPECIFIED),
           "RAW" => Ok(GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum::RAW),
           "RICE" => Ok(GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum::RICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform type reported.
pub enum GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum {
    

    /// Unknown platform.
    ///
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Threat posed to Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Threat posed to Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Threat posed to Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Threat posed to OS X.
    ///
    /// "OSX"
    #[serde(rename="OSX")]
    OSX,
    

    /// Threat posed to iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Threat posed to at least one of the defined platforms.
    ///
    /// "ANY_PLATFORM"
    #[serde(rename="ANY_PLATFORM")]
    ANYPLATFORM,
    

    /// Threat posed to all defined platforms.
    ///
    /// "ALL_PLATFORMS"
    #[serde(rename="ALL_PLATFORMS")]
    ALLPLATFORMS,
    

    /// Threat posed to Chrome.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::WINDOWS => "WINDOWS",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::LINUX => "LINUX",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::ANDROID => "ANDROID",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::OSX => "OSX",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::IOS => "IOS",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::ANYPLATFORM => "ANY_PLATFORM",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::ALLPLATFORMS => "ALL_PLATFORMS",
            GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::CHROME => "CHROME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED),
           "WINDOWS" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::WINDOWS),
           "LINUX" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::LINUX),
           "ANDROID" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::ANDROID),
           "OSX" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::OSX),
           "IOS" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::IOS),
           "ANY_PLATFORM" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::ANYPLATFORM),
           "ALL_PLATFORMS" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::ALLPLATFORMS),
           "CHROME" => Ok(GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum::CHROME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The threat type reported.
pub enum GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum {
    

    /// Unknown.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware threat type.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering threat type.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software threat type.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// Potentially harmful application threat type.
    ///
    /// "POTENTIALLY_HARMFUL_APPLICATION"
    #[serde(rename="POTENTIALLY_HARMFUL_APPLICATION")]
    POTENTIALLYHARMFULAPPLICATION,
    

    /// Social engineering threat type for internal use.
    ///
    /// "SOCIAL_ENGINEERING_INTERNAL"
    #[serde(rename="SOCIAL_ENGINEERING_INTERNAL")]
    SOCIALENGINEERINGINTERNAL,
    

    /// API abuse threat type.
    ///
    /// "API_ABUSE"
    #[serde(rename="API_ABUSE")]
    APIABUSE,
    

    /// Malicious binary threat type.
    ///
    /// "MALICIOUS_BINARY"
    #[serde(rename="MALICIOUS_BINARY")]
    MALICIOUSBINARY,
    

    /// Client side detection whitelist threat type.
    ///
    /// "CSD_WHITELIST"
    #[serde(rename="CSD_WHITELIST")]
    CSDWHITELIST,
    

    /// Client side download detection whitelist threat type.
    ///
    /// "CSD_DOWNLOAD_WHITELIST"
    #[serde(rename="CSD_DOWNLOAD_WHITELIST")]
    CSDDOWNLOADWHITELIST,
    

    /// Client incident threat type.
    ///
    /// "CLIENT_INCIDENT"
    #[serde(rename="CLIENT_INCIDENT")]
    CLIENTINCIDENT,
    

    /// Whitelist used when detecting client incident threats.
    ///
    /// "CLIENT_INCIDENT_WHITELIST"
    #[serde(rename="CLIENT_INCIDENT_WHITELIST")]
    CLIENTINCIDENTWHITELIST,
    

    /// List used for offline APK checks in PAM.
    ///
    /// "APK_MALWARE_OFFLINE"
    #[serde(rename="APK_MALWARE_OFFLINE")]
    APKMALWAREOFFLINE,
    

    /// Patterns to be used for activating the subresource filter.
    ///
    /// "SUBRESOURCE_FILTER"
    #[serde(rename="SUBRESOURCE_FILTER")]
    SUBRESOURCEFILTER,
    

    /// Entities that are suspected to present a threat.
    ///
    /// "SUSPICIOUS"
    #[serde(rename="SUSPICIOUS")]
    SUSPICIOUS,
    

    /// Trick-to-bill threat type.
    ///
    /// "TRICK_TO_BILL"
    #[serde(rename="TRICK_TO_BILL")]
    TRICKTOBILL,
    

    /// URL expressions that are very likely to be safe.
    ///
    /// "HIGH_CONFIDENCE_ALLOWLIST"
    #[serde(rename="HIGH_CONFIDENCE_ALLOWLIST")]
    HIGHCONFIDENCEALLOWLIST,
    

    /// An experimental threat type related to Jigsaw. See https://jigsaw.google.com/.
    ///
    /// "ACCURACY_TIPS"
    #[serde(rename="ACCURACY_TIPS")]
    ACCURACYTIPS,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::MALWARE => "MALWARE",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION => "POTENTIALLY_HARMFUL_APPLICATION",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SOCIALENGINEERINGINTERNAL => "SOCIAL_ENGINEERING_INTERNAL",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::APIABUSE => "API_ABUSE",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::MALICIOUSBINARY => "MALICIOUS_BINARY",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CSDWHITELIST => "CSD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CSDDOWNLOADWHITELIST => "CSD_DOWNLOAD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CLIENTINCIDENT => "CLIENT_INCIDENT",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CLIENTINCIDENTWHITELIST => "CLIENT_INCIDENT_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::APKMALWAREOFFLINE => "APK_MALWARE_OFFLINE",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SUBRESOURCEFILTER => "SUBRESOURCE_FILTER",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SUSPICIOUS => "SUSPICIOUS",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::TRICKTOBILL => "TRICK_TO_BILL",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::HIGHCONFIDENCEALLOWLIST => "HIGH_CONFIDENCE_ALLOWLIST",
            GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::ACCURACYTIPS => "ACCURACY_TIPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::UNWANTEDSOFTWARE),
           "POTENTIALLY_HARMFUL_APPLICATION" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION),
           "SOCIAL_ENGINEERING_INTERNAL" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SOCIALENGINEERINGINTERNAL),
           "API_ABUSE" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::APIABUSE),
           "MALICIOUS_BINARY" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::MALICIOUSBINARY),
           "CSD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CSDWHITELIST),
           "CSD_DOWNLOAD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CSDDOWNLOADWHITELIST),
           "CLIENT_INCIDENT" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CLIENTINCIDENT),
           "CLIENT_INCIDENT_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::CLIENTINCIDENTWHITELIST),
           "APK_MALWARE_OFFLINE" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::APKMALWAREOFFLINE),
           "SUBRESOURCE_FILTER" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SUBRESOURCEFILTER),
           "SUSPICIOUS" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::SUSPICIOUS),
           "TRICK_TO_BILL" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::TRICKTOBILL),
           "HIGH_CONFIDENCE_ALLOWLIST" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::HIGHCONFIDENCEALLOWLIST),
           "ACCURACY_TIPS" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum::ACCURACYTIPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of source reported.
pub enum GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum {
    

    /// Unknown.
    ///
    /// "THREAT_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_SOURCE_TYPE_UNSPECIFIED")]
    THREATSOURCETYPEUNSPECIFIED,
    

    /// The URL that matched the threat list (for which GetFullHash returned a valid hash).
    ///
    /// "MATCHING_URL"
    #[serde(rename="MATCHING_URL")]
    MATCHINGURL,
    

    /// The final top-level URL of the tab that the client was browsing when the match occurred.
    ///
    /// "TAB_URL"
    #[serde(rename="TAB_URL")]
    TABURL,
    

    /// A redirect URL that was fetched before hitting the final TAB_URL.
    ///
    /// "TAB_REDIRECT"
    #[serde(rename="TAB_REDIRECT")]
    TABREDIRECT,
    

    /// A resource loaded within the final TAB_URL.
    ///
    /// "TAB_RESOURCE"
    #[serde(rename="TAB_RESOURCE")]
    TABRESOURCE,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::THREATSOURCETYPEUNSPECIFIED => "THREAT_SOURCE_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::MATCHINGURL => "MATCHING_URL",
            GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::TABURL => "TAB_URL",
            GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::TABREDIRECT => "TAB_REDIRECT",
            GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::TABRESOURCE => "TAB_RESOURCE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_SOURCE_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::THREATSOURCETYPEUNSPECIFIED),
           "MATCHING_URL" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::MATCHINGURL),
           "TAB_URL" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::TABURL),
           "TAB_REDIRECT" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::TABREDIRECT),
           "TAB_RESOURCE" => Ok(GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum::TABRESOURCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform types to be checked.
pub enum GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum {
    

    /// Unknown platform.
    ///
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Threat posed to Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Threat posed to Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Threat posed to Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Threat posed to OS X.
    ///
    /// "OSX"
    #[serde(rename="OSX")]
    OSX,
    

    /// Threat posed to iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Threat posed to at least one of the defined platforms.
    ///
    /// "ANY_PLATFORM"
    #[serde(rename="ANY_PLATFORM")]
    ANYPLATFORM,
    

    /// Threat posed to all defined platforms.
    ///
    /// "ALL_PLATFORMS"
    #[serde(rename="ALL_PLATFORMS")]
    ALLPLATFORMS,
    

    /// Threat posed to Chrome.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::WINDOWS => "WINDOWS",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::LINUX => "LINUX",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::ANDROID => "ANDROID",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::OSX => "OSX",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::IOS => "IOS",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::ANYPLATFORM => "ANY_PLATFORM",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::ALLPLATFORMS => "ALL_PLATFORMS",
            GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::CHROME => "CHROME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::PLATFORMTYPEUNSPECIFIED),
           "WINDOWS" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::WINDOWS),
           "LINUX" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::LINUX),
           "ANDROID" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::ANDROID),
           "OSX" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::OSX),
           "IOS" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::IOS),
           "ANY_PLATFORM" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::ANYPLATFORM),
           "ALL_PLATFORMS" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::ALLPLATFORMS),
           "CHROME" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum::CHROME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entry types to be checked.
pub enum GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum {
    

    /// Unspecified.
    ///
    /// "THREAT_ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_ENTRY_TYPE_UNSPECIFIED")]
    THREATENTRYTYPEUNSPECIFIED,
    

    /// A URL.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// An executable program.
    ///
    /// "EXECUTABLE"
    #[serde(rename="EXECUTABLE")]
    EXECUTABLE,
    

    /// An IP range.
    ///
    /// "IP_RANGE"
    #[serde(rename="IP_RANGE")]
    IPRANGE,
    

    /// Chrome extension.
    ///
    /// "CHROME_EXTENSION"
    #[serde(rename="CHROME_EXTENSION")]
    CHROMEEXTENSION,
    

    /// Filename.
    ///
    /// "FILENAME"
    #[serde(rename="FILENAME")]
    FILENAME,
    

    /// CERT
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::THREATENTRYTYPEUNSPECIFIED => "THREAT_ENTRY_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::URL => "URL",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::EXECUTABLE => "EXECUTABLE",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::IPRANGE => "IP_RANGE",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::CHROMEEXTENSION => "CHROME_EXTENSION",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::FILENAME => "FILENAME",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::CERT => "CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::THREATENTRYTYPEUNSPECIFIED),
           "URL" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::URL),
           "EXECUTABLE" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::EXECUTABLE),
           "IP_RANGE" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::IPRANGE),
           "CHROME_EXTENSION" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::CHROMEEXTENSION),
           "FILENAME" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::FILENAME),
           "CERT" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum::CERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The threat types to be checked.
pub enum GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum {
    

    /// Unknown.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware threat type.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering threat type.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software threat type.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// Potentially harmful application threat type.
    ///
    /// "POTENTIALLY_HARMFUL_APPLICATION"
    #[serde(rename="POTENTIALLY_HARMFUL_APPLICATION")]
    POTENTIALLYHARMFULAPPLICATION,
    

    /// Social engineering threat type for internal use.
    ///
    /// "SOCIAL_ENGINEERING_INTERNAL"
    #[serde(rename="SOCIAL_ENGINEERING_INTERNAL")]
    SOCIALENGINEERINGINTERNAL,
    

    /// API abuse threat type.
    ///
    /// "API_ABUSE"
    #[serde(rename="API_ABUSE")]
    APIABUSE,
    

    /// Malicious binary threat type.
    ///
    /// "MALICIOUS_BINARY"
    #[serde(rename="MALICIOUS_BINARY")]
    MALICIOUSBINARY,
    

    /// Client side detection whitelist threat type.
    ///
    /// "CSD_WHITELIST"
    #[serde(rename="CSD_WHITELIST")]
    CSDWHITELIST,
    

    /// Client side download detection whitelist threat type.
    ///
    /// "CSD_DOWNLOAD_WHITELIST"
    #[serde(rename="CSD_DOWNLOAD_WHITELIST")]
    CSDDOWNLOADWHITELIST,
    

    /// Client incident threat type.
    ///
    /// "CLIENT_INCIDENT"
    #[serde(rename="CLIENT_INCIDENT")]
    CLIENTINCIDENT,
    

    /// Whitelist used when detecting client incident threats.
    ///
    /// "CLIENT_INCIDENT_WHITELIST"
    #[serde(rename="CLIENT_INCIDENT_WHITELIST")]
    CLIENTINCIDENTWHITELIST,
    

    /// List used for offline APK checks in PAM.
    ///
    /// "APK_MALWARE_OFFLINE"
    #[serde(rename="APK_MALWARE_OFFLINE")]
    APKMALWAREOFFLINE,
    

    /// Patterns to be used for activating the subresource filter.
    ///
    /// "SUBRESOURCE_FILTER"
    #[serde(rename="SUBRESOURCE_FILTER")]
    SUBRESOURCEFILTER,
    

    /// Entities that are suspected to present a threat.
    ///
    /// "SUSPICIOUS"
    #[serde(rename="SUSPICIOUS")]
    SUSPICIOUS,
    

    /// Trick-to-bill threat type.
    ///
    /// "TRICK_TO_BILL"
    #[serde(rename="TRICK_TO_BILL")]
    TRICKTOBILL,
    

    /// URL expressions that are very likely to be safe.
    ///
    /// "HIGH_CONFIDENCE_ALLOWLIST"
    #[serde(rename="HIGH_CONFIDENCE_ALLOWLIST")]
    HIGHCONFIDENCEALLOWLIST,
    

    /// An experimental threat type related to Jigsaw. See https://jigsaw.google.com/.
    ///
    /// "ACCURACY_TIPS"
    #[serde(rename="ACCURACY_TIPS")]
    ACCURACYTIPS,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::MALWARE => "MALWARE",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::POTENTIALLYHARMFULAPPLICATION => "POTENTIALLY_HARMFUL_APPLICATION",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SOCIALENGINEERINGINTERNAL => "SOCIAL_ENGINEERING_INTERNAL",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::APIABUSE => "API_ABUSE",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::MALICIOUSBINARY => "MALICIOUS_BINARY",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CSDWHITELIST => "CSD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CSDDOWNLOADWHITELIST => "CSD_DOWNLOAD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CLIENTINCIDENT => "CLIENT_INCIDENT",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CLIENTINCIDENTWHITELIST => "CLIENT_INCIDENT_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::APKMALWAREOFFLINE => "APK_MALWARE_OFFLINE",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SUBRESOURCEFILTER => "SUBRESOURCE_FILTER",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SUSPICIOUS => "SUSPICIOUS",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::TRICKTOBILL => "TRICK_TO_BILL",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::HIGHCONFIDENCEALLOWLIST => "HIGH_CONFIDENCE_ALLOWLIST",
            GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::ACCURACYTIPS => "ACCURACY_TIPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::UNWANTEDSOFTWARE),
           "POTENTIALLY_HARMFUL_APPLICATION" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::POTENTIALLYHARMFULAPPLICATION),
           "SOCIAL_ENGINEERING_INTERNAL" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SOCIALENGINEERINGINTERNAL),
           "API_ABUSE" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::APIABUSE),
           "MALICIOUS_BINARY" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::MALICIOUSBINARY),
           "CSD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CSDWHITELIST),
           "CSD_DOWNLOAD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CSDDOWNLOADWHITELIST),
           "CLIENT_INCIDENT" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CLIENTINCIDENT),
           "CLIENT_INCIDENT_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::CLIENTINCIDENTWHITELIST),
           "APK_MALWARE_OFFLINE" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::APKMALWAREOFFLINE),
           "SUBRESOURCE_FILTER" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SUBRESOURCEFILTER),
           "SUSPICIOUS" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::SUSPICIOUS),
           "TRICK_TO_BILL" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::TRICKTOBILL),
           "HIGH_CONFIDENCE_ALLOWLIST" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::HIGHCONFIDENCEALLOWLIST),
           "ACCURACY_TIPS" => Ok(GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum::ACCURACYTIPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform type targeted by the list's entries.
pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum {
    

    /// Unknown platform.
    ///
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Threat posed to Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Threat posed to Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Threat posed to Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Threat posed to OS X.
    ///
    /// "OSX"
    #[serde(rename="OSX")]
    OSX,
    

    /// Threat posed to iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Threat posed to at least one of the defined platforms.
    ///
    /// "ANY_PLATFORM"
    #[serde(rename="ANY_PLATFORM")]
    ANYPLATFORM,
    

    /// Threat posed to all defined platforms.
    ///
    /// "ALL_PLATFORMS"
    #[serde(rename="ALL_PLATFORMS")]
    ALLPLATFORMS,
    

    /// Threat posed to Chrome.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::WINDOWS => "WINDOWS",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::LINUX => "LINUX",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::ANDROID => "ANDROID",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::OSX => "OSX",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::IOS => "IOS",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::ANYPLATFORM => "ANY_PLATFORM",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::ALLPLATFORMS => "ALL_PLATFORMS",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::CHROME => "CHROME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED),
           "WINDOWS" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::WINDOWS),
           "LINUX" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::LINUX),
           "ANDROID" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::ANDROID),
           "OSX" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::OSX),
           "IOS" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::IOS),
           "ANY_PLATFORM" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::ANYPLATFORM),
           "ALL_PLATFORMS" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::ALLPLATFORMS),
           "CHROME" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum::CHROME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entry types contained in the list.
pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum {
    

    /// Unspecified.
    ///
    /// "THREAT_ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_ENTRY_TYPE_UNSPECIFIED")]
    THREATENTRYTYPEUNSPECIFIED,
    

    /// A URL.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// An executable program.
    ///
    /// "EXECUTABLE"
    #[serde(rename="EXECUTABLE")]
    EXECUTABLE,
    

    /// An IP range.
    ///
    /// "IP_RANGE"
    #[serde(rename="IP_RANGE")]
    IPRANGE,
    

    /// Chrome extension.
    ///
    /// "CHROME_EXTENSION"
    #[serde(rename="CHROME_EXTENSION")]
    CHROMEEXTENSION,
    

    /// Filename.
    ///
    /// "FILENAME"
    #[serde(rename="FILENAME")]
    FILENAME,
    

    /// CERT
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED => "THREAT_ENTRY_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::URL => "URL",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::EXECUTABLE => "EXECUTABLE",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::IPRANGE => "IP_RANGE",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::CHROMEEXTENSION => "CHROME_EXTENSION",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::FILENAME => "FILENAME",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::CERT => "CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED),
           "URL" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::URL),
           "EXECUTABLE" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::EXECUTABLE),
           "IP_RANGE" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::IPRANGE),
           "CHROME_EXTENSION" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::CHROMEEXTENSION),
           "FILENAME" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::FILENAME),
           "CERT" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum::CERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The threat type posed by the list's entries.
pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum {
    

    /// Unknown.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware threat type.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering threat type.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software threat type.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// Potentially harmful application threat type.
    ///
    /// "POTENTIALLY_HARMFUL_APPLICATION"
    #[serde(rename="POTENTIALLY_HARMFUL_APPLICATION")]
    POTENTIALLYHARMFULAPPLICATION,
    

    /// Social engineering threat type for internal use.
    ///
    /// "SOCIAL_ENGINEERING_INTERNAL"
    #[serde(rename="SOCIAL_ENGINEERING_INTERNAL")]
    SOCIALENGINEERINGINTERNAL,
    

    /// API abuse threat type.
    ///
    /// "API_ABUSE"
    #[serde(rename="API_ABUSE")]
    APIABUSE,
    

    /// Malicious binary threat type.
    ///
    /// "MALICIOUS_BINARY"
    #[serde(rename="MALICIOUS_BINARY")]
    MALICIOUSBINARY,
    

    /// Client side detection whitelist threat type.
    ///
    /// "CSD_WHITELIST"
    #[serde(rename="CSD_WHITELIST")]
    CSDWHITELIST,
    

    /// Client side download detection whitelist threat type.
    ///
    /// "CSD_DOWNLOAD_WHITELIST"
    #[serde(rename="CSD_DOWNLOAD_WHITELIST")]
    CSDDOWNLOADWHITELIST,
    

    /// Client incident threat type.
    ///
    /// "CLIENT_INCIDENT"
    #[serde(rename="CLIENT_INCIDENT")]
    CLIENTINCIDENT,
    

    /// Whitelist used when detecting client incident threats.
    ///
    /// "CLIENT_INCIDENT_WHITELIST"
    #[serde(rename="CLIENT_INCIDENT_WHITELIST")]
    CLIENTINCIDENTWHITELIST,
    

    /// List used for offline APK checks in PAM.
    ///
    /// "APK_MALWARE_OFFLINE"
    #[serde(rename="APK_MALWARE_OFFLINE")]
    APKMALWAREOFFLINE,
    

    /// Patterns to be used for activating the subresource filter.
    ///
    /// "SUBRESOURCE_FILTER"
    #[serde(rename="SUBRESOURCE_FILTER")]
    SUBRESOURCEFILTER,
    

    /// Entities that are suspected to present a threat.
    ///
    /// "SUSPICIOUS"
    #[serde(rename="SUSPICIOUS")]
    SUSPICIOUS,
    

    /// Trick-to-bill threat type.
    ///
    /// "TRICK_TO_BILL"
    #[serde(rename="TRICK_TO_BILL")]
    TRICKTOBILL,
    

    /// URL expressions that are very likely to be safe.
    ///
    /// "HIGH_CONFIDENCE_ALLOWLIST"
    #[serde(rename="HIGH_CONFIDENCE_ALLOWLIST")]
    HIGHCONFIDENCEALLOWLIST,
    

    /// An experimental threat type related to Jigsaw. See https://jigsaw.google.com/.
    ///
    /// "ACCURACY_TIPS"
    #[serde(rename="ACCURACY_TIPS")]
    ACCURACYTIPS,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::MALWARE => "MALWARE",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION => "POTENTIALLY_HARMFUL_APPLICATION",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SOCIALENGINEERINGINTERNAL => "SOCIAL_ENGINEERING_INTERNAL",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::APIABUSE => "API_ABUSE",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::MALICIOUSBINARY => "MALICIOUS_BINARY",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CSDWHITELIST => "CSD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CSDDOWNLOADWHITELIST => "CSD_DOWNLOAD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CLIENTINCIDENT => "CLIENT_INCIDENT",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CLIENTINCIDENTWHITELIST => "CLIENT_INCIDENT_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::APKMALWAREOFFLINE => "APK_MALWARE_OFFLINE",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SUBRESOURCEFILTER => "SUBRESOURCE_FILTER",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SUSPICIOUS => "SUSPICIOUS",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::TRICKTOBILL => "TRICK_TO_BILL",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::HIGHCONFIDENCEALLOWLIST => "HIGH_CONFIDENCE_ALLOWLIST",
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::ACCURACYTIPS => "ACCURACY_TIPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::UNWANTEDSOFTWARE),
           "POTENTIALLY_HARMFUL_APPLICATION" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION),
           "SOCIAL_ENGINEERING_INTERNAL" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SOCIALENGINEERINGINTERNAL),
           "API_ABUSE" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::APIABUSE),
           "MALICIOUS_BINARY" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::MALICIOUSBINARY),
           "CSD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CSDWHITELIST),
           "CSD_DOWNLOAD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CSDDOWNLOADWHITELIST),
           "CLIENT_INCIDENT" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CLIENTINCIDENT),
           "CLIENT_INCIDENT_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::CLIENTINCIDENTWHITELIST),
           "APK_MALWARE_OFFLINE" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::APKMALWAREOFFLINE),
           "SUBRESOURCE_FILTER" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SUBRESOURCEFILTER),
           "SUSPICIOUS" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::SUSPICIOUS),
           "TRICK_TO_BILL" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::TRICKTOBILL),
           "HIGH_CONFIDENCE_ALLOWLIST" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::HIGHCONFIDENCEALLOWLIST),
           "ACCURACY_TIPS" => Ok(GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum::ACCURACYTIPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform type matching this threat.
pub enum GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum {
    

    /// Unknown platform.
    ///
    /// "PLATFORM_TYPE_UNSPECIFIED"
    #[serde(rename="PLATFORM_TYPE_UNSPECIFIED")]
    PLATFORMTYPEUNSPECIFIED,
    

    /// Threat posed to Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Threat posed to Linux.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Threat posed to Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Threat posed to OS X.
    ///
    /// "OSX"
    #[serde(rename="OSX")]
    OSX,
    

    /// Threat posed to iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Threat posed to at least one of the defined platforms.
    ///
    /// "ANY_PLATFORM"
    #[serde(rename="ANY_PLATFORM")]
    ANYPLATFORM,
    

    /// Threat posed to all defined platforms.
    ///
    /// "ALL_PLATFORMS"
    #[serde(rename="ALL_PLATFORMS")]
    ALLPLATFORMS,
    

    /// Threat posed to Chrome.
    ///
    /// "CHROME"
    #[serde(rename="CHROME")]
    CHROME,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED => "PLATFORM_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::WINDOWS => "WINDOWS",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::LINUX => "LINUX",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::ANDROID => "ANDROID",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::OSX => "OSX",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::IOS => "IOS",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::ANYPLATFORM => "ANY_PLATFORM",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::ALLPLATFORMS => "ALL_PLATFORMS",
            GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::CHROME => "CHROME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::PLATFORMTYPEUNSPECIFIED),
           "WINDOWS" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::WINDOWS),
           "LINUX" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::LINUX),
           "ANDROID" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::ANDROID),
           "OSX" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::OSX),
           "IOS" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::IOS),
           "ANY_PLATFORM" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::ANYPLATFORM),
           "ALL_PLATFORMS" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::ALLPLATFORMS),
           "CHROME" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum::CHROME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The threat entry type matching this threat.
pub enum GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum {
    

    /// Unspecified.
    ///
    /// "THREAT_ENTRY_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_ENTRY_TYPE_UNSPECIFIED")]
    THREATENTRYTYPEUNSPECIFIED,
    

    /// A URL.
    ///
    /// "URL"
    #[serde(rename="URL")]
    URL,
    

    /// An executable program.
    ///
    /// "EXECUTABLE"
    #[serde(rename="EXECUTABLE")]
    EXECUTABLE,
    

    /// An IP range.
    ///
    /// "IP_RANGE"
    #[serde(rename="IP_RANGE")]
    IPRANGE,
    

    /// Chrome extension.
    ///
    /// "CHROME_EXTENSION"
    #[serde(rename="CHROME_EXTENSION")]
    CHROMEEXTENSION,
    

    /// Filename.
    ///
    /// "FILENAME"
    #[serde(rename="FILENAME")]
    FILENAME,
    

    /// CERT
    ///
    /// "CERT"
    #[serde(rename="CERT")]
    CERT,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED => "THREAT_ENTRY_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::URL => "URL",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::EXECUTABLE => "EXECUTABLE",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::IPRANGE => "IP_RANGE",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::CHROMEEXTENSION => "CHROME_EXTENSION",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::FILENAME => "FILENAME",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::CERT => "CERT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ENTRY_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::THREATENTRYTYPEUNSPECIFIED),
           "URL" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::URL),
           "EXECUTABLE" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::EXECUTABLE),
           "IP_RANGE" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::IPRANGE),
           "CHROME_EXTENSION" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::CHROMEEXTENSION),
           "FILENAME" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::FILENAME),
           "CERT" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum::CERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The threat type matching this threat.
pub enum GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum {
    

    /// Unknown.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware threat type.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering threat type.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software threat type.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// Potentially harmful application threat type.
    ///
    /// "POTENTIALLY_HARMFUL_APPLICATION"
    #[serde(rename="POTENTIALLY_HARMFUL_APPLICATION")]
    POTENTIALLYHARMFULAPPLICATION,
    

    /// Social engineering threat type for internal use.
    ///
    /// "SOCIAL_ENGINEERING_INTERNAL"
    #[serde(rename="SOCIAL_ENGINEERING_INTERNAL")]
    SOCIALENGINEERINGINTERNAL,
    

    /// API abuse threat type.
    ///
    /// "API_ABUSE"
    #[serde(rename="API_ABUSE")]
    APIABUSE,
    

    /// Malicious binary threat type.
    ///
    /// "MALICIOUS_BINARY"
    #[serde(rename="MALICIOUS_BINARY")]
    MALICIOUSBINARY,
    

    /// Client side detection whitelist threat type.
    ///
    /// "CSD_WHITELIST"
    #[serde(rename="CSD_WHITELIST")]
    CSDWHITELIST,
    

    /// Client side download detection whitelist threat type.
    ///
    /// "CSD_DOWNLOAD_WHITELIST"
    #[serde(rename="CSD_DOWNLOAD_WHITELIST")]
    CSDDOWNLOADWHITELIST,
    

    /// Client incident threat type.
    ///
    /// "CLIENT_INCIDENT"
    #[serde(rename="CLIENT_INCIDENT")]
    CLIENTINCIDENT,
    

    /// Whitelist used when detecting client incident threats.
    ///
    /// "CLIENT_INCIDENT_WHITELIST"
    #[serde(rename="CLIENT_INCIDENT_WHITELIST")]
    CLIENTINCIDENTWHITELIST,
    

    /// List used for offline APK checks in PAM.
    ///
    /// "APK_MALWARE_OFFLINE"
    #[serde(rename="APK_MALWARE_OFFLINE")]
    APKMALWAREOFFLINE,
    

    /// Patterns to be used for activating the subresource filter.
    ///
    /// "SUBRESOURCE_FILTER"
    #[serde(rename="SUBRESOURCE_FILTER")]
    SUBRESOURCEFILTER,
    

    /// Entities that are suspected to present a threat.
    ///
    /// "SUSPICIOUS"
    #[serde(rename="SUSPICIOUS")]
    SUSPICIOUS,
    

    /// Trick-to-bill threat type.
    ///
    /// "TRICK_TO_BILL"
    #[serde(rename="TRICK_TO_BILL")]
    TRICKTOBILL,
    

    /// URL expressions that are very likely to be safe.
    ///
    /// "HIGH_CONFIDENCE_ALLOWLIST"
    #[serde(rename="HIGH_CONFIDENCE_ALLOWLIST")]
    HIGHCONFIDENCEALLOWLIST,
    

    /// An experimental threat type related to Jigsaw. See https://jigsaw.google.com/.
    ///
    /// "ACCURACY_TIPS"
    #[serde(rename="ACCURACY_TIPS")]
    ACCURACYTIPS,
}

impl AsRef<str> for GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::MALWARE => "MALWARE",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION => "POTENTIALLY_HARMFUL_APPLICATION",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SOCIALENGINEERINGINTERNAL => "SOCIAL_ENGINEERING_INTERNAL",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::APIABUSE => "API_ABUSE",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::MALICIOUSBINARY => "MALICIOUS_BINARY",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CSDWHITELIST => "CSD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CSDDOWNLOADWHITELIST => "CSD_DOWNLOAD_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CLIENTINCIDENT => "CLIENT_INCIDENT",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CLIENTINCIDENTWHITELIST => "CLIENT_INCIDENT_WHITELIST",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::APKMALWAREOFFLINE => "APK_MALWARE_OFFLINE",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SUBRESOURCEFILTER => "SUBRESOURCE_FILTER",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SUSPICIOUS => "SUSPICIOUS",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::TRICKTOBILL => "TRICK_TO_BILL",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::HIGHCONFIDENCEALLOWLIST => "HIGH_CONFIDENCE_ALLOWLIST",
            GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::ACCURACYTIPS => "ACCURACY_TIPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::UNWANTEDSOFTWARE),
           "POTENTIALLY_HARMFUL_APPLICATION" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::POTENTIALLYHARMFULAPPLICATION),
           "SOCIAL_ENGINEERING_INTERNAL" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SOCIALENGINEERINGINTERNAL),
           "API_ABUSE" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::APIABUSE),
           "MALICIOUS_BINARY" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::MALICIOUSBINARY),
           "CSD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CSDWHITELIST),
           "CSD_DOWNLOAD_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CSDDOWNLOADWHITELIST),
           "CLIENT_INCIDENT" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CLIENTINCIDENT),
           "CLIENT_INCIDENT_WHITELIST" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::CLIENTINCIDENTWHITELIST),
           "APK_MALWARE_OFFLINE" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::APKMALWAREOFFLINE),
           "SUBRESOURCE_FILTER" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SUBRESOURCEFILTER),
           "SUSPICIOUS" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::SUSPICIOUS),
           "TRICK_TO_BILL" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::TRICKTOBILL),
           "HIGH_CONFIDENCE_ALLOWLIST" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::HIGHCONFIDENCEALLOWLIST),
           "ACCURACY_TIPS" => Ok(GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum::ACCURACYTIPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


