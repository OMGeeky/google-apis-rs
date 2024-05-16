use super::*;



// region AccountAccesPermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the user has no access, user access, or admin access to an account. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
pub enum AccountAccesPermissionEnum {
    
    /// "accountPermissionUnspecified"
    #[serde(rename="accountPermissionUnspecified")]
    AccountPermissionUnspecified,
    
    /// "noAccess"
    #[serde(rename="noAccess")]
    NoAccess,
    
    /// "user"
    #[serde(rename="user")]
    User,
    
    /// "admin"
    #[serde(rename="admin")]
    Admin,
}

impl AsRef<str> for AccountAccesPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountAccesPermissionEnum::AccountPermissionUnspecified => "accountPermissionUnspecified",
            AccountAccesPermissionEnum::NoAccess => "noAccess",
            AccountAccesPermissionEnum::User => "user",
            AccountAccesPermissionEnum::Admin => "admin",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountAccesPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "accountPermissionUnspecified" => Ok(AccountAccesPermissionEnum::AccountPermissionUnspecified),
           "noAccess" => Ok(AccountAccesPermissionEnum::NoAccess),
           "user" => Ok(AccountAccesPermissionEnum::User),
           "admin" => Ok(AccountAccesPermissionEnum::Admin),
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


// region BuiltInVariableTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of built-in variable. @required.tagmanager.accounts.containers.workspaces.built_in_variable.update @mutable tagmanager.accounts.containers.workspaces.built_in_variable.update
pub enum BuiltInVariableTypeEnum {
    
    /// "builtInVariableTypeUnspecified"
    #[serde(rename="builtInVariableTypeUnspecified")]
    BuiltInVariableTypeUnspecified,
    
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
    
    /// "environmentName"
    #[serde(rename="environmentName")]
    EnvironmentName,
    
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
    
    /// "requestPath"
    #[serde(rename="requestPath")]
    RequestPath,
    
    /// "requestMethod"
    #[serde(rename="requestMethod")]
    RequestMethod,
    
    /// "clientName"
    #[serde(rename="clientName")]
    ClientName,
    
    /// "queryString"
    #[serde(rename="queryString")]
    QueryString,
    
    /// "serverPageLocationUrl"
    #[serde(rename="serverPageLocationUrl")]
    ServerPageLocationUrl,
    
    /// "serverPageLocationPath"
    #[serde(rename="serverPageLocationPath")]
    ServerPageLocationPath,
    
    /// "serverPageLocationHostname"
    #[serde(rename="serverPageLocationHostname")]
    ServerPageLocationHostname,
    
    /// "visitorRegion"
    #[serde(rename="visitorRegion")]
    VisitorRegion,
}

