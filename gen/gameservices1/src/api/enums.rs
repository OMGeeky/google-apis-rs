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


// region AuthorizationLoggingOptionPermissionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the permission that was checked.
pub enum AuthorizationLoggingOptionPermissionTypeEnum {
    

    /// Default. Should not be used.
    ///
    /// "PERMISSION_TYPE_UNSPECIFIED"
    #[serde(rename="PERMISSION_TYPE_UNSPECIFIED")]
    PERMISSIONTYPEUNSPECIFIED,
    

    /// A read of admin (meta) data.
    ///
    /// "ADMIN_READ"
    #[serde(rename="ADMIN_READ")]
    ADMINREAD,
    

    /// A write of admin (meta) data.
    ///
    /// "ADMIN_WRITE"
    #[serde(rename="ADMIN_WRITE")]
    ADMINWRITE,
    

    /// A read of standard data.
    ///
    /// "DATA_READ"
    #[serde(rename="DATA_READ")]
    DATAREAD,
    

    /// A write of standard data.
    ///
    /// "DATA_WRITE"
    #[serde(rename="DATA_WRITE")]
    DATAWRITE,
}

impl AsRef<str> for AuthorizationLoggingOptionPermissionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuthorizationLoggingOptionPermissionTypeEnum::PERMISSIONTYPEUNSPECIFIED => "PERMISSION_TYPE_UNSPECIFIED",
            AuthorizationLoggingOptionPermissionTypeEnum::ADMINREAD => "ADMIN_READ",
            AuthorizationLoggingOptionPermissionTypeEnum::ADMINWRITE => "ADMIN_WRITE",
            AuthorizationLoggingOptionPermissionTypeEnum::DATAREAD => "DATA_READ",
            AuthorizationLoggingOptionPermissionTypeEnum::DATAWRITE => "DATA_WRITE",
        }
    }
}

