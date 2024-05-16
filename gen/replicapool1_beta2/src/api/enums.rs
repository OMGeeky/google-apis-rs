use super::*;



// region OperationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output Only] Status of the operation.
pub enum OperationStatusEnum {
    
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
}

impl AsRef<str> for OperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationStatusEnum::DONE => "DONE",
            OperationStatusEnum::PENDING => "PENDING",
            OperationStatusEnum::RUNNING => "RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DONE" => Ok(OperationStatusEnum::DONE),
           "PENDING" => Ok(OperationStatusEnum::PENDING),
           "RUNNING" => Ok(OperationStatusEnum::RUNNING),
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


// region ReplicaPoolAutoHealingPolicyActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action to perform when an instance becomes unhealthy. Possible values are RECREATE or REBOOT. RECREATE replaces an unhealthy instance with a new instance that is based on the instance template for this managed instance group. REBOOT performs a soft reboot on an instance. If the instance cannot reboot, the instance performs a hard restart.
pub enum ReplicaPoolAutoHealingPolicyActionTypeEnum {
    
    /// "REBOOT"
    #[serde(rename="REBOOT")]
    REBOOT,
    
    /// "RECREATE"
    #[serde(rename="RECREATE")]
    RECREATE,
}

impl AsRef<str> for ReplicaPoolAutoHealingPolicyActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReplicaPoolAutoHealingPolicyActionTypeEnum::REBOOT => "REBOOT",
            ReplicaPoolAutoHealingPolicyActionTypeEnum::RECREATE => "RECREATE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReplicaPoolAutoHealingPolicyActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REBOOT" => Ok(ReplicaPoolAutoHealingPolicyActionTypeEnum::REBOOT),
           "RECREATE" => Ok(ReplicaPoolAutoHealingPolicyActionTypeEnum::RECREATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReplicaPoolAutoHealingPolicyActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output only] The warning type identifier for this warning.
pub enum OperationWarningCodeEnum {
    
    /// "DEPRECATED_RESOURCE_USED"
    #[serde(rename="DEPRECATED_RESOURCE_USED")]
    DEPRECATEDRESOURCEUSED,
    
    /// "DISK_SIZE_LARGER_THAN_IMAGE_SIZE"
    #[serde(rename="DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    DISKSIZELARGERTHANIMAGESIZE,
    
    /// "INJECTED_KERNELS_DEPRECATED"
    #[serde(rename="INJECTED_KERNELS_DEPRECATED")]
    INJECTEDKERNELSDEPRECATED,
    
    /// "NEXT_HOP_ADDRESS_NOT_ASSIGNED"
    #[serde(rename="NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    NEXTHOPADDRESSNOTASSIGNED,
    
    /// "NEXT_HOP_CANNOT_IP_FORWARD"
    #[serde(rename="NEXT_HOP_CANNOT_IP_FORWARD")]
    NEXTHOPCANNOTIPFORWARD,
    
    /// "NEXT_HOP_INSTANCE_NOT_FOUND"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_FOUND")]
    NEXTHOPINSTANCENOTFOUND,
    
    /// "NEXT_HOP_INSTANCE_NOT_ON_NETWORK"
    #[serde(rename="NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    NEXTHOPINSTANCENOTONNETWORK,
    
    /// "NEXT_HOP_NOT_RUNNING"
    #[serde(rename="NEXT_HOP_NOT_RUNNING")]
    NEXTHOPNOTRUNNING,
    
    /// "NO_RESULTS_ON_PAGE"
    #[serde(rename="NO_RESULTS_ON_PAGE")]
    NORESULTSONPAGE,
    
    /// "REQUIRED_TOS_AGREEMENT"
    #[serde(rename="REQUIRED_TOS_AGREEMENT")]
    REQUIREDTOSAGREEMENT,
    
    /// "RESOURCE_NOT_DELETED"
    #[serde(rename="RESOURCE_NOT_DELETED")]
    RESOURCENOTDELETED,
    