impl AsRef<str> for BuiltInVariableTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BuiltInVariableTypeEnum::BuiltInVariableTypeUnspecified => "builtInVariableTypeUnspecified",
            BuiltInVariableTypeEnum::PageUrl => "pageUrl",
            BuiltInVariableTypeEnum::PageHostname => "pageHostname",
            BuiltInVariableTypeEnum::PagePath => "pagePath",
            BuiltInVariableTypeEnum::Referrer => "referrer",
            BuiltInVariableTypeEnum::Event => "event",
            BuiltInVariableTypeEnum::ClickElement => "clickElement",
            BuiltInVariableTypeEnum::ClickClasses => "clickClasses",
            BuiltInVariableTypeEnum::ClickId => "clickId",
            BuiltInVariableTypeEnum::ClickTarget => "clickTarget",
            BuiltInVariableTypeEnum::ClickUrl => "clickUrl",
            BuiltInVariableTypeEnum::ClickText => "clickText",
            BuiltInVariableTypeEnum::FirstPartyServingUrl => "firstPartyServingUrl",
            BuiltInVariableTypeEnum::FormElement => "formElement",
            BuiltInVariableTypeEnum::FormClasses => "formClasses",
            BuiltInVariableTypeEnum::FormId => "formId",
            BuiltInVariableTypeEnum::FormTarget => "formTarget",
            BuiltInVariableTypeEnum::FormUrl => "formUrl",
            BuiltInVariableTypeEnum::FormText => "formText",
            BuiltInVariableTypeEnum::ErrorMessage => "errorMessage",
            BuiltInVariableTypeEnum::ErrorUrl => "errorUrl",
            BuiltInVariableTypeEnum::ErrorLine => "errorLine",
            BuiltInVariableTypeEnum::NewHistoryUrl => "newHistoryUrl",
            BuiltInVariableTypeEnum::OldHistoryUrl => "oldHistoryUrl",
            BuiltInVariableTypeEnum::NewHistoryFragment => "newHistoryFragment",
            BuiltInVariableTypeEnum::OldHistoryFragment => "oldHistoryFragment",
            BuiltInVariableTypeEnum::NewHistoryState => "newHistoryState",
            BuiltInVariableTypeEnum::OldHistoryState => "oldHistoryState",
            BuiltInVariableTypeEnum::HistorySource => "historySource",
            BuiltInVariableTypeEnum::ContainerVersion => "containerVersion",
            BuiltInVariableTypeEnum::DebugMode => "debugMode",
            BuiltInVariableTypeEnum::RandomNumber => "randomNumber",
            BuiltInVariableTypeEnum::ContainerId => "containerId",
            BuiltInVariableTypeEnum::AppId => "appId",
            BuiltInVariableTypeEnum::AppName => "appName",
            BuiltInVariableTypeEnum::AppVersionCode => "appVersionCode",
            BuiltInVariableTypeEnum::AppVersionName => "appVersionName",
            BuiltInVariableTypeEnum::Language => "language",
            BuiltInVariableTypeEnum::OsVersion => "osVersion",
            BuiltInVariableTypeEnum::Platform => "platform",
            BuiltInVariableTypeEnum::SdkVersion => "sdkVersion",
            BuiltInVariableTypeEnum::DeviceName => "deviceName",
            BuiltInVariableTypeEnum::Resolution => "resolution",
            BuiltInVariableTypeEnum::AdvertiserId => "advertiserId",
            BuiltInVariableTypeEnum::AdvertisingTrackingEnabled => "advertisingTrackingEnabled",
            BuiltInVariableTypeEnum::HtmlId => "htmlId",
            BuiltInVariableTypeEnum::EnvironmentName => "environmentName",
            BuiltInVariableTypeEnum::AmpBrowserLanguage => "ampBrowserLanguage",
            BuiltInVariableTypeEnum::AmpCanonicalPath => "ampCanonicalPath",
            BuiltInVariableTypeEnum::AmpCanonicalUrl => "ampCanonicalUrl",
            BuiltInVariableTypeEnum::AmpCanonicalHost => "ampCanonicalHost",
            BuiltInVariableTypeEnum::AmpReferrer => "ampReferrer",
            BuiltInVariableTypeEnum::AmpTitle => "ampTitle",
            BuiltInVariableTypeEnum::AmpClientId => "ampClientId",
            BuiltInVariableTypeEnum::AmpClientTimezone => "ampClientTimezone",
            BuiltInVariableTypeEnum::AmpClientTimestamp => "ampClientTimestamp",
            BuiltInVariableTypeEnum::AmpClientScreenWidth => "ampClientScreenWidth",
            BuiltInVariableTypeEnum::AmpClientScreenHeight => "ampClientScreenHeight",
            BuiltInVariableTypeEnum::AmpClientScrollX => "ampClientScrollX",
            BuiltInVariableTypeEnum::AmpClientScrollY => "ampClientScrollY",
            BuiltInVariableTypeEnum::AmpClientMaxScrollX => "ampClientMaxScrollX",
            BuiltInVariableTypeEnum::AmpClientMaxScrollY => "ampClientMaxScrollY",
            BuiltInVariableTypeEnum::AmpTotalEngagedTime => "ampTotalEngagedTime",
            BuiltInVariableTypeEnum::AmpPageViewId => "ampPageViewId",
            BuiltInVariableTypeEnum::AmpPageLoadTime => "ampPageLoadTime",
            BuiltInVariableTypeEnum::AmpPageDownloadTime => "ampPageDownloadTime",
            BuiltInVariableTypeEnum::AmpGtmEvent => "ampGtmEvent",
            BuiltInVariableTypeEnum::EventName => "eventName",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaign => "firebaseEventParameterCampaign",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignAclid => "firebaseEventParameterCampaignAclid",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignAnid => "firebaseEventParameterCampaignAnid",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignClickTimestamp => "firebaseEventParameterCampaignClickTimestamp",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignContent => "firebaseEventParameterCampaignContent",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignCp1 => "firebaseEventParameterCampaignCp1",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignGclid => "firebaseEventParameterCampaignGclid",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignSource => "firebaseEventParameterCampaignSource",
            BuiltInVariableTypeEnum::FirebaseEventParameterCampaignTerm => "firebaseEventParameterCampaignTerm",
            BuiltInVariableTypeEnum::FirebaseEventParameterCurrency => "firebaseEventParameterCurrency",
            BuiltInVariableTypeEnum::FirebaseEventParameterDynamicLinkAcceptTime => "firebaseEventParameterDynamicLinkAcceptTime",
            BuiltInVariableTypeEnum::FirebaseEventParameterDynamicLinkLinkid => "firebaseEventParameterDynamicLinkLinkid",
            BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageDeviceTime => "firebaseEventParameterNotificationMessageDeviceTime",
            BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageId => "firebaseEventParameterNotificationMessageId",
            BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageName => "firebaseEventParameterNotificationMessageName",
            BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageTime => "firebaseEventParameterNotificationMessageTime",
            BuiltInVariableTypeEnum::FirebaseEventParameterNotificationTopic => "firebaseEventParameterNotificationTopic",
            BuiltInVariableTypeEnum::FirebaseEventParameterPreviousAppVersion => "firebaseEventParameterPreviousAppVersion",
            BuiltInVariableTypeEnum::FirebaseEventParameterPreviousOsVersion => "firebaseEventParameterPreviousOsVersion",
            BuiltInVariableTypeEnum::FirebaseEventParameterPrice => "firebaseEventParameterPrice",
            BuiltInVariableTypeEnum::FirebaseEventParameterProductId => "firebaseEventParameterProductId",
            BuiltInVariableTypeEnum::FirebaseEventParameterQuantity => "firebaseEventParameterQuantity",
            BuiltInVariableTypeEnum::FirebaseEventParameterValue => "firebaseEventParameterValue",
            BuiltInVariableTypeEnum::VideoProvider => "videoProvider",
            BuiltInVariableTypeEnum::VideoUrl => "videoUrl",
            BuiltInVariableTypeEnum::VideoTitle => "videoTitle",
            BuiltInVariableTypeEnum::VideoDuration => "videoDuration",
            BuiltInVariableTypeEnum::VideoPercent => "videoPercent",
            BuiltInVariableTypeEnum::VideoVisible => "videoVisible",
            BuiltInVariableTypeEnum::VideoStatus => "videoStatus",
            BuiltInVariableTypeEnum::VideoCurrentTime => "videoCurrentTime",
            BuiltInVariableTypeEnum::ScrollDepthThreshold => "scrollDepthThreshold",
            BuiltInVariableTypeEnum::ScrollDepthUnits => "scrollDepthUnits",
            BuiltInVariableTypeEnum::ScrollDepthDirection => "scrollDepthDirection",
            BuiltInVariableTypeEnum::ElementVisibilityRatio => "elementVisibilityRatio",
            BuiltInVariableTypeEnum::ElementVisibilityTime => "elementVisibilityTime",
            BuiltInVariableTypeEnum::ElementVisibilityFirstTime => "elementVisibilityFirstTime",
            BuiltInVariableTypeEnum::ElementVisibilityRecentTime => "elementVisibilityRecentTime",
            BuiltInVariableTypeEnum::RequestPath => "requestPath",
            BuiltInVariableTypeEnum::RequestMethod => "requestMethod",
            BuiltInVariableTypeEnum::ClientName => "clientName",
            BuiltInVariableTypeEnum::QueryString => "queryString",
            BuiltInVariableTypeEnum::ServerPageLocationUrl => "serverPageLocationUrl",
            BuiltInVariableTypeEnum::ServerPageLocationPath => "serverPageLocationPath",
            BuiltInVariableTypeEnum::ServerPageLocationHostname => "serverPageLocationHostname",
            BuiltInVariableTypeEnum::VisitorRegion => "visitorRegion",
        }
    }
}

