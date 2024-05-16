use super::*;



// region AgentPoolStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the state of the AgentPool.
pub enum AgentPoolStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// This is an initialization state. During this stage, resources are allocated for the AgentPool.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Determines that the AgentPool is created for use. At this state, Agents can join the AgentPool and participate in the transfer jobs in that pool.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// Determines that the AgentPool deletion has been initiated, and all the resources are scheduled to be cleaned up and freed.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for AgentPoolStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AgentPoolStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AgentPoolStateEnum::CREATING => "CREATING",
            AgentPoolStateEnum::CREATED => "CREATED",
            AgentPoolStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for AgentPoolStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AgentPoolStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(AgentPoolStateEnum::CREATING),
           "CREATED" => Ok(AgentPoolStateEnum::CREATED),
           "DELETING" => Ok(AgentPoolStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AgentPoolStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ErrorSummaryErrorCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required.
pub enum ErrorSummaryErrorCodeEnum {
    

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

impl AsRef<str> for ErrorSummaryErrorCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ErrorSummaryErrorCodeEnum::OK => "OK",
            ErrorSummaryErrorCodeEnum::CANCELLED => "CANCELLED",
            ErrorSummaryErrorCodeEnum::UNKNOWN => "UNKNOWN",
            ErrorSummaryErrorCodeEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            ErrorSummaryErrorCodeEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            ErrorSummaryErrorCodeEnum::NOTFOUND => "NOT_FOUND",
            ErrorSummaryErrorCodeEnum::ALREADYEXISTS => "ALREADY_EXISTS",
            ErrorSummaryErrorCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            ErrorSummaryErrorCodeEnum::UNAUTHENTICATED => "UNAUTHENTICATED",
            ErrorSummaryErrorCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            ErrorSummaryErrorCodeEnum::FAILEDPRECONDITION => "FAILED_PRECONDITION",
            ErrorSummaryErrorCodeEnum::ABORTED => "ABORTED",
            ErrorSummaryErrorCodeEnum::OUTOFRANGE => "OUT_OF_RANGE",
            ErrorSummaryErrorCodeEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            ErrorSummaryErrorCodeEnum::INTERNAL => "INTERNAL",
            ErrorSummaryErrorCodeEnum::UNAVAILABLE => "UNAVAILABLE",
            ErrorSummaryErrorCodeEnum::DATALOSS => "DATA_LOSS",
        }
    }
}

impl std::convert::TryFrom< &str> for ErrorSummaryErrorCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OK" => Ok(ErrorSummaryErrorCodeEnum::OK),
           "CANCELLED" => Ok(ErrorSummaryErrorCodeEnum::CANCELLED),
           "UNKNOWN" => Ok(ErrorSummaryErrorCodeEnum::UNKNOWN),
           "INVALID_ARGUMENT" => Ok(ErrorSummaryErrorCodeEnum::INVALIDARGUMENT),
           "DEADLINE_EXCEEDED" => Ok(ErrorSummaryErrorCodeEnum::DEADLINEEXCEEDED),
           "NOT_FOUND" => Ok(ErrorSummaryErrorCodeEnum::NOTFOUND),
           "ALREADY_EXISTS" => Ok(ErrorSummaryErrorCodeEnum::ALREADYEXISTS),
           "PERMISSION_DENIED" => Ok(ErrorSummaryErrorCodeEnum::PERMISSIONDENIED),
           "UNAUTHENTICATED" => Ok(ErrorSummaryErrorCodeEnum::UNAUTHENTICATED),
           "RESOURCE_EXHAUSTED" => Ok(ErrorSummaryErrorCodeEnum::RESOURCEEXHAUSTED),
           "FAILED_PRECONDITION" => Ok(ErrorSummaryErrorCodeEnum::FAILEDPRECONDITION),
           "ABORTED" => Ok(ErrorSummaryErrorCodeEnum::ABORTED),
           "OUT_OF_RANGE" => Ok(ErrorSummaryErrorCodeEnum::OUTOFRANGE),
           "UNIMPLEMENTED" => Ok(ErrorSummaryErrorCodeEnum::UNIMPLEMENTED),
           "INTERNAL" => Ok(ErrorSummaryErrorCodeEnum::INTERNAL),
           "UNAVAILABLE" => Ok(ErrorSummaryErrorCodeEnum::UNAVAILABLE),
           "DATA_LOSS" => Ok(ErrorSummaryErrorCodeEnum::DATALOSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ErrorSummaryErrorCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoggingConfigLogActionStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// States in which `log_actions` are logged. If empty, no logs are generated. Not supported for transfers with PosixFilesystem data sources; use enable_onprem_gcs_transfer_logs instead.
pub enum LoggingConfigLogActionStatesEnum {
    

    /// Default value. This value is unused.
    ///
    /// "LOGGABLE_ACTION_STATE_UNSPECIFIED"
    #[serde(rename="LOGGABLE_ACTION_STATE_UNSPECIFIED")]
    LOGGABLEACTIONSTATEUNSPECIFIED,
    

    /// `LoggableAction` completed successfully. `SUCCEEDED` actions are logged as INFO.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// `LoggableAction` terminated in an error state. `FAILED` actions are logged as ERROR.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for LoggingConfigLogActionStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoggingConfigLogActionStatesEnum::LOGGABLEACTIONSTATEUNSPECIFIED => "LOGGABLE_ACTION_STATE_UNSPECIFIED",
            LoggingConfigLogActionStatesEnum::SUCCEEDED => "SUCCEEDED",
            LoggingConfigLogActionStatesEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for LoggingConfigLogActionStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGGABLE_ACTION_STATE_UNSPECIFIED" => Ok(LoggingConfigLogActionStatesEnum::LOGGABLEACTIONSTATEUNSPECIFIED),
           "SUCCEEDED" => Ok(LoggingConfigLogActionStatesEnum::SUCCEEDED),
           "FAILED" => Ok(LoggingConfigLogActionStatesEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoggingConfigLogActionStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoggingConfigLogActionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the actions to be logged. If empty, no logs are generated. Not supported for transfers with PosixFilesystem data sources; use enable_onprem_gcs_transfer_logs instead.
pub enum LoggingConfigLogActionsEnum {
    

    /// Default value. This value is unused.
    ///
    /// "LOGGABLE_ACTION_UNSPECIFIED"
    #[serde(rename="LOGGABLE_ACTION_UNSPECIFIED")]
    LOGGABLEACTIONUNSPECIFIED,
    

    /// Listing objects in a bucket.
    ///
    /// "FIND"
    #[serde(rename="FIND")]
    FIND,
    

    /// Deleting objects at the source or the destination.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// Copying objects to Google Cloud Storage.
    ///
    /// "COPY"
    #[serde(rename="COPY")]
    COPY,
}

impl AsRef<str> for LoggingConfigLogActionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoggingConfigLogActionsEnum::LOGGABLEACTIONUNSPECIFIED => "LOGGABLE_ACTION_UNSPECIFIED",
            LoggingConfigLogActionsEnum::FIND => "FIND",
            LoggingConfigLogActionsEnum::DELETE => "DELETE",
            LoggingConfigLogActionsEnum::COPY => "COPY",
        }
    }
}

