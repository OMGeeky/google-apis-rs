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
    

    /// Warning when one or more regions are not reachable.  The returned result
set may be incomplete.
    ///
    /// "REGION_UNREACHABLE"
    #[serde(rename="REGION_UNREACHABLE")]
    REGIONUNREACHABLE,
}

impl AsRef<str> for ApiWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiWarningCodeEnum::SQLAPIWARNINGCODEUNSPECIFIED => "SQL_API_WARNING_CODE_UNSPECIFIED",
            ApiWarningCodeEnum::REGIONUNREACHABLE => "REGION_UNREACHABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQL_API_WARNING_CODE_UNSPECIFIED" => Ok(ApiWarningCodeEnum::SQLAPIWARNINGCODEUNSPECIFIED),
           "REGION_UNREACHABLE" => Ok(ApiWarningCodeEnum::REGIONUNREACHABLE),
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
    

    /// The backup is overdue across a given backup window. Indicates a
problem. Example: Long-running operation in progress during
the whole window.
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
    

    /// The backup was skipped (without problems) for a given backup
window. Example: Instance was idle.
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
/// The type of this run; can be either "AUTOMATED" or "ON_DEMAND".
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


// region DatabaseInstanceBackendTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// <code>FIRST_GEN</code>: First Generation instance. MySQL only. <br
/><code>SECOND_GEN</code>: Second Generation instance or PostgreSQL
instance. <br /><code>EXTERNAL</code>: A database server that is not
managed by Google. <br>This property is read-only; use the
<code>tier</code> property in the <code>settings</code> object to determine
the database type and Second or First Generation.
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
/// The database engine type and version. The <code>databaseVersion</code>
field can not be changed after instance creation.  MySQL Second Generation
instances: <code>MYSQL_5_7</code> (default) or <code>MYSQL_5_6</code>.
PostgreSQL instances: <code>POSTGRES_9_6</code> (default) or
<code>POSTGRES_11 Beta</code> MySQL First Generation
instances: <code>MYSQL_5_6</code> (default) or <code>MYSQL_5_5</code>
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
    

    /// The database version is PostgreSQL 9.6.
    ///
    /// "POSTGRES_9_6"
    #[serde(rename="POSTGRES_9_6")]
    POSTGRES96,
    

    /// The database version is PostgreSQL 11.
    ///
    /// "POSTGRES_11"
    #[serde(rename="POSTGRES_11")]
    POSTGRES11,
    

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
    

    /// The database version is PostgreSQL 10.
    ///
    /// "POSTGRES_10"
    #[serde(rename="POSTGRES_10")]
    POSTGRES10,
    

    /// The database version is PostgreSQL 12.
    ///
    /// "POSTGRES_12"
    #[serde(rename="POSTGRES_12")]
    POSTGRES12,
}

impl AsRef<str> for DatabaseInstanceDatabaseVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseInstanceDatabaseVersionEnum::SQLDATABASEVERSIONUNSPECIFIED => "SQL_DATABASE_VERSION_UNSPECIFIED",
            DatabaseInstanceDatabaseVersionEnum::MYSQL51 => "MYSQL_5_1",
            DatabaseInstanceDatabaseVersionEnum::MYSQL55 => "MYSQL_5_5",
            DatabaseInstanceDatabaseVersionEnum::MYSQL56 => "MYSQL_5_6",
            DatabaseInstanceDatabaseVersionEnum::MYSQL57 => "MYSQL_5_7",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES96 => "POSTGRES_9_6",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES11 => "POSTGRES_11",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017STANDARD => "SQLSERVER_2017_STANDARD",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017ENTERPRISE => "SQLSERVER_2017_ENTERPRISE",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017EXPRESS => "SQLSERVER_2017_EXPRESS",
            DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017WEB => "SQLSERVER_2017_WEB",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES10 => "POSTGRES_10",
            DatabaseInstanceDatabaseVersionEnum::POSTGRES12 => "POSTGRES_12",
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
           "POSTGRES_9_6" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES96),
           "POSTGRES_11" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES11),
           "SQLSERVER_2017_STANDARD" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017STANDARD),
           "SQLSERVER_2017_ENTERPRISE" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017ENTERPRISE),
           "SQLSERVER_2017_EXPRESS" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017EXPRESS),
           "SQLSERVER_2017_WEB" => Ok(DatabaseInstanceDatabaseVersionEnum::SQLSERVER2017WEB),
           "POSTGRES_10" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES10),
           "POSTGRES_12" => Ok(DatabaseInstanceDatabaseVersionEnum::POSTGRES12),
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
/// The instance type. This can be one of the following.
<br><code>CLOUD_SQL_INSTANCE</code>: A Cloud SQL instance that is not
replicating from a master. <br><code>ON_PREMISES_INSTANCE</code>: An
instance running on the
customer's premises. <br><code>READ_REPLICA_INSTANCE</code>: A Cloud SQL
instance configured as a read-replica.
pub enum DatabaseInstanceInstanceTypeEnum {
    

