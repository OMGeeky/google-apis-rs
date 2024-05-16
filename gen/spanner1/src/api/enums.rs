use super::*;



// region BackupDatabaseDialectEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The database dialect information for the backup.
pub enum BackupDatabaseDialectEnum {
    

    /// Default value. This value will create a database with the GOOGLE_STANDARD_SQL dialect.
    ///
    /// "DATABASE_DIALECT_UNSPECIFIED"
    #[serde(rename="DATABASE_DIALECT_UNSPECIFIED")]
    DATABASEDIALECTUNSPECIFIED,
    

    /// GoogleSQL supported SQL.
    ///
    /// "GOOGLE_STANDARD_SQL"
    #[serde(rename="GOOGLE_STANDARD_SQL")]
    GOOGLESTANDARDSQL,
    

    /// PostgreSQL supported SQL.
    ///
    /// "POSTGRESQL"
    #[serde(rename="POSTGRESQL")]
    POSTGRESQL,
}

impl AsRef<str> for BackupDatabaseDialectEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupDatabaseDialectEnum::DATABASEDIALECTUNSPECIFIED => "DATABASE_DIALECT_UNSPECIFIED",
            BackupDatabaseDialectEnum::GOOGLESTANDARDSQL => "GOOGLE_STANDARD_SQL",
            BackupDatabaseDialectEnum::POSTGRESQL => "POSTGRESQL",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupDatabaseDialectEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_DIALECT_UNSPECIFIED" => Ok(BackupDatabaseDialectEnum::DATABASEDIALECTUNSPECIFIED),
           "GOOGLE_STANDARD_SQL" => Ok(BackupDatabaseDialectEnum::GOOGLESTANDARDSQL),
           "POSTGRESQL" => Ok(BackupDatabaseDialectEnum::POSTGRESQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupDatabaseDialectEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the backup.
pub enum BackupStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The pending backup is still being created. Operations on the backup may fail with `FAILED_PRECONDITION` in this state.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The backup is complete and ready for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
}

impl AsRef<str> for BackupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BackupStateEnum::CREATING => "CREATING",
            BackupStateEnum::READY => "READY",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BackupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(BackupStateEnum::CREATING),
           "READY" => Ok(BackupStateEnum::READY),
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


// region ContextValueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of this context.
pub enum ContextValueSeverityEnum {
    

    /// Required default value.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Lowest severity level "Info".
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Middle severity level "Warning".
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Severity level signaling an error "Error"
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Severity level signaling a non recoverable error "Fatal"
    ///
    /// "FATAL"
    #[serde(rename="FATAL")]
    FATAL,
}

impl AsRef<str> for ContextValueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContextValueSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            ContextValueSeverityEnum::INFO => "INFO",
            ContextValueSeverityEnum::WARNING => "WARNING",
            ContextValueSeverityEnum::ERROR => "ERROR",
            ContextValueSeverityEnum::FATAL => "FATAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ContextValueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(ContextValueSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(ContextValueSeverityEnum::INFO),
           "WARNING" => Ok(ContextValueSeverityEnum::WARNING),
           "ERROR" => Ok(ContextValueSeverityEnum::ERROR),
           "FATAL" => Ok(ContextValueSeverityEnum::FATAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContextValueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CopyBackupEncryptionConfigEncryptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The encryption type of the backup.
pub enum CopyBackupEncryptionConfigEncryptionTypeEnum {
    

    /// Unspecified. Do not use.
    ///
    /// "ENCRYPTION_TYPE_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_TYPE_UNSPECIFIED")]
    ENCRYPTIONTYPEUNSPECIFIED,
    

    /// This is the default option for CopyBackup when encryption_config is not specified. For example, if the source backup is using `Customer_Managed_Encryption`, the backup will be using the same Cloud KMS key as the source backup.
    ///
    /// "USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION"
    #[serde(rename="USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION")]
    USECONFIGDEFAULTORBACKUPENCRYPTION,
    

    /// Use Google default encryption.
    ///
    /// "GOOGLE_DEFAULT_ENCRYPTION"
    #[serde(rename="GOOGLE_DEFAULT_ENCRYPTION")]
    GOOGLEDEFAULTENCRYPTION,
    

    /// Use customer managed encryption. If specified, either `kms_key_name` or `kms_key_names` must contain valid Cloud KMS key(s).
    ///
    /// "CUSTOMER_MANAGED_ENCRYPTION"
    #[serde(rename="CUSTOMER_MANAGED_ENCRYPTION")]
    CUSTOMERMANAGEDENCRYPTION,
}

impl AsRef<str> for CopyBackupEncryptionConfigEncryptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CopyBackupEncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED => "ENCRYPTION_TYPE_UNSPECIFIED",
            CopyBackupEncryptionConfigEncryptionTypeEnum::USECONFIGDEFAULTORBACKUPENCRYPTION => "USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION",
            CopyBackupEncryptionConfigEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION => "GOOGLE_DEFAULT_ENCRYPTION",
            CopyBackupEncryptionConfigEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION => "CUSTOMER_MANAGED_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for CopyBackupEncryptionConfigEncryptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_TYPE_UNSPECIFIED" => Ok(CopyBackupEncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED),
           "USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION" => Ok(CopyBackupEncryptionConfigEncryptionTypeEnum::USECONFIGDEFAULTORBACKUPENCRYPTION),
           "GOOGLE_DEFAULT_ENCRYPTION" => Ok(CopyBackupEncryptionConfigEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION),
           "CUSTOMER_MANAGED_ENCRYPTION" => Ok(CopyBackupEncryptionConfigEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CopyBackupEncryptionConfigEncryptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreateDatabaseRequestDatabaseDialectEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The dialect of the Cloud Spanner Database.
pub enum CreateDatabaseRequestDatabaseDialectEnum {
    

    /// Default value. This value will create a database with the GOOGLE_STANDARD_SQL dialect.
    ///
    /// "DATABASE_DIALECT_UNSPECIFIED"
    #[serde(rename="DATABASE_DIALECT_UNSPECIFIED")]
    DATABASEDIALECTUNSPECIFIED,
    

    /// GoogleSQL supported SQL.
    ///
    /// "GOOGLE_STANDARD_SQL"
    #[serde(rename="GOOGLE_STANDARD_SQL")]
    GOOGLESTANDARDSQL,
    

    /// PostgreSQL supported SQL.
    ///
    /// "POSTGRESQL"
    #[serde(rename="POSTGRESQL")]
    POSTGRESQL,
}

impl AsRef<str> for CreateDatabaseRequestDatabaseDialectEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreateDatabaseRequestDatabaseDialectEnum::DATABASEDIALECTUNSPECIFIED => "DATABASE_DIALECT_UNSPECIFIED",
            CreateDatabaseRequestDatabaseDialectEnum::GOOGLESTANDARDSQL => "GOOGLE_STANDARD_SQL",
            CreateDatabaseRequestDatabaseDialectEnum::POSTGRESQL => "POSTGRESQL",
        }
    }
}