impl std::convert::TryFrom< &str> for BuiltInVariableTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "builtInVariableTypeUnspecified" => Ok(BuiltInVariableTypeEnum::BuiltInVariableTypeUnspecified),
           "pageUrl" => Ok(BuiltInVariableTypeEnum::PageUrl),
           "pageHostname" => Ok(BuiltInVariableTypeEnum::PageHostname),
           "pagePath" => Ok(BuiltInVariableTypeEnum::PagePath),
           "referrer" => Ok(BuiltInVariableTypeEnum::Referrer),
           "event" => Ok(BuiltInVariableTypeEnum::Event),
           "clickElement" => Ok(BuiltInVariableTypeEnum::ClickElement),
           "clickClasses" => Ok(BuiltInVariableTypeEnum::ClickClasses),
           "clickId" => Ok(BuiltInVariableTypeEnum::ClickId),
           "clickTarget" => Ok(BuiltInVariableTypeEnum::ClickTarget),
           "clickUrl" => Ok(BuiltInVariableTypeEnum::ClickUrl),
           "clickText" => Ok(BuiltInVariableTypeEnum::ClickText),
           "firstPartyServingUrl" => Ok(BuiltInVariableTypeEnum::FirstPartyServingUrl),
           "formElement" => Ok(BuiltInVariableTypeEnum::FormElement),
           "formClasses" => Ok(BuiltInVariableTypeEnum::FormClasses),
           "formId" => Ok(BuiltInVariableTypeEnum::FormId),
           "formTarget" => Ok(BuiltInVariableTypeEnum::FormTarget),
           "formUrl" => Ok(BuiltInVariableTypeEnum::FormUrl),
           "formText" => Ok(BuiltInVariableTypeEnum::FormText),
           "errorMessage" => Ok(BuiltInVariableTypeEnum::ErrorMessage),
           "errorUrl" => Ok(BuiltInVariableTypeEnum::ErrorUrl),
           "errorLine" => Ok(BuiltInVariableTypeEnum::ErrorLine),
           "newHistoryUrl" => Ok(BuiltInVariableTypeEnum::NewHistoryUrl),
           "oldHistoryUrl" => Ok(BuiltInVariableTypeEnum::OldHistoryUrl),
           "newHistoryFragment" => Ok(BuiltInVariableTypeEnum::NewHistoryFragment),
           "oldHistoryFragment" => Ok(BuiltInVariableTypeEnum::OldHistoryFragment),
           "newHistoryState" => Ok(BuiltInVariableTypeEnum::NewHistoryState),
           "oldHistoryState" => Ok(BuiltInVariableTypeEnum::OldHistoryState),
           "historySource" => Ok(BuiltInVariableTypeEnum::HistorySource),
           "containerVersion" => Ok(BuiltInVariableTypeEnum::ContainerVersion),
           "debugMode" => Ok(BuiltInVariableTypeEnum::DebugMode),
           "randomNumber" => Ok(BuiltInVariableTypeEnum::RandomNumber),
           "containerId" => Ok(BuiltInVariableTypeEnum::ContainerId),
           "appId" => Ok(BuiltInVariableTypeEnum::AppId),
           "appName" => Ok(BuiltInVariableTypeEnum::AppName),
           "appVersionCode" => Ok(BuiltInVariableTypeEnum::AppVersionCode),
           "appVersionName" => Ok(BuiltInVariableTypeEnum::AppVersionName),
           "language" => Ok(BuiltInVariableTypeEnum::Language),
           "osVersion" => Ok(BuiltInVariableTypeEnum::OsVersion),
           "platform" => Ok(BuiltInVariableTypeEnum::Platform),
           "sdkVersion" => Ok(BuiltInVariableTypeEnum::SdkVersion),
           "deviceName" => Ok(BuiltInVariableTypeEnum::DeviceName),
           "resolution" => Ok(BuiltInVariableTypeEnum::Resolution),
           "advertiserId" => Ok(BuiltInVariableTypeEnum::AdvertiserId),
           "advertisingTrackingEnabled" => Ok(BuiltInVariableTypeEnum::AdvertisingTrackingEnabled),
           "htmlId" => Ok(BuiltInVariableTypeEnum::HtmlId),
           "environmentName" => Ok(BuiltInVariableTypeEnum::EnvironmentName),
           "ampBrowserLanguage" => Ok(BuiltInVariableTypeEnum::AmpBrowserLanguage),
           "ampCanonicalPath" => Ok(BuiltInVariableTypeEnum::AmpCanonicalPath),
           "ampCanonicalUrl" => Ok(BuiltInVariableTypeEnum::AmpCanonicalUrl),
           "ampCanonicalHost" => Ok(BuiltInVariableTypeEnum::AmpCanonicalHost),
           "ampReferrer" => Ok(BuiltInVariableTypeEnum::AmpReferrer),
           "ampTitle" => Ok(BuiltInVariableTypeEnum::AmpTitle),
           "ampClientId" => Ok(BuiltInVariableTypeEnum::AmpClientId),
           "ampClientTimezone" => Ok(BuiltInVariableTypeEnum::AmpClientTimezone),
           "ampClientTimestamp" => Ok(BuiltInVariableTypeEnum::AmpClientTimestamp),
           "ampClientScreenWidth" => Ok(BuiltInVariableTypeEnum::AmpClientScreenWidth),
           "ampClientScreenHeight" => Ok(BuiltInVariableTypeEnum::AmpClientScreenHeight),
           "ampClientScrollX" => Ok(BuiltInVariableTypeEnum::AmpClientScrollX),
           "ampClientScrollY" => Ok(BuiltInVariableTypeEnum::AmpClientScrollY),
           "ampClientMaxScrollX" => Ok(BuiltInVariableTypeEnum::AmpClientMaxScrollX),
           "ampClientMaxScrollY" => Ok(BuiltInVariableTypeEnum::AmpClientMaxScrollY),
           "ampTotalEngagedTime" => Ok(BuiltInVariableTypeEnum::AmpTotalEngagedTime),
           "ampPageViewId" => Ok(BuiltInVariableTypeEnum::AmpPageViewId),
           "ampPageLoadTime" => Ok(BuiltInVariableTypeEnum::AmpPageLoadTime),
           "ampPageDownloadTime" => Ok(BuiltInVariableTypeEnum::AmpPageDownloadTime),
           "ampGtmEvent" => Ok(BuiltInVariableTypeEnum::AmpGtmEvent),
           "eventName" => Ok(BuiltInVariableTypeEnum::EventName),
           "firebaseEventParameterCampaign" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaign),
           "firebaseEventParameterCampaignAclid" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignAclid),
           "firebaseEventParameterCampaignAnid" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignAnid),
           "firebaseEventParameterCampaignClickTimestamp" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignClickTimestamp),
           "firebaseEventParameterCampaignContent" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignContent),
           "firebaseEventParameterCampaignCp1" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignCp1),
           "firebaseEventParameterCampaignGclid" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignGclid),
           "firebaseEventParameterCampaignSource" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignSource),
           "firebaseEventParameterCampaignTerm" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCampaignTerm),
           "firebaseEventParameterCurrency" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterCurrency),
           "firebaseEventParameterDynamicLinkAcceptTime" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterDynamicLinkAcceptTime),
           "firebaseEventParameterDynamicLinkLinkid" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterDynamicLinkLinkid),
           "firebaseEventParameterNotificationMessageDeviceTime" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageDeviceTime),
           "firebaseEventParameterNotificationMessageId" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageId),
           "firebaseEventParameterNotificationMessageName" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageName),
           "firebaseEventParameterNotificationMessageTime" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterNotificationMessageTime),
           "firebaseEventParameterNotificationTopic" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterNotificationTopic),
           "firebaseEventParameterPreviousAppVersion" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterPreviousAppVersion),
           "firebaseEventParameterPreviousOsVersion" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterPreviousOsVersion),
           "firebaseEventParameterPrice" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterPrice),
           "firebaseEventParameterProductId" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterProductId),
           "firebaseEventParameterQuantity" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterQuantity),
           "firebaseEventParameterValue" => Ok(BuiltInVariableTypeEnum::FirebaseEventParameterValue),
           "videoProvider" => Ok(BuiltInVariableTypeEnum::VideoProvider),
           "videoUrl" => Ok(BuiltInVariableTypeEnum::VideoUrl),
           "videoTitle" => Ok(BuiltInVariableTypeEnum::VideoTitle),
           "videoDuration" => Ok(BuiltInVariableTypeEnum::VideoDuration),
           "videoPercent" => Ok(BuiltInVariableTypeEnum::VideoPercent),
           "videoVisible" => Ok(BuiltInVariableTypeEnum::VideoVisible),
           "videoStatus" => Ok(BuiltInVariableTypeEnum::VideoStatus),
           "videoCurrentTime" => Ok(BuiltInVariableTypeEnum::VideoCurrentTime),
           "scrollDepthThreshold" => Ok(BuiltInVariableTypeEnum::ScrollDepthThreshold),
           "scrollDepthUnits" => Ok(BuiltInVariableTypeEnum::ScrollDepthUnits),
           "scrollDepthDirection" => Ok(BuiltInVariableTypeEnum::ScrollDepthDirection),
           "elementVisibilityRatio" => Ok(BuiltInVariableTypeEnum::ElementVisibilityRatio),
           "elementVisibilityTime" => Ok(BuiltInVariableTypeEnum::ElementVisibilityTime),
           "elementVisibilityFirstTime" => Ok(BuiltInVariableTypeEnum::ElementVisibilityFirstTime),
           "elementVisibilityRecentTime" => Ok(BuiltInVariableTypeEnum::ElementVisibilityRecentTime),
           "requestPath" => Ok(BuiltInVariableTypeEnum::RequestPath),
           "requestMethod" => Ok(BuiltInVariableTypeEnum::RequestMethod),
           "clientName" => Ok(BuiltInVariableTypeEnum::ClientName),
           "queryString" => Ok(BuiltInVariableTypeEnum::QueryString),
           "serverPageLocationUrl" => Ok(BuiltInVariableTypeEnum::ServerPageLocationUrl),
           "serverPageLocationPath" => Ok(BuiltInVariableTypeEnum::ServerPageLocationPath),
           "serverPageLocationHostname" => Ok(BuiltInVariableTypeEnum::ServerPageLocationHostname),
           "visitorRegion" => Ok(BuiltInVariableTypeEnum::VisitorRegion),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BuiltInVariableTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ConditionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of operator for this condition. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