    /// This is an unknown Cloud SQL instance type.
    ///
    /// "SQL_INSTANCE_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_INSTANCE_TYPE_UNSPECIFIED")]
    SQLINSTANCETYPEUNSPECIFIED,
    

    /// A regular Cloud SQL instance.
    ///
    /// "CLOUD_SQL_INSTANCE"
    #[serde(rename="CLOUD_SQL_INSTANCE")]
    CLOUDSQLINSTANCE,
    

    /// An instance running on the customer's premises that is not managed by
Cloud SQL.
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


// region DatabaseInstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current serving state of the Cloud SQL instance. This can be one of the
following. <br><code>RUNNABLE</code>: The instance is running, or is ready
to run when accessed. <br><code>SUSPENDED</code>: The instance is not
available, for example due to problems with billing.
<br><code>PENDING_CREATE</code>: The instance is being created.
<br><code>MAINTENANCE</code>: The instance is down for maintenance.
<br><code>FAILED</code>: The instance creation failed.
<br><code>UNKNOWN_STATE</code>: The state of the instance is unknown.
pub enum DatabaseInstanceStateEnum {
    

    /// The state of the instance is unknown.
    ///
    /// "SQL_INSTANCE_STATE_UNSPECIFIED"
    #[serde(rename="SQL_INSTANCE_STATE_UNSPECIFIED")]
    SQLINSTANCESTATEUNSPECIFIED,
    

    /// The instance is running.
    ///
    /// "RUNNABLE"
    #[serde(rename="RUNNABLE")]
    RUNNABLE,
    

    /// The instance is currently offline, but it may run again in the future.
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
    

    /// The instance failed to be created.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
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
    

    /// The instance is suspended due to billing issues (e.g., GCP account issue)
    ///
    /// "BILLING_ISSUE"
    #[serde(rename="BILLING_ISSUE")]
    BILLINGISSUE,
    

    /// The instance is suspended due to illegal content (e.g., child pornography,
copyrighted material, etc.).
    ///
    /// "LEGAL_ISSUE"
    #[serde(rename="LEGAL_ISSUE")]
    LEGALISSUE,
    

    /// The instance is causing operational issues (e.g., causing the database
to crash).
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
/// The file type for the specified uri. <br><code>SQL</code>: The file
contains SQL statements. <br><code>CSV</code>: The file contains CSV data.
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
/// The database version this flag applies to. Can be <code>MYSQL_5_5</code>,
<code>MYSQL_5_6</code>, or <code>MYSQL_5_7</code>. <code>MYSQL_5_7</code>
is applicable only to Second Generation instances.
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
    

    /// The database version is PostgreSQL 9.6.
    ///
    /// "POSTGRES_9_6"
    #[serde(rename="POSTGRES_9_6")]
    POSTGRES96,
    

    /// The database version is PostgreSQL 11.
    ///
    /// "POSTGRES_11"
    #[serde(rename="POSTGRES_11")]
    POSTGRES11,
    

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
    

    /// The database version is PostgreSQL 10.
    ///
    /// "POSTGRES_10"
    #[serde(rename="POSTGRES_10")]
    POSTGRES10,
    

    /// The database version is PostgreSQL 12.
    ///
    /// "POSTGRES_12"
    #[serde(rename="POSTGRES_12")]
    POSTGRES12,
}

