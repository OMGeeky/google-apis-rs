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


// region AutomationRunStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the `AutomationRun`.
pub enum AutomationRunStateEnum {
    

    /// The `AutomationRun` has an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The `AutomationRun` has succeeded.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The `AutomationRun` was cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The `AutomationRun` has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The `AutomationRun` is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The `AutomationRun` is pending.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The `AutomationRun` was aborted.
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
}

impl AsRef<str> for AutomationRunStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutomationRunStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AutomationRunStateEnum::SUCCEEDED => "SUCCEEDED",
            AutomationRunStateEnum::CANCELLED => "CANCELLED",
            AutomationRunStateEnum::FAILED => "FAILED",
            AutomationRunStateEnum::INPROGRESS => "IN_PROGRESS",
            AutomationRunStateEnum::PENDING => "PENDING",
            AutomationRunStateEnum::ABORTED => "ABORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for AutomationRunStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AutomationRunStateEnum::STATEUNSPECIFIED),
           "SUCCEEDED" => Ok(AutomationRunStateEnum::SUCCEEDED),
           "CANCELLED" => Ok(AutomationRunStateEnum::CANCELLED),
           "FAILED" => Ok(AutomationRunStateEnum::FAILED),
           "IN_PROGRESS" => Ok(AutomationRunStateEnum::INPROGRESS),
           "PENDING" => Ok(AutomationRunStateEnum::PENDING),
           "ABORTED" => Ok(AutomationRunStateEnum::ABORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutomationRunStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeployJobRunFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason the deploy failed. This will always be unspecified while the deploy is in progress or if it succeeded.
pub enum DeployJobRunFailureCauseEnum {
    

    /// No reason for failure is specified.
    ///
    /// "FAILURE_CAUSE_UNSPECIFIED"
    #[serde(rename="FAILURE_CAUSE_UNSPECIFIED")]
    FAILURECAUSEUNSPECIFIED,
    

    /// Cloud Build is not available, either because it is not enabled or because Cloud Deploy has insufficient permissions. See [Required permission](https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions).
    ///
    /// "CLOUD_BUILD_UNAVAILABLE"
    #[serde(rename="CLOUD_BUILD_UNAVAILABLE")]
    CLOUDBUILDUNAVAILABLE,
    

    /// The deploy operation did not complete successfully; check Cloud Build logs.
    ///
    /// "EXECUTION_FAILED"
    #[serde(rename="EXECUTION_FAILED")]
    EXECUTIONFAILED,
    

    /// The deploy job run did not complete within the alloted time.
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// There were missing resources in the runtime environment required for a canary deployment. Check the Cloud Build logs for more information.
    ///
    /// "MISSING_RESOURCES_FOR_CANARY"
    #[serde(rename="MISSING_RESOURCES_FOR_CANARY")]
    MISSINGRESOURCESFORCANARY,
    

    /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message for additional details.
    ///
    /// "CLOUD_BUILD_REQUEST_FAILED"
    #[serde(rename="CLOUD_BUILD_REQUEST_FAILED")]
    CLOUDBUILDREQUESTFAILED,
    

    /// The deploy operation had a feature configured that is not supported.
    ///
    /// "DEPLOY_FEATURE_NOT_SUPPORTED"
    #[serde(rename="DEPLOY_FEATURE_NOT_SUPPORTED")]
    DEPLOYFEATURENOTSUPPORTED,
}

impl AsRef<str> for DeployJobRunFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeployJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED => "FAILURE_CAUSE_UNSPECIFIED",
            DeployJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE => "CLOUD_BUILD_UNAVAILABLE",
            DeployJobRunFailureCauseEnum::EXECUTIONFAILED => "EXECUTION_FAILED",
            DeployJobRunFailureCauseEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            DeployJobRunFailureCauseEnum::MISSINGRESOURCESFORCANARY => "MISSING_RESOURCES_FOR_CANARY",
            DeployJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED => "CLOUD_BUILD_REQUEST_FAILED",
            DeployJobRunFailureCauseEnum::DEPLOYFEATURENOTSUPPORTED => "DEPLOY_FEATURE_NOT_SUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeployJobRunFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_CAUSE_UNSPECIFIED" => Ok(DeployJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED),
           "CLOUD_BUILD_UNAVAILABLE" => Ok(DeployJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE),
           "EXECUTION_FAILED" => Ok(DeployJobRunFailureCauseEnum::EXECUTIONFAILED),
           "DEADLINE_EXCEEDED" => Ok(DeployJobRunFailureCauseEnum::DEADLINEEXCEEDED),
           "MISSING_RESOURCES_FOR_CANARY" => Ok(DeployJobRunFailureCauseEnum::MISSINGRESOURCESFORCANARY),
           "CLOUD_BUILD_REQUEST_FAILED" => Ok(DeployJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED),
           "DEPLOY_FEATURE_NOT_SUPPORTED" => Ok(DeployJobRunFailureCauseEnum::DEPLOYFEATURENOTSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeployJobRunFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExecutionConfigUsagesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Usages when this configuration should be applied.
pub enum ExecutionConfigUsagesEnum {
    

    /// Default value. This value is unused.
    ///
    /// "EXECUTION_ENVIRONMENT_USAGE_UNSPECIFIED"
    #[serde(rename="EXECUTION_ENVIRONMENT_USAGE_UNSPECIFIED")]
    EXECUTIONENVIRONMENTUSAGEUNSPECIFIED,
    

    /// Use for rendering.
    ///
    /// "RENDER"
    #[serde(rename="RENDER")]
    RENDER,
    

    /// Use for deploying and deployment hooks.
    ///
    /// "DEPLOY"
    #[serde(rename="DEPLOY")]
    DEPLOY,
    

    /// Use for deployment verification.
    ///
    /// "VERIFY"
    #[serde(rename="VERIFY")]
    VERIFY,
    

    /// Use for predeploy job execution.
    ///
    /// "PREDEPLOY"
    #[serde(rename="PREDEPLOY")]
    PREDEPLOY,
    

    /// Use for postdeploy job execution.
    ///
    /// "POSTDEPLOY"
    #[serde(rename="POSTDEPLOY")]
    POSTDEPLOY,
}

impl AsRef<str> for ExecutionConfigUsagesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecutionConfigUsagesEnum::EXECUTIONENVIRONMENTUSAGEUNSPECIFIED => "EXECUTION_ENVIRONMENT_USAGE_UNSPECIFIED",
            ExecutionConfigUsagesEnum::RENDER => "RENDER",
            ExecutionConfigUsagesEnum::DEPLOY => "DEPLOY",
            ExecutionConfigUsagesEnum::VERIFY => "VERIFY",
            ExecutionConfigUsagesEnum::PREDEPLOY => "PREDEPLOY",
            ExecutionConfigUsagesEnum::POSTDEPLOY => "POSTDEPLOY",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecutionConfigUsagesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_ENVIRONMENT_USAGE_UNSPECIFIED" => Ok(ExecutionConfigUsagesEnum::EXECUTIONENVIRONMENTUSAGEUNSPECIFIED),
           "RENDER" => Ok(ExecutionConfigUsagesEnum::RENDER),
           "DEPLOY" => Ok(ExecutionConfigUsagesEnum::DEPLOY),
           "VERIFY" => Ok(ExecutionConfigUsagesEnum::VERIFY),
           "PREDEPLOY" => Ok(ExecutionConfigUsagesEnum::PREDEPLOY),
           "POSTDEPLOY" => Ok(ExecutionConfigUsagesEnum::POSTDEPLOY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecutionConfigUsagesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the Job.
pub enum JobStateEnum {
    

    /// The Job has an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Job is waiting for an earlier Phase(s) or Job(s) to complete.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The Job is disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The Job is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The Job succeeded.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The Job failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The Job was aborted.
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
    

    /// The Job was skipped.
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
    

    /// The Job was ignored.
    ///
    /// "IGNORED"
    #[serde(rename="IGNORED")]
    IGNORED,
}

impl AsRef<str> for JobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            JobStateEnum::PENDING => "PENDING",
            JobStateEnum::DISABLED => "DISABLED",
            JobStateEnum::INPROGRESS => "IN_PROGRESS",
            JobStateEnum::SUCCEEDED => "SUCCEEDED",
            JobStateEnum::FAILED => "FAILED",
            JobStateEnum::ABORTED => "ABORTED",
            JobStateEnum::SKIPPED => "SKIPPED",
            JobStateEnum::IGNORED => "IGNORED",
        }
    }
}

