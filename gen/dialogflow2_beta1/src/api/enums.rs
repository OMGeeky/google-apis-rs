use super::*;



// region GoogleCloudDialogflowV2beta1AgentApiVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. API version displayed in Dialogflow console. If not specified, V2 API is assumed. Clients are free to query different service endpoints for different API versions. However, bots connectors and webhook calls will follow the specified API version.
pub enum GoogleCloudDialogflowV2beta1AgentApiVersionEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AgentApiVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONUNSPECIFIED => "API_VERSION_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONV1 => "API_VERSION_V1",
            GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONV2 => "API_VERSION_V2",
            GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONV2BETA1 => "API_VERSION_V2_BETA_1",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AgentApiVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "API_VERSION_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONUNSPECIFIED),
           "API_VERSION_V1" => Ok(GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONV1),
           "API_VERSION_V2" => Ok(GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONV2),
           "API_VERSION_V2_BETA_1" => Ok(GoogleCloudDialogflowV2beta1AgentApiVersionEnum::APIVERSIONV2BETA1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AgentApiVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AgentMatchModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Determines how intents are detected from user queries.
pub enum GoogleCloudDialogflowV2beta1AgentMatchModeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AgentMatchModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AgentMatchModeEnum::MATCHMODEUNSPECIFIED => "MATCH_MODE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AgentMatchModeEnum::MATCHMODEHYBRID => "MATCH_MODE_HYBRID",
            GoogleCloudDialogflowV2beta1AgentMatchModeEnum::MATCHMODEMLONLY => "MATCH_MODE_ML_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AgentMatchModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCH_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AgentMatchModeEnum::MATCHMODEUNSPECIFIED),
           "MATCH_MODE_HYBRID" => Ok(GoogleCloudDialogflowV2beta1AgentMatchModeEnum::MATCHMODEHYBRID),
           "MATCH_MODE_ML_ONLY" => Ok(GoogleCloudDialogflowV2beta1AgentMatchModeEnum::MATCHMODEMLONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AgentMatchModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AgentTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The agent tier. If not specified, TIER_STANDARD is assumed.
pub enum GoogleCloudDialogflowV2beta1AgentTierEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AgentTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AgentTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AgentTierEnum::TIERSTANDARD => "TIER_STANDARD",
            GoogleCloudDialogflowV2beta1AgentTierEnum::TIERENTERPRISE => "TIER_ENTERPRISE",
            GoogleCloudDialogflowV2beta1AgentTierEnum::TIERENTERPRISEPLUS => "TIER_ENTERPRISE_PLUS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AgentTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AgentTierEnum::TIERUNSPECIFIED),
           "TIER_STANDARD" => Ok(GoogleCloudDialogflowV2beta1AgentTierEnum::TIERSTANDARD),
           "TIER_ENTERPRISE" => Ok(GoogleCloudDialogflowV2beta1AgentTierEnum::TIERENTERPRISE),
           "TIER_ENTERPRISE_PLUS" => Ok(GoogleCloudDialogflowV2beta1AgentTierEnum::TIERENTERPRISEPLUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AgentTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the suggested answer is relevant. For example: * Query: "Can I change my mailing address?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * answer_relevance: AnswerRelevance.IRRELEVANT
pub enum GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum::ANSWERRELEVANCEUNSPECIFIED => "ANSWER_RELEVANCE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum::IRRELEVANT => "IRRELEVANT",
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum::RELEVANT => "RELEVANT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANSWER_RELEVANCE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum::ANSWERRELEVANCEUNSPECIFIED),
           "IRRELEVANT" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum::IRRELEVANT),
           "RELEVANT" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum::RELEVANT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AgentAssistantFeedbackAnswerRelevanceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the information in the document is correct. For example: * Query: "Can I return the package in 2 days once received?" * Suggested document says: "Items must be returned/exchanged within 60 days of the purchase date." * Ground truth: "No return or exchange is allowed." * [document_correctness]: INCORRECT
pub enum GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum::DOCUMENTCORRECTNESSUNSPECIFIED => "DOCUMENT_CORRECTNESS_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum::INCORRECT => "INCORRECT",
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum::CORRECT => "CORRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOCUMENT_CORRECTNESS_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum::DOCUMENTCORRECTNESSUNSPECIFIED),
           "INCORRECT" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum::INCORRECT),
           "CORRECT" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum::CORRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentCorrectnessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the suggested document is efficient. For example, if the document is poorly written, hard to understand, hard to use or too long to find useful information, document_efficiency is DocumentEfficiency.INEFFICIENT.
pub enum GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum::DOCUMENTEFFICIENCYUNSPECIFIED => "DOCUMENT_EFFICIENCY_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum::INEFFICIENT => "INEFFICIENT",
            GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum::EFFICIENT => "EFFICIENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOCUMENT_EFFICIENCY_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum::DOCUMENTEFFICIENCYUNSPECIFIED),
           "INEFFICIENT" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum::INEFFICIENT),
           "EFFICIENT" => Ok(GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum::EFFICIENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AgentAssistantFeedbackDocumentEfficiencyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The correctness level of the specific answer.
pub enum GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::CORRECTNESSLEVELUNSPECIFIED => "CORRECTNESS_LEVEL_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::NOTCORRECT => "NOT_CORRECT",
            GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::PARTIALLYCORRECT => "PARTIALLY_CORRECT",
            GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::FULLYCORRECT => "FULLY_CORRECT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CORRECTNESS_LEVEL_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::CORRECTNESSLEVELUNSPECIFIED),
           "NOT_CORRECT" => Ok(GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::NOTCORRECT),
           "PARTIALLY_CORRECT" => Ok(GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::PARTIALLYCORRECT),
           "FULLY_CORRECT" => Ok(GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum::FULLYCORRECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AnswerFeedbackCorrectnessLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// AutomatedAgentReply type.
pub enum GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum::AUTOMATEDAGENTREPLYTYPEUNSPECIFIED => "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum::PARTIAL => "PARTIAL",
            GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum::FINAL => "FINAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTOMATED_AGENT_REPLY_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum::AUTOMATEDAGENTREPLYTYPEUNSPECIFIED),
           "PARTIAL" => Ok(GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum::PARTIAL),
           "FINAL" => Ok(GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum::FINAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1AutomatedAgentReplyAutomatedAgentReplyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The resource view to apply to the returned intent.
pub enum GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWUNSPECIFIED => "INTENT_VIEW_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWFULL => "INTENT_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTENT_VIEW_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWUNSPECIFIED),
           "INTENT_VIEW_FULL" => Ok(GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum::INTENTVIEWFULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1BatchUpdateIntentsRequestIntentViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The participant role to remove the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
pub enum GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the suggestion feature to remove.
pub enum GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    

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
    

    /// Run FAQ model.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// Run smart reply model for chat.
    ///
    /// "SMART_REPLY"
    #[serde(rename="SMART_REPLY")]
    SMARTREPLY,
    

    /// Run Dialogflow assist model for chat, which will return automated agent response as suggestion.
    ///
    /// "DIALOGFLOW_ASSIST"
    #[serde(rename="DIALOGFLOW_ASSIST")]
    DIALOGFLOWASSIST,
    

    /// Run conversation summarization model for chat.
    ///
    /// "CONVERSATION_SUMMARIZATION"
    #[serde(rename="CONVERSATION_SUMMARIZATION")]
    CONVERSATIONSUMMARIZATION,
    

    /// Run knowledge search with text input from agent or text generated query.
    ///
    /// "KNOWLEDGE_SEARCH"
    #[serde(rename="KNOWLEDGE_SEARCH")]
    KNOWLEDGESEARCH,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::SMARTREPLY => "SMART_REPLY",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::DIALOGFLOWASSIST => "DIALOGFLOW_ASSIST",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::CONVERSATIONSUMMARIZATION => "CONVERSATION_SUMMARIZATION",
            GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::KNOWLEDGESEARCH => "KNOWLEDGE_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::TYPEUNSPECIFIED),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::ARTICLESUGGESTION),
           "FAQ" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::FAQ),
           "SMART_REPLY" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::SMARTREPLY),
           "DIALOGFLOW_ASSIST" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::DIALOGFLOWASSIST),
           "CONVERSATION_SUMMARIZATION" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::CONVERSATIONSUMMARIZATION),
           "KNOWLEDGE_SEARCH" => Ok(GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum::KNOWLEDGESEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ClearSuggestionFeatureConfigRequestSuggestionFeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ConversationConversationStageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The stage of a conversation. It indicates whether the virtual agent or a human agent is handling the conversation. If the conversation is created with the conversation profile that has Dialogflow config set, defaults to ConversationStage.VIRTUAL_AGENT_STAGE; Otherwise, defaults to ConversationStage.HUMAN_ASSIST_STAGE. If the conversation is created with the conversation profile that has Dialogflow config set but explicitly sets conversation_stage to ConversationStage.HUMAN_ASSIST_STAGE, it skips ConversationStage.VIRTUAL_AGENT_STAGE stage and directly goes to ConversationStage.HUMAN_ASSIST_STAGE.
pub enum GoogleCloudDialogflowV2beta1ConversationConversationStageEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1ConversationConversationStageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ConversationConversationStageEnum::CONVERSATIONSTAGEUNSPECIFIED => "CONVERSATION_STAGE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ConversationConversationStageEnum::VIRTUALAGENTSTAGE => "VIRTUAL_AGENT_STAGE",
            GoogleCloudDialogflowV2beta1ConversationConversationStageEnum::HUMANASSISTSTAGE => "HUMAN_ASSIST_STAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ConversationConversationStageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONVERSATION_STAGE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ConversationConversationStageEnum::CONVERSATIONSTAGEUNSPECIFIED),
           "VIRTUAL_AGENT_STAGE" => Ok(GoogleCloudDialogflowV2beta1ConversationConversationStageEnum::VIRTUALAGENTSTAGE),
           "HUMAN_ASSIST_STAGE" => Ok(GoogleCloudDialogflowV2beta1ConversationConversationStageEnum::HUMANASSISTSTAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ConversationConversationStageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the Conversation.
pub enum GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED => "LIFECYCLE_STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum::COMPLETED => "COMPLETED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIFECYCLE_STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum::LIFECYCLESTATEUNSPECIFIED),
           "IN_PROGRESS" => Ok(GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum::INPROGRESS),
           "COMPLETED" => Ok(GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum::COMPLETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ConversationLifecycleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The knowledge type of document content.
pub enum GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum {
    

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
    

    /// The legacy enum for agent-facing smart reply feature.
    ///
    /// "SMART_REPLY"
    #[serde(rename="SMART_REPLY")]
    SMARTREPLY,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED => "KNOWLEDGE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::EXTRACTIVEQA => "EXTRACTIVE_QA",
            GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::AGENTFACINGSMARTREPLY => "AGENT_FACING_SMART_REPLY",
            GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::SMARTREPLY => "SMART_REPLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KNOWLEDGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::FAQ),
           "EXTRACTIVE_QA" => Ok(GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::EXTRACTIVEQA),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::ARTICLESUGGESTION),
           "AGENT_FACING_SMART_REPLY" => Ok(GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::AGENTFACINGSMARTREPLY),
           "SMART_REPLY" => Ok(GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum::SMARTREPLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1DocumentKnowledgeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1DocumentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the document.
pub enum GoogleCloudDialogflowV2beta1DocumentStateEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1DocumentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1DocumentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1DocumentStateEnum::CREATING => "CREATING",
            GoogleCloudDialogflowV2beta1DocumentStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDialogflowV2beta1DocumentStateEnum::UPDATING => "UPDATING",
            GoogleCloudDialogflowV2beta1DocumentStateEnum::RELOADING => "RELOADING",
            GoogleCloudDialogflowV2beta1DocumentStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1DocumentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1DocumentStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleCloudDialogflowV2beta1DocumentStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleCloudDialogflowV2beta1DocumentStateEnum::ACTIVE),
           "UPDATING" => Ok(GoogleCloudDialogflowV2beta1DocumentStateEnum::UPDATING),
           "RELOADING" => Ok(GoogleCloudDialogflowV2beta1DocumentStateEnum::RELOADING),
           "DELETING" => Ok(GoogleCloudDialogflowV2beta1DocumentStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1DocumentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates whether the entity type can be automatically expanded.
pub enum GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEUNSPECIFIED => "AUTO_EXPANSION_MODE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEDEFAULT => "AUTO_EXPANSION_MODE_DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO_EXPANSION_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEUNSPECIFIED),
           "AUTO_EXPANSION_MODE_DEFAULT" => Ok(GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEDEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1EntityTypeKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the kind of entity type.
pub enum GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDUNSPECIFIED => "KIND_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDMAP => "KIND_MAP",
            GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDLIST => "KIND_LIST",
            GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDREGEXP => "KIND_REGEXP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KIND_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDUNSPECIFIED),
           "KIND_MAP" => Ok(GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDMAP),
           "KIND_LIST" => Ok(GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDLIST),
           "KIND_REGEXP" => Ok(GoogleCloudDialogflowV2beta1EntityTypeKindEnum::KINDREGEXP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1EnvironmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this environment. This field is read-only, i.e., it cannot be set by create and update methods.
pub enum GoogleCloudDialogflowV2beta1EnvironmentStateEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1EnvironmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1EnvironmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1EnvironmentStateEnum::STOPPED => "STOPPED",
            GoogleCloudDialogflowV2beta1EnvironmentStateEnum::LOADING => "LOADING",
            GoogleCloudDialogflowV2beta1EnvironmentStateEnum::RUNNING => "RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1EnvironmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1EnvironmentStateEnum::STATEUNSPECIFIED),
           "STOPPED" => Ok(GoogleCloudDialogflowV2beta1EnvironmentStateEnum::STOPPED),
           "LOADING" => Ok(GoogleCloudDialogflowV2beta1EnvironmentStateEnum::LOADING),
           "RUNNING" => Ok(GoogleCloudDialogflowV2beta1EnvironmentStateEnum::RUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1EnvironmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the feature that enabled for fulfillment.
pub enum GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum::SMALLTALK => "SMALLTALK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum::TYPEUNSPECIFIED),
           "SMALLTALK" => Ok(GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum::SMALLTALK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1FulfillmentFeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The selected sections chosen to return when requesting a summary of a conversation. A duplicate selected section will be treated as a single selected section. If section types are not provided, the default will be {SITUATION, ACTION, RESULT}.
pub enum GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SECTIONTYPEUNSPECIFIED => "SECTION_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SITUATION => "SITUATION",
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ACTION => "ACTION",
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::RESOLUTION => "RESOLUTION",
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::REASONFORCANCELLATION => "REASON_FOR_CANCELLATION",
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::CUSTOMERSATISFACTION => "CUSTOMER_SATISFACTION",
            GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ENTITIES => "ENTITIES",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SECTIONTYPEUNSPECIFIED),
           "SITUATION" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::SITUATION),
           "ACTION" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ACTION),
           "RESOLUTION" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::RESOLUTION),
           "REASON_FOR_CANCELLATION" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::REASONFORCANCELLATION),
           "CUSTOMER_SATISFACTION" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::CUSTOMERSATISFACTION),
           "ENTITIES" => Ok(GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum::ENTITIES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1HumanAgentAssistantConfigSuggestionQueryConfigSectionSectionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The knowledge type of document content.
pub enum GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum {
    

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
    

    /// The legacy enum for agent-facing smart reply feature.
    ///
    /// "SMART_REPLY"
    #[serde(rename="SMART_REPLY")]
    SMARTREPLY,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED => "KNOWLEDGE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::EXTRACTIVEQA => "EXTRACTIVE_QA",
            GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::AGENTFACINGSMARTREPLY => "AGENT_FACING_SMART_REPLY",
            GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::SMARTREPLY => "SMART_REPLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KNOWLEDGE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::KNOWLEDGETYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::FAQ),
           "EXTRACTIVE_QA" => Ok(GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::EXTRACTIVEQA),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::ARTICLESUGGESTION),
           "AGENT_FACING_SMART_REPLY" => Ok(GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::AGENTFACINGSMARTREPLY),
           "SMART_REPLY" => Ok(GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum::SMARTREPLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ImportDocumentTemplateKnowledgeTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the audio content to process.
pub enum GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED => "AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGLINEAR16 => "AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGFLAC => "AUDIO_ENCODING_FLAC",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGMULAW => "AUDIO_ENCODING_MULAW",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMR => "AUDIO_ENCODING_AMR",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMRWB => "AUDIO_ENCODING_AMR_WB",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGOGGOPUS => "AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGSPEEXWITHHEADERBYTE => "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED),
           "AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGLINEAR16),
           "AUDIO_ENCODING_FLAC" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGFLAC),
           "AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGMULAW),
           "AUDIO_ENCODING_AMR" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMR),
           "AUDIO_ENCODING_AMR_WB" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMRWB),
           "AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGOGGOPUS),
           "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum::AUDIOENCODINGSPEEXWITHHEADERBYTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1InputAudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which variant of the Speech model to use.
