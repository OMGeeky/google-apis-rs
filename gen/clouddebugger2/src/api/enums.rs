use super::*;



// region AliasContextKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The alias kind.
pub enum AliasContextKindEnum {
    

    /// Do not use.
    ///
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
    

    /// Git tag
    ///
    /// "FIXED"
    #[serde(rename="FIXED")]
    FIXED,
    

    /// Git branch
    ///
    /// "MOVABLE"
    #[serde(rename="MOVABLE")]
    MOVABLE,
    

    /// OTHER is used to specify non-standard aliases, those not of the kinds above. For example, if a Git repo has a ref named "refs/foo/bar", it is considered to be of kind OTHER.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
}

impl AsRef<str> for AliasContextKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AliasContextKindEnum::ANY => "ANY",
            AliasContextKindEnum::FIXED => "FIXED",
            AliasContextKindEnum::MOVABLE => "MOVABLE",
            AliasContextKindEnum::OTHER => "OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for AliasContextKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANY" => Ok(AliasContextKindEnum::ANY),
           "FIXED" => Ok(AliasContextKindEnum::FIXED),
           "MOVABLE" => Ok(AliasContextKindEnum::MOVABLE),
           "OTHER" => Ok(AliasContextKindEnum::OTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AliasContextKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BreakpointActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Action that the agent should perform when the code at the breakpoint location is hit.
pub enum BreakpointActionEnum {
    

    /// Capture stack frame and variables and update the breakpoint. The data is only captured once. After that the breakpoint is set in a final state.
    ///
    /// "CAPTURE"
    #[serde(rename="CAPTURE")]
    CAPTURE,
    

    /// Log each breakpoint hit. The breakpoint remains active until deleted or expired.
    ///
    /// "LOG"
    #[serde(rename="LOG")]
    LOG,
}

impl AsRef<str> for BreakpointActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BreakpointActionEnum::CAPTURE => "CAPTURE",
            BreakpointActionEnum::LOG => "LOG",
        }
    }
}

impl std::convert::TryFrom< &str> for BreakpointActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAPTURE" => Ok(BreakpointActionEnum::CAPTURE),
           "LOG" => Ok(BreakpointActionEnum::LOG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BreakpointActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BreakpointLogLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the severity of the log. Only relevant when action is `LOG`.
pub enum BreakpointLogLevelEnum {
    

    /// Information log message.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Warning log message.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Error log message.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for BreakpointLogLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BreakpointLogLevelEnum::INFO => "INFO",
            BreakpointLogLevelEnum::WARNING => "WARNING",
            BreakpointLogLevelEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for BreakpointLogLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INFO" => Ok(BreakpointLogLevelEnum::INFO),
           "WARNING" => Ok(BreakpointLogLevelEnum::WARNING),
           "ERROR" => Ok(BreakpointLogLevelEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BreakpointLogLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BreakpointStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the breakpoint.
pub enum BreakpointStateEnum {
    

    /// Breakpoint state UNSPECIFIED.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Enabling canary but no agents are available.
    ///
    /// "STATE_CANARY_PENDING_AGENTS"
    #[serde(rename="STATE_CANARY_PENDING_AGENTS")]
    STATECANARYPENDINGAGENTS,
    

    /// Enabling canary and successfully assigning canary agents.
    ///
    /// "STATE_CANARY_ACTIVE"
    #[serde(rename="STATE_CANARY_ACTIVE")]
    STATECANARYACTIVE,
    

    /// Breakpoint rolling out to all agents.
    ///
    /// "STATE_ROLLING_TO_ALL"
    #[serde(rename="STATE_ROLLING_TO_ALL")]
    STATEROLLINGTOALL,
    

    /// Breakpoint is hit/complete/failed.
    ///
    /// "STATE_IS_FINAL"
    #[serde(rename="STATE_IS_FINAL")]
    STATEISFINAL,
}

impl AsRef<str> for BreakpointStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BreakpointStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BreakpointStateEnum::STATECANARYPENDINGAGENTS => "STATE_CANARY_PENDING_AGENTS",
            BreakpointStateEnum::STATECANARYACTIVE => "STATE_CANARY_ACTIVE",
            BreakpointStateEnum::STATEROLLINGTOALL => "STATE_ROLLING_TO_ALL",
            BreakpointStateEnum::STATEISFINAL => "STATE_IS_FINAL",
        }
    }
}

