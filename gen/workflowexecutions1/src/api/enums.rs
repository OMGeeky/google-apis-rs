use super::*;



// region ExecutionCallLogLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The call logging level associated to this execution.
pub enum ExecutionCallLogLevelEnum {
    

    /// No call logging level specified.
    ///
    /// "CALL_LOG_LEVEL_UNSPECIFIED"
    #[serde(rename="CALL_LOG_LEVEL_UNSPECIFIED")]
    CALLLOGLEVELUNSPECIFIED,
    

    /// Log all call steps within workflows, all call returns, and all exceptions raised.
    ///
    /// "LOG_ALL_CALLS"
    #[serde(rename="LOG_ALL_CALLS")]
    LOGALLCALLS,
    

    /// Log only exceptions that are raised from call steps within workflows.
    ///
    /// "LOG_ERRORS_ONLY"
    #[serde(rename="LOG_ERRORS_ONLY")]
    LOGERRORSONLY,
    

    /// Explicitly log nothing.
    ///
    /// "LOG_NONE"
    #[serde(rename="LOG_NONE")]
    LOGNONE,
}

impl AsRef<str> for ExecutionCallLogLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecutionCallLogLevelEnum::CALLLOGLEVELUNSPECIFIED => "CALL_LOG_LEVEL_UNSPECIFIED",
            ExecutionCallLogLevelEnum::LOGALLCALLS => "LOG_ALL_CALLS",
            ExecutionCallLogLevelEnum::LOGERRORSONLY => "LOG_ERRORS_ONLY",
            ExecutionCallLogLevelEnum::LOGNONE => "LOG_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecutionCallLogLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CALL_LOG_LEVEL_UNSPECIFIED" => Ok(ExecutionCallLogLevelEnum::CALLLOGLEVELUNSPECIFIED),
           "LOG_ALL_CALLS" => Ok(ExecutionCallLogLevelEnum::LOGALLCALLS),
           "LOG_ERRORS_ONLY" => Ok(ExecutionCallLogLevelEnum::LOGERRORSONLY),
           "LOG_NONE" => Ok(ExecutionCallLogLevelEnum::LOGNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecutionCallLogLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExecutionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the execution.
pub enum ExecutionStateEnum {
    

    /// Invalid state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The execution is in progress.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The execution finished successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The execution failed with an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The execution was stopped intentionally.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Execution data is unavailable. See the `state_error` field.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// Request has been placed in the backlog for processing at a later time.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
}

impl AsRef<str> for ExecutionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecutionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ExecutionStateEnum::ACTIVE => "ACTIVE",
            ExecutionStateEnum::SUCCEEDED => "SUCCEEDED",
            ExecutionStateEnum::FAILED => "FAILED",
            ExecutionStateEnum::CANCELLED => "CANCELLED",
            ExecutionStateEnum::UNAVAILABLE => "UNAVAILABLE",
            ExecutionStateEnum::QUEUED => "QUEUED",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecutionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ExecutionStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ExecutionStateEnum::ACTIVE),
           "SUCCEEDED" => Ok(ExecutionStateEnum::SUCCEEDED),
           "FAILED" => Ok(ExecutionStateEnum::FAILED),
           "CANCELLED" => Ok(ExecutionStateEnum::CANCELLED),
           "UNAVAILABLE" => Ok(ExecutionStateEnum::UNAVAILABLE),
           "QUEUED" => Ok(ExecutionStateEnum::QUEUED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecutionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StateErrorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this state error.
pub enum StateErrorTypeEnum {
    

    /// No type specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Caused by an issue with KMS.
    ///
    /// "KMS_ERROR"
    #[serde(rename="KMS_ERROR")]
    KMSERROR,
}

impl AsRef<str> for StateErrorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StateErrorTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            StateErrorTypeEnum::KMSERROR => "KMS_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for StateErrorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(StateErrorTypeEnum::TYPEUNSPECIFIED),
           "KMS_ERROR" => Ok(StateErrorTypeEnum::KMSERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StateErrorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StepEntryStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the step entry.
pub enum StepEntryStateEnum {
    

    /// Invalid state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The step entry is in progress.
    ///
    /// "STATE_IN_PROGRESS"
    #[serde(rename="STATE_IN_PROGRESS")]
    STATEINPROGRESS,
    

    /// The step entry finished successfully.
    ///
    /// "STATE_SUCCEEDED"
    #[serde(rename="STATE_SUCCEEDED")]
    STATESUCCEEDED,
    

    /// The step entry failed with an error.
    ///
    /// "STATE_FAILED"
    #[serde(rename="STATE_FAILED")]
    STATEFAILED,
}

impl AsRef<str> for StepEntryStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StepEntryStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            StepEntryStateEnum::STATEINPROGRESS => "STATE_IN_PROGRESS",
            StepEntryStateEnum::STATESUCCEEDED => "STATE_SUCCEEDED",
            StepEntryStateEnum::STATEFAILED => "STATE_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for StepEntryStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(StepEntryStateEnum::STATEUNSPECIFIED),
           "STATE_IN_PROGRESS" => Ok(StepEntryStateEnum::STATEINPROGRESS),
           "STATE_SUCCEEDED" => Ok(StepEntryStateEnum::STATESUCCEEDED),
           "STATE_FAILED" => Ok(StepEntryStateEnum::STATEFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StepEntryStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StepEntryStepTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the step this step entry belongs to.
pub enum StepEntryStepTypeEnum {
    

    /// Invalid step type.
    ///
    /// "STEP_TYPE_UNSPECIFIED"
    #[serde(rename="STEP_TYPE_UNSPECIFIED")]
    STEPTYPEUNSPECIFIED,
    

    /// The step entry assigns new variable(s).
    ///
    /// "STEP_ASSIGN"
    #[serde(rename="STEP_ASSIGN")]
    STEPASSIGN,
    

    /// The step entry calls a standard library routine.
    ///
    /// "STEP_STD_LIB_CALL"
    #[serde(rename="STEP_STD_LIB_CALL")]
    STEPSTDLIBCALL,
    

    /// The step entry calls a connector.
    ///
    /// "STEP_CONNECTOR_CALL"
    #[serde(rename="STEP_CONNECTOR_CALL")]
    STEPCONNECTORCALL,
    

    /// The step entry calls a subworklfow.
    ///
    /// "STEP_SUBWORKFLOW_CALL"
    #[serde(rename="STEP_SUBWORKFLOW_CALL")]
    STEPSUBWORKFLOWCALL,
    

    /// The step entry calls a subworkflow/stdlib.
    ///
    /// "STEP_CALL"
    #[serde(rename="STEP_CALL")]
    STEPCALL,
    

    /// The step entry executes a switch-case block.
    ///
    /// "STEP_SWITCH"
    #[serde(rename="STEP_SWITCH")]
    STEPSWITCH,
    

    /// The step entry executes a condition inside a switch.
    ///
    /// "STEP_CONDITION"
    #[serde(rename="STEP_CONDITION")]
    STEPCONDITION,
    

    /// The step entry executes a for loop.
    ///
    /// "STEP_FOR"
    #[serde(rename="STEP_FOR")]
    STEPFOR,
    

    /// The step entry executes a iteration of a for loop.
    ///
    /// "STEP_FOR_ITERATION"
    #[serde(rename="STEP_FOR_ITERATION")]
    STEPFORITERATION,
    

    /// The step entry executes a parallel for loop.
    ///
    /// "STEP_PARALLEL_FOR"
    #[serde(rename="STEP_PARALLEL_FOR")]
    STEPPARALLELFOR,
    

    /// The step entry executes a series of parallel branch(es).
    ///
    /// "STEP_PARALLEL_BRANCH"
    #[serde(rename="STEP_PARALLEL_BRANCH")]
    STEPPARALLELBRANCH,
    

    /// The step entry executes a branch of a parallel branch.
    ///
    /// "STEP_PARALLEL_BRANCH_ENTRY"
    #[serde(rename="STEP_PARALLEL_BRANCH_ENTRY")]
    STEPPARALLELBRANCHENTRY,
    

    /// The step entry executes a try/retry/except block.
    ///
    /// "STEP_TRY_RETRY_EXCEPT"
    #[serde(rename="STEP_TRY_RETRY_EXCEPT")]
    STEPTRYRETRYEXCEPT,
    

    /// The step entry executes the try part of a try/retry/except block.
    ///
    /// "STEP_TRY"
    #[serde(rename="STEP_TRY")]
    STEPTRY,
    

    /// The step entry executes the retry part of a try/retry/except block.
    ///
    /// "STEP_RETRY"
    #[serde(rename="STEP_RETRY")]
    STEPRETRY,
    

    /// The step entry executes the except part of a try/retry/except block.
    ///
    /// "STEP_EXCEPT"
    #[serde(rename="STEP_EXCEPT")]
    STEPEXCEPT,
    

    /// The step entry returns.
    ///
    /// "STEP_RETURN"
    #[serde(rename="STEP_RETURN")]
    STEPRETURN,
    

    /// The step entry raises an error.
    ///
    /// "STEP_RAISE"
    #[serde(rename="STEP_RAISE")]
    STEPRAISE,
    

    /// The step entry jumps to another step.
    ///
    /// "STEP_GOTO"
    #[serde(rename="STEP_GOTO")]
    STEPGOTO,
}

impl AsRef<str> for StepEntryStepTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StepEntryStepTypeEnum::STEPTYPEUNSPECIFIED => "STEP_TYPE_UNSPECIFIED",
            StepEntryStepTypeEnum::STEPASSIGN => "STEP_ASSIGN",
            StepEntryStepTypeEnum::STEPSTDLIBCALL => "STEP_STD_LIB_CALL",
            StepEntryStepTypeEnum::STEPCONNECTORCALL => "STEP_CONNECTOR_CALL",
            StepEntryStepTypeEnum::STEPSUBWORKFLOWCALL => "STEP_SUBWORKFLOW_CALL",
            StepEntryStepTypeEnum::STEPCALL => "STEP_CALL",
            StepEntryStepTypeEnum::STEPSWITCH => "STEP_SWITCH",
            StepEntryStepTypeEnum::STEPCONDITION => "STEP_CONDITION",
            StepEntryStepTypeEnum::STEPFOR => "STEP_FOR",
            StepEntryStepTypeEnum::STEPFORITERATION => "STEP_FOR_ITERATION",
            StepEntryStepTypeEnum::STEPPARALLELFOR => "STEP_PARALLEL_FOR",
            StepEntryStepTypeEnum::STEPPARALLELBRANCH => "STEP_PARALLEL_BRANCH",
            StepEntryStepTypeEnum::STEPPARALLELBRANCHENTRY => "STEP_PARALLEL_BRANCH_ENTRY",
            StepEntryStepTypeEnum::STEPTRYRETRYEXCEPT => "STEP_TRY_RETRY_EXCEPT",
            StepEntryStepTypeEnum::STEPTRY => "STEP_TRY",
            StepEntryStepTypeEnum::STEPRETRY => "STEP_RETRY",
            StepEntryStepTypeEnum::STEPEXCEPT => "STEP_EXCEPT",
            StepEntryStepTypeEnum::STEPRETURN => "STEP_RETURN",
            StepEntryStepTypeEnum::STEPRAISE => "STEP_RAISE",
            StepEntryStepTypeEnum::STEPGOTO => "STEP_GOTO",
        }
    }
}