    /// "SINGLE_INSTANCE_PROPERTY_TEMPLATE"
    #[serde(rename="SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    SINGLEINSTANCEPROPERTYTEMPLATE,
    
    /// "UNREACHABLE"
    #[serde(rename="UNREACHABLE")]
    UNREACHABLE,
}

impl AsRef<str> for OperationWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationWarningCodeEnum::DEPRECATEDRESOURCEUSED => "DEPRECATED_RESOURCE_USED",
            OperationWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE => "DISK_SIZE_LARGER_THAN_IMAGE_SIZE",
            OperationWarningCodeEnum::INJECTEDKERNELSDEPRECATED => "INJECTED_KERNELS_DEPRECATED",
            OperationWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED => "NEXT_HOP_ADDRESS_NOT_ASSIGNED",
            OperationWarningCodeEnum::NEXTHOPCANNOTIPFORWARD => "NEXT_HOP_CANNOT_IP_FORWARD",
            OperationWarningCodeEnum::NEXTHOPINSTANCENOTFOUND => "NEXT_HOP_INSTANCE_NOT_FOUND",
            OperationWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK => "NEXT_HOP_INSTANCE_NOT_ON_NETWORK",
            OperationWarningCodeEnum::NEXTHOPNOTRUNNING => "NEXT_HOP_NOT_RUNNING",
            OperationWarningCodeEnum::NORESULTSONPAGE => "NO_RESULTS_ON_PAGE",
            OperationWarningCodeEnum::REQUIREDTOSAGREEMENT => "REQUIRED_TOS_AGREEMENT",
            OperationWarningCodeEnum::RESOURCENOTDELETED => "RESOURCE_NOT_DELETED",
            OperationWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE => "SINGLE_INSTANCE_PROPERTY_TEMPLATE",
            OperationWarningCodeEnum::UNREACHABLE => "UNREACHABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEPRECATED_RESOURCE_USED" => Ok(OperationWarningCodeEnum::DEPRECATEDRESOURCEUSED),
           "DISK_SIZE_LARGER_THAN_IMAGE_SIZE" => Ok(OperationWarningCodeEnum::DISKSIZELARGERTHANIMAGESIZE),
           "INJECTED_KERNELS_DEPRECATED" => Ok(OperationWarningCodeEnum::INJECTEDKERNELSDEPRECATED),
           "NEXT_HOP_ADDRESS_NOT_ASSIGNED" => Ok(OperationWarningCodeEnum::NEXTHOPADDRESSNOTASSIGNED),
           "NEXT_HOP_CANNOT_IP_FORWARD" => Ok(OperationWarningCodeEnum::NEXTHOPCANNOTIPFORWARD),
           "NEXT_HOP_INSTANCE_NOT_FOUND" => Ok(OperationWarningCodeEnum::NEXTHOPINSTANCENOTFOUND),
           "NEXT_HOP_INSTANCE_NOT_ON_NETWORK" => Ok(OperationWarningCodeEnum::NEXTHOPINSTANCENOTONNETWORK),
           "NEXT_HOP_NOT_RUNNING" => Ok(OperationWarningCodeEnum::NEXTHOPNOTRUNNING),
           "NO_RESULTS_ON_PAGE" => Ok(OperationWarningCodeEnum::NORESULTSONPAGE),
           "REQUIRED_TOS_AGREEMENT" => Ok(OperationWarningCodeEnum::REQUIREDTOSAGREEMENT),
           "RESOURCE_NOT_DELETED" => Ok(OperationWarningCodeEnum::RESOURCENOTDELETED),
           "SINGLE_INSTANCE_PROPERTY_TEMPLATE" => Ok(OperationWarningCodeEnum::SINGLEINSTANCEPROPERTYTEMPLATE),
           "UNREACHABLE" => Ok(OperationWarningCodeEnum::UNREACHABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


