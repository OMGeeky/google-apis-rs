use super::*;



// region AssetDiscoveryConfigInclusionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The mode to use for filtering asset discovery.
pub enum AssetDiscoveryConfigInclusionModeEnum {
    

    /// Unspecified. Setting the mode with this value will disable inclusion/exclusion filtering for Asset Discovery.
    ///
    /// "INCLUSION_MODE_UNSPECIFIED"
    #[serde(rename="INCLUSION_MODE_UNSPECIFIED")]
    INCLUSIONMODEUNSPECIFIED,
    

    /// Asset Discovery will capture only the resources within the projects specified. All other resources will be ignored.
    ///
    /// "INCLUDE_ONLY"
    #[serde(rename="INCLUDE_ONLY")]
    INCLUDEONLY,
    

    /// Asset Discovery will ignore all resources under the projects specified. All other resources will be retrieved.
    ///
    /// "EXCLUDE"
    #[serde(rename="EXCLUDE")]
    EXCLUDE,
}

impl AsRef<str> for AssetDiscoveryConfigInclusionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssetDiscoveryConfigInclusionModeEnum::INCLUSIONMODEUNSPECIFIED => "INCLUSION_MODE_UNSPECIFIED",
            AssetDiscoveryConfigInclusionModeEnum::INCLUDEONLY => "INCLUDE_ONLY",
            AssetDiscoveryConfigInclusionModeEnum::EXCLUDE => "EXCLUDE",
        }
    }
}

impl std::convert::TryFrom< &str> for AssetDiscoveryConfigInclusionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INCLUSION_MODE_UNSPECIFIED" => Ok(AssetDiscoveryConfigInclusionModeEnum::INCLUSIONMODEUNSPECIFIED),
           "INCLUDE_ONLY" => Ok(AssetDiscoveryConfigInclusionModeEnum::INCLUDEONLY),
           "EXCLUDE" => Ok(AssetDiscoveryConfigInclusionModeEnum::EXCLUDE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssetDiscoveryConfigInclusionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AttackExposureStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What state this AttackExposure is in. This captures whether or not an attack exposure has been calculated or not.
pub enum AttackExposureStateEnum {
    

    /// The state is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The attack exposure has been calculated.
    ///
    /// "CALCULATED"
    #[serde(rename="CALCULATED")]
    CALCULATED,
    

    /// The attack exposure has not been calculated.
    ///
    /// "NOT_CALCULATED"
    #[serde(rename="NOT_CALCULATED")]
    NOTCALCULATED,
}

impl AsRef<str> for AttackExposureStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttackExposureStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AttackExposureStateEnum::CALCULATED => "CALCULATED",
            AttackExposureStateEnum::NOTCALCULATED => "NOT_CALCULATED",
        }
    }
}

impl std::convert::TryFrom< &str> for AttackExposureStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AttackExposureStateEnum::STATEUNSPECIFIED),
           "CALCULATED" => Ok(AttackExposureStateEnum::CALCULATED),
           "NOT_CALCULATED" => Ok(AttackExposureStateEnum::NOTCALCULATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttackExposureStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AttackStepNodeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Attack step type. Can be either AND, OR or DEFENSE
pub enum AttackStepNodeTypeEnum {
    

    /// Type not specified
    ///
    /// "NODE_TYPE_UNSPECIFIED"
    #[serde(rename="NODE_TYPE_UNSPECIFIED")]
    NODETYPEUNSPECIFIED,
    

    /// Incoming edge joined with AND
    ///
    /// "NODE_TYPE_AND"
    #[serde(rename="NODE_TYPE_AND")]
    NODETYPEAND,
    

    /// Incoming edge joined with OR
    ///
    /// "NODE_TYPE_OR"
    #[serde(rename="NODE_TYPE_OR")]
    NODETYPEOR,
    

    /// Incoming edge is defense
    ///
    /// "NODE_TYPE_DEFENSE"
    #[serde(rename="NODE_TYPE_DEFENSE")]
    NODETYPEDEFENSE,
    

    /// Incoming edge is attacker
    ///
    /// "NODE_TYPE_ATTACKER"
    #[serde(rename="NODE_TYPE_ATTACKER")]
    NODETYPEATTACKER,
}

impl AsRef<str> for AttackStepNodeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AttackStepNodeTypeEnum::NODETYPEUNSPECIFIED => "NODE_TYPE_UNSPECIFIED",
            AttackStepNodeTypeEnum::NODETYPEAND => "NODE_TYPE_AND",
            AttackStepNodeTypeEnum::NODETYPEOR => "NODE_TYPE_OR",
            AttackStepNodeTypeEnum::NODETYPEDEFENSE => "NODE_TYPE_DEFENSE",
            AttackStepNodeTypeEnum::NODETYPEATTACKER => "NODE_TYPE_ATTACKER",
        }
    }
}

