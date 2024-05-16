use super::*;



// region CheckErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The error code.
pub enum CheckErrorCodeEnum {
    

    /// This is never used in `CheckResponse`.
    ///
    /// "ERROR_CODE_UNSPECIFIED"
    #[serde(rename="ERROR_CODE_UNSPECIFIED")]
    ERRORCODEUNSPECIFIED,
    

    /// The consumer's project id, network container, or resource container was not found. Same as google.rpc.Code.NOT_FOUND.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// The consumer doesn't have access to the specified resource. Same as google.rpc.Code.PERMISSION_DENIED.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// Quota check failed. Same as google.rpc.Code.RESOURCE_EXHAUSTED.
    ///
    /// "RESOURCE_EXHAUSTED"
    #[serde(rename="RESOURCE_EXHAUSTED")]
    RESOURCEEXHAUSTED,
    

    /// Budget check failed.
    ///
    /// "BUDGET_EXCEEDED"
    #[serde(rename="BUDGET_EXCEEDED")]
    BUDGETEXCEEDED,
    

    /// The consumer's request has been flagged as a DoS attack.
    ///
    /// "DENIAL_OF_SERVICE_DETECTED"
    #[serde(rename="DENIAL_OF_SERVICE_DETECTED")]
    DENIALOFSERVICEDETECTED,
    

    /// The consumer's request should be rejected in order to protect the service from being overloaded.
    ///
    /// "LOAD_SHEDDING"
    #[serde(rename="LOAD_SHEDDING")]
    LOADSHEDDING,
    

    /// The consumer has been flagged as an abuser.
    ///
    /// "ABUSER_DETECTED"
    #[serde(rename="ABUSER_DETECTED")]
    ABUSERDETECTED,
    

    /// The consumer hasn't activated the service.
    ///
    /// "SERVICE_NOT_ACTIVATED"
    #[serde(rename="SERVICE_NOT_ACTIVATED")]
    SERVICENOTACTIVATED,
    

    /// The consumer cannot access the service due to visibility configuration.
    ///
    /// "VISIBILITY_DENIED"
    #[serde(rename="VISIBILITY_DENIED")]
    VISIBILITYDENIED,
    

    /// The consumer cannot access the service because billing is disabled.
    ///
    /// "BILLING_DISABLED"
    #[serde(rename="BILLING_DISABLED")]
    BILLINGDISABLED,
    

    /// The consumer's project has been marked as deleted (soft deletion).
    ///
    /// "PROJECT_DELETED"
    #[serde(rename="PROJECT_DELETED")]
    PROJECTDELETED,
    

    /// The consumer's project number or id does not represent a valid project.
    ///
    /// "PROJECT_INVALID"
    #[serde(rename="PROJECT_INVALID")]
    PROJECTINVALID,
    

    /// The input consumer info does not represent a valid consumer folder or organization.
    ///
    /// "CONSUMER_INVALID"
    #[serde(rename="CONSUMER_INVALID")]
    CONSUMERINVALID,
    

    /// The IP address of the consumer is invalid for the specific consumer project.
    ///
    /// "IP_ADDRESS_BLOCKED"
    #[serde(rename="IP_ADDRESS_BLOCKED")]
    IPADDRESSBLOCKED,
    

    /// The referer address of the consumer request is invalid for the specific consumer project.
    ///
    /// "REFERER_BLOCKED"
    #[serde(rename="REFERER_BLOCKED")]
    REFERERBLOCKED,
    

    /// The client application of the consumer request is invalid for the specific consumer project.
    ///
    /// "CLIENT_APP_BLOCKED"
    #[serde(rename="CLIENT_APP_BLOCKED")]
    CLIENTAPPBLOCKED,
    

    /// The API targeted by this request is invalid for the specified consumer project.
    ///
    /// "API_TARGET_BLOCKED"
    #[serde(rename="API_TARGET_BLOCKED")]
    APITARGETBLOCKED,
    

    /// The consumer's API key is invalid.
    ///
    /// "API_KEY_INVALID"
    #[serde(rename="API_KEY_INVALID")]
    APIKEYINVALID,
    

    /// The consumer's API Key has expired.
    ///
    /// "API_KEY_EXPIRED"
    #[serde(rename="API_KEY_EXPIRED")]
    APIKEYEXPIRED,
    

    /// The consumer's API Key was not found in config record.
    ///
    /// "API_KEY_NOT_FOUND"
    #[serde(rename="API_KEY_NOT_FOUND")]
    APIKEYNOTFOUND,
    

    /// The consumer's spatula header is invalid.
    ///
    /// "SPATULA_HEADER_INVALID"
    #[serde(rename="SPATULA_HEADER_INVALID")]
    SPATULAHEADERINVALID,
    

