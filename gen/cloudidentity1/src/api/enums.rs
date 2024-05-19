use super::*;



// region DynamicGroupQueryResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Resource type for the Dynamic Group Query
pub enum DynamicGroupQueryResourceTypeEnum {
    

    /// Default value (not valid)
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// For queries on User
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
}

impl AsRef<str> for DynamicGroupQueryResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicGroupQueryResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            DynamicGroupQueryResourceTypeEnum::USER => "USER",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicGroupQueryResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(DynamicGroupQueryResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "USER" => Ok(DynamicGroupQueryResourceTypeEnum::USER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicGroupQueryResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DynamicGroupStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the dynamic group.
pub enum DynamicGroupStatusStatusEnum {
    

    /// Default.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The dynamic group is up-to-date.
    ///
    /// "UP_TO_DATE"
    #[serde(rename="UP_TO_DATE")]
    UPTODATE,
    

    /// The dynamic group has just been created and memberships are being updated.
    ///
    /// "UPDATING_MEMBERSHIPS"
    #[serde(rename="UPDATING_MEMBERSHIPS")]
    UPDATINGMEMBERSHIPS,
    

    /// Group is in an unrecoverable state and its memberships can't be updated.
    ///
    /// "INVALID_QUERY"
    #[serde(rename="INVALID_QUERY")]
    INVALIDQUERY,
}

impl AsRef<str> for DynamicGroupStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DynamicGroupStatusStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            DynamicGroupStatusStatusEnum::UPTODATE => "UP_TO_DATE",
            DynamicGroupStatusStatusEnum::UPDATINGMEMBERSHIPS => "UPDATING_MEMBERSHIPS",
            DynamicGroupStatusStatusEnum::INVALIDQUERY => "INVALID_QUERY",
        }
    }
}

