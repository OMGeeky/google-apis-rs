use super::*;



// region AccountAccesPermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of Account permissions. Valid account permissions are read and manage. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
pub enum AccountAccesPermissionEnum {
    
    /// "read"
    #[serde(rename="read")]
    Read,
    
    /// "edit"
    #[serde(rename="edit")]
    Edit,
    
    /// "publish"
    #[serde(rename="publish")]
    Publish,
    
    /// "delete"
    #[serde(rename="delete")]
    Delete,
    
    /// "manage"
    #[serde(rename="manage")]
    Manage,
    
    /// "editWorkspace"
    #[serde(rename="editWorkspace")]
    EditWorkspace,
}

impl AsRef<str> for AccountAccesPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountAccesPermissionEnum::Read => "read",
            AccountAccesPermissionEnum::Edit => "edit",
            AccountAccesPermissionEnum::Publish => "publish",
            AccountAccesPermissionEnum::Delete => "delete",
            AccountAccesPermissionEnum::Manage => "manage",
            AccountAccesPermissionEnum::EditWorkspace => "editWorkspace",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountAccesPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "read" => Ok(AccountAccesPermissionEnum::Read),
           "edit" => Ok(AccountAccesPermissionEnum::Edit),
           "publish" => Ok(AccountAccesPermissionEnum::Publish),
           "delete" => Ok(AccountAccesPermissionEnum::Delete),
           "manage" => Ok(AccountAccesPermissionEnum::Manage),
           "editWorkspace" => Ok(AccountAccesPermissionEnum::EditWorkspace),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountAccesPermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConditionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of operator for this condition. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
pub enum ConditionTypeEnum {
    
    /// "equals"
    #[serde(rename="equals")]
    Equals,
    
    /// "contains"
    #[serde(rename="contains")]
    Contains,
    
    /// "startsWith"
    #[serde(rename="startsWith")]
    StartsWith,
    
    /// "endsWith"
    #[serde(rename="endsWith")]
    EndsWith,
    
    /// "matchRegex"
    #[serde(rename="matchRegex")]
    MatchRegex,
    
    /// "greater"
    #[serde(rename="greater")]
    Greater,
    
    /// "greaterOrEquals"
    #[serde(rename="greaterOrEquals")]
    GreaterOrEquals,
    
    /// "less"
    #[serde(rename="less")]
    Less,
    
    /// "lessOrEquals"
    #[serde(rename="lessOrEquals")]
    LessOrEquals,
    
    /// "cssSelector"
    #[serde(rename="cssSelector")]
    CssSelector,
    
    /// "urlMatches"
    #[serde(rename="urlMatches")]
    UrlMatches,
}

impl AsRef<str> for ConditionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionTypeEnum::Equals => "equals",
            ConditionTypeEnum::Contains => "contains",
            ConditionTypeEnum::StartsWith => "startsWith",
            ConditionTypeEnum::EndsWith => "endsWith",
            ConditionTypeEnum::MatchRegex => "matchRegex",
            ConditionTypeEnum::Greater => "greater",
            ConditionTypeEnum::GreaterOrEquals => "greaterOrEquals",
            ConditionTypeEnum::Less => "less",
            ConditionTypeEnum::LessOrEquals => "lessOrEquals",
            ConditionTypeEnum::CssSelector => "cssSelector",
            ConditionTypeEnum::UrlMatches => "urlMatches",
        }
    }
}

impl std::convert::TryFrom< &str> for ConditionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "equals" => Ok(ConditionTypeEnum::Equals),
           "contains" => Ok(ConditionTypeEnum::Contains),
           "startsWith" => Ok(ConditionTypeEnum::StartsWith),
           "endsWith" => Ok(ConditionTypeEnum::EndsWith),
           "matchRegex" => Ok(ConditionTypeEnum::MatchRegex),
           "greater" => Ok(ConditionTypeEnum::Greater),
           "greaterOrEquals" => Ok(ConditionTypeEnum::GreaterOrEquals),
           "less" => Ok(ConditionTypeEnum::Less),
           "lessOrEquals" => Ok(ConditionTypeEnum::LessOrEquals),
           "cssSelector" => Ok(ConditionTypeEnum::CssSelector),
           "urlMatches" => Ok(ConditionTypeEnum::UrlMatches),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConditionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContainerEnabledBuiltInVariableEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
pub enum ContainerEnabledBuiltInVariableEnum {
    
    /// "pageUrl"
    #[serde(rename="pageUrl")]
    PageUrl,
    
    /// "pageHostname"
    #[serde(rename="pageHostname")]
    PageHostname,
    
    /// "pagePath"
    #[serde(rename="pagePath")]
    PagePath,
    
    /// "referrer"
    #[serde(rename="referrer")]
    Referrer,
    

    /// For web or mobile.
    ///
    /// "event"
    #[serde(rename="event")]
    Event,
    
    /// "clickElement"
    #[serde(rename="clickElement")]
    ClickElement,
    
    /// "clickClasses"
    #[serde(rename="clickClasses")]
    ClickClasses,
    
    /// "clickId"
    #[serde(rename="clickId")]
    ClickId,
    
    /// "clickTarget"
    #[serde(rename="clickTarget")]
    ClickTarget,
    
    /// "clickUrl"
    #[serde(rename="clickUrl")]
    ClickUrl,
    
    /// "clickText"
    #[serde(rename="clickText")]
    ClickText,
    
    /// "firstPartyServingUrl"
    #[serde(rename="firstPartyServingUrl")]
    FirstPartyServingUrl,
    
    /// "formElement"
    #[serde(rename="formElement")]
    FormElement,
    
    /// "formClasses"
    #[serde(rename="formClasses")]
    FormClasses,
    
    /// "formId"
    #[serde(rename="formId")]
    FormId,
    
    /// "formTarget"
    #[serde(rename="formTarget")]
    FormTarget,
    
    /// "formUrl"
    #[serde(rename="formUrl")]
    FormUrl,
    
    /// "formText"
    #[serde(rename="formText")]
    FormText,
    
    /// "environmentName"
    #[serde(rename="environmentName")]
    EnvironmentName,
    