pub enum ConditionTypeEnum {
    
    /// "conditionTypeUnspecified"
    #[serde(rename="conditionTypeUnspecified")]
    ConditionTypeUnspecified,
    
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
    

    /// NOTE(lanzone): When defining a ConditionType here, don't forget to also define a matching PredicateType (in condition.proto).
    ///
    /// "urlMatches"
    #[serde(rename="urlMatches")]
    UrlMatches,
}

impl AsRef<str> for ConditionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionTypeEnum::ConditionTypeUnspecified => "conditionTypeUnspecified",
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
           "conditionTypeUnspecified" => Ok(ConditionTypeEnum::ConditionTypeUnspecified),
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


// region ContainerUsageContextEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// List of Usage Contexts for the Container. Valid values include: web, android, or ios. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update
pub enum ContainerUsageContextEnum {
    
    /// "usageContextUnspecified"
    #[serde(rename="usageContextUnspecified")]
    UsageContextUnspecified,
    
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
    
    /// "server"
    #[serde(rename="server")]
    Server,
}

impl AsRef<str> for ContainerUsageContextEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContainerUsageContextEnum::UsageContextUnspecified => "usageContextUnspecified",
            ContainerUsageContextEnum::Web => "web",
            ContainerUsageContextEnum::Android => "android",
            ContainerUsageContextEnum::Ios => "ios",
            ContainerUsageContextEnum::AndroidSdk5 => "androidSdk5",
            ContainerUsageContextEnum::IosSdk5 => "iosSdk5",
            ContainerUsageContextEnum::Amp => "amp",
            ContainerUsageContextEnum::Server => "server",
        }
    }
}

impl std::convert::TryFrom< &str> for ContainerUsageContextEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "usageContextUnspecified" => Ok(ContainerUsageContextEnum::UsageContextUnspecified),
           "web" => Ok(ContainerUsageContextEnum::Web),
           "android" => Ok(ContainerUsageContextEnum::Android),
           "ios" => Ok(ContainerUsageContextEnum::Ios),
           "androidSdk5" => Ok(ContainerUsageContextEnum::AndroidSdk5),
           "iosSdk5" => Ok(ContainerUsageContextEnum::IosSdk5),
           "amp" => Ok(ContainerUsageContextEnum::Amp),
           "server" => Ok(ContainerUsageContextEnum::Server),
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
/// List of Container permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update
pub enum ContainerAccesPermissionEnum {
    
    /// "containerPermissionUnspecified"
    #[serde(rename="containerPermissionUnspecified")]
    ContainerPermissionUnspecified,
    
    /// "noAccess"
    #[serde(rename="noAccess")]
    NoAccess,
    
    /// "read"
    #[serde(rename="read")]
    Read,
    
    /// "edit"
    #[serde(rename="edit")]
    Edit,
    
    /// "approve"
    #[serde(rename="approve")]
    Approve,
    
    /// "publish"
    #[serde(rename="publish")]
    Publish,
}

impl AsRef<str> for ContainerAccesPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContainerAccesPermissionEnum::ContainerPermissionUnspecified => "containerPermissionUnspecified",
            ContainerAccesPermissionEnum::NoAccess => "noAccess",
            ContainerAccesPermissionEnum::Read => "read",
            ContainerAccesPermissionEnum::Edit => "edit",
            ContainerAccesPermissionEnum::Approve => "approve",
            ContainerAccesPermissionEnum::Publish => "publish",
        }
    }
}

impl std::convert::TryFrom< &str> for ContainerAccesPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "containerPermissionUnspecified" => Ok(ContainerAccesPermissionEnum::ContainerPermissionUnspecified),
           "noAccess" => Ok(ContainerAccesPermissionEnum::NoAccess),
           "read" => Ok(ContainerAccesPermissionEnum::Read),
           "edit" => Ok(ContainerAccesPermissionEnum::Edit),
           "approve" => Ok(ContainerAccesPermissionEnum::Approve),
           "publish" => Ok(ContainerAccesPermissionEnum::Publish),
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


// region EntityChangeStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Represents how the entity has been changed in the workspace.
pub enum EntityChangeStatusEnum {
    
    /// "changeStatusUnspecified"
    #[serde(rename="changeStatusUnspecified")]
    ChangeStatusUnspecified,
    

    /// The entity has never been changed.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is added to the workspace.
    ///
    /// "added"
    #[serde(rename="added")]
    Added,
    

    /// The entity is deleted from the workspace.
    ///
    /// "deleted"
    #[serde(rename="deleted")]
    Deleted,
    

    /// The entity has been updated in the workspace.
    ///
    /// "updated"
    #[serde(rename="updated")]
    Updated,
}

impl AsRef<str> for EntityChangeStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EntityChangeStatusEnum::ChangeStatusUnspecified => "changeStatusUnspecified",
            EntityChangeStatusEnum::None => "none",
            EntityChangeStatusEnum::Added => "added",
            EntityChangeStatusEnum::Deleted => "deleted",
            EntityChangeStatusEnum::Updated => "updated",
        }
    }
}

impl std::convert::TryFrom< &str> for EntityChangeStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "changeStatusUnspecified" => Ok(EntityChangeStatusEnum::ChangeStatusUnspecified),
           "none" => Ok(EntityChangeStatusEnum::None),
           "added" => Ok(EntityChangeStatusEnum::Added),
           "deleted" => Ok(EntityChangeStatusEnum::Deleted),
           "updated" => Ok(EntityChangeStatusEnum::Updated),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EntityChangeStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this environment.
pub enum EnvironmentTypeEnum {
    

    /// Points to a user defined environment.
    ///
    /// "user"
    #[serde(rename="user")]
    User,
    

    /// Points to the current live container version.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// Points to the latest container version.
    ///
    /// "latest"
    #[serde(rename="latest")]
    Latest,
    

    /// Automatically managed environment that points to a workspace preview or version created by a workspace.
    ///
    /// "workspace"
    #[serde(rename="workspace")]
    Workspace,
}

impl AsRef<str> for EnvironmentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentTypeEnum::User => "user",
            EnvironmentTypeEnum::Live => "live",
            EnvironmentTypeEnum::Latest => "latest",
            EnvironmentTypeEnum::Workspace => "workspace",
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
           "workspace" => Ok(EnvironmentTypeEnum::Workspace),
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
/// The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update
pub enum ParameterTypeEnum {
    
    /// "typeUnspecified"
    #[serde(rename="typeUnspecified")]
    TypeUnspecified,
    

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
            ParameterTypeEnum::TypeUnspecified => "typeUnspecified",
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
           "typeUnspecified" => Ok(ParameterTypeEnum::TypeUnspecified),
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
    
    /// "tagFiringOptionUnspecified"
    #[serde(rename="tagFiringOptionUnspecified")]
    TagFiringOptionUnspecified,
    

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
            TagTagFiringOptionEnum::TagFiringOptionUnspecified => "tagFiringOptionUnspecified",
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
           "tagFiringOptionUnspecified" => Ok(TagTagFiringOptionEnum::TagFiringOptionUnspecified),
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


// region TagConsentSettingConsentStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The tag's consent status. If set to NEEDED, the runtime will check that the consent types specified by the consent_type field have been granted.
pub enum TagConsentSettingConsentStatusEnum {
    

