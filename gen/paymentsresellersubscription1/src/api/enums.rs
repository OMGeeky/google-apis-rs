use super::*;



// region GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the reason for the cancellation.
pub enum GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum {
    

    /// Reason is unspecified.
    ///
    /// "CANCELLATION_REASON_UNSPECIFIED"
    #[serde(rename="CANCELLATION_REASON_UNSPECIFIED")]
    CANCELLATIONREASONUNSPECIFIED,
    

    /// Fraudualant transaction.
    ///
    /// "CANCELLATION_REASON_FRAUD"
    #[serde(rename="CANCELLATION_REASON_FRAUD")]
    CANCELLATIONREASONFRAUD,
    

    /// Buyer's remorse.
    ///
    /// "CANCELLATION_REASON_REMORSE"
    #[serde(rename="CANCELLATION_REASON_REMORSE")]
    CANCELLATIONREASONREMORSE,
    

    /// Accidential purchase.
    ///
    /// "CANCELLATION_REASON_ACCIDENTAL_PURCHASE"
    #[serde(rename="CANCELLATION_REASON_ACCIDENTAL_PURCHASE")]
    CANCELLATIONREASONACCIDENTALPURCHASE,
    

    /// Payment is past due.
    ///
    /// "CANCELLATION_REASON_PAST_DUE"
    #[serde(rename="CANCELLATION_REASON_PAST_DUE")]
    CANCELLATIONREASONPASTDUE,
    

    /// Used for notification only, do not use in Cancel API. User account closed.
    ///
    /// "CANCELLATION_REASON_ACCOUNT_CLOSED"
    #[serde(rename="CANCELLATION_REASON_ACCOUNT_CLOSED")]
    CANCELLATIONREASONACCOUNTCLOSED,
    

    /// Used for notification only, do not use in Cancel API. Cancellation due to upgrade or downgrade.
    ///
    /// "CANCELLATION_REASON_UPGRADE_DOWNGRADE"
    #[serde(rename="CANCELLATION_REASON_UPGRADE_DOWNGRADE")]
    CANCELLATIONREASONUPGRADEDOWNGRADE,
    

    /// Cancellation due to user delinquency
    ///
    /// "CANCELLATION_REASON_USER_DELINQUENCY"
    #[serde(rename="CANCELLATION_REASON_USER_DELINQUENCY")]
    CANCELLATIONREASONUSERDELINQUENCY,
    

    /// Used for notification only, do not use in Cancel API. Cancellation due to an unrecoverable system error.
    ///
    /// "CANCELLATION_REASON_SYSTEM_ERROR"
    #[serde(rename="CANCELLATION_REASON_SYSTEM_ERROR")]
    CANCELLATIONREASONSYSTEMERROR,
    

    /// Used for notification only, do not use in Cancel API. The subscription is cancelled by Google automatically since it is no longer valid.
    ///
    /// "CANCELLATION_REASON_SYSTEM_CANCEL"
    #[serde(rename="CANCELLATION_REASON_SYSTEM_CANCEL")]
    CANCELLATIONREASONSYSTEMCANCEL,
    

