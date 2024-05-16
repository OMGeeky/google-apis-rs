use super::*;



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


// region CaPoolTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The Tier of this CaPool.
pub enum CaPoolTierEnum {
    

    /// Not specified.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// Enterprise tier.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// DevOps tier.
    ///
    /// "DEVOPS"
    #[serde(rename="DEVOPS")]
    DEVOPS,
}

impl AsRef<str> for CaPoolTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaPoolTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            CaPoolTierEnum::ENTERPRISE => "ENTERPRISE",
            CaPoolTierEnum::DEVOPS => "DEVOPS",
        }
    }
}

impl std::convert::TryFrom< &str> for CaPoolTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(CaPoolTierEnum::TIERUNSPECIFIED),
           "ENTERPRISE" => Ok(CaPoolTierEnum::ENTERPRISE),
           "DEVOPS" => Ok(CaPoolTierEnum::DEVOPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaPoolTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateSubjectModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. Specifies how the Certificate's identity fields are to be decided. If this is omitted, the `DEFAULT` subject mode will be used.
pub enum CertificateSubjectModeEnum {
    

    /// Not specified.
    ///
    /// "SUBJECT_REQUEST_MODE_UNSPECIFIED"
    #[serde(rename="SUBJECT_REQUEST_MODE_UNSPECIFIED")]
    SUBJECTREQUESTMODEUNSPECIFIED,
    

    /// The default mode used in most cases. Indicates that the certificate's Subject and/or SubjectAltNames are specified in the certificate request. This mode requires the caller to have the `privateca.certificates.create` permission.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// A mode reserved for special cases. Indicates that the certificate should have one SPIFFE SubjectAltNames set by the service based on the caller's identity. This mode will ignore any explicitly specified Subject and/or SubjectAltNames in the certificate request. This mode requires the caller to have the `privateca.certificates.createForSelf` permission.
    ///
    /// "REFLECTED_SPIFFE"
    #[serde(rename="REFLECTED_SPIFFE")]
    REFLECTEDSPIFFE,
}

impl AsRef<str> for CertificateSubjectModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateSubjectModeEnum::SUBJECTREQUESTMODEUNSPECIFIED => "SUBJECT_REQUEST_MODE_UNSPECIFIED",
            CertificateSubjectModeEnum::DEFAULT => "DEFAULT",
            CertificateSubjectModeEnum::REFLECTEDSPIFFE => "REFLECTED_SPIFFE",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateSubjectModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBJECT_REQUEST_MODE_UNSPECIFIED" => Ok(CertificateSubjectModeEnum::SUBJECTREQUESTMODEUNSPECIFIED),
           "DEFAULT" => Ok(CertificateSubjectModeEnum::DEFAULT),
           "REFLECTED_SPIFFE" => Ok(CertificateSubjectModeEnum::REFLECTEDSPIFFE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateSubjectModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateAuthorityStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The State for this CertificateAuthority.
pub enum CertificateAuthorityStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Certificates can be issued from this CA. CRLs will be generated for this CA. The CA will be part of the CaPool's trust anchor, and will be used to issue certificates from the CaPool.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Certificates cannot be issued from this CA. CRLs will still be generated. The CA will be part of the CaPool's trust anchor, but will not be used to issue certificates from the CaPool.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Certificates can be issued from this CA. CRLs will be generated for this CA. The CA will be part of the CaPool's trust anchor, but will not be used to issue certificates from the CaPool.
    ///
    /// "STAGED"
    #[serde(rename="STAGED")]
    STAGED,
    

    /// Certificates cannot be issued from this CA. CRLs will not be generated. The CA will not be part of the CaPool's trust anchor, and will not be used to issue certificates from the CaPool.
    ///
    /// "AWAITING_USER_ACTIVATION"
    #[serde(rename="AWAITING_USER_ACTIVATION")]
    AWAITINGUSERACTIVATION,
    

    /// Certificates cannot be issued from this CA. CRLs will not be generated. The CA may still be recovered by calling CertificateAuthorityService.UndeleteCertificateAuthority before expire_time. The CA will not be part of the CaPool's trust anchor, and will not be used to issue certificates from the CaPool.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CertificateAuthorityStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateAuthorityStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CertificateAuthorityStateEnum::ENABLED => "ENABLED",
            CertificateAuthorityStateEnum::DISABLED => "DISABLED",
            CertificateAuthorityStateEnum::STAGED => "STAGED",
            CertificateAuthorityStateEnum::AWAITINGUSERACTIVATION => "AWAITING_USER_ACTIVATION",
            CertificateAuthorityStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateAuthorityStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CertificateAuthorityStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(CertificateAuthorityStateEnum::ENABLED),
           "DISABLED" => Ok(CertificateAuthorityStateEnum::DISABLED),
           "STAGED" => Ok(CertificateAuthorityStateEnum::STAGED),
           "AWAITING_USER_ACTIVATION" => Ok(CertificateAuthorityStateEnum::AWAITINGUSERACTIVATION),
           "DELETED" => Ok(CertificateAuthorityStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateAuthorityStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateAuthorityTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The CaPool.Tier of the CaPool that includes this CertificateAuthority.
pub enum CertificateAuthorityTierEnum {
    

    /// Not specified.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// Enterprise tier.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// DevOps tier.
    ///
    /// "DEVOPS"
    #[serde(rename="DEVOPS")]
    DEVOPS,
}

impl AsRef<str> for CertificateAuthorityTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateAuthorityTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            CertificateAuthorityTierEnum::ENTERPRISE => "ENTERPRISE",
            CertificateAuthorityTierEnum::DEVOPS => "DEVOPS",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateAuthorityTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(CertificateAuthorityTierEnum::TIERUNSPECIFIED),
           "ENTERPRISE" => Ok(CertificateAuthorityTierEnum::ENTERPRISE),
           "DEVOPS" => Ok(CertificateAuthorityTierEnum::DEVOPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateAuthorityTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateAuthorityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The Type of this CertificateAuthority.
pub enum CertificateAuthorityTypeEnum {
    

    /// Not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Self-signed CA.
    ///
    /// "SELF_SIGNED"
    #[serde(rename="SELF_SIGNED")]
    SELFSIGNED,
    

    /// Subordinate CA. Could be issued by a Private CA CertificateAuthority or an unmanaged CA.
    ///
    /// "SUBORDINATE"
    #[serde(rename="SUBORDINATE")]
    SUBORDINATE,
}

impl AsRef<str> for CertificateAuthorityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateAuthorityTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            CertificateAuthorityTypeEnum::SELFSIGNED => "SELF_SIGNED",
            CertificateAuthorityTypeEnum::SUBORDINATE => "SUBORDINATE",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateAuthorityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(CertificateAuthorityTypeEnum::TYPEUNSPECIFIED),
           "SELF_SIGNED" => Ok(CertificateAuthorityTypeEnum::SELFSIGNED),
           "SUBORDINATE" => Ok(CertificateAuthorityTypeEnum::SUBORDINATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateAuthorityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateExtensionConstraintKnownExtensionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A set of named X.509 extensions. Will be combined with additional_extensions to determine the full set of X.509 extensions.
pub enum CertificateExtensionConstraintKnownExtensionsEnum {
    

    /// Not specified.
    ///
    /// "KNOWN_CERTIFICATE_EXTENSION_UNSPECIFIED"
    #[serde(rename="KNOWN_CERTIFICATE_EXTENSION_UNSPECIFIED")]
    KNOWNCERTIFICATEEXTENSIONUNSPECIFIED,
    

    /// Refers to a certificate's Key Usage extension, as described in [RFC 5280 section 4.2.1.3](https://tools.ietf.org/html/rfc5280#section-4.2.1.3). This corresponds to the KeyUsage.base_key_usage field.
    ///
    /// "BASE_KEY_USAGE"
    #[serde(rename="BASE_KEY_USAGE")]
    BASEKEYUSAGE,
    

    /// Refers to a certificate's Extended Key Usage extension, as described in [RFC 5280 section 4.2.1.12](https://tools.ietf.org/html/rfc5280#section-4.2.1.12). This corresponds to the KeyUsage.extended_key_usage message.
    ///
    /// "EXTENDED_KEY_USAGE"
    #[serde(rename="EXTENDED_KEY_USAGE")]
    EXTENDEDKEYUSAGE,
    

    /// Refers to a certificate's Basic Constraints extension, as described in [RFC 5280 section 4.2.1.9](https://tools.ietf.org/html/rfc5280#section-4.2.1.9). This corresponds to the X509Parameters.ca_options field.
    ///
    /// "CA_OPTIONS"
    #[serde(rename="CA_OPTIONS")]
    CAOPTIONS,
    

    /// Refers to a certificate's Policy object identifiers, as described in [RFC 5280 section 4.2.1.4](https://tools.ietf.org/html/rfc5280#section-4.2.1.4). This corresponds to the X509Parameters.policy_ids field.
    ///
    /// "POLICY_IDS"
    #[serde(rename="POLICY_IDS")]
    POLICYIDS,
    

    /// Refers to OCSP servers in a certificate's Authority Information Access extension, as described in [RFC 5280 section 4.2.2.1](https://tools.ietf.org/html/rfc5280#section-4.2.2.1), This corresponds to the X509Parameters.aia_ocsp_servers field.
    ///
    /// "AIA_OCSP_SERVERS"
    #[serde(rename="AIA_OCSP_SERVERS")]
    AIAOCSPSERVERS,
    

    /// Refers to Name Constraints extension as described in [RFC 5280 section 4.2.1.10](https://tools.ietf.org/html/rfc5280#section-4.2.1.10)
    ///
    /// "NAME_CONSTRAINTS"
    #[serde(rename="NAME_CONSTRAINTS")]
    NAMECONSTRAINTS,
}

impl AsRef<str> for CertificateExtensionConstraintKnownExtensionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateExtensionConstraintKnownExtensionsEnum::KNOWNCERTIFICATEEXTENSIONUNSPECIFIED => "KNOWN_CERTIFICATE_EXTENSION_UNSPECIFIED",
            CertificateExtensionConstraintKnownExtensionsEnum::BASEKEYUSAGE => "BASE_KEY_USAGE",
            CertificateExtensionConstraintKnownExtensionsEnum::EXTENDEDKEYUSAGE => "EXTENDED_KEY_USAGE",
            CertificateExtensionConstraintKnownExtensionsEnum::CAOPTIONS => "CA_OPTIONS",
            CertificateExtensionConstraintKnownExtensionsEnum::POLICYIDS => "POLICY_IDS",
            CertificateExtensionConstraintKnownExtensionsEnum::AIAOCSPSERVERS => "AIA_OCSP_SERVERS",
            CertificateExtensionConstraintKnownExtensionsEnum::NAMECONSTRAINTS => "NAME_CONSTRAINTS",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateExtensionConstraintKnownExtensionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KNOWN_CERTIFICATE_EXTENSION_UNSPECIFIED" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::KNOWNCERTIFICATEEXTENSIONUNSPECIFIED),
           "BASE_KEY_USAGE" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::BASEKEYUSAGE),
           "EXTENDED_KEY_USAGE" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::EXTENDEDKEYUSAGE),
           "CA_OPTIONS" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::CAOPTIONS),
           "POLICY_IDS" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::POLICYIDS),
           "AIA_OCSP_SERVERS" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::AIAOCSPSERVERS),
           "NAME_CONSTRAINTS" => Ok(CertificateExtensionConstraintKnownExtensionsEnum::NAMECONSTRAINTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateExtensionConstraintKnownExtensionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateRevocationListStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The State for this CertificateRevocationList.
pub enum CertificateRevocationListStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The CertificateRevocationList is up to date.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The CertificateRevocationList is no longer current.
    ///
    /// "SUPERSEDED"
    #[serde(rename="SUPERSEDED")]
    SUPERSEDED,
}

