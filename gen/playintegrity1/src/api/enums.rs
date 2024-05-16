use super::*;



// region AccountActivityActivityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the activity level of the account.
pub enum AccountActivityActivityLevelEnum {
    

    /// Activity level has not been set.
    ///
    /// "ACTIVITY_LEVEL_UNSPECIFIED"
    #[serde(rename="ACTIVITY_LEVEL_UNSPECIFIED")]
    ACTIVITYLEVELUNSPECIFIED,
    

    /// Account activity level is not evaluated.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
    

    /// Unusual activity for at least one of the user accounts on the device.
    ///
    /// "UNUSUAL"
    #[serde(rename="UNUSUAL")]
    UNUSUAL,
    

    /// Insufficient activity to verify the user account on the device.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Typical activity for the user account or accounts on the device.
    ///
    /// "TYPICAL_BASIC"
    #[serde(rename="TYPICAL_BASIC")]
    TYPICALBASIC,
    

    /// Typical for the user account or accounts on the device, with harder to replicate signals.
    ///
    /// "TYPICAL_STRONG"
    #[serde(rename="TYPICAL_STRONG")]
    TYPICALSTRONG,
}

impl AsRef<str> for AccountActivityActivityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountActivityActivityLevelEnum::ACTIVITYLEVELUNSPECIFIED => "ACTIVITY_LEVEL_UNSPECIFIED",
            AccountActivityActivityLevelEnum::UNEVALUATED => "UNEVALUATED",
            AccountActivityActivityLevelEnum::UNUSUAL => "UNUSUAL",
            AccountActivityActivityLevelEnum::UNKNOWN => "UNKNOWN",
            AccountActivityActivityLevelEnum::TYPICALBASIC => "TYPICAL_BASIC",
            AccountActivityActivityLevelEnum::TYPICALSTRONG => "TYPICAL_STRONG",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountActivityActivityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVITY_LEVEL_UNSPECIFIED" => Ok(AccountActivityActivityLevelEnum::ACTIVITYLEVELUNSPECIFIED),
           "UNEVALUATED" => Ok(AccountActivityActivityLevelEnum::UNEVALUATED),
           "UNUSUAL" => Ok(AccountActivityActivityLevelEnum::UNUSUAL),
           "UNKNOWN" => Ok(AccountActivityActivityLevelEnum::UNKNOWN),
           "TYPICAL_BASIC" => Ok(AccountActivityActivityLevelEnum::TYPICALBASIC),
           "TYPICAL_STRONG" => Ok(AccountActivityActivityLevelEnum::TYPICALSTRONG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountActivityActivityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountDetailAppLicensingVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Details about the licensing status of the user for the app in the scope.
pub enum AccountDetailAppLicensingVerdictEnum {
    

    /// Play does not have sufficient information to evaluate licensing details
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The app and certificate match the versions distributed by Play.
    ///
    /// "LICENSED"
    #[serde(rename="LICENSED")]
    LICENSED,
    

    /// The certificate or package name does not match Google Play records.
    ///
    /// "UNLICENSED"
    #[serde(rename="UNLICENSED")]
    UNLICENSED,
    

    /// Licensing details were not evaluated since a necessary requirement was missed. For example DeviceIntegrity did not meet the minimum bar or the application was not a known Play version.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
}

impl AsRef<str> for AccountDetailAppLicensingVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountDetailAppLicensingVerdictEnum::UNKNOWN => "UNKNOWN",
            AccountDetailAppLicensingVerdictEnum::LICENSED => "LICENSED",
            AccountDetailAppLicensingVerdictEnum::UNLICENSED => "UNLICENSED",
            AccountDetailAppLicensingVerdictEnum::UNEVALUATED => "UNEVALUATED",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountDetailAppLicensingVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(AccountDetailAppLicensingVerdictEnum::UNKNOWN),
           "LICENSED" => Ok(AccountDetailAppLicensingVerdictEnum::LICENSED),
           "UNLICENSED" => Ok(AccountDetailAppLicensingVerdictEnum::UNLICENSED),
           "UNEVALUATED" => Ok(AccountDetailAppLicensingVerdictEnum::UNEVALUATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountDetailAppLicensingVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppAccessRiskVerdictOtherAppsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. App access risk verdict related to apps that are not installed by Google Play, and are not preloaded on the system image by the device manufacturer.
pub enum AppAccessRiskVerdictOtherAppsEnum {
    

    /// Risk type is unknown.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// App access risk was not evaluated because a requirement was missed, such as the device not being trusted enough.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
    

    /// No apps under this field are installed on the device. This is only valid for the other apps field.
    ///
    /// "NOT_INSTALLED"
    #[serde(rename="NOT_INSTALLED")]
    NOTINSTALLED,
    

    /// One or more apps under this field are installed on the device.
    ///
    /// "INSTALLED"
    #[serde(rename="INSTALLED")]
    INSTALLED,
    

    /// Apps under this field are running that could be used to read or capture inputs and outputs of the requesting app, such as screen recording apps.
    ///
    /// "CAPTURING"
    #[serde(rename="CAPTURING")]
    CAPTURING,
    

    /// Apps under this field are running that could be used to control the device and inputs and outputs of the requesting app, such as remote controlling apps.
    ///
    /// "CONTROLLING"
    #[serde(rename="CONTROLLING")]
    CONTROLLING,
}

impl AsRef<str> for AppAccessRiskVerdictOtherAppsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppAccessRiskVerdictOtherAppsEnum::UNKNOWN => "UNKNOWN",
            AppAccessRiskVerdictOtherAppsEnum::UNEVALUATED => "UNEVALUATED",
            AppAccessRiskVerdictOtherAppsEnum::NOTINSTALLED => "NOT_INSTALLED",
            AppAccessRiskVerdictOtherAppsEnum::INSTALLED => "INSTALLED",
            AppAccessRiskVerdictOtherAppsEnum::CAPTURING => "CAPTURING",
            AppAccessRiskVerdictOtherAppsEnum::CONTROLLING => "CONTROLLING",
        }
    }
}