impl AsRef<str> for FlagAppliesToEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FlagAppliesToEnum::SQLDATABASEVERSIONUNSPECIFIED => "SQL_DATABASE_VERSION_UNSPECIFIED",
            FlagAppliesToEnum::MYSQL51 => "MYSQL_5_1",
            FlagAppliesToEnum::MYSQL55 => "MYSQL_5_5",
            FlagAppliesToEnum::MYSQL56 => "MYSQL_5_6",
            FlagAppliesToEnum::MYSQL57 => "MYSQL_5_7",
            FlagAppliesToEnum::POSTGRES96 => "POSTGRES_9_6",
            FlagAppliesToEnum::POSTGRES11 => "POSTGRES_11",
            FlagAppliesToEnum::SQLSERVER2017STANDARD => "SQLSERVER_2017_STANDARD",
            FlagAppliesToEnum::SQLSERVER2017ENTERPRISE => "SQLSERVER_2017_ENTERPRISE",
            FlagAppliesToEnum::SQLSERVER2017EXPRESS => "SQLSERVER_2017_EXPRESS",
            FlagAppliesToEnum::SQLSERVER2017WEB => "SQLSERVER_2017_WEB",
            FlagAppliesToEnum::POSTGRES10 => "POSTGRES_10",
            FlagAppliesToEnum::POSTGRES12 => "POSTGRES_12",
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
           "POSTGRES_9_6" => Ok(FlagAppliesToEnum::POSTGRES96),
           "POSTGRES_11" => Ok(FlagAppliesToEnum::POSTGRES11),
           "SQLSERVER_2017_STANDARD" => Ok(FlagAppliesToEnum::SQLSERVER2017STANDARD),
           "SQLSERVER_2017_ENTERPRISE" => Ok(FlagAppliesToEnum::SQLSERVER2017ENTERPRISE),
           "SQLSERVER_2017_EXPRESS" => Ok(FlagAppliesToEnum::SQLSERVER2017EXPRESS),
           "SQLSERVER_2017_WEB" => Ok(FlagAppliesToEnum::SQLSERVER2017WEB),
           "POSTGRES_10" => Ok(FlagAppliesToEnum::POSTGRES10),
           "POSTGRES_12" => Ok(FlagAppliesToEnum::POSTGRES12),
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
/// The type of the flag. Flags are typed to being <code>BOOLEAN</code>,
<code>STRING</code>, <code>INTEGER</code> or <code>NONE</code>.
<code>NONE</code> is used for flags which do not take a value, such as
<code>skip_grant_tables</code>.
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
    

    /// Type introduced specically for MySQL TimeZone offset. Accept a string value
with the format [-12:59, 13:00].
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
/// The file type for the specified uri. <br><code>SQL</code>: The file
contains SQL statements. <br><code>CSV</code>: The file contains CSV data.
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


// region IpMappingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this IP address. A <code>PRIMARY</code> address is a public
address that can accept incoming connections. A <code>PRIVATE</code>
address is a private address that can accept incoming connections. An
<code>OUTGOING</code> address is the source address of connections
originating from the instance, if supported.
pub enum IpMappingTypeEnum {
    

    /// This is an unknown IP address type.
    ///
    /// "SQL_IP_ADDRESS_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_IP_ADDRESS_TYPE_UNSPECIFIED")]
    SQLIPADDRESSTYPEUNSPECIFIED,
    

    /// IP address the customer is supposed to connect to. Usually this is the
load balancer's IP address
    ///
    /// "PRIMARY"
    #[serde(rename="PRIMARY")]
    PRIMARY,
    

