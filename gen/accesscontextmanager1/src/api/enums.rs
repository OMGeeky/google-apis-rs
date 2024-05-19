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


// region AuthorizedOrgsDescAssetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The asset type of this authorized orgs desc. Valid values are `ASSET_TYPE_DEVICE`, and `ASSET_TYPE_CREDENTIAL_STRENGTH`.
pub enum AuthorizedOrgsDescAssetTypeEnum {
    

    /// No asset type specified.
    ///
    /// "ASSET_TYPE_UNSPECIFIED"
    #[serde(rename="ASSET_TYPE_UNSPECIFIED")]
    ASSETTYPEUNSPECIFIED,
    

    /// Device asset type.
    ///
    /// "ASSET_TYPE_DEVICE"
    #[serde(rename="ASSET_TYPE_DEVICE")]
    ASSETTYPEDEVICE,
    

    /// Credential strength asset type.
    ///
    /// "ASSET_TYPE_CREDENTIAL_STRENGTH"
    #[serde(rename="ASSET_TYPE_CREDENTIAL_STRENGTH")]
    ASSETTYPECREDENTIALSTRENGTH,
}

impl AsRef<str> for AuthorizedOrgsDescAssetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizedOrgsDescAssetTypeEnum::ASSETTYPEUNSPECIFIED => "ASSET_TYPE_UNSPECIFIED",
            AuthorizedOrgsDescAssetTypeEnum::ASSETTYPEDEVICE => "ASSET_TYPE_DEVICE",
            AuthorizedOrgsDescAssetTypeEnum::ASSETTYPECREDENTIALSTRENGTH => "ASSET_TYPE_CREDENTIAL_STRENGTH",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizedOrgsDescAssetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSET_TYPE_UNSPECIFIED" => Ok(AuthorizedOrgsDescAssetTypeEnum::ASSETTYPEUNSPECIFIED),
           "ASSET_TYPE_DEVICE" => Ok(AuthorizedOrgsDescAssetTypeEnum::ASSETTYPEDEVICE),
           "ASSET_TYPE_CREDENTIAL_STRENGTH" => Ok(AuthorizedOrgsDescAssetTypeEnum::ASSETTYPECREDENTIALSTRENGTH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizedOrgsDescAssetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorizedOrgsDescAuthorizationDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The direction of the authorization relationship between this organization and the organizations listed in the `orgs` field. The valid values for this field include the following: `AUTHORIZATION_DIRECTION_FROM`: Allows this organization to evaluate traffic in the organizations listed in the `orgs` field. `AUTHORIZATION_DIRECTION_TO`: Allows the organizations listed in the `orgs` field to evaluate the traffic in this organization. For the authorization relationship to take effect, all of the organizations must authorize and specify the appropriate relationship direction. For example, if organization A authorized organization B and C to evaluate its traffic, by specifying `AUTHORIZATION_DIRECTION_TO` as the authorization direction, organizations B and C must specify `AUTHORIZATION_DIRECTION_FROM` as the authorization direction in their `AuthorizedOrgsDesc` resource.
pub enum AuthorizedOrgsDescAuthorizationDirectionEnum {
    

    /// No direction specified.
    ///
    /// "AUTHORIZATION_DIRECTION_UNSPECIFIED"
    #[serde(rename="AUTHORIZATION_DIRECTION_UNSPECIFIED")]
    AUTHORIZATIONDIRECTIONUNSPECIFIED,
    

    /// The specified organizations are authorized to evaluate traffic in this organization.
    ///
    /// "AUTHORIZATION_DIRECTION_TO"
    #[serde(rename="AUTHORIZATION_DIRECTION_TO")]
    AUTHORIZATIONDIRECTIONTO,
    

    /// The traffic of the specified organizations can be evaluated by this organization.
    ///
    /// "AUTHORIZATION_DIRECTION_FROM"
    #[serde(rename="AUTHORIZATION_DIRECTION_FROM")]
    AUTHORIZATIONDIRECTIONFROM,
}

impl AsRef<str> for AuthorizedOrgsDescAuthorizationDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizedOrgsDescAuthorizationDirectionEnum::AUTHORIZATIONDIRECTIONUNSPECIFIED => "AUTHORIZATION_DIRECTION_UNSPECIFIED",
            AuthorizedOrgsDescAuthorizationDirectionEnum::AUTHORIZATIONDIRECTIONTO => "AUTHORIZATION_DIRECTION_TO",
            AuthorizedOrgsDescAuthorizationDirectionEnum::AUTHORIZATIONDIRECTIONFROM => "AUTHORIZATION_DIRECTION_FROM",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizedOrgsDescAuthorizationDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHORIZATION_DIRECTION_UNSPECIFIED" => Ok(AuthorizedOrgsDescAuthorizationDirectionEnum::AUTHORIZATIONDIRECTIONUNSPECIFIED),
           "AUTHORIZATION_DIRECTION_TO" => Ok(AuthorizedOrgsDescAuthorizationDirectionEnum::AUTHORIZATIONDIRECTIONTO),
           "AUTHORIZATION_DIRECTION_FROM" => Ok(AuthorizedOrgsDescAuthorizationDirectionEnum::AUTHORIZATIONDIRECTIONFROM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizedOrgsDescAuthorizationDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuthorizedOrgsDescAuthorizationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A granular control type for authorization levels. Valid value is `AUTHORIZATION_TYPE_TRUST`.
pub enum AuthorizedOrgsDescAuthorizationTypeEnum {
    

    /// No authorization type specified.
    ///
    /// "AUTHORIZATION_TYPE_UNSPECIFIED"
    #[serde(rename="AUTHORIZATION_TYPE_UNSPECIFIED")]
    AUTHORIZATIONTYPEUNSPECIFIED,
    

    /// This authorization relationship is "trust".
    ///
    /// "AUTHORIZATION_TYPE_TRUST"
    #[serde(rename="AUTHORIZATION_TYPE_TRUST")]
    AUTHORIZATIONTYPETRUST,
}

impl AsRef<str> for AuthorizedOrgsDescAuthorizationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizedOrgsDescAuthorizationTypeEnum::AUTHORIZATIONTYPEUNSPECIFIED => "AUTHORIZATION_TYPE_UNSPECIFIED",
            AuthorizedOrgsDescAuthorizationTypeEnum::AUTHORIZATIONTYPETRUST => "AUTHORIZATION_TYPE_TRUST",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizedOrgsDescAuthorizationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTHORIZATION_TYPE_UNSPECIFIED" => Ok(AuthorizedOrgsDescAuthorizationTypeEnum::AUTHORIZATIONTYPEUNSPECIFIED),
           "AUTHORIZATION_TYPE_TRUST" => Ok(AuthorizedOrgsDescAuthorizationTypeEnum::AUTHORIZATIONTYPETRUST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizedOrgsDescAuthorizationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


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


// region EgressFromIdentityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
pub enum EgressFromIdentityTypeEnum {
    

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

impl AsRef<str> for EgressFromIdentityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EgressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED => "IDENTITY_TYPE_UNSPECIFIED",
            EgressFromIdentityTypeEnum::ANYIDENTITY => "ANY_IDENTITY",
            EgressFromIdentityTypeEnum::ANYUSERACCOUNT => "ANY_USER_ACCOUNT",
            EgressFromIdentityTypeEnum::ANYSERVICEACCOUNT => "ANY_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for EgressFromIdentityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY_TYPE_UNSPECIFIED" => Ok(EgressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED),
           "ANY_IDENTITY" => Ok(EgressFromIdentityTypeEnum::ANYIDENTITY),
           "ANY_USER_ACCOUNT" => Ok(EgressFromIdentityTypeEnum::ANYUSERACCOUNT),
           "ANY_SERVICE_ACCOUNT" => Ok(EgressFromIdentityTypeEnum::ANYSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EgressFromIdentityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EgressFromSourceRestrictionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether to enforce traffic restrictions based on `sources` field. If the `sources` fields is non-empty, then this field must be set to `SOURCE_RESTRICTION_ENABLED`.
pub enum EgressFromSourceRestrictionEnum {
    

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

impl AsRef<str> for EgressFromSourceRestrictionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EgressFromSourceRestrictionEnum::SOURCERESTRICTIONUNSPECIFIED => "SOURCE_RESTRICTION_UNSPECIFIED",
            EgressFromSourceRestrictionEnum::SOURCERESTRICTIONENABLED => "SOURCE_RESTRICTION_ENABLED",
            EgressFromSourceRestrictionEnum::SOURCERESTRICTIONDISABLED => "SOURCE_RESTRICTION_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for EgressFromSourceRestrictionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOURCE_RESTRICTION_UNSPECIFIED" => Ok(EgressFromSourceRestrictionEnum::SOURCERESTRICTIONUNSPECIFIED),
           "SOURCE_RESTRICTION_ENABLED" => Ok(EgressFromSourceRestrictionEnum::SOURCERESTRICTIONENABLED),
           "SOURCE_RESTRICTION_DISABLED" => Ok(EgressFromSourceRestrictionEnum::SOURCERESTRICTIONDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EgressFromSourceRestrictionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IngressFromIdentityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
pub enum IngressFromIdentityTypeEnum {
    

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

impl AsRef<str> for IngressFromIdentityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IngressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED => "IDENTITY_TYPE_UNSPECIFIED",
            IngressFromIdentityTypeEnum::ANYIDENTITY => "ANY_IDENTITY",
            IngressFromIdentityTypeEnum::ANYUSERACCOUNT => "ANY_USER_ACCOUNT",
            IngressFromIdentityTypeEnum::ANYSERVICEACCOUNT => "ANY_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for IngressFromIdentityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY_TYPE_UNSPECIFIED" => Ok(IngressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED),
           "ANY_IDENTITY" => Ok(IngressFromIdentityTypeEnum::ANYIDENTITY),
           "ANY_USER_ACCOUNT" => Ok(IngressFromIdentityTypeEnum::ANYUSERACCOUNT),
           "ANY_SERVICE_ACCOUNT" => Ok(IngressFromIdentityTypeEnum::ANYSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IngressFromIdentityTypeEnum {
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
/// Perimeter type indicator. A single project or VPC network is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty.
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


// region SupportedServiceSupportStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The support stage of the service.
pub enum SupportedServiceSupportStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for SupportedServiceSupportStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SupportedServiceSupportStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            SupportedServiceSupportStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            SupportedServiceSupportStageEnum::PRELAUNCH => "PRELAUNCH",
            SupportedServiceSupportStageEnum::EARLYACCESS => "EARLY_ACCESS",
            SupportedServiceSupportStageEnum::ALPHA => "ALPHA",
            SupportedServiceSupportStageEnum::BETA => "BETA",
            SupportedServiceSupportStageEnum::GA => "GA",
            SupportedServiceSupportStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for SupportedServiceSupportStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(SupportedServiceSupportStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(SupportedServiceSupportStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(SupportedServiceSupportStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(SupportedServiceSupportStageEnum::EARLYACCESS),
           "ALPHA" => Ok(SupportedServiceSupportStageEnum::ALPHA),
           "BETA" => Ok(SupportedServiceSupportStageEnum::BETA),
           "GA" => Ok(SupportedServiceSupportStageEnum::GA),
           "DEPRECATED" => Ok(SupportedServiceSupportStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SupportedServiceSupportStageEnum {
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


