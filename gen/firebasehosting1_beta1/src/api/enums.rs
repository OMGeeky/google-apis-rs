use super::*;



// region CertificateStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the certificate. Only the `CERT_ACTIVE` and `CERT_EXPIRING_SOON` states provide SSL coverage for a domain name. If the state is `PROPAGATING` and Hosting had an active cert for the domain name before, that formerly-active cert provides SSL coverage for the domain name until the current cert propagates.
pub enum CertificateStateEnum {
    

    /// The certificate's state is unspecified. The message is invalid if this is unspecified.
    ///
    /// "CERT_STATE_UNSPECIFIED"
    #[serde(rename="CERT_STATE_UNSPECIFIED")]
    CERTSTATEUNSPECIFIED,
    

    /// The initial state of every certificate, represents Hosting's intent to create a certificate, before requests to a Certificate Authority are made.
    ///
    /// "CERT_PREPARING"
    #[serde(rename="CERT_PREPARING")]
    CERTPREPARING,
    

    /// Hosting is validating whether a domain name's DNS records are in a state that allow certificate creation on its behalf.
    ///
    /// "CERT_VALIDATING"
    #[serde(rename="CERT_VALIDATING")]
    CERTVALIDATING,
    

    /// The certificate was recently created, and needs time to propagate in Hosting's CDN.
    ///
    /// "CERT_PROPAGATING"
    #[serde(rename="CERT_PROPAGATING")]
    CERTPROPAGATING,
    

    /// The certificate is active, providing secure connections for the domain names it represents.
    ///
    /// "CERT_ACTIVE"
    #[serde(rename="CERT_ACTIVE")]
    CERTACTIVE,
    

    /// The certificate is expiring, all domain names on it will be given new certificates.
    ///
    /// "CERT_EXPIRING_SOON"
    #[serde(rename="CERT_EXPIRING_SOON")]
    CERTEXPIRINGSOON,
    

    /// The certificate has expired. Hosting can no longer serve secure content on your domain name.
    ///
    /// "CERT_EXPIRED"
    #[serde(rename="CERT_EXPIRED")]
    CERTEXPIRED,
}

impl AsRef<str> for CertificateStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateStateEnum::CERTSTATEUNSPECIFIED => "CERT_STATE_UNSPECIFIED",
            CertificateStateEnum::CERTPREPARING => "CERT_PREPARING",
            CertificateStateEnum::CERTVALIDATING => "CERT_VALIDATING",
            CertificateStateEnum::CERTPROPAGATING => "CERT_PROPAGATING",
            CertificateStateEnum::CERTACTIVE => "CERT_ACTIVE",
            CertificateStateEnum::CERTEXPIRINGSOON => "CERT_EXPIRING_SOON",
            CertificateStateEnum::CERTEXPIRED => "CERT_EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERT_STATE_UNSPECIFIED" => Ok(CertificateStateEnum::CERTSTATEUNSPECIFIED),
           "CERT_PREPARING" => Ok(CertificateStateEnum::CERTPREPARING),
           "CERT_VALIDATING" => Ok(CertificateStateEnum::CERTVALIDATING),
           "CERT_PROPAGATING" => Ok(CertificateStateEnum::CERTPROPAGATING),
           "CERT_ACTIVE" => Ok(CertificateStateEnum::CERTACTIVE),
           "CERT_EXPIRING_SOON" => Ok(CertificateStateEnum::CERTEXPIRINGSOON),
           "CERT_EXPIRED" => Ok(CertificateStateEnum::CERTEXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CertificateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The certificate's type.
pub enum CertificateTypeEnum {
    

    /// The certificate's type is unspecified. The message is invalid if this is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A short-lived certificate type that covers a domain name temporarily, while Hosting creates a more permanent certificate.
    ///
    /// "TEMPORARY"
    #[serde(rename="TEMPORARY")]
    TEMPORARY,
    

    /// The standard certificate for Spark plan custom domains.
    ///
    /// "GROUPED"
    #[serde(rename="GROUPED")]
    GROUPED,
    

    /// Blaze plan only. A certificate that covers from 1 to 100 domain names with custom domains on the same Firebase project.
    ///
    /// "PROJECT_GROUPED"
    #[serde(rename="PROJECT_GROUPED")]
    PROJECTGROUPED,
    

    /// Blaze plan only. A certificate that covers a single domain name.
    ///
    /// "DEDICATED"
    #[serde(rename="DEDICATED")]
    DEDICATED,
}

impl AsRef<str> for CertificateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CertificateTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            CertificateTypeEnum::TEMPORARY => "TEMPORARY",
            CertificateTypeEnum::GROUPED => "GROUPED",
            CertificateTypeEnum::PROJECTGROUPED => "PROJECT_GROUPED",
            CertificateTypeEnum::DEDICATED => "DEDICATED",
        }
    }
}

