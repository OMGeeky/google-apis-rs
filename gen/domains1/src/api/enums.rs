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


// region ConfigureContactSettingsRequestContactNoticesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of contact notices that the caller acknowledges. The notices needed here depend on the values specified in `contact_settings`.
pub enum ConfigureContactSettingsRequestContactNoticesEnum {
    

    /// The notice is undefined.
    ///
    /// "CONTACT_NOTICE_UNSPECIFIED"
    #[serde(rename="CONTACT_NOTICE_UNSPECIFIED")]
    CONTACTNOTICEUNSPECIFIED,
    

    /// Required when setting the `privacy` field of `ContactSettings` to `PUBLIC_CONTACT_DATA`, which exposes contact data publicly.
    ///
    /// "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"
    #[serde(rename="PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT")]
    PUBLICCONTACTDATAACKNOWLEDGEMENT,
}

impl AsRef<str> for ConfigureContactSettingsRequestContactNoticesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConfigureContactSettingsRequestContactNoticesEnum::CONTACTNOTICEUNSPECIFIED => "CONTACT_NOTICE_UNSPECIFIED",
            ConfigureContactSettingsRequestContactNoticesEnum::PUBLICCONTACTDATAACKNOWLEDGEMENT => "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for ConfigureContactSettingsRequestContactNoticesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_NOTICE_UNSPECIFIED" => Ok(ConfigureContactSettingsRequestContactNoticesEnum::CONTACTNOTICEUNSPECIFIED),
           "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT" => Ok(ConfigureContactSettingsRequestContactNoticesEnum::PUBLICCONTACTDATAACKNOWLEDGEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConfigureContactSettingsRequestContactNoticesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContactSettingPrivacyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Privacy setting for the contacts associated with the `Registration`.
pub enum ContactSettingPrivacyEnum {
    

    /// The contact privacy settings are undefined.
    ///
    /// "CONTACT_PRIVACY_UNSPECIFIED"
    #[serde(rename="CONTACT_PRIVACY_UNSPECIFIED")]
    CONTACTPRIVACYUNSPECIFIED,
    

    /// All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request.
    ///
    /// "PUBLIC_CONTACT_DATA"
    #[serde(rename="PUBLIC_CONTACT_DATA")]
    PUBLICCONTACTDATA,
    

    /// Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost.
    ///
    /// "PRIVATE_CONTACT_DATA"
    #[serde(rename="PRIVATE_CONTACT_DATA")]
    PRIVATECONTACTDATA,
    

    /// The organization name (if provided) and limited non-identifying data from `ContactSettings` is available to the public (e.g. country and state). The remaining data is marked as `REDACTED FOR PRIVACY` in the WHOIS database. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242).
    ///
    /// "REDACTED_CONTACT_DATA"
    #[serde(rename="REDACTED_CONTACT_DATA")]
    REDACTEDCONTACTDATA,
}

impl AsRef<str> for ContactSettingPrivacyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContactSettingPrivacyEnum::CONTACTPRIVACYUNSPECIFIED => "CONTACT_PRIVACY_UNSPECIFIED",
            ContactSettingPrivacyEnum::PUBLICCONTACTDATA => "PUBLIC_CONTACT_DATA",
            ContactSettingPrivacyEnum::PRIVATECONTACTDATA => "PRIVATE_CONTACT_DATA",
            ContactSettingPrivacyEnum::REDACTEDCONTACTDATA => "REDACTED_CONTACT_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for ContactSettingPrivacyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_PRIVACY_UNSPECIFIED" => Ok(ContactSettingPrivacyEnum::CONTACTPRIVACYUNSPECIFIED),
           "PUBLIC_CONTACT_DATA" => Ok(ContactSettingPrivacyEnum::PUBLICCONTACTDATA),
           "PRIVATE_CONTACT_DATA" => Ok(ContactSettingPrivacyEnum::PRIVATECONTACTDATA),
           "REDACTED_CONTACT_DATA" => Ok(ContactSettingPrivacyEnum::REDACTEDCONTACTDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContactSettingPrivacyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainResourceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of this domain as a `Registration` resource.
pub enum DomainResourceStateEnum {
    

    /// The assessment is undefined.
    ///
    /// "RESOURCE_STATE_UNSPECIFIED"
    #[serde(rename="RESOURCE_STATE_UNSPECIFIED")]
    RESOURCESTATEUNSPECIFIED,
    

    /// A `Registration` resource can be created for this domain by calling `ImportDomain`.
    ///
    /// "IMPORTABLE"
    #[serde(rename="IMPORTABLE")]
    IMPORTABLE,
    

    /// A `Registration` resource cannot be created for this domain because it is not supported by Cloud Domains; for example, the top-level domain is not supported or the registry charges non-standard pricing for yearly renewals.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// A `Registration` resource cannot be created for this domain because it is suspended and needs to be resolved with Google Domains.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// A `Registration` resource cannot be created for this domain because it is expired and needs to be renewed with Google Domains.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// A `Registration` resource cannot be created for this domain because it is deleted, but it may be possible to restore it with Google Domains.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for DomainResourceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainResourceStateEnum::RESOURCESTATEUNSPECIFIED => "RESOURCE_STATE_UNSPECIFIED",
            DomainResourceStateEnum::IMPORTABLE => "IMPORTABLE",
            DomainResourceStateEnum::UNSUPPORTED => "UNSUPPORTED",
            DomainResourceStateEnum::SUSPENDED => "SUSPENDED",
            DomainResourceStateEnum::EXPIRED => "EXPIRED",
            DomainResourceStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainResourceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_STATE_UNSPECIFIED" => Ok(DomainResourceStateEnum::RESOURCESTATEUNSPECIFIED),
           "IMPORTABLE" => Ok(DomainResourceStateEnum::IMPORTABLE),
           "UNSUPPORTED" => Ok(DomainResourceStateEnum::UNSUPPORTED),
           "SUSPENDED" => Ok(DomainResourceStateEnum::SUSPENDED),
           "EXPIRED" => Ok(DomainResourceStateEnum::EXPIRED),
           "DELETED" => Ok(DomainResourceStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainResourceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DsRecordAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The algorithm used to generate the referenced DNSKEY.
pub enum DsRecordAlgorithmEnum {
    

    /// The algorithm is unspecified.
    ///
    /// "ALGORITHM_UNSPECIFIED"
    #[serde(rename="ALGORITHM_UNSPECIFIED")]
    ALGORITHMUNSPECIFIED,
    

    /// RSA/MD5. Cannot be used for new deployments.
    ///
    /// "RSAMD5"
    #[serde(rename="RSAMD5")]
    RSAMD5,
    

    /// Diffie-Hellman. Cannot be used for new deployments.
    ///
    /// "DH"
    #[serde(rename="DH")]
    DH,
    

    /// DSA/SHA1. Not recommended for new deployments.
    ///
    /// "DSA"
    #[serde(rename="DSA")]
    DSA,
    

    /// ECC. Not recommended for new deployments.
    ///
    /// "ECC"
    #[serde(rename="ECC")]
    ECC,
    

    /// RSA/SHA-1. Not recommended for new deployments.
    ///
    /// "RSASHA1"
    #[serde(rename="RSASHA1")]
    RSASHA1,
    

    /// DSA-NSEC3-SHA1. Not recommended for new deployments.
    ///
    /// "DSANSEC3SHA1"
    #[serde(rename="DSANSEC3SHA1")]
    DSANSEC3SHA1,
    

    /// RSA/SHA1-NSEC3-SHA1. Not recommended for new deployments.
    ///
    /// "RSASHA1NSEC3SHA1"
    #[serde(rename="RSASHA1NSEC3SHA1")]
    RSASHA1NSEC3SHA1,
    

    /// RSA/SHA-256.
    ///
    /// "RSASHA256"
    #[serde(rename="RSASHA256")]
    RSASHA256,
    

    /// RSA/SHA-512.
    ///
    /// "RSASHA512"
    #[serde(rename="RSASHA512")]
    RSASHA512,
    

    /// GOST R 34.10-2001.
    ///
    /// "ECCGOST"
    #[serde(rename="ECCGOST")]
    ECCGOST,
    

    /// ECDSA Curve P-256 with SHA-256.
    ///
    /// "ECDSAP256SHA256"
    #[serde(rename="ECDSAP256SHA256")]
    ECDSAP256SHA256,
    

    /// ECDSA Curve P-384 with SHA-384.
    ///
    /// "ECDSAP384SHA384"
    #[serde(rename="ECDSAP384SHA384")]
    ECDSAP384SHA384,
    

    /// Ed25519.
    ///
    /// "ED25519"
    #[serde(rename="ED25519")]
    ED25519,
    

    /// Ed448.
    ///
    /// "ED448"
    #[serde(rename="ED448")]
    ED448,
    

    /// Reserved for Indirect Keys. Cannot be used for new deployments.
    ///
    /// "INDIRECT"
    #[serde(rename="INDIRECT")]
    INDIRECT,
    

    /// Private algorithm. Cannot be used for new deployments.
    ///
    /// "PRIVATEDNS"
    #[serde(rename="PRIVATEDNS")]
    PRIVATEDNS,
    

    /// Private algorithm OID. Cannot be used for new deployments.
    ///
    /// "PRIVATEOID"
    #[serde(rename="PRIVATEOID")]
    PRIVATEOID,
}

impl AsRef<str> for DsRecordAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DsRecordAlgorithmEnum::ALGORITHMUNSPECIFIED => "ALGORITHM_UNSPECIFIED",
            DsRecordAlgorithmEnum::RSAMD5 => "RSAMD5",
            DsRecordAlgorithmEnum::DH => "DH",
            DsRecordAlgorithmEnum::DSA => "DSA",
            DsRecordAlgorithmEnum::ECC => "ECC",
            DsRecordAlgorithmEnum::RSASHA1 => "RSASHA1",
            DsRecordAlgorithmEnum::DSANSEC3SHA1 => "DSANSEC3SHA1",
            DsRecordAlgorithmEnum::RSASHA1NSEC3SHA1 => "RSASHA1NSEC3SHA1",
            DsRecordAlgorithmEnum::RSASHA256 => "RSASHA256",
            DsRecordAlgorithmEnum::RSASHA512 => "RSASHA512",
            DsRecordAlgorithmEnum::ECCGOST => "ECCGOST",
            DsRecordAlgorithmEnum::ECDSAP256SHA256 => "ECDSAP256SHA256",
            DsRecordAlgorithmEnum::ECDSAP384SHA384 => "ECDSAP384SHA384",
            DsRecordAlgorithmEnum::ED25519 => "ED25519",
            DsRecordAlgorithmEnum::ED448 => "ED448",
            DsRecordAlgorithmEnum::INDIRECT => "INDIRECT",
            DsRecordAlgorithmEnum::PRIVATEDNS => "PRIVATEDNS",
            DsRecordAlgorithmEnum::PRIVATEOID => "PRIVATEOID",
        }
    }
}

impl std::convert::TryFrom< &str> for DsRecordAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALGORITHM_UNSPECIFIED" => Ok(DsRecordAlgorithmEnum::ALGORITHMUNSPECIFIED),
           "RSAMD5" => Ok(DsRecordAlgorithmEnum::RSAMD5),
           "DH" => Ok(DsRecordAlgorithmEnum::DH),
           "DSA" => Ok(DsRecordAlgorithmEnum::DSA),
           "ECC" => Ok(DsRecordAlgorithmEnum::ECC),
           "RSASHA1" => Ok(DsRecordAlgorithmEnum::RSASHA1),
           "DSANSEC3SHA1" => Ok(DsRecordAlgorithmEnum::DSANSEC3SHA1),
           "RSASHA1NSEC3SHA1" => Ok(DsRecordAlgorithmEnum::RSASHA1NSEC3SHA1),
           "RSASHA256" => Ok(DsRecordAlgorithmEnum::RSASHA256),
           "RSASHA512" => Ok(DsRecordAlgorithmEnum::RSASHA512),
           "ECCGOST" => Ok(DsRecordAlgorithmEnum::ECCGOST),
           "ECDSAP256SHA256" => Ok(DsRecordAlgorithmEnum::ECDSAP256SHA256),
           "ECDSAP384SHA384" => Ok(DsRecordAlgorithmEnum::ECDSAP384SHA384),
           "ED25519" => Ok(DsRecordAlgorithmEnum::ED25519),
           "ED448" => Ok(DsRecordAlgorithmEnum::ED448),
           "INDIRECT" => Ok(DsRecordAlgorithmEnum::INDIRECT),
           "PRIVATEDNS" => Ok(DsRecordAlgorithmEnum::PRIVATEDNS),
           "PRIVATEOID" => Ok(DsRecordAlgorithmEnum::PRIVATEOID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DsRecordAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DsRecordDigestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The hash function used to generate the digest of the referenced DNSKEY.
pub enum DsRecordDigestTypeEnum {
    

    /// The DigestType is unspecified.
    ///
    /// "DIGEST_TYPE_UNSPECIFIED"
    #[serde(rename="DIGEST_TYPE_UNSPECIFIED")]
    DIGESTTYPEUNSPECIFIED,
    

    /// SHA-1. Not recommended for new deployments.
    ///
    /// "SHA1"
    #[serde(rename="SHA1")]
    SHA1,
    

    /// SHA-256.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// GOST R 34.11-94.
    ///
    /// "GOST3411"
    #[serde(rename="GOST3411")]
    GOST3411,
    

    /// SHA-384.
    ///
    /// "SHA384"
    #[serde(rename="SHA384")]
    SHA384,
}

impl AsRef<str> for DsRecordDigestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DsRecordDigestTypeEnum::DIGESTTYPEUNSPECIFIED => "DIGEST_TYPE_UNSPECIFIED",
            DsRecordDigestTypeEnum::SHA1 => "SHA1",
            DsRecordDigestTypeEnum::SHA256 => "SHA256",
            DsRecordDigestTypeEnum::GOST3411 => "GOST3411",
            DsRecordDigestTypeEnum::SHA384 => "SHA384",
        }
    }
}