impl std::convert::TryFrom< &str> for AttackStepNodeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NODE_TYPE_UNSPECIFIED" => Ok(AttackStepNodeTypeEnum::NODETYPEUNSPECIFIED),
           "NODE_TYPE_AND" => Ok(AttackStepNodeTypeEnum::NODETYPEAND),
           "NODE_TYPE_OR" => Ok(AttackStepNodeTypeEnum::NODETYPEOR),
           "NODE_TYPE_DEFENSE" => Ok(AttackStepNodeTypeEnum::NODETYPEDEFENSE),
           "NODE_TYPE_ATTACKER" => Ok(AttackStepNodeTypeEnum::NODETYPEATTACKER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AttackStepNodeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum AuditLogConfigLogTypeEnum {
    

    /// Default case. Should never be this.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Admin reads. Example: CloudIAM getIamPolicy
    ///
    /// "ADMIN_READ"
    #[serde(rename="ADMIN_READ")]
    ADMINREAD,
    

    /// Data writes. Example: CloudSQL Users create
    ///
    /// "DATA_WRITE"
    #[serde(rename="DATA_WRITE")]
    DATAWRITE,
    

    /// Data reads. Example: CloudSQL Users list
    ///
    /// "DATA_READ"
    #[serde(rename="DATA_READ")]
    DATAREAD,
}

impl AsRef<str> for AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudDlpDataProfileParentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The resource hierarchy level at which the data profile was generated.
pub enum CloudDlpDataProfileParentTypeEnum {
    

    /// Unspecified parent type.
    ///
    /// "PARENT_TYPE_UNSPECIFIED"
    #[serde(rename="PARENT_TYPE_UNSPECIFIED")]
    PARENTTYPEUNSPECIFIED,
    

    /// Organization-level configurations.
    ///
    /// "ORGANIZATION"
    #[serde(rename="ORGANIZATION")]
    ORGANIZATION,
    

    /// Project-level configurations.
    ///
    /// "PROJECT"
    #[serde(rename="PROJECT")]
    PROJECT,
}

impl AsRef<str> for CloudDlpDataProfileParentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudDlpDataProfileParentTypeEnum::PARENTTYPEUNSPECIFIED => "PARENT_TYPE_UNSPECIFIED",
            CloudDlpDataProfileParentTypeEnum::ORGANIZATION => "ORGANIZATION",
            CloudDlpDataProfileParentTypeEnum::PROJECT => "PROJECT",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudDlpDataProfileParentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARENT_TYPE_UNSPECIFIED" => Ok(CloudDlpDataProfileParentTypeEnum::PARENTTYPEUNSPECIFIED),
           "ORGANIZATION" => Ok(CloudDlpDataProfileParentTypeEnum::ORGANIZATION),
           "PROJECT" => Ok(CloudDlpDataProfileParentTypeEnum::PROJECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudDlpDataProfileParentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// IANA Internet Protocol Number such as TCP(6) and UDP(17).
pub enum ConnectionProtocolEnum {
    

    /// Unspecified protocol (not HOPOPT).
    ///
    /// "PROTOCOL_UNSPECIFIED"
    #[serde(rename="PROTOCOL_UNSPECIFIED")]
    PROTOCOLUNSPECIFIED,
    

    /// Internet Control Message Protocol.
    ///
    /// "ICMP"
    #[serde(rename="ICMP")]
    ICMP,
    

    /// Transmission Control Protocol.
    ///
    /// "TCP"
    #[serde(rename="TCP")]
    TCP,
    

    /// User Datagram Protocol.
    ///
    /// "UDP"
    #[serde(rename="UDP")]
    UDP,
    

    /// Generic Routing Encapsulation.
    ///
    /// "GRE"
    #[serde(rename="GRE")]
    GRE,
    

    /// Encap Security Payload.
    ///
    /// "ESP"
    #[serde(rename="ESP")]
    ESP,
}

impl AsRef<str> for ConnectionProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionProtocolEnum::PROTOCOLUNSPECIFIED => "PROTOCOL_UNSPECIFIED",
            ConnectionProtocolEnum::ICMP => "ICMP",
            ConnectionProtocolEnum::TCP => "TCP",
            ConnectionProtocolEnum::UDP => "UDP",
            ConnectionProtocolEnum::GRE => "GRE",
            ConnectionProtocolEnum::ESP => "ESP",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTOCOL_UNSPECIFIED" => Ok(ConnectionProtocolEnum::PROTOCOLUNSPECIFIED),
           "ICMP" => Ok(ConnectionProtocolEnum::ICMP),
           "TCP" => Ok(ConnectionProtocolEnum::TCP),
           "UDP" => Ok(ConnectionProtocolEnum::UDP),
           "GRE" => Ok(ConnectionProtocolEnum::GRE),
           "ESP" => Ok(ConnectionProtocolEnum::ESP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CveExploitationActivityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The exploitation activity of the vulnerability in the wild.
pub enum CveExploitationActivityEnum {
    

    /// Invalid or empty value.
    ///
    /// "EXPLOITATION_ACTIVITY_UNSPECIFIED"
    #[serde(rename="EXPLOITATION_ACTIVITY_UNSPECIFIED")]
    EXPLOITATIONACTIVITYUNSPECIFIED,
    

    /// Exploitation has been reported or confirmed to widely occur.
    ///
    /// "WIDE"
    #[serde(rename="WIDE")]
    WIDE,
    

    /// Limited reported or confirmed exploitation activities.
    ///
    /// "CONFIRMED"
    #[serde(rename="CONFIRMED")]
    CONFIRMED,
    

    /// Exploit is publicly available.
    ///
    /// "AVAILABLE"
    #[serde(rename="AVAILABLE")]
    AVAILABLE,
    

    /// No known exploitation activity, but has a high potential for exploitation.
    ///
    /// "ANTICIPATED"
    #[serde(rename="ANTICIPATED")]
    ANTICIPATED,
    

    /// No known exploitation activity.
    ///
    /// "NO_KNOWN"
    #[serde(rename="NO_KNOWN")]
    NOKNOWN,
}

impl AsRef<str> for CveExploitationActivityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CveExploitationActivityEnum::EXPLOITATIONACTIVITYUNSPECIFIED => "EXPLOITATION_ACTIVITY_UNSPECIFIED",
            CveExploitationActivityEnum::WIDE => "WIDE",
            CveExploitationActivityEnum::CONFIRMED => "CONFIRMED",
            CveExploitationActivityEnum::AVAILABLE => "AVAILABLE",
            CveExploitationActivityEnum::ANTICIPATED => "ANTICIPATED",
            CveExploitationActivityEnum::NOKNOWN => "NO_KNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for CveExploitationActivityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPLOITATION_ACTIVITY_UNSPECIFIED" => Ok(CveExploitationActivityEnum::EXPLOITATIONACTIVITYUNSPECIFIED),
           "WIDE" => Ok(CveExploitationActivityEnum::WIDE),
           "CONFIRMED" => Ok(CveExploitationActivityEnum::CONFIRMED),
           "AVAILABLE" => Ok(CveExploitationActivityEnum::AVAILABLE),
           "ANTICIPATED" => Ok(CveExploitationActivityEnum::ANTICIPATED),
           "NO_KNOWN" => Ok(CveExploitationActivityEnum::NOKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CveExploitationActivityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CveImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The potential impact of the vulnerability if it was to be exploited.
pub enum CveImpactEnum {
    

    /// Invalid or empty value.
    ///
    /// "RISK_RATING_UNSPECIFIED"
    #[serde(rename="RISK_RATING_UNSPECIFIED")]
    RISKRATINGUNSPECIFIED,
    

    /// Exploitation would have little to no security impact.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Exploitation would enable attackers to perform activities, or could allow attackers to have a direct impact, but would require additional steps.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Exploitation would enable attackers to have a notable direct impact without needing to overcome any major mitigating factors.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Exploitation would fundamentally undermine the security of affected systems, enable actors to perform significant attacks with minimal effort, with little to no mitigating factors to overcome.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for CveImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CveImpactEnum::RISKRATINGUNSPECIFIED => "RISK_RATING_UNSPECIFIED",
            CveImpactEnum::LOW => "LOW",
            CveImpactEnum::MEDIUM => "MEDIUM",
            CveImpactEnum::HIGH => "HIGH",
            CveImpactEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CveImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RISK_RATING_UNSPECIFIED" => Ok(CveImpactEnum::RISKRATINGUNSPECIFIED),
           "LOW" => Ok(CveImpactEnum::LOW),
           "MEDIUM" => Ok(CveImpactEnum::MEDIUM),
           "HIGH" => Ok(CveImpactEnum::HIGH),
           "CRITICAL" => Ok(CveImpactEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CveImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3AttackComplexityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This metric describes the conditions beyond the attacker's control that must exist in order to exploit the vulnerability.
pub enum Cvssv3AttackComplexityEnum {
    

    /// Invalid value.
    ///
    /// "ATTACK_COMPLEXITY_UNSPECIFIED"
    #[serde(rename="ATTACK_COMPLEXITY_UNSPECIFIED")]
    ATTACKCOMPLEXITYUNSPECIFIED,
    

    /// Specialized access conditions or extenuating circumstances do not exist. An attacker can expect repeatable success when attacking the vulnerable component.
    ///
    /// "ATTACK_COMPLEXITY_LOW"
    #[serde(rename="ATTACK_COMPLEXITY_LOW")]
    ATTACKCOMPLEXITYLOW,
    

    /// A successful attack depends on conditions beyond the attacker's control. That is, a successful attack cannot be accomplished at will, but requires the attacker to invest in some measurable amount of effort in preparation or execution against the vulnerable component before a successful attack can be expected.
    ///
    /// "ATTACK_COMPLEXITY_HIGH"
    #[serde(rename="ATTACK_COMPLEXITY_HIGH")]
    ATTACKCOMPLEXITYHIGH,
}

impl AsRef<str> for Cvssv3AttackComplexityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3AttackComplexityEnum::ATTACKCOMPLEXITYUNSPECIFIED => "ATTACK_COMPLEXITY_UNSPECIFIED",
            Cvssv3AttackComplexityEnum::ATTACKCOMPLEXITYLOW => "ATTACK_COMPLEXITY_LOW",
            Cvssv3AttackComplexityEnum::ATTACKCOMPLEXITYHIGH => "ATTACK_COMPLEXITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3AttackComplexityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTACK_COMPLEXITY_UNSPECIFIED" => Ok(Cvssv3AttackComplexityEnum::ATTACKCOMPLEXITYUNSPECIFIED),
           "ATTACK_COMPLEXITY_LOW" => Ok(Cvssv3AttackComplexityEnum::ATTACKCOMPLEXITYLOW),
           "ATTACK_COMPLEXITY_HIGH" => Ok(Cvssv3AttackComplexityEnum::ATTACKCOMPLEXITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3AttackComplexityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3AttackVectorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments. This metric reflects the context by which vulnerability exploitation is possible.
pub enum Cvssv3AttackVectorEnum {
    

    /// Invalid value.
    ///
    /// "ATTACK_VECTOR_UNSPECIFIED"
    #[serde(rename="ATTACK_VECTOR_UNSPECIFIED")]
    ATTACKVECTORUNSPECIFIED,
    

    /// The vulnerable component is bound to the network stack and the set of possible attackers extends beyond the other options listed below, up to and including the entire Internet.
    ///
    /// "ATTACK_VECTOR_NETWORK"
    #[serde(rename="ATTACK_VECTOR_NETWORK")]
    ATTACKVECTORNETWORK,
    

    /// The vulnerable component is bound to the network stack, but the attack is limited at the protocol level to a logically adjacent topology.
    ///
    /// "ATTACK_VECTOR_ADJACENT"
    #[serde(rename="ATTACK_VECTOR_ADJACENT")]
    ATTACKVECTORADJACENT,
    

    /// The vulnerable component is not bound to the network stack and the attacker's path is via read/write/execute capabilities.
    ///
    /// "ATTACK_VECTOR_LOCAL"
    #[serde(rename="ATTACK_VECTOR_LOCAL")]
    ATTACKVECTORLOCAL,
    

    /// The attack requires the attacker to physically touch or manipulate the vulnerable component.
    ///
    /// "ATTACK_VECTOR_PHYSICAL"
    #[serde(rename="ATTACK_VECTOR_PHYSICAL")]
    ATTACKVECTORPHYSICAL,
}

impl AsRef<str> for Cvssv3AttackVectorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3AttackVectorEnum::ATTACKVECTORUNSPECIFIED => "ATTACK_VECTOR_UNSPECIFIED",
            Cvssv3AttackVectorEnum::ATTACKVECTORNETWORK => "ATTACK_VECTOR_NETWORK",
            Cvssv3AttackVectorEnum::ATTACKVECTORADJACENT => "ATTACK_VECTOR_ADJACENT",
            Cvssv3AttackVectorEnum::ATTACKVECTORLOCAL => "ATTACK_VECTOR_LOCAL",
            Cvssv3AttackVectorEnum::ATTACKVECTORPHYSICAL => "ATTACK_VECTOR_PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3AttackVectorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTACK_VECTOR_UNSPECIFIED" => Ok(Cvssv3AttackVectorEnum::ATTACKVECTORUNSPECIFIED),
           "ATTACK_VECTOR_NETWORK" => Ok(Cvssv3AttackVectorEnum::ATTACKVECTORNETWORK),
           "ATTACK_VECTOR_ADJACENT" => Ok(Cvssv3AttackVectorEnum::ATTACKVECTORADJACENT),
           "ATTACK_VECTOR_LOCAL" => Ok(Cvssv3AttackVectorEnum::ATTACKVECTORLOCAL),
           "ATTACK_VECTOR_PHYSICAL" => Ok(Cvssv3AttackVectorEnum::ATTACKVECTORPHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3AttackVectorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3AvailabilityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This metric measures the impact to the availability of the impacted component resulting from a successfully exploited vulnerability.
pub enum Cvssv3AvailabilityImpactEnum {
    

    /// Invalid value.
    ///
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    

    /// High impact.
    ///
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    

    /// Low impact.
    ///
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    

    /// No impact.
    ///
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
}

impl AsRef<str> for Cvssv3AvailabilityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3AvailabilityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            Cvssv3AvailabilityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            Cvssv3AvailabilityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            Cvssv3AvailabilityImpactEnum::IMPACTNONE => "IMPACT_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3AvailabilityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(Cvssv3AvailabilityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(Cvssv3AvailabilityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(Cvssv3AvailabilityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(Cvssv3AvailabilityImpactEnum::IMPACTNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3AvailabilityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3ConfidentialityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This metric measures the impact to the confidentiality of the information resources managed by a software component due to a successfully exploited vulnerability.
pub enum Cvssv3ConfidentialityImpactEnum {
    

    /// Invalid value.
    ///
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    

    /// High impact.
    ///
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    

    /// Low impact.
    ///
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    

    /// No impact.
    ///
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
}

impl AsRef<str> for Cvssv3ConfidentialityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3ConfidentialityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            Cvssv3ConfidentialityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            Cvssv3ConfidentialityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            Cvssv3ConfidentialityImpactEnum::IMPACTNONE => "IMPACT_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3ConfidentialityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(Cvssv3ConfidentialityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(Cvssv3ConfidentialityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(Cvssv3ConfidentialityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(Cvssv3ConfidentialityImpactEnum::IMPACTNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3ConfidentialityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3IntegrityImpactEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This metric measures the impact to integrity of a successfully exploited vulnerability.
pub enum Cvssv3IntegrityImpactEnum {
    

    /// Invalid value.
    ///
    /// "IMPACT_UNSPECIFIED"
    #[serde(rename="IMPACT_UNSPECIFIED")]
    IMPACTUNSPECIFIED,
    

    /// High impact.
    ///
    /// "IMPACT_HIGH"
    #[serde(rename="IMPACT_HIGH")]
    IMPACTHIGH,
    

    /// Low impact.
    ///
    /// "IMPACT_LOW"
    #[serde(rename="IMPACT_LOW")]
    IMPACTLOW,
    

    /// No impact.
    ///
    /// "IMPACT_NONE"
    #[serde(rename="IMPACT_NONE")]
    IMPACTNONE,
}

impl AsRef<str> for Cvssv3IntegrityImpactEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3IntegrityImpactEnum::IMPACTUNSPECIFIED => "IMPACT_UNSPECIFIED",
            Cvssv3IntegrityImpactEnum::IMPACTHIGH => "IMPACT_HIGH",
            Cvssv3IntegrityImpactEnum::IMPACTLOW => "IMPACT_LOW",
            Cvssv3IntegrityImpactEnum::IMPACTNONE => "IMPACT_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3IntegrityImpactEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPACT_UNSPECIFIED" => Ok(Cvssv3IntegrityImpactEnum::IMPACTUNSPECIFIED),
           "IMPACT_HIGH" => Ok(Cvssv3IntegrityImpactEnum::IMPACTHIGH),
           "IMPACT_LOW" => Ok(Cvssv3IntegrityImpactEnum::IMPACTLOW),
           "IMPACT_NONE" => Ok(Cvssv3IntegrityImpactEnum::IMPACTNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3IntegrityImpactEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3PrivilegesRequiredEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This metric describes the level of privileges an attacker must possess before successfully exploiting the vulnerability.
pub enum Cvssv3PrivilegesRequiredEnum {
    

    /// Invalid value.
    ///
    /// "PRIVILEGES_REQUIRED_UNSPECIFIED"
    #[serde(rename="PRIVILEGES_REQUIRED_UNSPECIFIED")]
    PRIVILEGESREQUIREDUNSPECIFIED,
    

    /// The attacker is unauthorized prior to attack, and therefore does not require any access to settings or files of the vulnerable system to carry out an attack.
    ///
    /// "PRIVILEGES_REQUIRED_NONE"
    #[serde(rename="PRIVILEGES_REQUIRED_NONE")]
    PRIVILEGESREQUIREDNONE,
    

    /// The attacker requires privileges that provide basic user capabilities that could normally affect only settings and files owned by a user. Alternatively, an attacker with Low privileges has the ability to access only non-sensitive resources.
    ///
    /// "PRIVILEGES_REQUIRED_LOW"
    #[serde(rename="PRIVILEGES_REQUIRED_LOW")]
    PRIVILEGESREQUIREDLOW,
    

    /// The attacker requires privileges that provide significant (e.g., administrative) control over the vulnerable component allowing access to component-wide settings and files.
    ///
    /// "PRIVILEGES_REQUIRED_HIGH"
    #[serde(rename="PRIVILEGES_REQUIRED_HIGH")]
    PRIVILEGESREQUIREDHIGH,
}

impl AsRef<str> for Cvssv3PrivilegesRequiredEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDUNSPECIFIED => "PRIVILEGES_REQUIRED_UNSPECIFIED",
            Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDNONE => "PRIVILEGES_REQUIRED_NONE",
            Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDLOW => "PRIVILEGES_REQUIRED_LOW",
            Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDHIGH => "PRIVILEGES_REQUIRED_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3PrivilegesRequiredEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIVILEGES_REQUIRED_UNSPECIFIED" => Ok(Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDUNSPECIFIED),
           "PRIVILEGES_REQUIRED_NONE" => Ok(Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDNONE),
           "PRIVILEGES_REQUIRED_LOW" => Ok(Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDLOW),
           "PRIVILEGES_REQUIRED_HIGH" => Ok(Cvssv3PrivilegesRequiredEnum::PRIVILEGESREQUIREDHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3PrivilegesRequiredEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3ScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Scope metric captures whether a vulnerability in one vulnerable component impacts resources in components beyond its security scope.
pub enum Cvssv3ScopeEnum {
    

    /// Invalid value.
    ///
    /// "SCOPE_UNSPECIFIED"
    #[serde(rename="SCOPE_UNSPECIFIED")]
    SCOPEUNSPECIFIED,
    

    /// An exploited vulnerability can only affect resources managed by the same security authority.
    ///
    /// "SCOPE_UNCHANGED"
    #[serde(rename="SCOPE_UNCHANGED")]
    SCOPEUNCHANGED,
    

    /// An exploited vulnerability can affect resources beyond the security scope managed by the security authority of the vulnerable component.
    ///
    /// "SCOPE_CHANGED"
    #[serde(rename="SCOPE_CHANGED")]
    SCOPECHANGED,
}

impl AsRef<str> for Cvssv3ScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3ScopeEnum::SCOPEUNSPECIFIED => "SCOPE_UNSPECIFIED",
            Cvssv3ScopeEnum::SCOPEUNCHANGED => "SCOPE_UNCHANGED",
            Cvssv3ScopeEnum::SCOPECHANGED => "SCOPE_CHANGED",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3ScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCOPE_UNSPECIFIED" => Ok(Cvssv3ScopeEnum::SCOPEUNSPECIFIED),
           "SCOPE_UNCHANGED" => Ok(Cvssv3ScopeEnum::SCOPEUNCHANGED),
           "SCOPE_CHANGED" => Ok(Cvssv3ScopeEnum::SCOPECHANGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3ScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Cvssv3UserInteractionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This metric captures the requirement for a human user, other than the attacker, to participate in the successful compromise of the vulnerable component.
pub enum Cvssv3UserInteractionEnum {
    

    /// Invalid value.
    ///
    /// "USER_INTERACTION_UNSPECIFIED"
    #[serde(rename="USER_INTERACTION_UNSPECIFIED")]
    USERINTERACTIONUNSPECIFIED,
    

    /// The vulnerable system can be exploited without interaction from any user.
    ///
    /// "USER_INTERACTION_NONE"
    #[serde(rename="USER_INTERACTION_NONE")]
    USERINTERACTIONNONE,
    

    /// Successful exploitation of this vulnerability requires a user to take some action before the vulnerability can be exploited.
    ///
    /// "USER_INTERACTION_REQUIRED"
    #[serde(rename="USER_INTERACTION_REQUIRED")]
    USERINTERACTIONREQUIRED,
}

impl AsRef<str> for Cvssv3UserInteractionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Cvssv3UserInteractionEnum::USERINTERACTIONUNSPECIFIED => "USER_INTERACTION_UNSPECIFIED",
            Cvssv3UserInteractionEnum::USERINTERACTIONNONE => "USER_INTERACTION_NONE",
            Cvssv3UserInteractionEnum::USERINTERACTIONREQUIRED => "USER_INTERACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for Cvssv3UserInteractionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_INTERACTION_UNSPECIFIED" => Ok(Cvssv3UserInteractionEnum::USERINTERACTIONUNSPECIFIED),
           "USER_INTERACTION_NONE" => Ok(Cvssv3UserInteractionEnum::USERINTERACTIONNONE),
           "USER_INTERACTION_REQUIRED" => Ok(Cvssv3UserInteractionEnum::USERINTERACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Cvssv3UserInteractionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EffectiveEventThreatDetectionCustomModuleEnablementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The effective state of enablement for the module at the given level of the hierarchy.
pub enum EffectiveEventThreatDetectionCustomModuleEnablementStateEnum {
    

    /// Unspecified enablement state.
    ///
    /// "ENABLEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ENABLEMENT_STATE_UNSPECIFIED")]
    ENABLEMENTSTATEUNSPECIFIED,
    

    /// The module is enabled at the given level.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The module is disabled at the given level.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for EffectiveEventThreatDetectionCustomModuleEnablementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EffectiveEventThreatDetectionCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED => "ENABLEMENT_STATE_UNSPECIFIED",
            EffectiveEventThreatDetectionCustomModuleEnablementStateEnum::ENABLED => "ENABLED",
            EffectiveEventThreatDetectionCustomModuleEnablementStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for EffectiveEventThreatDetectionCustomModuleEnablementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENABLEMENT_STATE_UNSPECIFIED" => Ok(EffectiveEventThreatDetectionCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED),
           "ENABLED" => Ok(EffectiveEventThreatDetectionCustomModuleEnablementStateEnum::ENABLED),
           "DISABLED" => Ok(EffectiveEventThreatDetectionCustomModuleEnablementStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EffectiveEventThreatDetectionCustomModuleEnablementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventThreatDetectionCustomModuleEnablementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of enablement for the module at the given level of the hierarchy.
pub enum EventThreatDetectionCustomModuleEnablementStateEnum {
    

    /// Unspecified enablement state.
    ///
    /// "ENABLEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ENABLEMENT_STATE_UNSPECIFIED")]
    ENABLEMENTSTATEUNSPECIFIED,
    

    /// The module is enabled at the given level.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The module is disabled at the given level.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// When the enablement state is inherited.
    ///
    /// "INHERITED"
    #[serde(rename="INHERITED")]
    INHERITED,
}

impl AsRef<str> for EventThreatDetectionCustomModuleEnablementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventThreatDetectionCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED => "ENABLEMENT_STATE_UNSPECIFIED",
            EventThreatDetectionCustomModuleEnablementStateEnum::ENABLED => "ENABLED",
            EventThreatDetectionCustomModuleEnablementStateEnum::DISABLED => "DISABLED",
            EventThreatDetectionCustomModuleEnablementStateEnum::INHERITED => "INHERITED",
        }
    }
}

impl std::convert::TryFrom< &str> for EventThreatDetectionCustomModuleEnablementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENABLEMENT_STATE_UNSPECIFIED" => Ok(EventThreatDetectionCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED),
           "ENABLED" => Ok(EventThreatDetectionCustomModuleEnablementStateEnum::ENABLED),
           "DISABLED" => Ok(EventThreatDetectionCustomModuleEnablementStateEnum::DISABLED),
           "INHERITED" => Ok(EventThreatDetectionCustomModuleEnablementStateEnum::INHERITED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventThreatDetectionCustomModuleEnablementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FindingFindingClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The class of the finding.
pub enum FindingFindingClassEnum {
    

    /// Unspecified finding class.
    ///
    /// "FINDING_CLASS_UNSPECIFIED"
    #[serde(rename="FINDING_CLASS_UNSPECIFIED")]
    FINDINGCLASSUNSPECIFIED,
    

    /// Describes unwanted or malicious activity.
    ///
    /// "THREAT"
    #[serde(rename="THREAT")]
    THREAT,
    

    /// Describes a potential weakness in software that increases risk to Confidentiality & Integrity & Availability.
    ///
    /// "VULNERABILITY"
    #[serde(rename="VULNERABILITY")]
    VULNERABILITY,
    

    /// Describes a potential weakness in cloud resource/asset configuration that increases risk.
    ///
    /// "MISCONFIGURATION"
    #[serde(rename="MISCONFIGURATION")]
    MISCONFIGURATION,
    

    /// Describes a security observation that is for informational purposes.
    ///
    /// "OBSERVATION"
    #[serde(rename="OBSERVATION")]
    OBSERVATION,
    

    /// Describes an error that prevents some SCC functionality.
    ///
    /// "SCC_ERROR"
    #[serde(rename="SCC_ERROR")]
    SCCERROR,
    

    /// Describes a potential security risk due to a change in the security posture.
    ///
    /// "POSTURE_VIOLATION"
    #[serde(rename="POSTURE_VIOLATION")]
    POSTUREVIOLATION,
}

impl AsRef<str> for FindingFindingClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FindingFindingClassEnum::FINDINGCLASSUNSPECIFIED => "FINDING_CLASS_UNSPECIFIED",
            FindingFindingClassEnum::THREAT => "THREAT",
            FindingFindingClassEnum::VULNERABILITY => "VULNERABILITY",
            FindingFindingClassEnum::MISCONFIGURATION => "MISCONFIGURATION",
            FindingFindingClassEnum::OBSERVATION => "OBSERVATION",
            FindingFindingClassEnum::SCCERROR => "SCC_ERROR",
            FindingFindingClassEnum::POSTUREVIOLATION => "POSTURE_VIOLATION",
        }
    }
}

impl std::convert::TryFrom< &str> for FindingFindingClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FINDING_CLASS_UNSPECIFIED" => Ok(FindingFindingClassEnum::FINDINGCLASSUNSPECIFIED),
           "THREAT" => Ok(FindingFindingClassEnum::THREAT),
           "VULNERABILITY" => Ok(FindingFindingClassEnum::VULNERABILITY),
           "MISCONFIGURATION" => Ok(FindingFindingClassEnum::MISCONFIGURATION),
           "OBSERVATION" => Ok(FindingFindingClassEnum::OBSERVATION),
           "SCC_ERROR" => Ok(FindingFindingClassEnum::SCCERROR),
           "POSTURE_VIOLATION" => Ok(FindingFindingClassEnum::POSTUREVIOLATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FindingFindingClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FindingMuteEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the mute state of a finding (either muted, unmuted or undefined). Unlike other attributes of a finding, a finding provider shouldn't set the value of mute.
pub enum FindingMuteEnum {
    

    /// Unspecified.
    ///
    /// "MUTE_UNSPECIFIED"
    #[serde(rename="MUTE_UNSPECIFIED")]
    MUTEUNSPECIFIED,
    

    /// Finding has been muted.
    ///
    /// "MUTED"
    #[serde(rename="MUTED")]
    MUTED,
    

    /// Finding has been unmuted.
    ///
    /// "UNMUTED"
    #[serde(rename="UNMUTED")]
    UNMUTED,
    

    /// Finding has never been muted/unmuted.
    ///
    /// "UNDEFINED"
    #[serde(rename="UNDEFINED")]
    UNDEFINED,
}

impl AsRef<str> for FindingMuteEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FindingMuteEnum::MUTEUNSPECIFIED => "MUTE_UNSPECIFIED",
            FindingMuteEnum::MUTED => "MUTED",
            FindingMuteEnum::UNMUTED => "UNMUTED",
            FindingMuteEnum::UNDEFINED => "UNDEFINED",
        }
    }
}

impl std::convert::TryFrom< &str> for FindingMuteEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MUTE_UNSPECIFIED" => Ok(FindingMuteEnum::MUTEUNSPECIFIED),
           "MUTED" => Ok(FindingMuteEnum::MUTED),
           "UNMUTED" => Ok(FindingMuteEnum::UNMUTED),
           "UNDEFINED" => Ok(FindingMuteEnum::UNDEFINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FindingMuteEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FindingSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the finding. This field is managed by the source that writes the finding.
pub enum FindingSeverityEnum {
    

    /// This value is used for findings when a source doesn't write a severity value.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Vulnerability: A critical vulnerability is easily discoverable by an external actor, exploitable, and results in the direct ability to execute arbitrary code, exfiltrate data, and otherwise gain additional access and privileges to cloud resources and workloads. Examples include publicly accessible unprotected user data and public SSH access with weak or no passwords. Threat: Indicates a threat that is able to access, modify, or delete data or execute unauthorized code within existing resources.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
    

    /// Vulnerability: A high risk vulnerability can be easily discovered and exploited in combination with other vulnerabilities in order to gain direct access and the ability to execute arbitrary code, exfiltrate data, and otherwise gain additional access and privileges to cloud resources and workloads. An example is a database with weak or no passwords that is only accessible internally. This database could easily be compromised by an actor that had access to the internal network. Threat: Indicates a threat that is able to create new computational resources in an environment but not able to access data or execute code in existing resources.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Vulnerability: A medium risk vulnerability could be used by an actor to gain access to resources or privileges that enable them to eventually (through multiple steps or a complex exploit) gain access and the ability to execute arbitrary code or exfiltrate data. An example is a service account with access to more projects than it should have. If an actor gains access to the service account, they could potentially use that access to manipulate a project the service account was not intended to. Threat: Indicates a threat that is able to cause operational impact but may not access data or execute unauthorized code.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Vulnerability: A low risk vulnerability hampers a security organization's ability to detect vulnerabilities or active threats in their deployment, or prevents the root cause investigation of security issues. An example is monitoring and logs being disabled for resource configurations and access. Threat: Indicates a threat that has obtained minimal access to an environment but is not able to access data, execute code, or create resources.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
}

impl AsRef<str> for FindingSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FindingSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            FindingSeverityEnum::CRITICAL => "CRITICAL",
            FindingSeverityEnum::HIGH => "HIGH",
            FindingSeverityEnum::MEDIUM => "MEDIUM",
            FindingSeverityEnum::LOW => "LOW",
        }
    }
}

impl std::convert::TryFrom< &str> for FindingSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(FindingSeverityEnum::SEVERITYUNSPECIFIED),
           "CRITICAL" => Ok(FindingSeverityEnum::CRITICAL),
           "HIGH" => Ok(FindingSeverityEnum::HIGH),
           "MEDIUM" => Ok(FindingSeverityEnum::MEDIUM),
           "LOW" => Ok(FindingSeverityEnum::LOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FindingSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FindingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the finding.
pub enum FindingStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The finding requires attention and has not been addressed yet.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The finding has been fixed, triaged as a non-issue or otherwise addressed and is no longer active.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for FindingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FindingStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FindingStateEnum::ACTIVE => "ACTIVE",
            FindingStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for FindingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FindingStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(FindingStateEnum::ACTIVE),
           "INACTIVE" => Ok(FindingStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FindingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1CustomConfigSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity to assign to findings generated by the module.
pub enum GoogleCloudSecuritycenterV1CustomConfigSeverityEnum {
    

    /// Unspecified severity.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
    

    /// High severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1CustomConfigSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::CRITICAL => "CRITICAL",
            GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::HIGH => "HIGH",
            GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::MEDIUM => "MEDIUM",
            GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::LOW => "LOW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1CustomConfigSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::SEVERITYUNSPECIFIED),
           "CRITICAL" => Ok(GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::CRITICAL),
           "HIGH" => Ok(GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::HIGH),
           "MEDIUM" => Ok(GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::MEDIUM),
           "LOW" => Ok(GoogleCloudSecuritycenterV1CustomConfigSeverityEnum::LOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1CustomConfigSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The effective state of enablement for the module at the given level of the hierarchy.
pub enum GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    

    /// Unspecified enablement state.
    ///
    /// "ENABLEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ENABLEMENT_STATE_UNSPECIFIED")]
    ENABLEMENTSTATEUNSPECIFIED,
    

    /// The module is enabled at the given level.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The module is disabled at the given level.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED => "ENABLEMENT_STATE_UNSPECIFIED",
            GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLED => "ENABLED",
            GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENABLEMENT_STATE_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED),
           "ENABLED" => Ok(GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLED),
           "DISABLED" => Ok(GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1EffectiveSecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Cloud provider this configuration applies to
pub enum GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum {
    

    /// The cloud provider is unspecified.
    ///
    /// "CLOUD_PROVIDER_UNSPECIFIED"
    #[serde(rename="CLOUD_PROVIDER_UNSPECIFIED")]
    CLOUDPROVIDERUNSPECIFIED,
    

    /// The cloud provider is Google Cloud Platform.
    ///
    /// "GOOGLE_CLOUD_PLATFORM"
    #[serde(rename="GOOGLE_CLOUD_PLATFORM")]
    GOOGLECLOUDPLATFORM,
    

    /// The cloud provider is Amazon Web Services.
    ///
    /// "AMAZON_WEB_SERVICES"
    #[serde(rename="AMAZON_WEB_SERVICES")]
    AMAZONWEBSERVICES,
    

    /// The cloud provider is Microsoft Azure.
    ///
    /// "MICROSOFT_AZURE"
    #[serde(rename="MICROSOFT_AZURE")]
    MICROSOFTAZURE,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::CLOUDPROVIDERUNSPECIFIED => "CLOUD_PROVIDER_UNSPECIFIED",
            GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::GOOGLECLOUDPLATFORM => "GOOGLE_CLOUD_PLATFORM",
            GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::AMAZONWEBSERVICES => "AMAZON_WEB_SERVICES",
            GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::MICROSOFTAZURE => "MICROSOFT_AZURE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLOUD_PROVIDER_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::CLOUDPROVIDERUNSPECIFIED),
           "GOOGLE_CLOUD_PLATFORM" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::GOOGLECLOUDPLATFORM),
           "AMAZON_WEB_SERVICES" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::AMAZONWEBSERVICES),
           "MICROSOFT_AZURE" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum::MICROSOFTAZURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1ResourceValueConfigCloudProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Resource value level this expression represents
pub enum GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum {
    

    /// Unspecific value
    ///
    /// "RESOURCE_VALUE_UNSPECIFIED"
    #[serde(rename="RESOURCE_VALUE_UNSPECIFIED")]
    RESOURCEVALUEUNSPECIFIED,
    

    /// High resource value
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Medium resource value
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Low resource value
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// No resource value, e.g. ignore these resources
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::RESOURCEVALUEUNSPECIFIED => "RESOURCE_VALUE_UNSPECIFIED",
            GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::HIGH => "HIGH",
            GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::MEDIUM => "MEDIUM",
            GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::LOW => "LOW",
            GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_VALUE_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::RESOURCEVALUEUNSPECIFIED),
           "HIGH" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::HIGH),
           "MEDIUM" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::MEDIUM),
           "LOW" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::LOW),
           "NONE" => Ok(GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1ResourceValueConfigResourceValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The enablement state of the custom module.
pub enum GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    

    /// Unspecified enablement state.
    ///
    /// "ENABLEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ENABLEMENT_STATE_UNSPECIFIED")]
    ENABLEMENTSTATEUNSPECIFIED,
    

    /// The module is enabled at the given CRM resource.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The module is disabled at the given CRM resource.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// State is inherited from an ancestor module. The module will either be effectively ENABLED or DISABLED based on its closest non-inherited ancestor module in the CRM hierarchy.
    ///
    /// "INHERITED"
    #[serde(rename="INHERITED")]
    INHERITED,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED => "ENABLEMENT_STATE_UNSPECIFIED",
            GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLED => "ENABLED",
            GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::DISABLED => "DISABLED",
            GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::INHERITED => "INHERITED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENABLEMENT_STATE_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLEMENTSTATEUNSPECIFIED),
           "ENABLED" => Ok(GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::ENABLED),
           "DISABLED" => Ok(GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::DISABLED),
           "INHERITED" => Ok(GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum::INHERITED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModuleEnablementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Resource value mapping for high-sensitivity Sensitive Data Protection findings
pub enum GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum {
    

    /// Unspecific value
    ///
    /// "RESOURCE_VALUE_UNSPECIFIED"
    #[serde(rename="RESOURCE_VALUE_UNSPECIFIED")]
    RESOURCEVALUEUNSPECIFIED,
    

    /// High resource value
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Medium resource value
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Low resource value
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// No resource value, e.g. ignore these resources
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::RESOURCEVALUEUNSPECIFIED => "RESOURCE_VALUE_UNSPECIFIED",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::HIGH => "HIGH",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::MEDIUM => "MEDIUM",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::LOW => "LOW",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_VALUE_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::RESOURCEVALUEUNSPECIFIED),
           "HIGH" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::HIGH),
           "MEDIUM" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::MEDIUM),
           "LOW" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::LOW),
           "NONE" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingHighSensitivityMappingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Resource value mapping for medium-sensitivity Sensitive Data Protection findings
pub enum GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum {
    

    /// Unspecific value
    ///
    /// "RESOURCE_VALUE_UNSPECIFIED"
    #[serde(rename="RESOURCE_VALUE_UNSPECIFIED")]
    RESOURCEVALUEUNSPECIFIED,
    

    /// High resource value
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Medium resource value
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Low resource value
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// No resource value, e.g. ignore these resources
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::RESOURCEVALUEUNSPECIFIED => "RESOURCE_VALUE_UNSPECIFIED",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::HIGH => "HIGH",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::MEDIUM => "MEDIUM",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::LOW => "LOW",
            GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_VALUE_UNSPECIFIED" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::RESOURCEVALUEUNSPECIFIED),
           "HIGH" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::HIGH),
           "MEDIUM" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::MEDIUM),
           "LOW" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::LOW),
           "NONE" => Ok(GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudSecuritycenterV1SensitiveDataProtectionMappingMediumSensitivityMappingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IamBindingActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action that was performed on a Binding.
pub enum IamBindingActionEnum {
    

    /// Unspecified.
    ///
    /// "ACTION_UNSPECIFIED"
    #[serde(rename="ACTION_UNSPECIFIED")]
    ACTIONUNSPECIFIED,
    

    /// Addition of a Binding.
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// Removal of a Binding.
    ///
    /// "REMOVE"
    #[serde(rename="REMOVE")]
    REMOVE,
}

impl AsRef<str> for IamBindingActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IamBindingActionEnum::ACTIONUNSPECIFIED => "ACTION_UNSPECIFIED",
            IamBindingActionEnum::ADD => "ADD",
            IamBindingActionEnum::REMOVE => "REMOVE",
        }
    }
}

