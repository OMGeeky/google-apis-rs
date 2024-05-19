use super::*;



// region AddTargetedAppsRequestTargetingModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The targeting mode that should be applied to the list of app IDs. If there are existing targeted app IDs, must be equal to the existing PretargetingConfig.appTargeting.mobileAppTargeting.targetingMode or a 400 bad request error will be returned.
pub enum AddTargetedAppsRequestTargetingModeEnum {
    

    /// Placeholder for undefined targeting mode.
    ///
    /// "TARGETING_MODE_UNSPECIFIED"
    #[serde(rename="TARGETING_MODE_UNSPECIFIED")]
    TARGETINGMODEUNSPECIFIED,
    

    /// The inclusive list type. Inventory must match an item in this list to be targeted.
    ///
    /// "INCLUSIVE"
    #[serde(rename="INCLUSIVE")]
    INCLUSIVE,
    

    /// The exclusive list type. Inventory must not match any item in this list to be targeted.
    ///
    /// "EXCLUSIVE"
    #[serde(rename="EXCLUSIVE")]
    EXCLUSIVE,
}

impl AsRef<str> for AddTargetedAppsRequestTargetingModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AddTargetedAppsRequestTargetingModeEnum::TARGETINGMODEUNSPECIFIED => "TARGETING_MODE_UNSPECIFIED",
            AddTargetedAppsRequestTargetingModeEnum::INCLUSIVE => "INCLUSIVE",
            AddTargetedAppsRequestTargetingModeEnum::EXCLUSIVE => "EXCLUSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AddTargetedAppsRequestTargetingModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_MODE_UNSPECIFIED" => Ok(AddTargetedAppsRequestTargetingModeEnum::TARGETINGMODEUNSPECIFIED),
           "INCLUSIVE" => Ok(AddTargetedAppsRequestTargetingModeEnum::INCLUSIVE),
           "EXCLUSIVE" => Ok(AddTargetedAppsRequestTargetingModeEnum::EXCLUSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AddTargetedAppsRequestTargetingModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AddTargetedPublishersRequestTargetingModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The targeting mode that should be applied to the list of publisher IDs. If are existing publisher IDs, must be equal to the existing PretargetingConfig.publisherTargeting.targetingMode or a 400 bad request error will be returned.
pub enum AddTargetedPublishersRequestTargetingModeEnum {
    

    /// Placeholder for undefined targeting mode.
    ///
    /// "TARGETING_MODE_UNSPECIFIED"
    #[serde(rename="TARGETING_MODE_UNSPECIFIED")]
    TARGETINGMODEUNSPECIFIED,
    

    /// The inclusive list type. Inventory must match an item in this list to be targeted.
    ///
    /// "INCLUSIVE"
    #[serde(rename="INCLUSIVE")]
    INCLUSIVE,
    

    /// The exclusive list type. Inventory must not match any item in this list to be targeted.
    ///
    /// "EXCLUSIVE"
    #[serde(rename="EXCLUSIVE")]
    EXCLUSIVE,
}

impl AsRef<str> for AddTargetedPublishersRequestTargetingModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AddTargetedPublishersRequestTargetingModeEnum::TARGETINGMODEUNSPECIFIED => "TARGETING_MODE_UNSPECIFIED",
            AddTargetedPublishersRequestTargetingModeEnum::INCLUSIVE => "INCLUSIVE",
            AddTargetedPublishersRequestTargetingModeEnum::EXCLUSIVE => "EXCLUSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AddTargetedPublishersRequestTargetingModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_MODE_UNSPECIFIED" => Ok(AddTargetedPublishersRequestTargetingModeEnum::TARGETINGMODEUNSPECIFIED),
           "INCLUSIVE" => Ok(AddTargetedPublishersRequestTargetingModeEnum::INCLUSIVE),
           "EXCLUSIVE" => Ok(AddTargetedPublishersRequestTargetingModeEnum::EXCLUSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AddTargetedPublishersRequestTargetingModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AddTargetedSitesRequestTargetingModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The targeting mode that should be applied to the list of site URLs. If there are existing targeted sites, must be equal to the existing PretargetingConfig.webTargeting.targetingMode or a 400 bad request error will be returned.
pub enum AddTargetedSitesRequestTargetingModeEnum {
    

    /// Placeholder for undefined targeting mode.
    ///
    /// "TARGETING_MODE_UNSPECIFIED"
    #[serde(rename="TARGETING_MODE_UNSPECIFIED")]
    TARGETINGMODEUNSPECIFIED,
    

    /// The inclusive list type. Inventory must match an item in this list to be targeted.
    ///
    /// "INCLUSIVE"
    #[serde(rename="INCLUSIVE")]
    INCLUSIVE,
    

    /// The exclusive list type. Inventory must not match any item in this list to be targeted.
    ///
    /// "EXCLUSIVE"
    #[serde(rename="EXCLUSIVE")]
    EXCLUSIVE,
}

impl AsRef<str> for AddTargetedSitesRequestTargetingModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AddTargetedSitesRequestTargetingModeEnum::TARGETINGMODEUNSPECIFIED => "TARGETING_MODE_UNSPECIFIED",
            AddTargetedSitesRequestTargetingModeEnum::INCLUSIVE => "INCLUSIVE",
            AddTargetedSitesRequestTargetingModeEnum::EXCLUSIVE => "EXCLUSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for AddTargetedSitesRequestTargetingModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_MODE_UNSPECIFIED" => Ok(AddTargetedSitesRequestTargetingModeEnum::TARGETINGMODEUNSPECIFIED),
           "INCLUSIVE" => Ok(AddTargetedSitesRequestTargetingModeEnum::INCLUSIVE),
           "EXCLUSIVE" => Ok(AddTargetedSitesRequestTargetingModeEnum::EXCLUSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AddTargetedSitesRequestTargetingModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeCreativeFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The format of this creative. Can be used to filter the response of the creatives.list method.
pub enum CreativeCreativeFormatEnum {
    

    /// The format is unknown.
    ///
    /// "CREATIVE_FORMAT_UNSPECIFIED"
    #[serde(rename="CREATIVE_FORMAT_UNSPECIFIED")]
    CREATIVEFORMATUNSPECIFIED,
    

    /// HTML creative.
    ///
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
    

    /// Video creative.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// Native creative.
    ///
    /// "NATIVE"
    #[serde(rename="NATIVE")]
    NATIVE,
}

impl AsRef<str> for CreativeCreativeFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeCreativeFormatEnum::CREATIVEFORMATUNSPECIFIED => "CREATIVE_FORMAT_UNSPECIFIED",
            CreativeCreativeFormatEnum::HTML => "HTML",
            CreativeCreativeFormatEnum::VIDEO => "VIDEO",
            CreativeCreativeFormatEnum::NATIVE => "NATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeCreativeFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_FORMAT_UNSPECIFIED" => Ok(CreativeCreativeFormatEnum::CREATIVEFORMATUNSPECIFIED),
           "HTML" => Ok(CreativeCreativeFormatEnum::HTML),
           "VIDEO" => Ok(CreativeCreativeFormatEnum::VIDEO),
           "NATIVE" => Ok(CreativeCreativeFormatEnum::NATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeCreativeFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeDeclaredAttributesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All declared attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto") contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction.
pub enum CreativeDeclaredAttributesEnum {
    

    /// Do not use. This is a placeholder value only.
    ///
    /// "ATTRIBUTE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_UNSPECIFIED")]
    ATTRIBUTEUNSPECIFIED,
    

    /// The creative is of type image/rich media. For pretargeting.
    ///
    /// "IMAGE_RICH_MEDIA"
    #[serde(rename="IMAGE_RICH_MEDIA")]
    IMAGERICHMEDIA,
    

    /// The creative is of video type Adobe Flash FLV. For pretargeting.
    ///
    /// "ADOBE_FLASH_FLV"
    #[serde(rename="ADOBE_FLASH_FLV")]
    ADOBEFLASHFLV,
    

    /// The creative is tagged.
    ///
    /// "IS_TAGGED"
    #[serde(rename="IS_TAGGED")]
    ISTAGGED,
    

    /// The creative is cookie targeted.
    ///
    /// "IS_COOKIE_TARGETED"
    #[serde(rename="IS_COOKIE_TARGETED")]
    ISCOOKIETARGETED,
    

    /// The creative is user interest targeted.
    ///
    /// "IS_USER_INTEREST_TARGETED"
    #[serde(rename="IS_USER_INTEREST_TARGETED")]
    ISUSERINTERESTTARGETED,
    

    /// The creative does not expand.
    ///
    /// "EXPANDING_DIRECTION_NONE"
    #[serde(rename="EXPANDING_DIRECTION_NONE")]
    EXPANDINGDIRECTIONNONE,
    

    /// The creative expands up.
    ///
    /// "EXPANDING_DIRECTION_UP"
    #[serde(rename="EXPANDING_DIRECTION_UP")]
    EXPANDINGDIRECTIONUP,
    

    /// The creative expands down.
    ///
    /// "EXPANDING_DIRECTION_DOWN"
    #[serde(rename="EXPANDING_DIRECTION_DOWN")]
    EXPANDINGDIRECTIONDOWN,
    

    /// The creative expands left.
    ///
    /// "EXPANDING_DIRECTION_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_LEFT")]
    EXPANDINGDIRECTIONLEFT,
    

    /// The creative expands right.
    ///
    /// "EXPANDING_DIRECTION_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_RIGHT")]
    EXPANDINGDIRECTIONRIGHT,
    

    /// The creative expands up and left.
    ///
    /// "EXPANDING_DIRECTION_UP_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_UP_LEFT")]
    EXPANDINGDIRECTIONUPLEFT,
    

    /// The creative expands up and right.
    ///
    /// "EXPANDING_DIRECTION_UP_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_UP_RIGHT")]
    EXPANDINGDIRECTIONUPRIGHT,
    

    /// The creative expands down and left.
    ///
    /// "EXPANDING_DIRECTION_DOWN_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_DOWN_LEFT")]
    EXPANDINGDIRECTIONDOWNLEFT,
    

    /// The creative expands down and right.
    ///
    /// "EXPANDING_DIRECTION_DOWN_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_DOWN_RIGHT")]
    EXPANDINGDIRECTIONDOWNRIGHT,
    

    /// The creative type is HTML.
    ///
    /// "CREATIVE_TYPE_HTML"
    #[serde(rename="CREATIVE_TYPE_HTML")]
    CREATIVETYPEHTML,
    

    /// The creative type is VAST video.
    ///
    /// "CREATIVE_TYPE_VAST_VIDEO"
    #[serde(rename="CREATIVE_TYPE_VAST_VIDEO")]
    CREATIVETYPEVASTVIDEO,
    

    /// The creative expands up or down.
    ///
    /// "EXPANDING_DIRECTION_UP_OR_DOWN"
    #[serde(rename="EXPANDING_DIRECTION_UP_OR_DOWN")]
    EXPANDINGDIRECTIONUPORDOWN,
    

    /// The creative expands left or right.
    ///
    /// "EXPANDING_DIRECTION_LEFT_OR_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
    EXPANDINGDIRECTIONLEFTORRIGHT,
    

    /// The creative expands on any diagonal.
    ///
    /// "EXPANDING_DIRECTION_ANY_DIAGONAL"
    #[serde(rename="EXPANDING_DIRECTION_ANY_DIAGONAL")]
    EXPANDINGDIRECTIONANYDIAGONAL,
    

    /// The creative expands when rolled over.
    ///
    /// "EXPANDING_ACTION_ROLLOVER_TO_EXPAND"
    #[serde(rename="EXPANDING_ACTION_ROLLOVER_TO_EXPAND")]
    EXPANDINGACTIONROLLOVERTOEXPAND,
    

    /// The instream vast video type is vpaid flash.
    ///
    /// "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH"
    #[serde(rename="INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH")]
    INSTREAMVASTVIDEOTYPEVPAIDFLASH,
    

    /// The creative is MRAID.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_MRAID"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_MRAID")]
    RICHMEDIACAPABILITYTYPEMRAID,
    

    /// The creative is Flash.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_FLASH"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_FLASH")]
    RICHMEDIACAPABILITYTYPEFLASH,
    

    /// The creative is HTML5.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_HTML5"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_HTML5")]
    RICHMEDIACAPABILITYTYPEHTML5,
    

    /// The creative has an instream VAST video type of skippable instream video. For pretargeting.
    ///
    /// "SKIPPABLE_INSTREAM_VIDEO"
    #[serde(rename="SKIPPABLE_INSTREAM_VIDEO")]
    SKIPPABLEINSTREAMVIDEO,
    

    /// The creative is SSL.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_SSL"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_SSL")]
    RICHMEDIACAPABILITYTYPESSL,
    

    /// The creative is non-SSL.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_NON_SSL")]
    RICHMEDIACAPABILITYTYPENONSSL,
    

    /// The creative is an interstitial.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL")]
    RICHMEDIACAPABILITYTYPEINTERSTITIAL,
    

    /// The creative has an instream VAST video type of non-skippable instream video. For pretargeting.
    ///
    /// "NON_SKIPPABLE_INSTREAM_VIDEO"
    #[serde(rename="NON_SKIPPABLE_INSTREAM_VIDEO")]
    NONSKIPPABLEINSTREAMVIDEO,
    

    /// The creative is eligible for native.
    ///
    /// "NATIVE_ELIGIBILITY_ELIGIBLE"
    #[serde(rename="NATIVE_ELIGIBILITY_ELIGIBLE")]
    NATIVEELIGIBILITYELIGIBLE,
    

    /// The creative has an instream VAST video type of non-VPAID. For pretargeting.
    ///
    /// "NON_VPAID"
    #[serde(rename="NON_VPAID")]
    NONVPAID,
    

    /// The creative is not eligible for native.
    ///
    /// "NATIVE_ELIGIBILITY_NOT_ELIGIBLE"
    #[serde(rename="NATIVE_ELIGIBILITY_NOT_ELIGIBLE")]
    NATIVEELIGIBILITYNOTELIGIBLE,
    

    /// The creative has an interstitial size of any interstitial. For pretargeting.
    ///
    /// "ANY_INTERSTITIAL"
    #[serde(rename="ANY_INTERSTITIAL")]
    ANYINTERSTITIAL,
    

    /// The creative has an interstitial size of non interstitial. For pretargeting.
    ///
    /// "NON_INTERSTITIAL"
    #[serde(rename="NON_INTERSTITIAL")]
    NONINTERSTITIAL,
    

    /// The video type is in-banner video.
    ///
    /// "IN_BANNER_VIDEO"
    #[serde(rename="IN_BANNER_VIDEO")]
    INBANNERVIDEO,
    

    /// The creative can dynamically resize to fill a variety of slot sizes.
    ///
    /// "RENDERING_SIZELESS_ADX"
    #[serde(rename="RENDERING_SIZELESS_ADX")]
    RENDERINGSIZELESSADX,
    

    /// The open measurement SDK is supported.
    ///
    /// "OMSDK_1_0"
    #[serde(rename="OMSDK_1_0")]
    OMSDK10,
    

    /// The creative is considered a playable display creative.
    ///
    /// "RENDERING_PLAYABLE"
    #[serde(rename="RENDERING_PLAYABLE")]
    RENDERINGPLAYABLE,
}

impl AsRef<str> for CreativeDeclaredAttributesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeDeclaredAttributesEnum::ATTRIBUTEUNSPECIFIED => "ATTRIBUTE_UNSPECIFIED",
            CreativeDeclaredAttributesEnum::IMAGERICHMEDIA => "IMAGE_RICH_MEDIA",
            CreativeDeclaredAttributesEnum::ADOBEFLASHFLV => "ADOBE_FLASH_FLV",
            CreativeDeclaredAttributesEnum::ISTAGGED => "IS_TAGGED",
            CreativeDeclaredAttributesEnum::ISCOOKIETARGETED => "IS_COOKIE_TARGETED",
            CreativeDeclaredAttributesEnum::ISUSERINTERESTTARGETED => "IS_USER_INTEREST_TARGETED",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONNONE => "EXPANDING_DIRECTION_NONE",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUP => "EXPANDING_DIRECTION_UP",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONDOWN => "EXPANDING_DIRECTION_DOWN",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONLEFT => "EXPANDING_DIRECTION_LEFT",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONRIGHT => "EXPANDING_DIRECTION_RIGHT",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUPLEFT => "EXPANDING_DIRECTION_UP_LEFT",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUPRIGHT => "EXPANDING_DIRECTION_UP_RIGHT",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONDOWNLEFT => "EXPANDING_DIRECTION_DOWN_LEFT",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONDOWNRIGHT => "EXPANDING_DIRECTION_DOWN_RIGHT",
            CreativeDeclaredAttributesEnum::CREATIVETYPEHTML => "CREATIVE_TYPE_HTML",
            CreativeDeclaredAttributesEnum::CREATIVETYPEVASTVIDEO => "CREATIVE_TYPE_VAST_VIDEO",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUPORDOWN => "EXPANDING_DIRECTION_UP_OR_DOWN",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONLEFTORRIGHT => "EXPANDING_DIRECTION_LEFT_OR_RIGHT",
            CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONANYDIAGONAL => "EXPANDING_DIRECTION_ANY_DIAGONAL",
            CreativeDeclaredAttributesEnum::EXPANDINGACTIONROLLOVERTOEXPAND => "EXPANDING_ACTION_ROLLOVER_TO_EXPAND",
            CreativeDeclaredAttributesEnum::INSTREAMVASTVIDEOTYPEVPAIDFLASH => "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH",
            CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEMRAID => "RICH_MEDIA_CAPABILITY_TYPE_MRAID",
            CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEFLASH => "RICH_MEDIA_CAPABILITY_TYPE_FLASH",
            CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEHTML5 => "RICH_MEDIA_CAPABILITY_TYPE_HTML5",
            CreativeDeclaredAttributesEnum::SKIPPABLEINSTREAMVIDEO => "SKIPPABLE_INSTREAM_VIDEO",
            CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPESSL => "RICH_MEDIA_CAPABILITY_TYPE_SSL",
            CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPENONSSL => "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL",
            CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEINTERSTITIAL => "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL",
            CreativeDeclaredAttributesEnum::NONSKIPPABLEINSTREAMVIDEO => "NON_SKIPPABLE_INSTREAM_VIDEO",
            CreativeDeclaredAttributesEnum::NATIVEELIGIBILITYELIGIBLE => "NATIVE_ELIGIBILITY_ELIGIBLE",
            CreativeDeclaredAttributesEnum::NONVPAID => "NON_VPAID",
            CreativeDeclaredAttributesEnum::NATIVEELIGIBILITYNOTELIGIBLE => "NATIVE_ELIGIBILITY_NOT_ELIGIBLE",
            CreativeDeclaredAttributesEnum::ANYINTERSTITIAL => "ANY_INTERSTITIAL",
            CreativeDeclaredAttributesEnum::NONINTERSTITIAL => "NON_INTERSTITIAL",
            CreativeDeclaredAttributesEnum::INBANNERVIDEO => "IN_BANNER_VIDEO",
            CreativeDeclaredAttributesEnum::RENDERINGSIZELESSADX => "RENDERING_SIZELESS_ADX",
            CreativeDeclaredAttributesEnum::OMSDK10 => "OMSDK_1_0",
            CreativeDeclaredAttributesEnum::RENDERINGPLAYABLE => "RENDERING_PLAYABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeDeclaredAttributesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_UNSPECIFIED" => Ok(CreativeDeclaredAttributesEnum::ATTRIBUTEUNSPECIFIED),
           "IMAGE_RICH_MEDIA" => Ok(CreativeDeclaredAttributesEnum::IMAGERICHMEDIA),
           "ADOBE_FLASH_FLV" => Ok(CreativeDeclaredAttributesEnum::ADOBEFLASHFLV),
           "IS_TAGGED" => Ok(CreativeDeclaredAttributesEnum::ISTAGGED),
           "IS_COOKIE_TARGETED" => Ok(CreativeDeclaredAttributesEnum::ISCOOKIETARGETED),
           "IS_USER_INTEREST_TARGETED" => Ok(CreativeDeclaredAttributesEnum::ISUSERINTERESTTARGETED),
           "EXPANDING_DIRECTION_NONE" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONNONE),
           "EXPANDING_DIRECTION_UP" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUP),
           "EXPANDING_DIRECTION_DOWN" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONDOWN),
           "EXPANDING_DIRECTION_LEFT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONLEFT),
           "EXPANDING_DIRECTION_RIGHT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONRIGHT),
           "EXPANDING_DIRECTION_UP_LEFT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUPLEFT),
           "EXPANDING_DIRECTION_UP_RIGHT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUPRIGHT),
           "EXPANDING_DIRECTION_DOWN_LEFT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONDOWNLEFT),
           "EXPANDING_DIRECTION_DOWN_RIGHT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONDOWNRIGHT),
           "CREATIVE_TYPE_HTML" => Ok(CreativeDeclaredAttributesEnum::CREATIVETYPEHTML),
           "CREATIVE_TYPE_VAST_VIDEO" => Ok(CreativeDeclaredAttributesEnum::CREATIVETYPEVASTVIDEO),
           "EXPANDING_DIRECTION_UP_OR_DOWN" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONUPORDOWN),
           "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONLEFTORRIGHT),
           "EXPANDING_DIRECTION_ANY_DIAGONAL" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGDIRECTIONANYDIAGONAL),
           "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => Ok(CreativeDeclaredAttributesEnum::EXPANDINGACTIONROLLOVERTOEXPAND),
           "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => Ok(CreativeDeclaredAttributesEnum::INSTREAMVASTVIDEOTYPEVPAIDFLASH),
           "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => Ok(CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEMRAID),
           "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => Ok(CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEFLASH),
           "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => Ok(CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEHTML5),
           "SKIPPABLE_INSTREAM_VIDEO" => Ok(CreativeDeclaredAttributesEnum::SKIPPABLEINSTREAMVIDEO),
           "RICH_MEDIA_CAPABILITY_TYPE_SSL" => Ok(CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPESSL),
           "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => Ok(CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPENONSSL),
           "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => Ok(CreativeDeclaredAttributesEnum::RICHMEDIACAPABILITYTYPEINTERSTITIAL),
           "NON_SKIPPABLE_INSTREAM_VIDEO" => Ok(CreativeDeclaredAttributesEnum::NONSKIPPABLEINSTREAMVIDEO),
           "NATIVE_ELIGIBILITY_ELIGIBLE" => Ok(CreativeDeclaredAttributesEnum::NATIVEELIGIBILITYELIGIBLE),
           "NON_VPAID" => Ok(CreativeDeclaredAttributesEnum::NONVPAID),
           "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => Ok(CreativeDeclaredAttributesEnum::NATIVEELIGIBILITYNOTELIGIBLE),
           "ANY_INTERSTITIAL" => Ok(CreativeDeclaredAttributesEnum::ANYINTERSTITIAL),
           "NON_INTERSTITIAL" => Ok(CreativeDeclaredAttributesEnum::NONINTERSTITIAL),
           "IN_BANNER_VIDEO" => Ok(CreativeDeclaredAttributesEnum::INBANNERVIDEO),
           "RENDERING_SIZELESS_ADX" => Ok(CreativeDeclaredAttributesEnum::RENDERINGSIZELESSADX),
           "OMSDK_1_0" => Ok(CreativeDeclaredAttributesEnum::OMSDK10),
           "RENDERING_PLAYABLE" => Ok(CreativeDeclaredAttributesEnum::RENDERINGPLAYABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeDeclaredAttributesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeDeclaredRestrictedCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All declared restricted categories for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method.
pub enum CreativeDeclaredRestrictedCategoriesEnum {
    

    /// Default value that should never be used.
    ///
    /// "RESTRICTED_CATEGORY_UNSPECIFIED"
    #[serde(rename="RESTRICTED_CATEGORY_UNSPECIFIED")]
    RESTRICTEDCATEGORYUNSPECIFIED,
    

    /// The alcohol restricted category.
    ///
    /// "ALCOHOL"
    #[serde(rename="ALCOHOL")]
    ALCOHOL,
}

impl AsRef<str> for CreativeDeclaredRestrictedCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeDeclaredRestrictedCategoriesEnum::RESTRICTEDCATEGORYUNSPECIFIED => "RESTRICTED_CATEGORY_UNSPECIFIED",
            CreativeDeclaredRestrictedCategoriesEnum::ALCOHOL => "ALCOHOL",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeDeclaredRestrictedCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTRICTED_CATEGORY_UNSPECIFIED" => Ok(CreativeDeclaredRestrictedCategoriesEnum::RESTRICTEDCATEGORYUNSPECIFIED),
           "ALCOHOL" => Ok(CreativeDeclaredRestrictedCategoriesEnum::ALCOHOL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeDeclaredRestrictedCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRestrictedCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All restricted categories for the ads that may be shown from this creative.
pub enum CreativeRestrictedCategoriesEnum {
    

    /// Default value that should never be used.
    ///
    /// "RESTRICTED_CATEGORY_UNSPECIFIED"
    #[serde(rename="RESTRICTED_CATEGORY_UNSPECIFIED")]
    RESTRICTEDCATEGORYUNSPECIFIED,
    

    /// The alcohol restricted category.
    ///
    /// "ALCOHOL"
    #[serde(rename="ALCOHOL")]
    ALCOHOL,
}

impl AsRef<str> for CreativeRestrictedCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRestrictedCategoriesEnum::RESTRICTEDCATEGORYUNSPECIFIED => "RESTRICTED_CATEGORY_UNSPECIFIED",
            CreativeRestrictedCategoriesEnum::ALCOHOL => "ALCOHOL",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRestrictedCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTRICTED_CATEGORY_UNSPECIFIED" => Ok(CreativeRestrictedCategoriesEnum::RESTRICTEDCATEGORYUNSPECIFIED),
           "ALCOHOL" => Ok(CreativeRestrictedCategoriesEnum::ALCOHOL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRestrictedCategoriesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeServingDecisionDetectedAttributesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Publisher-excludable attributes that were detected for this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction.
pub enum CreativeServingDecisionDetectedAttributesEnum {
    

    /// Do not use. This is a placeholder value only.
    ///
    /// "ATTRIBUTE_UNSPECIFIED"
    #[serde(rename="ATTRIBUTE_UNSPECIFIED")]
    ATTRIBUTEUNSPECIFIED,
    

    /// The creative is of type image/rich media. For pretargeting.
    ///
    /// "IMAGE_RICH_MEDIA"
    #[serde(rename="IMAGE_RICH_MEDIA")]
    IMAGERICHMEDIA,
    

    /// The creative is of video type Adobe Flash FLV. For pretargeting.
    ///
    /// "ADOBE_FLASH_FLV"
    #[serde(rename="ADOBE_FLASH_FLV")]
    ADOBEFLASHFLV,
    

    /// The creative is tagged.
    ///
    /// "IS_TAGGED"
    #[serde(rename="IS_TAGGED")]
    ISTAGGED,
    

    /// The creative is cookie targeted.
    ///
    /// "IS_COOKIE_TARGETED"
    #[serde(rename="IS_COOKIE_TARGETED")]
    ISCOOKIETARGETED,
    

    /// The creative is user interest targeted.
    ///
    /// "IS_USER_INTEREST_TARGETED"
    #[serde(rename="IS_USER_INTEREST_TARGETED")]
    ISUSERINTERESTTARGETED,
    

    /// The creative does not expand.
    ///
    /// "EXPANDING_DIRECTION_NONE"
    #[serde(rename="EXPANDING_DIRECTION_NONE")]
    EXPANDINGDIRECTIONNONE,
    

    /// The creative expands up.
    ///
    /// "EXPANDING_DIRECTION_UP"
    #[serde(rename="EXPANDING_DIRECTION_UP")]
    EXPANDINGDIRECTIONUP,
    

    /// The creative expands down.
    ///
    /// "EXPANDING_DIRECTION_DOWN"
    #[serde(rename="EXPANDING_DIRECTION_DOWN")]
    EXPANDINGDIRECTIONDOWN,
    

    /// The creative expands left.
    ///
    /// "EXPANDING_DIRECTION_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_LEFT")]
    EXPANDINGDIRECTIONLEFT,
    

    /// The creative expands right.
    ///
    /// "EXPANDING_DIRECTION_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_RIGHT")]
    EXPANDINGDIRECTIONRIGHT,
    

    /// The creative expands up and left.
    ///
    /// "EXPANDING_DIRECTION_UP_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_UP_LEFT")]
    EXPANDINGDIRECTIONUPLEFT,
    

    /// The creative expands up and right.
    ///
    /// "EXPANDING_DIRECTION_UP_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_UP_RIGHT")]
    EXPANDINGDIRECTIONUPRIGHT,
    

    /// The creative expands down and left.
    ///
    /// "EXPANDING_DIRECTION_DOWN_LEFT"
    #[serde(rename="EXPANDING_DIRECTION_DOWN_LEFT")]
    EXPANDINGDIRECTIONDOWNLEFT,
    

    /// The creative expands down and right.
    ///
    /// "EXPANDING_DIRECTION_DOWN_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_DOWN_RIGHT")]
    EXPANDINGDIRECTIONDOWNRIGHT,
    

    /// The creative type is HTML.
    ///
    /// "CREATIVE_TYPE_HTML"
    #[serde(rename="CREATIVE_TYPE_HTML")]
    CREATIVETYPEHTML,
    

    /// The creative type is VAST video.
    ///
    /// "CREATIVE_TYPE_VAST_VIDEO"
    #[serde(rename="CREATIVE_TYPE_VAST_VIDEO")]
    CREATIVETYPEVASTVIDEO,
    

    /// The creative expands up or down.
    ///
    /// "EXPANDING_DIRECTION_UP_OR_DOWN"
    #[serde(rename="EXPANDING_DIRECTION_UP_OR_DOWN")]
    EXPANDINGDIRECTIONUPORDOWN,
    

    /// The creative expands left or right.
    ///
    /// "EXPANDING_DIRECTION_LEFT_OR_RIGHT"
    #[serde(rename="EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
    EXPANDINGDIRECTIONLEFTORRIGHT,
    

    /// The creative expands on any diagonal.
    ///
    /// "EXPANDING_DIRECTION_ANY_DIAGONAL"
    #[serde(rename="EXPANDING_DIRECTION_ANY_DIAGONAL")]
    EXPANDINGDIRECTIONANYDIAGONAL,
    

    /// The creative expands when rolled over.
    ///
    /// "EXPANDING_ACTION_ROLLOVER_TO_EXPAND"
    #[serde(rename="EXPANDING_ACTION_ROLLOVER_TO_EXPAND")]
    EXPANDINGACTIONROLLOVERTOEXPAND,
    

    /// The instream vast video type is vpaid flash.
    ///
    /// "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH"
    #[serde(rename="INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH")]
    INSTREAMVASTVIDEOTYPEVPAIDFLASH,
    

    /// The creative is MRAID.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_MRAID"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_MRAID")]
    RICHMEDIACAPABILITYTYPEMRAID,
    

    /// The creative is Flash.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_FLASH"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_FLASH")]
    RICHMEDIACAPABILITYTYPEFLASH,
    

    /// The creative is HTML5.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_HTML5"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_HTML5")]
    RICHMEDIACAPABILITYTYPEHTML5,
    

    /// The creative has an instream VAST video type of skippable instream video. For pretargeting.
    ///
    /// "SKIPPABLE_INSTREAM_VIDEO"
    #[serde(rename="SKIPPABLE_INSTREAM_VIDEO")]
    SKIPPABLEINSTREAMVIDEO,
    

    /// The creative is SSL.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_SSL"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_SSL")]
    RICHMEDIACAPABILITYTYPESSL,
    

    /// The creative is non-SSL.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_NON_SSL")]
    RICHMEDIACAPABILITYTYPENONSSL,
    

    /// The creative is an interstitial.
    ///
    /// "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL"
    #[serde(rename="RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL")]
    RICHMEDIACAPABILITYTYPEINTERSTITIAL,
    

    /// The creative has an instream VAST video type of non-skippable instream video. For pretargeting.
    ///
    /// "NON_SKIPPABLE_INSTREAM_VIDEO"
    #[serde(rename="NON_SKIPPABLE_INSTREAM_VIDEO")]
    NONSKIPPABLEINSTREAMVIDEO,
    

    /// The creative is eligible for native.
    ///
    /// "NATIVE_ELIGIBILITY_ELIGIBLE"
    #[serde(rename="NATIVE_ELIGIBILITY_ELIGIBLE")]
    NATIVEELIGIBILITYELIGIBLE,
    

    /// The creative has an instream VAST video type of non-VPAID. For pretargeting.
    ///
    /// "NON_VPAID"
    #[serde(rename="NON_VPAID")]
    NONVPAID,
    

    /// The creative is not eligible for native.
    ///
    /// "NATIVE_ELIGIBILITY_NOT_ELIGIBLE"
    #[serde(rename="NATIVE_ELIGIBILITY_NOT_ELIGIBLE")]
    NATIVEELIGIBILITYNOTELIGIBLE,
    

    /// The creative has an interstitial size of any interstitial. For pretargeting.
    ///
    /// "ANY_INTERSTITIAL"
    #[serde(rename="ANY_INTERSTITIAL")]
    ANYINTERSTITIAL,
    

    /// The creative has an interstitial size of non interstitial. For pretargeting.
    ///
    /// "NON_INTERSTITIAL"
    #[serde(rename="NON_INTERSTITIAL")]
    NONINTERSTITIAL,
    

    /// The video type is in-banner video.
    ///
    /// "IN_BANNER_VIDEO"
    #[serde(rename="IN_BANNER_VIDEO")]
    INBANNERVIDEO,
    

    /// The creative can dynamically resize to fill a variety of slot sizes.
    ///
    /// "RENDERING_SIZELESS_ADX"
    #[serde(rename="RENDERING_SIZELESS_ADX")]
    RENDERINGSIZELESSADX,
    

    /// The open measurement SDK is supported.
    ///
    /// "OMSDK_1_0"
    #[serde(rename="OMSDK_1_0")]
    OMSDK10,
    

    /// The creative is considered a playable display creative.
    ///
    /// "RENDERING_PLAYABLE"
    #[serde(rename="RENDERING_PLAYABLE")]
    RENDERINGPLAYABLE,
}

impl AsRef<str> for CreativeServingDecisionDetectedAttributesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeServingDecisionDetectedAttributesEnum::ATTRIBUTEUNSPECIFIED => "ATTRIBUTE_UNSPECIFIED",
            CreativeServingDecisionDetectedAttributesEnum::IMAGERICHMEDIA => "IMAGE_RICH_MEDIA",
            CreativeServingDecisionDetectedAttributesEnum::ADOBEFLASHFLV => "ADOBE_FLASH_FLV",
            CreativeServingDecisionDetectedAttributesEnum::ISTAGGED => "IS_TAGGED",
            CreativeServingDecisionDetectedAttributesEnum::ISCOOKIETARGETED => "IS_COOKIE_TARGETED",
            CreativeServingDecisionDetectedAttributesEnum::ISUSERINTERESTTARGETED => "IS_USER_INTEREST_TARGETED",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONNONE => "EXPANDING_DIRECTION_NONE",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUP => "EXPANDING_DIRECTION_UP",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONDOWN => "EXPANDING_DIRECTION_DOWN",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONLEFT => "EXPANDING_DIRECTION_LEFT",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONRIGHT => "EXPANDING_DIRECTION_RIGHT",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUPLEFT => "EXPANDING_DIRECTION_UP_LEFT",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUPRIGHT => "EXPANDING_DIRECTION_UP_RIGHT",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONDOWNLEFT => "EXPANDING_DIRECTION_DOWN_LEFT",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONDOWNRIGHT => "EXPANDING_DIRECTION_DOWN_RIGHT",
            CreativeServingDecisionDetectedAttributesEnum::CREATIVETYPEHTML => "CREATIVE_TYPE_HTML",
            CreativeServingDecisionDetectedAttributesEnum::CREATIVETYPEVASTVIDEO => "CREATIVE_TYPE_VAST_VIDEO",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUPORDOWN => "EXPANDING_DIRECTION_UP_OR_DOWN",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONLEFTORRIGHT => "EXPANDING_DIRECTION_LEFT_OR_RIGHT",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONANYDIAGONAL => "EXPANDING_DIRECTION_ANY_DIAGONAL",
            CreativeServingDecisionDetectedAttributesEnum::EXPANDINGACTIONROLLOVERTOEXPAND => "EXPANDING_ACTION_ROLLOVER_TO_EXPAND",
            CreativeServingDecisionDetectedAttributesEnum::INSTREAMVASTVIDEOTYPEVPAIDFLASH => "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH",
            CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEMRAID => "RICH_MEDIA_CAPABILITY_TYPE_MRAID",
            CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEFLASH => "RICH_MEDIA_CAPABILITY_TYPE_FLASH",
            CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEHTML5 => "RICH_MEDIA_CAPABILITY_TYPE_HTML5",
            CreativeServingDecisionDetectedAttributesEnum::SKIPPABLEINSTREAMVIDEO => "SKIPPABLE_INSTREAM_VIDEO",
            CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPESSL => "RICH_MEDIA_CAPABILITY_TYPE_SSL",
            CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPENONSSL => "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL",
            CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEINTERSTITIAL => "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL",
            CreativeServingDecisionDetectedAttributesEnum::NONSKIPPABLEINSTREAMVIDEO => "NON_SKIPPABLE_INSTREAM_VIDEO",
            CreativeServingDecisionDetectedAttributesEnum::NATIVEELIGIBILITYELIGIBLE => "NATIVE_ELIGIBILITY_ELIGIBLE",
            CreativeServingDecisionDetectedAttributesEnum::NONVPAID => "NON_VPAID",
            CreativeServingDecisionDetectedAttributesEnum::NATIVEELIGIBILITYNOTELIGIBLE => "NATIVE_ELIGIBILITY_NOT_ELIGIBLE",
            CreativeServingDecisionDetectedAttributesEnum::ANYINTERSTITIAL => "ANY_INTERSTITIAL",
            CreativeServingDecisionDetectedAttributesEnum::NONINTERSTITIAL => "NON_INTERSTITIAL",
            CreativeServingDecisionDetectedAttributesEnum::INBANNERVIDEO => "IN_BANNER_VIDEO",
            CreativeServingDecisionDetectedAttributesEnum::RENDERINGSIZELESSADX => "RENDERING_SIZELESS_ADX",
            CreativeServingDecisionDetectedAttributesEnum::OMSDK10 => "OMSDK_1_0",
            CreativeServingDecisionDetectedAttributesEnum::RENDERINGPLAYABLE => "RENDERING_PLAYABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeServingDecisionDetectedAttributesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_UNSPECIFIED" => Ok(CreativeServingDecisionDetectedAttributesEnum::ATTRIBUTEUNSPECIFIED),
           "IMAGE_RICH_MEDIA" => Ok(CreativeServingDecisionDetectedAttributesEnum::IMAGERICHMEDIA),
           "ADOBE_FLASH_FLV" => Ok(CreativeServingDecisionDetectedAttributesEnum::ADOBEFLASHFLV),
           "IS_TAGGED" => Ok(CreativeServingDecisionDetectedAttributesEnum::ISTAGGED),
           "IS_COOKIE_TARGETED" => Ok(CreativeServingDecisionDetectedAttributesEnum::ISCOOKIETARGETED),
           "IS_USER_INTEREST_TARGETED" => Ok(CreativeServingDecisionDetectedAttributesEnum::ISUSERINTERESTTARGETED),
           "EXPANDING_DIRECTION_NONE" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONNONE),
           "EXPANDING_DIRECTION_UP" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUP),
           "EXPANDING_DIRECTION_DOWN" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONDOWN),
           "EXPANDING_DIRECTION_LEFT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONLEFT),
           "EXPANDING_DIRECTION_RIGHT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONRIGHT),
           "EXPANDING_DIRECTION_UP_LEFT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUPLEFT),
           "EXPANDING_DIRECTION_UP_RIGHT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUPRIGHT),
           "EXPANDING_DIRECTION_DOWN_LEFT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONDOWNLEFT),
           "EXPANDING_DIRECTION_DOWN_RIGHT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONDOWNRIGHT),
           "CREATIVE_TYPE_HTML" => Ok(CreativeServingDecisionDetectedAttributesEnum::CREATIVETYPEHTML),
           "CREATIVE_TYPE_VAST_VIDEO" => Ok(CreativeServingDecisionDetectedAttributesEnum::CREATIVETYPEVASTVIDEO),
           "EXPANDING_DIRECTION_UP_OR_DOWN" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONUPORDOWN),
           "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONLEFTORRIGHT),
           "EXPANDING_DIRECTION_ANY_DIAGONAL" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGDIRECTIONANYDIAGONAL),
           "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => Ok(CreativeServingDecisionDetectedAttributesEnum::EXPANDINGACTIONROLLOVERTOEXPAND),
           "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => Ok(CreativeServingDecisionDetectedAttributesEnum::INSTREAMVASTVIDEOTYPEVPAIDFLASH),
           "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => Ok(CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEMRAID),
           "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => Ok(CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEFLASH),
           "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => Ok(CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEHTML5),
           "SKIPPABLE_INSTREAM_VIDEO" => Ok(CreativeServingDecisionDetectedAttributesEnum::SKIPPABLEINSTREAMVIDEO),
           "RICH_MEDIA_CAPABILITY_TYPE_SSL" => Ok(CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPESSL),
           "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => Ok(CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPENONSSL),
           "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => Ok(CreativeServingDecisionDetectedAttributesEnum::RICHMEDIACAPABILITYTYPEINTERSTITIAL),
           "NON_SKIPPABLE_INSTREAM_VIDEO" => Ok(CreativeServingDecisionDetectedAttributesEnum::NONSKIPPABLEINSTREAMVIDEO),
           "NATIVE_ELIGIBILITY_ELIGIBLE" => Ok(CreativeServingDecisionDetectedAttributesEnum::NATIVEELIGIBILITYELIGIBLE),
           "NON_VPAID" => Ok(CreativeServingDecisionDetectedAttributesEnum::NONVPAID),
           "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => Ok(CreativeServingDecisionDetectedAttributesEnum::NATIVEELIGIBILITYNOTELIGIBLE),
           "ANY_INTERSTITIAL" => Ok(CreativeServingDecisionDetectedAttributesEnum::ANYINTERSTITIAL),
           "NON_INTERSTITIAL" => Ok(CreativeServingDecisionDetectedAttributesEnum::NONINTERSTITIAL),
           "IN_BANNER_VIDEO" => Ok(CreativeServingDecisionDetectedAttributesEnum::INBANNERVIDEO),
           "RENDERING_SIZELESS_ADX" => Ok(CreativeServingDecisionDetectedAttributesEnum::RENDERINGSIZELESSADX),
           "OMSDK_1_0" => Ok(CreativeServingDecisionDetectedAttributesEnum::OMSDK10),
           "RENDERING_PLAYABLE" => Ok(CreativeServingDecisionDetectedAttributesEnum::RENDERINGPLAYABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeServingDecisionDetectedAttributesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationNotCrawlableEvidenceReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reason of destination not crawlable.
pub enum DestinationNotCrawlableEvidenceReasonEnum {
    

    /// Default value that should never be used.
    ///
    /// "REASON_UNSPECIFIED"
    #[serde(rename="REASON_UNSPECIFIED")]
    REASONUNSPECIFIED,
    

    /// Site's robots exclusion file (for example, robots.txt) was unreachable.
    ///
    /// "UNREACHABLE_ROBOTS"
    #[serde(rename="UNREACHABLE_ROBOTS")]
    UNREACHABLEROBOTS,
    

    /// Timed out reading site's robots exclusion file (for example, robots.txt).
    ///
    /// "TIMEOUT_ROBOTS"
    #[serde(rename="TIMEOUT_ROBOTS")]
    TIMEOUTROBOTS,
    

    /// Crawler was disallowed by the site's robots exclusion file (for example, robots.txt).
    ///
    /// "ROBOTED_DENIED"
    #[serde(rename="ROBOTED_DENIED")]
    ROBOTEDDENIED,
    

    /// Unknown reason.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
}

impl AsRef<str> for DestinationNotCrawlableEvidenceReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationNotCrawlableEvidenceReasonEnum::REASONUNSPECIFIED => "REASON_UNSPECIFIED",
            DestinationNotCrawlableEvidenceReasonEnum::UNREACHABLEROBOTS => "UNREACHABLE_ROBOTS",
            DestinationNotCrawlableEvidenceReasonEnum::TIMEOUTROBOTS => "TIMEOUT_ROBOTS",
            DestinationNotCrawlableEvidenceReasonEnum::ROBOTEDDENIED => "ROBOTED_DENIED",
            DestinationNotCrawlableEvidenceReasonEnum::UNKNOWN => "UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationNotCrawlableEvidenceReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REASON_UNSPECIFIED" => Ok(DestinationNotCrawlableEvidenceReasonEnum::REASONUNSPECIFIED),
           "UNREACHABLE_ROBOTS" => Ok(DestinationNotCrawlableEvidenceReasonEnum::UNREACHABLEROBOTS),
           "TIMEOUT_ROBOTS" => Ok(DestinationNotCrawlableEvidenceReasonEnum::TIMEOUTROBOTS),
           "ROBOTED_DENIED" => Ok(DestinationNotCrawlableEvidenceReasonEnum::ROBOTEDDENIED),
           "UNKNOWN" => Ok(DestinationNotCrawlableEvidenceReasonEnum::UNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationNotCrawlableEvidenceReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationNotWorkingEvidenceDnsErrorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// DNS lookup errors.
pub enum DestinationNotWorkingEvidenceDnsErrorEnum {
    

    /// Default value that should never be used.
    ///
    /// "DNS_ERROR_UNSPECIFIED"
    #[serde(rename="DNS_ERROR_UNSPECIFIED")]
    DNSERRORUNSPECIFIED,
    

    /// DNS name was not found.
    ///
    /// "ERROR_DNS"
    #[serde(rename="ERROR_DNS")]
    ERRORDNS,
    

    /// An internal issue occurred when Google's crawler tried to resolve the DNS entry. This is a Google-internal issue and may not be the result of an issue with the landing page.
    ///
    /// "GOOGLE_CRAWLER_DNS_ISSUE"
    #[serde(rename="GOOGLE_CRAWLER_DNS_ISSUE")]
    GOOGLECRAWLERDNSISSUE,
}

impl AsRef<str> for DestinationNotWorkingEvidenceDnsErrorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationNotWorkingEvidenceDnsErrorEnum::DNSERRORUNSPECIFIED => "DNS_ERROR_UNSPECIFIED",
            DestinationNotWorkingEvidenceDnsErrorEnum::ERRORDNS => "ERROR_DNS",
            DestinationNotWorkingEvidenceDnsErrorEnum::GOOGLECRAWLERDNSISSUE => "GOOGLE_CRAWLER_DNS_ISSUE",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationNotWorkingEvidenceDnsErrorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DNS_ERROR_UNSPECIFIED" => Ok(DestinationNotWorkingEvidenceDnsErrorEnum::DNSERRORUNSPECIFIED),
           "ERROR_DNS" => Ok(DestinationNotWorkingEvidenceDnsErrorEnum::ERRORDNS),
           "GOOGLE_CRAWLER_DNS_ISSUE" => Ok(DestinationNotWorkingEvidenceDnsErrorEnum::GOOGLECRAWLERDNSISSUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationNotWorkingEvidenceDnsErrorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationNotWorkingEvidenceInvalidPageEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Page was crawled successfully, but was detected as either a page with no content or an error page.
pub enum DestinationNotWorkingEvidenceInvalidPageEnum {
    

    /// Default value that should never be used.
    ///
    /// "INVALID_PAGE_UNSPECIFIED"
    #[serde(rename="INVALID_PAGE_UNSPECIFIED")]
    INVALIDPAGEUNSPECIFIED,
    

    /// Page was empty or had an error.
    ///
    /// "EMPTY_OR_ERROR_PAGE"
    #[serde(rename="EMPTY_OR_ERROR_PAGE")]
    EMPTYORERRORPAGE,
}

impl AsRef<str> for DestinationNotWorkingEvidenceInvalidPageEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationNotWorkingEvidenceInvalidPageEnum::INVALIDPAGEUNSPECIFIED => "INVALID_PAGE_UNSPECIFIED",
            DestinationNotWorkingEvidenceInvalidPageEnum::EMPTYORERRORPAGE => "EMPTY_OR_ERROR_PAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationNotWorkingEvidenceInvalidPageEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INVALID_PAGE_UNSPECIFIED" => Ok(DestinationNotWorkingEvidenceInvalidPageEnum::INVALIDPAGEUNSPECIFIED),
           "EMPTY_OR_ERROR_PAGE" => Ok(DestinationNotWorkingEvidenceInvalidPageEnum::EMPTYORERRORPAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationNotWorkingEvidenceInvalidPageEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationNotWorkingEvidencePlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Platform of the non-working URL.
pub enum DestinationNotWorkingEvidencePlatformEnum {
    

    /// Default value that should never be used.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// The personal computer platform.
    ///
    /// "PERSONAL_COMPUTER"
    #[serde(rename="PERSONAL_COMPUTER")]
    PERSONALCOMPUTER,
    

    /// The Android platform.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// The iOS platform.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
}

impl AsRef<str> for DestinationNotWorkingEvidencePlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationNotWorkingEvidencePlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            DestinationNotWorkingEvidencePlatformEnum::PERSONALCOMPUTER => "PERSONAL_COMPUTER",
            DestinationNotWorkingEvidencePlatformEnum::ANDROID => "ANDROID",
            DestinationNotWorkingEvidencePlatformEnum::IOS => "IOS",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationNotWorkingEvidencePlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(DestinationNotWorkingEvidencePlatformEnum::PLATFORMUNSPECIFIED),
           "PERSONAL_COMPUTER" => Ok(DestinationNotWorkingEvidencePlatformEnum::PERSONALCOMPUTER),
           "ANDROID" => Ok(DestinationNotWorkingEvidencePlatformEnum::ANDROID),
           "IOS" => Ok(DestinationNotWorkingEvidencePlatformEnum::IOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationNotWorkingEvidencePlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationNotWorkingEvidenceRedirectionErrorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// HTTP redirect chain error.
pub enum DestinationNotWorkingEvidenceRedirectionErrorEnum {
    

    /// Default value that should never be used.
    ///
    /// "REDIRECTION_ERROR_UNSPECIFIED"
    #[serde(rename="REDIRECTION_ERROR_UNSPECIFIED")]
    REDIRECTIONERRORUNSPECIFIED,
    

    /// Too many redirect hops.
    ///
    /// "TOO_MANY_REDIRECTS"
    #[serde(rename="TOO_MANY_REDIRECTS")]
    TOOMANYREDIRECTS,
    

    /// Got a redirect but it was invalid.
    ///
    /// "INVALID_REDIRECT"
    #[serde(rename="INVALID_REDIRECT")]
    INVALIDREDIRECT,
    

    /// Got a redirect but it was empty.
    ///
    /// "EMPTY_REDIRECT"
    #[serde(rename="EMPTY_REDIRECT")]
    EMPTYREDIRECT,
    

    /// Unknown redirect error.
    ///
    /// "REDIRECT_ERROR_UNKNOWN"
    #[serde(rename="REDIRECT_ERROR_UNKNOWN")]
    REDIRECTERRORUNKNOWN,
}

impl AsRef<str> for DestinationNotWorkingEvidenceRedirectionErrorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationNotWorkingEvidenceRedirectionErrorEnum::REDIRECTIONERRORUNSPECIFIED => "REDIRECTION_ERROR_UNSPECIFIED",
            DestinationNotWorkingEvidenceRedirectionErrorEnum::TOOMANYREDIRECTS => "TOO_MANY_REDIRECTS",
            DestinationNotWorkingEvidenceRedirectionErrorEnum::INVALIDREDIRECT => "INVALID_REDIRECT",
            DestinationNotWorkingEvidenceRedirectionErrorEnum::EMPTYREDIRECT => "EMPTY_REDIRECT",
            DestinationNotWorkingEvidenceRedirectionErrorEnum::REDIRECTERRORUNKNOWN => "REDIRECT_ERROR_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationNotWorkingEvidenceRedirectionErrorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDIRECTION_ERROR_UNSPECIFIED" => Ok(DestinationNotWorkingEvidenceRedirectionErrorEnum::REDIRECTIONERRORUNSPECIFIED),
           "TOO_MANY_REDIRECTS" => Ok(DestinationNotWorkingEvidenceRedirectionErrorEnum::TOOMANYREDIRECTS),
           "INVALID_REDIRECT" => Ok(DestinationNotWorkingEvidenceRedirectionErrorEnum::INVALIDREDIRECT),
           "EMPTY_REDIRECT" => Ok(DestinationNotWorkingEvidenceRedirectionErrorEnum::EMPTYREDIRECT),
           "REDIRECT_ERROR_UNKNOWN" => Ok(DestinationNotWorkingEvidenceRedirectionErrorEnum::REDIRECTERRORUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationNotWorkingEvidenceRedirectionErrorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DestinationNotWorkingEvidenceUrlRejectedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rejected because of malformed URLs or invalid requests.
pub enum DestinationNotWorkingEvidenceUrlRejectedEnum {
    

    /// Default value that should never be used.
    ///
    /// "URL_REJECTED_UNSPECIFIED"
    #[serde(rename="URL_REJECTED_UNSPECIFIED")]
    URLREJECTEDUNSPECIFIED,
    

    /// URL rejected because of a malformed request.
    ///
    /// "BAD_REQUEST"
    #[serde(rename="BAD_REQUEST")]
    BADREQUEST,
    

    /// URL rejected because of a malformed URL.
    ///
    /// "MALFORMED_URL"
    #[serde(rename="MALFORMED_URL")]
    MALFORMEDURL,
    

    /// URL rejected because of unknown reason.
    ///
    /// "URL_REJECTED_UNKNOWN"
    #[serde(rename="URL_REJECTED_UNKNOWN")]
    URLREJECTEDUNKNOWN,
}

impl AsRef<str> for DestinationNotWorkingEvidenceUrlRejectedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DestinationNotWorkingEvidenceUrlRejectedEnum::URLREJECTEDUNSPECIFIED => "URL_REJECTED_UNSPECIFIED",
            DestinationNotWorkingEvidenceUrlRejectedEnum::BADREQUEST => "BAD_REQUEST",
            DestinationNotWorkingEvidenceUrlRejectedEnum::MALFORMEDURL => "MALFORMED_URL",
            DestinationNotWorkingEvidenceUrlRejectedEnum::URLREJECTEDUNKNOWN => "URL_REJECTED_UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for DestinationNotWorkingEvidenceUrlRejectedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "URL_REJECTED_UNSPECIFIED" => Ok(DestinationNotWorkingEvidenceUrlRejectedEnum::URLREJECTEDUNSPECIFIED),
           "BAD_REQUEST" => Ok(DestinationNotWorkingEvidenceUrlRejectedEnum::BADREQUEST),
           "MALFORMED_URL" => Ok(DestinationNotWorkingEvidenceUrlRejectedEnum::MALFORMEDURL),
           "URL_REJECTED_UNKNOWN" => Ok(DestinationNotWorkingEvidenceUrlRejectedEnum::URLREJECTEDUNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DestinationNotWorkingEvidenceUrlRejectedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointBidProtocolEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The protocol that the bidder endpoint is using.
pub enum EndpointBidProtocolEnum {
    

    /// Placeholder for undefined bid protocol. This value should not be used.
    ///
    /// "BID_PROTOCOL_UNSPECIFIED"
    #[serde(rename="BID_PROTOCOL_UNSPECIFIED")]
    BIDPROTOCOLUNSPECIFIED,
    

    /// Google RTB protocol / Protobuf encoding.
    ///
    /// "GOOGLE_RTB"
    #[serde(rename="GOOGLE_RTB")]
    GOOGLERTB,
    

    /// OpenRTB / JSON encoding (unversioned/latest).
    ///
    /// "OPENRTB_JSON"
    #[serde(rename="OPENRTB_JSON")]
    OPENRTBJSON,
    

    /// OpenRTB / Protobuf encoding (unversioned/latest).
    ///
    /// "OPENRTB_PROTOBUF"
    #[serde(rename="OPENRTB_PROTOBUF")]
    OPENRTBPROTOBUF,
}

impl AsRef<str> for EndpointBidProtocolEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointBidProtocolEnum::BIDPROTOCOLUNSPECIFIED => "BID_PROTOCOL_UNSPECIFIED",
            EndpointBidProtocolEnum::GOOGLERTB => "GOOGLE_RTB",
            EndpointBidProtocolEnum::OPENRTBJSON => "OPENRTB_JSON",
            EndpointBidProtocolEnum::OPENRTBPROTOBUF => "OPENRTB_PROTOBUF",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointBidProtocolEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BID_PROTOCOL_UNSPECIFIED" => Ok(EndpointBidProtocolEnum::BIDPROTOCOLUNSPECIFIED),
           "GOOGLE_RTB" => Ok(EndpointBidProtocolEnum::GOOGLERTB),
           "OPENRTB_JSON" => Ok(EndpointBidProtocolEnum::OPENRTBJSON),
           "OPENRTB_PROTOBUF" => Ok(EndpointBidProtocolEnum::OPENRTBPROTOBUF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointBidProtocolEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EndpointTradingLocationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The trading location that bid requests should be sent from. See https://developers.google.com/authorized-buyers/rtb/peer-guide#trading-locations for further information.
pub enum EndpointTradingLocationEnum {
    

    /// A placeholder for an undefined trading region. This value should not be used.
    ///
    /// "TRADING_LOCATION_UNSPECIFIED"
    #[serde(rename="TRADING_LOCATION_UNSPECIFIED")]
    TRADINGLOCATIONUNSPECIFIED,
    

    /// The Western US trading location.
    ///
    /// "US_WEST"
    #[serde(rename="US_WEST")]
    USWEST,
    

    /// The Eastern US trading location.
    ///
    /// "US_EAST"
    #[serde(rename="US_EAST")]
    USEAST,
    

    /// The European trading location.
    ///
    /// "EUROPE"
    #[serde(rename="EUROPE")]
    EUROPE,
    

    /// The Asia trading location.
    ///
    /// "ASIA"
    #[serde(rename="ASIA")]
    ASIA,
}

impl AsRef<str> for EndpointTradingLocationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EndpointTradingLocationEnum::TRADINGLOCATIONUNSPECIFIED => "TRADING_LOCATION_UNSPECIFIED",
            EndpointTradingLocationEnum::USWEST => "US_WEST",
            EndpointTradingLocationEnum::USEAST => "US_EAST",
            EndpointTradingLocationEnum::EUROPE => "EUROPE",
            EndpointTradingLocationEnum::ASIA => "ASIA",
        }
    }
}

impl std::convert::TryFrom< &str> for EndpointTradingLocationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRADING_LOCATION_UNSPECIFIED" => Ok(EndpointTradingLocationEnum::TRADINGLOCATIONUNSPECIFIED),
           "US_WEST" => Ok(EndpointTradingLocationEnum::USWEST),
           "US_EAST" => Ok(EndpointTradingLocationEnum::USEAST),
           "EUROPE" => Ok(EndpointTradingLocationEnum::EUROPE),
           "ASIA" => Ok(EndpointTradingLocationEnum::ASIA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EndpointTradingLocationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MediaFileMimeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The MIME type of this media file. Can be used to filter the response of the creatives.list method.
pub enum MediaFileMimeTypeEnum {
    

    /// Default value that should never be used.
    ///
    /// "VIDEO_MIME_TYPE_UNSPECIFIED"
    #[serde(rename="VIDEO_MIME_TYPE_UNSPECIFIED")]
    VIDEOMIMETYPEUNSPECIFIED,
    

    /// Flash container.
    ///
    /// "MIME_VIDEO_XFLV"
    #[serde(rename="MIME_VIDEO_XFLV")]
    MIMEVIDEOXFLV,
    

    /// WebM container assuming VP9 codec.
    ///
    /// "MIME_VIDEO_WEBM"
    #[serde(rename="MIME_VIDEO_WEBM")]
    MIMEVIDEOWEBM,
    

    /// MPEG-4 container typically with H.264 codec.
    ///
    /// "MIME_VIDEO_MP4"
    #[serde(rename="MIME_VIDEO_MP4")]
    MIMEVIDEOMP4,
    

    /// Ogg container assuming Theora codec.
    ///
    /// "MIME_VIDEO_OGG"
    #[serde(rename="MIME_VIDEO_OGG")]
    MIMEVIDEOOGG,
    

    /// Video files hosted on YouTube.
    ///
    /// "MIME_VIDEO_YT_HOSTED"
    #[serde(rename="MIME_VIDEO_YT_HOSTED")]
    MIMEVIDEOYTHOSTED,
    

    /// Windows Media Video Codec.
    ///
    /// "MIME_VIDEO_X_MS_WMV"
    #[serde(rename="MIME_VIDEO_X_MS_WMV")]
    MIMEVIDEOXMSWMV,
    

    /// 3GPP container format used on 3G phones.
    ///
    /// "MIME_VIDEO_3GPP"
    #[serde(rename="MIME_VIDEO_3GPP")]
    MIMEVIDEO3GPP,
    

    /// Quicktime container format.
    ///
    /// "MIME_VIDEO_MOV"
    #[serde(rename="MIME_VIDEO_MOV")]
    MIMEVIDEOMOV,
    

    /// Shockwave Flash (used for VPAID ads).
    ///
    /// "MIME_APPLICATION_SWF"
    #[serde(rename="MIME_APPLICATION_SWF")]
    MIMEAPPLICATIONSWF,
    

    /// Properties of VAST served by consumer survey.
    ///
    /// "MIME_APPLICATION_SURVEY"
    #[serde(rename="MIME_APPLICATION_SURVEY")]
    MIMEAPPLICATIONSURVEY,
    

    /// JavaScript (used for VPAID ads).
    ///
    /// "MIME_APPLICATION_JAVASCRIPT"
    #[serde(rename="MIME_APPLICATION_JAVASCRIPT")]
    MIMEAPPLICATIONJAVASCRIPT,
    

    /// Silverlight (used for VPAID ads).
    ///
    /// "MIME_APPLICATION_SILVERLIGHT"
    #[serde(rename="MIME_APPLICATION_SILVERLIGHT")]
    MIMEAPPLICATIONSILVERLIGHT,
    

    /// HLS/M3U8.
    ///
    /// "MIME_APPLICATION_MPEGURL"
    #[serde(rename="MIME_APPLICATION_MPEGURL")]
    MIMEAPPLICATIONMPEGURL,
    

    /// DASH.
    ///
    /// "MIME_APPLICATION_MPEGDASH"
    #[serde(rename="MIME_APPLICATION_MPEGDASH")]
    MIMEAPPLICATIONMPEGDASH,
    

    /// MPEG-4 audio format.
    ///
    /// "MIME_AUDIO_MP4A"
    #[serde(rename="MIME_AUDIO_MP4A")]
    MIMEAUDIOMP4A,
    

    /// MPEG-3 audio format.
    ///
    /// "MIME_AUDIO_MP3"
    #[serde(rename="MIME_AUDIO_MP3")]
    MIMEAUDIOMP3,
    

    /// Ogg audio format
    ///
    /// "MIME_AUDIO_OGG"
    #[serde(rename="MIME_AUDIO_OGG")]
    MIMEAUDIOOGG,
}

impl AsRef<str> for MediaFileMimeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaFileMimeTypeEnum::VIDEOMIMETYPEUNSPECIFIED => "VIDEO_MIME_TYPE_UNSPECIFIED",
            MediaFileMimeTypeEnum::MIMEVIDEOXFLV => "MIME_VIDEO_XFLV",
            MediaFileMimeTypeEnum::MIMEVIDEOWEBM => "MIME_VIDEO_WEBM",
            MediaFileMimeTypeEnum::MIMEVIDEOMP4 => "MIME_VIDEO_MP4",
            MediaFileMimeTypeEnum::MIMEVIDEOOGG => "MIME_VIDEO_OGG",
            MediaFileMimeTypeEnum::MIMEVIDEOYTHOSTED => "MIME_VIDEO_YT_HOSTED",
            MediaFileMimeTypeEnum::MIMEVIDEOXMSWMV => "MIME_VIDEO_X_MS_WMV",
            MediaFileMimeTypeEnum::MIMEVIDEO3GPP => "MIME_VIDEO_3GPP",
            MediaFileMimeTypeEnum::MIMEVIDEOMOV => "MIME_VIDEO_MOV",
            MediaFileMimeTypeEnum::MIMEAPPLICATIONSWF => "MIME_APPLICATION_SWF",
            MediaFileMimeTypeEnum::MIMEAPPLICATIONSURVEY => "MIME_APPLICATION_SURVEY",
            MediaFileMimeTypeEnum::MIMEAPPLICATIONJAVASCRIPT => "MIME_APPLICATION_JAVASCRIPT",
            MediaFileMimeTypeEnum::MIMEAPPLICATIONSILVERLIGHT => "MIME_APPLICATION_SILVERLIGHT",
            MediaFileMimeTypeEnum::MIMEAPPLICATIONMPEGURL => "MIME_APPLICATION_MPEGURL",
            MediaFileMimeTypeEnum::MIMEAPPLICATIONMPEGDASH => "MIME_APPLICATION_MPEGDASH",
            MediaFileMimeTypeEnum::MIMEAUDIOMP4A => "MIME_AUDIO_MP4A",
            MediaFileMimeTypeEnum::MIMEAUDIOMP3 => "MIME_AUDIO_MP3",
            MediaFileMimeTypeEnum::MIMEAUDIOOGG => "MIME_AUDIO_OGG",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaFileMimeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIDEO_MIME_TYPE_UNSPECIFIED" => Ok(MediaFileMimeTypeEnum::VIDEOMIMETYPEUNSPECIFIED),
           "MIME_VIDEO_XFLV" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOXFLV),
           "MIME_VIDEO_WEBM" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOWEBM),
           "MIME_VIDEO_MP4" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOMP4),
           "MIME_VIDEO_OGG" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOOGG),
           "MIME_VIDEO_YT_HOSTED" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOYTHOSTED),
           "MIME_VIDEO_X_MS_WMV" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOXMSWMV),
           "MIME_VIDEO_3GPP" => Ok(MediaFileMimeTypeEnum::MIMEVIDEO3GPP),
           "MIME_VIDEO_MOV" => Ok(MediaFileMimeTypeEnum::MIMEVIDEOMOV),
           "MIME_APPLICATION_SWF" => Ok(MediaFileMimeTypeEnum::MIMEAPPLICATIONSWF),
           "MIME_APPLICATION_SURVEY" => Ok(MediaFileMimeTypeEnum::MIMEAPPLICATIONSURVEY),
           "MIME_APPLICATION_JAVASCRIPT" => Ok(MediaFileMimeTypeEnum::MIMEAPPLICATIONJAVASCRIPT),
           "MIME_APPLICATION_SILVERLIGHT" => Ok(MediaFileMimeTypeEnum::MIMEAPPLICATIONSILVERLIGHT),
           "MIME_APPLICATION_MPEGURL" => Ok(MediaFileMimeTypeEnum::MIMEAPPLICATIONMPEGURL),
           "MIME_APPLICATION_MPEGDASH" => Ok(MediaFileMimeTypeEnum::MIMEAPPLICATIONMPEGDASH),
           "MIME_AUDIO_MP4A" => Ok(MediaFileMimeTypeEnum::MIMEAUDIOMP4A),
           "MIME_AUDIO_MP3" => Ok(MediaFileMimeTypeEnum::MIMEAUDIOMP3),
           "MIME_AUDIO_OGG" => Ok(MediaFileMimeTypeEnum::MIMEAUDIOOGG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaFileMimeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PolicyComplianceStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Serving status for the given transaction type (for example, open auction, deals) or region (for example, China, Russia). Can be used to filter the response of the creatives.list method.
pub enum PolicyComplianceStatusEnum {
    

    /// Default value that should never be used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// Creative is pending review.
    ///
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
    

    /// Creative cannot serve.
    ///
    /// "DISAPPROVED"
    #[serde(rename="DISAPPROVED")]
    DISAPPROVED,
    

    /// Creative is approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// Certificates are required for the creative to be served in some regions. For more information about creative certification, refer to: https://support.google.com/authorizedbuyers/answer/7450776
    ///
    /// "CERTIFICATE_REQUIRED"
    #[serde(rename="CERTIFICATE_REQUIRED")]
    CERTIFICATEREQUIRED,
}

impl AsRef<str> for PolicyComplianceStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PolicyComplianceStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            PolicyComplianceStatusEnum::PENDINGREVIEW => "PENDING_REVIEW",
            PolicyComplianceStatusEnum::DISAPPROVED => "DISAPPROVED",
            PolicyComplianceStatusEnum::APPROVED => "APPROVED",
            PolicyComplianceStatusEnum::CERTIFICATEREQUIRED => "CERTIFICATE_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for PolicyComplianceStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(PolicyComplianceStatusEnum::STATUSUNSPECIFIED),
           "PENDING_REVIEW" => Ok(PolicyComplianceStatusEnum::PENDINGREVIEW),
           "DISAPPROVED" => Ok(PolicyComplianceStatusEnum::DISAPPROVED),
           "APPROVED" => Ok(PolicyComplianceStatusEnum::APPROVED),
           "CERTIFICATE_REQUIRED" => Ok(PolicyComplianceStatusEnum::CERTIFICATEREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PolicyComplianceStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigAllowedUserTargetingModesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Targeting modes included by this configuration. A bid request must allow all the specified targeting modes. An unset value allows all bid requests to be sent, regardless of which targeting modes they allow.
pub enum PretargetingConfigAllowedUserTargetingModesEnum {
    

    /// Placeholder for undefined user targeting mode.
    ///
    /// "USER_TARGETING_MODE_UNSPECIFIED"
    #[serde(rename="USER_TARGETING_MODE_UNSPECIFIED")]
    USERTARGETINGMODEUNSPECIFIED,
    

    /// Remarketing ads are allowed to serve.
    ///
    /// "REMARKETING_ADS"
    #[serde(rename="REMARKETING_ADS")]
    REMARKETINGADS,
    

    /// Ads based on user interest category targeting are allowed to serve.
    ///
    /// "INTEREST_BASED_TARGETING"
    #[serde(rename="INTEREST_BASED_TARGETING")]
    INTERESTBASEDTARGETING,
}

impl AsRef<str> for PretargetingConfigAllowedUserTargetingModesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigAllowedUserTargetingModesEnum::USERTARGETINGMODEUNSPECIFIED => "USER_TARGETING_MODE_UNSPECIFIED",
            PretargetingConfigAllowedUserTargetingModesEnum::REMARKETINGADS => "REMARKETING_ADS",
            PretargetingConfigAllowedUserTargetingModesEnum::INTERESTBASEDTARGETING => "INTEREST_BASED_TARGETING",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigAllowedUserTargetingModesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_TARGETING_MODE_UNSPECIFIED" => Ok(PretargetingConfigAllowedUserTargetingModesEnum::USERTARGETINGMODEUNSPECIFIED),
           "REMARKETING_ADS" => Ok(PretargetingConfigAllowedUserTargetingModesEnum::REMARKETINGADS),
           "INTEREST_BASED_TARGETING" => Ok(PretargetingConfigAllowedUserTargetingModesEnum::INTERESTBASEDTARGETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigAllowedUserTargetingModesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigIncludedEnvironmentsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Environments that are being included. Bid requests will not be sent for a given environment if it is not included. Further restrictions can be applied to included environments to target only a subset of its inventory. An unset value includes all environments.
pub enum PretargetingConfigIncludedEnvironmentsEnum {
    

    /// Placeholder for unspecified environment. This value should not be used.
    ///
    /// "ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_UNSPECIFIED")]
    ENVIRONMENTUNSPECIFIED,
    

    /// App environment.
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
    

    /// Web environment.
    ///
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
}

impl AsRef<str> for PretargetingConfigIncludedEnvironmentsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigIncludedEnvironmentsEnum::ENVIRONMENTUNSPECIFIED => "ENVIRONMENT_UNSPECIFIED",
            PretargetingConfigIncludedEnvironmentsEnum::APP => "APP",
            PretargetingConfigIncludedEnvironmentsEnum::WEB => "WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigIncludedEnvironmentsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_UNSPECIFIED" => Ok(PretargetingConfigIncludedEnvironmentsEnum::ENVIRONMENTUNSPECIFIED),
           "APP" => Ok(PretargetingConfigIncludedEnvironmentsEnum::APP),
           "WEB" => Ok(PretargetingConfigIncludedEnvironmentsEnum::WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigIncludedEnvironmentsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigIncludedFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Creative formats included by this configuration. Only bid requests eligible for at least one of the specified creative formats will be sent. An unset value will allow all bid requests to be sent, regardless of format.
pub enum PretargetingConfigIncludedFormatsEnum {
    

    /// Placeholder for undefined creative format. This value should not be used.
    ///
    /// "CREATIVE_FORMAT_UNSPECIFIED"
    #[serde(rename="CREATIVE_FORMAT_UNSPECIFIED")]
    CREATIVEFORMATUNSPECIFIED,
    

    /// HTML and AMPHTML creatives.
    ///
    /// "HTML"
    #[serde(rename="HTML")]
    HTML,
    

    /// VAST video or audio creative.
    ///
    /// "VAST"
    #[serde(rename="VAST")]
    VAST,
    

    /// Native creative, including standard and video native ads.
    ///
    /// "NATIVE"
    #[serde(rename="NATIVE")]
    NATIVE,
}

impl AsRef<str> for PretargetingConfigIncludedFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigIncludedFormatsEnum::CREATIVEFORMATUNSPECIFIED => "CREATIVE_FORMAT_UNSPECIFIED",
            PretargetingConfigIncludedFormatsEnum::HTML => "HTML",
            PretargetingConfigIncludedFormatsEnum::VAST => "VAST",
            PretargetingConfigIncludedFormatsEnum::NATIVE => "NATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigIncludedFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_FORMAT_UNSPECIFIED" => Ok(PretargetingConfigIncludedFormatsEnum::CREATIVEFORMATUNSPECIFIED),
           "HTML" => Ok(PretargetingConfigIncludedFormatsEnum::HTML),
           "VAST" => Ok(PretargetingConfigIncludedFormatsEnum::VAST),
           "NATIVE" => Ok(PretargetingConfigIncludedFormatsEnum::NATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigIncludedFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigIncludedPlatformsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platforms included by this configration. Bid requests for devices with the specified platform types will be sent. An unset value allows all bid requests to be sent, regardless of platform.
pub enum PretargetingConfigIncludedPlatformsEnum {
    

    /// Placeholder for an undefined platform. This value should not be used.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// The personal computer platform.
    ///
    /// "PERSONAL_COMPUTER"
    #[serde(rename="PERSONAL_COMPUTER")]
    PERSONALCOMPUTER,
    

    /// The mobile platform.
    ///
    /// "PHONE"
    #[serde(rename="PHONE")]
    PHONE,
    

    /// The tablet platform.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
    

    /// The connected TV platform.
    ///
    /// "CONNECTED_TV"
    #[serde(rename="CONNECTED_TV")]
    CONNECTEDTV,
}

impl AsRef<str> for PretargetingConfigIncludedPlatformsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigIncludedPlatformsEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            PretargetingConfigIncludedPlatformsEnum::PERSONALCOMPUTER => "PERSONAL_COMPUTER",
            PretargetingConfigIncludedPlatformsEnum::PHONE => "PHONE",
            PretargetingConfigIncludedPlatformsEnum::TABLET => "TABLET",
            PretargetingConfigIncludedPlatformsEnum::CONNECTEDTV => "CONNECTED_TV",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigIncludedPlatformsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(PretargetingConfigIncludedPlatformsEnum::PLATFORMUNSPECIFIED),
           "PERSONAL_COMPUTER" => Ok(PretargetingConfigIncludedPlatformsEnum::PERSONALCOMPUTER),
           "PHONE" => Ok(PretargetingConfigIncludedPlatformsEnum::PHONE),
           "TABLET" => Ok(PretargetingConfigIncludedPlatformsEnum::TABLET),
           "CONNECTED_TV" => Ok(PretargetingConfigIncludedPlatformsEnum::CONNECTEDTV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigIncludedPlatformsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigIncludedUserIdTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// User identifier types included in this configuration. At least one of the user identifier types specified in this list must be available for the bid request to be sent.
pub enum PretargetingConfigIncludedUserIdTypesEnum {
    

    /// Placeholder for unspecified user identifier.
    ///
    /// "USER_ID_TYPE_UNSPECIFIED"
    #[serde(rename="USER_ID_TYPE_UNSPECIFIED")]
    USERIDTYPEUNSPECIFIED,
    

    /// Hosted match data, referring to hosted_match_data in the bid request.
    ///
    /// "HOSTED_MATCH_DATA"
    #[serde(rename="HOSTED_MATCH_DATA")]
    HOSTEDMATCHDATA,
    

    /// Google cookie, referring to google_user_id in the bid request.
    ///
    /// "GOOGLE_COOKIE"
    #[serde(rename="GOOGLE_COOKIE")]
    GOOGLECOOKIE,
    

    /// Mobile device advertising ID.
    ///
    /// "DEVICE_ID"
    #[serde(rename="DEVICE_ID")]
    DEVICEID,
    

    /// The request has a publisher-provided ID available to the bidder.
    ///
    /// "PUBLISHER_PROVIDED_ID"
    #[serde(rename="PUBLISHER_PROVIDED_ID")]
    PUBLISHERPROVIDEDID,
}

impl AsRef<str> for PretargetingConfigIncludedUserIdTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigIncludedUserIdTypesEnum::USERIDTYPEUNSPECIFIED => "USER_ID_TYPE_UNSPECIFIED",
            PretargetingConfigIncludedUserIdTypesEnum::HOSTEDMATCHDATA => "HOSTED_MATCH_DATA",
            PretargetingConfigIncludedUserIdTypesEnum::GOOGLECOOKIE => "GOOGLE_COOKIE",
            PretargetingConfigIncludedUserIdTypesEnum::DEVICEID => "DEVICE_ID",
            PretargetingConfigIncludedUserIdTypesEnum::PUBLISHERPROVIDEDID => "PUBLISHER_PROVIDED_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigIncludedUserIdTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_ID_TYPE_UNSPECIFIED" => Ok(PretargetingConfigIncludedUserIdTypesEnum::USERIDTYPEUNSPECIFIED),
           "HOSTED_MATCH_DATA" => Ok(PretargetingConfigIncludedUserIdTypesEnum::HOSTEDMATCHDATA),
           "GOOGLE_COOKIE" => Ok(PretargetingConfigIncludedUserIdTypesEnum::GOOGLECOOKIE),
           "DEVICE_ID" => Ok(PretargetingConfigIncludedUserIdTypesEnum::DEVICEID),
           "PUBLISHER_PROVIDED_ID" => Ok(PretargetingConfigIncludedUserIdTypesEnum::PUBLISHERPROVIDEDID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigIncludedUserIdTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigInterstitialTargetingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not.
pub enum PretargetingConfigInterstitialTargetingEnum {
    

    /// Unspecified interstitial targeting. Represents an interstitial-agnostic selection.
    ///
    /// "INTERSTITIAL_TARGETING_UNSPECIFIED"
    #[serde(rename="INTERSTITIAL_TARGETING_UNSPECIFIED")]
    INTERSTITIALTARGETINGUNSPECIFIED,
    

    /// Only bid requests for interstitial inventory should be sent.
    ///
    /// "ONLY_INTERSTITIAL_REQUESTS"
    #[serde(rename="ONLY_INTERSTITIAL_REQUESTS")]
    ONLYINTERSTITIALREQUESTS,
    

    /// Only bid requests for non-interstitial inventory should be sent.
    ///
    /// "ONLY_NON_INTERSTITIAL_REQUESTS"
    #[serde(rename="ONLY_NON_INTERSTITIAL_REQUESTS")]
    ONLYNONINTERSTITIALREQUESTS,
}

impl AsRef<str> for PretargetingConfigInterstitialTargetingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigInterstitialTargetingEnum::INTERSTITIALTARGETINGUNSPECIFIED => "INTERSTITIAL_TARGETING_UNSPECIFIED",
            PretargetingConfigInterstitialTargetingEnum::ONLYINTERSTITIALREQUESTS => "ONLY_INTERSTITIAL_REQUESTS",
            PretargetingConfigInterstitialTargetingEnum::ONLYNONINTERSTITIALREQUESTS => "ONLY_NON_INTERSTITIAL_REQUESTS",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigInterstitialTargetingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INTERSTITIAL_TARGETING_UNSPECIFIED" => Ok(PretargetingConfigInterstitialTargetingEnum::INTERSTITIALTARGETINGUNSPECIFIED),
           "ONLY_INTERSTITIAL_REQUESTS" => Ok(PretargetingConfigInterstitialTargetingEnum::ONLYINTERSTITIALREQUESTS),
           "ONLY_NON_INTERSTITIAL_REQUESTS" => Ok(PretargetingConfigInterstitialTargetingEnum::ONLYNONINTERSTITIALREQUESTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigInterstitialTargetingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PretargetingConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this pretargeting configuration.
pub enum PretargetingConfigStateEnum {
    

    /// Placeholder for undefined state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// This pretargeting configuration is actively being used to filter bid requests.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// This pretargeting configuration is suspended and not used in serving.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for PretargetingConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PretargetingConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PretargetingConfigStateEnum::ACTIVE => "ACTIVE",
            PretargetingConfigStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for PretargetingConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PretargetingConfigStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(PretargetingConfigStateEnum::ACTIVE),
           "SUSPENDED" => Ok(PretargetingConfigStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PretargetingConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublisherConnectionBiddingStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the publisher has been approved by the bidder.
pub enum PublisherConnectionBiddingStateEnum {
    

    /// An unspecified bidding status.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Indicates a request for connection from the publisher that the bidder needs to review.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Indicates that the publisher was rejected.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// Indicates that the publisher was approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
}

impl AsRef<str> for PublisherConnectionBiddingStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublisherConnectionBiddingStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            PublisherConnectionBiddingStateEnum::PENDING => "PENDING",
            PublisherConnectionBiddingStateEnum::REJECTED => "REJECTED",
            PublisherConnectionBiddingStateEnum::APPROVED => "APPROVED",
        }
    }
}

impl std::convert::TryFrom< &str> for PublisherConnectionBiddingStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(PublisherConnectionBiddingStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(PublisherConnectionBiddingStateEnum::PENDING),
           "REJECTED" => Ok(PublisherConnectionBiddingStateEnum::REJECTED),
           "APPROVED" => Ok(PublisherConnectionBiddingStateEnum::APPROVED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublisherConnectionBiddingStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublisherConnectionPublisherPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Whether the publisher is an Ad Manager or AdMob publisher.
pub enum PublisherConnectionPublisherPlatformEnum {
    

    /// An unspecified publisher platform.
    ///
    /// "PUBLISHER_PLATFORM_UNSPECIFIED"
    #[serde(rename="PUBLISHER_PLATFORM_UNSPECIFIED")]
    PUBLISHERPLATFORMUNSPECIFIED,
    

    /// A Google Ad Manager publisher.
    ///
    /// "GOOGLE_AD_MANAGER"
    #[serde(rename="GOOGLE_AD_MANAGER")]
    GOOGLEADMANAGER,
    

    /// An AdMob publisher.
    ///
    /// "ADMOB"
    #[serde(rename="ADMOB")]
    ADMOB,
}

impl AsRef<str> for PublisherConnectionPublisherPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PublisherConnectionPublisherPlatformEnum::PUBLISHERPLATFORMUNSPECIFIED => "PUBLISHER_PLATFORM_UNSPECIFIED",
            PublisherConnectionPublisherPlatformEnum::GOOGLEADMANAGER => "GOOGLE_AD_MANAGER",
            PublisherConnectionPublisherPlatformEnum::ADMOB => "ADMOB",
        }
    }
}

impl std::convert::TryFrom< &str> for PublisherConnectionPublisherPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLISHER_PLATFORM_UNSPECIFIED" => Ok(PublisherConnectionPublisherPlatformEnum::PUBLISHERPLATFORMUNSPECIFIED),
           "GOOGLE_AD_MANAGER" => Ok(PublisherConnectionPublisherPlatformEnum::GOOGLEADMANAGER),
           "ADMOB" => Ok(PublisherConnectionPublisherPlatformEnum::ADMOB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PublisherConnectionPublisherPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StringTargetingDimensionTargetingModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the items in this list should be targeted.
pub enum StringTargetingDimensionTargetingModeEnum {
    

    /// Placeholder for undefined targeting mode.
    ///
    /// "TARGETING_MODE_UNSPECIFIED"
    #[serde(rename="TARGETING_MODE_UNSPECIFIED")]
    TARGETINGMODEUNSPECIFIED,
    

    /// The inclusive list type. Inventory must match an item in this list to be targeted.
    ///
    /// "INCLUSIVE"
    #[serde(rename="INCLUSIVE")]
    INCLUSIVE,
    

    /// The exclusive list type. Inventory must not match any item in this list to be targeted.
    ///
    /// "EXCLUSIVE"
    #[serde(rename="EXCLUSIVE")]
    EXCLUSIVE,
}

impl AsRef<str> for StringTargetingDimensionTargetingModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StringTargetingDimensionTargetingModeEnum::TARGETINGMODEUNSPECIFIED => "TARGETING_MODE_UNSPECIFIED",
            StringTargetingDimensionTargetingModeEnum::INCLUSIVE => "INCLUSIVE",
            StringTargetingDimensionTargetingModeEnum::EXCLUSIVE => "EXCLUSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for StringTargetingDimensionTargetingModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARGETING_MODE_UNSPECIFIED" => Ok(StringTargetingDimensionTargetingModeEnum::TARGETINGMODEUNSPECIFIED),
           "INCLUSIVE" => Ok(StringTargetingDimensionTargetingModeEnum::INCLUSIVE),
           "EXCLUSIVE" => Ok(StringTargetingDimensionTargetingModeEnum::EXCLUSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StringTargetingDimensionTargetingModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UrlRestrictionRestrictionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The restriction type for the specified URL.
pub enum UrlRestrictionRestrictionTypeEnum {
    

    /// Default value that should never be used.
    ///
    /// "RESTRICTION_TYPE_UNSPECIFIED"
    #[serde(rename="RESTRICTION_TYPE_UNSPECIFIED")]
    RESTRICTIONTYPEUNSPECIFIED,
    

    /// The tag URL (as recorded by the pixel callback) contains the specified URL.
    ///
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
    

    /// The tag URL (as recorded by the pixel callback) exactly matches the specified URL.
    ///
    /// "EQUALS"
    #[serde(rename="EQUALS")]
    EQUALS,
    

    /// The tag URL (as recorded by the pixel callback) starts with the specified URL.
    ///
    /// "STARTS_WITH"
    #[serde(rename="STARTS_WITH")]
    STARTSWITH,
    

    /// The tag URL (as recorded by the pixel callback) ends with the specified URL.
    ///
    /// "ENDS_WITH"
    #[serde(rename="ENDS_WITH")]
    ENDSWITH,
    

    /// The tag URL (as recorded by the pixel callback) does not equal the specified URL.
    ///
    /// "DOES_NOT_EQUAL"
    #[serde(rename="DOES_NOT_EQUAL")]
    DOESNOTEQUAL,
    

    /// The tag URL (as recorded by the pixel callback) does not contain the specified URL.
    ///
    /// "DOES_NOT_CONTAIN"
    #[serde(rename="DOES_NOT_CONTAIN")]
    DOESNOTCONTAIN,
    

    /// The tag URL (as recorded by the pixel callback) does not start with the specified URL.
    ///
    /// "DOES_NOT_START_WITH"
    #[serde(rename="DOES_NOT_START_WITH")]
    DOESNOTSTARTWITH,
    

    /// The tag URL (as recorded by the pixel callback) does not end with the specified URL.
    ///
    /// "DOES_NOT_END_WITH"
    #[serde(rename="DOES_NOT_END_WITH")]
    DOESNOTENDWITH,
}

impl AsRef<str> for UrlRestrictionRestrictionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlRestrictionRestrictionTypeEnum::RESTRICTIONTYPEUNSPECIFIED => "RESTRICTION_TYPE_UNSPECIFIED",
            UrlRestrictionRestrictionTypeEnum::CONTAINS => "CONTAINS",
            UrlRestrictionRestrictionTypeEnum::EQUALS => "EQUALS",
            UrlRestrictionRestrictionTypeEnum::STARTSWITH => "STARTS_WITH",
            UrlRestrictionRestrictionTypeEnum::ENDSWITH => "ENDS_WITH",
            UrlRestrictionRestrictionTypeEnum::DOESNOTEQUAL => "DOES_NOT_EQUAL",
            UrlRestrictionRestrictionTypeEnum::DOESNOTCONTAIN => "DOES_NOT_CONTAIN",
            UrlRestrictionRestrictionTypeEnum::DOESNOTSTARTWITH => "DOES_NOT_START_WITH",
            UrlRestrictionRestrictionTypeEnum::DOESNOTENDWITH => "DOES_NOT_END_WITH",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlRestrictionRestrictionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTRICTION_TYPE_UNSPECIFIED" => Ok(UrlRestrictionRestrictionTypeEnum::RESTRICTIONTYPEUNSPECIFIED),
           "CONTAINS" => Ok(UrlRestrictionRestrictionTypeEnum::CONTAINS),
           "EQUALS" => Ok(UrlRestrictionRestrictionTypeEnum::EQUALS),
           "STARTS_WITH" => Ok(UrlRestrictionRestrictionTypeEnum::STARTSWITH),
           "ENDS_WITH" => Ok(UrlRestrictionRestrictionTypeEnum::ENDSWITH),
           "DOES_NOT_EQUAL" => Ok(UrlRestrictionRestrictionTypeEnum::DOESNOTEQUAL),
           "DOES_NOT_CONTAIN" => Ok(UrlRestrictionRestrictionTypeEnum::DOESNOTCONTAIN),
           "DOES_NOT_START_WITH" => Ok(UrlRestrictionRestrictionTypeEnum::DOESNOTSTARTWITH),
           "DOES_NOT_END_WITH" => Ok(UrlRestrictionRestrictionTypeEnum::DOESNOTENDWITH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlRestrictionRestrictionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserListStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The status of the user list. A new user list starts out as open.
pub enum UserListStatusEnum {
    

    /// Default value that should never be used.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// New users can be added to the user list.
    ///
    /// "OPEN"
    #[serde(rename="OPEN")]
    OPEN,
    

    /// New users cannot be added to the user list.
    ///
    /// "CLOSED"
    #[serde(rename="CLOSED")]
    CLOSED,
}

impl AsRef<str> for UserListStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserListStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            UserListStatusEnum::OPEN => "OPEN",
            UserListStatusEnum::CLOSED => "CLOSED",
        }
    }
}

impl std::convert::TryFrom< &str> for UserListStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(UserListStatusEnum::STATUSUNSPECIFIED),
           "OPEN" => Ok(UserListStatusEnum::OPEN),
           "CLOSED" => Ok(UserListStatusEnum::CLOSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserListStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoMetadataVastVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The maximum VAST version across all wrapped VAST documents. Can be used to filter the response of the creatives.list method.
pub enum VideoMetadataVastVersionEnum {
    

    /// Default value that should never be used.
    ///
    /// "VAST_VERSION_UNSPECIFIED"
    #[serde(rename="VAST_VERSION_UNSPECIFIED")]
    VASTVERSIONUNSPECIFIED,
    

    /// VAST 1.0
    ///
    /// "VAST_VERSION_1_0"
    #[serde(rename="VAST_VERSION_1_0")]
    VASTVERSION10,
    

    /// VAST 2.0
    ///
    /// "VAST_VERSION_2_0"
    #[serde(rename="VAST_VERSION_2_0")]
    VASTVERSION20,
    

    /// VAST 3.0
    ///
    /// "VAST_VERSION_3_0"
    #[serde(rename="VAST_VERSION_3_0")]
    VASTVERSION30,
    

    /// VAST 4.0
    ///
    /// "VAST_VERSION_4_0"
    #[serde(rename="VAST_VERSION_4_0")]
    VASTVERSION40,
}

impl AsRef<str> for VideoMetadataVastVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoMetadataVastVersionEnum::VASTVERSIONUNSPECIFIED => "VAST_VERSION_UNSPECIFIED",
            VideoMetadataVastVersionEnum::VASTVERSION10 => "VAST_VERSION_1_0",
            VideoMetadataVastVersionEnum::VASTVERSION20 => "VAST_VERSION_2_0",
            VideoMetadataVastVersionEnum::VASTVERSION30 => "VAST_VERSION_3_0",
            VideoMetadataVastVersionEnum::VASTVERSION40 => "VAST_VERSION_4_0",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoMetadataVastVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VAST_VERSION_UNSPECIFIED" => Ok(VideoMetadataVastVersionEnum::VASTVERSIONUNSPECIFIED),
           "VAST_VERSION_1_0" => Ok(VideoMetadataVastVersionEnum::VASTVERSION10),
           "VAST_VERSION_2_0" => Ok(VideoMetadataVastVersionEnum::VASTVERSION20),
           "VAST_VERSION_3_0" => Ok(VideoMetadataVastVersionEnum::VASTVERSION30),
           "VAST_VERSION_4_0" => Ok(VideoMetadataVastVersionEnum::VASTVERSION40),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoMetadataVastVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BidderViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as "FULL".
pub enum BidderViewEnum {
    

    /// Not specified, equivalent to SERVING_DECISION_ONLY.
    ///
    /// "CREATIVE_VIEW_UNSPECIFIED"
    #[serde(rename="CREATIVE_VIEW_UNSPECIFIED")]
    CREATIVEVIEWUNSPECIFIED,
    

    /// Only creativeServingDecision is included in the response.
    ///
    /// "SERVING_DECISION_ONLY"
    #[serde(rename="SERVING_DECISION_ONLY")]
    SERVINGDECISIONONLY,
    

    /// The entire creative resource (including the declared fields and the creative content) is included in the response.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for BidderViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BidderViewEnum::CREATIVEVIEWUNSPECIFIED => "CREATIVE_VIEW_UNSPECIFIED",
            BidderViewEnum::SERVINGDECISIONONLY => "SERVING_DECISION_ONLY",
            BidderViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for BidderViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_VIEW_UNSPECIFIED" => Ok(BidderViewEnum::CREATIVEVIEWUNSPECIFIED),
           "SERVING_DECISION_ONLY" => Ok(BidderViewEnum::SERVINGDECISIONONLY),
           "FULL" => Ok(BidderViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BidderViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BuyerViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as "FULL".
pub enum BuyerViewEnum {
    

    /// Not specified, equivalent to SERVING_DECISION_ONLY.
    ///
    /// "CREATIVE_VIEW_UNSPECIFIED"
    #[serde(rename="CREATIVE_VIEW_UNSPECIFIED")]
    CREATIVEVIEWUNSPECIFIED,
    

    /// Only creativeServingDecision is included in the response.
    ///
    /// "SERVING_DECISION_ONLY"
    #[serde(rename="SERVING_DECISION_ONLY")]
    SERVINGDECISIONONLY,
    

    /// The entire creative resource (including the declared fields and the creative content) is included in the response.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for BuyerViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuyerViewEnum::CREATIVEVIEWUNSPECIFIED => "CREATIVE_VIEW_UNSPECIFIED",
            BuyerViewEnum::SERVINGDECISIONONLY => "SERVING_DECISION_ONLY",
            BuyerViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for BuyerViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_VIEW_UNSPECIFIED" => Ok(BuyerViewEnum::CREATIVEVIEWUNSPECIFIED),
           "SERVING_DECISION_ONLY" => Ok(BuyerViewEnum::SERVINGDECISIONONLY),
           "FULL" => Ok(BuyerViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuyerViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


