use super::*;



// region GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Default summarization model to be used.
pub enum GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum {
    

    /// Unspecified summarization model.
    ///
    /// "SUMMARIZATION_MODEL_UNSPECIFIED"
    #[serde(rename="SUMMARIZATION_MODEL_UNSPECIFIED")]
    SUMMARIZATIONMODELUNSPECIFIED,
    

    /// The CCAI baseline model.
    ///
    /// "BASELINE_MODEL"
    #[serde(rename="BASELINE_MODEL")]
    BASELINEMODEL,
    

    /// The CCAI baseline model, V2.0.
    ///
    /// "BASELINE_MODEL_V2_0"
    #[serde(rename="BASELINE_MODEL_V2_0")]
    BASELINEMODELV20,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum::SUMMARIZATIONMODELUNSPECIFIED => "SUMMARIZATION_MODEL_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum::BASELINEMODEL => "BASELINE_MODEL",
            GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum::BASELINEMODELV20 => "BASELINE_MODEL_V2_0",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUMMARIZATION_MODEL_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum::SUMMARIZATIONMODELUNSPECIFIED),
           "BASELINE_MODEL" => Ok(GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum::BASELINEMODEL),
           "BASELINE_MODEL_V2_0" => Ok(GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum::BASELINEMODELV20),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1AnnotatorSelectorSummarizationConfigSummarizationModelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The correctness level of an answer.
pub enum GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum {
    

    /// Correctness level unspecified.
    ///
    /// "CORRECTNESS_LEVEL_UNSPECIFIED"
    #[serde(rename="CORRECTNESS_LEVEL_UNSPECIFIED")]
    CORRECTNESSLEVELUNSPECIFIED,
    

    /// Answer is totally wrong.
    ///
    /// "NOT_CORRECT"
    #[serde(rename="NOT_CORRECT")]
    NOTCORRECT,
    

    /// Answer is partially correct.
    ///
    /// "PARTIALLY_CORRECT"
    #[serde(rename="PARTIALLY_CORRECT")]
    PARTIALLYCORRECT,
    

