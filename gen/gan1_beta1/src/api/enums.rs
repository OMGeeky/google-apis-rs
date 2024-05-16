use super::*;



// region AdvertiserRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the requester. Valid values: 'advertisers' or 'publishers'.
pub enum AdvertiserRoleEnum {
    

    /// The requester is requesting as an advertiser.
    ///
    /// "advertisers"
    #[serde(rename="advertisers")]
    Advertisers,
    

    /// The requester is requesting as a publisher.
    ///
    /// "publishers"
    #[serde(rename="publishers")]
    Publishers,
}

impl AsRef<str> for AdvertiserRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserRoleEnum::Advertisers => "advertisers",
            AdvertiserRoleEnum::Publishers => "publishers",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "advertisers" => Ok(AdvertiserRoleEnum::Advertisers),
           "publishers" => Ok(AdvertiserRoleEnum::Publishers),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AdvertiserRelationshipStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all advertisers for which do not have the given relationship status with the requesting publisher.
pub enum AdvertiserRelationshipStatusEnum {
    

    /// An advertiser that has approved your application.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    

    /// An advertiser program that's accepting new publishers.
    ///
    /// "available"
    #[serde(rename="available")]
    Available,
    

    /// Deactivated means either the advertiser has removed you from their program, or it could also mean that you chose to remove yourself from the advertiser's program.
    ///
    /// "deactivated"
    #[serde(rename="deactivated")]
    Deactivated,
    

    /// An advertiser that did not approve your application.
    ///
    /// "declined"
    #[serde(rename="declined")]
    Declined,
    

    /// An advertiser program that you've already applied to, but they haven't yet decided to approve or decline your application.
    ///
    /// "pending"
    #[serde(rename="pending")]
    Pending,
}

impl AsRef<str> for AdvertiserRelationshipStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvertiserRelationshipStatusEnum::Approved => "approved",
            AdvertiserRelationshipStatusEnum::Available => "available",
            AdvertiserRelationshipStatusEnum::Deactivated => "deactivated",
            AdvertiserRelationshipStatusEnum::Declined => "declined",
            AdvertiserRelationshipStatusEnum::Pending => "pending",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvertiserRelationshipStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(AdvertiserRelationshipStatusEnum::Approved),
           "available" => Ok(AdvertiserRelationshipStatusEnum::Available),
           "deactivated" => Ok(AdvertiserRelationshipStatusEnum::Deactivated),
           "declined" => Ok(AdvertiserRelationshipStatusEnum::Declined),
           "pending" => Ok(AdvertiserRelationshipStatusEnum::Pending),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvertiserRelationshipStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CcOfferProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The set of fields to return.
pub enum CcOfferProjectionEnum {
    

    /// Include all offer fields. This is the default.
    ///
    /// "full"
    #[serde(rename="full")]
    Full,
    

    /// Include only the basic fields needed to display an offer.
    ///
    /// "summary"
    #[serde(rename="summary")]
    Summary,
}

impl AsRef<str> for CcOfferProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CcOfferProjectionEnum::Full => "full",
            CcOfferProjectionEnum::Summary => "summary",
        }
    }
}

impl std::convert::TryFrom< &str> for CcOfferProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "full" => Ok(CcOfferProjectionEnum::Full),
           "summary" => Ok(CcOfferProjectionEnum::Summary),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CcOfferProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventChargeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all charge events that are not of the given charge type. Valid values: 'other', 'slotting_fee', 'monthly_minimum', 'tier_bonus', 'credit', 'debit'. Optional.
pub enum EventChargeTypeEnum {
    

    /// A credit increases the publisher's payout amount and decreases the advertiser's invoice amount.
    ///
    /// "credit"
    #[serde(rename="credit")]
    Credit,
    

    /// A debit reduces the publisher's payout and increases the advertiser's invoice amount.
    ///
    /// "debit"
    #[serde(rename="debit")]
    Debit,
    

    /// A payment made to Google by an advertiser as a minimum monthly network fee.
    ///
    /// "monthly_minimum"
    #[serde(rename="monthly_minimum")]
    MonthlyMinimum,
    

    /// Catch all. Default if unset
    ///
    /// "other"
    #[serde(rename="other")]
    Other,
    

