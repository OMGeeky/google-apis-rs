use super::*;



// region ActivityContentDetailsPromotedItemCtaTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of call-to-action, a message to the user indicating action that can be taken.
pub enum ActivityContentDetailsPromotedItemCtaTypeEnum {
    
    /// "ctaTypeUnspecified"
    #[serde(rename="ctaTypeUnspecified")]
    CtaTypeUnspecified,
    
    /// "visitAdvertiserSite"
    #[serde(rename="visitAdvertiserSite")]
    VisitAdvertiserSite,
}

impl AsRef<str> for ActivityContentDetailsPromotedItemCtaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityContentDetailsPromotedItemCtaTypeEnum::CtaTypeUnspecified => "ctaTypeUnspecified",
            ActivityContentDetailsPromotedItemCtaTypeEnum::VisitAdvertiserSite => "visitAdvertiserSite",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityContentDetailsPromotedItemCtaTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ctaTypeUnspecified" => Ok(ActivityContentDetailsPromotedItemCtaTypeEnum::CtaTypeUnspecified),
           "visitAdvertiserSite" => Ok(ActivityContentDetailsPromotedItemCtaTypeEnum::VisitAdvertiserSite),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityContentDetailsPromotedItemCtaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityContentDetailsRecommendationReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that the resource is recommended to the user.
pub enum ActivityContentDetailsRecommendationReasonEnum {
    
    /// "reasonUnspecified"
    #[serde(rename="reasonUnspecified")]
    ReasonUnspecified,
    
    /// "videoFavorited"
    #[serde(rename="videoFavorited")]
    VideoFavorited,
    
    /// "videoLiked"
    #[serde(rename="videoLiked")]
    VideoLiked,
    
    /// "videoWatched"
    #[serde(rename="videoWatched")]
    VideoWatched,
}

impl AsRef<str> for ActivityContentDetailsRecommendationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityContentDetailsRecommendationReasonEnum::ReasonUnspecified => "reasonUnspecified",
            ActivityContentDetailsRecommendationReasonEnum::VideoFavorited => "videoFavorited",
            ActivityContentDetailsRecommendationReasonEnum::VideoLiked => "videoLiked",
            ActivityContentDetailsRecommendationReasonEnum::VideoWatched => "videoWatched",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityContentDetailsRecommendationReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "reasonUnspecified" => Ok(ActivityContentDetailsRecommendationReasonEnum::ReasonUnspecified),
           "videoFavorited" => Ok(ActivityContentDetailsRecommendationReasonEnum::VideoFavorited),
           "videoLiked" => Ok(ActivityContentDetailsRecommendationReasonEnum::VideoLiked),
           "videoWatched" => Ok(ActivityContentDetailsRecommendationReasonEnum::VideoWatched),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityContentDetailsRecommendationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityContentDetailsSocialTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The name of the social network.
pub enum ActivityContentDetailsSocialTypeEnum {
    
    /// "unspecified"
    #[serde(rename="unspecified")]
    Unspecified,
    
    /// "googlePlus"
    #[serde(rename="googlePlus")]
    GooglePlus,
    
    /// "facebook"
    #[serde(rename="facebook")]
    Facebook,
    
    /// "twitter"
    #[serde(rename="twitter")]
    Twitter,
}

impl AsRef<str> for ActivityContentDetailsSocialTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityContentDetailsSocialTypeEnum::Unspecified => "unspecified",
            ActivityContentDetailsSocialTypeEnum::GooglePlus => "googlePlus",
            ActivityContentDetailsSocialTypeEnum::Facebook => "facebook",
            ActivityContentDetailsSocialTypeEnum::Twitter => "twitter",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityContentDetailsSocialTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unspecified" => Ok(ActivityContentDetailsSocialTypeEnum::Unspecified),
           "googlePlus" => Ok(ActivityContentDetailsSocialTypeEnum::GooglePlus),
           "facebook" => Ok(ActivityContentDetailsSocialTypeEnum::Facebook),
           "twitter" => Ok(ActivityContentDetailsSocialTypeEnum::Twitter),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityContentDetailsSocialTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivitySnippetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of activity that the resource describes.
pub enum ActivitySnippetTypeEnum {
    
    /// "typeUnspecified"
    #[serde(rename="typeUnspecified")]
    TypeUnspecified,
    
    /// "upload"
    #[serde(rename="upload")]
    Upload,
    
    /// "like"
    #[serde(rename="like")]
    Like,
    
    /// "favorite"
    #[serde(rename="favorite")]
    Favorite,
    
    /// "comment"
    #[serde(rename="comment")]
    Comment,
    
    /// "subscription"
    #[serde(rename="subscription")]
    Subscription,
    
    /// "playlistItem"
    #[serde(rename="playlistItem")]
    PlaylistItem,
    
    /// "recommendation"
    #[serde(rename="recommendation")]
    Recommendation,
    
    /// "bulletin"
    #[serde(rename="bulletin")]
    Bulletin,
    
    /// "social"
    #[serde(rename="social")]
    Social,
    
    /// "channelItem"
    #[serde(rename="channelItem")]
    ChannelItem,
    
    /// "promotedItem"
    #[serde(rename="promotedItem")]
    PromotedItem,
}

impl AsRef<str> for ActivitySnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivitySnippetTypeEnum::TypeUnspecified => "typeUnspecified",
            ActivitySnippetTypeEnum::Upload => "upload",
            ActivitySnippetTypeEnum::Like => "like",
            ActivitySnippetTypeEnum::Favorite => "favorite",
            ActivitySnippetTypeEnum::Comment => "comment",
            ActivitySnippetTypeEnum::Subscription => "subscription",
            ActivitySnippetTypeEnum::PlaylistItem => "playlistItem",
            ActivitySnippetTypeEnum::Recommendation => "recommendation",
            ActivitySnippetTypeEnum::Bulletin => "bulletin",
            ActivitySnippetTypeEnum::Social => "social",
            ActivitySnippetTypeEnum::ChannelItem => "channelItem",
            ActivitySnippetTypeEnum::PromotedItem => "promotedItem",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivitySnippetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "typeUnspecified" => Ok(ActivitySnippetTypeEnum::TypeUnspecified),
           "upload" => Ok(ActivitySnippetTypeEnum::Upload),
           "like" => Ok(ActivitySnippetTypeEnum::Like),
           "favorite" => Ok(ActivitySnippetTypeEnum::Favorite),
           "comment" => Ok(ActivitySnippetTypeEnum::Comment),
           "subscription" => Ok(ActivitySnippetTypeEnum::Subscription),
           "playlistItem" => Ok(ActivitySnippetTypeEnum::PlaylistItem),
           "recommendation" => Ok(ActivitySnippetTypeEnum::Recommendation),
           "bulletin" => Ok(ActivitySnippetTypeEnum::Bulletin),
           "social" => Ok(ActivitySnippetTypeEnum::Social),
           "channelItem" => Ok(ActivitySnippetTypeEnum::ChannelItem),
           "promotedItem" => Ok(ActivitySnippetTypeEnum::PromotedItem),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivitySnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CaptionSnippetAudioTrackTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of audio track associated with the caption track.
pub enum CaptionSnippetAudioTrackTypeEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "primary"
    #[serde(rename="primary")]
    Primary,
    
    /// "commentary"
    #[serde(rename="commentary")]
    Commentary,
    
    /// "descriptive"
    #[serde(rename="descriptive")]
    Descriptive,
}

impl AsRef<str> for CaptionSnippetAudioTrackTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetAudioTrackTypeEnum::Unknown => "unknown",
            CaptionSnippetAudioTrackTypeEnum::Primary => "primary",
            CaptionSnippetAudioTrackTypeEnum::Commentary => "commentary",
            CaptionSnippetAudioTrackTypeEnum::Descriptive => "descriptive",
        }
    }
}

impl std::convert::TryFrom< &str> for CaptionSnippetAudioTrackTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(CaptionSnippetAudioTrackTypeEnum::Unknown),
           "primary" => Ok(CaptionSnippetAudioTrackTypeEnum::Primary),
           "commentary" => Ok(CaptionSnippetAudioTrackTypeEnum::Commentary),
           "descriptive" => Ok(CaptionSnippetAudioTrackTypeEnum::Descriptive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetAudioTrackTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CaptionSnippetFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed.
pub enum CaptionSnippetFailureReasonEnum {
    
    /// "unknownFormat"
    #[serde(rename="unknownFormat")]
    UnknownFormat,
    
    /// "unsupportedFormat"
    #[serde(rename="unsupportedFormat")]
    UnsupportedFormat,
    
    /// "processingFailed"
    #[serde(rename="processingFailed")]
    ProcessingFailed,
}

impl AsRef<str> for CaptionSnippetFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetFailureReasonEnum::UnknownFormat => "unknownFormat",
            CaptionSnippetFailureReasonEnum::UnsupportedFormat => "unsupportedFormat",
            CaptionSnippetFailureReasonEnum::ProcessingFailed => "processingFailed",
        }
    }
}

impl std::convert::TryFrom< &str> for CaptionSnippetFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknownFormat" => Ok(CaptionSnippetFailureReasonEnum::UnknownFormat),
           "unsupportedFormat" => Ok(CaptionSnippetFailureReasonEnum::UnsupportedFormat),
           "processingFailed" => Ok(CaptionSnippetFailureReasonEnum::ProcessingFailed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CaptionSnippetStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The caption track's status.
pub enum CaptionSnippetStatusEnum {
    
    /// "serving"
    #[serde(rename="serving")]
    Serving,
    
    /// "syncing"
    #[serde(rename="syncing")]
    Syncing,
    
    /// "failed"
    #[serde(rename="failed")]
    Failed,
}

impl AsRef<str> for CaptionSnippetStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetStatusEnum::Serving => "serving",
            CaptionSnippetStatusEnum::Syncing => "syncing",
            CaptionSnippetStatusEnum::Failed => "failed",
        }
    }
}

impl std::convert::TryFrom< &str> for CaptionSnippetStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "serving" => Ok(CaptionSnippetStatusEnum::Serving),
           "syncing" => Ok(CaptionSnippetStatusEnum::Syncing),
           "failed" => Ok(CaptionSnippetStatusEnum::Failed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CaptionSnippetTrackKindEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The caption track's type.
pub enum CaptionSnippetTrackKindEnum {
    
    /// "standard"
    #[serde(rename="standard")]
    Standard,
    
    /// "ASR"
    #[serde(rename="ASR")]
    ASR,
    
    /// "forced"
    #[serde(rename="forced")]
    Forced,
}

impl AsRef<str> for CaptionSnippetTrackKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetTrackKindEnum::Standard => "standard",
            CaptionSnippetTrackKindEnum::ASR => "ASR",
            CaptionSnippetTrackKindEnum::Forced => "forced",
        }
    }
}

impl std::convert::TryFrom< &str> for CaptionSnippetTrackKindEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "standard" => Ok(CaptionSnippetTrackKindEnum::Standard),
           "ASR" => Ok(CaptionSnippetTrackKindEnum::ASR),
           "forced" => Ok(CaptionSnippetTrackKindEnum::Forced),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetTrackKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CdnSettingFrameRateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The frame rate of the inbound video data.
pub enum CdnSettingFrameRateEnum {
    
    /// "30fps"
    #[serde(rename="30fps")]
    _30fps,
    
    /// "60fps"
    #[serde(rename="60fps")]
    _60fps,
    
    /// "variable"
    #[serde(rename="variable")]
    Variable,
}

impl AsRef<str> for CdnSettingFrameRateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CdnSettingFrameRateEnum::_30fps => "30fps",
            CdnSettingFrameRateEnum::_60fps => "60fps",
            CdnSettingFrameRateEnum::Variable => "variable",
        }
    }
}

impl std::convert::TryFrom< &str> for CdnSettingFrameRateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "30fps" => Ok(CdnSettingFrameRateEnum::_30fps),
           "60fps" => Ok(CdnSettingFrameRateEnum::_60fps),
           "variable" => Ok(CdnSettingFrameRateEnum::Variable),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CdnSettingFrameRateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CdnSettingIngestionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
///  The method or protocol used to transmit the video stream.
pub enum CdnSettingIngestionTypeEnum {
    
    /// "rtmp"
    #[serde(rename="rtmp")]
    Rtmp,
    
    /// "dash"
    #[serde(rename="dash")]
    Dash,
    
    /// "webrtc"
    #[serde(rename="webrtc")]
    Webrtc,
    
    /// "hls"
    #[serde(rename="hls")]
    Hls,
}

impl AsRef<str> for CdnSettingIngestionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CdnSettingIngestionTypeEnum::Rtmp => "rtmp",
            CdnSettingIngestionTypeEnum::Dash => "dash",
            CdnSettingIngestionTypeEnum::Webrtc => "webrtc",
            CdnSettingIngestionTypeEnum::Hls => "hls",
        }
    }
}

impl std::convert::TryFrom< &str> for CdnSettingIngestionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rtmp" => Ok(CdnSettingIngestionTypeEnum::Rtmp),
           "dash" => Ok(CdnSettingIngestionTypeEnum::Dash),
           "webrtc" => Ok(CdnSettingIngestionTypeEnum::Webrtc),
           "hls" => Ok(CdnSettingIngestionTypeEnum::Hls),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CdnSettingIngestionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CdnSettingResolutionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The resolution of the inbound video data.
pub enum CdnSettingResolutionEnum {
    
    /// "240p"
    #[serde(rename="240p")]
    _240p,
    
    /// "360p"
    #[serde(rename="360p")]
    _360p,
    
    /// "480p"
    #[serde(rename="480p")]
    _480p,
    
    /// "720p"
    #[serde(rename="720p")]
    _720p,
    
    /// "1080p"
    #[serde(rename="1080p")]
    _1080p,
    
    /// "1440p"
    #[serde(rename="1440p")]
    _1440p,
    
    /// "2160p"
    #[serde(rename="2160p")]
    _2160p,
    
    /// "variable"
    #[serde(rename="variable")]
    Variable,
}

impl AsRef<str> for CdnSettingResolutionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CdnSettingResolutionEnum::_240p => "240p",
            CdnSettingResolutionEnum::_360p => "360p",
            CdnSettingResolutionEnum::_480p => "480p",
            CdnSettingResolutionEnum::_720p => "720p",
            CdnSettingResolutionEnum::_1080p => "1080p",
            CdnSettingResolutionEnum::_1440p => "1440p",
            CdnSettingResolutionEnum::_2160p => "2160p",
            CdnSettingResolutionEnum::Variable => "variable",
        }
    }
}

impl std::convert::TryFrom< &str> for CdnSettingResolutionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "240p" => Ok(CdnSettingResolutionEnum::_240p),
           "360p" => Ok(CdnSettingResolutionEnum::_360p),
           "480p" => Ok(CdnSettingResolutionEnum::_480p),
           "720p" => Ok(CdnSettingResolutionEnum::_720p),
           "1080p" => Ok(CdnSettingResolutionEnum::_1080p),
           "1440p" => Ok(CdnSettingResolutionEnum::_1440p),
           "2160p" => Ok(CdnSettingResolutionEnum::_2160p),
           "variable" => Ok(CdnSettingResolutionEnum::Variable),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CdnSettingResolutionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChannelConversionPingContextEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the context of the ping.
pub enum ChannelConversionPingContextEnum {
    
    /// "subscribe"
    #[serde(rename="subscribe")]
    Subscribe,
    
    /// "unsubscribe"
    #[serde(rename="unsubscribe")]
    Unsubscribe,
    
    /// "cview"
    #[serde(rename="cview")]
    Cview,
}

impl AsRef<str> for ChannelConversionPingContextEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelConversionPingContextEnum::Subscribe => "subscribe",
            ChannelConversionPingContextEnum::Unsubscribe => "unsubscribe",
            ChannelConversionPingContextEnum::Cview => "cview",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelConversionPingContextEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "subscribe" => Ok(ChannelConversionPingContextEnum::Subscribe),
           "unsubscribe" => Ok(ChannelConversionPingContextEnum::Unsubscribe),
           "cview" => Ok(ChannelConversionPingContextEnum::Cview),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelConversionPingContextEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChannelSectionSnippetStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The style of the channel section.
pub enum ChannelSectionSnippetStyleEnum {
    
    /// "channelsectionStyleUnspecified"
    #[serde(rename="channelsectionStyleUnspecified")]
    ChannelsectionStyleUnspecified,
    
    /// "horizontalRow"
    #[serde(rename="horizontalRow")]
    HorizontalRow,
    
    /// "verticalList"
    #[serde(rename="verticalList")]
    VerticalList,
}

impl AsRef<str> for ChannelSectionSnippetStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelSectionSnippetStyleEnum::ChannelsectionStyleUnspecified => "channelsectionStyleUnspecified",
            ChannelSectionSnippetStyleEnum::HorizontalRow => "horizontalRow",
            ChannelSectionSnippetStyleEnum::VerticalList => "verticalList",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelSectionSnippetStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "channelsectionStyleUnspecified" => Ok(ChannelSectionSnippetStyleEnum::ChannelsectionStyleUnspecified),
           "horizontalRow" => Ok(ChannelSectionSnippetStyleEnum::HorizontalRow),
           "verticalList" => Ok(ChannelSectionSnippetStyleEnum::VerticalList),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelSectionSnippetStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChannelSectionSnippetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the channel section.
pub enum ChannelSectionSnippetTypeEnum {
    
    /// "channelsectionTypeUndefined"
    #[serde(rename="channelsectionTypeUndefined")]
    ChannelsectionTypeUndefined,
    
    /// "singlePlaylist"
    #[serde(rename="singlePlaylist")]
    SinglePlaylist,
    
    /// "multiplePlaylists"
    #[serde(rename="multiplePlaylists")]
    MultiplePlaylists,
    
    /// "popularUploads"
    #[serde(rename="popularUploads")]
    PopularUploads,
    
    /// "recentUploads"
    #[serde(rename="recentUploads")]
    RecentUploads,
    
    /// "likes"
    #[serde(rename="likes")]
    Likes,
    
    /// "allPlaylists"
    #[serde(rename="allPlaylists")]
    AllPlaylists,
    
    /// "likedPlaylists"
    #[serde(rename="likedPlaylists")]
    LikedPlaylists,
    
    /// "recentPosts"
    #[serde(rename="recentPosts")]
    RecentPosts,
    
    /// "recentActivity"
    #[serde(rename="recentActivity")]
    RecentActivity,
    
    /// "liveEvents"
    #[serde(rename="liveEvents")]
    LiveEvents,
    
    /// "upcomingEvents"
    #[serde(rename="upcomingEvents")]
    UpcomingEvents,
    
    /// "completedEvents"
    #[serde(rename="completedEvents")]
    CompletedEvents,
    
    /// "multipleChannels"
    #[serde(rename="multipleChannels")]
    MultipleChannels,
    
    /// "postedVideos"
    #[serde(rename="postedVideos")]
    PostedVideos,
    
    /// "postedPlaylists"
    #[serde(rename="postedPlaylists")]
    PostedPlaylists,
    
    /// "subscriptions"
    #[serde(rename="subscriptions")]
    Subscriptions,
}

impl AsRef<str> for ChannelSectionSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelSectionSnippetTypeEnum::ChannelsectionTypeUndefined => "channelsectionTypeUndefined",
            ChannelSectionSnippetTypeEnum::SinglePlaylist => "singlePlaylist",
            ChannelSectionSnippetTypeEnum::MultiplePlaylists => "multiplePlaylists",
            ChannelSectionSnippetTypeEnum::PopularUploads => "popularUploads",
            ChannelSectionSnippetTypeEnum::RecentUploads => "recentUploads",
            ChannelSectionSnippetTypeEnum::Likes => "likes",
            ChannelSectionSnippetTypeEnum::AllPlaylists => "allPlaylists",
            ChannelSectionSnippetTypeEnum::LikedPlaylists => "likedPlaylists",
            ChannelSectionSnippetTypeEnum::RecentPosts => "recentPosts",
            ChannelSectionSnippetTypeEnum::RecentActivity => "recentActivity",
            ChannelSectionSnippetTypeEnum::LiveEvents => "liveEvents",
            ChannelSectionSnippetTypeEnum::UpcomingEvents => "upcomingEvents",
            ChannelSectionSnippetTypeEnum::CompletedEvents => "completedEvents",
            ChannelSectionSnippetTypeEnum::MultipleChannels => "multipleChannels",
            ChannelSectionSnippetTypeEnum::PostedVideos => "postedVideos",
            ChannelSectionSnippetTypeEnum::PostedPlaylists => "postedPlaylists",
            ChannelSectionSnippetTypeEnum::Subscriptions => "subscriptions",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelSectionSnippetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "channelsectionTypeUndefined" => Ok(ChannelSectionSnippetTypeEnum::ChannelsectionTypeUndefined),
           "singlePlaylist" => Ok(ChannelSectionSnippetTypeEnum::SinglePlaylist),
           "multiplePlaylists" => Ok(ChannelSectionSnippetTypeEnum::MultiplePlaylists),
           "popularUploads" => Ok(ChannelSectionSnippetTypeEnum::PopularUploads),
           "recentUploads" => Ok(ChannelSectionSnippetTypeEnum::RecentUploads),
           "likes" => Ok(ChannelSectionSnippetTypeEnum::Likes),
           "allPlaylists" => Ok(ChannelSectionSnippetTypeEnum::AllPlaylists),
           "likedPlaylists" => Ok(ChannelSectionSnippetTypeEnum::LikedPlaylists),
           "recentPosts" => Ok(ChannelSectionSnippetTypeEnum::RecentPosts),
           "recentActivity" => Ok(ChannelSectionSnippetTypeEnum::RecentActivity),
           "liveEvents" => Ok(ChannelSectionSnippetTypeEnum::LiveEvents),
           "upcomingEvents" => Ok(ChannelSectionSnippetTypeEnum::UpcomingEvents),
           "completedEvents" => Ok(ChannelSectionSnippetTypeEnum::CompletedEvents),
           "multipleChannels" => Ok(ChannelSectionSnippetTypeEnum::MultipleChannels),
           "postedVideos" => Ok(ChannelSectionSnippetTypeEnum::PostedVideos),
           "postedPlaylists" => Ok(ChannelSectionSnippetTypeEnum::PostedPlaylists),
           "subscriptions" => Ok(ChannelSectionSnippetTypeEnum::Subscriptions),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelSectionSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChannelStatusLongUploadsStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The long uploads status of this channel. See https://support.google.com/youtube/answer/71673 for more information.
pub enum ChannelStatusLongUploadsStatusEnum {
    
    /// "longUploadsUnspecified"
    #[serde(rename="longUploadsUnspecified")]
    LongUploadsUnspecified,
    
    /// "allowed"
    #[serde(rename="allowed")]
    Allowed,
    
    /// "eligible"
    #[serde(rename="eligible")]
    Eligible,
    
    /// "disallowed"
    #[serde(rename="disallowed")]
    Disallowed,
}

impl AsRef<str> for ChannelStatusLongUploadsStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelStatusLongUploadsStatusEnum::LongUploadsUnspecified => "longUploadsUnspecified",
            ChannelStatusLongUploadsStatusEnum::Allowed => "allowed",
            ChannelStatusLongUploadsStatusEnum::Eligible => "eligible",
            ChannelStatusLongUploadsStatusEnum::Disallowed => "disallowed",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelStatusLongUploadsStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "longUploadsUnspecified" => Ok(ChannelStatusLongUploadsStatusEnum::LongUploadsUnspecified),
           "allowed" => Ok(ChannelStatusLongUploadsStatusEnum::Allowed),
           "eligible" => Ok(ChannelStatusLongUploadsStatusEnum::Eligible),
           "disallowed" => Ok(ChannelStatusLongUploadsStatusEnum::Disallowed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelStatusLongUploadsStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChannelStatusPrivacyStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Privacy status of the channel.
pub enum ChannelStatusPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for ChannelStatusPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelStatusPrivacyStatusEnum::Public => "public",
            ChannelStatusPrivacyStatusEnum::Unlisted => "unlisted",
            ChannelStatusPrivacyStatusEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelStatusPrivacyStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(ChannelStatusPrivacyStatusEnum::Public),
           "unlisted" => Ok(ChannelStatusPrivacyStatusEnum::Unlisted),
           "private" => Ok(ChannelStatusPrivacyStatusEnum::Private),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelStatusPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current billing profile status.
pub enum ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum {
    
    /// "billingStatusUnspecified"
    #[serde(rename="billingStatusUnspecified")]
    BillingStatusUnspecified,
    
    /// "billingStatusPending"
    #[serde(rename="billingStatusPending")]
    BillingStatusPending,
    
    /// "billingStatusActive"
    #[serde(rename="billingStatusActive")]
    BillingStatusActive,
    
    /// "billingStatusInactive"
    #[serde(rename="billingStatusInactive")]
    BillingStatusInactive,
}

impl AsRef<str> for ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusUnspecified => "billingStatusUnspecified",
            ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusPending => "billingStatusPending",
            ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusActive => "billingStatusActive",
            ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusInactive => "billingStatusInactive",
        }
    }
}

impl std::convert::TryFrom< &str> for ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "billingStatusUnspecified" => Ok(ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusUnspecified),
           "billingStatusPending" => Ok(ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusPending),
           "billingStatusActive" => Ok(ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusActive),
           "billingStatusInactive" => Ok(ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum::BillingStatusInactive),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelToStoreLinkDetailsBillingDetailBillingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommentSnippetModerationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The comment's moderation status. Will not be set if the comments were requested through the id filter.
pub enum CommentSnippetModerationStatusEnum {
    

    /// The comment is available for public display.
    ///
    /// "published"
    #[serde(rename="published")]
    Published,
    

    /// The comment is awaiting review by a moderator.
    ///
    /// "heldForReview"
    #[serde(rename="heldForReview")]
    HeldForReview,
    
    /// "likelySpam"
    #[serde(rename="likelySpam")]
    LikelySpam,
    

    /// The comment is unfit for display.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
}

impl AsRef<str> for CommentSnippetModerationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentSnippetModerationStatusEnum::Published => "published",
            CommentSnippetModerationStatusEnum::HeldForReview => "heldForReview",
            CommentSnippetModerationStatusEnum::LikelySpam => "likelySpam",
            CommentSnippetModerationStatusEnum::Rejected => "rejected",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentSnippetModerationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "published" => Ok(CommentSnippetModerationStatusEnum::Published),
           "heldForReview" => Ok(CommentSnippetModerationStatusEnum::HeldForReview),
           "likelySpam" => Ok(CommentSnippetModerationStatusEnum::LikelySpam),
           "rejected" => Ok(CommentSnippetModerationStatusEnum::Rejected),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentSnippetModerationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommentSnippetViewerRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future.
pub enum CommentSnippetViewerRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for CommentSnippetViewerRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentSnippetViewerRatingEnum::None => "none",
            CommentSnippetViewerRatingEnum::Like => "like",
            CommentSnippetViewerRatingEnum::Dislike => "dislike",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentSnippetViewerRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(CommentSnippetViewerRatingEnum::None),
           "like" => Ok(CommentSnippetViewerRatingEnum::Like),
           "dislike" => Ok(CommentSnippetViewerRatingEnum::Dislike),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentSnippetViewerRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingAcbRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming.
pub enum ContentRatingAcbRatingEnum {
    
    /// "acbUnspecified"
    #[serde(rename="acbUnspecified")]
    AcbUnspecified,
    

    /// E
    ///
    /// "acbE"
    #[serde(rename="acbE")]
    AcbE,
    

    /// Programs that have been given a P classification by the Australian Communications and Media Authority. These programs are intended for preschool children.
    ///
    /// "acbP"
    #[serde(rename="acbP")]
    AcbP,
    

    /// Programs that have been given a C classification by the Australian Communications and Media Authority. These programs are intended for children (other than preschool children) who are younger than 14 years of age.
    ///
    /// "acbC"
    #[serde(rename="acbC")]
    AcbC,
    

    /// G
    ///
    /// "acbG"
    #[serde(rename="acbG")]
    AcbG,
    

    /// PG
    ///
    /// "acbPg"
    #[serde(rename="acbPg")]
    AcbPg,
    

    /// M
    ///
    /// "acbM"
    #[serde(rename="acbM")]
    AcbM,
    

    /// MA15+
    ///
    /// "acbMa15plus"
    #[serde(rename="acbMa15plus")]
    AcbMa15plus,
    

    /// R18+
    ///
    /// "acbR18plus"
    #[serde(rename="acbR18plus")]
    AcbR18plus,
    
    /// "acbUnrated"
    #[serde(rename="acbUnrated")]
    AcbUnrated,
}

impl AsRef<str> for ContentRatingAcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingAcbRatingEnum::AcbUnspecified => "acbUnspecified",
            ContentRatingAcbRatingEnum::AcbE => "acbE",
            ContentRatingAcbRatingEnum::AcbP => "acbP",
            ContentRatingAcbRatingEnum::AcbC => "acbC",
            ContentRatingAcbRatingEnum::AcbG => "acbG",
            ContentRatingAcbRatingEnum::AcbPg => "acbPg",
            ContentRatingAcbRatingEnum::AcbM => "acbM",
            ContentRatingAcbRatingEnum::AcbMa15plus => "acbMa15plus",
            ContentRatingAcbRatingEnum::AcbR18plus => "acbR18plus",
            ContentRatingAcbRatingEnum::AcbUnrated => "acbUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingAcbRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "acbUnspecified" => Ok(ContentRatingAcbRatingEnum::AcbUnspecified),
           "acbE" => Ok(ContentRatingAcbRatingEnum::AcbE),
           "acbP" => Ok(ContentRatingAcbRatingEnum::AcbP),
           "acbC" => Ok(ContentRatingAcbRatingEnum::AcbC),
           "acbG" => Ok(ContentRatingAcbRatingEnum::AcbG),
           "acbPg" => Ok(ContentRatingAcbRatingEnum::AcbPg),
           "acbM" => Ok(ContentRatingAcbRatingEnum::AcbM),
           "acbMa15plus" => Ok(ContentRatingAcbRatingEnum::AcbMa15plus),
           "acbR18plus" => Ok(ContentRatingAcbRatingEnum::AcbR18plus),
           "acbUnrated" => Ok(ContentRatingAcbRatingEnum::AcbUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingAcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingAgcomRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM).
pub enum ContentRatingAgcomRatingEnum {
    
    /// "agcomUnspecified"
    #[serde(rename="agcomUnspecified")]
    AgcomUnspecified,
    

    /// T
    ///
    /// "agcomT"
    #[serde(rename="agcomT")]
    AgcomT,
    

    /// VM14
    ///
    /// "agcomVm14"
    #[serde(rename="agcomVm14")]
    AgcomVm14,
    

    /// VM18
    ///
    /// "agcomVm18"
    #[serde(rename="agcomVm18")]
    AgcomVm18,
    
    /// "agcomUnrated"
    #[serde(rename="agcomUnrated")]
    AgcomUnrated,
}

