use super::*;



// region BackupSourceInstanceTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The service tier of the source Filestore instance that this backup is created from.
pub enum BackupSourceInstanceTierEnum {
    

    /// Not set.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// STANDARD tier. BASIC_HDD is the preferred term for this tier.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// PREMIUM tier. BASIC_SSD is the preferred term for this tier.
    ///
    /// "PREMIUM"
    #[serde(rename="PREMIUM")]
    PREMIUM,
    

    /// BASIC instances offer a maximum capacity of 63.9 TB. BASIC_HDD is an alias for STANDARD Tier, offering economical performance backed by HDD.
    ///
    /// "BASIC_HDD"
    #[serde(rename="BASIC_HDD")]
    BASICHDD,
    

    /// BASIC instances offer a maximum capacity of 63.9 TB. BASIC_SSD is an alias for PREMIUM Tier, and offers improved performance backed by SSD.
    ///
    /// "BASIC_SSD"
    #[serde(rename="BASIC_SSD")]
    BASICSSD,
    

    /// HIGH_SCALE instances offer expanded capacity and performance scaling capabilities.
    ///
    /// "HIGH_SCALE_SSD"
    #[serde(rename="HIGH_SCALE_SSD")]
    HIGHSCALESSD,
    

    /// ENTERPRISE instances offer the features and availability needed for mission-critical workloads.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// ZONAL instances offer expanded capacity and performance scaling capabilities.
    ///
    /// "ZONAL"
    #[serde(rename="ZONAL")]
    ZONAL,
    

    /// REGIONAL instances offer the features and availability needed for mission-critical workloads.
    ///
    /// "REGIONAL"
    #[serde(rename="REGIONAL")]
    REGIONAL,
}

