use super::*;



// region AlloyDbSettingDatabaseVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The database engine major version. This is an optional field. If a database version is not supplied at cluster creation time, then a default database version will be used.
pub enum AlloyDbSettingDatabaseVersionEnum {
    

    /// This is an unknown database version.
    ///
    /// "DATABASE_VERSION_UNSPECIFIED"
    #[serde(rename="DATABASE_VERSION_UNSPECIFIED")]
    DATABASEVERSIONUNSPECIFIED,
    

    /// The database version is Postgres 14.
    ///
    /// "POSTGRES_14"
    #[serde(rename="POSTGRES_14")]
    POSTGRES14,
    

    /// The database version is Postgres 15.
    ///
    /// "POSTGRES_15"
    #[serde(rename="POSTGRES_15")]
    POSTGRES15,
}

impl AsRef<str> for AlloyDbSettingDatabaseVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AlloyDbSettingDatabaseVersionEnum::DATABASEVERSIONUNSPECIFIED => "DATABASE_VERSION_UNSPECIFIED",
            AlloyDbSettingDatabaseVersionEnum::POSTGRES14 => "POSTGRES_14",
            AlloyDbSettingDatabaseVersionEnum::POSTGRES15 => "POSTGRES_15",
        }
    }
}

impl std::convert::TryFrom< &str> for AlloyDbSettingDatabaseVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_VERSION_UNSPECIFIED" => Ok(AlloyDbSettingDatabaseVersionEnum::DATABASEVERSIONUNSPECIFIED),
           "POSTGRES_14" => Ok(AlloyDbSettingDatabaseVersionEnum::POSTGRES14),
           "POSTGRES_15" => Ok(AlloyDbSettingDatabaseVersionEnum::POSTGRES15),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AlloyDbSettingDatabaseVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


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


// region BackgroundJobLogEntryCompletionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Job completion state, i.e. the final state after the job completed.
pub enum BackgroundJobLogEntryCompletionStateEnum {
    

    /// The status is not specified. This state is used when job is not yet finished.
    ///
    /// "JOB_COMPLETION_STATE_UNSPECIFIED"
    #[serde(rename="JOB_COMPLETION_STATE_UNSPECIFIED")]
    JOBCOMPLETIONSTATEUNSPECIFIED,
    