impl std::convert::TryFrom< &str> for DsRecordDigestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIGEST_TYPE_UNSPECIFIED" => Ok(DsRecordDigestTypeEnum::DIGESTTYPEUNSPECIFIED),
           "SHA1" => Ok(DsRecordDigestTypeEnum::SHA1),
           "SHA256" => Ok(DsRecordDigestTypeEnum::SHA256),
           "GOST3411" => Ok(DsRecordDigestTypeEnum::GOST3411),
           "SHA384" => Ok(DsRecordDigestTypeEnum::SHA384),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DsRecordDigestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleDomainsDnDsStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of DS records for this domain. Used to enable or disable automatic DNSSEC.
pub enum GoogleDomainsDnDsStateEnum {
    

    /// DS state is unspecified.
    ///
    /// "DS_STATE_UNSPECIFIED"
    #[serde(rename="DS_STATE_UNSPECIFIED")]
    DSSTATEUNSPECIFIED,
    

    /// DNSSEC is disabled for this domain. No DS records for this domain are published in the parent DNS zone.
    ///
    /// "DS_RECORDS_UNPUBLISHED"
    #[serde(rename="DS_RECORDS_UNPUBLISHED")]
    DSRECORDSUNPUBLISHED,
    

    /// DNSSEC is enabled for this domain. Appropriate DS records for this domain are published in the parent DNS zone. This option is valid only if the DNS zone referenced in the `Registration`'s `dns_provider` field is already DNSSEC-signed.
    ///
    /// "DS_RECORDS_PUBLISHED"
    #[serde(rename="DS_RECORDS_PUBLISHED")]
    DSRECORDSPUBLISHED,
}