impl std::convert::TryFrom< &str> for AuthorizationLoggingOptionPermissionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_TYPE_UNSPECIFIED" => Ok(AuthorizationLoggingOptionPermissionTypeEnum::PERMISSIONTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(AuthorizationLoggingOptionPermissionTypeEnum::ADMINREAD),
           "ADMIN_WRITE" => Ok(AuthorizationLoggingOptionPermissionTypeEnum::ADMINWRITE),
           "DATA_READ" => Ok(AuthorizationLoggingOptionPermissionTypeEnum::DATAREAD),
           "DATA_WRITE" => Ok(AuthorizationLoggingOptionPermissionTypeEnum::DATAWRITE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuthorizationLoggingOptionPermissionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudAuditOptionLogNameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log_name to populate in the Cloud Audit Record.
pub enum CloudAuditOptionLogNameEnum {
    

    /// Default. Should not be used.
    ///
    /// "UNSPECIFIED_LOG_NAME"
    #[serde(rename="UNSPECIFIED_LOG_NAME")]
    UNSPECIFIEDLOGNAME,
    

    /// Corresponds to "cloudaudit.googleapis.com/activity"
    ///
    /// "ADMIN_ACTIVITY"
    #[serde(rename="ADMIN_ACTIVITY")]
    ADMINACTIVITY,
    

    /// Corresponds to "cloudaudit.googleapis.com/data_access"
    ///
    /// "DATA_ACCESS"
    #[serde(rename="DATA_ACCESS")]
    DATAACCESS,
}

impl AsRef<str> for CloudAuditOptionLogNameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudAuditOptionLogNameEnum::UNSPECIFIEDLOGNAME => "UNSPECIFIED_LOG_NAME",
            CloudAuditOptionLogNameEnum::ADMINACTIVITY => "ADMIN_ACTIVITY",
            CloudAuditOptionLogNameEnum::DATAACCESS => "DATA_ACCESS",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudAuditOptionLogNameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_LOG_NAME" => Ok(CloudAuditOptionLogNameEnum::UNSPECIFIEDLOGNAME),
           "ADMIN_ACTIVITY" => Ok(CloudAuditOptionLogNameEnum::ADMINACTIVITY),
           "DATA_ACCESS" => Ok(CloudAuditOptionLogNameEnum::DATAACCESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudAuditOptionLogNameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConditionIamEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Trusted attributes supplied by the IAM system.
pub enum ConditionIamEnum {
    

    /// Default non-attribute.
    ///
    /// "NO_ATTR"
    #[serde(rename="NO_ATTR")]
    NOATTR,
    

    /// Either principal or (if present) authority selector.
    ///
    /// "AUTHORITY"
    #[serde(rename="AUTHORITY")]
    AUTHORITY,
    

    /// The principal (even if an authority selector is present), which must only be used for attribution, not authorization.
    ///
    /// "ATTRIBUTION"
    #[serde(rename="ATTRIBUTION")]
    ATTRIBUTION,
    

    /// Any of the security realms in the IAMContext (go/security-realms). When used with IN, the condition indicates "any of the request's realms match one of the given values; with NOT_IN, "none of the realms match any of the given values". Note that a value can be: - 'self:campus' (i.e., clients that are in the same campus) - 'self:metro' (i.e., clients that are in the same metro) - 'self:cloud-region' (i.e., allow connections from clients that are in the same cloud region) - 'self:prod-region' (i.e., allow connections from clients that are in the same prod region) - 'guardians' (i.e., allow connections from its guardian realms. See go/security-realms-glossary#guardian for more information.) - 'self' [DEPRECATED] (i.e., allow connections from clients that are in the same security realm, which is currently but not guaranteed to be campus-sized) - a realm (e.g., 'campus-abc') - a realm group (e.g., 'realms-for-borg-cell-xx', see: go/realm-groups) A match is determined by a realm group membership check performed by a RealmAclRep object (go/realm-acl-howto). It is not permitted to grant access based on the *absence* of a realm, so realm conditions can only be used in a "positive" context (e.g., ALLOW/IN or DENY/NOT_IN).
    ///
    /// "SECURITY_REALM"
    #[serde(rename="SECURITY_REALM")]
    SECURITYREALM,
    

    /// An approver (distinct from the requester) that has authorized this request. When used with IN, the condition indicates that one of the approvers associated with the request matches the specified principal, or is a member of the specified group. Approvers can only grant additional access, and are thus only used in a strictly positive context (e.g. ALLOW/IN or DENY/NOT_IN).
    ///
    /// "APPROVER"
    #[serde(rename="APPROVER")]
    APPROVER,
    

    /// What types of justifications have been supplied with this request. String values should match enum names from security.credentials.JustificationType, e.g. "MANUAL_STRING". It is not permitted to grant access based on the *absence* of a justification, so justification conditions can only be used in a "positive" context (e.g., ALLOW/IN or DENY/NOT_IN). Multiple justifications, e.g., a Buganizer ID and a manually-entered reason, are normal and supported.
    ///
    /// "JUSTIFICATION_TYPE"
    #[serde(rename="JUSTIFICATION_TYPE")]
    JUSTIFICATIONTYPE,
    

    /// What type of credentials have been supplied with this request. String values should match enum names from security_loas_l2.CredentialsType - currently, only CREDS_TYPE_EMERGENCY is supported. It is not permitted to grant access based on the *absence* of a credentials type, so the conditions can only be used in a "positive" context (e.g., ALLOW/IN or DENY/NOT_IN).
    ///
    /// "CREDENTIALS_TYPE"
    #[serde(rename="CREDENTIALS_TYPE")]
    CREDENTIALSTYPE,
    

    /// EXPERIMENTAL -- DO NOT USE. The conditions can only be used in a "positive" context (e.g., ALLOW/IN or DENY/NOT_IN).
    ///
    /// "CREDS_ASSERTION"
    #[serde(rename="CREDS_ASSERTION")]
    CREDSASSERTION,
}

impl AsRef<str> for ConditionIamEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionIamEnum::NOATTR => "NO_ATTR",
            ConditionIamEnum::AUTHORITY => "AUTHORITY",
            ConditionIamEnum::ATTRIBUTION => "ATTRIBUTION",
            ConditionIamEnum::SECURITYREALM => "SECURITY_REALM",
            ConditionIamEnum::APPROVER => "APPROVER",
            ConditionIamEnum::JUSTIFICATIONTYPE => "JUSTIFICATION_TYPE",
            ConditionIamEnum::CREDENTIALSTYPE => "CREDENTIALS_TYPE",
            ConditionIamEnum::CREDSASSERTION => "CREDS_ASSERTION",
        }
    }
}

