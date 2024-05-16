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


// region EndpointsApiServiceRolloutStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted.
pub enum EndpointsApiServiceRolloutStrategyEnum {
    

    /// Not specified. Defaults to FIXED.
    ///
    /// "UNSPECIFIED_ROLLOUT_STRATEGY"
    #[serde(rename="UNSPECIFIED_ROLLOUT_STRATEGY")]
    UNSPECIFIEDROLLOUTSTRATEGY,
    

    /// Endpoints service configuration id will be fixed to the configuration id specified by config_id.
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// Endpoints service configuration id will be updated with each rollout.
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


// region InstanceAvailabilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Availability of the instance.@OutputOnly
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


// region TrafficSplitShardByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed.
pub enum TrafficSplitShardByEnum {
    

    /// Diversion method unspecified.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Diversion based on a specially named cookie, "GOOGAPPUID." The cookie must be set by the application itself or else no diversion will occur.
    ///
    /// "COOKIE"
    #[serde(rename="COOKIE")]
    COOKIE,
    

    /// Diversion based on applying the modulus operation to a fingerprint of the IP address.
    ///
    /// "IP"
    #[serde(rename="IP")]
    IP,
}

impl AsRef<str> for TrafficSplitShardByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TrafficSplitShardByEnum::UNSPECIFIED => "UNSPECIFIED",
            TrafficSplitShardByEnum::COOKIE => "COOKIE",
            TrafficSplitShardByEnum::IP => "IP",
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
/// Level of login required to access this resource.
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


// region AppViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the set of fields returned in the List response.
pub enum AppViewEnum {
    

    /// no description found
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// no description found
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