impl std::convert::TryFrom< &str> for StepEntryStepTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STEP_TYPE_UNSPECIFIED" => Ok(StepEntryStepTypeEnum::STEPTYPEUNSPECIFIED),
           "STEP_ASSIGN" => Ok(StepEntryStepTypeEnum::STEPASSIGN),
           "STEP_STD_LIB_CALL" => Ok(StepEntryStepTypeEnum::STEPSTDLIBCALL),
           "STEP_CONNECTOR_CALL" => Ok(StepEntryStepTypeEnum::STEPCONNECTORCALL),
           "STEP_SUBWORKFLOW_CALL" => Ok(StepEntryStepTypeEnum::STEPSUBWORKFLOWCALL),
           "STEP_CALL" => Ok(StepEntryStepTypeEnum::STEPCALL),
           "STEP_SWITCH" => Ok(StepEntryStepTypeEnum::STEPSWITCH),
           "STEP_CONDITION" => Ok(StepEntryStepTypeEnum::STEPCONDITION),
           "STEP_FOR" => Ok(StepEntryStepTypeEnum::STEPFOR),
           "STEP_FOR_ITERATION" => Ok(StepEntryStepTypeEnum::STEPFORITERATION),
           "STEP_PARALLEL_FOR" => Ok(StepEntryStepTypeEnum::STEPPARALLELFOR),
           "STEP_PARALLEL_BRANCH" => Ok(StepEntryStepTypeEnum::STEPPARALLELBRANCH),
           "STEP_PARALLEL_BRANCH_ENTRY" => Ok(StepEntryStepTypeEnum::STEPPARALLELBRANCHENTRY),
           "STEP_TRY_RETRY_EXCEPT" => Ok(StepEntryStepTypeEnum::STEPTRYRETRYEXCEPT),
           "STEP_TRY" => Ok(StepEntryStepTypeEnum::STEPTRY),
           "STEP_RETRY" => Ok(StepEntryStepTypeEnum::STEPRETRY),
           "STEP_EXCEPT" => Ok(StepEntryStepTypeEnum::STEPEXCEPT),
           "STEP_RETURN" => Ok(StepEntryStepTypeEnum::STEPRETURN),
           "STEP_RAISE" => Ok(StepEntryStepTypeEnum::STEPRAISE),
           "STEP_GOTO" => Ok(StepEntryStepTypeEnum::STEPGOTO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StepEntryStepTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StepEntryMetadataProgressTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Progress type of this step entry.
pub enum StepEntryMetadataProgressTypeEnum {
    

    /// Current step entry does not have any progress data.
    ///
    /// "PROGRESS_TYPE_UNSPECIFIED"
    #[serde(rename="PROGRESS_TYPE_UNSPECIFIED")]
    PROGRESSTYPEUNSPECIFIED,
    

    /// Current step entry is in progress of a FOR step.
    ///
    /// "PROGRESS_TYPE_FOR"
    #[serde(rename="PROGRESS_TYPE_FOR")]
    PROGRESSTYPEFOR,
    

    /// Current step entry is in progress of a SWITCH step.
    ///
    /// "PROGRESS_TYPE_SWITCH"
    #[serde(rename="PROGRESS_TYPE_SWITCH")]
    PROGRESSTYPESWITCH,
    

    /// Current step entry is in progress of a RETRY step.
    ///
    /// "PROGRESS_TYPE_RETRY"
    #[serde(rename="PROGRESS_TYPE_RETRY")]
    PROGRESSTYPERETRY,
    

    /// Current step entry is in progress of a PARALLEL FOR step.
    ///
    /// "PROGRESS_TYPE_PARALLEL_FOR"
    #[serde(rename="PROGRESS_TYPE_PARALLEL_FOR")]
    PROGRESSTYPEPARALLELFOR,
    

    /// Current step entry is in progress of a PARALLEL BRANCH step.
    ///
    /// "PROGRESS_TYPE_PARALLEL_BRANCH"
    #[serde(rename="PROGRESS_TYPE_PARALLEL_BRANCH")]
    PROGRESSTYPEPARALLELBRANCH,
}

impl AsRef<str> for StepEntryMetadataProgressTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StepEntryMetadataProgressTypeEnum::PROGRESSTYPEUNSPECIFIED => "PROGRESS_TYPE_UNSPECIFIED",
            StepEntryMetadataProgressTypeEnum::PROGRESSTYPEFOR => "PROGRESS_TYPE_FOR",
            StepEntryMetadataProgressTypeEnum::PROGRESSTYPESWITCH => "PROGRESS_TYPE_SWITCH",
            StepEntryMetadataProgressTypeEnum::PROGRESSTYPERETRY => "PROGRESS_TYPE_RETRY",
            StepEntryMetadataProgressTypeEnum::PROGRESSTYPEPARALLELFOR => "PROGRESS_TYPE_PARALLEL_FOR",
            StepEntryMetadataProgressTypeEnum::PROGRESSTYPEPARALLELBRANCH => "PROGRESS_TYPE_PARALLEL_BRANCH",
        }
    }
}