    /// A one time payment made from an advertiser to a publisher.
    ///
    /// "slotting_fee"
    #[serde(rename="slotting_fee")]
    SlottingFee,
    

    /// A payment from an advertiser to a publisher for the publisher maintaining a high tier level
    ///
    /// "tier_bonus"
    #[serde(rename="tier_bonus")]
    TierBonus,
}

impl AsRef<str> for EventChargeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventChargeTypeEnum::Credit => "credit",
            EventChargeTypeEnum::Debit => "debit",
            EventChargeTypeEnum::MonthlyMinimum => "monthly_minimum",
            EventChargeTypeEnum::Other => "other",
            EventChargeTypeEnum::SlottingFee => "slotting_fee",
            EventChargeTypeEnum::TierBonus => "tier_bonus",
        }
    }
}

impl std::convert::TryFrom< &str> for EventChargeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "credit" => Ok(EventChargeTypeEnum::Credit),
           "debit" => Ok(EventChargeTypeEnum::Debit),
           "monthly_minimum" => Ok(EventChargeTypeEnum::MonthlyMinimum),
           "other" => Ok(EventChargeTypeEnum::Other),
           "slotting_fee" => Ok(EventChargeTypeEnum::SlottingFee),
           "tier_bonus" => Ok(EventChargeTypeEnum::TierBonus),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventChargeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the requester. Valid values: 'advertisers' or 'publishers'.
pub enum EventRoleEnum {
    

    /// The requester is requesting as an advertiser.
    ///
    /// "advertisers"
    #[serde(rename="advertisers")]
    Advertisers,
    

    /// The requester is requesting as a publisher.
    ///
    /// "publishers"
    #[serde(rename="publishers")]
    Publishers,
}

impl AsRef<str> for EventRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventRoleEnum::Advertisers => "advertisers",
            EventRoleEnum::Publishers => "publishers",
        }
    }
}

impl std::convert::TryFrom< &str> for EventRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "advertisers" => Ok(EventRoleEnum::Advertisers),
           "publishers" => Ok(EventRoleEnum::Publishers),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all events that do not have the given status. Valid values: 'active', 'canceled'. Optional.
pub enum EventStatusEnum {
    

    /// Event is currently active.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    

    /// Event is currently canceled.
    ///
    /// "canceled"
    #[serde(rename="canceled")]
    Canceled,
}

impl AsRef<str> for EventStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventStatusEnum::Active => "active",
            EventStatusEnum::Canceled => "canceled",
        }
    }
}

impl std::convert::TryFrom< &str> for EventStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "active" => Ok(EventStatusEnum::Active),
           "canceled" => Ok(EventStatusEnum::Canceled),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all events that are not of the given type. Valid values: 'action', 'transaction', 'charge'. Optional.
pub enum EventTypeEnum {
    

    /// The completion of an application, sign-up, or other process. For example, an action occurs if a user clicks an ad for a credit card and completes an application for that card.
    ///
    /// "action"
    #[serde(rename="action")]
    Action,
    

    /// A charge event is typically a payment between an advertiser, publisher or Google.
    ///
    /// "charge"
    #[serde(rename="charge")]
    Charge,
    

    /// A conversion event, typically an e-commerce transaction. Some advertisers use a transaction to record other types of events, such as magazine subscriptions.
    ///
    /// "transaction"
    #[serde(rename="transaction")]
    Transaction,
}

impl AsRef<str> for EventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTypeEnum::Action => "action",
            EventTypeEnum::Charge => "charge",
            EventTypeEnum::Transaction => "transaction",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "action" => Ok(EventTypeEnum::Action),
           "charge" => Ok(EventTypeEnum::Charge),
           "transaction" => Ok(EventTypeEnum::Transaction),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the requester. Valid values: 'advertisers' or 'publishers'.
pub enum LinkRoleEnum {
    

    /// The requester is requesting as an advertiser.
    ///
    /// "advertisers"
    #[serde(rename="advertisers")]
    Advertisers,
    

    /// The requester is requesting as a publisher.
    ///
    /// "publishers"
    #[serde(rename="publishers")]
    Publishers,
}

impl AsRef<str> for LinkRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkRoleEnum::Advertisers => "advertisers",
            LinkRoleEnum::Publishers => "publishers",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "advertisers" => Ok(LinkRoleEnum::Advertisers),
           "publishers" => Ok(LinkRoleEnum::Publishers),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkAuthorshipEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the author of the link.
