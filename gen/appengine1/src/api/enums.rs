use super::*;



// region ApiConfigHandlerAuthFailActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Action to take when users access resources that require authentication. Defaults to redirect.
pub enum ApiConfigHandlerAuthFailActionEnum {
    

    /// Not specified. AUTH_FAIL_ACTION_REDIRECT is assumed.
    ///
    /// "AUTH_FAIL_ACTION_UNSPECIFIED"
    #[serde(rename="AUTH_FAIL_ACTION_UNSPECIFIED")]
    AUTHFAILACTIONUNSPECIFIED,
    

    /// Redirects user to "accounts.google.com". The user is redirected back to the application URL after signing in or creating an account.
    ///
    /// "AUTH_FAIL_ACTION_REDIRECT"
    #[serde(rename="AUTH_FAIL_ACTION_REDIRECT")]
    AUTHFAILACTIONREDIRECT,
    

    /// Rejects request with a 401 HTTP status code and an error message.
    ///
    /// "AUTH_FAIL_ACTION_UNAUTHORIZED"
    #[serde(rename="AUTH_FAIL_ACTION_UNAUTHORIZED")]
    AUTHFAILACTIONUNAUTHORIZED,
}

impl AsRef<str> for ApiConfigHandlerAuthFailActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiConfigHandlerAuthFailActionEnum::AUTHFAILACTIONUNSPECIFIED => "AUTH_FAIL_ACTION_UNSPECIFIED",
            ApiConfigHandlerAuthFailActionEnum::AUTHFAILACTIONREDIRECT => "AUTH_FAIL_ACTION_REDIRECT",
            ApiConfigHandlerAuthFailActionEnum::AUTHFAILACTIONUNAUTHORIZED => "AUTH_FAIL_ACTION_UNAUTHORIZED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiConfigHandlerAuthFailActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_FAIL_ACTION_UNSPECIFIED" => Ok(ApiConfigHandlerAuthFailActionEnum::AUTHFAILACTIONUNSPECIFIED),
           "AUTH_FAIL_ACTION_REDIRECT" => Ok(ApiConfigHandlerAuthFailActionEnum::AUTHFAILACTIONREDIRECT),
           "AUTH_FAIL_ACTION_UNAUTHORIZED" => Ok(ApiConfigHandlerAuthFailActionEnum::AUTHFAILACTIONUNAUTHORIZED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiConfigHandlerAuthFailActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApiConfigHandlerLoginEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Level of login required to access this resource. Defaults to optional.
pub enum ApiConfigHandlerLoginEnum {
    

    /// Not specified. LOGIN_OPTIONAL is assumed.
    ///
    /// "LOGIN_UNSPECIFIED"
    #[serde(rename="LOGIN_UNSPECIFIED")]
    LOGINUNSPECIFIED,
    

    /// Does not require that the user is signed in.
    ///
    /// "LOGIN_OPTIONAL"
    #[serde(rename="LOGIN_OPTIONAL")]
    LOGINOPTIONAL,
    

    /// If the user is not signed in, the auth_fail_action is taken. In addition, if the user is not an administrator for the application, they are given an error message regardless of auth_fail_action. If the user is an administrator, the handler proceeds.
    ///
    /// "LOGIN_ADMIN"
    #[serde(rename="LOGIN_ADMIN")]
    LOGINADMIN,
    

    /// If the user has signed in, the handler proceeds normally. Otherwise, the auth_fail_action is taken.
    ///
    /// "LOGIN_REQUIRED"
    #[serde(rename="LOGIN_REQUIRED")]
    LOGINREQUIRED,
}

impl AsRef<str> for ApiConfigHandlerLoginEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiConfigHandlerLoginEnum::LOGINUNSPECIFIED => "LOGIN_UNSPECIFIED",
            ApiConfigHandlerLoginEnum::LOGINOPTIONAL => "LOGIN_OPTIONAL",
            ApiConfigHandlerLoginEnum::LOGINADMIN => "LOGIN_ADMIN",
            ApiConfigHandlerLoginEnum::LOGINREQUIRED => "LOGIN_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiConfigHandlerLoginEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGIN_UNSPECIFIED" => Ok(ApiConfigHandlerLoginEnum::LOGINUNSPECIFIED),
           "LOGIN_OPTIONAL" => Ok(ApiConfigHandlerLoginEnum::LOGINOPTIONAL),
           "LOGIN_ADMIN" => Ok(ApiConfigHandlerLoginEnum::LOGINADMIN),
           "LOGIN_REQUIRED" => Ok(ApiConfigHandlerLoginEnum::LOGINREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiConfigHandlerLoginEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApiConfigHandlerSecurityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Security (HTTPS) enforcement for this URL.
pub enum ApiConfigHandlerSecurityLevelEnum {
    

    /// Not specified.
    ///
    /// "SECURE_UNSPECIFIED"
    #[serde(rename="SECURE_UNSPECIFIED")]
    SECUREUNSPECIFIED,
    

    /// Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used, and respond accordingly.
    ///
    /// "SECURE_DEFAULT"
    #[serde(rename="SECURE_DEFAULT")]
    SECUREDEFAULT,
    

    /// Requests for a URL that match this handler that use HTTPS are automatically redirected to the HTTP equivalent URL.
    ///
    /// "SECURE_NEVER"
    #[serde(rename="SECURE_NEVER")]
    SECURENEVER,
    

    /// Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly.
    ///
    /// "SECURE_OPTIONAL"
    #[serde(rename="SECURE_OPTIONAL")]
    SECUREOPTIONAL,
    

    /// Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect.
    ///
    /// "SECURE_ALWAYS"
    #[serde(rename="SECURE_ALWAYS")]
    SECUREALWAYS,
}

impl AsRef<str> for ApiConfigHandlerSecurityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApiConfigHandlerSecurityLevelEnum::SECUREUNSPECIFIED => "SECURE_UNSPECIFIED",
            ApiConfigHandlerSecurityLevelEnum::SECUREDEFAULT => "SECURE_DEFAULT",
            ApiConfigHandlerSecurityLevelEnum::SECURENEVER => "SECURE_NEVER",
            ApiConfigHandlerSecurityLevelEnum::SECUREOPTIONAL => "SECURE_OPTIONAL",
            ApiConfigHandlerSecurityLevelEnum::SECUREALWAYS => "SECURE_ALWAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for ApiConfigHandlerSecurityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECURE_UNSPECIFIED" => Ok(ApiConfigHandlerSecurityLevelEnum::SECUREUNSPECIFIED),
           "SECURE_DEFAULT" => Ok(ApiConfigHandlerSecurityLevelEnum::SECUREDEFAULT),
           "SECURE_NEVER" => Ok(ApiConfigHandlerSecurityLevelEnum::SECURENEVER),
           "SECURE_OPTIONAL" => Ok(ApiConfigHandlerSecurityLevelEnum::SECUREOPTIONAL),
           "SECURE_ALWAYS" => Ok(ApiConfigHandlerSecurityLevelEnum::SECUREALWAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApiConfigHandlerSecurityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationDatabaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the Cloud Firestore or Cloud Datastore database associated with this application.
pub enum ApplicationDatabaseTypeEnum {
    

    /// Database type is unspecified.
    ///
    /// "DATABASE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_TYPE_UNSPECIFIED")]
    DATABASETYPEUNSPECIFIED,
    

    /// Cloud Datastore
    ///
    /// "CLOUD_DATASTORE"
    #[serde(rename="CLOUD_DATASTORE")]
    CLOUDDATASTORE,
    

    /// Cloud Firestore Native
    ///
    /// "CLOUD_FIRESTORE"
    #[serde(rename="CLOUD_FIRESTORE")]
    CLOUDFIRESTORE,
    

    /// Cloud Firestore in Datastore Mode
    ///
    /// "CLOUD_DATASTORE_COMPATIBILITY"
    #[serde(rename="CLOUD_DATASTORE_COMPATIBILITY")]
    CLOUDDATASTORECOMPATIBILITY,
}

impl AsRef<str> for ApplicationDatabaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationDatabaseTypeEnum::DATABASETYPEUNSPECIFIED => "DATABASE_TYPE_UNSPECIFIED",
            ApplicationDatabaseTypeEnum::CLOUDDATASTORE => "CLOUD_DATASTORE",
            ApplicationDatabaseTypeEnum::CLOUDFIRESTORE => "CLOUD_FIRESTORE",
            ApplicationDatabaseTypeEnum::CLOUDDATASTORECOMPATIBILITY => "CLOUD_DATASTORE_COMPATIBILITY",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationDatabaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_TYPE_UNSPECIFIED" => Ok(ApplicationDatabaseTypeEnum::DATABASETYPEUNSPECIFIED),
           "CLOUD_DATASTORE" => Ok(ApplicationDatabaseTypeEnum::CLOUDDATASTORE),
           "CLOUD_FIRESTORE" => Ok(ApplicationDatabaseTypeEnum::CLOUDFIRESTORE),
           "CLOUD_DATASTORE_COMPATIBILITY" => Ok(ApplicationDatabaseTypeEnum::CLOUDDATASTORECOMPATIBILITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationDatabaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationServingStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Serving status of this application.
pub enum ApplicationServingStatusEnum {
    

    /// Serving status is unspecified.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Application is serving.
    ///
    /// "SERVING"
    #[serde(rename="SERVING")]
    SERVING,
    

    /// Application has been disabled by the user.
    ///
    /// "USER_DISABLED"
    #[serde(rename="USER_DISABLED")]
    USERDISABLED,
    

    /// Application has been disabled by the system.
    ///
    /// "SYSTEM_DISABLED"
    #[serde(rename="SYSTEM_DISABLED")]
    SYSTEMDISABLED,
}

impl AsRef<str> for ApplicationServingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationServingStatusEnum::UNSPECIFIED => "UNSPECIFIED",
            ApplicationServingStatusEnum::SERVING => "SERVING",
            ApplicationServingStatusEnum::USERDISABLED => "USER_DISABLED",
            ApplicationServingStatusEnum::SYSTEMDISABLED => "SYSTEM_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationServingStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(ApplicationServingStatusEnum::UNSPECIFIED),
           "SERVING" => Ok(ApplicationServingStatusEnum::SERVING),
           "USER_DISABLED" => Ok(ApplicationServingStatusEnum::USERDISABLED),
           "SYSTEM_DISABLED" => Ok(ApplicationServingStatusEnum::SYSTEMDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationServingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointsApiServiceRolloutStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.
pub enum EndpointsApiServiceRolloutStrategyEnum {
    

    /// Not specified. Defaults to FIXED.
    ///
    /// "UNSPECIFIED_ROLLOUT_STRATEGY"
    #[serde(rename="UNSPECIFIED_ROLLOUT_STRATEGY")]
    UNSPECIFIEDROLLOUTSTRATEGY,
    

    /// Endpoints service configuration ID will be fixed to the configuration ID specified by config_id.
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// Endpoints service configuration ID will be updated with each rollout.
    ///
    /// "MANAGED"
    #[serde(rename="MANAGED")]
    MANAGED,
}

impl AsRef<str> for EndpointsApiServiceRolloutStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointsApiServiceRolloutStrategyEnum::UNSPECIFIEDROLLOUTSTRATEGY => "UNSPECIFIED_ROLLOUT_STRATEGY",
            EndpointsApiServiceRolloutStrategyEnum::FIXED => "FIXED",
            EndpointsApiServiceRolloutStrategyEnum::MANAGED => "MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointsApiServiceRolloutStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_ROLLOUT_STRATEGY" => Ok(EndpointsApiServiceRolloutStrategyEnum::UNSPECIFIEDROLLOUTSTRATEGY),
           "FIXED" => Ok(EndpointsApiServiceRolloutStrategyEnum::FIXED),
           "MANAGED" => Ok(EndpointsApiServiceRolloutStrategyEnum::MANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointsApiServiceRolloutStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ErrorHandlerErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error condition this handler applies to.
pub enum ErrorHandlerErrorCodeEnum {
    

    /// Not specified. ERROR_CODE_DEFAULT is assumed.
    ///
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// All other error types.
    ///
    /// "ERROR_CODE_DEFAULT"
    #[serde(rename="ERROR_CODE_DEFAULT")]
    ERRORCODEDEFAULT,
    

    /// Application has exceeded a resource quota.
    ///
    /// "ERROR_CODE_OVER_QUOTA"
    #[serde(rename="ERROR_CODE_OVER_QUOTA")]
    ERRORCODEOVERQUOTA,
    

    /// Client blocked by the application's Denial of Service protection configuration.
    ///
    /// "ERROR_CODE_DOS_API_DENIAL"
    #[serde(rename="ERROR_CODE_DOS_API_DENIAL")]
    ERRORCODEDOSAPIDENIAL,
    

    /// Deadline reached before the application responds.
    ///
    /// "ERROR_CODE_TIMEOUT"
    #[serde(rename="ERROR_CODE_TIMEOUT")]
    ERRORCODETIMEOUT,
}

impl AsRef<str> for ErrorHandlerErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ErrorHandlerErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            ErrorHandlerErrorCodeEnum::ERRORCODEDEFAULT => "ERROR_CODE_DEFAULT",
            ErrorHandlerErrorCodeEnum::ERRORCODEOVERQUOTA => "ERROR_CODE_OVER_QUOTA",
            ErrorHandlerErrorCodeEnum::ERRORCODEDOSAPIDENIAL => "ERROR_CODE_DOS_API_DENIAL",
            ErrorHandlerErrorCodeEnum::ERRORCODETIMEOUT => "ERROR_CODE_TIMEOUT",
        }
    }
}

impl std::convert::TryFrom< &str> for ErrorHandlerErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(ErrorHandlerErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "ERROR_CODE_DEFAULT" => Ok(ErrorHandlerErrorCodeEnum::ERRORCODEDEFAULT),
           "ERROR_CODE_OVER_QUOTA" => Ok(ErrorHandlerErrorCodeEnum::ERRORCODEOVERQUOTA),
           "ERROR_CODE_DOS_API_DENIAL" => Ok(ErrorHandlerErrorCodeEnum::ERRORCODEDOSAPIDENIAL),
           "ERROR_CODE_TIMEOUT" => Ok(ErrorHandlerErrorCodeEnum::ERRORCODETIMEOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ErrorHandlerErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FirewallRuleActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action to take on matched requests.
pub enum FirewallRuleActionEnum {
    
    /// "UNSPECIFIED_ACTION"
    #[serde(rename="UNSPECIFIED_ACTION")]
    UNSPECIFIEDACTION,
    

    /// Matching requests are allowed.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Matching requests are denied.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for FirewallRuleActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FirewallRuleActionEnum::UNSPECIFIEDACTION => "UNSPECIFIED_ACTION",
            FirewallRuleActionEnum::ALLOW => "ALLOW",
            FirewallRuleActionEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for FirewallRuleActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_ACTION" => Ok(FirewallRuleActionEnum::UNSPECIFIEDACTION),
           "ALLOW" => Ok(FirewallRuleActionEnum::ALLOW),
           "DENY" => Ok(FirewallRuleActionEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FirewallRuleActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceAvailabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Availability of the instance.
pub enum InstanceAvailabilityEnum {
    
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    
    /// "RESIDENT"
    #[serde(rename="RESIDENT")]
    RESIDENT,
    
    /// "DYNAMIC"
    #[serde(rename="DYNAMIC")]
    DYNAMIC,
}

impl AsRef<str> for InstanceAvailabilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceAvailabilityEnum::UNSPECIFIED => "UNSPECIFIED",
            InstanceAvailabilityEnum::RESIDENT => "RESIDENT",
            InstanceAvailabilityEnum::DYNAMIC => "DYNAMIC",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceAvailabilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(InstanceAvailabilityEnum::UNSPECIFIED),
           "RESIDENT" => Ok(InstanceAvailabilityEnum::RESIDENT),
           "DYNAMIC" => Ok(InstanceAvailabilityEnum::DYNAMIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceAvailabilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceVmLivenessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The liveness health check of this instance. Only applicable for instances in App Engine flexible environment.
pub enum InstanceVmLivenessEnum {
    

    /// There is no liveness health check for the instance. Only applicable for instances in App Engine standard environment.
    ///
    /// "LIVENESS_STATE_UNSPECIFIED"
    #[serde(rename="LIVENESS_STATE_UNSPECIFIED")]
    LIVENESSSTATEUNSPECIFIED,
    

    /// The health checking system is aware of the instance but its health is not known at the moment.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The instance is reachable i.e. a connection to the application health checking endpoint can be established, and conforms to the requirements defined by the health check.
    ///
    /// "HEALTHY"
    #[serde(rename="HEALTHY")]
    HEALTHY,
    

    /// The instance is reachable, but does not conform to the requirements defined by the health check.
    ///
    /// "UNHEALTHY"
    #[serde(rename="UNHEALTHY")]
    UNHEALTHY,
    

    /// The instance is being drained. The existing connections to the instance have time to complete, but the new ones are being refused.
    ///
    /// "DRAINING"
    #[serde(rename="DRAINING")]
    DRAINING,
    

    /// The instance is unreachable i.e. a connection to the application health checking endpoint cannot be established, or the server does not respond within the specified timeout.
    ///
    /// "TIMEOUT"
    #[serde(rename="TIMEOUT")]
    TIMEOUT,
}

impl AsRef<str> for InstanceVmLivenessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceVmLivenessEnum::LIVENESSSTATEUNSPECIFIED => "LIVENESS_STATE_UNSPECIFIED",
            InstanceVmLivenessEnum::UNKNOWN => "UNKNOWN",
            InstanceVmLivenessEnum::HEALTHY => "HEALTHY",
            InstanceVmLivenessEnum::UNHEALTHY => "UNHEALTHY",
            InstanceVmLivenessEnum::DRAINING => "DRAINING",
            InstanceVmLivenessEnum::TIMEOUT => "TIMEOUT",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceVmLivenessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVENESS_STATE_UNSPECIFIED" => Ok(InstanceVmLivenessEnum::LIVENESSSTATEUNSPECIFIED),
           "UNKNOWN" => Ok(InstanceVmLivenessEnum::UNKNOWN),
           "HEALTHY" => Ok(InstanceVmLivenessEnum::HEALTHY),
           "UNHEALTHY" => Ok(InstanceVmLivenessEnum::UNHEALTHY),
           "DRAINING" => Ok(InstanceVmLivenessEnum::DRAINING),
           "TIMEOUT" => Ok(InstanceVmLivenessEnum::TIMEOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceVmLivenessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManagedCertificateStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of certificate management. Refers to the most recent certificate acquisition or renewal attempt.@OutputOnly
pub enum ManagedCertificateStatusEnum {
    
    /// "MANAGEMENT_STATUS_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_STATUS_UNSPECIFIED")]
    MANAGEMENTSTATUSUNSPECIFIED,
    

    /// Certificate was successfully obtained and inserted into the serving system.
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// Certificate is under active attempts to acquire or renew.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Most recent renewal failed due to an invalid DNS setup and will be retried. Renewal attempts will continue to fail until the certificate domain's DNS configuration is fixed. The last successfully provisioned certificate may still be serving.
    ///
    /// "FAILED_RETRYING_NOT_VISIBLE"
    #[serde(rename="FAILED_RETRYING_NOT_VISIBLE")]
    FAILEDRETRYINGNOTVISIBLE,
    

    /// All renewal attempts have been exhausted, likely due to an invalid DNS setup.
    ///
    /// "FAILED_PERMANENT"
    #[serde(rename="FAILED_PERMANENT")]
    FAILEDPERMANENT,
    

    /// Most recent renewal failed due to an explicit CAA record that does not include one of the in-use CAs (Google CA and Let's Encrypt). Renewals will continue to fail until the CAA is reconfigured. The last successfully provisioned certificate may still be serving.
    ///
    /// "FAILED_RETRYING_CAA_FORBIDDEN"
    #[serde(rename="FAILED_RETRYING_CAA_FORBIDDEN")]
    FAILEDRETRYINGCAAFORBIDDEN,
    

    /// Most recent renewal failed due to a CAA retrieval failure. This means that the domain's DNS provider does not properly handle CAA records, failing requests for CAA records when no CAA records are defined. Renewals will continue to fail until the DNS provider is changed or a CAA record is added for the given domain. The last successfully provisioned certificate may still be serving.
    ///
    /// "FAILED_RETRYING_CAA_CHECKING"
    #[serde(rename="FAILED_RETRYING_CAA_CHECKING")]
    FAILEDRETRYINGCAACHECKING,
}

impl AsRef<str> for ManagedCertificateStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManagedCertificateStatusEnum::MANAGEMENTSTATUSUNSPECIFIED => "MANAGEMENT_STATUS_UNSPECIFIED",
            ManagedCertificateStatusEnum::OK => "OK",
            ManagedCertificateStatusEnum::PENDING => "PENDING",
            ManagedCertificateStatusEnum::FAILEDRETRYINGNOTVISIBLE => "FAILED_RETRYING_NOT_VISIBLE",
            ManagedCertificateStatusEnum::FAILEDPERMANENT => "FAILED_PERMANENT",
            ManagedCertificateStatusEnum::FAILEDRETRYINGCAAFORBIDDEN => "FAILED_RETRYING_CAA_FORBIDDEN",
            ManagedCertificateStatusEnum::FAILEDRETRYINGCAACHECKING => "FAILED_RETRYING_CAA_CHECKING",
        }
    }
}

impl std::convert::TryFrom< &str> for ManagedCertificateStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_STATUS_UNSPECIFIED" => Ok(ManagedCertificateStatusEnum::MANAGEMENTSTATUSUNSPECIFIED),
           "OK" => Ok(ManagedCertificateStatusEnum::OK),
           "PENDING" => Ok(ManagedCertificateStatusEnum::PENDING),
           "FAILED_RETRYING_NOT_VISIBLE" => Ok(ManagedCertificateStatusEnum::FAILEDRETRYINGNOTVISIBLE),
           "FAILED_PERMANENT" => Ok(ManagedCertificateStatusEnum::FAILEDPERMANENT),
           "FAILED_RETRYING_CAA_FORBIDDEN" => Ok(ManagedCertificateStatusEnum::FAILEDRETRYINGCAAFORBIDDEN),
           "FAILED_RETRYING_CAA_CHECKING" => Ok(ManagedCertificateStatusEnum::FAILEDRETRYINGCAACHECKING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManagedCertificateStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkInstanceIpModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The IP mode for instances. Only applicable in the App Engine flexible environment.
pub enum NetworkInstanceIpModeEnum {
    

    /// Unspecified is treated as EXTERNAL.
    ///
    /// "INSTANCE_IP_MODE_UNSPECIFIED"
    #[serde(rename="INSTANCE_IP_MODE_UNSPECIFIED")]
    INSTANCEIPMODEUNSPECIFIED,
    

    /// Instances are created with both internal and external IP addresses.
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
    

    /// Instances are created with internal IP addresses only.
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
}

impl AsRef<str> for NetworkInstanceIpModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkInstanceIpModeEnum::INSTANCEIPMODEUNSPECIFIED => "INSTANCE_IP_MODE_UNSPECIFIED",
            NetworkInstanceIpModeEnum::EXTERNAL => "EXTERNAL",
            NetworkInstanceIpModeEnum::INTERNAL => "INTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkInstanceIpModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSTANCE_IP_MODE_UNSPECIFIED" => Ok(NetworkInstanceIpModeEnum::INSTANCEIPMODEUNSPECIFIED),
           "EXTERNAL" => Ok(NetworkInstanceIpModeEnum::EXTERNAL),
           "INTERNAL" => Ok(NetworkInstanceIpModeEnum::INTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkInstanceIpModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkSettingIngressTrafficAllowedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ingress settings for version or service.
pub enum NetworkSettingIngressTrafficAllowedEnum {
    

    /// Unspecified
    ///
    /// "INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED"
    #[serde(rename="INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED")]
    INGRESSTRAFFICALLOWEDUNSPECIFIED,
    

    /// Allow HTTP traffic from public and private sources.
    ///
    /// "INGRESS_TRAFFIC_ALLOWED_ALL"
    #[serde(rename="INGRESS_TRAFFIC_ALLOWED_ALL")]
    INGRESSTRAFFICALLOWEDALL,
    

    /// Allow HTTP traffic from only private VPC sources.
    ///
    /// "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY"
    #[serde(rename="INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY")]
    INGRESSTRAFFICALLOWEDINTERNALONLY,
    

    /// Allow HTTP traffic from private VPC sources and through load balancers.
    ///
    /// "INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB"
    #[serde(rename="INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB")]
    INGRESSTRAFFICALLOWEDINTERNALANDLB,
}

impl AsRef<str> for NetworkSettingIngressTrafficAllowedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDUNSPECIFIED => "INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED",
            NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDALL => "INGRESS_TRAFFIC_ALLOWED_ALL",
            NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDINTERNALONLY => "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY",
            NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDINTERNALANDLB => "INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkSettingIngressTrafficAllowedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED" => Ok(NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDUNSPECIFIED),
           "INGRESS_TRAFFIC_ALLOWED_ALL" => Ok(NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDALL),
           "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY" => Ok(NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDINTERNALONLY),
           "INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB" => Ok(NetworkSettingIngressTrafficAllowedEnum::INGRESSTRAFFICALLOWEDINTERNALANDLB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkSettingIngressTrafficAllowedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ResourceRecordTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Resource record type. Example: AAAA.
pub enum ResourceRecordTypeEnum {
    

    /// An unknown resource record.
    ///
    /// "RECORD_TYPE_UNSPECIFIED"
    #[serde(rename="RECORD_TYPE_UNSPECIFIED")]
    RECORDTYPEUNSPECIFIED,
    

    /// An A resource record. Data is an IPv4 address.
    ///
    /// "A"
    #[serde(rename="A")]
    A,
    

    /// An AAAA resource record. Data is an IPv6 address.
    ///
    /// "AAAA"
    #[serde(rename="AAAA")]
    AAAA,
    

    /// A CNAME resource record. Data is a domain name to be aliased.
    ///
    /// "CNAME"
    #[serde(rename="CNAME")]
    CNAME,
}

impl AsRef<str> for ResourceRecordTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ResourceRecordTypeEnum::RECORDTYPEUNSPECIFIED => "RECORD_TYPE_UNSPECIFIED",
            ResourceRecordTypeEnum::A => "A",
            ResourceRecordTypeEnum::AAAA => "AAAA",
            ResourceRecordTypeEnum::CNAME => "CNAME",
        }
    }
}

impl std::convert::TryFrom< &str> for ResourceRecordTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RECORD_TYPE_UNSPECIFIED" => Ok(ResourceRecordTypeEnum::RECORDTYPEUNSPECIFIED),
           "A" => Ok(ResourceRecordTypeEnum::A),
           "AAAA" => Ok(ResourceRecordTypeEnum::AAAA),
           "CNAME" => Ok(ResourceRecordTypeEnum::CNAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ResourceRecordTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The environment of the runtime.
pub enum RuntimeEnvironmentEnum {
    

    /// Default value.
    ///
    /// "ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_UNSPECIFIED")]
    ENVIRONMENTUNSPECIFIED,
    

    /// App Engine Standard.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// App Engine Flexible
    ///
    /// "FLEXIBLE"
    #[serde(rename="FLEXIBLE")]
    FLEXIBLE,
}

impl AsRef<str> for RuntimeEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeEnvironmentEnum::ENVIRONMENTUNSPECIFIED => "ENVIRONMENT_UNSPECIFIED",
            RuntimeEnvironmentEnum::STANDARD => "STANDARD",
            RuntimeEnvironmentEnum::FLEXIBLE => "FLEXIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_UNSPECIFIED" => Ok(RuntimeEnvironmentEnum::ENVIRONMENTUNSPECIFIED),
           "STANDARD" => Ok(RuntimeEnvironmentEnum::STANDARD),
           "FLEXIBLE" => Ok(RuntimeEnvironmentEnum::FLEXIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The stage of life this runtime is in, e.g., BETA, GA, etc.
pub enum RuntimeStageEnum {
    

    /// Not specified.
    ///
    /// "RUNTIME_STAGE_UNSPECIFIED"
    #[serde(rename="RUNTIME_STAGE_UNSPECIFIED")]
    RUNTIMESTAGEUNSPECIFIED,
    

    /// The runtime is in development.
    ///
    /// "DEVELOPMENT"
    #[serde(rename="DEVELOPMENT")]
    DEVELOPMENT,
    

    /// The runtime is in the Alpha stage.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// The runtime is in the Beta stage.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// The runtime is generally available.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// The runtime is deprecated.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// The runtime is no longer supported.
    ///
    /// "DECOMMISSIONED"
    #[serde(rename="DECOMMISSIONED")]
    DECOMMISSIONED,
    

    /// The runtime is end of support.
    ///
    /// "END_OF_SUPPORT"
    #[serde(rename="END_OF_SUPPORT")]
    ENDOFSUPPORT,
}

impl AsRef<str> for RuntimeStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeStageEnum::RUNTIMESTAGEUNSPECIFIED => "RUNTIME_STAGE_UNSPECIFIED",
            RuntimeStageEnum::DEVELOPMENT => "DEVELOPMENT",
            RuntimeStageEnum::ALPHA => "ALPHA",
            RuntimeStageEnum::BETA => "BETA",
            RuntimeStageEnum::GA => "GA",
            RuntimeStageEnum::DEPRECATED => "DEPRECATED",
            RuntimeStageEnum::DECOMMISSIONED => "DECOMMISSIONED",
            RuntimeStageEnum::ENDOFSUPPORT => "END_OF_SUPPORT",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RUNTIME_STAGE_UNSPECIFIED" => Ok(RuntimeStageEnum::RUNTIMESTAGEUNSPECIFIED),
           "DEVELOPMENT" => Ok(RuntimeStageEnum::DEVELOPMENT),
           "ALPHA" => Ok(RuntimeStageEnum::ALPHA),
           "BETA" => Ok(RuntimeStageEnum::BETA),
           "GA" => Ok(RuntimeStageEnum::GA),
           "DEPRECATED" => Ok(RuntimeStageEnum::DEPRECATED),
           "DECOMMISSIONED" => Ok(RuntimeStageEnum::DECOMMISSIONED),
           "END_OF_SUPPORT" => Ok(RuntimeStageEnum::ENDOFSUPPORT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SslSettingSslManagementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// SSL management type for this domain. If AUTOMATIC, a managed certificate is automatically provisioned. If MANUAL, certificate_id must be manually specified in order to configure SSL for this domain.
pub enum SslSettingSslManagementTypeEnum {
    

    /// Defaults to AUTOMATIC.
    ///
    /// "SSL_MANAGEMENT_TYPE_UNSPECIFIED"
    #[serde(rename="SSL_MANAGEMENT_TYPE_UNSPECIFIED")]
    SSLMANAGEMENTTYPEUNSPECIFIED,
    

    /// SSL support for this domain is configured automatically. The mapped SSL certificate will be automatically renewed.
    ///
    /// "AUTOMATIC"
    #[serde(rename="AUTOMATIC")]
    AUTOMATIC,
    

    /// SSL support for this domain is configured manually by the user. Either the domain has no SSL support or a user-obtained SSL certificate has been explictly mapped to this domain.
    ///
    /// "MANUAL"
    #[serde(rename="MANUAL")]
    MANUAL,
}

impl AsRef<str> for SslSettingSslManagementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SslSettingSslManagementTypeEnum::SSLMANAGEMENTTYPEUNSPECIFIED => "SSL_MANAGEMENT_TYPE_UNSPECIFIED",
            SslSettingSslManagementTypeEnum::AUTOMATIC => "AUTOMATIC",
            SslSettingSslManagementTypeEnum::MANUAL => "MANUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for SslSettingSslManagementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSL_MANAGEMENT_TYPE_UNSPECIFIED" => Ok(SslSettingSslManagementTypeEnum::SSLMANAGEMENTTYPEUNSPECIFIED),
           "AUTOMATIC" => Ok(SslSettingSslManagementTypeEnum::AUTOMATIC),
           "MANUAL" => Ok(SslSettingSslManagementTypeEnum::MANUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SslSettingSslManagementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TrafficSplitShardByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed.
pub enum TrafficSplitShardByEnum {
    

    /// Diversion method unspecified.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Diversion based on a specially named cookie, "GOOGAPPUID." The cookie must be set by the application itself or no diversion will occur.
    ///
    /// "COOKIE"
    #[serde(rename="COOKIE")]
    COOKIE,
    

    /// Diversion based on applying the modulus operation to a fingerprint of the IP address.
    ///
    /// "IP"
    #[serde(rename="IP")]
    IP,
    

    /// Diversion based on weighted random assignment. An incoming request is randomly routed to a version in the traffic split, with probability proportional to the version's traffic share.
    ///
    /// "RANDOM"
    #[serde(rename="RANDOM")]
    RANDOM,
}

impl AsRef<str> for TrafficSplitShardByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrafficSplitShardByEnum::UNSPECIFIED => "UNSPECIFIED",
            TrafficSplitShardByEnum::COOKIE => "COOKIE",
            TrafficSplitShardByEnum::IP => "IP",
            TrafficSplitShardByEnum::RANDOM => "RANDOM",
        }
    }
}

impl std::convert::TryFrom< &str> for TrafficSplitShardByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(TrafficSplitShardByEnum::UNSPECIFIED),
           "COOKIE" => Ok(TrafficSplitShardByEnum::COOKIE),
           "IP" => Ok(TrafficSplitShardByEnum::IP),
           "RANDOM" => Ok(TrafficSplitShardByEnum::RANDOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TrafficSplitShardByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UrlMapAuthFailActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Action to take when users access resources that require authentication. Defaults to redirect.
pub enum UrlMapAuthFailActionEnum {
    

    /// Not specified. AUTH_FAIL_ACTION_REDIRECT is assumed.
    ///
    /// "AUTH_FAIL_ACTION_UNSPECIFIED"
    #[serde(rename="AUTH_FAIL_ACTION_UNSPECIFIED")]
    AUTHFAILACTIONUNSPECIFIED,
    

    /// Redirects user to "accounts.google.com". The user is redirected back to the application URL after signing in or creating an account.
    ///
    /// "AUTH_FAIL_ACTION_REDIRECT"
    #[serde(rename="AUTH_FAIL_ACTION_REDIRECT")]
    AUTHFAILACTIONREDIRECT,
    

    /// Rejects request with a 401 HTTP status code and an error message.
    ///
    /// "AUTH_FAIL_ACTION_UNAUTHORIZED"
    #[serde(rename="AUTH_FAIL_ACTION_UNAUTHORIZED")]
    AUTHFAILACTIONUNAUTHORIZED,
}

impl AsRef<str> for UrlMapAuthFailActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlMapAuthFailActionEnum::AUTHFAILACTIONUNSPECIFIED => "AUTH_FAIL_ACTION_UNSPECIFIED",
            UrlMapAuthFailActionEnum::AUTHFAILACTIONREDIRECT => "AUTH_FAIL_ACTION_REDIRECT",
            UrlMapAuthFailActionEnum::AUTHFAILACTIONUNAUTHORIZED => "AUTH_FAIL_ACTION_UNAUTHORIZED",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlMapAuthFailActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_FAIL_ACTION_UNSPECIFIED" => Ok(UrlMapAuthFailActionEnum::AUTHFAILACTIONUNSPECIFIED),
           "AUTH_FAIL_ACTION_REDIRECT" => Ok(UrlMapAuthFailActionEnum::AUTHFAILACTIONREDIRECT),
           "AUTH_FAIL_ACTION_UNAUTHORIZED" => Ok(UrlMapAuthFailActionEnum::AUTHFAILACTIONUNAUTHORIZED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlMapAuthFailActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UrlMapLoginEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Level of login required to access this resource. Not supported for Node.js in the App Engine standard environment.
pub enum UrlMapLoginEnum {
    

    /// Not specified. LOGIN_OPTIONAL is assumed.
    ///
    /// "LOGIN_UNSPECIFIED"
    #[serde(rename="LOGIN_UNSPECIFIED")]
    LOGINUNSPECIFIED,
    

    /// Does not require that the user is signed in.
    ///
    /// "LOGIN_OPTIONAL"
    #[serde(rename="LOGIN_OPTIONAL")]
    LOGINOPTIONAL,
    

    /// If the user is not signed in, the auth_fail_action is taken. In addition, if the user is not an administrator for the application, they are given an error message regardless of auth_fail_action. If the user is an administrator, the handler proceeds.
    ///
    /// "LOGIN_ADMIN"
    #[serde(rename="LOGIN_ADMIN")]
    LOGINADMIN,
    

    /// If the user has signed in, the handler proceeds normally. Otherwise, the auth_fail_action is taken.
    ///
    /// "LOGIN_REQUIRED"
    #[serde(rename="LOGIN_REQUIRED")]
    LOGINREQUIRED,
}

impl AsRef<str> for UrlMapLoginEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlMapLoginEnum::LOGINUNSPECIFIED => "LOGIN_UNSPECIFIED",
            UrlMapLoginEnum::LOGINOPTIONAL => "LOGIN_OPTIONAL",
            UrlMapLoginEnum::LOGINADMIN => "LOGIN_ADMIN",
            UrlMapLoginEnum::LOGINREQUIRED => "LOGIN_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlMapLoginEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGIN_UNSPECIFIED" => Ok(UrlMapLoginEnum::LOGINUNSPECIFIED),
           "LOGIN_OPTIONAL" => Ok(UrlMapLoginEnum::LOGINOPTIONAL),
           "LOGIN_ADMIN" => Ok(UrlMapLoginEnum::LOGINADMIN),
           "LOGIN_REQUIRED" => Ok(UrlMapLoginEnum::LOGINREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlMapLoginEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UrlMapRedirectHttpResponseCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// 30x code to use when performing redirects for the secure field. Defaults to 302.
pub enum UrlMapRedirectHttpResponseCodeEnum {
    

    /// Not specified. 302 is assumed.
    ///
    /// "REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED"
    #[serde(rename="REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED")]
    REDIRECTHTTPRESPONSECODEUNSPECIFIED,
    

    /// 301 Moved Permanently code.
    ///
    /// "REDIRECT_HTTP_RESPONSE_CODE_301"
    #[serde(rename="REDIRECT_HTTP_RESPONSE_CODE_301")]
    REDIRECTHTTPRESPONSECODE301,
    

    /// 302 Moved Temporarily code.
    ///
    /// "REDIRECT_HTTP_RESPONSE_CODE_302"
    #[serde(rename="REDIRECT_HTTP_RESPONSE_CODE_302")]
    REDIRECTHTTPRESPONSECODE302,
    

    /// 303 See Other code.
    ///
    /// "REDIRECT_HTTP_RESPONSE_CODE_303"
    #[serde(rename="REDIRECT_HTTP_RESPONSE_CODE_303")]
    REDIRECTHTTPRESPONSECODE303,
    

    /// 307 Temporary Redirect code.
    ///
    /// "REDIRECT_HTTP_RESPONSE_CODE_307"
    #[serde(rename="REDIRECT_HTTP_RESPONSE_CODE_307")]
    REDIRECTHTTPRESPONSECODE307,
}

impl AsRef<str> for UrlMapRedirectHttpResponseCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODEUNSPECIFIED => "REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED",
            UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE301 => "REDIRECT_HTTP_RESPONSE_CODE_301",
            UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE302 => "REDIRECT_HTTP_RESPONSE_CODE_302",
            UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE303 => "REDIRECT_HTTP_RESPONSE_CODE_303",
            UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE307 => "REDIRECT_HTTP_RESPONSE_CODE_307",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlMapRedirectHttpResponseCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED" => Ok(UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODEUNSPECIFIED),
           "REDIRECT_HTTP_RESPONSE_CODE_301" => Ok(UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE301),
           "REDIRECT_HTTP_RESPONSE_CODE_302" => Ok(UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE302),
           "REDIRECT_HTTP_RESPONSE_CODE_303" => Ok(UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE303),
           "REDIRECT_HTTP_RESPONSE_CODE_307" => Ok(UrlMapRedirectHttpResponseCodeEnum::REDIRECTHTTPRESPONSECODE307),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlMapRedirectHttpResponseCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UrlMapSecurityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Security (HTTPS) enforcement for this URL.
pub enum UrlMapSecurityLevelEnum {
    

    /// Not specified.
    ///
    /// "SECURE_UNSPECIFIED"
    #[serde(rename="SECURE_UNSPECIFIED")]
    SECUREUNSPECIFIED,
    

    /// Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used, and respond accordingly.
    ///
    /// "SECURE_DEFAULT"
    #[serde(rename="SECURE_DEFAULT")]
    SECUREDEFAULT,
    

    /// Requests for a URL that match this handler that use HTTPS are automatically redirected to the HTTP equivalent URL.
    ///
    /// "SECURE_NEVER"
    #[serde(rename="SECURE_NEVER")]
    SECURENEVER,
    

    /// Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly.
    ///
    /// "SECURE_OPTIONAL"
    #[serde(rename="SECURE_OPTIONAL")]
    SECUREOPTIONAL,
    

    /// Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect.
    ///
    /// "SECURE_ALWAYS"
    #[serde(rename="SECURE_ALWAYS")]
    SECUREALWAYS,
}

impl AsRef<str> for UrlMapSecurityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlMapSecurityLevelEnum::SECUREUNSPECIFIED => "SECURE_UNSPECIFIED",
            UrlMapSecurityLevelEnum::SECUREDEFAULT => "SECURE_DEFAULT",
            UrlMapSecurityLevelEnum::SECURENEVER => "SECURE_NEVER",
            UrlMapSecurityLevelEnum::SECUREOPTIONAL => "SECURE_OPTIONAL",
            UrlMapSecurityLevelEnum::SECUREALWAYS => "SECURE_ALWAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlMapSecurityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECURE_UNSPECIFIED" => Ok(UrlMapSecurityLevelEnum::SECUREUNSPECIFIED),
           "SECURE_DEFAULT" => Ok(UrlMapSecurityLevelEnum::SECUREDEFAULT),
           "SECURE_NEVER" => Ok(UrlMapSecurityLevelEnum::SECURENEVER),
           "SECURE_OPTIONAL" => Ok(UrlMapSecurityLevelEnum::SECUREOPTIONAL),
           "SECURE_ALWAYS" => Ok(UrlMapSecurityLevelEnum::SECUREALWAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlMapSecurityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VersionInboundServicesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Before an application can receive email or XMPP messages, the application must be configured to enable the service.
pub enum VersionInboundServicesEnum {
    

    /// Not specified.
    ///
    /// "INBOUND_SERVICE_UNSPECIFIED"
    #[serde(rename="INBOUND_SERVICE_UNSPECIFIED")]
    INBOUNDSERVICEUNSPECIFIED,
    

    /// Allows an application to receive mail.
    ///
    /// "INBOUND_SERVICE_MAIL"
    #[serde(rename="INBOUND_SERVICE_MAIL")]
    INBOUNDSERVICEMAIL,
    

    /// Allows an application to receive email-bound notifications.
    ///
    /// "INBOUND_SERVICE_MAIL_BOUNCE"
    #[serde(rename="INBOUND_SERVICE_MAIL_BOUNCE")]
    INBOUNDSERVICEMAILBOUNCE,
    

    /// Allows an application to receive error stanzas.
    ///
    /// "INBOUND_SERVICE_XMPP_ERROR"
    #[serde(rename="INBOUND_SERVICE_XMPP_ERROR")]
    INBOUNDSERVICEXMPPERROR,
    

    /// Allows an application to receive instant messages.
    ///
    /// "INBOUND_SERVICE_XMPP_MESSAGE"
    #[serde(rename="INBOUND_SERVICE_XMPP_MESSAGE")]
    INBOUNDSERVICEXMPPMESSAGE,
    

    /// Allows an application to receive user subscription POSTs.
    ///
    /// "INBOUND_SERVICE_XMPP_SUBSCRIBE"
    #[serde(rename="INBOUND_SERVICE_XMPP_SUBSCRIBE")]
    INBOUNDSERVICEXMPPSUBSCRIBE,
    

    /// Allows an application to receive a user's chat presence.
    ///
    /// "INBOUND_SERVICE_XMPP_PRESENCE"
    #[serde(rename="INBOUND_SERVICE_XMPP_PRESENCE")]
    INBOUNDSERVICEXMPPPRESENCE,
    

    /// Registers an application for notifications when a client connects or disconnects from a channel.
    ///
    /// "INBOUND_SERVICE_CHANNEL_PRESENCE"
    #[serde(rename="INBOUND_SERVICE_CHANNEL_PRESENCE")]
    INBOUNDSERVICECHANNELPRESENCE,
    

    /// Enables warmup requests.
    ///
    /// "INBOUND_SERVICE_WARMUP"
    #[serde(rename="INBOUND_SERVICE_WARMUP")]
    INBOUNDSERVICEWARMUP,
}

impl AsRef<str> for VersionInboundServicesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VersionInboundServicesEnum::INBOUNDSERVICEUNSPECIFIED => "INBOUND_SERVICE_UNSPECIFIED",
            VersionInboundServicesEnum::INBOUNDSERVICEMAIL => "INBOUND_SERVICE_MAIL",
            VersionInboundServicesEnum::INBOUNDSERVICEMAILBOUNCE => "INBOUND_SERVICE_MAIL_BOUNCE",
            VersionInboundServicesEnum::INBOUNDSERVICEXMPPERROR => "INBOUND_SERVICE_XMPP_ERROR",
            VersionInboundServicesEnum::INBOUNDSERVICEXMPPMESSAGE => "INBOUND_SERVICE_XMPP_MESSAGE",
            VersionInboundServicesEnum::INBOUNDSERVICEXMPPSUBSCRIBE => "INBOUND_SERVICE_XMPP_SUBSCRIBE",
            VersionInboundServicesEnum::INBOUNDSERVICEXMPPPRESENCE => "INBOUND_SERVICE_XMPP_PRESENCE",
            VersionInboundServicesEnum::INBOUNDSERVICECHANNELPRESENCE => "INBOUND_SERVICE_CHANNEL_PRESENCE",
            VersionInboundServicesEnum::INBOUNDSERVICEWARMUP => "INBOUND_SERVICE_WARMUP",
        }
    }
}

impl std::convert::TryFrom< &str> for VersionInboundServicesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INBOUND_SERVICE_UNSPECIFIED" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEUNSPECIFIED),
           "INBOUND_SERVICE_MAIL" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEMAIL),
           "INBOUND_SERVICE_MAIL_BOUNCE" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEMAILBOUNCE),
           "INBOUND_SERVICE_XMPP_ERROR" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEXMPPERROR),
           "INBOUND_SERVICE_XMPP_MESSAGE" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEXMPPMESSAGE),
           "INBOUND_SERVICE_XMPP_SUBSCRIBE" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEXMPPSUBSCRIBE),
           "INBOUND_SERVICE_XMPP_PRESENCE" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEXMPPPRESENCE),
           "INBOUND_SERVICE_CHANNEL_PRESENCE" => Ok(VersionInboundServicesEnum::INBOUNDSERVICECHANNELPRESENCE),
           "INBOUND_SERVICE_WARMUP" => Ok(VersionInboundServicesEnum::INBOUNDSERVICEWARMUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VersionInboundServicesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VersionServingStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING.
pub enum VersionServingStatusEnum {
    

    /// Not specified.
    ///
    /// "SERVING_STATUS_UNSPECIFIED"
    #[serde(rename="SERVING_STATUS_UNSPECIFIED")]
    SERVINGSTATUSUNSPECIFIED,
    

    /// Currently serving. Instances are created according to the scaling settings of the version.
    ///
    /// "SERVING"
    #[serde(rename="SERVING")]
    SERVING,
    

    /// Disabled. No instances will be created and the scaling settings are ignored until the state of the version changes to SERVING.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
}

impl AsRef<str> for VersionServingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VersionServingStatusEnum::SERVINGSTATUSUNSPECIFIED => "SERVING_STATUS_UNSPECIFIED",
            VersionServingStatusEnum::SERVING => "SERVING",
            VersionServingStatusEnum::STOPPED => "STOPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for VersionServingStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVING_STATUS_UNSPECIFIED" => Ok(VersionServingStatusEnum::SERVINGSTATUSUNSPECIFIED),
           "SERVING" => Ok(VersionServingStatusEnum::SERVING),
           "STOPPED" => Ok(VersionServingStatusEnum::STOPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VersionServingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VpcAccessConnectorEgressSettingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The egress setting for the connector, controlling what traffic is diverted through it.
pub enum VpcAccessConnectorEgressSettingEnum {
    
    /// "EGRESS_SETTING_UNSPECIFIED"
    #[serde(rename="EGRESS_SETTING_UNSPECIFIED")]
    EGRESSSETTINGUNSPECIFIED,
    

    /// Force the use of VPC Access for all egress traffic from the function.
    ///
    /// "ALL_TRAFFIC"
    #[serde(rename="ALL_TRAFFIC")]
    ALLTRAFFIC,
    

    /// Use the VPC Access Connector for private IP space from RFC1918.
    ///
    /// "PRIVATE_IP_RANGES"
    #[serde(rename="PRIVATE_IP_RANGES")]
    PRIVATEIPRANGES,
}

impl AsRef<str> for VpcAccessConnectorEgressSettingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VpcAccessConnectorEgressSettingEnum::EGRESSSETTINGUNSPECIFIED => "EGRESS_SETTING_UNSPECIFIED",
            VpcAccessConnectorEgressSettingEnum::ALLTRAFFIC => "ALL_TRAFFIC",
            VpcAccessConnectorEgressSettingEnum::PRIVATEIPRANGES => "PRIVATE_IP_RANGES",
        }
    }
}

impl std::convert::TryFrom< &str> for VpcAccessConnectorEgressSettingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EGRESS_SETTING_UNSPECIFIED" => Ok(VpcAccessConnectorEgressSettingEnum::EGRESSSETTINGUNSPECIFIED),
           "ALL_TRAFFIC" => Ok(VpcAccessConnectorEgressSettingEnum::ALLTRAFFIC),
           "PRIVATE_IP_RANGES" => Ok(VpcAccessConnectorEgressSettingEnum::PRIVATEIPRANGES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VpcAccessConnectorEgressSettingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the set of fields returned in the List response.
pub enum AppViewEnum {
    

    /// Basic version information including scaling and inbound services, but not detailed deployment information.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// The information from BASIC, plus detailed information about the deployment. This format is required when creating resources, but is not returned in Get or List by default.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for AppViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppViewEnum::BASIC => "BASIC",
            AppViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for AppViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(AppViewEnum::BASIC),
           "FULL" => Ok(AppViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppOverrideStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the domain creation should override any existing mappings for this domain. By default, overrides are rejected.
pub enum AppOverrideStrategyEnum {
    

    /// Strategy unspecified. Defaults to STRICT.
    ///
    /// "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY"
    #[serde(rename="UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY")]
    UNSPECIFIEDDOMAINOVERRIDESTRATEGY,
    

    /// Overrides not allowed. If a mapping already exists for the specified domain, the request will return an ALREADY_EXISTS (409).
    ///
    /// "STRICT"
    #[serde(rename="STRICT")]
    STRICT,
    

    /// Overrides allowed. If a mapping already exists for the specified domain, the request will overwrite it. Note that this might stop another Google product from serving. For example, if the domain is mapped to another App Engine application, that app will no longer serve from that domain.
    ///
    /// "OVERRIDE"
    #[serde(rename="OVERRIDE")]
    OVERRIDE,
}

impl AsRef<str> for AppOverrideStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppOverrideStrategyEnum::UNSPECIFIEDDOMAINOVERRIDESTRATEGY => "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY",
            AppOverrideStrategyEnum::STRICT => "STRICT",
            AppOverrideStrategyEnum::OVERRIDE => "OVERRIDE",
        }
    }
}

impl std::convert::TryFrom< &str> for AppOverrideStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY" => Ok(AppOverrideStrategyEnum::UNSPECIFIEDDOMAINOVERRIDESTRATEGY),
           "STRICT" => Ok(AppOverrideStrategyEnum::STRICT),
           "OVERRIDE" => Ok(AppOverrideStrategyEnum::OVERRIDE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppOverrideStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppIncludeExtraDataEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Options to include extra data
pub enum AppIncludeExtraDataEnum {
    

    /// Unspecified: No extra data will be returned
    ///
    /// "INCLUDE_EXTRA_DATA_UNSPECIFIED"
    #[serde(rename="INCLUDE_EXTRA_DATA_UNSPECIFIED")]
    INCLUDEEXTRADATAUNSPECIFIED,
    

    /// Do not return any extra data
    ///
    /// "INCLUDE_EXTRA_DATA_NONE"
    #[serde(rename="INCLUDE_EXTRA_DATA_NONE")]
    INCLUDEEXTRADATANONE,
    

    /// Return GGCM associated with the resources
    ///
    /// "INCLUDE_GOOGLE_GENERATED_METADATA"
    #[serde(rename="INCLUDE_GOOGLE_GENERATED_METADATA")]
    INCLUDEGOOGLEGENERATEDMETADATA,
}

impl AsRef<str> for AppIncludeExtraDataEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppIncludeExtraDataEnum::INCLUDEEXTRADATAUNSPECIFIED => "INCLUDE_EXTRA_DATA_UNSPECIFIED",
            AppIncludeExtraDataEnum::INCLUDEEXTRADATANONE => "INCLUDE_EXTRA_DATA_NONE",
            AppIncludeExtraDataEnum::INCLUDEGOOGLEGENERATEDMETADATA => "INCLUDE_GOOGLE_GENERATED_METADATA",
        }
    }
}

