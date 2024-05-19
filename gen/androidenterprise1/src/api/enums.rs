use super::*;



// region AdministratorWebTokenSpecPermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. Use PlaySearch.approveApps.
pub enum AdministratorWebTokenSpecPermissionEnum {
    

    /// Unknown permission.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// Permission to approve and unapprove apps.
    ///
    /// "approveApps"
    #[serde(rename="approveApps")]
    ApproveApps,
    

    /// Permission to manage app restrictions.
    ///
    /// "manageMcm"
    #[serde(rename="manageMcm")]
    ManageMcm,
}

impl AsRef<str> for AdministratorWebTokenSpecPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdministratorWebTokenSpecPermissionEnum::Unknown => "unknown",
            AdministratorWebTokenSpecPermissionEnum::ApproveApps => "approveApps",
            AdministratorWebTokenSpecPermissionEnum::ManageMcm => "manageMcm",
        }
    }
}

impl std::convert::TryFrom< &str> for AdministratorWebTokenSpecPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(AdministratorWebTokenSpecPermissionEnum::Unknown),
           "approveApps" => Ok(AdministratorWebTokenSpecPermissionEnum::ApproveApps),
           "manageMcm" => Ok(AdministratorWebTokenSpecPermissionEnum::ManageMcm),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdministratorWebTokenSpecPermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppRestrictionsSchemaRestrictionRestrictionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the restriction.
pub enum AppRestrictionsSchemaRestrictionRestrictionTypeEnum {
    

    /// A restriction of boolean type.
    ///
    /// "bool"
    #[serde(rename="bool")]
    Bool,
    

    /// A restriction of string type.
    ///
    /// "string"
    #[serde(rename="string")]
    String,
    

    /// A restriction of integer type.
    ///
    /// "integer"
    #[serde(rename="integer")]
    Integer,
    

    /// A choice of one item from a set.
    ///
    /// "choice"
    #[serde(rename="choice")]
    Choice,
    

    /// A choice of multiple items from a set.
    ///
    /// "multiselect"
    #[serde(rename="multiselect")]
    Multiselect,
    

    /// A hidden restriction of string type (the default value can be used to pass along information that cannot be modified, such as a version code).
    ///
    /// "hidden"
    #[serde(rename="hidden")]
    Hidden,
    

    /// [M+ devices only] A bundle of restrictions
    ///
    /// "bundle"
    #[serde(rename="bundle")]
    Bundle,
    

    /// [M+ devices only] An array of restriction bundles
    ///
    /// "bundleArray"
    #[serde(rename="bundleArray")]
    BundleArray,
}

impl AsRef<str> for AppRestrictionsSchemaRestrictionRestrictionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Bool => "bool",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::String => "string",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Integer => "integer",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Choice => "choice",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Multiselect => "multiselect",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Hidden => "hidden",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Bundle => "bundle",
            AppRestrictionsSchemaRestrictionRestrictionTypeEnum::BundleArray => "bundleArray",
        }
    }
}

impl std::convert::TryFrom< &str> for AppRestrictionsSchemaRestrictionRestrictionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "bool" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Bool),
           "string" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::String),
           "integer" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Integer),
           "choice" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Choice),
           "multiselect" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Multiselect),
           "hidden" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Hidden),
           "bundle" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::Bundle),
           "bundleArray" => Ok(AppRestrictionsSchemaRestrictionRestrictionTypeEnum::BundleArray),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppRestrictionsSchemaRestrictionRestrictionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the value being provided.
pub enum AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum {
    

    /// A restriction of boolean type.
    ///
    /// "bool"
    #[serde(rename="bool")]
    Bool,
    

    /// A restriction of string type.
    ///
    /// "string"
    #[serde(rename="string")]
    String,
    

    /// A restriction of integer type.
    ///
    /// "integer"
    #[serde(rename="integer")]
    Integer,
    

    /// A choice of one item from a set.
    ///
    /// "choice"
    #[serde(rename="choice")]
    Choice,
    

    /// A choice of multiple items from a set.
    ///
    /// "multiselect"
    #[serde(rename="multiselect")]
    Multiselect,
    

    /// A hidden restriction of string type (the default value can be used to pass along information that cannot be modified, such as a version code).
    ///
    /// "hidden"
    #[serde(rename="hidden")]
    Hidden,
    

    /// [M+ devices only] A bundle of restrictions
    ///
    /// "bundle"
    #[serde(rename="bundle")]
    Bundle,
    

    /// [M+ devices only] An array of restriction bundles
    ///
    /// "bundleArray"
    #[serde(rename="bundleArray")]
    BundleArray,
}

impl AsRef<str> for AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Bool => "bool",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::String => "string",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Integer => "integer",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Choice => "choice",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Multiselect => "multiselect",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Hidden => "hidden",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Bundle => "bundle",
            AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::BundleArray => "bundleArray",
        }
    }
}

impl std::convert::TryFrom< &str> for AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "bool" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Bool),
           "string" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::String),
           "integer" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Integer),
           "choice" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Choice),
           "multiselect" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Multiselect),
           "hidden" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Hidden),
           "bundle" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::Bundle),
           "bundleArray" => Ok(AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum::BundleArray),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppRestrictionsSchemaRestrictionRestrictionValueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppVersionTrackEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated, use trackId instead.
pub enum AppVersionTrackEnum {
    
    /// "appTrackUnspecified"
    #[serde(rename="appTrackUnspecified")]
    AppTrackUnspecified,
    
    /// "production"
    #[serde(rename="production")]
    Production,
    
    /// "beta"
    #[serde(rename="beta")]
    Beta,
    
    /// "alpha"
    #[serde(rename="alpha")]
    Alpha,
}

impl AsRef<str> for AppVersionTrackEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppVersionTrackEnum::AppTrackUnspecified => "appTrackUnspecified",
            AppVersionTrackEnum::Production => "production",
            AppVersionTrackEnum::Beta => "beta",
            AppVersionTrackEnum::Alpha => "alpha",
        }
    }
}

impl std::convert::TryFrom< &str> for AppVersionTrackEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "appTrackUnspecified" => Ok(AppVersionTrackEnum::AppTrackUnspecified),
           "production" => Ok(AppVersionTrackEnum::Production),
           "beta" => Ok(AppVersionTrackEnum::Beta),
           "alpha" => Ok(AppVersionTrackEnum::Alpha),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppVersionTrackEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutoInstallConstraintChargingStateConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Charging state constraint.
pub enum AutoInstallConstraintChargingStateConstraintEnum {
    
    /// "chargingStateConstraintUnspecified"
    #[serde(rename="chargingStateConstraintUnspecified")]
    ChargingStateConstraintUnspecified,
    

    /// Device doesn't have to be charging.
    ///
    /// "chargingNotRequired"
    #[serde(rename="chargingNotRequired")]
    ChargingNotRequired,
    

    /// Device has to be charging.
    ///
    /// "chargingRequired"
    #[serde(rename="chargingRequired")]
    ChargingRequired,
}

impl AsRef<str> for AutoInstallConstraintChargingStateConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoInstallConstraintChargingStateConstraintEnum::ChargingStateConstraintUnspecified => "chargingStateConstraintUnspecified",
            AutoInstallConstraintChargingStateConstraintEnum::ChargingNotRequired => "chargingNotRequired",
            AutoInstallConstraintChargingStateConstraintEnum::ChargingRequired => "chargingRequired",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoInstallConstraintChargingStateConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "chargingStateConstraintUnspecified" => Ok(AutoInstallConstraintChargingStateConstraintEnum::ChargingStateConstraintUnspecified),
           "chargingNotRequired" => Ok(AutoInstallConstraintChargingStateConstraintEnum::ChargingNotRequired),
           "chargingRequired" => Ok(AutoInstallConstraintChargingStateConstraintEnum::ChargingRequired),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoInstallConstraintChargingStateConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutoInstallConstraintDeviceIdleStateConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Device idle state constraint.
