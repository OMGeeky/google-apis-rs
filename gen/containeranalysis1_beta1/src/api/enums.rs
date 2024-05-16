use super::*;



// region AliasContextKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The alias kind.
pub enum AliasContextKindEnum {
    

    /// Unknown.
    ///
    /// "KIND_UNSPECIFIED"
    #[serde(rename="KIND_UNSPECIFIED")]
    KINDUNSPECIFIED,
    

    /// Git tag.
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// Git branch.
    ///
    /// "MOVABLE"
    #[serde(rename="MOVABLE")]
    MOVABLE,
    

    /// Used to specify non-standard aliases. For example, if a Git repo has a ref named "refs/foo/bar".
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for AliasContextKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AliasContextKindEnum::KINDUNSPECIFIED => "KIND_UNSPECIFIED",
            AliasContextKindEnum::FIXED => "FIXED",
            AliasContextKindEnum::MOVABLE => "MOVABLE",
            AliasContextKindEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for AliasContextKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KIND_UNSPECIFIED" => Ok(AliasContextKindEnum::KINDUNSPECIFIED),
           "FIXED" => Ok(AliasContextKindEnum::FIXED),
           "MOVABLE" => Ok(AliasContextKindEnum::MOVABLE),
           "OTHER" => Ok(AliasContextKindEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AliasContextKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssessmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Provides the state of this Vulnerability assessment.
pub enum AssessmentStateEnum {
    

    /// No state is specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// This product is known to be affected by this vulnerability.
    ///
    /// "AFFECTED"
    #[serde(rename="AFFECTED")]
    AFFECTED,
    

    /// This product is known to be not affected by this vulnerability.
    ///
    /// "NOT_AFFECTED"
    #[serde(rename="NOT_AFFECTED")]
    NOTAFFECTED,
    

    /// This product contains a fix for this vulnerability.
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// It is not known yet whether these versions are or are not affected by the vulnerability. However, it is still under investigation.
    ///
    /// "UNDER_INVESTIGATION"
    #[serde(rename="UNDER_INVESTIGATION")]
    UNDERINVESTIGATION,
}

impl AsRef<str> for AssessmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssessmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AssessmentStateEnum::AFFECTED => "AFFECTED",
            AssessmentStateEnum::NOTAFFECTED => "NOT_AFFECTED",
            AssessmentStateEnum::FIXED => "FIXED",
            AssessmentStateEnum::UNDERINVESTIGATION => "UNDER_INVESTIGATION",
        }
    }
}

