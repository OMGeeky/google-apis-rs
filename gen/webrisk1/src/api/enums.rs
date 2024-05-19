use super::*;



// region GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of response. This may indicate that an action must be taken by the client when the response is received.
pub enum GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum {
    

    /// Unknown.
    ///
    /// "RESPONSE_TYPE_UNSPECIFIED"
    #[serde(rename="RESPONSE_TYPE_UNSPECIFIED")]
    RESPONSETYPEUNSPECIFIED,
    

    /// Partial updates are applied to the client's existing local database.
    ///
    /// "DIFF"
    #[serde(rename="DIFF")]
    DIFF,
    

    /// Full updates resets the client's entire local database. This means that either the client had no state, was seriously out-of-date, or the client is believed to be corrupt.
    ///
    /// "RESET"
    #[serde(rename="RESET")]
    RESET,
}

impl AsRef<str> for GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum::RESPONSETYPEUNSPECIFIED => "RESPONSE_TYPE_UNSPECIFIED",
            GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum::DIFF => "DIFF",
            GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum::RESET => "RESET",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_TYPE_UNSPECIFIED" => Ok(GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum::RESPONSETYPEUNSPECIFIED),
           "DIFF" => Ok(GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum::DIFF),
           "RESET" => Ok(GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum::RESET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ThreatList this threat belongs to. This must contain at least one entry.
pub enum GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum {
    

    /// No entries should match this threat type. This threat type is unused.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware targeting any platform.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software targeting any platform.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// A list of extended coverage social engineering URIs targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING_EXTENDED_COVERAGE"
    #[serde(rename="SOCIAL_ENGINEERING_EXTENDED_COVERAGE")]
    SOCIALENGINEERINGEXTENDEDCOVERAGE,
}

impl AsRef<str> for GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::MALWARE => "MALWARE",
            GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE => "SOCIAL_ENGINEERING_EXTENDED_COVERAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::UNWANTEDSOFTWARE),
           "SOCIAL_ENGINEERING_EXTENDED_COVERAGE" => Ok(GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ThreatList this threat belongs to.
pub enum GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum {
    

    /// No entries should match this threat type. This threat type is unused.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware targeting any platform.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software targeting any platform.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// A list of extended coverage social engineering URIs targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING_EXTENDED_COVERAGE"
    #[serde(rename="SOCIAL_ENGINEERING_EXTENDED_COVERAGE")]
    SOCIALENGINEERINGEXTENDEDCOVERAGE,
}

impl AsRef<str> for GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::MALWARE => "MALWARE",
            GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE => "SOCIAL_ENGINEERING_EXTENDED_COVERAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::UNWANTEDSOFTWARE),
           "SOCIAL_ENGINEERING_EXTENDED_COVERAGE" => Ok(GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HashThreatTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The ThreatLists to search in. Multiple ThreatLists may be specified.
pub enum HashThreatTypesEnum {
    

    /// No entries should match this threat type. This threat type is unused.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware targeting any platform.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software targeting any platform.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// A list of extended coverage social engineering URIs targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING_EXTENDED_COVERAGE"
    #[serde(rename="SOCIAL_ENGINEERING_EXTENDED_COVERAGE")]
    SOCIALENGINEERINGEXTENDEDCOVERAGE,
}

impl AsRef<str> for HashThreatTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HashThreatTypesEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            HashThreatTypesEnum::MALWARE => "MALWARE",
            HashThreatTypesEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            HashThreatTypesEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            HashThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE => "SOCIAL_ENGINEERING_EXTENDED_COVERAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for HashThreatTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(HashThreatTypesEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(HashThreatTypesEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(HashThreatTypesEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(HashThreatTypesEnum::UNWANTEDSOFTWARE),
           "SOCIAL_ENGINEERING_EXTENDED_COVERAGE" => Ok(HashThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HashThreatTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThreatListConstraintsSupportedCompressionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The compression types supported by the client.
pub enum ThreatListConstraintsSupportedCompressionsEnum {
    

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

impl AsRef<str> for ThreatListConstraintsSupportedCompressionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThreatListConstraintsSupportedCompressionsEnum::COMPRESSIONTYPEUNSPECIFIED => "COMPRESSION_TYPE_UNSPECIFIED",
            ThreatListConstraintsSupportedCompressionsEnum::RAW => "RAW",
            ThreatListConstraintsSupportedCompressionsEnum::RICE => "RICE",
        }
    }
}

impl std::convert::TryFrom< &str> for ThreatListConstraintsSupportedCompressionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPRESSION_TYPE_UNSPECIFIED" => Ok(ThreatListConstraintsSupportedCompressionsEnum::COMPRESSIONTYPEUNSPECIFIED),
           "RAW" => Ok(ThreatListConstraintsSupportedCompressionsEnum::RAW),
           "RICE" => Ok(ThreatListConstraintsSupportedCompressionsEnum::RICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThreatListConstraintsSupportedCompressionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThreatListThreatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The threat list to update. Only a single ThreatType should be specified per request. If you want to handle multiple ThreatTypes, you must make one request per ThreatType.
pub enum ThreatListThreatTypeEnum {
    

    /// No entries should match this threat type. This threat type is unused.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware targeting any platform.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software targeting any platform.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// A list of extended coverage social engineering URIs targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING_EXTENDED_COVERAGE"
    #[serde(rename="SOCIAL_ENGINEERING_EXTENDED_COVERAGE")]
    SOCIALENGINEERINGEXTENDEDCOVERAGE,
}

impl AsRef<str> for ThreatListThreatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThreatListThreatTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            ThreatListThreatTypeEnum::MALWARE => "MALWARE",
            ThreatListThreatTypeEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            ThreatListThreatTypeEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            ThreatListThreatTypeEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE => "SOCIAL_ENGINEERING_EXTENDED_COVERAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ThreatListThreatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(ThreatListThreatTypeEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(ThreatListThreatTypeEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(ThreatListThreatTypeEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(ThreatListThreatTypeEnum::UNWANTEDSOFTWARE),
           "SOCIAL_ENGINEERING_EXTENDED_COVERAGE" => Ok(ThreatListThreatTypeEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThreatListThreatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UriThreatTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The ThreatLists to search in. Multiple ThreatLists may be specified.
pub enum UriThreatTypesEnum {
    

    /// No entries should match this threat type. This threat type is unused.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Malware targeting any platform.
    ///
    /// "MALWARE"
    #[serde(rename="MALWARE")]
    MALWARE,
    

    /// Social engineering targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING"
    #[serde(rename="SOCIAL_ENGINEERING")]
    SOCIALENGINEERING,
    

    /// Unwanted software targeting any platform.
    ///
    /// "UNWANTED_SOFTWARE"
    #[serde(rename="UNWANTED_SOFTWARE")]
    UNWANTEDSOFTWARE,
    

    /// A list of extended coverage social engineering URIs targeting any platform.
    ///
    /// "SOCIAL_ENGINEERING_EXTENDED_COVERAGE"
    #[serde(rename="SOCIAL_ENGINEERING_EXTENDED_COVERAGE")]
    SOCIALENGINEERINGEXTENDEDCOVERAGE,
}

impl AsRef<str> for UriThreatTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UriThreatTypesEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            UriThreatTypesEnum::MALWARE => "MALWARE",
            UriThreatTypesEnum::SOCIALENGINEERING => "SOCIAL_ENGINEERING",
            UriThreatTypesEnum::UNWANTEDSOFTWARE => "UNWANTED_SOFTWARE",
            UriThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE => "SOCIAL_ENGINEERING_EXTENDED_COVERAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for UriThreatTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(UriThreatTypesEnum::THREATTYPEUNSPECIFIED),
           "MALWARE" => Ok(UriThreatTypesEnum::MALWARE),
           "SOCIAL_ENGINEERING" => Ok(UriThreatTypesEnum::SOCIALENGINEERING),
           "UNWANTED_SOFTWARE" => Ok(UriThreatTypesEnum::UNWANTEDSOFTWARE),
           "SOCIAL_ENGINEERING_EXTENDED_COVERAGE" => Ok(UriThreatTypesEnum::SOCIALENGINEERINGEXTENDEDCOVERAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UriThreatTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


