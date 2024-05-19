use super::*;



// region CryptoKeyVersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the CryptoKeyVersion.
pub enum CryptoKeyVersionStateEnum {
    

    /// Not specified.
    ///
    /// "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_VERSION_STATE_UNSPECIFIED")]
    CRYPTOKEYVERSIONSTATEUNSPECIFIED,
    

    /// This version may be used in Encrypt and
Decrypt requests.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// This version may not be used, but the key material is still available,
and the version can be placed back into the ENABLED state.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// This version is destroyed, and the key material is no longer stored.
A version may not leave this state once entered.
    ///
    /// "DESTROYED"
    #[serde(rename="DESTROYED")]
    DESTROYED,
    

    /// This version is scheduled for destruction, and will be destroyed soon.
Call
RestoreCryptoKeyVersion
to put it back into the DISABLED state.
    ///
    /// "DESTROY_SCHEDULED"
    #[serde(rename="DESTROY_SCHEDULED")]
    DESTROYSCHEDULED,
}

impl AsRef<str> for CryptoKeyVersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyVersionStateEnum::CRYPTOKEYVERSIONSTATEUNSPECIFIED => "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED",
            CryptoKeyVersionStateEnum::ENABLED => "ENABLED",
            CryptoKeyVersionStateEnum::DISABLED => "DISABLED",
            CryptoKeyVersionStateEnum::DESTROYED => "DESTROYED",
            CryptoKeyVersionStateEnum::DESTROYSCHEDULED => "DESTROY_SCHEDULED",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyVersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => Ok(CryptoKeyVersionStateEnum::CRYPTOKEYVERSIONSTATEUNSPECIFIED),
           "ENABLED" => Ok(CryptoKeyVersionStateEnum::ENABLED),
           "DISABLED" => Ok(CryptoKeyVersionStateEnum::DISABLED),
           "DESTROYED" => Ok(CryptoKeyVersionStateEnum::DESTROYED),
           "DESTROY_SCHEDULED" => Ok(CryptoKeyVersionStateEnum::DESTROYSCHEDULED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyVersionStateEnum {
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


// region CryptoKeyPurposeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The immutable purpose of this CryptoKey. Currently, the only acceptable
purpose is ENCRYPT_DECRYPT.
pub enum CryptoKeyPurposeEnum {
    

    /// Not specified.
    ///
    /// "CRYPTO_KEY_PURPOSE_UNSPECIFIED"
    #[serde(rename="CRYPTO_KEY_PURPOSE_UNSPECIFIED")]
    CRYPTOKEYPURPOSEUNSPECIFIED,
    

    /// CryptoKeys with this purpose may be used with
Encrypt and
Decrypt.
    ///
    /// "ENCRYPT_DECRYPT"
    #[serde(rename="ENCRYPT_DECRYPT")]
    ENCRYPTDECRYPT,
}

impl AsRef<str> for CryptoKeyPurposeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CryptoKeyPurposeEnum::CRYPTOKEYPURPOSEUNSPECIFIED => "CRYPTO_KEY_PURPOSE_UNSPECIFIED",
            CryptoKeyPurposeEnum::ENCRYPTDECRYPT => "ENCRYPT_DECRYPT",
        }
    }
}

impl std::convert::TryFrom< &str> for CryptoKeyPurposeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => Ok(CryptoKeyPurposeEnum::CRYPTOKEYPURPOSEUNSPECIFIED),
           "ENCRYPT_DECRYPT" => Ok(CryptoKeyPurposeEnum::ENCRYPTDECRYPT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CryptoKeyPurposeEnum {
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
    

    /// Matching 'Entries' grant access and the caller promises to log
the request per the returned log_configs.
    ///
    /// "ALLOW_WITH_LOG"
    #[serde(rename="ALLOW_WITH_LOG")]
    ALLOWWITHLOG,
    

    /// Matching 'Entries' deny access.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
    

    /// Matching 'Entries' deny access and the caller promises to log
the request per the returned log_configs.
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
    

    /// The condition is true if the subject (or any element of it if it is
a set) matches any of the supplied values.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// The condition is true if the subject (or every element of it if it is
a set) matches none of the supplied values.
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
/// Trusted attributes supplied by any service that owns resources and uses
the IAM system for access control.
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
    

    /// The principal (even if an authority selector is present), which
must only be used for attribution, not authorization.
    ///
    /// "ATTRIBUTION"
    #[serde(rename="ATTRIBUTION")]
    ATTRIBUTION,
    

    /// An approver (distinct from the requester) that has authorized this
request.
When used with IN, the condition indicates that one of the approvers
associated with the request matches the specified principal, or is a
member of the specified group. Approvers can only grant additional
access, and are thus only used in a strictly positive context
(e.g. ALLOW/IN or DENY/NOT_IN).
See: go/rpc-security-policy-dynamicauth.
    ///
    /// "APPROVER"
    #[serde(rename="APPROVER")]
    APPROVER,
    

    /// What types of justifications have been supplied with this request.
String values should match enum names from tech.iam.JustificationType,
e.g. "MANUAL_STRING". It is not permitted to grant access based on
the *absence* of a justification, so justification conditions can only
be used in a "positive" context (e.g., ALLOW/IN or DENY/NOT_IN).

Multiple justifications, e.g., a Buganizer ID and a manually-entered
reason, are normal and supported.
    ///
    /// "JUSTIFICATION_TYPE"
    #[serde(rename="JUSTIFICATION_TYPE")]
    JUSTIFICATIONTYPE,
}

impl AsRef<str> for ConditionIamEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionIamEnum::NOATTR => "NO_ATTR",
            ConditionIamEnum::AUTHORITY => "AUTHORITY",
            ConditionIamEnum::ATTRIBUTION => "ATTRIBUTION",
            ConditionIamEnum::APPROVER => "APPROVER",
            ConditionIamEnum::JUSTIFICATIONTYPE => "JUSTIFICATION_TYPE",
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
           "APPROVER" => Ok(ConditionIamEnum::APPROVER),
           "JUSTIFICATION_TYPE" => Ok(ConditionIamEnum::JUSTIFICATIONTYPE),
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


