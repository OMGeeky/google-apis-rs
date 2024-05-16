use super::*;



// region ApiWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Code to uniquely identify the warning type.
pub enum ApiWarningCodeEnum {
    

    /// An unknown or unset warning type from Cloud SQL API.
    ///
    /// "SQL_API_WARNING_CODE_UNSPECIFIED"
    #[serde(rename="SQL_API_WARNING_CODE_UNSPECIFIED")]
    SQLAPIWARNINGCODEUNSPECIFIED,
    

    /// Warning when one or more regions are not reachable. The returned result set may be incomplete.
    ///
    /// "REGION_UNREACHABLE"
    #[serde(rename="REGION_UNREACHABLE")]
    REGIONUNREACHABLE,
    

    /// Warning when user provided maxResults parameter exceeds the limit. The returned result set may be incomplete.
    ///
    /// "MAX_RESULTS_EXCEEDS_LIMIT"
    #[serde(rename="MAX_RESULTS_EXCEEDS_LIMIT")]
    MAXRESULTSEXCEEDSLIMIT,
    

    /// Warning when user tries to create/update a user with credentials that have previously been compromised by a public data breach.
    ///
    /// "COMPROMISED_CREDENTIALS"
    #[serde(rename="COMPROMISED_CREDENTIALS")]
    COMPROMISEDCREDENTIALS,
    

    /// Warning when the operation succeeds but some non-critical workflow state failed.
    ///
    /// "INTERNAL_STATE_FAILURE"
    #[serde(rename="INTERNAL_STATE_FAILURE")]
    INTERNALSTATEFAILURE,
}

