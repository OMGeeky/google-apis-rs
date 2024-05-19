use super::*;



// region BasicLevelCombiningFunctionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND.
pub enum BasicLevelCombiningFunctionEnum {
    

    /// All `Conditions` must be true for the `BasicLevel` to be true.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
    

    /// If at least one `Condition` is true, then the `BasicLevel` is true.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
}

impl AsRef<str> for BasicLevelCombiningFunctionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BasicLevelCombiningFunctionEnum::AND => "AND",
            BasicLevelCombiningFunctionEnum::OR => "OR",
        }
    }
}

impl std::convert::TryFrom< &str> for BasicLevelCombiningFunctionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AND" => Ok(BasicLevelCombiningFunctionEnum::AND),
           "OR" => Ok(BasicLevelCombiningFunctionEnum::OR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BasicLevelCombiningFunctionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DevicePolicyAllowedDeviceManagementLevelsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed device management levels, an empty list allows all management levels.
pub enum DevicePolicyAllowedDeviceManagementLevelsEnum {
    

    /// The device's management level is not specified or not known.
    ///
    /// "MANAGEMENT_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_UNSPECIFIED")]
    MANAGEMENTUNSPECIFIED,
    

    /// The device is not managed.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Basic management is enabled, which is generally limited to monitoring and wiping the corporate account.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Complete device management. This includes more thorough monitoring and the ability to directly manage the device (such as remote wiping). This can be enabled through the Android Enterprise Platform.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
}

impl AsRef<str> for DevicePolicyAllowedDeviceManagementLevelsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DevicePolicyAllowedDeviceManagementLevelsEnum::MANAGEMENTUNSPECIFIED => "MANAGEMENT_UNSPECIFIED",
            DevicePolicyAllowedDeviceManagementLevelsEnum::NONE => "NONE",
            DevicePolicyAllowedDeviceManagementLevelsEnum::BASIC => "BASIC",
            DevicePolicyAllowedDeviceManagementLevelsEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for DevicePolicyAllowedDeviceManagementLevelsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_UNSPECIFIED" => Ok(DevicePolicyAllowedDeviceManagementLevelsEnum::MANAGEMENTUNSPECIFIED),
           "NONE" => Ok(DevicePolicyAllowedDeviceManagementLevelsEnum::NONE),
           "BASIC" => Ok(DevicePolicyAllowedDeviceManagementLevelsEnum::BASIC),
           "COMPLETE" => Ok(DevicePolicyAllowedDeviceManagementLevelsEnum::COMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DevicePolicyAllowedDeviceManagementLevelsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DevicePolicyAllowedEncryptionStatusesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed encryptions statuses, an empty list allows all statuses.
pub enum DevicePolicyAllowedEncryptionStatusesEnum {
    

    /// The encryption status of the device is not specified or not known.
    ///
    /// "ENCRYPTION_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_UNSPECIFIED")]
    ENCRYPTIONUNSPECIFIED,
    

    /// The device does not support encryption.
    ///
    /// "ENCRYPTION_UNSUPPORTED"
    #[serde(rename="ENCRYPTION_UNSUPPORTED")]
    ENCRYPTIONUNSUPPORTED,
    

    /// The device supports encryption, but is currently unencrypted.
    ///
    /// "UNENCRYPTED"
    #[serde(rename="UNENCRYPTED")]
    UNENCRYPTED,
    

    /// The device is encrypted.
    ///
    /// "ENCRYPTED"
    #[serde(rename="ENCRYPTED")]
    ENCRYPTED,
}

impl AsRef<str> for DevicePolicyAllowedEncryptionStatusesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSPECIFIED => "ENCRYPTION_UNSPECIFIED",
            DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSUPPORTED => "ENCRYPTION_UNSUPPORTED",
            DevicePolicyAllowedEncryptionStatusesEnum::UNENCRYPTED => "UNENCRYPTED",
            DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTED => "ENCRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DevicePolicyAllowedEncryptionStatusesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_UNSPECIFIED" => Ok(DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSPECIFIED),
           "ENCRYPTION_UNSUPPORTED" => Ok(DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSUPPORTED),
           "UNENCRYPTED" => Ok(DevicePolicyAllowedEncryptionStatusesEnum::UNENCRYPTED),
           "ENCRYPTED" => Ok(DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DevicePolicyAllowedEncryptionStatusesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OsConstraintOsTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The allowed OS type.
pub enum OsConstraintOsTypeEnum {
    

    /// The operating system of the device is not specified or not known.
    ///
    /// "OS_UNSPECIFIED"
    #[serde(rename="OS_UNSPECIFIED")]
    OSUNSPECIFIED,
    

    /// A desktop Mac operating system.
    ///
    /// "DESKTOP_MAC"
    #[serde(rename="DESKTOP_MAC")]
    DESKTOPMAC,
    

    /// A desktop Windows operating system.
    ///
    /// "DESKTOP_WINDOWS"
    #[serde(rename="DESKTOP_WINDOWS")]
    DESKTOPWINDOWS,
    

    /// A desktop Linux operating system.
    ///
    /// "DESKTOP_LINUX"
    #[serde(rename="DESKTOP_LINUX")]
    DESKTOPLINUX,
    

    /// A desktop ChromeOS operating system.
    ///
    /// "DESKTOP_CHROME_OS"
    #[serde(rename="DESKTOP_CHROME_OS")]
    DESKTOPCHROMEOS,
    

    /// An Android operating system.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// An iOS operating system.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
}

impl AsRef<str> for OsConstraintOsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OsConstraintOsTypeEnum::OSUNSPECIFIED => "OS_UNSPECIFIED",
            OsConstraintOsTypeEnum::DESKTOPMAC => "DESKTOP_MAC",
            OsConstraintOsTypeEnum::DESKTOPWINDOWS => "DESKTOP_WINDOWS",
            OsConstraintOsTypeEnum::DESKTOPLINUX => "DESKTOP_LINUX",
            OsConstraintOsTypeEnum::DESKTOPCHROMEOS => "DESKTOP_CHROME_OS",
            OsConstraintOsTypeEnum::ANDROID => "ANDROID",
            OsConstraintOsTypeEnum::IOS => "IOS",
        }
    }
}