    /// "errorMessage"
    #[serde(rename="errorMessage")]
    ErrorMessage,
    
    /// "errorUrl"
    #[serde(rename="errorUrl")]
    ErrorUrl,
    
    /// "errorLine"
    #[serde(rename="errorLine")]
    ErrorLine,
    
    /// "newHistoryUrl"
    #[serde(rename="newHistoryUrl")]
    NewHistoryUrl,
    
    /// "oldHistoryUrl"
    #[serde(rename="oldHistoryUrl")]
    OldHistoryUrl,
    
    /// "newHistoryFragment"
    #[serde(rename="newHistoryFragment")]
    NewHistoryFragment,
    
    /// "oldHistoryFragment"
    #[serde(rename="oldHistoryFragment")]
    OldHistoryFragment,
    
    /// "newHistoryState"
    #[serde(rename="newHistoryState")]
    NewHistoryState,
    
    /// "oldHistoryState"
    #[serde(rename="oldHistoryState")]
    OldHistoryState,
    
    /// "historySource"
    #[serde(rename="historySource")]
    HistorySource,
    

    /// For web or mobile.
    ///
    /// "containerVersion"
    #[serde(rename="containerVersion")]
    ContainerVersion,
    
    /// "debugMode"
    #[serde(rename="debugMode")]
    DebugMode,
    

    /// For web or mobile.
    ///
    /// "randomNumber"
    #[serde(rename="randomNumber")]
    RandomNumber,
    

    /// For web or mobile.
    ///
    /// "containerId"
    #[serde(rename="containerId")]
    ContainerId,
    
    /// "appId"
    #[serde(rename="appId")]
    AppId,
    
    /// "appName"
    #[serde(rename="appName")]
    AppName,
    
    /// "appVersionCode"
    #[serde(rename="appVersionCode")]
    AppVersionCode,
    
    /// "appVersionName"
    #[serde(rename="appVersionName")]
    AppVersionName,
    
    /// "language"
    #[serde(rename="language")]
    Language,
    
    /// "osVersion"
    #[serde(rename="osVersion")]
    OsVersion,
    
    /// "platform"
    #[serde(rename="platform")]
    Platform,
    
    /// "sdkVersion"
    #[serde(rename="sdkVersion")]
    SdkVersion,
    
    /// "deviceName"
    #[serde(rename="deviceName")]
    DeviceName,
    
    /// "resolution"
    #[serde(rename="resolution")]
    Resolution,
    
    /// "advertiserId"
    #[serde(rename="advertiserId")]
    AdvertiserId,
    
    /// "advertisingTrackingEnabled"
    #[serde(rename="advertisingTrackingEnabled")]
    AdvertisingTrackingEnabled,
    
    /// "htmlId"
    #[serde(rename="htmlId")]
    HtmlId,
    
    /// "ampBrowserLanguage"
    #[serde(rename="ampBrowserLanguage")]
    AmpBrowserLanguage,
    
    /// "ampCanonicalPath"
    #[serde(rename="ampCanonicalPath")]
    AmpCanonicalPath,
    
    /// "ampCanonicalUrl"
    #[serde(rename="ampCanonicalUrl")]
    AmpCanonicalUrl,
    
    /// "ampCanonicalHost"
    #[serde(rename="ampCanonicalHost")]
    AmpCanonicalHost,
    
    /// "ampReferrer"
    #[serde(rename="ampReferrer")]
    AmpReferrer,
    
    /// "ampTitle"
    #[serde(rename="ampTitle")]
    AmpTitle,
    
    /// "ampClientId"
    #[serde(rename="ampClientId")]
    AmpClientId,
    
    /// "ampClientTimezone"
    #[serde(rename="ampClientTimezone")]
    AmpClientTimezone,
    
    /// "ampClientTimestamp"
    #[serde(rename="ampClientTimestamp")]
    AmpClientTimestamp,
    
    /// "ampClientScreenWidth"
    #[serde(rename="ampClientScreenWidth")]
    AmpClientScreenWidth,
    
    /// "ampClientScreenHeight"
    #[serde(rename="ampClientScreenHeight")]
    AmpClientScreenHeight,
    
    /// "ampClientScrollX"
    #[serde(rename="ampClientScrollX")]
    AmpClientScrollX,
    
    /// "ampClientScrollY"
    #[serde(rename="ampClientScrollY")]
    AmpClientScrollY,
    
    /// "ampClientMaxScrollX"
    #[serde(rename="ampClientMaxScrollX")]
    AmpClientMaxScrollX,
    
    /// "ampClientMaxScrollY"
    #[serde(rename="ampClientMaxScrollY")]
    AmpClientMaxScrollY,
    
    /// "ampTotalEngagedTime"
    #[serde(rename="ampTotalEngagedTime")]
    AmpTotalEngagedTime,
    
    /// "ampPageViewId"
    #[serde(rename="ampPageViewId")]
    AmpPageViewId,
    
    /// "ampPageLoadTime"
    #[serde(rename="ampPageLoadTime")]
    AmpPageLoadTime,
    
    /// "ampPageDownloadTime"
    #[serde(rename="ampPageDownloadTime")]
    AmpPageDownloadTime,
    
    /// "ampGtmEvent"
    #[serde(rename="ampGtmEvent")]
    AmpGtmEvent,
    
    /// "eventName"
    #[serde(rename="eventName")]
    EventName,
    
    /// "firebaseEventParameterCampaign"
    #[serde(rename="firebaseEventParameterCampaign")]
    FirebaseEventParameterCampaign,
    
    /// "firebaseEventParameterCampaignAclid"
    #[serde(rename="firebaseEventParameterCampaignAclid")]
    FirebaseEventParameterCampaignAclid,
    
    /// "firebaseEventParameterCampaignAnid"
    #[serde(rename="firebaseEventParameterCampaignAnid")]
    FirebaseEventParameterCampaignAnid,
    
    /// "firebaseEventParameterCampaignClickTimestamp"
    #[serde(rename="firebaseEventParameterCampaignClickTimestamp")]
    FirebaseEventParameterCampaignClickTimestamp,
    