impl std::convert::TryFrom< &str> for CreateDatabaseRequestDatabaseDialectEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_DIALECT_UNSPECIFIED" => Ok(CreateDatabaseRequestDatabaseDialectEnum::DATABASEDIALECTUNSPECIFIED),
           "GOOGLE_STANDARD_SQL" => Ok(CreateDatabaseRequestDatabaseDialectEnum::GOOGLESTANDARDSQL),
           "POSTGRESQL" => Ok(CreateDatabaseRequestDatabaseDialectEnum::POSTGRESQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreateDatabaseRequestDatabaseDialectEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseDatabaseDialectEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The dialect of the Cloud Spanner Database.
pub enum DatabaseDatabaseDialectEnum {
    

    /// Default value. This value will create a database with the GOOGLE_STANDARD_SQL dialect.
    ///
    /// "DATABASE_DIALECT_UNSPECIFIED"
    #[serde(rename="DATABASE_DIALECT_UNSPECIFIED")]
    DATABASEDIALECTUNSPECIFIED,
    

    /// GoogleSQL supported SQL.
    ///
    /// "GOOGLE_STANDARD_SQL"
    #[serde(rename="GOOGLE_STANDARD_SQL")]
    GOOGLESTANDARDSQL,
    

    /// PostgreSQL supported SQL.
    ///
    /// "POSTGRESQL"
    #[serde(rename="POSTGRESQL")]
    POSTGRESQL,
}

impl AsRef<str> for DatabaseDatabaseDialectEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseDatabaseDialectEnum::DATABASEDIALECTUNSPECIFIED => "DATABASE_DIALECT_UNSPECIFIED",
            DatabaseDatabaseDialectEnum::GOOGLESTANDARDSQL => "GOOGLE_STANDARD_SQL",
            DatabaseDatabaseDialectEnum::POSTGRESQL => "POSTGRESQL",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseDatabaseDialectEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_DIALECT_UNSPECIFIED" => Ok(DatabaseDatabaseDialectEnum::DATABASEDIALECTUNSPECIFIED),
           "GOOGLE_STANDARD_SQL" => Ok(DatabaseDatabaseDialectEnum::GOOGLESTANDARDSQL),
           "POSTGRESQL" => Ok(DatabaseDatabaseDialectEnum::POSTGRESQL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseDatabaseDialectEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current database state.
pub enum DatabaseStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The database is still being created. Operations on the database may fail with `FAILED_PRECONDITION` in this state.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The database is fully created and ready for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The database is fully created and ready for use, but is still being optimized for performance and cannot handle full load. In this state, the database still references the backup it was restore from, preventing the backup from being deleted. When optimizations are complete, the full performance of the database will be restored, and the database will transition to `READY` state.
    ///
    /// "READY_OPTIMIZING"
    #[serde(rename="READY_OPTIMIZING")]
    READYOPTIMIZING,
}

impl AsRef<str> for DatabaseStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            DatabaseStateEnum::CREATING => "CREATING",
            DatabaseStateEnum::READY => "READY",
            DatabaseStateEnum::READYOPTIMIZING => "READY_OPTIMIZING",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(DatabaseStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(DatabaseStateEnum::CREATING),
           "READY" => Ok(DatabaseStateEnum::READY),
           "READY_OPTIMIZING" => Ok(DatabaseStateEnum::READYOPTIMIZING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiagnosticMessageSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the diagnostic message.
pub enum DiagnosticMessageSeverityEnum {
    

    /// Required default value.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Lowest severity level "Info".
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Middle severity level "Warning".
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Severity level signaling an error "Error"
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Severity level signaling a non recoverable error "Fatal"
    ///
    /// "FATAL"
    #[serde(rename="FATAL")]
    FATAL,
}

impl AsRef<str> for DiagnosticMessageSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiagnosticMessageSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            DiagnosticMessageSeverityEnum::INFO => "INFO",
            DiagnosticMessageSeverityEnum::WARNING => "WARNING",
            DiagnosticMessageSeverityEnum::ERROR => "ERROR",
            DiagnosticMessageSeverityEnum::FATAL => "FATAL",
        }
    }
}