    /// Default value where user has not specified any setting on it.
    ///
    /// "notSet"
    #[serde(rename="notSet")]
    NotSet,
    

    /// Tag doesn't require any additional consent settings.
    ///
    /// "notNeeded"
    #[serde(rename="notNeeded")]
    NotNeeded,
    

    /// Tag requires additional consent settings.
    ///
    /// "needed"
    #[serde(rename="needed")]
    Needed,
}

impl AsRef<str> for TagConsentSettingConsentStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TagConsentSettingConsentStatusEnum::NotSet => "notSet",
            TagConsentSettingConsentStatusEnum::NotNeeded => "notNeeded",
            TagConsentSettingConsentStatusEnum::Needed => "needed",
        }
    }
}

impl std::convert::TryFrom< &str> for TagConsentSettingConsentStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "notSet" => Ok(TagConsentSettingConsentStatusEnum::NotSet),
           "notNeeded" => Ok(TagConsentSettingConsentStatusEnum::NotNeeded),
           "needed" => Ok(TagConsentSettingConsentStatusEnum::Needed),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TagConsentSettingConsentStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TriggerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update
pub enum TriggerTypeEnum {
    
    /// "eventTypeUnspecified"
    #[serde(rename="eventTypeUnspecified")]
    EventTypeUnspecified,
    
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
    
    /// "init"
    #[serde(rename="init")]
    Init,
    
    /// "consentInit"
    #[serde(rename="consentInit")]
    ConsentInit,
    
    /// "serverPageview"
    #[serde(rename="serverPageview")]
    ServerPageview,
    
    /// "always"
    #[serde(rename="always")]
    Always,
    
    /// "firebaseAppException"
    #[serde(rename="firebaseAppException")]
    FirebaseAppException,
    
    /// "firebaseAppUpdate"
    #[serde(rename="firebaseAppUpdate")]
    FirebaseAppUpdate,
    
    /// "firebaseCampaign"
    #[serde(rename="firebaseCampaign")]
    FirebaseCampaign,
    
    /// "firebaseFirstOpen"
    #[serde(rename="firebaseFirstOpen")]
    FirebaseFirstOpen,
    
    /// "firebaseInAppPurchase"
    #[serde(rename="firebaseInAppPurchase")]
    FirebaseInAppPurchase,
    
    /// "firebaseNotificationDismiss"
    #[serde(rename="firebaseNotificationDismiss")]
    FirebaseNotificationDismiss,
    
    /// "firebaseNotificationForeground"
    #[serde(rename="firebaseNotificationForeground")]
    FirebaseNotificationForeground,
    
    /// "firebaseNotificationOpen"
    #[serde(rename="firebaseNotificationOpen")]
    FirebaseNotificationOpen,
    
    /// "firebaseNotificationReceive"
    #[serde(rename="firebaseNotificationReceive")]
    FirebaseNotificationReceive,
    
    /// "firebaseOsUpdate"
    #[serde(rename="firebaseOsUpdate")]
    FirebaseOsUpdate,
    
    /// "firebaseSessionStart"
    #[serde(rename="firebaseSessionStart")]
    FirebaseSessionStart,
    
    /// "firebaseUserEngagement"
    #[serde(rename="firebaseUserEngagement")]
    FirebaseUserEngagement,
    
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
            TriggerTypeEnum::EventTypeUnspecified => "eventTypeUnspecified",
            TriggerTypeEnum::Pageview => "pageview",
            TriggerTypeEnum::DomReady => "domReady",
            TriggerTypeEnum::WindowLoaded => "windowLoaded",
            TriggerTypeEnum::CustomEvent => "customEvent",
            TriggerTypeEnum::TriggerGroup => "triggerGroup",
            TriggerTypeEnum::Init => "init",
            TriggerTypeEnum::ConsentInit => "consentInit",
            TriggerTypeEnum::ServerPageview => "serverPageview",
            TriggerTypeEnum::Always => "always",
            TriggerTypeEnum::FirebaseAppException => "firebaseAppException",
            TriggerTypeEnum::FirebaseAppUpdate => "firebaseAppUpdate",
            TriggerTypeEnum::FirebaseCampaign => "firebaseCampaign",
            TriggerTypeEnum::FirebaseFirstOpen => "firebaseFirstOpen",
            TriggerTypeEnum::FirebaseInAppPurchase => "firebaseInAppPurchase",
            TriggerTypeEnum::FirebaseNotificationDismiss => "firebaseNotificationDismiss",
            TriggerTypeEnum::FirebaseNotificationForeground => "firebaseNotificationForeground",
            TriggerTypeEnum::FirebaseNotificationOpen => "firebaseNotificationOpen",
            TriggerTypeEnum::FirebaseNotificationReceive => "firebaseNotificationReceive",
            TriggerTypeEnum::FirebaseOsUpdate => "firebaseOsUpdate",
            TriggerTypeEnum::FirebaseSessionStart => "firebaseSessionStart",
            TriggerTypeEnum::FirebaseUserEngagement => "firebaseUserEngagement",
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
           "eventTypeUnspecified" => Ok(TriggerTypeEnum::EventTypeUnspecified),
           "pageview" => Ok(TriggerTypeEnum::Pageview),
           "domReady" => Ok(TriggerTypeEnum::DomReady),
           "windowLoaded" => Ok(TriggerTypeEnum::WindowLoaded),
           "customEvent" => Ok(TriggerTypeEnum::CustomEvent),
           "triggerGroup" => Ok(TriggerTypeEnum::TriggerGroup),
           "init" => Ok(TriggerTypeEnum::Init),
           "consentInit" => Ok(TriggerTypeEnum::ConsentInit),
           "serverPageview" => Ok(TriggerTypeEnum::ServerPageview),
           "always" => Ok(TriggerTypeEnum::Always),
           "firebaseAppException" => Ok(TriggerTypeEnum::FirebaseAppException),
           "firebaseAppUpdate" => Ok(TriggerTypeEnum::FirebaseAppUpdate),
           "firebaseCampaign" => Ok(TriggerTypeEnum::FirebaseCampaign),
           "firebaseFirstOpen" => Ok(TriggerTypeEnum::FirebaseFirstOpen),
           "firebaseInAppPurchase" => Ok(TriggerTypeEnum::FirebaseInAppPurchase),
           "firebaseNotificationDismiss" => Ok(TriggerTypeEnum::FirebaseNotificationDismiss),
           "firebaseNotificationForeground" => Ok(TriggerTypeEnum::FirebaseNotificationForeground),
           "firebaseNotificationOpen" => Ok(TriggerTypeEnum::FirebaseNotificationOpen),
           "firebaseNotificationReceive" => Ok(TriggerTypeEnum::FirebaseNotificationReceive),
           "firebaseOsUpdate" => Ok(TriggerTypeEnum::FirebaseOsUpdate),
           "firebaseSessionStart" => Ok(TriggerTypeEnum::FirebaseSessionStart),
           "firebaseUserEngagement" => Ok(TriggerTypeEnum::FirebaseUserEngagement),
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


// region VariableFormatValueCaseConversionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The option to convert a string-type variable value to either lowercase or uppercase.
pub enum VariableFormatValueCaseConversionTypeEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The option to convert a variable value to lowercase.
    ///
    /// "lowercase"
    #[serde(rename="lowercase")]
    Lowercase,
    