impl std::convert::TryFrom< &str> for StepEntryMetadataProgressTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROGRESS_TYPE_UNSPECIFIED" => Ok(StepEntryMetadataProgressTypeEnum::PROGRESSTYPEUNSPECIFIED),
           "PROGRESS_TYPE_FOR" => Ok(StepEntryMetadataProgressTypeEnum::PROGRESSTYPEFOR),
           "PROGRESS_TYPE_SWITCH" => Ok(StepEntryMetadataProgressTypeEnum::PROGRESSTYPESWITCH),
           "PROGRESS_TYPE_RETRY" => Ok(StepEntryMetadataProgressTypeEnum::PROGRESSTYPERETRY),
           "PROGRESS_TYPE_PARALLEL_FOR" => Ok(StepEntryMetadataProgressTypeEnum::PROGRESSTYPEPARALLELFOR),
           "PROGRESS_TYPE_PARALLEL_BRANCH" => Ok(StepEntryMetadataProgressTypeEnum::PROGRESSTYPEPARALLELBRANCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StepEntryMetadataProgressTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. A view defining which fields should be filled in the returned executions. The API will default to the BASIC view.
pub enum ProjectViewEnum {
    

    /// The default / unset value.
    ///
    /// "EXECUTION_VIEW_UNSPECIFIED"
    #[serde(rename="EXECUTION_VIEW_UNSPECIFIED")]
    EXECUTIONVIEWUNSPECIFIED,
    

    /// Includes only basic metadata about the execution. The following fields are returned: name, start_time, end_time, duration, state, and workflow_revision_id.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Includes all data.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::EXECUTIONVIEWUNSPECIFIED => "EXECUTION_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EXECUTION_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::EXECUTIONVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
           "FULL" => Ok(ProjectViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