    /// Source IP address of the connection a read replica establishes to its
external master. This IP address can be whitelisted by the customer
in case it has a firewall that filters incoming connection to its
on premises master.
    ///
    /// "OUTGOING"
    #[serde(rename="OUTGOING")]
    OUTGOING,
    

    /// Private IP used when using private IPs and network peering.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
    

    /// V1 IP of a migrated instance. We want the user to
decommission this IP as soon as the migration is complete.
Note: V1 instances with V1 ip addresses will be counted as PRIMARY.
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
/// Maintenance timing setting: <code>canary</code> (Earlier) or
<code>stable</code> (Later). <br /><a
href="/sql/docs/db_path/instance-settings#maintenance-timing-2ndgen">
Learn more</a>.
pub enum MaintenanceWindowUpdateTrackEnum {
    

    /// This is an unknown maintenance timing preference.
    ///
    /// "SQL_UPDATE_TRACK_UNSPECIFIED"
    #[serde(rename="SQL_UPDATE_TRACK_UNSPECIFIED")]
    SQLUPDATETRACKUNSPECIFIED,
    

    /// For instance update that requires a restart, this update track indicates
your instance prefer to restart for new version early in maintenance
window.
    ///
    /// "canary"
    #[serde(rename="canary")]
    Canary,
    

    /// For instance update that requires a restart, this update track indicates
your instance prefer to let Cloud SQL choose the timing of restart (within
its Maintenance window, if applicable).
    ///
    /// "stable"
    #[serde(rename="stable")]
    Stable,
}

impl AsRef<str> for MaintenanceWindowUpdateTrackEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MaintenanceWindowUpdateTrackEnum::SQLUPDATETRACKUNSPECIFIED => "SQL_UPDATE_TRACK_UNSPECIFIED",
            MaintenanceWindowUpdateTrackEnum::Canary => "canary",
            MaintenanceWindowUpdateTrackEnum::Stable => "stable",
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
/// The type of the operation. Valid values are <code>CREATE</code>,
<code>DELETE</code>, <code>UPDATE</code>, <code>RESTART</code>,
<code>IMPORT</code>, <code>EXPORT</code>, <code>BACKUP_VOLUME</code>,
<code>RESTORE_VOLUME</code>, <code>CREATE_USER</code>,
<code>DELETE_USER</code>, <code>CREATE_DATABASE</code>,
<code>DELETE_DATABASE</code> .
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
    

    /// Exports data from a Cloud SQL instance to a Cloud Storage
bucket.
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
    

    /// Performs failover of an HA-enabled Cloud SQL
failover replica.
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
    

    /// Demotes the stand-alone instance to be a Cloud SQL
read replica for an external database server.
    ///
    /// "DEMOTE_MASTER"
    #[serde(rename="DEMOTE_MASTER")]
    DEMOTEMASTER,
    

    /// Indicates that the instance is currently in maintenance. Maintenance
typically causes the instance to be unavailable for 1-3 minutes.
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
    

    /// Starts external sync of a Cloud SQL EM replica to an external master.
    ///
    /// "START_EXTERNAL_SYNC"
    #[serde(rename="START_EXTERNAL_SYNC")]
    STARTEXTERNALSYNC,
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
/// The status of an operation. Valid values are <code>PENDING</code>,
<code>RUNNING</code>, <code>DONE</code>,
<code>SQL_OPERATION_STATUS_UNSPECIFIED</code>.
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


// region RescheduleRescheduleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the reschedule.
pub enum RescheduleRescheduleTypeEnum {
    
    /// "RESCHEDULE_TYPE_UNSPECIFIED"
    #[serde(rename="RESCHEDULE_TYPE_UNSPECIFIED")]
    RESCHEDULETYPEUNSPECIFIED,
    

    /// If the user wants to schedule the maintenance to happen now.
    ///
    /// "IMMEDIATE"
    #[serde(rename="IMMEDIATE")]
    IMMEDIATE,
    

    /// If the user wants to use the existing maintenance policy to find the
next available window.
    ///
    /// "NEXT_AVAILABLE_WINDOW"
    #[serde(rename="NEXT_AVAILABLE_WINDOW")]
    NEXTAVAILABLEWINDOW,
    

