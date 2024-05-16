use super::*;



// region AllowedClientMountPermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mount permissions.
pub enum AllowedClientMountPermissionsEnum {
    

    /// Permissions were not specified.
    ///
    /// "MOUNT_PERMISSIONS_UNSPECIFIED"
    #[serde(rename="MOUNT_PERMISSIONS_UNSPECIFIED")]
    MOUNTPERMISSIONSUNSPECIFIED,
    

    /// NFS share can be mount with read-only permissions.
    ///
    /// "READ"
    #[serde(rename="READ")]
    READ,
    

    /// NFS share can be mount with read-write permissions.
    ///
    /// "READ_WRITE"
    #[serde(rename="READ_WRITE")]
    READWRITE,
}

impl AsRef<str> for AllowedClientMountPermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AllowedClientMountPermissionsEnum::MOUNTPERMISSIONSUNSPECIFIED => "MOUNT_PERMISSIONS_UNSPECIFIED",
            AllowedClientMountPermissionsEnum::READ => "READ",
            AllowedClientMountPermissionsEnum::READWRITE => "READ_WRITE",
        }
    }
}

impl std::convert::TryFrom< &str> for AllowedClientMountPermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MOUNT_PERMISSIONS_UNSPECIFIED" => Ok(AllowedClientMountPermissionsEnum::MOUNTPERMISSIONSUNSPECIFIED),
           "READ" => Ok(AllowedClientMountPermissionsEnum::READ),
           "READ_WRITE" => Ok(AllowedClientMountPermissionsEnum::READWRITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AllowedClientMountPermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the server.
pub enum InstanceStateEnum {
    

    /// The server is in an unknown state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The server is being provisioned.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The server is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The server has been deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The server is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The server is starting.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The server is stopping.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The server is shutdown.
    ///
    /// "SHUTDOWN"
    #[serde(rename="SHUTDOWN")]
    SHUTDOWN,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::PROVISIONING => "PROVISIONING",
            InstanceStateEnum::RUNNING => "RUNNING",
            InstanceStateEnum::DELETED => "DELETED",
            InstanceStateEnum::UPDATING => "UPDATING",
            InstanceStateEnum::STARTING => "STARTING",
            InstanceStateEnum::STOPPING => "STOPPING",
            InstanceStateEnum::SHUTDOWN => "SHUTDOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstanceStateEnum::STATEUNSPECIFIED),
           "PROVISIONING" => Ok(InstanceStateEnum::PROVISIONING),
           "RUNNING" => Ok(InstanceStateEnum::RUNNING),
           "DELETED" => Ok(InstanceStateEnum::DELETED),
           "UPDATING" => Ok(InstanceStateEnum::UPDATING),
           "STARTING" => Ok(InstanceStateEnum::STARTING),
           "STOPPING" => Ok(InstanceStateEnum::STOPPING),
           "SHUTDOWN" => Ok(InstanceStateEnum::SHUTDOWN),
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


// region InstanceWorkloadProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The workload profile for the instance.
pub enum InstanceWorkloadProfileEnum {
    

    /// The workload profile is in an unknown state.
    ///
    /// "WORKLOAD_PROFILE_UNSPECIFIED"
    #[serde(rename="WORKLOAD_PROFILE_UNSPECIFIED")]
    WORKLOADPROFILEUNSPECIFIED,
    

    /// The workload profile is generic.
    ///
    /// "WORKLOAD_PROFILE_GENERIC"
    #[serde(rename="WORKLOAD_PROFILE_GENERIC")]
    WORKLOADPROFILEGENERIC,
    

    /// The workload profile is hana.
    ///
    /// "WORKLOAD_PROFILE_HANA"
    #[serde(rename="WORKLOAD_PROFILE_HANA")]
    WORKLOADPROFILEHANA,
}

impl AsRef<str> for InstanceWorkloadProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceWorkloadProfileEnum::WORKLOADPROFILEUNSPECIFIED => "WORKLOAD_PROFILE_UNSPECIFIED",
            InstanceWorkloadProfileEnum::WORKLOADPROFILEGENERIC => "WORKLOAD_PROFILE_GENERIC",
            InstanceWorkloadProfileEnum::WORKLOADPROFILEHANA => "WORKLOAD_PROFILE_HANA",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceWorkloadProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WORKLOAD_PROFILE_UNSPECIFIED" => Ok(InstanceWorkloadProfileEnum::WORKLOADPROFILEUNSPECIFIED),
           "WORKLOAD_PROFILE_GENERIC" => Ok(InstanceWorkloadProfileEnum::WORKLOADPROFILEGENERIC),
           "WORKLOAD_PROFILE_HANA" => Ok(InstanceWorkloadProfileEnum::WORKLOADPROFILEHANA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceWorkloadProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceConfigNetworkConfigEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of network configuration on the instance.
pub enum InstanceConfigNetworkConfigEnum {
    

    /// The unspecified network configuration.
    ///
    /// "NETWORKCONFIG_UNSPECIFIED"
    #[serde(rename="NETWORKCONFIG_UNSPECIFIED")]
    NETWORKCONFIGUNSPECIFIED,
    

    /// Instance part of single client network and single private network.
    ///
    /// "SINGLE_VLAN"
    #[serde(rename="SINGLE_VLAN")]
    SINGLEVLAN,
    

    /// Instance part of multiple (or single) client networks and private networks.
    ///
    /// "MULTI_VLAN"
    #[serde(rename="MULTI_VLAN")]
    MULTIVLAN,
}

impl AsRef<str> for InstanceConfigNetworkConfigEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceConfigNetworkConfigEnum::NETWORKCONFIGUNSPECIFIED => "NETWORKCONFIG_UNSPECIFIED",
            InstanceConfigNetworkConfigEnum::SINGLEVLAN => "SINGLE_VLAN",
            InstanceConfigNetworkConfigEnum::MULTIVLAN => "MULTI_VLAN",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceConfigNetworkConfigEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORKCONFIG_UNSPECIFIED" => Ok(InstanceConfigNetworkConfigEnum::NETWORKCONFIGUNSPECIFIED),
           "SINGLE_VLAN" => Ok(InstanceConfigNetworkConfigEnum::SINGLEVLAN),
           "MULTI_VLAN" => Ok(InstanceConfigNetworkConfigEnum::MULTIVLAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceConfigNetworkConfigEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogicalNetworkInterfaceNetworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of network.
pub enum LogicalNetworkInterfaceNetworkTypeEnum {
    

    /// Unspecified value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Client network, a network peered to a Google Cloud VPC.
    ///
    /// "CLIENT"
    #[serde(rename="CLIENT")]
    CLIENT,
    

    /// Private network, a network local to the Bare Metal Solution environment.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for LogicalNetworkInterfaceNetworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogicalNetworkInterfaceNetworkTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            LogicalNetworkInterfaceNetworkTypeEnum::CLIENT => "CLIENT",
            LogicalNetworkInterfaceNetworkTypeEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for LogicalNetworkInterfaceNetworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(LogicalNetworkInterfaceNetworkTypeEnum::TYPEUNSPECIFIED),
           "CLIENT" => Ok(LogicalNetworkInterfaceNetworkTypeEnum::CLIENT),
           "PRIVATE" => Ok(LogicalNetworkInterfaceNetworkTypeEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogicalNetworkInterfaceNetworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LunMultiprotocolTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The LUN multiprotocol type ensures the characteristics of the LUN are optimized for each operating system.
pub enum LunMultiprotocolTypeEnum {
    

    /// Server has no OS specified.
    ///
    /// "MULTIPROTOCOL_TYPE_UNSPECIFIED"
    #[serde(rename="MULTIPROTOCOL_TYPE_UNSPECIFIED")]
    MULTIPROTOCOLTYPEUNSPECIFIED,
    

    /// Server with Linux OS.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
}

impl AsRef<str> for LunMultiprotocolTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LunMultiprotocolTypeEnum::MULTIPROTOCOLTYPEUNSPECIFIED => "MULTIPROTOCOL_TYPE_UNSPECIFIED",
            LunMultiprotocolTypeEnum::LINUX => "LINUX",
        }
    }
}

impl std::convert::TryFrom< &str> for LunMultiprotocolTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MULTIPROTOCOL_TYPE_UNSPECIFIED" => Ok(LunMultiprotocolTypeEnum::MULTIPROTOCOLTYPEUNSPECIFIED),
           "LINUX" => Ok(LunMultiprotocolTypeEnum::LINUX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LunMultiprotocolTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LunStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of this storage volume.
pub enum LunStateEnum {
    

    /// The LUN is in an unknown state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The LUN is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The LUN is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The LUN is ready for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The LUN has been requested to be deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The LUN is in cool off state. It will be deleted after `expire_time`.
    ///
    /// "COOL_OFF"
    #[serde(rename="COOL_OFF")]
    COOLOFF,
}

impl AsRef<str> for LunStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LunStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            LunStateEnum::CREATING => "CREATING",
            LunStateEnum::UPDATING => "UPDATING",
            LunStateEnum::READY => "READY",
            LunStateEnum::DELETING => "DELETING",
            LunStateEnum::COOLOFF => "COOL_OFF",
        }
    }
}

impl std::convert::TryFrom< &str> for LunStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(LunStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(LunStateEnum::CREATING),
           "UPDATING" => Ok(LunStateEnum::UPDATING),
           "READY" => Ok(LunStateEnum::READY),
           "DELETING" => Ok(LunStateEnum::DELETING),
           "COOL_OFF" => Ok(LunStateEnum::COOLOFF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LunStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LunStorageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The storage type for this LUN.
pub enum LunStorageTypeEnum {
    

    /// The storage type for this LUN is unknown.
    ///
    /// "STORAGE_TYPE_UNSPECIFIED"
    #[serde(rename="STORAGE_TYPE_UNSPECIFIED")]
    STORAGETYPEUNSPECIFIED,
    

    /// This storage type for this LUN is SSD.
    ///
    /// "SSD"
    #[serde(rename="SSD")]
    SSD,
    

    /// This storage type for this LUN is HDD.
    ///
    /// "HDD"
    #[serde(rename="HDD")]
    HDD,
}

impl AsRef<str> for LunStorageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LunStorageTypeEnum::STORAGETYPEUNSPECIFIED => "STORAGE_TYPE_UNSPECIFIED",
            LunStorageTypeEnum::SSD => "SSD",
            LunStorageTypeEnum::HDD => "HDD",
        }
    }
}