impl std::convert::TryFrom< &str> for DiagnosticMessageSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(DiagnosticMessageSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(DiagnosticMessageSeverityEnum::INFO),
           "WARNING" => Ok(DiagnosticMessageSeverityEnum::WARNING),
           "ERROR" => Ok(DiagnosticMessageSeverityEnum::ERROR),
           "FATAL" => Ok(DiagnosticMessageSeverityEnum::FATAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiagnosticMessageSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptionInfoEncryptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of encryption.
pub enum EncryptionInfoEncryptionTypeEnum {
    

    /// Encryption type was not specified, though data at rest remains encrypted.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The data is encrypted at rest with a key that is fully managed by Google. No key version or status will be populated. This is the default state.
    ///
    /// "GOOGLE_DEFAULT_ENCRYPTION"
    #[serde(rename="GOOGLE_DEFAULT_ENCRYPTION")]
    GOOGLEDEFAULTENCRYPTION,
    

    /// The data is encrypted at rest with a key that is managed by the customer. The active version of the key. `kms_key_version` will be populated, and `encryption_status` may be populated.
    ///
    /// "CUSTOMER_MANAGED_ENCRYPTION"
    #[serde(rename="CUSTOMER_MANAGED_ENCRYPTION")]
    CUSTOMERMANAGEDENCRYPTION,
}

impl AsRef<str> for EncryptionInfoEncryptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptionInfoEncryptionTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            EncryptionInfoEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION => "GOOGLE_DEFAULT_ENCRYPTION",
            EncryptionInfoEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION => "CUSTOMER_MANAGED_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptionInfoEncryptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(EncryptionInfoEncryptionTypeEnum::TYPEUNSPECIFIED),
           "GOOGLE_DEFAULT_ENCRYPTION" => Ok(EncryptionInfoEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION),
           "CUSTOMER_MANAGED_ENCRYPTION" => Ok(EncryptionInfoEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EncryptionInfoEncryptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExecuteSqlRequestQueryModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Used to control the amount of debugging information returned in ResultSetStats. If partition_token is set, query_mode can only be set to QueryMode.NORMAL.
pub enum ExecuteSqlRequestQueryModeEnum {
    

    /// The default mode. Only the statement results are returned.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// This mode returns only the query plan, without any results or execution statistics information.
    ///
    /// "PLAN"
    #[serde(rename="PLAN")]
    PLAN,
    

    /// This mode returns both the query plan and the execution statistics along with the results.
    ///
    /// "PROFILE"
    #[serde(rename="PROFILE")]
    PROFILE,
}

impl AsRef<str> for ExecuteSqlRequestQueryModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecuteSqlRequestQueryModeEnum::NORMAL => "NORMAL",
            ExecuteSqlRequestQueryModeEnum::PLAN => "PLAN",
            ExecuteSqlRequestQueryModeEnum::PROFILE => "PROFILE",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecuteSqlRequestQueryModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NORMAL" => Ok(ExecuteSqlRequestQueryModeEnum::NORMAL),
           "PLAN" => Ok(ExecuteSqlRequestQueryModeEnum::PLAN),
           "PROFILE" => Ok(ExecuteSqlRequestQueryModeEnum::PROFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecuteSqlRequestQueryModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FreeInstanceMetadataExpireBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the expiration behavior of a free instance. The default of ExpireBehavior is `REMOVE_AFTER_GRACE_PERIOD`. This can be modified during or after creation, and before expiration.
pub enum FreeInstanceMetadataExpireBehaviorEnum {
    

    /// Not specified.
    ///
    /// "EXPIRE_BEHAVIOR_UNSPECIFIED"
    #[serde(rename="EXPIRE_BEHAVIOR_UNSPECIFIED")]
    EXPIREBEHAVIORUNSPECIFIED,
    

    /// When the free instance expires, upgrade the instance to a provisioned instance.
    ///
    /// "FREE_TO_PROVISIONED"
    #[serde(rename="FREE_TO_PROVISIONED")]
    FREETOPROVISIONED,
    

    /// When the free instance expires, disable the instance, and delete it after the grace period passes if it has not been upgraded.
    ///
    /// "REMOVE_AFTER_GRACE_PERIOD"
    #[serde(rename="REMOVE_AFTER_GRACE_PERIOD")]
    REMOVEAFTERGRACEPERIOD,
}

impl AsRef<str> for FreeInstanceMetadataExpireBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FreeInstanceMetadataExpireBehaviorEnum::EXPIREBEHAVIORUNSPECIFIED => "EXPIRE_BEHAVIOR_UNSPECIFIED",
            FreeInstanceMetadataExpireBehaviorEnum::FREETOPROVISIONED => "FREE_TO_PROVISIONED",
            FreeInstanceMetadataExpireBehaviorEnum::REMOVEAFTERGRACEPERIOD => "REMOVE_AFTER_GRACE_PERIOD",
        }
    }
}

impl std::convert::TryFrom< &str> for FreeInstanceMetadataExpireBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPIRE_BEHAVIOR_UNSPECIFIED" => Ok(FreeInstanceMetadataExpireBehaviorEnum::EXPIREBEHAVIORUNSPECIFIED),
           "FREE_TO_PROVISIONED" => Ok(FreeInstanceMetadataExpireBehaviorEnum::FREETOPROVISIONED),
           "REMOVE_AFTER_GRACE_PERIOD" => Ok(FreeInstanceMetadataExpireBehaviorEnum::REMOVEAFTERGRACEPERIOD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FreeInstanceMetadataExpireBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceInstanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The `InstanceType` of the current instance.
pub enum InstanceInstanceTypeEnum {
    

    /// Not specified.
    ///
    /// "INSTANCE_TYPE_UNSPECIFIED"
    #[serde(rename="INSTANCE_TYPE_UNSPECIFIED")]
    INSTANCETYPEUNSPECIFIED,
    

    /// Provisioned instances have dedicated resources, standard usage limits and support.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
    

    /// Free instances provide no guarantee for dedicated resources, [node_count, processing_units] should be 0. They come with stricter usage limits and limited support.
    ///
    /// "FREE_INSTANCE"
    #[serde(rename="FREE_INSTANCE")]
    FREEINSTANCE,
}

impl AsRef<str> for InstanceInstanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceInstanceTypeEnum::INSTANCETYPEUNSPECIFIED => "INSTANCE_TYPE_UNSPECIFIED",
            InstanceInstanceTypeEnum::PROVISIONED => "PROVISIONED",
            InstanceInstanceTypeEnum::FREEINSTANCE => "FREE_INSTANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceInstanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTANCE_TYPE_UNSPECIFIED" => Ok(InstanceInstanceTypeEnum::INSTANCETYPEUNSPECIFIED),
           "PROVISIONED" => Ok(InstanceInstanceTypeEnum::PROVISIONED),
           "FREE_INSTANCE" => Ok(InstanceInstanceTypeEnum::FREEINSTANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceInstanceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current instance state. For CreateInstance, the state must be either omitted or set to `CREATING`. For UpdateInstance, the state must be either omitted or set to `READY`.
pub enum InstanceStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The instance is still being created. Resources may not be available yet, and operations such as database creation may not work.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The instance is fully created and ready to do work such as creating databases.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::CREATING => "CREATING",
            InstanceStateEnum::READY => "READY",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstanceStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(InstanceStateEnum::CREATING),
           "READY" => Ok(InstanceStateEnum::READY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceConfigConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Whether this instance config is a Google or User Managed Configuration.
pub enum InstanceConfigConfigTypeEnum {
    

    /// Unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Google managed configuration.
    ///
    /// "GOOGLE_MANAGED"
    #[serde(rename="GOOGLE_MANAGED")]
    GOOGLEMANAGED,
    

    /// User managed configuration.
    ///
    /// "USER_MANAGED"
    #[serde(rename="USER_MANAGED")]
    USERMANAGED,
}

impl AsRef<str> for InstanceConfigConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceConfigConfigTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            InstanceConfigConfigTypeEnum::GOOGLEMANAGED => "GOOGLE_MANAGED",
            InstanceConfigConfigTypeEnum::USERMANAGED => "USER_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceConfigConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(InstanceConfigConfigTypeEnum::TYPEUNSPECIFIED),
           "GOOGLE_MANAGED" => Ok(InstanceConfigConfigTypeEnum::GOOGLEMANAGED),
           "USER_MANAGED" => Ok(InstanceConfigConfigTypeEnum::USERMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceConfigConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceConfigFreeInstanceAvailabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Describes whether free instances are available to be created in this instance config.
pub enum InstanceConfigFreeInstanceAvailabilityEnum {
    

    /// Not specified.
    ///
    /// "FREE_INSTANCE_AVAILABILITY_UNSPECIFIED"
    #[serde(rename="FREE_INSTANCE_AVAILABILITY_UNSPECIFIED")]
    FREEINSTANCEAVAILABILITYUNSPECIFIED,
    

    /// Indicates that free instances are available to be created in this instance config.
    ///
    /// "AVAILABLE"
    #[serde(rename="AVAILABLE")]
    AVAILABLE,
    

    /// Indicates that free instances are not supported in this instance config.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// Indicates that free instances are currently not available to be created in this instance config.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Indicates that additional free instances cannot be created in this instance config because the project has reached its limit of free instances.
    ///
    /// "QUOTA_EXCEEDED"
    #[serde(rename="QUOTA_EXCEEDED")]
    QUOTAEXCEEDED,
}

impl AsRef<str> for InstanceConfigFreeInstanceAvailabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceConfigFreeInstanceAvailabilityEnum::FREEINSTANCEAVAILABILITYUNSPECIFIED => "FREE_INSTANCE_AVAILABILITY_UNSPECIFIED",
            InstanceConfigFreeInstanceAvailabilityEnum::AVAILABLE => "AVAILABLE",
            InstanceConfigFreeInstanceAvailabilityEnum::UNSUPPORTED => "UNSUPPORTED",
            InstanceConfigFreeInstanceAvailabilityEnum::DISABLED => "DISABLED",
            InstanceConfigFreeInstanceAvailabilityEnum::QUOTAEXCEEDED => "QUOTA_EXCEEDED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceConfigFreeInstanceAvailabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FREE_INSTANCE_AVAILABILITY_UNSPECIFIED" => Ok(InstanceConfigFreeInstanceAvailabilityEnum::FREEINSTANCEAVAILABILITYUNSPECIFIED),
           "AVAILABLE" => Ok(InstanceConfigFreeInstanceAvailabilityEnum::AVAILABLE),
           "UNSUPPORTED" => Ok(InstanceConfigFreeInstanceAvailabilityEnum::UNSUPPORTED),
           "DISABLED" => Ok(InstanceConfigFreeInstanceAvailabilityEnum::DISABLED),
           "QUOTA_EXCEEDED" => Ok(InstanceConfigFreeInstanceAvailabilityEnum::QUOTAEXCEEDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceConfigFreeInstanceAvailabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current instance config state. Applicable only for USER_MANAGED configs.
pub enum InstanceConfigStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The instance config is still being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The instance config is fully created and ready to be used to create instances.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
}

impl AsRef<str> for InstanceConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceConfigStateEnum::CREATING => "CREATING",
            InstanceConfigStateEnum::READY => "READY",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstanceConfigStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(InstanceConfigStateEnum::CREATING),
           "READY" => Ok(InstanceConfigStateEnum::READY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstancePartitionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current instance partition state.
pub enum InstancePartitionStateEnum {
    

    /// Not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The instance partition is still being created. Resources may not be available yet, and operations such as creating placements using this instance partition may not work.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The instance partition is fully created and ready to do work such as creating placements and using in databases.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
}

impl AsRef<str> for InstancePartitionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstancePartitionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstancePartitionStateEnum::CREATING => "CREATING",
            InstancePartitionStateEnum::READY => "READY",
        }
    }
}

impl std::convert::TryFrom< &str> for InstancePartitionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstancePartitionStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(InstancePartitionStateEnum::CREATING),
           "READY" => Ok(InstancePartitionStateEnum::READY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstancePartitionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricAggregationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The aggregation function used to aggregate each key bucket
pub enum MetricAggregationEnum {
    

    /// Required default value.
    ///
    /// "AGGREGATION_UNSPECIFIED"
    #[serde(rename="AGGREGATION_UNSPECIFIED")]
    AGGREGATIONUNSPECIFIED,
    

    /// Use the maximum of all values.
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
    

    /// Use the sum of all values.
    ///
    /// "SUM"
    #[serde(rename="SUM")]
    SUM,
}

impl AsRef<str> for MetricAggregationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricAggregationEnum::AGGREGATIONUNSPECIFIED => "AGGREGATION_UNSPECIFIED",
            MetricAggregationEnum::MAX => "MAX",
            MetricAggregationEnum::SUM => "SUM",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricAggregationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGGREGATION_UNSPECIFIED" => Ok(MetricAggregationEnum::AGGREGATIONUNSPECIFIED),
           "MAX" => Ok(MetricAggregationEnum::MAX),
           "SUM" => Ok(MetricAggregationEnum::SUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricAggregationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlanNodeKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Used to determine the type of node. May be needed for visualizing different kinds of nodes differently. For example, If the node is a SCALAR node, it will have a condensed representation which can be used to directly embed a description of the node in its parent.
pub enum PlanNodeKindEnum {
    

    /// Not specified.
    ///
    /// "KIND_UNSPECIFIED"
    #[serde(rename="KIND_UNSPECIFIED")]
    KINDUNSPECIFIED,
    

    /// Denotes a Relational operator node in the expression tree. Relational operators represent iterative processing of rows during query execution. For example, a `TableScan` operation that reads rows from a table.
    ///
    /// "RELATIONAL"
    #[serde(rename="RELATIONAL")]
    RELATIONAL,
    

    /// Denotes a Scalar node in the expression tree. Scalar nodes represent non-iterable entities in the query plan. For example, constants or arithmetic operators appearing inside predicate expressions or references to column names.
    ///
    /// "SCALAR"
    #[serde(rename="SCALAR")]
    SCALAR,
}

impl AsRef<str> for PlanNodeKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlanNodeKindEnum::KINDUNSPECIFIED => "KIND_UNSPECIFIED",
            PlanNodeKindEnum::RELATIONAL => "RELATIONAL",
            PlanNodeKindEnum::SCALAR => "SCALAR",
        }
    }
}

impl std::convert::TryFrom< &str> for PlanNodeKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KIND_UNSPECIFIED" => Ok(PlanNodeKindEnum::KINDUNSPECIFIED),
           "RELATIONAL" => Ok(PlanNodeKindEnum::RELATIONAL),
           "SCALAR" => Ok(PlanNodeKindEnum::SCALAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlanNodeKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReadWriteReadLockModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Read lock mode for the transaction.
pub enum ReadWriteReadLockModeEnum {
    

    /// Default value. If the value is not specified, the pessimistic read lock is used.
    ///
    /// "READ_LOCK_MODE_UNSPECIFIED"
    #[serde(rename="READ_LOCK_MODE_UNSPECIFIED")]
    READLOCKMODEUNSPECIFIED,
    

    /// Pessimistic lock mode. Read locks are acquired immediately on read.
    ///
    /// "PESSIMISTIC"
    #[serde(rename="PESSIMISTIC")]
    PESSIMISTIC,
    

    /// Optimistic lock mode. Locks for reads within the transaction are not acquired on read. Instead the locks are acquired on a commit to validate that read/queried data has not changed since the transaction started.
    ///
    /// "OPTIMISTIC"
    #[serde(rename="OPTIMISTIC")]
    OPTIMISTIC,
}

impl AsRef<str> for ReadWriteReadLockModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReadWriteReadLockModeEnum::READLOCKMODEUNSPECIFIED => "READ_LOCK_MODE_UNSPECIFIED",
            ReadWriteReadLockModeEnum::PESSIMISTIC => "PESSIMISTIC",
            ReadWriteReadLockModeEnum::OPTIMISTIC => "OPTIMISTIC",
        }
    }
}

