use super::*;



// region LinkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The relationship of the current span relative to the linked span.
pub enum LinkTypeEnum {
    

    /// The relationship of the two spans is unknown.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The linked span is a child of the current span.
    ///
    /// "CHILD_LINKED_SPAN"
    #[serde(rename="CHILD_LINKED_SPAN")]
    CHILDLINKEDSPAN,
    

    /// The linked span is a parent of the current span.
    ///
    /// "PARENT_LINKED_SPAN"
    #[serde(rename="PARENT_LINKED_SPAN")]
    PARENTLINKEDSPAN,
}

impl AsRef<str> for LinkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            LinkTypeEnum::CHILDLINKEDSPAN => "CHILD_LINKED_SPAN",
            LinkTypeEnum::PARENTLINKEDSPAN => "PARENT_LINKED_SPAN",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(LinkTypeEnum::TYPEUNSPECIFIED),
           "CHILD_LINKED_SPAN" => Ok(LinkTypeEnum::CHILDLINKEDSPAN),
           "PARENT_LINKED_SPAN" => Ok(LinkTypeEnum::PARENTLINKEDSPAN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MessageEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of MessageEvent. Indicates whether the message was sent or received.
pub enum MessageEventTypeEnum {
    

    /// Unknown event type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Indicates a sent message.
    ///
    /// "SENT"
    #[serde(rename="SENT")]
    SENT,
    

    /// Indicates a received message.
    ///
    /// "RECEIVED"
    #[serde(rename="RECEIVED")]
    RECEIVED,
}

impl AsRef<str> for MessageEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MessageEventTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            MessageEventTypeEnum::SENT => "SENT",
            MessageEventTypeEnum::RECEIVED => "RECEIVED",
        }
    }
}

impl std::convert::TryFrom< &str> for MessageEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(MessageEventTypeEnum::TYPEUNSPECIFIED),
           "SENT" => Ok(MessageEventTypeEnum::SENT),
           "RECEIVED" => Ok(MessageEventTypeEnum::RECEIVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MessageEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SpanSpanKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call.
pub enum SpanSpanKindEnum {
    

    /// Unspecified. Do NOT use as default. Implementations MAY assume SpanKind.INTERNAL to be default.
    ///
    /// "SPAN_KIND_UNSPECIFIED"
    #[serde(rename="SPAN_KIND_UNSPECIFIED")]
    SPANKINDUNSPECIFIED,
    

    /// Indicates that the span is used internally. Default value.
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    

    /// Indicates that the span covers server-side handling of an RPC or other remote network request.
    ///
    /// "SERVER"
    #[serde(rename="SERVER")]
    SERVER,
    

    /// Indicates that the span covers the client-side wrapper around an RPC or other remote request.
    ///
    /// "CLIENT"
    #[serde(rename="CLIENT")]
    CLIENT,
    

    /// Indicates that the span describes producer sending a message to a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. publishing a message to a pubsub service).
    ///
    /// "PRODUCER"
    #[serde(rename="PRODUCER")]
    PRODUCER,
    

    /// Indicates that the span describes consumer receiving a message from a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. receiving a message from a pubsub service subscription).
    ///
    /// "CONSUMER"
    #[serde(rename="CONSUMER")]
    CONSUMER,
}

impl AsRef<str> for SpanSpanKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SpanSpanKindEnum::SPANKINDUNSPECIFIED => "SPAN_KIND_UNSPECIFIED",
            SpanSpanKindEnum::INTERNAL => "INTERNAL",
            SpanSpanKindEnum::SERVER => "SERVER",
            SpanSpanKindEnum::CLIENT => "CLIENT",
            SpanSpanKindEnum::PRODUCER => "PRODUCER",
            SpanSpanKindEnum::CONSUMER => "CONSUMER",
        }
    }
}

impl std::convert::TryFrom< &str> for SpanSpanKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPAN_KIND_UNSPECIFIED" => Ok(SpanSpanKindEnum::SPANKINDUNSPECIFIED),
           "INTERNAL" => Ok(SpanSpanKindEnum::INTERNAL),
           "SERVER" => Ok(SpanSpanKindEnum::SERVER),
           "CLIENT" => Ok(SpanSpanKindEnum::CLIENT),
           "PRODUCER" => Ok(SpanSpanKindEnum::PRODUCER),
           "CONSUMER" => Ok(SpanSpanKindEnum::CONSUMER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SpanSpanKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


