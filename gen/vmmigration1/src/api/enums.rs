use super::*;



// region AppliedLicenseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The license type that was used in OS adaptation.
pub enum AppliedLicenseTypeEnum {
    

    /// Unspecified license for the OS.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// No license available for the OS.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// The license type is Pay As You Go license type.
    ///
    /// "PAYG"
    #[serde(rename="PAYG")]
    PAYG,
    

    /// The license type is Bring Your Own License type.
    ///
    /// "BYOL"
    #[serde(rename="BYOL")]
    BYOL,
}

impl AsRef<str> for AppliedLicenseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppliedLicenseTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AppliedLicenseTypeEnum::NONE => "NONE",
            AppliedLicenseTypeEnum::PAYG => "PAYG",
            AppliedLicenseTypeEnum::BYOL => "BYOL",
        }
    }
}

impl std::convert::TryFrom< &str> for AppliedLicenseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AppliedLicenseTypeEnum::TYPEUNSPECIFIED),
           "NONE" => Ok(AppliedLicenseTypeEnum::NONE),
           "PAYG" => Ok(AppliedLicenseTypeEnum::PAYG),
           "BYOL" => Ok(AppliedLicenseTypeEnum::BYOL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppliedLicenseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AwsSourceDetailStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the source as determined by the health check.
pub enum AwsSourceDetailStateEnum {
    

    /// The state is unknown. This is used for API compatibility only and is not used by the system.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The state was not sampled by the health checks yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The source is available but might not be usable yet due to invalid credentials or another reason. The error message will contain further details.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The source exists and its credentials were verified.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for AwsSourceDetailStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsSourceDetailStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AwsSourceDetailStateEnum::PENDING => "PENDING",
            AwsSourceDetailStateEnum::FAILED => "FAILED",
            AwsSourceDetailStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsSourceDetailStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AwsSourceDetailStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(AwsSourceDetailStateEnum::PENDING),
           "FAILED" => Ok(AwsSourceDetailStateEnum::FAILED),
           "ACTIVE" => Ok(AwsSourceDetailStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsSourceDetailStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AwsSourceVmDetailFirmwareEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The firmware type of the source VM.
pub enum AwsSourceVmDetailFirmwareEnum {
    

    /// The firmware is unknown.
    ///
    /// "FIRMWARE_UNSPECIFIED"
    #[serde(rename="FIRMWARE_UNSPECIFIED")]
    FIRMWAREUNSPECIFIED,
    

    /// The firmware is EFI.
    ///
    /// "EFI"
    #[serde(rename="EFI")]
    EFI,
    

    /// The firmware is BIOS.
    ///
    /// "BIOS"
    #[serde(rename="BIOS")]
    BIOS,
}

impl AsRef<str> for AwsSourceVmDetailFirmwareEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsSourceVmDetailFirmwareEnum::FIRMWAREUNSPECIFIED => "FIRMWARE_UNSPECIFIED",
            AwsSourceVmDetailFirmwareEnum::EFI => "EFI",
            AwsSourceVmDetailFirmwareEnum::BIOS => "BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsSourceVmDetailFirmwareEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FIRMWARE_UNSPECIFIED" => Ok(AwsSourceVmDetailFirmwareEnum::FIRMWAREUNSPECIFIED),
           "EFI" => Ok(AwsSourceVmDetailFirmwareEnum::EFI),
           "BIOS" => Ok(AwsSourceVmDetailFirmwareEnum::BIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsSourceVmDetailFirmwareEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AwsVmDetailArchitectureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The CPU architecture.
pub enum AwsVmDetailArchitectureEnum {
    

    /// The architecture is unknown.
    ///
    /// "VM_ARCHITECTURE_UNSPECIFIED"
    #[serde(rename="VM_ARCHITECTURE_UNSPECIFIED")]
    VMARCHITECTUREUNSPECIFIED,
    

    /// The architecture is I386.
    ///
    /// "I386"
    #[serde(rename="I386")]
    I386,
    

    /// The architecture is X86_64.
    ///
    /// "X86_64"
    #[serde(rename="X86_64")]
    X8664,
    

    /// The architecture is ARM64.
    ///
    /// "ARM64"
    #[serde(rename="ARM64")]
    ARM64,
    

    /// The architecture is X86_64_MAC.
    ///
    /// "X86_64_MAC"
    #[serde(rename="X86_64_MAC")]
    X8664MAC,
}

impl AsRef<str> for AwsVmDetailArchitectureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsVmDetailArchitectureEnum::VMARCHITECTUREUNSPECIFIED => "VM_ARCHITECTURE_UNSPECIFIED",
            AwsVmDetailArchitectureEnum::I386 => "I386",
            AwsVmDetailArchitectureEnum::X8664 => "X86_64",
            AwsVmDetailArchitectureEnum::ARM64 => "ARM64",
            AwsVmDetailArchitectureEnum::X8664MAC => "X86_64_MAC",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsVmDetailArchitectureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VM_ARCHITECTURE_UNSPECIFIED" => Ok(AwsVmDetailArchitectureEnum::VMARCHITECTUREUNSPECIFIED),
           "I386" => Ok(AwsVmDetailArchitectureEnum::I386),
           "X86_64" => Ok(AwsVmDetailArchitectureEnum::X8664),
           "ARM64" => Ok(AwsVmDetailArchitectureEnum::ARM64),
           "X86_64_MAC" => Ok(AwsVmDetailArchitectureEnum::X8664MAC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsVmDetailArchitectureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AwsVmDetailBootOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The VM Boot Option.
pub enum AwsVmDetailBootOptionEnum {
    

    /// The boot option is unknown.
    ///
    /// "BOOT_OPTION_UNSPECIFIED"
    #[serde(rename="BOOT_OPTION_UNSPECIFIED")]
    BOOTOPTIONUNSPECIFIED,
    

    /// The boot option is UEFI.
    ///
    /// "EFI"
    #[serde(rename="EFI")]
    EFI,
    

    /// The boot option is LEGACY-BIOS.
    ///
    /// "BIOS"
    #[serde(rename="BIOS")]
    BIOS,
}

impl AsRef<str> for AwsVmDetailBootOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsVmDetailBootOptionEnum::BOOTOPTIONUNSPECIFIED => "BOOT_OPTION_UNSPECIFIED",
            AwsVmDetailBootOptionEnum::EFI => "EFI",
            AwsVmDetailBootOptionEnum::BIOS => "BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsVmDetailBootOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BOOT_OPTION_UNSPECIFIED" => Ok(AwsVmDetailBootOptionEnum::BOOTOPTIONUNSPECIFIED),
           "EFI" => Ok(AwsVmDetailBootOptionEnum::EFI),
           "BIOS" => Ok(AwsVmDetailBootOptionEnum::BIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsVmDetailBootOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AwsVmDetailPowerStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The power state of the VM at the moment list was taken.
pub enum AwsVmDetailPowerStateEnum {
    

    /// Power state is not specified.
    ///
    /// "POWER_STATE_UNSPECIFIED"
    #[serde(rename="POWER_STATE_UNSPECIFIED")]
    POWERSTATEUNSPECIFIED,
    

    /// The VM is turned on.
    ///
    /// "ON"
    #[serde(rename="ON")]
    ON,
    

    /// The VM is turned off.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// The VM is suspended. This is similar to hibernation or sleep mode.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
    

    /// The VM is starting.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for AwsVmDetailPowerStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsVmDetailPowerStateEnum::POWERSTATEUNSPECIFIED => "POWER_STATE_UNSPECIFIED",
            AwsVmDetailPowerStateEnum::ON => "ON",
            AwsVmDetailPowerStateEnum::OFF => "OFF",
            AwsVmDetailPowerStateEnum::SUSPENDED => "SUSPENDED",
            AwsVmDetailPowerStateEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsVmDetailPowerStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POWER_STATE_UNSPECIFIED" => Ok(AwsVmDetailPowerStateEnum::POWERSTATEUNSPECIFIED),
           "ON" => Ok(AwsVmDetailPowerStateEnum::ON),
           "OFF" => Ok(AwsVmDetailPowerStateEnum::OFF),
           "SUSPENDED" => Ok(AwsVmDetailPowerStateEnum::SUSPENDED),
           "PENDING" => Ok(AwsVmDetailPowerStateEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsVmDetailPowerStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AwsVmDetailVirtualizationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The virtualization type.
pub enum AwsVmDetailVirtualizationTypeEnum {
    

    /// The virtualization type is unknown.
    ///
    /// "VM_VIRTUALIZATION_TYPE_UNSPECIFIED"
    #[serde(rename="VM_VIRTUALIZATION_TYPE_UNSPECIFIED")]
    VMVIRTUALIZATIONTYPEUNSPECIFIED,
    

    /// The virtualziation type is HVM.
    ///
    /// "HVM"
    #[serde(rename="HVM")]
    HVM,
    

    /// The virtualziation type is PARAVIRTUAL.
    ///
    /// "PARAVIRTUAL"
    #[serde(rename="PARAVIRTUAL")]
    PARAVIRTUAL,
}

impl AsRef<str> for AwsVmDetailVirtualizationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AwsVmDetailVirtualizationTypeEnum::VMVIRTUALIZATIONTYPEUNSPECIFIED => "VM_VIRTUALIZATION_TYPE_UNSPECIFIED",
            AwsVmDetailVirtualizationTypeEnum::HVM => "HVM",
            AwsVmDetailVirtualizationTypeEnum::PARAVIRTUAL => "PARAVIRTUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for AwsVmDetailVirtualizationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VM_VIRTUALIZATION_TYPE_UNSPECIFIED" => Ok(AwsVmDetailVirtualizationTypeEnum::VMVIRTUALIZATIONTYPEUNSPECIFIED),
           "HVM" => Ok(AwsVmDetailVirtualizationTypeEnum::HVM),
           "PARAVIRTUAL" => Ok(AwsVmDetailVirtualizationTypeEnum::PARAVIRTUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AwsVmDetailVirtualizationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AzureSourceDetailStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the source as determined by the health check.
pub enum AzureSourceDetailStateEnum {
    

    /// The state is unknown. This is used for API compatibility only and is not used by the system.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The state was not sampled by the health checks yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The source is available but might not be usable yet due to invalid credentials or another reason. The error message will contain further details.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The source exists and its credentials were verified.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for AzureSourceDetailStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AzureSourceDetailStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AzureSourceDetailStateEnum::PENDING => "PENDING",
            AzureSourceDetailStateEnum::FAILED => "FAILED",
            AzureSourceDetailStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AzureSourceDetailStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AzureSourceDetailStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(AzureSourceDetailStateEnum::PENDING),
           "FAILED" => Ok(AzureSourceDetailStateEnum::FAILED),
           "ACTIVE" => Ok(AzureSourceDetailStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AzureSourceDetailStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AzureSourceVmDetailFirmwareEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The firmware type of the source VM.
pub enum AzureSourceVmDetailFirmwareEnum {
    

    /// The firmware is unknown.
    ///
    /// "FIRMWARE_UNSPECIFIED"
    #[serde(rename="FIRMWARE_UNSPECIFIED")]
    FIRMWAREUNSPECIFIED,
    

    /// The firmware is EFI.
    ///
    /// "EFI"
    #[serde(rename="EFI")]
    EFI,
    

    /// The firmware is BIOS.
    ///
    /// "BIOS"
    #[serde(rename="BIOS")]
    BIOS,
}

impl AsRef<str> for AzureSourceVmDetailFirmwareEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AzureSourceVmDetailFirmwareEnum::FIRMWAREUNSPECIFIED => "FIRMWARE_UNSPECIFIED",
            AzureSourceVmDetailFirmwareEnum::EFI => "EFI",
            AzureSourceVmDetailFirmwareEnum::BIOS => "BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for AzureSourceVmDetailFirmwareEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FIRMWARE_UNSPECIFIED" => Ok(AzureSourceVmDetailFirmwareEnum::FIRMWAREUNSPECIFIED),
           "EFI" => Ok(AzureSourceVmDetailFirmwareEnum::EFI),
           "BIOS" => Ok(AzureSourceVmDetailFirmwareEnum::BIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AzureSourceVmDetailFirmwareEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AzureVmDetailBootOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The VM Boot Option.
pub enum AzureVmDetailBootOptionEnum {
    

    /// The boot option is unknown.
    ///
    /// "BOOT_OPTION_UNSPECIFIED"
    #[serde(rename="BOOT_OPTION_UNSPECIFIED")]
    BOOTOPTIONUNSPECIFIED,
    

    /// The boot option is UEFI.
    ///
    /// "EFI"
    #[serde(rename="EFI")]
    EFI,
    

    /// The boot option is BIOS.
    ///
    /// "BIOS"
    #[serde(rename="BIOS")]
    BIOS,
}

impl AsRef<str> for AzureVmDetailBootOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AzureVmDetailBootOptionEnum::BOOTOPTIONUNSPECIFIED => "BOOT_OPTION_UNSPECIFIED",
            AzureVmDetailBootOptionEnum::EFI => "EFI",
            AzureVmDetailBootOptionEnum::BIOS => "BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for AzureVmDetailBootOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BOOT_OPTION_UNSPECIFIED" => Ok(AzureVmDetailBootOptionEnum::BOOTOPTIONUNSPECIFIED),
           "EFI" => Ok(AzureVmDetailBootOptionEnum::EFI),
           "BIOS" => Ok(AzureVmDetailBootOptionEnum::BIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AzureVmDetailBootOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AzureVmDetailPowerStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The power state of the VM at the moment list was taken.
pub enum AzureVmDetailPowerStateEnum {
    

    /// Power state is not specified.
    ///
    /// "POWER_STATE_UNSPECIFIED"
    #[serde(rename="POWER_STATE_UNSPECIFIED")]
    POWERSTATEUNSPECIFIED,
    

    /// The VM is starting.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The VM is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The VM is stopping.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The VM is stopped.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// The VM is deallocating.
    ///
    /// "DEALLOCATING"
    #[serde(rename="DEALLOCATING")]
    DEALLOCATING,
    

    /// The VM is deallocated.
    ///
    /// "DEALLOCATED"
    #[serde(rename="DEALLOCATED")]
    DEALLOCATED,
    

    /// The VM's power state is unknown.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
}

impl AsRef<str> for AzureVmDetailPowerStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AzureVmDetailPowerStateEnum::POWERSTATEUNSPECIFIED => "POWER_STATE_UNSPECIFIED",
            AzureVmDetailPowerStateEnum::STARTING => "STARTING",
            AzureVmDetailPowerStateEnum::RUNNING => "RUNNING",
            AzureVmDetailPowerStateEnum::STOPPING => "STOPPING",
            AzureVmDetailPowerStateEnum::STOPPED => "STOPPED",
            AzureVmDetailPowerStateEnum::DEALLOCATING => "DEALLOCATING",
            AzureVmDetailPowerStateEnum::DEALLOCATED => "DEALLOCATED",
            AzureVmDetailPowerStateEnum::UNKNOWN => "UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for AzureVmDetailPowerStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POWER_STATE_UNSPECIFIED" => Ok(AzureVmDetailPowerStateEnum::POWERSTATEUNSPECIFIED),
           "STARTING" => Ok(AzureVmDetailPowerStateEnum::STARTING),
           "RUNNING" => Ok(AzureVmDetailPowerStateEnum::RUNNING),
           "STOPPING" => Ok(AzureVmDetailPowerStateEnum::STOPPING),
           "STOPPED" => Ok(AzureVmDetailPowerStateEnum::STOPPED),
           "DEALLOCATING" => Ok(AzureVmDetailPowerStateEnum::DEALLOCATING),
           "DEALLOCATED" => Ok(AzureVmDetailPowerStateEnum::DEALLOCATED),
           "UNKNOWN" => Ok(AzureVmDetailPowerStateEnum::UNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AzureVmDetailPowerStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BootDiskDefaultDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of disk provisioning to use for the VM.
pub enum BootDiskDefaultDiskTypeEnum {
    

    /// An unspecified disk type. Will be used as STANDARD.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED")]
    COMPUTEENGINEDISKTYPEUNSPECIFIED,
    

    /// A Standard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_STANDARD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_STANDARD")]
    COMPUTEENGINEDISKTYPESTANDARD,
    

    /// SSD hard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_SSD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_SSD")]
    COMPUTEENGINEDISKTYPESSD,
    

    /// An alternative to SSD persistent disks that balance performance and cost.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_BALANCED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_BALANCED")]
    COMPUTEENGINEDISKTYPEBALANCED,
}

impl AsRef<str> for BootDiskDefaultDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED => "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED",
            BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD => "COMPUTE_ENGINE_DISK_TYPE_STANDARD",
            BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESSD => "COMPUTE_ENGINE_DISK_TYPE_SSD",
            BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED => "COMPUTE_ENGINE_DISK_TYPE_BALANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for BootDiskDefaultDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED" => Ok(BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED),
           "COMPUTE_ENGINE_DISK_TYPE_STANDARD" => Ok(BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD),
           "COMPUTE_ENGINE_DISK_TYPE_SSD" => Ok(BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESSD),
           "COMPUTE_ENGINE_DISK_TYPE_BALANCED" => Ok(BootDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BootDiskDefaultDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloneJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the clone job.
pub enum CloneJobStateEnum {
    

    /// The state is unknown. This is used for API compatibility only and is not used by the system.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The clone job has not yet started.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The clone job is active and running.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The clone job finished with errors.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The clone job finished successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The clone job was cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The clone job is being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// OS adaptation is running as part of the clone job to generate license.
    ///
    /// "ADAPTING_OS"
    #[serde(rename="ADAPTING_OS")]
    ADAPTINGOS,
}

impl AsRef<str> for CloneJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloneJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CloneJobStateEnum::PENDING => "PENDING",
            CloneJobStateEnum::ACTIVE => "ACTIVE",
            CloneJobStateEnum::FAILED => "FAILED",
            CloneJobStateEnum::SUCCEEDED => "SUCCEEDED",
            CloneJobStateEnum::CANCELLED => "CANCELLED",
            CloneJobStateEnum::CANCELLING => "CANCELLING",
            CloneJobStateEnum::ADAPTINGOS => "ADAPTING_OS",
        }
    }
}

impl std::convert::TryFrom< &str> for CloneJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CloneJobStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(CloneJobStateEnum::PENDING),
           "ACTIVE" => Ok(CloneJobStateEnum::ACTIVE),
           "FAILED" => Ok(CloneJobStateEnum::FAILED),
           "SUCCEEDED" => Ok(CloneJobStateEnum::SUCCEEDED),
           "CANCELLED" => Ok(CloneJobStateEnum::CANCELLED),
           "CANCELLING" => Ok(CloneJobStateEnum::CANCELLING),
           "ADAPTING_OS" => Ok(CloneJobStateEnum::ADAPTINGOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloneJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeEngineTargetDefaultBootOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The VM Boot Option, as set in the source VM.
pub enum ComputeEngineTargetDefaultBootOptionEnum {
    

    /// The boot option is unknown.
    ///
    /// "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED"
    #[serde(rename="COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED")]
    COMPUTEENGINEBOOTOPTIONUNSPECIFIED,
    

    /// The boot option is EFI.
    ///
    /// "COMPUTE_ENGINE_BOOT_OPTION_EFI"
    #[serde(rename="COMPUTE_ENGINE_BOOT_OPTION_EFI")]
    COMPUTEENGINEBOOTOPTIONEFI,
    

    /// The boot option is BIOS.
    ///
    /// "COMPUTE_ENGINE_BOOT_OPTION_BIOS"
    #[serde(rename="COMPUTE_ENGINE_BOOT_OPTION_BIOS")]
    COMPUTEENGINEBOOTOPTIONBIOS,
}

impl AsRef<str> for ComputeEngineTargetDefaultBootOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeEngineTargetDefaultBootOptionEnum::COMPUTEENGINEBOOTOPTIONUNSPECIFIED => "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED",
            ComputeEngineTargetDefaultBootOptionEnum::COMPUTEENGINEBOOTOPTIONEFI => "COMPUTE_ENGINE_BOOT_OPTION_EFI",
            ComputeEngineTargetDefaultBootOptionEnum::COMPUTEENGINEBOOTOPTIONBIOS => "COMPUTE_ENGINE_BOOT_OPTION_BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeEngineTargetDefaultBootOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED" => Ok(ComputeEngineTargetDefaultBootOptionEnum::COMPUTEENGINEBOOTOPTIONUNSPECIFIED),
           "COMPUTE_ENGINE_BOOT_OPTION_EFI" => Ok(ComputeEngineTargetDefaultBootOptionEnum::COMPUTEENGINEBOOTOPTIONEFI),
           "COMPUTE_ENGINE_BOOT_OPTION_BIOS" => Ok(ComputeEngineTargetDefaultBootOptionEnum::COMPUTEENGINEBOOTOPTIONBIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeEngineTargetDefaultBootOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeEngineTargetDefaultDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The disk type to use in the VM.
pub enum ComputeEngineTargetDefaultDiskTypeEnum {
    

    /// An unspecified disk type. Will be used as STANDARD.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED")]
    COMPUTEENGINEDISKTYPEUNSPECIFIED,
    

    /// A Standard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_STANDARD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_STANDARD")]
    COMPUTEENGINEDISKTYPESTANDARD,
    

    /// SSD hard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_SSD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_SSD")]
    COMPUTEENGINEDISKTYPESSD,
    

    /// An alternative to SSD persistent disks that balance performance and cost.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_BALANCED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_BALANCED")]
    COMPUTEENGINEDISKTYPEBALANCED,
}

impl AsRef<str> for ComputeEngineTargetDefaultDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED => "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED",
            ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD => "COMPUTE_ENGINE_DISK_TYPE_STANDARD",
            ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESSD => "COMPUTE_ENGINE_DISK_TYPE_SSD",
            ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED => "COMPUTE_ENGINE_DISK_TYPE_BALANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeEngineTargetDefaultDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED" => Ok(ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED),
           "COMPUTE_ENGINE_DISK_TYPE_STANDARD" => Ok(ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD),
           "COMPUTE_ENGINE_DISK_TYPE_SSD" => Ok(ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESSD),
           "COMPUTE_ENGINE_DISK_TYPE_BALANCED" => Ok(ComputeEngineTargetDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeEngineTargetDefaultDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeEngineTargetDefaultLicenseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The license type to use in OS adaptation.
pub enum ComputeEngineTargetDefaultLicenseTypeEnum {
    

    /// The license type is the default for the OS.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT")]
    COMPUTEENGINELICENSETYPEDEFAULT,
    

    /// The license type is Pay As You Go license type.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_PAYG"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_PAYG")]
    COMPUTEENGINELICENSETYPEPAYG,
    

    /// The license type is Bring Your Own License type.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_BYOL"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_BYOL")]
    COMPUTEENGINELICENSETYPEBYOL,
}

impl AsRef<str> for ComputeEngineTargetDefaultLicenseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeEngineTargetDefaultLicenseTypeEnum::COMPUTEENGINELICENSETYPEDEFAULT => "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT",
            ComputeEngineTargetDefaultLicenseTypeEnum::COMPUTEENGINELICENSETYPEPAYG => "COMPUTE_ENGINE_LICENSE_TYPE_PAYG",
            ComputeEngineTargetDefaultLicenseTypeEnum::COMPUTEENGINELICENSETYPEBYOL => "COMPUTE_ENGINE_LICENSE_TYPE_BYOL",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeEngineTargetDefaultLicenseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT" => Ok(ComputeEngineTargetDefaultLicenseTypeEnum::COMPUTEENGINELICENSETYPEDEFAULT),
           "COMPUTE_ENGINE_LICENSE_TYPE_PAYG" => Ok(ComputeEngineTargetDefaultLicenseTypeEnum::COMPUTEENGINELICENSETYPEPAYG),
           "COMPUTE_ENGINE_LICENSE_TYPE_BYOL" => Ok(ComputeEngineTargetDefaultLicenseTypeEnum::COMPUTEENGINELICENSETYPEBYOL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeEngineTargetDefaultLicenseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeEngineTargetDetailBootOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The VM Boot Option, as set in the source VM.
pub enum ComputeEngineTargetDetailBootOptionEnum {
    

    /// The boot option is unknown.
    ///
    /// "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED"
    #[serde(rename="COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED")]
    COMPUTEENGINEBOOTOPTIONUNSPECIFIED,
    

    /// The boot option is EFI.
    ///
    /// "COMPUTE_ENGINE_BOOT_OPTION_EFI"
    #[serde(rename="COMPUTE_ENGINE_BOOT_OPTION_EFI")]
    COMPUTEENGINEBOOTOPTIONEFI,
    

    /// The boot option is BIOS.
    ///
    /// "COMPUTE_ENGINE_BOOT_OPTION_BIOS"
    #[serde(rename="COMPUTE_ENGINE_BOOT_OPTION_BIOS")]
    COMPUTEENGINEBOOTOPTIONBIOS,
}

impl AsRef<str> for ComputeEngineTargetDetailBootOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeEngineTargetDetailBootOptionEnum::COMPUTEENGINEBOOTOPTIONUNSPECIFIED => "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED",
            ComputeEngineTargetDetailBootOptionEnum::COMPUTEENGINEBOOTOPTIONEFI => "COMPUTE_ENGINE_BOOT_OPTION_EFI",
            ComputeEngineTargetDetailBootOptionEnum::COMPUTEENGINEBOOTOPTIONBIOS => "COMPUTE_ENGINE_BOOT_OPTION_BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeEngineTargetDetailBootOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED" => Ok(ComputeEngineTargetDetailBootOptionEnum::COMPUTEENGINEBOOTOPTIONUNSPECIFIED),
           "COMPUTE_ENGINE_BOOT_OPTION_EFI" => Ok(ComputeEngineTargetDetailBootOptionEnum::COMPUTEENGINEBOOTOPTIONEFI),
           "COMPUTE_ENGINE_BOOT_OPTION_BIOS" => Ok(ComputeEngineTargetDetailBootOptionEnum::COMPUTEENGINEBOOTOPTIONBIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeEngineTargetDetailBootOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeEngineTargetDetailDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The disk type to use in the VM.
pub enum ComputeEngineTargetDetailDiskTypeEnum {
    

    /// An unspecified disk type. Will be used as STANDARD.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED")]
    COMPUTEENGINEDISKTYPEUNSPECIFIED,
    

    /// A Standard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_STANDARD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_STANDARD")]
    COMPUTEENGINEDISKTYPESTANDARD,
    

    /// SSD hard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_SSD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_SSD")]
    COMPUTEENGINEDISKTYPESSD,
    

    /// An alternative to SSD persistent disks that balance performance and cost.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_BALANCED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_BALANCED")]
    COMPUTEENGINEDISKTYPEBALANCED,
}

impl AsRef<str> for ComputeEngineTargetDetailDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED => "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED",
            ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD => "COMPUTE_ENGINE_DISK_TYPE_STANDARD",
            ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPESSD => "COMPUTE_ENGINE_DISK_TYPE_SSD",
            ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED => "COMPUTE_ENGINE_DISK_TYPE_BALANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeEngineTargetDetailDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED" => Ok(ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED),
           "COMPUTE_ENGINE_DISK_TYPE_STANDARD" => Ok(ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD),
           "COMPUTE_ENGINE_DISK_TYPE_SSD" => Ok(ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPESSD),
           "COMPUTE_ENGINE_DISK_TYPE_BALANCED" => Ok(ComputeEngineTargetDetailDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeEngineTargetDetailDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeEngineTargetDetailLicenseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The license type to use in OS adaptation.
pub enum ComputeEngineTargetDetailLicenseTypeEnum {
    

    /// The license type is the default for the OS.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT")]
    COMPUTEENGINELICENSETYPEDEFAULT,
    

    /// The license type is Pay As You Go license type.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_PAYG"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_PAYG")]
    COMPUTEENGINELICENSETYPEPAYG,
    

    /// The license type is Bring Your Own License type.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_BYOL"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_BYOL")]
    COMPUTEENGINELICENSETYPEBYOL,
}

impl AsRef<str> for ComputeEngineTargetDetailLicenseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeEngineTargetDetailLicenseTypeEnum::COMPUTEENGINELICENSETYPEDEFAULT => "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT",
            ComputeEngineTargetDetailLicenseTypeEnum::COMPUTEENGINELICENSETYPEPAYG => "COMPUTE_ENGINE_LICENSE_TYPE_PAYG",
            ComputeEngineTargetDetailLicenseTypeEnum::COMPUTEENGINELICENSETYPEBYOL => "COMPUTE_ENGINE_LICENSE_TYPE_BYOL",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeEngineTargetDetailLicenseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT" => Ok(ComputeEngineTargetDetailLicenseTypeEnum::COMPUTEENGINELICENSETYPEDEFAULT),
           "COMPUTE_ENGINE_LICENSE_TYPE_PAYG" => Ok(ComputeEngineTargetDetailLicenseTypeEnum::COMPUTEENGINELICENSETYPEPAYG),
           "COMPUTE_ENGINE_LICENSE_TYPE_BYOL" => Ok(ComputeEngineTargetDetailLicenseTypeEnum::COMPUTEENGINELICENSETYPEBYOL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeEngineTargetDetailLicenseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeSchedulingOnHostMaintenanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the instance should behave when the host machine undergoes maintenance that may temporarily impact instance performance.
pub enum ComputeSchedulingOnHostMaintenanceEnum {
    

    /// An unknown, unexpected behavior.
    ///
    /// "ON_HOST_MAINTENANCE_UNSPECIFIED"
    #[serde(rename="ON_HOST_MAINTENANCE_UNSPECIFIED")]
    ONHOSTMAINTENANCEUNSPECIFIED,
    

    /// Terminate the instance when the host machine undergoes maintenance.
    ///
    /// "TERMINATE"
    #[serde(rename="TERMINATE")]
    TERMINATE,
    

    /// Migrate the instance when the host machine undergoes maintenance.
    ///
    /// "MIGRATE"
    #[serde(rename="MIGRATE")]
    MIGRATE,
}

impl AsRef<str> for ComputeSchedulingOnHostMaintenanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeSchedulingOnHostMaintenanceEnum::ONHOSTMAINTENANCEUNSPECIFIED => "ON_HOST_MAINTENANCE_UNSPECIFIED",
            ComputeSchedulingOnHostMaintenanceEnum::TERMINATE => "TERMINATE",
            ComputeSchedulingOnHostMaintenanceEnum::MIGRATE => "MIGRATE",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeSchedulingOnHostMaintenanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ON_HOST_MAINTENANCE_UNSPECIFIED" => Ok(ComputeSchedulingOnHostMaintenanceEnum::ONHOSTMAINTENANCEUNSPECIFIED),
           "TERMINATE" => Ok(ComputeSchedulingOnHostMaintenanceEnum::TERMINATE),
           "MIGRATE" => Ok(ComputeSchedulingOnHostMaintenanceEnum::MIGRATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeSchedulingOnHostMaintenanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComputeSchedulingRestartTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the Instance should be automatically restarted whenever it is terminated by Compute Engine (not terminated by user). This configuration is identical to `automaticRestart` field in Compute Engine create instance under scheduling. It was changed to an enum (instead of a boolean) to match the default value in Compute Engine which is automatic restart.
pub enum ComputeSchedulingRestartTypeEnum {
    

    /// Unspecified behavior. This will use the default.
    ///
    /// "RESTART_TYPE_UNSPECIFIED"
    #[serde(rename="RESTART_TYPE_UNSPECIFIED")]
    RESTARTTYPEUNSPECIFIED,
    

    /// The Instance should be automatically restarted whenever it is terminated by Compute Engine.
    ///
    /// "AUTOMATIC_RESTART"
    #[serde(rename="AUTOMATIC_RESTART")]
    AUTOMATICRESTART,
    

    /// The Instance isn't automatically restarted whenever it is terminated by Compute Engine.
    ///
    /// "NO_AUTOMATIC_RESTART"
    #[serde(rename="NO_AUTOMATIC_RESTART")]
    NOAUTOMATICRESTART,
}

impl AsRef<str> for ComputeSchedulingRestartTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComputeSchedulingRestartTypeEnum::RESTARTTYPEUNSPECIFIED => "RESTART_TYPE_UNSPECIFIED",
            ComputeSchedulingRestartTypeEnum::AUTOMATICRESTART => "AUTOMATIC_RESTART",
            ComputeSchedulingRestartTypeEnum::NOAUTOMATICRESTART => "NO_AUTOMATIC_RESTART",
        }
    }
}

impl std::convert::TryFrom< &str> for ComputeSchedulingRestartTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTART_TYPE_UNSPECIFIED" => Ok(ComputeSchedulingRestartTypeEnum::RESTARTTYPEUNSPECIFIED),
           "AUTOMATIC_RESTART" => Ok(ComputeSchedulingRestartTypeEnum::AUTOMATICRESTART),
           "NO_AUTOMATIC_RESTART" => Ok(ComputeSchedulingRestartTypeEnum::NOAUTOMATICRESTART),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComputeSchedulingRestartTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CutoverJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the cutover job.
pub enum CutoverJobStateEnum {
    

    /// The state is unknown. This is used for API compatibility only and is not used by the system.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The cutover job has not yet started.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The cutover job finished with errors.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The cutover job finished successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The cutover job was cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The cutover job is being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The cutover job is active and running.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// OS adaptation is running as part of the cutover job to generate license.
    ///
    /// "ADAPTING_OS"
    #[serde(rename="ADAPTING_OS")]
    ADAPTINGOS,
}

impl AsRef<str> for CutoverJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CutoverJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            CutoverJobStateEnum::PENDING => "PENDING",
            CutoverJobStateEnum::FAILED => "FAILED",
            CutoverJobStateEnum::SUCCEEDED => "SUCCEEDED",
            CutoverJobStateEnum::CANCELLED => "CANCELLED",
            CutoverJobStateEnum::CANCELLING => "CANCELLING",
            CutoverJobStateEnum::ACTIVE => "ACTIVE",
            CutoverJobStateEnum::ADAPTINGOS => "ADAPTING_OS",
        }
    }
}

impl std::convert::TryFrom< &str> for CutoverJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(CutoverJobStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(CutoverJobStateEnum::PENDING),
           "FAILED" => Ok(CutoverJobStateEnum::FAILED),
           "SUCCEEDED" => Ok(CutoverJobStateEnum::SUCCEEDED),
           "CANCELLED" => Ok(CutoverJobStateEnum::CANCELLED),
           "CANCELLING" => Ok(CutoverJobStateEnum::CANCELLING),
           "ACTIVE" => Ok(CutoverJobStateEnum::ACTIVE),
           "ADAPTING_OS" => Ok(CutoverJobStateEnum::ADAPTINGOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CutoverJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatacenterConnectorStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the DatacenterConnector, as determined by the health checks.
pub enum DatacenterConnectorStateEnum {
    

    /// The state is unknown. This is used for API compatibility only and is not used by the system.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The state was not sampled by the health checks yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The source was sampled by health checks and is not available.
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
    

    /// The source is available but might not be usable yet due to unvalidated credentials or another reason. The credentials referred to are the ones to the Source. The error message will contain further details.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The source exists and its credentials were verified.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for DatacenterConnectorStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatacenterConnectorStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            DatacenterConnectorStateEnum::PENDING => "PENDING",
            DatacenterConnectorStateEnum::OFFLINE => "OFFLINE",
            DatacenterConnectorStateEnum::FAILED => "FAILED",
            DatacenterConnectorStateEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for DatacenterConnectorStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(DatacenterConnectorStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(DatacenterConnectorStateEnum::PENDING),
           "OFFLINE" => Ok(DatacenterConnectorStateEnum::OFFLINE),
           "FAILED" => Ok(DatacenterConnectorStateEnum::FAILED),
           "ACTIVE" => Ok(DatacenterConnectorStateEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatacenterConnectorStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupMigrationTargetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The target type of this group.
pub enum GroupMigrationTargetTypeEnum {
    

    /// Group type is not specified. This defaults to Compute Engine targets.
    ///
    /// "MIGRATION_TARGET_TYPE_UNSPECIFIED"
    #[serde(rename="MIGRATION_TARGET_TYPE_UNSPECIFIED")]
    MIGRATIONTARGETTYPEUNSPECIFIED,
    

    /// All MigratingVMs in the group must have Compute Engine targets.
    ///
    /// "MIGRATION_TARGET_TYPE_GCE"
    #[serde(rename="MIGRATION_TARGET_TYPE_GCE")]
    MIGRATIONTARGETTYPEGCE,
    

    /// All MigratingVMs in the group must have Compute Engine Disks targets.
    ///
    /// "MIGRATION_TARGET_TYPE_DISKS"
    #[serde(rename="MIGRATION_TARGET_TYPE_DISKS")]
    MIGRATIONTARGETTYPEDISKS,
}

impl AsRef<str> for GroupMigrationTargetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupMigrationTargetTypeEnum::MIGRATIONTARGETTYPEUNSPECIFIED => "MIGRATION_TARGET_TYPE_UNSPECIFIED",
            GroupMigrationTargetTypeEnum::MIGRATIONTARGETTYPEGCE => "MIGRATION_TARGET_TYPE_GCE",
            GroupMigrationTargetTypeEnum::MIGRATIONTARGETTYPEDISKS => "MIGRATION_TARGET_TYPE_DISKS",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupMigrationTargetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MIGRATION_TARGET_TYPE_UNSPECIFIED" => Ok(GroupMigrationTargetTypeEnum::MIGRATIONTARGETTYPEUNSPECIFIED),
           "MIGRATION_TARGET_TYPE_GCE" => Ok(GroupMigrationTargetTypeEnum::MIGRATIONTARGETTYPEGCE),
           "MIGRATION_TARGET_TYPE_DISKS" => Ok(GroupMigrationTargetTypeEnum::MIGRATIONTARGETTYPEDISKS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupMigrationTargetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImageImportJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the image import.
pub enum ImageImportJobStateEnum {
    

    /// The state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The image import has not yet started.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The image import is active and running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The image import has finished successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The image import has finished with errors.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The image import is being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The image import was cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for ImageImportJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImageImportJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ImageImportJobStateEnum::PENDING => "PENDING",
            ImageImportJobStateEnum::RUNNING => "RUNNING",
            ImageImportJobStateEnum::SUCCEEDED => "SUCCEEDED",
            ImageImportJobStateEnum::FAILED => "FAILED",
            ImageImportJobStateEnum::CANCELLING => "CANCELLING",
            ImageImportJobStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ImageImportJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ImageImportJobStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(ImageImportJobStateEnum::PENDING),
           "RUNNING" => Ok(ImageImportJobStateEnum::RUNNING),
           "SUCCEEDED" => Ok(ImageImportJobStateEnum::SUCCEEDED),
           "FAILED" => Ok(ImageImportJobStateEnum::FAILED),
           "CANCELLING" => Ok(ImageImportJobStateEnum::CANCELLING),
           "CANCELLED" => Ok(ImageImportJobStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImageImportJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ImageImportOsAdaptationParameterLicenseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Choose which type of license to apply to the imported image.
pub enum ImageImportOsAdaptationParameterLicenseTypeEnum {
    

    /// The license type is the default for the OS.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT")]
    COMPUTEENGINELICENSETYPEDEFAULT,
    

    /// The license type is Pay As You Go license type.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_PAYG"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_PAYG")]
    COMPUTEENGINELICENSETYPEPAYG,
    

    /// The license type is Bring Your Own License type.
    ///
    /// "COMPUTE_ENGINE_LICENSE_TYPE_BYOL"
    #[serde(rename="COMPUTE_ENGINE_LICENSE_TYPE_BYOL")]
    COMPUTEENGINELICENSETYPEBYOL,
}

impl AsRef<str> for ImageImportOsAdaptationParameterLicenseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ImageImportOsAdaptationParameterLicenseTypeEnum::COMPUTEENGINELICENSETYPEDEFAULT => "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT",
            ImageImportOsAdaptationParameterLicenseTypeEnum::COMPUTEENGINELICENSETYPEPAYG => "COMPUTE_ENGINE_LICENSE_TYPE_PAYG",
            ImageImportOsAdaptationParameterLicenseTypeEnum::COMPUTEENGINELICENSETYPEBYOL => "COMPUTE_ENGINE_LICENSE_TYPE_BYOL",
        }
    }
}

impl std::convert::TryFrom< &str> for ImageImportOsAdaptationParameterLicenseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT" => Ok(ImageImportOsAdaptationParameterLicenseTypeEnum::COMPUTEENGINELICENSETYPEDEFAULT),
           "COMPUTE_ENGINE_LICENSE_TYPE_PAYG" => Ok(ImageImportOsAdaptationParameterLicenseTypeEnum::COMPUTEENGINELICENSETYPEPAYG),
           "COMPUTE_ENGINE_LICENSE_TYPE_BYOL" => Ok(ImageImportOsAdaptationParameterLicenseTypeEnum::COMPUTEENGINELICENSETYPEBYOL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ImageImportOsAdaptationParameterLicenseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigratingVmStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the MigratingVm.
pub enum MigratingVmStateEnum {
    

    /// The state was not sampled by the health checks yet.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The VM in the source is being verified.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The source VM was verified, and it's ready to start replication.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Migration is going through the first sync cycle.
    ///
    /// "FIRST_SYNC"
    #[serde(rename="FIRST_SYNC")]
    FIRSTSYNC,
    

    /// The replication is active, and it's running or scheduled to run.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The source VM is being turned off, and a final replication is currently running.
    ///
    /// "CUTTING_OVER"
    #[serde(rename="CUTTING_OVER")]
    CUTTINGOVER,
    

    /// The source VM was stopped and replicated. The replication is currently paused.
    ///
    /// "CUTOVER"
    #[serde(rename="CUTOVER")]
    CUTOVER,
    

    /// A cutover job is active and replication cycle is running the final sync.
    ///
    /// "FINAL_SYNC"
    #[serde(rename="FINAL_SYNC")]
    FINALSYNC,
    

    /// The replication was paused by the user and no cycles are scheduled to run.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The migrating VM is being finalized and migration resources are being removed.
    ///
    /// "FINALIZING"
    #[serde(rename="FINALIZING")]
    FINALIZING,
    

    /// The replication process is done. The migrating VM is finalized and no longer consumes billable resources.
    ///
    /// "FINALIZED"
    #[serde(rename="FINALIZED")]
    FINALIZED,
    

    /// The replication process encountered an unrecoverable error and was aborted.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for MigratingVmStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigratingVmStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MigratingVmStateEnum::PENDING => "PENDING",
            MigratingVmStateEnum::READY => "READY",
            MigratingVmStateEnum::FIRSTSYNC => "FIRST_SYNC",
            MigratingVmStateEnum::ACTIVE => "ACTIVE",
            MigratingVmStateEnum::CUTTINGOVER => "CUTTING_OVER",
            MigratingVmStateEnum::CUTOVER => "CUTOVER",
            MigratingVmStateEnum::FINALSYNC => "FINAL_SYNC",
            MigratingVmStateEnum::PAUSED => "PAUSED",
            MigratingVmStateEnum::FINALIZING => "FINALIZING",
            MigratingVmStateEnum::FINALIZED => "FINALIZED",
            MigratingVmStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for MigratingVmStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MigratingVmStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(MigratingVmStateEnum::PENDING),
           "READY" => Ok(MigratingVmStateEnum::READY),
           "FIRST_SYNC" => Ok(MigratingVmStateEnum::FIRSTSYNC),
           "ACTIVE" => Ok(MigratingVmStateEnum::ACTIVE),
           "CUTTING_OVER" => Ok(MigratingVmStateEnum::CUTTINGOVER),
           "CUTOVER" => Ok(MigratingVmStateEnum::CUTOVER),
           "FINAL_SYNC" => Ok(MigratingVmStateEnum::FINALSYNC),
           "PAUSED" => Ok(MigratingVmStateEnum::PAUSED),
           "FINALIZING" => Ok(MigratingVmStateEnum::FINALIZING),
           "FINALIZED" => Ok(MigratingVmStateEnum::FINALIZED),
           "ERROR" => Ok(MigratingVmStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigratingVmStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationWarningCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The warning code.
pub enum MigrationWarningCodeEnum {
    

    /// Default value. This value is not used.
    ///
    /// "WARNING_CODE_UNSPECIFIED"
    #[serde(rename="WARNING_CODE_UNSPECIFIED")]
    WARNINGCODEUNSPECIFIED,
    

    /// A warning originated from OS Adaptation.
    ///
    /// "ADAPTATION_WARNING"
    #[serde(rename="ADAPTATION_WARNING")]
    ADAPTATIONWARNING,
}

impl AsRef<str> for MigrationWarningCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationWarningCodeEnum::WARNINGCODEUNSPECIFIED => "WARNING_CODE_UNSPECIFIED",
            MigrationWarningCodeEnum::ADAPTATIONWARNING => "ADAPTATION_WARNING",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationWarningCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WARNING_CODE_UNSPECIFIED" => Ok(MigrationWarningCodeEnum::WARNINGCODEUNSPECIFIED),
           "ADAPTATION_WARNING" => Ok(MigrationWarningCodeEnum::ADAPTATIONWARNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationWarningCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersistentDiskDefaultDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The disk type to use.
pub enum PersistentDiskDefaultDiskTypeEnum {
    

    /// An unspecified disk type. Will be used as STANDARD.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED")]
    COMPUTEENGINEDISKTYPEUNSPECIFIED,
    

    /// A Standard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_STANDARD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_STANDARD")]
    COMPUTEENGINEDISKTYPESTANDARD,
    

    /// SSD hard disk type.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_SSD"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_SSD")]
    COMPUTEENGINEDISKTYPESSD,
    

    /// An alternative to SSD persistent disks that balance performance and cost.
    ///
    /// "COMPUTE_ENGINE_DISK_TYPE_BALANCED"
    #[serde(rename="COMPUTE_ENGINE_DISK_TYPE_BALANCED")]
    COMPUTEENGINEDISKTYPEBALANCED,
}

impl AsRef<str> for PersistentDiskDefaultDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED => "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED",
            PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD => "COMPUTE_ENGINE_DISK_TYPE_STANDARD",
            PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESSD => "COMPUTE_ENGINE_DISK_TYPE_SSD",
            PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED => "COMPUTE_ENGINE_DISK_TYPE_BALANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for PersistentDiskDefaultDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED" => Ok(PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEUNSPECIFIED),
           "COMPUTE_ENGINE_DISK_TYPE_STANDARD" => Ok(PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESTANDARD),
           "COMPUTE_ENGINE_DISK_TYPE_SSD" => Ok(PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPESSD),
           "COMPUTE_ENGINE_DISK_TYPE_BALANCED" => Ok(PersistentDiskDefaultDiskTypeEnum::COMPUTEENGINEDISKTYPEBALANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersistentDiskDefaultDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReplicationCycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the ReplicationCycle.
pub enum ReplicationCycleStateEnum {
    

    /// The state is unknown. This is used for API compatibility only and is not used by the system.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The replication cycle is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The replication cycle is paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The replication cycle finished with errors.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The replication cycle finished successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
}

impl AsRef<str> for ReplicationCycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReplicationCycleStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ReplicationCycleStateEnum::RUNNING => "RUNNING",
            ReplicationCycleStateEnum::PAUSED => "PAUSED",
            ReplicationCycleStateEnum::FAILED => "FAILED",
            ReplicationCycleStateEnum::SUCCEEDED => "SUCCEEDED",
        }
    }
}

impl std::convert::TryFrom< &str> for ReplicationCycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ReplicationCycleStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(ReplicationCycleStateEnum::RUNNING),
           "PAUSED" => Ok(ReplicationCycleStateEnum::PAUSED),
           "FAILED" => Ok(ReplicationCycleStateEnum::FAILED),
           "SUCCEEDED" => Ok(ReplicationCycleStateEnum::SUCCEEDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReplicationCycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchedulingNodeAffinityOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to use for the node resources specified in the `values` parameter.
pub enum SchedulingNodeAffinityOperatorEnum {
    

    /// An unknown, unexpected behavior.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The node resource group should be in these resources affinity.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// The node resource group should not be in these resources affinity.
    ///
    /// "NOT_IN"
    #[serde(rename="NOT_IN")]
    NOTIN,
}

impl AsRef<str> for SchedulingNodeAffinityOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchedulingNodeAffinityOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            SchedulingNodeAffinityOperatorEnum::IN => "IN",
            SchedulingNodeAffinityOperatorEnum::NOTIN => "NOT_IN",
        }
    }
}

impl std::convert::TryFrom< &str> for SchedulingNodeAffinityOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(SchedulingNodeAffinityOperatorEnum::OPERATORUNSPECIFIED),
           "IN" => Ok(SchedulingNodeAffinityOperatorEnum::IN),
           "NOT_IN" => Ok(SchedulingNodeAffinityOperatorEnum::NOTIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchedulingNodeAffinityOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpgradeStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the upgradeAppliance operation.
pub enum UpgradeStatusStateEnum {
    

    /// The state was not sampled by the health checks yet.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The upgrade has started.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The upgrade failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The upgrade finished successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
}

impl AsRef<str> for UpgradeStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpgradeStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            UpgradeStatusStateEnum::RUNNING => "RUNNING",
            UpgradeStatusStateEnum::FAILED => "FAILED",
            UpgradeStatusStateEnum::SUCCEEDED => "SUCCEEDED",
        }
    }
}

impl std::convert::TryFrom< &str> for UpgradeStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(UpgradeStatusStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(UpgradeStatusStateEnum::RUNNING),
           "FAILED" => Ok(UpgradeStatusStateEnum::FAILED),
           "SUCCEEDED" => Ok(UpgradeStatusStateEnum::SUCCEEDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpgradeStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UtilizationReportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the report.
pub enum UtilizationReportStateEnum {
    

    /// The state is unknown. This value is not in use.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The report is in the making.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Report creation completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Report creation failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for UtilizationReportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UtilizationReportStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            UtilizationReportStateEnum::CREATING => "CREATING",
            UtilizationReportStateEnum::SUCCEEDED => "SUCCEEDED",
            UtilizationReportStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for UtilizationReportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(UtilizationReportStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(UtilizationReportStateEnum::CREATING),
           "SUCCEEDED" => Ok(UtilizationReportStateEnum::SUCCEEDED),
           "FAILED" => Ok(UtilizationReportStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UtilizationReportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UtilizationReportTimeFrameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Time frame of the report.
pub enum UtilizationReportTimeFrameEnum {
    

    /// The time frame was not specified and will default to WEEK.
    ///
    /// "TIME_FRAME_UNSPECIFIED"
    #[serde(rename="TIME_FRAME_UNSPECIFIED")]
    TIMEFRAMEUNSPECIFIED,
    

    /// One week.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// One month.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// One year.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
}

impl AsRef<str> for UtilizationReportTimeFrameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UtilizationReportTimeFrameEnum::TIMEFRAMEUNSPECIFIED => "TIME_FRAME_UNSPECIFIED",
            UtilizationReportTimeFrameEnum::WEEK => "WEEK",
            UtilizationReportTimeFrameEnum::MONTH => "MONTH",
            UtilizationReportTimeFrameEnum::YEAR => "YEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for UtilizationReportTimeFrameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_FRAME_UNSPECIFIED" => Ok(UtilizationReportTimeFrameEnum::TIMEFRAMEUNSPECIFIED),
           "WEEK" => Ok(UtilizationReportTimeFrameEnum::WEEK),
           "MONTH" => Ok(UtilizationReportTimeFrameEnum::MONTH),
           "YEAR" => Ok(UtilizationReportTimeFrameEnum::YEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UtilizationReportTimeFrameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VmCapabilityOsCapabilitiesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Unordered list. List of certain VM OS capabilities needed for some Compute Engine features.
pub enum VmCapabilityOsCapabilitiesEnum {
    

    /// This is for API compatibility only and is not in use.
    ///
    /// "OS_CAPABILITY_UNSPECIFIED"
    #[serde(rename="OS_CAPABILITY_UNSPECIFIED")]
    OSCAPABILITYUNSPECIFIED,
    

    /// NVMe driver installed and the VM can use NVMe PD or local SSD.
    ///
    /// "OS_CAPABILITY_NVME_STORAGE_ACCESS"
    #[serde(rename="OS_CAPABILITY_NVME_STORAGE_ACCESS")]
    OSCAPABILITYNVMESTORAGEACCESS,
    

    /// gVNIC virtual NIC driver supported.
    ///
    /// "OS_CAPABILITY_GVNIC_NETWORK_INTERFACE"
    #[serde(rename="OS_CAPABILITY_GVNIC_NETWORK_INTERFACE")]
    OSCAPABILITYGVNICNETWORKINTERFACE,
}

impl AsRef<str> for VmCapabilityOsCapabilitiesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VmCapabilityOsCapabilitiesEnum::OSCAPABILITYUNSPECIFIED => "OS_CAPABILITY_UNSPECIFIED",
            VmCapabilityOsCapabilitiesEnum::OSCAPABILITYNVMESTORAGEACCESS => "OS_CAPABILITY_NVME_STORAGE_ACCESS",
            VmCapabilityOsCapabilitiesEnum::OSCAPABILITYGVNICNETWORKINTERFACE => "OS_CAPABILITY_GVNIC_NETWORK_INTERFACE",
        }
    }
}

impl std::convert::TryFrom< &str> for VmCapabilityOsCapabilitiesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OS_CAPABILITY_UNSPECIFIED" => Ok(VmCapabilityOsCapabilitiesEnum::OSCAPABILITYUNSPECIFIED),
           "OS_CAPABILITY_NVME_STORAGE_ACCESS" => Ok(VmCapabilityOsCapabilitiesEnum::OSCAPABILITYNVMESTORAGEACCESS),
           "OS_CAPABILITY_GVNIC_NETWORK_INTERFACE" => Ok(VmCapabilityOsCapabilitiesEnum::OSCAPABILITYGVNICNETWORKINTERFACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VmCapabilityOsCapabilitiesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VmwareSourceVmDetailFirmwareEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The firmware type of the source VM.
pub enum VmwareSourceVmDetailFirmwareEnum {
    

    /// The firmware is unknown.
    ///
    /// "FIRMWARE_UNSPECIFIED"
    #[serde(rename="FIRMWARE_UNSPECIFIED")]
    FIRMWAREUNSPECIFIED,
    

    /// The firmware is EFI.
    ///
    /// "EFI"
    #[serde(rename="EFI")]
    EFI,
    

    /// The firmware is BIOS.
    ///
    /// "BIOS"
    #[serde(rename="BIOS")]
    BIOS,
}

impl AsRef<str> for VmwareSourceVmDetailFirmwareEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VmwareSourceVmDetailFirmwareEnum::FIRMWAREUNSPECIFIED => "FIRMWARE_UNSPECIFIED",
            VmwareSourceVmDetailFirmwareEnum::EFI => "EFI",
            VmwareSourceVmDetailFirmwareEnum::BIOS => "BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for VmwareSourceVmDetailFirmwareEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FIRMWARE_UNSPECIFIED" => Ok(VmwareSourceVmDetailFirmwareEnum::FIRMWAREUNSPECIFIED),
           "EFI" => Ok(VmwareSourceVmDetailFirmwareEnum::EFI),
           "BIOS" => Ok(VmwareSourceVmDetailFirmwareEnum::BIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VmwareSourceVmDetailFirmwareEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VmwareVmDetailBootOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The VM Boot Option.
pub enum VmwareVmDetailBootOptionEnum {
    

    /// The boot option is unknown.
    ///
    /// "BOOT_OPTION_UNSPECIFIED"
    #[serde(rename="BOOT_OPTION_UNSPECIFIED")]
    BOOTOPTIONUNSPECIFIED,
    

    /// The boot option is EFI.
    ///
    /// "EFI"
    #[serde(rename="EFI")]
    EFI,
    

    /// The boot option is BIOS.
    ///
    /// "BIOS"
    #[serde(rename="BIOS")]
    BIOS,
}

impl AsRef<str> for VmwareVmDetailBootOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VmwareVmDetailBootOptionEnum::BOOTOPTIONUNSPECIFIED => "BOOT_OPTION_UNSPECIFIED",
            VmwareVmDetailBootOptionEnum::EFI => "EFI",
            VmwareVmDetailBootOptionEnum::BIOS => "BIOS",
        }
    }
}

impl std::convert::TryFrom< &str> for VmwareVmDetailBootOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BOOT_OPTION_UNSPECIFIED" => Ok(VmwareVmDetailBootOptionEnum::BOOTOPTIONUNSPECIFIED),
           "EFI" => Ok(VmwareVmDetailBootOptionEnum::EFI),
           "BIOS" => Ok(VmwareVmDetailBootOptionEnum::BIOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VmwareVmDetailBootOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VmwareVmDetailPowerStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The power state of the VM at the moment list was taken.
pub enum VmwareVmDetailPowerStateEnum {
    

    /// Power state is not specified.
    ///
    /// "POWER_STATE_UNSPECIFIED"
    #[serde(rename="POWER_STATE_UNSPECIFIED")]
    POWERSTATEUNSPECIFIED,
    

    /// The VM is turned ON.
    ///
    /// "ON"
    #[serde(rename="ON")]
    ON,
    

    /// The VM is turned OFF.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// The VM is suspended. This is similar to hibernation or sleep mode.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for VmwareVmDetailPowerStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VmwareVmDetailPowerStateEnum::POWERSTATEUNSPECIFIED => "POWER_STATE_UNSPECIFIED",
            VmwareVmDetailPowerStateEnum::ON => "ON",
            VmwareVmDetailPowerStateEnum::OFF => "OFF",
            VmwareVmDetailPowerStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for VmwareVmDetailPowerStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POWER_STATE_UNSPECIFIED" => Ok(VmwareVmDetailPowerStateEnum::POWERSTATEUNSPECIFIED),
           "ON" => Ok(VmwareVmDetailPowerStateEnum::ON),
           "OFF" => Ok(VmwareVmDetailPowerStateEnum::OFF),
           "SUSPENDED" => Ok(VmwareVmDetailPowerStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VmwareVmDetailPowerStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The level of details of each report. Defaults to BASIC.
pub enum ProjectViewEnum {
    

    /// The default / unset value. The API will default to FULL on single report request and BASIC for multiple reports request.
    ///
    /// "UTILIZATION_REPORT_VIEW_UNSPECIFIED"
    #[serde(rename="UTILIZATION_REPORT_VIEW_UNSPECIFIED")]
    UTILIZATIONREPORTVIEWUNSPECIFIED,
    

    /// Get the report metadata, without the list of VMs and their utilization info.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include everything.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::UTILIZATIONREPORTVIEWUNSPECIFIED => "UTILIZATION_REPORT_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UTILIZATION_REPORT_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::UTILIZATIONREPORTVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
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