pub enum AutoInstallConstraintDeviceIdleStateConstraintEnum {
    
    /// "deviceIdleStateConstraintUnspecified"
    #[serde(rename="deviceIdleStateConstraintUnspecified")]
    DeviceIdleStateConstraintUnspecified,
    

    /// Device doesn't have to be idle, app can be installed while the user is interacting with the device.
    ///
    /// "deviceIdleNotRequired"
    #[serde(rename="deviceIdleNotRequired")]
    DeviceIdleNotRequired,
    

    /// Device has to be idle.
    ///
    /// "deviceIdleRequired"
    #[serde(rename="deviceIdleRequired")]
    DeviceIdleRequired,
}

impl AsRef<str> for AutoInstallConstraintDeviceIdleStateConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoInstallConstraintDeviceIdleStateConstraintEnum::DeviceIdleStateConstraintUnspecified => "deviceIdleStateConstraintUnspecified",
            AutoInstallConstraintDeviceIdleStateConstraintEnum::DeviceIdleNotRequired => "deviceIdleNotRequired",
            AutoInstallConstraintDeviceIdleStateConstraintEnum::DeviceIdleRequired => "deviceIdleRequired",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoInstallConstraintDeviceIdleStateConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "deviceIdleStateConstraintUnspecified" => Ok(AutoInstallConstraintDeviceIdleStateConstraintEnum::DeviceIdleStateConstraintUnspecified),
           "deviceIdleNotRequired" => Ok(AutoInstallConstraintDeviceIdleStateConstraintEnum::DeviceIdleNotRequired),
           "deviceIdleRequired" => Ok(AutoInstallConstraintDeviceIdleStateConstraintEnum::DeviceIdleRequired),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoInstallConstraintDeviceIdleStateConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutoInstallConstraintNetworkTypeConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Network type constraint.
pub enum AutoInstallConstraintNetworkTypeConstraintEnum {
    
    /// "networkTypeConstraintUnspecified"
    #[serde(rename="networkTypeConstraintUnspecified")]
    NetworkTypeConstraintUnspecified,
    

    /// Any active networks (Wi-Fi, cellular, etc.).
    ///
    /// "anyNetwork"
    #[serde(rename="anyNetwork")]
    AnyNetwork,
    

    /// Any unmetered network (e.g. Wi-FI).
    ///
    /// "unmeteredNetwork"
    #[serde(rename="unmeteredNetwork")]
    UnmeteredNetwork,
}

impl AsRef<str> for AutoInstallConstraintNetworkTypeConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoInstallConstraintNetworkTypeConstraintEnum::NetworkTypeConstraintUnspecified => "networkTypeConstraintUnspecified",
            AutoInstallConstraintNetworkTypeConstraintEnum::AnyNetwork => "anyNetwork",
            AutoInstallConstraintNetworkTypeConstraintEnum::UnmeteredNetwork => "unmeteredNetwork",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoInstallConstraintNetworkTypeConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "networkTypeConstraintUnspecified" => Ok(AutoInstallConstraintNetworkTypeConstraintEnum::NetworkTypeConstraintUnspecified),
           "anyNetwork" => Ok(AutoInstallConstraintNetworkTypeConstraintEnum::AnyNetwork),
           "unmeteredNetwork" => Ok(AutoInstallConstraintNetworkTypeConstraintEnum::UnmeteredNetwork),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoInstallConstraintNetworkTypeConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutoInstallPolicyAutoInstallModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The auto-install mode. If unset, defaults to "doNotAutoInstall". An app is automatically installed regardless of a set maintenance window.
pub enum AutoInstallPolicyAutoInstallModeEnum {
    
    /// "autoInstallModeUnspecified"
    #[serde(rename="autoInstallModeUnspecified")]
    AutoInstallModeUnspecified,
    

    /// The product is not installed automatically, the user needs to install it from the Play Store.
    ///
    /// "doNotAutoInstall"
    #[serde(rename="doNotAutoInstall")]
    DoNotAutoInstall,
    

    /// The product is automatically installed once, if the user uninstalls the product it will not be installed again.
    ///
    /// "autoInstallOnce"
    #[serde(rename="autoInstallOnce")]
    AutoInstallOnce,
    

    /// The product is automatically installed, if the user uninstalls the product it will be installed again. On managed devices the DPC should block uninstall.
    ///
    /// "forceAutoInstall"
    #[serde(rename="forceAutoInstall")]
    ForceAutoInstall,
}

impl AsRef<str> for AutoInstallPolicyAutoInstallModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutoInstallPolicyAutoInstallModeEnum::AutoInstallModeUnspecified => "autoInstallModeUnspecified",
            AutoInstallPolicyAutoInstallModeEnum::DoNotAutoInstall => "doNotAutoInstall",
            AutoInstallPolicyAutoInstallModeEnum::AutoInstallOnce => "autoInstallOnce",
            AutoInstallPolicyAutoInstallModeEnum::ForceAutoInstall => "forceAutoInstall",
        }
    }
}

impl std::convert::TryFrom< &str> for AutoInstallPolicyAutoInstallModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "autoInstallModeUnspecified" => Ok(AutoInstallPolicyAutoInstallModeEnum::AutoInstallModeUnspecified),
           "doNotAutoInstall" => Ok(AutoInstallPolicyAutoInstallModeEnum::DoNotAutoInstall),
           "autoInstallOnce" => Ok(AutoInstallPolicyAutoInstallModeEnum::AutoInstallOnce),
           "forceAutoInstall" => Ok(AutoInstallPolicyAutoInstallModeEnum::ForceAutoInstall),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutoInstallPolicyAutoInstallModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceManagementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations. Possible values include: - "managedDevice", a device that has the EMM's device policy controller (DPC) as the device owner. - "managedProfile", a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. - "containerApp", no longer used (deprecated). - "unmanagedProfile", a device that has been allowed (by the domain's admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC. 
pub enum DeviceManagementTypeEnum {
    
    /// "managedDevice"
    #[serde(rename="managedDevice")]
    ManagedDevice,
    
    /// "managedProfile"
    #[serde(rename="managedProfile")]
    ManagedProfile,
    
    /// "containerApp"
    #[serde(rename="containerApp")]
    ContainerApp,
    
    /// "unmanagedProfile"
    #[serde(rename="unmanagedProfile")]
    UnmanagedProfile,
}

impl AsRef<str> for DeviceManagementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceManagementTypeEnum::ManagedDevice => "managedDevice",
            DeviceManagementTypeEnum::ManagedProfile => "managedProfile",
            DeviceManagementTypeEnum::ContainerApp => "containerApp",
            DeviceManagementTypeEnum::UnmanagedProfile => "unmanagedProfile",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceManagementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "managedDevice" => Ok(DeviceManagementTypeEnum::ManagedDevice),
           "managedProfile" => Ok(DeviceManagementTypeEnum::ManagedProfile),
           "containerApp" => Ok(DeviceManagementTypeEnum::ContainerApp),
           "unmanagedProfile" => Ok(DeviceManagementTypeEnum::UnmanagedProfile),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceManagementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceStateAccountStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the Google account on the device. "enabled" indicates that the Google account on the device can be used to access Google services (including Google Play), while "disabled" means that it cannot. A new device is initially in the "disabled" state.
pub enum DeviceStateAccountStateEnum {
    
    /// "enabled"
    #[serde(rename="enabled")]
    Enabled,
    