impl AsRef<str> for GoogleDomainsDnDsStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleDomainsDnDsStateEnum::DSSTATEUNSPECIFIED => "DS_STATE_UNSPECIFIED",
            GoogleDomainsDnDsStateEnum::DSRECORDSUNPUBLISHED => "DS_RECORDS_UNPUBLISHED",
            GoogleDomainsDnDsStateEnum::DSRECORDSPUBLISHED => "DS_RECORDS_PUBLISHED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleDomainsDnDsStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DS_STATE_UNSPECIFIED" => Ok(GoogleDomainsDnDsStateEnum::DSSTATEUNSPECIFIED),
           "DS_RECORDS_UNPUBLISHED" => Ok(GoogleDomainsDnDsStateEnum::DSRECORDSUNPUBLISHED),
           "DS_RECORDS_PUBLISHED" => Ok(GoogleDomainsDnDsStateEnum::DSRECORDSPUBLISHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleDomainsDnDsStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagementSettingPreferredRenewalMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The desired renewal method for this `Registration`. The actual `renewal_method` is automatically updated to reflect this choice. If unset or equal to `RENEWAL_METHOD_UNSPECIFIED`, the actual `renewalMethod` is treated as if it were set to `AUTOMATIC_RENEWAL`. You cannot use `RENEWAL_DISABLED` during resource creation, and you can update the renewal status only when the `Registration` resource has state `ACTIVE` or `SUSPENDED`. When `preferred_renewal_method` is set to `AUTOMATIC_RENEWAL`, the actual `renewal_method` can be set to `RENEWAL_DISABLED` in case of problems with the billing account or reported domain abuse. In such cases, check the `issues` field on the `Registration`. After the problem is resolved, the `renewal_method` is automatically updated to `preferred_renewal_method` in a few hours.
pub enum ManagementSettingPreferredRenewalMethodEnum {
    

    /// The renewal method is undefined.
    ///
    /// "RENEWAL_METHOD_UNSPECIFIED"
    #[serde(rename="RENEWAL_METHOD_UNSPECIFIED")]
    RENEWALMETHODUNSPECIFIED,
    

    /// The domain is automatically renewed each year.
    ///
    /// "AUTOMATIC_RENEWAL"
    #[serde(rename="AUTOMATIC_RENEWAL")]
    AUTOMATICRENEWAL,
    

    /// Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). This option was never used. Use `RENEWAL_DISABLED` instead.
    ///
    /// "MANUAL_RENEWAL"
    #[serde(rename="MANUAL_RENEWAL")]
    MANUALRENEWAL,
    

    /// The domain won't be renewed and will expire at its expiration time.
    ///
    /// "RENEWAL_DISABLED"
    #[serde(rename="RENEWAL_DISABLED")]
    RENEWALDISABLED,
}

impl AsRef<str> for ManagementSettingPreferredRenewalMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagementSettingPreferredRenewalMethodEnum::RENEWALMETHODUNSPECIFIED => "RENEWAL_METHOD_UNSPECIFIED",
            ManagementSettingPreferredRenewalMethodEnum::AUTOMATICRENEWAL => "AUTOMATIC_RENEWAL",
            ManagementSettingPreferredRenewalMethodEnum::MANUALRENEWAL => "MANUAL_RENEWAL",
            ManagementSettingPreferredRenewalMethodEnum::RENEWALDISABLED => "RENEWAL_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagementSettingPreferredRenewalMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RENEWAL_METHOD_UNSPECIFIED" => Ok(ManagementSettingPreferredRenewalMethodEnum::RENEWALMETHODUNSPECIFIED),
           "AUTOMATIC_RENEWAL" => Ok(ManagementSettingPreferredRenewalMethodEnum::AUTOMATICRENEWAL),
           "MANUAL_RENEWAL" => Ok(ManagementSettingPreferredRenewalMethodEnum::MANUALRENEWAL),
           "RENEWAL_DISABLED" => Ok(ManagementSettingPreferredRenewalMethodEnum::RENEWALDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagementSettingPreferredRenewalMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagementSettingRenewalMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The actual renewal method for this `Registration`. When `preferred_renewal_method` is set to `AUTOMATIC_RENEWAL`, the actual `renewal_method` can be equal to `RENEWAL_DISABLED`—for example, when there are problems with the billing account or reported domain abuse. In such cases, check the `issues` field on the `Registration`. After the problem is resolved, the `renewal_method` is automatically updated to `preferred_renewal_method` in a few hours.
pub enum ManagementSettingRenewalMethodEnum {
    

    /// The renewal method is undefined.
    ///
    /// "RENEWAL_METHOD_UNSPECIFIED"
    #[serde(rename="RENEWAL_METHOD_UNSPECIFIED")]
    RENEWALMETHODUNSPECIFIED,
    

    /// The domain is automatically renewed each year.
    ///
    /// "AUTOMATIC_RENEWAL"
    #[serde(rename="AUTOMATIC_RENEWAL")]
    AUTOMATICRENEWAL,
    

    /// Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). This option was never used. Use `RENEWAL_DISABLED` instead.
    ///
    /// "MANUAL_RENEWAL"
    #[serde(rename="MANUAL_RENEWAL")]
    MANUALRENEWAL,
    

    /// The domain won't be renewed and will expire at its expiration time.
    ///
    /// "RENEWAL_DISABLED"
    #[serde(rename="RENEWAL_DISABLED")]
    RENEWALDISABLED,
}

impl AsRef<str> for ManagementSettingRenewalMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagementSettingRenewalMethodEnum::RENEWALMETHODUNSPECIFIED => "RENEWAL_METHOD_UNSPECIFIED",
            ManagementSettingRenewalMethodEnum::AUTOMATICRENEWAL => "AUTOMATIC_RENEWAL",
            ManagementSettingRenewalMethodEnum::MANUALRENEWAL => "MANUAL_RENEWAL",
            ManagementSettingRenewalMethodEnum::RENEWALDISABLED => "RENEWAL_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagementSettingRenewalMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RENEWAL_METHOD_UNSPECIFIED" => Ok(ManagementSettingRenewalMethodEnum::RENEWALMETHODUNSPECIFIED),
           "AUTOMATIC_RENEWAL" => Ok(ManagementSettingRenewalMethodEnum::AUTOMATICRENEWAL),
           "MANUAL_RENEWAL" => Ok(ManagementSettingRenewalMethodEnum::MANUALRENEWAL),
           "RENEWAL_DISABLED" => Ok(ManagementSettingRenewalMethodEnum::RENEWALDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagementSettingRenewalMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagementSettingTransferLockStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether the domain can be transferred to another registrar.
pub enum ManagementSettingTransferLockStateEnum {
    

    /// The state is unspecified.
    ///
    /// "TRANSFER_LOCK_STATE_UNSPECIFIED"
    #[serde(rename="TRANSFER_LOCK_STATE_UNSPECIFIED")]
    TRANSFERLOCKSTATEUNSPECIFIED,
    

    /// The domain is unlocked and can be transferred to another registrar.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
    

    /// The domain is locked and cannot be transferred to another registrar.
    ///
    /// "LOCKED"
    #[serde(rename="LOCKED")]
    LOCKED,
}

impl AsRef<str> for ManagementSettingTransferLockStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagementSettingTransferLockStateEnum::TRANSFERLOCKSTATEUNSPECIFIED => "TRANSFER_LOCK_STATE_UNSPECIFIED",
            ManagementSettingTransferLockStateEnum::UNLOCKED => "UNLOCKED",
            ManagementSettingTransferLockStateEnum::LOCKED => "LOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagementSettingTransferLockStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_LOCK_STATE_UNSPECIFIED" => Ok(ManagementSettingTransferLockStateEnum::TRANSFERLOCKSTATEUNSPECIFIED),
           "UNLOCKED" => Ok(ManagementSettingTransferLockStateEnum::UNLOCKED),
           "LOCKED" => Ok(ManagementSettingTransferLockStateEnum::LOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagementSettingTransferLockStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegisterDomainRequestContactNoticesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of contact notices that the caller acknowledges. The notices needed here depend on the values specified in `registration.contact_settings`.
pub enum RegisterDomainRequestContactNoticesEnum {
    

    /// The notice is undefined.
    ///
    /// "CONTACT_NOTICE_UNSPECIFIED"
    #[serde(rename="CONTACT_NOTICE_UNSPECIFIED")]
    CONTACTNOTICEUNSPECIFIED,
    

    /// Required when setting the `privacy` field of `ContactSettings` to `PUBLIC_CONTACT_DATA`, which exposes contact data publicly.
    ///
    /// "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"
    #[serde(rename="PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT")]
    PUBLICCONTACTDATAACKNOWLEDGEMENT,
}

impl AsRef<str> for RegisterDomainRequestContactNoticesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegisterDomainRequestContactNoticesEnum::CONTACTNOTICEUNSPECIFIED => "CONTACT_NOTICE_UNSPECIFIED",
            RegisterDomainRequestContactNoticesEnum::PUBLICCONTACTDATAACKNOWLEDGEMENT => "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for RegisterDomainRequestContactNoticesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_NOTICE_UNSPECIFIED" => Ok(RegisterDomainRequestContactNoticesEnum::CONTACTNOTICEUNSPECIFIED),
           "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT" => Ok(RegisterDomainRequestContactNoticesEnum::PUBLICCONTACTDATAACKNOWLEDGEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegisterDomainRequestContactNoticesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegisterDomainRequestDomainNoticesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of domain notices that you acknowledge. Call `RetrieveRegisterParameters` to see the notices that need acknowledgement.
pub enum RegisterDomainRequestDomainNoticesEnum {
    

    /// The notice is undefined.
    ///
    /// "DOMAIN_NOTICE_UNSPECIFIED"
    #[serde(rename="DOMAIN_NOTICE_UNSPECIFIED")]
    DOMAINNOTICEUNSPECIFIED,
    

    /// Indicates that the domain is preloaded on the HTTP Strict Transport Security list in browsers. Serving a website on such domain requires an SSL certificate. For details, see [how to get an SSL certificate](https://support.google.com/domains/answer/7638036).
    ///
    /// "HSTS_PRELOADED"
    #[serde(rename="HSTS_PRELOADED")]
    HSTSPRELOADED,
}

impl AsRef<str> for RegisterDomainRequestDomainNoticesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegisterDomainRequestDomainNoticesEnum::DOMAINNOTICEUNSPECIFIED => "DOMAIN_NOTICE_UNSPECIFIED",
            RegisterDomainRequestDomainNoticesEnum::HSTSPRELOADED => "HSTS_PRELOADED",
        }
    }
}

impl std::convert::TryFrom< &str> for RegisterDomainRequestDomainNoticesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOMAIN_NOTICE_UNSPECIFIED" => Ok(RegisterDomainRequestDomainNoticesEnum::DOMAINNOTICEUNSPECIFIED),
           "HSTS_PRELOADED" => Ok(RegisterDomainRequestDomainNoticesEnum::HSTSPRELOADED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegisterDomainRequestDomainNoticesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegisterParameterAvailabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the domain is available for registration. This value is accurate when obtained by calling `RetrieveRegisterParameters`, but is approximate when obtained by calling `SearchDomains`.
pub enum RegisterParameterAvailabilityEnum {
    

    /// The availability is unspecified.
    ///
    /// "AVAILABILITY_UNSPECIFIED"
    #[serde(rename="AVAILABILITY_UNSPECIFIED")]
    AVAILABILITYUNSPECIFIED,
    

    /// The domain is available for registration.
    ///
    /// "AVAILABLE"
    #[serde(rename="AVAILABLE")]
    AVAILABLE,
    

    /// The domain is not available for registration. Generally this means it is already registered to another party.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// The domain is not currently supported by Cloud Domains, but may be available elsewhere.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// Cloud Domains is unable to determine domain availability, generally due to system maintenance at the domain name registry.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
}

impl AsRef<str> for RegisterParameterAvailabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegisterParameterAvailabilityEnum::AVAILABILITYUNSPECIFIED => "AVAILABILITY_UNSPECIFIED",
            RegisterParameterAvailabilityEnum::AVAILABLE => "AVAILABLE",
            RegisterParameterAvailabilityEnum::UNAVAILABLE => "UNAVAILABLE",
            RegisterParameterAvailabilityEnum::UNSUPPORTED => "UNSUPPORTED",
            RegisterParameterAvailabilityEnum::UNKNOWN => "UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for RegisterParameterAvailabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AVAILABILITY_UNSPECIFIED" => Ok(RegisterParameterAvailabilityEnum::AVAILABILITYUNSPECIFIED),
           "AVAILABLE" => Ok(RegisterParameterAvailabilityEnum::AVAILABLE),
           "UNAVAILABLE" => Ok(RegisterParameterAvailabilityEnum::UNAVAILABLE),
           "UNSUPPORTED" => Ok(RegisterParameterAvailabilityEnum::UNSUPPORTED),
           "UNKNOWN" => Ok(RegisterParameterAvailabilityEnum::UNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegisterParameterAvailabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegisterParameterDomainNoticesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Notices about special properties of the domain.
pub enum RegisterParameterDomainNoticesEnum {
    

    /// The notice is undefined.
    ///
    /// "DOMAIN_NOTICE_UNSPECIFIED"
    #[serde(rename="DOMAIN_NOTICE_UNSPECIFIED")]
    DOMAINNOTICEUNSPECIFIED,
    

    /// Indicates that the domain is preloaded on the HTTP Strict Transport Security list in browsers. Serving a website on such domain requires an SSL certificate. For details, see [how to get an SSL certificate](https://support.google.com/domains/answer/7638036).
    ///
    /// "HSTS_PRELOADED"
    #[serde(rename="HSTS_PRELOADED")]
    HSTSPRELOADED,
}

impl AsRef<str> for RegisterParameterDomainNoticesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegisterParameterDomainNoticesEnum::DOMAINNOTICEUNSPECIFIED => "DOMAIN_NOTICE_UNSPECIFIED",
            RegisterParameterDomainNoticesEnum::HSTSPRELOADED => "HSTS_PRELOADED",
        }
    }
}

impl std::convert::TryFrom< &str> for RegisterParameterDomainNoticesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOMAIN_NOTICE_UNSPECIFIED" => Ok(RegisterParameterDomainNoticesEnum::DOMAINNOTICEUNSPECIFIED),
           "HSTS_PRELOADED" => Ok(RegisterParameterDomainNoticesEnum::HSTSPRELOADED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegisterParameterDomainNoticesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegisterParameterSupportedPrivacyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Contact privacy options that the domain supports.
pub enum RegisterParameterSupportedPrivacyEnum {
    

    /// The contact privacy settings are undefined.
    ///
    /// "CONTACT_PRIVACY_UNSPECIFIED"
    #[serde(rename="CONTACT_PRIVACY_UNSPECIFIED")]
    CONTACTPRIVACYUNSPECIFIED,
    

    /// All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request.
    ///
    /// "PUBLIC_CONTACT_DATA"
    #[serde(rename="PUBLIC_CONTACT_DATA")]
    PUBLICCONTACTDATA,
    

    /// Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost.
    ///
    /// "PRIVATE_CONTACT_DATA"
    #[serde(rename="PRIVATE_CONTACT_DATA")]
    PRIVATECONTACTDATA,
    

    /// The organization name (if provided) and limited non-identifying data from `ContactSettings` is available to the public (e.g. country and state). The remaining data is marked as `REDACTED FOR PRIVACY` in the WHOIS database. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242).
    ///
    /// "REDACTED_CONTACT_DATA"
    #[serde(rename="REDACTED_CONTACT_DATA")]
    REDACTEDCONTACTDATA,
}

impl AsRef<str> for RegisterParameterSupportedPrivacyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegisterParameterSupportedPrivacyEnum::CONTACTPRIVACYUNSPECIFIED => "CONTACT_PRIVACY_UNSPECIFIED",
            RegisterParameterSupportedPrivacyEnum::PUBLICCONTACTDATA => "PUBLIC_CONTACT_DATA",
            RegisterParameterSupportedPrivacyEnum::PRIVATECONTACTDATA => "PRIVATE_CONTACT_DATA",
            RegisterParameterSupportedPrivacyEnum::REDACTEDCONTACTDATA => "REDACTED_CONTACT_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for RegisterParameterSupportedPrivacyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_PRIVACY_UNSPECIFIED" => Ok(RegisterParameterSupportedPrivacyEnum::CONTACTPRIVACYUNSPECIFIED),
           "PUBLIC_CONTACT_DATA" => Ok(RegisterParameterSupportedPrivacyEnum::PUBLICCONTACTDATA),
           "PRIVATE_CONTACT_DATA" => Ok(RegisterParameterSupportedPrivacyEnum::PRIVATECONTACTDATA),
           "REDACTED_CONTACT_DATA" => Ok(RegisterParameterSupportedPrivacyEnum::REDACTEDCONTACTDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegisterParameterSupportedPrivacyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegistrationIssuesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The set of issues with the `Registration` that require attention.
pub enum RegistrationIssuesEnum {
    

    /// The issue is undefined.
    ///
    /// "ISSUE_UNSPECIFIED"
    #[serde(rename="ISSUE_UNSPECIFIED")]
    ISSUEUNSPECIFIED,
    

    /// Contact the Cloud Support team to resolve a problem with this domain.
    ///
    /// "CONTACT_SUPPORT"
    #[serde(rename="CONTACT_SUPPORT")]
    CONTACTSUPPORT,
    

    /// [ICANN](https://icann.org/) requires verification of the email address in the `Registration`'s `contact_settings.registrant_contact` field. To verify the email address, follow the instructions in the email the `registrant_contact` receives following registration. If you do not complete email verification within 15 days of registration, the domain is suspended. To resend the verification email, call ConfigureContactSettings and provide the current `registrant_contact.email`.
    ///
    /// "UNVERIFIED_EMAIL"
    #[serde(rename="UNVERIFIED_EMAIL")]
    UNVERIFIEDEMAIL,
    

    /// The billing account is not in good standing. The domain is not automatically renewed at its expiration time unless you resolve problems with your billing account.
    ///
    /// "PROBLEM_WITH_BILLING"
    #[serde(rename="PROBLEM_WITH_BILLING")]
    PROBLEMWITHBILLING,
}

impl AsRef<str> for RegistrationIssuesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegistrationIssuesEnum::ISSUEUNSPECIFIED => "ISSUE_UNSPECIFIED",
            RegistrationIssuesEnum::CONTACTSUPPORT => "CONTACT_SUPPORT",
            RegistrationIssuesEnum::UNVERIFIEDEMAIL => "UNVERIFIED_EMAIL",
            RegistrationIssuesEnum::PROBLEMWITHBILLING => "PROBLEM_WITH_BILLING",
        }
    }
}