impl std::convert::TryFrom< &str> for ReadWriteReadLockModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_LOCK_MODE_UNSPECIFIED" => Ok(ReadWriteReadLockModeEnum::READLOCKMODEUNSPECIFIED),
           "PESSIMISTIC" => Ok(ReadWriteReadLockModeEnum::PESSIMISTIC),
           "OPTIMISTIC" => Ok(ReadWriteReadLockModeEnum::OPTIMISTIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReadWriteReadLockModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReplicaInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of replica.
pub enum ReplicaInfoTypeEnum {
    

    /// Not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Read-write replicas support both reads and writes. These replicas: * Maintain a full copy of your data. * Serve reads. * Can vote whether to commit a write. * Participate in leadership election. * Are eligible to become a leader.
    ///
    /// "READ_WRITE"
    #[serde(rename="READ_WRITE")]
    READWRITE,
    

    /// Read-only replicas only support reads (not writes). Read-only replicas: * Maintain a full copy of your data. * Serve reads. * Do not participate in voting to commit writes. * Are not eligible to become a leader.
    ///
    /// "READ_ONLY"
    #[serde(rename="READ_ONLY")]
    READONLY,
    

    /// Witness replicas don't support reads but do participate in voting to commit writes. Witness replicas: * Do not maintain a full copy of data. * Do not serve reads. * Vote whether to commit writes. * Participate in leader election but are not eligible to become leader.
    ///
    /// "WITNESS"
    #[serde(rename="WITNESS")]
    WITNESS,
}

impl AsRef<str> for ReplicaInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReplicaInfoTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ReplicaInfoTypeEnum::READWRITE => "READ_WRITE",
            ReplicaInfoTypeEnum::READONLY => "READ_ONLY",
            ReplicaInfoTypeEnum::WITNESS => "WITNESS",
        }
    }
}