    /// "firebaseEventParameterCampaignContent"
    #[serde(rename="firebaseEventParameterCampaignContent")]
    FirebaseEventParameterCampaignContent,
    
    /// "firebaseEventParameterCampaignCp1"
    #[serde(rename="firebaseEventParameterCampaignCp1")]
    FirebaseEventParameterCampaignCp1,
    
    /// "firebaseEventParameterCampaignGclid"
    #[serde(rename="firebaseEventParameterCampaignGclid")]
    FirebaseEventParameterCampaignGclid,
    
    /// "firebaseEventParameterCampaignSource"
    #[serde(rename="firebaseEventParameterCampaignSource")]
    FirebaseEventParameterCampaignSource,
    
    /// "firebaseEventParameterCampaignTerm"
    #[serde(rename="firebaseEventParameterCampaignTerm")]
    FirebaseEventParameterCampaignTerm,
    
    /// "firebaseEventParameterCurrency"
    #[serde(rename="firebaseEventParameterCurrency")]
    FirebaseEventParameterCurrency,
    
    /// "firebaseEventParameterDynamicLinkAcceptTime"
    #[serde(rename="firebaseEventParameterDynamicLinkAcceptTime")]
    FirebaseEventParameterDynamicLinkAcceptTime,
    
    /// "firebaseEventParameterDynamicLinkLinkid"
    #[serde(rename="firebaseEventParameterDynamicLinkLinkid")]
    FirebaseEventParameterDynamicLinkLinkid,
    
    /// "firebaseEventParameterNotificationMessageDeviceTime"
    #[serde(rename="firebaseEventParameterNotificationMessageDeviceTime")]
    FirebaseEventParameterNotificationMessageDeviceTime,
    
    /// "firebaseEventParameterNotificationMessageId"
    #[serde(rename="firebaseEventParameterNotificationMessageId")]
    FirebaseEventParameterNotificationMessageId,
    
    /// "firebaseEventParameterNotificationMessageName"
    #[serde(rename="firebaseEventParameterNotificationMessageName")]
    FirebaseEventParameterNotificationMessageName,
    
    /// "firebaseEventParameterNotificationMessageTime"
    #[serde(rename="firebaseEventParameterNotificationMessageTime")]
    FirebaseEventParameterNotificationMessageTime,
    
    /// "firebaseEventParameterNotificationTopic"
    #[serde(rename="firebaseEventParameterNotificationTopic")]
    FirebaseEventParameterNotificationTopic,
    
    /// "firebaseEventParameterPreviousAppVersion"
    #[serde(rename="firebaseEventParameterPreviousAppVersion")]
    FirebaseEventParameterPreviousAppVersion,
    
    /// "firebaseEventParameterPreviousOsVersion"
    #[serde(rename="firebaseEventParameterPreviousOsVersion")]
    FirebaseEventParameterPreviousOsVersion,
    
    /// "firebaseEventParameterPrice"
    #[serde(rename="firebaseEventParameterPrice")]
    FirebaseEventParameterPrice,
    
    /// "firebaseEventParameterProductId"
    #[serde(rename="firebaseEventParameterProductId")]
    FirebaseEventParameterProductId,
    
    /// "firebaseEventParameterQuantity"
    #[serde(rename="firebaseEventParameterQuantity")]
    FirebaseEventParameterQuantity,
    
    /// "firebaseEventParameterValue"
    #[serde(rename="firebaseEventParameterValue")]
    FirebaseEventParameterValue,
    
    /// "videoProvider"
    #[serde(rename="videoProvider")]
    VideoProvider,
    
    /// "videoUrl"
    #[serde(rename="videoUrl")]
    VideoUrl,
    
    /// "videoTitle"
    #[serde(rename="videoTitle")]
    VideoTitle,
    
    /// "videoDuration"
    #[serde(rename="videoDuration")]
    VideoDuration,
    
    /// "videoPercent"
    #[serde(rename="videoPercent")]
    VideoPercent,
    
    /// "videoVisible"
    #[serde(rename="videoVisible")]
    VideoVisible,
    
    /// "videoStatus"
    #[serde(rename="videoStatus")]
    VideoStatus,
    
    /// "videoCurrentTime"
    #[serde(rename="videoCurrentTime")]
    VideoCurrentTime,
    
    /// "scrollDepthThreshold"
    #[serde(rename="scrollDepthThreshold")]
    ScrollDepthThreshold,
    
    /// "scrollDepthUnits"
    #[serde(rename="scrollDepthUnits")]
    ScrollDepthUnits,
    
    /// "scrollDepthDirection"
    #[serde(rename="scrollDepthDirection")]
    ScrollDepthDirection,
    
    /// "elementVisibilityRatio"
    #[serde(rename="elementVisibilityRatio")]
    ElementVisibilityRatio,
    
    /// "elementVisibilityTime"
    #[serde(rename="elementVisibilityTime")]
    ElementVisibilityTime,
    
    /// "elementVisibilityFirstTime"
    #[serde(rename="elementVisibilityFirstTime")]
    ElementVisibilityFirstTime,
    
    /// "elementVisibilityRecentTime"
    #[serde(rename="elementVisibilityRecentTime")]
    ElementVisibilityRecentTime,
}