impl AsRef<str> for ContentRatingAgcomRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingAgcomRatingEnum::AgcomUnspecified => "agcomUnspecified",
            ContentRatingAgcomRatingEnum::AgcomT => "agcomT",
            ContentRatingAgcomRatingEnum::AgcomVm14 => "agcomVm14",
            ContentRatingAgcomRatingEnum::AgcomVm18 => "agcomVm18",
            ContentRatingAgcomRatingEnum::AgcomUnrated => "agcomUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingAgcomRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "agcomUnspecified" => Ok(ContentRatingAgcomRatingEnum::AgcomUnspecified),
           "agcomT" => Ok(ContentRatingAgcomRatingEnum::AgcomT),
           "agcomVm14" => Ok(ContentRatingAgcomRatingEnum::AgcomVm14),
           "agcomVm18" => Ok(ContentRatingAgcomRatingEnum::AgcomVm18),
           "agcomUnrated" => Ok(ContentRatingAgcomRatingEnum::AgcomUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingAgcomRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingAnatelRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television.
pub enum ContentRatingAnatelRatingEnum {
    
    /// "anatelUnspecified"
    #[serde(rename="anatelUnspecified")]
    AnatelUnspecified,
    

    /// F
    ///
    /// "anatelF"
    #[serde(rename="anatelF")]
    AnatelF,
    

    /// I
    ///
    /// "anatelI"
    #[serde(rename="anatelI")]
    AnatelI,
    

    /// I-7
    ///
    /// "anatelI7"
    #[serde(rename="anatelI7")]
    AnatelI7,
    

    /// I-10
    ///
    /// "anatelI10"
    #[serde(rename="anatelI10")]
    AnatelI10,
    

    /// I-12
    ///
    /// "anatelI12"
    #[serde(rename="anatelI12")]
    AnatelI12,
    

    /// R
    ///
    /// "anatelR"
    #[serde(rename="anatelR")]
    AnatelR,
    

    /// A
    ///
    /// "anatelA"
    #[serde(rename="anatelA")]
    AnatelA,
    
    /// "anatelUnrated"
    #[serde(rename="anatelUnrated")]
    AnatelUnrated,
}

impl AsRef<str> for ContentRatingAnatelRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingAnatelRatingEnum::AnatelUnspecified => "anatelUnspecified",
            ContentRatingAnatelRatingEnum::AnatelF => "anatelF",
            ContentRatingAnatelRatingEnum::AnatelI => "anatelI",
            ContentRatingAnatelRatingEnum::AnatelI7 => "anatelI7",
            ContentRatingAnatelRatingEnum::AnatelI10 => "anatelI10",
            ContentRatingAnatelRatingEnum::AnatelI12 => "anatelI12",
            ContentRatingAnatelRatingEnum::AnatelR => "anatelR",
            ContentRatingAnatelRatingEnum::AnatelA => "anatelA",
            ContentRatingAnatelRatingEnum::AnatelUnrated => "anatelUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingAnatelRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "anatelUnspecified" => Ok(ContentRatingAnatelRatingEnum::AnatelUnspecified),
           "anatelF" => Ok(ContentRatingAnatelRatingEnum::AnatelF),
           "anatelI" => Ok(ContentRatingAnatelRatingEnum::AnatelI),
           "anatelI7" => Ok(ContentRatingAnatelRatingEnum::AnatelI7),
           "anatelI10" => Ok(ContentRatingAnatelRatingEnum::AnatelI10),
           "anatelI12" => Ok(ContentRatingAnatelRatingEnum::AnatelI12),
           "anatelR" => Ok(ContentRatingAnatelRatingEnum::AnatelR),
           "anatelA" => Ok(ContentRatingAnatelRatingEnum::AnatelA),
           "anatelUnrated" => Ok(ContentRatingAnatelRatingEnum::AnatelUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingAnatelRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingBbfcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's British Board of Film Classification (BBFC) rating.
pub enum ContentRatingBbfcRatingEnum {
    
    /// "bbfcUnspecified"
    #[serde(rename="bbfcUnspecified")]
    BbfcUnspecified,
    

    /// U
    ///
    /// "bbfcU"
    #[serde(rename="bbfcU")]
    BbfcU,
    

    /// PG
    ///
    /// "bbfcPg"
    #[serde(rename="bbfcPg")]
    BbfcPg,
    

    /// 12A
    ///
    /// "bbfc12a"
    #[serde(rename="bbfc12a")]
    Bbfc12a,
    

    /// 12
    ///
    /// "bbfc12"
    #[serde(rename="bbfc12")]
    Bbfc12,
    

    /// 15
    ///
    /// "bbfc15"
    #[serde(rename="bbfc15")]
    Bbfc15,
    

    /// 18
    ///
    /// "bbfc18"
    #[serde(rename="bbfc18")]
    Bbfc18,
    

    /// R18
    ///
    /// "bbfcR18"
    #[serde(rename="bbfcR18")]
    BbfcR18,
    
    /// "bbfcUnrated"
    #[serde(rename="bbfcUnrated")]
    BbfcUnrated,
}

impl AsRef<str> for ContentRatingBbfcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingBbfcRatingEnum::BbfcUnspecified => "bbfcUnspecified",
            ContentRatingBbfcRatingEnum::BbfcU => "bbfcU",
            ContentRatingBbfcRatingEnum::BbfcPg => "bbfcPg",
            ContentRatingBbfcRatingEnum::Bbfc12a => "bbfc12a",
            ContentRatingBbfcRatingEnum::Bbfc12 => "bbfc12",
            ContentRatingBbfcRatingEnum::Bbfc15 => "bbfc15",
            ContentRatingBbfcRatingEnum::Bbfc18 => "bbfc18",
            ContentRatingBbfcRatingEnum::BbfcR18 => "bbfcR18",
            ContentRatingBbfcRatingEnum::BbfcUnrated => "bbfcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingBbfcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "bbfcUnspecified" => Ok(ContentRatingBbfcRatingEnum::BbfcUnspecified),
           "bbfcU" => Ok(ContentRatingBbfcRatingEnum::BbfcU),
           "bbfcPg" => Ok(ContentRatingBbfcRatingEnum::BbfcPg),
           "bbfc12a" => Ok(ContentRatingBbfcRatingEnum::Bbfc12a),
           "bbfc12" => Ok(ContentRatingBbfcRatingEnum::Bbfc12),
           "bbfc15" => Ok(ContentRatingBbfcRatingEnum::Bbfc15),
           "bbfc18" => Ok(ContentRatingBbfcRatingEnum::Bbfc18),
           "bbfcR18" => Ok(ContentRatingBbfcRatingEnum::BbfcR18),
           "bbfcUnrated" => Ok(ContentRatingBbfcRatingEnum::BbfcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingBbfcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingBfvcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Thailand's Board of Film and Video Censors.
pub enum ContentRatingBfvcRatingEnum {
    
    /// "bfvcUnspecified"
    #[serde(rename="bfvcUnspecified")]
    BfvcUnspecified,
    

    /// G
    ///
    /// "bfvcG"
    #[serde(rename="bfvcG")]
    BfvcG,
    

    /// E
    ///
    /// "bfvcE"
    #[serde(rename="bfvcE")]
    BfvcE,
    

    /// 13
    ///
    /// "bfvc13"
    #[serde(rename="bfvc13")]
    Bfvc13,
    

    /// 15
    ///
    /// "bfvc15"
    #[serde(rename="bfvc15")]
    Bfvc15,
    

    /// 18
    ///
    /// "bfvc18"
    #[serde(rename="bfvc18")]
    Bfvc18,
    

    /// 20
    ///
    /// "bfvc20"
    #[serde(rename="bfvc20")]
    Bfvc20,
    

    /// B
    ///
    /// "bfvcB"
    #[serde(rename="bfvcB")]
    BfvcB,
    
    /// "bfvcUnrated"
    #[serde(rename="bfvcUnrated")]
    BfvcUnrated,
}

impl AsRef<str> for ContentRatingBfvcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingBfvcRatingEnum::BfvcUnspecified => "bfvcUnspecified",
            ContentRatingBfvcRatingEnum::BfvcG => "bfvcG",
            ContentRatingBfvcRatingEnum::BfvcE => "bfvcE",
            ContentRatingBfvcRatingEnum::Bfvc13 => "bfvc13",
            ContentRatingBfvcRatingEnum::Bfvc15 => "bfvc15",
            ContentRatingBfvcRatingEnum::Bfvc18 => "bfvc18",
            ContentRatingBfvcRatingEnum::Bfvc20 => "bfvc20",
            ContentRatingBfvcRatingEnum::BfvcB => "bfvcB",
            ContentRatingBfvcRatingEnum::BfvcUnrated => "bfvcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingBfvcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "bfvcUnspecified" => Ok(ContentRatingBfvcRatingEnum::BfvcUnspecified),
           "bfvcG" => Ok(ContentRatingBfvcRatingEnum::BfvcG),
           "bfvcE" => Ok(ContentRatingBfvcRatingEnum::BfvcE),
           "bfvc13" => Ok(ContentRatingBfvcRatingEnum::Bfvc13),
           "bfvc15" => Ok(ContentRatingBfvcRatingEnum::Bfvc15),
           "bfvc18" => Ok(ContentRatingBfvcRatingEnum::Bfvc18),
           "bfvc20" => Ok(ContentRatingBfvcRatingEnum::Bfvc20),
           "bfvcB" => Ok(ContentRatingBfvcRatingEnum::BfvcB),
           "bfvcUnrated" => Ok(ContentRatingBfvcRatingEnum::BfvcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingBfvcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingBmukkRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur).
pub enum ContentRatingBmukkRatingEnum {
    
    /// "bmukkUnspecified"
    #[serde(rename="bmukkUnspecified")]
    BmukkUnspecified,
    

    /// Unrestricted
    ///
    /// "bmukkAa"
    #[serde(rename="bmukkAa")]
    BmukkAa,
    

    /// 6+
    ///
    /// "bmukk6"
    #[serde(rename="bmukk6")]
    Bmukk6,
    

    /// 8+
    ///
    /// "bmukk8"
    #[serde(rename="bmukk8")]
    Bmukk8,
    

    /// 10+
    ///
    /// "bmukk10"
    #[serde(rename="bmukk10")]
    Bmukk10,
    

    /// 12+
    ///
    /// "bmukk12"
    #[serde(rename="bmukk12")]
    Bmukk12,
    

    /// 14+
    ///
    /// "bmukk14"
    #[serde(rename="bmukk14")]
    Bmukk14,
    

    /// 16+
    ///
    /// "bmukk16"
    #[serde(rename="bmukk16")]
    Bmukk16,
    
    /// "bmukkUnrated"
    #[serde(rename="bmukkUnrated")]
    BmukkUnrated,
}

impl AsRef<str> for ContentRatingBmukkRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingBmukkRatingEnum::BmukkUnspecified => "bmukkUnspecified",
            ContentRatingBmukkRatingEnum::BmukkAa => "bmukkAa",
            ContentRatingBmukkRatingEnum::Bmukk6 => "bmukk6",
            ContentRatingBmukkRatingEnum::Bmukk8 => "bmukk8",
            ContentRatingBmukkRatingEnum::Bmukk10 => "bmukk10",
            ContentRatingBmukkRatingEnum::Bmukk12 => "bmukk12",
            ContentRatingBmukkRatingEnum::Bmukk14 => "bmukk14",
            ContentRatingBmukkRatingEnum::Bmukk16 => "bmukk16",
            ContentRatingBmukkRatingEnum::BmukkUnrated => "bmukkUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingBmukkRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "bmukkUnspecified" => Ok(ContentRatingBmukkRatingEnum::BmukkUnspecified),
           "bmukkAa" => Ok(ContentRatingBmukkRatingEnum::BmukkAa),
           "bmukk6" => Ok(ContentRatingBmukkRatingEnum::Bmukk6),
           "bmukk8" => Ok(ContentRatingBmukkRatingEnum::Bmukk8),
           "bmukk10" => Ok(ContentRatingBmukkRatingEnum::Bmukk10),
           "bmukk12" => Ok(ContentRatingBmukkRatingEnum::Bmukk12),
           "bmukk14" => Ok(ContentRatingBmukkRatingEnum::Bmukk14),
           "bmukk16" => Ok(ContentRatingBmukkRatingEnum::Bmukk16),
           "bmukkUnrated" => Ok(ContentRatingBmukkRatingEnum::BmukkUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingBmukkRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCatvRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
pub enum ContentRatingCatvRatingEnum {
    
    /// "catvUnspecified"
    #[serde(rename="catvUnspecified")]
    CatvUnspecified,
    

    /// C
    ///
    /// "catvC"
    #[serde(rename="catvC")]
    CatvC,
    

    /// C8
    ///
    /// "catvC8"
    #[serde(rename="catvC8")]
    CatvC8,
    

    /// G
    ///
    /// "catvG"
    #[serde(rename="catvG")]
    CatvG,
    

    /// PG
    ///
    /// "catvPg"
    #[serde(rename="catvPg")]
    CatvPg,
    

    /// 14+
    ///
    /// "catv14plus"
    #[serde(rename="catv14plus")]
    Catv14plus,
    

    /// 18+
    ///
    /// "catv18plus"
    #[serde(rename="catv18plus")]
    Catv18plus,
    
    /// "catvUnrated"
    #[serde(rename="catvUnrated")]
    CatvUnrated,
    
    /// "catvE"
    #[serde(rename="catvE")]
    CatvE,
}

impl AsRef<str> for ContentRatingCatvRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCatvRatingEnum::CatvUnspecified => "catvUnspecified",
            ContentRatingCatvRatingEnum::CatvC => "catvC",
            ContentRatingCatvRatingEnum::CatvC8 => "catvC8",
            ContentRatingCatvRatingEnum::CatvG => "catvG",
            ContentRatingCatvRatingEnum::CatvPg => "catvPg",
            ContentRatingCatvRatingEnum::Catv14plus => "catv14plus",
            ContentRatingCatvRatingEnum::Catv18plus => "catv18plus",
            ContentRatingCatvRatingEnum::CatvUnrated => "catvUnrated",
            ContentRatingCatvRatingEnum::CatvE => "catvE",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCatvRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "catvUnspecified" => Ok(ContentRatingCatvRatingEnum::CatvUnspecified),
           "catvC" => Ok(ContentRatingCatvRatingEnum::CatvC),
           "catvC8" => Ok(ContentRatingCatvRatingEnum::CatvC8),
           "catvG" => Ok(ContentRatingCatvRatingEnum::CatvG),
           "catvPg" => Ok(ContentRatingCatvRatingEnum::CatvPg),
           "catv14plus" => Ok(ContentRatingCatvRatingEnum::Catv14plus),
           "catv18plus" => Ok(ContentRatingCatvRatingEnum::Catv18plus),
           "catvUnrated" => Ok(ContentRatingCatvRatingEnum::CatvUnrated),
           "catvE" => Ok(ContentRatingCatvRatingEnum::CatvE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCatvRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCatvfrRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
pub enum ContentRatingCatvfrRatingEnum {
    
    /// "catvfrUnspecified"
    #[serde(rename="catvfrUnspecified")]
    CatvfrUnspecified,
    

    /// G
    ///
    /// "catvfrG"
    #[serde(rename="catvfrG")]
    CatvfrG,
    

    /// 8+
    ///
    /// "catvfr8plus"
    #[serde(rename="catvfr8plus")]
    Catvfr8plus,
    

    /// 13+
    ///
    /// "catvfr13plus"
    #[serde(rename="catvfr13plus")]
    Catvfr13plus,
    

    /// 16+
    ///
    /// "catvfr16plus"
    #[serde(rename="catvfr16plus")]
    Catvfr16plus,
    

    /// 18+
    ///
    /// "catvfr18plus"
    #[serde(rename="catvfr18plus")]
    Catvfr18plus,
    
    /// "catvfrUnrated"
    #[serde(rename="catvfrUnrated")]
    CatvfrUnrated,
    
    /// "catvfrE"
    #[serde(rename="catvfrE")]
    CatvfrE,
}

impl AsRef<str> for ContentRatingCatvfrRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCatvfrRatingEnum::CatvfrUnspecified => "catvfrUnspecified",
            ContentRatingCatvfrRatingEnum::CatvfrG => "catvfrG",
            ContentRatingCatvfrRatingEnum::Catvfr8plus => "catvfr8plus",
            ContentRatingCatvfrRatingEnum::Catvfr13plus => "catvfr13plus",
            ContentRatingCatvfrRatingEnum::Catvfr16plus => "catvfr16plus",
            ContentRatingCatvfrRatingEnum::Catvfr18plus => "catvfr18plus",
            ContentRatingCatvfrRatingEnum::CatvfrUnrated => "catvfrUnrated",
            ContentRatingCatvfrRatingEnum::CatvfrE => "catvfrE",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCatvfrRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "catvfrUnspecified" => Ok(ContentRatingCatvfrRatingEnum::CatvfrUnspecified),
           "catvfrG" => Ok(ContentRatingCatvfrRatingEnum::CatvfrG),
           "catvfr8plus" => Ok(ContentRatingCatvfrRatingEnum::Catvfr8plus),
           "catvfr13plus" => Ok(ContentRatingCatvfrRatingEnum::Catvfr13plus),
           "catvfr16plus" => Ok(ContentRatingCatvfrRatingEnum::Catvfr16plus),
           "catvfr18plus" => Ok(ContentRatingCatvfrRatingEnum::Catvfr18plus),
           "catvfrUnrated" => Ok(ContentRatingCatvfrRatingEnum::CatvfrUnrated),
           "catvfrE" => Ok(ContentRatingCatvfrRatingEnum::CatvfrE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCatvfrRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCbfcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Central Board of Film Certification (CBFC - India) rating.
pub enum ContentRatingCbfcRatingEnum {
    
    /// "cbfcUnspecified"
    #[serde(rename="cbfcUnspecified")]
    CbfcUnspecified,
    

    /// U
    ///
    /// "cbfcU"
    #[serde(rename="cbfcU")]
    CbfcU,
    

    /// U/A
    ///
    /// "cbfcUA"
    #[serde(rename="cbfcUA")]
    CbfcUA,
    

    /// U/A 7+
    ///
    /// "cbfcUA7plus"
    #[serde(rename="cbfcUA7plus")]
    CbfcUA7plus,
    

    /// U/A 13+
    ///
    /// "cbfcUA13plus"
    #[serde(rename="cbfcUA13plus")]
    CbfcUA13plus,
    

    /// U/A 16+
    ///
    /// "cbfcUA16plus"
    #[serde(rename="cbfcUA16plus")]
    CbfcUA16plus,
    

    /// A
    ///
    /// "cbfcA"
    #[serde(rename="cbfcA")]
    CbfcA,
    

    /// S
    ///
    /// "cbfcS"
    #[serde(rename="cbfcS")]
    CbfcS,
    
    /// "cbfcUnrated"
    #[serde(rename="cbfcUnrated")]
    CbfcUnrated,
}

impl AsRef<str> for ContentRatingCbfcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCbfcRatingEnum::CbfcUnspecified => "cbfcUnspecified",
            ContentRatingCbfcRatingEnum::CbfcU => "cbfcU",
            ContentRatingCbfcRatingEnum::CbfcUA => "cbfcUA",
            ContentRatingCbfcRatingEnum::CbfcUA7plus => "cbfcUA7plus",
            ContentRatingCbfcRatingEnum::CbfcUA13plus => "cbfcUA13plus",
            ContentRatingCbfcRatingEnum::CbfcUA16plus => "cbfcUA16plus",
            ContentRatingCbfcRatingEnum::CbfcA => "cbfcA",
            ContentRatingCbfcRatingEnum::CbfcS => "cbfcS",
            ContentRatingCbfcRatingEnum::CbfcUnrated => "cbfcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCbfcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cbfcUnspecified" => Ok(ContentRatingCbfcRatingEnum::CbfcUnspecified),
           "cbfcU" => Ok(ContentRatingCbfcRatingEnum::CbfcU),
           "cbfcUA" => Ok(ContentRatingCbfcRatingEnum::CbfcUA),
           "cbfcUA7plus" => Ok(ContentRatingCbfcRatingEnum::CbfcUA7plus),
           "cbfcUA13plus" => Ok(ContentRatingCbfcRatingEnum::CbfcUA13plus),
           "cbfcUA16plus" => Ok(ContentRatingCbfcRatingEnum::CbfcUA16plus),
           "cbfcA" => Ok(ContentRatingCbfcRatingEnum::CbfcA),
           "cbfcS" => Ok(ContentRatingCbfcRatingEnum::CbfcS),
           "cbfcUnrated" => Ok(ContentRatingCbfcRatingEnum::CbfcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCbfcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCccRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Consejo de Calificación Cinematográfica (Chile) rating.
pub enum ContentRatingCccRatingEnum {
    
    /// "cccUnspecified"
    #[serde(rename="cccUnspecified")]
    CccUnspecified,
    

    /// Todo espectador
    ///
    /// "cccTe"
    #[serde(rename="cccTe")]
    CccTe,
    

    /// 6+ - Inconveniente para menores de 7 años
    ///
    /// "ccc6"
    #[serde(rename="ccc6")]
    Ccc6,
    

    /// 14+
    ///
    /// "ccc14"
    #[serde(rename="ccc14")]
    Ccc14,
    

    /// 18+
    ///
    /// "ccc18"
    #[serde(rename="ccc18")]
    Ccc18,
    

    /// 18+ - contenido excesivamente violento
    ///
    /// "ccc18v"
    #[serde(rename="ccc18v")]
    Ccc18v,
    

    /// 18+ - contenido pornográfico
    ///
    /// "ccc18s"
    #[serde(rename="ccc18s")]
    Ccc18s,
    
    /// "cccUnrated"
    #[serde(rename="cccUnrated")]
    CccUnrated,
}

impl AsRef<str> for ContentRatingCccRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCccRatingEnum::CccUnspecified => "cccUnspecified",
            ContentRatingCccRatingEnum::CccTe => "cccTe",
            ContentRatingCccRatingEnum::Ccc6 => "ccc6",
            ContentRatingCccRatingEnum::Ccc14 => "ccc14",
            ContentRatingCccRatingEnum::Ccc18 => "ccc18",
            ContentRatingCccRatingEnum::Ccc18v => "ccc18v",
            ContentRatingCccRatingEnum::Ccc18s => "ccc18s",
            ContentRatingCccRatingEnum::CccUnrated => "cccUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCccRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cccUnspecified" => Ok(ContentRatingCccRatingEnum::CccUnspecified),
           "cccTe" => Ok(ContentRatingCccRatingEnum::CccTe),
           "ccc6" => Ok(ContentRatingCccRatingEnum::Ccc6),
           "ccc14" => Ok(ContentRatingCccRatingEnum::Ccc14),
           "ccc18" => Ok(ContentRatingCccRatingEnum::Ccc18),
           "ccc18v" => Ok(ContentRatingCccRatingEnum::Ccc18v),
           "ccc18s" => Ok(ContentRatingCccRatingEnum::Ccc18s),
           "cccUnrated" => Ok(ContentRatingCccRatingEnum::CccUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCccRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCceRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Portugal's Comissão de Classificação de Espect´culos.
pub enum ContentRatingCceRatingEnum {
    
    /// "cceUnspecified"
    #[serde(rename="cceUnspecified")]
    CceUnspecified,
    

    /// 4
    ///
    /// "cceM4"
    #[serde(rename="cceM4")]
    CceM4,
    

    /// 6
    ///
    /// "cceM6"
    #[serde(rename="cceM6")]
    CceM6,
    

    /// 12
    ///
    /// "cceM12"
    #[serde(rename="cceM12")]
    CceM12,
    

    /// 16
    ///
    /// "cceM16"
    #[serde(rename="cceM16")]
    CceM16,
    

    /// 18
    ///
    /// "cceM18"
    #[serde(rename="cceM18")]
    CceM18,
    
    /// "cceUnrated"
    #[serde(rename="cceUnrated")]
    CceUnrated,
    

    /// 14
    ///
    /// "cceM14"
    #[serde(rename="cceM14")]
    CceM14,
}

impl AsRef<str> for ContentRatingCceRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCceRatingEnum::CceUnspecified => "cceUnspecified",
            ContentRatingCceRatingEnum::CceM4 => "cceM4",
            ContentRatingCceRatingEnum::CceM6 => "cceM6",
            ContentRatingCceRatingEnum::CceM12 => "cceM12",
            ContentRatingCceRatingEnum::CceM16 => "cceM16",
            ContentRatingCceRatingEnum::CceM18 => "cceM18",
            ContentRatingCceRatingEnum::CceUnrated => "cceUnrated",
            ContentRatingCceRatingEnum::CceM14 => "cceM14",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCceRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cceUnspecified" => Ok(ContentRatingCceRatingEnum::CceUnspecified),
           "cceM4" => Ok(ContentRatingCceRatingEnum::CceM4),
           "cceM6" => Ok(ContentRatingCceRatingEnum::CceM6),
           "cceM12" => Ok(ContentRatingCceRatingEnum::CceM12),
           "cceM16" => Ok(ContentRatingCceRatingEnum::CceM16),
           "cceM18" => Ok(ContentRatingCceRatingEnum::CceM18),
           "cceUnrated" => Ok(ContentRatingCceRatingEnum::CceUnrated),
           "cceM14" => Ok(ContentRatingCceRatingEnum::CceM14),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCceRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingChfilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Switzerland.
pub enum ContentRatingChfilmRatingEnum {
    
    /// "chfilmUnspecified"
    #[serde(rename="chfilmUnspecified")]
    ChfilmUnspecified,
    

    /// 0
    ///
    /// "chfilm0"
    #[serde(rename="chfilm0")]
    Chfilm0,
    

    /// 6
    ///
    /// "chfilm6"
    #[serde(rename="chfilm6")]
    Chfilm6,
    

    /// 12
    ///
    /// "chfilm12"
    #[serde(rename="chfilm12")]
    Chfilm12,
    

    /// 16
    ///
    /// "chfilm16"
    #[serde(rename="chfilm16")]
    Chfilm16,
    

    /// 18
    ///
    /// "chfilm18"
    #[serde(rename="chfilm18")]
    Chfilm18,
    
    /// "chfilmUnrated"
    #[serde(rename="chfilmUnrated")]
    ChfilmUnrated,
}

impl AsRef<str> for ContentRatingChfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingChfilmRatingEnum::ChfilmUnspecified => "chfilmUnspecified",
            ContentRatingChfilmRatingEnum::Chfilm0 => "chfilm0",
            ContentRatingChfilmRatingEnum::Chfilm6 => "chfilm6",
            ContentRatingChfilmRatingEnum::Chfilm12 => "chfilm12",
            ContentRatingChfilmRatingEnum::Chfilm16 => "chfilm16",
            ContentRatingChfilmRatingEnum::Chfilm18 => "chfilm18",
            ContentRatingChfilmRatingEnum::ChfilmUnrated => "chfilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingChfilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "chfilmUnspecified" => Ok(ContentRatingChfilmRatingEnum::ChfilmUnspecified),
           "chfilm0" => Ok(ContentRatingChfilmRatingEnum::Chfilm0),
           "chfilm6" => Ok(ContentRatingChfilmRatingEnum::Chfilm6),
           "chfilm12" => Ok(ContentRatingChfilmRatingEnum::Chfilm12),
           "chfilm16" => Ok(ContentRatingChfilmRatingEnum::Chfilm16),
           "chfilm18" => Ok(ContentRatingChfilmRatingEnum::Chfilm18),
           "chfilmUnrated" => Ok(ContentRatingChfilmRatingEnum::ChfilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingChfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingChvrsRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Canadian Home Video Rating System (CHVRS) rating.
pub enum ContentRatingChvrsRatingEnum {
    
    /// "chvrsUnspecified"
    #[serde(rename="chvrsUnspecified")]
    ChvrsUnspecified,
    

    /// G
    ///
    /// "chvrsG"
    #[serde(rename="chvrsG")]
    ChvrsG,
    

    /// PG
    ///
    /// "chvrsPg"
    #[serde(rename="chvrsPg")]
    ChvrsPg,
    

    /// 14A
    ///
    /// "chvrs14a"
    #[serde(rename="chvrs14a")]
    Chvrs14a,
    

    /// 18A
    ///
    /// "chvrs18a"
    #[serde(rename="chvrs18a")]
    Chvrs18a,
    

    /// R
    ///
    /// "chvrsR"
    #[serde(rename="chvrsR")]
    ChvrsR,
    

    /// E
    ///
    /// "chvrsE"
    #[serde(rename="chvrsE")]
    ChvrsE,
    
    /// "chvrsUnrated"
    #[serde(rename="chvrsUnrated")]
    ChvrsUnrated,
}

impl AsRef<str> for ContentRatingChvrsRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingChvrsRatingEnum::ChvrsUnspecified => "chvrsUnspecified",
            ContentRatingChvrsRatingEnum::ChvrsG => "chvrsG",
            ContentRatingChvrsRatingEnum::ChvrsPg => "chvrsPg",
            ContentRatingChvrsRatingEnum::Chvrs14a => "chvrs14a",
            ContentRatingChvrsRatingEnum::Chvrs18a => "chvrs18a",
            ContentRatingChvrsRatingEnum::ChvrsR => "chvrsR",
            ContentRatingChvrsRatingEnum::ChvrsE => "chvrsE",
            ContentRatingChvrsRatingEnum::ChvrsUnrated => "chvrsUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingChvrsRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "chvrsUnspecified" => Ok(ContentRatingChvrsRatingEnum::ChvrsUnspecified),
           "chvrsG" => Ok(ContentRatingChvrsRatingEnum::ChvrsG),
           "chvrsPg" => Ok(ContentRatingChvrsRatingEnum::ChvrsPg),
           "chvrs14a" => Ok(ContentRatingChvrsRatingEnum::Chvrs14a),
           "chvrs18a" => Ok(ContentRatingChvrsRatingEnum::Chvrs18a),
           "chvrsR" => Ok(ContentRatingChvrsRatingEnum::ChvrsR),
           "chvrsE" => Ok(ContentRatingChvrsRatingEnum::ChvrsE),
           "chvrsUnrated" => Ok(ContentRatingChvrsRatingEnum::ChvrsUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingChvrsRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCicfRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Commission de Contrôle des Films (Belgium).
pub enum ContentRatingCicfRatingEnum {
    
    /// "cicfUnspecified"
    #[serde(rename="cicfUnspecified")]
    CicfUnspecified,
    

    /// E
    ///
    /// "cicfE"
    #[serde(rename="cicfE")]
    CicfE,
    

    /// KT/EA
    ///
    /// "cicfKtEa"
    #[serde(rename="cicfKtEa")]
    CicfKtEa,
    

    /// KNT/ENA
    ///
    /// "cicfKntEna"
    #[serde(rename="cicfKntEna")]
    CicfKntEna,
    
    /// "cicfUnrated"
    #[serde(rename="cicfUnrated")]
    CicfUnrated,
}

impl AsRef<str> for ContentRatingCicfRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCicfRatingEnum::CicfUnspecified => "cicfUnspecified",
            ContentRatingCicfRatingEnum::CicfE => "cicfE",
            ContentRatingCicfRatingEnum::CicfKtEa => "cicfKtEa",
            ContentRatingCicfRatingEnum::CicfKntEna => "cicfKntEna",
            ContentRatingCicfRatingEnum::CicfUnrated => "cicfUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCicfRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cicfUnspecified" => Ok(ContentRatingCicfRatingEnum::CicfUnspecified),
           "cicfE" => Ok(ContentRatingCicfRatingEnum::CicfE),
           "cicfKtEa" => Ok(ContentRatingCicfRatingEnum::CicfKtEa),
           "cicfKntEna" => Ok(ContentRatingCicfRatingEnum::CicfKntEna),
           "cicfUnrated" => Ok(ContentRatingCicfRatingEnum::CicfUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCicfRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCnaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA).
pub enum ContentRatingCnaRatingEnum {
    
    /// "cnaUnspecified"
    #[serde(rename="cnaUnspecified")]
    CnaUnspecified,
    

    /// AP
    ///
    /// "cnaAp"
    #[serde(rename="cnaAp")]
    CnaAp,
    

    /// 12
    ///
    /// "cna12"
    #[serde(rename="cna12")]
    Cna12,
    

    /// 15
    ///
    /// "cna15"
    #[serde(rename="cna15")]
    Cna15,
    

    /// 18
    ///
    /// "cna18"
    #[serde(rename="cna18")]
    Cna18,
    

    /// 18+
    ///
    /// "cna18plus"
    #[serde(rename="cna18plus")]
    Cna18plus,
    
    /// "cnaUnrated"
    #[serde(rename="cnaUnrated")]
    CnaUnrated,
}

impl AsRef<str> for ContentRatingCnaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCnaRatingEnum::CnaUnspecified => "cnaUnspecified",
            ContentRatingCnaRatingEnum::CnaAp => "cnaAp",
            ContentRatingCnaRatingEnum::Cna12 => "cna12",
            ContentRatingCnaRatingEnum::Cna15 => "cna15",
            ContentRatingCnaRatingEnum::Cna18 => "cna18",
            ContentRatingCnaRatingEnum::Cna18plus => "cna18plus",
            ContentRatingCnaRatingEnum::CnaUnrated => "cnaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCnaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cnaUnspecified" => Ok(ContentRatingCnaRatingEnum::CnaUnspecified),
           "cnaAp" => Ok(ContentRatingCnaRatingEnum::CnaAp),
           "cna12" => Ok(ContentRatingCnaRatingEnum::Cna12),
           "cna15" => Ok(ContentRatingCnaRatingEnum::Cna15),
           "cna18" => Ok(ContentRatingCnaRatingEnum::Cna18),
           "cna18plus" => Ok(ContentRatingCnaRatingEnum::Cna18plus),
           "cnaUnrated" => Ok(ContentRatingCnaRatingEnum::CnaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCnaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCncRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating system in France - Commission de classification cinematographique
pub enum ContentRatingCncRatingEnum {
    
    /// "cncUnspecified"
    #[serde(rename="cncUnspecified")]
    CncUnspecified,
    

    /// T
    ///
    /// "cncT"
    #[serde(rename="cncT")]
    CncT,
    

    /// 10
    ///
    /// "cnc10"
    #[serde(rename="cnc10")]
    Cnc10,
    

    /// 12
    ///
    /// "cnc12"
    #[serde(rename="cnc12")]
    Cnc12,
    

    /// 16
    ///
    /// "cnc16"
    #[serde(rename="cnc16")]
    Cnc16,
    

    /// 18
    ///
    /// "cnc18"
    #[serde(rename="cnc18")]
    Cnc18,
    

    /// E
    ///
    /// "cncE"
    #[serde(rename="cncE")]
    CncE,
    

    /// interdiction
    ///
    /// "cncInterdiction"
    #[serde(rename="cncInterdiction")]
    CncInterdiction,
    
    /// "cncUnrated"
    #[serde(rename="cncUnrated")]
    CncUnrated,
}

impl AsRef<str> for ContentRatingCncRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCncRatingEnum::CncUnspecified => "cncUnspecified",
            ContentRatingCncRatingEnum::CncT => "cncT",
            ContentRatingCncRatingEnum::Cnc10 => "cnc10",
            ContentRatingCncRatingEnum::Cnc12 => "cnc12",
            ContentRatingCncRatingEnum::Cnc16 => "cnc16",
            ContentRatingCncRatingEnum::Cnc18 => "cnc18",
            ContentRatingCncRatingEnum::CncE => "cncE",
            ContentRatingCncRatingEnum::CncInterdiction => "cncInterdiction",
            ContentRatingCncRatingEnum::CncUnrated => "cncUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCncRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cncUnspecified" => Ok(ContentRatingCncRatingEnum::CncUnspecified),
           "cncT" => Ok(ContentRatingCncRatingEnum::CncT),
           "cnc10" => Ok(ContentRatingCncRatingEnum::Cnc10),
           "cnc12" => Ok(ContentRatingCncRatingEnum::Cnc12),
           "cnc16" => Ok(ContentRatingCncRatingEnum::Cnc16),
           "cnc18" => Ok(ContentRatingCncRatingEnum::Cnc18),
           "cncE" => Ok(ContentRatingCncRatingEnum::CncE),
           "cncInterdiction" => Ok(ContentRatingCncRatingEnum::CncInterdiction),
           "cncUnrated" => Ok(ContentRatingCncRatingEnum::CncUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCncRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCsaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content.
pub enum ContentRatingCsaRatingEnum {
    
    /// "csaUnspecified"
    #[serde(rename="csaUnspecified")]
    CsaUnspecified,
    

    /// T
    ///
    /// "csaT"
    #[serde(rename="csaT")]
    CsaT,
    

    /// 10
    ///
    /// "csa10"
    #[serde(rename="csa10")]
    Csa10,
    

    /// 12
    ///
    /// "csa12"
    #[serde(rename="csa12")]
    Csa12,
    

    /// 16
    ///
    /// "csa16"
    #[serde(rename="csa16")]
    Csa16,
    

    /// 18
    ///
    /// "csa18"
    #[serde(rename="csa18")]
    Csa18,
    

    /// Interdiction
    ///
    /// "csaInterdiction"
    #[serde(rename="csaInterdiction")]
    CsaInterdiction,
    
    /// "csaUnrated"
    #[serde(rename="csaUnrated")]
    CsaUnrated,
}

impl AsRef<str> for ContentRatingCsaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCsaRatingEnum::CsaUnspecified => "csaUnspecified",
            ContentRatingCsaRatingEnum::CsaT => "csaT",
            ContentRatingCsaRatingEnum::Csa10 => "csa10",
            ContentRatingCsaRatingEnum::Csa12 => "csa12",
            ContentRatingCsaRatingEnum::Csa16 => "csa16",
            ContentRatingCsaRatingEnum::Csa18 => "csa18",
            ContentRatingCsaRatingEnum::CsaInterdiction => "csaInterdiction",
            ContentRatingCsaRatingEnum::CsaUnrated => "csaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCsaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "csaUnspecified" => Ok(ContentRatingCsaRatingEnum::CsaUnspecified),
           "csaT" => Ok(ContentRatingCsaRatingEnum::CsaT),
           "csa10" => Ok(ContentRatingCsaRatingEnum::Csa10),
           "csa12" => Ok(ContentRatingCsaRatingEnum::Csa12),
           "csa16" => Ok(ContentRatingCsaRatingEnum::Csa16),
           "csa18" => Ok(ContentRatingCsaRatingEnum::Csa18),
           "csaInterdiction" => Ok(ContentRatingCsaRatingEnum::CsaInterdiction),
           "csaUnrated" => Ok(ContentRatingCsaRatingEnum::CsaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCsaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCscfRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF).
pub enum ContentRatingCscfRatingEnum {
    
    /// "cscfUnspecified"
    #[serde(rename="cscfUnspecified")]
    CscfUnspecified,
    

    /// AL
    ///
    /// "cscfAl"
    #[serde(rename="cscfAl")]
    CscfAl,
    

    /// A
    ///
    /// "cscfA"
    #[serde(rename="cscfA")]
    CscfA,
    

    /// 6
    ///
    /// "cscf6"
    #[serde(rename="cscf6")]
    Cscf6,
    

    /// 9
    ///
    /// "cscf9"
    #[serde(rename="cscf9")]
    Cscf9,
    

    /// 12
    ///
    /// "cscf12"
    #[serde(rename="cscf12")]
    Cscf12,
    

    /// 16
    ///
    /// "cscf16"
    #[serde(rename="cscf16")]
    Cscf16,
    

    /// 18
    ///
    /// "cscf18"
    #[serde(rename="cscf18")]
    Cscf18,
    
    /// "cscfUnrated"
    #[serde(rename="cscfUnrated")]
    CscfUnrated,
}

impl AsRef<str> for ContentRatingCscfRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCscfRatingEnum::CscfUnspecified => "cscfUnspecified",
            ContentRatingCscfRatingEnum::CscfAl => "cscfAl",
            ContentRatingCscfRatingEnum::CscfA => "cscfA",
            ContentRatingCscfRatingEnum::Cscf6 => "cscf6",
            ContentRatingCscfRatingEnum::Cscf9 => "cscf9",
            ContentRatingCscfRatingEnum::Cscf12 => "cscf12",
            ContentRatingCscfRatingEnum::Cscf16 => "cscf16",
            ContentRatingCscfRatingEnum::Cscf18 => "cscf18",
            ContentRatingCscfRatingEnum::CscfUnrated => "cscfUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCscfRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cscfUnspecified" => Ok(ContentRatingCscfRatingEnum::CscfUnspecified),
           "cscfAl" => Ok(ContentRatingCscfRatingEnum::CscfAl),
           "cscfA" => Ok(ContentRatingCscfRatingEnum::CscfA),
           "cscf6" => Ok(ContentRatingCscfRatingEnum::Cscf6),
           "cscf9" => Ok(ContentRatingCscfRatingEnum::Cscf9),
           "cscf12" => Ok(ContentRatingCscfRatingEnum::Cscf12),
           "cscf16" => Ok(ContentRatingCscfRatingEnum::Cscf16),
           "cscf18" => Ok(ContentRatingCscfRatingEnum::Cscf18),
           "cscfUnrated" => Ok(ContentRatingCscfRatingEnum::CscfUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCscfRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingCzfilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in the Czech Republic.
pub enum ContentRatingCzfilmRatingEnum {
    
    /// "czfilmUnspecified"
    #[serde(rename="czfilmUnspecified")]
    CzfilmUnspecified,
    

    /// U
    ///
    /// "czfilmU"
    #[serde(rename="czfilmU")]
    CzfilmU,
    

    /// 12
    ///
    /// "czfilm12"
    #[serde(rename="czfilm12")]
    Czfilm12,
    

    /// 14
    ///
    /// "czfilm14"
    #[serde(rename="czfilm14")]
    Czfilm14,
    

    /// 18
    ///
    /// "czfilm18"
    #[serde(rename="czfilm18")]
    Czfilm18,
    
    /// "czfilmUnrated"
    #[serde(rename="czfilmUnrated")]
    CzfilmUnrated,
}

impl AsRef<str> for ContentRatingCzfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCzfilmRatingEnum::CzfilmUnspecified => "czfilmUnspecified",
            ContentRatingCzfilmRatingEnum::CzfilmU => "czfilmU",
            ContentRatingCzfilmRatingEnum::Czfilm12 => "czfilm12",
            ContentRatingCzfilmRatingEnum::Czfilm14 => "czfilm14",
            ContentRatingCzfilmRatingEnum::Czfilm18 => "czfilm18",
            ContentRatingCzfilmRatingEnum::CzfilmUnrated => "czfilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingCzfilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "czfilmUnspecified" => Ok(ContentRatingCzfilmRatingEnum::CzfilmUnspecified),
           "czfilmU" => Ok(ContentRatingCzfilmRatingEnum::CzfilmU),
           "czfilm12" => Ok(ContentRatingCzfilmRatingEnum::Czfilm12),
           "czfilm14" => Ok(ContentRatingCzfilmRatingEnum::Czfilm14),
           "czfilm18" => Ok(ContentRatingCzfilmRatingEnum::Czfilm18),
           "czfilmUnrated" => Ok(ContentRatingCzfilmRatingEnum::CzfilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCzfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingDjctqRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating.
pub enum ContentRatingDjctqRatingEnum {
    
    /// "djctqUnspecified"
    #[serde(rename="djctqUnspecified")]
    DjctqUnspecified,
    

    /// L
    ///
    /// "djctqL"
    #[serde(rename="djctqL")]
    DjctqL,
    

    /// 10
    ///
    /// "djctq10"
    #[serde(rename="djctq10")]
    Djctq10,
    

    /// 12
    ///
    /// "djctq12"
    #[serde(rename="djctq12")]
    Djctq12,
    

    /// 14
    ///
    /// "djctq14"
    #[serde(rename="djctq14")]
    Djctq14,
    

    /// 16
    ///
    /// "djctq16"
    #[serde(rename="djctq16")]
    Djctq16,
    

    /// 18
    ///
    /// "djctq18"
    #[serde(rename="djctq18")]
    Djctq18,
    
    /// "djctqEr"
    #[serde(rename="djctqEr")]
    DjctqEr,
    
    /// "djctqL10"
    #[serde(rename="djctqL10")]
    DjctqL10,
    
    /// "djctqL12"
    #[serde(rename="djctqL12")]
    DjctqL12,
    
    /// "djctqL14"
    #[serde(rename="djctqL14")]
    DjctqL14,
    
    /// "djctqL16"
    #[serde(rename="djctqL16")]
    DjctqL16,
    
    /// "djctqL18"
    #[serde(rename="djctqL18")]
    DjctqL18,
    
    /// "djctq1012"
    #[serde(rename="djctq1012")]
    Djctq1012,
    
    /// "djctq1014"
    #[serde(rename="djctq1014")]
    Djctq1014,
    
    /// "djctq1016"
    #[serde(rename="djctq1016")]
    Djctq1016,
    
    /// "djctq1018"
    #[serde(rename="djctq1018")]
    Djctq1018,
    
    /// "djctq1214"
    #[serde(rename="djctq1214")]
    Djctq1214,
    
    /// "djctq1216"
    #[serde(rename="djctq1216")]
    Djctq1216,
    
    /// "djctq1218"
    #[serde(rename="djctq1218")]
    Djctq1218,
    
    /// "djctq1416"
    #[serde(rename="djctq1416")]
    Djctq1416,
    
    /// "djctq1418"
    #[serde(rename="djctq1418")]
    Djctq1418,
    
    /// "djctq1618"
    #[serde(rename="djctq1618")]
    Djctq1618,
    
    /// "djctqUnrated"
    #[serde(rename="djctqUnrated")]
    DjctqUnrated,
}

impl AsRef<str> for ContentRatingDjctqRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingDjctqRatingEnum::DjctqUnspecified => "djctqUnspecified",
            ContentRatingDjctqRatingEnum::DjctqL => "djctqL",
            ContentRatingDjctqRatingEnum::Djctq10 => "djctq10",
            ContentRatingDjctqRatingEnum::Djctq12 => "djctq12",
            ContentRatingDjctqRatingEnum::Djctq14 => "djctq14",
            ContentRatingDjctqRatingEnum::Djctq16 => "djctq16",
            ContentRatingDjctqRatingEnum::Djctq18 => "djctq18",
            ContentRatingDjctqRatingEnum::DjctqEr => "djctqEr",
            ContentRatingDjctqRatingEnum::DjctqL10 => "djctqL10",
            ContentRatingDjctqRatingEnum::DjctqL12 => "djctqL12",
            ContentRatingDjctqRatingEnum::DjctqL14 => "djctqL14",
            ContentRatingDjctqRatingEnum::DjctqL16 => "djctqL16",
            ContentRatingDjctqRatingEnum::DjctqL18 => "djctqL18",
            ContentRatingDjctqRatingEnum::Djctq1012 => "djctq1012",
            ContentRatingDjctqRatingEnum::Djctq1014 => "djctq1014",
            ContentRatingDjctqRatingEnum::Djctq1016 => "djctq1016",
            ContentRatingDjctqRatingEnum::Djctq1018 => "djctq1018",
            ContentRatingDjctqRatingEnum::Djctq1214 => "djctq1214",
            ContentRatingDjctqRatingEnum::Djctq1216 => "djctq1216",
            ContentRatingDjctqRatingEnum::Djctq1218 => "djctq1218",
            ContentRatingDjctqRatingEnum::Djctq1416 => "djctq1416",
            ContentRatingDjctqRatingEnum::Djctq1418 => "djctq1418",
            ContentRatingDjctqRatingEnum::Djctq1618 => "djctq1618",
            ContentRatingDjctqRatingEnum::DjctqUnrated => "djctqUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingDjctqRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "djctqUnspecified" => Ok(ContentRatingDjctqRatingEnum::DjctqUnspecified),
           "djctqL" => Ok(ContentRatingDjctqRatingEnum::DjctqL),
           "djctq10" => Ok(ContentRatingDjctqRatingEnum::Djctq10),
           "djctq12" => Ok(ContentRatingDjctqRatingEnum::Djctq12),
           "djctq14" => Ok(ContentRatingDjctqRatingEnum::Djctq14),
           "djctq16" => Ok(ContentRatingDjctqRatingEnum::Djctq16),
           "djctq18" => Ok(ContentRatingDjctqRatingEnum::Djctq18),
           "djctqEr" => Ok(ContentRatingDjctqRatingEnum::DjctqEr),
           "djctqL10" => Ok(ContentRatingDjctqRatingEnum::DjctqL10),
           "djctqL12" => Ok(ContentRatingDjctqRatingEnum::DjctqL12),
           "djctqL14" => Ok(ContentRatingDjctqRatingEnum::DjctqL14),
           "djctqL16" => Ok(ContentRatingDjctqRatingEnum::DjctqL16),
           "djctqL18" => Ok(ContentRatingDjctqRatingEnum::DjctqL18),
           "djctq1012" => Ok(ContentRatingDjctqRatingEnum::Djctq1012),
           "djctq1014" => Ok(ContentRatingDjctqRatingEnum::Djctq1014),
           "djctq1016" => Ok(ContentRatingDjctqRatingEnum::Djctq1016),
           "djctq1018" => Ok(ContentRatingDjctqRatingEnum::Djctq1018),
           "djctq1214" => Ok(ContentRatingDjctqRatingEnum::Djctq1214),
           "djctq1216" => Ok(ContentRatingDjctqRatingEnum::Djctq1216),
           "djctq1218" => Ok(ContentRatingDjctqRatingEnum::Djctq1218),
           "djctq1416" => Ok(ContentRatingDjctqRatingEnum::Djctq1416),
           "djctq1418" => Ok(ContentRatingDjctqRatingEnum::Djctq1418),
           "djctq1618" => Ok(ContentRatingDjctqRatingEnum::Djctq1618),
           "djctqUnrated" => Ok(ContentRatingDjctqRatingEnum::DjctqUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingDjctqRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingDjctqRatingReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reasons that explain why the video received its DJCQT (Brazil) rating.
pub enum ContentRatingDjctqRatingReasonsEnum {
    
    /// "djctqRatingReasonUnspecified"
    #[serde(rename="djctqRatingReasonUnspecified")]
    DjctqRatingReasonUnspecified,
    

    /// Brazil rating content descriptors. See http://go/brazilratings section F. Violência (Violence)
    ///
    /// "djctqViolence"
    #[serde(rename="djctqViolence")]
    DjctqViolence,
    

    /// Violência extrema (Extreme violence)
    ///
    /// "djctqExtremeViolence"
    #[serde(rename="djctqExtremeViolence")]
    DjctqExtremeViolence,
    

    /// Conteúdo sexual (Sexual content)
    ///
    /// "djctqSexualContent"
    #[serde(rename="djctqSexualContent")]
    DjctqSexualContent,
    

    /// Nudez (Nudity)
    ///
    /// "djctqNudity"
    #[serde(rename="djctqNudity")]
    DjctqNudity,
    

    /// Sexo (Sex)
    ///
    /// "djctqSex"
    #[serde(rename="djctqSex")]
    DjctqSex,
    

    /// Sexo Explícito (Explicit sex)
    ///
    /// "djctqExplicitSex"
    #[serde(rename="djctqExplicitSex")]
    DjctqExplicitSex,
    

    /// Drogas (Drugs)
    ///
    /// "djctqDrugs"
    #[serde(rename="djctqDrugs")]
    DjctqDrugs,
    

    /// Drogas Lícitas (Legal drugs)
    ///
    /// "djctqLegalDrugs"
    #[serde(rename="djctqLegalDrugs")]
    DjctqLegalDrugs,
    

    /// Drogas Ilícitas (Illegal drugs)
    ///
    /// "djctqIllegalDrugs"
    #[serde(rename="djctqIllegalDrugs")]
    DjctqIllegalDrugs,
    

    /// Linguagem Imprópria (Inappropriate language)
    ///
    /// "djctqInappropriateLanguage"
    #[serde(rename="djctqInappropriateLanguage")]
    DjctqInappropriateLanguage,
    

    /// Atos Criminosos (Criminal Acts)
    ///
    /// "djctqCriminalActs"
    #[serde(rename="djctqCriminalActs")]
    DjctqCriminalActs,
    

    /// Conteúdo Impactante (Impacting content)
    ///
    /// "djctqImpactingContent"
    #[serde(rename="djctqImpactingContent")]
    DjctqImpactingContent,
}

impl AsRef<str> for ContentRatingDjctqRatingReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingDjctqRatingReasonsEnum::DjctqRatingReasonUnspecified => "djctqRatingReasonUnspecified",
            ContentRatingDjctqRatingReasonsEnum::DjctqViolence => "djctqViolence",
            ContentRatingDjctqRatingReasonsEnum::DjctqExtremeViolence => "djctqExtremeViolence",
            ContentRatingDjctqRatingReasonsEnum::DjctqSexualContent => "djctqSexualContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqNudity => "djctqNudity",
            ContentRatingDjctqRatingReasonsEnum::DjctqSex => "djctqSex",
            ContentRatingDjctqRatingReasonsEnum::DjctqExplicitSex => "djctqExplicitSex",
            ContentRatingDjctqRatingReasonsEnum::DjctqDrugs => "djctqDrugs",
            ContentRatingDjctqRatingReasonsEnum::DjctqLegalDrugs => "djctqLegalDrugs",
            ContentRatingDjctqRatingReasonsEnum::DjctqIllegalDrugs => "djctqIllegalDrugs",
            ContentRatingDjctqRatingReasonsEnum::DjctqInappropriateLanguage => "djctqInappropriateLanguage",
            ContentRatingDjctqRatingReasonsEnum::DjctqCriminalActs => "djctqCriminalActs",
            ContentRatingDjctqRatingReasonsEnum::DjctqImpactingContent => "djctqImpactingContent",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingDjctqRatingReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "djctqRatingReasonUnspecified" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqRatingReasonUnspecified),
           "djctqViolence" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqViolence),
           "djctqExtremeViolence" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqExtremeViolence),
           "djctqSexualContent" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqSexualContent),
           "djctqNudity" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqNudity),
           "djctqSex" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqSex),
           "djctqExplicitSex" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqExplicitSex),
           "djctqDrugs" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqDrugs),
           "djctqLegalDrugs" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqLegalDrugs),
           "djctqIllegalDrugs" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqIllegalDrugs),
           "djctqInappropriateLanguage" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqInappropriateLanguage),
           "djctqCriminalActs" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqCriminalActs),
           "djctqImpactingContent" => Ok(ContentRatingDjctqRatingReasonsEnum::DjctqImpactingContent),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingDjctqRatingReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingEcbmctRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism
pub enum ContentRatingEcbmctRatingEnum {
    
    /// "ecbmctUnspecified"
    #[serde(rename="ecbmctUnspecified")]
    EcbmctUnspecified,
    

    /// G
    ///
    /// "ecbmctG"
    #[serde(rename="ecbmctG")]
    EcbmctG,
    

    /// 7A
    ///
    /// "ecbmct7a"
    #[serde(rename="ecbmct7a")]
    Ecbmct7a,
    

    /// 7+
    ///
    /// "ecbmct7plus"
    #[serde(rename="ecbmct7plus")]
    Ecbmct7plus,
    

    /// 13A
    ///
    /// "ecbmct13a"
    #[serde(rename="ecbmct13a")]
    Ecbmct13a,
    

    /// 13+
    ///
    /// "ecbmct13plus"
    #[serde(rename="ecbmct13plus")]
    Ecbmct13plus,
    

    /// 15A
    ///
    /// "ecbmct15a"
    #[serde(rename="ecbmct15a")]
    Ecbmct15a,
    

    /// 15+
    ///
    /// "ecbmct15plus"
    #[serde(rename="ecbmct15plus")]
    Ecbmct15plus,
    

    /// 18+
    ///
    /// "ecbmct18plus"
    #[serde(rename="ecbmct18plus")]
    Ecbmct18plus,
    
    /// "ecbmctUnrated"
    #[serde(rename="ecbmctUnrated")]
    EcbmctUnrated,
}

impl AsRef<str> for ContentRatingEcbmctRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEcbmctRatingEnum::EcbmctUnspecified => "ecbmctUnspecified",
            ContentRatingEcbmctRatingEnum::EcbmctG => "ecbmctG",
            ContentRatingEcbmctRatingEnum::Ecbmct7a => "ecbmct7a",
            ContentRatingEcbmctRatingEnum::Ecbmct7plus => "ecbmct7plus",
            ContentRatingEcbmctRatingEnum::Ecbmct13a => "ecbmct13a",
            ContentRatingEcbmctRatingEnum::Ecbmct13plus => "ecbmct13plus",
            ContentRatingEcbmctRatingEnum::Ecbmct15a => "ecbmct15a",
            ContentRatingEcbmctRatingEnum::Ecbmct15plus => "ecbmct15plus",
            ContentRatingEcbmctRatingEnum::Ecbmct18plus => "ecbmct18plus",
            ContentRatingEcbmctRatingEnum::EcbmctUnrated => "ecbmctUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingEcbmctRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ecbmctUnspecified" => Ok(ContentRatingEcbmctRatingEnum::EcbmctUnspecified),
           "ecbmctG" => Ok(ContentRatingEcbmctRatingEnum::EcbmctG),
           "ecbmct7a" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct7a),
           "ecbmct7plus" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct7plus),
           "ecbmct13a" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct13a),
           "ecbmct13plus" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct13plus),
           "ecbmct15a" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct15a),
           "ecbmct15plus" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct15plus),
           "ecbmct18plus" => Ok(ContentRatingEcbmctRatingEnum::Ecbmct18plus),
           "ecbmctUnrated" => Ok(ContentRatingEcbmctRatingEnum::EcbmctUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEcbmctRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingEefilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Estonia.
pub enum ContentRatingEefilmRatingEnum {
    
    /// "eefilmUnspecified"
    #[serde(rename="eefilmUnspecified")]
    EefilmUnspecified,
    

    /// Pere
    ///
    /// "eefilmPere"
    #[serde(rename="eefilmPere")]
    EefilmPere,
    

    /// L
    ///
    /// "eefilmL"
    #[serde(rename="eefilmL")]
    EefilmL,
    

    /// MS-6
    ///
    /// "eefilmMs6"
    #[serde(rename="eefilmMs6")]
    EefilmMs6,
    

    /// K-6
    ///
    /// "eefilmK6"
    #[serde(rename="eefilmK6")]
    EefilmK6,
    

    /// MS-12
    ///
    /// "eefilmMs12"
    #[serde(rename="eefilmMs12")]
    EefilmMs12,
    

    /// K-12
    ///
    /// "eefilmK12"
    #[serde(rename="eefilmK12")]
    EefilmK12,
    

    /// K-14
    ///
    /// "eefilmK14"
    #[serde(rename="eefilmK14")]
    EefilmK14,
    

    /// K-16
    ///
    /// "eefilmK16"
    #[serde(rename="eefilmK16")]
    EefilmK16,
    
    /// "eefilmUnrated"
    #[serde(rename="eefilmUnrated")]
    EefilmUnrated,
}

impl AsRef<str> for ContentRatingEefilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEefilmRatingEnum::EefilmUnspecified => "eefilmUnspecified",
            ContentRatingEefilmRatingEnum::EefilmPere => "eefilmPere",
            ContentRatingEefilmRatingEnum::EefilmL => "eefilmL",
            ContentRatingEefilmRatingEnum::EefilmMs6 => "eefilmMs6",
            ContentRatingEefilmRatingEnum::EefilmK6 => "eefilmK6",
            ContentRatingEefilmRatingEnum::EefilmMs12 => "eefilmMs12",
            ContentRatingEefilmRatingEnum::EefilmK12 => "eefilmK12",
            ContentRatingEefilmRatingEnum::EefilmK14 => "eefilmK14",
            ContentRatingEefilmRatingEnum::EefilmK16 => "eefilmK16",
            ContentRatingEefilmRatingEnum::EefilmUnrated => "eefilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingEefilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "eefilmUnspecified" => Ok(ContentRatingEefilmRatingEnum::EefilmUnspecified),
           "eefilmPere" => Ok(ContentRatingEefilmRatingEnum::EefilmPere),
           "eefilmL" => Ok(ContentRatingEefilmRatingEnum::EefilmL),
           "eefilmMs6" => Ok(ContentRatingEefilmRatingEnum::EefilmMs6),
           "eefilmK6" => Ok(ContentRatingEefilmRatingEnum::EefilmK6),
           "eefilmMs12" => Ok(ContentRatingEefilmRatingEnum::EefilmMs12),
           "eefilmK12" => Ok(ContentRatingEefilmRatingEnum::EefilmK12),
           "eefilmK14" => Ok(ContentRatingEefilmRatingEnum::EefilmK14),
           "eefilmK16" => Ok(ContentRatingEefilmRatingEnum::EefilmK16),
           "eefilmUnrated" => Ok(ContentRatingEefilmRatingEnum::EefilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEefilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingEgfilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Egypt.
pub enum ContentRatingEgfilmRatingEnum {
    
    /// "egfilmUnspecified"
    #[serde(rename="egfilmUnspecified")]
    EgfilmUnspecified,
    

    /// GN
    ///
    /// "egfilmGn"
    #[serde(rename="egfilmGn")]
    EgfilmGn,
    

    /// 18
    ///
    /// "egfilm18"
    #[serde(rename="egfilm18")]
    Egfilm18,
    

    /// BN
    ///
    /// "egfilmBn"
    #[serde(rename="egfilmBn")]
    EgfilmBn,
    
    /// "egfilmUnrated"
    #[serde(rename="egfilmUnrated")]
    EgfilmUnrated,
}

impl AsRef<str> for ContentRatingEgfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEgfilmRatingEnum::EgfilmUnspecified => "egfilmUnspecified",
            ContentRatingEgfilmRatingEnum::EgfilmGn => "egfilmGn",
            ContentRatingEgfilmRatingEnum::Egfilm18 => "egfilm18",
            ContentRatingEgfilmRatingEnum::EgfilmBn => "egfilmBn",
            ContentRatingEgfilmRatingEnum::EgfilmUnrated => "egfilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingEgfilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "egfilmUnspecified" => Ok(ContentRatingEgfilmRatingEnum::EgfilmUnspecified),
           "egfilmGn" => Ok(ContentRatingEgfilmRatingEnum::EgfilmGn),
           "egfilm18" => Ok(ContentRatingEgfilmRatingEnum::Egfilm18),
           "egfilmBn" => Ok(ContentRatingEgfilmRatingEnum::EgfilmBn),
           "egfilmUnrated" => Ok(ContentRatingEgfilmRatingEnum::EgfilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEgfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingEirinRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Eirin (映倫) rating. Eirin is the Japanese rating system.
pub enum ContentRatingEirinRatingEnum {
    
    /// "eirinUnspecified"
    #[serde(rename="eirinUnspecified")]
    EirinUnspecified,
    

    /// G
    ///
    /// "eirinG"
    #[serde(rename="eirinG")]
    EirinG,
    

    /// PG-12
    ///
    /// "eirinPg12"
    #[serde(rename="eirinPg12")]
    EirinPg12,
    

    /// R15+
    ///
    /// "eirinR15plus"
    #[serde(rename="eirinR15plus")]
    EirinR15plus,
    

    /// R18+
    ///
    /// "eirinR18plus"
    #[serde(rename="eirinR18plus")]
    EirinR18plus,
    
    /// "eirinUnrated"
    #[serde(rename="eirinUnrated")]
    EirinUnrated,
}

impl AsRef<str> for ContentRatingEirinRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEirinRatingEnum::EirinUnspecified => "eirinUnspecified",
            ContentRatingEirinRatingEnum::EirinG => "eirinG",
            ContentRatingEirinRatingEnum::EirinPg12 => "eirinPg12",
            ContentRatingEirinRatingEnum::EirinR15plus => "eirinR15plus",
            ContentRatingEirinRatingEnum::EirinR18plus => "eirinR18plus",
            ContentRatingEirinRatingEnum::EirinUnrated => "eirinUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingEirinRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "eirinUnspecified" => Ok(ContentRatingEirinRatingEnum::EirinUnspecified),
           "eirinG" => Ok(ContentRatingEirinRatingEnum::EirinG),
           "eirinPg12" => Ok(ContentRatingEirinRatingEnum::EirinPg12),
           "eirinR15plus" => Ok(ContentRatingEirinRatingEnum::EirinR15plus),
           "eirinR18plus" => Ok(ContentRatingEirinRatingEnum::EirinR18plus),
           "eirinUnrated" => Ok(ContentRatingEirinRatingEnum::EirinUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEirinRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingFcbmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Malaysia's Film Censorship Board.
pub enum ContentRatingFcbmRatingEnum {
    
    /// "fcbmUnspecified"
    #[serde(rename="fcbmUnspecified")]
    FcbmUnspecified,
    

    /// U
    ///
    /// "fcbmU"
    #[serde(rename="fcbmU")]
    FcbmU,
    

    /// PG13
    ///
    /// "fcbmPg13"
    #[serde(rename="fcbmPg13")]
    FcbmPg13,
    

    /// P13
    ///
    /// "fcbmP13"
    #[serde(rename="fcbmP13")]
    FcbmP13,
    

    /// 18
    ///
    /// "fcbm18"
    #[serde(rename="fcbm18")]
    Fcbm18,
    

    /// 18SX
    ///
    /// "fcbm18sx"
    #[serde(rename="fcbm18sx")]
    Fcbm18sx,
    

    /// 18PA
    ///
    /// "fcbm18pa"
    #[serde(rename="fcbm18pa")]
    Fcbm18pa,
    

    /// 18SG
    ///
    /// "fcbm18sg"
    #[serde(rename="fcbm18sg")]
    Fcbm18sg,
    

    /// 18PL
    ///
    /// "fcbm18pl"
    #[serde(rename="fcbm18pl")]
    Fcbm18pl,
    
    /// "fcbmUnrated"
    #[serde(rename="fcbmUnrated")]
    FcbmUnrated,
}

impl AsRef<str> for ContentRatingFcbmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFcbmRatingEnum::FcbmUnspecified => "fcbmUnspecified",
            ContentRatingFcbmRatingEnum::FcbmU => "fcbmU",
            ContentRatingFcbmRatingEnum::FcbmPg13 => "fcbmPg13",
            ContentRatingFcbmRatingEnum::FcbmP13 => "fcbmP13",
            ContentRatingFcbmRatingEnum::Fcbm18 => "fcbm18",
            ContentRatingFcbmRatingEnum::Fcbm18sx => "fcbm18sx",
            ContentRatingFcbmRatingEnum::Fcbm18pa => "fcbm18pa",
            ContentRatingFcbmRatingEnum::Fcbm18sg => "fcbm18sg",
            ContentRatingFcbmRatingEnum::Fcbm18pl => "fcbm18pl",
            ContentRatingFcbmRatingEnum::FcbmUnrated => "fcbmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingFcbmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fcbmUnspecified" => Ok(ContentRatingFcbmRatingEnum::FcbmUnspecified),
           "fcbmU" => Ok(ContentRatingFcbmRatingEnum::FcbmU),
           "fcbmPg13" => Ok(ContentRatingFcbmRatingEnum::FcbmPg13),
           "fcbmP13" => Ok(ContentRatingFcbmRatingEnum::FcbmP13),
           "fcbm18" => Ok(ContentRatingFcbmRatingEnum::Fcbm18),
           "fcbm18sx" => Ok(ContentRatingFcbmRatingEnum::Fcbm18sx),
           "fcbm18pa" => Ok(ContentRatingFcbmRatingEnum::Fcbm18pa),
           "fcbm18sg" => Ok(ContentRatingFcbmRatingEnum::Fcbm18sg),
           "fcbm18pl" => Ok(ContentRatingFcbmRatingEnum::Fcbm18pl),
           "fcbmUnrated" => Ok(ContentRatingFcbmRatingEnum::FcbmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFcbmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingFcoRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration.
pub enum ContentRatingFcoRatingEnum {
    
    /// "fcoUnspecified"
    #[serde(rename="fcoUnspecified")]
    FcoUnspecified,
    

    /// I
    ///
    /// "fcoI"
    #[serde(rename="fcoI")]
    FcoI,
    

    /// IIA
    ///
    /// "fcoIia"
    #[serde(rename="fcoIia")]
    FcoIia,
    

    /// IIB
    ///
    /// "fcoIib"
    #[serde(rename="fcoIib")]
    FcoIib,
    

    /// II
    ///
    /// "fcoIi"
    #[serde(rename="fcoIi")]
    FcoIi,
    

    /// III
    ///
    /// "fcoIii"
    #[serde(rename="fcoIii")]
    FcoIii,
    
    /// "fcoUnrated"
    #[serde(rename="fcoUnrated")]
    FcoUnrated,
}

impl AsRef<str> for ContentRatingFcoRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFcoRatingEnum::FcoUnspecified => "fcoUnspecified",
            ContentRatingFcoRatingEnum::FcoI => "fcoI",
            ContentRatingFcoRatingEnum::FcoIia => "fcoIia",
            ContentRatingFcoRatingEnum::FcoIib => "fcoIib",
            ContentRatingFcoRatingEnum::FcoIi => "fcoIi",
            ContentRatingFcoRatingEnum::FcoIii => "fcoIii",
            ContentRatingFcoRatingEnum::FcoUnrated => "fcoUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingFcoRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fcoUnspecified" => Ok(ContentRatingFcoRatingEnum::FcoUnspecified),
           "fcoI" => Ok(ContentRatingFcoRatingEnum::FcoI),
           "fcoIia" => Ok(ContentRatingFcoRatingEnum::FcoIia),
           "fcoIib" => Ok(ContentRatingFcoRatingEnum::FcoIib),
           "fcoIi" => Ok(ContentRatingFcoRatingEnum::FcoIi),
           "fcoIii" => Ok(ContentRatingFcoRatingEnum::FcoIii),
           "fcoUnrated" => Ok(ContentRatingFcoRatingEnum::FcoUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFcoRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingFmocRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This property has been deprecated. Use the contentDetails.contentRating.cncRating instead.
pub enum ContentRatingFmocRatingEnum {
    
    /// "fmocUnspecified"
    #[serde(rename="fmocUnspecified")]
    FmocUnspecified,
    

    /// U
    ///
    /// "fmocU"
    #[serde(rename="fmocU")]
    FmocU,
    

    /// 10
    ///
    /// "fmoc10"
    #[serde(rename="fmoc10")]
    Fmoc10,
    

    /// 12
    ///
    /// "fmoc12"
    #[serde(rename="fmoc12")]
    Fmoc12,
    

    /// 16
    ///
    /// "fmoc16"
    #[serde(rename="fmoc16")]
    Fmoc16,
    

    /// 18
    ///
    /// "fmoc18"
    #[serde(rename="fmoc18")]
    Fmoc18,
    

    /// E
    ///
    /// "fmocE"
    #[serde(rename="fmocE")]
    FmocE,
    
    /// "fmocUnrated"
    #[serde(rename="fmocUnrated")]
    FmocUnrated,
}

impl AsRef<str> for ContentRatingFmocRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFmocRatingEnum::FmocUnspecified => "fmocUnspecified",
            ContentRatingFmocRatingEnum::FmocU => "fmocU",
            ContentRatingFmocRatingEnum::Fmoc10 => "fmoc10",
            ContentRatingFmocRatingEnum::Fmoc12 => "fmoc12",
            ContentRatingFmocRatingEnum::Fmoc16 => "fmoc16",
            ContentRatingFmocRatingEnum::Fmoc18 => "fmoc18",
            ContentRatingFmocRatingEnum::FmocE => "fmocE",
            ContentRatingFmocRatingEnum::FmocUnrated => "fmocUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingFmocRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fmocUnspecified" => Ok(ContentRatingFmocRatingEnum::FmocUnspecified),
           "fmocU" => Ok(ContentRatingFmocRatingEnum::FmocU),
           "fmoc10" => Ok(ContentRatingFmocRatingEnum::Fmoc10),
           "fmoc12" => Ok(ContentRatingFmocRatingEnum::Fmoc12),
           "fmoc16" => Ok(ContentRatingFmocRatingEnum::Fmoc16),
           "fmoc18" => Ok(ContentRatingFmocRatingEnum::Fmoc18),
           "fmocE" => Ok(ContentRatingFmocRatingEnum::FmocE),
           "fmocUnrated" => Ok(ContentRatingFmocRatingEnum::FmocUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFmocRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingFpbRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from South Africa's Film and Publication Board.
pub enum ContentRatingFpbRatingEnum {
    
    /// "fpbUnspecified"
    #[serde(rename="fpbUnspecified")]
    FpbUnspecified,
    

    /// A
    ///
    /// "fpbA"
    #[serde(rename="fpbA")]
    FpbA,
    

    /// PG
    ///
    /// "fpbPg"
    #[serde(rename="fpbPg")]
    FpbPg,
    

    /// 7-9PG
    ///
    /// "fpb79Pg"
    #[serde(rename="fpb79Pg")]
    Fpb79Pg,
    

    /// 10-12PG
    ///
    /// "fpb1012Pg"
    #[serde(rename="fpb1012Pg")]
    Fpb1012Pg,
    

    /// 13
    ///
    /// "fpb13"
    #[serde(rename="fpb13")]
    Fpb13,
    

    /// 16
    ///
    /// "fpb16"
    #[serde(rename="fpb16")]
    Fpb16,
    

    /// 18
    ///
    /// "fpb18"
    #[serde(rename="fpb18")]
    Fpb18,
    

    /// X18
    ///
    /// "fpbX18"
    #[serde(rename="fpbX18")]
    FpbX18,
    

    /// XX
    ///
    /// "fpbXx"
    #[serde(rename="fpbXx")]
    FpbXx,
    
    /// "fpbUnrated"
    #[serde(rename="fpbUnrated")]
    FpbUnrated,
    

    /// 10
    ///
    /// "fpb10"
    #[serde(rename="fpb10")]
    Fpb10,
}

impl AsRef<str> for ContentRatingFpbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFpbRatingEnum::FpbUnspecified => "fpbUnspecified",
            ContentRatingFpbRatingEnum::FpbA => "fpbA",
            ContentRatingFpbRatingEnum::FpbPg => "fpbPg",
            ContentRatingFpbRatingEnum::Fpb79Pg => "fpb79Pg",
            ContentRatingFpbRatingEnum::Fpb1012Pg => "fpb1012Pg",
            ContentRatingFpbRatingEnum::Fpb13 => "fpb13",
            ContentRatingFpbRatingEnum::Fpb16 => "fpb16",
            ContentRatingFpbRatingEnum::Fpb18 => "fpb18",
            ContentRatingFpbRatingEnum::FpbX18 => "fpbX18",
            ContentRatingFpbRatingEnum::FpbXx => "fpbXx",
            ContentRatingFpbRatingEnum::FpbUnrated => "fpbUnrated",
            ContentRatingFpbRatingEnum::Fpb10 => "fpb10",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingFpbRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fpbUnspecified" => Ok(ContentRatingFpbRatingEnum::FpbUnspecified),
           "fpbA" => Ok(ContentRatingFpbRatingEnum::FpbA),
           "fpbPg" => Ok(ContentRatingFpbRatingEnum::FpbPg),
           "fpb79Pg" => Ok(ContentRatingFpbRatingEnum::Fpb79Pg),
           "fpb1012Pg" => Ok(ContentRatingFpbRatingEnum::Fpb1012Pg),
           "fpb13" => Ok(ContentRatingFpbRatingEnum::Fpb13),
           "fpb16" => Ok(ContentRatingFpbRatingEnum::Fpb16),
           "fpb18" => Ok(ContentRatingFpbRatingEnum::Fpb18),
           "fpbX18" => Ok(ContentRatingFpbRatingEnum::FpbX18),
           "fpbXx" => Ok(ContentRatingFpbRatingEnum::FpbXx),
           "fpbUnrated" => Ok(ContentRatingFpbRatingEnum::FpbUnrated),
           "fpb10" => Ok(ContentRatingFpbRatingEnum::Fpb10),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFpbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingFpbRatingReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Reasons that explain why the video received its FPB (South Africa) rating.
pub enum ContentRatingFpbRatingReasonsEnum {
    
    /// "fpbRatingReasonUnspecified"
    #[serde(rename="fpbRatingReasonUnspecified")]
    FpbRatingReasonUnspecified,
    

    /// South Africa rating content descriptors.
    ///
    /// "fpbBlasphemy"
    #[serde(rename="fpbBlasphemy")]
    FpbBlasphemy,
    
    /// "fpbLanguage"
    #[serde(rename="fpbLanguage")]
    FpbLanguage,
    
    /// "fpbNudity"
    #[serde(rename="fpbNudity")]
    FpbNudity,
    
    /// "fpbPrejudice"
    #[serde(rename="fpbPrejudice")]
    FpbPrejudice,
    
    /// "fpbSex"
    #[serde(rename="fpbSex")]
    FpbSex,
    
    /// "fpbViolence"
    #[serde(rename="fpbViolence")]
    FpbViolence,
    
    /// "fpbDrugs"
    #[serde(rename="fpbDrugs")]
    FpbDrugs,
    
    /// "fpbSexualViolence"
    #[serde(rename="fpbSexualViolence")]
    FpbSexualViolence,
    
    /// "fpbHorror"
    #[serde(rename="fpbHorror")]
    FpbHorror,
    
    /// "fpbCriminalTechniques"
    #[serde(rename="fpbCriminalTechniques")]
    FpbCriminalTechniques,
    
    /// "fpbImitativeActsTechniques"
    #[serde(rename="fpbImitativeActsTechniques")]
    FpbImitativeActsTechniques,
}

impl AsRef<str> for ContentRatingFpbRatingReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFpbRatingReasonsEnum::FpbRatingReasonUnspecified => "fpbRatingReasonUnspecified",
            ContentRatingFpbRatingReasonsEnum::FpbBlasphemy => "fpbBlasphemy",
            ContentRatingFpbRatingReasonsEnum::FpbLanguage => "fpbLanguage",
            ContentRatingFpbRatingReasonsEnum::FpbNudity => "fpbNudity",
            ContentRatingFpbRatingReasonsEnum::FpbPrejudice => "fpbPrejudice",
            ContentRatingFpbRatingReasonsEnum::FpbSex => "fpbSex",
            ContentRatingFpbRatingReasonsEnum::FpbViolence => "fpbViolence",
            ContentRatingFpbRatingReasonsEnum::FpbDrugs => "fpbDrugs",
            ContentRatingFpbRatingReasonsEnum::FpbSexualViolence => "fpbSexualViolence",
            ContentRatingFpbRatingReasonsEnum::FpbHorror => "fpbHorror",
            ContentRatingFpbRatingReasonsEnum::FpbCriminalTechniques => "fpbCriminalTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbImitativeActsTechniques => "fpbImitativeActsTechniques",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingFpbRatingReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fpbRatingReasonUnspecified" => Ok(ContentRatingFpbRatingReasonsEnum::FpbRatingReasonUnspecified),
           "fpbBlasphemy" => Ok(ContentRatingFpbRatingReasonsEnum::FpbBlasphemy),
           "fpbLanguage" => Ok(ContentRatingFpbRatingReasonsEnum::FpbLanguage),
           "fpbNudity" => Ok(ContentRatingFpbRatingReasonsEnum::FpbNudity),
           "fpbPrejudice" => Ok(ContentRatingFpbRatingReasonsEnum::FpbPrejudice),
           "fpbSex" => Ok(ContentRatingFpbRatingReasonsEnum::FpbSex),
           "fpbViolence" => Ok(ContentRatingFpbRatingReasonsEnum::FpbViolence),
           "fpbDrugs" => Ok(ContentRatingFpbRatingReasonsEnum::FpbDrugs),
           "fpbSexualViolence" => Ok(ContentRatingFpbRatingReasonsEnum::FpbSexualViolence),
           "fpbHorror" => Ok(ContentRatingFpbRatingReasonsEnum::FpbHorror),
           "fpbCriminalTechniques" => Ok(ContentRatingFpbRatingReasonsEnum::FpbCriminalTechniques),
           "fpbImitativeActsTechniques" => Ok(ContentRatingFpbRatingReasonsEnum::FpbImitativeActsTechniques),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFpbRatingReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingFskRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating.
pub enum ContentRatingFskRatingEnum {
    
    /// "fskUnspecified"
    #[serde(rename="fskUnspecified")]
    FskUnspecified,
    

    /// FSK 0
    ///
    /// "fsk0"
    #[serde(rename="fsk0")]
    Fsk0,
    

    /// FSK 6
    ///
    /// "fsk6"
    #[serde(rename="fsk6")]
    Fsk6,
    

    /// FSK 12
    ///
    /// "fsk12"
    #[serde(rename="fsk12")]
    Fsk12,
    

    /// FSK 16
    ///
    /// "fsk16"
    #[serde(rename="fsk16")]
    Fsk16,
    

    /// FSK 18
    ///
    /// "fsk18"
    #[serde(rename="fsk18")]
    Fsk18,
    
    /// "fskUnrated"
    #[serde(rename="fskUnrated")]
    FskUnrated,
}

impl AsRef<str> for ContentRatingFskRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFskRatingEnum::FskUnspecified => "fskUnspecified",
            ContentRatingFskRatingEnum::Fsk0 => "fsk0",
            ContentRatingFskRatingEnum::Fsk6 => "fsk6",
            ContentRatingFskRatingEnum::Fsk12 => "fsk12",
            ContentRatingFskRatingEnum::Fsk16 => "fsk16",
            ContentRatingFskRatingEnum::Fsk18 => "fsk18",
            ContentRatingFskRatingEnum::FskUnrated => "fskUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingFskRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fskUnspecified" => Ok(ContentRatingFskRatingEnum::FskUnspecified),
           "fsk0" => Ok(ContentRatingFskRatingEnum::Fsk0),
           "fsk6" => Ok(ContentRatingFskRatingEnum::Fsk6),
           "fsk12" => Ok(ContentRatingFskRatingEnum::Fsk12),
           "fsk16" => Ok(ContentRatingFskRatingEnum::Fsk16),
           "fsk18" => Ok(ContentRatingFskRatingEnum::Fsk18),
           "fskUnrated" => Ok(ContentRatingFskRatingEnum::FskUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFskRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingGrfilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Greece.
pub enum ContentRatingGrfilmRatingEnum {
    
    /// "grfilmUnspecified"
    #[serde(rename="grfilmUnspecified")]
    GrfilmUnspecified,
    

    /// K
    ///
    /// "grfilmK"
    #[serde(rename="grfilmK")]
    GrfilmK,
    

    /// E
    ///
    /// "grfilmE"
    #[serde(rename="grfilmE")]
    GrfilmE,
    

    /// K-12
    ///
    /// "grfilmK12"
    #[serde(rename="grfilmK12")]
    GrfilmK12,
    

    /// K-13
    ///
    /// "grfilmK13"
    #[serde(rename="grfilmK13")]
    GrfilmK13,
    

    /// K-15
    ///
    /// "grfilmK15"
    #[serde(rename="grfilmK15")]
    GrfilmK15,
    

    /// K-17
    ///
    /// "grfilmK17"
    #[serde(rename="grfilmK17")]
    GrfilmK17,
    

    /// K-18
    ///
    /// "grfilmK18"
    #[serde(rename="grfilmK18")]
    GrfilmK18,
    
    /// "grfilmUnrated"
    #[serde(rename="grfilmUnrated")]
    GrfilmUnrated,
}

impl AsRef<str> for ContentRatingGrfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingGrfilmRatingEnum::GrfilmUnspecified => "grfilmUnspecified",
            ContentRatingGrfilmRatingEnum::GrfilmK => "grfilmK",
            ContentRatingGrfilmRatingEnum::GrfilmE => "grfilmE",
            ContentRatingGrfilmRatingEnum::GrfilmK12 => "grfilmK12",
            ContentRatingGrfilmRatingEnum::GrfilmK13 => "grfilmK13",
            ContentRatingGrfilmRatingEnum::GrfilmK15 => "grfilmK15",
            ContentRatingGrfilmRatingEnum::GrfilmK17 => "grfilmK17",
            ContentRatingGrfilmRatingEnum::GrfilmK18 => "grfilmK18",
            ContentRatingGrfilmRatingEnum::GrfilmUnrated => "grfilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingGrfilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "grfilmUnspecified" => Ok(ContentRatingGrfilmRatingEnum::GrfilmUnspecified),
           "grfilmK" => Ok(ContentRatingGrfilmRatingEnum::GrfilmK),
           "grfilmE" => Ok(ContentRatingGrfilmRatingEnum::GrfilmE),
           "grfilmK12" => Ok(ContentRatingGrfilmRatingEnum::GrfilmK12),
           "grfilmK13" => Ok(ContentRatingGrfilmRatingEnum::GrfilmK13),
           "grfilmK15" => Ok(ContentRatingGrfilmRatingEnum::GrfilmK15),
           "grfilmK17" => Ok(ContentRatingGrfilmRatingEnum::GrfilmK17),
           "grfilmK18" => Ok(ContentRatingGrfilmRatingEnum::GrfilmK18),
           "grfilmUnrated" => Ok(ContentRatingGrfilmRatingEnum::GrfilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingGrfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingIcaaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating.
pub enum ContentRatingIcaaRatingEnum {
    
    /// "icaaUnspecified"
    #[serde(rename="icaaUnspecified")]
    IcaaUnspecified,
    

    /// APTA
    ///
    /// "icaaApta"
    #[serde(rename="icaaApta")]
    IcaaApta,
    

    /// 7
    ///
    /// "icaa7"
    #[serde(rename="icaa7")]
    Icaa7,
    

    /// 12
    ///
    /// "icaa12"
    #[serde(rename="icaa12")]
    Icaa12,
    

    /// 13
    ///
    /// "icaa13"
    #[serde(rename="icaa13")]
    Icaa13,
    

    /// 16
    ///
    /// "icaa16"
    #[serde(rename="icaa16")]
    Icaa16,
    

    /// 18
    ///
    /// "icaa18"
    #[serde(rename="icaa18")]
    Icaa18,
    

    /// X
    ///
    /// "icaaX"
    #[serde(rename="icaaX")]
    IcaaX,
    
    /// "icaaUnrated"
    #[serde(rename="icaaUnrated")]
    IcaaUnrated,
}

impl AsRef<str> for ContentRatingIcaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIcaaRatingEnum::IcaaUnspecified => "icaaUnspecified",
            ContentRatingIcaaRatingEnum::IcaaApta => "icaaApta",
            ContentRatingIcaaRatingEnum::Icaa7 => "icaa7",
            ContentRatingIcaaRatingEnum::Icaa12 => "icaa12",
            ContentRatingIcaaRatingEnum::Icaa13 => "icaa13",
            ContentRatingIcaaRatingEnum::Icaa16 => "icaa16",
            ContentRatingIcaaRatingEnum::Icaa18 => "icaa18",
            ContentRatingIcaaRatingEnum::IcaaX => "icaaX",
            ContentRatingIcaaRatingEnum::IcaaUnrated => "icaaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingIcaaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "icaaUnspecified" => Ok(ContentRatingIcaaRatingEnum::IcaaUnspecified),
           "icaaApta" => Ok(ContentRatingIcaaRatingEnum::IcaaApta),
           "icaa7" => Ok(ContentRatingIcaaRatingEnum::Icaa7),
           "icaa12" => Ok(ContentRatingIcaaRatingEnum::Icaa12),
           "icaa13" => Ok(ContentRatingIcaaRatingEnum::Icaa13),
           "icaa16" => Ok(ContentRatingIcaaRatingEnum::Icaa16),
           "icaa18" => Ok(ContentRatingIcaaRatingEnum::Icaa18),
           "icaaX" => Ok(ContentRatingIcaaRatingEnum::IcaaX),
           "icaaUnrated" => Ok(ContentRatingIcaaRatingEnum::IcaaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIcaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingIfcoRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information.
pub enum ContentRatingIfcoRatingEnum {
    
    /// "ifcoUnspecified"
    #[serde(rename="ifcoUnspecified")]
    IfcoUnspecified,
    

    /// G
    ///
    /// "ifcoG"
    #[serde(rename="ifcoG")]
    IfcoG,
    

    /// PG
    ///
    /// "ifcoPg"
    #[serde(rename="ifcoPg")]
    IfcoPg,
    

    /// 12
    ///
    /// "ifco12"
    #[serde(rename="ifco12")]
    Ifco12,
    

    /// 12A
    ///
    /// "ifco12a"
    #[serde(rename="ifco12a")]
    Ifco12a,
    

    /// 15
    ///
    /// "ifco15"
    #[serde(rename="ifco15")]
    Ifco15,
    

    /// 15A
    ///
    /// "ifco15a"
    #[serde(rename="ifco15a")]
    Ifco15a,
    

    /// 16
    ///
    /// "ifco16"
    #[serde(rename="ifco16")]
    Ifco16,
    

    /// 18
    ///
    /// "ifco18"
    #[serde(rename="ifco18")]
    Ifco18,
    
    /// "ifcoUnrated"
    #[serde(rename="ifcoUnrated")]
    IfcoUnrated,
}

impl AsRef<str> for ContentRatingIfcoRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIfcoRatingEnum::IfcoUnspecified => "ifcoUnspecified",
            ContentRatingIfcoRatingEnum::IfcoG => "ifcoG",
            ContentRatingIfcoRatingEnum::IfcoPg => "ifcoPg",
            ContentRatingIfcoRatingEnum::Ifco12 => "ifco12",
            ContentRatingIfcoRatingEnum::Ifco12a => "ifco12a",
            ContentRatingIfcoRatingEnum::Ifco15 => "ifco15",
            ContentRatingIfcoRatingEnum::Ifco15a => "ifco15a",
            ContentRatingIfcoRatingEnum::Ifco16 => "ifco16",
            ContentRatingIfcoRatingEnum::Ifco18 => "ifco18",
            ContentRatingIfcoRatingEnum::IfcoUnrated => "ifcoUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingIfcoRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ifcoUnspecified" => Ok(ContentRatingIfcoRatingEnum::IfcoUnspecified),
           "ifcoG" => Ok(ContentRatingIfcoRatingEnum::IfcoG),
           "ifcoPg" => Ok(ContentRatingIfcoRatingEnum::IfcoPg),
           "ifco12" => Ok(ContentRatingIfcoRatingEnum::Ifco12),
           "ifco12a" => Ok(ContentRatingIfcoRatingEnum::Ifco12a),
           "ifco15" => Ok(ContentRatingIfcoRatingEnum::Ifco15),
           "ifco15a" => Ok(ContentRatingIfcoRatingEnum::Ifco15a),
           "ifco16" => Ok(ContentRatingIfcoRatingEnum::Ifco16),
           "ifco18" => Ok(ContentRatingIfcoRatingEnum::Ifco18),
           "ifcoUnrated" => Ok(ContentRatingIfcoRatingEnum::IfcoUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIfcoRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingIlfilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Israel.
pub enum ContentRatingIlfilmRatingEnum {
    
    /// "ilfilmUnspecified"
    #[serde(rename="ilfilmUnspecified")]
    IlfilmUnspecified,
    

    /// AA
    ///
    /// "ilfilmAa"
    #[serde(rename="ilfilmAa")]
    IlfilmAa,
    

    /// 12
    ///
    /// "ilfilm12"
    #[serde(rename="ilfilm12")]
    Ilfilm12,
    

    /// 14
    ///
    /// "ilfilm14"
    #[serde(rename="ilfilm14")]
    Ilfilm14,
    

    /// 16
    ///
    /// "ilfilm16"
    #[serde(rename="ilfilm16")]
    Ilfilm16,
    

    /// 18
    ///
    /// "ilfilm18"
    #[serde(rename="ilfilm18")]
    Ilfilm18,
    
    /// "ilfilmUnrated"
    #[serde(rename="ilfilmUnrated")]
    IlfilmUnrated,
}

impl AsRef<str> for ContentRatingIlfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIlfilmRatingEnum::IlfilmUnspecified => "ilfilmUnspecified",
            ContentRatingIlfilmRatingEnum::IlfilmAa => "ilfilmAa",
            ContentRatingIlfilmRatingEnum::Ilfilm12 => "ilfilm12",
            ContentRatingIlfilmRatingEnum::Ilfilm14 => "ilfilm14",
            ContentRatingIlfilmRatingEnum::Ilfilm16 => "ilfilm16",
            ContentRatingIlfilmRatingEnum::Ilfilm18 => "ilfilm18",
            ContentRatingIlfilmRatingEnum::IlfilmUnrated => "ilfilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingIlfilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ilfilmUnspecified" => Ok(ContentRatingIlfilmRatingEnum::IlfilmUnspecified),
           "ilfilmAa" => Ok(ContentRatingIlfilmRatingEnum::IlfilmAa),
           "ilfilm12" => Ok(ContentRatingIlfilmRatingEnum::Ilfilm12),
           "ilfilm14" => Ok(ContentRatingIlfilmRatingEnum::Ilfilm14),
           "ilfilm16" => Ok(ContentRatingIlfilmRatingEnum::Ilfilm16),
           "ilfilm18" => Ok(ContentRatingIlfilmRatingEnum::Ilfilm18),
           "ilfilmUnrated" => Ok(ContentRatingIlfilmRatingEnum::IlfilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIlfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingIncaaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating.
pub enum ContentRatingIncaaRatingEnum {
    
    /// "incaaUnspecified"
    #[serde(rename="incaaUnspecified")]
    IncaaUnspecified,
    

    /// ATP (Apta para todo publico)
    ///
    /// "incaaAtp"
    #[serde(rename="incaaAtp")]
    IncaaAtp,
    

    /// 13 (Solo apta para mayores de 13 años)
    ///
    /// "incaaSam13"
    #[serde(rename="incaaSam13")]
    IncaaSam13,
    

    /// 16 (Solo apta para mayores de 16 años)
    ///
    /// "incaaSam16"
    #[serde(rename="incaaSam16")]
    IncaaSam16,
    

    /// 18 (Solo apta para mayores de 18 años)
    ///
    /// "incaaSam18"
    #[serde(rename="incaaSam18")]
    IncaaSam18,
    

    /// X (Solo apta para mayores de 18 años, de exhibición condicionada)
    ///
    /// "incaaC"
    #[serde(rename="incaaC")]
    IncaaC,
    
    /// "incaaUnrated"
    #[serde(rename="incaaUnrated")]
    IncaaUnrated,
}

impl AsRef<str> for ContentRatingIncaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIncaaRatingEnum::IncaaUnspecified => "incaaUnspecified",
            ContentRatingIncaaRatingEnum::IncaaAtp => "incaaAtp",
            ContentRatingIncaaRatingEnum::IncaaSam13 => "incaaSam13",
            ContentRatingIncaaRatingEnum::IncaaSam16 => "incaaSam16",
            ContentRatingIncaaRatingEnum::IncaaSam18 => "incaaSam18",
            ContentRatingIncaaRatingEnum::IncaaC => "incaaC",
            ContentRatingIncaaRatingEnum::IncaaUnrated => "incaaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingIncaaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "incaaUnspecified" => Ok(ContentRatingIncaaRatingEnum::IncaaUnspecified),
           "incaaAtp" => Ok(ContentRatingIncaaRatingEnum::IncaaAtp),
           "incaaSam13" => Ok(ContentRatingIncaaRatingEnum::IncaaSam13),
           "incaaSam16" => Ok(ContentRatingIncaaRatingEnum::IncaaSam16),
           "incaaSam18" => Ok(ContentRatingIncaaRatingEnum::IncaaSam18),
           "incaaC" => Ok(ContentRatingIncaaRatingEnum::IncaaC),
           "incaaUnrated" => Ok(ContentRatingIncaaRatingEnum::IncaaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIncaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingKfcbRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Kenya Film Classification Board.
pub enum ContentRatingKfcbRatingEnum {
    
    /// "kfcbUnspecified"
    #[serde(rename="kfcbUnspecified")]
    KfcbUnspecified,
    

    /// GE
    ///
    /// "kfcbG"
    #[serde(rename="kfcbG")]
    KfcbG,
    

    /// PG
    ///
    /// "kfcbPg"
    #[serde(rename="kfcbPg")]
    KfcbPg,
    

    /// 16
    ///
    /// "kfcb16plus"
    #[serde(rename="kfcb16plus")]
    Kfcb16plus,
    

    /// 18
    ///
    /// "kfcbR"
    #[serde(rename="kfcbR")]
    KfcbR,
    
    /// "kfcbUnrated"
    #[serde(rename="kfcbUnrated")]
    KfcbUnrated,
}

impl AsRef<str> for ContentRatingKfcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingKfcbRatingEnum::KfcbUnspecified => "kfcbUnspecified",
            ContentRatingKfcbRatingEnum::KfcbG => "kfcbG",
            ContentRatingKfcbRatingEnum::KfcbPg => "kfcbPg",
            ContentRatingKfcbRatingEnum::Kfcb16plus => "kfcb16plus",
            ContentRatingKfcbRatingEnum::KfcbR => "kfcbR",
            ContentRatingKfcbRatingEnum::KfcbUnrated => "kfcbUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingKfcbRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "kfcbUnspecified" => Ok(ContentRatingKfcbRatingEnum::KfcbUnspecified),
           "kfcbG" => Ok(ContentRatingKfcbRatingEnum::KfcbG),
           "kfcbPg" => Ok(ContentRatingKfcbRatingEnum::KfcbPg),
           "kfcb16plus" => Ok(ContentRatingKfcbRatingEnum::Kfcb16plus),
           "kfcbR" => Ok(ContentRatingKfcbRatingEnum::KfcbR),
           "kfcbUnrated" => Ok(ContentRatingKfcbRatingEnum::KfcbUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingKfcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingKijkwijzerRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands).
pub enum ContentRatingKijkwijzerRatingEnum {
    
    /// "kijkwijzerUnspecified"
    #[serde(rename="kijkwijzerUnspecified")]
    KijkwijzerUnspecified,
    

    /// AL
    ///
    /// "kijkwijzerAl"
    #[serde(rename="kijkwijzerAl")]
    KijkwijzerAl,
    

    /// 6
    ///
    /// "kijkwijzer6"
    #[serde(rename="kijkwijzer6")]
    Kijkwijzer6,
    

    /// 9
    ///
    /// "kijkwijzer9"
    #[serde(rename="kijkwijzer9")]
    Kijkwijzer9,
    

    /// 12
    ///
    /// "kijkwijzer12"
    #[serde(rename="kijkwijzer12")]
    Kijkwijzer12,
    

    /// 16
    ///
    /// "kijkwijzer16"
    #[serde(rename="kijkwijzer16")]
    Kijkwijzer16,
    
    /// "kijkwijzer18"
    #[serde(rename="kijkwijzer18")]
    Kijkwijzer18,
    
    /// "kijkwijzerUnrated"
    #[serde(rename="kijkwijzerUnrated")]
    KijkwijzerUnrated,
}

impl AsRef<str> for ContentRatingKijkwijzerRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingKijkwijzerRatingEnum::KijkwijzerUnspecified => "kijkwijzerUnspecified",
            ContentRatingKijkwijzerRatingEnum::KijkwijzerAl => "kijkwijzerAl",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer6 => "kijkwijzer6",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer9 => "kijkwijzer9",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer12 => "kijkwijzer12",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer16 => "kijkwijzer16",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer18 => "kijkwijzer18",
            ContentRatingKijkwijzerRatingEnum::KijkwijzerUnrated => "kijkwijzerUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingKijkwijzerRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "kijkwijzerUnspecified" => Ok(ContentRatingKijkwijzerRatingEnum::KijkwijzerUnspecified),
           "kijkwijzerAl" => Ok(ContentRatingKijkwijzerRatingEnum::KijkwijzerAl),
           "kijkwijzer6" => Ok(ContentRatingKijkwijzerRatingEnum::Kijkwijzer6),
           "kijkwijzer9" => Ok(ContentRatingKijkwijzerRatingEnum::Kijkwijzer9),
           "kijkwijzer12" => Ok(ContentRatingKijkwijzerRatingEnum::Kijkwijzer12),
           "kijkwijzer16" => Ok(ContentRatingKijkwijzerRatingEnum::Kijkwijzer16),
           "kijkwijzer18" => Ok(ContentRatingKijkwijzerRatingEnum::Kijkwijzer18),
           "kijkwijzerUnrated" => Ok(ContentRatingKijkwijzerRatingEnum::KijkwijzerUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingKijkwijzerRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingKmrbRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea.
pub enum ContentRatingKmrbRatingEnum {
    
    /// "kmrbUnspecified"
    #[serde(rename="kmrbUnspecified")]
    KmrbUnspecified,
    

    /// 전체관람가
    ///
    /// "kmrbAll"
    #[serde(rename="kmrbAll")]
    KmrbAll,
    

    /// 12세 이상 관람가
    ///
    /// "kmrb12plus"
    #[serde(rename="kmrb12plus")]
    Kmrb12plus,
    

    /// 15세 이상 관람가
    ///
    /// "kmrb15plus"
    #[serde(rename="kmrb15plus")]
    Kmrb15plus,
    
    /// "kmrbTeenr"
    #[serde(rename="kmrbTeenr")]
    KmrbTeenr,
    

    /// 청소년 관람불가
    ///
    /// "kmrbR"
    #[serde(rename="kmrbR")]
    KmrbR,
    
    /// "kmrbUnrated"
    #[serde(rename="kmrbUnrated")]
    KmrbUnrated,
}

impl AsRef<str> for ContentRatingKmrbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingKmrbRatingEnum::KmrbUnspecified => "kmrbUnspecified",
            ContentRatingKmrbRatingEnum::KmrbAll => "kmrbAll",
            ContentRatingKmrbRatingEnum::Kmrb12plus => "kmrb12plus",
            ContentRatingKmrbRatingEnum::Kmrb15plus => "kmrb15plus",
            ContentRatingKmrbRatingEnum::KmrbTeenr => "kmrbTeenr",
            ContentRatingKmrbRatingEnum::KmrbR => "kmrbR",
            ContentRatingKmrbRatingEnum::KmrbUnrated => "kmrbUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingKmrbRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "kmrbUnspecified" => Ok(ContentRatingKmrbRatingEnum::KmrbUnspecified),
           "kmrbAll" => Ok(ContentRatingKmrbRatingEnum::KmrbAll),
           "kmrb12plus" => Ok(ContentRatingKmrbRatingEnum::Kmrb12plus),
           "kmrb15plus" => Ok(ContentRatingKmrbRatingEnum::Kmrb15plus),
           "kmrbTeenr" => Ok(ContentRatingKmrbRatingEnum::KmrbTeenr),
           "kmrbR" => Ok(ContentRatingKmrbRatingEnum::KmrbR),
           "kmrbUnrated" => Ok(ContentRatingKmrbRatingEnum::KmrbUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingKmrbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingLsfRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Indonesia's Lembaga Sensor Film.
pub enum ContentRatingLsfRatingEnum {
    
    /// "lsfUnspecified"
    #[serde(rename="lsfUnspecified")]
    LsfUnspecified,
    

    /// SU
    ///
    /// "lsfSu"
    #[serde(rename="lsfSu")]
    LsfSu,
    

    /// A
    ///
    /// "lsfA"
    #[serde(rename="lsfA")]
    LsfA,
    

    /// BO
    ///
    /// "lsfBo"
    #[serde(rename="lsfBo")]
    LsfBo,
    

    /// 13
    ///
    /// "lsf13"
    #[serde(rename="lsf13")]
    Lsf13,
    

    /// R
    ///
    /// "lsfR"
    #[serde(rename="lsfR")]
    LsfR,
    

    /// 17
    ///
    /// "lsf17"
    #[serde(rename="lsf17")]
    Lsf17,
    

    /// D
    ///
    /// "lsfD"
    #[serde(rename="lsfD")]
    LsfD,
    

    /// 21
    ///
    /// "lsf21"
    #[serde(rename="lsf21")]
    Lsf21,
    
    /// "lsfUnrated"
    #[serde(rename="lsfUnrated")]
    LsfUnrated,
}

impl AsRef<str> for ContentRatingLsfRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingLsfRatingEnum::LsfUnspecified => "lsfUnspecified",
            ContentRatingLsfRatingEnum::LsfSu => "lsfSu",
            ContentRatingLsfRatingEnum::LsfA => "lsfA",
            ContentRatingLsfRatingEnum::LsfBo => "lsfBo",
            ContentRatingLsfRatingEnum::Lsf13 => "lsf13",
            ContentRatingLsfRatingEnum::LsfR => "lsfR",
            ContentRatingLsfRatingEnum::Lsf17 => "lsf17",
            ContentRatingLsfRatingEnum::LsfD => "lsfD",
            ContentRatingLsfRatingEnum::Lsf21 => "lsf21",
            ContentRatingLsfRatingEnum::LsfUnrated => "lsfUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingLsfRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "lsfUnspecified" => Ok(ContentRatingLsfRatingEnum::LsfUnspecified),
           "lsfSu" => Ok(ContentRatingLsfRatingEnum::LsfSu),
           "lsfA" => Ok(ContentRatingLsfRatingEnum::LsfA),
           "lsfBo" => Ok(ContentRatingLsfRatingEnum::LsfBo),
           "lsf13" => Ok(ContentRatingLsfRatingEnum::Lsf13),
           "lsfR" => Ok(ContentRatingLsfRatingEnum::LsfR),
           "lsf17" => Ok(ContentRatingLsfRatingEnum::Lsf17),
           "lsfD" => Ok(ContentRatingLsfRatingEnum::LsfD),
           "lsf21" => Ok(ContentRatingLsfRatingEnum::Lsf21),
           "lsfUnrated" => Ok(ContentRatingLsfRatingEnum::LsfUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingLsfRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMccaaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Malta's Film Age-Classification Board.
pub enum ContentRatingMccaaRatingEnum {
    
    /// "mccaaUnspecified"
    #[serde(rename="mccaaUnspecified")]
    MccaaUnspecified,
    

    /// U
    ///
    /// "mccaaU"
    #[serde(rename="mccaaU")]
    MccaaU,
    

    /// PG
    ///
    /// "mccaaPg"
    #[serde(rename="mccaaPg")]
    MccaaPg,
    

    /// 12A
    ///
    /// "mccaa12a"
    #[serde(rename="mccaa12a")]
    Mccaa12a,
    

    /// 12
    ///
    /// "mccaa12"
    #[serde(rename="mccaa12")]
    Mccaa12,
    

    /// 14 - this rating was removed from the new classification structure introduced in 2013.
    ///
    /// "mccaa14"
    #[serde(rename="mccaa14")]
    Mccaa14,
    

    /// 15
    ///
    /// "mccaa15"
    #[serde(rename="mccaa15")]
    Mccaa15,
    

    /// 16 - this rating was removed from the new classification structure introduced in 2013.
    ///
    /// "mccaa16"
    #[serde(rename="mccaa16")]
    Mccaa16,
    

    /// 18
    ///
    /// "mccaa18"
    #[serde(rename="mccaa18")]
    Mccaa18,
    
    /// "mccaaUnrated"
    #[serde(rename="mccaaUnrated")]
    MccaaUnrated,
}

impl AsRef<str> for ContentRatingMccaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMccaaRatingEnum::MccaaUnspecified => "mccaaUnspecified",
            ContentRatingMccaaRatingEnum::MccaaU => "mccaaU",
            ContentRatingMccaaRatingEnum::MccaaPg => "mccaaPg",
            ContentRatingMccaaRatingEnum::Mccaa12a => "mccaa12a",
            ContentRatingMccaaRatingEnum::Mccaa12 => "mccaa12",
            ContentRatingMccaaRatingEnum::Mccaa14 => "mccaa14",
            ContentRatingMccaaRatingEnum::Mccaa15 => "mccaa15",
            ContentRatingMccaaRatingEnum::Mccaa16 => "mccaa16",
            ContentRatingMccaaRatingEnum::Mccaa18 => "mccaa18",
            ContentRatingMccaaRatingEnum::MccaaUnrated => "mccaaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMccaaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mccaaUnspecified" => Ok(ContentRatingMccaaRatingEnum::MccaaUnspecified),
           "mccaaU" => Ok(ContentRatingMccaaRatingEnum::MccaaU),
           "mccaaPg" => Ok(ContentRatingMccaaRatingEnum::MccaaPg),
           "mccaa12a" => Ok(ContentRatingMccaaRatingEnum::Mccaa12a),
           "mccaa12" => Ok(ContentRatingMccaaRatingEnum::Mccaa12),
           "mccaa14" => Ok(ContentRatingMccaaRatingEnum::Mccaa14),
           "mccaa15" => Ok(ContentRatingMccaaRatingEnum::Mccaa15),
           "mccaa16" => Ok(ContentRatingMccaaRatingEnum::Mccaa16),
           "mccaa18" => Ok(ContentRatingMccaaRatingEnum::Mccaa18),
           "mccaaUnrated" => Ok(ContentRatingMccaaRatingEnum::MccaaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMccaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMccypRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People.
pub enum ContentRatingMccypRatingEnum {
    
    /// "mccypUnspecified"
    #[serde(rename="mccypUnspecified")]
    MccypUnspecified,
    

    /// A
    ///
    /// "mccypA"
    #[serde(rename="mccypA")]
    MccypA,
    

    /// 7
    ///
    /// "mccyp7"
    #[serde(rename="mccyp7")]
    Mccyp7,
    

    /// 11
    ///
    /// "mccyp11"
    #[serde(rename="mccyp11")]
    Mccyp11,
    

    /// 15
    ///
    /// "mccyp15"
    #[serde(rename="mccyp15")]
    Mccyp15,
    
    /// "mccypUnrated"
    #[serde(rename="mccypUnrated")]
    MccypUnrated,
}

impl AsRef<str> for ContentRatingMccypRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMccypRatingEnum::MccypUnspecified => "mccypUnspecified",
            ContentRatingMccypRatingEnum::MccypA => "mccypA",
            ContentRatingMccypRatingEnum::Mccyp7 => "mccyp7",
            ContentRatingMccypRatingEnum::Mccyp11 => "mccyp11",
            ContentRatingMccypRatingEnum::Mccyp15 => "mccyp15",
            ContentRatingMccypRatingEnum::MccypUnrated => "mccypUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMccypRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mccypUnspecified" => Ok(ContentRatingMccypRatingEnum::MccypUnspecified),
           "mccypA" => Ok(ContentRatingMccypRatingEnum::MccypA),
           "mccyp7" => Ok(ContentRatingMccypRatingEnum::Mccyp7),
           "mccyp11" => Ok(ContentRatingMccypRatingEnum::Mccyp11),
           "mccyp15" => Ok(ContentRatingMccypRatingEnum::Mccyp15),
           "mccypUnrated" => Ok(ContentRatingMccypRatingEnum::MccypUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMccypRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMcstRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating system for Vietnam - MCST
pub enum ContentRatingMcstRatingEnum {
    
    /// "mcstUnspecified"
    #[serde(rename="mcstUnspecified")]
    McstUnspecified,
    

    /// P
    ///
    /// "mcstP"
    #[serde(rename="mcstP")]
    McstP,
    

    /// 0
    ///
    /// "mcst0"
    #[serde(rename="mcst0")]
    Mcst0,
    

    /// C13
    ///
    /// "mcstC13"
    #[serde(rename="mcstC13")]
    McstC13,
    

    /// C16
    ///
    /// "mcstC16"
    #[serde(rename="mcstC16")]
    McstC16,
    

    /// 16+
    ///
    /// "mcst16plus"
    #[serde(rename="mcst16plus")]
    Mcst16plus,
    

    /// C18
    ///
    /// "mcstC18"
    #[serde(rename="mcstC18")]
    McstC18,
    

    /// MCST_G_PG
    ///
    /// "mcstGPg"
    #[serde(rename="mcstGPg")]
    McstGPg,
    
    /// "mcstUnrated"
    #[serde(rename="mcstUnrated")]
    McstUnrated,
}

impl AsRef<str> for ContentRatingMcstRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMcstRatingEnum::McstUnspecified => "mcstUnspecified",
            ContentRatingMcstRatingEnum::McstP => "mcstP",
            ContentRatingMcstRatingEnum::Mcst0 => "mcst0",
            ContentRatingMcstRatingEnum::McstC13 => "mcstC13",
            ContentRatingMcstRatingEnum::McstC16 => "mcstC16",
            ContentRatingMcstRatingEnum::Mcst16plus => "mcst16plus",
            ContentRatingMcstRatingEnum::McstC18 => "mcstC18",
            ContentRatingMcstRatingEnum::McstGPg => "mcstGPg",
            ContentRatingMcstRatingEnum::McstUnrated => "mcstUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMcstRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mcstUnspecified" => Ok(ContentRatingMcstRatingEnum::McstUnspecified),
           "mcstP" => Ok(ContentRatingMcstRatingEnum::McstP),
           "mcst0" => Ok(ContentRatingMcstRatingEnum::Mcst0),
           "mcstC13" => Ok(ContentRatingMcstRatingEnum::McstC13),
           "mcstC16" => Ok(ContentRatingMcstRatingEnum::McstC16),
           "mcst16plus" => Ok(ContentRatingMcstRatingEnum::Mcst16plus),
           "mcstC18" => Ok(ContentRatingMcstRatingEnum::McstC18),
           "mcstGPg" => Ok(ContentRatingMcstRatingEnum::McstGPg),
           "mcstUnrated" => Ok(ContentRatingMcstRatingEnum::McstUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMcstRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMdaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC).
pub enum ContentRatingMdaRatingEnum {
    
    /// "mdaUnspecified"
    #[serde(rename="mdaUnspecified")]
    MdaUnspecified,
    

    /// G
    ///
    /// "mdaG"
    #[serde(rename="mdaG")]
    MdaG,
    

    /// PG
    ///
    /// "mdaPg"
    #[serde(rename="mdaPg")]
    MdaPg,
    

    /// PG13
    ///
    /// "mdaPg13"
    #[serde(rename="mdaPg13")]
    MdaPg13,
    

    /// NC16
    ///
    /// "mdaNc16"
    #[serde(rename="mdaNc16")]
    MdaNc16,
    

    /// M18
    ///
    /// "mdaM18"
    #[serde(rename="mdaM18")]
    MdaM18,
    

    /// R21
    ///
    /// "mdaR21"
    #[serde(rename="mdaR21")]
    MdaR21,
    
    /// "mdaUnrated"
    #[serde(rename="mdaUnrated")]
    MdaUnrated,
}

impl AsRef<str> for ContentRatingMdaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMdaRatingEnum::MdaUnspecified => "mdaUnspecified",
            ContentRatingMdaRatingEnum::MdaG => "mdaG",
            ContentRatingMdaRatingEnum::MdaPg => "mdaPg",
            ContentRatingMdaRatingEnum::MdaPg13 => "mdaPg13",
            ContentRatingMdaRatingEnum::MdaNc16 => "mdaNc16",
            ContentRatingMdaRatingEnum::MdaM18 => "mdaM18",
            ContentRatingMdaRatingEnum::MdaR21 => "mdaR21",
            ContentRatingMdaRatingEnum::MdaUnrated => "mdaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMdaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mdaUnspecified" => Ok(ContentRatingMdaRatingEnum::MdaUnspecified),
           "mdaG" => Ok(ContentRatingMdaRatingEnum::MdaG),
           "mdaPg" => Ok(ContentRatingMdaRatingEnum::MdaPg),
           "mdaPg13" => Ok(ContentRatingMdaRatingEnum::MdaPg13),
           "mdaNc16" => Ok(ContentRatingMdaRatingEnum::MdaNc16),
           "mdaM18" => Ok(ContentRatingMdaRatingEnum::MdaM18),
           "mdaR21" => Ok(ContentRatingMdaRatingEnum::MdaR21),
           "mdaUnrated" => Ok(ContentRatingMdaRatingEnum::MdaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMdaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMedietilsynetRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Medietilsynet, the Norwegian Media Authority.
pub enum ContentRatingMedietilsynetRatingEnum {
    
    /// "medietilsynetUnspecified"
    #[serde(rename="medietilsynetUnspecified")]
    MedietilsynetUnspecified,
    

    /// A
    ///
    /// "medietilsynetA"
    #[serde(rename="medietilsynetA")]
    MedietilsynetA,
    

    /// 6
    ///
    /// "medietilsynet6"
    #[serde(rename="medietilsynet6")]
    Medietilsynet6,
    

    /// 7
    ///
    /// "medietilsynet7"
    #[serde(rename="medietilsynet7")]
    Medietilsynet7,
    

    /// 9
    ///
    /// "medietilsynet9"
    #[serde(rename="medietilsynet9")]
    Medietilsynet9,
    

    /// 11
    ///
    /// "medietilsynet11"
    #[serde(rename="medietilsynet11")]
    Medietilsynet11,
    

    /// 12
    ///
    /// "medietilsynet12"
    #[serde(rename="medietilsynet12")]
    Medietilsynet12,
    

    /// 15
    ///
    /// "medietilsynet15"
    #[serde(rename="medietilsynet15")]
    Medietilsynet15,
    

    /// 18
    ///
    /// "medietilsynet18"
    #[serde(rename="medietilsynet18")]
    Medietilsynet18,
    
    /// "medietilsynetUnrated"
    #[serde(rename="medietilsynetUnrated")]
    MedietilsynetUnrated,
}

impl AsRef<str> for ContentRatingMedietilsynetRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMedietilsynetRatingEnum::MedietilsynetUnspecified => "medietilsynetUnspecified",
            ContentRatingMedietilsynetRatingEnum::MedietilsynetA => "medietilsynetA",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet6 => "medietilsynet6",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet7 => "medietilsynet7",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet9 => "medietilsynet9",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet11 => "medietilsynet11",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet12 => "medietilsynet12",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet15 => "medietilsynet15",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet18 => "medietilsynet18",
            ContentRatingMedietilsynetRatingEnum::MedietilsynetUnrated => "medietilsynetUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMedietilsynetRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "medietilsynetUnspecified" => Ok(ContentRatingMedietilsynetRatingEnum::MedietilsynetUnspecified),
           "medietilsynetA" => Ok(ContentRatingMedietilsynetRatingEnum::MedietilsynetA),
           "medietilsynet6" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet6),
           "medietilsynet7" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet7),
           "medietilsynet9" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet9),
           "medietilsynet11" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet11),
           "medietilsynet12" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet12),
           "medietilsynet15" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet15),
           "medietilsynet18" => Ok(ContentRatingMedietilsynetRatingEnum::Medietilsynet18),
           "medietilsynetUnrated" => Ok(ContentRatingMedietilsynetRatingEnum::MedietilsynetUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMedietilsynetRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMekuRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute).
pub enum ContentRatingMekuRatingEnum {
    
    /// "mekuUnspecified"
    #[serde(rename="mekuUnspecified")]
    MekuUnspecified,
    

    /// S
    ///
    /// "mekuS"
    #[serde(rename="mekuS")]
    MekuS,
    

    /// 7
    ///
    /// "meku7"
    #[serde(rename="meku7")]
    Meku7,
    

    /// 12
    ///
    /// "meku12"
    #[serde(rename="meku12")]
    Meku12,
    

    /// 16
    ///
    /// "meku16"
    #[serde(rename="meku16")]
    Meku16,
    

    /// 18
    ///
    /// "meku18"
    #[serde(rename="meku18")]
    Meku18,
    
    /// "mekuUnrated"
    #[serde(rename="mekuUnrated")]
    MekuUnrated,
}

impl AsRef<str> for ContentRatingMekuRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMekuRatingEnum::MekuUnspecified => "mekuUnspecified",
            ContentRatingMekuRatingEnum::MekuS => "mekuS",
            ContentRatingMekuRatingEnum::Meku7 => "meku7",
            ContentRatingMekuRatingEnum::Meku12 => "meku12",
            ContentRatingMekuRatingEnum::Meku16 => "meku16",
            ContentRatingMekuRatingEnum::Meku18 => "meku18",
            ContentRatingMekuRatingEnum::MekuUnrated => "mekuUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMekuRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mekuUnspecified" => Ok(ContentRatingMekuRatingEnum::MekuUnspecified),
           "mekuS" => Ok(ContentRatingMekuRatingEnum::MekuS),
           "meku7" => Ok(ContentRatingMekuRatingEnum::Meku7),
           "meku12" => Ok(ContentRatingMekuRatingEnum::Meku12),
           "meku16" => Ok(ContentRatingMekuRatingEnum::Meku16),
           "meku18" => Ok(ContentRatingMekuRatingEnum::Meku18),
           "mekuUnrated" => Ok(ContentRatingMekuRatingEnum::MekuUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMekuRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMenaMpaaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rating system for MENA countries, a clone of MPAA. It is needed to prevent titles go live w/o additional QC check, since some of them can be inappropriate for the countries at all. See b/33408548 for more details.
pub enum ContentRatingMenaMpaaRatingEnum {
    
    /// "menaMpaaUnspecified"
    #[serde(rename="menaMpaaUnspecified")]
    MenaMpaaUnspecified,
    

    /// G
    ///
    /// "menaMpaaG"
    #[serde(rename="menaMpaaG")]
    MenaMpaaG,
    

    /// PG
    ///
    /// "menaMpaaPg"
    #[serde(rename="menaMpaaPg")]
    MenaMpaaPg,
    

    /// PG-13
    ///
    /// "menaMpaaPg13"
    #[serde(rename="menaMpaaPg13")]
    MenaMpaaPg13,
    

    /// R
    ///
    /// "menaMpaaR"
    #[serde(rename="menaMpaaR")]
    MenaMpaaR,
    

    /// To keep the same enum values as MPAA's items have, skip NC_17.
    ///
    /// "menaMpaaUnrated"
    #[serde(rename="menaMpaaUnrated")]
    MenaMpaaUnrated,
}

impl AsRef<str> for ContentRatingMenaMpaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMenaMpaaRatingEnum::MenaMpaaUnspecified => "menaMpaaUnspecified",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaG => "menaMpaaG",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaPg => "menaMpaaPg",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaPg13 => "menaMpaaPg13",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaR => "menaMpaaR",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaUnrated => "menaMpaaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMenaMpaaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "menaMpaaUnspecified" => Ok(ContentRatingMenaMpaaRatingEnum::MenaMpaaUnspecified),
           "menaMpaaG" => Ok(ContentRatingMenaMpaaRatingEnum::MenaMpaaG),
           "menaMpaaPg" => Ok(ContentRatingMenaMpaaRatingEnum::MenaMpaaPg),
           "menaMpaaPg13" => Ok(ContentRatingMenaMpaaRatingEnum::MenaMpaaPg13),
           "menaMpaaR" => Ok(ContentRatingMenaMpaaRatingEnum::MenaMpaaR),
           "menaMpaaUnrated" => Ok(ContentRatingMenaMpaaRatingEnum::MenaMpaaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMenaMpaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMibacRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy).
pub enum ContentRatingMibacRatingEnum {
    
    /// "mibacUnspecified"
    #[serde(rename="mibacUnspecified")]
    MibacUnspecified,
    
    /// "mibacT"
    #[serde(rename="mibacT")]
    MibacT,
    
    /// "mibacVap"
    #[serde(rename="mibacVap")]
    MibacVap,
    
    /// "mibacVm6"
    #[serde(rename="mibacVm6")]
    MibacVm6,
    
    /// "mibacVm12"
    #[serde(rename="mibacVm12")]
    MibacVm12,
    
    /// "mibacVm14"
    #[serde(rename="mibacVm14")]
    MibacVm14,
    
    /// "mibacVm16"
    #[serde(rename="mibacVm16")]
    MibacVm16,
    
    /// "mibacVm18"
    #[serde(rename="mibacVm18")]
    MibacVm18,
    
    /// "mibacUnrated"
    #[serde(rename="mibacUnrated")]
    MibacUnrated,
}

impl AsRef<str> for ContentRatingMibacRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMibacRatingEnum::MibacUnspecified => "mibacUnspecified",
            ContentRatingMibacRatingEnum::MibacT => "mibacT",
            ContentRatingMibacRatingEnum::MibacVap => "mibacVap",
            ContentRatingMibacRatingEnum::MibacVm6 => "mibacVm6",
            ContentRatingMibacRatingEnum::MibacVm12 => "mibacVm12",
            ContentRatingMibacRatingEnum::MibacVm14 => "mibacVm14",
            ContentRatingMibacRatingEnum::MibacVm16 => "mibacVm16",
            ContentRatingMibacRatingEnum::MibacVm18 => "mibacVm18",
            ContentRatingMibacRatingEnum::MibacUnrated => "mibacUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMibacRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mibacUnspecified" => Ok(ContentRatingMibacRatingEnum::MibacUnspecified),
           "mibacT" => Ok(ContentRatingMibacRatingEnum::MibacT),
           "mibacVap" => Ok(ContentRatingMibacRatingEnum::MibacVap),
           "mibacVm6" => Ok(ContentRatingMibacRatingEnum::MibacVm6),
           "mibacVm12" => Ok(ContentRatingMibacRatingEnum::MibacVm12),
           "mibacVm14" => Ok(ContentRatingMibacRatingEnum::MibacVm14),
           "mibacVm16" => Ok(ContentRatingMibacRatingEnum::MibacVm16),
           "mibacVm18" => Ok(ContentRatingMibacRatingEnum::MibacVm18),
           "mibacUnrated" => Ok(ContentRatingMibacRatingEnum::MibacUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMibacRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMocRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Ministerio de Cultura (Colombia) rating.
pub enum ContentRatingMocRatingEnum {
    
    /// "mocUnspecified"
    #[serde(rename="mocUnspecified")]
    MocUnspecified,
    

    /// E
    ///
    /// "mocE"
    #[serde(rename="mocE")]
    MocE,
    

    /// T
    ///
    /// "mocT"
    #[serde(rename="mocT")]
    MocT,
    

    /// 7
    ///
    /// "moc7"
    #[serde(rename="moc7")]
    Moc7,
    

    /// 12
    ///
    /// "moc12"
    #[serde(rename="moc12")]
    Moc12,
    

    /// 15
    ///
    /// "moc15"
    #[serde(rename="moc15")]
    Moc15,
    

    /// 18
    ///
    /// "moc18"
    #[serde(rename="moc18")]
    Moc18,
    

    /// X
    ///
    /// "mocX"
    #[serde(rename="mocX")]
    MocX,
    

    /// Banned
    ///
    /// "mocBanned"
    #[serde(rename="mocBanned")]
    MocBanned,
    
    /// "mocUnrated"
    #[serde(rename="mocUnrated")]
    MocUnrated,
}

impl AsRef<str> for ContentRatingMocRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMocRatingEnum::MocUnspecified => "mocUnspecified",
            ContentRatingMocRatingEnum::MocE => "mocE",
            ContentRatingMocRatingEnum::MocT => "mocT",
            ContentRatingMocRatingEnum::Moc7 => "moc7",
            ContentRatingMocRatingEnum::Moc12 => "moc12",
            ContentRatingMocRatingEnum::Moc15 => "moc15",
            ContentRatingMocRatingEnum::Moc18 => "moc18",
            ContentRatingMocRatingEnum::MocX => "mocX",
            ContentRatingMocRatingEnum::MocBanned => "mocBanned",
            ContentRatingMocRatingEnum::MocUnrated => "mocUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMocRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mocUnspecified" => Ok(ContentRatingMocRatingEnum::MocUnspecified),
           "mocE" => Ok(ContentRatingMocRatingEnum::MocE),
           "mocT" => Ok(ContentRatingMocRatingEnum::MocT),
           "moc7" => Ok(ContentRatingMocRatingEnum::Moc7),
           "moc12" => Ok(ContentRatingMocRatingEnum::Moc12),
           "moc15" => Ok(ContentRatingMocRatingEnum::Moc15),
           "moc18" => Ok(ContentRatingMocRatingEnum::Moc18),
           "mocX" => Ok(ContentRatingMocRatingEnum::MocX),
           "mocBanned" => Ok(ContentRatingMocRatingEnum::MocBanned),
           "mocUnrated" => Ok(ContentRatingMocRatingEnum::MocUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMocRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMoctwRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Taiwan's Ministry of Culture (文化部).
pub enum ContentRatingMoctwRatingEnum {
    
    /// "moctwUnspecified"
    #[serde(rename="moctwUnspecified")]
    MoctwUnspecified,
    

    /// G
    ///
    /// "moctwG"
    #[serde(rename="moctwG")]
    MoctwG,
    

    /// P
    ///
    /// "moctwP"
    #[serde(rename="moctwP")]
    MoctwP,
    

    /// PG
    ///
    /// "moctwPg"
    #[serde(rename="moctwPg")]
    MoctwPg,
    

    /// R
    ///
    /// "moctwR"
    #[serde(rename="moctwR")]
    MoctwR,
    
    /// "moctwUnrated"
    #[serde(rename="moctwUnrated")]
    MoctwUnrated,
    

    /// R-12
    ///
    /// "moctwR12"
    #[serde(rename="moctwR12")]
    MoctwR12,
    

    /// R-15
    ///
    /// "moctwR15"
    #[serde(rename="moctwR15")]
    MoctwR15,
}

impl AsRef<str> for ContentRatingMoctwRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMoctwRatingEnum::MoctwUnspecified => "moctwUnspecified",
            ContentRatingMoctwRatingEnum::MoctwG => "moctwG",
            ContentRatingMoctwRatingEnum::MoctwP => "moctwP",
            ContentRatingMoctwRatingEnum::MoctwPg => "moctwPg",
            ContentRatingMoctwRatingEnum::MoctwR => "moctwR",
            ContentRatingMoctwRatingEnum::MoctwUnrated => "moctwUnrated",
            ContentRatingMoctwRatingEnum::MoctwR12 => "moctwR12",
            ContentRatingMoctwRatingEnum::MoctwR15 => "moctwR15",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMoctwRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "moctwUnspecified" => Ok(ContentRatingMoctwRatingEnum::MoctwUnspecified),
           "moctwG" => Ok(ContentRatingMoctwRatingEnum::MoctwG),
           "moctwP" => Ok(ContentRatingMoctwRatingEnum::MoctwP),
           "moctwPg" => Ok(ContentRatingMoctwRatingEnum::MoctwPg),
           "moctwR" => Ok(ContentRatingMoctwRatingEnum::MoctwR),
           "moctwUnrated" => Ok(ContentRatingMoctwRatingEnum::MoctwUnrated),
           "moctwR12" => Ok(ContentRatingMoctwRatingEnum::MoctwR12),
           "moctwR15" => Ok(ContentRatingMoctwRatingEnum::MoctwR15),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMoctwRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMpaaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Motion Picture Association of America (MPAA) rating.
pub enum ContentRatingMpaaRatingEnum {
    
    /// "mpaaUnspecified"
    #[serde(rename="mpaaUnspecified")]
    MpaaUnspecified,
    

    /// G
    ///
    /// "mpaaG"
    #[serde(rename="mpaaG")]
    MpaaG,
    

    /// PG
    ///
    /// "mpaaPg"
    #[serde(rename="mpaaPg")]
    MpaaPg,
    

    /// PG-13
    ///
    /// "mpaaPg13"
    #[serde(rename="mpaaPg13")]
    MpaaPg13,
    

    /// R
    ///
    /// "mpaaR"
    #[serde(rename="mpaaR")]
    MpaaR,
    

    /// NC-17
    ///
    /// "mpaaNc17"
    #[serde(rename="mpaaNc17")]
    MpaaNc17,
    

    /// ! X
    ///
    /// "mpaaX"
    #[serde(rename="mpaaX")]
    MpaaX,
    
    /// "mpaaUnrated"
    #[serde(rename="mpaaUnrated")]
    MpaaUnrated,
}

impl AsRef<str> for ContentRatingMpaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMpaaRatingEnum::MpaaUnspecified => "mpaaUnspecified",
            ContentRatingMpaaRatingEnum::MpaaG => "mpaaG",
            ContentRatingMpaaRatingEnum::MpaaPg => "mpaaPg",
            ContentRatingMpaaRatingEnum::MpaaPg13 => "mpaaPg13",
            ContentRatingMpaaRatingEnum::MpaaR => "mpaaR",
            ContentRatingMpaaRatingEnum::MpaaNc17 => "mpaaNc17",
            ContentRatingMpaaRatingEnum::MpaaX => "mpaaX",
            ContentRatingMpaaRatingEnum::MpaaUnrated => "mpaaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMpaaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mpaaUnspecified" => Ok(ContentRatingMpaaRatingEnum::MpaaUnspecified),
           "mpaaG" => Ok(ContentRatingMpaaRatingEnum::MpaaG),
           "mpaaPg" => Ok(ContentRatingMpaaRatingEnum::MpaaPg),
           "mpaaPg13" => Ok(ContentRatingMpaaRatingEnum::MpaaPg13),
           "mpaaR" => Ok(ContentRatingMpaaRatingEnum::MpaaR),
           "mpaaNc17" => Ok(ContentRatingMpaaRatingEnum::MpaaNc17),
           "mpaaX" => Ok(ContentRatingMpaaRatingEnum::MpaaX),
           "mpaaUnrated" => Ok(ContentRatingMpaaRatingEnum::MpaaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMpaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMpaatRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html.
pub enum ContentRatingMpaatRatingEnum {
    
    /// "mpaatUnspecified"
    #[serde(rename="mpaatUnspecified")]
    MpaatUnspecified,
    

    /// GB
    ///
    /// "mpaatGb"
    #[serde(rename="mpaatGb")]
    MpaatGb,
    

    /// RB
    ///
    /// "mpaatRb"
    #[serde(rename="mpaatRb")]
    MpaatRb,
}

impl AsRef<str> for ContentRatingMpaatRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMpaatRatingEnum::MpaatUnspecified => "mpaatUnspecified",
            ContentRatingMpaatRatingEnum::MpaatGb => "mpaatGb",
            ContentRatingMpaatRatingEnum::MpaatRb => "mpaatRb",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMpaatRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mpaatUnspecified" => Ok(ContentRatingMpaatRatingEnum::MpaatUnspecified),
           "mpaatGb" => Ok(ContentRatingMpaatRatingEnum::MpaatGb),
           "mpaatRb" => Ok(ContentRatingMpaatRatingEnum::MpaatRb),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMpaatRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingMtrcbRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Movie and Television Review and Classification Board (Philippines).
pub enum ContentRatingMtrcbRatingEnum {
    
    /// "mtrcbUnspecified"
    #[serde(rename="mtrcbUnspecified")]
    MtrcbUnspecified,
    

    /// G
    ///
    /// "mtrcbG"
    #[serde(rename="mtrcbG")]
    MtrcbG,
    

    /// PG
    ///
    /// "mtrcbPg"
    #[serde(rename="mtrcbPg")]
    MtrcbPg,
    

    /// R-13
    ///
    /// "mtrcbR13"
    #[serde(rename="mtrcbR13")]
    MtrcbR13,
    

    /// R-16
    ///
    /// "mtrcbR16"
    #[serde(rename="mtrcbR16")]
    MtrcbR16,
    

    /// R-18
    ///
    /// "mtrcbR18"
    #[serde(rename="mtrcbR18")]
    MtrcbR18,
    

    /// X
    ///
    /// "mtrcbX"
    #[serde(rename="mtrcbX")]
    MtrcbX,
    
    /// "mtrcbUnrated"
    #[serde(rename="mtrcbUnrated")]
    MtrcbUnrated,
}

impl AsRef<str> for ContentRatingMtrcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMtrcbRatingEnum::MtrcbUnspecified => "mtrcbUnspecified",
            ContentRatingMtrcbRatingEnum::MtrcbG => "mtrcbG",
            ContentRatingMtrcbRatingEnum::MtrcbPg => "mtrcbPg",
            ContentRatingMtrcbRatingEnum::MtrcbR13 => "mtrcbR13",
            ContentRatingMtrcbRatingEnum::MtrcbR16 => "mtrcbR16",
            ContentRatingMtrcbRatingEnum::MtrcbR18 => "mtrcbR18",
            ContentRatingMtrcbRatingEnum::MtrcbX => "mtrcbX",
            ContentRatingMtrcbRatingEnum::MtrcbUnrated => "mtrcbUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingMtrcbRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "mtrcbUnspecified" => Ok(ContentRatingMtrcbRatingEnum::MtrcbUnspecified),
           "mtrcbG" => Ok(ContentRatingMtrcbRatingEnum::MtrcbG),
           "mtrcbPg" => Ok(ContentRatingMtrcbRatingEnum::MtrcbPg),
           "mtrcbR13" => Ok(ContentRatingMtrcbRatingEnum::MtrcbR13),
           "mtrcbR16" => Ok(ContentRatingMtrcbRatingEnum::MtrcbR16),
           "mtrcbR18" => Ok(ContentRatingMtrcbRatingEnum::MtrcbR18),
           "mtrcbX" => Ok(ContentRatingMtrcbRatingEnum::MtrcbX),
           "mtrcbUnrated" => Ok(ContentRatingMtrcbRatingEnum::MtrcbUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMtrcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingNbcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Maldives National Bureau of Classification.
pub enum ContentRatingNbcRatingEnum {
    
    /// "nbcUnspecified"
    #[serde(rename="nbcUnspecified")]
    NbcUnspecified,
    

    /// G
    ///
    /// "nbcG"
    #[serde(rename="nbcG")]
    NbcG,
    

    /// PG
    ///
    /// "nbcPg"
    #[serde(rename="nbcPg")]
    NbcPg,
    

    /// 12+
    ///
    /// "nbc12plus"
    #[serde(rename="nbc12plus")]
    Nbc12plus,
    

    /// 15+
    ///
    /// "nbc15plus"
    #[serde(rename="nbc15plus")]
    Nbc15plus,
    

    /// 18+
    ///
    /// "nbc18plus"
    #[serde(rename="nbc18plus")]
    Nbc18plus,
    

    /// 18+R
    ///
    /// "nbc18plusr"
    #[serde(rename="nbc18plusr")]
    Nbc18plusr,
    

    /// PU
    ///
    /// "nbcPu"
    #[serde(rename="nbcPu")]
    NbcPu,
    
    /// "nbcUnrated"
    #[serde(rename="nbcUnrated")]
    NbcUnrated,
}

impl AsRef<str> for ContentRatingNbcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNbcRatingEnum::NbcUnspecified => "nbcUnspecified",
            ContentRatingNbcRatingEnum::NbcG => "nbcG",
            ContentRatingNbcRatingEnum::NbcPg => "nbcPg",
            ContentRatingNbcRatingEnum::Nbc12plus => "nbc12plus",
            ContentRatingNbcRatingEnum::Nbc15plus => "nbc15plus",
            ContentRatingNbcRatingEnum::Nbc18plus => "nbc18plus",
            ContentRatingNbcRatingEnum::Nbc18plusr => "nbc18plusr",
            ContentRatingNbcRatingEnum::NbcPu => "nbcPu",
            ContentRatingNbcRatingEnum::NbcUnrated => "nbcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingNbcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nbcUnspecified" => Ok(ContentRatingNbcRatingEnum::NbcUnspecified),
           "nbcG" => Ok(ContentRatingNbcRatingEnum::NbcG),
           "nbcPg" => Ok(ContentRatingNbcRatingEnum::NbcPg),
           "nbc12plus" => Ok(ContentRatingNbcRatingEnum::Nbc12plus),
           "nbc15plus" => Ok(ContentRatingNbcRatingEnum::Nbc15plus),
           "nbc18plus" => Ok(ContentRatingNbcRatingEnum::Nbc18plus),
           "nbc18plusr" => Ok(ContentRatingNbcRatingEnum::Nbc18plusr),
           "nbcPu" => Ok(ContentRatingNbcRatingEnum::NbcPu),
           "nbcUnrated" => Ok(ContentRatingNbcRatingEnum::NbcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNbcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingNbcplRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Poland.
pub enum ContentRatingNbcplRatingEnum {
    
    /// "nbcplUnspecified"
    #[serde(rename="nbcplUnspecified")]
    NbcplUnspecified,
    
    /// "nbcplI"
    #[serde(rename="nbcplI")]
    NbcplI,
    
    /// "nbcplIi"
    #[serde(rename="nbcplIi")]
    NbcplIi,
    
    /// "nbcplIii"
    #[serde(rename="nbcplIii")]
    NbcplIii,
    
    /// "nbcplIv"
    #[serde(rename="nbcplIv")]
    NbcplIv,
    
    /// "nbcpl18plus"
    #[serde(rename="nbcpl18plus")]
    Nbcpl18plus,
    
    /// "nbcplUnrated"
    #[serde(rename="nbcplUnrated")]
    NbcplUnrated,
}

impl AsRef<str> for ContentRatingNbcplRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNbcplRatingEnum::NbcplUnspecified => "nbcplUnspecified",
            ContentRatingNbcplRatingEnum::NbcplI => "nbcplI",
            ContentRatingNbcplRatingEnum::NbcplIi => "nbcplIi",
            ContentRatingNbcplRatingEnum::NbcplIii => "nbcplIii",
            ContentRatingNbcplRatingEnum::NbcplIv => "nbcplIv",
            ContentRatingNbcplRatingEnum::Nbcpl18plus => "nbcpl18plus",
            ContentRatingNbcplRatingEnum::NbcplUnrated => "nbcplUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingNbcplRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nbcplUnspecified" => Ok(ContentRatingNbcplRatingEnum::NbcplUnspecified),
           "nbcplI" => Ok(ContentRatingNbcplRatingEnum::NbcplI),
           "nbcplIi" => Ok(ContentRatingNbcplRatingEnum::NbcplIi),
           "nbcplIii" => Ok(ContentRatingNbcplRatingEnum::NbcplIii),
           "nbcplIv" => Ok(ContentRatingNbcplRatingEnum::NbcplIv),
           "nbcpl18plus" => Ok(ContentRatingNbcplRatingEnum::Nbcpl18plus),
           "nbcplUnrated" => Ok(ContentRatingNbcplRatingEnum::NbcplUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNbcplRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingNfrcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Bulgarian National Film Center.
pub enum ContentRatingNfrcRatingEnum {
    
    /// "nfrcUnspecified"
    #[serde(rename="nfrcUnspecified")]
    NfrcUnspecified,
    

    /// A
    ///
    /// "nfrcA"
    #[serde(rename="nfrcA")]
    NfrcA,
    

    /// B
    ///
    /// "nfrcB"
    #[serde(rename="nfrcB")]
    NfrcB,
    

    /// C
    ///
    /// "nfrcC"
    #[serde(rename="nfrcC")]
    NfrcC,
    

    /// D
    ///
    /// "nfrcD"
    #[serde(rename="nfrcD")]
    NfrcD,
    

    /// X
    ///
    /// "nfrcX"
    #[serde(rename="nfrcX")]
    NfrcX,
    
    /// "nfrcUnrated"
    #[serde(rename="nfrcUnrated")]
    NfrcUnrated,
}

impl AsRef<str> for ContentRatingNfrcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNfrcRatingEnum::NfrcUnspecified => "nfrcUnspecified",
            ContentRatingNfrcRatingEnum::NfrcA => "nfrcA",
            ContentRatingNfrcRatingEnum::NfrcB => "nfrcB",
            ContentRatingNfrcRatingEnum::NfrcC => "nfrcC",
            ContentRatingNfrcRatingEnum::NfrcD => "nfrcD",
            ContentRatingNfrcRatingEnum::NfrcX => "nfrcX",
            ContentRatingNfrcRatingEnum::NfrcUnrated => "nfrcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingNfrcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nfrcUnspecified" => Ok(ContentRatingNfrcRatingEnum::NfrcUnspecified),
           "nfrcA" => Ok(ContentRatingNfrcRatingEnum::NfrcA),
           "nfrcB" => Ok(ContentRatingNfrcRatingEnum::NfrcB),
           "nfrcC" => Ok(ContentRatingNfrcRatingEnum::NfrcC),
           "nfrcD" => Ok(ContentRatingNfrcRatingEnum::NfrcD),
           "nfrcX" => Ok(ContentRatingNfrcRatingEnum::NfrcX),
           "nfrcUnrated" => Ok(ContentRatingNfrcRatingEnum::NfrcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNfrcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingNfvcbRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Nigeria's National Film and Video Censors Board.
pub enum ContentRatingNfvcbRatingEnum {
    
    /// "nfvcbUnspecified"
    #[serde(rename="nfvcbUnspecified")]
    NfvcbUnspecified,
    

    /// G
    ///
    /// "nfvcbG"
    #[serde(rename="nfvcbG")]
    NfvcbG,
    

    /// PG
    ///
    /// "nfvcbPg"
    #[serde(rename="nfvcbPg")]
    NfvcbPg,
    

    /// 12
    ///
    /// "nfvcb12"
    #[serde(rename="nfvcb12")]
    Nfvcb12,
    

    /// 12A
    ///
    /// "nfvcb12a"
    #[serde(rename="nfvcb12a")]
    Nfvcb12a,
    

    /// 15
    ///
    /// "nfvcb15"
    #[serde(rename="nfvcb15")]
    Nfvcb15,
    

    /// 18
    ///
    /// "nfvcb18"
    #[serde(rename="nfvcb18")]
    Nfvcb18,
    

    /// RE
    ///
    /// "nfvcbRe"
    #[serde(rename="nfvcbRe")]
    NfvcbRe,
    
    /// "nfvcbUnrated"
    #[serde(rename="nfvcbUnrated")]
    NfvcbUnrated,
}

impl AsRef<str> for ContentRatingNfvcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNfvcbRatingEnum::NfvcbUnspecified => "nfvcbUnspecified",
            ContentRatingNfvcbRatingEnum::NfvcbG => "nfvcbG",
            ContentRatingNfvcbRatingEnum::NfvcbPg => "nfvcbPg",
            ContentRatingNfvcbRatingEnum::Nfvcb12 => "nfvcb12",
            ContentRatingNfvcbRatingEnum::Nfvcb12a => "nfvcb12a",
            ContentRatingNfvcbRatingEnum::Nfvcb15 => "nfvcb15",
            ContentRatingNfvcbRatingEnum::Nfvcb18 => "nfvcb18",
            ContentRatingNfvcbRatingEnum::NfvcbRe => "nfvcbRe",
            ContentRatingNfvcbRatingEnum::NfvcbUnrated => "nfvcbUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingNfvcbRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nfvcbUnspecified" => Ok(ContentRatingNfvcbRatingEnum::NfvcbUnspecified),
           "nfvcbG" => Ok(ContentRatingNfvcbRatingEnum::NfvcbG),
           "nfvcbPg" => Ok(ContentRatingNfvcbRatingEnum::NfvcbPg),
           "nfvcb12" => Ok(ContentRatingNfvcbRatingEnum::Nfvcb12),
           "nfvcb12a" => Ok(ContentRatingNfvcbRatingEnum::Nfvcb12a),
           "nfvcb15" => Ok(ContentRatingNfvcbRatingEnum::Nfvcb15),
           "nfvcb18" => Ok(ContentRatingNfvcbRatingEnum::Nfvcb18),
           "nfvcbRe" => Ok(ContentRatingNfvcbRatingEnum::NfvcbRe),
           "nfvcbUnrated" => Ok(ContentRatingNfvcbRatingEnum::NfvcbUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNfvcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingNkclvRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia).
pub enum ContentRatingNkclvRatingEnum {
    
    /// "nkclvUnspecified"
    #[serde(rename="nkclvUnspecified")]
    NkclvUnspecified,
    

    /// U
    ///
    /// "nkclvU"
    #[serde(rename="nkclvU")]
    NkclvU,
    

    /// 7+
    ///
    /// "nkclv7plus"
    #[serde(rename="nkclv7plus")]
    Nkclv7plus,
    

    /// 12+
    ///
    /// "nkclv12plus"
    #[serde(rename="nkclv12plus")]
    Nkclv12plus,
    

    /// ! 16+
    ///
    /// "nkclv16plus"
    #[serde(rename="nkclv16plus")]
    Nkclv16plus,
    

    /// 18+
    ///
    /// "nkclv18plus"
    #[serde(rename="nkclv18plus")]
    Nkclv18plus,
    
    /// "nkclvUnrated"
    #[serde(rename="nkclvUnrated")]
    NkclvUnrated,
}

impl AsRef<str> for ContentRatingNkclvRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNkclvRatingEnum::NkclvUnspecified => "nkclvUnspecified",
            ContentRatingNkclvRatingEnum::NkclvU => "nkclvU",
            ContentRatingNkclvRatingEnum::Nkclv7plus => "nkclv7plus",
            ContentRatingNkclvRatingEnum::Nkclv12plus => "nkclv12plus",
            ContentRatingNkclvRatingEnum::Nkclv16plus => "nkclv16plus",
            ContentRatingNkclvRatingEnum::Nkclv18plus => "nkclv18plus",
            ContentRatingNkclvRatingEnum::NkclvUnrated => "nkclvUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingNkclvRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nkclvUnspecified" => Ok(ContentRatingNkclvRatingEnum::NkclvUnspecified),
           "nkclvU" => Ok(ContentRatingNkclvRatingEnum::NkclvU),
           "nkclv7plus" => Ok(ContentRatingNkclvRatingEnum::Nkclv7plus),
           "nkclv12plus" => Ok(ContentRatingNkclvRatingEnum::Nkclv12plus),
           "nkclv16plus" => Ok(ContentRatingNkclvRatingEnum::Nkclv16plus),
           "nkclv18plus" => Ok(ContentRatingNkclvRatingEnum::Nkclv18plus),
           "nkclvUnrated" => Ok(ContentRatingNkclvRatingEnum::NkclvUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNkclvRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingNmcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The National Media Council ratings system for United Arab Emirates.
pub enum ContentRatingNmcRatingEnum {
    
    /// "nmcUnspecified"
    #[serde(rename="nmcUnspecified")]
    NmcUnspecified,
    

    /// G
    ///
    /// "nmcG"
    #[serde(rename="nmcG")]
    NmcG,
    

    /// PG
    ///
    /// "nmcPg"
    #[serde(rename="nmcPg")]
    NmcPg,
    

    /// PG-13
    ///
    /// "nmcPg13"
    #[serde(rename="nmcPg13")]
    NmcPg13,
    

    /// PG-15
    ///
    /// "nmcPg15"
    #[serde(rename="nmcPg15")]
    NmcPg15,
    

    /// 15+
    ///
    /// "nmc15plus"
    #[serde(rename="nmc15plus")]
    Nmc15plus,
    

    /// 18+
    ///
    /// "nmc18plus"
    #[serde(rename="nmc18plus")]
    Nmc18plus,
    

    /// 18TC
    ///
    /// "nmc18tc"
    #[serde(rename="nmc18tc")]
    Nmc18tc,
    
    /// "nmcUnrated"
    #[serde(rename="nmcUnrated")]
    NmcUnrated,
}

impl AsRef<str> for ContentRatingNmcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNmcRatingEnum::NmcUnspecified => "nmcUnspecified",
            ContentRatingNmcRatingEnum::NmcG => "nmcG",
            ContentRatingNmcRatingEnum::NmcPg => "nmcPg",
            ContentRatingNmcRatingEnum::NmcPg13 => "nmcPg13",
            ContentRatingNmcRatingEnum::NmcPg15 => "nmcPg15",
            ContentRatingNmcRatingEnum::Nmc15plus => "nmc15plus",
            ContentRatingNmcRatingEnum::Nmc18plus => "nmc18plus",
            ContentRatingNmcRatingEnum::Nmc18tc => "nmc18tc",
            ContentRatingNmcRatingEnum::NmcUnrated => "nmcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingNmcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nmcUnspecified" => Ok(ContentRatingNmcRatingEnum::NmcUnspecified),
           "nmcG" => Ok(ContentRatingNmcRatingEnum::NmcG),
           "nmcPg" => Ok(ContentRatingNmcRatingEnum::NmcPg),
           "nmcPg13" => Ok(ContentRatingNmcRatingEnum::NmcPg13),
           "nmcPg15" => Ok(ContentRatingNmcRatingEnum::NmcPg15),
           "nmc15plus" => Ok(ContentRatingNmcRatingEnum::Nmc15plus),
           "nmc18plus" => Ok(ContentRatingNmcRatingEnum::Nmc18plus),
           "nmc18tc" => Ok(ContentRatingNmcRatingEnum::Nmc18tc),
           "nmcUnrated" => Ok(ContentRatingNmcRatingEnum::NmcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNmcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingOflcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Office of Film and Literature Classification (OFLC - New Zealand) rating.
pub enum ContentRatingOflcRatingEnum {
    
    /// "oflcUnspecified"
    #[serde(rename="oflcUnspecified")]
    OflcUnspecified,
    

    /// G
    ///
    /// "oflcG"
    #[serde(rename="oflcG")]
    OflcG,
    

    /// PG
    ///
    /// "oflcPg"
    #[serde(rename="oflcPg")]
    OflcPg,
    

    /// M
    ///
    /// "oflcM"
    #[serde(rename="oflcM")]
    OflcM,
    

    /// R13
    ///
    /// "oflcR13"
    #[serde(rename="oflcR13")]
    OflcR13,
    

    /// R15
    ///
    /// "oflcR15"
    #[serde(rename="oflcR15")]
    OflcR15,
    

    /// R16
    ///
    /// "oflcR16"
    #[serde(rename="oflcR16")]
    OflcR16,
    

    /// R18
    ///
    /// "oflcR18"
    #[serde(rename="oflcR18")]
    OflcR18,
    
    /// "oflcUnrated"
    #[serde(rename="oflcUnrated")]
    OflcUnrated,
    

    /// RP13
    ///
    /// "oflcRp13"
    #[serde(rename="oflcRp13")]
    OflcRp13,
    

    /// RP16
    ///
    /// "oflcRp16"
    #[serde(rename="oflcRp16")]
    OflcRp16,
    

    /// RP18
    ///
    /// "oflcRp18"
    #[serde(rename="oflcRp18")]
    OflcRp18,
}

impl AsRef<str> for ContentRatingOflcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingOflcRatingEnum::OflcUnspecified => "oflcUnspecified",
            ContentRatingOflcRatingEnum::OflcG => "oflcG",
            ContentRatingOflcRatingEnum::OflcPg => "oflcPg",
            ContentRatingOflcRatingEnum::OflcM => "oflcM",
            ContentRatingOflcRatingEnum::OflcR13 => "oflcR13",
            ContentRatingOflcRatingEnum::OflcR15 => "oflcR15",
            ContentRatingOflcRatingEnum::OflcR16 => "oflcR16",
            ContentRatingOflcRatingEnum::OflcR18 => "oflcR18",
            ContentRatingOflcRatingEnum::OflcUnrated => "oflcUnrated",
            ContentRatingOflcRatingEnum::OflcRp13 => "oflcRp13",
            ContentRatingOflcRatingEnum::OflcRp16 => "oflcRp16",
            ContentRatingOflcRatingEnum::OflcRp18 => "oflcRp18",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingOflcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "oflcUnspecified" => Ok(ContentRatingOflcRatingEnum::OflcUnspecified),
           "oflcG" => Ok(ContentRatingOflcRatingEnum::OflcG),
           "oflcPg" => Ok(ContentRatingOflcRatingEnum::OflcPg),
           "oflcM" => Ok(ContentRatingOflcRatingEnum::OflcM),
           "oflcR13" => Ok(ContentRatingOflcRatingEnum::OflcR13),
           "oflcR15" => Ok(ContentRatingOflcRatingEnum::OflcR15),
           "oflcR16" => Ok(ContentRatingOflcRatingEnum::OflcR16),
           "oflcR18" => Ok(ContentRatingOflcRatingEnum::OflcR18),
           "oflcUnrated" => Ok(ContentRatingOflcRatingEnum::OflcUnrated),
           "oflcRp13" => Ok(ContentRatingOflcRatingEnum::OflcRp13),
           "oflcRp16" => Ok(ContentRatingOflcRatingEnum::OflcRp16),
           "oflcRp18" => Ok(ContentRatingOflcRatingEnum::OflcRp18),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingOflcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingPefilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Peru.
pub enum ContentRatingPefilmRatingEnum {
    
    /// "pefilmUnspecified"
    #[serde(rename="pefilmUnspecified")]
    PefilmUnspecified,
    

    /// PT
    ///
    /// "pefilmPt"
    #[serde(rename="pefilmPt")]
    PefilmPt,
    

    /// PG
    ///
    /// "pefilmPg"
    #[serde(rename="pefilmPg")]
    PefilmPg,
    

    /// 14
    ///
    /// "pefilm14"
    #[serde(rename="pefilm14")]
    Pefilm14,
    

    /// 18
    ///
    /// "pefilm18"
    #[serde(rename="pefilm18")]
    Pefilm18,
    
    /// "pefilmUnrated"
    #[serde(rename="pefilmUnrated")]
    PefilmUnrated,
}

impl AsRef<str> for ContentRatingPefilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingPefilmRatingEnum::PefilmUnspecified => "pefilmUnspecified",
            ContentRatingPefilmRatingEnum::PefilmPt => "pefilmPt",
            ContentRatingPefilmRatingEnum::PefilmPg => "pefilmPg",
            ContentRatingPefilmRatingEnum::Pefilm14 => "pefilm14",
            ContentRatingPefilmRatingEnum::Pefilm18 => "pefilm18",
            ContentRatingPefilmRatingEnum::PefilmUnrated => "pefilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingPefilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "pefilmUnspecified" => Ok(ContentRatingPefilmRatingEnum::PefilmUnspecified),
           "pefilmPt" => Ok(ContentRatingPefilmRatingEnum::PefilmPt),
           "pefilmPg" => Ok(ContentRatingPefilmRatingEnum::PefilmPg),
           "pefilm14" => Ok(ContentRatingPefilmRatingEnum::Pefilm14),
           "pefilm18" => Ok(ContentRatingPefilmRatingEnum::Pefilm18),
           "pefilmUnrated" => Ok(ContentRatingPefilmRatingEnum::PefilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingPefilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingRcnofRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film.
pub enum ContentRatingRcnofRatingEnum {
    
    /// "rcnofUnspecified"
    #[serde(rename="rcnofUnspecified")]
    RcnofUnspecified,
    
    /// "rcnofI"
    #[serde(rename="rcnofI")]
    RcnofI,
    
    /// "rcnofIi"
    #[serde(rename="rcnofIi")]
    RcnofIi,
    
    /// "rcnofIii"
    #[serde(rename="rcnofIii")]
    RcnofIii,
    
    /// "rcnofIv"
    #[serde(rename="rcnofIv")]
    RcnofIv,
    
    /// "rcnofV"
    #[serde(rename="rcnofV")]
    RcnofV,
    
    /// "rcnofVi"
    #[serde(rename="rcnofVi")]
    RcnofVi,
    
    /// "rcnofUnrated"
    #[serde(rename="rcnofUnrated")]
    RcnofUnrated,
}

impl AsRef<str> for ContentRatingRcnofRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRcnofRatingEnum::RcnofUnspecified => "rcnofUnspecified",
            ContentRatingRcnofRatingEnum::RcnofI => "rcnofI",
            ContentRatingRcnofRatingEnum::RcnofIi => "rcnofIi",
            ContentRatingRcnofRatingEnum::RcnofIii => "rcnofIii",
            ContentRatingRcnofRatingEnum::RcnofIv => "rcnofIv",
            ContentRatingRcnofRatingEnum::RcnofV => "rcnofV",
            ContentRatingRcnofRatingEnum::RcnofVi => "rcnofVi",
            ContentRatingRcnofRatingEnum::RcnofUnrated => "rcnofUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingRcnofRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rcnofUnspecified" => Ok(ContentRatingRcnofRatingEnum::RcnofUnspecified),
           "rcnofI" => Ok(ContentRatingRcnofRatingEnum::RcnofI),
           "rcnofIi" => Ok(ContentRatingRcnofRatingEnum::RcnofIi),
           "rcnofIii" => Ok(ContentRatingRcnofRatingEnum::RcnofIii),
           "rcnofIv" => Ok(ContentRatingRcnofRatingEnum::RcnofIv),
           "rcnofV" => Ok(ContentRatingRcnofRatingEnum::RcnofV),
           "rcnofVi" => Ok(ContentRatingRcnofRatingEnum::RcnofVi),
           "rcnofUnrated" => Ok(ContentRatingRcnofRatingEnum::RcnofUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRcnofRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingResorteviolenciaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Venezuela.
pub enum ContentRatingResorteviolenciaRatingEnum {
    
    /// "resorteviolenciaUnspecified"
    #[serde(rename="resorteviolenciaUnspecified")]
    ResorteviolenciaUnspecified,
    

    /// A
    ///
    /// "resorteviolenciaA"
    #[serde(rename="resorteviolenciaA")]
    ResorteviolenciaA,
    

    /// B
    ///
    /// "resorteviolenciaB"
    #[serde(rename="resorteviolenciaB")]
    ResorteviolenciaB,
    

    /// C
    ///
    /// "resorteviolenciaC"
    #[serde(rename="resorteviolenciaC")]
    ResorteviolenciaC,
    

    /// D
    ///
    /// "resorteviolenciaD"
    #[serde(rename="resorteviolenciaD")]
    ResorteviolenciaD,
    

    /// E
    ///
    /// "resorteviolenciaE"
    #[serde(rename="resorteviolenciaE")]
    ResorteviolenciaE,
    
    /// "resorteviolenciaUnrated"
    #[serde(rename="resorteviolenciaUnrated")]
    ResorteviolenciaUnrated,
}

impl AsRef<str> for ContentRatingResorteviolenciaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaUnspecified => "resorteviolenciaUnspecified",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaA => "resorteviolenciaA",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaB => "resorteviolenciaB",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaC => "resorteviolenciaC",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaD => "resorteviolenciaD",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaE => "resorteviolenciaE",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaUnrated => "resorteviolenciaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingResorteviolenciaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "resorteviolenciaUnspecified" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaUnspecified),
           "resorteviolenciaA" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaA),
           "resorteviolenciaB" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaB),
           "resorteviolenciaC" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaC),
           "resorteviolenciaD" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaD),
           "resorteviolenciaE" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaE),
           "resorteviolenciaUnrated" => Ok(ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingResorteviolenciaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingRtcRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's General Directorate of Radio, Television and Cinematography (Mexico) rating.
pub enum ContentRatingRtcRatingEnum {
    
    /// "rtcUnspecified"
    #[serde(rename="rtcUnspecified")]
    RtcUnspecified,
    

    /// AA
    ///
    /// "rtcAa"
    #[serde(rename="rtcAa")]
    RtcAa,
    

    /// A
    ///
    /// "rtcA"
    #[serde(rename="rtcA")]
    RtcA,
    

    /// B
    ///
    /// "rtcB"
    #[serde(rename="rtcB")]
    RtcB,
    

    /// B15
    ///
    /// "rtcB15"
    #[serde(rename="rtcB15")]
    RtcB15,
    

    /// C
    ///
    /// "rtcC"
    #[serde(rename="rtcC")]
    RtcC,
    

    /// D
    ///
    /// "rtcD"
    #[serde(rename="rtcD")]
    RtcD,
    
    /// "rtcUnrated"
    #[serde(rename="rtcUnrated")]
    RtcUnrated,
}

impl AsRef<str> for ContentRatingRtcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRtcRatingEnum::RtcUnspecified => "rtcUnspecified",
            ContentRatingRtcRatingEnum::RtcAa => "rtcAa",
            ContentRatingRtcRatingEnum::RtcA => "rtcA",
            ContentRatingRtcRatingEnum::RtcB => "rtcB",
            ContentRatingRtcRatingEnum::RtcB15 => "rtcB15",
            ContentRatingRtcRatingEnum::RtcC => "rtcC",
            ContentRatingRtcRatingEnum::RtcD => "rtcD",
            ContentRatingRtcRatingEnum::RtcUnrated => "rtcUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingRtcRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rtcUnspecified" => Ok(ContentRatingRtcRatingEnum::RtcUnspecified),
           "rtcAa" => Ok(ContentRatingRtcRatingEnum::RtcAa),
           "rtcA" => Ok(ContentRatingRtcRatingEnum::RtcA),
           "rtcB" => Ok(ContentRatingRtcRatingEnum::RtcB),
           "rtcB15" => Ok(ContentRatingRtcRatingEnum::RtcB15),
           "rtcC" => Ok(ContentRatingRtcRatingEnum::RtcC),
           "rtcD" => Ok(ContentRatingRtcRatingEnum::RtcD),
           "rtcUnrated" => Ok(ContentRatingRtcRatingEnum::RtcUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRtcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingRteRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Ireland's Raidió Teilifís Éireann.
pub enum ContentRatingRteRatingEnum {
    
    /// "rteUnspecified"
    #[serde(rename="rteUnspecified")]
    RteUnspecified,
    

    /// GA
    ///
    /// "rteGa"
    #[serde(rename="rteGa")]
    RteGa,
    

    /// CH
    ///
    /// "rteCh"
    #[serde(rename="rteCh")]
    RteCh,
    

    /// PS
    ///
    /// "rtePs"
    #[serde(rename="rtePs")]
    RtePs,
    

    /// MA
    ///
    /// "rteMa"
    #[serde(rename="rteMa")]
    RteMa,
    
    /// "rteUnrated"
    #[serde(rename="rteUnrated")]
    RteUnrated,
}

impl AsRef<str> for ContentRatingRteRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRteRatingEnum::RteUnspecified => "rteUnspecified",
            ContentRatingRteRatingEnum::RteGa => "rteGa",
            ContentRatingRteRatingEnum::RteCh => "rteCh",
            ContentRatingRteRatingEnum::RtePs => "rtePs",
            ContentRatingRteRatingEnum::RteMa => "rteMa",
            ContentRatingRteRatingEnum::RteUnrated => "rteUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingRteRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rteUnspecified" => Ok(ContentRatingRteRatingEnum::RteUnspecified),
           "rteGa" => Ok(ContentRatingRteRatingEnum::RteGa),
           "rteCh" => Ok(ContentRatingRteRatingEnum::RteCh),
           "rtePs" => Ok(ContentRatingRteRatingEnum::RtePs),
           "rteMa" => Ok(ContentRatingRteRatingEnum::RteMa),
           "rteUnrated" => Ok(ContentRatingRteRatingEnum::RteUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRteRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingRussiaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's National Film Registry of the Russian Federation (MKRF - Russia) rating.
pub enum ContentRatingRussiaRatingEnum {
    
    /// "russiaUnspecified"
    #[serde(rename="russiaUnspecified")]
    RussiaUnspecified,
    

    /// 0+
    ///
    /// "russia0"
    #[serde(rename="russia0")]
    Russia0,
    

    /// 6+
    ///
    /// "russia6"
    #[serde(rename="russia6")]
    Russia6,
    

    /// 12+
    ///
    /// "russia12"
    #[serde(rename="russia12")]
    Russia12,
    

    /// 16+
    ///
    /// "russia16"
    #[serde(rename="russia16")]
    Russia16,
    

    /// 18+
    ///
    /// "russia18"
    #[serde(rename="russia18")]
    Russia18,
    
    /// "russiaUnrated"
    #[serde(rename="russiaUnrated")]
    RussiaUnrated,
}

impl AsRef<str> for ContentRatingRussiaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRussiaRatingEnum::RussiaUnspecified => "russiaUnspecified",
            ContentRatingRussiaRatingEnum::Russia0 => "russia0",
            ContentRatingRussiaRatingEnum::Russia6 => "russia6",
            ContentRatingRussiaRatingEnum::Russia12 => "russia12",
            ContentRatingRussiaRatingEnum::Russia16 => "russia16",
            ContentRatingRussiaRatingEnum::Russia18 => "russia18",
            ContentRatingRussiaRatingEnum::RussiaUnrated => "russiaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingRussiaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "russiaUnspecified" => Ok(ContentRatingRussiaRatingEnum::RussiaUnspecified),
           "russia0" => Ok(ContentRatingRussiaRatingEnum::Russia0),
           "russia6" => Ok(ContentRatingRussiaRatingEnum::Russia6),
           "russia12" => Ok(ContentRatingRussiaRatingEnum::Russia12),
           "russia16" => Ok(ContentRatingRussiaRatingEnum::Russia16),
           "russia18" => Ok(ContentRatingRussiaRatingEnum::Russia18),
           "russiaUnrated" => Ok(ContentRatingRussiaRatingEnum::RussiaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRussiaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingSkfilmRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Slovakia.
pub enum ContentRatingSkfilmRatingEnum {
    
    /// "skfilmUnspecified"
    #[serde(rename="skfilmUnspecified")]
    SkfilmUnspecified,
    

    /// G
    ///
    /// "skfilmG"
    #[serde(rename="skfilmG")]
    SkfilmG,
    

    /// P2
    ///
    /// "skfilmP2"
    #[serde(rename="skfilmP2")]
    SkfilmP2,
    

    /// P5
    ///
    /// "skfilmP5"
    #[serde(rename="skfilmP5")]
    SkfilmP5,
    

    /// P8
    ///
    /// "skfilmP8"
    #[serde(rename="skfilmP8")]
    SkfilmP8,
    
    /// "skfilmUnrated"
    #[serde(rename="skfilmUnrated")]
    SkfilmUnrated,
}

impl AsRef<str> for ContentRatingSkfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingSkfilmRatingEnum::SkfilmUnspecified => "skfilmUnspecified",
            ContentRatingSkfilmRatingEnum::SkfilmG => "skfilmG",
            ContentRatingSkfilmRatingEnum::SkfilmP2 => "skfilmP2",
            ContentRatingSkfilmRatingEnum::SkfilmP5 => "skfilmP5",
            ContentRatingSkfilmRatingEnum::SkfilmP8 => "skfilmP8",
            ContentRatingSkfilmRatingEnum::SkfilmUnrated => "skfilmUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingSkfilmRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "skfilmUnspecified" => Ok(ContentRatingSkfilmRatingEnum::SkfilmUnspecified),
           "skfilmG" => Ok(ContentRatingSkfilmRatingEnum::SkfilmG),
           "skfilmP2" => Ok(ContentRatingSkfilmRatingEnum::SkfilmP2),
           "skfilmP5" => Ok(ContentRatingSkfilmRatingEnum::SkfilmP5),
           "skfilmP8" => Ok(ContentRatingSkfilmRatingEnum::SkfilmP8),
           "skfilmUnrated" => Ok(ContentRatingSkfilmRatingEnum::SkfilmUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingSkfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingSmaisRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Iceland.
pub enum ContentRatingSmaisRatingEnum {
    
    /// "smaisUnspecified"
    #[serde(rename="smaisUnspecified")]
    SmaisUnspecified,
    

    /// L
    ///
    /// "smaisL"
    #[serde(rename="smaisL")]
    SmaisL,
    

    /// 7
    ///
    /// "smais7"
    #[serde(rename="smais7")]
    Smais7,
    

    /// 12
    ///
    /// "smais12"
    #[serde(rename="smais12")]
    Smais12,
    

    /// 14
    ///
    /// "smais14"
    #[serde(rename="smais14")]
    Smais14,
    

    /// 16
    ///
    /// "smais16"
    #[serde(rename="smais16")]
    Smais16,
    

    /// 18
    ///
    /// "smais18"
    #[serde(rename="smais18")]
    Smais18,
    
    /// "smaisUnrated"
    #[serde(rename="smaisUnrated")]
    SmaisUnrated,
}

impl AsRef<str> for ContentRatingSmaisRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingSmaisRatingEnum::SmaisUnspecified => "smaisUnspecified",
            ContentRatingSmaisRatingEnum::SmaisL => "smaisL",
            ContentRatingSmaisRatingEnum::Smais7 => "smais7",
            ContentRatingSmaisRatingEnum::Smais12 => "smais12",
            ContentRatingSmaisRatingEnum::Smais14 => "smais14",
            ContentRatingSmaisRatingEnum::Smais16 => "smais16",
            ContentRatingSmaisRatingEnum::Smais18 => "smais18",
            ContentRatingSmaisRatingEnum::SmaisUnrated => "smaisUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingSmaisRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "smaisUnspecified" => Ok(ContentRatingSmaisRatingEnum::SmaisUnspecified),
           "smaisL" => Ok(ContentRatingSmaisRatingEnum::SmaisL),
           "smais7" => Ok(ContentRatingSmaisRatingEnum::Smais7),
           "smais12" => Ok(ContentRatingSmaisRatingEnum::Smais12),
           "smais14" => Ok(ContentRatingSmaisRatingEnum::Smais14),
           "smais16" => Ok(ContentRatingSmaisRatingEnum::Smais16),
           "smais18" => Ok(ContentRatingSmaisRatingEnum::Smais18),
           "smaisUnrated" => Ok(ContentRatingSmaisRatingEnum::SmaisUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingSmaisRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingSmsaRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Statens medieråd (Sweden's National Media Council).
pub enum ContentRatingSmsaRatingEnum {
    
    /// "smsaUnspecified"
    #[serde(rename="smsaUnspecified")]
    SmsaUnspecified,
    

    /// All ages
    ///
    /// "smsaA"
    #[serde(rename="smsaA")]
    SmsaA,
    

    /// 7
    ///
    /// "smsa7"
    #[serde(rename="smsa7")]
    Smsa7,
    

    /// 11
    ///
    /// "smsa11"
    #[serde(rename="smsa11")]
    Smsa11,
    

    /// 15
    ///
    /// "smsa15"
    #[serde(rename="smsa15")]
    Smsa15,
    
    /// "smsaUnrated"
    #[serde(rename="smsaUnrated")]
    SmsaUnrated,
}

impl AsRef<str> for ContentRatingSmsaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingSmsaRatingEnum::SmsaUnspecified => "smsaUnspecified",
            ContentRatingSmsaRatingEnum::SmsaA => "smsaA",
            ContentRatingSmsaRatingEnum::Smsa7 => "smsa7",
            ContentRatingSmsaRatingEnum::Smsa11 => "smsa11",
            ContentRatingSmsaRatingEnum::Smsa15 => "smsa15",
            ContentRatingSmsaRatingEnum::SmsaUnrated => "smsaUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingSmsaRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "smsaUnspecified" => Ok(ContentRatingSmsaRatingEnum::SmsaUnspecified),
           "smsaA" => Ok(ContentRatingSmsaRatingEnum::SmsaA),
           "smsa7" => Ok(ContentRatingSmsaRatingEnum::Smsa7),
           "smsa11" => Ok(ContentRatingSmsaRatingEnum::Smsa11),
           "smsa15" => Ok(ContentRatingSmsaRatingEnum::Smsa15),
           "smsaUnrated" => Ok(ContentRatingSmsaRatingEnum::SmsaUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingSmsaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingTvpgRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's TV Parental Guidelines (TVPG) rating.
pub enum ContentRatingTvpgRatingEnum {
    
    /// "tvpgUnspecified"
    #[serde(rename="tvpgUnspecified")]
    TvpgUnspecified,
    

    /// TV-Y
    ///
    /// "tvpgY"
    #[serde(rename="tvpgY")]
    TvpgY,
    

    /// TV-Y7
    ///
    /// "tvpgY7"
    #[serde(rename="tvpgY7")]
    TvpgY7,
    

    /// TV-Y7-FV
    ///
    /// "tvpgY7Fv"
    #[serde(rename="tvpgY7Fv")]
    TvpgY7Fv,
    

    /// TV-G
    ///
    /// "tvpgG"
    #[serde(rename="tvpgG")]
    TvpgG,
    

    /// TV-PG
    ///
    /// "tvpgPg"
    #[serde(rename="tvpgPg")]
    TvpgPg,
    

    /// TV-14
    ///
    /// "pg14"
    #[serde(rename="pg14")]
    Pg14,
    

    /// TV-MA
    ///
    /// "tvpgMa"
    #[serde(rename="tvpgMa")]
    TvpgMa,
    
    /// "tvpgUnrated"
    #[serde(rename="tvpgUnrated")]
    TvpgUnrated,
}

impl AsRef<str> for ContentRatingTvpgRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingTvpgRatingEnum::TvpgUnspecified => "tvpgUnspecified",
            ContentRatingTvpgRatingEnum::TvpgY => "tvpgY",
            ContentRatingTvpgRatingEnum::TvpgY7 => "tvpgY7",
            ContentRatingTvpgRatingEnum::TvpgY7Fv => "tvpgY7Fv",
            ContentRatingTvpgRatingEnum::TvpgG => "tvpgG",
            ContentRatingTvpgRatingEnum::TvpgPg => "tvpgPg",
            ContentRatingTvpgRatingEnum::Pg14 => "pg14",
            ContentRatingTvpgRatingEnum::TvpgMa => "tvpgMa",
            ContentRatingTvpgRatingEnum::TvpgUnrated => "tvpgUnrated",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingTvpgRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "tvpgUnspecified" => Ok(ContentRatingTvpgRatingEnum::TvpgUnspecified),
           "tvpgY" => Ok(ContentRatingTvpgRatingEnum::TvpgY),
           "tvpgY7" => Ok(ContentRatingTvpgRatingEnum::TvpgY7),
           "tvpgY7Fv" => Ok(ContentRatingTvpgRatingEnum::TvpgY7Fv),
           "tvpgG" => Ok(ContentRatingTvpgRatingEnum::TvpgG),
           "tvpgPg" => Ok(ContentRatingTvpgRatingEnum::TvpgPg),
           "pg14" => Ok(ContentRatingTvpgRatingEnum::Pg14),
           "tvpgMa" => Ok(ContentRatingTvpgRatingEnum::TvpgMa),
           "tvpgUnrated" => Ok(ContentRatingTvpgRatingEnum::TvpgUnrated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingTvpgRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContentRatingYtRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A rating that YouTube uses to identify age-restricted content.
pub enum ContentRatingYtRatingEnum {
    
    /// "ytUnspecified"
    #[serde(rename="ytUnspecified")]
    YtUnspecified,
    
    /// "ytAgeRestricted"
    #[serde(rename="ytAgeRestricted")]
    YtAgeRestricted,
}

impl AsRef<str> for ContentRatingYtRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingYtRatingEnum::YtUnspecified => "ytUnspecified",
            ContentRatingYtRatingEnum::YtAgeRestricted => "ytAgeRestricted",
        }
    }
}

impl std::convert::TryFrom< &str> for ContentRatingYtRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ytUnspecified" => Ok(ContentRatingYtRatingEnum::YtUnspecified),
           "ytAgeRestricted" => Ok(ContentRatingYtRatingEnum::YtAgeRestricted),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingYtRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CuepointCueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CuepointCueTypeEnum {
    
    /// "cueTypeUnspecified"
    #[serde(rename="cueTypeUnspecified")]
    CueTypeUnspecified,
    
    /// "cueTypeAd"
    #[serde(rename="cueTypeAd")]
    CueTypeAd,
}

impl AsRef<str> for CuepointCueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CuepointCueTypeEnum::CueTypeUnspecified => "cueTypeUnspecified",
            CuepointCueTypeEnum::CueTypeAd => "cueTypeAd",
        }
    }
}

impl std::convert::TryFrom< &str> for CuepointCueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cueTypeUnspecified" => Ok(CuepointCueTypeEnum::CueTypeUnspecified),
           "cueTypeAd" => Ok(CuepointCueTypeEnum::CueTypeAd),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CuepointCueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CuepointScheduleScheduleStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The strategy to use when scheduling cuepoints.
pub enum CuepointScheduleScheduleStrategyEnum {
    
    /// "scheduleStrategyUnspecified"
    #[serde(rename="scheduleStrategyUnspecified")]
    ScheduleStrategyUnspecified,
    

    /// Strategy to schedule cuepoints at one time for all viewers.
    ///
    /// "concurrent"
    #[serde(rename="concurrent")]
    Concurrent,
    

    /// Strategy to schedule cuepoints at an increased rate to allow viewers to receive cuepoints when eligible. See go/lcr-non-concurrent-ads for more details.
    ///
    /// "nonConcurrent"
    #[serde(rename="nonConcurrent")]
    NonConcurrent,
}

impl AsRef<str> for CuepointScheduleScheduleStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CuepointScheduleScheduleStrategyEnum::ScheduleStrategyUnspecified => "scheduleStrategyUnspecified",
            CuepointScheduleScheduleStrategyEnum::Concurrent => "concurrent",
            CuepointScheduleScheduleStrategyEnum::NonConcurrent => "nonConcurrent",
        }
    }
}

impl std::convert::TryFrom< &str> for CuepointScheduleScheduleStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "scheduleStrategyUnspecified" => Ok(CuepointScheduleScheduleStrategyEnum::ScheduleStrategyUnspecified),
           "concurrent" => Ok(CuepointScheduleScheduleStrategyEnum::Concurrent),
           "nonConcurrent" => Ok(CuepointScheduleScheduleStrategyEnum::NonConcurrent),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CuepointScheduleScheduleStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InvideoPositionCornerPositionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes in which corner of the video the visual widget will appear.
pub enum InvideoPositionCornerPositionEnum {
    
    /// "topLeft"
    #[serde(rename="topLeft")]
    TopLeft,
    
    /// "topRight"
    #[serde(rename="topRight")]
    TopRight,
    
    /// "bottomLeft"
    #[serde(rename="bottomLeft")]
    BottomLeft,
    
    /// "bottomRight"
    #[serde(rename="bottomRight")]
    BottomRight,
}

impl AsRef<str> for InvideoPositionCornerPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvideoPositionCornerPositionEnum::TopLeft => "topLeft",
            InvideoPositionCornerPositionEnum::TopRight => "topRight",
            InvideoPositionCornerPositionEnum::BottomLeft => "bottomLeft",
            InvideoPositionCornerPositionEnum::BottomRight => "bottomRight",
        }
    }
}

impl std::convert::TryFrom< &str> for InvideoPositionCornerPositionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "topLeft" => Ok(InvideoPositionCornerPositionEnum::TopLeft),
           "topRight" => Ok(InvideoPositionCornerPositionEnum::TopRight),
           "bottomLeft" => Ok(InvideoPositionCornerPositionEnum::BottomLeft),
           "bottomRight" => Ok(InvideoPositionCornerPositionEnum::BottomRight),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvideoPositionCornerPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InvideoPositionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the position type.
pub enum InvideoPositionTypeEnum {
    
    /// "corner"
    #[serde(rename="corner")]
    Corner,
}

impl AsRef<str> for InvideoPositionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvideoPositionTypeEnum::Corner => "corner",
        }
    }
}

impl std::convert::TryFrom< &str> for InvideoPositionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "corner" => Ok(InvideoPositionTypeEnum::Corner),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvideoPositionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InvideoTimingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.
pub enum InvideoTimingTypeEnum {
    
    /// "offsetFromStart"
    #[serde(rename="offsetFromStart")]
    OffsetFromStart,
    
    /// "offsetFromEnd"
    #[serde(rename="offsetFromEnd")]
    OffsetFromEnd,
}

impl AsRef<str> for InvideoTimingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvideoTimingTypeEnum::OffsetFromStart => "offsetFromStart",
            InvideoTimingTypeEnum::OffsetFromEnd => "offsetFromEnd",
        }
    }
}

impl std::convert::TryFrom< &str> for InvideoTimingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "offsetFromStart" => Ok(InvideoTimingTypeEnum::OffsetFromStart),
           "offsetFromEnd" => Ok(InvideoTimingTypeEnum::OffsetFromEnd),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvideoTimingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastContentDetailClosedCaptionsTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    
    /// "closedCaptionsTypeUnspecified"
    #[serde(rename="closedCaptionsTypeUnspecified")]
    ClosedCaptionsTypeUnspecified,
    
    /// "closedCaptionsDisabled"
    #[serde(rename="closedCaptionsDisabled")]
    ClosedCaptionsDisabled,
    
    /// "closedCaptionsHttpPost"
    #[serde(rename="closedCaptionsHttpPost")]
    ClosedCaptionsHttpPost,
    
    /// "closedCaptionsEmbedded"
    #[serde(rename="closedCaptionsEmbedded")]
    ClosedCaptionsEmbedded,
}

impl AsRef<str> for LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsTypeUnspecified => "closedCaptionsTypeUnspecified",
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsDisabled => "closedCaptionsDisabled",
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsHttpPost => "closedCaptionsHttpPost",
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsEmbedded => "closedCaptionsEmbedded",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "closedCaptionsTypeUnspecified" => Ok(LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsTypeUnspecified),
           "closedCaptionsDisabled" => Ok(LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsDisabled),
           "closedCaptionsHttpPost" => Ok(LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsHttpPost),
           "closedCaptionsEmbedded" => Ok(LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsEmbedded),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastContentDetailLatencyPreferenceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If both this and enable_low_latency are set, they must match. LATENCY_NORMAL should match enable_low_latency=false LATENCY_LOW should match enable_low_latency=true LATENCY_ULTRA_LOW should have enable_low_latency omitted.
pub enum LiveBroadcastContentDetailLatencyPreferenceEnum {
    
    /// "latencyPreferenceUnspecified"
    #[serde(rename="latencyPreferenceUnspecified")]
    LatencyPreferenceUnspecified,
    

    /// Best for: highest quality viewer playbacks and higher resolutions.
    ///
    /// "normal"
    #[serde(rename="normal")]
    Normal,
    

    /// Best for: near real-time interaction, with minimal playback buffering.
    ///
    /// "low"
    #[serde(rename="low")]
    Low,
    

    /// Best for: real-time interaction Does not support: Closed captions, 1440p, and 4k resolutions
    ///
    /// "ultraLow"
    #[serde(rename="ultraLow")]
    UltraLow,
}

impl AsRef<str> for LiveBroadcastContentDetailLatencyPreferenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailLatencyPreferenceEnum::LatencyPreferenceUnspecified => "latencyPreferenceUnspecified",
            LiveBroadcastContentDetailLatencyPreferenceEnum::Normal => "normal",
            LiveBroadcastContentDetailLatencyPreferenceEnum::Low => "low",
            LiveBroadcastContentDetailLatencyPreferenceEnum::UltraLow => "ultraLow",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastContentDetailLatencyPreferenceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "latencyPreferenceUnspecified" => Ok(LiveBroadcastContentDetailLatencyPreferenceEnum::LatencyPreferenceUnspecified),
           "normal" => Ok(LiveBroadcastContentDetailLatencyPreferenceEnum::Normal),
           "low" => Ok(LiveBroadcastContentDetailLatencyPreferenceEnum::Low),
           "ultraLow" => Ok(LiveBroadcastContentDetailLatencyPreferenceEnum::UltraLow),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailLatencyPreferenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastContentDetailProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The projection format of this broadcast. This defaults to rectangular.
pub enum LiveBroadcastContentDetailProjectionEnum {
    
    /// "projectionUnspecified"
    #[serde(rename="projectionUnspecified")]
    ProjectionUnspecified,
    
    /// "rectangular"
    #[serde(rename="rectangular")]
    Rectangular,
    
    /// "360"
    #[serde(rename="360")]
    _360,
    
    /// "mesh"
    #[serde(rename="mesh")]
    Mesh,
}

impl AsRef<str> for LiveBroadcastContentDetailProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailProjectionEnum::ProjectionUnspecified => "projectionUnspecified",
            LiveBroadcastContentDetailProjectionEnum::Rectangular => "rectangular",
            LiveBroadcastContentDetailProjectionEnum::_360 => "360",
            LiveBroadcastContentDetailProjectionEnum::Mesh => "mesh",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastContentDetailProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "projectionUnspecified" => Ok(LiveBroadcastContentDetailProjectionEnum::ProjectionUnspecified),
           "rectangular" => Ok(LiveBroadcastContentDetailProjectionEnum::Rectangular),
           "360" => Ok(LiveBroadcastContentDetailProjectionEnum::_360),
           "mesh" => Ok(LiveBroadcastContentDetailProjectionEnum::Mesh),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastContentDetailStereoLayoutEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The 3D stereo layout of this broadcast. This defaults to mono.
pub enum LiveBroadcastContentDetailStereoLayoutEnum {
    
    /// "stereoLayoutUnspecified"
    #[serde(rename="stereoLayoutUnspecified")]
    StereoLayoutUnspecified,
    
    /// "mono"
    #[serde(rename="mono")]
    Mono,
    
    /// "leftRight"
    #[serde(rename="leftRight")]
    LeftRight,
    
    /// "topBottom"
    #[serde(rename="topBottom")]
    TopBottom,
}

impl AsRef<str> for LiveBroadcastContentDetailStereoLayoutEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailStereoLayoutEnum::StereoLayoutUnspecified => "stereoLayoutUnspecified",
            LiveBroadcastContentDetailStereoLayoutEnum::Mono => "mono",
            LiveBroadcastContentDetailStereoLayoutEnum::LeftRight => "leftRight",
            LiveBroadcastContentDetailStereoLayoutEnum::TopBottom => "topBottom",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastContentDetailStereoLayoutEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "stereoLayoutUnspecified" => Ok(LiveBroadcastContentDetailStereoLayoutEnum::StereoLayoutUnspecified),
           "mono" => Ok(LiveBroadcastContentDetailStereoLayoutEnum::Mono),
           "leftRight" => Ok(LiveBroadcastContentDetailStereoLayoutEnum::LeftRight),
           "topBottom" => Ok(LiveBroadcastContentDetailStereoLayoutEnum::TopBottom),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailStereoLayoutEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastStatusLifeCycleStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method.
pub enum LiveBroadcastStatusLifeCycleStatusEnum {
    

    /// No value or the value is unknown.
    ///
    /// "lifeCycleStatusUnspecified"
    #[serde(rename="lifeCycleStatusUnspecified")]
    LifeCycleStatusUnspecified,
    

    /// Incomplete settings, but otherwise valid
    ///
    /// "created"
    #[serde(rename="created")]
    Created,
    

    /// Complete settings
    ///
    /// "ready"
    #[serde(rename="ready")]
    Ready,
    

    /// Visible only to partner, may need special UI treatment
    ///
    /// "testing"
    #[serde(rename="testing")]
    Testing,
    

    /// Viper is recording; this means the "clock" is running
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The broadcast is finished.
    ///
    /// "complete"
    #[serde(rename="complete")]
    Complete,
    

    /// This broadcast was removed by admin action
    ///
    /// "revoked"
    #[serde(rename="revoked")]
    Revoked,
    

    /// Transition into TESTING has been requested
    ///
    /// "testStarting"
    #[serde(rename="testStarting")]
    TestStarting,
    

    /// Transition into LIVE has been requested
    ///
    /// "liveStarting"
    #[serde(rename="liveStarting")]
    LiveStarting,
}

impl AsRef<str> for LiveBroadcastStatusLifeCycleStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatusLifeCycleStatusEnum::LifeCycleStatusUnspecified => "lifeCycleStatusUnspecified",
            LiveBroadcastStatusLifeCycleStatusEnum::Created => "created",
            LiveBroadcastStatusLifeCycleStatusEnum::Ready => "ready",
            LiveBroadcastStatusLifeCycleStatusEnum::Testing => "testing",
            LiveBroadcastStatusLifeCycleStatusEnum::Live => "live",
            LiveBroadcastStatusLifeCycleStatusEnum::Complete => "complete",
            LiveBroadcastStatusLifeCycleStatusEnum::Revoked => "revoked",
            LiveBroadcastStatusLifeCycleStatusEnum::TestStarting => "testStarting",
            LiveBroadcastStatusLifeCycleStatusEnum::LiveStarting => "liveStarting",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastStatusLifeCycleStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "lifeCycleStatusUnspecified" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::LifeCycleStatusUnspecified),
           "created" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::Created),
           "ready" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::Ready),
           "testing" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::Testing),
           "live" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::Live),
           "complete" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::Complete),
           "revoked" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::Revoked),
           "testStarting" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::TestStarting),
           "liveStarting" => Ok(LiveBroadcastStatusLifeCycleStatusEnum::LiveStarting),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatusLifeCycleStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastStatusLiveBroadcastPriorityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Priority of the live broadcast event (internal state).
pub enum LiveBroadcastStatusLiveBroadcastPriorityEnum {
    
    /// "liveBroadcastPriorityUnspecified"
    #[serde(rename="liveBroadcastPriorityUnspecified")]
    LiveBroadcastPriorityUnspecified,
    

    /// Low priority broadcast: for low view count HoAs or other low priority broadcasts.
    ///
    /// "low"
    #[serde(rename="low")]
    Low,
    

    /// Normal priority broadcast: for regular HoAs and broadcasts.
    ///
    /// "normal"
    #[serde(rename="normal")]
    Normal,
    

    /// High priority broadcast: for high profile HoAs, like PixelCorp ones.
    ///
    /// "high"
    #[serde(rename="high")]
    High,
}

impl AsRef<str> for LiveBroadcastStatusLiveBroadcastPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatusLiveBroadcastPriorityEnum::LiveBroadcastPriorityUnspecified => "liveBroadcastPriorityUnspecified",
            LiveBroadcastStatusLiveBroadcastPriorityEnum::Low => "low",
            LiveBroadcastStatusLiveBroadcastPriorityEnum::Normal => "normal",
            LiveBroadcastStatusLiveBroadcastPriorityEnum::High => "high",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastStatusLiveBroadcastPriorityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "liveBroadcastPriorityUnspecified" => Ok(LiveBroadcastStatusLiveBroadcastPriorityEnum::LiveBroadcastPriorityUnspecified),
           "low" => Ok(LiveBroadcastStatusLiveBroadcastPriorityEnum::Low),
           "normal" => Ok(LiveBroadcastStatusLiveBroadcastPriorityEnum::Normal),
           "high" => Ok(LiveBroadcastStatusLiveBroadcastPriorityEnum::High),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatusLiveBroadcastPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastStatusPrivacyStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
pub enum LiveBroadcastStatusPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for LiveBroadcastStatusPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatusPrivacyStatusEnum::Public => "public",
            LiveBroadcastStatusPrivacyStatusEnum::Unlisted => "unlisted",
            LiveBroadcastStatusPrivacyStatusEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastStatusPrivacyStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(LiveBroadcastStatusPrivacyStatusEnum::Public),
           "unlisted" => Ok(LiveBroadcastStatusPrivacyStatusEnum::Unlisted),
           "private" => Ok(LiveBroadcastStatusPrivacyStatusEnum::Private),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatusPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastStatusRecordingStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The broadcast's recording status.
pub enum LiveBroadcastStatusRecordingStatusEnum {
    

    /// No value or the value is unknown.
    ///
    /// "liveBroadcastRecordingStatusUnspecified"
    #[serde(rename="liveBroadcastRecordingStatusUnspecified")]
    LiveBroadcastRecordingStatusUnspecified,
    

    /// The recording has not yet been started.
    ///
    /// "notRecording"
    #[serde(rename="notRecording")]
    NotRecording,
    

    /// The recording is currently on.
    ///
    /// "recording"
    #[serde(rename="recording")]
    Recording,
    

    /// The recording is completed, and cannot be started again.
    ///
    /// "recorded"
    #[serde(rename="recorded")]
    Recorded,
}

impl AsRef<str> for LiveBroadcastStatusRecordingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatusRecordingStatusEnum::LiveBroadcastRecordingStatusUnspecified => "liveBroadcastRecordingStatusUnspecified",
            LiveBroadcastStatusRecordingStatusEnum::NotRecording => "notRecording",
            LiveBroadcastStatusRecordingStatusEnum::Recording => "recording",
            LiveBroadcastStatusRecordingStatusEnum::Recorded => "recorded",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastStatusRecordingStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "liveBroadcastRecordingStatusUnspecified" => Ok(LiveBroadcastStatusRecordingStatusEnum::LiveBroadcastRecordingStatusUnspecified),
           "notRecording" => Ok(LiveBroadcastStatusRecordingStatusEnum::NotRecording),
           "recording" => Ok(LiveBroadcastStatusRecordingStatusEnum::Recording),
           "recorded" => Ok(LiveBroadcastStatusRecordingStatusEnum::Recorded),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatusRecordingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveChatBanSnippetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of ban.
pub enum LiveChatBanSnippetTypeEnum {
    

    /// An invalid ban type.
    ///
    /// "liveChatBanTypeUnspecified"
    #[serde(rename="liveChatBanTypeUnspecified")]
    LiveChatBanTypeUnspecified,
    

    /// A permanent ban.
    ///
    /// "permanent"
    #[serde(rename="permanent")]
    Permanent,
    

    /// A temporary ban.
    ///
    /// "temporary"
    #[serde(rename="temporary")]
    Temporary,
}

impl AsRef<str> for LiveChatBanSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatBanSnippetTypeEnum::LiveChatBanTypeUnspecified => "liveChatBanTypeUnspecified",
            LiveChatBanSnippetTypeEnum::Permanent => "permanent",
            LiveChatBanSnippetTypeEnum::Temporary => "temporary",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveChatBanSnippetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "liveChatBanTypeUnspecified" => Ok(LiveChatBanSnippetTypeEnum::LiveChatBanTypeUnspecified),
           "permanent" => Ok(LiveChatBanSnippetTypeEnum::Permanent),
           "temporary" => Ok(LiveChatBanSnippetTypeEnum::Temporary),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatBanSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveChatMessageSnippetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of message, this will always be present, it determines the contents of the message as well as which fields will be present.
pub enum LiveChatMessageSnippetTypeEnum {
    
    /// "invalidType"
    #[serde(rename="invalidType")]
    InvalidType,
    
    /// "textMessageEvent"
    #[serde(rename="textMessageEvent")]
    TextMessageEvent,
    
    /// "tombstone"
    #[serde(rename="tombstone")]
    Tombstone,
    
    /// "fanFundingEvent"
    #[serde(rename="fanFundingEvent")]
    FanFundingEvent,
    
    /// "chatEndedEvent"
    #[serde(rename="chatEndedEvent")]
    ChatEndedEvent,
    
    /// "sponsorOnlyModeStartedEvent"
    #[serde(rename="sponsorOnlyModeStartedEvent")]
    SponsorOnlyModeStartedEvent,
    
    /// "sponsorOnlyModeEndedEvent"
    #[serde(rename="sponsorOnlyModeEndedEvent")]
    SponsorOnlyModeEndedEvent,
    
    /// "newSponsorEvent"
    #[serde(rename="newSponsorEvent")]
    NewSponsorEvent,
    
    /// "memberMilestoneChatEvent"
    #[serde(rename="memberMilestoneChatEvent")]
    MemberMilestoneChatEvent,
    
    /// "membershipGiftingEvent"
    #[serde(rename="membershipGiftingEvent")]
    MembershipGiftingEvent,
    
    /// "giftMembershipReceivedEvent"
    #[serde(rename="giftMembershipReceivedEvent")]
    GiftMembershipReceivedEvent,
    
    /// "messageDeletedEvent"
    #[serde(rename="messageDeletedEvent")]
    MessageDeletedEvent,
    
    /// "messageRetractedEvent"
    #[serde(rename="messageRetractedEvent")]
    MessageRetractedEvent,
    
    /// "userBannedEvent"
    #[serde(rename="userBannedEvent")]
    UserBannedEvent,
    
    /// "superChatEvent"
    #[serde(rename="superChatEvent")]
    SuperChatEvent,
    
    /// "superStickerEvent"
    #[serde(rename="superStickerEvent")]
    SuperStickerEvent,
    
    /// "pollEvent"
    #[serde(rename="pollEvent")]
    PollEvent,
}

impl AsRef<str> for LiveChatMessageSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatMessageSnippetTypeEnum::InvalidType => "invalidType",
            LiveChatMessageSnippetTypeEnum::TextMessageEvent => "textMessageEvent",
            LiveChatMessageSnippetTypeEnum::Tombstone => "tombstone",
            LiveChatMessageSnippetTypeEnum::FanFundingEvent => "fanFundingEvent",
            LiveChatMessageSnippetTypeEnum::ChatEndedEvent => "chatEndedEvent",
            LiveChatMessageSnippetTypeEnum::SponsorOnlyModeStartedEvent => "sponsorOnlyModeStartedEvent",
            LiveChatMessageSnippetTypeEnum::SponsorOnlyModeEndedEvent => "sponsorOnlyModeEndedEvent",
            LiveChatMessageSnippetTypeEnum::NewSponsorEvent => "newSponsorEvent",
            LiveChatMessageSnippetTypeEnum::MemberMilestoneChatEvent => "memberMilestoneChatEvent",
            LiveChatMessageSnippetTypeEnum::MembershipGiftingEvent => "membershipGiftingEvent",
            LiveChatMessageSnippetTypeEnum::GiftMembershipReceivedEvent => "giftMembershipReceivedEvent",
            LiveChatMessageSnippetTypeEnum::MessageDeletedEvent => "messageDeletedEvent",
            LiveChatMessageSnippetTypeEnum::MessageRetractedEvent => "messageRetractedEvent",
            LiveChatMessageSnippetTypeEnum::UserBannedEvent => "userBannedEvent",
            LiveChatMessageSnippetTypeEnum::SuperChatEvent => "superChatEvent",
            LiveChatMessageSnippetTypeEnum::SuperStickerEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::PollEvent => "pollEvent",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveChatMessageSnippetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "invalidType" => Ok(LiveChatMessageSnippetTypeEnum::InvalidType),
           "textMessageEvent" => Ok(LiveChatMessageSnippetTypeEnum::TextMessageEvent),
           "tombstone" => Ok(LiveChatMessageSnippetTypeEnum::Tombstone),
           "fanFundingEvent" => Ok(LiveChatMessageSnippetTypeEnum::FanFundingEvent),
           "chatEndedEvent" => Ok(LiveChatMessageSnippetTypeEnum::ChatEndedEvent),
           "sponsorOnlyModeStartedEvent" => Ok(LiveChatMessageSnippetTypeEnum::SponsorOnlyModeStartedEvent),
           "sponsorOnlyModeEndedEvent" => Ok(LiveChatMessageSnippetTypeEnum::SponsorOnlyModeEndedEvent),
           "newSponsorEvent" => Ok(LiveChatMessageSnippetTypeEnum::NewSponsorEvent),
           "memberMilestoneChatEvent" => Ok(LiveChatMessageSnippetTypeEnum::MemberMilestoneChatEvent),
           "membershipGiftingEvent" => Ok(LiveChatMessageSnippetTypeEnum::MembershipGiftingEvent),
           "giftMembershipReceivedEvent" => Ok(LiveChatMessageSnippetTypeEnum::GiftMembershipReceivedEvent),
           "messageDeletedEvent" => Ok(LiveChatMessageSnippetTypeEnum::MessageDeletedEvent),
           "messageRetractedEvent" => Ok(LiveChatMessageSnippetTypeEnum::MessageRetractedEvent),
           "userBannedEvent" => Ok(LiveChatMessageSnippetTypeEnum::UserBannedEvent),
           "superChatEvent" => Ok(LiveChatMessageSnippetTypeEnum::SuperChatEvent),
           "superStickerEvent" => Ok(LiveChatMessageSnippetTypeEnum::SuperStickerEvent),
           "pollEvent" => Ok(LiveChatMessageSnippetTypeEnum::PollEvent),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatMessageSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveChatPollDetailStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiveChatPollDetailStatusEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "closed"
    #[serde(rename="closed")]
    Closed,
}

impl AsRef<str> for LiveChatPollDetailStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatPollDetailStatusEnum::Unknown => "unknown",
            LiveChatPollDetailStatusEnum::Active => "active",
            LiveChatPollDetailStatusEnum::Closed => "closed",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveChatPollDetailStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(LiveChatPollDetailStatusEnum::Unknown),
           "active" => Ok(LiveChatPollDetailStatusEnum::Active),
           "closed" => Ok(LiveChatPollDetailStatusEnum::Closed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatPollDetailStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveChatUserBannedMessageDetailBanTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of ban.
pub enum LiveChatUserBannedMessageDetailBanTypeEnum {
    
    /// "permanent"
    #[serde(rename="permanent")]
    Permanent,
    
    /// "temporary"
    #[serde(rename="temporary")]
    Temporary,
}

impl AsRef<str> for LiveChatUserBannedMessageDetailBanTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatUserBannedMessageDetailBanTypeEnum::Permanent => "permanent",
            LiveChatUserBannedMessageDetailBanTypeEnum::Temporary => "temporary",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveChatUserBannedMessageDetailBanTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "permanent" => Ok(LiveChatUserBannedMessageDetailBanTypeEnum::Permanent),
           "temporary" => Ok(LiveChatUserBannedMessageDetailBanTypeEnum::Temporary),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatUserBannedMessageDetailBanTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveStreamConfigurationIssueSeverityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How severe this issue is to the stream.
pub enum LiveStreamConfigurationIssueSeverityEnum {
    
    /// "info"
    #[serde(rename="info")]
    Info,
    
    /// "warning"
    #[serde(rename="warning")]
    Warning,
    
    /// "error"
    #[serde(rename="error")]
    Error,
}

impl AsRef<str> for LiveStreamConfigurationIssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamConfigurationIssueSeverityEnum::Info => "info",
            LiveStreamConfigurationIssueSeverityEnum::Warning => "warning",
            LiveStreamConfigurationIssueSeverityEnum::Error => "error",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveStreamConfigurationIssueSeverityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "info" => Ok(LiveStreamConfigurationIssueSeverityEnum::Info),
           "warning" => Ok(LiveStreamConfigurationIssueSeverityEnum::Warning),
           "error" => Ok(LiveStreamConfigurationIssueSeverityEnum::Error),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamConfigurationIssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveStreamConfigurationIssueTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The kind of error happening.
pub enum LiveStreamConfigurationIssueTypeEnum {
    
    /// "gopSizeOver"
    #[serde(rename="gopSizeOver")]
    GopSizeOver,
    
    /// "gopSizeLong"
    #[serde(rename="gopSizeLong")]
    GopSizeLong,
    
    /// "gopSizeShort"
    #[serde(rename="gopSizeShort")]
    GopSizeShort,
    
    /// "openGop"
    #[serde(rename="openGop")]
    OpenGop,
    
    /// "badContainer"
    #[serde(rename="badContainer")]
    BadContainer,
    
    /// "audioBitrateHigh"
    #[serde(rename="audioBitrateHigh")]
    AudioBitrateHigh,
    
    /// "audioBitrateLow"
    #[serde(rename="audioBitrateLow")]
    AudioBitrateLow,
    
    /// "audioSampleRate"
    #[serde(rename="audioSampleRate")]
    AudioSampleRate,
    
    /// "bitrateHigh"
    #[serde(rename="bitrateHigh")]
    BitrateHigh,
    
    /// "bitrateLow"
    #[serde(rename="bitrateLow")]
    BitrateLow,
    
    /// "audioCodec"
    #[serde(rename="audioCodec")]
    AudioCodec,
    
    /// "videoCodec"
    #[serde(rename="videoCodec")]
    VideoCodec,
    
    /// "noAudioStream"
    #[serde(rename="noAudioStream")]
    NoAudioStream,
    
    /// "noVideoStream"
    #[serde(rename="noVideoStream")]
    NoVideoStream,
    
    /// "multipleVideoStreams"
    #[serde(rename="multipleVideoStreams")]
    MultipleVideoStreams,
    
    /// "multipleAudioStreams"
    #[serde(rename="multipleAudioStreams")]
    MultipleAudioStreams,
    
    /// "audioTooManyChannels"
    #[serde(rename="audioTooManyChannels")]
    AudioTooManyChannels,
    
    /// "interlacedVideo"
    #[serde(rename="interlacedVideo")]
    InterlacedVideo,
    
    /// "frameRateHigh"
    #[serde(rename="frameRateHigh")]
    FrameRateHigh,
    
    /// "resolutionMismatch"
    #[serde(rename="resolutionMismatch")]
    ResolutionMismatch,
    
    /// "videoCodecMismatch"
    #[serde(rename="videoCodecMismatch")]
    VideoCodecMismatch,
    
    /// "videoInterlaceMismatch"
    #[serde(rename="videoInterlaceMismatch")]
    VideoInterlaceMismatch,
    
    /// "videoProfileMismatch"
    #[serde(rename="videoProfileMismatch")]
    VideoProfileMismatch,
    
    /// "videoBitrateMismatch"
    #[serde(rename="videoBitrateMismatch")]
    VideoBitrateMismatch,
    
    /// "framerateMismatch"
    #[serde(rename="framerateMismatch")]
    FramerateMismatch,
    
    /// "gopMismatch"
    #[serde(rename="gopMismatch")]
    GopMismatch,
    
    /// "audioSampleRateMismatch"
    #[serde(rename="audioSampleRateMismatch")]
    AudioSampleRateMismatch,
    
    /// "audioStereoMismatch"
    #[serde(rename="audioStereoMismatch")]
    AudioStereoMismatch,
    
    /// "audioCodecMismatch"
    #[serde(rename="audioCodecMismatch")]
    AudioCodecMismatch,
    
    /// "audioBitrateMismatch"
    #[serde(rename="audioBitrateMismatch")]
    AudioBitrateMismatch,
    
    /// "videoResolutionSuboptimal"
    #[serde(rename="videoResolutionSuboptimal")]
    VideoResolutionSuboptimal,
    
    /// "videoResolutionUnsupported"
    #[serde(rename="videoResolutionUnsupported")]
    VideoResolutionUnsupported,
    
    /// "videoIngestionStarved"
    #[serde(rename="videoIngestionStarved")]
    VideoIngestionStarved,
    
    /// "videoIngestionFasterThanRealtime"
    #[serde(rename="videoIngestionFasterThanRealtime")]
    VideoIngestionFasterThanRealtime,
}

impl AsRef<str> for LiveStreamConfigurationIssueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamConfigurationIssueTypeEnum::GopSizeOver => "gopSizeOver",
            LiveStreamConfigurationIssueTypeEnum::GopSizeLong => "gopSizeLong",
            LiveStreamConfigurationIssueTypeEnum::GopSizeShort => "gopSizeShort",
            LiveStreamConfigurationIssueTypeEnum::OpenGop => "openGop",
            LiveStreamConfigurationIssueTypeEnum::BadContainer => "badContainer",
            LiveStreamConfigurationIssueTypeEnum::AudioBitrateHigh => "audioBitrateHigh",
            LiveStreamConfigurationIssueTypeEnum::AudioBitrateLow => "audioBitrateLow",
            LiveStreamConfigurationIssueTypeEnum::AudioSampleRate => "audioSampleRate",
            LiveStreamConfigurationIssueTypeEnum::BitrateHigh => "bitrateHigh",
            LiveStreamConfigurationIssueTypeEnum::BitrateLow => "bitrateLow",
            LiveStreamConfigurationIssueTypeEnum::AudioCodec => "audioCodec",
            LiveStreamConfigurationIssueTypeEnum::VideoCodec => "videoCodec",
            LiveStreamConfigurationIssueTypeEnum::NoAudioStream => "noAudioStream",
            LiveStreamConfigurationIssueTypeEnum::NoVideoStream => "noVideoStream",
            LiveStreamConfigurationIssueTypeEnum::MultipleVideoStreams => "multipleVideoStreams",
            LiveStreamConfigurationIssueTypeEnum::MultipleAudioStreams => "multipleAudioStreams",
            LiveStreamConfigurationIssueTypeEnum::AudioTooManyChannels => "audioTooManyChannels",
            LiveStreamConfigurationIssueTypeEnum::InterlacedVideo => "interlacedVideo",
            LiveStreamConfigurationIssueTypeEnum::FrameRateHigh => "frameRateHigh",
            LiveStreamConfigurationIssueTypeEnum::ResolutionMismatch => "resolutionMismatch",
            LiveStreamConfigurationIssueTypeEnum::VideoCodecMismatch => "videoCodecMismatch",
            LiveStreamConfigurationIssueTypeEnum::VideoInterlaceMismatch => "videoInterlaceMismatch",
            LiveStreamConfigurationIssueTypeEnum::VideoProfileMismatch => "videoProfileMismatch",
            LiveStreamConfigurationIssueTypeEnum::VideoBitrateMismatch => "videoBitrateMismatch",
            LiveStreamConfigurationIssueTypeEnum::FramerateMismatch => "framerateMismatch",
            LiveStreamConfigurationIssueTypeEnum::GopMismatch => "gopMismatch",
            LiveStreamConfigurationIssueTypeEnum::AudioSampleRateMismatch => "audioSampleRateMismatch",
            LiveStreamConfigurationIssueTypeEnum::AudioStereoMismatch => "audioStereoMismatch",
            LiveStreamConfigurationIssueTypeEnum::AudioCodecMismatch => "audioCodecMismatch",
            LiveStreamConfigurationIssueTypeEnum::AudioBitrateMismatch => "audioBitrateMismatch",
            LiveStreamConfigurationIssueTypeEnum::VideoResolutionSuboptimal => "videoResolutionSuboptimal",
            LiveStreamConfigurationIssueTypeEnum::VideoResolutionUnsupported => "videoResolutionUnsupported",
            LiveStreamConfigurationIssueTypeEnum::VideoIngestionStarved => "videoIngestionStarved",
            LiveStreamConfigurationIssueTypeEnum::VideoIngestionFasterThanRealtime => "videoIngestionFasterThanRealtime",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveStreamConfigurationIssueTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "gopSizeOver" => Ok(LiveStreamConfigurationIssueTypeEnum::GopSizeOver),
           "gopSizeLong" => Ok(LiveStreamConfigurationIssueTypeEnum::GopSizeLong),
           "gopSizeShort" => Ok(LiveStreamConfigurationIssueTypeEnum::GopSizeShort),
           "openGop" => Ok(LiveStreamConfigurationIssueTypeEnum::OpenGop),
           "badContainer" => Ok(LiveStreamConfigurationIssueTypeEnum::BadContainer),
           "audioBitrateHigh" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioBitrateHigh),
           "audioBitrateLow" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioBitrateLow),
           "audioSampleRate" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioSampleRate),
           "bitrateHigh" => Ok(LiveStreamConfigurationIssueTypeEnum::BitrateHigh),
           "bitrateLow" => Ok(LiveStreamConfigurationIssueTypeEnum::BitrateLow),
           "audioCodec" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioCodec),
           "videoCodec" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoCodec),
           "noAudioStream" => Ok(LiveStreamConfigurationIssueTypeEnum::NoAudioStream),
           "noVideoStream" => Ok(LiveStreamConfigurationIssueTypeEnum::NoVideoStream),
           "multipleVideoStreams" => Ok(LiveStreamConfigurationIssueTypeEnum::MultipleVideoStreams),
           "multipleAudioStreams" => Ok(LiveStreamConfigurationIssueTypeEnum::MultipleAudioStreams),
           "audioTooManyChannels" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioTooManyChannels),
           "interlacedVideo" => Ok(LiveStreamConfigurationIssueTypeEnum::InterlacedVideo),
           "frameRateHigh" => Ok(LiveStreamConfigurationIssueTypeEnum::FrameRateHigh),
           "resolutionMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::ResolutionMismatch),
           "videoCodecMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoCodecMismatch),
           "videoInterlaceMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoInterlaceMismatch),
           "videoProfileMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoProfileMismatch),
           "videoBitrateMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoBitrateMismatch),
           "framerateMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::FramerateMismatch),
           "gopMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::GopMismatch),
           "audioSampleRateMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioSampleRateMismatch),
           "audioStereoMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioStereoMismatch),
           "audioCodecMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioCodecMismatch),
           "audioBitrateMismatch" => Ok(LiveStreamConfigurationIssueTypeEnum::AudioBitrateMismatch),
           "videoResolutionSuboptimal" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoResolutionSuboptimal),
           "videoResolutionUnsupported" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoResolutionUnsupported),
           "videoIngestionStarved" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoIngestionStarved),
           "videoIngestionFasterThanRealtime" => Ok(LiveStreamConfigurationIssueTypeEnum::VideoIngestionFasterThanRealtime),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamConfigurationIssueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveStreamHealthStatusStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status code of this stream