impl std::convert::TryFrom< &str> for LoggingConfigLogActionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGGABLE_ACTION_UNSPECIFIED" => Ok(LoggingConfigLogActionsEnum::LOGGABLEACTIONUNSPECIFIED),
           "FIND" => Ok(LoggingConfigLogActionsEnum::FIND),
           "DELETE" => Ok(LoggingConfigLogActionsEnum::DELETE),
           "COPY" => Ok(LoggingConfigLogActionsEnum::COPY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoggingConfigLogActionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionAclEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each object's ACLs should be preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as ACL_DESTINATION_BUCKET_DEFAULT.
pub enum MetadataOptionAclEnum {
    

    /// ACL behavior is unspecified.
    ///
    /// "ACL_UNSPECIFIED"
    #[serde(rename="ACL_UNSPECIFIED")]
    ACLUNSPECIFIED,
    

    /// Use the destination bucket's default object ACLS, if applicable.
    ///
    /// "ACL_DESTINATION_BUCKET_DEFAULT"
    #[serde(rename="ACL_DESTINATION_BUCKET_DEFAULT")]
    ACLDESTINATIONBUCKETDEFAULT,
    

    /// Preserve the object's original ACLs. This requires the service account to have `storage.objects.getIamPolicy` permission for the source object. [Uniform bucket-level access](https://cloud.google.com/storage/docs/uniform-bucket-level-access) must not be enabled on either the source or destination buckets.
    ///
    /// "ACL_PRESERVE"
    #[serde(rename="ACL_PRESERVE")]
    ACLPRESERVE,
}

impl AsRef<str> for MetadataOptionAclEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionAclEnum::ACLUNSPECIFIED => "ACL_UNSPECIFIED",
            MetadataOptionAclEnum::ACLDESTINATIONBUCKETDEFAULT => "ACL_DESTINATION_BUCKET_DEFAULT",
            MetadataOptionAclEnum::ACLPRESERVE => "ACL_PRESERVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionAclEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACL_UNSPECIFIED" => Ok(MetadataOptionAclEnum::ACLUNSPECIFIED),
           "ACL_DESTINATION_BUCKET_DEFAULT" => Ok(MetadataOptionAclEnum::ACLDESTINATIONBUCKETDEFAULT),
           "ACL_PRESERVE" => Ok(MetadataOptionAclEnum::ACLPRESERVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionAclEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionGidEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each file's POSIX group ID (GID) attribute should be handled by the transfer. By default, GID is not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
pub enum MetadataOptionGidEnum {
    

    /// GID behavior is unspecified.
    ///
    /// "GID_UNSPECIFIED"
    #[serde(rename="GID_UNSPECIFIED")]
    GIDUNSPECIFIED,
    

    /// Do not preserve GID during a transfer job.
    ///
    /// "GID_SKIP"
    #[serde(rename="GID_SKIP")]
    GIDSKIP,
    

    /// Preserve GID during a transfer job.
    ///
    /// "GID_NUMBER"
    #[serde(rename="GID_NUMBER")]
    GIDNUMBER,
}

impl AsRef<str> for MetadataOptionGidEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionGidEnum::GIDUNSPECIFIED => "GID_UNSPECIFIED",
            MetadataOptionGidEnum::GIDSKIP => "GID_SKIP",
            MetadataOptionGidEnum::GIDNUMBER => "GID_NUMBER",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionGidEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GID_UNSPECIFIED" => Ok(MetadataOptionGidEnum::GIDUNSPECIFIED),
           "GID_SKIP" => Ok(MetadataOptionGidEnum::GIDSKIP),
           "GID_NUMBER" => Ok(MetadataOptionGidEnum::GIDNUMBER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionGidEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionKmsKeyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each object's Cloud KMS customer-managed encryption key (CMEK) is preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as KMS_KEY_DESTINATION_BUCKET_DEFAULT.
pub enum MetadataOptionKmsKeyEnum {
    

    /// KmsKey behavior is unspecified.
    ///
    /// "KMS_KEY_UNSPECIFIED"
    #[serde(rename="KMS_KEY_UNSPECIFIED")]
    KMSKEYUNSPECIFIED,
    

    /// Use the destination bucket's default encryption settings.
    ///
    /// "KMS_KEY_DESTINATION_BUCKET_DEFAULT"
    #[serde(rename="KMS_KEY_DESTINATION_BUCKET_DEFAULT")]
    KMSKEYDESTINATIONBUCKETDEFAULT,
    

    /// Preserve the object's original Cloud KMS customer-managed encryption key (CMEK) if present. Objects that do not use a Cloud KMS encryption key will be encrypted using the destination bucket's encryption settings.
    ///
    /// "KMS_KEY_PRESERVE"
    #[serde(rename="KMS_KEY_PRESERVE")]
    KMSKEYPRESERVE,
}

impl AsRef<str> for MetadataOptionKmsKeyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionKmsKeyEnum::KMSKEYUNSPECIFIED => "KMS_KEY_UNSPECIFIED",
            MetadataOptionKmsKeyEnum::KMSKEYDESTINATIONBUCKETDEFAULT => "KMS_KEY_DESTINATION_BUCKET_DEFAULT",
            MetadataOptionKmsKeyEnum::KMSKEYPRESERVE => "KMS_KEY_PRESERVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionKmsKeyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KMS_KEY_UNSPECIFIED" => Ok(MetadataOptionKmsKeyEnum::KMSKEYUNSPECIFIED),
           "KMS_KEY_DESTINATION_BUCKET_DEFAULT" => Ok(MetadataOptionKmsKeyEnum::KMSKEYDESTINATIONBUCKETDEFAULT),
           "KMS_KEY_PRESERVE" => Ok(MetadataOptionKmsKeyEnum::KMSKEYPRESERVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionKmsKeyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each file's mode attribute should be handled by the transfer. By default, mode is not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
pub enum MetadataOptionModeEnum {
    

    /// Mode behavior is unspecified.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Do not preserve mode during a transfer job.
    ///
    /// "MODE_SKIP"
    #[serde(rename="MODE_SKIP")]
    MODESKIP,
    

    /// Preserve mode during a transfer job.
    ///
    /// "MODE_PRESERVE"
    #[serde(rename="MODE_PRESERVE")]
    MODEPRESERVE,
}

impl AsRef<str> for MetadataOptionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            MetadataOptionModeEnum::MODESKIP => "MODE_SKIP",
            MetadataOptionModeEnum::MODEPRESERVE => "MODE_PRESERVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(MetadataOptionModeEnum::MODEUNSPECIFIED),
           "MODE_SKIP" => Ok(MetadataOptionModeEnum::MODESKIP),
           "MODE_PRESERVE" => Ok(MetadataOptionModeEnum::MODEPRESERVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionStorageClassEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the storage class to set on objects being transferred to Google Cloud Storage buckets. If unspecified, the default behavior is the same as STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT.
pub enum MetadataOptionStorageClassEnum {
    

    /// Storage class behavior is unspecified.
    ///
    /// "STORAGE_CLASS_UNSPECIFIED"
    #[serde(rename="STORAGE_CLASS_UNSPECIFIED")]
    STORAGECLASSUNSPECIFIED,
    

    /// Use the destination bucket's default storage class.
    ///
    /// "STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT"
    #[serde(rename="STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT")]
    STORAGECLASSDESTINATIONBUCKETDEFAULT,
    

    /// Preserve the object's original storage class. This is only supported for transfers from Google Cloud Storage buckets. REGIONAL and MULTI_REGIONAL storage classes will be mapped to STANDARD to ensure they can be written to the destination bucket.
    ///
    /// "STORAGE_CLASS_PRESERVE"
    #[serde(rename="STORAGE_CLASS_PRESERVE")]
    STORAGECLASSPRESERVE,
    

    /// Set the storage class to STANDARD.
    ///
    /// "STORAGE_CLASS_STANDARD"
    #[serde(rename="STORAGE_CLASS_STANDARD")]
    STORAGECLASSSTANDARD,
    

    /// Set the storage class to NEARLINE.
    ///
    /// "STORAGE_CLASS_NEARLINE"
    #[serde(rename="STORAGE_CLASS_NEARLINE")]
    STORAGECLASSNEARLINE,
    

    /// Set the storage class to COLDLINE.
    ///
    /// "STORAGE_CLASS_COLDLINE"
    #[serde(rename="STORAGE_CLASS_COLDLINE")]
    STORAGECLASSCOLDLINE,
    

    /// Set the storage class to ARCHIVE.
    ///
    /// "STORAGE_CLASS_ARCHIVE"
    #[serde(rename="STORAGE_CLASS_ARCHIVE")]
    STORAGECLASSARCHIVE,
}

impl AsRef<str> for MetadataOptionStorageClassEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionStorageClassEnum::STORAGECLASSUNSPECIFIED => "STORAGE_CLASS_UNSPECIFIED",
            MetadataOptionStorageClassEnum::STORAGECLASSDESTINATIONBUCKETDEFAULT => "STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT",
            MetadataOptionStorageClassEnum::STORAGECLASSPRESERVE => "STORAGE_CLASS_PRESERVE",
            MetadataOptionStorageClassEnum::STORAGECLASSSTANDARD => "STORAGE_CLASS_STANDARD",
            MetadataOptionStorageClassEnum::STORAGECLASSNEARLINE => "STORAGE_CLASS_NEARLINE",
            MetadataOptionStorageClassEnum::STORAGECLASSCOLDLINE => "STORAGE_CLASS_COLDLINE",
            MetadataOptionStorageClassEnum::STORAGECLASSARCHIVE => "STORAGE_CLASS_ARCHIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionStorageClassEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_CLASS_UNSPECIFIED" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSUNSPECIFIED),
           "STORAGE_CLASS_DESTINATION_BUCKET_DEFAULT" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSDESTINATIONBUCKETDEFAULT),
           "STORAGE_CLASS_PRESERVE" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSPRESERVE),
           "STORAGE_CLASS_STANDARD" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSSTANDARD),
           "STORAGE_CLASS_NEARLINE" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSNEARLINE),
           "STORAGE_CLASS_COLDLINE" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSCOLDLINE),
           "STORAGE_CLASS_ARCHIVE" => Ok(MetadataOptionStorageClassEnum::STORAGECLASSARCHIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionStorageClassEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionSymlinkEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how symlinks should be handled by the transfer. By default, symlinks are not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
pub enum MetadataOptionSymlinkEnum {
    

    /// Symlink behavior is unspecified.
    ///
    /// "SYMLINK_UNSPECIFIED"
    #[serde(rename="SYMLINK_UNSPECIFIED")]
    SYMLINKUNSPECIFIED,
    

    /// Do not preserve symlinks during a transfer job.
    ///
    /// "SYMLINK_SKIP"
    #[serde(rename="SYMLINK_SKIP")]
    SYMLINKSKIP,
    

    /// Preserve symlinks during a transfer job.
    ///
    /// "SYMLINK_PRESERVE"
    #[serde(rename="SYMLINK_PRESERVE")]
    SYMLINKPRESERVE,
}

impl AsRef<str> for MetadataOptionSymlinkEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionSymlinkEnum::SYMLINKUNSPECIFIED => "SYMLINK_UNSPECIFIED",
            MetadataOptionSymlinkEnum::SYMLINKSKIP => "SYMLINK_SKIP",
            MetadataOptionSymlinkEnum::SYMLINKPRESERVE => "SYMLINK_PRESERVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionSymlinkEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYMLINK_UNSPECIFIED" => Ok(MetadataOptionSymlinkEnum::SYMLINKUNSPECIFIED),
           "SYMLINK_SKIP" => Ok(MetadataOptionSymlinkEnum::SYMLINKSKIP),
           "SYMLINK_PRESERVE" => Ok(MetadataOptionSymlinkEnum::SYMLINKPRESERVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionSymlinkEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionTemporaryHoldEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each object's temporary hold status should be preserved for transfers between Google Cloud Storage buckets. If unspecified, the default behavior is the same as TEMPORARY_HOLD_PRESERVE.
pub enum MetadataOptionTemporaryHoldEnum {
    

    /// Temporary hold behavior is unspecified.
    ///
    /// "TEMPORARY_HOLD_UNSPECIFIED"
    #[serde(rename="TEMPORARY_HOLD_UNSPECIFIED")]
    TEMPORARYHOLDUNSPECIFIED,
    

    /// Do not set a temporary hold on the destination object.
    ///
    /// "TEMPORARY_HOLD_SKIP"
    #[serde(rename="TEMPORARY_HOLD_SKIP")]
    TEMPORARYHOLDSKIP,
    

    /// Preserve the object's original temporary hold status.
    ///
    /// "TEMPORARY_HOLD_PRESERVE"
    #[serde(rename="TEMPORARY_HOLD_PRESERVE")]
    TEMPORARYHOLDPRESERVE,
}

impl AsRef<str> for MetadataOptionTemporaryHoldEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionTemporaryHoldEnum::TEMPORARYHOLDUNSPECIFIED => "TEMPORARY_HOLD_UNSPECIFIED",
            MetadataOptionTemporaryHoldEnum::TEMPORARYHOLDSKIP => "TEMPORARY_HOLD_SKIP",
            MetadataOptionTemporaryHoldEnum::TEMPORARYHOLDPRESERVE => "TEMPORARY_HOLD_PRESERVE",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionTemporaryHoldEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEMPORARY_HOLD_UNSPECIFIED" => Ok(MetadataOptionTemporaryHoldEnum::TEMPORARYHOLDUNSPECIFIED),
           "TEMPORARY_HOLD_SKIP" => Ok(MetadataOptionTemporaryHoldEnum::TEMPORARYHOLDSKIP),
           "TEMPORARY_HOLD_PRESERVE" => Ok(MetadataOptionTemporaryHoldEnum::TEMPORARYHOLDPRESERVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionTemporaryHoldEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionTimeCreatedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each object's `timeCreated` metadata is preserved for transfers. If unspecified, the default behavior is the same as TIME_CREATED_SKIP. This behavior is supported for transfers to GCS buckets from GCS, S3, Azure, S3 Compatible, and Azure sources.
pub enum MetadataOptionTimeCreatedEnum {
    

    /// TimeCreated behavior is unspecified.
    ///
    /// "TIME_CREATED_UNSPECIFIED"
    #[serde(rename="TIME_CREATED_UNSPECIFIED")]
    TIMECREATEDUNSPECIFIED,
    

    /// Do not preserve the `timeCreated` metadata from the source object.
    ///
    /// "TIME_CREATED_SKIP"
    #[serde(rename="TIME_CREATED_SKIP")]
    TIMECREATEDSKIP,
    

    /// Preserves the source object's `timeCreated` or `lastModified` metadata in the `customTime` field in the destination object. Note that any value stored in the source object's `customTime` field will not be propagated to the destination object.
    ///
    /// "TIME_CREATED_PRESERVE_AS_CUSTOM_TIME"
    #[serde(rename="TIME_CREATED_PRESERVE_AS_CUSTOM_TIME")]
    TIMECREATEDPRESERVEASCUSTOMTIME,
}

impl AsRef<str> for MetadataOptionTimeCreatedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionTimeCreatedEnum::TIMECREATEDUNSPECIFIED => "TIME_CREATED_UNSPECIFIED",
            MetadataOptionTimeCreatedEnum::TIMECREATEDSKIP => "TIME_CREATED_SKIP",
            MetadataOptionTimeCreatedEnum::TIMECREATEDPRESERVEASCUSTOMTIME => "TIME_CREATED_PRESERVE_AS_CUSTOM_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionTimeCreatedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_CREATED_UNSPECIFIED" => Ok(MetadataOptionTimeCreatedEnum::TIMECREATEDUNSPECIFIED),
           "TIME_CREATED_SKIP" => Ok(MetadataOptionTimeCreatedEnum::TIMECREATEDSKIP),
           "TIME_CREATED_PRESERVE_AS_CUSTOM_TIME" => Ok(MetadataOptionTimeCreatedEnum::TIMECREATEDPRESERVEASCUSTOMTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionTimeCreatedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetadataOptionUidEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how each file's POSIX user ID (UID) attribute should be handled by the transfer. By default, UID is not preserved. Only applicable to transfers involving POSIX file systems, and ignored for other transfers.
pub enum MetadataOptionUidEnum {
    

    /// UID behavior is unspecified.
    ///
    /// "UID_UNSPECIFIED"
    #[serde(rename="UID_UNSPECIFIED")]
    UIDUNSPECIFIED,
    

    /// Do not preserve UID during a transfer job.
    ///
    /// "UID_SKIP"
    #[serde(rename="UID_SKIP")]
    UIDSKIP,
    

    /// Preserve UID during a transfer job.
    ///
    /// "UID_NUMBER"
    #[serde(rename="UID_NUMBER")]
    UIDNUMBER,
}

impl AsRef<str> for MetadataOptionUidEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetadataOptionUidEnum::UIDUNSPECIFIED => "UID_UNSPECIFIED",
            MetadataOptionUidEnum::UIDSKIP => "UID_SKIP",
            MetadataOptionUidEnum::UIDNUMBER => "UID_NUMBER",
        }
    }
}

impl std::convert::TryFrom< &str> for MetadataOptionUidEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UID_UNSPECIFIED" => Ok(MetadataOptionUidEnum::UIDUNSPECIFIED),
           "UID_SKIP" => Ok(MetadataOptionUidEnum::UIDSKIP),
           "UID_NUMBER" => Ok(MetadataOptionUidEnum::UIDNUMBER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetadataOptionUidEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationConfigEventTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event types for which a notification is desired. If empty, send notifications for all event types.
pub enum NotificationConfigEventTypesEnum {
    

    /// Illegal value, to avoid allowing a default.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// `TransferOperation` completed with status SUCCESS.
    ///
    /// "TRANSFER_OPERATION_SUCCESS"
    #[serde(rename="TRANSFER_OPERATION_SUCCESS")]
    TRANSFEROPERATIONSUCCESS,
    

    /// `TransferOperation` completed with status FAILED.
    ///
    /// "TRANSFER_OPERATION_FAILED"
    #[serde(rename="TRANSFER_OPERATION_FAILED")]
    TRANSFEROPERATIONFAILED,
    

    /// `TransferOperation` completed with status ABORTED.
    ///
    /// "TRANSFER_OPERATION_ABORTED"
    #[serde(rename="TRANSFER_OPERATION_ABORTED")]
    TRANSFEROPERATIONABORTED,
}

impl AsRef<str> for NotificationConfigEventTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationConfigEventTypesEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            NotificationConfigEventTypesEnum::TRANSFEROPERATIONSUCCESS => "TRANSFER_OPERATION_SUCCESS",
            NotificationConfigEventTypesEnum::TRANSFEROPERATIONFAILED => "TRANSFER_OPERATION_FAILED",
            NotificationConfigEventTypesEnum::TRANSFEROPERATIONABORTED => "TRANSFER_OPERATION_ABORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationConfigEventTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(NotificationConfigEventTypesEnum::EVENTTYPEUNSPECIFIED),
           "TRANSFER_OPERATION_SUCCESS" => Ok(NotificationConfigEventTypesEnum::TRANSFEROPERATIONSUCCESS),
           "TRANSFER_OPERATION_FAILED" => Ok(NotificationConfigEventTypesEnum::TRANSFEROPERATIONFAILED),
           "TRANSFER_OPERATION_ABORTED" => Ok(NotificationConfigEventTypesEnum::TRANSFEROPERATIONABORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationConfigEventTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NotificationConfigPayloadFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The desired format of the notification message payloads.
pub enum NotificationConfigPayloadFormatEnum {
    

    /// Illegal value, to avoid allowing a default.
    ///
    /// "PAYLOAD_FORMAT_UNSPECIFIED"
    #[serde(rename="PAYLOAD_FORMAT_UNSPECIFIED")]
    PAYLOADFORMATUNSPECIFIED,
    

    /// No payload is included with the notification.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// `TransferOperation` is [formatted as a JSON response](https://developers.google.com/protocol-buffers/docs/proto3#json), in application/json.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for NotificationConfigPayloadFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NotificationConfigPayloadFormatEnum::PAYLOADFORMATUNSPECIFIED => "PAYLOAD_FORMAT_UNSPECIFIED",
            NotificationConfigPayloadFormatEnum::NONE => "NONE",
            NotificationConfigPayloadFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for NotificationConfigPayloadFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAYLOAD_FORMAT_UNSPECIFIED" => Ok(NotificationConfigPayloadFormatEnum::PAYLOADFORMATUNSPECIFIED),
           "NONE" => Ok(NotificationConfigPayloadFormatEnum::NONE),
           "JSON" => Ok(NotificationConfigPayloadFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NotificationConfigPayloadFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region S3CompatibleMetadataAuthMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the authentication and authorization method used by the storage service. When not specified, Transfer Service will attempt to determine right auth method to use.
pub enum S3CompatibleMetadataAuthMethodEnum {
    

    /// AuthMethod is not specified.
    ///
    /// "AUTH_METHOD_UNSPECIFIED"
    #[serde(rename="AUTH_METHOD_UNSPECIFIED")]
    AUTHMETHODUNSPECIFIED,
    

    /// Auth requests with AWS SigV4.
    ///
    /// "AUTH_METHOD_AWS_SIGNATURE_V4"
    #[serde(rename="AUTH_METHOD_AWS_SIGNATURE_V4")]
    AUTHMETHODAWSSIGNATUREV4,
    

    /// Auth requests with AWS SigV2.
    ///
    /// "AUTH_METHOD_AWS_SIGNATURE_V2"
    #[serde(rename="AUTH_METHOD_AWS_SIGNATURE_V2")]
    AUTHMETHODAWSSIGNATUREV2,
}

impl AsRef<str> for S3CompatibleMetadataAuthMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            S3CompatibleMetadataAuthMethodEnum::AUTHMETHODUNSPECIFIED => "AUTH_METHOD_UNSPECIFIED",
            S3CompatibleMetadataAuthMethodEnum::AUTHMETHODAWSSIGNATUREV4 => "AUTH_METHOD_AWS_SIGNATURE_V4",
            S3CompatibleMetadataAuthMethodEnum::AUTHMETHODAWSSIGNATUREV2 => "AUTH_METHOD_AWS_SIGNATURE_V2",
        }
    }
}

impl std::convert::TryFrom< &str> for S3CompatibleMetadataAuthMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTH_METHOD_UNSPECIFIED" => Ok(S3CompatibleMetadataAuthMethodEnum::AUTHMETHODUNSPECIFIED),
           "AUTH_METHOD_AWS_SIGNATURE_V4" => Ok(S3CompatibleMetadataAuthMethodEnum::AUTHMETHODAWSSIGNATUREV4),
           "AUTH_METHOD_AWS_SIGNATURE_V2" => Ok(S3CompatibleMetadataAuthMethodEnum::AUTHMETHODAWSSIGNATUREV2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a S3CompatibleMetadataAuthMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region S3CompatibleMetadataListApiEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Listing API to use for discovering objects. When not specified, Transfer Service will attempt to determine the right API to use.
pub enum S3CompatibleMetadataListApiEnum {
    

    /// ListApi is not specified.
    ///
    /// "LIST_API_UNSPECIFIED"
    #[serde(rename="LIST_API_UNSPECIFIED")]
    LISTAPIUNSPECIFIED,
    

    /// Perform listing using ListObjectsV2 API.
    ///
    /// "LIST_OBJECTS_V2"
    #[serde(rename="LIST_OBJECTS_V2")]
    LISTOBJECTSV2,
    

    /// Legacy ListObjects API.
    ///
    /// "LIST_OBJECTS"
    #[serde(rename="LIST_OBJECTS")]
    LISTOBJECTS,
}

impl AsRef<str> for S3CompatibleMetadataListApiEnum {
    fn as_ref(&self) -> &str {
        match *self {
            S3CompatibleMetadataListApiEnum::LISTAPIUNSPECIFIED => "LIST_API_UNSPECIFIED",
            S3CompatibleMetadataListApiEnum::LISTOBJECTSV2 => "LIST_OBJECTS_V2",
            S3CompatibleMetadataListApiEnum::LISTOBJECTS => "LIST_OBJECTS",
        }
    }
}

impl std::convert::TryFrom< &str> for S3CompatibleMetadataListApiEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIST_API_UNSPECIFIED" => Ok(S3CompatibleMetadataListApiEnum::LISTAPIUNSPECIFIED),
           "LIST_OBJECTS_V2" => Ok(S3CompatibleMetadataListApiEnum::LISTOBJECTSV2),
           "LIST_OBJECTS" => Ok(S3CompatibleMetadataListApiEnum::LISTOBJECTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a S3CompatibleMetadataListApiEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region S3CompatibleMetadataProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the network protocol of the agent. When not specified, the default value of NetworkProtocol NETWORK_PROTOCOL_HTTPS is used.
pub enum S3CompatibleMetadataProtocolEnum {
    

    /// NetworkProtocol is not specified.
    ///
    /// "NETWORK_PROTOCOL_UNSPECIFIED"
    #[serde(rename="NETWORK_PROTOCOL_UNSPECIFIED")]
    NETWORKPROTOCOLUNSPECIFIED,
    

    /// Perform requests using HTTPS.
    ///
    /// "NETWORK_PROTOCOL_HTTPS"
    #[serde(rename="NETWORK_PROTOCOL_HTTPS")]
    NETWORKPROTOCOLHTTPS,
    

    /// Not recommended: This sends data in clear-text. This is only appropriate within a closed network or for publicly available data. Perform requests using HTTP.
    ///
    /// "NETWORK_PROTOCOL_HTTP"
    #[serde(rename="NETWORK_PROTOCOL_HTTP")]
    NETWORKPROTOCOLHTTP,
}

impl AsRef<str> for S3CompatibleMetadataProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            S3CompatibleMetadataProtocolEnum::NETWORKPROTOCOLUNSPECIFIED => "NETWORK_PROTOCOL_UNSPECIFIED",
            S3CompatibleMetadataProtocolEnum::NETWORKPROTOCOLHTTPS => "NETWORK_PROTOCOL_HTTPS",
            S3CompatibleMetadataProtocolEnum::NETWORKPROTOCOLHTTP => "NETWORK_PROTOCOL_HTTP",
        }
    }
}

impl std::convert::TryFrom< &str> for S3CompatibleMetadataProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NETWORK_PROTOCOL_UNSPECIFIED" => Ok(S3CompatibleMetadataProtocolEnum::NETWORKPROTOCOLUNSPECIFIED),
           "NETWORK_PROTOCOL_HTTPS" => Ok(S3CompatibleMetadataProtocolEnum::NETWORKPROTOCOLHTTPS),
           "NETWORK_PROTOCOL_HTTP" => Ok(S3CompatibleMetadataProtocolEnum::NETWORKPROTOCOLHTTP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a S3CompatibleMetadataProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region S3CompatibleMetadataRequestModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the API request model used to call the storage service. When not specified, the default value of RequestModel REQUEST_MODEL_VIRTUAL_HOSTED_STYLE is used.
pub enum S3CompatibleMetadataRequestModelEnum {
    

    /// RequestModel is not specified.
    ///
    /// "REQUEST_MODEL_UNSPECIFIED"
    #[serde(rename="REQUEST_MODEL_UNSPECIFIED")]
    REQUESTMODELUNSPECIFIED,
    

    /// Perform requests using Virtual Hosted Style. Example: https://bucket-name.s3.region.amazonaws.com/key-name
    ///
    /// "REQUEST_MODEL_VIRTUAL_HOSTED_STYLE"
    #[serde(rename="REQUEST_MODEL_VIRTUAL_HOSTED_STYLE")]
    REQUESTMODELVIRTUALHOSTEDSTYLE,
    

    /// Perform requests using Path Style. Example: https://s3.region.amazonaws.com/bucket-name/key-name
    ///
    /// "REQUEST_MODEL_PATH_STYLE"
    #[serde(rename="REQUEST_MODEL_PATH_STYLE")]
    REQUESTMODELPATHSTYLE,
}

impl AsRef<str> for S3CompatibleMetadataRequestModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            S3CompatibleMetadataRequestModelEnum::REQUESTMODELUNSPECIFIED => "REQUEST_MODEL_UNSPECIFIED",
            S3CompatibleMetadataRequestModelEnum::REQUESTMODELVIRTUALHOSTEDSTYLE => "REQUEST_MODEL_VIRTUAL_HOSTED_STYLE",
            S3CompatibleMetadataRequestModelEnum::REQUESTMODELPATHSTYLE => "REQUEST_MODEL_PATH_STYLE",
        }
    }
}