impl AsRef<str> for ContainerEnabledBuiltInVariableEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContainerEnabledBuiltInVariableEnum::PageUrl => "pageUrl",
            ContainerEnabledBuiltInVariableEnum::PageHostname => "pageHostname",
            ContainerEnabledBuiltInVariableEnum::PagePath => "pagePath",
            ContainerEnabledBuiltInVariableEnum::Referrer => "referrer",
            ContainerEnabledBuiltInVariableEnum::Event => "event",
            ContainerEnabledBuiltInVariableEnum::ClickElement => "clickElement",
            ContainerEnabledBuiltInVariableEnum::ClickClasses => "clickClasses",
            ContainerEnabledBuiltInVariableEnum::ClickId => "clickId",
            ContainerEnabledBuiltInVariableEnum::ClickTarget => "clickTarget",
            ContainerEnabledBuiltInVariableEnum::ClickUrl => "clickUrl",
            ContainerEnabledBuiltInVariableEnum::ClickText => "clickText",
            ContainerEnabledBuiltInVariableEnum::FirstPartyServingUrl => "firstPartyServingUrl",
            ContainerEnabledBuiltInVariableEnum::FormElement => "formElement",
            ContainerEnabledBuiltInVariableEnum::FormClasses => "formClasses",
            ContainerEnabledBuiltInVariableEnum::FormId => "formId",
            ContainerEnabledBuiltInVariableEnum::FormTarget => "formTarget",
            ContainerEnabledBuiltInVariableEnum::FormUrl => "formUrl",
            ContainerEnabledBuiltInVariableEnum::FormText => "formText",
            ContainerEnabledBuiltInVariableEnum::EnvironmentName => "environmentName",
            ContainerEnabledBuiltInVariableEnum::ErrorMessage => "errorMessage",
            ContainerEnabledBuiltInVariableEnum::ErrorUrl => "errorUrl",
            ContainerEnabledBuiltInVariableEnum::ErrorLine => "errorLine",
            ContainerEnabledBuiltInVariableEnum::NewHistoryUrl => "newHistoryUrl",
            ContainerEnabledBuiltInVariableEnum::OldHistoryUrl => "oldHistoryUrl",
            ContainerEnabledBuiltInVariableEnum::NewHistoryFragment => "newHistoryFragment",
            ContainerEnabledBuiltInVariableEnum::OldHistoryFragment => "oldHistoryFragment",
            ContainerEnabledBuiltInVariableEnum::NewHistoryState => "newHistoryState",
            ContainerEnabledBuiltInVariableEnum::OldHistoryState => "oldHistoryState",
            ContainerEnabledBuiltInVariableEnum::HistorySource => "historySource",
            ContainerEnabledBuiltInVariableEnum::ContainerVersion => "containerVersion",
            ContainerEnabledBuiltInVariableEnum::DebugMode => "debugMode",
            ContainerEnabledBuiltInVariableEnum::RandomNumber => "randomNumber",
            ContainerEnabledBuiltInVariableEnum::ContainerId => "containerId",
            ContainerEnabledBuiltInVariableEnum::AppId => "appId",
            ContainerEnabledBuiltInVariableEnum::AppName => "appName",
            ContainerEnabledBuiltInVariableEnum::AppVersionCode => "appVersionCode",
            ContainerEnabledBuiltInVariableEnum::AppVersionName => "appVersionName",
            ContainerEnabledBuiltInVariableEnum::Language => "language",
            ContainerEnabledBuiltInVariableEnum::OsVersion => "osVersion",
            ContainerEnabledBuiltInVariableEnum::Platform => "platform",
            ContainerEnabledBuiltInVariableEnum::SdkVersion => "sdkVersion",
            ContainerEnabledBuiltInVariableEnum::DeviceName => "deviceName",
            ContainerEnabledBuiltInVariableEnum::Resolution => "resolution",
            ContainerEnabledBuiltInVariableEnum::AdvertiserId => "advertiserId",
            ContainerEnabledBuiltInVariableEnum::AdvertisingTrackingEnabled => "advertisingTrackingEnabled",
            ContainerEnabledBuiltInVariableEnum::HtmlId => "htmlId",
            ContainerEnabledBuiltInVariableEnum::AmpBrowserLanguage => "ampBrowserLanguage",
            ContainerEnabledBuiltInVariableEnum::AmpCanonicalPath => "ampCanonicalPath",
            ContainerEnabledBuiltInVariableEnum::AmpCanonicalUrl => "ampCanonicalUrl",
            ContainerEnabledBuiltInVariableEnum::AmpCanonicalHost => "ampCanonicalHost",
            ContainerEnabledBuiltInVariableEnum::AmpReferrer => "ampReferrer",
            ContainerEnabledBuiltInVariableEnum::AmpTitle => "ampTitle",
            ContainerEnabledBuiltInVariableEnum::AmpClientId => "ampClientId",
            ContainerEnabledBuiltInVariableEnum::AmpClientTimezone => "ampClientTimezone",
            ContainerEnabledBuiltInVariableEnum::AmpClientTimestamp => "ampClientTimestamp",
            ContainerEnabledBuiltInVariableEnum::AmpClientScreenWidth => "ampClientScreenWidth",
            ContainerEnabledBuiltInVariableEnum::AmpClientScreenHeight => "ampClientScreenHeight",
            ContainerEnabledBuiltInVariableEnum::AmpClientScrollX => "ampClientScrollX",
            ContainerEnabledBuiltInVariableEnum::AmpClientScrollY => "ampClientScrollY",
            ContainerEnabledBuiltInVariableEnum::AmpClientMaxScrollX => "ampClientMaxScrollX",
            ContainerEnabledBuiltInVariableEnum::AmpClientMaxScrollY => "ampClientMaxScrollY",
            ContainerEnabledBuiltInVariableEnum::AmpTotalEngagedTime => "ampTotalEngagedTime",
            ContainerEnabledBuiltInVariableEnum::AmpPageViewId => "ampPageViewId",
            ContainerEnabledBuiltInVariableEnum::AmpPageLoadTime => "ampPageLoadTime",
            ContainerEnabledBuiltInVariableEnum::AmpPageDownloadTime => "ampPageDownloadTime",
            ContainerEnabledBuiltInVariableEnum::AmpGtmEvent => "ampGtmEvent",
            ContainerEnabledBuiltInVariableEnum::EventName => "eventName",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaign => "firebaseEventParameterCampaign",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignAclid => "firebaseEventParameterCampaignAclid",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignAnid => "firebaseEventParameterCampaignAnid",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignClickTimestamp => "firebaseEventParameterCampaignClickTimestamp",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignContent => "firebaseEventParameterCampaignContent",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignCp1 => "firebaseEventParameterCampaignCp1",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignGclid => "firebaseEventParameterCampaignGclid",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignSource => "firebaseEventParameterCampaignSource",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignTerm => "firebaseEventParameterCampaignTerm",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCurrency => "firebaseEventParameterCurrency",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterDynamicLinkAcceptTime => "firebaseEventParameterDynamicLinkAcceptTime",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterDynamicLinkLinkid => "firebaseEventParameterDynamicLinkLinkid",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageDeviceTime => "firebaseEventParameterNotificationMessageDeviceTime",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageId => "firebaseEventParameterNotificationMessageId",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageName => "firebaseEventParameterNotificationMessageName",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageTime => "firebaseEventParameterNotificationMessageTime",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationTopic => "firebaseEventParameterNotificationTopic",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterPreviousAppVersion => "firebaseEventParameterPreviousAppVersion",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterPreviousOsVersion => "firebaseEventParameterPreviousOsVersion",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterPrice => "firebaseEventParameterPrice",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterProductId => "firebaseEventParameterProductId",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterQuantity => "firebaseEventParameterQuantity",
            ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterValue => "firebaseEventParameterValue",
            ContainerEnabledBuiltInVariableEnum::VideoProvider => "videoProvider",
            ContainerEnabledBuiltInVariableEnum::VideoUrl => "videoUrl",
            ContainerEnabledBuiltInVariableEnum::VideoTitle => "videoTitle",
            ContainerEnabledBuiltInVariableEnum::VideoDuration => "videoDuration",
            ContainerEnabledBuiltInVariableEnum::VideoPercent => "videoPercent",
            ContainerEnabledBuiltInVariableEnum::VideoVisible => "videoVisible",
            ContainerEnabledBuiltInVariableEnum::VideoStatus => "videoStatus",
            ContainerEnabledBuiltInVariableEnum::VideoCurrentTime => "videoCurrentTime",
            ContainerEnabledBuiltInVariableEnum::ScrollDepthThreshold => "scrollDepthThreshold",
            ContainerEnabledBuiltInVariableEnum::ScrollDepthUnits => "scrollDepthUnits",
            ContainerEnabledBuiltInVariableEnum::ScrollDepthDirection => "scrollDepthDirection",
            ContainerEnabledBuiltInVariableEnum::ElementVisibilityRatio => "elementVisibilityRatio",
            ContainerEnabledBuiltInVariableEnum::ElementVisibilityTime => "elementVisibilityTime",
            ContainerEnabledBuiltInVariableEnum::ElementVisibilityFirstTime => "elementVisibilityFirstTime",
            ContainerEnabledBuiltInVariableEnum::ElementVisibilityRecentTime => "elementVisibilityRecentTime",
        }
    }
}

