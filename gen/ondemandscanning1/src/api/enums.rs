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


// region CVSAttackComplexityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CVSAttackComplexityEnum {
    
    /// "ATTACK_COMPLEXITY_UNSPECIFIED"
    #[serde(rename="ATTACK_COMPLEXITY_UNSPECIFIED")]
    ATTACKCOMPLEXITYUNSPECIFIED,
    
    /// "ATTACK_COMPLEXITY_LOW"
    #[serde(rename="ATTACK_COMPLEXITY_LOW")]
    ATTACKCOMPLEXITYLOW,
    
    /// "ATTACK_COMPLEXITY_HIGH"
    #[serde(rename="ATTACK_COMPLEXITY_HIGH")]
    ATTACKCOMPLEXITYHIGH,
    
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
/// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments.
pub enum CVSAttackVectorEnum {
    
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
pub enum CVSAuthenticationEnum {
    
    /// "AUTHENTICATION_UNSPECIFIED"
    #[serde(rename="AUTHENTICATION_UNSPECIFIED")]
    AUTHENTICATIONUNSPECIFIED,
    
    /// "AUTHENTICATION_MULTIPLE"
    #[serde(rename="AUTHENTICATION_MULTIPLE")]
    AUTHENTICATIONMULTIPLE,
    
    /// "AUTHENTICATION_SINGLE"
    #[serde(rename="AUTHENTICATION_SINGLE")]
    AUTHENTICATIONSINGLE,
    
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
pub enum CVSAvailabilityImpactEnum {
    
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
    
    /// "IMPACT_PARTIAL"
    #[serde(rename="IMPACT_PARTIAL")]
    IMPACTPARTIAL,
    
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
pub enum CVSConfidentialityImpactEnum {
    
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
    
    /// "IMPACT_PARTIAL"
    #[serde(rename="IMPACT_PARTIAL")]
    IMPACTPARTIAL,
    
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
pub enum CVSIntegrityImpactEnum {
    
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
    
