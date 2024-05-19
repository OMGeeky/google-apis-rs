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


// region ChannelStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of a Channel.
pub enum ChannelStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The PENDING state indicates that a Channel has been created successfully and there is a new activation token available for the subscriber to use to convey the Channel to the provider in order to create a Connection.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The ACTIVE state indicates that a Channel has been successfully connected with the event provider. An ACTIVE Channel is ready to receive and route events from the event provider.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The INACTIVE state indicates that the Channel cannot receive events permanently. There are two possible cases this state can happen: 1. The SaaS provider disconnected from this Channel. 2. The Channel activation token has expired but the SaaS provider wasn't connected. To re-establish a Connection with a provider, the subscriber should create a new Channel and give it to the provider.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for ChannelStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ChannelStateEnum::PENDING => "PENDING",
            ChannelStateEnum::ACTIVE => "ACTIVE",
            ChannelStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ChannelStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(ChannelStateEnum::PENDING),
           "ACTIVE" => Ok(ChannelStateEnum::ACTIVE),
           "INACTIVE" => Ok(ChannelStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StateConditionCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The canonical code of the condition.
pub enum StateConditionCodeEnum {
    

    /// Not an error; returned on success. HTTP Mapping: 200 OK
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request
    ///
    /// "INVALID_ARGUMENT"
    #[serde(rename="INVALID_ARGUMENT")]
    INVALIDARGUMENT,
    

    /// The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict
    ///
    /// "ALREADY_EXISTS"
    #[serde(rename="ALREADY_EXISTS")]
    ALREADYEXISTS,
    

    /// The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized
    ///
    /// "UNAUTHENTICATED"
    #[serde(rename="UNAUTHENTICATED")]
    UNAUTHENTICATED,
    

    /// Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests
    ///
    /// "RESOURCE_EXHAUSTED"
    #[serde(rename="RESOURCE_EXHAUSTED")]
    RESOURCEEXHAUSTED,
    

    /// The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level. For example, when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence. (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. For example, if an "rmdir" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request
    ///
    /// "FAILED_PRECONDITION"
    #[serde(rename="FAILED_PRECONDITION")]
    FAILEDPRECONDITION,
    

    /// The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
    

    /// The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request
    ///
    /// "OUT_OF_RANGE"
    #[serde(rename="OUT_OF_RANGE")]
    OUTOFRANGE,
    

    /// The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    

    /// The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error
    ///
    /// "DATA_LOSS"
    #[serde(rename="DATA_LOSS")]
    DATALOSS,
}

impl AsRef<str> for StateConditionCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StateConditionCodeEnum::OK => "OK",
            StateConditionCodeEnum::CANCELLED => "CANCELLED",
            StateConditionCodeEnum::UNKNOWN => "UNKNOWN",
            StateConditionCodeEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            StateConditionCodeEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            StateConditionCodeEnum::NOTFOUND => "NOT_FOUND",
            StateConditionCodeEnum::ALREADYEXISTS => "ALREADY_EXISTS",
            StateConditionCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            StateConditionCodeEnum::UNAUTHENTICATED => "UNAUTHENTICATED",
            StateConditionCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            StateConditionCodeEnum::FAILEDPRECONDITION => "FAILED_PRECONDITION",
            StateConditionCodeEnum::ABORTED => "ABORTED",
            StateConditionCodeEnum::OUTOFRANGE => "OUT_OF_RANGE",
            StateConditionCodeEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            StateConditionCodeEnum::INTERNAL => "INTERNAL",
            StateConditionCodeEnum::UNAVAILABLE => "UNAVAILABLE",
            StateConditionCodeEnum::DATALOSS => "DATA_LOSS",
        }
    }
}

impl std::convert::TryFrom< &str> for StateConditionCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OK" => Ok(StateConditionCodeEnum::OK),
           "CANCELLED" => Ok(StateConditionCodeEnum::CANCELLED),
           "UNKNOWN" => Ok(StateConditionCodeEnum::UNKNOWN),
           "INVALID_ARGUMENT" => Ok(StateConditionCodeEnum::INVALIDARGUMENT),
           "DEADLINE_EXCEEDED" => Ok(StateConditionCodeEnum::DEADLINEEXCEEDED),
           "NOT_FOUND" => Ok(StateConditionCodeEnum::NOTFOUND),
           "ALREADY_EXISTS" => Ok(StateConditionCodeEnum::ALREADYEXISTS),
           "PERMISSION_DENIED" => Ok(StateConditionCodeEnum::PERMISSIONDENIED),
           "UNAUTHENTICATED" => Ok(StateConditionCodeEnum::UNAUTHENTICATED),
           "RESOURCE_EXHAUSTED" => Ok(StateConditionCodeEnum::RESOURCEEXHAUSTED),
           "FAILED_PRECONDITION" => Ok(StateConditionCodeEnum::FAILEDPRECONDITION),
           "ABORTED" => Ok(StateConditionCodeEnum::ABORTED),
           "OUT_OF_RANGE" => Ok(StateConditionCodeEnum::OUTOFRANGE),
           "UNIMPLEMENTED" => Ok(StateConditionCodeEnum::UNIMPLEMENTED),
           "INTERNAL" => Ok(StateConditionCodeEnum::INTERNAL),
           "UNAVAILABLE" => Ok(StateConditionCodeEnum::UNAVAILABLE),
           "DATA_LOSS" => Ok(StateConditionCodeEnum::DATALOSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StateConditionCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