impl std::convert::TryFrom< &str> for ReplicaInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ReplicaInfoTypeEnum::TYPEUNSPECIFIED),
           "READ_WRITE" => Ok(ReplicaInfoTypeEnum::READWRITE),
           "READ_ONLY" => Ok(ReplicaInfoTypeEnum::READONLY),
           "WITNESS" => Ok(ReplicaInfoTypeEnum::WITNESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReplicaInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReplicaSelectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of replica.
pub enum ReplicaSelectionTypeEnum {
    

    /// Not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Read-write replicas support both reads and writes.
    ///
    /// "READ_WRITE"
    #[serde(rename="READ_WRITE")]
    READWRITE,
    

    /// Read-only replicas only support reads (not writes).
    ///
    /// "READ_ONLY"
    #[serde(rename="READ_ONLY")]
    READONLY,
}

impl AsRef<str> for ReplicaSelectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReplicaSelectionTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ReplicaSelectionTypeEnum::READWRITE => "READ_WRITE",
            ReplicaSelectionTypeEnum::READONLY => "READ_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for ReplicaSelectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ReplicaSelectionTypeEnum::TYPEUNSPECIFIED),
           "READ_WRITE" => Ok(ReplicaSelectionTypeEnum::READWRITE),
           "READ_ONLY" => Ok(ReplicaSelectionTypeEnum::READONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReplicaSelectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RequestOptionPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Priority for the request.
pub enum RequestOptionPriorityEnum {
    

    /// `PRIORITY_UNSPECIFIED` is equivalent to `PRIORITY_HIGH`.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    

    /// This specifies that the request is low priority.
    ///
    /// "PRIORITY_LOW"
    #[serde(rename="PRIORITY_LOW")]
    PRIORITYLOW,
    

    /// This specifies that the request is medium priority.
    ///
    /// "PRIORITY_MEDIUM"
    #[serde(rename="PRIORITY_MEDIUM")]
    PRIORITYMEDIUM,
    

    /// This specifies that the request is high priority.
    ///
    /// "PRIORITY_HIGH"
    #[serde(rename="PRIORITY_HIGH")]
    PRIORITYHIGH,
}

impl AsRef<str> for RequestOptionPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RequestOptionPriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            RequestOptionPriorityEnum::PRIORITYLOW => "PRIORITY_LOW",
            RequestOptionPriorityEnum::PRIORITYMEDIUM => "PRIORITY_MEDIUM",
            RequestOptionPriorityEnum::PRIORITYHIGH => "PRIORITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for RequestOptionPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(RequestOptionPriorityEnum::PRIORITYUNSPECIFIED),
           "PRIORITY_LOW" => Ok(RequestOptionPriorityEnum::PRIORITYLOW),
           "PRIORITY_MEDIUM" => Ok(RequestOptionPriorityEnum::PRIORITYMEDIUM),
           "PRIORITY_HIGH" => Ok(RequestOptionPriorityEnum::PRIORITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RequestOptionPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestoreDatabaseEncryptionConfigEncryptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The encryption type of the restored database.
pub enum RestoreDatabaseEncryptionConfigEncryptionTypeEnum {
    

    /// Unspecified. Do not use.
    ///
    /// "ENCRYPTION_TYPE_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_TYPE_UNSPECIFIED")]
    ENCRYPTIONTYPEUNSPECIFIED,
    

    /// This is the default option when encryption_config is not specified.
    ///
    /// "USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION"
    #[serde(rename="USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION")]
    USECONFIGDEFAULTORBACKUPENCRYPTION,
    

    /// Use Google default encryption.
    ///
    /// "GOOGLE_DEFAULT_ENCRYPTION"
    #[serde(rename="GOOGLE_DEFAULT_ENCRYPTION")]
    GOOGLEDEFAULTENCRYPTION,
    

    /// Use customer managed encryption. If specified, `kms_key_name` must must contain a valid Cloud KMS key.
    ///
    /// "CUSTOMER_MANAGED_ENCRYPTION"
    #[serde(rename="CUSTOMER_MANAGED_ENCRYPTION")]
    CUSTOMERMANAGEDENCRYPTION,
}

impl AsRef<str> for RestoreDatabaseEncryptionConfigEncryptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreDatabaseEncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED => "ENCRYPTION_TYPE_UNSPECIFIED",
            RestoreDatabaseEncryptionConfigEncryptionTypeEnum::USECONFIGDEFAULTORBACKUPENCRYPTION => "USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION",
            RestoreDatabaseEncryptionConfigEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION => "GOOGLE_DEFAULT_ENCRYPTION",
            RestoreDatabaseEncryptionConfigEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION => "CUSTOMER_MANAGED_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreDatabaseEncryptionConfigEncryptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_TYPE_UNSPECIFIED" => Ok(RestoreDatabaseEncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED),
           "USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION" => Ok(RestoreDatabaseEncryptionConfigEncryptionTypeEnum::USECONFIGDEFAULTORBACKUPENCRYPTION),
           "GOOGLE_DEFAULT_ENCRYPTION" => Ok(RestoreDatabaseEncryptionConfigEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION),
           "CUSTOMER_MANAGED_ENCRYPTION" => Ok(RestoreDatabaseEncryptionConfigEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestoreDatabaseEncryptionConfigEncryptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestoreInfoSourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the restore source.
pub enum RestoreInfoSourceTypeEnum {
    

    /// No restore associated.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A backup was used as the source of the restore.
    ///
    /// "BACKUP"
    #[serde(rename="BACKUP")]
    BACKUP,
}

impl AsRef<str> for RestoreInfoSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreInfoSourceTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            RestoreInfoSourceTypeEnum::BACKUP => "BACKUP",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreInfoSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(RestoreInfoSourceTypeEnum::TYPEUNSPECIFIED),
           "BACKUP" => Ok(RestoreInfoSourceTypeEnum::BACKUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestoreInfoSourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TypeCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The TypeCode for this type.
pub enum TypeCodeEnum {
    

    /// Not specified.
    ///
    /// "TYPE_CODE_UNSPECIFIED"
    #[serde(rename="TYPE_CODE_UNSPECIFIED")]
    TYPECODEUNSPECIFIED,
    

    /// Encoded as JSON `true` or `false`.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// Encoded as `string`, in decimal format.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// Encoded as `number`, or the strings `"NaN"`, `"Infinity"`, or `"-Infinity"`.
    ///
    /// "FLOAT64"
    #[serde(rename="FLOAT64")]
    FLOAT64,
    

    /// Encoded as `number`, or the strings `"NaN"`, `"Infinity"`, or `"-Infinity"`.
    ///
    /// "FLOAT32"
    #[serde(rename="FLOAT32")]
    FLOAT32,
    

    /// Encoded as `string` in RFC 3339 timestamp format. The time zone must be present, and must be `"Z"`. If the schema has the column option `allow_commit_timestamp=true`, the placeholder string `"spanner.commit_timestamp()"` can be used to instruct the system to insert the commit timestamp associated with the transaction commit.
    ///
    /// "TIMESTAMP"
    #[serde(rename="TIMESTAMP")]
    TIMESTAMP,
    

    /// Encoded as `string` in RFC 3339 date format.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Encoded as `string`.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Encoded as a base64-encoded `string`, as described in RFC 4648, section 4.
    ///
    /// "BYTES"
    #[serde(rename="BYTES")]
    BYTES,
    

    /// Encoded as `list`, where the list elements are represented according to array_element_type.
    ///
    /// "ARRAY"
    #[serde(rename="ARRAY")]
    ARRAY,
    

    /// Encoded as `list`, where list element `i` is represented according to [struct_type.fields[i]][google.spanner.v1.StructType.fields].
    ///
    /// "STRUCT"
    #[serde(rename="STRUCT")]
    STRUCT,
    

    /// Encoded as `string`, in decimal format or scientific notation format. Decimal format: `[+-]Digits[.[Digits]]` or `+-.Digits` Scientific notation: `[+-]Digits[.[Digits]][ExponentIndicator[+-]Digits]` or `+-.Digits[ExponentIndicator[+-]Digits]` (ExponentIndicator is `"e"` or `"E"`)
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// Encoded as a JSON-formatted `string` as described in RFC 7159. The following rules are applied when parsing JSON input: - Whitespace characters are not preserved. - If a JSON object has duplicate keys, only the first key is preserved. - Members of a JSON object are not guaranteed to have their order preserved. - JSON array elements will have their order preserved.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Encoded as a base64-encoded `string`, as described in RFC 4648, section 4.
    ///
    /// "PROTO"
    #[serde(rename="PROTO")]
    PROTO,
    

    /// Encoded as `string`, in decimal format.
    ///
    /// "ENUM"
    #[serde(rename="ENUM")]
    ENUM,
}

impl AsRef<str> for TypeCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TypeCodeEnum::TYPECODEUNSPECIFIED => "TYPE_CODE_UNSPECIFIED",
            TypeCodeEnum::BOOL => "BOOL",
            TypeCodeEnum::INT64 => "INT64",
            TypeCodeEnum::FLOAT64 => "FLOAT64",
            TypeCodeEnum::FLOAT32 => "FLOAT32",
            TypeCodeEnum::TIMESTAMP => "TIMESTAMP",
            TypeCodeEnum::DATE => "DATE",
            TypeCodeEnum::STRING => "STRING",
            TypeCodeEnum::BYTES => "BYTES",
            TypeCodeEnum::ARRAY => "ARRAY",
            TypeCodeEnum::STRUCT => "STRUCT",
            TypeCodeEnum::NUMERIC => "NUMERIC",
            TypeCodeEnum::JSON => "JSON",
            TypeCodeEnum::PROTO => "PROTO",
            TypeCodeEnum::ENUM => "ENUM",
        }
    }
}

impl std::convert::TryFrom< &str> for TypeCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_CODE_UNSPECIFIED" => Ok(TypeCodeEnum::TYPECODEUNSPECIFIED),
           "BOOL" => Ok(TypeCodeEnum::BOOL),
           "INT64" => Ok(TypeCodeEnum::INT64),
           "FLOAT64" => Ok(TypeCodeEnum::FLOAT64),
           "FLOAT32" => Ok(TypeCodeEnum::FLOAT32),
           "TIMESTAMP" => Ok(TypeCodeEnum::TIMESTAMP),
           "DATE" => Ok(TypeCodeEnum::DATE),
           "STRING" => Ok(TypeCodeEnum::STRING),
           "BYTES" => Ok(TypeCodeEnum::BYTES),
           "ARRAY" => Ok(TypeCodeEnum::ARRAY),
           "STRUCT" => Ok(TypeCodeEnum::STRUCT),
           "NUMERIC" => Ok(TypeCodeEnum::NUMERIC),
           "JSON" => Ok(TypeCodeEnum::JSON),
           "PROTO" => Ok(TypeCodeEnum::PROTO),
           "ENUM" => Ok(TypeCodeEnum::ENUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TypeCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TypeTypeAnnotationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The TypeAnnotationCode that disambiguates SQL type that Spanner will use to represent values of this type during query processing. This is necessary for some type codes because a single TypeCode can be mapped to different SQL types depending on the SQL dialect. type_annotation typically is not needed to process the content of a value (it doesn't affect serialization) and clients can ignore it on the read path.
pub enum TypeTypeAnnotationEnum {
    

    /// Not specified.
    ///
    /// "TYPE_ANNOTATION_CODE_UNSPECIFIED"
    #[serde(rename="TYPE_ANNOTATION_CODE_UNSPECIFIED")]
    TYPEANNOTATIONCODEUNSPECIFIED,
    

    /// PostgreSQL compatible NUMERIC type. This annotation needs to be applied to Type instances having NUMERIC type code to specify that values of this type should be treated as PostgreSQL NUMERIC values. Currently this annotation is always needed for NUMERIC when a client interacts with PostgreSQL-enabled Spanner databases.
    ///
    /// "PG_NUMERIC"
    #[serde(rename="PG_NUMERIC")]
    PGNUMERIC,
    

    /// PostgreSQL compatible JSONB type. This annotation needs to be applied to Type instances having JSON type code to specify that values of this type should be treated as PostgreSQL JSONB values. Currently this annotation is always needed for JSON when a client interacts with PostgreSQL-enabled Spanner databases.
    ///
    /// "PG_JSONB"
    #[serde(rename="PG_JSONB")]
    PGJSONB,
}

impl AsRef<str> for TypeTypeAnnotationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TypeTypeAnnotationEnum::TYPEANNOTATIONCODEUNSPECIFIED => "TYPE_ANNOTATION_CODE_UNSPECIFIED",
            TypeTypeAnnotationEnum::PGNUMERIC => "PG_NUMERIC",
            TypeTypeAnnotationEnum::PGJSONB => "PG_JSONB",
        }
    }
}

impl std::convert::TryFrom< &str> for TypeTypeAnnotationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_ANNOTATION_CODE_UNSPECIFIED" => Ok(TypeTypeAnnotationEnum::TYPEANNOTATIONCODEUNSPECIFIED),
           "PG_NUMERIC" => Ok(TypeTypeAnnotationEnum::PGNUMERIC),
           "PG_JSONB" => Ok(TypeTypeAnnotationEnum::PGJSONB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TypeTypeAnnotationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VisualizationDataKeyUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The unit for the key: e.g. 'key' or 'chunk'.
pub enum VisualizationDataKeyUnitEnum {
    

    /// Required default value
    ///
    /// "KEY_UNIT_UNSPECIFIED"
    #[serde(rename="KEY_UNIT_UNSPECIFIED")]
    KEYUNITUNSPECIFIED,
    

    /// Each entry corresponds to one key
    ///
    /// "KEY"
    #[serde(rename="KEY")]
    KEY,
    

    /// Each entry corresponds to a chunk of keys
    ///
    /// "CHUNK"
    #[serde(rename="CHUNK")]
    CHUNK,
}

impl AsRef<str> for VisualizationDataKeyUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VisualizationDataKeyUnitEnum::KEYUNITUNSPECIFIED => "KEY_UNIT_UNSPECIFIED",
            VisualizationDataKeyUnitEnum::KEY => "KEY",
            VisualizationDataKeyUnitEnum::CHUNK => "CHUNK",
        }
    }
}