impl std::convert::TryFrom< &str> for RegistrationIssuesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ISSUE_UNSPECIFIED" => Ok(RegistrationIssuesEnum::ISSUEUNSPECIFIED),
           "CONTACT_SUPPORT" => Ok(RegistrationIssuesEnum::CONTACTSUPPORT),
           "UNVERIFIED_EMAIL" => Ok(RegistrationIssuesEnum::UNVERIFIEDEMAIL),
           "PROBLEM_WITH_BILLING" => Ok(RegistrationIssuesEnum::PROBLEMWITHBILLING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegistrationIssuesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegistrationRegisterFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state.
pub enum RegistrationRegisterFailureReasonEnum {
    

    /// Register failure unspecified.
    ///
    /// "REGISTER_FAILURE_REASON_UNSPECIFIED"
    #[serde(rename="REGISTER_FAILURE_REASON_UNSPECIFIED")]
    REGISTERFAILUREREASONUNSPECIFIED,
    

    /// Registration failed for an unknown reason.
    ///
    /// "REGISTER_FAILURE_REASON_UNKNOWN"
    #[serde(rename="REGISTER_FAILURE_REASON_UNKNOWN")]
    REGISTERFAILUREREASONUNKNOWN,
    

    /// The domain is not available for registration.
    ///
    /// "DOMAIN_NOT_AVAILABLE"
    #[serde(rename="DOMAIN_NOT_AVAILABLE")]
    DOMAINNOTAVAILABLE,
    

    /// The provided contact information was rejected.
    ///
    /// "INVALID_CONTACTS"
    #[serde(rename="INVALID_CONTACTS")]
    INVALIDCONTACTS,
}

impl AsRef<str> for RegistrationRegisterFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegistrationRegisterFailureReasonEnum::REGISTERFAILUREREASONUNSPECIFIED => "REGISTER_FAILURE_REASON_UNSPECIFIED",
            RegistrationRegisterFailureReasonEnum::REGISTERFAILUREREASONUNKNOWN => "REGISTER_FAILURE_REASON_UNKNOWN",
            RegistrationRegisterFailureReasonEnum::DOMAINNOTAVAILABLE => "DOMAIN_NOT_AVAILABLE",
            RegistrationRegisterFailureReasonEnum::INVALIDCONTACTS => "INVALID_CONTACTS",
        }
    }
}