impl std::convert::TryFrom< &str> for OsConstraintOsTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OS_UNSPECIFIED" => Ok(OsConstraintOsTypeEnum::OSUNSPECIFIED),
           "DESKTOP_MAC" => Ok(OsConstraintOsTypeEnum::DESKTOPMAC),
           "DESKTOP_WINDOWS" => Ok(OsConstraintOsTypeEnum::DESKTOPWINDOWS),
           "DESKTOP_LINUX" => Ok(OsConstraintOsTypeEnum::DESKTOPLINUX),
           "DESKTOP_CHROME_OS" => Ok(OsConstraintOsTypeEnum::DESKTOPCHROMEOS),
           "ANDROID" => Ok(OsConstraintOsTypeEnum::ANDROID),
           "IOS" => Ok(OsConstraintOsTypeEnum::IOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OsConstraintOsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServicePerimeterPerimeterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, restricted/unrestricted service lists as well as access lists must be empty.
pub enum ServicePerimeterPerimeterTypeEnum {
    

    /// Regular Perimeter. When no value is specified, the perimeter uses this type.
    ///
    /// "PERIMETER_TYPE_REGULAR"
    #[serde(rename="PERIMETER_TYPE_REGULAR")]
    PERIMETERTYPEREGULAR,
    

    /// Perimeter Bridge.
    ///
    /// "PERIMETER_TYPE_BRIDGE"
    #[serde(rename="PERIMETER_TYPE_BRIDGE")]
    PERIMETERTYPEBRIDGE,
}

impl AsRef<str> for ServicePerimeterPerimeterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEREGULAR => "PERIMETER_TYPE_REGULAR",
            ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEBRIDGE => "PERIMETER_TYPE_BRIDGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ServicePerimeterPerimeterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIMETER_TYPE_REGULAR" => Ok(ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEREGULAR),
           "PERIMETER_TYPE_BRIDGE" => Ok(ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEBRIDGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServicePerimeterPerimeterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccessPolicyAccessLevelFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether to return `BasicLevels` in the Cloud Common Expression language, as `CustomLevels`, rather than as `BasicLevels`. Defaults to returning `AccessLevels` in the format they were defined.
pub enum AccessPolicyAccessLevelFormatEnum {
    

    /// The format was not specified.
    ///
    /// "LEVEL_FORMAT_UNSPECIFIED"
    #[serde(rename="LEVEL_FORMAT_UNSPECIFIED")]
    LEVELFORMATUNSPECIFIED,
    

    /// Uses the format the resource was defined in. BasicLevels are returned as BasicLevels, CustomLevels are returned as CustomLevels.
    ///
    /// "AS_DEFINED"
    #[serde(rename="AS_DEFINED")]
    ASDEFINED,
    

    /// Use Cloud Common Expression Language when returning the resource. Both BasicLevels and CustomLevels are returned as CustomLevels.
    ///
    /// "CEL"
    #[serde(rename="CEL")]
    CEL,
}

impl AsRef<str> for AccessPolicyAccessLevelFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccessPolicyAccessLevelFormatEnum::LEVELFORMATUNSPECIFIED => "LEVEL_FORMAT_UNSPECIFIED",
            AccessPolicyAccessLevelFormatEnum::ASDEFINED => "AS_DEFINED",
            AccessPolicyAccessLevelFormatEnum::CEL => "CEL",
        }
    }
}

impl std::convert::TryFrom< &str> for AccessPolicyAccessLevelFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEVEL_FORMAT_UNSPECIFIED" => Ok(AccessPolicyAccessLevelFormatEnum::LEVELFORMATUNSPECIFIED),
           "AS_DEFINED" => Ok(AccessPolicyAccessLevelFormatEnum::ASDEFINED),
           "CEL" => Ok(AccessPolicyAccessLevelFormatEnum::CEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccessPolicyAccessLevelFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