impl std::convert::TryFrom< &str> for DynamicGroupStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(DynamicGroupStatusStatusEnum::STATUSUNSPECIFIED),
           "UP_TO_DATE" => Ok(DynamicGroupStatusStatusEnum::UPTODATE),
           "UPDATING_MEMBERSHIPS" => Ok(DynamicGroupStatusStatusEnum::UPDATINGMEMBERSHIPS),
           "INVALID_QUERY" => Ok(DynamicGroupStatusStatusEnum::INVALIDQUERY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DynamicGroupStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ownership privileges on device.
pub enum GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum {
    

    /// Ownership privilege is not set.
    ///
    /// "OWNERSHIP_PRIVILEGE_UNSPECIFIED"
    #[serde(rename="OWNERSHIP_PRIVILEGE_UNSPECIFIED")]
    OWNERSHIPPRIVILEGEUNSPECIFIED,
    

    /// Active device administrator privileges on the device.
    ///
    /// "DEVICE_ADMINISTRATOR"
    #[serde(rename="DEVICE_ADMINISTRATOR")]
    DEVICEADMINISTRATOR,
    

    /// Profile Owner privileges. The account is in a managed corporate profile.
    ///
    /// "PROFILE_OWNER"
    #[serde(rename="PROFILE_OWNER")]
    PROFILEOWNER,
    

    /// Device Owner privileges on the device.
    ///
    /// "DEVICE_OWNER"
    #[serde(rename="DEVICE_OWNER")]
    DEVICEOWNER,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::OWNERSHIPPRIVILEGEUNSPECIFIED => "OWNERSHIP_PRIVILEGE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::DEVICEADMINISTRATOR => "DEVICE_ADMINISTRATOR",
            GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::PROFILEOWNER => "PROFILE_OWNER",
            GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::DEVICEOWNER => "DEVICE_OWNER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OWNERSHIP_PRIVILEGE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::OWNERSHIPPRIVILEGEUNSPECIFIED),
           "DEVICE_ADMINISTRATOR" => Ok(GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::DEVICEADMINISTRATOR),
           "PROFILE_OWNER" => Ok(GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::PROFILEOWNER),
           "DEVICE_OWNER" => Ok(GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum::DEVICEOWNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1AndroidAttributeOwnershipPrivilegeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Browser's management state.
pub enum GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum {
    

    /// Management state is not specified.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Browser/Profile is not managed by any customer.
    ///
    /// "UNMANAGED"
    #[serde(rename="UNMANAGED")]
    UNMANAGED,
    

    /// Browser/Profile is managed, but by some other customer.
    ///
    /// "MANAGED_BY_OTHER_DOMAIN"
    #[serde(rename="MANAGED_BY_OTHER_DOMAIN")]
    MANAGEDBYOTHERDOMAIN,
    

    /// Profile is managed by customer.
    ///
    /// "PROFILE_MANAGED"
    #[serde(rename="PROFILE_MANAGED")]
    PROFILEMANAGED,
    

    /// Browser is managed by customer.
    ///
    /// "BROWSER_MANAGED"
    #[serde(rename="BROWSER_MANAGED")]
    BROWSERMANAGED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::UNSPECIFIED => "UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::UNMANAGED => "UNMANAGED",
            GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::MANAGEDBYOTHERDOMAIN => "MANAGED_BY_OTHER_DOMAIN",
            GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::PROFILEMANAGED => "PROFILE_MANAGED",
            GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::BROWSERMANAGED => "BROWSER_MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::UNSPECIFIED),
           "UNMANAGED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::UNMANAGED),
           "MANAGED_BY_OTHER_DOMAIN" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::MANAGEDBYOTHERDOMAIN),
           "PROFILE_MANAGED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::PROFILEMANAGED),
           "BROWSER_MANAGED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum::BROWSERMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1BrowserInfoBrowserManagementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current state of [password protection trigger](https://chromeenterprise.google/policies/#PasswordProtectionWarningTrigger).
pub enum GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum {
    

    /// Password protection is not specified.
    ///
    /// "PASSWORD_PROTECTION_TRIGGER_UNSPECIFIED"
    #[serde(rename="PASSWORD_PROTECTION_TRIGGER_UNSPECIFIED")]
    PASSWORDPROTECTIONTRIGGERUNSPECIFIED,
    

    /// Password reuse is never detected.
    ///
    /// "PROTECTION_OFF"
    #[serde(rename="PROTECTION_OFF")]
    PROTECTIONOFF,
    

    /// Warning is shown when the user reuses their protected password on a non-allowed site.
    ///
    /// "PASSWORD_REUSE"
    #[serde(rename="PASSWORD_REUSE")]
    PASSWORDREUSE,
    

    /// Warning is shown when the user reuses their protected password on a phishing site.
    ///
    /// "PHISHING_REUSE"
    #[serde(rename="PHISHING_REUSE")]
    PHISHINGREUSE,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PASSWORDPROTECTIONTRIGGERUNSPECIFIED => "PASSWORD_PROTECTION_TRIGGER_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PROTECTIONOFF => "PROTECTION_OFF",
            GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PASSWORDREUSE => "PASSWORD_REUSE",
            GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PHISHINGREUSE => "PHISHING_REUSE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASSWORD_PROTECTION_TRIGGER_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PASSWORDPROTECTIONTRIGGERUNSPECIFIED),
           "PROTECTION_OFF" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PROTECTIONOFF),
           "PASSWORD_REUSE" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PASSWORDREUSE),
           "PHISHING_REUSE" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum::PHISHINGREUSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1BrowserInfoPasswordProtectionWarningTriggerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current state of [Safe Browsing protection level](https://chromeenterprise.google/policies/#SafeBrowsingProtectionLevel).
pub enum GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum {
    

    /// Browser protection level is not specified.
    ///
    /// "SAFE_BROWSING_LEVEL_UNSPECIFIED"
    #[serde(rename="SAFE_BROWSING_LEVEL_UNSPECIFIED")]
    SAFEBROWSINGLEVELUNSPECIFIED,
    

    /// No protection against dangerous websites, downloads, and extensions.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Standard protection against websites, downloads, and extensions that are known to be dangerous.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Faster, proactive protection against dangerous websites, downloads, and extensions.
    ///
    /// "ENHANCED"
    #[serde(rename="ENHANCED")]
    ENHANCED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::SAFEBROWSINGLEVELUNSPECIFIED => "SAFE_BROWSING_LEVEL_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::DISABLED => "DISABLED",
            GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::STANDARD => "STANDARD",
            GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::ENHANCED => "ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAFE_BROWSING_LEVEL_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::SAFEBROWSINGLEVELUNSPECIFIED),
           "DISABLED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::DISABLED),
           "STANDARD" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::STANDARD),
           "ENHANCED" => Ok(GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum::ENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1BrowserInfoSafeBrowsingProtectionLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Validation state of this certificate.
pub enum GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum {
    

    /// Default value.
    ///
    /// "CERTIFICATE_VALIDATION_STATE_UNSPECIFIED"
    #[serde(rename="CERTIFICATE_VALIDATION_STATE_UNSPECIFIED")]
    CERTIFICATEVALIDATIONSTATEUNSPECIFIED,
    

    /// Certificate validation was successful.
    ///
    /// "VALIDATION_SUCCESSFUL"
    #[serde(rename="VALIDATION_SUCCESSFUL")]
    VALIDATIONSUCCESSFUL,
    

    /// Certificate validation failed.
    ///
    /// "VALIDATION_FAILED"
    #[serde(rename="VALIDATION_FAILED")]
    VALIDATIONFAILED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum::CERTIFICATEVALIDATIONSTATEUNSPECIFIED => "CERTIFICATE_VALIDATION_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum::VALIDATIONSUCCESSFUL => "VALIDATION_SUCCESSFUL",
            GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum::VALIDATIONFAILED => "VALIDATION_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CERTIFICATE_VALIDATION_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum::CERTIFICATEVALIDATIONSTATEUNSPECIFIED),
           "VALIDATION_SUCCESSFUL" => Ok(GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum::VALIDATIONSUCCESSFUL),
           "VALIDATION_FAILED" => Ok(GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum::VALIDATIONFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1CertificateAttributeValidationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The compliance state of the resource as specified by the API client.
pub enum GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum {
    

    /// The compliance state of the resource is unknown or unspecified.
    ///
    /// "COMPLIANCE_STATE_UNSPECIFIED"
    #[serde(rename="COMPLIANCE_STATE_UNSPECIFIED")]
    COMPLIANCESTATEUNSPECIFIED,
    

    /// Device is compliant with third party policies
    ///
    /// "COMPLIANT"
    #[serde(rename="COMPLIANT")]
    COMPLIANT,
    

    /// Device is not compliant with third party policies
    ///
    /// "NON_COMPLIANT"
    #[serde(rename="NON_COMPLIANT")]
    NONCOMPLIANT,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum::COMPLIANCESTATEUNSPECIFIED => "COMPLIANCE_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum::COMPLIANT => "COMPLIANT",
            GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum::NONCOMPLIANT => "NON_COMPLIANT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPLIANCE_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum::COMPLIANCESTATEUNSPECIFIED),
           "COMPLIANT" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum::COMPLIANT),
           "NON_COMPLIANT" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum::NONCOMPLIANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies.
pub enum GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum {
    

    /// Default value
    ///
    /// "HEALTH_SCORE_UNSPECIFIED"
    #[serde(rename="HEALTH_SCORE_UNSPECIFIED")]
    HEALTHSCOREUNSPECIFIED,
    

    /// The object is in very poor health as defined by the caller.
    ///
    /// "VERY_POOR"
    #[serde(rename="VERY_POOR")]
    VERYPOOR,
    

    /// The object is in poor health as defined by the caller.
    ///
    /// "POOR"
    #[serde(rename="POOR")]
    POOR,
    

    /// The object health is neither good nor poor, as defined by the caller.
    ///
    /// "NEUTRAL"
    #[serde(rename="NEUTRAL")]
    NEUTRAL,
    

    /// The object is in good health as defined by the caller.
    ///
    /// "GOOD"
    #[serde(rename="GOOD")]
    GOOD,
    

    /// The object is in very good health as defined by the caller.
    ///
    /// "VERY_GOOD"
    #[serde(rename="VERY_GOOD")]
    VERYGOOD,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::HEALTHSCOREUNSPECIFIED => "HEALTH_SCORE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::VERYPOOR => "VERY_POOR",
            GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::POOR => "POOR",
            GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::NEUTRAL => "NEUTRAL",
            GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::GOOD => "GOOD",
            GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::VERYGOOD => "VERY_GOOD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEALTH_SCORE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::HEALTHSCOREUNSPECIFIED),
           "VERY_POOR" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::VERYPOOR),
           "POOR" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::POOR),
           "NEUTRAL" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::NEUTRAL),
           "GOOD" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::GOOD),
           "VERY_GOOD" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum::VERYGOOD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The management state of the resource as specified by the API client.
pub enum GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum {
    

    /// The management state of the resource is unknown or unspecified.
    ///
    /// "MANAGED_STATE_UNSPECIFIED"
    #[serde(rename="MANAGED_STATE_UNSPECIFIED")]
    MANAGEDSTATEUNSPECIFIED,
    

    /// The resource is managed.
    ///
    /// "MANAGED"
    #[serde(rename="MANAGED")]
    MANAGED,
    

    /// The resource is not managed.
    ///
    /// "UNMANAGED"
    #[serde(rename="UNMANAGED")]
    UNMANAGED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum::MANAGEDSTATEUNSPECIFIED => "MANAGED_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum::MANAGED => "MANAGED",
            GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum::UNMANAGED => "UNMANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGED_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum::MANAGEDSTATEUNSPECIFIED),
           "MANAGED" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum::MANAGED),
           "UNMANAGED" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum::UNMANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The owner of the ClientState
pub enum GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum {
    

    /// Unknown owner type
    ///
    /// "OWNER_TYPE_UNSPECIFIED"
    #[serde(rename="OWNER_TYPE_UNSPECIFIED")]
    OWNERTYPEUNSPECIFIED,
    

    /// Customer is the owner
    ///
    /// "OWNER_TYPE_CUSTOMER"
    #[serde(rename="OWNER_TYPE_CUSTOMER")]
    OWNERTYPECUSTOMER,
    

    /// Partner is the owner
    ///
    /// "OWNER_TYPE_PARTNER"
    #[serde(rename="OWNER_TYPE_PARTNER")]
    OWNERTYPEPARTNER,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum::OWNERTYPEUNSPECIFIED => "OWNER_TYPE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum::OWNERTYPECUSTOMER => "OWNER_TYPE_CUSTOMER",
            GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum::OWNERTYPEPARTNER => "OWNER_TYPE_PARTNER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OWNER_TYPE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum::OWNERTYPEUNSPECIFIED),
           "OWNER_TYPE_CUSTOMER" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum::OWNERTYPECUSTOMER),
           "OWNER_TYPE_PARTNER" => Ok(GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum::OWNERTYPEPARTNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Represents whether the Device is compromised.
pub enum GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum {
    

    /// Default value.
    ///
    /// "COMPROMISED_STATE_UNSPECIFIED"
    #[serde(rename="COMPROMISED_STATE_UNSPECIFIED")]
    COMPROMISEDSTATEUNSPECIFIED,
    

    /// The device is compromised (currently, this means Android device is rooted).
    ///
    /// "COMPROMISED"
    #[serde(rename="COMPROMISED")]
    COMPROMISED,
    

    /// The device is safe (currently, this means Android device is unrooted).
    ///
    /// "UNCOMPROMISED"
    #[serde(rename="UNCOMPROMISED")]
    UNCOMPROMISED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum::COMPROMISEDSTATEUNSPECIFIED => "COMPROMISED_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum::COMPROMISED => "COMPROMISED",
            GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum::UNCOMPROMISED => "UNCOMPROMISED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPROMISED_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum::COMPROMISEDSTATEUNSPECIFIED),
           "COMPROMISED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum::COMPROMISED),
           "UNCOMPROMISED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum::UNCOMPROMISED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of device.
pub enum GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum {
    

    /// Unknown device type
    ///
    /// "DEVICE_TYPE_UNSPECIFIED"
    #[serde(rename="DEVICE_TYPE_UNSPECIFIED")]
    DEVICETYPEUNSPECIFIED,
    

    /// Device is an Android device
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Device is an iOS device
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Device is a Google Sync device.
    ///
    /// "GOOGLE_SYNC"
    #[serde(rename="GOOGLE_SYNC")]
    GOOGLESYNC,
    

    /// Device is a Windows device.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Device is a MacOS device.
    ///
    /// "MAC_OS"
    #[serde(rename="MAC_OS")]
    MACOS,
    

    /// Device is a Linux device.
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Device is a ChromeOS device.
    ///
    /// "CHROME_OS"
    #[serde(rename="CHROME_OS")]
    CHROMEOS,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::DEVICETYPEUNSPECIFIED => "DEVICE_TYPE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::ANDROID => "ANDROID",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::IOS => "IOS",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::GOOGLESYNC => "GOOGLE_SYNC",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::WINDOWS => "WINDOWS",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::MACOS => "MAC_OS",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::LINUX => "LINUX",
            GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::CHROMEOS => "CHROME_OS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_TYPE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::DEVICETYPEUNSPECIFIED),
           "ANDROID" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::ANDROID),
           "IOS" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::IOS),
           "GOOGLE_SYNC" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::GOOGLESYNC),
           "WINDOWS" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::WINDOWS),
           "MAC_OS" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::MACOS),
           "LINUX" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::LINUX),
           "CHROME_OS" => Ok(GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum::CHROMEOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Device encryption state.
pub enum GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum {
    

    /// Encryption Status is not set.
    ///
    /// "ENCRYPTION_STATE_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_STATE_UNSPECIFIED")]
    ENCRYPTIONSTATEUNSPECIFIED,
    

    /// Device doesn't support encryption.
    ///
    /// "UNSUPPORTED_BY_DEVICE"
    #[serde(rename="UNSUPPORTED_BY_DEVICE")]
    UNSUPPORTEDBYDEVICE,
    

    /// Device is encrypted.
    ///
    /// "ENCRYPTED"
    #[serde(rename="ENCRYPTED")]
    ENCRYPTED,
    

    /// Device is not encrypted.
    ///
    /// "NOT_ENCRYPTED"
    #[serde(rename="NOT_ENCRYPTED")]
    NOTENCRYPTED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::ENCRYPTIONSTATEUNSPECIFIED => "ENCRYPTION_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::UNSUPPORTEDBYDEVICE => "UNSUPPORTED_BY_DEVICE",
            GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::ENCRYPTED => "ENCRYPTED",
            GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::NOTENCRYPTED => "NOT_ENCRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::ENCRYPTIONSTATEUNSPECIFIED),
           "UNSUPPORTED_BY_DEVICE" => Ok(GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::UNSUPPORTEDBYDEVICE),
           "ENCRYPTED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::ENCRYPTED),
           "NOT_ENCRYPTED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum::NOTENCRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Management state of the device
pub enum GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "MANAGEMENT_STATE_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_STATE_UNSPECIFIED")]
    MANAGEMENTSTATEUNSPECIFIED,
    

    /// Device is approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Device is blocked.
    ///
    /// "BLOCKED"
    #[serde(rename="BLOCKED")]
    BLOCKED,
    

    /// Device is pending approval.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The device is not provisioned. Device will start from this state until some action is taken (i.e. a user starts using the device).
    ///
    /// "UNPROVISIONED"
    #[serde(rename="UNPROVISIONED")]
    UNPROVISIONED,
    

    /// Data and settings on the device are being removed.
    ///
    /// "WIPING"
    #[serde(rename="WIPING")]
    WIPING,
    

    /// All data and settings on the device are removed.
    ///
    /// "WIPED"
    #[serde(rename="WIPED")]
    WIPED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::MANAGEMENTSTATEUNSPECIFIED => "MANAGEMENT_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::APPROVED => "APPROVED",
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::BLOCKED => "BLOCKED",
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::PENDING => "PENDING",
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::UNPROVISIONED => "UNPROVISIONED",
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::WIPING => "WIPING",
            GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::WIPED => "WIPED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::MANAGEMENTSTATEUNSPECIFIED),
           "APPROVED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::APPROVED),
           "BLOCKED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::BLOCKED),
           "PENDING" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::PENDING),
           "UNPROVISIONED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::UNPROVISIONED),
           "WIPING" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::WIPING),
           "WIPED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum::WIPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Whether the device is owned by the company or an individual