    /// "disabled"
    #[serde(rename="disabled")]
    Disabled,
}

impl AsRef<str> for DeviceStateAccountStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceStateAccountStateEnum::Enabled => "enabled",
            DeviceStateAccountStateEnum::Disabled => "disabled",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceStateAccountStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "enabled" => Ok(DeviceStateAccountStateEnum::Enabled),
           "disabled" => Ok(DeviceStateAccountStateEnum::Disabled),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceStateAccountStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EntitlementReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason for the entitlement. For example, "free" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses.
pub enum EntitlementReasonEnum {
    
    /// "free"
    #[serde(rename="free")]
    Free,
    
    /// "groupLicense"
    #[serde(rename="groupLicense")]
    GroupLicense,
    
    /// "userPurchase"
    #[serde(rename="userPurchase")]
    UserPurchase,
}

impl AsRef<str> for EntitlementReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntitlementReasonEnum::Free => "free",
            EntitlementReasonEnum::GroupLicense => "groupLicense",
            EntitlementReasonEnum::UserPurchase => "userPurchase",
        }
    }
}

impl std::convert::TryFrom< &str> for EntitlementReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "free" => Ok(EntitlementReasonEnum::Free),
           "groupLicense" => Ok(EntitlementReasonEnum::GroupLicense),
           "userPurchase" => Ok(EntitlementReasonEnum::UserPurchase),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntitlementReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAuthenticationSettingDedicatedDevicesAllowedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether dedicated devices are allowed.
pub enum GoogleAuthenticationSettingDedicatedDevicesAllowedEnum {
    

    /// This value is unused.
    ///
    /// "dedicatedDevicesAllowedUnspecified"
    #[serde(rename="dedicatedDevicesAllowedUnspecified")]
    DedicatedDevicesAllowedUnspecified,
    

    /// Dedicated devices are not allowed.
    ///
    /// "disallowed"
    #[serde(rename="disallowed")]
    Disallowed,
    

    /// Dedicated devices are allowed.
    ///
    /// "allowed"
    #[serde(rename="allowed")]
    Allowed,
}

impl AsRef<str> for GoogleAuthenticationSettingDedicatedDevicesAllowedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAuthenticationSettingDedicatedDevicesAllowedEnum::DedicatedDevicesAllowedUnspecified => "dedicatedDevicesAllowedUnspecified",
            GoogleAuthenticationSettingDedicatedDevicesAllowedEnum::Disallowed => "disallowed",
            GoogleAuthenticationSettingDedicatedDevicesAllowedEnum::Allowed => "allowed",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAuthenticationSettingDedicatedDevicesAllowedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "dedicatedDevicesAllowedUnspecified" => Ok(GoogleAuthenticationSettingDedicatedDevicesAllowedEnum::DedicatedDevicesAllowedUnspecified),
           "disallowed" => Ok(GoogleAuthenticationSettingDedicatedDevicesAllowedEnum::Disallowed),
           "allowed" => Ok(GoogleAuthenticationSettingDedicatedDevicesAllowedEnum::Allowed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAuthenticationSettingDedicatedDevicesAllowedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether Google authentication is required.
pub enum GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum {
    

    /// This value is unused.
    ///
    /// "googleAuthenticationRequiredUnspecified"
    #[serde(rename="googleAuthenticationRequiredUnspecified")]
    GoogleAuthenticationRequiredUnspecified,
    

    /// Google authentication is not required.
    ///
    /// "notRequired"
    #[serde(rename="notRequired")]
    NotRequired,
    

    /// User is required to be successfully authenticated by Google.
    ///
    /// "required"
    #[serde(rename="required")]
    Required,
}

impl AsRef<str> for GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum::GoogleAuthenticationRequiredUnspecified => "googleAuthenticationRequiredUnspecified",
            GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum::NotRequired => "notRequired",
            GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum::Required => "required",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "googleAuthenticationRequiredUnspecified" => Ok(GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum::GoogleAuthenticationRequiredUnspecified),
           "notRequired" => Ok(GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum::NotRequired),
           "required" => Ok(GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum::Required),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAuthenticationSettingGoogleAuthenticationRequiredEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupLicenseAcquisitionKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How this group license was acquired. "bulkPurchase" means that this Grouplicenses resource was created because the enterprise purchased licenses for this product; otherwise, the value is "free" (for free products).
pub enum GroupLicenseAcquisitionKindEnum {
    
    /// "free"
    #[serde(rename="free")]
    Free,
    
    /// "bulkPurchase"
    #[serde(rename="bulkPurchase")]
    BulkPurchase,
}

impl AsRef<str> for GroupLicenseAcquisitionKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupLicenseAcquisitionKindEnum::Free => "free",
            GroupLicenseAcquisitionKindEnum::BulkPurchase => "bulkPurchase",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupLicenseAcquisitionKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "free" => Ok(GroupLicenseAcquisitionKindEnum::Free),
           "bulkPurchase" => Ok(GroupLicenseAcquisitionKindEnum::BulkPurchase),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupLicenseAcquisitionKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupLicenseApprovalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the product to which this group license relates is currently approved by the enterprise. Products are approved when a group license is first created, but this approval may be revoked by an enterprise admin via Google Play. Unapproved products will not be visible to end users in collections, and new entitlements to them should not normally be created.
pub enum GroupLicenseApprovalEnum {
    
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "unapproved"
    #[serde(rename="unapproved")]
    Unapproved,
}

impl AsRef<str> for GroupLicenseApprovalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupLicenseApprovalEnum::Approved => "approved",
            GroupLicenseApprovalEnum::Unapproved => "unapproved",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupLicenseApprovalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(GroupLicenseApprovalEnum::Approved),
           "unapproved" => Ok(GroupLicenseApprovalEnum::Unapproved),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupLicenseApprovalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupLicensePermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The permission approval status of the product. This field is only set if the product is approved. Possible states are: - "currentApproved", the current set of permissions is approved, but additional permissions will require the administrator to reapprove the product (If the product was approved without specifying the approved permissions setting, then this is the default behavior.), - "needsReapproval", the product has unapproved permissions. No additional product licenses can be assigned until the product is reapproved, - "allCurrentAndFutureApproved", the current permissions are approved and any future permission updates will be automatically approved without administrator review. 
pub enum GroupLicensePermissionsEnum {
    
    /// "currentApproved"
    #[serde(rename="currentApproved")]
    CurrentApproved,
    
    /// "needsReapproval"
    #[serde(rename="needsReapproval")]
    NeedsReapproval,
    
    /// "allCurrentAndFutureApproved"
    #[serde(rename="allCurrentAndFutureApproved")]
    AllCurrentAndFutureApproved,
}

impl AsRef<str> for GroupLicensePermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupLicensePermissionsEnum::CurrentApproved => "currentApproved",
            GroupLicensePermissionsEnum::NeedsReapproval => "needsReapproval",
            GroupLicensePermissionsEnum::AllCurrentAndFutureApproved => "allCurrentAndFutureApproved",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupLicensePermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "currentApproved" => Ok(GroupLicensePermissionsEnum::CurrentApproved),
           "needsReapproval" => Ok(GroupLicensePermissionsEnum::NeedsReapproval),
           "allCurrentAndFutureApproved" => Ok(GroupLicensePermissionsEnum::AllCurrentAndFutureApproved),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupLicensePermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstallInstallStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Install state. The state "installPending" means that an install request has recently been made and download to the device is in progress. The state "installed" means that the app has been installed. This field is read-only.
pub enum InstallInstallStateEnum {
    
    /// "installed"
    #[serde(rename="installed")]
    Installed,
    
    /// "installPending"
    #[serde(rename="installPending")]
    InstallPending,
}

impl AsRef<str> for InstallInstallStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstallInstallStateEnum::Installed => "installed",
            InstallInstallStateEnum::InstallPending => "installPending",
        }
    }
}

