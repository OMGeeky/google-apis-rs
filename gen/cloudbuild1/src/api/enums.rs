use super::*;



// region ApprovalResultDecisionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The decision of this manual approval.
pub enum ApprovalResultDecisionEnum {
    

    /// Default enum type. This should not be used.
    ///
    /// "DECISION_UNSPECIFIED"
    #[serde(rename="DECISION_UNSPECIFIED")]
    DECISIONUNSPECIFIED,
    

    /// Build is approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Build is rejected.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
}

impl AsRef<str> for ApprovalResultDecisionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApprovalResultDecisionEnum::DECISIONUNSPECIFIED => "DECISION_UNSPECIFIED",
            ApprovalResultDecisionEnum::APPROVED => "APPROVED",
            ApprovalResultDecisionEnum::REJECTED => "REJECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for ApprovalResultDecisionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DECISION_UNSPECIFIED" => Ok(ApprovalResultDecisionEnum::DECISIONUNSPECIFIED),
           "APPROVED" => Ok(ApprovalResultDecisionEnum::APPROVED),
           "REJECTED" => Ok(ApprovalResultDecisionEnum::REJECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApprovalResultDecisionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Status of the build.
pub enum BuildStatusEnum {
    

    /// Status of the build is unknown.
    ///
    /// "STATUS_UNKNOWN"
    #[serde(rename="STATUS_UNKNOWN")]
    STATUSUNKNOWN,
    

    /// Build has been created and is pending execution and queuing. It has not been queued.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Build or step is queued; work has not yet begun.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
    

    /// Build or step is being executed.
    ///
    /// "WORKING"
    #[serde(rename="WORKING")]
    WORKING,
    

    /// Build or step finished successfully.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// Build or step failed to complete successfully.
    ///
    /// "FAILURE"
    #[serde(rename="FAILURE")]
    FAILURE,
    

    /// Build or step failed due to an internal cause.
    ///
    /// "INTERNAL_ERROR"
    #[serde(rename="INTERNAL_ERROR")]
    INTERNALERROR,
    

    /// Build or step took longer than was allowed.
    ///
    /// "TIMEOUT"
    #[serde(rename="TIMEOUT")]
    TIMEOUT,
    

    /// Build or step was canceled by a user.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Build was enqueued for longer than the value of `queue_ttl`.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
}

impl AsRef<str> for BuildStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildStatusEnum::STATUSUNKNOWN => "STATUS_UNKNOWN",
            BuildStatusEnum::PENDING => "PENDING",
            BuildStatusEnum::QUEUED => "QUEUED",
            BuildStatusEnum::WORKING => "WORKING",
            BuildStatusEnum::SUCCESS => "SUCCESS",
            BuildStatusEnum::FAILURE => "FAILURE",
            BuildStatusEnum::INTERNALERROR => "INTERNAL_ERROR",
            BuildStatusEnum::TIMEOUT => "TIMEOUT",
            BuildStatusEnum::CANCELLED => "CANCELLED",
            BuildStatusEnum::EXPIRED => "EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNKNOWN" => Ok(BuildStatusEnum::STATUSUNKNOWN),
           "PENDING" => Ok(BuildStatusEnum::PENDING),
           "QUEUED" => Ok(BuildStatusEnum::QUEUED),
           "WORKING" => Ok(BuildStatusEnum::WORKING),
           "SUCCESS" => Ok(BuildStatusEnum::SUCCESS),
           "FAILURE" => Ok(BuildStatusEnum::FAILURE),
           "INTERNAL_ERROR" => Ok(BuildStatusEnum::INTERNALERROR),
           "TIMEOUT" => Ok(BuildStatusEnum::TIMEOUT),
           "CANCELLED" => Ok(BuildStatusEnum::CANCELLED),
           "EXPIRED" => Ok(BuildStatusEnum::EXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildApprovalStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this build's approval.
pub enum BuildApprovalStateEnum {
    

    /// Default enum type. This should not be used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Build approval is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Build approval has been approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Build approval has been rejected.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Build was cancelled while it was still pending approval.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for BuildApprovalStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildApprovalStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BuildApprovalStateEnum::PENDING => "PENDING",
            BuildApprovalStateEnum::APPROVED => "APPROVED",
            BuildApprovalStateEnum::REJECTED => "REJECTED",
            BuildApprovalStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildApprovalStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BuildApprovalStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(BuildApprovalStateEnum::PENDING),
           "APPROVED" => Ok(BuildApprovalStateEnum::APPROVED),
           "REJECTED" => Ok(BuildApprovalStateEnum::REJECTED),
           "CANCELLED" => Ok(BuildApprovalStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildApprovalStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionDefaultLogsBucketBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Option to specify how default logs buckets are setup.
pub enum BuildOptionDefaultLogsBucketBehaviorEnum {
    

    /// Unspecified.
    ///
    /// "DEFAULT_LOGS_BUCKET_BEHAVIOR_UNSPECIFIED"
    #[serde(rename="DEFAULT_LOGS_BUCKET_BEHAVIOR_UNSPECIFIED")]
    DEFAULTLOGSBUCKETBEHAVIORUNSPECIFIED,
    

    /// Bucket is located in user-owned project in the same region as the build. The builder service account must have access to create and write to Cloud Storage buckets in the build project.
    ///
    /// "REGIONAL_USER_OWNED_BUCKET"
    #[serde(rename="REGIONAL_USER_OWNED_BUCKET")]
    REGIONALUSEROWNEDBUCKET,
}

impl AsRef<str> for BuildOptionDefaultLogsBucketBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionDefaultLogsBucketBehaviorEnum::DEFAULTLOGSBUCKETBEHAVIORUNSPECIFIED => "DEFAULT_LOGS_BUCKET_BEHAVIOR_UNSPECIFIED",
            BuildOptionDefaultLogsBucketBehaviorEnum::REGIONALUSEROWNEDBUCKET => "REGIONAL_USER_OWNED_BUCKET",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionDefaultLogsBucketBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT_LOGS_BUCKET_BEHAVIOR_UNSPECIFIED" => Ok(BuildOptionDefaultLogsBucketBehaviorEnum::DEFAULTLOGSBUCKETBEHAVIORUNSPECIFIED),
           "REGIONAL_USER_OWNED_BUCKET" => Ok(BuildOptionDefaultLogsBucketBehaviorEnum::REGIONALUSEROWNEDBUCKET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionDefaultLogsBucketBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionLogStreamingOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option to define build log streaming behavior to Cloud Storage.
pub enum BuildOptionLogStreamingOptionEnum {
    

    /// Service may automatically determine build log streaming behavior.
    ///
    /// "STREAM_DEFAULT"
    #[serde(rename="STREAM_DEFAULT")]
    STREAMDEFAULT,
    

    /// Build logs should be streamed to Cloud Storage.
    ///
    /// "STREAM_ON"
    #[serde(rename="STREAM_ON")]
    STREAMON,
    

    /// Build logs should not be streamed to Cloud Storage; they will be written when the build is completed.
    ///
    /// "STREAM_OFF"
    #[serde(rename="STREAM_OFF")]
    STREAMOFF,
}

impl AsRef<str> for BuildOptionLogStreamingOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionLogStreamingOptionEnum::STREAMDEFAULT => "STREAM_DEFAULT",
            BuildOptionLogStreamingOptionEnum::STREAMON => "STREAM_ON",
            BuildOptionLogStreamingOptionEnum::STREAMOFF => "STREAM_OFF",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionLogStreamingOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STREAM_DEFAULT" => Ok(BuildOptionLogStreamingOptionEnum::STREAMDEFAULT),
           "STREAM_ON" => Ok(BuildOptionLogStreamingOptionEnum::STREAMON),
           "STREAM_OFF" => Ok(BuildOptionLogStreamingOptionEnum::STREAMOFF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionLogStreamingOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionLoggingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option to specify the logging mode, which determines if and where build logs are stored.
pub enum BuildOptionLoggingEnum {
    

    /// The service determines the logging mode. The default is `LEGACY`. Do not rely on the default logging behavior as it may change in the future.
    ///
    /// "LOGGING_UNSPECIFIED"
    #[serde(rename="LOGGING_UNSPECIFIED")]
    LOGGINGUNSPECIFIED,
    

    /// Build logs are stored in Cloud Logging and Cloud Storage.
    ///
    /// "LEGACY"
    #[serde(rename="LEGACY")]
    LEGACY,
    

    /// Build logs are stored in Cloud Storage.
    ///
    /// "GCS_ONLY"
    #[serde(rename="GCS_ONLY")]
    GCSONLY,
    

    /// This option is the same as CLOUD_LOGGING_ONLY.
    ///
    /// "STACKDRIVER_ONLY"
    #[serde(rename="STACKDRIVER_ONLY")]
    STACKDRIVERONLY,
    

    /// Build logs are stored in Cloud Logging. Selecting this option will not allow [logs streaming](https://cloud.google.com/sdk/gcloud/reference/builds/log).
    ///
    /// "CLOUD_LOGGING_ONLY"
    #[serde(rename="CLOUD_LOGGING_ONLY")]
    CLOUDLOGGINGONLY,
    

    /// Turn off all logging. No build logs will be captured.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
}

impl AsRef<str> for BuildOptionLoggingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionLoggingEnum::LOGGINGUNSPECIFIED => "LOGGING_UNSPECIFIED",
            BuildOptionLoggingEnum::LEGACY => "LEGACY",
            BuildOptionLoggingEnum::GCSONLY => "GCS_ONLY",
            BuildOptionLoggingEnum::STACKDRIVERONLY => "STACKDRIVER_ONLY",
            BuildOptionLoggingEnum::CLOUDLOGGINGONLY => "CLOUD_LOGGING_ONLY",
            BuildOptionLoggingEnum::NONE => "NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionLoggingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOGGING_UNSPECIFIED" => Ok(BuildOptionLoggingEnum::LOGGINGUNSPECIFIED),
           "LEGACY" => Ok(BuildOptionLoggingEnum::LEGACY),
           "GCS_ONLY" => Ok(BuildOptionLoggingEnum::GCSONLY),
           "STACKDRIVER_ONLY" => Ok(BuildOptionLoggingEnum::STACKDRIVERONLY),
           "CLOUD_LOGGING_ONLY" => Ok(BuildOptionLoggingEnum::CLOUDLOGGINGONLY),
           "NONE" => Ok(BuildOptionLoggingEnum::NONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionLoggingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionMachineTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Compute Engine machine type on which to run the build.
pub enum BuildOptionMachineTypeEnum {
    

    /// Standard machine type.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Highcpu machine with 8 CPUs.
    ///
    /// "N1_HIGHCPU_8"
    #[serde(rename="N1_HIGHCPU_8")]
    N1HIGHCPU8,
    

    /// Highcpu machine with 32 CPUs.
    ///
    /// "N1_HIGHCPU_32"
    #[serde(rename="N1_HIGHCPU_32")]
    N1HIGHCPU32,
    

    /// Highcpu e2 machine with 8 CPUs.
    ///
    /// "E2_HIGHCPU_8"
    #[serde(rename="E2_HIGHCPU_8")]
    E2HIGHCPU8,
    

    /// Highcpu e2 machine with 32 CPUs.
    ///
    /// "E2_HIGHCPU_32"
    #[serde(rename="E2_HIGHCPU_32")]
    E2HIGHCPU32,
    

    /// E2 machine with 1 CPU.
    ///
    /// "E2_MEDIUM"
    #[serde(rename="E2_MEDIUM")]
    E2MEDIUM,
}

impl AsRef<str> for BuildOptionMachineTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionMachineTypeEnum::UNSPECIFIED => "UNSPECIFIED",
            BuildOptionMachineTypeEnum::N1HIGHCPU8 => "N1_HIGHCPU_8",
            BuildOptionMachineTypeEnum::N1HIGHCPU32 => "N1_HIGHCPU_32",
            BuildOptionMachineTypeEnum::E2HIGHCPU8 => "E2_HIGHCPU_8",
            BuildOptionMachineTypeEnum::E2HIGHCPU32 => "E2_HIGHCPU_32",
            BuildOptionMachineTypeEnum::E2MEDIUM => "E2_MEDIUM",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionMachineTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(BuildOptionMachineTypeEnum::UNSPECIFIED),
           "N1_HIGHCPU_8" => Ok(BuildOptionMachineTypeEnum::N1HIGHCPU8),
           "N1_HIGHCPU_32" => Ok(BuildOptionMachineTypeEnum::N1HIGHCPU32),
           "E2_HIGHCPU_8" => Ok(BuildOptionMachineTypeEnum::E2HIGHCPU8),
           "E2_HIGHCPU_32" => Ok(BuildOptionMachineTypeEnum::E2HIGHCPU32),
           "E2_MEDIUM" => Ok(BuildOptionMachineTypeEnum::E2MEDIUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionMachineTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionRequestedVerifyOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requested verifiability options.
pub enum BuildOptionRequestedVerifyOptionEnum {
    

    /// Not a verifiable build (the default).
    ///
    /// "NOT_VERIFIED"
    #[serde(rename="NOT_VERIFIED")]
    NOTVERIFIED,
    

    /// Build must be verified.
    ///
    /// "VERIFIED"
    #[serde(rename="VERIFIED")]
    VERIFIED,
}

impl AsRef<str> for BuildOptionRequestedVerifyOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionRequestedVerifyOptionEnum::NOTVERIFIED => "NOT_VERIFIED",
            BuildOptionRequestedVerifyOptionEnum::VERIFIED => "VERIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionRequestedVerifyOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOT_VERIFIED" => Ok(BuildOptionRequestedVerifyOptionEnum::NOTVERIFIED),
           "VERIFIED" => Ok(BuildOptionRequestedVerifyOptionEnum::VERIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionRequestedVerifyOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionSourceProvenanceHashEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requested hash for SourceProvenance.
pub enum BuildOptionSourceProvenanceHashEnum {
    

    /// No hash requested.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Use a sha256 hash.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// Use a md5 hash.
    ///
    /// "MD5"
    #[serde(rename="MD5")]
    MD5,
    

    /// Use a sha512 hash.
    ///
    /// "SHA512"
    #[serde(rename="SHA512")]
    SHA512,
}

impl AsRef<str> for BuildOptionSourceProvenanceHashEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionSourceProvenanceHashEnum::NONE => "NONE",
            BuildOptionSourceProvenanceHashEnum::SHA256 => "SHA256",
            BuildOptionSourceProvenanceHashEnum::MD5 => "MD5",
            BuildOptionSourceProvenanceHashEnum::SHA512 => "SHA512",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionSourceProvenanceHashEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(BuildOptionSourceProvenanceHashEnum::NONE),
           "SHA256" => Ok(BuildOptionSourceProvenanceHashEnum::SHA256),
           "MD5" => Ok(BuildOptionSourceProvenanceHashEnum::MD5),
           "SHA512" => Ok(BuildOptionSourceProvenanceHashEnum::SHA512),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionSourceProvenanceHashEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildOptionSubstitutionOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option to specify behavior when there is an error in the substitution checks. NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden in the build configuration file.
pub enum BuildOptionSubstitutionOptionEnum {
    

    /// Fails the build if error in substitutions checks, like missing a substitution in the template or in the map.
    ///
    /// "MUST_MATCH"
    #[serde(rename="MUST_MATCH")]
    MUSTMATCH,
    

    /// Do not fail the build if error in substitutions checks.
    ///
    /// "ALLOW_LOOSE"
    #[serde(rename="ALLOW_LOOSE")]
    ALLOWLOOSE,
}

impl AsRef<str> for BuildOptionSubstitutionOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildOptionSubstitutionOptionEnum::MUSTMATCH => "MUST_MATCH",
            BuildOptionSubstitutionOptionEnum::ALLOWLOOSE => "ALLOW_LOOSE",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildOptionSubstitutionOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MUST_MATCH" => Ok(BuildOptionSubstitutionOptionEnum::MUSTMATCH),
           "ALLOW_LOOSE" => Ok(BuildOptionSubstitutionOptionEnum::ALLOWLOOSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildOptionSubstitutionOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildStepStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Status of the build step. At this time, build step status is only updated on build completion; step status is not updated in real-time as the build progresses.
pub enum BuildStepStatusEnum {
    

    /// Status of the build is unknown.
    ///
    /// "STATUS_UNKNOWN"
    #[serde(rename="STATUS_UNKNOWN")]
    STATUSUNKNOWN,
    

    /// Build has been created and is pending execution and queuing. It has not been queued.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Build or step is queued; work has not yet begun.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
    

    /// Build or step is being executed.
    ///
    /// "WORKING"
    #[serde(rename="WORKING")]
    WORKING,
    

    /// Build or step finished successfully.
    ///
    /// "SUCCESS"
    #[serde(rename="SUCCESS")]
    SUCCESS,
    

    /// Build or step failed to complete successfully.
    ///
    /// "FAILURE"
    #[serde(rename="FAILURE")]
    FAILURE,
    

    /// Build or step failed due to an internal cause.
    ///
    /// "INTERNAL_ERROR"
    #[serde(rename="INTERNAL_ERROR")]
    INTERNALERROR,
    

    /// Build or step took longer than was allowed.
    ///
    /// "TIMEOUT"
    #[serde(rename="TIMEOUT")]
    TIMEOUT,
    

    /// Build or step was canceled by a user.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Build was enqueued for longer than the value of `queue_ttl`.
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
}

impl AsRef<str> for BuildStepStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildStepStatusEnum::STATUSUNKNOWN => "STATUS_UNKNOWN",
            BuildStepStatusEnum::PENDING => "PENDING",
            BuildStepStatusEnum::QUEUED => "QUEUED",
            BuildStepStatusEnum::WORKING => "WORKING",
            BuildStepStatusEnum::SUCCESS => "SUCCESS",
            BuildStepStatusEnum::FAILURE => "FAILURE",
            BuildStepStatusEnum::INTERNALERROR => "INTERNAL_ERROR",
            BuildStepStatusEnum::TIMEOUT => "TIMEOUT",
            BuildStepStatusEnum::CANCELLED => "CANCELLED",
            BuildStepStatusEnum::EXPIRED => "EXPIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildStepStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNKNOWN" => Ok(BuildStepStatusEnum::STATUSUNKNOWN),
           "PENDING" => Ok(BuildStepStatusEnum::PENDING),
           "QUEUED" => Ok(BuildStepStatusEnum::QUEUED),
           "WORKING" => Ok(BuildStepStatusEnum::WORKING),
           "SUCCESS" => Ok(BuildStepStatusEnum::SUCCESS),
           "FAILURE" => Ok(BuildStepStatusEnum::FAILURE),
           "INTERNAL_ERROR" => Ok(BuildStepStatusEnum::INTERNALERROR),
           "TIMEOUT" => Ok(BuildStepStatusEnum::TIMEOUT),
           "CANCELLED" => Ok(BuildStepStatusEnum::CANCELLED),
           "EXPIRED" => Ok(BuildStepStatusEnum::EXPIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildStepStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildTriggerEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// EventType allows the user to explicitly set the type of event to which this BuildTrigger should respond. This field will be validated against the rest of the configuration if it is set.
pub enum BuildTriggerEventTypeEnum {
    

    /// EVENT_TYPE_UNSPECIFIED event_types are ignored.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// REPO corresponds to the supported VCS integrations.
    ///
    /// "REPO"
    #[serde(rename="REPO")]
    REPO,
    

    /// WEBHOOK corresponds to webhook triggers.
    ///
    /// "WEBHOOK"
    #[serde(rename="WEBHOOK")]
    WEBHOOK,
    

    /// PUBSUB corresponds to pubsub triggers.
    ///
    /// "PUBSUB"
    #[serde(rename="PUBSUB")]
    PUBSUB,
    

    /// MANUAL corresponds to manual-only invoked triggers.
    ///
    /// "MANUAL"
    #[serde(rename="MANUAL")]
    MANUAL,
}

impl AsRef<str> for BuildTriggerEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildTriggerEventTypeEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            BuildTriggerEventTypeEnum::REPO => "REPO",
            BuildTriggerEventTypeEnum::WEBHOOK => "WEBHOOK",
            BuildTriggerEventTypeEnum::PUBSUB => "PUBSUB",
            BuildTriggerEventTypeEnum::MANUAL => "MANUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildTriggerEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(BuildTriggerEventTypeEnum::EVENTTYPEUNSPECIFIED),
           "REPO" => Ok(BuildTriggerEventTypeEnum::REPO),
           "WEBHOOK" => Ok(BuildTriggerEventTypeEnum::WEBHOOK),
           "PUBSUB" => Ok(BuildTriggerEventTypeEnum::PUBSUB),
           "MANUAL" => Ok(BuildTriggerEventTypeEnum::MANUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildTriggerEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuildTriggerIncludeBuildLogsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If set to INCLUDE_BUILD_LOGS_WITH_STATUS, log url will be shown on GitHub page when build status is final. Setting this field to INCLUDE_BUILD_LOGS_WITH_STATUS for non GitHub triggers results in INVALID_ARGUMENT error.
pub enum BuildTriggerIncludeBuildLogsEnum {
    

    /// Build logs will not be shown on GitHub.
    ///
    /// "INCLUDE_BUILD_LOGS_UNSPECIFIED"
    #[serde(rename="INCLUDE_BUILD_LOGS_UNSPECIFIED")]
    INCLUDEBUILDLOGSUNSPECIFIED,
    

    /// Build logs will be shown on GitHub.
    ///
    /// "INCLUDE_BUILD_LOGS_WITH_STATUS"
    #[serde(rename="INCLUDE_BUILD_LOGS_WITH_STATUS")]
    INCLUDEBUILDLOGSWITHSTATUS,
}

impl AsRef<str> for BuildTriggerIncludeBuildLogsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuildTriggerIncludeBuildLogsEnum::INCLUDEBUILDLOGSUNSPECIFIED => "INCLUDE_BUILD_LOGS_UNSPECIFIED",
            BuildTriggerIncludeBuildLogsEnum::INCLUDEBUILDLOGSWITHSTATUS => "INCLUDE_BUILD_LOGS_WITH_STATUS",
        }
    }
}

impl std::convert::TryFrom< &str> for BuildTriggerIncludeBuildLogsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INCLUDE_BUILD_LOGS_UNSPECIFIED" => Ok(BuildTriggerIncludeBuildLogsEnum::INCLUDEBUILDLOGSUNSPECIFIED),
           "INCLUDE_BUILD_LOGS_WITH_STATUS" => Ok(BuildTriggerIncludeBuildLogsEnum::INCLUDEBUILDLOGSWITHSTATUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuildTriggerIncludeBuildLogsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FailureInfoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The name of the failure.
pub enum FailureInfoTypeEnum {
    

    /// Type unspecified
    ///
    /// "FAILURE_TYPE_UNSPECIFIED"
    #[serde(rename="FAILURE_TYPE_UNSPECIFIED")]
    FAILURETYPEUNSPECIFIED,
    

    /// Unable to push the image to the repository.
    ///
    /// "PUSH_FAILED"
    #[serde(rename="PUSH_FAILED")]
    PUSHFAILED,
    

    /// Final image not found.
    ///
    /// "PUSH_IMAGE_NOT_FOUND"
    #[serde(rename="PUSH_IMAGE_NOT_FOUND")]
    PUSHIMAGENOTFOUND,
    

    /// Unauthorized push of the final image.
    ///
    /// "PUSH_NOT_AUTHORIZED"
    #[serde(rename="PUSH_NOT_AUTHORIZED")]
    PUSHNOTAUTHORIZED,
    

    /// Backend logging failures. Should retry.
    ///
    /// "LOGGING_FAILURE"
    #[serde(rename="LOGGING_FAILURE")]
    LOGGINGFAILURE,
    

    /// A build step has failed.
    ///
    /// "USER_BUILD_STEP"
    #[serde(rename="USER_BUILD_STEP")]
    USERBUILDSTEP,
    

    /// The source fetching has failed.
    ///
    /// "FETCH_SOURCE_FAILED"
    #[serde(rename="FETCH_SOURCE_FAILED")]
    FETCHSOURCEFAILED,
}

impl AsRef<str> for FailureInfoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FailureInfoTypeEnum::FAILURETYPEUNSPECIFIED => "FAILURE_TYPE_UNSPECIFIED",
            FailureInfoTypeEnum::PUSHFAILED => "PUSH_FAILED",
            FailureInfoTypeEnum::PUSHIMAGENOTFOUND => "PUSH_IMAGE_NOT_FOUND",
            FailureInfoTypeEnum::PUSHNOTAUTHORIZED => "PUSH_NOT_AUTHORIZED",
            FailureInfoTypeEnum::LOGGINGFAILURE => "LOGGING_FAILURE",
            FailureInfoTypeEnum::USERBUILDSTEP => "USER_BUILD_STEP",
            FailureInfoTypeEnum::FETCHSOURCEFAILED => "FETCH_SOURCE_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for FailureInfoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_TYPE_UNSPECIFIED" => Ok(FailureInfoTypeEnum::FAILURETYPEUNSPECIFIED),
           "PUSH_FAILED" => Ok(FailureInfoTypeEnum::PUSHFAILED),
           "PUSH_IMAGE_NOT_FOUND" => Ok(FailureInfoTypeEnum::PUSHIMAGENOTFOUND),
           "PUSH_NOT_AUTHORIZED" => Ok(FailureInfoTypeEnum::PUSHNOTAUTHORIZED),
           "LOGGING_FAILURE" => Ok(FailureInfoTypeEnum::LOGGINGFAILURE),
           "USER_BUILD_STEP" => Ok(FailureInfoTypeEnum::USERBUILDSTEP),
           "FETCH_SOURCE_FAILED" => Ok(FailureInfoTypeEnum::FETCHSOURCEFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FailureInfoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GitFileSourceRepoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// See RepoType above.
pub enum GitFileSourceRepoTypeEnum {
    

    /// The default, unknown repo type. Don't use it, instead use one of the other repo types.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// A Google Cloud Source Repositories-hosted repo.
    ///
    /// "CLOUD_SOURCE_REPOSITORIES"
    #[serde(rename="CLOUD_SOURCE_REPOSITORIES")]
    CLOUDSOURCEREPOSITORIES,
    

    /// A GitHub-hosted repo not necessarily on "github.com" (i.e. GitHub Enterprise).
    ///
    /// "GITHUB"
    #[serde(rename="GITHUB")]
    GITHUB,
    

    /// A Bitbucket Server-hosted repo.
    ///
    /// "BITBUCKET_SERVER"
    #[serde(rename="BITBUCKET_SERVER")]
    BITBUCKETSERVER,
    

    /// A GitLab-hosted repo.
    ///
    /// "GITLAB"
    #[serde(rename="GITLAB")]
    GITLAB,
    

    /// A Bitbucket Cloud-hosted repo.
    ///
    /// "BITBUCKET_CLOUD"
    #[serde(rename="BITBUCKET_CLOUD")]
    BITBUCKETCLOUD,
}

impl AsRef<str> for GitFileSourceRepoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GitFileSourceRepoTypeEnum::UNKNOWN => "UNKNOWN",
            GitFileSourceRepoTypeEnum::CLOUDSOURCEREPOSITORIES => "CLOUD_SOURCE_REPOSITORIES",
            GitFileSourceRepoTypeEnum::GITHUB => "GITHUB",
            GitFileSourceRepoTypeEnum::BITBUCKETSERVER => "BITBUCKET_SERVER",
            GitFileSourceRepoTypeEnum::GITLAB => "GITLAB",
            GitFileSourceRepoTypeEnum::BITBUCKETCLOUD => "BITBUCKET_CLOUD",
        }
    }
}

impl std::convert::TryFrom< &str> for GitFileSourceRepoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(GitFileSourceRepoTypeEnum::UNKNOWN),
           "CLOUD_SOURCE_REPOSITORIES" => Ok(GitFileSourceRepoTypeEnum::CLOUDSOURCEREPOSITORIES),
           "GITHUB" => Ok(GitFileSourceRepoTypeEnum::GITHUB),
           "BITBUCKET_SERVER" => Ok(GitFileSourceRepoTypeEnum::BITBUCKETSERVER),
           "GITLAB" => Ok(GitFileSourceRepoTypeEnum::GITLAB),
           "BITBUCKET_CLOUD" => Ok(GitFileSourceRepoTypeEnum::BITBUCKETCLOUD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GitFileSourceRepoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GitRepoSourceRepoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// See RepoType below.
pub enum GitRepoSourceRepoTypeEnum {
    

    /// The default, unknown repo type. Don't use it, instead use one of the other repo types.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// A Google Cloud Source Repositories-hosted repo.
    ///
    /// "CLOUD_SOURCE_REPOSITORIES"
    #[serde(rename="CLOUD_SOURCE_REPOSITORIES")]
    CLOUDSOURCEREPOSITORIES,
    

    /// A GitHub-hosted repo not necessarily on "github.com" (i.e. GitHub Enterprise).
    ///
    /// "GITHUB"
    #[serde(rename="GITHUB")]
    GITHUB,
    

    /// A Bitbucket Server-hosted repo.
    ///
    /// "BITBUCKET_SERVER"
    #[serde(rename="BITBUCKET_SERVER")]
    BITBUCKETSERVER,
    

    /// A GitLab-hosted repo.
    ///
    /// "GITLAB"
    #[serde(rename="GITLAB")]
    GITLAB,
    

    /// A Bitbucket Cloud-hosted repo.
    ///
    /// "BITBUCKET_CLOUD"
    #[serde(rename="BITBUCKET_CLOUD")]
    BITBUCKETCLOUD,
}

impl AsRef<str> for GitRepoSourceRepoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GitRepoSourceRepoTypeEnum::UNKNOWN => "UNKNOWN",
            GitRepoSourceRepoTypeEnum::CLOUDSOURCEREPOSITORIES => "CLOUD_SOURCE_REPOSITORIES",
            GitRepoSourceRepoTypeEnum::GITHUB => "GITHUB",
            GitRepoSourceRepoTypeEnum::BITBUCKETSERVER => "BITBUCKET_SERVER",
            GitRepoSourceRepoTypeEnum::GITLAB => "GITLAB",
            GitRepoSourceRepoTypeEnum::BITBUCKETCLOUD => "BITBUCKET_CLOUD",
        }
    }
}

impl std::convert::TryFrom< &str> for GitRepoSourceRepoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(GitRepoSourceRepoTypeEnum::UNKNOWN),
           "CLOUD_SOURCE_REPOSITORIES" => Ok(GitRepoSourceRepoTypeEnum::CLOUDSOURCEREPOSITORIES),
           "GITHUB" => Ok(GitRepoSourceRepoTypeEnum::GITHUB),
           "BITBUCKET_SERVER" => Ok(GitRepoSourceRepoTypeEnum::BITBUCKETSERVER),
           "GITLAB" => Ok(GitRepoSourceRepoTypeEnum::GITLAB),
           "BITBUCKET_CLOUD" => Ok(GitRepoSourceRepoTypeEnum::BITBUCKETCLOUD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GitRepoSourceRepoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HashTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of hash that was performed.
pub enum HashTypeEnum {
    

    /// No hash requested.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Use a sha256 hash.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// Use a md5 hash.
    ///
    /// "MD5"
    #[serde(rename="MD5")]
    MD5,
    

    /// Use a sha512 hash.
    ///
    /// "SHA512"
    #[serde(rename="SHA512")]
    SHA512,
}

impl AsRef<str> for HashTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HashTypeEnum::NONE => "NONE",
            HashTypeEnum::SHA256 => "SHA256",
            HashTypeEnum::MD5 => "MD5",
            HashTypeEnum::SHA512 => "SHA512",
        }
    }
}

impl std::convert::TryFrom< &str> for HashTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(HashTypeEnum::NONE),
           "SHA256" => Ok(HashTypeEnum::SHA256),
           "MD5" => Ok(HashTypeEnum::MD5),
           "SHA512" => Ok(HashTypeEnum::SHA512),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HashTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigEgressOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option to configure network egress for the workers.
pub enum NetworkConfigEgressOptionEnum {
    

    /// If set, defaults to PUBLIC_EGRESS.
    ///
    /// "EGRESS_OPTION_UNSPECIFIED"
    #[serde(rename="EGRESS_OPTION_UNSPECIFIED")]
    EGRESSOPTIONUNSPECIFIED,
    

    /// If set, workers are created without any public address, which prevents network egress to public IPs unless a network proxy is configured.
    ///
    /// "NO_PUBLIC_EGRESS"
    #[serde(rename="NO_PUBLIC_EGRESS")]
    NOPUBLICEGRESS,
    

    /// If set, workers are created with a public address which allows for public internet egress.
    ///
    /// "PUBLIC_EGRESS"
    #[serde(rename="PUBLIC_EGRESS")]
    PUBLICEGRESS,
}

impl AsRef<str> for NetworkConfigEgressOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigEgressOptionEnum::EGRESSOPTIONUNSPECIFIED => "EGRESS_OPTION_UNSPECIFIED",
            NetworkConfigEgressOptionEnum::NOPUBLICEGRESS => "NO_PUBLIC_EGRESS",
            NetworkConfigEgressOptionEnum::PUBLICEGRESS => "PUBLIC_EGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigEgressOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EGRESS_OPTION_UNSPECIFIED" => Ok(NetworkConfigEgressOptionEnum::EGRESSOPTIONUNSPECIFIED),
           "NO_PUBLIC_EGRESS" => Ok(NetworkConfigEgressOptionEnum::NOPUBLICEGRESS),
           "PUBLIC_EGRESS" => Ok(NetworkConfigEgressOptionEnum::PUBLICEGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigEgressOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PubsubConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Potential issues with the underlying Pub/Sub subscription configuration. Only populated on get requests.
pub enum PubsubConfigStateEnum {
    

    /// The subscription configuration has not been checked.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Pub/Sub subscription is properly configured.
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// The subscription has been deleted.
    ///
    /// "SUBSCRIPTION_DELETED"
    #[serde(rename="SUBSCRIPTION_DELETED")]
    SUBSCRIPTIONDELETED,
    

    /// The topic has been deleted.
    ///
    /// "TOPIC_DELETED"
    #[serde(rename="TOPIC_DELETED")]
    TOPICDELETED,
    

    /// Some of the subscription's field are misconfigured.
    ///
    /// "SUBSCRIPTION_MISCONFIGURED"
    #[serde(rename="SUBSCRIPTION_MISCONFIGURED")]
    SUBSCRIPTIONMISCONFIGURED,
}

impl AsRef<str> for PubsubConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PubsubConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PubsubConfigStateEnum::OK => "OK",
            PubsubConfigStateEnum::SUBSCRIPTIONDELETED => "SUBSCRIPTION_DELETED",
            PubsubConfigStateEnum::TOPICDELETED => "TOPIC_DELETED",
            PubsubConfigStateEnum::SUBSCRIPTIONMISCONFIGURED => "SUBSCRIPTION_MISCONFIGURED",
        }
    }
}

impl std::convert::TryFrom< &str> for PubsubConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PubsubConfigStateEnum::STATEUNSPECIFIED),
           "OK" => Ok(PubsubConfigStateEnum::OK),
           "SUBSCRIPTION_DELETED" => Ok(PubsubConfigStateEnum::SUBSCRIPTIONDELETED),
           "TOPIC_DELETED" => Ok(PubsubConfigStateEnum::TOPICDELETED),
           "SUBSCRIPTION_MISCONFIGURED" => Ok(PubsubConfigStateEnum::SUBSCRIPTIONMISCONFIGURED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PubsubConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PullRequestFilterCommentControlEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If CommentControl is enabled, depending on the setting, builds may not fire until a repository writer comments `/gcbrun` on a pull request or `/gcbrun` is in the pull request description. Only PR comments that contain `/gcbrun` will trigger builds. If CommentControl is set to disabled, comments with `/gcbrun` from a user with repository write permission or above will still trigger builds to run.
pub enum PullRequestFilterCommentControlEnum {
    

    /// Do not require `/gcbrun` comments from a user with repository write permission or above on pull requests before builds are triggered. Comments that contain `/gcbrun` will still fire builds so this should be thought of as comments not required.
    ///
    /// "COMMENTS_DISABLED"
    #[serde(rename="COMMENTS_DISABLED")]
    COMMENTSDISABLED,
    

    /// Builds will only fire in response to pull requests if: 1. The pull request author has repository write permission or above and `/gcbrun` is in the PR description. 2. A user with repository writer permissions or above comments `/gcbrun` on a pull request authored by any user.
    ///
    /// "COMMENTS_ENABLED"
    #[serde(rename="COMMENTS_ENABLED")]
    COMMENTSENABLED,
    

    /// Builds will only fire in response to pull requests if: 1. The pull request author is a repository writer or above. 2. If the author does not have write permissions, a user with write permissions or above must comment `/gcbrun` in order to fire a build.
    ///
    /// "COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY"
    #[serde(rename="COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY")]
    COMMENTSENABLEDFOREXTERNALCONTRIBUTORSONLY,
}

impl AsRef<str> for PullRequestFilterCommentControlEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PullRequestFilterCommentControlEnum::COMMENTSDISABLED => "COMMENTS_DISABLED",
            PullRequestFilterCommentControlEnum::COMMENTSENABLED => "COMMENTS_ENABLED",
            PullRequestFilterCommentControlEnum::COMMENTSENABLEDFOREXTERNALCONTRIBUTORSONLY => "COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for PullRequestFilterCommentControlEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMMENTS_DISABLED" => Ok(PullRequestFilterCommentControlEnum::COMMENTSDISABLED),
           "COMMENTS_ENABLED" => Ok(PullRequestFilterCommentControlEnum::COMMENTSENABLED),
           "COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY" => Ok(PullRequestFilterCommentControlEnum::COMMENTSENABLEDFOREXTERNALCONTRIBUTORSONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PullRequestFilterCommentControlEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RepositoryEventConfigRepositoryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the SCM vendor the repository points to.
pub enum RepositoryEventConfigRepositoryTypeEnum {
    

    /// If unspecified, RepositoryType defaults to GITHUB.
    ///
    /// "REPOSITORY_TYPE_UNSPECIFIED"
    #[serde(rename="REPOSITORY_TYPE_UNSPECIFIED")]
    REPOSITORYTYPEUNSPECIFIED,
    

    /// The SCM repo is GITHUB.
    ///
    /// "GITHUB"
    #[serde(rename="GITHUB")]
    GITHUB,
    

    /// The SCM repo is GITHUB Enterprise.
    ///
    /// "GITHUB_ENTERPRISE"
    #[serde(rename="GITHUB_ENTERPRISE")]
    GITHUBENTERPRISE,
    

    /// The SCM repo is GITLAB Enterprise.
    ///
    /// "GITLAB_ENTERPRISE"
    #[serde(rename="GITLAB_ENTERPRISE")]
    GITLABENTERPRISE,
    

    /// The SCM repo is BITBUCKET Data Center.
    ///
    /// "BITBUCKET_DATA_CENTER"
    #[serde(rename="BITBUCKET_DATA_CENTER")]
    BITBUCKETDATACENTER,
    

    /// The SCM repo is BITBUCKET Cloud.
    ///
    /// "BITBUCKET_CLOUD"
    #[serde(rename="BITBUCKET_CLOUD")]
    BITBUCKETCLOUD,
}

impl AsRef<str> for RepositoryEventConfigRepositoryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RepositoryEventConfigRepositoryTypeEnum::REPOSITORYTYPEUNSPECIFIED => "REPOSITORY_TYPE_UNSPECIFIED",
            RepositoryEventConfigRepositoryTypeEnum::GITHUB => "GITHUB",
            RepositoryEventConfigRepositoryTypeEnum::GITHUBENTERPRISE => "GITHUB_ENTERPRISE",
            RepositoryEventConfigRepositoryTypeEnum::GITLABENTERPRISE => "GITLAB_ENTERPRISE",
            RepositoryEventConfigRepositoryTypeEnum::BITBUCKETDATACENTER => "BITBUCKET_DATA_CENTER",
            RepositoryEventConfigRepositoryTypeEnum::BITBUCKETCLOUD => "BITBUCKET_CLOUD",
        }
    }
}

