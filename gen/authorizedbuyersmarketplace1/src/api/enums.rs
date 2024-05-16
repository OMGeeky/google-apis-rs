use super::*;



// region AdSizeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the ad slot size.
pub enum AdSizeTypeEnum {
    

    /// A placeholder for an undefined size type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Ad slot with size specified by height and width in pixels.
    ///
    /// "PIXEL"
    #[serde(rename="PIXEL")]
    PIXEL,
    

    /// Special size to describe an interstitial ad slot.
    ///
    /// "INTERSTITIAL"
    #[serde(rename="INTERSTITIAL")]
    INTERSTITIAL,
    

    /// Native (mobile) ads rendered by the publisher.
    ///
    /// "NATIVE"
    #[serde(rename="NATIVE")]
    NATIVE,
    

    /// Fluid size (responsive size) can be resized automatically with the change of outside environment.
    ///
    /// "FLUID"
    #[serde(rename="FLUID")]
    FLUID,
}

impl AsRef<str> for AdSizeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdSizeTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AdSizeTypeEnum::PIXEL => "PIXEL",
            AdSizeTypeEnum::INTERSTITIAL => "INTERSTITIAL",
            AdSizeTypeEnum::NATIVE => "NATIVE",
            AdSizeTypeEnum::FLUID => "FLUID",
        }
    }
}

impl std::convert::TryFrom< &str> for AdSizeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AdSizeTypeEnum::TYPEUNSPECIFIED),
           "PIXEL" => Ok(AdSizeTypeEnum::PIXEL),
           "INTERSTITIAL" => Ok(AdSizeTypeEnum::INTERSTITIAL),
           "NATIVE" => Ok(AdSizeTypeEnum::NATIVE),
           "FLUID" => Ok(AdSizeTypeEnum::FLUID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdSizeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClientRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The role assigned to the client. Each role implies a set of permissions granted to the client.
pub enum ClientRoleEnum {
    

    /// A placeholder for an undefined client role. This value should never be specified in user input for create or update method, otherwise an error will be returned.
    ///
    /// "CLIENT_ROLE_UNSPECIFIED"
    #[serde(rename="CLIENT_ROLE_UNSPECIFIED")]
    CLIENTROLEUNSPECIFIED,
    

    /// Users associated with this client role can only view proposals and deals in the Marketplace UI. They cannot negotiate or approve proposals and deals sent from publishers or send RFP to publishers.
    ///
    /// "CLIENT_DEAL_VIEWER"
    #[serde(rename="CLIENT_DEAL_VIEWER")]
    CLIENTDEALVIEWER,
    

    /// Users associated with this client role can view and negotiate on the proposals and deals in the Marketplace UI sent from publishers and send RFP to publishers, but cannot approve the proposals and deals by themselves. The buyer can approve the proposals and deals on behalf of the client.
    ///
    /// "CLIENT_DEAL_NEGOTIATOR"
    #[serde(rename="CLIENT_DEAL_NEGOTIATOR")]
    CLIENTDEALNEGOTIATOR,
    

    /// Users associated with this client role can view, negotiate and approve proposals and deals in the Marketplace UI and send RFP to publishers.
    ///
    /// "CLIENT_DEAL_APPROVER"
    #[serde(rename="CLIENT_DEAL_APPROVER")]
    CLIENTDEALAPPROVER,
}

impl AsRef<str> for ClientRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClientRoleEnum::CLIENTROLEUNSPECIFIED => "CLIENT_ROLE_UNSPECIFIED",
            ClientRoleEnum::CLIENTDEALVIEWER => "CLIENT_DEAL_VIEWER",
            ClientRoleEnum::CLIENTDEALNEGOTIATOR => "CLIENT_DEAL_NEGOTIATOR",
            ClientRoleEnum::CLIENTDEALAPPROVER => "CLIENT_DEAL_APPROVER",
        }
    }
}