pub enum LinkAuthorshipEnum {
    
    /// "advertiser"
    #[serde(rename="advertiser")]
    Advertiser,
    
    /// "publisher"
    #[serde(rename="publisher")]
    Publisher,
}

impl AsRef<str> for LinkAuthorshipEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkAuthorshipEnum::Advertiser => "advertiser",
            LinkAuthorshipEnum::Publisher => "publisher",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkAuthorshipEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "advertiser" => Ok(LinkAuthorshipEnum::Advertiser),
           "publisher" => Ok(LinkAuthorshipEnum::Publisher),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkAuthorshipEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkLinkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the link.
pub enum LinkLinkTypeEnum {
    
    /// "banner"
    #[serde(rename="banner")]
    Banner,
    
    /// "text"
    #[serde(rename="text")]
    Text,
}

impl AsRef<str> for LinkLinkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkLinkTypeEnum::Banner => "banner",
            LinkLinkTypeEnum::Text => "text",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkLinkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "banner" => Ok(LinkLinkTypeEnum::Banner),
           "text" => Ok(LinkLinkTypeEnum::Text),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkLinkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkPromotionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The promotion type.
pub enum LinkPromotionTypeEnum {
    
    /// "coupon"
    #[serde(rename="coupon")]
    Coupon,
    
    /// "free_gift"
    #[serde(rename="free_gift")]
    FreeGift,
    
    /// "free_shipping"
    #[serde(rename="free_shipping")]
    FreeShipping,
    
    /// "percent_off"
    #[serde(rename="percent_off")]
    PercentOff,
    
    /// "price_cut"
    #[serde(rename="price_cut")]
    PriceCut,
}

impl AsRef<str> for LinkPromotionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkPromotionTypeEnum::Coupon => "coupon",
            LinkPromotionTypeEnum::FreeGift => "free_gift",
            LinkPromotionTypeEnum::FreeShipping => "free_shipping",
            LinkPromotionTypeEnum::PercentOff => "percent_off",
            LinkPromotionTypeEnum::PriceCut => "price_cut",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkPromotionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "coupon" => Ok(LinkPromotionTypeEnum::Coupon),
           "free_gift" => Ok(LinkPromotionTypeEnum::FreeGift),
           "free_shipping" => Ok(LinkPromotionTypeEnum::FreeShipping),
           "percent_off" => Ok(LinkPromotionTypeEnum::PercentOff),
           "price_cut" => Ok(LinkPromotionTypeEnum::PriceCut),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkPromotionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkRelationshipStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the relationship.
pub enum LinkRelationshipStatusEnum {
    
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    
    /// "available"
    #[serde(rename="available")]
    Available,
}

impl AsRef<str> for LinkRelationshipStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkRelationshipStatusEnum::Approved => "approved",
            LinkRelationshipStatusEnum::Available => "available",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkRelationshipStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(LinkRelationshipStatusEnum::Approved),
           "available" => Ok(LinkRelationshipStatusEnum::Available),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkRelationshipStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublisherRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the requester. Valid values: 'advertisers' or 'publishers'.
pub enum PublisherRoleEnum {
    

    /// The requester is requesting as an advertiser.
    ///
    /// "advertisers"
    #[serde(rename="advertisers")]
    Advertisers,
    

    /// The requester is requesting as a publisher.
    ///
    /// "publishers"
    #[serde(rename="publishers")]
    Publishers,
}

impl AsRef<str> for PublisherRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublisherRoleEnum::Advertisers => "advertisers",
            PublisherRoleEnum::Publishers => "publishers",
        }
    }
}

impl std::convert::TryFrom< &str> for PublisherRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "advertisers" => Ok(PublisherRoleEnum::Advertisers),
           "publishers" => Ok(PublisherRoleEnum::Publishers),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublisherRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublisherRelationshipStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all publishers for which do not have the given relationship status with the requesting publisher.
pub enum PublisherRelationshipStatusEnum {
    

    /// Publishers you've approved to your program.
    ///
    /// "approved"
    #[serde(rename="approved")]
    Approved,
    

    /// Publishers available for you to recruit.
    ///
    /// "available"
    #[serde(rename="available")]
    Available,
    