pub enum LiveStreamHealthStatusStatusEnum {
    
    /// "good"
    #[serde(rename="good")]
    Good,
    
    /// "ok"
    #[serde(rename="ok")]
    Ok,
    
    /// "bad"
    #[serde(rename="bad")]
    Bad,
    
    /// "noData"
    #[serde(rename="noData")]
    NoData,
    
    /// "revoked"
    #[serde(rename="revoked")]
    Revoked,
}

impl AsRef<str> for LiveStreamHealthStatusStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamHealthStatusStatusEnum::Good => "good",
            LiveStreamHealthStatusStatusEnum::Ok => "ok",
            LiveStreamHealthStatusStatusEnum::Bad => "bad",
            LiveStreamHealthStatusStatusEnum::NoData => "noData",
            LiveStreamHealthStatusStatusEnum::Revoked => "revoked",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveStreamHealthStatusStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "good" => Ok(LiveStreamHealthStatusStatusEnum::Good),
           "ok" => Ok(LiveStreamHealthStatusStatusEnum::Ok),
           "bad" => Ok(LiveStreamHealthStatusStatusEnum::Bad),
           "noData" => Ok(LiveStreamHealthStatusStatusEnum::NoData),
           "revoked" => Ok(LiveStreamHealthStatusStatusEnum::Revoked),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamHealthStatusStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveStreamStatusStreamStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiveStreamStatusStreamStatusEnum {
    
    /// "created"
    #[serde(rename="created")]
    Created,
    
    /// "ready"
    #[serde(rename="ready")]
    Ready,
    
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
    
    /// "error"
    #[serde(rename="error")]
    Error,
}