impl std::convert::TryFrom< &str> for CertificateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(CertificateTypeEnum::TYPEUNSPECIFIED),
           "TEMPORARY" => Ok(CertificateTypeEnum::TEMPORARY),
           "GROUPED" => Ok(CertificateTypeEnum::GROUPED),
           "PROJECT_GROUPED" => Ok(CertificateTypeEnum::PROJECTGROUPED),
           "DEDICATED" => Ok(CertificateTypeEnum::DEDICATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CertificateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomDomainCertPreferenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A field that lets you specify which SSL certificate type Hosting creates for your domain name. Spark plan custom domains only have access to the `GROUPED` cert type, while Blaze plan domains can select any option.
pub enum CustomDomainCertPreferenceEnum {
    

    /// The certificate's type is unspecified. The message is invalid if this is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A short-lived certificate type that covers a domain name temporarily, while Hosting creates a more permanent certificate.
    ///
    /// "TEMPORARY"
    #[serde(rename="TEMPORARY")]
    TEMPORARY,
    

    /// The standard certificate for Spark plan custom domains.
    ///
    /// "GROUPED"
    #[serde(rename="GROUPED")]
    GROUPED,
    

    /// Blaze plan only. A certificate that covers from 1 to 100 domain names with custom domains on the same Firebase project.
    ///
    /// "PROJECT_GROUPED"
    #[serde(rename="PROJECT_GROUPED")]
    PROJECTGROUPED,
    

    /// Blaze plan only. A certificate that covers a single domain name.
    ///
    /// "DEDICATED"
    #[serde(rename="DEDICATED")]
    DEDICATED,
}

impl AsRef<str> for CustomDomainCertPreferenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomDomainCertPreferenceEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            CustomDomainCertPreferenceEnum::TEMPORARY => "TEMPORARY",
            CustomDomainCertPreferenceEnum::GROUPED => "GROUPED",
            CustomDomainCertPreferenceEnum::PROJECTGROUPED => "PROJECT_GROUPED",
            CustomDomainCertPreferenceEnum::DEDICATED => "DEDICATED",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomDomainCertPreferenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(CustomDomainCertPreferenceEnum::TYPEUNSPECIFIED),
           "TEMPORARY" => Ok(CustomDomainCertPreferenceEnum::TEMPORARY),
           "GROUPED" => Ok(CustomDomainCertPreferenceEnum::GROUPED),
           "PROJECT_GROUPED" => Ok(CustomDomainCertPreferenceEnum::PROJECTGROUPED),
           "DEDICATED" => Ok(CustomDomainCertPreferenceEnum::DEDICATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomDomainCertPreferenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomDomainHostStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The `HostState` of the domain name this `CustomDomain` refers to.
pub enum CustomDomainHostStateEnum {
    

    /// Your custom domain's host state is unspecified. The message is invalid if this is unspecified.
    ///
    /// "HOST_STATE_UNSPECIFIED"
    #[serde(rename="HOST_STATE_UNSPECIFIED")]
    HOSTSTATEUNSPECIFIED,
    

    /// Your custom domain's domain name isn't associated with any IP addresses.
    ///
    /// "HOST_UNHOSTED"
    #[serde(rename="HOST_UNHOSTED")]
    HOSTUNHOSTED,
    

    /// Your custom domain's domain name can't be reached. Hosting services' DNS queries to find your domain name's IP addresses resulted in errors. See your `CustomDomain` object's `issues` field for more details.
    ///
    /// "HOST_UNREACHABLE"
    #[serde(rename="HOST_UNREACHABLE")]
    HOSTUNREACHABLE,
    

    /// Your custom domain's domain name has IP addresses that don't ultimately resolve to Hosting.
    ///
    /// "HOST_MISMATCH"
    #[serde(rename="HOST_MISMATCH")]
    HOSTMISMATCH,
    

    /// Your custom domain's domain name has IP addresses that resolve to both Hosting and other services. To ensure consistent results, remove `A` and `AAAA` records related to non-Hosting services.
    ///
    /// "HOST_CONFLICT"
    #[serde(rename="HOST_CONFLICT")]
    HOSTCONFLICT,
    

    /// All requests against your custom domain's domain name are served by Hosting. If the custom domain's `OwnershipState` is also `ACTIVE`, Hosting serves your Hosting site's content on the domain name.
    ///
    /// "HOST_ACTIVE"
    #[serde(rename="HOST_ACTIVE")]
    HOSTACTIVE,
}

impl AsRef<str> for CustomDomainHostStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomDomainHostStateEnum::HOSTSTATEUNSPECIFIED => "HOST_STATE_UNSPECIFIED",
            CustomDomainHostStateEnum::HOSTUNHOSTED => "HOST_UNHOSTED",
            CustomDomainHostStateEnum::HOSTUNREACHABLE => "HOST_UNREACHABLE",
            CustomDomainHostStateEnum::HOSTMISMATCH => "HOST_MISMATCH",
            CustomDomainHostStateEnum::HOSTCONFLICT => "HOST_CONFLICT",
            CustomDomainHostStateEnum::HOSTACTIVE => "HOST_ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomDomainHostStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HOST_STATE_UNSPECIFIED" => Ok(CustomDomainHostStateEnum::HOSTSTATEUNSPECIFIED),
           "HOST_UNHOSTED" => Ok(CustomDomainHostStateEnum::HOSTUNHOSTED),
           "HOST_UNREACHABLE" => Ok(CustomDomainHostStateEnum::HOSTUNREACHABLE),
           "HOST_MISMATCH" => Ok(CustomDomainHostStateEnum::HOSTMISMATCH),
           "HOST_CONFLICT" => Ok(CustomDomainHostStateEnum::HOSTCONFLICT),
           "HOST_ACTIVE" => Ok(CustomDomainHostStateEnum::HOSTACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomDomainHostStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CustomDomainOwnershipStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The `OwnershipState` of the domain name this `CustomDomain` refers to.
pub enum CustomDomainOwnershipStateEnum {
    

    /// Your custom domain's ownership state is unspecified. This should never happen.
    ///
    /// "OWNERSHIP_STATE_UNSPECIFIED"
    #[serde(rename="OWNERSHIP_STATE_UNSPECIFIED")]
    OWNERSHIPSTATEUNSPECIFIED,
    

    /// Your custom domain's domain name has no Hosting-related ownership records; no Firebase project has permission to act on the domain name's behalf.
    ///
    /// "OWNERSHIP_MISSING"
    #[serde(rename="OWNERSHIP_MISSING")]
    OWNERSHIPMISSING,
    

    /// Your custom domain's domain name can't be reached. Hosting services' DNS queries to find your domain name's ownership records resulted in errors. See your `CustomDomain` object's `issues` field for more details.
    ///
    /// "OWNERSHIP_UNREACHABLE"
    #[serde(rename="OWNERSHIP_UNREACHABLE")]
    OWNERSHIPUNREACHABLE,
    

    /// Your custom domain's domain name is owned by another Firebase project. Remove the conflicting `TXT` records and replace them with project-specific records for your current Firebase project.
    ///
    /// "OWNERSHIP_MISMATCH"
    #[serde(rename="OWNERSHIP_MISMATCH")]
    OWNERSHIPMISMATCH,
    

    /// Your custom domain's domain name has conflicting `TXT` records that indicate ownership by both your current Firebase project and another project. Remove the other project's ownership records to grant the current project ownership.
    ///
    /// "OWNERSHIP_CONFLICT"
    #[serde(rename="OWNERSHIP_CONFLICT")]
    OWNERSHIPCONFLICT,
    

    /// Your custom domain's DNS records are configured correctly. Hosting will transfer ownership of your domain to this `CustomDomain` within 24 hours.
    ///
    /// "OWNERSHIP_PENDING"
    #[serde(rename="OWNERSHIP_PENDING")]
    OWNERSHIPPENDING,
    

    /// Your custom domain's domain name has `TXT` records that grant its project permission to act on its behalf.
    ///
    /// "OWNERSHIP_ACTIVE"
    #[serde(rename="OWNERSHIP_ACTIVE")]
    OWNERSHIPACTIVE,
}

impl AsRef<str> for CustomDomainOwnershipStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CustomDomainOwnershipStateEnum::OWNERSHIPSTATEUNSPECIFIED => "OWNERSHIP_STATE_UNSPECIFIED",
            CustomDomainOwnershipStateEnum::OWNERSHIPMISSING => "OWNERSHIP_MISSING",
            CustomDomainOwnershipStateEnum::OWNERSHIPUNREACHABLE => "OWNERSHIP_UNREACHABLE",
            CustomDomainOwnershipStateEnum::OWNERSHIPMISMATCH => "OWNERSHIP_MISMATCH",
            CustomDomainOwnershipStateEnum::OWNERSHIPCONFLICT => "OWNERSHIP_CONFLICT",
            CustomDomainOwnershipStateEnum::OWNERSHIPPENDING => "OWNERSHIP_PENDING",
            CustomDomainOwnershipStateEnum::OWNERSHIPACTIVE => "OWNERSHIP_ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for CustomDomainOwnershipStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OWNERSHIP_STATE_UNSPECIFIED" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPSTATEUNSPECIFIED),
           "OWNERSHIP_MISSING" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPMISSING),
           "OWNERSHIP_UNREACHABLE" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPUNREACHABLE),
           "OWNERSHIP_MISMATCH" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPMISMATCH),
           "OWNERSHIP_CONFLICT" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPCONFLICT),
           "OWNERSHIP_PENDING" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPPENDING),
           "OWNERSHIP_ACTIVE" => Ok(CustomDomainOwnershipStateEnum::OWNERSHIPACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CustomDomainOwnershipStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsRecordRequiredActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. An enum that indicates the a required action for this record.
pub enum DnsRecordRequiredActionEnum {
    

    /// No action necessary.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Add this record to your DNS records.
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// Remove this record from your DNS records.
    ///
    /// "REMOVE"
    #[serde(rename="REMOVE")]
    REMOVE,
}

impl AsRef<str> for DnsRecordRequiredActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsRecordRequiredActionEnum::NONE => "NONE",
            DnsRecordRequiredActionEnum::ADD => "ADD",
            DnsRecordRequiredActionEnum::REMOVE => "REMOVE",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsRecordRequiredActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(DnsRecordRequiredActionEnum::NONE),
           "ADD" => Ok(DnsRecordRequiredActionEnum::ADD),
           "REMOVE" => Ok(DnsRecordRequiredActionEnum::REMOVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsRecordRequiredActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DnsRecordTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The record's type, which determines what data the record contains.
pub enum DnsRecordTypeEnum {
    

    /// The record's type is unspecified. The message is invalid if this is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// An `A` record, as defined in [RFC 1035](https://tools.ietf.org/html/rfc1035). A records determine which IPv4 addresses a domain name directs traffic towards.
    ///
    /// "A"
    #[serde(rename="A")]
    A,
    

    /// A `CNAME` record, as defined in [RFC 1035](https://tools.ietf.org/html/rfc1035). `CNAME` or Canonical Name records map a domain name to a different, canonical domain name. If a `CNAME` record is present, it should be the only record on the domain name.
    ///
    /// "CNAME"
    #[serde(rename="CNAME")]
    CNAME,
    

    /// A `TXT` record, as defined in [RFC 1035](https://tools.ietf.org/html/rfc1035). `TXT` records hold arbitrary text data on a domain name. Hosting uses `TXT` records to establish which Firebase Project has permission to act on a domain name.
    ///
    /// "TXT"
    #[serde(rename="TXT")]
    TXT,
    

    /// An AAAA record, as defined in [RFC 3596](https://tools.ietf.org/html/rfc3596) AAAA records determine which IPv6 addresses a domain name directs traffic towards.
    ///
    /// "AAAA"
    #[serde(rename="AAAA")]
    AAAA,
    

    /// A CAA record, as defined in [RFC 6844](https://tools.ietf.org/html/rfc6844). CAA, or Certificate Authority Authorization, records determine which Certificate Authorities (SSL certificate minting organizations) are authorized to mint a certificate for the domain name. Firebase Hosting uses `pki.goog` as its primary CA. CAA records cascade. A CAA record on `foo.com` also applies to `bar.foo.com` unless `bar.foo.com` has its own set of CAA records. CAA records are optional. If a domain name and its parents have no CAA records, all CAs are authorized to mint certificates on its behalf. In general, Hosting only asks you to modify CAA records when doing so is required to unblock SSL cert creation.
    ///
    /// "CAA"
    #[serde(rename="CAA")]
    CAA,
}

impl AsRef<str> for DnsRecordTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DnsRecordTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DnsRecordTypeEnum::A => "A",
            DnsRecordTypeEnum::CNAME => "CNAME",
            DnsRecordTypeEnum::TXT => "TXT",
            DnsRecordTypeEnum::AAAA => "AAAA",
            DnsRecordTypeEnum::CAA => "CAA",
        }
    }
}