impl std::convert::TryFrom< &str> for InstallInstallStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "installed" => Ok(InstallInstallStateEnum::Installed),
           "installPending" => Ok(InstallInstallStateEnum::InstallPending),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstallInstallStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstallFailureEventFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason for the installation failure. This field will always be present.
pub enum InstallFailureEventFailureReasonEnum {
    

    /// Used whenever no better reason for failure can be provided.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// Used when the installation timed out. This can cover a number of situations, for example when the device did not have connectivity at any point during the retry period, or if the device is OOM.
    ///
    /// "timeout"
    #[serde(rename="timeout")]
    Timeout,
}

impl AsRef<str> for InstallFailureEventFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstallFailureEventFailureReasonEnum::Unknown => "unknown",
            InstallFailureEventFailureReasonEnum::Timeout => "timeout",
        }
    }
}

impl std::convert::TryFrom< &str> for InstallFailureEventFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(InstallFailureEventFailureReasonEnum::Unknown),
           "timeout" => Ok(InstallFailureEventFailureReasonEnum::Timeout),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstallFailureEventFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyedAppStateSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Severity of the app state. This field will always be present.
pub enum KeyedAppStateSeverityEnum {
    
    /// "severityUnknown"
    #[serde(rename="severityUnknown")]
    SeverityUnknown,
    
    /// "severityInfo"
    #[serde(rename="severityInfo")]
    SeverityInfo,
    
    /// "severityError"
    #[serde(rename="severityError")]
    SeverityError,
}

impl AsRef<str> for KeyedAppStateSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyedAppStateSeverityEnum::SeverityUnknown => "severityUnknown",
            KeyedAppStateSeverityEnum::SeverityInfo => "severityInfo",
            KeyedAppStateSeverityEnum::SeverityError => "severityError",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyedAppStateSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "severityUnknown" => Ok(KeyedAppStateSeverityEnum::SeverityUnknown),
           "severityInfo" => Ok(KeyedAppStateSeverityEnum::SeverityInfo),
           "severityError" => Ok(KeyedAppStateSeverityEnum::SeverityError),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KeyedAppStateSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NewDeviceEventManagementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Identifies the extent to which the device is controlled by an Android EMM in various deployment configurations. Possible values include: - "managedDevice", a device where the DPC is set as device owner, - "managedProfile", a device where the DPC is set as profile owner. 
pub enum NewDeviceEventManagementTypeEnum {
    
    /// "managedDevice"
    #[serde(rename="managedDevice")]
    ManagedDevice,
    
    /// "managedProfile"
    #[serde(rename="managedProfile")]
    ManagedProfile,
}

impl AsRef<str> for NewDeviceEventManagementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NewDeviceEventManagementTypeEnum::ManagedDevice => "managedDevice",
            NewDeviceEventManagementTypeEnum::ManagedProfile => "managedProfile",
        }
    }
}

impl std::convert::TryFrom< &str> for NewDeviceEventManagementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "managedDevice" => Ok(NewDeviceEventManagementTypeEnum::ManagedDevice),
           "managedProfile" => Ok(NewDeviceEventManagementTypeEnum::ManagedProfile),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NewDeviceEventManagementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationNotificationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the notification.
pub enum NotificationNotificationTypeEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// A test push notification.
    ///
    /// "testNotification"
    #[serde(rename="testNotification")]
    TestNotification,
    

    /// Notification about change to a product's approval status.
    ///
    /// "productApproval"
    #[serde(rename="productApproval")]
    ProductApproval,
    

    /// Notification about an app installation failure.
    ///
    /// "installFailure"
    #[serde(rename="installFailure")]
    InstallFailure,
    

    /// Notification about app update.
    ///
    /// "appUpdate"
    #[serde(rename="appUpdate")]
    AppUpdate,
    

    /// Notification about new app permissions.
    ///
    /// "newPermissions"
    #[serde(rename="newPermissions")]
    NewPermissions,
    

    /// Notification about new app restrictions schema change.
    ///
    /// "appRestricionsSchemaChange"
    #[serde(rename="appRestricionsSchemaChange")]
    AppRestricionsSchemaChange,
    

    /// Notification about product availability change.
    ///
    /// "productAvailabilityChange"
    #[serde(rename="productAvailabilityChange")]
    ProductAvailabilityChange,
    

    /// Notification about a new device.
    ///
    /// "newDevice"
    #[serde(rename="newDevice")]
    NewDevice,
    

    /// Notification about an updated device report.
    ///
    /// "deviceReportUpdate"
    #[serde(rename="deviceReportUpdate")]
    DeviceReportUpdate,
}

impl AsRef<str> for NotificationNotificationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationNotificationTypeEnum::Unknown => "unknown",
            NotificationNotificationTypeEnum::TestNotification => "testNotification",
            NotificationNotificationTypeEnum::ProductApproval => "productApproval",
            NotificationNotificationTypeEnum::InstallFailure => "installFailure",
            NotificationNotificationTypeEnum::AppUpdate => "appUpdate",
            NotificationNotificationTypeEnum::NewPermissions => "newPermissions",
            NotificationNotificationTypeEnum::AppRestricionsSchemaChange => "appRestricionsSchemaChange",
            NotificationNotificationTypeEnum::ProductAvailabilityChange => "productAvailabilityChange",
            NotificationNotificationTypeEnum::NewDevice => "newDevice",
            NotificationNotificationTypeEnum::DeviceReportUpdate => "deviceReportUpdate",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationNotificationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(NotificationNotificationTypeEnum::Unknown),
           "testNotification" => Ok(NotificationNotificationTypeEnum::TestNotification),
           "productApproval" => Ok(NotificationNotificationTypeEnum::ProductApproval),
           "installFailure" => Ok(NotificationNotificationTypeEnum::InstallFailure),
           "appUpdate" => Ok(NotificationNotificationTypeEnum::AppUpdate),
           "newPermissions" => Ok(NotificationNotificationTypeEnum::NewPermissions),
           "appRestricionsSchemaChange" => Ok(NotificationNotificationTypeEnum::AppRestricionsSchemaChange),
           "productAvailabilityChange" => Ok(NotificationNotificationTypeEnum::ProductAvailabilityChange),
           "newDevice" => Ok(NotificationNotificationTypeEnum::NewDevice),
           "deviceReportUpdate" => Ok(NotificationNotificationTypeEnum::DeviceReportUpdate),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationNotificationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAutoUpdatePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls when automatic app updates on the device can be applied. Recommended alternative: autoUpdateMode which is set per app, provides greater flexibility around update frequency. When autoUpdateMode is set to AUTO_UPDATE_POSTPONED or AUTO_UPDATE_HIGH_PRIORITY, autoUpdatePolicy has no effect. "choiceToTheUser" allows the device's user to configure the app update policy. "always" enables auto updates. "never" disables auto updates. "wifiOnly" enables auto updates only when the device is connected to wifi.
pub enum PolicyAutoUpdatePolicyEnum {
    

    /// The auto update policy is not set.
    ///
    /// "autoUpdatePolicyUnspecified"
    #[serde(rename="autoUpdatePolicyUnspecified")]
    AutoUpdatePolicyUnspecified,
    

    /// The user can control auto-updates.
    ///
    /// "choiceToTheUser"
    #[serde(rename="choiceToTheUser")]
    ChoiceToTheUser,
    

    /// Apps are never auto-updated.
    ///
    /// "never"
    #[serde(rename="never")]
    Never,
    

    /// Apps are auto-updated over WiFi only.
    ///
    /// "wifiOnly"
    #[serde(rename="wifiOnly")]
    WifiOnly,
    