impl AsRef<str> for CertificateRevocationListStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateRevocationListStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CertificateRevocationListStateEnum::ACTIVE => "ACTIVE",
            CertificateRevocationListStateEnum::SUPERSEDED => "SUPERSEDED",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateRevocationListStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CertificateRevocationListStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(CertificateRevocationListStateEnum::ACTIVE),
           "SUPERSEDED" => Ok(CertificateRevocationListStateEnum::SUPERSEDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateRevocationListStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EcKeyTypeSignatureAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A signature algorithm that must be used. If this is omitted, any EC-based signature algorithm will be allowed.
pub enum EcKeyTypeSignatureAlgorithmEnum {
    

    /// Not specified. Signifies that any signature algorithm may be used.
    ///
    /// "EC_SIGNATURE_ALGORITHM_UNSPECIFIED"
    #[serde(rename="EC_SIGNATURE_ALGORITHM_UNSPECIFIED")]
    ECSIGNATUREALGORITHMUNSPECIFIED,
    

    /// Refers to the Elliptic Curve Digital Signature Algorithm over the NIST P-256 curve.
    ///
    /// "ECDSA_P256"
    #[serde(rename="ECDSA_P256")]
    ECDSAP256,
    

    /// Refers to the Elliptic Curve Digital Signature Algorithm over the NIST P-384 curve.
    ///
    /// "ECDSA_P384"
    #[serde(rename="ECDSA_P384")]
    ECDSAP384,
    

    /// Refers to the Edwards-curve Digital Signature Algorithm over curve 25519, as described in RFC 8410.
    ///
    /// "EDDSA_25519"
    #[serde(rename="EDDSA_25519")]
    EDDSA25519,
}

impl AsRef<str> for EcKeyTypeSignatureAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EcKeyTypeSignatureAlgorithmEnum::ECSIGNATUREALGORITHMUNSPECIFIED => "EC_SIGNATURE_ALGORITHM_UNSPECIFIED",
            EcKeyTypeSignatureAlgorithmEnum::ECDSAP256 => "ECDSA_P256",
            EcKeyTypeSignatureAlgorithmEnum::ECDSAP384 => "ECDSA_P384",
            EcKeyTypeSignatureAlgorithmEnum::EDDSA25519 => "EDDSA_25519",
        }
    }
}