impl std::convert::TryFrom< &str> for IamBindingActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNSPECIFIED" => Ok(IamBindingActionEnum::ACTIONUNSPECIFIED),
           "ADD" => Ok(IamBindingActionEnum::ADD),
           "REMOVE" => Ok(IamBindingActionEnum::REMOVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IamBindingActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListAssetsResultStateChangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State change of the asset between the points in time.
pub enum ListAssetsResultStateChangeEnum {
    

    /// State change is unused, this is the canonical default for this enum.
    ///
    /// "UNUSED"
    #[serde(rename="UNUSED")]
    UNUSED,
    

    /// Asset was added between the points in time.
    ///
    /// "ADDED"
    #[serde(rename="ADDED")]
    ADDED,
    

    /// Asset was removed between the points in time.
    ///
    /// "REMOVED"
    #[serde(rename="REMOVED")]
    REMOVED,
    

    /// Asset was present at both point(s) in time.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for ListAssetsResultStateChangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListAssetsResultStateChangeEnum::UNUSED => "UNUSED",
            ListAssetsResultStateChangeEnum::ADDED => "ADDED",
            ListAssetsResultStateChangeEnum::REMOVED => "REMOVED",
            ListAssetsResultStateChangeEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ListAssetsResultStateChangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNUSED" => Ok(ListAssetsResultStateChangeEnum::UNUSED),
           "ADDED" => Ok(ListAssetsResultStateChangeEnum::ADDED),
           "REMOVED" => Ok(ListAssetsResultStateChangeEnum::REMOVED),
           "ACTIVE" => Ok(ListAssetsResultStateChangeEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListAssetsResultStateChangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListFindingsResultStateChangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State change of the finding between the points in time.
pub enum ListFindingsResultStateChangeEnum {
    

    /// State change is unused, this is the canonical default for this enum.
    ///
    /// "UNUSED"
    #[serde(rename="UNUSED")]
    UNUSED,
    

    /// The finding has changed state in some way between the points in time and existed at both points.
    ///
    /// "CHANGED"
    #[serde(rename="CHANGED")]
    CHANGED,
    

    /// The finding has not changed state between the points in time and existed at both points.
    ///
    /// "UNCHANGED"
    #[serde(rename="UNCHANGED")]
    UNCHANGED,
    

    /// The finding was created between the points in time.
    ///
    /// "ADDED"
    #[serde(rename="ADDED")]
    ADDED,
    

    /// The finding at timestamp does not match the filter specified, but it did at timestamp - compare_duration.
    ///
    /// "REMOVED"
    #[serde(rename="REMOVED")]
    REMOVED,
}

impl AsRef<str> for ListFindingsResultStateChangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListFindingsResultStateChangeEnum::UNUSED => "UNUSED",
            ListFindingsResultStateChangeEnum::CHANGED => "CHANGED",
            ListFindingsResultStateChangeEnum::UNCHANGED => "UNCHANGED",
            ListFindingsResultStateChangeEnum::ADDED => "ADDED",
            ListFindingsResultStateChangeEnum::REMOVED => "REMOVED",
        }
    }
}

impl std::convert::TryFrom< &str> for ListFindingsResultStateChangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNUSED" => Ok(ListFindingsResultStateChangeEnum::UNUSED),
           "CHANGED" => Ok(ListFindingsResultStateChangeEnum::CHANGED),
           "UNCHANGED" => Ok(ListFindingsResultStateChangeEnum::UNCHANGED),
           "ADDED" => Ok(ListFindingsResultStateChangeEnum::ADDED),
           "REMOVED" => Ok(ListFindingsResultStateChangeEnum::REMOVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListFindingsResultStateChangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MitreAttackAdditionalTacticsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Additional MITRE ATT&CK tactics related to this finding, if any.
pub enum MitreAttackAdditionalTacticsEnum {
    

    /// Unspecified value.
    ///
    /// "TACTIC_UNSPECIFIED"
    #[serde(rename="TACTIC_UNSPECIFIED")]
    TACTICUNSPECIFIED,
    

    /// TA0043
    ///
    /// "RECONNAISSANCE"
    #[serde(rename="RECONNAISSANCE")]
    RECONNAISSANCE,
    

    /// TA0042
    ///
    /// "RESOURCE_DEVELOPMENT"
    #[serde(rename="RESOURCE_DEVELOPMENT")]
    RESOURCEDEVELOPMENT,
    

    /// TA0001
    ///
    /// "INITIAL_ACCESS"
    #[serde(rename="INITIAL_ACCESS")]
    INITIALACCESS,
    

    /// TA0002
    ///
    /// "EXECUTION"
    #[serde(rename="EXECUTION")]
    EXECUTION,
    

    /// TA0003
    ///
    /// "PERSISTENCE"
    #[serde(rename="PERSISTENCE")]
    PERSISTENCE,
    

    /// TA0004
    ///
    /// "PRIVILEGE_ESCALATION"
    #[serde(rename="PRIVILEGE_ESCALATION")]
    PRIVILEGEESCALATION,
    

    /// TA0005
    ///
    /// "DEFENSE_EVASION"
    #[serde(rename="DEFENSE_EVASION")]
    DEFENSEEVASION,
    

    /// TA0006
    ///
    /// "CREDENTIAL_ACCESS"
    #[serde(rename="CREDENTIAL_ACCESS")]
    CREDENTIALACCESS,
    

    /// TA0007
    ///
    /// "DISCOVERY"
    #[serde(rename="DISCOVERY")]
    DISCOVERY,
    

    /// TA0008
    ///
    /// "LATERAL_MOVEMENT"
    #[serde(rename="LATERAL_MOVEMENT")]
    LATERALMOVEMENT,
    

    /// TA0009
    ///
    /// "COLLECTION"
    #[serde(rename="COLLECTION")]
    COLLECTION,
    

    /// TA0011
    ///
    /// "COMMAND_AND_CONTROL"
    #[serde(rename="COMMAND_AND_CONTROL")]
    COMMANDANDCONTROL,
    

    /// TA0010
    ///
    /// "EXFILTRATION"
    #[serde(rename="EXFILTRATION")]
    EXFILTRATION,
    

    /// TA0040
    ///
    /// "IMPACT"
    #[serde(rename="IMPACT")]
    IMPACT,
}

impl AsRef<str> for MitreAttackAdditionalTacticsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MitreAttackAdditionalTacticsEnum::TACTICUNSPECIFIED => "TACTIC_UNSPECIFIED",
            MitreAttackAdditionalTacticsEnum::RECONNAISSANCE => "RECONNAISSANCE",
            MitreAttackAdditionalTacticsEnum::RESOURCEDEVELOPMENT => "RESOURCE_DEVELOPMENT",
            MitreAttackAdditionalTacticsEnum::INITIALACCESS => "INITIAL_ACCESS",
            MitreAttackAdditionalTacticsEnum::EXECUTION => "EXECUTION",
            MitreAttackAdditionalTacticsEnum::PERSISTENCE => "PERSISTENCE",
            MitreAttackAdditionalTacticsEnum::PRIVILEGEESCALATION => "PRIVILEGE_ESCALATION",
            MitreAttackAdditionalTacticsEnum::DEFENSEEVASION => "DEFENSE_EVASION",
            MitreAttackAdditionalTacticsEnum::CREDENTIALACCESS => "CREDENTIAL_ACCESS",
            MitreAttackAdditionalTacticsEnum::DISCOVERY => "DISCOVERY",
            MitreAttackAdditionalTacticsEnum::LATERALMOVEMENT => "LATERAL_MOVEMENT",
            MitreAttackAdditionalTacticsEnum::COLLECTION => "COLLECTION",
            MitreAttackAdditionalTacticsEnum::COMMANDANDCONTROL => "COMMAND_AND_CONTROL",
            MitreAttackAdditionalTacticsEnum::EXFILTRATION => "EXFILTRATION",
            MitreAttackAdditionalTacticsEnum::IMPACT => "IMPACT",
        }
    }
}

