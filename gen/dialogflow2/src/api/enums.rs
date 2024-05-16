use super::*;



// region GoogleCloudDialogflowV2AgentApiVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version.
pub enum GoogleCloudDialogflowV2AgentApiVersionEnum {
    

    /// Not specified.
    ///
    /// "API_VERSION_UNSPECIFIED"
    #[serde(rename="API_VERSION_UNSPECIFIED")]
    APIVERSIONUNSPECIFIED,
    

    /// Legacy V1 API.
    ///
    /// "API_VERSION_V1"
    #[serde(rename="API_VERSION_V1")]
    APIVERSIONV1,
    

    /// V2 API.
    ///
    /// "API_VERSION_V2"
    #[serde(rename="API_VERSION_V2")]
    APIVERSIONV2,
    

    /// V2beta1 API.
    ///
    /// "API_VERSION_V2_BETA_1"
    #[serde(rename="API_VERSION_V2_BETA_1")]
    APIVERSIONV2BETA1,
}

impl AsRef<str> for GoogleCloudDialogflowV2AgentApiVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONUNSPECIFIED => "API_VERSION_UNSPECIFIED",
            GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONV1 => "API_VERSION_V1",
            GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONV2 => "API_VERSION_V2",
            GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONV2BETA1 => "API_VERSION_V2_BETA_1",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AgentApiVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "API_VERSION_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONUNSPECIFIED),
           "API_VERSION_V1" => Ok(GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONV1),
           "API_VERSION_V2" => Ok(GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONV2),
           "API_VERSION_V2_BETA_1" => Ok(GoogleCloudDialogflowV2AgentApiVersionEnum::APIVERSIONV2BETA1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AgentApiVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AgentMatchModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Determines how intents are detected from user queries.
pub enum GoogleCloudDialogflowV2AgentMatchModeEnum {
    

    /// Not specified.
    ///
    /// "MATCH_MODE_UNSPECIFIED"
    #[serde(rename="MATCH_MODE_UNSPECIFIED")]
    MATCHMODEUNSPECIFIED,
    

    /// Best for agents with a small number of examples in intents and/or wide use of templates syntax and composite entities.
    ///
    /// "MATCH_MODE_HYBRID"
    #[serde(rename="MATCH_MODE_HYBRID")]
    MATCHMODEHYBRID,
    

    /// Can be used for agents with a large number of examples in intents, especially the ones using @sys.any or very large custom entities.
    ///
    /// "MATCH_MODE_ML_ONLY"
    #[serde(rename="MATCH_MODE_ML_ONLY")]
    MATCHMODEMLONLY,
}

impl AsRef<str> for GoogleCloudDialogflowV2AgentMatchModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AgentMatchModeEnum::MATCHMODEUNSPECIFIED => "MATCH_MODE_UNSPECIFIED",
            GoogleCloudDialogflowV2AgentMatchModeEnum::MATCHMODEHYBRID => "MATCH_MODE_HYBRID",
            GoogleCloudDialogflowV2AgentMatchModeEnum::MATCHMODEMLONLY => "MATCH_MODE_ML_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AgentMatchModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCH_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AgentMatchModeEnum::MATCHMODEUNSPECIFIED),
           "MATCH_MODE_HYBRID" => Ok(GoogleCloudDialogflowV2AgentMatchModeEnum::MATCHMODEHYBRID),
           "MATCH_MODE_ML_ONLY" => Ok(GoogleCloudDialogflowV2AgentMatchModeEnum::MATCHMODEMLONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AgentMatchModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AgentTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The agent tier. If not specified, TIER_STANDARD is assumed.
pub enum GoogleCloudDialogflowV2AgentTierEnum {
    

    /// Not specified. This value should never be used.
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// Trial Edition, previously known as Standard Edition.
    ///
    /// "TIER_STANDARD"
    #[serde(rename="TIER_STANDARD")]
    TIERSTANDARD,
    

    /// Essentials Edition, previously known as Enterprise Essential Edition.
    ///
    /// "TIER_ENTERPRISE"
    #[serde(rename="TIER_ENTERPRISE")]
    TIERENTERPRISE,
    

    /// Essentials Edition (same as TIER_ENTERPRISE), previously known as Enterprise Plus Edition.
    ///
    /// "TIER_ENTERPRISE_PLUS"
    #[serde(rename="TIER_ENTERPRISE_PLUS")]
    TIERENTERPRISEPLUS,
}

impl AsRef<str> for GoogleCloudDialogflowV2AgentTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AgentTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            GoogleCloudDialogflowV2AgentTierEnum::TIERSTANDARD => "TIER_STANDARD",
            GoogleCloudDialogflowV2AgentTierEnum::TIERENTERPRISE => "TIER_ENTERPRISE",
            GoogleCloudDialogflowV2AgentTierEnum::TIERENTERPRISEPLUS => "TIER_ENTERPRISE_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AgentTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AgentTierEnum::TIERUNSPECIFIED),
           "TIER_STANDARD" => Ok(GoogleCloudDialogflowV2AgentTierEnum::TIERSTANDARD),
           "TIER_ENTERPRISE" => Ok(GoogleCloudDialogflowV2AgentTierEnum::TIERENTERPRISE),
           "TIER_ENTERPRISE_PLUS" => Ok(GoogleCloudDialogflowV2AgentTierEnum::TIERENTERPRISEPLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AgentTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the suggested answer is relevant. For example: * Query: "Can I change my mailing address?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * answer_relevance: AnswerRelevance.IRRELEVANT
pub enum GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum {
    

    /// Answer relevance unspecified.
    ///
    /// "ANSWER_RELEVANCE_UNSPECIFIED"
    #[serde(rename="ANSWER_RELEVANCE_UNSPECIFIED")]
    ANSWERRELEVANCEUNSPECIFIED,
    

    /// Answer is irrelevant to query.
    ///
    /// "IRRELEVANT"
    #[serde(rename="IRRELEVANT")]
    IRRELEVANT,
    

    /// Answer is relevant to query.
    ///
    /// "RELEVANT"
    #[serde(rename="RELEVANT")]
    RELEVANT,
}

impl AsRef<str> for GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum::ANSWERRELEVANCEUNSPECIFIED => "ANSWER_RELEVANCE_UNSPECIFIED",
            GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum::IRRELEVANT => "IRRELEVANT",
            GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum::RELEVANT => "RELEVANT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANSWER_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum::ANSWERRELEVANCEUNSPECIFIED),
           "IRRELEVANT" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum::IRRELEVANT),
           "RELEVANT" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum::RELEVANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AgentAssistantFeedbackAnswerRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the information in the document is correct. For example: * Query: "Can I return the package in 2 days once received?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * Ground truth: "No return or exchange is allowed." * [document_correctness]: INCORRECT
pub enum GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum {
    

    /// Document correctness unspecified.
    ///
    /// "DOCUMENT_CORRECTNESS_UNSPECIFIED"
    #[serde(rename="DOCUMENT_CORRECTNESS_UNSPECIFIED")]
    DOCUMENTCORRECTNESSUNSPECIFIED,
    

    /// Information in document is incorrect.
    ///
    /// "INCORRECT"
    #[serde(rename="INCORRECT")]
    INCORRECT,
    

    /// Information in document is correct.
    ///
    /// "CORRECT"
    #[serde(rename="CORRECT")]
    CORRECT,
}

impl AsRef<str> for GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum::DOCUMENTCORRECTNESSUNSPECIFIED => "DOCUMENT_CORRECTNESS_UNSPECIFIED",
            GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum::INCORRECT => "INCORRECT",
            GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum::CORRECT => "CORRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOCUMENT_CORRECTNESS_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum::DOCUMENTCORRECTNESSUNSPECIFIED),
           "INCORRECT" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum::INCORRECT),
           "CORRECT" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum::CORRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentCorrectnessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the suggested document is efficient. For example, if the document is poorly written, hard to understand, hard to use or too long to find useful information, document_efficiency is DocumentEfficiency.INEFFICIENT.
pub enum GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum {
    

    /// Document efficiency unspecified.
    ///
    /// "DOCUMENT_EFFICIENCY_UNSPECIFIED"
    #[serde(rename="DOCUMENT_EFFICIENCY_UNSPECIFIED")]
    DOCUMENTEFFICIENCYUNSPECIFIED,
    

    /// Document is inefficient.
    ///
    /// "INEFFICIENT"
    #[serde(rename="INEFFICIENT")]
    INEFFICIENT,
    

    /// Document is efficient.
    ///
    /// "EFFICIENT"
    #[serde(rename="EFFICIENT")]
    EFFICIENT,
}

impl AsRef<str> for GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum::DOCUMENTEFFICIENCYUNSPECIFIED => "DOCUMENT_EFFICIENCY_UNSPECIFIED",
            GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum::INEFFICIENT => "INEFFICIENT",
            GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum::EFFICIENT => "EFFICIENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOCUMENT_EFFICIENCY_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum::DOCUMENTEFFICIENCYUNSPECIFIED),
           "INEFFICIENT" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum::INEFFICIENT),
           "EFFICIENT" => Ok(GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum::EFFICIENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AgentAssistantFeedbackDocumentEfficiencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The correctness level of the specific answer.
pub enum GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::CORRECTNESSLEVELUNSPECIFIED => "CORRECTNESS_LEVEL_UNSPECIFIED",
            GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::NOTCORRECT => "NOT_CORRECT",
            GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::PARTIALLYCORRECT => "PARTIALLY_CORRECT",
            GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::FULLYCORRECT => "FULLY_CORRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CORRECTNESS_LEVEL_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::CORRECTNESSLEVELUNSPECIFIED),
           "NOT_CORRECT" => Ok(GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::NOTCORRECT),
           "PARTIALLY_CORRECT" => Ok(GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::PARTIALLYCORRECT),
           "FULLY_CORRECT" => Ok(GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum::FULLYCORRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AnswerFeedbackCorrectnessLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of the article suggestion model. If not provided, model_type is used.
pub enum GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum {
    

    /// ModelType unspecified.
    ///
    /// "MODEL_TYPE_UNSPECIFIED"
    #[serde(rename="MODEL_TYPE_UNSPECIFIED")]
    MODELTYPEUNSPECIFIED,
    

    /// ModelType smart reply dual encoder model.
    ///
    /// "SMART_REPLY_DUAL_ENCODER_MODEL"
    #[serde(rename="SMART_REPLY_DUAL_ENCODER_MODEL")]
    SMARTREPLYDUALENCODERMODEL,
    

    /// ModelType smart reply bert model.
    ///
    /// "SMART_REPLY_BERT_MODEL"
    #[serde(rename="SMART_REPLY_BERT_MODEL")]
    SMARTREPLYBERTMODEL,
}

impl AsRef<str> for GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum::MODELTYPEUNSPECIFIED => "MODEL_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum::SMARTREPLYDUALENCODERMODEL => "SMART_REPLY_DUAL_ENCODER_MODEL",
            GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum::SMARTREPLYBERTMODEL => "SMART_REPLY_BERT_MODEL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum::MODELTYPEUNSPECIFIED),
           "SMART_REPLY_DUAL_ENCODER_MODEL" => Ok(GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum::SMARTREPLYDUALENCODERMODEL),
           "SMART_REPLY_BERT_MODEL" => Ok(GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum::SMARTREPLYBERTMODEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ArticleSuggestionModelMetadataTrainingModelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// AutomatedAgentReply type.
pub enum GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    

    /// Not specified. This should never happen.
    ///
    /// "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED"
    #[serde(rename="AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED")]
    AUTOMATEDAGENTREPLYTYPEUNSPECIFIED,
    

    /// Partial reply. e.g. Aggregated responses in a `Fulfillment` that enables `return_partial_response` can be returned as partial reply. WARNING: partial reply is not eligible for barge-in.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Final reply.
    ///
    /// "FINAL"
    #[serde(rename="FINAL")]
    FINAL,
}

impl AsRef<str> for GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum::AUTOMATEDAGENTREPLYTYPEUNSPECIFIED => "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum::PARTIAL => "PARTIAL",
            GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum::FINAL => "FINAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum::AUTOMATEDAGENTREPLYTYPEUNSPECIFIED),
           "PARTIAL" => Ok(GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum::PARTIAL),
           "FINAL" => Ok(GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum::FINAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The resource view to apply to the returned intent.
pub enum GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum {
    

    /// Training phrases field is not populated in the response.
    ///
    /// "INTENT_VIEW_UNSPECIFIED"
    #[serde(rename="INTENT_VIEW_UNSPECIFIED")]
    INTENTVIEWUNSPECIFIED,
    

    /// All fields are populated.
    ///
    /// "INTENT_VIEW_FULL"
    #[serde(rename="INTENT_VIEW_FULL")]
    INTENTVIEWFULL,
}

impl AsRef<str> for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWUNSPECIFIED => "INTENT_VIEW_UNSPECIFIED",
            GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWFULL => "INTENT_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTENT_VIEW_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWUNSPECIFIED),
           "INTENT_VIEW_FULL" => Ok(GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWFULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2BatchUpdateIntentsRequestIntentViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The participant role to remove the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
pub enum GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    

    /// Participant role not set.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Participant is a human agent.
    ///
    /// "HUMAN_AGENT"
    #[serde(rename="HUMAN_AGENT")]
    HUMANAGENT,
    

    /// Participant is an automated agent, such as a Dialogflow agent.
    ///
    /// "AUTOMATED_AGENT"
    #[serde(rename="AUTOMATED_AGENT")]
    AUTOMATEDAGENT,
    

    /// Participant is an end user that has called or chatted with Dialogflow services.
    ///
    /// "END_USER"
    #[serde(rename="END_USER")]
    ENDUSER,
}

impl AsRef<str> for GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the suggestion feature to remove.
pub enum GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    

    /// Unspecified feature type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Run article suggestion model for chat.
    ///
    /// "ARTICLE_SUGGESTION"
    #[serde(rename="ARTICLE_SUGGESTION")]
    ARTICLESUGGESTION,
    

    /// Run FAQ model for chat.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// Run smart reply model for chat.
    ///
    /// "SMART_REPLY"
    #[serde(rename="SMART_REPLY")]
    SMARTREPLY,
    

    /// Run knowledge search with text input from agent or text generated query.
    ///
    /// "KNOWLEDGE_SEARCH"
    #[serde(rename="KNOWLEDGE_SEARCH")]
    KNOWLEDGESEARCH,
}

impl AsRef<str> for GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::SMARTREPLY => "SMART_REPLY",
            GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::KNOWLEDGESEARCH => "KNOWLEDGE_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::TYPEUNSPECIFIED),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::ARTICLESUGGESTION),
           "FAQ" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::FAQ),
           "SMART_REPLY" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::SMARTREPLY),
           "KNOWLEDGE_SEARCH" => Ok(GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::KNOWLEDGESEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ConversationConversationStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE.
pub enum GoogleCloudDialogflowV2ConversationConversationStageEnum {
    

    /// Unknown. Should never be used after a conversation is successfully created.
    ///
    /// "CONVERSATION_STAGE_UNSPECIFIED"
    #[serde(rename="CONVERSATION_STAGE_UNSPECIFIED")]
    CONVERSATIONSTAGEUNSPECIFIED,
    

    /// The conversation should return virtual agent responses into the conversation.
    ///
    /// "VIRTUAL_AGENT_STAGE"
    #[serde(rename="VIRTUAL_AGENT_STAGE")]
    VIRTUALAGENTSTAGE,
    

    /// The conversation should not provide responses, just listen and provide suggestions.
    ///
    /// "HUMAN_ASSIST_STAGE"
    #[serde(rename="HUMAN_ASSIST_STAGE")]
    HUMANASSISTSTAGE,
}

impl AsRef<str> for GoogleCloudDialogflowV2ConversationConversationStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ConversationConversationStageEnum::CONVERSATIONSTAGEUNSPECIFIED => "CONVERSATION_STAGE_UNSPECIFIED",
            GoogleCloudDialogflowV2ConversationConversationStageEnum::VIRTUALAGENTSTAGE => "VIRTUAL_AGENT_STAGE",
            GoogleCloudDialogflowV2ConversationConversationStageEnum::HUMANASSISTSTAGE => "HUMAN_ASSIST_STAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ConversationConversationStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONVERSATION_STAGE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ConversationConversationStageEnum::CONVERSATIONSTAGEUNSPECIFIED),
           "VIRTUAL_AGENT_STAGE" => Ok(GoogleCloudDialogflowV2ConversationConversationStageEnum::VIRTUALAGENTSTAGE),
           "HUMAN_ASSIST_STAGE" => Ok(GoogleCloudDialogflowV2ConversationConversationStageEnum::HUMANASSISTSTAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ConversationConversationStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ConversationLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the Conversation.
pub enum GoogleCloudDialogflowV2ConversationLifecycleStateEnum {
    

    /// Unknown.
    ///
    /// "LIFECYCLE_STATE_UNSPECIFIED"
    #[serde(rename="LIFECYCLE_STATE_UNSPECIFIED")]
    LIFECYCLESTATEUNSPECIFIED,
    

    /// Conversation is currently open for media analysis.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Conversation has been completed.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
}

impl AsRef<str> for GoogleCloudDialogflowV2ConversationLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ConversationLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2ConversationLifecycleStateEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudDialogflowV2ConversationLifecycleStateEnum::COMPLETED => "COMPLETED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ConversationLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ConversationLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "IN_PROGRESS" => Ok(GoogleCloudDialogflowV2ConversationLifecycleStateEnum::INPROGRESS),
           "COMPLETED" => Ok(GoogleCloudDialogflowV2ConversationLifecycleStateEnum::COMPLETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ConversationLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ConversationModelStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the model. A model can only serve prediction requests after it gets deployed.
pub enum GoogleCloudDialogflowV2ConversationModelStateEnum {
    

    /// Should not be used, an un-set enum has this value by default.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Model being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Model is not deployed but ready to deploy.
    ///
    /// "UNDEPLOYED"
    #[serde(rename="UNDEPLOYED")]
    UNDEPLOYED,
    

    /// Model is deploying.
    ///
    /// "DEPLOYING"
    #[serde(rename="DEPLOYING")]
    DEPLOYING,
    

    /// Model is deployed and ready to use.
    ///
    /// "DEPLOYED"
    #[serde(rename="DEPLOYED")]
    DEPLOYED,
    

    /// Model is undeploying.
    ///
    /// "UNDEPLOYING"
    #[serde(rename="UNDEPLOYING")]
    UNDEPLOYING,
    

    /// Model is deleting.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Model is in error state. Not ready to deploy and use.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Model is being created but the training has not started, The model may remain in this state until there is enough capacity to start training.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for GoogleCloudDialogflowV2ConversationModelStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ConversationModelStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2ConversationModelStateEnum::CREATING => "CREATING",
            GoogleCloudDialogflowV2ConversationModelStateEnum::UNDEPLOYED => "UNDEPLOYED",
            GoogleCloudDialogflowV2ConversationModelStateEnum::DEPLOYING => "DEPLOYING",
            GoogleCloudDialogflowV2ConversationModelStateEnum::DEPLOYED => "DEPLOYED",
            GoogleCloudDialogflowV2ConversationModelStateEnum::UNDEPLOYING => "UNDEPLOYING",
            GoogleCloudDialogflowV2ConversationModelStateEnum::DELETING => "DELETING",
            GoogleCloudDialogflowV2ConversationModelStateEnum::FAILED => "FAILED",
            GoogleCloudDialogflowV2ConversationModelStateEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ConversationModelStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::CREATING),
           "UNDEPLOYED" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::UNDEPLOYED),
           "DEPLOYING" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::DEPLOYING),
           "DEPLOYED" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::DEPLOYED),
           "UNDEPLOYING" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::UNDEPLOYING),
           "DELETING" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::DELETING),
           "FAILED" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::FAILED),
           "PENDING" => Ok(GoogleCloudDialogflowV2ConversationModelStateEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ConversationModelStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The knowledge type of document content.
pub enum GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum {
    

    /// The type is unspecified or arbitrary.
    ///
    /// "KNOWLEDGE_TYPE_UNSPECIFIED"
    #[serde(rename="KNOWLEDGE_TYPE_UNSPECIFIED")]
    KNOWLEDGETYPEUNSPECIFIED,
    

    /// The document content contains question and answer pairs as either HTML or CSV. Typical FAQ HTML formats are parsed accurately, but unusual formats may fail to be parsed. CSV must have questions in the first column and answers in the second, with no header. Because of this explicit format, they are always parsed accurately.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// Documents for which unstructured text is extracted and used for question answering.
    ///
    /// "EXTRACTIVE_QA"
    #[serde(rename="EXTRACTIVE_QA")]
    EXTRACTIVEQA,
    

    /// The entire document content as a whole can be used for query results. Only for Contact Center Solutions on Dialogflow.
    ///
    /// "ARTICLE_SUGGESTION"
    #[serde(rename="ARTICLE_SUGGESTION")]
    ARTICLESUGGESTION,
    

    /// The document contains agent-facing Smart Reply entries.
    ///
    /// "AGENT_FACING_SMART_REPLY"
    #[serde(rename="AGENT_FACING_SMART_REPLY")]
    AGENTFACINGSMARTREPLY,
}

impl AsRef<str> for GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED => "KNOWLEDGE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::EXTRACTIVEQA => "EXTRACTIVE_QA",
            GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::AGENTFACINGSMARTREPLY => "AGENT_FACING_SMART_REPLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KNOWLEDGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::FAQ),
           "EXTRACTIVE_QA" => Ok(GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::EXTRACTIVEQA),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::ARTICLESUGGESTION),
           "AGENT_FACING_SMART_REPLY" => Ok(GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum::AGENTFACINGSMARTREPLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2DocumentKnowledgeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2DocumentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the document.
pub enum GoogleCloudDialogflowV2DocumentStateEnum {
    

    /// The document state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The document creation is in progress.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The document is active and ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The document updation is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The document is reloading.
    ///
    /// "RELOADING"
    #[serde(rename="RELOADING")]
    RELOADING,
    

    /// The document deletion is in progress.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for GoogleCloudDialogflowV2DocumentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2DocumentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2DocumentStateEnum::CREATING => "CREATING",
            GoogleCloudDialogflowV2DocumentStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDialogflowV2DocumentStateEnum::UPDATING => "UPDATING",
            GoogleCloudDialogflowV2DocumentStateEnum::RELOADING => "RELOADING",
            GoogleCloudDialogflowV2DocumentStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2DocumentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2DocumentStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudDialogflowV2DocumentStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudDialogflowV2DocumentStateEnum::ACTIVE),
           "UPDATING" => Ok(GoogleCloudDialogflowV2DocumentStateEnum::UPDATING),
           "RELOADING" => Ok(GoogleCloudDialogflowV2DocumentStateEnum::RELOADING),
           "DELETING" => Ok(GoogleCloudDialogflowV2DocumentStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2DocumentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates whether the entity type can be automatically expanded.
pub enum GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
    

    /// Auto expansion disabled for the entity.
    ///
    /// "AUTO_EXPANSION_MODE_UNSPECIFIED"
    #[serde(rename="AUTO_EXPANSION_MODE_UNSPECIFIED")]
    AUTOEXPANSIONMODEUNSPECIFIED,
    

    /// Allows an agent to recognize values that have not been explicitly listed in the entity.
    ///
    /// "AUTO_EXPANSION_MODE_DEFAULT"
    #[serde(rename="AUTO_EXPANSION_MODE_DEFAULT")]
    AUTOEXPANSIONMODEDEFAULT,
}

impl AsRef<str> for GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEUNSPECIFIED => "AUTO_EXPANSION_MODE_UNSPECIFIED",
            GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEDEFAULT => "AUTO_EXPANSION_MODE_DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO_EXPANSION_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEUNSPECIFIED),
           "AUTO_EXPANSION_MODE_DEFAULT" => Ok(GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEDEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2EntityTypeKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the kind of entity type.
pub enum GoogleCloudDialogflowV2EntityTypeKindEnum {
    

    /// Not specified. This value should be never used.
    ///
    /// "KIND_UNSPECIFIED"
    #[serde(rename="KIND_UNSPECIFIED")]
    KINDUNSPECIFIED,
    

    /// Map entity types allow mapping of a group of synonyms to a reference value.
    ///
    /// "KIND_MAP"
    #[serde(rename="KIND_MAP")]
    KINDMAP,
    

    /// List entity types contain a set of entries that do not map to reference values. However, list entity types can contain references to other entity types (with or without aliases).
    ///
    /// "KIND_LIST"
    #[serde(rename="KIND_LIST")]
    KINDLIST,
    

    /// Regexp entity types allow to specify regular expressions in entries values.
    ///
    /// "KIND_REGEXP"
    #[serde(rename="KIND_REGEXP")]
    KINDREGEXP,
}

impl AsRef<str> for GoogleCloudDialogflowV2EntityTypeKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2EntityTypeKindEnum::KINDUNSPECIFIED => "KIND_UNSPECIFIED",
            GoogleCloudDialogflowV2EntityTypeKindEnum::KINDMAP => "KIND_MAP",
            GoogleCloudDialogflowV2EntityTypeKindEnum::KINDLIST => "KIND_LIST",
            GoogleCloudDialogflowV2EntityTypeKindEnum::KINDREGEXP => "KIND_REGEXP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2EntityTypeKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KIND_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2EntityTypeKindEnum::KINDUNSPECIFIED),
           "KIND_MAP" => Ok(GoogleCloudDialogflowV2EntityTypeKindEnum::KINDMAP),
           "KIND_LIST" => Ok(GoogleCloudDialogflowV2EntityTypeKindEnum::KINDLIST),
           "KIND_REGEXP" => Ok(GoogleCloudDialogflowV2EntityTypeKindEnum::KINDREGEXP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2EntityTypeKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2EnvironmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods.
pub enum GoogleCloudDialogflowV2EnvironmentStateEnum {
    

    /// Not specified. This value is not used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Stopped.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// Loading.
    ///
    /// "LOADING"
    #[serde(rename="LOADING")]
    LOADING,
    

    /// Running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
}

impl AsRef<str> for GoogleCloudDialogflowV2EnvironmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2EnvironmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2EnvironmentStateEnum::STOPPED => "STOPPED",
            GoogleCloudDialogflowV2EnvironmentStateEnum::LOADING => "LOADING",
            GoogleCloudDialogflowV2EnvironmentStateEnum::RUNNING => "RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2EnvironmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2EnvironmentStateEnum::STATEUNSPECIFIED),
           "STOPPED" => Ok(GoogleCloudDialogflowV2EnvironmentStateEnum::STOPPED),
           "LOADING" => Ok(GoogleCloudDialogflowV2EnvironmentStateEnum::LOADING),
           "RUNNING" => Ok(GoogleCloudDialogflowV2EnvironmentStateEnum::RUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2EnvironmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the feature that enabled for fulfillment.
pub enum GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum {
    

    /// Feature type not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Fulfillment is enabled for SmallTalk.
    ///
    /// "SMALLTALK"
    #[serde(rename="SMALLTALK")]
    SMALLTALK,
}

impl AsRef<str> for GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum::SMALLTALK => "SMALLTALK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum::TYPEUNSPECIFIED),
           "SMALLTALK" => Ok(GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum::SMALLTALK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2FulfillmentFeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The selected sections chosen to return when requesting a summary of a conversation. A duplicate selected section will be treated as a single selected section. If section types are not provided, the default will be {SITUATION, ACTION, RESULT}.
pub enum GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    

    /// Undefined section type, does not return anything.
    ///
    /// "SECTION_TYPE_UNSPECIFIED"
    #[serde(rename="SECTION_TYPE_UNSPECIFIED")]
    SECTIONTYPEUNSPECIFIED,
    

    /// What the customer needs help with or has question about. Section name: "situation".
    ///
    /// "SITUATION"
    #[serde(rename="SITUATION")]
    SITUATION,
    

    /// What the agent does to help the customer. Section name: "action".
    ///
    /// "ACTION"
    #[serde(rename="ACTION")]
    ACTION,
    

    /// Result of the customer service. A single word describing the result of the conversation. Section name: "resolution".
    ///
    /// "RESOLUTION"
    #[serde(rename="RESOLUTION")]
    RESOLUTION,
    

    /// Reason for cancellation if the customer requests for a cancellation. "N/A" otherwise. Section name: "reason_for_cancellation".
    ///
    /// "REASON_FOR_CANCELLATION"
    #[serde(rename="REASON_FOR_CANCELLATION")]
    REASONFORCANCELLATION,
    

    /// "Unsatisfied" or "Satisfied" depending on the customer's feelings at the end of the conversation. Section name: "customer_satisfaction".
    ///
    /// "CUSTOMER_SATISFACTION"
    #[serde(rename="CUSTOMER_SATISFACTION")]
    CUSTOMERSATISFACTION,
    

    /// Key entities extracted from the conversation, such as ticket number, order number, dollar amount, etc. Section names are prefixed by "entities/".
    ///
    /// "ENTITIES"
    #[serde(rename="ENTITIES")]
    ENTITIES,
}

impl AsRef<str> for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SITUATION => "SITUATION",
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ACTION => "ACTION",
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::RESOLUTION => "RESOLUTION",
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::REASONFORCANCELLATION => "REASON_FOR_CANCELLATION",
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::CUSTOMERSATISFACTION => "CUSTOMER_SATISFACTION",
            GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ENTITIES => "ENTITIES",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SECTIONTYPEUNSPECIFIED),
           "SITUATION" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SITUATION),
           "ACTION" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ACTION),
           "RESOLUTION" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::RESOLUTION),
           "REASON_FOR_CANCELLATION" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::REASONFORCANCELLATION),
           "CUSTOMER_SATISFACTION" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::CUSTOMERSATISFACTION),
           "ENTITIES" => Ok(GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ENTITIES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The knowledge type of document content.
pub enum GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum {
    

    /// The type is unspecified or arbitrary.
    ///
    /// "KNOWLEDGE_TYPE_UNSPECIFIED"
    #[serde(rename="KNOWLEDGE_TYPE_UNSPECIFIED")]
    KNOWLEDGETYPEUNSPECIFIED,
    

    /// The document content contains question and answer pairs as either HTML or CSV. Typical FAQ HTML formats are parsed accurately, but unusual formats may fail to be parsed. CSV must have questions in the first column and answers in the second, with no header. Because of this explicit format, they are always parsed accurately.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// Documents for which unstructured text is extracted and used for question answering.
    ///
    /// "EXTRACTIVE_QA"
    #[serde(rename="EXTRACTIVE_QA")]
    EXTRACTIVEQA,
    

    /// The entire document content as a whole can be used for query results. Only for Contact Center Solutions on Dialogflow.
    ///
    /// "ARTICLE_SUGGESTION"
    #[serde(rename="ARTICLE_SUGGESTION")]
    ARTICLESUGGESTION,
    

    /// The document contains agent-facing Smart Reply entries.
    ///
    /// "AGENT_FACING_SMART_REPLY"
    #[serde(rename="AGENT_FACING_SMART_REPLY")]
    AGENTFACINGSMARTREPLY,
}

impl AsRef<str> for GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED => "KNOWLEDGE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::EXTRACTIVEQA => "EXTRACTIVE_QA",
            GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::AGENTFACINGSMARTREPLY => "AGENT_FACING_SMART_REPLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KNOWLEDGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::FAQ),
           "EXTRACTIVE_QA" => Ok(GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::EXTRACTIVEQA),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::ARTICLESUGGESTION),
           "AGENT_FACING_SMART_REPLY" => Ok(GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum::AGENTFACINGSMARTREPLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ImportDocumentTemplateKnowledgeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the audio content to process.
pub enum GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum {
    

    /// Not specified.
    ///
    /// "AUDIO_ENCODING_UNSPECIFIED"
    #[serde(rename="AUDIO_ENCODING_UNSPECIFIED")]
    AUDIOENCODINGUNSPECIFIED,
    

    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    ///
    /// "AUDIO_ENCODING_LINEAR_16"
    #[serde(rename="AUDIO_ENCODING_LINEAR_16")]
    AUDIOENCODINGLINEAR16,
    

    /// [`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio Codec) is the recommended encoding because it is lossless (therefore recognition is not compromised) and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported.
    ///
    /// "AUDIO_ENCODING_FLAC"
    #[serde(rename="AUDIO_ENCODING_FLAC")]
    AUDIOENCODINGFLAC,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    ///
    /// "AUDIO_ENCODING_MULAW"
    #[serde(rename="AUDIO_ENCODING_MULAW")]
    AUDIOENCODINGMULAW,
    

    /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    ///
    /// "AUDIO_ENCODING_AMR"
    #[serde(rename="AUDIO_ENCODING_AMR")]
    AUDIOENCODINGAMR,
    

    /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    ///
    /// "AUDIO_ENCODING_AMR_WB"
    #[serde(rename="AUDIO_ENCODING_AMR_WB")]
    AUDIOENCODINGAMRWB,
    

    /// Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be 16000.
    ///
    /// "AUDIO_ENCODING_OGG_OPUS"
    #[serde(rename="AUDIO_ENCODING_OGG_OPUS")]
    AUDIOENCODINGOGGOPUS,
    

    /// Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Dialogflow API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000.
    ///
    /// "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE"
    #[serde(rename="AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE")]
    AUDIOENCODINGSPEEXWITHHEADERBYTE,
}

impl AsRef<str> for GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED => "AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGLINEAR16 => "AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGFLAC => "AUDIO_ENCODING_FLAC",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGMULAW => "AUDIO_ENCODING_MULAW",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMR => "AUDIO_ENCODING_AMR",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMRWB => "AUDIO_ENCODING_AMR_WB",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGOGGOPUS => "AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGSPEEXWITHHEADERBYTE => "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED),
           "AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGLINEAR16),
           "AUDIO_ENCODING_FLAC" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGFLAC),
           "AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGMULAW),
           "AUDIO_ENCODING_AMR" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMR),
           "AUDIO_ENCODING_AMR_WB" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMRWB),
           "AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGOGGOPUS),
           "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE" => Ok(GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum::AUDIOENCODINGSPEEXWITHHEADERBYTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2InputAudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which variant of the Speech model to use.
pub enum GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum {
    

    /// No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE.
    ///
    /// "SPEECH_MODEL_VARIANT_UNSPECIFIED"
    #[serde(rename="SPEECH_MODEL_VARIANT_UNSPECIFIED")]
    SPEECHMODELVARIANTUNSPECIFIED,
    

    /// Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models.
    ///
    /// "USE_BEST_AVAILABLE"
    #[serde(rename="USE_BEST_AVAILABLE")]
    USEBESTAVAILABLE,
    

    /// Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models.
    ///
    /// "USE_STANDARD"
    #[serde(rename="USE_STANDARD")]
    USESTANDARD,
    

    /// Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible.
    ///
    /// "USE_ENHANCED"
    #[serde(rename="USE_ENHANCED")]
    USEENHANCED,
}

impl AsRef<str> for GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED => "SPEECH_MODEL_VARIANT_UNSPECIFIED",
            GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::USEBESTAVAILABLE => "USE_BEST_AVAILABLE",
            GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::USESTANDARD => "USE_STANDARD",
            GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::USEENHANCED => "USE_ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPEECH_MODEL_VARIANT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED),
           "USE_BEST_AVAILABLE" => Ok(GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::USEBESTAVAILABLE),
           "USE_STANDARD" => Ok(GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::USESTANDARD),
           "USE_ENHANCED" => Ok(GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum::USEENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2InputAudioConfigModelVariantEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform).
pub enum GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
    

    /// Default platform.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// Facebook.
    ///
    /// "FACEBOOK"
    #[serde(rename="FACEBOOK")]
    FACEBOOK,
    

    /// Slack.
    ///
    /// "SLACK"
    #[serde(rename="SLACK")]
    SLACK,
    

    /// Telegram.
    ///
    /// "TELEGRAM"
    #[serde(rename="TELEGRAM")]
    TELEGRAM,
    

    /// Kik.
    ///
    /// "KIK"
    #[serde(rename="KIK")]
    KIK,
    

    /// Skype.
    ///
    /// "SKYPE"
    #[serde(rename="SKYPE")]
    SKYPE,
    

    /// Line.
    ///
    /// "LINE"
    #[serde(rename="LINE")]
    LINE,
    

    /// Viber.
    ///
    /// "VIBER"
    #[serde(rename="VIBER")]
    VIBER,
    

    /// Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)
    ///
    /// "ACTIONS_ON_GOOGLE"
    #[serde(rename="ACTIONS_ON_GOOGLE")]
    ACTIONSONGOOGLE,
    

    /// Google Hangouts.
    ///
    /// "GOOGLE_HANGOUTS"
    #[serde(rename="GOOGLE_HANGOUTS")]
    GOOGLEHANGOUTS,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::FACEBOOK => "FACEBOOK",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::SLACK => "SLACK",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::TELEGRAM => "TELEGRAM",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::KIK => "KIK",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::SKYPE => "SKYPE",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::LINE => "LINE",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::VIBER => "VIBER",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::ACTIONSONGOOGLE => "ACTIONS_ON_GOOGLE",
            GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::GOOGLEHANGOUTS => "GOOGLE_HANGOUTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::PLATFORMUNSPECIFIED),
           "FACEBOOK" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::FACEBOOK),
           "SLACK" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::SLACK),
           "TELEGRAM" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::TELEGRAM),
           "KIK" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::KIK),
           "SKYPE" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::SKYPE),
           "LINE" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::LINE),
           "VIBER" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::VIBER),
           "ACTIONS_ON_GOOGLE" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::ACTIONSONGOOGLE),
           "GOOGLE_HANGOUTS" => Ok(GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum::GOOGLEHANGOUTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentWebhookStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates whether webhooks are enabled for the intent.
pub enum GoogleCloudDialogflowV2IntentWebhookStateEnum {
    

    /// Webhook is disabled in the agent and in the intent.
    ///
    /// "WEBHOOK_STATE_UNSPECIFIED"
    #[serde(rename="WEBHOOK_STATE_UNSPECIFIED")]
    WEBHOOKSTATEUNSPECIFIED,
    

    /// Webhook is enabled in the agent and in the intent.
    ///
    /// "WEBHOOK_STATE_ENABLED"
    #[serde(rename="WEBHOOK_STATE_ENABLED")]
    WEBHOOKSTATEENABLED,
    

    /// Webhook is enabled in the agent and in the intent. Also, each slot filling prompt is forwarded to the webhook.
    ///
    /// "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING"
    #[serde(rename="WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING")]
    WEBHOOKSTATEENABLEDFORSLOTFILLING,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentWebhookStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentWebhookStateEnum::WEBHOOKSTATEUNSPECIFIED => "WEBHOOK_STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentWebhookStateEnum::WEBHOOKSTATEENABLED => "WEBHOOK_STATE_ENABLED",
            GoogleCloudDialogflowV2IntentWebhookStateEnum::WEBHOOKSTATEENABLEDFORSLOTFILLING => "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentWebhookStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEBHOOK_STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentWebhookStateEnum::WEBHOOKSTATEUNSPECIFIED),
           "WEBHOOK_STATE_ENABLED" => Ok(GoogleCloudDialogflowV2IntentWebhookStateEnum::WEBHOOKSTATEENABLED),
           "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING" => Ok(GoogleCloudDialogflowV2IntentWebhookStateEnum::WEBHOOKSTATEENABLEDFORSLOTFILLING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentWebhookStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentMessagePlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The platform that this message is intended for.
pub enum GoogleCloudDialogflowV2IntentMessagePlatformEnum {
    

    /// Default platform.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// Facebook.
    ///
    /// "FACEBOOK"
    #[serde(rename="FACEBOOK")]
    FACEBOOK,
    

    /// Slack.
    ///
    /// "SLACK"
    #[serde(rename="SLACK")]
    SLACK,
    

    /// Telegram.
    ///
    /// "TELEGRAM"
    #[serde(rename="TELEGRAM")]
    TELEGRAM,
    

    /// Kik.
    ///
    /// "KIK"
    #[serde(rename="KIK")]
    KIK,
    

    /// Skype.
    ///
    /// "SKYPE"
    #[serde(rename="SKYPE")]
    SKYPE,
    

    /// Line.
    ///
    /// "LINE"
    #[serde(rename="LINE")]
    LINE,
    

    /// Viber.
    ///
    /// "VIBER"
    #[serde(rename="VIBER")]
    VIBER,
    

    /// Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)
    ///
    /// "ACTIONS_ON_GOOGLE"
    #[serde(rename="ACTIONS_ON_GOOGLE")]
    ACTIONSONGOOGLE,
    

    /// Google Hangouts.
    ///
    /// "GOOGLE_HANGOUTS"
    #[serde(rename="GOOGLE_HANGOUTS")]
    GOOGLEHANGOUTS,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentMessagePlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::FACEBOOK => "FACEBOOK",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::SLACK => "SLACK",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::TELEGRAM => "TELEGRAM",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::KIK => "KIK",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::SKYPE => "SKYPE",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::LINE => "LINE",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::VIBER => "VIBER",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::ACTIONSONGOOGLE => "ACTIONS_ON_GOOGLE",
            GoogleCloudDialogflowV2IntentMessagePlatformEnum::GOOGLEHANGOUTS => "GOOGLE_HANGOUTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentMessagePlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::PLATFORMUNSPECIFIED),
           "FACEBOOK" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::FACEBOOK),
           "SLACK" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::SLACK),
           "TELEGRAM" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::TELEGRAM),
           "KIK" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::KIK),
           "SKYPE" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::SKYPE),
           "LINE" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::LINE),
           "VIBER" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::VIBER),
           "ACTIONS_ON_GOOGLE" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::ACTIONSONGOOGLE),
           "GOOGLE_HANGOUTS" => Ok(GoogleCloudDialogflowV2IntentMessagePlatformEnum::GOOGLEHANGOUTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentMessagePlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Settings for displaying the image. Applies to every image in items.
pub enum GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    

    /// Fill the gaps between the image and the image container with gray bars.
    ///
    /// "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED"
    #[serde(rename="IMAGE_DISPLAY_OPTIONS_UNSPECIFIED")]
    IMAGEDISPLAYOPTIONSUNSPECIFIED,
    

    /// Fill the gaps between the image and the image container with gray bars.
    ///
    /// "GRAY"
    #[serde(rename="GRAY")]
    GRAY,
    

    /// Fill the gaps between the image and the image container with white bars.
    ///
    /// "WHITE"
    #[serde(rename="WHITE")]
    WHITE,
    

    /// Image is scaled such that the image width and height match or exceed the container dimensions. This may crop the top and bottom of the image if the scaled image height is greater than the container height, or crop the left and right of the image if the scaled image width is greater than the container width. This is similar to "Zoom Mode" on a widescreen TV when playing a 4:3 video.
    ///
    /// "CROPPED"
    #[serde(rename="CROPPED")]
    CROPPED,
    

    /// Pad the gaps between image and image frame with a blurred copy of the same image.
    ///
    /// "BLURRED_BACKGROUND"
    #[serde(rename="BLURRED_BACKGROUND")]
    BLURREDBACKGROUND,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::IMAGEDISPLAYOPTIONSUNSPECIFIED => "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::GRAY => "GRAY",
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::WHITE => "WHITE",
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::CROPPED => "CROPPED",
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::BLURREDBACKGROUND => "BLURRED_BACKGROUND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::IMAGEDISPLAYOPTIONSUNSPECIFIED),
           "GRAY" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::GRAY),
           "WHITE" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::WHITE),
           "CROPPED" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::CROPPED),
           "BLURRED_BACKGROUND" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::BLURREDBACKGROUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser.