impl std::convert::TryFrom< &str> for EcKeyTypeSignatureAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EC_SIGNATURE_ALGORITHM_UNSPECIFIED" => Ok(EcKeyTypeSignatureAlgorithmEnum::ECSIGNATUREALGORITHMUNSPECIFIED),
           "ECDSA_P256" => Ok(EcKeyTypeSignatureAlgorithmEnum::ECDSAP256),
           "ECDSA_P384" => Ok(EcKeyTypeSignatureAlgorithmEnum::ECDSAP384),
           "EDDSA_25519" => Ok(EcKeyTypeSignatureAlgorithmEnum::EDDSA25519),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EcKeyTypeSignatureAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyVersionSpecAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The algorithm to use for creating a managed Cloud KMS key for a for a simplified experience. All managed keys will be have their ProtectionLevel as `HSM`.
pub enum KeyVersionSpecAlgorithmEnum {
    

    /// Not specified.
    ///
    /// "SIGN_HASH_ALGORITHM_UNSPECIFIED"
    #[serde(rename="SIGN_HASH_ALGORITHM_UNSPECIFIED")]
    SIGNHASHALGORITHMUNSPECIFIED,
    

    /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PSS_2048_SHA256
    ///
    /// "RSA_PSS_2048_SHA256"
    #[serde(rename="RSA_PSS_2048_SHA256")]
    RSAPSS2048SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm. RSA_SIGN_PSS_3072_SHA256
    ///
    /// "RSA_PSS_3072_SHA256"
    #[serde(rename="RSA_PSS_3072_SHA256")]
    RSAPSS3072SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PSS_4096_SHA256
    ///
    /// "RSA_PSS_4096_SHA256"
    #[serde(rename="RSA_PSS_4096_SHA256")]
    RSAPSS4096SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_2048_SHA256
    ///
    /// "RSA_PKCS1_2048_SHA256"
    #[serde(rename="RSA_PKCS1_2048_SHA256")]
    RSAPKCS12048SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_3072_SHA256
    ///
    /// "RSA_PKCS1_3072_SHA256"
    #[serde(rename="RSA_PKCS1_3072_SHA256")]
    RSAPKCS13072SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_4096_SHA256
    ///
    /// "RSA_PKCS1_4096_SHA256"
    #[serde(rename="RSA_PKCS1_4096_SHA256")]
    RSAPKCS14096SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P256_SHA256
    ///
    /// "EC_P256_SHA256"
    #[serde(rename="EC_P256_SHA256")]
    ECP256SHA256,
    

