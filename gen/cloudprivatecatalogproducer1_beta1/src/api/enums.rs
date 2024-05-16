use super::*;



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


