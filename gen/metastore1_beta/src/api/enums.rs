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


// region BackendMetastoreMetastoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the backend metastore.
pub enum BackendMetastoreMetastoreTypeEnum {
    

    /// The metastore type is not set.
    ///
    /// "METASTORE_TYPE_UNSPECIFIED"
    #[serde(rename="METASTORE_TYPE_UNSPECIFIED")]
    METASTORETYPEUNSPECIFIED,
    

    /// The backend metastore is Dataplex.
    ///
    /// "DATAPLEX"
    #[serde(rename="DATAPLEX")]
    DATAPLEX,
    

    /// The backend metastore is BigQuery.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
    

    /// The backend metastore is Dataproc Metastore.
    ///
    /// "DATAPROC_METASTORE"
    #[serde(rename="DATAPROC_METASTORE")]
    DATAPROCMETASTORE,
}

impl AsRef<str> for BackendMetastoreMetastoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackendMetastoreMetastoreTypeEnum::METASTORETYPEUNSPECIFIED => "METASTORE_TYPE_UNSPECIFIED",
            BackendMetastoreMetastoreTypeEnum::DATAPLEX => "DATAPLEX",
            BackendMetastoreMetastoreTypeEnum::BIGQUERY => "BIGQUERY",
            BackendMetastoreMetastoreTypeEnum::DATAPROCMETASTORE => "DATAPROC_METASTORE",
        }
    }
}