impl std::convert::TryFrom< &str> for LunStorageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_TYPE_UNSPECIFIED" => Ok(LunStorageTypeEnum::STORAGETYPEUNSPECIFIED),
           "SSD" => Ok(LunStorageTypeEnum::SSD),
           "HDD" => Ok(LunStorageTypeEnum::HDD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LunStorageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Network state.
pub enum NetworkStateEnum {
    

    /// The Network is in an unknown state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Network is provisioning.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The Network has been provisioned.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
    

    /// The Network is being deprovisioned.
    ///
    /// "DEPROVISIONING"
    #[serde(rename="DEPROVISIONING")]
    DEPROVISIONING,
    

    /// The Network is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for NetworkStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            NetworkStateEnum::PROVISIONING => "PROVISIONING",
            NetworkStateEnum::PROVISIONED => "PROVISIONED",
            NetworkStateEnum::DEPROVISIONING => "DEPROVISIONING",
            NetworkStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(NetworkStateEnum::STATEUNSPECIFIED),
           "PROVISIONING" => Ok(NetworkStateEnum::PROVISIONING),
           "PROVISIONED" => Ok(NetworkStateEnum::PROVISIONED),
           "DEPROVISIONING" => Ok(NetworkStateEnum::DEPROVISIONING),
           "UPDATING" => Ok(NetworkStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this network.
pub enum NetworkTypeEnum {
    

    /// Unspecified value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Client network, a network peered to a Google Cloud VPC.
    ///
    /// "CLIENT"
    #[serde(rename="CLIENT")]
    CLIENT,
    

    /// Private network, a network local to the Bare Metal Solution environment.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for NetworkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            NetworkTypeEnum::CLIENT => "CLIENT",
            NetworkTypeEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(NetworkTypeEnum::TYPEUNSPECIFIED),
           "CLIENT" => Ok(NetworkTypeEnum::CLIENT),
           "PRIVATE" => Ok(NetworkTypeEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigBandwidthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Interconnect bandwidth. Set only when type is CLIENT.
pub enum NetworkConfigBandwidthEnum {
    

    /// Unspecified value.
    ///
    /// "BANDWIDTH_UNSPECIFIED"
    #[serde(rename="BANDWIDTH_UNSPECIFIED")]
    BANDWIDTHUNSPECIFIED,
    

    /// 1 Gbps.
    ///
    /// "BW_1_GBPS"
    #[serde(rename="BW_1_GBPS")]
    BW1GBPS,
    

    /// 2 Gbps.
    ///
    /// "BW_2_GBPS"
    #[serde(rename="BW_2_GBPS")]
    BW2GBPS,
    

    /// 5 Gbps.
    ///
    /// "BW_5_GBPS"
    #[serde(rename="BW_5_GBPS")]
    BW5GBPS,
    

    /// 10 Gbps.
    ///
    /// "BW_10_GBPS"
    #[serde(rename="BW_10_GBPS")]
    BW10GBPS,
}

impl AsRef<str> for NetworkConfigBandwidthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigBandwidthEnum::BANDWIDTHUNSPECIFIED => "BANDWIDTH_UNSPECIFIED",
            NetworkConfigBandwidthEnum::BW1GBPS => "BW_1_GBPS",
            NetworkConfigBandwidthEnum::BW2GBPS => "BW_2_GBPS",
            NetworkConfigBandwidthEnum::BW5GBPS => "BW_5_GBPS",
            NetworkConfigBandwidthEnum::BW10GBPS => "BW_10_GBPS",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigBandwidthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BANDWIDTH_UNSPECIFIED" => Ok(NetworkConfigBandwidthEnum::BANDWIDTHUNSPECIFIED),
           "BW_1_GBPS" => Ok(NetworkConfigBandwidthEnum::BW1GBPS),
           "BW_2_GBPS" => Ok(NetworkConfigBandwidthEnum::BW2GBPS),
           "BW_5_GBPS" => Ok(NetworkConfigBandwidthEnum::BW5GBPS),
           "BW_10_GBPS" => Ok(NetworkConfigBandwidthEnum::BW10GBPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigBandwidthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigServiceCidrEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Service CIDR, if any.
pub enum NetworkConfigServiceCidrEnum {
    

    /// Unspecified value.
    ///
    /// "SERVICE_CIDR_UNSPECIFIED"
    #[serde(rename="SERVICE_CIDR_UNSPECIFIED")]
    SERVICECIDRUNSPECIFIED,
    

    /// Services are disabled for the given network.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Use the highest /26 block of the network to host services.
    ///
    /// "HIGH_26"
    #[serde(rename="HIGH_26")]
    HIGH26,
    

    /// Use the highest /27 block of the network to host services.
    ///
    /// "HIGH_27"
    #[serde(rename="HIGH_27")]
    HIGH27,
    

    /// Use the highest /28 block of the network to host services.
    ///
    /// "HIGH_28"
    #[serde(rename="HIGH_28")]
    HIGH28,
}

impl AsRef<str> for NetworkConfigServiceCidrEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigServiceCidrEnum::SERVICECIDRUNSPECIFIED => "SERVICE_CIDR_UNSPECIFIED",
            NetworkConfigServiceCidrEnum::DISABLED => "DISABLED",
            NetworkConfigServiceCidrEnum::HIGH26 => "HIGH_26",
            NetworkConfigServiceCidrEnum::HIGH27 => "HIGH_27",
            NetworkConfigServiceCidrEnum::HIGH28 => "HIGH_28",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigServiceCidrEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_CIDR_UNSPECIFIED" => Ok(NetworkConfigServiceCidrEnum::SERVICECIDRUNSPECIFIED),
           "DISABLED" => Ok(NetworkConfigServiceCidrEnum::DISABLED),
           "HIGH_26" => Ok(NetworkConfigServiceCidrEnum::HIGH26),
           "HIGH_27" => Ok(NetworkConfigServiceCidrEnum::HIGH27),
           "HIGH_28" => Ok(NetworkConfigServiceCidrEnum::HIGH28),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigServiceCidrEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this network, either Client or Private.
pub enum NetworkConfigTypeEnum {
    

    /// Unspecified value.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Client network, that is a network peered to a GCP VPC.
    ///
    /// "CLIENT"
    #[serde(rename="CLIENT")]
    CLIENT,
    

    /// Private network, that is a network local to the BMS POD.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for NetworkConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            NetworkConfigTypeEnum::CLIENT => "CLIENT",
            NetworkConfigTypeEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(NetworkConfigTypeEnum::TYPEUNSPECIFIED),
           "CLIENT" => Ok(NetworkConfigTypeEnum::CLIENT),
           "PRIVATE" => Ok(NetworkConfigTypeEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NfsExportPermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Export permissions.
pub enum NfsExportPermissionsEnum {
    

    /// Unspecified value.
    ///
    /// "PERMISSIONS_UNSPECIFIED"
    #[serde(rename="PERMISSIONS_UNSPECIFIED")]
    PERMISSIONSUNSPECIFIED,
    

    /// Read-only permission.
    ///
    /// "READ_ONLY"
    #[serde(rename="READ_ONLY")]
    READONLY,
    

    /// Read-write permission.
    ///
    /// "READ_WRITE"
    #[serde(rename="READ_WRITE")]
    READWRITE,
}

impl AsRef<str> for NfsExportPermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NfsExportPermissionsEnum::PERMISSIONSUNSPECIFIED => "PERMISSIONS_UNSPECIFIED",
            NfsExportPermissionsEnum::READONLY => "READ_ONLY",
            NfsExportPermissionsEnum::READWRITE => "READ_WRITE",
        }
    }
}

impl std::convert::TryFrom< &str> for NfsExportPermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSIONS_UNSPECIFIED" => Ok(NfsExportPermissionsEnum::PERMISSIONSUNSPECIFIED),
           "READ_ONLY" => Ok(NfsExportPermissionsEnum::READONLY),
           "READ_WRITE" => Ok(NfsExportPermissionsEnum::READWRITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NfsExportPermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NfsShareStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the NFS share.
pub enum NfsShareStateEnum {
    

    /// The share is in an unknown state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The share has been provisioned.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
    

    /// The NFS Share is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The NFS Share is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The NFS Share has been requested to be deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for NfsShareStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NfsShareStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            NfsShareStateEnum::PROVISIONED => "PROVISIONED",
            NfsShareStateEnum::CREATING => "CREATING",
            NfsShareStateEnum::UPDATING => "UPDATING",
            NfsShareStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for NfsShareStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(NfsShareStateEnum::STATEUNSPECIFIED),
           "PROVISIONED" => Ok(NfsShareStateEnum::PROVISIONED),
           "CREATING" => Ok(NfsShareStateEnum::CREATING),
           "UPDATING" => Ok(NfsShareStateEnum::UPDATING),
           "DELETING" => Ok(NfsShareStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NfsShareStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NfsShareStorageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The storage type of the underlying volume.
pub enum NfsShareStorageTypeEnum {
    

    /// The storage type for this volume is unknown.
    ///
    /// "STORAGE_TYPE_UNSPECIFIED"
    #[serde(rename="STORAGE_TYPE_UNSPECIFIED")]
    STORAGETYPEUNSPECIFIED,
    

    /// The storage type for this volume is SSD.
    ///
    /// "SSD"
    #[serde(rename="SSD")]
    SSD,
    

    /// This storage type for this volume is HDD.
    ///
    /// "HDD"
    #[serde(rename="HDD")]
    HDD,
}

impl AsRef<str> for NfsShareStorageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NfsShareStorageTypeEnum::STORAGETYPEUNSPECIFIED => "STORAGE_TYPE_UNSPECIFIED",
            NfsShareStorageTypeEnum::SSD => "SSD",
            NfsShareStorageTypeEnum::HDD => "HDD",
        }
    }
}

impl std::convert::TryFrom< &str> for NfsShareStorageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_TYPE_UNSPECIFIED" => Ok(NfsShareStorageTypeEnum::STORAGETYPEUNSPECIFIED),
           "SSD" => Ok(NfsShareStorageTypeEnum::SSD),
           "HDD" => Ok(NfsShareStorageTypeEnum::HDD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NfsShareStorageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProvisioningConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of ProvisioningConfig.
pub enum ProvisioningConfigStateEnum {
    

    /// State wasn't specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// ProvisioningConfig is a draft and can be freely modified.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// ProvisioningConfig was already submitted and cannot be modified.
    ///
    /// "SUBMITTED"
    #[serde(rename="SUBMITTED")]
    SUBMITTED,
    

    /// ProvisioningConfig was in the provisioning state. Initially this state comes from the work order table in big query when SNOW is used. Later this field can be set by the work order API.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// ProvisioningConfig was provisioned, meaning the resources exist.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
    

    /// ProvisioningConfig was validated. A validation tool will be run to set this state.
    ///
    /// "VALIDATED"
    #[serde(rename="VALIDATED")]
    VALIDATED,
    

    /// ProvisioningConfig was canceled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The request is submitted for provisioning, with error return.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for ProvisioningConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProvisioningConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ProvisioningConfigStateEnum::DRAFT => "DRAFT",
            ProvisioningConfigStateEnum::SUBMITTED => "SUBMITTED",
            ProvisioningConfigStateEnum::PROVISIONING => "PROVISIONING",
            ProvisioningConfigStateEnum::PROVISIONED => "PROVISIONED",
            ProvisioningConfigStateEnum::VALIDATED => "VALIDATED",
            ProvisioningConfigStateEnum::CANCELLED => "CANCELLED",
            ProvisioningConfigStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProvisioningConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ProvisioningConfigStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(ProvisioningConfigStateEnum::DRAFT),
           "SUBMITTED" => Ok(ProvisioningConfigStateEnum::SUBMITTED),
           "PROVISIONING" => Ok(ProvisioningConfigStateEnum::PROVISIONING),
           "PROVISIONED" => Ok(ProvisioningConfigStateEnum::PROVISIONED),
           "VALIDATED" => Ok(ProvisioningConfigStateEnum::VALIDATED),
           "CANCELLED" => Ok(ProvisioningConfigStateEnum::CANCELLED),
           "FAILED" => Ok(ProvisioningConfigStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProvisioningConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProvisioningQuotaAssetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The asset type of this provisioning quota.
pub enum ProvisioningQuotaAssetTypeEnum {
    

    /// The unspecified type.
    ///
    /// "ASSET_TYPE_UNSPECIFIED"
    #[serde(rename="ASSET_TYPE_UNSPECIFIED")]
    ASSETTYPEUNSPECIFIED,
    

    /// The server asset type.
    ///
    /// "ASSET_TYPE_SERVER"
    #[serde(rename="ASSET_TYPE_SERVER")]
    ASSETTYPESERVER,
    

    /// The storage asset type.
    ///
    /// "ASSET_TYPE_STORAGE"
    #[serde(rename="ASSET_TYPE_STORAGE")]
    ASSETTYPESTORAGE,
    

    /// The network asset type.
    ///
    /// "ASSET_TYPE_NETWORK"
    #[serde(rename="ASSET_TYPE_NETWORK")]
    ASSETTYPENETWORK,
}

impl AsRef<str> for ProvisioningQuotaAssetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProvisioningQuotaAssetTypeEnum::ASSETTYPEUNSPECIFIED => "ASSET_TYPE_UNSPECIFIED",
            ProvisioningQuotaAssetTypeEnum::ASSETTYPESERVER => "ASSET_TYPE_SERVER",
            ProvisioningQuotaAssetTypeEnum::ASSETTYPESTORAGE => "ASSET_TYPE_STORAGE",
            ProvisioningQuotaAssetTypeEnum::ASSETTYPENETWORK => "ASSET_TYPE_NETWORK",
        }
    }
}

impl std::convert::TryFrom< &str> for ProvisioningQuotaAssetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSET_TYPE_UNSPECIFIED" => Ok(ProvisioningQuotaAssetTypeEnum::ASSETTYPEUNSPECIFIED),
           "ASSET_TYPE_SERVER" => Ok(ProvisioningQuotaAssetTypeEnum::ASSETTYPESERVER),
           "ASSET_TYPE_STORAGE" => Ok(ProvisioningQuotaAssetTypeEnum::ASSETTYPESTORAGE),
           "ASSET_TYPE_NETWORK" => Ok(ProvisioningQuotaAssetTypeEnum::ASSETTYPENETWORK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProvisioningQuotaAssetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VRFStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The possible state of VRF.
pub enum VRFStateEnum {
    

    /// The unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The vrf is provisioning.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The vrf is provisioned.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
}

impl AsRef<str> for VRFStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VRFStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            VRFStateEnum::PROVISIONING => "PROVISIONING",
            VRFStateEnum::PROVISIONED => "PROVISIONED",
        }
    }
}

impl std::convert::TryFrom< &str> for VRFStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(VRFStateEnum::STATEUNSPECIFIED),
           "PROVISIONING" => Ok(VRFStateEnum::PROVISIONING),
           "PROVISIONED" => Ok(VRFStateEnum::PROVISIONED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VRFStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumePerformanceTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. Performance tier of the Volume. Default is SHARED.
pub enum VolumePerformanceTierEnum {
    

    /// Value is not specified.
    ///
    /// "VOLUME_PERFORMANCE_TIER_UNSPECIFIED"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_UNSPECIFIED")]
    VOLUMEPERFORMANCETIERUNSPECIFIED,
    

    /// Regular volumes, shared aggregates.
    ///
    /// "VOLUME_PERFORMANCE_TIER_SHARED"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_SHARED")]
    VOLUMEPERFORMANCETIERSHARED,
    

    /// Assigned aggregates.
    ///
    /// "VOLUME_PERFORMANCE_TIER_ASSIGNED"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_ASSIGNED")]
    VOLUMEPERFORMANCETIERASSIGNED,
    

    /// High throughput aggregates.
    ///
    /// "VOLUME_PERFORMANCE_TIER_HT"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_HT")]
    VOLUMEPERFORMANCETIERHT,
    

    /// QoS 2.0 high performance storage.
    ///
    /// "VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE")]
    VOLUMEPERFORMANCETIERQOS2PERFORMANCE,
}

impl AsRef<str> for VolumePerformanceTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERUNSPECIFIED => "VOLUME_PERFORMANCE_TIER_UNSPECIFIED",
            VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERSHARED => "VOLUME_PERFORMANCE_TIER_SHARED",
            VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERASSIGNED => "VOLUME_PERFORMANCE_TIER_ASSIGNED",
            VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERHT => "VOLUME_PERFORMANCE_TIER_HT",
            VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERQOS2PERFORMANCE => "VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumePerformanceTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VOLUME_PERFORMANCE_TIER_UNSPECIFIED" => Ok(VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERUNSPECIFIED),
           "VOLUME_PERFORMANCE_TIER_SHARED" => Ok(VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERSHARED),
           "VOLUME_PERFORMANCE_TIER_ASSIGNED" => Ok(VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERASSIGNED),
           "VOLUME_PERFORMANCE_TIER_HT" => Ok(VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERHT),
           "VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE" => Ok(VolumePerformanceTierEnum::VOLUMEPERFORMANCETIERQOS2PERFORMANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumePerformanceTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Storage protocol for the Volume.
pub enum VolumeProtocolEnum {
    

    /// Value is not specified.
    ///
    /// "PROTOCOL_UNSPECIFIED"
    #[serde(rename="PROTOCOL_UNSPECIFIED")]
    PROTOCOLUNSPECIFIED,
    

    /// Fibre Channel protocol.
    ///
    /// "FIBRE_CHANNEL"
    #[serde(rename="FIBRE_CHANNEL")]
    FIBRECHANNEL,
    

    /// NFS protocol means Volume is a NFS Share volume. Such volumes cannot be manipulated via Volumes API.
    ///
    /// "NFS"
    #[serde(rename="NFS")]
    NFS,
}

impl AsRef<str> for VolumeProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeProtocolEnum::PROTOCOLUNSPECIFIED => "PROTOCOL_UNSPECIFIED",
            VolumeProtocolEnum::FIBRECHANNEL => "FIBRE_CHANNEL",
            VolumeProtocolEnum::NFS => "NFS",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTOCOL_UNSPECIFIED" => Ok(VolumeProtocolEnum::PROTOCOLUNSPECIFIED),
           "FIBRE_CHANNEL" => Ok(VolumeProtocolEnum::FIBRECHANNEL),
           "NFS" => Ok(VolumeProtocolEnum::NFS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeSnapshotAutoDeleteBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The behavior to use when snapshot reserved space is full.
pub enum VolumeSnapshotAutoDeleteBehaviorEnum {
    

    /// The unspecified behavior.
    ///
    /// "SNAPSHOT_AUTO_DELETE_BEHAVIOR_UNSPECIFIED"
    #[serde(rename="SNAPSHOT_AUTO_DELETE_BEHAVIOR_UNSPECIFIED")]
    SNAPSHOTAUTODELETEBEHAVIORUNSPECIFIED,
    

    /// Don't delete any snapshots. This disables new snapshot creation, as long as the snapshot reserved space is full.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Delete the oldest snapshots first.
    ///
    /// "OLDEST_FIRST"
    #[serde(rename="OLDEST_FIRST")]
    OLDESTFIRST,
    

    /// Delete the newest snapshots first.
    ///
    /// "NEWEST_FIRST"
    #[serde(rename="NEWEST_FIRST")]
    NEWESTFIRST,
}

impl AsRef<str> for VolumeSnapshotAutoDeleteBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeSnapshotAutoDeleteBehaviorEnum::SNAPSHOTAUTODELETEBEHAVIORUNSPECIFIED => "SNAPSHOT_AUTO_DELETE_BEHAVIOR_UNSPECIFIED",
            VolumeSnapshotAutoDeleteBehaviorEnum::DISABLED => "DISABLED",
            VolumeSnapshotAutoDeleteBehaviorEnum::OLDESTFIRST => "OLDEST_FIRST",
            VolumeSnapshotAutoDeleteBehaviorEnum::NEWESTFIRST => "NEWEST_FIRST",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeSnapshotAutoDeleteBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SNAPSHOT_AUTO_DELETE_BEHAVIOR_UNSPECIFIED" => Ok(VolumeSnapshotAutoDeleteBehaviorEnum::SNAPSHOTAUTODELETEBEHAVIORUNSPECIFIED),
           "DISABLED" => Ok(VolumeSnapshotAutoDeleteBehaviorEnum::DISABLED),
           "OLDEST_FIRST" => Ok(VolumeSnapshotAutoDeleteBehaviorEnum::OLDESTFIRST),
           "NEWEST_FIRST" => Ok(VolumeSnapshotAutoDeleteBehaviorEnum::NEWESTFIRST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeSnapshotAutoDeleteBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of this storage volume.
pub enum VolumeStateEnum {
    

    /// The storage volume is in an unknown state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The storage volume is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The storage volume is ready for use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The storage volume has been requested to be deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The storage volume is being updated.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The storage volume is in cool off state. It will be deleted after `expire_time`.
    ///
    /// "COOL_OFF"
    #[serde(rename="COOL_OFF")]
    COOLOFF,
}

impl AsRef<str> for VolumeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            VolumeStateEnum::CREATING => "CREATING",
            VolumeStateEnum::READY => "READY",
            VolumeStateEnum::DELETING => "DELETING",
            VolumeStateEnum::UPDATING => "UPDATING",
            VolumeStateEnum::COOLOFF => "COOL_OFF",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(VolumeStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(VolumeStateEnum::CREATING),
           "READY" => Ok(VolumeStateEnum::READY),
           "DELETING" => Ok(VolumeStateEnum::DELETING),
           "UPDATING" => Ok(VolumeStateEnum::UPDATING),
           "COOL_OFF" => Ok(VolumeStateEnum::COOLOFF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeStorageTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The storage type for this volume.
pub enum VolumeStorageTypeEnum {
    

    /// The storage type for this volume is unknown.
    ///
    /// "STORAGE_TYPE_UNSPECIFIED"
    #[serde(rename="STORAGE_TYPE_UNSPECIFIED")]
    STORAGETYPEUNSPECIFIED,
    

    /// The storage type for this volume is SSD.
    ///
    /// "SSD"
    #[serde(rename="SSD")]
    SSD,
    

    /// This storage type for this volume is HDD.
    ///
    /// "HDD"
    #[serde(rename="HDD")]
    HDD,
}

impl AsRef<str> for VolumeStorageTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeStorageTypeEnum::STORAGETYPEUNSPECIFIED => "STORAGE_TYPE_UNSPECIFIED",
            VolumeStorageTypeEnum::SSD => "SSD",
            VolumeStorageTypeEnum::HDD => "HDD",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeStorageTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_TYPE_UNSPECIFIED" => Ok(VolumeStorageTypeEnum::STORAGETYPEUNSPECIFIED),
           "SSD" => Ok(VolumeStorageTypeEnum::SSD),
           "HDD" => Ok(VolumeStorageTypeEnum::HDD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeStorageTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeWorkloadProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The workload profile for the volume.
pub enum VolumeWorkloadProfileEnum {
    

    /// The workload profile is in an unknown state.
    ///
    /// "WORKLOAD_PROFILE_UNSPECIFIED"
    #[serde(rename="WORKLOAD_PROFILE_UNSPECIFIED")]
    WORKLOADPROFILEUNSPECIFIED,
    

    /// The workload profile is generic.
    ///
    /// "GENERIC"
    #[serde(rename="GENERIC")]
    GENERIC,
    

    /// The workload profile is hana.
    ///
    /// "HANA"
    #[serde(rename="HANA")]
    HANA,
}

impl AsRef<str> for VolumeWorkloadProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeWorkloadProfileEnum::WORKLOADPROFILEUNSPECIFIED => "WORKLOAD_PROFILE_UNSPECIFIED",
            VolumeWorkloadProfileEnum::GENERIC => "GENERIC",
            VolumeWorkloadProfileEnum::HANA => "HANA",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeWorkloadProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WORKLOAD_PROFILE_UNSPECIFIED" => Ok(VolumeWorkloadProfileEnum::WORKLOADPROFILEUNSPECIFIED),
           "GENERIC" => Ok(VolumeWorkloadProfileEnum::GENERIC),
           "HANA" => Ok(VolumeWorkloadProfileEnum::HANA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeWorkloadProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeConfigPerformanceTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Performance tier of the Volume. Default is SHARED.
pub enum VolumeConfigPerformanceTierEnum {
    

    /// Value is not specified.
    ///
    /// "VOLUME_PERFORMANCE_TIER_UNSPECIFIED"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_UNSPECIFIED")]
    VOLUMEPERFORMANCETIERUNSPECIFIED,
    

    /// Regular volumes, shared aggregates.
    ///
    /// "VOLUME_PERFORMANCE_TIER_SHARED"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_SHARED")]
    VOLUMEPERFORMANCETIERSHARED,
    

    /// Assigned aggregates.
    ///
    /// "VOLUME_PERFORMANCE_TIER_ASSIGNED"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_ASSIGNED")]
    VOLUMEPERFORMANCETIERASSIGNED,
    

    /// High throughput aggregates.
    ///
    /// "VOLUME_PERFORMANCE_TIER_HT"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_HT")]
    VOLUMEPERFORMANCETIERHT,
    

    /// QoS 2.0 high performance storage.
    ///
    /// "VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE"
    #[serde(rename="VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE")]
    VOLUMEPERFORMANCETIERQOS2PERFORMANCE,
}

impl AsRef<str> for VolumeConfigPerformanceTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERUNSPECIFIED => "VOLUME_PERFORMANCE_TIER_UNSPECIFIED",
            VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERSHARED => "VOLUME_PERFORMANCE_TIER_SHARED",
            VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERASSIGNED => "VOLUME_PERFORMANCE_TIER_ASSIGNED",
            VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERHT => "VOLUME_PERFORMANCE_TIER_HT",
            VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERQOS2PERFORMANCE => "VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeConfigPerformanceTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VOLUME_PERFORMANCE_TIER_UNSPECIFIED" => Ok(VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERUNSPECIFIED),
           "VOLUME_PERFORMANCE_TIER_SHARED" => Ok(VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERSHARED),
           "VOLUME_PERFORMANCE_TIER_ASSIGNED" => Ok(VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERASSIGNED),
           "VOLUME_PERFORMANCE_TIER_HT" => Ok(VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERHT),
           "VOLUME_PERFORMANCE_TIER_QOS2_PERFORMANCE" => Ok(VolumeConfigPerformanceTierEnum::VOLUMEPERFORMANCETIERQOS2PERFORMANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeConfigPerformanceTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeConfigProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Volume protocol.
pub enum VolumeConfigProtocolEnum {
    

    /// Unspecified value.
    ///
    /// "PROTOCOL_UNSPECIFIED"
    #[serde(rename="PROTOCOL_UNSPECIFIED")]
    PROTOCOLUNSPECIFIED,
    

    /// Fibre channel.
    ///
    /// "PROTOCOL_FC"
    #[serde(rename="PROTOCOL_FC")]
    PROTOCOLFC,
    

    /// Network file system.
    ///
    /// "PROTOCOL_NFS"
    #[serde(rename="PROTOCOL_NFS")]
    PROTOCOLNFS,
}

impl AsRef<str> for VolumeConfigProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeConfigProtocolEnum::PROTOCOLUNSPECIFIED => "PROTOCOL_UNSPECIFIED",
            VolumeConfigProtocolEnum::PROTOCOLFC => "PROTOCOL_FC",
            VolumeConfigProtocolEnum::PROTOCOLNFS => "PROTOCOL_NFS",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeConfigProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROTOCOL_UNSPECIFIED" => Ok(VolumeConfigProtocolEnum::PROTOCOLUNSPECIFIED),
           "PROTOCOL_FC" => Ok(VolumeConfigProtocolEnum::PROTOCOLFC),
           "PROTOCOL_NFS" => Ok(VolumeConfigProtocolEnum::PROTOCOLNFS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeConfigProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this Volume.
pub enum VolumeConfigTypeEnum {
    

    /// The unspecified type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// This Volume is on flash.
    ///
    /// "FLASH"
    #[serde(rename="FLASH")]
    FLASH,
    

    /// This Volume is on disk.
    ///
    /// "DISK"
    #[serde(rename="DISK")]
    DISK,
}

impl AsRef<str> for VolumeConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeConfigTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            VolumeConfigTypeEnum::FLASH => "FLASH",
            VolumeConfigTypeEnum::DISK => "DISK",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(VolumeConfigTypeEnum::TYPEUNSPECIFIED),
           "FLASH" => Ok(VolumeConfigTypeEnum::FLASH),
           "DISK" => Ok(VolumeConfigTypeEnum::DISK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VolumeSnapshotTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the snapshot which indicates whether it was scheduled or manual/ad-hoc.
pub enum VolumeSnapshotTypeEnum {
    

    /// Type is not specified.
    ///
    /// "SNAPSHOT_TYPE_UNSPECIFIED"
    #[serde(rename="SNAPSHOT_TYPE_UNSPECIFIED")]
    SNAPSHOTTYPEUNSPECIFIED,
    

    /// Snapshot was taken manually by user.
    ///
    /// "AD_HOC"
    #[serde(rename="AD_HOC")]
    ADHOC,
    

    /// Snapshot was taken automatically as a part of a snapshot schedule.
    ///
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
}

impl AsRef<str> for VolumeSnapshotTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VolumeSnapshotTypeEnum::SNAPSHOTTYPEUNSPECIFIED => "SNAPSHOT_TYPE_UNSPECIFIED",
            VolumeSnapshotTypeEnum::ADHOC => "AD_HOC",
            VolumeSnapshotTypeEnum::SCHEDULED => "SCHEDULED",
        }
    }
}

impl std::convert::TryFrom< &str> for VolumeSnapshotTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SNAPSHOT_TYPE_UNSPECIFIED" => Ok(VolumeSnapshotTypeEnum::SNAPSHOTTYPEUNSPECIFIED),
           "AD_HOC" => Ok(VolumeSnapshotTypeEnum::ADHOC),
           "SCHEDULED" => Ok(VolumeSnapshotTypeEnum::SCHEDULED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VolumeSnapshotTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