impl std::convert::TryFrom< &str> for ClientRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLIENT_ROLE_UNSPECIFIED" => Ok(ClientRoleEnum::CLIENTROLEUNSPECIFIED),
           "CLIENT_DEAL_VIEWER" => Ok(ClientRoleEnum::CLIENTDEALVIEWER),
           "CLIENT_DEAL_NEGOTIATOR" => Ok(ClientRoleEnum::CLIENTDEALNEGOTIATOR),
           "CLIENT_DEAL_APPROVER" => Ok(ClientRoleEnum::CLIENTDEALAPPROVER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClientRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClientStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the client.
pub enum ClientStateEnum {
    

    /// A placeholder for an undefined client state. Should not be used.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// A client that is currently active and allowed to access the Authorized Buyers UI.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// A client that is currently inactive and not allowed to access the Authorized Buyers UI.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for ClientStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClientStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ClientStateEnum::ACTIVE => "ACTIVE",
            ClientStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ClientStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ClientStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(ClientStateEnum::ACTIVE),
           "INACTIVE" => Ok(ClientStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClientStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClientUserStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the client user.
pub enum ClientUserStateEnum {
    

    /// A placeholder for an undefined user state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// A user who was created but hasn't accepted the invitation yet, not allowed to access the Authorized Buyers UI.
    ///
    /// "INVITED"
    #[serde(rename="INVITED")]
    INVITED,
    

    /// A user that is currently active and allowed to access the Authorized Buyers UI.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// A user that is currently inactive and not allowed to access the Authorized Buyers UI.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
}

impl AsRef<str> for ClientUserStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClientUserStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ClientUserStateEnum::INVITED => "INVITED",
            ClientUserStateEnum::ACTIVE => "ACTIVE",
            ClientUserStateEnum::INACTIVE => "INACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ClientUserStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ClientUserStateEnum::STATEUNSPECIFIED),
           "INVITED" => Ok(ClientUserStateEnum::INVITED),
           "ACTIVE" => Ok(ClientUserStateEnum::ACTIVE),
           "INACTIVE" => Ok(ClientUserStateEnum::INACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClientUserStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRequirementCreativeFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The format of the creative, only applicable for programmatic guaranteed and preferred deals.
pub enum CreativeRequirementCreativeFormatEnum {
    

    /// A placeholder for an unspecified creative format.
    ///
    /// "CREATIVE_FORMAT_UNSPECIFIED"
    #[serde(rename="CREATIVE_FORMAT_UNSPECIFIED")]
    CREATIVEFORMATUNSPECIFIED,
    

    /// Banner creatives such as image or HTML5 assets.
    ///
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    

    /// Video creatives that can be played in a video player.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// Audio creatives that can play during audio content or point to a third party ad server.
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
}

impl AsRef<str> for CreativeRequirementCreativeFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRequirementCreativeFormatEnum::CREATIVEFORMATUNSPECIFIED => "CREATIVE_FORMAT_UNSPECIFIED",
            CreativeRequirementCreativeFormatEnum::DISPLAY => "DISPLAY",
            CreativeRequirementCreativeFormatEnum::VIDEO => "VIDEO",
            CreativeRequirementCreativeFormatEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRequirementCreativeFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_FORMAT_UNSPECIFIED" => Ok(CreativeRequirementCreativeFormatEnum::CREATIVEFORMATUNSPECIFIED),
           "DISPLAY" => Ok(CreativeRequirementCreativeFormatEnum::DISPLAY),
           "VIDEO" => Ok(CreativeRequirementCreativeFormatEnum::VIDEO),
           "AUDIO" => Ok(CreativeRequirementCreativeFormatEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRequirementCreativeFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRequirementCreativePreApprovalPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the creative pre-approval policy.
pub enum CreativeRequirementCreativePreApprovalPolicyEnum {
    

    /// A placeholder for an undefined creative pre-approval policy.
    ///
    /// "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED"
    #[serde(rename="CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED")]
    CREATIVEPREAPPROVALPOLICYUNSPECIFIED,
    

    /// The seller needs to approve each creative before it can serve.
    ///
    /// "SELLER_PRE_APPROVAL_REQUIRED"
    #[serde(rename="SELLER_PRE_APPROVAL_REQUIRED")]
    SELLERPREAPPROVALREQUIRED,
    

    /// The seller does not need to approve each creative before it can serve.
    ///
    /// "SELLER_PRE_APPROVAL_NOT_REQUIRED"
    #[serde(rename="SELLER_PRE_APPROVAL_NOT_REQUIRED")]
    SELLERPREAPPROVALNOTREQUIRED,
}

impl AsRef<str> for CreativeRequirementCreativePreApprovalPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRequirementCreativePreApprovalPolicyEnum::CREATIVEPREAPPROVALPOLICYUNSPECIFIED => "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED",
            CreativeRequirementCreativePreApprovalPolicyEnum::SELLERPREAPPROVALREQUIRED => "SELLER_PRE_APPROVAL_REQUIRED",
            CreativeRequirementCreativePreApprovalPolicyEnum::SELLERPREAPPROVALNOTREQUIRED => "SELLER_PRE_APPROVAL_NOT_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRequirementCreativePreApprovalPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED" => Ok(CreativeRequirementCreativePreApprovalPolicyEnum::CREATIVEPREAPPROVALPOLICYUNSPECIFIED),
           "SELLER_PRE_APPROVAL_REQUIRED" => Ok(CreativeRequirementCreativePreApprovalPolicyEnum::SELLERPREAPPROVALREQUIRED),
           "SELLER_PRE_APPROVAL_NOT_REQUIRED" => Ok(CreativeRequirementCreativePreApprovalPolicyEnum::SELLERPREAPPROVALNOTREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRequirementCreativePreApprovalPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRequirementCreativeSafeFrameCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies whether the creative is safeFrame compatible.
pub enum CreativeRequirementCreativeSafeFrameCompatibilityEnum {
    

    /// A placeholder for an undefined creative safe-frame compatibility.
    ///
    /// "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED"
    #[serde(rename="CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED")]
    CREATIVESAFEFRAMECOMPATIBILITYUNSPECIFIED,
    

    /// The creatives need to be compatible with the safe frame option.
    ///
    /// "COMPATIBLE"
    #[serde(rename="COMPATIBLE")]
    COMPATIBLE,
    

    /// The creatives can be incompatible with the safe frame option.
    ///
    /// "INCOMPATIBLE"
    #[serde(rename="INCOMPATIBLE")]
    INCOMPATIBLE,
}

impl AsRef<str> for CreativeRequirementCreativeSafeFrameCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRequirementCreativeSafeFrameCompatibilityEnum::CREATIVESAFEFRAMECOMPATIBILITYUNSPECIFIED => "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED",
            CreativeRequirementCreativeSafeFrameCompatibilityEnum::COMPATIBLE => "COMPATIBLE",
            CreativeRequirementCreativeSafeFrameCompatibilityEnum::INCOMPATIBLE => "INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRequirementCreativeSafeFrameCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED" => Ok(CreativeRequirementCreativeSafeFrameCompatibilityEnum::CREATIVESAFEFRAMECOMPATIBILITYUNSPECIFIED),
           "COMPATIBLE" => Ok(CreativeRequirementCreativeSafeFrameCompatibilityEnum::COMPATIBLE),
           "INCOMPATIBLE" => Ok(CreativeRequirementCreativeSafeFrameCompatibilityEnum::INCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRequirementCreativeSafeFrameCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRequirementProgrammaticCreativeSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the creative source for programmatic deals. PUBLISHER means creative is provided by seller and ADVERTISER means creative is provided by the buyer.
pub enum CreativeRequirementProgrammaticCreativeSourceEnum {
    

    /// A placeholder for an undefined programmatic creative source.
    ///
    /// "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED"
    #[serde(rename="PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED")]
    PROGRAMMATICCREATIVESOURCEUNSPECIFIED,
    

    /// The advertiser provides the creatives.
    ///
    /// "ADVERTISER"
    #[serde(rename="ADVERTISER")]
    ADVERTISER,
    

    /// The publisher provides the creatives to be served.
    ///
    /// "PUBLISHER"
    #[serde(rename="PUBLISHER")]
    PUBLISHER,
}

impl AsRef<str> for CreativeRequirementProgrammaticCreativeSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRequirementProgrammaticCreativeSourceEnum::PROGRAMMATICCREATIVESOURCEUNSPECIFIED => "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED",
            CreativeRequirementProgrammaticCreativeSourceEnum::ADVERTISER => "ADVERTISER",
            CreativeRequirementProgrammaticCreativeSourceEnum::PUBLISHER => "PUBLISHER",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRequirementProgrammaticCreativeSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED" => Ok(CreativeRequirementProgrammaticCreativeSourceEnum::PROGRAMMATICCREATIVESOURCEUNSPECIFIED),
           "ADVERTISER" => Ok(CreativeRequirementProgrammaticCreativeSourceEnum::ADVERTISER),
           "PUBLISHER" => Ok(CreativeRequirementProgrammaticCreativeSourceEnum::PUBLISHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRequirementProgrammaticCreativeSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRequirementSkippableAdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Skippable video ads allow viewers to skip ads after 5 seconds. Only applicable for deals with video creatives.
pub enum CreativeRequirementSkippableAdTypeEnum {
    

    /// A placeholder for an unspecified skippable ad type.
    ///
    /// "SKIPPABLE_AD_TYPE_UNSPECIFIED"
    #[serde(rename="SKIPPABLE_AD_TYPE_UNSPECIFIED")]
    SKIPPABLEADTYPEUNSPECIFIED,
    

    /// Video ad that can be skipped after 5 seconds. This value will appear in RTB bid requests as SkippableBidRequestType::REQUIRE_SKIPPABLE.
    ///
    /// "SKIPPABLE"
    #[serde(rename="SKIPPABLE")]
    SKIPPABLE,
    

    /// Video ad that can be skipped after 5 seconds, and is counted as engaged view after 30 seconds. The creative is hosted on YouTube only, and viewcount of the YouTube video increments after the engaged view. This value will appear in RTB bid requests as SkippableBidRequestType::REQUIRE_SKIPPABLE.
    ///
    /// "INSTREAM_SELECT"
    #[serde(rename="INSTREAM_SELECT")]
    INSTREAMSELECT,
    

    /// This video ad is not skippable. This value will appear in RTB bid requests as SkippableBidRequestType::BLOCK_SKIPPABLE.
    ///
    /// "NOT_SKIPPABLE"
    #[serde(rename="NOT_SKIPPABLE")]
    NOTSKIPPABLE,
    

    /// This video ad can be skipped after 5 seconds or not-skippable. This value will appear in RTB bid requests as SkippableBidRequestType::ALLOW_SKIPPABLE.
    ///
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
}

impl AsRef<str> for CreativeRequirementSkippableAdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRequirementSkippableAdTypeEnum::SKIPPABLEADTYPEUNSPECIFIED => "SKIPPABLE_AD_TYPE_UNSPECIFIED",
            CreativeRequirementSkippableAdTypeEnum::SKIPPABLE => "SKIPPABLE",
            CreativeRequirementSkippableAdTypeEnum::INSTREAMSELECT => "INSTREAM_SELECT",
            CreativeRequirementSkippableAdTypeEnum::NOTSKIPPABLE => "NOT_SKIPPABLE",
            CreativeRequirementSkippableAdTypeEnum::ANY => "ANY",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRequirementSkippableAdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SKIPPABLE_AD_TYPE_UNSPECIFIED" => Ok(CreativeRequirementSkippableAdTypeEnum::SKIPPABLEADTYPEUNSPECIFIED),
           "SKIPPABLE" => Ok(CreativeRequirementSkippableAdTypeEnum::SKIPPABLE),
           "INSTREAM_SELECT" => Ok(CreativeRequirementSkippableAdTypeEnum::INSTREAMSELECT),
           "NOT_SKIPPABLE" => Ok(CreativeRequirementSkippableAdTypeEnum::NOTSKIPPABLE),
           "ANY" => Ok(CreativeRequirementSkippableAdTypeEnum::ANY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRequirementSkippableAdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DayPartDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Day of week for the period.
pub enum DayPartDayOfWeekEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for DayPartDayOfWeekEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DayPartDayOfWeekEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            DayPartDayOfWeekEnum::MONDAY => "MONDAY",
            DayPartDayOfWeekEnum::TUESDAY => "TUESDAY",
            DayPartDayOfWeekEnum::WEDNESDAY => "WEDNESDAY",
            DayPartDayOfWeekEnum::THURSDAY => "THURSDAY",
            DayPartDayOfWeekEnum::FRIDAY => "FRIDAY",
            DayPartDayOfWeekEnum::SATURDAY => "SATURDAY",
            DayPartDayOfWeekEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for DayPartDayOfWeekEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(DayPartDayOfWeekEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(DayPartDayOfWeekEnum::MONDAY),
           "TUESDAY" => Ok(DayPartDayOfWeekEnum::TUESDAY),
           "WEDNESDAY" => Ok(DayPartDayOfWeekEnum::WEDNESDAY),
           "THURSDAY" => Ok(DayPartDayOfWeekEnum::THURSDAY),
           "FRIDAY" => Ok(DayPartDayOfWeekEnum::FRIDAY),
           "SATURDAY" => Ok(DayPartDayOfWeekEnum::SATURDAY),
           "SUNDAY" => Ok(DayPartDayOfWeekEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DayPartDayOfWeekEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DayPartTargetingTimeZoneTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time zone type of the day parts
pub enum DayPartTargetingTimeZoneTypeEnum {
    

    /// Default value. This field is unused.
    ///
    /// "TIME_ZONE_TYPE_UNSPECIFIED"
    #[serde(rename="TIME_ZONE_TYPE_UNSPECIFIED")]
    TIMEZONETYPEUNSPECIFIED,
    

    /// The publisher's time zone
    ///
    /// "SELLER"
    #[serde(rename="SELLER")]
    SELLER,
    

    /// The user's time zone
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
}

impl AsRef<str> for DayPartTargetingTimeZoneTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DayPartTargetingTimeZoneTypeEnum::TIMEZONETYPEUNSPECIFIED => "TIME_ZONE_TYPE_UNSPECIFIED",
            DayPartTargetingTimeZoneTypeEnum::SELLER => "SELLER",
            DayPartTargetingTimeZoneTypeEnum::USER => "USER",
        }
    }
}

impl std::convert::TryFrom< &str> for DayPartTargetingTimeZoneTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_ZONE_TYPE_UNSPECIFIED" => Ok(DayPartTargetingTimeZoneTypeEnum::TIMEZONETYPEUNSPECIFIED),
           "SELLER" => Ok(DayPartTargetingTimeZoneTypeEnum::SELLER),
           "USER" => Ok(DayPartTargetingTimeZoneTypeEnum::USER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DayPartTargetingTimeZoneTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealDealTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of deal.
pub enum DealDealTypeEnum {
    

    /// Default, unspecified deal type.
    ///
    /// "DEAL_TYPE_UNSPECIFIED"
    #[serde(rename="DEAL_TYPE_UNSPECIFIED")]
    DEALTYPEUNSPECIFIED,
    

    /// Preferred deals.
    ///
    /// "PREFERRED_DEAL"
    #[serde(rename="PREFERRED_DEAL")]
    PREFERREDDEAL,
    

    /// Private auction deals.
    ///
    /// "PRIVATE_AUCTION"
    #[serde(rename="PRIVATE_AUCTION")]
    PRIVATEAUCTION,
    

    /// Programmatic guaranteed deals.
    ///
    /// "PROGRAMMATIC_GUARANTEED"
    #[serde(rename="PROGRAMMATIC_GUARANTEED")]
    PROGRAMMATICGUARANTEED,
}

impl AsRef<str> for DealDealTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealDealTypeEnum::DEALTYPEUNSPECIFIED => "DEAL_TYPE_UNSPECIFIED",
            DealDealTypeEnum::PREFERREDDEAL => "PREFERRED_DEAL",
            DealDealTypeEnum::PRIVATEAUCTION => "PRIVATE_AUCTION",
            DealDealTypeEnum::PROGRAMMATICGUARANTEED => "PROGRAMMATIC_GUARANTEED",
        }
    }
}

impl std::convert::TryFrom< &str> for DealDealTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEAL_TYPE_UNSPECIFIED" => Ok(DealDealTypeEnum::DEALTYPEUNSPECIFIED),
           "PREFERRED_DEAL" => Ok(DealDealTypeEnum::PREFERREDDEAL),
           "PRIVATE_AUCTION" => Ok(DealDealTypeEnum::PRIVATEAUCTION),
           "PROGRAMMATIC_GUARANTEED" => Ok(DealDealTypeEnum::PROGRAMMATICGUARANTEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealDealTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealPausingInfoPauseRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The party that first paused the deal; unspecified for active deals.
pub enum DealPausingInfoPauseRoleEnum {
    

    /// A placeholder for an undefined buyer/seller role.
    ///
    /// "BUYER_SELLER_ROLE_UNSPECIFIED"
    #[serde(rename="BUYER_SELLER_ROLE_UNSPECIFIED")]
    BUYERSELLERROLEUNSPECIFIED,
    

    /// Specifies the role as buyer.
    ///
    /// "BUYER"
    #[serde(rename="BUYER")]
    BUYER,
    

    /// Specifies the role as seller.
    ///
    /// "SELLER"
    #[serde(rename="SELLER")]
    SELLER,
}

impl AsRef<str> for DealPausingInfoPauseRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealPausingInfoPauseRoleEnum::BUYERSELLERROLEUNSPECIFIED => "BUYER_SELLER_ROLE_UNSPECIFIED",
            DealPausingInfoPauseRoleEnum::BUYER => "BUYER",
            DealPausingInfoPauseRoleEnum::SELLER => "SELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for DealPausingInfoPauseRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUYER_SELLER_ROLE_UNSPECIFIED" => Ok(DealPausingInfoPauseRoleEnum::BUYERSELLERROLEUNSPECIFIED),
           "BUYER" => Ok(DealPausingInfoPauseRoleEnum::BUYER),
           "SELLER" => Ok(DealPausingInfoPauseRoleEnum::SELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealPausingInfoPauseRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliveryControlCompanionDeliveryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies roadblocking in a main companion lineitem.
pub enum DeliveryControlCompanionDeliveryTypeEnum {
    

    /// A placeholder for an unspecified companion delivery type.
    ///
    /// "COMPANION_DELIVERY_TYPE_UNSPECIFIED"
    #[serde(rename="COMPANION_DELIVERY_TYPE_UNSPECIFIED")]
    COMPANIONDELIVERYTYPEUNSPECIFIED,
    

    /// Companions are not required to serve a creative set. The creative set can serve an inventory that has zero or more matching companions.
    ///
    /// "DELIVERY_OPTIONAL"
    #[serde(rename="DELIVERY_OPTIONAL")]
    DELIVERYOPTIONAL,
    

    /// At least one companion must be served in order for the creative set to be used.
    ///
    /// "DELIVERY_AT_LEAST_ONE"
    #[serde(rename="DELIVERY_AT_LEAST_ONE")]
    DELIVERYATLEASTONE,
    

    /// All companions in the set must be served in order for the creative set to be used. This can still serve to inventory that has more companions than can be filled.
    ///
    /// "DELIVERY_ALL"
    #[serde(rename="DELIVERY_ALL")]
    DELIVERYALL,
}

impl AsRef<str> for DeliveryControlCompanionDeliveryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryControlCompanionDeliveryTypeEnum::COMPANIONDELIVERYTYPEUNSPECIFIED => "COMPANION_DELIVERY_TYPE_UNSPECIFIED",
            DeliveryControlCompanionDeliveryTypeEnum::DELIVERYOPTIONAL => "DELIVERY_OPTIONAL",
            DeliveryControlCompanionDeliveryTypeEnum::DELIVERYATLEASTONE => "DELIVERY_AT_LEAST_ONE",
            DeliveryControlCompanionDeliveryTypeEnum::DELIVERYALL => "DELIVERY_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryControlCompanionDeliveryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPANION_DELIVERY_TYPE_UNSPECIFIED" => Ok(DeliveryControlCompanionDeliveryTypeEnum::COMPANIONDELIVERYTYPEUNSPECIFIED),
           "DELIVERY_OPTIONAL" => Ok(DeliveryControlCompanionDeliveryTypeEnum::DELIVERYOPTIONAL),
           "DELIVERY_AT_LEAST_ONE" => Ok(DeliveryControlCompanionDeliveryTypeEnum::DELIVERYATLEASTONE),
           "DELIVERY_ALL" => Ok(DeliveryControlCompanionDeliveryTypeEnum::DELIVERYALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryControlCompanionDeliveryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliveryControlCreativeRotationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies strategy to use for selecting a creative when multiple creatives of the same size are available.
pub enum DeliveryControlCreativeRotationTypeEnum {
    

    /// Creatives are displayed roughly the same number of times over the duration of the deal.
    ///
    /// "CREATIVE_ROTATION_TYPE_UNSPECIFIED"
    #[serde(rename="CREATIVE_ROTATION_TYPE_UNSPECIFIED")]
    CREATIVEROTATIONTYPEUNSPECIFIED,
    

    /// Creatives are displayed roughly the same number of times over the duration of the deal.
    ///
    /// "ROTATION_EVEN"
    #[serde(rename="ROTATION_EVEN")]
    ROTATIONEVEN,
    

    /// Creatives are served roughly proportionally to their performance.
    ///
    /// "ROTATION_OPTIMIZED"
    #[serde(rename="ROTATION_OPTIMIZED")]
    ROTATIONOPTIMIZED,
    

    /// Creatives are served roughly proportionally to their weights.
    ///
    /// "ROTATION_MANUAL"
    #[serde(rename="ROTATION_MANUAL")]
    ROTATIONMANUAL,
    

    /// Creatives are served exactly in sequential order, also known as Storyboarding.
    ///
    /// "ROTATION_SEQUENTIAL"
    #[serde(rename="ROTATION_SEQUENTIAL")]
    ROTATIONSEQUENTIAL,
}

impl AsRef<str> for DeliveryControlCreativeRotationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryControlCreativeRotationTypeEnum::CREATIVEROTATIONTYPEUNSPECIFIED => "CREATIVE_ROTATION_TYPE_UNSPECIFIED",
            DeliveryControlCreativeRotationTypeEnum::ROTATIONEVEN => "ROTATION_EVEN",
            DeliveryControlCreativeRotationTypeEnum::ROTATIONOPTIMIZED => "ROTATION_OPTIMIZED",
            DeliveryControlCreativeRotationTypeEnum::ROTATIONMANUAL => "ROTATION_MANUAL",
            DeliveryControlCreativeRotationTypeEnum::ROTATIONSEQUENTIAL => "ROTATION_SEQUENTIAL",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryControlCreativeRotationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_ROTATION_TYPE_UNSPECIFIED" => Ok(DeliveryControlCreativeRotationTypeEnum::CREATIVEROTATIONTYPEUNSPECIFIED),
           "ROTATION_EVEN" => Ok(DeliveryControlCreativeRotationTypeEnum::ROTATIONEVEN),
           "ROTATION_OPTIMIZED" => Ok(DeliveryControlCreativeRotationTypeEnum::ROTATIONOPTIMIZED),
           "ROTATION_MANUAL" => Ok(DeliveryControlCreativeRotationTypeEnum::ROTATIONMANUAL),
           "ROTATION_SEQUENTIAL" => Ok(DeliveryControlCreativeRotationTypeEnum::ROTATIONSEQUENTIAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryControlCreativeRotationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliveryControlDeliveryRateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies how the impression delivery will be paced.
pub enum DeliveryControlDeliveryRateTypeEnum {
    

    /// A placeholder for an undefined delivery rate type.
    ///
    /// "DELIVERY_RATE_TYPE_UNSPECIFIED"
    #[serde(rename="DELIVERY_RATE_TYPE_UNSPECIFIED")]
    DELIVERYRATETYPEUNSPECIFIED,
    

    /// Impressions are served uniformly over the life of the deal.
    ///
    /// "EVENLY"
    #[serde(rename="EVENLY")]
    EVENLY,
    

    /// Impressions are served front-loaded.
    ///
    /// "FRONT_LOADED"
    #[serde(rename="FRONT_LOADED")]
    FRONTLOADED,
    

    /// Impressions are served as fast as possible.
    ///
    /// "AS_FAST_AS_POSSIBLE"
    #[serde(rename="AS_FAST_AS_POSSIBLE")]
    ASFASTASPOSSIBLE,
}

impl AsRef<str> for DeliveryControlDeliveryRateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryControlDeliveryRateTypeEnum::DELIVERYRATETYPEUNSPECIFIED => "DELIVERY_RATE_TYPE_UNSPECIFIED",
            DeliveryControlDeliveryRateTypeEnum::EVENLY => "EVENLY",
            DeliveryControlDeliveryRateTypeEnum::FRONTLOADED => "FRONT_LOADED",
            DeliveryControlDeliveryRateTypeEnum::ASFASTASPOSSIBLE => "AS_FAST_AS_POSSIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryControlDeliveryRateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELIVERY_RATE_TYPE_UNSPECIFIED" => Ok(DeliveryControlDeliveryRateTypeEnum::DELIVERYRATETYPEUNSPECIFIED),
           "EVENLY" => Ok(DeliveryControlDeliveryRateTypeEnum::EVENLY),
           "FRONT_LOADED" => Ok(DeliveryControlDeliveryRateTypeEnum::FRONTLOADED),
           "AS_FAST_AS_POSSIBLE" => Ok(DeliveryControlDeliveryRateTypeEnum::ASFASTASPOSSIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryControlDeliveryRateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliveryControlRoadblockingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the roadblocking type in display creatives.
pub enum DeliveryControlRoadblockingTypeEnum {
    

    /// A placeholder for an unspecified roadblocking type.
    ///
    /// "ROADBLOCKING_TYPE_UNSPECIFIED"
    #[serde(rename="ROADBLOCKING_TYPE_UNSPECIFIED")]
    ROADBLOCKINGTYPEUNSPECIFIED,
    

    /// Only one creative from a deal can serve per ad request. https://support.google.com/admanager/answer/177277.
    ///
    /// "ONLY_ONE"
    #[serde(rename="ONLY_ONE")]
    ONLYONE,
    

    /// Any number of creatives from a deal can serve together per ad request.
    ///
    /// "ONE_OR_MORE"
    #[serde(rename="ONE_OR_MORE")]
    ONEORMORE,
    

    /// As many creatives from a deal as can fit on a page will serve. This could mean anywhere from one to all of a deal's creatives given the size constraints of ad slots on a page.
    ///
    /// "AS_MANY_AS_POSSIBLE"
    #[serde(rename="AS_MANY_AS_POSSIBLE")]
    ASMANYASPOSSIBLE,
    

    /// All or none of the creatives from a deal will serve.
    ///
    /// "ALL_ROADBLOCK"
    #[serde(rename="ALL_ROADBLOCK")]
    ALLROADBLOCK,
    

    /// A main/companion creative set roadblocking type.
    ///
    /// "CREATIVE_SET"
    #[serde(rename="CREATIVE_SET")]
    CREATIVESET,
}

impl AsRef<str> for DeliveryControlRoadblockingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryControlRoadblockingTypeEnum::ROADBLOCKINGTYPEUNSPECIFIED => "ROADBLOCKING_TYPE_UNSPECIFIED",
            DeliveryControlRoadblockingTypeEnum::ONLYONE => "ONLY_ONE",
            DeliveryControlRoadblockingTypeEnum::ONEORMORE => "ONE_OR_MORE",
            DeliveryControlRoadblockingTypeEnum::ASMANYASPOSSIBLE => "AS_MANY_AS_POSSIBLE",
            DeliveryControlRoadblockingTypeEnum::ALLROADBLOCK => "ALL_ROADBLOCK",
            DeliveryControlRoadblockingTypeEnum::CREATIVESET => "CREATIVE_SET",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryControlRoadblockingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROADBLOCKING_TYPE_UNSPECIFIED" => Ok(DeliveryControlRoadblockingTypeEnum::ROADBLOCKINGTYPEUNSPECIFIED),
           "ONLY_ONE" => Ok(DeliveryControlRoadblockingTypeEnum::ONLYONE),
           "ONE_OR_MORE" => Ok(DeliveryControlRoadblockingTypeEnum::ONEORMORE),
           "AS_MANY_AS_POSSIBLE" => Ok(DeliveryControlRoadblockingTypeEnum::ASMANYASPOSSIBLE),
           "ALL_ROADBLOCK" => Ok(DeliveryControlRoadblockingTypeEnum::ALLROADBLOCK),
           "CREATIVE_SET" => Ok(DeliveryControlRoadblockingTypeEnum::CREATIVESET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryControlRoadblockingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FinalizedDealDealServingStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Serving status of the deal.
pub enum FinalizedDealDealServingStatusEnum {
    

    /// Unspecified.
    ///
    /// "DEAL_SERVING_STATUS_UNSPECIFIED"
    #[serde(rename="DEAL_SERVING_STATUS_UNSPECIFIED")]
    DEALSERVINGSTATUSUNSPECIFIED,
    

    /// The deal is actively serving or ready to serve when the start date is reached.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The deal serving has ended.
    ///
    /// "ENDED"
    #[serde(rename="ENDED")]
    ENDED,
    

    /// The deal serving is paused by buyer.
    ///
    /// "PAUSED_BY_BUYER"
    #[serde(rename="PAUSED_BY_BUYER")]
    PAUSEDBYBUYER,
    

    /// The deal serving is paused by seller.
    ///
    /// "PAUSED_BY_SELLER"
    #[serde(rename="PAUSED_BY_SELLER")]
    PAUSEDBYSELLER,
}

impl AsRef<str> for FinalizedDealDealServingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FinalizedDealDealServingStatusEnum::DEALSERVINGSTATUSUNSPECIFIED => "DEAL_SERVING_STATUS_UNSPECIFIED",
            FinalizedDealDealServingStatusEnum::ACTIVE => "ACTIVE",
            FinalizedDealDealServingStatusEnum::ENDED => "ENDED",
            FinalizedDealDealServingStatusEnum::PAUSEDBYBUYER => "PAUSED_BY_BUYER",
            FinalizedDealDealServingStatusEnum::PAUSEDBYSELLER => "PAUSED_BY_SELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for FinalizedDealDealServingStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEAL_SERVING_STATUS_UNSPECIFIED" => Ok(FinalizedDealDealServingStatusEnum::DEALSERVINGSTATUSUNSPECIFIED),
           "ACTIVE" => Ok(FinalizedDealDealServingStatusEnum::ACTIVE),
           "ENDED" => Ok(FinalizedDealDealServingStatusEnum::ENDED),
           "PAUSED_BY_BUYER" => Ok(FinalizedDealDealServingStatusEnum::PAUSEDBYBUYER),
           "PAUSED_BY_SELLER" => Ok(FinalizedDealDealServingStatusEnum::PAUSEDBYSELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FinalizedDealDealServingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FrequencyCapTimeUnitTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time unit. Along with num_time_units defines the amount of time over which impressions per user are counted and capped.
pub enum FrequencyCapTimeUnitTypeEnum {
    

    /// A placeholder for an undefined time unit type. This just indicates the variable with this value hasn't been initialized.
    ///
    /// "TIME_UNIT_TYPE_UNSPECIFIED"
    #[serde(rename="TIME_UNIT_TYPE_UNSPECIFIED")]
    TIMEUNITTYPEUNSPECIFIED,
    

    /// Minute unit.
    ///
    /// "MINUTE"
    #[serde(rename="MINUTE")]
    MINUTE,
    

    /// Hour unit.
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
    

    /// Day unit.
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// Week unit.
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// Month unit.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Lifecycle/Lifetime unit.
    ///
    /// "LIFETIME"
    #[serde(rename="LIFETIME")]
    LIFETIME,
    

    /// Pod unit.
    ///
    /// "POD"
    #[serde(rename="POD")]
    POD,
    

    /// Stream unit.
    ///
    /// "STREAM"
    #[serde(rename="STREAM")]
    STREAM,
}

impl AsRef<str> for FrequencyCapTimeUnitTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FrequencyCapTimeUnitTypeEnum::TIMEUNITTYPEUNSPECIFIED => "TIME_UNIT_TYPE_UNSPECIFIED",
            FrequencyCapTimeUnitTypeEnum::MINUTE => "MINUTE",
            FrequencyCapTimeUnitTypeEnum::HOUR => "HOUR",
            FrequencyCapTimeUnitTypeEnum::DAY => "DAY",
            FrequencyCapTimeUnitTypeEnum::WEEK => "WEEK",
            FrequencyCapTimeUnitTypeEnum::MONTH => "MONTH",
            FrequencyCapTimeUnitTypeEnum::LIFETIME => "LIFETIME",
            FrequencyCapTimeUnitTypeEnum::POD => "POD",
            FrequencyCapTimeUnitTypeEnum::STREAM => "STREAM",
        }
    }
}

impl std::convert::TryFrom< &str> for FrequencyCapTimeUnitTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_UNIT_TYPE_UNSPECIFIED" => Ok(FrequencyCapTimeUnitTypeEnum::TIMEUNITTYPEUNSPECIFIED),
           "MINUTE" => Ok(FrequencyCapTimeUnitTypeEnum::MINUTE),
           "HOUR" => Ok(FrequencyCapTimeUnitTypeEnum::HOUR),
           "DAY" => Ok(FrequencyCapTimeUnitTypeEnum::DAY),
           "WEEK" => Ok(FrequencyCapTimeUnitTypeEnum::WEEK),
           "MONTH" => Ok(FrequencyCapTimeUnitTypeEnum::MONTH),
           "LIFETIME" => Ok(FrequencyCapTimeUnitTypeEnum::LIFETIME),
           "POD" => Ok(FrequencyCapTimeUnitTypeEnum::POD),
           "STREAM" => Ok(FrequencyCapTimeUnitTypeEnum::STREAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FrequencyCapTimeUnitTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InventoryTypeTargetingInventoryTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of targeted inventory types for the bid request.
pub enum InventoryTypeTargetingInventoryTypesEnum {
    

    /// Unspecified inventory type
    ///
    /// "INVENTORY_TYPE_UNSPECIFIED"
    #[serde(rename="INVENTORY_TYPE_UNSPECIFIED")]
    INVENTORYTYPEUNSPECIFIED,
    

    /// Desktop or mobile web browser excluding ads inside a video player
    ///
    /// "BROWSER"
    #[serde(rename="BROWSER")]
    BROWSER,
    

    /// Mobile apps other than video players and web browsers
    ///
    /// "MOBILE_APP"
    #[serde(rename="MOBILE_APP")]
    MOBILEAPP,
    

    /// Instream video and audio
    ///
    /// "VIDEO_PLAYER"
    #[serde(rename="VIDEO_PLAYER")]
    VIDEOPLAYER,
}

impl AsRef<str> for InventoryTypeTargetingInventoryTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InventoryTypeTargetingInventoryTypesEnum::INVENTORYTYPEUNSPECIFIED => "INVENTORY_TYPE_UNSPECIFIED",
            InventoryTypeTargetingInventoryTypesEnum::BROWSER => "BROWSER",
            InventoryTypeTargetingInventoryTypesEnum::MOBILEAPP => "MOBILE_APP",
            InventoryTypeTargetingInventoryTypesEnum::VIDEOPLAYER => "VIDEO_PLAYER",
        }
    }
}

impl std::convert::TryFrom< &str> for InventoryTypeTargetingInventoryTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVENTORY_TYPE_UNSPECIFIED" => Ok(InventoryTypeTargetingInventoryTypesEnum::INVENTORYTYPEUNSPECIFIED),
           "BROWSER" => Ok(InventoryTypeTargetingInventoryTypesEnum::BROWSER),
           "MOBILE_APP" => Ok(InventoryTypeTargetingInventoryTypesEnum::MOBILEAPP),
           "VIDEO_PLAYER" => Ok(InventoryTypeTargetingInventoryTypesEnum::VIDEOPLAYER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InventoryTypeTargetingInventoryTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NoteCreatorRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The role who created the note.
pub enum NoteCreatorRoleEnum {
    

    /// A placeholder for an undefined buyer/seller role.
    ///
    /// "BUYER_SELLER_ROLE_UNSPECIFIED"
    #[serde(rename="BUYER_SELLER_ROLE_UNSPECIFIED")]
    BUYERSELLERROLEUNSPECIFIED,
    

    /// Specifies the role as buyer.
    ///
    /// "BUYER"
    #[serde(rename="BUYER")]
    BUYER,
    

    /// Specifies the role as seller.
    ///
    /// "SELLER"
    #[serde(rename="SELLER")]
    SELLER,
}

impl AsRef<str> for NoteCreatorRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NoteCreatorRoleEnum::BUYERSELLERROLEUNSPECIFIED => "BUYER_SELLER_ROLE_UNSPECIFIED",
            NoteCreatorRoleEnum::BUYER => "BUYER",
            NoteCreatorRoleEnum::SELLER => "SELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for NoteCreatorRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUYER_SELLER_ROLE_UNSPECIFIED" => Ok(NoteCreatorRoleEnum::BUYERSELLERROLEUNSPECIFIED),
           "BUYER" => Ok(NoteCreatorRoleEnum::BUYER),
           "SELLER" => Ok(NoteCreatorRoleEnum::SELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NoteCreatorRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PriceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The pricing type for the deal.
pub enum PriceTypeEnum {
    

    /// A placeholder for an undefined pricing type. If the pricing type is unspecified, CPM will be used instead.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Cost per thousand impressions.
    ///
    /// "CPM"
    #[serde(rename="CPM")]
    CPM,
    

    /// Cost per day.
    ///
    /// "CPD"
    #[serde(rename="CPD")]
    CPD,
}

impl AsRef<str> for PriceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PriceTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            PriceTypeEnum::CPM => "CPM",
            PriceTypeEnum::CPD => "CPD",
        }
    }
}

impl std::convert::TryFrom< &str> for PriceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(PriceTypeEnum::TYPEUNSPECIFIED),
           "CPM" => Ok(PriceTypeEnum::CPM),
           "CPD" => Ok(PriceTypeEnum::CPD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PriceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProgrammaticGuaranteedTermReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reservation type for a Programmatic Guaranteed deal. This indicates whether the number of impressions is fixed, or a percent of available impressions. If not specified, the default reservation type is STANDARD.
pub enum ProgrammaticGuaranteedTermReservationTypeEnum {
    

    /// An unspecified reservation type.
    ///
    /// "RESERVATION_TYPE_UNSPECIFIED"
    #[serde(rename="RESERVATION_TYPE_UNSPECIFIED")]
    RESERVATIONTYPEUNSPECIFIED,
    

    /// Non-sponsorship deal.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Sponsorship deals don't have impression goal (guaranteed_looks) and they are served based on the flight dates. For CPM Sponsorship deals, impression_cap is the lifetime impression limit.
    ///
    /// "SPONSORSHIP"
    #[serde(rename="SPONSORSHIP")]
    SPONSORSHIP,
}

impl AsRef<str> for ProgrammaticGuaranteedTermReservationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProgrammaticGuaranteedTermReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED => "RESERVATION_TYPE_UNSPECIFIED",
            ProgrammaticGuaranteedTermReservationTypeEnum::STANDARD => "STANDARD",
            ProgrammaticGuaranteedTermReservationTypeEnum::SPONSORSHIP => "SPONSORSHIP",
        }
    }
}

impl std::convert::TryFrom< &str> for ProgrammaticGuaranteedTermReservationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESERVATION_TYPE_UNSPECIFIED" => Ok(ProgrammaticGuaranteedTermReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED),
           "STANDARD" => Ok(ProgrammaticGuaranteedTermReservationTypeEnum::STANDARD),
           "SPONSORSHIP" => Ok(ProgrammaticGuaranteedTermReservationTypeEnum::SPONSORSHIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProgrammaticGuaranteedTermReservationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProposalDealTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of deal the proposal contains.
pub enum ProposalDealTypeEnum {
    

    /// Default, unspecified deal type.
    ///
    /// "DEAL_TYPE_UNSPECIFIED"
    #[serde(rename="DEAL_TYPE_UNSPECIFIED")]
    DEALTYPEUNSPECIFIED,
    

    /// Preferred deals.
    ///
    /// "PREFERRED_DEAL"
    #[serde(rename="PREFERRED_DEAL")]
    PREFERREDDEAL,
    

    /// Private auction deals.
    ///
    /// "PRIVATE_AUCTION"
    #[serde(rename="PRIVATE_AUCTION")]
    PRIVATEAUCTION,
    

    /// Programmatic guaranteed deals.
    ///
    /// "PROGRAMMATIC_GUARANTEED"
    #[serde(rename="PROGRAMMATIC_GUARANTEED")]
    PROGRAMMATICGUARANTEED,
}

impl AsRef<str> for ProposalDealTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProposalDealTypeEnum::DEALTYPEUNSPECIFIED => "DEAL_TYPE_UNSPECIFIED",
            ProposalDealTypeEnum::PREFERREDDEAL => "PREFERRED_DEAL",
            ProposalDealTypeEnum::PRIVATEAUCTION => "PRIVATE_AUCTION",
            ProposalDealTypeEnum::PROGRAMMATICGUARANTEED => "PROGRAMMATIC_GUARANTEED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProposalDealTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEAL_TYPE_UNSPECIFIED" => Ok(ProposalDealTypeEnum::DEALTYPEUNSPECIFIED),
           "PREFERRED_DEAL" => Ok(ProposalDealTypeEnum::PREFERREDDEAL),
           "PRIVATE_AUCTION" => Ok(ProposalDealTypeEnum::PRIVATEAUCTION),
           "PROGRAMMATIC_GUARANTEED" => Ok(ProposalDealTypeEnum::PROGRAMMATICGUARANTEED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProposalDealTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProposalLastUpdaterOrCommentorRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The role of the last user that either updated the proposal or left a comment.
pub enum ProposalLastUpdaterOrCommentorRoleEnum {
    

    /// A placeholder for an undefined buyer/seller role.
    ///
    /// "BUYER_SELLER_ROLE_UNSPECIFIED"
    #[serde(rename="BUYER_SELLER_ROLE_UNSPECIFIED")]
    BUYERSELLERROLEUNSPECIFIED,
    

    /// Specifies the role as buyer.
    ///
    /// "BUYER"
    #[serde(rename="BUYER")]
    BUYER,
    

    /// Specifies the role as seller.
    ///
    /// "SELLER"
    #[serde(rename="SELLER")]
    SELLER,
}

impl AsRef<str> for ProposalLastUpdaterOrCommentorRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProposalLastUpdaterOrCommentorRoleEnum::BUYERSELLERROLEUNSPECIFIED => "BUYER_SELLER_ROLE_UNSPECIFIED",
            ProposalLastUpdaterOrCommentorRoleEnum::BUYER => "BUYER",
            ProposalLastUpdaterOrCommentorRoleEnum::SELLER => "SELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for ProposalLastUpdaterOrCommentorRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUYER_SELLER_ROLE_UNSPECIFIED" => Ok(ProposalLastUpdaterOrCommentorRoleEnum::BUYERSELLERROLEUNSPECIFIED),
           "BUYER" => Ok(ProposalLastUpdaterOrCommentorRoleEnum::BUYER),
           "SELLER" => Ok(ProposalLastUpdaterOrCommentorRoleEnum::SELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProposalLastUpdaterOrCommentorRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProposalOriginatorRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates whether the buyer/seller created the proposal.
pub enum ProposalOriginatorRoleEnum {
    

    /// A placeholder for an undefined buyer/seller role.
    ///
    /// "BUYER_SELLER_ROLE_UNSPECIFIED"
    #[serde(rename="BUYER_SELLER_ROLE_UNSPECIFIED")]
    BUYERSELLERROLEUNSPECIFIED,
    

    /// Specifies the role as buyer.
    ///
    /// "BUYER"
    #[serde(rename="BUYER")]
    BUYER,
    

    /// Specifies the role as seller.
    ///
    /// "SELLER"
    #[serde(rename="SELLER")]
    SELLER,
}

impl AsRef<str> for ProposalOriginatorRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProposalOriginatorRoleEnum::BUYERSELLERROLEUNSPECIFIED => "BUYER_SELLER_ROLE_UNSPECIFIED",
            ProposalOriginatorRoleEnum::BUYER => "BUYER",
            ProposalOriginatorRoleEnum::SELLER => "SELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for ProposalOriginatorRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUYER_SELLER_ROLE_UNSPECIFIED" => Ok(ProposalOriginatorRoleEnum::BUYERSELLERROLEUNSPECIFIED),
           "BUYER" => Ok(ProposalOriginatorRoleEnum::BUYER),
           "SELLER" => Ok(ProposalOriginatorRoleEnum::SELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProposalOriginatorRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProposalStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Indicates the state of the proposal.
pub enum ProposalStateEnum {
    

    /// Unspecified proposal state
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// When a proposal is waiting for buyer to review.
    ///
    /// "BUYER_REVIEW_REQUESTED"
    #[serde(rename="BUYER_REVIEW_REQUESTED")]
    BUYERREVIEWREQUESTED,
    

    /// When the proposal is waiting for the seller to review.
    ///
    /// "SELLER_REVIEW_REQUESTED"
    #[serde(rename="SELLER_REVIEW_REQUESTED")]
    SELLERREVIEWREQUESTED,
    

    /// When the seller accepted the proposal and sent it to the buyer for review.
    ///
    /// "BUYER_ACCEPTANCE_REQUESTED"
    #[serde(rename="BUYER_ACCEPTANCE_REQUESTED")]
    BUYERACCEPTANCEREQUESTED,
    

    /// When both buyer and seller has accepted the proposal
    ///
    /// "FINALIZED"
    #[serde(rename="FINALIZED")]
    FINALIZED,
    

    /// When either buyer or seller has cancelled the proposal
    ///
    /// "TERMINATED"
    #[serde(rename="TERMINATED")]
    TERMINATED,
}

impl AsRef<str> for ProposalStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProposalStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ProposalStateEnum::BUYERREVIEWREQUESTED => "BUYER_REVIEW_REQUESTED",
            ProposalStateEnum::SELLERREVIEWREQUESTED => "SELLER_REVIEW_REQUESTED",
            ProposalStateEnum::BUYERACCEPTANCEREQUESTED => "BUYER_ACCEPTANCE_REQUESTED",
            ProposalStateEnum::FINALIZED => "FINALIZED",
            ProposalStateEnum::TERMINATED => "TERMINATED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProposalStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ProposalStateEnum::STATEUNSPECIFIED),
           "BUYER_REVIEW_REQUESTED" => Ok(ProposalStateEnum::BUYERREVIEWREQUESTED),
           "SELLER_REVIEW_REQUESTED" => Ok(ProposalStateEnum::SELLERREVIEWREQUESTED),
           "BUYER_ACCEPTANCE_REQUESTED" => Ok(ProposalStateEnum::BUYERACCEPTANCEREQUESTED),
           "FINALIZED" => Ok(ProposalStateEnum::FINALIZED),
           "TERMINATED" => Ok(ProposalStateEnum::TERMINATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProposalStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublisherProfileMobileApplicationAppStoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The app store the app belongs to. Can be used to filter the response of the publisherProfiles.list method.
pub enum PublisherProfileMobileApplicationAppStoreEnum {
    

    /// A placeholder for an unknown app store.
    ///
    /// "APP_STORE_TYPE_UNSPECIFIED"
    #[serde(rename="APP_STORE_TYPE_UNSPECIFIED")]
    APPSTORETYPEUNSPECIFIED,
    

    /// Apple iTunes
    ///
    /// "APPLE_ITUNES"
    #[serde(rename="APPLE_ITUNES")]
    APPLEITUNES,
    

    /// Google Play
    ///
    /// "GOOGLE_PLAY"
    #[serde(rename="GOOGLE_PLAY")]
    GOOGLEPLAY,
    

    /// Roku
    ///
    /// "ROKU"
    #[serde(rename="ROKU")]
    ROKU,
    

    /// Amazon Fire TV
    ///
    /// "AMAZON_FIRE_TV"
    #[serde(rename="AMAZON_FIRE_TV")]
    AMAZONFIRETV,
    

    /// PlayStation
    ///
    /// "PLAYSTATION"
    #[serde(rename="PLAYSTATION")]
    PLAYSTATION,
    

    /// Xbox
    ///
    /// "XBOX"
    #[serde(rename="XBOX")]
    XBOX,
    

    /// Samsung TV
    ///
    /// "SAMSUNG_TV"
    #[serde(rename="SAMSUNG_TV")]
    SAMSUNGTV,
    

    /// Amazon Appstore
    ///
    /// "AMAZON"
    #[serde(rename="AMAZON")]
    AMAZON,
    

    /// OPPO App Market
    ///
    /// "OPPO"
    #[serde(rename="OPPO")]
    OPPO,
    

    /// Samsung Galaxy Store
    ///
    /// "SAMSUNG"
    #[serde(rename="SAMSUNG")]
    SAMSUNG,
    

    /// VIVO App Store
    ///
    /// "VIVO"
    #[serde(rename="VIVO")]
    VIVO,
    

    /// Xiaomi GetApps
    ///
    /// "XIAOMI"
    #[serde(rename="XIAOMI")]
    XIAOMI,
    

    /// LG TV
    ///
    /// "LG_TV"
    #[serde(rename="LG_TV")]
    LGTV,
}

impl AsRef<str> for PublisherProfileMobileApplicationAppStoreEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublisherProfileMobileApplicationAppStoreEnum::APPSTORETYPEUNSPECIFIED => "APP_STORE_TYPE_UNSPECIFIED",
            PublisherProfileMobileApplicationAppStoreEnum::APPLEITUNES => "APPLE_ITUNES",
            PublisherProfileMobileApplicationAppStoreEnum::GOOGLEPLAY => "GOOGLE_PLAY",
            PublisherProfileMobileApplicationAppStoreEnum::ROKU => "ROKU",
            PublisherProfileMobileApplicationAppStoreEnum::AMAZONFIRETV => "AMAZON_FIRE_TV",
            PublisherProfileMobileApplicationAppStoreEnum::PLAYSTATION => "PLAYSTATION",
            PublisherProfileMobileApplicationAppStoreEnum::XBOX => "XBOX",
            PublisherProfileMobileApplicationAppStoreEnum::SAMSUNGTV => "SAMSUNG_TV",
            PublisherProfileMobileApplicationAppStoreEnum::AMAZON => "AMAZON",
            PublisherProfileMobileApplicationAppStoreEnum::OPPO => "OPPO",
            PublisherProfileMobileApplicationAppStoreEnum::SAMSUNG => "SAMSUNG",
            PublisherProfileMobileApplicationAppStoreEnum::VIVO => "VIVO",
            PublisherProfileMobileApplicationAppStoreEnum::XIAOMI => "XIAOMI",
            PublisherProfileMobileApplicationAppStoreEnum::LGTV => "LG_TV",
        }
    }
}

impl std::convert::TryFrom< &str> for PublisherProfileMobileApplicationAppStoreEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_STORE_TYPE_UNSPECIFIED" => Ok(PublisherProfileMobileApplicationAppStoreEnum::APPSTORETYPEUNSPECIFIED),
           "APPLE_ITUNES" => Ok(PublisherProfileMobileApplicationAppStoreEnum::APPLEITUNES),
           "GOOGLE_PLAY" => Ok(PublisherProfileMobileApplicationAppStoreEnum::GOOGLEPLAY),
           "ROKU" => Ok(PublisherProfileMobileApplicationAppStoreEnum::ROKU),
           "AMAZON_FIRE_TV" => Ok(PublisherProfileMobileApplicationAppStoreEnum::AMAZONFIRETV),
           "PLAYSTATION" => Ok(PublisherProfileMobileApplicationAppStoreEnum::PLAYSTATION),
           "XBOX" => Ok(PublisherProfileMobileApplicationAppStoreEnum::XBOX),
           "SAMSUNG_TV" => Ok(PublisherProfileMobileApplicationAppStoreEnum::SAMSUNGTV),
           "AMAZON" => Ok(PublisherProfileMobileApplicationAppStoreEnum::AMAZON),
           "OPPO" => Ok(PublisherProfileMobileApplicationAppStoreEnum::OPPO),
           "SAMSUNG" => Ok(PublisherProfileMobileApplicationAppStoreEnum::SAMSUNG),
           "VIVO" => Ok(PublisherProfileMobileApplicationAppStoreEnum::VIVO),
           "XIAOMI" => Ok(PublisherProfileMobileApplicationAppStoreEnum::XIAOMI),
           "LG_TV" => Ok(PublisherProfileMobileApplicationAppStoreEnum::LGTV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublisherProfileMobileApplicationAppStoreEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoTargetingExcludedPositionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of video positions to be excluded. When this field is populated, the targeted_position_types field must be empty.
pub enum VideoTargetingExcludedPositionTypesEnum {
    

    /// A placeholder for an undefined video position.
    ///
    /// "POSITION_TYPE_UNSPECIFIED"
    #[serde(rename="POSITION_TYPE_UNSPECIFIED")]
    POSITIONTYPEUNSPECIFIED,
    

    /// Ad is played before the video.
    ///
    /// "PREROLL"
    #[serde(rename="PREROLL")]
    PREROLL,
    

    /// Ad is played during the video.
    ///
    /// "MIDROLL"
    #[serde(rename="MIDROLL")]
    MIDROLL,
    

    /// Ad is played after the video.
    ///
    /// "POSTROLL"
    #[serde(rename="POSTROLL")]
    POSTROLL,
}

impl AsRef<str> for VideoTargetingExcludedPositionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoTargetingExcludedPositionTypesEnum::POSITIONTYPEUNSPECIFIED => "POSITION_TYPE_UNSPECIFIED",
            VideoTargetingExcludedPositionTypesEnum::PREROLL => "PREROLL",
            VideoTargetingExcludedPositionTypesEnum::MIDROLL => "MIDROLL",
            VideoTargetingExcludedPositionTypesEnum::POSTROLL => "POSTROLL",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoTargetingExcludedPositionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POSITION_TYPE_UNSPECIFIED" => Ok(VideoTargetingExcludedPositionTypesEnum::POSITIONTYPEUNSPECIFIED),
           "PREROLL" => Ok(VideoTargetingExcludedPositionTypesEnum::PREROLL),
           "MIDROLL" => Ok(VideoTargetingExcludedPositionTypesEnum::MIDROLL),
           "POSTROLL" => Ok(VideoTargetingExcludedPositionTypesEnum::POSTROLL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoTargetingExcludedPositionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoTargetingTargetedPositionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of video positions to be included. When this field is populated, the excluded_position_types field must be empty.
pub enum VideoTargetingTargetedPositionTypesEnum {
    

    /// A placeholder for an undefined video position.
    ///
    /// "POSITION_TYPE_UNSPECIFIED"
    #[serde(rename="POSITION_TYPE_UNSPECIFIED")]
    POSITIONTYPEUNSPECIFIED,
    

    /// Ad is played before the video.
    ///
    /// "PREROLL"
    #[serde(rename="PREROLL")]
    PREROLL,
    

    /// Ad is played during the video.
    ///
    /// "MIDROLL"
    #[serde(rename="MIDROLL")]
    MIDROLL,
    

    /// Ad is played after the video.
    ///
    /// "POSTROLL"
    #[serde(rename="POSTROLL")]
    POSTROLL,
}

impl AsRef<str> for VideoTargetingTargetedPositionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoTargetingTargetedPositionTypesEnum::POSITIONTYPEUNSPECIFIED => "POSITION_TYPE_UNSPECIFIED",
            VideoTargetingTargetedPositionTypesEnum::PREROLL => "PREROLL",
            VideoTargetingTargetedPositionTypesEnum::MIDROLL => "MIDROLL",
            VideoTargetingTargetedPositionTypesEnum::POSTROLL => "POSTROLL",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoTargetingTargetedPositionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POSITION_TYPE_UNSPECIFIED" => Ok(VideoTargetingTargetedPositionTypesEnum::POSITIONTYPEUNSPECIFIED),
           "PREROLL" => Ok(VideoTargetingTargetedPositionTypesEnum::PREROLL),
           "MIDROLL" => Ok(VideoTargetingTargetedPositionTypesEnum::MIDROLL),
           "POSTROLL" => Ok(VideoTargetingTargetedPositionTypesEnum::POSTROLL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoTargetingTargetedPositionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


