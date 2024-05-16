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


// region DomainMappingSpecCertificateModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The mode of the certificate.
pub enum DomainMappingSpecCertificateModeEnum {
    
    /// "CERTIFICATE_MODE_UNSPECIFIED"
    #[serde(rename="CERTIFICATE_MODE_UNSPECIFIED")]
    CERTIFICATEMODEUNSPECIFIED,
    

    /// Do not provision an HTTPS certificate.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Automatically provisions an HTTPS certificate via GoogleCA.
    ///
    /// "AUTOMATIC"
    #[serde(rename="AUTOMATIC")]
    AUTOMATIC,
}

impl AsRef<str> for DomainMappingSpecCertificateModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DomainMappingSpecCertificateModeEnum::CERTIFICATEMODEUNSPECIFIED => "CERTIFICATE_MODE_UNSPECIFIED",
            DomainMappingSpecCertificateModeEnum::NONE => "NONE",
            DomainMappingSpecCertificateModeEnum::AUTOMATIC => "AUTOMATIC",
        }
    }
}

impl std::convert::TryFrom< &str> for DomainMappingSpecCertificateModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERTIFICATE_MODE_UNSPECIFIED" => Ok(DomainMappingSpecCertificateModeEnum::CERTIFICATEMODEUNSPECIFIED),
           "NONE" => Ok(DomainMappingSpecCertificateModeEnum::NONE),
           "AUTOMATIC" => Ok(DomainMappingSpecCertificateModeEnum::AUTOMATIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DomainMappingSpecCertificateModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceRecordTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Resource record type. Example: `AAAA`.
pub enum ResourceRecordTypeEnum {
    

    /// An unknown resource record.
    ///
    /// "RECORD_TYPE_UNSPECIFIED"
    #[serde(rename="RECORD_TYPE_UNSPECIFIED")]
    RECORDTYPEUNSPECIFIED,
    

    /// An A resource record. Data is an IPv4 address.
    ///
    /// "A"
    #[serde(rename="A")]
    A,
    

    /// An AAAA resource record. Data is an IPv6 address.
    ///
    /// "AAAA"
    #[serde(rename="AAAA")]
    AAAA,
    

    /// A CNAME resource record. Data is a domain name to be aliased.
    ///
    /// "CNAME"
    #[serde(rename="CNAME")]
    CNAME,
}

impl AsRef<str> for ResourceRecordTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceRecordTypeEnum::RECORDTYPEUNSPECIFIED => "RECORD_TYPE_UNSPECIFIED",
            ResourceRecordTypeEnum::A => "A",
            ResourceRecordTypeEnum::AAAA => "AAAA",
            ResourceRecordTypeEnum::CNAME => "CNAME",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceRecordTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECORD_TYPE_UNSPECIFIED" => Ok(ResourceRecordTypeEnum::RECORDTYPEUNSPECIFIED),
           "A" => Ok(ResourceRecordTypeEnum::A),
           "AAAA" => Ok(ResourceRecordTypeEnum::AAAA),
           "CNAME" => Ok(ResourceRecordTypeEnum::CNAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceRecordTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