pub enum GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum {
    

    /// Default value. The value is unused.
    ///
    /// "DEVICE_OWNERSHIP_UNSPECIFIED"
    #[serde(rename="DEVICE_OWNERSHIP_UNSPECIFIED")]
    DEVICEOWNERSHIPUNSPECIFIED,
    

    /// Company owns the device.
    ///
    /// "COMPANY"
    #[serde(rename="COMPANY")]
    COMPANY,
    

    /// Bring Your Own Device (i.e. individual owns the device)
    ///
    /// "BYOD"
    #[serde(rename="BYOD")]
    BYOD,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum::DEVICEOWNERSHIPUNSPECIFIED => "DEVICE_OWNERSHIP_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum::COMPANY => "COMPANY",
            GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum::BYOD => "BYOD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEVICE_OWNERSHIP_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum::DEVICEOWNERSHIPUNSPECIFIED),
           "COMPANY" => Ok(GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum::COMPANY),
           "BYOD" => Ok(GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum::BYOD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compromised State of the DeviceUser object
pub enum GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum {
    

    /// Compromised state of Device User account is unknown or unspecified.
    ///
    /// "COMPROMISED_STATE_UNSPECIFIED"
    #[serde(rename="COMPROMISED_STATE_UNSPECIFIED")]
    COMPROMISEDSTATEUNSPECIFIED,
    

    /// Device User Account is compromised.
    ///
    /// "COMPROMISED"
    #[serde(rename="COMPROMISED")]
    COMPROMISED,
    

    /// Device User Account is not compromised.
    ///
    /// "NOT_COMPROMISED"
    #[serde(rename="NOT_COMPROMISED")]
    NOTCOMPROMISED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum::COMPROMISEDSTATEUNSPECIFIED => "COMPROMISED_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum::COMPROMISED => "COMPROMISED",
            GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum::NOTCOMPROMISED => "NOT_COMPROMISED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPROMISED_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum::COMPROMISEDSTATEUNSPECIFIED),
           "COMPROMISED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum::COMPROMISED),
           "NOT_COMPROMISED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum::NOTCOMPROMISED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Management state of the user on the device.
pub enum GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "MANAGEMENT_STATE_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_STATE_UNSPECIFIED")]
    MANAGEMENTSTATEUNSPECIFIED,
    