    /// Apps are auto-updated at any time. Data charges may apply.
    ///
    /// "always"
    #[serde(rename="always")]
    Always,
}

impl AsRef<str> for PolicyAutoUpdatePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAutoUpdatePolicyEnum::AutoUpdatePolicyUnspecified => "autoUpdatePolicyUnspecified",
            PolicyAutoUpdatePolicyEnum::ChoiceToTheUser => "choiceToTheUser",
            PolicyAutoUpdatePolicyEnum::Never => "never",
            PolicyAutoUpdatePolicyEnum::WifiOnly => "wifiOnly",
            PolicyAutoUpdatePolicyEnum::Always => "always",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAutoUpdatePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "autoUpdatePolicyUnspecified" => Ok(PolicyAutoUpdatePolicyEnum::AutoUpdatePolicyUnspecified),
           "choiceToTheUser" => Ok(PolicyAutoUpdatePolicyEnum::ChoiceToTheUser),
           "never" => Ok(PolicyAutoUpdatePolicyEnum::Never),
           "wifiOnly" => Ok(PolicyAutoUpdatePolicyEnum::WifiOnly),
           "always" => Ok(PolicyAutoUpdatePolicyEnum::Always),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAutoUpdatePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyDeviceReportPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the device reports app states to the EMM. The default value is "deviceReportDisabled".
pub enum PolicyDeviceReportPolicyEnum {
    

    /// The device report policy is not set.
    ///
    /// "deviceReportPolicyUnspecified"
    #[serde(rename="deviceReportPolicyUnspecified")]
    DeviceReportPolicyUnspecified,
    

    /// Device reports are disabled.
    ///
    /// "deviceReportDisabled"
    #[serde(rename="deviceReportDisabled")]
    DeviceReportDisabled,
    

    /// Device reports are enabled.
    ///
    /// "deviceReportEnabled"
    #[serde(rename="deviceReportEnabled")]
    DeviceReportEnabled,
}

impl AsRef<str> for PolicyDeviceReportPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyDeviceReportPolicyEnum::DeviceReportPolicyUnspecified => "deviceReportPolicyUnspecified",
            PolicyDeviceReportPolicyEnum::DeviceReportDisabled => "deviceReportDisabled",
            PolicyDeviceReportPolicyEnum::DeviceReportEnabled => "deviceReportEnabled",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyDeviceReportPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "deviceReportPolicyUnspecified" => Ok(PolicyDeviceReportPolicyEnum::DeviceReportPolicyUnspecified),
           "deviceReportDisabled" => Ok(PolicyDeviceReportPolicyEnum::DeviceReportDisabled),
           "deviceReportEnabled" => Ok(PolicyDeviceReportPolicyEnum::DeviceReportEnabled),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyDeviceReportPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyProductAvailabilityPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The availability granted to the device for the specified products. "all" gives the device access to all products, regardless of approval status. "all" does not enable automatic visibility of "alpha" or "beta" tracks. "whitelist" grants the device access the products specified in productPolicy[]. Only products that are approved or products that were previously approved (products with revoked approval) by the enterprise can be whitelisted. If no value is provided, the availability set at the user level is applied by default.
pub enum PolicyProductAvailabilityPolicyEnum {
    

    /// Unspecified, applies the user available product set by default.
    ///
    /// "productAvailabilityPolicyUnspecified"
    #[serde(rename="productAvailabilityPolicyUnspecified")]
    ProductAvailabilityPolicyUnspecified,
    

    /// The approved products with product availability set to AVAILABLE in the product policy are available.
    ///
    /// "whitelist"
    #[serde(rename="whitelist")]
    Whitelist,
    

    /// All products are available except those explicitly marked as unavailable in the product availability policy.
    ///
    /// "all"
    #[serde(rename="all")]
    All,
}

impl AsRef<str> for PolicyProductAvailabilityPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyProductAvailabilityPolicyEnum::ProductAvailabilityPolicyUnspecified => "productAvailabilityPolicyUnspecified",
            PolicyProductAvailabilityPolicyEnum::Whitelist => "whitelist",
            PolicyProductAvailabilityPolicyEnum::All => "all",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyProductAvailabilityPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "productAvailabilityPolicyUnspecified" => Ok(PolicyProductAvailabilityPolicyEnum::ProductAvailabilityPolicyUnspecified),
           "whitelist" => Ok(PolicyProductAvailabilityPolicyEnum::Whitelist),
           "all" => Ok(PolicyProductAvailabilityPolicyEnum::All),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyProductAvailabilityPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductAvailableTracksEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated, use appTracks instead.
pub enum ProductAvailableTracksEnum {
    
    /// "appTrackUnspecified"
    #[serde(rename="appTrackUnspecified")]
    AppTrackUnspecified,
    
    /// "production"
    #[serde(rename="production")]
    Production,
    
    /// "beta"
    #[serde(rename="beta")]
    Beta,
    
    /// "alpha"
    #[serde(rename="alpha")]
    Alpha,
}

impl AsRef<str> for ProductAvailableTracksEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductAvailableTracksEnum::AppTrackUnspecified => "appTrackUnspecified",
            ProductAvailableTracksEnum::Production => "production",
            ProductAvailableTracksEnum::Beta => "beta",
            ProductAvailableTracksEnum::Alpha => "alpha",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductAvailableTracksEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "appTrackUnspecified" => Ok(ProductAvailableTracksEnum::AppTrackUnspecified),
           "production" => Ok(ProductAvailableTracksEnum::Production),
           "beta" => Ok(ProductAvailableTracksEnum::Beta),
           "alpha" => Ok(ProductAvailableTracksEnum::Alpha),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductAvailableTracksEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductContentRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The content rating for this app.
pub enum ProductContentRatingEnum {
    
    /// "ratingUnknown"
    #[serde(rename="ratingUnknown")]
    RatingUnknown,
    
    /// "all"
    #[serde(rename="all")]
    All,
    
    /// "preTeen"
    #[serde(rename="preTeen")]
    PreTeen,
    
    /// "teen"
    #[serde(rename="teen")]
    Teen,
    
    /// "mature"
    #[serde(rename="mature")]
    Mature,
}

impl AsRef<str> for ProductContentRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductContentRatingEnum::RatingUnknown => "ratingUnknown",
            ProductContentRatingEnum::All => "all",
            ProductContentRatingEnum::PreTeen => "preTeen",
            ProductContentRatingEnum::Teen => "teen",
            ProductContentRatingEnum::Mature => "mature",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductContentRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ratingUnknown" => Ok(ProductContentRatingEnum::RatingUnknown),
           "all" => Ok(ProductContentRatingEnum::All),
           "preTeen" => Ok(ProductContentRatingEnum::PreTeen),
           "teen" => Ok(ProductContentRatingEnum::Teen),
           "mature" => Ok(ProductContentRatingEnum::Mature),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductContentRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductDistributionChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How and to whom the package is made available. The value publicGoogleHosted means that the package is available through the Play store and not restricted to a specific enterprise. The value privateGoogleHosted means that the package is a private app (restricted to an enterprise) but hosted by Google. The value privateSelfHosted means that the package is a private app (restricted to an enterprise) and is privately hosted.
pub enum ProductDistributionChannelEnum {
    
    /// "publicGoogleHosted"
    #[serde(rename="publicGoogleHosted")]
    PublicGoogleHosted,
    
    /// "privateGoogleHosted"
    #[serde(rename="privateGoogleHosted")]
    PrivateGoogleHosted,
    
    /// "privateSelfHosted"
    #[serde(rename="privateSelfHosted")]
    PrivateSelfHosted,
}

