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


// region EndpointSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Lowest threat severity that this endpoint will alert on.
pub enum EndpointSeverityEnum {
    

    /// Not set.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Informational alerts.
    ///
    /// "INFORMATIONAL"
    #[serde(rename="INFORMATIONAL")]
    INFORMATIONAL,
    

    /// Low severity alerts.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Medium severity alerts.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// High severity alerts.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Critical severity alerts.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for EndpointSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            EndpointSeverityEnum::INFORMATIONAL => "INFORMATIONAL",
            EndpointSeverityEnum::LOW => "LOW",
            EndpointSeverityEnum::MEDIUM => "MEDIUM",
            EndpointSeverityEnum::HIGH => "HIGH",
            EndpointSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(EndpointSeverityEnum::SEVERITYUNSPECIFIED),
           "INFORMATIONAL" => Ok(EndpointSeverityEnum::INFORMATIONAL),
           "LOW" => Ok(EndpointSeverityEnum::LOW),
           "MEDIUM" => Ok(EndpointSeverityEnum::MEDIUM),
           "HIGH" => Ok(EndpointSeverityEnum::HIGH),
           "CRITICAL" => Ok(EndpointSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the endpoint.
pub enum EndpointStateEnum {
    

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
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for EndpointStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            EndpointStateEnum::CREATING => "CREATING",
            EndpointStateEnum::READY => "READY",
            EndpointStateEnum::DELETING => "DELETING",
            EndpointStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(EndpointStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(EndpointStateEnum::CREATING),
           "READY" => Ok(EndpointStateEnum::READY),
           "DELETING" => Ok(EndpointStateEnum::DELETING),
           "UPDATING" => Ok(EndpointStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


