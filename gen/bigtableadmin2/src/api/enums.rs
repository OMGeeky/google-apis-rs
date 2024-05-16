use super::*;



// region AppProfilePriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This field has been deprecated in favor of `standard_isolation.priority`. If you set this field, `standard_isolation.priority` will be set instead. The priority of requests sent using this app profile.
pub enum AppProfilePriorityEnum {
    

    /// Default value. Mapped to PRIORITY_HIGH (the legacy behavior) on creation.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    
    /// "PRIORITY_LOW"
    #[serde(rename="PRIORITY_LOW")]
    PRIORITYLOW,
    
    /// "PRIORITY_MEDIUM"
    #[serde(rename="PRIORITY_MEDIUM")]
    PRIORITYMEDIUM,
    
    /// "PRIORITY_HIGH"
    #[serde(rename="PRIORITY_HIGH")]
    PRIORITYHIGH,
}

impl AsRef<str> for AppProfilePriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppProfilePriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            AppProfilePriorityEnum::PRIORITYLOW => "PRIORITY_LOW",
            AppProfilePriorityEnum::PRIORITYMEDIUM => "PRIORITY_MEDIUM",
            AppProfilePriorityEnum::PRIORITYHIGH => "PRIORITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for AppProfilePriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(AppProfilePriorityEnum::PRIORITYUNSPECIFIED),
           "PRIORITY_LOW" => Ok(AppProfilePriorityEnum::PRIORITYLOW),
           "PRIORITY_MEDIUM" => Ok(AppProfilePriorityEnum::PRIORITYMEDIUM),
           "PRIORITY_HIGH" => Ok(AppProfilePriorityEnum::PRIORITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppProfilePriorityEnum {
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


// region ClusterDefaultStorageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden.
pub enum ClusterDefaultStorageTypeEnum {
    

    /// The user did not specify a storage type.
    ///
    /// "STORAGE_TYPE_UNSPECIFIED"
    #[serde(rename="STORAGE_TYPE_UNSPECIFIED")]
    STORAGETYPEUNSPECIFIED,
    

    /// Flash (SSD) storage should be used.
    ///
    /// "SSD"
    #[serde(rename="SSD")]
    SSD,
    

    /// Magnetic drive (HDD) storage should be used.
    ///
    /// "HDD"
    #[serde(rename="HDD")]
    HDD,
}

impl AsRef<str> for ClusterDefaultStorageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterDefaultStorageTypeEnum::STORAGETYPEUNSPECIFIED => "STORAGE_TYPE_UNSPECIFIED",
            ClusterDefaultStorageTypeEnum::SSD => "SSD",
            ClusterDefaultStorageTypeEnum::HDD => "HDD",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterDefaultStorageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_TYPE_UNSPECIFIED" => Ok(ClusterDefaultStorageTypeEnum::STORAGETYPEUNSPECIFIED),
           "SSD" => Ok(ClusterDefaultStorageTypeEnum::SSD),
           "HDD" => Ok(ClusterDefaultStorageTypeEnum::HDD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterDefaultStorageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the cluster.
pub enum ClusterStateEnum {
    

    /// The state of the cluster could not be determined.
    ///
    /// "STATE_NOT_KNOWN"
    #[serde(rename="STATE_NOT_KNOWN")]
    STATENOTKNOWN,
    

    /// The cluster has been successfully created and is ready to serve requests.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The cluster is currently being created, and may be destroyed if the creation process encounters an error. A cluster may not be able to serve requests while being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The cluster is currently being resized, and may revert to its previous node count if the process encounters an error. A cluster is still capable of serving requests while being resized, but may exhibit performance as if its number of allocated nodes is between the starting and requested states.
    ///
    /// "RESIZING"
    #[serde(rename="RESIZING")]
    RESIZING,
    

    /// The cluster has no backing nodes. The data (tables) still exist, but no operations can be performed on the cluster.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for ClusterStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterStateEnum::STATENOTKNOWN => "STATE_NOT_KNOWN",
            ClusterStateEnum::READY => "READY",
            ClusterStateEnum::CREATING => "CREATING",
            ClusterStateEnum::RESIZING => "RESIZING",
            ClusterStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_NOT_KNOWN" => Ok(ClusterStateEnum::STATENOTKNOWN),
           "READY" => Ok(ClusterStateEnum::READY),
           "CREATING" => Ok(ClusterStateEnum::CREATING),
           "RESIZING" => Ok(ClusterStateEnum::RESIZING),
           "DISABLED" => Ok(ClusterStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterStateReplicationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of replication for the table in this cluster.
pub enum ClusterStateReplicationStateEnum {
    

    /// The replication state of the table is unknown in this cluster.
    ///
    /// "STATE_NOT_KNOWN"
    #[serde(rename="STATE_NOT_KNOWN")]
    STATENOTKNOWN,
    

    /// The cluster was recently created, and the table must finish copying over pre-existing data from other clusters before it can begin receiving live replication updates and serving Data API requests.
    ///
    /// "INITIALIZING"
    #[serde(rename="INITIALIZING")]
    INITIALIZING,
    

    /// The table is temporarily unable to serve Data API requests from this cluster due to planned internal maintenance.
    ///
    /// "PLANNED_MAINTENANCE"
    #[serde(rename="PLANNED_MAINTENANCE")]
    PLANNEDMAINTENANCE,
    

    /// The table is temporarily unable to serve Data API requests from this cluster due to unplanned or emergency maintenance.
    ///
    /// "UNPLANNED_MAINTENANCE"
    #[serde(rename="UNPLANNED_MAINTENANCE")]
    UNPLANNEDMAINTENANCE,
    

    /// The table can serve Data API requests from this cluster. Depending on replication delay, reads may not immediately reflect the state of the table in other clusters.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The table is fully created and ready for use after a restore, and is being optimized for performance. When optimizations are complete, the table will transition to `READY` state.
    ///
    /// "READY_OPTIMIZING"
    #[serde(rename="READY_OPTIMIZING")]
    READYOPTIMIZING,
}

impl AsRef<str> for ClusterStateReplicationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterStateReplicationStateEnum::STATENOTKNOWN => "STATE_NOT_KNOWN",
            ClusterStateReplicationStateEnum::INITIALIZING => "INITIALIZING",
            ClusterStateReplicationStateEnum::PLANNEDMAINTENANCE => "PLANNED_MAINTENANCE",
            ClusterStateReplicationStateEnum::UNPLANNEDMAINTENANCE => "UNPLANNED_MAINTENANCE",
            ClusterStateReplicationStateEnum::READY => "READY",
            ClusterStateReplicationStateEnum::READYOPTIMIZING => "READY_OPTIMIZING",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterStateReplicationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_NOT_KNOWN" => Ok(ClusterStateReplicationStateEnum::STATENOTKNOWN),
           "INITIALIZING" => Ok(ClusterStateReplicationStateEnum::INITIALIZING),
           "PLANNED_MAINTENANCE" => Ok(ClusterStateReplicationStateEnum::PLANNEDMAINTENANCE),
           "UNPLANNED_MAINTENANCE" => Ok(ClusterStateReplicationStateEnum::UNPLANNEDMAINTENANCE),
           "READY" => Ok(ClusterStateReplicationStateEnum::READY),
           "READY_OPTIMIZING" => Ok(ClusterStateReplicationStateEnum::READYOPTIMIZING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterStateReplicationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataBoostIsolationReadOnlyComputeBillingOwnerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Compute Billing Owner for this Data Boost App Profile.
pub enum DataBoostIsolationReadOnlyComputeBillingOwnerEnum {
    

    /// Unspecified value.
    ///
    /// "COMPUTE_BILLING_OWNER_UNSPECIFIED"
    #[serde(rename="COMPUTE_BILLING_OWNER_UNSPECIFIED")]
    COMPUTEBILLINGOWNERUNSPECIFIED,
    

    /// The host Cloud Project containing the targeted Bigtable Instance / Table pays for compute.
    ///
    /// "HOST_PAYS"
    #[serde(rename="HOST_PAYS")]
    HOSTPAYS,
}

impl AsRef<str> for DataBoostIsolationReadOnlyComputeBillingOwnerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataBoostIsolationReadOnlyComputeBillingOwnerEnum::COMPUTEBILLINGOWNERUNSPECIFIED => "COMPUTE_BILLING_OWNER_UNSPECIFIED",
            DataBoostIsolationReadOnlyComputeBillingOwnerEnum::HOSTPAYS => "HOST_PAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for DataBoostIsolationReadOnlyComputeBillingOwnerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_BILLING_OWNER_UNSPECIFIED" => Ok(DataBoostIsolationReadOnlyComputeBillingOwnerEnum::COMPUTEBILLINGOWNERUNSPECIFIED),
           "HOST_PAYS" => Ok(DataBoostIsolationReadOnlyComputeBillingOwnerEnum::HOSTPAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataBoostIsolationReadOnlyComputeBillingOwnerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EncryptionInfoEncryptionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of encryption used to protect this resource.
pub enum EncryptionInfoEncryptionTypeEnum {
    

    /// Encryption type was not specified, though data at rest remains encrypted.
    ///
    /// "ENCRYPTION_TYPE_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_TYPE_UNSPECIFIED")]
    ENCRYPTIONTYPEUNSPECIFIED,
    

    /// The data backing this resource is encrypted at rest with a key that is fully managed by Google. No key version or status will be populated. This is the default state.
    ///
    /// "GOOGLE_DEFAULT_ENCRYPTION"
    #[serde(rename="GOOGLE_DEFAULT_ENCRYPTION")]
    GOOGLEDEFAULTENCRYPTION,
    

    /// The data backing this resource is encrypted at rest with a key that is managed by the customer. The in-use version of the key and its status are populated for CMEK-protected tables. CMEK-protected backups are pinned to the key version that was in use at the time the backup was taken. This key version is populated but its status is not tracked and is reported as `UNKNOWN`.
    ///
    /// "CUSTOMER_MANAGED_ENCRYPTION"
    #[serde(rename="CUSTOMER_MANAGED_ENCRYPTION")]
    CUSTOMERMANAGEDENCRYPTION,
}

impl AsRef<str> for EncryptionInfoEncryptionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EncryptionInfoEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED => "ENCRYPTION_TYPE_UNSPECIFIED",
            EncryptionInfoEncryptionTypeEnum::GOOGLEDEFAULTENCRYPTION => "GOOGLE_DEFAULT_ENCRYPTION",
            EncryptionInfoEncryptionTypeEnum::CUSTOMERMANAGEDENCRYPTION => "CUSTOMER_MANAGED_ENCRYPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for EncryptionInfoEncryptionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_TYPE_UNSPECIFIED" => Ok(EncryptionInfoEncryptionTypeEnum::ENCRYPTIONTYPEUNSPECIFIED),
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


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the instance.
pub enum InstanceStateEnum {
    

    /// The state of the instance could not be determined.
    ///
    /// "STATE_NOT_KNOWN"
    #[serde(rename="STATE_NOT_KNOWN")]
    STATENOTKNOWN,
    

    /// The instance has been successfully created and can serve requests to its tables.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The instance is currently being created, and may be destroyed if the creation process encounters an error.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATENOTKNOWN => "STATE_NOT_KNOWN",
            InstanceStateEnum::READY => "READY",
            InstanceStateEnum::CREATING => "CREATING",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_NOT_KNOWN" => Ok(InstanceStateEnum::STATENOTKNOWN),
           "READY" => Ok(InstanceStateEnum::READY),
           "CREATING" => Ok(InstanceStateEnum::CREATING),
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


// region InstanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the instance. Defaults to `PRODUCTION`.
pub enum InstanceTypeEnum {
    

    /// The type of the instance is unspecified. If set when creating an instance, a `PRODUCTION` instance will be created. If set when updating an instance, the type will be left unchanged.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// An instance meant for production use. `serve_nodes` must be set on the cluster.
    ///
    /// "PRODUCTION"
    #[serde(rename="PRODUCTION")]
    PRODUCTION,
    

    /// DEPRECATED: Prefer PRODUCTION for all use cases, as it no longer enforces a higher minimum node count than DEVELOPMENT.
    ///
    /// "DEVELOPMENT"
    #[serde(rename="DEVELOPMENT")]
    DEVELOPMENT,
}

impl AsRef<str> for InstanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            InstanceTypeEnum::PRODUCTION => "PRODUCTION",
            InstanceTypeEnum::DEVELOPMENT => "DEVELOPMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(InstanceTypeEnum::TYPEUNSPECIFIED),
           "PRODUCTION" => Ok(InstanceTypeEnum::PRODUCTION),
           "DEVELOPMENT" => Ok(InstanceTypeEnum::DEVELOPMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceTypeEnum {
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
    /// "RESTORE_SOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESTORE_SOURCE_TYPE_UNSPECIFIED")]
    RESTORESOURCETYPEUNSPECIFIED,
    

    /// A backup was used as the source of the restore.
    ///
    /// "BACKUP"
    #[serde(rename="BACKUP")]
    BACKUP,
}

impl AsRef<str> for RestoreInfoSourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreInfoSourceTypeEnum::RESTORESOURCETYPEUNSPECIFIED => "RESTORE_SOURCE_TYPE_UNSPECIFIED",
            RestoreInfoSourceTypeEnum::BACKUP => "BACKUP",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreInfoSourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTORE_SOURCE_TYPE_UNSPECIFIED" => Ok(RestoreInfoSourceTypeEnum::RESTORESOURCETYPEUNSPECIFIED),
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


// region StandardIsolationPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The priority of requests sent using this app profile.
pub enum StandardIsolationPriorityEnum {
    

    /// Default value. Mapped to PRIORITY_HIGH (the legacy behavior) on creation.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    
    /// "PRIORITY_LOW"
    #[serde(rename="PRIORITY_LOW")]
    PRIORITYLOW,
    
    /// "PRIORITY_MEDIUM"
    #[serde(rename="PRIORITY_MEDIUM")]
    PRIORITYMEDIUM,
    
    /// "PRIORITY_HIGH"
    #[serde(rename="PRIORITY_HIGH")]
    PRIORITYHIGH,
}

impl AsRef<str> for StandardIsolationPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StandardIsolationPriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            StandardIsolationPriorityEnum::PRIORITYLOW => "PRIORITY_LOW",
            StandardIsolationPriorityEnum::PRIORITYMEDIUM => "PRIORITY_MEDIUM",
            StandardIsolationPriorityEnum::PRIORITYHIGH => "PRIORITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for StandardIsolationPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(StandardIsolationPriorityEnum::PRIORITYUNSPECIFIED),
           "PRIORITY_LOW" => Ok(StandardIsolationPriorityEnum::PRIORITYLOW),
           "PRIORITY_MEDIUM" => Ok(StandardIsolationPriorityEnum::PRIORITYMEDIUM),
           "PRIORITY_HIGH" => Ok(StandardIsolationPriorityEnum::PRIORITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StandardIsolationPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TableGranularityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The granularity (i.e. `MILLIS`) at which timestamps are stored in this table. Timestamps not matching the granularity will be rejected. If unspecified at creation time, the value will be set to `MILLIS`. Views: `SCHEMA_VIEW`, `FULL`.
pub enum TableGranularityEnum {
    

    /// The user did not specify a granularity. Should not be returned. When specified during table creation, MILLIS will be used.
    ///
    /// "TIMESTAMP_GRANULARITY_UNSPECIFIED"
    #[serde(rename="TIMESTAMP_GRANULARITY_UNSPECIFIED")]
    TIMESTAMPGRANULARITYUNSPECIFIED,
    

    /// The table keeps data versioned at a granularity of 1ms.
    ///
    /// "MILLIS"
    #[serde(rename="MILLIS")]
    MILLIS,
}

impl AsRef<str> for TableGranularityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TableGranularityEnum::TIMESTAMPGRANULARITYUNSPECIFIED => "TIMESTAMP_GRANULARITY_UNSPECIFIED",
            TableGranularityEnum::MILLIS => "MILLIS",
        }
    }
}

impl std::convert::TryFrom< &str> for TableGranularityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIMESTAMP_GRANULARITY_UNSPECIFIED" => Ok(TableGranularityEnum::TIMESTAMPGRANULARITYUNSPECIFIED),
           "MILLIS" => Ok(TableGranularityEnum::MILLIS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TableGranularityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The view to be applied to the returned tables' fields. Only NAME_ONLY view (default), REPLICATION_VIEW and ENCRYPTION_VIEW are supported.
pub enum ProjectViewEnum {
    

    /// Uses the default view for each method as documented in its request.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Only populates `name`.
    ///
    /// "NAME_ONLY"
    #[serde(rename="NAME_ONLY")]
    NAMEONLY,
    

    /// Only populates `name` and fields related to the table's schema.
    ///
    /// "SCHEMA_VIEW"
    #[serde(rename="SCHEMA_VIEW")]
    SCHEMAVIEW,
    

    /// Only populates `name` and fields related to the table's replication state.
    ///
    /// "REPLICATION_VIEW"
    #[serde(rename="REPLICATION_VIEW")]
    REPLICATIONVIEW,
    

    /// Only populates `name` and fields related to the table's encryption state.
    ///
    /// "ENCRYPTION_VIEW"
    #[serde(rename="ENCRYPTION_VIEW")]
    ENCRYPTIONVIEW,
    

    /// Only populates `name` and fields related to the table's stats (e.g. TableStats and ColumnFamilyStats).
    ///
    /// "STATS_VIEW"
    #[serde(rename="STATS_VIEW")]
    STATSVIEW,
    

    /// Populates all fields except for stats. See STATS_VIEW to request stats.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            ProjectViewEnum::NAMEONLY => "NAME_ONLY",
            ProjectViewEnum::SCHEMAVIEW => "SCHEMA_VIEW",
            ProjectViewEnum::REPLICATIONVIEW => "REPLICATION_VIEW",
            ProjectViewEnum::ENCRYPTIONVIEW => "ENCRYPTION_VIEW",
            ProjectViewEnum::STATSVIEW => "STATS_VIEW",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::VIEWUNSPECIFIED),
           "NAME_ONLY" => Ok(ProjectViewEnum::NAMEONLY),
           "SCHEMA_VIEW" => Ok(ProjectViewEnum::SCHEMAVIEW),
           "REPLICATION_VIEW" => Ok(ProjectViewEnum::REPLICATIONVIEW),
           "ENCRYPTION_VIEW" => Ok(ProjectViewEnum::ENCRYPTIONVIEW),
           "STATS_VIEW" => Ok(ProjectViewEnum::STATSVIEW),
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