impl std::convert::TryFrom< &str> for AssessmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AssessmentStateEnum::STATEUNSPECIFIED),
           "AFFECTED" => Ok(AssessmentStateEnum::AFFECTED),
           "NOT_AFFECTED" => Ok(AssessmentStateEnum::NOTAFFECTED),
           "FIXED" => Ok(AssessmentStateEnum::FIXED),
           "UNDER_INVESTIGATION" => Ok(AssessmentStateEnum::UNDERINVESTIGATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssessmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildSignatureKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the key, either stored in `public_key` or referenced in `key_id`.
pub enum BuildSignatureKeyTypeEnum {
    

    /// `KeyType` is not set.
    ///
    /// "KEY_TYPE_UNSPECIFIED"
    #[serde(rename="KEY_TYPE_UNSPECIFIED")]
    KEYTYPEUNSPECIFIED,
    

    /// `PGP ASCII Armored` public key.
    ///
    /// "PGP_ASCII_ARMORED"
    #[serde(rename="PGP_ASCII_ARMORED")]
    PGPASCIIARMORED,
    

    /// `PKIX PEM` public key.
    ///
    /// "PKIX_PEM"
    #[serde(rename="PKIX_PEM")]
    PKIXPEM,
}

impl AsRef<str> for BuildSignatureKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildSignatureKeyTypeEnum::KEYTYPEUNSPECIFIED => "KEY_TYPE_UNSPECIFIED",
            BuildSignatureKeyTypeEnum::PGPASCIIARMORED => "PGP_ASCII_ARMORED",
            BuildSignatureKeyTypeEnum::PKIXPEM => "PKIX_PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildSignatureKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_TYPE_UNSPECIFIED" => Ok(BuildSignatureKeyTypeEnum::KEYTYPEUNSPECIFIED),
           "PGP_ASCII_ARMORED" => Ok(BuildSignatureKeyTypeEnum::PGPASCIIARMORED),
           "PKIX_PEM" => Ok(BuildSignatureKeyTypeEnum::PKIXPEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildSignatureKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSAttackComplexityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3, CVSS v2
pub enum CVSAttackComplexityEnum {
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_COMPLEXITY_UNSPECIFIED"
    #[serde(rename="ATTACK_COMPLEXITY_UNSPECIFIED")]
    ATTACKCOMPLEXITYUNSPECIFIED,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_COMPLEXITY_LOW"
    #[serde(rename="ATTACK_COMPLEXITY_LOW")]
    ATTACKCOMPLEXITYLOW,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_COMPLEXITY_HIGH"
    #[serde(rename="ATTACK_COMPLEXITY_HIGH")]
    ATTACKCOMPLEXITYHIGH,
    

    /// Defined in CVSS v2
    ///
    /// "ATTACK_COMPLEXITY_MEDIUM"
    #[serde(rename="ATTACK_COMPLEXITY_MEDIUM")]
    ATTACKCOMPLEXITYMEDIUM,
}

impl AsRef<str> for CVSAttackComplexityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSAttackComplexityEnum::ATTACKCOMPLEXITYUNSPECIFIED => "ATTACK_COMPLEXITY_UNSPECIFIED",
            CVSAttackComplexityEnum::ATTACKCOMPLEXITYLOW => "ATTACK_COMPLEXITY_LOW",
            CVSAttackComplexityEnum::ATTACKCOMPLEXITYHIGH => "ATTACK_COMPLEXITY_HIGH",
            CVSAttackComplexityEnum::ATTACKCOMPLEXITYMEDIUM => "ATTACK_COMPLEXITY_MEDIUM",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSAttackComplexityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTACK_COMPLEXITY_UNSPECIFIED" => Ok(CVSAttackComplexityEnum::ATTACKCOMPLEXITYUNSPECIFIED),
           "ATTACK_COMPLEXITY_LOW" => Ok(CVSAttackComplexityEnum::ATTACKCOMPLEXITYLOW),
           "ATTACK_COMPLEXITY_HIGH" => Ok(CVSAttackComplexityEnum::ATTACKCOMPLEXITYHIGH),
           "ATTACK_COMPLEXITY_MEDIUM" => Ok(CVSAttackComplexityEnum::ATTACKCOMPLEXITYMEDIUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSAttackComplexityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSAttackVectorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments. Defined in CVSS v3, CVSS v2
pub enum CVSAttackVectorEnum {
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_VECTOR_UNSPECIFIED"
    #[serde(rename="ATTACK_VECTOR_UNSPECIFIED")]
    ATTACKVECTORUNSPECIFIED,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_VECTOR_NETWORK"
    #[serde(rename="ATTACK_VECTOR_NETWORK")]
    ATTACKVECTORNETWORK,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_VECTOR_ADJACENT"
    #[serde(rename="ATTACK_VECTOR_ADJACENT")]
    ATTACKVECTORADJACENT,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "ATTACK_VECTOR_LOCAL"
    #[serde(rename="ATTACK_VECTOR_LOCAL")]
    ATTACKVECTORLOCAL,
    

    /// Defined in CVSS v3
    ///
    /// "ATTACK_VECTOR_PHYSICAL"
    #[serde(rename="ATTACK_VECTOR_PHYSICAL")]
    ATTACKVECTORPHYSICAL,
}

impl AsRef<str> for CVSAttackVectorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSAttackVectorEnum::ATTACKVECTORUNSPECIFIED => "ATTACK_VECTOR_UNSPECIFIED",
            CVSAttackVectorEnum::ATTACKVECTORNETWORK => "ATTACK_VECTOR_NETWORK",
            CVSAttackVectorEnum::ATTACKVECTORADJACENT => "ATTACK_VECTOR_ADJACENT",
            CVSAttackVectorEnum::ATTACKVECTORLOCAL => "ATTACK_VECTOR_LOCAL",
            CVSAttackVectorEnum::ATTACKVECTORPHYSICAL => "ATTACK_VECTOR_PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSAttackVectorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTACK_VECTOR_UNSPECIFIED" => Ok(CVSAttackVectorEnum::ATTACKVECTORUNSPECIFIED),
           "ATTACK_VECTOR_NETWORK" => Ok(CVSAttackVectorEnum::ATTACKVECTORNETWORK),
           "ATTACK_VECTOR_ADJACENT" => Ok(CVSAttackVectorEnum::ATTACKVECTORADJACENT),
           "ATTACK_VECTOR_LOCAL" => Ok(CVSAttackVectorEnum::ATTACKVECTORLOCAL),
           "ATTACK_VECTOR_PHYSICAL" => Ok(CVSAttackVectorEnum::ATTACKVECTORPHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSAttackVectorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSAuthenticationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v2
pub enum CVSAuthenticationEnum {
    

    /// Defined in CVSS v2
    ///
    /// "AUTHENTICATION_UNSPECIFIED"
    #[serde(rename="AUTHENTICATION_UNSPECIFIED")]
    AUTHENTICATIONUNSPECIFIED,
    

    /// Defined in CVSS v2
    ///
    /// "AUTHENTICATION_MULTIPLE"
    #[serde(rename="AUTHENTICATION_MULTIPLE")]
    AUTHENTICATIONMULTIPLE,
    

    /// Defined in CVSS v2
    ///
    /// "AUTHENTICATION_SINGLE"
    #[serde(rename="AUTHENTICATION_SINGLE")]
    AUTHENTICATIONSINGLE,
    

    /// Defined in CVSS v2
    ///
    /// "AUTHENTICATION_NONE"
    #[serde(rename="AUTHENTICATION_NONE")]
    AUTHENTICATIONNONE,
}

impl AsRef<str> for CVSAuthenticationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSAuthenticationEnum::AUTHENTICATIONUNSPECIFIED => "AUTHENTICATION_UNSPECIFIED",
            CVSAuthenticationEnum::AUTHENTICATIONMULTIPLE => "AUTHENTICATION_MULTIPLE",
            CVSAuthenticationEnum::AUTHENTICATIONSINGLE => "AUTHENTICATION_SINGLE",
            CVSAuthenticationEnum::AUTHENTICATIONNONE => "AUTHENTICATION_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSAuthenticationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHENTICATION_UNSPECIFIED" => Ok(CVSAuthenticationEnum::AUTHENTICATIONUNSPECIFIED),
           "AUTHENTICATION_MULTIPLE" => Ok(CVSAuthenticationEnum::AUTHENTICATIONMULTIPLE),
           "AUTHENTICATION_SINGLE" => Ok(CVSAuthenticationEnum::AUTHENTICATIONSINGLE),
           "AUTHENTICATION_NONE" => Ok(CVSAuthenticationEnum::AUTHENTICATIONNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSAuthenticationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSAvailabilityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3, CVSS v2
pub enum CVSAvailabilityImpactEnum {
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    

    /// Defined in CVSS v3
    ///
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    

    /// Defined in CVSS v3
    ///
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
    

    /// Defined in CVSS v2
    ///
    /// "IMPACT_PARTIAL"
    #[serde(rename="IMPACT_PARTIAL")]
    IMPACTPARTIAL,
    

    /// Defined in CVSS v2
    ///
    /// "IMPACT_COMPLETE"
    #[serde(rename="IMPACT_COMPLETE")]
    IMPACTCOMPLETE,
}

impl AsRef<str> for CVSAvailabilityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSAvailabilityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            CVSAvailabilityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            CVSAvailabilityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            CVSAvailabilityImpactEnum::IMPACTNONE => "IMPACT_NONE",
            CVSAvailabilityImpactEnum::IMPACTPARTIAL => "IMPACT_PARTIAL",
            CVSAvailabilityImpactEnum::IMPACTCOMPLETE => "IMPACT_COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSAvailabilityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(CVSAvailabilityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(CVSAvailabilityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(CVSAvailabilityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(CVSAvailabilityImpactEnum::IMPACTNONE),
           "IMPACT_PARTIAL" => Ok(CVSAvailabilityImpactEnum::IMPACTPARTIAL),
           "IMPACT_COMPLETE" => Ok(CVSAvailabilityImpactEnum::IMPACTCOMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSAvailabilityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSConfidentialityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3, CVSS v2
pub enum CVSConfidentialityImpactEnum {
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    

    /// Defined in CVSS v3
    ///
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    

    /// Defined in CVSS v3
    ///
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
    

    /// Defined in CVSS v2
    ///
    /// "IMPACT_PARTIAL"
    #[serde(rename="IMPACT_PARTIAL")]
    IMPACTPARTIAL,
    

    /// Defined in CVSS v2
    ///
    /// "IMPACT_COMPLETE"
    #[serde(rename="IMPACT_COMPLETE")]
    IMPACTCOMPLETE,
}

impl AsRef<str> for CVSConfidentialityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSConfidentialityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            CVSConfidentialityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            CVSConfidentialityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            CVSConfidentialityImpactEnum::IMPACTNONE => "IMPACT_NONE",
            CVSConfidentialityImpactEnum::IMPACTPARTIAL => "IMPACT_PARTIAL",
            CVSConfidentialityImpactEnum::IMPACTCOMPLETE => "IMPACT_COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSConfidentialityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(CVSConfidentialityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(CVSConfidentialityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(CVSConfidentialityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(CVSConfidentialityImpactEnum::IMPACTNONE),
           "IMPACT_PARTIAL" => Ok(CVSConfidentialityImpactEnum::IMPACTPARTIAL),
           "IMPACT_COMPLETE" => Ok(CVSConfidentialityImpactEnum::IMPACTCOMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSConfidentialityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSIntegrityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3, CVSS v2
pub enum CVSIntegrityImpactEnum {
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    

    /// Defined in CVSS v3
    ///
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    

    /// Defined in CVSS v3
    ///
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    

    /// Defined in CVSS v3, CVSS v2
    ///
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
    

    /// Defined in CVSS v2
    ///
    /// "IMPACT_PARTIAL"
    #[serde(rename="IMPACT_PARTIAL")]
    IMPACTPARTIAL,
    

    /// Defined in CVSS v2
    ///
    /// "IMPACT_COMPLETE"
    #[serde(rename="IMPACT_COMPLETE")]
    IMPACTCOMPLETE,
}

impl AsRef<str> for CVSIntegrityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSIntegrityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            CVSIntegrityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            CVSIntegrityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            CVSIntegrityImpactEnum::IMPACTNONE => "IMPACT_NONE",
            CVSIntegrityImpactEnum::IMPACTPARTIAL => "IMPACT_PARTIAL",
            CVSIntegrityImpactEnum::IMPACTCOMPLETE => "IMPACT_COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSIntegrityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(CVSIntegrityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(CVSIntegrityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(CVSIntegrityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(CVSIntegrityImpactEnum::IMPACTNONE),
           "IMPACT_PARTIAL" => Ok(CVSIntegrityImpactEnum::IMPACTPARTIAL),
           "IMPACT_COMPLETE" => Ok(CVSIntegrityImpactEnum::IMPACTCOMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSIntegrityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSPrivilegesRequiredEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3
pub enum CVSPrivilegesRequiredEnum {
    

    /// Defined in CVSS v3
    ///
    /// "PRIVILEGES_REQUIRED_UNSPECIFIED"
    #[serde(rename="PRIVILEGES_REQUIRED_UNSPECIFIED")]
    PRIVILEGESREQUIREDUNSPECIFIED,
    

    /// Defined in CVSS v3
    ///
    /// "PRIVILEGES_REQUIRED_NONE"
    #[serde(rename="PRIVILEGES_REQUIRED_NONE")]
    PRIVILEGESREQUIREDNONE,
    

    /// Defined in CVSS v3
    ///
    /// "PRIVILEGES_REQUIRED_LOW"
    #[serde(rename="PRIVILEGES_REQUIRED_LOW")]
    PRIVILEGESREQUIREDLOW,
    

    /// Defined in CVSS v3
    ///
    /// "PRIVILEGES_REQUIRED_HIGH"
    #[serde(rename="PRIVILEGES_REQUIRED_HIGH")]
    PRIVILEGESREQUIREDHIGH,
}

impl AsRef<str> for CVSPrivilegesRequiredEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDUNSPECIFIED => "PRIVILEGES_REQUIRED_UNSPECIFIED",
            CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDNONE => "PRIVILEGES_REQUIRED_NONE",
            CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDLOW => "PRIVILEGES_REQUIRED_LOW",
            CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDHIGH => "PRIVILEGES_REQUIRED_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSPrivilegesRequiredEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIVILEGES_REQUIRED_UNSPECIFIED" => Ok(CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDUNSPECIFIED),
           "PRIVILEGES_REQUIRED_NONE" => Ok(CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDNONE),
           "PRIVILEGES_REQUIRED_LOW" => Ok(CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDLOW),
           "PRIVILEGES_REQUIRED_HIGH" => Ok(CVSPrivilegesRequiredEnum::PRIVILEGESREQUIREDHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSPrivilegesRequiredEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3
pub enum CVSScopeEnum {
    

    /// Defined in CVSS v3
    ///
    /// "SCOPE_UNSPECIFIED"
    #[serde(rename="SCOPE_UNSPECIFIED")]
    SCOPEUNSPECIFIED,
    

    /// Defined in CVSS v3
    ///
    /// "SCOPE_UNCHANGED"
    #[serde(rename="SCOPE_UNCHANGED")]
    SCOPEUNCHANGED,
    

    /// Defined in CVSS v3
    ///
    /// "SCOPE_CHANGED"
    #[serde(rename="SCOPE_CHANGED")]
    SCOPECHANGED,
}

impl AsRef<str> for CVSScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSScopeEnum::SCOPEUNSPECIFIED => "SCOPE_UNSPECIFIED",
            CVSScopeEnum::SCOPEUNCHANGED => "SCOPE_UNCHANGED",
            CVSScopeEnum::SCOPECHANGED => "SCOPE_CHANGED",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCOPE_UNSPECIFIED" => Ok(CVSScopeEnum::SCOPEUNSPECIFIED),
           "SCOPE_UNCHANGED" => Ok(CVSScopeEnum::SCOPEUNCHANGED),
           "SCOPE_CHANGED" => Ok(CVSScopeEnum::SCOPECHANGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSUserInteractionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defined in CVSS v3
pub enum CVSUserInteractionEnum {
    

    /// Defined in CVSS v3
    ///
    /// "USER_INTERACTION_UNSPECIFIED"
    #[serde(rename="USER_INTERACTION_UNSPECIFIED")]
    USERINTERACTIONUNSPECIFIED,
    

    /// Defined in CVSS v3
    ///
    /// "USER_INTERACTION_NONE"
    #[serde(rename="USER_INTERACTION_NONE")]
    USERINTERACTIONNONE,
    

    /// Defined in CVSS v3
    ///
    /// "USER_INTERACTION_REQUIRED"
    #[serde(rename="USER_INTERACTION_REQUIRED")]
    USERINTERACTIONREQUIRED,
}

impl AsRef<str> for CVSUserInteractionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSUserInteractionEnum::USERINTERACTIONUNSPECIFIED => "USER_INTERACTION_UNSPECIFIED",
            CVSUserInteractionEnum::USERINTERACTIONNONE => "USER_INTERACTION_NONE",
            CVSUserInteractionEnum::USERINTERACTIONREQUIRED => "USER_INTERACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSUserInteractionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_INTERACTION_UNSPECIFIED" => Ok(CVSUserInteractionEnum::USERINTERACTIONUNSPECIFIED),
           "USER_INTERACTION_NONE" => Ok(CVSUserInteractionEnum::USERINTERACTIONNONE),
           "USER_INTERACTION_REQUIRED" => Ok(CVSUserInteractionEnum::USERINTERACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSUserInteractionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3AttackComplexityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3AttackComplexityEnum {
    
    /// "ATTACK_COMPLEXITY_UNSPECIFIED"
    #[serde(rename="ATTACK_COMPLEXITY_UNSPECIFIED")]
    ATTACKCOMPLEXITYUNSPECIFIED,
    
    /// "ATTACK_COMPLEXITY_LOW"
    #[serde(rename="ATTACK_COMPLEXITY_LOW")]
    ATTACKCOMPLEXITYLOW,
    
    /// "ATTACK_COMPLEXITY_HIGH"
    #[serde(rename="ATTACK_COMPLEXITY_HIGH")]
    ATTACKCOMPLEXITYHIGH,
}

impl AsRef<str> for CVSSv3AttackComplexityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3AttackComplexityEnum::ATTACKCOMPLEXITYUNSPECIFIED => "ATTACK_COMPLEXITY_UNSPECIFIED",
            CVSSv3AttackComplexityEnum::ATTACKCOMPLEXITYLOW => "ATTACK_COMPLEXITY_LOW",
            CVSSv3AttackComplexityEnum::ATTACKCOMPLEXITYHIGH => "ATTACK_COMPLEXITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3AttackComplexityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTACK_COMPLEXITY_UNSPECIFIED" => Ok(CVSSv3AttackComplexityEnum::ATTACKCOMPLEXITYUNSPECIFIED),
           "ATTACK_COMPLEXITY_LOW" => Ok(CVSSv3AttackComplexityEnum::ATTACKCOMPLEXITYLOW),
           "ATTACK_COMPLEXITY_HIGH" => Ok(CVSSv3AttackComplexityEnum::ATTACKCOMPLEXITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3AttackComplexityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3AttackVectorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments.
pub enum CVSSv3AttackVectorEnum {
    
    /// "ATTACK_VECTOR_UNSPECIFIED"
    #[serde(rename="ATTACK_VECTOR_UNSPECIFIED")]
    ATTACKVECTORUNSPECIFIED,
    
    /// "ATTACK_VECTOR_NETWORK"
    #[serde(rename="ATTACK_VECTOR_NETWORK")]
    ATTACKVECTORNETWORK,
    
    /// "ATTACK_VECTOR_ADJACENT"
    #[serde(rename="ATTACK_VECTOR_ADJACENT")]
    ATTACKVECTORADJACENT,
    
    /// "ATTACK_VECTOR_LOCAL"
    #[serde(rename="ATTACK_VECTOR_LOCAL")]
    ATTACKVECTORLOCAL,
    
    /// "ATTACK_VECTOR_PHYSICAL"
    #[serde(rename="ATTACK_VECTOR_PHYSICAL")]
    ATTACKVECTORPHYSICAL,
}

impl AsRef<str> for CVSSv3AttackVectorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3AttackVectorEnum::ATTACKVECTORUNSPECIFIED => "ATTACK_VECTOR_UNSPECIFIED",
            CVSSv3AttackVectorEnum::ATTACKVECTORNETWORK => "ATTACK_VECTOR_NETWORK",
            CVSSv3AttackVectorEnum::ATTACKVECTORADJACENT => "ATTACK_VECTOR_ADJACENT",
            CVSSv3AttackVectorEnum::ATTACKVECTORLOCAL => "ATTACK_VECTOR_LOCAL",
            CVSSv3AttackVectorEnum::ATTACKVECTORPHYSICAL => "ATTACK_VECTOR_PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3AttackVectorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTACK_VECTOR_UNSPECIFIED" => Ok(CVSSv3AttackVectorEnum::ATTACKVECTORUNSPECIFIED),
           "ATTACK_VECTOR_NETWORK" => Ok(CVSSv3AttackVectorEnum::ATTACKVECTORNETWORK),
           "ATTACK_VECTOR_ADJACENT" => Ok(CVSSv3AttackVectorEnum::ATTACKVECTORADJACENT),
           "ATTACK_VECTOR_LOCAL" => Ok(CVSSv3AttackVectorEnum::ATTACKVECTORLOCAL),
           "ATTACK_VECTOR_PHYSICAL" => Ok(CVSSv3AttackVectorEnum::ATTACKVECTORPHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3AttackVectorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3AvailabilityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3AvailabilityImpactEnum {
    
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
}

impl AsRef<str> for CVSSv3AvailabilityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3AvailabilityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            CVSSv3AvailabilityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            CVSSv3AvailabilityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            CVSSv3AvailabilityImpactEnum::IMPACTNONE => "IMPACT_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3AvailabilityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(CVSSv3AvailabilityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(CVSSv3AvailabilityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(CVSSv3AvailabilityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(CVSSv3AvailabilityImpactEnum::IMPACTNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3AvailabilityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3ConfidentialityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3ConfidentialityImpactEnum {
    
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
}

impl AsRef<str> for CVSSv3ConfidentialityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3ConfidentialityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            CVSSv3ConfidentialityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            CVSSv3ConfidentialityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            CVSSv3ConfidentialityImpactEnum::IMPACTNONE => "IMPACT_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3ConfidentialityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(CVSSv3ConfidentialityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(CVSSv3ConfidentialityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(CVSSv3ConfidentialityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(CVSSv3ConfidentialityImpactEnum::IMPACTNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3ConfidentialityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3IntegrityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3IntegrityImpactEnum {
    
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
}

impl AsRef<str> for CVSSv3IntegrityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3IntegrityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            CVSSv3IntegrityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            CVSSv3IntegrityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            CVSSv3IntegrityImpactEnum::IMPACTNONE => "IMPACT_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3IntegrityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(CVSSv3IntegrityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(CVSSv3IntegrityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(CVSSv3IntegrityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(CVSSv3IntegrityImpactEnum::IMPACTNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3IntegrityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3PrivilegesRequiredEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3PrivilegesRequiredEnum {
    
    /// "PRIVILEGES_REQUIRED_UNSPECIFIED"
    #[serde(rename="PRIVILEGES_REQUIRED_UNSPECIFIED")]
    PRIVILEGESREQUIREDUNSPECIFIED,
    
    /// "PRIVILEGES_REQUIRED_NONE"
    #[serde(rename="PRIVILEGES_REQUIRED_NONE")]
    PRIVILEGESREQUIREDNONE,
    
    /// "PRIVILEGES_REQUIRED_LOW"
    #[serde(rename="PRIVILEGES_REQUIRED_LOW")]
    PRIVILEGESREQUIREDLOW,
    
    /// "PRIVILEGES_REQUIRED_HIGH"
    #[serde(rename="PRIVILEGES_REQUIRED_HIGH")]
    PRIVILEGESREQUIREDHIGH,
}

impl AsRef<str> for CVSSv3PrivilegesRequiredEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDUNSPECIFIED => "PRIVILEGES_REQUIRED_UNSPECIFIED",
            CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDNONE => "PRIVILEGES_REQUIRED_NONE",
            CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDLOW => "PRIVILEGES_REQUIRED_LOW",
            CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDHIGH => "PRIVILEGES_REQUIRED_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3PrivilegesRequiredEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIVILEGES_REQUIRED_UNSPECIFIED" => Ok(CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDUNSPECIFIED),
           "PRIVILEGES_REQUIRED_NONE" => Ok(CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDNONE),
           "PRIVILEGES_REQUIRED_LOW" => Ok(CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDLOW),
           "PRIVILEGES_REQUIRED_HIGH" => Ok(CVSSv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3PrivilegesRequiredEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3ScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3ScopeEnum {
    
    /// "SCOPE_UNSPECIFIED"
    #[serde(rename="SCOPE_UNSPECIFIED")]
    SCOPEUNSPECIFIED,
    
    /// "SCOPE_UNCHANGED"
    #[serde(rename="SCOPE_UNCHANGED")]
    SCOPEUNCHANGED,
    
    /// "SCOPE_CHANGED"
    #[serde(rename="SCOPE_CHANGED")]
    SCOPECHANGED,
}

impl AsRef<str> for CVSSv3ScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3ScopeEnum::SCOPEUNSPECIFIED => "SCOPE_UNSPECIFIED",
            CVSSv3ScopeEnum::SCOPEUNCHANGED => "SCOPE_UNCHANGED",
            CVSSv3ScopeEnum::SCOPECHANGED => "SCOPE_CHANGED",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3ScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCOPE_UNSPECIFIED" => Ok(CVSSv3ScopeEnum::SCOPEUNSPECIFIED),
           "SCOPE_UNCHANGED" => Ok(CVSSv3ScopeEnum::SCOPEUNCHANGED),
           "SCOPE_CHANGED" => Ok(CVSSv3ScopeEnum::SCOPECHANGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3ScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CVSSv3UserInteractionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSSv3UserInteractionEnum {
    
    /// "USER_INTERACTION_UNSPECIFIED"
    #[serde(rename="USER_INTERACTION_UNSPECIFIED")]
    USERINTERACTIONUNSPECIFIED,
    
    /// "USER_INTERACTION_NONE"
    #[serde(rename="USER_INTERACTION_NONE")]
    USERINTERACTIONNONE,
    
    /// "USER_INTERACTION_REQUIRED"
    #[serde(rename="USER_INTERACTION_REQUIRED")]
    USERINTERACTIONREQUIRED,
}

impl AsRef<str> for CVSSv3UserInteractionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CVSSv3UserInteractionEnum::USERINTERACTIONUNSPECIFIED => "USER_INTERACTION_UNSPECIFIED",
            CVSSv3UserInteractionEnum::USERINTERACTIONNONE => "USER_INTERACTION_NONE",
            CVSSv3UserInteractionEnum::USERINTERACTIONREQUIRED => "USER_INTERACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for CVSSv3UserInteractionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_INTERACTION_UNSPECIFIED" => Ok(CVSSv3UserInteractionEnum::USERINTERACTIONUNSPECIFIED),
           "USER_INTERACTION_NONE" => Ok(CVSSv3UserInteractionEnum::USERINTERACTIONNONE),
           "USER_INTERACTION_REQUIRED" => Ok(CVSSv3UserInteractionEnum::USERINTERACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CVSSv3UserInteractionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeploymentPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Platform hosting this deployment.
pub enum DeploymentPlatformEnum {
    

    /// Unknown.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// Google Container Engine.
    ///
    /// "GKE"
    #[serde(rename="GKE")]
    GKE,
    

    /// Google App Engine: Flexible Environment.
    ///
    /// "FLEX"
    #[serde(rename="FLEX")]
    FLEX,
    

    /// Custom user-defined platform.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for DeploymentPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentPlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            DeploymentPlatformEnum::GKE => "GKE",
            DeploymentPlatformEnum::FLEX => "FLEX",
            DeploymentPlatformEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(DeploymentPlatformEnum::PLATFORMUNSPECIFIED),
           "GKE" => Ok(DeploymentPlatformEnum::GKE),
           "FLEX" => Ok(DeploymentPlatformEnum::FLEX),
           "CUSTOM" => Ok(DeploymentPlatformEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoveredAnalysisStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of discovery for the resource.
pub enum DiscoveredAnalysisStatusEnum {
    

    /// Unknown.
    ///
    /// "ANALYSIS_STATUS_UNSPECIFIED"
    #[serde(rename="ANALYSIS_STATUS_UNSPECIFIED")]
    ANALYSISSTATUSUNSPECIFIED,
    

    /// Resource is known but no action has been taken yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Resource is being analyzed.
    ///
    /// "SCANNING"
    #[serde(rename="SCANNING")]
    SCANNING,
    

    /// Analysis has finished successfully.
    ///
    /// "FINISHED_SUCCESS"
    #[serde(rename="FINISHED_SUCCESS")]
    FINISHEDSUCCESS,
    

    /// Analysis has completed.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
    

    /// Analysis has finished unsuccessfully, the analysis itself is in a bad state.
    ///
    /// "FINISHED_FAILED"
    #[serde(rename="FINISHED_FAILED")]
    FINISHEDFAILED,
    

    /// The resource is known not to be supported
    ///
    /// "FINISHED_UNSUPPORTED"
    #[serde(rename="FINISHED_UNSUPPORTED")]
    FINISHEDUNSUPPORTED,
}

impl AsRef<str> for DiscoveredAnalysisStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoveredAnalysisStatusEnum::ANALYSISSTATUSUNSPECIFIED => "ANALYSIS_STATUS_UNSPECIFIED",
            DiscoveredAnalysisStatusEnum::PENDING => "PENDING",
            DiscoveredAnalysisStatusEnum::SCANNING => "SCANNING",
            DiscoveredAnalysisStatusEnum::FINISHEDSUCCESS => "FINISHED_SUCCESS",
            DiscoveredAnalysisStatusEnum::COMPLETE => "COMPLETE",
            DiscoveredAnalysisStatusEnum::FINISHEDFAILED => "FINISHED_FAILED",
            DiscoveredAnalysisStatusEnum::FINISHEDUNSUPPORTED => "FINISHED_UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoveredAnalysisStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANALYSIS_STATUS_UNSPECIFIED" => Ok(DiscoveredAnalysisStatusEnum::ANALYSISSTATUSUNSPECIFIED),
           "PENDING" => Ok(DiscoveredAnalysisStatusEnum::PENDING),
           "SCANNING" => Ok(DiscoveredAnalysisStatusEnum::SCANNING),
           "FINISHED_SUCCESS" => Ok(DiscoveredAnalysisStatusEnum::FINISHEDSUCCESS),
           "COMPLETE" => Ok(DiscoveredAnalysisStatusEnum::COMPLETE),
           "FINISHED_FAILED" => Ok(DiscoveredAnalysisStatusEnum::FINISHEDFAILED),
           "FINISHED_UNSUPPORTED" => Ok(DiscoveredAnalysisStatusEnum::FINISHEDUNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoveredAnalysisStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoveredContinuousAnalysisEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the resource is continuously analyzed.
pub enum DiscoveredContinuousAnalysisEnum {
    

    /// Unknown.
    ///
    /// "CONTINUOUS_ANALYSIS_UNSPECIFIED"
    #[serde(rename="CONTINUOUS_ANALYSIS_UNSPECIFIED")]
    CONTINUOUSANALYSISUNSPECIFIED,
    

    /// The resource is continuously analyzed.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The resource is ignored for continuous analysis.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for DiscoveredContinuousAnalysisEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoveredContinuousAnalysisEnum::CONTINUOUSANALYSISUNSPECIFIED => "CONTINUOUS_ANALYSIS_UNSPECIFIED",
            DiscoveredContinuousAnalysisEnum::ACTIVE => "ACTIVE",
            DiscoveredContinuousAnalysisEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoveredContinuousAnalysisEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTINUOUS_ANALYSIS_UNSPECIFIED" => Ok(DiscoveredContinuousAnalysisEnum::CONTINUOUSANALYSISUNSPECIFIED),
           "ACTIVE" => Ok(DiscoveredContinuousAnalysisEnum::ACTIVE),
           "INACTIVE" => Ok(DiscoveredContinuousAnalysisEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoveredContinuousAnalysisEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoveryAnalysisKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The kind of analysis that is handled by this discovery.
pub enum DiscoveryAnalysisKindEnum {
    

    /// Default value. This value is unused.
    ///
    /// "NOTE_KIND_UNSPECIFIED"
    #[serde(rename="NOTE_KIND_UNSPECIFIED")]
    NOTEKINDUNSPECIFIED,
    

    /// The note and occurrence represent a package vulnerability.
    ///
    /// "VULNERABILITY"
    #[serde(rename="VULNERABILITY")]
    VULNERABILITY,
    

    /// The note and occurrence assert build provenance.
    ///
    /// "BUILD"
    #[serde(rename="BUILD")]
    BUILD,
    

    /// This represents an image basis relationship.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// This represents a package installed via a package manager.
    ///
    /// "PACKAGE"
    #[serde(rename="PACKAGE")]
    PACKAGE,
    

    /// The note and occurrence track deployment events.
    ///
    /// "DEPLOYMENT"
    #[serde(rename="DEPLOYMENT")]
    DEPLOYMENT,
    

    /// The note and occurrence track the initial discovery status of a resource.
    ///
    /// "DISCOVERY"
    #[serde(rename="DISCOVERY")]
    DISCOVERY,
    

    /// This represents a logical "role" that can attest to artifacts.
    ///
    /// "ATTESTATION"
    #[serde(rename="ATTESTATION")]
    ATTESTATION,
    

    /// This represents an in-toto link.
    ///
    /// "INTOTO"
    #[serde(rename="INTOTO")]
    INTOTO,
    

    /// This represents a software bill of materials.
    ///
    /// "SBOM"
    #[serde(rename="SBOM")]
    SBOM,
    

    /// This represents an SPDX Package.
    ///
    /// "SPDX_PACKAGE"
    #[serde(rename="SPDX_PACKAGE")]
    SPDXPACKAGE,
    

    /// This represents an SPDX File.
    ///
    /// "SPDX_FILE"
    #[serde(rename="SPDX_FILE")]
    SPDXFILE,
    

    /// This represents an SPDX Relationship.
    ///
    /// "SPDX_RELATIONSHIP"
    #[serde(rename="SPDX_RELATIONSHIP")]
    SPDXRELATIONSHIP,
    

    /// This represents a Vulnerability Assessment.
    ///
    /// "VULNERABILITY_ASSESSMENT"
    #[serde(rename="VULNERABILITY_ASSESSMENT")]
    VULNERABILITYASSESSMENT,
    

    /// This represents an SBOM Reference.
    ///
    /// "SBOM_REFERENCE"
    #[serde(rename="SBOM_REFERENCE")]
    SBOMREFERENCE,
}

impl AsRef<str> for DiscoveryAnalysisKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoveryAnalysisKindEnum::NOTEKINDUNSPECIFIED => "NOTE_KIND_UNSPECIFIED",
            DiscoveryAnalysisKindEnum::VULNERABILITY => "VULNERABILITY",
            DiscoveryAnalysisKindEnum::BUILD => "BUILD",
            DiscoveryAnalysisKindEnum::IMAGE => "IMAGE",
            DiscoveryAnalysisKindEnum::PACKAGE => "PACKAGE",
            DiscoveryAnalysisKindEnum::DEPLOYMENT => "DEPLOYMENT",
            DiscoveryAnalysisKindEnum::DISCOVERY => "DISCOVERY",
            DiscoveryAnalysisKindEnum::ATTESTATION => "ATTESTATION",
            DiscoveryAnalysisKindEnum::INTOTO => "INTOTO",
            DiscoveryAnalysisKindEnum::SBOM => "SBOM",
            DiscoveryAnalysisKindEnum::SPDXPACKAGE => "SPDX_PACKAGE",
            DiscoveryAnalysisKindEnum::SPDXFILE => "SPDX_FILE",
            DiscoveryAnalysisKindEnum::SPDXRELATIONSHIP => "SPDX_RELATIONSHIP",
            DiscoveryAnalysisKindEnum::VULNERABILITYASSESSMENT => "VULNERABILITY_ASSESSMENT",
            DiscoveryAnalysisKindEnum::SBOMREFERENCE => "SBOM_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoveryAnalysisKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTE_KIND_UNSPECIFIED" => Ok(DiscoveryAnalysisKindEnum::NOTEKINDUNSPECIFIED),
           "VULNERABILITY" => Ok(DiscoveryAnalysisKindEnum::VULNERABILITY),
           "BUILD" => Ok(DiscoveryAnalysisKindEnum::BUILD),
           "IMAGE" => Ok(DiscoveryAnalysisKindEnum::IMAGE),
           "PACKAGE" => Ok(DiscoveryAnalysisKindEnum::PACKAGE),
           "DEPLOYMENT" => Ok(DiscoveryAnalysisKindEnum::DEPLOYMENT),
           "DISCOVERY" => Ok(DiscoveryAnalysisKindEnum::DISCOVERY),
           "ATTESTATION" => Ok(DiscoveryAnalysisKindEnum::ATTESTATION),
           "INTOTO" => Ok(DiscoveryAnalysisKindEnum::INTOTO),
           "SBOM" => Ok(DiscoveryAnalysisKindEnum::SBOM),
           "SPDX_PACKAGE" => Ok(DiscoveryAnalysisKindEnum::SPDXPACKAGE),
           "SPDX_FILE" => Ok(DiscoveryAnalysisKindEnum::SPDXFILE),
           "SPDX_RELATIONSHIP" => Ok(DiscoveryAnalysisKindEnum::SPDXRELATIONSHIP),
           "VULNERABILITY_ASSESSMENT" => Ok(DiscoveryAnalysisKindEnum::VULNERABILITYASSESSMENT),
           "SBOM_REFERENCE" => Ok(DiscoveryAnalysisKindEnum::SBOMREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoveryAnalysisKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DistributionArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The CPU architecture for which packages in this distribution channel were built.
pub enum DistributionArchitectureEnum {
    

    /// Unknown architecture.
    ///
    /// "ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="ARCHITECTURE_UNSPECIFIED")]
    ARCHITECTUREUNSPECIFIED,
    

    /// X86 architecture.
    ///
    /// "X86"
    #[serde(rename="X86")]
    X86,
    

    /// X64 architecture.
    ///
    /// "X64"
    #[serde(rename="X64")]
    X64,
}

impl AsRef<str> for DistributionArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DistributionArchitectureEnum::ARCHITECTUREUNSPECIFIED => "ARCHITECTURE_UNSPECIFIED",
            DistributionArchitectureEnum::X86 => "X86",
            DistributionArchitectureEnum::X64 => "X64",
        }
    }
}

impl std::convert::TryFrom< &str> for DistributionArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARCHITECTURE_UNSPECIFIED" => Ok(DistributionArchitectureEnum::ARCHITECTUREUNSPECIFIED),
           "X86" => Ok(DistributionArchitectureEnum::X86),
           "X64" => Ok(DistributionArchitectureEnum::X64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DistributionArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExternalRefCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An External Reference allows a Package to reference an external source of additional information, metadata, enumerations, asset identifiers, or downloadable content believed to be relevant to the Package
pub enum ExternalRefCategoryEnum {
    

    /// Unspecified
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// Security (e.g. cpe22Type, cpe23Type)
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Package Manager (e.g. maven-central, npm, nuget, bower, purl)
    ///
    /// "PACKAGE_MANAGER"
    #[serde(rename="PACKAGE_MANAGER")]
    PACKAGEMANAGER,
    

    /// Persistent-Id (e.g. swh)
    ///
    /// "PERSISTENT_ID"
    #[serde(rename="PERSISTENT_ID")]
    PERSISTENTID,
    

    /// Other
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for ExternalRefCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExternalRefCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            ExternalRefCategoryEnum::SECURITY => "SECURITY",
            ExternalRefCategoryEnum::PACKAGEMANAGER => "PACKAGE_MANAGER",
            ExternalRefCategoryEnum::PERSISTENTID => "PERSISTENT_ID",
            ExternalRefCategoryEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for ExternalRefCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(ExternalRefCategoryEnum::CATEGORYUNSPECIFIED),
           "SECURITY" => Ok(ExternalRefCategoryEnum::SECURITY),
           "PACKAGE_MANAGER" => Ok(ExternalRefCategoryEnum::PACKAGEMANAGER),
           "PERSISTENT_ID" => Ok(ExternalRefCategoryEnum::PERSISTENTID),
           "OTHER" => Ok(ExternalRefCategoryEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExternalRefCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FileNoteFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This field provides information about the type of file identified
pub enum FileNoteFileTypeEnum {
    

    /// Unspecified
    ///
    /// "FILE_TYPE_UNSPECIFIED"
    #[serde(rename="FILE_TYPE_UNSPECIFIED")]
    FILETYPEUNSPECIFIED,
    

    /// The file is human readable source code (.c, .html, etc.)
    ///
    /// "SOURCE"
    #[serde(rename="SOURCE")]
    SOURCE,
    

    /// The file is a compiled object, target image or binary executable (.o, .a, etc.)
    ///
    /// "BINARY"
    #[serde(rename="BINARY")]
    BINARY,
    

    /// The file represents an archive (.tar, .jar, etc.)
    ///
    /// "ARCHIVE"
    #[serde(rename="ARCHIVE")]
    ARCHIVE,
    

    /// The file is associated with a specific application type (MIME type of application/*)
    ///
    /// "APPLICATION"
    #[serde(rename="APPLICATION")]
    APPLICATION,
    

    /// The file is associated with an audio file (MIME type of audio/* , e.g. .mp3)
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    

    /// The file is associated with an picture image file (MIME type of image/*, e.g., .jpg, .gif)
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// The file is human readable text file (MIME type of text/*)
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// The file is associated with a video file type (MIME type of video/*)
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// The file serves as documentation
    ///
    /// "DOCUMENTATION"
    #[serde(rename="DOCUMENTATION")]
    DOCUMENTATION,
    

    /// The file is an SPDX document
    ///
    /// "SPDX"
    #[serde(rename="SPDX")]
    SPDX,
    

    /// The file doesn't fit into the above categories (generated artifacts, data files, etc.)
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for FileNoteFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileNoteFileTypeEnum::FILETYPEUNSPECIFIED => "FILE_TYPE_UNSPECIFIED",
            FileNoteFileTypeEnum::SOURCE => "SOURCE",
            FileNoteFileTypeEnum::BINARY => "BINARY",
            FileNoteFileTypeEnum::ARCHIVE => "ARCHIVE",
            FileNoteFileTypeEnum::APPLICATION => "APPLICATION",
            FileNoteFileTypeEnum::AUDIO => "AUDIO",
            FileNoteFileTypeEnum::IMAGE => "IMAGE",
            FileNoteFileTypeEnum::TEXT => "TEXT",
            FileNoteFileTypeEnum::VIDEO => "VIDEO",
            FileNoteFileTypeEnum::DOCUMENTATION => "DOCUMENTATION",
            FileNoteFileTypeEnum::SPDX => "SPDX",
            FileNoteFileTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for FileNoteFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILE_TYPE_UNSPECIFIED" => Ok(FileNoteFileTypeEnum::FILETYPEUNSPECIFIED),
           "SOURCE" => Ok(FileNoteFileTypeEnum::SOURCE),
           "BINARY" => Ok(FileNoteFileTypeEnum::BINARY),
           "ARCHIVE" => Ok(FileNoteFileTypeEnum::ARCHIVE),
           "APPLICATION" => Ok(FileNoteFileTypeEnum::APPLICATION),
           "AUDIO" => Ok(FileNoteFileTypeEnum::AUDIO),
           "IMAGE" => Ok(FileNoteFileTypeEnum::IMAGE),
           "TEXT" => Ok(FileNoteFileTypeEnum::TEXT),
           "VIDEO" => Ok(FileNoteFileTypeEnum::VIDEO),
           "DOCUMENTATION" => Ok(FileNoteFileTypeEnum::DOCUMENTATION),
           "SPDX" => Ok(FileNoteFileTypeEnum::SPDX),
           "OTHER" => Ok(FileNoteFileTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileNoteFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FixableTotalByDigestSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity for this count. SEVERITY_UNSPECIFIED indicates total across all severities.
pub enum FixableTotalByDigestSeverityEnum {
    

    /// Unknown.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Minimal severity.
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
    

    /// Low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// High severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for FixableTotalByDigestSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FixableTotalByDigestSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            FixableTotalByDigestSeverityEnum::MINIMAL => "MINIMAL",
            FixableTotalByDigestSeverityEnum::LOW => "LOW",
            FixableTotalByDigestSeverityEnum::MEDIUM => "MEDIUM",
            FixableTotalByDigestSeverityEnum::HIGH => "HIGH",
            FixableTotalByDigestSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for FixableTotalByDigestSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(FixableTotalByDigestSeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(FixableTotalByDigestSeverityEnum::MINIMAL),
           "LOW" => Ok(FixableTotalByDigestSeverityEnum::LOW),
           "MEDIUM" => Ok(FixableTotalByDigestSeverityEnum::MEDIUM),
           "HIGH" => Ok(FixableTotalByDigestSeverityEnum::HIGH),
           "CRITICAL" => Ok(FixableTotalByDigestSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FixableTotalByDigestSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GenericSignedAttestationContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema).
pub enum GenericSignedAttestationContentTypeEnum {
    

    /// `ContentType` is not set.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Atomic format attestation signature. See https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md The payload extracted in `plaintext` is a JSON blob conforming to the linked schema.
    ///
    /// "SIMPLE_SIGNING_JSON"
    #[serde(rename="SIMPLE_SIGNING_JSON")]
    SIMPLESIGNINGJSON,
}

impl AsRef<str> for GenericSignedAttestationContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GenericSignedAttestationContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            GenericSignedAttestationContentTypeEnum::SIMPLESIGNINGJSON => "SIMPLE_SIGNING_JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for GenericSignedAttestationContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(GenericSignedAttestationContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "SIMPLE_SIGNING_JSON" => Ok(GenericSignedAttestationContentTypeEnum::SIMPLESIGNINGJSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GenericSignedAttestationContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GrafeasV1beta1VulnerabilityDetailCvssVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. CVSS version used to populate cvss_score and severity.
pub enum GrafeasV1beta1VulnerabilityDetailCvssVersionEnum {
    
    /// "CVSS_VERSION_UNSPECIFIED"
    #[serde(rename="CVSS_VERSION_UNSPECIFIED")]
    CVSSVERSIONUNSPECIFIED,
    
    /// "CVSS_VERSION_2"
    #[serde(rename="CVSS_VERSION_2")]
    CVSSVERSION2,
    
    /// "CVSS_VERSION_3"
    #[serde(rename="CVSS_VERSION_3")]
    CVSSVERSION3,
}

impl AsRef<str> for GrafeasV1beta1VulnerabilityDetailCvssVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GrafeasV1beta1VulnerabilityDetailCvssVersionEnum::CVSSVERSIONUNSPECIFIED => "CVSS_VERSION_UNSPECIFIED",
            GrafeasV1beta1VulnerabilityDetailCvssVersionEnum::CVSSVERSION2 => "CVSS_VERSION_2",
            GrafeasV1beta1VulnerabilityDetailCvssVersionEnum::CVSSVERSION3 => "CVSS_VERSION_3",
        }
    }
}

impl std::convert::TryFrom< &str> for GrafeasV1beta1VulnerabilityDetailCvssVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CVSS_VERSION_UNSPECIFIED" => Ok(GrafeasV1beta1VulnerabilityDetailCvssVersionEnum::CVSSVERSIONUNSPECIFIED),
           "CVSS_VERSION_2" => Ok(GrafeasV1beta1VulnerabilityDetailCvssVersionEnum::CVSSVERSION2),
           "CVSS_VERSION_3" => Ok(GrafeasV1beta1VulnerabilityDetailCvssVersionEnum::CVSSVERSION3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GrafeasV1beta1VulnerabilityDetailCvssVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The distro assigned severity for this vulnerability when it is available, and note provider assigned severity when distro has not yet assigned a severity for this vulnerability. When there are multiple PackageIssues for this vulnerability, they can have different effective severities because some might be provided by the distro while others are provided by the language ecosystem for a language pack. For this reason, it is advised to use the effective severity on the PackageIssue level. In the case where multiple PackageIssues have differing effective severities, this field should be the highest severity for any of the PackageIssues.
pub enum GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum {
    

    /// Unknown.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Minimal severity.
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
    

    /// Low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// High severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::MINIMAL => "MINIMAL",
            GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::LOW => "LOW",
            GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::MEDIUM => "MEDIUM",
            GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::HIGH => "HIGH",
            GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::MINIMAL),
           "LOW" => Ok(GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::LOW),
           "MEDIUM" => Ok(GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::MEDIUM),
           "HIGH" => Ok(GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::HIGH),
           "CRITICAL" => Ok(GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GrafeasV1beta1VulnerabilityDetailSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The note provider assigned Severity of the vulnerability.
pub enum GrafeasV1beta1VulnerabilityDetailSeverityEnum {
    

    /// Unknown.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Minimal severity.
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
    

    /// Low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// High severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for GrafeasV1beta1VulnerabilityDetailSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GrafeasV1beta1VulnerabilityDetailSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GrafeasV1beta1VulnerabilityDetailSeverityEnum::MINIMAL => "MINIMAL",
            GrafeasV1beta1VulnerabilityDetailSeverityEnum::LOW => "LOW",
            GrafeasV1beta1VulnerabilityDetailSeverityEnum::MEDIUM => "MEDIUM",
            GrafeasV1beta1VulnerabilityDetailSeverityEnum::HIGH => "HIGH",
            GrafeasV1beta1VulnerabilityDetailSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GrafeasV1beta1VulnerabilityDetailSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GrafeasV1beta1VulnerabilityDetailSeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(GrafeasV1beta1VulnerabilityDetailSeverityEnum::MINIMAL),
           "LOW" => Ok(GrafeasV1beta1VulnerabilityDetailSeverityEnum::LOW),
           "MEDIUM" => Ok(GrafeasV1beta1VulnerabilityDetailSeverityEnum::MEDIUM),
           "HIGH" => Ok(GrafeasV1beta1VulnerabilityDetailSeverityEnum::HIGH),
           "CRITICAL" => Ok(GrafeasV1beta1VulnerabilityDetailSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GrafeasV1beta1VulnerabilityDetailSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HashTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of hash that was performed.
pub enum HashTypeEnum {
    

    /// Unknown.
    ///
    /// "HASH_TYPE_UNSPECIFIED"
    #[serde(rename="HASH_TYPE_UNSPECIFIED")]
    HASHTYPEUNSPECIFIED,
    

    /// A SHA-256 hash.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// Dirhash of a Go module's source code which is then hex-encoded. See b/244466565 and https://github.com/in-toto/attestation/pull/108.
    ///
    /// "GO_MODULE_H1"
    #[serde(rename="GO_MODULE_H1")]
    GOMODULEH1,
    

    /// A SHA-512 hash.
    ///
    /// "SHA512"
    #[serde(rename="SHA512")]
    SHA512,
}

impl AsRef<str> for HashTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HashTypeEnum::HASHTYPEUNSPECIFIED => "HASH_TYPE_UNSPECIFIED",
            HashTypeEnum::SHA256 => "SHA256",
            HashTypeEnum::GOMODULEH1 => "GO_MODULE_H1",
            HashTypeEnum::SHA512 => "SHA512",
        }
    }
}

impl std::convert::TryFrom< &str> for HashTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HASH_TYPE_UNSPECIFIED" => Ok(HashTypeEnum::HASHTYPEUNSPECIFIED),
           "SHA256" => Ok(HashTypeEnum::SHA256),
           "GO_MODULE_H1" => Ok(HashTypeEnum::GOMODULEH1),
           "SHA512" => Ok(HashTypeEnum::SHA512),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HashTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstallationArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The CPU architecture for which packages in this distribution channel were built. Architecture will be blank for language packages.
pub enum InstallationArchitectureEnum {
    

    /// Unknown architecture.
    ///
    /// "ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="ARCHITECTURE_UNSPECIFIED")]
    ARCHITECTUREUNSPECIFIED,
    

    /// X86 architecture.
    ///
    /// "X86"
    #[serde(rename="X86")]
    X86,
    

    /// X64 architecture.
    ///
    /// "X64"
    #[serde(rename="X64")]
    X64,
}

impl AsRef<str> for InstallationArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstallationArchitectureEnum::ARCHITECTUREUNSPECIFIED => "ARCHITECTURE_UNSPECIFIED",
            InstallationArchitectureEnum::X86 => "X86",
            InstallationArchitectureEnum::X64 => "X64",
        }
    }
}

impl std::convert::TryFrom< &str> for InstallationArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARCHITECTURE_UNSPECIFIED" => Ok(InstallationArchitectureEnum::ARCHITECTUREUNSPECIFIED),
           "X86" => Ok(InstallationArchitectureEnum::X86),
           "X64" => Ok(InstallationArchitectureEnum::X64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstallationArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JustificationJustificationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The justification type for this vulnerability.
pub enum JustificationJustificationTypeEnum {
    

    /// JUSTIFICATION_TYPE_UNSPECIFIED.
    ///
    /// "JUSTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="JUSTIFICATION_TYPE_UNSPECIFIED")]
    JUSTIFICATIONTYPEUNSPECIFIED,
    

    /// The vulnerable component is not present in the product.
    ///
    /// "COMPONENT_NOT_PRESENT"
    #[serde(rename="COMPONENT_NOT_PRESENT")]
    COMPONENTNOTPRESENT,
    

    /// The vulnerable code is not present. Typically this case occurs when source code is configured or built in a way that excludes the vulnerable code.
    ///
    /// "VULNERABLE_CODE_NOT_PRESENT"
    #[serde(rename="VULNERABLE_CODE_NOT_PRESENT")]
    VULNERABLECODENOTPRESENT,
    

    /// The vulnerable code can not be executed. Typically this case occurs when the product includes the vulnerable code but does not call or use the vulnerable code.
    ///
    /// "VULNERABLE_CODE_NOT_IN_EXECUTE_PATH"
    #[serde(rename="VULNERABLE_CODE_NOT_IN_EXECUTE_PATH")]
    VULNERABLECODENOTINEXECUTEPATH,
    

    /// The vulnerable code cannot be controlled by an attacker to exploit the vulnerability.
    ///
    /// "VULNERABLE_CODE_CANNOT_BE_CONTROLLED_BY_ADVERSARY"
    #[serde(rename="VULNERABLE_CODE_CANNOT_BE_CONTROLLED_BY_ADVERSARY")]
    VULNERABLECODECANNOTBECONTROLLEDBYADVERSARY,
    

    /// The product includes built-in protections or features that prevent exploitation of the vulnerability. These built-in protections cannot be subverted by the attacker and cannot be configured or disabled by the user. These mitigations completely prevent exploitation based on known attack vectors.
    ///
    /// "INLINE_MITIGATIONS_ALREADY_EXIST"
    #[serde(rename="INLINE_MITIGATIONS_ALREADY_EXIST")]
    INLINEMITIGATIONSALREADYEXIST,
}

impl AsRef<str> for JustificationJustificationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JustificationJustificationTypeEnum::JUSTIFICATIONTYPEUNSPECIFIED => "JUSTIFICATION_TYPE_UNSPECIFIED",
            JustificationJustificationTypeEnum::COMPONENTNOTPRESENT => "COMPONENT_NOT_PRESENT",
            JustificationJustificationTypeEnum::VULNERABLECODENOTPRESENT => "VULNERABLE_CODE_NOT_PRESENT",
            JustificationJustificationTypeEnum::VULNERABLECODENOTINEXECUTEPATH => "VULNERABLE_CODE_NOT_IN_EXECUTE_PATH",
            JustificationJustificationTypeEnum::VULNERABLECODECANNOTBECONTROLLEDBYADVERSARY => "VULNERABLE_CODE_CANNOT_BE_CONTROLLED_BY_ADVERSARY",
            JustificationJustificationTypeEnum::INLINEMITIGATIONSALREADYEXIST => "INLINE_MITIGATIONS_ALREADY_EXIST",
        }
    }
}

impl std::convert::TryFrom< &str> for JustificationJustificationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JUSTIFICATION_TYPE_UNSPECIFIED" => Ok(JustificationJustificationTypeEnum::JUSTIFICATIONTYPEUNSPECIFIED),
           "COMPONENT_NOT_PRESENT" => Ok(JustificationJustificationTypeEnum::COMPONENTNOTPRESENT),
           "VULNERABLE_CODE_NOT_PRESENT" => Ok(JustificationJustificationTypeEnum::VULNERABLECODENOTPRESENT),
           "VULNERABLE_CODE_NOT_IN_EXECUTE_PATH" => Ok(JustificationJustificationTypeEnum::VULNERABLECODENOTINEXECUTEPATH),
           "VULNERABLE_CODE_CANNOT_BE_CONTROLLED_BY_ADVERSARY" => Ok(JustificationJustificationTypeEnum::VULNERABLECODECANNOTBECONTROLLEDBYADVERSARY),
           "INLINE_MITIGATIONS_ALREADY_EXIST" => Ok(JustificationJustificationTypeEnum::INLINEMITIGATIONSALREADYEXIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JustificationJustificationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LayerDirectiveEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The recovered Dockerfile directive used to construct this layer.
pub enum LayerDirectiveEnum {
    

    /// Default value for unsupported/missing directive.
    ///
    /// "DIRECTIVE_UNSPECIFIED"
    #[serde(rename="DIRECTIVE_UNSPECIFIED")]
    DIRECTIVEUNSPECIFIED,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "MAINTAINER"
    #[serde(rename="MAINTAINER")]
    MAINTAINER,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "RUN"
    #[serde(rename="RUN")]
    RUN,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "CMD"
    #[serde(rename="CMD")]
    CMD,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "LABEL"
    #[serde(rename="LABEL")]
    LABEL,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "EXPOSE"
    #[serde(rename="EXPOSE")]
    EXPOSE,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "ENV"
    #[serde(rename="ENV")]
    ENV,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "COPY"
    #[serde(rename="COPY")]
    COPY,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "ENTRYPOINT"
    #[serde(rename="ENTRYPOINT")]
    ENTRYPOINT,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "VOLUME"
    #[serde(rename="VOLUME")]
    VOLUME,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "WORKDIR"
    #[serde(rename="WORKDIR")]
    WORKDIR,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "ARG"
    #[serde(rename="ARG")]
    ARG,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "ONBUILD"
    #[serde(rename="ONBUILD")]
    ONBUILD,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "STOPSIGNAL"
    #[serde(rename="STOPSIGNAL")]
    STOPSIGNAL,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "HEALTHCHECK"
    #[serde(rename="HEALTHCHECK")]
    HEALTHCHECK,
    

    /// https://docs.docker.com/engine/reference/builder/
    ///
    /// "SHELL"
    #[serde(rename="SHELL")]
    SHELL,
}

impl AsRef<str> for LayerDirectiveEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LayerDirectiveEnum::DIRECTIVEUNSPECIFIED => "DIRECTIVE_UNSPECIFIED",
            LayerDirectiveEnum::MAINTAINER => "MAINTAINER",
            LayerDirectiveEnum::RUN => "RUN",
            LayerDirectiveEnum::CMD => "CMD",
            LayerDirectiveEnum::LABEL => "LABEL",
            LayerDirectiveEnum::EXPOSE => "EXPOSE",
            LayerDirectiveEnum::ENV => "ENV",
            LayerDirectiveEnum::ADD => "ADD",
            LayerDirectiveEnum::COPY => "COPY",
            LayerDirectiveEnum::ENTRYPOINT => "ENTRYPOINT",
            LayerDirectiveEnum::VOLUME => "VOLUME",
            LayerDirectiveEnum::USER => "USER",
            LayerDirectiveEnum::WORKDIR => "WORKDIR",
            LayerDirectiveEnum::ARG => "ARG",
            LayerDirectiveEnum::ONBUILD => "ONBUILD",
            LayerDirectiveEnum::STOPSIGNAL => "STOPSIGNAL",
            LayerDirectiveEnum::HEALTHCHECK => "HEALTHCHECK",
            LayerDirectiveEnum::SHELL => "SHELL",
        }
    }
}

impl std::convert::TryFrom< &str> for LayerDirectiveEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIRECTIVE_UNSPECIFIED" => Ok(LayerDirectiveEnum::DIRECTIVEUNSPECIFIED),
           "MAINTAINER" => Ok(LayerDirectiveEnum::MAINTAINER),
           "RUN" => Ok(LayerDirectiveEnum::RUN),
           "CMD" => Ok(LayerDirectiveEnum::CMD),
           "LABEL" => Ok(LayerDirectiveEnum::LABEL),
           "EXPOSE" => Ok(LayerDirectiveEnum::EXPOSE),
           "ENV" => Ok(LayerDirectiveEnum::ENV),
           "ADD" => Ok(LayerDirectiveEnum::ADD),
           "COPY" => Ok(LayerDirectiveEnum::COPY),
           "ENTRYPOINT" => Ok(LayerDirectiveEnum::ENTRYPOINT),
           "VOLUME" => Ok(LayerDirectiveEnum::VOLUME),
           "USER" => Ok(LayerDirectiveEnum::USER),
           "WORKDIR" => Ok(LayerDirectiveEnum::WORKDIR),
           "ARG" => Ok(LayerDirectiveEnum::ARG),
           "ONBUILD" => Ok(LayerDirectiveEnum::ONBUILD),
           "STOPSIGNAL" => Ok(LayerDirectiveEnum::STOPSIGNAL),
           "HEALTHCHECK" => Ok(LayerDirectiveEnum::HEALTHCHECK),
           "SHELL" => Ok(LayerDirectiveEnum::SHELL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LayerDirectiveEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NoteKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of analysis. This field can be used as a filter in list requests.
pub enum NoteKindEnum {
    

    /// Default value. This value is unused.
    ///
    /// "NOTE_KIND_UNSPECIFIED"
    #[serde(rename="NOTE_KIND_UNSPECIFIED")]
    NOTEKINDUNSPECIFIED,
    

    /// The note and occurrence represent a package vulnerability.
    ///
    /// "VULNERABILITY"
    #[serde(rename="VULNERABILITY")]
    VULNERABILITY,
    

    /// The note and occurrence assert build provenance.
    ///
    /// "BUILD"
    #[serde(rename="BUILD")]
    BUILD,
    

    /// This represents an image basis relationship.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// This represents a package installed via a package manager.
    ///
    /// "PACKAGE"
    #[serde(rename="PACKAGE")]
    PACKAGE,
    

    /// The note and occurrence track deployment events.
    ///
    /// "DEPLOYMENT"
    #[serde(rename="DEPLOYMENT")]
    DEPLOYMENT,
    

    /// The note and occurrence track the initial discovery status of a resource.
    ///
    /// "DISCOVERY"
    #[serde(rename="DISCOVERY")]
    DISCOVERY,
    

    /// This represents a logical "role" that can attest to artifacts.
    ///
    /// "ATTESTATION"
    #[serde(rename="ATTESTATION")]
    ATTESTATION,
    

    /// This represents an in-toto link.
    ///
    /// "INTOTO"
    #[serde(rename="INTOTO")]
    INTOTO,
    

    /// This represents a software bill of materials.
    ///
    /// "SBOM"
    #[serde(rename="SBOM")]
    SBOM,
    

    /// This represents an SPDX Package.
    ///
    /// "SPDX_PACKAGE"
    #[serde(rename="SPDX_PACKAGE")]
    SPDXPACKAGE,
    

    /// This represents an SPDX File.
    ///
    /// "SPDX_FILE"
    #[serde(rename="SPDX_FILE")]
    SPDXFILE,
    

    /// This represents an SPDX Relationship.
    ///
    /// "SPDX_RELATIONSHIP"
    #[serde(rename="SPDX_RELATIONSHIP")]
    SPDXRELATIONSHIP,
    

    /// This represents a Vulnerability Assessment.
    ///
    /// "VULNERABILITY_ASSESSMENT"
    #[serde(rename="VULNERABILITY_ASSESSMENT")]
    VULNERABILITYASSESSMENT,
    

    /// This represents an SBOM Reference.
    ///
    /// "SBOM_REFERENCE"
    #[serde(rename="SBOM_REFERENCE")]
    SBOMREFERENCE,
}

impl AsRef<str> for NoteKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NoteKindEnum::NOTEKINDUNSPECIFIED => "NOTE_KIND_UNSPECIFIED",
            NoteKindEnum::VULNERABILITY => "VULNERABILITY",
            NoteKindEnum::BUILD => "BUILD",
            NoteKindEnum::IMAGE => "IMAGE",
            NoteKindEnum::PACKAGE => "PACKAGE",
            NoteKindEnum::DEPLOYMENT => "DEPLOYMENT",
            NoteKindEnum::DISCOVERY => "DISCOVERY",
            NoteKindEnum::ATTESTATION => "ATTESTATION",
            NoteKindEnum::INTOTO => "INTOTO",
            NoteKindEnum::SBOM => "SBOM",
            NoteKindEnum::SPDXPACKAGE => "SPDX_PACKAGE",
            NoteKindEnum::SPDXFILE => "SPDX_FILE",
            NoteKindEnum::SPDXRELATIONSHIP => "SPDX_RELATIONSHIP",
            NoteKindEnum::VULNERABILITYASSESSMENT => "VULNERABILITY_ASSESSMENT",
            NoteKindEnum::SBOMREFERENCE => "SBOM_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for NoteKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTE_KIND_UNSPECIFIED" => Ok(NoteKindEnum::NOTEKINDUNSPECIFIED),
           "VULNERABILITY" => Ok(NoteKindEnum::VULNERABILITY),
           "BUILD" => Ok(NoteKindEnum::BUILD),
           "IMAGE" => Ok(NoteKindEnum::IMAGE),
           "PACKAGE" => Ok(NoteKindEnum::PACKAGE),
           "DEPLOYMENT" => Ok(NoteKindEnum::DEPLOYMENT),
           "DISCOVERY" => Ok(NoteKindEnum::DISCOVERY),
           "ATTESTATION" => Ok(NoteKindEnum::ATTESTATION),
           "INTOTO" => Ok(NoteKindEnum::INTOTO),
           "SBOM" => Ok(NoteKindEnum::SBOM),
           "SPDX_PACKAGE" => Ok(NoteKindEnum::SPDXPACKAGE),
           "SPDX_FILE" => Ok(NoteKindEnum::SPDXFILE),
           "SPDX_RELATIONSHIP" => Ok(NoteKindEnum::SPDXRELATIONSHIP),
           "VULNERABILITY_ASSESSMENT" => Ok(NoteKindEnum::VULNERABILITYASSESSMENT),
           "SBOM_REFERENCE" => Ok(NoteKindEnum::SBOMREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NoteKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OccurrenceKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests.
pub enum OccurrenceKindEnum {
    

    /// Default value. This value is unused.
    ///
    /// "NOTE_KIND_UNSPECIFIED"
    #[serde(rename="NOTE_KIND_UNSPECIFIED")]
    NOTEKINDUNSPECIFIED,
    

    /// The note and occurrence represent a package vulnerability.
    ///
    /// "VULNERABILITY"
    #[serde(rename="VULNERABILITY")]
    VULNERABILITY,
    

    /// The note and occurrence assert build provenance.
    ///
    /// "BUILD"
    #[serde(rename="BUILD")]
    BUILD,
    

    /// This represents an image basis relationship.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// This represents a package installed via a package manager.
    ///
    /// "PACKAGE"
    #[serde(rename="PACKAGE")]
    PACKAGE,
    

    /// The note and occurrence track deployment events.
    ///
    /// "DEPLOYMENT"
    #[serde(rename="DEPLOYMENT")]
    DEPLOYMENT,
    

    /// The note and occurrence track the initial discovery status of a resource.
    ///
    /// "DISCOVERY"
    #[serde(rename="DISCOVERY")]
    DISCOVERY,
    

    /// This represents a logical "role" that can attest to artifacts.
    ///
    /// "ATTESTATION"
    #[serde(rename="ATTESTATION")]
    ATTESTATION,
    

    /// This represents an in-toto link.
    ///
    /// "INTOTO"
    #[serde(rename="INTOTO")]
    INTOTO,
    

    /// This represents a software bill of materials.
    ///
    /// "SBOM"
    #[serde(rename="SBOM")]
    SBOM,
    

    /// This represents an SPDX Package.
    ///
    /// "SPDX_PACKAGE"
    #[serde(rename="SPDX_PACKAGE")]
    SPDXPACKAGE,
    

    /// This represents an SPDX File.
    ///
    /// "SPDX_FILE"
    #[serde(rename="SPDX_FILE")]
    SPDXFILE,
    

    /// This represents an SPDX Relationship.
    ///
    /// "SPDX_RELATIONSHIP"
    #[serde(rename="SPDX_RELATIONSHIP")]
    SPDXRELATIONSHIP,
    

    /// This represents a Vulnerability Assessment.
    ///
    /// "VULNERABILITY_ASSESSMENT"
    #[serde(rename="VULNERABILITY_ASSESSMENT")]
    VULNERABILITYASSESSMENT,
    

    /// This represents an SBOM Reference.
    ///
    /// "SBOM_REFERENCE"
    #[serde(rename="SBOM_REFERENCE")]
    SBOMREFERENCE,
}

impl AsRef<str> for OccurrenceKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OccurrenceKindEnum::NOTEKINDUNSPECIFIED => "NOTE_KIND_UNSPECIFIED",
            OccurrenceKindEnum::VULNERABILITY => "VULNERABILITY",
            OccurrenceKindEnum::BUILD => "BUILD",
            OccurrenceKindEnum::IMAGE => "IMAGE",
            OccurrenceKindEnum::PACKAGE => "PACKAGE",
            OccurrenceKindEnum::DEPLOYMENT => "DEPLOYMENT",
            OccurrenceKindEnum::DISCOVERY => "DISCOVERY",
            OccurrenceKindEnum::ATTESTATION => "ATTESTATION",
            OccurrenceKindEnum::INTOTO => "INTOTO",
            OccurrenceKindEnum::SBOM => "SBOM",
            OccurrenceKindEnum::SPDXPACKAGE => "SPDX_PACKAGE",
            OccurrenceKindEnum::SPDXFILE => "SPDX_FILE",
            OccurrenceKindEnum::SPDXRELATIONSHIP => "SPDX_RELATIONSHIP",
            OccurrenceKindEnum::VULNERABILITYASSESSMENT => "VULNERABILITY_ASSESSMENT",
            OccurrenceKindEnum::SBOMREFERENCE => "SBOM_REFERENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for OccurrenceKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTE_KIND_UNSPECIFIED" => Ok(OccurrenceKindEnum::NOTEKINDUNSPECIFIED),
           "VULNERABILITY" => Ok(OccurrenceKindEnum::VULNERABILITY),
           "BUILD" => Ok(OccurrenceKindEnum::BUILD),
           "IMAGE" => Ok(OccurrenceKindEnum::IMAGE),
           "PACKAGE" => Ok(OccurrenceKindEnum::PACKAGE),
           "DEPLOYMENT" => Ok(OccurrenceKindEnum::DEPLOYMENT),
           "DISCOVERY" => Ok(OccurrenceKindEnum::DISCOVERY),
           "ATTESTATION" => Ok(OccurrenceKindEnum::ATTESTATION),
           "INTOTO" => Ok(OccurrenceKindEnum::INTOTO),
           "SBOM" => Ok(OccurrenceKindEnum::SBOM),
           "SPDX_PACKAGE" => Ok(OccurrenceKindEnum::SPDXPACKAGE),
           "SPDX_FILE" => Ok(OccurrenceKindEnum::SPDXFILE),
           "SPDX_RELATIONSHIP" => Ok(OccurrenceKindEnum::SPDXRELATIONSHIP),
           "VULNERABILITY_ASSESSMENT" => Ok(OccurrenceKindEnum::VULNERABILITYASSESSMENT),
           "SBOM_REFERENCE" => Ok(OccurrenceKindEnum::SBOMREFERENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OccurrenceKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PackageArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The CPU architecture for which packages in this distribution channel were built. Architecture will be blank for language packages.
pub enum PackageArchitectureEnum {
    

    /// Unknown architecture.
    ///
    /// "ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="ARCHITECTURE_UNSPECIFIED")]
    ARCHITECTUREUNSPECIFIED,
    

    /// X86 architecture.
    ///
    /// "X86"
    #[serde(rename="X86")]
    X86,
    

    /// X64 architecture.
    ///
    /// "X64"
    #[serde(rename="X64")]
    X64,
}

impl AsRef<str> for PackageArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PackageArchitectureEnum::ARCHITECTUREUNSPECIFIED => "ARCHITECTURE_UNSPECIFIED",
            PackageArchitectureEnum::X86 => "X86",
            PackageArchitectureEnum::X64 => "X64",
        }
    }
}

impl std::convert::TryFrom< &str> for PackageArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARCHITECTURE_UNSPECIFIED" => Ok(PackageArchitectureEnum::ARCHITECTUREUNSPECIFIED),
           "X86" => Ok(PackageArchitectureEnum::X86),
           "X64" => Ok(PackageArchitectureEnum::X64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PackageArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PackageIssueEffectiveSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The distro or language system assigned severity for this vulnerability when that is available and note provider assigned severity when it is not available.
pub enum PackageIssueEffectiveSeverityEnum {
    

    /// Unknown.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Minimal severity.
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
    

    /// Low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// High severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for PackageIssueEffectiveSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PackageIssueEffectiveSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            PackageIssueEffectiveSeverityEnum::MINIMAL => "MINIMAL",
            PackageIssueEffectiveSeverityEnum::LOW => "LOW",
            PackageIssueEffectiveSeverityEnum::MEDIUM => "MEDIUM",
            PackageIssueEffectiveSeverityEnum::HIGH => "HIGH",
            PackageIssueEffectiveSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for PackageIssueEffectiveSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(PackageIssueEffectiveSeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(PackageIssueEffectiveSeverityEnum::MINIMAL),
           "LOW" => Ok(PackageIssueEffectiveSeverityEnum::LOW),
           "MEDIUM" => Ok(PackageIssueEffectiveSeverityEnum::MEDIUM),
           "HIGH" => Ok(PackageIssueEffectiveSeverityEnum::HIGH),
           "CRITICAL" => Ok(PackageIssueEffectiveSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PackageIssueEffectiveSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PgpSignedAttestationContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema).
pub enum PgpSignedAttestationContentTypeEnum {
    

    /// `ContentType` is not set.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Atomic format attestation signature. See https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md The payload extracted from `signature` is a JSON blob conforming to the linked schema.
    ///
    /// "SIMPLE_SIGNING_JSON"
    #[serde(rename="SIMPLE_SIGNING_JSON")]
    SIMPLESIGNINGJSON,
}

impl AsRef<str> for PgpSignedAttestationContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PgpSignedAttestationContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            PgpSignedAttestationContentTypeEnum::SIMPLESIGNINGJSON => "SIMPLE_SIGNING_JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for PgpSignedAttestationContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(PgpSignedAttestationContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "SIMPLE_SIGNING_JSON" => Ok(PgpSignedAttestationContentTypeEnum::SIMPLESIGNINGJSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PgpSignedAttestationContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RelationshipNoteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of relationship between the source and target SPDX elements
pub enum RelationshipNoteTypeEnum {
    

    /// Unspecified
    ///
    /// "RELATIONSHIP_TYPE_UNSPECIFIED"
    #[serde(rename="RELATIONSHIP_TYPE_UNSPECIFIED")]
    RELATIONSHIPTYPEUNSPECIFIED,
    

    /// Is to be used when SPDXRef-DOCUMENT describes SPDXRef-A
    ///
    /// "DESCRIBES"
    #[serde(rename="DESCRIBES")]
    DESCRIBES,
    

    /// Is to be used when SPDXRef-A is described by SPDXREF-Document
    ///
    /// "DESCRIBED_BY"
    #[serde(rename="DESCRIBED_BY")]
    DESCRIBEDBY,
    

    /// Is to be used when SPDXRef-A contains SPDXRef-B
    ///
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    

    /// Is to be used when SPDXRef-A is contained by SPDXRef-B
    ///
    /// "CONTAINED_BY"
    #[serde(rename="CONTAINED_BY")]
    CONTAINEDBY,
    

    /// Is to be used when SPDXRef-A depends on SPDXRef-B
    ///
    /// "DEPENDS_ON"
    #[serde(rename="DEPENDS_ON")]
    DEPENDSON,
    

    /// Is to be used when SPDXRef-A is dependency of SPDXRef-B
    ///
    /// "DEPENDENCY_OF"
    #[serde(rename="DEPENDENCY_OF")]
    DEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a manifest file that lists a set of dependencies for SPDXRef-B
    ///
    /// "DEPENDENCY_MANIFEST_OF"
    #[serde(rename="DEPENDENCY_MANIFEST_OF")]
    DEPENDENCYMANIFESTOF,
    

    /// Is to be used when SPDXRef-A is a build dependency of SPDXRef-B
    ///
    /// "BUILD_DEPENDENCY_OF"
    #[serde(rename="BUILD_DEPENDENCY_OF")]
    BUILDDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a development dependency of SPDXRef-B
    ///
    /// "DEV_DEPENDENCY_OF"
    #[serde(rename="DEV_DEPENDENCY_OF")]
    DEVDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is an optional dependency of SPDXRef-B
    ///
    /// "OPTIONAL_DEPENDENCY_OF"
    #[serde(rename="OPTIONAL_DEPENDENCY_OF")]
    OPTIONALDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a to be provided dependency of SPDXRef-B
    ///
    /// "PROVIDED_DEPENDENCY_OF"
    #[serde(rename="PROVIDED_DEPENDENCY_OF")]
    PROVIDEDDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a test dependency of SPDXRef-B
    ///
    /// "TEST_DEPENDENCY_OF"
    #[serde(rename="TEST_DEPENDENCY_OF")]
    TESTDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a dependency required for the execution of SPDXRef-B
    ///
    /// "RUNTIME_DEPENDENCY_OF"
    #[serde(rename="RUNTIME_DEPENDENCY_OF")]
    RUNTIMEDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is an example of SPDXRef-B
    ///
    /// "EXAMPLE_OF"
    #[serde(rename="EXAMPLE_OF")]
    EXAMPLEOF,
    

    /// Is to be used when SPDXRef-A generates SPDXRef-B
    ///
    /// "GENERATES"
    #[serde(rename="GENERATES")]
    GENERATES,
    

    /// Is to be used when SPDXRef-A was generated from SPDXRef-B
    ///
    /// "GENERATED_FROM"
    #[serde(rename="GENERATED_FROM")]
    GENERATEDFROM,
    

    /// Is to be used when SPDXRef-A is an ancestor (same lineage but pre-dates) SPDXRef-B
    ///
    /// "ANCESTOR_OF"
    #[serde(rename="ANCESTOR_OF")]
    ANCESTOROF,
    

    /// Is to be used when SPDXRef-A is a descendant of (same lineage but postdates) SPDXRef-B
    ///
    /// "DESCENDANT_OF"
    #[serde(rename="DESCENDANT_OF")]
    DESCENDANTOF,
    

    /// Is to be used when SPDXRef-A is a variant of (same lineage but not clear which came first) SPDXRef-B
    ///
    /// "VARIANT_OF"
    #[serde(rename="VARIANT_OF")]
    VARIANTOF,
    

    /// Is to be used when distributing SPDXRef-A requires that SPDXRef-B also be distributed
    ///
    /// "DISTRIBUTION_ARTIFACT"
    #[serde(rename="DISTRIBUTION_ARTIFACT")]
    DISTRIBUTIONARTIFACT,
    

    /// Is to be used when SPDXRef-A is a patch file for (to be applied to) SPDXRef-B
    ///
    /// "PATCH_FOR"
    #[serde(rename="PATCH_FOR")]
    PATCHFOR,
    

    /// Is to be used when SPDXRef-A is a patch file that has been applied to SPDXRef-B
    ///
    /// "PATCH_APPLIED"
    #[serde(rename="PATCH_APPLIED")]
    PATCHAPPLIED,
    

    /// Is to be used when SPDXRef-A is an exact copy of SPDXRef-B
    ///
    /// "COPY_OF"
    #[serde(rename="COPY_OF")]
    COPYOF,
    

    /// Is to be used when SPDXRef-A is a file that was added to SPDXRef-B
    ///
    /// "FILE_ADDED"
    #[serde(rename="FILE_ADDED")]
    FILEADDED,
    

    /// Is to be used when SPDXRef-A is a file that was deleted from SPDXRef-B
    ///
    /// "FILE_DELETED"
    #[serde(rename="FILE_DELETED")]
    FILEDELETED,
    

    /// Is to be used when SPDXRef-A is a file that was modified from SPDXRef-B
    ///
    /// "FILE_MODIFIED"
    #[serde(rename="FILE_MODIFIED")]
    FILEMODIFIED,
    

    /// Is to be used when SPDXRef-A is expanded from the archive SPDXRef-B
    ///
    /// "EXPANDED_FROM_ARCHIVE"
    #[serde(rename="EXPANDED_FROM_ARCHIVE")]
    EXPANDEDFROMARCHIVE,
    

    /// Is to be used when SPDXRef-A dynamically links to SPDXRef-B
    ///
    /// "DYNAMIC_LINK"
    #[serde(rename="DYNAMIC_LINK")]
    DYNAMICLINK,
    

    /// Is to be used when SPDXRef-A statically links to SPDXRef-B
    ///
    /// "STATIC_LINK"
    #[serde(rename="STATIC_LINK")]
    STATICLINK,
    

    /// Is to be used when SPDXRef-A is a data file used in SPDXRef-B
    ///
    /// "DATA_FILE_OF"
    #[serde(rename="DATA_FILE_OF")]
    DATAFILEOF,
    

    /// Is to be used when SPDXRef-A is a test case used in testing SPDXRef-B
    ///
    /// "TEST_CASE_OF"
    #[serde(rename="TEST_CASE_OF")]
    TESTCASEOF,
    

    /// Is to be used when SPDXRef-A is used to build SPDXRef-B
    ///
    /// "BUILD_TOOL_OF"
    #[serde(rename="BUILD_TOOL_OF")]
    BUILDTOOLOF,
    

    /// Is to be used when SPDXRef-A is used as a development tool for SPDXRef-B
    ///
    /// "DEV_TOOL_OF"
    #[serde(rename="DEV_TOOL_OF")]
    DEVTOOLOF,
    

    /// Is to be used when SPDXRef-A is used for testing SPDXRef-B
    ///
    /// "TEST_OF"
    #[serde(rename="TEST_OF")]
    TESTOF,
    

    /// Is to be used when SPDXRef-A is used as a test tool for SPDXRef-B
    ///
    /// "TEST_TOOL_OF"
    #[serde(rename="TEST_TOOL_OF")]
    TESTTOOLOF,
    

    /// Is to be used when SPDXRef-A provides documentation of SPDXRef-B
    ///
    /// "DOCUMENTATION_OF"
    #[serde(rename="DOCUMENTATION_OF")]
    DOCUMENTATIONOF,
    

    /// Is to be used when SPDXRef-A is an optional component of SPDXRef-B
    ///
    /// "OPTIONAL_COMPONENT_OF"
    #[serde(rename="OPTIONAL_COMPONENT_OF")]
    OPTIONALCOMPONENTOF,
    

    /// Is to be used when SPDXRef-A is a metafile of SPDXRef-B
    ///
    /// "METAFILE_OF"
    #[serde(rename="METAFILE_OF")]
    METAFILEOF,
    

    /// Is to be used when SPDXRef-A is used as a package as part of SPDXRef-B
    ///
    /// "PACKAGE_OF"
    #[serde(rename="PACKAGE_OF")]
    PACKAGEOF,
    

    /// Is to be used when (current) SPDXRef-DOCUMENT amends the SPDX information in SPDXRef-B
    ///
    /// "AMENDS"
    #[serde(rename="AMENDS")]
    AMENDS,
    

    /// Is to be used when SPDXRef-A is a prerequisite for SPDXRef-B
    ///
    /// "PREREQUISITE_FOR"
    #[serde(rename="PREREQUISITE_FOR")]
    PREREQUISITEFOR,
    

    /// Is to be used when SPDXRef-A has as a prerequisite SPDXRef-B
    ///
    /// "HAS_PREREQUISITE"
    #[serde(rename="HAS_PREREQUISITE")]
    HASPREREQUISITE,
    

    /// Is to be used for a relationship which has not been defined in the formal SPDX specification. A description of the relationship should be included in the Relationship comments field
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for RelationshipNoteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RelationshipNoteTypeEnum::RELATIONSHIPTYPEUNSPECIFIED => "RELATIONSHIP_TYPE_UNSPECIFIED",
            RelationshipNoteTypeEnum::DESCRIBES => "DESCRIBES",
            RelationshipNoteTypeEnum::DESCRIBEDBY => "DESCRIBED_BY",
            RelationshipNoteTypeEnum::CONTAINS => "CONTAINS",
            RelationshipNoteTypeEnum::CONTAINEDBY => "CONTAINED_BY",
            RelationshipNoteTypeEnum::DEPENDSON => "DEPENDS_ON",
            RelationshipNoteTypeEnum::DEPENDENCYOF => "DEPENDENCY_OF",
            RelationshipNoteTypeEnum::DEPENDENCYMANIFESTOF => "DEPENDENCY_MANIFEST_OF",
            RelationshipNoteTypeEnum::BUILDDEPENDENCYOF => "BUILD_DEPENDENCY_OF",
            RelationshipNoteTypeEnum::DEVDEPENDENCYOF => "DEV_DEPENDENCY_OF",
            RelationshipNoteTypeEnum::OPTIONALDEPENDENCYOF => "OPTIONAL_DEPENDENCY_OF",
            RelationshipNoteTypeEnum::PROVIDEDDEPENDENCYOF => "PROVIDED_DEPENDENCY_OF",
            RelationshipNoteTypeEnum::TESTDEPENDENCYOF => "TEST_DEPENDENCY_OF",
            RelationshipNoteTypeEnum::RUNTIMEDEPENDENCYOF => "RUNTIME_DEPENDENCY_OF",
            RelationshipNoteTypeEnum::EXAMPLEOF => "EXAMPLE_OF",
            RelationshipNoteTypeEnum::GENERATES => "GENERATES",
            RelationshipNoteTypeEnum::GENERATEDFROM => "GENERATED_FROM",
            RelationshipNoteTypeEnum::ANCESTOROF => "ANCESTOR_OF",
            RelationshipNoteTypeEnum::DESCENDANTOF => "DESCENDANT_OF",
            RelationshipNoteTypeEnum::VARIANTOF => "VARIANT_OF",
            RelationshipNoteTypeEnum::DISTRIBUTIONARTIFACT => "DISTRIBUTION_ARTIFACT",
            RelationshipNoteTypeEnum::PATCHFOR => "PATCH_FOR",
            RelationshipNoteTypeEnum::PATCHAPPLIED => "PATCH_APPLIED",
            RelationshipNoteTypeEnum::COPYOF => "COPY_OF",
            RelationshipNoteTypeEnum::FILEADDED => "FILE_ADDED",
            RelationshipNoteTypeEnum::FILEDELETED => "FILE_DELETED",
            RelationshipNoteTypeEnum::FILEMODIFIED => "FILE_MODIFIED",
            RelationshipNoteTypeEnum::EXPANDEDFROMARCHIVE => "EXPANDED_FROM_ARCHIVE",
            RelationshipNoteTypeEnum::DYNAMICLINK => "DYNAMIC_LINK",
            RelationshipNoteTypeEnum::STATICLINK => "STATIC_LINK",
            RelationshipNoteTypeEnum::DATAFILEOF => "DATA_FILE_OF",
            RelationshipNoteTypeEnum::TESTCASEOF => "TEST_CASE_OF",
            RelationshipNoteTypeEnum::BUILDTOOLOF => "BUILD_TOOL_OF",
            RelationshipNoteTypeEnum::DEVTOOLOF => "DEV_TOOL_OF",
            RelationshipNoteTypeEnum::TESTOF => "TEST_OF",
            RelationshipNoteTypeEnum::TESTTOOLOF => "TEST_TOOL_OF",
            RelationshipNoteTypeEnum::DOCUMENTATIONOF => "DOCUMENTATION_OF",
            RelationshipNoteTypeEnum::OPTIONALCOMPONENTOF => "OPTIONAL_COMPONENT_OF",
            RelationshipNoteTypeEnum::METAFILEOF => "METAFILE_OF",
            RelationshipNoteTypeEnum::PACKAGEOF => "PACKAGE_OF",
            RelationshipNoteTypeEnum::AMENDS => "AMENDS",
            RelationshipNoteTypeEnum::PREREQUISITEFOR => "PREREQUISITE_FOR",
            RelationshipNoteTypeEnum::HASPREREQUISITE => "HAS_PREREQUISITE",
            RelationshipNoteTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for RelationshipNoteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATIONSHIP_TYPE_UNSPECIFIED" => Ok(RelationshipNoteTypeEnum::RELATIONSHIPTYPEUNSPECIFIED),
           "DESCRIBES" => Ok(RelationshipNoteTypeEnum::DESCRIBES),
           "DESCRIBED_BY" => Ok(RelationshipNoteTypeEnum::DESCRIBEDBY),
           "CONTAINS" => Ok(RelationshipNoteTypeEnum::CONTAINS),
           "CONTAINED_BY" => Ok(RelationshipNoteTypeEnum::CONTAINEDBY),
           "DEPENDS_ON" => Ok(RelationshipNoteTypeEnum::DEPENDSON),
           "DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::DEPENDENCYOF),
           "DEPENDENCY_MANIFEST_OF" => Ok(RelationshipNoteTypeEnum::DEPENDENCYMANIFESTOF),
           "BUILD_DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::BUILDDEPENDENCYOF),
           "DEV_DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::DEVDEPENDENCYOF),
           "OPTIONAL_DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::OPTIONALDEPENDENCYOF),
           "PROVIDED_DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::PROVIDEDDEPENDENCYOF),
           "TEST_DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::TESTDEPENDENCYOF),
           "RUNTIME_DEPENDENCY_OF" => Ok(RelationshipNoteTypeEnum::RUNTIMEDEPENDENCYOF),
           "EXAMPLE_OF" => Ok(RelationshipNoteTypeEnum::EXAMPLEOF),
           "GENERATES" => Ok(RelationshipNoteTypeEnum::GENERATES),
           "GENERATED_FROM" => Ok(RelationshipNoteTypeEnum::GENERATEDFROM),
           "ANCESTOR_OF" => Ok(RelationshipNoteTypeEnum::ANCESTOROF),
           "DESCENDANT_OF" => Ok(RelationshipNoteTypeEnum::DESCENDANTOF),
           "VARIANT_OF" => Ok(RelationshipNoteTypeEnum::VARIANTOF),
           "DISTRIBUTION_ARTIFACT" => Ok(RelationshipNoteTypeEnum::DISTRIBUTIONARTIFACT),
           "PATCH_FOR" => Ok(RelationshipNoteTypeEnum::PATCHFOR),
           "PATCH_APPLIED" => Ok(RelationshipNoteTypeEnum::PATCHAPPLIED),
           "COPY_OF" => Ok(RelationshipNoteTypeEnum::COPYOF),
           "FILE_ADDED" => Ok(RelationshipNoteTypeEnum::FILEADDED),
           "FILE_DELETED" => Ok(RelationshipNoteTypeEnum::FILEDELETED),
           "FILE_MODIFIED" => Ok(RelationshipNoteTypeEnum::FILEMODIFIED),
           "EXPANDED_FROM_ARCHIVE" => Ok(RelationshipNoteTypeEnum::EXPANDEDFROMARCHIVE),
           "DYNAMIC_LINK" => Ok(RelationshipNoteTypeEnum::DYNAMICLINK),
           "STATIC_LINK" => Ok(RelationshipNoteTypeEnum::STATICLINK),
           "DATA_FILE_OF" => Ok(RelationshipNoteTypeEnum::DATAFILEOF),
           "TEST_CASE_OF" => Ok(RelationshipNoteTypeEnum::TESTCASEOF),
           "BUILD_TOOL_OF" => Ok(RelationshipNoteTypeEnum::BUILDTOOLOF),
           "DEV_TOOL_OF" => Ok(RelationshipNoteTypeEnum::DEVTOOLOF),
           "TEST_OF" => Ok(RelationshipNoteTypeEnum::TESTOF),
           "TEST_TOOL_OF" => Ok(RelationshipNoteTypeEnum::TESTTOOLOF),
           "DOCUMENTATION_OF" => Ok(RelationshipNoteTypeEnum::DOCUMENTATIONOF),
           "OPTIONAL_COMPONENT_OF" => Ok(RelationshipNoteTypeEnum::OPTIONALCOMPONENTOF),
           "METAFILE_OF" => Ok(RelationshipNoteTypeEnum::METAFILEOF),
           "PACKAGE_OF" => Ok(RelationshipNoteTypeEnum::PACKAGEOF),
           "AMENDS" => Ok(RelationshipNoteTypeEnum::AMENDS),
           "PREREQUISITE_FOR" => Ok(RelationshipNoteTypeEnum::PREREQUISITEFOR),
           "HAS_PREREQUISITE" => Ok(RelationshipNoteTypeEnum::HASPREREQUISITE),
           "OTHER" => Ok(RelationshipNoteTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RelationshipNoteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RelationshipOccurrenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of relationship between the source and target SPDX elements
pub enum RelationshipOccurrenceTypeEnum {
    

    /// Unspecified
    ///
    /// "RELATIONSHIP_TYPE_UNSPECIFIED"
    #[serde(rename="RELATIONSHIP_TYPE_UNSPECIFIED")]
    RELATIONSHIPTYPEUNSPECIFIED,
    

    /// Is to be used when SPDXRef-DOCUMENT describes SPDXRef-A
    ///
    /// "DESCRIBES"
    #[serde(rename="DESCRIBES")]
    DESCRIBES,
    

    /// Is to be used when SPDXRef-A is described by SPDXREF-Document
    ///
    /// "DESCRIBED_BY"
    #[serde(rename="DESCRIBED_BY")]
    DESCRIBEDBY,
    

    /// Is to be used when SPDXRef-A contains SPDXRef-B
    ///
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    

    /// Is to be used when SPDXRef-A is contained by SPDXRef-B
    ///
    /// "CONTAINED_BY"
    #[serde(rename="CONTAINED_BY")]
    CONTAINEDBY,
    

    /// Is to be used when SPDXRef-A depends on SPDXRef-B
    ///
    /// "DEPENDS_ON"
    #[serde(rename="DEPENDS_ON")]
    DEPENDSON,
    

    /// Is to be used when SPDXRef-A is dependency of SPDXRef-B
    ///
    /// "DEPENDENCY_OF"
    #[serde(rename="DEPENDENCY_OF")]
    DEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a manifest file that lists a set of dependencies for SPDXRef-B
    ///
    /// "DEPENDENCY_MANIFEST_OF"
    #[serde(rename="DEPENDENCY_MANIFEST_OF")]
    DEPENDENCYMANIFESTOF,
    

    /// Is to be used when SPDXRef-A is a build dependency of SPDXRef-B
    ///
    /// "BUILD_DEPENDENCY_OF"
    #[serde(rename="BUILD_DEPENDENCY_OF")]
    BUILDDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a development dependency of SPDXRef-B
    ///
    /// "DEV_DEPENDENCY_OF"
    #[serde(rename="DEV_DEPENDENCY_OF")]
    DEVDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is an optional dependency of SPDXRef-B
    ///
    /// "OPTIONAL_DEPENDENCY_OF"
    #[serde(rename="OPTIONAL_DEPENDENCY_OF")]
    OPTIONALDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a to be provided dependency of SPDXRef-B
    ///
    /// "PROVIDED_DEPENDENCY_OF"
    #[serde(rename="PROVIDED_DEPENDENCY_OF")]
    PROVIDEDDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a test dependency of SPDXRef-B
    ///
    /// "TEST_DEPENDENCY_OF"
    #[serde(rename="TEST_DEPENDENCY_OF")]
    TESTDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is a dependency required for the execution of SPDXRef-B
    ///
    /// "RUNTIME_DEPENDENCY_OF"
    #[serde(rename="RUNTIME_DEPENDENCY_OF")]
    RUNTIMEDEPENDENCYOF,
    

    /// Is to be used when SPDXRef-A is an example of SPDXRef-B
    ///
    /// "EXAMPLE_OF"
    #[serde(rename="EXAMPLE_OF")]
    EXAMPLEOF,
    

    /// Is to be used when SPDXRef-A generates SPDXRef-B
    ///
    /// "GENERATES"
    #[serde(rename="GENERATES")]
    GENERATES,
    

    /// Is to be used when SPDXRef-A was generated from SPDXRef-B
    ///
    /// "GENERATED_FROM"
    #[serde(rename="GENERATED_FROM")]
    GENERATEDFROM,
    

    /// Is to be used when SPDXRef-A is an ancestor (same lineage but pre-dates) SPDXRef-B
    ///
    /// "ANCESTOR_OF"
    #[serde(rename="ANCESTOR_OF")]
    ANCESTOROF,
    

    /// Is to be used when SPDXRef-A is a descendant of (same lineage but postdates) SPDXRef-B
    ///
    /// "DESCENDANT_OF"
    #[serde(rename="DESCENDANT_OF")]
    DESCENDANTOF,
    

    /// Is to be used when SPDXRef-A is a variant of (same lineage but not clear which came first) SPDXRef-B
    ///
    /// "VARIANT_OF"
    #[serde(rename="VARIANT_OF")]
    VARIANTOF,
    

    /// Is to be used when distributing SPDXRef-A requires that SPDXRef-B also be distributed
    ///
    /// "DISTRIBUTION_ARTIFACT"
    #[serde(rename="DISTRIBUTION_ARTIFACT")]
    DISTRIBUTIONARTIFACT,
    

    /// Is to be used when SPDXRef-A is a patch file for (to be applied to) SPDXRef-B
    ///
    /// "PATCH_FOR"
    #[serde(rename="PATCH_FOR")]
    PATCHFOR,
    

    /// Is to be used when SPDXRef-A is a patch file that has been applied to SPDXRef-B
    ///
    /// "PATCH_APPLIED"
    #[serde(rename="PATCH_APPLIED")]
    PATCHAPPLIED,
    

    /// Is to be used when SPDXRef-A is an exact copy of SPDXRef-B
    ///
    /// "COPY_OF"
    #[serde(rename="COPY_OF")]
    COPYOF,
    

    /// Is to be used when SPDXRef-A is a file that was added to SPDXRef-B
    ///
    /// "FILE_ADDED"
    #[serde(rename="FILE_ADDED")]
    FILEADDED,
    

    /// Is to be used when SPDXRef-A is a file that was deleted from SPDXRef-B
    ///
    /// "FILE_DELETED"
    #[serde(rename="FILE_DELETED")]
    FILEDELETED,
    

    /// Is to be used when SPDXRef-A is a file that was modified from SPDXRef-B
    ///
    /// "FILE_MODIFIED"
    #[serde(rename="FILE_MODIFIED")]
    FILEMODIFIED,
    

    /// Is to be used when SPDXRef-A is expanded from the archive SPDXRef-B
    ///
    /// "EXPANDED_FROM_ARCHIVE"
    #[serde(rename="EXPANDED_FROM_ARCHIVE")]
    EXPANDEDFROMARCHIVE,
    

    /// Is to be used when SPDXRef-A dynamically links to SPDXRef-B
    ///
    /// "DYNAMIC_LINK"
    #[serde(rename="DYNAMIC_LINK")]
    DYNAMICLINK,
    

    /// Is to be used when SPDXRef-A statically links to SPDXRef-B
    ///
    /// "STATIC_LINK"
    #[serde(rename="STATIC_LINK")]
    STATICLINK,
    

    /// Is to be used when SPDXRef-A is a data file used in SPDXRef-B
    ///
    /// "DATA_FILE_OF"
    #[serde(rename="DATA_FILE_OF")]
    DATAFILEOF,
    

    /// Is to be used when SPDXRef-A is a test case used in testing SPDXRef-B
    ///
    /// "TEST_CASE_OF"
    #[serde(rename="TEST_CASE_OF")]
    TESTCASEOF,
    

    /// Is to be used when SPDXRef-A is used to build SPDXRef-B
    ///
    /// "BUILD_TOOL_OF"
    #[serde(rename="BUILD_TOOL_OF")]
    BUILDTOOLOF,
    

    /// Is to be used when SPDXRef-A is used as a development tool for SPDXRef-B
    ///
    /// "DEV_TOOL_OF"
    #[serde(rename="DEV_TOOL_OF")]
    DEVTOOLOF,
    

    /// Is to be used when SPDXRef-A is used for testing SPDXRef-B
    ///
    /// "TEST_OF"
    #[serde(rename="TEST_OF")]
    TESTOF,
    

    /// Is to be used when SPDXRef-A is used as a test tool for SPDXRef-B
    ///
    /// "TEST_TOOL_OF"
    #[serde(rename="TEST_TOOL_OF")]
    TESTTOOLOF,
    

    /// Is to be used when SPDXRef-A provides documentation of SPDXRef-B
    ///
    /// "DOCUMENTATION_OF"
    #[serde(rename="DOCUMENTATION_OF")]
    DOCUMENTATIONOF,
    

    /// Is to be used when SPDXRef-A is an optional component of SPDXRef-B
    ///
    /// "OPTIONAL_COMPONENT_OF"
    #[serde(rename="OPTIONAL_COMPONENT_OF")]
    OPTIONALCOMPONENTOF,
    

    /// Is to be used when SPDXRef-A is a metafile of SPDXRef-B
    ///
    /// "METAFILE_OF"
    #[serde(rename="METAFILE_OF")]
    METAFILEOF,
    

    /// Is to be used when SPDXRef-A is used as a package as part of SPDXRef-B
    ///
    /// "PACKAGE_OF"
    #[serde(rename="PACKAGE_OF")]
    PACKAGEOF,
    

    /// Is to be used when (current) SPDXRef-DOCUMENT amends the SPDX information in SPDXRef-B
    ///
    /// "AMENDS"
    #[serde(rename="AMENDS")]
    AMENDS,
    

    /// Is to be used when SPDXRef-A is a prerequisite for SPDXRef-B
    ///
    /// "PREREQUISITE_FOR"
    #[serde(rename="PREREQUISITE_FOR")]
    PREREQUISITEFOR,
    

    /// Is to be used when SPDXRef-A has as a prerequisite SPDXRef-B
    ///
    /// "HAS_PREREQUISITE"
    #[serde(rename="HAS_PREREQUISITE")]
    HASPREREQUISITE,
    

    /// Is to be used for a relationship which has not been defined in the formal SPDX specification. A description of the relationship should be included in the Relationship comments field
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for RelationshipOccurrenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RelationshipOccurrenceTypeEnum::RELATIONSHIPTYPEUNSPECIFIED => "RELATIONSHIP_TYPE_UNSPECIFIED",
            RelationshipOccurrenceTypeEnum::DESCRIBES => "DESCRIBES",
            RelationshipOccurrenceTypeEnum::DESCRIBEDBY => "DESCRIBED_BY",
            RelationshipOccurrenceTypeEnum::CONTAINS => "CONTAINS",
            RelationshipOccurrenceTypeEnum::CONTAINEDBY => "CONTAINED_BY",
            RelationshipOccurrenceTypeEnum::DEPENDSON => "DEPENDS_ON",
            RelationshipOccurrenceTypeEnum::DEPENDENCYOF => "DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::DEPENDENCYMANIFESTOF => "DEPENDENCY_MANIFEST_OF",
            RelationshipOccurrenceTypeEnum::BUILDDEPENDENCYOF => "BUILD_DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::DEVDEPENDENCYOF => "DEV_DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::OPTIONALDEPENDENCYOF => "OPTIONAL_DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::PROVIDEDDEPENDENCYOF => "PROVIDED_DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::TESTDEPENDENCYOF => "TEST_DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::RUNTIMEDEPENDENCYOF => "RUNTIME_DEPENDENCY_OF",
            RelationshipOccurrenceTypeEnum::EXAMPLEOF => "EXAMPLE_OF",
            RelationshipOccurrenceTypeEnum::GENERATES => "GENERATES",
            RelationshipOccurrenceTypeEnum::GENERATEDFROM => "GENERATED_FROM",
            RelationshipOccurrenceTypeEnum::ANCESTOROF => "ANCESTOR_OF",
            RelationshipOccurrenceTypeEnum::DESCENDANTOF => "DESCENDANT_OF",
            RelationshipOccurrenceTypeEnum::VARIANTOF => "VARIANT_OF",
            RelationshipOccurrenceTypeEnum::DISTRIBUTIONARTIFACT => "DISTRIBUTION_ARTIFACT",
            RelationshipOccurrenceTypeEnum::PATCHFOR => "PATCH_FOR",
            RelationshipOccurrenceTypeEnum::PATCHAPPLIED => "PATCH_APPLIED",
            RelationshipOccurrenceTypeEnum::COPYOF => "COPY_OF",
            RelationshipOccurrenceTypeEnum::FILEADDED => "FILE_ADDED",
            RelationshipOccurrenceTypeEnum::FILEDELETED => "FILE_DELETED",
            RelationshipOccurrenceTypeEnum::FILEMODIFIED => "FILE_MODIFIED",
            RelationshipOccurrenceTypeEnum::EXPANDEDFROMARCHIVE => "EXPANDED_FROM_ARCHIVE",
            RelationshipOccurrenceTypeEnum::DYNAMICLINK => "DYNAMIC_LINK",
            RelationshipOccurrenceTypeEnum::STATICLINK => "STATIC_LINK",
            RelationshipOccurrenceTypeEnum::DATAFILEOF => "DATA_FILE_OF",
            RelationshipOccurrenceTypeEnum::TESTCASEOF => "TEST_CASE_OF",
            RelationshipOccurrenceTypeEnum::BUILDTOOLOF => "BUILD_TOOL_OF",
            RelationshipOccurrenceTypeEnum::DEVTOOLOF => "DEV_TOOL_OF",
            RelationshipOccurrenceTypeEnum::TESTOF => "TEST_OF",
            RelationshipOccurrenceTypeEnum::TESTTOOLOF => "TEST_TOOL_OF",
            RelationshipOccurrenceTypeEnum::DOCUMENTATIONOF => "DOCUMENTATION_OF",
            RelationshipOccurrenceTypeEnum::OPTIONALCOMPONENTOF => "OPTIONAL_COMPONENT_OF",
            RelationshipOccurrenceTypeEnum::METAFILEOF => "METAFILE_OF",
            RelationshipOccurrenceTypeEnum::PACKAGEOF => "PACKAGE_OF",
            RelationshipOccurrenceTypeEnum::AMENDS => "AMENDS",
            RelationshipOccurrenceTypeEnum::PREREQUISITEFOR => "PREREQUISITE_FOR",
            RelationshipOccurrenceTypeEnum::HASPREREQUISITE => "HAS_PREREQUISITE",
            RelationshipOccurrenceTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for RelationshipOccurrenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATIONSHIP_TYPE_UNSPECIFIED" => Ok(RelationshipOccurrenceTypeEnum::RELATIONSHIPTYPEUNSPECIFIED),
           "DESCRIBES" => Ok(RelationshipOccurrenceTypeEnum::DESCRIBES),
           "DESCRIBED_BY" => Ok(RelationshipOccurrenceTypeEnum::DESCRIBEDBY),
           "CONTAINS" => Ok(RelationshipOccurrenceTypeEnum::CONTAINS),
           "CONTAINED_BY" => Ok(RelationshipOccurrenceTypeEnum::CONTAINEDBY),
           "DEPENDS_ON" => Ok(RelationshipOccurrenceTypeEnum::DEPENDSON),
           "DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::DEPENDENCYOF),
           "DEPENDENCY_MANIFEST_OF" => Ok(RelationshipOccurrenceTypeEnum::DEPENDENCYMANIFESTOF),
           "BUILD_DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::BUILDDEPENDENCYOF),
           "DEV_DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::DEVDEPENDENCYOF),
           "OPTIONAL_DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::OPTIONALDEPENDENCYOF),
           "PROVIDED_DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::PROVIDEDDEPENDENCYOF),
           "TEST_DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::TESTDEPENDENCYOF),
           "RUNTIME_DEPENDENCY_OF" => Ok(RelationshipOccurrenceTypeEnum::RUNTIMEDEPENDENCYOF),
           "EXAMPLE_OF" => Ok(RelationshipOccurrenceTypeEnum::EXAMPLEOF),
           "GENERATES" => Ok(RelationshipOccurrenceTypeEnum::GENERATES),
           "GENERATED_FROM" => Ok(RelationshipOccurrenceTypeEnum::GENERATEDFROM),
           "ANCESTOR_OF" => Ok(RelationshipOccurrenceTypeEnum::ANCESTOROF),
           "DESCENDANT_OF" => Ok(RelationshipOccurrenceTypeEnum::DESCENDANTOF),
           "VARIANT_OF" => Ok(RelationshipOccurrenceTypeEnum::VARIANTOF),
           "DISTRIBUTION_ARTIFACT" => Ok(RelationshipOccurrenceTypeEnum::DISTRIBUTIONARTIFACT),
           "PATCH_FOR" => Ok(RelationshipOccurrenceTypeEnum::PATCHFOR),
           "PATCH_APPLIED" => Ok(RelationshipOccurrenceTypeEnum::PATCHAPPLIED),
           "COPY_OF" => Ok(RelationshipOccurrenceTypeEnum::COPYOF),
           "FILE_ADDED" => Ok(RelationshipOccurrenceTypeEnum::FILEADDED),
           "FILE_DELETED" => Ok(RelationshipOccurrenceTypeEnum::FILEDELETED),
           "FILE_MODIFIED" => Ok(RelationshipOccurrenceTypeEnum::FILEMODIFIED),
           "EXPANDED_FROM_ARCHIVE" => Ok(RelationshipOccurrenceTypeEnum::EXPANDEDFROMARCHIVE),
           "DYNAMIC_LINK" => Ok(RelationshipOccurrenceTypeEnum::DYNAMICLINK),
           "STATIC_LINK" => Ok(RelationshipOccurrenceTypeEnum::STATICLINK),
           "DATA_FILE_OF" => Ok(RelationshipOccurrenceTypeEnum::DATAFILEOF),
           "TEST_CASE_OF" => Ok(RelationshipOccurrenceTypeEnum::TESTCASEOF),
           "BUILD_TOOL_OF" => Ok(RelationshipOccurrenceTypeEnum::BUILDTOOLOF),
           "DEV_TOOL_OF" => Ok(RelationshipOccurrenceTypeEnum::DEVTOOLOF),
           "TEST_OF" => Ok(RelationshipOccurrenceTypeEnum::TESTOF),
           "TEST_TOOL_OF" => Ok(RelationshipOccurrenceTypeEnum::TESTTOOLOF),
           "DOCUMENTATION_OF" => Ok(RelationshipOccurrenceTypeEnum::DOCUMENTATIONOF),
           "OPTIONAL_COMPONENT_OF" => Ok(RelationshipOccurrenceTypeEnum::OPTIONALCOMPONENTOF),
           "METAFILE_OF" => Ok(RelationshipOccurrenceTypeEnum::METAFILEOF),
           "PACKAGE_OF" => Ok(RelationshipOccurrenceTypeEnum::PACKAGEOF),
           "AMENDS" => Ok(RelationshipOccurrenceTypeEnum::AMENDS),
           "PREREQUISITE_FOR" => Ok(RelationshipOccurrenceTypeEnum::PREREQUISITEFOR),
           "HAS_PREREQUISITE" => Ok(RelationshipOccurrenceTypeEnum::HASPREREQUISITE),
           "OTHER" => Ok(RelationshipOccurrenceTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RelationshipOccurrenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RemediationRemediationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of remediation that can be applied.
pub enum RemediationRemediationTypeEnum {
    

    /// No remediation type specified.
    ///
    /// "REMEDIATION_TYPE_UNSPECIFIED"
    #[serde(rename="REMEDIATION_TYPE_UNSPECIFIED")]
    REMEDIATIONTYPEUNSPECIFIED,
    

    /// A MITIGATION is available.
    ///
    /// "MITIGATION"
    #[serde(rename="MITIGATION")]
    MITIGATION,
    

    /// No fix is planned.
    ///
    /// "NO_FIX_PLANNED"
    #[serde(rename="NO_FIX_PLANNED")]
    NOFIXPLANNED,
    

    /// Not available.
    ///
    /// "NONE_AVAILABLE"
    #[serde(rename="NONE_AVAILABLE")]
    NONEAVAILABLE,
    

    /// A vendor fix is available.
    ///
    /// "VENDOR_FIX"
    #[serde(rename="VENDOR_FIX")]
    VENDORFIX,
    

    /// A workaround is available.
    ///
    /// "WORKAROUND"
    #[serde(rename="WORKAROUND")]
    WORKAROUND,
}

impl AsRef<str> for RemediationRemediationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RemediationRemediationTypeEnum::REMEDIATIONTYPEUNSPECIFIED => "REMEDIATION_TYPE_UNSPECIFIED",
            RemediationRemediationTypeEnum::MITIGATION => "MITIGATION",
            RemediationRemediationTypeEnum::NOFIXPLANNED => "NO_FIX_PLANNED",
            RemediationRemediationTypeEnum::NONEAVAILABLE => "NONE_AVAILABLE",
            RemediationRemediationTypeEnum::VENDORFIX => "VENDOR_FIX",
            RemediationRemediationTypeEnum::WORKAROUND => "WORKAROUND",
        }
    }
}

impl std::convert::TryFrom< &str> for RemediationRemediationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REMEDIATION_TYPE_UNSPECIFIED" => Ok(RemediationRemediationTypeEnum::REMEDIATIONTYPEUNSPECIFIED),
           "MITIGATION" => Ok(RemediationRemediationTypeEnum::MITIGATION),
           "NO_FIX_PLANNED" => Ok(RemediationRemediationTypeEnum::NOFIXPLANNED),
           "NONE_AVAILABLE" => Ok(RemediationRemediationTypeEnum::NONEAVAILABLE),
           "VENDOR_FIX" => Ok(RemediationRemediationTypeEnum::VENDORFIX),
           "WORKAROUND" => Ok(RemediationRemediationTypeEnum::WORKAROUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RemediationRemediationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SBOMStatusSbomStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The progress of the SBOM generation.
pub enum SBOMStatusSbomStateEnum {
    

    /// Default unknown state.
    ///
    /// "SBOM_STATE_UNSPECIFIED"
    #[serde(rename="SBOM_STATE_UNSPECIFIED")]
    SBOMSTATEUNSPECIFIED,
    

    /// SBOM scanning is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// SBOM scanning has completed.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
}

impl AsRef<str> for SBOMStatusSbomStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SBOMStatusSbomStateEnum::SBOMSTATEUNSPECIFIED => "SBOM_STATE_UNSPECIFIED",
            SBOMStatusSbomStateEnum::PENDING => "PENDING",
            SBOMStatusSbomStateEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for SBOMStatusSbomStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SBOM_STATE_UNSPECIFIED" => Ok(SBOMStatusSbomStateEnum::SBOMSTATEUNSPECIFIED),
           "PENDING" => Ok(SBOMStatusSbomStateEnum::PENDING),
           "COMPLETE" => Ok(SBOMStatusSbomStateEnum::COMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SBOMStatusSbomStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VersionKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Distinguishes between sentinel MIN/MAX versions and normal versions.
pub enum VersionKindEnum {
    

    /// Unknown.
    ///
    /// "VERSION_KIND_UNSPECIFIED"
    #[serde(rename="VERSION_KIND_UNSPECIFIED")]
    VERSIONKINDUNSPECIFIED,
    

    /// A standard package version.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// A special version representing negative infinity.
    ///
    /// "MINIMUM"
    #[serde(rename="MINIMUM")]
    MINIMUM,
    

    /// A special version representing positive infinity.
    ///
    /// "MAXIMUM"
    #[serde(rename="MAXIMUM")]
    MAXIMUM,
}

impl AsRef<str> for VersionKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VersionKindEnum::VERSIONKINDUNSPECIFIED => "VERSION_KIND_UNSPECIFIED",
            VersionKindEnum::NORMAL => "NORMAL",
            VersionKindEnum::MINIMUM => "MINIMUM",
            VersionKindEnum::MAXIMUM => "MAXIMUM",
        }
    }
}

impl std::convert::TryFrom< &str> for VersionKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_KIND_UNSPECIFIED" => Ok(VersionKindEnum::VERSIONKINDUNSPECIFIED),
           "NORMAL" => Ok(VersionKindEnum::NORMAL),
           "MINIMUM" => Ok(VersionKindEnum::MINIMUM),
           "MAXIMUM" => Ok(VersionKindEnum::MAXIMUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VersionKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VexAssessmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Provides the state of this Vulnerability assessment.
pub enum VexAssessmentStateEnum {
    

    /// No state is specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// This product is known to be affected by this vulnerability.
    ///
    /// "AFFECTED"
    #[serde(rename="AFFECTED")]
    AFFECTED,
    

    /// This product is known to be not affected by this vulnerability.
    ///
    /// "NOT_AFFECTED"
    #[serde(rename="NOT_AFFECTED")]
    NOTAFFECTED,
    

    /// This product contains a fix for this vulnerability.
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// It is not known yet whether these versions are or are not affected by the vulnerability. However, it is still under investigation.
    ///
    /// "UNDER_INVESTIGATION"
    #[serde(rename="UNDER_INVESTIGATION")]
    UNDERINVESTIGATION,
}

impl AsRef<str> for VexAssessmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VexAssessmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            VexAssessmentStateEnum::AFFECTED => "AFFECTED",
            VexAssessmentStateEnum::NOTAFFECTED => "NOT_AFFECTED",
            VexAssessmentStateEnum::FIXED => "FIXED",
            VexAssessmentStateEnum::UNDERINVESTIGATION => "UNDER_INVESTIGATION",
        }
    }
}

impl std::convert::TryFrom< &str> for VexAssessmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(VexAssessmentStateEnum::STATEUNSPECIFIED),
           "AFFECTED" => Ok(VexAssessmentStateEnum::AFFECTED),
           "NOT_AFFECTED" => Ok(VexAssessmentStateEnum::NOTAFFECTED),
           "FIXED" => Ok(VexAssessmentStateEnum::FIXED),
           "UNDER_INVESTIGATION" => Ok(VexAssessmentStateEnum::UNDERINVESTIGATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VexAssessmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VulnerabilityCvssVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// CVSS version used to populate cvss_score and severity.
pub enum VulnerabilityCvssVersionEnum {
    
    /// "CVSS_VERSION_UNSPECIFIED"
    #[serde(rename="CVSS_VERSION_UNSPECIFIED")]
    CVSSVERSIONUNSPECIFIED,
    
    /// "CVSS_VERSION_2"
    #[serde(rename="CVSS_VERSION_2")]
    CVSSVERSION2,
    
    /// "CVSS_VERSION_3"
    #[serde(rename="CVSS_VERSION_3")]
    CVSSVERSION3,
}

impl AsRef<str> for VulnerabilityCvssVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VulnerabilityCvssVersionEnum::CVSSVERSIONUNSPECIFIED => "CVSS_VERSION_UNSPECIFIED",
            VulnerabilityCvssVersionEnum::CVSSVERSION2 => "CVSS_VERSION_2",
            VulnerabilityCvssVersionEnum::CVSSVERSION3 => "CVSS_VERSION_3",
        }
    }
}

impl std::convert::TryFrom< &str> for VulnerabilityCvssVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CVSS_VERSION_UNSPECIFIED" => Ok(VulnerabilityCvssVersionEnum::CVSSVERSIONUNSPECIFIED),
           "CVSS_VERSION_2" => Ok(VulnerabilityCvssVersionEnum::CVSSVERSION2),
           "CVSS_VERSION_3" => Ok(VulnerabilityCvssVersionEnum::CVSSVERSION3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VulnerabilityCvssVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VulnerabilitySeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Note provider assigned impact of the vulnerability.
pub enum VulnerabilitySeverityEnum {
    

    /// Unknown.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Minimal severity.
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
    

    /// Low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// High severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for VulnerabilitySeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VulnerabilitySeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            VulnerabilitySeverityEnum::MINIMAL => "MINIMAL",
            VulnerabilitySeverityEnum::LOW => "LOW",
            VulnerabilitySeverityEnum::MEDIUM => "MEDIUM",
            VulnerabilitySeverityEnum::HIGH => "HIGH",
            VulnerabilitySeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for VulnerabilitySeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(VulnerabilitySeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(VulnerabilitySeverityEnum::MINIMAL),
           "LOW" => Ok(VulnerabilitySeverityEnum::LOW),
           "MEDIUM" => Ok(VulnerabilitySeverityEnum::MEDIUM),
           "HIGH" => Ok(VulnerabilitySeverityEnum::HIGH),
           "CRITICAL" => Ok(VulnerabilitySeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VulnerabilitySeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