impl std::convert::TryFrom< &str> for ConditionIamEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_ATTR" => Ok(ConditionIamEnum::NOATTR),
           "AUTHORITY" => Ok(ConditionIamEnum::AUTHORITY),
           "ATTRIBUTION" => Ok(ConditionIamEnum::ATTRIBUTION),
           "SECURITY_REALM" => Ok(ConditionIamEnum::SECURITYREALM),
           "APPROVER" => Ok(ConditionIamEnum::APPROVER),
           "JUSTIFICATION_TYPE" => Ok(ConditionIamEnum::JUSTIFICATIONTYPE),
           "CREDENTIALS_TYPE" => Ok(ConditionIamEnum::CREDENTIALSTYPE),
           "CREDS_ASSERTION" => Ok(ConditionIamEnum::CREDSASSERTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConditionIamEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConditionOpEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An operator to apply the subject with.
pub enum ConditionOpEnum {
    

    /// Default no-op.
    ///
    /// "NO_OP"
    #[serde(rename="NO_OP")]
    NOOP,
    

    /// DEPRECATED. Use IN instead.
    ///
    /// "EQUALS"
    #[serde(rename="EQUALS")]
    EQUALS,
    

    /// DEPRECATED. Use NOT_IN instead.
    ///
    /// "NOT_EQUALS"
    #[serde(rename="NOT_EQUALS")]
    NOTEQUALS,
    

    /// The condition is true if the subject (or any element of it if it is a set) matches any of the supplied values.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// The condition is true if the subject (or every element of it if it is a set) matches none of the supplied values.
    ///
    /// "NOT_IN"
    #[serde(rename="NOT_IN")]
    NOTIN,
    

    /// Subject is discharged
    ///
    /// "DISCHARGED"
    #[serde(rename="DISCHARGED")]
    DISCHARGED,
}

impl AsRef<str> for ConditionOpEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionOpEnum::NOOP => "NO_OP",
            ConditionOpEnum::EQUALS => "EQUALS",
            ConditionOpEnum::NOTEQUALS => "NOT_EQUALS",
            ConditionOpEnum::IN => "IN",
            ConditionOpEnum::NOTIN => "NOT_IN",
            ConditionOpEnum::DISCHARGED => "DISCHARGED",
        }
    }
}

impl std::convert::TryFrom< &str> for ConditionOpEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_OP" => Ok(ConditionOpEnum::NOOP),
           "EQUALS" => Ok(ConditionOpEnum::EQUALS),
           "NOT_EQUALS" => Ok(ConditionOpEnum::NOTEQUALS),
           "IN" => Ok(ConditionOpEnum::IN),
           "NOT_IN" => Ok(ConditionOpEnum::NOTIN),
           "DISCHARGED" => Ok(ConditionOpEnum::DISCHARGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConditionOpEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConditionSysEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Trusted attributes supplied by any service that owns resources and uses the IAM system for access control.
pub enum ConditionSysEnum {
    

    /// Default non-attribute type
    ///
    /// "NO_ATTR"
    #[serde(rename="NO_ATTR")]
    NOATTR,
    

    /// Region of the resource
    ///
    /// "REGION"
    #[serde(rename="REGION")]
    REGION,
    

    /// Service name
    ///
    /// "SERVICE"
    #[serde(rename="SERVICE")]
    SERVICE,
    

    /// Resource name
    ///
    /// "NAME"
    #[serde(rename="NAME")]
    NAME,
    

    /// IP address of the caller
    ///
    /// "IP"
    #[serde(rename="IP")]
    IP,
}

impl AsRef<str> for ConditionSysEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionSysEnum::NOATTR => "NO_ATTR",
            ConditionSysEnum::REGION => "REGION",
            ConditionSysEnum::SERVICE => "SERVICE",
            ConditionSysEnum::NAME => "NAME",
            ConditionSysEnum::IP => "IP",
        }
    }
}