    /// This user's data and profile is being removed from the device.
    ///
    /// "WIPING"
    #[serde(rename="WIPING")]
    WIPING,
    

    /// This user's data and profile is removed from the device.
    ///
    /// "WIPED"
    #[serde(rename="WIPED")]
    WIPED,
    

    /// User is approved to access data on the device.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// User is blocked from accessing data on the device.
    ///
    /// "BLOCKED"
    #[serde(rename="BLOCKED")]
    BLOCKED,
    

    /// User is awaiting approval.
    ///
    /// "PENDING_APPROVAL"
    #[serde(rename="PENDING_APPROVAL")]
    PENDINGAPPROVAL,
    

    /// User is unenrolled from Advanced Windows Management, but the Windows account is still intact.
    ///
    /// "UNENROLLED"
    #[serde(rename="UNENROLLED")]
    UNENROLLED,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::MANAGEMENTSTATEUNSPECIFIED => "MANAGEMENT_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::WIPING => "WIPING",
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::WIPED => "WIPED",
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::APPROVED => "APPROVED",
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::BLOCKED => "BLOCKED",
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::PENDINGAPPROVAL => "PENDING_APPROVAL",
            GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::UNENROLLED => "UNENROLLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::MANAGEMENTSTATEUNSPECIFIED),
           "WIPING" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::WIPING),
           "WIPED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::WIPED),
           "APPROVED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::APPROVED),
           "BLOCKED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::BLOCKED),
           "PENDING_APPROVAL" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::PENDINGAPPROVAL),
           "UNENROLLED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum::UNENROLLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Password state of the DeviceUser object