    /// Other reason.
    ///
    /// "CANCELLATION_REASON_OTHER"
    #[serde(rename="CANCELLATION_REASON_OTHER")]
    CANCELLATIONREASONOTHER,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONUNSPECIFIED => "CANCELLATION_REASON_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONFRAUD => "CANCELLATION_REASON_FRAUD",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONREMORSE => "CANCELLATION_REASON_REMORSE",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONACCIDENTALPURCHASE => "CANCELLATION_REASON_ACCIDENTAL_PURCHASE",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONPASTDUE => "CANCELLATION_REASON_PAST_DUE",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONACCOUNTCLOSED => "CANCELLATION_REASON_ACCOUNT_CLOSED",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONUPGRADEDOWNGRADE => "CANCELLATION_REASON_UPGRADE_DOWNGRADE",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONUSERDELINQUENCY => "CANCELLATION_REASON_USER_DELINQUENCY",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONSYSTEMERROR => "CANCELLATION_REASON_SYSTEM_ERROR",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONSYSTEMCANCEL => "CANCELLATION_REASON_SYSTEM_CANCEL",
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONOTHER => "CANCELLATION_REASON_OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CANCELLATION_REASON_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONUNSPECIFIED),
           "CANCELLATION_REASON_FRAUD" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONFRAUD),
           "CANCELLATION_REASON_REMORSE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONREMORSE),
           "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONACCIDENTALPURCHASE),
           "CANCELLATION_REASON_PAST_DUE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONPASTDUE),
           "CANCELLATION_REASON_ACCOUNT_CLOSED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONACCOUNTCLOSED),
           "CANCELLATION_REASON_UPGRADE_DOWNGRADE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONUPGRADEDOWNGRADE),
           "CANCELLATION_REASON_USER_DELINQUENCY" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONUSERDELINQUENCY),
           "CANCELLATION_REASON_SYSTEM_ERROR" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONSYSTEMERROR),
           "CANCELLATION_REASON_SYSTEM_CANCEL" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONSYSTEMCANCEL),
           "CANCELLATION_REASON_OTHER" => Ok(GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum::CANCELLATIONREASONOTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The unit used for the duration
pub enum GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum {
    

    /// Default value, reserved as an invalid or an unexpected value.
    ///
    /// "UNIT_UNSPECIFIED"
    #[serde(rename="UNIT_UNSPECIFIED")]
    UNITUNSPECIFIED,
    

    /// Unit of a calendar month.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Unit of a day.
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// Unit of an hour. It is used for testing.
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::UNITUNSPECIFIED => "UNIT_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::MONTH => "MONTH",
            GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::DAY => "DAY",
            GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::HOUR => "HOUR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNIT_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::UNITUNSPECIFIED),
           "MONTH" => Ok(GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::MONTH),
           "DAY" => Ok(GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::DAY),
           "HOUR" => Ok(GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum::HOUR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1DurationUnitEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of offering the subscription was sold by the partner. e.g. VAS.
pub enum GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum {
    

    /// The type of partner offering is unspecified.
    ///
    /// "OFFERING_UNSPECIFIED"
    #[serde(rename="OFFERING_UNSPECIFIED")]
    OFFERINGUNSPECIFIED,
    

    /// Google One product purchased as a Value added service in addition to existing partner's products. Customer pays additional amount for Google One product.
    ///
    /// "OFFERING_VAS_BUNDLE"
    #[serde(rename="OFFERING_VAS_BUNDLE")]
    OFFERINGVASBUNDLE,
    

    /// Google One product purchased by itself by customer as a value add service. Customer pays additional amount for Google One product.
    ///
    /// "OFFERING_VAS_STANDALONE"
    #[serde(rename="OFFERING_VAS_STANDALONE")]
    OFFERINGVASSTANDALONE,
    

    /// Product purchased as part of a hard bundle where Google One was included with the bundle. Google One pricing is included in the bundle.
    ///
    /// "OFFERING_HARD_BUNDLE"
    #[serde(rename="OFFERING_HARD_BUNDLE")]
    OFFERINGHARDBUNDLE,
    

    /// Purchased as part of a bundle where Google One was provided as an option. Google One pricing is included in the bundle.
    ///
    /// "OFFERING_SOFT_BUNDLE"
    #[serde(rename="OFFERING_SOFT_BUNDLE")]
    OFFERINGSOFTBUNDLE,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGUNSPECIFIED => "OFFERING_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGVASBUNDLE => "OFFERING_VAS_BUNDLE",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGVASSTANDALONE => "OFFERING_VAS_STANDALONE",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGHARDBUNDLE => "OFFERING_HARD_BUNDLE",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGSOFTBUNDLE => "OFFERING_SOFT_BUNDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OFFERING_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGUNSPECIFIED),
           "OFFERING_VAS_BUNDLE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGVASBUNDLE),
           "OFFERING_VAS_STANDALONE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGVASSTANDALONE),
           "OFFERING_HARD_BUNDLE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGHARDBUNDLE),
           "OFFERING_SOFT_BUNDLE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum::OFFERINGSOFTBUNDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOfferingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of sales channel through which the subscription was sold.
pub enum GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum {
    

    /// The channel type is unspecified.
    ///
    /// "CHANNEL_UNSPECIFIED"
    #[serde(rename="CHANNEL_UNSPECIFIED")]
    CHANNELUNSPECIFIED,
    

    /// Sold at store.
    ///
    /// "CHANNEL_RETAIL"
    #[serde(rename="CHANNEL_RETAIL")]
    CHANNELRETAIL,
    

    /// Sold through partner website.
    ///
    /// "CHANNEL_ONLINE_WEB"
    #[serde(rename="CHANNEL_ONLINE_WEB")]
    CHANNELONLINEWEB,
    

    /// Sold through partner android app.
    ///
    /// "CHANNEL_ONLINE_ANDROID_APP"
    #[serde(rename="CHANNEL_ONLINE_ANDROID_APP")]
    CHANNELONLINEANDROIDAPP,
    

    /// Sold through partner iOS app.
    ///
    /// "CHANNEL_ONLINE_IOS_APP"
    #[serde(rename="CHANNEL_ONLINE_IOS_APP")]
    CHANNELONLINEIOSAPP,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELUNSPECIFIED => "CHANNEL_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELRETAIL => "CHANNEL_RETAIL",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELONLINEWEB => "CHANNEL_ONLINE_WEB",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELONLINEANDROIDAPP => "CHANNEL_ONLINE_ANDROID_APP",
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELONLINEIOSAPP => "CHANNEL_ONLINE_IOS_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANNEL_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELUNSPECIFIED),
           "CHANNEL_RETAIL" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELRETAIL),
           "CHANNEL_ONLINE_WEB" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELONLINEWEB),
           "CHANNEL_ONLINE_ANDROID_APP" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELONLINEANDROIDAPP),
           "CHANNEL_ONLINE_IOS_APP" => Ok(GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum::CHANNELONLINEIOSAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Output Only. Specifies the type of the product.
pub enum GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum {
    

    /// Unspecified. It's reserved as an unexpected value, should not be used.
    ///
    /// "PRODUCT_TYPE_UNSPECIFIED"
    #[serde(rename="PRODUCT_TYPE_UNSPECIFIED")]
    PRODUCTTYPEUNSPECIFIED,
    

    /// The product is a subscription.
    ///
    /// "PRODUCT_TYPE_SUBSCRIPTION"
    #[serde(rename="PRODUCT_TYPE_SUBSCRIPTION")]
    PRODUCTTYPESUBSCRIPTION,
    

    /// The product is a bundled subscription plan, which includes multiple subscription elements.
    ///
    /// "PRODUCT_TYPE_BUNDLE_SUBSCRIPTION"
    #[serde(rename="PRODUCT_TYPE_BUNDLE_SUBSCRIPTION")]
    PRODUCTTYPEBUNDLESUBSCRIPTION,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum::PRODUCTTYPEUNSPECIFIED => "PRODUCT_TYPE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum::PRODUCTTYPESUBSCRIPTION => "PRODUCT_TYPE_SUBSCRIPTION",
            GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum::PRODUCTTYPEBUNDLESUBSCRIPTION => "PRODUCT_TYPE_BUNDLE_SUBSCRIPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRODUCT_TYPE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum::PRODUCTTYPEUNSPECIFIED),
           "PRODUCT_TYPE_SUBSCRIPTION" => Ok(GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum::PRODUCTTYPESUBSCRIPTION),
           "PRODUCT_TYPE_BUNDLE_SUBSCRIPTION" => Ok(GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum::PRODUCTTYPEBUNDLESUBSCRIPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1ProductProductTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Output Only. Specifies the type of the promotion.
pub enum GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum {
    

    /// The promotion type is unspecified.
    ///
    /// "PROMOTION_TYPE_UNSPECIFIED"
    #[serde(rename="PROMOTION_TYPE_UNSPECIFIED")]
    PROMOTIONTYPEUNSPECIFIED,
    

    /// The promotion is a free trial.
    ///
    /// "PROMOTION_TYPE_FREE_TRIAL"
    #[serde(rename="PROMOTION_TYPE_FREE_TRIAL")]
    PROMOTIONTYPEFREETRIAL,
    

    /// The promotion is a reduced introductory pricing.
    ///
    /// "PROMOTION_TYPE_INTRODUCTORY_PRICING"
    #[serde(rename="PROMOTION_TYPE_INTRODUCTORY_PRICING")]
    PROMOTIONTYPEINTRODUCTORYPRICING,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum::PROMOTIONTYPEUNSPECIFIED => "PROMOTION_TYPE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum::PROMOTIONTYPEFREETRIAL => "PROMOTION_TYPE_FREE_TRIAL",
            GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum::PROMOTIONTYPEINTRODUCTORYPRICING => "PROMOTION_TYPE_INTRODUCTORY_PRICING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROMOTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum::PROMOTIONTYPEUNSPECIFIED),
           "PROMOTION_TYPE_FREE_TRIAL" => Ok(GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum::PROMOTIONTYPEFREETRIAL),
           "PROMOTION_TYPE_INTRODUCTORY_PRICING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum::PROMOTIONTYPEINTRODUCTORYPRICING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle).
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum {
    

    /// The processing state is unspecified.
    ///
    /// "PROCESSING_STATE_UNSPECIFIED"
    #[serde(rename="PROCESSING_STATE_UNSPECIFIED")]
    PROCESSINGSTATEUNSPECIFIED,
    

    /// The subscription is being cancelled.
    ///
    /// "PROCESSING_STATE_CANCELLING"
    #[serde(rename="PROCESSING_STATE_CANCELLING")]
    PROCESSINGSTATECANCELLING,
    

    /// The subscription is recurring.
    ///
    /// "PROCESSING_STATE_RECURRING"
    #[serde(rename="PROCESSING_STATE_RECURRING")]
    PROCESSINGSTATERECURRING,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum::PROCESSINGSTATEUNSPECIFIED => "PROCESSING_STATE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum::PROCESSINGSTATECANCELLING => "PROCESSING_STATE_CANCELLING",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum::PROCESSINGSTATERECURRING => "PROCESSING_STATE_RECURRING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROCESSING_STATE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum::PROCESSINGSTATEUNSPECIFIED),
           "PROCESSING_STATE_CANCELLING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum::PROCESSINGSTATECANCELLING),
           "PROCESSING_STATE_RECURRING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum::PROCESSINGSTATERECURRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle).
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum {
    

    /// The state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The subscription is created, a state before it is moved to STATE_ACTIVE.
    ///
    /// "STATE_CREATED"
    #[serde(rename="STATE_CREATED")]
    STATECREATED,
    

    /// The subscription is active.
    ///
    /// "STATE_ACTIVE"
    #[serde(rename="STATE_ACTIVE")]
    STATEACTIVE,
    

    /// The subscription is cancelled. This is the final state of the subscription, as it can no longer be modified or reactivated.
    ///
    /// "STATE_CANCELLED"
    #[serde(rename="STATE_CANCELLED")]
    STATECANCELLED,
    

    /// The subscription is in grace period. It can happen: 1) in manual extend mode, the subscription is not extended by the partner at the end of current cycle. 2) for outbound authorization enabled partners, a renewal purchase order is rejected.
    ///
    /// "STATE_IN_GRACE_PERIOD"
    #[serde(rename="STATE_IN_GRACE_PERIOD")]
    STATEINGRACEPERIOD,
    

    /// The subscription is waiting to be cancelled by the next recurrence cycle.
    ///
    /// "STATE_CANCEL_AT_END_OF_CYCLE"
    #[serde(rename="STATE_CANCEL_AT_END_OF_CYCLE")]
    STATECANCELATENDOFCYCLE,
    

    /// The subscription is suspended.
    ///
    /// "STATE_SUSPENDED"
    #[serde(rename="STATE_SUSPENDED")]
    STATESUSPENDED,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATECREATED => "STATE_CREATED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATEACTIVE => "STATE_ACTIVE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATECANCELLED => "STATE_CANCELLED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATEINGRACEPERIOD => "STATE_IN_GRACE_PERIOD",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATECANCELATENDOFCYCLE => "STATE_CANCEL_AT_END_OF_CYCLE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATESUSPENDED => "STATE_SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATEUNSPECIFIED),
           "STATE_CREATED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATECREATED),
           "STATE_ACTIVE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATEACTIVE),
           "STATE_CANCELLED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATECANCELLED),
           "STATE_IN_GRACE_PERIOD" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATEINGRACEPERIOD),
           "STATE_CANCEL_AT_END_OF_CYCLE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATECANCELATENDOFCYCLE),
           "STATE_SUSPENDED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum::STATESUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The reason of the cancellation.
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum {
    

    /// Reason is unspecified.
    ///
    /// "CANCELLATION_REASON_UNSPECIFIED"
    #[serde(rename="CANCELLATION_REASON_UNSPECIFIED")]
    CANCELLATIONREASONUNSPECIFIED,
    

    /// Fraudualant transaction.
    ///
    /// "CANCELLATION_REASON_FRAUD"
    #[serde(rename="CANCELLATION_REASON_FRAUD")]
    CANCELLATIONREASONFRAUD,
    

    /// Buyer's remorse.
    ///
    /// "CANCELLATION_REASON_REMORSE"
    #[serde(rename="CANCELLATION_REASON_REMORSE")]
    CANCELLATIONREASONREMORSE,
    

    /// Accidential purchase.
    ///
    /// "CANCELLATION_REASON_ACCIDENTAL_PURCHASE"
    #[serde(rename="CANCELLATION_REASON_ACCIDENTAL_PURCHASE")]
    CANCELLATIONREASONACCIDENTALPURCHASE,
    

    /// Payment is past due.
    ///
    /// "CANCELLATION_REASON_PAST_DUE"
    #[serde(rename="CANCELLATION_REASON_PAST_DUE")]
    CANCELLATIONREASONPASTDUE,
    

    /// Used for notification only, do not use in Cancel API. User account closed.
    ///
    /// "CANCELLATION_REASON_ACCOUNT_CLOSED"
    #[serde(rename="CANCELLATION_REASON_ACCOUNT_CLOSED")]
    CANCELLATIONREASONACCOUNTCLOSED,
    

    /// Used for notification only, do not use in Cancel API. Cancellation due to upgrade or downgrade.
    ///
    /// "CANCELLATION_REASON_UPGRADE_DOWNGRADE"
    #[serde(rename="CANCELLATION_REASON_UPGRADE_DOWNGRADE")]
    CANCELLATIONREASONUPGRADEDOWNGRADE,
    

    /// Cancellation due to user delinquency
    ///
    /// "CANCELLATION_REASON_USER_DELINQUENCY"
    #[serde(rename="CANCELLATION_REASON_USER_DELINQUENCY")]
    CANCELLATIONREASONUSERDELINQUENCY,
    

    /// Used for notification only, do not use in Cancel API. Cancellation due to an unrecoverable system error.
    ///
    /// "CANCELLATION_REASON_SYSTEM_ERROR"
    #[serde(rename="CANCELLATION_REASON_SYSTEM_ERROR")]
    CANCELLATIONREASONSYSTEMERROR,
    

    /// Used for notification only, do not use in Cancel API. The subscription is cancelled by Google automatically since it is no longer valid.
    ///
    /// "CANCELLATION_REASON_SYSTEM_CANCEL"
    #[serde(rename="CANCELLATION_REASON_SYSTEM_CANCEL")]
    CANCELLATIONREASONSYSTEMCANCEL,
    

    /// Other reason.
    ///
    /// "CANCELLATION_REASON_OTHER"
    #[serde(rename="CANCELLATION_REASON_OTHER")]
    CANCELLATIONREASONOTHER,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONUNSPECIFIED => "CANCELLATION_REASON_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONFRAUD => "CANCELLATION_REASON_FRAUD",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONREMORSE => "CANCELLATION_REASON_REMORSE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONACCIDENTALPURCHASE => "CANCELLATION_REASON_ACCIDENTAL_PURCHASE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONPASTDUE => "CANCELLATION_REASON_PAST_DUE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONACCOUNTCLOSED => "CANCELLATION_REASON_ACCOUNT_CLOSED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONUPGRADEDOWNGRADE => "CANCELLATION_REASON_UPGRADE_DOWNGRADE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONUSERDELINQUENCY => "CANCELLATION_REASON_USER_DELINQUENCY",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONSYSTEMERROR => "CANCELLATION_REASON_SYSTEM_ERROR",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONSYSTEMCANCEL => "CANCELLATION_REASON_SYSTEM_CANCEL",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONOTHER => "CANCELLATION_REASON_OTHER",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CANCELLATION_REASON_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONUNSPECIFIED),
           "CANCELLATION_REASON_FRAUD" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONFRAUD),
           "CANCELLATION_REASON_REMORSE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONREMORSE),
           "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONACCIDENTALPURCHASE),
           "CANCELLATION_REASON_PAST_DUE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONPASTDUE),
           "CANCELLATION_REASON_ACCOUNT_CLOSED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONACCOUNTCLOSED),
           "CANCELLATION_REASON_UPGRADE_DOWNGRADE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONUPGRADEDOWNGRADE),
           "CANCELLATION_REASON_USER_DELINQUENCY" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONUSERDELINQUENCY),
           "CANCELLATION_REASON_SYSTEM_ERROR" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONSYSTEMERROR),
           "CANCELLATION_REASON_SYSTEM_CANCEL" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONSYSTEMCANCEL),
           "CANCELLATION_REASON_OTHER" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum::CANCELLATIONREASONOTHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The recurrence type of the line item.
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum {
    

    /// The line item recurrence type is unspecified.
    ///
    /// "LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED")]
    LINEITEMRECURRENCETYPEUNSPECIFIED,
    

    /// The line item recurs periodically.
    ///
    /// "LINE_ITEM_RECURRENCE_TYPE_PERIODIC"
    #[serde(rename="LINE_ITEM_RECURRENCE_TYPE_PERIODIC")]
    LINEITEMRECURRENCETYPEPERIODIC,
    

    /// The line item does not recur in the future.
    ///
    /// "LINE_ITEM_RECURRENCE_TYPE_ONE_TIME"
    #[serde(rename="LINE_ITEM_RECURRENCE_TYPE_ONE_TIME")]
    LINEITEMRECURRENCETYPEONETIME,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum::LINEITEMRECURRENCETYPEUNSPECIFIED => "LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum::LINEITEMRECURRENCETYPEPERIODIC => "LINE_ITEM_RECURRENCE_TYPE_PERIODIC",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum::LINEITEMRECURRENCETYPEONETIME => "LINE_ITEM_RECURRENCE_TYPE_ONE_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum::LINEITEMRECURRENCETYPEUNSPECIFIED),
           "LINE_ITEM_RECURRENCE_TYPE_PERIODIC" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum::LINEITEMRECURRENCETYPEPERIODIC),
           "LINE_ITEM_RECURRENCE_TYPE_ONE_TIME" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum::LINEITEMRECURRENCETYPEONETIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the line item.
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum {
    

    /// Unspecified state.
    ///
    /// "LINE_ITEM_STATE_UNSPECIFIED"
    #[serde(rename="LINE_ITEM_STATE_UNSPECIFIED")]
    LINEITEMSTATEUNSPECIFIED,
    

    /// The line item is in ACTIVE state. If the subscription is cancelled or suspended, the line item will not be charged even if the line item is active.
    ///
    /// "LINE_ITEM_STATE_ACTIVE"
    #[serde(rename="LINE_ITEM_STATE_ACTIVE")]
    LINEITEMSTATEACTIVE,
    

    /// The line item is in INACTIVE state.
    ///
    /// "LINE_ITEM_STATE_INACTIVE"
    #[serde(rename="LINE_ITEM_STATE_INACTIVE")]
    LINEITEMSTATEINACTIVE,
    

    /// The line item is new, and is not activated or charged yet.
    ///
    /// "LINE_ITEM_STATE_NEW"
    #[serde(rename="LINE_ITEM_STATE_NEW")]
    LINEITEMSTATENEW,
    

    /// The line item is being activated in order to be charged. If a free trial applies to the line item, the line item is pending a prorated charge at the end of the free trial period, as indicated by `line_item_free_trial_end_time`.
    ///
    /// "LINE_ITEM_STATE_ACTIVATING"
    #[serde(rename="LINE_ITEM_STATE_ACTIVATING")]
    LINEITEMSTATEACTIVATING,
    

    /// The line item is being deactivated, and a prorated refund in being processed.
    ///
    /// "LINE_ITEM_STATE_DEACTIVATING"
    #[serde(rename="LINE_ITEM_STATE_DEACTIVATING")]
    LINEITEMSTATEDEACTIVATING,
    

    /// The line item is scheduled to be deactivated at the end of the current cycle.
    ///
    /// "LINE_ITEM_STATE_WAITING_TO_DEACTIVATE"
    #[serde(rename="LINE_ITEM_STATE_WAITING_TO_DEACTIVATE")]
    LINEITEMSTATEWAITINGTODEACTIVATE,
    

    /// Line item is being charged off-cycle.
    ///
    /// "LINE_ITEM_STATE_OFF_CYCLE_CHARGING"
    #[serde(rename="LINE_ITEM_STATE_OFF_CYCLE_CHARGING")]
    LINEITEMSTATEOFFCYCLECHARGING,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEUNSPECIFIED => "LINE_ITEM_STATE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEACTIVE => "LINE_ITEM_STATE_ACTIVE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEINACTIVE => "LINE_ITEM_STATE_INACTIVE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATENEW => "LINE_ITEM_STATE_NEW",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEACTIVATING => "LINE_ITEM_STATE_ACTIVATING",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEDEACTIVATING => "LINE_ITEM_STATE_DEACTIVATING",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEWAITINGTODEACTIVATE => "LINE_ITEM_STATE_WAITING_TO_DEACTIVATE",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEOFFCYCLECHARGING => "LINE_ITEM_STATE_OFF_CYCLE_CHARGING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINE_ITEM_STATE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEUNSPECIFIED),
           "LINE_ITEM_STATE_ACTIVE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEACTIVE),
           "LINE_ITEM_STATE_INACTIVE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEINACTIVE),
           "LINE_ITEM_STATE_NEW" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATENEW),
           "LINE_ITEM_STATE_ACTIVATING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEACTIVATING),
           "LINE_ITEM_STATE_DEACTIVATING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEDEACTIVATING),
           "LINE_ITEM_STATE_WAITING_TO_DEACTIVATE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEWAITINGTODEACTIVATE),
           "LINE_ITEM_STATE_OFF_CYCLE_CHARGING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum::LINEITEMSTATEOFFCYCLECHARGING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the promotion for the spec.
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum {
    

    /// The promotion type is unspecified.
    ///
    /// "PROMOTION_TYPE_UNSPECIFIED"
    #[serde(rename="PROMOTION_TYPE_UNSPECIFIED")]
    PROMOTIONTYPEUNSPECIFIED,
    

    /// The promotion is a free trial.
    ///
    /// "PROMOTION_TYPE_FREE_TRIAL"
    #[serde(rename="PROMOTION_TYPE_FREE_TRIAL")]
    PROMOTIONTYPEFREETRIAL,
    

    /// The promotion is a reduced introductory pricing.
    ///
    /// "PROMOTION_TYPE_INTRODUCTORY_PRICING"
    #[serde(rename="PROMOTION_TYPE_INTRODUCTORY_PRICING")]
    PROMOTIONTYPEINTRODUCTORYPRICING,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum::PROMOTIONTYPEUNSPECIFIED => "PROMOTION_TYPE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum::PROMOTIONTYPEFREETRIAL => "PROMOTION_TYPE_FREE_TRIAL",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum::PROMOTIONTYPEINTRODUCTORYPRICING => "PROMOTION_TYPE_INTRODUCTORY_PRICING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROMOTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum::PROMOTIONTYPEUNSPECIFIED),
           "PROMOTION_TYPE_FREE_TRIAL" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum::PROMOTIONTYPEFREETRIAL),
           "PROMOTION_TYPE_INTRODUCTORY_PRICING" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum::PROMOTIONTYPEINTRODUCTORYPRICING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies the billing cycle spec for the new upgraded/downgraded subscription.
pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum {
    

    /// Billing cycle spec is not specified.
    ///
    /// "BILLING_CYCLE_SPEC_UNSPECIFIED"
    #[serde(rename="BILLING_CYCLE_SPEC_UNSPECIFIED")]
    BILLINGCYCLESPECUNSPECIFIED,
    

    /// The billing cycle of the new subscription aligns with the previous subscription it upgrades or downgrades from.
    ///
    /// "BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION"
    #[serde(rename="BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION")]
    BILLINGCYCLESPECALIGNWITHPREVIOUSSUBSCRIPTION,
    

    /// The billing cycle of the new subscription starts immediately.
    ///
    /// "BILLING_CYCLE_SPEC_START_IMMEDIATELY"
    #[serde(rename="BILLING_CYCLE_SPEC_START_IMMEDIATELY")]
    BILLINGCYCLESPECSTARTIMMEDIATELY,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum::BILLINGCYCLESPECUNSPECIFIED => "BILLING_CYCLE_SPEC_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum::BILLINGCYCLESPECALIGNWITHPREVIOUSSUBSCRIPTION => "BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION",
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum::BILLINGCYCLESPECSTARTIMMEDIATELY => "BILLING_CYCLE_SPEC_START_IMMEDIATELY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BILLING_CYCLE_SPEC_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum::BILLINGCYCLESPECUNSPECIFIED),
           "BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum::BILLINGCYCLESPECALIGNWITHPREVIOUSSUBSCRIPTION),
           "BILLING_CYCLE_SPEC_START_IMMEDIATELY" => Ok(GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum::BILLINGCYCLESPECSTARTIMMEDIATELY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailBillingCycleSpecEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the plan type offered to the end user by the partner.
pub enum GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum {
    

    /// Unspecified. Should not use, reserved as an invalid value.
    ///
    /// "PARTNER_PLAN_TYPE_UNSPECIFIED"
    #[serde(rename="PARTNER_PLAN_TYPE_UNSPECIFIED")]
    PARTNERPLANTYPEUNSPECIFIED,
    

    /// This item is offered as a standalone product to the user.
    ///
    /// "PARTNER_PLAN_TYPE_STANDALONE"
    #[serde(rename="PARTNER_PLAN_TYPE_STANDALONE")]
    PARTNERPLANTYPESTANDALONE,
    

    /// This item is bundled with another partner offering, the item is provisioned at purchase time.
    ///
    /// "PARTNER_PLAN_TYPE_HARD_BUNDLE"
    #[serde(rename="PARTNER_PLAN_TYPE_HARD_BUNDLE")]
    PARTNERPLANTYPEHARDBUNDLE,
    

    /// This item is bundled with another partner offering, the item is provisioned after puchase, when the user opts in this Google service.
    ///
    /// "PARTNER_PLAN_TYPE_SOFT_BUNDLE"
    #[serde(rename="PARTNER_PLAN_TYPE_SOFT_BUNDLE")]
    PARTNERPLANTYPESOFTBUNDLE,
}

impl AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPEUNSPECIFIED => "PARTNER_PLAN_TYPE_UNSPECIFIED",
            GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPESTANDALONE => "PARTNER_PLAN_TYPE_STANDALONE",
            GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPEHARDBUNDLE => "PARTNER_PLAN_TYPE_HARD_BUNDLE",
            GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPESOFTBUNDLE => "PARTNER_PLAN_TYPE_SOFT_BUNDLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTNER_PLAN_TYPE_UNSPECIFIED" => Ok(GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPEUNSPECIFIED),
           "PARTNER_PLAN_TYPE_STANDALONE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPESTANDALONE),
           "PARTNER_PLAN_TYPE_HARD_BUNDLE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPEHARDBUNDLE),
           "PARTNER_PLAN_TYPE_SOFT_BUNDLE" => Ok(GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum::PARTNERPLANTYPESOFTBUNDLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudPaymentsResellerSubscriptionV1YoutubePayloadPartnerPlanTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductBundleDetailEntitlementModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The entitlement mode of the bundle product.
pub enum ProductBundleDetailEntitlementModeEnum {
    

    /// Unspecified. It's reserved as an unexpected value, should not be used.
    ///
    /// "ENTITLEMENT_MODE_UNSPECIFIED"
    #[serde(rename="ENTITLEMENT_MODE_UNSPECIFIED")]
    ENTITLEMENTMODEUNSPECIFIED,
    

    /// All the bundle elements must be fully activated in a single request.
    ///
    /// "ENTITLEMENT_MODE_FULL"
    #[serde(rename="ENTITLEMENT_MODE_FULL")]
    ENTITLEMENTMODEFULL,
    

    /// The bundle elements could be incrementally activated.
    ///
    /// "ENTITLEMENT_MODE_INCREMENTAL"
    #[serde(rename="ENTITLEMENT_MODE_INCREMENTAL")]
    ENTITLEMENTMODEINCREMENTAL,
}

impl AsRef<str> for ProductBundleDetailEntitlementModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductBundleDetailEntitlementModeEnum::ENTITLEMENTMODEUNSPECIFIED => "ENTITLEMENT_MODE_UNSPECIFIED",
            ProductBundleDetailEntitlementModeEnum::ENTITLEMENTMODEFULL => "ENTITLEMENT_MODE_FULL",
            ProductBundleDetailEntitlementModeEnum::ENTITLEMENTMODEINCREMENTAL => "ENTITLEMENT_MODE_INCREMENTAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductBundleDetailEntitlementModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITLEMENT_MODE_UNSPECIFIED" => Ok(ProductBundleDetailEntitlementModeEnum::ENTITLEMENTMODEUNSPECIFIED),
           "ENTITLEMENT_MODE_FULL" => Ok(ProductBundleDetailEntitlementModeEnum::ENTITLEMENTMODEFULL),
           "ENTITLEMENT_MODE_INCREMENTAL" => Ok(ProductBundleDetailEntitlementModeEnum::ENTITLEMENTMODEINCREMENTAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductBundleDetailEntitlementModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


