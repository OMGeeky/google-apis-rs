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


// region OrganizationLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The organization's current lifecycle state. Assigned by the server.
pub enum OrganizationLifecycleStateEnum {
    

    /// Unspecified state. This is only useful for distinguishing unset values.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The organization has been marked for deletion by the user.
    ///
    /// "DELETE_REQUESTED"
    #[serde(rename="DELETE_REQUESTED")]
    DELETEREQUESTED,
}

impl AsRef<str> for OrganizationLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            OrganizationLifecycleStateEnum::ACTIVE => "ACTIVE",
            OrganizationLifecycleStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(OrganizationLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "ACTIVE" => Ok(OrganizationLifecycleStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(OrganizationLifecycleStateEnum::DELETEREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Project lifecycle state. Read-only.
pub enum ProjectLifecycleStateEnum {
    

    /// Unspecified state. This is only used/useful for distinguishing unset values.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The project has been marked for deletion by the user (by invoking DeleteProject) or by the system (Google Cloud Platform). This can generally be reversed by invoking UndeleteProject.
    ///
    /// "DELETE_REQUESTED"
    #[serde(rename="DELETE_REQUESTED")]
    DELETEREQUESTED,
    

    /// This lifecycle state is no longer used and is not returned by the API.
    ///
    /// "DELETE_IN_PROGRESS"
    #[serde(rename="DELETE_IN_PROGRESS")]
    DELETEINPROGRESS,
}

impl AsRef<str> for ProjectLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            ProjectLifecycleStateEnum::ACTIVE => "ACTIVE",
            ProjectLifecycleStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
            ProjectLifecycleStateEnum::DELETEINPROGRESS => "DELETE_IN_PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(ProjectLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "ACTIVE" => Ok(ProjectLifecycleStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(ProjectLifecycleStateEnum::DELETEREQUESTED),
           "DELETE_IN_PROGRESS" => Ok(ProjectLifecycleStateEnum::DELETEINPROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