    /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P384_SHA384
    ///
    /// "EC_P384_SHA384"
    #[serde(rename="EC_P384_SHA384")]
    ECP384SHA384,
}

impl AsRef<str> for KeyVersionSpecAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyVersionSpecAlgorithmEnum::SIGNHASHALGORITHMUNSPECIFIED => "SIGN_HASH_ALGORITHM_UNSPECIFIED",
            KeyVersionSpecAlgorithmEnum::RSAPSS2048SHA256 => "RSA_PSS_2048_SHA256",
            KeyVersionSpecAlgorithmEnum::RSAPSS3072SHA256 => "RSA_PSS_3072_SHA256",
            KeyVersionSpecAlgorithmEnum::RSAPSS4096SHA256 => "RSA_PSS_4096_SHA256",
            KeyVersionSpecAlgorithmEnum::RSAPKCS12048SHA256 => "RSA_PKCS1_2048_SHA256",
            KeyVersionSpecAlgorithmEnum::RSAPKCS13072SHA256 => "RSA_PKCS1_3072_SHA256",
            KeyVersionSpecAlgorithmEnum::RSAPKCS14096SHA256 => "RSA_PKCS1_4096_SHA256",
            KeyVersionSpecAlgorithmEnum::ECP256SHA256 => "EC_P256_SHA256",
            KeyVersionSpecAlgorithmEnum::ECP384SHA384 => "EC_P384_SHA384",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyVersionSpecAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SIGN_HASH_ALGORITHM_UNSPECIFIED" => Ok(KeyVersionSpecAlgorithmEnum::SIGNHASHALGORITHMUNSPECIFIED),
           "RSA_PSS_2048_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::RSAPSS2048SHA256),
           "RSA_PSS_3072_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::RSAPSS3072SHA256),
           "RSA_PSS_4096_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::RSAPSS4096SHA256),
           "RSA_PKCS1_2048_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::RSAPKCS12048SHA256),
           "RSA_PKCS1_3072_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::RSAPKCS13072SHA256),
           "RSA_PKCS1_4096_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::RSAPKCS14096SHA256),
           "EC_P256_SHA256" => Ok(KeyVersionSpecAlgorithmEnum::ECP256SHA256),
           "EC_P384_SHA384" => Ok(KeyVersionSpecAlgorithmEnum::ECP384SHA384),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KeyVersionSpecAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublicKeyFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The format of the public key.
