use super::*;



// region GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the cost is calculated.
pub enum GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum {
    

    /// Default pricing type.
    ///
    /// "PRICING_TYPE_UNSPECIFIED"
    #[serde(rename="PRICING_TYPE_UNSPECIFIED")]
    PRICINGTYPEUNSPECIFIED,
    

    /// The price listed by GCP for all customers.
    ///
    /// "LIST_PRICE"
    #[serde(rename="LIST_PRICE")]
    LISTPRICE,
    

    /// A price derived from past usage and billing.
    ///
    /// "CUSTOM_PRICE"
    #[serde(rename="CUSTOM_PRICE")]
    CUSTOMPRICE,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum::PRICINGTYPEUNSPECIFIED => "PRICING_TYPE_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum::LISTPRICE => "LIST_PRICE",
            GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum::CUSTOMPRICE => "CUSTOM_PRICE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICING_TYPE_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum::PRICINGTYPEUNSPECIFIED),
           "LIST_PRICE" => Ok(GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum::LISTPRICE),
           "CUSTOM_PRICE" => Ok(GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum::CUSTOMPRICE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1CostProjectionPricingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1ImpactCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Category that is being targeted.
pub enum GoogleCloudRecommenderV1beta1ImpactCategoryEnum {
    

    /// Default unspecified category. Don't use directly.
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// Indicates a potential increase or decrease in cost.
    ///
    /// "COST"
    #[serde(rename="COST")]
    COST,
    

    /// Indicates a potential increase or decrease in security.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// Indicates a potential increase or decrease in performance.
    ///
    /// "PERFORMANCE"
    #[serde(rename="PERFORMANCE")]
    PERFORMANCE,
    

    /// Indicates a potential increase or decrease in manageability.
    ///
    /// "MANAGEABILITY"
    #[serde(rename="MANAGEABILITY")]
    MANAGEABILITY,
    

    /// Indicates a potential increase or decrease in sustainability.
    ///
    /// "SUSTAINABILITY"
    #[serde(rename="SUSTAINABILITY")]
    SUSTAINABILITY,
    

    /// Indicates a potential increase or decrease in reliability.
    ///
    /// "RELIABILITY"
    #[serde(rename="RELIABILITY")]
    RELIABILITY,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1ImpactCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::COST => "COST",
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::SECURITY => "SECURITY",
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::PERFORMANCE => "PERFORMANCE",
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::MANAGEABILITY => "MANAGEABILITY",
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::SUSTAINABILITY => "SUSTAINABILITY",
            GoogleCloudRecommenderV1beta1ImpactCategoryEnum::RELIABILITY => "RELIABILITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1ImpactCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::CATEGORYUNSPECIFIED),
           "COST" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::COST),
           "SECURITY" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::SECURITY),
           "PERFORMANCE" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::PERFORMANCE),
           "MANAGEABILITY" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::MANAGEABILITY),
           "SUSTAINABILITY" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::SUSTAINABILITY),
           "RELIABILITY" => Ok(GoogleCloudRecommenderV1beta1ImpactCategoryEnum::RELIABILITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1ImpactCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1InsightCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Category being targeted by the insight.
pub enum GoogleCloudRecommenderV1beta1InsightCategoryEnum {
    

    /// Unspecified category.
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// The insight is related to cost.
    ///
    /// "COST"
    #[serde(rename="COST")]
    COST,
    

    /// The insight is related to security.
    ///
    /// "SECURITY"
    #[serde(rename="SECURITY")]
    SECURITY,
    

    /// The insight is related to performance.
    ///
    /// "PERFORMANCE"
    #[serde(rename="PERFORMANCE")]
    PERFORMANCE,
    

    /// This insight is related to manageability.
    ///
    /// "MANAGEABILITY"
    #[serde(rename="MANAGEABILITY")]
    MANAGEABILITY,
    

    /// The insight is related to sustainability.
    ///
    /// "SUSTAINABILITY"
    #[serde(rename="SUSTAINABILITY")]
    SUSTAINABILITY,
    

    /// The insight is related to reliability.
    ///
    /// "RELIABILITY"
    #[serde(rename="RELIABILITY")]
    RELIABILITY,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1InsightCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::COST => "COST",
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::SECURITY => "SECURITY",
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::PERFORMANCE => "PERFORMANCE",
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::MANAGEABILITY => "MANAGEABILITY",
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::SUSTAINABILITY => "SUSTAINABILITY",
            GoogleCloudRecommenderV1beta1InsightCategoryEnum::RELIABILITY => "RELIABILITY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1InsightCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::CATEGORYUNSPECIFIED),
           "COST" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::COST),
           "SECURITY" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::SECURITY),
           "PERFORMANCE" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::PERFORMANCE),
           "MANAGEABILITY" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::MANAGEABILITY),
           "SUSTAINABILITY" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::SUSTAINABILITY),
           "RELIABILITY" => Ok(GoogleCloudRecommenderV1beta1InsightCategoryEnum::RELIABILITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1InsightCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1InsightSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Insight's severity.
pub enum GoogleCloudRecommenderV1beta1InsightSeverityEnum {
    

    /// Insight has unspecified severity.
    ///
    /// "SEVERITY_UNSPECIFIED"
    #[serde(rename="SEVERITY_UNSPECIFIED")]
    SEVERITYUNSPECIFIED,
    

    /// Insight has low severity.
    ///
    /// "LOW"
    #[serde(rename="LOW")]
    LOW,
    

    /// Insight has medium severity.
    ///
    /// "MEDIUM"
    #[serde(rename="MEDIUM")]
    MEDIUM,
    

    /// Insight has high severity.
    ///
    /// "HIGH"
    #[serde(rename="HIGH")]
    HIGH,
    

    /// Insight has critical severity.
    ///
    /// "CRITICAL"
    #[serde(rename="CRITICAL")]
    CRITICAL,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1InsightSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1InsightSeverityEnum::SEVERITYUNSPECIFIED => "SEVERITY_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1InsightSeverityEnum::LOW => "LOW",
            GoogleCloudRecommenderV1beta1InsightSeverityEnum::MEDIUM => "MEDIUM",
            GoogleCloudRecommenderV1beta1InsightSeverityEnum::HIGH => "HIGH",
            GoogleCloudRecommenderV1beta1InsightSeverityEnum::CRITICAL => "CRITICAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1InsightSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEVERITY_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1InsightSeverityEnum::SEVERITYUNSPECIFIED),
           "LOW" => Ok(GoogleCloudRecommenderV1beta1InsightSeverityEnum::LOW),
           "MEDIUM" => Ok(GoogleCloudRecommenderV1beta1InsightSeverityEnum::MEDIUM),
           "HIGH" => Ok(GoogleCloudRecommenderV1beta1InsightSeverityEnum::HIGH),
           "CRITICAL" => Ok(GoogleCloudRecommenderV1beta1InsightSeverityEnum::CRITICAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1InsightSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Insight state.
pub enum GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Insight is active. Content for ACTIVE insights can be updated by Google. ACTIVE insights can be marked DISMISSED OR ACCEPTED.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Some action has been taken based on this insight. Insights become accepted when a recommendation derived from the insight has been marked CLAIMED, SUCCEEDED, or FAILED. ACTIVE insights can also be marked ACCEPTED explicitly. Content for ACCEPTED insights is immutable. ACCEPTED insights can only be marked ACCEPTED (which may update state metadata).
    ///
    /// "ACCEPTED"
    #[serde(rename="ACCEPTED")]
    ACCEPTED,
    

    /// Insight is dismissed. Content for DISMISSED insights can be updated by Google. DISMISSED insights can be marked as ACTIVE.
    ///
    /// "DISMISSED"
    #[serde(rename="DISMISSED")]
    DISMISSED,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::ACCEPTED => "ACCEPTED",
            GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::DISMISSED => "DISMISSED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::ACTIVE),
           "ACCEPTED" => Ok(GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::ACCEPTED),
           "DISMISSED" => Ok(GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum::DISMISSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1InsightStateInfoStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1RecommendationPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Recommendation's priority.
pub enum GoogleCloudRecommenderV1beta1RecommendationPriorityEnum {
    

    /// Recommendation has unspecified priority.
    ///
    /// "PRIORITY_UNSPECIFIED"
    #[serde(rename="PRIORITY_UNSPECIFIED")]
    PRIORITYUNSPECIFIED,
    

    /// Recommendation has P4 priority (lowest priority).
    ///
    /// "P4"
    #[serde(rename="P4")]
    P4,
    

    /// Recommendation has P3 priority (second lowest priority).
    ///
    /// "P3"
    #[serde(rename="P3")]
    P3,
    

    /// Recommendation has P2 priority (second highest priority).
    ///
    /// "P2"
    #[serde(rename="P2")]
    P2,
    

    /// Recommendation has P1 priority (highest priority).
    ///
    /// "P1"
    #[serde(rename="P1")]
    P1,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1RecommendationPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::PRIORITYUNSPECIFIED => "PRIORITY_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P4 => "P4",
            GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P3 => "P3",
            GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P2 => "P2",
            GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P1 => "P1",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1RecommendationPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIORITY_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::PRIORITYUNSPECIFIED),
           "P4" => Ok(GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P4),
           "P3" => Ok(GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P3),
           "P2" => Ok(GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P2),
           "P1" => Ok(GoogleCloudRecommenderV1beta1RecommendationPriorityEnum::P1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1RecommendationPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED.
pub enum GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum {
    

    /// Default state. Don't use directly.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Recommendation is active and can be applied. Recommendations content can be updated by Google. ACTIVE recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Recommendation is in claimed state. Recommendations content is immutable and cannot be updated by Google. CLAIMED recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
    ///
    /// "CLAIMED"
    #[serde(rename="CLAIMED")]
    CLAIMED,
    

    /// Recommendation is in succeeded state. Recommendations content is immutable and cannot be updated by Google. SUCCEEDED recommendations can be marked as SUCCEEDED, or FAILED.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Recommendation is in failed state. Recommendations content is immutable and cannot be updated by Google. FAILED recommendations can be marked as SUCCEEDED, or FAILED.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Recommendation is in dismissed state. Recommendation content can be updated by Google. DISMISSED recommendations can be marked as ACTIVE.
    ///
    /// "DISMISSED"
    #[serde(rename="DISMISSED")]
    DISMISSED,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::CLAIMED => "CLAIMED",
            GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::FAILED => "FAILED",
            GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::DISMISSED => "DISMISSED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::ACTIVE),
           "CLAIMED" => Ok(GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::CLAIMED),
           "SUCCEEDED" => Ok(GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::FAILED),
           "DISMISSED" => Ok(GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum::DISMISSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1RecommendationStateInfoStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reliability risks mitigated by this recommendation.
pub enum GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum {
    

    /// Default unspecified risk. Don't use directly.
    ///
    /// "RISK_TYPE_UNSPECIFIED"
    #[serde(rename="RISK_TYPE_UNSPECIFIED")]
    RISKTYPEUNSPECIFIED,
    

    /// Potential service downtime.
    ///
    /// "SERVICE_DISRUPTION"
    #[serde(rename="SERVICE_DISRUPTION")]
    SERVICEDISRUPTION,
    

    /// Potential data loss.
    ///
    /// "DATA_LOSS"
    #[serde(rename="DATA_LOSS")]
    DATALOSS,
    

    /// Potential access denial. The service is still up but some or all clients can't access it.
    ///
    /// "ACCESS_DENY"
    #[serde(rename="ACCESS_DENY")]
    ACCESSDENY,
}

impl AsRef<str> for GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::RISKTYPEUNSPECIFIED => "RISK_TYPE_UNSPECIFIED",
            GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::SERVICEDISRUPTION => "SERVICE_DISRUPTION",
            GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::DATALOSS => "DATA_LOSS",
            GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::ACCESSDENY => "ACCESS_DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RISK_TYPE_UNSPECIFIED" => Ok(GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::RISKTYPEUNSPECIFIED),
           "SERVICE_DISRUPTION" => Ok(GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::SERVICEDISRUPTION),
           "DATA_LOSS" => Ok(GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::DATALOSS),
           "ACCESS_DENY" => Ok(GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum::ACCESSDENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudRecommenderV1beta1ReliabilityProjectionRisksEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