pub enum GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum {
    

    /// Password state not set.
    ///
    /// "PASSWORD_STATE_UNSPECIFIED"
    #[serde(rename="PASSWORD_STATE_UNSPECIFIED")]
    PASSWORDSTATEUNSPECIFIED,
    

    /// Password set in object.
    ///
    /// "PASSWORD_SET"
    #[serde(rename="PASSWORD_SET")]
    PASSWORDSET,
    

    /// Password not set in object.
    ///
    /// "PASSWORD_NOT_SET"
    #[serde(rename="PASSWORD_NOT_SET")]
    PASSWORDNOTSET,
}

impl AsRef<str> for GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum::PASSWORDSTATEUNSPECIFIED => "PASSWORD_STATE_UNSPECIFIED",
            GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum::PASSWORDSET => "PASSWORD_SET",
            GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum::PASSWORDNOTSET => "PASSWORD_NOT_SET",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PASSWORD_STATE_UNSPECIFIED" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum::PASSWORDSTATEUNSPECIFIED),
           "PASSWORD_SET" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum::PASSWORDSET),
           "PASSWORD_NOT_SET" => Ok(GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum::PASSWORDNOTSET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupRelationRelationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relation between the member and the transitive group.
pub enum GroupRelationRelationTypeEnum {
    

    /// The relation type is undefined or undetermined.
    ///
    /// "RELATION_TYPE_UNSPECIFIED"
    #[serde(rename="RELATION_TYPE_UNSPECIFIED")]
    RELATIONTYPEUNSPECIFIED,
    

    /// The two entities have only a direct membership with each other.
    ///
    /// "DIRECT"
    #[serde(rename="DIRECT")]
    DIRECT,
    

    /// The two entities have only an indirect membership with each other.
    ///
    /// "INDIRECT"
    #[serde(rename="INDIRECT")]
    INDIRECT,
    

    /// The two entities have both a direct and an indirect membership with each other.
    ///
    /// "DIRECT_AND_INDIRECT"
    #[serde(rename="DIRECT_AND_INDIRECT")]
    DIRECTANDINDIRECT,
}

impl AsRef<str> for GroupRelationRelationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupRelationRelationTypeEnum::RELATIONTYPEUNSPECIFIED => "RELATION_TYPE_UNSPECIFIED",
            GroupRelationRelationTypeEnum::DIRECT => "DIRECT",
            GroupRelationRelationTypeEnum::INDIRECT => "INDIRECT",
            GroupRelationRelationTypeEnum::DIRECTANDINDIRECT => "DIRECT_AND_INDIRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupRelationRelationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATION_TYPE_UNSPECIFIED" => Ok(GroupRelationRelationTypeEnum::RELATIONTYPEUNSPECIFIED),
           "DIRECT" => Ok(GroupRelationRelationTypeEnum::DIRECT),
           "INDIRECT" => Ok(GroupRelationRelationTypeEnum::INDIRECT),
           "DIRECT_AND_INDIRECT" => Ok(GroupRelationRelationTypeEnum::DIRECTANDINDIRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupRelationRelationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InboundSsoAssignmentSsoModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Inbound SSO behavior.
pub enum InboundSsoAssignmentSsoModeEnum {
    

    /// Not allowed.
    ///
    /// "SSO_MODE_UNSPECIFIED"
    #[serde(rename="SSO_MODE_UNSPECIFIED")]
    SSOMODEUNSPECIFIED,
    

    /// Disable SSO for the targeted users.
    ///
    /// "SSO_OFF"
    #[serde(rename="SSO_OFF")]
    SSOOFF,
    

    /// Use an external SAML Identity Provider for SSO for the targeted users.
    ///
    /// "SAML_SSO"
    #[serde(rename="SAML_SSO")]
    SAMLSSO,
    

    /// Use the domain-wide SAML Identity Provider for the targeted users if one is configured; otherwise, this is equivalent to `SSO_OFF`. Note that this will also be equivalent to `SSO_OFF` if/when support for domain-wide SAML is removed. Google may disallow this mode at that point and existing assignments with this mode may be automatically changed to `SSO_OFF`.
    ///
    /// "DOMAIN_WIDE_SAML_IF_ENABLED"
    #[serde(rename="DOMAIN_WIDE_SAML_IF_ENABLED")]
    DOMAINWIDESAMLIFENABLED,
}

impl AsRef<str> for InboundSsoAssignmentSsoModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InboundSsoAssignmentSsoModeEnum::SSOMODEUNSPECIFIED => "SSO_MODE_UNSPECIFIED",
            InboundSsoAssignmentSsoModeEnum::SSOOFF => "SSO_OFF",
            InboundSsoAssignmentSsoModeEnum::SAMLSSO => "SAML_SSO",
            InboundSsoAssignmentSsoModeEnum::DOMAINWIDESAMLIFENABLED => "DOMAIN_WIDE_SAML_IF_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for InboundSsoAssignmentSsoModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSO_MODE_UNSPECIFIED" => Ok(InboundSsoAssignmentSsoModeEnum::SSOMODEUNSPECIFIED),
           "SSO_OFF" => Ok(InboundSsoAssignmentSsoModeEnum::SSOOFF),
           "SAML_SSO" => Ok(InboundSsoAssignmentSsoModeEnum::SAMLSSO),
           "DOMAIN_WIDE_SAML_IF_ENABLED" => Ok(InboundSsoAssignmentSsoModeEnum::DOMAINWIDESAMLIFENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InboundSsoAssignmentSsoModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MemberRelationRelationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relation between the group and the transitive member.
pub enum MemberRelationRelationTypeEnum {
    

    /// The relation type is undefined or undetermined.
    ///
    /// "RELATION_TYPE_UNSPECIFIED"
    #[serde(rename="RELATION_TYPE_UNSPECIFIED")]
    RELATIONTYPEUNSPECIFIED,
    

    /// The two entities have only a direct membership with each other.
    ///
    /// "DIRECT"
    #[serde(rename="DIRECT")]
    DIRECT,
    

    /// The two entities have only an indirect membership with each other.
    ///
    /// "INDIRECT"
    #[serde(rename="INDIRECT")]
    INDIRECT,
    

    /// The two entities have both a direct and an indirect membership with each other.
    ///
    /// "DIRECT_AND_INDIRECT"
    #[serde(rename="DIRECT_AND_INDIRECT")]
    DIRECTANDINDIRECT,
}

impl AsRef<str> for MemberRelationRelationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MemberRelationRelationTypeEnum::RELATIONTYPEUNSPECIFIED => "RELATION_TYPE_UNSPECIFIED",
            MemberRelationRelationTypeEnum::DIRECT => "DIRECT",
            MemberRelationRelationTypeEnum::INDIRECT => "INDIRECT",
            MemberRelationRelationTypeEnum::DIRECTANDINDIRECT => "DIRECT_AND_INDIRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for MemberRelationRelationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELATION_TYPE_UNSPECIFIED" => Ok(MemberRelationRelationTypeEnum::RELATIONTYPEUNSPECIFIED),
           "DIRECT" => Ok(MemberRelationRelationTypeEnum::DIRECT),
           "INDIRECT" => Ok(MemberRelationRelationTypeEnum::INDIRECT),
           "DIRECT_AND_INDIRECT" => Ok(MemberRelationRelationTypeEnum::DIRECTANDINDIRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MemberRelationRelationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MembershipDeliverySettingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Delivery setting associated with the membership.
pub enum MembershipDeliverySettingEnum {
    

    /// Default. Should not be used.
    ///
    /// "DELIVERY_SETTING_UNSPECIFIED"
    #[serde(rename="DELIVERY_SETTING_UNSPECIFIED")]
    DELIVERYSETTINGUNSPECIFIED,
    

    /// Represents each mail should be delivered
    ///
    /// "ALL_MAIL"
    #[serde(rename="ALL_MAIL")]
    ALLMAIL,
    

    /// Represents 1 email for every 25 messages.
    ///
    /// "DIGEST"
    #[serde(rename="DIGEST")]
    DIGEST,
    

    /// Represents daily summary of messages.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
    

    /// Represents no delivery.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Represents disabled state.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for MembershipDeliverySettingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MembershipDeliverySettingEnum::DELIVERYSETTINGUNSPECIFIED => "DELIVERY_SETTING_UNSPECIFIED",
            MembershipDeliverySettingEnum::ALLMAIL => "ALL_MAIL",
            MembershipDeliverySettingEnum::DIGEST => "DIGEST",
            MembershipDeliverySettingEnum::DAILY => "DAILY",
            MembershipDeliverySettingEnum::NONE => "NONE",
            MembershipDeliverySettingEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for MembershipDeliverySettingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELIVERY_SETTING_UNSPECIFIED" => Ok(MembershipDeliverySettingEnum::DELIVERYSETTINGUNSPECIFIED),
           "ALL_MAIL" => Ok(MembershipDeliverySettingEnum::ALLMAIL),
           "DIGEST" => Ok(MembershipDeliverySettingEnum::DIGEST),
           "DAILY" => Ok(MembershipDeliverySettingEnum::DAILY),
           "NONE" => Ok(MembershipDeliverySettingEnum::NONE),
           "DISABLED" => Ok(MembershipDeliverySettingEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MembershipDeliverySettingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MembershipTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the membership.
pub enum MembershipTypeEnum {
    

    /// Default. Should not be used.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Represents user type.
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
    

    /// Represents service account type.
    ///
    /// "SERVICE_ACCOUNT"
    #[serde(rename="SERVICE_ACCOUNT")]
    SERVICEACCOUNT,
    

    /// Represents group type.
    ///
    /// "GROUP"
    #[serde(rename="GROUP")]
    GROUP,
    

    /// Represents Shared drive.
    ///
    /// "SHARED_DRIVE"
    #[serde(rename="SHARED_DRIVE")]
    SHAREDDRIVE,
    

    /// Represents other type.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for MembershipTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MembershipTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            MembershipTypeEnum::USER => "USER",
            MembershipTypeEnum::SERVICEACCOUNT => "SERVICE_ACCOUNT",
            MembershipTypeEnum::GROUP => "GROUP",
            MembershipTypeEnum::SHAREDDRIVE => "SHARED_DRIVE",
            MembershipTypeEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for MembershipTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(MembershipTypeEnum::TYPEUNSPECIFIED),
           "USER" => Ok(MembershipTypeEnum::USER),
           "SERVICE_ACCOUNT" => Ok(MembershipTypeEnum::SERVICEACCOUNT),
           "GROUP" => Ok(MembershipTypeEnum::GROUP),
           "SHARED_DRIVE" => Ok(MembershipTypeEnum::SHAREDDRIVE),
           "OTHER" => Ok(MembershipTypeEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MembershipTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MembershipRoleRestrictionEvaluationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the restriction
pub enum MembershipRoleRestrictionEvaluationStateEnum {
    

    /// Default. Should not be used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The member adheres to the parent group's restriction.
    ///
    /// "COMPLIANT"
    #[serde(rename="COMPLIANT")]
    COMPLIANT,
    

    /// The group-group membership might be currently violating some parent group's restriction but in future, it will never allow any new member in the child group which can violate parent group's restriction.
    ///
    /// "FORWARD_COMPLIANT"
    #[serde(rename="FORWARD_COMPLIANT")]
    FORWARDCOMPLIANT,
    

    /// The member violates the parent group's restriction.
    ///
    /// "NON_COMPLIANT"
    #[serde(rename="NON_COMPLIANT")]
    NONCOMPLIANT,
    

    /// The state of the membership is under evaluation.
    ///
    /// "EVALUATING"
    #[serde(rename="EVALUATING")]
    EVALUATING,
}

impl AsRef<str> for MembershipRoleRestrictionEvaluationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MembershipRoleRestrictionEvaluationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            MembershipRoleRestrictionEvaluationStateEnum::COMPLIANT => "COMPLIANT",
            MembershipRoleRestrictionEvaluationStateEnum::FORWARDCOMPLIANT => "FORWARD_COMPLIANT",
            MembershipRoleRestrictionEvaluationStateEnum::NONCOMPLIANT => "NON_COMPLIANT",
            MembershipRoleRestrictionEvaluationStateEnum::EVALUATING => "EVALUATING",
        }
    }
}

impl std::convert::TryFrom< &str> for MembershipRoleRestrictionEvaluationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(MembershipRoleRestrictionEvaluationStateEnum::STATEUNSPECIFIED),
           "COMPLIANT" => Ok(MembershipRoleRestrictionEvaluationStateEnum::COMPLIANT),
           "FORWARD_COMPLIANT" => Ok(MembershipRoleRestrictionEvaluationStateEnum::FORWARDCOMPLIANT),
           "NON_COMPLIANT" => Ok(MembershipRoleRestrictionEvaluationStateEnum::NONCOMPLIANT),
           "EVALUATING" => Ok(MembershipRoleRestrictionEvaluationStateEnum::EVALUATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MembershipRoleRestrictionEvaluationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestrictionEvaluationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the restriction
pub enum RestrictionEvaluationStateEnum {
    

    /// Default. Should not be used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The restriction state is currently being evaluated.
    ///
    /// "EVALUATING"
    #[serde(rename="EVALUATING")]
    EVALUATING,
    

    /// All transitive memberships are adhering to restriction.
    ///
    /// "COMPLIANT"
    #[serde(rename="COMPLIANT")]
    COMPLIANT,
    

    /// Some transitive memberships violate the restriction. No new violating memberships can be added.
    ///
    /// "FORWARD_COMPLIANT"
    #[serde(rename="FORWARD_COMPLIANT")]
    FORWARDCOMPLIANT,
    

    /// Some transitive memberships violate the restriction. New violating direct memberships will be denied while indirect memberships may be added.
    ///
    /// "NON_COMPLIANT"
    #[serde(rename="NON_COMPLIANT")]
    NONCOMPLIANT,
}

impl AsRef<str> for RestrictionEvaluationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestrictionEvaluationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RestrictionEvaluationStateEnum::EVALUATING => "EVALUATING",
            RestrictionEvaluationStateEnum::COMPLIANT => "COMPLIANT",
            RestrictionEvaluationStateEnum::FORWARDCOMPLIANT => "FORWARD_COMPLIANT",
            RestrictionEvaluationStateEnum::NONCOMPLIANT => "NON_COMPLIANT",
        }
    }
}

impl std::convert::TryFrom< &str> for RestrictionEvaluationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RestrictionEvaluationStateEnum::STATEUNSPECIFIED),
           "EVALUATING" => Ok(RestrictionEvaluationStateEnum::EVALUATING),
           "COMPLIANT" => Ok(RestrictionEvaluationStateEnum::COMPLIANT),
           "FORWARD_COMPLIANT" => Ok(RestrictionEvaluationStateEnum::FORWARDCOMPLIANT),
           "NON_COMPLIANT" => Ok(RestrictionEvaluationStateEnum::NONCOMPLIANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestrictionEvaluationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SignInBehaviorRedirectConditionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When to redirect sign-ins to the IdP.
pub enum SignInBehaviorRedirectConditionEnum {
    

    /// Default and means "always"
    ///
    /// "REDIRECT_CONDITION_UNSPECIFIED"
    #[serde(rename="REDIRECT_CONDITION_UNSPECIFIED")]
    REDIRECTCONDITIONUNSPECIFIED,
    

    /// Sign-in flows where the user is prompted for their identity will not redirect to the IdP (so the user will most likely be prompted by Google for a password), but special flows like IdP-initiated SAML and sign-in following automatic redirection to the IdP by domain-specific service URLs will accept the IdP's assertion of the user's identity.
    ///
    /// "NEVER"
    #[serde(rename="NEVER")]
    NEVER,
}

impl AsRef<str> for SignInBehaviorRedirectConditionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SignInBehaviorRedirectConditionEnum::REDIRECTCONDITIONUNSPECIFIED => "REDIRECT_CONDITION_UNSPECIFIED",
            SignInBehaviorRedirectConditionEnum::NEVER => "NEVER",
        }
    }
}

