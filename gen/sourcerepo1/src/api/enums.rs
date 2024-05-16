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


// region PubsubConfigMessageFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of the Cloud Pub/Sub messages.
pub enum PubsubConfigMessageFormatEnum {
    

    /// Unspecified.
    ///
    /// "MESSAGE_FORMAT_UNSPECIFIED"
    #[serde(rename="MESSAGE_FORMAT_UNSPECIFIED")]
    MESSAGEFORMATUNSPECIFIED,
    

    /// The message payload is a serialized protocol buffer of SourceRepoEvent.
    ///
    /// "PROTOBUF"
    #[serde(rename="PROTOBUF")]
    PROTOBUF,
    

    /// The message payload is a JSON string of SourceRepoEvent.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for PubsubConfigMessageFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PubsubConfigMessageFormatEnum::MESSAGEFORMATUNSPECIFIED => "MESSAGE_FORMAT_UNSPECIFIED",
            PubsubConfigMessageFormatEnum::PROTOBUF => "PROTOBUF",
            PubsubConfigMessageFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for PubsubConfigMessageFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_FORMAT_UNSPECIFIED" => Ok(PubsubConfigMessageFormatEnum::MESSAGEFORMATUNSPECIFIED),
           "PROTOBUF" => Ok(PubsubConfigMessageFormatEnum::PROTOBUF),
           "JSON" => Ok(PubsubConfigMessageFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PubsubConfigMessageFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


