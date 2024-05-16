use super::*;



// region GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The App Check enforcement mode for this resource. This will override the EnforcementMode setting on the parent service.
pub enum GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum {
    

    /// Firebase App Check is not enforced for the service, nor are App Check metrics collected. Though the service is not protected by App Check in this mode, other applicable protections, such as user authorization, are still enforced. An unconfigured service is in this mode by default.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// Firebase App Check is not enforced for the service. App Check metrics are collected to help you decide when to turn on enforcement for the service. Though the service is not protected by App Check in this mode, other applicable protections, such as user authorization, are still enforced. Some services require certain conditions to be met before they will work with App Check, such as requiring you to upgrade to a specific service tier. Until those requirements are met for a service, this `UNENFORCED` setting will have no effect and App Check will not work with that service.
    ///
    /// "UNENFORCED"
    #[serde(rename="UNENFORCED")]
    UNENFORCED,
    

    /// Firebase App Check is enforced for the service. The service will reject any request that attempts to access your project's resources if it does not have valid App Check token attached, with some exceptions depending on the service; for example, some services will still allow requests bearing the developer's privileged service account credentials without an App Check token. App Check metrics continue to be collected to help you detect issues with your App Check integration and monitor the composition of your callers. While the service is protected by App Check, other applicable protections, such as user authorization, continue to be enforced at the same time. Use caution when choosing to enforce App Check on a Firebase service. If your users have not updated to an App Check capable version of your app, their apps will no longer be able to use your Firebase services that are enforcing App Check. App Check metrics can help you decide whether to enforce App Check on your Firebase services. If your app has not launched yet, you should enable enforcement immediately, since there are no outdated clients in use. Some services require certain conditions to be met before they will work with App Check, such as requiring you to upgrade to a specific service tier. Until those requirements are met for a service, this `ENFORCED` setting will have no effect and App Check will not work with that service.
    ///
    /// "ENFORCED"
    #[serde(rename="ENFORCED")]
    ENFORCED,
}

impl AsRef<str> for GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum::OFF => "OFF",
            GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum::UNENFORCED => "UNENFORCED",
            GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum::ENFORCED => "ENFORCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFF" => Ok(GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum::OFF),
           "UNENFORCED" => Ok(GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum::UNENFORCED),
           "ENFORCED" => Ok(GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum::ENFORCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirebaseAppcheckV1betaResourcePolicyEnforcementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The App Check enforcement mode for this service.
pub enum GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum {
    

    /// Firebase App Check is not enforced for the service, nor are App Check metrics collected. Though the service is not protected by App Check in this mode, other applicable protections, such as user authorization, are still enforced. An unconfigured service is in this mode by default.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
    

    /// Firebase App Check is not enforced for the service. App Check metrics are collected to help you decide when to turn on enforcement for the service. Though the service is not protected by App Check in this mode, other applicable protections, such as user authorization, are still enforced. Some services require certain conditions to be met before they will work with App Check, such as requiring you to upgrade to a specific service tier. Until those requirements are met for a service, this `UNENFORCED` setting will have no effect and App Check will not work with that service.
    ///
    /// "UNENFORCED"
    #[serde(rename="UNENFORCED")]
    UNENFORCED,
    

    /// Firebase App Check is enforced for the service. The service will reject any request that attempts to access your project's resources if it does not have valid App Check token attached, with some exceptions depending on the service; for example, some services will still allow requests bearing the developer's privileged service account credentials without an App Check token. App Check metrics continue to be collected to help you detect issues with your App Check integration and monitor the composition of your callers. While the service is protected by App Check, other applicable protections, such as user authorization, continue to be enforced at the same time. Use caution when choosing to enforce App Check on a Firebase service. If your users have not updated to an App Check capable version of your app, their apps will no longer be able to use your Firebase services that are enforcing App Check. App Check metrics can help you decide whether to enforce App Check on your Firebase services. If your app has not launched yet, you should enable enforcement immediately, since there are no outdated clients in use. Some services require certain conditions to be met before they will work with App Check, such as requiring you to upgrade to a specific service tier. Until those requirements are met for a service, this `ENFORCED` setting will have no effect and App Check will not work with that service.
    ///
    /// "ENFORCED"
    #[serde(rename="ENFORCED")]
    ENFORCED,
}

impl AsRef<str> for GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum::OFF => "OFF",
            GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum::UNENFORCED => "UNENFORCED",
            GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum::ENFORCED => "ENFORCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFF" => Ok(GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum::OFF),
           "UNENFORCED" => Ok(GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum::UNENFORCED),
           "ENFORCED" => Ok(GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum::ENFORCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirebaseAppcheckV1betaServiceEnforcementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


