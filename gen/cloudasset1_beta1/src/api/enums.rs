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


// region ExportAssetsRequestContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Asset content type. If not specified, no content but the asset name will be returned.
pub enum ExportAssetsRequestContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
}

impl AsRef<str> for ExportAssetsRequestContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportAssetsRequestContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            ExportAssetsRequestContentTypeEnum::RESOURCE => "RESOURCE",
            ExportAssetsRequestContentTypeEnum::IAMPOLICY => "IAM_POLICY",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportAssetsRequestContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(ExportAssetsRequestContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(ExportAssetsRequestContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(ExportAssetsRequestContentTypeEnum::IAMPOLICY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportAssetsRequestContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The policy all_values state.
pub enum GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    

    /// Indicates that allowed_values or denied_values must be set.
    ///
    /// "ALL_VALUES_UNSPECIFIED"
    #[serde(rename="ALL_VALUES_UNSPECIFIED")]
    ALLVALUESUNSPECIFIED,
    

    /// A policy with this set allows all values.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// A policy with this set denies all values.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLVALUESUNSPECIFIED => "ALL_VALUES_UNSPECIFIED",
            GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLOW => "ALLOW",
            GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_VALUES_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLVALUESUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLOW),
           "DENY" => Ok(GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND.
pub enum GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    

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

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::AND => "AND",
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::OR => "OR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AND" => Ok(GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::AND),
           "OR" => Ok(GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::OR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed device management levels, an empty list allows all management levels.
pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    

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

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::MANAGEMENTUNSPECIFIED => "MANAGEMENT_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::NONE => "NONE",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::BASIC => "BASIC",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::MANAGEMENTUNSPECIFIED),
           "NONE" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::NONE),
           "BASIC" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::BASIC),
           "COMPLETE" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::COMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed encryptions statuses, an empty list allows all statuses.
pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    

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

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSPECIFIED => "ENCRYPTION_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSUPPORTED => "ENCRYPTION_UNSUPPORTED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::UNENCRYPTED => "UNENCRYPTED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTED => "ENCRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSPECIFIED),
           "ENCRYPTION_UNSUPPORTED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSUPPORTED),
           "UNENCRYPTED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::UNENCRYPTED),
           "ENCRYPTED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
pub enum GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    

    /// No blanket identity group specified.
    ///
    /// "IDENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="IDENTITY_TYPE_UNSPECIFIED")]
    IDENTITYTYPEUNSPECIFIED,
    

    /// Authorize access from all identities outside the perimeter.
    ///
    /// "ANY_IDENTITY"
    #[serde(rename="ANY_IDENTITY")]
    ANYIDENTITY,
    

    /// Authorize access from all human users outside the perimeter.
    ///
    /// "ANY_USER_ACCOUNT"
    #[serde(rename="ANY_USER_ACCOUNT")]
    ANYUSERACCOUNT,
    

    /// Authorize access from all service accounts outside the perimeter.
    ///
    /// "ANY_SERVICE_ACCOUNT"
    #[serde(rename="ANY_SERVICE_ACCOUNT")]
    ANYSERVICEACCOUNT,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED => "IDENTITY_TYPE_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYIDENTITY => "ANY_IDENTITY",
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYUSERACCOUNT => "ANY_USER_ACCOUNT",
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYSERVICEACCOUNT => "ANY_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY_TYPE_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED),
           "ANY_IDENTITY" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYIDENTITY),
           "ANY_USER_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYUSERACCOUNT),
           "ANY_SERVICE_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether to enforce traffic restrictions based on `sources` field. If the `sources` fields is non-empty, then this field must be set to `SOURCE_RESTRICTION_ENABLED`.