impl std::convert::TryFrom< &str> for VisualizationDataKeyUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEY_UNIT_UNSPECIFIED" => Ok(VisualizationDataKeyUnitEnum::KEYUNITUNSPECIFIED),
           "KEY" => Ok(VisualizationDataKeyUnitEnum::KEY),
           "CHUNK" => Ok(VisualizationDataKeyUnitEnum::CHUNK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VisualizationDataKeyUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectEncryptionConfigEncryptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The encryption type of the backup.
pub enum ProjectEncryptionConfigEncryptionTypeEnum {
    

    /// Unspecified. Do not use.
    ///
    /// "ENCRYPTION_TYPE_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_TYPE_UNSPECIFIED")]
    ENCRYPTIONTYPEUNSPECIFIED,
    

    /// Use the same encryption configuration as the database. This is the default option when encryption_config is empty. For example, if the database is using `Customer_Managed_Encryption`, the backup will be using the same Cloud KMS key as the database.
    ///
    /// "USE_DATABASE_ENCRYPTION"
    #[serde(rename="USE_DATABASE_ENCRYPTION")]
    USEDATABASEENCRYPTION,
    

    /// Use Google default encryption.
    ///
    /// "GOOGLE_DEFAULT_ENCRYPTION"
    #[serde(rename="GOOGLE_DEFAULT_ENCRYPTION")]
    GOOGLEDEFAULTENCRYPTION,
    

    /// Use customer managed encryption. If specified, `kms_key_name` must contain a valid Cloud KMS key.
    ///
    /// "CUSTOMER_MANAGED_ENCRYPTION"
    #[serde(rename="CUSTOMER_MANAGED_ENCRYPTION")]
    CUSTOMERMANAGEDENCRYPTION,
}

impl AsRef<str> for ProjectEncryptionConfigEncryptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectEncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED => "ENCRYPTION_TYPE_UNSPECIFIED",
            ProjectEncryptionConfigEncryptionTypeEnum::USEDATABASEENCRYPTION => "USE_DATABASE_ENCRYPTION",
            ProjectEncryptionConfigEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION => "GOOGLE_DEFAULT_ENCRYPTION",
            ProjectEncryptionConfigEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION => "CUSTOMER_MANAGED_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectEncryptionConfigEncryptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_TYPE_UNSPECIFIED" => Ok(ProjectEncryptionConfigEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED),
           "USE_DATABASE_ENCRYPTION" => Ok(ProjectEncryptionConfigEncryptionTypeEnum::USEDATABASEENCRYPTION),
           "GOOGLE_DEFAULT_ENCRYPTION" => Ok(ProjectEncryptionConfigEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION),
           "CUSTOMER_MANAGED_ENCRYPTION" => Ok(ProjectEncryptionConfigEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectEncryptionConfigEncryptionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which parts of the Scan should be returned in the response. Note, if left unspecified, the FULL view is assumed.
pub enum ProjectViewEnum {
    

    /// Not specified, equivalent to SUMMARY.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Server responses only include `name`, `details`, `start_time` and `end_time`. The default value. Note, the ListScans method may only use this view type, others view types are not supported.
    ///
    /// "SUMMARY"
    #[serde(rename="SUMMARY")]
    SUMMARY,
    

    /// Full representation of the scan is returned in the server response, including `data`.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            ProjectViewEnum::SUMMARY => "SUMMARY",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::VIEWUNSPECIFIED),
           "SUMMARY" => Ok(ProjectViewEnum::SUMMARY),
           "FULL" => Ok(ProjectViewEnum::FULL),
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


// region ScanViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which parts of the Scan should be returned in the response. Note, only the SUMMARY view (the default) is currently supported for ListScans.
pub enum ScanViewEnum {
    

    /// Not specified, equivalent to SUMMARY.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Server responses only include `name`, `details`, `start_time` and `end_time`. The default value. Note, the ListScans method may only use this view type, others view types are not supported.
    ///
    /// "SUMMARY"
    #[serde(rename="SUMMARY")]
    SUMMARY,
    

    /// Full representation of the scan is returned in the server response, including `data`.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ScanViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScanViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            ScanViewEnum::SUMMARY => "SUMMARY",
            ScanViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ScanViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(ScanViewEnum::VIEWUNSPECIFIED),
           "SUMMARY" => Ok(ScanViewEnum::SUMMARY),
           "FULL" => Ok(ScanViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScanViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


