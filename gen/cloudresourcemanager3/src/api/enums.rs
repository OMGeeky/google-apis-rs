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


// region FolderStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The lifecycle state of the folder. Updates to the state must be performed using DeleteFolder and UndeleteFolder.
pub enum FolderStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

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

impl AsRef<str> for FolderStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FolderStateEnum::ACTIVE => "ACTIVE",
            FolderStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FolderStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(FolderStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(FolderStateEnum::DELETEREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The organization's current lifecycle state.
pub enum OrganizationStateEnum {
    

    /// Unspecified state. This is only useful for distinguishing unset values.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

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

impl AsRef<str> for OrganizationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            OrganizationStateEnum::ACTIVE => "ACTIVE",
            OrganizationStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(OrganizationStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(OrganizationStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(OrganizationStateEnum::DELETEREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The project lifecycle state.
pub enum ProjectStateEnum {
    

    /// Unspecified state. This is only used/useful for distinguishing unset values.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

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
}

impl AsRef<str> for ProjectStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ProjectStateEnum::ACTIVE => "ACTIVE",
            ProjectStateEnum::DELETEREQUESTED => "DELETE_REQUESTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ProjectStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ProjectStateEnum::ACTIVE),
           "DELETE_REQUESTED" => Ok(ProjectStateEnum::DELETEREQUESTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TagKeyPurposeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. A purpose does not grant a policy engine exclusive rights to the Tag, and it may be referenced by other policy engines. A purpose cannot be changed once set.
pub enum TagKeyPurposeEnum {
    

    /// Unspecified purpose.
    ///
    /// "PURPOSE_UNSPECIFIED"
    #[serde(rename="PURPOSE_UNSPECIFIED")]
    PURPOSEUNSPECIFIED,
    

    /// Purpose for Compute Engine firewalls. A corresponding `purpose_data` should be set for the network the tag is intended for. The key should be `network` and the value should be in ## either of these two formats: `https://www.googleapis.com/compute/{compute_version}/projects/{project_id}/global/networks/{network_id}` - `{project_id}/{network_name}` ## Examples: `https://www.googleapis.com/compute/staging_v1/projects/fail-closed-load-testing/global/networks/6992953698831725600` - `fail-closed-load-testing/load-testing-network`
    ///
    /// "GCE_FIREWALL"
    #[serde(rename="GCE_FIREWALL")]
    GCEFIREWALL,
    

    /// Purpose for data governance. Tag Values created under a key with this purpose may have Tag Value children. No `purpose_data` should be set.
    ///
    /// "DATA_GOVERNANCE"
    #[serde(rename="DATA_GOVERNANCE")]
    DATAGOVERNANCE,
}

impl AsRef<str> for TagKeyPurposeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TagKeyPurposeEnum::PURPOSEUNSPECIFIED => "PURPOSE_UNSPECIFIED",
            TagKeyPurposeEnum::GCEFIREWALL => "GCE_FIREWALL",
            TagKeyPurposeEnum::DATAGOVERNANCE => "DATA_GOVERNANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for TagKeyPurposeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PURPOSE_UNSPECIFIED" => Ok(TagKeyPurposeEnum::PURPOSEUNSPECIFIED),
           "GCE_FIREWALL" => Ok(TagKeyPurposeEnum::GCEFIREWALL),
           "DATA_GOVERNANCE" => Ok(TagKeyPurposeEnum::DATAGOVERNANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TagKeyPurposeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


