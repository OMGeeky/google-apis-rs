use super::*;



// region GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Rating from user for the specific Dialogflow response.
pub enum GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum {
    

    /// Rating not specified.
    ///
    /// "RATING_UNSPECIFIED"
    #[serde(rename="RATING_UNSPECIFIED")]
    RATINGUNSPECIFIED,
    

    /// Thumbs up feedback from user.
    ///
    /// "THUMBS_UP"
    #[serde(rename="THUMBS_UP")]
    THUMBSUP,
    

    /// Thumbs down feedback from user.
    ///
    /// "THUMBS_DOWN"
    #[serde(rename="THUMBS_DOWN")]
    THUMBSDOWN,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum::RATINGUNSPECIFIED => "RATING_UNSPECIFIED",
            GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum::THUMBSUP => "THUMBS_UP",
            GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum::THUMBSDOWN => "THUMBS_DOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RATING_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum::RATINGUNSPECIFIED),
           "THUMBS_UP" => Ok(GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum::THUMBSUP),
           "THUMBS_DOWN" => Ok(GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum::THUMBSDOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3AnswerFeedbackRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The result of this continuous test run, i.e. whether all the tests in this continuous test run pass or not.
pub enum GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum {
    

    /// Not specified. Should never be used.
    ///
    /// "AGGREGATED_TEST_RESULT_UNSPECIFIED"
    #[serde(rename="AGGREGATED_TEST_RESULT_UNSPECIFIED")]
    AGGREGATEDTESTRESULTUNSPECIFIED,
    

    /// All the tests passed.
    ///
    /// "PASSED"
    #[serde(rename="PASSED")]
    PASSED,
    

    /// At least one test did not pass.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum::AGGREGATEDTESTRESULTUNSPECIFIED => "AGGREGATED_TEST_RESULT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum::PASSED => "PASSED",
            GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AGGREGATED_TEST_RESULT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum::AGGREGATEDTESTRESULTUNSPECIFIED),
           "PASSED" => Ok(GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum::PASSED),
           "FAILED" => Ok(GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ContinuousTestResultResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the connected data store.
pub enum GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum {
    

    /// Not specified. This value indicates that the data store type is not specified, so it will not be used during search.
    ///
    /// "DATA_STORE_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_STORE_TYPE_UNSPECIFIED")]
    DATASTORETYPEUNSPECIFIED,
    

    /// A data store that contains public web content.
    ///
    /// "PUBLIC_WEB"
    #[serde(rename="PUBLIC_WEB")]
    PUBLICWEB,
    

    /// A data store that contains unstructured private data.
    ///
    /// "UNSTRUCTURED"
    #[serde(rename="UNSTRUCTURED")]
    UNSTRUCTURED,
    

    /// A data store that contains structured data (for example FAQ).
    ///
    /// "STRUCTURED"
    #[serde(rename="STRUCTURED")]
    STRUCTURED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::DATASTORETYPEUNSPECIFIED => "DATA_STORE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::PUBLICWEB => "PUBLIC_WEB",
            GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::UNSTRUCTURED => "UNSTRUCTURED",
            GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::STRUCTURED => "STRUCTURED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_STORE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::DATASTORETYPEUNSPECIFIED),
           "PUBLIC_WEB" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::PUBLICWEB),
           "UNSTRUCTURED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::UNSTRUCTURED),
           "STRUCTURED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum::STRUCTURED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DataStoreConnectionDataStoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents the decision of the grounding check.
pub enum GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum {
    

    /// Decision not specified.
    ///
    /// "GROUNDING_DECISION_UNSPECIFIED"
    #[serde(rename="GROUNDING_DECISION_UNSPECIFIED")]
    GROUNDINGDECISIONUNSPECIFIED,
    

    /// Grounding have accepted the answer.
    ///
    /// "ACCEPTED_BY_GROUNDING"
    #[serde(rename="ACCEPTED_BY_GROUNDING")]
    ACCEPTEDBYGROUNDING,
    

    /// Grounding have rejected the answer.
    ///
    /// "REJECTED_BY_GROUNDING"
    #[serde(rename="REJECTED_BY_GROUNDING")]
    REJECTEDBYGROUNDING,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum::GROUNDINGDECISIONUNSPECIFIED => "GROUNDING_DECISION_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum::ACCEPTEDBYGROUNDING => "ACCEPTED_BY_GROUNDING",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum::REJECTEDBYGROUNDING => "REJECTED_BY_GROUNDING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GROUNDING_DECISION_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum::GROUNDINGDECISIONUNSPECIFIED),
           "ACCEPTED_BY_GROUNDING" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum::ACCEPTEDBYGROUNDING),
           "REJECTED_BY_GROUNDING" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum::REJECTEDBYGROUNDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalDecisionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Grounding score bucket setting.
pub enum GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum {
    

    /// Score not specified.
    ///
    /// "GROUNDING_SCORE_BUCKET_UNSPECIFIED"
    #[serde(rename="GROUNDING_SCORE_BUCKET_UNSPECIFIED")]
    GROUNDINGSCOREBUCKETUNSPECIFIED,
    

    /// We have very low confidence that the answer is grounded.
    ///
    /// "VERY_LOW"
    #[serde(rename="VERY_LOW")]
    VERYLOW,
    

    /// We have low confidence that the answer is grounded.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// We have medium confidence that the answer is grounded.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// We have high confidence that the answer is grounded.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// We have very high confidence that the answer is grounded.
    ///
    /// "VERY_HIGH"
    #[serde(rename="VERY_HIGH")]
    VERYHIGH,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::GROUNDINGSCOREBUCKETUNSPECIFIED => "GROUNDING_SCORE_BUCKET_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::VERYLOW => "VERY_LOW",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::LOW => "LOW",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::MEDIUM => "MEDIUM",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::HIGH => "HIGH",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::VERYHIGH => "VERY_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GROUNDING_SCORE_BUCKET_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::GROUNDINGSCOREBUCKETUNSPECIFIED),
           "VERY_LOW" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::VERYLOW),
           "LOW" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::LOW),
           "MEDIUM" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::MEDIUM),
           "HIGH" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::HIGH),
           "VERY_HIGH" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum::VERYHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DataStoreConnectionSignalsGroundingSignalScoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies banned phrase match subject.
pub enum GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum {
    

    /// No banned phrase check was executed.
    ///
    /// "BANNED_PHRASE_MATCH_UNSPECIFIED"
    #[serde(rename="BANNED_PHRASE_MATCH_UNSPECIFIED")]
    BANNEDPHRASEMATCHUNSPECIFIED,
    

    /// All banned phrase checks led to no match.
    ///
    /// "BANNED_PHRASE_MATCH_NONE"
    #[serde(rename="BANNED_PHRASE_MATCH_NONE")]
    BANNEDPHRASEMATCHNONE,
    

    /// A banned phrase matched the query.
    ///
    /// "BANNED_PHRASE_MATCH_QUERY"
    #[serde(rename="BANNED_PHRASE_MATCH_QUERY")]
    BANNEDPHRASEMATCHQUERY,
    

    /// A banned phrase matched the response.
    ///
    /// "BANNED_PHRASE_MATCH_RESPONSE"
    #[serde(rename="BANNED_PHRASE_MATCH_RESPONSE")]
    BANNEDPHRASEMATCHRESPONSE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHUNSPECIFIED => "BANNED_PHRASE_MATCH_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHNONE => "BANNED_PHRASE_MATCH_NONE",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHQUERY => "BANNED_PHRASE_MATCH_QUERY",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHRESPONSE => "BANNED_PHRASE_MATCH_RESPONSE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BANNED_PHRASE_MATCH_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHUNSPECIFIED),
           "BANNED_PHRASE_MATCH_NONE" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHNONE),
           "BANNED_PHRASE_MATCH_QUERY" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHQUERY),
           "BANNED_PHRASE_MATCH_RESPONSE" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum::BANNEDPHRASEMATCHRESPONSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalBannedPhraseMatchEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Safety decision.
pub enum GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum {
    