impl std::convert::TryFrom< &str> for BreakpointStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BreakpointStateEnum::STATEUNSPECIFIED),
           "STATE_CANARY_PENDING_AGENTS" => Ok(BreakpointStateEnum::STATECANARYPENDINGAGENTS),
           "STATE_CANARY_ACTIVE" => Ok(BreakpointStateEnum::STATECANARYACTIVE),
           "STATE_ROLLING_TO_ALL" => Ok(BreakpointStateEnum::STATEROLLINGTOALL),
           "STATE_IS_FINAL" => Ok(BreakpointStateEnum::STATEISFINAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BreakpointStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DebuggeeCanaryModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Used when setting breakpoint canary for this debuggee.
pub enum DebuggeeCanaryModeEnum {
    

    /// CANARY_MODE_UNSPECIFIED is equivalent to CANARY_MODE_ALWAYS_DISABLED so that if the debuggee is not configured to use the canary feature, the feature will be disabled.
    ///
    /// "CANARY_MODE_UNSPECIFIED"
    #[serde(rename="CANARY_MODE_UNSPECIFIED")]
    CANARYMODEUNSPECIFIED,
    

    /// Always enable breakpoint canary regardless of the value of breakpoint's canary option.
    ///
    /// "CANARY_MODE_ALWAYS_ENABLED"
    #[serde(rename="CANARY_MODE_ALWAYS_ENABLED")]
    CANARYMODEALWAYSENABLED,
    

    /// Always disable breakpoint canary regardless of the value of breakpoint's canary option.
    ///
    /// "CANARY_MODE_ALWAYS_DISABLED"
    #[serde(rename="CANARY_MODE_ALWAYS_DISABLED")]
    CANARYMODEALWAYSDISABLED,
    

    /// Depends on the breakpoint's canary option. Enable canary by default if the breakpoint's canary option is not specified.
    ///
    /// "CANARY_MODE_DEFAULT_ENABLED"
    #[serde(rename="CANARY_MODE_DEFAULT_ENABLED")]
    CANARYMODEDEFAULTENABLED,
    

    /// Depends on the breakpoint's canary option. Disable canary by default if the breakpoint's canary option is not specified.
    ///
    /// "CANARY_MODE_DEFAULT_DISABLED"
    #[serde(rename="CANARY_MODE_DEFAULT_DISABLED")]
    CANARYMODEDEFAULTDISABLED,
}

impl AsRef<str> for DebuggeeCanaryModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DebuggeeCanaryModeEnum::CANARYMODEUNSPECIFIED => "CANARY_MODE_UNSPECIFIED",
            DebuggeeCanaryModeEnum::CANARYMODEALWAYSENABLED => "CANARY_MODE_ALWAYS_ENABLED",
            DebuggeeCanaryModeEnum::CANARYMODEALWAYSDISABLED => "CANARY_MODE_ALWAYS_DISABLED",
            DebuggeeCanaryModeEnum::CANARYMODEDEFAULTENABLED => "CANARY_MODE_DEFAULT_ENABLED",
            DebuggeeCanaryModeEnum::CANARYMODEDEFAULTDISABLED => "CANARY_MODE_DEFAULT_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for DebuggeeCanaryModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CANARY_MODE_UNSPECIFIED" => Ok(DebuggeeCanaryModeEnum::CANARYMODEUNSPECIFIED),
           "CANARY_MODE_ALWAYS_ENABLED" => Ok(DebuggeeCanaryModeEnum::CANARYMODEALWAYSENABLED),
           "CANARY_MODE_ALWAYS_DISABLED" => Ok(DebuggeeCanaryModeEnum::CANARYMODEALWAYSDISABLED),
           "CANARY_MODE_DEFAULT_ENABLED" => Ok(DebuggeeCanaryModeEnum::CANARYMODEDEFAULTENABLED),
           "CANARY_MODE_DEFAULT_DISABLED" => Ok(DebuggeeCanaryModeEnum::CANARYMODEDEFAULTDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DebuggeeCanaryModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StatusMessageRefersToEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reference to which the message applies.
pub enum StatusMessageRefersToEnum {
    

    /// Status doesn't refer to any particular input.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Status applies to the breakpoint and is related to its location.
    ///
    /// "BREAKPOINT_SOURCE_LOCATION"
    #[serde(rename="BREAKPOINT_SOURCE_LOCATION")]
    BREAKPOINTSOURCELOCATION,
    

    /// Status applies to the breakpoint and is related to its condition.
    ///
    /// "BREAKPOINT_CONDITION"
    #[serde(rename="BREAKPOINT_CONDITION")]
    BREAKPOINTCONDITION,
    

    /// Status applies to the breakpoint and is related to its expressions.
    ///
    /// "BREAKPOINT_EXPRESSION"
    #[serde(rename="BREAKPOINT_EXPRESSION")]
    BREAKPOINTEXPRESSION,
    

    /// Status applies to the breakpoint and is related to its age.
    ///
    /// "BREAKPOINT_AGE"
    #[serde(rename="BREAKPOINT_AGE")]
    BREAKPOINTAGE,
    

    /// Status applies to the breakpoint when the breakpoint failed to exit the canary state.
    ///
    /// "BREAKPOINT_CANARY_FAILED"
    #[serde(rename="BREAKPOINT_CANARY_FAILED")]
    BREAKPOINTCANARYFAILED,
    

    /// Status applies to the entire variable.
    ///
    /// "VARIABLE_NAME"
    #[serde(rename="VARIABLE_NAME")]
    VARIABLENAME,
    

    /// Status applies to variable value (variable name is valid).
    ///
    /// "VARIABLE_VALUE"
    #[serde(rename="VARIABLE_VALUE")]
    VARIABLEVALUE,
}

impl AsRef<str> for StatusMessageRefersToEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StatusMessageRefersToEnum::UNSPECIFIED => "UNSPECIFIED",
            StatusMessageRefersToEnum::BREAKPOINTSOURCELOCATION => "BREAKPOINT_SOURCE_LOCATION",
            StatusMessageRefersToEnum::BREAKPOINTCONDITION => "BREAKPOINT_CONDITION",
            StatusMessageRefersToEnum::BREAKPOINTEXPRESSION => "BREAKPOINT_EXPRESSION",
            StatusMessageRefersToEnum::BREAKPOINTAGE => "BREAKPOINT_AGE",
            StatusMessageRefersToEnum::BREAKPOINTCANARYFAILED => "BREAKPOINT_CANARY_FAILED",
            StatusMessageRefersToEnum::VARIABLENAME => "VARIABLE_NAME",
            StatusMessageRefersToEnum::VARIABLEVALUE => "VARIABLE_VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for StatusMessageRefersToEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(StatusMessageRefersToEnum::UNSPECIFIED),
           "BREAKPOINT_SOURCE_LOCATION" => Ok(StatusMessageRefersToEnum::BREAKPOINTSOURCELOCATION),
           "BREAKPOINT_CONDITION" => Ok(StatusMessageRefersToEnum::BREAKPOINTCONDITION),
           "BREAKPOINT_EXPRESSION" => Ok(StatusMessageRefersToEnum::BREAKPOINTEXPRESSION),
           "BREAKPOINT_AGE" => Ok(StatusMessageRefersToEnum::BREAKPOINTAGE),
           "BREAKPOINT_CANARY_FAILED" => Ok(StatusMessageRefersToEnum::BREAKPOINTCANARYFAILED),
           "VARIABLE_NAME" => Ok(StatusMessageRefersToEnum::VARIABLENAME),
           "VARIABLE_VALUE" => Ok(StatusMessageRefersToEnum::VARIABLEVALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StatusMessageRefersToEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DebuggerActionValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Only breakpoints with the specified action will pass the filter.
pub enum DebuggerActionValueEnum {
    

    /// Capture stack frame and variables and update the breakpoint. The data is only captured once. After that the breakpoint is set in a final state.
    ///
    /// "CAPTURE"
    #[serde(rename="CAPTURE")]
    CAPTURE,
    

    /// Log each breakpoint hit. The breakpoint remains active until deleted or expired.
    ///
    /// "LOG"
    #[serde(rename="LOG")]
    LOG,
}

impl AsRef<str> for DebuggerActionValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DebuggerActionValueEnum::CAPTURE => "CAPTURE",
            DebuggerActionValueEnum::LOG => "LOG",
        }
    }
}