impl std::convert::TryFrom< &str> for ContainerEnabledBuiltInVariableEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "pageUrl" => Ok(ContainerEnabledBuiltInVariableEnum::PageUrl),
           "pageHostname" => Ok(ContainerEnabledBuiltInVariableEnum::PageHostname),
           "pagePath" => Ok(ContainerEnabledBuiltInVariableEnum::PagePath),
           "referrer" => Ok(ContainerEnabledBuiltInVariableEnum::Referrer),
           "event" => Ok(ContainerEnabledBuiltInVariableEnum::Event),
           "clickElement" => Ok(ContainerEnabledBuiltInVariableEnum::ClickElement),
           "clickClasses" => Ok(ContainerEnabledBuiltInVariableEnum::ClickClasses),
           "clickId" => Ok(ContainerEnabledBuiltInVariableEnum::ClickId),
           "clickTarget" => Ok(ContainerEnabledBuiltInVariableEnum::ClickTarget),
           "clickUrl" => Ok(ContainerEnabledBuiltInVariableEnum::ClickUrl),
           "clickText" => Ok(ContainerEnabledBuiltInVariableEnum::ClickText),
           "firstPartyServingUrl" => Ok(ContainerEnabledBuiltInVariableEnum::FirstPartyServingUrl),
           "formElement" => Ok(ContainerEnabledBuiltInVariableEnum::FormElement),
           "formClasses" => Ok(ContainerEnabledBuiltInVariableEnum::FormClasses),
           "formId" => Ok(ContainerEnabledBuiltInVariableEnum::FormId),
           "formTarget" => Ok(ContainerEnabledBuiltInVariableEnum::FormTarget),
           "formUrl" => Ok(ContainerEnabledBuiltInVariableEnum::FormUrl),
           "formText" => Ok(ContainerEnabledBuiltInVariableEnum::FormText),
           "environmentName" => Ok(ContainerEnabledBuiltInVariableEnum::EnvironmentName),
           "errorMessage" => Ok(ContainerEnabledBuiltInVariableEnum::ErrorMessage),
           "errorUrl" => Ok(ContainerEnabledBuiltInVariableEnum::ErrorUrl),
           "errorLine" => Ok(ContainerEnabledBuiltInVariableEnum::ErrorLine),
           "newHistoryUrl" => Ok(ContainerEnabledBuiltInVariableEnum::NewHistoryUrl),
           "oldHistoryUrl" => Ok(ContainerEnabledBuiltInVariableEnum::OldHistoryUrl),
           "newHistoryFragment" => Ok(ContainerEnabledBuiltInVariableEnum::NewHistoryFragment),
           "oldHistoryFragment" => Ok(ContainerEnabledBuiltInVariableEnum::OldHistoryFragment),
           "newHistoryState" => Ok(ContainerEnabledBuiltInVariableEnum::NewHistoryState),
           "oldHistoryState" => Ok(ContainerEnabledBuiltInVariableEnum::OldHistoryState),
           "historySource" => Ok(ContainerEnabledBuiltInVariableEnum::HistorySource),
           "containerVersion" => Ok(ContainerEnabledBuiltInVariableEnum::ContainerVersion),
           "debugMode" => Ok(ContainerEnabledBuiltInVariableEnum::DebugMode),
           "randomNumber" => Ok(ContainerEnabledBuiltInVariableEnum::RandomNumber),
           "containerId" => Ok(ContainerEnabledBuiltInVariableEnum::ContainerId),
           "appId" => Ok(ContainerEnabledBuiltInVariableEnum::AppId),
           "appName" => Ok(ContainerEnabledBuiltInVariableEnum::AppName),
           "appVersionCode" => Ok(ContainerEnabledBuiltInVariableEnum::AppVersionCode),
           "appVersionName" => Ok(ContainerEnabledBuiltInVariableEnum::AppVersionName),
           "language" => Ok(ContainerEnabledBuiltInVariableEnum::Language),
           "osVersion" => Ok(ContainerEnabledBuiltInVariableEnum::OsVersion),
           "platform" => Ok(ContainerEnabledBuiltInVariableEnum::Platform),
           "sdkVersion" => Ok(ContainerEnabledBuiltInVariableEnum::SdkVersion),
           "deviceName" => Ok(ContainerEnabledBuiltInVariableEnum::DeviceName),
           "resolution" => Ok(ContainerEnabledBuiltInVariableEnum::Resolution),
           "advertiserId" => Ok(ContainerEnabledBuiltInVariableEnum::AdvertiserId),
           "advertisingTrackingEnabled" => Ok(ContainerEnabledBuiltInVariableEnum::AdvertisingTrackingEnabled),
           "htmlId" => Ok(ContainerEnabledBuiltInVariableEnum::HtmlId),
           "ampBrowserLanguage" => Ok(ContainerEnabledBuiltInVariableEnum::AmpBrowserLanguage),
           "ampCanonicalPath" => Ok(ContainerEnabledBuiltInVariableEnum::AmpCanonicalPath),
           "ampCanonicalUrl" => Ok(ContainerEnabledBuiltInVariableEnum::AmpCanonicalUrl),
           "ampCanonicalHost" => Ok(ContainerEnabledBuiltInVariableEnum::AmpCanonicalHost),
           "ampReferrer" => Ok(ContainerEnabledBuiltInVariableEnum::AmpReferrer),
           "ampTitle" => Ok(ContainerEnabledBuiltInVariableEnum::AmpTitle),
           "ampClientId" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientId),
           "ampClientTimezone" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientTimezone),
           "ampClientTimestamp" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientTimestamp),
           "ampClientScreenWidth" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientScreenWidth),
           "ampClientScreenHeight" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientScreenHeight),
           "ampClientScrollX" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientScrollX),
           "ampClientScrollY" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientScrollY),
           "ampClientMaxScrollX" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientMaxScrollX),
           "ampClientMaxScrollY" => Ok(ContainerEnabledBuiltInVariableEnum::AmpClientMaxScrollY),
           "ampTotalEngagedTime" => Ok(ContainerEnabledBuiltInVariableEnum::AmpTotalEngagedTime),
           "ampPageViewId" => Ok(ContainerEnabledBuiltInVariableEnum::AmpPageViewId),
           "ampPageLoadTime" => Ok(ContainerEnabledBuiltInVariableEnum::AmpPageLoadTime),
           "ampPageDownloadTime" => Ok(ContainerEnabledBuiltInVariableEnum::AmpPageDownloadTime),
           "ampGtmEvent" => Ok(ContainerEnabledBuiltInVariableEnum::AmpGtmEvent),
           "eventName" => Ok(ContainerEnabledBuiltInVariableEnum::EventName),
           "firebaseEventParameterCampaign" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaign),
           "firebaseEventParameterCampaignAclid" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignAclid),
           "firebaseEventParameterCampaignAnid" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignAnid),
           "firebaseEventParameterCampaignClickTimestamp" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignClickTimestamp),
           "firebaseEventParameterCampaignContent" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignContent),
           "firebaseEventParameterCampaignCp1" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignCp1),
           "firebaseEventParameterCampaignGclid" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignGclid),
           "firebaseEventParameterCampaignSource" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignSource),
           "firebaseEventParameterCampaignTerm" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCampaignTerm),
           "firebaseEventParameterCurrency" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterCurrency),
           "firebaseEventParameterDynamicLinkAcceptTime" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterDynamicLinkAcceptTime),
           "firebaseEventParameterDynamicLinkLinkid" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterDynamicLinkLinkid),
           "firebaseEventParameterNotificationMessageDeviceTime" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageDeviceTime),
           "firebaseEventParameterNotificationMessageId" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageId),
           "firebaseEventParameterNotificationMessageName" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageName),
           "firebaseEventParameterNotificationMessageTime" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationMessageTime),
           "firebaseEventParameterNotificationTopic" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterNotificationTopic),
           "firebaseEventParameterPreviousAppVersion" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterPreviousAppVersion),
           "firebaseEventParameterPreviousOsVersion" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterPreviousOsVersion),
           "firebaseEventParameterPrice" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterPrice),
           "firebaseEventParameterProductId" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterProductId),
           "firebaseEventParameterQuantity" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterQuantity),
           "firebaseEventParameterValue" => Ok(ContainerEnabledBuiltInVariableEnum::FirebaseEventParameterValue),
           "videoProvider" => Ok(ContainerEnabledBuiltInVariableEnum::VideoProvider),
           "videoUrl" => Ok(ContainerEnabledBuiltInVariableEnum::VideoUrl),
           "videoTitle" => Ok(ContainerEnabledBuiltInVariableEnum::VideoTitle),
           "videoDuration" => Ok(ContainerEnabledBuiltInVariableEnum::VideoDuration),
           "videoPercent" => Ok(ContainerEnabledBuiltInVariableEnum::VideoPercent),
           "videoVisible" => Ok(ContainerEnabledBuiltInVariableEnum::VideoVisible),
           "videoStatus" => Ok(ContainerEnabledBuiltInVariableEnum::VideoStatus),
           "videoCurrentTime" => Ok(ContainerEnabledBuiltInVariableEnum::VideoCurrentTime),
           "scrollDepthThreshold" => Ok(ContainerEnabledBuiltInVariableEnum::ScrollDepthThreshold),
           "scrollDepthUnits" => Ok(ContainerEnabledBuiltInVariableEnum::ScrollDepthUnits),
           "scrollDepthDirection" => Ok(ContainerEnabledBuiltInVariableEnum::ScrollDepthDirection),
           "elementVisibilityRatio" => Ok(ContainerEnabledBuiltInVariableEnum::ElementVisibilityRatio),
           "elementVisibilityTime" => Ok(ContainerEnabledBuiltInVariableEnum::ElementVisibilityTime),
           "elementVisibilityFirstTime" => Ok(ContainerEnabledBuiltInVariableEnum::ElementVisibilityFirstTime),
           "elementVisibilityRecentTime" => Ok(ContainerEnabledBuiltInVariableEnum::ElementVisibilityRecentTime),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContainerEnabledBuiltInVariableEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContainerUsageContextEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of Usage Contexts for the Container. Valid values include: web, android, ios. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