    /// The consumer's LOAS role is invalid.
    ///
    /// "LOAS_ROLE_INVALID"
    #[serde(rename="LOAS_ROLE_INVALID")]
    LOASROLEINVALID,
    

    /// The consumer's LOAS role has no associated project.
    ///
    /// "NO_LOAS_PROJECT"
    #[serde(rename="NO_LOAS_PROJECT")]
    NOLOASPROJECT,
    

    /// The consumer's LOAS project is not `ACTIVE` in LoquatV2.
    ///
    /// "LOAS_PROJECT_DISABLED"
    #[serde(rename="LOAS_PROJECT_DISABLED")]
    LOASPROJECTDISABLED,
    

    /// Request is not allowed as per security policies defined in Org Policy.
    ///
    /// "SECURITY_POLICY_VIOLATED"
    #[serde(rename="SECURITY_POLICY_VIOLATED")]
    SECURITYPOLICYVIOLATED,
    

    /// The credential in the request can not be verified.
    ///
    /// "INVALID_CREDENTIAL"
    #[serde(rename="INVALID_CREDENTIAL")]
    INVALIDCREDENTIAL,
    

    /// Request is not allowed as per location policies defined in Org Policy.
    ///
    /// "LOCATION_POLICY_VIOLATED"
    #[serde(rename="LOCATION_POLICY_VIOLATED")]
    LOCATIONPOLICYVIOLATED,
    

    /// The backend server for looking up project id/number is unavailable.
    ///
    /// "NAMESPACE_LOOKUP_UNAVAILABLE"
    #[serde(rename="NAMESPACE_LOOKUP_UNAVAILABLE")]
    NAMESPACELOOKUPUNAVAILABLE,
    

    /// The backend server for checking service status is unavailable.
    ///
    /// "SERVICE_STATUS_UNAVAILABLE"
    #[serde(rename="SERVICE_STATUS_UNAVAILABLE")]
    SERVICESTATUSUNAVAILABLE,
    

    /// The backend server for checking billing status is unavailable.
    ///
    /// "BILLING_STATUS_UNAVAILABLE"
    #[serde(rename="BILLING_STATUS_UNAVAILABLE")]
    BILLINGSTATUSUNAVAILABLE,
    

    /// The backend server for checking quota limits is unavailable.
    ///
    /// "QUOTA_CHECK_UNAVAILABLE"
    #[serde(rename="QUOTA_CHECK_UNAVAILABLE")]
    QUOTACHECKUNAVAILABLE,
    

    /// The Spanner for looking up LOAS project is unavailable.
    ///
    /// "LOAS_PROJECT_LOOKUP_UNAVAILABLE"
    #[serde(rename="LOAS_PROJECT_LOOKUP_UNAVAILABLE")]
    LOASPROJECTLOOKUPUNAVAILABLE,
    

    /// Cloud Resource Manager backend server is unavailable.
    ///
    /// "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE"
    #[serde(rename="CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE")]
    CLOUDRESOURCEMANAGERBACKENDUNAVAILABLE,
    

    /// NOTE: for customers in the scope of Beta/GA of https://cloud.google.com/vpc-service-controls, this error is no longer returned. If the security backend is unavailable, rpc UNAVAILABLE status will be returned instead. It should be ignored and should not be used to reject client requests.
    ///
    /// "SECURITY_POLICY_BACKEND_UNAVAILABLE"
    #[serde(rename="SECURITY_POLICY_BACKEND_UNAVAILABLE")]
    SECURITYPOLICYBACKENDUNAVAILABLE,
    

    /// Backend server for evaluating location policy is unavailable.
    ///
    /// "LOCATION_POLICY_BACKEND_UNAVAILABLE"
    #[serde(rename="LOCATION_POLICY_BACKEND_UNAVAILABLE")]
    LOCATIONPOLICYBACKENDUNAVAILABLE,
    

    /// Part of the project of fault injection: go/chemist-slo-validation. To distinguish between artificially injected errors and organic ones, this value will be exported for the per_service_check_error_count streamz. http://google3/apiserving/servicecontrol/server/controller_service.cc;l=196 Rpcinjectz2 works by injecting errors early in the rpc life cycle, before any of the chemist business logic runs.
    ///
    /// "INJECTED_ERROR"
    #[serde(rename="INJECTED_ERROR")]
    INJECTEDERROR,
}

