use super::*;



// region GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Requested video annotation features.
pub enum GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum {
    

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
    

    /// Explicit content detection.
    ///
    /// "EXPLICIT_CONTENT_DETECTION"
    #[serde(rename="EXPLICIT_CONTENT_DETECTION")]
    EXPLICITCONTENTDETECTION,
    

    /// Human face detection.
    ///
    /// "FACE_DETECTION"
    #[serde(rename="FACE_DETECTION")]
    FACEDETECTION,
    

    /// Speech transcription.
    ///
    /// "SPEECH_TRANSCRIPTION"
    #[serde(rename="SPEECH_TRANSCRIPTION")]
    SPEECHTRANSCRIPTION,
    

    /// OCR text detection and tracking.
    ///
    /// "TEXT_DETECTION"
    #[serde(rename="TEXT_DETECTION")]
    TEXTDETECTION,
    

    /// Object detection and tracking.
    ///
    /// "OBJECT_TRACKING"
    #[serde(rename="OBJECT_TRACKING")]
    OBJECTTRACKING,
    

    /// Logo detection, tracking, and recognition.
    ///
    /// "LOGO_RECOGNITION"
    #[serde(rename="LOGO_RECOGNITION")]
    LOGORECOGNITION,
    

    /// Person detection.
    ///
    /// "PERSON_DETECTION"
    #[serde(rename="PERSON_DETECTION")]
    PERSONDETECTION,
}

impl AsRef<str> for GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::LABELDETECTION => "LABEL_DETECTION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::SHOTCHANGEDETECTION => "SHOT_CHANGE_DETECTION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::EXPLICITCONTENTDETECTION => "EXPLICIT_CONTENT_DETECTION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::FACEDETECTION => "FACE_DETECTION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::SPEECHTRANSCRIPTION => "SPEECH_TRANSCRIPTION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::TEXTDETECTION => "TEXT_DETECTION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::OBJECTTRACKING => "OBJECT_TRACKING",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::LOGORECOGNITION => "LOGO_RECOGNITION",
            GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::PERSONDETECTION => "PERSON_DETECTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::FEATUREUNSPECIFIED),
           "LABEL_DETECTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::LABELDETECTION),
           "SHOT_CHANGE_DETECTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::SHOTCHANGEDETECTION),
           "EXPLICIT_CONTENT_DETECTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::EXPLICITCONTENTDETECTION),
           "FACE_DETECTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::FACEDETECTION),
           "SPEECH_TRANSCRIPTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::SPEECHTRANSCRIPTION),
           "TEXT_DETECTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::TEXTDETECTION),
           "OBJECT_TRACKING" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::OBJECTTRACKING),
           "LOGO_RECOGNITION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::LOGORECOGNITION),
           "PERSON_DETECTION" => Ok(GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum::PERSONDETECTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`.
pub enum GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum {
    

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

impl AsRef<str> for GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::LABELDETECTIONMODEUNSPECIFIED => "LABEL_DETECTION_MODE_UNSPECIFIED",
            GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::SHOTMODE => "SHOT_MODE",
            GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::FRAMEMODE => "FRAME_MODE",
            GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::SHOTANDFRAMEMODE => "SHOT_AND_FRAME_MODE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LABEL_DETECTION_MODE_UNSPECIFIED" => Ok(GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::LABELDETECTIONMODEUNSPECIFIED),
           "SHOT_MODE" => Ok(GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::SHOTMODE),
           "FRAME_MODE" => Ok(GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::FRAMEMODE),
           "SHOT_AND_FRAME_MODE" => Ok(GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum::SHOTANDFRAMEMODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