impl std::convert::TryFrom< &str> for SignInBehaviorRedirectConditionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDIRECT_CONDITION_UNSPECIFIED" => Ok(SignInBehaviorRedirectConditionEnum::REDIRECTCONDITIONUNSPECIFIED),
           "NEVER" => Ok(SignInBehaviorRedirectConditionEnum::NEVER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SignInBehaviorRedirectConditionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserInvitationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the `UserInvitation`.
pub enum UserInvitationStateEnum {
    

    /// The default value. This value is used if the state is omitted.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The `UserInvitation` has been created and is ready for sending as an email.
    ///
    /// "NOT_YET_SENT"
    #[serde(rename="NOT_YET_SENT")]
    NOTYETSENT,
    

    /// The user has been invited by email.
    ///
    /// "INVITED"
    #[serde(rename="INVITED")]
    INVITED,
    

    /// The user has accepted the invitation and is part of the organization.
    ///
    /// "ACCEPTED"
    #[serde(rename="ACCEPTED")]
    ACCEPTED,
    

    /// The user declined the invitation.
    ///
    /// "DECLINED"
    #[serde(rename="DECLINED")]
    DECLINED,
}

impl AsRef<str> for UserInvitationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserInvitationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            UserInvitationStateEnum::NOTYETSENT => "NOT_YET_SENT",
            UserInvitationStateEnum::INVITED => "INVITED",
            UserInvitationStateEnum::ACCEPTED => "ACCEPTED",
            UserInvitationStateEnum::DECLINED => "DECLINED",
        }
    }
}