impl std::convert::TryFrom< &str> for RepositoryEventConfigRepositoryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPOSITORY_TYPE_UNSPECIFIED" => Ok(RepositoryEventConfigRepositoryTypeEnum::REPOSITORYTYPEUNSPECIFIED),
           "GITHUB" => Ok(RepositoryEventConfigRepositoryTypeEnum::GITHUB),
           "GITHUB_ENTERPRISE" => Ok(RepositoryEventConfigRepositoryTypeEnum::GITHUBENTERPRISE),
           "GITLAB_ENTERPRISE" => Ok(RepositoryEventConfigRepositoryTypeEnum::GITLABENTERPRISE),
           "BITBUCKET_DATA_CENTER" => Ok(RepositoryEventConfigRepositoryTypeEnum::BITBUCKETDATACENTER),
           "BITBUCKET_CLOUD" => Ok(RepositoryEventConfigRepositoryTypeEnum::BITBUCKETCLOUD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RepositoryEventConfigRepositoryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StorageSourceSourceFetcherEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Option to specify the tool to fetch the source file for the build.
pub enum StorageSourceSourceFetcherEnum {
    

    /// Unspecified defaults to GSUTIL.
    ///
    /// "SOURCE_FETCHER_UNSPECIFIED"
    #[serde(rename="SOURCE_FETCHER_UNSPECIFIED")]
    SOURCEFETCHERUNSPECIFIED,
    

    /// Use the "gsutil" tool to download the source file.
    ///
    /// "GSUTIL"
    #[serde(rename="GSUTIL")]
    GSUTIL,
    

    /// Use the Cloud Storage Fetcher tool to download the source file.
    ///
    /// "GCS_FETCHER"
    #[serde(rename="GCS_FETCHER")]
    GCSFETCHER,
}

impl AsRef<str> for StorageSourceSourceFetcherEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StorageSourceSourceFetcherEnum::SOURCEFETCHERUNSPECIFIED => "SOURCE_FETCHER_UNSPECIFIED",
            StorageSourceSourceFetcherEnum::GSUTIL => "GSUTIL",
            StorageSourceSourceFetcherEnum::GCSFETCHER => "GCS_FETCHER",
        }
    }
}

