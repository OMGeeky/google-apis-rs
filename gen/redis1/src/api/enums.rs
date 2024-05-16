use super::*;



// region AOFConfigAppendFsyncEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. fsync configuration.
pub enum AOFConfigAppendFsyncEnum {
    

    /// Not set. Default: EVERYSEC
    ///
    /// "APPEND_FSYNC_UNSPECIFIED"
    #[serde(rename="APPEND_FSYNC_UNSPECIFIED")]
    APPENDFSYNCUNSPECIFIED,
    

    /// Never fsync. Normally Linux will flush data every 30 seconds with this configuration, but it's up to the kernel's exact tuning.
    ///
    /// "NO"
    #[serde(rename="NO")]
    NO,
    

    /// fsync every second. Fast enough, and you may lose 1 second of data if there is a disaster
    ///
    /// "EVERYSEC"
    #[serde(rename="EVERYSEC")]
    EVERYSEC,
    

    /// fsync every time new commands are appended to the AOF. It has the best data loss protection at the cost of performance
    ///
    /// "ALWAYS"
    #[serde(rename="ALWAYS")]
    ALWAYS,
}

impl AsRef<str> for AOFConfigAppendFsyncEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AOFConfigAppendFsyncEnum::APPENDFSYNCUNSPECIFIED => "APPEND_FSYNC_UNSPECIFIED",
            AOFConfigAppendFsyncEnum::NO => "NO",
            AOFConfigAppendFsyncEnum::EVERYSEC => "EVERYSEC",
            AOFConfigAppendFsyncEnum::ALWAYS => "ALWAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for AOFConfigAppendFsyncEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPEND_FSYNC_UNSPECIFIED" => Ok(AOFConfigAppendFsyncEnum::APPENDFSYNCUNSPECIFIED),
           "NO" => Ok(AOFConfigAppendFsyncEnum::NO),
           "EVERYSEC" => Ok(AOFConfigAppendFsyncEnum::EVERYSEC),
           "ALWAYS" => Ok(AOFConfigAppendFsyncEnum::ALWAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AOFConfigAppendFsyncEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterAuthorizationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The authorization mode of the Redis cluster. If not provided, auth feature is disabled for the cluster.
pub enum ClusterAuthorizationModeEnum {
    

    /// Not set.
    ///
    /// "AUTH_MODE_UNSPECIFIED"
    #[serde(rename="AUTH_MODE_UNSPECIFIED")]
    AUTHMODEUNSPECIFIED,
    

    /// IAM basic authorization mode
    ///
    /// "AUTH_MODE_IAM_AUTH"
    #[serde(rename="AUTH_MODE_IAM_AUTH")]
    AUTHMODEIAMAUTH,
    

    /// Authorization disabled mode
    ///
    /// "AUTH_MODE_DISABLED"
    #[serde(rename="AUTH_MODE_DISABLED")]
    AUTHMODEDISABLED,
}

impl AsRef<str> for ClusterAuthorizationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterAuthorizationModeEnum::AUTHMODEUNSPECIFIED => "AUTH_MODE_UNSPECIFIED",
            ClusterAuthorizationModeEnum::AUTHMODEIAMAUTH => "AUTH_MODE_IAM_AUTH",
            ClusterAuthorizationModeEnum::AUTHMODEDISABLED => "AUTH_MODE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterAuthorizationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_MODE_UNSPECIFIED" => Ok(ClusterAuthorizationModeEnum::AUTHMODEUNSPECIFIED),
           "AUTH_MODE_IAM_AUTH" => Ok(ClusterAuthorizationModeEnum::AUTHMODEIAMAUTH),
           "AUTH_MODE_DISABLED" => Ok(ClusterAuthorizationModeEnum::AUTHMODEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterAuthorizationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterNodeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of a redis node in the cluster. NodeType determines the underlying machine-type of a redis node.
pub enum ClusterNodeTypeEnum {
    
    /// "NODE_TYPE_UNSPECIFIED"
    #[serde(rename="NODE_TYPE_UNSPECIFIED")]
    NODETYPEUNSPECIFIED,
    

    /// Redis shared core nano node_type.
    ///
    /// "REDIS_SHARED_CORE_NANO"
    #[serde(rename="REDIS_SHARED_CORE_NANO")]
    REDISSHAREDCORENANO,
    

    /// Redis highmem medium node_type.
    ///
    /// "REDIS_HIGHMEM_MEDIUM"
    #[serde(rename="REDIS_HIGHMEM_MEDIUM")]
    REDISHIGHMEMMEDIUM,
    

    /// Redis highmem xlarge node_type.
    ///
    /// "REDIS_HIGHMEM_XLARGE"
    #[serde(rename="REDIS_HIGHMEM_XLARGE")]
    REDISHIGHMEMXLARGE,
    

    /// Redis standard small node_type.
    ///
    /// "REDIS_STANDARD_SMALL"
    #[serde(rename="REDIS_STANDARD_SMALL")]
    REDISSTANDARDSMALL,
}

impl AsRef<str> for ClusterNodeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterNodeTypeEnum::NODETYPEUNSPECIFIED => "NODE_TYPE_UNSPECIFIED",
            ClusterNodeTypeEnum::REDISSHAREDCORENANO => "REDIS_SHARED_CORE_NANO",
            ClusterNodeTypeEnum::REDISHIGHMEMMEDIUM => "REDIS_HIGHMEM_MEDIUM",
            ClusterNodeTypeEnum::REDISHIGHMEMXLARGE => "REDIS_HIGHMEM_XLARGE",
            ClusterNodeTypeEnum::REDISSTANDARDSMALL => "REDIS_STANDARD_SMALL",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterNodeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NODE_TYPE_UNSPECIFIED" => Ok(ClusterNodeTypeEnum::NODETYPEUNSPECIFIED),
           "REDIS_SHARED_CORE_NANO" => Ok(ClusterNodeTypeEnum::REDISSHAREDCORENANO),
           "REDIS_HIGHMEM_MEDIUM" => Ok(ClusterNodeTypeEnum::REDISHIGHMEMMEDIUM),
           "REDIS_HIGHMEM_XLARGE" => Ok(ClusterNodeTypeEnum::REDISHIGHMEMXLARGE),
           "REDIS_STANDARD_SMALL" => Ok(ClusterNodeTypeEnum::REDISSTANDARDSMALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterNodeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this cluster. Can be CREATING, READY, UPDATING, DELETING and SUSPENDED
pub enum ClusterStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Redis cluster is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Redis cluster has been created and is fully usable.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Redis cluster configuration is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// Redis cluster is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for ClusterStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ClusterStateEnum::CREATING => "CREATING",
            ClusterStateEnum::ACTIVE => "ACTIVE",
            ClusterStateEnum::UPDATING => "UPDATING",
            ClusterStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ClusterStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(ClusterStateEnum::CREATING),
           "ACTIVE" => Ok(ClusterStateEnum::ACTIVE),
           "UPDATING" => Ok(ClusterStateEnum::UPDATING),
           "DELETING" => Ok(ClusterStateEnum::DELETING),
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


// region ClusterTransitEncryptionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The in-transit encryption for the Redis cluster. If not provided, encryption is disabled for the cluster.
pub enum ClusterTransitEncryptionModeEnum {
    

    /// In-transit encryption not set.
    ///
    /// "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED"
    #[serde(rename="TRANSIT_ENCRYPTION_MODE_UNSPECIFIED")]
    TRANSITENCRYPTIONMODEUNSPECIFIED,
    

    /// In-transit encryption disabled.
    ///
    /// "TRANSIT_ENCRYPTION_MODE_DISABLED"
    #[serde(rename="TRANSIT_ENCRYPTION_MODE_DISABLED")]
    TRANSITENCRYPTIONMODEDISABLED,
    

    /// Use server managed encryption for in-transit encryption.
    ///
    /// "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION"
    #[serde(rename="TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION")]
    TRANSITENCRYPTIONMODESERVERAUTHENTICATION,
}

impl AsRef<str> for ClusterTransitEncryptionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterTransitEncryptionModeEnum::TRANSITENCRYPTIONMODEUNSPECIFIED => "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED",
            ClusterTransitEncryptionModeEnum::TRANSITENCRYPTIONMODEDISABLED => "TRANSIT_ENCRYPTION_MODE_DISABLED",
            ClusterTransitEncryptionModeEnum::TRANSITENCRYPTIONMODESERVERAUTHENTICATION => "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterTransitEncryptionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED" => Ok(ClusterTransitEncryptionModeEnum::TRANSITENCRYPTIONMODEUNSPECIFIED),
           "TRANSIT_ENCRYPTION_MODE_DISABLED" => Ok(ClusterTransitEncryptionModeEnum::TRANSITENCRYPTIONMODEDISABLED),
           "TRANSIT_ENCRYPTION_MODE_SERVER_AUTHENTICATION" => Ok(ClusterTransitEncryptionModeEnum::TRANSITENCRYPTIONMODESERVERAUTHENTICATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterTransitEncryptionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterPersistenceConfigModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The mode of persistence.
pub enum ClusterPersistenceConfigModeEnum {
    

    /// Not set.
    ///
    /// "PERSISTENCE_MODE_UNSPECIFIED"
    #[serde(rename="PERSISTENCE_MODE_UNSPECIFIED")]
    PERSISTENCEMODEUNSPECIFIED,
    

    /// Persistence is disabled, and any snapshot data is deleted.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// RDB based persistence is enabled.
    ///
    /// "RDB"
    #[serde(rename="RDB")]
    RDB,
    

    /// AOF based persistence is enabled.
    ///
    /// "AOF"
    #[serde(rename="AOF")]
    AOF,
}

impl AsRef<str> for ClusterPersistenceConfigModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterPersistenceConfigModeEnum::PERSISTENCEMODEUNSPECIFIED => "PERSISTENCE_MODE_UNSPECIFIED",
            ClusterPersistenceConfigModeEnum::DISABLED => "DISABLED",
            ClusterPersistenceConfigModeEnum::RDB => "RDB",
            ClusterPersistenceConfigModeEnum::AOF => "AOF",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterPersistenceConfigModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERSISTENCE_MODE_UNSPECIFIED" => Ok(ClusterPersistenceConfigModeEnum::PERSISTENCEMODEUNSPECIFIED),
           "DISABLED" => Ok(ClusterPersistenceConfigModeEnum::DISABLED),
           "RDB" => Ok(ClusterPersistenceConfigModeEnum::RDB),
           "AOF" => Ok(ClusterPersistenceConfigModeEnum::AOF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterPersistenceConfigModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FailoverInstanceRequestDataProtectionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Available data protection modes that the user can choose. If it's unspecified, data protection mode will be LIMITED_DATA_LOSS by default.
pub enum FailoverInstanceRequestDataProtectionModeEnum {
    

    /// Defaults to LIMITED_DATA_LOSS if a data protection mode is not specified.
    ///
    /// "DATA_PROTECTION_MODE_UNSPECIFIED"
    #[serde(rename="DATA_PROTECTION_MODE_UNSPECIFIED")]
    DATAPROTECTIONMODEUNSPECIFIED,
    

    /// Instance failover will be protected with data loss control. More specifically, the failover will only be performed if the current replication offset diff between primary and replica is under a certain threshold.
    ///
    /// "LIMITED_DATA_LOSS"
    #[serde(rename="LIMITED_DATA_LOSS")]
    LIMITEDDATALOSS,
    

    /// Instance failover will be performed without data loss control.
    ///
    /// "FORCE_DATA_LOSS"
    #[serde(rename="FORCE_DATA_LOSS")]
    FORCEDATALOSS,
}

impl AsRef<str> for FailoverInstanceRequestDataProtectionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FailoverInstanceRequestDataProtectionModeEnum::DATAPROTECTIONMODEUNSPECIFIED => "DATA_PROTECTION_MODE_UNSPECIFIED",
            FailoverInstanceRequestDataProtectionModeEnum::LIMITEDDATALOSS => "LIMITED_DATA_LOSS",
            FailoverInstanceRequestDataProtectionModeEnum::FORCEDATALOSS => "FORCE_DATA_LOSS",
        }
    }
}

impl std::convert::TryFrom< &str> for FailoverInstanceRequestDataProtectionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_PROTECTION_MODE_UNSPECIFIED" => Ok(FailoverInstanceRequestDataProtectionModeEnum::DATAPROTECTIONMODEUNSPECIFIED),
           "LIMITED_DATA_LOSS" => Ok(FailoverInstanceRequestDataProtectionModeEnum::LIMITEDDATALOSS),
           "FORCE_DATA_LOSS" => Ok(FailoverInstanceRequestDataProtectionModeEnum::FORCEDATALOSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FailoverInstanceRequestDataProtectionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceConnectModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING.
pub enum InstanceConnectModeEnum {
    

    /// Not set.
    ///
    /// "CONNECT_MODE_UNSPECIFIED"
    #[serde(rename="CONNECT_MODE_UNSPECIFIED")]
    CONNECTMODEUNSPECIFIED,
    

    /// Connect via direct peering to the Memorystore for Redis hosted service.
    ///
    /// "DIRECT_PEERING"
    #[serde(rename="DIRECT_PEERING")]
    DIRECTPEERING,
    

    /// Connect your Memorystore for Redis instance using Private Service Access. Private services access provides an IP address range for multiple Google Cloud services, including Memorystore.
    ///
    /// "PRIVATE_SERVICE_ACCESS"
    #[serde(rename="PRIVATE_SERVICE_ACCESS")]
    PRIVATESERVICEACCESS,
}

impl AsRef<str> for InstanceConnectModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceConnectModeEnum::CONNECTMODEUNSPECIFIED => "CONNECT_MODE_UNSPECIFIED",
            InstanceConnectModeEnum::DIRECTPEERING => "DIRECT_PEERING",
            InstanceConnectModeEnum::PRIVATESERVICEACCESS => "PRIVATE_SERVICE_ACCESS",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceConnectModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECT_MODE_UNSPECIFIED" => Ok(InstanceConnectModeEnum::CONNECTMODEUNSPECIFIED),
           "DIRECT_PEERING" => Ok(InstanceConnectModeEnum::DIRECTPEERING),
           "PRIVATE_SERVICE_ACCESS" => Ok(InstanceConnectModeEnum::PRIVATESERVICEACCESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceConnectModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceReadReplicasModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Read replicas mode for the instance. Defaults to READ_REPLICAS_DISABLED.
pub enum InstanceReadReplicasModeEnum {
    

    /// If not set, Memorystore Redis backend will default to READ_REPLICAS_DISABLED.
    ///
    /// "READ_REPLICAS_MODE_UNSPECIFIED"
    #[serde(rename="READ_REPLICAS_MODE_UNSPECIFIED")]
    READREPLICASMODEUNSPECIFIED,
    

    /// If disabled, read endpoint will not be provided and the instance cannot scale up or down the number of replicas.
    ///
    /// "READ_REPLICAS_DISABLED"
    #[serde(rename="READ_REPLICAS_DISABLED")]
    READREPLICASDISABLED,
    

    /// If enabled, read endpoint will be provided and the instance can scale up and down the number of replicas. Not valid for basic tier.
    ///
    /// "READ_REPLICAS_ENABLED"
    #[serde(rename="READ_REPLICAS_ENABLED")]
    READREPLICASENABLED,
}

impl AsRef<str> for InstanceReadReplicasModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceReadReplicasModeEnum::READREPLICASMODEUNSPECIFIED => "READ_REPLICAS_MODE_UNSPECIFIED",
            InstanceReadReplicasModeEnum::READREPLICASDISABLED => "READ_REPLICAS_DISABLED",
            InstanceReadReplicasModeEnum::READREPLICASENABLED => "READ_REPLICAS_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceReadReplicasModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_REPLICAS_MODE_UNSPECIFIED" => Ok(InstanceReadReplicasModeEnum::READREPLICASMODEUNSPECIFIED),
           "READ_REPLICAS_DISABLED" => Ok(InstanceReadReplicasModeEnum::READREPLICASDISABLED),
           "READ_REPLICAS_ENABLED" => Ok(InstanceReadReplicasModeEnum::READREPLICASENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceReadReplicasModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this instance.
pub enum InstanceStateEnum {
    

    /// Not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Redis instance is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Redis instance has been created and is fully usable.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Redis instance configuration is being updated. Certain kinds of updates may cause the instance to become unusable while the update is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// Redis instance is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Redis instance is being repaired and may be unusable.
    ///
    /// "REPAIRING"
    #[serde(rename="REPAIRING")]
    REPAIRING,
    

    /// Maintenance is being performed on this Redis instance.
    ///
    /// "MAINTENANCE"
    #[serde(rename="MAINTENANCE")]
    MAINTENANCE,
    

    /// Redis instance is importing data (availability may be affected).
    ///
    /// "IMPORTING"
    #[serde(rename="IMPORTING")]
    IMPORTING,
    

    /// Redis instance is failing over (availability may be affected).
    ///
    /// "FAILING_OVER"
    #[serde(rename="FAILING_OVER")]
    FAILINGOVER,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::CREATING => "CREATING",
            InstanceStateEnum::READY => "READY",
            InstanceStateEnum::UPDATING => "UPDATING",
            InstanceStateEnum::DELETING => "DELETING",
            InstanceStateEnum::REPAIRING => "REPAIRING",
            InstanceStateEnum::MAINTENANCE => "MAINTENANCE",
            InstanceStateEnum::IMPORTING => "IMPORTING",
            InstanceStateEnum::FAILINGOVER => "FAILING_OVER",
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
           "UPDATING" => Ok(InstanceStateEnum::UPDATING),
           "DELETING" => Ok(InstanceStateEnum::DELETING),
           "REPAIRING" => Ok(InstanceStateEnum::REPAIRING),
           "MAINTENANCE" => Ok(InstanceStateEnum::MAINTENANCE),
           "IMPORTING" => Ok(InstanceStateEnum::IMPORTING),
           "FAILING_OVER" => Ok(InstanceStateEnum::FAILINGOVER),
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


// region InstanceSuspensionReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. reasons that causes instance in "SUSPENDED" state.
pub enum InstanceSuspensionReasonsEnum {
    

    /// Not set.
    ///
    /// "SUSPENSION_REASON_UNSPECIFIED"
    #[serde(rename="SUSPENSION_REASON_UNSPECIFIED")]
    SUSPENSIONREASONUNSPECIFIED,
    

    /// Something wrong with the CMEK key provided by customer.
    ///
    /// "CUSTOMER_MANAGED_KEY_ISSUE"
    #[serde(rename="CUSTOMER_MANAGED_KEY_ISSUE")]
    CUSTOMERMANAGEDKEYISSUE,
}

impl AsRef<str> for InstanceSuspensionReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceSuspensionReasonsEnum::SUSPENSIONREASONUNSPECIFIED => "SUSPENSION_REASON_UNSPECIFIED",
            InstanceSuspensionReasonsEnum::CUSTOMERMANAGEDKEYISSUE => "CUSTOMER_MANAGED_KEY_ISSUE",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceSuspensionReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUSPENSION_REASON_UNSPECIFIED" => Ok(InstanceSuspensionReasonsEnum::SUSPENSIONREASONUNSPECIFIED),
           "CUSTOMER_MANAGED_KEY_ISSUE" => Ok(InstanceSuspensionReasonsEnum::CUSTOMERMANAGEDKEYISSUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceSuspensionReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The service tier of the instance.
pub enum InstanceTierEnum {
    

    /// Not set.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// BASIC tier: standalone instance
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// STANDARD_HA tier: highly available primary/replica instances
    ///
    /// "STANDARD_HA"
    #[serde(rename="STANDARD_HA")]
    STANDARDHA,
}

impl AsRef<str> for InstanceTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            InstanceTierEnum::BASIC => "BASIC",
            InstanceTierEnum::STANDARDHA => "STANDARD_HA",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(InstanceTierEnum::TIERUNSPECIFIED),
           "BASIC" => Ok(InstanceTierEnum::BASIC),
           "STANDARD_HA" => Ok(InstanceTierEnum::STANDARDHA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceTransitEncryptionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance.
pub enum InstanceTransitEncryptionModeEnum {
    

    /// Not set.
    ///
    /// "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED"
    #[serde(rename="TRANSIT_ENCRYPTION_MODE_UNSPECIFIED")]
    TRANSITENCRYPTIONMODEUNSPECIFIED,
    

    /// Client to Server traffic encryption enabled with server authentication.
    ///
    /// "SERVER_AUTHENTICATION"
    #[serde(rename="SERVER_AUTHENTICATION")]
    SERVERAUTHENTICATION,
    

    /// TLS is disabled for the instance.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for InstanceTransitEncryptionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceTransitEncryptionModeEnum::TRANSITENCRYPTIONMODEUNSPECIFIED => "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED",
            InstanceTransitEncryptionModeEnum::SERVERAUTHENTICATION => "SERVER_AUTHENTICATION",
            InstanceTransitEncryptionModeEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceTransitEncryptionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED" => Ok(InstanceTransitEncryptionModeEnum::TRANSITENCRYPTIONMODEUNSPECIFIED),
           "SERVER_AUTHENTICATION" => Ok(InstanceTransitEncryptionModeEnum::SERVERAUTHENTICATION),
           "DISABLED" => Ok(InstanceTransitEncryptionModeEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceTransitEncryptionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersistenceConfigPersistenceModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.
pub enum PersistenceConfigPersistenceModeEnum {
    

    /// Not set.
    ///
    /// "PERSISTENCE_MODE_UNSPECIFIED"
    #[serde(rename="PERSISTENCE_MODE_UNSPECIFIED")]
    PERSISTENCEMODEUNSPECIFIED,
    

    /// Persistence is disabled for the instance, and any existing snapshots are deleted.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// RDB based Persistence is enabled.
    ///
    /// "RDB"
    #[serde(rename="RDB")]
    RDB,
}

impl AsRef<str> for PersistenceConfigPersistenceModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersistenceConfigPersistenceModeEnum::PERSISTENCEMODEUNSPECIFIED => "PERSISTENCE_MODE_UNSPECIFIED",
            PersistenceConfigPersistenceModeEnum::DISABLED => "DISABLED",
            PersistenceConfigPersistenceModeEnum::RDB => "RDB",
        }
    }
}

impl std::convert::TryFrom< &str> for PersistenceConfigPersistenceModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERSISTENCE_MODE_UNSPECIFIED" => Ok(PersistenceConfigPersistenceModeEnum::PERSISTENCEMODEUNSPECIFIED),
           "DISABLED" => Ok(PersistenceConfigPersistenceModeEnum::DISABLED),
           "RDB" => Ok(PersistenceConfigPersistenceModeEnum::RDB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersistenceConfigPersistenceModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersistenceConfigRdbSnapshotPeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Period between RDB snapshots. Snapshots will be attempted every period starting from the provided snapshot start time. For example, a start time of 01/01/2033 06:45 and SIX_HOURS snapshot period will do nothing until 01/01/2033, and then trigger snapshots every day at 06:45, 12:45, 18:45, and 00:45 the next day, and so on. If not provided, TWENTY_FOUR_HOURS will be used as default.
pub enum PersistenceConfigRdbSnapshotPeriodEnum {
    

    /// Not set.
    ///
    /// "SNAPSHOT_PERIOD_UNSPECIFIED"
    #[serde(rename="SNAPSHOT_PERIOD_UNSPECIFIED")]
    SNAPSHOTPERIODUNSPECIFIED,
    

    /// Snapshot every 1 hour.
    ///
    /// "ONE_HOUR"
    #[serde(rename="ONE_HOUR")]
    ONEHOUR,
    

    /// Snapshot every 6 hours.
    ///
    /// "SIX_HOURS"
    #[serde(rename="SIX_HOURS")]
    SIXHOURS,
    

    /// Snapshot every 12 hours.
    ///
    /// "TWELVE_HOURS"
    #[serde(rename="TWELVE_HOURS")]
    TWELVEHOURS,
    

    /// Snapshot every 24 hours.
    ///
    /// "TWENTY_FOUR_HOURS"
    #[serde(rename="TWENTY_FOUR_HOURS")]
    TWENTYFOURHOURS,
}

impl AsRef<str> for PersistenceConfigRdbSnapshotPeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersistenceConfigRdbSnapshotPeriodEnum::SNAPSHOTPERIODUNSPECIFIED => "SNAPSHOT_PERIOD_UNSPECIFIED",
            PersistenceConfigRdbSnapshotPeriodEnum::ONEHOUR => "ONE_HOUR",
            PersistenceConfigRdbSnapshotPeriodEnum::SIXHOURS => "SIX_HOURS",
            PersistenceConfigRdbSnapshotPeriodEnum::TWELVEHOURS => "TWELVE_HOURS",
            PersistenceConfigRdbSnapshotPeriodEnum::TWENTYFOURHOURS => "TWENTY_FOUR_HOURS",
        }
    }
}

impl std::convert::TryFrom< &str> for PersistenceConfigRdbSnapshotPeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SNAPSHOT_PERIOD_UNSPECIFIED" => Ok(PersistenceConfigRdbSnapshotPeriodEnum::SNAPSHOTPERIODUNSPECIFIED),
           "ONE_HOUR" => Ok(PersistenceConfigRdbSnapshotPeriodEnum::ONEHOUR),
           "SIX_HOURS" => Ok(PersistenceConfigRdbSnapshotPeriodEnum::SIXHOURS),
           "TWELVE_HOURS" => Ok(PersistenceConfigRdbSnapshotPeriodEnum::TWELVEHOURS),
           "TWENTY_FOUR_HOURS" => Ok(PersistenceConfigRdbSnapshotPeriodEnum::TWENTYFOURHOURS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersistenceConfigRdbSnapshotPeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RDBConfigRdbSnapshotPeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Period between RDB snapshots.
pub enum RDBConfigRdbSnapshotPeriodEnum {
    

    /// Not set.
    ///
    /// "SNAPSHOT_PERIOD_UNSPECIFIED"
    #[serde(rename="SNAPSHOT_PERIOD_UNSPECIFIED")]
    SNAPSHOTPERIODUNSPECIFIED,
    

    /// One hour.
    ///
    /// "ONE_HOUR"
    #[serde(rename="ONE_HOUR")]
    ONEHOUR,
    

    /// Six hours.
    ///
    /// "SIX_HOURS"
    #[serde(rename="SIX_HOURS")]
    SIXHOURS,
    

    /// Twelve hours.
    ///
    /// "TWELVE_HOURS"
    #[serde(rename="TWELVE_HOURS")]
    TWELVEHOURS,
    

    /// Twenty four hours.
    ///
    /// "TWENTY_FOUR_HOURS"
    #[serde(rename="TWENTY_FOUR_HOURS")]
    TWENTYFOURHOURS,
}

impl AsRef<str> for RDBConfigRdbSnapshotPeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RDBConfigRdbSnapshotPeriodEnum::SNAPSHOTPERIODUNSPECIFIED => "SNAPSHOT_PERIOD_UNSPECIFIED",
            RDBConfigRdbSnapshotPeriodEnum::ONEHOUR => "ONE_HOUR",
            RDBConfigRdbSnapshotPeriodEnum::SIXHOURS => "SIX_HOURS",
            RDBConfigRdbSnapshotPeriodEnum::TWELVEHOURS => "TWELVE_HOURS",
            RDBConfigRdbSnapshotPeriodEnum::TWENTYFOURHOURS => "TWENTY_FOUR_HOURS",
        }
    }
}

impl std::convert::TryFrom< &str> for RDBConfigRdbSnapshotPeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SNAPSHOT_PERIOD_UNSPECIFIED" => Ok(RDBConfigRdbSnapshotPeriodEnum::SNAPSHOTPERIODUNSPECIFIED),
           "ONE_HOUR" => Ok(RDBConfigRdbSnapshotPeriodEnum::ONEHOUR),
           "SIX_HOURS" => Ok(RDBConfigRdbSnapshotPeriodEnum::SIXHOURS),
           "TWELVE_HOURS" => Ok(RDBConfigRdbSnapshotPeriodEnum::TWELVEHOURS),
           "TWENTY_FOUR_HOURS" => Ok(RDBConfigRdbSnapshotPeriodEnum::TWENTYFOURHOURS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RDBConfigRdbSnapshotPeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RescheduleMaintenanceRequestRescheduleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. If reschedule type is SPECIFIC_TIME, must set up schedule_time as well.
pub enum RescheduleMaintenanceRequestRescheduleTypeEnum {
    

    /// Not set.
    ///
    /// "RESCHEDULE_TYPE_UNSPECIFIED"
    #[serde(rename="RESCHEDULE_TYPE_UNSPECIFIED")]
    RESCHEDULETYPEUNSPECIFIED,
    

    /// If the user wants to schedule the maintenance to happen now.
    ///
    /// "IMMEDIATE"
    #[serde(rename="IMMEDIATE")]
    IMMEDIATE,
    

    /// If the user wants to use the existing maintenance policy to find the next available window.
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

impl AsRef<str> for RescheduleMaintenanceRequestRescheduleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RescheduleMaintenanceRequestRescheduleTypeEnum::RESCHEDULETYPEUNSPECIFIED => "RESCHEDULE_TYPE_UNSPECIFIED",
            RescheduleMaintenanceRequestRescheduleTypeEnum::IMMEDIATE => "IMMEDIATE",
            RescheduleMaintenanceRequestRescheduleTypeEnum::NEXTAVAILABLEWINDOW => "NEXT_AVAILABLE_WINDOW",
            RescheduleMaintenanceRequestRescheduleTypeEnum::SPECIFICTIME => "SPECIFIC_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for RescheduleMaintenanceRequestRescheduleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESCHEDULE_TYPE_UNSPECIFIED" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::RESCHEDULETYPEUNSPECIFIED),
           "IMMEDIATE" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::IMMEDIATE),
           "NEXT_AVAILABLE_WINDOW" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::NEXTAVAILABLEWINDOW),
           "SPECIFIC_TIME" => Ok(RescheduleMaintenanceRequestRescheduleTypeEnum::SPECIFICTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RescheduleMaintenanceRequestRescheduleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WeeklyMaintenanceWindowDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The day of week that maintenance updates occur.
pub enum WeeklyMaintenanceWindowDayEnum {
    

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

impl AsRef<str> for WeeklyMaintenanceWindowDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WeeklyMaintenanceWindowDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            WeeklyMaintenanceWindowDayEnum::MONDAY => "MONDAY",
            WeeklyMaintenanceWindowDayEnum::TUESDAY => "TUESDAY",
            WeeklyMaintenanceWindowDayEnum::WEDNESDAY => "WEDNESDAY",
            WeeklyMaintenanceWindowDayEnum::THURSDAY => "THURSDAY",
            WeeklyMaintenanceWindowDayEnum::FRIDAY => "FRIDAY",
            WeeklyMaintenanceWindowDayEnum::SATURDAY => "SATURDAY",
            WeeklyMaintenanceWindowDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for WeeklyMaintenanceWindowDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(WeeklyMaintenanceWindowDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(WeeklyMaintenanceWindowDayEnum::MONDAY),
           "TUESDAY" => Ok(WeeklyMaintenanceWindowDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(WeeklyMaintenanceWindowDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(WeeklyMaintenanceWindowDayEnum::THURSDAY),
           "FRIDAY" => Ok(WeeklyMaintenanceWindowDayEnum::FRIDAY),
           "SATURDAY" => Ok(WeeklyMaintenanceWindowDayEnum::SATURDAY),
           "SUNDAY" => Ok(WeeklyMaintenanceWindowDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WeeklyMaintenanceWindowDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