pub enum GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED => "SPEECH_MODEL_VARIANT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::USEBESTAVAILABLE => "USE_BEST_AVAILABLE",
            GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::USESTANDARD => "USE_STANDARD",
            GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::USEENHANCED => "USE_ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPEECH_MODEL_VARIANT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED),
           "USE_BEST_AVAILABLE" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::USEBESTAVAILABLE),
           "USE_STANDARD" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::USESTANDARD),
           "USE_ENHANCED" => Ok(GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum::USEENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1InputAudioConfigModelVariantEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform).
pub enum GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
    

    /// Not specified.
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
    

    /// Telephony Gateway.
    ///
    /// "TELEPHONY"
    #[serde(rename="TELEPHONY")]
    TELEPHONY,
    

    /// Google Hangouts.
    ///
    /// "GOOGLE_HANGOUTS"
    #[serde(rename="GOOGLE_HANGOUTS")]
    GOOGLEHANGOUTS,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::FACEBOOK => "FACEBOOK",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::SLACK => "SLACK",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::TELEGRAM => "TELEGRAM",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::KIK => "KIK",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::SKYPE => "SKYPE",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::LINE => "LINE",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::VIBER => "VIBER",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::ACTIONSONGOOGLE => "ACTIONS_ON_GOOGLE",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::TELEPHONY => "TELEPHONY",
            GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::GOOGLEHANGOUTS => "GOOGLE_HANGOUTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::PLATFORMUNSPECIFIED),
           "FACEBOOK" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::FACEBOOK),
           "SLACK" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::SLACK),
           "TELEGRAM" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::TELEGRAM),
           "KIK" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::KIK),
           "SKYPE" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::SKYPE),
           "LINE" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::LINE),
           "VIBER" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::VIBER),
           "ACTIONS_ON_GOOGLE" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::ACTIONSONGOOGLE),
           "TELEPHONY" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::TELEPHONY),
           "GOOGLE_HANGOUTS" => Ok(GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum::GOOGLEHANGOUTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentWebhookStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates whether webhooks are enabled for the intent.
pub enum GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentWebhookStateEnum::WEBHOOKSTATEUNSPECIFIED => "WEBHOOK_STATE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentWebhookStateEnum::WEBHOOKSTATEENABLED => "WEBHOOK_STATE_ENABLED",
            GoogleCloudDialogflowV2beta1IntentWebhookStateEnum::WEBHOOKSTATEENABLEDFORSLOTFILLING => "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEBHOOK_STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentWebhookStateEnum::WEBHOOKSTATEUNSPECIFIED),
           "WEBHOOK_STATE_ENABLED" => Ok(GoogleCloudDialogflowV2beta1IntentWebhookStateEnum::WEBHOOKSTATEENABLED),
           "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING" => Ok(GoogleCloudDialogflowV2beta1IntentWebhookStateEnum::WEBHOOKSTATEENABLEDFORSLOTFILLING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The platform that this message is intended for.
pub enum GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
    

    /// Not specified.
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
    

    /// Telephony Gateway.
    ///
    /// "TELEPHONY"
    #[serde(rename="TELEPHONY")]
    TELEPHONY,
    

    /// Google Hangouts.
    ///
    /// "GOOGLE_HANGOUTS"
    #[serde(rename="GOOGLE_HANGOUTS")]
    GOOGLEHANGOUTS,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::FACEBOOK => "FACEBOOK",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::SLACK => "SLACK",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::TELEGRAM => "TELEGRAM",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::KIK => "KIK",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::SKYPE => "SKYPE",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::LINE => "LINE",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::VIBER => "VIBER",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::ACTIONSONGOOGLE => "ACTIONS_ON_GOOGLE",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::TELEPHONY => "TELEPHONY",
            GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::GOOGLEHANGOUTS => "GOOGLE_HANGOUTS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::PLATFORMUNSPECIFIED),
           "FACEBOOK" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::FACEBOOK),
           "SLACK" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::SLACK),
           "TELEGRAM" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::TELEGRAM),
           "KIK" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::KIK),
           "SKYPE" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::SKYPE),
           "LINE" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::LINE),
           "VIBER" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::VIBER),
           "ACTIONS_ON_GOOGLE" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::ACTIONSONGOOGLE),
           "TELEPHONY" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::TELEPHONY),
           "GOOGLE_HANGOUTS" => Ok(GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum::GOOGLEHANGOUTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Settings for displaying the image. Applies to every image in items.
pub enum GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::IMAGEDISPLAYOPTIONSUNSPECIFIED => "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::GRAY => "GRAY",
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::WHITE => "WHITE",
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::CROPPED => "CROPPED",
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::BLURREDBACKGROUND => "BLURRED_BACKGROUND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::IMAGEDISPLAYOPTIONSUNSPECIFIED),
           "GRAY" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::GRAY),
           "WHITE" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::WHITE),
           "CROPPED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::CROPPED),
           "BLURRED_BACKGROUND" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum::BLURREDBACKGROUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser.