impl AsRef<str> for LiveStreamStatusStreamStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamStatusStreamStatusEnum::Created => "created",
            LiveStreamStatusStreamStatusEnum::Ready => "ready",
            LiveStreamStatusStreamStatusEnum::Active => "active",
            LiveStreamStatusStreamStatusEnum::Inactive => "inactive",
            LiveStreamStatusStreamStatusEnum::Error => "error",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveStreamStatusStreamStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "created" => Ok(LiveStreamStatusStreamStatusEnum::Created),
           "ready" => Ok(LiveStreamStatusStreamStatusEnum::Ready),
           "active" => Ok(LiveStreamStatusStreamStatusEnum::Active),
           "inactive" => Ok(LiveStreamStatusStreamStatusEnum::Inactive),
           "error" => Ok(LiveStreamStatusStreamStatusEnum::Error),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamStatusStreamStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlaylistImageSnippetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The image type.
pub enum PlaylistImageSnippetTypeEnum {
    

    /// The main image that will be used for this playlist.
    ///
    /// "hero"
    #[serde(rename="hero")]
    Hero,
}

impl AsRef<str> for PlaylistImageSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaylistImageSnippetTypeEnum::Hero => "hero",
        }
    }
}

impl std::convert::TryFrom< &str> for PlaylistImageSnippetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "hero" => Ok(PlaylistImageSnippetTypeEnum::Hero),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaylistImageSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlaylistItemStatusPrivacyStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This resource's privacy status.
pub enum PlaylistItemStatusPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for PlaylistItemStatusPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaylistItemStatusPrivacyStatusEnum::Public => "public",
            PlaylistItemStatusPrivacyStatusEnum::Unlisted => "unlisted",
            PlaylistItemStatusPrivacyStatusEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for PlaylistItemStatusPrivacyStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(PlaylistItemStatusPrivacyStatusEnum::Public),
           "unlisted" => Ok(PlaylistItemStatusPrivacyStatusEnum::Unlisted),
           "private" => Ok(PlaylistItemStatusPrivacyStatusEnum::Private),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaylistItemStatusPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlaylistStatusPrivacyStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The playlist's privacy status.
