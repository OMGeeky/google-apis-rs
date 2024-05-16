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


// region CloudSqlPropertyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the Cloud SQL database.
pub enum CloudSqlPropertyTypeEnum {
    

    /// Unspecified database type.
    ///
    /// "DATABASE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_TYPE_UNSPECIFIED")]
    DATABASETYPEUNSPECIFIED,
    

    /// Cloud SQL for PostgreSQL.
    ///
    /// "POSTGRES"
    #[serde(rename="POSTGRES")]
    POSTGRES,
    

    /// Cloud SQL for MySQL.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
}

impl AsRef<str> for CloudSqlPropertyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudSqlPropertyTypeEnum::DATABASETYPEUNSPECIFIED => "DATABASE_TYPE_UNSPECIFIED",
            CloudSqlPropertyTypeEnum::POSTGRES => "POSTGRES",
            CloudSqlPropertyTypeEnum::MYSQL => "MYSQL",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudSqlPropertyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_TYPE_UNSPECIFIED" => Ok(CloudSqlPropertyTypeEnum::DATABASETYPEUNSPECIFIED),
           "POSTGRES" => Ok(CloudSqlPropertyTypeEnum::POSTGRES),
           "MYSQL" => Ok(CloudSqlPropertyTypeEnum::MYSQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudSqlPropertyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


