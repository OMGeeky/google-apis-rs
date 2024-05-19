use super::*;



// region AddressGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the Address Group. Possible values are "IPv4" or "IPV6".
pub enum AddressGroupTypeEnum {
    

    /// Default value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// IP v4 ranges.
    ///
    /// "IPV4"
    #[serde(rename="IPV4")]
    IPV4,
    

    /// IP v6 ranges.
    ///
    /// "IPV6"
    #[serde(rename="IPV6")]
    IPV6,
}

impl AsRef<str> for AddressGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AddressGroupTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AddressGroupTypeEnum::IPV4 => "IPV4",
            AddressGroupTypeEnum::IPV6 => "IPV6",
        }
    }
}

impl std::convert::TryFrom< &str> for AddressGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AddressGroupTypeEnum::TYPEUNSPECIFIED),
           "IPV4" => Ok(AddressGroupTypeEnum::IPV4),
           "IPV6" => Ok(AddressGroupTypeEnum::IPV6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AddressGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorizationPolicyActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The action to take when a rule match is found. Possible values are "ALLOW" or "DENY".
pub enum AuthorizationPolicyActionEnum {
    

    /// Default value.
    ///
    /// "ACTION_UNSPECIFIED"
    #[serde(rename="ACTION_UNSPECIFIED")]
    ACTIONUNSPECIFIED,
    

    /// Grant access.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Deny access. Deny rules should be avoided unless they are used to provide a default "deny all" fallback.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for AuthorizationPolicyActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizationPolicyActionEnum::ACTIONUNSPECIFIED => "ACTION_UNSPECIFIED",
            AuthorizationPolicyActionEnum::ALLOW => "ALLOW",
            AuthorizationPolicyActionEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizationPolicyActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNSPECIFIED" => Ok(AuthorizationPolicyActionEnum::ACTIONUNSPECIFIED),
           "ALLOW" => Ok(AuthorizationPolicyActionEnum::ALLOW),
           "DENY" => Ok(AuthorizationPolicyActionEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizationPolicyActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirewallEndpointStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the endpoint.
pub enum FirewallEndpointStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Processing configuration updates.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Down or in an error state.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for FirewallEndpointStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirewallEndpointStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FirewallEndpointStateEnum::CREATING => "CREATING",
            FirewallEndpointStateEnum::ACTIVE => "ACTIVE",
            FirewallEndpointStateEnum::DELETING => "DELETING",
            FirewallEndpointStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for FirewallEndpointStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FirewallEndpointStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(FirewallEndpointStateEnum::CREATING),
           "ACTIVE" => Ok(FirewallEndpointStateEnum::ACTIVE),
           "DELETING" => Ok(FirewallEndpointStateEnum::DELETING),
           "INACTIVE" => Ok(FirewallEndpointStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirewallEndpointStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirewallEndpointAssociationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the association.
pub enum FirewallEndpointAssociationStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Active and ready for traffic.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Down or in an error state.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for FirewallEndpointAssociationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirewallEndpointAssociationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FirewallEndpointAssociationStateEnum::CREATING => "CREATING",
            FirewallEndpointAssociationStateEnum::ACTIVE => "ACTIVE",
            FirewallEndpointAssociationStateEnum::DELETING => "DELETING",
            FirewallEndpointAssociationStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for FirewallEndpointAssociationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FirewallEndpointAssociationStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(FirewallEndpointAssociationStateEnum::CREATING),
           "ACTIVE" => Ok(FirewallEndpointAssociationStateEnum::ACTIVE),
           "DELETING" => Ok(FirewallEndpointAssociationStateEnum::DELETING),
           "INACTIVE" => Ok(FirewallEndpointAssociationStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirewallEndpointAssociationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewaySecurityPolicyRuleBasicProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Profile which tells what the primitive action should be.
pub enum GatewaySecurityPolicyRuleBasicProfileEnum {
    

    /// If there is not a mentioned action for the target.
    ///
    /// "BASIC_PROFILE_UNSPECIFIED"
    #[serde(rename="BASIC_PROFILE_UNSPECIFIED")]
    BASICPROFILEUNSPECIFIED,
    

    /// Allow the matched traffic.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Deny the matched traffic.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GatewaySecurityPolicyRuleBasicProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewaySecurityPolicyRuleBasicProfileEnum::BASICPROFILEUNSPECIFIED => "BASIC_PROFILE_UNSPECIFIED",
            GatewaySecurityPolicyRuleBasicProfileEnum::ALLOW => "ALLOW",
            GatewaySecurityPolicyRuleBasicProfileEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewaySecurityPolicyRuleBasicProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC_PROFILE_UNSPECIFIED" => Ok(GatewaySecurityPolicyRuleBasicProfileEnum::BASICPROFILEUNSPECIFIED),
           "ALLOW" => Ok(GatewaySecurityPolicyRuleBasicProfileEnum::ALLOW),
           "DENY" => Ok(GatewaySecurityPolicyRuleBasicProfileEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewaySecurityPolicyRuleBasicProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamV1AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
    

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

impl AsRef<str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamV1AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MTLSPolicyClientValidationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When the client presents an invalid certificate or no certificate to the load balancer, the `client_validation_mode` specifies how the client connection is handled. Required if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty.
pub enum MTLSPolicyClientValidationModeEnum {
    

    /// Not allowed.
    ///
    /// "CLIENT_VALIDATION_MODE_UNSPECIFIED"
    #[serde(rename="CLIENT_VALIDATION_MODE_UNSPECIFIED")]
    CLIENTVALIDATIONMODEUNSPECIFIED,
    

    /// Allow connection even if certificate chain validation of the client certificate failed or no client certificate was presented. The proof of possession of the private key is always checked if client certificate was presented. This mode requires the backend to implement processing of data extracted from a client certificate to authenticate the peer, or to reject connections if the client certificate fingerprint is missing.
    ///
    /// "ALLOW_INVALID_OR_MISSING_CLIENT_CERT"
    #[serde(rename="ALLOW_INVALID_OR_MISSING_CLIENT_CERT")]
    ALLOWINVALIDORMISSINGCLIENTCERT,
    

    /// Require a client certificate and allow connection to the backend only if validation of the client certificate passed. If set, requires a reference to non-empty TrustConfig specified in `client_validation_trust_config`.
    ///
    /// "REJECT_INVALID"
    #[serde(rename="REJECT_INVALID")]
    REJECTINVALID,
}

impl AsRef<str> for MTLSPolicyClientValidationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MTLSPolicyClientValidationModeEnum::CLIENTVALIDATIONMODEUNSPECIFIED => "CLIENT_VALIDATION_MODE_UNSPECIFIED",
            MTLSPolicyClientValidationModeEnum::ALLOWINVALIDORMISSINGCLIENTCERT => "ALLOW_INVALID_OR_MISSING_CLIENT_CERT",
            MTLSPolicyClientValidationModeEnum::REJECTINVALID => "REJECT_INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for MTLSPolicyClientValidationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLIENT_VALIDATION_MODE_UNSPECIFIED" => Ok(MTLSPolicyClientValidationModeEnum::CLIENTVALIDATIONMODEUNSPECIFIED),
           "ALLOW_INVALID_OR_MISSING_CLIENT_CERT" => Ok(MTLSPolicyClientValidationModeEnum::ALLOWINVALIDORMISSINGCLIENTCERT),
           "REJECT_INVALID" => Ok(MTLSPolicyClientValidationModeEnum::REJECTINVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MTLSPolicyClientValidationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SecurityProfileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The single ProfileType that the SecurityProfile resource configures.
pub enum SecurityProfileTypeEnum {
    

    /// Profile type not specified.
    ///
    /// "PROFILE_TYPE_UNSPECIFIED"
    #[serde(rename="PROFILE_TYPE_UNSPECIFIED")]
    PROFILETYPEUNSPECIFIED,
    

    /// Profile type for threat prevention.
    ///
    /// "THREAT_PREVENTION"
    #[serde(rename="THREAT_PREVENTION")]
    THREATPREVENTION,
}

impl AsRef<str> for SecurityProfileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecurityProfileTypeEnum::PROFILETYPEUNSPECIFIED => "PROFILE_TYPE_UNSPECIFIED",
            SecurityProfileTypeEnum::THREATPREVENTION => "THREAT_PREVENTION",
        }
    }
}

impl std::convert::TryFrom< &str> for SecurityProfileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_TYPE_UNSPECIFIED" => Ok(SecurityProfileTypeEnum::PROFILETYPEUNSPECIFIED),
           "THREAT_PREVENTION" => Ok(SecurityProfileTypeEnum::THREATPREVENTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecurityProfileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SeverityOverrideActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Threat action override.
pub enum SeverityOverrideActionEnum {
    

    /// Threat action not specified.
    ///
    /// "THREAT_ACTION_UNSPECIFIED"
    #[serde(rename="THREAT_ACTION_UNSPECIFIED")]
    THREATACTIONUNSPECIFIED,
    

    /// The default action (as specified by the vendor) is taken.
    ///
    /// "DEFAULT_ACTION"
    #[serde(rename="DEFAULT_ACTION")]
    DEFAULTACTION,
    

    /// The packet matching this rule will be allowed to transmit.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// The packet matching this rule will be allowed to transmit, but a threat_log entry will be sent to the consumer project.
    ///
    /// "ALERT"
    #[serde(rename="ALERT")]
    ALERT,
    

    /// The packet matching this rule will be dropped, and a threat_log entry will be sent to the consumer project.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for SeverityOverrideActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SeverityOverrideActionEnum::THREATACTIONUNSPECIFIED => "THREAT_ACTION_UNSPECIFIED",
            SeverityOverrideActionEnum::DEFAULTACTION => "DEFAULT_ACTION",
            SeverityOverrideActionEnum::ALLOW => "ALLOW",
            SeverityOverrideActionEnum::ALERT => "ALERT",
            SeverityOverrideActionEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for SeverityOverrideActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ACTION_UNSPECIFIED" => Ok(SeverityOverrideActionEnum::THREATACTIONUNSPECIFIED),
           "DEFAULT_ACTION" => Ok(SeverityOverrideActionEnum::DEFAULTACTION),
           "ALLOW" => Ok(SeverityOverrideActionEnum::ALLOW),
           "ALERT" => Ok(SeverityOverrideActionEnum::ALERT),
           "DENY" => Ok(SeverityOverrideActionEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SeverityOverrideActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SeverityOverrideSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Severity level to match.
pub enum SeverityOverrideSeverityEnum {
    

    /// Severity level not specified.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Suspicious events that do not pose an immediate threat, but that are reported to call attention to deeper problems that could possibly exist.
    ///
    /// "INFORMATIONAL"
    #[serde(rename="INFORMATIONAL")]
    INFORMATIONAL,
    

    /// Warning-level threats that have very little impact on an organization's infrastructure. They usually require local or physical system access and may often result in victim privacy issues and information leakage.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Minor threats in which impact is minimized, that do not compromise the target or exploits that require an attacker to reside on the same local network as the victim, affect only non-standard configurations or obscure applications, or provide very limited access.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Threats that have the ability to become critical but have mitigating factors; for example, they may be difficult to exploit, do not result in elevated privileges, or do not have a large victim pool.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Serious threats, such as those that affect default installations of widely deployed software, result in root compromise of servers, and the exploit code is widely available to attackers. The attacker usually does not need any special authentication credentials or knowledge about the individual victims and the target does not need to be manipulated into performing any special functions.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for SeverityOverrideSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SeverityOverrideSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            SeverityOverrideSeverityEnum::INFORMATIONAL => "INFORMATIONAL",
            SeverityOverrideSeverityEnum::LOW => "LOW",
            SeverityOverrideSeverityEnum::MEDIUM => "MEDIUM",
            SeverityOverrideSeverityEnum::HIGH => "HIGH",
            SeverityOverrideSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SeverityOverrideSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(SeverityOverrideSeverityEnum::SEVERITYUNSPECIFIED),
           "INFORMATIONAL" => Ok(SeverityOverrideSeverityEnum::INFORMATIONAL),
           "LOW" => Ok(SeverityOverrideSeverityEnum::LOW),
           "MEDIUM" => Ok(SeverityOverrideSeverityEnum::MEDIUM),
           "HIGH" => Ok(SeverityOverrideSeverityEnum::HIGH),
           "CRITICAL" => Ok(SeverityOverrideSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SeverityOverrideSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThreatOverrideActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Threat action override. For some threat types, only a subset of actions applies.
pub enum ThreatOverrideActionEnum {
    

    /// Threat action not specified.
    ///
    /// "THREAT_ACTION_UNSPECIFIED"
    #[serde(rename="THREAT_ACTION_UNSPECIFIED")]
    THREATACTIONUNSPECIFIED,
    

    /// The default action (as specified by the vendor) is taken.
    ///
    /// "DEFAULT_ACTION"
    #[serde(rename="DEFAULT_ACTION")]
    DEFAULTACTION,
    

    /// The packet matching this rule will be allowed to transmit.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// The packet matching this rule will be allowed to transmit, but a threat_log entry will be sent to the consumer project.
    ///
    /// "ALERT"
    #[serde(rename="ALERT")]
    ALERT,
    

    /// The packet matching this rule will be dropped, and a threat_log entry will be sent to the consumer project.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for ThreatOverrideActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThreatOverrideActionEnum::THREATACTIONUNSPECIFIED => "THREAT_ACTION_UNSPECIFIED",
            ThreatOverrideActionEnum::DEFAULTACTION => "DEFAULT_ACTION",
            ThreatOverrideActionEnum::ALLOW => "ALLOW",
            ThreatOverrideActionEnum::ALERT => "ALERT",
            ThreatOverrideActionEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for ThreatOverrideActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_ACTION_UNSPECIFIED" => Ok(ThreatOverrideActionEnum::THREATACTIONUNSPECIFIED),
           "DEFAULT_ACTION" => Ok(ThreatOverrideActionEnum::DEFAULTACTION),
           "ALLOW" => Ok(ThreatOverrideActionEnum::ALLOW),
           "ALERT" => Ok(ThreatOverrideActionEnum::ALERT),
           "DENY" => Ok(ThreatOverrideActionEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThreatOverrideActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThreatOverrideTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of the threat (read only).
pub enum ThreatOverrideTypeEnum {
    

    /// Type of threat not specified.
    ///
    /// "THREAT_TYPE_UNSPECIFIED"
    #[serde(rename="THREAT_TYPE_UNSPECIFIED")]
    THREATTYPEUNSPECIFIED,
    

    /// Type of threat is not derivable from threat ID. An override will be created for all types. Firewall will ignore overridden signature ID's that don't exist in the specific type.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Threats related to system flaws that an attacker might otherwise attempt to exploit.
    ///
    /// "VULNERABILITY"
    #[serde(rename="VULNERABILITY")]
    VULNERABILITY,
    

    /// Threats related to viruses and malware found in executables and file types.
    ///
    /// "ANTIVIRUS"
    #[serde(rename="ANTIVIRUS")]
    ANTIVIRUS,
    

    /// Threats related to command-and-control (C2) activity, where spyware on an infected client is collecting data without the user's consent and/or communicating with a remote attacker.
    ///
    /// "SPYWARE"
    #[serde(rename="SPYWARE")]
    SPYWARE,
    

    /// Threats related to DNS.
    ///
    /// "DNS"
    #[serde(rename="DNS")]
    DNS,
}

impl AsRef<str> for ThreatOverrideTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThreatOverrideTypeEnum::THREATTYPEUNSPECIFIED => "THREAT_TYPE_UNSPECIFIED",
            ThreatOverrideTypeEnum::UNKNOWN => "UNKNOWN",
            ThreatOverrideTypeEnum::VULNERABILITY => "VULNERABILITY",
            ThreatOverrideTypeEnum::ANTIVIRUS => "ANTIVIRUS",
            ThreatOverrideTypeEnum::SPYWARE => "SPYWARE",
            ThreatOverrideTypeEnum::DNS => "DNS",
        }
    }
}

impl std::convert::TryFrom< &str> for ThreatOverrideTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THREAT_TYPE_UNSPECIFIED" => Ok(ThreatOverrideTypeEnum::THREATTYPEUNSPECIFIED),
           "UNKNOWN" => Ok(ThreatOverrideTypeEnum::UNKNOWN),
           "VULNERABILITY" => Ok(ThreatOverrideTypeEnum::VULNERABILITY),
           "ANTIVIRUS" => Ok(ThreatOverrideTypeEnum::ANTIVIRUS),
           "SPYWARE" => Ok(ThreatOverrideTypeEnum::SPYWARE),
           "DNS" => Ok(ThreatOverrideTypeEnum::DNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThreatOverrideTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TlsInspectionPolicyMinTlsVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
pub enum TlsInspectionPolicyMinTlsVersionEnum {
    

    /// Indicates no TLS version was specified.
    ///
    /// "TLS_VERSION_UNSPECIFIED"
    #[serde(rename="TLS_VERSION_UNSPECIFIED")]
    TLSVERSIONUNSPECIFIED,
    

    /// TLS 1.0
    ///
    /// "TLS_1_0"
    #[serde(rename="TLS_1_0")]
    TLS10,
    

    /// TLS 1.1
    ///
    /// "TLS_1_1"
    #[serde(rename="TLS_1_1")]
    TLS11,
    

    /// TLS 1.2
    ///
    /// "TLS_1_2"
    #[serde(rename="TLS_1_2")]
    TLS12,
    

    /// TLS 1.3
    ///
    /// "TLS_1_3"
    #[serde(rename="TLS_1_3")]
    TLS13,
}

impl AsRef<str> for TlsInspectionPolicyMinTlsVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TlsInspectionPolicyMinTlsVersionEnum::TLSVERSIONUNSPECIFIED => "TLS_VERSION_UNSPECIFIED",
            TlsInspectionPolicyMinTlsVersionEnum::TLS10 => "TLS_1_0",
            TlsInspectionPolicyMinTlsVersionEnum::TLS11 => "TLS_1_1",
            TlsInspectionPolicyMinTlsVersionEnum::TLS12 => "TLS_1_2",
            TlsInspectionPolicyMinTlsVersionEnum::TLS13 => "TLS_1_3",
        }
    }
}

impl std::convert::TryFrom< &str> for TlsInspectionPolicyMinTlsVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TLS_VERSION_UNSPECIFIED" => Ok(TlsInspectionPolicyMinTlsVersionEnum::TLSVERSIONUNSPECIFIED),
           "TLS_1_0" => Ok(TlsInspectionPolicyMinTlsVersionEnum::TLS10),
           "TLS_1_1" => Ok(TlsInspectionPolicyMinTlsVersionEnum::TLS11),
           "TLS_1_2" => Ok(TlsInspectionPolicyMinTlsVersionEnum::TLS12),
           "TLS_1_3" => Ok(TlsInspectionPolicyMinTlsVersionEnum::TLS13),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TlsInspectionPolicyMinTlsVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TlsInspectionPolicyTlsFeatureProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers ("PROFILE_COMPATIBLE"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
pub enum TlsInspectionPolicyTlsFeatureProfileEnum {
    

    /// Indicates no profile was specified.
    ///
    /// "PROFILE_UNSPECIFIED"
    #[serde(rename="PROFILE_UNSPECIFIED")]
    PROFILEUNSPECIFIED,
    

    /// Compatible profile. Allows the broadest set of clients, even those which support only out-of-date SSL features to negotiate with the TLS inspection proxy.
    ///
    /// "PROFILE_COMPATIBLE"
    #[serde(rename="PROFILE_COMPATIBLE")]
    PROFILECOMPATIBLE,
    

    /// Modern profile. Supports a wide set of SSL features, allowing modern clients to negotiate SSL with the TLS inspection proxy.
    ///
    /// "PROFILE_MODERN"
    #[serde(rename="PROFILE_MODERN")]
    PROFILEMODERN,
    

    /// Restricted profile. Supports a reduced set of SSL features, intended to meet stricter compliance requirements.
    ///
    /// "PROFILE_RESTRICTED"
    #[serde(rename="PROFILE_RESTRICTED")]
    PROFILERESTRICTED,
    

    /// Custom profile. Allow only the set of allowed SSL features specified in the custom_features field of SslPolicy.
    ///
    /// "PROFILE_CUSTOM"
    #[serde(rename="PROFILE_CUSTOM")]
    PROFILECUSTOM,
}

impl AsRef<str> for TlsInspectionPolicyTlsFeatureProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TlsInspectionPolicyTlsFeatureProfileEnum::PROFILEUNSPECIFIED => "PROFILE_UNSPECIFIED",
            TlsInspectionPolicyTlsFeatureProfileEnum::PROFILECOMPATIBLE => "PROFILE_COMPATIBLE",
            TlsInspectionPolicyTlsFeatureProfileEnum::PROFILEMODERN => "PROFILE_MODERN",
            TlsInspectionPolicyTlsFeatureProfileEnum::PROFILERESTRICTED => "PROFILE_RESTRICTED",
            TlsInspectionPolicyTlsFeatureProfileEnum::PROFILECUSTOM => "PROFILE_CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for TlsInspectionPolicyTlsFeatureProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_UNSPECIFIED" => Ok(TlsInspectionPolicyTlsFeatureProfileEnum::PROFILEUNSPECIFIED),
           "PROFILE_COMPATIBLE" => Ok(TlsInspectionPolicyTlsFeatureProfileEnum::PROFILECOMPATIBLE),
           "PROFILE_MODERN" => Ok(TlsInspectionPolicyTlsFeatureProfileEnum::PROFILEMODERN),
           "PROFILE_RESTRICTED" => Ok(TlsInspectionPolicyTlsFeatureProfileEnum::PROFILERESTRICTED),
           "PROFILE_CUSTOM" => Ok(TlsInspectionPolicyTlsFeatureProfileEnum::PROFILECUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TlsInspectionPolicyTlsFeatureProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