impl std::convert::TryFrom< &str> for AppAccessRiskVerdictOtherAppsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(AppAccessRiskVerdictOtherAppsEnum::UNKNOWN),
           "UNEVALUATED" => Ok(AppAccessRiskVerdictOtherAppsEnum::UNEVALUATED),
           "NOT_INSTALLED" => Ok(AppAccessRiskVerdictOtherAppsEnum::NOTINSTALLED),
           "INSTALLED" => Ok(AppAccessRiskVerdictOtherAppsEnum::INSTALLED),
           "CAPTURING" => Ok(AppAccessRiskVerdictOtherAppsEnum::CAPTURING),
           "CONTROLLING" => Ok(AppAccessRiskVerdictOtherAppsEnum::CONTROLLING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppAccessRiskVerdictOtherAppsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppAccessRiskVerdictPlayOrSystemAppsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. App access risk verdict related to apps that are not installed by the Google Play Store, and are not preloaded on the system image by the device manufacturer.
pub enum AppAccessRiskVerdictPlayOrSystemAppsEnum {
    

    /// Risk type is unknown.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// App access risk was not evaluated because a requirement was missed, such as the device not being trusted enough.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
    

    /// No apps under this field are installed on the device. This is only valid for the other apps field.
    ///
    /// "NOT_INSTALLED"
    #[serde(rename="NOT_INSTALLED")]
    NOTINSTALLED,
    

    /// One or more apps under this field are installed on the device.
    ///
    /// "INSTALLED"
    #[serde(rename="INSTALLED")]
    INSTALLED,
    

    /// Apps under this field are running that could be used to read or capture inputs and outputs of the requesting app, such as screen recording apps.
    ///
    /// "CAPTURING"
    #[serde(rename="CAPTURING")]
    CAPTURING,
    

    /// Apps under this field are running that could be used to control the device and inputs and outputs of the requesting app, such as remote controlling apps.
    ///
    /// "CONTROLLING"
    #[serde(rename="CONTROLLING")]
    CONTROLLING,
}

impl AsRef<str> for AppAccessRiskVerdictPlayOrSystemAppsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppAccessRiskVerdictPlayOrSystemAppsEnum::UNKNOWN => "UNKNOWN",
            AppAccessRiskVerdictPlayOrSystemAppsEnum::UNEVALUATED => "UNEVALUATED",
            AppAccessRiskVerdictPlayOrSystemAppsEnum::NOTINSTALLED => "NOT_INSTALLED",
            AppAccessRiskVerdictPlayOrSystemAppsEnum::INSTALLED => "INSTALLED",
            AppAccessRiskVerdictPlayOrSystemAppsEnum::CAPTURING => "CAPTURING",
            AppAccessRiskVerdictPlayOrSystemAppsEnum::CONTROLLING => "CONTROLLING",
        }
    }
}

