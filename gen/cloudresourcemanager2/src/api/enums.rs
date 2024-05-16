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


// region FolderLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the folder. Updates to the lifecycle_state must be performed via DeleteFolder and UndeleteFolder.
pub enum FolderLifecycleStateEnum {
    

    /// Unspecified state.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The folder has been marked for deletion by the user.
    ///
    /// "DELETE_REQUESTED"
    #[serde(rename="DELETE_REQUESTED")]
    DELETEREQUESTED,
}

impl AsRef<str> for FolderLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            FolderLifecycleStateEnum::ACTIVE => "ACTIVE",
            FolderLifecycleStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(FolderLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "ACTIVE" => Ok(FolderLifecycleStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(FolderLifecycleStateEnum::DELETEREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