pub enum GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    

    /// Enforcement preference unspecified, will not enforce traffic restrictions based on `sources` in EgressFrom.
    ///
    /// "SOURCE_RESTRICTION_UNSPECIFIED"
    #[serde(rename="SOURCE_RESTRICTION_UNSPECIFIED")]
    SOURCERESTRICTIONUNSPECIFIED,
    

    /// Enforcement preference enabled, traffic restrictions will be enforced based on `sources` in EgressFrom.
    ///
    /// "SOURCE_RESTRICTION_ENABLED"
    #[serde(rename="SOURCE_RESTRICTION_ENABLED")]
    SOURCERESTRICTIONENABLED,
    

    /// Enforcement preference disabled, will not enforce traffic restrictions based on `sources` in EgressFrom.
    ///
    /// "SOURCE_RESTRICTION_DISABLED"
    #[serde(rename="SOURCE_RESTRICTION_DISABLED")]
    SOURCERESTRICTIONDISABLED,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONUNSPECIFIED => "SOURCE_RESTRICTION_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONENABLED => "SOURCE_RESTRICTION_ENABLED",
            GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONDISABLED => "SOURCE_RESTRICTION_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOURCE_RESTRICTION_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONUNSPECIFIED),
           "SOURCE_RESTRICTION_ENABLED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONENABLED),
           "SOURCE_RESTRICTION_DISABLED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
pub enum GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    

    /// No blanket identity group specified.
    ///
    /// "IDENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="IDENTITY_TYPE_UNSPECIFIED")]
    IDENTITYTYPEUNSPECIFIED,
    

    /// Authorize access from all identities outside the perimeter.
    ///
    /// "ANY_IDENTITY"
    #[serde(rename="ANY_IDENTITY")]
    ANYIDENTITY,
    

    /// Authorize access from all human users outside the perimeter.
    ///
    /// "ANY_USER_ACCOUNT"
    #[serde(rename="ANY_USER_ACCOUNT")]
    ANYUSERACCOUNT,
    

    /// Authorize access from all service accounts outside the perimeter.
    ///
    /// "ANY_SERVICE_ACCOUNT"
    #[serde(rename="ANY_SERVICE_ACCOUNT")]
    ANYSERVICEACCOUNT,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED => "IDENTITY_TYPE_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYIDENTITY => "ANY_IDENTITY",
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYUSERACCOUNT => "ANY_USER_ACCOUNT",
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYSERVICEACCOUNT => "ANY_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY_TYPE_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED),
           "ANY_IDENTITY" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYIDENTITY),
           "ANY_USER_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYUSERACCOUNT),
           "ANY_SERVICE_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The allowed OS type.
pub enum GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    

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

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::OSUNSPECIFIED => "OS_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPMAC => "DESKTOP_MAC",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPWINDOWS => "DESKTOP_WINDOWS",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPLINUX => "DESKTOP_LINUX",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPCHROMEOS => "DESKTOP_CHROME_OS",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::ANDROID => "ANDROID",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::IOS => "IOS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OS_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::OSUNSPECIFIED),
           "DESKTOP_MAC" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPMAC),
           "DESKTOP_WINDOWS" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPWINDOWS),
           "DESKTOP_LINUX" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPLINUX),
           "DESKTOP_CHROME_OS" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPCHROMEOS),
           "ANDROID" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::ANDROID),
           "IOS" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::IOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Perimeter type indicator. A single project or VPC network is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty.
pub enum GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    

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

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEREGULAR => "PERIMETER_TYPE_REGULAR",
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEBRIDGE => "PERIMETER_TYPE_BRIDGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIMETER_TYPE_REGULAR" => Ok(GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEREGULAR),
           "PERIMETER_TYPE_BRIDGE" => Ok(GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEBRIDGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrganizationContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The content type.
pub enum OrganizationContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
}

impl AsRef<str> for OrganizationContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrganizationContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            OrganizationContentTypeEnum::RESOURCE => "RESOURCE",
            OrganizationContentTypeEnum::IAMPOLICY => "IAM_POLICY",
        }
    }
}

impl std::convert::TryFrom< &str> for OrganizationContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(OrganizationContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(OrganizationContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(OrganizationContentTypeEnum::IAMPOLICY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrganizationContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The content type.
pub enum ProjectContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
}

impl AsRef<str> for ProjectContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            ProjectContentTypeEnum::RESOURCE => "RESOURCE",
            ProjectContentTypeEnum::IAMPOLICY => "IAM_POLICY",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(ProjectContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(ProjectContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(ProjectContentTypeEnum::IAMPOLICY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