impl std::convert::TryFrom< &str> for MitreAttackAdditionalTacticsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TACTIC_UNSPECIFIED" => Ok(MitreAttackAdditionalTacticsEnum::TACTICUNSPECIFIED),
           "RECONNAISSANCE" => Ok(MitreAttackAdditionalTacticsEnum::RECONNAISSANCE),
           "RESOURCE_DEVELOPMENT" => Ok(MitreAttackAdditionalTacticsEnum::RESOURCEDEVELOPMENT),
           "INITIAL_ACCESS" => Ok(MitreAttackAdditionalTacticsEnum::INITIALACCESS),
           "EXECUTION" => Ok(MitreAttackAdditionalTacticsEnum::EXECUTION),
           "PERSISTENCE" => Ok(MitreAttackAdditionalTacticsEnum::PERSISTENCE),
           "PRIVILEGE_ESCALATION" => Ok(MitreAttackAdditionalTacticsEnum::PRIVILEGEESCALATION),
           "DEFENSE_EVASION" => Ok(MitreAttackAdditionalTacticsEnum::DEFENSEEVASION),
           "CREDENTIAL_ACCESS" => Ok(MitreAttackAdditionalTacticsEnum::CREDENTIALACCESS),
           "DISCOVERY" => Ok(MitreAttackAdditionalTacticsEnum::DISCOVERY),
           "LATERAL_MOVEMENT" => Ok(MitreAttackAdditionalTacticsEnum::LATERALMOVEMENT),
           "COLLECTION" => Ok(MitreAttackAdditionalTacticsEnum::COLLECTION),
           "COMMAND_AND_CONTROL" => Ok(MitreAttackAdditionalTacticsEnum::COMMANDANDCONTROL),
           "EXFILTRATION" => Ok(MitreAttackAdditionalTacticsEnum::EXFILTRATION),
           "IMPACT" => Ok(MitreAttackAdditionalTacticsEnum::IMPACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MitreAttackAdditionalTacticsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MitreAttackAdditionalTechniquesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Additional MITRE ATT&CK techniques related to this finding, if any, along with any of their respective parent techniques.
pub enum MitreAttackAdditionalTechniquesEnum {
    

    /// Unspecified value.
    ///
    /// "TECHNIQUE_UNSPECIFIED"
    #[serde(rename="TECHNIQUE_UNSPECIFIED")]
    TECHNIQUEUNSPECIFIED,
    

    /// T1036
    ///
    /// "MASQUERADING"
    #[serde(rename="MASQUERADING")]
    MASQUERADING,
    

    /// T1036.005
    ///
    /// "MATCH_LEGITIMATE_NAME_OR_LOCATION"
    #[serde(rename="MATCH_LEGITIMATE_NAME_OR_LOCATION")]
    MATCHLEGITIMATENAMEORLOCATION,
    

    /// T1037
    ///
    /// "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS"
    #[serde(rename="BOOT_OR_LOGON_INITIALIZATION_SCRIPTS")]
    BOOTORLOGONINITIALIZATIONSCRIPTS,
    

    /// T1037.005
    ///
    /// "STARTUP_ITEMS"
    #[serde(rename="STARTUP_ITEMS")]
    STARTUPITEMS,
    

    /// T1046
    ///
    /// "NETWORK_SERVICE_DISCOVERY"
    #[serde(rename="NETWORK_SERVICE_DISCOVERY")]
    NETWORKSERVICEDISCOVERY,
    

    /// T1057
    ///
    /// "PROCESS_DISCOVERY"
    #[serde(rename="PROCESS_DISCOVERY")]
    PROCESSDISCOVERY,
    

    /// T1059
    ///
    /// "COMMAND_AND_SCRIPTING_INTERPRETER"
    #[serde(rename="COMMAND_AND_SCRIPTING_INTERPRETER")]
    COMMANDANDSCRIPTINGINTERPRETER,
    

    /// T1059.004
    ///
    /// "UNIX_SHELL"
    #[serde(rename="UNIX_SHELL")]
    UNIXSHELL,
    

    /// T1059.006
    ///
    /// "PYTHON"
    #[serde(rename="PYTHON")]
    PYTHON,
    

    /// T1069
    ///
    /// "PERMISSION_GROUPS_DISCOVERY"
    #[serde(rename="PERMISSION_GROUPS_DISCOVERY")]
    PERMISSIONGROUPSDISCOVERY,
    

    /// T1069.003
    ///
    /// "CLOUD_GROUPS"
    #[serde(rename="CLOUD_GROUPS")]
    CLOUDGROUPS,
    

    /// T1071
    ///
    /// "APPLICATION_LAYER_PROTOCOL"
    #[serde(rename="APPLICATION_LAYER_PROTOCOL")]
    APPLICATIONLAYERPROTOCOL,
    

    /// T1071.004
    ///
    /// "DNS"
    #[serde(rename="DNS")]
    DNS,
    

    /// T1072
    ///
    /// "SOFTWARE_DEPLOYMENT_TOOLS"
    #[serde(rename="SOFTWARE_DEPLOYMENT_TOOLS")]
    SOFTWAREDEPLOYMENTTOOLS,
    

    /// T1078
    ///
    /// "VALID_ACCOUNTS"
    #[serde(rename="VALID_ACCOUNTS")]
    VALIDACCOUNTS,
    

    /// T1078.001
    ///
    /// "DEFAULT_ACCOUNTS"
    #[serde(rename="DEFAULT_ACCOUNTS")]
    DEFAULTACCOUNTS,
    

    /// T1078.003
    ///
    /// "LOCAL_ACCOUNTS"
    #[serde(rename="LOCAL_ACCOUNTS")]
    LOCALACCOUNTS,
    

    /// T1078.004
    ///
    /// "CLOUD_ACCOUNTS"
    #[serde(rename="CLOUD_ACCOUNTS")]
    CLOUDACCOUNTS,
    

    /// T1090
    ///
    /// "PROXY"
    #[serde(rename="PROXY")]
    PROXY,
    

    /// T1090.002
    ///
    /// "EXTERNAL_PROXY"
    #[serde(rename="EXTERNAL_PROXY")]
    EXTERNALPROXY,
    

    /// T1090.003
    ///
    /// "MULTI_HOP_PROXY"
    #[serde(rename="MULTI_HOP_PROXY")]
    MULTIHOPPROXY,
    

    /// T1098
    ///
    /// "ACCOUNT_MANIPULATION"
    #[serde(rename="ACCOUNT_MANIPULATION")]
    ACCOUNTMANIPULATION,
    

    /// T1098.001
    ///
    /// "ADDITIONAL_CLOUD_CREDENTIALS"
    #[serde(rename="ADDITIONAL_CLOUD_CREDENTIALS")]
    ADDITIONALCLOUDCREDENTIALS,
    

    /// T1098.004
    ///
    /// "SSH_AUTHORIZED_KEYS"
    #[serde(rename="SSH_AUTHORIZED_KEYS")]
    SSHAUTHORIZEDKEYS,
    

    /// T1098.006
    ///
    /// "ADDITIONAL_CONTAINER_CLUSTER_ROLES"
    #[serde(rename="ADDITIONAL_CONTAINER_CLUSTER_ROLES")]
    ADDITIONALCONTAINERCLUSTERROLES,
    

    /// T1105
    ///
    /// "INGRESS_TOOL_TRANSFER"
    #[serde(rename="INGRESS_TOOL_TRANSFER")]
    INGRESSTOOLTRANSFER,
    

    /// T1106
    ///
    /// "NATIVE_API"
    #[serde(rename="NATIVE_API")]
    NATIVEAPI,
    

    /// T1110
    ///
    /// "BRUTE_FORCE"
    #[serde(rename="BRUTE_FORCE")]
    BRUTEFORCE,
    

    /// T1129
    ///
    /// "SHARED_MODULES"
    #[serde(rename="SHARED_MODULES")]
    SHAREDMODULES,
    

    /// T1134
    ///
    /// "ACCESS_TOKEN_MANIPULATION"
    #[serde(rename="ACCESS_TOKEN_MANIPULATION")]
    ACCESSTOKENMANIPULATION,
    

    /// T1134.001
    ///
    /// "TOKEN_IMPERSONATION_OR_THEFT"
    #[serde(rename="TOKEN_IMPERSONATION_OR_THEFT")]
    TOKENIMPERSONATIONORTHEFT,
    

    /// T1190
    ///
    /// "EXPLOIT_PUBLIC_FACING_APPLICATION"
    #[serde(rename="EXPLOIT_PUBLIC_FACING_APPLICATION")]
    EXPLOITPUBLICFACINGAPPLICATION,
    

    /// T1484
    ///
    /// "DOMAIN_POLICY_MODIFICATION"
    #[serde(rename="DOMAIN_POLICY_MODIFICATION")]
    DOMAINPOLICYMODIFICATION,
    

    /// T1485
    ///
    /// "DATA_DESTRUCTION"
    #[serde(rename="DATA_DESTRUCTION")]
    DATADESTRUCTION,
    

    /// T1489
    ///
    /// "SERVICE_STOP"
    #[serde(rename="SERVICE_STOP")]
    SERVICESTOP,
    

    /// T1490
    ///
    /// "INHIBIT_SYSTEM_RECOVERY"
    #[serde(rename="INHIBIT_SYSTEM_RECOVERY")]
    INHIBITSYSTEMRECOVERY,
    

    /// T1496
    ///
    /// "RESOURCE_HIJACKING"
    #[serde(rename="RESOURCE_HIJACKING")]
    RESOURCEHIJACKING,
    

    /// T1498
    ///
    /// "NETWORK_DENIAL_OF_SERVICE"
    #[serde(rename="NETWORK_DENIAL_OF_SERVICE")]
    NETWORKDENIALOFSERVICE,
    

    /// T1526
    ///
    /// "CLOUD_SERVICE_DISCOVERY"
    #[serde(rename="CLOUD_SERVICE_DISCOVERY")]
    CLOUDSERVICEDISCOVERY,
    

    /// T1528
    ///
    /// "STEAL_APPLICATION_ACCESS_TOKEN"
    #[serde(rename="STEAL_APPLICATION_ACCESS_TOKEN")]
    STEALAPPLICATIONACCESSTOKEN,
    

    /// T1531
    ///
    /// "ACCOUNT_ACCESS_REMOVAL"
    #[serde(rename="ACCOUNT_ACCESS_REMOVAL")]
    ACCOUNTACCESSREMOVAL,
    

    /// T1539
    ///
    /// "STEAL_WEB_SESSION_COOKIE"
    #[serde(rename="STEAL_WEB_SESSION_COOKIE")]
    STEALWEBSESSIONCOOKIE,
    

    /// T1543
    ///
    /// "CREATE_OR_MODIFY_SYSTEM_PROCESS"
    #[serde(rename="CREATE_OR_MODIFY_SYSTEM_PROCESS")]
    CREATEORMODIFYSYSTEMPROCESS,
    

    /// T1548
    ///
    /// "ABUSE_ELEVATION_CONTROL_MECHANISM"
    #[serde(rename="ABUSE_ELEVATION_CONTROL_MECHANISM")]
    ABUSEELEVATIONCONTROLMECHANISM,
    

    /// T1552
    ///
    /// "UNSECURED_CREDENTIALS"
    #[serde(rename="UNSECURED_CREDENTIALS")]
    UNSECUREDCREDENTIALS,
    

    /// T1556
    ///
    /// "MODIFY_AUTHENTICATION_PROCESS"
    #[serde(rename="MODIFY_AUTHENTICATION_PROCESS")]
    MODIFYAUTHENTICATIONPROCESS,
    

    /// T1562
    ///
    /// "IMPAIR_DEFENSES"
    #[serde(rename="IMPAIR_DEFENSES")]
    IMPAIRDEFENSES,
    

    /// T1562.001
    ///
    /// "DISABLE_OR_MODIFY_TOOLS"
    #[serde(rename="DISABLE_OR_MODIFY_TOOLS")]
    DISABLEORMODIFYTOOLS,
    

    /// T1567
    ///
    /// "EXFILTRATION_OVER_WEB_SERVICE"
    #[serde(rename="EXFILTRATION_OVER_WEB_SERVICE")]
    EXFILTRATIONOVERWEBSERVICE,
    

    /// T1567.002
    ///
    /// "EXFILTRATION_TO_CLOUD_STORAGE"
    #[serde(rename="EXFILTRATION_TO_CLOUD_STORAGE")]
    EXFILTRATIONTOCLOUDSTORAGE,
    

    /// T1568
    ///
    /// "DYNAMIC_RESOLUTION"
    #[serde(rename="DYNAMIC_RESOLUTION")]
    DYNAMICRESOLUTION,
    

    /// T1570
    ///
    /// "LATERAL_TOOL_TRANSFER"
    #[serde(rename="LATERAL_TOOL_TRANSFER")]
    LATERALTOOLTRANSFER,
    

    /// T1578
    ///
    /// "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE"
    #[serde(rename="MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE")]
    MODIFYCLOUDCOMPUTEINFRASTRUCTURE,
    

    /// T1578.001
    ///
    /// "CREATE_SNAPSHOT"
    #[serde(rename="CREATE_SNAPSHOT")]
    CREATESNAPSHOT,
    

    /// T1580
    ///
    /// "CLOUD_INFRASTRUCTURE_DISCOVERY"
    #[serde(rename="CLOUD_INFRASTRUCTURE_DISCOVERY")]
    CLOUDINFRASTRUCTUREDISCOVERY,
    

    /// T1588
    ///
    /// "OBTAIN_CAPABILITIES"
    #[serde(rename="OBTAIN_CAPABILITIES")]
    OBTAINCAPABILITIES,
    

    /// T1595
    ///
    /// "ACTIVE_SCANNING"
    #[serde(rename="ACTIVE_SCANNING")]
    ACTIVESCANNING,
    

    /// T1595.001
    ///
    /// "SCANNING_IP_BLOCKS"
    #[serde(rename="SCANNING_IP_BLOCKS")]
    SCANNINGIPBLOCKS,
    

    /// T1613
    ///
    /// "CONTAINER_AND_RESOURCE_DISCOVERY"
    #[serde(rename="CONTAINER_AND_RESOURCE_DISCOVERY")]
    CONTAINERANDRESOURCEDISCOVERY,
}

impl AsRef<str> for MitreAttackAdditionalTechniquesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MitreAttackAdditionalTechniquesEnum::TECHNIQUEUNSPECIFIED => "TECHNIQUE_UNSPECIFIED",
            MitreAttackAdditionalTechniquesEnum::MASQUERADING => "MASQUERADING",
            MitreAttackAdditionalTechniquesEnum::MATCHLEGITIMATENAMEORLOCATION => "MATCH_LEGITIMATE_NAME_OR_LOCATION",
            MitreAttackAdditionalTechniquesEnum::BOOTORLOGONINITIALIZATIONSCRIPTS => "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS",
            MitreAttackAdditionalTechniquesEnum::STARTUPITEMS => "STARTUP_ITEMS",
            MitreAttackAdditionalTechniquesEnum::NETWORKSERVICEDISCOVERY => "NETWORK_SERVICE_DISCOVERY",
            MitreAttackAdditionalTechniquesEnum::PROCESSDISCOVERY => "PROCESS_DISCOVERY",
            MitreAttackAdditionalTechniquesEnum::COMMANDANDSCRIPTINGINTERPRETER => "COMMAND_AND_SCRIPTING_INTERPRETER",
            MitreAttackAdditionalTechniquesEnum::UNIXSHELL => "UNIX_SHELL",
            MitreAttackAdditionalTechniquesEnum::PYTHON => "PYTHON",
            MitreAttackAdditionalTechniquesEnum::PERMISSIONGROUPSDISCOVERY => "PERMISSION_GROUPS_DISCOVERY",
            MitreAttackAdditionalTechniquesEnum::CLOUDGROUPS => "CLOUD_GROUPS",
            MitreAttackAdditionalTechniquesEnum::APPLICATIONLAYERPROTOCOL => "APPLICATION_LAYER_PROTOCOL",
            MitreAttackAdditionalTechniquesEnum::DNS => "DNS",
            MitreAttackAdditionalTechniquesEnum::SOFTWAREDEPLOYMENTTOOLS => "SOFTWARE_DEPLOYMENT_TOOLS",
            MitreAttackAdditionalTechniquesEnum::VALIDACCOUNTS => "VALID_ACCOUNTS",
            MitreAttackAdditionalTechniquesEnum::DEFAULTACCOUNTS => "DEFAULT_ACCOUNTS",
            MitreAttackAdditionalTechniquesEnum::LOCALACCOUNTS => "LOCAL_ACCOUNTS",
            MitreAttackAdditionalTechniquesEnum::CLOUDACCOUNTS => "CLOUD_ACCOUNTS",
            MitreAttackAdditionalTechniquesEnum::PROXY => "PROXY",
            MitreAttackAdditionalTechniquesEnum::EXTERNALPROXY => "EXTERNAL_PROXY",
            MitreAttackAdditionalTechniquesEnum::MULTIHOPPROXY => "MULTI_HOP_PROXY",
            MitreAttackAdditionalTechniquesEnum::ACCOUNTMANIPULATION => "ACCOUNT_MANIPULATION",
            MitreAttackAdditionalTechniquesEnum::ADDITIONALCLOUDCREDENTIALS => "ADDITIONAL_CLOUD_CREDENTIALS",
            MitreAttackAdditionalTechniquesEnum::SSHAUTHORIZEDKEYS => "SSH_AUTHORIZED_KEYS",
            MitreAttackAdditionalTechniquesEnum::ADDITIONALCONTAINERCLUSTERROLES => "ADDITIONAL_CONTAINER_CLUSTER_ROLES",
            MitreAttackAdditionalTechniquesEnum::INGRESSTOOLTRANSFER => "INGRESS_TOOL_TRANSFER",
            MitreAttackAdditionalTechniquesEnum::NATIVEAPI => "NATIVE_API",
            MitreAttackAdditionalTechniquesEnum::BRUTEFORCE => "BRUTE_FORCE",
            MitreAttackAdditionalTechniquesEnum::SHAREDMODULES => "SHARED_MODULES",
            MitreAttackAdditionalTechniquesEnum::ACCESSTOKENMANIPULATION => "ACCESS_TOKEN_MANIPULATION",
            MitreAttackAdditionalTechniquesEnum::TOKENIMPERSONATIONORTHEFT => "TOKEN_IMPERSONATION_OR_THEFT",
            MitreAttackAdditionalTechniquesEnum::EXPLOITPUBLICFACINGAPPLICATION => "EXPLOIT_PUBLIC_FACING_APPLICATION",
            MitreAttackAdditionalTechniquesEnum::DOMAINPOLICYMODIFICATION => "DOMAIN_POLICY_MODIFICATION",
            MitreAttackAdditionalTechniquesEnum::DATADESTRUCTION => "DATA_DESTRUCTION",
            MitreAttackAdditionalTechniquesEnum::SERVICESTOP => "SERVICE_STOP",
            MitreAttackAdditionalTechniquesEnum::INHIBITSYSTEMRECOVERY => "INHIBIT_SYSTEM_RECOVERY",
            MitreAttackAdditionalTechniquesEnum::RESOURCEHIJACKING => "RESOURCE_HIJACKING",
            MitreAttackAdditionalTechniquesEnum::NETWORKDENIALOFSERVICE => "NETWORK_DENIAL_OF_SERVICE",
            MitreAttackAdditionalTechniquesEnum::CLOUDSERVICEDISCOVERY => "CLOUD_SERVICE_DISCOVERY",
            MitreAttackAdditionalTechniquesEnum::STEALAPPLICATIONACCESSTOKEN => "STEAL_APPLICATION_ACCESS_TOKEN",
            MitreAttackAdditionalTechniquesEnum::ACCOUNTACCESSREMOVAL => "ACCOUNT_ACCESS_REMOVAL",
            MitreAttackAdditionalTechniquesEnum::STEALWEBSESSIONCOOKIE => "STEAL_WEB_SESSION_COOKIE",
            MitreAttackAdditionalTechniquesEnum::CREATEORMODIFYSYSTEMPROCESS => "CREATE_OR_MODIFY_SYSTEM_PROCESS",
            MitreAttackAdditionalTechniquesEnum::ABUSEELEVATIONCONTROLMECHANISM => "ABUSE_ELEVATION_CONTROL_MECHANISM",
            MitreAttackAdditionalTechniquesEnum::UNSECUREDCREDENTIALS => "UNSECURED_CREDENTIALS",
            MitreAttackAdditionalTechniquesEnum::MODIFYAUTHENTICATIONPROCESS => "MODIFY_AUTHENTICATION_PROCESS",
            MitreAttackAdditionalTechniquesEnum::IMPAIRDEFENSES => "IMPAIR_DEFENSES",
            MitreAttackAdditionalTechniquesEnum::DISABLEORMODIFYTOOLS => "DISABLE_OR_MODIFY_TOOLS",
            MitreAttackAdditionalTechniquesEnum::EXFILTRATIONOVERWEBSERVICE => "EXFILTRATION_OVER_WEB_SERVICE",
            MitreAttackAdditionalTechniquesEnum::EXFILTRATIONTOCLOUDSTORAGE => "EXFILTRATION_TO_CLOUD_STORAGE",
            MitreAttackAdditionalTechniquesEnum::DYNAMICRESOLUTION => "DYNAMIC_RESOLUTION",
            MitreAttackAdditionalTechniquesEnum::LATERALTOOLTRANSFER => "LATERAL_TOOL_TRANSFER",
            MitreAttackAdditionalTechniquesEnum::MODIFYCLOUDCOMPUTEINFRASTRUCTURE => "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE",
            MitreAttackAdditionalTechniquesEnum::CREATESNAPSHOT => "CREATE_SNAPSHOT",
            MitreAttackAdditionalTechniquesEnum::CLOUDINFRASTRUCTUREDISCOVERY => "CLOUD_INFRASTRUCTURE_DISCOVERY",
            MitreAttackAdditionalTechniquesEnum::OBTAINCAPABILITIES => "OBTAIN_CAPABILITIES",
            MitreAttackAdditionalTechniquesEnum::ACTIVESCANNING => "ACTIVE_SCANNING",
            MitreAttackAdditionalTechniquesEnum::SCANNINGIPBLOCKS => "SCANNING_IP_BLOCKS",
            MitreAttackAdditionalTechniquesEnum::CONTAINERANDRESOURCEDISCOVERY => "CONTAINER_AND_RESOURCE_DISCOVERY",
        }
    }
}