impl std::convert::TryFrom< &str> for RegistrationRegisterFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REGISTER_FAILURE_REASON_UNSPECIFIED" => Ok(RegistrationRegisterFailureReasonEnum::REGISTERFAILUREREASONUNSPECIFIED),
           "REGISTER_FAILURE_REASON_UNKNOWN" => Ok(RegistrationRegisterFailureReasonEnum::REGISTERFAILUREREASONUNKNOWN),
           "DOMAIN_NOT_AVAILABLE" => Ok(RegistrationRegisterFailureReasonEnum::DOMAINNOTAVAILABLE),
           "INVALID_CONTACTS" => Ok(RegistrationRegisterFailureReasonEnum::INVALIDCONTACTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegistrationRegisterFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegistrationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the `Registration`
pub enum RegistrationStateEnum {
    

    /// The state is undefined.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The domain is being registered.
    ///
    /// "REGISTRATION_PENDING"
    #[serde(rename="REGISTRATION_PENDING")]
    REGISTRATIONPENDING,
    

    /// The domain registration failed. You can delete resources in this state to allow registration to be retried.
    ///
    /// "REGISTRATION_FAILED"
    #[serde(rename="REGISTRATION_FAILED")]
    REGISTRATIONFAILED,
    

    /// The domain is being transferred from another registrar to Cloud Domains.
    ///
    /// "TRANSFER_PENDING"
    #[serde(rename="TRANSFER_PENDING")]
    TRANSFERPENDING,
    

    /// The attempt to transfer the domain from another registrar to Cloud Domains failed. You can delete resources in this state and retry the transfer.
    ///
    /// "TRANSFER_FAILED"
    #[serde(rename="TRANSFER_FAILED")]
    TRANSFERFAILED,
    

    /// The domain is being imported from Google Domains to Cloud Domains.
    ///
    /// "IMPORT_PENDING"
    #[serde(rename="IMPORT_PENDING")]
    IMPORTPENDING,
    

    /// The domain is registered and operational. The domain renews automatically as long as it remains in this state and the `RenewalMethod` is set to `AUTOMATIC_RENEWAL`.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The domain is suspended and inoperative. For more details, see the `issues` field.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// The domain is no longer managed with Cloud Domains. It may have been transferred to another registrar or exported for management in [Google Domains](https://domains.google/). You can no longer update it with this API, and information shown about it may be stale. Domains in this state are not automatically renewed by Cloud Domains.
    ///
    /// "EXPORTED"
    #[serde(rename="EXPORTED")]
    EXPORTED,
    

    /// The domain is expired.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
}

impl AsRef<str> for RegistrationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegistrationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RegistrationStateEnum::REGISTRATIONPENDING => "REGISTRATION_PENDING",
            RegistrationStateEnum::REGISTRATIONFAILED => "REGISTRATION_FAILED",
            RegistrationStateEnum::TRANSFERPENDING => "TRANSFER_PENDING",
            RegistrationStateEnum::TRANSFERFAILED => "TRANSFER_FAILED",
            RegistrationStateEnum::IMPORTPENDING => "IMPORT_PENDING",
            RegistrationStateEnum::ACTIVE => "ACTIVE",
            RegistrationStateEnum::SUSPENDED => "SUSPENDED",
            RegistrationStateEnum::EXPORTED => "EXPORTED",
            RegistrationStateEnum::EXPIRED => "EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for RegistrationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RegistrationStateEnum::STATEUNSPECIFIED),
           "REGISTRATION_PENDING" => Ok(RegistrationStateEnum::REGISTRATIONPENDING),
           "REGISTRATION_FAILED" => Ok(RegistrationStateEnum::REGISTRATIONFAILED),
           "TRANSFER_PENDING" => Ok(RegistrationStateEnum::TRANSFERPENDING),
           "TRANSFER_FAILED" => Ok(RegistrationStateEnum::TRANSFERFAILED),
           "IMPORT_PENDING" => Ok(RegistrationStateEnum::IMPORTPENDING),
           "ACTIVE" => Ok(RegistrationStateEnum::ACTIVE),
           "SUSPENDED" => Ok(RegistrationStateEnum::SUSPENDED),
           "EXPORTED" => Ok(RegistrationStateEnum::EXPORTED),
           "EXPIRED" => Ok(RegistrationStateEnum::EXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegistrationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegistrationSupportedPrivacyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports.
pub enum RegistrationSupportedPrivacyEnum {
    

    /// The contact privacy settings are undefined.
    ///
    /// "CONTACT_PRIVACY_UNSPECIFIED"
    #[serde(rename="CONTACT_PRIVACY_UNSPECIFIED")]
    CONTACTPRIVACYUNSPECIFIED,
    

    /// All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request.
    ///
    /// "PUBLIC_CONTACT_DATA"
    #[serde(rename="PUBLIC_CONTACT_DATA")]
    PUBLICCONTACTDATA,
    

    /// Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost.
    ///
    /// "PRIVATE_CONTACT_DATA"
    #[serde(rename="PRIVATE_CONTACT_DATA")]
    PRIVATECONTACTDATA,
    

    /// The organization name (if provided) and limited non-identifying data from `ContactSettings` is available to the public (e.g. country and state). The remaining data is marked as `REDACTED FOR PRIVACY` in the WHOIS database. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242).
    ///
    /// "REDACTED_CONTACT_DATA"
    #[serde(rename="REDACTED_CONTACT_DATA")]
    REDACTEDCONTACTDATA,
}

impl AsRef<str> for RegistrationSupportedPrivacyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegistrationSupportedPrivacyEnum::CONTACTPRIVACYUNSPECIFIED => "CONTACT_PRIVACY_UNSPECIFIED",
            RegistrationSupportedPrivacyEnum::PUBLICCONTACTDATA => "PUBLIC_CONTACT_DATA",
            RegistrationSupportedPrivacyEnum::PRIVATECONTACTDATA => "PRIVATE_CONTACT_DATA",
            RegistrationSupportedPrivacyEnum::REDACTEDCONTACTDATA => "REDACTED_CONTACT_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for RegistrationSupportedPrivacyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_PRIVACY_UNSPECIFIED" => Ok(RegistrationSupportedPrivacyEnum::CONTACTPRIVACYUNSPECIFIED),
           "PUBLIC_CONTACT_DATA" => Ok(RegistrationSupportedPrivacyEnum::PUBLICCONTACTDATA),
           "PRIVATE_CONTACT_DATA" => Ok(RegistrationSupportedPrivacyEnum::PRIVATECONTACTDATA),
           "REDACTED_CONTACT_DATA" => Ok(RegistrationSupportedPrivacyEnum::REDACTEDCONTACTDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegistrationSupportedPrivacyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RegistrationTransferFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state.
pub enum RegistrationTransferFailureReasonEnum {
    

    /// Transfer failure unspecified.
    ///
    /// "TRANSFER_FAILURE_REASON_UNSPECIFIED"
    #[serde(rename="TRANSFER_FAILURE_REASON_UNSPECIFIED")]
    TRANSFERFAILUREREASONUNSPECIFIED,
    

    /// Transfer failed for an unknown reason.
    ///
    /// "TRANSFER_FAILURE_REASON_UNKNOWN"
    #[serde(rename="TRANSFER_FAILURE_REASON_UNKNOWN")]
    TRANSFERFAILUREREASONUNKNOWN,
    

    /// An email confirmation sent to the user was rejected or expired.
    ///
    /// "EMAIL_CONFIRMATION_FAILURE"
    #[serde(rename="EMAIL_CONFIRMATION_FAILURE")]
    EMAILCONFIRMATIONFAILURE,
    

    /// The domain is available for registration.
    ///
    /// "DOMAIN_NOT_REGISTERED"
    #[serde(rename="DOMAIN_NOT_REGISTERED")]
    DOMAINNOTREGISTERED,
    

    /// The domain has a transfer lock with its current registrar which must be removed prior to transfer.
    ///
    /// "DOMAIN_HAS_TRANSFER_LOCK"
    #[serde(rename="DOMAIN_HAS_TRANSFER_LOCK")]
    DOMAINHASTRANSFERLOCK,
    

    /// The authorization code entered is not valid.
    ///
    /// "INVALID_AUTHORIZATION_CODE"
    #[serde(rename="INVALID_AUTHORIZATION_CODE")]
    INVALIDAUTHORIZATIONCODE,
    

    /// The transfer was cancelled by the domain owner, current registrar, or TLD registry.
    ///
    /// "TRANSFER_CANCELLED"
    #[serde(rename="TRANSFER_CANCELLED")]
    TRANSFERCANCELLED,
    

    /// The transfer was rejected by the current registrar. Contact the current registrar for more information.
    ///
    /// "TRANSFER_REJECTED"
    #[serde(rename="TRANSFER_REJECTED")]
    TRANSFERREJECTED,
    

    /// The registrant email address cannot be parsed from the domain's current public contact data.
    ///
    /// "INVALID_REGISTRANT_EMAIL_ADDRESS"
    #[serde(rename="INVALID_REGISTRANT_EMAIL_ADDRESS")]
    INVALIDREGISTRANTEMAILADDRESS,
    

    /// The domain is not eligible for transfer due requirements imposed by the current registrar or TLD registry.
    ///
    /// "DOMAIN_NOT_ELIGIBLE_FOR_TRANSFER"
    #[serde(rename="DOMAIN_NOT_ELIGIBLE_FOR_TRANSFER")]
    DOMAINNOTELIGIBLEFORTRANSFER,
    

    /// Another transfer is already pending for this domain. The existing transfer attempt must expire or be cancelled in order to proceed.
    ///
    /// "TRANSFER_ALREADY_PENDING"
    #[serde(rename="TRANSFER_ALREADY_PENDING")]
    TRANSFERALREADYPENDING,
}

impl AsRef<str> for RegistrationTransferFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RegistrationTransferFailureReasonEnum::TRANSFERFAILUREREASONUNSPECIFIED => "TRANSFER_FAILURE_REASON_UNSPECIFIED",
            RegistrationTransferFailureReasonEnum::TRANSFERFAILUREREASONUNKNOWN => "TRANSFER_FAILURE_REASON_UNKNOWN",
            RegistrationTransferFailureReasonEnum::EMAILCONFIRMATIONFAILURE => "EMAIL_CONFIRMATION_FAILURE",
            RegistrationTransferFailureReasonEnum::DOMAINNOTREGISTERED => "DOMAIN_NOT_REGISTERED",
            RegistrationTransferFailureReasonEnum::DOMAINHASTRANSFERLOCK => "DOMAIN_HAS_TRANSFER_LOCK",
            RegistrationTransferFailureReasonEnum::INVALIDAUTHORIZATIONCODE => "INVALID_AUTHORIZATION_CODE",
            RegistrationTransferFailureReasonEnum::TRANSFERCANCELLED => "TRANSFER_CANCELLED",
            RegistrationTransferFailureReasonEnum::TRANSFERREJECTED => "TRANSFER_REJECTED",
            RegistrationTransferFailureReasonEnum::INVALIDREGISTRANTEMAILADDRESS => "INVALID_REGISTRANT_EMAIL_ADDRESS",
            RegistrationTransferFailureReasonEnum::DOMAINNOTELIGIBLEFORTRANSFER => "DOMAIN_NOT_ELIGIBLE_FOR_TRANSFER",
            RegistrationTransferFailureReasonEnum::TRANSFERALREADYPENDING => "TRANSFER_ALREADY_PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for RegistrationTransferFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_FAILURE_REASON_UNSPECIFIED" => Ok(RegistrationTransferFailureReasonEnum::TRANSFERFAILUREREASONUNSPECIFIED),
           "TRANSFER_FAILURE_REASON_UNKNOWN" => Ok(RegistrationTransferFailureReasonEnum::TRANSFERFAILUREREASONUNKNOWN),
           "EMAIL_CONFIRMATION_FAILURE" => Ok(RegistrationTransferFailureReasonEnum::EMAILCONFIRMATIONFAILURE),
           "DOMAIN_NOT_REGISTERED" => Ok(RegistrationTransferFailureReasonEnum::DOMAINNOTREGISTERED),
           "DOMAIN_HAS_TRANSFER_LOCK" => Ok(RegistrationTransferFailureReasonEnum::DOMAINHASTRANSFERLOCK),
           "INVALID_AUTHORIZATION_CODE" => Ok(RegistrationTransferFailureReasonEnum::INVALIDAUTHORIZATIONCODE),
           "TRANSFER_CANCELLED" => Ok(RegistrationTransferFailureReasonEnum::TRANSFERCANCELLED),
           "TRANSFER_REJECTED" => Ok(RegistrationTransferFailureReasonEnum::TRANSFERREJECTED),
           "INVALID_REGISTRANT_EMAIL_ADDRESS" => Ok(RegistrationTransferFailureReasonEnum::INVALIDREGISTRANTEMAILADDRESS),
           "DOMAIN_NOT_ELIGIBLE_FOR_TRANSFER" => Ok(RegistrationTransferFailureReasonEnum::DOMAINNOTELIGIBLEFORTRANSFER),
           "TRANSFER_ALREADY_PENDING" => Ok(RegistrationTransferFailureReasonEnum::TRANSFERALREADYPENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RegistrationTransferFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferDomainRequestContactNoticesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of contact notices that you acknowledge. The notices needed here depend on the values specified in `registration.contact_settings`.
pub enum TransferDomainRequestContactNoticesEnum {
    

    /// The notice is undefined.
    ///
    /// "CONTACT_NOTICE_UNSPECIFIED"
    #[serde(rename="CONTACT_NOTICE_UNSPECIFIED")]
    CONTACTNOTICEUNSPECIFIED,
    

    /// Required when setting the `privacy` field of `ContactSettings` to `PUBLIC_CONTACT_DATA`, which exposes contact data publicly.
    ///
    /// "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"
    #[serde(rename="PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT")]
    PUBLICCONTACTDATAACKNOWLEDGEMENT,
}

impl AsRef<str> for TransferDomainRequestContactNoticesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferDomainRequestContactNoticesEnum::CONTACTNOTICEUNSPECIFIED => "CONTACT_NOTICE_UNSPECIFIED",
            TransferDomainRequestContactNoticesEnum::PUBLICCONTACTDATAACKNOWLEDGEMENT => "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferDomainRequestContactNoticesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_NOTICE_UNSPECIFIED" => Ok(TransferDomainRequestContactNoticesEnum::CONTACTNOTICEUNSPECIFIED),
           "PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT" => Ok(TransferDomainRequestContactNoticesEnum::PUBLICCONTACTDATAACKNOWLEDGEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferDomainRequestContactNoticesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferParameterSupportedPrivacyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Contact privacy options that the domain supports.
pub enum TransferParameterSupportedPrivacyEnum {
    

    /// The contact privacy settings are undefined.
    ///
    /// "CONTACT_PRIVACY_UNSPECIFIED"
    #[serde(rename="CONTACT_PRIVACY_UNSPECIFIED")]
    CONTACTPRIVACYUNSPECIFIED,
    

    /// All the data from `ContactSettings` is publicly available. When setting this option, you must also provide a `PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT` in the `contact_notices` field of the request.
    ///
    /// "PUBLIC_CONTACT_DATA"
    #[serde(rename="PUBLIC_CONTACT_DATA")]
    PUBLICCONTACTDATA,
    

    /// Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). None of the data from `ContactSettings` is publicly available. Instead, proxy contact data is published for your domain. Email sent to the proxy email address is forwarded to the registrant's email address. Cloud Domains provides this privacy proxy service at no additional cost.
    ///
    /// "PRIVATE_CONTACT_DATA"
    #[serde(rename="PRIVATE_CONTACT_DATA")]
    PRIVATECONTACTDATA,
    

    /// The organization name (if provided) and limited non-identifying data from `ContactSettings` is available to the public (e.g. country and state). The remaining data is marked as `REDACTED FOR PRIVACY` in the WHOIS database. The actual information redacted depends on the domain. For details, see [the registration privacy article](https://support.google.com/domains/answer/3251242).
    ///
    /// "REDACTED_CONTACT_DATA"
    #[serde(rename="REDACTED_CONTACT_DATA")]
    REDACTEDCONTACTDATA,
}

impl AsRef<str> for TransferParameterSupportedPrivacyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferParameterSupportedPrivacyEnum::CONTACTPRIVACYUNSPECIFIED => "CONTACT_PRIVACY_UNSPECIFIED",
            TransferParameterSupportedPrivacyEnum::PUBLICCONTACTDATA => "PUBLIC_CONTACT_DATA",
            TransferParameterSupportedPrivacyEnum::PRIVATECONTACTDATA => "PRIVATE_CONTACT_DATA",
            TransferParameterSupportedPrivacyEnum::REDACTEDCONTACTDATA => "REDACTED_CONTACT_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferParameterSupportedPrivacyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTACT_PRIVACY_UNSPECIFIED" => Ok(TransferParameterSupportedPrivacyEnum::CONTACTPRIVACYUNSPECIFIED),
           "PUBLIC_CONTACT_DATA" => Ok(TransferParameterSupportedPrivacyEnum::PUBLICCONTACTDATA),
           "PRIVATE_CONTACT_DATA" => Ok(TransferParameterSupportedPrivacyEnum::PRIVATECONTACTDATA),
           "REDACTED_CONTACT_DATA" => Ok(TransferParameterSupportedPrivacyEnum::REDACTEDCONTACTDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferParameterSupportedPrivacyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferParameterTransferLockStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the domain is protected by a transfer lock. For a transfer to succeed, this must show `UNLOCKED`. To unlock a domain, go to its current registrar.
pub enum TransferParameterTransferLockStateEnum {
    

    /// The state is unspecified.
    ///
    /// "TRANSFER_LOCK_STATE_UNSPECIFIED"
    #[serde(rename="TRANSFER_LOCK_STATE_UNSPECIFIED")]
    TRANSFERLOCKSTATEUNSPECIFIED,
    

    /// The domain is unlocked and can be transferred to another registrar.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
    

    /// The domain is locked and cannot be transferred to another registrar.
    ///
    /// "LOCKED"
    #[serde(rename="LOCKED")]
    LOCKED,
}

impl AsRef<str> for TransferParameterTransferLockStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferParameterTransferLockStateEnum::TRANSFERLOCKSTATEUNSPECIFIED => "TRANSFER_LOCK_STATE_UNSPECIFIED",
            TransferParameterTransferLockStateEnum::UNLOCKED => "UNLOCKED",
            TransferParameterTransferLockStateEnum::LOCKED => "LOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferParameterTransferLockStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_LOCK_STATE_UNSPECIFIED" => Ok(TransferParameterTransferLockStateEnum::TRANSFERLOCKSTATEUNSPECIFIED),
           "UNLOCKED" => Ok(TransferParameterTransferLockStateEnum::UNLOCKED),
           "LOCKED" => Ok(TransferParameterTransferLockStateEnum::LOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferParameterTransferLockStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