pub enum ContainerUsageContextEnum {
    
    /// "web"
    #[serde(rename="web")]
    Web,
    
    /// "android"
    #[serde(rename="android")]
    Android,
    
    /// "ios"
    #[serde(rename="ios")]
    Ios,
    
    /// "androidSdk5"
    #[serde(rename="androidSdk5")]
    AndroidSdk5,
    
    /// "iosSdk5"
    #[serde(rename="iosSdk5")]
    IosSdk5,
    
    /// "amp"
    #[serde(rename="amp")]
    Amp,
}

impl AsRef<str> for ContainerUsageContextEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContainerUsageContextEnum::Web => "web",
            ContainerUsageContextEnum::Android => "android",
            ContainerUsageContextEnum::Ios => "ios",
            ContainerUsageContextEnum::AndroidSdk5 => "androidSdk5",
            ContainerUsageContextEnum::IosSdk5 => "iosSdk5",
            ContainerUsageContextEnum::Amp => "amp",
        }
    }
}

impl std::convert::TryFrom< &str> for ContainerUsageContextEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "web" => Ok(ContainerUsageContextEnum::Web),
           "android" => Ok(ContainerUsageContextEnum::Android),
           "ios" => Ok(ContainerUsageContextEnum::Ios),
           "androidSdk5" => Ok(ContainerUsageContextEnum::AndroidSdk5),
           "iosSdk5" => Ok(ContainerUsageContextEnum::IosSdk5),
           "amp" => Ok(ContainerUsageContextEnum::Amp),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContainerUsageContextEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ContainerAccesPermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of Container permissions. Valid container permissions are: read, edit, delete, publish. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
