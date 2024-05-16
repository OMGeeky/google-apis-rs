use super::*;



// region AdSizeSizeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The size type of the ad slot.
pub enum AdSizeSizeTypeEnum {
    

    /// A placeholder for an undefined size type.
    ///
    /// "SIZE_TYPE_UNSPECIFIED"
    #[serde(rename="SIZE_TYPE_UNSPECIFIED")]
    SIZETYPEUNSPECIFIED,
    

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
    

    /// Fluid size (for example, responsive size) can be resized automatically with the change of outside environment.
    ///
    /// "FLUID"
    #[serde(rename="FLUID")]
    FLUID,
}

impl AsRef<str> for AdSizeSizeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdSizeSizeTypeEnum::SIZETYPEUNSPECIFIED => "SIZE_TYPE_UNSPECIFIED",
            AdSizeSizeTypeEnum::PIXEL => "PIXEL",
            AdSizeSizeTypeEnum::INTERSTITIAL => "INTERSTITIAL",
            AdSizeSizeTypeEnum::NATIVE => "NATIVE",
            AdSizeSizeTypeEnum::FLUID => "FLUID",
        }
    }
}

impl std::convert::TryFrom< &str> for AdSizeSizeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SIZE_TYPE_UNSPECIFIED" => Ok(AdSizeSizeTypeEnum::SIZETYPEUNSPECIFIED),
           "PIXEL" => Ok(AdSizeSizeTypeEnum::PIXEL),
           "INTERSTITIAL" => Ok(AdSizeSizeTypeEnum::INTERSTITIAL),
           "NATIVE" => Ok(AdSizeSizeTypeEnum::NATIVE),
           "FLUID" => Ok(AdSizeSizeTypeEnum::FLUID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdSizeSizeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppContextAppTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The app types this restriction applies to.
pub enum AppContextAppTypesEnum {
    

    /// Native app context.
    ///
    /// "NATIVE"
    #[serde(rename="NATIVE")]
    NATIVE,
    

    /// Mobile web app context.
    ///
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
}

impl AsRef<str> for AppContextAppTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppContextAppTypesEnum::NATIVE => "NATIVE",
            AppContextAppTypesEnum::WEB => "WEB",
        }
    }
}

impl std::convert::TryFrom< &str> for AppContextAppTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NATIVE" => Ok(AppContextAppTypesEnum::NATIVE),
           "WEB" => Ok(AppContextAppTypesEnum::WEB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppContextAppTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuctionContextAuctionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The auction types this restriction applies to.
pub enum AuctionContextAuctionTypesEnum {
    

    /// The restriction applies to open auction.
    ///
    /// "OPEN_AUCTION"
    #[serde(rename="OPEN_AUCTION")]
    OPENAUCTION,
    

    /// The restriction applies to direct deals.
    ///
    /// "DIRECT_DEALS"
    #[serde(rename="DIRECT_DEALS")]
    DIRECTDEALS,
}

impl AsRef<str> for AuctionContextAuctionTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuctionContextAuctionTypesEnum::OPENAUCTION => "OPEN_AUCTION",
            AuctionContextAuctionTypesEnum::DIRECTDEALS => "DIRECT_DEALS",
        }
    }
}

impl std::convert::TryFrom< &str> for AuctionContextAuctionTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPEN_AUCTION" => Ok(AuctionContextAuctionTypesEnum::OPENAUCTION),
           "DIRECT_DEALS" => Ok(AuctionContextAuctionTypesEnum::DIRECTDEALS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuctionContextAuctionTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BidResponseWithoutBidsStatusRowStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status specifying why the bid responses were considered to have no applicable bids.
pub enum BidResponseWithoutBidsStatusRowStatusEnum {
    

    /// A placeholder for an undefined status. This value will never be returned in responses.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The response had no bids.
    ///
    /// "RESPONSES_WITHOUT_BIDS"
    #[serde(rename="RESPONSES_WITHOUT_BIDS")]
    RESPONSESWITHOUTBIDS,
    

    /// The response had no bids for the specified account, though it may have included bids on behalf of other accounts. Applies if: 1. Request is on behalf of a bidder and an account filter is present. 2. Request is on behalf of a child seat.
    ///
    /// "RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT"
    #[serde(rename="RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT")]
    RESPONSESWITHOUTBIDSFORACCOUNT,
    

    /// The response had no bids for the specified deal, though it may have included bids on other deals on behalf of the account to which the deal belongs. If request is on behalf of a bidder and an account filter is not present, this also includes responses that have bids on behalf of accounts other than the account to which the deal belongs.
    ///
    /// "RESPONSES_WITHOUT_BIDS_FOR_DEAL"
    #[serde(rename="RESPONSES_WITHOUT_BIDS_FOR_DEAL")]
    RESPONSESWITHOUTBIDSFORDEAL,
}

impl AsRef<str> for BidResponseWithoutBidsStatusRowStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BidResponseWithoutBidsStatusRowStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            BidResponseWithoutBidsStatusRowStatusEnum::RESPONSESWITHOUTBIDS => "RESPONSES_WITHOUT_BIDS",
            BidResponseWithoutBidsStatusRowStatusEnum::RESPONSESWITHOUTBIDSFORACCOUNT => "RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT",
            BidResponseWithoutBidsStatusRowStatusEnum::RESPONSESWITHOUTBIDSFORDEAL => "RESPONSES_WITHOUT_BIDS_FOR_DEAL",
        }
    }
}

