use super::*;



// region DatabaseInstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The database's lifecycle state. Read-only.
pub enum DatabaseInstanceStateEnum {
    

    /// Unspecified state, likely the result of an error on the backend. This is only used for distinguishing unset values.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// The normal and active state.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The database is in a disabled state. It can be re-enabled later.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The database is in a deleted state.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for DatabaseInstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            DatabaseInstanceStateEnum::ACTIVE => "ACTIVE",
            DatabaseInstanceStateEnum::DISABLED => "DISABLED",
            DatabaseInstanceStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(DatabaseInstanceStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "ACTIVE" => Ok(DatabaseInstanceStateEnum::ACTIVE),
           "DISABLED" => Ok(DatabaseInstanceStateEnum::DISABLED),
           "DELETED" => Ok(DatabaseInstanceStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseInstanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The database instance type. On creation only USER_DATABASE is allowed, which is also the default when omitted.
pub enum DatabaseInstanceTypeEnum {
    

    /// Unknown state, likely the result of an error on the backend. This is only used for distinguishing unset values.
    ///
    /// "DATABASE_INSTANCE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_INSTANCE_TYPE_UNSPECIFIED")]
    DATABASEINSTANCETYPEUNSPECIFIED,
    

    /// The default database that is provisioned when a project is created.
    ///
    /// "DEFAULT_DATABASE"
    #[serde(rename="DEFAULT_DATABASE")]
    DEFAULTDATABASE,
    

    /// A database that the user created.
    ///
    /// "USER_DATABASE"
    #[serde(rename="USER_DATABASE")]
    USERDATABASE,
}

impl AsRef<str> for DatabaseInstanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceTypeEnum::DATABASEINSTANCETYPEUNSPECIFIED => "DATABASE_INSTANCE_TYPE_UNSPECIFIED",
            DatabaseInstanceTypeEnum::DEFAULTDATABASE => "DEFAULT_DATABASE",
            DatabaseInstanceTypeEnum::USERDATABASE => "USER_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_INSTANCE_TYPE_UNSPECIFIED" => Ok(DatabaseInstanceTypeEnum::DATABASEINSTANCETYPEUNSPECIFIED),
           "DEFAULT_DATABASE" => Ok(DatabaseInstanceTypeEnum::DEFAULTDATABASE),
           "USER_DATABASE" => Ok(DatabaseInstanceTypeEnum::USERDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