pub enum ContainerAccesPermissionEnum {
    
    /// "read"
    #[serde(rename="read")]
    Read,
    
    /// "edit"
    #[serde(rename="edit")]
    Edit,
    
    /// "publish"
    #[serde(rename="publish")]
    Publish,
    
    /// "delete"
    #[serde(rename="delete")]
    Delete,
    
    /// "manage"
    #[serde(rename="manage")]
    Manage,
    
    /// "editWorkspace"
    #[serde(rename="editWorkspace")]
    EditWorkspace,
}

impl AsRef<str> for ContainerAccesPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContainerAccesPermissionEnum::Read => "read",
            ContainerAccesPermissionEnum::Edit => "edit",
            ContainerAccesPermissionEnum::Publish => "publish",
            ContainerAccesPermissionEnum::Delete => "delete",
            ContainerAccesPermissionEnum::Manage => "manage",
            ContainerAccesPermissionEnum::EditWorkspace => "editWorkspace",
        }
    }
}

impl std::convert::TryFrom< &str> for ContainerAccesPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "read" => Ok(ContainerAccesPermissionEnum::Read),
           "edit" => Ok(ContainerAccesPermissionEnum::Edit),
           "publish" => Ok(ContainerAccesPermissionEnum::Publish),
           "delete" => Ok(ContainerAccesPermissionEnum::Delete),
           "manage" => Ok(ContainerAccesPermissionEnum::Manage),
           "editWorkspace" => Ok(ContainerAccesPermissionEnum::EditWorkspace),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContainerAccesPermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this environment.
pub enum EnvironmentTypeEnum {
    

    /// Used for user defined environments.
    ///
    /// "user"
    #[serde(rename="user")]
    User,
    

    /// Used for Live environment, which points to the live published container version.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// Used for Latest environment, which points to the latest created container version.
    ///
    /// "latest"
    #[serde(rename="latest")]
    Latest,
    

    /// Used for Draft environment, which points to the single draft in the container.
    ///
    /// "draft"
    #[serde(rename="draft")]
    Draft,
}

impl AsRef<str> for EnvironmentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentTypeEnum::User => "user",
            EnvironmentTypeEnum::Live => "live",
            EnvironmentTypeEnum::Latest => "latest",
            EnvironmentTypeEnum::Draft => "draft",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "user" => Ok(EnvironmentTypeEnum::User),
           "live" => Ok(EnvironmentTypeEnum::Live),
           "latest" => Ok(EnvironmentTypeEnum::Latest),
           "draft" => Ok(EnvironmentTypeEnum::Draft),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ParameterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.variables.create @mutable tagmanager.accounts.containers.variables.update @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update @mutable tagmanager.accounts.containers.tags.create @mutable tagmanager.accounts.containers.tags.update
pub enum ParameterTypeEnum {
    

    /// May include variable references (such as "{{myVariable}}").
    ///
    /// "template"
    #[serde(rename="template")]
    Template,
    
    /// "integer"
    #[serde(rename="integer")]
    Integer,
    
    /// "boolean"
    #[serde(rename="boolean")]
    Boolean,
    
    /// "list"
    #[serde(rename="list")]
    List,
    
    /// "map"
    #[serde(rename="map")]
    Map,
    
    /// "triggerReference"
    #[serde(rename="triggerReference")]
    TriggerReference,
    
    /// "tagReference"
    #[serde(rename="tagReference")]
    TagReference,
}

impl AsRef<str> for ParameterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ParameterTypeEnum::Template => "template",
            ParameterTypeEnum::Integer => "integer",
            ParameterTypeEnum::Boolean => "boolean",
            ParameterTypeEnum::List => "list",
            ParameterTypeEnum::Map => "map",
            ParameterTypeEnum::TriggerReference => "triggerReference",
            ParameterTypeEnum::TagReference => "tagReference",
        }
    }
}

impl std::convert::TryFrom< &str> for ParameterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "template" => Ok(ParameterTypeEnum::Template),
           "integer" => Ok(ParameterTypeEnum::Integer),
           "boolean" => Ok(ParameterTypeEnum::Boolean),
           "list" => Ok(ParameterTypeEnum::List),
           "map" => Ok(ParameterTypeEnum::Map),
           "triggerReference" => Ok(ParameterTypeEnum::TriggerReference),
           "tagReference" => Ok(ParameterTypeEnum::TagReference),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ParameterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TagTagFiringOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Option to fire this tag.