impl std::convert::TryFrom< &str> for UserInvitationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(UserInvitationStateEnum::STATEUNSPECIFIED),
           "NOT_YET_SENT" => Ok(UserInvitationStateEnum::NOTYETSENT),
           "INVITED" => Ok(UserInvitationStateEnum::INVITED),
           "ACCEPTED" => Ok(UserInvitationStateEnum::ACCEPTED),
           "DECLINED" => Ok(UserInvitationStateEnum::DECLINED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserInvitationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeviceViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The view to use for the List request.
pub enum DeviceViewEnum {
    

    /// Default value. The value is unused.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// This view contains all devices imported by the company admin. Each device in the response contains all information specified by the company admin when importing the device (i.e. asset tags). This includes devices that may be unaassigned or assigned to users.
    ///
    /// "COMPANY_INVENTORY"
    #[serde(rename="COMPANY_INVENTORY")]
    COMPANYINVENTORY,
    

    /// This view contains all devices with at least one user registered on the device. Each device in the response contains all device information, except for asset tags.
    ///
    /// "USER_ASSIGNED_DEVICES"
    #[serde(rename="USER_ASSIGNED_DEVICES")]
    USERASSIGNEDDEVICES,
}

impl AsRef<str> for DeviceViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeviceViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            DeviceViewEnum::COMPANYINVENTORY => "COMPANY_INVENTORY",
            DeviceViewEnum::USERASSIGNEDDEVICES => "USER_ASSIGNED_DEVICES",
        }
    }
}