impl AsRef<str> for CheckErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CheckErrorCodeEnum::ERRORCODEUNSPECIFIED => "ERROR_CODE_UNSPECIFIED",
            CheckErrorCodeEnum::NOTFOUND => "NOT_FOUND",
            CheckErrorCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            CheckErrorCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            CheckErrorCodeEnum::BUDGETEXCEEDED => "BUDGET_EXCEEDED",
            CheckErrorCodeEnum::DENIALOFSERVICEDETECTED => "DENIAL_OF_SERVICE_DETECTED",
            CheckErrorCodeEnum::LOADSHEDDING => "LOAD_SHEDDING",
            CheckErrorCodeEnum::ABUSERDETECTED => "ABUSER_DETECTED",
            CheckErrorCodeEnum::SERVICENOTACTIVATED => "SERVICE_NOT_ACTIVATED",
            CheckErrorCodeEnum::VISIBILITYDENIED => "VISIBILITY_DENIED",
            CheckErrorCodeEnum::BILLINGDISABLED => "BILLING_DISABLED",
            CheckErrorCodeEnum::PROJECTDELETED => "PROJECT_DELETED",
            CheckErrorCodeEnum::PROJECTINVALID => "PROJECT_INVALID",
            CheckErrorCodeEnum::CONSUMERINVALID => "CONSUMER_INVALID",
            CheckErrorCodeEnum::IPADDRESSBLOCKED => "IP_ADDRESS_BLOCKED",
            CheckErrorCodeEnum::REFERERBLOCKED => "REFERER_BLOCKED",
            CheckErrorCodeEnum::CLIENTAPPBLOCKED => "CLIENT_APP_BLOCKED",
            CheckErrorCodeEnum::APITARGETBLOCKED => "API_TARGET_BLOCKED",
            CheckErrorCodeEnum::APIKEYINVALID => "API_KEY_INVALID",
            CheckErrorCodeEnum::APIKEYEXPIRED => "API_KEY_EXPIRED",
            CheckErrorCodeEnum::APIKEYNOTFOUND => "API_KEY_NOT_FOUND",
            CheckErrorCodeEnum::SPATULAHEADERINVALID => "SPATULA_HEADER_INVALID",
            CheckErrorCodeEnum::LOASROLEINVALID => "LOAS_ROLE_INVALID",
            CheckErrorCodeEnum::NOLOASPROJECT => "NO_LOAS_PROJECT",
            CheckErrorCodeEnum::LOASPROJECTDISABLED => "LOAS_PROJECT_DISABLED",
            CheckErrorCodeEnum::SECURITYPOLICYVIOLATED => "SECURITY_POLICY_VIOLATED",
            CheckErrorCodeEnum::INVALIDCREDENTIAL => "INVALID_CREDENTIAL",
            CheckErrorCodeEnum::LOCATIONPOLICYVIOLATED => "LOCATION_POLICY_VIOLATED",
            CheckErrorCodeEnum::NAMESPACELOOKUPUNAVAILABLE => "NAMESPACE_LOOKUP_UNAVAILABLE",
            CheckErrorCodeEnum::SERVICESTATUSUNAVAILABLE => "SERVICE_STATUS_UNAVAILABLE",
            CheckErrorCodeEnum::BILLINGSTATUSUNAVAILABLE => "BILLING_STATUS_UNAVAILABLE",
            CheckErrorCodeEnum::QUOTACHECKUNAVAILABLE => "QUOTA_CHECK_UNAVAILABLE",
            CheckErrorCodeEnum::LOASPROJECTLOOKUPUNAVAILABLE => "LOAS_PROJECT_LOOKUP_UNAVAILABLE",
            CheckErrorCodeEnum::CLOUDRESOURCEMANAGERBACKENDUNAVAILABLE => "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE",
            CheckErrorCodeEnum::SECURITYPOLICYBACKENDUNAVAILABLE => "SECURITY_POLICY_BACKEND_UNAVAILABLE",
            CheckErrorCodeEnum::LOCATIONPOLICYBACKENDUNAVAILABLE => "LOCATION_POLICY_BACKEND_UNAVAILABLE",
            CheckErrorCodeEnum::INJECTEDERROR => "INJECTED_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for CheckErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_CODE_UNSPECIFIED" => Ok(CheckErrorCodeEnum::ERRORCODEUNSPECIFIED),
           "NOT_FOUND" => Ok(CheckErrorCodeEnum::NOTFOUND),
           "PERMISSION_DENIED" => Ok(CheckErrorCodeEnum::PERMISSIONDENIED),
           "RESOURCE_EXHAUSTED" => Ok(CheckErrorCodeEnum::RESOURCEEXHAUSTED),
           "BUDGET_EXCEEDED" => Ok(CheckErrorCodeEnum::BUDGETEXCEEDED),
           "DENIAL_OF_SERVICE_DETECTED" => Ok(CheckErrorCodeEnum::DENIALOFSERVICEDETECTED),
           "LOAD_SHEDDING" => Ok(CheckErrorCodeEnum::LOADSHEDDING),
           "ABUSER_DETECTED" => Ok(CheckErrorCodeEnum::ABUSERDETECTED),
           "SERVICE_NOT_ACTIVATED" => Ok(CheckErrorCodeEnum::SERVICENOTACTIVATED),
           "VISIBILITY_DENIED" => Ok(CheckErrorCodeEnum::VISIBILITYDENIED),
           "BILLING_DISABLED" => Ok(CheckErrorCodeEnum::BILLINGDISABLED),
           "PROJECT_DELETED" => Ok(CheckErrorCodeEnum::PROJECTDELETED),
           "PROJECT_INVALID" => Ok(CheckErrorCodeEnum::PROJECTINVALID),
           "CONSUMER_INVALID" => Ok(CheckErrorCodeEnum::CONSUMERINVALID),
           "IP_ADDRESS_BLOCKED" => Ok(CheckErrorCodeEnum::IPADDRESSBLOCKED),
           "REFERER_BLOCKED" => Ok(CheckErrorCodeEnum::REFERERBLOCKED),
           "CLIENT_APP_BLOCKED" => Ok(CheckErrorCodeEnum::CLIENTAPPBLOCKED),
           "API_TARGET_BLOCKED" => Ok(CheckErrorCodeEnum::APITARGETBLOCKED),
           "API_KEY_INVALID" => Ok(CheckErrorCodeEnum::APIKEYINVALID),
           "API_KEY_EXPIRED" => Ok(CheckErrorCodeEnum::APIKEYEXPIRED),
           "API_KEY_NOT_FOUND" => Ok(CheckErrorCodeEnum::APIKEYNOTFOUND),
           "SPATULA_HEADER_INVALID" => Ok(CheckErrorCodeEnum::SPATULAHEADERINVALID),
           "LOAS_ROLE_INVALID" => Ok(CheckErrorCodeEnum::LOASROLEINVALID),
           "NO_LOAS_PROJECT" => Ok(CheckErrorCodeEnum::NOLOASPROJECT),
           "LOAS_PROJECT_DISABLED" => Ok(CheckErrorCodeEnum::LOASPROJECTDISABLED),
           "SECURITY_POLICY_VIOLATED" => Ok(CheckErrorCodeEnum::SECURITYPOLICYVIOLATED),
           "INVALID_CREDENTIAL" => Ok(CheckErrorCodeEnum::INVALIDCREDENTIAL),
           "LOCATION_POLICY_VIOLATED" => Ok(CheckErrorCodeEnum::LOCATIONPOLICYVIOLATED),
           "NAMESPACE_LOOKUP_UNAVAILABLE" => Ok(CheckErrorCodeEnum::NAMESPACELOOKUPUNAVAILABLE),
           "SERVICE_STATUS_UNAVAILABLE" => Ok(CheckErrorCodeEnum::SERVICESTATUSUNAVAILABLE),
           "BILLING_STATUS_UNAVAILABLE" => Ok(CheckErrorCodeEnum::BILLINGSTATUSUNAVAILABLE),
           "QUOTA_CHECK_UNAVAILABLE" => Ok(CheckErrorCodeEnum::QUOTACHECKUNAVAILABLE),
           "LOAS_PROJECT_LOOKUP_UNAVAILABLE" => Ok(CheckErrorCodeEnum::LOASPROJECTLOOKUPUNAVAILABLE),
           "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE" => Ok(CheckErrorCodeEnum::CLOUDRESOURCEMANAGERBACKENDUNAVAILABLE),
           "SECURITY_POLICY_BACKEND_UNAVAILABLE" => Ok(CheckErrorCodeEnum::SECURITYPOLICYBACKENDUNAVAILABLE),
           "LOCATION_POLICY_BACKEND_UNAVAILABLE" => Ok(CheckErrorCodeEnum::LOCATIONPOLICYBACKENDUNAVAILABLE),
           "INJECTED_ERROR" => Ok(CheckErrorCodeEnum::INJECTEDERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CheckErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConsumerInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the consumer which should have been defined in [Google Resource Manager](https://cloud.google.com/resource-manager/).
pub enum ConsumerInfoTypeEnum {
    

    /// This is never used.
    ///
    /// "CONSUMER_TYPE_UNSPECIFIED"
    #[serde(rename="CONSUMER_TYPE_UNSPECIFIED")]
    CONSUMERTYPEUNSPECIFIED,
    

    /// The consumer is a Google Cloud Project.
    ///
    /// "PROJECT"
    #[serde(rename="PROJECT")]
    PROJECT,
    

    /// The consumer is a Google Cloud Folder.
    ///
    /// "FOLDER"
    #[serde(rename="FOLDER")]
    FOLDER,
    

    /// The consumer is a Google Cloud Organization.
    ///
    /// "ORGANIZATION"
    #[serde(rename="ORGANIZATION")]
    ORGANIZATION,
    

    /// Service-specific resource container which is defined by the service producer to offer their users the ability to manage service control functionalities at a finer level of granularity than the PROJECT.
    ///
    /// "SERVICE_SPECIFIC"
    #[serde(rename="SERVICE_SPECIFIC")]
    SERVICESPECIFIC,
}

impl AsRef<str> for ConsumerInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConsumerInfoTypeEnum::CONSUMERTYPEUNSPECIFIED => "CONSUMER_TYPE_UNSPECIFIED",
            ConsumerInfoTypeEnum::PROJECT => "PROJECT",
            ConsumerInfoTypeEnum::FOLDER => "FOLDER",
            ConsumerInfoTypeEnum::ORGANIZATION => "ORGANIZATION",
            ConsumerInfoTypeEnum::SERVICESPECIFIC => "SERVICE_SPECIFIC",
        }
    }
}