impl std::convert::TryFrom< &str> for DebuggerActionValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CAPTURE" => Ok(DebuggerActionValueEnum::CAPTURE),
           "LOG" => Ok(DebuggerActionValueEnum::LOG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DebuggerActionValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DebuggerCanaryOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The canary option set by the user upon setting breakpoint.
pub enum DebuggerCanaryOptionEnum {
    

    /// Depends on the canary_mode of the debuggee.
    ///
    /// "CANARY_OPTION_UNSPECIFIED"
    #[serde(rename="CANARY_OPTION_UNSPECIFIED")]
    CANARYOPTIONUNSPECIFIED,
    

    /// Enable the canary for this breakpoint if the canary_mode of the debuggee is not CANARY_MODE_ALWAYS_ENABLED or CANARY_MODE_ALWAYS_DISABLED.
    ///
    /// "CANARY_OPTION_TRY_ENABLE"
    #[serde(rename="CANARY_OPTION_TRY_ENABLE")]
    CANARYOPTIONTRYENABLE,
    

    /// Disable the canary for this breakpoint if the canary_mode of the debuggee is not CANARY_MODE_ALWAYS_ENABLED or CANARY_MODE_ALWAYS_DISABLED.
    ///
    /// "CANARY_OPTION_TRY_DISABLE"
    #[serde(rename="CANARY_OPTION_TRY_DISABLE")]
    CANARYOPTIONTRYDISABLE,
}

impl AsRef<str> for DebuggerCanaryOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DebuggerCanaryOptionEnum::CANARYOPTIONUNSPECIFIED => "CANARY_OPTION_UNSPECIFIED",
            DebuggerCanaryOptionEnum::CANARYOPTIONTRYENABLE => "CANARY_OPTION_TRY_ENABLE",
            DebuggerCanaryOptionEnum::CANARYOPTIONTRYDISABLE => "CANARY_OPTION_TRY_DISABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DebuggerCanaryOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CANARY_OPTION_UNSPECIFIED" => Ok(DebuggerCanaryOptionEnum::CANARYOPTIONUNSPECIFIED),
           "CANARY_OPTION_TRY_ENABLE" => Ok(DebuggerCanaryOptionEnum::CANARYOPTIONTRYENABLE),
           "CANARY_OPTION_TRY_DISABLE" => Ok(DebuggerCanaryOptionEnum::CANARYOPTIONTRYDISABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DebuggerCanaryOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