impl std::convert::TryFrom< &str> for DnsRecordTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DnsRecordTypeEnum::TYPEUNSPECIFIED),
           "A" => Ok(DnsRecordTypeEnum::A),
           "CNAME" => Ok(DnsRecordTypeEnum::CNAME),
           "TXT" => Ok(DnsRecordTypeEnum::TXT),
           "AAAA" => Ok(DnsRecordTypeEnum::AAAA),
           "CAA" => Ok(DnsRecordTypeEnum::CAA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DnsRecordTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Additional status of the domain association.
pub enum DomainStatusEnum {
    

    /// Unspecified domain association status.
    ///
    /// "DOMAIN_STATUS_UNSPECIFIED"
    #[serde(rename="DOMAIN_STATUS_UNSPECIFIED")]
    DOMAINSTATUSUNSPECIFIED,
    

    /// An external operation is in progress on the domain association and no further operations can be performed until it is complete. Formerly used for metabase updates. Not currently used
    ///
    /// "DOMAIN_CHANGE_PENDING"
    #[serde(rename="DOMAIN_CHANGE_PENDING")]
    DOMAINCHANGEPENDING,
    

    /// The domain association is active and no additional action is required.
    ///
    /// "DOMAIN_ACTIVE"
    #[serde(rename="DOMAIN_ACTIVE")]
    DOMAINACTIVE,
    

    /// The domain was previously verified in the legacy system. User must reverify the domain through the ownership service.
    ///
    /// "DOMAIN_VERIFICATION_REQUIRED"
    #[serde(rename="DOMAIN_VERIFICATION_REQUIRED")]
    DOMAINVERIFICATIONREQUIRED,
    

    /// The domain verification has been lost and the domain is in the grace period before being removed from the Firebase Hosting site.
    ///
    /// "DOMAIN_VERIFICATION_LOST"
    #[serde(rename="DOMAIN_VERIFICATION_LOST")]
    DOMAINVERIFICATIONLOST,
}

impl AsRef<str> for DomainStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainStatusEnum::DOMAINSTATUSUNSPECIFIED => "DOMAIN_STATUS_UNSPECIFIED",
            DomainStatusEnum::DOMAINCHANGEPENDING => "DOMAIN_CHANGE_PENDING",
            DomainStatusEnum::DOMAINACTIVE => "DOMAIN_ACTIVE",
            DomainStatusEnum::DOMAINVERIFICATIONREQUIRED => "DOMAIN_VERIFICATION_REQUIRED",
            DomainStatusEnum::DOMAINVERIFICATIONLOST => "DOMAIN_VERIFICATION_LOST",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOMAIN_STATUS_UNSPECIFIED" => Ok(DomainStatusEnum::DOMAINSTATUSUNSPECIFIED),
           "DOMAIN_CHANGE_PENDING" => Ok(DomainStatusEnum::DOMAINCHANGEPENDING),
           "DOMAIN_ACTIVE" => Ok(DomainStatusEnum::DOMAINACTIVE),
           "DOMAIN_VERIFICATION_REQUIRED" => Ok(DomainStatusEnum::DOMAINVERIFICATIONREQUIRED),
           "DOMAIN_VERIFICATION_LOST" => Ok(DomainStatusEnum::DOMAINVERIFICATIONLOST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainProvisioningCertStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The certificate provisioning status; updated when Firebase Hosting provisions an SSL certificate for the domain.
pub enum DomainProvisioningCertStatusEnum {
    

    /// Unspecified certificate provisioning status.
    ///
    /// "CERT_STATUS_UNSPECIFIED"
    #[serde(rename="CERT_STATUS_UNSPECIFIED")]
    CERTSTATUSUNSPECIFIED,
    

    /// Waiting for certificate challenge to be created.
    ///
    /// "CERT_PENDING"
    #[serde(rename="CERT_PENDING")]
    CERTPENDING,
    

    /// Waiting for certificate challenge to be met.
    ///
    /// "CERT_MISSING"
    #[serde(rename="CERT_MISSING")]
    CERTMISSING,
    

    /// Certificate challenge met; attempting to acquire/propagate certificate.
    ///
    /// "CERT_PROCESSING"
    #[serde(rename="CERT_PROCESSING")]
    CERTPROCESSING,
    

    /// Certificate obtained; propagating to the CDN.
    ///
    /// "CERT_PROPAGATING"
    #[serde(rename="CERT_PROPAGATING")]
    CERTPROPAGATING,
    

    /// Certificate provisioned and deployed across the CDN.
    ///
    /// "CERT_ACTIVE"
    #[serde(rename="CERT_ACTIVE")]
    CERTACTIVE,
    

    /// Certificate provisioning failed in a non-recoverable manner.
    ///
    /// "CERT_ERROR"
    #[serde(rename="CERT_ERROR")]
    CERTERROR,
}

impl AsRef<str> for DomainProvisioningCertStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainProvisioningCertStatusEnum::CERTSTATUSUNSPECIFIED => "CERT_STATUS_UNSPECIFIED",
            DomainProvisioningCertStatusEnum::CERTPENDING => "CERT_PENDING",
            DomainProvisioningCertStatusEnum::CERTMISSING => "CERT_MISSING",
            DomainProvisioningCertStatusEnum::CERTPROCESSING => "CERT_PROCESSING",
            DomainProvisioningCertStatusEnum::CERTPROPAGATING => "CERT_PROPAGATING",
            DomainProvisioningCertStatusEnum::CERTACTIVE => "CERT_ACTIVE",
            DomainProvisioningCertStatusEnum::CERTERROR => "CERT_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainProvisioningCertStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERT_STATUS_UNSPECIFIED" => Ok(DomainProvisioningCertStatusEnum::CERTSTATUSUNSPECIFIED),
           "CERT_PENDING" => Ok(DomainProvisioningCertStatusEnum::CERTPENDING),
           "CERT_MISSING" => Ok(DomainProvisioningCertStatusEnum::CERTMISSING),
           "CERT_PROCESSING" => Ok(DomainProvisioningCertStatusEnum::CERTPROCESSING),
           "CERT_PROPAGATING" => Ok(DomainProvisioningCertStatusEnum::CERTPROPAGATING),
           "CERT_ACTIVE" => Ok(DomainProvisioningCertStatusEnum::CERTACTIVE),
           "CERT_ERROR" => Ok(DomainProvisioningCertStatusEnum::CERTERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainProvisioningCertStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainProvisioningDnsStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The DNS record match status as of the last DNS fetch.
pub enum DomainProvisioningDnsStatusEnum {
    

    /// Unspecified DNS status.
    ///
    /// "DNS_STATUS_UNSPECIFIED"
    #[serde(rename="DNS_STATUS_UNSPECIFIED")]
    DNSSTATUSUNSPECIFIED,
    

    /// No DNS records have been specified for this domain yet.
    ///
    /// "DNS_PENDING"
    #[serde(rename="DNS_PENDING")]
    DNSPENDING,
    

    /// None of the required DNS records have been detected on the domain.
    ///
    /// "DNS_MISSING"
    #[serde(rename="DNS_MISSING")]
    DNSMISSING,
    

    /// Some of the required DNS records were detected, but not all of them. No extra (non-required) DNS records were detected.
    ///
    /// "DNS_PARTIAL_MATCH"
    #[serde(rename="DNS_PARTIAL_MATCH")]
    DNSPARTIALMATCH,
    

    /// All required DNS records were detected. No extra (non-required) DNS records were detected.
    ///
    /// "DNS_MATCH"
    #[serde(rename="DNS_MATCH")]
    DNSMATCH,
    

    /// The domain has at least one of the required DNS records, and it has at least one extra (non-required) DNS record.
    ///
    /// "DNS_EXTRANEOUS_MATCH"
    #[serde(rename="DNS_EXTRANEOUS_MATCH")]
    DNSEXTRANEOUSMATCH,
}

impl AsRef<str> for DomainProvisioningDnsStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainProvisioningDnsStatusEnum::DNSSTATUSUNSPECIFIED => "DNS_STATUS_UNSPECIFIED",
            DomainProvisioningDnsStatusEnum::DNSPENDING => "DNS_PENDING",
            DomainProvisioningDnsStatusEnum::DNSMISSING => "DNS_MISSING",
            DomainProvisioningDnsStatusEnum::DNSPARTIALMATCH => "DNS_PARTIAL_MATCH",
            DomainProvisioningDnsStatusEnum::DNSMATCH => "DNS_MATCH",
            DomainProvisioningDnsStatusEnum::DNSEXTRANEOUSMATCH => "DNS_EXTRANEOUS_MATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainProvisioningDnsStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DNS_STATUS_UNSPECIFIED" => Ok(DomainProvisioningDnsStatusEnum::DNSSTATUSUNSPECIFIED),
           "DNS_PENDING" => Ok(DomainProvisioningDnsStatusEnum::DNSPENDING),
           "DNS_MISSING" => Ok(DomainProvisioningDnsStatusEnum::DNSMISSING),
           "DNS_PARTIAL_MATCH" => Ok(DomainProvisioningDnsStatusEnum::DNSPARTIALMATCH),
           "DNS_MATCH" => Ok(DomainProvisioningDnsStatusEnum::DNSMATCH),
           "DNS_EXTRANEOUS_MATCH" => Ok(DomainProvisioningDnsStatusEnum::DNSEXTRANEOUSMATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainProvisioningDnsStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DomainRedirectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The redirect status code.
pub enum DomainRedirectTypeEnum {
    

    /// The default redirect type; should not be intentionlly used.
    ///
    /// "REDIRECT_TYPE_UNSPECIFIED"
    #[serde(rename="REDIRECT_TYPE_UNSPECIFIED")]
    REDIRECTTYPEUNSPECIFIED,
    

    /// The redirect will respond with an HTTP status code of `301 Moved Permanently`.
    ///
    /// "MOVED_PERMANENTLY"
    #[serde(rename="MOVED_PERMANENTLY")]
    MOVEDPERMANENTLY,
}

impl AsRef<str> for DomainRedirectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainRedirectTypeEnum::REDIRECTTYPEUNSPECIFIED => "REDIRECT_TYPE_UNSPECIFIED",
            DomainRedirectTypeEnum::MOVEDPERMANENTLY => "MOVED_PERMANENTLY",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainRedirectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDIRECT_TYPE_UNSPECIFIED" => Ok(DomainRedirectTypeEnum::REDIRECTTYPEUNSPECIFIED),
           "MOVED_PERMANENTLY" => Ok(DomainRedirectTypeEnum::MOVEDPERMANENTLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainRedirectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReleaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Explains the reason for the release. Specify a value for this field only when creating a `SITE_DISABLE` type release.
pub enum ReleaseTypeEnum {
    

    /// An unspecified type. Indicates that a version was released. This is the default value when no other `type` is explicitly specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A version was uploaded to Firebase Hosting and released.
    ///
    /// "DEPLOY"
    #[serde(rename="DEPLOY")]
    DEPLOY,
    

    /// The release points back to a previously deployed version.
    ///
    /// "ROLLBACK"
    #[serde(rename="ROLLBACK")]
    ROLLBACK,
    

    /// The release prevents the site from serving content. Firebase Hosting acts as if the site never existed.
    ///
    /// "SITE_DISABLE"
    #[serde(rename="SITE_DISABLE")]
    SITEDISABLE,
}

impl AsRef<str> for ReleaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReleaseTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ReleaseTypeEnum::DEPLOY => "DEPLOY",
            ReleaseTypeEnum::ROLLBACK => "ROLLBACK",
            ReleaseTypeEnum::SITEDISABLE => "SITE_DISABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReleaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ReleaseTypeEnum::TYPEUNSPECIFIED),
           "DEPLOY" => Ok(ReleaseTypeEnum::DEPLOY),
           "ROLLBACK" => Ok(ReleaseTypeEnum::ROLLBACK),
           "SITE_DISABLE" => Ok(ReleaseTypeEnum::SITEDISABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReleaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServingConfigAppAssociationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How to handle well known App Association files.
pub enum ServingConfigAppAssociationEnum {
    

    /// The app association files will be automatically created from the apps that exist in the Firebase project.
    ///
    /// "AUTO"
    #[serde(rename="AUTO")]
    AUTO,
    

    /// No special handling of the app association files will occur, these paths will result in a 404 unless caught with a Rewrite.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for ServingConfigAppAssociationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServingConfigAppAssociationEnum::AUTO => "AUTO",
            ServingConfigAppAssociationEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServingConfigAppAssociationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO" => Ok(ServingConfigAppAssociationEnum::AUTO),
           "NONE" => Ok(ServingConfigAppAssociationEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServingConfigAppAssociationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServingConfigTrailingSlashBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines how to handle a trailing slash in the URL path.
pub enum ServingConfigTrailingSlashBehaviorEnum {
    

    /// No behavior is specified. Files are served at their exact location only, and trailing slashes are only added to directory indexes.
    ///
    /// "TRAILING_SLASH_BEHAVIOR_UNSPECIFIED"
    #[serde(rename="TRAILING_SLASH_BEHAVIOR_UNSPECIFIED")]
    TRAILINGSLASHBEHAVIORUNSPECIFIED,
    

    /// Trailing slashes are _added_ to directory indexes as well as to any URL path not ending in a file extension.
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// Trailing slashes are _removed_ from directory indexes as well as from any URL path not ending in a file extension.
    ///
    /// "REMOVE"
    #[serde(rename="REMOVE")]
    REMOVE,
}

impl AsRef<str> for ServingConfigTrailingSlashBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServingConfigTrailingSlashBehaviorEnum::TRAILINGSLASHBEHAVIORUNSPECIFIED => "TRAILING_SLASH_BEHAVIOR_UNSPECIFIED",
            ServingConfigTrailingSlashBehaviorEnum::ADD => "ADD",
            ServingConfigTrailingSlashBehaviorEnum::REMOVE => "REMOVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServingConfigTrailingSlashBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRAILING_SLASH_BEHAVIOR_UNSPECIFIED" => Ok(ServingConfigTrailingSlashBehaviorEnum::TRAILINGSLASHBEHAVIORUNSPECIFIED),
           "ADD" => Ok(ServingConfigTrailingSlashBehaviorEnum::ADD),
           "REMOVE" => Ok(ServingConfigTrailingSlashBehaviorEnum::REMOVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServingConfigTrailingSlashBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of Hosting site. Every Firebase project has a `DEFAULT_SITE`, which is created when Hosting is provisioned for the project. All additional sites are `USER_SITE`.
pub enum SiteTypeEnum {
    

    /// Unknown state, likely the result of an error on the backend.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The default Hosting site that is provisioned when a Firebase project is created.
    ///
    /// "DEFAULT_SITE"
    #[serde(rename="DEFAULT_SITE")]
    DEFAULTSITE,
    

    /// A Hosting site that the user created.
    ///
    /// "USER_SITE"
    #[serde(rename="USER_SITE")]
    USERSITE,
}

impl AsRef<str> for SiteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            SiteTypeEnum::DEFAULTSITE => "DEFAULT_SITE",
            SiteTypeEnum::USERSITE => "USER_SITE",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(SiteTypeEnum::TYPEUNSPECIFIED),
           "DEFAULT_SITE" => Ok(SiteTypeEnum::DEFAULTSITE),
           "USER_SITE" => Ok(SiteTypeEnum::USERSITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VersionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The deploy status of the version. For a successful deploy, call [`CreateVersion`](sites.versions/create) to make a new version (`CREATED` status), [upload all desired files](sites.versions/populateFiles) to the version, then [update](sites.versions/patch) the version to the `FINALIZED` status. Note that if you leave the version in the `CREATED` state for more than 12 hours, the system will automatically mark the version as `ABANDONED`. You can also change the status of a version to `DELETED` by calling [`DeleteVersion`](sites.versions/delete).
pub enum VersionStatusEnum {
    

    /// The default status; should not be intentionally used.
    ///
    /// "VERSION_STATUS_UNSPECIFIED"
    #[serde(rename="VERSION_STATUS_UNSPECIFIED")]
    VERSIONSTATUSUNSPECIFIED,
    

    /// The version has been created, and content is currently being added to the version.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// All content has been added to the version, and the version can no longer be changed.
    ///
    /// "FINALIZED"
    #[serde(rename="FINALIZED")]
    FINALIZED,
    

    /// The version has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The version was not updated to `FINALIZED` within 12 hours and was automatically deleted.
    ///
    /// "ABANDONED"
    #[serde(rename="ABANDONED")]
    ABANDONED,
    

    /// The version is outside the site-configured limit for the number of retained versions, so the version's content is scheduled for deletion.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// The version is being cloned from another version. All content is still being copied over.
    ///
    /// "CLONING"
    #[serde(rename="CLONING")]
    CLONING,
}

impl AsRef<str> for VersionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VersionStatusEnum::VERSIONSTATUSUNSPECIFIED => "VERSION_STATUS_UNSPECIFIED",
            VersionStatusEnum::CREATED => "CREATED",
            VersionStatusEnum::FINALIZED => "FINALIZED",
            VersionStatusEnum::DELETED => "DELETED",
            VersionStatusEnum::ABANDONED => "ABANDONED",
            VersionStatusEnum::EXPIRED => "EXPIRED",
            VersionStatusEnum::CLONING => "CLONING",
        }
    }
}

impl std::convert::TryFrom< &str> for VersionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_STATUS_UNSPECIFIED" => Ok(VersionStatusEnum::VERSIONSTATUSUNSPECIFIED),
           "CREATED" => Ok(VersionStatusEnum::CREATED),
           "FINALIZED" => Ok(VersionStatusEnum::FINALIZED),
           "DELETED" => Ok(VersionStatusEnum::DELETED),
           "ABANDONED" => Ok(VersionStatusEnum::ABANDONED),
           "EXPIRED" => Ok(VersionStatusEnum::EXPIRED),
           "CLONING" => Ok(VersionStatusEnum::CLONING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VersionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VersionFileStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current status of a particular file in the specified version. The value will be either `pending upload` or `uploaded`.
pub enum VersionFileStatusEnum {
    

    /// The default status; should not be intentionally used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The file has been included in the version and is expected to be uploaded in the near future.
    ///
    /// "EXPECTED"
    #[serde(rename="EXPECTED")]
    EXPECTED,
    

    /// The file has already been uploaded to Firebase Hosting.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for VersionFileStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VersionFileStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            VersionFileStatusEnum::EXPECTED => "EXPECTED",
            VersionFileStatusEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for VersionFileStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(VersionFileStatusEnum::STATUSUNSPECIFIED),
           "EXPECTED" => Ok(VersionFileStatusEnum::EXPECTED),
           "ACTIVE" => Ok(VersionFileStatusEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VersionFileStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
///  The type of files that should be listed for the specified version.
pub enum ProjectStatusEnum {
    

    /// The default status; should not be intentionally used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The file has been included in the version and is expected to be uploaded in the near future.
    ///
    /// "EXPECTED"
    #[serde(rename="EXPECTED")]
    EXPECTED,
    

    /// The file has already been uploaded to Firebase Hosting.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for ProjectStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            ProjectStatusEnum::EXPECTED => "EXPECTED",
            ProjectStatusEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(ProjectStatusEnum::STATUSUNSPECIFIED),
           "EXPECTED" => Ok(ProjectStatusEnum::EXPECTED),
           "ACTIVE" => Ok(ProjectStatusEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SiteStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
///  The type of files that should be listed for the specified version.
pub enum SiteStatusEnum {
    

    /// The default status; should not be intentionally used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The file has been included in the version and is expected to be uploaded in the near future.
    ///
    /// "EXPECTED"
    #[serde(rename="EXPECTED")]
    EXPECTED,
    

    /// The file has already been uploaded to Firebase Hosting.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for SiteStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SiteStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            SiteStatusEnum::EXPECTED => "EXPECTED",
            SiteStatusEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for SiteStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(SiteStatusEnum::STATUSUNSPECIFIED),
           "EXPECTED" => Ok(SiteStatusEnum::EXPECTED),
           "ACTIVE" => Ok(SiteStatusEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SiteStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