impl AsRef<str> for BackupSourceInstanceTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupSourceInstanceTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            BackupSourceInstanceTierEnum::STANDARD => "STANDARD",
            BackupSourceInstanceTierEnum::PREMIUM => "PREMIUM",
            BackupSourceInstanceTierEnum::BASICHDD => "BASIC_HDD",
            BackupSourceInstanceTierEnum::BASICSSD => "BASIC_SSD",
            BackupSourceInstanceTierEnum::HIGHSCALESSD => "HIGH_SCALE_SSD",
            BackupSourceInstanceTierEnum::ENTERPRISE => "ENTERPRISE",
            BackupSourceInstanceTierEnum::ZONAL => "ZONAL",
            BackupSourceInstanceTierEnum::REGIONAL => "REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupSourceInstanceTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(BackupSourceInstanceTierEnum::TIERUNSPECIFIED),
           "STANDARD" => Ok(BackupSourceInstanceTierEnum::STANDARD),
           "PREMIUM" => Ok(BackupSourceInstanceTierEnum::PREMIUM),
           "BASIC_HDD" => Ok(BackupSourceInstanceTierEnum::BASICHDD),
           "BASIC_SSD" => Ok(BackupSourceInstanceTierEnum::BASICSSD),
           "HIGH_SCALE_SSD" => Ok(BackupSourceInstanceTierEnum::HIGHSCALESSD),
           "ENTERPRISE" => Ok(BackupSourceInstanceTierEnum::ENTERPRISE),
           "ZONAL" => Ok(BackupSourceInstanceTierEnum::ZONAL),
           "REGIONAL" => Ok(BackupSourceInstanceTierEnum::REGIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BackupSourceInstanceTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BackupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The backup state.
pub enum BackupStateEnum {
    

    /// State not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Backup is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Backup has been taken and the operation is being finalized. At this point, changes to the file share will not be reflected in the backup.
    ///
    /// "FINALIZING"
    #[serde(rename="FINALIZING")]
    FINALIZING,
    

    /// Backup is available for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Backup is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Backup is not valid and cannot be used for creating new instances or restoring existing instances.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for BackupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BackupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BackupStateEnum::CREATING => "CREATING",
            BackupStateEnum::FINALIZING => "FINALIZING",
            BackupStateEnum::READY => "READY",
            BackupStateEnum::DELETING => "DELETING",
            BackupStateEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for BackupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BackupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(BackupStateEnum::CREATING),
           "FINALIZING" => Ok(BackupStateEnum::FINALIZING),
           "READY" => Ok(BackupStateEnum::READY),
           "DELETING" => Ok(BackupStateEnum::DELETING),
           "INVALID" => Ok(BackupStateEnum::INVALID),
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


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The instance state.
pub enum InstanceStateEnum {
    

    /// State not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The instance is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The instance is available for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Work is being done on the instance. You can get further details from the `statusMessage` field of the `Instance` resource.
    ///
    /// "REPAIRING"
    #[serde(rename="REPAIRING")]
    REPAIRING,
    

    /// The instance is shutting down.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The instance is experiencing an issue and might be unusable. You can get further details from the `statusMessage` field of the `Instance` resource.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The instance is restoring a backup to an existing file share and may be unusable during this time.
    ///
    /// "RESTORING"
    #[serde(rename="RESTORING")]
    RESTORING,
    

    /// The instance is suspended. You can get further details from the `suspension_reasons` field of the `Instance` resource.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// The instance is in the process of becoming suspended.
    ///
    /// "SUSPENDING"
    #[serde(rename="SUSPENDING")]
    SUSPENDING,
    

    /// The instance is in the process of becoming active.
    ///
    /// "RESUMING"
    #[serde(rename="RESUMING")]
    RESUMING,
    

    /// The instance is reverting to a snapshot.
    ///
    /// "REVERTING"
    #[serde(rename="REVERTING")]
    REVERTING,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::CREATING => "CREATING",
            InstanceStateEnum::READY => "READY",
            InstanceStateEnum::REPAIRING => "REPAIRING",
            InstanceStateEnum::DELETING => "DELETING",
            InstanceStateEnum::ERROR => "ERROR",
            InstanceStateEnum::RESTORING => "RESTORING",
            InstanceStateEnum::SUSPENDED => "SUSPENDED",
            InstanceStateEnum::SUSPENDING => "SUSPENDING",
            InstanceStateEnum::RESUMING => "RESUMING",
            InstanceStateEnum::REVERTING => "REVERTING",
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
           "REPAIRING" => Ok(InstanceStateEnum::REPAIRING),
           "DELETING" => Ok(InstanceStateEnum::DELETING),
           "ERROR" => Ok(InstanceStateEnum::ERROR),
           "RESTORING" => Ok(InstanceStateEnum::RESTORING),
           "SUSPENDED" => Ok(InstanceStateEnum::SUSPENDED),
           "SUSPENDING" => Ok(InstanceStateEnum::SUSPENDING),
           "RESUMING" => Ok(InstanceStateEnum::RESUMING),
           "REVERTING" => Ok(InstanceStateEnum::REVERTING),
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
/// Output only. Field indicates all the reasons the instance is in "SUSPENDED" state.
pub enum InstanceSuspensionReasonsEnum {
    

    /// Not set.
    ///
    /// "SUSPENSION_REASON_UNSPECIFIED"
    #[serde(rename="SUSPENSION_REASON_UNSPECIFIED")]
    SUSPENSIONREASONUNSPECIFIED,
    

    /// The KMS key used by the instance is either revoked or denied access to.
    ///
    /// "KMS_KEY_ISSUE"
    #[serde(rename="KMS_KEY_ISSUE")]
    KMSKEYISSUE,
}

impl AsRef<str> for InstanceSuspensionReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceSuspensionReasonsEnum::SUSPENSIONREASONUNSPECIFIED => "SUSPENSION_REASON_UNSPECIFIED",
            InstanceSuspensionReasonsEnum::KMSKEYISSUE => "KMS_KEY_ISSUE",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceSuspensionReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUSPENSION_REASON_UNSPECIFIED" => Ok(InstanceSuspensionReasonsEnum::SUSPENSIONREASONUNSPECIFIED),
           "KMS_KEY_ISSUE" => Ok(InstanceSuspensionReasonsEnum::KMSKEYISSUE),
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
/// The service tier of the instance.
pub enum InstanceTierEnum {
    

    /// Not set.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// STANDARD tier. BASIC_HDD is the preferred term for this tier.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// PREMIUM tier. BASIC_SSD is the preferred term for this tier.
    ///
    /// "PREMIUM"
    #[serde(rename="PREMIUM")]
    PREMIUM,
    

    /// BASIC instances offer a maximum capacity of 63.9 TB. BASIC_HDD is an alias for STANDARD Tier, offering economical performance backed by HDD.
    ///
    /// "BASIC_HDD"
    #[serde(rename="BASIC_HDD")]
    BASICHDD,
    

    /// BASIC instances offer a maximum capacity of 63.9 TB. BASIC_SSD is an alias for PREMIUM Tier, and offers improved performance backed by SSD.
    ///
    /// "BASIC_SSD"
    #[serde(rename="BASIC_SSD")]
    BASICSSD,
    

    /// HIGH_SCALE instances offer expanded capacity and performance scaling capabilities.
    ///
    /// "HIGH_SCALE_SSD"
    #[serde(rename="HIGH_SCALE_SSD")]
    HIGHSCALESSD,
    

    /// ENTERPRISE instances offer the features and availability needed for mission-critical workloads.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// ZONAL instances offer expanded capacity and performance scaling capabilities.
    ///
    /// "ZONAL"
    #[serde(rename="ZONAL")]
    ZONAL,
    

    /// REGIONAL instances offer the features and availability needed for mission-critical workloads.
    ///
    /// "REGIONAL"
    #[serde(rename="REGIONAL")]
    REGIONAL,
}

impl AsRef<str> for InstanceTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            InstanceTierEnum::STANDARD => "STANDARD",
            InstanceTierEnum::PREMIUM => "PREMIUM",
            InstanceTierEnum::BASICHDD => "BASIC_HDD",
            InstanceTierEnum::BASICSSD => "BASIC_SSD",
            InstanceTierEnum::HIGHSCALESSD => "HIGH_SCALE_SSD",
            InstanceTierEnum::ENTERPRISE => "ENTERPRISE",
            InstanceTierEnum::ZONAL => "ZONAL",
            InstanceTierEnum::REGIONAL => "REGIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(InstanceTierEnum::TIERUNSPECIFIED),
           "STANDARD" => Ok(InstanceTierEnum::STANDARD),
           "PREMIUM" => Ok(InstanceTierEnum::PREMIUM),
           "BASIC_HDD" => Ok(InstanceTierEnum::BASICHDD),
           "BASIC_SSD" => Ok(InstanceTierEnum::BASICSSD),
           "HIGH_SCALE_SSD" => Ok(InstanceTierEnum::HIGHSCALESSD),
           "ENTERPRISE" => Ok(InstanceTierEnum::ENTERPRISE),
           "ZONAL" => Ok(InstanceTierEnum::ZONAL),
           "REGIONAL" => Ok(InstanceTierEnum::REGIONAL),
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


// region NetworkConfigConnectModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The network connect mode of the Filestore instance. If not provided, the connect mode defaults to DIRECT_PEERING.
pub enum NetworkConfigConnectModeEnum {
    

    /// Not set.
    ///
    /// "CONNECT_MODE_UNSPECIFIED"
    #[serde(rename="CONNECT_MODE_UNSPECIFIED")]
    CONNECTMODEUNSPECIFIED,
    

    /// Connect via direct peering to the Filestore service.
    ///
    /// "DIRECT_PEERING"
    #[serde(rename="DIRECT_PEERING")]
    DIRECTPEERING,
    

    /// Connect to your Filestore instance using Private Service Access. Private services access provides an IP address range for multiple Google Cloud services, including Filestore.
    ///
    /// "PRIVATE_SERVICE_ACCESS"
    #[serde(rename="PRIVATE_SERVICE_ACCESS")]
    PRIVATESERVICEACCESS,
}

impl AsRef<str> for NetworkConfigConnectModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigConnectModeEnum::CONNECTMODEUNSPECIFIED => "CONNECT_MODE_UNSPECIFIED",
            NetworkConfigConnectModeEnum::DIRECTPEERING => "DIRECT_PEERING",
            NetworkConfigConnectModeEnum::PRIVATESERVICEACCESS => "PRIVATE_SERVICE_ACCESS",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigConnectModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECT_MODE_UNSPECIFIED" => Ok(NetworkConfigConnectModeEnum::CONNECTMODEUNSPECIFIED),
           "DIRECT_PEERING" => Ok(NetworkConfigConnectModeEnum::DIRECTPEERING),
           "PRIVATE_SERVICE_ACCESS" => Ok(NetworkConfigConnectModeEnum::PRIVATESERVICEACCESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigConnectModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigModesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Internet protocol versions for which the instance has IP addresses assigned. For this version, only MODE_IPV4 is supported.
pub enum NetworkConfigModesEnum {
    

    /// Internet protocol not set.
    ///
    /// "ADDRESS_MODE_UNSPECIFIED"
    #[serde(rename="ADDRESS_MODE_UNSPECIFIED")]
    ADDRESSMODEUNSPECIFIED,
    

    /// Use the IPv4 internet protocol.
    ///
    /// "MODE_IPV4"
    #[serde(rename="MODE_IPV4")]
    MODEIPV4,
}

impl AsRef<str> for NetworkConfigModesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigModesEnum::ADDRESSMODEUNSPECIFIED => "ADDRESS_MODE_UNSPECIFIED",
            NetworkConfigModesEnum::MODEIPV4 => "MODE_IPV4",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigModesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ADDRESS_MODE_UNSPECIFIED" => Ok(NetworkConfigModesEnum::ADDRESSMODEUNSPECIFIED),
           "MODE_IPV4" => Ok(NetworkConfigModesEnum::MODEIPV4),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigModesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NfsExportOptionAccessModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Either READ_ONLY, for allowing only read requests on the exported directory, or READ_WRITE, for allowing both read and write requests. The default is READ_WRITE.
pub enum NfsExportOptionAccessModeEnum {
    

    /// AccessMode not set.
    ///
    /// "ACCESS_MODE_UNSPECIFIED"
    #[serde(rename="ACCESS_MODE_UNSPECIFIED")]
    ACCESSMODEUNSPECIFIED,
    

    /// The client can only read the file share.
    ///
    /// "READ_ONLY"
    #[serde(rename="READ_ONLY")]
    READONLY,
    

    /// The client can read and write the file share (default).
    ///
    /// "READ_WRITE"
    #[serde(rename="READ_WRITE")]
    READWRITE,
}

impl AsRef<str> for NfsExportOptionAccessModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NfsExportOptionAccessModeEnum::ACCESSMODEUNSPECIFIED => "ACCESS_MODE_UNSPECIFIED",
            NfsExportOptionAccessModeEnum::READONLY => "READ_ONLY",
            NfsExportOptionAccessModeEnum::READWRITE => "READ_WRITE",
        }
    }
}

impl std::convert::TryFrom< &str> for NfsExportOptionAccessModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_MODE_UNSPECIFIED" => Ok(NfsExportOptionAccessModeEnum::ACCESSMODEUNSPECIFIED),
           "READ_ONLY" => Ok(NfsExportOptionAccessModeEnum::READONLY),
           "READ_WRITE" => Ok(NfsExportOptionAccessModeEnum::READWRITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NfsExportOptionAccessModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NfsExportOptionSquashModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Either NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH, for not allowing root access. The default is NO_ROOT_SQUASH.
pub enum NfsExportOptionSquashModeEnum {
    

    /// SquashMode not set.
    ///
    /// "SQUASH_MODE_UNSPECIFIED"
    #[serde(rename="SQUASH_MODE_UNSPECIFIED")]
    SQUASHMODEUNSPECIFIED,
    

    /// The Root user has root access to the file share (default).
    ///
    /// "NO_ROOT_SQUASH"
    #[serde(rename="NO_ROOT_SQUASH")]
    NOROOTSQUASH,
    

    /// The Root user has squashed access to the anonymous uid/gid.
    ///
    /// "ROOT_SQUASH"
    #[serde(rename="ROOT_SQUASH")]
    ROOTSQUASH,
}

impl AsRef<str> for NfsExportOptionSquashModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NfsExportOptionSquashModeEnum::SQUASHMODEUNSPECIFIED => "SQUASH_MODE_UNSPECIFIED",
            NfsExportOptionSquashModeEnum::NOROOTSQUASH => "NO_ROOT_SQUASH",
            NfsExportOptionSquashModeEnum::ROOTSQUASH => "ROOT_SQUASH",
        }
    }
}

impl std::convert::TryFrom< &str> for NfsExportOptionSquashModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SQUASH_MODE_UNSPECIFIED" => Ok(NfsExportOptionSquashModeEnum::SQUASHMODEUNSPECIFIED),
           "NO_ROOT_SQUASH" => Ok(NfsExportOptionSquashModeEnum::NOROOTSQUASH),
           "ROOT_SQUASH" => Ok(NfsExportOptionSquashModeEnum::ROOTSQUASH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NfsExportOptionSquashModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SnapshotStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The snapshot state.
pub enum SnapshotStateEnum {
    

    /// State not set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Snapshot is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Snapshot is available for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Snapshot is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for SnapshotStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SnapshotStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SnapshotStateEnum::CREATING => "CREATING",
            SnapshotStateEnum::READY => "READY",
            SnapshotStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for SnapshotStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SnapshotStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(SnapshotStateEnum::CREATING),
           "READY" => Ok(SnapshotStateEnum::READY),
           "DELETING" => Ok(SnapshotStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SnapshotStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