pub enum TagTagFiringOptionEnum {
    

    /// Tag can be fired multiple times per event.
    ///
    /// "unlimited"
    #[serde(rename="unlimited")]
    Unlimited,
    

    /// Tag can only be fired per event but can be fired multiple times per load (e.g., app load or page load).
    ///
    /// "oncePerEvent"
    #[serde(rename="oncePerEvent")]
    OncePerEvent,
    

    /// Tag can only be fired per load (e.g., app load or page load).
    ///
    /// "oncePerLoad"
    #[serde(rename="oncePerLoad")]
    OncePerLoad,
}

impl AsRef<str> for TagTagFiringOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TagTagFiringOptionEnum::Unlimited => "unlimited",
            TagTagFiringOptionEnum::OncePerEvent => "oncePerEvent",
            TagTagFiringOptionEnum::OncePerLoad => "oncePerLoad",
        }
    }
}

impl std::convert::TryFrom< &str> for TagTagFiringOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "unlimited" => Ok(TagTagFiringOptionEnum::Unlimited),
           "oncePerEvent" => Ok(TagTagFiringOptionEnum::OncePerEvent),
           "oncePerLoad" => Ok(TagTagFiringOptionEnum::OncePerLoad),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TagTagFiringOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TriggerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.triggers.create @mutable tagmanager.accounts.containers.triggers.update
pub enum TriggerTypeEnum {
    
    /// "pageview"
    #[serde(rename="pageview")]
    Pageview,
    
    /// "domReady"
    #[serde(rename="domReady")]
    DomReady,
    
    /// "windowLoaded"
    #[serde(rename="windowLoaded")]
    WindowLoaded,
    
    /// "customEvent"
    #[serde(rename="customEvent")]
    CustomEvent,
    
    /// "triggerGroup"
    #[serde(rename="triggerGroup")]
    TriggerGroup,
    
    /// "always"
    #[serde(rename="always")]
    Always,
    
    /// "formSubmission"
    #[serde(rename="formSubmission")]
    FormSubmission,
    
    /// "click"
    #[serde(rename="click")]
    Click,
    
    /// "linkClick"
    #[serde(rename="linkClick")]
    LinkClick,
    
    /// "jsError"
    #[serde(rename="jsError")]
    JsError,
    
    /// "historyChange"
    #[serde(rename="historyChange")]
    HistoryChange,
    
    /// "timer"
    #[serde(rename="timer")]
    Timer,
    
    /// "ampClick"
    #[serde(rename="ampClick")]
    AmpClick,
    
    /// "ampTimer"
    #[serde(rename="ampTimer")]
    AmpTimer,
    
    /// "ampScroll"
    #[serde(rename="ampScroll")]
    AmpScroll,
    
    /// "ampVisibility"
    #[serde(rename="ampVisibility")]
    AmpVisibility,
    
    /// "youTubeVideo"
    #[serde(rename="youTubeVideo")]
    YouTubeVideo,
    
    /// "scrollDepth"
    #[serde(rename="scrollDepth")]
    ScrollDepth,
    
    /// "elementVisibility"
    #[serde(rename="elementVisibility")]
    ElementVisibility,
}

impl AsRef<str> for TriggerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TriggerTypeEnum::Pageview => "pageview",
            TriggerTypeEnum::DomReady => "domReady",
            TriggerTypeEnum::WindowLoaded => "windowLoaded",
            TriggerTypeEnum::CustomEvent => "customEvent",
            TriggerTypeEnum::TriggerGroup => "triggerGroup",
            TriggerTypeEnum::Always => "always",
            TriggerTypeEnum::FormSubmission => "formSubmission",
            TriggerTypeEnum::Click => "click",
            TriggerTypeEnum::LinkClick => "linkClick",
            TriggerTypeEnum::JsError => "jsError",
            TriggerTypeEnum::HistoryChange => "historyChange",
            TriggerTypeEnum::Timer => "timer",
            TriggerTypeEnum::AmpClick => "ampClick",
            TriggerTypeEnum::AmpTimer => "ampTimer",
            TriggerTypeEnum::AmpScroll => "ampScroll",
            TriggerTypeEnum::AmpVisibility => "ampVisibility",
            TriggerTypeEnum::YouTubeVideo => "youTubeVideo",
            TriggerTypeEnum::ScrollDepth => "scrollDepth",
            TriggerTypeEnum::ElementVisibility => "elementVisibility",
        }
    }
}

impl std::convert::TryFrom< &str> for TriggerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "pageview" => Ok(TriggerTypeEnum::Pageview),
           "domReady" => Ok(TriggerTypeEnum::DomReady),
           "windowLoaded" => Ok(TriggerTypeEnum::WindowLoaded),
           "customEvent" => Ok(TriggerTypeEnum::CustomEvent),
           "triggerGroup" => Ok(TriggerTypeEnum::TriggerGroup),
           "always" => Ok(TriggerTypeEnum::Always),
           "formSubmission" => Ok(TriggerTypeEnum::FormSubmission),
           "click" => Ok(TriggerTypeEnum::Click),
           "linkClick" => Ok(TriggerTypeEnum::LinkClick),
           "jsError" => Ok(TriggerTypeEnum::JsError),
           "historyChange" => Ok(TriggerTypeEnum::HistoryChange),
           "timer" => Ok(TriggerTypeEnum::Timer),
           "ampClick" => Ok(TriggerTypeEnum::AmpClick),
           "ampTimer" => Ok(TriggerTypeEnum::AmpTimer),
           "ampScroll" => Ok(TriggerTypeEnum::AmpScroll),
           "ampVisibility" => Ok(TriggerTypeEnum::AmpVisibility),
           "youTubeVideo" => Ok(TriggerTypeEnum::YouTubeVideo),
           "scrollDepth" => Ok(TriggerTypeEnum::ScrollDepth),
           "elementVisibility" => Ok(TriggerTypeEnum::ElementVisibility),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TriggerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