pub enum PublicKeyFormatEnum {
    

    /// Default unspecified value.
    ///
    /// "KEY_FORMAT_UNSPECIFIED"
    #[serde(rename="KEY_FORMAT_UNSPECIFIED")]
    KEYFORMATUNSPECIFIED,
    

    /// The key is PEM-encoded as defined in [RFC 7468](https://tools.ietf.org/html/rfc7468). It can be any of the following: a PEM-encoded PKCS#1/RFC 3447 RSAPublicKey structure, an RFC 5280 [SubjectPublicKeyInfo](https://tools.ietf.org/html/rfc5280#section-4.1) or a PEM-encoded X.509 certificate signing request (CSR). If a [SubjectPublicKeyInfo](https://tools.ietf.org/html/rfc5280#section-4.1) is specified, it can contain a A PEM-encoded PKCS#1/RFC 3447 RSAPublicKey or a NIST P-256/secp256r1/prime256v1 or P-384 key. If a CSR is specified, it will used solely for the purpose of extracting the public key. When generated by the service, it will always be an RFC 5280 [SubjectPublicKeyInfo](https://tools.ietf.org/html/rfc5280#section-4.1) structure containing an algorithm identifier and a key.
    ///
    /// "PEM"
    #[serde(rename="PEM")]
    PEM,
}

impl AsRef<str> for PublicKeyFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublicKeyFormatEnum::KEYFORMATUNSPECIFIED => "KEY_FORMAT_UNSPECIFIED",
            PublicKeyFormatEnum::PEM => "PEM",
        }
    }
}

impl std::convert::TryFrom< &str> for PublicKeyFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_FORMAT_UNSPECIFIED" => Ok(PublicKeyFormatEnum::KEYFORMATUNSPECIFIED),
           "PEM" => Ok(PublicKeyFormatEnum::PEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublicKeyFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublishingOptionEncodingFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the encoding format of each CertificateAuthority resource's CA certificate and CRLs. If this is omitted, CA certificates and CRLs will be published in PEM.
pub enum PublishingOptionEncodingFormatEnum {
    

    /// Not specified. By default, PEM format will be used.
    ///
    /// "ENCODING_FORMAT_UNSPECIFIED"
    #[serde(rename="ENCODING_FORMAT_UNSPECIFIED")]
    ENCODINGFORMATUNSPECIFIED,
    

    /// The CertificateAuthority's CA certificate and CRLs will be published in PEM format.
    ///
    /// "PEM"
    #[serde(rename="PEM")]
    PEM,
    

    /// The CertificateAuthority's CA certificate and CRLs will be published in DER format.
    ///
    /// "DER"
    #[serde(rename="DER")]
    DER,
}

impl AsRef<str> for PublishingOptionEncodingFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublishingOptionEncodingFormatEnum::ENCODINGFORMATUNSPECIFIED => "ENCODING_FORMAT_UNSPECIFIED",
            PublishingOptionEncodingFormatEnum::PEM => "PEM",
            PublishingOptionEncodingFormatEnum::DER => "DER",
        }
    }
}