impl std::convert::TryFrom< &str> for AppAccessRiskVerdictPlayOrSystemAppsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(AppAccessRiskVerdictPlayOrSystemAppsEnum::UNKNOWN),
           "UNEVALUATED" => Ok(AppAccessRiskVerdictPlayOrSystemAppsEnum::UNEVALUATED),
           "NOT_INSTALLED" => Ok(AppAccessRiskVerdictPlayOrSystemAppsEnum::NOTINSTALLED),
           "INSTALLED" => Ok(AppAccessRiskVerdictPlayOrSystemAppsEnum::INSTALLED),
           "CAPTURING" => Ok(AppAccessRiskVerdictPlayOrSystemAppsEnum::CAPTURING),
           "CONTROLLING" => Ok(AppAccessRiskVerdictPlayOrSystemAppsEnum::CONTROLLING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppAccessRiskVerdictPlayOrSystemAppsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppIntegrityAppRecognitionVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Details about the app recognition verdict
pub enum AppIntegrityAppRecognitionVerdictEnum {
    

    /// Play does not have sufficient information to evaluate app integrity
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The app and certificate match the versions distributed by Play.
    ///
    /// "PLAY_RECOGNIZED"
    #[serde(rename="PLAY_RECOGNIZED")]
    PLAYRECOGNIZED,
    

    /// The certificate or package name does not match Google Play records.
    ///
    /// "UNRECOGNIZED_VERSION"
    #[serde(rename="UNRECOGNIZED_VERSION")]
    UNRECOGNIZEDVERSION,
    

    /// Application integrity was not evaluated since a necessary requirement was missed. For example DeviceIntegrity did not meet the minimum bar.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
}

impl AsRef<str> for AppIntegrityAppRecognitionVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppIntegrityAppRecognitionVerdictEnum::UNKNOWN => "UNKNOWN",
            AppIntegrityAppRecognitionVerdictEnum::PLAYRECOGNIZED => "PLAY_RECOGNIZED",
            AppIntegrityAppRecognitionVerdictEnum::UNRECOGNIZEDVERSION => "UNRECOGNIZED_VERSION",
            AppIntegrityAppRecognitionVerdictEnum::UNEVALUATED => "UNEVALUATED",
        }
    }
}