impl std::convert::TryFrom< &str> for JobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(JobStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(JobStateEnum::PENDING),
           "DISABLED" => Ok(JobStateEnum::DISABLED),
           "IN_PROGRESS" => Ok(JobStateEnum::INPROGRESS),
           "SUCCEEDED" => Ok(JobStateEnum::SUCCEEDED),
           "FAILED" => Ok(JobStateEnum::FAILED),
           "ABORTED" => Ok(JobStateEnum::ABORTED),
           "SKIPPED" => Ok(JobStateEnum::SKIPPED),
           "IGNORED" => Ok(JobStateEnum::IGNORED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobRunStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the `JobRun`.
pub enum JobRunStateEnum {
    

    /// The `JobRun` has an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The `JobRun` is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The `JobRun` has succeeded.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The `JobRun` has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The `JobRun` is terminating.
    ///
    /// "TERMINATING"
    #[serde(rename="TERMINATING")]
    TERMINATING,
    

    /// The `JobRun` was terminated.
    ///
    /// "TERMINATED"
    #[serde(rename="TERMINATED")]
    TERMINATED,
}

impl AsRef<str> for JobRunStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobRunStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            JobRunStateEnum::INPROGRESS => "IN_PROGRESS",
            JobRunStateEnum::SUCCEEDED => "SUCCEEDED",
            JobRunStateEnum::FAILED => "FAILED",
            JobRunStateEnum::TERMINATING => "TERMINATING",
            JobRunStateEnum::TERMINATED => "TERMINATED",
        }
    }
}