impl std::convert::TryFrom< &str> for DeviceViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(DeviceViewEnum::VIEWUNSPECIFIED),
           "COMPANY_INVENTORY" => Ok(DeviceViewEnum::COMPANYINVENTORY),
           "USER_ASSIGNED_DEVICES" => Ok(DeviceViewEnum::USERASSIGNEDDEVICES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeviceViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The level of detail to be returned. If unspecified, defaults to `View.BASIC`.
pub enum GroupViewEnum {
    

    /// Default. Should not be used.
    ///
    /// "VIEW_UNSPECIFIED"
    #[serde(rename="VIEW_UNSPECIFIED")]
    VIEWUNSPECIFIED,
    

    /// Only basic resource information is returned.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// All resource information is returned.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for GroupViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupViewEnum::VIEWUNSPECIFIED => "VIEW_UNSPECIFIED",
            GroupViewEnum::BASIC => "BASIC",
            GroupViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_UNSPECIFIED" => Ok(GroupViewEnum::VIEWUNSPECIFIED),
           "BASIC" => Ok(GroupViewEnum::BASIC),
           "FULL" => Ok(GroupViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GroupInitialGroupConfigEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The initial configuration option for the `Group`.
pub enum GroupInitialGroupConfigEnum {
    

    /// Default. Should not be used.
    ///
    /// "INITIAL_GROUP_CONFIG_UNSPECIFIED"
    #[serde(rename="INITIAL_GROUP_CONFIG_UNSPECIFIED")]
    INITIALGROUPCONFIGUNSPECIFIED,
    

    /// The end user making the request will be added as the initial owner of the `Group`.
    ///
    /// "WITH_INITIAL_OWNER"
    #[serde(rename="WITH_INITIAL_OWNER")]
    WITHINITIALOWNER,
    

    /// An empty group is created without any initial owners. This can only be used by admins of the domain.
    ///
    /// "EMPTY"
    #[serde(rename="EMPTY")]
    EMPTY,
}

impl AsRef<str> for GroupInitialGroupConfigEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GroupInitialGroupConfigEnum::INITIALGROUPCONFIGUNSPECIFIED => "INITIAL_GROUP_CONFIG_UNSPECIFIED",
            GroupInitialGroupConfigEnum::WITHINITIALOWNER => "WITH_INITIAL_OWNER",
            GroupInitialGroupConfigEnum::EMPTY => "EMPTY",
        }
    }
}

impl std::convert::TryFrom< &str> for GroupInitialGroupConfigEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INITIAL_GROUP_CONFIG_UNSPECIFIED" => Ok(GroupInitialGroupConfigEnum::INITIALGROUPCONFIGUNSPECIFIED),
           "WITH_INITIAL_OWNER" => Ok(GroupInitialGroupConfigEnum::WITHINITIALOWNER),
           "EMPTY" => Ok(GroupInitialGroupConfigEnum::EMPTY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GroupInitialGroupConfigEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