impl std::convert::TryFrom< &str> for AppIntegrityAppRecognitionVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(AppIntegrityAppRecognitionVerdictEnum::UNKNOWN),
           "PLAY_RECOGNIZED" => Ok(AppIntegrityAppRecognitionVerdictEnum::PLAYRECOGNIZED),
           "UNRECOGNIZED_VERSION" => Ok(AppIntegrityAppRecognitionVerdictEnum::UNRECOGNIZEDVERSION),
           "UNEVALUATED" => Ok(AppIntegrityAppRecognitionVerdictEnum::UNEVALUATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppIntegrityAppRecognitionVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceIntegrityDeviceRecognitionVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Details about the integrity of the device the app is running on.
pub enum DeviceIntegrityDeviceRecognitionVerdictEnum {
    

    /// Play does not have sufficient information to evaluate device integrity
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// App is running on a device that passes basic system integrity checks, but may not meet Android platform compatibility requirements and may not be approved to run Google Play services.
    ///
    /// "MEETS_BASIC_INTEGRITY"
    #[serde(rename="MEETS_BASIC_INTEGRITY")]
    MEETSBASICINTEGRITY,
    

    /// App is running on GMS Android device with Google Play services.
    ///
    /// "MEETS_DEVICE_INTEGRITY"
    #[serde(rename="MEETS_DEVICE_INTEGRITY")]
    MEETSDEVICEINTEGRITY,
    

    /// App is running on GMS Android device with Google Play services and has a strong guarantee of system integrity such as a hardware-backed keystore.
    ///
    /// "MEETS_STRONG_INTEGRITY"
    #[serde(rename="MEETS_STRONG_INTEGRITY")]
    MEETSSTRONGINTEGRITY,
    

    /// App is running on an Android emulator with Google Play services which meets core Android compatibility requirements.
    ///
    /// "MEETS_VIRTUAL_INTEGRITY"
    #[serde(rename="MEETS_VIRTUAL_INTEGRITY")]
    MEETSVIRTUALINTEGRITY,
}

impl AsRef<str> for DeviceIntegrityDeviceRecognitionVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceIntegrityDeviceRecognitionVerdictEnum::UNKNOWN => "UNKNOWN",
            DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSBASICINTEGRITY => "MEETS_BASIC_INTEGRITY",
            DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSDEVICEINTEGRITY => "MEETS_DEVICE_INTEGRITY",
            DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSSTRONGINTEGRITY => "MEETS_STRONG_INTEGRITY",
            DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSVIRTUALINTEGRITY => "MEETS_VIRTUAL_INTEGRITY",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceIntegrityDeviceRecognitionVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(DeviceIntegrityDeviceRecognitionVerdictEnum::UNKNOWN),
           "MEETS_BASIC_INTEGRITY" => Ok(DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSBASICINTEGRITY),
           "MEETS_DEVICE_INTEGRITY" => Ok(DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSDEVICEINTEGRITY),
           "MEETS_STRONG_INTEGRITY" => Ok(DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSSTRONGINTEGRITY),
           "MEETS_VIRTUAL_INTEGRITY" => Ok(DeviceIntegrityDeviceRecognitionVerdictEnum::MEETSVIRTUALINTEGRITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceIntegrityDeviceRecognitionVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentDetailPlayProtectVerdictEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The evaluation of Play Protect verdict.
pub enum EnvironmentDetailPlayProtectVerdictEnum {
    

    /// Play Protect verdict has not been set.
    ///
    /// "PLAY_PROTECT_VERDICT_UNSPECIFIED"
    #[serde(rename="PLAY_PROTECT_VERDICT_UNSPECIFIED")]
    PLAYPROTECTVERDICTUNSPECIFIED,
    

    /// Play Protect state was not evaluated. Device may not be trusted.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
    

    /// Play Protect is on and no issues found.
    ///
    /// "NO_ISSUES"
    #[serde(rename="NO_ISSUES")]
    NOISSUES,
    

    /// Play Protect is on but no scan has been performed yet. The device or Play Store app may have been reset.
    ///
    /// "NO_DATA"
    #[serde(rename="NO_DATA")]
    NODATA,
    

    /// Play Protect is on and warnings found.
    ///
    /// "MEDIUM_RISK"
    #[serde(rename="MEDIUM_RISK")]
    MEDIUMRISK,
    

    /// Play Protect is on and high severity issues found.
    ///
    /// "HIGH_RISK"
    #[serde(rename="HIGH_RISK")]
    HIGHRISK,
    

    /// Play Protect is turned off. Turn on Play Protect.
    ///
    /// "POSSIBLE_RISK"
    #[serde(rename="POSSIBLE_RISK")]
    POSSIBLERISK,
}

impl AsRef<str> for EnvironmentDetailPlayProtectVerdictEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentDetailPlayProtectVerdictEnum::PLAYPROTECTVERDICTUNSPECIFIED => "PLAY_PROTECT_VERDICT_UNSPECIFIED",
            EnvironmentDetailPlayProtectVerdictEnum::UNEVALUATED => "UNEVALUATED",
            EnvironmentDetailPlayProtectVerdictEnum::NOISSUES => "NO_ISSUES",
            EnvironmentDetailPlayProtectVerdictEnum::NODATA => "NO_DATA",
            EnvironmentDetailPlayProtectVerdictEnum::MEDIUMRISK => "MEDIUM_RISK",
            EnvironmentDetailPlayProtectVerdictEnum::HIGHRISK => "HIGH_RISK",
            EnvironmentDetailPlayProtectVerdictEnum::POSSIBLERISK => "POSSIBLE_RISK",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentDetailPlayProtectVerdictEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLAY_PROTECT_VERDICT_UNSPECIFIED" => Ok(EnvironmentDetailPlayProtectVerdictEnum::PLAYPROTECTVERDICTUNSPECIFIED),
           "UNEVALUATED" => Ok(EnvironmentDetailPlayProtectVerdictEnum::UNEVALUATED),
           "NO_ISSUES" => Ok(EnvironmentDetailPlayProtectVerdictEnum::NOISSUES),
           "NO_DATA" => Ok(EnvironmentDetailPlayProtectVerdictEnum::NODATA),
           "MEDIUM_RISK" => Ok(EnvironmentDetailPlayProtectVerdictEnum::MEDIUMRISK),
           "HIGH_RISK" => Ok(EnvironmentDetailPlayProtectVerdictEnum::HIGHRISK),
           "POSSIBLE_RISK" => Ok(EnvironmentDetailPlayProtectVerdictEnum::POSSIBLERISK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentDetailPlayProtectVerdictEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RecentDeviceActivityDeviceActivityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the activity level of the device.
pub enum RecentDeviceActivityDeviceActivityLevelEnum {
    

    /// Device activity level has not been set.
    ///
    /// "DEVICE_ACTIVITY_LEVEL_UNSPECIFIED"
    #[serde(rename="DEVICE_ACTIVITY_LEVEL_UNSPECIFIED")]
    DEVICEACTIVITYLEVELUNSPECIFIED,
    

    /// Device activity level has not been evaluated.
    ///
    /// "UNEVALUATED"
    #[serde(rename="UNEVALUATED")]
    UNEVALUATED,
    

    /// Indicates the amount of used tokens. See the documentation for details.
    ///
    /// "LEVEL_1"
    #[serde(rename="LEVEL_1")]
    LEVEL1,
    

    /// Indicates the amount of used tokens. See the documentation for details.
    ///
    /// "LEVEL_2"
    #[serde(rename="LEVEL_2")]
    LEVEL2,
    

    /// Indicates the amount of used tokens. See the documentation for details.
    ///
    /// "LEVEL_3"
    #[serde(rename="LEVEL_3")]
    LEVEL3,
    

    /// Indicates the amount of used tokens. See the documentation for details.
    ///
    /// "LEVEL_4"
    #[serde(rename="LEVEL_4")]
    LEVEL4,
}

impl AsRef<str> for RecentDeviceActivityDeviceActivityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RecentDeviceActivityDeviceActivityLevelEnum::DEVICEACTIVITYLEVELUNSPECIFIED => "DEVICE_ACTIVITY_LEVEL_UNSPECIFIED",
            RecentDeviceActivityDeviceActivityLevelEnum::UNEVALUATED => "UNEVALUATED",
            RecentDeviceActivityDeviceActivityLevelEnum::LEVEL1 => "LEVEL_1",
            RecentDeviceActivityDeviceActivityLevelEnum::LEVEL2 => "LEVEL_2",
            RecentDeviceActivityDeviceActivityLevelEnum::LEVEL3 => "LEVEL_3",
            RecentDeviceActivityDeviceActivityLevelEnum::LEVEL4 => "LEVEL_4",
        }
    }
}

impl std::convert::TryFrom< &str> for RecentDeviceActivityDeviceActivityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_ACTIVITY_LEVEL_UNSPECIFIED" => Ok(RecentDeviceActivityDeviceActivityLevelEnum::DEVICEACTIVITYLEVELUNSPECIFIED),
           "UNEVALUATED" => Ok(RecentDeviceActivityDeviceActivityLevelEnum::UNEVALUATED),
           "LEVEL_1" => Ok(RecentDeviceActivityDeviceActivityLevelEnum::LEVEL1),
           "LEVEL_2" => Ok(RecentDeviceActivityDeviceActivityLevelEnum::LEVEL2),
           "LEVEL_3" => Ok(RecentDeviceActivityDeviceActivityLevelEnum::LEVEL3),
           "LEVEL_4" => Ok(RecentDeviceActivityDeviceActivityLevelEnum::LEVEL4),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RecentDeviceActivityDeviceActivityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