impl std::convert::TryFrom< &str> for BidResponseWithoutBidsStatusRowStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(BidResponseWithoutBidsStatusRowStatusEnum::STATUSUNSPECIFIED),
           "RESPONSES_WITHOUT_BIDS" => Ok(BidResponseWithoutBidsStatusRowStatusEnum::RESPONSESWITHOUTBIDS),
           "RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT" => Ok(BidResponseWithoutBidsStatusRowStatusEnum::RESPONSESWITHOUTBIDSFORACCOUNT),
           "RESPONSES_WITHOUT_BIDS_FOR_DEAL" => Ok(BidResponseWithoutBidsStatusRowStatusEnum::RESPONSESWITHOUTBIDSFORDEAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BidResponseWithoutBidsStatusRowStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClientEntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// An optional field for specifying the type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`.
pub enum ClientEntityTypeEnum {
    

    /// A placeholder for an undefined client entity type. Should not be used.
    ///
    /// "ENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="ENTITY_TYPE_UNSPECIFIED")]
    ENTITYTYPEUNSPECIFIED,
    

    /// An advertiser.
    ///
    /// "ADVERTISER"
    #[serde(rename="ADVERTISER")]
    ADVERTISER,
    

    /// A brand.
    ///
    /// "BRAND"
    #[serde(rename="BRAND")]
    BRAND,
    

    /// An advertising agency.
    ///
    /// "AGENCY"
    #[serde(rename="AGENCY")]
    AGENCY,
    

    /// An explicit value for a client that was not yet classified as any particular entity.
    ///
    /// "ENTITY_TYPE_UNCLASSIFIED"
    #[serde(rename="ENTITY_TYPE_UNCLASSIFIED")]
    ENTITYTYPEUNCLASSIFIED,
}

impl AsRef<str> for ClientEntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClientEntityTypeEnum::ENTITYTYPEUNSPECIFIED => "ENTITY_TYPE_UNSPECIFIED",
            ClientEntityTypeEnum::ADVERTISER => "ADVERTISER",
            ClientEntityTypeEnum::BRAND => "BRAND",
            ClientEntityTypeEnum::AGENCY => "AGENCY",
            ClientEntityTypeEnum::ENTITYTYPEUNCLASSIFIED => "ENTITY_TYPE_UNCLASSIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for ClientEntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTITY_TYPE_UNSPECIFIED" => Ok(ClientEntityTypeEnum::ENTITYTYPEUNSPECIFIED),
           "ADVERTISER" => Ok(ClientEntityTypeEnum::ADVERTISER),
           "BRAND" => Ok(ClientEntityTypeEnum::BRAND),
           "AGENCY" => Ok(ClientEntityTypeEnum::AGENCY),
           "ENTITY_TYPE_UNCLASSIFIED" => Ok(ClientEntityTypeEnum::ENTITYTYPEUNCLASSIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClientEntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClientRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role which is assigned to the client buyer. Each role implies a set of permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`, `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`.
pub enum ClientRoleEnum {
    

    /// A placeholder for an undefined client role.
    ///
    /// "CLIENT_ROLE_UNSPECIFIED"
    #[serde(rename="CLIENT_ROLE_UNSPECIFIED")]
    CLIENTROLEUNSPECIFIED,
    

    /// Users associated with this client can see publisher deal offers in the Marketplace. They can neither negotiate proposals nor approve deals. If this client is visible to publishers, they can send deal proposals to this client.
    ///
    /// "CLIENT_DEAL_VIEWER"
    #[serde(rename="CLIENT_DEAL_VIEWER")]
    CLIENTDEALVIEWER,
    

    /// Users associated with this client can respond to deal proposals sent to them by publishers. They can also initiate deal proposals of their own.
    ///
    /// "CLIENT_DEAL_NEGOTIATOR"
    #[serde(rename="CLIENT_DEAL_NEGOTIATOR")]
    CLIENTDEALNEGOTIATOR,
    

    /// Users associated with this client can approve eligible deals on your behalf. Some deals may still explicitly require publisher finalization. If this role is not selected, the sponsor buyer will need to manually approve each of their deals.
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


// region ClientStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the client buyer.
pub enum ClientStatusEnum {
    

    /// A placeholder for an undefined client status.
    ///
    /// "CLIENT_STATUS_UNSPECIFIED"
    #[serde(rename="CLIENT_STATUS_UNSPECIFIED")]
    CLIENTSTATUSUNSPECIFIED,
    

    /// A client that is currently disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// A client that is currently active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
}

impl AsRef<str> for ClientStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClientStatusEnum::CLIENTSTATUSUNSPECIFIED => "CLIENT_STATUS_UNSPECIFIED",
            ClientStatusEnum::DISABLED => "DISABLED",
            ClientStatusEnum::ACTIVE => "ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ClientStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLIENT_STATUS_UNSPECIFIED" => Ok(ClientStatusEnum::CLIENTSTATUSUNSPECIFIED),
           "DISABLED" => Ok(ClientStatusEnum::DISABLED),
           "ACTIVE" => Ok(ClientStatusEnum::ACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClientStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClientUserStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the client user.
pub enum ClientUserStatusEnum {
    

    /// A placeholder for an undefined user status.
    ///
    /// "USER_STATUS_UNSPECIFIED"
    #[serde(rename="USER_STATUS_UNSPECIFIED")]
    USERSTATUSUNSPECIFIED,
    

    /// A user who was already created but hasn't accepted the invitation yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// A user that is currently active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// A user that is currently disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for ClientUserStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClientUserStatusEnum::USERSTATUSUNSPECIFIED => "USER_STATUS_UNSPECIFIED",
            ClientUserStatusEnum::PENDING => "PENDING",
            ClientUserStatusEnum::ACTIVE => "ACTIVE",
            ClientUserStatusEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for ClientUserStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_STATUS_UNSPECIFIED" => Ok(ClientUserStatusEnum::USERSTATUSUNSPECIFIED),
           "PENDING" => Ok(ClientUserStatusEnum::PENDING),
           "ACTIVE" => Ok(ClientUserStatusEnum::ACTIVE),
           "DISABLED" => Ok(ClientUserStatusEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClientUserStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CorrectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of correction that was applied to the creative.
pub enum CorrectionTypeEnum {
    

    /// The correction type is unknown. Refer to the details for more information.
    ///
    /// "CORRECTION_TYPE_UNSPECIFIED"
    #[serde(rename="CORRECTION_TYPE_UNSPECIFIED")]
    CORRECTIONTYPEUNSPECIFIED,
    

    /// The ad's declared vendors did not match the vendors that were detected. The detected vendors were added.
    ///
    /// "VENDOR_IDS_ADDED"
    #[serde(rename="VENDOR_IDS_ADDED")]
    VENDORIDSADDED,
    

    /// The ad had the SSL attribute declared but was not SSL-compliant. The SSL attribute was removed.
    ///
    /// "SSL_ATTRIBUTE_REMOVED"
    #[serde(rename="SSL_ATTRIBUTE_REMOVED")]
    SSLATTRIBUTEREMOVED,
    

    /// The ad was declared as Flash-free but contained Flash, so the Flash-free attribute was removed.
    ///
    /// "FLASH_FREE_ATTRIBUTE_REMOVED"
    #[serde(rename="FLASH_FREE_ATTRIBUTE_REMOVED")]
    FLASHFREEATTRIBUTEREMOVED,
    

    /// The ad was not declared as Flash-free but it did not reference any flash content, so the Flash-free attribute was added.
    ///
    /// "FLASH_FREE_ATTRIBUTE_ADDED"
    #[serde(rename="FLASH_FREE_ATTRIBUTE_ADDED")]
    FLASHFREEATTRIBUTEADDED,
    

    /// The ad did not declare a required creative attribute. The attribute was added.
    ///
    /// "REQUIRED_ATTRIBUTE_ADDED"
    #[serde(rename="REQUIRED_ATTRIBUTE_ADDED")]
    REQUIREDATTRIBUTEADDED,
    

    /// The ad did not declare a required technology vendor. The technology vendor was added.
    ///
    /// "REQUIRED_VENDOR_ADDED"
    #[serde(rename="REQUIRED_VENDOR_ADDED")]
    REQUIREDVENDORADDED,
    

    /// The ad did not declare the SSL attribute but was SSL-compliant, so the SSL attribute was added.
    ///
    /// "SSL_ATTRIBUTE_ADDED"
    #[serde(rename="SSL_ATTRIBUTE_ADDED")]
    SSLATTRIBUTEADDED,
    

    /// Properties consistent with In-banner video were found, so an In-Banner Video attribute was added.
    ///
    /// "IN_BANNER_VIDEO_ATTRIBUTE_ADDED"
    #[serde(rename="IN_BANNER_VIDEO_ATTRIBUTE_ADDED")]
    INBANNERVIDEOATTRIBUTEADDED,
    

    /// The ad makes calls to the MRAID API so the MRAID attribute was added.
    ///
    /// "MRAID_ATTRIBUTE_ADDED"
    #[serde(rename="MRAID_ATTRIBUTE_ADDED")]
    MRAIDATTRIBUTEADDED,
    

    /// The ad unnecessarily declared the Flash attribute, so the Flash attribute was removed.
    ///
    /// "FLASH_ATTRIBUTE_REMOVED"
    #[serde(rename="FLASH_ATTRIBUTE_REMOVED")]
    FLASHATTRIBUTEREMOVED,
    

    /// The ad contains video content.
    ///
    /// "VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED"
    #[serde(rename="VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED")]
    VIDEOINSNIPPETATTRIBUTEADDED,
}

impl AsRef<str> for CorrectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CorrectionTypeEnum::CORRECTIONTYPEUNSPECIFIED => "CORRECTION_TYPE_UNSPECIFIED",
            CorrectionTypeEnum::VENDORIDSADDED => "VENDOR_IDS_ADDED",
            CorrectionTypeEnum::SSLATTRIBUTEREMOVED => "SSL_ATTRIBUTE_REMOVED",
            CorrectionTypeEnum::FLASHFREEATTRIBUTEREMOVED => "FLASH_FREE_ATTRIBUTE_REMOVED",
            CorrectionTypeEnum::FLASHFREEATTRIBUTEADDED => "FLASH_FREE_ATTRIBUTE_ADDED",
            CorrectionTypeEnum::REQUIREDATTRIBUTEADDED => "REQUIRED_ATTRIBUTE_ADDED",
            CorrectionTypeEnum::REQUIREDVENDORADDED => "REQUIRED_VENDOR_ADDED",
            CorrectionTypeEnum::SSLATTRIBUTEADDED => "SSL_ATTRIBUTE_ADDED",
            CorrectionTypeEnum::INBANNERVIDEOATTRIBUTEADDED => "IN_BANNER_VIDEO_ATTRIBUTE_ADDED",
            CorrectionTypeEnum::MRAIDATTRIBUTEADDED => "MRAID_ATTRIBUTE_ADDED",
            CorrectionTypeEnum::FLASHATTRIBUTEREMOVED => "FLASH_ATTRIBUTE_REMOVED",
            CorrectionTypeEnum::VIDEOINSNIPPETATTRIBUTEADDED => "VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED",
        }
    }
}

impl std::convert::TryFrom< &str> for CorrectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CORRECTION_TYPE_UNSPECIFIED" => Ok(CorrectionTypeEnum::CORRECTIONTYPEUNSPECIFIED),
           "VENDOR_IDS_ADDED" => Ok(CorrectionTypeEnum::VENDORIDSADDED),
           "SSL_ATTRIBUTE_REMOVED" => Ok(CorrectionTypeEnum::SSLATTRIBUTEREMOVED),
           "FLASH_FREE_ATTRIBUTE_REMOVED" => Ok(CorrectionTypeEnum::FLASHFREEATTRIBUTEREMOVED),
           "FLASH_FREE_ATTRIBUTE_ADDED" => Ok(CorrectionTypeEnum::FLASHFREEATTRIBUTEADDED),
           "REQUIRED_ATTRIBUTE_ADDED" => Ok(CorrectionTypeEnum::REQUIREDATTRIBUTEADDED),
           "REQUIRED_VENDOR_ADDED" => Ok(CorrectionTypeEnum::REQUIREDVENDORADDED),
           "SSL_ATTRIBUTE_ADDED" => Ok(CorrectionTypeEnum::SSLATTRIBUTEADDED),
           "IN_BANNER_VIDEO_ATTRIBUTE_ADDED" => Ok(CorrectionTypeEnum::INBANNERVIDEOATTRIBUTEADDED),
           "MRAID_ATTRIBUTE_ADDED" => Ok(CorrectionTypeEnum::MRAIDATTRIBUTEADDED),
           "FLASH_ATTRIBUTE_REMOVED" => Ok(CorrectionTypeEnum::FLASHATTRIBUTEREMOVED),
           "VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED" => Ok(CorrectionTypeEnum::VIDEOINSNIPPETATTRIBUTEADDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CorrectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeAttributesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method.
pub enum CreativeAttributesEnum {
    

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

impl AsRef<str> for CreativeAttributesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeAttributesEnum::ATTRIBUTEUNSPECIFIED => "ATTRIBUTE_UNSPECIFIED",
            CreativeAttributesEnum::IMAGERICHMEDIA => "IMAGE_RICH_MEDIA",
            CreativeAttributesEnum::ADOBEFLASHFLV => "ADOBE_FLASH_FLV",
            CreativeAttributesEnum::ISTAGGED => "IS_TAGGED",
            CreativeAttributesEnum::ISCOOKIETARGETED => "IS_COOKIE_TARGETED",
            CreativeAttributesEnum::ISUSERINTERESTTARGETED => "IS_USER_INTEREST_TARGETED",
            CreativeAttributesEnum::EXPANDINGDIRECTIONNONE => "EXPANDING_DIRECTION_NONE",
            CreativeAttributesEnum::EXPANDINGDIRECTIONUP => "EXPANDING_DIRECTION_UP",
            CreativeAttributesEnum::EXPANDINGDIRECTIONDOWN => "EXPANDING_DIRECTION_DOWN",
            CreativeAttributesEnum::EXPANDINGDIRECTIONLEFT => "EXPANDING_DIRECTION_LEFT",
            CreativeAttributesEnum::EXPANDINGDIRECTIONRIGHT => "EXPANDING_DIRECTION_RIGHT",
            CreativeAttributesEnum::EXPANDINGDIRECTIONUPLEFT => "EXPANDING_DIRECTION_UP_LEFT",
            CreativeAttributesEnum::EXPANDINGDIRECTIONUPRIGHT => "EXPANDING_DIRECTION_UP_RIGHT",
            CreativeAttributesEnum::EXPANDINGDIRECTIONDOWNLEFT => "EXPANDING_DIRECTION_DOWN_LEFT",
            CreativeAttributesEnum::EXPANDINGDIRECTIONDOWNRIGHT => "EXPANDING_DIRECTION_DOWN_RIGHT",
            CreativeAttributesEnum::CREATIVETYPEHTML => "CREATIVE_TYPE_HTML",
            CreativeAttributesEnum::CREATIVETYPEVASTVIDEO => "CREATIVE_TYPE_VAST_VIDEO",
            CreativeAttributesEnum::EXPANDINGDIRECTIONUPORDOWN => "EXPANDING_DIRECTION_UP_OR_DOWN",
            CreativeAttributesEnum::EXPANDINGDIRECTIONLEFTORRIGHT => "EXPANDING_DIRECTION_LEFT_OR_RIGHT",
            CreativeAttributesEnum::EXPANDINGDIRECTIONANYDIAGONAL => "EXPANDING_DIRECTION_ANY_DIAGONAL",
            CreativeAttributesEnum::EXPANDINGACTIONROLLOVERTOEXPAND => "EXPANDING_ACTION_ROLLOVER_TO_EXPAND",
            CreativeAttributesEnum::INSTREAMVASTVIDEOTYPEVPAIDFLASH => "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH",
            CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEMRAID => "RICH_MEDIA_CAPABILITY_TYPE_MRAID",
            CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEFLASH => "RICH_MEDIA_CAPABILITY_TYPE_FLASH",
            CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEHTML5 => "RICH_MEDIA_CAPABILITY_TYPE_HTML5",
            CreativeAttributesEnum::SKIPPABLEINSTREAMVIDEO => "SKIPPABLE_INSTREAM_VIDEO",
            CreativeAttributesEnum::RICHMEDIACAPABILITYTYPESSL => "RICH_MEDIA_CAPABILITY_TYPE_SSL",
            CreativeAttributesEnum::RICHMEDIACAPABILITYTYPENONSSL => "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL",
            CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEINTERSTITIAL => "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL",
            CreativeAttributesEnum::NONSKIPPABLEINSTREAMVIDEO => "NON_SKIPPABLE_INSTREAM_VIDEO",
            CreativeAttributesEnum::NATIVEELIGIBILITYELIGIBLE => "NATIVE_ELIGIBILITY_ELIGIBLE",
            CreativeAttributesEnum::NONVPAID => "NON_VPAID",
            CreativeAttributesEnum::NATIVEELIGIBILITYNOTELIGIBLE => "NATIVE_ELIGIBILITY_NOT_ELIGIBLE",
            CreativeAttributesEnum::ANYINTERSTITIAL => "ANY_INTERSTITIAL",
            CreativeAttributesEnum::NONINTERSTITIAL => "NON_INTERSTITIAL",
            CreativeAttributesEnum::INBANNERVIDEO => "IN_BANNER_VIDEO",
            CreativeAttributesEnum::RENDERINGSIZELESSADX => "RENDERING_SIZELESS_ADX",
            CreativeAttributesEnum::OMSDK10 => "OMSDK_1_0",
            CreativeAttributesEnum::RENDERINGPLAYABLE => "RENDERING_PLAYABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeAttributesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ATTRIBUTE_UNSPECIFIED" => Ok(CreativeAttributesEnum::ATTRIBUTEUNSPECIFIED),
           "IMAGE_RICH_MEDIA" => Ok(CreativeAttributesEnum::IMAGERICHMEDIA),
           "ADOBE_FLASH_FLV" => Ok(CreativeAttributesEnum::ADOBEFLASHFLV),
           "IS_TAGGED" => Ok(CreativeAttributesEnum::ISTAGGED),
           "IS_COOKIE_TARGETED" => Ok(CreativeAttributesEnum::ISCOOKIETARGETED),
           "IS_USER_INTEREST_TARGETED" => Ok(CreativeAttributesEnum::ISUSERINTERESTTARGETED),
           "EXPANDING_DIRECTION_NONE" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONNONE),
           "EXPANDING_DIRECTION_UP" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONUP),
           "EXPANDING_DIRECTION_DOWN" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONDOWN),
           "EXPANDING_DIRECTION_LEFT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONLEFT),
           "EXPANDING_DIRECTION_RIGHT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONRIGHT),
           "EXPANDING_DIRECTION_UP_LEFT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONUPLEFT),
           "EXPANDING_DIRECTION_UP_RIGHT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONUPRIGHT),
           "EXPANDING_DIRECTION_DOWN_LEFT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONDOWNLEFT),
           "EXPANDING_DIRECTION_DOWN_RIGHT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONDOWNRIGHT),
           "CREATIVE_TYPE_HTML" => Ok(CreativeAttributesEnum::CREATIVETYPEHTML),
           "CREATIVE_TYPE_VAST_VIDEO" => Ok(CreativeAttributesEnum::CREATIVETYPEVASTVIDEO),
           "EXPANDING_DIRECTION_UP_OR_DOWN" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONUPORDOWN),
           "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONLEFTORRIGHT),
           "EXPANDING_DIRECTION_ANY_DIAGONAL" => Ok(CreativeAttributesEnum::EXPANDINGDIRECTIONANYDIAGONAL),
           "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => Ok(CreativeAttributesEnum::EXPANDINGACTIONROLLOVERTOEXPAND),
           "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => Ok(CreativeAttributesEnum::INSTREAMVASTVIDEOTYPEVPAIDFLASH),
           "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => Ok(CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEMRAID),
           "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => Ok(CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEFLASH),
           "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => Ok(CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEHTML5),
           "SKIPPABLE_INSTREAM_VIDEO" => Ok(CreativeAttributesEnum::SKIPPABLEINSTREAMVIDEO),
           "RICH_MEDIA_CAPABILITY_TYPE_SSL" => Ok(CreativeAttributesEnum::RICHMEDIACAPABILITYTYPESSL),
           "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => Ok(CreativeAttributesEnum::RICHMEDIACAPABILITYTYPENONSSL),
           "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => Ok(CreativeAttributesEnum::RICHMEDIACAPABILITYTYPEINTERSTITIAL),
           "NON_SKIPPABLE_INSTREAM_VIDEO" => Ok(CreativeAttributesEnum::NONSKIPPABLEINSTREAMVIDEO),
           "NATIVE_ELIGIBILITY_ELIGIBLE" => Ok(CreativeAttributesEnum::NATIVEELIGIBILITYELIGIBLE),
           "NON_VPAID" => Ok(CreativeAttributesEnum::NONVPAID),
           "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => Ok(CreativeAttributesEnum::NATIVEELIGIBILITYNOTELIGIBLE),
           "ANY_INTERSTITIAL" => Ok(CreativeAttributesEnum::ANYINTERSTITIAL),
           "NON_INTERSTITIAL" => Ok(CreativeAttributesEnum::NONINTERSTITIAL),
           "IN_BANNER_VIDEO" => Ok(CreativeAttributesEnum::INBANNERVIDEO),
           "RENDERING_SIZELESS_ADX" => Ok(CreativeAttributesEnum::RENDERINGSIZELESSADX),
           "OMSDK_1_0" => Ok(CreativeAttributesEnum::OMSDK10),
           "RENDERING_PLAYABLE" => Ok(CreativeAttributesEnum::RENDERINGPLAYABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeAttributesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeDealsStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The top-level deals status of this creative. If disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method.
pub enum CreativeDealsStatusEnum {
    

    /// The status is unknown.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The creative has not been checked.
    ///
    /// "NOT_CHECKED"
    #[serde(rename="NOT_CHECKED")]
    NOTCHECKED,
    

    /// The creative has been conditionally approved. See serving_restrictions for details.
    ///
    /// "CONDITIONALLY_APPROVED"
    #[serde(rename="CONDITIONALLY_APPROVED")]
    CONDITIONALLYAPPROVED,
    

    /// The creative has been approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// The creative has been disapproved.
    ///
    /// "DISAPPROVED"
    #[serde(rename="DISAPPROVED")]
    DISAPPROVED,
    

    /// Placeholder for transition to v1beta1. Currently not used.
    ///
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
    

    /// Placeholder for transition to v1beta1. Currently not used.
    ///
    /// "STATUS_TYPE_UNSPECIFIED"
    #[serde(rename="STATUS_TYPE_UNSPECIFIED")]
    STATUSTYPEUNSPECIFIED,
}

impl AsRef<str> for CreativeDealsStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeDealsStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            CreativeDealsStatusEnum::NOTCHECKED => "NOT_CHECKED",
            CreativeDealsStatusEnum::CONDITIONALLYAPPROVED => "CONDITIONALLY_APPROVED",
            CreativeDealsStatusEnum::APPROVED => "APPROVED",
            CreativeDealsStatusEnum::DISAPPROVED => "DISAPPROVED",
            CreativeDealsStatusEnum::PENDINGREVIEW => "PENDING_REVIEW",
            CreativeDealsStatusEnum::STATUSTYPEUNSPECIFIED => "STATUS_TYPE_UNSPECIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeDealsStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(CreativeDealsStatusEnum::STATUSUNSPECIFIED),
           "NOT_CHECKED" => Ok(CreativeDealsStatusEnum::NOTCHECKED),
           "CONDITIONALLY_APPROVED" => Ok(CreativeDealsStatusEnum::CONDITIONALLYAPPROVED),
           "APPROVED" => Ok(CreativeDealsStatusEnum::APPROVED),
           "DISAPPROVED" => Ok(CreativeDealsStatusEnum::DISAPPROVED),
           "PENDING_REVIEW" => Ok(CreativeDealsStatusEnum::PENDINGREVIEW),
           "STATUS_TYPE_UNSPECIFIED" => Ok(CreativeDealsStatusEnum::STATUSTYPEUNSPECIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeDealsStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeOpenAuctionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The top-level open auction status of this creative. If disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method.
pub enum CreativeOpenAuctionStatusEnum {
    

    /// The status is unknown.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The creative has not been checked.
    ///
    /// "NOT_CHECKED"
    #[serde(rename="NOT_CHECKED")]
    NOTCHECKED,
    

    /// The creative has been conditionally approved. See serving_restrictions for details.
    ///
    /// "CONDITIONALLY_APPROVED"
    #[serde(rename="CONDITIONALLY_APPROVED")]
    CONDITIONALLYAPPROVED,
    

    /// The creative has been approved.
    ///
    /// "APPROVED"
    #[serde(rename="APPROVED")]
    APPROVED,
    

    /// The creative has been disapproved.
    ///
    /// "DISAPPROVED"
    #[serde(rename="DISAPPROVED")]
    DISAPPROVED,
    

    /// Placeholder for transition to v1beta1. Currently not used.
    ///
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
    

    /// Placeholder for transition to v1beta1. Currently not used.
    ///
    /// "STATUS_TYPE_UNSPECIFIED"
    #[serde(rename="STATUS_TYPE_UNSPECIFIED")]
    STATUSTYPEUNSPECIFIED,
}

impl AsRef<str> for CreativeOpenAuctionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeOpenAuctionStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            CreativeOpenAuctionStatusEnum::NOTCHECKED => "NOT_CHECKED",
            CreativeOpenAuctionStatusEnum::CONDITIONALLYAPPROVED => "CONDITIONALLY_APPROVED",
            CreativeOpenAuctionStatusEnum::APPROVED => "APPROVED",
            CreativeOpenAuctionStatusEnum::DISAPPROVED => "DISAPPROVED",
            CreativeOpenAuctionStatusEnum::PENDINGREVIEW => "PENDING_REVIEW",
            CreativeOpenAuctionStatusEnum::STATUSTYPEUNSPECIFIED => "STATUS_TYPE_UNSPECIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeOpenAuctionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(CreativeOpenAuctionStatusEnum::STATUSUNSPECIFIED),
           "NOT_CHECKED" => Ok(CreativeOpenAuctionStatusEnum::NOTCHECKED),
           "CONDITIONALLY_APPROVED" => Ok(CreativeOpenAuctionStatusEnum::CONDITIONALLYAPPROVED),
           "APPROVED" => Ok(CreativeOpenAuctionStatusEnum::APPROVED),
           "DISAPPROVED" => Ok(CreativeOpenAuctionStatusEnum::DISAPPROVED),
           "PENDING_REVIEW" => Ok(CreativeOpenAuctionStatusEnum::PENDINGREVIEW),
           "STATUS_TYPE_UNSPECIFIED" => Ok(CreativeOpenAuctionStatusEnum::STATUSTYPEUNSPECIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeOpenAuctionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRestrictedCategoriesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All restricted categories for the ads that may be shown from this creative.
pub enum CreativeRestrictedCategoriesEnum {
    

    /// The ad has no restricted categories
    ///
    /// "NO_RESTRICTED_CATEGORIES"
    #[serde(rename="NO_RESTRICTED_CATEGORIES")]
    NORESTRICTEDCATEGORIES,
    

    /// The alcohol restricted category.
    ///
    /// "ALCOHOL"
    #[serde(rename="ALCOHOL")]
    ALCOHOL,
}

impl AsRef<str> for CreativeRestrictedCategoriesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRestrictedCategoriesEnum::NORESTRICTEDCATEGORIES => "NO_RESTRICTED_CATEGORIES",
            CreativeRestrictedCategoriesEnum::ALCOHOL => "ALCOHOL",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRestrictedCategoriesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_RESTRICTED_CATEGORIES" => Ok(CreativeRestrictedCategoriesEnum::NORESTRICTEDCATEGORIES),
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


// region CreativeRestrictionCreativeFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The format of the environment that the creatives will be displayed in.
pub enum CreativeRestrictionCreativeFormatEnum {
    

    /// A placeholder for an undefined creative format.
    ///
    /// "CREATIVE_FORMAT_UNSPECIFIED"
    #[serde(rename="CREATIVE_FORMAT_UNSPECIFIED")]
    CREATIVEFORMATUNSPECIFIED,
    

    /// A creative that will be displayed in environments such as a browser.
    ///
    /// "DISPLAY"
    #[serde(rename="DISPLAY")]
    DISPLAY,
    

    /// A video creative that will be displayed in environments such as a video player.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
}

impl AsRef<str> for CreativeRestrictionCreativeFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRestrictionCreativeFormatEnum::CREATIVEFORMATUNSPECIFIED => "CREATIVE_FORMAT_UNSPECIFIED",
            CreativeRestrictionCreativeFormatEnum::DISPLAY => "DISPLAY",
            CreativeRestrictionCreativeFormatEnum::VIDEO => "VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRestrictionCreativeFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_FORMAT_UNSPECIFIED" => Ok(CreativeRestrictionCreativeFormatEnum::CREATIVEFORMATUNSPECIFIED),
           "DISPLAY" => Ok(CreativeRestrictionCreativeFormatEnum::DISPLAY),
           "VIDEO" => Ok(CreativeRestrictionCreativeFormatEnum::VIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRestrictionCreativeFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeRestrictionSkippableAdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Skippable video ads allow viewers to skip ads after 5 seconds.
pub enum CreativeRestrictionSkippableAdTypeEnum {
    

    /// A placeholder for an undefined skippable ad type.
    ///
    /// "SKIPPABLE_AD_TYPE_UNSPECIFIED"
    #[serde(rename="SKIPPABLE_AD_TYPE_UNSPECIFIED")]
    SKIPPABLEADTYPEUNSPECIFIED,
    

    /// This video ad can be skipped after 5 seconds.
    ///
    /// "SKIPPABLE"
    #[serde(rename="SKIPPABLE")]
    SKIPPABLE,
    

    /// This video ad can be skipped after 5 seconds, and is counted as engaged view after 30 seconds. The creative is hosted on YouTube only, and viewcount of the YouTube video increments after the engaged view.
    ///
    /// "INSTREAM_SELECT"
    #[serde(rename="INSTREAM_SELECT")]
    INSTREAMSELECT,
    

    /// This video ad is not skippable.
    ///
    /// "NOT_SKIPPABLE"
    #[serde(rename="NOT_SKIPPABLE")]
    NOTSKIPPABLE,
}

impl AsRef<str> for CreativeRestrictionSkippableAdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeRestrictionSkippableAdTypeEnum::SKIPPABLEADTYPEUNSPECIFIED => "SKIPPABLE_AD_TYPE_UNSPECIFIED",
            CreativeRestrictionSkippableAdTypeEnum::SKIPPABLE => "SKIPPABLE",
            CreativeRestrictionSkippableAdTypeEnum::INSTREAMSELECT => "INSTREAM_SELECT",
            CreativeRestrictionSkippableAdTypeEnum::NOTSKIPPABLE => "NOT_SKIPPABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeRestrictionSkippableAdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SKIPPABLE_AD_TYPE_UNSPECIFIED" => Ok(CreativeRestrictionSkippableAdTypeEnum::SKIPPABLEADTYPEUNSPECIFIED),
           "SKIPPABLE" => Ok(CreativeRestrictionSkippableAdTypeEnum::SKIPPABLE),
           "INSTREAM_SELECT" => Ok(CreativeRestrictionSkippableAdTypeEnum::INSTREAMSELECT),
           "NOT_SKIPPABLE" => Ok(CreativeRestrictionSkippableAdTypeEnum::NOTSKIPPABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeRestrictionSkippableAdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeSizeAllowedFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What formats are allowed by the publisher. If this repeated field is empty then all formats are allowed. For example, if this field contains AllowedFormatType.AUDIO then the publisher only allows an audio ad (without any video).
pub enum CreativeSizeAllowedFormatsEnum {
    

    /// A placeholder for an undefined allowed format.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// An audio-only ad (without any video).
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
}

impl AsRef<str> for CreativeSizeAllowedFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeSizeAllowedFormatsEnum::UNKNOWN => "UNKNOWN",
            CreativeSizeAllowedFormatsEnum::AUDIO => "AUDIO",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeSizeAllowedFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(CreativeSizeAllowedFormatsEnum::UNKNOWN),
           "AUDIO" => Ok(CreativeSizeAllowedFormatsEnum::AUDIO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeSizeAllowedFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeSizeCreativeSizeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The creative size type.
pub enum CreativeSizeCreativeSizeTypeEnum {
    

    /// A placeholder for an undefined creative size type.
    ///
    /// "CREATIVE_SIZE_TYPE_UNSPECIFIED"
    #[serde(rename="CREATIVE_SIZE_TYPE_UNSPECIFIED")]
    CREATIVESIZETYPEUNSPECIFIED,
    

    /// The creative is a regular desktop creative.
    ///
    /// "REGULAR"
    #[serde(rename="REGULAR")]
    REGULAR,
    

    /// The creative is an interstitial creative.
    ///
    /// "INTERSTITIAL"
    #[serde(rename="INTERSTITIAL")]
    INTERSTITIAL,
    

    /// The creative is a video creative.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// The creative is a native (mobile) creative.
    ///
    /// "NATIVE"
    #[serde(rename="NATIVE")]
    NATIVE,
}

impl AsRef<str> for CreativeSizeCreativeSizeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeSizeCreativeSizeTypeEnum::CREATIVESIZETYPEUNSPECIFIED => "CREATIVE_SIZE_TYPE_UNSPECIFIED",
            CreativeSizeCreativeSizeTypeEnum::REGULAR => "REGULAR",
            CreativeSizeCreativeSizeTypeEnum::INTERSTITIAL => "INTERSTITIAL",
            CreativeSizeCreativeSizeTypeEnum::VIDEO => "VIDEO",
            CreativeSizeCreativeSizeTypeEnum::NATIVE => "NATIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeSizeCreativeSizeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_SIZE_TYPE_UNSPECIFIED" => Ok(CreativeSizeCreativeSizeTypeEnum::CREATIVESIZETYPEUNSPECIFIED),
           "REGULAR" => Ok(CreativeSizeCreativeSizeTypeEnum::REGULAR),
           "INTERSTITIAL" => Ok(CreativeSizeCreativeSizeTypeEnum::INTERSTITIAL),
           "VIDEO" => Ok(CreativeSizeCreativeSizeTypeEnum::VIDEO),
           "NATIVE" => Ok(CreativeSizeCreativeSizeTypeEnum::NATIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeSizeCreativeSizeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeSizeNativeTemplateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The native template for this creative. It will have a value only if creative_size_type = CreativeSizeType.NATIVE.
pub enum CreativeSizeNativeTemplateEnum {
    

    /// A placeholder for an undefined native template.
    ///
    /// "UNKNOWN_NATIVE_TEMPLATE"
    #[serde(rename="UNKNOWN_NATIVE_TEMPLATE")]
    UNKNOWNNATIVETEMPLATE,
    

    /// The creative is linked to native content ad.
    ///
    /// "NATIVE_CONTENT_AD"
    #[serde(rename="NATIVE_CONTENT_AD")]
    NATIVECONTENTAD,
    

    /// The creative is linked to native app install ad.
    ///
    /// "NATIVE_APP_INSTALL_AD"
    #[serde(rename="NATIVE_APP_INSTALL_AD")]
    NATIVEAPPINSTALLAD,
    

    /// The creative is linked to native video content ad.
    ///
    /// "NATIVE_VIDEO_CONTENT_AD"
    #[serde(rename="NATIVE_VIDEO_CONTENT_AD")]
    NATIVEVIDEOCONTENTAD,
    

    /// The creative is linked to native video app install ad.
    ///
    /// "NATIVE_VIDEO_APP_INSTALL_AD"
    #[serde(rename="NATIVE_VIDEO_APP_INSTALL_AD")]
    NATIVEVIDEOAPPINSTALLAD,
}

impl AsRef<str> for CreativeSizeNativeTemplateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeSizeNativeTemplateEnum::UNKNOWNNATIVETEMPLATE => "UNKNOWN_NATIVE_TEMPLATE",
            CreativeSizeNativeTemplateEnum::NATIVECONTENTAD => "NATIVE_CONTENT_AD",
            CreativeSizeNativeTemplateEnum::NATIVEAPPINSTALLAD => "NATIVE_APP_INSTALL_AD",
            CreativeSizeNativeTemplateEnum::NATIVEVIDEOCONTENTAD => "NATIVE_VIDEO_CONTENT_AD",
            CreativeSizeNativeTemplateEnum::NATIVEVIDEOAPPINSTALLAD => "NATIVE_VIDEO_APP_INSTALL_AD",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeSizeNativeTemplateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_NATIVE_TEMPLATE" => Ok(CreativeSizeNativeTemplateEnum::UNKNOWNNATIVETEMPLATE),
           "NATIVE_CONTENT_AD" => Ok(CreativeSizeNativeTemplateEnum::NATIVECONTENTAD),
           "NATIVE_APP_INSTALL_AD" => Ok(CreativeSizeNativeTemplateEnum::NATIVEAPPINSTALLAD),
           "NATIVE_VIDEO_CONTENT_AD" => Ok(CreativeSizeNativeTemplateEnum::NATIVEVIDEOCONTENTAD),
           "NATIVE_VIDEO_APP_INSTALL_AD" => Ok(CreativeSizeNativeTemplateEnum::NATIVEVIDEOAPPINSTALLAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeSizeNativeTemplateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CreativeSizeSkippableAdTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of skippable ad for this creative. It will have a value only if creative_size_type = CreativeSizeType.VIDEO.
pub enum CreativeSizeSkippableAdTypeEnum {
    

    /// A placeholder for an undefined skippable ad type.
    ///
    /// "SKIPPABLE_AD_TYPE_UNSPECIFIED"
    #[serde(rename="SKIPPABLE_AD_TYPE_UNSPECIFIED")]
    SKIPPABLEADTYPEUNSPECIFIED,
    

    /// This video ad can be skipped after 5 seconds.
    ///
    /// "GENERIC"
    #[serde(rename="GENERIC")]
    GENERIC,
    

    /// This video ad can be skipped after 5 seconds, and count as engaged view after 30 seconds. The creative is hosted on YouTube only, and viewcount of the YouTube video increments after the engaged view.
    ///
    /// "INSTREAM_SELECT"
    #[serde(rename="INSTREAM_SELECT")]
    INSTREAMSELECT,
    

    /// This video ad is not skippable.
    ///
    /// "NOT_SKIPPABLE"
    #[serde(rename="NOT_SKIPPABLE")]
    NOTSKIPPABLE,
}

impl AsRef<str> for CreativeSizeSkippableAdTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CreativeSizeSkippableAdTypeEnum::SKIPPABLEADTYPEUNSPECIFIED => "SKIPPABLE_AD_TYPE_UNSPECIFIED",
            CreativeSizeSkippableAdTypeEnum::GENERIC => "GENERIC",
            CreativeSizeSkippableAdTypeEnum::INSTREAMSELECT => "INSTREAM_SELECT",
            CreativeSizeSkippableAdTypeEnum::NOTSKIPPABLE => "NOT_SKIPPABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CreativeSizeSkippableAdTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SKIPPABLE_AD_TYPE_UNSPECIFIED" => Ok(CreativeSizeSkippableAdTypeEnum::SKIPPABLEADTYPEUNSPECIFIED),
           "GENERIC" => Ok(CreativeSizeSkippableAdTypeEnum::GENERIC),
           "INSTREAM_SELECT" => Ok(CreativeSizeSkippableAdTypeEnum::INSTREAMSELECT),
           "NOT_SKIPPABLE" => Ok(CreativeSizeSkippableAdTypeEnum::NOTSKIPPABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CreativeSizeSkippableAdTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DayPartDayOfWeekEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The day of the week to target. If unspecified, applicable to all days.
pub enum DayPartDayOfWeekEnum {
    

    /// A placeholder for when the day of the week is not specified.
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
/// The timezone to use for interpreting the day part targeting.
pub enum DayPartTargetingTimeZoneTypeEnum {
    

    /// A placeholder for an undefined time zone source.
    ///
    /// "TIME_ZONE_SOURCE_UNSPECIFIED"
    #[serde(rename="TIME_ZONE_SOURCE_UNSPECIFIED")]
    TIMEZONESOURCEUNSPECIFIED,
    

    /// Use publisher's time zone setting.
    ///
    /// "PUBLISHER"
    #[serde(rename="PUBLISHER")]
    PUBLISHER,
    

    /// Use the user's time zone setting.
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
}

impl AsRef<str> for DayPartTargetingTimeZoneTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DayPartTargetingTimeZoneTypeEnum::TIMEZONESOURCEUNSPECIFIED => "TIME_ZONE_SOURCE_UNSPECIFIED",
            DayPartTargetingTimeZoneTypeEnum::PUBLISHER => "PUBLISHER",
            DayPartTargetingTimeZoneTypeEnum::USER => "USER",
        }
    }
}

impl std::convert::TryFrom< &str> for DayPartTargetingTimeZoneTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_ZONE_SOURCE_UNSPECIFIED" => Ok(DayPartTargetingTimeZoneTypeEnum::TIMEZONESOURCEUNSPECIFIED),
           "PUBLISHER" => Ok(DayPartTargetingTimeZoneTypeEnum::PUBLISHER),
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


// region DealCreativePreApprovalPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the creative pre-approval policy.
pub enum DealCreativePreApprovalPolicyEnum {
    

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

impl AsRef<str> for DealCreativePreApprovalPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealCreativePreApprovalPolicyEnum::CREATIVEPREAPPROVALPOLICYUNSPECIFIED => "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED",
            DealCreativePreApprovalPolicyEnum::SELLERPREAPPROVALREQUIRED => "SELLER_PRE_APPROVAL_REQUIRED",
            DealCreativePreApprovalPolicyEnum::SELLERPREAPPROVALNOTREQUIRED => "SELLER_PRE_APPROVAL_NOT_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for DealCreativePreApprovalPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED" => Ok(DealCreativePreApprovalPolicyEnum::CREATIVEPREAPPROVALPOLICYUNSPECIFIED),
           "SELLER_PRE_APPROVAL_REQUIRED" => Ok(DealCreativePreApprovalPolicyEnum::SELLERPREAPPROVALREQUIRED),
           "SELLER_PRE_APPROVAL_NOT_REQUIRED" => Ok(DealCreativePreApprovalPolicyEnum::SELLERPREAPPROVALNOTREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealCreativePreApprovalPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealCreativeSafeFrameCompatibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies whether the creative is safeFrame compatible.
pub enum DealCreativeSafeFrameCompatibilityEnum {
    

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

impl AsRef<str> for DealCreativeSafeFrameCompatibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealCreativeSafeFrameCompatibilityEnum::CREATIVESAFEFRAMECOMPATIBILITYUNSPECIFIED => "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED",
            DealCreativeSafeFrameCompatibilityEnum::COMPATIBLE => "COMPATIBLE",
            DealCreativeSafeFrameCompatibilityEnum::INCOMPATIBLE => "INCOMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for DealCreativeSafeFrameCompatibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED" => Ok(DealCreativeSafeFrameCompatibilityEnum::CREATIVESAFEFRAMECOMPATIBILITYUNSPECIFIED),
           "COMPATIBLE" => Ok(DealCreativeSafeFrameCompatibilityEnum::COMPATIBLE),
           "INCOMPATIBLE" => Ok(DealCreativeSafeFrameCompatibilityEnum::INCOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealCreativeSafeFrameCompatibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealProgrammaticCreativeSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specifies the creative source for programmatic deals. PUBLISHER means creative is provided by seller and ADVERTISER means creative is provided by buyer.
pub enum DealProgrammaticCreativeSourceEnum {
    

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

impl AsRef<str> for DealProgrammaticCreativeSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealProgrammaticCreativeSourceEnum::PROGRAMMATICCREATIVESOURCEUNSPECIFIED => "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED",
            DealProgrammaticCreativeSourceEnum::ADVERTISER => "ADVERTISER",
            DealProgrammaticCreativeSourceEnum::PUBLISHER => "PUBLISHER",
        }
    }
}

impl std::convert::TryFrom< &str> for DealProgrammaticCreativeSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED" => Ok(DealProgrammaticCreativeSourceEnum::PROGRAMMATICCREATIVESOURCEUNSPECIFIED),
           "ADVERTISER" => Ok(DealProgrammaticCreativeSourceEnum::ADVERTISER),
           "PUBLISHER" => Ok(DealProgrammaticCreativeSourceEnum::PUBLISHER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealProgrammaticCreativeSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealSyndicationProductEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The syndication product associated with the deal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error.
pub enum DealSyndicationProductEnum {
    

    /// A placeholder for an undefined syndication product.
    ///
    /// "SYNDICATION_PRODUCT_UNSPECIFIED"
    #[serde(rename="SYNDICATION_PRODUCT_UNSPECIFIED")]
    SYNDICATIONPRODUCTUNSPECIFIED,
    

    /// This typically represents a web page.
    ///
    /// "CONTENT"
    #[serde(rename="CONTENT")]
    CONTENT,
    

    /// This represents a mobile property.
    ///
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
    

    /// This represents video ad formats.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// This represents ads shown within games.
    ///
    /// "GAMES"
    #[serde(rename="GAMES")]
    GAMES,
}

impl AsRef<str> for DealSyndicationProductEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealSyndicationProductEnum::SYNDICATIONPRODUCTUNSPECIFIED => "SYNDICATION_PRODUCT_UNSPECIFIED",
            DealSyndicationProductEnum::CONTENT => "CONTENT",
            DealSyndicationProductEnum::MOBILE => "MOBILE",
            DealSyndicationProductEnum::VIDEO => "VIDEO",
            DealSyndicationProductEnum::GAMES => "GAMES",
        }
    }
}

impl std::convert::TryFrom< &str> for DealSyndicationProductEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYNDICATION_PRODUCT_UNSPECIFIED" => Ok(DealSyndicationProductEnum::SYNDICATIONPRODUCTUNSPECIFIED),
           "CONTENT" => Ok(DealSyndicationProductEnum::CONTENT),
           "MOBILE" => Ok(DealSyndicationProductEnum::MOBILE),
           "VIDEO" => Ok(DealSyndicationProductEnum::VIDEO),
           "GAMES" => Ok(DealSyndicationProductEnum::GAMES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealSyndicationProductEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealPauseStatusFirstPausedByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The role of the person who first paused this deal.
pub enum DealPauseStatusFirstPausedByEnum {
    

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

impl AsRef<str> for DealPauseStatusFirstPausedByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealPauseStatusFirstPausedByEnum::BUYERSELLERROLEUNSPECIFIED => "BUYER_SELLER_ROLE_UNSPECIFIED",
            DealPauseStatusFirstPausedByEnum::BUYER => "BUYER",
            DealPauseStatusFirstPausedByEnum::SELLER => "SELLER",
        }
    }
}

impl std::convert::TryFrom< &str> for DealPauseStatusFirstPausedByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BUYER_SELLER_ROLE_UNSPECIFIED" => Ok(DealPauseStatusFirstPausedByEnum::BUYERSELLERROLEUNSPECIFIED),
           "BUYER" => Ok(DealPauseStatusFirstPausedByEnum::BUYER),
           "SELLER" => Ok(DealPauseStatusFirstPausedByEnum::SELLER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealPauseStatusFirstPausedByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DealTermBrandingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Visibility of the URL in bid requests. (default: BRANDED)
pub enum DealTermBrandingTypeEnum {
    

    /// A placeholder for an undefined branding type.
    ///
    /// "BRANDING_TYPE_UNSPECIFIED"
    #[serde(rename="BRANDING_TYPE_UNSPECIFIED")]
    BRANDINGTYPEUNSPECIFIED,
    

    /// Full URL is included in bid requests.
    ///
    /// "BRANDED"
    #[serde(rename="BRANDED")]
    BRANDED,
    

    /// A TopLevelDomain or masked URL is sent in bid requests rather than the full one.
    ///
    /// "SEMI_TRANSPARENT"
    #[serde(rename="SEMI_TRANSPARENT")]
    SEMITRANSPARENT,
}

impl AsRef<str> for DealTermBrandingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DealTermBrandingTypeEnum::BRANDINGTYPEUNSPECIFIED => "BRANDING_TYPE_UNSPECIFIED",
            DealTermBrandingTypeEnum::BRANDED => "BRANDED",
            DealTermBrandingTypeEnum::SEMITRANSPARENT => "SEMI_TRANSPARENT",
        }
    }
}

impl std::convert::TryFrom< &str> for DealTermBrandingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BRANDING_TYPE_UNSPECIFIED" => Ok(DealTermBrandingTypeEnum::BRANDINGTYPEUNSPECIFIED),
           "BRANDED" => Ok(DealTermBrandingTypeEnum::BRANDED),
           "SEMI_TRANSPARENT" => Ok(DealTermBrandingTypeEnum::SEMITRANSPARENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DealTermBrandingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeliveryControlCreativeBlockingLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Specified the creative blocking levels to be applied.
pub enum DeliveryControlCreativeBlockingLevelEnum {
    

    /// A placeholder for an undefined creative blocking level.
    ///
    /// "CREATIVE_BLOCKING_LEVEL_UNSPECIFIED"
    #[serde(rename="CREATIVE_BLOCKING_LEVEL_UNSPECIFIED")]
    CREATIVEBLOCKINGLEVELUNSPECIFIED,
    

    /// Publisher blocking rules will be applied.
    ///
    /// "PUBLISHER_BLOCKING_RULES"
    #[serde(rename="PUBLISHER_BLOCKING_RULES")]
    PUBLISHERBLOCKINGRULES,
    

    /// The Ad Exchange policy blocking rules will be applied.
    ///
    /// "ADX_POLICY_BLOCKING_ONLY"
    #[serde(rename="ADX_POLICY_BLOCKING_ONLY")]
    ADXPOLICYBLOCKINGONLY,
}

impl AsRef<str> for DeliveryControlCreativeBlockingLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeliveryControlCreativeBlockingLevelEnum::CREATIVEBLOCKINGLEVELUNSPECIFIED => "CREATIVE_BLOCKING_LEVEL_UNSPECIFIED",
            DeliveryControlCreativeBlockingLevelEnum::PUBLISHERBLOCKINGRULES => "PUBLISHER_BLOCKING_RULES",
            DeliveryControlCreativeBlockingLevelEnum::ADXPOLICYBLOCKINGONLY => "ADX_POLICY_BLOCKING_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for DeliveryControlCreativeBlockingLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREATIVE_BLOCKING_LEVEL_UNSPECIFIED" => Ok(DeliveryControlCreativeBlockingLevelEnum::CREATIVEBLOCKINGLEVELUNSPECIFIED),
           "PUBLISHER_BLOCKING_RULES" => Ok(DeliveryControlCreativeBlockingLevelEnum::PUBLISHERBLOCKINGRULES),
           "ADX_POLICY_BLOCKING_ONLY" => Ok(DeliveryControlCreativeBlockingLevelEnum::ADXPOLICYBLOCKINGONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeliveryControlCreativeBlockingLevelEnum {
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


// region DisapprovalReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The categorized reason for disapproval.
pub enum DisapprovalReasonEnum {
    

    /// The length of the image animation is longer than allowed.
    ///
    /// "LENGTH_OF_IMAGE_ANIMATION"
    #[serde(rename="LENGTH_OF_IMAGE_ANIMATION")]
    LENGTHOFIMAGEANIMATION,
    

    /// The click through URL doesn't work properly.
    ///
    /// "BROKEN_URL"
    #[serde(rename="BROKEN_URL")]
    BROKENURL,
    

    /// Something is wrong with the creative itself.
    ///
    /// "MEDIA_NOT_FUNCTIONAL"
    #[serde(rename="MEDIA_NOT_FUNCTIONAL")]
    MEDIANOTFUNCTIONAL,
    

    /// The ad makes a fourth party call to an unapproved vendor.
    ///
    /// "INVALID_FOURTH_PARTY_CALL"
    #[serde(rename="INVALID_FOURTH_PARTY_CALL")]
    INVALIDFOURTHPARTYCALL,
    

    /// The ad targets consumers using remarketing lists and/or collects data for subsequent use in retargeting, but does not correctly declare that use.
    ///
    /// "INCORRECT_REMARKETING_DECLARATION"
    #[serde(rename="INCORRECT_REMARKETING_DECLARATION")]
    INCORRECTREMARKETINGDECLARATION,
    

    /// Clicking on the ad leads to an error page.
    ///
    /// "LANDING_PAGE_ERROR"
    #[serde(rename="LANDING_PAGE_ERROR")]
    LANDINGPAGEERROR,
    

    /// The ad size when rendered does not match the declaration.
    ///
    /// "AD_SIZE_DOES_NOT_MATCH_AD_SLOT"
    #[serde(rename="AD_SIZE_DOES_NOT_MATCH_AD_SLOT")]
    ADSIZEDOESNOTMATCHADSLOT,
    

    /// Ads with a white background require a border, which was missing.
    ///
    /// "NO_BORDER"
    #[serde(rename="NO_BORDER")]
    NOBORDER,
    

    /// The creative attempts to set cookies from a fourth party that is not certified.
    ///
    /// "FOURTH_PARTY_BROWSER_COOKIES"
    #[serde(rename="FOURTH_PARTY_BROWSER_COOKIES")]
    FOURTHPARTYBROWSERCOOKIES,
    

    /// The creative sets an LSO object.
    ///
    /// "LSO_OBJECTS"
    #[serde(rename="LSO_OBJECTS")]
    LSOOBJECTS,
    

    /// The ad serves a blank.
    ///
    /// "BLANK_CREATIVE"
    #[serde(rename="BLANK_CREATIVE")]
    BLANKCREATIVE,
    

    /// The ad uses rotation, but not all destination URLs were declared.
    ///
    /// "DESTINATION_URLS_UNDECLARED"
    #[serde(rename="DESTINATION_URLS_UNDECLARED")]
    DESTINATIONURLSUNDECLARED,
    

    /// There is a problem with the way the click macro is used.
    ///
    /// "PROBLEM_WITH_CLICK_MACRO"
    #[serde(rename="PROBLEM_WITH_CLICK_MACRO")]
    PROBLEMWITHCLICKMACRO,
    

    /// The ad technology declaration is not accurate.
    ///
    /// "INCORRECT_AD_TECHNOLOGY_DECLARATION"
    #[serde(rename="INCORRECT_AD_TECHNOLOGY_DECLARATION")]
    INCORRECTADTECHNOLOGYDECLARATION,
    

    /// The actual destination URL does not match the declared destination URL.
    ///
    /// "INCORRECT_DESTINATION_URL_DECLARATION"
    #[serde(rename="INCORRECT_DESTINATION_URL_DECLARATION")]
    INCORRECTDESTINATIONURLDECLARATION,
    

    /// The declared expanding direction does not match the actual direction.
    ///
    /// "EXPANDABLE_INCORRECT_DIRECTION"
    #[serde(rename="EXPANDABLE_INCORRECT_DIRECTION")]
    EXPANDABLEINCORRECTDIRECTION,
    

    /// The ad does not expand in a supported direction.
    ///
    /// "EXPANDABLE_DIRECTION_NOT_SUPPORTED"
    #[serde(rename="EXPANDABLE_DIRECTION_NOT_SUPPORTED")]
    EXPANDABLEDIRECTIONNOTSUPPORTED,
    

    /// The ad uses an expandable vendor that is not supported.
    ///
    /// "EXPANDABLE_INVALID_VENDOR"
    #[serde(rename="EXPANDABLE_INVALID_VENDOR")]
    EXPANDABLEINVALIDVENDOR,
    

    /// There was an issue with the expandable ad.
    ///
    /// "EXPANDABLE_FUNCTIONALITY"
    #[serde(rename="EXPANDABLE_FUNCTIONALITY")]
    EXPANDABLEFUNCTIONALITY,
    

    /// The ad uses a video vendor that is not supported.
    ///
    /// "VIDEO_INVALID_VENDOR"
    #[serde(rename="VIDEO_INVALID_VENDOR")]
    VIDEOINVALIDVENDOR,
    

    /// The length of the video ad is not supported.
    ///
    /// "VIDEO_UNSUPPORTED_LENGTH"
    #[serde(rename="VIDEO_UNSUPPORTED_LENGTH")]
    VIDEOUNSUPPORTEDLENGTH,
    

    /// The format of the video ad is not supported.
    ///
    /// "VIDEO_UNSUPPORTED_FORMAT"
    #[serde(rename="VIDEO_UNSUPPORTED_FORMAT")]
    VIDEOUNSUPPORTEDFORMAT,
    

    /// There was an issue with the video ad.
    ///
    /// "VIDEO_FUNCTIONALITY"
    #[serde(rename="VIDEO_FUNCTIONALITY")]
    VIDEOFUNCTIONALITY,
    

    /// The landing page does not conform to Ad Exchange policy.
    ///
    /// "LANDING_PAGE_DISABLED"
    #[serde(rename="LANDING_PAGE_DISABLED")]
    LANDINGPAGEDISABLED,
    

    /// The ad or the landing page may contain malware.
    ///
    /// "MALWARE_SUSPECTED"
    #[serde(rename="MALWARE_SUSPECTED")]
    MALWARESUSPECTED,
    

    /// The ad contains adult images or video content.
    ///
    /// "ADULT_IMAGE_OR_VIDEO"
    #[serde(rename="ADULT_IMAGE_OR_VIDEO")]
    ADULTIMAGEORVIDEO,
    

    /// The ad contains text that is unclear or inaccurate.
    ///
    /// "INACCURATE_AD_TEXT"
    #[serde(rename="INACCURATE_AD_TEXT")]
    INACCURATEADTEXT,
    

    /// The ad promotes counterfeit designer goods.
    ///
    /// "COUNTERFEIT_DESIGNER_GOODS"
    #[serde(rename="COUNTERFEIT_DESIGNER_GOODS")]
    COUNTERFEITDESIGNERGOODS,
    

    /// The ad causes a popup window to appear.
    ///
    /// "POP_UP"
    #[serde(rename="POP_UP")]
    POPUP,
    

    /// The creative does not follow policies set for the RTB protocol.
    ///
    /// "INVALID_RTB_PROTOCOL_USAGE"
    #[serde(rename="INVALID_RTB_PROTOCOL_USAGE")]
    INVALIDRTBPROTOCOLUSAGE,
    

    /// The ad contains a URL that uses a numeric IP address for the domain.
    ///
    /// "RAW_IP_ADDRESS_IN_SNIPPET"
    #[serde(rename="RAW_IP_ADDRESS_IN_SNIPPET")]
    RAWIPADDRESSINSNIPPET,
    

    /// The ad or landing page contains unacceptable content because it initiated a software or executable download.
    ///
    /// "UNACCEPTABLE_CONTENT_SOFTWARE"
    #[serde(rename="UNACCEPTABLE_CONTENT_SOFTWARE")]
    UNACCEPTABLECONTENTSOFTWARE,
    

    /// The ad set an unauthorized cookie on a Google domain.
    ///
    /// "UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN"
    #[serde(rename="UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN")]
    UNAUTHORIZEDCOOKIEONGOOGLEDOMAIN,
    

    /// Flash content found when no flash was declared.
    ///
    /// "UNDECLARED_FLASH_OBJECTS"
    #[serde(rename="UNDECLARED_FLASH_OBJECTS")]
    UNDECLAREDFLASHOBJECTS,
    

    /// SSL support declared but not working correctly.
    ///
    /// "INVALID_SSL_DECLARATION"
    #[serde(rename="INVALID_SSL_DECLARATION")]
    INVALIDSSLDECLARATION,
    

    /// Rich Media - Direct Download in Ad (ex. PDF download).
    ///
    /// "DIRECT_DOWNLOAD_IN_AD"
    #[serde(rename="DIRECT_DOWNLOAD_IN_AD")]
    DIRECTDOWNLOADINAD,
    

    /// Maximum download size exceeded.
    ///
    /// "MAXIMUM_DOWNLOAD_SIZE_EXCEEDED"
    #[serde(rename="MAXIMUM_DOWNLOAD_SIZE_EXCEEDED")]
    MAXIMUMDOWNLOADSIZEEXCEEDED,
    

    /// Bad Destination URL: Site Not Crawlable.
    ///
    /// "DESTINATION_URL_SITE_NOT_CRAWLABLE"
    #[serde(rename="DESTINATION_URL_SITE_NOT_CRAWLABLE")]
    DESTINATIONURLSITENOTCRAWLABLE,
    

    /// Bad URL: Legal disapproval.
    ///
    /// "BAD_URL_LEGAL_DISAPPROVAL"
    #[serde(rename="BAD_URL_LEGAL_DISAPPROVAL")]
    BADURLLEGALDISAPPROVAL,
    

    /// Pharmaceuticals, Gambling, Alcohol not allowed and at least one was detected.
    ///
    /// "PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED"
    #[serde(rename="PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED")]
    PHARMAGAMBLINGALCOHOLNOTALLOWED,
    

    /// Dynamic DNS at Destination URL.
    ///
    /// "DYNAMIC_DNS_AT_DESTINATION_URL"
    #[serde(rename="DYNAMIC_DNS_AT_DESTINATION_URL")]
    DYNAMICDNSATDESTINATIONURL,
    

    /// Poor Image / Video Quality.
    ///
    /// "POOR_IMAGE_OR_VIDEO_QUALITY"
    #[serde(rename="POOR_IMAGE_OR_VIDEO_QUALITY")]
    POORIMAGEORVIDEOQUALITY,
    

    /// For example, Image Trick to Click.
    ///
    /// "UNACCEPTABLE_IMAGE_CONTENT"
    #[serde(rename="UNACCEPTABLE_IMAGE_CONTENT")]
    UNACCEPTABLEIMAGECONTENT,
    

    /// Incorrect Image Layout.
    ///
    /// "INCORRECT_IMAGE_LAYOUT"
    #[serde(rename="INCORRECT_IMAGE_LAYOUT")]
    INCORRECTIMAGELAYOUT,
    

    /// Irrelevant Image / Video.
    ///
    /// "IRRELEVANT_IMAGE_OR_VIDEO"
    #[serde(rename="IRRELEVANT_IMAGE_OR_VIDEO")]
    IRRELEVANTIMAGEORVIDEO,
    

    /// Broken back button.
    ///
    /// "DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK"
    #[serde(rename="DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK")]
    DESTINATIONSITEDOESNOTALLOWGOINGBACK,
    

    /// Misleading/Inaccurate claims in ads.
    ///
    /// "MISLEADING_CLAIMS_IN_AD"
    #[serde(rename="MISLEADING_CLAIMS_IN_AD")]
    MISLEADINGCLAIMSINAD,
    

    /// Restricted Products.
    ///
    /// "RESTRICTED_PRODUCTS"
    #[serde(rename="RESTRICTED_PRODUCTS")]
    RESTRICTEDPRODUCTS,
    

    /// Unacceptable content. For example, malware.
    ///
    /// "UNACCEPTABLE_CONTENT"
    #[serde(rename="UNACCEPTABLE_CONTENT")]
    UNACCEPTABLECONTENT,
    

    /// The ad automatically redirects to the destination site without a click, or reports a click when none were made.
    ///
    /// "AUTOMATED_AD_CLICKING"
    #[serde(rename="AUTOMATED_AD_CLICKING")]
    AUTOMATEDADCLICKING,
    

    /// The ad uses URL protocols that do not exist or are not allowed on AdX.
    ///
    /// "INVALID_URL_PROTOCOL"
    #[serde(rename="INVALID_URL_PROTOCOL")]
    INVALIDURLPROTOCOL,
    

    /// Restricted content (for example, alcohol) was found in the ad but not declared.
    ///
    /// "UNDECLARED_RESTRICTED_CONTENT"
    #[serde(rename="UNDECLARED_RESTRICTED_CONTENT")]
    UNDECLAREDRESTRICTEDCONTENT,
    

    /// Violation of the remarketing list policy.
    ///
    /// "INVALID_REMARKETING_LIST_USAGE"
    #[serde(rename="INVALID_REMARKETING_LIST_USAGE")]
    INVALIDREMARKETINGLISTUSAGE,
    

    /// The destination site's robot.txt file prevents it from being crawled.
    ///
    /// "DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT"
    #[serde(rename="DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT")]
    DESTINATIONSITENOTCRAWLABLEROBOTSTXT,
    

    /// Click to download must link to an app.
    ///
    /// "CLICK_TO_DOWNLOAD_NOT_AN_APP"
    #[serde(rename="CLICK_TO_DOWNLOAD_NOT_AN_APP")]
    CLICKTODOWNLOADNOTANAPP,
    

    /// A review extension must be an accurate review.
    ///
    /// "INACCURATE_REVIEW_EXTENSION"
    #[serde(rename="INACCURATE_REVIEW_EXTENSION")]
    INACCURATEREVIEWEXTENSION,
    

    /// Sexually explicit content.
    ///
    /// "SEXUALLY_EXPLICIT_CONTENT"
    #[serde(rename="SEXUALLY_EXPLICIT_CONTENT")]
    SEXUALLYEXPLICITCONTENT,
    

    /// The ad tries to gain an unfair traffic advantage.
    ///
    /// "GAINING_AN_UNFAIR_ADVANTAGE"
    #[serde(rename="GAINING_AN_UNFAIR_ADVANTAGE")]
    GAININGANUNFAIRADVANTAGE,
    

    /// The ad tries to circumvent Google's advertising systems.
    ///
    /// "GAMING_THE_GOOGLE_NETWORK"
    #[serde(rename="GAMING_THE_GOOGLE_NETWORK")]
    GAMINGTHEGOOGLENETWORK,
    

    /// The ad promotes dangerous knives.
    ///
    /// "DANGEROUS_PRODUCTS_KNIVES"
    #[serde(rename="DANGEROUS_PRODUCTS_KNIVES")]
    DANGEROUSPRODUCTSKNIVES,
    

    /// The ad promotes explosives.
    ///
    /// "DANGEROUS_PRODUCTS_EXPLOSIVES"
    #[serde(rename="DANGEROUS_PRODUCTS_EXPLOSIVES")]
    DANGEROUSPRODUCTSEXPLOSIVES,
    

    /// The ad promotes guns & parts.
    ///
    /// "DANGEROUS_PRODUCTS_GUNS"
    #[serde(rename="DANGEROUS_PRODUCTS_GUNS")]
    DANGEROUSPRODUCTSGUNS,
    

    /// The ad promotes recreational drugs/services & related equipment.
    ///
    /// "DANGEROUS_PRODUCTS_DRUGS"
    #[serde(rename="DANGEROUS_PRODUCTS_DRUGS")]
    DANGEROUSPRODUCTSDRUGS,
    

    /// The ad promotes tobacco products/services & related equipment.
    ///
    /// "DANGEROUS_PRODUCTS_TOBACCO"
    #[serde(rename="DANGEROUS_PRODUCTS_TOBACCO")]
    DANGEROUSPRODUCTSTOBACCO,
    

    /// The ad promotes weapons.
    ///
    /// "DANGEROUS_PRODUCTS_WEAPONS"
    #[serde(rename="DANGEROUS_PRODUCTS_WEAPONS")]
    DANGEROUSPRODUCTSWEAPONS,
    

    /// The ad is unclear or irrelevant to the destination site.
    ///
    /// "UNCLEAR_OR_IRRELEVANT_AD"
    #[serde(rename="UNCLEAR_OR_IRRELEVANT_AD")]
    UNCLEARORIRRELEVANTAD,
    

    /// The ad does not meet professional standards.
    ///
    /// "PROFESSIONAL_STANDARDS"
    #[serde(rename="PROFESSIONAL_STANDARDS")]
    PROFESSIONALSTANDARDS,
    

    /// The promotion is unnecessarily difficult to navigate.
    ///
    /// "DYSFUNCTIONAL_PROMOTION"
    #[serde(rename="DYSFUNCTIONAL_PROMOTION")]
    DYSFUNCTIONALPROMOTION,
    

    /// Violation of Google's policy for interest-based ads.
    ///
    /// "INVALID_INTEREST_BASED_AD"
    #[serde(rename="INVALID_INTEREST_BASED_AD")]
    INVALIDINTERESTBASEDAD,
    

    /// Misuse of personal information.
    ///
    /// "MISUSE_OF_PERSONAL_INFORMATION"
    #[serde(rename="MISUSE_OF_PERSONAL_INFORMATION")]
    MISUSEOFPERSONALINFORMATION,
    

    /// Omission of relevant information.
    ///
    /// "OMISSION_OF_RELEVANT_INFORMATION"
    #[serde(rename="OMISSION_OF_RELEVANT_INFORMATION")]
    OMISSIONOFRELEVANTINFORMATION,
    

    /// Unavailable promotions.
    ///
    /// "UNAVAILABLE_PROMOTIONS"
    #[serde(rename="UNAVAILABLE_PROMOTIONS")]
    UNAVAILABLEPROMOTIONS,
    

    /// Misleading or unrealistic promotions.
    ///
    /// "MISLEADING_PROMOTIONS"
    #[serde(rename="MISLEADING_PROMOTIONS")]
    MISLEADINGPROMOTIONS,
    

    /// Offensive or inappropriate content.
    ///
    /// "INAPPROPRIATE_CONTENT"
    #[serde(rename="INAPPROPRIATE_CONTENT")]
    INAPPROPRIATECONTENT,
    

    /// Capitalizing on sensitive events.
    ///
    /// "SENSITIVE_EVENTS"
    #[serde(rename="SENSITIVE_EVENTS")]
    SENSITIVEEVENTS,
    

    /// Shocking content.
    ///
    /// "SHOCKING_CONTENT"
    #[serde(rename="SHOCKING_CONTENT")]
    SHOCKINGCONTENT,
    

    /// Products & Services that enable dishonest behavior.
    ///
    /// "ENABLING_DISHONEST_BEHAVIOR"
    #[serde(rename="ENABLING_DISHONEST_BEHAVIOR")]
    ENABLINGDISHONESTBEHAVIOR,
    

    /// The ad does not meet technical requirements.
    ///
    /// "TECHNICAL_REQUIREMENTS"
    #[serde(rename="TECHNICAL_REQUIREMENTS")]
    TECHNICALREQUIREMENTS,
    

    /// Restricted political content.
    ///
    /// "RESTRICTED_POLITICAL_CONTENT"
    #[serde(rename="RESTRICTED_POLITICAL_CONTENT")]
    RESTRICTEDPOLITICALCONTENT,
    

    /// Unsupported content.
    ///
    /// "UNSUPPORTED_CONTENT"
    #[serde(rename="UNSUPPORTED_CONTENT")]
    UNSUPPORTEDCONTENT,
    

    /// Invalid bidding method.
    ///
    /// "INVALID_BIDDING_METHOD"
    #[serde(rename="INVALID_BIDDING_METHOD")]
    INVALIDBIDDINGMETHOD,
    

    /// Video length exceeds limits.
    ///
    /// "VIDEO_TOO_LONG"
    #[serde(rename="VIDEO_TOO_LONG")]
    VIDEOTOOLONG,
    

    /// Unacceptable content: Japanese healthcare.
    ///
    /// "VIOLATES_JAPANESE_PHARMACY_LAW"
    #[serde(rename="VIOLATES_JAPANESE_PHARMACY_LAW")]
    VIOLATESJAPANESEPHARMACYLAW,
    

    /// Online pharmacy ID required.
    ///
    /// "UNACCREDITED_PET_PHARMACY"
    #[serde(rename="UNACCREDITED_PET_PHARMACY")]
    UNACCREDITEDPETPHARMACY,
    

    /// Unacceptable content: Abortion.
    ///
    /// "ABORTION"
    #[serde(rename="ABORTION")]
    ABORTION,
    

    /// Unacceptable content: Birth control.
    ///
    /// "CONTRACEPTIVES"
    #[serde(rename="CONTRACEPTIVES")]
    CONTRACEPTIVES,
    

    /// Restricted in China.
    ///
    /// "NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA"
    #[serde(rename="NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA")]
    NEEDCERTIFICATESTOADVERTISEINCHINA,
    

    /// Unacceptable content: Korean healthcare.
    ///
    /// "KCDSP_REGISTRATION"
    #[serde(rename="KCDSP_REGISTRATION")]
    KCDSPREGISTRATION,
    

    /// Non-family safe or adult content.
    ///
    /// "NOT_FAMILY_SAFE"
    #[serde(rename="NOT_FAMILY_SAFE")]
    NOTFAMILYSAFE,
    

    /// Clinical trial recruitment.
    ///
    /// "CLINICAL_TRIAL_RECRUITMENT"
    #[serde(rename="CLINICAL_TRIAL_RECRUITMENT")]
    CLINICALTRIALRECRUITMENT,
    

    /// Maximum number of HTTP calls exceeded.
    ///
    /// "MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED"
    #[serde(rename="MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED")]
    MAXIMUMNUMBEROFHTTPCALLSEXCEEDED,
    

    /// Maximum number of cookies exceeded.
    ///
    /// "MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED"
    #[serde(rename="MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED")]
    MAXIMUMNUMBEROFCOOKIESEXCEEDED,
    

    /// Financial service ad does not adhere to specifications.
    ///
    /// "PERSONAL_LOANS"
    #[serde(rename="PERSONAL_LOANS")]
    PERSONALLOANS,
    

    /// Flash content was found in an unsupported context.
    ///
    /// "UNSUPPORTED_FLASH_CONTENT"
    #[serde(rename="UNSUPPORTED_FLASH_CONTENT")]
    UNSUPPORTEDFLASHCONTENT,
    

    /// Misuse by an Open Measurement SDK script.
    ///
    /// "MISUSE_BY_OMID_SCRIPT"
    #[serde(rename="MISUSE_BY_OMID_SCRIPT")]
    MISUSEBYOMIDSCRIPT,
    

    /// Use of an Open Measurement SDK vendor not on approved vendor list.
    ///
    /// "NON_WHITELISTED_OMID_VENDOR"
    #[serde(rename="NON_WHITELISTED_OMID_VENDOR")]
    NONWHITELISTEDOMIDVENDOR,
    

    /// Unacceptable landing page.
    ///
    /// "DESTINATION_EXPERIENCE"
    #[serde(rename="DESTINATION_EXPERIENCE")]
    DESTINATIONEXPERIENCE,
    

    /// Unsupported language.
    ///
    /// "UNSUPPORTED_LANGUAGE"
    #[serde(rename="UNSUPPORTED_LANGUAGE")]
    UNSUPPORTEDLANGUAGE,
    

    /// Non-SSL compliant.
    ///
    /// "NON_SSL_COMPLIANT"
    #[serde(rename="NON_SSL_COMPLIANT")]
    NONSSLCOMPLIANT,
    

    /// Temporary pausing of creative.
    ///
    /// "TEMPORARY_PAUSE"
    #[serde(rename="TEMPORARY_PAUSE")]
    TEMPORARYPAUSE,
    

    /// Promotes services related to bail bonds.
    ///
    /// "BAIL_BONDS"
    #[serde(rename="BAIL_BONDS")]
    BAILBONDS,
    

    /// Promotes speculative and/or experimental medical treatments.
    ///
    /// "EXPERIMENTAL_MEDICAL_TREATMENT"
    #[serde(rename="EXPERIMENTAL_MEDICAL_TREATMENT")]
    EXPERIMENTALMEDICALTREATMENT,
}

impl AsRef<str> for DisapprovalReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DisapprovalReasonEnum::LENGTHOFIMAGEANIMATION => "LENGTH_OF_IMAGE_ANIMATION",
            DisapprovalReasonEnum::BROKENURL => "BROKEN_URL",
            DisapprovalReasonEnum::MEDIANOTFUNCTIONAL => "MEDIA_NOT_FUNCTIONAL",
            DisapprovalReasonEnum::INVALIDFOURTHPARTYCALL => "INVALID_FOURTH_PARTY_CALL",
            DisapprovalReasonEnum::INCORRECTREMARKETINGDECLARATION => "INCORRECT_REMARKETING_DECLARATION",
            DisapprovalReasonEnum::LANDINGPAGEERROR => "LANDING_PAGE_ERROR",
            DisapprovalReasonEnum::ADSIZEDOESNOTMATCHADSLOT => "AD_SIZE_DOES_NOT_MATCH_AD_SLOT",
            DisapprovalReasonEnum::NOBORDER => "NO_BORDER",
            DisapprovalReasonEnum::FOURTHPARTYBROWSERCOOKIES => "FOURTH_PARTY_BROWSER_COOKIES",
            DisapprovalReasonEnum::LSOOBJECTS => "LSO_OBJECTS",
            DisapprovalReasonEnum::BLANKCREATIVE => "BLANK_CREATIVE",
            DisapprovalReasonEnum::DESTINATIONURLSUNDECLARED => "DESTINATION_URLS_UNDECLARED",
            DisapprovalReasonEnum::PROBLEMWITHCLICKMACRO => "PROBLEM_WITH_CLICK_MACRO",
            DisapprovalReasonEnum::INCORRECTADTECHNOLOGYDECLARATION => "INCORRECT_AD_TECHNOLOGY_DECLARATION",
            DisapprovalReasonEnum::INCORRECTDESTINATIONURLDECLARATION => "INCORRECT_DESTINATION_URL_DECLARATION",
            DisapprovalReasonEnum::EXPANDABLEINCORRECTDIRECTION => "EXPANDABLE_INCORRECT_DIRECTION",
            DisapprovalReasonEnum::EXPANDABLEDIRECTIONNOTSUPPORTED => "EXPANDABLE_DIRECTION_NOT_SUPPORTED",
            DisapprovalReasonEnum::EXPANDABLEINVALIDVENDOR => "EXPANDABLE_INVALID_VENDOR",
            DisapprovalReasonEnum::EXPANDABLEFUNCTIONALITY => "EXPANDABLE_FUNCTIONALITY",
            DisapprovalReasonEnum::VIDEOINVALIDVENDOR => "VIDEO_INVALID_VENDOR",
            DisapprovalReasonEnum::VIDEOUNSUPPORTEDLENGTH => "VIDEO_UNSUPPORTED_LENGTH",
            DisapprovalReasonEnum::VIDEOUNSUPPORTEDFORMAT => "VIDEO_UNSUPPORTED_FORMAT",
            DisapprovalReasonEnum::VIDEOFUNCTIONALITY => "VIDEO_FUNCTIONALITY",
            DisapprovalReasonEnum::LANDINGPAGEDISABLED => "LANDING_PAGE_DISABLED",
            DisapprovalReasonEnum::MALWARESUSPECTED => "MALWARE_SUSPECTED",
            DisapprovalReasonEnum::ADULTIMAGEORVIDEO => "ADULT_IMAGE_OR_VIDEO",
            DisapprovalReasonEnum::INACCURATEADTEXT => "INACCURATE_AD_TEXT",
            DisapprovalReasonEnum::COUNTERFEITDESIGNERGOODS => "COUNTERFEIT_DESIGNER_GOODS",
            DisapprovalReasonEnum::POPUP => "POP_UP",
            DisapprovalReasonEnum::INVALIDRTBPROTOCOLUSAGE => "INVALID_RTB_PROTOCOL_USAGE",
            DisapprovalReasonEnum::RAWIPADDRESSINSNIPPET => "RAW_IP_ADDRESS_IN_SNIPPET",
            DisapprovalReasonEnum::UNACCEPTABLECONTENTSOFTWARE => "UNACCEPTABLE_CONTENT_SOFTWARE",
            DisapprovalReasonEnum::UNAUTHORIZEDCOOKIEONGOOGLEDOMAIN => "UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN",
            DisapprovalReasonEnum::UNDECLAREDFLASHOBJECTS => "UNDECLARED_FLASH_OBJECTS",
            DisapprovalReasonEnum::INVALIDSSLDECLARATION => "INVALID_SSL_DECLARATION",
            DisapprovalReasonEnum::DIRECTDOWNLOADINAD => "DIRECT_DOWNLOAD_IN_AD",
            DisapprovalReasonEnum::MAXIMUMDOWNLOADSIZEEXCEEDED => "MAXIMUM_DOWNLOAD_SIZE_EXCEEDED",
            DisapprovalReasonEnum::DESTINATIONURLSITENOTCRAWLABLE => "DESTINATION_URL_SITE_NOT_CRAWLABLE",
            DisapprovalReasonEnum::BADURLLEGALDISAPPROVAL => "BAD_URL_LEGAL_DISAPPROVAL",
            DisapprovalReasonEnum::PHARMAGAMBLINGALCOHOLNOTALLOWED => "PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED",
            DisapprovalReasonEnum::DYNAMICDNSATDESTINATIONURL => "DYNAMIC_DNS_AT_DESTINATION_URL",
            DisapprovalReasonEnum::POORIMAGEORVIDEOQUALITY => "POOR_IMAGE_OR_VIDEO_QUALITY",
            DisapprovalReasonEnum::UNACCEPTABLEIMAGECONTENT => "UNACCEPTABLE_IMAGE_CONTENT",
            DisapprovalReasonEnum::INCORRECTIMAGELAYOUT => "INCORRECT_IMAGE_LAYOUT",
            DisapprovalReasonEnum::IRRELEVANTIMAGEORVIDEO => "IRRELEVANT_IMAGE_OR_VIDEO",
            DisapprovalReasonEnum::DESTINATIONSITEDOESNOTALLOWGOINGBACK => "DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK",
            DisapprovalReasonEnum::MISLEADINGCLAIMSINAD => "MISLEADING_CLAIMS_IN_AD",
            DisapprovalReasonEnum::RESTRICTEDPRODUCTS => "RESTRICTED_PRODUCTS",
            DisapprovalReasonEnum::UNACCEPTABLECONTENT => "UNACCEPTABLE_CONTENT",
            DisapprovalReasonEnum::AUTOMATEDADCLICKING => "AUTOMATED_AD_CLICKING",
            DisapprovalReasonEnum::INVALIDURLPROTOCOL => "INVALID_URL_PROTOCOL",
            DisapprovalReasonEnum::UNDECLAREDRESTRICTEDCONTENT => "UNDECLARED_RESTRICTED_CONTENT",
            DisapprovalReasonEnum::INVALIDREMARKETINGLISTUSAGE => "INVALID_REMARKETING_LIST_USAGE",
            DisapprovalReasonEnum::DESTINATIONSITENOTCRAWLABLEROBOTSTXT => "DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT",
            DisapprovalReasonEnum::CLICKTODOWNLOADNOTANAPP => "CLICK_TO_DOWNLOAD_NOT_AN_APP",
            DisapprovalReasonEnum::INACCURATEREVIEWEXTENSION => "INACCURATE_REVIEW_EXTENSION",
            DisapprovalReasonEnum::SEXUALLYEXPLICITCONTENT => "SEXUALLY_EXPLICIT_CONTENT",
            DisapprovalReasonEnum::GAININGANUNFAIRADVANTAGE => "GAINING_AN_UNFAIR_ADVANTAGE",
            DisapprovalReasonEnum::GAMINGTHEGOOGLENETWORK => "GAMING_THE_GOOGLE_NETWORK",
            DisapprovalReasonEnum::DANGEROUSPRODUCTSKNIVES => "DANGEROUS_PRODUCTS_KNIVES",
            DisapprovalReasonEnum::DANGEROUSPRODUCTSEXPLOSIVES => "DANGEROUS_PRODUCTS_EXPLOSIVES",
            DisapprovalReasonEnum::DANGEROUSPRODUCTSGUNS => "DANGEROUS_PRODUCTS_GUNS",
            DisapprovalReasonEnum::DANGEROUSPRODUCTSDRUGS => "DANGEROUS_PRODUCTS_DRUGS",
            DisapprovalReasonEnum::DANGEROUSPRODUCTSTOBACCO => "DANGEROUS_PRODUCTS_TOBACCO",
            DisapprovalReasonEnum::DANGEROUSPRODUCTSWEAPONS => "DANGEROUS_PRODUCTS_WEAPONS",
            DisapprovalReasonEnum::UNCLEARORIRRELEVANTAD => "UNCLEAR_OR_IRRELEVANT_AD",
            DisapprovalReasonEnum::PROFESSIONALSTANDARDS => "PROFESSIONAL_STANDARDS",
            DisapprovalReasonEnum::DYSFUNCTIONALPROMOTION => "DYSFUNCTIONAL_PROMOTION",
            DisapprovalReasonEnum::INVALIDINTERESTBASEDAD => "INVALID_INTEREST_BASED_AD",
            DisapprovalReasonEnum::MISUSEOFPERSONALINFORMATION => "MISUSE_OF_PERSONAL_INFORMATION",
            DisapprovalReasonEnum::OMISSIONOFRELEVANTINFORMATION => "OMISSION_OF_RELEVANT_INFORMATION",
            DisapprovalReasonEnum::UNAVAILABLEPROMOTIONS => "UNAVAILABLE_PROMOTIONS",
            DisapprovalReasonEnum::MISLEADINGPROMOTIONS => "MISLEADING_PROMOTIONS",
            DisapprovalReasonEnum::INAPPROPRIATECONTENT => "INAPPROPRIATE_CONTENT",
            DisapprovalReasonEnum::SENSITIVEEVENTS => "SENSITIVE_EVENTS",
            DisapprovalReasonEnum::SHOCKINGCONTENT => "SHOCKING_CONTENT",
            DisapprovalReasonEnum::ENABLINGDISHONESTBEHAVIOR => "ENABLING_DISHONEST_BEHAVIOR",
            DisapprovalReasonEnum::TECHNICALREQUIREMENTS => "TECHNICAL_REQUIREMENTS",
            DisapprovalReasonEnum::RESTRICTEDPOLITICALCONTENT => "RESTRICTED_POLITICAL_CONTENT",
            DisapprovalReasonEnum::UNSUPPORTEDCONTENT => "UNSUPPORTED_CONTENT",
            DisapprovalReasonEnum::INVALIDBIDDINGMETHOD => "INVALID_BIDDING_METHOD",
            DisapprovalReasonEnum::VIDEOTOOLONG => "VIDEO_TOO_LONG",
            DisapprovalReasonEnum::VIOLATESJAPANESEPHARMACYLAW => "VIOLATES_JAPANESE_PHARMACY_LAW",
            DisapprovalReasonEnum::UNACCREDITEDPETPHARMACY => "UNACCREDITED_PET_PHARMACY",
            DisapprovalReasonEnum::ABORTION => "ABORTION",
            DisapprovalReasonEnum::CONTRACEPTIVES => "CONTRACEPTIVES",
            DisapprovalReasonEnum::NEEDCERTIFICATESTOADVERTISEINCHINA => "NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA",
            DisapprovalReasonEnum::KCDSPREGISTRATION => "KCDSP_REGISTRATION",
            DisapprovalReasonEnum::NOTFAMILYSAFE => "NOT_FAMILY_SAFE",
            DisapprovalReasonEnum::CLINICALTRIALRECRUITMENT => "CLINICAL_TRIAL_RECRUITMENT",
            DisapprovalReasonEnum::MAXIMUMNUMBEROFHTTPCALLSEXCEEDED => "MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED",
            DisapprovalReasonEnum::MAXIMUMNUMBEROFCOOKIESEXCEEDED => "MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED",
            DisapprovalReasonEnum::PERSONALLOANS => "PERSONAL_LOANS",
            DisapprovalReasonEnum::UNSUPPORTEDFLASHCONTENT => "UNSUPPORTED_FLASH_CONTENT",
            DisapprovalReasonEnum::MISUSEBYOMIDSCRIPT => "MISUSE_BY_OMID_SCRIPT",
            DisapprovalReasonEnum::NONWHITELISTEDOMIDVENDOR => "NON_WHITELISTED_OMID_VENDOR",
            DisapprovalReasonEnum::DESTINATIONEXPERIENCE => "DESTINATION_EXPERIENCE",
            DisapprovalReasonEnum::UNSUPPORTEDLANGUAGE => "UNSUPPORTED_LANGUAGE",
            DisapprovalReasonEnum::NONSSLCOMPLIANT => "NON_SSL_COMPLIANT",
            DisapprovalReasonEnum::TEMPORARYPAUSE => "TEMPORARY_PAUSE",
            DisapprovalReasonEnum::BAILBONDS => "BAIL_BONDS",
            DisapprovalReasonEnum::EXPERIMENTALMEDICALTREATMENT => "EXPERIMENTAL_MEDICAL_TREATMENT",
        }
    }
}

impl std::convert::TryFrom< &str> for DisapprovalReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LENGTH_OF_IMAGE_ANIMATION" => Ok(DisapprovalReasonEnum::LENGTHOFIMAGEANIMATION),
           "BROKEN_URL" => Ok(DisapprovalReasonEnum::BROKENURL),
           "MEDIA_NOT_FUNCTIONAL" => Ok(DisapprovalReasonEnum::MEDIANOTFUNCTIONAL),
           "INVALID_FOURTH_PARTY_CALL" => Ok(DisapprovalReasonEnum::INVALIDFOURTHPARTYCALL),
           "INCORRECT_REMARKETING_DECLARATION" => Ok(DisapprovalReasonEnum::INCORRECTREMARKETINGDECLARATION),
           "LANDING_PAGE_ERROR" => Ok(DisapprovalReasonEnum::LANDINGPAGEERROR),
           "AD_SIZE_DOES_NOT_MATCH_AD_SLOT" => Ok(DisapprovalReasonEnum::ADSIZEDOESNOTMATCHADSLOT),
           "NO_BORDER" => Ok(DisapprovalReasonEnum::NOBORDER),
           "FOURTH_PARTY_BROWSER_COOKIES" => Ok(DisapprovalReasonEnum::FOURTHPARTYBROWSERCOOKIES),
           "LSO_OBJECTS" => Ok(DisapprovalReasonEnum::LSOOBJECTS),
           "BLANK_CREATIVE" => Ok(DisapprovalReasonEnum::BLANKCREATIVE),
           "DESTINATION_URLS_UNDECLARED" => Ok(DisapprovalReasonEnum::DESTINATIONURLSUNDECLARED),
           "PROBLEM_WITH_CLICK_MACRO" => Ok(DisapprovalReasonEnum::PROBLEMWITHCLICKMACRO),
           "INCORRECT_AD_TECHNOLOGY_DECLARATION" => Ok(DisapprovalReasonEnum::INCORRECTADTECHNOLOGYDECLARATION),
           "INCORRECT_DESTINATION_URL_DECLARATION" => Ok(DisapprovalReasonEnum::INCORRECTDESTINATIONURLDECLARATION),
           "EXPANDABLE_INCORRECT_DIRECTION" => Ok(DisapprovalReasonEnum::EXPANDABLEINCORRECTDIRECTION),
           "EXPANDABLE_DIRECTION_NOT_SUPPORTED" => Ok(DisapprovalReasonEnum::EXPANDABLEDIRECTIONNOTSUPPORTED),
           "EXPANDABLE_INVALID_VENDOR" => Ok(DisapprovalReasonEnum::EXPANDABLEINVALIDVENDOR),
           "EXPANDABLE_FUNCTIONALITY" => Ok(DisapprovalReasonEnum::EXPANDABLEFUNCTIONALITY),
           "VIDEO_INVALID_VENDOR" => Ok(DisapprovalReasonEnum::VIDEOINVALIDVENDOR),
           "VIDEO_UNSUPPORTED_LENGTH" => Ok(DisapprovalReasonEnum::VIDEOUNSUPPORTEDLENGTH),
           "VIDEO_UNSUPPORTED_FORMAT" => Ok(DisapprovalReasonEnum::VIDEOUNSUPPORTEDFORMAT),
           "VIDEO_FUNCTIONALITY" => Ok(DisapprovalReasonEnum::VIDEOFUNCTIONALITY),
           "LANDING_PAGE_DISABLED" => Ok(DisapprovalReasonEnum::LANDINGPAGEDISABLED),
           "MALWARE_SUSPECTED" => Ok(DisapprovalReasonEnum::MALWARESUSPECTED),
           "ADULT_IMAGE_OR_VIDEO" => Ok(DisapprovalReasonEnum::ADULTIMAGEORVIDEO),
           "INACCURATE_AD_TEXT" => Ok(DisapprovalReasonEnum::INACCURATEADTEXT),
           "COUNTERFEIT_DESIGNER_GOODS" => Ok(DisapprovalReasonEnum::COUNTERFEITDESIGNERGOODS),
           "POP_UP" => Ok(DisapprovalReasonEnum::POPUP),
           "INVALID_RTB_PROTOCOL_USAGE" => Ok(DisapprovalReasonEnum::INVALIDRTBPROTOCOLUSAGE),
           "RAW_IP_ADDRESS_IN_SNIPPET" => Ok(DisapprovalReasonEnum::RAWIPADDRESSINSNIPPET),
           "UNACCEPTABLE_CONTENT_SOFTWARE" => Ok(DisapprovalReasonEnum::UNACCEPTABLECONTENTSOFTWARE),
           "UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN" => Ok(DisapprovalReasonEnum::UNAUTHORIZEDCOOKIEONGOOGLEDOMAIN),
           "UNDECLARED_FLASH_OBJECTS" => Ok(DisapprovalReasonEnum::UNDECLAREDFLASHOBJECTS),
           "INVALID_SSL_DECLARATION" => Ok(DisapprovalReasonEnum::INVALIDSSLDECLARATION),
           "DIRECT_DOWNLOAD_IN_AD" => Ok(DisapprovalReasonEnum::DIRECTDOWNLOADINAD),
           "MAXIMUM_DOWNLOAD_SIZE_EXCEEDED" => Ok(DisapprovalReasonEnum::MAXIMUMDOWNLOADSIZEEXCEEDED),
           "DESTINATION_URL_SITE_NOT_CRAWLABLE" => Ok(DisapprovalReasonEnum::DESTINATIONURLSITENOTCRAWLABLE),
           "BAD_URL_LEGAL_DISAPPROVAL" => Ok(DisapprovalReasonEnum::BADURLLEGALDISAPPROVAL),
           "PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED" => Ok(DisapprovalReasonEnum::PHARMAGAMBLINGALCOHOLNOTALLOWED),
           "DYNAMIC_DNS_AT_DESTINATION_URL" => Ok(DisapprovalReasonEnum::DYNAMICDNSATDESTINATIONURL),
           "POOR_IMAGE_OR_VIDEO_QUALITY" => Ok(DisapprovalReasonEnum::POORIMAGEORVIDEOQUALITY),
           "UNACCEPTABLE_IMAGE_CONTENT" => Ok(DisapprovalReasonEnum::UNACCEPTABLEIMAGECONTENT),
           "INCORRECT_IMAGE_LAYOUT" => Ok(DisapprovalReasonEnum::INCORRECTIMAGELAYOUT),
           "IRRELEVANT_IMAGE_OR_VIDEO" => Ok(DisapprovalReasonEnum::IRRELEVANTIMAGEORVIDEO),
           "DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK" => Ok(DisapprovalReasonEnum::DESTINATIONSITEDOESNOTALLOWGOINGBACK),
           "MISLEADING_CLAIMS_IN_AD" => Ok(DisapprovalReasonEnum::MISLEADINGCLAIMSINAD),
           "RESTRICTED_PRODUCTS" => Ok(DisapprovalReasonEnum::RESTRICTEDPRODUCTS),
           "UNACCEPTABLE_CONTENT" => Ok(DisapprovalReasonEnum::UNACCEPTABLECONTENT),
           "AUTOMATED_AD_CLICKING" => Ok(DisapprovalReasonEnum::AUTOMATEDADCLICKING),
           "INVALID_URL_PROTOCOL" => Ok(DisapprovalReasonEnum::INVALIDURLPROTOCOL),
           "UNDECLARED_RESTRICTED_CONTENT" => Ok(DisapprovalReasonEnum::UNDECLAREDRESTRICTEDCONTENT),
           "INVALID_REMARKETING_LIST_USAGE" => Ok(DisapprovalReasonEnum::INVALIDREMARKETINGLISTUSAGE),
           "DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT" => Ok(DisapprovalReasonEnum::DESTINATIONSITENOTCRAWLABLEROBOTSTXT),
           "CLICK_TO_DOWNLOAD_NOT_AN_APP" => Ok(DisapprovalReasonEnum::CLICKTODOWNLOADNOTANAPP),
           "INACCURATE_REVIEW_EXTENSION" => Ok(DisapprovalReasonEnum::INACCURATEREVIEWEXTENSION),
           "SEXUALLY_EXPLICIT_CONTENT" => Ok(DisapprovalReasonEnum::SEXUALLYEXPLICITCONTENT),
           "GAINING_AN_UNFAIR_ADVANTAGE" => Ok(DisapprovalReasonEnum::GAININGANUNFAIRADVANTAGE),
           "GAMING_THE_GOOGLE_NETWORK" => Ok(DisapprovalReasonEnum::GAMINGTHEGOOGLENETWORK),
           "DANGEROUS_PRODUCTS_KNIVES" => Ok(DisapprovalReasonEnum::DANGEROUSPRODUCTSKNIVES),
           "DANGEROUS_PRODUCTS_EXPLOSIVES" => Ok(DisapprovalReasonEnum::DANGEROUSPRODUCTSEXPLOSIVES),
           "DANGEROUS_PRODUCTS_GUNS" => Ok(DisapprovalReasonEnum::DANGEROUSPRODUCTSGUNS),
           "DANGEROUS_PRODUCTS_DRUGS" => Ok(DisapprovalReasonEnum::DANGEROUSPRODUCTSDRUGS),
           "DANGEROUS_PRODUCTS_TOBACCO" => Ok(DisapprovalReasonEnum::DANGEROUSPRODUCTSTOBACCO),
           "DANGEROUS_PRODUCTS_WEAPONS" => Ok(DisapprovalReasonEnum::DANGEROUSPRODUCTSWEAPONS),
           "UNCLEAR_OR_IRRELEVANT_AD" => Ok(DisapprovalReasonEnum::UNCLEARORIRRELEVANTAD),
           "PROFESSIONAL_STANDARDS" => Ok(DisapprovalReasonEnum::PROFESSIONALSTANDARDS),
           "DYSFUNCTIONAL_PROMOTION" => Ok(DisapprovalReasonEnum::DYSFUNCTIONALPROMOTION),
           "INVALID_INTEREST_BASED_AD" => Ok(DisapprovalReasonEnum::INVALIDINTERESTBASEDAD),
           "MISUSE_OF_PERSONAL_INFORMATION" => Ok(DisapprovalReasonEnum::MISUSEOFPERSONALINFORMATION),
           "OMISSION_OF_RELEVANT_INFORMATION" => Ok(DisapprovalReasonEnum::OMISSIONOFRELEVANTINFORMATION),
           "UNAVAILABLE_PROMOTIONS" => Ok(DisapprovalReasonEnum::UNAVAILABLEPROMOTIONS),
           "MISLEADING_PROMOTIONS" => Ok(DisapprovalReasonEnum::MISLEADINGPROMOTIONS),
           "INAPPROPRIATE_CONTENT" => Ok(DisapprovalReasonEnum::INAPPROPRIATECONTENT),
           "SENSITIVE_EVENTS" => Ok(DisapprovalReasonEnum::SENSITIVEEVENTS),
           "SHOCKING_CONTENT" => Ok(DisapprovalReasonEnum::SHOCKINGCONTENT),
           "ENABLING_DISHONEST_BEHAVIOR" => Ok(DisapprovalReasonEnum::ENABLINGDISHONESTBEHAVIOR),
           "TECHNICAL_REQUIREMENTS" => Ok(DisapprovalReasonEnum::TECHNICALREQUIREMENTS),
           "RESTRICTED_POLITICAL_CONTENT" => Ok(DisapprovalReasonEnum::RESTRICTEDPOLITICALCONTENT),
           "UNSUPPORTED_CONTENT" => Ok(DisapprovalReasonEnum::UNSUPPORTEDCONTENT),
           "INVALID_BIDDING_METHOD" => Ok(DisapprovalReasonEnum::INVALIDBIDDINGMETHOD),
           "VIDEO_TOO_LONG" => Ok(DisapprovalReasonEnum::VIDEOTOOLONG),
           "VIOLATES_JAPANESE_PHARMACY_LAW" => Ok(DisapprovalReasonEnum::VIOLATESJAPANESEPHARMACYLAW),
           "UNACCREDITED_PET_PHARMACY" => Ok(DisapprovalReasonEnum::UNACCREDITEDPETPHARMACY),
           "ABORTION" => Ok(DisapprovalReasonEnum::ABORTION),
           "CONTRACEPTIVES" => Ok(DisapprovalReasonEnum::CONTRACEPTIVES),
           "NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA" => Ok(DisapprovalReasonEnum::NEEDCERTIFICATESTOADVERTISEINCHINA),
           "KCDSP_REGISTRATION" => Ok(DisapprovalReasonEnum::KCDSPREGISTRATION),
           "NOT_FAMILY_SAFE" => Ok(DisapprovalReasonEnum::NOTFAMILYSAFE),
           "CLINICAL_TRIAL_RECRUITMENT" => Ok(DisapprovalReasonEnum::CLINICALTRIALRECRUITMENT),
           "MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED" => Ok(DisapprovalReasonEnum::MAXIMUMNUMBEROFHTTPCALLSEXCEEDED),
           "MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED" => Ok(DisapprovalReasonEnum::MAXIMUMNUMBEROFCOOKIESEXCEEDED),
           "PERSONAL_LOANS" => Ok(DisapprovalReasonEnum::PERSONALLOANS),
           "UNSUPPORTED_FLASH_CONTENT" => Ok(DisapprovalReasonEnum::UNSUPPORTEDFLASHCONTENT),
           "MISUSE_BY_OMID_SCRIPT" => Ok(DisapprovalReasonEnum::MISUSEBYOMIDSCRIPT),
           "NON_WHITELISTED_OMID_VENDOR" => Ok(DisapprovalReasonEnum::NONWHITELISTEDOMIDVENDOR),
           "DESTINATION_EXPERIENCE" => Ok(DisapprovalReasonEnum::DESTINATIONEXPERIENCE),
           "UNSUPPORTED_LANGUAGE" => Ok(DisapprovalReasonEnum::UNSUPPORTEDLANGUAGE),
           "NON_SSL_COMPLIANT" => Ok(DisapprovalReasonEnum::NONSSLCOMPLIANT),
           "TEMPORARY_PAUSE" => Ok(DisapprovalReasonEnum::TEMPORARYPAUSE),
           "BAIL_BONDS" => Ok(DisapprovalReasonEnum::BAILBONDS),
           "EXPERIMENTAL_MEDICAL_TREATMENT" => Ok(DisapprovalReasonEnum::EXPERIMENTALMEDICALTREATMENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DisapprovalReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterSetBreakdownDimensionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The set of dimensions along which to break down the response; may be empty. If multiple dimensions are requested, the breakdown is along the Cartesian product of the requested dimensions.
pub enum FilterSetBreakdownDimensionsEnum {
    

    /// A placeholder for an unspecified dimension; should not be used.
    ///
    /// "BREAKDOWN_DIMENSION_UNSPECIFIED"
    #[serde(rename="BREAKDOWN_DIMENSION_UNSPECIFIED")]
    BREAKDOWNDIMENSIONUNSPECIFIED,
    

    /// The response should be broken down by publisher identifier. This option is available only for Open Bidding buyers.
    ///
    /// "PUBLISHER_IDENTIFIER"
    #[serde(rename="PUBLISHER_IDENTIFIER")]
    PUBLISHERIDENTIFIER,
}

impl AsRef<str> for FilterSetBreakdownDimensionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterSetBreakdownDimensionsEnum::BREAKDOWNDIMENSIONUNSPECIFIED => "BREAKDOWN_DIMENSION_UNSPECIFIED",
            FilterSetBreakdownDimensionsEnum::PUBLISHERIDENTIFIER => "PUBLISHER_IDENTIFIER",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterSetBreakdownDimensionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BREAKDOWN_DIMENSION_UNSPECIFIED" => Ok(FilterSetBreakdownDimensionsEnum::BREAKDOWNDIMENSIONUNSPECIFIED),
           "PUBLISHER_IDENTIFIER" => Ok(FilterSetBreakdownDimensionsEnum::PUBLISHERIDENTIFIER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterSetBreakdownDimensionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterSetEnvironmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The environment on which to filter; optional.
pub enum FilterSetEnvironmentEnum {
    

    /// A placeholder for an undefined environment; indicates that no environment filter will be applied.
    ///
    /// "ENVIRONMENT_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_UNSPECIFIED")]
    ENVIRONMENTUNSPECIFIED,
    

    /// The ad impression appears on the web.
    ///
    /// "WEB"
    #[serde(rename="WEB")]
    WEB,
    

    /// The ad impression appears in an app.
    ///
    /// "APP"
    #[serde(rename="APP")]
    APP,
}

impl AsRef<str> for FilterSetEnvironmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterSetEnvironmentEnum::ENVIRONMENTUNSPECIFIED => "ENVIRONMENT_UNSPECIFIED",
            FilterSetEnvironmentEnum::WEB => "WEB",
            FilterSetEnvironmentEnum::APP => "APP",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterSetEnvironmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_UNSPECIFIED" => Ok(FilterSetEnvironmentEnum::ENVIRONMENTUNSPECIFIED),
           "WEB" => Ok(FilterSetEnvironmentEnum::WEB),
           "APP" => Ok(FilterSetEnvironmentEnum::APP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterSetEnvironmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterSetFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Creative format bidded on or allowed to bid on, can be empty.
pub enum FilterSetFormatEnum {
    

    /// A placeholder for an undefined format; indicates that no format filter will be applied.
    ///
    /// "FORMAT_UNSPECIFIED"
    #[serde(rename="FORMAT_UNSPECIFIED")]
    FORMATUNSPECIFIED,
    

    /// The ad impression is a native ad, and display (for example, image) format.
    ///
    /// "NATIVE_DISPLAY"
    #[serde(rename="NATIVE_DISPLAY")]
    NATIVEDISPLAY,
    

    /// The ad impression is a native ad, and video format.
    ///
    /// "NATIVE_VIDEO"
    #[serde(rename="NATIVE_VIDEO")]
    NATIVEVIDEO,
    

    /// The ad impression is not a native ad, and display (for example, image) format.
    ///
    /// "NON_NATIVE_DISPLAY"
    #[serde(rename="NON_NATIVE_DISPLAY")]
    NONNATIVEDISPLAY,
    

    /// The ad impression is not a native ad, and video format.
    ///
    /// "NON_NATIVE_VIDEO"
    #[serde(rename="NON_NATIVE_VIDEO")]
    NONNATIVEVIDEO,
}

impl AsRef<str> for FilterSetFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterSetFormatEnum::FORMATUNSPECIFIED => "FORMAT_UNSPECIFIED",
            FilterSetFormatEnum::NATIVEDISPLAY => "NATIVE_DISPLAY",
            FilterSetFormatEnum::NATIVEVIDEO => "NATIVE_VIDEO",
            FilterSetFormatEnum::NONNATIVEDISPLAY => "NON_NATIVE_DISPLAY",
            FilterSetFormatEnum::NONNATIVEVIDEO => "NON_NATIVE_VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterSetFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMAT_UNSPECIFIED" => Ok(FilterSetFormatEnum::FORMATUNSPECIFIED),
           "NATIVE_DISPLAY" => Ok(FilterSetFormatEnum::NATIVEDISPLAY),
           "NATIVE_VIDEO" => Ok(FilterSetFormatEnum::NATIVEVIDEO),
           "NON_NATIVE_DISPLAY" => Ok(FilterSetFormatEnum::NONNATIVEDISPLAY),
           "NON_NATIVE_VIDEO" => Ok(FilterSetFormatEnum::NONNATIVEVIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterSetFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterSetFormatsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Creative formats bidded on or allowed to bid on, can be empty. Although this field is a list, it can only be populated with a single item. A HTTP 400 bad request error will be returned in the response if you specify multiple items.
pub enum FilterSetFormatsEnum {
    

    /// A placeholder for an undefined format; indicates that no format filter will be applied.
    ///
    /// "FORMAT_UNSPECIFIED"
    #[serde(rename="FORMAT_UNSPECIFIED")]
    FORMATUNSPECIFIED,
    

    /// The ad impression is a native ad, and display (for example, image) format.
    ///
    /// "NATIVE_DISPLAY"
    #[serde(rename="NATIVE_DISPLAY")]
    NATIVEDISPLAY,
    

    /// The ad impression is a native ad, and video format.
    ///
    /// "NATIVE_VIDEO"
    #[serde(rename="NATIVE_VIDEO")]
    NATIVEVIDEO,
    

    /// The ad impression is not a native ad, and display (for example, image) format.
    ///
    /// "NON_NATIVE_DISPLAY"
    #[serde(rename="NON_NATIVE_DISPLAY")]
    NONNATIVEDISPLAY,
    

    /// The ad impression is not a native ad, and video format.
    ///
    /// "NON_NATIVE_VIDEO"
    #[serde(rename="NON_NATIVE_VIDEO")]
    NONNATIVEVIDEO,
}

impl AsRef<str> for FilterSetFormatsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterSetFormatsEnum::FORMATUNSPECIFIED => "FORMAT_UNSPECIFIED",
            FilterSetFormatsEnum::NATIVEDISPLAY => "NATIVE_DISPLAY",
            FilterSetFormatsEnum::NATIVEVIDEO => "NATIVE_VIDEO",
            FilterSetFormatsEnum::NONNATIVEDISPLAY => "NON_NATIVE_DISPLAY",
            FilterSetFormatsEnum::NONNATIVEVIDEO => "NON_NATIVE_VIDEO",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterSetFormatsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMAT_UNSPECIFIED" => Ok(FilterSetFormatsEnum::FORMATUNSPECIFIED),
           "NATIVE_DISPLAY" => Ok(FilterSetFormatsEnum::NATIVEDISPLAY),
           "NATIVE_VIDEO" => Ok(FilterSetFormatsEnum::NATIVEVIDEO),
           "NON_NATIVE_DISPLAY" => Ok(FilterSetFormatsEnum::NONNATIVEDISPLAY),
           "NON_NATIVE_VIDEO" => Ok(FilterSetFormatsEnum::NONNATIVEVIDEO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterSetFormatsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterSetPlatformsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The list of platforms on which to filter; may be empty. The filters represented by multiple platforms are ORed together (for example, if non-empty, results must match any one of the platforms).
pub enum FilterSetPlatformsEnum {
    

    /// A placeholder for an undefined platform; indicates that no platform filter will be applied.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// The ad impression appears on a desktop.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// The ad impression appears on a tablet.
    ///
    /// "TABLET"
    #[serde(rename="TABLET")]
    TABLET,
    

    /// The ad impression appears on a mobile device.
    ///
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
}

impl AsRef<str> for FilterSetPlatformsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterSetPlatformsEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            FilterSetPlatformsEnum::DESKTOP => "DESKTOP",
            FilterSetPlatformsEnum::TABLET => "TABLET",
            FilterSetPlatformsEnum::MOBILE => "MOBILE",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterSetPlatformsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(FilterSetPlatformsEnum::PLATFORMUNSPECIFIED),
           "DESKTOP" => Ok(FilterSetPlatformsEnum::DESKTOP),
           "TABLET" => Ok(FilterSetPlatformsEnum::TABLET),
           "MOBILE" => Ok(FilterSetPlatformsEnum::MOBILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterSetPlatformsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterSetTimeSeriesGranularityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The granularity of time intervals if a time series breakdown is preferred; optional.
pub enum FilterSetTimeSeriesGranularityEnum {
    

    /// A placeholder for an unspecified interval; no time series is applied. All rows in response will contain data for the entire requested time range.
    ///
    /// "TIME_SERIES_GRANULARITY_UNSPECIFIED"
    #[serde(rename="TIME_SERIES_GRANULARITY_UNSPECIFIED")]
    TIMESERIESGRANULARITYUNSPECIFIED,
    

    /// Indicates that data will be broken down by the hour.
    ///
    /// "HOURLY"
    #[serde(rename="HOURLY")]
    HOURLY,
    

    /// Indicates that data will be broken down by the day.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
}

impl AsRef<str> for FilterSetTimeSeriesGranularityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterSetTimeSeriesGranularityEnum::TIMESERIESGRANULARITYUNSPECIFIED => "TIME_SERIES_GRANULARITY_UNSPECIFIED",
            FilterSetTimeSeriesGranularityEnum::HOURLY => "HOURLY",
            FilterSetTimeSeriesGranularityEnum::DAILY => "DAILY",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterSetTimeSeriesGranularityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIME_SERIES_GRANULARITY_UNSPECIFIED" => Ok(FilterSetTimeSeriesGranularityEnum::TIMESERIESGRANULARITYUNSPECIFIED),
           "HOURLY" => Ok(FilterSetTimeSeriesGranularityEnum::HOURLY),
           "DAILY" => Ok(FilterSetTimeSeriesGranularityEnum::DAILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterSetTimeSeriesGranularityEnum {
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
    

    /// Minute
    ///
    /// "MINUTE"
    #[serde(rename="MINUTE")]
    MINUTE,
    

    /// Hour
    ///
    /// "HOUR"
    #[serde(rename="HOUR")]
    HOUR,
    

    /// Day
    ///
    /// "DAY"
    #[serde(rename="DAY")]
    DAY,
    

    /// Week
    ///
    /// "WEEK"
    #[serde(rename="WEEK")]
    WEEK,
    

    /// Month
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// Lifetime
    ///
    /// "LIFETIME"
    #[serde(rename="LIFETIME")]
    LIFETIME,
    

    /// Pod
    ///
    /// "POD"
    #[serde(rename="POD")]
    POD,
    

    /// Stream
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


// region GuaranteedFixedPriceTermReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reservation type for a Programmatic Guaranteed deal. This indicates whether the number of impressions is fixed, or a percent of available impressions. If not specified, the default reservation type is STANDARD.
pub enum GuaranteedFixedPriceTermReservationTypeEnum {
    

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

impl AsRef<str> for GuaranteedFixedPriceTermReservationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuaranteedFixedPriceTermReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED => "RESERVATION_TYPE_UNSPECIFIED",
            GuaranteedFixedPriceTermReservationTypeEnum::STANDARD => "STANDARD",
            GuaranteedFixedPriceTermReservationTypeEnum::SPONSORSHIP => "SPONSORSHIP",
        }
    }
}

impl std::convert::TryFrom< &str> for GuaranteedFixedPriceTermReservationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESERVATION_TYPE_UNSPECIFIED" => Ok(GuaranteedFixedPriceTermReservationTypeEnum::RESERVATIONTYPEUNSPECIFIED),
           "STANDARD" => Ok(GuaranteedFixedPriceTermReservationTypeEnum::STANDARD),
           "SPONSORSHIP" => Ok(GuaranteedFixedPriceTermReservationTypeEnum::SPONSORSHIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuaranteedFixedPriceTermReservationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of detail that the detail IDs represent.
pub enum ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum {
    

    /// A placeholder for an undefined status. This value will never be returned in responses.
    ///
    /// "DETAIL_TYPE_UNSPECIFIED"
    #[serde(rename="DETAIL_TYPE_UNSPECIFIED")]
    DETAILTYPEUNSPECIFIED,
    

    /// Indicates that the detail ID refers to a creative attribute; see [publisher-excludable-creative-attributes](https://developers.google.com/authorized-buyers/rtb/downloads/publisher-excludable-creative-attributes).
    ///
    /// "CREATIVE_ATTRIBUTE"
    #[serde(rename="CREATIVE_ATTRIBUTE")]
    CREATIVEATTRIBUTE,
    

    /// Indicates that the detail ID refers to a vendor; see [vendors](https://developers.google.com/authorized-buyers/rtb/downloads/vendors). This namespace is different from that of the `ATP_VENDOR` detail type.
    ///
    /// "VENDOR"
    #[serde(rename="VENDOR")]
    VENDOR,
    

    /// Indicates that the detail ID refers to a sensitive category; see [ad-sensitive-categories](https://developers.google.com/authorized-buyers/rtb/downloads/ad-sensitive-categories).
    ///
    /// "SENSITIVE_CATEGORY"
    #[serde(rename="SENSITIVE_CATEGORY")]
    SENSITIVECATEGORY,
    

    /// Indicates that the detail ID refers to a product category; see [ad-product-categories](https://developers.google.com/authorized-buyers/rtb/downloads/ad-product-categories).
    ///
    /// "PRODUCT_CATEGORY"
    #[serde(rename="PRODUCT_CATEGORY")]
    PRODUCTCATEGORY,
    

    /// Indicates that the detail ID refers to a disapproval reason; see DisapprovalReason enum in [snippet-status-report-proto](https://developers.google.com/authorized-buyers/rtb/downloads/snippet-status-report-proto).
    ///
    /// "DISAPPROVAL_REASON"
    #[serde(rename="DISAPPROVAL_REASON")]
    DISAPPROVALREASON,
    

    /// Indicates that the detail ID refers to a policy topic.
    ///
    /// "POLICY_TOPIC"
    #[serde(rename="POLICY_TOPIC")]
    POLICYTOPIC,
    

    /// Indicates that the detail ID refers to an ad technology provider (ATP); see [providers] (https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv). This namespace is different from the `VENDOR` detail type; see [ad technology providers](https://support.google.com/admanager/answer/9012903) for more information.
    ///
    /// "ATP_VENDOR"
    #[serde(rename="ATP_VENDOR")]
    ATPVENDOR,
    

    /// Indicates that the detail string refers the domain of an unknown vendor.
    ///
    /// "VENDOR_DOMAIN"
    #[serde(rename="VENDOR_DOMAIN")]
    VENDORDOMAIN,
    

    /// Indicates that the detail ID refers an IAB GVL ID which Google did not detect in the latest TCF Vendor List. See [Global Vendor List] (https://vendor-list.consensu.org/v2/vendor-list.json)
    ///
    /// "GVL_ID"
    #[serde(rename="GVL_ID")]
    GVLID,
}

impl AsRef<str> for ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::DETAILTYPEUNSPECIFIED => "DETAIL_TYPE_UNSPECIFIED",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::CREATIVEATTRIBUTE => "CREATIVE_ATTRIBUTE",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::VENDOR => "VENDOR",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::SENSITIVECATEGORY => "SENSITIVE_CATEGORY",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::PRODUCTCATEGORY => "PRODUCT_CATEGORY",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::DISAPPROVALREASON => "DISAPPROVAL_REASON",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::POLICYTOPIC => "POLICY_TOPIC",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::ATPVENDOR => "ATP_VENDOR",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::VENDORDOMAIN => "VENDOR_DOMAIN",
            ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::GVLID => "GVL_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DETAIL_TYPE_UNSPECIFIED" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::DETAILTYPEUNSPECIFIED),
           "CREATIVE_ATTRIBUTE" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::CREATIVEATTRIBUTE),
           "VENDOR" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::VENDOR),
           "SENSITIVE_CATEGORY" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::SENSITIVECATEGORY),
           "PRODUCT_CATEGORY" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::PRODUCTCATEGORY),
           "DISAPPROVAL_REASON" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::DISAPPROVALREASON),
           "POLICY_TOPIC" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::POLICYTOPIC),
           "ATP_VENDOR" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::ATPVENDOR),
           "VENDOR_DOMAIN" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::VENDORDOMAIN),
           "GVL_ID" => Ok(ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum::GVLID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NonBillableWinningBidStatusRowStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status specifying why the winning bids were not billed.
pub enum NonBillableWinningBidStatusRowStatusEnum {
    

    /// A placeholder for an undefined status. This value will never be returned in responses.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The buyer was not billed because the ad was not rendered by the publisher.
    ///
    /// "AD_NOT_RENDERED"
    #[serde(rename="AD_NOT_RENDERED")]
    ADNOTRENDERED,
    

    /// The buyer was not billed because the impression won by the bid was determined to be invalid.
    ///
    /// "INVALID_IMPRESSION"
    #[serde(rename="INVALID_IMPRESSION")]
    INVALIDIMPRESSION,
    

    /// A video impression was served but a fatal error was reported from the client during playback.
    ///
    /// "FATAL_VAST_ERROR"
    #[serde(rename="FATAL_VAST_ERROR")]
    FATALVASTERROR,
    

    /// The buyer was not billed because the ad was outplaced in the mediation waterfall.
    ///
    /// "LOST_IN_MEDIATION"
    #[serde(rename="LOST_IN_MEDIATION")]
    LOSTINMEDIATION,
}

impl AsRef<str> for NonBillableWinningBidStatusRowStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NonBillableWinningBidStatusRowStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            NonBillableWinningBidStatusRowStatusEnum::ADNOTRENDERED => "AD_NOT_RENDERED",
            NonBillableWinningBidStatusRowStatusEnum::INVALIDIMPRESSION => "INVALID_IMPRESSION",
            NonBillableWinningBidStatusRowStatusEnum::FATALVASTERROR => "FATAL_VAST_ERROR",
            NonBillableWinningBidStatusRowStatusEnum::LOSTINMEDIATION => "LOST_IN_MEDIATION",
        }
    }
}

impl std::convert::TryFrom< &str> for NonBillableWinningBidStatusRowStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(NonBillableWinningBidStatusRowStatusEnum::STATUSUNSPECIFIED),
           "AD_NOT_RENDERED" => Ok(NonBillableWinningBidStatusRowStatusEnum::ADNOTRENDERED),
           "INVALID_IMPRESSION" => Ok(NonBillableWinningBidStatusRowStatusEnum::INVALIDIMPRESSION),
           "FATAL_VAST_ERROR" => Ok(NonBillableWinningBidStatusRowStatusEnum::FATALVASTERROR),
           "LOST_IN_MEDIATION" => Ok(NonBillableWinningBidStatusRowStatusEnum::LOSTINMEDIATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NonBillableWinningBidStatusRowStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NoteCreatorRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The role of the person (buyer/seller) creating the note.
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


// region PlatformContextPlatformsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platforms this restriction applies to.
pub enum PlatformContextPlatformsEnum {
    

    /// Desktop platform.
    ///
    /// "DESKTOP"
    #[serde(rename="DESKTOP")]
    DESKTOP,
    

    /// Android platform.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// iOS platform.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
}

impl AsRef<str> for PlatformContextPlatformsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlatformContextPlatformsEnum::DESKTOP => "DESKTOP",
            PlatformContextPlatformsEnum::ANDROID => "ANDROID",
            PlatformContextPlatformsEnum::IOS => "IOS",
        }
    }
}

impl std::convert::TryFrom< &str> for PlatformContextPlatformsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DESKTOP" => Ok(PlatformContextPlatformsEnum::DESKTOP),
           "ANDROID" => Ok(PlatformContextPlatformsEnum::ANDROID),
           "IOS" => Ok(PlatformContextPlatformsEnum::IOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlatformContextPlatformsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PricePricingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The pricing type for the deal/product. (default: CPM)
pub enum PricePricingTypeEnum {
    

    /// A placeholder for an undefined pricing type. If the pricing type is unpsecified, `COST_PER_MILLE` will be used instead.
    ///
    /// "PRICING_TYPE_UNSPECIFIED"
    #[serde(rename="PRICING_TYPE_UNSPECIFIED")]
    PRICINGTYPEUNSPECIFIED,
    

    /// Cost per thousand impressions.
    ///
    /// "COST_PER_MILLE"
    #[serde(rename="COST_PER_MILLE")]
    COSTPERMILLE,
    

    /// Cost per day
    ///
    /// "COST_PER_DAY"
    #[serde(rename="COST_PER_DAY")]
    COSTPERDAY,
}

impl AsRef<str> for PricePricingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PricePricingTypeEnum::PRICINGTYPEUNSPECIFIED => "PRICING_TYPE_UNSPECIFIED",
            PricePricingTypeEnum::COSTPERMILLE => "COST_PER_MILLE",
            PricePricingTypeEnum::COSTPERDAY => "COST_PER_DAY",
        }
    }
}

impl std::convert::TryFrom< &str> for PricePricingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRICING_TYPE_UNSPECIFIED" => Ok(PricePricingTypeEnum::PRICINGTYPEUNSPECIFIED),
           "COST_PER_MILLE" => Ok(PricePricingTypeEnum::COSTPERMILLE),
           "COST_PER_DAY" => Ok(PricePricingTypeEnum::COSTPERDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PricePricingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProductSyndicationProductEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The syndication product associated with the deal.
pub enum ProductSyndicationProductEnum {
    

    /// A placeholder for an undefined syndication product.
    ///
    /// "SYNDICATION_PRODUCT_UNSPECIFIED"
    #[serde(rename="SYNDICATION_PRODUCT_UNSPECIFIED")]
    SYNDICATIONPRODUCTUNSPECIFIED,
    

    /// This typically represents a web page.
    ///
    /// "CONTENT"
    #[serde(rename="CONTENT")]
    CONTENT,
    

    /// This represents a mobile property.
    ///
    /// "MOBILE"
    #[serde(rename="MOBILE")]
    MOBILE,
    

    /// This represents video ad formats.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// This represents ads shown within games.
    ///
    /// "GAMES"
    #[serde(rename="GAMES")]
    GAMES,
}

impl AsRef<str> for ProductSyndicationProductEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProductSyndicationProductEnum::SYNDICATIONPRODUCTUNSPECIFIED => "SYNDICATION_PRODUCT_UNSPECIFIED",
            ProductSyndicationProductEnum::CONTENT => "CONTENT",
            ProductSyndicationProductEnum::MOBILE => "MOBILE",
            ProductSyndicationProductEnum::VIDEO => "VIDEO",
            ProductSyndicationProductEnum::GAMES => "GAMES",
        }
    }
}

impl std::convert::TryFrom< &str> for ProductSyndicationProductEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYNDICATION_PRODUCT_UNSPECIFIED" => Ok(ProductSyndicationProductEnum::SYNDICATIONPRODUCTUNSPECIFIED),
           "CONTENT" => Ok(ProductSyndicationProductEnum::CONTENT),
           "MOBILE" => Ok(ProductSyndicationProductEnum::MOBILE),
           "VIDEO" => Ok(ProductSyndicationProductEnum::VIDEO),
           "GAMES" => Ok(ProductSyndicationProductEnum::GAMES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProductSyndicationProductEnum {
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


// region ProposalProposalStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the proposal.
pub enum ProposalProposalStateEnum {
    

    /// A placeholder for an undefined proposal state.
    ///
    /// "PROPOSAL_STATE_UNSPECIFIED"
    #[serde(rename="PROPOSAL_STATE_UNSPECIFIED")]
    PROPOSALSTATEUNSPECIFIED,
    

    /// The proposal is under negotiation or renegotiation.
    ///
    /// "PROPOSED"
    #[serde(rename="PROPOSED")]
    PROPOSED,
    

    /// The proposal has been accepted by the buyer.
    ///
    /// "BUYER_ACCEPTED"
    #[serde(rename="BUYER_ACCEPTED")]
    BUYERACCEPTED,
    

    /// The proposal has been accepted by the seller.
    ///
    /// "SELLER_ACCEPTED"
    #[serde(rename="SELLER_ACCEPTED")]
    SELLERACCEPTED,
    

    /// The negotiations on the proposal were canceled and the proposal was never finalized.
    ///
    /// "CANCELED"
    #[serde(rename="CANCELED")]
    CANCELED,
    

    /// The proposal is finalized. During renegotiation, the proposal may not be in this state.
    ///
    /// "FINALIZED"
    #[serde(rename="FINALIZED")]
    FINALIZED,
}

impl AsRef<str> for ProposalProposalStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProposalProposalStateEnum::PROPOSALSTATEUNSPECIFIED => "PROPOSAL_STATE_UNSPECIFIED",
            ProposalProposalStateEnum::PROPOSED => "PROPOSED",
            ProposalProposalStateEnum::BUYERACCEPTED => "BUYER_ACCEPTED",
            ProposalProposalStateEnum::SELLERACCEPTED => "SELLER_ACCEPTED",
            ProposalProposalStateEnum::CANCELED => "CANCELED",
            ProposalProposalStateEnum::FINALIZED => "FINALIZED",
        }
    }
}

impl std::convert::TryFrom< &str> for ProposalProposalStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROPOSAL_STATE_UNSPECIFIED" => Ok(ProposalProposalStateEnum::PROPOSALSTATEUNSPECIFIED),
           "PROPOSED" => Ok(ProposalProposalStateEnum::PROPOSED),
           "BUYER_ACCEPTED" => Ok(ProposalProposalStateEnum::BUYERACCEPTED),
           "SELLER_ACCEPTED" => Ok(ProposalProposalStateEnum::SELLERACCEPTED),
           "CANCELED" => Ok(ProposalProposalStateEnum::CANCELED),
           "FINALIZED" => Ok(ProposalProposalStateEnum::FINALIZED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProposalProposalStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PublisherProfileMobileApplicationAppStoreEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The app store the app belongs to.
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
    /// "AMAZON_FIRETV"
    #[serde(rename="AMAZON_FIRETV")]
    AMAZONFIRETV,
    

    /// Playstation
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
            PublisherProfileMobileApplicationAppStoreEnum::AMAZONFIRETV => "AMAZON_FIRETV",
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
           "AMAZON_FIRETV" => Ok(PublisherProfileMobileApplicationAppStoreEnum::AMAZONFIRETV),
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


// region SecurityContextSecuritiesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The security types in this context.
pub enum SecurityContextSecuritiesEnum {
    

    /// Matches impressions that require insecure compatibility.
    ///
    /// "INSECURE"
    #[serde(rename="INSECURE")]
    INSECURE,
    

    /// Matches impressions that require SSL compatibility.
    ///
    /// "SSL"
    #[serde(rename="SSL")]
    SSL,
}

impl AsRef<str> for SecurityContextSecuritiesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecurityContextSecuritiesEnum::INSECURE => "INSECURE",
            SecurityContextSecuritiesEnum::SSL => "SSL",
        }
    }
}

impl std::convert::TryFrom< &str> for SecurityContextSecuritiesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INSECURE" => Ok(SecurityContextSecuritiesEnum::INSECURE),
           "SSL" => Ok(SecurityContextSecuritiesEnum::SSL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecurityContextSecuritiesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServingContextAllEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Matches all contexts.
pub enum ServingContextAllEnum {
    

    /// A simple context.
    ///
    /// "SIMPLE_CONTEXT"
    #[serde(rename="SIMPLE_CONTEXT")]
    SIMPLECONTEXT,
}

impl AsRef<str> for ServingContextAllEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServingContextAllEnum::SIMPLECONTEXT => "SIMPLE_CONTEXT",
        }
    }
}

impl std::convert::TryFrom< &str> for ServingContextAllEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SIMPLE_CONTEXT" => Ok(ServingContextAllEnum::SIMPLECONTEXT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServingContextAllEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ServingRestrictionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the creative in this context (for example, it has been explicitly disapproved or is pending review).
pub enum ServingRestrictionStatusEnum {
    

    /// The status is not known.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The ad was disapproved in this context.
    ///
    /// "DISAPPROVAL"
    #[serde(rename="DISAPPROVAL")]
    DISAPPROVAL,
    

    /// The ad is pending review in this context.
    ///
    /// "PENDING_REVIEW"
    #[serde(rename="PENDING_REVIEW")]
    PENDINGREVIEW,
}

impl AsRef<str> for ServingRestrictionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ServingRestrictionStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            ServingRestrictionStatusEnum::DISAPPROVAL => "DISAPPROVAL",
            ServingRestrictionStatusEnum::PENDINGREVIEW => "PENDING_REVIEW",
        }
    }
}

impl std::convert::TryFrom< &str> for ServingRestrictionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(ServingRestrictionStatusEnum::STATUSUNSPECIFIED),
           "DISAPPROVAL" => Ok(ServingRestrictionStatusEnum::DISAPPROVAL),
           "PENDING_REVIEW" => Ok(ServingRestrictionStatusEnum::PENDINGREVIEW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ServingRestrictionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoTargetingExcludedPositionTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of video positions to be excluded. Position types can either be included or excluded (XOR).
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
/// A list of video positions to be included. When the included list is present, the excluded list must be empty. When the excluded list is present, the included list must be empty.
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


// region AccountDuplicateIdModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates if multiple creatives can share an ID or not. Default is NO_DUPLICATES (one ID per creative).
pub enum AccountDuplicateIdModeEnum {
    

    /// Recommended. This means that an ID will be unique to a single creative. Multiple creatives will not share an ID.
    ///
    /// "NO_DUPLICATES"
    #[serde(rename="NO_DUPLICATES")]
    NODUPLICATES,
    

    /// Not recommended. Using this option will allow multiple creatives to share the same ID. Get and Update requests will not be possible for any ID that has more than one creative associated. (List will still function.) This is only intended for backwards compatibility in cases where a single ID is already shared by multiple creatives from previous APIs.
    ///
    /// "FORCE_ENABLE_DUPLICATE_IDS"
    #[serde(rename="FORCE_ENABLE_DUPLICATE_IDS")]
    FORCEENABLEDUPLICATEIDS,
}

impl AsRef<str> for AccountDuplicateIdModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountDuplicateIdModeEnum::NODUPLICATES => "NO_DUPLICATES",
            AccountDuplicateIdModeEnum::FORCEENABLEDUPLICATEIDS => "FORCE_ENABLE_DUPLICATE_IDS",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountDuplicateIdModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_DUPLICATES" => Ok(AccountDuplicateIdModeEnum::NODUPLICATES),
           "FORCE_ENABLE_DUPLICATE_IDS" => Ok(AccountDuplicateIdModeEnum::FORCEENABLEDUPLICATEIDS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountDuplicateIdModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountFilterSyntaxEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Syntax the filter is written in. Current implementation defaults to PQL but in the future it will be LIST_FILTER.
pub enum AccountFilterSyntaxEnum {
    

    /// A placeholder for an undefined filter syntax.
    ///
    /// "FILTER_SYNTAX_UNSPECIFIED"
    #[serde(rename="FILTER_SYNTAX_UNSPECIFIED")]
    FILTERSYNTAXUNSPECIFIED,
    

    /// PQL query syntax. Visit https://developers.google.com/ad-manager/api/pqlreference for PQL documentation and examples.
    ///
    /// "PQL"
    #[serde(rename="PQL")]
    PQL,
    

    /// API list filtering syntax. Read about syntax and usage at https://developers.google.com/authorized-buyers/apis/guides/v2/list-filters.
    ///
    /// "LIST_FILTER"
    #[serde(rename="LIST_FILTER")]
    LISTFILTER,
}

impl AsRef<str> for AccountFilterSyntaxEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountFilterSyntaxEnum::FILTERSYNTAXUNSPECIFIED => "FILTER_SYNTAX_UNSPECIFIED",
            AccountFilterSyntaxEnum::PQL => "PQL",
            AccountFilterSyntaxEnum::LISTFILTER => "LIST_FILTER",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountFilterSyntaxEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FILTER_SYNTAX_UNSPECIFIED" => Ok(AccountFilterSyntaxEnum::FILTERSYNTAXUNSPECIFIED),
           "PQL" => Ok(AccountFilterSyntaxEnum::PQL),
           "LIST_FILTER" => Ok(AccountFilterSyntaxEnum::LISTFILTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountFilterSyntaxEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