impl std::convert::TryFrom< &str> for StorageSourceSourceFetcherEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOURCE_FETCHER_UNSPECIFIED" => Ok(StorageSourceSourceFetcherEnum::SOURCEFETCHERUNSPECIFIED),
           "GSUTIL" => Ok(StorageSourceSourceFetcherEnum::GSUTIL),
           "GCS_FETCHER" => Ok(StorageSourceSourceFetcherEnum::GCSFETCHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StorageSourceSourceFetcherEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WarningPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The priority for this warning.
pub enum WarningPriorityEnum {
    

    /// Should not be used.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    

    /// e.g. deprecation warnings and alternative feature highlights.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// e.g. automated detection of possible issues with the build.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// e.g. alerts that a feature used in the build is pending removal
    ///
    /// "ALERT"
    #[serde(rename="ALERT")]
    ALERT,
}

impl AsRef<str> for WarningPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WarningPriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            WarningPriorityEnum::INFO => "INFO",
            WarningPriorityEnum::WARNING => "WARNING",
            WarningPriorityEnum::ALERT => "ALERT",
        }
    }
}

impl std::convert::TryFrom< &str> for WarningPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(WarningPriorityEnum::PRIORITYUNSPECIFIED),
           "INFO" => Ok(WarningPriorityEnum::INFO),
           "WARNING" => Ok(WarningPriorityEnum::WARNING),
           "ALERT" => Ok(WarningPriorityEnum::ALERT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WarningPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WebhookConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Potential issues with the underlying Pub/Sub subscription configuration. Only populated on get requests.
pub enum WebhookConfigStateEnum {
    

    /// The webhook auth configuration not been checked.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The auth configuration is properly setup.
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// The secret provided in auth_method has been deleted.
    ///
    /// "SECRET_DELETED"
    #[serde(rename="SECRET_DELETED")]
    SECRETDELETED,
}

impl AsRef<str> for WebhookConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WebhookConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WebhookConfigStateEnum::OK => "OK",
            WebhookConfigStateEnum::SECRETDELETED => "SECRET_DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for WebhookConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WebhookConfigStateEnum::STATEUNSPECIFIED),
           "OK" => Ok(WebhookConfigStateEnum::OK),
           "SECRET_DELETED" => Ok(WebhookConfigStateEnum::SECRETDELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WebhookConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkerPoolStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. `WorkerPool` state.
pub enum WorkerPoolStateEnum {
    

    /// State of the `WorkerPool` is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// `WorkerPool` is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// `WorkerPool` is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// `WorkerPool` is being deleted: cancelling builds and draining workers.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// `WorkerPool` is deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// `WorkerPool` is being updated; new builds cannot be run.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for WorkerPoolStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkerPoolStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkerPoolStateEnum::CREATING => "CREATING",
            WorkerPoolStateEnum::RUNNING => "RUNNING",
            WorkerPoolStateEnum::DELETING => "DELETING",
            WorkerPoolStateEnum::DELETED => "DELETED",
            WorkerPoolStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkerPoolStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkerPoolStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(WorkerPoolStateEnum::CREATING),
           "RUNNING" => Ok(WorkerPoolStateEnum::RUNNING),
           "DELETING" => Ok(WorkerPoolStateEnum::DELETING),
           "DELETED" => Ok(WorkerPoolStateEnum::DELETED),
           "UPDATING" => Ok(WorkerPoolStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkerPoolStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