    /// Answer is fully correct.
    ///
    /// "FULLY_CORRECT"
    #[serde(rename="FULLY_CORRECT")]
    FULLYCORRECT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::CORRECTNESSLEVELUNSPECIFIED => "CORRECTNESS_LEVEL_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::NOTCORRECT => "NOT_CORRECT",
            GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::PARTIALLYCORRECT => "PARTIALLY_CORRECT",
            GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::FULLYCORRECT => "FULLY_CORRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CORRECTNESS_LEVEL_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::CORRECTNESSLEVELUNSPECIFIED),
           "NOT_CORRECT" => Ok(GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::NOTCORRECT),
           "PARTIALLY_CORRECT" => Ok(GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::PARTIALLYCORRECT),
           "FULLY_CORRECT" => Ok(GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum::FULLYCORRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1AnswerFeedbackCorrectnessLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1ConversationMediumEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The conversation medium, if unspecified will default to PHONE_CALL.
pub enum GoogleCloudContactcenterinsightsV1ConversationMediumEnum {
    

    /// Default value, if unspecified will default to PHONE_CALL.
    ///
    /// "MEDIUM_UNSPECIFIED"
    #[serde(rename="MEDIUM_UNSPECIFIED")]
    MEDIUMUNSPECIFIED,
    

    /// The format for conversations that took place over the phone.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// The format for conversations that took place over chat.
    ///
    /// "CHAT"
    #[serde(rename="CHAT")]
    CHAT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1ConversationMediumEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1ConversationMediumEnum::MEDIUMUNSPECIFIED => "MEDIUM_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1ConversationMediumEnum::PHONECALL => "PHONE_CALL",
            GoogleCloudContactcenterinsightsV1ConversationMediumEnum::CHAT => "CHAT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1ConversationMediumEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIUM_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1ConversationMediumEnum::MEDIUMUNSPECIFIED),
           "PHONE_CALL" => Ok(GoogleCloudContactcenterinsightsV1ConversationMediumEnum::PHONECALL),
           "CHAT" => Ok(GoogleCloudContactcenterinsightsV1ConversationMediumEnum::CHAT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1ConversationMediumEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the participant.
pub enum GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum {
    

    /// Participant's role is not set.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Participant is a human agent.
    ///
    /// "HUMAN_AGENT"
    #[serde(rename="HUMAN_AGENT")]
    HUMANAGENT,
    

    /// Participant is an automated agent.
    ///
    /// "AUTOMATED_AGENT"
    #[serde(rename="AUTOMATED_AGENT")]
    AUTOMATEDAGENT,
    

    /// Participant is an end user who conversed with the contact center.
    ///
    /// "END_USER"
    #[serde(rename="END_USER")]
    ENDUSER,
    

    /// Participant is either a human or automated agent.
    ///
    /// "ANY_AGENT"
    #[serde(rename="ANY_AGENT")]
    ANYAGENT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::ENDUSER => "END_USER",
            GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::ANYAGENT => "ANY_AGENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::ENDUSER),
           "ANY_AGENT" => Ok(GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum::ANYAGENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1ConversationParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1EntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entity type.
pub enum GoogleCloudContactcenterinsightsV1EntityTypeEnum {
    

    /// Unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Person.
    ///
    /// "PERSON"
    #[serde(rename="PERSON")]
    PERSON,
    

    /// Location.
    ///
    /// "LOCATION"
    #[serde(rename="LOCATION")]
    LOCATION,
    

    /// Organization.
    ///
    /// "ORGANIZATION"
    #[serde(rename="ORGANIZATION")]
    ORGANIZATION,
    

    /// Event.
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
    

    /// Artwork.
    ///
    /// "WORK_OF_ART"
    #[serde(rename="WORK_OF_ART")]
    WORKOFART,
    

    /// Consumer product.
    ///
    /// "CONSUMER_GOOD"
    #[serde(rename="CONSUMER_GOOD")]
    CONSUMERGOOD,
    

    /// Other types of entities.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    

    /// Phone number. The metadata lists the phone number (formatted according to local convention), plus whichever additional elements appear in the text: * `number` - The actual number, broken down into sections according to local convention. * `national_prefix` - Country code, if detected. * `area_code` - Region or area code, if detected. * `extension` - Phone extension (to be dialed after connection), if detected.
    ///
    /// "PHONE_NUMBER"
    #[serde(rename="PHONE_NUMBER")]
    PHONENUMBER,
    

    /// Address. The metadata identifies the street number and locality plus whichever additional elements appear in the text: * `street_number` - Street number. * `locality` - City or town. * `street_name` - Street/route name, if detected. * `postal_code` - Postal code, if detected. * `country` - Country, if detected. * `broad_region` - Administrative area, such as the state, if detected. * `narrow_region` - Smaller administrative area, such as county, if detected. * `sublocality` - Used in Asian addresses to demark a district within a city, if detected.
    ///
    /// "ADDRESS"
    #[serde(rename="ADDRESS")]
    ADDRESS,
    

    /// Date. The metadata identifies the components of the date: * `year` - Four digit year, if detected. * `month` - Two digit month number, if detected. * `day` - Two digit day number, if detected.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Number. The metadata is the number itself.
    ///
    /// "NUMBER"
    #[serde(rename="NUMBER")]
    NUMBER,
    

    /// Price. The metadata identifies the `value` and `currency`.
    ///
    /// "PRICE"
    #[serde(rename="PRICE")]
    PRICE,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1EntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::PERSON => "PERSON",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::LOCATION => "LOCATION",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::ORGANIZATION => "ORGANIZATION",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::EVENT => "EVENT",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::WORKOFART => "WORK_OF_ART",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::CONSUMERGOOD => "CONSUMER_GOOD",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::OTHER => "OTHER",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::PHONENUMBER => "PHONE_NUMBER",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::ADDRESS => "ADDRESS",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::DATE => "DATE",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::NUMBER => "NUMBER",
            GoogleCloudContactcenterinsightsV1EntityTypeEnum::PRICE => "PRICE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1EntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::TYPEUNSPECIFIED),
           "PERSON" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::PERSON),
           "LOCATION" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::LOCATION),
           "ORGANIZATION" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::ORGANIZATION),
           "EVENT" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::EVENT),
           "WORK_OF_ART" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::WORKOFART),
           "CONSUMER_GOOD" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::CONSUMERGOOD),
           "OTHER" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::OTHER),
           "PHONE_NUMBER" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::PHONENUMBER),
           "ADDRESS" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::ADDRESS),
           "DATE" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::DATE),
           "NUMBER" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::NUMBER),
           "PRICE" => Ok(GoogleCloudContactcenterinsightsV1EntityTypeEnum::PRICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1EntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the entity mention.
pub enum GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum {
    

    /// Unspecified.
    ///
    /// "MENTION_TYPE_UNSPECIFIED"
    #[serde(rename="MENTION_TYPE_UNSPECIFIED")]
    MENTIONTYPEUNSPECIFIED,
    

    /// Proper noun.
    ///
    /// "PROPER"
    #[serde(rename="PROPER")]
    PROPER,
    

    /// Common noun (or noun compound).
    ///
    /// "COMMON"
    #[serde(rename="COMMON")]
    COMMON,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum::MENTIONTYPEUNSPECIFIED => "MENTION_TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum::PROPER => "PROPER",
            GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum::COMMON => "COMMON",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MENTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum::MENTIONTYPEUNSPECIFIED),
           "PROPER" => Ok(GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum::PROPER),
           "COMMON" => Ok(GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum::COMMON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1EntityMentionDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Options for what to do if the destination table already exists.
pub enum GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum {
    

    /// Write disposition is not specified. Defaults to WRITE_TRUNCATE.
    ///
    /// "WRITE_DISPOSITION_UNSPECIFIED"
    #[serde(rename="WRITE_DISPOSITION_UNSPECIFIED")]
    WRITEDISPOSITIONUNSPECIFIED,
    

    /// If the table already exists, BigQuery will overwrite the table data and use the schema from the load.
    ///
    /// "WRITE_TRUNCATE"
    #[serde(rename="WRITE_TRUNCATE")]
    WRITETRUNCATE,
    

    /// If the table already exists, BigQuery will append data to the table.
    ///
    /// "WRITE_APPEND"
    #[serde(rename="WRITE_APPEND")]
    WRITEAPPEND,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED => "WRITE_DISPOSITION_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum::WRITETRUNCATE => "WRITE_TRUNCATE",
            GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum::WRITEAPPEND => "WRITE_APPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WRITE_DISPOSITION_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum::WRITEDISPOSITIONUNSPECIFIED),
           "WRITE_TRUNCATE" => Ok(GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum::WRITETRUNCATE),
           "WRITE_APPEND" => Ok(GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum::WRITEAPPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1ExportInsightsDataRequestWriteDispositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the type of the objects in `bucket_uri`.
pub enum GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum {
    

    /// The object type is unspecified and will default to `TRANSCRIPT`.
    ///
    /// "BUCKET_OBJECT_TYPE_UNSPECIFIED"
    #[serde(rename="BUCKET_OBJECT_TYPE_UNSPECIFIED")]
    BUCKETOBJECTTYPEUNSPECIFIED,
    

    /// The object is a transcript.
    ///
    /// "TRANSCRIPT"
    #[serde(rename="TRANSCRIPT")]
    TRANSCRIPT,
    

    /// The object is an audio file.
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum::BUCKETOBJECTTYPEUNSPECIFIED => "BUCKET_OBJECT_TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum::TRANSCRIPT => "TRANSCRIPT",
            GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUCKET_OBJECT_TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum::BUCKETOBJECTTYPEUNSPECIFIED),
           "TRANSCRIPT" => Ok(GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum::TRANSCRIPT),
           "AUDIO" => Ok(GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1IngestConversationsRequestGcsSourceBucketObjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The medium transcript objects represent.
pub enum GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum {
    

    /// Default value, if unspecified will default to PHONE_CALL.
    ///
    /// "MEDIUM_UNSPECIFIED"
    #[serde(rename="MEDIUM_UNSPECIFIED")]
    MEDIUMUNSPECIFIED,
    

    /// The format for conversations that took place over the phone.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// The format for conversations that took place over chat.
    ///
    /// "CHAT"
    #[serde(rename="CHAT")]
    CHAT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum::MEDIUMUNSPECIFIED => "MEDIUM_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum::PHONECALL => "PHONE_CALL",
            GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum::CHAT => "CHAT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIUM_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum::MEDIUMUNSPECIFIED),
           "PHONE_CALL" => Ok(GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum::PHONECALL),
           "CHAT" => Ok(GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum::CHAT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1IngestConversationsRequestTranscriptObjectConfigMediumEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the model.
pub enum GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum {
    

    /// Unspecified model type.
    ///
    /// "MODEL_TYPE_UNSPECIFIED"
    #[serde(rename="MODEL_TYPE_UNSPECIFIED")]
    MODELTYPEUNSPECIFIED,
    

    /// Type V1.
    ///
    /// "TYPE_V1"
    #[serde(rename="TYPE_V1")]
    TYPEV1,
    

    /// Type V2.
    ///
    /// "TYPE_V2"
    #[serde(rename="TYPE_V2")]
    TYPEV2,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum::MODELTYPEUNSPECIFIED => "MODEL_TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum::TYPEV1 => "TYPE_V1",
            GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum::TYPEV2 => "TYPE_V2",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum::MODELTYPEUNSPECIFIED),
           "TYPE_V1" => Ok(GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum::TYPEV1),
           "TYPE_V2" => Ok(GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum::TYPEV2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1IssueModelModelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1IssueModelStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the model.
pub enum GoogleCloudContactcenterinsightsV1IssueModelStateEnum {
    

    /// Unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Model is not deployed but is ready to deploy.
    ///
    /// "UNDEPLOYED"
    #[serde(rename="UNDEPLOYED")]
    UNDEPLOYED,
    

    /// Model is being deployed.
    ///
    /// "DEPLOYING"
    #[serde(rename="DEPLOYING")]
    DEPLOYING,
    

    /// Model is deployed and is ready to be used. A model can only be used in analysis if it's in this state.
    ///
    /// "DEPLOYED"
    #[serde(rename="DEPLOYED")]
    DEPLOYED,
    

    /// Model is being undeployed.
    ///
    /// "UNDEPLOYING"
    #[serde(rename="UNDEPLOYING")]
    UNDEPLOYING,
    

    /// Model is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1IssueModelStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1IssueModelStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1IssueModelStateEnum::UNDEPLOYED => "UNDEPLOYED",
            GoogleCloudContactcenterinsightsV1IssueModelStateEnum::DEPLOYING => "DEPLOYING",
            GoogleCloudContactcenterinsightsV1IssueModelStateEnum::DEPLOYED => "DEPLOYED",
            GoogleCloudContactcenterinsightsV1IssueModelStateEnum::UNDEPLOYING => "UNDEPLOYING",
            GoogleCloudContactcenterinsightsV1IssueModelStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1IssueModelStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1IssueModelStateEnum::STATEUNSPECIFIED),
           "UNDEPLOYED" => Ok(GoogleCloudContactcenterinsightsV1IssueModelStateEnum::UNDEPLOYED),
           "DEPLOYING" => Ok(GoogleCloudContactcenterinsightsV1IssueModelStateEnum::DEPLOYING),
           "DEPLOYED" => Ok(GoogleCloudContactcenterinsightsV1IssueModelStateEnum::DEPLOYED),
           "UNDEPLOYING" => Ok(GoogleCloudContactcenterinsightsV1IssueModelStateEnum::UNDEPLOYING),
           "DELETING" => Ok(GoogleCloudContactcenterinsightsV1IssueModelStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1IssueModelStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Medium of conversations used in training data. This field is being deprecated. To specify the medium to be used in training a new issue model, set the `medium` field on `filter`.
pub enum GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum {
    

    /// Default value, if unspecified will default to PHONE_CALL.
    ///
    /// "MEDIUM_UNSPECIFIED"
    #[serde(rename="MEDIUM_UNSPECIFIED")]
    MEDIUMUNSPECIFIED,
    

    /// The format for conversations that took place over the phone.
    ///
    /// "PHONE_CALL"
    #[serde(rename="PHONE_CALL")]
    PHONECALL,
    

    /// The format for conversations that took place over chat.
    ///
    /// "CHAT"
    #[serde(rename="CHAT")]
    CHAT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum::MEDIUMUNSPECIFIED => "MEDIUM_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum::PHONECALL => "PHONE_CALL",
            GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum::CHAT => "CHAT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MEDIUM_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum::MEDIUMUNSPECIFIED),
           "PHONE_CALL" => Ok(GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum::PHONECALL),
           "CHAT" => Ok(GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum::CHAT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1IssueModelInputDataConfigMediumEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of this phrase match rule group.
pub enum GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum {
    

    /// Unspecified.
    ///
    /// "PHRASE_MATCH_RULE_GROUP_TYPE_UNSPECIFIED"
    #[serde(rename="PHRASE_MATCH_RULE_GROUP_TYPE_UNSPECIFIED")]
    PHRASEMATCHRULEGROUPTYPEUNSPECIFIED,
    

    /// Must meet all phrase match rules or there is no match.
    ///
    /// "ALL_OF"
    #[serde(rename="ALL_OF")]
    ALLOF,
    

    /// If any of the phrase match rules are met, there is a match.
    ///
    /// "ANY_OF"
    #[serde(rename="ANY_OF")]
    ANYOF,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum::PHRASEMATCHRULEGROUPTYPEUNSPECIFIED => "PHRASE_MATCH_RULE_GROUP_TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum::ALLOF => "ALL_OF",
            GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum::ANYOF => "ANY_OF",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PHRASE_MATCH_RULE_GROUP_TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum::PHRASEMATCHRULEGROUPTYPEUNSPECIFIED),
           "ALL_OF" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum::ALLOF),
           "ANY_OF" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum::ANYOF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1PhraseMatchRuleGroupTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role whose utterances the phrase matcher should be matched against. If the role is ROLE_UNSPECIFIED it will be matched against any utterances in the transcript.
pub enum GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum {
    

    /// Participant's role is not set.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Participant is a human agent.
    ///
    /// "HUMAN_AGENT"
    #[serde(rename="HUMAN_AGENT")]
    HUMANAGENT,
    

    /// Participant is an automated agent.
    ///
    /// "AUTOMATED_AGENT"
    #[serde(rename="AUTOMATED_AGENT")]
    AUTOMATEDAGENT,
    

    /// Participant is an end user who conversed with the contact center.
    ///
    /// "END_USER"
    #[serde(rename="END_USER")]
    ENDUSER,
    

    /// Participant is either a human or automated agent.
    ///
    /// "ANY_AGENT"
    #[serde(rename="ANY_AGENT")]
    ANYAGENT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::ENDUSER => "END_USER",
            GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::ANYAGENT => "ANY_AGENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::ENDUSER),
           "ANY_AGENT" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum::ANYAGENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1PhraseMatcherRoleMatchEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of this phrase matcher.
pub enum GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum {
    

    /// Unspecified.
    ///
    /// "PHRASE_MATCHER_TYPE_UNSPECIFIED"
    #[serde(rename="PHRASE_MATCHER_TYPE_UNSPECIFIED")]
    PHRASEMATCHERTYPEUNSPECIFIED,
    

    /// Must meet all phrase match rule groups or there is no match.
    ///
    /// "ALL_OF"
    #[serde(rename="ALL_OF")]
    ALLOF,
    

    /// If any of the phrase match rule groups are met, there is a match.
    ///
    /// "ANY_OF"
    #[serde(rename="ANY_OF")]
    ANYOF,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum::PHRASEMATCHERTYPEUNSPECIFIED => "PHRASE_MATCHER_TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum::ALLOF => "ALL_OF",
            GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum::ANYOF => "ANY_OF",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PHRASE_MATCHER_TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum::PHRASEMATCHERTYPEUNSPECIFIED),
           "ALL_OF" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum::ALLOF),
           "ANY_OF" => Ok(GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum::ANYOF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1PhraseMatcherTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the answer.
pub enum GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum {
    

    /// The answer has a unspecified type.
    ///
    /// "ANSWER_TYPE_UNSPECIFIED"
    #[serde(rename="ANSWER_TYPE_UNSPECIFIED")]
    ANSWERTYPEUNSPECIFIED,
    

    /// The answer is from FAQ documents.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// The answer is from generative model.
    ///
    /// "GENERATIVE"
    #[serde(rename="GENERATIVE")]
    GENERATIVE,
    

    /// The answer is from intent matching.
    ///
    /// "INTENT"
    #[serde(rename="INTENT")]
    INTENT,
}

impl AsRef<str> for GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::ANSWERTYPEUNSPECIFIED => "ANSWER_TYPE_UNSPECIFIED",
            GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::FAQ => "FAQ",
            GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::GENERATIVE => "GENERATIVE",
            GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::INTENT => "INTENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANSWER_TYPE_UNSPECIFIED" => Ok(GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::ANSWERTYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::FAQ),
           "GENERATIVE" => Ok(GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::GENERATIVE),
           "INTENT" => Ok(GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum::INTENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudContactcenterinsightsV1SearchKnowledgeAnswerAnswerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The level of details of the conversation. Default is `BASIC`.
pub enum ProjectViewEnum {
    

    /// The conversation view is not specified. * Defaults to `FULL` in `GetConversationRequest`. * Defaults to `BASIC` in `ListConversationsRequest`.
    ///
    /// "CONVERSATION_VIEW_UNSPECIFIED"
    #[serde(rename="CONVERSATION_VIEW_UNSPECIFIED")]
    CONVERSATIONVIEWUNSPECIFIED,
    

    /// Populates all fields in the conversation.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Populates all fields in the conversation except the transcript.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::CONVERSATIONVIEWUNSPECIFIED => "CONVERSATION_VIEW_UNSPECIFIED",
            ProjectViewEnum::FULL => "FULL",
            ProjectViewEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONVERSATION_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::CONVERSATIONVIEWUNSPECIFIED),
           "FULL" => Ok(ProjectViewEnum::FULL),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
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