pub enum PlaylistStatusPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for PlaylistStatusPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaylistStatusPrivacyStatusEnum::Public => "public",
            PlaylistStatusPrivacyStatusEnum::Unlisted => "unlisted",
            PlaylistStatusPrivacyStatusEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for PlaylistStatusPrivacyStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(PlaylistStatusPrivacyStatusEnum::Public),
           "unlisted" => Ok(PlaylistStatusPrivacyStatusEnum::Unlisted),
           "private" => Ok(PlaylistStatusPrivacyStatusEnum::Private),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaylistStatusPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchResultSnippetLiveBroadcastContentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's "none" if there is not any upcoming/active live broadcasts.
pub enum SearchResultSnippetLiveBroadcastContentEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The live broadcast is upcoming.
    ///
    /// "upcoming"
    #[serde(rename="upcoming")]
    Upcoming,
    

    /// The live broadcast is active.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The live broadcast has been completed.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for SearchResultSnippetLiveBroadcastContentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchResultSnippetLiveBroadcastContentEnum::None => "none",
            SearchResultSnippetLiveBroadcastContentEnum::Upcoming => "upcoming",
            SearchResultSnippetLiveBroadcastContentEnum::Live => "live",
            SearchResultSnippetLiveBroadcastContentEnum::Completed => "completed",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchResultSnippetLiveBroadcastContentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(SearchResultSnippetLiveBroadcastContentEnum::None),
           "upcoming" => Ok(SearchResultSnippetLiveBroadcastContentEnum::Upcoming),
           "live" => Ok(SearchResultSnippetLiveBroadcastContentEnum::Live),
           "completed" => Ok(SearchResultSnippetLiveBroadcastContentEnum::Completed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchResultSnippetLiveBroadcastContentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionContentDetailActivityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of activity this subscription is for (only uploads, everything).
pub enum SubscriptionContentDetailActivityTypeEnum {
    
    /// "subscriptionActivityTypeUnspecified"
    #[serde(rename="subscriptionActivityTypeUnspecified")]
    SubscriptionActivityTypeUnspecified,
    
    /// "all"
    #[serde(rename="all")]
    All,
    
    /// "uploads"
    #[serde(rename="uploads")]
    Uploads,
}

impl AsRef<str> for SubscriptionContentDetailActivityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionContentDetailActivityTypeEnum::SubscriptionActivityTypeUnspecified => "subscriptionActivityTypeUnspecified",
            SubscriptionContentDetailActivityTypeEnum::All => "all",
            SubscriptionContentDetailActivityTypeEnum::Uploads => "uploads",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionContentDetailActivityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "subscriptionActivityTypeUnspecified" => Ok(SubscriptionContentDetailActivityTypeEnum::SubscriptionActivityTypeUnspecified),
           "all" => Ok(SubscriptionContentDetailActivityTypeEnum::All),
           "uploads" => Ok(SubscriptionContentDetailActivityTypeEnum::Uploads),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionContentDetailActivityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThirdPartyLinkSnippetTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the link named after the entities that are being linked.
pub enum ThirdPartyLinkSnippetTypeEnum {
    
    /// "linkUnspecified"
    #[serde(rename="linkUnspecified")]
    LinkUnspecified,
    

    /// A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube.
    ///
    /// "channelToStoreLink"
    #[serde(rename="channelToStoreLink")]
    ChannelToStoreLink,
}

impl AsRef<str> for ThirdPartyLinkSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyLinkSnippetTypeEnum::LinkUnspecified => "linkUnspecified",
            ThirdPartyLinkSnippetTypeEnum::ChannelToStoreLink => "channelToStoreLink",
        }
    }
}

