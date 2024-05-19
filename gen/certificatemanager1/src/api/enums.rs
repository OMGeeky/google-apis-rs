use super::*;



// region AuthorizationAttemptInfoFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Reason for failure of the authorization attempt for the domain.
pub enum AuthorizationAttemptInfoFailureReasonEnum {
    

    /// FailureReason is unspecified.
    ///
    /// "FAILURE_REASON_UNSPECIFIED"
    #[serde(rename="FAILURE_REASON_UNSPECIFIED")]
    FAILUREREASONUNSPECIFIED,
    

    /// There was a problem with the user's DNS or load balancer configuration for this domain.
    ///
    /// "CONFIG"
    #[serde(rename="CONFIG")]
    CONFIG,
    

    /// Certificate issuance forbidden by an explicit CAA record for the domain or a failure to check CAA records for the domain.
    ///
    /// "CAA"
    #[serde(rename="CAA")]
    CAA,
    

    /// Reached a CA or internal rate-limit for the domain, e.g. for certificates per top-level private domain.
    ///
    /// "RATE_LIMITED"
    #[serde(rename="RATE_LIMITED")]
    RATELIMITED,
}

impl AsRef<str> for AuthorizationAttemptInfoFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizationAttemptInfoFailureReasonEnum::FAILUREREASONUNSPECIFIED => "FAILURE_REASON_UNSPECIFIED",
            AuthorizationAttemptInfoFailureReasonEnum::CONFIG => "CONFIG",
            AuthorizationAttemptInfoFailureReasonEnum::CAA => "CAA",
            AuthorizationAttemptInfoFailureReasonEnum::RATELIMITED => "RATE_LIMITED",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizationAttemptInfoFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_REASON_UNSPECIFIED" => Ok(AuthorizationAttemptInfoFailureReasonEnum::FAILUREREASONUNSPECIFIED),
           "CONFIG" => Ok(AuthorizationAttemptInfoFailureReasonEnum::CONFIG),
           "CAA" => Ok(AuthorizationAttemptInfoFailureReasonEnum::CAA),
           "RATE_LIMITED" => Ok(AuthorizationAttemptInfoFailureReasonEnum::RATELIMITED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizationAttemptInfoFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorizationAttemptInfoStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the domain for managed certificate issuance.
pub enum AuthorizationAttemptInfoStateEnum {
    

    /// State is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Certificate provisioning for this domain is under way. Google Cloud will attempt to authorize the domain.
    ///
    /// "AUTHORIZING"
    #[serde(rename="AUTHORIZING")]
    AUTHORIZING,
    

    /// A managed certificate can be provisioned, no issues for this domain.
    ///
    /// "AUTHORIZED"
    #[serde(rename="AUTHORIZED")]
    AUTHORIZED,
    

    /// Attempt to authorize the domain failed. This prevents the Managed Certificate from being issued. See `failure_reason` and `details` fields for more information.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for AuthorizationAttemptInfoStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizationAttemptInfoStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AuthorizationAttemptInfoStateEnum::AUTHORIZING => "AUTHORIZING",
            AuthorizationAttemptInfoStateEnum::AUTHORIZED => "AUTHORIZED",
            AuthorizationAttemptInfoStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizationAttemptInfoStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AuthorizationAttemptInfoStateEnum::STATEUNSPECIFIED),
           "AUTHORIZING" => Ok(AuthorizationAttemptInfoStateEnum::AUTHORIZING),
           "AUTHORIZED" => Ok(AuthorizationAttemptInfoStateEnum::AUTHORIZED),
           "FAILED" => Ok(AuthorizationAttemptInfoStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizationAttemptInfoStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The scope of the certificate.
pub enum CertificateScopeEnum {
    

    /// Certificates with default scope are served from core Google data centers. If unsure, choose this option.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence. See https://cloud.google.com/vpc/docs/edge-locations.
    ///
    /// "EDGE_CACHE"
    #[serde(rename="EDGE_CACHE")]
    EDGECACHE,
    

    /// Certificates with ALL_REGIONS scope are served from all Google Cloud regions. See https://cloud.google.com/compute/docs/regions-zones.
    ///
    /// "ALL_REGIONS"
    #[serde(rename="ALL_REGIONS")]
    ALLREGIONS,
}

impl AsRef<str> for CertificateScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateScopeEnum::DEFAULT => "DEFAULT",
            CertificateScopeEnum::EDGECACHE => "EDGE_CACHE",
            CertificateScopeEnum::ALLREGIONS => "ALL_REGIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(CertificateScopeEnum::DEFAULT),
           "EDGE_CACHE" => Ok(CertificateScopeEnum::EDGECACHE),
           "ALL_REGIONS" => Ok(CertificateScopeEnum::ALLREGIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateIssuanceConfigKeyAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The key algorithm to use when generating the private key.
pub enum CertificateIssuanceConfigKeyAlgorithmEnum {
    

    /// Unspecified key algorithm.
    ///
    /// "KEY_ALGORITHM_UNSPECIFIED"
    #[serde(rename="KEY_ALGORITHM_UNSPECIFIED")]
    KEYALGORITHMUNSPECIFIED,
    

    /// Specifies RSA with a 2048-bit modulus.
    ///
    /// "RSA_2048"
    #[serde(rename="RSA_2048")]
    RSA2048,
    

    /// Specifies ECDSA with curve P256.
    ///
    /// "ECDSA_P256"
    #[serde(rename="ECDSA_P256")]
    ECDSAP256,
}

impl AsRef<str> for CertificateIssuanceConfigKeyAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateIssuanceConfigKeyAlgorithmEnum::KEYALGORITHMUNSPECIFIED => "KEY_ALGORITHM_UNSPECIFIED",
            CertificateIssuanceConfigKeyAlgorithmEnum::RSA2048 => "RSA_2048",
            CertificateIssuanceConfigKeyAlgorithmEnum::ECDSAP256 => "ECDSA_P256",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateIssuanceConfigKeyAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_ALGORITHM_UNSPECIFIED" => Ok(CertificateIssuanceConfigKeyAlgorithmEnum::KEYALGORITHMUNSPECIFIED),
           "RSA_2048" => Ok(CertificateIssuanceConfigKeyAlgorithmEnum::RSA2048),
           "ECDSA_P256" => Ok(CertificateIssuanceConfigKeyAlgorithmEnum::ECDSAP256),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateIssuanceConfigKeyAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateMapEntryMatcherEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A predefined matcher for particular cases, other than SNI selection.
pub enum CertificateMapEntryMatcherEnum {
    

    /// A matcher has't been recognized.
    ///
    /// "MATCHER_UNSPECIFIED"
    #[serde(rename="MATCHER_UNSPECIFIED")]
    MATCHERUNSPECIFIED,
    

    /// A primary certificate that is served when SNI wasn't specified in the request or SNI couldn't be found in the map.
    ///
    /// "PRIMARY"
    #[serde(rename="PRIMARY")]
    PRIMARY,
}

impl AsRef<str> for CertificateMapEntryMatcherEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateMapEntryMatcherEnum::MATCHERUNSPECIFIED => "MATCHER_UNSPECIFIED",
            CertificateMapEntryMatcherEnum::PRIMARY => "PRIMARY",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateMapEntryMatcherEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCHER_UNSPECIFIED" => Ok(CertificateMapEntryMatcherEnum::MATCHERUNSPECIFIED),
           "PRIMARY" => Ok(CertificateMapEntryMatcherEnum::PRIMARY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateMapEntryMatcherEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateMapEntryStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A serving state of this Certificate Map Entry.
pub enum CertificateMapEntryStateEnum {
    

    /// The status is undefined.
    ///
    /// "SERVING_STATE_UNSPECIFIED"
    #[serde(rename="SERVING_STATE_UNSPECIFIED")]
    SERVINGSTATEUNSPECIFIED,
    

    /// The configuration is serving.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Update is in progress. Some frontends may serve this configuration.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for CertificateMapEntryStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateMapEntryStateEnum::SERVINGSTATEUNSPECIFIED => "SERVING_STATE_UNSPECIFIED",
            CertificateMapEntryStateEnum::ACTIVE => "ACTIVE",
            CertificateMapEntryStateEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateMapEntryStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVING_STATE_UNSPECIFIED" => Ok(CertificateMapEntryStateEnum::SERVINGSTATEUNSPECIFIED),
           "ACTIVE" => Ok(CertificateMapEntryStateEnum::ACTIVE),
           "PENDING" => Ok(CertificateMapEntryStateEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateMapEntryStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsAuthorizationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. Type of DnsAuthorization. If unset during resource creation the following default will be used: - in location global: FIXED_RECORD.
pub enum DnsAuthorizationTypeEnum {
    

    /// Type is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// FIXED_RECORD DNS authorization uses DNS-01 validation method.
    ///
    /// "FIXED_RECORD"
    #[serde(rename="FIXED_RECORD")]
    FIXEDRECORD,
    

    /// PER_PROJECT_RECORD DNS authorization allows for independent management of Google-managed certificates with DNS authorization across multiple projects.
    ///
    /// "PER_PROJECT_RECORD"
    #[serde(rename="PER_PROJECT_RECORD")]
    PERPROJECTRECORD,
}

impl AsRef<str> for DnsAuthorizationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsAuthorizationTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DnsAuthorizationTypeEnum::FIXEDRECORD => "FIXED_RECORD",
            DnsAuthorizationTypeEnum::PERPROJECTRECORD => "PER_PROJECT_RECORD",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsAuthorizationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DnsAuthorizationTypeEnum::TYPEUNSPECIFIED),
           "FIXED_RECORD" => Ok(DnsAuthorizationTypeEnum::FIXEDRECORD),
           "PER_PROJECT_RECORD" => Ok(DnsAuthorizationTypeEnum::PERPROJECTRECORD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsAuthorizationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedCertificateStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the managed certificate resource.
pub enum ManagedCertificateStateEnum {
    

    /// State is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Certificate Manager attempts to provision or renew the certificate. If the process takes longer than expected, consult the `provisioning_issue` field.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// Multiple certificate provisioning attempts failed and Certificate Manager gave up. To try again, delete and create a new managed Certificate resource. For details see the `provisioning_issue` field.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The certificate management is working, and a certificate has been provisioned.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for ManagedCertificateStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedCertificateStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ManagedCertificateStateEnum::PROVISIONING => "PROVISIONING",
            ManagedCertificateStateEnum::FAILED => "FAILED",
            ManagedCertificateStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedCertificateStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ManagedCertificateStateEnum::STATEUNSPECIFIED),
           "PROVISIONING" => Ok(ManagedCertificateStateEnum::PROVISIONING),
           "FAILED" => Ok(ManagedCertificateStateEnum::FAILED),
           "ACTIVE" => Ok(ManagedCertificateStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedCertificateStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProvisioningIssueReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Reason for provisioning failures.
pub enum ProvisioningIssueReasonEnum {
    

    /// Reason is unspecified.
    ///
    /// "REASON_UNSPECIFIED"
    #[serde(rename="REASON_UNSPECIFIED")]
    REASONUNSPECIFIED,
    

    /// Certificate provisioning failed due to an issue with one or more of the domains on the certificate. For details of which domains failed, consult the `authorization_attempt_info` field.
    ///
    /// "AUTHORIZATION_ISSUE"
    #[serde(rename="AUTHORIZATION_ISSUE")]
    AUTHORIZATIONISSUE,
    

    /// Exceeded Certificate Authority quotas or internal rate limits of the system. Provisioning may take longer to complete.
    ///
    /// "RATE_LIMITED"
    #[serde(rename="RATE_LIMITED")]
    RATELIMITED,
}

impl AsRef<str> for ProvisioningIssueReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProvisioningIssueReasonEnum::REASONUNSPECIFIED => "REASON_UNSPECIFIED",
            ProvisioningIssueReasonEnum::AUTHORIZATIONISSUE => "AUTHORIZATION_ISSUE",
            ProvisioningIssueReasonEnum::RATELIMITED => "RATE_LIMITED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProvisioningIssueReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNSPECIFIED" => Ok(ProvisioningIssueReasonEnum::REASONUNSPECIFIED),
           "AUTHORIZATION_ISSUE" => Ok(ProvisioningIssueReasonEnum::AUTHORIZATIONISSUE),
           "RATE_LIMITED" => Ok(ProvisioningIssueReasonEnum::RATELIMITED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProvisioningIssueReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