    /// A publisher that you terminated from your program. Publishers also have the ability to remove themselves from your program.
    ///
    /// "deactivated"
    #[serde(rename="deactivated")]
    Deactivated,
    

    /// A publisher that you did not approve to your program.
    ///
    /// "declined"
    #[serde(rename="declined")]
    Declined,
    

    /// Publishers that have applied to your program. We recommend reviewing and deciding on pending publishers on a weekly basis.
    ///
    /// "pending"
    #[serde(rename="pending")]
    Pending,
}

impl AsRef<str> for PublisherRelationshipStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublisherRelationshipStatusEnum::Approved => "approved",
            PublisherRelationshipStatusEnum::Available => "available",
            PublisherRelationshipStatusEnum::Deactivated => "deactivated",
            PublisherRelationshipStatusEnum::Declined => "declined",
            PublisherRelationshipStatusEnum::Pending => "pending",
        }
    }
}

impl std::convert::TryFrom< &str> for PublisherRelationshipStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "approved" => Ok(PublisherRelationshipStatusEnum::Approved),
           "available" => Ok(PublisherRelationshipStatusEnum::Available),
           "deactivated" => Ok(PublisherRelationshipStatusEnum::Deactivated),
           "declined" => Ok(PublisherRelationshipStatusEnum::Declined),
           "pending" => Ok(PublisherRelationshipStatusEnum::Pending),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublisherRelationshipStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all events that are not of the given type. Valid values: 'action', 'transaction', or 'charge'. Optional.
pub enum ReportEventTypeEnum {
    

    /// Event type is action.
    ///
    /// "action"
    #[serde(rename="action")]
    Action,
    

    /// Event type is charge.
    ///
    /// "charge"
    #[serde(rename="charge")]
    Charge,
    

    /// Event type is transaction.
    ///
    /// "transaction"
    #[serde(rename="transaction")]
    Transaction,
}

impl AsRef<str> for ReportEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportEventTypeEnum::Action => "action",
            ReportEventTypeEnum::Charge => "charge",
            ReportEventTypeEnum::Transaction => "transaction",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "action" => Ok(ReportEventTypeEnum::Action),
           "charge" => Ok(ReportEventTypeEnum::Charge),
           "transaction" => Ok(ReportEventTypeEnum::Transaction),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportReportTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of report being requested. Valid values: 'order_delta'. Required.
pub enum ReportReportTypeEnum {
    

    /// The order delta report type.
    ///
    /// "order_delta"
    #[serde(rename="order_delta")]
    OrderDelta,
}

impl AsRef<str> for ReportReportTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportReportTypeEnum::OrderDelta => "order_delta",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportReportTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "order_delta" => Ok(ReportReportTypeEnum::OrderDelta),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportReportTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the requester. Valid values: 'advertisers' or 'publishers'.
pub enum ReportRoleEnum {
    

    /// The requester is requesting as an advertiser.
    ///
    /// "advertisers"
    #[serde(rename="advertisers")]
    Advertisers,
    

    /// The requester is requesting as a publisher.
    ///
    /// "publishers"
    #[serde(rename="publishers")]
    Publishers,
}

impl AsRef<str> for ReportRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportRoleEnum::Advertisers => "advertisers",
            ReportRoleEnum::Publishers => "publishers",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "advertisers" => Ok(ReportRoleEnum::Advertisers),
           "publishers" => Ok(ReportRoleEnum::Publishers),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filters out all events that do not have the given status. Valid values: 'active', 'canceled', or 'invalid'. Optional.
pub enum ReportStatusEnum {
    

    /// Event is currently active.
    ///
    /// "active"
    #[serde(rename="active")]
    Active,
    

    /// Event is currently canceled.
    ///
    /// "canceled"
    #[serde(rename="canceled")]
    Canceled,
    

    /// Event is currently invalid.
    ///
    /// "invalid"
    #[serde(rename="invalid")]
    Invalid,
}

impl AsRef<str> for ReportStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportStatusEnum::Active => "active",
            ReportStatusEnum::Canceled => "canceled",
            ReportStatusEnum::Invalid => "invalid",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "active" => Ok(ReportStatusEnum::Active),
           "canceled" => Ok(ReportStatusEnum::Canceled),
           "invalid" => Ok(ReportStatusEnum::Invalid),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