impl std::convert::TryFrom< &str> for ThirdPartyLinkSnippetTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "linkUnspecified" => Ok(ThirdPartyLinkSnippetTypeEnum::LinkUnspecified),
           "channelToStoreLink" => Ok(ThirdPartyLinkSnippetTypeEnum::ChannelToStoreLink),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyLinkSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ThirdPartyLinkStatusLinkStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ThirdPartyLinkStatusLinkStatusEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "failed"
    #[serde(rename="failed")]
    Failed,
    
    /// "pending"
    #[serde(rename="pending")]
    Pending,
    
    /// "linked"
    #[serde(rename="linked")]
    Linked,
}

impl AsRef<str> for ThirdPartyLinkStatusLinkStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyLinkStatusLinkStatusEnum::Unknown => "unknown",
            ThirdPartyLinkStatusLinkStatusEnum::Failed => "failed",
            ThirdPartyLinkStatusLinkStatusEnum::Pending => "pending",
            ThirdPartyLinkStatusLinkStatusEnum::Linked => "linked",
        }
    }
}

impl std::convert::TryFrom< &str> for ThirdPartyLinkStatusLinkStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknown" => Ok(ThirdPartyLinkStatusLinkStatusEnum::Unknown),
           "failed" => Ok(ThirdPartyLinkStatusLinkStatusEnum::Failed),
           "pending" => Ok(ThirdPartyLinkStatusLinkStatusEnum::Pending),
           "linked" => Ok(ThirdPartyLinkStatusLinkStatusEnum::Linked),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyLinkStatusLinkStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoAgeGatingVideoGameRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Video game rating, if any.