impl std::convert::TryFrom< &str> for MitreAttackAdditionalTechniquesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TECHNIQUE_UNSPECIFIED" => Ok(MitreAttackAdditionalTechniquesEnum::TECHNIQUEUNSPECIFIED),
           "MASQUERADING" => Ok(MitreAttackAdditionalTechniquesEnum::MASQUERADING),
           "MATCH_LEGITIMATE_NAME_OR_LOCATION" => Ok(MitreAttackAdditionalTechniquesEnum::MATCHLEGITIMATENAMEORLOCATION),
           "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS" => Ok(MitreAttackAdditionalTechniquesEnum::BOOTORLOGONINITIALIZATIONSCRIPTS),
           "STARTUP_ITEMS" => Ok(MitreAttackAdditionalTechniquesEnum::STARTUPITEMS),
           "NETWORK_SERVICE_DISCOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::NETWORKSERVICEDISCOVERY),
           "PROCESS_DISCOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::PROCESSDISCOVERY),
           "COMMAND_AND_SCRIPTING_INTERPRETER" => Ok(MitreAttackAdditionalTechniquesEnum::COMMANDANDSCRIPTINGINTERPRETER),
           "UNIX_SHELL" => Ok(MitreAttackAdditionalTechniquesEnum::UNIXSHELL),
           "PYTHON" => Ok(MitreAttackAdditionalTechniquesEnum::PYTHON),
           "PERMISSION_GROUPS_DISCOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::PERMISSIONGROUPSDISCOVERY),
           "CLOUD_GROUPS" => Ok(MitreAttackAdditionalTechniquesEnum::CLOUDGROUPS),
           "APPLICATION_LAYER_PROTOCOL" => Ok(MitreAttackAdditionalTechniquesEnum::APPLICATIONLAYERPROTOCOL),
           "DNS" => Ok(MitreAttackAdditionalTechniquesEnum::DNS),
           "SOFTWARE_DEPLOYMENT_TOOLS" => Ok(MitreAttackAdditionalTechniquesEnum::SOFTWAREDEPLOYMENTTOOLS),
           "VALID_ACCOUNTS" => Ok(MitreAttackAdditionalTechniquesEnum::VALIDACCOUNTS),
           "DEFAULT_ACCOUNTS" => Ok(MitreAttackAdditionalTechniquesEnum::DEFAULTACCOUNTS),
           "LOCAL_ACCOUNTS" => Ok(MitreAttackAdditionalTechniquesEnum::LOCALACCOUNTS),
           "CLOUD_ACCOUNTS" => Ok(MitreAttackAdditionalTechniquesEnum::CLOUDACCOUNTS),
           "PROXY" => Ok(MitreAttackAdditionalTechniquesEnum::PROXY),
           "EXTERNAL_PROXY" => Ok(MitreAttackAdditionalTechniquesEnum::EXTERNALPROXY),
           "MULTI_HOP_PROXY" => Ok(MitreAttackAdditionalTechniquesEnum::MULTIHOPPROXY),
           "ACCOUNT_MANIPULATION" => Ok(MitreAttackAdditionalTechniquesEnum::ACCOUNTMANIPULATION),
           "ADDITIONAL_CLOUD_CREDENTIALS" => Ok(MitreAttackAdditionalTechniquesEnum::ADDITIONALCLOUDCREDENTIALS),
           "SSH_AUTHORIZED_KEYS" => Ok(MitreAttackAdditionalTechniquesEnum::SSHAUTHORIZEDKEYS),
           "ADDITIONAL_CONTAINER_CLUSTER_ROLES" => Ok(MitreAttackAdditionalTechniquesEnum::ADDITIONALCONTAINERCLUSTERROLES),
           "INGRESS_TOOL_TRANSFER" => Ok(MitreAttackAdditionalTechniquesEnum::INGRESSTOOLTRANSFER),
           "NATIVE_API" => Ok(MitreAttackAdditionalTechniquesEnum::NATIVEAPI),
           "BRUTE_FORCE" => Ok(MitreAttackAdditionalTechniquesEnum::BRUTEFORCE),
           "SHARED_MODULES" => Ok(MitreAttackAdditionalTechniquesEnum::SHAREDMODULES),
           "ACCESS_TOKEN_MANIPULATION" => Ok(MitreAttackAdditionalTechniquesEnum::ACCESSTOKENMANIPULATION),
           "TOKEN_IMPERSONATION_OR_THEFT" => Ok(MitreAttackAdditionalTechniquesEnum::TOKENIMPERSONATIONORTHEFT),
           "EXPLOIT_PUBLIC_FACING_APPLICATION" => Ok(MitreAttackAdditionalTechniquesEnum::EXPLOITPUBLICFACINGAPPLICATION),
           "DOMAIN_POLICY_MODIFICATION" => Ok(MitreAttackAdditionalTechniquesEnum::DOMAINPOLICYMODIFICATION),
           "DATA_DESTRUCTION" => Ok(MitreAttackAdditionalTechniquesEnum::DATADESTRUCTION),
           "SERVICE_STOP" => Ok(MitreAttackAdditionalTechniquesEnum::SERVICESTOP),
           "INHIBIT_SYSTEM_RECOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::INHIBITSYSTEMRECOVERY),
           "RESOURCE_HIJACKING" => Ok(MitreAttackAdditionalTechniquesEnum::RESOURCEHIJACKING),
           "NETWORK_DENIAL_OF_SERVICE" => Ok(MitreAttackAdditionalTechniquesEnum::NETWORKDENIALOFSERVICE),
           "CLOUD_SERVICE_DISCOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::CLOUDSERVICEDISCOVERY),
           "STEAL_APPLICATION_ACCESS_TOKEN" => Ok(MitreAttackAdditionalTechniquesEnum::STEALAPPLICATIONACCESSTOKEN),
           "ACCOUNT_ACCESS_REMOVAL" => Ok(MitreAttackAdditionalTechniquesEnum::ACCOUNTACCESSREMOVAL),
           "STEAL_WEB_SESSION_COOKIE" => Ok(MitreAttackAdditionalTechniquesEnum::STEALWEBSESSIONCOOKIE),
           "CREATE_OR_MODIFY_SYSTEM_PROCESS" => Ok(MitreAttackAdditionalTechniquesEnum::CREATEORMODIFYSYSTEMPROCESS),
           "ABUSE_ELEVATION_CONTROL_MECHANISM" => Ok(MitreAttackAdditionalTechniquesEnum::ABUSEELEVATIONCONTROLMECHANISM),
           "UNSECURED_CREDENTIALS" => Ok(MitreAttackAdditionalTechniquesEnum::UNSECUREDCREDENTIALS),
           "MODIFY_AUTHENTICATION_PROCESS" => Ok(MitreAttackAdditionalTechniquesEnum::MODIFYAUTHENTICATIONPROCESS),
           "IMPAIR_DEFENSES" => Ok(MitreAttackAdditionalTechniquesEnum::IMPAIRDEFENSES),
           "DISABLE_OR_MODIFY_TOOLS" => Ok(MitreAttackAdditionalTechniquesEnum::DISABLEORMODIFYTOOLS),
           "EXFILTRATION_OVER_WEB_SERVICE" => Ok(MitreAttackAdditionalTechniquesEnum::EXFILTRATIONOVERWEBSERVICE),
           "EXFILTRATION_TO_CLOUD_STORAGE" => Ok(MitreAttackAdditionalTechniquesEnum::EXFILTRATIONTOCLOUDSTORAGE),
           "DYNAMIC_RESOLUTION" => Ok(MitreAttackAdditionalTechniquesEnum::DYNAMICRESOLUTION),
           "LATERAL_TOOL_TRANSFER" => Ok(MitreAttackAdditionalTechniquesEnum::LATERALTOOLTRANSFER),
           "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE" => Ok(MitreAttackAdditionalTechniquesEnum::MODIFYCLOUDCOMPUTEINFRASTRUCTURE),
           "CREATE_SNAPSHOT" => Ok(MitreAttackAdditionalTechniquesEnum::CREATESNAPSHOT),
           "CLOUD_INFRASTRUCTURE_DISCOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::CLOUDINFRASTRUCTUREDISCOVERY),
           "OBTAIN_CAPABILITIES" => Ok(MitreAttackAdditionalTechniquesEnum::OBTAINCAPABILITIES),
           "ACTIVE_SCANNING" => Ok(MitreAttackAdditionalTechniquesEnum::ACTIVESCANNING),
           "SCANNING_IP_BLOCKS" => Ok(MitreAttackAdditionalTechniquesEnum::SCANNINGIPBLOCKS),
           "CONTAINER_AND_RESOURCE_DISCOVERY" => Ok(MitreAttackAdditionalTechniquesEnum::CONTAINERANDRESOURCEDISCOVERY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MitreAttackAdditionalTechniquesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MitreAttackPrimaryTacticEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The MITRE ATT&CK tactic most closely represented by this finding, if any.
pub enum MitreAttackPrimaryTacticEnum {
    

    /// Unspecified value.
    ///
    /// "TACTIC_UNSPECIFIED"
    #[serde(rename="TACTIC_UNSPECIFIED")]
    TACTICUNSPECIFIED,
    

    /// TA0043
    ///
    /// "RECONNAISSANCE"
    #[serde(rename="RECONNAISSANCE")]
    RECONNAISSANCE,
    

    /// TA0042
    ///
    /// "RESOURCE_DEVELOPMENT"
    #[serde(rename="RESOURCE_DEVELOPMENT")]
    RESOURCEDEVELOPMENT,
    

    /// TA0001
    ///
    /// "INITIAL_ACCESS"
    #[serde(rename="INITIAL_ACCESS")]
    INITIALACCESS,
    

    /// TA0002
    ///
    /// "EXECUTION"
    #[serde(rename="EXECUTION")]
    EXECUTION,
    

    /// TA0003
    ///
    /// "PERSISTENCE"
    #[serde(rename="PERSISTENCE")]
    PERSISTENCE,
    

    /// TA0004
    ///
    /// "PRIVILEGE_ESCALATION"
    #[serde(rename="PRIVILEGE_ESCALATION")]
    PRIVILEGEESCALATION,
    

    /// TA0005
    ///
    /// "DEFENSE_EVASION"
    #[serde(rename="DEFENSE_EVASION")]
    DEFENSEEVASION,
    

    /// TA0006
    ///
    /// "CREDENTIAL_ACCESS"
    #[serde(rename="CREDENTIAL_ACCESS")]
    CREDENTIALACCESS,
    

    /// TA0007
    ///
    /// "DISCOVERY"
    #[serde(rename="DISCOVERY")]
    DISCOVERY,
    

    /// TA0008
    ///
    /// "LATERAL_MOVEMENT"
    #[serde(rename="LATERAL_MOVEMENT")]
    LATERALMOVEMENT,
    

    /// TA0009
    ///
    /// "COLLECTION"
    #[serde(rename="COLLECTION")]
    COLLECTION,
    

    /// TA0011
    ///
    /// "COMMAND_AND_CONTROL"
    #[serde(rename="COMMAND_AND_CONTROL")]
    COMMANDANDCONTROL,
    

    /// TA0010
    ///
    /// "EXFILTRATION"
    #[serde(rename="EXFILTRATION")]
    EXFILTRATION,
    

    /// TA0040
    ///
    /// "IMPACT"
    #[serde(rename="IMPACT")]
    IMPACT,
}

impl AsRef<str> for MitreAttackPrimaryTacticEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MitreAttackPrimaryTacticEnum::TACTICUNSPECIFIED => "TACTIC_UNSPECIFIED",
            MitreAttackPrimaryTacticEnum::RECONNAISSANCE => "RECONNAISSANCE",
            MitreAttackPrimaryTacticEnum::RESOURCEDEVELOPMENT => "RESOURCE_DEVELOPMENT",
            MitreAttackPrimaryTacticEnum::INITIALACCESS => "INITIAL_ACCESS",
            MitreAttackPrimaryTacticEnum::EXECUTION => "EXECUTION",
            MitreAttackPrimaryTacticEnum::PERSISTENCE => "PERSISTENCE",
            MitreAttackPrimaryTacticEnum::PRIVILEGEESCALATION => "PRIVILEGE_ESCALATION",
            MitreAttackPrimaryTacticEnum::DEFENSEEVASION => "DEFENSE_EVASION",
            MitreAttackPrimaryTacticEnum::CREDENTIALACCESS => "CREDENTIAL_ACCESS",
            MitreAttackPrimaryTacticEnum::DISCOVERY => "DISCOVERY",
            MitreAttackPrimaryTacticEnum::LATERALMOVEMENT => "LATERAL_MOVEMENT",
            MitreAttackPrimaryTacticEnum::COLLECTION => "COLLECTION",
            MitreAttackPrimaryTacticEnum::COMMANDANDCONTROL => "COMMAND_AND_CONTROL",
            MitreAttackPrimaryTacticEnum::EXFILTRATION => "EXFILTRATION",
            MitreAttackPrimaryTacticEnum::IMPACT => "IMPACT",
        }
    }
}