impl std::convert::TryFrom< &str> for S3CompatibleMetadataRequestModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REQUEST_MODEL_UNSPECIFIED" => Ok(S3CompatibleMetadataRequestModelEnum::REQUESTMODELUNSPECIFIED),
           "REQUEST_MODEL_VIRTUAL_HOSTED_STYLE" => Ok(S3CompatibleMetadataRequestModelEnum::REQUESTMODELVIRTUALHOSTEDSTYLE),
           "REQUEST_MODEL_PATH_STYLE" => Ok(S3CompatibleMetadataRequestModelEnum::REQUESTMODELPATHSTYLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a S3CompatibleMetadataRequestModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferJobStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the job. This value MUST be specified for `CreateTransferJobRequests`. **Note:** The effect of the new job status takes place during a subsequent job run. For example, if you change the job status from ENABLED to DISABLED, and an operation spawned by the transfer is running, the status change would not affect the current operation.
pub enum TransferJobStatusEnum {
    

    /// Zero is an illegal value.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// New transfers are performed based on the schedule.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// New transfers are not scheduled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// This is a soft delete state. After a transfer job is set to this state, the job and all the transfer executions are subject to garbage collection. Transfer jobs become eligible for garbage collection 30 days after their status is set to `DELETED`.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for TransferJobStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferJobStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            TransferJobStatusEnum::ENABLED => "ENABLED",
            TransferJobStatusEnum::DISABLED => "DISABLED",
            TransferJobStatusEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferJobStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(TransferJobStatusEnum::STATUSUNSPECIFIED),
           "ENABLED" => Ok(TransferJobStatusEnum::ENABLED),
           "DISABLED" => Ok(TransferJobStatusEnum::DISABLED),
           "DELETED" => Ok(TransferJobStatusEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferJobStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferOperationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the transfer operation.
pub enum TransferOperationStatusEnum {
    

    /// Zero is an illegal value.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// In progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Paused.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Completed successfully.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// Terminated due to an unrecoverable failure.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Aborted by the user.
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
    

    /// Temporarily delayed by the system. No user action is required.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
    

    /// The operation is suspending and draining the ongoing work to completion.
    ///
    /// "SUSPENDING"
    #[serde(rename="SUSPENDING")]
    SUSPENDING,
}

impl AsRef<str> for TransferOperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferOperationStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            TransferOperationStatusEnum::INPROGRESS => "IN_PROGRESS",
            TransferOperationStatusEnum::PAUSED => "PAUSED",
            TransferOperationStatusEnum::SUCCESS => "SUCCESS",
            TransferOperationStatusEnum::FAILED => "FAILED",
            TransferOperationStatusEnum::ABORTED => "ABORTED",
            TransferOperationStatusEnum::QUEUED => "QUEUED",
            TransferOperationStatusEnum::SUSPENDING => "SUSPENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferOperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(TransferOperationStatusEnum::STATUSUNSPECIFIED),
           "IN_PROGRESS" => Ok(TransferOperationStatusEnum::INPROGRESS),
           "PAUSED" => Ok(TransferOperationStatusEnum::PAUSED),
           "SUCCESS" => Ok(TransferOperationStatusEnum::SUCCESS),
           "FAILED" => Ok(TransferOperationStatusEnum::FAILED),
           "ABORTED" => Ok(TransferOperationStatusEnum::ABORTED),
           "QUEUED" => Ok(TransferOperationStatusEnum::QUEUED),
           "SUSPENDING" => Ok(TransferOperationStatusEnum::SUSPENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferOperationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TransferOptionOverwriteWhenEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// When to overwrite objects that already exist in the sink. If not set, overwrite behavior is determined by overwrite_objects_already_existing_in_sink.
pub enum TransferOptionOverwriteWhenEnum {
    

    /// Overwrite behavior is unspecified.
    ///
    /// "OVERWRITE_WHEN_UNSPECIFIED"
    #[serde(rename="OVERWRITE_WHEN_UNSPECIFIED")]
    OVERWRITEWHENUNSPECIFIED,
    

    /// Overwrites destination objects with the source objects, only if the objects have the same name but different HTTP ETags or checksum values.
    ///
    /// "DIFFERENT"
    #[serde(rename="DIFFERENT")]
    DIFFERENT,
    

    /// Never overwrites a destination object if a source object has the same name. In this case, the source object is not transferred.
    ///
    /// "NEVER"
    #[serde(rename="NEVER")]
    NEVER,
    

    /// Always overwrite the destination object with the source object, even if the HTTP Etags or checksum values are the same.
    ///
    /// "ALWAYS"
    #[serde(rename="ALWAYS")]
    ALWAYS,
}

impl AsRef<str> for TransferOptionOverwriteWhenEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TransferOptionOverwriteWhenEnum::OVERWRITEWHENUNSPECIFIED => "OVERWRITE_WHEN_UNSPECIFIED",
            TransferOptionOverwriteWhenEnum::DIFFERENT => "DIFFERENT",
            TransferOptionOverwriteWhenEnum::NEVER => "NEVER",
            TransferOptionOverwriteWhenEnum::ALWAYS => "ALWAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for TransferOptionOverwriteWhenEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OVERWRITE_WHEN_UNSPECIFIED" => Ok(TransferOptionOverwriteWhenEnum::OVERWRITEWHENUNSPECIFIED),
           "DIFFERENT" => Ok(TransferOptionOverwriteWhenEnum::DIFFERENT),
           "NEVER" => Ok(TransferOptionOverwriteWhenEnum::NEVER),
           "ALWAYS" => Ok(TransferOptionOverwriteWhenEnum::ALWAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TransferOptionOverwriteWhenEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