    /// "IMPACT_PARTIAL"
    #[serde(rename="IMPACT_PARTIAL")]
    IMPACTPARTIAL,
    
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
pub enum CVSPrivilegesRequiredEnum {
    
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
pub enum CVSScopeEnum {
    
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
pub enum CVSUserInteractionEnum {
    
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


// region DeploymentOccurrencePlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Platform hosting this deployment.
pub enum DeploymentOccurrencePlatformEnum {
    

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

impl AsRef<str> for DeploymentOccurrencePlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeploymentOccurrencePlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            DeploymentOccurrencePlatformEnum::GKE => "GKE",
            DeploymentOccurrencePlatformEnum::FLEX => "FLEX",
            DeploymentOccurrencePlatformEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for DeploymentOccurrencePlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(DeploymentOccurrencePlatformEnum::PLATFORMUNSPECIFIED),
           "GKE" => Ok(DeploymentOccurrencePlatformEnum::GKE),
           "FLEX" => Ok(DeploymentOccurrencePlatformEnum::FLEX),
           "CUSTOM" => Ok(DeploymentOccurrencePlatformEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeploymentOccurrencePlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoveryOccurrenceAnalysisStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of discovery for the resource.
pub enum DiscoveryOccurrenceAnalysisStatusEnum {
    

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
    

    /// The resource is known not to be supported.
    ///
    /// "FINISHED_UNSUPPORTED"
    #[serde(rename="FINISHED_UNSUPPORTED")]
    FINISHEDUNSUPPORTED,
}

impl AsRef<str> for DiscoveryOccurrenceAnalysisStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoveryOccurrenceAnalysisStatusEnum::ANALYSISSTATUSUNSPECIFIED => "ANALYSIS_STATUS_UNSPECIFIED",
            DiscoveryOccurrenceAnalysisStatusEnum::PENDING => "PENDING",
            DiscoveryOccurrenceAnalysisStatusEnum::SCANNING => "SCANNING",
            DiscoveryOccurrenceAnalysisStatusEnum::FINISHEDSUCCESS => "FINISHED_SUCCESS",
            DiscoveryOccurrenceAnalysisStatusEnum::COMPLETE => "COMPLETE",
            DiscoveryOccurrenceAnalysisStatusEnum::FINISHEDFAILED => "FINISHED_FAILED",
            DiscoveryOccurrenceAnalysisStatusEnum::FINISHEDUNSUPPORTED => "FINISHED_UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoveryOccurrenceAnalysisStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANALYSIS_STATUS_UNSPECIFIED" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::ANALYSISSTATUSUNSPECIFIED),
           "PENDING" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::PENDING),
           "SCANNING" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::SCANNING),
           "FINISHED_SUCCESS" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::FINISHEDSUCCESS),
           "COMPLETE" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::COMPLETE),
           "FINISHED_FAILED" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::FINISHEDFAILED),
           "FINISHED_UNSUPPORTED" => Ok(DiscoveryOccurrenceAnalysisStatusEnum::FINISHEDUNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoveryOccurrenceAnalysisStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiscoveryOccurrenceContinuousAnalysisEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the resource is continuously analyzed.
pub enum DiscoveryOccurrenceContinuousAnalysisEnum {
    

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

impl AsRef<str> for DiscoveryOccurrenceContinuousAnalysisEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiscoveryOccurrenceContinuousAnalysisEnum::CONTINUOUSANALYSISUNSPECIFIED => "CONTINUOUS_ANALYSIS_UNSPECIFIED",
            DiscoveryOccurrenceContinuousAnalysisEnum::ACTIVE => "ACTIVE",
            DiscoveryOccurrenceContinuousAnalysisEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for DiscoveryOccurrenceContinuousAnalysisEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTINUOUS_ANALYSIS_UNSPECIFIED" => Ok(DiscoveryOccurrenceContinuousAnalysisEnum::CONTINUOUSANALYSISUNSPECIFIED),
           "ACTIVE" => Ok(DiscoveryOccurrenceContinuousAnalysisEnum::ACTIVE),
           "INACTIVE" => Ok(DiscoveryOccurrenceContinuousAnalysisEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiscoveryOccurrenceContinuousAnalysisEnum {
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
    

    /// This represents an available package upgrade.
    ///
    /// "UPGRADE"
    #[serde(rename="UPGRADE")]
    UPGRADE,
    

    /// This represents a Compliance Note
    ///
    /// "COMPLIANCE"
    #[serde(rename="COMPLIANCE")]
    COMPLIANCE,
    

    /// This represents a DSSE attestation Note
    ///
    /// "DSSE_ATTESTATION"
    #[serde(rename="DSSE_ATTESTATION")]
    DSSEATTESTATION,
    

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
            OccurrenceKindEnum::UPGRADE => "UPGRADE",
            OccurrenceKindEnum::COMPLIANCE => "COMPLIANCE",
            OccurrenceKindEnum::DSSEATTESTATION => "DSSE_ATTESTATION",
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
           "UPGRADE" => Ok(OccurrenceKindEnum::UPGRADE),
           "COMPLIANCE" => Ok(OccurrenceKindEnum::COMPLIANCE),
           "DSSE_ATTESTATION" => Ok(OccurrenceKindEnum::DSSEATTESTATION),
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


// region PackageDataPackageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of package: os, maven, go, etc.
pub enum PackageDataPackageTypeEnum {
    
    /// "PACKAGE_TYPE_UNSPECIFIED"
    #[serde(rename="PACKAGE_TYPE_UNSPECIFIED")]
    PACKAGETYPEUNSPECIFIED,
    

    /// Operating System
    ///
    /// "OS"
    #[serde(rename="OS")]
    OS,
    

    /// Java packages from Maven.
    ///
    /// "MAVEN"
    #[serde(rename="MAVEN")]
    MAVEN,
    

    /// Go third-party packages.
    ///
    /// "GO"
    #[serde(rename="GO")]
    GO,
    

    /// Go toolchain + standard library packages.
    ///
    /// "GO_STDLIB"
    #[serde(rename="GO_STDLIB")]
    GOSTDLIB,
    

    /// Python packages.
    ///
    /// "PYPI"
    #[serde(rename="PYPI")]
    PYPI,
    

    /// NPM packages.
    ///
    /// "NPM"
    #[serde(rename="NPM")]
    NPM,
    

    /// Nuget (C#/.NET) packages.
    ///
    /// "NUGET"
    #[serde(rename="NUGET")]
    NUGET,
    

    /// Ruby packges (from RubyGems package manager).
    ///
    /// "RUBYGEMS"
    #[serde(rename="RUBYGEMS")]
    RUBYGEMS,
    

    /// Rust packages from Cargo (Github ecosystem is `RUST`).
    ///
    /// "RUST"
    #[serde(rename="RUST")]
    RUST,
    

    /// PHP packages from Composer package manager.
    ///
    /// "COMPOSER"
    #[serde(rename="COMPOSER")]
    COMPOSER,
}

impl AsRef<str> for PackageDataPackageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PackageDataPackageTypeEnum::PACKAGETYPEUNSPECIFIED => "PACKAGE_TYPE_UNSPECIFIED",
            PackageDataPackageTypeEnum::OS => "OS",
            PackageDataPackageTypeEnum::MAVEN => "MAVEN",
            PackageDataPackageTypeEnum::GO => "GO",
            PackageDataPackageTypeEnum::GOSTDLIB => "GO_STDLIB",
            PackageDataPackageTypeEnum::PYPI => "PYPI",
            PackageDataPackageTypeEnum::NPM => "NPM",
            PackageDataPackageTypeEnum::NUGET => "NUGET",
            PackageDataPackageTypeEnum::RUBYGEMS => "RUBYGEMS",
            PackageDataPackageTypeEnum::RUST => "RUST",
            PackageDataPackageTypeEnum::COMPOSER => "COMPOSER",
        }
    }
}

impl std::convert::TryFrom< &str> for PackageDataPackageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PACKAGE_TYPE_UNSPECIFIED" => Ok(PackageDataPackageTypeEnum::PACKAGETYPEUNSPECIFIED),
           "OS" => Ok(PackageDataPackageTypeEnum::OS),
           "MAVEN" => Ok(PackageDataPackageTypeEnum::MAVEN),
           "GO" => Ok(PackageDataPackageTypeEnum::GO),
           "GO_STDLIB" => Ok(PackageDataPackageTypeEnum::GOSTDLIB),
           "PYPI" => Ok(PackageDataPackageTypeEnum::PYPI),
           "NPM" => Ok(PackageDataPackageTypeEnum::NPM),
           "NUGET" => Ok(PackageDataPackageTypeEnum::NUGET),
           "RUBYGEMS" => Ok(PackageDataPackageTypeEnum::RUBYGEMS),
           "RUST" => Ok(PackageDataPackageTypeEnum::RUST),
           "COMPOSER" => Ok(PackageDataPackageTypeEnum::COMPOSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PackageDataPackageTypeEnum {
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


// region PackageOccurrenceArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The CPU architecture for which packages in this distribution channel were built. Architecture will be blank for language packages.
pub enum PackageOccurrenceArchitectureEnum {
    

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

impl AsRef<str> for PackageOccurrenceArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PackageOccurrenceArchitectureEnum::ARCHITECTUREUNSPECIFIED => "ARCHITECTURE_UNSPECIFIED",
            PackageOccurrenceArchitectureEnum::X86 => "X86",
            PackageOccurrenceArchitectureEnum::X64 => "X64",
        }
    }
}

impl std::convert::TryFrom< &str> for PackageOccurrenceArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARCHITECTURE_UNSPECIFIED" => Ok(PackageOccurrenceArchitectureEnum::ARCHITECTUREUNSPECIFIED),
           "X86" => Ok(PackageOccurrenceArchitectureEnum::X86),
           "X64" => Ok(PackageOccurrenceArchitectureEnum::X64),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PackageOccurrenceArchitectureEnum {
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


// region VulnerabilityOccurrenceCvssVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. CVSS version used to populate cvss_score and severity.
pub enum VulnerabilityOccurrenceCvssVersionEnum {
    
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

impl AsRef<str> for VulnerabilityOccurrenceCvssVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VulnerabilityOccurrenceCvssVersionEnum::CVSSVERSIONUNSPECIFIED => "CVSS_VERSION_UNSPECIFIED",
            VulnerabilityOccurrenceCvssVersionEnum::CVSSVERSION2 => "CVSS_VERSION_2",
            VulnerabilityOccurrenceCvssVersionEnum::CVSSVERSION3 => "CVSS_VERSION_3",
        }
    }
}

impl std::convert::TryFrom< &str> for VulnerabilityOccurrenceCvssVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CVSS_VERSION_UNSPECIFIED" => Ok(VulnerabilityOccurrenceCvssVersionEnum::CVSSVERSIONUNSPECIFIED),
           "CVSS_VERSION_2" => Ok(VulnerabilityOccurrenceCvssVersionEnum::CVSSVERSION2),
           "CVSS_VERSION_3" => Ok(VulnerabilityOccurrenceCvssVersionEnum::CVSSVERSION3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VulnerabilityOccurrenceCvssVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VulnerabilityOccurrenceEffectiveSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity. When there are multiple PackageIssues for this vulnerability, they can have different effective severities because some might be provided by the distro while others are provided by the language ecosystem for a language pack. For this reason, it is advised to use the effective severity on the PackageIssue level. In the case where multiple PackageIssues have differing effective severities, this field should be the highest severity for any of the PackageIssues.
pub enum VulnerabilityOccurrenceEffectiveSeverityEnum {
    

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

impl AsRef<str> for VulnerabilityOccurrenceEffectiveSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VulnerabilityOccurrenceEffectiveSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            VulnerabilityOccurrenceEffectiveSeverityEnum::MINIMAL => "MINIMAL",
            VulnerabilityOccurrenceEffectiveSeverityEnum::LOW => "LOW",
            VulnerabilityOccurrenceEffectiveSeverityEnum::MEDIUM => "MEDIUM",
            VulnerabilityOccurrenceEffectiveSeverityEnum::HIGH => "HIGH",
            VulnerabilityOccurrenceEffectiveSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for VulnerabilityOccurrenceEffectiveSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(VulnerabilityOccurrenceEffectiveSeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(VulnerabilityOccurrenceEffectiveSeverityEnum::MINIMAL),
           "LOW" => Ok(VulnerabilityOccurrenceEffectiveSeverityEnum::LOW),
           "MEDIUM" => Ok(VulnerabilityOccurrenceEffectiveSeverityEnum::MEDIUM),
           "HIGH" => Ok(VulnerabilityOccurrenceEffectiveSeverityEnum::HIGH),
           "CRITICAL" => Ok(VulnerabilityOccurrenceEffectiveSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VulnerabilityOccurrenceEffectiveSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VulnerabilityOccurrenceSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The note provider assigned severity of this vulnerability.
pub enum VulnerabilityOccurrenceSeverityEnum {
    

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

impl AsRef<str> for VulnerabilityOccurrenceSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VulnerabilityOccurrenceSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            VulnerabilityOccurrenceSeverityEnum::MINIMAL => "MINIMAL",
            VulnerabilityOccurrenceSeverityEnum::LOW => "LOW",
            VulnerabilityOccurrenceSeverityEnum::MEDIUM => "MEDIUM",
            VulnerabilityOccurrenceSeverityEnum::HIGH => "HIGH",
            VulnerabilityOccurrenceSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for VulnerabilityOccurrenceSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(VulnerabilityOccurrenceSeverityEnum::SEVERITYUNSPECIFIED),
           "MINIMAL" => Ok(VulnerabilityOccurrenceSeverityEnum::MINIMAL),
           "LOW" => Ok(VulnerabilityOccurrenceSeverityEnum::LOW),
           "MEDIUM" => Ok(VulnerabilityOccurrenceSeverityEnum::MEDIUM),
           "HIGH" => Ok(VulnerabilityOccurrenceSeverityEnum::HIGH),
           "CRITICAL" => Ok(VulnerabilityOccurrenceSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VulnerabilityOccurrenceSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


