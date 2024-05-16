use super::*;



// region DeliveryConfigDeliveryRequirementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The DeliveryRequirement for this subscription.
pub enum DeliveryConfigDeliveryRequirementEnum {
    

    /// Default value. This value is unused.
    ///
    /// "DELIVERY_REQUIREMENT_UNSPECIFIED"
    #[serde(rename="DELIVERY_REQUIREMENT_UNSPECIFIED")]
    DELIVERYREQUIREMENTUNSPECIFIED,
    

    /// The server does not wait for a published message to be successfully written to storage before delivering it to subscribers.
    ///
    /// "DELIVER_IMMEDIATELY"
    #[serde(rename="DELIVER_IMMEDIATELY")]
    DELIVERIMMEDIATELY,
    

    /// The server will not deliver a published message to subscribers until the message has been successfully written to storage. This will result in higher end-to-end latency, but consistent delivery.
    ///
    /// "DELIVER_AFTER_STORED"
    #[serde(rename="DELIVER_AFTER_STORED")]
    DELIVERAFTERSTORED,
}

impl AsRef<str> for DeliveryConfigDeliveryRequirementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryConfigDeliveryRequirementEnum::DELIVERYREQUIREMENTUNSPECIFIED => "DELIVERY_REQUIREMENT_UNSPECIFIED",
            DeliveryConfigDeliveryRequirementEnum::DELIVERIMMEDIATELY => "DELIVER_IMMEDIATELY",
            DeliveryConfigDeliveryRequirementEnum::DELIVERAFTERSTORED => "DELIVER_AFTER_STORED",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryConfigDeliveryRequirementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELIVERY_REQUIREMENT_UNSPECIFIED" => Ok(DeliveryConfigDeliveryRequirementEnum::DELIVERYREQUIREMENTUNSPECIFIED),
           "DELIVER_IMMEDIATELY" => Ok(DeliveryConfigDeliveryRequirementEnum::DELIVERIMMEDIATELY),
           "DELIVER_AFTER_STORED" => Ok(DeliveryConfigDeliveryRequirementEnum::DELIVERAFTERSTORED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryConfigDeliveryRequirementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportConfigCurrentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the export, which may be different to the desired state due to errors. This field is output only.
pub enum ExportConfigCurrentStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Messages are being exported.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Exporting messages is suspended.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Messages cannot be exported due to permission denied errors. Output only.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// Messages cannot be exported due to missing resources. Output only.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
}

impl AsRef<str> for ExportConfigCurrentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportConfigCurrentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ExportConfigCurrentStateEnum::ACTIVE => "ACTIVE",
            ExportConfigCurrentStateEnum::PAUSED => "PAUSED",
            ExportConfigCurrentStateEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            ExportConfigCurrentStateEnum::NOTFOUND => "NOT_FOUND",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportConfigCurrentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ExportConfigCurrentStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ExportConfigCurrentStateEnum::ACTIVE),
           "PAUSED" => Ok(ExportConfigCurrentStateEnum::PAUSED),
           "PERMISSION_DENIED" => Ok(ExportConfigCurrentStateEnum::PERMISSIONDENIED),
           "NOT_FOUND" => Ok(ExportConfigCurrentStateEnum::NOTFOUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportConfigCurrentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportConfigDesiredStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired state of this export. Setting this to values other than `ACTIVE` and `PAUSED` will result in an error.
pub enum ExportConfigDesiredStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Messages are being exported.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Exporting messages is suspended.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Messages cannot be exported due to permission denied errors. Output only.
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// Messages cannot be exported due to missing resources. Output only.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
}

impl AsRef<str> for ExportConfigDesiredStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportConfigDesiredStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ExportConfigDesiredStateEnum::ACTIVE => "ACTIVE",
            ExportConfigDesiredStateEnum::PAUSED => "PAUSED",
            ExportConfigDesiredStateEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            ExportConfigDesiredStateEnum::NOTFOUND => "NOT_FOUND",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportConfigDesiredStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ExportConfigDesiredStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ExportConfigDesiredStateEnum::ACTIVE),
           "PAUSED" => Ok(ExportConfigDesiredStateEnum::PAUSED),
           "PERMISSION_DENIED" => Ok(ExportConfigDesiredStateEnum::PERMISSIONDENIED),
           "NOT_FOUND" => Ok(ExportConfigDesiredStateEnum::NOTFOUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportConfigDesiredStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SeekSubscriptionRequestNamedTargetEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Seek to a named position with respect to the message backlog.
pub enum SeekSubscriptionRequestNamedTargetEnum {
    

    /// Unspecified named target. Do not use.
    ///
    /// "NAMED_TARGET_UNSPECIFIED"
    #[serde(rename="NAMED_TARGET_UNSPECIFIED")]
    NAMEDTARGETUNSPECIFIED,
    

    /// Seek to the oldest retained message.
    ///
    /// "TAIL"
    #[serde(rename="TAIL")]
    TAIL,
    

    /// Seek past all recently published messages, skipping the entire message backlog.
    ///
    /// "HEAD"
    #[serde(rename="HEAD")]
    HEAD,
}

impl AsRef<str> for SeekSubscriptionRequestNamedTargetEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SeekSubscriptionRequestNamedTargetEnum::NAMEDTARGETUNSPECIFIED => "NAMED_TARGET_UNSPECIFIED",
            SeekSubscriptionRequestNamedTargetEnum::TAIL => "TAIL",
            SeekSubscriptionRequestNamedTargetEnum::HEAD => "HEAD",
        }
    }
}

impl std::convert::TryFrom< &str> for SeekSubscriptionRequestNamedTargetEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NAMED_TARGET_UNSPECIFIED" => Ok(SeekSubscriptionRequestNamedTargetEnum::NAMEDTARGETUNSPECIFIED),
           "TAIL" => Ok(SeekSubscriptionRequestNamedTargetEnum::TAIL),
           "HEAD" => Ok(SeekSubscriptionRequestNamedTargetEnum::HEAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SeekSubscriptionRequestNamedTargetEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


