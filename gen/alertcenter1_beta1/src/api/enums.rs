use super::*;



// region AlertFeedbackTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the feedback.
pub enum AlertFeedbackTypeEnum {
    

    /// The feedback type is not specified.
    ///
    /// "ALERT_FEEDBACK_TYPE_UNSPECIFIED"
    #[serde(rename="ALERT_FEEDBACK_TYPE_UNSPECIFIED")]
    ALERTFEEDBACKTYPEUNSPECIFIED,
    

    /// The alert report is not useful.
    ///
    /// "NOT_USEFUL"
    #[serde(rename="NOT_USEFUL")]
    NOTUSEFUL,
    

    /// The alert report is somewhat useful.
    ///
    /// "SOMEWHAT_USEFUL"
    #[serde(rename="SOMEWHAT_USEFUL")]
    SOMEWHATUSEFUL,
    

    /// The alert report is very useful.
    ///
    /// "VERY_USEFUL"
    #[serde(rename="VERY_USEFUL")]
    VERYUSEFUL,
}

impl AsRef<str> for AlertFeedbackTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AlertFeedbackTypeEnum::ALERTFEEDBACKTYPEUNSPECIFIED => "ALERT_FEEDBACK_TYPE_UNSPECIFIED",
            AlertFeedbackTypeEnum::NOTUSEFUL => "NOT_USEFUL",
            AlertFeedbackTypeEnum::SOMEWHATUSEFUL => "SOMEWHAT_USEFUL",
            AlertFeedbackTypeEnum::VERYUSEFUL => "VERY_USEFUL",
        }
    }
}

impl std::convert::TryFrom< &str> for AlertFeedbackTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALERT_FEEDBACK_TYPE_UNSPECIFIED" => Ok(AlertFeedbackTypeEnum::ALERTFEEDBACKTYPEUNSPECIFIED),
           "NOT_USEFUL" => Ok(AlertFeedbackTypeEnum::NOTUSEFUL),
           "SOMEWHAT_USEFUL" => Ok(AlertFeedbackTypeEnum::SOMEWHATUSEFUL),
           "VERY_USEFUL" => Ok(AlertFeedbackTypeEnum::VERYUSEFUL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AlertFeedbackTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudPubsubTopicPayloadFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The format of the payload that would be sent. If not specified the format will be JSON.
pub enum CloudPubsubTopicPayloadFormatEnum {
    

    /// Payload format is not specified (will use JSON as default).
    ///
    /// "PAYLOAD_FORMAT_UNSPECIFIED"
    #[serde(rename="PAYLOAD_FORMAT_UNSPECIFIED")]
    PAYLOADFORMATUNSPECIFIED,
    

    /// Use JSON.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for CloudPubsubTopicPayloadFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudPubsubTopicPayloadFormatEnum::PAYLOADFORMATUNSPECIFIED => "PAYLOAD_FORMAT_UNSPECIFIED",
            CloudPubsubTopicPayloadFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudPubsubTopicPayloadFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PAYLOAD_FORMAT_UNSPECIFIED" => Ok(CloudPubsubTopicPayloadFormatEnum::PAYLOADFORMATUNSPECIFIED),
           "JSON" => Ok(CloudPubsubTopicPayloadFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudPubsubTopicPayloadFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