pub enum VideoAgeGatingVideoGameRatingEnum {
    
    /// "anyone"
    #[serde(rename="anyone")]
    Anyone,
    
    /// "m15Plus"
    #[serde(rename="m15Plus")]
    M15Plus,
    
    /// "m16Plus"
    #[serde(rename="m16Plus")]
    M16Plus,
    
    /// "m17Plus"
    #[serde(rename="m17Plus")]
    M17Plus,
}

impl AsRef<str> for VideoAgeGatingVideoGameRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoAgeGatingVideoGameRatingEnum::Anyone => "anyone",
            VideoAgeGatingVideoGameRatingEnum::M15Plus => "m15Plus",
            VideoAgeGatingVideoGameRatingEnum::M16Plus => "m16Plus",
            VideoAgeGatingVideoGameRatingEnum::M17Plus => "m17Plus",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoAgeGatingVideoGameRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "anyone" => Ok(VideoAgeGatingVideoGameRatingEnum::Anyone),
           "m15Plus" => Ok(VideoAgeGatingVideoGameRatingEnum::M15Plus),
           "m16Plus" => Ok(VideoAgeGatingVideoGameRatingEnum::M16Plus),
           "m17Plus" => Ok(VideoAgeGatingVideoGameRatingEnum::M17Plus),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoAgeGatingVideoGameRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoContentDetailCaptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value of captions indicates whether the video has captions or not.
pub enum VideoContentDetailCaptionEnum {
    
    /// "true"
    #[serde(rename="true")]
    True,
    
    /// "false"
    #[serde(rename="false")]
    False,
}

impl AsRef<str> for VideoContentDetailCaptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoContentDetailCaptionEnum::True => "true",
            VideoContentDetailCaptionEnum::False => "false",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoContentDetailCaptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "true" => Ok(VideoContentDetailCaptionEnum::True),
           "false" => Ok(VideoContentDetailCaptionEnum::False),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoContentDetailCaptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoContentDetailDefinitionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value of definition indicates whether the video is available in high definition or only in standard definition.
pub enum VideoContentDetailDefinitionEnum {
    

    /// sd
    ///
    /// "sd"
    #[serde(rename="sd")]
    Sd,
    

    /// hd
    ///
    /// "hd"
    #[serde(rename="hd")]
    Hd,
}

impl AsRef<str> for VideoContentDetailDefinitionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoContentDetailDefinitionEnum::Sd => "sd",
            VideoContentDetailDefinitionEnum::Hd => "hd",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoContentDetailDefinitionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "sd" => Ok(VideoContentDetailDefinitionEnum::Sd),
           "hd" => Ok(VideoContentDetailDefinitionEnum::Hd),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoContentDetailDefinitionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoContentDetailProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the projection format of the video.
pub enum VideoContentDetailProjectionEnum {
    
    /// "rectangular"
    #[serde(rename="rectangular")]
    Rectangular,
    
    /// "360"
    #[serde(rename="360")]
    _360,
}

impl AsRef<str> for VideoContentDetailProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoContentDetailProjectionEnum::Rectangular => "rectangular",
            VideoContentDetailProjectionEnum::_360 => "360",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoContentDetailProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "rectangular" => Ok(VideoContentDetailProjectionEnum::Rectangular),
           "360" => Ok(VideoContentDetailProjectionEnum::_360),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoContentDetailProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoFileDetailFileTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.
pub enum VideoFileDetailFileTypeEnum {
    

    /// Known video file (e.g., an MP4 file).
    ///
    /// "video"
    #[serde(rename="video")]
    Video,
    

    /// Audio only file (e.g., an MP3 file).
    ///
    /// "audio"
    #[serde(rename="audio")]
    Audio,
    

    /// Image file (e.g., a JPEG image).
    ///
    /// "image"
    #[serde(rename="image")]
    Image,
    

    /// Archive file (e.g., a ZIP archive).
    ///
    /// "archive"
    #[serde(rename="archive")]
    Archive,
    

    /// Document or text file (e.g., MS Word document).
    ///
    /// "document"
    #[serde(rename="document")]
    Document,
    

    /// Movie project file (e.g., Microsoft Windows Movie Maker project).
    ///
    /// "project"
    #[serde(rename="project")]
    Project,
    

    /// Other non-video file type.
    ///
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for VideoFileDetailFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoFileDetailFileTypeEnum::Video => "video",
            VideoFileDetailFileTypeEnum::Audio => "audio",
            VideoFileDetailFileTypeEnum::Image => "image",
            VideoFileDetailFileTypeEnum::Archive => "archive",
            VideoFileDetailFileTypeEnum::Document => "document",
            VideoFileDetailFileTypeEnum::Project => "project",
            VideoFileDetailFileTypeEnum::Other => "other",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoFileDetailFileTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "video" => Ok(VideoFileDetailFileTypeEnum::Video),
           "audio" => Ok(VideoFileDetailFileTypeEnum::Audio),
           "image" => Ok(VideoFileDetailFileTypeEnum::Image),
           "archive" => Ok(VideoFileDetailFileTypeEnum::Archive),
           "document" => Ok(VideoFileDetailFileTypeEnum::Document),
           "project" => Ok(VideoFileDetailFileTypeEnum::Project),
           "other" => Ok(VideoFileDetailFileTypeEnum::Other),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoFileDetailFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoFileDetailsVideoStreamRotationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The amount that YouTube needs to rotate the original source content to properly display the video.
pub enum VideoFileDetailsVideoStreamRotationEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    
    /// "clockwise"
    #[serde(rename="clockwise")]
    Clockwise,
    
    /// "upsideDown"
    #[serde(rename="upsideDown")]
    UpsideDown,
    
    /// "counterClockwise"
    #[serde(rename="counterClockwise")]
    CounterClockwise,
    
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for VideoFileDetailsVideoStreamRotationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoFileDetailsVideoStreamRotationEnum::None => "none",
            VideoFileDetailsVideoStreamRotationEnum::Clockwise => "clockwise",
            VideoFileDetailsVideoStreamRotationEnum::UpsideDown => "upsideDown",
            VideoFileDetailsVideoStreamRotationEnum::CounterClockwise => "counterClockwise",
            VideoFileDetailsVideoStreamRotationEnum::Other => "other",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoFileDetailsVideoStreamRotationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(VideoFileDetailsVideoStreamRotationEnum::None),
           "clockwise" => Ok(VideoFileDetailsVideoStreamRotationEnum::Clockwise),
           "upsideDown" => Ok(VideoFileDetailsVideoStreamRotationEnum::UpsideDown),
           "counterClockwise" => Ok(VideoFileDetailsVideoStreamRotationEnum::CounterClockwise),
           "other" => Ok(VideoFileDetailsVideoStreamRotationEnum::Other),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoFileDetailsVideoStreamRotationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoProcessingDetailProcessingFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed.
pub enum VideoProcessingDetailProcessingFailureReasonEnum {
    
    /// "uploadFailed"
    #[serde(rename="uploadFailed")]
    UploadFailed,
    
    /// "transcodeFailed"
    #[serde(rename="transcodeFailed")]
    TranscodeFailed,
    
    /// "streamingFailed"
    #[serde(rename="streamingFailed")]
    StreamingFailed,
    
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for VideoProcessingDetailProcessingFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoProcessingDetailProcessingFailureReasonEnum::UploadFailed => "uploadFailed",
            VideoProcessingDetailProcessingFailureReasonEnum::TranscodeFailed => "transcodeFailed",
            VideoProcessingDetailProcessingFailureReasonEnum::StreamingFailed => "streamingFailed",
            VideoProcessingDetailProcessingFailureReasonEnum::Other => "other",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoProcessingDetailProcessingFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "uploadFailed" => Ok(VideoProcessingDetailProcessingFailureReasonEnum::UploadFailed),
           "transcodeFailed" => Ok(VideoProcessingDetailProcessingFailureReasonEnum::TranscodeFailed),
           "streamingFailed" => Ok(VideoProcessingDetailProcessingFailureReasonEnum::StreamingFailed),
           "other" => Ok(VideoProcessingDetailProcessingFailureReasonEnum::Other),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoProcessingDetailProcessingFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoProcessingDetailProcessingStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
pub enum VideoProcessingDetailProcessingStatusEnum {
    
    /// "processing"
    #[serde(rename="processing")]
    Processing,
    
    /// "succeeded"
    #[serde(rename="succeeded")]
    Succeeded,
    
    /// "failed"
    #[serde(rename="failed")]
    Failed,
    