    /// The option to convert a variable value to uppercase.
    ///
    /// "uppercase"
    #[serde(rename="uppercase")]
    Uppercase,
}

impl AsRef<str> for VariableFormatValueCaseConversionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VariableFormatValueCaseConversionTypeEnum::None => "none",
            VariableFormatValueCaseConversionTypeEnum::Lowercase => "lowercase",
            VariableFormatValueCaseConversionTypeEnum::Uppercase => "uppercase",
        }
    }
}

impl std::convert::TryFrom< &str> for VariableFormatValueCaseConversionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "none" => Ok(VariableFormatValueCaseConversionTypeEnum::None),
           "lowercase" => Ok(VariableFormatValueCaseConversionTypeEnum::Lowercase),
           "uppercase" => Ok(VariableFormatValueCaseConversionTypeEnum::Uppercase),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VariableFormatValueCaseConversionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of built-in variable to revert.
pub enum AccountTypeEnum {
    
    /// "builtInVariableTypeUnspecified"
    #[serde(rename="builtInVariableTypeUnspecified")]
    BuiltInVariableTypeUnspecified,
    
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
    
    /// "environmentName"
    #[serde(rename="environmentName")]
    EnvironmentName,
    
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
    
    /// "requestPath"
    #[serde(rename="requestPath")]
    RequestPath,
    
    /// "requestMethod"
    #[serde(rename="requestMethod")]
    RequestMethod,
    
    /// "clientName"
    #[serde(rename="clientName")]
    ClientName,
    
    /// "queryString"
    #[serde(rename="queryString")]
    QueryString,
    
    /// "serverPageLocationUrl"
    #[serde(rename="serverPageLocationUrl")]
    ServerPageLocationUrl,
    
    /// "serverPageLocationPath"
    #[serde(rename="serverPageLocationPath")]
    ServerPageLocationPath,
    
    /// "serverPageLocationHostname"
    #[serde(rename="serverPageLocationHostname")]
    ServerPageLocationHostname,
    
    /// "visitorRegion"
    #[serde(rename="visitorRegion")]
    VisitorRegion,
}