impl AsRef<str> for ProductDistributionChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductDistributionChannelEnum::PublicGoogleHosted => "publicGoogleHosted",
            ProductDistributionChannelEnum::PrivateGoogleHosted => "privateGoogleHosted",
            ProductDistributionChannelEnum::PrivateSelfHosted => "privateSelfHosted",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductDistributionChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "publicGoogleHosted" => Ok(ProductDistributionChannelEnum::PublicGoogleHosted),
           "privateGoogleHosted" => Ok(ProductDistributionChannelEnum::PrivateGoogleHosted),
           "privateSelfHosted" => Ok(ProductDistributionChannelEnum::PrivateSelfHosted),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductDistributionChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Noteworthy features (if any) of this product.
pub enum ProductFeaturesEnum {
    
    /// "featureUnknown"
    #[serde(rename="featureUnknown")]
    FeatureUnknown,
    

    /// The app is a VPN.
    ///
    /// "vpnApp"
    #[serde(rename="vpnApp")]
    VpnApp,
}

impl AsRef<str> for ProductFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductFeaturesEnum::FeatureUnknown => "featureUnknown",
            ProductFeaturesEnum::VpnApp => "vpnApp",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "featureUnknown" => Ok(ProductFeaturesEnum::FeatureUnknown),
           "vpnApp" => Ok(ProductFeaturesEnum::VpnApp),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductProductPricingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether this product is free, free with in-app purchases, or paid. If the pricing is unknown, this means the product is not generally available anymore (even though it might still be available to people who own it).
pub enum ProductProductPricingEnum {
    

    /// Unknown pricing, used to denote an approved product that is not generally available.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// The product is free.
    ///
    /// "free"
    #[serde(rename="free")]
    Free,
    

    /// The product is free, but offers in-app purchases.
    ///
    /// "freeWithInAppPurchase"
    #[serde(rename="freeWithInAppPurchase")]
    FreeWithInAppPurchase,
    

    /// The product is paid.
    ///
    /// "paid"
    #[serde(rename="paid")]
    Paid,
}

impl AsRef<str> for ProductProductPricingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductProductPricingEnum::Unknown => "unknown",
            ProductProductPricingEnum::Free => "free",
            ProductProductPricingEnum::FreeWithInAppPurchase => "freeWithInAppPurchase",
            ProductProductPricingEnum::Paid => "paid",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductProductPricingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(ProductProductPricingEnum::Unknown),
           "free" => Ok(ProductProductPricingEnum::Free),
           "freeWithInAppPurchase" => Ok(ProductProductPricingEnum::FreeWithInAppPurchase),
           "paid" => Ok(ProductProductPricingEnum::Paid),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductProductPricingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductApprovalEventApprovedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the product was approved or unapproved. This field will always be present.
pub enum ProductApprovalEventApprovedEnum {
    

    /// Conveys no information.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// The product was approved.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    

    /// The product was unapproved.
    ///
    /// "unapproved"
    #[serde(rename="unapproved")]
    Unapproved,
}

impl AsRef<str> for ProductApprovalEventApprovedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductApprovalEventApprovedEnum::Unknown => "unknown",
            ProductApprovalEventApprovedEnum::Approved => "approved",
            ProductApprovalEventApprovedEnum::Unapproved => "unapproved",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductApprovalEventApprovedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(ProductApprovalEventApprovedEnum::Unknown),
           "approved" => Ok(ProductApprovalEventApprovedEnum::Approved),
           "unapproved" => Ok(ProductApprovalEventApprovedEnum::Unapproved),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductApprovalEventApprovedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductAvailabilityChangeEventAvailabilityStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The new state of the product. This field will always be present.
pub enum ProductAvailabilityChangeEventAvailabilityStatusEnum {
    

    /// Conveys no information.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// The previously unavailable product is again available on Google Play.
    ///
    /// "available"
    #[serde(rename="available")]
    Available,
    

    /// The product was removed from Google Play.
    ///
    /// "removed"
    #[serde(rename="removed")]
    Removed,
    

    /// The product was unpublished by the developer.
    ///
    /// "unpublished"
    #[serde(rename="unpublished")]
    Unpublished,
}

impl AsRef<str> for ProductAvailabilityChangeEventAvailabilityStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductAvailabilityChangeEventAvailabilityStatusEnum::Unknown => "unknown",
            ProductAvailabilityChangeEventAvailabilityStatusEnum::Available => "available",
            ProductAvailabilityChangeEventAvailabilityStatusEnum::Removed => "removed",
            ProductAvailabilityChangeEventAvailabilityStatusEnum::Unpublished => "unpublished",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductAvailabilityChangeEventAvailabilityStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(ProductAvailabilityChangeEventAvailabilityStatusEnum::Unknown),
           "available" => Ok(ProductAvailabilityChangeEventAvailabilityStatusEnum::Available),
           "removed" => Ok(ProductAvailabilityChangeEventAvailabilityStatusEnum::Removed),
           "unpublished" => Ok(ProductAvailabilityChangeEventAvailabilityStatusEnum::Unpublished),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductAvailabilityChangeEventAvailabilityStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductPermissionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the permission has been accepted or not.
pub enum ProductPermissionStateEnum {
    

    /// The permission is required by the app but has not yet been accepted by the enterprise.
    ///
    /// "required"
    #[serde(rename="required")]
    Required,
    

    /// The permission has been accepted by the enterprise.
    ///
    /// "accepted"
    #[serde(rename="accepted")]
    Accepted,
}

impl AsRef<str> for ProductPermissionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductPermissionStateEnum::Required => "required",
            ProductPermissionStateEnum::Accepted => "accepted",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductPermissionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "required" => Ok(ProductPermissionStateEnum::Required),
           "accepted" => Ok(ProductPermissionStateEnum::Accepted),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductPermissionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductPolicyAutoUpdateModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The auto-update mode for the product. When autoUpdateMode is used, it always takes precedence over the user's choice. So when a user makes changes to the device settings manually, these changes are ignored.
pub enum ProductPolicyAutoUpdateModeEnum {
    

    /// Unspecified. Defaults to AUTO_UPDATE_DEFAULT.
    ///
    /// "autoUpdateModeUnspecified"
    #[serde(rename="autoUpdateModeUnspecified")]
    AutoUpdateModeUnspecified,
    

    /// The app is automatically updated with low priority to minimize the impact on the user. The app is updated when the following constraints are met: * The device is not actively used * The device is connected to an unmetered network * The device is charging The device is notified about a new update within 24 hours after it is published by the developer, after which the app is updated the next time the constraints above are met.
    ///
    /// "autoUpdateDefault"
    #[serde(rename="autoUpdateDefault")]
    AutoUpdateDefault,
    

    /// The app is not automatically updated for a maximum of 90 days after the app becomes out of date. 90 days after the app becomes out of date, the latest available version is installed automatically with low priority (see AUTO_UPDATE_DEFAULT). After the app is updated it is not automatically updated again until 90 days after it becomes out of date again. The user can still manually update the app from the Play Store at any time.
    ///
    /// "autoUpdatePostponed"
    #[serde(rename="autoUpdatePostponed")]
    AutoUpdatePostponed,
    

    /// The app is updated as soon as possible. No constraints are applied. The device is notified as soon as possible about a new app update after it is published by the developer.
    ///
    /// "autoUpdateHighPriority"
    #[serde(rename="autoUpdateHighPriority")]
    AutoUpdateHighPriority,
}