impl std::convert::TryFrom< &str> for AppIncludeExtraDataEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INCLUDE_EXTRA_DATA_UNSPECIFIED" => Ok(AppIncludeExtraDataEnum::INCLUDEEXTRADATAUNSPECIFIED),
           "INCLUDE_EXTRA_DATA_NONE" => Ok(AppIncludeExtraDataEnum::INCLUDEEXTRADATANONE),
           "INCLUDE_GOOGLE_GENERATED_METADATA" => Ok(AppIncludeExtraDataEnum::INCLUDEGOOGLEGENERATEDMETADATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppIncludeExtraDataEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The environment of the Application.
pub enum AppEnvironmentEnum {
    

    /// Default value.
    ///
    /// "ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_UNSPECIFIED")]
    ENVIRONMENTUNSPECIFIED,
    

    /// App Engine Standard.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// App Engine Flexible
    ///
    /// "FLEXIBLE"
    #[serde(rename="FLEXIBLE")]
    FLEXIBLE,
}

impl AsRef<str> for AppEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppEnvironmentEnum::ENVIRONMENTUNSPECIFIED => "ENVIRONMENT_UNSPECIFIED",
            AppEnvironmentEnum::STANDARD => "STANDARD",
            AppEnvironmentEnum::FLEXIBLE => "FLEXIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for AppEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_UNSPECIFIED" => Ok(AppEnvironmentEnum::ENVIRONMENTUNSPECIFIED),
           "STANDARD" => Ok(AppEnvironmentEnum::STANDARD),
           "FLEXIBLE" => Ok(AppEnvironmentEnum::FLEXIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