impl std::convert::TryFrom< &str> for PublishingOptionEncodingFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCODING_FORMAT_UNSPECIFIED" => Ok(PublishingOptionEncodingFormatEnum::ENCODINGFORMATUNSPECIFIED),
           "PEM" => Ok(PublishingOptionEncodingFormatEnum::PEM),
           "DER" => Ok(PublishingOptionEncodingFormatEnum::DER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublishingOptionEncodingFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RevocationDetailRevocationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates why a Certificate was revoked.
pub enum RevocationDetailRevocationStateEnum {
    

    /// Default unspecified value. This value does indicate that a Certificate has been revoked, but that a reason has not been recorded.
    ///
    /// "REVOCATION_REASON_UNSPECIFIED"
    #[serde(rename="REVOCATION_REASON_UNSPECIFIED")]
    REVOCATIONREASONUNSPECIFIED,
    

    /// Key material for this Certificate may have leaked.
    ///
    /// "KEY_COMPROMISE"
    #[serde(rename="KEY_COMPROMISE")]
    KEYCOMPROMISE,
    

    /// The key material for a certificate authority in the issuing path may have leaked.
    ///
    /// "CERTIFICATE_AUTHORITY_COMPROMISE"
    #[serde(rename="CERTIFICATE_AUTHORITY_COMPROMISE")]
    CERTIFICATEAUTHORITYCOMPROMISE,
    

    /// The subject or other attributes in this Certificate have changed.
    ///
    /// "AFFILIATION_CHANGED"
    #[serde(rename="AFFILIATION_CHANGED")]
    AFFILIATIONCHANGED,
    

    /// This Certificate has been superseded.
    ///
    /// "SUPERSEDED"
    #[serde(rename="SUPERSEDED")]
    SUPERSEDED,
    

    /// This Certificate or entities in the issuing path have ceased to operate.
    ///
    /// "CESSATION_OF_OPERATION"
    #[serde(rename="CESSATION_OF_OPERATION")]
    CESSATIONOFOPERATION,
    

    /// This Certificate should not be considered valid, it is expected that it may become valid in the future.
    ///
    /// "CERTIFICATE_HOLD"
    #[serde(rename="CERTIFICATE_HOLD")]
    CERTIFICATEHOLD,
    

    /// This Certificate no longer has permission to assert the listed attributes.
    ///
    /// "PRIVILEGE_WITHDRAWN"
    #[serde(rename="PRIVILEGE_WITHDRAWN")]
    PRIVILEGEWITHDRAWN,
    

    /// The authority which determines appropriate attributes for a Certificate may have been compromised.
    ///
    /// "ATTRIBUTE_AUTHORITY_COMPROMISE"
    #[serde(rename="ATTRIBUTE_AUTHORITY_COMPROMISE")]
    ATTRIBUTEAUTHORITYCOMPROMISE,
}

impl AsRef<str> for RevocationDetailRevocationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RevocationDetailRevocationStateEnum::REVOCATIONREASONUNSPECIFIED => "REVOCATION_REASON_UNSPECIFIED",
            RevocationDetailRevocationStateEnum::KEYCOMPROMISE => "KEY_COMPROMISE",
            RevocationDetailRevocationStateEnum::CERTIFICATEAUTHORITYCOMPROMISE => "CERTIFICATE_AUTHORITY_COMPROMISE",
            RevocationDetailRevocationStateEnum::AFFILIATIONCHANGED => "AFFILIATION_CHANGED",
            RevocationDetailRevocationStateEnum::SUPERSEDED => "SUPERSEDED",
            RevocationDetailRevocationStateEnum::CESSATIONOFOPERATION => "CESSATION_OF_OPERATION",
            RevocationDetailRevocationStateEnum::CERTIFICATEHOLD => "CERTIFICATE_HOLD",
            RevocationDetailRevocationStateEnum::PRIVILEGEWITHDRAWN => "PRIVILEGE_WITHDRAWN",
            RevocationDetailRevocationStateEnum::ATTRIBUTEAUTHORITYCOMPROMISE => "ATTRIBUTE_AUTHORITY_COMPROMISE",
        }
    }
}