impl AsRef<str> for ProductPolicyAutoUpdateModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductPolicyAutoUpdateModeEnum::AutoUpdateModeUnspecified => "autoUpdateModeUnspecified",
            ProductPolicyAutoUpdateModeEnum::AutoUpdateDefault => "autoUpdateDefault",
            ProductPolicyAutoUpdateModeEnum::AutoUpdatePostponed => "autoUpdatePostponed",
            ProductPolicyAutoUpdateModeEnum::AutoUpdateHighPriority => "autoUpdateHighPriority",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductPolicyAutoUpdateModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "autoUpdateModeUnspecified" => Ok(ProductPolicyAutoUpdateModeEnum::AutoUpdateModeUnspecified),
           "autoUpdateDefault" => Ok(ProductPolicyAutoUpdateModeEnum::AutoUpdateDefault),
           "autoUpdatePostponed" => Ok(ProductPolicyAutoUpdateModeEnum::AutoUpdatePostponed),
           "autoUpdateHighPriority" => Ok(ProductPolicyAutoUpdateModeEnum::AutoUpdateHighPriority),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductPolicyAutoUpdateModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductPolicyTracksEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. Use trackIds instead.
pub enum ProductPolicyTracksEnum {
    
    /// "appTrackUnspecified"
    #[serde(rename="appTrackUnspecified")]
    AppTrackUnspecified,
    
    /// "production"
    #[serde(rename="production")]
    Production,
    
    /// "beta"
    #[serde(rename="beta")]
    Beta,
    
    /// "alpha"
    #[serde(rename="alpha")]
    Alpha,
}

impl AsRef<str> for ProductPolicyTracksEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductPolicyTracksEnum::AppTrackUnspecified => "appTrackUnspecified",
            ProductPolicyTracksEnum::Production => "production",
            ProductPolicyTracksEnum::Beta => "beta",
            ProductPolicyTracksEnum::Alpha => "alpha",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductPolicyTracksEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "appTrackUnspecified" => Ok(ProductPolicyTracksEnum::AppTrackUnspecified),
           "production" => Ok(ProductPolicyTracksEnum::Production),
           "beta" => Ok(ProductPolicyTracksEnum::Beta),
           "alpha" => Ok(ProductPolicyTracksEnum::Alpha),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductPolicyTracksEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductSetProductSetBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The interpretation of this product set. "unknown" should never be sent and is ignored if received. "whitelist" means that the user is entitled to access the product set. "includeAll" means that all products are accessible, including products that are approved, products with revoked approval, and products that have never been approved. "allApproved" means that the user is entitled to access all products that are approved for the enterprise. If the value is "allApproved" or "includeAll", the productId field is ignored. If no value is provided, it is interpreted as "whitelist" for backwards compatibility. Further "allApproved" or "includeAll" does not enable automatic visibility of "alpha" or "beta" tracks for Android app. Use ProductVisibility to enable "alpha" or "beta" tracks per user.
pub enum ProductSetProductSetBehaviorEnum {
    

    /// This value should never be sent and ignored if received.
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// This product set constitutes a whitelist.
    ///
    /// "whitelist"
    #[serde(rename="whitelist")]
    Whitelist,
    

    /// This product set represents all products. For Android app it represents only "production" track. (The value of the productId field is therefore ignored).
    ///
    /// "includeAll"
    #[serde(rename="includeAll")]
    IncludeAll,
    

    /// This product set represents all approved products. For Android app it represents only "production" track. (The value of the product_id field is therefore ignored).
    ///
    /// "allApproved"
    #[serde(rename="allApproved")]
    AllApproved,
}

impl AsRef<str> for ProductSetProductSetBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductSetProductSetBehaviorEnum::Unknown => "unknown",
            ProductSetProductSetBehaviorEnum::Whitelist => "whitelist",
            ProductSetProductSetBehaviorEnum::IncludeAll => "includeAll",
            ProductSetProductSetBehaviorEnum::AllApproved => "allApproved",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductSetProductSetBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(ProductSetProductSetBehaviorEnum::Unknown),
           "whitelist" => Ok(ProductSetProductSetBehaviorEnum::Whitelist),
           "includeAll" => Ok(ProductSetProductSetBehaviorEnum::IncludeAll),
           "allApproved" => Ok(ProductSetProductSetBehaviorEnum::AllApproved),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductSetProductSetBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductVisibilityTracksEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated. Use trackIds instead.
pub enum ProductVisibilityTracksEnum {
    
    /// "appTrackUnspecified"
    #[serde(rename="appTrackUnspecified")]
    AppTrackUnspecified,
    
    /// "production"
    #[serde(rename="production")]
    Production,
    
    /// "beta"
    #[serde(rename="beta")]
    Beta,
    
    /// "alpha"
    #[serde(rename="alpha")]
    Alpha,
}

impl AsRef<str> for ProductVisibilityTracksEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductVisibilityTracksEnum::AppTrackUnspecified => "appTrackUnspecified",
            ProductVisibilityTracksEnum::Production => "production",
            ProductVisibilityTracksEnum::Beta => "beta",
            ProductVisibilityTracksEnum::Alpha => "alpha",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductVisibilityTracksEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "appTrackUnspecified" => Ok(ProductVisibilityTracksEnum::AppTrackUnspecified),
           "production" => Ok(ProductVisibilityTracksEnum::Production),
           "beta" => Ok(ProductVisibilityTracksEnum::Beta),
           "alpha" => Ok(ProductVisibilityTracksEnum::Alpha),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductVisibilityTracksEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductsApproveRequestApprovedPermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets how new permission requests for the product are handled. "allPermissions" automatically approves all current and future permissions for the product. "currentPermissionsOnly" approves the current set of permissions for the product, but any future permissions added through updates will require manual reapproval. If not specified, only the current set of permissions will be approved.
pub enum ProductsApproveRequestApprovedPermissionsEnum {
    

    /// Approve only the permissions the product requires at approval time. If an update requires additional permissions, the app will not be updated on devices associated with enterprise users until the additional permissions are approved.
    ///
    /// "currentPermissionsOnly"
    #[serde(rename="currentPermissionsOnly")]
    CurrentPermissionsOnly,
    

    /// All current and future permissions the app requires are automatically approved.
    ///
    /// "allPermissions"
    #[serde(rename="allPermissions")]
    AllPermissions,
}

impl AsRef<str> for ProductsApproveRequestApprovedPermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductsApproveRequestApprovedPermissionsEnum::CurrentPermissionsOnly => "currentPermissionsOnly",
            ProductsApproveRequestApprovedPermissionsEnum::AllPermissions => "allPermissions",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductsApproveRequestApprovedPermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "currentPermissionsOnly" => Ok(ProductsApproveRequestApprovedPermissionsEnum::CurrentPermissionsOnly),
           "allPermissions" => Ok(ProductsApproveRequestApprovedPermissionsEnum::AllPermissions),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductsApproveRequestApprovedPermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServiceAccountKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The file format of the generated key data.
pub enum ServiceAccountKeyTypeEnum {
    

    /// Google Credentials File format.
    ///
    /// "googleCredentials"
    #[serde(rename="googleCredentials")]
    GoogleCredentials,
    

    /// PKCS12 format. The password for the PKCS12 file is 'notasecret'. For more information, see https://tools.ietf.org/html/rfc7292. The data for keys of this type are base64 encoded according to RFC 4648 Section 4. See http://tools.ietf.org/html/rfc4648#section-4.
    ///
    /// "pkcs12"
    #[serde(rename="pkcs12")]
    Pkcs12,
}

impl AsRef<str> for ServiceAccountKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServiceAccountKeyTypeEnum::GoogleCredentials => "googleCredentials",
            ServiceAccountKeyTypeEnum::Pkcs12 => "pkcs12",
        }
    }
}