impl std::convert::TryFrom< &str> for ConsumerInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSUMER_TYPE_UNSPECIFIED" => Ok(ConsumerInfoTypeEnum::CONSUMERTYPEUNSPECIFIED),
           "PROJECT" => Ok(ConsumerInfoTypeEnum::PROJECT),
           "FOLDER" => Ok(ConsumerInfoTypeEnum::FOLDER),
           "ORGANIZATION" => Ok(ConsumerInfoTypeEnum::ORGANIZATION),
           "SERVICE_SPECIFIC" => Ok(ConsumerInfoTypeEnum::SERVICESPECIFIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConsumerInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LogEntrySeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the log entry. The default value is `LogSeverity.DEFAULT`.
pub enum LogEntrySeverityEnum {
    

    /// (0) The log entry has no assigned severity level.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// (100) Debug or trace information.
    ///
    /// "DEBUG"
    #[serde(rename="DEBUG")]
    DEBUG,
    

    /// (200) Routine information, such as ongoing status or performance.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// (300) Normal but significant events, such as start up, shut down, or a configuration change.
    ///
    /// "NOTICE"
    #[serde(rename="NOTICE")]
    NOTICE,
    

    /// (400) Warning events might cause problems.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// (500) Error events are likely to cause problems.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// (600) Critical events cause more severe problems or outages.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
    

    /// (700) A person must take an action immediately.
    ///
    /// "ALERT"
    #[serde(rename="ALERT")]
    ALERT,
    

    /// (800) One or more systems are unusable.
    ///
    /// "EMERGENCY"
    #[serde(rename="EMERGENCY")]
    EMERGENCY,
}

impl AsRef<str> for LogEntrySeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LogEntrySeverityEnum::DEFAULT => "DEFAULT",
            LogEntrySeverityEnum::DEBUG => "DEBUG",
            LogEntrySeverityEnum::INFO => "INFO",
            LogEntrySeverityEnum::NOTICE => "NOTICE",
            LogEntrySeverityEnum::WARNING => "WARNING",
            LogEntrySeverityEnum::ERROR => "ERROR",
            LogEntrySeverityEnum::CRITICAL => "CRITICAL",
            LogEntrySeverityEnum::ALERT => "ALERT",
            LogEntrySeverityEnum::EMERGENCY => "EMERGENCY",
        }
    }
}

