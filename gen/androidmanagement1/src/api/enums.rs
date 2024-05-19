use super::*;



// region AdvancedSecurityOverrideCommonCriteriaModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls Common Criteria Mode—security standards defined in the Common Criteria for Information Technology Security Evaluation (https://www.commoncriteriaportal.org/) (CC). Enabling Common Criteria Mode increases certain security components on a device, including AES-GCM encryption of Bluetooth Long Term Keys, and Wi-Fi configuration stores.Warning: Common Criteria Mode enforces a strict security model typically only required for IT products used in national security systems and other highly sensitive organizations. Standard device use may be affected. Only enabled if required.
pub enum AdvancedSecurityOverrideCommonCriteriaModeEnum {
    

    /// Unspecified. Defaults to COMMON_CRITERIA_MODE_DISABLED.
    ///
    /// "COMMON_CRITERIA_MODE_UNSPECIFIED"
    #[serde(rename="COMMON_CRITERIA_MODE_UNSPECIFIED")]
    COMMONCRITERIAMODEUNSPECIFIED,
    

    /// Default. Disables Common Criteria Mode.
    ///
    /// "COMMON_CRITERIA_MODE_DISABLED"
    #[serde(rename="COMMON_CRITERIA_MODE_DISABLED")]
    COMMONCRITERIAMODEDISABLED,
    

    /// Enables Common Criteria Mode.
    ///
    /// "COMMON_CRITERIA_MODE_ENABLED"
    #[serde(rename="COMMON_CRITERIA_MODE_ENABLED")]
    COMMONCRITERIAMODEENABLED,
}