impl std::convert::TryFrom< &str> for MitreAttackPrimaryTacticEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TACTIC_UNSPECIFIED" => Ok(MitreAttackPrimaryTacticEnum::TACTICUNSPECIFIED),
           "RECONNAISSANCE" => Ok(MitreAttackPrimaryTacticEnum::RECONNAISSANCE),
           "RESOURCE_DEVELOPMENT" => Ok(MitreAttackPrimaryTacticEnum::RESOURCEDEVELOPMENT),
           "INITIAL_ACCESS" => Ok(MitreAttackPrimaryTacticEnum::INITIALACCESS),
           "EXECUTION" => Ok(MitreAttackPrimaryTacticEnum::EXECUTION),
           "PERSISTENCE" => Ok(MitreAttackPrimaryTacticEnum::PERSISTENCE),
           "PRIVILEGE_ESCALATION" => Ok(MitreAttackPrimaryTacticEnum::PRIVILEGEESCALATION),
           "DEFENSE_EVASION" => Ok(MitreAttackPrimaryTacticEnum::DEFENSEEVASION),
           "CREDENTIAL_ACCESS" => Ok(MitreAttackPrimaryTacticEnum::CREDENTIALACCESS),
           "DISCOVERY" => Ok(MitreAttackPrimaryTacticEnum::DISCOVERY),
           "LATERAL_MOVEMENT" => Ok(MitreAttackPrimaryTacticEnum::LATERALMOVEMENT),
           "COLLECTION" => Ok(MitreAttackPrimaryTacticEnum::COLLECTION),
           "COMMAND_AND_CONTROL" => Ok(MitreAttackPrimaryTacticEnum::COMMANDANDCONTROL),
           "EXFILTRATION" => Ok(MitreAttackPrimaryTacticEnum::EXFILTRATION),
           "IMPACT" => Ok(MitreAttackPrimaryTacticEnum::IMPACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MitreAttackPrimaryTacticEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MitreAttackPrimaryTechniquesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The MITRE ATT&CK technique most closely represented by this finding, if any. primary_techniques is a repeated field because there are multiple levels of MITRE ATT&CK techniques. If the technique most closely represented by this finding is a sub-technique (e.g. `SCANNING_IP_BLOCKS`), both the sub-technique and its parent technique(s) will be listed (e.g. `SCANNING_IP_BLOCKS`, `ACTIVE_SCANNING`).
pub enum MitreAttackPrimaryTechniquesEnum {
    

    /// Unspecified value.
    ///
    /// "TECHNIQUE_UNSPECIFIED"
    #[serde(rename="TECHNIQUE_UNSPECIFIED")]
    TECHNIQUEUNSPECIFIED,
    

    /// T1036
    ///
    /// "MASQUERADING"
    #[serde(rename="MASQUERADING")]
    MASQUERADING,
    

    /// T1036.005
    ///
    /// "MATCH_LEGITIMATE_NAME_OR_LOCATION"
    #[serde(rename="MATCH_LEGITIMATE_NAME_OR_LOCATION")]
    MATCHLEGITIMATENAMEORLOCATION,
    

    /// T1037
    ///
    /// "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS"
    #[serde(rename="BOOT_OR_LOGON_INITIALIZATION_SCRIPTS")]
    BOOTORLOGONINITIALIZATIONSCRIPTS,
    

    /// T1037.005
    ///
    /// "STARTUP_ITEMS"
    #[serde(rename="STARTUP_ITEMS")]
    STARTUPITEMS,
    

    /// T1046
    ///
    /// "NETWORK_SERVICE_DISCOVERY"
    #[serde(rename="NETWORK_SERVICE_DISCOVERY")]
    NETWORKSERVICEDISCOVERY,
    

    /// T1057
    ///
    /// "PROCESS_DISCOVERY"
    #[serde(rename="PROCESS_DISCOVERY")]
    PROCESSDISCOVERY,
    

    /// T1059
    ///
    /// "COMMAND_AND_SCRIPTING_INTERPRETER"
    #[serde(rename="COMMAND_AND_SCRIPTING_INTERPRETER")]
    COMMANDANDSCRIPTINGINTERPRETER,
    

    /// T1059.004
    ///
    /// "UNIX_SHELL"
    #[serde(rename="UNIX_SHELL")]
    UNIXSHELL,
    

    /// T1059.006
    ///
    /// "PYTHON"
    #[serde(rename="PYTHON")]
    PYTHON,
    

    /// T1069
    ///
    /// "PERMISSION_GROUPS_DISCOVERY"
    #[serde(rename="PERMISSION_GROUPS_DISCOVERY")]
    PERMISSIONGROUPSDISCOVERY,
    

    /// T1069.003
    ///
    /// "CLOUD_GROUPS"
    #[serde(rename="CLOUD_GROUPS")]
    CLOUDGROUPS,
    

    /// T1071
    ///
    /// "APPLICATION_LAYER_PROTOCOL"
    #[serde(rename="APPLICATION_LAYER_PROTOCOL")]
    APPLICATIONLAYERPROTOCOL,
    

    /// T1071.004
    ///
    /// "DNS"
    #[serde(rename="DNS")]
    DNS,
    

    /// T1072
    ///
    /// "SOFTWARE_DEPLOYMENT_TOOLS"
    #[serde(rename="SOFTWARE_DEPLOYMENT_TOOLS")]
    SOFTWAREDEPLOYMENTTOOLS,
    

    /// T1078
    ///
    /// "VALID_ACCOUNTS"
    #[serde(rename="VALID_ACCOUNTS")]
    VALIDACCOUNTS,
    

    /// T1078.001
    ///
    /// "DEFAULT_ACCOUNTS"
    #[serde(rename="DEFAULT_ACCOUNTS")]
    DEFAULTACCOUNTS,
    

    /// T1078.003
    ///
    /// "LOCAL_ACCOUNTS"
    #[serde(rename="LOCAL_ACCOUNTS")]
    LOCALACCOUNTS,
    

    /// T1078.004
    ///
    /// "CLOUD_ACCOUNTS"
    #[serde(rename="CLOUD_ACCOUNTS")]
    CLOUDACCOUNTS,
    

    /// T1090
    ///
    /// "PROXY"
    #[serde(rename="PROXY")]
    PROXY,
    

    /// T1090.002
    ///
    /// "EXTERNAL_PROXY"
    #[serde(rename="EXTERNAL_PROXY")]
    EXTERNALPROXY,
    

    /// T1090.003
    ///
    /// "MULTI_HOP_PROXY"
    #[serde(rename="MULTI_HOP_PROXY")]
    MULTIHOPPROXY,
    

    /// T1098
    ///
    /// "ACCOUNT_MANIPULATION"
    #[serde(rename="ACCOUNT_MANIPULATION")]
    ACCOUNTMANIPULATION,
    

    /// T1098.001
    ///
    /// "ADDITIONAL_CLOUD_CREDENTIALS"
    #[serde(rename="ADDITIONAL_CLOUD_CREDENTIALS")]
    ADDITIONALCLOUDCREDENTIALS,
    

    /// T1098.004
    ///
    /// "SSH_AUTHORIZED_KEYS"
    #[serde(rename="SSH_AUTHORIZED_KEYS")]
    SSHAUTHORIZEDKEYS,
    

    /// T1098.006
    ///
    /// "ADDITIONAL_CONTAINER_CLUSTER_ROLES"
    #[serde(rename="ADDITIONAL_CONTAINER_CLUSTER_ROLES")]
    ADDITIONALCONTAINERCLUSTERROLES,
    

    /// T1105
    ///
    /// "INGRESS_TOOL_TRANSFER"
    #[serde(rename="INGRESS_TOOL_TRANSFER")]
    INGRESSTOOLTRANSFER,
    

    /// T1106
    ///
    /// "NATIVE_API"
    #[serde(rename="NATIVE_API")]
    NATIVEAPI,
    

    /// T1110
    ///
    /// "BRUTE_FORCE"
    #[serde(rename="BRUTE_FORCE")]
    BRUTEFORCE,
    

    /// T1129
    ///
    /// "SHARED_MODULES"
    #[serde(rename="SHARED_MODULES")]
    SHAREDMODULES,
    

    /// T1134
    ///
    /// "ACCESS_TOKEN_MANIPULATION"
    #[serde(rename="ACCESS_TOKEN_MANIPULATION")]
    ACCESSTOKENMANIPULATION,
    

    /// T1134.001
    ///
    /// "TOKEN_IMPERSONATION_OR_THEFT"
    #[serde(rename="TOKEN_IMPERSONATION_OR_THEFT")]
    TOKENIMPERSONATIONORTHEFT,
    

    /// T1190
    ///
    /// "EXPLOIT_PUBLIC_FACING_APPLICATION"
    #[serde(rename="EXPLOIT_PUBLIC_FACING_APPLICATION")]
    EXPLOITPUBLICFACINGAPPLICATION,
    

    /// T1484
    ///
    /// "DOMAIN_POLICY_MODIFICATION"
    #[serde(rename="DOMAIN_POLICY_MODIFICATION")]
    DOMAINPOLICYMODIFICATION,
    

    /// T1485
    ///
    /// "DATA_DESTRUCTION"
    #[serde(rename="DATA_DESTRUCTION")]
    DATADESTRUCTION,
    

    /// T1489
    ///
    /// "SERVICE_STOP"
    #[serde(rename="SERVICE_STOP")]
    SERVICESTOP,
    

    /// T1490
    ///
    /// "INHIBIT_SYSTEM_RECOVERY"
    #[serde(rename="INHIBIT_SYSTEM_RECOVERY")]
    INHIBITSYSTEMRECOVERY,
    

    /// T1496
    ///
    /// "RESOURCE_HIJACKING"
    #[serde(rename="RESOURCE_HIJACKING")]
    RESOURCEHIJACKING,
    

    /// T1498
    ///
    /// "NETWORK_DENIAL_OF_SERVICE"
    #[serde(rename="NETWORK_DENIAL_OF_SERVICE")]
    NETWORKDENIALOFSERVICE,
    

    /// T1526
    ///
    /// "CLOUD_SERVICE_DISCOVERY"
    #[serde(rename="CLOUD_SERVICE_DISCOVERY")]
    CLOUDSERVICEDISCOVERY,
    

    /// T1528
    ///
    /// "STEAL_APPLICATION_ACCESS_TOKEN"
    #[serde(rename="STEAL_APPLICATION_ACCESS_TOKEN")]
    STEALAPPLICATIONACCESSTOKEN,
    

    /// T1531
    ///
    /// "ACCOUNT_ACCESS_REMOVAL"
    #[serde(rename="ACCOUNT_ACCESS_REMOVAL")]
    ACCOUNTACCESSREMOVAL,
    

    /// T1539
    ///
    /// "STEAL_WEB_SESSION_COOKIE"
    #[serde(rename="STEAL_WEB_SESSION_COOKIE")]
    STEALWEBSESSIONCOOKIE,
    

    /// T1543
    ///
    /// "CREATE_OR_MODIFY_SYSTEM_PROCESS"
    #[serde(rename="CREATE_OR_MODIFY_SYSTEM_PROCESS")]
    CREATEORMODIFYSYSTEMPROCESS,
    

    /// T1548
    ///
    /// "ABUSE_ELEVATION_CONTROL_MECHANISM"
    #[serde(rename="ABUSE_ELEVATION_CONTROL_MECHANISM")]
    ABUSEELEVATIONCONTROLMECHANISM,
    

    /// T1552
    ///
    /// "UNSECURED_CREDENTIALS"
    #[serde(rename="UNSECURED_CREDENTIALS")]
    UNSECUREDCREDENTIALS,
    

    /// T1556
    ///
    /// "MODIFY_AUTHENTICATION_PROCESS"
    #[serde(rename="MODIFY_AUTHENTICATION_PROCESS")]
    MODIFYAUTHENTICATIONPROCESS,
    

    /// T1562
    ///
    /// "IMPAIR_DEFENSES"
    #[serde(rename="IMPAIR_DEFENSES")]
    IMPAIRDEFENSES,
    

    /// T1562.001
    ///
    /// "DISABLE_OR_MODIFY_TOOLS"
    #[serde(rename="DISABLE_OR_MODIFY_TOOLS")]
    DISABLEORMODIFYTOOLS,
    

    /// T1567
    ///
    /// "EXFILTRATION_OVER_WEB_SERVICE"
    #[serde(rename="EXFILTRATION_OVER_WEB_SERVICE")]
    EXFILTRATIONOVERWEBSERVICE,
    

    /// T1567.002
    ///
    /// "EXFILTRATION_TO_CLOUD_STORAGE"
    #[serde(rename="EXFILTRATION_TO_CLOUD_STORAGE")]
    EXFILTRATIONTOCLOUDSTORAGE,
    

    /// T1568
    ///
    /// "DYNAMIC_RESOLUTION"
    #[serde(rename="DYNAMIC_RESOLUTION")]
    DYNAMICRESOLUTION,
    

    /// T1570
    ///
    /// "LATERAL_TOOL_TRANSFER"
    #[serde(rename="LATERAL_TOOL_TRANSFER")]
    LATERALTOOLTRANSFER,
    

    /// T1578
    ///
    /// "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE"
    #[serde(rename="MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE")]
    MODIFYCLOUDCOMPUTEINFRASTRUCTURE,
    

    /// T1578.001
    ///
    /// "CREATE_SNAPSHOT"
    #[serde(rename="CREATE_SNAPSHOT")]
    CREATESNAPSHOT,
    

    /// T1580
    ///
    /// "CLOUD_INFRASTRUCTURE_DISCOVERY"
    #[serde(rename="CLOUD_INFRASTRUCTURE_DISCOVERY")]
    CLOUDINFRASTRUCTUREDISCOVERY,
    

    /// T1588
    ///
    /// "OBTAIN_CAPABILITIES"
    #[serde(rename="OBTAIN_CAPABILITIES")]
    OBTAINCAPABILITIES,
    

    /// T1595
    ///
    /// "ACTIVE_SCANNING"
    #[serde(rename="ACTIVE_SCANNING")]
    ACTIVESCANNING,
    

    /// T1595.001
    ///
    /// "SCANNING_IP_BLOCKS"
    #[serde(rename="SCANNING_IP_BLOCKS")]
    SCANNINGIPBLOCKS,
    

    /// T1613
    ///
    /// "CONTAINER_AND_RESOURCE_DISCOVERY"
    #[serde(rename="CONTAINER_AND_RESOURCE_DISCOVERY")]
    CONTAINERANDRESOURCEDISCOVERY,
}

impl AsRef<str> for MitreAttackPrimaryTechniquesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MitreAttackPrimaryTechniquesEnum::TECHNIQUEUNSPECIFIED => "TECHNIQUE_UNSPECIFIED",
            MitreAttackPrimaryTechniquesEnum::MASQUERADING => "MASQUERADING",
            MitreAttackPrimaryTechniquesEnum::MATCHLEGITIMATENAMEORLOCATION => "MATCH_LEGITIMATE_NAME_OR_LOCATION",
            MitreAttackPrimaryTechniquesEnum::BOOTORLOGONINITIALIZATIONSCRIPTS => "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS",
            MitreAttackPrimaryTechniquesEnum::STARTUPITEMS => "STARTUP_ITEMS",
            MitreAttackPrimaryTechniquesEnum::NETWORKSERVICEDISCOVERY => "NETWORK_SERVICE_DISCOVERY",
            MitreAttackPrimaryTechniquesEnum::PROCESSDISCOVERY => "PROCESS_DISCOVERY",
            MitreAttackPrimaryTechniquesEnum::COMMANDANDSCRIPTINGINTERPRETER => "COMMAND_AND_SCRIPTING_INTERPRETER",
            MitreAttackPrimaryTechniquesEnum::UNIXSHELL => "UNIX_SHELL",
            MitreAttackPrimaryTechniquesEnum::PYTHON => "PYTHON",
            MitreAttackPrimaryTechniquesEnum::PERMISSIONGROUPSDISCOVERY => "PERMISSION_GROUPS_DISCOVERY",
            MitreAttackPrimaryTechniquesEnum::CLOUDGROUPS => "CLOUD_GROUPS",
            MitreAttackPrimaryTechniquesEnum::APPLICATIONLAYERPROTOCOL => "APPLICATION_LAYER_PROTOCOL",
            MitreAttackPrimaryTechniquesEnum::DNS => "DNS",
            MitreAttackPrimaryTechniquesEnum::SOFTWAREDEPLOYMENTTOOLS => "SOFTWARE_DEPLOYMENT_TOOLS",
            MitreAttackPrimaryTechniquesEnum::VALIDACCOUNTS => "VALID_ACCOUNTS",
            MitreAttackPrimaryTechniquesEnum::DEFAULTACCOUNTS => "DEFAULT_ACCOUNTS",
            MitreAttackPrimaryTechniquesEnum::LOCALACCOUNTS => "LOCAL_ACCOUNTS",
            MitreAttackPrimaryTechniquesEnum::CLOUDACCOUNTS => "CLOUD_ACCOUNTS",
            MitreAttackPrimaryTechniquesEnum::PROXY => "PROXY",
            MitreAttackPrimaryTechniquesEnum::EXTERNALPROXY => "EXTERNAL_PROXY",
            MitreAttackPrimaryTechniquesEnum::MULTIHOPPROXY => "MULTI_HOP_PROXY",
            MitreAttackPrimaryTechniquesEnum::ACCOUNTMANIPULATION => "ACCOUNT_MANIPULATION",
            MitreAttackPrimaryTechniquesEnum::ADDITIONALCLOUDCREDENTIALS => "ADDITIONAL_CLOUD_CREDENTIALS",
            MitreAttackPrimaryTechniquesEnum::SSHAUTHORIZEDKEYS => "SSH_AUTHORIZED_KEYS",
            MitreAttackPrimaryTechniquesEnum::ADDITIONALCONTAINERCLUSTERROLES => "ADDITIONAL_CONTAINER_CLUSTER_ROLES",
            MitreAttackPrimaryTechniquesEnum::INGRESSTOOLTRANSFER => "INGRESS_TOOL_TRANSFER",
            MitreAttackPrimaryTechniquesEnum::NATIVEAPI => "NATIVE_API",
            MitreAttackPrimaryTechniquesEnum::BRUTEFORCE => "BRUTE_FORCE",
            MitreAttackPrimaryTechniquesEnum::SHAREDMODULES => "SHARED_MODULES",
            MitreAttackPrimaryTechniquesEnum::ACCESSTOKENMANIPULATION => "ACCESS_TOKEN_MANIPULATION",
            MitreAttackPrimaryTechniquesEnum::TOKENIMPERSONATIONORTHEFT => "TOKEN_IMPERSONATION_OR_THEFT",
            MitreAttackPrimaryTechniquesEnum::EXPLOITPUBLICFACINGAPPLICATION => "EXPLOIT_PUBLIC_FACING_APPLICATION",
            MitreAttackPrimaryTechniquesEnum::DOMAINPOLICYMODIFICATION => "DOMAIN_POLICY_MODIFICATION",
            MitreAttackPrimaryTechniquesEnum::DATADESTRUCTION => "DATA_DESTRUCTION",
            MitreAttackPrimaryTechniquesEnum::SERVICESTOP => "SERVICE_STOP",
            MitreAttackPrimaryTechniquesEnum::INHIBITSYSTEMRECOVERY => "INHIBIT_SYSTEM_RECOVERY",
            MitreAttackPrimaryTechniquesEnum::RESOURCEHIJACKING => "RESOURCE_HIJACKING",
            MitreAttackPrimaryTechniquesEnum::NETWORKDENIALOFSERVICE => "NETWORK_DENIAL_OF_SERVICE",
            MitreAttackPrimaryTechniquesEnum::CLOUDSERVICEDISCOVERY => "CLOUD_SERVICE_DISCOVERY",
            MitreAttackPrimaryTechniquesEnum::STEALAPPLICATIONACCESSTOKEN => "STEAL_APPLICATION_ACCESS_TOKEN",
            MitreAttackPrimaryTechniquesEnum::ACCOUNTACCESSREMOVAL => "ACCOUNT_ACCESS_REMOVAL",
            MitreAttackPrimaryTechniquesEnum::STEALWEBSESSIONCOOKIE => "STEAL_WEB_SESSION_COOKIE",
            MitreAttackPrimaryTechniquesEnum::CREATEORMODIFYSYSTEMPROCESS => "CREATE_OR_MODIFY_SYSTEM_PROCESS",
            MitreAttackPrimaryTechniquesEnum::ABUSEELEVATIONCONTROLMECHANISM => "ABUSE_ELEVATION_CONTROL_MECHANISM",
            MitreAttackPrimaryTechniquesEnum::UNSECUREDCREDENTIALS => "UNSECURED_CREDENTIALS",
            MitreAttackPrimaryTechniquesEnum::MODIFYAUTHENTICATIONPROCESS => "MODIFY_AUTHENTICATION_PROCESS",
            MitreAttackPrimaryTechniquesEnum::IMPAIRDEFENSES => "IMPAIR_DEFENSES",
            MitreAttackPrimaryTechniquesEnum::DISABLEORMODIFYTOOLS => "DISABLE_OR_MODIFY_TOOLS",
            MitreAttackPrimaryTechniquesEnum::EXFILTRATIONOVERWEBSERVICE => "EXFILTRATION_OVER_WEB_SERVICE",
            MitreAttackPrimaryTechniquesEnum::EXFILTRATIONTOCLOUDSTORAGE => "EXFILTRATION_TO_CLOUD_STORAGE",
            MitreAttackPrimaryTechniquesEnum::DYNAMICRESOLUTION => "DYNAMIC_RESOLUTION",
            MitreAttackPrimaryTechniquesEnum::LATERALTOOLTRANSFER => "LATERAL_TOOL_TRANSFER",
            MitreAttackPrimaryTechniquesEnum::MODIFYCLOUDCOMPUTEINFRASTRUCTURE => "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE",
            MitreAttackPrimaryTechniquesEnum::CREATESNAPSHOT => "CREATE_SNAPSHOT",
            MitreAttackPrimaryTechniquesEnum::CLOUDINFRASTRUCTUREDISCOVERY => "CLOUD_INFRASTRUCTURE_DISCOVERY",
            MitreAttackPrimaryTechniquesEnum::OBTAINCAPABILITIES => "OBTAIN_CAPABILITIES",
            MitreAttackPrimaryTechniquesEnum::ACTIVESCANNING => "ACTIVE_SCANNING",
            MitreAttackPrimaryTechniquesEnum::SCANNINGIPBLOCKS => "SCANNING_IP_BLOCKS",
            MitreAttackPrimaryTechniquesEnum::CONTAINERANDRESOURCEDISCOVERY => "CONTAINER_AND_RESOURCE_DISCOVERY",
        }
    }
}