impl std::convert::TryFrom< &str> for JobRunStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(JobRunStateEnum::STATEUNSPECIFIED),
           "IN_PROGRESS" => Ok(JobRunStateEnum::INPROGRESS),
           "SUCCEEDED" => Ok(JobRunStateEnum::SUCCEEDED),
           "FAILED" => Ok(JobRunStateEnum::FAILED),
           "TERMINATING" => Ok(JobRunStateEnum::TERMINATING),
           "TERMINATED" => Ok(JobRunStateEnum::TERMINATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobRunStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PhaseStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the Phase.
pub enum PhaseStateEnum {
    

    /// The Phase has an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Phase is waiting for an earlier Phase(s) to complete.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The Phase is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The Phase has succeeded.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The Phase has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The Phase was aborted.
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
    

    /// The Phase was skipped.
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
}

impl AsRef<str> for PhaseStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PhaseStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PhaseStateEnum::PENDING => "PENDING",
            PhaseStateEnum::INPROGRESS => "IN_PROGRESS",
            PhaseStateEnum::SUCCEEDED => "SUCCEEDED",
            PhaseStateEnum::FAILED => "FAILED",
            PhaseStateEnum::ABORTED => "ABORTED",
            PhaseStateEnum::SKIPPED => "SKIPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for PhaseStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PhaseStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(PhaseStateEnum::PENDING),
           "IN_PROGRESS" => Ok(PhaseStateEnum::INPROGRESS),
           "SUCCEEDED" => Ok(PhaseStateEnum::SUCCEEDED),
           "FAILED" => Ok(PhaseStateEnum::FAILED),
           "ABORTED" => Ok(PhaseStateEnum::ABORTED),
           "SKIPPED" => Ok(PhaseStateEnum::SKIPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PhaseStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostdeployJobRunFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason the postdeploy failed. This will always be unspecified while the postdeploy is in progress or if it succeeded.
pub enum PostdeployJobRunFailureCauseEnum {
    

    /// No reason for failure is specified.
    ///
    /// "FAILURE_CAUSE_UNSPECIFIED"
    #[serde(rename="FAILURE_CAUSE_UNSPECIFIED")]
    FAILURECAUSEUNSPECIFIED,
    

    /// Cloud Build is not available, either because it is not enabled or because Cloud Deploy has insufficient permissions. See [required permission](https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions).
    ///
    /// "CLOUD_BUILD_UNAVAILABLE"
    #[serde(rename="CLOUD_BUILD_UNAVAILABLE")]
    CLOUDBUILDUNAVAILABLE,
    

    /// The postdeploy operation did not complete successfully; check Cloud Build logs.
    ///
    /// "EXECUTION_FAILED"
    #[serde(rename="EXECUTION_FAILED")]
    EXECUTIONFAILED,
    

    /// The postdeploy job run did not complete within the alloted time.
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message for additional details.
    ///
    /// "CLOUD_BUILD_REQUEST_FAILED"
    #[serde(rename="CLOUD_BUILD_REQUEST_FAILED")]
    CLOUDBUILDREQUESTFAILED,
}

impl AsRef<str> for PostdeployJobRunFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostdeployJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED => "FAILURE_CAUSE_UNSPECIFIED",
            PostdeployJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE => "CLOUD_BUILD_UNAVAILABLE",
            PostdeployJobRunFailureCauseEnum::EXECUTIONFAILED => "EXECUTION_FAILED",
            PostdeployJobRunFailureCauseEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            PostdeployJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED => "CLOUD_BUILD_REQUEST_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostdeployJobRunFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_CAUSE_UNSPECIFIED" => Ok(PostdeployJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED),
           "CLOUD_BUILD_UNAVAILABLE" => Ok(PostdeployJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE),
           "EXECUTION_FAILED" => Ok(PostdeployJobRunFailureCauseEnum::EXECUTIONFAILED),
           "DEADLINE_EXCEEDED" => Ok(PostdeployJobRunFailureCauseEnum::DEADLINEEXCEEDED),
           "CLOUD_BUILD_REQUEST_FAILED" => Ok(PostdeployJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostdeployJobRunFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PredeployJobRunFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason the predeploy failed. This will always be unspecified while the predeploy is in progress or if it succeeded.
pub enum PredeployJobRunFailureCauseEnum {
    

    /// No reason for failure is specified.
    ///
    /// "FAILURE_CAUSE_UNSPECIFIED"
    #[serde(rename="FAILURE_CAUSE_UNSPECIFIED")]
    FAILURECAUSEUNSPECIFIED,
    

    /// Cloud Build is not available, either because it is not enabled or because Cloud Deploy has insufficient permissions. See [required permission](https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions).
    ///
    /// "CLOUD_BUILD_UNAVAILABLE"
    #[serde(rename="CLOUD_BUILD_UNAVAILABLE")]
    CLOUDBUILDUNAVAILABLE,
    

    /// The predeploy operation did not complete successfully; check Cloud Build logs.
    ///
    /// "EXECUTION_FAILED"
    #[serde(rename="EXECUTION_FAILED")]
    EXECUTIONFAILED,
    

    /// The predeploy job run did not complete within the alloted time.
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message for additional details.
    ///
    /// "CLOUD_BUILD_REQUEST_FAILED"
    #[serde(rename="CLOUD_BUILD_REQUEST_FAILED")]
    CLOUDBUILDREQUESTFAILED,
}

impl AsRef<str> for PredeployJobRunFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PredeployJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED => "FAILURE_CAUSE_UNSPECIFIED",
            PredeployJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE => "CLOUD_BUILD_UNAVAILABLE",
            PredeployJobRunFailureCauseEnum::EXECUTIONFAILED => "EXECUTION_FAILED",
            PredeployJobRunFailureCauseEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            PredeployJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED => "CLOUD_BUILD_REQUEST_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for PredeployJobRunFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_CAUSE_UNSPECIFIED" => Ok(PredeployJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED),
           "CLOUD_BUILD_UNAVAILABLE" => Ok(PredeployJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE),
           "EXECUTION_FAILED" => Ok(PredeployJobRunFailureCauseEnum::EXECUTIONFAILED),
           "DEADLINE_EXCEEDED" => Ok(PredeployJobRunFailureCauseEnum::DEADLINEEXCEEDED),
           "CLOUD_BUILD_REQUEST_FAILED" => Ok(PredeployJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PredeployJobRunFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReleaseRenderStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the render operation.
pub enum ReleaseRenderStateEnum {
    

    /// The render state is unspecified.
    ///
    /// "RENDER_STATE_UNSPECIFIED"
    #[serde(rename="RENDER_STATE_UNSPECIFIED")]
    RENDERSTATEUNSPECIFIED,
    

    /// All rendering operations have completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// All rendering operations have completed, and one or more have failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Rendering has started and is not complete.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
}

impl AsRef<str> for ReleaseRenderStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReleaseRenderStateEnum::RENDERSTATEUNSPECIFIED => "RENDER_STATE_UNSPECIFIED",
            ReleaseRenderStateEnum::SUCCEEDED => "SUCCEEDED",
            ReleaseRenderStateEnum::FAILED => "FAILED",
            ReleaseRenderStateEnum::INPROGRESS => "IN_PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for ReleaseRenderStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RENDER_STATE_UNSPECIFIED" => Ok(ReleaseRenderStateEnum::RENDERSTATEUNSPECIFIED),
           "SUCCEEDED" => Ok(ReleaseRenderStateEnum::SUCCEEDED),
           "FAILED" => Ok(ReleaseRenderStateEnum::FAILED),
           "IN_PROGRESS" => Ok(ReleaseRenderStateEnum::INPROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReleaseRenderStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RetryBackoffModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The pattern of how wait time will be increased. Default is linear. Backoff mode will be ignored if `wait` is 0.
pub enum RetryBackoffModeEnum {
    

    /// No WaitMode is specified.
    ///
    /// "BACKOFF_MODE_UNSPECIFIED"
    #[serde(rename="BACKOFF_MODE_UNSPECIFIED")]
    BACKOFFMODEUNSPECIFIED,
    

    /// Increases the wait time linearly.
    ///
    /// "BACKOFF_MODE_LINEAR"
    #[serde(rename="BACKOFF_MODE_LINEAR")]
    BACKOFFMODELINEAR,
    

    /// Increases the wait time exponentially.
    ///
    /// "BACKOFF_MODE_EXPONENTIAL"
    #[serde(rename="BACKOFF_MODE_EXPONENTIAL")]
    BACKOFFMODEEXPONENTIAL,
}

impl AsRef<str> for RetryBackoffModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RetryBackoffModeEnum::BACKOFFMODEUNSPECIFIED => "BACKOFF_MODE_UNSPECIFIED",
            RetryBackoffModeEnum::BACKOFFMODELINEAR => "BACKOFF_MODE_LINEAR",
            RetryBackoffModeEnum::BACKOFFMODEEXPONENTIAL => "BACKOFF_MODE_EXPONENTIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for RetryBackoffModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BACKOFF_MODE_UNSPECIFIED" => Ok(RetryBackoffModeEnum::BACKOFFMODEUNSPECIFIED),
           "BACKOFF_MODE_LINEAR" => Ok(RetryBackoffModeEnum::BACKOFFMODELINEAR),
           "BACKOFF_MODE_EXPONENTIAL" => Ok(RetryBackoffModeEnum::BACKOFFMODEEXPONENTIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RetryBackoffModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RetryAttemptStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Valid state of this retry action.
pub enum RetryAttemptStateEnum {
    

    /// The `repair` has an unspecified state.
    ///
    /// "REPAIR_STATE_UNSPECIFIED"
    #[serde(rename="REPAIR_STATE_UNSPECIFIED")]
    REPAIRSTATEUNSPECIFIED,
    

    /// The `repair` action has succeeded.
    ///
    /// "REPAIR_STATE_SUCCEEDED"
    #[serde(rename="REPAIR_STATE_SUCCEEDED")]
    REPAIRSTATESUCCEEDED,
    

    /// The `repair` action was cancelled.
    ///
    /// "REPAIR_STATE_CANCELLED"
    #[serde(rename="REPAIR_STATE_CANCELLED")]
    REPAIRSTATECANCELLED,
    

    /// The `repair` action has failed.
    ///
    /// "REPAIR_STATE_FAILED"
    #[serde(rename="REPAIR_STATE_FAILED")]
    REPAIRSTATEFAILED,
    

    /// The `repair` action is in progress.
    ///
    /// "REPAIR_STATE_IN_PROGRESS"
    #[serde(rename="REPAIR_STATE_IN_PROGRESS")]
    REPAIRSTATEINPROGRESS,
    

    /// The `repair` action is pending.
    ///
    /// "REPAIR_STATE_PENDING"
    #[serde(rename="REPAIR_STATE_PENDING")]
    REPAIRSTATEPENDING,
    

    /// The `repair` action was skipped.
    ///
    /// "REPAIR_STATE_SKIPPED"
    #[serde(rename="REPAIR_STATE_SKIPPED")]
    REPAIRSTATESKIPPED,
    

    /// The `repair` action was aborted.
    ///
    /// "REPAIR_STATE_ABORTED"
    #[serde(rename="REPAIR_STATE_ABORTED")]
    REPAIRSTATEABORTED,
}

impl AsRef<str> for RetryAttemptStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RetryAttemptStateEnum::REPAIRSTATEUNSPECIFIED => "REPAIR_STATE_UNSPECIFIED",
            RetryAttemptStateEnum::REPAIRSTATESUCCEEDED => "REPAIR_STATE_SUCCEEDED",
            RetryAttemptStateEnum::REPAIRSTATECANCELLED => "REPAIR_STATE_CANCELLED",
            RetryAttemptStateEnum::REPAIRSTATEFAILED => "REPAIR_STATE_FAILED",
            RetryAttemptStateEnum::REPAIRSTATEINPROGRESS => "REPAIR_STATE_IN_PROGRESS",
            RetryAttemptStateEnum::REPAIRSTATEPENDING => "REPAIR_STATE_PENDING",
            RetryAttemptStateEnum::REPAIRSTATESKIPPED => "REPAIR_STATE_SKIPPED",
            RetryAttemptStateEnum::REPAIRSTATEABORTED => "REPAIR_STATE_ABORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RetryAttemptStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPAIR_STATE_UNSPECIFIED" => Ok(RetryAttemptStateEnum::REPAIRSTATEUNSPECIFIED),
           "REPAIR_STATE_SUCCEEDED" => Ok(RetryAttemptStateEnum::REPAIRSTATESUCCEEDED),
           "REPAIR_STATE_CANCELLED" => Ok(RetryAttemptStateEnum::REPAIRSTATECANCELLED),
           "REPAIR_STATE_FAILED" => Ok(RetryAttemptStateEnum::REPAIRSTATEFAILED),
           "REPAIR_STATE_IN_PROGRESS" => Ok(RetryAttemptStateEnum::REPAIRSTATEINPROGRESS),
           "REPAIR_STATE_PENDING" => Ok(RetryAttemptStateEnum::REPAIRSTATEPENDING),
           "REPAIR_STATE_SKIPPED" => Ok(RetryAttemptStateEnum::REPAIRSTATESKIPPED),
           "REPAIR_STATE_ABORTED" => Ok(RetryAttemptStateEnum::REPAIRSTATEABORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RetryAttemptStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RetryPhaseBackoffModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The pattern of how the wait time of the retry attempt is calculated.
pub enum RetryPhaseBackoffModeEnum {
    

    /// No WaitMode is specified.
    ///
    /// "BACKOFF_MODE_UNSPECIFIED"
    #[serde(rename="BACKOFF_MODE_UNSPECIFIED")]
    BACKOFFMODEUNSPECIFIED,
    

    /// Increases the wait time linearly.
    ///
    /// "BACKOFF_MODE_LINEAR"
    #[serde(rename="BACKOFF_MODE_LINEAR")]
    BACKOFFMODELINEAR,
    

    /// Increases the wait time exponentially.
    ///
    /// "BACKOFF_MODE_EXPONENTIAL"
    #[serde(rename="BACKOFF_MODE_EXPONENTIAL")]
    BACKOFFMODEEXPONENTIAL,
}

impl AsRef<str> for RetryPhaseBackoffModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RetryPhaseBackoffModeEnum::BACKOFFMODEUNSPECIFIED => "BACKOFF_MODE_UNSPECIFIED",
            RetryPhaseBackoffModeEnum::BACKOFFMODELINEAR => "BACKOFF_MODE_LINEAR",
            RetryPhaseBackoffModeEnum::BACKOFFMODEEXPONENTIAL => "BACKOFF_MODE_EXPONENTIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for RetryPhaseBackoffModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BACKOFF_MODE_UNSPECIFIED" => Ok(RetryPhaseBackoffModeEnum::BACKOFFMODEUNSPECIFIED),
           "BACKOFF_MODE_LINEAR" => Ok(RetryPhaseBackoffModeEnum::BACKOFFMODELINEAR),
           "BACKOFF_MODE_EXPONENTIAL" => Ok(RetryPhaseBackoffModeEnum::BACKOFFMODEEXPONENTIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RetryPhaseBackoffModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RollbackAttemptStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Valid state of this rollback action.
pub enum RollbackAttemptStateEnum {
    

    /// The `repair` has an unspecified state.
    ///
    /// "REPAIR_STATE_UNSPECIFIED"
    #[serde(rename="REPAIR_STATE_UNSPECIFIED")]
    REPAIRSTATEUNSPECIFIED,
    

    /// The `repair` action has succeeded.
    ///
    /// "REPAIR_STATE_SUCCEEDED"
    #[serde(rename="REPAIR_STATE_SUCCEEDED")]
    REPAIRSTATESUCCEEDED,
    

    /// The `repair` action was cancelled.
    ///
    /// "REPAIR_STATE_CANCELLED"
    #[serde(rename="REPAIR_STATE_CANCELLED")]
    REPAIRSTATECANCELLED,
    

    /// The `repair` action has failed.
    ///
    /// "REPAIR_STATE_FAILED"
    #[serde(rename="REPAIR_STATE_FAILED")]
    REPAIRSTATEFAILED,
    

    /// The `repair` action is in progress.
    ///
    /// "REPAIR_STATE_IN_PROGRESS"
    #[serde(rename="REPAIR_STATE_IN_PROGRESS")]
    REPAIRSTATEINPROGRESS,
    

    /// The `repair` action is pending.
    ///
    /// "REPAIR_STATE_PENDING"
    #[serde(rename="REPAIR_STATE_PENDING")]
    REPAIRSTATEPENDING,
    

    /// The `repair` action was skipped.
    ///
    /// "REPAIR_STATE_SKIPPED"
    #[serde(rename="REPAIR_STATE_SKIPPED")]
    REPAIRSTATESKIPPED,
    

    /// The `repair` action was aborted.
    ///
    /// "REPAIR_STATE_ABORTED"
    #[serde(rename="REPAIR_STATE_ABORTED")]
    REPAIRSTATEABORTED,
}

impl AsRef<str> for RollbackAttemptStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RollbackAttemptStateEnum::REPAIRSTATEUNSPECIFIED => "REPAIR_STATE_UNSPECIFIED",
            RollbackAttemptStateEnum::REPAIRSTATESUCCEEDED => "REPAIR_STATE_SUCCEEDED",
            RollbackAttemptStateEnum::REPAIRSTATECANCELLED => "REPAIR_STATE_CANCELLED",
            RollbackAttemptStateEnum::REPAIRSTATEFAILED => "REPAIR_STATE_FAILED",
            RollbackAttemptStateEnum::REPAIRSTATEINPROGRESS => "REPAIR_STATE_IN_PROGRESS",
            RollbackAttemptStateEnum::REPAIRSTATEPENDING => "REPAIR_STATE_PENDING",
            RollbackAttemptStateEnum::REPAIRSTATESKIPPED => "REPAIR_STATE_SKIPPED",
            RollbackAttemptStateEnum::REPAIRSTATEABORTED => "REPAIR_STATE_ABORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RollbackAttemptStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPAIR_STATE_UNSPECIFIED" => Ok(RollbackAttemptStateEnum::REPAIRSTATEUNSPECIFIED),
           "REPAIR_STATE_SUCCEEDED" => Ok(RollbackAttemptStateEnum::REPAIRSTATESUCCEEDED),
           "REPAIR_STATE_CANCELLED" => Ok(RollbackAttemptStateEnum::REPAIRSTATECANCELLED),
           "REPAIR_STATE_FAILED" => Ok(RollbackAttemptStateEnum::REPAIRSTATEFAILED),
           "REPAIR_STATE_IN_PROGRESS" => Ok(RollbackAttemptStateEnum::REPAIRSTATEINPROGRESS),
           "REPAIR_STATE_PENDING" => Ok(RollbackAttemptStateEnum::REPAIRSTATEPENDING),
           "REPAIR_STATE_SKIPPED" => Ok(RollbackAttemptStateEnum::REPAIRSTATESKIPPED),
           "REPAIR_STATE_ABORTED" => Ok(RollbackAttemptStateEnum::REPAIRSTATEABORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RollbackAttemptStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RolloutApprovalStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Approval state of the `Rollout`.
pub enum RolloutApprovalStateEnum {
    

    /// The `Rollout` has an unspecified approval state.
    ///
    /// "APPROVAL_STATE_UNSPECIFIED"
    #[serde(rename="APPROVAL_STATE_UNSPECIFIED")]
    APPROVALSTATEUNSPECIFIED,
    

    /// The `Rollout` requires approval.
    ///
    /// "NEEDS_APPROVAL"
    #[serde(rename="NEEDS_APPROVAL")]
    NEEDSAPPROVAL,
    

    /// The `Rollout` does not require approval.
    ///
    /// "DOES_NOT_NEED_APPROVAL"
    #[serde(rename="DOES_NOT_NEED_APPROVAL")]
    DOESNOTNEEDAPPROVAL,
    

    /// The `Rollout` has been approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// The `Rollout` has been rejected.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
}

impl AsRef<str> for RolloutApprovalStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RolloutApprovalStateEnum::APPROVALSTATEUNSPECIFIED => "APPROVAL_STATE_UNSPECIFIED",
            RolloutApprovalStateEnum::NEEDSAPPROVAL => "NEEDS_APPROVAL",
            RolloutApprovalStateEnum::DOESNOTNEEDAPPROVAL => "DOES_NOT_NEED_APPROVAL",
            RolloutApprovalStateEnum::APPROVED => "APPROVED",
            RolloutApprovalStateEnum::REJECTED => "REJECTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RolloutApprovalStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APPROVAL_STATE_UNSPECIFIED" => Ok(RolloutApprovalStateEnum::APPROVALSTATEUNSPECIFIED),
           "NEEDS_APPROVAL" => Ok(RolloutApprovalStateEnum::NEEDSAPPROVAL),
           "DOES_NOT_NEED_APPROVAL" => Ok(RolloutApprovalStateEnum::DOESNOTNEEDAPPROVAL),
           "APPROVED" => Ok(RolloutApprovalStateEnum::APPROVED),
           "REJECTED" => Ok(RolloutApprovalStateEnum::REJECTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RolloutApprovalStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RolloutDeployFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason this rollout failed. This will always be unspecified while the rollout is in progress.
pub enum RolloutDeployFailureCauseEnum {
    

    /// No reason for failure is specified.
    ///
    /// "FAILURE_CAUSE_UNSPECIFIED"
    #[serde(rename="FAILURE_CAUSE_UNSPECIFIED")]
    FAILURECAUSEUNSPECIFIED,
    

    /// Cloud Build is not available, either because it is not enabled or because Cloud Deploy has insufficient permissions. See [required permission](https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions).
    ///
    /// "CLOUD_BUILD_UNAVAILABLE"
    #[serde(rename="CLOUD_BUILD_UNAVAILABLE")]
    CLOUDBUILDUNAVAILABLE,
    

    /// The deploy operation did not complete successfully; check Cloud Build logs.
    ///
    /// "EXECUTION_FAILED"
    #[serde(rename="EXECUTION_FAILED")]
    EXECUTIONFAILED,
    

    /// Deployment did not complete within the alloted time.
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// Release is in a failed state.
    ///
    /// "RELEASE_FAILED"
    #[serde(rename="RELEASE_FAILED")]
    RELEASEFAILED,
    

    /// Release is abandoned.
    ///
    /// "RELEASE_ABANDONED"
    #[serde(rename="RELEASE_ABANDONED")]
    RELEASEABANDONED,
    

    /// No Skaffold verify configuration was found.
    ///
    /// "VERIFICATION_CONFIG_NOT_FOUND"
    #[serde(rename="VERIFICATION_CONFIG_NOT_FOUND")]
    VERIFICATIONCONFIGNOTFOUND,
    

    /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message for additional details.
    ///
    /// "CLOUD_BUILD_REQUEST_FAILED"
    #[serde(rename="CLOUD_BUILD_REQUEST_FAILED")]
    CLOUDBUILDREQUESTFAILED,
    

    /// A Rollout operation had a feature configured that is not supported.
    ///
    /// "OPERATION_FEATURE_NOT_SUPPORTED"
    #[serde(rename="OPERATION_FEATURE_NOT_SUPPORTED")]
    OPERATIONFEATURENOTSUPPORTED,
}

impl AsRef<str> for RolloutDeployFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RolloutDeployFailureCauseEnum::FAILURECAUSEUNSPECIFIED => "FAILURE_CAUSE_UNSPECIFIED",
            RolloutDeployFailureCauseEnum::CLOUDBUILDUNAVAILABLE => "CLOUD_BUILD_UNAVAILABLE",
            RolloutDeployFailureCauseEnum::EXECUTIONFAILED => "EXECUTION_FAILED",
            RolloutDeployFailureCauseEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            RolloutDeployFailureCauseEnum::RELEASEFAILED => "RELEASE_FAILED",
            RolloutDeployFailureCauseEnum::RELEASEABANDONED => "RELEASE_ABANDONED",
            RolloutDeployFailureCauseEnum::VERIFICATIONCONFIGNOTFOUND => "VERIFICATION_CONFIG_NOT_FOUND",
            RolloutDeployFailureCauseEnum::CLOUDBUILDREQUESTFAILED => "CLOUD_BUILD_REQUEST_FAILED",
            RolloutDeployFailureCauseEnum::OPERATIONFEATURENOTSUPPORTED => "OPERATION_FEATURE_NOT_SUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RolloutDeployFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_CAUSE_UNSPECIFIED" => Ok(RolloutDeployFailureCauseEnum::FAILURECAUSEUNSPECIFIED),
           "CLOUD_BUILD_UNAVAILABLE" => Ok(RolloutDeployFailureCauseEnum::CLOUDBUILDUNAVAILABLE),
           "EXECUTION_FAILED" => Ok(RolloutDeployFailureCauseEnum::EXECUTIONFAILED),
           "DEADLINE_EXCEEDED" => Ok(RolloutDeployFailureCauseEnum::DEADLINEEXCEEDED),
           "RELEASE_FAILED" => Ok(RolloutDeployFailureCauseEnum::RELEASEFAILED),
           "RELEASE_ABANDONED" => Ok(RolloutDeployFailureCauseEnum::RELEASEABANDONED),
           "VERIFICATION_CONFIG_NOT_FOUND" => Ok(RolloutDeployFailureCauseEnum::VERIFICATIONCONFIGNOTFOUND),
           "CLOUD_BUILD_REQUEST_FAILED" => Ok(RolloutDeployFailureCauseEnum::CLOUDBUILDREQUESTFAILED),
           "OPERATION_FEATURE_NOT_SUPPORTED" => Ok(RolloutDeployFailureCauseEnum::OPERATIONFEATURENOTSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RolloutDeployFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RolloutStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the `Rollout`.
pub enum RolloutStateEnum {
    

    /// The `Rollout` has an unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The `Rollout` has completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The `Rollout` has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The `Rollout` is being deployed.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// The `Rollout` needs approval.
    ///
    /// "PENDING_APPROVAL"
    #[serde(rename="PENDING_APPROVAL")]
    PENDINGAPPROVAL,
    

    /// An approver rejected the `Rollout`.
    ///
    /// "APPROVAL_REJECTED"
    #[serde(rename="APPROVAL_REJECTED")]
    APPROVALREJECTED,
    

    /// The `Rollout` is waiting for an earlier Rollout(s) to complete on this `Target`.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The `Rollout` is waiting for the `Release` to be fully rendered.
    ///
    /// "PENDING_RELEASE"
    #[serde(rename="PENDING_RELEASE")]
    PENDINGRELEASE,
    

    /// The `Rollout` is in the process of being cancelled.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The `Rollout` has been cancelled.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The `Rollout` is halted.
    ///
    /// "HALTED"
    #[serde(rename="HALTED")]
    HALTED,
}

impl AsRef<str> for RolloutStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RolloutStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RolloutStateEnum::SUCCEEDED => "SUCCEEDED",
            RolloutStateEnum::FAILED => "FAILED",
            RolloutStateEnum::INPROGRESS => "IN_PROGRESS",
            RolloutStateEnum::PENDINGAPPROVAL => "PENDING_APPROVAL",
            RolloutStateEnum::APPROVALREJECTED => "APPROVAL_REJECTED",
            RolloutStateEnum::PENDING => "PENDING",
            RolloutStateEnum::PENDINGRELEASE => "PENDING_RELEASE",
            RolloutStateEnum::CANCELLING => "CANCELLING",
            RolloutStateEnum::CANCELLED => "CANCELLED",
            RolloutStateEnum::HALTED => "HALTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RolloutStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RolloutStateEnum::STATEUNSPECIFIED),
           "SUCCEEDED" => Ok(RolloutStateEnum::SUCCEEDED),
           "FAILED" => Ok(RolloutStateEnum::FAILED),
           "IN_PROGRESS" => Ok(RolloutStateEnum::INPROGRESS),
           "PENDING_APPROVAL" => Ok(RolloutStateEnum::PENDINGAPPROVAL),
           "APPROVAL_REJECTED" => Ok(RolloutStateEnum::APPROVALREJECTED),
           "PENDING" => Ok(RolloutStateEnum::PENDING),
           "PENDING_RELEASE" => Ok(RolloutStateEnum::PENDINGRELEASE),
           "CANCELLING" => Ok(RolloutStateEnum::CANCELLING),
           "CANCELLED" => Ok(RolloutStateEnum::CANCELLED),
           "HALTED" => Ok(RolloutStateEnum::HALTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RolloutStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SkaffoldSupportedConditionSkaffoldSupportStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Skaffold support state for this release's version of Skaffold.
pub enum SkaffoldSupportedConditionSkaffoldSupportStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "SKAFFOLD_SUPPORT_STATE_UNSPECIFIED"
    #[serde(rename="SKAFFOLD_SUPPORT_STATE_UNSPECIFIED")]
    SKAFFOLDSUPPORTSTATEUNSPECIFIED,
    

    /// This Skaffold version is currently supported.
    ///
    /// "SKAFFOLD_SUPPORT_STATE_SUPPORTED"
    #[serde(rename="SKAFFOLD_SUPPORT_STATE_SUPPORTED")]
    SKAFFOLDSUPPORTSTATESUPPORTED,
    

    /// This Skaffold version is in maintenance mode.
    ///
    /// "SKAFFOLD_SUPPORT_STATE_MAINTENANCE_MODE"
    #[serde(rename="SKAFFOLD_SUPPORT_STATE_MAINTENANCE_MODE")]
    SKAFFOLDSUPPORTSTATEMAINTENANCEMODE,
    

    /// This Skaffold version is no longer supported.
    ///
    /// "SKAFFOLD_SUPPORT_STATE_UNSUPPORTED"
    #[serde(rename="SKAFFOLD_SUPPORT_STATE_UNSUPPORTED")]
    SKAFFOLDSUPPORTSTATEUNSUPPORTED,
}

impl AsRef<str> for SkaffoldSupportedConditionSkaffoldSupportStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATEUNSPECIFIED => "SKAFFOLD_SUPPORT_STATE_UNSPECIFIED",
            SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATESUPPORTED => "SKAFFOLD_SUPPORT_STATE_SUPPORTED",
            SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATEMAINTENANCEMODE => "SKAFFOLD_SUPPORT_STATE_MAINTENANCE_MODE",
            SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATEUNSUPPORTED => "SKAFFOLD_SUPPORT_STATE_UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for SkaffoldSupportedConditionSkaffoldSupportStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SKAFFOLD_SUPPORT_STATE_UNSPECIFIED" => Ok(SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATEUNSPECIFIED),
           "SKAFFOLD_SUPPORT_STATE_SUPPORTED" => Ok(SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATESUPPORTED),
           "SKAFFOLD_SUPPORT_STATE_MAINTENANCE_MODE" => Ok(SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATEMAINTENANCEMODE),
           "SKAFFOLD_SUPPORT_STATE_UNSUPPORTED" => Ok(SkaffoldSupportedConditionSkaffoldSupportStateEnum::SKAFFOLDSUPPORTSTATEUNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SkaffoldSupportedConditionSkaffoldSupportStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetRenderFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Reason this render failed. This will always be unspecified while the render in progress.
pub enum TargetRenderFailureCauseEnum {
    

    /// No reason for failure is specified.
    ///
    /// "FAILURE_CAUSE_UNSPECIFIED"
    #[serde(rename="FAILURE_CAUSE_UNSPECIFIED")]
    FAILURECAUSEUNSPECIFIED,
    

    /// Cloud Build is not available, either because it is not enabled or because Cloud Deploy has insufficient permissions. See [required permission](https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions).
    ///
    /// "CLOUD_BUILD_UNAVAILABLE"
    #[serde(rename="CLOUD_BUILD_UNAVAILABLE")]
    CLOUDBUILDUNAVAILABLE,
    

    /// The render operation did not complete successfully; check Cloud Build logs.
    ///
    /// "EXECUTION_FAILED"
    #[serde(rename="EXECUTION_FAILED")]
    EXECUTIONFAILED,
    

    /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message for additional details.
    ///
    /// "CLOUD_BUILD_REQUEST_FAILED"
    #[serde(rename="CLOUD_BUILD_REQUEST_FAILED")]
    CLOUDBUILDREQUESTFAILED,
    

    /// The render operation did not complete successfully because the verification stanza required for verify was not found on the Skaffold configuration.
    ///
    /// "VERIFICATION_CONFIG_NOT_FOUND"
    #[serde(rename="VERIFICATION_CONFIG_NOT_FOUND")]
    VERIFICATIONCONFIGNOTFOUND,
    

    /// The render operation did not complete successfully because the custom action required for predeploy or postdeploy was not found in the Skaffold configuration. See failure_message for additional details.
    ///
    /// "CUSTOM_ACTION_NOT_FOUND"
    #[serde(rename="CUSTOM_ACTION_NOT_FOUND")]
    CUSTOMACTIONNOTFOUND,
    

    /// Release failed during rendering because the release configuration is not supported with the specified deployment strategy.
    ///
    /// "DEPLOYMENT_STRATEGY_NOT_SUPPORTED"
    #[serde(rename="DEPLOYMENT_STRATEGY_NOT_SUPPORTED")]
    DEPLOYMENTSTRATEGYNOTSUPPORTED,
    

    /// The render operation had a feature configured that is not supported.
    ///
    /// "RENDER_FEATURE_NOT_SUPPORTED"
    #[serde(rename="RENDER_FEATURE_NOT_SUPPORTED")]
    RENDERFEATURENOTSUPPORTED,
}

impl AsRef<str> for TargetRenderFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetRenderFailureCauseEnum::FAILURECAUSEUNSPECIFIED => "FAILURE_CAUSE_UNSPECIFIED",
            TargetRenderFailureCauseEnum::CLOUDBUILDUNAVAILABLE => "CLOUD_BUILD_UNAVAILABLE",
            TargetRenderFailureCauseEnum::EXECUTIONFAILED => "EXECUTION_FAILED",
            TargetRenderFailureCauseEnum::CLOUDBUILDREQUESTFAILED => "CLOUD_BUILD_REQUEST_FAILED",
            TargetRenderFailureCauseEnum::VERIFICATIONCONFIGNOTFOUND => "VERIFICATION_CONFIG_NOT_FOUND",
            TargetRenderFailureCauseEnum::CUSTOMACTIONNOTFOUND => "CUSTOM_ACTION_NOT_FOUND",
            TargetRenderFailureCauseEnum::DEPLOYMENTSTRATEGYNOTSUPPORTED => "DEPLOYMENT_STRATEGY_NOT_SUPPORTED",
            TargetRenderFailureCauseEnum::RENDERFEATURENOTSUPPORTED => "RENDER_FEATURE_NOT_SUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetRenderFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_CAUSE_UNSPECIFIED" => Ok(TargetRenderFailureCauseEnum::FAILURECAUSEUNSPECIFIED),
           "CLOUD_BUILD_UNAVAILABLE" => Ok(TargetRenderFailureCauseEnum::CLOUDBUILDUNAVAILABLE),
           "EXECUTION_FAILED" => Ok(TargetRenderFailureCauseEnum::EXECUTIONFAILED),
           "CLOUD_BUILD_REQUEST_FAILED" => Ok(TargetRenderFailureCauseEnum::CLOUDBUILDREQUESTFAILED),
           "VERIFICATION_CONFIG_NOT_FOUND" => Ok(TargetRenderFailureCauseEnum::VERIFICATIONCONFIGNOTFOUND),
           "CUSTOM_ACTION_NOT_FOUND" => Ok(TargetRenderFailureCauseEnum::CUSTOMACTIONNOTFOUND),
           "DEPLOYMENT_STRATEGY_NOT_SUPPORTED" => Ok(TargetRenderFailureCauseEnum::DEPLOYMENTSTRATEGYNOTSUPPORTED),
           "RENDER_FEATURE_NOT_SUPPORTED" => Ok(TargetRenderFailureCauseEnum::RENDERFEATURENOTSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetRenderFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetRenderRenderingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the render operation for this Target.
pub enum TargetRenderRenderingStateEnum {
    

    /// The render operation state is unspecified.
    ///
    /// "TARGET_RENDER_STATE_UNSPECIFIED"
    #[serde(rename="TARGET_RENDER_STATE_UNSPECIFIED")]
    TARGETRENDERSTATEUNSPECIFIED,
    

    /// The render operation has completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The render operation has failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The render operation is in progress.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
}

impl AsRef<str> for TargetRenderRenderingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetRenderRenderingStateEnum::TARGETRENDERSTATEUNSPECIFIED => "TARGET_RENDER_STATE_UNSPECIFIED",
            TargetRenderRenderingStateEnum::SUCCEEDED => "SUCCEEDED",
            TargetRenderRenderingStateEnum::FAILED => "FAILED",
            TargetRenderRenderingStateEnum::INPROGRESS => "IN_PROGRESS",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetRenderRenderingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGET_RENDER_STATE_UNSPECIFIED" => Ok(TargetRenderRenderingStateEnum::TARGETRENDERSTATEUNSPECIFIED),
           "SUCCEEDED" => Ok(TargetRenderRenderingStateEnum::SUCCEEDED),
           "FAILED" => Ok(TargetRenderRenderingStateEnum::FAILED),
           "IN_PROGRESS" => Ok(TargetRenderRenderingStateEnum::INPROGRESS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetRenderRenderingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VerifyJobRunFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason the verify failed. This will always be unspecified while the verify is in progress or if it succeeded.
pub enum VerifyJobRunFailureCauseEnum {
    

    /// No reason for failure is specified.
    ///
    /// "FAILURE_CAUSE_UNSPECIFIED"
    #[serde(rename="FAILURE_CAUSE_UNSPECIFIED")]
    FAILURECAUSEUNSPECIFIED,
    

    /// Cloud Build is not available, either because it is not enabled or because Cloud Deploy has insufficient permissions. See [required permission](https://cloud.google.com/deploy/docs/cloud-deploy-service-account#required_permissions).
    ///
    /// "CLOUD_BUILD_UNAVAILABLE"
    #[serde(rename="CLOUD_BUILD_UNAVAILABLE")]
    CLOUDBUILDUNAVAILABLE,
    

    /// The verify operation did not complete successfully; check Cloud Build logs.
    ///
    /// "EXECUTION_FAILED"
    #[serde(rename="EXECUTION_FAILED")]
    EXECUTIONFAILED,
    

    /// The verify job run did not complete within the alloted time.
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// No Skaffold verify configuration was found.
    ///
    /// "VERIFICATION_CONFIG_NOT_FOUND"
    #[serde(rename="VERIFICATION_CONFIG_NOT_FOUND")]
    VERIFICATIONCONFIGNOTFOUND,
    

    /// Cloud Build failed to fulfill Cloud Deploy's request. See failure_message for additional details.
    ///
    /// "CLOUD_BUILD_REQUEST_FAILED"
    #[serde(rename="CLOUD_BUILD_REQUEST_FAILED")]
    CLOUDBUILDREQUESTFAILED,
}

impl AsRef<str> for VerifyJobRunFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VerifyJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED => "FAILURE_CAUSE_UNSPECIFIED",
            VerifyJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE => "CLOUD_BUILD_UNAVAILABLE",
            VerifyJobRunFailureCauseEnum::EXECUTIONFAILED => "EXECUTION_FAILED",
            VerifyJobRunFailureCauseEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            VerifyJobRunFailureCauseEnum::VERIFICATIONCONFIGNOTFOUND => "VERIFICATION_CONFIG_NOT_FOUND",
            VerifyJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED => "CLOUD_BUILD_REQUEST_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for VerifyJobRunFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_CAUSE_UNSPECIFIED" => Ok(VerifyJobRunFailureCauseEnum::FAILURECAUSEUNSPECIFIED),
           "CLOUD_BUILD_UNAVAILABLE" => Ok(VerifyJobRunFailureCauseEnum::CLOUDBUILDUNAVAILABLE),
           "EXECUTION_FAILED" => Ok(VerifyJobRunFailureCauseEnum::EXECUTIONFAILED),
           "DEADLINE_EXCEEDED" => Ok(VerifyJobRunFailureCauseEnum::DEADLINEEXCEEDED),
           "VERIFICATION_CONFIG_NOT_FOUND" => Ok(VerifyJobRunFailureCauseEnum::VERIFICATIONCONFIGNOTFOUND),
           "CLOUD_BUILD_REQUEST_FAILED" => Ok(VerifyJobRunFailureCauseEnum::CLOUDBUILDREQUESTFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VerifyJobRunFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