    /// Success.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for BackgroundJobLogEntryCompletionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackgroundJobLogEntryCompletionStateEnum::JOBCOMPLETIONSTATEUNSPECIFIED => "JOB_COMPLETION_STATE_UNSPECIFIED",
            BackgroundJobLogEntryCompletionStateEnum::SUCCEEDED => "SUCCEEDED",
            BackgroundJobLogEntryCompletionStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for BackgroundJobLogEntryCompletionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_COMPLETION_STATE_UNSPECIFIED" => Ok(BackgroundJobLogEntryCompletionStateEnum::JOBCOMPLETIONSTATEUNSPECIFIED),
           "SUCCEEDED" => Ok(BackgroundJobLogEntryCompletionStateEnum::SUCCEEDED),
           "FAILED" => Ok(BackgroundJobLogEntryCompletionStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackgroundJobLogEntryCompletionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackgroundJobLogEntryJobTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of job that was executed.
pub enum BackgroundJobLogEntryJobTypeEnum {
    

    /// Unspecified background job type.
    ///
    /// "BACKGROUND_JOB_TYPE_UNSPECIFIED"
    #[serde(rename="BACKGROUND_JOB_TYPE_UNSPECIFIED")]
    BACKGROUNDJOBTYPEUNSPECIFIED,
    

    /// Job to seed from the source database.
    ///
    /// "BACKGROUND_JOB_TYPE_SOURCE_SEED"
    #[serde(rename="BACKGROUND_JOB_TYPE_SOURCE_SEED")]
    BACKGROUNDJOBTYPESOURCESEED,
    

    /// Job to convert the source database into a draft of the destination database.
    ///
    /// "BACKGROUND_JOB_TYPE_CONVERT"
    #[serde(rename="BACKGROUND_JOB_TYPE_CONVERT")]
    BACKGROUNDJOBTYPECONVERT,
    

    /// Job to apply the draft tree onto the destination.
    ///
    /// "BACKGROUND_JOB_TYPE_APPLY_DESTINATION"
    #[serde(rename="BACKGROUND_JOB_TYPE_APPLY_DESTINATION")]
    BACKGROUNDJOBTYPEAPPLYDESTINATION,
    

    /// Job to import and convert mapping rules from an external source such as an ora2pg config file.
    ///
    /// "BACKGROUND_JOB_TYPE_IMPORT_RULES_FILE"
    #[serde(rename="BACKGROUND_JOB_TYPE_IMPORT_RULES_FILE")]
    BACKGROUNDJOBTYPEIMPORTRULESFILE,
}

impl AsRef<str> for BackgroundJobLogEntryJobTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPEUNSPECIFIED => "BACKGROUND_JOB_TYPE_UNSPECIFIED",
            BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPESOURCESEED => "BACKGROUND_JOB_TYPE_SOURCE_SEED",
            BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPECONVERT => "BACKGROUND_JOB_TYPE_CONVERT",
            BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPEAPPLYDESTINATION => "BACKGROUND_JOB_TYPE_APPLY_DESTINATION",
            BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPEIMPORTRULESFILE => "BACKGROUND_JOB_TYPE_IMPORT_RULES_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for BackgroundJobLogEntryJobTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BACKGROUND_JOB_TYPE_UNSPECIFIED" => Ok(BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPEUNSPECIFIED),
           "BACKGROUND_JOB_TYPE_SOURCE_SEED" => Ok(BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPESOURCESEED),
           "BACKGROUND_JOB_TYPE_CONVERT" => Ok(BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPECONVERT),
           "BACKGROUND_JOB_TYPE_APPLY_DESTINATION" => Ok(BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPEAPPLYDESTINATION),
           "BACKGROUND_JOB_TYPE_IMPORT_RULES_FILE" => Ok(BackgroundJobLogEntryJobTypeEnum::BACKGROUNDJOBTYPEIMPORTRULESFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackgroundJobLogEntryJobTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudSqlSettingActivationPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Valid values: 'ALWAYS': The instance is on, and remains so even in the absence of connection requests. `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
pub enum CloudSqlSettingActivationPolicyEnum {
    

    /// unspecified policy.
    ///
    /// "SQL_ACTIVATION_POLICY_UNSPECIFIED"
    #[serde(rename="SQL_ACTIVATION_POLICY_UNSPECIFIED")]
    SQLACTIVATIONPOLICYUNSPECIFIED,
    

    /// The instance is always up and running.
    ///
    /// "ALWAYS"
    #[serde(rename="ALWAYS")]
    ALWAYS,
    

    /// The instance should never spin up.
    ///
    /// "NEVER"
    #[serde(rename="NEVER")]
    NEVER,
}

impl AsRef<str> for CloudSqlSettingActivationPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudSqlSettingActivationPolicyEnum::SQLACTIVATIONPOLICYUNSPECIFIED => "SQL_ACTIVATION_POLICY_UNSPECIFIED",
            CloudSqlSettingActivationPolicyEnum::ALWAYS => "ALWAYS",
            CloudSqlSettingActivationPolicyEnum::NEVER => "NEVER",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudSqlSettingActivationPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_ACTIVATION_POLICY_UNSPECIFIED" => Ok(CloudSqlSettingActivationPolicyEnum::SQLACTIVATIONPOLICYUNSPECIFIED),
           "ALWAYS" => Ok(CloudSqlSettingActivationPolicyEnum::ALWAYS),
           "NEVER" => Ok(CloudSqlSettingActivationPolicyEnum::NEVER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudSqlSettingActivationPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudSqlSettingAvailabilityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data availability. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available).
pub enum CloudSqlSettingAvailabilityTypeEnum {
    

    /// This is an unknown Availability type.
    ///
    /// "SQL_AVAILABILITY_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_AVAILABILITY_TYPE_UNSPECIFIED")]
    SQLAVAILABILITYTYPEUNSPECIFIED,
    

    /// Zonal availablility instance.
    ///
    /// "ZONAL"
    #[serde(rename="ZONAL")]
    ZONAL,
    

    /// Regional availability instance.
    ///
    /// "REGIONAL"
    #[serde(rename="REGIONAL")]
    REGIONAL,
}

impl AsRef<str> for CloudSqlSettingAvailabilityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudSqlSettingAvailabilityTypeEnum::SQLAVAILABILITYTYPEUNSPECIFIED => "SQL_AVAILABILITY_TYPE_UNSPECIFIED",
            CloudSqlSettingAvailabilityTypeEnum::ZONAL => "ZONAL",
            CloudSqlSettingAvailabilityTypeEnum::REGIONAL => "REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudSqlSettingAvailabilityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_AVAILABILITY_TYPE_UNSPECIFIED" => Ok(CloudSqlSettingAvailabilityTypeEnum::SQLAVAILABILITYTYPEUNSPECIFIED),
           "ZONAL" => Ok(CloudSqlSettingAvailabilityTypeEnum::ZONAL),
           "REGIONAL" => Ok(CloudSqlSettingAvailabilityTypeEnum::REGIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudSqlSettingAvailabilityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudSqlSettingDataDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of storage: `PD_SSD` (default) or `PD_HDD`.
pub enum CloudSqlSettingDataDiskTypeEnum {
    

    /// Unspecified.
    ///
    /// "SQL_DATA_DISK_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_DATA_DISK_TYPE_UNSPECIFIED")]
    SQLDATADISKTYPEUNSPECIFIED,
    

    /// SSD disk.
    ///
    /// "PD_SSD"
    #[serde(rename="PD_SSD")]
    PDSSD,
    

    /// HDD disk.
    ///
    /// "PD_HDD"
    #[serde(rename="PD_HDD")]
    PDHDD,
}

impl AsRef<str> for CloudSqlSettingDataDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudSqlSettingDataDiskTypeEnum::SQLDATADISKTYPEUNSPECIFIED => "SQL_DATA_DISK_TYPE_UNSPECIFIED",
            CloudSqlSettingDataDiskTypeEnum::PDSSD => "PD_SSD",
            CloudSqlSettingDataDiskTypeEnum::PDHDD => "PD_HDD",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudSqlSettingDataDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_DATA_DISK_TYPE_UNSPECIFIED" => Ok(CloudSqlSettingDataDiskTypeEnum::SQLDATADISKTYPEUNSPECIFIED),
           "PD_SSD" => Ok(CloudSqlSettingDataDiskTypeEnum::PDSSD),
           "PD_HDD" => Ok(CloudSqlSettingDataDiskTypeEnum::PDHDD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudSqlSettingDataDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudSqlSettingDatabaseVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database engine type and version.
pub enum CloudSqlSettingDatabaseVersionEnum {
    

    /// Unspecified version.
    ///
    /// "SQL_DATABASE_VERSION_UNSPECIFIED"
    #[serde(rename="SQL_DATABASE_VERSION_UNSPECIFIED")]
    SQLDATABASEVERSIONUNSPECIFIED,
    

    /// MySQL 5.6.
    ///
    /// "MYSQL_5_6"
    #[serde(rename="MYSQL_5_6")]
    MYSQL56,
    

    /// MySQL 5.7.
    ///
    /// "MYSQL_5_7"
    #[serde(rename="MYSQL_5_7")]
    MYSQL57,
    

    /// MySQL 8.0.
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
    

    /// PostgreSQL 9.6.
    ///
    /// "POSTGRES_9_6"
    #[serde(rename="POSTGRES_9_6")]
    POSTGRES96,
    

    /// PostgreSQL 11.
    ///
    /// "POSTGRES_11"
    #[serde(rename="POSTGRES_11")]
    POSTGRES11,
    

    /// PostgreSQL 10.
    ///
    /// "POSTGRES_10"
    #[serde(rename="POSTGRES_10")]
    POSTGRES10,
    

    /// PostgreSQL 12.
    ///
    /// "POSTGRES_12"
    #[serde(rename="POSTGRES_12")]
    POSTGRES12,
    

    /// PostgreSQL 13.
    ///
    /// "POSTGRES_13"
    #[serde(rename="POSTGRES_13")]
    POSTGRES13,
    

    /// PostgreSQL 14.
    ///
    /// "POSTGRES_14"
    #[serde(rename="POSTGRES_14")]
    POSTGRES14,
    

    /// PostgreSQL 15.
    ///
    /// "POSTGRES_15"
    #[serde(rename="POSTGRES_15")]
    POSTGRES15,
}

impl AsRef<str> for CloudSqlSettingDatabaseVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudSqlSettingDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED => "SQL_DATABASE_VERSION_UNSPECIFIED",
            CloudSqlSettingDatabaseVersionEnum::MYSQL56 => "MYSQL_5_6",
            CloudSqlSettingDatabaseVersionEnum::MYSQL57 => "MYSQL_5_7",
            CloudSqlSettingDatabaseVersionEnum::MYSQL80 => "MYSQL_8_0",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8018 => "MYSQL_8_0_18",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8026 => "MYSQL_8_0_26",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8027 => "MYSQL_8_0_27",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8028 => "MYSQL_8_0_28",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8030 => "MYSQL_8_0_30",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8031 => "MYSQL_8_0_31",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8032 => "MYSQL_8_0_32",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8033 => "MYSQL_8_0_33",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8034 => "MYSQL_8_0_34",
            CloudSqlSettingDatabaseVersionEnum::MYSQL8035 => "MYSQL_8_0_35",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES96 => "POSTGRES_9_6",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES11 => "POSTGRES_11",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES10 => "POSTGRES_10",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES12 => "POSTGRES_12",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES13 => "POSTGRES_13",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES14 => "POSTGRES_14",
            CloudSqlSettingDatabaseVersionEnum::POSTGRES15 => "POSTGRES_15",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudSqlSettingDatabaseVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_DATABASE_VERSION_UNSPECIFIED" => Ok(CloudSqlSettingDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED),
           "MYSQL_5_6" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL56),
           "MYSQL_5_7" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL57),
           "MYSQL_8_0" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL80),
           "MYSQL_8_0_18" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8018),
           "MYSQL_8_0_26" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8026),
           "MYSQL_8_0_27" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8027),
           "MYSQL_8_0_28" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8028),
           "MYSQL_8_0_30" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8030),
           "MYSQL_8_0_31" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8031),
           "MYSQL_8_0_32" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8032),
           "MYSQL_8_0_33" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8033),
           "MYSQL_8_0_34" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8034),
           "MYSQL_8_0_35" => Ok(CloudSqlSettingDatabaseVersionEnum::MYSQL8035),
           "POSTGRES_9_6" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES96),
           "POSTGRES_11" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES11),
           "POSTGRES_10" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES10),
           "POSTGRES_12" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES12),
           "POSTGRES_13" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES13),
           "POSTGRES_14" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES14),
           "POSTGRES_15" => Ok(CloudSqlSettingDatabaseVersionEnum::POSTGRES15),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudSqlSettingDatabaseVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudSqlSettingEditionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The edition of the given Cloud SQL instance.
pub enum CloudSqlSettingEditionEnum {
    

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
    

    /// The instance is an enterprise plus edition.
    ///
    /// "ENTERPRISE_PLUS"
    #[serde(rename="ENTERPRISE_PLUS")]
    ENTERPRISEPLUS,
}

impl AsRef<str> for CloudSqlSettingEditionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudSqlSettingEditionEnum::EDITIONUNSPECIFIED => "EDITION_UNSPECIFIED",
            CloudSqlSettingEditionEnum::ENTERPRISE => "ENTERPRISE",
            CloudSqlSettingEditionEnum::ENTERPRISEPLUS => "ENTERPRISE_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudSqlSettingEditionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EDITION_UNSPECIFIED" => Ok(CloudSqlSettingEditionEnum::EDITIONUNSPECIFIED),
           "ENTERPRISE" => Ok(CloudSqlSettingEditionEnum::ENTERPRISE),
           "ENTERPRISE_PLUS" => Ok(CloudSqlSettingEditionEnum::ENTERPRISEPLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudSqlSettingEditionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionProfileProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database provider.
pub enum ConnectionProfileProviderEnum {
    

    /// Use this value for on-premise source database instances.
    ///
    /// "DATABASE_PROVIDER_UNSPECIFIED"
    #[serde(rename="DATABASE_PROVIDER_UNSPECIFIED")]
    DATABASEPROVIDERUNSPECIFIED,
    

    /// Cloud SQL is the source instance provider.
    ///
    /// "CLOUDSQL"
    #[serde(rename="CLOUDSQL")]
    CLOUDSQL,
    

    /// Amazon RDS is the source instance provider.
    ///
    /// "RDS"
    #[serde(rename="RDS")]
    RDS,
    

    /// Amazon Aurora is the source instance provider.
    ///
    /// "AURORA"
    #[serde(rename="AURORA")]
    AURORA,
    

    /// AlloyDB for PostgreSQL is the source instance provider.
    ///
    /// "ALLOYDB"
    #[serde(rename="ALLOYDB")]
    ALLOYDB,
}

impl AsRef<str> for ConnectionProfileProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionProfileProviderEnum::DATABASEPROVIDERUNSPECIFIED => "DATABASE_PROVIDER_UNSPECIFIED",
            ConnectionProfileProviderEnum::CLOUDSQL => "CLOUDSQL",
            ConnectionProfileProviderEnum::RDS => "RDS",
            ConnectionProfileProviderEnum::AURORA => "AURORA",
            ConnectionProfileProviderEnum::ALLOYDB => "ALLOYDB",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionProfileProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_PROVIDER_UNSPECIFIED" => Ok(ConnectionProfileProviderEnum::DATABASEPROVIDERUNSPECIFIED),
           "CLOUDSQL" => Ok(ConnectionProfileProviderEnum::CLOUDSQL),
           "RDS" => Ok(ConnectionProfileProviderEnum::RDS),
           "AURORA" => Ok(ConnectionProfileProviderEnum::AURORA),
           "ALLOYDB" => Ok(ConnectionProfileProviderEnum::ALLOYDB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionProfileProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConnectionProfileStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current connection profile state (e.g. DRAFT, READY, or FAILED).
pub enum ConnectionProfileStateEnum {
    

    /// The state of the connection profile is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The connection profile is in draft mode and fully editable.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// The connection profile is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The connection profile is ready.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The connection profile is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The connection profile is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The connection profile has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The last action on the connection profile failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for ConnectionProfileStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConnectionProfileStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ConnectionProfileStateEnum::DRAFT => "DRAFT",
            ConnectionProfileStateEnum::CREATING => "CREATING",
            ConnectionProfileStateEnum::READY => "READY",
            ConnectionProfileStateEnum::UPDATING => "UPDATING",
            ConnectionProfileStateEnum::DELETING => "DELETING",
            ConnectionProfileStateEnum::DELETED => "DELETED",
            ConnectionProfileStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConnectionProfileStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ConnectionProfileStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(ConnectionProfileStateEnum::DRAFT),
           "CREATING" => Ok(ConnectionProfileStateEnum::CREATING),
           "READY" => Ok(ConnectionProfileStateEnum::READY),
           "UPDATING" => Ok(ConnectionProfileStateEnum::UPDATING),
           "DELETING" => Ok(ConnectionProfileStateEnum::DELETING),
           "DELETED" => Ok(ConnectionProfileStateEnum::DELETED),
           "FAILED" => Ok(ConnectionProfileStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConnectionProfileStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseEngineInfoEngineEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Engine type.
pub enum DatabaseEngineInfoEngineEnum {
    

    /// The source database engine of the migration job is unknown.
    ///
    /// "DATABASE_ENGINE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENGINE_UNSPECIFIED")]
    DATABASEENGINEUNSPECIFIED,
    

    /// The source engine is MySQL.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// The source engine is PostgreSQL.
    ///
    /// "POSTGRESQL"
    #[serde(rename="POSTGRESQL")]
    POSTGRESQL,
    

    /// The source engine is SQL Server.
    ///
    /// "SQLSERVER"
    #[serde(rename="SQLSERVER")]
    SQLSERVER,
    

    /// The source engine is Oracle.
    ///
    /// "ORACLE"
    #[serde(rename="ORACLE")]
    ORACLE,
}

impl AsRef<str> for DatabaseEngineInfoEngineEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseEngineInfoEngineEnum::DATABASEENGINEUNSPECIFIED => "DATABASE_ENGINE_UNSPECIFIED",
            DatabaseEngineInfoEngineEnum::MYSQL => "MYSQL",
            DatabaseEngineInfoEngineEnum::POSTGRESQL => "POSTGRESQL",
            DatabaseEngineInfoEngineEnum::SQLSERVER => "SQLSERVER",
            DatabaseEngineInfoEngineEnum::ORACLE => "ORACLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseEngineInfoEngineEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENGINE_UNSPECIFIED" => Ok(DatabaseEngineInfoEngineEnum::DATABASEENGINEUNSPECIFIED),
           "MYSQL" => Ok(DatabaseEngineInfoEngineEnum::MYSQL),
           "POSTGRESQL" => Ok(DatabaseEngineInfoEngineEnum::POSTGRESQL),
           "SQLSERVER" => Ok(DatabaseEngineInfoEngineEnum::SQLSERVER),
           "ORACLE" => Ok(DatabaseEngineInfoEngineEnum::ORACLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseEngineInfoEngineEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseEntityEntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the database entity (table, view, index, ...).
pub enum DatabaseEntityEntityTypeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for DatabaseEntityEntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseEntityEntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(DatabaseEntityEntityTypeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseEntityEntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseEntityTreeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of tree the entity belongs to.
pub enum DatabaseEntityTreeEnum {
    

    /// Tree type unspecified.
    ///
    /// "TREE_TYPE_UNSPECIFIED"
    #[serde(rename="TREE_TYPE_UNSPECIFIED")]
    TREETYPEUNSPECIFIED,
    

    /// Tree of entities loaded from a source database.
    ///
    /// "SOURCE"
    #[serde(rename="SOURCE")]
    SOURCE,
    

    /// Tree of entities converted from the source tree using the mapping rules.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Tree of entities observed on the destination database.
    ///
    /// "DESTINATION"
    #[serde(rename="DESTINATION")]
    DESTINATION,
}

impl AsRef<str> for DatabaseEntityTreeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseEntityTreeEnum::TREETYPEUNSPECIFIED => "TREE_TYPE_UNSPECIFIED",
            DatabaseEntityTreeEnum::SOURCE => "SOURCE",
            DatabaseEntityTreeEnum::DRAFT => "DRAFT",
            DatabaseEntityTreeEnum::DESTINATION => "DESTINATION",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseEntityTreeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TREE_TYPE_UNSPECIFIED" => Ok(DatabaseEntityTreeEnum::TREETYPEUNSPECIFIED),
           "SOURCE" => Ok(DatabaseEntityTreeEnum::SOURCE),
           "DRAFT" => Ok(DatabaseEntityTreeEnum::DRAFT),
           "DESTINATION" => Ok(DatabaseEntityTreeEnum::DESTINATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseEntityTreeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseTypeEngineEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database engine.
pub enum DatabaseTypeEngineEnum {
    

    /// The source database engine of the migration job is unknown.
    ///
    /// "DATABASE_ENGINE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENGINE_UNSPECIFIED")]
    DATABASEENGINEUNSPECIFIED,
    

    /// The source engine is MySQL.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// The source engine is PostgreSQL.
    ///
    /// "POSTGRESQL"
    #[serde(rename="POSTGRESQL")]
    POSTGRESQL,
    

    /// The source engine is SQL Server.
    ///
    /// "SQLSERVER"
    #[serde(rename="SQLSERVER")]
    SQLSERVER,
    

    /// The source engine is Oracle.
    ///
    /// "ORACLE"
    #[serde(rename="ORACLE")]
    ORACLE,
}

impl AsRef<str> for DatabaseTypeEngineEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseTypeEngineEnum::DATABASEENGINEUNSPECIFIED => "DATABASE_ENGINE_UNSPECIFIED",
            DatabaseTypeEngineEnum::MYSQL => "MYSQL",
            DatabaseTypeEngineEnum::POSTGRESQL => "POSTGRESQL",
            DatabaseTypeEngineEnum::SQLSERVER => "SQLSERVER",
            DatabaseTypeEngineEnum::ORACLE => "ORACLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseTypeEngineEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENGINE_UNSPECIFIED" => Ok(DatabaseTypeEngineEnum::DATABASEENGINEUNSPECIFIED),
           "MYSQL" => Ok(DatabaseTypeEngineEnum::MYSQL),
           "POSTGRESQL" => Ok(DatabaseTypeEngineEnum::POSTGRESQL),
           "SQLSERVER" => Ok(DatabaseTypeEngineEnum::SQLSERVER),
           "ORACLE" => Ok(DatabaseTypeEngineEnum::ORACLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseTypeEngineEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseTypeProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The database provider.
pub enum DatabaseTypeProviderEnum {
    

    /// Use this value for on-premise source database instances.
    ///
    /// "DATABASE_PROVIDER_UNSPECIFIED"
    #[serde(rename="DATABASE_PROVIDER_UNSPECIFIED")]
    DATABASEPROVIDERUNSPECIFIED,
    

    /// Cloud SQL is the source instance provider.
    ///
    /// "CLOUDSQL"
    #[serde(rename="CLOUDSQL")]
    CLOUDSQL,
    

    /// Amazon RDS is the source instance provider.
    ///
    /// "RDS"
    #[serde(rename="RDS")]
    RDS,
    

    /// Amazon Aurora is the source instance provider.
    ///
    /// "AURORA"
    #[serde(rename="AURORA")]
    AURORA,
    

    /// AlloyDB for PostgreSQL is the source instance provider.
    ///
    /// "ALLOYDB"
    #[serde(rename="ALLOYDB")]
    ALLOYDB,
}

impl AsRef<str> for DatabaseTypeProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseTypeProviderEnum::DATABASEPROVIDERUNSPECIFIED => "DATABASE_PROVIDER_UNSPECIFIED",
            DatabaseTypeProviderEnum::CLOUDSQL => "CLOUDSQL",
            DatabaseTypeProviderEnum::RDS => "RDS",
            DatabaseTypeProviderEnum::AURORA => "AURORA",
            DatabaseTypeProviderEnum::ALLOYDB => "ALLOYDB",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseTypeProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_PROVIDER_UNSPECIFIED" => Ok(DatabaseTypeProviderEnum::DATABASEPROVIDERUNSPECIFIED),
           "CLOUDSQL" => Ok(DatabaseTypeProviderEnum::CLOUDSQL),
           "RDS" => Ok(DatabaseTypeProviderEnum::RDS),
           "AURORA" => Ok(DatabaseTypeProviderEnum::AURORA),
           "ALLOYDB" => Ok(DatabaseTypeProviderEnum::ALLOYDB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseTypeProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DoubleComparisonFilterValueComparisonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Relation between source value and compare value
pub enum DoubleComparisonFilterValueComparisonEnum {
    

    /// Value comparison unspecified.
    ///
    /// "VALUE_COMPARISON_UNSPECIFIED"
    #[serde(rename="VALUE_COMPARISON_UNSPECIFIED")]
    VALUECOMPARISONUNSPECIFIED,
    

    /// Value is smaller than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_SMALLER_THAN")]
    VALUECOMPARISONIFVALUESMALLERTHAN,
    

    /// Value is smaller or equal than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN")]
    VALUECOMPARISONIFVALUESMALLEREQUALTHAN,
    

    /// Value is larger than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_LARGER_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_LARGER_THAN")]
    VALUECOMPARISONIFVALUELARGERTHAN,
    

    /// Value is larger or equal than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN")]
    VALUECOMPARISONIFVALUELARGEREQUALTHAN,
}

impl AsRef<str> for DoubleComparisonFilterValueComparisonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONUNSPECIFIED => "VALUE_COMPARISON_UNSPECIFIED",
            DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLERTHAN => "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN",
            DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLEREQUALTHAN => "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN",
            DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGERTHAN => "VALUE_COMPARISON_IF_VALUE_LARGER_THAN",
            DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGEREQUALTHAN => "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN",
        }
    }
}

impl std::convert::TryFrom< &str> for DoubleComparisonFilterValueComparisonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_COMPARISON_UNSPECIFIED" => Ok(DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONUNSPECIFIED),
           "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN" => Ok(DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLERTHAN),
           "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN" => Ok(DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLEREQUALTHAN),
           "VALUE_COMPARISON_IF_VALUE_LARGER_THAN" => Ok(DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGERTHAN),
           "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN" => Ok(DoubleComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGEREQUALTHAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DoubleComparisonFilterValueComparisonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityDdlEntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entity type (if the DDL is for a sub entity).
pub enum EntityDdlEntityTypeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for EntityDdlEntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            EntityDdlEntityTypeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityDdlEntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(EntityDdlEntityTypeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityDdlEntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityIssueEntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entity type (if the DDL is for a sub entity).
pub enum EntityIssueEntityTypeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for EntityIssueEntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            EntityIssueEntityTypeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityIssueEntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(EntityIssueEntityTypeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityIssueEntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityIssueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Severity of the issue
pub enum EntityIssueSeverityEnum {
    

    /// Unspecified issue severity
    ///
    /// "ISSUE_SEVERITY_UNSPECIFIED"
    #[serde(rename="ISSUE_SEVERITY_UNSPECIFIED")]
    ISSUESEVERITYUNSPECIFIED,
    

    /// Info
    ///
    /// "ISSUE_SEVERITY_INFO"
    #[serde(rename="ISSUE_SEVERITY_INFO")]
    ISSUESEVERITYINFO,
    

    /// Warning
    ///
    /// "ISSUE_SEVERITY_WARNING"
    #[serde(rename="ISSUE_SEVERITY_WARNING")]
    ISSUESEVERITYWARNING,
    

    /// Error
    ///
    /// "ISSUE_SEVERITY_ERROR"
    #[serde(rename="ISSUE_SEVERITY_ERROR")]
    ISSUESEVERITYERROR,
}

impl AsRef<str> for EntityIssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityIssueSeverityEnum::ISSUESEVERITYUNSPECIFIED => "ISSUE_SEVERITY_UNSPECIFIED",
            EntityIssueSeverityEnum::ISSUESEVERITYINFO => "ISSUE_SEVERITY_INFO",
            EntityIssueSeverityEnum::ISSUESEVERITYWARNING => "ISSUE_SEVERITY_WARNING",
            EntityIssueSeverityEnum::ISSUESEVERITYERROR => "ISSUE_SEVERITY_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityIssueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ISSUE_SEVERITY_UNSPECIFIED" => Ok(EntityIssueSeverityEnum::ISSUESEVERITYUNSPECIFIED),
           "ISSUE_SEVERITY_INFO" => Ok(EntityIssueSeverityEnum::ISSUESEVERITYINFO),
           "ISSUE_SEVERITY_WARNING" => Ok(EntityIssueSeverityEnum::ISSUESEVERITYWARNING),
           "ISSUE_SEVERITY_ERROR" => Ok(EntityIssueSeverityEnum::ISSUESEVERITYERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityIssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityIssueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the issue.
pub enum EntityIssueTypeEnum {
    

    /// Unspecified issue type.
    ///
    /// "ISSUE_TYPE_UNSPECIFIED"
    #[serde(rename="ISSUE_TYPE_UNSPECIFIED")]
    ISSUETYPEUNSPECIFIED,
    

    /// Issue originated from the DDL
    ///
    /// "ISSUE_TYPE_DDL"
    #[serde(rename="ISSUE_TYPE_DDL")]
    ISSUETYPEDDL,
    

    /// Issue originated during the apply process
    ///
    /// "ISSUE_TYPE_APPLY"
    #[serde(rename="ISSUE_TYPE_APPLY")]
    ISSUETYPEAPPLY,
    

    /// Issue originated during the convert process
    ///
    /// "ISSUE_TYPE_CONVERT"
    #[serde(rename="ISSUE_TYPE_CONVERT")]
    ISSUETYPECONVERT,
}

impl AsRef<str> for EntityIssueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityIssueTypeEnum::ISSUETYPEUNSPECIFIED => "ISSUE_TYPE_UNSPECIFIED",
            EntityIssueTypeEnum::ISSUETYPEDDL => "ISSUE_TYPE_DDL",
            EntityIssueTypeEnum::ISSUETYPEAPPLY => "ISSUE_TYPE_APPLY",
            EntityIssueTypeEnum::ISSUETYPECONVERT => "ISSUE_TYPE_CONVERT",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityIssueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ISSUE_TYPE_UNSPECIFIED" => Ok(EntityIssueTypeEnum::ISSUETYPEUNSPECIFIED),
           "ISSUE_TYPE_DDL" => Ok(EntityIssueTypeEnum::ISSUETYPEDDL),
           "ISSUE_TYPE_APPLY" => Ok(EntityIssueTypeEnum::ISSUETYPEAPPLY),
           "ISSUE_TYPE_CONVERT" => Ok(EntityIssueTypeEnum::ISSUETYPECONVERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityIssueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityMappingDraftTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of draft entity.
pub enum EntityMappingDraftTypeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for EntityMappingDraftTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            EntityMappingDraftTypeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityMappingDraftTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(EntityMappingDraftTypeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityMappingDraftTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntityMappingSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of source entity.
pub enum EntityMappingSourceTypeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for EntityMappingSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            EntityMappingSourceTypeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityMappingSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(EntityMappingSourceTypeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityMappingSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportMappingRulesRequestRulesFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The format of the rules content file.
pub enum ImportMappingRulesRequestRulesFormatEnum {
    

    /// Unspecified rules format.
    ///
    /// "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED"
    #[serde(rename="IMPORT_RULES_FILE_FORMAT_UNSPECIFIED")]
    IMPORTRULESFILEFORMATUNSPECIFIED,
    

    /// HarbourBridge session file.
    ///
    /// "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE"
    #[serde(rename="IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE")]
    IMPORTRULESFILEFORMATHARBOURBRIDGESESSIONFILE,
    

    /// Ora2Pg configuration file.
    ///
    /// "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE"
    #[serde(rename="IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE")]
    IMPORTRULESFILEFORMATORATOPGCONFIGFILE,
}

impl AsRef<str> for ImportMappingRulesRequestRulesFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportMappingRulesRequestRulesFormatEnum::IMPORTRULESFILEFORMATUNSPECIFIED => "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED",
            ImportMappingRulesRequestRulesFormatEnum::IMPORTRULESFILEFORMATHARBOURBRIDGESESSIONFILE => "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE",
            ImportMappingRulesRequestRulesFormatEnum::IMPORTRULESFILEFORMATORATOPGCONFIGFILE => "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportMappingRulesRequestRulesFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED" => Ok(ImportMappingRulesRequestRulesFormatEnum::IMPORTRULESFILEFORMATUNSPECIFIED),
           "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE" => Ok(ImportMappingRulesRequestRulesFormatEnum::IMPORTRULESFILEFORMATHARBOURBRIDGESESSIONFILE),
           "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE" => Ok(ImportMappingRulesRequestRulesFormatEnum::IMPORTRULESFILEFORMATORATOPGCONFIGFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportMappingRulesRequestRulesFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImportRulesJobDetailFileFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The requested file format.
pub enum ImportRulesJobDetailFileFormatEnum {
    

    /// Unspecified rules format.
    ///
    /// "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED"
    #[serde(rename="IMPORT_RULES_FILE_FORMAT_UNSPECIFIED")]
    IMPORTRULESFILEFORMATUNSPECIFIED,
    

    /// HarbourBridge session file.
    ///
    /// "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE"
    #[serde(rename="IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE")]
    IMPORTRULESFILEFORMATHARBOURBRIDGESESSIONFILE,
    

    /// Ora2Pg configuration file.
    ///
    /// "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE"
    #[serde(rename="IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE")]
    IMPORTRULESFILEFORMATORATOPGCONFIGFILE,
}

impl AsRef<str> for ImportRulesJobDetailFileFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImportRulesJobDetailFileFormatEnum::IMPORTRULESFILEFORMATUNSPECIFIED => "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED",
            ImportRulesJobDetailFileFormatEnum::IMPORTRULESFILEFORMATHARBOURBRIDGESESSIONFILE => "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE",
            ImportRulesJobDetailFileFormatEnum::IMPORTRULESFILEFORMATORATOPGCONFIGFILE => "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE",
        }
    }
}

impl std::convert::TryFrom< &str> for ImportRulesJobDetailFileFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORT_RULES_FILE_FORMAT_UNSPECIFIED" => Ok(ImportRulesJobDetailFileFormatEnum::IMPORTRULESFILEFORMATUNSPECIFIED),
           "IMPORT_RULES_FILE_FORMAT_HARBOUR_BRIDGE_SESSION_FILE" => Ok(ImportRulesJobDetailFileFormatEnum::IMPORTRULESFILEFORMATHARBOURBRIDGESESSIONFILE),
           "IMPORT_RULES_FILE_FORMAT_ORATOPG_CONFIG_FILE" => Ok(ImportRulesJobDetailFileFormatEnum::IMPORTRULESFILEFORMATORATOPGCONFIGFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImportRulesJobDetailFileFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IntComparisonFilterValueComparisonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Relation between source value and compare value
pub enum IntComparisonFilterValueComparisonEnum {
    

    /// Value comparison unspecified.
    ///
    /// "VALUE_COMPARISON_UNSPECIFIED"
    #[serde(rename="VALUE_COMPARISON_UNSPECIFIED")]
    VALUECOMPARISONUNSPECIFIED,
    

    /// Value is smaller than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_SMALLER_THAN")]
    VALUECOMPARISONIFVALUESMALLERTHAN,
    

    /// Value is smaller or equal than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN")]
    VALUECOMPARISONIFVALUESMALLEREQUALTHAN,
    

    /// Value is larger than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_LARGER_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_LARGER_THAN")]
    VALUECOMPARISONIFVALUELARGERTHAN,
    

    /// Value is larger or equal than the Compare value.
    ///
    /// "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN"
    #[serde(rename="VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN")]
    VALUECOMPARISONIFVALUELARGEREQUALTHAN,
}

impl AsRef<str> for IntComparisonFilterValueComparisonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IntComparisonFilterValueComparisonEnum::VALUECOMPARISONUNSPECIFIED => "VALUE_COMPARISON_UNSPECIFIED",
            IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLERTHAN => "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN",
            IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLEREQUALTHAN => "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN",
            IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGERTHAN => "VALUE_COMPARISON_IF_VALUE_LARGER_THAN",
            IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGEREQUALTHAN => "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN",
        }
    }
}

impl std::convert::TryFrom< &str> for IntComparisonFilterValueComparisonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_COMPARISON_UNSPECIFIED" => Ok(IntComparisonFilterValueComparisonEnum::VALUECOMPARISONUNSPECIFIED),
           "VALUE_COMPARISON_IF_VALUE_SMALLER_THAN" => Ok(IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLERTHAN),
           "VALUE_COMPARISON_IF_VALUE_SMALLER_EQUAL_THAN" => Ok(IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUESMALLEREQUALTHAN),
           "VALUE_COMPARISON_IF_VALUE_LARGER_THAN" => Ok(IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGERTHAN),
           "VALUE_COMPARISON_IF_VALUE_LARGER_EQUAL_THAN" => Ok(IntComparisonFilterValueComparisonEnum::VALUECOMPARISONIFVALUELARGEREQUALTHAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IntComparisonFilterValueComparisonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MappingRuleRuleScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The rule scope
pub enum MappingRuleRuleScopeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for MappingRuleRuleScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            MappingRuleRuleScopeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for MappingRuleRuleScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(MappingRuleRuleScopeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MappingRuleRuleScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MappingRuleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The mapping rule state
pub enum MappingRuleStateEnum {
    

    /// The state of the mapping rule is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The rule is enabled.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The rule is disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The rule is logically deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for MappingRuleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MappingRuleStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MappingRuleStateEnum::ENABLED => "ENABLED",
            MappingRuleStateEnum::DISABLED => "DISABLED",
            MappingRuleStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for MappingRuleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MappingRuleStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(MappingRuleStateEnum::ENABLED),
           "DISABLED" => Ok(MappingRuleStateEnum::DISABLED),
           "DELETED" => Ok(MappingRuleStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MappingRuleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationJobDumpTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of the data dump. Supported for MySQL to CloudSQL for MySQL migrations only.
pub enum MigrationJobDumpTypeEnum {
    

    /// If not specified, defaults to LOGICAL
    ///
    /// "DUMP_TYPE_UNSPECIFIED"
    #[serde(rename="DUMP_TYPE_UNSPECIFIED")]
    DUMPTYPEUNSPECIFIED,
    

    /// Logical dump.
    ///
    /// "LOGICAL"
    #[serde(rename="LOGICAL")]
    LOGICAL,
    

    /// Physical file-based dump. Supported for MySQL to CloudSQL for MySQL migrations only.
    ///
    /// "PHYSICAL"
    #[serde(rename="PHYSICAL")]
    PHYSICAL,
}

impl AsRef<str> for MigrationJobDumpTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationJobDumpTypeEnum::DUMPTYPEUNSPECIFIED => "DUMP_TYPE_UNSPECIFIED",
            MigrationJobDumpTypeEnum::LOGICAL => "LOGICAL",
            MigrationJobDumpTypeEnum::PHYSICAL => "PHYSICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationJobDumpTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DUMP_TYPE_UNSPECIFIED" => Ok(MigrationJobDumpTypeEnum::DUMPTYPEUNSPECIFIED),
           "LOGICAL" => Ok(MigrationJobDumpTypeEnum::LOGICAL),
           "PHYSICAL" => Ok(MigrationJobDumpTypeEnum::PHYSICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationJobDumpTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationJobPhaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current migration job phase.
pub enum MigrationJobPhaseEnum {
    

    /// The phase of the migration job is unknown.
    ///
    /// "PHASE_UNSPECIFIED"
    #[serde(rename="PHASE_UNSPECIFIED")]
    PHASEUNSPECIFIED,
    

    /// The migration job is in the full dump phase.
    ///
    /// "FULL_DUMP"
    #[serde(rename="FULL_DUMP")]
    FULLDUMP,
    

    /// The migration job is CDC phase.
    ///
    /// "CDC"
    #[serde(rename="CDC")]
    CDC,
    

    /// The migration job is running the promote phase.
    ///
    /// "PROMOTE_IN_PROGRESS"
    #[serde(rename="PROMOTE_IN_PROGRESS")]
    PROMOTEINPROGRESS,
    

    /// Only RDS flow - waiting for source writes to stop
    ///
    /// "WAITING_FOR_SOURCE_WRITES_TO_STOP"
    #[serde(rename="WAITING_FOR_SOURCE_WRITES_TO_STOP")]
    WAITINGFORSOURCEWRITESTOSTOP,
    

    /// Only RDS flow - the sources writes stopped, waiting for dump to begin
    ///
    /// "PREPARING_THE_DUMP"
    #[serde(rename="PREPARING_THE_DUMP")]
    PREPARINGTHEDUMP,
    

    /// The migration job is ready to be promoted.
    ///
    /// "READY_FOR_PROMOTE"
    #[serde(rename="READY_FOR_PROMOTE")]
    READYFORPROMOTE,
}

impl AsRef<str> for MigrationJobPhaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationJobPhaseEnum::PHASEUNSPECIFIED => "PHASE_UNSPECIFIED",
            MigrationJobPhaseEnum::FULLDUMP => "FULL_DUMP",
            MigrationJobPhaseEnum::CDC => "CDC",
            MigrationJobPhaseEnum::PROMOTEINPROGRESS => "PROMOTE_IN_PROGRESS",
            MigrationJobPhaseEnum::WAITINGFORSOURCEWRITESTOSTOP => "WAITING_FOR_SOURCE_WRITES_TO_STOP",
            MigrationJobPhaseEnum::PREPARINGTHEDUMP => "PREPARING_THE_DUMP",
            MigrationJobPhaseEnum::READYFORPROMOTE => "READY_FOR_PROMOTE",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationJobPhaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PHASE_UNSPECIFIED" => Ok(MigrationJobPhaseEnum::PHASEUNSPECIFIED),
           "FULL_DUMP" => Ok(MigrationJobPhaseEnum::FULLDUMP),
           "CDC" => Ok(MigrationJobPhaseEnum::CDC),
           "PROMOTE_IN_PROGRESS" => Ok(MigrationJobPhaseEnum::PROMOTEINPROGRESS),
           "WAITING_FOR_SOURCE_WRITES_TO_STOP" => Ok(MigrationJobPhaseEnum::WAITINGFORSOURCEWRITESTOSTOP),
           "PREPARING_THE_DUMP" => Ok(MigrationJobPhaseEnum::PREPARINGTHEDUMP),
           "READY_FOR_PROMOTE" => Ok(MigrationJobPhaseEnum::READYFORPROMOTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationJobPhaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current migration job state.
pub enum MigrationJobStateEnum {
    

    /// The state of the migration job is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The migration job is down for maintenance.
    ///
    /// "MAINTENANCE"
    #[serde(rename="MAINTENANCE")]
    MAINTENANCE,
    

    /// The migration job is in draft mode and no resources are created.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// The migration job is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The migration job is created and not started.
    ///
    /// "NOT_STARTED"
    #[serde(rename="NOT_STARTED")]
    NOTSTARTED,
    

    /// The migration job is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The migration job failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The migration job has been completed.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// The migration job is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The migration job is being stopped.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The migration job is currently stopped.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// The migration job has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The migration job is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The migration job is starting.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The migration job is restarting.
    ///
    /// "RESTARTING"
    #[serde(rename="RESTARTING")]
    RESTARTING,
    

    /// The migration job is resuming.
    ///
    /// "RESUMING"
    #[serde(rename="RESUMING")]
    RESUMING,
}

impl AsRef<str> for MigrationJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MigrationJobStateEnum::MAINTENANCE => "MAINTENANCE",
            MigrationJobStateEnum::DRAFT => "DRAFT",
            MigrationJobStateEnum::CREATING => "CREATING",
            MigrationJobStateEnum::NOTSTARTED => "NOT_STARTED",
            MigrationJobStateEnum::RUNNING => "RUNNING",
            MigrationJobStateEnum::FAILED => "FAILED",
            MigrationJobStateEnum::COMPLETED => "COMPLETED",
            MigrationJobStateEnum::DELETING => "DELETING",
            MigrationJobStateEnum::STOPPING => "STOPPING",
            MigrationJobStateEnum::STOPPED => "STOPPED",
            MigrationJobStateEnum::DELETED => "DELETED",
            MigrationJobStateEnum::UPDATING => "UPDATING",
            MigrationJobStateEnum::STARTING => "STARTING",
            MigrationJobStateEnum::RESTARTING => "RESTARTING",
            MigrationJobStateEnum::RESUMING => "RESUMING",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MigrationJobStateEnum::STATEUNSPECIFIED),
           "MAINTENANCE" => Ok(MigrationJobStateEnum::MAINTENANCE),
           "DRAFT" => Ok(MigrationJobStateEnum::DRAFT),
           "CREATING" => Ok(MigrationJobStateEnum::CREATING),
           "NOT_STARTED" => Ok(MigrationJobStateEnum::NOTSTARTED),
           "RUNNING" => Ok(MigrationJobStateEnum::RUNNING),
           "FAILED" => Ok(MigrationJobStateEnum::FAILED),
           "COMPLETED" => Ok(MigrationJobStateEnum::COMPLETED),
           "DELETING" => Ok(MigrationJobStateEnum::DELETING),
           "STOPPING" => Ok(MigrationJobStateEnum::STOPPING),
           "STOPPED" => Ok(MigrationJobStateEnum::STOPPED),
           "DELETED" => Ok(MigrationJobStateEnum::DELETED),
           "UPDATING" => Ok(MigrationJobStateEnum::UPDATING),
           "STARTING" => Ok(MigrationJobStateEnum::STARTING),
           "RESTARTING" => Ok(MigrationJobStateEnum::RESTARTING),
           "RESUMING" => Ok(MigrationJobStateEnum::RESUMING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationJobTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The migration job type.
pub enum MigrationJobTypeEnum {
    

    /// The type of the migration job is unknown.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The migration job is a one time migration.
    ///
    /// "ONE_TIME"
    #[serde(rename="ONE_TIME")]
    ONETIME,
    

    /// The migration job is a continuous migration.
    ///
    /// "CONTINUOUS"
    #[serde(rename="CONTINUOUS")]
    CONTINUOUS,
}

impl AsRef<str> for MigrationJobTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationJobTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            MigrationJobTypeEnum::ONETIME => "ONE_TIME",
            MigrationJobTypeEnum::CONTINUOUS => "CONTINUOUS",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationJobTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(MigrationJobTypeEnum::TYPEUNSPECIFIED),
           "ONE_TIME" => Ok(MigrationJobTypeEnum::ONETIME),
           "CONTINUOUS" => Ok(MigrationJobTypeEnum::CONTINUOUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationJobTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MultiEntityRenameSourceNameTransformationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Additional transformation that can be done on the source entity name before it is being used by the new_name_pattern, for example lower case. If no transformation is desired, use NO_TRANSFORMATION
pub enum MultiEntityRenameSourceNameTransformationEnum {
    

    /// Entity name transformation unspecified.
    ///
    /// "ENTITY_NAME_TRANSFORMATION_UNSPECIFIED"
    #[serde(rename="ENTITY_NAME_TRANSFORMATION_UNSPECIFIED")]
    ENTITYNAMETRANSFORMATIONUNSPECIFIED,
    

    /// No transformation.
    ///
    /// "ENTITY_NAME_TRANSFORMATION_NO_TRANSFORMATION"
    #[serde(rename="ENTITY_NAME_TRANSFORMATION_NO_TRANSFORMATION")]
    ENTITYNAMETRANSFORMATIONNOTRANSFORMATION,
    

    /// Transform to lower case.
    ///
    /// "ENTITY_NAME_TRANSFORMATION_LOWER_CASE"
    #[serde(rename="ENTITY_NAME_TRANSFORMATION_LOWER_CASE")]
    ENTITYNAMETRANSFORMATIONLOWERCASE,
    

    /// Transform to upper case.
    ///
    /// "ENTITY_NAME_TRANSFORMATION_UPPER_CASE"
    #[serde(rename="ENTITY_NAME_TRANSFORMATION_UPPER_CASE")]
    ENTITYNAMETRANSFORMATIONUPPERCASE,
    

    /// Transform to capitalized case.
    ///
    /// "ENTITY_NAME_TRANSFORMATION_CAPITALIZED_CASE"
    #[serde(rename="ENTITY_NAME_TRANSFORMATION_CAPITALIZED_CASE")]
    ENTITYNAMETRANSFORMATIONCAPITALIZEDCASE,
}

impl AsRef<str> for MultiEntityRenameSourceNameTransformationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONUNSPECIFIED => "ENTITY_NAME_TRANSFORMATION_UNSPECIFIED",
            MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONNOTRANSFORMATION => "ENTITY_NAME_TRANSFORMATION_NO_TRANSFORMATION",
            MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONLOWERCASE => "ENTITY_NAME_TRANSFORMATION_LOWER_CASE",
            MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONUPPERCASE => "ENTITY_NAME_TRANSFORMATION_UPPER_CASE",
            MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONCAPITALIZEDCASE => "ENTITY_NAME_TRANSFORMATION_CAPITALIZED_CASE",
        }
    }
}

impl std::convert::TryFrom< &str> for MultiEntityRenameSourceNameTransformationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_NAME_TRANSFORMATION_UNSPECIFIED" => Ok(MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONUNSPECIFIED),
           "ENTITY_NAME_TRANSFORMATION_NO_TRANSFORMATION" => Ok(MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONNOTRANSFORMATION),
           "ENTITY_NAME_TRANSFORMATION_LOWER_CASE" => Ok(MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONLOWERCASE),
           "ENTITY_NAME_TRANSFORMATION_UPPER_CASE" => Ok(MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONUPPERCASE),
           "ENTITY_NAME_TRANSFORMATION_CAPITALIZED_CASE" => Ok(MultiEntityRenameSourceNameTransformationEnum::ENTITYNAMETRANSFORMATIONCAPITALIZEDCASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MultiEntityRenameSourceNameTransformationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PerformanceConfigDumpParallelLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Initial dump parallelism level.
pub enum PerformanceConfigDumpParallelLevelEnum {
    

    /// Unknown dump parallel level. Will be defaulted to OPTIMAL.
    ///
    /// "DUMP_PARALLEL_LEVEL_UNSPECIFIED"
    #[serde(rename="DUMP_PARALLEL_LEVEL_UNSPECIFIED")]
    DUMPPARALLELLEVELUNSPECIFIED,
    

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

impl AsRef<str> for PerformanceConfigDumpParallelLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PerformanceConfigDumpParallelLevelEnum::DUMPPARALLELLEVELUNSPECIFIED => "DUMP_PARALLEL_LEVEL_UNSPECIFIED",
            PerformanceConfigDumpParallelLevelEnum::MIN => "MIN",
            PerformanceConfigDumpParallelLevelEnum::OPTIMAL => "OPTIMAL",
            PerformanceConfigDumpParallelLevelEnum::MAX => "MAX",
        }
    }
}

impl std::convert::TryFrom< &str> for PerformanceConfigDumpParallelLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DUMP_PARALLEL_LEVEL_UNSPECIFIED" => Ok(PerformanceConfigDumpParallelLevelEnum::DUMPPARALLELLEVELUNSPECIFIED),
           "MIN" => Ok(PerformanceConfigDumpParallelLevelEnum::MIN),
           "OPTIMAL" => Ok(PerformanceConfigDumpParallelLevelEnum::OPTIMAL),
           "MAX" => Ok(PerformanceConfigDumpParallelLevelEnum::MAX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PerformanceConfigDumpParallelLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostgreSqlConnectionProfileNetworkArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with.
pub enum PostgreSqlConnectionProfileNetworkArchitectureEnum {
    
    /// "NETWORK_ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="NETWORK_ARCHITECTURE_UNSPECIFIED")]
    NETWORKARCHITECTUREUNSPECIFIED,
    

    /// Instance is in Cloud SQL's old producer network architecture.
    ///
    /// "NETWORK_ARCHITECTURE_OLD_CSQL_PRODUCER"
    #[serde(rename="NETWORK_ARCHITECTURE_OLD_CSQL_PRODUCER")]
    NETWORKARCHITECTUREOLDCSQLPRODUCER,
    

    /// Instance is in Cloud SQL's new producer network architecture.
    ///
    /// "NETWORK_ARCHITECTURE_NEW_CSQL_PRODUCER"
    #[serde(rename="NETWORK_ARCHITECTURE_NEW_CSQL_PRODUCER")]
    NETWORKARCHITECTURENEWCSQLPRODUCER,
}

impl AsRef<str> for PostgreSqlConnectionProfileNetworkArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostgreSqlConnectionProfileNetworkArchitectureEnum::NETWORKARCHITECTUREUNSPECIFIED => "NETWORK_ARCHITECTURE_UNSPECIFIED",
            PostgreSqlConnectionProfileNetworkArchitectureEnum::NETWORKARCHITECTUREOLDCSQLPRODUCER => "NETWORK_ARCHITECTURE_OLD_CSQL_PRODUCER",
            PostgreSqlConnectionProfileNetworkArchitectureEnum::NETWORKARCHITECTURENEWCSQLPRODUCER => "NETWORK_ARCHITECTURE_NEW_CSQL_PRODUCER",
        }
    }
}

impl std::convert::TryFrom< &str> for PostgreSqlConnectionProfileNetworkArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_ARCHITECTURE_UNSPECIFIED" => Ok(PostgreSqlConnectionProfileNetworkArchitectureEnum::NETWORKARCHITECTUREUNSPECIFIED),
           "NETWORK_ARCHITECTURE_OLD_CSQL_PRODUCER" => Ok(PostgreSqlConnectionProfileNetworkArchitectureEnum::NETWORKARCHITECTUREOLDCSQLPRODUCER),
           "NETWORK_ARCHITECTURE_NEW_CSQL_PRODUCER" => Ok(PostgreSqlConnectionProfileNetworkArchitectureEnum::NETWORKARCHITECTURENEWCSQLPRODUCER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostgreSqlConnectionProfileNetworkArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PrivateConnectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the private connection.
pub enum PrivateConnectionStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The private connection is in creation state - creating resources.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The private connection has been created with all of its resources.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// The private connection creation has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The private connection is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Delete request has failed, resource is in invalid state.
    ///
    /// "FAILED_TO_DELETE"
    #[serde(rename="FAILED_TO_DELETE")]
    FAILEDTODELETE,
    

    /// The private connection has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for PrivateConnectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PrivateConnectionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PrivateConnectionStateEnum::CREATING => "CREATING",
            PrivateConnectionStateEnum::CREATED => "CREATED",
            PrivateConnectionStateEnum::FAILED => "FAILED",
            PrivateConnectionStateEnum::DELETING => "DELETING",
            PrivateConnectionStateEnum::FAILEDTODELETE => "FAILED_TO_DELETE",
            PrivateConnectionStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for PrivateConnectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PrivateConnectionStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(PrivateConnectionStateEnum::CREATING),
           "CREATED" => Ok(PrivateConnectionStateEnum::CREATED),
           "FAILED" => Ok(PrivateConnectionStateEnum::FAILED),
           "DELETING" => Ok(PrivateConnectionStateEnum::DELETING),
           "FAILED_TO_DELETE" => Ok(PrivateConnectionStateEnum::FAILEDTODELETE),
           "DELETED" => Ok(PrivateConnectionStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PrivateConnectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SourceNumericFilterNumericFilterOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Enum to set the option defining the datatypes numeric filter has to be applied to
pub enum SourceNumericFilterNumericFilterOptionEnum {
    

    /// Numeric filter option unspecified
    ///
    /// "NUMERIC_FILTER_OPTION_UNSPECIFIED"
    #[serde(rename="NUMERIC_FILTER_OPTION_UNSPECIFIED")]
    NUMERICFILTEROPTIONUNSPECIFIED,
    

    /// Numeric filter option that matches all numeric columns.
    ///
    /// "NUMERIC_FILTER_OPTION_ALL"
    #[serde(rename="NUMERIC_FILTER_OPTION_ALL")]
    NUMERICFILTEROPTIONALL,
    

    /// Numeric filter option that matches columns having numeric datatypes with specified precision and scale within the limited range of filter.
    ///
    /// "NUMERIC_FILTER_OPTION_LIMIT"
    #[serde(rename="NUMERIC_FILTER_OPTION_LIMIT")]
    NUMERICFILTEROPTIONLIMIT,
    

    /// Numeric filter option that matches only the numeric columns with no precision and scale specified.
    ///
    /// "NUMERIC_FILTER_OPTION_LIMITLESS"
    #[serde(rename="NUMERIC_FILTER_OPTION_LIMITLESS")]
    NUMERICFILTEROPTIONLIMITLESS,
}

impl AsRef<str> for SourceNumericFilterNumericFilterOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONUNSPECIFIED => "NUMERIC_FILTER_OPTION_UNSPECIFIED",
            SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONALL => "NUMERIC_FILTER_OPTION_ALL",
            SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONLIMIT => "NUMERIC_FILTER_OPTION_LIMIT",
            SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONLIMITLESS => "NUMERIC_FILTER_OPTION_LIMITLESS",
        }
    }
}

impl std::convert::TryFrom< &str> for SourceNumericFilterNumericFilterOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NUMERIC_FILTER_OPTION_UNSPECIFIED" => Ok(SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONUNSPECIFIED),
           "NUMERIC_FILTER_OPTION_ALL" => Ok(SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONALL),
           "NUMERIC_FILTER_OPTION_LIMIT" => Ok(SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONLIMIT),
           "NUMERIC_FILTER_OPTION_LIMITLESS" => Ok(SourceNumericFilterNumericFilterOptionEnum::NUMERICFILTEROPTIONLIMITLESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SourceNumericFilterNumericFilterOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The ssl config type according to 'client_key', 'client_certificate' and 'ca_certificate'.
pub enum SslConfigTypeEnum {
    

    /// Unspecified.
    ///
    /// "SSL_TYPE_UNSPECIFIED"
    #[serde(rename="SSL_TYPE_UNSPECIFIED")]
    SSLTYPEUNSPECIFIED,
    

    /// Only 'ca_certificate' specified.
    ///
    /// "SERVER_ONLY"
    #[serde(rename="SERVER_ONLY")]
    SERVERONLY,
    

    /// Both server ('ca_certificate'), and client ('client_key', 'client_certificate') specified.
    ///
    /// "SERVER_CLIENT"
    #[serde(rename="SERVER_CLIENT")]
    SERVERCLIENT,
}

impl AsRef<str> for SslConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslConfigTypeEnum::SSLTYPEUNSPECIFIED => "SSL_TYPE_UNSPECIFIED",
            SslConfigTypeEnum::SERVERONLY => "SERVER_ONLY",
            SslConfigTypeEnum::SERVERCLIENT => "SERVER_CLIENT",
        }
    }
}

impl std::convert::TryFrom< &str> for SslConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSL_TYPE_UNSPECIFIED" => Ok(SslConfigTypeEnum::SSLTYPEUNSPECIFIED),
           "SERVER_ONLY" => Ok(SslConfigTypeEnum::SERVERONLY),
           "SERVER_CLIENT" => Ok(SslConfigTypeEnum::SERVERCLIENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SynonymEntitySourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the entity for which the synonym is being created (usually a table or a sequence).
pub enum SynonymEntitySourceTypeEnum {
    

    /// Unspecified database entity type.
    ///
    /// "DATABASE_ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_TYPE_UNSPECIFIED")]
    DATABASEENTITYTYPEUNSPECIFIED,
    

    /// Schema.
    ///
    /// "DATABASE_ENTITY_TYPE_SCHEMA"
    #[serde(rename="DATABASE_ENTITY_TYPE_SCHEMA")]
    DATABASEENTITYTYPESCHEMA,
    

    /// Table.
    ///
    /// "DATABASE_ENTITY_TYPE_TABLE"
    #[serde(rename="DATABASE_ENTITY_TYPE_TABLE")]
    DATABASEENTITYTYPETABLE,
    

    /// Column.
    ///
    /// "DATABASE_ENTITY_TYPE_COLUMN"
    #[serde(rename="DATABASE_ENTITY_TYPE_COLUMN")]
    DATABASEENTITYTYPECOLUMN,
    

    /// Constraint.
    ///
    /// "DATABASE_ENTITY_TYPE_CONSTRAINT"
    #[serde(rename="DATABASE_ENTITY_TYPE_CONSTRAINT")]
    DATABASEENTITYTYPECONSTRAINT,
    

    /// Index.
    ///
    /// "DATABASE_ENTITY_TYPE_INDEX"
    #[serde(rename="DATABASE_ENTITY_TYPE_INDEX")]
    DATABASEENTITYTYPEINDEX,
    

    /// Trigger.
    ///
    /// "DATABASE_ENTITY_TYPE_TRIGGER"
    #[serde(rename="DATABASE_ENTITY_TYPE_TRIGGER")]
    DATABASEENTITYTYPETRIGGER,
    

    /// View.
    ///
    /// "DATABASE_ENTITY_TYPE_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_VIEW")]
    DATABASEENTITYTYPEVIEW,
    

    /// Sequence.
    ///
    /// "DATABASE_ENTITY_TYPE_SEQUENCE"
    #[serde(rename="DATABASE_ENTITY_TYPE_SEQUENCE")]
    DATABASEENTITYTYPESEQUENCE,
    

    /// Stored Procedure.
    ///
    /// "DATABASE_ENTITY_TYPE_STORED_PROCEDURE"
    #[serde(rename="DATABASE_ENTITY_TYPE_STORED_PROCEDURE")]
    DATABASEENTITYTYPESTOREDPROCEDURE,
    

    /// Function.
    ///
    /// "DATABASE_ENTITY_TYPE_FUNCTION"
    #[serde(rename="DATABASE_ENTITY_TYPE_FUNCTION")]
    DATABASEENTITYTYPEFUNCTION,
    

    /// Synonym.
    ///
    /// "DATABASE_ENTITY_TYPE_SYNONYM"
    #[serde(rename="DATABASE_ENTITY_TYPE_SYNONYM")]
    DATABASEENTITYTYPESYNONYM,
    

    /// Package.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE_PACKAGE")]
    DATABASEENTITYTYPEDATABASEPACKAGE,
    

    /// UDT.
    ///
    /// "DATABASE_ENTITY_TYPE_UDT"
    #[serde(rename="DATABASE_ENTITY_TYPE_UDT")]
    DATABASEENTITYTYPEUDT,
    

    /// Materialized View.
    ///
    /// "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW"
    #[serde(rename="DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW")]
    DATABASEENTITYTYPEMATERIALIZEDVIEW,
    

    /// Database.
    ///
    /// "DATABASE_ENTITY_TYPE_DATABASE"
    #[serde(rename="DATABASE_ENTITY_TYPE_DATABASE")]
    DATABASEENTITYTYPEDATABASE,
}

impl AsRef<str> for SynonymEntitySourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEUNSPECIFIED => "DATABASE_ENTITY_TYPE_UNSPECIFIED",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESCHEMA => "DATABASE_ENTITY_TYPE_SCHEMA",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPETABLE => "DATABASE_ENTITY_TYPE_TABLE",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPECOLUMN => "DATABASE_ENTITY_TYPE_COLUMN",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPECONSTRAINT => "DATABASE_ENTITY_TYPE_CONSTRAINT",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEINDEX => "DATABASE_ENTITY_TYPE_INDEX",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPETRIGGER => "DATABASE_ENTITY_TYPE_TRIGGER",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEVIEW => "DATABASE_ENTITY_TYPE_VIEW",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESEQUENCE => "DATABASE_ENTITY_TYPE_SEQUENCE",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE => "DATABASE_ENTITY_TYPE_STORED_PROCEDURE",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEFUNCTION => "DATABASE_ENTITY_TYPE_FUNCTION",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESYNONYM => "DATABASE_ENTITY_TYPE_SYNONYM",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE => "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEUDT => "DATABASE_ENTITY_TYPE_UDT",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW => "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW",
            SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEDATABASE => "DATABASE_ENTITY_TYPE_DATABASE",
        }
    }
}

impl std::convert::TryFrom< &str> for SynonymEntitySourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_TYPE_UNSPECIFIED" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEUNSPECIFIED),
           "DATABASE_ENTITY_TYPE_SCHEMA" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESCHEMA),
           "DATABASE_ENTITY_TYPE_TABLE" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPETABLE),
           "DATABASE_ENTITY_TYPE_COLUMN" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPECOLUMN),
           "DATABASE_ENTITY_TYPE_CONSTRAINT" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPECONSTRAINT),
           "DATABASE_ENTITY_TYPE_INDEX" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEINDEX),
           "DATABASE_ENTITY_TYPE_TRIGGER" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPETRIGGER),
           "DATABASE_ENTITY_TYPE_VIEW" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEVIEW),
           "DATABASE_ENTITY_TYPE_SEQUENCE" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESEQUENCE),
           "DATABASE_ENTITY_TYPE_STORED_PROCEDURE" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESTOREDPROCEDURE),
           "DATABASE_ENTITY_TYPE_FUNCTION" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEFUNCTION),
           "DATABASE_ENTITY_TYPE_SYNONYM" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPESYNONYM),
           "DATABASE_ENTITY_TYPE_DATABASE_PACKAGE" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEDATABASEPACKAGE),
           "DATABASE_ENTITY_TYPE_UDT" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEUDT),
           "DATABASE_ENTITY_TYPE_MATERIALIZED_VIEW" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEMATERIALIZEDVIEW),
           "DATABASE_ENTITY_TYPE_DATABASE" => Ok(SynonymEntitySourceTypeEnum::DATABASEENTITYTYPEDATABASE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SynonymEntitySourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValueListFilterValuePresentListEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether the filter matches rows with values that are present in the list or those with values not present in it.
pub enum ValueListFilterValuePresentListEnum {
    

    /// Value present in list unspecified
    ///
    /// "VALUE_PRESENT_IN_LIST_UNSPECIFIED"
    #[serde(rename="VALUE_PRESENT_IN_LIST_UNSPECIFIED")]
    VALUEPRESENTINLISTUNSPECIFIED,
    

    /// If the source value is in the supplied list at value_list
    ///
    /// "VALUE_PRESENT_IN_LIST_IF_VALUE_LIST"
    #[serde(rename="VALUE_PRESENT_IN_LIST_IF_VALUE_LIST")]
    VALUEPRESENTINLISTIFVALUELIST,
    

    /// If the source value is not in the supplied list at value_list
    ///
    /// "VALUE_PRESENT_IN_LIST_IF_VALUE_NOT_LIST"
    #[serde(rename="VALUE_PRESENT_IN_LIST_IF_VALUE_NOT_LIST")]
    VALUEPRESENTINLISTIFVALUENOTLIST,
}

impl AsRef<str> for ValueListFilterValuePresentListEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValueListFilterValuePresentListEnum::VALUEPRESENTINLISTUNSPECIFIED => "VALUE_PRESENT_IN_LIST_UNSPECIFIED",
            ValueListFilterValuePresentListEnum::VALUEPRESENTINLISTIFVALUELIST => "VALUE_PRESENT_IN_LIST_IF_VALUE_LIST",
            ValueListFilterValuePresentListEnum::VALUEPRESENTINLISTIFVALUENOTLIST => "VALUE_PRESENT_IN_LIST_IF_VALUE_NOT_LIST",
        }
    }
}

impl std::convert::TryFrom< &str> for ValueListFilterValuePresentListEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VALUE_PRESENT_IN_LIST_UNSPECIFIED" => Ok(ValueListFilterValuePresentListEnum::VALUEPRESENTINLISTUNSPECIFIED),
           "VALUE_PRESENT_IN_LIST_IF_VALUE_LIST" => Ok(ValueListFilterValuePresentListEnum::VALUEPRESENTINLISTIFVALUELIST),
           "VALUE_PRESENT_IN_LIST_IF_VALUE_NOT_LIST" => Ok(ValueListFilterValuePresentListEnum::VALUEPRESENTINLISTIFVALUENOTLIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValueListFilterValuePresentListEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectTreeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The tree to fetch.
pub enum ProjectTreeEnum {
    

    /// Unspecified tree type.
    ///
    /// "DB_TREE_TYPE_UNSPECIFIED"
    #[serde(rename="DB_TREE_TYPE_UNSPECIFIED")]
    DBTREETYPEUNSPECIFIED,
    

    /// The source database tree.
    ///
    /// "SOURCE_TREE"
    #[serde(rename="SOURCE_TREE")]
    SOURCETREE,
    

    /// The draft database tree.
    ///
    /// "DRAFT_TREE"
    #[serde(rename="DRAFT_TREE")]
    DRAFTTREE,
    

    /// The destination database tree.
    ///
    /// "DESTINATION_TREE"
    #[serde(rename="DESTINATION_TREE")]
    DESTINATIONTREE,
}

impl AsRef<str> for ProjectTreeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectTreeEnum::DBTREETYPEUNSPECIFIED => "DB_TREE_TYPE_UNSPECIFIED",
            ProjectTreeEnum::SOURCETREE => "SOURCE_TREE",
            ProjectTreeEnum::DRAFTTREE => "DRAFT_TREE",
            ProjectTreeEnum::DESTINATIONTREE => "DESTINATION_TREE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectTreeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DB_TREE_TYPE_UNSPECIFIED" => Ok(ProjectTreeEnum::DBTREETYPEUNSPECIFIED),
           "SOURCE_TREE" => Ok(ProjectTreeEnum::SOURCETREE),
           "DRAFT_TREE" => Ok(ProjectTreeEnum::DRAFTTREE),
           "DESTINATION_TREE" => Ok(ProjectTreeEnum::DESTINATIONTREE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectTreeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Results view based on AIP-157
pub enum ProjectViewEnum {
    

    /// Unspecified view. Defaults to basic view.
    ///
    /// "DATABASE_ENTITY_VIEW_UNSPECIFIED"
    #[serde(rename="DATABASE_ENTITY_VIEW_UNSPECIFIED")]
    DATABASEENTITYVIEWUNSPECIFIED,
    

    /// Default view. Does not return DDLs or Issues.
    ///
    /// "DATABASE_ENTITY_VIEW_BASIC"
    #[serde(rename="DATABASE_ENTITY_VIEW_BASIC")]
    DATABASEENTITYVIEWBASIC,
    

    /// Return full entity details including mappings, ddl and issues.
    ///
    /// "DATABASE_ENTITY_VIEW_FULL"
    #[serde(rename="DATABASE_ENTITY_VIEW_FULL")]
    DATABASEENTITYVIEWFULL,
    

    /// Top-most (Database, Schema) nodes which are returned contains summary details for their decendents such as the number of entities per type and issues rollups. When this view is used, only a single page of result is returned and the page_size property of the request is ignored. The returned page will only include the top-most node types.
    ///
    /// "DATABASE_ENTITY_VIEW_ROOT_SUMMARY"
    #[serde(rename="DATABASE_ENTITY_VIEW_ROOT_SUMMARY")]
    DATABASEENTITYVIEWROOTSUMMARY,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::DATABASEENTITYVIEWUNSPECIFIED => "DATABASE_ENTITY_VIEW_UNSPECIFIED",
            ProjectViewEnum::DATABASEENTITYVIEWBASIC => "DATABASE_ENTITY_VIEW_BASIC",
            ProjectViewEnum::DATABASEENTITYVIEWFULL => "DATABASE_ENTITY_VIEW_FULL",
            ProjectViewEnum::DATABASEENTITYVIEWROOTSUMMARY => "DATABASE_ENTITY_VIEW_ROOT_SUMMARY",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_ENTITY_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::DATABASEENTITYVIEWUNSPECIFIED),
           "DATABASE_ENTITY_VIEW_BASIC" => Ok(ProjectViewEnum::DATABASEENTITYVIEWBASIC),
           "DATABASE_ENTITY_VIEW_FULL" => Ok(ProjectViewEnum::DATABASEENTITYVIEWFULL),
           "DATABASE_ENTITY_VIEW_ROOT_SUMMARY" => Ok(ProjectViewEnum::DATABASEENTITYVIEWROOTSUMMARY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