pub enum GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::URLTYPEHINTUNSPECIFIED => "URL_TYPE_HINT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPACTION => "AMP_ACTION",
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPCONTENT => "AMP_CONTENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "URL_TYPE_HINT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::URLTYPEHINTUNSPECIFIED),
           "AMP_ACTION" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPACTION),
           "AMP_CONTENT" => Ok(GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum::AMPCONTENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Defines text alignment for all cells in this column.
pub enum GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::HORIZONTALALIGNMENTUNSPECIFIED => "HORIZONTAL_ALIGNMENT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::LEADING => "LEADING",
            GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::CENTER => "CENTER",
            GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::TRAILING => "TRAILING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::HORIZONTALALIGNMENTUNSPECIFIED),
           "LEADING" => Ok(GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::LEADING),
           "CENTER" => Ok(GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::CENTER),
           "TRAILING" => Ok(GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum::TRAILING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageColumnPropertyHorizontalAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. What type of media is the content (ie "audio").
pub enum GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum::RESPONSEMEDIATYPEUNSPECIFIED => "RESPONSE_MEDIA_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_MEDIA_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum::RESPONSEMEDIATYPEUNSPECIFIED),
           "AUDIO" => Ok(GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required for cards with vertical orientation. The height of the media within a rich card with a vertical layout. For a standalone card with horizontal layout, height is not customizable, and this field is ignored.
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum {
    

    /// Not specified.
    ///
    /// "HEIGHT_UNSPECIFIED"
    #[serde(rename="HEIGHT_UNSPECIFIED")]
    HEIGHTUNSPECIFIED,
    

    /// 112 DP.
    ///
    /// "SHORT"
    #[serde(rename="SHORT")]
    SHORT,
    

    /// 168 DP.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// 264 DP. Not available for rich card carousels when the card width is set to small.
    ///
    /// "TALL"
    #[serde(rename="TALL")]
    TALL,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::HEIGHTUNSPECIFIED => "HEIGHT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::SHORT => "SHORT",
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::MEDIUM => "MEDIUM",
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::TALL => "TALL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEIGHT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::HEIGHTUNSPECIFIED),
           "SHORT" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::SHORT),
           "MEDIUM" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::MEDIUM),
           "TALL" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum::TALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The width of the cards in the carousel.
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum {
    

    /// Not specified.
    ///
    /// "CARD_WIDTH_UNSPECIFIED"
    #[serde(rename="CARD_WIDTH_UNSPECIFIED")]
    CARDWIDTHUNSPECIFIED,
    

    /// 120 DP. Note that tall media cannot be used.
    ///
    /// "SMALL"
    #[serde(rename="SMALL")]
    SMALL,
    

    /// 232 DP.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum::CARDWIDTHUNSPECIFIED => "CARD_WIDTH_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum::SMALL => "SMALL",
            GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum::MEDIUM => "MEDIUM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CARD_WIDTH_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum::CARDWIDTHUNSPECIFIED),
           "SMALL" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum::SMALL),
           "MEDIUM" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum::MEDIUM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Orientation of the card.
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum {
    

    /// Not specified.
    ///
    /// "CARD_ORIENTATION_UNSPECIFIED"
    #[serde(rename="CARD_ORIENTATION_UNSPECIFIED")]
    CARDORIENTATIONUNSPECIFIED,
    

    /// Horizontal layout.
    ///
    /// "HORIZONTAL"
    #[serde(rename="HORIZONTAL")]
    HORIZONTAL,
    

    /// Vertical layout.
    ///
    /// "VERTICAL"
    #[serde(rename="VERTICAL")]
    VERTICAL,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum::CARDORIENTATIONUNSPECIFIED => "CARD_ORIENTATION_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum::HORIZONTAL => "HORIZONTAL",
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum::VERTICAL => "VERTICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CARD_ORIENTATION_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum::CARDORIENTATIONUNSPECIFIED),
           "HORIZONTAL" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum::HORIZONTAL),
           "VERTICAL" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum::VERTICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required if orientation is horizontal. Image preview alignment for standalone cards with horizontal layout.
pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum {
    

    /// Not specified.
    ///
    /// "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED"
    #[serde(rename="THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED")]
    THUMBNAILIMAGEALIGNMENTUNSPECIFIED,
    

    /// Thumbnail preview is left-aligned.
    ///
    /// "LEFT"
    #[serde(rename="LEFT")]
    LEFT,
    

    /// Thumbnail preview is right-aligned.
    ///
    /// "RIGHT"
    #[serde(rename="RIGHT")]
    RIGHT,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum::THUMBNAILIMAGEALIGNMENTUNSPECIFIED => "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum::LEFT => "LEFT",
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum::RIGHT => "RIGHT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum::THUMBNAILIMAGEALIGNMENTUNSPECIFIED),
           "LEFT" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum::LEFT),
           "RIGHT" => Ok(GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum::RIGHT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the training phrase.
pub enum GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
    

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
    

    /// Templates are not annotated with entity types, but they can contain @-prefixed entity type names as substrings. Note: Template mode has been deprecated. Example mode is the only supported way to create new training phrases. If you have existing training phrases in template mode, they will be removed during training and it can cause a drop in agent performance.
    ///
    /// "TEMPLATE"
    #[serde(rename="TEMPLATE")]
    TEMPLATE,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum::EXAMPLE => "EXAMPLE",
            GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum::TEMPLATE => "TEMPLATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum::TYPEUNSPECIFIED),
           "EXAMPLE" => Ok(GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum::EXAMPLE),
           "TEMPLATE" => Ok(GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum::TEMPLATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The system's confidence level that this knowledge answer is a good match for this conversational query. NOTE: The confidence level for a given `` pair may change without notice, as it depends on models that are constantly being improved. However, it will change less frequently than the confidence score below, and should be preferred for referencing the quality of an answer.
pub enum GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum {
    

    /// Not specified.
    ///
    /// "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED"
    #[serde(rename="MATCH_CONFIDENCE_LEVEL_UNSPECIFIED")]
    MATCHCONFIDENCELEVELUNSPECIFIED,
    

    /// Indicates that the confidence is low.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Indicates our confidence is medium.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Indicates our confidence is high.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::MATCHCONFIDENCELEVELUNSPECIFIED => "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::LOW => "LOW",
            GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::MEDIUM => "MEDIUM",
            GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::HIGH => "HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::MATCHCONFIDENCELEVELUNSPECIFIED),
           "LOW" => Ok(GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::LOW),
           "MEDIUM" => Ok(GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::MEDIUM),
           "HIGH" => Ok(GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum::HIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The role of the participant.
pub enum GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1MessageParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Format of message.
pub enum GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum::MESSAGEFORMATUNSPECIFIED => "MESSAGE_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum::PROTO => "PROTO",
            GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MESSAGE_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum::MESSAGEFORMATUNSPECIFIED),
           "PROTO" => Ok(GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum::PROTO),
           "JSON" => Ok(GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1NotificationConfigMessageFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the synthesized audio content.
pub enum GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3 => "OUTPUT_AUDIO_ENCODING_MP3",
            GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS => "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS",
            GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS => "OUTPUT_AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW => "OUTPUT_AUDIO_ENCODING_MULAW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED),
           "OUTPUT_AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16),
           "OUTPUT_AUDIO_ENCODING_MP3" => Ok(GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3),
           "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS" => Ok(GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS),
           "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS),
           "OUTPUT_AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1OutputAudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Immutable. The role this participant plays in the conversation. This field must be set during participant creation and is then immutable.
pub enum GoogleCloudDialogflowV2beta1ParticipantRoleEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1ParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2beta1ParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2beta1ParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2beta1ParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2beta1ParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2beta1ParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the answer.
pub enum GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::ANSWERTYPEUNSPECIFIED => "ANSWER_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::GENERATIVE => "GENERATIVE",
            GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::INTENT => "INTENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANSWER_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::ANSWERTYPEUNSPECIFIED),
           "FAQ" => Ok(GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::FAQ),
           "GENERATIVE" => Ok(GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::GENERATIVE),
           "INTENT" => Ok(GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum::INTENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1SearchKnowledgeAnswerAnswerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether the additional data should override or supplement the custom entity type definition.
pub enum GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEUNSPECIFIED => "ENTITY_OVERRIDE_MODE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEOVERRIDE => "ENTITY_OVERRIDE_MODE_OVERRIDE",
            GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODESUPPLEMENT => "ENTITY_OVERRIDE_MODE_SUPPLEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_OVERRIDE_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEUNSPECIFIED),
           "ENTITY_OVERRIDE_MODE_OVERRIDE" => Ok(GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEOVERRIDE),
           "ENTITY_OVERRIDE_MODE_SUPPLEMENT" => Ok(GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODESUPPLEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The participant role to add or update the suggestion feature config. Only HUMAN_AGENT or END_USER can be used.
pub enum GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT => "HUMAN_AGENT",
            GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT => "AUTOMATED_AGENT",
            GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER => "END_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::ROLEUNSPECIFIED),
           "HUMAN_AGENT" => Ok(GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::HUMANAGENT),
           "AUTOMATED_AGENT" => Ok(GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::AUTOMATEDAGENT),
           "END_USER" => Ok(GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum::ENDUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1SetSuggestionFeatureConfigRequestParticipantRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The speech model used in speech to text. `SPEECH_MODEL_VARIANT_UNSPECIFIED`, `USE_BEST_AVAILABLE` will be treated as `USE_ENHANCED`. It can be overridden in AnalyzeContentRequest and StreamingAnalyzeContentRequest request. If enhanced model variant is specified and an enhanced version of the specified model for the language does not exist, then it would emit an error.
pub enum GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED => "SPEECH_MODEL_VARIANT_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::USEBESTAVAILABLE => "USE_BEST_AVAILABLE",
            GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::USESTANDARD => "USE_STANDARD",
            GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::USEENHANCED => "USE_ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPEECH_MODEL_VARIANT_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED),
           "USE_BEST_AVAILABLE" => Ok(GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::USEBESTAVAILABLE),
           "USE_STANDARD" => Ok(GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::USESTANDARD),
           "USE_ENHANCED" => Ok(GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum::USEENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1SpeechToTextConfigSpeechModelVariantEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of Human Agent Assistant API feature to request.
pub enum GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum {
    

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
    

    /// Run FAQ model.
    ///
    /// "FAQ"
    #[serde(rename="FAQ")]
    FAQ,
    

    /// Run smart reply model for chat.
    ///
    /// "SMART_REPLY"
    #[serde(rename="SMART_REPLY")]
    SMARTREPLY,
    

    /// Run Dialogflow assist model for chat, which will return automated agent response as suggestion.
    ///
    /// "DIALOGFLOW_ASSIST"
    #[serde(rename="DIALOGFLOW_ASSIST")]
    DIALOGFLOWASSIST,
    

    /// Run conversation summarization model for chat.
    ///
    /// "CONVERSATION_SUMMARIZATION"
    #[serde(rename="CONVERSATION_SUMMARIZATION")]
    CONVERSATIONSUMMARIZATION,
    

    /// Run knowledge search with text input from agent or text generated query.
    ///
    /// "KNOWLEDGE_SEARCH"
    #[serde(rename="KNOWLEDGE_SEARCH")]
    KNOWLEDGESEARCH,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::ARTICLESUGGESTION => "ARTICLE_SUGGESTION",
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::FAQ => "FAQ",
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::SMARTREPLY => "SMART_REPLY",
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::DIALOGFLOWASSIST => "DIALOGFLOW_ASSIST",
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::CONVERSATIONSUMMARIZATION => "CONVERSATION_SUMMARIZATION",
            GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::KNOWLEDGESEARCH => "KNOWLEDGE_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::TYPEUNSPECIFIED),
           "ARTICLE_SUGGESTION" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::ARTICLESUGGESTION),
           "FAQ" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::FAQ),
           "SMART_REPLY" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::SMARTREPLY),
           "DIALOGFLOW_ASSIST" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::DIALOGFLOWASSIST),
           "CONVERSATION_SUMMARIZATION" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::CONVERSATIONSUMMARIZATION),
           "KNOWLEDGE_SEARCH" => Ok(GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum::KNOWLEDGESEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1SuggestionFeatureTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A sequence of TelephonyDtmf digits.
pub enum GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum {
    

    /// Not specified. This value may be used to indicate an absent digit.
    ///
    /// "TELEPHONY_DTMF_UNSPECIFIED"
    #[serde(rename="TELEPHONY_DTMF_UNSPECIFIED")]
    TELEPHONYDTMFUNSPECIFIED,
    

    /// Number: '1'.
    ///
    /// "DTMF_ONE"
    #[serde(rename="DTMF_ONE")]
    DTMFONE,
    

    /// Number: '2'.
    ///
    /// "DTMF_TWO"
    #[serde(rename="DTMF_TWO")]
    DTMFTWO,
    

    /// Number: '3'.
    ///
    /// "DTMF_THREE"
    #[serde(rename="DTMF_THREE")]
    DTMFTHREE,
    

    /// Number: '4'.
    ///
    /// "DTMF_FOUR"
    #[serde(rename="DTMF_FOUR")]
    DTMFFOUR,
    

    /// Number: '5'.
    ///
    /// "DTMF_FIVE"
    #[serde(rename="DTMF_FIVE")]
    DTMFFIVE,
    

    /// Number: '6'.
    ///
    /// "DTMF_SIX"
    #[serde(rename="DTMF_SIX")]
    DTMFSIX,
    

    /// Number: '7'.
    ///
    /// "DTMF_SEVEN"
    #[serde(rename="DTMF_SEVEN")]
    DTMFSEVEN,
    

    /// Number: '8'.
    ///
    /// "DTMF_EIGHT"
    #[serde(rename="DTMF_EIGHT")]
    DTMFEIGHT,
    

    /// Number: '9'.
    ///
    /// "DTMF_NINE"
    #[serde(rename="DTMF_NINE")]
    DTMFNINE,
    

    /// Number: '0'.
    ///
    /// "DTMF_ZERO"
    #[serde(rename="DTMF_ZERO")]
    DTMFZERO,
    

    /// Letter: 'A'.
    ///
    /// "DTMF_A"
    #[serde(rename="DTMF_A")]
    DTMFA,
    

    /// Letter: 'B'.
    ///
    /// "DTMF_B"
    #[serde(rename="DTMF_B")]
    DTMFB,
    

    /// Letter: 'C'.
    ///
    /// "DTMF_C"
    #[serde(rename="DTMF_C")]
    DTMFC,
    

    /// Letter: 'D'.
    ///
    /// "DTMF_D"
    #[serde(rename="DTMF_D")]
    DTMFD,
    

    /// Asterisk/star: '*'.
    ///
    /// "DTMF_STAR"
    #[serde(rename="DTMF_STAR")]
    DTMFSTAR,
    

    /// Pound/diamond/hash/square/gate/octothorpe: '#'.
    ///
    /// "DTMF_POUND"
    #[serde(rename="DTMF_POUND")]
    DTMFPOUND,
}

impl AsRef<str> for GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::TELEPHONYDTMFUNSPECIFIED => "TELEPHONY_DTMF_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFONE => "DTMF_ONE",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFTWO => "DTMF_TWO",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFTHREE => "DTMF_THREE",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFFOUR => "DTMF_FOUR",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFFIVE => "DTMF_FIVE",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFSIX => "DTMF_SIX",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFSEVEN => "DTMF_SEVEN",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFEIGHT => "DTMF_EIGHT",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFNINE => "DTMF_NINE",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFZERO => "DTMF_ZERO",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFA => "DTMF_A",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFB => "DTMF_B",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFC => "DTMF_C",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFD => "DTMF_D",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFSTAR => "DTMF_STAR",
            GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFPOUND => "DTMF_POUND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TELEPHONY_DTMF_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::TELEPHONYDTMFUNSPECIFIED),
           "DTMF_ONE" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFONE),
           "DTMF_TWO" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFTWO),
           "DTMF_THREE" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFTHREE),
           "DTMF_FOUR" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFFOUR),
           "DTMF_FIVE" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFFIVE),
           "DTMF_SIX" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFSIX),
           "DTMF_SEVEN" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFSEVEN),
           "DTMF_EIGHT" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFEIGHT),
           "DTMF_NINE" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFNINE),
           "DTMF_ZERO" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFZERO),
           "DTMF_A" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFA),
           "DTMF_B" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFB),
           "DTMF_C" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFC),
           "DTMF_D" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFD),
           "DTMF_STAR" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFSTAR),
           "DTMF_POUND" => Ok(GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum::DTMFPOUND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1TelephonyDtmfEventDtmfEventsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the synthesized audio content.
pub enum GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3 => "OUTPUT_AUDIO_ENCODING_MP3",
            GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS => "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS",
            GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS => "OUTPUT_AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW => "OUTPUT_AUDIO_ENCODING_MULAW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED),
           "OUTPUT_AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16),
           "OUTPUT_AUDIO_ENCODING_MP3" => Ok(GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3),
           "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS" => Ok(GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS),
           "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS),
           "OUTPUT_AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1TextToSpeechSettingOutputAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The severity of the error.