    /// Decision not specified.
    ///
    /// "SAFETY_DECISION_UNSPECIFIED"
    #[serde(rename="SAFETY_DECISION_UNSPECIFIED")]
    SAFETYDECISIONUNSPECIFIED,
    

    /// No manual or automatic safety check fired.
    ///
    /// "ACCEPTED_BY_SAFETY_CHECK"
    #[serde(rename="ACCEPTED_BY_SAFETY_CHECK")]
    ACCEPTEDBYSAFETYCHECK,
    

    /// One ore more safety checks fired.
    ///
    /// "REJECTED_BY_SAFETY_CHECK"
    #[serde(rename="REJECTED_BY_SAFETY_CHECK")]
    REJECTEDBYSAFETYCHECK,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum::SAFETYDECISIONUNSPECIFIED => "SAFETY_DECISION_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum::ACCEPTEDBYSAFETYCHECK => "ACCEPTED_BY_SAFETY_CHECK",
            GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum::REJECTEDBYSAFETYCHECK => "REJECTED_BY_SAFETY_CHECK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAFETY_DECISION_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum::SAFETYDECISIONUNSPECIFIED),
           "ACCEPTED_BY_SAFETY_CHECK" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum::ACCEPTEDBYSAFETYCHECK),
           "REJECTED_BY_SAFETY_CHECK" => Ok(GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum::REJECTEDBYSAFETYCHECK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DataStoreConnectionSignalsSafetySignalDecisionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DeploymentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the deployment.
pub enum GoogleCloudDialogflowCxV3DeploymentStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The deployment is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The deployment succeeded.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The deployment failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DeploymentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DeploymentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DeploymentStateEnum::RUNNING => "RUNNING",
            GoogleCloudDialogflowCxV3DeploymentStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudDialogflowCxV3DeploymentStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DeploymentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DeploymentStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GoogleCloudDialogflowCxV3DeploymentStateEnum::RUNNING),
           "SUCCEEDED" => Ok(GoogleCloudDialogflowCxV3DeploymentStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudDialogflowCxV3DeploymentStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DeploymentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Response type.
pub enum GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum {
    

    /// Not specified. This should never happen.
    ///
    /// "RESPONSE_TYPE_UNSPECIFIED"
    #[serde(rename="RESPONSE_TYPE_UNSPECIFIED")]
    RESPONSETYPEUNSPECIFIED,
    

    /// Partial response. e.g. Aggregated responses in a Fulfillment that enables `return_partial_response` can be returned as partial response. WARNING: partial response is not eligible for barge-in.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// Final response.
    ///
    /// "FINAL"
    #[serde(rename="FINAL")]
    FINAL,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum::RESPONSETYPEUNSPECIFIED => "RESPONSE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum::PARTIAL => "PARTIAL",
            GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum::FINAL => "FINAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum::RESPONSETYPEUNSPECIFIED),
           "PARTIAL" => Ok(GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum::PARTIAL),
           "FINAL" => Ok(GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum::FINAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3DetectIntentResponseResponseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the entity type can be automatically expanded.
pub enum GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEUNSPECIFIED => "AUTO_EXPANSION_MODE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEDEFAULT => "AUTO_EXPANSION_MODE_DEFAULT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUTO_EXPANSION_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEUNSPECIFIED),
           "AUTO_EXPANSION_MODE_DEFAULT" => Ok(GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum::AUTOEXPANSIONMODEDEFAULT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3EntityTypeAutoExpansionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3EntityTypeKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates the kind of entity type.
pub enum GoogleCloudDialogflowCxV3EntityTypeKindEnum {
    

    /// Not specified. This value should be never used.
    ///
    /// "KIND_UNSPECIFIED"
    #[serde(rename="KIND_UNSPECIFIED")]
    KINDUNSPECIFIED,
    

    /// Map entity types allow mapping of a group of synonyms to a canonical value.
    ///
    /// "KIND_MAP"
    #[serde(rename="KIND_MAP")]
    KINDMAP,
    

    /// List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases).
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

impl AsRef<str> for GoogleCloudDialogflowCxV3EntityTypeKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDUNSPECIFIED => "KIND_UNSPECIFIED",
            GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDMAP => "KIND_MAP",
            GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDLIST => "KIND_LIST",
            GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDREGEXP => "KIND_REGEXP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3EntityTypeKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KIND_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDUNSPECIFIED),
           "KIND_MAP" => Ok(GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDMAP),
           "KIND_LIST" => Ok(GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDLIST),
           "KIND_REGEXP" => Ok(GoogleCloudDialogflowCxV3EntityTypeKindEnum::KINDREGEXP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3EntityTypeKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExperimentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the experiment. Transition triggered by Experiments.StartExperiment: DRAFT->RUNNING. Transition triggered by Experiments.CancelExperiment: DRAFT->DONE or RUNNING->DONE.
pub enum GoogleCloudDialogflowCxV3ExperimentStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The experiment is created but not started yet.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// The experiment is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The experiment is done.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    

    /// The experiment with auto-rollout enabled has failed.
    ///
    /// "ROLLOUT_FAILED"
    #[serde(rename="ROLLOUT_FAILED")]
    ROLLOUTFAILED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExperimentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExperimentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExperimentStateEnum::DRAFT => "DRAFT",
            GoogleCloudDialogflowCxV3ExperimentStateEnum::RUNNING => "RUNNING",
            GoogleCloudDialogflowCxV3ExperimentStateEnum::DONE => "DONE",
            GoogleCloudDialogflowCxV3ExperimentStateEnum::ROLLOUTFAILED => "ROLLOUT_FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExperimentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExperimentStateEnum::STATEUNSPECIFIED),
           "DRAFT" => Ok(GoogleCloudDialogflowCxV3ExperimentStateEnum::DRAFT),
           "RUNNING" => Ok(GoogleCloudDialogflowCxV3ExperimentStateEnum::RUNNING),
           "DONE" => Ok(GoogleCloudDialogflowCxV3ExperimentStateEnum::DONE),
           "ROLLOUT_FAILED" => Ok(GoogleCloudDialogflowCxV3ExperimentStateEnum::ROLLOUTFAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExperimentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Count-based metric type. Only one of type or count_type is specified in each Metric.
pub enum GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum {
    

    /// Count type unspecified.
    ///
    /// "COUNT_TYPE_UNSPECIFIED"
    #[serde(rename="COUNT_TYPE_UNSPECIFIED")]
    COUNTTYPEUNSPECIFIED,
    

    /// Total number of occurrences of a 'NO_MATCH'.
    ///
    /// "TOTAL_NO_MATCH_COUNT"
    #[serde(rename="TOTAL_NO_MATCH_COUNT")]
    TOTALNOMATCHCOUNT,
    

    /// Total number of turn counts.
    ///
    /// "TOTAL_TURN_COUNT"
    #[serde(rename="TOTAL_TURN_COUNT")]
    TOTALTURNCOUNT,
    

    /// Average turn count in a session.
    ///
    /// "AVERAGE_TURN_COUNT"
    #[serde(rename="AVERAGE_TURN_COUNT")]
    AVERAGETURNCOUNT,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::COUNTTYPEUNSPECIFIED => "COUNT_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::TOTALNOMATCHCOUNT => "TOTAL_NO_MATCH_COUNT",
            GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::TOTALTURNCOUNT => "TOTAL_TURN_COUNT",
            GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::AVERAGETURNCOUNT => "AVERAGE_TURN_COUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COUNT_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::COUNTTYPEUNSPECIFIED),
           "TOTAL_NO_MATCH_COUNT" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::TOTALNOMATCHCOUNT),
           "TOTAL_TURN_COUNT" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::TOTALTURNCOUNT),
           "AVERAGE_TURN_COUNT" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum::AVERAGETURNCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExperimentResultMetricCountTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Ratio-based metric type. Only one of type or count_type is specified in each Metric.
pub enum GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum {
    

    /// Metric unspecified.
    ///
    /// "METRIC_UNSPECIFIED"
    #[serde(rename="METRIC_UNSPECIFIED")]
    METRICUNSPECIFIED,
    

    /// Percentage of contained sessions without user calling back in 24 hours.
    ///
    /// "CONTAINED_SESSION_NO_CALLBACK_RATE"
    #[serde(rename="CONTAINED_SESSION_NO_CALLBACK_RATE")]
    CONTAINEDSESSIONNOCALLBACKRATE,
    

    /// Percentage of sessions that were handed to a human agent.
    ///
    /// "LIVE_AGENT_HANDOFF_RATE"
    #[serde(rename="LIVE_AGENT_HANDOFF_RATE")]
    LIVEAGENTHANDOFFRATE,
    

    /// Percentage of sessions with the same user calling back.
    ///
    /// "CALLBACK_SESSION_RATE"
    #[serde(rename="CALLBACK_SESSION_RATE")]
    CALLBACKSESSIONRATE,
    

    /// Percentage of sessions where user hung up.
    ///
    /// "ABANDONED_SESSION_RATE"
    #[serde(rename="ABANDONED_SESSION_RATE")]
    ABANDONEDSESSIONRATE,
    

    /// Percentage of sessions reached Dialogflow 'END_PAGE' or 'END_SESSION'.
    ///
    /// "SESSION_END_RATE"
    #[serde(rename="SESSION_END_RATE")]
    SESSIONENDRATE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::METRICUNSPECIFIED => "METRIC_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::CONTAINEDSESSIONNOCALLBACKRATE => "CONTAINED_SESSION_NO_CALLBACK_RATE",
            GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::LIVEAGENTHANDOFFRATE => "LIVE_AGENT_HANDOFF_RATE",
            GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::CALLBACKSESSIONRATE => "CALLBACK_SESSION_RATE",
            GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::ABANDONEDSESSIONRATE => "ABANDONED_SESSION_RATE",
            GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::SESSIONENDRATE => "SESSION_END_RATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::METRICUNSPECIFIED),
           "CONTAINED_SESSION_NO_CALLBACK_RATE" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::CONTAINEDSESSIONNOCALLBACKRATE),
           "LIVE_AGENT_HANDOFF_RATE" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::LIVEAGENTHANDOFFRATE),
           "CALLBACK_SESSION_RATE" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::CALLBACKSESSIONRATE),
           "ABANDONED_SESSION_RATE" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::ABANDONEDSESSIONRATE),
           "SESSION_END_RATE" => Ok(GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum::SESSIONENDRATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExperimentResultMetricTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The data format of the exported agent. If not specified, `BLOB` is assumed.
pub enum GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum {
    

    /// Unspecified format.
    ///
    /// "DATA_FORMAT_UNSPECIFIED"
    #[serde(rename="DATA_FORMAT_UNSPECIFIED")]
    DATAFORMATUNSPECIFIED,
    

    /// Agent content will be exported as raw bytes.
    ///
    /// "BLOB"
    #[serde(rename="BLOB")]
    BLOB,
    

    /// Agent content will be exported in JSON Package format.
    ///
    /// "JSON_PACKAGE"
    #[serde(rename="JSON_PACKAGE")]
    JSONPACKAGE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum::DATAFORMATUNSPECIFIED => "DATA_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum::BLOB => "BLOB",
            GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum::JSONPACKAGE => "JSON_PACKAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum::DATAFORMATUNSPECIFIED),
           "BLOB" => Ok(GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum::BLOB),
           "JSON_PACKAGE" => Ok(GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum::JSONPACKAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExportAgentRequestDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The data format of the exported entity types. If not specified, `BLOB` is assumed.
pub enum GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum {
    

    /// Unspecified format. Treated as `BLOB`.
    ///
    /// "DATA_FORMAT_UNSPECIFIED"
    #[serde(rename="DATA_FORMAT_UNSPECIFIED")]
    DATAFORMATUNSPECIFIED,
    

    /// EntityTypes will be exported as raw bytes.
    ///
    /// "BLOB"
    #[serde(rename="BLOB")]
    BLOB,
    

    /// EntityTypes will be exported in JSON Package format.
    ///
    /// "JSON_PACKAGE"
    #[serde(rename="JSON_PACKAGE")]
    JSONPACKAGE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum::DATAFORMATUNSPECIFIED => "DATA_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum::BLOB => "BLOB",
            GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum::JSONPACKAGE => "JSON_PACKAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum::DATAFORMATUNSPECIFIED),
           "BLOB" => Ok(GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum::BLOB),
           "JSON_PACKAGE" => Ok(GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum::JSONPACKAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExportEntityTypesRequestDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The data format of the exported intents. If not specified, `BLOB` is assumed.
pub enum GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum {
    

    /// Unspecified format. Treated as `BLOB`.
    ///
    /// "DATA_FORMAT_UNSPECIFIED"
    #[serde(rename="DATA_FORMAT_UNSPECIFIED")]
    DATAFORMATUNSPECIFIED,
    

    /// Intents will be exported as raw bytes.
    ///
    /// "BLOB"
    #[serde(rename="BLOB")]
    BLOB,
    

    /// Intents will be exported in JSON format.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Intents will be exported in CSV format.
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::DATAFORMATUNSPECIFIED => "DATA_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::BLOB => "BLOB",
            GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::JSON => "JSON",
            GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::CSV => "CSV",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::DATAFORMATUNSPECIFIED),
           "BLOB" => Ok(GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::BLOB),
           "JSON" => Ok(GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::JSON),
           "CSV" => Ok(GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum::CSV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExportIntentsRequestDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The data format of the exported test cases. If not specified, `BLOB` is assumed.
pub enum GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum {
    

    /// Unspecified format.
    ///
    /// "DATA_FORMAT_UNSPECIFIED"
    #[serde(rename="DATA_FORMAT_UNSPECIFIED")]
    DATAFORMATUNSPECIFIED,
    

    /// Raw bytes.
    ///
    /// "BLOB"
    #[serde(rename="BLOB")]
    BLOB,
    

    /// JSON format.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum::DATAFORMATUNSPECIFIED => "DATA_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum::BLOB => "BLOB",
            GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum::JSON => "JSON",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum::DATAFORMATUNSPECIFIED),
           "BLOB" => Ok(GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum::BLOB),
           "JSON" => Ok(GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum::JSON),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ExportTestCasesRequestDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Import strategy for resource conflict resolution, applied globally throughout the flow. It will be applied for all display name conflicts in the imported content. If not specified, 'CREATE_NEW' is assumed.
pub enum GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum {
    

    /// Unspecified. Treated as 'CREATE_NEW'.
    ///
    /// "IMPORT_STRATEGY_UNSPECIFIED"
    #[serde(rename="IMPORT_STRATEGY_UNSPECIFIED")]
    IMPORTSTRATEGYUNSPECIFIED,
    

    /// Create a new resource with a numeric suffix appended to the end of the existing display name.
    ///
    /// "IMPORT_STRATEGY_CREATE_NEW"
    #[serde(rename="IMPORT_STRATEGY_CREATE_NEW")]
    IMPORTSTRATEGYCREATENEW,
    

    /// Replace existing resource with incoming resource in the content to be imported.
    ///
    /// "IMPORT_STRATEGY_REPLACE"
    #[serde(rename="IMPORT_STRATEGY_REPLACE")]
    IMPORTSTRATEGYREPLACE,
    

    /// Keep existing resource and discard incoming resource in the content to be imported.
    ///
    /// "IMPORT_STRATEGY_KEEP"
    #[serde(rename="IMPORT_STRATEGY_KEEP")]
    IMPORTSTRATEGYKEEP,
    

    /// Combine existing and incoming resources when a conflict is encountered.
    ///
    /// "IMPORT_STRATEGY_MERGE"
    #[serde(rename="IMPORT_STRATEGY_MERGE")]
    IMPORTSTRATEGYMERGE,
    

    /// Throw error if a conflict is encountered.
    ///
    /// "IMPORT_STRATEGY_THROW_ERROR"
    #[serde(rename="IMPORT_STRATEGY_THROW_ERROR")]
    IMPORTSTRATEGYTHROWERROR,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYUNSPECIFIED => "IMPORT_STRATEGY_UNSPECIFIED",
            GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYCREATENEW => "IMPORT_STRATEGY_CREATE_NEW",
            GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYREPLACE => "IMPORT_STRATEGY_REPLACE",
            GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYKEEP => "IMPORT_STRATEGY_KEEP",
            GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYMERGE => "IMPORT_STRATEGY_MERGE",
            GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYTHROWERROR => "IMPORT_STRATEGY_THROW_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORT_STRATEGY_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYUNSPECIFIED),
           "IMPORT_STRATEGY_CREATE_NEW" => Ok(GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYCREATENEW),
           "IMPORT_STRATEGY_REPLACE" => Ok(GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYREPLACE),
           "IMPORT_STRATEGY_KEEP" => Ok(GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYKEEP),
           "IMPORT_STRATEGY_MERGE" => Ok(GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYMERGE),
           "IMPORT_STRATEGY_THROW_ERROR" => Ok(GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum::IMPORTSTRATEGYTHROWERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3FlowImportStrategyGlobalImportStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Merge option for importing entity types.
pub enum GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum {
    

    /// Unspecified. If used, system uses REPORT_CONFLICT as default.
    ///
    /// "MERGE_OPTION_UNSPECIFIED"
    #[serde(rename="MERGE_OPTION_UNSPECIFIED")]
    MERGEOPTIONUNSPECIFIED,
    

    /// Replace the original entity type in the agent with the new entity type when display name conflicts exist.
    ///
    /// "REPLACE"
    #[serde(rename="REPLACE")]
    REPLACE,
    

    /// Merge the original entity type with the new entity type when display name conflicts exist.
    ///
    /// "MERGE"
    #[serde(rename="MERGE")]
    MERGE,
    

    /// Create new entity types with new display names to differentiate them from the existing entity types when display name conflicts exist.
    ///
    /// "RENAME"
    #[serde(rename="RENAME")]
    RENAME,
    

    /// Report conflict information if display names conflict is detected. Otherwise, import entity types.
    ///
    /// "REPORT_CONFLICT"
    #[serde(rename="REPORT_CONFLICT")]
    REPORTCONFLICT,
    

    /// Keep the original entity type and discard the conflicting new entity type when display name conflicts exist.
    ///
    /// "KEEP"
    #[serde(rename="KEEP")]
    KEEP,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::MERGEOPTIONUNSPECIFIED => "MERGE_OPTION_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::REPLACE => "REPLACE",
            GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::MERGE => "MERGE",
            GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::RENAME => "RENAME",
            GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::REPORTCONFLICT => "REPORT_CONFLICT",
            GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::KEEP => "KEEP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MERGE_OPTION_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::MERGEOPTIONUNSPECIFIED),
           "REPLACE" => Ok(GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::REPLACE),
           "MERGE" => Ok(GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::MERGE),
           "RENAME" => Ok(GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::RENAME),
           "REPORT_CONFLICT" => Ok(GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::REPORTCONFLICT),
           "KEEP" => Ok(GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum::KEEP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ImportEntityTypesRequestMergeOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Flow import mode. If not specified, `KEEP` is assumed.
pub enum GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum {
    

    /// Unspecified. Treated as `KEEP`.
    ///
    /// "IMPORT_OPTION_UNSPECIFIED"
    #[serde(rename="IMPORT_OPTION_UNSPECIFIED")]
    IMPORTOPTIONUNSPECIFIED,
    

    /// Always respect settings in exported flow content. It may cause a import failure if some settings (e.g. custom NLU) are not supported in the agent to import into.
    ///
    /// "KEEP"
    #[serde(rename="KEEP")]
    KEEP,
    

    /// Fallback to default settings if some settings are not supported in the agent to import into. E.g. Standard NLU will be used if custom NLU is not available.
    ///
    /// "FALLBACK"
    #[serde(rename="FALLBACK")]
    FALLBACK,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum::IMPORTOPTIONUNSPECIFIED => "IMPORT_OPTION_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum::KEEP => "KEEP",
            GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum::FALLBACK => "FALLBACK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPORT_OPTION_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum::IMPORTOPTIONUNSPECIFIED),
           "KEEP" => Ok(GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum::KEEP),
           "FALLBACK" => Ok(GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum::FALLBACK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ImportFlowRequestImportOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Merge option for importing intents. If not specified, `REJECT` is assumed.
pub enum GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum {
    

    /// Unspecified. Should not be used.
    ///
    /// "MERGE_OPTION_UNSPECIFIED"
    #[serde(rename="MERGE_OPTION_UNSPECIFIED")]
    MERGEOPTIONUNSPECIFIED,
    

    /// DEPRECATED: Please use REPORT_CONFLICT instead. Fail the request if there are intents whose display names conflict with the display names of intents in the agent.
    ///
    /// "REJECT"
    #[serde(rename="REJECT")]
    REJECT,
    

    /// Replace the original intent in the agent with the new intent when display name conflicts exist.
    ///
    /// "REPLACE"
    #[serde(rename="REPLACE")]
    REPLACE,
    

    /// Merge the original intent with the new intent when display name conflicts exist.
    ///
    /// "MERGE"
    #[serde(rename="MERGE")]
    MERGE,
    

    /// Create new intents with new display names to differentiate them from the existing intents when display name conflicts exist.
    ///
    /// "RENAME"
    #[serde(rename="RENAME")]
    RENAME,
    

    /// Report conflict information if display names conflict is detected. Otherwise, import intents.
    ///
    /// "REPORT_CONFLICT"
    #[serde(rename="REPORT_CONFLICT")]
    REPORTCONFLICT,
    

    /// Keep the original intent and discard the conflicting new intent when display name conflicts exist.
    ///
    /// "KEEP"
    #[serde(rename="KEEP")]
    KEEP,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::MERGEOPTIONUNSPECIFIED => "MERGE_OPTION_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::REJECT => "REJECT",
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::REPLACE => "REPLACE",
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::MERGE => "MERGE",
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::RENAME => "RENAME",
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::REPORTCONFLICT => "REPORT_CONFLICT",
            GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::KEEP => "KEEP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MERGE_OPTION_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::MERGEOPTIONUNSPECIFIED),
           "REJECT" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::REJECT),
           "REPLACE" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::REPLACE),
           "MERGE" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::MERGE),
           "RENAME" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::RENAME),
           "REPORT_CONFLICT" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::REPORTCONFLICT),
           "KEEP" => Ok(GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum::KEEP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ImportIntentsRequestMergeOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the audio content to process.
pub enum GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED => "AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGLINEAR16 => "AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGFLAC => "AUDIO_ENCODING_FLAC",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGMULAW => "AUDIO_ENCODING_MULAW",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMR => "AUDIO_ENCODING_AMR",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMRWB => "AUDIO_ENCODING_AMR_WB",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGOGGOPUS => "AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGSPEEXWITHHEADERBYTE => "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGUNSPECIFIED),
           "AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGLINEAR16),
           "AUDIO_ENCODING_FLAC" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGFLAC),
           "AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGMULAW),
           "AUDIO_ENCODING_AMR" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMR),
           "AUDIO_ENCODING_AMR_WB" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGAMRWB),
           "AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGOGGOPUS),
           "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum::AUDIOENCODINGSPEEXWITHHEADERBYTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Which variant of the Speech model to use.
pub enum GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
    

    /// No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE.
    ///
    /// "SPEECH_MODEL_VARIANT_UNSPECIFIED"
    #[serde(rename="SPEECH_MODEL_VARIANT_UNSPECIFIED")]
    SPEECHMODELVARIANTUNSPECIFIED,
    

    /// Use the best available variant of the Speech model that the caller is eligible for.
    ///
    /// "USE_BEST_AVAILABLE"
    #[serde(rename="USE_BEST_AVAILABLE")]
    USEBESTAVAILABLE,
    

    /// Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models.
    ///
    /// "USE_STANDARD"
    #[serde(rename="USE_STANDARD")]
    USESTANDARD,
    

    /// Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants.
    ///
    /// "USE_ENHANCED"
    #[serde(rename="USE_ENHANCED")]
    USEENHANCED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED => "SPEECH_MODEL_VARIANT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::USEBESTAVAILABLE => "USE_BEST_AVAILABLE",
            GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::USESTANDARD => "USE_STANDARD",
            GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::USEENHANCED => "USE_ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SPEECH_MODEL_VARIANT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::SPEECHMODELVARIANTUNSPECIFIED),
           "USE_BEST_AVAILABLE" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::USEBESTAVAILABLE),
           "USE_STANDARD" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::USESTANDARD),
           "USE_ENHANCED" => Ok(GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum::USEENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3MatchMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this Match.
pub enum GoogleCloudDialogflowCxV3MatchMatchTypeEnum {
    

    /// Not specified. Should never be used.
    ///
    /// "MATCH_TYPE_UNSPECIFIED"
    #[serde(rename="MATCH_TYPE_UNSPECIFIED")]
    MATCHTYPEUNSPECIFIED,
    

    /// The query was matched to an intent.
    ///
    /// "INTENT"
    #[serde(rename="INTENT")]
    INTENT,
    

    /// The query directly triggered an intent.
    ///
    /// "DIRECT_INTENT"
    #[serde(rename="DIRECT_INTENT")]
    DIRECTINTENT,
    

    /// The query was used for parameter filling.
    ///
    /// "PARAMETER_FILLING"
    #[serde(rename="PARAMETER_FILLING")]
    PARAMETERFILLING,
    

    /// No match was found for the query.
    ///
    /// "NO_MATCH"
    #[serde(rename="NO_MATCH")]
    NOMATCH,
    

    /// Indicates an empty query.
    ///
    /// "NO_INPUT"
    #[serde(rename="NO_INPUT")]
    NOINPUT,
    

    /// The query directly triggered an event.
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3MatchMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::MATCHTYPEUNSPECIFIED => "MATCH_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::INTENT => "INTENT",
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::DIRECTINTENT => "DIRECT_INTENT",
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::PARAMETERFILLING => "PARAMETER_FILLING",
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::NOMATCH => "NO_MATCH",
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::NOINPUT => "NO_INPUT",
            GoogleCloudDialogflowCxV3MatchMatchTypeEnum::EVENT => "EVENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3MatchMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MATCH_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::MATCHTYPEUNSPECIFIED),
           "INTENT" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::INTENT),
           "DIRECT_INTENT" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::DIRECTINTENT),
           "PARAMETER_FILLING" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::PARAMETERFILLING),
           "NO_MATCH" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::NOMATCH),
           "NO_INPUT" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::NOINPUT),
           "EVENT" => Ok(GoogleCloudDialogflowCxV3MatchMatchTypeEnum::EVENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3MatchMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates NLU model training mode.
pub enum GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum {
    

    /// Not specified. `MODEL_TRAINING_MODE_AUTOMATIC` will be used.
    ///
    /// "MODEL_TRAINING_MODE_UNSPECIFIED"
    #[serde(rename="MODEL_TRAINING_MODE_UNSPECIFIED")]
    MODELTRAININGMODEUNSPECIFIED,
    

    /// NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode.
    ///
    /// "MODEL_TRAINING_MODE_AUTOMATIC"
    #[serde(rename="MODEL_TRAINING_MODE_AUTOMATIC")]
    MODELTRAININGMODEAUTOMATIC,
    

    /// User needs to manually trigger NLU model training. Best for large flows whose models take long time to train.
    ///
    /// "MODEL_TRAINING_MODE_MANUAL"
    #[serde(rename="MODEL_TRAINING_MODE_MANUAL")]
    MODELTRAININGMODEMANUAL,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum::MODELTRAININGMODEUNSPECIFIED => "MODEL_TRAINING_MODE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum::MODELTRAININGMODEAUTOMATIC => "MODEL_TRAINING_MODE_AUTOMATIC",
            GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum::MODELTRAININGMODEMANUAL => "MODEL_TRAINING_MODE_MANUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_TRAINING_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum::MODELTRAININGMODEUNSPECIFIED),
           "MODEL_TRAINING_MODE_AUTOMATIC" => Ok(GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum::MODELTRAININGMODEAUTOMATIC),
           "MODEL_TRAINING_MODE_MANUAL" => Ok(GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum::MODELTRAININGMODEMANUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3NluSettingModelTrainingModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3NluSettingModelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the type of NLU model.
pub enum GoogleCloudDialogflowCxV3NluSettingModelTypeEnum {
    

    /// Not specified. `MODEL_TYPE_STANDARD` will be used.
    ///
    /// "MODEL_TYPE_UNSPECIFIED"
    #[serde(rename="MODEL_TYPE_UNSPECIFIED")]
    MODELTYPEUNSPECIFIED,
    

    /// Use standard NLU model.
    ///
    /// "MODEL_TYPE_STANDARD"
    #[serde(rename="MODEL_TYPE_STANDARD")]
    MODELTYPESTANDARD,
    

    /// Use advanced NLU model.
    ///
    /// "MODEL_TYPE_ADVANCED"
    #[serde(rename="MODEL_TYPE_ADVANCED")]
    MODELTYPEADVANCED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3NluSettingModelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3NluSettingModelTypeEnum::MODELTYPEUNSPECIFIED => "MODEL_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3NluSettingModelTypeEnum::MODELTYPESTANDARD => "MODEL_TYPE_STANDARD",
            GoogleCloudDialogflowCxV3NluSettingModelTypeEnum::MODELTYPEADVANCED => "MODEL_TYPE_ADVANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3NluSettingModelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODEL_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3NluSettingModelTypeEnum::MODELTYPEUNSPECIFIED),
           "MODEL_TYPE_STANDARD" => Ok(GoogleCloudDialogflowCxV3NluSettingModelTypeEnum::MODELTYPESTANDARD),
           "MODEL_TYPE_ADVANCED" => Ok(GoogleCloudDialogflowCxV3NluSettingModelTypeEnum::MODELTYPEADVANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3NluSettingModelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Audio encoding of the synthesized audio content.
pub enum GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED => "OUTPUT_AUDIO_ENCODING_UNSPECIFIED",
            GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16 => "OUTPUT_AUDIO_ENCODING_LINEAR_16",
            GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3 => "OUTPUT_AUDIO_ENCODING_MP3",
            GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS => "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS",
            GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS => "OUTPUT_AUDIO_ENCODING_OGG_OPUS",
            GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW => "OUTPUT_AUDIO_ENCODING_MULAW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OUTPUT_AUDIO_ENCODING_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGUNSPECIFIED),
           "OUTPUT_AUDIO_ENCODING_LINEAR_16" => Ok(GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGLINEAR16),
           "OUTPUT_AUDIO_ENCODING_MP3" => Ok(GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP3),
           "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS" => Ok(GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMP364KBPS),
           "OUTPUT_AUDIO_ENCODING_OGG_OPUS" => Ok(GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGOGGOPUS),
           "OUTPUT_AUDIO_ENCODING_MULAW" => Ok(GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum::OUTPUTAUDIOENCODINGMULAW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3OutputAudioConfigAudioEncodingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Response type.
pub enum GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum {
    

    /// Not specified.
    ///
    /// "RESPONSE_TYPE_UNSPECIFIED"
    #[serde(rename="RESPONSE_TYPE_UNSPECIFIED")]
    RESPONSETYPEUNSPECIFIED,
    

    /// The response is from an entry prompt in the page.
    ///
    /// "ENTRY_PROMPT"
    #[serde(rename="ENTRY_PROMPT")]
    ENTRYPROMPT,
    

    /// The response is from form-filling prompt in the page.
    ///
    /// "PARAMETER_PROMPT"
    #[serde(rename="PARAMETER_PROMPT")]
    PARAMETERPROMPT,
    

    /// The response is from a transition route or an event handler in the page or flow or transition route group.
    ///
    /// "HANDLER_PROMPT"
    #[serde(rename="HANDLER_PROMPT")]
    HANDLERPROMPT,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::RESPONSETYPEUNSPECIFIED => "RESPONSE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::ENTRYPROMPT => "ENTRY_PROMPT",
            GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::PARAMETERPROMPT => "PARAMETER_PROMPT",
            GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::HANDLERPROMPT => "HANDLER_PROMPT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESPONSE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::RESPONSETYPEUNSPECIFIED),
           "ENTRY_PROMPT" => Ok(GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::ENTRYPROMPT),
           "PARAMETER_PROMPT" => Ok(GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::PARAMETERPROMPT),
           "HANDLER_PROMPT" => Ok(GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum::HANDLERPROMPT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ResponseMessageResponseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Agent restore mode. If not specified, `KEEP` is assumed.
pub enum GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum {
    

    /// Unspecified. Treated as KEEP.
    ///
    /// "RESTORE_OPTION_UNSPECIFIED"
    #[serde(rename="RESTORE_OPTION_UNSPECIFIED")]
    RESTOREOPTIONUNSPECIFIED,
    

    /// Always respect the settings from the exported agent file. It may cause a restoration failure if some settings (e.g. model type) are not supported in the target agent.
    ///
    /// "KEEP"
    #[serde(rename="KEEP")]
    KEEP,
    

    /// Fallback to default settings if some settings are not supported in the target agent.
    ///
    /// "FALLBACK"
    #[serde(rename="FALLBACK")]
    FALLBACK,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum::RESTOREOPTIONUNSPECIFIED => "RESTORE_OPTION_UNSPECIFIED",
            GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum::KEEP => "KEEP",
            GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum::FALLBACK => "FALLBACK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTORE_OPTION_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum::RESTOREOPTIONUNSPECIFIED),
           "KEEP" => Ok(GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum::KEEP),
           "FALLBACK" => Ok(GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum::FALLBACK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3RestoreAgentRequestRestoreOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of types of data to remove when retention settings triggers purge.
pub enum GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum {
    

    /// Unspecified. Do not use.
    ///
    /// "PURGE_DATA_TYPE_UNSPECIFIED"
    #[serde(rename="PURGE_DATA_TYPE_UNSPECIFIED")]
    PURGEDATATYPEUNSPECIFIED,
    

    /// Dialogflow history. This does not include Cloud logging, which is owned by the user - not Dialogflow.
    ///
    /// "DIALOGFLOW_HISTORY"
    #[serde(rename="DIALOGFLOW_HISTORY")]
    DIALOGFLOWHISTORY,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum::PURGEDATATYPEUNSPECIFIED => "PURGE_DATA_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum::DIALOGFLOWHISTORY => "DIALOGFLOW_HISTORY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PURGE_DATA_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum::PURGEDATATYPEUNSPECIFIED),
           "DIALOGFLOW_HISTORY" => Ok(GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum::DIALOGFLOWHISTORY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3SecuritySettingPurgeDataTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the data for which Dialogflow applies redaction. Dialogflow does not redact data that it does not have access to – for example, Cloud logging.
pub enum GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum {
    

    /// Don't redact any kind of data.
    ///
    /// "REDACTION_SCOPE_UNSPECIFIED"
    #[serde(rename="REDACTION_SCOPE_UNSPECIFIED")]
    REDACTIONSCOPEUNSPECIFIED,
    

    /// On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk.
    ///
    /// "REDACT_DISK_STORAGE"
    #[serde(rename="REDACT_DISK_STORAGE")]
    REDACTDISKSTORAGE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum::REDACTIONSCOPEUNSPECIFIED => "REDACTION_SCOPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum::REDACTDISKSTORAGE => "REDACT_DISK_STORAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDACTION_SCOPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum::REDACTIONSCOPEUNSPECIFIED),
           "REDACT_DISK_STORAGE" => Ok(GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum::REDACTDISKSTORAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3SecuritySettingRedactionScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Strategy that defines how we do redaction.
pub enum GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum {
    

    /// Do not redact.
    ///
    /// "REDACTION_STRATEGY_UNSPECIFIED"
    #[serde(rename="REDACTION_STRATEGY_UNSPECIFIED")]
    REDACTIONSTRATEGYUNSPECIFIED,
    

    /// Call redaction service to clean up the data to be persisted.
    ///
    /// "REDACT_WITH_SERVICE"
    #[serde(rename="REDACT_WITH_SERVICE")]
    REDACTWITHSERVICE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum::REDACTIONSTRATEGYUNSPECIFIED => "REDACTION_STRATEGY_UNSPECIFIED",
            GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum::REDACTWITHSERVICE => "REDACT_WITH_SERVICE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDACTION_STRATEGY_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum::REDACTIONSTRATEGYUNSPECIFIED),
           "REDACT_WITH_SERVICE" => Ok(GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum::REDACTWITHSERVICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3SecuritySettingRedactionStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the retention behavior defined by SecuritySettings.RetentionStrategy.
pub enum GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum {
    

    /// Retains the persisted data with Dialogflow's internal default 365d TTLs.
    ///
    /// "RETENTION_STRATEGY_UNSPECIFIED"
    #[serde(rename="RETENTION_STRATEGY_UNSPECIFIED")]
    RETENTIONSTRATEGYUNSPECIFIED,
    

    /// Removes data when the conversation ends. If there is no Conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends.
    ///
    /// "REMOVE_AFTER_CONVERSATION"
    #[serde(rename="REMOVE_AFTER_CONVERSATION")]
    REMOVEAFTERCONVERSATION,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum::RETENTIONSTRATEGYUNSPECIFIED => "RETENTION_STRATEGY_UNSPECIFIED",
            GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum::REMOVEAFTERCONVERSATION => "REMOVE_AFTER_CONVERSATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RETENTION_STRATEGY_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum::RETENTIONSTRATEGYUNSPECIFIED),
           "REMOVE_AFTER_CONVERSATION" => Ok(GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum::REMOVEAFTERCONVERSATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3SecuritySettingRetentionStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// File format for exported audio file. Currently only in telephony recordings.
pub enum GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum {
    

    /// Unspecified. Do not use.
    ///
    /// "AUDIO_FORMAT_UNSPECIFIED"
    #[serde(rename="AUDIO_FORMAT_UNSPECIFIED")]
    AUDIOFORMATUNSPECIFIED,
    

    /// G.711 mu-law PCM with 8kHz sample rate.
    ///
    /// "MULAW"
    #[serde(rename="MULAW")]
    MULAW,
    

    /// MP3 file format.
    ///
    /// "MP3"
    #[serde(rename="MP3")]
    MP3,
    

    /// OGG Vorbis.
    ///
    /// "OGG"
    #[serde(rename="OGG")]
    OGG,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::AUDIOFORMATUNSPECIFIED => "AUDIO_FORMAT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::MULAW => "MULAW",
            GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::MP3 => "MP3",
            GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::OGG => "OGG",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AUDIO_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::AUDIOFORMATUNSPECIFIED),
           "MULAW" => Ok(GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::MULAW),
           "MP3" => Ok(GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::MP3),
           "OGG" => Ok(GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum::OGG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3SecuritySettingsAudioExportSettingAudioFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Indicates whether the additional data should override or supplement the custom entity type definition.
pub enum GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEUNSPECIFIED => "ENTITY_OVERRIDE_MODE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEOVERRIDE => "ENTITY_OVERRIDE_MODE_OVERRIDE",
            GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODESUPPLEMENT => "ENTITY_OVERRIDE_MODE_SUPPLEMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_OVERRIDE_MODE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEUNSPECIFIED),
           "ENTITY_OVERRIDE_MODE_OVERRIDE" => Ok(GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODEOVERRIDE),
           "ENTITY_OVERRIDE_MODE_SUPPLEMENT" => Ok(GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum::ENTITYOVERRIDEMODESUPPLEMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3SessionEntityTypeEntityOverrideModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the test case passed in the agent environment.
pub enum GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
    

    /// Not specified. Should never be used.
    ///
    /// "TEST_RESULT_UNSPECIFIED"
    #[serde(rename="TEST_RESULT_UNSPECIFIED")]
    TESTRESULTUNSPECIFIED,
    

    /// The test passed.
    ///
    /// "PASSED"
    #[serde(rename="PASSED")]
    PASSED,
    

    /// The test did not pass.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum::TESTRESULTUNSPECIFIED => "TEST_RESULT_UNSPECIFIED",
            GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum::PASSED => "PASSED",
            GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEST_RESULT_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum::TESTRESULTUNSPECIFIED),
           "PASSED" => Ok(GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum::PASSED),
           "FAILED" => Ok(GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of diff.
pub enum GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
    

    /// Should never be used.
    ///
    /// "DIFF_TYPE_UNSPECIFIED"
    #[serde(rename="DIFF_TYPE_UNSPECIFIED")]
    DIFFTYPEUNSPECIFIED,
    

    /// The intent.
    ///
    /// "INTENT"
    #[serde(rename="INTENT")]
    INTENT,
    

    /// The page.
    ///
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
    

    /// The parameters.
    ///
    /// "PARAMETERS"
    #[serde(rename="PARAMETERS")]
    PARAMETERS,
    

    /// The message utterance.
    ///
    /// "UTTERANCE"
    #[serde(rename="UTTERANCE")]
    UTTERANCE,
    

    /// The flow.
    ///
    /// "FLOW"
    #[serde(rename="FLOW")]
    FLOW,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::DIFFTYPEUNSPECIFIED => "DIFF_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::INTENT => "INTENT",
            GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::PAGE => "PAGE",
            GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::PARAMETERS => "PARAMETERS",
            GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::UTTERANCE => "UTTERANCE",
            GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::FLOW => "FLOW",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIFF_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::DIFFTYPEUNSPECIFIED),
           "INTENT" => Ok(GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::INTENT),
           "PAGE" => Ok(GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::PAGE),
           "PARAMETERS" => Ok(GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::PARAMETERS),
           "UTTERANCE" => Ok(GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::UTTERANCE),
           "FLOW" => Ok(GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum::FLOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the resources where the message is found.
pub enum GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum {
    

    /// Unspecified.
    ///
    /// "RESOURCE_TYPE_UNSPECIFIED"
    #[serde(rename="RESOURCE_TYPE_UNSPECIFIED")]
    RESOURCETYPEUNSPECIFIED,
    

    /// Agent.
    ///
    /// "AGENT"
    #[serde(rename="AGENT")]
    AGENT,
    

    /// Intent.
    ///
    /// "INTENT"
    #[serde(rename="INTENT")]
    INTENT,
    

    /// Intent training phrase.
    ///
    /// "INTENT_TRAINING_PHRASE"
    #[serde(rename="INTENT_TRAINING_PHRASE")]
    INTENTTRAININGPHRASE,
    

    /// Intent parameter.
    ///
    /// "INTENT_PARAMETER"
    #[serde(rename="INTENT_PARAMETER")]
    INTENTPARAMETER,
    

    /// Multiple intents.
    ///
    /// "INTENTS"
    #[serde(rename="INTENTS")]
    INTENTS,
    

    /// Multiple training phrases.
    ///
    /// "INTENT_TRAINING_PHRASES"
    #[serde(rename="INTENT_TRAINING_PHRASES")]
    INTENTTRAININGPHRASES,
    

    /// Entity type.
    ///
    /// "ENTITY_TYPE"
    #[serde(rename="ENTITY_TYPE")]
    ENTITYTYPE,
    

    /// Multiple entity types.
    ///
    /// "ENTITY_TYPES"
    #[serde(rename="ENTITY_TYPES")]
    ENTITYTYPES,
    

    /// Webhook.
    ///
    /// "WEBHOOK"
    #[serde(rename="WEBHOOK")]
    WEBHOOK,
    

    /// Flow.
    ///
    /// "FLOW"
    #[serde(rename="FLOW")]
    FLOW,
    

    /// Page.
    ///
    /// "PAGE"
    #[serde(rename="PAGE")]
    PAGE,
    

    /// Multiple pages.
    ///
    /// "PAGES"
    #[serde(rename="PAGES")]
    PAGES,
    

    /// Transition route group.
    ///
    /// "TRANSITION_ROUTE_GROUP"
    #[serde(rename="TRANSITION_ROUTE_GROUP")]
    TRANSITIONROUTEGROUP,
    

    /// Agent transition route group.
    ///
    /// "AGENT_TRANSITION_ROUTE_GROUP"
    #[serde(rename="AGENT_TRANSITION_ROUTE_GROUP")]
    AGENTTRANSITIONROUTEGROUP,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::RESOURCETYPEUNSPECIFIED => "RESOURCE_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::AGENT => "AGENT",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENT => "INTENT",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTTRAININGPHRASE => "INTENT_TRAINING_PHRASE",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTPARAMETER => "INTENT_PARAMETER",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTS => "INTENTS",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTTRAININGPHRASES => "INTENT_TRAINING_PHRASES",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::ENTITYTYPE => "ENTITY_TYPE",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::ENTITYTYPES => "ENTITY_TYPES",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::WEBHOOK => "WEBHOOK",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::FLOW => "FLOW",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::PAGE => "PAGE",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::PAGES => "PAGES",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::TRANSITIONROUTEGROUP => "TRANSITION_ROUTE_GROUP",
            GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::AGENTTRANSITIONROUTEGROUP => "AGENT_TRANSITION_ROUTE_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESOURCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::RESOURCETYPEUNSPECIFIED),
           "AGENT" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::AGENT),
           "INTENT" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENT),
           "INTENT_TRAINING_PHRASE" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTTRAININGPHRASE),
           "INTENT_PARAMETER" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTPARAMETER),
           "INTENTS" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTS),
           "INTENT_TRAINING_PHRASES" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::INTENTTRAININGPHRASES),
           "ENTITY_TYPE" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::ENTITYTYPE),
           "ENTITY_TYPES" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::ENTITYTYPES),
           "WEBHOOK" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::WEBHOOK),
           "FLOW" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::FLOW),
           "PAGE" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::PAGE),
           "PAGES" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::PAGES),
           "TRANSITION_ROUTE_GROUP" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::TRANSITIONROUTEGROUP),
           "AGENT_TRANSITION_ROUTE_GROUP" => Ok(GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum::AGENTTRANSITIONROUTEGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ValidationMessageResourceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the severity of the message.
pub enum GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum {
    

    /// Unspecified.
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
    

    /// The agent may experience failures.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::INFO => "INFO",
            GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::WARNING => "WARNING",
            GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::SEVERITYUNSPECIFIED),
           "INFO" => Ok(GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::INFO),
           "WARNING" => Ok(GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::WARNING),
           "ERROR" => Ok(GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3ValidationMessageSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3VersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this version. This field is read-only and cannot be set by create and update methods.
pub enum GoogleCloudDialogflowCxV3VersionStateEnum {
    

    /// Not specified. This value is not used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Version is not ready to serve (e.g. training is running).
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Training has succeeded and this version is ready to serve.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Version training failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3VersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3VersionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3VersionStateEnum::RUNNING => "RUNNING",
            GoogleCloudDialogflowCxV3VersionStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudDialogflowCxV3VersionStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3VersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3VersionStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GoogleCloudDialogflowCxV3VersionStateEnum::RUNNING),
           "SUCCEEDED" => Ok(GoogleCloudDialogflowCxV3VersionStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudDialogflowCxV3VersionStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3VersionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer substitutes a voice with a different gender rather than failing the request.
pub enum GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum {
    

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

impl AsRef<str> for GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED => "SSML_VOICE_GENDER_UNSPECIFIED",
            GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERMALE => "SSML_VOICE_GENDER_MALE",
            GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERFEMALE => "SSML_VOICE_GENDER_FEMALE",
            GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERNEUTRAL => "SSML_VOICE_GENDER_NEUTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SSML_VOICE_GENDER_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERUNSPECIFIED),
           "SSML_VOICE_GENDER_MALE" => Ok(GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERMALE),
           "SSML_VOICE_GENDER_FEMALE" => Ok(GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERFEMALE),
           "SSML_VOICE_GENDER_NEUTRAL" => Ok(GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum::SSMLVOICEGENDERNEUTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3VoiceSelectionParamSsmlGenderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. HTTP method for the flexible webhook calls. Standard webhook always uses POST.
pub enum GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum {
    

    /// HTTP method not specified.
    ///
    /// "HTTP_METHOD_UNSPECIFIED"
    #[serde(rename="HTTP_METHOD_UNSPECIFIED")]
    HTTPMETHODUNSPECIFIED,
    

    /// HTTP POST Method.
    ///
    /// "POST"
    #[serde(rename="POST")]
    POST,
    

    /// HTTP GET Method.
    ///
    /// "GET"
    #[serde(rename="GET")]
    GET,
    

    /// HTTP HEAD Method.
    ///
    /// "HEAD"
    #[serde(rename="HEAD")]
    HEAD,
    

    /// HTTP PUT Method.
    ///
    /// "PUT"
    #[serde(rename="PUT")]
    PUT,
    

    /// HTTP DELETE Method.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// HTTP PATCH Method.
    ///
    /// "PATCH"
    #[serde(rename="PATCH")]
    PATCH,
    

    /// HTTP OPTIONS Method.
    ///
    /// "OPTIONS"
    #[serde(rename="OPTIONS")]
    OPTIONS,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::HTTPMETHODUNSPECIFIED => "HTTP_METHOD_UNSPECIFIED",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::POST => "POST",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::GET => "GET",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::HEAD => "HEAD",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::PUT => "PUT",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::DELETE => "DELETE",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::PATCH => "PATCH",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::OPTIONS => "OPTIONS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HTTP_METHOD_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::HTTPMETHODUNSPECIFIED),
           "POST" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::POST),
           "GET" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::GET),
           "HEAD" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::HEAD),
           "PUT" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::PUT),
           "DELETE" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::DELETE),
           "PATCH" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::PATCH),
           "OPTIONS" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum::OPTIONS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3WebhookGenericWebServiceHttpMethodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicate the auth token type generated from the [Diglogflow service agent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent). The generated token is sent in the Authorization header.
pub enum GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum {
    

    /// Service agent auth type unspecified. Default to ID_TOKEN.
    ///
    /// "SERVICE_AGENT_AUTH_UNSPECIFIED"
    #[serde(rename="SERVICE_AGENT_AUTH_UNSPECIFIED")]
    SERVICEAGENTAUTHUNSPECIFIED,
    

    /// No token used.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Use [ID token](https://cloud.google.com/docs/authentication/token-types#id) generated from service agent. This can be used to access Cloud Function and Cloud Run after you grant Invoker role to `service-@gcp-sa-dialogflow.iam.gserviceaccount.com`.
    ///
    /// "ID_TOKEN"
    #[serde(rename="ID_TOKEN")]
    IDTOKEN,
    

    /// Use [access token](https://cloud.google.com/docs/authentication/token-types#access) generated from service agent. This can be used to access other Google Cloud APIs after you grant required roles to `service-@gcp-sa-dialogflow.iam.gserviceaccount.com`.
    ///
    /// "ACCESS_TOKEN"
    #[serde(rename="ACCESS_TOKEN")]
    ACCESSTOKEN,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::SERVICEAGENTAUTHUNSPECIFIED => "SERVICE_AGENT_AUTH_UNSPECIFIED",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::NONE => "NONE",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::IDTOKEN => "ID_TOKEN",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::ACCESSTOKEN => "ACCESS_TOKEN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_AGENT_AUTH_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::SERVICEAGENTAUTHUNSPECIFIED),
           "NONE" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::NONE),
           "ID_TOKEN" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::IDTOKEN),
           "ACCESS_TOKEN" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum::ACCESSTOKEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3WebhookGenericWebServiceServiceAgentAuthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of the webhook.
pub enum GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum {
    

    /// Default value. This value is unused.
    ///
    /// "WEBHOOK_TYPE_UNSPECIFIED"
    #[serde(rename="WEBHOOK_TYPE_UNSPECIFIED")]
    WEBHOOKTYPEUNSPECIFIED,
    

    /// Represents a standard webhook.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Represents a flexible webhook.
    ///
    /// "FLEXIBLE"
    #[serde(rename="FLEXIBLE")]
    FLEXIBLE,
}

impl AsRef<str> for GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum::WEBHOOKTYPEUNSPECIFIED => "WEBHOOK_TYPE_UNSPECIFIED",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum::STANDARD => "STANDARD",
            GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum::FLEXIBLE => "FLEXIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEBHOOK_TYPE_UNSPECIFIED" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum::WEBHOOKTYPEUNSPECIFIED),
           "STANDARD" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum::STANDARD),
           "FLEXIBLE" => Ok(GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum::FLEXIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDialogflowCxV3WebhookGenericWebServiceWebhookTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectIntentViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The resource view to apply to the returned intent.
pub enum ProjectIntentViewEnum {
    

    /// Not specified. Treated as INTENT_VIEW_FULL.
    ///
    /// "INTENT_VIEW_UNSPECIFIED"
    #[serde(rename="INTENT_VIEW_UNSPECIFIED")]
    INTENTVIEWUNSPECIFIED,
    

    /// Training phrases field is not populated in the response.
    ///
    /// "INTENT_VIEW_PARTIAL"
    #[serde(rename="INTENT_VIEW_PARTIAL")]
    INTENTVIEWPARTIAL,
    

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
            ProjectIntentViewEnum::INTENTVIEWPARTIAL => "INTENT_VIEW_PARTIAL",
            ProjectIntentViewEnum::INTENTVIEWFULL => "INTENT_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectIntentViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTENT_VIEW_UNSPECIFIED" => Ok(ProjectIntentViewEnum::INTENTVIEWUNSPECIFIED),
           "INTENT_VIEW_PARTIAL" => Ok(ProjectIntentViewEnum::INTENTVIEWPARTIAL),
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


// region ProjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of coverage requested.
pub enum ProjectTypeEnum {
    

    /// Should never be used.
    ///
    /// "COVERAGE_TYPE_UNSPECIFIED"
    #[serde(rename="COVERAGE_TYPE_UNSPECIFIED")]
    COVERAGETYPEUNSPECIFIED,
    

    /// Intent coverage.
    ///
    /// "INTENT"
    #[serde(rename="INTENT")]
    INTENT,
    

    /// Page transition coverage.
    ///
    /// "PAGE_TRANSITION"
    #[serde(rename="PAGE_TRANSITION")]
    PAGETRANSITION,
    

    /// Transition route group coverage.
    ///
    /// "TRANSITION_ROUTE_GROUP"
    #[serde(rename="TRANSITION_ROUTE_GROUP")]
    TRANSITIONROUTEGROUP,
}

impl AsRef<str> for ProjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectTypeEnum::COVERAGETYPEUNSPECIFIED => "COVERAGE_TYPE_UNSPECIFIED",
            ProjectTypeEnum::INTENT => "INTENT",
            ProjectTypeEnum::PAGETRANSITION => "PAGE_TRANSITION",
            ProjectTypeEnum::TRANSITIONROUTEGROUP => "TRANSITION_ROUTE_GROUP",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COVERAGE_TYPE_UNSPECIFIED" => Ok(ProjectTypeEnum::COVERAGETYPEUNSPECIFIED),
           "INTENT" => Ok(ProjectTypeEnum::INTENT),
           "PAGE_TRANSITION" => Ok(ProjectTypeEnum::PAGETRANSITION),
           "TRANSITION_ROUTE_GROUP" => Ok(ProjectTypeEnum::TRANSITIONROUTEGROUP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether response should include all fields or just the metadata.
pub enum ProjectViewEnum {
    

    /// The default / unset value. The API will default to the BASIC view.
    ///
    /// "TEST_CASE_VIEW_UNSPECIFIED"
    #[serde(rename="TEST_CASE_VIEW_UNSPECIFIED")]
    TESTCASEVIEWUNSPECIFIED,
    

    /// Include basic metadata about the test case, but not the conversation turns. This is the default value.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include everything.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::TESTCASEVIEWUNSPECIFIED => "TEST_CASE_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TEST_CASE_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::TESTCASEVIEWUNSPECIFIED),
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