impl AsRef<str> for ApiWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiWarningCodeEnum::SQLAPIWARNINGCODEUNSPECIFIED => "SQL_API_WARNING_CODE_UNSPECIFIED",
            ApiWarningCodeEnum::REGIONUNREACHABLE => "REGION_UNREACHABLE",
            ApiWarningCodeEnum::MAXRESULTSEXCEEDSLIMIT => "MAX_RESULTS_EXCEEDS_LIMIT",
            ApiWarningCodeEnum::COMPROMISEDCREDENTIALS => "COMPROMISED_CREDENTIALS",
            ApiWarningCodeEnum::INTERNALSTATEFAILURE => "INTERNAL_STATE_FAILURE",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_API_WARNING_CODE_UNSPECIFIED" => Ok(ApiWarningCodeEnum::SQLAPIWARNINGCODEUNSPECIFIED),
           "REGION_UNREACHABLE" => Ok(ApiWarningCodeEnum::REGIONUNREACHABLE),
           "MAX_RESULTS_EXCEEDS_LIMIT" => Ok(ApiWarningCodeEnum::MAXRESULTSEXCEEDSLIMIT),
           "COMPROMISED_CREDENTIALS" => Ok(ApiWarningCodeEnum::COMPROMISEDCREDENTIALS),
           "INTERNAL_STATE_FAILURE" => Ok(ApiWarningCodeEnum::INTERNALSTATEFAILURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupConfigurationTransactionalLogStorageStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. This value contains the storage location of transactional logs for the database for point-in-time recovery.
pub enum BackupConfigurationTransactionalLogStorageStateEnum {
    

    /// Unspecified.
    ///
    /// "TRANSACTIONAL_LOG_STORAGE_STATE_UNSPECIFIED"
    #[serde(rename="TRANSACTIONAL_LOG_STORAGE_STATE_UNSPECIFIED")]
    TRANSACTIONALLOGSTORAGESTATEUNSPECIFIED,
    

    /// The transaction logs for the instance are stored on a data disk.
    ///
    /// "DISK"
    #[serde(rename="DISK")]
    DISK,
    

    /// The transaction logs for the instance are switching from being stored on a data disk to being stored in Cloud Storage.
    ///
    /// "SWITCHING_TO_CLOUD_STORAGE"
    #[serde(rename="SWITCHING_TO_CLOUD_STORAGE")]
    SWITCHINGTOCLOUDSTORAGE,
    

    /// The transaction logs for the instance are now stored in Cloud Storage. Previously, they were stored on a data disk.
    ///
    /// "SWITCHED_TO_CLOUD_STORAGE"
    #[serde(rename="SWITCHED_TO_CLOUD_STORAGE")]
    SWITCHEDTOCLOUDSTORAGE,
    

    /// The transaction logs for the instance are stored in Cloud Storage.
    ///
    /// "CLOUD_STORAGE"
    #[serde(rename="CLOUD_STORAGE")]
    CLOUDSTORAGE,
}

impl AsRef<str> for BackupConfigurationTransactionalLogStorageStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupConfigurationTransactionalLogStorageStateEnum::TRANSACTIONALLOGSTORAGESTATEUNSPECIFIED => "TRANSACTIONAL_LOG_STORAGE_STATE_UNSPECIFIED",
            BackupConfigurationTransactionalLogStorageStateEnum::DISK => "DISK",
            BackupConfigurationTransactionalLogStorageStateEnum::SWITCHINGTOCLOUDSTORAGE => "SWITCHING_TO_CLOUD_STORAGE",
            BackupConfigurationTransactionalLogStorageStateEnum::SWITCHEDTOCLOUDSTORAGE => "SWITCHED_TO_CLOUD_STORAGE",
            BackupConfigurationTransactionalLogStorageStateEnum::CLOUDSTORAGE => "CLOUD_STORAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupConfigurationTransactionalLogStorageStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSACTIONAL_LOG_STORAGE_STATE_UNSPECIFIED" => Ok(BackupConfigurationTransactionalLogStorageStateEnum::TRANSACTIONALLOGSTORAGESTATEUNSPECIFIED),
           "DISK" => Ok(BackupConfigurationTransactionalLogStorageStateEnum::DISK),
           "SWITCHING_TO_CLOUD_STORAGE" => Ok(BackupConfigurationTransactionalLogStorageStateEnum::SWITCHINGTOCLOUDSTORAGE),
           "SWITCHED_TO_CLOUD_STORAGE" => Ok(BackupConfigurationTransactionalLogStorageStateEnum::SWITCHEDTOCLOUDSTORAGE),
           "CLOUD_STORAGE" => Ok(BackupConfigurationTransactionalLogStorageStateEnum::CLOUDSTORAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupConfigurationTransactionalLogStorageStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupReencryptionConfigBackupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of backups users want to re-encrypt.
pub enum BackupReencryptionConfigBackupTypeEnum {
    

    /// Unknown backup type, will be defaulted to AUTOMATIC backup type
    ///
    /// "BACKUP_TYPE_UNSPECIFIED"
    #[serde(rename="BACKUP_TYPE_UNSPECIFIED")]
    BACKUPTYPEUNSPECIFIED,
    

    /// Reencrypt automatic backups
    ///
    /// "AUTOMATED"
    #[serde(rename="AUTOMATED")]
    AUTOMATED,
    

    /// Reencrypt on-demand backups
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
}

impl AsRef<str> for BackupReencryptionConfigBackupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupReencryptionConfigBackupTypeEnum::BACKUPTYPEUNSPECIFIED => "BACKUP_TYPE_UNSPECIFIED",
            BackupReencryptionConfigBackupTypeEnum::AUTOMATED => "AUTOMATED",
            BackupReencryptionConfigBackupTypeEnum::ONDEMAND => "ON_DEMAND",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupReencryptionConfigBackupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BACKUP_TYPE_UNSPECIFIED" => Ok(BackupReencryptionConfigBackupTypeEnum::BACKUPTYPEUNSPECIFIED),
           "AUTOMATED" => Ok(BackupReencryptionConfigBackupTypeEnum::AUTOMATED),
           "ON_DEMAND" => Ok(BackupReencryptionConfigBackupTypeEnum::ONDEMAND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupReencryptionConfigBackupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupRetentionSettingRetentionUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The unit that 'retained_backups' represents.
pub enum BackupRetentionSettingRetentionUnitEnum {
    

    /// Backup retention unit is unspecified, will be treated as COUNT.
    ///
    /// "RETENTION_UNIT_UNSPECIFIED"
    #[serde(rename="RETENTION_UNIT_UNSPECIFIED")]
    RETENTIONUNITUNSPECIFIED,
    

    /// Retention will be by count, eg. "retain the most recent 7 backups".
    ///
    /// "COUNT"
    #[serde(rename="COUNT")]
    COUNT,
}

impl AsRef<str> for BackupRetentionSettingRetentionUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupRetentionSettingRetentionUnitEnum::RETENTIONUNITUNSPECIFIED => "RETENTION_UNIT_UNSPECIFIED",
            BackupRetentionSettingRetentionUnitEnum::COUNT => "COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupRetentionSettingRetentionUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RETENTION_UNIT_UNSPECIFIED" => Ok(BackupRetentionSettingRetentionUnitEnum::RETENTIONUNITUNSPECIFIED),
           "COUNT" => Ok(BackupRetentionSettingRetentionUnitEnum::COUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupRetentionSettingRetentionUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupRunBackupKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT.
pub enum BackupRunBackupKindEnum {
    

    /// This is an unknown BackupKind.
    ///
    /// "SQL_BACKUP_KIND_UNSPECIFIED"
    #[serde(rename="SQL_BACKUP_KIND_UNSPECIFIED")]
    SQLBACKUPKINDUNSPECIFIED,
    

    /// The snapshot based backups
    ///
    /// "SNAPSHOT"
    #[serde(rename="SNAPSHOT")]
    SNAPSHOT,
    

    /// Physical backups
    ///
    /// "PHYSICAL"
    #[serde(rename="PHYSICAL")]
    PHYSICAL,
}

impl AsRef<str> for BackupRunBackupKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupRunBackupKindEnum::SQLBACKUPKINDUNSPECIFIED => "SQL_BACKUP_KIND_UNSPECIFIED",
            BackupRunBackupKindEnum::SNAPSHOT => "SNAPSHOT",
            BackupRunBackupKindEnum::PHYSICAL => "PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupRunBackupKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_BACKUP_KIND_UNSPECIFIED" => Ok(BackupRunBackupKindEnum::SQLBACKUPKINDUNSPECIFIED),
           "SNAPSHOT" => Ok(BackupRunBackupKindEnum::SNAPSHOT),
           "PHYSICAL" => Ok(BackupRunBackupKindEnum::PHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupRunBackupKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupRunStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of this run.
pub enum BackupRunStatusEnum {
    

    /// The status of the run is unknown.
    ///
    /// "SQL_BACKUP_RUN_STATUS_UNSPECIFIED"
    #[serde(rename="SQL_BACKUP_RUN_STATUS_UNSPECIFIED")]
    SQLBACKUPRUNSTATUSUNSPECIFIED,
    

    /// The backup operation was enqueued.
    ///
    /// "ENQUEUED"
    #[serde(rename="ENQUEUED")]
    ENQUEUED,
    

    /// The backup is overdue across a given backup window. Indicates a problem. Example: Long-running operation in progress during the whole window.
    ///
    /// "OVERDUE"
    #[serde(rename="OVERDUE")]
    OVERDUE,
    

    /// The backup is in progress.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The backup failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The backup was successful.
    ///
    /// "SUCCESSFUL"
    #[serde(rename="SUCCESSFUL")]
    SUCCESSFUL,
    

    /// The backup was skipped (without problems) for a given backup window. Example: Instance was idle.
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
    

    /// The backup is about to be deleted.
    ///
    /// "DELETION_PENDING"
    #[serde(rename="DELETION_PENDING")]
    DELETIONPENDING,
    

    /// The backup deletion failed.
    ///
    /// "DELETION_FAILED"
    #[serde(rename="DELETION_FAILED")]
    DELETIONFAILED,
    

    /// The backup has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for BackupRunStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupRunStatusEnum::SQLBACKUPRUNSTATUSUNSPECIFIED => "SQL_BACKUP_RUN_STATUS_UNSPECIFIED",
            BackupRunStatusEnum::ENQUEUED => "ENQUEUED",
            BackupRunStatusEnum::OVERDUE => "OVERDUE",
            BackupRunStatusEnum::RUNNING => "RUNNING",
            BackupRunStatusEnum::FAILED => "FAILED",
            BackupRunStatusEnum::SUCCESSFUL => "SUCCESSFUL",
            BackupRunStatusEnum::SKIPPED => "SKIPPED",
            BackupRunStatusEnum::DELETIONPENDING => "DELETION_PENDING",
            BackupRunStatusEnum::DELETIONFAILED => "DELETION_FAILED",
            BackupRunStatusEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupRunStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_BACKUP_RUN_STATUS_UNSPECIFIED" => Ok(BackupRunStatusEnum::SQLBACKUPRUNSTATUSUNSPECIFIED),
           "ENQUEUED" => Ok(BackupRunStatusEnum::ENQUEUED),
           "OVERDUE" => Ok(BackupRunStatusEnum::OVERDUE),
           "RUNNING" => Ok(BackupRunStatusEnum::RUNNING),
           "FAILED" => Ok(BackupRunStatusEnum::FAILED),
           "SUCCESSFUL" => Ok(BackupRunStatusEnum::SUCCESSFUL),
           "SKIPPED" => Ok(BackupRunStatusEnum::SKIPPED),
           "DELETION_PENDING" => Ok(BackupRunStatusEnum::DELETIONPENDING),
           "DELETION_FAILED" => Ok(BackupRunStatusEnum::DELETIONFAILED),
           "DELETED" => Ok(BackupRunStatusEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupRunStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupRunTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this run; can be either "AUTOMATED" or "ON_DEMAND" or "FINAL". This field defaults to "ON_DEMAND" and is ignored, when specified for insert requests.
pub enum BackupRunTypeEnum {
    

    /// This is an unknown BackupRun type.
    ///
    /// "SQL_BACKUP_RUN_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_BACKUP_RUN_TYPE_UNSPECIFIED")]
    SQLBACKUPRUNTYPEUNSPECIFIED,
    

    /// The backup schedule automatically triggers a backup.
    ///
    /// "AUTOMATED"
    #[serde(rename="AUTOMATED")]
    AUTOMATED,
    

    /// The user manually triggers a backup.
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
}

impl AsRef<str> for BackupRunTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupRunTypeEnum::SQLBACKUPRUNTYPEUNSPECIFIED => "SQL_BACKUP_RUN_TYPE_UNSPECIFIED",
            BackupRunTypeEnum::AUTOMATED => "AUTOMATED",
            BackupRunTypeEnum::ONDEMAND => "ON_DEMAND",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupRunTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_BACKUP_RUN_TYPE_UNSPECIFIED" => Ok(BackupRunTypeEnum::SQLBACKUPRUNTYPEUNSPECIFIED),
           "AUTOMATED" => Ok(BackupRunTypeEnum::AUTOMATED),
           "ON_DEMAND" => Ok(BackupRunTypeEnum::ONDEMAND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupRunTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectSettingBackendTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type.
pub enum ConnectSettingBackendTypeEnum {
    

    /// This is an unknown backend type for instance.
    ///
    /// "SQL_BACKEND_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_BACKEND_TYPE_UNSPECIFIED")]
    SQLBACKENDTYPEUNSPECIFIED,
    

    /// V1 speckle instance.
    ///
    /// "FIRST_GEN"
    #[serde(rename="FIRST_GEN")]
    FIRSTGEN,
    

    /// V2 speckle instance.
    ///
    /// "SECOND_GEN"
    #[serde(rename="SECOND_GEN")]
    SECONDGEN,
    

    /// On premises instance.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
}

impl AsRef<str> for ConnectSettingBackendTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectSettingBackendTypeEnum::SQLBACKENDTYPEUNSPECIFIED => "SQL_BACKEND_TYPE_UNSPECIFIED",
            ConnectSettingBackendTypeEnum::FIRSTGEN => "FIRST_GEN",
            ConnectSettingBackendTypeEnum::SECONDGEN => "SECOND_GEN",
            ConnectSettingBackendTypeEnum::EXTERNAL => "EXTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectSettingBackendTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_BACKEND_TYPE_UNSPECIFIED" => Ok(ConnectSettingBackendTypeEnum::SQLBACKENDTYPEUNSPECIFIED),
           "FIRST_GEN" => Ok(ConnectSettingBackendTypeEnum::FIRSTGEN),
           "SECOND_GEN" => Ok(ConnectSettingBackendTypeEnum::SECONDGEN),
           "EXTERNAL" => Ok(ConnectSettingBackendTypeEnum::EXTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectSettingBackendTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectSettingDatabaseVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database engine type and version. The `databaseVersion` field cannot be changed after instance creation. MySQL instances: `MYSQL_8_0`, `MYSQL_5_7` (default), or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11`, `POSTGRES_12` (default), `POSTGRES_13`, or `POSTGRES_14`. SQL Server instances: `SQLSERVER_2017_STANDARD` (default), `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`.
pub enum ConnectSettingDatabaseVersionEnum {
    

    /// This is an unknown database version.
    ///
    /// "SQL_DATABASE_VERSION_UNSPECIFIED"
    #[serde(rename="SQL_DATABASE_VERSION_UNSPECIFIED")]
    SQLDATABASEVERSIONUNSPECIFIED,
    

    /// The database version is MySQL 5.1.
    ///
    /// "MYSQL_5_1"
    #[serde(rename="MYSQL_5_1")]
    MYSQL51,
    

    /// The database version is MySQL 5.5.
    ///
    /// "MYSQL_5_5"
    #[serde(rename="MYSQL_5_5")]
    MYSQL55,
    

    /// The database version is MySQL 5.6.
    ///
    /// "MYSQL_5_6"
    #[serde(rename="MYSQL_5_6")]
    MYSQL56,
    

    /// The database version is MySQL 5.7.
    ///
    /// "MYSQL_5_7"
    #[serde(rename="MYSQL_5_7")]
    MYSQL57,
    

    /// The database version is SQL Server 2017 Standard.
    ///
    /// "SQLSERVER_2017_STANDARD"
    #[serde(rename="SQLSERVER_2017_STANDARD")]
    SQLSERVER2017STANDARD,
    

    /// The database version is SQL Server 2017 Enterprise.
    ///
    /// "SQLSERVER_2017_ENTERPRISE"
    #[serde(rename="SQLSERVER_2017_ENTERPRISE")]
    SQLSERVER2017ENTERPRISE,
    

    /// The database version is SQL Server 2017 Express.
    ///
    /// "SQLSERVER_2017_EXPRESS"
    #[serde(rename="SQLSERVER_2017_EXPRESS")]
    SQLSERVER2017EXPRESS,
    

    /// The database version is SQL Server 2017 Web.
    ///
    /// "SQLSERVER_2017_WEB"
    #[serde(rename="SQLSERVER_2017_WEB")]
    SQLSERVER2017WEB,
    

    /// The database version is PostgreSQL 9.6.
    ///
    /// "POSTGRES_9_6"
    #[serde(rename="POSTGRES_9_6")]
    POSTGRES96,
    

    /// The database version is PostgreSQL 10.
    ///
    /// "POSTGRES_10"
    #[serde(rename="POSTGRES_10")]
    POSTGRES10,
    

    /// The database version is PostgreSQL 11.
    ///
    /// "POSTGRES_11"
    #[serde(rename="POSTGRES_11")]
    POSTGRES11,
    

    /// The database version is PostgreSQL 12.
    ///
    /// "POSTGRES_12"
    #[serde(rename="POSTGRES_12")]
    POSTGRES12,
    

    /// The database version is PostgreSQL 13.
    ///
    /// "POSTGRES_13"
    #[serde(rename="POSTGRES_13")]
    POSTGRES13,
    

    /// The database version is PostgreSQL 14.
    ///
    /// "POSTGRES_14"
    #[serde(rename="POSTGRES_14")]
    POSTGRES14,
    

    /// The database version is PostgreSQL 15.
    ///
    /// "POSTGRES_15"
    #[serde(rename="POSTGRES_15")]
    POSTGRES15,
    

    /// The database version is MySQL 8.
    ///
    /// "MYSQL_8_0"
    #[serde(rename="MYSQL_8_0")]
    MYSQL80,
    

    /// The database major version is MySQL 8.0 and the minor version is 18.
    ///
    /// "MYSQL_8_0_18"
    #[serde(rename="MYSQL_8_0_18")]
    MYSQL8018,
    

    /// The database major version is MySQL 8.0 and the minor version is 26.
    ///
    /// "MYSQL_8_0_26"
    #[serde(rename="MYSQL_8_0_26")]
    MYSQL8026,
    

    /// The database major version is MySQL 8.0 and the minor version is 27.
    ///
    /// "MYSQL_8_0_27"
    #[serde(rename="MYSQL_8_0_27")]
    MYSQL8027,
    

    /// The database major version is MySQL 8.0 and the minor version is 28.
    ///
    /// "MYSQL_8_0_28"
    #[serde(rename="MYSQL_8_0_28")]
    MYSQL8028,
    

    /// The database major version is MySQL 8.0 and the minor version is 29.
    ///
    /// "MYSQL_8_0_29"
    #[serde(rename="MYSQL_8_0_29")]
    MYSQL8029,
    

    /// The database major version is MySQL 8.0 and the minor version is 30.
    ///
    /// "MYSQL_8_0_30"
    #[serde(rename="MYSQL_8_0_30")]
    MYSQL8030,
    

    /// The database major version is MySQL 8.0 and the minor version is 31.
    ///
    /// "MYSQL_8_0_31"
    #[serde(rename="MYSQL_8_0_31")]
    MYSQL8031,
    

    /// The database major version is MySQL 8.0 and the minor version is 32.
    ///
    /// "MYSQL_8_0_32"
    #[serde(rename="MYSQL_8_0_32")]
    MYSQL8032,
    

    /// The database major version is MySQL 8.0 and the minor version is 33.
    ///
    /// "MYSQL_8_0_33"
    #[serde(rename="MYSQL_8_0_33")]
    MYSQL8033,
    

    /// The database major version is MySQL 8.0 and the minor version is 34.
    ///
    /// "MYSQL_8_0_34"
    #[serde(rename="MYSQL_8_0_34")]
    MYSQL8034,
    

    /// The database major version is MySQL 8.0 and the minor version is 35.
    ///
    /// "MYSQL_8_0_35"
    #[serde(rename="MYSQL_8_0_35")]
    MYSQL8035,
    

    /// The database major version is MySQL 8.0 and the minor version is 36.
    ///
    /// "MYSQL_8_0_36"
    #[serde(rename="MYSQL_8_0_36")]
    MYSQL8036,
    

    /// The database major version is MySQL 8.0 and the minor version is 37.
    ///
    /// "MYSQL_8_0_37"
    #[serde(rename="MYSQL_8_0_37")]
    MYSQL8037,
    

    /// The database major version is MySQL 8.0 and the minor version is 38.
    ///
    /// "MYSQL_8_0_38"
    #[serde(rename="MYSQL_8_0_38")]
    MYSQL8038,
    

    /// The database major version is MySQL 8.0 and the minor version is 39.
    ///
    /// "MYSQL_8_0_39"
    #[serde(rename="MYSQL_8_0_39")]
    MYSQL8039,
    

    /// The database major version is MySQL 8.0 and the minor version is 40.
    ///
    /// "MYSQL_8_0_40"
    #[serde(rename="MYSQL_8_0_40")]
    MYSQL8040,
    

    /// The database version is SQL Server 2019 Standard.
    ///
    /// "SQLSERVER_2019_STANDARD"
    #[serde(rename="SQLSERVER_2019_STANDARD")]
    SQLSERVER2019STANDARD,
    

    /// The database version is SQL Server 2019 Enterprise.
    ///
    /// "SQLSERVER_2019_ENTERPRISE"
    #[serde(rename="SQLSERVER_2019_ENTERPRISE")]
    SQLSERVER2019ENTERPRISE,
    

    /// The database version is SQL Server 2019 Express.
    ///
    /// "SQLSERVER_2019_EXPRESS"
    #[serde(rename="SQLSERVER_2019_EXPRESS")]
    SQLSERVER2019EXPRESS,
    

    /// The database version is SQL Server 2019 Web.
    ///
    /// "SQLSERVER_2019_WEB"
    #[serde(rename="SQLSERVER_2019_WEB")]
    SQLSERVER2019WEB,
    

    /// The database version is SQL Server 2022 Standard.
    ///
    /// "SQLSERVER_2022_STANDARD"
    #[serde(rename="SQLSERVER_2022_STANDARD")]
    SQLSERVER2022STANDARD,
    

    /// The database version is SQL Server 2022 Enterprise.
    ///
    /// "SQLSERVER_2022_ENTERPRISE"
    #[serde(rename="SQLSERVER_2022_ENTERPRISE")]
    SQLSERVER2022ENTERPRISE,
    

    /// The database version is SQL Server 2022 Express.
    ///
    /// "SQLSERVER_2022_EXPRESS"
    #[serde(rename="SQLSERVER_2022_EXPRESS")]
    SQLSERVER2022EXPRESS,
    

    /// The database version is SQL Server 2022 Web.
    ///
    /// "SQLSERVER_2022_WEB"
    #[serde(rename="SQLSERVER_2022_WEB")]
    SQLSERVER2022WEB,
}

impl AsRef<str> for ConnectSettingDatabaseVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectSettingDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED => "SQL_DATABASE_VERSION_UNSPECIFIED",
            ConnectSettingDatabaseVersionEnum::MYSQL51 => "MYSQL_5_1",
            ConnectSettingDatabaseVersionEnum::MYSQL55 => "MYSQL_5_5",
            ConnectSettingDatabaseVersionEnum::MYSQL56 => "MYSQL_5_6",
            ConnectSettingDatabaseVersionEnum::MYSQL57 => "MYSQL_5_7",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2017STANDARD => "SQLSERVER_2017_STANDARD",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2017ENTERPRISE => "SQLSERVER_2017_ENTERPRISE",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2017EXPRESS => "SQLSERVER_2017_EXPRESS",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2017WEB => "SQLSERVER_2017_WEB",
            ConnectSettingDatabaseVersionEnum::POSTGRES96 => "POSTGRES_9_6",
            ConnectSettingDatabaseVersionEnum::POSTGRES10 => "POSTGRES_10",
            ConnectSettingDatabaseVersionEnum::POSTGRES11 => "POSTGRES_11",
            ConnectSettingDatabaseVersionEnum::POSTGRES12 => "POSTGRES_12",
            ConnectSettingDatabaseVersionEnum::POSTGRES13 => "POSTGRES_13",
            ConnectSettingDatabaseVersionEnum::POSTGRES14 => "POSTGRES_14",
            ConnectSettingDatabaseVersionEnum::POSTGRES15 => "POSTGRES_15",
            ConnectSettingDatabaseVersionEnum::MYSQL80 => "MYSQL_8_0",
            ConnectSettingDatabaseVersionEnum::MYSQL8018 => "MYSQL_8_0_18",
            ConnectSettingDatabaseVersionEnum::MYSQL8026 => "MYSQL_8_0_26",
            ConnectSettingDatabaseVersionEnum::MYSQL8027 => "MYSQL_8_0_27",
            ConnectSettingDatabaseVersionEnum::MYSQL8028 => "MYSQL_8_0_28",
            ConnectSettingDatabaseVersionEnum::MYSQL8029 => "MYSQL_8_0_29",
            ConnectSettingDatabaseVersionEnum::MYSQL8030 => "MYSQL_8_0_30",
            ConnectSettingDatabaseVersionEnum::MYSQL8031 => "MYSQL_8_0_31",
            ConnectSettingDatabaseVersionEnum::MYSQL8032 => "MYSQL_8_0_32",
            ConnectSettingDatabaseVersionEnum::MYSQL8033 => "MYSQL_8_0_33",
            ConnectSettingDatabaseVersionEnum::MYSQL8034 => "MYSQL_8_0_34",
            ConnectSettingDatabaseVersionEnum::MYSQL8035 => "MYSQL_8_0_35",
            ConnectSettingDatabaseVersionEnum::MYSQL8036 => "MYSQL_8_0_36",
            ConnectSettingDatabaseVersionEnum::MYSQL8037 => "MYSQL_8_0_37",
            ConnectSettingDatabaseVersionEnum::MYSQL8038 => "MYSQL_8_0_38",
            ConnectSettingDatabaseVersionEnum::MYSQL8039 => "MYSQL_8_0_39",
            ConnectSettingDatabaseVersionEnum::MYSQL8040 => "MYSQL_8_0_40",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2019STANDARD => "SQLSERVER_2019_STANDARD",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2019ENTERPRISE => "SQLSERVER_2019_ENTERPRISE",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2019EXPRESS => "SQLSERVER_2019_EXPRESS",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2019WEB => "SQLSERVER_2019_WEB",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2022STANDARD => "SQLSERVER_2022_STANDARD",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2022ENTERPRISE => "SQLSERVER_2022_ENTERPRISE",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2022EXPRESS => "SQLSERVER_2022_EXPRESS",
            ConnectSettingDatabaseVersionEnum::SQLSERVER2022WEB => "SQLSERVER_2022_WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectSettingDatabaseVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_DATABASE_VERSION_UNSPECIFIED" => Ok(ConnectSettingDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED),
           "MYSQL_5_1" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL51),
           "MYSQL_5_5" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL55),
           "MYSQL_5_6" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL56),
           "MYSQL_5_7" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL57),
           "SQLSERVER_2017_STANDARD" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2017STANDARD),
           "SQLSERVER_2017_ENTERPRISE" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2017ENTERPRISE),
           "SQLSERVER_2017_EXPRESS" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2017EXPRESS),
           "SQLSERVER_2017_WEB" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2017WEB),
           "POSTGRES_9_6" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES96),
           "POSTGRES_10" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES10),
           "POSTGRES_11" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES11),
           "POSTGRES_12" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES12),
           "POSTGRES_13" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES13),
           "POSTGRES_14" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES14),
           "POSTGRES_15" => Ok(ConnectSettingDatabaseVersionEnum::POSTGRES15),
           "MYSQL_8_0" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL80),
           "MYSQL_8_0_18" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8018),
           "MYSQL_8_0_26" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8026),
           "MYSQL_8_0_27" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8027),
           "MYSQL_8_0_28" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8028),
           "MYSQL_8_0_29" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8029),
           "MYSQL_8_0_30" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8030),
           "MYSQL_8_0_31" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8031),
           "MYSQL_8_0_32" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8032),
           "MYSQL_8_0_33" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8033),
           "MYSQL_8_0_34" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8034),
           "MYSQL_8_0_35" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8035),
           "MYSQL_8_0_36" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8036),
           "MYSQL_8_0_37" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8037),
           "MYSQL_8_0_38" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8038),
           "MYSQL_8_0_39" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8039),
           "MYSQL_8_0_40" => Ok(ConnectSettingDatabaseVersionEnum::MYSQL8040),
           "SQLSERVER_2019_STANDARD" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2019STANDARD),
           "SQLSERVER_2019_ENTERPRISE" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2019ENTERPRISE),
           "SQLSERVER_2019_EXPRESS" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2019EXPRESS),
           "SQLSERVER_2019_WEB" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2019WEB),
           "SQLSERVER_2022_STANDARD" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2022STANDARD),
           "SQLSERVER_2022_ENTERPRISE" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2022ENTERPRISE),
           "SQLSERVER_2022_EXPRESS" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2022EXPRESS),
           "SQLSERVER_2022_WEB" => Ok(ConnectSettingDatabaseVersionEnum::SQLSERVER2022WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectSettingDatabaseVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseInstanceBackendTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The backend type. `SECOND_GEN`: Cloud SQL database instance. `EXTERNAL`: A database server that is not managed by Google. This property is read-only; use the `tier` property in the `settings` object to determine the database type.
pub enum DatabaseInstanceBackendTypeEnum {
    

    /// This is an unknown backend type for instance.
    ///
    /// "SQL_BACKEND_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_BACKEND_TYPE_UNSPECIFIED")]
    SQLBACKENDTYPEUNSPECIFIED,
    

    /// V1 speckle instance.
    ///
    /// "FIRST_GEN"
    #[serde(rename="FIRST_GEN")]
    FIRSTGEN,
    

    /// V2 speckle instance.
    ///
    /// "SECOND_GEN"
    #[serde(rename="SECOND_GEN")]
    SECONDGEN,
    

    /// On premises instance.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
}

impl AsRef<str> for DatabaseInstanceBackendTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceBackendTypeEnum::SQLBACKENDTYPEUNSPECIFIED => "SQL_BACKEND_TYPE_UNSPECIFIED",
            DatabaseInstanceBackendTypeEnum::FIRSTGEN => "FIRST_GEN",
            DatabaseInstanceBackendTypeEnum::SECONDGEN => "SECOND_GEN",
            DatabaseInstanceBackendTypeEnum::EXTERNAL => "EXTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceBackendTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_BACKEND_TYPE_UNSPECIFIED" => Ok(DatabaseInstanceBackendTypeEnum::SQLBACKENDTYPEUNSPECIFIED),
           "FIRST_GEN" => Ok(DatabaseInstanceBackendTypeEnum::FIRSTGEN),
           "SECOND_GEN" => Ok(DatabaseInstanceBackendTypeEnum::SECONDGEN),
           "EXTERNAL" => Ok(DatabaseInstanceBackendTypeEnum::EXTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceBackendTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseInstanceDatabaseVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database engine type and version. The `databaseVersion` field cannot be changed after instance creation.
pub enum DatabaseInstanceDatabaseVersionEnum {
    

    /// This is an unknown database version.
    ///
    /// "SQL_DATABASE_VERSION_UNSPECIFIED"
    #[serde(rename="SQL_DATABASE_VERSION_UNSPECIFIED")]
    SQLDATABASEVERSIONUNSPECIFIED,
    

    /// The database version is MySQL 5.1.
    ///
    /// "MYSQL_5_1"
    #[serde(rename="MYSQL_5_1")]
    MYSQL51,
    

    /// The database version is MySQL 5.5.
    ///
    /// "MYSQL_5_5"
    #[serde(rename="MYSQL_5_5")]
    MYSQL55,
    

    /// The database version is MySQL 5.6.
    ///
    /// "MYSQL_5_6"
    #[serde(rename="MYSQL_5_6")]
    MYSQL56,
    

    /// The database version is MySQL 5.7.
    ///
    /// "MYSQL_5_7"
    #[serde(rename="MYSQL_5_7")]
    MYSQL57,
    

    /// The database version is SQL Server 2017 Standard.
    ///
    /// "SQLSERVER_2017_STANDARD"
    #[serde(rename="SQLSERVER_2017_STANDARD")]
    SQLSERVER2017STANDARD,
    

    /// The database version is SQL Server 2017 Enterprise.
    ///
    /// "SQLSERVER_2017_ENTERPRISE"
    #[serde(rename="SQLSERVER_2017_ENTERPRISE")]
    SQLSERVER2017ENTERPRISE,
    

    /// The database version is SQL Server 2017 Express.
    ///
    /// "SQLSERVER_2017_EXPRESS"
    #[serde(rename="SQLSERVER_2017_EXPRESS")]
    SQLSERVER2017EXPRESS,
    

    /// The database version is SQL Server 2017 Web.
    ///
    /// "SQLSERVER_2017_WEB"
    #[serde(rename="SQLSERVER_2017_WEB")]
    SQLSERVER2017WEB,
    

    /// The database version is PostgreSQL 9.6.
    ///
    /// "POSTGRES_9_6"
    #[serde(rename="POSTGRES_9_6")]
    POSTGRES96,
    

    /// The database version is PostgreSQL 10.
    ///
    /// "POSTGRES_10"
    #[serde(rename="POSTGRES_10")]
    POSTGRES10,
    

    /// The database version is PostgreSQL 11.
    ///
    /// "POSTGRES_11"
    #[serde(rename="POSTGRES_11")]
    POSTGRES11,
    

    /// The database version is PostgreSQL 12.
    ///
    /// "POSTGRES_12"
    #[serde(rename="POSTGRES_12")]
    POSTGRES12,
    

    /// The database version is PostgreSQL 13.
    ///
    /// "POSTGRES_13"
    #[serde(rename="POSTGRES_13")]
    POSTGRES13,
    

    /// The database version is PostgreSQL 14.
    ///
    /// "POSTGRES_14"
    #[serde(rename="POSTGRES_14")]
    POSTGRES14,
    

    /// The database version is PostgreSQL 15.
    ///
    /// "POSTGRES_15"
    #[serde(rename="POSTGRES_15")]
    POSTGRES15,
    

    /// The database version is MySQL 8.
    ///
    /// "MYSQL_8_0"
    #[serde(rename="MYSQL_8_0")]
    MYSQL80,
    

    /// The database major version is MySQL 8.0 and the minor version is 18.
    ///
    /// "MYSQL_8_0_18"
    #[serde(rename="MYSQL_8_0_18")]
    MYSQL8018,
    

    /// The database major version is MySQL 8.0 and the minor version is 26.
    ///
    /// "MYSQL_8_0_26"
    #[serde(rename="MYSQL_8_0_26")]
    MYSQL8026,
    

    /// The database major version is MySQL 8.0 and the minor version is 27.
    ///
    /// "MYSQL_8_0_27"
    #[serde(rename="MYSQL_8_0_27")]
    MYSQL8027,
    

    /// The database major version is MySQL 8.0 and the minor version is 28.
    ///
    /// "MYSQL_8_0_28"
    #[serde(rename="MYSQL_8_0_28")]
    MYSQL8028,
    

    /// The database major version is MySQL 8.0 and the minor version is 29.
    ///
    /// "MYSQL_8_0_29"
    #[serde(rename="MYSQL_8_0_29")]
    MYSQL8029,
    

    /// The database major version is MySQL 8.0 and the minor version is 30.
    ///
    /// "MYSQL_8_0_30"
    #[serde(rename="MYSQL_8_0_30")]
    MYSQL8030,
    

    /// The database major version is MySQL 8.0 and the minor version is 31.
    ///
    /// "MYSQL_8_0_31"
    #[serde(rename="MYSQL_8_0_31")]
    MYSQL8031,
    

    /// The database major version is MySQL 8.0 and the minor version is 32.
    ///
    /// "MYSQL_8_0_32"
    #[serde(rename="MYSQL_8_0_32")]
    MYSQL8032,
    

    /// The database major version is MySQL 8.0 and the minor version is 33.
    ///
    /// "MYSQL_8_0_33"
    #[serde(rename="MYSQL_8_0_33")]
    MYSQL8033,
    

    /// The database major version is MySQL 8.0 and the minor version is 34.
    ///
    /// "MYSQL_8_0_34"
    #[serde(rename="MYSQL_8_0_34")]
    MYSQL8034,
    

    /// The database major version is MySQL 8.0 and the minor version is 35.
    ///
    /// "MYSQL_8_0_35"
    #[serde(rename="MYSQL_8_0_35")]
    MYSQL8035,
    

    /// The database major version is MySQL 8.0 and the minor version is 36.
    ///
    /// "MYSQL_8_0_36"
    #[serde(rename="MYSQL_8_0_36")]
    MYSQL8036,
    

    /// The database major version is MySQL 8.0 and the minor version is 37.
    ///
    /// "MYSQL_8_0_37"
    #[serde(rename="MYSQL_8_0_37")]
    MYSQL8037,
    

    /// The database major version is MySQL 8.0 and the minor version is 38.
    ///
    /// "MYSQL_8_0_38"
    #[serde(rename="MYSQL_8_0_38")]
    MYSQL8038,
    

    /// The database major version is MySQL 8.0 and the minor version is 39.
    ///
    /// "MYSQL_8_0_39"
    #[serde(rename="MYSQL_8_0_39")]
    MYSQL8039,
    

    /// The database major version is MySQL 8.0 and the minor version is 40.
    ///
    /// "MYSQL_8_0_40"
    #[serde(rename="MYSQL_8_0_40")]
    MYSQL8040,
    

    /// The database version is SQL Server 2019 Standard.
    ///
    /// "SQLSERVER_2019_STANDARD"
    #[serde(rename="SQLSERVER_2019_STANDARD")]
    SQLSERVER2019STANDARD,
    

    /// The database version is SQL Server 2019 Enterprise.
    ///
    /// "SQLSERVER_2019_ENTERPRISE"
    #[serde(rename="SQLSERVER_2019_ENTERPRISE")]
    SQLSERVER2019ENTERPRISE,
    

    /// The database version is SQL Server 2019 Express.
    ///
    /// "SQLSERVER_2019_EXPRESS"
    #[serde(rename="SQLSERVER_2019_EXPRESS")]
    SQLSERVER2019EXPRESS,
    

    /// The database version is SQL Server 2019 Web.
    ///
    /// "SQLSERVER_2019_WEB"
    #[serde(rename="SQLSERVER_2019_WEB")]
    SQLSERVER2019WEB,
    

    /// The database version is SQL Server 2022 Standard.
    ///
    /// "SQLSERVER_2022_STANDARD"
    #[serde(rename="SQLSERVER_2022_STANDARD")]
    SQLSERVER2022STANDARD,
    

    /// The database version is SQL Server 2022 Enterprise.
    ///
    /// "SQLSERVER_2022_ENTERPRISE"
    #[serde(rename="SQLSERVER_2022_ENTERPRISE")]
    SQLSERVER2022ENTERPRISE,
    

    /// The database version is SQL Server 2022 Express.
    ///
    /// "SQLSERVER_2022_EXPRESS"
    #[serde(rename="SQLSERVER_2022_EXPRESS")]
    SQLSERVER2022EXPRESS,
    

    /// The database version is SQL Server 2022 Web.
    ///
    /// "SQLSERVER_2022_WEB"
    #[serde(rename="SQLSERVER_2022_WEB")]
    SQLSERVER2022WEB,
}

impl AsRef<str> for DatabaseInstanceDatabaseVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED => "SQL_DATABASE_VERSION_UNSPECIFIED",
            DatabaseInstanceDatabaseVersionEnum::MYSQL51 => "MYSQL_5_1",
            DatabaseInstanceDatabaseVersionEnum::MYSQL55 => "MYSQL_5_5",
            DatabaseInstanceDatabaseVersionEnum::MYSQL56 => "MYSQL_5_6",
            DatabaseInstanceDatabaseVersionEnum::MYSQL57 => "MYSQL_5_7",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017STANDARD => "SQLSERVER_2017_STANDARD",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017ENTERPRISE => "SQLSERVER_2017_ENTERPRISE",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017EXPRESS => "SQLSERVER_2017_EXPRESS",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017WEB => "SQLSERVER_2017_WEB",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES96 => "POSTGRES_9_6",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES10 => "POSTGRES_10",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES11 => "POSTGRES_11",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES12 => "POSTGRES_12",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES13 => "POSTGRES_13",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES14 => "POSTGRES_14",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES15 => "POSTGRES_15",
            DatabaseInstanceDatabaseVersionEnum::MYSQL80 => "MYSQL_8_0",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8018 => "MYSQL_8_0_18",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8026 => "MYSQL_8_0_26",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8027 => "MYSQL_8_0_27",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8028 => "MYSQL_8_0_28",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8029 => "MYSQL_8_0_29",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8030 => "MYSQL_8_0_30",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8031 => "MYSQL_8_0_31",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8032 => "MYSQL_8_0_32",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8033 => "MYSQL_8_0_33",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8034 => "MYSQL_8_0_34",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8035 => "MYSQL_8_0_35",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8036 => "MYSQL_8_0_36",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8037 => "MYSQL_8_0_37",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8038 => "MYSQL_8_0_38",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8039 => "MYSQL_8_0_39",
            DatabaseInstanceDatabaseVersionEnum::MYSQL8040 => "MYSQL_8_0_40",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019STANDARD => "SQLSERVER_2019_STANDARD",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019ENTERPRISE => "SQLSERVER_2019_ENTERPRISE",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019EXPRESS => "SQLSERVER_2019_EXPRESS",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019WEB => "SQLSERVER_2019_WEB",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022STANDARD => "SQLSERVER_2022_STANDARD",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022ENTERPRISE => "SQLSERVER_2022_ENTERPRISE",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022EXPRESS => "SQLSERVER_2022_EXPRESS",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022WEB => "SQLSERVER_2022_WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceDatabaseVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_DATABASE_VERSION_UNSPECIFIED" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED),
           "MYSQL_5_1" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL51),
           "MYSQL_5_5" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL55),
           "MYSQL_5_6" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL56),
           "MYSQL_5_7" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL57),
           "SQLSERVER_2017_STANDARD" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017STANDARD),
           "SQLSERVER_2017_ENTERPRISE" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017ENTERPRISE),
           "SQLSERVER_2017_EXPRESS" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017EXPRESS),
           "SQLSERVER_2017_WEB" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017WEB),
           "POSTGRES_9_6" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES96),
           "POSTGRES_10" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES10),
           "POSTGRES_11" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES11),
           "POSTGRES_12" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES12),
           "POSTGRES_13" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES13),
           "POSTGRES_14" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES14),
           "POSTGRES_15" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES15),
           "MYSQL_8_0" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL80),
           "MYSQL_8_0_18" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8018),
           "MYSQL_8_0_26" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8026),
           "MYSQL_8_0_27" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8027),
           "MYSQL_8_0_28" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8028),
           "MYSQL_8_0_29" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8029),
           "MYSQL_8_0_30" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8030),
           "MYSQL_8_0_31" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8031),
           "MYSQL_8_0_32" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8032),
           "MYSQL_8_0_33" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8033),
           "MYSQL_8_0_34" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8034),
           "MYSQL_8_0_35" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8035),
           "MYSQL_8_0_36" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8036),
           "MYSQL_8_0_37" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8037),
           "MYSQL_8_0_38" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8038),
           "MYSQL_8_0_39" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8039),
           "MYSQL_8_0_40" => Ok(DatabaseInstanceDatabaseVersionEnum::MYSQL8040),
           "SQLSERVER_2019_STANDARD" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019STANDARD),
           "SQLSERVER_2019_ENTERPRISE" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019ENTERPRISE),
           "SQLSERVER_2019_EXPRESS" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019EXPRESS),
           "SQLSERVER_2019_WEB" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2019WEB),
           "SQLSERVER_2022_STANDARD" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022STANDARD),
           "SQLSERVER_2022_ENTERPRISE" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022ENTERPRISE),
           "SQLSERVER_2022_EXPRESS" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022EXPRESS),
           "SQLSERVER_2022_WEB" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2022WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceDatabaseVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseInstanceInstanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The instance type.
pub enum DatabaseInstanceInstanceTypeEnum {
    

    /// This is an unknown Cloud SQL instance type.
    ///
    /// "SQL_INSTANCE_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_INSTANCE_TYPE_UNSPECIFIED")]
    SQLINSTANCETYPEUNSPECIFIED,
    

    /// A regular Cloud SQL instance that is not replicating from a primary instance.
    ///
    /// "CLOUD_SQL_INSTANCE"
    #[serde(rename="CLOUD_SQL_INSTANCE")]
    CLOUDSQLINSTANCE,
    

    /// An instance running on the customer's premises that is not managed by Cloud SQL.
    ///
    /// "ON_PREMISES_INSTANCE"
    #[serde(rename="ON_PREMISES_INSTANCE")]
    ONPREMISESINSTANCE,
    

    /// A Cloud SQL instance acting as a read-replica.
    ///
    /// "READ_REPLICA_INSTANCE"
    #[serde(rename="READ_REPLICA_INSTANCE")]
    READREPLICAINSTANCE,
}

impl AsRef<str> for DatabaseInstanceInstanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceInstanceTypeEnum::SQLINSTANCETYPEUNSPECIFIED => "SQL_INSTANCE_TYPE_UNSPECIFIED",
            DatabaseInstanceInstanceTypeEnum::CLOUDSQLINSTANCE => "CLOUD_SQL_INSTANCE",
            DatabaseInstanceInstanceTypeEnum::ONPREMISESINSTANCE => "ON_PREMISES_INSTANCE",
            DatabaseInstanceInstanceTypeEnum::READREPLICAINSTANCE => "READ_REPLICA_INSTANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceInstanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_INSTANCE_TYPE_UNSPECIFIED" => Ok(DatabaseInstanceInstanceTypeEnum::SQLINSTANCETYPEUNSPECIFIED),
           "CLOUD_SQL_INSTANCE" => Ok(DatabaseInstanceInstanceTypeEnum::CLOUDSQLINSTANCE),
           "ON_PREMISES_INSTANCE" => Ok(DatabaseInstanceInstanceTypeEnum::ONPREMISESINSTANCE),
           "READ_REPLICA_INSTANCE" => Ok(DatabaseInstanceInstanceTypeEnum::READREPLICAINSTANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceInstanceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseInstanceSqlNetworkArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum DatabaseInstanceSqlNetworkArchitectureEnum {
    
    /// "SQL_NETWORK_ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="SQL_NETWORK_ARCHITECTURE_UNSPECIFIED")]
    SQLNETWORKARCHITECTUREUNSPECIFIED,
    

    /// The instance uses the new network architecture.
    ///
    /// "NEW_NETWORK_ARCHITECTURE"
    #[serde(rename="NEW_NETWORK_ARCHITECTURE")]
    NEWNETWORKARCHITECTURE,
    

    /// The instance uses the old network architecture.
    ///
    /// "OLD_NETWORK_ARCHITECTURE"
    #[serde(rename="OLD_NETWORK_ARCHITECTURE")]
    OLDNETWORKARCHITECTURE,
}

impl AsRef<str> for DatabaseInstanceSqlNetworkArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceSqlNetworkArchitectureEnum::SQLNETWORKARCHITECTUREUNSPECIFIED => "SQL_NETWORK_ARCHITECTURE_UNSPECIFIED",
            DatabaseInstanceSqlNetworkArchitectureEnum::NEWNETWORKARCHITECTURE => "NEW_NETWORK_ARCHITECTURE",
            DatabaseInstanceSqlNetworkArchitectureEnum::OLDNETWORKARCHITECTURE => "OLD_NETWORK_ARCHITECTURE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceSqlNetworkArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_NETWORK_ARCHITECTURE_UNSPECIFIED" => Ok(DatabaseInstanceSqlNetworkArchitectureEnum::SQLNETWORKARCHITECTUREUNSPECIFIED),
           "NEW_NETWORK_ARCHITECTURE" => Ok(DatabaseInstanceSqlNetworkArchitectureEnum::NEWNETWORKARCHITECTURE),
           "OLD_NETWORK_ARCHITECTURE" => Ok(DatabaseInstanceSqlNetworkArchitectureEnum::OLDNETWORKARCHITECTURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceSqlNetworkArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseInstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current serving state of the Cloud SQL instance.
pub enum DatabaseInstanceStateEnum {
    

    /// The state of the instance is unknown.
    ///
    /// "SQL_INSTANCE_STATE_UNSPECIFIED"
    #[serde(rename="SQL_INSTANCE_STATE_UNSPECIFIED")]
    SQLINSTANCESTATEUNSPECIFIED,
    

    /// The instance is running, or has been stopped by owner.
    ///
    /// "RUNNABLE"
    #[serde(rename="RUNNABLE")]
    RUNNABLE,
    

    /// The instance is not available, for example due to problems with billing.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// The instance is being deleted.
    ///
    /// "PENDING_DELETE"
    #[serde(rename="PENDING_DELETE")]
    PENDINGDELETE,
    

    /// The instance is being created.
    ///
    /// "PENDING_CREATE"
    #[serde(rename="PENDING_CREATE")]
    PENDINGCREATE,
    

    /// The instance is down for maintenance.
    ///
    /// "MAINTENANCE"
    #[serde(rename="MAINTENANCE")]
    MAINTENANCE,
    

    /// The creation of the instance failed or a fatal error occurred during maintenance.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Deprecated
    ///
    /// "ONLINE_MAINTENANCE"
    #[serde(rename="ONLINE_MAINTENANCE")]
    ONLINEMAINTENANCE,
}

impl AsRef<str> for DatabaseInstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceStateEnum::SQLINSTANCESTATEUNSPECIFIED => "SQL_INSTANCE_STATE_UNSPECIFIED",
            DatabaseInstanceStateEnum::RUNNABLE => "RUNNABLE",
            DatabaseInstanceStateEnum::SUSPENDED => "SUSPENDED",
            DatabaseInstanceStateEnum::PENDINGDELETE => "PENDING_DELETE",
            DatabaseInstanceStateEnum::PENDINGCREATE => "PENDING_CREATE",
            DatabaseInstanceStateEnum::MAINTENANCE => "MAINTENANCE",
            DatabaseInstanceStateEnum::FAILED => "FAILED",
            DatabaseInstanceStateEnum::ONLINEMAINTENANCE => "ONLINE_MAINTENANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_INSTANCE_STATE_UNSPECIFIED" => Ok(DatabaseInstanceStateEnum::SQLINSTANCESTATEUNSPECIFIED),
           "RUNNABLE" => Ok(DatabaseInstanceStateEnum::RUNNABLE),
           "SUSPENDED" => Ok(DatabaseInstanceStateEnum::SUSPENDED),
           "PENDING_DELETE" => Ok(DatabaseInstanceStateEnum::PENDINGDELETE),
           "PENDING_CREATE" => Ok(DatabaseInstanceStateEnum::PENDINGCREATE),
           "MAINTENANCE" => Ok(DatabaseInstanceStateEnum::MAINTENANCE),
           "FAILED" => Ok(DatabaseInstanceStateEnum::FAILED),
           "ONLINE_MAINTENANCE" => Ok(DatabaseInstanceStateEnum::ONLINEMAINTENANCE),
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


// region DatabaseInstanceSuspensionReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If the instance state is SUSPENDED, the reason for the suspension.
pub enum DatabaseInstanceSuspensionReasonEnum {
    

    /// This is an unknown suspension reason.
    ///
    /// "SQL_SUSPENSION_REASON_UNSPECIFIED"
    #[serde(rename="SQL_SUSPENSION_REASON_UNSPECIFIED")]
    SQLSUSPENSIONREASONUNSPECIFIED,
    

    /// The instance is suspended due to billing issues (for example:, GCP account issue)
    ///
    /// "BILLING_ISSUE"
    #[serde(rename="BILLING_ISSUE")]
    BILLINGISSUE,
    

    /// The instance is suspended due to illegal content (for example:, child pornography, copyrighted material, etc.).
    ///
    /// "LEGAL_ISSUE"
    #[serde(rename="LEGAL_ISSUE")]
    LEGALISSUE,
    

    /// The instance is causing operational issues (for example:, causing the database to crash).
    ///
    /// "OPERATIONAL_ISSUE"
    #[serde(rename="OPERATIONAL_ISSUE")]
    OPERATIONALISSUE,
    

    /// The KMS key used by the instance is either revoked or denied access to
    ///
    /// "KMS_KEY_ISSUE"
    #[serde(rename="KMS_KEY_ISSUE")]
    KMSKEYISSUE,
}

impl AsRef<str> for DatabaseInstanceSuspensionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceSuspensionReasonEnum::SQLSUSPENSIONREASONUNSPECIFIED => "SQL_SUSPENSION_REASON_UNSPECIFIED",
            DatabaseInstanceSuspensionReasonEnum::BILLINGISSUE => "BILLING_ISSUE",
            DatabaseInstanceSuspensionReasonEnum::LEGALISSUE => "LEGAL_ISSUE",
            DatabaseInstanceSuspensionReasonEnum::OPERATIONALISSUE => "OPERATIONAL_ISSUE",
            DatabaseInstanceSuspensionReasonEnum::KMSKEYISSUE => "KMS_KEY_ISSUE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseInstanceSuspensionReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_SUSPENSION_REASON_UNSPECIFIED" => Ok(DatabaseInstanceSuspensionReasonEnum::SQLSUSPENSIONREASONUNSPECIFIED),
           "BILLING_ISSUE" => Ok(DatabaseInstanceSuspensionReasonEnum::BILLINGISSUE),
           "LEGAL_ISSUE" => Ok(DatabaseInstanceSuspensionReasonEnum::LEGALISSUE),
           "OPERATIONAL_ISSUE" => Ok(DatabaseInstanceSuspensionReasonEnum::OPERATIONALISSUE),
           "KMS_KEY_ISSUE" => Ok(DatabaseInstanceSuspensionReasonEnum::KMSKEYISSUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseInstanceSuspensionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportContextFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file type for the specified uri.
pub enum ExportContextFileTypeEnum {
    

    /// Unknown file type.
    ///
    /// "SQL_FILE_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_FILE_TYPE_UNSPECIFIED")]
    SQLFILETYPEUNSPECIFIED,
    

    /// File containing SQL statements.
    ///
    /// "SQL"
    #[serde(rename="SQL")]
    SQL,
    

    /// File in CSV format.
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    
    /// "BAK"
    #[serde(rename="BAK")]
    BAK,
}

impl AsRef<str> for ExportContextFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportContextFileTypeEnum::SQLFILETYPEUNSPECIFIED => "SQL_FILE_TYPE_UNSPECIFIED",
            ExportContextFileTypeEnum::SQL => "SQL",
            ExportContextFileTypeEnum::CSV => "CSV",
            ExportContextFileTypeEnum::BAK => "BAK",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportContextFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_FILE_TYPE_UNSPECIFIED" => Ok(ExportContextFileTypeEnum::SQLFILETYPEUNSPECIFIED),
           "SQL" => Ok(ExportContextFileTypeEnum::SQL),
           "CSV" => Ok(ExportContextFileTypeEnum::CSV),
           "BAK" => Ok(ExportContextFileTypeEnum::BAK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportContextFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlagAppliesToEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database version this flag applies to. Can be MySQL instances: `MYSQL_8_0`, `MYSQL_8_0_18`, `MYSQL_8_0_26`, `MYSQL_5_7`, or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11` or `POSTGRES_12`. SQL Server instances: `SQLSERVER_2017_STANDARD`, `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`. See [the complete list](/sql/docs/mysql/admin-api/rest/v1/SqlDatabaseVersion).
pub enum FlagAppliesToEnum {
    

    /// This is an unknown database version.
    ///
    /// "SQL_DATABASE_VERSION_UNSPECIFIED"
    #[serde(rename="SQL_DATABASE_VERSION_UNSPECIFIED")]
    SQLDATABASEVERSIONUNSPECIFIED,
    

    /// The database version is MySQL 5.1.
    ///
    /// "MYSQL_5_1"
    #[serde(rename="MYSQL_5_1")]
    MYSQL51,
    

    /// The database version is MySQL 5.5.
    ///
    /// "MYSQL_5_5"
    #[serde(rename="MYSQL_5_5")]
    MYSQL55,
    

    /// The database version is MySQL 5.6.
    ///
    /// "MYSQL_5_6"
    #[serde(rename="MYSQL_5_6")]
    MYSQL56,
    

    /// The database version is MySQL 5.7.
    ///
    /// "MYSQL_5_7"
    #[serde(rename="MYSQL_5_7")]
    MYSQL57,
    

    /// The database version is SQL Server 2017 Standard.
    ///
    /// "SQLSERVER_2017_STANDARD"
    #[serde(rename="SQLSERVER_2017_STANDARD")]
    SQLSERVER2017STANDARD,
    

    /// The database version is SQL Server 2017 Enterprise.
    ///
    /// "SQLSERVER_2017_ENTERPRISE"
    #[serde(rename="SQLSERVER_2017_ENTERPRISE")]
    SQLSERVER2017ENTERPRISE,
    

    /// The database version is SQL Server 2017 Express.
    ///
    /// "SQLSERVER_2017_EXPRESS"
    #[serde(rename="SQLSERVER_2017_EXPRESS")]
    SQLSERVER2017EXPRESS,
    

    /// The database version is SQL Server 2017 Web.
    ///
    /// "SQLSERVER_2017_WEB"
    #[serde(rename="SQLSERVER_2017_WEB")]
    SQLSERVER2017WEB,
    

    /// The database version is PostgreSQL 9.6.
    ///
    /// "POSTGRES_9_6"
    #[serde(rename="POSTGRES_9_6")]
    POSTGRES96,
    

    /// The database version is PostgreSQL 10.
    ///
    /// "POSTGRES_10"
    #[serde(rename="POSTGRES_10")]
    POSTGRES10,
    

    /// The database version is PostgreSQL 11.
    ///
    /// "POSTGRES_11"
    #[serde(rename="POSTGRES_11")]
    POSTGRES11,
    

    /// The database version is PostgreSQL 12.
    ///
    /// "POSTGRES_12"
    #[serde(rename="POSTGRES_12")]
    POSTGRES12,
    

    /// The database version is PostgreSQL 13.
    ///
    /// "POSTGRES_13"
    #[serde(rename="POSTGRES_13")]
    POSTGRES13,
    

    /// The database version is PostgreSQL 14.
    ///
    /// "POSTGRES_14"
    #[serde(rename="POSTGRES_14")]
    POSTGRES14,
    

    /// The database version is PostgreSQL 15.
    ///
    /// "POSTGRES_15"
    #[serde(rename="POSTGRES_15")]
    POSTGRES15,
    

    /// The database version is MySQL 8.
    ///
    /// "MYSQL_8_0"
    #[serde(rename="MYSQL_8_0")]
    MYSQL80,
    

    /// The database major version is MySQL 8.0 and the minor version is 18.
    ///
    /// "MYSQL_8_0_18"
    #[serde(rename="MYSQL_8_0_18")]
    MYSQL8018,
    

    /// The database major version is MySQL 8.0 and the minor version is 26.
    ///
    /// "MYSQL_8_0_26"
    #[serde(rename="MYSQL_8_0_26")]
    MYSQL8026,
    

    /// The database major version is MySQL 8.0 and the minor version is 27.
    ///
    /// "MYSQL_8_0_27"
    #[serde(rename="MYSQL_8_0_27")]
    MYSQL8027,
    

    /// The database major version is MySQL 8.0 and the minor version is 28.
    ///
    /// "MYSQL_8_0_28"
    #[serde(rename="MYSQL_8_0_28")]
    MYSQL8028,
    

    /// The database major version is MySQL 8.0 and the minor version is 29.
    ///
    /// "MYSQL_8_0_29"
    #[serde(rename="MYSQL_8_0_29")]
    MYSQL8029,
    

    /// The database major version is MySQL 8.0 and the minor version is 30.
    ///
    /// "MYSQL_8_0_30"
    #[serde(rename="MYSQL_8_0_30")]
    MYSQL8030,
    

    /// The database major version is MySQL 8.0 and the minor version is 31.
    ///
    /// "MYSQL_8_0_31"
    #[serde(rename="MYSQL_8_0_31")]
    MYSQL8031,
    

    /// The database major version is MySQL 8.0 and the minor version is 32.
    ///
    /// "MYSQL_8_0_32"
    #[serde(rename="MYSQL_8_0_32")]
    MYSQL8032,
    

    /// The database major version is MySQL 8.0 and the minor version is 33.
    ///
    /// "MYSQL_8_0_33"
    #[serde(rename="MYSQL_8_0_33")]
    MYSQL8033,
    

    /// The database major version is MySQL 8.0 and the minor version is 34.
    ///
    /// "MYSQL_8_0_34"
    #[serde(rename="MYSQL_8_0_34")]
    MYSQL8034,
    

    /// The database major version is MySQL 8.0 and the minor version is 35.
    ///
    /// "MYSQL_8_0_35"
    #[serde(rename="MYSQL_8_0_35")]
    MYSQL8035,
    

    /// The database major version is MySQL 8.0 and the minor version is 36.
    ///
    /// "MYSQL_8_0_36"
    #[serde(rename="MYSQL_8_0_36")]
    MYSQL8036,
    

    /// The database major version is MySQL 8.0 and the minor version is 37.
    ///
    /// "MYSQL_8_0_37"
    #[serde(rename="MYSQL_8_0_37")]
    MYSQL8037,
    

    /// The database major version is MySQL 8.0 and the minor version is 38.
    ///
    /// "MYSQL_8_0_38"
    #[serde(rename="MYSQL_8_0_38")]
    MYSQL8038,
    

    /// The database major version is MySQL 8.0 and the minor version is 39.
    ///
    /// "MYSQL_8_0_39"
    #[serde(rename="MYSQL_8_0_39")]
    MYSQL8039,
    

    /// The database major version is MySQL 8.0 and the minor version is 40.
    ///
    /// "MYSQL_8_0_40"
    #[serde(rename="MYSQL_8_0_40")]
    MYSQL8040,
    

    /// The database version is SQL Server 2019 Standard.
    ///
    /// "SQLSERVER_2019_STANDARD"
    #[serde(rename="SQLSERVER_2019_STANDARD")]
    SQLSERVER2019STANDARD,
    

    /// The database version is SQL Server 2019 Enterprise.
    ///
    /// "SQLSERVER_2019_ENTERPRISE"
    #[serde(rename="SQLSERVER_2019_ENTERPRISE")]
    SQLSERVER2019ENTERPRISE,
    

    /// The database version is SQL Server 2019 Express.
    ///
    /// "SQLSERVER_2019_EXPRESS"
    #[serde(rename="SQLSERVER_2019_EXPRESS")]
    SQLSERVER2019EXPRESS,
    

    /// The database version is SQL Server 2019 Web.
    ///
    /// "SQLSERVER_2019_WEB"
    #[serde(rename="SQLSERVER_2019_WEB")]
    SQLSERVER2019WEB,
    

    /// The database version is SQL Server 2022 Standard.
    ///
    /// "SQLSERVER_2022_STANDARD"
    #[serde(rename="SQLSERVER_2022_STANDARD")]
    SQLSERVER2022STANDARD,
    

    /// The database version is SQL Server 2022 Enterprise.
    ///
    /// "SQLSERVER_2022_ENTERPRISE"
    #[serde(rename="SQLSERVER_2022_ENTERPRISE")]
    SQLSERVER2022ENTERPRISE,
    

    /// The database version is SQL Server 2022 Express.
    ///
    /// "SQLSERVER_2022_EXPRESS"
    #[serde(rename="SQLSERVER_2022_EXPRESS")]
    SQLSERVER2022EXPRESS,
    

    /// The database version is SQL Server 2022 Web.
    ///
    /// "SQLSERVER_2022_WEB"
    #[serde(rename="SQLSERVER_2022_WEB")]
    SQLSERVER2022WEB,
}

impl AsRef<str> for FlagAppliesToEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlagAppliesToEnum::SQLDATABASEVERSIONUNSPECIFIED => "SQL_DATABASE_VERSION_UNSPECIFIED",
            FlagAppliesToEnum::MYSQL51 => "MYSQL_5_1",
            FlagAppliesToEnum::MYSQL55 => "MYSQL_5_5",
            FlagAppliesToEnum::MYSQL56 => "MYSQL_5_6",
            FlagAppliesToEnum::MYSQL57 => "MYSQL_5_7",
            FlagAppliesToEnum::SQLSERVER2017STANDARD => "SQLSERVER_2017_STANDARD",
            FlagAppliesToEnum::SQLSERVER2017ENTERPRISE => "SQLSERVER_2017_ENTERPRISE",
            FlagAppliesToEnum::SQLSERVER2017EXPRESS => "SQLSERVER_2017_EXPRESS",
            FlagAppliesToEnum::SQLSERVER2017WEB => "SQLSERVER_2017_WEB",
            FlagAppliesToEnum::POSTGRES96 => "POSTGRES_9_6",
            FlagAppliesToEnum::POSTGRES10 => "POSTGRES_10",
            FlagAppliesToEnum::POSTGRES11 => "POSTGRES_11",
            FlagAppliesToEnum::POSTGRES12 => "POSTGRES_12",
            FlagAppliesToEnum::POSTGRES13 => "POSTGRES_13",
            FlagAppliesToEnum::POSTGRES14 => "POSTGRES_14",
            FlagAppliesToEnum::POSTGRES15 => "POSTGRES_15",
            FlagAppliesToEnum::MYSQL80 => "MYSQL_8_0",
            FlagAppliesToEnum::MYSQL8018 => "MYSQL_8_0_18",
            FlagAppliesToEnum::MYSQL8026 => "MYSQL_8_0_26",
            FlagAppliesToEnum::MYSQL8027 => "MYSQL_8_0_27",
            FlagAppliesToEnum::MYSQL8028 => "MYSQL_8_0_28",
            FlagAppliesToEnum::MYSQL8029 => "MYSQL_8_0_29",
            FlagAppliesToEnum::MYSQL8030 => "MYSQL_8_0_30",
            FlagAppliesToEnum::MYSQL8031 => "MYSQL_8_0_31",
            FlagAppliesToEnum::MYSQL8032 => "MYSQL_8_0_32",
            FlagAppliesToEnum::MYSQL8033 => "MYSQL_8_0_33",
            FlagAppliesToEnum::MYSQL8034 => "MYSQL_8_0_34",
            FlagAppliesToEnum::MYSQL8035 => "MYSQL_8_0_35",
            FlagAppliesToEnum::MYSQL8036 => "MYSQL_8_0_36",
            FlagAppliesToEnum::MYSQL8037 => "MYSQL_8_0_37",
            FlagAppliesToEnum::MYSQL8038 => "MYSQL_8_0_38",
            FlagAppliesToEnum::MYSQL8039 => "MYSQL_8_0_39",
            FlagAppliesToEnum::MYSQL8040 => "MYSQL_8_0_40",
            FlagAppliesToEnum::SQLSERVER2019STANDARD => "SQLSERVER_2019_STANDARD",
            FlagAppliesToEnum::SQLSERVER2019ENTERPRISE => "SQLSERVER_2019_ENTERPRISE",
            FlagAppliesToEnum::SQLSERVER2019EXPRESS => "SQLSERVER_2019_EXPRESS",
            FlagAppliesToEnum::SQLSERVER2019WEB => "SQLSERVER_2019_WEB",
            FlagAppliesToEnum::SQLSERVER2022STANDARD => "SQLSERVER_2022_STANDARD",
            FlagAppliesToEnum::SQLSERVER2022ENTERPRISE => "SQLSERVER_2022_ENTERPRISE",
            FlagAppliesToEnum::SQLSERVER2022EXPRESS => "SQLSERVER_2022_EXPRESS",
            FlagAppliesToEnum::SQLSERVER2022WEB => "SQLSERVER_2022_WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for FlagAppliesToEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_DATABASE_VERSION_UNSPECIFIED" => Ok(FlagAppliesToEnum::SQLDATABASEVERSIONUNSPECIFIED),
           "MYSQL_5_1" => Ok(FlagAppliesToEnum::MYSQL51),
           "MYSQL_5_5" => Ok(FlagAppliesToEnum::MYSQL55),
           "MYSQL_5_6" => Ok(FlagAppliesToEnum::MYSQL56),
           "MYSQL_5_7" => Ok(FlagAppliesToEnum::MYSQL57),
           "SQLSERVER_2017_STANDARD" => Ok(FlagAppliesToEnum::SQLSERVER2017STANDARD),
           "SQLSERVER_2017_ENTERPRISE" => Ok(FlagAppliesToEnum::SQLSERVER2017ENTERPRISE),
           "SQLSERVER_2017_EXPRESS" => Ok(FlagAppliesToEnum::SQLSERVER2017EXPRESS),
           "SQLSERVER_2017_WEB" => Ok(FlagAppliesToEnum::SQLSERVER2017WEB),
           "POSTGRES_9_6" => Ok(FlagAppliesToEnum::POSTGRES96),
           "POSTGRES_10" => Ok(FlagAppliesToEnum::POSTGRES10),
           "POSTGRES_11" => Ok(FlagAppliesToEnum::POSTGRES11),
           "POSTGRES_12" => Ok(FlagAppliesToEnum::POSTGRES12),
           "POSTGRES_13" => Ok(FlagAppliesToEnum::POSTGRES13),
           "POSTGRES_14" => Ok(FlagAppliesToEnum::POSTGRES14),
           "POSTGRES_15" => Ok(FlagAppliesToEnum::POSTGRES15),
           "MYSQL_8_0" => Ok(FlagAppliesToEnum::MYSQL80),
           "MYSQL_8_0_18" => Ok(FlagAppliesToEnum::MYSQL8018),
           "MYSQL_8_0_26" => Ok(FlagAppliesToEnum::MYSQL8026),
           "MYSQL_8_0_27" => Ok(FlagAppliesToEnum::MYSQL8027),
           "MYSQL_8_0_28" => Ok(FlagAppliesToEnum::MYSQL8028),
           "MYSQL_8_0_29" => Ok(FlagAppliesToEnum::MYSQL8029),
           "MYSQL_8_0_30" => Ok(FlagAppliesToEnum::MYSQL8030),
           "MYSQL_8_0_31" => Ok(FlagAppliesToEnum::MYSQL8031),
           "MYSQL_8_0_32" => Ok(FlagAppliesToEnum::MYSQL8032),
           "MYSQL_8_0_33" => Ok(FlagAppliesToEnum::MYSQL8033),
           "MYSQL_8_0_34" => Ok(FlagAppliesToEnum::MYSQL8034),
           "MYSQL_8_0_35" => Ok(FlagAppliesToEnum::MYSQL8035),
           "MYSQL_8_0_36" => Ok(FlagAppliesToEnum::MYSQL8036),
           "MYSQL_8_0_37" => Ok(FlagAppliesToEnum::MYSQL8037),
           "MYSQL_8_0_38" => Ok(FlagAppliesToEnum::MYSQL8038),
           "MYSQL_8_0_39" => Ok(FlagAppliesToEnum::MYSQL8039),
           "MYSQL_8_0_40" => Ok(FlagAppliesToEnum::MYSQL8040),
           "SQLSERVER_2019_STANDARD" => Ok(FlagAppliesToEnum::SQLSERVER2019STANDARD),
           "SQLSERVER_2019_ENTERPRISE" => Ok(FlagAppliesToEnum::SQLSERVER2019ENTERPRISE),
           "SQLSERVER_2019_EXPRESS" => Ok(FlagAppliesToEnum::SQLSERVER2019EXPRESS),
           "SQLSERVER_2019_WEB" => Ok(FlagAppliesToEnum::SQLSERVER2019WEB),
           "SQLSERVER_2022_STANDARD" => Ok(FlagAppliesToEnum::SQLSERVER2022STANDARD),
           "SQLSERVER_2022_ENTERPRISE" => Ok(FlagAppliesToEnum::SQLSERVER2022ENTERPRISE),
           "SQLSERVER_2022_EXPRESS" => Ok(FlagAppliesToEnum::SQLSERVER2022EXPRESS),
           "SQLSERVER_2022_WEB" => Ok(FlagAppliesToEnum::SQLSERVER2022WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlagAppliesToEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FlagTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the flag. Flags are typed to being `BOOLEAN`, `STRING`, `INTEGER` or `NONE`. `NONE` is used for flags that do not take a value, such as `skip_grant_tables`.
pub enum FlagTypeEnum {
    

    /// This is an unknown flag type.
    ///
    /// "SQL_FLAG_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_FLAG_TYPE_UNSPECIFIED")]
    SQLFLAGTYPEUNSPECIFIED,
    

    /// Boolean type flag.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// String type flag.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Integer type flag.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// Flag type used for a server startup option.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Type introduced specially for MySQL TimeZone offset. Accept a string value with the format [-12:59, 13:00].
    ///
    /// "MYSQL_TIMEZONE_OFFSET"
    #[serde(rename="MYSQL_TIMEZONE_OFFSET")]
    MYSQLTIMEZONEOFFSET,
    

    /// Float type flag.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// Comma-separated list of the strings in a SqlFlagType enum.
    ///
    /// "REPEATED_STRING"
    #[serde(rename="REPEATED_STRING")]
    REPEATEDSTRING,
}

impl AsRef<str> for FlagTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlagTypeEnum::SQLFLAGTYPEUNSPECIFIED => "SQL_FLAG_TYPE_UNSPECIFIED",
            FlagTypeEnum::BOOLEAN => "BOOLEAN",
            FlagTypeEnum::STRING => "STRING",
            FlagTypeEnum::INTEGER => "INTEGER",
            FlagTypeEnum::NONE => "NONE",
            FlagTypeEnum::MYSQLTIMEZONEOFFSET => "MYSQL_TIMEZONE_OFFSET",
            FlagTypeEnum::FLOAT => "FLOAT",
            FlagTypeEnum::REPEATEDSTRING => "REPEATED_STRING",
        }
    }
}

impl std::convert::TryFrom< &str> for FlagTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_FLAG_TYPE_UNSPECIFIED" => Ok(FlagTypeEnum::SQLFLAGTYPEUNSPECIFIED),
           "BOOLEAN" => Ok(FlagTypeEnum::BOOLEAN),
           "STRING" => Ok(FlagTypeEnum::STRING),
           "INTEGER" => Ok(FlagTypeEnum::INTEGER),
           "NONE" => Ok(FlagTypeEnum::NONE),
           "MYSQL_TIMEZONE_OFFSET" => Ok(FlagTypeEnum::MYSQLTIMEZONEOFFSET),
           "FLOAT" => Ok(FlagTypeEnum::FLOAT),
           "REPEATED_STRING" => Ok(FlagTypeEnum::REPEATEDSTRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FlagTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportContextFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file type for the specified uri.\`SQL`: The file contains SQL statements. \`CSV`: The file contains CSV data.
pub enum ImportContextFileTypeEnum {
    

    /// Unknown file type.
    ///
    /// "SQL_FILE_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_FILE_TYPE_UNSPECIFIED")]
    SQLFILETYPEUNSPECIFIED,
    

    /// File containing SQL statements.
    ///
    /// "SQL"
    #[serde(rename="SQL")]
    SQL,
    

    /// File in CSV format.
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    
    /// "BAK"
    #[serde(rename="BAK")]
    BAK,
}

impl AsRef<str> for ImportContextFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportContextFileTypeEnum::SQLFILETYPEUNSPECIFIED => "SQL_FILE_TYPE_UNSPECIFIED",
            ImportContextFileTypeEnum::SQL => "SQL",
            ImportContextFileTypeEnum::CSV => "CSV",
            ImportContextFileTypeEnum::BAK => "BAK",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportContextFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_FILE_TYPE_UNSPECIFIED" => Ok(ImportContextFileTypeEnum::SQLFILETYPEUNSPECIFIED),
           "SQL" => Ok(ImportContextFileTypeEnum::SQL),
           "CSV" => Ok(ImportContextFileTypeEnum::CSV),
           "BAK" => Ok(ImportContextFileTypeEnum::BAK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportContextFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IpConfigurationSslModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specify how SSL/TLS is enforced in database connections. If you must use the `require_ssl` flag for backward compatibility, then only the following value pairs are valid: For PostgreSQL and MySQL: * `ssl_mode=ALLOW_UNENCRYPTED_AND_ENCRYPTED` and `require_ssl=false` * `ssl_mode=ENCRYPTED_ONLY` and `require_ssl=false` * `ssl_mode=TRUSTED_CLIENT_CERTIFICATE_REQUIRED` and `require_ssl=true` For SQL Server: * `ssl_mode=ALLOW_UNENCRYPTED_AND_ENCRYPTED` and `require_ssl=false` * `ssl_mode=ENCRYPTED_ONLY` and `require_ssl=true` The value of `ssl_mode` gets priority over the value of `require_ssl`. For example, for the pair `ssl_mode=ENCRYPTED_ONLY` and `require_ssl=false`, the `ssl_mode=ENCRYPTED_ONLY` means only accept SSL connections, while the `require_ssl=false` means accept both non-SSL and SSL connections. MySQL and PostgreSQL databases respect `ssl_mode` in this case and accept only SSL connections.
pub enum IpConfigurationSslModeEnum {
    

    /// The SSL mode is unknown.
    ///
    /// "SSL_MODE_UNSPECIFIED"
    #[serde(rename="SSL_MODE_UNSPECIFIED")]
    SSLMODEUNSPECIFIED,
    

    /// Allow non-SSL/non-TLS and SSL/TLS connections. For SSL/TLS connections, the client certificate won't be verified. When this value is used, the legacy `require_ssl` flag must be false or cleared to avoid the conflict between values of two flags.
    ///
    /// "ALLOW_UNENCRYPTED_AND_ENCRYPTED"
    #[serde(rename="ALLOW_UNENCRYPTED_AND_ENCRYPTED")]
    ALLOWUNENCRYPTEDANDENCRYPTED,
    

    /// Only allow connections encrypted with SSL/TLS. When this value is used, the legacy `require_ssl` flag must be false or cleared to avoid the conflict between values of two flags.
    ///
    /// "ENCRYPTED_ONLY"
    #[serde(rename="ENCRYPTED_ONLY")]
    ENCRYPTEDONLY,
    

    /// Only allow connections encrypted with SSL/TLS and with valid client certificates. When this value is used, the legacy `require_ssl` flag must be true or cleared to avoid the conflict between values of two flags. PostgreSQL clients or users that connect using IAM database authentication must use either the [Cloud SQL Auth Proxy](https://cloud.google.com/sql/docs/postgres/connect-auth-proxy) or [Cloud SQL Connectors](https://cloud.google.com/sql/docs/postgres/connect-connectors) to enforce client identity verification.
    ///
    /// "TRUSTED_CLIENT_CERTIFICATE_REQUIRED"
    #[serde(rename="TRUSTED_CLIENT_CERTIFICATE_REQUIRED")]
    TRUSTEDCLIENTCERTIFICATEREQUIRED,
}

impl AsRef<str> for IpConfigurationSslModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IpConfigurationSslModeEnum::SSLMODEUNSPECIFIED => "SSL_MODE_UNSPECIFIED",
            IpConfigurationSslModeEnum::ALLOWUNENCRYPTEDANDENCRYPTED => "ALLOW_UNENCRYPTED_AND_ENCRYPTED",
            IpConfigurationSslModeEnum::ENCRYPTEDONLY => "ENCRYPTED_ONLY",
            IpConfigurationSslModeEnum::TRUSTEDCLIENTCERTIFICATEREQUIRED => "TRUSTED_CLIENT_CERTIFICATE_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for IpConfigurationSslModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSL_MODE_UNSPECIFIED" => Ok(IpConfigurationSslModeEnum::SSLMODEUNSPECIFIED),
           "ALLOW_UNENCRYPTED_AND_ENCRYPTED" => Ok(IpConfigurationSslModeEnum::ALLOWUNENCRYPTEDANDENCRYPTED),
           "ENCRYPTED_ONLY" => Ok(IpConfigurationSslModeEnum::ENCRYPTEDONLY),
           "TRUSTED_CLIENT_CERTIFICATE_REQUIRED" => Ok(IpConfigurationSslModeEnum::TRUSTEDCLIENTCERTIFICATEREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IpConfigurationSslModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IpMappingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this IP address. A `PRIMARY` address is a public address that can accept incoming connections. A `PRIVATE` address is a private address that can accept incoming connections. An `OUTGOING` address is the source address of connections originating from the instance, if supported.
pub enum IpMappingTypeEnum {
    

    /// This is an unknown IP address type.
    ///
    /// "SQL_IP_ADDRESS_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_IP_ADDRESS_TYPE_UNSPECIFIED")]
    SQLIPADDRESSTYPEUNSPECIFIED,
    

    /// IP address the customer is supposed to connect to. Usually this is the load balancer's IP address
    ///
    /// "PRIMARY"
    #[serde(rename="PRIMARY")]
    PRIMARY,
    

    /// Source IP address of the connection a read replica establishes to its external primary instance. This IP address can be allowlisted by the customer in case it has a firewall that filters incoming connection to its on premises primary instance.
    ///
    /// "OUTGOING"
    #[serde(rename="OUTGOING")]
    OUTGOING,
    

    /// Private IP used when using private IPs and network peering.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
    

    /// V1 IP of a migrated instance. We want the user to decommission this IP as soon as the migration is complete. Note: V1 instances with V1 ip addresses will be counted as PRIMARY.
    ///
    /// "MIGRATED_1ST_GEN"
    #[serde(rename="MIGRATED_1ST_GEN")]
    MIGRATED1STGEN,
}

impl AsRef<str> for IpMappingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IpMappingTypeEnum::SQLIPADDRESSTYPEUNSPECIFIED => "SQL_IP_ADDRESS_TYPE_UNSPECIFIED",
            IpMappingTypeEnum::PRIMARY => "PRIMARY",
            IpMappingTypeEnum::OUTGOING => "OUTGOING",
            IpMappingTypeEnum::PRIVATE => "PRIVATE",
            IpMappingTypeEnum::MIGRATED1STGEN => "MIGRATED_1ST_GEN",
        }
    }
}

impl std::convert::TryFrom< &str> for IpMappingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_IP_ADDRESS_TYPE_UNSPECIFIED" => Ok(IpMappingTypeEnum::SQLIPADDRESSTYPEUNSPECIFIED),
           "PRIMARY" => Ok(IpMappingTypeEnum::PRIMARY),
           "OUTGOING" => Ok(IpMappingTypeEnum::OUTGOING),
           "PRIVATE" => Ok(IpMappingTypeEnum::PRIVATE),
           "MIGRATED_1ST_GEN" => Ok(IpMappingTypeEnum::MIGRATED1STGEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IpMappingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MaintenanceWindowUpdateTrackEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Maintenance timing setting: `canary` (Earlier) or `stable` (Later). [Learn more](https://cloud.google.com/sql/docs/mysql/instance-settings#maintenance-timing-2ndgen).
pub enum MaintenanceWindowUpdateTrackEnum {
    

    /// This is an unknown maintenance timing preference.
    ///
    /// "SQL_UPDATE_TRACK_UNSPECIFIED"
    #[serde(rename="SQL_UPDATE_TRACK_UNSPECIFIED")]
    SQLUPDATETRACKUNSPECIFIED,
    

    /// For instance update that requires a restart, this update track indicates your instance prefer to restart for new version early in maintenance window.
    ///
    /// "canary"
    #[serde(rename="canary")]
    Canary,
    

    /// For instance update that requires a restart, this update track indicates your instance prefer to let Cloud SQL choose the timing of restart (within its Maintenance window, if applicable).
    ///
    /// "stable"
    #[serde(rename="stable")]
    Stable,
    

    /// For instance update that requires a restart, this update track indicates your instance prefer to let Cloud SQL choose the timing of restart (within its Maintenance window, if applicable) to be at least 5 weeks after the notification.
    ///
    /// "week5"
    #[serde(rename="week5")]
    Week5,
}

impl AsRef<str> for MaintenanceWindowUpdateTrackEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MaintenanceWindowUpdateTrackEnum::SQLUPDATETRACKUNSPECIFIED => "SQL_UPDATE_TRACK_UNSPECIFIED",
            MaintenanceWindowUpdateTrackEnum::Canary => "canary",
            MaintenanceWindowUpdateTrackEnum::Stable => "stable",
            MaintenanceWindowUpdateTrackEnum::Week5 => "week5",
        }
    }
}

impl std::convert::TryFrom< &str> for MaintenanceWindowUpdateTrackEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_UPDATE_TRACK_UNSPECIFIED" => Ok(MaintenanceWindowUpdateTrackEnum::SQLUPDATETRACKUNSPECIFIED),
           "canary" => Ok(MaintenanceWindowUpdateTrackEnum::Canary),
           "stable" => Ok(MaintenanceWindowUpdateTrackEnum::Stable),
           "week5" => Ok(MaintenanceWindowUpdateTrackEnum::Week5),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MaintenanceWindowUpdateTrackEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationOperationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the operation. Valid values are: * `CREATE` * `DELETE` * `UPDATE` * `RESTART` * `IMPORT` * `EXPORT` * `BACKUP_VOLUME` * `RESTORE_VOLUME` * `CREATE_USER` * `DELETE_USER` * `CREATE_DATABASE` * `DELETE_DATABASE`
pub enum OperationOperationTypeEnum {
    

    /// Unknown operation type.
    ///
    /// "SQL_OPERATION_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_OPERATION_TYPE_UNSPECIFIED")]
    SQLOPERATIONTYPEUNSPECIFIED,
    

    /// Imports data into a Cloud SQL instance.
    ///
    /// "IMPORT"
    #[serde(rename="IMPORT")]
    IMPORT,
    

    /// Exports data from a Cloud SQL instance to a Cloud Storage bucket.
    ///
    /// "EXPORT"
    #[serde(rename="EXPORT")]
    EXPORT,
    

    /// Creates a new Cloud SQL instance.
    ///
    /// "CREATE"
    #[serde(rename="CREATE")]
    CREATE,
    

    /// Updates the settings of a Cloud SQL instance.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// Deletes a Cloud SQL instance.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// Restarts the Cloud SQL instance.
    ///
    /// "RESTART"
    #[serde(rename="RESTART")]
    RESTART,
    
    /// "BACKUP"
    #[serde(rename="BACKUP")]
    BACKUP,
    
    /// "SNAPSHOT"
    #[serde(rename="SNAPSHOT")]
    SNAPSHOT,
    

    /// Performs instance backup.
    ///
    /// "BACKUP_VOLUME"
    #[serde(rename="BACKUP_VOLUME")]
    BACKUPVOLUME,
    

    /// Deletes an instance backup.
    ///
    /// "DELETE_VOLUME"
    #[serde(rename="DELETE_VOLUME")]
    DELETEVOLUME,
    

    /// Restores an instance backup.
    ///
    /// "RESTORE_VOLUME"
    #[serde(rename="RESTORE_VOLUME")]
    RESTOREVOLUME,
    

    /// Injects a privileged user in mysql for MOB instances.
    ///
    /// "INJECT_USER"
    #[serde(rename="INJECT_USER")]
    INJECTUSER,
    

    /// Clones a Cloud SQL instance.
    ///
    /// "CLONE"
    #[serde(rename="CLONE")]
    CLONE,
    

    /// Stops replication on a Cloud SQL read replica instance.
    ///
    /// "STOP_REPLICA"
    #[serde(rename="STOP_REPLICA")]
    STOPREPLICA,
    

    /// Starts replication on a Cloud SQL read replica instance.
    ///
    /// "START_REPLICA"
    #[serde(rename="START_REPLICA")]
    STARTREPLICA,
    

    /// Promotes a Cloud SQL replica instance.
    ///
    /// "PROMOTE_REPLICA"
    #[serde(rename="PROMOTE_REPLICA")]
    PROMOTEREPLICA,
    

    /// Creates a Cloud SQL replica instance.
    ///
    /// "CREATE_REPLICA"
    #[serde(rename="CREATE_REPLICA")]
    CREATEREPLICA,
    

    /// Creates a new user in a Cloud SQL instance.
    ///
    /// "CREATE_USER"
    #[serde(rename="CREATE_USER")]
    CREATEUSER,
    

    /// Deletes a user from a Cloud SQL instance.
    ///
    /// "DELETE_USER"
    #[serde(rename="DELETE_USER")]
    DELETEUSER,
    

    /// Updates an existing user in a Cloud SQL instance.
    ///
    /// "UPDATE_USER"
    #[serde(rename="UPDATE_USER")]
    UPDATEUSER,
    

    /// Creates a database in the Cloud SQL instance.
    ///
    /// "CREATE_DATABASE"
    #[serde(rename="CREATE_DATABASE")]
    CREATEDATABASE,
    

    /// Deletes a database in the Cloud SQL instance.
    ///
    /// "DELETE_DATABASE"
    #[serde(rename="DELETE_DATABASE")]
    DELETEDATABASE,
    

    /// Updates a database in the Cloud SQL instance.
    ///
    /// "UPDATE_DATABASE"
    #[serde(rename="UPDATE_DATABASE")]
    UPDATEDATABASE,
    

    /// Performs failover of an HA-enabled Cloud SQL failover replica.
    ///
    /// "FAILOVER"
    #[serde(rename="FAILOVER")]
    FAILOVER,
    

    /// Deletes the backup taken by a backup run.
    ///
    /// "DELETE_BACKUP"
    #[serde(rename="DELETE_BACKUP")]
    DELETEBACKUP,
    
    /// "RECREATE_REPLICA"
    #[serde(rename="RECREATE_REPLICA")]
    RECREATEREPLICA,
    

    /// Truncates a general or slow log table in MySQL.
    ///
    /// "TRUNCATE_LOG"
    #[serde(rename="TRUNCATE_LOG")]
    TRUNCATELOG,
    

    /// Demotes the stand-alone instance to be a Cloud SQL read replica for an external database server.
    ///
    /// "DEMOTE_MASTER"
    #[serde(rename="DEMOTE_MASTER")]
    DEMOTEMASTER,
    

    /// Indicates that the instance is currently in maintenance. Maintenance typically causes the instance to be unavailable for 1-3 minutes.
    ///
    /// "MAINTENANCE"
    #[serde(rename="MAINTENANCE")]
    MAINTENANCE,
    

    /// This field is deprecated, and will be removed in future version of API.
    ///
    /// "ENABLE_PRIVATE_IP"
    #[serde(rename="ENABLE_PRIVATE_IP")]
    ENABLEPRIVATEIP,
    
    /// "DEFER_MAINTENANCE"
    #[serde(rename="DEFER_MAINTENANCE")]
    DEFERMAINTENANCE,
    

    /// Creates clone instance.
    ///
    /// "CREATE_CLONE"
    #[serde(rename="CREATE_CLONE")]
    CREATECLONE,
    

    /// Reschedule maintenance to another time.
    ///
    /// "RESCHEDULE_MAINTENANCE"
    #[serde(rename="RESCHEDULE_MAINTENANCE")]
    RESCHEDULEMAINTENANCE,
    

    /// Starts external sync of a Cloud SQL EM replica to an external primary instance.
    ///
    /// "START_EXTERNAL_SYNC"
    #[serde(rename="START_EXTERNAL_SYNC")]
    STARTEXTERNALSYNC,
    

    /// Recovers logs from an instance's old data disk.
    ///
    /// "LOG_CLEANUP"
    #[serde(rename="LOG_CLEANUP")]
    LOGCLEANUP,
    

    /// Performs auto-restart of an HA-enabled Cloud SQL database for auto recovery.
    ///
    /// "AUTO_RESTART"
    #[serde(rename="AUTO_RESTART")]
    AUTORESTART,
    

    /// Re-encrypts CMEK instances with latest key version.
    ///
    /// "REENCRYPT"
    #[serde(rename="REENCRYPT")]
    REENCRYPT,
    

    /// Switches over to replica instance from primary.
    ///
    /// "SWITCHOVER"
    #[serde(rename="SWITCHOVER")]
    SWITCHOVER,
    

    /// Acquire a lease for the setup of SQL Server Reporting Services (SSRS).
    ///
    /// "ACQUIRE_SSRS_LEASE"
    #[serde(rename="ACQUIRE_SSRS_LEASE")]
    ACQUIRESSRSLEASE,
    

    /// Release a lease for the setup of SQL Server Reporting Services (SSRS).
    ///
    /// "RELEASE_SSRS_LEASE"
    #[serde(rename="RELEASE_SSRS_LEASE")]
    RELEASESSRSLEASE,
    

    /// Reconfigures old primary after a promote replica operation. Effect of a promote operation to the old primary is executed in this operation, asynchronously from the promote replica operation executed to the replica.
    ///
    /// "RECONFIGURE_OLD_PRIMARY"
    #[serde(rename="RECONFIGURE_OLD_PRIMARY")]
    RECONFIGUREOLDPRIMARY,
}

impl AsRef<str> for OperationOperationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationOperationTypeEnum::SQLOPERATIONTYPEUNSPECIFIED => "SQL_OPERATION_TYPE_UNSPECIFIED",
            OperationOperationTypeEnum::IMPORT => "IMPORT",
            OperationOperationTypeEnum::EXPORT => "EXPORT",
            OperationOperationTypeEnum::CREATE => "CREATE",
            OperationOperationTypeEnum::UPDATE => "UPDATE",
            OperationOperationTypeEnum::DELETE => "DELETE",
            OperationOperationTypeEnum::RESTART => "RESTART",
            OperationOperationTypeEnum::BACKUP => "BACKUP",
            OperationOperationTypeEnum::SNAPSHOT => "SNAPSHOT",
            OperationOperationTypeEnum::BACKUPVOLUME => "BACKUP_VOLUME",
            OperationOperationTypeEnum::DELETEVOLUME => "DELETE_VOLUME",
            OperationOperationTypeEnum::RESTOREVOLUME => "RESTORE_VOLUME",
            OperationOperationTypeEnum::INJECTUSER => "INJECT_USER",
            OperationOperationTypeEnum::CLONE => "CLONE",
            OperationOperationTypeEnum::STOPREPLICA => "STOP_REPLICA",
            OperationOperationTypeEnum::STARTREPLICA => "START_REPLICA",
            OperationOperationTypeEnum::PROMOTEREPLICA => "PROMOTE_REPLICA",
            OperationOperationTypeEnum::CREATEREPLICA => "CREATE_REPLICA",
            OperationOperationTypeEnum::CREATEUSER => "CREATE_USER",
            OperationOperationTypeEnum::DELETEUSER => "DELETE_USER",
            OperationOperationTypeEnum::UPDATEUSER => "UPDATE_USER",
            OperationOperationTypeEnum::CREATEDATABASE => "CREATE_DATABASE",
            OperationOperationTypeEnum::DELETEDATABASE => "DELETE_DATABASE",
            OperationOperationTypeEnum::UPDATEDATABASE => "UPDATE_DATABASE",
            OperationOperationTypeEnum::FAILOVER => "FAILOVER",
            OperationOperationTypeEnum::DELETEBACKUP => "DELETE_BACKUP",
            OperationOperationTypeEnum::RECREATEREPLICA => "RECREATE_REPLICA",
            OperationOperationTypeEnum::TRUNCATELOG => "TRUNCATE_LOG",
            OperationOperationTypeEnum::DEMOTEMASTER => "DEMOTE_MASTER",
            OperationOperationTypeEnum::MAINTENANCE => "MAINTENANCE",
            OperationOperationTypeEnum::ENABLEPRIVATEIP => "ENABLE_PRIVATE_IP",
            OperationOperationTypeEnum::DEFERMAINTENANCE => "DEFER_MAINTENANCE",
            OperationOperationTypeEnum::CREATECLONE => "CREATE_CLONE",
            OperationOperationTypeEnum::RESCHEDULEMAINTENANCE => "RESCHEDULE_MAINTENANCE",
            OperationOperationTypeEnum::STARTEXTERNALSYNC => "START_EXTERNAL_SYNC",
            OperationOperationTypeEnum::LOGCLEANUP => "LOG_CLEANUP",
            OperationOperationTypeEnum::AUTORESTART => "AUTO_RESTART",
            OperationOperationTypeEnum::REENCRYPT => "REENCRYPT",
            OperationOperationTypeEnum::SWITCHOVER => "SWITCHOVER",
            OperationOperationTypeEnum::ACQUIRESSRSLEASE => "ACQUIRE_SSRS_LEASE",
            OperationOperationTypeEnum::RELEASESSRSLEASE => "RELEASE_SSRS_LEASE",
            OperationOperationTypeEnum::RECONFIGUREOLDPRIMARY => "RECONFIGURE_OLD_PRIMARY",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationOperationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_OPERATION_TYPE_UNSPECIFIED" => Ok(OperationOperationTypeEnum::SQLOPERATIONTYPEUNSPECIFIED),
           "IMPORT" => Ok(OperationOperationTypeEnum::IMPORT),
           "EXPORT" => Ok(OperationOperationTypeEnum::EXPORT),
           "CREATE" => Ok(OperationOperationTypeEnum::CREATE),
           "UPDATE" => Ok(OperationOperationTypeEnum::UPDATE),
           "DELETE" => Ok(OperationOperationTypeEnum::DELETE),
           "RESTART" => Ok(OperationOperationTypeEnum::RESTART),
           "BACKUP" => Ok(OperationOperationTypeEnum::BACKUP),
           "SNAPSHOT" => Ok(OperationOperationTypeEnum::SNAPSHOT),
           "BACKUP_VOLUME" => Ok(OperationOperationTypeEnum::BACKUPVOLUME),
           "DELETE_VOLUME" => Ok(OperationOperationTypeEnum::DELETEVOLUME),
           "RESTORE_VOLUME" => Ok(OperationOperationTypeEnum::RESTOREVOLUME),
           "INJECT_USER" => Ok(OperationOperationTypeEnum::INJECTUSER),
           "CLONE" => Ok(OperationOperationTypeEnum::CLONE),
           "STOP_REPLICA" => Ok(OperationOperationTypeEnum::STOPREPLICA),
           "START_REPLICA" => Ok(OperationOperationTypeEnum::STARTREPLICA),
           "PROMOTE_REPLICA" => Ok(OperationOperationTypeEnum::PROMOTEREPLICA),
           "CREATE_REPLICA" => Ok(OperationOperationTypeEnum::CREATEREPLICA),
           "CREATE_USER" => Ok(OperationOperationTypeEnum::CREATEUSER),
           "DELETE_USER" => Ok(OperationOperationTypeEnum::DELETEUSER),
           "UPDATE_USER" => Ok(OperationOperationTypeEnum::UPDATEUSER),
           "CREATE_DATABASE" => Ok(OperationOperationTypeEnum::CREATEDATABASE),
           "DELETE_DATABASE" => Ok(OperationOperationTypeEnum::DELETEDATABASE),
           "UPDATE_DATABASE" => Ok(OperationOperationTypeEnum::UPDATEDATABASE),
           "FAILOVER" => Ok(OperationOperationTypeEnum::FAILOVER),
           "DELETE_BACKUP" => Ok(OperationOperationTypeEnum::DELETEBACKUP),
           "RECREATE_REPLICA" => Ok(OperationOperationTypeEnum::RECREATEREPLICA),
           "TRUNCATE_LOG" => Ok(OperationOperationTypeEnum::TRUNCATELOG),
           "DEMOTE_MASTER" => Ok(OperationOperationTypeEnum::DEMOTEMASTER),
           "MAINTENANCE" => Ok(OperationOperationTypeEnum::MAINTENANCE),
           "ENABLE_PRIVATE_IP" => Ok(OperationOperationTypeEnum::ENABLEPRIVATEIP),
           "DEFER_MAINTENANCE" => Ok(OperationOperationTypeEnum::DEFERMAINTENANCE),
           "CREATE_CLONE" => Ok(OperationOperationTypeEnum::CREATECLONE),
           "RESCHEDULE_MAINTENANCE" => Ok(OperationOperationTypeEnum::RESCHEDULEMAINTENANCE),
           "START_EXTERNAL_SYNC" => Ok(OperationOperationTypeEnum::STARTEXTERNALSYNC),
           "LOG_CLEANUP" => Ok(OperationOperationTypeEnum::LOGCLEANUP),
           "AUTO_RESTART" => Ok(OperationOperationTypeEnum::AUTORESTART),
           "REENCRYPT" => Ok(OperationOperationTypeEnum::REENCRYPT),
           "SWITCHOVER" => Ok(OperationOperationTypeEnum::SWITCHOVER),
           "ACQUIRE_SSRS_LEASE" => Ok(OperationOperationTypeEnum::ACQUIRESSRSLEASE),
           "RELEASE_SSRS_LEASE" => Ok(OperationOperationTypeEnum::RELEASESSRSLEASE),
           "RECONFIGURE_OLD_PRIMARY" => Ok(OperationOperationTypeEnum::RECONFIGUREOLDPRIMARY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationOperationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of an operation.
pub enum OperationStatusEnum {
    

    /// The state of the operation is unknown.
    ///
    /// "SQL_OPERATION_STATUS_UNSPECIFIED"
    #[serde(rename="SQL_OPERATION_STATUS_UNSPECIFIED")]
    SQLOPERATIONSTATUSUNSPECIFIED,
    

    /// The operation has been queued, but has not started yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The operation is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The operation completed.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
}

impl AsRef<str> for OperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationStatusEnum::SQLOPERATIONSTATUSUNSPECIFIED => "SQL_OPERATION_STATUS_UNSPECIFIED",
            OperationStatusEnum::PENDING => "PENDING",
            OperationStatusEnum::RUNNING => "RUNNING",
            OperationStatusEnum::DONE => "DONE",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_OPERATION_STATUS_UNSPECIFIED" => Ok(OperationStatusEnum::SQLOPERATIONSTATUSUNSPECIFIED),
           "PENDING" => Ok(OperationStatusEnum::PENDING),
           "RUNNING" => Ok(OperationStatusEnum::RUNNING),
           "DONE" => Ok(OperationStatusEnum::DONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasswordValidationPolicyComplexityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The complexity of the password.
pub enum PasswordValidationPolicyComplexityEnum {
    

    /// Complexity check is not specified.
    ///
    /// "COMPLEXITY_UNSPECIFIED"
    #[serde(rename="COMPLEXITY_UNSPECIFIED")]
    COMPLEXITYUNSPECIFIED,
    

    /// A combination of lowercase, uppercase, numeric, and non-alphanumeric characters.
    ///
    /// "COMPLEXITY_DEFAULT"
    #[serde(rename="COMPLEXITY_DEFAULT")]
    COMPLEXITYDEFAULT,
}

impl AsRef<str> for PasswordValidationPolicyComplexityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasswordValidationPolicyComplexityEnum::COMPLEXITYUNSPECIFIED => "COMPLEXITY_UNSPECIFIED",
            PasswordValidationPolicyComplexityEnum::COMPLEXITYDEFAULT => "COMPLEXITY_DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for PasswordValidationPolicyComplexityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLEXITY_UNSPECIFIED" => Ok(PasswordValidationPolicyComplexityEnum::COMPLEXITYUNSPECIFIED),
           "COMPLEXITY_DEFAULT" => Ok(PasswordValidationPolicyComplexityEnum::COMPLEXITYDEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasswordValidationPolicyComplexityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RescheduleRescheduleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the reschedule.
pub enum RescheduleRescheduleTypeEnum {
    
    /// "RESCHEDULE_TYPE_UNSPECIFIED"
    #[serde(rename="RESCHEDULE_TYPE_UNSPECIFIED")]
    RESCHEDULETYPEUNSPECIFIED,
    

    /// Reschedules maintenance to happen now (within 5 minutes).
    ///
    /// "IMMEDIATE"
    #[serde(rename="IMMEDIATE")]
    IMMEDIATE,
    

    /// Reschedules maintenance to occur within one week from the originally scheduled day and time.
    ///
    /// "NEXT_AVAILABLE_WINDOW"
    #[serde(rename="NEXT_AVAILABLE_WINDOW")]
    NEXTAVAILABLEWINDOW,
    

    /// Reschedules maintenance to a specific time and day.
    ///
    /// "SPECIFIC_TIME"
    #[serde(rename="SPECIFIC_TIME")]
    SPECIFICTIME,
}

impl AsRef<str> for RescheduleRescheduleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RescheduleRescheduleTypeEnum::RESCHEDULETYPEUNSPECIFIED => "RESCHEDULE_TYPE_UNSPECIFIED",
            RescheduleRescheduleTypeEnum::IMMEDIATE => "IMMEDIATE",
            RescheduleRescheduleTypeEnum::NEXTAVAILABLEWINDOW => "NEXT_AVAILABLE_WINDOW",
            RescheduleRescheduleTypeEnum::SPECIFICTIME => "SPECIFIC_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for RescheduleRescheduleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESCHEDULE_TYPE_UNSPECIFIED" => Ok(RescheduleRescheduleTypeEnum::RESCHEDULETYPEUNSPECIFIED),
           "IMMEDIATE" => Ok(RescheduleRescheduleTypeEnum::IMMEDIATE),
           "NEXT_AVAILABLE_WINDOW" => Ok(RescheduleRescheduleTypeEnum::NEXTAVAILABLEWINDOW),
           "SPECIFIC_TIME" => Ok(RescheduleRescheduleTypeEnum::SPECIFICTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RescheduleRescheduleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingActivationPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The activation policy specifies when the instance is activated; it is applicable only when the instance state is RUNNABLE. Valid values: * `ALWAYS`: The instance is on, and remains so even in the absence of connection requests. * `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
pub enum SettingActivationPolicyEnum {
    

    /// Unknown activation plan.
    ///
    /// "SQL_ACTIVATION_POLICY_UNSPECIFIED"
    #[serde(rename="SQL_ACTIVATION_POLICY_UNSPECIFIED")]
    SQLACTIVATIONPOLICYUNSPECIFIED,
    

    /// The instance is always up and running.
    ///
    /// "ALWAYS"
    #[serde(rename="ALWAYS")]
    ALWAYS,
    

    /// The instance never starts.
    ///
    /// "NEVER"
    #[serde(rename="NEVER")]
    NEVER,
    

    /// The instance starts upon receiving requests.
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
}

impl AsRef<str> for SettingActivationPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingActivationPolicyEnum::SQLACTIVATIONPOLICYUNSPECIFIED => "SQL_ACTIVATION_POLICY_UNSPECIFIED",
            SettingActivationPolicyEnum::ALWAYS => "ALWAYS",
            SettingActivationPolicyEnum::NEVER => "NEVER",
            SettingActivationPolicyEnum::ONDEMAND => "ON_DEMAND",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingActivationPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_ACTIVATION_POLICY_UNSPECIFIED" => Ok(SettingActivationPolicyEnum::SQLACTIVATIONPOLICYUNSPECIFIED),
           "ALWAYS" => Ok(SettingActivationPolicyEnum::ALWAYS),
           "NEVER" => Ok(SettingActivationPolicyEnum::NEVER),
           "ON_DEMAND" => Ok(SettingActivationPolicyEnum::ONDEMAND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingActivationPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingAvailabilityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data accessibility. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available)./ For more information, see [Overview of the High Availability Configuration](https://cloud.google.com/sql/docs/mysql/high-availability).
pub enum SettingAvailabilityTypeEnum {
    

    /// This is an unknown Availability type.
    ///
    /// "SQL_AVAILABILITY_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_AVAILABILITY_TYPE_UNSPECIFIED")]
    SQLAVAILABILITYTYPEUNSPECIFIED,
    

    /// Zonal available instance.
    ///
    /// "ZONAL"
    #[serde(rename="ZONAL")]
    ZONAL,
    

    /// Regional available instance.
    ///
    /// "REGIONAL"
    #[serde(rename="REGIONAL")]
    REGIONAL,
}

impl AsRef<str> for SettingAvailabilityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingAvailabilityTypeEnum::SQLAVAILABILITYTYPEUNSPECIFIED => "SQL_AVAILABILITY_TYPE_UNSPECIFIED",
            SettingAvailabilityTypeEnum::ZONAL => "ZONAL",
            SettingAvailabilityTypeEnum::REGIONAL => "REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingAvailabilityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_AVAILABILITY_TYPE_UNSPECIFIED" => Ok(SettingAvailabilityTypeEnum::SQLAVAILABILITYTYPEUNSPECIFIED),
           "ZONAL" => Ok(SettingAvailabilityTypeEnum::ZONAL),
           "REGIONAL" => Ok(SettingAvailabilityTypeEnum::REGIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingAvailabilityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingConnectorEnforcementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies if connections must use Cloud SQL connectors. Option values include the following: `NOT_REQUIRED` (Cloud SQL instances can be connected without Cloud SQL Connectors) and `REQUIRED` (Only allow connections that use Cloud SQL Connectors). Note that using REQUIRED disables all existing authorized networks. If this field is not specified when creating a new instance, NOT_REQUIRED is used. If this field is not specified when patching or updating an existing instance, it is left unchanged in the instance.
pub enum SettingConnectorEnforcementEnum {
    

    /// The requirement for Cloud SQL connectors is unknown.
    ///
    /// "CONNECTOR_ENFORCEMENT_UNSPECIFIED"
    #[serde(rename="CONNECTOR_ENFORCEMENT_UNSPECIFIED")]
    CONNECTORENFORCEMENTUNSPECIFIED,
    

    /// Do not require Cloud SQL connectors.
    ///
    /// "NOT_REQUIRED"
    #[serde(rename="NOT_REQUIRED")]
    NOTREQUIRED,
    

    /// Require all connections to use Cloud SQL connectors, including the Cloud SQL Auth Proxy and Cloud SQL Java, Python, and Go connectors. Note: This disables all existing authorized networks.
    ///
    /// "REQUIRED"
    #[serde(rename="REQUIRED")]
    REQUIRED,
}

impl AsRef<str> for SettingConnectorEnforcementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingConnectorEnforcementEnum::CONNECTORENFORCEMENTUNSPECIFIED => "CONNECTOR_ENFORCEMENT_UNSPECIFIED",
            SettingConnectorEnforcementEnum::NOTREQUIRED => "NOT_REQUIRED",
            SettingConnectorEnforcementEnum::REQUIRED => "REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingConnectorEnforcementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTOR_ENFORCEMENT_UNSPECIFIED" => Ok(SettingConnectorEnforcementEnum::CONNECTORENFORCEMENTUNSPECIFIED),
           "NOT_REQUIRED" => Ok(SettingConnectorEnforcementEnum::NOTREQUIRED),
           "REQUIRED" => Ok(SettingConnectorEnforcementEnum::REQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingConnectorEnforcementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingDataDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of data disk: `PD_SSD` (default) or `PD_HDD`. Not used for First Generation instances.
pub enum SettingDataDiskTypeEnum {
    

    /// This is an unknown data disk type.
    ///
    /// "SQL_DATA_DISK_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_DATA_DISK_TYPE_UNSPECIFIED")]
    SQLDATADISKTYPEUNSPECIFIED,
    

    /// An SSD data disk.
    ///
    /// "PD_SSD"
    #[serde(rename="PD_SSD")]
    PDSSD,
    

    /// An HDD data disk.
    ///
    /// "PD_HDD"
    #[serde(rename="PD_HDD")]
    PDHDD,
    

    /// This field is deprecated and will be removed from a future version of the API.
    ///
    /// "OBSOLETE_LOCAL_SSD"
    #[serde(rename="OBSOLETE_LOCAL_SSD")]
    OBSOLETELOCALSSD,
}

impl AsRef<str> for SettingDataDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingDataDiskTypeEnum::SQLDATADISKTYPEUNSPECIFIED => "SQL_DATA_DISK_TYPE_UNSPECIFIED",
            SettingDataDiskTypeEnum::PDSSD => "PD_SSD",
            SettingDataDiskTypeEnum::PDHDD => "PD_HDD",
            SettingDataDiskTypeEnum::OBSOLETELOCALSSD => "OBSOLETE_LOCAL_SSD",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingDataDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_DATA_DISK_TYPE_UNSPECIFIED" => Ok(SettingDataDiskTypeEnum::SQLDATADISKTYPEUNSPECIFIED),
           "PD_SSD" => Ok(SettingDataDiskTypeEnum::PDSSD),
           "PD_HDD" => Ok(SettingDataDiskTypeEnum::PDHDD),
           "OBSOLETE_LOCAL_SSD" => Ok(SettingDataDiskTypeEnum::OBSOLETELOCALSSD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingDataDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingEditionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The edition of the instance.
pub enum SettingEditionEnum {
    

    /// The instance did not specify the edition.
    ///
    /// "EDITION_UNSPECIFIED"
    #[serde(rename="EDITION_UNSPECIFIED")]
    EDITIONUNSPECIFIED,
    

    /// The instance is an enterprise edition.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// The instance is an Enterprise Plus edition.
    ///
    /// "ENTERPRISE_PLUS"
    #[serde(rename="ENTERPRISE_PLUS")]
    ENTERPRISEPLUS,
}

impl AsRef<str> for SettingEditionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingEditionEnum::EDITIONUNSPECIFIED => "EDITION_UNSPECIFIED",
            SettingEditionEnum::ENTERPRISE => "ENTERPRISE",
            SettingEditionEnum::ENTERPRISEPLUS => "ENTERPRISE_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingEditionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EDITION_UNSPECIFIED" => Ok(SettingEditionEnum::EDITIONUNSPECIFIED),
           "ENTERPRISE" => Ok(SettingEditionEnum::ENTERPRISE),
           "ENTERPRISE_PLUS" => Ok(SettingEditionEnum::ENTERPRISEPLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingEditionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingPricingPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The pricing plan for this instance. This can be either `PER_USE` or `PACKAGE`. Only `PER_USE` is supported for Second Generation instances.
pub enum SettingPricingPlanEnum {
    

    /// This is an unknown pricing plan for this instance.
    ///
    /// "SQL_PRICING_PLAN_UNSPECIFIED"
    #[serde(rename="SQL_PRICING_PLAN_UNSPECIFIED")]
    SQLPRICINGPLANUNSPECIFIED,
    

    /// The instance is billed at a monthly flat rate.
    ///
    /// "PACKAGE"
    #[serde(rename="PACKAGE")]
    PACKAGE,
    

    /// The instance is billed per usage.
    ///
    /// "PER_USE"
    #[serde(rename="PER_USE")]
    PERUSE,
}

impl AsRef<str> for SettingPricingPlanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingPricingPlanEnum::SQLPRICINGPLANUNSPECIFIED => "SQL_PRICING_PLAN_UNSPECIFIED",
            SettingPricingPlanEnum::PACKAGE => "PACKAGE",
            SettingPricingPlanEnum::PERUSE => "PER_USE",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingPricingPlanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_PRICING_PLAN_UNSPECIFIED" => Ok(SettingPricingPlanEnum::SQLPRICINGPLANUNSPECIFIED),
           "PACKAGE" => Ok(SettingPricingPlanEnum::PACKAGE),
           "PER_USE" => Ok(SettingPricingPlanEnum::PERUSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingPricingPlanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SettingReplicationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of replication this instance uses. This can be either `ASYNCHRONOUS` or `SYNCHRONOUS`. (Deprecated) This property was only applicable to First Generation instances.
pub enum SettingReplicationTypeEnum {
    

    /// This is an unknown replication type for a Cloud SQL instance.
    ///
    /// "SQL_REPLICATION_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_REPLICATION_TYPE_UNSPECIFIED")]
    SQLREPLICATIONTYPEUNSPECIFIED,
    

    /// The synchronous replication mode for First Generation instances. It is the default value.
    ///
    /// "SYNCHRONOUS"
    #[serde(rename="SYNCHRONOUS")]
    SYNCHRONOUS,
    

    /// The asynchronous replication mode for First Generation instances. It provides a slight performance gain, but if an outage occurs while this option is set to asynchronous, you can lose up to a few seconds of updates to your data.
    ///
    /// "ASYNCHRONOUS"
    #[serde(rename="ASYNCHRONOUS")]
    ASYNCHRONOUS,
}

impl AsRef<str> for SettingReplicationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SettingReplicationTypeEnum::SQLREPLICATIONTYPEUNSPECIFIED => "SQL_REPLICATION_TYPE_UNSPECIFIED",
            SettingReplicationTypeEnum::SYNCHRONOUS => "SYNCHRONOUS",
            SettingReplicationTypeEnum::ASYNCHRONOUS => "ASYNCHRONOUS",
        }
    }
}

impl std::convert::TryFrom< &str> for SettingReplicationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_REPLICATION_TYPE_UNSPECIFIED" => Ok(SettingReplicationTypeEnum::SQLREPLICATIONTYPEUNSPECIFIED),
           "SYNCHRONOUS" => Ok(SettingReplicationTypeEnum::SYNCHRONOUS),
           "ASYNCHRONOUS" => Ok(SettingReplicationTypeEnum::ASYNCHRONOUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SettingReplicationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlExternalSyncSettingErrorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies the specific error that occurred.
pub enum SqlExternalSyncSettingErrorTypeEnum {
    
    /// "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED")]
    SQLEXTERNALSYNCSETTINGERRORTYPEUNSPECIFIED,
    
    /// "CONNECTION_FAILURE"
    #[serde(rename="CONNECTION_FAILURE")]
    CONNECTIONFAILURE,
    
    /// "BINLOG_NOT_ENABLED"
    #[serde(rename="BINLOG_NOT_ENABLED")]
    BINLOGNOTENABLED,
    
    /// "INCOMPATIBLE_DATABASE_VERSION"
    #[serde(rename="INCOMPATIBLE_DATABASE_VERSION")]
    INCOMPATIBLEDATABASEVERSION,
    
    /// "REPLICA_ALREADY_SETUP"
    #[serde(rename="REPLICA_ALREADY_SETUP")]
    REPLICAALREADYSETUP,
    

    /// The replication user is missing privileges that are required.
    ///
    /// "INSUFFICIENT_PRIVILEGE"
    #[serde(rename="INSUFFICIENT_PRIVILEGE")]
    INSUFFICIENTPRIVILEGE,
    

    /// Unsupported migration type.
    ///
    /// "UNSUPPORTED_MIGRATION_TYPE"
    #[serde(rename="UNSUPPORTED_MIGRATION_TYPE")]
    UNSUPPORTEDMIGRATIONTYPE,
    

    /// No pglogical extension installed on databases, applicable for postgres.
    ///
    /// "NO_PGLOGICAL_INSTALLED"
    #[serde(rename="NO_PGLOGICAL_INSTALLED")]
    NOPGLOGICALINSTALLED,
    

    /// pglogical node already exists on databases, applicable for postgres.
    ///
    /// "PGLOGICAL_NODE_ALREADY_EXISTS"
    #[serde(rename="PGLOGICAL_NODE_ALREADY_EXISTS")]
    PGLOGICALNODEALREADYEXISTS,
    

    /// The value of parameter wal_level is not set to logical.
    ///
    /// "INVALID_WAL_LEVEL"
    #[serde(rename="INVALID_WAL_LEVEL")]
    INVALIDWALLEVEL,
    

    /// The value of parameter shared_preload_libraries does not include pglogical.
    ///
    /// "INVALID_SHARED_PRELOAD_LIBRARY"
    #[serde(rename="INVALID_SHARED_PRELOAD_LIBRARY")]
    INVALIDSHAREDPRELOADLIBRARY,
    

    /// The value of parameter max_replication_slots is not sufficient.
    ///
    /// "INSUFFICIENT_MAX_REPLICATION_SLOTS"
    #[serde(rename="INSUFFICIENT_MAX_REPLICATION_SLOTS")]
    INSUFFICIENTMAXREPLICATIONSLOTS,
    

    /// The value of parameter max_wal_senders is not sufficient.
    ///
    /// "INSUFFICIENT_MAX_WAL_SENDERS"
    #[serde(rename="INSUFFICIENT_MAX_WAL_SENDERS")]
    INSUFFICIENTMAXWALSENDERS,
    

    /// The value of parameter max_worker_processes is not sufficient.
    ///
    /// "INSUFFICIENT_MAX_WORKER_PROCESSES"
    #[serde(rename="INSUFFICIENT_MAX_WORKER_PROCESSES")]
    INSUFFICIENTMAXWORKERPROCESSES,
    

    /// Extensions installed are either not supported or having unsupported versions.
    ///
    /// "UNSUPPORTED_EXTENSIONS"
    #[serde(rename="UNSUPPORTED_EXTENSIONS")]
    UNSUPPORTEDEXTENSIONS,
    

    /// The value of parameter rds.logical_replication is not set to 1.
    ///
    /// "INVALID_RDS_LOGICAL_REPLICATION"
    #[serde(rename="INVALID_RDS_LOGICAL_REPLICATION")]
    INVALIDRDSLOGICALREPLICATION,
    

    /// The primary instance logging setup doesn't allow EM sync.
    ///
    /// "INVALID_LOGGING_SETUP"
    #[serde(rename="INVALID_LOGGING_SETUP")]
    INVALIDLOGGINGSETUP,
    

    /// The primary instance database parameter setup doesn't allow EM sync.
    ///
    /// "INVALID_DB_PARAM"
    #[serde(rename="INVALID_DB_PARAM")]
    INVALIDDBPARAM,
    

    /// The gtid_mode is not supported, applicable for MySQL.
    ///
    /// "UNSUPPORTED_GTID_MODE"
    #[serde(rename="UNSUPPORTED_GTID_MODE")]
    UNSUPPORTEDGTIDMODE,
    

    /// SQL Server Agent is not running.
    ///
    /// "SQLSERVER_AGENT_NOT_RUNNING"
    #[serde(rename="SQLSERVER_AGENT_NOT_RUNNING")]
    SQLSERVERAGENTNOTRUNNING,
    

    /// The table definition is not support due to missing primary key or replica identity, applicable for postgres.
    ///
    /// "UNSUPPORTED_TABLE_DEFINITION"
    #[serde(rename="UNSUPPORTED_TABLE_DEFINITION")]
    UNSUPPORTEDTABLEDEFINITION,
    

    /// The customer has a definer that will break EM setup.
    ///
    /// "UNSUPPORTED_DEFINER"
    #[serde(rename="UNSUPPORTED_DEFINER")]
    UNSUPPORTEDDEFINER,
    

    /// SQL Server @@SERVERNAME does not match actual host name.
    ///
    /// "SQLSERVER_SERVERNAME_MISMATCH"
    #[serde(rename="SQLSERVER_SERVERNAME_MISMATCH")]
    SQLSERVERSERVERNAMEMISMATCH,
    

    /// The primary instance has been setup and will fail the setup.
    ///
    /// "PRIMARY_ALREADY_SETUP"
    #[serde(rename="PRIMARY_ALREADY_SETUP")]
    PRIMARYALREADYSETUP,
    

    /// The primary instance has unsupported binary log format.
    ///
    /// "UNSUPPORTED_BINLOG_FORMAT"
    #[serde(rename="UNSUPPORTED_BINLOG_FORMAT")]
    UNSUPPORTEDBINLOGFORMAT,
    

    /// The primary instance's binary log retention setting.
    ///
    /// "BINLOG_RETENTION_SETTING"
    #[serde(rename="BINLOG_RETENTION_SETTING")]
    BINLOGRETENTIONSETTING,
    

    /// The primary instance has tables with unsupported storage engine.
    ///
    /// "UNSUPPORTED_STORAGE_ENGINE"
    #[serde(rename="UNSUPPORTED_STORAGE_ENGINE")]
    UNSUPPORTEDSTORAGEENGINE,
    

    /// Source has tables with limited support eg: PostgreSQL tables without primary keys.
    ///
    /// "LIMITED_SUPPORT_TABLES"
    #[serde(rename="LIMITED_SUPPORT_TABLES")]
    LIMITEDSUPPORTTABLES,
    

    /// The replica instance contains existing data.
    ///
    /// "EXISTING_DATA_IN_REPLICA"
    #[serde(rename="EXISTING_DATA_IN_REPLICA")]
    EXISTINGDATAINREPLICA,
    

    /// The replication user is missing privileges that are optional.
    ///
    /// "MISSING_OPTIONAL_PRIVILEGES"
    #[serde(rename="MISSING_OPTIONAL_PRIVILEGES")]
    MISSINGOPTIONALPRIVILEGES,
    

    /// Additional BACKUP_ADMIN privilege is granted to the replication user which may lock source MySQL 8 instance for DDLs during initial sync.
    ///
    /// "RISKY_BACKUP_ADMIN_PRIVILEGE"
    #[serde(rename="RISKY_BACKUP_ADMIN_PRIVILEGE")]
    RISKYBACKUPADMINPRIVILEGE,
    

    /// The Cloud Storage bucket is missing necessary permissions.
    ///
    /// "INSUFFICIENT_GCS_PERMISSIONS"
    #[serde(rename="INSUFFICIENT_GCS_PERMISSIONS")]
    INSUFFICIENTGCSPERMISSIONS,
    

    /// The Cloud Storage bucket has an error in the file or contains invalid file information.
    ///
    /// "INVALID_FILE_INFO"
    #[serde(rename="INVALID_FILE_INFO")]
    INVALIDFILEINFO,
    

    /// The source instance has unsupported database settings for migration.
    ///
    /// "UNSUPPORTED_DATABASE_SETTINGS"
    #[serde(rename="UNSUPPORTED_DATABASE_SETTINGS")]
    UNSUPPORTEDDATABASESETTINGS,
    

    /// The replication user is missing parallel import specific privileges. (e.g. LOCK TABLES) for MySQL.
    ///
    /// "MYSQL_PARALLEL_IMPORT_INSUFFICIENT_PRIVILEGE"
    #[serde(rename="MYSQL_PARALLEL_IMPORT_INSUFFICIENT_PRIVILEGE")]
    MYSQLPARALLELIMPORTINSUFFICIENTPRIVILEGE,
    

    /// The global variable local_infile is off on external server replica.
    ///
    /// "LOCAL_INFILE_OFF"
    #[serde(rename="LOCAL_INFILE_OFF")]
    LOCALINFILEOFF,
    

    /// This code instructs customers to turn on point-in-time recovery manually for the instance after promoting the Cloud SQL for PostgreSQL instance.
    ///
    /// "TURN_ON_PITR_AFTER_PROMOTE"
    #[serde(rename="TURN_ON_PITR_AFTER_PROMOTE")]
    TURNONPITRAFTERPROMOTE,
    

    /// The minor version of replica database is incompatible with the source.
    ///
    /// "INCOMPATIBLE_DATABASE_MINOR_VERSION"
    #[serde(rename="INCOMPATIBLE_DATABASE_MINOR_VERSION")]
    INCOMPATIBLEDATABASEMINORVERSION,
    

    /// This warning message indicates that Cloud SQL uses the maximum number of subscriptions to migrate data from the source to the destination.
    ///
    /// "SOURCE_MAX_SUBSCRIPTIONS"
    #[serde(rename="SOURCE_MAX_SUBSCRIPTIONS")]
    SOURCEMAXSUBSCRIPTIONS,
    

    /// Unable to verify definers on the source for MySQL.
    ///
    /// "UNABLE_TO_VERIFY_DEFINERS"
    #[serde(rename="UNABLE_TO_VERIFY_DEFINERS")]
    UNABLETOVERIFYDEFINERS,
    

    /// If a time out occurs while the subscription counts are calculated, then this value is set to 1. Otherwise, this value is set to 2.
    ///
    /// "SUBSCRIPTION_CALCULATION_STATUS"
    #[serde(rename="SUBSCRIPTION_CALCULATION_STATUS")]
    SUBSCRIPTIONCALCULATIONSTATUS,
    

    /// Count of subscriptions needed to sync source data for PostgreSQL database.
    ///
    /// "PG_SUBSCRIPTION_COUNT"
    #[serde(rename="PG_SUBSCRIPTION_COUNT")]
    PGSUBSCRIPTIONCOUNT,
    

    /// Final parallel level that is used to do migration.
    ///
    /// "PG_SYNC_PARALLEL_LEVEL"
    #[serde(rename="PG_SYNC_PARALLEL_LEVEL")]
    PGSYNCPARALLELLEVEL,
    

    /// The disk size of the replica instance is smaller than the data size of the source instance.
    ///
    /// "INSUFFICIENT_DISK_SIZE"
    #[serde(rename="INSUFFICIENT_DISK_SIZE")]
    INSUFFICIENTDISKSIZE,
    

    /// The data size of the source instance is greater than 1 TB, the number of cores of the replica instance is less than 8, and the memory of the replica is less than 32 GB.
    ///
    /// "INSUFFICIENT_MACHINE_TIER"
    #[serde(rename="INSUFFICIENT_MACHINE_TIER")]
    INSUFFICIENTMACHINETIER,
}

impl AsRef<str> for SqlExternalSyncSettingErrorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlExternalSyncSettingErrorTypeEnum::SQLEXTERNALSYNCSETTINGERRORTYPEUNSPECIFIED => "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED",
            SqlExternalSyncSettingErrorTypeEnum::CONNECTIONFAILURE => "CONNECTION_FAILURE",
            SqlExternalSyncSettingErrorTypeEnum::BINLOGNOTENABLED => "BINLOG_NOT_ENABLED",
            SqlExternalSyncSettingErrorTypeEnum::INCOMPATIBLEDATABASEVERSION => "INCOMPATIBLE_DATABASE_VERSION",
            SqlExternalSyncSettingErrorTypeEnum::REPLICAALREADYSETUP => "REPLICA_ALREADY_SETUP",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTPRIVILEGE => "INSUFFICIENT_PRIVILEGE",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDMIGRATIONTYPE => "UNSUPPORTED_MIGRATION_TYPE",
            SqlExternalSyncSettingErrorTypeEnum::NOPGLOGICALINSTALLED => "NO_PGLOGICAL_INSTALLED",
            SqlExternalSyncSettingErrorTypeEnum::PGLOGICALNODEALREADYEXISTS => "PGLOGICAL_NODE_ALREADY_EXISTS",
            SqlExternalSyncSettingErrorTypeEnum::INVALIDWALLEVEL => "INVALID_WAL_LEVEL",
            SqlExternalSyncSettingErrorTypeEnum::INVALIDSHAREDPRELOADLIBRARY => "INVALID_SHARED_PRELOAD_LIBRARY",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMAXREPLICATIONSLOTS => "INSUFFICIENT_MAX_REPLICATION_SLOTS",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMAXWALSENDERS => "INSUFFICIENT_MAX_WAL_SENDERS",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMAXWORKERPROCESSES => "INSUFFICIENT_MAX_WORKER_PROCESSES",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDEXTENSIONS => "UNSUPPORTED_EXTENSIONS",
            SqlExternalSyncSettingErrorTypeEnum::INVALIDRDSLOGICALREPLICATION => "INVALID_RDS_LOGICAL_REPLICATION",
            SqlExternalSyncSettingErrorTypeEnum::INVALIDLOGGINGSETUP => "INVALID_LOGGING_SETUP",
            SqlExternalSyncSettingErrorTypeEnum::INVALIDDBPARAM => "INVALID_DB_PARAM",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDGTIDMODE => "UNSUPPORTED_GTID_MODE",
            SqlExternalSyncSettingErrorTypeEnum::SQLSERVERAGENTNOTRUNNING => "SQLSERVER_AGENT_NOT_RUNNING",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDTABLEDEFINITION => "UNSUPPORTED_TABLE_DEFINITION",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDDEFINER => "UNSUPPORTED_DEFINER",
            SqlExternalSyncSettingErrorTypeEnum::SQLSERVERSERVERNAMEMISMATCH => "SQLSERVER_SERVERNAME_MISMATCH",
            SqlExternalSyncSettingErrorTypeEnum::PRIMARYALREADYSETUP => "PRIMARY_ALREADY_SETUP",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDBINLOGFORMAT => "UNSUPPORTED_BINLOG_FORMAT",
            SqlExternalSyncSettingErrorTypeEnum::BINLOGRETENTIONSETTING => "BINLOG_RETENTION_SETTING",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDSTORAGEENGINE => "UNSUPPORTED_STORAGE_ENGINE",
            SqlExternalSyncSettingErrorTypeEnum::LIMITEDSUPPORTTABLES => "LIMITED_SUPPORT_TABLES",
            SqlExternalSyncSettingErrorTypeEnum::EXISTINGDATAINREPLICA => "EXISTING_DATA_IN_REPLICA",
            SqlExternalSyncSettingErrorTypeEnum::MISSINGOPTIONALPRIVILEGES => "MISSING_OPTIONAL_PRIVILEGES",
            SqlExternalSyncSettingErrorTypeEnum::RISKYBACKUPADMINPRIVILEGE => "RISKY_BACKUP_ADMIN_PRIVILEGE",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTGCSPERMISSIONS => "INSUFFICIENT_GCS_PERMISSIONS",
            SqlExternalSyncSettingErrorTypeEnum::INVALIDFILEINFO => "INVALID_FILE_INFO",
            SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDDATABASESETTINGS => "UNSUPPORTED_DATABASE_SETTINGS",
            SqlExternalSyncSettingErrorTypeEnum::MYSQLPARALLELIMPORTINSUFFICIENTPRIVILEGE => "MYSQL_PARALLEL_IMPORT_INSUFFICIENT_PRIVILEGE",
            SqlExternalSyncSettingErrorTypeEnum::LOCALINFILEOFF => "LOCAL_INFILE_OFF",
            SqlExternalSyncSettingErrorTypeEnum::TURNONPITRAFTERPROMOTE => "TURN_ON_PITR_AFTER_PROMOTE",
            SqlExternalSyncSettingErrorTypeEnum::INCOMPATIBLEDATABASEMINORVERSION => "INCOMPATIBLE_DATABASE_MINOR_VERSION",
            SqlExternalSyncSettingErrorTypeEnum::SOURCEMAXSUBSCRIPTIONS => "SOURCE_MAX_SUBSCRIPTIONS",
            SqlExternalSyncSettingErrorTypeEnum::UNABLETOVERIFYDEFINERS => "UNABLE_TO_VERIFY_DEFINERS",
            SqlExternalSyncSettingErrorTypeEnum::SUBSCRIPTIONCALCULATIONSTATUS => "SUBSCRIPTION_CALCULATION_STATUS",
            SqlExternalSyncSettingErrorTypeEnum::PGSUBSCRIPTIONCOUNT => "PG_SUBSCRIPTION_COUNT",
            SqlExternalSyncSettingErrorTypeEnum::PGSYNCPARALLELLEVEL => "PG_SYNC_PARALLEL_LEVEL",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTDISKSIZE => "INSUFFICIENT_DISK_SIZE",
            SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMACHINETIER => "INSUFFICIENT_MACHINE_TIER",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlExternalSyncSettingErrorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED" => Ok(SqlExternalSyncSettingErrorTypeEnum::SQLEXTERNALSYNCSETTINGERRORTYPEUNSPECIFIED),
           "CONNECTION_FAILURE" => Ok(SqlExternalSyncSettingErrorTypeEnum::CONNECTIONFAILURE),
           "BINLOG_NOT_ENABLED" => Ok(SqlExternalSyncSettingErrorTypeEnum::BINLOGNOTENABLED),
           "INCOMPATIBLE_DATABASE_VERSION" => Ok(SqlExternalSyncSettingErrorTypeEnum::INCOMPATIBLEDATABASEVERSION),
           "REPLICA_ALREADY_SETUP" => Ok(SqlExternalSyncSettingErrorTypeEnum::REPLICAALREADYSETUP),
           "INSUFFICIENT_PRIVILEGE" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTPRIVILEGE),
           "UNSUPPORTED_MIGRATION_TYPE" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDMIGRATIONTYPE),
           "NO_PGLOGICAL_INSTALLED" => Ok(SqlExternalSyncSettingErrorTypeEnum::NOPGLOGICALINSTALLED),
           "PGLOGICAL_NODE_ALREADY_EXISTS" => Ok(SqlExternalSyncSettingErrorTypeEnum::PGLOGICALNODEALREADYEXISTS),
           "INVALID_WAL_LEVEL" => Ok(SqlExternalSyncSettingErrorTypeEnum::INVALIDWALLEVEL),
           "INVALID_SHARED_PRELOAD_LIBRARY" => Ok(SqlExternalSyncSettingErrorTypeEnum::INVALIDSHAREDPRELOADLIBRARY),
           "INSUFFICIENT_MAX_REPLICATION_SLOTS" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMAXREPLICATIONSLOTS),
           "INSUFFICIENT_MAX_WAL_SENDERS" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMAXWALSENDERS),
           "INSUFFICIENT_MAX_WORKER_PROCESSES" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMAXWORKERPROCESSES),
           "UNSUPPORTED_EXTENSIONS" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDEXTENSIONS),
           "INVALID_RDS_LOGICAL_REPLICATION" => Ok(SqlExternalSyncSettingErrorTypeEnum::INVALIDRDSLOGICALREPLICATION),
           "INVALID_LOGGING_SETUP" => Ok(SqlExternalSyncSettingErrorTypeEnum::INVALIDLOGGINGSETUP),
           "INVALID_DB_PARAM" => Ok(SqlExternalSyncSettingErrorTypeEnum::INVALIDDBPARAM),
           "UNSUPPORTED_GTID_MODE" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDGTIDMODE),
           "SQLSERVER_AGENT_NOT_RUNNING" => Ok(SqlExternalSyncSettingErrorTypeEnum::SQLSERVERAGENTNOTRUNNING),
           "UNSUPPORTED_TABLE_DEFINITION" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDTABLEDEFINITION),
           "UNSUPPORTED_DEFINER" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDDEFINER),
           "SQLSERVER_SERVERNAME_MISMATCH" => Ok(SqlExternalSyncSettingErrorTypeEnum::SQLSERVERSERVERNAMEMISMATCH),
           "PRIMARY_ALREADY_SETUP" => Ok(SqlExternalSyncSettingErrorTypeEnum::PRIMARYALREADYSETUP),
           "UNSUPPORTED_BINLOG_FORMAT" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDBINLOGFORMAT),
           "BINLOG_RETENTION_SETTING" => Ok(SqlExternalSyncSettingErrorTypeEnum::BINLOGRETENTIONSETTING),
           "UNSUPPORTED_STORAGE_ENGINE" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDSTORAGEENGINE),
           "LIMITED_SUPPORT_TABLES" => Ok(SqlExternalSyncSettingErrorTypeEnum::LIMITEDSUPPORTTABLES),
           "EXISTING_DATA_IN_REPLICA" => Ok(SqlExternalSyncSettingErrorTypeEnum::EXISTINGDATAINREPLICA),
           "MISSING_OPTIONAL_PRIVILEGES" => Ok(SqlExternalSyncSettingErrorTypeEnum::MISSINGOPTIONALPRIVILEGES),
           "RISKY_BACKUP_ADMIN_PRIVILEGE" => Ok(SqlExternalSyncSettingErrorTypeEnum::RISKYBACKUPADMINPRIVILEGE),
           "INSUFFICIENT_GCS_PERMISSIONS" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTGCSPERMISSIONS),
           "INVALID_FILE_INFO" => Ok(SqlExternalSyncSettingErrorTypeEnum::INVALIDFILEINFO),
           "UNSUPPORTED_DATABASE_SETTINGS" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNSUPPORTEDDATABASESETTINGS),
           "MYSQL_PARALLEL_IMPORT_INSUFFICIENT_PRIVILEGE" => Ok(SqlExternalSyncSettingErrorTypeEnum::MYSQLPARALLELIMPORTINSUFFICIENTPRIVILEGE),
           "LOCAL_INFILE_OFF" => Ok(SqlExternalSyncSettingErrorTypeEnum::LOCALINFILEOFF),
           "TURN_ON_PITR_AFTER_PROMOTE" => Ok(SqlExternalSyncSettingErrorTypeEnum::TURNONPITRAFTERPROMOTE),
           "INCOMPATIBLE_DATABASE_MINOR_VERSION" => Ok(SqlExternalSyncSettingErrorTypeEnum::INCOMPATIBLEDATABASEMINORVERSION),
           "SOURCE_MAX_SUBSCRIPTIONS" => Ok(SqlExternalSyncSettingErrorTypeEnum::SOURCEMAXSUBSCRIPTIONS),
           "UNABLE_TO_VERIFY_DEFINERS" => Ok(SqlExternalSyncSettingErrorTypeEnum::UNABLETOVERIFYDEFINERS),
           "SUBSCRIPTION_CALCULATION_STATUS" => Ok(SqlExternalSyncSettingErrorTypeEnum::SUBSCRIPTIONCALCULATIONSTATUS),
           "PG_SUBSCRIPTION_COUNT" => Ok(SqlExternalSyncSettingErrorTypeEnum::PGSUBSCRIPTIONCOUNT),
           "PG_SYNC_PARALLEL_LEVEL" => Ok(SqlExternalSyncSettingErrorTypeEnum::PGSYNCPARALLELLEVEL),
           "INSUFFICIENT_DISK_SIZE" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTDISKSIZE),
           "INSUFFICIENT_MACHINE_TIER" => Ok(SqlExternalSyncSettingErrorTypeEnum::INSUFFICIENTMACHINETIER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlExternalSyncSettingErrorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlInstancesStartExternalSyncRequestMigrationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. MigrationType decides if the migration is a physical file based migration or logical migration.
pub enum SqlInstancesStartExternalSyncRequestMigrationTypeEnum {
    

    /// Default value is logical migration
    ///
    /// "MIGRATION_TYPE_UNSPECIFIED"
    #[serde(rename="MIGRATION_TYPE_UNSPECIFIED")]
    MIGRATIONTYPEUNSPECIFIED,
    

    /// Logical Migrations
    ///
    /// "LOGICAL"
    #[serde(rename="LOGICAL")]
    LOGICAL,
    

    /// Physical file based Migrations
    ///
    /// "PHYSICAL"
    #[serde(rename="PHYSICAL")]
    PHYSICAL,
}

impl AsRef<str> for SqlInstancesStartExternalSyncRequestMigrationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlInstancesStartExternalSyncRequestMigrationTypeEnum::MIGRATIONTYPEUNSPECIFIED => "MIGRATION_TYPE_UNSPECIFIED",
            SqlInstancesStartExternalSyncRequestMigrationTypeEnum::LOGICAL => "LOGICAL",
            SqlInstancesStartExternalSyncRequestMigrationTypeEnum::PHYSICAL => "PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlInstancesStartExternalSyncRequestMigrationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MIGRATION_TYPE_UNSPECIFIED" => Ok(SqlInstancesStartExternalSyncRequestMigrationTypeEnum::MIGRATIONTYPEUNSPECIFIED),
           "LOGICAL" => Ok(SqlInstancesStartExternalSyncRequestMigrationTypeEnum::LOGICAL),
           "PHYSICAL" => Ok(SqlInstancesStartExternalSyncRequestMigrationTypeEnum::PHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlInstancesStartExternalSyncRequestMigrationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlInstancesStartExternalSyncRequestSyncModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// External sync mode.
pub enum SqlInstancesStartExternalSyncRequestSyncModeEnum {
    

    /// Unknown external sync mode, will be defaulted to ONLINE mode
    ///
    /// "EXTERNAL_SYNC_MODE_UNSPECIFIED"
    #[serde(rename="EXTERNAL_SYNC_MODE_UNSPECIFIED")]
    EXTERNALSYNCMODEUNSPECIFIED,
    

    /// Online external sync will set up replication after initial data external sync
    ///
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// Offline external sync only dumps and loads a one-time snapshot of the primary instance's data
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for SqlInstancesStartExternalSyncRequestSyncModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlInstancesStartExternalSyncRequestSyncModeEnum::EXTERNALSYNCMODEUNSPECIFIED => "EXTERNAL_SYNC_MODE_UNSPECIFIED",
            SqlInstancesStartExternalSyncRequestSyncModeEnum::ONLINE => "ONLINE",
            SqlInstancesStartExternalSyncRequestSyncModeEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlInstancesStartExternalSyncRequestSyncModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_SYNC_MODE_UNSPECIFIED" => Ok(SqlInstancesStartExternalSyncRequestSyncModeEnum::EXTERNALSYNCMODEUNSPECIFIED),
           "ONLINE" => Ok(SqlInstancesStartExternalSyncRequestSyncModeEnum::ONLINE),
           "OFFLINE" => Ok(SqlInstancesStartExternalSyncRequestSyncModeEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlInstancesStartExternalSyncRequestSyncModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Parallel level for initial data sync. Currently only applicable for MySQL.
pub enum SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum {
    

    /// Unknown sync parallel level. Will be defaulted to OPTIMAL.
    ///
    /// "EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED"
    #[serde(rename="EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED")]
    EXTERNALSYNCPARALLELLEVELUNSPECIFIED,
    

    /// Minimal parallel level.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Optimal parallel level.
    ///
    /// "OPTIMAL"
    #[serde(rename="OPTIMAL")]
    OPTIMAL,
    

    /// Maximum parallel level.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
}

impl AsRef<str> for SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::EXTERNALSYNCPARALLELLEVELUNSPECIFIED => "EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED",
            SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::MIN => "MIN",
            SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::OPTIMAL => "OPTIMAL",
            SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::MAX => "MAX",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED" => Ok(SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::EXTERNALSYNCPARALLELLEVELUNSPECIFIED),
           "MIN" => Ok(SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::MIN),
           "OPTIMAL" => Ok(SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::OPTIMAL),
           "MAX" => Ok(SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum::MAX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlInstancesStartExternalSyncRequestSyncParallelLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. MigrationType decides if the migration is a physical file based migration or logical migration
pub enum SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum {
    

    /// Default value is logical migration
    ///
    /// "MIGRATION_TYPE_UNSPECIFIED"
    #[serde(rename="MIGRATION_TYPE_UNSPECIFIED")]
    MIGRATIONTYPEUNSPECIFIED,
    

    /// Logical Migrations
    ///
    /// "LOGICAL"
    #[serde(rename="LOGICAL")]
    LOGICAL,
    

    /// Physical file based Migrations
    ///
    /// "PHYSICAL"
    #[serde(rename="PHYSICAL")]
    PHYSICAL,
}

impl AsRef<str> for SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum::MIGRATIONTYPEUNSPECIFIED => "MIGRATION_TYPE_UNSPECIFIED",
            SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum::LOGICAL => "LOGICAL",
            SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum::PHYSICAL => "PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MIGRATION_TYPE_UNSPECIFIED" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum::MIGRATIONTYPEUNSPECIFIED),
           "LOGICAL" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum::LOGICAL),
           "PHYSICAL" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum::PHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlInstancesVerifyExternalSyncSettingsRequestMigrationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// External sync mode
pub enum SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum {
    

    /// Unknown external sync mode, will be defaulted to ONLINE mode
    ///
    /// "EXTERNAL_SYNC_MODE_UNSPECIFIED"
    #[serde(rename="EXTERNAL_SYNC_MODE_UNSPECIFIED")]
    EXTERNALSYNCMODEUNSPECIFIED,
    

    /// Online external sync will set up replication after initial data external sync
    ///
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// Offline external sync only dumps and loads a one-time snapshot of the primary instance's data
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum::EXTERNALSYNCMODEUNSPECIFIED => "EXTERNAL_SYNC_MODE_UNSPECIFIED",
            SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum::ONLINE => "ONLINE",
            SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_SYNC_MODE_UNSPECIFIED" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum::EXTERNALSYNCMODEUNSPECIFIED),
           "ONLINE" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum::ONLINE),
           "OFFLINE" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlInstancesVerifyExternalSyncSettingsRequestSyncModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Parallel level for initial data sync. Currently only applicable for PostgreSQL.
pub enum SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum {
    

    /// Unknown sync parallel level. Will be defaulted to OPTIMAL.
    ///
    /// "EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED"
    #[serde(rename="EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED")]
    EXTERNALSYNCPARALLELLEVELUNSPECIFIED,
    

    /// Minimal parallel level.
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Optimal parallel level.
    ///
    /// "OPTIMAL"
    #[serde(rename="OPTIMAL")]
    OPTIMAL,
    

    /// Maximum parallel level.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
}

impl AsRef<str> for SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::EXTERNALSYNCPARALLELLEVELUNSPECIFIED => "EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED",
            SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::MIN => "MIN",
            SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::OPTIMAL => "OPTIMAL",
            SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::MAX => "MAX",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_SYNC_PARALLEL_LEVEL_UNSPECIFIED" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::EXTERNALSYNCPARALLELLEVELUNSPECIFIED),
           "MIN" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::MIN),
           "OPTIMAL" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::OPTIMAL),
           "MAX" => Ok(SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum::MAX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlInstancesVerifyExternalSyncSettingsRequestSyncParallelLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SqlOutOfDiskReportSqlOutOfDiskStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This field represents the state generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job
pub enum SqlOutOfDiskReportSqlOutOfDiskStateEnum {
    

    /// Unspecified state
    ///
    /// "SQL_OUT_OF_DISK_STATE_UNSPECIFIED"
    #[serde(rename="SQL_OUT_OF_DISK_STATE_UNSPECIFIED")]
    SQLOUTOFDISKSTATEUNSPECIFIED,
    

    /// The instance has plenty space on data disk
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// Data disk is almost used up. It is shutdown to prevent data corruption.
    ///
    /// "SOFT_SHUTDOWN"
    #[serde(rename="SOFT_SHUTDOWN")]
    SOFTSHUTDOWN,
}

impl AsRef<str> for SqlOutOfDiskReportSqlOutOfDiskStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SqlOutOfDiskReportSqlOutOfDiskStateEnum::SQLOUTOFDISKSTATEUNSPECIFIED => "SQL_OUT_OF_DISK_STATE_UNSPECIFIED",
            SqlOutOfDiskReportSqlOutOfDiskStateEnum::NORMAL => "NORMAL",
            SqlOutOfDiskReportSqlOutOfDiskStateEnum::SOFTSHUTDOWN => "SOFT_SHUTDOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for SqlOutOfDiskReportSqlOutOfDiskStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_OUT_OF_DISK_STATE_UNSPECIFIED" => Ok(SqlOutOfDiskReportSqlOutOfDiskStateEnum::SQLOUTOFDISKSTATEUNSPECIFIED),
           "NORMAL" => Ok(SqlOutOfDiskReportSqlOutOfDiskStateEnum::NORMAL),
           "SOFT_SHUTDOWN" => Ok(SqlOutOfDiskReportSqlOutOfDiskStateEnum::SOFTSHUTDOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SqlOutOfDiskReportSqlOutOfDiskStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserDualPasswordTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Dual password status for the user.
pub enum UserDualPasswordTypeEnum {
    

    /// The default value.
    ///
    /// "DUAL_PASSWORD_TYPE_UNSPECIFIED"
    #[serde(rename="DUAL_PASSWORD_TYPE_UNSPECIFIED")]
    DUALPASSWORDTYPEUNSPECIFIED,
    

    /// Do not update the user's dual password status.
    ///
    /// "NO_MODIFY_DUAL_PASSWORD"
    #[serde(rename="NO_MODIFY_DUAL_PASSWORD")]
    NOMODIFYDUALPASSWORD,
    

    /// No dual password usable for connecting using this user.
    ///
    /// "NO_DUAL_PASSWORD"
    #[serde(rename="NO_DUAL_PASSWORD")]
    NODUALPASSWORD,
    

    /// Dual password usable for connecting using this user.
    ///
    /// "DUAL_PASSWORD"
    #[serde(rename="DUAL_PASSWORD")]
    DUALPASSWORD,
}

impl AsRef<str> for UserDualPasswordTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserDualPasswordTypeEnum::DUALPASSWORDTYPEUNSPECIFIED => "DUAL_PASSWORD_TYPE_UNSPECIFIED",
            UserDualPasswordTypeEnum::NOMODIFYDUALPASSWORD => "NO_MODIFY_DUAL_PASSWORD",
            UserDualPasswordTypeEnum::NODUALPASSWORD => "NO_DUAL_PASSWORD",
            UserDualPasswordTypeEnum::DUALPASSWORD => "DUAL_PASSWORD",
        }
    }
}

impl std::convert::TryFrom< &str> for UserDualPasswordTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DUAL_PASSWORD_TYPE_UNSPECIFIED" => Ok(UserDualPasswordTypeEnum::DUALPASSWORDTYPEUNSPECIFIED),
           "NO_MODIFY_DUAL_PASSWORD" => Ok(UserDualPasswordTypeEnum::NOMODIFYDUALPASSWORD),
           "NO_DUAL_PASSWORD" => Ok(UserDualPasswordTypeEnum::NODUALPASSWORD),
           "DUAL_PASSWORD" => Ok(UserDualPasswordTypeEnum::DUALPASSWORD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserDualPasswordTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The user type. It determines the method to authenticate the user during login. The default is the database's built-in user type.
pub enum UserTypeEnum {
    

    /// The database's built-in user type.
    ///
    /// "BUILT_IN"
    #[serde(rename="BUILT_IN")]
    BUILTIN,
    

    /// Cloud IAM user.
    ///
    /// "CLOUD_IAM_USER"
    #[serde(rename="CLOUD_IAM_USER")]
    CLOUDIAMUSER,
    

    /// Cloud IAM service account.
    ///
    /// "CLOUD_IAM_SERVICE_ACCOUNT"
    #[serde(rename="CLOUD_IAM_SERVICE_ACCOUNT")]
    CLOUDIAMSERVICEACCOUNT,
    

    /// Cloud IAM group non-login user.
    ///
    /// "CLOUD_IAM_GROUP"
    #[serde(rename="CLOUD_IAM_GROUP")]
    CLOUDIAMGROUP,
    

    /// Cloud IAM group login user.
    ///
    /// "CLOUD_IAM_GROUP_USER"
    #[serde(rename="CLOUD_IAM_GROUP_USER")]
    CLOUDIAMGROUPUSER,
    

    /// Cloud IAM group login service account.
    ///
    /// "CLOUD_IAM_GROUP_SERVICE_ACCOUNT"
    #[serde(rename="CLOUD_IAM_GROUP_SERVICE_ACCOUNT")]
    CLOUDIAMGROUPSERVICEACCOUNT,
}

impl AsRef<str> for UserTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserTypeEnum::BUILTIN => "BUILT_IN",
            UserTypeEnum::CLOUDIAMUSER => "CLOUD_IAM_USER",
            UserTypeEnum::CLOUDIAMSERVICEACCOUNT => "CLOUD_IAM_SERVICE_ACCOUNT",
            UserTypeEnum::CLOUDIAMGROUP => "CLOUD_IAM_GROUP",
            UserTypeEnum::CLOUDIAMGROUPUSER => "CLOUD_IAM_GROUP_USER",
            UserTypeEnum::CLOUDIAMGROUPSERVICEACCOUNT => "CLOUD_IAM_GROUP_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for UserTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUILT_IN" => Ok(UserTypeEnum::BUILTIN),
           "CLOUD_IAM_USER" => Ok(UserTypeEnum::CLOUDIAMUSER),
           "CLOUD_IAM_SERVICE_ACCOUNT" => Ok(UserTypeEnum::CLOUDIAMSERVICEACCOUNT),
           "CLOUD_IAM_GROUP" => Ok(UserTypeEnum::CLOUDIAMGROUP),
           "CLOUD_IAM_GROUP_USER" => Ok(UserTypeEnum::CLOUDIAMGROUPUSER),
           "CLOUD_IAM_GROUP_SERVICE_ACCOUNT" => Ok(UserTypeEnum::CLOUDIAMGROUPSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportContextBakExportOptionBakTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this bak file will be export, FULL or DIFF, SQL Server only
pub enum ExportContextBakExportOptionBakTypeEnum {
    

    /// Default type.
    ///
    /// "BAK_TYPE_UNSPECIFIED"
    #[serde(rename="BAK_TYPE_UNSPECIFIED")]
    BAKTYPEUNSPECIFIED,
    

    /// Full backup.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Differential backup.
    ///
    /// "DIFF"
    #[serde(rename="DIFF")]
    DIFF,
    

    /// Transaction Log backup
    ///
    /// "TLOG"
    #[serde(rename="TLOG")]
    TLOG,
}

impl AsRef<str> for ExportContextBakExportOptionBakTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportContextBakExportOptionBakTypeEnum::BAKTYPEUNSPECIFIED => "BAK_TYPE_UNSPECIFIED",
            ExportContextBakExportOptionBakTypeEnum::FULL => "FULL",
            ExportContextBakExportOptionBakTypeEnum::DIFF => "DIFF",
            ExportContextBakExportOptionBakTypeEnum::TLOG => "TLOG",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportContextBakExportOptionBakTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BAK_TYPE_UNSPECIFIED" => Ok(ExportContextBakExportOptionBakTypeEnum::BAKTYPEUNSPECIFIED),
           "FULL" => Ok(ExportContextBakExportOptionBakTypeEnum::FULL),
           "DIFF" => Ok(ExportContextBakExportOptionBakTypeEnum::DIFF),
           "TLOG" => Ok(ExportContextBakExportOptionBakTypeEnum::TLOG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportContextBakExportOptionBakTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportContextBakImportOptionBakTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the bak content, FULL or DIFF
pub enum ImportContextBakImportOptionBakTypeEnum {
    

    /// Default type.
    ///
    /// "BAK_TYPE_UNSPECIFIED"
    #[serde(rename="BAK_TYPE_UNSPECIFIED")]
    BAKTYPEUNSPECIFIED,
    

    /// Full backup.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Differential backup.
    ///
    /// "DIFF"
    #[serde(rename="DIFF")]
    DIFF,
    

    /// Transaction Log backup
    ///
    /// "TLOG"
    #[serde(rename="TLOG")]
    TLOG,
}

impl AsRef<str> for ImportContextBakImportOptionBakTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportContextBakImportOptionBakTypeEnum::BAKTYPEUNSPECIFIED => "BAK_TYPE_UNSPECIFIED",
            ImportContextBakImportOptionBakTypeEnum::FULL => "FULL",
            ImportContextBakImportOptionBakTypeEnum::DIFF => "DIFF",
            ImportContextBakImportOptionBakTypeEnum::TLOG => "TLOG",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportContextBakImportOptionBakTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BAK_TYPE_UNSPECIFIED" => Ok(ImportContextBakImportOptionBakTypeEnum::BAKTYPEUNSPECIFIED),
           "FULL" => Ok(ImportContextBakImportOptionBakTypeEnum::FULL),
           "DIFF" => Ok(ImportContextBakImportOptionBakTypeEnum::DIFF),
           "TLOG" => Ok(ImportContextBakImportOptionBakTypeEnum::TLOG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportContextBakImportOptionBakTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