pub enum GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    

    /// Unspecified
    ///
    /// "URL_TYPE_HINT_UNSPECIFIED"
    #[serde(rename="URL_TYPE_HINT_UNSPECIFIED")]
    URLTYPEHINTUNSPECIFIED,
    

    /// Url would be an amp action
    ///
    /// "AMP_ACTION"
    #[serde(rename="AMP_ACTION")]
    AMPACTION,
    

    /// URL that points directly to AMP content, or to a canonical URL which refers to AMP content via .
    ///
    /// "AMP_CONTENT"
    #[serde(rename="AMP_CONTENT")]
    AMPCONTENT,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::URLTYPEHINTUNSPECIFIED => "URL_TYPE_HINT_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPACTION => "AMP_ACTION",
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPCONTENT => "AMP_CONTENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "URL_TYPE_HINT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::URLTYPEHINTUNSPECIFIED),
           "AMP_ACTION" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPACTION),
           "AMP_CONTENT" => Ok(GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPCONTENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Defines text alignment for all cells in this column.
pub enum GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum {
    

    /// Text is aligned to the leading edge of the column.
    ///
    /// "HORIZONTAL_ALIGNMENT_UNSPECIFIED"
    #[serde(rename="HORIZONTAL_ALIGNMENT_UNSPECIFIED")]
    HORIZONTALALIGNMENTUNSPECIFIED,
    

    /// Text is aligned to the leading edge of the column.
    ///
    /// "LEADING"
    #[serde(rename="LEADING")]
    LEADING,
    

    /// Text is centered in the column.
    ///
    /// "CENTER"
    #[serde(rename="CENTER")]
    CENTER,
    

    /// Text is aligned to the trailing edge of the column.
    ///
    /// "TRAILING"
    #[serde(rename="TRAILING")]
    TRAILING,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::HORIZONTALALIGNMENTUNSPECIFIED => "HORIZONTAL_ALIGNMENT_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::LEADING => "LEADING",
            GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::CENTER => "CENTER",
            GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::TRAILING => "TRAILING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::HORIZONTALALIGNMENTUNSPECIFIED),
           "LEADING" => Ok(GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::LEADING),
           "CENTER" => Ok(GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::CENTER),
           "TRAILING" => Ok(GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum::TRAILING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentMessageColumnPropertyHorizontalAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. What type of media is the content (ie "audio").
pub enum GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
    

    /// Unspecified.
    ///
    /// "RESPONSE_MEDIA_TYPE_UNSPECIFIED"
    #[serde(rename="RESPONSE_MEDIA_TYPE_UNSPECIFIED")]
    RESPONSEMEDIATYPEUNSPECIFIED,
    

    /// Response media type is audio.
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum::RESPONSEMEDIATYPEUNSPECIFIED => "RESPONSE_MEDIA_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_MEDIA_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum::RESPONSEMEDIATYPEUNSPECIFIED),
           "AUDIO" => Ok(GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the training phrase.
pub enum GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
    

    /// Not specified. This value should never be used.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Examples do not contain @-prefixed entity type names, but example parts can be annotated with entity types.
    ///
    /// "EXAMPLE"
    #[serde(rename="EXAMPLE")]
    EXAMPLE,
    

    /// Templates are not annotated with entity types, but they can contain @-prefixed entity type names as substrings. Template mode has been deprecated. Example mode is the only supported way to create new training phrases. If you have existing training phrases that you've created in template mode, those will continue to work.
    ///
    /// "TEMPLATE"
    #[serde(rename="TEMPLATE")]
    TEMPLATE,
}

impl AsRef<str> for GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum::EXAMPLE => "EXAMPLE",
            GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum::TEMPLATE => "TEMPLATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum::TYPEUNSPECIFIED),
           "EXAMPLE" => Ok(GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum::EXAMPLE),
           "TEMPLATE" => Ok(GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum::TEMPLATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2MessageParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The role of the participant.
pub enum GoogleCloudDialogflowV2MessageParticipantRoleEnum {
    

    /// Participant role not set.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Participant is a human agent.
    ///
    /// "HUMAN_AGENT"
    #[serde(rename="HUMAN_AGENT")]
    HUMANAGENT,
    

    /// Participant is an automated agent, such as a Dialogflow agent.
    ///
    /// "AUTOMATED_AGENT"
    #[serde(rename="AUTOMATED_AGENT")]
    AUTOMATEDAGENT,
    

    /// Participant is an end user that has called or chatted with Dialogflow services.
    ///
    /// "END_USER"
    #[serde(rename="END_USER")]
    ENDUSER,
}

impl AsRef<str> for GoogleCloudDialogflowV2MessageParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2MessageParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2MessageParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2MessageParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2MessageParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2MessageParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2MessageParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2MessageParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2MessageParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2MessageParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2MessageParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Format of message.
pub enum GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum {
    

    /// If it is unspecified, PROTO will be used.
    ///
    /// "MESSAGE_FORMAT_UNSPECIFIED"
    #[serde(rename="MESSAGE_FORMAT_UNSPECIFIED")]
    MESSAGEFORMATUNSPECIFIED,
    

    /// Pub/Sub message will be serialized proto.
    ///
    /// "PROTO"
    #[serde(rename="PROTO")]
    PROTO,
    

    /// Pub/Sub message will be json.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum::MESSAGEFORMATUNSPECIFIED => "MESSAGE_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum::PROTO => "PROTO",
            GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum::MESSAGEFORMATUNSPECIFIED),
           "PROTO" => Ok(GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum::PROTO),
           "JSON" => Ok(GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2NotificationConfigMessageFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the synthesized audio content.
pub enum GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum {
    

    /// Not specified.
    ///
    /// "OUTPUT_AUDIO_ENCODING_UNSPECIFIED"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_UNSPECIFIED")]
    OUTPUTAUDIOENCODINGUNSPECIFIED,
    

    /// Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header.
    ///
    /// "OUTPUT_AUDIO_ENCODING_LINEAR_16"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_LINEAR_16")]
    OUTPUTAUDIOENCODINGLINEAR16,
    

    /// MP3 audio at 32kbps.
    ///
    /// "OUTPUT_AUDIO_ENCODING_MP3"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_MP3")]
    OUTPUTAUDIOENCODINGMP3,
    

    /// MP3 audio at 64kbps.
    ///
    /// "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_MP3_64_KBPS")]
    OUTPUTAUDIOENCODINGMP364KBPS,
    

    /// Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate.
    ///
    /// "OUTPUT_AUDIO_ENCODING_OGG_OPUS"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_OGG_OPUS")]
    OUTPUTAUDIOENCODINGOGGOPUS,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    ///
    /// "OUTPUT_AUDIO_ENCODING_MULAW"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_MULAW")]
    OUTPUTAUDIOENCODINGMULAW,
}

impl AsRef<str> for GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3 => "OUTPUT_AUDIO_ENCODING_MP3",
            GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS => "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS",
            GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS => "OUTPUT_AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW => "OUTPUT_AUDIO_ENCODING_MULAW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED),
           "OUTPUT_AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16),
           "OUTPUT_AUDIO_ENCODING_MP3" => Ok(GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3),
           "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS" => Ok(GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS),
           "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS),
           "OUTPUT_AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2OutputAudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable.
pub enum GoogleCloudDialogflowV2ParticipantRoleEnum {
    

    /// Participant role not set.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Participant is a human agent.
    ///
    /// "HUMAN_AGENT"
    #[serde(rename="HUMAN_AGENT")]
    HUMANAGENT,
    

    /// Participant is an automated agent, such as a Dialogflow agent.
    ///
    /// "AUTOMATED_AGENT"
    #[serde(rename="AUTOMATED_AGENT")]
    AUTOMATEDAGENT,
    

    /// Participant is an end user that has called or chatted with Dialogflow services.
    ///
    /// "END_USER"
    #[serde(rename="END_USER")]
    ENDUSER,
}

impl AsRef<str> for GoogleCloudDialogflowV2ParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2ParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2ParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2ParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2ParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2ParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2ParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the answer.
pub enum GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::ANSWERTYPEUNSPECIFIED => "ANSWER_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::GENERATIVE => "GENERATIVE",
            GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::INTENT => "INTENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANSWER_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::ANSWERTYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::FAQ),
           "GENERATIVE" => Ok(GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::GENERATIVE),
           "INTENT" => Ok(GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum::INTENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2SearchKnowledgeAnswerAnswerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether the additional data should override or supplement the custom entity type definition.
pub enum GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
    

    /// Not specified. This value should be never used.
    ///
    /// "ENTITY_OVERRIDE_MODE_UNSPECIFIED"
    #[serde(rename="ENTITY_OVERRIDE_MODE_UNSPECIFIED")]
    ENTITYOVERRIDEMODEUNSPECIFIED,
    

    /// The collection of session entities overrides the collection of entities in the corresponding custom entity type.
    ///
    /// "ENTITY_OVERRIDE_MODE_OVERRIDE"
    #[serde(rename="ENTITY_OVERRIDE_MODE_OVERRIDE")]
    ENTITYOVERRIDEMODEOVERRIDE,
    

    /// The collection of session entities extends the collection of entities in the corresponding custom entity type. Note: Even in this override mode calls to `ListSessionEntityTypes`, `GetSessionEntityType`, `CreateSessionEntityType` and `UpdateSessionEntityType` only return the additional entities added in this session entity type. If you want to get the supplemented list, please call EntityTypes.GetEntityType on the custom entity type and merge.
    ///
    /// "ENTITY_OVERRIDE_MODE_SUPPLEMENT"
    #[serde(rename="ENTITY_OVERRIDE_MODE_SUPPLEMENT")]
    ENTITYOVERRIDEMODESUPPLEMENT,
}

impl AsRef<str> for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEUNSPECIFIED => "ENTITY_OVERRIDE_MODE_UNSPECIFIED",
            GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEOVERRIDE => "ENTITY_OVERRIDE_MODE_OVERRIDE",
            GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODESUPPLEMENT => "ENTITY_OVERRIDE_MODE_SUPPLEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_OVERRIDE_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEUNSPECIFIED),
           "ENTITY_OVERRIDE_MODE_OVERRIDE" => Ok(GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEOVERRIDE),
           "ENTITY_OVERRIDE_MODE_SUPPLEMENT" => Ok(GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODESUPPLEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The participant role to add or update the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
pub enum GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    

    /// Participant role not set.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Participant is a human agent.
    ///
    /// "HUMAN_AGENT"
    #[serde(rename="HUMAN_AGENT")]
    HUMANAGENT,
    

    /// Participant is an automated agent, such as a Dialogflow agent.
    ///
    /// "AUTOMATED_AGENT"
    #[serde(rename="AUTOMATED_AGENT")]
    AUTOMATEDAGENT,
    

    /// Participant is an end user that has called or chatted with Dialogflow services.
    ///
    /// "END_USER"
    #[serde(rename="END_USER")]
    ENDUSER,
}

impl AsRef<str> for GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of the smart reply model. If not provided, model_type is used.
pub enum GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum {
    

    /// ModelType unspecified.
    ///
    /// "MODEL_TYPE_UNSPECIFIED"
    #[serde(rename="MODEL_TYPE_UNSPECIFIED")]
    MODELTYPEUNSPECIFIED,
    

    /// ModelType smart reply dual encoder model.
    ///
    /// "SMART_REPLY_DUAL_ENCODER_MODEL"
    #[serde(rename="SMART_REPLY_DUAL_ENCODER_MODEL")]
    SMARTREPLYDUALENCODERMODEL,
    

    /// ModelType smart reply bert model.
    ///
    /// "SMART_REPLY_BERT_MODEL"
    #[serde(rename="SMART_REPLY_BERT_MODEL")]
    SMARTREPLYBERTMODEL,
}

impl AsRef<str> for GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum::MODELTYPEUNSPECIFIED => "MODEL_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum::SMARTREPLYDUALENCODERMODEL => "SMART_REPLY_DUAL_ENCODER_MODEL",
            GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum::SMARTREPLYBERTMODEL => "SMART_REPLY_BERT_MODEL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum::MODELTYPEUNSPECIFIED),
           "SMART_REPLY_DUAL_ENCODER_MODEL" => Ok(GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum::SMARTREPLYDUALENCODERMODEL),
           "SMART_REPLY_BERT_MODEL" => Ok(GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum::SMARTREPLYBERTMODEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2SmartReplyModelMetadataTrainingModelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The speech model used in speech to text. `SPEECH_MODEL_VARIANT_UNSPECIFIED`, `USE_BEST_AVAILABLE` will be treated as `USE_ENHANCED`. It can be overridden in AnalyzeContentRequest and StreamingAnalyzeContentRequest request. If enhanced model variant is specified and an enhanced version of the specified model for the language does not exist, then it would emit an error.
pub enum GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum {
    

    /// No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE.
    ///
    /// "SPEECH_MODEL_VARIANT_UNSPECIFIED"
    #[serde(rename="SPEECH_MODEL_VARIANT_UNSPECIFIED")]
    SPEECHMODELVARIANTUNSPECIFIED,
    

    /// Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models.
    ///
    /// "USE_BEST_AVAILABLE"
    #[serde(rename="USE_BEST_AVAILABLE")]
    USEBESTAVAILABLE,
    

    /// Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models.
    ///
    /// "USE_STANDARD"
    #[serde(rename="USE_STANDARD")]
    USESTANDARD,
    

    /// Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible.
    ///
    /// "USE_ENHANCED"
    #[serde(rename="USE_ENHANCED")]
    USEENHANCED,
}

impl AsRef<str> for GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED => "SPEECH_MODEL_VARIANT_UNSPECIFIED",
            GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::USEBESTAVAILABLE => "USE_BEST_AVAILABLE",
            GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::USESTANDARD => "USE_STANDARD",
            GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::USEENHANCED => "USE_ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPEECH_MODEL_VARIANT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED),
           "USE_BEST_AVAILABLE" => Ok(GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::USEBESTAVAILABLE),
           "USE_STANDARD" => Ok(GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::USESTANDARD),
           "USE_ENHANCED" => Ok(GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum::USEENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2SpeechToTextConfigSpeechModelVariantEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2SuggestionFeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of Human Agent Assistant API feature to request.
pub enum GoogleCloudDialogflowV2SuggestionFeatureTypeEnum {
    

    /// Unspecified feature type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Run article suggestion model for chat.
    ///
    /// "ARTICLE_SUGGESTION"
    #[serde(rename="ARTICLE_SUGGESTION")]
    ARTICLESUGGESTION,
    

    /// Run FAQ model for chat.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// Run smart reply model for chat.
    ///
    /// "SMART_REPLY"
    #[serde(rename="SMART_REPLY")]
    SMARTREPLY,
    

    /// Run knowledge search with text input from agent or text generated query.
    ///
    /// "KNOWLEDGE_SEARCH"
    #[serde(rename="KNOWLEDGE_SEARCH")]
    KNOWLEDGESEARCH,
}

impl AsRef<str> for GoogleCloudDialogflowV2SuggestionFeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::SMARTREPLY => "SMART_REPLY",
            GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::KNOWLEDGESEARCH => "KNOWLEDGE_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2SuggestionFeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::TYPEUNSPECIFIED),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::ARTICLESUGGESTION),
           "FAQ" => Ok(GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::FAQ),
           "SMART_REPLY" => Ok(GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::SMARTREPLY),
           "KNOWLEDGE_SEARCH" => Ok(GoogleCloudDialogflowV2SuggestionFeatureTypeEnum::KNOWLEDGESEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2SuggestionFeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the synthesized audio content.
pub enum GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum {
    

    /// Not specified.
    ///
    /// "OUTPUT_AUDIO_ENCODING_UNSPECIFIED"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_UNSPECIFIED")]
    OUTPUTAUDIOENCODINGUNSPECIFIED,
    

    /// Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header.
    ///
    /// "OUTPUT_AUDIO_ENCODING_LINEAR_16"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_LINEAR_16")]
    OUTPUTAUDIOENCODINGLINEAR16,
    

    /// MP3 audio at 32kbps.
    ///
    /// "OUTPUT_AUDIO_ENCODING_MP3"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_MP3")]
    OUTPUTAUDIOENCODINGMP3,
    

    /// MP3 audio at 64kbps.
    ///
    /// "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_MP3_64_KBPS")]
    OUTPUTAUDIOENCODINGMP364KBPS,
    

    /// Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate.
    ///
    /// "OUTPUT_AUDIO_ENCODING_OGG_OPUS"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_OGG_OPUS")]
    OUTPUTAUDIOENCODINGOGGOPUS,
    

    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    ///
    /// "OUTPUT_AUDIO_ENCODING_MULAW"
    #[serde(rename="OUTPUT_AUDIO_ENCODING_MULAW")]
    OUTPUTAUDIOENCODINGMULAW,
}

impl AsRef<str> for GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3 => "OUTPUT_AUDIO_ENCODING_MP3",
            GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS => "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS",
            GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS => "OUTPUT_AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW => "OUTPUT_AUDIO_ENCODING_MULAW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED),
           "OUTPUT_AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16),
           "OUTPUT_AUDIO_ENCODING_MP3" => Ok(GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3),
           "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS" => Ok(GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS),
           "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS),
           "OUTPUT_AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2TextToSpeechSettingOutputAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2ValidationErrorSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the error.
pub enum GoogleCloudDialogflowV2ValidationErrorSeverityEnum {
    

    /// Not specified. This value should never be used.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// The agent doesn't follow Dialogflow best practices.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// The agent may not behave as expected.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// The agent may experience partial failures.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The agent may completely fail.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for GoogleCloudDialogflowV2ValidationErrorSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2ValidationErrorSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudDialogflowV2ValidationErrorSeverityEnum::INFO => "INFO",
            GoogleCloudDialogflowV2ValidationErrorSeverityEnum::WARNING => "WARNING",
            GoogleCloudDialogflowV2ValidationErrorSeverityEnum::ERROR => "ERROR",
            GoogleCloudDialogflowV2ValidationErrorSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2ValidationErrorSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2ValidationErrorSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(GoogleCloudDialogflowV2ValidationErrorSeverityEnum::INFO),
           "WARNING" => Ok(GoogleCloudDialogflowV2ValidationErrorSeverityEnum::WARNING),
           "ERROR" => Ok(GoogleCloudDialogflowV2ValidationErrorSeverityEnum::ERROR),
           "CRITICAL" => Ok(GoogleCloudDialogflowV2ValidationErrorSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2ValidationErrorSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2VersionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status of this version. This field is read-only and cannot be set by create and update methods.
pub enum GoogleCloudDialogflowV2VersionStatusEnum {
    

    /// Not specified. This value is not used.
    ///
    /// "VERSION_STATUS_UNSPECIFIED"
    #[serde(rename="VERSION_STATUS_UNSPECIFIED")]
    VERSIONSTATUSUNSPECIFIED,
    

    /// Version is not ready to serve (e.g. training is in progress).
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Version is ready to serve.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Version training failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleCloudDialogflowV2VersionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2VersionStatusEnum::VERSIONSTATUSUNSPECIFIED => "VERSION_STATUS_UNSPECIFIED",
            GoogleCloudDialogflowV2VersionStatusEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudDialogflowV2VersionStatusEnum::READY => "READY",
            GoogleCloudDialogflowV2VersionStatusEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2VersionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_STATUS_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2VersionStatusEnum::VERSIONSTATUSUNSPECIFIED),
           "IN_PROGRESS" => Ok(GoogleCloudDialogflowV2VersionStatusEnum::INPROGRESS),
           "READY" => Ok(GoogleCloudDialogflowV2VersionStatusEnum::READY),
           "FAILED" => Ok(GoogleCloudDialogflowV2VersionStatusEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2VersionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request.
pub enum GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum {
    

    /// An unspecified gender, which means that the client doesn't care which gender the selected voice will have.
    ///
    /// "SSML_VOICE_GENDER_UNSPECIFIED"
    #[serde(rename="SSML_VOICE_GENDER_UNSPECIFIED")]
    SSMLVOICEGENDERUNSPECIFIED,
    

    /// A male voice.
    ///
    /// "SSML_VOICE_GENDER_MALE"
    #[serde(rename="SSML_VOICE_GENDER_MALE")]
    SSMLVOICEGENDERMALE,
    

    /// A female voice.
    ///
    /// "SSML_VOICE_GENDER_FEMALE"
    #[serde(rename="SSML_VOICE_GENDER_FEMALE")]
    SSMLVOICEGENDERFEMALE,
    

    /// A gender-neutral voice.
    ///
    /// "SSML_VOICE_GENDER_NEUTRAL"
    #[serde(rename="SSML_VOICE_GENDER_NEUTRAL")]
    SSMLVOICEGENDERNEUTRAL,
}

impl AsRef<str> for GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED => "SSML_VOICE_GENDER_UNSPECIFIED",
            GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERMALE => "SSML_VOICE_GENDER_MALE",
            GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERFEMALE => "SSML_VOICE_GENDER_FEMALE",
            GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERNEUTRAL => "SSML_VOICE_GENDER_NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSML_VOICE_GENDER_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED),
           "SSML_VOICE_GENDER_MALE" => Ok(GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERMALE),
           "SSML_VOICE_GENDER_FEMALE" => Ok(GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERFEMALE),
           "SSML_VOICE_GENDER_NEUTRAL" => Ok(GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERNEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2VoiceSelectionParamSsmlGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectIntentViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The resource view to apply to the returned intent.
pub enum ProjectIntentViewEnum {
    

    /// Training phrases field is not populated in the response.
    ///
    /// "INTENT_VIEW_UNSPECIFIED"
    #[serde(rename="INTENT_VIEW_UNSPECIFIED")]
    INTENTVIEWUNSPECIFIED,
    

    /// All fields are populated.
    ///
    /// "INTENT_VIEW_FULL"
    #[serde(rename="INTENT_VIEW_FULL")]
    INTENTVIEWFULL,
}

impl AsRef<str> for ProjectIntentViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectIntentViewEnum::INTENTVIEWUNSPECIFIED => "INTENT_VIEW_UNSPECIFIED",
            ProjectIntentViewEnum::INTENTVIEWFULL => "INTENT_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectIntentViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTENT_VIEW_UNSPECIFIED" => Ok(ProjectIntentViewEnum::INTENTVIEWUNSPECIFIED),
           "INTENT_VIEW_FULL" => Ok(ProjectIntentViewEnum::INTENTVIEWFULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectIntentViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