impl std::convert::TryFrom< &str> for MitreAttackPrimaryTechniquesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TECHNIQUE_UNSPECIFIED" => Ok(MitreAttackPrimaryTechniquesEnum::TECHNIQUEUNSPECIFIED),
           "MASQUERADING" => Ok(MitreAttackPrimaryTechniquesEnum::MASQUERADING),
           "MATCH_LEGITIMATE_NAME_OR_LOCATION" => Ok(MitreAttackPrimaryTechniquesEnum::MATCHLEGITIMATENAMEORLOCATION),
           "BOOT_OR_LOGON_INITIALIZATION_SCRIPTS" => Ok(MitreAttackPrimaryTechniquesEnum::BOOTORLOGONINITIALIZATIONSCRIPTS),
           "STARTUP_ITEMS" => Ok(MitreAttackPrimaryTechniquesEnum::STARTUPITEMS),
           "NETWORK_SERVICE_DISCOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::NETWORKSERVICEDISCOVERY),
           "PROCESS_DISCOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::PROCESSDISCOVERY),
           "COMMAND_AND_SCRIPTING_INTERPRETER" => Ok(MitreAttackPrimaryTechniquesEnum::COMMANDANDSCRIPTINGINTERPRETER),
           "UNIX_SHELL" => Ok(MitreAttackPrimaryTechniquesEnum::UNIXSHELL),
           "PYTHON" => Ok(MitreAttackPrimaryTechniquesEnum::PYTHON),
           "PERMISSION_GROUPS_DISCOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::PERMISSIONGROUPSDISCOVERY),
           "CLOUD_GROUPS" => Ok(MitreAttackPrimaryTechniquesEnum::CLOUDGROUPS),
           "APPLICATION_LAYER_PROTOCOL" => Ok(MitreAttackPrimaryTechniquesEnum::APPLICATIONLAYERPROTOCOL),
           "DNS" => Ok(MitreAttackPrimaryTechniquesEnum::DNS),
           "SOFTWARE_DEPLOYMENT_TOOLS" => Ok(MitreAttackPrimaryTechniquesEnum::SOFTWAREDEPLOYMENTTOOLS),
           "VALID_ACCOUNTS" => Ok(MitreAttackPrimaryTechniquesEnum::VALIDACCOUNTS),
           "DEFAULT_ACCOUNTS" => Ok(MitreAttackPrimaryTechniquesEnum::DEFAULTACCOUNTS),
           "LOCAL_ACCOUNTS" => Ok(MitreAttackPrimaryTechniquesEnum::LOCALACCOUNTS),
           "CLOUD_ACCOUNTS" => Ok(MitreAttackPrimaryTechniquesEnum::CLOUDACCOUNTS),
           "PROXY" => Ok(MitreAttackPrimaryTechniquesEnum::PROXY),
           "EXTERNAL_PROXY" => Ok(MitreAttackPrimaryTechniquesEnum::EXTERNALPROXY),
           "MULTI_HOP_PROXY" => Ok(MitreAttackPrimaryTechniquesEnum::MULTIHOPPROXY),
           "ACCOUNT_MANIPULATION" => Ok(MitreAttackPrimaryTechniquesEnum::ACCOUNTMANIPULATION),
           "ADDITIONAL_CLOUD_CREDENTIALS" => Ok(MitreAttackPrimaryTechniquesEnum::ADDITIONALCLOUDCREDENTIALS),
           "SSH_AUTHORIZED_KEYS" => Ok(MitreAttackPrimaryTechniquesEnum::SSHAUTHORIZEDKEYS),
           "ADDITIONAL_CONTAINER_CLUSTER_ROLES" => Ok(MitreAttackPrimaryTechniquesEnum::ADDITIONALCONTAINERCLUSTERROLES),
           "INGRESS_TOOL_TRANSFER" => Ok(MitreAttackPrimaryTechniquesEnum::INGRESSTOOLTRANSFER),
           "NATIVE_API" => Ok(MitreAttackPrimaryTechniquesEnum::NATIVEAPI),
           "BRUTE_FORCE" => Ok(MitreAttackPrimaryTechniquesEnum::BRUTEFORCE),
           "SHARED_MODULES" => Ok(MitreAttackPrimaryTechniquesEnum::SHAREDMODULES),
           "ACCESS_TOKEN_MANIPULATION" => Ok(MitreAttackPrimaryTechniquesEnum::ACCESSTOKENMANIPULATION),
           "TOKEN_IMPERSONATION_OR_THEFT" => Ok(MitreAttackPrimaryTechniquesEnum::TOKENIMPERSONATIONORTHEFT),
           "EXPLOIT_PUBLIC_FACING_APPLICATION" => Ok(MitreAttackPrimaryTechniquesEnum::EXPLOITPUBLICFACINGAPPLICATION),
           "DOMAIN_POLICY_MODIFICATION" => Ok(MitreAttackPrimaryTechniquesEnum::DOMAINPOLICYMODIFICATION),
           "DATA_DESTRUCTION" => Ok(MitreAttackPrimaryTechniquesEnum::DATADESTRUCTION),
           "SERVICE_STOP" => Ok(MitreAttackPrimaryTechniquesEnum::SERVICESTOP),
           "INHIBIT_SYSTEM_RECOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::INHIBITSYSTEMRECOVERY),
           "RESOURCE_HIJACKING" => Ok(MitreAttackPrimaryTechniquesEnum::RESOURCEHIJACKING),
           "NETWORK_DENIAL_OF_SERVICE" => Ok(MitreAttackPrimaryTechniquesEnum::NETWORKDENIALOFSERVICE),
           "CLOUD_SERVICE_DISCOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::CLOUDSERVICEDISCOVERY),
           "STEAL_APPLICATION_ACCESS_TOKEN" => Ok(MitreAttackPrimaryTechniquesEnum::STEALAPPLICATIONACCESSTOKEN),
           "ACCOUNT_ACCESS_REMOVAL" => Ok(MitreAttackPrimaryTechniquesEnum::ACCOUNTACCESSREMOVAL),
           "STEAL_WEB_SESSION_COOKIE" => Ok(MitreAttackPrimaryTechniquesEnum::STEALWEBSESSIONCOOKIE),
           "CREATE_OR_MODIFY_SYSTEM_PROCESS" => Ok(MitreAttackPrimaryTechniquesEnum::CREATEORMODIFYSYSTEMPROCESS),
           "ABUSE_ELEVATION_CONTROL_MECHANISM" => Ok(MitreAttackPrimaryTechniquesEnum::ABUSEELEVATIONCONTROLMECHANISM),
           "UNSECURED_CREDENTIALS" => Ok(MitreAttackPrimaryTechniquesEnum::UNSECUREDCREDENTIALS),
           "MODIFY_AUTHENTICATION_PROCESS" => Ok(MitreAttackPrimaryTechniquesEnum::MODIFYAUTHENTICATIONPROCESS),
           "IMPAIR_DEFENSES" => Ok(MitreAttackPrimaryTechniquesEnum::IMPAIRDEFENSES),
           "DISABLE_OR_MODIFY_TOOLS" => Ok(MitreAttackPrimaryTechniquesEnum::DISABLEORMODIFYTOOLS),
           "EXFILTRATION_OVER_WEB_SERVICE" => Ok(MitreAttackPrimaryTechniquesEnum::EXFILTRATIONOVERWEBSERVICE),
           "EXFILTRATION_TO_CLOUD_STORAGE" => Ok(MitreAttackPrimaryTechniquesEnum::EXFILTRATIONTOCLOUDSTORAGE),
           "DYNAMIC_RESOLUTION" => Ok(MitreAttackPrimaryTechniquesEnum::DYNAMICRESOLUTION),
           "LATERAL_TOOL_TRANSFER" => Ok(MitreAttackPrimaryTechniquesEnum::LATERALTOOLTRANSFER),
           "MODIFY_CLOUD_COMPUTE_INFRASTRUCTURE" => Ok(MitreAttackPrimaryTechniquesEnum::MODIFYCLOUDCOMPUTEINFRASTRUCTURE),
           "CREATE_SNAPSHOT" => Ok(MitreAttackPrimaryTechniquesEnum::CREATESNAPSHOT),
           "CLOUD_INFRASTRUCTURE_DISCOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::CLOUDINFRASTRUCTUREDISCOVERY),
           "OBTAIN_CAPABILITIES" => Ok(MitreAttackPrimaryTechniquesEnum::OBTAINCAPABILITIES),
           "ACTIVE_SCANNING" => Ok(MitreAttackPrimaryTechniquesEnum::ACTIVESCANNING),
           "SCANNING_IP_BLOCKS" => Ok(MitreAttackPrimaryTechniquesEnum::SCANNINGIPBLOCKS),
           "CONTAINER_AND_RESOURCE_DISCOVERY" => Ok(MitreAttackPrimaryTechniquesEnum::CONTAINERANDRESOURCEDISCOVERY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MitreAttackPrimaryTechniquesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProcessSignatureSignatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes the type of resource associated with the signature.
pub enum ProcessSignatureSignatureTypeEnum {
    

    /// The default signature type.
    ///
    /// "SIGNATURE_TYPE_UNSPECIFIED"
    #[serde(rename="SIGNATURE_TYPE_UNSPECIFIED")]
    SIGNATURETYPEUNSPECIFIED,
    

    /// Used for signatures concerning processes.
    ///
    /// "SIGNATURE_TYPE_PROCESS"
    #[serde(rename="SIGNATURE_TYPE_PROCESS")]
    SIGNATURETYPEPROCESS,
    

    /// Used for signatures concerning disks.
    ///
    /// "SIGNATURE_TYPE_FILE"
    #[serde(rename="SIGNATURE_TYPE_FILE")]
    SIGNATURETYPEFILE,
}

impl AsRef<str> for ProcessSignatureSignatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProcessSignatureSignatureTypeEnum::SIGNATURETYPEUNSPECIFIED => "SIGNATURE_TYPE_UNSPECIFIED",
            ProcessSignatureSignatureTypeEnum::SIGNATURETYPEPROCESS => "SIGNATURE_TYPE_PROCESS",
            ProcessSignatureSignatureTypeEnum::SIGNATURETYPEFILE => "SIGNATURE_TYPE_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProcessSignatureSignatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SIGNATURE_TYPE_UNSPECIFIED" => Ok(ProcessSignatureSignatureTypeEnum::SIGNATURETYPEUNSPECIFIED),
           "SIGNATURE_TYPE_PROCESS" => Ok(ProcessSignatureSignatureTypeEnum::SIGNATURETYPEPROCESS),
           "SIGNATURE_TYPE_FILE" => Ok(ProcessSignatureSignatureTypeEnum::SIGNATURETYPEFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProcessSignatureSignatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceCloudProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates which cloud provider the finding is from.
pub enum ResourceCloudProviderEnum {
    

    /// The cloud provider is unspecified.
    ///
    /// "CLOUD_PROVIDER_UNSPECIFIED"
    #[serde(rename="CLOUD_PROVIDER_UNSPECIFIED")]
    CLOUDPROVIDERUNSPECIFIED,
    

    /// The cloud provider is Google Cloud Platform.
    ///
    /// "GOOGLE_CLOUD_PLATFORM"
    #[serde(rename="GOOGLE_CLOUD_PLATFORM")]
    GOOGLECLOUDPLATFORM,
    

    /// The cloud provider is Amazon Web Services.
    ///
    /// "AMAZON_WEB_SERVICES"
    #[serde(rename="AMAZON_WEB_SERVICES")]
    AMAZONWEBSERVICES,
    

    /// The cloud provider is Microsoft Azure.
    ///
    /// "MICROSOFT_AZURE"
    #[serde(rename="MICROSOFT_AZURE")]
    MICROSOFTAZURE,
}

impl AsRef<str> for ResourceCloudProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceCloudProviderEnum::CLOUDPROVIDERUNSPECIFIED => "CLOUD_PROVIDER_UNSPECIFIED",
            ResourceCloudProviderEnum::GOOGLECLOUDPLATFORM => "GOOGLE_CLOUD_PLATFORM",
            ResourceCloudProviderEnum::AMAZONWEBSERVICES => "AMAZON_WEB_SERVICES",
            ResourceCloudProviderEnum::MICROSOFTAZURE => "MICROSOFT_AZURE",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceCloudProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLOUD_PROVIDER_UNSPECIFIED" => Ok(ResourceCloudProviderEnum::CLOUDPROVIDERUNSPECIFIED),
           "GOOGLE_CLOUD_PLATFORM" => Ok(ResourceCloudProviderEnum::GOOGLECLOUDPLATFORM),
           "AMAZON_WEB_SERVICES" => Ok(ResourceCloudProviderEnum::AMAZONWEBSERVICES),
           "MICROSOFT_AZURE" => Ok(ResourceCloudProviderEnum::MICROSOFTAZURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceCloudProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourcePathNodeNodeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of resource this node represents.
pub enum ResourcePathNodeNodeTypeEnum {
    

    /// Node type is unspecified.
    ///
    /// "RESOURCE_PATH_NODE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_PATH_NODE_TYPE_UNSPECIFIED")]
    RESOURCEPATHNODETYPEUNSPECIFIED,
    

    /// The node represents a GCP organization.
    ///
    /// "GCP_ORGANIZATION"
    #[serde(rename="GCP_ORGANIZATION")]
    GCPORGANIZATION,
    

    /// The node represents a GCP folder.
    ///
    /// "GCP_FOLDER"
    #[serde(rename="GCP_FOLDER")]
    GCPFOLDER,
    

    /// The node represents a GCP project.
    ///
    /// "GCP_PROJECT"
    #[serde(rename="GCP_PROJECT")]
    GCPPROJECT,
    

    /// The node represents an AWS organization.
    ///
    /// "AWS_ORGANIZATION"
    #[serde(rename="AWS_ORGANIZATION")]
    AWSORGANIZATION,
    

    /// The node represents an AWS organizational unit.
    ///
    /// "AWS_ORGANIZATIONAL_UNIT"
    #[serde(rename="AWS_ORGANIZATIONAL_UNIT")]
    AWSORGANIZATIONALUNIT,
    

    /// The node represents an AWS account.
    ///
    /// "AWS_ACCOUNT"
    #[serde(rename="AWS_ACCOUNT")]
    AWSACCOUNT,
    

    /// The node represents an Azure management group.
    ///
    /// "AZURE_MANAGEMENT_GROUP"
    #[serde(rename="AZURE_MANAGEMENT_GROUP")]
    AZUREMANAGEMENTGROUP,
    

    /// The node represents an Azure subscription.
    ///
    /// "AZURE_SUBSCRIPTION"
    #[serde(rename="AZURE_SUBSCRIPTION")]
    AZURESUBSCRIPTION,
    

    /// The node represents an Azure resource group.
    ///
    /// "AZURE_RESOURCE_GROUP"
    #[serde(rename="AZURE_RESOURCE_GROUP")]
    AZURERESOURCEGROUP,
}

impl AsRef<str> for ResourcePathNodeNodeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourcePathNodeNodeTypeEnum::RESOURCEPATHNODETYPEUNSPECIFIED => "RESOURCE_PATH_NODE_TYPE_UNSPECIFIED",
            ResourcePathNodeNodeTypeEnum::GCPORGANIZATION => "GCP_ORGANIZATION",
            ResourcePathNodeNodeTypeEnum::GCPFOLDER => "GCP_FOLDER",
            ResourcePathNodeNodeTypeEnum::GCPPROJECT => "GCP_PROJECT",
            ResourcePathNodeNodeTypeEnum::AWSORGANIZATION => "AWS_ORGANIZATION",
            ResourcePathNodeNodeTypeEnum::AWSORGANIZATIONALUNIT => "AWS_ORGANIZATIONAL_UNIT",
            ResourcePathNodeNodeTypeEnum::AWSACCOUNT => "AWS_ACCOUNT",
            ResourcePathNodeNodeTypeEnum::AZUREMANAGEMENTGROUP => "AZURE_MANAGEMENT_GROUP",
            ResourcePathNodeNodeTypeEnum::AZURESUBSCRIPTION => "AZURE_SUBSCRIPTION",
            ResourcePathNodeNodeTypeEnum::AZURERESOURCEGROUP => "AZURE_RESOURCE_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourcePathNodeNodeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_PATH_NODE_TYPE_UNSPECIFIED" => Ok(ResourcePathNodeNodeTypeEnum::RESOURCEPATHNODETYPEUNSPECIFIED),
           "GCP_ORGANIZATION" => Ok(ResourcePathNodeNodeTypeEnum::GCPORGANIZATION),
           "GCP_FOLDER" => Ok(ResourcePathNodeNodeTypeEnum::GCPFOLDER),
           "GCP_PROJECT" => Ok(ResourcePathNodeNodeTypeEnum::GCPPROJECT),
           "AWS_ORGANIZATION" => Ok(ResourcePathNodeNodeTypeEnum::AWSORGANIZATION),
           "AWS_ORGANIZATIONAL_UNIT" => Ok(ResourcePathNodeNodeTypeEnum::AWSORGANIZATIONALUNIT),
           "AWS_ACCOUNT" => Ok(ResourcePathNodeNodeTypeEnum::AWSACCOUNT),
           "AZURE_MANAGEMENT_GROUP" => Ok(ResourcePathNodeNodeTypeEnum::AZUREMANAGEMENTGROUP),
           "AZURE_SUBSCRIPTION" => Ok(ResourcePathNodeNodeTypeEnum::AZURESUBSCRIPTION),
           "AZURE_RESOURCE_GROUP" => Ok(ResourcePathNodeNodeTypeEnum::AZURERESOURCEGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourcePathNodeNodeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RoleKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Role type.
pub enum RoleKindEnum {
    

    /// Role type is not specified.
    ///
    /// "KIND_UNSPECIFIED"
    #[serde(rename="KIND_UNSPECIFIED")]
    KINDUNSPECIFIED,
    

    /// Kubernetes Role.
    ///
    /// "ROLE"
    #[serde(rename="ROLE")]
    ROLE,
    

    /// Kubernetes ClusterRole.
    ///
    /// "CLUSTER_ROLE"
    #[serde(rename="CLUSTER_ROLE")]
    CLUSTERROLE,
}

impl AsRef<str> for RoleKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RoleKindEnum::KINDUNSPECIFIED => "KIND_UNSPECIFIED",
            RoleKindEnum::ROLE => "ROLE",
            RoleKindEnum::CLUSTERROLE => "CLUSTER_ROLE",
        }
    }
}

impl std::convert::TryFrom< &str> for RoleKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KIND_UNSPECIFIED" => Ok(RoleKindEnum::KINDUNSPECIFIED),
           "ROLE" => Ok(RoleKindEnum::ROLE),
           "CLUSTER_ROLE" => Ok(RoleKindEnum::CLUSTERROLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RoleKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SetFindingStateRequestStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The desired State of the finding.
pub enum SetFindingStateRequestStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The finding requires attention and has not been addressed yet.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The finding has been fixed, triaged as a non-issue or otherwise addressed and is no longer active.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for SetFindingStateRequestStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SetFindingStateRequestStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SetFindingStateRequestStateEnum::ACTIVE => "ACTIVE",
            SetFindingStateRequestStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for SetFindingStateRequestStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SetFindingStateRequestStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(SetFindingStateRequestStateEnum::ACTIVE),
           "INACTIVE" => Ok(SetFindingStateRequestStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SetFindingStateRequestStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SetMuteRequestMuteEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The desired state of the Mute.
pub enum SetMuteRequestMuteEnum {
    

    /// Unspecified.
    ///
    /// "MUTE_UNSPECIFIED"
    #[serde(rename="MUTE_UNSPECIFIED")]
    MUTEUNSPECIFIED,
    

    /// Finding has been muted.
    ///
    /// "MUTED"
    #[serde(rename="MUTED")]
    MUTED,
    

    /// Finding has been unmuted.
    ///
    /// "UNMUTED"
    #[serde(rename="UNMUTED")]
    UNMUTED,
    

    /// Finding has never been muted/unmuted.
    ///
    /// "UNDEFINED"
    #[serde(rename="UNDEFINED")]
    UNDEFINED,
}

impl AsRef<str> for SetMuteRequestMuteEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SetMuteRequestMuteEnum::MUTEUNSPECIFIED => "MUTE_UNSPECIFIED",
            SetMuteRequestMuteEnum::MUTED => "MUTED",
            SetMuteRequestMuteEnum::UNMUTED => "UNMUTED",
            SetMuteRequestMuteEnum::UNDEFINED => "UNDEFINED",
        }
    }
}