impl std::convert::TryFrom< &str> for ConditionSysEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_ATTR" => Ok(ConditionSysEnum::NOATTR),
           "REGION" => Ok(ConditionSysEnum::REGION),
           "SERVICE" => Ok(ConditionSysEnum::SERVICE),
           "NAME" => Ok(ConditionSysEnum::NAME),
           "IP" => Ok(ConditionSysEnum::IP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConditionSysEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataAccessOptionLogModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataAccessOptionLogModeEnum {
    

    /// Client is not required to write a partial Gin log immediately after the authorization check. If client chooses to write one and it fails, client may either fail open (allow the operation to continue) or fail closed (handle as a DENY outcome).
    ///
    /// "LOG_MODE_UNSPECIFIED"
    #[serde(rename="LOG_MODE_UNSPECIFIED")]
    LOGMODEUNSPECIFIED,
    

    /// The application's operation in the context of which this authorization check is being made may only be performed if it is successfully logged to Gin. For instance, the authorization library may satisfy this obligation by emitting a partial log entry at authorization check time and only returning ALLOW to the application if it succeeds. If a matching Rule has this directive, but the client has not indicated that it will honor such requirements, then the IAM check will result in authorization failure by setting CheckPolicyResponse.success=false.
    ///
    /// "LOG_FAIL_CLOSED"
    #[serde(rename="LOG_FAIL_CLOSED")]
    LOGFAILCLOSED,
}

impl AsRef<str> for DataAccessOptionLogModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataAccessOptionLogModeEnum::LOGMODEUNSPECIFIED => "LOG_MODE_UNSPECIFIED",
            DataAccessOptionLogModeEnum::LOGFAILCLOSED => "LOG_FAIL_CLOSED",
        }
    }
}

impl std::convert::TryFrom< &str> for DataAccessOptionLogModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_MODE_UNSPECIFIED" => Ok(DataAccessOptionLogModeEnum::LOGMODEUNSPECIFIED),
           "LOG_FAIL_CLOSED" => Ok(DataAccessOptionLogModeEnum::LOGFAILCLOSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataAccessOptionLogModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuleActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required
pub enum RuleActionEnum {
    

    /// Default no action.
    ///
    /// "NO_ACTION"
    #[serde(rename="NO_ACTION")]
    NOACTION,
    

    /// Matching 'Entries' grant access.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Matching 'Entries' grant access and the caller promises to log the request per the returned log_configs.
    ///
    /// "ALLOW_WITH_LOG"
    #[serde(rename="ALLOW_WITH_LOG")]
    ALLOWWITHLOG,
    

    /// Matching 'Entries' deny access.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
    

    /// Matching 'Entries' deny access and the caller promises to log the request per the returned log_configs.
    ///
    /// "DENY_WITH_LOG"
    #[serde(rename="DENY_WITH_LOG")]
    DENYWITHLOG,
    

    /// Matching 'Entries' tell IAM.Check callers to generate logs.
    ///
    /// "LOG"
    #[serde(rename="LOG")]
    LOG,
}

impl AsRef<str> for RuleActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuleActionEnum::NOACTION => "NO_ACTION",
            RuleActionEnum::ALLOW => "ALLOW",
            RuleActionEnum::ALLOWWITHLOG => "ALLOW_WITH_LOG",
            RuleActionEnum::DENY => "DENY",
            RuleActionEnum::DENYWITHLOG => "DENY_WITH_LOG",
            RuleActionEnum::LOG => "LOG",
        }
    }
}

impl std::convert::TryFrom< &str> for RuleActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_ACTION" => Ok(RuleActionEnum::NOACTION),
           "ALLOW" => Ok(RuleActionEnum::ALLOW),
           "ALLOW_WITH_LOG" => Ok(RuleActionEnum::ALLOWWITHLOG),
           "DENY" => Ok(RuleActionEnum::DENY),
           "DENY_WITH_LOG" => Ok(RuleActionEnum::DENYWITHLOG),
           "LOG" => Ok(RuleActionEnum::LOG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuleActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