impl std::convert::TryFrom< &str> for RevocationDetailRevocationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVOCATION_REASON_UNSPECIFIED" => Ok(RevocationDetailRevocationStateEnum::REVOCATIONREASONUNSPECIFIED),
           "KEY_COMPROMISE" => Ok(RevocationDetailRevocationStateEnum::KEYCOMPROMISE),
           "CERTIFICATE_AUTHORITY_COMPROMISE" => Ok(RevocationDetailRevocationStateEnum::CERTIFICATEAUTHORITYCOMPROMISE),
           "AFFILIATION_CHANGED" => Ok(RevocationDetailRevocationStateEnum::AFFILIATIONCHANGED),
           "SUPERSEDED" => Ok(RevocationDetailRevocationStateEnum::SUPERSEDED),
           "CESSATION_OF_OPERATION" => Ok(RevocationDetailRevocationStateEnum::CESSATIONOFOPERATION),
           "CERTIFICATE_HOLD" => Ok(RevocationDetailRevocationStateEnum::CERTIFICATEHOLD),
           "PRIVILEGE_WITHDRAWN" => Ok(RevocationDetailRevocationStateEnum::PRIVILEGEWITHDRAWN),
           "ATTRIBUTE_AUTHORITY_COMPROMISE" => Ok(RevocationDetailRevocationStateEnum::ATTRIBUTEAUTHORITYCOMPROMISE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RevocationDetailRevocationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RevokeCertificateRequestReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The RevocationReason for revoking this certificate.
pub enum RevokeCertificateRequestReasonEnum {
    

    /// Default unspecified value. This value does indicate that a Certificate has been revoked, but that a reason has not been recorded.
    ///
    /// "REVOCATION_REASON_UNSPECIFIED"
    #[serde(rename="REVOCATION_REASON_UNSPECIFIED")]
    REVOCATIONREASONUNSPECIFIED,
    

    /// Key material for this Certificate may have leaked.
    ///
    /// "KEY_COMPROMISE"
    #[serde(rename="KEY_COMPROMISE")]
    KEYCOMPROMISE,
    

    /// The key material for a certificate authority in the issuing path may have leaked.
    ///
    /// "CERTIFICATE_AUTHORITY_COMPROMISE"
    #[serde(rename="CERTIFICATE_AUTHORITY_COMPROMISE")]
    CERTIFICATEAUTHORITYCOMPROMISE,
    

    /// The subject or other attributes in this Certificate have changed.
    ///
    /// "AFFILIATION_CHANGED"
    #[serde(rename="AFFILIATION_CHANGED")]
    AFFILIATIONCHANGED,
    

    /// This Certificate has been superseded.
    ///
    /// "SUPERSEDED"
    #[serde(rename="SUPERSEDED")]
    SUPERSEDED,
    

    /// This Certificate or entities in the issuing path have ceased to operate.
    ///
    /// "CESSATION_OF_OPERATION"
    #[serde(rename="CESSATION_OF_OPERATION")]
    CESSATIONOFOPERATION,
    

    /// This Certificate should not be considered valid, it is expected that it may become valid in the future.
    ///
    /// "CERTIFICATE_HOLD"
    #[serde(rename="CERTIFICATE_HOLD")]
    CERTIFICATEHOLD,
    

    /// This Certificate no longer has permission to assert the listed attributes.
    ///
    /// "PRIVILEGE_WITHDRAWN"
    #[serde(rename="PRIVILEGE_WITHDRAWN")]
    PRIVILEGEWITHDRAWN,
    

    /// The authority which determines appropriate attributes for a Certificate may have been compromised.
    ///
    /// "ATTRIBUTE_AUTHORITY_COMPROMISE"
    #[serde(rename="ATTRIBUTE_AUTHORITY_COMPROMISE")]
    ATTRIBUTEAUTHORITYCOMPROMISE,
}

impl AsRef<str> for RevokeCertificateRequestReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RevokeCertificateRequestReasonEnum::REVOCATIONREASONUNSPECIFIED => "REVOCATION_REASON_UNSPECIFIED",
            RevokeCertificateRequestReasonEnum::KEYCOMPROMISE => "KEY_COMPROMISE",
            RevokeCertificateRequestReasonEnum::CERTIFICATEAUTHORITYCOMPROMISE => "CERTIFICATE_AUTHORITY_COMPROMISE",
            RevokeCertificateRequestReasonEnum::AFFILIATIONCHANGED => "AFFILIATION_CHANGED",
            RevokeCertificateRequestReasonEnum::SUPERSEDED => "SUPERSEDED",
            RevokeCertificateRequestReasonEnum::CESSATIONOFOPERATION => "CESSATION_OF_OPERATION",
            RevokeCertificateRequestReasonEnum::CERTIFICATEHOLD => "CERTIFICATE_HOLD",
            RevokeCertificateRequestReasonEnum::PRIVILEGEWITHDRAWN => "PRIVILEGE_WITHDRAWN",
            RevokeCertificateRequestReasonEnum::ATTRIBUTEAUTHORITYCOMPROMISE => "ATTRIBUTE_AUTHORITY_COMPROMISE",
        }
    }
}