impl std::convert::TryFrom< &str> for SetMuteRequestMuteEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MUTE_UNSPECIFIED" => Ok(SetMuteRequestMuteEnum::MUTEUNSPECIFIED),
           "MUTED" => Ok(SetMuteRequestMuteEnum::MUTED),
           "UNMUTED" => Ok(SetMuteRequestMuteEnum::UNMUTED),
           "UNDEFINED" => Ok(SetMuteRequestMuteEnum::UNDEFINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SetMuteRequestMuteEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SimulationCloudProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates which cloud provider was used in this simulation.
pub enum SimulationCloudProviderEnum {
    

    /// The cloud provider is unspecified.
    ///
    /// "CLOUD_PROVIDER_UNSPECIFIED"
    #[serde(rename="CLOUD_PROVIDER_UNSPECIFIED")]
    CLOUDPROVIDERUNSPECIFIED,
    

    /// The cloud provider is Google Cloud Platform.
    ///
    /// "GOOGLE_CLOUD_PLATFORM"
    #[serde(rename="GOOGLE_CLOUD_PLATFORM")]
    GOOGLECLOUDPLATFORM,
    

    /// The cloud provider is Amazon Web Services.
    ///
    /// "AMAZON_WEB_SERVICES"
    #[serde(rename="AMAZON_WEB_SERVICES")]
    AMAZONWEBSERVICES,
    

    /// The cloud provider is Microsoft Azure.
    ///
    /// "MICROSOFT_AZURE"
    #[serde(rename="MICROSOFT_AZURE")]
    MICROSOFTAZURE,
}

impl AsRef<str> for SimulationCloudProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SimulationCloudProviderEnum::CLOUDPROVIDERUNSPECIFIED => "CLOUD_PROVIDER_UNSPECIFIED",
            SimulationCloudProviderEnum::GOOGLECLOUDPLATFORM => "GOOGLE_CLOUD_PLATFORM",
            SimulationCloudProviderEnum::AMAZONWEBSERVICES => "AMAZON_WEB_SERVICES",
            SimulationCloudProviderEnum::MICROSOFTAZURE => "MICROSOFT_AZURE",
        }
    }
}

impl std::convert::TryFrom< &str> for SimulationCloudProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLOUD_PROVIDER_UNSPECIFIED" => Ok(SimulationCloudProviderEnum::CLOUDPROVIDERUNSPECIFIED),
           "GOOGLE_CLOUD_PLATFORM" => Ok(SimulationCloudProviderEnum::GOOGLECLOUDPLATFORM),
           "AMAZON_WEB_SERVICES" => Ok(SimulationCloudProviderEnum::AMAZONWEBSERVICES),
           "MICROSOFT_AZURE" => Ok(SimulationCloudProviderEnum::MICROSOFTAZURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SimulationCloudProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubjectKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Authentication type for the subject.
pub enum SubjectKindEnum {
    

    /// Authentication is not specified.
    ///
    /// "AUTH_TYPE_UNSPECIFIED"
    #[serde(rename="AUTH_TYPE_UNSPECIFIED")]
    AUTHTYPEUNSPECIFIED,
    

    /// User with valid certificate.
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
    

    /// Users managed by Kubernetes API with credentials stored as secrets.
    ///
    /// "SERVICEACCOUNT"
    #[serde(rename="SERVICEACCOUNT")]
    SERVICEACCOUNT,
    

    /// Collection of users.
    ///
    /// "GROUP"
    #[serde(rename="GROUP")]
    GROUP,
}

impl AsRef<str> for SubjectKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubjectKindEnum::AUTHTYPEUNSPECIFIED => "AUTH_TYPE_UNSPECIFIED",
            SubjectKindEnum::USER => "USER",
            SubjectKindEnum::SERVICEACCOUNT => "SERVICEACCOUNT",
            SubjectKindEnum::GROUP => "GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for SubjectKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_TYPE_UNSPECIFIED" => Ok(SubjectKindEnum::AUTHTYPEUNSPECIFIED),
           "USER" => Ok(SubjectKindEnum::USER),
           "SERVICEACCOUNT" => Ok(SubjectKindEnum::SERVICEACCOUNT),
           "GROUP" => Ok(SubjectKindEnum::GROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubjectKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValuedResourceResourceValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How valuable this resource is.
pub enum ValuedResourceResourceValueEnum {
    

    /// The resource value isn't specified.
    ///
    /// "RESOURCE_VALUE_UNSPECIFIED"
    #[serde(rename="RESOURCE_VALUE_UNSPECIFIED")]
    RESOURCEVALUEUNSPECIFIED,
    

    /// This is a low-value resource.
    ///
    /// "RESOURCE_VALUE_LOW"
    #[serde(rename="RESOURCE_VALUE_LOW")]
    RESOURCEVALUELOW,
    

    /// This is a medium-value resource.
    ///
    /// "RESOURCE_VALUE_MEDIUM"
    #[serde(rename="RESOURCE_VALUE_MEDIUM")]
    RESOURCEVALUEMEDIUM,
    

    /// This is a high-value resource.
    ///
    /// "RESOURCE_VALUE_HIGH"
    #[serde(rename="RESOURCE_VALUE_HIGH")]
    RESOURCEVALUEHIGH,
}

impl AsRef<str> for ValuedResourceResourceValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValuedResourceResourceValueEnum::RESOURCEVALUEUNSPECIFIED => "RESOURCE_VALUE_UNSPECIFIED",
            ValuedResourceResourceValueEnum::RESOURCEVALUELOW => "RESOURCE_VALUE_LOW",
            ValuedResourceResourceValueEnum::RESOURCEVALUEMEDIUM => "RESOURCE_VALUE_MEDIUM",
            ValuedResourceResourceValueEnum::RESOURCEVALUEHIGH => "RESOURCE_VALUE_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for ValuedResourceResourceValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_VALUE_UNSPECIFIED" => Ok(ValuedResourceResourceValueEnum::RESOURCEVALUEUNSPECIFIED),
           "RESOURCE_VALUE_LOW" => Ok(ValuedResourceResourceValueEnum::RESOURCEVALUELOW),
           "RESOURCE_VALUE_MEDIUM" => Ok(ValuedResourceResourceValueEnum::RESOURCEVALUEMEDIUM),
           "RESOURCE_VALUE_HIGH" => Ok(ValuedResourceResourceValueEnum::RESOURCEVALUEHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValuedResourceResourceValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


