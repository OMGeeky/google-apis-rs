use super::*;



// region GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requested video annotation features.
pub enum GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum {
    

    /// Unspecified.
    ///
    /// "FEATURE_UNSPECIFIED"
    #[serde(rename="FEATURE_UNSPECIFIED")]
    FEATUREUNSPECIFIED,
    

    /// Label detection. Detect objects, such as dog or flower.
    ///
    /// "LABEL_DETECTION"
    #[serde(rename="LABEL_DETECTION")]
    LABELDETECTION,
    

    /// Shot change detection.
    ///
    /// "SHOT_CHANGE_DETECTION"
    #[serde(rename="SHOT_CHANGE_DETECTION")]
    SHOTCHANGEDETECTION,
    

    /// Safe search detection.
    ///
    /// "SAFE_SEARCH_DETECTION"
    #[serde(rename="SAFE_SEARCH_DETECTION")]
    SAFESEARCHDETECTION,
}

impl AsRef<str> for GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::LABELDETECTION => "LABEL_DETECTION",
            GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::SHOTCHANGEDETECTION => "SHOT_CHANGE_DETECTION",
            GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::SAFESEARCHDETECTION => "SAFE_SEARCH_DETECTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::FEATUREUNSPECIFIED),
           "LABEL_DETECTION" => Ok(GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::LABELDETECTION),
           "SHOT_CHANGE_DETECTION" => Ok(GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::SHOTCHANGEDETECTION),
           "SAFE_SEARCH_DETECTION" => Ok(GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum::SAFESEARCHDETECTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudVideointelligenceV1beta1AnnotateVideoRequestFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If label detection has been requested, what labels should be detected
in addition to video-level labels or segment-level labels. If unspecified,
defaults to `SHOT_MODE`.
pub enum GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum {
    

    /// Unspecified.
    ///
    /// "LABEL_DETECTION_MODE_UNSPECIFIED"
    #[serde(rename="LABEL_DETECTION_MODE_UNSPECIFIED")]
    LABELDETECTIONMODEUNSPECIFIED,
    

    /// Detect shot-level labels.
    ///
    /// "SHOT_MODE"
    #[serde(rename="SHOT_MODE")]
    SHOTMODE,
    

    /// Detect frame-level labels.
    ///
    /// "FRAME_MODE"
    #[serde(rename="FRAME_MODE")]
    FRAMEMODE,
    

    /// Detect both shot-level and frame-level labels.
    ///
    /// "SHOT_AND_FRAME_MODE"
    #[serde(rename="SHOT_AND_FRAME_MODE")]
    SHOTANDFRAMEMODE,
}

impl AsRef<str> for GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::LABELDETECTIONMODEUNSPECIFIED => "LABEL_DETECTION_MODE_UNSPECIFIED",
            GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::SHOTMODE => "SHOT_MODE",
            GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::FRAMEMODE => "FRAME_MODE",
            GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::SHOTANDFRAMEMODE => "SHOT_AND_FRAME_MODE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LABEL_DETECTION_MODE_UNSPECIFIED" => Ok(GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::LABELDETECTIONMODEUNSPECIFIED),
           "SHOT_MODE" => Ok(GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::SHOTMODE),
           "FRAME_MODE" => Ok(GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::FRAMEMODE),
           "SHOT_AND_FRAME_MODE" => Ok(GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum::SHOTANDFRAMEMODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudVideointelligenceV1beta1VideoContextLabelDetectionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


