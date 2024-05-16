use super::*;



// region GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of impression event.
pub enum GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum {
    

    /// Unspecified type. Do not use.
    ///
    /// "IMPRESSION_TYPE_UNSPECIFIED"
    #[serde(rename="IMPRESSION_TYPE_UNSPECIFIED")]
    IMPRESSIONTYPEUNSPECIFIED,
    

    /// The playable location was presented to a player.
    ///
    /// "PRESENTED"
    #[serde(rename="PRESENTED")]
    PRESENTED,
    

    /// A player interacted with the playable location.
    ///
    /// "INTERACTED"
    #[serde(rename="INTERACTED")]
    INTERACTED,
}

impl AsRef<str> for GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum::IMPRESSIONTYPEUNSPECIFIED => "IMPRESSION_TYPE_UNSPECIFIED",
            GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum::PRESENTED => "PRESENTED",
            GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum::INTERACTED => "INTERACTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IMPRESSION_TYPE_UNSPECIFIED" => Ok(GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum::IMPRESSIONTYPEUNSPECIFIED),
           "PRESENTED" => Ok(GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum::PRESENTED),
           "INTERACTED" => Ok(GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum::INTERACTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleMapsPlayablelocationsV3ImpressionImpressionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. One or more reasons why this playable location is considered bad.
pub enum GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum {
    

    /// Unspecified reason. Do not use.
    ///
    /// "BAD_LOCATION_REASON_UNSPECIFIED"
    #[serde(rename="BAD_LOCATION_REASON_UNSPECIFIED")]
    BADLOCATIONREASONUNSPECIFIED,
    

    /// The reason isn't one of the reasons in this enumeration.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    

    /// The playable location isn't accessible to pedestrians. For example, if
it's in the middle of a highway.
    ///
    /// "NOT_PEDESTRIAN_ACCESSIBLE"
    #[serde(rename="NOT_PEDESTRIAN_ACCESSIBLE")]
    NOTPEDESTRIANACCESSIBLE,
    

    /// The playable location isn't open to the public. For example, a private
office building.
    ///
    /// "NOT_OPEN_TO_PUBLIC"
    #[serde(rename="NOT_OPEN_TO_PUBLIC")]
    NOTOPENTOPUBLIC,
    

    /// The playable location is permanently closed. For example, when a business
has been shut down.
    ///
    /// "PERMANENTLY_CLOSED"
    #[serde(rename="PERMANENTLY_CLOSED")]
    PERMANENTLYCLOSED,
    

    /// The playable location is temporarily inaccessible. For example, when a
business has closed for renovations.
    ///
    /// "TEMPORARILY_INACCESSIBLE"
    #[serde(rename="TEMPORARILY_INACCESSIBLE")]
    TEMPORARILYINACCESSIBLE,
}

impl AsRef<str> for GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::BADLOCATIONREASONUNSPECIFIED => "BAD_LOCATION_REASON_UNSPECIFIED",
            GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::OTHER => "OTHER",
            GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::NOTPEDESTRIANACCESSIBLE => "NOT_PEDESTRIAN_ACCESSIBLE",
            GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::NOTOPENTOPUBLIC => "NOT_OPEN_TO_PUBLIC",
            GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::PERMANENTLYCLOSED => "PERMANENTLY_CLOSED",
            GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::TEMPORARILYINACCESSIBLE => "TEMPORARILY_INACCESSIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BAD_LOCATION_REASON_UNSPECIFIED" => Ok(GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::BADLOCATIONREASONUNSPECIFIED),
           "OTHER" => Ok(GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::OTHER),
           "NOT_PEDESTRIAN_ACCESSIBLE" => Ok(GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::NOTPEDESTRIANACCESSIBLE),
           "NOT_OPEN_TO_PUBLIC" => Ok(GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::NOTOPENTOPUBLIC),
           "PERMANENTLY_CLOSED" => Ok(GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::PERMANENTLYCLOSED),
           "TEMPORARILY_INACCESSIBLE" => Ok(GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum::TEMPORARILYINACCESSIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleMapsPlayablelocationsV3PlayerReportReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies whether the minimum spacing constraint applies to the
center-point or to the snapped point of playable locations. The default
value is `CENTER_POINT`.

If a snapped point is not available for a playable location, its
center-point is used instead.

Set this to the point type used in your game.
pub enum GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum {
    

    /// Unspecified point type. Do not use this value.
    ///
    /// "POINT_TYPE_UNSPECIFIED"
    #[serde(rename="POINT_TYPE_UNSPECIFIED")]
    POINTTYPEUNSPECIFIED,
    

    /// The geographic coordinates correspond to the center of the location.
    ///
    /// "CENTER_POINT"
    #[serde(rename="CENTER_POINT")]
    CENTERPOINT,
    

    /// The geographic coordinates correspond to the location snapped to the
sidewalk of the nearest road (when a nearby road exists).
    ///
    /// "SNAPPED_POINT"
    #[serde(rename="SNAPPED_POINT")]
    SNAPPEDPOINT,
}

impl AsRef<str> for GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum::POINTTYPEUNSPECIFIED => "POINT_TYPE_UNSPECIFIED",
            GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum::CENTERPOINT => "CENTER_POINT",
            GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum::SNAPPEDPOINT => "SNAPPED_POINT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POINT_TYPE_UNSPECIFIED" => Ok(GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum::POINTTYPEUNSPECIFIED),
           "CENTER_POINT" => Ok(GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum::CENTERPOINT),
           "SNAPPED_POINT" => Ok(GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum::SNAPPEDPOINT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleMapsPlayablelocationsV3SampleSpacingOptionPointTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleMapsUnityClientInfoPlatformEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Platform where the application is running.
pub enum GoogleMapsUnityClientInfoPlatformEnum {
    

    /// Unspecified or unknown OS.
    ///
    /// "PLATFORM_UNSPECIFIED"
    #[serde(rename="PLATFORM_UNSPECIFIED")]
    PLATFORMUNSPECIFIED,
    

    /// Development environment.
    ///
    /// "EDITOR"
    #[serde(rename="EDITOR")]
    EDITOR,
    

    /// macOS.
    ///
    /// "MAC_OS"
    #[serde(rename="MAC_OS")]
    MACOS,
    

    /// Windows.
    ///
    /// "WINDOWS"
    #[serde(rename="WINDOWS")]
    WINDOWS,
    

    /// Linux
    ///
    /// "LINUX"
    #[serde(rename="LINUX")]
    LINUX,
    

    /// Android
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// iOS
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// WebGL.
    ///
    /// "WEB_GL"
    #[serde(rename="WEB_GL")]
    WEBGL,
}

impl AsRef<str> for GoogleMapsUnityClientInfoPlatformEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleMapsUnityClientInfoPlatformEnum::PLATFORMUNSPECIFIED => "PLATFORM_UNSPECIFIED",
            GoogleMapsUnityClientInfoPlatformEnum::EDITOR => "EDITOR",
            GoogleMapsUnityClientInfoPlatformEnum::MACOS => "MAC_OS",
            GoogleMapsUnityClientInfoPlatformEnum::WINDOWS => "WINDOWS",
            GoogleMapsUnityClientInfoPlatformEnum::LINUX => "LINUX",
            GoogleMapsUnityClientInfoPlatformEnum::ANDROID => "ANDROID",
            GoogleMapsUnityClientInfoPlatformEnum::IOS => "IOS",
            GoogleMapsUnityClientInfoPlatformEnum::WEBGL => "WEB_GL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleMapsUnityClientInfoPlatformEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PLATFORM_UNSPECIFIED" => Ok(GoogleMapsUnityClientInfoPlatformEnum::PLATFORMUNSPECIFIED),
           "EDITOR" => Ok(GoogleMapsUnityClientInfoPlatformEnum::EDITOR),
           "MAC_OS" => Ok(GoogleMapsUnityClientInfoPlatformEnum::MACOS),
           "WINDOWS" => Ok(GoogleMapsUnityClientInfoPlatformEnum::WINDOWS),
           "LINUX" => Ok(GoogleMapsUnityClientInfoPlatformEnum::LINUX),
           "ANDROID" => Ok(GoogleMapsUnityClientInfoPlatformEnum::ANDROID),
           "IOS" => Ok(GoogleMapsUnityClientInfoPlatformEnum::IOS),
           "WEB_GL" => Ok(GoogleMapsUnityClientInfoPlatformEnum::WEBGL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleMapsUnityClientInfoPlatformEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