    /// If the user wants to reschedule the maintenance to a specific time.
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
/// The activation policy specifies when the instance is activated; it is
applicable only when the instance state is <code>RUNNABLE</code>. Valid
values: <br><code>ALWAYS</code>: The instance is on, and remains so even in
the absence of connection requests. <br><code>NEVER</code>: The instance is
off; it is not activated, even if a connection request arrives.
<br><code>ON_DEMAND</code>: First Generation instances only. The instance
responds to incoming requests, and turns itself off when not in use.
Instances with <code>PER_USE</code> pricing turn off after 15 minutes of
inactivity. Instances with <code>PER_PACKAGE</code> pricing turn off after
12 hours of inactivity.
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
    

    /// The instance should never spin up.
    ///
    /// "NEVER"
    #[serde(rename="NEVER")]
    NEVER,
    

    /// The instance spins up upon receiving requests.
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
/// Availability type (PostgreSQL and MySQL instances only). Potential values:
<br><code>ZONAL</code>: The instance serves data from only one zone.
Outages in that zone affect data accessibility. <br><code>REGIONAL</code>:
The instance can serve data from more than one zone in a region (it is
highly available). <br>For more information, see <a
href="https://cloud.google.com/sql/docs/postgres/high-availability">Overview
of the High Availability Configuration</a>.
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


// region SettingDataDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of data disk: <code>PD_SSD</code> (default) or
<code>PD_HDD</code>. Not used for First Generation instances.
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
    

    /// This field is deprecated and will be removed from a future version of the
API.
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


// region SettingPricingPlanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The pricing plan for this instance. This can be either <code>PER_USE</code>
or <code>PACKAGE</code>. Only <code>PER_USE</code> is supported for Second
Generation instances.
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
/// The type of replication this instance uses. This can be either
<code>ASYNCHRONOUS</code> or <code>SYNCHRONOUS</code>. This property is
only applicable to First Generation instances.
pub enum SettingReplicationTypeEnum {
    

    /// This is an unknown replication type for a Cloud SQL instance.
    ///
    /// "SQL_REPLICATION_TYPE_UNSPECIFIED"
    #[serde(rename="SQL_REPLICATION_TYPE_UNSPECIFIED")]
    SQLREPLICATIONTYPEUNSPECIFIED,
    

    /// The synchronous replication mode for First Generation instances. It is the
default value.
    ///
    /// "SYNCHRONOUS"
    #[serde(rename="SYNCHRONOUS")]
    SYNCHRONOUS,
    

    /// The asynchronous replication mode for First Generation instances. It
provides a slight performance gain, but if an outage occurs while this
option is set to asynchronous, you can lose up to a few seconds of updates
to your data.
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


// region ProjectSyncModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// External sync mode
pub enum ProjectSyncModeEnum {
    

    /// no description found
    ///
    /// "EXTERNAL_SYNC_MODE_UNSPECIFIED"
    #[serde(rename="EXTERNAL_SYNC_MODE_UNSPECIFIED")]
    EXTERNALSYNCMODEUNSPECIFIED,
    

    /// no description found
    ///
    /// "ONLINE"
    #[serde(rename="ONLINE")]
    ONLINE,
    

    /// no description found
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
}

impl AsRef<str> for ProjectSyncModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectSyncModeEnum::EXTERNALSYNCMODEUNSPECIFIED => "EXTERNAL_SYNC_MODE_UNSPECIFIED",
            ProjectSyncModeEnum::ONLINE => "ONLINE",
            ProjectSyncModeEnum::OFFLINE => "OFFLINE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectSyncModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXTERNAL_SYNC_MODE_UNSPECIFIED" => Ok(ProjectSyncModeEnum::EXTERNALSYNCMODEUNSPECIFIED),
           "ONLINE" => Ok(ProjectSyncModeEnum::ONLINE),
           "OFFLINE" => Ok(ProjectSyncModeEnum::OFFLINE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectSyncModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