impl AsRef<str> for AdvancedSecurityOverrideCommonCriteriaModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvancedSecurityOverrideCommonCriteriaModeEnum::COMMONCRITERIAMODEUNSPECIFIED => "COMMON_CRITERIA_MODE_UNSPECIFIED",
            AdvancedSecurityOverrideCommonCriteriaModeEnum::COMMONCRITERIAMODEDISABLED => "COMMON_CRITERIA_MODE_DISABLED",
            AdvancedSecurityOverrideCommonCriteriaModeEnum::COMMONCRITERIAMODEENABLED => "COMMON_CRITERIA_MODE_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvancedSecurityOverrideCommonCriteriaModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMON_CRITERIA_MODE_UNSPECIFIED" => Ok(AdvancedSecurityOverrideCommonCriteriaModeEnum::COMMONCRITERIAMODEUNSPECIFIED),
           "COMMON_CRITERIA_MODE_DISABLED" => Ok(AdvancedSecurityOverrideCommonCriteriaModeEnum::COMMONCRITERIAMODEDISABLED),
           "COMMON_CRITERIA_MODE_ENABLED" => Ok(AdvancedSecurityOverrideCommonCriteriaModeEnum::COMMONCRITERIAMODEENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvancedSecurityOverrideCommonCriteriaModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvancedSecurityOverrideDeveloperSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls access to developer settings: developer options and safe boot. Replaces safeBootDisabled (deprecated) and debuggingFeaturesAllowed (deprecated).
pub enum AdvancedSecurityOverrideDeveloperSettingsEnum {
    

    /// Unspecified. Defaults to DEVELOPER_SETTINGS_DISABLED.
    ///
    /// "DEVELOPER_SETTINGS_UNSPECIFIED"
    #[serde(rename="DEVELOPER_SETTINGS_UNSPECIFIED")]
    DEVELOPERSETTINGSUNSPECIFIED,
    

    /// Default. Disables all developer settings and prevents the user from accessing them.
    ///
    /// "DEVELOPER_SETTINGS_DISABLED"
    #[serde(rename="DEVELOPER_SETTINGS_DISABLED")]
    DEVELOPERSETTINGSDISABLED,
    

    /// Allows all developer settings. The user can access and optionally configure the settings.
    ///
    /// "DEVELOPER_SETTINGS_ALLOWED"
    #[serde(rename="DEVELOPER_SETTINGS_ALLOWED")]
    DEVELOPERSETTINGSALLOWED,
}

impl AsRef<str> for AdvancedSecurityOverrideDeveloperSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvancedSecurityOverrideDeveloperSettingsEnum::DEVELOPERSETTINGSUNSPECIFIED => "DEVELOPER_SETTINGS_UNSPECIFIED",
            AdvancedSecurityOverrideDeveloperSettingsEnum::DEVELOPERSETTINGSDISABLED => "DEVELOPER_SETTINGS_DISABLED",
            AdvancedSecurityOverrideDeveloperSettingsEnum::DEVELOPERSETTINGSALLOWED => "DEVELOPER_SETTINGS_ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvancedSecurityOverrideDeveloperSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVELOPER_SETTINGS_UNSPECIFIED" => Ok(AdvancedSecurityOverrideDeveloperSettingsEnum::DEVELOPERSETTINGSUNSPECIFIED),
           "DEVELOPER_SETTINGS_DISABLED" => Ok(AdvancedSecurityOverrideDeveloperSettingsEnum::DEVELOPERSETTINGSDISABLED),
           "DEVELOPER_SETTINGS_ALLOWED" => Ok(AdvancedSecurityOverrideDeveloperSettingsEnum::DEVELOPERSETTINGSALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvancedSecurityOverrideDeveloperSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether Google Play Protect verification (https://support.google.com/accounts/answer/2812853) is enforced. Replaces ensureVerifyAppsEnabled (deprecated).
pub enum AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum {
    

    /// Unspecified. Defaults to VERIFY_APPS_ENFORCED.
    ///
    /// "GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED"
    #[serde(rename="GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED")]
    GOOGLEPLAYPROTECTVERIFYAPPSUNSPECIFIED,
    

    /// Default. Force-enables app verification.
    ///
    /// "VERIFY_APPS_ENFORCED"
    #[serde(rename="VERIFY_APPS_ENFORCED")]
    VERIFYAPPSENFORCED,
    

    /// Allows the user to choose whether to enable app verification.
    ///
    /// "VERIFY_APPS_USER_CHOICE"
    #[serde(rename="VERIFY_APPS_USER_CHOICE")]
    VERIFYAPPSUSERCHOICE,
}

impl AsRef<str> for AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum::GOOGLEPLAYPROTECTVERIFYAPPSUNSPECIFIED => "GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED",
            AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum::VERIFYAPPSENFORCED => "VERIFY_APPS_ENFORCED",
            AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum::VERIFYAPPSUSERCHOICE => "VERIFY_APPS_USER_CHOICE",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GOOGLE_PLAY_PROTECT_VERIFY_APPS_UNSPECIFIED" => Ok(AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum::GOOGLEPLAYPROTECTVERIFYAPPSUNSPECIFIED),
           "VERIFY_APPS_ENFORCED" => Ok(AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum::VERIFYAPPSENFORCED),
           "VERIFY_APPS_USER_CHOICE" => Ok(AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum::VERIFYAPPSUSERCHOICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvancedSecurityOverrideGooglePlayProtectVerifyAppsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvancedSecurityOverrideMtePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Controls Memory Tagging Extension (MTE) (https://source.android.com/docs/security/test/memory-safety/arm-mte) on the device. The device needs to be rebooted to apply changes to the MTE policy.
pub enum AdvancedSecurityOverrideMtePolicyEnum {
    

    /// Unspecified. Defaults to MTE_USER_CHOICE.
    ///
    /// "MTE_POLICY_UNSPECIFIED"
    #[serde(rename="MTE_POLICY_UNSPECIFIED")]
    MTEPOLICYUNSPECIFIED,
    

    /// The user can choose to enable or disable MTE on the device if the device supports this.
    ///
    /// "MTE_USER_CHOICE"
    #[serde(rename="MTE_USER_CHOICE")]
    MTEUSERCHOICE,
    

    /// MTE is enabled on the device and the user is not allowed to change this setting. This can be set on fully managed devices and work profiles on company-owned devices. A nonComplianceDetail with MANAGEMENT_MODE is reported for other management modes. A nonComplianceDetail with DEVICE_INCOMPATIBLE is reported if the device does not support MTE.Supported on Android 14 and above. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 14.
    ///
    /// "MTE_ENFORCED"
    #[serde(rename="MTE_ENFORCED")]
    MTEENFORCED,
    

    /// MTE is disabled on the device and the user is not allowed to change this setting. This applies only on fully managed devices. In other cases, a nonComplianceDetail with MANAGEMENT_MODE is reported. A nonComplianceDetail with DEVICE_INCOMPATIBLE is reported if the device does not support MTE.Supported on Android 14 and above. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 14.
    ///
    /// "MTE_DISABLED"
    #[serde(rename="MTE_DISABLED")]
    MTEDISABLED,
}

impl AsRef<str> for AdvancedSecurityOverrideMtePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvancedSecurityOverrideMtePolicyEnum::MTEPOLICYUNSPECIFIED => "MTE_POLICY_UNSPECIFIED",
            AdvancedSecurityOverrideMtePolicyEnum::MTEUSERCHOICE => "MTE_USER_CHOICE",
            AdvancedSecurityOverrideMtePolicyEnum::MTEENFORCED => "MTE_ENFORCED",
            AdvancedSecurityOverrideMtePolicyEnum::MTEDISABLED => "MTE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvancedSecurityOverrideMtePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MTE_POLICY_UNSPECIFIED" => Ok(AdvancedSecurityOverrideMtePolicyEnum::MTEPOLICYUNSPECIFIED),
           "MTE_USER_CHOICE" => Ok(AdvancedSecurityOverrideMtePolicyEnum::MTEUSERCHOICE),
           "MTE_ENFORCED" => Ok(AdvancedSecurityOverrideMtePolicyEnum::MTEENFORCED),
           "MTE_DISABLED" => Ok(AdvancedSecurityOverrideMtePolicyEnum::MTEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvancedSecurityOverrideMtePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvancedSecurityOverrideUntrustedAppsPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The policy for untrusted apps (apps from unknown sources) enforced on the device. Replaces install_unknown_sources_allowed (deprecated).
pub enum AdvancedSecurityOverrideUntrustedAppsPolicyEnum {
    

    /// Unspecified. Defaults to DISALLOW_INSTALL.
    ///
    /// "UNTRUSTED_APPS_POLICY_UNSPECIFIED"
    #[serde(rename="UNTRUSTED_APPS_POLICY_UNSPECIFIED")]
    UNTRUSTEDAPPSPOLICYUNSPECIFIED,
    

    /// Default. Disallow untrusted app installs on entire device.
    ///
    /// "DISALLOW_INSTALL"
    #[serde(rename="DISALLOW_INSTALL")]
    DISALLOWINSTALL,
    

    /// For devices with work profiles, allow untrusted app installs in the device's personal profile only.
    ///
    /// "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY"
    #[serde(rename="ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY")]
    ALLOWINSTALLINPERSONALPROFILEONLY,
    

    /// Allow untrusted app installs on entire device.
    ///
    /// "ALLOW_INSTALL_DEVICE_WIDE"
    #[serde(rename="ALLOW_INSTALL_DEVICE_WIDE")]
    ALLOWINSTALLDEVICEWIDE,
}

impl AsRef<str> for AdvancedSecurityOverrideUntrustedAppsPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvancedSecurityOverrideUntrustedAppsPolicyEnum::UNTRUSTEDAPPSPOLICYUNSPECIFIED => "UNTRUSTED_APPS_POLICY_UNSPECIFIED",
            AdvancedSecurityOverrideUntrustedAppsPolicyEnum::DISALLOWINSTALL => "DISALLOW_INSTALL",
            AdvancedSecurityOverrideUntrustedAppsPolicyEnum::ALLOWINSTALLINPERSONALPROFILEONLY => "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY",
            AdvancedSecurityOverrideUntrustedAppsPolicyEnum::ALLOWINSTALLDEVICEWIDE => "ALLOW_INSTALL_DEVICE_WIDE",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvancedSecurityOverrideUntrustedAppsPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNTRUSTED_APPS_POLICY_UNSPECIFIED" => Ok(AdvancedSecurityOverrideUntrustedAppsPolicyEnum::UNTRUSTEDAPPSPOLICYUNSPECIFIED),
           "DISALLOW_INSTALL" => Ok(AdvancedSecurityOverrideUntrustedAppsPolicyEnum::DISALLOWINSTALL),
           "ALLOW_INSTALL_IN_PERSONAL_PROFILE_ONLY" => Ok(AdvancedSecurityOverrideUntrustedAppsPolicyEnum::ALLOWINSTALLINPERSONALPROFILEONLY),
           "ALLOW_INSTALL_DEVICE_WIDE" => Ok(AdvancedSecurityOverrideUntrustedAppsPolicyEnum::ALLOWINSTALLDEVICEWIDE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvancedSecurityOverrideUntrustedAppsPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationAppPricingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether this app is free, free with in-app purchases, or paid. If the pricing is unspecified, this means the app is not generally available anymore (even though it might still be available to people who own it).
pub enum ApplicationAppPricingEnum {
    

    /// Unknown pricing, used to denote an approved app that is not generally available.
    ///
    /// "APP_PRICING_UNSPECIFIED"
    #[serde(rename="APP_PRICING_UNSPECIFIED")]
    APPPRICINGUNSPECIFIED,
    

    /// The app is free.
    ///
    /// "FREE"
    #[serde(rename="FREE")]
    FREE,
    

    /// The app is free, but offers in-app purchases.
    ///
    /// "FREE_WITH_IN_APP_PURCHASE"
    #[serde(rename="FREE_WITH_IN_APP_PURCHASE")]
    FREEWITHINAPPPURCHASE,
    

    /// The app is paid.
    ///
    /// "PAID"
    #[serde(rename="PAID")]
    PAID,
}

impl AsRef<str> for ApplicationAppPricingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationAppPricingEnum::APPPRICINGUNSPECIFIED => "APP_PRICING_UNSPECIFIED",
            ApplicationAppPricingEnum::FREE => "FREE",
            ApplicationAppPricingEnum::FREEWITHINAPPPURCHASE => "FREE_WITH_IN_APP_PURCHASE",
            ApplicationAppPricingEnum::PAID => "PAID",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationAppPricingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_PRICING_UNSPECIFIED" => Ok(ApplicationAppPricingEnum::APPPRICINGUNSPECIFIED),
           "FREE" => Ok(ApplicationAppPricingEnum::FREE),
           "FREE_WITH_IN_APP_PURCHASE" => Ok(ApplicationAppPricingEnum::FREEWITHINAPPPURCHASE),
           "PAID" => Ok(ApplicationAppPricingEnum::PAID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationAppPricingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationContentRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The content rating for this app.
pub enum ApplicationContentRatingEnum {
    

    /// Unspecified.
    ///
    /// "CONTENT_RATING_UNSPECIFIED"
    #[serde(rename="CONTENT_RATING_UNSPECIFIED")]
    CONTENTRATINGUNSPECIFIED,
    

    /// Content suitable for ages 3 and above only.
    ///
    /// "THREE_YEARS"
    #[serde(rename="THREE_YEARS")]
    THREEYEARS,
    

    /// Content suitable for ages 7 and above only.
    ///
    /// "SEVEN_YEARS"
    #[serde(rename="SEVEN_YEARS")]
    SEVENYEARS,
    

    /// Content suitable for ages 12 and above only.
    ///
    /// "TWELVE_YEARS"
    #[serde(rename="TWELVE_YEARS")]
    TWELVEYEARS,
    

    /// Content suitable for ages 16 and above only.
    ///
    /// "SIXTEEN_YEARS"
    #[serde(rename="SIXTEEN_YEARS")]
    SIXTEENYEARS,
    

    /// Content suitable for ages 18 and above only.
    ///
    /// "EIGHTEEN_YEARS"
    #[serde(rename="EIGHTEEN_YEARS")]
    EIGHTEENYEARS,
}

impl AsRef<str> for ApplicationContentRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationContentRatingEnum::CONTENTRATINGUNSPECIFIED => "CONTENT_RATING_UNSPECIFIED",
            ApplicationContentRatingEnum::THREEYEARS => "THREE_YEARS",
            ApplicationContentRatingEnum::SEVENYEARS => "SEVEN_YEARS",
            ApplicationContentRatingEnum::TWELVEYEARS => "TWELVE_YEARS",
            ApplicationContentRatingEnum::SIXTEENYEARS => "SIXTEEN_YEARS",
            ApplicationContentRatingEnum::EIGHTEENYEARS => "EIGHTEEN_YEARS",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationContentRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_RATING_UNSPECIFIED" => Ok(ApplicationContentRatingEnum::CONTENTRATINGUNSPECIFIED),
           "THREE_YEARS" => Ok(ApplicationContentRatingEnum::THREEYEARS),
           "SEVEN_YEARS" => Ok(ApplicationContentRatingEnum::SEVENYEARS),
           "TWELVE_YEARS" => Ok(ApplicationContentRatingEnum::TWELVEYEARS),
           "SIXTEEN_YEARS" => Ok(ApplicationContentRatingEnum::SIXTEENYEARS),
           "EIGHTEEN_YEARS" => Ok(ApplicationContentRatingEnum::EIGHTEENYEARS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationContentRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationDistributionChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How and to whom the package is made available.
pub enum ApplicationDistributionChannelEnum {
    

    /// Unspecified.
    ///
    /// "DISTRIBUTION_CHANNEL_UNSPECIFIED"
    #[serde(rename="DISTRIBUTION_CHANNEL_UNSPECIFIED")]
    DISTRIBUTIONCHANNELUNSPECIFIED,
    

    /// Package is available through the Play store and not restricted to a specific enterprise.
    ///
    /// "PUBLIC_GOOGLE_HOSTED"
    #[serde(rename="PUBLIC_GOOGLE_HOSTED")]
    PUBLICGOOGLEHOSTED,
    

    /// Package is a private app (restricted to an enterprise) but hosted by Google.
    ///
    /// "PRIVATE_GOOGLE_HOSTED"
    #[serde(rename="PRIVATE_GOOGLE_HOSTED")]
    PRIVATEGOOGLEHOSTED,
    

    /// Private app (restricted to an enterprise) and is privately hosted.
    ///
    /// "PRIVATE_SELF_HOSTED"
    #[serde(rename="PRIVATE_SELF_HOSTED")]
    PRIVATESELFHOSTED,
}

impl AsRef<str> for ApplicationDistributionChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationDistributionChannelEnum::DISTRIBUTIONCHANNELUNSPECIFIED => "DISTRIBUTION_CHANNEL_UNSPECIFIED",
            ApplicationDistributionChannelEnum::PUBLICGOOGLEHOSTED => "PUBLIC_GOOGLE_HOSTED",
            ApplicationDistributionChannelEnum::PRIVATEGOOGLEHOSTED => "PRIVATE_GOOGLE_HOSTED",
            ApplicationDistributionChannelEnum::PRIVATESELFHOSTED => "PRIVATE_SELF_HOSTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationDistributionChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISTRIBUTION_CHANNEL_UNSPECIFIED" => Ok(ApplicationDistributionChannelEnum::DISTRIBUTIONCHANNELUNSPECIFIED),
           "PUBLIC_GOOGLE_HOSTED" => Ok(ApplicationDistributionChannelEnum::PUBLICGOOGLEHOSTED),
           "PRIVATE_GOOGLE_HOSTED" => Ok(ApplicationDistributionChannelEnum::PRIVATEGOOGLEHOSTED),
           "PRIVATE_SELF_HOSTED" => Ok(ApplicationDistributionChannelEnum::PRIVATESELFHOSTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationDistributionChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Noteworthy features (if any) of this app.
pub enum ApplicationFeaturesEnum {
    

    /// Unspecified.
    ///
    /// "APP_FEATURE_UNSPECIFIED"
    #[serde(rename="APP_FEATURE_UNSPECIFIED")]
    APPFEATUREUNSPECIFIED,
    

    /// The app is a VPN.
    ///
    /// "VPN_APP"
    #[serde(rename="VPN_APP")]
    VPNAPP,
}

impl AsRef<str> for ApplicationFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationFeaturesEnum::APPFEATUREUNSPECIFIED => "APP_FEATURE_UNSPECIFIED",
            ApplicationFeaturesEnum::VPNAPP => "VPN_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_FEATURE_UNSPECIFIED" => Ok(ApplicationFeaturesEnum::APPFEATUREUNSPECIFIED),
           "VPN_APP" => Ok(ApplicationFeaturesEnum::VPNAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationEventEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// App event type.
pub enum ApplicationEventEventTypeEnum {
    

    /// This value is disallowed.
    ///
    /// "APPLICATION_EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="APPLICATION_EVENT_TYPE_UNSPECIFIED")]
    APPLICATIONEVENTTYPEUNSPECIFIED,
    

    /// The app was installed.
    ///
    /// "INSTALLED"
    #[serde(rename="INSTALLED")]
    INSTALLED,
    

    /// The app was changed, for example, a component was enabled or disabled.
    ///
    /// "CHANGED"
    #[serde(rename="CHANGED")]
    CHANGED,
    

    /// The app data was cleared.
    ///
    /// "DATA_CLEARED"
    #[serde(rename="DATA_CLEARED")]
    DATACLEARED,
    

    /// The app was removed.
    ///
    /// "REMOVED"
    #[serde(rename="REMOVED")]
    REMOVED,
    

    /// A new version of the app has been installed, replacing the old version.
    ///
    /// "REPLACED"
    #[serde(rename="REPLACED")]
    REPLACED,
    

    /// The app was restarted.
    ///
    /// "RESTARTED"
    #[serde(rename="RESTARTED")]
    RESTARTED,
    

    /// The app was pinned to the foreground.
    ///
    /// "PINNED"
    #[serde(rename="PINNED")]
    PINNED,
    

    /// The app was unpinned.
    ///
    /// "UNPINNED"
    #[serde(rename="UNPINNED")]
    UNPINNED,
}

impl AsRef<str> for ApplicationEventEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationEventEventTypeEnum::APPLICATIONEVENTTYPEUNSPECIFIED => "APPLICATION_EVENT_TYPE_UNSPECIFIED",
            ApplicationEventEventTypeEnum::INSTALLED => "INSTALLED",
            ApplicationEventEventTypeEnum::CHANGED => "CHANGED",
            ApplicationEventEventTypeEnum::DATACLEARED => "DATA_CLEARED",
            ApplicationEventEventTypeEnum::REMOVED => "REMOVED",
            ApplicationEventEventTypeEnum::REPLACED => "REPLACED",
            ApplicationEventEventTypeEnum::RESTARTED => "RESTARTED",
            ApplicationEventEventTypeEnum::PINNED => "PINNED",
            ApplicationEventEventTypeEnum::UNPINNED => "UNPINNED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationEventEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPLICATION_EVENT_TYPE_UNSPECIFIED" => Ok(ApplicationEventEventTypeEnum::APPLICATIONEVENTTYPEUNSPECIFIED),
           "INSTALLED" => Ok(ApplicationEventEventTypeEnum::INSTALLED),
           "CHANGED" => Ok(ApplicationEventEventTypeEnum::CHANGED),
           "DATA_CLEARED" => Ok(ApplicationEventEventTypeEnum::DATACLEARED),
           "REMOVED" => Ok(ApplicationEventEventTypeEnum::REMOVED),
           "REPLACED" => Ok(ApplicationEventEventTypeEnum::REPLACED),
           "RESTARTED" => Ok(ApplicationEventEventTypeEnum::RESTARTED),
           "PINNED" => Ok(ApplicationEventEventTypeEnum::PINNED),
           "UNPINNED" => Ok(ApplicationEventEventTypeEnum::UNPINNED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationEventEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the app is allowed networking when the VPN is not connected and alwaysOnVpnPackage.lockdownEnabled is enabled. If set to VPN_LOCKDOWN_ENFORCED, the app is not allowed networking, and if set to VPN_LOCKDOWN_EXEMPTION, the app is allowed networking. Only supported on devices running Android 10 and above. If this is not supported by the device, the device will contain a NonComplianceDetail with non_compliance_reason set to API_LEVEL and a fieldPath. If this is not applicable to the app, the device will contain a NonComplianceDetail with non_compliance_reason set to UNSUPPORTED and a fieldPath. The fieldPath is set to applications[i].alwaysOnVpnLockdownExemption, where i is the index of the package in the applications policy.
pub enum ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum {
    

    /// Unspecified. Defaults to VPN_LOCKDOWN_ENFORCED.
    ///
    /// "ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED"
    #[serde(rename="ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED")]
    ALWAYSONVPNLOCKDOWNEXEMPTIONUNSPECIFIED,
    

    /// The app respects the always-on VPN lockdown setting.
    ///
    /// "VPN_LOCKDOWN_ENFORCED"
    #[serde(rename="VPN_LOCKDOWN_ENFORCED")]
    VPNLOCKDOWNENFORCED,
    

    /// The app is exempt from the always-on VPN lockdown setting.
    ///
    /// "VPN_LOCKDOWN_EXEMPTION"
    #[serde(rename="VPN_LOCKDOWN_EXEMPTION")]
    VPNLOCKDOWNEXEMPTION,
}

impl AsRef<str> for ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum::ALWAYSONVPNLOCKDOWNEXEMPTIONUNSPECIFIED => "ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED",
            ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum::VPNLOCKDOWNENFORCED => "VPN_LOCKDOWN_ENFORCED",
            ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum::VPNLOCKDOWNEXEMPTION => "VPN_LOCKDOWN_EXEMPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALWAYS_ON_VPN_LOCKDOWN_EXEMPTION_UNSPECIFIED" => Ok(ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum::ALWAYSONVPNLOCKDOWNEXEMPTIONUNSPECIFIED),
           "VPN_LOCKDOWN_ENFORCED" => Ok(ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum::VPNLOCKDOWNENFORCED),
           "VPN_LOCKDOWN_EXEMPTION" => Ok(ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum::VPNLOCKDOWNEXEMPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyAlwaysOnVpnLockdownExemptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyAutoUpdateModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the auto-update mode for the app.
pub enum ApplicationPolicyAutoUpdateModeEnum {
    

    /// Unspecified. Defaults to AUTO_UPDATE_DEFAULT.
    ///
    /// "AUTO_UPDATE_MODE_UNSPECIFIED"
    #[serde(rename="AUTO_UPDATE_MODE_UNSPECIFIED")]
    AUTOUPDATEMODEUNSPECIFIED,
    

    /// The default update mode.The app is automatically updated with low priority to minimize the impact on the user.The app is updated when all of the following constraints are met: The device is not actively used. The device is connected to an unmetered network. The device is charging. The app to be updated is not running in the foreground.The device is notified about a new update within 24 hours after it is published by the developer, after which the app is updated the next time the constraints above are met.
    ///
    /// "AUTO_UPDATE_DEFAULT"
    #[serde(rename="AUTO_UPDATE_DEFAULT")]
    AUTOUPDATEDEFAULT,
    

    /// The app is not automatically updated for a maximum of 90 days after the app becomes out of date.90 days after the app becomes out of date, the latest available version is installed automatically with low priority (see AUTO_UPDATE_DEFAULT). After the app is updated it is not automatically updated again until 90 days after it becomes out of date again.The user can still manually update the app from the Play Store at any time.
    ///
    /// "AUTO_UPDATE_POSTPONED"
    #[serde(rename="AUTO_UPDATE_POSTPONED")]
    AUTOUPDATEPOSTPONED,
    

    /// The app is updated as soon as possible. No constraints are applied.The device is notified as soon as possible about a new update after it becomes available.
    ///
    /// "AUTO_UPDATE_HIGH_PRIORITY"
    #[serde(rename="AUTO_UPDATE_HIGH_PRIORITY")]
    AUTOUPDATEHIGHPRIORITY,
}

impl AsRef<str> for ApplicationPolicyAutoUpdateModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEMODEUNSPECIFIED => "AUTO_UPDATE_MODE_UNSPECIFIED",
            ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEDEFAULT => "AUTO_UPDATE_DEFAULT",
            ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEPOSTPONED => "AUTO_UPDATE_POSTPONED",
            ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEHIGHPRIORITY => "AUTO_UPDATE_HIGH_PRIORITY",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyAutoUpdateModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO_UPDATE_MODE_UNSPECIFIED" => Ok(ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEMODEUNSPECIFIED),
           "AUTO_UPDATE_DEFAULT" => Ok(ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEDEFAULT),
           "AUTO_UPDATE_POSTPONED" => Ok(ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEPOSTPONED),
           "AUTO_UPDATE_HIGH_PRIORITY" => Ok(ApplicationPolicyAutoUpdateModeEnum::AUTOUPDATEHIGHPRIORITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyAutoUpdateModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyConnectedWorkAndPersonalAppEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether the app can communicate with itself across a device’s work and personal profiles, subject to user consent.
pub enum ApplicationPolicyConnectedWorkAndPersonalAppEnum {
    

    /// Unspecified. Defaults to CONNECTED_WORK_AND_PERSONAL_APPS_DISALLOWED.
    ///
    /// "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED"
    #[serde(rename="CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED")]
    CONNECTEDWORKANDPERSONALAPPUNSPECIFIED,
    

    /// Default. Prevents the app from communicating cross-profile.
    ///
    /// "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED"
    #[serde(rename="CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED")]
    CONNECTEDWORKANDPERSONALAPPDISALLOWED,
    

    /// Allows the app to communicate across profiles after receiving user consent.
    ///
    /// "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED"
    #[serde(rename="CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED")]
    CONNECTEDWORKANDPERSONALAPPALLOWED,
}

impl AsRef<str> for ApplicationPolicyConnectedWorkAndPersonalAppEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyConnectedWorkAndPersonalAppEnum::CONNECTEDWORKANDPERSONALAPPUNSPECIFIED => "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED",
            ApplicationPolicyConnectedWorkAndPersonalAppEnum::CONNECTEDWORKANDPERSONALAPPDISALLOWED => "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED",
            ApplicationPolicyConnectedWorkAndPersonalAppEnum::CONNECTEDWORKANDPERSONALAPPALLOWED => "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyConnectedWorkAndPersonalAppEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTED_WORK_AND_PERSONAL_APP_UNSPECIFIED" => Ok(ApplicationPolicyConnectedWorkAndPersonalAppEnum::CONNECTEDWORKANDPERSONALAPPUNSPECIFIED),
           "CONNECTED_WORK_AND_PERSONAL_APP_DISALLOWED" => Ok(ApplicationPolicyConnectedWorkAndPersonalAppEnum::CONNECTEDWORKANDPERSONALAPPDISALLOWED),
           "CONNECTED_WORK_AND_PERSONAL_APP_ALLOWED" => Ok(ApplicationPolicyConnectedWorkAndPersonalAppEnum::CONNECTEDWORKANDPERSONALAPPALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyConnectedWorkAndPersonalAppEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyCredentialProviderPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether the app is allowed to act as a credential provider on Android 14 and above.
pub enum ApplicationPolicyCredentialProviderPolicyEnum {
    

    /// Unspecified. The behaviour is governed by credentialProviderPolicyDefault.
    ///
    /// "CREDENTIAL_PROVIDER_POLICY_UNSPECIFIED"
    #[serde(rename="CREDENTIAL_PROVIDER_POLICY_UNSPECIFIED")]
    CREDENTIALPROVIDERPOLICYUNSPECIFIED,
    

    /// App is allowed to act as a credential provider.
    ///
    /// "CREDENTIAL_PROVIDER_ALLOWED"
    #[serde(rename="CREDENTIAL_PROVIDER_ALLOWED")]
    CREDENTIALPROVIDERALLOWED,
}

impl AsRef<str> for ApplicationPolicyCredentialProviderPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyCredentialProviderPolicyEnum::CREDENTIALPROVIDERPOLICYUNSPECIFIED => "CREDENTIAL_PROVIDER_POLICY_UNSPECIFIED",
            ApplicationPolicyCredentialProviderPolicyEnum::CREDENTIALPROVIDERALLOWED => "CREDENTIAL_PROVIDER_ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyCredentialProviderPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREDENTIAL_PROVIDER_POLICY_UNSPECIFIED" => Ok(ApplicationPolicyCredentialProviderPolicyEnum::CREDENTIALPROVIDERPOLICYUNSPECIFIED),
           "CREDENTIAL_PROVIDER_ALLOWED" => Ok(ApplicationPolicyCredentialProviderPolicyEnum::CREDENTIALPROVIDERALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyCredentialProviderPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyDefaultPermissionPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The default policy for all permissions requested by the app. If specified, this overrides the policy-level default_permission_policy which applies to all apps. It does not override the permission_grants which applies to all apps.
pub enum ApplicationPolicyDefaultPermissionPolicyEnum {
    

    /// Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default.
    ///
    /// "PERMISSION_POLICY_UNSPECIFIED"
    #[serde(rename="PERMISSION_POLICY_UNSPECIFIED")]
    PERMISSIONPOLICYUNSPECIFIED,
    

    /// Prompt the user to grant a permission.
    ///
    /// "PROMPT"
    #[serde(rename="PROMPT")]
    PROMPT,
    

    /// Automatically grant a permission.On Android 12 and above, Manifest.permission.READ_SMS (https://developer.android.com/reference/android/Manifest.permission#READ_SMS) and following sensor-related permissions can only be granted on fully managed devices: Manifest.permission.ACCESS_FINE_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_FINE_LOCATION) Manifest.permission.ACCESS_BACKGROUND_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_BACKGROUND_LOCATION) Manifest.permission.ACCESS_COARSE_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_COARSE_LOCATION) Manifest.permission.CAMERA (https://developer.android.com/reference/android/Manifest.permission#CAMERA) Manifest.permission.RECORD_AUDIO (https://developer.android.com/reference/android/Manifest.permission#RECORD_AUDIO) Manifest.permission.ACTIVITY_RECOGNITION (https://developer.android.com/reference/android/Manifest.permission#ACTIVITY_RECOGNITION) Manifest.permission.BODY_SENSORS (https://developer.android.com/reference/android/Manifest.permission#BODY_SENSORS)
    ///
    /// "GRANT"
    #[serde(rename="GRANT")]
    GRANT,
    

    /// Automatically deny a permission.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for ApplicationPolicyDefaultPermissionPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyDefaultPermissionPolicyEnum::PERMISSIONPOLICYUNSPECIFIED => "PERMISSION_POLICY_UNSPECIFIED",
            ApplicationPolicyDefaultPermissionPolicyEnum::PROMPT => "PROMPT",
            ApplicationPolicyDefaultPermissionPolicyEnum::GRANT => "GRANT",
            ApplicationPolicyDefaultPermissionPolicyEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyDefaultPermissionPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_POLICY_UNSPECIFIED" => Ok(ApplicationPolicyDefaultPermissionPolicyEnum::PERMISSIONPOLICYUNSPECIFIED),
           "PROMPT" => Ok(ApplicationPolicyDefaultPermissionPolicyEnum::PROMPT),
           "GRANT" => Ok(ApplicationPolicyDefaultPermissionPolicyEnum::GRANT),
           "DENY" => Ok(ApplicationPolicyDefaultPermissionPolicyEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyDefaultPermissionPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyDelegatedScopesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scopes delegated to the app from Android Device Policy. These provide additional privileges for the applications they are applied to.
pub enum ApplicationPolicyDelegatedScopesEnum {
    

    /// No delegation scope specified.
    ///
    /// "DELEGATED_SCOPE_UNSPECIFIED"
    #[serde(rename="DELEGATED_SCOPE_UNSPECIFIED")]
    DELEGATEDSCOPEUNSPECIFIED,
    

    /// Grants access to certificate installation and management.
    ///
    /// "CERT_INSTALL"
    #[serde(rename="CERT_INSTALL")]
    CERTINSTALL,
    

    /// Grants access to managed configurations management.
    ///
    /// "MANAGED_CONFIGURATIONS"
    #[serde(rename="MANAGED_CONFIGURATIONS")]
    MANAGEDCONFIGURATIONS,
    

    /// Grants access to blocking uninstallation.
    ///
    /// "BLOCK_UNINSTALL"
    #[serde(rename="BLOCK_UNINSTALL")]
    BLOCKUNINSTALL,
    

    /// Grants access to permission policy and permission grant state.
    ///
    /// "PERMISSION_GRANT"
    #[serde(rename="PERMISSION_GRANT")]
    PERMISSIONGRANT,
    

    /// Grants access to package access state.
    ///
    /// "PACKAGE_ACCESS"
    #[serde(rename="PACKAGE_ACCESS")]
    PACKAGEACCESS,
    

    /// Grants access for enabling system apps.
    ///
    /// "ENABLE_SYSTEM_APP"
    #[serde(rename="ENABLE_SYSTEM_APP")]
    ENABLESYSTEMAPP,
    

    /// Grants access to network activity logs. Allows the delegated application to call setNetworkLoggingEnabled (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#setNetworkLoggingEnabled%28android.content.ComponentName,%20boolean%29), isNetworkLoggingEnabled (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#isNetworkLoggingEnabled%28android.content.ComponentName%29) and retrieveNetworkLogs (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#retrieveNetworkLogs%28android.content.ComponentName,%20long%29) methods. This scope can be delegated to at most one application. Supported for fully managed devices on Android 10 and above. Supported for a work profile on Android 12 and above. When delegation is supported and set, NETWORK_ACTIVITY_LOGS is ignored.
    ///
    /// "NETWORK_ACTIVITY_LOGS"
    #[serde(rename="NETWORK_ACTIVITY_LOGS")]
    NETWORKACTIVITYLOGS,
    

    /// Grants access to security logs. Allows the delegated application to call setSecurityLoggingEnabled (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#setSecurityLoggingEnabled%28android.content.ComponentName,%20boolean%29), isSecurityLoggingEnabled (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#isSecurityLoggingEnabled%28android.content.ComponentName%29), retrieveSecurityLogs (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#retrieveSecurityLogs%28android.content.ComponentName%29) and retrievePreRebootSecurityLogs (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#retrievePreRebootSecurityLogs%28android.content.ComponentName%29) methods. This scope can be delegated to at most one application. Supported for fully managed devices and company-owned devices with a work profile on Android 12 and above. When delegation is supported and set, SECURITY_LOGS is ignored.
    ///
    /// "SECURITY_LOGS"
    #[serde(rename="SECURITY_LOGS")]
    SECURITYLOGS,
    

    /// Grants access to selection of KeyChain certificates on behalf of requesting apps. Once granted, the delegated application will start receiving DelegatedAdminReceiver#onChoosePrivateKeyAlias (https://developer.android.com/reference/android/app/admin/DelegatedAdminReceiver#onChoosePrivateKeyAlias%28android.content.Context,%20android.content.Intent,%20int,%20android.net.Uri,%20java.lang.String%29). Allows the delegated application to call grantKeyPairToApp (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#grantKeyPairToApp%28android.content.ComponentName,%20java.lang.String,%20java.lang.String%29) and revokeKeyPairFromApp (https://developer.android.com/reference/android/app/admin/DevicePolicyManager#revokeKeyPairFromApp%28android.content.ComponentName,%20java.lang.String,%20java.lang.String%29) methods. There can be at most one app that has this delegation. choosePrivateKeyRules must be empty and privateKeySelectionEnabled has no effect if certificate selection is delegated to an application.
    ///
    /// "CERT_SELECTION"
    #[serde(rename="CERT_SELECTION")]
    CERTSELECTION,
}

impl AsRef<str> for ApplicationPolicyDelegatedScopesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyDelegatedScopesEnum::DELEGATEDSCOPEUNSPECIFIED => "DELEGATED_SCOPE_UNSPECIFIED",
            ApplicationPolicyDelegatedScopesEnum::CERTINSTALL => "CERT_INSTALL",
            ApplicationPolicyDelegatedScopesEnum::MANAGEDCONFIGURATIONS => "MANAGED_CONFIGURATIONS",
            ApplicationPolicyDelegatedScopesEnum::BLOCKUNINSTALL => "BLOCK_UNINSTALL",
            ApplicationPolicyDelegatedScopesEnum::PERMISSIONGRANT => "PERMISSION_GRANT",
            ApplicationPolicyDelegatedScopesEnum::PACKAGEACCESS => "PACKAGE_ACCESS",
            ApplicationPolicyDelegatedScopesEnum::ENABLESYSTEMAPP => "ENABLE_SYSTEM_APP",
            ApplicationPolicyDelegatedScopesEnum::NETWORKACTIVITYLOGS => "NETWORK_ACTIVITY_LOGS",
            ApplicationPolicyDelegatedScopesEnum::SECURITYLOGS => "SECURITY_LOGS",
            ApplicationPolicyDelegatedScopesEnum::CERTSELECTION => "CERT_SELECTION",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyDelegatedScopesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELEGATED_SCOPE_UNSPECIFIED" => Ok(ApplicationPolicyDelegatedScopesEnum::DELEGATEDSCOPEUNSPECIFIED),
           "CERT_INSTALL" => Ok(ApplicationPolicyDelegatedScopesEnum::CERTINSTALL),
           "MANAGED_CONFIGURATIONS" => Ok(ApplicationPolicyDelegatedScopesEnum::MANAGEDCONFIGURATIONS),
           "BLOCK_UNINSTALL" => Ok(ApplicationPolicyDelegatedScopesEnum::BLOCKUNINSTALL),
           "PERMISSION_GRANT" => Ok(ApplicationPolicyDelegatedScopesEnum::PERMISSIONGRANT),
           "PACKAGE_ACCESS" => Ok(ApplicationPolicyDelegatedScopesEnum::PACKAGEACCESS),
           "ENABLE_SYSTEM_APP" => Ok(ApplicationPolicyDelegatedScopesEnum::ENABLESYSTEMAPP),
           "NETWORK_ACTIVITY_LOGS" => Ok(ApplicationPolicyDelegatedScopesEnum::NETWORKACTIVITYLOGS),
           "SECURITY_LOGS" => Ok(ApplicationPolicyDelegatedScopesEnum::SECURITYLOGS),
           "CERT_SELECTION" => Ok(ApplicationPolicyDelegatedScopesEnum::CERTSELECTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyDelegatedScopesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyInstallTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of installation to perform.
pub enum ApplicationPolicyInstallTypeEnum {
    

    /// Unspecified. Defaults to AVAILABLE.
    ///
    /// "INSTALL_TYPE_UNSPECIFIED"
    #[serde(rename="INSTALL_TYPE_UNSPECIFIED")]
    INSTALLTYPEUNSPECIFIED,
    

    /// The app is automatically installed and can be removed by the user.
    ///
    /// "PREINSTALLED"
    #[serde(rename="PREINSTALLED")]
    PREINSTALLED,
    

    /// The app is automatically installed regardless of a set maintenance window and can't be removed by the user.
    ///
    /// "FORCE_INSTALLED"
    #[serde(rename="FORCE_INSTALLED")]
    FORCEINSTALLED,
    

    /// The app is blocked and can't be installed. If the app was installed under a previous policy, it will be uninstalled. This also blocks its instant app functionality.
    ///
    /// "BLOCKED"
    #[serde(rename="BLOCKED")]
    BLOCKED,
    

    /// The app is available to install.
    ///
    /// "AVAILABLE"
    #[serde(rename="AVAILABLE")]
    AVAILABLE,
    

    /// The app is automatically installed and can't be removed by the user and will prevent setup from completion until installation is complete.
    ///
    /// "REQUIRED_FOR_SETUP"
    #[serde(rename="REQUIRED_FOR_SETUP")]
    REQUIREDFORSETUP,
    

    /// The app is automatically installed in kiosk mode: it's set as the preferred home intent and whitelisted for lock task mode. Device setup won't complete until the app is installed. After installation, users won't be able to remove the app. You can only set this installType for one app per policy. When this is present in the policy, status bar will be automatically disabled.
    ///
    /// "KIOSK"
    #[serde(rename="KIOSK")]
    KIOSK,
}

impl AsRef<str> for ApplicationPolicyInstallTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyInstallTypeEnum::INSTALLTYPEUNSPECIFIED => "INSTALL_TYPE_UNSPECIFIED",
            ApplicationPolicyInstallTypeEnum::PREINSTALLED => "PREINSTALLED",
            ApplicationPolicyInstallTypeEnum::FORCEINSTALLED => "FORCE_INSTALLED",
            ApplicationPolicyInstallTypeEnum::BLOCKED => "BLOCKED",
            ApplicationPolicyInstallTypeEnum::AVAILABLE => "AVAILABLE",
            ApplicationPolicyInstallTypeEnum::REQUIREDFORSETUP => "REQUIRED_FOR_SETUP",
            ApplicationPolicyInstallTypeEnum::KIOSK => "KIOSK",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyInstallTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTALL_TYPE_UNSPECIFIED" => Ok(ApplicationPolicyInstallTypeEnum::INSTALLTYPEUNSPECIFIED),
           "PREINSTALLED" => Ok(ApplicationPolicyInstallTypeEnum::PREINSTALLED),
           "FORCE_INSTALLED" => Ok(ApplicationPolicyInstallTypeEnum::FORCEINSTALLED),
           "BLOCKED" => Ok(ApplicationPolicyInstallTypeEnum::BLOCKED),
           "AVAILABLE" => Ok(ApplicationPolicyInstallTypeEnum::AVAILABLE),
           "REQUIRED_FOR_SETUP" => Ok(ApplicationPolicyInstallTypeEnum::REQUIREDFORSETUP),
           "KIOSK" => Ok(ApplicationPolicyInstallTypeEnum::KIOSK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyInstallTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPolicyWorkProfileWidgetsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the app installed in the work profile is allowed to add widgets to the home screen.
pub enum ApplicationPolicyWorkProfileWidgetsEnum {
    

    /// Unspecified. Defaults to work_profile_widgets_default
    ///
    /// "WORK_PROFILE_WIDGETS_UNSPECIFIED"
    #[serde(rename="WORK_PROFILE_WIDGETS_UNSPECIFIED")]
    WORKPROFILEWIDGETSUNSPECIFIED,
    

    /// Work profile widgets are allowed. This means the application will be able to add widgets to the home screen.
    ///
    /// "WORK_PROFILE_WIDGETS_ALLOWED"
    #[serde(rename="WORK_PROFILE_WIDGETS_ALLOWED")]
    WORKPROFILEWIDGETSALLOWED,
    

    /// Work profile widgets are disallowed. This means the application will not be able to add widgets to the home screen.
    ///
    /// "WORK_PROFILE_WIDGETS_DISALLOWED"
    #[serde(rename="WORK_PROFILE_WIDGETS_DISALLOWED")]
    WORKPROFILEWIDGETSDISALLOWED,
}

impl AsRef<str> for ApplicationPolicyWorkProfileWidgetsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPolicyWorkProfileWidgetsEnum::WORKPROFILEWIDGETSUNSPECIFIED => "WORK_PROFILE_WIDGETS_UNSPECIFIED",
            ApplicationPolicyWorkProfileWidgetsEnum::WORKPROFILEWIDGETSALLOWED => "WORK_PROFILE_WIDGETS_ALLOWED",
            ApplicationPolicyWorkProfileWidgetsEnum::WORKPROFILEWIDGETSDISALLOWED => "WORK_PROFILE_WIDGETS_DISALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPolicyWorkProfileWidgetsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WORK_PROFILE_WIDGETS_UNSPECIFIED" => Ok(ApplicationPolicyWorkProfileWidgetsEnum::WORKPROFILEWIDGETSUNSPECIFIED),
           "WORK_PROFILE_WIDGETS_ALLOWED" => Ok(ApplicationPolicyWorkProfileWidgetsEnum::WORKPROFILEWIDGETSALLOWED),
           "WORK_PROFILE_WIDGETS_DISALLOWED" => Ok(ApplicationPolicyWorkProfileWidgetsEnum::WORKPROFILEWIDGETSDISALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPolicyWorkProfileWidgetsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationReportApplicationSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The source of the package.
pub enum ApplicationReportApplicationSourceEnum {
    

    /// The app was sideloaded from an unspecified source.
    ///
    /// "APPLICATION_SOURCE_UNSPECIFIED"
    #[serde(rename="APPLICATION_SOURCE_UNSPECIFIED")]
    APPLICATIONSOURCEUNSPECIFIED,
    

    /// This is a system app from the device's factory image.
    ///
    /// "SYSTEM_APP_FACTORY_VERSION"
    #[serde(rename="SYSTEM_APP_FACTORY_VERSION")]
    SYSTEMAPPFACTORYVERSION,
    

    /// This is an updated system app.
    ///
    /// "SYSTEM_APP_UPDATED_VERSION"
    #[serde(rename="SYSTEM_APP_UPDATED_VERSION")]
    SYSTEMAPPUPDATEDVERSION,
    

    /// The app was installed from the Google Play Store.
    ///
    /// "INSTALLED_FROM_PLAY_STORE"
    #[serde(rename="INSTALLED_FROM_PLAY_STORE")]
    INSTALLEDFROMPLAYSTORE,
}

impl AsRef<str> for ApplicationReportApplicationSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationReportApplicationSourceEnum::APPLICATIONSOURCEUNSPECIFIED => "APPLICATION_SOURCE_UNSPECIFIED",
            ApplicationReportApplicationSourceEnum::SYSTEMAPPFACTORYVERSION => "SYSTEM_APP_FACTORY_VERSION",
            ApplicationReportApplicationSourceEnum::SYSTEMAPPUPDATEDVERSION => "SYSTEM_APP_UPDATED_VERSION",
            ApplicationReportApplicationSourceEnum::INSTALLEDFROMPLAYSTORE => "INSTALLED_FROM_PLAY_STORE",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationReportApplicationSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPLICATION_SOURCE_UNSPECIFIED" => Ok(ApplicationReportApplicationSourceEnum::APPLICATIONSOURCEUNSPECIFIED),
           "SYSTEM_APP_FACTORY_VERSION" => Ok(ApplicationReportApplicationSourceEnum::SYSTEMAPPFACTORYVERSION),
           "SYSTEM_APP_UPDATED_VERSION" => Ok(ApplicationReportApplicationSourceEnum::SYSTEMAPPUPDATEDVERSION),
           "INSTALLED_FROM_PLAY_STORE" => Ok(ApplicationReportApplicationSourceEnum::INSTALLEDFROMPLAYSTORE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationReportApplicationSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationReportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Application state.
pub enum ApplicationReportStateEnum {
    

    /// App state is unspecified
    ///
    /// "APPLICATION_STATE_UNSPECIFIED"
    #[serde(rename="APPLICATION_STATE_UNSPECIFIED")]
    APPLICATIONSTATEUNSPECIFIED,
    

    /// App was removed from the device
    ///
    /// "REMOVED"
    #[serde(rename="REMOVED")]
    REMOVED,
    

    /// App is installed on the device
    ///
    /// "INSTALLED"
    #[serde(rename="INSTALLED")]
    INSTALLED,
}

impl AsRef<str> for ApplicationReportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationReportStateEnum::APPLICATIONSTATEUNSPECIFIED => "APPLICATION_STATE_UNSPECIFIED",
            ApplicationReportStateEnum::REMOVED => "REMOVED",
            ApplicationReportStateEnum::INSTALLED => "INSTALLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationReportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPLICATION_STATE_UNSPECIFIED" => Ok(ApplicationReportStateEnum::APPLICATIONSTATEUNSPECIFIED),
           "REMOVED" => Ok(ApplicationReportStateEnum::REMOVED),
           "INSTALLED" => Ok(ApplicationReportStateEnum::INSTALLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationReportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationReportUserFacingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the app is user facing.
pub enum ApplicationReportUserFacingTypeEnum {
    

    /// App user facing type is unspecified.
    ///
    /// "USER_FACING_TYPE_UNSPECIFIED"
    #[serde(rename="USER_FACING_TYPE_UNSPECIFIED")]
    USERFACINGTYPEUNSPECIFIED,
    

    /// App is not user facing.
    ///
    /// "NOT_USER_FACING"
    #[serde(rename="NOT_USER_FACING")]
    NOTUSERFACING,
    

    /// App is user facing.
    ///
    /// "USER_FACING"
    #[serde(rename="USER_FACING")]
    USERFACING,
}

impl AsRef<str> for ApplicationReportUserFacingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationReportUserFacingTypeEnum::USERFACINGTYPEUNSPECIFIED => "USER_FACING_TYPE_UNSPECIFIED",
            ApplicationReportUserFacingTypeEnum::NOTUSERFACING => "NOT_USER_FACING",
            ApplicationReportUserFacingTypeEnum::USERFACING => "USER_FACING",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationReportUserFacingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_FACING_TYPE_UNSPECIFIED" => Ok(ApplicationReportUserFacingTypeEnum::USERFACINGTYPEUNSPECIFIED),
           "NOT_USER_FACING" => Ok(ApplicationReportUserFacingTypeEnum::NOTUSERFACING),
           "USER_FACING" => Ok(ApplicationReportUserFacingTypeEnum::USERFACING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationReportUserFacingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlockActionBlockScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the scope of this BlockAction. Only applicable to devices that are company-owned.
pub enum BlockActionBlockScopeEnum {
    

    /// Unspecified. Defaults to BLOCK_SCOPE_WORK_PROFILE.
    ///
    /// "BLOCK_SCOPE_UNSPECIFIED"
    #[serde(rename="BLOCK_SCOPE_UNSPECIFIED")]
    BLOCKSCOPEUNSPECIFIED,
    

    /// Block action is only applied to apps in the work profile. Apps in the personal profile are unaffected.
    ///
    /// "BLOCK_SCOPE_WORK_PROFILE"
    #[serde(rename="BLOCK_SCOPE_WORK_PROFILE")]
    BLOCKSCOPEWORKPROFILE,
    

    /// Block action is applied to the entire device, including apps in the personal profile.
    ///
    /// "BLOCK_SCOPE_DEVICE"
    #[serde(rename="BLOCK_SCOPE_DEVICE")]
    BLOCKSCOPEDEVICE,
}

impl AsRef<str> for BlockActionBlockScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlockActionBlockScopeEnum::BLOCKSCOPEUNSPECIFIED => "BLOCK_SCOPE_UNSPECIFIED",
            BlockActionBlockScopeEnum::BLOCKSCOPEWORKPROFILE => "BLOCK_SCOPE_WORK_PROFILE",
            BlockActionBlockScopeEnum::BLOCKSCOPEDEVICE => "BLOCK_SCOPE_DEVICE",
        }
    }
}

impl std::convert::TryFrom< &str> for BlockActionBlockScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BLOCK_SCOPE_UNSPECIFIED" => Ok(BlockActionBlockScopeEnum::BLOCKSCOPEUNSPECIFIED),
           "BLOCK_SCOPE_WORK_PROFILE" => Ok(BlockActionBlockScopeEnum::BLOCKSCOPEWORKPROFILE),
           "BLOCK_SCOPE_DEVICE" => Ok(BlockActionBlockScopeEnum::BLOCKSCOPEDEVICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlockActionBlockScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommandErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If the command failed, an error code explaining the failure. This is not set when the command is cancelled by the caller.
pub enum CommandErrorCodeEnum {
    

    /// There was no error.
    ///
    /// "COMMAND_ERROR_CODE_UNSPECIFIED"
    #[serde(rename="COMMAND_ERROR_CODE_UNSPECIFIED")]
    COMMANDERRORCODEUNSPECIFIED,
    

    /// An unknown error occurred.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The API level of the device does not support this command.
    ///
    /// "API_LEVEL"
    #[serde(rename="API_LEVEL")]
    APILEVEL,
    

    /// The management mode (profile owner, device owner, etc.) does not support the command.
    ///
    /// "MANAGEMENT_MODE"
    #[serde(rename="MANAGEMENT_MODE")]
    MANAGEMENTMODE,
    

    /// The command has an invalid parameter value.
    ///
    /// "INVALID_VALUE"
    #[serde(rename="INVALID_VALUE")]
    INVALIDVALUE,
    

    /// The device doesn't support the command. Updating Android Device Policy to the latest version may resolve the issue.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
}

impl AsRef<str> for CommandErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommandErrorCodeEnum::COMMANDERRORCODEUNSPECIFIED => "COMMAND_ERROR_CODE_UNSPECIFIED",
            CommandErrorCodeEnum::UNKNOWN => "UNKNOWN",
            CommandErrorCodeEnum::APILEVEL => "API_LEVEL",
            CommandErrorCodeEnum::MANAGEMENTMODE => "MANAGEMENT_MODE",
            CommandErrorCodeEnum::INVALIDVALUE => "INVALID_VALUE",
            CommandErrorCodeEnum::UNSUPPORTED => "UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for CommandErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMAND_ERROR_CODE_UNSPECIFIED" => Ok(CommandErrorCodeEnum::COMMANDERRORCODEUNSPECIFIED),
           "UNKNOWN" => Ok(CommandErrorCodeEnum::UNKNOWN),
           "API_LEVEL" => Ok(CommandErrorCodeEnum::APILEVEL),
           "MANAGEMENT_MODE" => Ok(CommandErrorCodeEnum::MANAGEMENTMODE),
           "INVALID_VALUE" => Ok(CommandErrorCodeEnum::INVALIDVALUE),
           "UNSUPPORTED" => Ok(CommandErrorCodeEnum::UNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommandErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommandResetPasswordFlagsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// For commands of type RESET_PASSWORD, optionally specifies flags.
pub enum CommandResetPasswordFlagsEnum {
    

    /// This value is ignored.
    ///
    /// "RESET_PASSWORD_FLAG_UNSPECIFIED"
    #[serde(rename="RESET_PASSWORD_FLAG_UNSPECIFIED")]
    RESETPASSWORDFLAGUNSPECIFIED,
    

    /// Don't allow other admins to change the password again until the user has entered it.
    ///
    /// "REQUIRE_ENTRY"
    #[serde(rename="REQUIRE_ENTRY")]
    REQUIREENTRY,
    

    /// Don't ask for user credentials on device boot.
    ///
    /// "DO_NOT_ASK_CREDENTIALS_ON_BOOT"
    #[serde(rename="DO_NOT_ASK_CREDENTIALS_ON_BOOT")]
    DONOTASKCREDENTIALSONBOOT,
    

    /// Lock the device after password reset.
    ///
    /// "LOCK_NOW"
    #[serde(rename="LOCK_NOW")]
    LOCKNOW,
}

impl AsRef<str> for CommandResetPasswordFlagsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommandResetPasswordFlagsEnum::RESETPASSWORDFLAGUNSPECIFIED => "RESET_PASSWORD_FLAG_UNSPECIFIED",
            CommandResetPasswordFlagsEnum::REQUIREENTRY => "REQUIRE_ENTRY",
            CommandResetPasswordFlagsEnum::DONOTASKCREDENTIALSONBOOT => "DO_NOT_ASK_CREDENTIALS_ON_BOOT",
            CommandResetPasswordFlagsEnum::LOCKNOW => "LOCK_NOW",
        }
    }
}

impl std::convert::TryFrom< &str> for CommandResetPasswordFlagsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESET_PASSWORD_FLAG_UNSPECIFIED" => Ok(CommandResetPasswordFlagsEnum::RESETPASSWORDFLAGUNSPECIFIED),
           "REQUIRE_ENTRY" => Ok(CommandResetPasswordFlagsEnum::REQUIREENTRY),
           "DO_NOT_ASK_CREDENTIALS_ON_BOOT" => Ok(CommandResetPasswordFlagsEnum::DONOTASKCREDENTIALSONBOOT),
           "LOCK_NOW" => Ok(CommandResetPasswordFlagsEnum::LOCKNOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommandResetPasswordFlagsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommandTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the command.
pub enum CommandTypeEnum {
    

    /// This value is disallowed.
    ///
    /// "COMMAND_TYPE_UNSPECIFIED"
    #[serde(rename="COMMAND_TYPE_UNSPECIFIED")]
    COMMANDTYPEUNSPECIFIED,
    

    /// Lock the device, as if the lock screen timeout had expired.
    ///
    /// "LOCK"
    #[serde(rename="LOCK")]
    LOCK,
    

    /// Reset the user's password.
    ///
    /// "RESET_PASSWORD"
    #[serde(rename="RESET_PASSWORD")]
    RESETPASSWORD,
    

    /// Reboot the device. Only supported on fully managed devices running Android 7.0 (API level 24) or higher.
    ///
    /// "REBOOT"
    #[serde(rename="REBOOT")]
    REBOOT,
    

    /// Removes the work profile and all policies from a company-owned Android 8.0+ device, relinquishing the device for personal use. Apps and data associated with the personal profile(s) are preserved. The device will be deleted from the server after it acknowledges the command.
    ///
    /// "RELINQUISH_OWNERSHIP"
    #[serde(rename="RELINQUISH_OWNERSHIP")]
    RELINQUISHOWNERSHIP,
    

    /// Clears the application data of specified apps. This is supported on Android 9 and above. Note that an application can store data outside of its application data, for example in external storage or in a user dictionary. See also clear_apps_data_params.
    ///
    /// "CLEAR_APP_DATA"
    #[serde(rename="CLEAR_APP_DATA")]
    CLEARAPPDATA,
    

    /// Puts the device into lost mode. Only supported on fully managed devices or organization-owned devices with a managed profile. See also start_lost_mode_params.
    ///
    /// "START_LOST_MODE"
    #[serde(rename="START_LOST_MODE")]
    STARTLOSTMODE,
    

    /// Takes the device out of lost mode. Only supported on fully managed devices or organization-owned devices with a managed profile. See also stop_lost_mode_params.
    ///
    /// "STOP_LOST_MODE"
    #[serde(rename="STOP_LOST_MODE")]
    STOPLOSTMODE,
}

impl AsRef<str> for CommandTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommandTypeEnum::COMMANDTYPEUNSPECIFIED => "COMMAND_TYPE_UNSPECIFIED",
            CommandTypeEnum::LOCK => "LOCK",
            CommandTypeEnum::RESETPASSWORD => "RESET_PASSWORD",
            CommandTypeEnum::REBOOT => "REBOOT",
            CommandTypeEnum::RELINQUISHOWNERSHIP => "RELINQUISH_OWNERSHIP",
            CommandTypeEnum::CLEARAPPDATA => "CLEAR_APP_DATA",
            CommandTypeEnum::STARTLOSTMODE => "START_LOST_MODE",
            CommandTypeEnum::STOPLOSTMODE => "STOP_LOST_MODE",
        }
    }
}

impl std::convert::TryFrom< &str> for CommandTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMAND_TYPE_UNSPECIFIED" => Ok(CommandTypeEnum::COMMANDTYPEUNSPECIFIED),
           "LOCK" => Ok(CommandTypeEnum::LOCK),
           "RESET_PASSWORD" => Ok(CommandTypeEnum::RESETPASSWORD),
           "REBOOT" => Ok(CommandTypeEnum::REBOOT),
           "RELINQUISH_OWNERSHIP" => Ok(CommandTypeEnum::RELINQUISHOWNERSHIP),
           "CLEAR_APP_DATA" => Ok(CommandTypeEnum::CLEARAPPDATA),
           "START_LOST_MODE" => Ok(CommandTypeEnum::STARTLOSTMODE),
           "STOP_LOST_MODE" => Ok(CommandTypeEnum::STOPLOSTMODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommandTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommonCriteriaModeInfoCommonCriteriaModeStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether Common Criteria Mode is enabled.
pub enum CommonCriteriaModeInfoCommonCriteriaModeStatusEnum {
    

    /// Unknown status.
    ///
    /// "COMMON_CRITERIA_MODE_STATUS_UNKNOWN"
    #[serde(rename="COMMON_CRITERIA_MODE_STATUS_UNKNOWN")]
    COMMONCRITERIAMODESTATUSUNKNOWN,
    

    /// Common Criteria Mode is currently disabled.
    ///
    /// "COMMON_CRITERIA_MODE_DISABLED"
    #[serde(rename="COMMON_CRITERIA_MODE_DISABLED")]
    COMMONCRITERIAMODEDISABLED,
    

    /// Common Criteria Mode is currently enabled.
    ///
    /// "COMMON_CRITERIA_MODE_ENABLED"
    #[serde(rename="COMMON_CRITERIA_MODE_ENABLED")]
    COMMONCRITERIAMODEENABLED,
}

impl AsRef<str> for CommonCriteriaModeInfoCommonCriteriaModeStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommonCriteriaModeInfoCommonCriteriaModeStatusEnum::COMMONCRITERIAMODESTATUSUNKNOWN => "COMMON_CRITERIA_MODE_STATUS_UNKNOWN",
            CommonCriteriaModeInfoCommonCriteriaModeStatusEnum::COMMONCRITERIAMODEDISABLED => "COMMON_CRITERIA_MODE_DISABLED",
            CommonCriteriaModeInfoCommonCriteriaModeStatusEnum::COMMONCRITERIAMODEENABLED => "COMMON_CRITERIA_MODE_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for CommonCriteriaModeInfoCommonCriteriaModeStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMON_CRITERIA_MODE_STATUS_UNKNOWN" => Ok(CommonCriteriaModeInfoCommonCriteriaModeStatusEnum::COMMONCRITERIAMODESTATUSUNKNOWN),
           "COMMON_CRITERIA_MODE_DISABLED" => Ok(CommonCriteriaModeInfoCommonCriteriaModeStatusEnum::COMMONCRITERIAMODEDISABLED),
           "COMMON_CRITERIA_MODE_ENABLED" => Ok(CommonCriteriaModeInfoCommonCriteriaModeStatusEnum::COMMONCRITERIAMODEENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommonCriteriaModeInfoCommonCriteriaModeStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CrossProfilePolicyCrossProfileCopyPasteEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether text copied from one profile (personal or work) can be pasted in the other profile.
pub enum CrossProfilePolicyCrossProfileCopyPasteEnum {
    

    /// Unspecified. Defaults to COPY_FROM_WORK_TO_PERSONAL_DISALLOWED
    ///
    /// "CROSS_PROFILE_COPY_PASTE_UNSPECIFIED"
    #[serde(rename="CROSS_PROFILE_COPY_PASTE_UNSPECIFIED")]
    CROSSPROFILECOPYPASTEUNSPECIFIED,
    

    /// Default. Prevents users from pasting into the personal profile text copied from the work profile. Text copied from the personal profile can be pasted into the work profile, and text copied from the work profile can be pasted into the work profile.
    ///
    /// "COPY_FROM_WORK_TO_PERSONAL_DISALLOWED"
    #[serde(rename="COPY_FROM_WORK_TO_PERSONAL_DISALLOWED")]
    COPYFROMWORKTOPERSONALDISALLOWED,
    

    /// Text copied in either profile can be pasted in the other profile.
    ///
    /// "CROSS_PROFILE_COPY_PASTE_ALLOWED"
    #[serde(rename="CROSS_PROFILE_COPY_PASTE_ALLOWED")]
    CROSSPROFILECOPYPASTEALLOWED,
}

impl AsRef<str> for CrossProfilePolicyCrossProfileCopyPasteEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CrossProfilePolicyCrossProfileCopyPasteEnum::CROSSPROFILECOPYPASTEUNSPECIFIED => "CROSS_PROFILE_COPY_PASTE_UNSPECIFIED",
            CrossProfilePolicyCrossProfileCopyPasteEnum::COPYFROMWORKTOPERSONALDISALLOWED => "COPY_FROM_WORK_TO_PERSONAL_DISALLOWED",
            CrossProfilePolicyCrossProfileCopyPasteEnum::CROSSPROFILECOPYPASTEALLOWED => "CROSS_PROFILE_COPY_PASTE_ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for CrossProfilePolicyCrossProfileCopyPasteEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CROSS_PROFILE_COPY_PASTE_UNSPECIFIED" => Ok(CrossProfilePolicyCrossProfileCopyPasteEnum::CROSSPROFILECOPYPASTEUNSPECIFIED),
           "COPY_FROM_WORK_TO_PERSONAL_DISALLOWED" => Ok(CrossProfilePolicyCrossProfileCopyPasteEnum::COPYFROMWORKTOPERSONALDISALLOWED),
           "CROSS_PROFILE_COPY_PASTE_ALLOWED" => Ok(CrossProfilePolicyCrossProfileCopyPasteEnum::CROSSPROFILECOPYPASTEALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CrossProfilePolicyCrossProfileCopyPasteEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CrossProfilePolicyCrossProfileDataSharingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether data from one profile (personal or work) can be shared with apps in the other profile. Specifically controls simple data sharing via intents. Management of other cross-profile communication channels, such as contact search, copy/paste, or connected work & personal apps, are configured separately.
pub enum CrossProfilePolicyCrossProfileDataSharingEnum {
    

    /// Unspecified. Defaults to DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED.
    ///
    /// "CROSS_PROFILE_DATA_SHARING_UNSPECIFIED"
    #[serde(rename="CROSS_PROFILE_DATA_SHARING_UNSPECIFIED")]
    CROSSPROFILEDATASHARINGUNSPECIFIED,
    

    /// Prevents data from being shared from both the personal profile to the work profile and the work profile to the personal profile.
    ///
    /// "CROSS_PROFILE_DATA_SHARING_DISALLOWED"
    #[serde(rename="CROSS_PROFILE_DATA_SHARING_DISALLOWED")]
    CROSSPROFILEDATASHARINGDISALLOWED,
    

    /// Default. Prevents users from sharing data from the work profile to apps in the personal profile. Personal data can be shared with work apps.
    ///
    /// "DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED"
    #[serde(rename="DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED")]
    DATASHARINGFROMWORKTOPERSONALDISALLOWED,
    

    /// Data from either profile can be shared with the other profile.
    ///
    /// "CROSS_PROFILE_DATA_SHARING_ALLOWED"
    #[serde(rename="CROSS_PROFILE_DATA_SHARING_ALLOWED")]
    CROSSPROFILEDATASHARINGALLOWED,
}

impl AsRef<str> for CrossProfilePolicyCrossProfileDataSharingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CrossProfilePolicyCrossProfileDataSharingEnum::CROSSPROFILEDATASHARINGUNSPECIFIED => "CROSS_PROFILE_DATA_SHARING_UNSPECIFIED",
            CrossProfilePolicyCrossProfileDataSharingEnum::CROSSPROFILEDATASHARINGDISALLOWED => "CROSS_PROFILE_DATA_SHARING_DISALLOWED",
            CrossProfilePolicyCrossProfileDataSharingEnum::DATASHARINGFROMWORKTOPERSONALDISALLOWED => "DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED",
            CrossProfilePolicyCrossProfileDataSharingEnum::CROSSPROFILEDATASHARINGALLOWED => "CROSS_PROFILE_DATA_SHARING_ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for CrossProfilePolicyCrossProfileDataSharingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CROSS_PROFILE_DATA_SHARING_UNSPECIFIED" => Ok(CrossProfilePolicyCrossProfileDataSharingEnum::CROSSPROFILEDATASHARINGUNSPECIFIED),
           "CROSS_PROFILE_DATA_SHARING_DISALLOWED" => Ok(CrossProfilePolicyCrossProfileDataSharingEnum::CROSSPROFILEDATASHARINGDISALLOWED),
           "DATA_SHARING_FROM_WORK_TO_PERSONAL_DISALLOWED" => Ok(CrossProfilePolicyCrossProfileDataSharingEnum::DATASHARINGFROMWORKTOPERSONALDISALLOWED),
           "CROSS_PROFILE_DATA_SHARING_ALLOWED" => Ok(CrossProfilePolicyCrossProfileDataSharingEnum::CROSSPROFILEDATASHARINGALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CrossProfilePolicyCrossProfileDataSharingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CrossProfilePolicyShowWorkContactsInPersonalProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether personal apps can access contacts stored in the work profile.See also exemptions_to_show_work_contacts_in_personal_profile.
pub enum CrossProfilePolicyShowWorkContactsInPersonalProfileEnum {
    

    /// Unspecified. Defaults to SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED.When this is set, exemptions_to_show_work_contacts_in_personal_profile must not be set.
    ///
    /// "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED"
    #[serde(rename="SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED")]
    SHOWWORKCONTACTSINPERSONALPROFILEUNSPECIFIED,
    

    /// Prevents personal apps from accessing work profile contacts and looking up work contacts.When this is set, personal apps specified in exemptions_to_show_work_contacts_in_personal_profile are allowlisted and can access work profile contacts directly.Supported on Android 7.0 and above. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 7.0.
    ///
    /// "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED"
    #[serde(rename="SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED")]
    SHOWWORKCONTACTSINPERSONALPROFILEDISALLOWED,
    

    /// Default. Allows apps in the personal profile to access work profile contacts including contact searches and incoming calls.When this is set, personal apps specified in exemptions_to_show_work_contacts_in_personal_profile are blocklisted and can not access work profile contacts directly.Supported on Android 7.0 and above. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 7.0.
    ///
    /// "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED"
    #[serde(rename="SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED")]
    SHOWWORKCONTACTSINPERSONALPROFILEALLOWED,
    

    /// Prevents most personal apps from accessing work profile contacts including contact searches and incoming calls, except for the OEM default Dialer, Messages, and Contacts apps. Neither user-configured Dialer, Messages, and Contacts apps, nor any other system or play installed apps, will be able to query work contacts directly.When this is set, personal apps specified in exemptions_to_show_work_contacts_in_personal_profile are allowlisted and can access work profile contacts.Supported on Android 14 and above. If this is set on a device with Android version less than 14, the behaviour falls back to SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED and a nonComplianceDetail with API_LEVEL is reported.
    ///
    /// "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED_EXCEPT_SYSTEM"
    #[serde(rename="SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED_EXCEPT_SYSTEM")]
    SHOWWORKCONTACTSINPERSONALPROFILEDISALLOWEDEXCEPTSYSTEM,
}

impl AsRef<str> for CrossProfilePolicyShowWorkContactsInPersonalProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEUNSPECIFIED => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED",
            CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEDISALLOWED => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED",
            CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEALLOWED => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED",
            CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEDISALLOWEDEXCEPTSYSTEM => "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED_EXCEPT_SYSTEM",
        }
    }
}

impl std::convert::TryFrom< &str> for CrossProfilePolicyShowWorkContactsInPersonalProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_UNSPECIFIED" => Ok(CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEUNSPECIFIED),
           "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED" => Ok(CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEDISALLOWED),
           "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_ALLOWED" => Ok(CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEALLOWED),
           "SHOW_WORK_CONTACTS_IN_PERSONAL_PROFILE_DISALLOWED_EXCEPT_SYSTEM" => Ok(CrossProfilePolicyShowWorkContactsInPersonalProfileEnum::SHOWWORKCONTACTSINPERSONALPROFILEDISALLOWEDEXCEPTSYSTEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CrossProfilePolicyShowWorkContactsInPersonalProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CrossProfilePolicyWorkProfileWidgetsDefaultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the default behaviour for work profile widgets. If the policy does not specify work_profile_widgets for a specific application, it will behave according to the value specified here.
pub enum CrossProfilePolicyWorkProfileWidgetsDefaultEnum {
    

    /// Unspecified. Defaults to WORK_PROFILE_WIDGETS_DEFAULT_DISALLOWED.
    ///
    /// "WORK_PROFILE_WIDGETS_DEFAULT_UNSPECIFIED"
    #[serde(rename="WORK_PROFILE_WIDGETS_DEFAULT_UNSPECIFIED")]
    WORKPROFILEWIDGETSDEFAULTUNSPECIFIED,
    

    /// Work profile widgets are allowed by default. This means that if the policy does not specify work_profile_widgets as WORK_PROFILE_WIDGETS_DISALLOWED for the application, it will be able to add widgets to the home screen.
    ///
    /// "WORK_PROFILE_WIDGETS_DEFAULT_ALLOWED"
    #[serde(rename="WORK_PROFILE_WIDGETS_DEFAULT_ALLOWED")]
    WORKPROFILEWIDGETSDEFAULTALLOWED,
    

    /// Work profile widgets are disallowed by default. This means that if the policy does not specify work_profile_widgets as WORK_PROFILE_WIDGETS_ALLOWED for the application, it will be unable to add widgets to the home screen.
    ///
    /// "WORK_PROFILE_WIDGETS_DEFAULT_DISALLOWED"
    #[serde(rename="WORK_PROFILE_WIDGETS_DEFAULT_DISALLOWED")]
    WORKPROFILEWIDGETSDEFAULTDISALLOWED,
}

impl AsRef<str> for CrossProfilePolicyWorkProfileWidgetsDefaultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CrossProfilePolicyWorkProfileWidgetsDefaultEnum::WORKPROFILEWIDGETSDEFAULTUNSPECIFIED => "WORK_PROFILE_WIDGETS_DEFAULT_UNSPECIFIED",
            CrossProfilePolicyWorkProfileWidgetsDefaultEnum::WORKPROFILEWIDGETSDEFAULTALLOWED => "WORK_PROFILE_WIDGETS_DEFAULT_ALLOWED",
            CrossProfilePolicyWorkProfileWidgetsDefaultEnum::WORKPROFILEWIDGETSDEFAULTDISALLOWED => "WORK_PROFILE_WIDGETS_DEFAULT_DISALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for CrossProfilePolicyWorkProfileWidgetsDefaultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WORK_PROFILE_WIDGETS_DEFAULT_UNSPECIFIED" => Ok(CrossProfilePolicyWorkProfileWidgetsDefaultEnum::WORKPROFILEWIDGETSDEFAULTUNSPECIFIED),
           "WORK_PROFILE_WIDGETS_DEFAULT_ALLOWED" => Ok(CrossProfilePolicyWorkProfileWidgetsDefaultEnum::WORKPROFILEWIDGETSDEFAULTALLOWED),
           "WORK_PROFILE_WIDGETS_DEFAULT_DISALLOWED" => Ok(CrossProfilePolicyWorkProfileWidgetsDefaultEnum::WORKPROFILEWIDGETSDEFAULTDISALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CrossProfilePolicyWorkProfileWidgetsDefaultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceAppliedStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state currently applied to the device.
pub enum DeviceAppliedStateEnum {
    

    /// This value is disallowed.
    ///
    /// "DEVICE_STATE_UNSPECIFIED"
    #[serde(rename="DEVICE_STATE_UNSPECIFIED")]
    DEVICESTATEUNSPECIFIED,
    

    /// The device is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The device is disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The device was deleted. This state is never returned by an API call, but is used in the final status report when the device acknowledges the deletion. If the device is deleted via the API call, this state is published to Pub/Sub. If the user deletes the work profile or resets the device, the device state will remain unknown to the server.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The device is lost. This state is only possible on organization-owned devices.
    ///
    /// "LOST"
    #[serde(rename="LOST")]
    LOST,
    

    /// The device is preparing for migrating to Android Management API. No further action is needed for the migration to continue.
    ///
    /// "PREPARING_FOR_MIGRATION"
    #[serde(rename="PREPARING_FOR_MIGRATION")]
    PREPARINGFORMIGRATION,
}

impl AsRef<str> for DeviceAppliedStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceAppliedStateEnum::DEVICESTATEUNSPECIFIED => "DEVICE_STATE_UNSPECIFIED",
            DeviceAppliedStateEnum::ACTIVE => "ACTIVE",
            DeviceAppliedStateEnum::DISABLED => "DISABLED",
            DeviceAppliedStateEnum::DELETED => "DELETED",
            DeviceAppliedStateEnum::PROVISIONING => "PROVISIONING",
            DeviceAppliedStateEnum::LOST => "LOST",
            DeviceAppliedStateEnum::PREPARINGFORMIGRATION => "PREPARING_FOR_MIGRATION",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceAppliedStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_STATE_UNSPECIFIED" => Ok(DeviceAppliedStateEnum::DEVICESTATEUNSPECIFIED),
           "ACTIVE" => Ok(DeviceAppliedStateEnum::ACTIVE),
           "DISABLED" => Ok(DeviceAppliedStateEnum::DISABLED),
           "DELETED" => Ok(DeviceAppliedStateEnum::DELETED),
           "PROVISIONING" => Ok(DeviceAppliedStateEnum::PROVISIONING),
           "LOST" => Ok(DeviceAppliedStateEnum::LOST),
           "PREPARING_FOR_MIGRATION" => Ok(DeviceAppliedStateEnum::PREPARINGFORMIGRATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceAppliedStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceManagementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of management mode Android Device Policy takes on the device. This influences which policy settings are supported.
pub enum DeviceManagementModeEnum {
    

    /// This value is disallowed.
    ///
    /// "MANAGEMENT_MODE_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_MODE_UNSPECIFIED")]
    MANAGEMENTMODEUNSPECIFIED,
    

    /// Device owner. Android Device Policy has full control over the device.
    ///
    /// "DEVICE_OWNER"
    #[serde(rename="DEVICE_OWNER")]
    DEVICEOWNER,
    

    /// Profile owner. Android Device Policy has control over a managed profile on the device.
    ///
    /// "PROFILE_OWNER"
    #[serde(rename="PROFILE_OWNER")]
    PROFILEOWNER,
}

impl AsRef<str> for DeviceManagementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceManagementModeEnum::MANAGEMENTMODEUNSPECIFIED => "MANAGEMENT_MODE_UNSPECIFIED",
            DeviceManagementModeEnum::DEVICEOWNER => "DEVICE_OWNER",
            DeviceManagementModeEnum::PROFILEOWNER => "PROFILE_OWNER",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceManagementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_MODE_UNSPECIFIED" => Ok(DeviceManagementModeEnum::MANAGEMENTMODEUNSPECIFIED),
           "DEVICE_OWNER" => Ok(DeviceManagementModeEnum::DEVICEOWNER),
           "PROFILE_OWNER" => Ok(DeviceManagementModeEnum::PROFILEOWNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceManagementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceOwnershipEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ownership of the managed device.
pub enum DeviceOwnershipEnum {
    

    /// Ownership is unspecified.
    ///
    /// "OWNERSHIP_UNSPECIFIED"
    #[serde(rename="OWNERSHIP_UNSPECIFIED")]
    OWNERSHIPUNSPECIFIED,
    

    /// Device is company-owned.
    ///
    /// "COMPANY_OWNED"
    #[serde(rename="COMPANY_OWNED")]
    COMPANYOWNED,
    

    /// Device is personally-owned.
    ///
    /// "PERSONALLY_OWNED"
    #[serde(rename="PERSONALLY_OWNED")]
    PERSONALLYOWNED,
}

impl AsRef<str> for DeviceOwnershipEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceOwnershipEnum::OWNERSHIPUNSPECIFIED => "OWNERSHIP_UNSPECIFIED",
            DeviceOwnershipEnum::COMPANYOWNED => "COMPANY_OWNED",
            DeviceOwnershipEnum::PERSONALLYOWNED => "PERSONALLY_OWNED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceOwnershipEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OWNERSHIP_UNSPECIFIED" => Ok(DeviceOwnershipEnum::OWNERSHIPUNSPECIFIED),
           "COMPANY_OWNED" => Ok(DeviceOwnershipEnum::COMPANYOWNED),
           "PERSONALLY_OWNED" => Ok(DeviceOwnershipEnum::PERSONALLYOWNED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceOwnershipEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state to be applied to the device. This field can be modified by a patch request. Note that when calling enterprises.devices.patch, ACTIVE and DISABLED are the only allowable values. To enter the device into a DELETED state, call enterprises.devices.delete.
pub enum DeviceStateEnum {
    

    /// This value is disallowed.
    ///
    /// "DEVICE_STATE_UNSPECIFIED"
    #[serde(rename="DEVICE_STATE_UNSPECIFIED")]
    DEVICESTATEUNSPECIFIED,
    

    /// The device is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The device is disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The device was deleted. This state is never returned by an API call, but is used in the final status report when the device acknowledges the deletion. If the device is deleted via the API call, this state is published to Pub/Sub. If the user deletes the work profile or resets the device, the device state will remain unknown to the server.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The device is being provisioned. Newly enrolled devices are in this state until they have a policy applied.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The device is lost. This state is only possible on organization-owned devices.
    ///
    /// "LOST"
    #[serde(rename="LOST")]
    LOST,
    

    /// The device is preparing for migrating to Android Management API. No further action is needed for the migration to continue.
    ///
    /// "PREPARING_FOR_MIGRATION"
    #[serde(rename="PREPARING_FOR_MIGRATION")]
    PREPARINGFORMIGRATION,
}

impl AsRef<str> for DeviceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceStateEnum::DEVICESTATEUNSPECIFIED => "DEVICE_STATE_UNSPECIFIED",
            DeviceStateEnum::ACTIVE => "ACTIVE",
            DeviceStateEnum::DISABLED => "DISABLED",
            DeviceStateEnum::DELETED => "DELETED",
            DeviceStateEnum::PROVISIONING => "PROVISIONING",
            DeviceStateEnum::LOST => "LOST",
            DeviceStateEnum::PREPARINGFORMIGRATION => "PREPARING_FOR_MIGRATION",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_STATE_UNSPECIFIED" => Ok(DeviceStateEnum::DEVICESTATEUNSPECIFIED),
           "ACTIVE" => Ok(DeviceStateEnum::ACTIVE),
           "DISABLED" => Ok(DeviceStateEnum::DISABLED),
           "DELETED" => Ok(DeviceStateEnum::DELETED),
           "PROVISIONING" => Ok(DeviceStateEnum::PROVISIONING),
           "LOST" => Ok(DeviceStateEnum::LOST),
           "PREPARING_FOR_MIGRATION" => Ok(DeviceStateEnum::PREPARINGFORMIGRATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceConnectivityManagementConfigureWifiEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls Wi-Fi configuring privileges. Based on the option set, user will have either full or limited or no control in configuring Wi-Fi networks.
pub enum DeviceConnectivityManagementConfigureWifiEnum {
    

    /// Unspecified. Defaults to ALLOW_CONFIGURING_WIFI unless wifiConfigDisabled is set to true. If wifiConfigDisabled is set to true, this is equivalent to DISALLOW_CONFIGURING_WIFI.
    ///
    /// "CONFIGURE_WIFI_UNSPECIFIED"
    #[serde(rename="CONFIGURE_WIFI_UNSPECIFIED")]
    CONFIGUREWIFIUNSPECIFIED,
    

    /// The user is allowed to configure Wi-Fi. wifiConfigDisabled is ignored.
    ///
    /// "ALLOW_CONFIGURING_WIFI"
    #[serde(rename="ALLOW_CONFIGURING_WIFI")]
    ALLOWCONFIGURINGWIFI,
    

    /// Adding new Wi-Fi configurations is disallowed. The user is only able to switch between already configured networks. Supported on Android 13 and above, on fully managed devices and work profiles on company-owned devices. If the setting is not supported, ALLOW_CONFIGURING_WIFI is set. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13. wifiConfigDisabled is ignored.
    ///
    /// "DISALLOW_ADD_WIFI_CONFIG"
    #[serde(rename="DISALLOW_ADD_WIFI_CONFIG")]
    DISALLOWADDWIFICONFIG,
    

    /// Disallows configuring Wi-Fi networks. The setting wifiConfigDisabled is ignored when this value is set. Supported on fully managed devices and work profile on company-owned devices, on all supported API levels. For fully managed devices, setting this removes all configured networks and retains only the networks configured using openNetworkConfiguration policy. For work profiles on company-owned devices, existing configured networks are not affected and the user is not allowed to add, remove, or modify Wi-Fi networks. Note: If a network connection can't be made at boot time and configuring Wi-Fi is disabled then network escape hatch will be shown in order to refresh the device policy (see networkEscapeHatchEnabled).
    ///
    /// "DISALLOW_CONFIGURING_WIFI"
    #[serde(rename="DISALLOW_CONFIGURING_WIFI")]
    DISALLOWCONFIGURINGWIFI,
}

impl AsRef<str> for DeviceConnectivityManagementConfigureWifiEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceConnectivityManagementConfigureWifiEnum::CONFIGUREWIFIUNSPECIFIED => "CONFIGURE_WIFI_UNSPECIFIED",
            DeviceConnectivityManagementConfigureWifiEnum::ALLOWCONFIGURINGWIFI => "ALLOW_CONFIGURING_WIFI",
            DeviceConnectivityManagementConfigureWifiEnum::DISALLOWADDWIFICONFIG => "DISALLOW_ADD_WIFI_CONFIG",
            DeviceConnectivityManagementConfigureWifiEnum::DISALLOWCONFIGURINGWIFI => "DISALLOW_CONFIGURING_WIFI",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceConnectivityManagementConfigureWifiEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONFIGURE_WIFI_UNSPECIFIED" => Ok(DeviceConnectivityManagementConfigureWifiEnum::CONFIGUREWIFIUNSPECIFIED),
           "ALLOW_CONFIGURING_WIFI" => Ok(DeviceConnectivityManagementConfigureWifiEnum::ALLOWCONFIGURINGWIFI),
           "DISALLOW_ADD_WIFI_CONFIG" => Ok(DeviceConnectivityManagementConfigureWifiEnum::DISALLOWADDWIFICONFIG),
           "DISALLOW_CONFIGURING_WIFI" => Ok(DeviceConnectivityManagementConfigureWifiEnum::DISALLOWCONFIGURINGWIFI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceConnectivityManagementConfigureWifiEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceConnectivityManagementTetheringSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls tethering settings. Based on the value set, the user is partially or fully disallowed from using different forms of tethering.
pub enum DeviceConnectivityManagementTetheringSettingsEnum {
    

    /// Unspecified. Defaults to ALLOW_ALL_TETHERING unless tetheringConfigDisabled is set to true. If tetheringConfigDisabled is set to true, this is equivalent to DISALLOW_ALL_TETHERING.
    ///
    /// "TETHERING_SETTINGS_UNSPECIFIED"
    #[serde(rename="TETHERING_SETTINGS_UNSPECIFIED")]
    TETHERINGSETTINGSUNSPECIFIED,
    

    /// Allows configuration and use of all forms of tethering. tetheringConfigDisabled is ignored.
    ///
    /// "ALLOW_ALL_TETHERING"
    #[serde(rename="ALLOW_ALL_TETHERING")]
    ALLOWALLTETHERING,
    

    /// Disallows the user from using Wi-Fi tethering. Supported on company owned devices running Android 13 and above. If the setting is not supported, ALLOW_ALL_TETHERING will be set. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13. tetheringConfigDisabled is ignored.
    ///
    /// "DISALLOW_WIFI_TETHERING"
    #[serde(rename="DISALLOW_WIFI_TETHERING")]
    DISALLOWWIFITETHERING,
    

    /// Disallows all forms of tethering. Supported on fully managed devices and work profile on company-owned devices, on all supported android versions. The setting tetheringConfigDisabled is ignored.
    ///
    /// "DISALLOW_ALL_TETHERING"
    #[serde(rename="DISALLOW_ALL_TETHERING")]
    DISALLOWALLTETHERING,
}

impl AsRef<str> for DeviceConnectivityManagementTetheringSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceConnectivityManagementTetheringSettingsEnum::TETHERINGSETTINGSUNSPECIFIED => "TETHERING_SETTINGS_UNSPECIFIED",
            DeviceConnectivityManagementTetheringSettingsEnum::ALLOWALLTETHERING => "ALLOW_ALL_TETHERING",
            DeviceConnectivityManagementTetheringSettingsEnum::DISALLOWWIFITETHERING => "DISALLOW_WIFI_TETHERING",
            DeviceConnectivityManagementTetheringSettingsEnum::DISALLOWALLTETHERING => "DISALLOW_ALL_TETHERING",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceConnectivityManagementTetheringSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TETHERING_SETTINGS_UNSPECIFIED" => Ok(DeviceConnectivityManagementTetheringSettingsEnum::TETHERINGSETTINGSUNSPECIFIED),
           "ALLOW_ALL_TETHERING" => Ok(DeviceConnectivityManagementTetheringSettingsEnum::ALLOWALLTETHERING),
           "DISALLOW_WIFI_TETHERING" => Ok(DeviceConnectivityManagementTetheringSettingsEnum::DISALLOWWIFITETHERING),
           "DISALLOW_ALL_TETHERING" => Ok(DeviceConnectivityManagementTetheringSettingsEnum::DISALLOWALLTETHERING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceConnectivityManagementTetheringSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceConnectivityManagementUsbDataAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls what files and/or data can be transferred via USB. Supported only on company-owned devices.
pub enum DeviceConnectivityManagementUsbDataAccessEnum {
    

    /// Unspecified. Defaults to DISALLOW_USB_FILE_TRANSFER.
    ///
    /// "USB_DATA_ACCESS_UNSPECIFIED"
    #[serde(rename="USB_DATA_ACCESS_UNSPECIFIED")]
    USBDATAACCESSUNSPECIFIED,
    

    /// All types of USB data transfers are allowed. usbFileTransferDisabled is ignored.
    ///
    /// "ALLOW_USB_DATA_TRANSFER"
    #[serde(rename="ALLOW_USB_DATA_TRANSFER")]
    ALLOWUSBDATATRANSFER,
    

    /// Transferring files over USB is disallowed. Other types of USB data connections, such as mouse and keyboard connection, are allowed. usbFileTransferDisabled is ignored.
    ///
    /// "DISALLOW_USB_FILE_TRANSFER"
    #[serde(rename="DISALLOW_USB_FILE_TRANSFER")]
    DISALLOWUSBFILETRANSFER,
    

    /// When set, all types of USB data transfers are prohibited. Supported for devices running Android 12 or above with USB HAL 1.3 or above. If the setting is not supported, DISALLOW_USB_FILE_TRANSFER will be set. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 12. A nonComplianceDetail with DEVICE_INCOMPATIBLE is reported if the device does not have USB HAL 1.3 or above. usbFileTransferDisabled is ignored.
    ///
    /// "DISALLOW_USB_DATA_TRANSFER"
    #[serde(rename="DISALLOW_USB_DATA_TRANSFER")]
    DISALLOWUSBDATATRANSFER,
}

impl AsRef<str> for DeviceConnectivityManagementUsbDataAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceConnectivityManagementUsbDataAccessEnum::USBDATAACCESSUNSPECIFIED => "USB_DATA_ACCESS_UNSPECIFIED",
            DeviceConnectivityManagementUsbDataAccessEnum::ALLOWUSBDATATRANSFER => "ALLOW_USB_DATA_TRANSFER",
            DeviceConnectivityManagementUsbDataAccessEnum::DISALLOWUSBFILETRANSFER => "DISALLOW_USB_FILE_TRANSFER",
            DeviceConnectivityManagementUsbDataAccessEnum::DISALLOWUSBDATATRANSFER => "DISALLOW_USB_DATA_TRANSFER",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceConnectivityManagementUsbDataAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USB_DATA_ACCESS_UNSPECIFIED" => Ok(DeviceConnectivityManagementUsbDataAccessEnum::USBDATAACCESSUNSPECIFIED),
           "ALLOW_USB_DATA_TRANSFER" => Ok(DeviceConnectivityManagementUsbDataAccessEnum::ALLOWUSBDATATRANSFER),
           "DISALLOW_USB_FILE_TRANSFER" => Ok(DeviceConnectivityManagementUsbDataAccessEnum::DISALLOWUSBFILETRANSFER),
           "DISALLOW_USB_DATA_TRANSFER" => Ok(DeviceConnectivityManagementUsbDataAccessEnum::DISALLOWUSBDATATRANSFER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceConnectivityManagementUsbDataAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceConnectivityManagementWifiDirectSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls configuring and using Wi-Fi direct settings. Supported on company-owned devices running Android 13 and above.
pub enum DeviceConnectivityManagementWifiDirectSettingsEnum {
    

    /// Unspecified. Defaults to ALLOW_WIFI_DIRECT
    ///
    /// "WIFI_DIRECT_SETTINGS_UNSPECIFIED"
    #[serde(rename="WIFI_DIRECT_SETTINGS_UNSPECIFIED")]
    WIFIDIRECTSETTINGSUNSPECIFIED,
    

    /// The user is allowed to use Wi-Fi direct.
    ///
    /// "ALLOW_WIFI_DIRECT"
    #[serde(rename="ALLOW_WIFI_DIRECT")]
    ALLOWWIFIDIRECT,
    

    /// The user is not allowed to use Wi-Fi direct. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13.
    ///
    /// "DISALLOW_WIFI_DIRECT"
    #[serde(rename="DISALLOW_WIFI_DIRECT")]
    DISALLOWWIFIDIRECT,
}

impl AsRef<str> for DeviceConnectivityManagementWifiDirectSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceConnectivityManagementWifiDirectSettingsEnum::WIFIDIRECTSETTINGSUNSPECIFIED => "WIFI_DIRECT_SETTINGS_UNSPECIFIED",
            DeviceConnectivityManagementWifiDirectSettingsEnum::ALLOWWIFIDIRECT => "ALLOW_WIFI_DIRECT",
            DeviceConnectivityManagementWifiDirectSettingsEnum::DISALLOWWIFIDIRECT => "DISALLOW_WIFI_DIRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceConnectivityManagementWifiDirectSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WIFI_DIRECT_SETTINGS_UNSPECIFIED" => Ok(DeviceConnectivityManagementWifiDirectSettingsEnum::WIFIDIRECTSETTINGSUNSPECIFIED),
           "ALLOW_WIFI_DIRECT" => Ok(DeviceConnectivityManagementWifiDirectSettingsEnum::ALLOWWIFIDIRECT),
           "DISALLOW_WIFI_DIRECT" => Ok(DeviceConnectivityManagementWifiDirectSettingsEnum::DISALLOWWIFIDIRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceConnectivityManagementWifiDirectSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceRadioStateAirplaneModeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether airplane mode can be toggled by the user or not.
pub enum DeviceRadioStateAirplaneModeStateEnum {
    

    /// Unspecified. Defaults to AIRPLANE_MODE_USER_CHOICE.
    ///
    /// "AIRPLANE_MODE_STATE_UNSPECIFIED"
    #[serde(rename="AIRPLANE_MODE_STATE_UNSPECIFIED")]
    AIRPLANEMODESTATEUNSPECIFIED,
    

    /// The user is allowed to toggle airplane mode on or off.
    ///
    /// "AIRPLANE_MODE_USER_CHOICE"
    #[serde(rename="AIRPLANE_MODE_USER_CHOICE")]
    AIRPLANEMODEUSERCHOICE,
    

    /// Airplane mode is disabled. The user is not allowed to toggle airplane mode on. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 9.
    ///
    /// "AIRPLANE_MODE_DISABLED"
    #[serde(rename="AIRPLANE_MODE_DISABLED")]
    AIRPLANEMODEDISABLED,
}

impl AsRef<str> for DeviceRadioStateAirplaneModeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceRadioStateAirplaneModeStateEnum::AIRPLANEMODESTATEUNSPECIFIED => "AIRPLANE_MODE_STATE_UNSPECIFIED",
            DeviceRadioStateAirplaneModeStateEnum::AIRPLANEMODEUSERCHOICE => "AIRPLANE_MODE_USER_CHOICE",
            DeviceRadioStateAirplaneModeStateEnum::AIRPLANEMODEDISABLED => "AIRPLANE_MODE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceRadioStateAirplaneModeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AIRPLANE_MODE_STATE_UNSPECIFIED" => Ok(DeviceRadioStateAirplaneModeStateEnum::AIRPLANEMODESTATEUNSPECIFIED),
           "AIRPLANE_MODE_USER_CHOICE" => Ok(DeviceRadioStateAirplaneModeStateEnum::AIRPLANEMODEUSERCHOICE),
           "AIRPLANE_MODE_DISABLED" => Ok(DeviceRadioStateAirplaneModeStateEnum::AIRPLANEMODEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceRadioStateAirplaneModeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceRadioStateCellularTwoGStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether cellular 2G setting can be toggled by the user or not.
pub enum DeviceRadioStateCellularTwoGStateEnum {
    

    /// Unspecified. Defaults to CELLULAR_TWO_G_USER_CHOICE.
    ///
    /// "CELLULAR_TWO_G_STATE_UNSPECIFIED"
    #[serde(rename="CELLULAR_TWO_G_STATE_UNSPECIFIED")]
    CELLULARTWOGSTATEUNSPECIFIED,
    

    /// The user is allowed to toggle cellular 2G on or off.
    ///
    /// "CELLULAR_TWO_G_USER_CHOICE"
    #[serde(rename="CELLULAR_TWO_G_USER_CHOICE")]
    CELLULARTWOGUSERCHOICE,
    

    /// Cellular 2G is disabled. The user is not allowed to toggle cellular 2G on via settings. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 14.
    ///
    /// "CELLULAR_TWO_G_DISABLED"
    #[serde(rename="CELLULAR_TWO_G_DISABLED")]
    CELLULARTWOGDISABLED,
}

impl AsRef<str> for DeviceRadioStateCellularTwoGStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceRadioStateCellularTwoGStateEnum::CELLULARTWOGSTATEUNSPECIFIED => "CELLULAR_TWO_G_STATE_UNSPECIFIED",
            DeviceRadioStateCellularTwoGStateEnum::CELLULARTWOGUSERCHOICE => "CELLULAR_TWO_G_USER_CHOICE",
            DeviceRadioStateCellularTwoGStateEnum::CELLULARTWOGDISABLED => "CELLULAR_TWO_G_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceRadioStateCellularTwoGStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CELLULAR_TWO_G_STATE_UNSPECIFIED" => Ok(DeviceRadioStateCellularTwoGStateEnum::CELLULARTWOGSTATEUNSPECIFIED),
           "CELLULAR_TWO_G_USER_CHOICE" => Ok(DeviceRadioStateCellularTwoGStateEnum::CELLULARTWOGUSERCHOICE),
           "CELLULAR_TWO_G_DISABLED" => Ok(DeviceRadioStateCellularTwoGStateEnum::CELLULARTWOGDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceRadioStateCellularTwoGStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceRadioStateMinimumWifiSecurityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The minimum required security level of Wi-Fi networks that the device can connect to.
pub enum DeviceRadioStateMinimumWifiSecurityLevelEnum {
    

    /// Defaults to OPEN_NETWORK_SECURITY, which means the device will be able to connect to all types of Wi-Fi networks.
    ///
    /// "MINIMUM_WIFI_SECURITY_LEVEL_UNSPECIFIED"
    #[serde(rename="MINIMUM_WIFI_SECURITY_LEVEL_UNSPECIFIED")]
    MINIMUMWIFISECURITYLEVELUNSPECIFIED,
    

    /// The device will be able to connect to all types of Wi-Fi networks.
    ///
    /// "OPEN_NETWORK_SECURITY"
    #[serde(rename="OPEN_NETWORK_SECURITY")]
    OPENNETWORKSECURITY,
    

    /// A personal network such as WEP, WPA2-PSK is the minimum required security. The device will not be able to connect to open wifi networks. This is stricter than OPEN_NETWORK_SECURITY. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13.
    ///
    /// "PERSONAL_NETWORK_SECURITY"
    #[serde(rename="PERSONAL_NETWORK_SECURITY")]
    PERSONALNETWORKSECURITY,
    

    /// An enterprise EAP network is the minimum required security level. The device will not be able to connect to Wi-Fi network below this security level. This is stricter than PERSONAL_NETWORK_SECURITY. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13.
    ///
    /// "ENTERPRISE_NETWORK_SECURITY"
    #[serde(rename="ENTERPRISE_NETWORK_SECURITY")]
    ENTERPRISENETWORKSECURITY,
    

    /// A 192-bit enterprise network is the minimum required security level. The device will not be able to connect to Wi-Fi network below this security level. This is stricter than ENTERPRISE_NETWORK_SECURITY. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13.
    ///
    /// "ENTERPRISE_BIT192_NETWORK_SECURITY"
    #[serde(rename="ENTERPRISE_BIT192_NETWORK_SECURITY")]
    ENTERPRISEBIT192NETWORKSECURITY,
}

impl AsRef<str> for DeviceRadioStateMinimumWifiSecurityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceRadioStateMinimumWifiSecurityLevelEnum::MINIMUMWIFISECURITYLEVELUNSPECIFIED => "MINIMUM_WIFI_SECURITY_LEVEL_UNSPECIFIED",
            DeviceRadioStateMinimumWifiSecurityLevelEnum::OPENNETWORKSECURITY => "OPEN_NETWORK_SECURITY",
            DeviceRadioStateMinimumWifiSecurityLevelEnum::PERSONALNETWORKSECURITY => "PERSONAL_NETWORK_SECURITY",
            DeviceRadioStateMinimumWifiSecurityLevelEnum::ENTERPRISENETWORKSECURITY => "ENTERPRISE_NETWORK_SECURITY",
            DeviceRadioStateMinimumWifiSecurityLevelEnum::ENTERPRISEBIT192NETWORKSECURITY => "ENTERPRISE_BIT192_NETWORK_SECURITY",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceRadioStateMinimumWifiSecurityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MINIMUM_WIFI_SECURITY_LEVEL_UNSPECIFIED" => Ok(DeviceRadioStateMinimumWifiSecurityLevelEnum::MINIMUMWIFISECURITYLEVELUNSPECIFIED),
           "OPEN_NETWORK_SECURITY" => Ok(DeviceRadioStateMinimumWifiSecurityLevelEnum::OPENNETWORKSECURITY),
           "PERSONAL_NETWORK_SECURITY" => Ok(DeviceRadioStateMinimumWifiSecurityLevelEnum::PERSONALNETWORKSECURITY),
           "ENTERPRISE_NETWORK_SECURITY" => Ok(DeviceRadioStateMinimumWifiSecurityLevelEnum::ENTERPRISENETWORKSECURITY),
           "ENTERPRISE_BIT192_NETWORK_SECURITY" => Ok(DeviceRadioStateMinimumWifiSecurityLevelEnum::ENTERPRISEBIT192NETWORKSECURITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceRadioStateMinimumWifiSecurityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceRadioStateUltraWidebandStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the state of the ultra wideband setting and whether the user can toggle it on or off.
pub enum DeviceRadioStateUltraWidebandStateEnum {
    

    /// Unspecified. Defaults to ULTRA_WIDEBAND_USER_CHOICE.
    ///
    /// "ULTRA_WIDEBAND_STATE_UNSPECIFIED"
    #[serde(rename="ULTRA_WIDEBAND_STATE_UNSPECIFIED")]
    ULTRAWIDEBANDSTATEUNSPECIFIED,
    

    /// The user is allowed to toggle ultra wideband on or off.
    ///
    /// "ULTRA_WIDEBAND_USER_CHOICE"
    #[serde(rename="ULTRA_WIDEBAND_USER_CHOICE")]
    ULTRAWIDEBANDUSERCHOICE,
    

    /// Ultra wideband is disabled. The user is not allowed to toggle ultra wideband on via settings. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 14.
    ///
    /// "ULTRA_WIDEBAND_DISABLED"
    #[serde(rename="ULTRA_WIDEBAND_DISABLED")]
    ULTRAWIDEBANDDISABLED,
}

impl AsRef<str> for DeviceRadioStateUltraWidebandStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceRadioStateUltraWidebandStateEnum::ULTRAWIDEBANDSTATEUNSPECIFIED => "ULTRA_WIDEBAND_STATE_UNSPECIFIED",
            DeviceRadioStateUltraWidebandStateEnum::ULTRAWIDEBANDUSERCHOICE => "ULTRA_WIDEBAND_USER_CHOICE",
            DeviceRadioStateUltraWidebandStateEnum::ULTRAWIDEBANDDISABLED => "ULTRA_WIDEBAND_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceRadioStateUltraWidebandStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ULTRA_WIDEBAND_STATE_UNSPECIFIED" => Ok(DeviceRadioStateUltraWidebandStateEnum::ULTRAWIDEBANDSTATEUNSPECIFIED),
           "ULTRA_WIDEBAND_USER_CHOICE" => Ok(DeviceRadioStateUltraWidebandStateEnum::ULTRAWIDEBANDUSERCHOICE),
           "ULTRA_WIDEBAND_DISABLED" => Ok(DeviceRadioStateUltraWidebandStateEnum::ULTRAWIDEBANDDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceRadioStateUltraWidebandStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceRadioStateWifiStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls current state of Wi-Fi and if user can change its state.
pub enum DeviceRadioStateWifiStateEnum {
    

    /// Unspecified. Defaults to WIFI_STATE_USER_CHOICE
    ///
    /// "WIFI_STATE_UNSPECIFIED"
    #[serde(rename="WIFI_STATE_UNSPECIFIED")]
    WIFISTATEUNSPECIFIED,
    

    /// User is allowed to enable/disable Wi-Fi.
    ///
    /// "WIFI_STATE_USER_CHOICE"
    #[serde(rename="WIFI_STATE_USER_CHOICE")]
    WIFISTATEUSERCHOICE,
    

    /// Wi-Fi is on and the user is not allowed to turn it off. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13.
    ///
    /// "WIFI_ENABLED"
    #[serde(rename="WIFI_ENABLED")]
    WIFIENABLED,
    

    /// Wi-Fi is off and the user is not allowed to turn it on. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 13.
    ///
    /// "WIFI_DISABLED"
    #[serde(rename="WIFI_DISABLED")]
    WIFIDISABLED,
}

impl AsRef<str> for DeviceRadioStateWifiStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceRadioStateWifiStateEnum::WIFISTATEUNSPECIFIED => "WIFI_STATE_UNSPECIFIED",
            DeviceRadioStateWifiStateEnum::WIFISTATEUSERCHOICE => "WIFI_STATE_USER_CHOICE",
            DeviceRadioStateWifiStateEnum::WIFIENABLED => "WIFI_ENABLED",
            DeviceRadioStateWifiStateEnum::WIFIDISABLED => "WIFI_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceRadioStateWifiStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WIFI_STATE_UNSPECIFIED" => Ok(DeviceRadioStateWifiStateEnum::WIFISTATEUNSPECIFIED),
           "WIFI_STATE_USER_CHOICE" => Ok(DeviceRadioStateWifiStateEnum::WIFISTATEUSERCHOICE),
           "WIFI_ENABLED" => Ok(DeviceRadioStateWifiStateEnum::WIFIENABLED),
           "WIFI_DISABLED" => Ok(DeviceRadioStateWifiStateEnum::WIFIDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceRadioStateWifiStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceSettingEncryptionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Encryption status from DevicePolicyManager.
pub enum DeviceSettingEncryptionStatusEnum {
    

    /// Unspecified. No device should have this type.
    ///
    /// "ENCRYPTION_STATUS_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_STATUS_UNSPECIFIED")]
    ENCRYPTIONSTATUSUNSPECIFIED,
    

    /// Encryption is not supported by the device.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// Encryption is supported by the device, but is not currently active.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// Encryption is not currently active, but is currently being activated.
    ///
    /// "ACTIVATING"
    #[serde(rename="ACTIVATING")]
    ACTIVATING,
    

    /// Encryption is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Encryption is active, but an encryption key is not set by the user.
    ///
    /// "ACTIVE_DEFAULT_KEY"
    #[serde(rename="ACTIVE_DEFAULT_KEY")]
    ACTIVEDEFAULTKEY,
    

    /// Encryption is active, and the encryption key is tied to the user profile.
    ///
    /// "ACTIVE_PER_USER"
    #[serde(rename="ACTIVE_PER_USER")]
    ACTIVEPERUSER,
}

impl AsRef<str> for DeviceSettingEncryptionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceSettingEncryptionStatusEnum::ENCRYPTIONSTATUSUNSPECIFIED => "ENCRYPTION_STATUS_UNSPECIFIED",
            DeviceSettingEncryptionStatusEnum::UNSUPPORTED => "UNSUPPORTED",
            DeviceSettingEncryptionStatusEnum::INACTIVE => "INACTIVE",
            DeviceSettingEncryptionStatusEnum::ACTIVATING => "ACTIVATING",
            DeviceSettingEncryptionStatusEnum::ACTIVE => "ACTIVE",
            DeviceSettingEncryptionStatusEnum::ACTIVEDEFAULTKEY => "ACTIVE_DEFAULT_KEY",
            DeviceSettingEncryptionStatusEnum::ACTIVEPERUSER => "ACTIVE_PER_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceSettingEncryptionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_STATUS_UNSPECIFIED" => Ok(DeviceSettingEncryptionStatusEnum::ENCRYPTIONSTATUSUNSPECIFIED),
           "UNSUPPORTED" => Ok(DeviceSettingEncryptionStatusEnum::UNSUPPORTED),
           "INACTIVE" => Ok(DeviceSettingEncryptionStatusEnum::INACTIVE),
           "ACTIVATING" => Ok(DeviceSettingEncryptionStatusEnum::ACTIVATING),
           "ACTIVE" => Ok(DeviceSettingEncryptionStatusEnum::ACTIVE),
           "ACTIVE_DEFAULT_KEY" => Ok(DeviceSettingEncryptionStatusEnum::ACTIVEDEFAULTKEY),
           "ACTIVE_PER_USER" => Ok(DeviceSettingEncryptionStatusEnum::ACTIVEPERUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceSettingEncryptionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DisplayStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the display.
pub enum DisplayStateEnum {
    

    /// This value is disallowed.
    ///
    /// "DISPLAY_STATE_UNSPECIFIED"
    #[serde(rename="DISPLAY_STATE_UNSPECIFIED")]
    DISPLAYSTATEUNSPECIFIED,
    

    /// Display is off.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// Display is on.
    ///
    /// "ON"
    #[serde(rename="ON")]
    ON,
    

    /// Display is dozing in a low power state
    ///
    /// "DOZE"
    #[serde(rename="DOZE")]
    DOZE,
    

    /// Display is dozing in a suspended low power state.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for DisplayStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DisplayStateEnum::DISPLAYSTATEUNSPECIFIED => "DISPLAY_STATE_UNSPECIFIED",
            DisplayStateEnum::OFF => "OFF",
            DisplayStateEnum::ON => "ON",
            DisplayStateEnum::DOZE => "DOZE",
            DisplayStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for DisplayStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY_STATE_UNSPECIFIED" => Ok(DisplayStateEnum::DISPLAYSTATEUNSPECIFIED),
           "OFF" => Ok(DisplayStateEnum::OFF),
           "ON" => Ok(DisplayStateEnum::ON),
           "DOZE" => Ok(DisplayStateEnum::DOZE),
           "SUSPENDED" => Ok(DisplayStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DisplayStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnrollmentTokenAllowPersonalUsageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device.
pub enum EnrollmentTokenAllowPersonalUsageEnum {
    

    /// Personal usage restriction is not specified
    ///
    /// "ALLOW_PERSONAL_USAGE_UNSPECIFIED"
    #[serde(rename="ALLOW_PERSONAL_USAGE_UNSPECIFIED")]
    ALLOWPERSONALUSAGEUNSPECIFIED,
    

    /// Personal usage is allowed
    ///
    /// "PERSONAL_USAGE_ALLOWED"
    #[serde(rename="PERSONAL_USAGE_ALLOWED")]
    PERSONALUSAGEALLOWED,
    

    /// Personal usage is disallowed
    ///
    /// "PERSONAL_USAGE_DISALLOWED"
    #[serde(rename="PERSONAL_USAGE_DISALLOWED")]
    PERSONALUSAGEDISALLOWED,
}

impl AsRef<str> for EnrollmentTokenAllowPersonalUsageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnrollmentTokenAllowPersonalUsageEnum::ALLOWPERSONALUSAGEUNSPECIFIED => "ALLOW_PERSONAL_USAGE_UNSPECIFIED",
            EnrollmentTokenAllowPersonalUsageEnum::PERSONALUSAGEALLOWED => "PERSONAL_USAGE_ALLOWED",
            EnrollmentTokenAllowPersonalUsageEnum::PERSONALUSAGEDISALLOWED => "PERSONAL_USAGE_DISALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for EnrollmentTokenAllowPersonalUsageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALLOW_PERSONAL_USAGE_UNSPECIFIED" => Ok(EnrollmentTokenAllowPersonalUsageEnum::ALLOWPERSONALUSAGEUNSPECIFIED),
           "PERSONAL_USAGE_ALLOWED" => Ok(EnrollmentTokenAllowPersonalUsageEnum::PERSONALUSAGEALLOWED),
           "PERSONAL_USAGE_DISALLOWED" => Ok(EnrollmentTokenAllowPersonalUsageEnum::PERSONALUSAGEDISALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnrollmentTokenAllowPersonalUsageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseEnabledNotificationTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of Google Pub/Sub notifications enabled for the enterprise.
pub enum EnterpriseEnabledNotificationTypesEnum {
    

    /// This value is ignored.
    ///
    /// "NOTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="NOTIFICATION_TYPE_UNSPECIFIED")]
    NOTIFICATIONTYPEUNSPECIFIED,
    

    /// A notification sent when a device enrolls.
    ///
    /// "ENROLLMENT"
    #[serde(rename="ENROLLMENT")]
    ENROLLMENT,
    

    /// Deprecated.
    ///
    /// "COMPLIANCE_REPORT"
    #[serde(rename="COMPLIANCE_REPORT")]
    COMPLIANCEREPORT,
    

    /// A notification sent when a device issues a status report.
    ///
    /// "STATUS_REPORT"
    #[serde(rename="STATUS_REPORT")]
    STATUSREPORT,
    

    /// A notification sent when a device command has completed.
    ///
    /// "COMMAND"
    #[serde(rename="COMMAND")]
    COMMAND,
    

    /// A notification sent when device sends BatchUsageLogEvents.
    ///
    /// "USAGE_LOGS"
    #[serde(rename="USAGE_LOGS")]
    USAGELOGS,
}

impl AsRef<str> for EnterpriseEnabledNotificationTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseEnabledNotificationTypesEnum::NOTIFICATIONTYPEUNSPECIFIED => "NOTIFICATION_TYPE_UNSPECIFIED",
            EnterpriseEnabledNotificationTypesEnum::ENROLLMENT => "ENROLLMENT",
            EnterpriseEnabledNotificationTypesEnum::COMPLIANCEREPORT => "COMPLIANCE_REPORT",
            EnterpriseEnabledNotificationTypesEnum::STATUSREPORT => "STATUS_REPORT",
            EnterpriseEnabledNotificationTypesEnum::COMMAND => "COMMAND",
            EnterpriseEnabledNotificationTypesEnum::USAGELOGS => "USAGE_LOGS",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseEnabledNotificationTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOTIFICATION_TYPE_UNSPECIFIED" => Ok(EnterpriseEnabledNotificationTypesEnum::NOTIFICATIONTYPEUNSPECIFIED),
           "ENROLLMENT" => Ok(EnterpriseEnabledNotificationTypesEnum::ENROLLMENT),
           "COMPLIANCE_REPORT" => Ok(EnterpriseEnabledNotificationTypesEnum::COMPLIANCEREPORT),
           "STATUS_REPORT" => Ok(EnterpriseEnabledNotificationTypesEnum::STATUSREPORT),
           "COMMAND" => Ok(EnterpriseEnabledNotificationTypesEnum::COMMAND),
           "USAGE_LOGS" => Ok(EnterpriseEnabledNotificationTypesEnum::USAGELOGS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseEnabledNotificationTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstallConstraintChargingConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Charging constraint.
pub enum InstallConstraintChargingConstraintEnum {
    

    /// Unspecified. Default to CHARGING_NOT_REQUIRED.
    ///
    /// "CHARGING_CONSTRAINT_UNSPECIFIED"
    #[serde(rename="CHARGING_CONSTRAINT_UNSPECIFIED")]
    CHARGINGCONSTRAINTUNSPECIFIED,
    

    /// Device doesn't have to be charging.
    ///
    /// "CHARGING_NOT_REQUIRED"
    #[serde(rename="CHARGING_NOT_REQUIRED")]
    CHARGINGNOTREQUIRED,
    

    /// Device has to be charging.
    ///
    /// "INSTALL_ONLY_WHEN_CHARGING"
    #[serde(rename="INSTALL_ONLY_WHEN_CHARGING")]
    INSTALLONLYWHENCHARGING,
}

impl AsRef<str> for InstallConstraintChargingConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstallConstraintChargingConstraintEnum::CHARGINGCONSTRAINTUNSPECIFIED => "CHARGING_CONSTRAINT_UNSPECIFIED",
            InstallConstraintChargingConstraintEnum::CHARGINGNOTREQUIRED => "CHARGING_NOT_REQUIRED",
            InstallConstraintChargingConstraintEnum::INSTALLONLYWHENCHARGING => "INSTALL_ONLY_WHEN_CHARGING",
        }
    }
}

impl std::convert::TryFrom< &str> for InstallConstraintChargingConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHARGING_CONSTRAINT_UNSPECIFIED" => Ok(InstallConstraintChargingConstraintEnum::CHARGINGCONSTRAINTUNSPECIFIED),
           "CHARGING_NOT_REQUIRED" => Ok(InstallConstraintChargingConstraintEnum::CHARGINGNOTREQUIRED),
           "INSTALL_ONLY_WHEN_CHARGING" => Ok(InstallConstraintChargingConstraintEnum::INSTALLONLYWHENCHARGING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstallConstraintChargingConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstallConstraintDeviceIdleConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Device idle constraint.
pub enum InstallConstraintDeviceIdleConstraintEnum {
    

    /// Unspecified. Default to DEVICE_IDLE_NOT_REQUIRED.
    ///
    /// "DEVICE_IDLE_CONSTRAINT_UNSPECIFIED"
    #[serde(rename="DEVICE_IDLE_CONSTRAINT_UNSPECIFIED")]
    DEVICEIDLECONSTRAINTUNSPECIFIED,
    

    /// Device doesn't have to be idle, app can be installed while the user is interacting with the device.
    ///
    /// "DEVICE_IDLE_NOT_REQUIRED"
    #[serde(rename="DEVICE_IDLE_NOT_REQUIRED")]
    DEVICEIDLENOTREQUIRED,
    

    /// Device has to be idle.
    ///
    /// "INSTALL_ONLY_WHEN_DEVICE_IDLE"
    #[serde(rename="INSTALL_ONLY_WHEN_DEVICE_IDLE")]
    INSTALLONLYWHENDEVICEIDLE,
}

impl AsRef<str> for InstallConstraintDeviceIdleConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstallConstraintDeviceIdleConstraintEnum::DEVICEIDLECONSTRAINTUNSPECIFIED => "DEVICE_IDLE_CONSTRAINT_UNSPECIFIED",
            InstallConstraintDeviceIdleConstraintEnum::DEVICEIDLENOTREQUIRED => "DEVICE_IDLE_NOT_REQUIRED",
            InstallConstraintDeviceIdleConstraintEnum::INSTALLONLYWHENDEVICEIDLE => "INSTALL_ONLY_WHEN_DEVICE_IDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for InstallConstraintDeviceIdleConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_IDLE_CONSTRAINT_UNSPECIFIED" => Ok(InstallConstraintDeviceIdleConstraintEnum::DEVICEIDLECONSTRAINTUNSPECIFIED),
           "DEVICE_IDLE_NOT_REQUIRED" => Ok(InstallConstraintDeviceIdleConstraintEnum::DEVICEIDLENOTREQUIRED),
           "INSTALL_ONLY_WHEN_DEVICE_IDLE" => Ok(InstallConstraintDeviceIdleConstraintEnum::INSTALLONLYWHENDEVICEIDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstallConstraintDeviceIdleConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstallConstraintNetworkTypeConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Network type constraint.
pub enum InstallConstraintNetworkTypeConstraintEnum {
    

    /// Unspecified. Default to INSTALL_ON_ANY_NETWORK.
    ///
    /// "NETWORK_TYPE_CONSTRAINT_UNSPECIFIED"
    #[serde(rename="NETWORK_TYPE_CONSTRAINT_UNSPECIFIED")]
    NETWORKTYPECONSTRAINTUNSPECIFIED,
    

    /// Any active networks (Wi-Fi, cellular, etc.).
    ///
    /// "INSTALL_ON_ANY_NETWORK"
    #[serde(rename="INSTALL_ON_ANY_NETWORK")]
    INSTALLONANYNETWORK,
    

    /// Any unmetered network (e.g. Wi-FI).
    ///
    /// "INSTALL_ONLY_ON_UNMETERED_NETWORK"
    #[serde(rename="INSTALL_ONLY_ON_UNMETERED_NETWORK")]
    INSTALLONLYONUNMETEREDNETWORK,
}

impl AsRef<str> for InstallConstraintNetworkTypeConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstallConstraintNetworkTypeConstraintEnum::NETWORKTYPECONSTRAINTUNSPECIFIED => "NETWORK_TYPE_CONSTRAINT_UNSPECIFIED",
            InstallConstraintNetworkTypeConstraintEnum::INSTALLONANYNETWORK => "INSTALL_ON_ANY_NETWORK",
            InstallConstraintNetworkTypeConstraintEnum::INSTALLONLYONUNMETEREDNETWORK => "INSTALL_ONLY_ON_UNMETERED_NETWORK",
        }
    }
}

impl std::convert::TryFrom< &str> for InstallConstraintNetworkTypeConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_TYPE_CONSTRAINT_UNSPECIFIED" => Ok(InstallConstraintNetworkTypeConstraintEnum::NETWORKTYPECONSTRAINTUNSPECIFIED),
           "INSTALL_ON_ANY_NETWORK" => Ok(InstallConstraintNetworkTypeConstraintEnum::INSTALLONANYNETWORK),
           "INSTALL_ONLY_ON_UNMETERED_NETWORK" => Ok(InstallConstraintNetworkTypeConstraintEnum::INSTALLONLYONUNMETEREDNETWORK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstallConstraintNetworkTypeConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KeyedAppStateSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the app state.
pub enum KeyedAppStateSeverityEnum {
    

    /// Unspecified severity level.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Information severity level.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Error severity level. This should only be set for genuine error conditions that a management organization needs to take action to fix.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for KeyedAppStateSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KeyedAppStateSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            KeyedAppStateSeverityEnum::INFO => "INFO",
            KeyedAppStateSeverityEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for KeyedAppStateSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(KeyedAppStateSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(KeyedAppStateSeverityEnum::INFO),
           "ERROR" => Ok(KeyedAppStateSeverityEnum::ERROR),
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


// region KioskCustomizationDeviceSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the Settings app is allowed in kiosk mode.
pub enum KioskCustomizationDeviceSettingsEnum {
    

    /// Unspecified, defaults to SETTINGS_ACCESS_ALLOWED.
    ///
    /// "DEVICE_SETTINGS_UNSPECIFIED"
    #[serde(rename="DEVICE_SETTINGS_UNSPECIFIED")]
    DEVICESETTINGSUNSPECIFIED,
    

    /// Access to the Settings app is allowed in kiosk mode.
    ///
    /// "SETTINGS_ACCESS_ALLOWED"
    #[serde(rename="SETTINGS_ACCESS_ALLOWED")]
    SETTINGSACCESSALLOWED,
    

    /// Access to the Settings app is not allowed in kiosk mode.
    ///
    /// "SETTINGS_ACCESS_BLOCKED"
    #[serde(rename="SETTINGS_ACCESS_BLOCKED")]
    SETTINGSACCESSBLOCKED,
}

impl AsRef<str> for KioskCustomizationDeviceSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KioskCustomizationDeviceSettingsEnum::DEVICESETTINGSUNSPECIFIED => "DEVICE_SETTINGS_UNSPECIFIED",
            KioskCustomizationDeviceSettingsEnum::SETTINGSACCESSALLOWED => "SETTINGS_ACCESS_ALLOWED",
            KioskCustomizationDeviceSettingsEnum::SETTINGSACCESSBLOCKED => "SETTINGS_ACCESS_BLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for KioskCustomizationDeviceSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_SETTINGS_UNSPECIFIED" => Ok(KioskCustomizationDeviceSettingsEnum::DEVICESETTINGSUNSPECIFIED),
           "SETTINGS_ACCESS_ALLOWED" => Ok(KioskCustomizationDeviceSettingsEnum::SETTINGSACCESSALLOWED),
           "SETTINGS_ACCESS_BLOCKED" => Ok(KioskCustomizationDeviceSettingsEnum::SETTINGSACCESSBLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KioskCustomizationDeviceSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KioskCustomizationPowerButtonActionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the behavior of a device in kiosk mode when a user presses and holds (long-presses) the Power button.
pub enum KioskCustomizationPowerButtonActionsEnum {
    

    /// Unspecified, defaults to POWER_BUTTON_AVAILABLE.
    ///
    /// "POWER_BUTTON_ACTIONS_UNSPECIFIED"
    #[serde(rename="POWER_BUTTON_ACTIONS_UNSPECIFIED")]
    POWERBUTTONACTIONSUNSPECIFIED,
    

    /// The power menu (e.g. Power off, Restart) is shown when a user long-presses the Power button of a device in kiosk mode.
    ///
    /// "POWER_BUTTON_AVAILABLE"
    #[serde(rename="POWER_BUTTON_AVAILABLE")]
    POWERBUTTONAVAILABLE,
    

    /// The power menu (e.g. Power off, Restart) is not shown when a user long-presses the Power button of a device in kiosk mode. Note: this may prevent users from turning off the device.
    ///
    /// "POWER_BUTTON_BLOCKED"
    #[serde(rename="POWER_BUTTON_BLOCKED")]
    POWERBUTTONBLOCKED,
}

impl AsRef<str> for KioskCustomizationPowerButtonActionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KioskCustomizationPowerButtonActionsEnum::POWERBUTTONACTIONSUNSPECIFIED => "POWER_BUTTON_ACTIONS_UNSPECIFIED",
            KioskCustomizationPowerButtonActionsEnum::POWERBUTTONAVAILABLE => "POWER_BUTTON_AVAILABLE",
            KioskCustomizationPowerButtonActionsEnum::POWERBUTTONBLOCKED => "POWER_BUTTON_BLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for KioskCustomizationPowerButtonActionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POWER_BUTTON_ACTIONS_UNSPECIFIED" => Ok(KioskCustomizationPowerButtonActionsEnum::POWERBUTTONACTIONSUNSPECIFIED),
           "POWER_BUTTON_AVAILABLE" => Ok(KioskCustomizationPowerButtonActionsEnum::POWERBUTTONAVAILABLE),
           "POWER_BUTTON_BLOCKED" => Ok(KioskCustomizationPowerButtonActionsEnum::POWERBUTTONBLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KioskCustomizationPowerButtonActionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KioskCustomizationStatusBarEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether system info and notifications are disabled in kiosk mode.
pub enum KioskCustomizationStatusBarEnum {
    

    /// Unspecified, defaults to INFO_AND_NOTIFICATIONS_DISABLED.
    ///
    /// "STATUS_BAR_UNSPECIFIED"
    #[serde(rename="STATUS_BAR_UNSPECIFIED")]
    STATUSBARUNSPECIFIED,
    

    /// System info and notifications are shown on the status bar in kiosk mode.Note: For this policy to take effect, the device's home button must be enabled using kioskCustomization.systemNavigation.
    ///
    /// "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED"
    #[serde(rename="NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED")]
    NOTIFICATIONSANDSYSTEMINFOENABLED,
    

    /// System info and notifications are disabled in kiosk mode.
    ///
    /// "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED"
    #[serde(rename="NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED")]
    NOTIFICATIONSANDSYSTEMINFODISABLED,
    

    /// Only system info is shown on the status bar.
    ///
    /// "SYSTEM_INFO_ONLY"
    #[serde(rename="SYSTEM_INFO_ONLY")]
    SYSTEMINFOONLY,
}

impl AsRef<str> for KioskCustomizationStatusBarEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KioskCustomizationStatusBarEnum::STATUSBARUNSPECIFIED => "STATUS_BAR_UNSPECIFIED",
            KioskCustomizationStatusBarEnum::NOTIFICATIONSANDSYSTEMINFOENABLED => "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED",
            KioskCustomizationStatusBarEnum::NOTIFICATIONSANDSYSTEMINFODISABLED => "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED",
            KioskCustomizationStatusBarEnum::SYSTEMINFOONLY => "SYSTEM_INFO_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for KioskCustomizationStatusBarEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_BAR_UNSPECIFIED" => Ok(KioskCustomizationStatusBarEnum::STATUSBARUNSPECIFIED),
           "NOTIFICATIONS_AND_SYSTEM_INFO_ENABLED" => Ok(KioskCustomizationStatusBarEnum::NOTIFICATIONSANDSYSTEMINFOENABLED),
           "NOTIFICATIONS_AND_SYSTEM_INFO_DISABLED" => Ok(KioskCustomizationStatusBarEnum::NOTIFICATIONSANDSYSTEMINFODISABLED),
           "SYSTEM_INFO_ONLY" => Ok(KioskCustomizationStatusBarEnum::SYSTEMINFOONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KioskCustomizationStatusBarEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KioskCustomizationSystemErrorWarningsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether system error dialogs for crashed or unresponsive apps are blocked in kiosk mode. When blocked, the system will force-stop the app as if the user chooses the "close app" option on the UI.
pub enum KioskCustomizationSystemErrorWarningsEnum {
    

    /// Unspecified, defaults to ERROR_AND_WARNINGS_MUTED.
    ///
    /// "SYSTEM_ERROR_WARNINGS_UNSPECIFIED"
    #[serde(rename="SYSTEM_ERROR_WARNINGS_UNSPECIFIED")]
    SYSTEMERRORWARNINGSUNSPECIFIED,
    

    /// All system error dialogs such as crash and app not responding (ANR) are displayed.
    ///
    /// "ERROR_AND_WARNINGS_ENABLED"
    #[serde(rename="ERROR_AND_WARNINGS_ENABLED")]
    ERRORANDWARNINGSENABLED,
    

    /// All system error dialogs, such as crash and app not responding (ANR) are blocked. When blocked, the system force-stops the app as if the user closes the app from the UI.
    ///
    /// "ERROR_AND_WARNINGS_MUTED"
    #[serde(rename="ERROR_AND_WARNINGS_MUTED")]
    ERRORANDWARNINGSMUTED,
}

impl AsRef<str> for KioskCustomizationSystemErrorWarningsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KioskCustomizationSystemErrorWarningsEnum::SYSTEMERRORWARNINGSUNSPECIFIED => "SYSTEM_ERROR_WARNINGS_UNSPECIFIED",
            KioskCustomizationSystemErrorWarningsEnum::ERRORANDWARNINGSENABLED => "ERROR_AND_WARNINGS_ENABLED",
            KioskCustomizationSystemErrorWarningsEnum::ERRORANDWARNINGSMUTED => "ERROR_AND_WARNINGS_MUTED",
        }
    }
}

impl std::convert::TryFrom< &str> for KioskCustomizationSystemErrorWarningsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYSTEM_ERROR_WARNINGS_UNSPECIFIED" => Ok(KioskCustomizationSystemErrorWarningsEnum::SYSTEMERRORWARNINGSUNSPECIFIED),
           "ERROR_AND_WARNINGS_ENABLED" => Ok(KioskCustomizationSystemErrorWarningsEnum::ERRORANDWARNINGSENABLED),
           "ERROR_AND_WARNINGS_MUTED" => Ok(KioskCustomizationSystemErrorWarningsEnum::ERRORANDWARNINGSMUTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KioskCustomizationSystemErrorWarningsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region KioskCustomizationSystemNavigationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which navigation features are enabled (e.g. Home, Overview buttons) in kiosk mode.
pub enum KioskCustomizationSystemNavigationEnum {
    

    /// Unspecified, defaults to NAVIGATION_DISABLED.
    ///
    /// "SYSTEM_NAVIGATION_UNSPECIFIED"
    #[serde(rename="SYSTEM_NAVIGATION_UNSPECIFIED")]
    SYSTEMNAVIGATIONUNSPECIFIED,
    

    /// Home and overview buttons are enabled.
    ///
    /// "NAVIGATION_ENABLED"
    #[serde(rename="NAVIGATION_ENABLED")]
    NAVIGATIONENABLED,
    

    /// The home and Overview buttons are not accessible.
    ///
    /// "NAVIGATION_DISABLED"
    #[serde(rename="NAVIGATION_DISABLED")]
    NAVIGATIONDISABLED,
    

    /// Only the home button is enabled.
    ///
    /// "HOME_BUTTON_ONLY"
    #[serde(rename="HOME_BUTTON_ONLY")]
    HOMEBUTTONONLY,
}

impl AsRef<str> for KioskCustomizationSystemNavigationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            KioskCustomizationSystemNavigationEnum::SYSTEMNAVIGATIONUNSPECIFIED => "SYSTEM_NAVIGATION_UNSPECIFIED",
            KioskCustomizationSystemNavigationEnum::NAVIGATIONENABLED => "NAVIGATION_ENABLED",
            KioskCustomizationSystemNavigationEnum::NAVIGATIONDISABLED => "NAVIGATION_DISABLED",
            KioskCustomizationSystemNavigationEnum::HOMEBUTTONONLY => "HOME_BUTTON_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for KioskCustomizationSystemNavigationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYSTEM_NAVIGATION_UNSPECIFIED" => Ok(KioskCustomizationSystemNavigationEnum::SYSTEMNAVIGATIONUNSPECIFIED),
           "NAVIGATION_ENABLED" => Ok(KioskCustomizationSystemNavigationEnum::NAVIGATIONENABLED),
           "NAVIGATION_DISABLED" => Ok(KioskCustomizationSystemNavigationEnum::NAVIGATIONDISABLED),
           "HOME_BUTTON_ONLY" => Ok(KioskCustomizationSystemNavigationEnum::HOMEBUTTONONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a KioskCustomizationSystemNavigationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedPropertyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the property.
pub enum ManagedPropertyTypeEnum {
    

    /// Not used.
    ///
    /// "MANAGED_PROPERTY_TYPE_UNSPECIFIED"
    #[serde(rename="MANAGED_PROPERTY_TYPE_UNSPECIFIED")]
    MANAGEDPROPERTYTYPEUNSPECIFIED,
    

    /// A property of boolean type.
    ///
    /// "BOOL"
    #[serde(rename="BOOL")]
    BOOL,
    

    /// A property of string type.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// A property of integer type.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// A choice of one item from a set.
    ///
    /// "CHOICE"
    #[serde(rename="CHOICE")]
    CHOICE,
    

    /// A choice of multiple items from a set.
    ///
    /// "MULTISELECT"
    #[serde(rename="MULTISELECT")]
    MULTISELECT,
    

    /// A hidden restriction of string type (the default value can be used to pass along information that can't be modified, such as a version code).
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// A bundle of properties
    ///
    /// "BUNDLE"
    #[serde(rename="BUNDLE")]
    BUNDLE,
    

    /// An array of property bundles.
    ///
    /// "BUNDLE_ARRAY"
    #[serde(rename="BUNDLE_ARRAY")]
    BUNDLEARRAY,
}

impl AsRef<str> for ManagedPropertyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedPropertyTypeEnum::MANAGEDPROPERTYTYPEUNSPECIFIED => "MANAGED_PROPERTY_TYPE_UNSPECIFIED",
            ManagedPropertyTypeEnum::BOOL => "BOOL",
            ManagedPropertyTypeEnum::STRING => "STRING",
            ManagedPropertyTypeEnum::INTEGER => "INTEGER",
            ManagedPropertyTypeEnum::CHOICE => "CHOICE",
            ManagedPropertyTypeEnum::MULTISELECT => "MULTISELECT",
            ManagedPropertyTypeEnum::HIDDEN => "HIDDEN",
            ManagedPropertyTypeEnum::BUNDLE => "BUNDLE",
            ManagedPropertyTypeEnum::BUNDLEARRAY => "BUNDLE_ARRAY",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedPropertyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGED_PROPERTY_TYPE_UNSPECIFIED" => Ok(ManagedPropertyTypeEnum::MANAGEDPROPERTYTYPEUNSPECIFIED),
           "BOOL" => Ok(ManagedPropertyTypeEnum::BOOL),
           "STRING" => Ok(ManagedPropertyTypeEnum::STRING),
           "INTEGER" => Ok(ManagedPropertyTypeEnum::INTEGER),
           "CHOICE" => Ok(ManagedPropertyTypeEnum::CHOICE),
           "MULTISELECT" => Ok(ManagedPropertyTypeEnum::MULTISELECT),
           "HIDDEN" => Ok(ManagedPropertyTypeEnum::HIDDEN),
           "BUNDLE" => Ok(ManagedPropertyTypeEnum::BUNDLE),
           "BUNDLE_ARRAY" => Ok(ManagedPropertyTypeEnum::BUNDLEARRAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedPropertyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MemoryEventEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event type.
pub enum MemoryEventEventTypeEnum {
    

    /// Unspecified. No events have this type.
    ///
    /// "MEMORY_EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="MEMORY_EVENT_TYPE_UNSPECIFIED")]
    MEMORYEVENTTYPEUNSPECIFIED,
    

    /// Free space in RAM was measured.
    ///
    /// "RAM_MEASURED"
    #[serde(rename="RAM_MEASURED")]
    RAMMEASURED,
    

    /// Free space in internal storage was measured.
    ///
    /// "INTERNAL_STORAGE_MEASURED"
    #[serde(rename="INTERNAL_STORAGE_MEASURED")]
    INTERNALSTORAGEMEASURED,
    

    /// A new external storage medium was detected. The reported byte count is the total capacity of the storage medium.
    ///
    /// "EXTERNAL_STORAGE_DETECTED"
    #[serde(rename="EXTERNAL_STORAGE_DETECTED")]
    EXTERNALSTORAGEDETECTED,
    

    /// An external storage medium was removed. The reported byte count is zero.
    ///
    /// "EXTERNAL_STORAGE_REMOVED"
    #[serde(rename="EXTERNAL_STORAGE_REMOVED")]
    EXTERNALSTORAGEREMOVED,
    

    /// Free space in an external storage medium was measured.
    ///
    /// "EXTERNAL_STORAGE_MEASURED"
    #[serde(rename="EXTERNAL_STORAGE_MEASURED")]
    EXTERNALSTORAGEMEASURED,
}

impl AsRef<str> for MemoryEventEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MemoryEventEventTypeEnum::MEMORYEVENTTYPEUNSPECIFIED => "MEMORY_EVENT_TYPE_UNSPECIFIED",
            MemoryEventEventTypeEnum::RAMMEASURED => "RAM_MEASURED",
            MemoryEventEventTypeEnum::INTERNALSTORAGEMEASURED => "INTERNAL_STORAGE_MEASURED",
            MemoryEventEventTypeEnum::EXTERNALSTORAGEDETECTED => "EXTERNAL_STORAGE_DETECTED",
            MemoryEventEventTypeEnum::EXTERNALSTORAGEREMOVED => "EXTERNAL_STORAGE_REMOVED",
            MemoryEventEventTypeEnum::EXTERNALSTORAGEMEASURED => "EXTERNAL_STORAGE_MEASURED",
        }
    }
}

impl std::convert::TryFrom< &str> for MemoryEventEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEMORY_EVENT_TYPE_UNSPECIFIED" => Ok(MemoryEventEventTypeEnum::MEMORYEVENTTYPEUNSPECIFIED),
           "RAM_MEASURED" => Ok(MemoryEventEventTypeEnum::RAMMEASURED),
           "INTERNAL_STORAGE_MEASURED" => Ok(MemoryEventEventTypeEnum::INTERNALSTORAGEMEASURED),
           "EXTERNAL_STORAGE_DETECTED" => Ok(MemoryEventEventTypeEnum::EXTERNALSTORAGEDETECTED),
           "EXTERNAL_STORAGE_REMOVED" => Ok(MemoryEventEventTypeEnum::EXTERNALSTORAGEREMOVED),
           "EXTERNAL_STORAGE_MEASURED" => Ok(MemoryEventEventTypeEnum::EXTERNALSTORAGEMEASURED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MemoryEventEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrationTokenManagementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The management mode of the device or profile being migrated.
pub enum MigrationTokenManagementModeEnum {
    

    /// This value must not be used.
    ///
    /// "MANAGEMENT_MODE_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_MODE_UNSPECIFIED")]
    MANAGEMENTMODEUNSPECIFIED,
    

    /// A work profile on a personally owned device. Supported only on devices running Android 9 and above.
    ///
    /// "WORK_PROFILE_PERSONALLY_OWNED"
    #[serde(rename="WORK_PROFILE_PERSONALLY_OWNED")]
    WORKPROFILEPERSONALLYOWNED,
    

    /// A work profile on a company-owned device. Supported only on devices running Android 11 and above.
    ///
    /// "WORK_PROFILE_COMPANY_OWNED"
    #[serde(rename="WORK_PROFILE_COMPANY_OWNED")]
    WORKPROFILECOMPANYOWNED,
    

    /// A fully-managed device. Supported only on devices running Android 9 and above.
    ///
    /// "FULLY_MANAGED"
    #[serde(rename="FULLY_MANAGED")]
    FULLYMANAGED,
}

impl AsRef<str> for MigrationTokenManagementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrationTokenManagementModeEnum::MANAGEMENTMODEUNSPECIFIED => "MANAGEMENT_MODE_UNSPECIFIED",
            MigrationTokenManagementModeEnum::WORKPROFILEPERSONALLYOWNED => "WORK_PROFILE_PERSONALLY_OWNED",
            MigrationTokenManagementModeEnum::WORKPROFILECOMPANYOWNED => "WORK_PROFILE_COMPANY_OWNED",
            MigrationTokenManagementModeEnum::FULLYMANAGED => "FULLY_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrationTokenManagementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_MODE_UNSPECIFIED" => Ok(MigrationTokenManagementModeEnum::MANAGEMENTMODEUNSPECIFIED),
           "WORK_PROFILE_PERSONALLY_OWNED" => Ok(MigrationTokenManagementModeEnum::WORKPROFILEPERSONALLYOWNED),
           "WORK_PROFILE_COMPANY_OWNED" => Ok(MigrationTokenManagementModeEnum::WORKPROFILECOMPANYOWNED),
           "FULLY_MANAGED" => Ok(MigrationTokenManagementModeEnum::FULLYMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrationTokenManagementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NonComplianceDetailInstallationFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If package_name is set and the non-compliance reason is APP_NOT_INSTALLED or APP_NOT_UPDATED, the detailed reason the app can't be installed or updated.
pub enum NonComplianceDetailInstallationFailureReasonEnum {
    

    /// This value is disallowed.
    ///
    /// "INSTALLATION_FAILURE_REASON_UNSPECIFIED"
    #[serde(rename="INSTALLATION_FAILURE_REASON_UNSPECIFIED")]
    INSTALLATIONFAILUREREASONUNSPECIFIED,
    

    /// An unknown condition is preventing the app from being installed. Some potential reasons are that the device doesn't have enough storage, the device network connection is unreliable, or the installation is taking longer than expected. The installation will be retried automatically.
    ///
    /// "INSTALLATION_FAILURE_REASON_UNKNOWN"
    #[serde(rename="INSTALLATION_FAILURE_REASON_UNKNOWN")]
    INSTALLATIONFAILUREREASONUNKNOWN,
    

    /// The installation is still in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The app was not found in Play.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// The app is incompatible with the device.
    ///
    /// "NOT_COMPATIBLE_WITH_DEVICE"
    #[serde(rename="NOT_COMPATIBLE_WITH_DEVICE")]
    NOTCOMPATIBLEWITHDEVICE,
    

    /// The app has not been approved by the admin.
    ///
    /// "NOT_APPROVED"
    #[serde(rename="NOT_APPROVED")]
    NOTAPPROVED,
    

    /// The app has new permissions that have not been accepted by the admin.
    ///
    /// "PERMISSIONS_NOT_ACCEPTED"
    #[serde(rename="PERMISSIONS_NOT_ACCEPTED")]
    PERMISSIONSNOTACCEPTED,
    

    /// The app is not available in the user's country.
    ///
    /// "NOT_AVAILABLE_IN_COUNTRY"
    #[serde(rename="NOT_AVAILABLE_IN_COUNTRY")]
    NOTAVAILABLEINCOUNTRY,
    

    /// There are no licenses available to assign to the user.
    ///
    /// "NO_LICENSES_REMAINING"
    #[serde(rename="NO_LICENSES_REMAINING")]
    NOLICENSESREMAINING,
    

    /// The enterprise is no longer enrolled with Managed Google Play or the admin has not accepted the latest Managed Google Play Terms of Service.
    ///
    /// "NOT_ENROLLED"
    #[serde(rename="NOT_ENROLLED")]
    NOTENROLLED,
    

    /// The user is no longer valid. The user may have been deleted or disabled.
    ///
    /// "USER_INVALID"
    #[serde(rename="USER_INVALID")]
    USERINVALID,
    

    /// A network error on the user's device has prevented the install from succeeding. This usually happens when the device's internet connectivity is degraded, unavailable or there's a network configuration issue. Please ensure the device has access to full internet connectivity on a network that meets Android Enterprise Network Requirements (https://support.google.com/work/android/answer/10513641). App install or update will automatically resume once this is the case.
    ///
    /// "NETWORK_ERROR_UNRELIABLE_CONNECTION"
    #[serde(rename="NETWORK_ERROR_UNRELIABLE_CONNECTION")]
    NETWORKERRORUNRELIABLECONNECTION,
    

    /// The user's device does not have sufficient storage space to install the app. This can be resolved by clearing up storage space on the device. App install or update will automatically resume once the device has sufficient storage.
    ///
    /// "INSUFFICIENT_STORAGE"
    #[serde(rename="INSUFFICIENT_STORAGE")]
    INSUFFICIENTSTORAGE,
}

impl AsRef<str> for NonComplianceDetailInstallationFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NonComplianceDetailInstallationFailureReasonEnum::INSTALLATIONFAILUREREASONUNSPECIFIED => "INSTALLATION_FAILURE_REASON_UNSPECIFIED",
            NonComplianceDetailInstallationFailureReasonEnum::INSTALLATIONFAILUREREASONUNKNOWN => "INSTALLATION_FAILURE_REASON_UNKNOWN",
            NonComplianceDetailInstallationFailureReasonEnum::INPROGRESS => "IN_PROGRESS",
            NonComplianceDetailInstallationFailureReasonEnum::NOTFOUND => "NOT_FOUND",
            NonComplianceDetailInstallationFailureReasonEnum::NOTCOMPATIBLEWITHDEVICE => "NOT_COMPATIBLE_WITH_DEVICE",
            NonComplianceDetailInstallationFailureReasonEnum::NOTAPPROVED => "NOT_APPROVED",
            NonComplianceDetailInstallationFailureReasonEnum::PERMISSIONSNOTACCEPTED => "PERMISSIONS_NOT_ACCEPTED",
            NonComplianceDetailInstallationFailureReasonEnum::NOTAVAILABLEINCOUNTRY => "NOT_AVAILABLE_IN_COUNTRY",
            NonComplianceDetailInstallationFailureReasonEnum::NOLICENSESREMAINING => "NO_LICENSES_REMAINING",
            NonComplianceDetailInstallationFailureReasonEnum::NOTENROLLED => "NOT_ENROLLED",
            NonComplianceDetailInstallationFailureReasonEnum::USERINVALID => "USER_INVALID",
            NonComplianceDetailInstallationFailureReasonEnum::NETWORKERRORUNRELIABLECONNECTION => "NETWORK_ERROR_UNRELIABLE_CONNECTION",
            NonComplianceDetailInstallationFailureReasonEnum::INSUFFICIENTSTORAGE => "INSUFFICIENT_STORAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for NonComplianceDetailInstallationFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTALLATION_FAILURE_REASON_UNSPECIFIED" => Ok(NonComplianceDetailInstallationFailureReasonEnum::INSTALLATIONFAILUREREASONUNSPECIFIED),
           "INSTALLATION_FAILURE_REASON_UNKNOWN" => Ok(NonComplianceDetailInstallationFailureReasonEnum::INSTALLATIONFAILUREREASONUNKNOWN),
           "IN_PROGRESS" => Ok(NonComplianceDetailInstallationFailureReasonEnum::INPROGRESS),
           "NOT_FOUND" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NOTFOUND),
           "NOT_COMPATIBLE_WITH_DEVICE" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NOTCOMPATIBLEWITHDEVICE),
           "NOT_APPROVED" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NOTAPPROVED),
           "PERMISSIONS_NOT_ACCEPTED" => Ok(NonComplianceDetailInstallationFailureReasonEnum::PERMISSIONSNOTACCEPTED),
           "NOT_AVAILABLE_IN_COUNTRY" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NOTAVAILABLEINCOUNTRY),
           "NO_LICENSES_REMAINING" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NOLICENSESREMAINING),
           "NOT_ENROLLED" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NOTENROLLED),
           "USER_INVALID" => Ok(NonComplianceDetailInstallationFailureReasonEnum::USERINVALID),
           "NETWORK_ERROR_UNRELIABLE_CONNECTION" => Ok(NonComplianceDetailInstallationFailureReasonEnum::NETWORKERRORUNRELIABLECONNECTION),
           "INSUFFICIENT_STORAGE" => Ok(NonComplianceDetailInstallationFailureReasonEnum::INSUFFICIENTSTORAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NonComplianceDetailInstallationFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NonComplianceDetailNonComplianceReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason the device is not in compliance with the setting.
pub enum NonComplianceDetailNonComplianceReasonEnum {
    

    /// This value is disallowed.
    ///
    /// "NON_COMPLIANCE_REASON_UNSPECIFIED"
    #[serde(rename="NON_COMPLIANCE_REASON_UNSPECIFIED")]
    NONCOMPLIANCEREASONUNSPECIFIED,
    

    /// The setting is not supported in the API level of the Android version running on the device.
    ///
    /// "API_LEVEL"
    #[serde(rename="API_LEVEL")]
    APILEVEL,
    

    /// The management mode (profile owner, device owner, etc.) doesn't support the setting.
    ///
    /// "MANAGEMENT_MODE"
    #[serde(rename="MANAGEMENT_MODE")]
    MANAGEMENTMODE,
    

    /// The user has not taken required action to comply with the setting.
    ///
    /// "USER_ACTION"
    #[serde(rename="USER_ACTION")]
    USERACTION,
    

    /// The setting has an invalid value.
    ///
    /// "INVALID_VALUE"
    #[serde(rename="INVALID_VALUE")]
    INVALIDVALUE,
    

    /// The app required to implement the policy is not installed.
    ///
    /// "APP_NOT_INSTALLED"
    #[serde(rename="APP_NOT_INSTALLED")]
    APPNOTINSTALLED,
    

    /// The policy is not supported by the version of Android Device Policy on the device.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// A blocked app is installed.
    ///
    /// "APP_INSTALLED"
    #[serde(rename="APP_INSTALLED")]
    APPINSTALLED,
    

    /// The setting hasn't been applied at the time of the report, but is expected to be applied shortly.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough.
    ///
    /// "APP_INCOMPATIBLE"
    #[serde(rename="APP_INCOMPATIBLE")]
    APPINCOMPATIBLE,
    

    /// The app is installed, but it hasn't been updated to the minimum version code specified by policy.
    ///
    /// "APP_NOT_UPDATED"
    #[serde(rename="APP_NOT_UPDATED")]
    APPNOTUPDATED,
    

    /// The device is incompatible with the policy requirements.
    ///
    /// "DEVICE_INCOMPATIBLE"
    #[serde(rename="DEVICE_INCOMPATIBLE")]
    DEVICEINCOMPATIBLE,
}

impl AsRef<str> for NonComplianceDetailNonComplianceReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NonComplianceDetailNonComplianceReasonEnum::NONCOMPLIANCEREASONUNSPECIFIED => "NON_COMPLIANCE_REASON_UNSPECIFIED",
            NonComplianceDetailNonComplianceReasonEnum::APILEVEL => "API_LEVEL",
            NonComplianceDetailNonComplianceReasonEnum::MANAGEMENTMODE => "MANAGEMENT_MODE",
            NonComplianceDetailNonComplianceReasonEnum::USERACTION => "USER_ACTION",
            NonComplianceDetailNonComplianceReasonEnum::INVALIDVALUE => "INVALID_VALUE",
            NonComplianceDetailNonComplianceReasonEnum::APPNOTINSTALLED => "APP_NOT_INSTALLED",
            NonComplianceDetailNonComplianceReasonEnum::UNSUPPORTED => "UNSUPPORTED",
            NonComplianceDetailNonComplianceReasonEnum::APPINSTALLED => "APP_INSTALLED",
            NonComplianceDetailNonComplianceReasonEnum::PENDING => "PENDING",
            NonComplianceDetailNonComplianceReasonEnum::APPINCOMPATIBLE => "APP_INCOMPATIBLE",
            NonComplianceDetailNonComplianceReasonEnum::APPNOTUPDATED => "APP_NOT_UPDATED",
            NonComplianceDetailNonComplianceReasonEnum::DEVICEINCOMPATIBLE => "DEVICE_INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for NonComplianceDetailNonComplianceReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NON_COMPLIANCE_REASON_UNSPECIFIED" => Ok(NonComplianceDetailNonComplianceReasonEnum::NONCOMPLIANCEREASONUNSPECIFIED),
           "API_LEVEL" => Ok(NonComplianceDetailNonComplianceReasonEnum::APILEVEL),
           "MANAGEMENT_MODE" => Ok(NonComplianceDetailNonComplianceReasonEnum::MANAGEMENTMODE),
           "USER_ACTION" => Ok(NonComplianceDetailNonComplianceReasonEnum::USERACTION),
           "INVALID_VALUE" => Ok(NonComplianceDetailNonComplianceReasonEnum::INVALIDVALUE),
           "APP_NOT_INSTALLED" => Ok(NonComplianceDetailNonComplianceReasonEnum::APPNOTINSTALLED),
           "UNSUPPORTED" => Ok(NonComplianceDetailNonComplianceReasonEnum::UNSUPPORTED),
           "APP_INSTALLED" => Ok(NonComplianceDetailNonComplianceReasonEnum::APPINSTALLED),
           "PENDING" => Ok(NonComplianceDetailNonComplianceReasonEnum::PENDING),
           "APP_INCOMPATIBLE" => Ok(NonComplianceDetailNonComplianceReasonEnum::APPINCOMPATIBLE),
           "APP_NOT_UPDATED" => Ok(NonComplianceDetailNonComplianceReasonEnum::APPNOTUPDATED),
           "DEVICE_INCOMPATIBLE" => Ok(NonComplianceDetailNonComplianceReasonEnum::DEVICEINCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NonComplianceDetailNonComplianceReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NonComplianceDetailSpecificNonComplianceReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The policy-specific reason the device is not in compliance with the setting.
pub enum NonComplianceDetailSpecificNonComplianceReasonEnum {
    

    /// Specific non-compliance reason is not specified. Fields in specific_non_compliance_context are not set.
    ///
    /// "SPECIFIC_NON_COMPLIANCE_REASON_UNSPECIFIED"
    #[serde(rename="SPECIFIC_NON_COMPLIANCE_REASON_UNSPECIFIED")]
    SPECIFICNONCOMPLIANCEREASONUNSPECIFIED,
    

    /// User needs to confirm credentials by entering the screen lock. Fields in specific_non_compliance_context are not set. nonComplianceReason is set to USER_ACTION.
    ///
    /// "PASSWORD_POLICIES_USER_CREDENTIALS_CONFIRMATION_REQUIRED"
    #[serde(rename="PASSWORD_POLICIES_USER_CREDENTIALS_CONFIRMATION_REQUIRED")]
    PASSWORDPOLICIESUSERCREDENTIALSCONFIRMATIONREQUIRED,
    

    /// The device or profile password has expired. passwordPoliciesContext is set. nonComplianceReason is set to USER_ACTION.
    ///
    /// "PASSWORD_POLICIES_PASSWORD_EXPIRED"
    #[serde(rename="PASSWORD_POLICIES_PASSWORD_EXPIRED")]
    PASSWORDPOLICIESPASSWORDEXPIRED,
    

    /// The device password does not satisfy password requirements. passwordPoliciesContext is set. nonComplianceReason is set to USER_ACTION.
    ///
    /// "PASSWORD_POLICIES_PASSWORD_NOT_SUFFICIENT"
    #[serde(rename="PASSWORD_POLICIES_PASSWORD_NOT_SUFFICIENT")]
    PASSWORDPOLICIESPASSWORDNOTSUFFICIENT,
    

    /// There is an incorrect value in ONC Wi-Fi configuration. fieldPath specifies which field value is incorrect. oncWifiContext is set. nonComplianceReason is set to INVALID_VALUE.
    ///
    /// "ONC_WIFI_INVALID_VALUE"
    #[serde(rename="ONC_WIFI_INVALID_VALUE")]
    ONCWIFIINVALIDVALUE,
    

    /// The ONC Wi-Fi setting is not supported in the API level of the Android version running on the device. fieldPath specifies which field value is not supported. oncWifiContext is set. nonComplianceReason is set to API_LEVEL.
    ///
    /// "ONC_WIFI_API_LEVEL"
    #[serde(rename="ONC_WIFI_API_LEVEL")]
    ONCWIFIAPILEVEL,
    

    /// The enterprise Wi-Fi network is missing either the root CA or domain name. nonComplianceReason is set to INVALID_VALUE.
    ///
    /// "ONC_WIFI_INVALID_ENTERPRISE_CONFIG"
    #[serde(rename="ONC_WIFI_INVALID_ENTERPRISE_CONFIG")]
    ONCWIFIINVALIDENTERPRISECONFIG,
    

    /// User needs to remove the configured Wi-Fi network manually. This is applicable only on work profiles on personally-owned devices. nonComplianceReason is set to USER_ACTION.
    ///
    /// "ONC_WIFI_USER_SHOULD_REMOVE_NETWORK"
    #[serde(rename="ONC_WIFI_USER_SHOULD_REMOVE_NETWORK")]
    ONCWIFIUSERSHOULDREMOVENETWORK,
    

    /// Key pair alias specified via ClientCertKeyPairAlias (https://chromium.googlesource.com/chromium/src/+/main/components/onc/docs/onc_spec.md#eap-type) field in openNetworkConfiguration does not correspond to an existing key installed on the device. nonComplianceReason is set to INVALID_VALUE.
    ///
    /// "ONC_WIFI_KEY_PAIR_ALIAS_NOT_CORRESPONDING_TO_EXISTING_KEY"
    #[serde(rename="ONC_WIFI_KEY_PAIR_ALIAS_NOT_CORRESPONDING_TO_EXISTING_KEY")]
    ONCWIFIKEYPAIRALIASNOTCORRESPONDINGTOEXISTINGKEY,
}

impl AsRef<str> for NonComplianceDetailSpecificNonComplianceReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NonComplianceDetailSpecificNonComplianceReasonEnum::SPECIFICNONCOMPLIANCEREASONUNSPECIFIED => "SPECIFIC_NON_COMPLIANCE_REASON_UNSPECIFIED",
            NonComplianceDetailSpecificNonComplianceReasonEnum::PASSWORDPOLICIESUSERCREDENTIALSCONFIRMATIONREQUIRED => "PASSWORD_POLICIES_USER_CREDENTIALS_CONFIRMATION_REQUIRED",
            NonComplianceDetailSpecificNonComplianceReasonEnum::PASSWORDPOLICIESPASSWORDEXPIRED => "PASSWORD_POLICIES_PASSWORD_EXPIRED",
            NonComplianceDetailSpecificNonComplianceReasonEnum::PASSWORDPOLICIESPASSWORDNOTSUFFICIENT => "PASSWORD_POLICIES_PASSWORD_NOT_SUFFICIENT",
            NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIINVALIDVALUE => "ONC_WIFI_INVALID_VALUE",
            NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIAPILEVEL => "ONC_WIFI_API_LEVEL",
            NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIINVALIDENTERPRISECONFIG => "ONC_WIFI_INVALID_ENTERPRISE_CONFIG",
            NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIUSERSHOULDREMOVENETWORK => "ONC_WIFI_USER_SHOULD_REMOVE_NETWORK",
            NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIKEYPAIRALIASNOTCORRESPONDINGTOEXISTINGKEY => "ONC_WIFI_KEY_PAIR_ALIAS_NOT_CORRESPONDING_TO_EXISTING_KEY",
        }
    }
}

impl std::convert::TryFrom< &str> for NonComplianceDetailSpecificNonComplianceReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPECIFIC_NON_COMPLIANCE_REASON_UNSPECIFIED" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::SPECIFICNONCOMPLIANCEREASONUNSPECIFIED),
           "PASSWORD_POLICIES_USER_CREDENTIALS_CONFIRMATION_REQUIRED" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::PASSWORDPOLICIESUSERCREDENTIALSCONFIRMATIONREQUIRED),
           "PASSWORD_POLICIES_PASSWORD_EXPIRED" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::PASSWORDPOLICIESPASSWORDEXPIRED),
           "PASSWORD_POLICIES_PASSWORD_NOT_SUFFICIENT" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::PASSWORDPOLICIESPASSWORDNOTSUFFICIENT),
           "ONC_WIFI_INVALID_VALUE" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIINVALIDVALUE),
           "ONC_WIFI_API_LEVEL" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIAPILEVEL),
           "ONC_WIFI_INVALID_ENTERPRISE_CONFIG" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIINVALIDENTERPRISECONFIG),
           "ONC_WIFI_USER_SHOULD_REMOVE_NETWORK" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIUSERSHOULDREMOVENETWORK),
           "ONC_WIFI_KEY_PAIR_ALIAS_NOT_CORRESPONDING_TO_EXISTING_KEY" => Ok(NonComplianceDetailSpecificNonComplianceReasonEnum::ONCWIFIKEYPAIRALIASNOTCORRESPONDINGTOEXISTINGKEY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NonComplianceDetailSpecificNonComplianceReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NonComplianceDetailConditionNonComplianceReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason the device is not in compliance with the setting. If not set, then this condition matches any reason.
pub enum NonComplianceDetailConditionNonComplianceReasonEnum {
    

    /// This value is disallowed.
    ///
    /// "NON_COMPLIANCE_REASON_UNSPECIFIED"
    #[serde(rename="NON_COMPLIANCE_REASON_UNSPECIFIED")]
    NONCOMPLIANCEREASONUNSPECIFIED,
    

    /// The setting is not supported in the API level of the Android version running on the device.
    ///
    /// "API_LEVEL"
    #[serde(rename="API_LEVEL")]
    APILEVEL,
    

    /// The management mode (profile owner, device owner, etc.) doesn't support the setting.
    ///
    /// "MANAGEMENT_MODE"
    #[serde(rename="MANAGEMENT_MODE")]
    MANAGEMENTMODE,
    

    /// The user has not taken required action to comply with the setting.
    ///
    /// "USER_ACTION"
    #[serde(rename="USER_ACTION")]
    USERACTION,
    

    /// The setting has an invalid value.
    ///
    /// "INVALID_VALUE"
    #[serde(rename="INVALID_VALUE")]
    INVALIDVALUE,
    

    /// The app required to implement the policy is not installed.
    ///
    /// "APP_NOT_INSTALLED"
    #[serde(rename="APP_NOT_INSTALLED")]
    APPNOTINSTALLED,
    

    /// The policy is not supported by the version of Android Device Policy on the device.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
    

    /// A blocked app is installed.
    ///
    /// "APP_INSTALLED"
    #[serde(rename="APP_INSTALLED")]
    APPINSTALLED,
    

    /// The setting hasn't been applied at the time of the report, but is expected to be applied shortly.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The setting can't be applied to the app because the app doesn't support it, for example because its target SDK version is not high enough.
    ///
    /// "APP_INCOMPATIBLE"
    #[serde(rename="APP_INCOMPATIBLE")]
    APPINCOMPATIBLE,
    

    /// The app is installed, but it hasn't been updated to the minimum version code specified by policy.
    ///
    /// "APP_NOT_UPDATED"
    #[serde(rename="APP_NOT_UPDATED")]
    APPNOTUPDATED,
    

    /// The device is incompatible with the policy requirements.
    ///
    /// "DEVICE_INCOMPATIBLE"
    #[serde(rename="DEVICE_INCOMPATIBLE")]
    DEVICEINCOMPATIBLE,
}

impl AsRef<str> for NonComplianceDetailConditionNonComplianceReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NonComplianceDetailConditionNonComplianceReasonEnum::NONCOMPLIANCEREASONUNSPECIFIED => "NON_COMPLIANCE_REASON_UNSPECIFIED",
            NonComplianceDetailConditionNonComplianceReasonEnum::APILEVEL => "API_LEVEL",
            NonComplianceDetailConditionNonComplianceReasonEnum::MANAGEMENTMODE => "MANAGEMENT_MODE",
            NonComplianceDetailConditionNonComplianceReasonEnum::USERACTION => "USER_ACTION",
            NonComplianceDetailConditionNonComplianceReasonEnum::INVALIDVALUE => "INVALID_VALUE",
            NonComplianceDetailConditionNonComplianceReasonEnum::APPNOTINSTALLED => "APP_NOT_INSTALLED",
            NonComplianceDetailConditionNonComplianceReasonEnum::UNSUPPORTED => "UNSUPPORTED",
            NonComplianceDetailConditionNonComplianceReasonEnum::APPINSTALLED => "APP_INSTALLED",
            NonComplianceDetailConditionNonComplianceReasonEnum::PENDING => "PENDING",
            NonComplianceDetailConditionNonComplianceReasonEnum::APPINCOMPATIBLE => "APP_INCOMPATIBLE",
            NonComplianceDetailConditionNonComplianceReasonEnum::APPNOTUPDATED => "APP_NOT_UPDATED",
            NonComplianceDetailConditionNonComplianceReasonEnum::DEVICEINCOMPATIBLE => "DEVICE_INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for NonComplianceDetailConditionNonComplianceReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NON_COMPLIANCE_REASON_UNSPECIFIED" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::NONCOMPLIANCEREASONUNSPECIFIED),
           "API_LEVEL" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::APILEVEL),
           "MANAGEMENT_MODE" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::MANAGEMENTMODE),
           "USER_ACTION" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::USERACTION),
           "INVALID_VALUE" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::INVALIDVALUE),
           "APP_NOT_INSTALLED" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::APPNOTINSTALLED),
           "UNSUPPORTED" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::UNSUPPORTED),
           "APP_INSTALLED" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::APPINSTALLED),
           "PENDING" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::PENDING),
           "APP_INCOMPATIBLE" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::APPINCOMPATIBLE),
           "APP_NOT_UPDATED" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::APPNOTUPDATED),
           "DEVICE_INCOMPATIBLE" => Ok(NonComplianceDetailConditionNonComplianceReasonEnum::DEVICEINCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NonComplianceDetailConditionNonComplianceReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasswordPoliciesContextPasswordPolicyScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope of non-compliant password.
pub enum PasswordPoliciesContextPasswordPolicyScopeEnum {
    

    /// The scope is unspecified. The password requirements are applied to the work profile for work profile devices and the whole device for fully managed or dedicated devices.
    ///
    /// "SCOPE_UNSPECIFIED"
    #[serde(rename="SCOPE_UNSPECIFIED")]
    SCOPEUNSPECIFIED,
    

    /// The password requirements are only applied to the device.
    ///
    /// "SCOPE_DEVICE"
    #[serde(rename="SCOPE_DEVICE")]
    SCOPEDEVICE,
    

    /// The password requirements are only applied to the work profile.
    ///
    /// "SCOPE_PROFILE"
    #[serde(rename="SCOPE_PROFILE")]
    SCOPEPROFILE,
}

impl AsRef<str> for PasswordPoliciesContextPasswordPolicyScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasswordPoliciesContextPasswordPolicyScopeEnum::SCOPEUNSPECIFIED => "SCOPE_UNSPECIFIED",
            PasswordPoliciesContextPasswordPolicyScopeEnum::SCOPEDEVICE => "SCOPE_DEVICE",
            PasswordPoliciesContextPasswordPolicyScopeEnum::SCOPEPROFILE => "SCOPE_PROFILE",
        }
    }
}

impl std::convert::TryFrom< &str> for PasswordPoliciesContextPasswordPolicyScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCOPE_UNSPECIFIED" => Ok(PasswordPoliciesContextPasswordPolicyScopeEnum::SCOPEUNSPECIFIED),
           "SCOPE_DEVICE" => Ok(PasswordPoliciesContextPasswordPolicyScopeEnum::SCOPEDEVICE),
           "SCOPE_PROFILE" => Ok(PasswordPoliciesContextPasswordPolicyScopeEnum::SCOPEPROFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasswordPoliciesContextPasswordPolicyScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasswordRequirementPasswordQualityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The required password quality.
pub enum PasswordRequirementPasswordQualityEnum {
    

    /// There are no password requirements.
    ///
    /// "PASSWORD_QUALITY_UNSPECIFIED"
    #[serde(rename="PASSWORD_QUALITY_UNSPECIFIED")]
    PASSWORDQUALITYUNSPECIFIED,
    

    /// The device must be secured with a low-security biometric recognition technology, at minimum. This includes technologies that can recognize the identity of an individual that are roughly equivalent to a 3-digit PIN (false detection is less than 1 in 1,000).This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_LOW for application. See PasswordQuality for details.
    ///
    /// "BIOMETRIC_WEAK"
    #[serde(rename="BIOMETRIC_WEAK")]
    BIOMETRICWEAK,
    

    /// A password is required, but there are no restrictions on what the password must contain.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_LOW for application. See PasswordQuality for details.
    ///
    /// "SOMETHING"
    #[serde(rename="SOMETHING")]
    SOMETHING,
    

    /// The password must contain numeric characters.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_MEDIUM for application. See PasswordQuality for details.
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// The password must contain numeric characters with no repeating (4444) or ordered (1234, 4321, 2468) sequences.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_MEDIUM for application. See PasswordQuality for details.
    ///
    /// "NUMERIC_COMPLEX"
    #[serde(rename="NUMERIC_COMPLEX")]
    NUMERICCOMPLEX,
    

    /// The password must contain alphabetic (or symbol) characters.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_HIGH for application. See PasswordQuality for details.
    ///
    /// "ALPHABETIC"
    #[serde(rename="ALPHABETIC")]
    ALPHABETIC,
    

    /// The password must contain both numeric and alphabetic (or symbol) characters.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_HIGH for application. See PasswordQuality for details.
    ///
    /// "ALPHANUMERIC"
    #[serde(rename="ALPHANUMERIC")]
    ALPHANUMERIC,
    

    /// The password must meet the minimum requirements specified in passwordMinimumLength, passwordMinimumLetters, passwordMinimumSymbols, etc. For example, if passwordMinimumSymbols is 2, the password must contain at least two symbols.This, when applied on personally owned work profile devices on Android 12 device-scoped, will be treated as COMPLEXITY_HIGH for application. In this case, the requirements in passwordMinimumLength, passwordMinimumLetters, passwordMinimumSymbols, etc are not applied. See PasswordQuality for details.
    ///
    /// "COMPLEX"
    #[serde(rename="COMPLEX")]
    COMPLEX,
    

    /// Define the low password complexity band as: pattern PIN with repeating (4444) or ordered (1234, 4321, 2468) sequencesThis sets the minimum complexity band which the password must meet.Enforcement varies among different Android versions, management modes and password scopes. See PasswordQuality for details.
    ///
    /// "COMPLEXITY_LOW"
    #[serde(rename="COMPLEXITY_LOW")]
    COMPLEXITYLOW,
    

    /// Define the medium password complexity band as: PIN with no repeating (4444) or ordered (1234, 4321, 2468) sequences, length at least 4 alphabetic, length at least 4 alphanumeric, length at least 4This sets the minimum complexity band which the password must meet.Enforcement varies among different Android versions, management modes and password scopes. See PasswordQuality for details.
    ///
    /// "COMPLEXITY_MEDIUM"
    #[serde(rename="COMPLEXITY_MEDIUM")]
    COMPLEXITYMEDIUM,
    

    /// Define the high password complexity band as:On Android 12 and above: PIN with no repeating (4444) or ordered (1234, 4321, 2468) sequences, length at least 8 alphabetic, length at least 6 alphanumeric, length at least 6This sets the minimum complexity band which the password must meet.Enforcement varies among different Android versions, management modes and password scopes. See PasswordQuality for details.
    ///
    /// "COMPLEXITY_HIGH"
    #[serde(rename="COMPLEXITY_HIGH")]
    COMPLEXITYHIGH,
}

impl AsRef<str> for PasswordRequirementPasswordQualityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasswordRequirementPasswordQualityEnum::PASSWORDQUALITYUNSPECIFIED => "PASSWORD_QUALITY_UNSPECIFIED",
            PasswordRequirementPasswordQualityEnum::BIOMETRICWEAK => "BIOMETRIC_WEAK",
            PasswordRequirementPasswordQualityEnum::SOMETHING => "SOMETHING",
            PasswordRequirementPasswordQualityEnum::NUMERIC => "NUMERIC",
            PasswordRequirementPasswordQualityEnum::NUMERICCOMPLEX => "NUMERIC_COMPLEX",
            PasswordRequirementPasswordQualityEnum::ALPHABETIC => "ALPHABETIC",
            PasswordRequirementPasswordQualityEnum::ALPHANUMERIC => "ALPHANUMERIC",
            PasswordRequirementPasswordQualityEnum::COMPLEX => "COMPLEX",
            PasswordRequirementPasswordQualityEnum::COMPLEXITYLOW => "COMPLEXITY_LOW",
            PasswordRequirementPasswordQualityEnum::COMPLEXITYMEDIUM => "COMPLEXITY_MEDIUM",
            PasswordRequirementPasswordQualityEnum::COMPLEXITYHIGH => "COMPLEXITY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for PasswordRequirementPasswordQualityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASSWORD_QUALITY_UNSPECIFIED" => Ok(PasswordRequirementPasswordQualityEnum::PASSWORDQUALITYUNSPECIFIED),
           "BIOMETRIC_WEAK" => Ok(PasswordRequirementPasswordQualityEnum::BIOMETRICWEAK),
           "SOMETHING" => Ok(PasswordRequirementPasswordQualityEnum::SOMETHING),
           "NUMERIC" => Ok(PasswordRequirementPasswordQualityEnum::NUMERIC),
           "NUMERIC_COMPLEX" => Ok(PasswordRequirementPasswordQualityEnum::NUMERICCOMPLEX),
           "ALPHABETIC" => Ok(PasswordRequirementPasswordQualityEnum::ALPHABETIC),
           "ALPHANUMERIC" => Ok(PasswordRequirementPasswordQualityEnum::ALPHANUMERIC),
           "COMPLEX" => Ok(PasswordRequirementPasswordQualityEnum::COMPLEX),
           "COMPLEXITY_LOW" => Ok(PasswordRequirementPasswordQualityEnum::COMPLEXITYLOW),
           "COMPLEXITY_MEDIUM" => Ok(PasswordRequirementPasswordQualityEnum::COMPLEXITYMEDIUM),
           "COMPLEXITY_HIGH" => Ok(PasswordRequirementPasswordQualityEnum::COMPLEXITYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasswordRequirementPasswordQualityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasswordRequirementPasswordScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The scope that the password requirement applies to.
pub enum PasswordRequirementPasswordScopeEnum {
    

    /// The scope is unspecified. The password requirements are applied to the work profile for work profile devices and the whole device for fully managed or dedicated devices.
    ///
    /// "SCOPE_UNSPECIFIED"
    #[serde(rename="SCOPE_UNSPECIFIED")]
    SCOPEUNSPECIFIED,
    

    /// The password requirements are only applied to the device.
    ///
    /// "SCOPE_DEVICE"
    #[serde(rename="SCOPE_DEVICE")]
    SCOPEDEVICE,
    

    /// The password requirements are only applied to the work profile.
    ///
    /// "SCOPE_PROFILE"
    #[serde(rename="SCOPE_PROFILE")]
    SCOPEPROFILE,
}

impl AsRef<str> for PasswordRequirementPasswordScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasswordRequirementPasswordScopeEnum::SCOPEUNSPECIFIED => "SCOPE_UNSPECIFIED",
            PasswordRequirementPasswordScopeEnum::SCOPEDEVICE => "SCOPE_DEVICE",
            PasswordRequirementPasswordScopeEnum::SCOPEPROFILE => "SCOPE_PROFILE",
        }
    }
}

impl std::convert::TryFrom< &str> for PasswordRequirementPasswordScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCOPE_UNSPECIFIED" => Ok(PasswordRequirementPasswordScopeEnum::SCOPEUNSPECIFIED),
           "SCOPE_DEVICE" => Ok(PasswordRequirementPasswordScopeEnum::SCOPEDEVICE),
           "SCOPE_PROFILE" => Ok(PasswordRequirementPasswordScopeEnum::SCOPEPROFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasswordRequirementPasswordScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasswordRequirementRequirePasswordUnlockEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The length of time after a device or work profile is unlocked using a strong form of authentication (password, PIN, pattern) that it can be unlocked using any other authentication method (e.g. fingerprint, trust agents, face). After the specified time period elapses, only strong forms of authentication can be used to unlock the device or work profile.
pub enum PasswordRequirementRequirePasswordUnlockEnum {
    

    /// Unspecified. Defaults to USE_DEFAULT_DEVICE_TIMEOUT.
    ///
    /// "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED"
    #[serde(rename="REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED")]
    REQUIREPASSWORDUNLOCKUNSPECIFIED,
    

    /// The timeout period is set to the device’s default.
    ///
    /// "USE_DEFAULT_DEVICE_TIMEOUT"
    #[serde(rename="USE_DEFAULT_DEVICE_TIMEOUT")]
    USEDEFAULTDEVICETIMEOUT,
    

    /// The timeout period is set to 24 hours.
    ///
    /// "REQUIRE_EVERY_DAY"
    #[serde(rename="REQUIRE_EVERY_DAY")]
    REQUIREEVERYDAY,
}

impl AsRef<str> for PasswordRequirementRequirePasswordUnlockEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasswordRequirementRequirePasswordUnlockEnum::REQUIREPASSWORDUNLOCKUNSPECIFIED => "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED",
            PasswordRequirementRequirePasswordUnlockEnum::USEDEFAULTDEVICETIMEOUT => "USE_DEFAULT_DEVICE_TIMEOUT",
            PasswordRequirementRequirePasswordUnlockEnum::REQUIREEVERYDAY => "REQUIRE_EVERY_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for PasswordRequirementRequirePasswordUnlockEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REQUIRE_PASSWORD_UNLOCK_UNSPECIFIED" => Ok(PasswordRequirementRequirePasswordUnlockEnum::REQUIREPASSWORDUNLOCKUNSPECIFIED),
           "USE_DEFAULT_DEVICE_TIMEOUT" => Ok(PasswordRequirementRequirePasswordUnlockEnum::USEDEFAULTDEVICETIMEOUT),
           "REQUIRE_EVERY_DAY" => Ok(PasswordRequirementRequirePasswordUnlockEnum::REQUIREEVERYDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasswordRequirementRequirePasswordUnlockEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PasswordRequirementUnifiedLockSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether a unified lock is allowed for the device and the work profile, on devices running Android 9 and above with a work profile. This can be set only if password_scope is set to SCOPE_PROFILE, the policy will be rejected otherwise. If user has not set a separate work lock and this field is set to REQUIRE_SEPARATE_WORK_LOCK, a NonComplianceDetail is reported with nonComplianceReason set to USER_ACTION.
pub enum PasswordRequirementUnifiedLockSettingsEnum {
    

    /// Unspecified. Defaults to ALLOW_UNIFIED_WORK_AND_PERSONAL_LOCK.
    ///
    /// "UNIFIED_LOCK_SETTINGS_UNSPECIFIED"
    #[serde(rename="UNIFIED_LOCK_SETTINGS_UNSPECIFIED")]
    UNIFIEDLOCKSETTINGSUNSPECIFIED,
    

    /// A common lock for the device and the work profile is allowed.
    ///
    /// "ALLOW_UNIFIED_WORK_AND_PERSONAL_LOCK"
    #[serde(rename="ALLOW_UNIFIED_WORK_AND_PERSONAL_LOCK")]
    ALLOWUNIFIEDWORKANDPERSONALLOCK,
    

    /// A separate lock for the work profile is required.
    ///
    /// "REQUIRE_SEPARATE_WORK_LOCK"
    #[serde(rename="REQUIRE_SEPARATE_WORK_LOCK")]
    REQUIRESEPARATEWORKLOCK,
}

impl AsRef<str> for PasswordRequirementUnifiedLockSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PasswordRequirementUnifiedLockSettingsEnum::UNIFIEDLOCKSETTINGSUNSPECIFIED => "UNIFIED_LOCK_SETTINGS_UNSPECIFIED",
            PasswordRequirementUnifiedLockSettingsEnum::ALLOWUNIFIEDWORKANDPERSONALLOCK => "ALLOW_UNIFIED_WORK_AND_PERSONAL_LOCK",
            PasswordRequirementUnifiedLockSettingsEnum::REQUIRESEPARATEWORKLOCK => "REQUIRE_SEPARATE_WORK_LOCK",
        }
    }
}

impl std::convert::TryFrom< &str> for PasswordRequirementUnifiedLockSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNIFIED_LOCK_SETTINGS_UNSPECIFIED" => Ok(PasswordRequirementUnifiedLockSettingsEnum::UNIFIEDLOCKSETTINGSUNSPECIFIED),
           "ALLOW_UNIFIED_WORK_AND_PERSONAL_LOCK" => Ok(PasswordRequirementUnifiedLockSettingsEnum::ALLOWUNIFIEDWORKANDPERSONALLOCK),
           "REQUIRE_SEPARATE_WORK_LOCK" => Ok(PasswordRequirementUnifiedLockSettingsEnum::REQUIRESEPARATEWORKLOCK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PasswordRequirementUnifiedLockSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PerAppResultClearingResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The result of an attempt to clear the data of a single app.
pub enum PerAppResultClearingResultEnum {
    

    /// Unspecified result.
    ///
    /// "CLEARING_RESULT_UNSPECIFIED"
    #[serde(rename="CLEARING_RESULT_UNSPECIFIED")]
    CLEARINGRESULTUNSPECIFIED,
    

    /// This app’s data was successfully cleared.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// This app’s data could not be cleared because the app was not found.
    ///
    /// "APP_NOT_FOUND"
    #[serde(rename="APP_NOT_FOUND")]
    APPNOTFOUND,
    

    /// This app’s data could not be cleared because the app is protected. For example, this may apply to apps critical to the functioning of the device, such as Google Play Store.
    ///
    /// "APP_PROTECTED"
    #[serde(rename="APP_PROTECTED")]
    APPPROTECTED,
    

    /// This app’s data could not be cleared because the device API level does not support this command.
    ///
    /// "API_LEVEL"
    #[serde(rename="API_LEVEL")]
    APILEVEL,
}

impl AsRef<str> for PerAppResultClearingResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PerAppResultClearingResultEnum::CLEARINGRESULTUNSPECIFIED => "CLEARING_RESULT_UNSPECIFIED",
            PerAppResultClearingResultEnum::SUCCESS => "SUCCESS",
            PerAppResultClearingResultEnum::APPNOTFOUND => "APP_NOT_FOUND",
            PerAppResultClearingResultEnum::APPPROTECTED => "APP_PROTECTED",
            PerAppResultClearingResultEnum::APILEVEL => "API_LEVEL",
        }
    }
}

impl std::convert::TryFrom< &str> for PerAppResultClearingResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLEARING_RESULT_UNSPECIFIED" => Ok(PerAppResultClearingResultEnum::CLEARINGRESULTUNSPECIFIED),
           "SUCCESS" => Ok(PerAppResultClearingResultEnum::SUCCESS),
           "APP_NOT_FOUND" => Ok(PerAppResultClearingResultEnum::APPNOTFOUND),
           "APP_PROTECTED" => Ok(PerAppResultClearingResultEnum::APPPROTECTED),
           "API_LEVEL" => Ok(PerAppResultClearingResultEnum::APILEVEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PerAppResultClearingResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionGrantPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The policy for granting the permission.
pub enum PermissionGrantPolicyEnum {
    

    /// Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default.
    ///
    /// "PERMISSION_POLICY_UNSPECIFIED"
    #[serde(rename="PERMISSION_POLICY_UNSPECIFIED")]
    PERMISSIONPOLICYUNSPECIFIED,
    

    /// Prompt the user to grant a permission.
    ///
    /// "PROMPT"
    #[serde(rename="PROMPT")]
    PROMPT,
    

    /// Automatically grant a permission.On Android 12 and above, Manifest.permission.READ_SMS (https://developer.android.com/reference/android/Manifest.permission#READ_SMS) and following sensor-related permissions can only be granted on fully managed devices: Manifest.permission.ACCESS_FINE_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_FINE_LOCATION) Manifest.permission.ACCESS_BACKGROUND_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_BACKGROUND_LOCATION) Manifest.permission.ACCESS_COARSE_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_COARSE_LOCATION) Manifest.permission.CAMERA (https://developer.android.com/reference/android/Manifest.permission#CAMERA) Manifest.permission.RECORD_AUDIO (https://developer.android.com/reference/android/Manifest.permission#RECORD_AUDIO) Manifest.permission.ACTIVITY_RECOGNITION (https://developer.android.com/reference/android/Manifest.permission#ACTIVITY_RECOGNITION) Manifest.permission.BODY_SENSORS (https://developer.android.com/reference/android/Manifest.permission#BODY_SENSORS)
    ///
    /// "GRANT"
    #[serde(rename="GRANT")]
    GRANT,
    

    /// Automatically deny a permission.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for PermissionGrantPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionGrantPolicyEnum::PERMISSIONPOLICYUNSPECIFIED => "PERMISSION_POLICY_UNSPECIFIED",
            PermissionGrantPolicyEnum::PROMPT => "PROMPT",
            PermissionGrantPolicyEnum::GRANT => "GRANT",
            PermissionGrantPolicyEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionGrantPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_POLICY_UNSPECIFIED" => Ok(PermissionGrantPolicyEnum::PERMISSIONPOLICYUNSPECIFIED),
           "PROMPT" => Ok(PermissionGrantPolicyEnum::PROMPT),
           "GRANT" => Ok(PermissionGrantPolicyEnum::GRANT),
           "DENY" => Ok(PermissionGrantPolicyEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionGrantPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalApplicationPolicyInstallTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of installation to perform.
pub enum PersonalApplicationPolicyInstallTypeEnum {
    

    /// Unspecified. Defaults to AVAILABLE.
    ///
    /// "INSTALL_TYPE_UNSPECIFIED"
    #[serde(rename="INSTALL_TYPE_UNSPECIFIED")]
    INSTALLTYPEUNSPECIFIED,
    

    /// The app is blocked and can't be installed in the personal profile. If the app was previously installed in the device, it will be uninstalled.
    ///
    /// "BLOCKED"
    #[serde(rename="BLOCKED")]
    BLOCKED,
    

    /// The app is available to install in the personal profile.
    ///
    /// "AVAILABLE"
    #[serde(rename="AVAILABLE")]
    AVAILABLE,
}

impl AsRef<str> for PersonalApplicationPolicyInstallTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalApplicationPolicyInstallTypeEnum::INSTALLTYPEUNSPECIFIED => "INSTALL_TYPE_UNSPECIFIED",
            PersonalApplicationPolicyInstallTypeEnum::BLOCKED => "BLOCKED",
            PersonalApplicationPolicyInstallTypeEnum::AVAILABLE => "AVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalApplicationPolicyInstallTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTALL_TYPE_UNSPECIFIED" => Ok(PersonalApplicationPolicyInstallTypeEnum::INSTALLTYPEUNSPECIFIED),
           "BLOCKED" => Ok(PersonalApplicationPolicyInstallTypeEnum::BLOCKED),
           "AVAILABLE" => Ok(PersonalApplicationPolicyInstallTypeEnum::AVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalApplicationPolicyInstallTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonalUsagePolicyPersonalPlayStoreModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Used together with personalApplications to control how apps in the personal profile are allowed or blocked.
pub enum PersonalUsagePolicyPersonalPlayStoreModeEnum {
    

    /// Unspecified. Defaults to BLOCKLIST.
    ///
    /// "PLAY_STORE_MODE_UNSPECIFIED"
    #[serde(rename="PLAY_STORE_MODE_UNSPECIFIED")]
    PLAYSTOREMODEUNSPECIFIED,
    

    /// All Play Store apps are available for installation in the personal profile, except those whose installType is BLOCKED in personalApplications.
    ///
    /// "BLACKLIST"
    #[serde(rename="BLACKLIST")]
    BLACKLIST,
    

    /// All Play Store apps are available for installation in the personal profile, except those whose installType is BLOCKED in personalApplications.
    ///
    /// "BLOCKLIST"
    #[serde(rename="BLOCKLIST")]
    BLOCKLIST,
    

    /// Only apps explicitly specified in personalApplications with installType set to AVAILABLE are allowed to be installed in the personal profile.
    ///
    /// "ALLOWLIST"
    #[serde(rename="ALLOWLIST")]
    ALLOWLIST,
}

impl AsRef<str> for PersonalUsagePolicyPersonalPlayStoreModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonalUsagePolicyPersonalPlayStoreModeEnum::PLAYSTOREMODEUNSPECIFIED => "PLAY_STORE_MODE_UNSPECIFIED",
            PersonalUsagePolicyPersonalPlayStoreModeEnum::BLACKLIST => "BLACKLIST",
            PersonalUsagePolicyPersonalPlayStoreModeEnum::BLOCKLIST => "BLOCKLIST",
            PersonalUsagePolicyPersonalPlayStoreModeEnum::ALLOWLIST => "ALLOWLIST",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonalUsagePolicyPersonalPlayStoreModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLAY_STORE_MODE_UNSPECIFIED" => Ok(PersonalUsagePolicyPersonalPlayStoreModeEnum::PLAYSTOREMODEUNSPECIFIED),
           "BLACKLIST" => Ok(PersonalUsagePolicyPersonalPlayStoreModeEnum::BLACKLIST),
           "BLOCKLIST" => Ok(PersonalUsagePolicyPersonalPlayStoreModeEnum::BLOCKLIST),
           "ALLOWLIST" => Ok(PersonalUsagePolicyPersonalPlayStoreModeEnum::ALLOWLIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonalUsagePolicyPersonalPlayStoreModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAndroidDevicePolicyTracksEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This setting is not supported. Any value is ignored.
pub enum PolicyAndroidDevicePolicyTracksEnum {
    

    /// This value is ignored.
    ///
    /// "APP_TRACK_UNSPECIFIED"
    #[serde(rename="APP_TRACK_UNSPECIFIED")]
    APPTRACKUNSPECIFIED,
    

    /// The production track, which provides the latest stable release.
    ///
    /// "PRODUCTION"
    #[serde(rename="PRODUCTION")]
    PRODUCTION,
    

    /// The beta track, which provides the latest beta release.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
}

impl AsRef<str> for PolicyAndroidDevicePolicyTracksEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAndroidDevicePolicyTracksEnum::APPTRACKUNSPECIFIED => "APP_TRACK_UNSPECIFIED",
            PolicyAndroidDevicePolicyTracksEnum::PRODUCTION => "PRODUCTION",
            PolicyAndroidDevicePolicyTracksEnum::BETA => "BETA",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAndroidDevicePolicyTracksEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_TRACK_UNSPECIFIED" => Ok(PolicyAndroidDevicePolicyTracksEnum::APPTRACKUNSPECIFIED),
           "PRODUCTION" => Ok(PolicyAndroidDevicePolicyTracksEnum::PRODUCTION),
           "BETA" => Ok(PolicyAndroidDevicePolicyTracksEnum::BETA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAndroidDevicePolicyTracksEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAppAutoUpdatePolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Recommended alternative: autoUpdateMode which is set per app, provides greater flexibility around update frequency.When autoUpdateMode is set to AUTO_UPDATE_POSTPONED or AUTO_UPDATE_HIGH_PRIORITY, this field has no effect.The app auto update policy, which controls when automatic app updates can be applied.
pub enum PolicyAppAutoUpdatePolicyEnum {
    

    /// The auto-update policy is not set. Equivalent to CHOICE_TO_THE_USER.
    ///
    /// "APP_AUTO_UPDATE_POLICY_UNSPECIFIED"
    #[serde(rename="APP_AUTO_UPDATE_POLICY_UNSPECIFIED")]
    APPAUTOUPDATEPOLICYUNSPECIFIED,
    

    /// The user can control auto-updates.
    ///
    /// "CHOICE_TO_THE_USER"
    #[serde(rename="CHOICE_TO_THE_USER")]
    CHOICETOTHEUSER,
    

    /// Apps are never auto-updated.
    ///
    /// "NEVER"
    #[serde(rename="NEVER")]
    NEVER,
    

    /// Apps are auto-updated over Wi-Fi only.
    ///
    /// "WIFI_ONLY"
    #[serde(rename="WIFI_ONLY")]
    WIFIONLY,
    

    /// Apps are auto-updated at any time. Data charges may apply.
    ///
    /// "ALWAYS"
    #[serde(rename="ALWAYS")]
    ALWAYS,
}

impl AsRef<str> for PolicyAppAutoUpdatePolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAppAutoUpdatePolicyEnum::APPAUTOUPDATEPOLICYUNSPECIFIED => "APP_AUTO_UPDATE_POLICY_UNSPECIFIED",
            PolicyAppAutoUpdatePolicyEnum::CHOICETOTHEUSER => "CHOICE_TO_THE_USER",
            PolicyAppAutoUpdatePolicyEnum::NEVER => "NEVER",
            PolicyAppAutoUpdatePolicyEnum::WIFIONLY => "WIFI_ONLY",
            PolicyAppAutoUpdatePolicyEnum::ALWAYS => "ALWAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAppAutoUpdatePolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_AUTO_UPDATE_POLICY_UNSPECIFIED" => Ok(PolicyAppAutoUpdatePolicyEnum::APPAUTOUPDATEPOLICYUNSPECIFIED),
           "CHOICE_TO_THE_USER" => Ok(PolicyAppAutoUpdatePolicyEnum::CHOICETOTHEUSER),
           "NEVER" => Ok(PolicyAppAutoUpdatePolicyEnum::NEVER),
           "WIFI_ONLY" => Ok(PolicyAppAutoUpdatePolicyEnum::WIFIONLY),
           "ALWAYS" => Ok(PolicyAppAutoUpdatePolicyEnum::ALWAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAppAutoUpdatePolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyAutoDateAndTimeZoneEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether auto date, time, and time zone are enabled on a company-owned device. If this is set, then autoTimeRequired is ignored.
pub enum PolicyAutoDateAndTimeZoneEnum {
    

    /// Unspecified. Defaults to AUTO_DATE_AND_TIME_ZONE_USER_CHOICE.
    ///
    /// "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED"
    #[serde(rename="AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED")]
    AUTODATEANDTIMEZONEUNSPECIFIED,
    

    /// Auto date, time, and time zone are left to user's choice.
    ///
    /// "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE"
    #[serde(rename="AUTO_DATE_AND_TIME_ZONE_USER_CHOICE")]
    AUTODATEANDTIMEZONEUSERCHOICE,
    

    /// Enforce auto date, time, and time zone on the device.
    ///
    /// "AUTO_DATE_AND_TIME_ZONE_ENFORCED"
    #[serde(rename="AUTO_DATE_AND_TIME_ZONE_ENFORCED")]
    AUTODATEANDTIMEZONEENFORCED,
}

impl AsRef<str> for PolicyAutoDateAndTimeZoneEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyAutoDateAndTimeZoneEnum::AUTODATEANDTIMEZONEUNSPECIFIED => "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED",
            PolicyAutoDateAndTimeZoneEnum::AUTODATEANDTIMEZONEUSERCHOICE => "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE",
            PolicyAutoDateAndTimeZoneEnum::AUTODATEANDTIMEZONEENFORCED => "AUTO_DATE_AND_TIME_ZONE_ENFORCED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyAutoDateAndTimeZoneEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO_DATE_AND_TIME_ZONE_UNSPECIFIED" => Ok(PolicyAutoDateAndTimeZoneEnum::AUTODATEANDTIMEZONEUNSPECIFIED),
           "AUTO_DATE_AND_TIME_ZONE_USER_CHOICE" => Ok(PolicyAutoDateAndTimeZoneEnum::AUTODATEANDTIMEZONEUSERCHOICE),
           "AUTO_DATE_AND_TIME_ZONE_ENFORCED" => Ok(PolicyAutoDateAndTimeZoneEnum::AUTODATEANDTIMEZONEENFORCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyAutoDateAndTimeZoneEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyCameraAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the use of the camera and whether the user has access to the camera access toggle.
pub enum PolicyCameraAccessEnum {
    

    /// If camera_disabled is true, this is equivalent to CAMERA_ACCESS_DISABLED. Otherwise, this is equivalent to CAMERA_ACCESS_USER_CHOICE.
    ///
    /// "CAMERA_ACCESS_UNSPECIFIED"
    #[serde(rename="CAMERA_ACCESS_UNSPECIFIED")]
    CAMERAACCESSUNSPECIFIED,
    

    /// The field camera_disabled is ignored. This is the default device behaviour: all cameras on the device are available. On Android 12 and above, the user can use the camera access toggle.
    ///
    /// "CAMERA_ACCESS_USER_CHOICE"
    #[serde(rename="CAMERA_ACCESS_USER_CHOICE")]
    CAMERAACCESSUSERCHOICE,
    

    /// The field camera_disabled is ignored. All cameras on the device are disabled (for fully managed devices, this applies device-wide and for work profiles this applies only to the work profile).There are no explicit restrictions placed on the camera access toggle on Android 12 and above: on fully managed devices, the camera access toggle has no effect as all cameras are disabled. On devices with a work profile, this toggle has no effect on apps in the work profile, but it affects apps outside the work profile.
    ///
    /// "CAMERA_ACCESS_DISABLED"
    #[serde(rename="CAMERA_ACCESS_DISABLED")]
    CAMERAACCESSDISABLED,
    

    /// The field camera_disabled is ignored. All cameras on the device are available. On fully managed devices running Android 12 and above, the user is unable to use the camera access toggle. On devices which are not fully managed or which run Android 11 or below, this is equivalent to CAMERA_ACCESS_USER_CHOICE.
    ///
    /// "CAMERA_ACCESS_ENFORCED"
    #[serde(rename="CAMERA_ACCESS_ENFORCED")]
    CAMERAACCESSENFORCED,
}

impl AsRef<str> for PolicyCameraAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyCameraAccessEnum::CAMERAACCESSUNSPECIFIED => "CAMERA_ACCESS_UNSPECIFIED",
            PolicyCameraAccessEnum::CAMERAACCESSUSERCHOICE => "CAMERA_ACCESS_USER_CHOICE",
            PolicyCameraAccessEnum::CAMERAACCESSDISABLED => "CAMERA_ACCESS_DISABLED",
            PolicyCameraAccessEnum::CAMERAACCESSENFORCED => "CAMERA_ACCESS_ENFORCED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyCameraAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAMERA_ACCESS_UNSPECIFIED" => Ok(PolicyCameraAccessEnum::CAMERAACCESSUNSPECIFIED),
           "CAMERA_ACCESS_USER_CHOICE" => Ok(PolicyCameraAccessEnum::CAMERAACCESSUSERCHOICE),
           "CAMERA_ACCESS_DISABLED" => Ok(PolicyCameraAccessEnum::CAMERAACCESSDISABLED),
           "CAMERA_ACCESS_ENFORCED" => Ok(PolicyCameraAccessEnum::CAMERAACCESSENFORCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyCameraAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyCredentialProviderPolicyDefaultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls which apps are allowed to act as credential providers on Android 14 and above. These apps store credentials, see this (https://developer.android.com/training/sign-in/passkeys) and this (https://developer.android.com/reference/androidx/credentials/CredentialManager) for details. See also credentialProviderPolicy.
pub enum PolicyCredentialProviderPolicyDefaultEnum {
    

    /// Unspecified. Defaults to CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED.
    ///
    /// "CREDENTIAL_PROVIDER_POLICY_DEFAULT_UNSPECIFIED"
    #[serde(rename="CREDENTIAL_PROVIDER_POLICY_DEFAULT_UNSPECIFIED")]
    CREDENTIALPROVIDERPOLICYDEFAULTUNSPECIFIED,
    

    /// Apps with credentialProviderPolicy unspecified are not allowed to act as a credential provider.
    ///
    /// "CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED"
    #[serde(rename="CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED")]
    CREDENTIALPROVIDERDEFAULTDISALLOWED,
    

    /// Apps with credentialProviderPolicy unspecified are not allowed to act as a credential provider except for the OEM default credential providers. OEM default credential providers are always allowed to act as credential providers.
    ///
    /// "CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED_EXCEPT_SYSTEM"
    #[serde(rename="CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED_EXCEPT_SYSTEM")]
    CREDENTIALPROVIDERDEFAULTDISALLOWEDEXCEPTSYSTEM,
}

impl AsRef<str> for PolicyCredentialProviderPolicyDefaultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyCredentialProviderPolicyDefaultEnum::CREDENTIALPROVIDERPOLICYDEFAULTUNSPECIFIED => "CREDENTIAL_PROVIDER_POLICY_DEFAULT_UNSPECIFIED",
            PolicyCredentialProviderPolicyDefaultEnum::CREDENTIALPROVIDERDEFAULTDISALLOWED => "CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED",
            PolicyCredentialProviderPolicyDefaultEnum::CREDENTIALPROVIDERDEFAULTDISALLOWEDEXCEPTSYSTEM => "CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED_EXCEPT_SYSTEM",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyCredentialProviderPolicyDefaultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREDENTIAL_PROVIDER_POLICY_DEFAULT_UNSPECIFIED" => Ok(PolicyCredentialProviderPolicyDefaultEnum::CREDENTIALPROVIDERPOLICYDEFAULTUNSPECIFIED),
           "CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED" => Ok(PolicyCredentialProviderPolicyDefaultEnum::CREDENTIALPROVIDERDEFAULTDISALLOWED),
           "CREDENTIAL_PROVIDER_DEFAULT_DISALLOWED_EXCEPT_SYSTEM" => Ok(PolicyCredentialProviderPolicyDefaultEnum::CREDENTIALPROVIDERDEFAULTDISALLOWEDEXCEPTSYSTEM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyCredentialProviderPolicyDefaultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyDefaultPermissionPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The default permission policy for runtime permission requests.
pub enum PolicyDefaultPermissionPolicyEnum {
    

    /// Policy not specified. If no policy is specified for a permission at any level, then the PROMPT behavior is used by default.
    ///
    /// "PERMISSION_POLICY_UNSPECIFIED"
    #[serde(rename="PERMISSION_POLICY_UNSPECIFIED")]
    PERMISSIONPOLICYUNSPECIFIED,
    

    /// Prompt the user to grant a permission.
    ///
    /// "PROMPT"
    #[serde(rename="PROMPT")]
    PROMPT,
    

    /// Automatically grant a permission.On Android 12 and above, Manifest.permission.READ_SMS (https://developer.android.com/reference/android/Manifest.permission#READ_SMS) and following sensor-related permissions can only be granted on fully managed devices: Manifest.permission.ACCESS_FINE_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_FINE_LOCATION) Manifest.permission.ACCESS_BACKGROUND_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_BACKGROUND_LOCATION) Manifest.permission.ACCESS_COARSE_LOCATION (https://developer.android.com/reference/android/Manifest.permission#ACCESS_COARSE_LOCATION) Manifest.permission.CAMERA (https://developer.android.com/reference/android/Manifest.permission#CAMERA) Manifest.permission.RECORD_AUDIO (https://developer.android.com/reference/android/Manifest.permission#RECORD_AUDIO) Manifest.permission.ACTIVITY_RECOGNITION (https://developer.android.com/reference/android/Manifest.permission#ACTIVITY_RECOGNITION) Manifest.permission.BODY_SENSORS (https://developer.android.com/reference/android/Manifest.permission#BODY_SENSORS)
    ///
    /// "GRANT"
    #[serde(rename="GRANT")]
    GRANT,
    

    /// Automatically deny a permission.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for PolicyDefaultPermissionPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyDefaultPermissionPolicyEnum::PERMISSIONPOLICYUNSPECIFIED => "PERMISSION_POLICY_UNSPECIFIED",
            PolicyDefaultPermissionPolicyEnum::PROMPT => "PROMPT",
            PolicyDefaultPermissionPolicyEnum::GRANT => "GRANT",
            PolicyDefaultPermissionPolicyEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyDefaultPermissionPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_POLICY_UNSPECIFIED" => Ok(PolicyDefaultPermissionPolicyEnum::PERMISSIONPOLICYUNSPECIFIED),
           "PROMPT" => Ok(PolicyDefaultPermissionPolicyEnum::PROMPT),
           "GRANT" => Ok(PolicyDefaultPermissionPolicyEnum::GRANT),
           "DENY" => Ok(PolicyDefaultPermissionPolicyEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyDefaultPermissionPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyEncryptionPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether encryption is enabled
pub enum PolicyEncryptionPolicyEnum {
    

    /// This value is ignored, i.e. no encryption required
    ///
    /// "ENCRYPTION_POLICY_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_POLICY_UNSPECIFIED")]
    ENCRYPTIONPOLICYUNSPECIFIED,
    

    /// Encryption required but no password required to boot
    ///
    /// "ENABLED_WITHOUT_PASSWORD"
    #[serde(rename="ENABLED_WITHOUT_PASSWORD")]
    ENABLEDWITHOUTPASSWORD,
    

    /// Encryption required with password required to boot
    ///
    /// "ENABLED_WITH_PASSWORD"
    #[serde(rename="ENABLED_WITH_PASSWORD")]
    ENABLEDWITHPASSWORD,
}

impl AsRef<str> for PolicyEncryptionPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyEncryptionPolicyEnum::ENCRYPTIONPOLICYUNSPECIFIED => "ENCRYPTION_POLICY_UNSPECIFIED",
            PolicyEncryptionPolicyEnum::ENABLEDWITHOUTPASSWORD => "ENABLED_WITHOUT_PASSWORD",
            PolicyEncryptionPolicyEnum::ENABLEDWITHPASSWORD => "ENABLED_WITH_PASSWORD",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyEncryptionPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_POLICY_UNSPECIFIED" => Ok(PolicyEncryptionPolicyEnum::ENCRYPTIONPOLICYUNSPECIFIED),
           "ENABLED_WITHOUT_PASSWORD" => Ok(PolicyEncryptionPolicyEnum::ENABLEDWITHOUTPASSWORD),
           "ENABLED_WITH_PASSWORD" => Ok(PolicyEncryptionPolicyEnum::ENABLEDWITHPASSWORD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyEncryptionPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyKeyguardDisabledFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Disabled keyguard customizations, such as widgets.
pub enum PolicyKeyguardDisabledFeaturesEnum {
    

    /// This value is ignored.
    ///
    /// "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED"
    #[serde(rename="KEYGUARD_DISABLED_FEATURE_UNSPECIFIED")]
    KEYGUARDDISABLEDFEATUREUNSPECIFIED,
    

    /// Disable the camera on secure keyguard screens (e.g. PIN).
    ///
    /// "CAMERA"
    #[serde(rename="CAMERA")]
    CAMERA,
    

    /// Disable showing all notifications on secure keyguard screens.
    ///
    /// "NOTIFICATIONS"
    #[serde(rename="NOTIFICATIONS")]
    NOTIFICATIONS,
    

    /// Disable unredacted notifications on secure keyguard screens.
    ///
    /// "UNREDACTED_NOTIFICATIONS"
    #[serde(rename="UNREDACTED_NOTIFICATIONS")]
    UNREDACTEDNOTIFICATIONS,
    

    /// Ignore trust agent state on secure keyguard screens.
    ///
    /// "TRUST_AGENTS"
    #[serde(rename="TRUST_AGENTS")]
    TRUSTAGENTS,
    

    /// Disable fingerprint sensor on secure keyguard screens.
    ///
    /// "DISABLE_FINGERPRINT"
    #[serde(rename="DISABLE_FINGERPRINT")]
    DISABLEFINGERPRINT,
    

    /// On devices running Android 6 and below, disables text entry into notifications on secure keyguard screens. Has no effect on Android 7 and above.
    ///
    /// "DISABLE_REMOTE_INPUT"
    #[serde(rename="DISABLE_REMOTE_INPUT")]
    DISABLEREMOTEINPUT,
    

    /// Disable face authentication on secure keyguard screens.
    ///
    /// "FACE"
    #[serde(rename="FACE")]
    FACE,
    

    /// Disable iris authentication on secure keyguard screens.
    ///
    /// "IRIS"
    #[serde(rename="IRIS")]
    IRIS,
    

    /// Disable all biometric authentication on secure keyguard screens.
    ///
    /// "BIOMETRICS"
    #[serde(rename="BIOMETRICS")]
    BIOMETRICS,
    

    /// Disable all shortcuts on secure keyguard screen on Android 14 and above.
    ///
    /// "SHORTCUTS"
    #[serde(rename="SHORTCUTS")]
    SHORTCUTS,
    

    /// Disable all current and future keyguard customizations.
    ///
    /// "ALL_FEATURES"
    #[serde(rename="ALL_FEATURES")]
    ALLFEATURES,
}

impl AsRef<str> for PolicyKeyguardDisabledFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyKeyguardDisabledFeaturesEnum::KEYGUARDDISABLEDFEATUREUNSPECIFIED => "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED",
            PolicyKeyguardDisabledFeaturesEnum::CAMERA => "CAMERA",
            PolicyKeyguardDisabledFeaturesEnum::NOTIFICATIONS => "NOTIFICATIONS",
            PolicyKeyguardDisabledFeaturesEnum::UNREDACTEDNOTIFICATIONS => "UNREDACTED_NOTIFICATIONS",
            PolicyKeyguardDisabledFeaturesEnum::TRUSTAGENTS => "TRUST_AGENTS",
            PolicyKeyguardDisabledFeaturesEnum::DISABLEFINGERPRINT => "DISABLE_FINGERPRINT",
            PolicyKeyguardDisabledFeaturesEnum::DISABLEREMOTEINPUT => "DISABLE_REMOTE_INPUT",
            PolicyKeyguardDisabledFeaturesEnum::FACE => "FACE",
            PolicyKeyguardDisabledFeaturesEnum::IRIS => "IRIS",
            PolicyKeyguardDisabledFeaturesEnum::BIOMETRICS => "BIOMETRICS",
            PolicyKeyguardDisabledFeaturesEnum::SHORTCUTS => "SHORTCUTS",
            PolicyKeyguardDisabledFeaturesEnum::ALLFEATURES => "ALL_FEATURES",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyKeyguardDisabledFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEYGUARD_DISABLED_FEATURE_UNSPECIFIED" => Ok(PolicyKeyguardDisabledFeaturesEnum::KEYGUARDDISABLEDFEATUREUNSPECIFIED),
           "CAMERA" => Ok(PolicyKeyguardDisabledFeaturesEnum::CAMERA),
           "NOTIFICATIONS" => Ok(PolicyKeyguardDisabledFeaturesEnum::NOTIFICATIONS),
           "UNREDACTED_NOTIFICATIONS" => Ok(PolicyKeyguardDisabledFeaturesEnum::UNREDACTEDNOTIFICATIONS),
           "TRUST_AGENTS" => Ok(PolicyKeyguardDisabledFeaturesEnum::TRUSTAGENTS),
           "DISABLE_FINGERPRINT" => Ok(PolicyKeyguardDisabledFeaturesEnum::DISABLEFINGERPRINT),
           "DISABLE_REMOTE_INPUT" => Ok(PolicyKeyguardDisabledFeaturesEnum::DISABLEREMOTEINPUT),
           "FACE" => Ok(PolicyKeyguardDisabledFeaturesEnum::FACE),
           "IRIS" => Ok(PolicyKeyguardDisabledFeaturesEnum::IRIS),
           "BIOMETRICS" => Ok(PolicyKeyguardDisabledFeaturesEnum::BIOMETRICS),
           "SHORTCUTS" => Ok(PolicyKeyguardDisabledFeaturesEnum::SHORTCUTS),
           "ALL_FEATURES" => Ok(PolicyKeyguardDisabledFeaturesEnum::ALLFEATURES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyKeyguardDisabledFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyLocationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The degree of location detection enabled.
pub enum PolicyLocationModeEnum {
    

    /// Defaults to LOCATION_USER_CHOICE.
    ///
    /// "LOCATION_MODE_UNSPECIFIED"
    #[serde(rename="LOCATION_MODE_UNSPECIFIED")]
    LOCATIONMODEUNSPECIFIED,
    

    /// On Android 8 and below, all location detection methods are enabled, including GPS, networks, and other sensors. On Android 9 and above, this is equivalent to LOCATION_ENFORCED.
    ///
    /// "HIGH_ACCURACY"
    #[serde(rename="HIGH_ACCURACY")]
    HIGHACCURACY,
    

    /// On Android 8 and below, only GPS and other sensors are enabled. On Android 9 and above, this is equivalent to LOCATION_ENFORCED.
    ///
    /// "SENSORS_ONLY"
    #[serde(rename="SENSORS_ONLY")]
    SENSORSONLY,
    

    /// On Android 8 and below, only the network location provider is enabled. On Android 9 and above, this is equivalent to LOCATION_ENFORCED.
    ///
    /// "BATTERY_SAVING"
    #[serde(rename="BATTERY_SAVING")]
    BATTERYSAVING,
    

    /// On Android 8 and below, location setting and accuracy are disabled. On Android 9 and above, this is equivalent to LOCATION_DISABLED.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// Location setting is not restricted on the device. No specific behavior is set or enforced.
    ///
    /// "LOCATION_USER_CHOICE"
    #[serde(rename="LOCATION_USER_CHOICE")]
    LOCATIONUSERCHOICE,
    

    /// Enable location setting on the device.
    ///
    /// "LOCATION_ENFORCED"
    #[serde(rename="LOCATION_ENFORCED")]
    LOCATIONENFORCED,
    

    /// Disable location setting on the device.
    ///
    /// "LOCATION_DISABLED"
    #[serde(rename="LOCATION_DISABLED")]
    LOCATIONDISABLED,
}

impl AsRef<str> for PolicyLocationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyLocationModeEnum::LOCATIONMODEUNSPECIFIED => "LOCATION_MODE_UNSPECIFIED",
            PolicyLocationModeEnum::HIGHACCURACY => "HIGH_ACCURACY",
            PolicyLocationModeEnum::SENSORSONLY => "SENSORS_ONLY",
            PolicyLocationModeEnum::BATTERYSAVING => "BATTERY_SAVING",
            PolicyLocationModeEnum::OFF => "OFF",
            PolicyLocationModeEnum::LOCATIONUSERCHOICE => "LOCATION_USER_CHOICE",
            PolicyLocationModeEnum::LOCATIONENFORCED => "LOCATION_ENFORCED",
            PolicyLocationModeEnum::LOCATIONDISABLED => "LOCATION_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyLocationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_MODE_UNSPECIFIED" => Ok(PolicyLocationModeEnum::LOCATIONMODEUNSPECIFIED),
           "HIGH_ACCURACY" => Ok(PolicyLocationModeEnum::HIGHACCURACY),
           "SENSORS_ONLY" => Ok(PolicyLocationModeEnum::SENSORSONLY),
           "BATTERY_SAVING" => Ok(PolicyLocationModeEnum::BATTERYSAVING),
           "OFF" => Ok(PolicyLocationModeEnum::OFF),
           "LOCATION_USER_CHOICE" => Ok(PolicyLocationModeEnum::LOCATIONUSERCHOICE),
           "LOCATION_ENFORCED" => Ok(PolicyLocationModeEnum::LOCATIONENFORCED),
           "LOCATION_DISABLED" => Ok(PolicyLocationModeEnum::LOCATIONDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyLocationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyMicrophoneAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the use of the microphone and whether the user has access to the microphone access toggle. This applies only on fully managed devices.
pub enum PolicyMicrophoneAccessEnum {
    

    /// If unmute_microphone_disabled is true, this is equivalent to MICROPHONE_ACCESS_DISABLED. Otherwise, this is equivalent to MICROPHONE_ACCESS_USER_CHOICE.
    ///
    /// "MICROPHONE_ACCESS_UNSPECIFIED"
    #[serde(rename="MICROPHONE_ACCESS_UNSPECIFIED")]
    MICROPHONEACCESSUNSPECIFIED,
    

    /// The field unmute_microphone_disabled is ignored. This is the default device behaviour: the microphone on the device is available. On Android 12 and above, the user can use the microphone access toggle.
    ///
    /// "MICROPHONE_ACCESS_USER_CHOICE"
    #[serde(rename="MICROPHONE_ACCESS_USER_CHOICE")]
    MICROPHONEACCESSUSERCHOICE,
    

    /// The field unmute_microphone_disabled is ignored. The microphone on the device is disabled (for fully managed devices, this applies device-wide).The microphone access toggle has no effect as the microphone is disabled.
    ///
    /// "MICROPHONE_ACCESS_DISABLED"
    #[serde(rename="MICROPHONE_ACCESS_DISABLED")]
    MICROPHONEACCESSDISABLED,
    

    /// The field unmute_microphone_disabled is ignored. The microphone on the device is available. On devices running Android 12 and above, the user is unable to use the microphone access toggle. On devices which run Android 11 or below, this is equivalent to MICROPHONE_ACCESS_USER_CHOICE.
    ///
    /// "MICROPHONE_ACCESS_ENFORCED"
    #[serde(rename="MICROPHONE_ACCESS_ENFORCED")]
    MICROPHONEACCESSENFORCED,
}

impl AsRef<str> for PolicyMicrophoneAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyMicrophoneAccessEnum::MICROPHONEACCESSUNSPECIFIED => "MICROPHONE_ACCESS_UNSPECIFIED",
            PolicyMicrophoneAccessEnum::MICROPHONEACCESSUSERCHOICE => "MICROPHONE_ACCESS_USER_CHOICE",
            PolicyMicrophoneAccessEnum::MICROPHONEACCESSDISABLED => "MICROPHONE_ACCESS_DISABLED",
            PolicyMicrophoneAccessEnum::MICROPHONEACCESSENFORCED => "MICROPHONE_ACCESS_ENFORCED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyMicrophoneAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MICROPHONE_ACCESS_UNSPECIFIED" => Ok(PolicyMicrophoneAccessEnum::MICROPHONEACCESSUNSPECIFIED),
           "MICROPHONE_ACCESS_USER_CHOICE" => Ok(PolicyMicrophoneAccessEnum::MICROPHONEACCESSUSERCHOICE),
           "MICROPHONE_ACCESS_DISABLED" => Ok(PolicyMicrophoneAccessEnum::MICROPHONEACCESSDISABLED),
           "MICROPHONE_ACCESS_ENFORCED" => Ok(PolicyMicrophoneAccessEnum::MICROPHONEACCESSENFORCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyMicrophoneAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyPlayStoreModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This mode controls which apps are available to the user in the Play Store and the behavior on the device when apps are removed from the policy.
pub enum PolicyPlayStoreModeEnum {
    

    /// Unspecified. Defaults to WHITELIST.
    ///
    /// "PLAY_STORE_MODE_UNSPECIFIED"
    #[serde(rename="PLAY_STORE_MODE_UNSPECIFIED")]
    PLAYSTOREMODEUNSPECIFIED,
    

    /// Only apps that are in the policy are available and any app not in the policy will be automatically uninstalled from the device.
    ///
    /// "WHITELIST"
    #[serde(rename="WHITELIST")]
    WHITELIST,
    

    /// All apps are available and any app that should not be on the device should be explicitly marked as 'BLOCKED' in the applications policy.
    ///
    /// "BLACKLIST"
    #[serde(rename="BLACKLIST")]
    BLACKLIST,
}

impl AsRef<str> for PolicyPlayStoreModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyPlayStoreModeEnum::PLAYSTOREMODEUNSPECIFIED => "PLAY_STORE_MODE_UNSPECIFIED",
            PolicyPlayStoreModeEnum::WHITELIST => "WHITELIST",
            PolicyPlayStoreModeEnum::BLACKLIST => "BLACKLIST",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyPlayStoreModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLAY_STORE_MODE_UNSPECIFIED" => Ok(PolicyPlayStoreModeEnum::PLAYSTOREMODEUNSPECIFIED),
           "WHITELIST" => Ok(PolicyPlayStoreModeEnum::WHITELIST),
           "BLACKLIST" => Ok(PolicyPlayStoreModeEnum::BLACKLIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyPlayStoreModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyPreferentialNetworkServiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether preferential network service is enabled on the work profile. For example, an organization may have an agreement with a carrier that all of the work data from its employees' devices will be sent via a network service dedicated for enterprise use. An example of a supported preferential network service is the enterprise slice on 5G networks. This has no effect on fully managed devices.
pub enum PolicyPreferentialNetworkServiceEnum {
    

    /// Unspecified. Defaults to PREFERENTIAL_NETWORK_SERVICES_DISABLED.
    ///
    /// "PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED"
    #[serde(rename="PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED")]
    PREFERENTIALNETWORKSERVICEUNSPECIFIED,
    

    /// Preferential network service is disabled on the work profile.
    ///
    /// "PREFERENTIAL_NETWORK_SERVICE_DISABLED"
    #[serde(rename="PREFERENTIAL_NETWORK_SERVICE_DISABLED")]
    PREFERENTIALNETWORKSERVICEDISABLED,
    

    /// Preferential network service is enabled on the work profile.
    ///
    /// "PREFERENTIAL_NETWORK_SERVICE_ENABLED"
    #[serde(rename="PREFERENTIAL_NETWORK_SERVICE_ENABLED")]
    PREFERENTIALNETWORKSERVICEENABLED,
}

impl AsRef<str> for PolicyPreferentialNetworkServiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyPreferentialNetworkServiceEnum::PREFERENTIALNETWORKSERVICEUNSPECIFIED => "PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED",
            PolicyPreferentialNetworkServiceEnum::PREFERENTIALNETWORKSERVICEDISABLED => "PREFERENTIAL_NETWORK_SERVICE_DISABLED",
            PolicyPreferentialNetworkServiceEnum::PREFERENTIALNETWORKSERVICEENABLED => "PREFERENTIAL_NETWORK_SERVICE_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyPreferentialNetworkServiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PREFERENTIAL_NETWORK_SERVICE_UNSPECIFIED" => Ok(PolicyPreferentialNetworkServiceEnum::PREFERENTIALNETWORKSERVICEUNSPECIFIED),
           "PREFERENTIAL_NETWORK_SERVICE_DISABLED" => Ok(PolicyPreferentialNetworkServiceEnum::PREFERENTIALNETWORKSERVICEDISABLED),
           "PREFERENTIAL_NETWORK_SERVICE_ENABLED" => Ok(PolicyPreferentialNetworkServiceEnum::PREFERENTIALNETWORKSERVICEENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyPreferentialNetworkServiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyPrintingPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Controls whether printing is allowed. This is supported on devices running Android 9 and above. .
pub enum PolicyPrintingPolicyEnum {
    

    /// Unspecified. Defaults to PRINTING_ALLOWED.
    ///
    /// "PRINTING_POLICY_UNSPECIFIED"
    #[serde(rename="PRINTING_POLICY_UNSPECIFIED")]
    PRINTINGPOLICYUNSPECIFIED,
    

    /// Printing is disallowed. A nonComplianceDetail with API_LEVEL is reported if the Android version is less than 9.
    ///
    /// "PRINTING_DISALLOWED"
    #[serde(rename="PRINTING_DISALLOWED")]
    PRINTINGDISALLOWED,
    

    /// Printing is allowed.
    ///
    /// "PRINTING_ALLOWED"
    #[serde(rename="PRINTING_ALLOWED")]
    PRINTINGALLOWED,
}

impl AsRef<str> for PolicyPrintingPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyPrintingPolicyEnum::PRINTINGPOLICYUNSPECIFIED => "PRINTING_POLICY_UNSPECIFIED",
            PolicyPrintingPolicyEnum::PRINTINGDISALLOWED => "PRINTING_DISALLOWED",
            PolicyPrintingPolicyEnum::PRINTINGALLOWED => "PRINTING_ALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyPrintingPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRINTING_POLICY_UNSPECIFIED" => Ok(PolicyPrintingPolicyEnum::PRINTINGPOLICYUNSPECIFIED),
           "PRINTING_DISALLOWED" => Ok(PolicyPrintingPolicyEnum::PRINTINGDISALLOWED),
           "PRINTING_ALLOWED" => Ok(PolicyPrintingPolicyEnum::PRINTINGALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyPrintingPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyStayOnPluggedModesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The battery plugged in modes for which the device stays on. When using this setting, it is recommended to clear maximum_time_to_lock so that the device doesn't lock itself while it stays on.
pub enum PolicyStayOnPluggedModesEnum {
    

    /// This value is ignored.
    ///
    /// "BATTERY_PLUGGED_MODE_UNSPECIFIED"
    #[serde(rename="BATTERY_PLUGGED_MODE_UNSPECIFIED")]
    BATTERYPLUGGEDMODEUNSPECIFIED,
    

    /// Power source is an AC charger.
    ///
    /// "AC"
    #[serde(rename="AC")]
    AC,
    

    /// Power source is a USB port.
    ///
    /// "USB"
    #[serde(rename="USB")]
    USB,
    

    /// Power source is wireless.
    ///
    /// "WIRELESS"
    #[serde(rename="WIRELESS")]
    WIRELESS,
}

impl AsRef<str> for PolicyStayOnPluggedModesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyStayOnPluggedModesEnum::BATTERYPLUGGEDMODEUNSPECIFIED => "BATTERY_PLUGGED_MODE_UNSPECIFIED",
            PolicyStayOnPluggedModesEnum::AC => "AC",
            PolicyStayOnPluggedModesEnum::USB => "USB",
            PolicyStayOnPluggedModesEnum::WIRELESS => "WIRELESS",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyStayOnPluggedModesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BATTERY_PLUGGED_MODE_UNSPECIFIED" => Ok(PolicyStayOnPluggedModesEnum::BATTERYPLUGGEDMODEUNSPECIFIED),
           "AC" => Ok(PolicyStayOnPluggedModesEnum::AC),
           "USB" => Ok(PolicyStayOnPluggedModesEnum::USB),
           "WIRELESS" => Ok(PolicyStayOnPluggedModesEnum::WIRELESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyStayOnPluggedModesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostureDetailSecurityRiskEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A specific security risk that negatively affects the security posture of the device.
pub enum PostureDetailSecurityRiskEnum {
    

    /// Unspecified.
    ///
    /// "SECURITY_RISK_UNSPECIFIED"
    #[serde(rename="SECURITY_RISK_UNSPECIFIED")]
    SECURITYRISKUNSPECIFIED,
    

    /// Play Integrity API detects that the device is running an unknown OS (basicIntegrity check succeeds but ctsProfileMatch fails).
    ///
    /// "UNKNOWN_OS"
    #[serde(rename="UNKNOWN_OS")]
    UNKNOWNOS,
    

    /// Play Integrity API detects that the device is running a compromised OS (basicIntegrity check fails).
    ///
    /// "COMPROMISED_OS"
    #[serde(rename="COMPROMISED_OS")]
    COMPROMISEDOS,
    

    /// Play Integrity API detects that the device does not have a strong guarantee of system integrity, if the MEETS_STRONG_INTEGRITY label doesn't show in the device integrity field (https://developer.android.com/google/play/integrity/verdicts#device-integrity-field).
    ///
    /// "HARDWARE_BACKED_EVALUATION_FAILED"
    #[serde(rename="HARDWARE_BACKED_EVALUATION_FAILED")]
    HARDWAREBACKEDEVALUATIONFAILED,
}

impl AsRef<str> for PostureDetailSecurityRiskEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostureDetailSecurityRiskEnum::SECURITYRISKUNSPECIFIED => "SECURITY_RISK_UNSPECIFIED",
            PostureDetailSecurityRiskEnum::UNKNOWNOS => "UNKNOWN_OS",
            PostureDetailSecurityRiskEnum::COMPROMISEDOS => "COMPROMISED_OS",
            PostureDetailSecurityRiskEnum::HARDWAREBACKEDEVALUATIONFAILED => "HARDWARE_BACKED_EVALUATION_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostureDetailSecurityRiskEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECURITY_RISK_UNSPECIFIED" => Ok(PostureDetailSecurityRiskEnum::SECURITYRISKUNSPECIFIED),
           "UNKNOWN_OS" => Ok(PostureDetailSecurityRiskEnum::UNKNOWNOS),
           "COMPROMISED_OS" => Ok(PostureDetailSecurityRiskEnum::COMPROMISEDOS),
           "HARDWARE_BACKED_EVALUATION_FAILED" => Ok(PostureDetailSecurityRiskEnum::HARDWAREBACKEDEVALUATIONFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostureDetailSecurityRiskEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PowerManagementEventEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event type.
pub enum PowerManagementEventEventTypeEnum {
    

    /// Unspecified. No events have this type.
    ///
    /// "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED")]
    POWERMANAGEMENTEVENTTYPEUNSPECIFIED,
    

    /// Battery level was measured.
    ///
    /// "BATTERY_LEVEL_COLLECTED"
    #[serde(rename="BATTERY_LEVEL_COLLECTED")]
    BATTERYLEVELCOLLECTED,
    

    /// The device started charging.
    ///
    /// "POWER_CONNECTED"
    #[serde(rename="POWER_CONNECTED")]
    POWERCONNECTED,
    

    /// The device stopped charging.
    ///
    /// "POWER_DISCONNECTED"
    #[serde(rename="POWER_DISCONNECTED")]
    POWERDISCONNECTED,
    

    /// The device entered low-power mode.
    ///
    /// "BATTERY_LOW"
    #[serde(rename="BATTERY_LOW")]
    BATTERYLOW,
    

    /// The device exited low-power mode.
    ///
    /// "BATTERY_OKAY"
    #[serde(rename="BATTERY_OKAY")]
    BATTERYOKAY,
    

    /// The device booted.
    ///
    /// "BOOT_COMPLETED"
    #[serde(rename="BOOT_COMPLETED")]
    BOOTCOMPLETED,
    

    /// The device shut down.
    ///
    /// "SHUTDOWN"
    #[serde(rename="SHUTDOWN")]
    SHUTDOWN,
}

impl AsRef<str> for PowerManagementEventEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PowerManagementEventEventTypeEnum::POWERMANAGEMENTEVENTTYPEUNSPECIFIED => "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED",
            PowerManagementEventEventTypeEnum::BATTERYLEVELCOLLECTED => "BATTERY_LEVEL_COLLECTED",
            PowerManagementEventEventTypeEnum::POWERCONNECTED => "POWER_CONNECTED",
            PowerManagementEventEventTypeEnum::POWERDISCONNECTED => "POWER_DISCONNECTED",
            PowerManagementEventEventTypeEnum::BATTERYLOW => "BATTERY_LOW",
            PowerManagementEventEventTypeEnum::BATTERYOKAY => "BATTERY_OKAY",
            PowerManagementEventEventTypeEnum::BOOTCOMPLETED => "BOOT_COMPLETED",
            PowerManagementEventEventTypeEnum::SHUTDOWN => "SHUTDOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for PowerManagementEventEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POWER_MANAGEMENT_EVENT_TYPE_UNSPECIFIED" => Ok(PowerManagementEventEventTypeEnum::POWERMANAGEMENTEVENTTYPEUNSPECIFIED),
           "BATTERY_LEVEL_COLLECTED" => Ok(PowerManagementEventEventTypeEnum::BATTERYLEVELCOLLECTED),
           "POWER_CONNECTED" => Ok(PowerManagementEventEventTypeEnum::POWERCONNECTED),
           "POWER_DISCONNECTED" => Ok(PowerManagementEventEventTypeEnum::POWERDISCONNECTED),
           "BATTERY_LOW" => Ok(PowerManagementEventEventTypeEnum::BATTERYLOW),
           "BATTERY_OKAY" => Ok(PowerManagementEventEventTypeEnum::BATTERYOKAY),
           "BOOT_COMPLETED" => Ok(PowerManagementEventEventTypeEnum::BOOTCOMPLETED),
           "SHUTDOWN" => Ok(PowerManagementEventEventTypeEnum::SHUTDOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PowerManagementEventEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProvisioningInfoManagementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The management mode of the device or profile.
pub enum ProvisioningInfoManagementModeEnum {
    

    /// This value is disallowed.
    ///
    /// "MANAGEMENT_MODE_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_MODE_UNSPECIFIED")]
    MANAGEMENTMODEUNSPECIFIED,
    

    /// Device owner. Android Device Policy has full control over the device.
    ///
    /// "DEVICE_OWNER"
    #[serde(rename="DEVICE_OWNER")]
    DEVICEOWNER,
    

    /// Profile owner. Android Device Policy has control over a managed profile on the device.
    ///
    /// "PROFILE_OWNER"
    #[serde(rename="PROFILE_OWNER")]
    PROFILEOWNER,
}

impl AsRef<str> for ProvisioningInfoManagementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProvisioningInfoManagementModeEnum::MANAGEMENTMODEUNSPECIFIED => "MANAGEMENT_MODE_UNSPECIFIED",
            ProvisioningInfoManagementModeEnum::DEVICEOWNER => "DEVICE_OWNER",
            ProvisioningInfoManagementModeEnum::PROFILEOWNER => "PROFILE_OWNER",
        }
    }
}

impl std::convert::TryFrom< &str> for ProvisioningInfoManagementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_MODE_UNSPECIFIED" => Ok(ProvisioningInfoManagementModeEnum::MANAGEMENTMODEUNSPECIFIED),
           "DEVICE_OWNER" => Ok(ProvisioningInfoManagementModeEnum::DEVICEOWNER),
           "PROFILE_OWNER" => Ok(ProvisioningInfoManagementModeEnum::PROFILEOWNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProvisioningInfoManagementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProvisioningInfoOwnershipEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ownership of the managed device.
pub enum ProvisioningInfoOwnershipEnum {
    

    /// Ownership is unspecified.
    ///
    /// "OWNERSHIP_UNSPECIFIED"
    #[serde(rename="OWNERSHIP_UNSPECIFIED")]
    OWNERSHIPUNSPECIFIED,
    

    /// Device is company-owned.
    ///
    /// "COMPANY_OWNED"
    #[serde(rename="COMPANY_OWNED")]
    COMPANYOWNED,
    

    /// Device is personally-owned.
    ///
    /// "PERSONALLY_OWNED"
    #[serde(rename="PERSONALLY_OWNED")]
    PERSONALLYOWNED,
}

impl AsRef<str> for ProvisioningInfoOwnershipEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProvisioningInfoOwnershipEnum::OWNERSHIPUNSPECIFIED => "OWNERSHIP_UNSPECIFIED",
            ProvisioningInfoOwnershipEnum::COMPANYOWNED => "COMPANY_OWNED",
            ProvisioningInfoOwnershipEnum::PERSONALLYOWNED => "PERSONALLY_OWNED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProvisioningInfoOwnershipEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OWNERSHIP_UNSPECIFIED" => Ok(ProvisioningInfoOwnershipEnum::OWNERSHIPUNSPECIFIED),
           "COMPANY_OWNED" => Ok(ProvisioningInfoOwnershipEnum::COMPANYOWNED),
           "PERSONALLY_OWNED" => Ok(ProvisioningInfoOwnershipEnum::PERSONALLYOWNED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProvisioningInfoOwnershipEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SecurityPostureDevicePostureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Device's security posture value.
pub enum SecurityPostureDevicePostureEnum {
    

    /// Unspecified. There is no posture detail for this posture value.
    ///
    /// "POSTURE_UNSPECIFIED"
    #[serde(rename="POSTURE_UNSPECIFIED")]
    POSTUREUNSPECIFIED,
    

    /// This device is secure.
    ///
    /// "SECURE"
    #[serde(rename="SECURE")]
    SECURE,
    

    /// This device may be more vulnerable to malicious actors than is recommended for use with corporate data.
    ///
    /// "AT_RISK"
    #[serde(rename="AT_RISK")]
    ATRISK,
    

    /// This device may be compromised and corporate data may be accessible to unauthorized actors.
    ///
    /// "POTENTIALLY_COMPROMISED"
    #[serde(rename="POTENTIALLY_COMPROMISED")]
    POTENTIALLYCOMPROMISED,
}

impl AsRef<str> for SecurityPostureDevicePostureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecurityPostureDevicePostureEnum::POSTUREUNSPECIFIED => "POSTURE_UNSPECIFIED",
            SecurityPostureDevicePostureEnum::SECURE => "SECURE",
            SecurityPostureDevicePostureEnum::ATRISK => "AT_RISK",
            SecurityPostureDevicePostureEnum::POTENTIALLYCOMPROMISED => "POTENTIALLY_COMPROMISED",
        }
    }
}

impl std::convert::TryFrom< &str> for SecurityPostureDevicePostureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POSTURE_UNSPECIFIED" => Ok(SecurityPostureDevicePostureEnum::POSTUREUNSPECIFIED),
           "SECURE" => Ok(SecurityPostureDevicePostureEnum::SECURE),
           "AT_RISK" => Ok(SecurityPostureDevicePostureEnum::ATRISK),
           "POTENTIALLY_COMPROMISED" => Ok(SecurityPostureDevicePostureEnum::POTENTIALLYCOMPROMISED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecurityPostureDevicePostureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SigninDetailAllowPersonalUsageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls whether personal usage is allowed on a device provisioned with this enrollment token.For company-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage requires the user provision the device as a fully managed device.For personally-owned devices: Enabling personal usage allows the user to set up a work profile on the device. Disabling personal usage will prevent the device from provisioning. Personal usage cannot be disabled on personally-owned device.
pub enum SigninDetailAllowPersonalUsageEnum {
    

    /// Personal usage restriction is not specified
    ///
    /// "ALLOW_PERSONAL_USAGE_UNSPECIFIED"
    #[serde(rename="ALLOW_PERSONAL_USAGE_UNSPECIFIED")]
    ALLOWPERSONALUSAGEUNSPECIFIED,
    

    /// Personal usage is allowed
    ///
    /// "PERSONAL_USAGE_ALLOWED"
    #[serde(rename="PERSONAL_USAGE_ALLOWED")]
    PERSONALUSAGEALLOWED,
    

    /// Personal usage is disallowed
    ///
    /// "PERSONAL_USAGE_DISALLOWED"
    #[serde(rename="PERSONAL_USAGE_DISALLOWED")]
    PERSONALUSAGEDISALLOWED,
}

impl AsRef<str> for SigninDetailAllowPersonalUsageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SigninDetailAllowPersonalUsageEnum::ALLOWPERSONALUSAGEUNSPECIFIED => "ALLOW_PERSONAL_USAGE_UNSPECIFIED",
            SigninDetailAllowPersonalUsageEnum::PERSONALUSAGEALLOWED => "PERSONAL_USAGE_ALLOWED",
            SigninDetailAllowPersonalUsageEnum::PERSONALUSAGEDISALLOWED => "PERSONAL_USAGE_DISALLOWED",
        }
    }
}

impl std::convert::TryFrom< &str> for SigninDetailAllowPersonalUsageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALLOW_PERSONAL_USAGE_UNSPECIFIED" => Ok(SigninDetailAllowPersonalUsageEnum::ALLOWPERSONALUSAGEUNSPECIFIED),
           "PERSONAL_USAGE_ALLOWED" => Ok(SigninDetailAllowPersonalUsageEnum::PERSONALUSAGEALLOWED),
           "PERSONAL_USAGE_DISALLOWED" => Ok(SigninDetailAllowPersonalUsageEnum::PERSONALUSAGEDISALLOWED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SigninDetailAllowPersonalUsageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StartLostModeStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status. See StartLostModeStatus.
pub enum StartLostModeStatusStatusEnum {
    

    /// Unspecified. This value is not used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The device was put into lost mode.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// The device could not be put into lost mode because the admin reset the device's password recently.
    ///
    /// "RESET_PASSWORD_RECENTLY"
    #[serde(rename="RESET_PASSWORD_RECENTLY")]
    RESETPASSWORDRECENTLY,
    

    /// The device could not be put into lost mode because the user exited lost mode recently.
    ///
    /// "USER_EXIT_LOST_MODE_RECENTLY"
    #[serde(rename="USER_EXIT_LOST_MODE_RECENTLY")]
    USEREXITLOSTMODERECENTLY,
    

    /// The device is already in lost mode.
    ///
    /// "ALREADY_IN_LOST_MODE"
    #[serde(rename="ALREADY_IN_LOST_MODE")]
    ALREADYINLOSTMODE,
}

impl AsRef<str> for StartLostModeStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StartLostModeStatusStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            StartLostModeStatusStatusEnum::SUCCESS => "SUCCESS",
            StartLostModeStatusStatusEnum::RESETPASSWORDRECENTLY => "RESET_PASSWORD_RECENTLY",
            StartLostModeStatusStatusEnum::USEREXITLOSTMODERECENTLY => "USER_EXIT_LOST_MODE_RECENTLY",
            StartLostModeStatusStatusEnum::ALREADYINLOSTMODE => "ALREADY_IN_LOST_MODE",
        }
    }
}

impl std::convert::TryFrom< &str> for StartLostModeStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(StartLostModeStatusStatusEnum::STATUSUNSPECIFIED),
           "SUCCESS" => Ok(StartLostModeStatusStatusEnum::SUCCESS),
           "RESET_PASSWORD_RECENTLY" => Ok(StartLostModeStatusStatusEnum::RESETPASSWORDRECENTLY),
           "USER_EXIT_LOST_MODE_RECENTLY" => Ok(StartLostModeStatusStatusEnum::USEREXITLOSTMODERECENTLY),
           "ALREADY_IN_LOST_MODE" => Ok(StartLostModeStatusStatusEnum::ALREADYINLOSTMODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StartLostModeStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StopLostModeStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status. See StopLostModeStatus.
pub enum StopLostModeStatusStatusEnum {
    

    /// Unspecified. This value is not used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The device was taken out of lost mode.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// The device is not in lost mode.
    ///
    /// "NOT_IN_LOST_MODE"
    #[serde(rename="NOT_IN_LOST_MODE")]
    NOTINLOSTMODE,
}

impl AsRef<str> for StopLostModeStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StopLostModeStatusStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            StopLostModeStatusStatusEnum::SUCCESS => "SUCCESS",
            StopLostModeStatusStatusEnum::NOTINLOSTMODE => "NOT_IN_LOST_MODE",
        }
    }
}

impl std::convert::TryFrom< &str> for StopLostModeStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(StopLostModeStatusStatusEnum::STATUSUNSPECIFIED),
           "SUCCESS" => Ok(StopLostModeStatusStatusEnum::SUCCESS),
           "NOT_IN_LOST_MODE" => Ok(StopLostModeStatusStatusEnum::NOTINLOSTMODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StopLostModeStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SystemUpdateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of system update to configure.
pub enum SystemUpdateTypeEnum {
    

    /// Follow the default update behavior for the device, which typically requires the user to accept system updates.
    ///
    /// "SYSTEM_UPDATE_TYPE_UNSPECIFIED"
    #[serde(rename="SYSTEM_UPDATE_TYPE_UNSPECIFIED")]
    SYSTEMUPDATETYPEUNSPECIFIED,
    

    /// Install automatically as soon as an update is available.
    ///
    /// "AUTOMATIC"
    #[serde(rename="AUTOMATIC")]
    AUTOMATIC,
    

    /// Install automatically within a daily maintenance window. This also configures Play apps to be updated within the window. This is strongly recommended for kiosk devices because this is the only way apps persistently pinned to the foreground can be updated by Play.If autoUpdateMode is set to AUTO_UPDATE_HIGH_PRIORITY for an app, then the maintenance window is ignored for that app and it is updated as soon as possible even outside of the maintenance window.
    ///
    /// "WINDOWED"
    #[serde(rename="WINDOWED")]
    WINDOWED,
    

    /// Postpone automatic install up to a maximum of 30 days. This policy does not affect security updates (e.g. monthly security patches).
    ///
    /// "POSTPONE"
    #[serde(rename="POSTPONE")]
    POSTPONE,
}

impl AsRef<str> for SystemUpdateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SystemUpdateTypeEnum::SYSTEMUPDATETYPEUNSPECIFIED => "SYSTEM_UPDATE_TYPE_UNSPECIFIED",
            SystemUpdateTypeEnum::AUTOMATIC => "AUTOMATIC",
            SystemUpdateTypeEnum::WINDOWED => "WINDOWED",
            SystemUpdateTypeEnum::POSTPONE => "POSTPONE",
        }
    }
}

impl std::convert::TryFrom< &str> for SystemUpdateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYSTEM_UPDATE_TYPE_UNSPECIFIED" => Ok(SystemUpdateTypeEnum::SYSTEMUPDATETYPEUNSPECIFIED),
           "AUTOMATIC" => Ok(SystemUpdateTypeEnum::AUTOMATIC),
           "WINDOWED" => Ok(SystemUpdateTypeEnum::WINDOWED),
           "POSTPONE" => Ok(SystemUpdateTypeEnum::POSTPONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SystemUpdateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SystemUpdateInfoUpdateStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of an update: whether an update exists and what type it is.
pub enum SystemUpdateInfoUpdateStatusEnum {
    

    /// It is unknown whether there is a pending system update. This happens when, for example, the device API level is less than 26, or if the version of Android Device Policy is outdated.
    ///
    /// "UPDATE_STATUS_UNKNOWN"
    #[serde(rename="UPDATE_STATUS_UNKNOWN")]
    UPDATESTATUSUNKNOWN,
    

    /// There is no pending system update available on the device.
    ///
    /// "UP_TO_DATE"
    #[serde(rename="UP_TO_DATE")]
    UPTODATE,
    

    /// There is a pending system update available, but its type is not known.
    ///
    /// "UNKNOWN_UPDATE_AVAILABLE"
    #[serde(rename="UNKNOWN_UPDATE_AVAILABLE")]
    UNKNOWNUPDATEAVAILABLE,
    

    /// There is a pending security update available.
    ///
    /// "SECURITY_UPDATE_AVAILABLE"
    #[serde(rename="SECURITY_UPDATE_AVAILABLE")]
    SECURITYUPDATEAVAILABLE,
    

    /// There is a pending OS update available.
    ///
    /// "OS_UPDATE_AVAILABLE"
    #[serde(rename="OS_UPDATE_AVAILABLE")]
    OSUPDATEAVAILABLE,
}

impl AsRef<str> for SystemUpdateInfoUpdateStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SystemUpdateInfoUpdateStatusEnum::UPDATESTATUSUNKNOWN => "UPDATE_STATUS_UNKNOWN",
            SystemUpdateInfoUpdateStatusEnum::UPTODATE => "UP_TO_DATE",
            SystemUpdateInfoUpdateStatusEnum::UNKNOWNUPDATEAVAILABLE => "UNKNOWN_UPDATE_AVAILABLE",
            SystemUpdateInfoUpdateStatusEnum::SECURITYUPDATEAVAILABLE => "SECURITY_UPDATE_AVAILABLE",
            SystemUpdateInfoUpdateStatusEnum::OSUPDATEAVAILABLE => "OS_UPDATE_AVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for SystemUpdateInfoUpdateStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPDATE_STATUS_UNKNOWN" => Ok(SystemUpdateInfoUpdateStatusEnum::UPDATESTATUSUNKNOWN),
           "UP_TO_DATE" => Ok(SystemUpdateInfoUpdateStatusEnum::UPTODATE),
           "UNKNOWN_UPDATE_AVAILABLE" => Ok(SystemUpdateInfoUpdateStatusEnum::UNKNOWNUPDATEAVAILABLE),
           "SECURITY_UPDATE_AVAILABLE" => Ok(SystemUpdateInfoUpdateStatusEnum::SECURITYUPDATEAVAILABLE),
           "OS_UPDATE_AVAILABLE" => Ok(SystemUpdateInfoUpdateStatusEnum::OSUPDATEAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SystemUpdateInfoUpdateStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UsageLogEnabledLogTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which log types are enabled. Note that users will receive on-device messaging when usage logging is enabled.
pub enum UsageLogEnabledLogTypesEnum {
    

    /// This value is not used.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Enable logging of on-device security events, like when the device password is incorrectly entered or removable storage is mounted. See UsageLogEvent for a complete description of the logged security events. Supported for fully managed devices on Android 7 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only security events from the work profile are logged. Can be overridden by the application delegated scope SECURITY_LOGS
    ///
    /// "SECURITY_LOGS"
    #[serde(rename="SECURITY_LOGS")]
    SECURITYLOGS,
    

    /// Enable logging of on-device network events, like DNS lookups and TCP connections. See UsageLogEvent for a complete description of the logged network events. Supported for fully managed devices on Android 8 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only network events from the work profile are logged. Can be overridden by the application delegated scope NETWORK_ACTIVITY_LOGS
    ///
    /// "NETWORK_ACTIVITY_LOGS"
    #[serde(rename="NETWORK_ACTIVITY_LOGS")]
    NETWORKACTIVITYLOGS,
}

impl AsRef<str> for UsageLogEnabledLogTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UsageLogEnabledLogTypesEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            UsageLogEnabledLogTypesEnum::SECURITYLOGS => "SECURITY_LOGS",
            UsageLogEnabledLogTypesEnum::NETWORKACTIVITYLOGS => "NETWORK_ACTIVITY_LOGS",
        }
    }
}

impl std::convert::TryFrom< &str> for UsageLogEnabledLogTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(UsageLogEnabledLogTypesEnum::LOGTYPEUNSPECIFIED),
           "SECURITY_LOGS" => Ok(UsageLogEnabledLogTypesEnum::SECURITYLOGS),
           "NETWORK_ACTIVITY_LOGS" => Ok(UsageLogEnabledLogTypesEnum::NETWORKACTIVITYLOGS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UsageLogEnabledLogTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UsageLogUploadOnCellularAllowedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which of the enabled log types can be uploaded over mobile data. By default logs are queued for upload when the device connects to WiFi.
pub enum UsageLogUploadOnCellularAllowedEnum {
    

    /// This value is not used.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Enable logging of on-device security events, like when the device password is incorrectly entered or removable storage is mounted. See UsageLogEvent for a complete description of the logged security events. Supported for fully managed devices on Android 7 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only security events from the work profile are logged. Can be overridden by the application delegated scope SECURITY_LOGS
    ///
    /// "SECURITY_LOGS"
    #[serde(rename="SECURITY_LOGS")]
    SECURITYLOGS,
    

    /// Enable logging of on-device network events, like DNS lookups and TCP connections. See UsageLogEvent for a complete description of the logged network events. Supported for fully managed devices on Android 8 and above. Supported for company-owned devices with a work profile on Android 12 and above, on which only network events from the work profile are logged. Can be overridden by the application delegated scope NETWORK_ACTIVITY_LOGS
    ///
    /// "NETWORK_ACTIVITY_LOGS"
    #[serde(rename="NETWORK_ACTIVITY_LOGS")]
    NETWORKACTIVITYLOGS,
}

impl AsRef<str> for UsageLogUploadOnCellularAllowedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UsageLogUploadOnCellularAllowedEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            UsageLogUploadOnCellularAllowedEnum::SECURITYLOGS => "SECURITY_LOGS",
            UsageLogUploadOnCellularAllowedEnum::NETWORKACTIVITYLOGS => "NETWORK_ACTIVITY_LOGS",
        }
    }
}

impl std::convert::TryFrom< &str> for UsageLogUploadOnCellularAllowedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(UsageLogUploadOnCellularAllowedEnum::LOGTYPEUNSPECIFIED),
           "SECURITY_LOGS" => Ok(UsageLogUploadOnCellularAllowedEnum::SECURITYLOGS),
           "NETWORK_ACTIVITY_LOGS" => Ok(UsageLogUploadOnCellularAllowedEnum::NETWORKACTIVITYLOGS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UsageLogUploadOnCellularAllowedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WebAppDisplayModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The display mode of the web app.
pub enum WebAppDisplayModeEnum {
    

    /// Not used.
    ///
    /// "DISPLAY_MODE_UNSPECIFIED"
    #[serde(rename="DISPLAY_MODE_UNSPECIFIED")]
    DISPLAYMODEUNSPECIFIED,
    

    /// Opens the web app with a minimal set of browser UI elements for controlling navigation and viewing the page URL.
    ///
    /// "MINIMAL_UI"
    #[serde(rename="MINIMAL_UI")]
    MINIMALUI,
    

    /// Opens the web app to look and feel like a standalone native application. The browser UI elements and page URL are not visible, however the system status bar and back button are visible.
    ///
    /// "STANDALONE"
    #[serde(rename="STANDALONE")]
    STANDALONE,
    

    /// Opens the web app in full screen without any visible controls. The browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area.
    ///
    /// "FULL_SCREEN"
    #[serde(rename="FULL_SCREEN")]
    FULLSCREEN,
}

impl AsRef<str> for WebAppDisplayModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebAppDisplayModeEnum::DISPLAYMODEUNSPECIFIED => "DISPLAY_MODE_UNSPECIFIED",
            WebAppDisplayModeEnum::MINIMALUI => "MINIMAL_UI",
            WebAppDisplayModeEnum::STANDALONE => "STANDALONE",
            WebAppDisplayModeEnum::FULLSCREEN => "FULL_SCREEN",
        }
    }
}

impl std::convert::TryFrom< &str> for WebAppDisplayModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY_MODE_UNSPECIFIED" => Ok(WebAppDisplayModeEnum::DISPLAYMODEUNSPECIFIED),
           "MINIMAL_UI" => Ok(WebAppDisplayModeEnum::MINIMALUI),
           "STANDALONE" => Ok(WebAppDisplayModeEnum::STANDALONE),
           "FULL_SCREEN" => Ok(WebAppDisplayModeEnum::FULLSCREEN),
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


// region WebTokenEnabledFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The features to enable. Use this if you want to control exactly which feature(s) will be activated; leave empty to allow all features.Restrictions / things to note: - If no features are listed here, all features are enabled — this is the default behavior where you give access to all features to your admins. - This must not contain any FEATURE_UNSPECIFIED values. - Repeated values are ignored 
pub enum WebTokenEnabledFeaturesEnum {
    

    /// Unspecified feature.
    ///
    /// "FEATURE_UNSPECIFIED"
    #[serde(rename="FEATURE_UNSPECIFIED")]
    FEATUREUNSPECIFIED,
    

    /// The Managed Play search apps page (https://developers.google.com/android/management/apps#search-apps).
    ///
    /// "PLAY_SEARCH"
    #[serde(rename="PLAY_SEARCH")]
    PLAYSEARCH,
    

    /// The private apps page (https://developers.google.com/android/management/apps#private-apps).
    ///
    /// "PRIVATE_APPS"
    #[serde(rename="PRIVATE_APPS")]
    PRIVATEAPPS,
    

    /// The Web Apps page (https://developers.google.com/android/management/apps#web-apps).
    ///
    /// "WEB_APPS"
    #[serde(rename="WEB_APPS")]
    WEBAPPS,
    

    /// The organize apps page (https://developers.google.com/android/management/apps#organize-apps).
    ///
    /// "STORE_BUILDER"
    #[serde(rename="STORE_BUILDER")]
    STOREBUILDER,
    

    /// The managed configurations page (https://developers.google.com/android/management/managed-configurations-iframe).
    ///
    /// "MANAGED_CONFIGURATIONS"
    #[serde(rename="MANAGED_CONFIGURATIONS")]
    MANAGEDCONFIGURATIONS,
    

    /// The zero-touch iframe (https://developers.google.com/android/management/zero-touch-iframe).
    ///
    /// "ZERO_TOUCH_CUSTOMER_MANAGEMENT"
    #[serde(rename="ZERO_TOUCH_CUSTOMER_MANAGEMENT")]
    ZEROTOUCHCUSTOMERMANAGEMENT,
}

impl AsRef<str> for WebTokenEnabledFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebTokenEnabledFeaturesEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            WebTokenEnabledFeaturesEnum::PLAYSEARCH => "PLAY_SEARCH",
            WebTokenEnabledFeaturesEnum::PRIVATEAPPS => "PRIVATE_APPS",
            WebTokenEnabledFeaturesEnum::WEBAPPS => "WEB_APPS",
            WebTokenEnabledFeaturesEnum::STOREBUILDER => "STORE_BUILDER",
            WebTokenEnabledFeaturesEnum::MANAGEDCONFIGURATIONS => "MANAGED_CONFIGURATIONS",
            WebTokenEnabledFeaturesEnum::ZEROTOUCHCUSTOMERMANAGEMENT => "ZERO_TOUCH_CUSTOMER_MANAGEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for WebTokenEnabledFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(WebTokenEnabledFeaturesEnum::FEATUREUNSPECIFIED),
           "PLAY_SEARCH" => Ok(WebTokenEnabledFeaturesEnum::PLAYSEARCH),
           "PRIVATE_APPS" => Ok(WebTokenEnabledFeaturesEnum::PRIVATEAPPS),
           "WEB_APPS" => Ok(WebTokenEnabledFeaturesEnum::WEBAPPS),
           "STORE_BUILDER" => Ok(WebTokenEnabledFeaturesEnum::STOREBUILDER),
           "MANAGED_CONFIGURATIONS" => Ok(WebTokenEnabledFeaturesEnum::MANAGEDCONFIGURATIONS),
           "ZERO_TOUCH_CUSTOMER_MANAGEMENT" => Ok(WebTokenEnabledFeaturesEnum::ZEROTOUCHCUSTOMERMANAGEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebTokenEnabledFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WebTokenPermissionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Permissions available to an admin in the embedded UI. An admin must have all of these permissions in order to view the UI. This field is deprecated.
pub enum WebTokenPermissionsEnum {
    

    /// This value is ignored.
    ///
    /// "WEB_TOKEN_PERMISSION_UNSPECIFIED"
    #[serde(rename="WEB_TOKEN_PERMISSION_UNSPECIFIED")]
    WEBTOKENPERMISSIONUNSPECIFIED,
    

    /// The permission to approve apps for the enterprise.
    ///
    /// "APPROVE_APPS"
    #[serde(rename="APPROVE_APPS")]
    APPROVEAPPS,
}

impl AsRef<str> for WebTokenPermissionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebTokenPermissionsEnum::WEBTOKENPERMISSIONUNSPECIFIED => "WEB_TOKEN_PERMISSION_UNSPECIFIED",
            WebTokenPermissionsEnum::APPROVEAPPS => "APPROVE_APPS",
        }
    }
}

impl std::convert::TryFrom< &str> for WebTokenPermissionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEB_TOKEN_PERMISSION_UNSPECIFIED" => Ok(WebTokenPermissionsEnum::WEBTOKENPERMISSIONUNSPECIFIED),
           "APPROVE_APPS" => Ok(WebTokenPermissionsEnum::APPROVEAPPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebTokenPermissionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseWipeDataFlagsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional flags that control the device wiping behavior.
pub enum EnterpriseWipeDataFlagsEnum {
    

    /// This value is ignored.
    ///
    /// "WIPE_DATA_FLAG_UNSPECIFIED"
    #[serde(rename="WIPE_DATA_FLAG_UNSPECIFIED")]
    WIPEDATAFLAGUNSPECIFIED,
    

    /// Preserve the factory reset protection data on the device.
    ///
    /// "PRESERVE_RESET_PROTECTION_DATA"
    #[serde(rename="PRESERVE_RESET_PROTECTION_DATA")]
    PRESERVERESETPROTECTIONDATA,
    

    /// Additionally wipe the device's external storage (such as SD cards).
    ///
    /// "WIPE_EXTERNAL_STORAGE"
    #[serde(rename="WIPE_EXTERNAL_STORAGE")]
    WIPEEXTERNALSTORAGE,
}

impl AsRef<str> for EnterpriseWipeDataFlagsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseWipeDataFlagsEnum::WIPEDATAFLAGUNSPECIFIED => "WIPE_DATA_FLAG_UNSPECIFIED",
            EnterpriseWipeDataFlagsEnum::PRESERVERESETPROTECTIONDATA => "PRESERVE_RESET_PROTECTION_DATA",
            EnterpriseWipeDataFlagsEnum::WIPEEXTERNALSTORAGE => "WIPE_EXTERNAL_STORAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseWipeDataFlagsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WIPE_DATA_FLAG_UNSPECIFIED" => Ok(EnterpriseWipeDataFlagsEnum::WIPEDATAFLAGUNSPECIFIED),
           "PRESERVE_RESET_PROTECTION_DATA" => Ok(EnterpriseWipeDataFlagsEnum::PRESERVERESETPROTECTIONDATA),
           "WIPE_EXTERNAL_STORAGE" => Ok(EnterpriseWipeDataFlagsEnum::WIPEEXTERNALSTORAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseWipeDataFlagsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies which Enterprise fields to return. This method only supports BASIC.
pub enum EnterpriseViewEnum {
    

    /// The API will default to the BASIC view for the List method.
    ///
    /// "ENTERPRISE_VIEW_UNSPECIFIED"
    #[serde(rename="ENTERPRISE_VIEW_UNSPECIFIED")]
    ENTERPRISEVIEWUNSPECIFIED,
    

    /// Includes name and enterprise_display_name fields.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for EnterpriseViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseViewEnum::ENTERPRISEVIEWUNSPECIFIED => "ENTERPRISE_VIEW_UNSPECIFIED",
            EnterpriseViewEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTERPRISE_VIEW_UNSPECIFIED" => Ok(EnterpriseViewEnum::ENTERPRISEVIEWUNSPECIFIED),
           "BASIC" => Ok(EnterpriseViewEnum::BASIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