impl std::convert::TryFrom< &str> for LogEntrySeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(LogEntrySeverityEnum::DEFAULT),
           "DEBUG" => Ok(LogEntrySeverityEnum::DEBUG),
           "INFO" => Ok(LogEntrySeverityEnum::INFO),
           "NOTICE" => Ok(LogEntrySeverityEnum::NOTICE),
           "WARNING" => Ok(LogEntrySeverityEnum::WARNING),
           "ERROR" => Ok(LogEntrySeverityEnum::ERROR),
           "CRITICAL" => Ok(LogEntrySeverityEnum::CRITICAL),
           "ALERT" => Ok(LogEntrySeverityEnum::ALERT),
           "EMERGENCY" => Ok(LogEntrySeverityEnum::EMERGENCY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LogEntrySeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationImportanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// DO NOT USE. This is an experimental field.
pub enum OperationImportanceEnum {
    

    /// Allows data caching, batching, and aggregation. It provides higher performance with higher data loss risk.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Disables data aggregation to minimize data loss. It is for operations that contains significant monetary value or audit trail. This feature only applies to the client libraries.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Deprecated. Do not use. Disables data aggregation and enables additional validation logic. It should only be used during the onboarding process. It is only available to Google internal services, and the service must be approved by chemist-dev@google.com in order to use this level.
    ///
    /// "DEBUG"
    #[serde(rename="DEBUG")]
    DEBUG,
    

    /// Used internally by Chemist.
    ///
    /// "PROMOTED"
    #[serde(rename="PROMOTED")]
    PROMOTED,
}

impl AsRef<str> for OperationImportanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationImportanceEnum::LOW => "LOW",
            OperationImportanceEnum::HIGH => "HIGH",
            OperationImportanceEnum::DEBUG => "DEBUG",
            OperationImportanceEnum::PROMOTED => "PROMOTED",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationImportanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOW" => Ok(OperationImportanceEnum::LOW),
           "HIGH" => Ok(OperationImportanceEnum::HIGH),
           "DEBUG" => Ok(OperationImportanceEnum::DEBUG),
           "PROMOTED" => Ok(OperationImportanceEnum::PROMOTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationImportanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QuotaErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Error code.
pub enum QuotaErrorCodeEnum {
    

    /// This is never used.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Quota allocation failed. Same as google.rpc.Code.RESOURCE_EXHAUSTED.
    ///
    /// "RESOURCE_EXHAUSTED"
    #[serde(rename="RESOURCE_EXHAUSTED")]
    RESOURCEEXHAUSTED,
    

    /// Quota release failed. This error is ONLY returned on a NORMAL release. More formally: if a user requests a release of 10 tokens, but only 5 tokens were previously allocated, in a BEST_EFFORT release, this will be considered a success, 5 tokens will be released, and the result will be "Ok". If this is done in NORMAL mode, no tokens will be released, and an OUT_OF_RANGE error will be returned. Same as google.rpc.Code.OUT_OF_RANGE.
    ///
    /// "OUT_OF_RANGE"
    #[serde(rename="OUT_OF_RANGE")]
    OUTOFRANGE,
    

    /// Consumer cannot access the service because the service requires active billing.
    ///
    /// "BILLING_NOT_ACTIVE"
    #[serde(rename="BILLING_NOT_ACTIVE")]
    BILLINGNOTACTIVE,
    

    /// Consumer's project has been marked as deleted (soft deletion).
    ///
    /// "PROJECT_DELETED"
    #[serde(rename="PROJECT_DELETED")]
    PROJECTDELETED,
    

    /// Specified API key is invalid.
    ///
    /// "API_KEY_INVALID"
    #[serde(rename="API_KEY_INVALID")]
    APIKEYINVALID,
    

    /// Specified API Key has expired.
    ///
    /// "API_KEY_EXPIRED"
    #[serde(rename="API_KEY_EXPIRED")]
    APIKEYEXPIRED,
    

    /// Consumer's spatula header is invalid.
    ///
    /// "SPATULA_HEADER_INVALID"
    #[serde(rename="SPATULA_HEADER_INVALID")]
    SPATULAHEADERINVALID,
    

    /// The consumer's LOAS role is invalid.
    ///
    /// "LOAS_ROLE_INVALID"
    #[serde(rename="LOAS_ROLE_INVALID")]
    LOASROLEINVALID,
    

    /// The consumer's LOAS role has no associated project.
    ///
    /// "NO_LOAS_PROJECT"
    #[serde(rename="NO_LOAS_PROJECT")]
    NOLOASPROJECT,
    

    /// The backend server for looking up project id/number is unavailable.
    ///
    /// "PROJECT_STATUS_UNAVAILABLE"
    #[serde(rename="PROJECT_STATUS_UNAVAILABLE")]
    PROJECTSTATUSUNAVAILABLE,
    

    /// The backend server for checking service status is unavailable.
    ///
    /// "SERVICE_STATUS_UNAVAILABLE"
    #[serde(rename="SERVICE_STATUS_UNAVAILABLE")]
    SERVICESTATUSUNAVAILABLE,
    

    /// The backend server for checking billing status is unavailable.
    ///
    /// "BILLING_STATUS_UNAVAILABLE"
    #[serde(rename="BILLING_STATUS_UNAVAILABLE")]
    BILLINGSTATUSUNAVAILABLE,
    

    /// The backend server for checking quota limits is unavailable.
    ///
    /// "QUOTA_SYSTEM_UNAVAILABLE"
    #[serde(rename="QUOTA_SYSTEM_UNAVAILABLE")]
    QUOTASYSTEMUNAVAILABLE,
}

impl AsRef<str> for QuotaErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QuotaErrorCodeEnum::UNSPECIFIED => "UNSPECIFIED",
            QuotaErrorCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            QuotaErrorCodeEnum::OUTOFRANGE => "OUT_OF_RANGE",
            QuotaErrorCodeEnum::BILLINGNOTACTIVE => "BILLING_NOT_ACTIVE",
            QuotaErrorCodeEnum::PROJECTDELETED => "PROJECT_DELETED",
            QuotaErrorCodeEnum::APIKEYINVALID => "API_KEY_INVALID",
            QuotaErrorCodeEnum::APIKEYEXPIRED => "API_KEY_EXPIRED",
            QuotaErrorCodeEnum::SPATULAHEADERINVALID => "SPATULA_HEADER_INVALID",
            QuotaErrorCodeEnum::LOASROLEINVALID => "LOAS_ROLE_INVALID",
            QuotaErrorCodeEnum::NOLOASPROJECT => "NO_LOAS_PROJECT",
            QuotaErrorCodeEnum::PROJECTSTATUSUNAVAILABLE => "PROJECT_STATUS_UNAVAILABLE",
            QuotaErrorCodeEnum::SERVICESTATUSUNAVAILABLE => "SERVICE_STATUS_UNAVAILABLE",
            QuotaErrorCodeEnum::BILLINGSTATUSUNAVAILABLE => "BILLING_STATUS_UNAVAILABLE",
            QuotaErrorCodeEnum::QUOTASYSTEMUNAVAILABLE => "QUOTA_SYSTEM_UNAVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for QuotaErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(QuotaErrorCodeEnum::UNSPECIFIED),
           "RESOURCE_EXHAUSTED" => Ok(QuotaErrorCodeEnum::RESOURCEEXHAUSTED),
           "OUT_OF_RANGE" => Ok(QuotaErrorCodeEnum::OUTOFRANGE),
           "BILLING_NOT_ACTIVE" => Ok(QuotaErrorCodeEnum::BILLINGNOTACTIVE),
           "PROJECT_DELETED" => Ok(QuotaErrorCodeEnum::PROJECTDELETED),
           "API_KEY_INVALID" => Ok(QuotaErrorCodeEnum::APIKEYINVALID),
           "API_KEY_EXPIRED" => Ok(QuotaErrorCodeEnum::APIKEYEXPIRED),
           "SPATULA_HEADER_INVALID" => Ok(QuotaErrorCodeEnum::SPATULAHEADERINVALID),
           "LOAS_ROLE_INVALID" => Ok(QuotaErrorCodeEnum::LOASROLEINVALID),
           "NO_LOAS_PROJECT" => Ok(QuotaErrorCodeEnum::NOLOASPROJECT),
           "PROJECT_STATUS_UNAVAILABLE" => Ok(QuotaErrorCodeEnum::PROJECTSTATUSUNAVAILABLE),
           "SERVICE_STATUS_UNAVAILABLE" => Ok(QuotaErrorCodeEnum::SERVICESTATUSUNAVAILABLE),
           "BILLING_STATUS_UNAVAILABLE" => Ok(QuotaErrorCodeEnum::BILLINGSTATUSUNAVAILABLE),
           "QUOTA_SYSTEM_UNAVAILABLE" => Ok(QuotaErrorCodeEnum::QUOTASYSTEMUNAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QuotaErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QuotaOperationQuotaModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Quota mode for this operation.
pub enum QuotaOperationQuotaModeEnum {
    

    /// Guard against implicit default. Must not be used.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// For AllocateQuota request, allocates quota for the amount specified in the service configuration or specified using the quota metrics. If the amount is higher than the available quota, allocation error will be returned and no quota will be allocated. If multiple quotas are part of the request, and one fails, none of the quotas are allocated or released.
    ///
    /// "NORMAL"
    #[serde(rename="NORMAL")]
    NORMAL,
    

    /// The operation allocates quota for the amount specified in the service configuration or specified using the quota metrics. If the amount is higher than the available quota, request does not fail but all available quota will be allocated. For rate quota, BEST_EFFORT will continue to deduct from other groups even if one does not have enough quota. For allocation, it will find the minimum available amount across all groups and deduct that amount from all the affected groups.
    ///
    /// "BEST_EFFORT"
    #[serde(rename="BEST_EFFORT")]
    BESTEFFORT,
    

    /// For AllocateQuota request, only checks if there is enough quota available and does not change the available quota. No lock is placed on the available quota either.
    ///
    /// "CHECK_ONLY"
    #[serde(rename="CHECK_ONLY")]
    CHECKONLY,
    

    /// The operation allocates quota for the amount specified in the service configuration or specified using the quota metrics. If the requested amount is higher than the available quota, request does not fail and remaining quota would become negative (going over the limit). Not supported for Rate Quota.
    ///
    /// "ADJUST_ONLY"
    #[serde(rename="ADJUST_ONLY")]
    ADJUSTONLY,
}

impl AsRef<str> for QuotaOperationQuotaModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QuotaOperationQuotaModeEnum::UNSPECIFIED => "UNSPECIFIED",
            QuotaOperationQuotaModeEnum::NORMAL => "NORMAL",
            QuotaOperationQuotaModeEnum::BESTEFFORT => "BEST_EFFORT",
            QuotaOperationQuotaModeEnum::CHECKONLY => "CHECK_ONLY",
            QuotaOperationQuotaModeEnum::ADJUSTONLY => "ADJUST_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for QuotaOperationQuotaModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(QuotaOperationQuotaModeEnum::UNSPECIFIED),
           "NORMAL" => Ok(QuotaOperationQuotaModeEnum::NORMAL),
           "BEST_EFFORT" => Ok(QuotaOperationQuotaModeEnum::BESTEFFORT),
           "CHECK_ONLY" => Ok(QuotaOperationQuotaModeEnum::CHECKONLY),
           "ADJUST_ONLY" => Ok(QuotaOperationQuotaModeEnum::ADJUSTONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QuotaOperationQuotaModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QuotaPropertyQuotaModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Quota mode for this operation.
pub enum QuotaPropertyQuotaModeEnum {
    

    /// Decreases available quota by the cost specified for the operation. If cost is higher than available quota, operation fails and returns error.
    ///
    /// "ACQUIRE"
    #[serde(rename="ACQUIRE")]
    ACQUIRE,
    

    /// Decreases available quota by the cost specified for the operation. If cost is higher than available quota, operation does not fail and available quota goes down to zero but it returns error.
    ///
    /// "ACQUIRE_BEST_EFFORT"
    #[serde(rename="ACQUIRE_BEST_EFFORT")]
    ACQUIREBESTEFFORT,
    

    /// Does not change any available quota. Only checks if there is enough quota. No lock is placed on the checked tokens neither.
    ///
    /// "CHECK"
    #[serde(rename="CHECK")]
    CHECK,
}

impl AsRef<str> for QuotaPropertyQuotaModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QuotaPropertyQuotaModeEnum::ACQUIRE => "ACQUIRE",
            QuotaPropertyQuotaModeEnum::ACQUIREBESTEFFORT => "ACQUIRE_BEST_EFFORT",
            QuotaPropertyQuotaModeEnum::CHECK => "CHECK",
        }
    }
}

impl std::convert::TryFrom< &str> for QuotaPropertyQuotaModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACQUIRE" => Ok(QuotaPropertyQuotaModeEnum::ACQUIRE),
           "ACQUIRE_BEST_EFFORT" => Ok(QuotaPropertyQuotaModeEnum::ACQUIREBESTEFFORT),
           "CHECK" => Ok(QuotaPropertyQuotaModeEnum::CHECK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QuotaPropertyQuotaModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TraceSpanSpanKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call.
pub enum TraceSpanSpanKindEnum {
    

    /// Unspecified. Do NOT use as default. Implementations MAY assume SpanKind.INTERNAL to be default.
    ///
    /// "SPAN_KIND_UNSPECIFIED"
    #[serde(rename="SPAN_KIND_UNSPECIFIED")]
    SPANKINDUNSPECIFIED,
    

    /// Indicates that the span is used internally. Default value.
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    

    /// Indicates that the span covers server-side handling of an RPC or other remote network request.
    ///
    /// "SERVER"
    #[serde(rename="SERVER")]
    SERVER,
    

    /// Indicates that the span covers the client-side wrapper around an RPC or other remote request.
    ///
    /// "CLIENT"
    #[serde(rename="CLIENT")]
    CLIENT,
    

    /// Indicates that the span describes producer sending a message to a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. publishing a message to a pubsub service).
    ///
    /// "PRODUCER"
    #[serde(rename="PRODUCER")]
    PRODUCER,
    

    /// Indicates that the span describes consumer receiving a message from a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. receiving a message from a pubsub service subscription).
    ///
    /// "CONSUMER"
    #[serde(rename="CONSUMER")]
    CONSUMER,
}

impl AsRef<str> for TraceSpanSpanKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TraceSpanSpanKindEnum::SPANKINDUNSPECIFIED => "SPAN_KIND_UNSPECIFIED",
            TraceSpanSpanKindEnum::INTERNAL => "INTERNAL",
            TraceSpanSpanKindEnum::SERVER => "SERVER",
            TraceSpanSpanKindEnum::CLIENT => "CLIENT",
            TraceSpanSpanKindEnum::PRODUCER => "PRODUCER",
            TraceSpanSpanKindEnum::CONSUMER => "CONSUMER",
        }
    }
}

impl std::convert::TryFrom< &str> for TraceSpanSpanKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPAN_KIND_UNSPECIFIED" => Ok(TraceSpanSpanKindEnum::SPANKINDUNSPECIFIED),
           "INTERNAL" => Ok(TraceSpanSpanKindEnum::INTERNAL),
           "SERVER" => Ok(TraceSpanSpanKindEnum::SERVER),
           "CLIENT" => Ok(TraceSpanSpanKindEnum::CLIENT),
           "PRODUCER" => Ok(TraceSpanSpanKindEnum::PRODUCER),
           "CONSUMER" => Ok(TraceSpanSpanKindEnum::CONSUMER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TraceSpanSpanKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