impl AsRef<str> for AccountTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountTypeEnum::BuiltInVariableTypeUnspecified => "builtInVariableTypeUnspecified",
            AccountTypeEnum::PageUrl => "pageUrl",
            AccountTypeEnum::PageHostname => "pageHostname",
            AccountTypeEnum::PagePath => "pagePath",
            AccountTypeEnum::Referrer => "referrer",
            AccountTypeEnum::Event => "event",
            AccountTypeEnum::ClickElement => "clickElement",
            AccountTypeEnum::ClickClasses => "clickClasses",
            AccountTypeEnum::ClickId => "clickId",
            AccountTypeEnum::ClickTarget => "clickTarget",
            AccountTypeEnum::ClickUrl => "clickUrl",
            AccountTypeEnum::ClickText => "clickText",
            AccountTypeEnum::FirstPartyServingUrl => "firstPartyServingUrl",
            AccountTypeEnum::FormElement => "formElement",
            AccountTypeEnum::FormClasses => "formClasses",
            AccountTypeEnum::FormId => "formId",
            AccountTypeEnum::FormTarget => "formTarget",
            AccountTypeEnum::FormUrl => "formUrl",
            AccountTypeEnum::FormText => "formText",
            AccountTypeEnum::ErrorMessage => "errorMessage",
            AccountTypeEnum::ErrorUrl => "errorUrl",
            AccountTypeEnum::ErrorLine => "errorLine",
            AccountTypeEnum::NewHistoryUrl => "newHistoryUrl",
            AccountTypeEnum::OldHistoryUrl => "oldHistoryUrl",
            AccountTypeEnum::NewHistoryFragment => "newHistoryFragment",
            AccountTypeEnum::OldHistoryFragment => "oldHistoryFragment",
            AccountTypeEnum::NewHistoryState => "newHistoryState",
            AccountTypeEnum::OldHistoryState => "oldHistoryState",
            AccountTypeEnum::HistorySource => "historySource",
            AccountTypeEnum::ContainerVersion => "containerVersion",
            AccountTypeEnum::DebugMode => "debugMode",
            AccountTypeEnum::RandomNumber => "randomNumber",
            AccountTypeEnum::ContainerId => "containerId",
            AccountTypeEnum::AppId => "appId",
            AccountTypeEnum::AppName => "appName",
            AccountTypeEnum::AppVersionCode => "appVersionCode",
            AccountTypeEnum::AppVersionName => "appVersionName",
            AccountTypeEnum::Language => "language",
            AccountTypeEnum::OsVersion => "osVersion",
            AccountTypeEnum::Platform => "platform",
            AccountTypeEnum::SdkVersion => "sdkVersion",
            AccountTypeEnum::DeviceName => "deviceName",
            AccountTypeEnum::Resolution => "resolution",
            AccountTypeEnum::AdvertiserId => "advertiserId",
            AccountTypeEnum::AdvertisingTrackingEnabled => "advertisingTrackingEnabled",
            AccountTypeEnum::HtmlId => "htmlId",
            AccountTypeEnum::EnvironmentName => "environmentName",
            AccountTypeEnum::AmpBrowserLanguage => "ampBrowserLanguage",
            AccountTypeEnum::AmpCanonicalPath => "ampCanonicalPath",
            AccountTypeEnum::AmpCanonicalUrl => "ampCanonicalUrl",
            AccountTypeEnum::AmpCanonicalHost => "ampCanonicalHost",
            AccountTypeEnum::AmpReferrer => "ampReferrer",
            AccountTypeEnum::AmpTitle => "ampTitle",
            AccountTypeEnum::AmpClientId => "ampClientId",
            AccountTypeEnum::AmpClientTimezone => "ampClientTimezone",
            AccountTypeEnum::AmpClientTimestamp => "ampClientTimestamp",
            AccountTypeEnum::AmpClientScreenWidth => "ampClientScreenWidth",
            AccountTypeEnum::AmpClientScreenHeight => "ampClientScreenHeight",
            AccountTypeEnum::AmpClientScrollX => "ampClientScrollX",
            AccountTypeEnum::AmpClientScrollY => "ampClientScrollY",
            AccountTypeEnum::AmpClientMaxScrollX => "ampClientMaxScrollX",
            AccountTypeEnum::AmpClientMaxScrollY => "ampClientMaxScrollY",
            AccountTypeEnum::AmpTotalEngagedTime => "ampTotalEngagedTime",
            AccountTypeEnum::AmpPageViewId => "ampPageViewId",
            AccountTypeEnum::AmpPageLoadTime => "ampPageLoadTime",
            AccountTypeEnum::AmpPageDownloadTime => "ampPageDownloadTime",
            AccountTypeEnum::AmpGtmEvent => "ampGtmEvent",
            AccountTypeEnum::EventName => "eventName",
            AccountTypeEnum::FirebaseEventParameterCampaign => "firebaseEventParameterCampaign",
            AccountTypeEnum::FirebaseEventParameterCampaignAclid => "firebaseEventParameterCampaignAclid",
            AccountTypeEnum::FirebaseEventParameterCampaignAnid => "firebaseEventParameterCampaignAnid",
            AccountTypeEnum::FirebaseEventParameterCampaignClickTimestamp => "firebaseEventParameterCampaignClickTimestamp",
            AccountTypeEnum::FirebaseEventParameterCampaignContent => "firebaseEventParameterCampaignContent",
            AccountTypeEnum::FirebaseEventParameterCampaignCp1 => "firebaseEventParameterCampaignCp1",
            AccountTypeEnum::FirebaseEventParameterCampaignGclid => "firebaseEventParameterCampaignGclid",
            AccountTypeEnum::FirebaseEventParameterCampaignSource => "firebaseEventParameterCampaignSource",
            AccountTypeEnum::FirebaseEventParameterCampaignTerm => "firebaseEventParameterCampaignTerm",
            AccountTypeEnum::FirebaseEventParameterCurrency => "firebaseEventParameterCurrency",
            AccountTypeEnum::FirebaseEventParameterDynamicLinkAcceptTime => "firebaseEventParameterDynamicLinkAcceptTime",
            AccountTypeEnum::FirebaseEventParameterDynamicLinkLinkid => "firebaseEventParameterDynamicLinkLinkid",
            AccountTypeEnum::FirebaseEventParameterNotificationMessageDeviceTime => "firebaseEventParameterNotificationMessageDeviceTime",
            AccountTypeEnum::FirebaseEventParameterNotificationMessageId => "firebaseEventParameterNotificationMessageId",
            AccountTypeEnum::FirebaseEventParameterNotificationMessageName => "firebaseEventParameterNotificationMessageName",
            AccountTypeEnum::FirebaseEventParameterNotificationMessageTime => "firebaseEventParameterNotificationMessageTime",
            AccountTypeEnum::FirebaseEventParameterNotificationTopic => "firebaseEventParameterNotificationTopic",
            AccountTypeEnum::FirebaseEventParameterPreviousAppVersion => "firebaseEventParameterPreviousAppVersion",
            AccountTypeEnum::FirebaseEventParameterPreviousOsVersion => "firebaseEventParameterPreviousOsVersion",
            AccountTypeEnum::FirebaseEventParameterPrice => "firebaseEventParameterPrice",
            AccountTypeEnum::FirebaseEventParameterProductId => "firebaseEventParameterProductId",
            AccountTypeEnum::FirebaseEventParameterQuantity => "firebaseEventParameterQuantity",
            AccountTypeEnum::FirebaseEventParameterValue => "firebaseEventParameterValue",
            AccountTypeEnum::VideoProvider => "videoProvider",
            AccountTypeEnum::VideoUrl => "videoUrl",
            AccountTypeEnum::VideoTitle => "videoTitle",
            AccountTypeEnum::VideoDuration => "videoDuration",
            AccountTypeEnum::VideoPercent => "videoPercent",
            AccountTypeEnum::VideoVisible => "videoVisible",
            AccountTypeEnum::VideoStatus => "videoStatus",
            AccountTypeEnum::VideoCurrentTime => "videoCurrentTime",
            AccountTypeEnum::ScrollDepthThreshold => "scrollDepthThreshold",
            AccountTypeEnum::ScrollDepthUnits => "scrollDepthUnits",
            AccountTypeEnum::ScrollDepthDirection => "scrollDepthDirection",
            AccountTypeEnum::ElementVisibilityRatio => "elementVisibilityRatio",
            AccountTypeEnum::ElementVisibilityTime => "elementVisibilityTime",
            AccountTypeEnum::ElementVisibilityFirstTime => "elementVisibilityFirstTime",
            AccountTypeEnum::ElementVisibilityRecentTime => "elementVisibilityRecentTime",
            AccountTypeEnum::RequestPath => "requestPath",
            AccountTypeEnum::RequestMethod => "requestMethod",
            AccountTypeEnum::ClientName => "clientName",
            AccountTypeEnum::QueryString => "queryString",
            AccountTypeEnum::ServerPageLocationUrl => "serverPageLocationUrl",
            AccountTypeEnum::ServerPageLocationPath => "serverPageLocationPath",
            AccountTypeEnum::ServerPageLocationHostname => "serverPageLocationHostname",
            AccountTypeEnum::VisitorRegion => "visitorRegion",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "builtInVariableTypeUnspecified" => Ok(AccountTypeEnum::BuiltInVariableTypeUnspecified),
           "pageUrl" => Ok(AccountTypeEnum::PageUrl),
           "pageHostname" => Ok(AccountTypeEnum::PageHostname),
           "pagePath" => Ok(AccountTypeEnum::PagePath),
           "referrer" => Ok(AccountTypeEnum::Referrer),
           "event" => Ok(AccountTypeEnum::Event),
           "clickElement" => Ok(AccountTypeEnum::ClickElement),
           "clickClasses" => Ok(AccountTypeEnum::ClickClasses),
           "clickId" => Ok(AccountTypeEnum::ClickId),
           "clickTarget" => Ok(AccountTypeEnum::ClickTarget),
           "clickUrl" => Ok(AccountTypeEnum::ClickUrl),
           "clickText" => Ok(AccountTypeEnum::ClickText),
           "firstPartyServingUrl" => Ok(AccountTypeEnum::FirstPartyServingUrl),
           "formElement" => Ok(AccountTypeEnum::FormElement),
           "formClasses" => Ok(AccountTypeEnum::FormClasses),
           "formId" => Ok(AccountTypeEnum::FormId),
           "formTarget" => Ok(AccountTypeEnum::FormTarget),
           "formUrl" => Ok(AccountTypeEnum::FormUrl),
           "formText" => Ok(AccountTypeEnum::FormText),
           "errorMessage" => Ok(AccountTypeEnum::ErrorMessage),
           "errorUrl" => Ok(AccountTypeEnum::ErrorUrl),
           "errorLine" => Ok(AccountTypeEnum::ErrorLine),
           "newHistoryUrl" => Ok(AccountTypeEnum::NewHistoryUrl),
           "oldHistoryUrl" => Ok(AccountTypeEnum::OldHistoryUrl),
           "newHistoryFragment" => Ok(AccountTypeEnum::NewHistoryFragment),
           "oldHistoryFragment" => Ok(AccountTypeEnum::OldHistoryFragment),
           "newHistoryState" => Ok(AccountTypeEnum::NewHistoryState),
           "oldHistoryState" => Ok(AccountTypeEnum::OldHistoryState),
           "historySource" => Ok(AccountTypeEnum::HistorySource),
           "containerVersion" => Ok(AccountTypeEnum::ContainerVersion),
           "debugMode" => Ok(AccountTypeEnum::DebugMode),
           "randomNumber" => Ok(AccountTypeEnum::RandomNumber),
           "containerId" => Ok(AccountTypeEnum::ContainerId),
           "appId" => Ok(AccountTypeEnum::AppId),
           "appName" => Ok(AccountTypeEnum::AppName),
           "appVersionCode" => Ok(AccountTypeEnum::AppVersionCode),
           "appVersionName" => Ok(AccountTypeEnum::AppVersionName),
           "language" => Ok(AccountTypeEnum::Language),
           "osVersion" => Ok(AccountTypeEnum::OsVersion),
           "platform" => Ok(AccountTypeEnum::Platform),
           "sdkVersion" => Ok(AccountTypeEnum::SdkVersion),
           "deviceName" => Ok(AccountTypeEnum::DeviceName),
           "resolution" => Ok(AccountTypeEnum::Resolution),
           "advertiserId" => Ok(AccountTypeEnum::AdvertiserId),
           "advertisingTrackingEnabled" => Ok(AccountTypeEnum::AdvertisingTrackingEnabled),
           "htmlId" => Ok(AccountTypeEnum::HtmlId),
           "environmentName" => Ok(AccountTypeEnum::EnvironmentName),
           "ampBrowserLanguage" => Ok(AccountTypeEnum::AmpBrowserLanguage),
           "ampCanonicalPath" => Ok(AccountTypeEnum::AmpCanonicalPath),
           "ampCanonicalUrl" => Ok(AccountTypeEnum::AmpCanonicalUrl),
           "ampCanonicalHost" => Ok(AccountTypeEnum::AmpCanonicalHost),
           "ampReferrer" => Ok(AccountTypeEnum::AmpReferrer),
           "ampTitle" => Ok(AccountTypeEnum::AmpTitle),
           "ampClientId" => Ok(AccountTypeEnum::AmpClientId),
           "ampClientTimezone" => Ok(AccountTypeEnum::AmpClientTimezone),
           "ampClientTimestamp" => Ok(AccountTypeEnum::AmpClientTimestamp),
           "ampClientScreenWidth" => Ok(AccountTypeEnum::AmpClientScreenWidth),
           "ampClientScreenHeight" => Ok(AccountTypeEnum::AmpClientScreenHeight),
           "ampClientScrollX" => Ok(AccountTypeEnum::AmpClientScrollX),
           "ampClientScrollY" => Ok(AccountTypeEnum::AmpClientScrollY),
           "ampClientMaxScrollX" => Ok(AccountTypeEnum::AmpClientMaxScrollX),
           "ampClientMaxScrollY" => Ok(AccountTypeEnum::AmpClientMaxScrollY),
           "ampTotalEngagedTime" => Ok(AccountTypeEnum::AmpTotalEngagedTime),
           "ampPageViewId" => Ok(AccountTypeEnum::AmpPageViewId),
           "ampPageLoadTime" => Ok(AccountTypeEnum::AmpPageLoadTime),
           "ampPageDownloadTime" => Ok(AccountTypeEnum::AmpPageDownloadTime),
           "ampGtmEvent" => Ok(AccountTypeEnum::AmpGtmEvent),
           "eventName" => Ok(AccountTypeEnum::EventName),
           "firebaseEventParameterCampaign" => Ok(AccountTypeEnum::FirebaseEventParameterCampaign),
           "firebaseEventParameterCampaignAclid" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignAclid),
           "firebaseEventParameterCampaignAnid" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignAnid),
           "firebaseEventParameterCampaignClickTimestamp" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignClickTimestamp),
           "firebaseEventParameterCampaignContent" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignContent),
           "firebaseEventParameterCampaignCp1" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignCp1),
           "firebaseEventParameterCampaignGclid" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignGclid),
           "firebaseEventParameterCampaignSource" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignSource),
           "firebaseEventParameterCampaignTerm" => Ok(AccountTypeEnum::FirebaseEventParameterCampaignTerm),
           "firebaseEventParameterCurrency" => Ok(AccountTypeEnum::FirebaseEventParameterCurrency),
           "firebaseEventParameterDynamicLinkAcceptTime" => Ok(AccountTypeEnum::FirebaseEventParameterDynamicLinkAcceptTime),
           "firebaseEventParameterDynamicLinkLinkid" => Ok(AccountTypeEnum::FirebaseEventParameterDynamicLinkLinkid),
           "firebaseEventParameterNotificationMessageDeviceTime" => Ok(AccountTypeEnum::FirebaseEventParameterNotificationMessageDeviceTime),
           "firebaseEventParameterNotificationMessageId" => Ok(AccountTypeEnum::FirebaseEventParameterNotificationMessageId),
           "firebaseEventParameterNotificationMessageName" => Ok(AccountTypeEnum::FirebaseEventParameterNotificationMessageName),
           "firebaseEventParameterNotificationMessageTime" => Ok(AccountTypeEnum::FirebaseEventParameterNotificationMessageTime),
           "firebaseEventParameterNotificationTopic" => Ok(AccountTypeEnum::FirebaseEventParameterNotificationTopic),
           "firebaseEventParameterPreviousAppVersion" => Ok(AccountTypeEnum::FirebaseEventParameterPreviousAppVersion),
           "firebaseEventParameterPreviousOsVersion" => Ok(AccountTypeEnum::FirebaseEventParameterPreviousOsVersion),
           "firebaseEventParameterPrice" => Ok(AccountTypeEnum::FirebaseEventParameterPrice),
           "firebaseEventParameterProductId" => Ok(AccountTypeEnum::FirebaseEventParameterProductId),
           "firebaseEventParameterQuantity" => Ok(AccountTypeEnum::FirebaseEventParameterQuantity),
           "firebaseEventParameterValue" => Ok(AccountTypeEnum::FirebaseEventParameterValue),
           "videoProvider" => Ok(AccountTypeEnum::VideoProvider),
           "videoUrl" => Ok(AccountTypeEnum::VideoUrl),
           "videoTitle" => Ok(AccountTypeEnum::VideoTitle),
           "videoDuration" => Ok(AccountTypeEnum::VideoDuration),
           "videoPercent" => Ok(AccountTypeEnum::VideoPercent),
           "videoVisible" => Ok(AccountTypeEnum::VideoVisible),
           "videoStatus" => Ok(AccountTypeEnum::VideoStatus),
           "videoCurrentTime" => Ok(AccountTypeEnum::VideoCurrentTime),
           "scrollDepthThreshold" => Ok(AccountTypeEnum::ScrollDepthThreshold),
           "scrollDepthUnits" => Ok(AccountTypeEnum::ScrollDepthUnits),
           "scrollDepthDirection" => Ok(AccountTypeEnum::ScrollDepthDirection),
           "elementVisibilityRatio" => Ok(AccountTypeEnum::ElementVisibilityRatio),
           "elementVisibilityTime" => Ok(AccountTypeEnum::ElementVisibilityTime),
           "elementVisibilityFirstTime" => Ok(AccountTypeEnum::ElementVisibilityFirstTime),
           "elementVisibilityRecentTime" => Ok(AccountTypeEnum::ElementVisibilityRecentTime),
           "requestPath" => Ok(AccountTypeEnum::RequestPath),
           "requestMethod" => Ok(AccountTypeEnum::RequestMethod),
           "clientName" => Ok(AccountTypeEnum::ClientName),
           "queryString" => Ok(AccountTypeEnum::QueryString),
           "serverPageLocationUrl" => Ok(AccountTypeEnum::ServerPageLocationUrl),
           "serverPageLocationPath" => Ok(AccountTypeEnum::ServerPageLocationPath),
           "serverPageLocationHostname" => Ok(AccountTypeEnum::ServerPageLocationHostname),
           "visitorRegion" => Ok(AccountTypeEnum::VisitorRegion),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AccountSettingSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specify the source of config setting after combine
pub enum AccountSettingSourceEnum {
    
    /// "settingSourceUnspecified"
    #[serde(rename="settingSourceUnspecified")]
    SettingSourceUnspecified,
    

    /// Keep the current container config setting after combine
    ///
    /// "current"
    #[serde(rename="current")]
    Current,
    

    /// Use config setting from the other tag after combine
    ///
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for AccountSettingSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AccountSettingSourceEnum::SettingSourceUnspecified => "settingSourceUnspecified",
            AccountSettingSourceEnum::Current => "current",
            AccountSettingSourceEnum::Other => "other",
        }
    }
}

impl std::convert::TryFrom< &str> for AccountSettingSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "settingSourceUnspecified" => Ok(AccountSettingSourceEnum::SettingSourceUnspecified),
           "current" => Ok(AccountSettingSourceEnum::Current),
           "other" => Ok(AccountSettingSourceEnum::Other),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AccountSettingSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