    /// "terminated"
    #[serde(rename="terminated")]
    Terminated,
}

impl AsRef<str> for VideoProcessingDetailProcessingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoProcessingDetailProcessingStatusEnum::Processing => "processing",
            VideoProcessingDetailProcessingStatusEnum::Succeeded => "succeeded",
            VideoProcessingDetailProcessingStatusEnum::Failed => "failed",
            VideoProcessingDetailProcessingStatusEnum::Terminated => "terminated",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoProcessingDetailProcessingStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "processing" => Ok(VideoProcessingDetailProcessingStatusEnum::Processing),
           "succeeded" => Ok(VideoProcessingDetailProcessingStatusEnum::Succeeded),
           "failed" => Ok(VideoProcessingDetailProcessingStatusEnum::Failed),
           "terminated" => Ok(VideoProcessingDetailProcessingStatusEnum::Terminated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoProcessingDetailProcessingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoRatingRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating of a video.
pub enum VideoRatingRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for VideoRatingRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoRatingRatingEnum::None => "none",
            VideoRatingRatingEnum::Like => "like",
            VideoRatingRatingEnum::Dislike => "dislike",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoRatingRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(VideoRatingRatingEnum::None),
           "like" => Ok(VideoRatingRatingEnum::Like),
           "dislike" => Ok(VideoRatingRatingEnum::Dislike),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoRatingRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoSnippetLiveBroadcastContentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast.
pub enum VideoSnippetLiveBroadcastContentEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The live broadcast is upcoming.
    ///
    /// "upcoming"
    #[serde(rename="upcoming")]
    Upcoming,
    

    /// The live broadcast is active.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The live broadcast has been completed.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for VideoSnippetLiveBroadcastContentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSnippetLiveBroadcastContentEnum::None => "none",
            VideoSnippetLiveBroadcastContentEnum::Upcoming => "upcoming",
            VideoSnippetLiveBroadcastContentEnum::Live => "live",
            VideoSnippetLiveBroadcastContentEnum::Completed => "completed",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoSnippetLiveBroadcastContentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(VideoSnippetLiveBroadcastContentEnum::None),
           "upcoming" => Ok(VideoSnippetLiveBroadcastContentEnum::Upcoming),
           "live" => Ok(VideoSnippetLiveBroadcastContentEnum::Live),
           "completed" => Ok(VideoSnippetLiveBroadcastContentEnum::Completed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSnippetLiveBroadcastContentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoStatusFailureReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
pub enum VideoStatusFailureReasonEnum {
    

    /// Unable to convert video content.
    ///
    /// "conversion"
    #[serde(rename="conversion")]
    Conversion,
    

    /// Invalid file format.
    ///
    /// "invalidFile"
    #[serde(rename="invalidFile")]
    InvalidFile,
    

    /// Empty file.
    ///
    /// "emptyFile"
    #[serde(rename="emptyFile")]
    EmptyFile,
    

    /// File was too small.
    ///
    /// "tooSmall"
    #[serde(rename="tooSmall")]
    TooSmall,
    

    /// Unsupported codec.
    ///
    /// "codec"
    #[serde(rename="codec")]
    Codec,
    

    /// Upload wasn't finished.
    ///
    /// "uploadAborted"
    #[serde(rename="uploadAborted")]
    UploadAborted,
}

impl AsRef<str> for VideoStatusFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatusFailureReasonEnum::Conversion => "conversion",
            VideoStatusFailureReasonEnum::InvalidFile => "invalidFile",
            VideoStatusFailureReasonEnum::EmptyFile => "emptyFile",
            VideoStatusFailureReasonEnum::TooSmall => "tooSmall",
            VideoStatusFailureReasonEnum::Codec => "codec",
            VideoStatusFailureReasonEnum::UploadAborted => "uploadAborted",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoStatusFailureReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "conversion" => Ok(VideoStatusFailureReasonEnum::Conversion),
           "invalidFile" => Ok(VideoStatusFailureReasonEnum::InvalidFile),
           "emptyFile" => Ok(VideoStatusFailureReasonEnum::EmptyFile),
           "tooSmall" => Ok(VideoStatusFailureReasonEnum::TooSmall),
           "codec" => Ok(VideoStatusFailureReasonEnum::Codec),
           "uploadAborted" => Ok(VideoStatusFailureReasonEnum::UploadAborted),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatusFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoStatusLicenseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's license. @mutable youtube.videos.insert youtube.videos.update
pub enum VideoStatusLicenseEnum {
    
    /// "youtube"
    #[serde(rename="youtube")]
    Youtube,
    
    /// "creativeCommon"
    #[serde(rename="creativeCommon")]
    CreativeCommon,
}

impl AsRef<str> for VideoStatusLicenseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatusLicenseEnum::Youtube => "youtube",
            VideoStatusLicenseEnum::CreativeCommon => "creativeCommon",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoStatusLicenseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "youtube" => Ok(VideoStatusLicenseEnum::Youtube),
           "creativeCommon" => Ok(VideoStatusLicenseEnum::CreativeCommon),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatusLicenseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoStatusPrivacyStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's privacy status.
pub enum VideoStatusPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for VideoStatusPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatusPrivacyStatusEnum::Public => "public",
            VideoStatusPrivacyStatusEnum::Unlisted => "unlisted",
            VideoStatusPrivacyStatusEnum::Private => "private",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoStatusPrivacyStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(VideoStatusPrivacyStatusEnum::Public),
           "unlisted" => Ok(VideoStatusPrivacyStatusEnum::Unlisted),
           "private" => Ok(VideoStatusPrivacyStatusEnum::Private),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatusPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoStatusRejectionReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
pub enum VideoStatusRejectionReasonEnum {
    

    /// Copyright infringement.
    ///
    /// "copyright"
    #[serde(rename="copyright")]
    Copyright,
    

    /// Inappropriate video content.
    ///
    /// "inappropriate"
    #[serde(rename="inappropriate")]
    Inappropriate,
    

    /// Duplicate upload in the same channel.
    ///
    /// "duplicate"
    #[serde(rename="duplicate")]
    Duplicate,
    

    /// Terms of use violation.
    ///
    /// "termsOfUse"
    #[serde(rename="termsOfUse")]
    TermsOfUse,
    

    /// Uploader account was suspended.
    ///
    /// "uploaderAccountSuspended"
    #[serde(rename="uploaderAccountSuspended")]
    UploaderAccountSuspended,
    

    /// Video duration was too long.
    ///
    /// "length"
    #[serde(rename="length")]
    Length,
    

    /// Blocked by content owner.
    ///
    /// "claim"
    #[serde(rename="claim")]
    Claim,
    

    /// Uploader closed his/her account.
    ///
    /// "uploaderAccountClosed"
    #[serde(rename="uploaderAccountClosed")]
    UploaderAccountClosed,
    

    /// Trademark infringement.
    ///
    /// "trademark"
    #[serde(rename="trademark")]
    Trademark,
    

    /// An unspecified legal reason.
    ///
    /// "legal"
    #[serde(rename="legal")]
    Legal,
}

impl AsRef<str> for VideoStatusRejectionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatusRejectionReasonEnum::Copyright => "copyright",
            VideoStatusRejectionReasonEnum::Inappropriate => "inappropriate",
            VideoStatusRejectionReasonEnum::Duplicate => "duplicate",
            VideoStatusRejectionReasonEnum::TermsOfUse => "termsOfUse",
            VideoStatusRejectionReasonEnum::UploaderAccountSuspended => "uploaderAccountSuspended",
            VideoStatusRejectionReasonEnum::Length => "length",
            VideoStatusRejectionReasonEnum::Claim => "claim",
            VideoStatusRejectionReasonEnum::UploaderAccountClosed => "uploaderAccountClosed",
            VideoStatusRejectionReasonEnum::Trademark => "trademark",
            VideoStatusRejectionReasonEnum::Legal => "legal",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoStatusRejectionReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "copyright" => Ok(VideoStatusRejectionReasonEnum::Copyright),
           "inappropriate" => Ok(VideoStatusRejectionReasonEnum::Inappropriate),
           "duplicate" => Ok(VideoStatusRejectionReasonEnum::Duplicate),
           "termsOfUse" => Ok(VideoStatusRejectionReasonEnum::TermsOfUse),
           "uploaderAccountSuspended" => Ok(VideoStatusRejectionReasonEnum::UploaderAccountSuspended),
           "length" => Ok(VideoStatusRejectionReasonEnum::Length),
           "claim" => Ok(VideoStatusRejectionReasonEnum::Claim),
           "uploaderAccountClosed" => Ok(VideoStatusRejectionReasonEnum::UploaderAccountClosed),
           "trademark" => Ok(VideoStatusRejectionReasonEnum::Trademark),
           "legal" => Ok(VideoStatusRejectionReasonEnum::Legal),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatusRejectionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoStatusUploadStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the uploaded video.
pub enum VideoStatusUploadStatusEnum {
    

    /// Video has been uploaded but not processed yet.
    ///
    /// "uploaded"
    #[serde(rename="uploaded")]
    Uploaded,
    

    /// Video has been successfully processed.
    ///
    /// "processed"
    #[serde(rename="processed")]
    Processed,
    

    /// Processing has failed. See FailureReason.
    ///
    /// "failed"
    #[serde(rename="failed")]
    Failed,
    

    /// Video has been rejected. See RejectionReason.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    

    /// Video has been deleted.
    ///
    /// "deleted"
    #[serde(rename="deleted")]
    Deleted,
}

impl AsRef<str> for VideoStatusUploadStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatusUploadStatusEnum::Uploaded => "uploaded",
            VideoStatusUploadStatusEnum::Processed => "processed",
            VideoStatusUploadStatusEnum::Failed => "failed",
            VideoStatusUploadStatusEnum::Rejected => "rejected",
            VideoStatusUploadStatusEnum::Deleted => "deleted",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoStatusUploadStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "uploaded" => Ok(VideoStatusUploadStatusEnum::Uploaded),
           "processed" => Ok(VideoStatusUploadStatusEnum::Processed),
           "failed" => Ok(VideoStatusUploadStatusEnum::Failed),
           "rejected" => Ok(VideoStatusUploadStatusEnum::Rejected),
           "deleted" => Ok(VideoStatusUploadStatusEnum::Deleted),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatusUploadStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoSuggestionEditorSuggestionsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of video editing operations that might improve the video quality or playback experience of the uploaded video.
pub enum VideoSuggestionEditorSuggestionsEnum {
    

    /// Picture brightness levels seem off and could be corrected.
    ///
    /// "videoAutoLevels"
    #[serde(rename="videoAutoLevels")]
    VideoAutoLevels,
    

    /// The video appears shaky and could be stabilized.
    ///
    /// "videoStabilize"
    #[serde(rename="videoStabilize")]
    VideoStabilize,
    

    /// Margins (mattes) detected around the picture could be cropped.
    ///
    /// "videoCrop"
    #[serde(rename="videoCrop")]
    VideoCrop,
    

    /// The audio track appears silent and could be swapped with a better quality one.
    ///
    /// "audioQuietAudioSwap"
    #[serde(rename="audioQuietAudioSwap")]
    AudioQuietAudioSwap,
}

impl AsRef<str> for VideoSuggestionEditorSuggestionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionEditorSuggestionsEnum::VideoAutoLevels => "videoAutoLevels",
            VideoSuggestionEditorSuggestionsEnum::VideoStabilize => "videoStabilize",
            VideoSuggestionEditorSuggestionsEnum::VideoCrop => "videoCrop",
            VideoSuggestionEditorSuggestionsEnum::AudioQuietAudioSwap => "audioQuietAudioSwap",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoSuggestionEditorSuggestionsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoAutoLevels" => Ok(VideoSuggestionEditorSuggestionsEnum::VideoAutoLevels),
           "videoStabilize" => Ok(VideoSuggestionEditorSuggestionsEnum::VideoStabilize),
           "videoCrop" => Ok(VideoSuggestionEditorSuggestionsEnum::VideoCrop),
           "audioQuietAudioSwap" => Ok(VideoSuggestionEditorSuggestionsEnum::AudioQuietAudioSwap),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionEditorSuggestionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoSuggestionProcessingErrorsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed.
pub enum VideoSuggestionProcessingErrorsEnum {
    

    /// File contains audio only (e.g., an MP3 file).
    ///
    /// "audioFile"
    #[serde(rename="audioFile")]
    AudioFile,
    

    /// Image file (e.g., a JPEG image).
    ///
    /// "imageFile"
    #[serde(rename="imageFile")]
    ImageFile,
    

    /// Movie project file (e.g., Microsoft Windows Movie Maker project).
    ///
    /// "projectFile"
    #[serde(rename="projectFile")]
    ProjectFile,
    

    /// Other non-video file.
    ///
    /// "notAVideoFile"
    #[serde(rename="notAVideoFile")]
    NotAVideoFile,
    

    /// Document or text file (e.g., MS Word document).
    ///
    /// "docFile"
    #[serde(rename="docFile")]
    DocFile,
    

    /// An archive file (e.g., a ZIP archive).
    ///
    /// "archiveFile"
    #[serde(rename="archiveFile")]
    ArchiveFile,
    

    /// Unsupported spatial audio layout type.
    ///
    /// "unsupportedSpatialAudioLayout"
    #[serde(rename="unsupportedSpatialAudioLayout")]
    UnsupportedSpatialAudioLayout,
}

impl AsRef<str> for VideoSuggestionProcessingErrorsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionProcessingErrorsEnum::AudioFile => "audioFile",
            VideoSuggestionProcessingErrorsEnum::ImageFile => "imageFile",
            VideoSuggestionProcessingErrorsEnum::ProjectFile => "projectFile",
            VideoSuggestionProcessingErrorsEnum::NotAVideoFile => "notAVideoFile",
            VideoSuggestionProcessingErrorsEnum::DocFile => "docFile",
            VideoSuggestionProcessingErrorsEnum::ArchiveFile => "archiveFile",
            VideoSuggestionProcessingErrorsEnum::UnsupportedSpatialAudioLayout => "unsupportedSpatialAudioLayout",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoSuggestionProcessingErrorsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "audioFile" => Ok(VideoSuggestionProcessingErrorsEnum::AudioFile),
           "imageFile" => Ok(VideoSuggestionProcessingErrorsEnum::ImageFile),
           "projectFile" => Ok(VideoSuggestionProcessingErrorsEnum::ProjectFile),
           "notAVideoFile" => Ok(VideoSuggestionProcessingErrorsEnum::NotAVideoFile),
           "docFile" => Ok(VideoSuggestionProcessingErrorsEnum::DocFile),
           "archiveFile" => Ok(VideoSuggestionProcessingErrorsEnum::ArchiveFile),
           "unsupportedSpatialAudioLayout" => Ok(VideoSuggestionProcessingErrorsEnum::UnsupportedSpatialAudioLayout),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionProcessingErrorsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoSuggestionProcessingHintsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of suggestions that may improve YouTube's ability to process the video.
pub enum VideoSuggestionProcessingHintsEnum {
    

    /// The MP4 file is not streamable, this will slow down the processing. MOOV atom was not found at the beginning of the file.
    ///
    /// "nonStreamableMov"
    #[serde(rename="nonStreamableMov")]
    NonStreamableMov,
    

    /// Probably a better quality version of the video exists. The video has wide screen aspect ratio, but is not an HD video.
    ///
    /// "sendBestQualityVideo"
    #[serde(rename="sendBestQualityVideo")]
    SendBestQualityVideo,
    

    /// Uploaded video is spherical video.
    ///
    /// "sphericalVideo"
    #[serde(rename="sphericalVideo")]
    SphericalVideo,
    

    /// Uploaded video has spatial audio.
    ///
    /// "spatialAudio"
    #[serde(rename="spatialAudio")]
    SpatialAudio,
    

    /// Uploaded video is VR video.
    ///
    /// "vrVideo"
    #[serde(rename="vrVideo")]
    VrVideo,
    

    /// Uploaded video is HDR video.
    ///
    /// "hdrVideo"
    #[serde(rename="hdrVideo")]
    HdrVideo,
}

impl AsRef<str> for VideoSuggestionProcessingHintsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionProcessingHintsEnum::NonStreamableMov => "nonStreamableMov",
            VideoSuggestionProcessingHintsEnum::SendBestQualityVideo => "sendBestQualityVideo",
            VideoSuggestionProcessingHintsEnum::SphericalVideo => "sphericalVideo",
            VideoSuggestionProcessingHintsEnum::SpatialAudio => "spatialAudio",
            VideoSuggestionProcessingHintsEnum::VrVideo => "vrVideo",
            VideoSuggestionProcessingHintsEnum::HdrVideo => "hdrVideo",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoSuggestionProcessingHintsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "nonStreamableMov" => Ok(VideoSuggestionProcessingHintsEnum::NonStreamableMov),
           "sendBestQualityVideo" => Ok(VideoSuggestionProcessingHintsEnum::SendBestQualityVideo),
           "sphericalVideo" => Ok(VideoSuggestionProcessingHintsEnum::SphericalVideo),
           "spatialAudio" => Ok(VideoSuggestionProcessingHintsEnum::SpatialAudio),
           "vrVideo" => Ok(VideoSuggestionProcessingHintsEnum::VrVideo),
           "hdrVideo" => Ok(VideoSuggestionProcessingHintsEnum::HdrVideo),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionProcessingHintsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoSuggestionProcessingWarningsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.
pub enum VideoSuggestionProcessingWarningsEnum {
    

    /// Unrecognized file format, transcoding is likely to fail.
    ///
    /// "unknownContainer"
    #[serde(rename="unknownContainer")]
    UnknownContainer,
    

    /// Unrecognized video codec, transcoding is likely to fail.
    ///
    /// "unknownVideoCodec"
    #[serde(rename="unknownVideoCodec")]
    UnknownVideoCodec,
    

    /// Unrecognized audio codec, transcoding is likely to fail.
    ///
    /// "unknownAudioCodec"
    #[serde(rename="unknownAudioCodec")]
    UnknownAudioCodec,
    

    /// Conflicting container and stream resolutions.
    ///
    /// "inconsistentResolution"
    #[serde(rename="inconsistentResolution")]
    InconsistentResolution,
    

    /// Edit lists are not currently supported.
    ///
    /// "hasEditlist"
    #[serde(rename="hasEditlist")]
    HasEditlist,
    

    /// Video codec that is known to cause problems was used.
    ///
    /// "problematicVideoCodec"
    #[serde(rename="problematicVideoCodec")]
    ProblematicVideoCodec,
    

    /// Audio codec that is known to cause problems was used.
    ///
    /// "problematicAudioCodec"
    #[serde(rename="problematicAudioCodec")]
    ProblematicAudioCodec,
    

    /// Unsupported VR video stereo mode.
    ///
    /// "unsupportedVrStereoMode"
    #[serde(rename="unsupportedVrStereoMode")]
    UnsupportedVrStereoMode,
    

    /// Unsupported spherical video projection type.
    ///
    /// "unsupportedSphericalProjectionType"
    #[serde(rename="unsupportedSphericalProjectionType")]
    UnsupportedSphericalProjectionType,
    

    /// Unsupported HDR pixel format.
    ///
    /// "unsupportedHdrPixelFormat"
    #[serde(rename="unsupportedHdrPixelFormat")]
    UnsupportedHdrPixelFormat,
    

    /// Unspecified HDR color metadata.
    ///
    /// "unsupportedHdrColorMetadata"
    #[serde(rename="unsupportedHdrColorMetadata")]
    UnsupportedHdrColorMetadata,
    

    /// Problematic HDR lookup table attached.
    ///
    /// "problematicHdrLookupTable"
    #[serde(rename="problematicHdrLookupTable")]
    ProblematicHdrLookupTable,
}

impl AsRef<str> for VideoSuggestionProcessingWarningsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionProcessingWarningsEnum::UnknownContainer => "unknownContainer",
            VideoSuggestionProcessingWarningsEnum::UnknownVideoCodec => "unknownVideoCodec",
            VideoSuggestionProcessingWarningsEnum::UnknownAudioCodec => "unknownAudioCodec",
            VideoSuggestionProcessingWarningsEnum::InconsistentResolution => "inconsistentResolution",
            VideoSuggestionProcessingWarningsEnum::HasEditlist => "hasEditlist",
            VideoSuggestionProcessingWarningsEnum::ProblematicVideoCodec => "problematicVideoCodec",
            VideoSuggestionProcessingWarningsEnum::ProblematicAudioCodec => "problematicAudioCodec",
            VideoSuggestionProcessingWarningsEnum::UnsupportedVrStereoMode => "unsupportedVrStereoMode",
            VideoSuggestionProcessingWarningsEnum::UnsupportedSphericalProjectionType => "unsupportedSphericalProjectionType",
            VideoSuggestionProcessingWarningsEnum::UnsupportedHdrPixelFormat => "unsupportedHdrPixelFormat",
            VideoSuggestionProcessingWarningsEnum::UnsupportedHdrColorMetadata => "unsupportedHdrColorMetadata",
            VideoSuggestionProcessingWarningsEnum::ProblematicHdrLookupTable => "problematicHdrLookupTable",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoSuggestionProcessingWarningsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unknownContainer" => Ok(VideoSuggestionProcessingWarningsEnum::UnknownContainer),
           "unknownVideoCodec" => Ok(VideoSuggestionProcessingWarningsEnum::UnknownVideoCodec),
           "unknownAudioCodec" => Ok(VideoSuggestionProcessingWarningsEnum::UnknownAudioCodec),
           "inconsistentResolution" => Ok(VideoSuggestionProcessingWarningsEnum::InconsistentResolution),
           "hasEditlist" => Ok(VideoSuggestionProcessingWarningsEnum::HasEditlist),
           "problematicVideoCodec" => Ok(VideoSuggestionProcessingWarningsEnum::ProblematicVideoCodec),
           "problematicAudioCodec" => Ok(VideoSuggestionProcessingWarningsEnum::ProblematicAudioCodec),
           "unsupportedVrStereoMode" => Ok(VideoSuggestionProcessingWarningsEnum::UnsupportedVrStereoMode),
           "unsupportedSphericalProjectionType" => Ok(VideoSuggestionProcessingWarningsEnum::UnsupportedSphericalProjectionType),
           "unsupportedHdrPixelFormat" => Ok(VideoSuggestionProcessingWarningsEnum::UnsupportedHdrPixelFormat),
           "unsupportedHdrColorMetadata" => Ok(VideoSuggestionProcessingWarningsEnum::UnsupportedHdrColorMetadata),
           "problematicHdrLookupTable" => Ok(VideoSuggestionProcessingWarningsEnum::ProblematicHdrLookupTable),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionProcessingWarningsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommentThreadModerationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Limits the returned comment threads to those with the specified moderation status. Not compatible with the 'id' filter. Valid values: published, heldForReview, likelySpam.
pub enum CommentThreadModerationStatusEnum {
    

    /// The comment is available for public display.
    ///
    /// "published"
    #[serde(rename="published")]
    Published,
    

    /// The comment is awaiting review by a moderator.
    ///
    /// "heldForReview"
    #[serde(rename="heldForReview")]
    HeldForReview,
    
    /// "likelySpam"
    #[serde(rename="likelySpam")]
    LikelySpam,
    

    /// The comment is unfit for display.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
}

impl AsRef<str> for CommentThreadModerationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentThreadModerationStatusEnum::Published => "published",
            CommentThreadModerationStatusEnum::HeldForReview => "heldForReview",
            CommentThreadModerationStatusEnum::LikelySpam => "likelySpam",
            CommentThreadModerationStatusEnum::Rejected => "rejected",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentThreadModerationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "published" => Ok(CommentThreadModerationStatusEnum::Published),
           "heldForReview" => Ok(CommentThreadModerationStatusEnum::HeldForReview),
           "likelySpam" => Ok(CommentThreadModerationStatusEnum::LikelySpam),
           "rejected" => Ok(CommentThreadModerationStatusEnum::Rejected),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentThreadModerationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentThreadModerationStatusEnum {
    fn default() -> CommentThreadModerationStatusEnum {
        CommentThreadModerationStatusEnum::Published
    }
}

// endregion


// region CommentThreadOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommentThreadOrderEnum {
    
    /// "orderUnspecified"
    #[serde(rename="orderUnspecified")]
    OrderUnspecified,
    

    /// Order by time.
    ///
    /// "time"
    #[serde(rename="time")]
    Time,
    

    /// Order by relevance.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
}

impl AsRef<str> for CommentThreadOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentThreadOrderEnum::OrderUnspecified => "orderUnspecified",
            CommentThreadOrderEnum::Time => "time",
            CommentThreadOrderEnum::Relevance => "relevance",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentThreadOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "orderUnspecified" => Ok(CommentThreadOrderEnum::OrderUnspecified),
           "time" => Ok(CommentThreadOrderEnum::Time),
           "relevance" => Ok(CommentThreadOrderEnum::Relevance),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentThreadOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentThreadOrderEnum {
    fn default() -> CommentThreadOrderEnum {
        CommentThreadOrderEnum::Time
    }
}

// endregion


// region CommentThreadTextFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested text format for the returned comments.
pub enum CommentThreadTextFormatEnum {
    
    /// "textFormatUnspecified"
    #[serde(rename="textFormatUnspecified")]
    TextFormatUnspecified,
    

    /// Returns the comments in HTML format. This is the default value.
    ///
    /// "html"
    #[serde(rename="html")]
    Html,
    

    /// Returns the comments in plain text format.
    ///
    /// "plainText"
    #[serde(rename="plainText")]
    PlainText,
}

impl AsRef<str> for CommentThreadTextFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentThreadTextFormatEnum::TextFormatUnspecified => "textFormatUnspecified",
            CommentThreadTextFormatEnum::Html => "html",
            CommentThreadTextFormatEnum::PlainText => "plainText",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentThreadTextFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "textFormatUnspecified" => Ok(CommentThreadTextFormatEnum::TextFormatUnspecified),
           "html" => Ok(CommentThreadTextFormatEnum::Html),
           "plainText" => Ok(CommentThreadTextFormatEnum::PlainText),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentThreadTextFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentThreadTextFormatEnum {
    fn default() -> CommentThreadTextFormatEnum {
        CommentThreadTextFormatEnum::Html
    }
}

// endregion


// region CommentTextFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested text format for the returned comments.
pub enum CommentTextFormatEnum {
    
    /// "textFormatUnspecified"
    #[serde(rename="textFormatUnspecified")]
    TextFormatUnspecified,
    

    /// Returns the comments in HTML format. This is the default value.
    ///
    /// "html"
    #[serde(rename="html")]
    Html,
    

    /// Returns the comments in plain text format.
    ///
    /// "plainText"
    #[serde(rename="plainText")]
    PlainText,
}

impl AsRef<str> for CommentTextFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentTextFormatEnum::TextFormatUnspecified => "textFormatUnspecified",
            CommentTextFormatEnum::Html => "html",
            CommentTextFormatEnum::PlainText => "plainText",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentTextFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "textFormatUnspecified" => Ok(CommentTextFormatEnum::TextFormatUnspecified),
           "html" => Ok(CommentTextFormatEnum::Html),
           "plainText" => Ok(CommentTextFormatEnum::PlainText),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentTextFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentTextFormatEnum {
    fn default() -> CommentTextFormatEnum {
        CommentTextFormatEnum::Html
    }
}

// endregion


// region CommentModerationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: 'heldForReview', 'published' or 'rejected'.
pub enum CommentModerationStatusEnum {
    

    /// The comment is available for public display.
    ///
    /// "published"
    #[serde(rename="published")]
    Published,
    

    /// The comment is awaiting review by a moderator.
    ///
    /// "heldForReview"
    #[serde(rename="heldForReview")]
    HeldForReview,
    
    /// "likelySpam"
    #[serde(rename="likelySpam")]
    LikelySpam,
    

    /// The comment is unfit for display.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
}

impl AsRef<str> for CommentModerationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentModerationStatusEnum::Published => "published",
            CommentModerationStatusEnum::HeldForReview => "heldForReview",
            CommentModerationStatusEnum::LikelySpam => "likelySpam",
            CommentModerationStatusEnum::Rejected => "rejected",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentModerationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "published" => Ok(CommentModerationStatusEnum::Published),
           "heldForReview" => Ok(CommentModerationStatusEnum::HeldForReview),
           "likelySpam" => Ok(CommentModerationStatusEnum::LikelySpam),
           "rejected" => Ok(CommentModerationStatusEnum::Rejected),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentModerationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastBroadcastStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status to which the broadcast is going to transition.
pub enum LiveBroadcastBroadcastStatusEnum {
    
    /// "statusUnspecified"
    #[serde(rename="statusUnspecified")]
    StatusUnspecified,
    

    /// Start testing the broadcast. YouTube transmits video to the broadcast's monitor stream. Note that you can only transition a broadcast to the testing state if its contentDetails.monitorStream.enableMonitorStream property is set to true.",
    ///
    /// "testing"
    #[serde(rename="testing")]
    Testing,
    

    /// Return only persistent broadcasts.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The broadcast is over. YouTube stops transmitting video.
    ///
    /// "complete"
    #[serde(rename="complete")]
    Complete,
}

impl AsRef<str> for LiveBroadcastBroadcastStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastBroadcastStatusEnum::StatusUnspecified => "statusUnspecified",
            LiveBroadcastBroadcastStatusEnum::Testing => "testing",
            LiveBroadcastBroadcastStatusEnum::Live => "live",
            LiveBroadcastBroadcastStatusEnum::Complete => "complete",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastBroadcastStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "statusUnspecified" => Ok(LiveBroadcastBroadcastStatusEnum::StatusUnspecified),
           "testing" => Ok(LiveBroadcastBroadcastStatusEnum::Testing),
           "live" => Ok(LiveBroadcastBroadcastStatusEnum::Live),
           "complete" => Ok(LiveBroadcastBroadcastStatusEnum::Complete),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastBroadcastStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LiveBroadcastBroadcastTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return only broadcasts with the selected type.
pub enum LiveBroadcastBroadcastTypeEnum {
    
    /// "broadcastTypeFilterUnspecified"
    #[serde(rename="broadcastTypeFilterUnspecified")]
    BroadcastTypeFilterUnspecified,
    

    /// Return all broadcasts.
    ///
    /// "all"
    #[serde(rename="all")]
    All,
    

    /// Return only scheduled event broadcasts.
    ///
    /// "event"
    #[serde(rename="event")]
    Event,
    

    /// Return only persistent broadcasts.
    ///
    /// "persistent"
    #[serde(rename="persistent")]
    Persistent,
}

impl AsRef<str> for LiveBroadcastBroadcastTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastBroadcastTypeEnum::BroadcastTypeFilterUnspecified => "broadcastTypeFilterUnspecified",
            LiveBroadcastBroadcastTypeEnum::All => "all",
            LiveBroadcastBroadcastTypeEnum::Event => "event",
            LiveBroadcastBroadcastTypeEnum::Persistent => "persistent",
        }
    }
}

impl std::convert::TryFrom< &str> for LiveBroadcastBroadcastTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "broadcastTypeFilterUnspecified" => Ok(LiveBroadcastBroadcastTypeEnum::BroadcastTypeFilterUnspecified),
           "all" => Ok(LiveBroadcastBroadcastTypeEnum::All),
           "event" => Ok(LiveBroadcastBroadcastTypeEnum::Event),
           "persistent" => Ok(LiveBroadcastBroadcastTypeEnum::Persistent),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastBroadcastTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for LiveBroadcastBroadcastTypeEnum {
    fn default() -> LiveBroadcastBroadcastTypeEnum {
        LiveBroadcastBroadcastTypeEnum::Event
    }
}

// endregion


// region MemberModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Parameter that specifies which channel members to return.
pub enum MemberModeEnum {
    
    /// "listMembersModeUnknown"
    #[serde(rename="listMembersModeUnknown")]
    ListMembersModeUnknown,
    

    /// Return only members that joined after the first call with this mode was made.
    ///
    /// "updates"
    #[serde(rename="updates")]
    Updates,
    

    /// Return all current members, from newest to oldest.
    ///
    /// "all_current"
    #[serde(rename="all_current")]
    AllCurrent,
}

impl AsRef<str> for MemberModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MemberModeEnum::ListMembersModeUnknown => "listMembersModeUnknown",
            MemberModeEnum::Updates => "updates",
            MemberModeEnum::AllCurrent => "all_current",
        }
    }
}

impl std::convert::TryFrom< &str> for MemberModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "listMembersModeUnknown" => Ok(MemberModeEnum::ListMembersModeUnknown),
           "updates" => Ok(MemberModeEnum::Updates),
           "all_current" => Ok(MemberModeEnum::AllCurrent),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MemberModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for MemberModeEnum {
    fn default() -> MemberModeEnum {
        MemberModeEnum::AllCurrent
    }
}

// endregion


// region SearchChannelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Add a filter on the channel search.
pub enum SearchChannelTypeEnum {
    
    /// "channelTypeUnspecified"
    #[serde(rename="channelTypeUnspecified")]
    ChannelTypeUnspecified,
    

    /// Return all channels.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve shows.
    ///
    /// "show"
    #[serde(rename="show")]
    Show,
}

impl AsRef<str> for SearchChannelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchChannelTypeEnum::ChannelTypeUnspecified => "channelTypeUnspecified",
            SearchChannelTypeEnum::Any => "any",
            SearchChannelTypeEnum::Show => "show",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchChannelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "channelTypeUnspecified" => Ok(SearchChannelTypeEnum::ChannelTypeUnspecified),
           "any" => Ok(SearchChannelTypeEnum::Any),
           "show" => Ok(SearchChannelTypeEnum::Show),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchChannelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the livestream status of the videos.
pub enum SearchEventTypeEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The live broadcast is upcoming.
    ///
    /// "upcoming"
    #[serde(rename="upcoming")]
    Upcoming,
    

    /// The live broadcast is active.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The live broadcast has been completed.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for SearchEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchEventTypeEnum::None => "none",
            SearchEventTypeEnum::Upcoming => "upcoming",
            SearchEventTypeEnum::Live => "live",
            SearchEventTypeEnum::Completed => "completed",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(SearchEventTypeEnum::None),
           "upcoming" => Ok(SearchEventTypeEnum::Upcoming),
           "live" => Ok(SearchEventTypeEnum::Live),
           "completed" => Ok(SearchEventTypeEnum::Completed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort order of the results.
pub enum SearchOrderEnum {
    
    /// "searchSortUnspecified"
    #[serde(rename="searchSortUnspecified")]
    SearchSortUnspecified,
    

    /// Resources are sorted in reverse chronological order based on the date they were created.
    ///
    /// "date"
    #[serde(rename="date")]
    Date,
    

    /// Resources are sorted from highest to lowest rating.
    ///
    /// "rating"
    #[serde(rename="rating")]
    Rating,
    

    /// Resources are sorted from highest to lowest number of views.
    ///
    /// "viewCount"
    #[serde(rename="viewCount")]
    ViewCount,
    

    /// Resources are sorted based on their relevance to the search query. This is the default value for this parameter.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
    

    /// Resources are sorted alphabetically by title.
    ///
    /// "title"
    #[serde(rename="title")]
    Title,
    

    /// Channels are sorted in descending order of their number of uploaded videos.
    ///
    /// "videoCount"
    #[serde(rename="videoCount")]
    VideoCount,
}

impl AsRef<str> for SearchOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchOrderEnum::SearchSortUnspecified => "searchSortUnspecified",
            SearchOrderEnum::Date => "date",
            SearchOrderEnum::Rating => "rating",
            SearchOrderEnum::ViewCount => "viewCount",
            SearchOrderEnum::Relevance => "relevance",
            SearchOrderEnum::Title => "title",
            SearchOrderEnum::VideoCount => "videoCount",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "searchSortUnspecified" => Ok(SearchOrderEnum::SearchSortUnspecified),
           "date" => Ok(SearchOrderEnum::Date),
           "rating" => Ok(SearchOrderEnum::Rating),
           "viewCount" => Ok(SearchOrderEnum::ViewCount),
           "relevance" => Ok(SearchOrderEnum::Relevance),
           "title" => Ok(SearchOrderEnum::Title),
           "videoCount" => Ok(SearchOrderEnum::VideoCount),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SearchOrderEnum {
    fn default() -> SearchOrderEnum {
        SearchOrderEnum::Relevance
    }
}

// endregion


// region SearchSafeSearchEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the search results should include restricted content as well as standard content.
pub enum SearchSafeSearchEnum {
    
    /// "safeSearchSettingUnspecified"
    #[serde(rename="safeSearchSettingUnspecified")]
    SafeSearchSettingUnspecified,
    

    /// YouTube will not filter the search result set.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// YouTube will filter some content from search results and, at the least, will filter content that is restricted in your locale. Based on their content, search results could be removed from search results or demoted in search results. This is the default parameter value.
    ///
    /// "moderate"
    #[serde(rename="moderate")]
    Moderate,
    

    /// YouTube will try to exclude all restricted content from the search result set. Based on their content, search results could be removed from search results or demoted in search results.
    ///
    /// "strict"
    #[serde(rename="strict")]
    Strict,
}

impl AsRef<str> for SearchSafeSearchEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchSafeSearchEnum::SafeSearchSettingUnspecified => "safeSearchSettingUnspecified",
            SearchSafeSearchEnum::None => "none",
            SearchSafeSearchEnum::Moderate => "moderate",
            SearchSafeSearchEnum::Strict => "strict",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchSafeSearchEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "safeSearchSettingUnspecified" => Ok(SearchSafeSearchEnum::SafeSearchSettingUnspecified),
           "none" => Ok(SearchSafeSearchEnum::None),
           "moderate" => Ok(SearchSafeSearchEnum::Moderate),
           "strict" => Ok(SearchSafeSearchEnum::Strict),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchSafeSearchEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SearchSafeSearchEnum {
    fn default() -> SearchSafeSearchEnum {
        SearchSafeSearchEnum::Moderate
    }
}

// endregion


// region SearchVideoCaptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the presence of captions on the videos.
pub enum SearchVideoCaptionEnum {
    
    /// "videoCaptionUnspecified"
    #[serde(rename="videoCaptionUnspecified")]
    VideoCaptionUnspecified,
    

    /// Do not filter results based on caption availability.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only include videos that have captions.
    ///
    /// "closedCaption"
    #[serde(rename="closedCaption")]
    ClosedCaption,
    

    /// Only include videos that do not have captions.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
}

impl AsRef<str> for SearchVideoCaptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoCaptionEnum::VideoCaptionUnspecified => "videoCaptionUnspecified",
            SearchVideoCaptionEnum::Any => "any",
            SearchVideoCaptionEnum::ClosedCaption => "closedCaption",
            SearchVideoCaptionEnum::None => "none",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoCaptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoCaptionUnspecified" => Ok(SearchVideoCaptionEnum::VideoCaptionUnspecified),
           "any" => Ok(SearchVideoCaptionEnum::Any),
           "closedCaption" => Ok(SearchVideoCaptionEnum::ClosedCaption),
           "none" => Ok(SearchVideoCaptionEnum::None),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoCaptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoDefinitionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the definition of the videos.
pub enum SearchVideoDefinitionEnum {
    

    /// Return all videos, regardless of their resolution.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve videos in standard definition.
    ///
    /// "standard"
    #[serde(rename="standard")]
    Standard,
    

    /// Only retrieve HD videos.
    ///
    /// "high"
    #[serde(rename="high")]
    High,
}

impl AsRef<str> for SearchVideoDefinitionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoDefinitionEnum::Any => "any",
            SearchVideoDefinitionEnum::Standard => "standard",
            SearchVideoDefinitionEnum::High => "high",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoDefinitionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "any" => Ok(SearchVideoDefinitionEnum::Any),
           "standard" => Ok(SearchVideoDefinitionEnum::Standard),
           "high" => Ok(SearchVideoDefinitionEnum::High),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoDefinitionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoDimensionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on 3d videos.
pub enum SearchVideoDimensionEnum {
    

    /// Include both 3D and non-3D videos in returned results. This is the default value.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Restrict search results to exclude 3D videos.
    ///
    /// "2d"
    #[serde(rename="2d")]
    _2d,
    

    /// Restrict search results to only include 3D videos.
    ///
    /// "3d"
    #[serde(rename="3d")]
    _3d,
}

impl AsRef<str> for SearchVideoDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoDimensionEnum::Any => "any",
            SearchVideoDimensionEnum::_2d => "2d",
            SearchVideoDimensionEnum::_3d => "3d",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoDimensionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "any" => Ok(SearchVideoDimensionEnum::Any),
           "2d" => Ok(SearchVideoDimensionEnum::_2d),
           "3d" => Ok(SearchVideoDimensionEnum::_3d),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoDurationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the duration of the videos.
pub enum SearchVideoDurationEnum {
    
    /// "videoDurationUnspecified"
    #[serde(rename="videoDurationUnspecified")]
    VideoDurationUnspecified,
    

    /// Do not filter video search results based on their duration. This is the default value.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only include videos that are less than four minutes long.
    ///
    /// "short"
    #[serde(rename="short")]
    Short,
    

    /// Only include videos that are between four and 20 minutes long (inclusive).
    ///
    /// "medium"
    #[serde(rename="medium")]
    Medium,
    

    /// Only include videos longer than 20 minutes.
    ///
    /// "long"
    #[serde(rename="long")]
    Long,
}

impl AsRef<str> for SearchVideoDurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoDurationEnum::VideoDurationUnspecified => "videoDurationUnspecified",
            SearchVideoDurationEnum::Any => "any",
            SearchVideoDurationEnum::Short => "short",
            SearchVideoDurationEnum::Medium => "medium",
            SearchVideoDurationEnum::Long => "long",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoDurationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoDurationUnspecified" => Ok(SearchVideoDurationEnum::VideoDurationUnspecified),
           "any" => Ok(SearchVideoDurationEnum::Any),
           "short" => Ok(SearchVideoDurationEnum::Short),
           "medium" => Ok(SearchVideoDurationEnum::Medium),
           "long" => Ok(SearchVideoDurationEnum::Long),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoDurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoEmbeddableEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on embeddable videos.
pub enum SearchVideoEmbeddableEnum {
    
    /// "videoEmbeddableUnspecified"
    #[serde(rename="videoEmbeddableUnspecified")]
    VideoEmbeddableUnspecified,
    

    /// Return all videos, embeddable or not.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve embeddable videos.
    ///
    /// "true"
    #[serde(rename="true")]
    True,
}

impl AsRef<str> for SearchVideoEmbeddableEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoEmbeddableEnum::VideoEmbeddableUnspecified => "videoEmbeddableUnspecified",
            SearchVideoEmbeddableEnum::Any => "any",
            SearchVideoEmbeddableEnum::True => "true",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoEmbeddableEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoEmbeddableUnspecified" => Ok(SearchVideoEmbeddableEnum::VideoEmbeddableUnspecified),
           "any" => Ok(SearchVideoEmbeddableEnum::Any),
           "true" => Ok(SearchVideoEmbeddableEnum::True),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoEmbeddableEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoLicenseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the license of the videos.
pub enum SearchVideoLicenseEnum {
    

    /// Return all videos, regardless of which license they have, that match the query parameters.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only return videos that have the standard YouTube license.
    ///
    /// "youtube"
    #[serde(rename="youtube")]
    Youtube,
    

    /// Only return videos that have a Creative Commons license. Users can reuse videos with this license in other videos that they create. Learn more.
    ///
    /// "creativeCommon"
    #[serde(rename="creativeCommon")]
    CreativeCommon,
}

impl AsRef<str> for SearchVideoLicenseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoLicenseEnum::Any => "any",
            SearchVideoLicenseEnum::Youtube => "youtube",
            SearchVideoLicenseEnum::CreativeCommon => "creativeCommon",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoLicenseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "any" => Ok(SearchVideoLicenseEnum::Any),
           "youtube" => Ok(SearchVideoLicenseEnum::Youtube),
           "creativeCommon" => Ok(SearchVideoLicenseEnum::CreativeCommon),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoLicenseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoPaidProductPlacementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum SearchVideoPaidProductPlacementEnum {
    
    /// "videoPaidProductPlacementUnspecified"
    #[serde(rename="videoPaidProductPlacementUnspecified")]
    VideoPaidProductPlacementUnspecified,
    

    /// Return all videos, paid product placement or not.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Restrict results to only videos with paid product placement.
    ///
    /// "true"
    #[serde(rename="true")]
    True,
}

impl AsRef<str> for SearchVideoPaidProductPlacementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoPaidProductPlacementEnum::VideoPaidProductPlacementUnspecified => "videoPaidProductPlacementUnspecified",
            SearchVideoPaidProductPlacementEnum::Any => "any",
            SearchVideoPaidProductPlacementEnum::True => "true",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoPaidProductPlacementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoPaidProductPlacementUnspecified" => Ok(SearchVideoPaidProductPlacementEnum::VideoPaidProductPlacementUnspecified),
           "any" => Ok(SearchVideoPaidProductPlacementEnum::Any),
           "true" => Ok(SearchVideoPaidProductPlacementEnum::True),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoPaidProductPlacementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoSyndicatedEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on syndicated videos.
pub enum SearchVideoSyndicatedEnum {
    
    /// "videoSyndicatedUnspecified"
    #[serde(rename="videoSyndicatedUnspecified")]
    VideoSyndicatedUnspecified,
    

    /// Return all videos, syndicated or not.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve syndicated videos.
    ///
    /// "true"
    #[serde(rename="true")]
    True,
}

impl AsRef<str> for SearchVideoSyndicatedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoSyndicatedEnum::VideoSyndicatedUnspecified => "videoSyndicatedUnspecified",
            SearchVideoSyndicatedEnum::Any => "any",
            SearchVideoSyndicatedEnum::True => "true",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoSyndicatedEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoSyndicatedUnspecified" => Ok(SearchVideoSyndicatedEnum::VideoSyndicatedUnspecified),
           "any" => Ok(SearchVideoSyndicatedEnum::Any),
           "true" => Ok(SearchVideoSyndicatedEnum::True),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoSyndicatedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchVideoTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on videos of a specific type.
pub enum SearchVideoTypeEnum {
    
    /// "videoTypeUnspecified"
    #[serde(rename="videoTypeUnspecified")]
    VideoTypeUnspecified,
    

    /// Return all videos.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve movies.
    ///
    /// "movie"
    #[serde(rename="movie")]
    Movie,
    

    /// Only retrieve episodes of shows.
    ///
    /// "episode"
    #[serde(rename="episode")]
    Episode,
}

impl AsRef<str> for SearchVideoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoTypeEnum::VideoTypeUnspecified => "videoTypeUnspecified",
            SearchVideoTypeEnum::Any => "any",
            SearchVideoTypeEnum::Movie => "movie",
            SearchVideoTypeEnum::Episode => "episode",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchVideoTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "videoTypeUnspecified" => Ok(SearchVideoTypeEnum::VideoTypeUnspecified),
           "any" => Ok(SearchVideoTypeEnum::Any),
           "movie" => Ok(SearchVideoTypeEnum::Movie),
           "episode" => Ok(SearchVideoTypeEnum::Episode),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SubscriptionOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order of the returned subscriptions
pub enum SubscriptionOrderEnum {
    
    /// "subscriptionOrderUnspecified"
    #[serde(rename="subscriptionOrderUnspecified")]
    SubscriptionOrderUnspecified,
    

    /// Sort by relevance.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
    

    /// Sort by order of activity.
    ///
    /// "unread"
    #[serde(rename="unread")]
    Unread,
    

    /// Sort alphabetically.
    ///
    /// "alphabetical"
    #[serde(rename="alphabetical")]
    Alphabetical,
}

impl AsRef<str> for SubscriptionOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionOrderEnum::SubscriptionOrderUnspecified => "subscriptionOrderUnspecified",
            SubscriptionOrderEnum::Relevance => "relevance",
            SubscriptionOrderEnum::Unread => "unread",
            SubscriptionOrderEnum::Alphabetical => "alphabetical",
        }
    }
}

impl std::convert::TryFrom< &str> for SubscriptionOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "subscriptionOrderUnspecified" => Ok(SubscriptionOrderEnum::SubscriptionOrderUnspecified),
           "relevance" => Ok(SubscriptionOrderEnum::Relevance),
           "unread" => Ok(SubscriptionOrderEnum::Unread),
           "alphabetical" => Ok(SubscriptionOrderEnum::Alphabetical),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SubscriptionOrderEnum {
    fn default() -> SubscriptionOrderEnum {
        SubscriptionOrderEnum::Relevance
    }
}

// endregion


// region ThirdPartyLinkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Get a third party link of the given type.
pub enum ThirdPartyLinkTypeEnum {
    
    /// "linkUnspecified"
    #[serde(rename="linkUnspecified")]
    LinkUnspecified,
    

    /// A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube.
    ///
    /// "channelToStoreLink"
    #[serde(rename="channelToStoreLink")]
    ChannelToStoreLink,
}

impl AsRef<str> for ThirdPartyLinkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyLinkTypeEnum::LinkUnspecified => "linkUnspecified",
            ThirdPartyLinkTypeEnum::ChannelToStoreLink => "channelToStoreLink",
        }
    }
}

impl std::convert::TryFrom< &str> for ThirdPartyLinkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "linkUnspecified" => Ok(ThirdPartyLinkTypeEnum::LinkUnspecified),
           "channelToStoreLink" => Ok(ThirdPartyLinkTypeEnum::ChannelToStoreLink),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyLinkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoChartEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return the videos that are in the specified chart.
pub enum VideoChartEnum {
    
    /// "chartUnspecified"
    #[serde(rename="chartUnspecified")]
    ChartUnspecified,
    

    /// Return the most popular videos for the specified content region and video category.
    ///
    /// "mostPopular"
    #[serde(rename="mostPopular")]
    MostPopular,
}

impl AsRef<str> for VideoChartEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoChartEnum::ChartUnspecified => "chartUnspecified",
            VideoChartEnum::MostPopular => "mostPopular",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoChartEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "chartUnspecified" => Ok(VideoChartEnum::ChartUnspecified),
           "mostPopular" => Ok(VideoChartEnum::MostPopular),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoChartEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoMyRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return videos liked/disliked by the authenticated user. Does not support RateType.RATED_TYPE_NONE.
pub enum VideoMyRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for VideoMyRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoMyRatingEnum::None => "none",
            VideoMyRatingEnum::Like => "like",
            VideoMyRatingEnum::Dislike => "dislike",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoMyRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(VideoMyRatingEnum::None),
           "like" => Ok(VideoMyRatingEnum::Like),
           "dislike" => Ok(VideoMyRatingEnum::Dislike),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoMyRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VideoRatingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for VideoRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoRatingEnum::None => "none",
            VideoRatingEnum::Like => "like",
            VideoRatingEnum::Dislike => "dislike",
        }
    }
}

impl std::convert::TryFrom< &str> for VideoRatingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(VideoRatingEnum::None),
           "like" => Ok(VideoRatingEnum::Like),
           "dislike" => Ok(VideoRatingEnum::Dislike),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