impl std::convert::TryFrom< &str> for ServiceAccountKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "googleCredentials" => Ok(ServiceAccountKeyTypeEnum::GoogleCredentials),
           "pkcs12" => Ok(ServiceAccountKeyTypeEnum::Pkcs12),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServiceAccountKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StoreLayoutStoreLayoutTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The store layout type. By default, this value is set to "basic" if the homepageId field is not set, and to "custom" otherwise. If set to "basic", the layout will consist of all approved apps that have been whitelisted for the user.
pub enum StoreLayoutStoreLayoutTypeEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "basic"
    #[serde(rename="basic")]
    Basic,
    
    /// "custom"
    #[serde(rename="custom")]
    Custom,
}

impl AsRef<str> for StoreLayoutStoreLayoutTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StoreLayoutStoreLayoutTypeEnum::Unknown => "unknown",
            StoreLayoutStoreLayoutTypeEnum::Basic => "basic",
            StoreLayoutStoreLayoutTypeEnum::Custom => "custom",
        }
    }
}

impl std::convert::TryFrom< &str> for StoreLayoutStoreLayoutTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(StoreLayoutStoreLayoutTypeEnum::Unknown),
           "basic" => Ok(StoreLayoutStoreLayoutTypeEnum::Basic),
           "custom" => Ok(StoreLayoutStoreLayoutTypeEnum::Custom),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StoreLayoutStoreLayoutTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserAccountTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount.
pub enum UserAccountTypeEnum {
    
    /// "deviceAccount"
    #[serde(rename="deviceAccount")]
    DeviceAccount,
    
    /// "userAccount"
    #[serde(rename="userAccount")]
    UserAccount,
}

impl AsRef<str> for UserAccountTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserAccountTypeEnum::DeviceAccount => "deviceAccount",
            UserAccountTypeEnum::UserAccount => "userAccount",
        }
    }
}

impl std::convert::TryFrom< &str> for UserAccountTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "deviceAccount" => Ok(UserAccountTypeEnum::DeviceAccount),
           "userAccount" => Ok(UserAccountTypeEnum::UserAccount),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserAccountTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserManagementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge.
pub enum UserManagementTypeEnum {
    
    /// "googleManaged"
    #[serde(rename="googleManaged")]
    GoogleManaged,
    
    /// "emmManaged"
    #[serde(rename="emmManaged")]
    EmmManaged,
}

impl AsRef<str> for UserManagementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserManagementTypeEnum::GoogleManaged => "googleManaged",
            UserManagementTypeEnum::EmmManaged => "emmManaged",
        }
    }
}

impl std::convert::TryFrom< &str> for UserManagementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "googleManaged" => Ok(UserManagementTypeEnum::GoogleManaged),
           "emmManaged" => Ok(UserManagementTypeEnum::EmmManaged),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserManagementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WebAppDisplayModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The display mode of the web app. Possible values include: - "minimalUi", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - "standalone", the device's status bar and navigation bar are visible when the app is open. - "fullScreen", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area. 
pub enum WebAppDisplayModeEnum {
    
    /// "displayModeUnspecified"
    #[serde(rename="displayModeUnspecified")]
    DisplayModeUnspecified,
    

    /// Opens the web app with a minimal set of browser UI elements for controlling navigation and viewing the page URL.
    ///
    /// "minimalUi"
    #[serde(rename="minimalUi")]
    MinimalUi,
    

    /// Opens the web app to look and feel like a standalone native application. The browser UI elements and page URL are not visible, however the system status bar and back button are visible.
    ///
    /// "standalone"
    #[serde(rename="standalone")]
    Standalone,
    

    /// Opens the web app in full screen without any visible controls. The browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area.
    ///
    /// "fullScreen"
    #[serde(rename="fullScreen")]
    FullScreen,
}

impl AsRef<str> for WebAppDisplayModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebAppDisplayModeEnum::DisplayModeUnspecified => "displayModeUnspecified",
            WebAppDisplayModeEnum::MinimalUi => "minimalUi",
            WebAppDisplayModeEnum::Standalone => "standalone",
            WebAppDisplayModeEnum::FullScreen => "fullScreen",
        }
    }
}

impl std::convert::TryFrom< &str> for WebAppDisplayModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "displayModeUnspecified" => Ok(WebAppDisplayModeEnum::DisplayModeUnspecified),
           "minimalUi" => Ok(WebAppDisplayModeEnum::MinimalUi),
           "standalone" => Ok(WebAppDisplayModeEnum::Standalone),
           "fullScreen" => Ok(WebAppDisplayModeEnum::FullScreen),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebAppDisplayModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether it’s a dedicated device or a knowledge worker device.
pub enum EnterpriseDeviceTypeEnum {
    

    /// This value is unused
    ///
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    

    /// This device is a dedicated device.
    ///
    /// "dedicatedDevice"
    #[serde(rename="dedicatedDevice")]
    DedicatedDevice,
    

    /// This device is required to have an authenticated user.
    ///
    /// "knowledgeWorker"
    #[serde(rename="knowledgeWorker")]
    KnowledgeWorker,
}

impl AsRef<str> for EnterpriseDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseDeviceTypeEnum::Unknown => "unknown",
            EnterpriseDeviceTypeEnum::DedicatedDevice => "dedicatedDevice",
            EnterpriseDeviceTypeEnum::KnowledgeWorker => "knowledgeWorker",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(EnterpriseDeviceTypeEnum::Unknown),
           "dedicatedDevice" => Ok(EnterpriseDeviceTypeEnum::DedicatedDevice),
           "knowledgeWorker" => Ok(EnterpriseDeviceTypeEnum::KnowledgeWorker),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseKeyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of credential to return with the service account. Required.
pub enum EnterpriseKeyTypeEnum {
    

    /// Google Credentials File format.
    ///
    /// "googleCredentials"
    #[serde(rename="googleCredentials")]
    GoogleCredentials,
    

    /// PKCS12 format. The password for the PKCS12 file is 'notasecret'. For more information, see https://tools.ietf.org/html/rfc7292. The data for keys of this type are base64 encoded according to RFC 4648 Section 4. See http://tools.ietf.org/html/rfc4648#section-4.
    ///
    /// "pkcs12"
    #[serde(rename="pkcs12")]
    Pkcs12,
}

impl AsRef<str> for EnterpriseKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseKeyTypeEnum::GoogleCredentials => "googleCredentials",
            EnterpriseKeyTypeEnum::Pkcs12 => "pkcs12",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseKeyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "googleCredentials" => Ok(EnterpriseKeyTypeEnum::GoogleCredentials),
           "pkcs12" => Ok(EnterpriseKeyTypeEnum::Pkcs12),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseKeyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseRequestModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The request mode for pulling notifications. Specifying waitForNotifications will cause the request to block and wait until one or more notifications are present, or return an empty notification list if no notifications are present after some time. Specifying returnImmediately will cause the request to immediately return the pending notifications, or an empty list if no notifications are present. If omitted, defaults to waitForNotifications.
pub enum EnterpriseRequestModeEnum {
    

    /// Wait until one or more notifications are present.
    ///
    /// "waitForNotifications"
    #[serde(rename="waitForNotifications")]
    WaitForNotifications,
    

    /// Returns immediately whether notifications are present or not.
    ///
    /// "returnImmediately"
    #[serde(rename="returnImmediately")]
    ReturnImmediately,
}

impl AsRef<str> for EnterpriseRequestModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseRequestModeEnum::WaitForNotifications => "waitForNotifications",
            EnterpriseRequestModeEnum::ReturnImmediately => "returnImmediately",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseRequestModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "waitForNotifications" => Ok(EnterpriseRequestModeEnum::WaitForNotifications),
           "returnImmediately" => Ok(EnterpriseRequestModeEnum::ReturnImmediately),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseRequestModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