impl std::convert::TryFrom< &str> for BackendMetastoreMetastoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METASTORE_TYPE_UNSPECIFIED" => Ok(BackendMetastoreMetastoreTypeEnum::METASTORETYPEUNSPECIFIED),
           "DATAPLEX" => Ok(BackendMetastoreMetastoreTypeEnum::DATAPLEX),
           "BIGQUERY" => Ok(BackendMetastoreMetastoreTypeEnum::BIGQUERY),
           "DATAPROC_METASTORE" => Ok(BackendMetastoreMetastoreTypeEnum::DATAPROCMETASTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackendMetastoreMetastoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the backup.
pub enum BackupStateEnum {
    

    /// The state of the backup is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The backup is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The backup is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The backup is active and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The backup failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The backup is being restored.
    ///
    /// "RESTORING"
    #[serde(rename="RESTORING")]
    RESTORING,
}

impl AsRef<str> for BackupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BackupStateEnum::CREATING => "CREATING",
            BackupStateEnum::DELETING => "DELETING",
            BackupStateEnum::ACTIVE => "ACTIVE",
            BackupStateEnum::FAILED => "FAILED",
            BackupStateEnum::RESTORING => "RESTORING",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BackupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(BackupStateEnum::CREATING),
           "DELETING" => Ok(BackupStateEnum::DELETING),
           "ACTIVE" => Ok(BackupStateEnum::ACTIVE),
           "FAILED" => Ok(BackupStateEnum::FAILED),
           "RESTORING" => Ok(BackupStateEnum::RESTORING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseDumpDatabaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the database.
pub enum DatabaseDumpDatabaseTypeEnum {
    

    /// The type of the source database is unknown.
    ///
    /// "DATABASE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_TYPE_UNSPECIFIED")]
    DATABASETYPEUNSPECIFIED,
    

    /// The type of the source database is MySQL.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
}

impl AsRef<str> for DatabaseDumpDatabaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseDumpDatabaseTypeEnum::DATABASETYPEUNSPECIFIED => "DATABASE_TYPE_UNSPECIFIED",
            DatabaseDumpDatabaseTypeEnum::MYSQL => "MYSQL",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseDumpDatabaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_TYPE_UNSPECIFIED" => Ok(DatabaseDumpDatabaseTypeEnum::DATABASETYPEUNSPECIFIED),
           "MYSQL" => Ok(DatabaseDumpDatabaseTypeEnum::MYSQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseDumpDatabaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseDumpTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of the database dump. If unspecified, defaults to MYSQL.
pub enum DatabaseDumpTypeEnum {
    

    /// The type of the database dump is unknown.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Database dump is a MySQL dump file.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// Database dump contains Avro files.
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
}

impl AsRef<str> for DatabaseDumpTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseDumpTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DatabaseDumpTypeEnum::MYSQL => "MYSQL",
            DatabaseDumpTypeEnum::AVRO => "AVRO",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseDumpTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DatabaseDumpTypeEnum::TYPEUNSPECIFIED),
           "MYSQL" => Ok(DatabaseDumpTypeEnum::MYSQL),
           "AVRO" => Ok(DatabaseDumpTypeEnum::AVRO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseDumpTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportMetadataRequestDatabaseDumpTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of the database dump. If unspecified, defaults to MYSQL.
pub enum ExportMetadataRequestDatabaseDumpTypeEnum {
    

    /// The type of the database dump is unknown.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Database dump is a MySQL dump file.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// Database dump contains Avro files.
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
}

impl AsRef<str> for ExportMetadataRequestDatabaseDumpTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportMetadataRequestDatabaseDumpTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ExportMetadataRequestDatabaseDumpTypeEnum::MYSQL => "MYSQL",
            ExportMetadataRequestDatabaseDumpTypeEnum::AVRO => "AVRO",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportMetadataRequestDatabaseDumpTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ExportMetadataRequestDatabaseDumpTypeEnum::TYPEUNSPECIFIED),
           "MYSQL" => Ok(ExportMetadataRequestDatabaseDumpTypeEnum::MYSQL),
           "AVRO" => Ok(ExportMetadataRequestDatabaseDumpTypeEnum::AVRO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportMetadataRequestDatabaseDumpTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FederationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the federation.
pub enum FederationStateEnum {
    

    /// The state of the metastore federation is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The metastore federation is in the process of being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The metastore federation is running and ready to serve queries.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The metastore federation is being updated. It remains usable but cannot accept additional update requests or be deleted at this time.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The metastore federation is undergoing deletion. It cannot be used.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The metastore federation has encountered an error and cannot be used. The metastore federation should be deleted.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for FederationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FederationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            FederationStateEnum::CREATING => "CREATING",
            FederationStateEnum::ACTIVE => "ACTIVE",
            FederationStateEnum::UPDATING => "UPDATING",
            FederationStateEnum::DELETING => "DELETING",
            FederationStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for FederationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(FederationStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(FederationStateEnum::CREATING),
           "ACTIVE" => Ok(FederationStateEnum::ACTIVE),
           "UPDATING" => Ok(FederationStateEnum::UPDATING),
           "DELETING" => Ok(FederationStateEnum::DELETING),
           "ERROR" => Ok(FederationStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FederationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HiveMetastoreConfigEndpointProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The protocol to use for the metastore service endpoint. If unspecified, defaults to THRIFT.
pub enum HiveMetastoreConfigEndpointProtocolEnum {
    

    /// The protocol is not set.
    ///
    /// "ENDPOINT_PROTOCOL_UNSPECIFIED"
    #[serde(rename="ENDPOINT_PROTOCOL_UNSPECIFIED")]
    ENDPOINTPROTOCOLUNSPECIFIED,
    

    /// Use the legacy Apache Thrift protocol for the metastore service endpoint.
    ///
    /// "THRIFT"
    #[serde(rename="THRIFT")]
    THRIFT,
    

    /// Use the modernized gRPC protocol for the metastore service endpoint.
    ///
    /// "GRPC"
    #[serde(rename="GRPC")]
    GRPC,
}

impl AsRef<str> for HiveMetastoreConfigEndpointProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HiveMetastoreConfigEndpointProtocolEnum::ENDPOINTPROTOCOLUNSPECIFIED => "ENDPOINT_PROTOCOL_UNSPECIFIED",
            HiveMetastoreConfigEndpointProtocolEnum::THRIFT => "THRIFT",
            HiveMetastoreConfigEndpointProtocolEnum::GRPC => "GRPC",
        }
    }
}

impl std::convert::TryFrom< &str> for HiveMetastoreConfigEndpointProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENDPOINT_PROTOCOL_UNSPECIFIED" => Ok(HiveMetastoreConfigEndpointProtocolEnum::ENDPOINTPROTOCOLUNSPECIFIED),
           "THRIFT" => Ok(HiveMetastoreConfigEndpointProtocolEnum::THRIFT),
           "GRPC" => Ok(HiveMetastoreConfigEndpointProtocolEnum::GRPC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HiveMetastoreConfigEndpointProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LatestBackupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the backup.
pub enum LatestBackupStateEnum {
    

    /// The state of the backup is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The backup is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The backup completed.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The backup failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for LatestBackupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LatestBackupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            LatestBackupStateEnum::INPROGRESS => "IN_PROGRESS",
            LatestBackupStateEnum::SUCCEEDED => "SUCCEEDED",
            LatestBackupStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for LatestBackupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(LatestBackupStateEnum::STATEUNSPECIFIED),
           "IN_PROGRESS" => Ok(LatestBackupStateEnum::INPROGRESS),
           "SUCCEEDED" => Ok(LatestBackupStateEnum::SUCCEEDED),
           "FAILED" => Ok(LatestBackupStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LatestBackupStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MaintenanceWindowDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The day of week, when the window starts.
pub enum MaintenanceWindowDayOfWeekEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for MaintenanceWindowDayOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MaintenanceWindowDayOfWeekEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            MaintenanceWindowDayOfWeekEnum::MONDAY => "MONDAY",
            MaintenanceWindowDayOfWeekEnum::TUESDAY => "TUESDAY",
            MaintenanceWindowDayOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            MaintenanceWindowDayOfWeekEnum::THURSDAY => "THURSDAY",
            MaintenanceWindowDayOfWeekEnum::FRIDAY => "FRIDAY",
            MaintenanceWindowDayOfWeekEnum::SATURDAY => "SATURDAY",
            MaintenanceWindowDayOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for MaintenanceWindowDayOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(MaintenanceWindowDayOfWeekEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(MaintenanceWindowDayOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(MaintenanceWindowDayOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(MaintenanceWindowDayOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(MaintenanceWindowDayOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(MaintenanceWindowDayOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(MaintenanceWindowDayOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(MaintenanceWindowDayOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MaintenanceWindowDayOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataExportDatabaseDumpTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the database dump.
pub enum MetadataExportDatabaseDumpTypeEnum {
    

    /// The type of the database dump is unknown.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Database dump is a MySQL dump file.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// Database dump contains Avro files.
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
}

impl AsRef<str> for MetadataExportDatabaseDumpTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataExportDatabaseDumpTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            MetadataExportDatabaseDumpTypeEnum::MYSQL => "MYSQL",
            MetadataExportDatabaseDumpTypeEnum::AVRO => "AVRO",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataExportDatabaseDumpTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(MetadataExportDatabaseDumpTypeEnum::TYPEUNSPECIFIED),
           "MYSQL" => Ok(MetadataExportDatabaseDumpTypeEnum::MYSQL),
           "AVRO" => Ok(MetadataExportDatabaseDumpTypeEnum::AVRO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataExportDatabaseDumpTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataExportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the export.
pub enum MetadataExportStateEnum {
    

    /// The state of the metadata export is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The metadata export is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The metadata export completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The metadata export failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The metadata export is cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for MetadataExportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataExportStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MetadataExportStateEnum::RUNNING => "RUNNING",
            MetadataExportStateEnum::SUCCEEDED => "SUCCEEDED",
            MetadataExportStateEnum::FAILED => "FAILED",
            MetadataExportStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataExportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MetadataExportStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(MetadataExportStateEnum::RUNNING),
           "SUCCEEDED" => Ok(MetadataExportStateEnum::SUCCEEDED),
           "FAILED" => Ok(MetadataExportStateEnum::FAILED),
           "CANCELLED" => Ok(MetadataExportStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataExportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataImportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the metadata import.
pub enum MetadataImportStateEnum {
    

    /// The state of the metadata import is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The metadata import is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The metadata import completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The metadata import is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The metadata import failed, and attempted metadata changes were rolled back.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for MetadataImportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataImportStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MetadataImportStateEnum::RUNNING => "RUNNING",
            MetadataImportStateEnum::SUCCEEDED => "SUCCEEDED",
            MetadataImportStateEnum::UPDATING => "UPDATING",
            MetadataImportStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataImportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MetadataImportStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(MetadataImportStateEnum::RUNNING),
           "SUCCEEDED" => Ok(MetadataImportStateEnum::SUCCEEDED),
           "UPDATING" => Ok(MetadataImportStateEnum::UPDATING),
           "FAILED" => Ok(MetadataImportStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataImportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationExecutionPhaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current phase of the migration execution.
pub enum MigrationExecutionPhaseEnum {
    

    /// The phase of the migration execution is unknown.
    ///
    /// "PHASE_UNSPECIFIED"
    #[serde(rename="PHASE_UNSPECIFIED")]
    PHASEUNSPECIFIED,
    

    /// Replication phase refers to the migration phase when Dataproc Metastore is running a pipeline to replicate changes in the customer database to its backend database. During this phase, Dataproc Metastore uses the customer database as the hive metastore backend database.
    ///
    /// "REPLICATION"
    #[serde(rename="REPLICATION")]
    REPLICATION,
    

    /// Cutover phase refers to the migration phase when Dataproc Metastore switches to using its own backend database. Migration enters this phase when customer is done migrating all their clusters/workloads to Dataproc Metastore and triggers CompleteMigration.
    ///
    /// "CUTOVER"
    #[serde(rename="CUTOVER")]
    CUTOVER,
}

impl AsRef<str> for MigrationExecutionPhaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationExecutionPhaseEnum::PHASEUNSPECIFIED => "PHASE_UNSPECIFIED",
            MigrationExecutionPhaseEnum::REPLICATION => "REPLICATION",
            MigrationExecutionPhaseEnum::CUTOVER => "CUTOVER",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationExecutionPhaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PHASE_UNSPECIFIED" => Ok(MigrationExecutionPhaseEnum::PHASEUNSPECIFIED),
           "REPLICATION" => Ok(MigrationExecutionPhaseEnum::REPLICATION),
           "CUTOVER" => Ok(MigrationExecutionPhaseEnum::CUTOVER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationExecutionPhaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationExecutionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the migration execution.
pub enum MigrationExecutionStateEnum {
    

    /// The state of the migration execution is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The migration execution is starting.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The migration execution is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The migration execution is in the process of being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The migration execution is awaiting user action.
    ///
    /// "AWAITING_USER_ACTION"
    #[serde(rename="AWAITING_USER_ACTION")]
    AWAITINGUSERACTION,
    

    /// The migration execution has completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The migration execution has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The migration execution is cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The migration execution is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for MigrationExecutionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationExecutionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MigrationExecutionStateEnum::STARTING => "STARTING",
            MigrationExecutionStateEnum::RUNNING => "RUNNING",
            MigrationExecutionStateEnum::CANCELLING => "CANCELLING",
            MigrationExecutionStateEnum::AWAITINGUSERACTION => "AWAITING_USER_ACTION",
            MigrationExecutionStateEnum::SUCCEEDED => "SUCCEEDED",
            MigrationExecutionStateEnum::FAILED => "FAILED",
            MigrationExecutionStateEnum::CANCELLED => "CANCELLED",
            MigrationExecutionStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationExecutionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MigrationExecutionStateEnum::STATEUNSPECIFIED),
           "STARTING" => Ok(MigrationExecutionStateEnum::STARTING),
           "RUNNING" => Ok(MigrationExecutionStateEnum::RUNNING),
           "CANCELLING" => Ok(MigrationExecutionStateEnum::CANCELLING),
           "AWAITING_USER_ACTION" => Ok(MigrationExecutionStateEnum::AWAITINGUSERACTION),
           "SUCCEEDED" => Ok(MigrationExecutionStateEnum::SUCCEEDED),
           "FAILED" => Ok(MigrationExecutionStateEnum::FAILED),
           "CANCELLED" => Ok(MigrationExecutionStateEnum::CANCELLED),
           "DELETING" => Ok(MigrationExecutionStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationExecutionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestoreStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the restore.
pub enum RestoreStateEnum {
    

    /// The state of the metadata restore is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The metadata restore is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The metadata restore completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The metadata restore failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The metadata restore is cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for RestoreStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RestoreStateEnum::RUNNING => "RUNNING",
            RestoreStateEnum::SUCCEEDED => "SUCCEEDED",
            RestoreStateEnum::FAILED => "FAILED",
            RestoreStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RestoreStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(RestoreStateEnum::RUNNING),
           "SUCCEEDED" => Ok(RestoreStateEnum::SUCCEEDED),
           "FAILED" => Ok(RestoreStateEnum::FAILED),
           "CANCELLED" => Ok(RestoreStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestoreStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of restore.
pub enum RestoreTypeEnum {
    

    /// The restore type is unknown.
    ///
    /// "RESTORE_TYPE_UNSPECIFIED"
    #[serde(rename="RESTORE_TYPE_UNSPECIFIED")]
    RESTORETYPEUNSPECIFIED,
    

    /// The service's metadata and configuration are restored.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Only the service's metadata is restored.
    ///
    /// "METADATA_ONLY"
    #[serde(rename="METADATA_ONLY")]
    METADATAONLY,
}

impl AsRef<str> for RestoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreTypeEnum::RESTORETYPEUNSPECIFIED => "RESTORE_TYPE_UNSPECIFIED",
            RestoreTypeEnum::FULL => "FULL",
            RestoreTypeEnum::METADATAONLY => "METADATA_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTORE_TYPE_UNSPECIFIED" => Ok(RestoreTypeEnum::RESTORETYPEUNSPECIFIED),
           "FULL" => Ok(RestoreTypeEnum::FULL),
           "METADATA_ONLY" => Ok(RestoreTypeEnum::METADATAONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestoreServiceRequestRestoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of restore. If unspecified, defaults to METADATA_ONLY.
pub enum RestoreServiceRequestRestoreTypeEnum {
    

    /// The restore type is unknown.
    ///
    /// "RESTORE_TYPE_UNSPECIFIED"
    #[serde(rename="RESTORE_TYPE_UNSPECIFIED")]
    RESTORETYPEUNSPECIFIED,
    

    /// The service's metadata and configuration are restored.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Only the service's metadata is restored.
    ///
    /// "METADATA_ONLY"
    #[serde(rename="METADATA_ONLY")]
    METADATAONLY,
}

impl AsRef<str> for RestoreServiceRequestRestoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreServiceRequestRestoreTypeEnum::RESTORETYPEUNSPECIFIED => "RESTORE_TYPE_UNSPECIFIED",
            RestoreServiceRequestRestoreTypeEnum::FULL => "FULL",
            RestoreServiceRequestRestoreTypeEnum::METADATAONLY => "METADATA_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreServiceRequestRestoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTORE_TYPE_UNSPECIFIED" => Ok(RestoreServiceRequestRestoreTypeEnum::RESTORETYPEUNSPECIFIED),
           "FULL" => Ok(RestoreServiceRequestRestoreTypeEnum::FULL),
           "METADATA_ONLY" => Ok(RestoreServiceRequestRestoreTypeEnum::METADATAONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestoreServiceRequestRestoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScalingConfigInstanceSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An enum of readable instance sizes, with each instance size mapping to a float value (e.g. InstanceSize.EXTRA_SMALL = scaling_factor(0.1))
pub enum ScalingConfigInstanceSizeEnum {
    

    /// Unspecified instance size
    ///
    /// "INSTANCE_SIZE_UNSPECIFIED"
    #[serde(rename="INSTANCE_SIZE_UNSPECIFIED")]
    INSTANCESIZEUNSPECIFIED,
    

    /// Extra small instance size, maps to a scaling factor of 0.1.
    ///
    /// "EXTRA_SMALL"
    #[serde(rename="EXTRA_SMALL")]
    EXTRASMALL,
    

    /// Small instance size, maps to a scaling factor of 0.5.
    ///
    /// "SMALL"
    #[serde(rename="SMALL")]
    SMALL,
    

    /// Medium instance size, maps to a scaling factor of 1.0.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Large instance size, maps to a scaling factor of 3.0.
    ///
    /// "LARGE"
    #[serde(rename="LARGE")]
    LARGE,
    

    /// Extra large instance size, maps to a scaling factor of 6.0.
    ///
    /// "EXTRA_LARGE"
    #[serde(rename="EXTRA_LARGE")]
    EXTRALARGE,
}

impl AsRef<str> for ScalingConfigInstanceSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScalingConfigInstanceSizeEnum::INSTANCESIZEUNSPECIFIED => "INSTANCE_SIZE_UNSPECIFIED",
            ScalingConfigInstanceSizeEnum::EXTRASMALL => "EXTRA_SMALL",
            ScalingConfigInstanceSizeEnum::SMALL => "SMALL",
            ScalingConfigInstanceSizeEnum::MEDIUM => "MEDIUM",
            ScalingConfigInstanceSizeEnum::LARGE => "LARGE",
            ScalingConfigInstanceSizeEnum::EXTRALARGE => "EXTRA_LARGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ScalingConfigInstanceSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTANCE_SIZE_UNSPECIFIED" => Ok(ScalingConfigInstanceSizeEnum::INSTANCESIZEUNSPECIFIED),
           "EXTRA_SMALL" => Ok(ScalingConfigInstanceSizeEnum::EXTRASMALL),
           "SMALL" => Ok(ScalingConfigInstanceSizeEnum::SMALL),
           "MEDIUM" => Ok(ScalingConfigInstanceSizeEnum::MEDIUM),
           "LARGE" => Ok(ScalingConfigInstanceSizeEnum::LARGE),
           "EXTRA_LARGE" => Ok(ScalingConfigInstanceSizeEnum::EXTRALARGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScalingConfigInstanceSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceDatabaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The database type that the Metastore service stores its data.
pub enum ServiceDatabaseTypeEnum {
    

    /// The DATABASE_TYPE is not set.
    ///
    /// "DATABASE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_TYPE_UNSPECIFIED")]
    DATABASETYPEUNSPECIFIED,
    

    /// MySQL is used to persist the metastore data.
    ///
    /// "MYSQL"
    #[serde(rename="MYSQL")]
    MYSQL,
    

    /// Spanner is used to persist the metastore data.
    ///
    /// "SPANNER"
    #[serde(rename="SPANNER")]
    SPANNER,
}

impl AsRef<str> for ServiceDatabaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceDatabaseTypeEnum::DATABASETYPEUNSPECIFIED => "DATABASE_TYPE_UNSPECIFIED",
            ServiceDatabaseTypeEnum::MYSQL => "MYSQL",
            ServiceDatabaseTypeEnum::SPANNER => "SPANNER",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceDatabaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_TYPE_UNSPECIFIED" => Ok(ServiceDatabaseTypeEnum::DATABASETYPEUNSPECIFIED),
           "MYSQL" => Ok(ServiceDatabaseTypeEnum::MYSQL),
           "SPANNER" => Ok(ServiceDatabaseTypeEnum::SPANNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceDatabaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceReleaseChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The release channel of the service. If unspecified, defaults to STABLE.
pub enum ServiceReleaseChannelEnum {
    

    /// Release channel is not specified.
    ///
    /// "RELEASE_CHANNEL_UNSPECIFIED"
    #[serde(rename="RELEASE_CHANNEL_UNSPECIFIED")]
    RELEASECHANNELUNSPECIFIED,
    

    /// The CANARY release channel contains the newest features, which may be unstable and subject to unresolved issues with no known workarounds. Services using the CANARY release channel are not subject to any SLAs.
    ///
    /// "CANARY"
    #[serde(rename="CANARY")]
    CANARY,
    

    /// The STABLE release channel contains features that are considered stable and have been validated for production use.
    ///
    /// "STABLE"
    #[serde(rename="STABLE")]
    STABLE,
}

impl AsRef<str> for ServiceReleaseChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceReleaseChannelEnum::RELEASECHANNELUNSPECIFIED => "RELEASE_CHANNEL_UNSPECIFIED",
            ServiceReleaseChannelEnum::CANARY => "CANARY",
            ServiceReleaseChannelEnum::STABLE => "STABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceReleaseChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELEASE_CHANNEL_UNSPECIFIED" => Ok(ServiceReleaseChannelEnum::RELEASECHANNELUNSPECIFIED),
           "CANARY" => Ok(ServiceReleaseChannelEnum::CANARY),
           "STABLE" => Ok(ServiceReleaseChannelEnum::STABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceReleaseChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the metastore service.
pub enum ServiceStateEnum {
    

    /// The state of the metastore service is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The metastore service is in the process of being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The metastore service is running and ready to serve queries.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The metastore service is entering suspension. Its query-serving availability may cease unexpectedly.
    ///
    /// "SUSPENDING"
    #[serde(rename="SUSPENDING")]
    SUSPENDING,
    

    /// The metastore service is suspended and unable to serve queries.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// The metastore service is being updated. It remains usable but cannot accept additional update requests or be deleted at this time.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The metastore service is undergoing deletion. It cannot be used.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The metastore service has encountered an error and cannot be used. The metastore service should be deleted.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The metastore service is processing a managed migration.
    ///
    /// "MIGRATING"
    #[serde(rename="MIGRATING")]
    MIGRATING,
}

impl AsRef<str> for ServiceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ServiceStateEnum::CREATING => "CREATING",
            ServiceStateEnum::ACTIVE => "ACTIVE",
            ServiceStateEnum::SUSPENDING => "SUSPENDING",
            ServiceStateEnum::SUSPENDED => "SUSPENDED",
            ServiceStateEnum::UPDATING => "UPDATING",
            ServiceStateEnum::DELETING => "DELETING",
            ServiceStateEnum::ERROR => "ERROR",
            ServiceStateEnum::MIGRATING => "MIGRATING",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ServiceStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(ServiceStateEnum::CREATING),
           "ACTIVE" => Ok(ServiceStateEnum::ACTIVE),
           "SUSPENDING" => Ok(ServiceStateEnum::SUSPENDING),
           "SUSPENDED" => Ok(ServiceStateEnum::SUSPENDED),
           "UPDATING" => Ok(ServiceStateEnum::UPDATING),
           "DELETING" => Ok(ServiceStateEnum::DELETING),
           "ERROR" => Ok(ServiceStateEnum::ERROR),
           "MIGRATING" => Ok(ServiceStateEnum::MIGRATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The tier of the service.
pub enum ServiceTierEnum {
    

    /// The tier is not set.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// The developer tier provides limited scalability and no fault tolerance. Good for low-cost proof-of-concept.
    ///
    /// "DEVELOPER"
    #[serde(rename="DEVELOPER")]
    DEVELOPER,
    

    /// The enterprise tier provides multi-zone high availability, and sufficient scalability for enterprise-level Dataproc Metastore workloads.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
}

impl AsRef<str> for ServiceTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            ServiceTierEnum::DEVELOPER => "DEVELOPER",
            ServiceTierEnum::ENTERPRISE => "ENTERPRISE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(ServiceTierEnum::TIERUNSPECIFIED),
           "DEVELOPER" => Ok(ServiceTierEnum::DEVELOPER),
           "ENTERPRISE" => Ok(ServiceTierEnum::ENTERPRISE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TelemetryConfigLogFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The output format of the Dataproc Metastore service's logs.
pub enum TelemetryConfigLogFormatEnum {
    

    /// The LOG_FORMAT is not set.
    ///
    /// "LOG_FORMAT_UNSPECIFIED"
    #[serde(rename="LOG_FORMAT_UNSPECIFIED")]
    LOGFORMATUNSPECIFIED,
    

    /// Logging output uses the legacy textPayload format.
    ///
    /// "LEGACY"
    #[serde(rename="LEGACY")]
    LEGACY,
    

    /// Logging output uses the jsonPayload format.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for TelemetryConfigLogFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TelemetryConfigLogFormatEnum::LOGFORMATUNSPECIFIED => "LOG_FORMAT_UNSPECIFIED",
            TelemetryConfigLogFormatEnum::LEGACY => "LEGACY",
            TelemetryConfigLogFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for TelemetryConfigLogFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_FORMAT_UNSPECIFIED" => Ok(TelemetryConfigLogFormatEnum::LOGFORMATUNSPECIFIED),
           "LEGACY" => Ok(TelemetryConfigLogFormatEnum::LEGACY),
           "JSON" => Ok(TelemetryConfigLogFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TelemetryConfigLogFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