impl std::convert::TryFrom< &str> for RevokeCertificateRequestReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVOCATION_REASON_UNSPECIFIED" => Ok(RevokeCertificateRequestReasonEnum::REVOCATIONREASONUNSPECIFIED),
           "KEY_COMPROMISE" => Ok(RevokeCertificateRequestReasonEnum::KEYCOMPROMISE),
           "CERTIFICATE_AUTHORITY_COMPROMISE" => Ok(RevokeCertificateRequestReasonEnum::CERTIFICATEAUTHORITYCOMPROMISE),
           "AFFILIATION_CHANGED" => Ok(RevokeCertificateRequestReasonEnum::AFFILIATIONCHANGED),
           "SUPERSEDED" => Ok(RevokeCertificateRequestReasonEnum::SUPERSEDED),
           "CESSATION_OF_OPERATION" => Ok(RevokeCertificateRequestReasonEnum::CESSATIONOFOPERATION),
           "CERTIFICATE_HOLD" => Ok(RevokeCertificateRequestReasonEnum::CERTIFICATEHOLD),
           "PRIVILEGE_WITHDRAWN" => Ok(RevokeCertificateRequestReasonEnum::PRIVILEGEWITHDRAWN),
           "ATTRIBUTE_AUTHORITY_COMPROMISE" => Ok(RevokeCertificateRequestReasonEnum::ATTRIBUTEAUTHORITYCOMPROMISE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RevokeCertificateRequestReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RevokedCertificateRevocationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason the Certificate was revoked.
pub enum RevokedCertificateRevocationReasonEnum {
    

    /// Default unspecified value. This value does indicate that a Certificate has been revoked, but that a reason has not been recorded.
    ///
    /// "REVOCATION_REASON_UNSPECIFIED"
    #[serde(rename="REVOCATION_REASON_UNSPECIFIED")]
    REVOCATIONREASONUNSPECIFIED,
    

    /// Key material for this Certificate may have leaked.
    ///
    /// "KEY_COMPROMISE"
    #[serde(rename="KEY_COMPROMISE")]
    KEYCOMPROMISE,
    

    /// The key material for a certificate authority in the issuing path may have leaked.
    ///
    /// "CERTIFICATE_AUTHORITY_COMPROMISE"
    #[serde(rename="CERTIFICATE_AUTHORITY_COMPROMISE")]
    CERTIFICATEAUTHORITYCOMPROMISE,
    

    /// The subject or other attributes in this Certificate have changed.
    ///
    /// "AFFILIATION_CHANGED"
    #[serde(rename="AFFILIATION_CHANGED")]
    AFFILIATIONCHANGED,
    

    /// This Certificate has been superseded.
    ///
    /// "SUPERSEDED"
    #[serde(rename="SUPERSEDED")]
    SUPERSEDED,
    

    /// This Certificate or entities in the issuing path have ceased to operate.
    ///
    /// "CESSATION_OF_OPERATION"
    #[serde(rename="CESSATION_OF_OPERATION")]
    CESSATIONOFOPERATION,
    

    /// This Certificate should not be considered valid, it is expected that it may become valid in the future.
    ///
    /// "CERTIFICATE_HOLD"
    #[serde(rename="CERTIFICATE_HOLD")]
    CERTIFICATEHOLD,
    

    /// This Certificate no longer has permission to assert the listed attributes.
    ///
    /// "PRIVILEGE_WITHDRAWN"
    #[serde(rename="PRIVILEGE_WITHDRAWN")]
    PRIVILEGEWITHDRAWN,
    

    /// The authority which determines appropriate attributes for a Certificate may have been compromised.
    ///
    /// "ATTRIBUTE_AUTHORITY_COMPROMISE"
    #[serde(rename="ATTRIBUTE_AUTHORITY_COMPROMISE")]
    ATTRIBUTEAUTHORITYCOMPROMISE,
}

impl AsRef<str> for RevokedCertificateRevocationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RevokedCertificateRevocationReasonEnum::REVOCATIONREASONUNSPECIFIED => "REVOCATION_REASON_UNSPECIFIED",
            RevokedCertificateRevocationReasonEnum::KEYCOMPROMISE => "KEY_COMPROMISE",
            RevokedCertificateRevocationReasonEnum::CERTIFICATEAUTHORITYCOMPROMISE => "CERTIFICATE_AUTHORITY_COMPROMISE",
            RevokedCertificateRevocationReasonEnum::AFFILIATIONCHANGED => "AFFILIATION_CHANGED",
            RevokedCertificateRevocationReasonEnum::SUPERSEDED => "SUPERSEDED",
            RevokedCertificateRevocationReasonEnum::CESSATIONOFOPERATION => "CESSATION_OF_OPERATION",
            RevokedCertificateRevocationReasonEnum::CERTIFICATEHOLD => "CERTIFICATE_HOLD",
            RevokedCertificateRevocationReasonEnum::PRIVILEGEWITHDRAWN => "PRIVILEGE_WITHDRAWN",
            RevokedCertificateRevocationReasonEnum::ATTRIBUTEAUTHORITYCOMPROMISE => "ATTRIBUTE_AUTHORITY_COMPROMISE",
        }
    }
}

impl std::convert::TryFrom< &str> for RevokedCertificateRevocationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVOCATION_REASON_UNSPECIFIED" => Ok(RevokedCertificateRevocationReasonEnum::REVOCATIONREASONUNSPECIFIED),
           "KEY_COMPROMISE" => Ok(RevokedCertificateRevocationReasonEnum::KEYCOMPROMISE),
           "CERTIFICATE_AUTHORITY_COMPROMISE" => Ok(RevokedCertificateRevocationReasonEnum::CERTIFICATEAUTHORITYCOMPROMISE),
           "AFFILIATION_CHANGED" => Ok(RevokedCertificateRevocationReasonEnum::AFFILIATIONCHANGED),
           "SUPERSEDED" => Ok(RevokedCertificateRevocationReasonEnum::SUPERSEDED),
           "CESSATION_OF_OPERATION" => Ok(RevokedCertificateRevocationReasonEnum::CESSATIONOFOPERATION),
           "CERTIFICATE_HOLD" => Ok(RevokedCertificateRevocationReasonEnum::CERTIFICATEHOLD),
           "PRIVILEGE_WITHDRAWN" => Ok(RevokedCertificateRevocationReasonEnum::PRIVILEGEWITHDRAWN),
           "ATTRIBUTE_AUTHORITY_COMPROMISE" => Ok(RevokedCertificateRevocationReasonEnum::ATTRIBUTEAUTHORITYCOMPROMISE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RevokedCertificateRevocationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


