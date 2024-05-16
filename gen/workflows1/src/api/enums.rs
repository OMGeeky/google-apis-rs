use super::*;



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


// region WorkflowCallLogLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Describes the level of platform logging to apply to calls and call responses during executions of this workflow. If both the workflow and the execution specify a logging level, the execution level takes precedence.
pub enum WorkflowCallLogLevelEnum {
    

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

impl AsRef<str> for WorkflowCallLogLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkflowCallLogLevelEnum::CALLLOGLEVELUNSPECIFIED => "CALL_LOG_LEVEL_UNSPECIFIED",
            WorkflowCallLogLevelEnum::LOGALLCALLS => "LOG_ALL_CALLS",
            WorkflowCallLogLevelEnum::LOGERRORSONLY => "LOG_ERRORS_ONLY",
            WorkflowCallLogLevelEnum::LOGNONE => "LOG_NONE",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkflowCallLogLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CALL_LOG_LEVEL_UNSPECIFIED" => Ok(WorkflowCallLogLevelEnum::CALLLOGLEVELUNSPECIFIED),
           "LOG_ALL_CALLS" => Ok(WorkflowCallLogLevelEnum::LOGALLCALLS),
           "LOG_ERRORS_ONLY" => Ok(WorkflowCallLogLevelEnum::LOGERRORSONLY),
           "LOG_NONE" => Ok(WorkflowCallLogLevelEnum::LOGNONE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkflowCallLogLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkflowStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the workflow deployment.
pub enum WorkflowStateEnum {
    

    /// Invalid state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The workflow has been deployed successfully and is serving.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Workflow data is unavailable. See the `state_error` field.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
}

impl AsRef<str> for WorkflowStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkflowStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            WorkflowStateEnum::ACTIVE => "ACTIVE",
            WorkflowStateEnum::UNAVAILABLE => "UNAVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkflowStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(WorkflowStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(WorkflowStateEnum::ACTIVE),
           "UNAVAILABLE" => Ok(WorkflowStateEnum::UNAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkflowStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