pub enum GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::INFO => "INFO",
            GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::WARNING => "WARNING",
            GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::ERROR => "ERROR",
            GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::INFO),
           "WARNING" => Ok(GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::WARNING),
           "ERROR" => Ok(GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::ERROR),
           "CRITICAL" => Ok(GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1ValidationErrorSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1VersionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status of this version. This field is read-only and cannot be set by create and update methods.
pub enum GoogleCloudDialogflowV2beta1VersionStatusEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1VersionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1VersionStatusEnum::VERSIONSTATUSUNSPECIFIED => "VERSION_STATUS_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1VersionStatusEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudDialogflowV2beta1VersionStatusEnum::READY => "READY",
            GoogleCloudDialogflowV2beta1VersionStatusEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1VersionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_STATUS_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1VersionStatusEnum::VERSIONSTATUSUNSPECIFIED),
           "IN_PROGRESS" => Ok(GoogleCloudDialogflowV2beta1VersionStatusEnum::INPROGRESS),
           "READY" => Ok(GoogleCloudDialogflowV2beta1VersionStatusEnum::READY),
           "FAILED" => Ok(GoogleCloudDialogflowV2beta1VersionStatusEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1VersionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request.
pub enum GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED => "SSML_VOICE_GENDER_UNSPECIFIED",
            GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERMALE => "SSML_VOICE_GENDER_MALE",
            GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERFEMALE => "SSML_VOICE_GENDER_FEMALE",
            GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERNEUTRAL => "SSML_VOICE_GENDER_NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSML_VOICE_GENDER_UNSPECIFIED" => Ok(GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED),
           "SSML_VOICE_GENDER_MALE" => Ok(GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERMALE),
           "SSML_VOICE_GENDER_FEMALE" => Ok(GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERFEMALE),
           "SSML_VOICE_GENDER_NEUTRAL" => Ok(GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERNEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowV2beta1VoiceSelectionParamSsmlGenderEnum {
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


