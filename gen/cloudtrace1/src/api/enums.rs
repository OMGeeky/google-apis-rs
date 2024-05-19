use super::*;



// region TraceSpanKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `RPC_CLIENT` and `RPC_SERVER` to identify queueing latency associated with the span.
pub enum TraceSpanKindEnum {
    

    /// Unspecified.
    ///
    /// "SPAN_KIND_UNSPECIFIED"
    #[serde(rename="SPAN_KIND_UNSPECIFIED")]
    SPANKINDUNSPECIFIED,
    

    /// Indicates that the span covers server-side handling of an RPC or other remote network request.
    ///
    /// "RPC_SERVER"
    #[serde(rename="RPC_SERVER")]
    RPCSERVER,
    

    /// Indicates that the span covers the client-side wrapper around an RPC or other remote request.
    ///
    /// "RPC_CLIENT"
    #[serde(rename="RPC_CLIENT")]
    RPCCLIENT,
}

impl AsRef<str> for TraceSpanKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TraceSpanKindEnum::SPANKINDUNSPECIFIED => "SPAN_KIND_UNSPECIFIED",
            TraceSpanKindEnum::RPCSERVER => "RPC_SERVER",
            TraceSpanKindEnum::RPCCLIENT => "RPC_CLIENT",
        }
    }
}

impl std::convert::TryFrom< &str> for TraceSpanKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPAN_KIND_UNSPECIFIED" => Ok(TraceSpanKindEnum::SPANKINDUNSPECIFIED),
           "RPC_SERVER" => Ok(TraceSpanKindEnum::RPCSERVER),
           "RPC_CLIENT" => Ok(TraceSpanKindEnum::RPCCLIENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TraceSpanKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of data returned for traces in the list. Default is `MINIMAL`.
pub enum ProjectViewEnum {
    

    /// Default is `MINIMAL` if unspecified.
    ///
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    

    /// Minimal view of the trace record that contains only the project and trace IDs.
    ///
    /// "MINIMAL"
    #[serde(rename="MINIMAL")]
    MINIMAL,
    

    /// Root span view of the trace record that returns the root spans along with the minimal trace data.
    ///
    /// "ROOTSPAN"
    #[serde(rename="ROOTSPAN")]
    ROOTSPAN,
    

    /// Complete view of the trace record that contains the actual trace data. This is equivalent to calling the REST `get` or RPC `GetTrace` method using the ID of each listed trace.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            ProjectViewEnum::MINIMAL => "MINIMAL",
            ProjectViewEnum::ROOTSPAN => "ROOTSPAN",
            ProjectViewEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(ProjectViewEnum::VIEWTYPEUNSPECIFIED),
           "MINIMAL" => Ok(ProjectViewEnum::MINIMAL),
           "ROOTSPAN" => Ok(ProjectViewEnum::ROOTSPAN),
           "COMPLETE" => Ok(ProjectViewEnum::COMPLETE),
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


