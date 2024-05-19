use super::*;



// region GoogleCloudRunV2ConditionExecutionReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A reason for the execution condition.
pub enum GoogleCloudRunV2ConditionExecutionReasonEnum {
    

    /// Default value.
    ///
    /// "EXECUTION_REASON_UNDEFINED"
    #[serde(rename="EXECUTION_REASON_UNDEFINED")]
    EXECUTIONREASONUNDEFINED,
    

    /// Internal system error getting execution status. System will retry.
    ///
    /// "JOB_STATUS_SERVICE_POLLING_ERROR"
    #[serde(rename="JOB_STATUS_SERVICE_POLLING_ERROR")]
    JOBSTATUSSERVICEPOLLINGERROR,
    

    /// A task reached its retry limit and the last attempt failed due to the user container exiting with a non-zero exit code.
    ///
    /// "NON_ZERO_EXIT_CODE"
    #[serde(rename="NON_ZERO_EXIT_CODE")]
    NONZEROEXITCODE,
    

    /// The execution was cancelled by users.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The execution is in the process of being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The execution was deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for GoogleCloudRunV2ConditionExecutionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ConditionExecutionReasonEnum::EXECUTIONREASONUNDEFINED => "EXECUTION_REASON_UNDEFINED",
            GoogleCloudRunV2ConditionExecutionReasonEnum::JOBSTATUSSERVICEPOLLINGERROR => "JOB_STATUS_SERVICE_POLLING_ERROR",
            GoogleCloudRunV2ConditionExecutionReasonEnum::NONZEROEXITCODE => "NON_ZERO_EXIT_CODE",
            GoogleCloudRunV2ConditionExecutionReasonEnum::CANCELLED => "CANCELLED",
            GoogleCloudRunV2ConditionExecutionReasonEnum::CANCELLING => "CANCELLING",
            GoogleCloudRunV2ConditionExecutionReasonEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ConditionExecutionReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_REASON_UNDEFINED" => Ok(GoogleCloudRunV2ConditionExecutionReasonEnum::EXECUTIONREASONUNDEFINED),
           "JOB_STATUS_SERVICE_POLLING_ERROR" => Ok(GoogleCloudRunV2ConditionExecutionReasonEnum::JOBSTATUSSERVICEPOLLINGERROR),
           "NON_ZERO_EXIT_CODE" => Ok(GoogleCloudRunV2ConditionExecutionReasonEnum::NONZEROEXITCODE),
           "CANCELLED" => Ok(GoogleCloudRunV2ConditionExecutionReasonEnum::CANCELLED),
           "CANCELLING" => Ok(GoogleCloudRunV2ConditionExecutionReasonEnum::CANCELLING),
           "DELETED" => Ok(GoogleCloudRunV2ConditionExecutionReasonEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ConditionExecutionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ConditionReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A common (service-level) reason for this condition.
pub enum GoogleCloudRunV2ConditionReasonEnum {
    

    /// Default value.
    ///
    /// "COMMON_REASON_UNDEFINED"
    #[serde(rename="COMMON_REASON_UNDEFINED")]
    COMMONREASONUNDEFINED,
    

    /// Reason unknown. Further details will be in message.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Revision creation process failed.
    ///
    /// "REVISION_FAILED"
    #[serde(rename="REVISION_FAILED")]
    REVISIONFAILED,
    

    /// Timed out waiting for completion.
    ///
    /// "PROGRESS_DEADLINE_EXCEEDED"
    #[serde(rename="PROGRESS_DEADLINE_EXCEEDED")]
    PROGRESSDEADLINEEXCEEDED,
    

    /// The container image path is incorrect.
    ///
    /// "CONTAINER_MISSING"
    #[serde(rename="CONTAINER_MISSING")]
    CONTAINERMISSING,
    

    /// Insufficient permissions on the container image.
    ///
    /// "CONTAINER_PERMISSION_DENIED"
    #[serde(rename="CONTAINER_PERMISSION_DENIED")]
    CONTAINERPERMISSIONDENIED,
    

    /// Container image is not authorized by policy.
    ///
    /// "CONTAINER_IMAGE_UNAUTHORIZED"
    #[serde(rename="CONTAINER_IMAGE_UNAUTHORIZED")]
    CONTAINERIMAGEUNAUTHORIZED,
    

    /// Container image policy authorization check failed.
    ///
    /// "CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED"
    #[serde(rename="CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED")]
    CONTAINERIMAGEAUTHORIZATIONCHECKFAILED,
    

    /// Insufficient permissions on encryption key.
    ///
    /// "ENCRYPTION_KEY_PERMISSION_DENIED"
    #[serde(rename="ENCRYPTION_KEY_PERMISSION_DENIED")]
    ENCRYPTIONKEYPERMISSIONDENIED,
    

    /// Permission check on encryption key failed.
    ///
    /// "ENCRYPTION_KEY_CHECK_FAILED"
    #[serde(rename="ENCRYPTION_KEY_CHECK_FAILED")]
    ENCRYPTIONKEYCHECKFAILED,
    

    /// At least one Access check on secrets failed.
    ///
    /// "SECRETS_ACCESS_CHECK_FAILED"
    #[serde(rename="SECRETS_ACCESS_CHECK_FAILED")]
    SECRETSACCESSCHECKFAILED,
    

    /// Waiting for operation to complete.
    ///
    /// "WAITING_FOR_OPERATION"
    #[serde(rename="WAITING_FOR_OPERATION")]
    WAITINGFOROPERATION,
    

    /// System will retry immediately.
    ///
    /// "IMMEDIATE_RETRY"
    #[serde(rename="IMMEDIATE_RETRY")]
    IMMEDIATERETRY,
    

    /// System will retry later; current attempt failed.
    ///
    /// "POSTPONED_RETRY"
    #[serde(rename="POSTPONED_RETRY")]
    POSTPONEDRETRY,
    

    /// An internal error occurred. Further information may be in the message.
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
}

impl AsRef<str> for GoogleCloudRunV2ConditionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ConditionReasonEnum::COMMONREASONUNDEFINED => "COMMON_REASON_UNDEFINED",
            GoogleCloudRunV2ConditionReasonEnum::UNKNOWN => "UNKNOWN",
            GoogleCloudRunV2ConditionReasonEnum::REVISIONFAILED => "REVISION_FAILED",
            GoogleCloudRunV2ConditionReasonEnum::PROGRESSDEADLINEEXCEEDED => "PROGRESS_DEADLINE_EXCEEDED",
            GoogleCloudRunV2ConditionReasonEnum::CONTAINERMISSING => "CONTAINER_MISSING",
            GoogleCloudRunV2ConditionReasonEnum::CONTAINERPERMISSIONDENIED => "CONTAINER_PERMISSION_DENIED",
            GoogleCloudRunV2ConditionReasonEnum::CONTAINERIMAGEUNAUTHORIZED => "CONTAINER_IMAGE_UNAUTHORIZED",
            GoogleCloudRunV2ConditionReasonEnum::CONTAINERIMAGEAUTHORIZATIONCHECKFAILED => "CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED",
            GoogleCloudRunV2ConditionReasonEnum::ENCRYPTIONKEYPERMISSIONDENIED => "ENCRYPTION_KEY_PERMISSION_DENIED",
            GoogleCloudRunV2ConditionReasonEnum::ENCRYPTIONKEYCHECKFAILED => "ENCRYPTION_KEY_CHECK_FAILED",
            GoogleCloudRunV2ConditionReasonEnum::SECRETSACCESSCHECKFAILED => "SECRETS_ACCESS_CHECK_FAILED",
            GoogleCloudRunV2ConditionReasonEnum::WAITINGFOROPERATION => "WAITING_FOR_OPERATION",
            GoogleCloudRunV2ConditionReasonEnum::IMMEDIATERETRY => "IMMEDIATE_RETRY",
            GoogleCloudRunV2ConditionReasonEnum::POSTPONEDRETRY => "POSTPONED_RETRY",
            GoogleCloudRunV2ConditionReasonEnum::INTERNAL => "INTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ConditionReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMON_REASON_UNDEFINED" => Ok(GoogleCloudRunV2ConditionReasonEnum::COMMONREASONUNDEFINED),
           "UNKNOWN" => Ok(GoogleCloudRunV2ConditionReasonEnum::UNKNOWN),
           "REVISION_FAILED" => Ok(GoogleCloudRunV2ConditionReasonEnum::REVISIONFAILED),
           "PROGRESS_DEADLINE_EXCEEDED" => Ok(GoogleCloudRunV2ConditionReasonEnum::PROGRESSDEADLINEEXCEEDED),
           "CONTAINER_MISSING" => Ok(GoogleCloudRunV2ConditionReasonEnum::CONTAINERMISSING),
           "CONTAINER_PERMISSION_DENIED" => Ok(GoogleCloudRunV2ConditionReasonEnum::CONTAINERPERMISSIONDENIED),
           "CONTAINER_IMAGE_UNAUTHORIZED" => Ok(GoogleCloudRunV2ConditionReasonEnum::CONTAINERIMAGEUNAUTHORIZED),
           "CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED" => Ok(GoogleCloudRunV2ConditionReasonEnum::CONTAINERIMAGEAUTHORIZATIONCHECKFAILED),
           "ENCRYPTION_KEY_PERMISSION_DENIED" => Ok(GoogleCloudRunV2ConditionReasonEnum::ENCRYPTIONKEYPERMISSIONDENIED),
           "ENCRYPTION_KEY_CHECK_FAILED" => Ok(GoogleCloudRunV2ConditionReasonEnum::ENCRYPTIONKEYCHECKFAILED),
           "SECRETS_ACCESS_CHECK_FAILED" => Ok(GoogleCloudRunV2ConditionReasonEnum::SECRETSACCESSCHECKFAILED),
           "WAITING_FOR_OPERATION" => Ok(GoogleCloudRunV2ConditionReasonEnum::WAITINGFOROPERATION),
           "IMMEDIATE_RETRY" => Ok(GoogleCloudRunV2ConditionReasonEnum::IMMEDIATERETRY),
           "POSTPONED_RETRY" => Ok(GoogleCloudRunV2ConditionReasonEnum::POSTPONEDRETRY),
           "INTERNAL" => Ok(GoogleCloudRunV2ConditionReasonEnum::INTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ConditionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ConditionRevisionReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A reason for the revision condition.
pub enum GoogleCloudRunV2ConditionRevisionReasonEnum {
    

    /// Default value.
    ///
    /// "REVISION_REASON_UNDEFINED"
    #[serde(rename="REVISION_REASON_UNDEFINED")]
    REVISIONREASONUNDEFINED,
    

    /// Revision in Pending state.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Revision is in Reserve state.
    ///
    /// "RESERVE"
    #[serde(rename="RESERVE")]
    RESERVE,
    

    /// Revision is Retired.
    ///
    /// "RETIRED"
    #[serde(rename="RETIRED")]
    RETIRED,
    

    /// Revision is being retired.
    ///
    /// "RETIRING"
    #[serde(rename="RETIRING")]
    RETIRING,
    

    /// Revision is being recreated.
    ///
    /// "RECREATING"
    #[serde(rename="RECREATING")]
    RECREATING,
    

    /// There was a health check error.
    ///
    /// "HEALTH_CHECK_CONTAINER_ERROR"
    #[serde(rename="HEALTH_CHECK_CONTAINER_ERROR")]
    HEALTHCHECKCONTAINERERROR,
    

    /// Health check failed due to user error from customized path of the container. System will retry.
    ///
    /// "CUSTOMIZED_PATH_RESPONSE_PENDING"
    #[serde(rename="CUSTOMIZED_PATH_RESPONSE_PENDING")]
    CUSTOMIZEDPATHRESPONSEPENDING,
    

    /// A revision with min_instance_count > 0 was created and is reserved, but it was not configured to serve traffic, so it's not live. This can also happen momentarily during traffic migration.
    ///
    /// "MIN_INSTANCES_NOT_PROVISIONED"
    #[serde(rename="MIN_INSTANCES_NOT_PROVISIONED")]
    MININSTANCESNOTPROVISIONED,
    

    /// The maximum allowed number of active revisions has been reached.
    ///
    /// "ACTIVE_REVISION_LIMIT_REACHED"
    #[serde(rename="ACTIVE_REVISION_LIMIT_REACHED")]
    ACTIVEREVISIONLIMITREACHED,
    

    /// There was no deployment defined. This value is no longer used, but Services created in older versions of the API might contain this value.
    ///
    /// "NO_DEPLOYMENT"
    #[serde(rename="NO_DEPLOYMENT")]
    NODEPLOYMENT,
    

    /// A revision's container has no port specified since the revision is of a manually scaled service with 0 instance count
    ///
    /// "HEALTH_CHECK_SKIPPED"
    #[serde(rename="HEALTH_CHECK_SKIPPED")]
    HEALTHCHECKSKIPPED,
    

    /// A revision with min_instance_count > 0 was created and is waiting for enough instances to begin a traffic migration.
    ///
    /// "MIN_INSTANCES_WARMING"
    #[serde(rename="MIN_INSTANCES_WARMING")]
    MININSTANCESWARMING,
}

impl AsRef<str> for GoogleCloudRunV2ConditionRevisionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ConditionRevisionReasonEnum::REVISIONREASONUNDEFINED => "REVISION_REASON_UNDEFINED",
            GoogleCloudRunV2ConditionRevisionReasonEnum::PENDING => "PENDING",
            GoogleCloudRunV2ConditionRevisionReasonEnum::RESERVE => "RESERVE",
            GoogleCloudRunV2ConditionRevisionReasonEnum::RETIRED => "RETIRED",
            GoogleCloudRunV2ConditionRevisionReasonEnum::RETIRING => "RETIRING",
            GoogleCloudRunV2ConditionRevisionReasonEnum::RECREATING => "RECREATING",
            GoogleCloudRunV2ConditionRevisionReasonEnum::HEALTHCHECKCONTAINERERROR => "HEALTH_CHECK_CONTAINER_ERROR",
            GoogleCloudRunV2ConditionRevisionReasonEnum::CUSTOMIZEDPATHRESPONSEPENDING => "CUSTOMIZED_PATH_RESPONSE_PENDING",
            GoogleCloudRunV2ConditionRevisionReasonEnum::MININSTANCESNOTPROVISIONED => "MIN_INSTANCES_NOT_PROVISIONED",
            GoogleCloudRunV2ConditionRevisionReasonEnum::ACTIVEREVISIONLIMITREACHED => "ACTIVE_REVISION_LIMIT_REACHED",
            GoogleCloudRunV2ConditionRevisionReasonEnum::NODEPLOYMENT => "NO_DEPLOYMENT",
            GoogleCloudRunV2ConditionRevisionReasonEnum::HEALTHCHECKSKIPPED => "HEALTH_CHECK_SKIPPED",
            GoogleCloudRunV2ConditionRevisionReasonEnum::MININSTANCESWARMING => "MIN_INSTANCES_WARMING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ConditionRevisionReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVISION_REASON_UNDEFINED" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::REVISIONREASONUNDEFINED),
           "PENDING" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::PENDING),
           "RESERVE" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::RESERVE),
           "RETIRED" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::RETIRED),
           "RETIRING" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::RETIRING),
           "RECREATING" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::RECREATING),
           "HEALTH_CHECK_CONTAINER_ERROR" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::HEALTHCHECKCONTAINERERROR),
           "CUSTOMIZED_PATH_RESPONSE_PENDING" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::CUSTOMIZEDPATHRESPONSEPENDING),
           "MIN_INSTANCES_NOT_PROVISIONED" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::MININSTANCESNOTPROVISIONED),
           "ACTIVE_REVISION_LIMIT_REACHED" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::ACTIVEREVISIONLIMITREACHED),
           "NO_DEPLOYMENT" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::NODEPLOYMENT),
           "HEALTH_CHECK_SKIPPED" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::HEALTHCHECKSKIPPED),
           "MIN_INSTANCES_WARMING" => Ok(GoogleCloudRunV2ConditionRevisionReasonEnum::MININSTANCESWARMING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ConditionRevisionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ConditionSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How to interpret failures of this condition, one of Error, Warning, Info
pub enum GoogleCloudRunV2ConditionSeverityEnum {
    

    /// Unspecified severity
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Error severity.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Warning severity.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Info severity.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
}

impl AsRef<str> for GoogleCloudRunV2ConditionSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ConditionSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudRunV2ConditionSeverityEnum::ERROR => "ERROR",
            GoogleCloudRunV2ConditionSeverityEnum::WARNING => "WARNING",
            GoogleCloudRunV2ConditionSeverityEnum::INFO => "INFO",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ConditionSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudRunV2ConditionSeverityEnum::SEVERITYUNSPECIFIED),
           "ERROR" => Ok(GoogleCloudRunV2ConditionSeverityEnum::ERROR),
           "WARNING" => Ok(GoogleCloudRunV2ConditionSeverityEnum::WARNING),
           "INFO" => Ok(GoogleCloudRunV2ConditionSeverityEnum::INFO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ConditionSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ConditionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the condition.
pub enum GoogleCloudRunV2ConditionStateEnum {
    

    /// The default value. This value is used if the state is omitted.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Transient state: Reconciliation has not started yet.
    ///
    /// "CONDITION_PENDING"
    #[serde(rename="CONDITION_PENDING")]
    CONDITIONPENDING,
    

    /// Transient state: reconciliation is still in progress.
    ///
    /// "CONDITION_RECONCILING"
    #[serde(rename="CONDITION_RECONCILING")]
    CONDITIONRECONCILING,
    

    /// Terminal state: Reconciliation did not succeed.
    ///
    /// "CONDITION_FAILED"
    #[serde(rename="CONDITION_FAILED")]
    CONDITIONFAILED,
    

    /// Terminal state: Reconciliation completed successfully.
    ///
    /// "CONDITION_SUCCEEDED"
    #[serde(rename="CONDITION_SUCCEEDED")]
    CONDITIONSUCCEEDED,
}

impl AsRef<str> for GoogleCloudRunV2ConditionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ConditionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudRunV2ConditionStateEnum::CONDITIONPENDING => "CONDITION_PENDING",
            GoogleCloudRunV2ConditionStateEnum::CONDITIONRECONCILING => "CONDITION_RECONCILING",
            GoogleCloudRunV2ConditionStateEnum::CONDITIONFAILED => "CONDITION_FAILED",
            GoogleCloudRunV2ConditionStateEnum::CONDITIONSUCCEEDED => "CONDITION_SUCCEEDED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ConditionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudRunV2ConditionStateEnum::STATEUNSPECIFIED),
           "CONDITION_PENDING" => Ok(GoogleCloudRunV2ConditionStateEnum::CONDITIONPENDING),
           "CONDITION_RECONCILING" => Ok(GoogleCloudRunV2ConditionStateEnum::CONDITIONRECONCILING),
           "CONDITION_FAILED" => Ok(GoogleCloudRunV2ConditionStateEnum::CONDITIONFAILED),
           "CONDITION_SUCCEEDED" => Ok(GoogleCloudRunV2ConditionStateEnum::CONDITIONSUCCEEDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ConditionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The medium on which the data is stored. Acceptable values today is only MEMORY or none. When none, the default will currently be backed by memory but could change over time. +optional
pub enum GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum {
    

    /// When not specified, falls back to the default implementation which is currently in memory (this may change over time).
    ///
    /// "MEDIUM_UNSPECIFIED"
    #[serde(rename="MEDIUM_UNSPECIFIED")]
    MEDIUMUNSPECIFIED,
    

    /// Explicitly set the EmptyDir to be in memory. Uses tmpfs.
    ///
    /// "MEMORY"
    #[serde(rename="MEMORY")]
    MEMORY,
}

impl AsRef<str> for GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum::MEDIUMUNSPECIFIED => "MEDIUM_UNSPECIFIED",
            GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum::MEMORY => "MEMORY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIUM_UNSPECIFIED" => Ok(GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum::MEDIUMUNSPECIFIED),
           "MEMORY" => Ok(GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum::MEMORY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2EmptyDirVolumeSourceMediumEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ExecutionLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The least stable launch stage needed to create this resource, as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. Note that this value might not be what was used as input. For example, if ALPHA was provided as input in the parent resource, but only BETA and GA-level features are were, this field will be BETA.
pub enum GoogleCloudRunV2ExecutionLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for GoogleCloudRunV2ExecutionLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ExecutionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            GoogleCloudRunV2ExecutionLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            GoogleCloudRunV2ExecutionLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            GoogleCloudRunV2ExecutionLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            GoogleCloudRunV2ExecutionLaunchStageEnum::ALPHA => "ALPHA",
            GoogleCloudRunV2ExecutionLaunchStageEnum::BETA => "BETA",
            GoogleCloudRunV2ExecutionLaunchStageEnum::GA => "GA",
            GoogleCloudRunV2ExecutionLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ExecutionLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::ALPHA),
           "BETA" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::BETA),
           "GA" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::GA),
           "DEPRECATED" => Ok(GoogleCloudRunV2ExecutionLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ExecutionLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ExportStatusResponseOperationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the overall export operation.
pub enum GoogleCloudRunV2ExportStatusResponseOperationStateEnum {
    

    /// State unspecified.
    ///
    /// "OPERATION_STATE_UNSPECIFIED"
    #[serde(rename="OPERATION_STATE_UNSPECIFIED")]
    OPERATIONSTATEUNSPECIFIED,
    

    /// Operation still in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Operation finished.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
}

impl AsRef<str> for GoogleCloudRunV2ExportStatusResponseOperationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ExportStatusResponseOperationStateEnum::OPERATIONSTATEUNSPECIFIED => "OPERATION_STATE_UNSPECIFIED",
            GoogleCloudRunV2ExportStatusResponseOperationStateEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudRunV2ExportStatusResponseOperationStateEnum::FINISHED => "FINISHED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ExportStatusResponseOperationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATION_STATE_UNSPECIFIED" => Ok(GoogleCloudRunV2ExportStatusResponseOperationStateEnum::OPERATIONSTATEUNSPECIFIED),
           "IN_PROGRESS" => Ok(GoogleCloudRunV2ExportStatusResponseOperationStateEnum::INPROGRESS),
           "FINISHED" => Ok(GoogleCloudRunV2ExportStatusResponseOperationStateEnum::FINISHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ExportStatusResponseOperationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ImageExportStatusExportJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Has the image export job finished (regardless of successful or failure).
pub enum GoogleCloudRunV2ImageExportStatusExportJobStateEnum {
    

    /// State unspecified.
    ///
    /// "EXPORT_JOB_STATE_UNSPECIFIED"
    #[serde(rename="EXPORT_JOB_STATE_UNSPECIFIED")]
    EXPORTJOBSTATEUNSPECIFIED,
    

    /// Job still in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Job finished.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
}

impl AsRef<str> for GoogleCloudRunV2ImageExportStatusExportJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ImageExportStatusExportJobStateEnum::EXPORTJOBSTATEUNSPECIFIED => "EXPORT_JOB_STATE_UNSPECIFIED",
            GoogleCloudRunV2ImageExportStatusExportJobStateEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudRunV2ImageExportStatusExportJobStateEnum::FINISHED => "FINISHED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ImageExportStatusExportJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXPORT_JOB_STATE_UNSPECIFIED" => Ok(GoogleCloudRunV2ImageExportStatusExportJobStateEnum::EXPORTJOBSTATEUNSPECIFIED),
           "IN_PROGRESS" => Ok(GoogleCloudRunV2ImageExportStatusExportJobStateEnum::INPROGRESS),
           "FINISHED" => Ok(GoogleCloudRunV2ImageExportStatusExportJobStateEnum::FINISHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ImageExportStatusExportJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2JobLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output.
pub enum GoogleCloudRunV2JobLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for GoogleCloudRunV2JobLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2JobLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            GoogleCloudRunV2JobLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            GoogleCloudRunV2JobLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            GoogleCloudRunV2JobLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            GoogleCloudRunV2JobLaunchStageEnum::ALPHA => "ALPHA",
            GoogleCloudRunV2JobLaunchStageEnum::BETA => "BETA",
            GoogleCloudRunV2JobLaunchStageEnum::GA => "GA",
            GoogleCloudRunV2JobLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2JobLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(GoogleCloudRunV2JobLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(GoogleCloudRunV2JobLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(GoogleCloudRunV2JobLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(GoogleCloudRunV2JobLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(GoogleCloudRunV2JobLaunchStageEnum::ALPHA),
           "BETA" => Ok(GoogleCloudRunV2JobLaunchStageEnum::BETA),
           "GA" => Ok(GoogleCloudRunV2JobLaunchStageEnum::GA),
           "DEPRECATED" => Ok(GoogleCloudRunV2JobLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2JobLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The action to take if the encryption key is revoked.
pub enum GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum {
    

    /// Unspecified
    ///
    /// "ENCRYPTION_KEY_REVOCATION_ACTION_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_KEY_REVOCATION_ACTION_UNSPECIFIED")]
    ENCRYPTIONKEYREVOCATIONACTIONUNSPECIFIED,
    

    /// Prevents the creation of new instances.
    ///
    /// "PREVENT_NEW"
    #[serde(rename="PREVENT_NEW")]
    PREVENTNEW,
    

    /// Shuts down existing instances, and prevents creation of new ones.
    ///
    /// "SHUTDOWN"
    #[serde(rename="SHUTDOWN")]
    SHUTDOWN,
}

impl AsRef<str> for GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum::ENCRYPTIONKEYREVOCATIONACTIONUNSPECIFIED => "ENCRYPTION_KEY_REVOCATION_ACTION_UNSPECIFIED",
            GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum::PREVENTNEW => "PREVENT_NEW",
            GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum::SHUTDOWN => "SHUTDOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_KEY_REVOCATION_ACTION_UNSPECIFIED" => Ok(GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum::ENCRYPTIONKEYREVOCATIONACTIONUNSPECIFIED),
           "PREVENT_NEW" => Ok(GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum::PREVENTNEW),
           "SHUTDOWN" => Ok(GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum::SHUTDOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2RevisionEncryptionKeyRevocationActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2RevisionExecutionEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The execution environment being used to host this Revision.
pub enum GoogleCloudRunV2RevisionExecutionEnvironmentEnum {
    

    /// Unspecified
    ///
    /// "EXECUTION_ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="EXECUTION_ENVIRONMENT_UNSPECIFIED")]
    EXECUTIONENVIRONMENTUNSPECIFIED,
    

    /// Uses the First Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN1"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN1")]
    EXECUTIONENVIRONMENTGEN1,
    

    /// Uses Second Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN2"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN2")]
    EXECUTIONENVIRONMENTGEN2,
}

impl AsRef<str> for GoogleCloudRunV2RevisionExecutionEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2RevisionExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED => "EXECUTION_ENVIRONMENT_UNSPECIFIED",
            GoogleCloudRunV2RevisionExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1 => "EXECUTION_ENVIRONMENT_GEN1",
            GoogleCloudRunV2RevisionExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2 => "EXECUTION_ENVIRONMENT_GEN2",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2RevisionExecutionEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_ENVIRONMENT_UNSPECIFIED" => Ok(GoogleCloudRunV2RevisionExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED),
           "EXECUTION_ENVIRONMENT_GEN1" => Ok(GoogleCloudRunV2RevisionExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1),
           "EXECUTION_ENVIRONMENT_GEN2" => Ok(GoogleCloudRunV2RevisionExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2RevisionExecutionEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2RevisionLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The least stable launch stage needed to create this resource, as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. Note that this value might not be what was used as input. For example, if ALPHA was provided as input in the parent resource, but only BETA and GA-level features are were, this field will be BETA.
pub enum GoogleCloudRunV2RevisionLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for GoogleCloudRunV2RevisionLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2RevisionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            GoogleCloudRunV2RevisionLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            GoogleCloudRunV2RevisionLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            GoogleCloudRunV2RevisionLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            GoogleCloudRunV2RevisionLaunchStageEnum::ALPHA => "ALPHA",
            GoogleCloudRunV2RevisionLaunchStageEnum::BETA => "BETA",
            GoogleCloudRunV2RevisionLaunchStageEnum::GA => "GA",
            GoogleCloudRunV2RevisionLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2RevisionLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::ALPHA),
           "BETA" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::BETA),
           "GA" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::GA),
           "DEPRECATED" => Ok(GoogleCloudRunV2RevisionLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2RevisionLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The sandbox environment to host this Revision.
pub enum GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum {
    

    /// Unspecified
    ///
    /// "EXECUTION_ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="EXECUTION_ENVIRONMENT_UNSPECIFIED")]
    EXECUTIONENVIRONMENTUNSPECIFIED,
    

    /// Uses the First Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN1"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN1")]
    EXECUTIONENVIRONMENTGEN1,
    

    /// Uses Second Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN2"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN2")]
    EXECUTIONENVIRONMENTGEN2,
}

impl AsRef<str> for GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED => "EXECUTION_ENVIRONMENT_UNSPECIFIED",
            GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1 => "EXECUTION_ENVIRONMENT_GEN1",
            GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2 => "EXECUTION_ENVIRONMENT_GEN2",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_ENVIRONMENT_UNSPECIFIED" => Ok(GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED),
           "EXECUTION_ENVIRONMENT_GEN1" => Ok(GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1),
           "EXECUTION_ENVIRONMENT_GEN2" => Ok(GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2RevisionTemplateExecutionEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ServiceIngressEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Provides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active.
pub enum GoogleCloudRunV2ServiceIngressEnum {
    

    /// Unspecified
    ///
    /// "INGRESS_TRAFFIC_UNSPECIFIED"
    #[serde(rename="INGRESS_TRAFFIC_UNSPECIFIED")]
    INGRESSTRAFFICUNSPECIFIED,
    

    /// All inbound traffic is allowed.
    ///
    /// "INGRESS_TRAFFIC_ALL"
    #[serde(rename="INGRESS_TRAFFIC_ALL")]
    INGRESSTRAFFICALL,
    

    /// Only internal traffic is allowed.
    ///
    /// "INGRESS_TRAFFIC_INTERNAL_ONLY"
    #[serde(rename="INGRESS_TRAFFIC_INTERNAL_ONLY")]
    INGRESSTRAFFICINTERNALONLY,
    

    /// Both internal and Google Cloud Load Balancer traffic is allowed.
    ///
    /// "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER"
    #[serde(rename="INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER")]
    INGRESSTRAFFICINTERNALLOADBALANCER,
    

    /// No ingress traffic is allowed.
    ///
    /// "INGRESS_TRAFFIC_NONE"
    #[serde(rename="INGRESS_TRAFFIC_NONE")]
    INGRESSTRAFFICNONE,
}

impl AsRef<str> for GoogleCloudRunV2ServiceIngressEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICUNSPECIFIED => "INGRESS_TRAFFIC_UNSPECIFIED",
            GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICALL => "INGRESS_TRAFFIC_ALL",
            GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICINTERNALONLY => "INGRESS_TRAFFIC_INTERNAL_ONLY",
            GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICINTERNALLOADBALANCER => "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER",
            GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICNONE => "INGRESS_TRAFFIC_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ServiceIngressEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INGRESS_TRAFFIC_UNSPECIFIED" => Ok(GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICUNSPECIFIED),
           "INGRESS_TRAFFIC_ALL" => Ok(GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICALL),
           "INGRESS_TRAFFIC_INTERNAL_ONLY" => Ok(GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICINTERNALONLY),
           "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER" => Ok(GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICINTERNALLOADBALANCER),
           "INGRESS_TRAFFIC_NONE" => Ok(GoogleCloudRunV2ServiceIngressEnum::INGRESSTRAFFICNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ServiceIngressEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2ServiceLaunchStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output.
pub enum GoogleCloudRunV2ServiceLaunchStageEnum {
    

    /// Do not use this default value.
    ///
    /// "LAUNCH_STAGE_UNSPECIFIED"
    #[serde(rename="LAUNCH_STAGE_UNSPECIFIED")]
    LAUNCHSTAGEUNSPECIFIED,
    

    /// The feature is not yet implemented. Users can not use it.
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Prelaunch features are hidden from users and are only visible internally.
    ///
    /// "PRELAUNCH"
    #[serde(rename="PRELAUNCH")]
    PRELAUNCH,
    

    /// Early Access features are limited to a closed group of testers. To use these features, you must sign up in advance and sign a Trusted Tester agreement (which includes confidentiality provisions). These features may be unstable, changed in backward-incompatible ways, and are not guaranteed to be released.
    ///
    /// "EARLY_ACCESS"
    #[serde(rename="EARLY_ACCESS")]
    EARLYACCESS,
    

    /// Alpha is a limited availability test for releases before they are cleared for widespread use. By Alpha, all significant design issues are resolved and we are in the process of verifying functionality. Alpha customers need to apply for access, agree to applicable terms, and have their projects allowlisted. Alpha releases don't have to be feature complete, no SLAs are provided, and there are no technical support obligations, but they will be far enough along that customers can actually use them in test environments or for limited-use tests -- just like they would in normal production cases.
    ///
    /// "ALPHA"
    #[serde(rename="ALPHA")]
    ALPHA,
    

    /// Beta is the point at which we are ready to open a release for any customer to use. There are no SLA or technical support obligations in a Beta release. Products will be complete from a feature perspective, but may have some open outstanding issues. Beta releases are suitable for limited production use cases.
    ///
    /// "BETA"
    #[serde(rename="BETA")]
    BETA,
    

    /// GA features are open to all developers and are considered stable and fully qualified for production use.
    ///
    /// "GA"
    #[serde(rename="GA")]
    GA,
    

    /// Deprecated features are scheduled to be shut down and removed. For more information, see the "Deprecation Policy" section of our [Terms of Service](https://cloud.google.com/terms/) and the [Google Cloud Platform Subject to the Deprecation Policy](https://cloud.google.com/terms/deprecation) documentation.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
}

impl AsRef<str> for GoogleCloudRunV2ServiceLaunchStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2ServiceLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED => "LAUNCH_STAGE_UNSPECIFIED",
            GoogleCloudRunV2ServiceLaunchStageEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            GoogleCloudRunV2ServiceLaunchStageEnum::PRELAUNCH => "PRELAUNCH",
            GoogleCloudRunV2ServiceLaunchStageEnum::EARLYACCESS => "EARLY_ACCESS",
            GoogleCloudRunV2ServiceLaunchStageEnum::ALPHA => "ALPHA",
            GoogleCloudRunV2ServiceLaunchStageEnum::BETA => "BETA",
            GoogleCloudRunV2ServiceLaunchStageEnum::GA => "GA",
            GoogleCloudRunV2ServiceLaunchStageEnum::DEPRECATED => "DEPRECATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2ServiceLaunchStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LAUNCH_STAGE_UNSPECIFIED" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::LAUNCHSTAGEUNSPECIFIED),
           "UNIMPLEMENTED" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::UNIMPLEMENTED),
           "PRELAUNCH" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::PRELAUNCH),
           "EARLY_ACCESS" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::EARLYACCESS),
           "ALPHA" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::ALPHA),
           "BETA" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::BETA),
           "GA" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::GA),
           "DEPRECATED" => Ok(GoogleCloudRunV2ServiceLaunchStageEnum::DEPRECATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2ServiceLaunchStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2TaskExecutionEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The execution environment being used to host this Task.
pub enum GoogleCloudRunV2TaskExecutionEnvironmentEnum {
    

    /// Unspecified
    ///
    /// "EXECUTION_ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="EXECUTION_ENVIRONMENT_UNSPECIFIED")]
    EXECUTIONENVIRONMENTUNSPECIFIED,
    

    /// Uses the First Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN1"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN1")]
    EXECUTIONENVIRONMENTGEN1,
    

    /// Uses Second Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN2"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN2")]
    EXECUTIONENVIRONMENTGEN2,
}

impl AsRef<str> for GoogleCloudRunV2TaskExecutionEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2TaskExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED => "EXECUTION_ENVIRONMENT_UNSPECIFIED",
            GoogleCloudRunV2TaskExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1 => "EXECUTION_ENVIRONMENT_GEN1",
            GoogleCloudRunV2TaskExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2 => "EXECUTION_ENVIRONMENT_GEN2",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2TaskExecutionEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_ENVIRONMENT_UNSPECIFIED" => Ok(GoogleCloudRunV2TaskExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED),
           "EXECUTION_ENVIRONMENT_GEN1" => Ok(GoogleCloudRunV2TaskExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1),
           "EXECUTION_ENVIRONMENT_GEN2" => Ok(GoogleCloudRunV2TaskExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2TaskExecutionEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The execution environment being used to host this Task.
pub enum GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum {
    

    /// Unspecified
    ///
    /// "EXECUTION_ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="EXECUTION_ENVIRONMENT_UNSPECIFIED")]
    EXECUTIONENVIRONMENTUNSPECIFIED,
    

    /// Uses the First Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN1"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN1")]
    EXECUTIONENVIRONMENTGEN1,
    

    /// Uses Second Generation environment.
    ///
    /// "EXECUTION_ENVIRONMENT_GEN2"
    #[serde(rename="EXECUTION_ENVIRONMENT_GEN2")]
    EXECUTIONENVIRONMENTGEN2,
}

impl AsRef<str> for GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED => "EXECUTION_ENVIRONMENT_UNSPECIFIED",
            GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1 => "EXECUTION_ENVIRONMENT_GEN1",
            GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2 => "EXECUTION_ENVIRONMENT_GEN2",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_ENVIRONMENT_UNSPECIFIED" => Ok(GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTUNSPECIFIED),
           "EXECUTION_ENVIRONMENT_GEN1" => Ok(GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN1),
           "EXECUTION_ENVIRONMENT_GEN2" => Ok(GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum::EXECUTIONENVIRONMENTGEN2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2TaskTemplateExecutionEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2TrafficTargetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The allocation type for this traffic target.
pub enum GoogleCloudRunV2TrafficTargetTypeEnum {
    

    /// Unspecified instance allocation type.
    ///
    /// "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED")]
    TRAFFICTARGETALLOCATIONTYPEUNSPECIFIED,
    

    /// Allocates instances to the Service's latest ready Revision.
    ///
    /// "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    #[serde(rename="TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST")]
    TRAFFICTARGETALLOCATIONTYPELATEST,
    

    /// Allocates instances to a Revision by name.
    ///
    /// "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION"
    #[serde(rename="TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION")]
    TRAFFICTARGETALLOCATIONTYPEREVISION,
}

impl AsRef<str> for GoogleCloudRunV2TrafficTargetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2TrafficTargetTypeEnum::TRAFFICTARGETALLOCATIONTYPEUNSPECIFIED => "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED",
            GoogleCloudRunV2TrafficTargetTypeEnum::TRAFFICTARGETALLOCATIONTYPELATEST => "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST",
            GoogleCloudRunV2TrafficTargetTypeEnum::TRAFFICTARGETALLOCATIONTYPEREVISION => "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2TrafficTargetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudRunV2TrafficTargetTypeEnum::TRAFFICTARGETALLOCATIONTYPEUNSPECIFIED),
           "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST" => Ok(GoogleCloudRunV2TrafficTargetTypeEnum::TRAFFICTARGETALLOCATIONTYPELATEST),
           "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION" => Ok(GoogleCloudRunV2TrafficTargetTypeEnum::TRAFFICTARGETALLOCATIONTYPEREVISION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2TrafficTargetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2TrafficTargetStatusTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The allocation type for this traffic target.
pub enum GoogleCloudRunV2TrafficTargetStatusTypeEnum {
    

    /// Unspecified instance allocation type.
    ///
    /// "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED")]
    TRAFFICTARGETALLOCATIONTYPEUNSPECIFIED,
    

    /// Allocates instances to the Service's latest ready Revision.
    ///
    /// "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    #[serde(rename="TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST")]
    TRAFFICTARGETALLOCATIONTYPELATEST,
    

    /// Allocates instances to a Revision by name.
    ///
    /// "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION"
    #[serde(rename="TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION")]
    TRAFFICTARGETALLOCATIONTYPEREVISION,
}

impl AsRef<str> for GoogleCloudRunV2TrafficTargetStatusTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2TrafficTargetStatusTypeEnum::TRAFFICTARGETALLOCATIONTYPEUNSPECIFIED => "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED",
            GoogleCloudRunV2TrafficTargetStatusTypeEnum::TRAFFICTARGETALLOCATIONTYPELATEST => "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST",
            GoogleCloudRunV2TrafficTargetStatusTypeEnum::TRAFFICTARGETALLOCATIONTYPEREVISION => "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2TrafficTargetStatusTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudRunV2TrafficTargetStatusTypeEnum::TRAFFICTARGETALLOCATIONTYPEUNSPECIFIED),
           "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST" => Ok(GoogleCloudRunV2TrafficTargetStatusTypeEnum::TRAFFICTARGETALLOCATIONTYPELATEST),
           "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION" => Ok(GoogleCloudRunV2TrafficTargetStatusTypeEnum::TRAFFICTARGETALLOCATIONTYPEREVISION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2TrafficTargetStatusTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRunV2VpcAccesEgressEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Traffic VPC egress settings. If not provided, it defaults to PRIVATE_RANGES_ONLY.
pub enum GoogleCloudRunV2VpcAccesEgressEnum {
    

    /// Unspecified
    ///
    /// "VPC_EGRESS_UNSPECIFIED"
    #[serde(rename="VPC_EGRESS_UNSPECIFIED")]
    VPCEGRESSUNSPECIFIED,
    

    /// All outbound traffic is routed through the VPC connector.
    ///
    /// "ALL_TRAFFIC"
    #[serde(rename="ALL_TRAFFIC")]
    ALLTRAFFIC,
    

    /// Only private IP ranges are routed through the VPC connector.
    ///
    /// "PRIVATE_RANGES_ONLY"
    #[serde(rename="PRIVATE_RANGES_ONLY")]
    PRIVATERANGESONLY,
}

impl AsRef<str> for GoogleCloudRunV2VpcAccesEgressEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRunV2VpcAccesEgressEnum::VPCEGRESSUNSPECIFIED => "VPC_EGRESS_UNSPECIFIED",
            GoogleCloudRunV2VpcAccesEgressEnum::ALLTRAFFIC => "ALL_TRAFFIC",
            GoogleCloudRunV2VpcAccesEgressEnum::PRIVATERANGESONLY => "PRIVATE_RANGES_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRunV2VpcAccesEgressEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VPC_EGRESS_UNSPECIFIED" => Ok(GoogleCloudRunV2VpcAccesEgressEnum::VPCEGRESSUNSPECIFIED),
           "ALL_TRAFFIC" => Ok(GoogleCloudRunV2VpcAccesEgressEnum::ALLTRAFFIC),
           "PRIVATE_RANGES_ONLY" => Ok(GoogleCloudRunV2VpcAccesEgressEnum::PRIVATERANGESONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRunV2VpcAccesEgressEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamV1AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
    

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

impl AsRef<str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamV1AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


