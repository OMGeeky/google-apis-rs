use super::*;



// region GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Source of the annotation.
pub enum GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum {
    
    /// "ANNOTATION_SOURCE_UNSPECIFIED"
    #[serde(rename="ANNOTATION_SOURCE_UNSPECIFIED")]
    ANNOTATIONSOURCEUNSPECIFIED,
    

    /// Answer is provided by a human contributor.
    ///
    /// "OPERATOR"
    #[serde(rename="OPERATOR")]
    OPERATOR,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum::ANNOTATIONSOURCEUNSPECIFIED => "ANNOTATION_SOURCE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum::OPERATOR => "OPERATOR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_SOURCE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum::ANNOTATIONSOURCEUNSPECIFIED),
           "OPERATOR" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum::OPERATOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of the annotation. It is specified when starting labeling task.
pub enum GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum {
    
    /// "ANNOTATION_TYPE_UNSPECIFIED"
    #[serde(rename="ANNOTATION_TYPE_UNSPECIFIED")]
    ANNOTATIONTYPEUNSPECIFIED,
    

    /// Classification annotations in an image. Allowed for continuous evaluation.
    ///
    /// "IMAGE_CLASSIFICATION_ANNOTATION"
    #[serde(rename="IMAGE_CLASSIFICATION_ANNOTATION")]
    IMAGECLASSIFICATIONANNOTATION,
    

    /// Bounding box annotations in an image. A form of image object detection. Allowed for continuous evaluation.
    ///
    /// "IMAGE_BOUNDING_BOX_ANNOTATION"
    #[serde(rename="IMAGE_BOUNDING_BOX_ANNOTATION")]
    IMAGEBOUNDINGBOXANNOTATION,
    

    /// Oriented bounding box. The box does not have to be parallel to horizontal line.
    ///
    /// "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION"
    #[serde(rename="IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION")]
    IMAGEORIENTEDBOUNDINGBOXANNOTATION,
    

    /// Bounding poly annotations in an image.
    ///
    /// "IMAGE_BOUNDING_POLY_ANNOTATION"
    #[serde(rename="IMAGE_BOUNDING_POLY_ANNOTATION")]
    IMAGEBOUNDINGPOLYANNOTATION,
    

    /// Polyline annotations in an image.
    ///
    /// "IMAGE_POLYLINE_ANNOTATION"
    #[serde(rename="IMAGE_POLYLINE_ANNOTATION")]
    IMAGEPOLYLINEANNOTATION,
    

    /// Segmentation annotations in an image.
    ///
    /// "IMAGE_SEGMENTATION_ANNOTATION"
    #[serde(rename="IMAGE_SEGMENTATION_ANNOTATION")]
    IMAGESEGMENTATIONANNOTATION,
    

    /// Classification annotations in video shots.
    ///
    /// "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION"
    #[serde(rename="VIDEO_SHOTS_CLASSIFICATION_ANNOTATION")]
    VIDEOSHOTSCLASSIFICATIONANNOTATION,
    

    /// Video object tracking annotation.
    ///
    /// "VIDEO_OBJECT_TRACKING_ANNOTATION"
    #[serde(rename="VIDEO_OBJECT_TRACKING_ANNOTATION")]
    VIDEOOBJECTTRACKINGANNOTATION,
    

    /// Video object detection annotation.
    ///
    /// "VIDEO_OBJECT_DETECTION_ANNOTATION"
    #[serde(rename="VIDEO_OBJECT_DETECTION_ANNOTATION")]
    VIDEOOBJECTDETECTIONANNOTATION,
    

    /// Video event annotation.
    ///
    /// "VIDEO_EVENT_ANNOTATION"
    #[serde(rename="VIDEO_EVENT_ANNOTATION")]
    VIDEOEVENTANNOTATION,
    

    /// Classification for text. Allowed for continuous evaluation.
    ///
    /// "TEXT_CLASSIFICATION_ANNOTATION"
    #[serde(rename="TEXT_CLASSIFICATION_ANNOTATION")]
    TEXTCLASSIFICATIONANNOTATION,
    

    /// Entity extraction for text.
    ///
    /// "TEXT_ENTITY_EXTRACTION_ANNOTATION"
    #[serde(rename="TEXT_ENTITY_EXTRACTION_ANNOTATION")]
    TEXTENTITYEXTRACTIONANNOTATION,
    

    /// General classification. Allowed for continuous evaluation.
    ///
    /// "GENERAL_CLASSIFICATION_ANNOTATION"
    #[serde(rename="GENERAL_CLASSIFICATION_ANNOTATION")]
    GENERALCLASSIFICATIONANNOTATION,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::ANNOTATIONTYPEUNSPECIFIED => "ANNOTATION_TYPE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGECLASSIFICATIONANNOTATION => "IMAGE_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEBOUNDINGBOXANNOTATION => "IMAGE_BOUNDING_BOX_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEORIENTEDBOUNDINGBOXANNOTATION => "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEBOUNDINGPOLYANNOTATION => "IMAGE_BOUNDING_POLY_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEPOLYLINEANNOTATION => "IMAGE_POLYLINE_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGESEGMENTATIONANNOTATION => "IMAGE_SEGMENTATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOSHOTSCLASSIFICATIONANNOTATION => "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOOBJECTTRACKINGANNOTATION => "VIDEO_OBJECT_TRACKING_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOOBJECTDETECTIONANNOTATION => "VIDEO_OBJECT_DETECTION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOEVENTANNOTATION => "VIDEO_EVENT_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::TEXTCLASSIFICATIONANNOTATION => "TEXT_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::TEXTENTITYEXTRACTIONANNOTATION => "TEXT_ENTITY_EXTRACTION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::GENERALCLASSIFICATIONANNOTATION => "GENERAL_CLASSIFICATION_ANNOTATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::ANNOTATIONTYPEUNSPECIFIED),
           "IMAGE_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGECLASSIFICATIONANNOTATION),
           "IMAGE_BOUNDING_BOX_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEBOUNDINGBOXANNOTATION),
           "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEORIENTEDBOUNDINGBOXANNOTATION),
           "IMAGE_BOUNDING_POLY_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEBOUNDINGPOLYANNOTATION),
           "IMAGE_POLYLINE_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGEPOLYLINEANNOTATION),
           "IMAGE_SEGMENTATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::IMAGESEGMENTATIONANNOTATION),
           "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOSHOTSCLASSIFICATIONANNOTATION),
           "VIDEO_OBJECT_TRACKING_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOOBJECTTRACKINGANNOTATION),
           "VIDEO_OBJECT_DETECTION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOOBJECTDETECTIONANNOTATION),
           "VIDEO_EVENT_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::VIDEOEVENTANNOTATION),
           "TEXT_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::TEXTCLASSIFICATIONANNOTATION),
           "TEXT_ENTITY_EXTRACTION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::TEXTENTITYEXTRACTIONANNOTATION),
           "GENERAL_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum::GENERALCLASSIFICATIONANNOTATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1AnnotatedDatasetAnnotationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Sentiment for this annotation.
pub enum GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum {
    
    /// "ANNOTATION_SENTIMENT_UNSPECIFIED"
    #[serde(rename="ANNOTATION_SENTIMENT_UNSPECIFIED")]
    ANNOTATIONSENTIMENTUNSPECIFIED,
    

    /// This annotation describes negatively about the data.
    ///
    /// "NEGATIVE"
    #[serde(rename="NEGATIVE")]
    NEGATIVE,
    

    /// This label describes positively about the data.
    ///
    /// "POSITIVE"
    #[serde(rename="POSITIVE")]
    POSITIVE,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum::ANNOTATIONSENTIMENTUNSPECIFIED => "ANNOTATION_SENTIMENT_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum::NEGATIVE => "NEGATIVE",
            GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum::POSITIVE => "POSITIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_SENTIMENT_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum::ANNOTATIONSENTIMENTUNSPECIFIED),
           "NEGATIVE" => Ok(GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum::NEGATIVE),
           "POSITIVE" => Ok(GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum::POSITIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1AnnotationAnnotationSentimentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The source of the annotation.
pub enum GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum {
    
    /// "ANNOTATION_SOURCE_UNSPECIFIED"
    #[serde(rename="ANNOTATION_SOURCE_UNSPECIFIED")]
    ANNOTATIONSOURCEUNSPECIFIED,
    

    /// Answer is provided by a human contributor.
    ///
    /// "OPERATOR"
    #[serde(rename="OPERATOR")]
    OPERATOR,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum::ANNOTATIONSOURCEUNSPECIFIED => "ANNOTATION_SOURCE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum::OPERATOR => "OPERATOR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_SOURCE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum::ANNOTATIONSOURCEUNSPECIFIED),
           "OPERATOR" => Ok(GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum::OPERATOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1AnnotationAnnotationSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Type of task that the model version being evaluated performs, as defined in the evaluationJobConfig.inputConfig.annotationType field of the evaluation job that created this evaluation.
pub enum GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum {
    
    /// "ANNOTATION_TYPE_UNSPECIFIED"
    #[serde(rename="ANNOTATION_TYPE_UNSPECIFIED")]
    ANNOTATIONTYPEUNSPECIFIED,
    

    /// Classification annotations in an image. Allowed for continuous evaluation.
    ///
    /// "IMAGE_CLASSIFICATION_ANNOTATION"
    #[serde(rename="IMAGE_CLASSIFICATION_ANNOTATION")]
    IMAGECLASSIFICATIONANNOTATION,
    

    /// Bounding box annotations in an image. A form of image object detection. Allowed for continuous evaluation.
    ///
    /// "IMAGE_BOUNDING_BOX_ANNOTATION"
    #[serde(rename="IMAGE_BOUNDING_BOX_ANNOTATION")]
    IMAGEBOUNDINGBOXANNOTATION,
    

    /// Oriented bounding box. The box does not have to be parallel to horizontal line.
    ///
    /// "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION"
    #[serde(rename="IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION")]
    IMAGEORIENTEDBOUNDINGBOXANNOTATION,
    

    /// Bounding poly annotations in an image.
    ///
    /// "IMAGE_BOUNDING_POLY_ANNOTATION"
    #[serde(rename="IMAGE_BOUNDING_POLY_ANNOTATION")]
    IMAGEBOUNDINGPOLYANNOTATION,
    

    /// Polyline annotations in an image.
    ///
    /// "IMAGE_POLYLINE_ANNOTATION"
    #[serde(rename="IMAGE_POLYLINE_ANNOTATION")]
    IMAGEPOLYLINEANNOTATION,
    

    /// Segmentation annotations in an image.
    ///
    /// "IMAGE_SEGMENTATION_ANNOTATION"
    #[serde(rename="IMAGE_SEGMENTATION_ANNOTATION")]
    IMAGESEGMENTATIONANNOTATION,
    

    /// Classification annotations in video shots.
    ///
    /// "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION"
    #[serde(rename="VIDEO_SHOTS_CLASSIFICATION_ANNOTATION")]
    VIDEOSHOTSCLASSIFICATIONANNOTATION,
    

    /// Video object tracking annotation.
    ///
    /// "VIDEO_OBJECT_TRACKING_ANNOTATION"
    #[serde(rename="VIDEO_OBJECT_TRACKING_ANNOTATION")]
    VIDEOOBJECTTRACKINGANNOTATION,
    

    /// Video object detection annotation.
    ///
    /// "VIDEO_OBJECT_DETECTION_ANNOTATION"
    #[serde(rename="VIDEO_OBJECT_DETECTION_ANNOTATION")]
    VIDEOOBJECTDETECTIONANNOTATION,
    

    /// Video event annotation.
    ///
    /// "VIDEO_EVENT_ANNOTATION"
    #[serde(rename="VIDEO_EVENT_ANNOTATION")]
    VIDEOEVENTANNOTATION,
    

    /// Classification for text. Allowed for continuous evaluation.
    ///
    /// "TEXT_CLASSIFICATION_ANNOTATION"
    #[serde(rename="TEXT_CLASSIFICATION_ANNOTATION")]
    TEXTCLASSIFICATIONANNOTATION,
    

    /// Entity extraction for text.
    ///
    /// "TEXT_ENTITY_EXTRACTION_ANNOTATION"
    #[serde(rename="TEXT_ENTITY_EXTRACTION_ANNOTATION")]
    TEXTENTITYEXTRACTIONANNOTATION,
    

    /// General classification. Allowed for continuous evaluation.
    ///
    /// "GENERAL_CLASSIFICATION_ANNOTATION"
    #[serde(rename="GENERAL_CLASSIFICATION_ANNOTATION")]
    GENERALCLASSIFICATIONANNOTATION,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::ANNOTATIONTYPEUNSPECIFIED => "ANNOTATION_TYPE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGECLASSIFICATIONANNOTATION => "IMAGE_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEBOUNDINGBOXANNOTATION => "IMAGE_BOUNDING_BOX_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEORIENTEDBOUNDINGBOXANNOTATION => "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEBOUNDINGPOLYANNOTATION => "IMAGE_BOUNDING_POLY_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEPOLYLINEANNOTATION => "IMAGE_POLYLINE_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGESEGMENTATIONANNOTATION => "IMAGE_SEGMENTATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOSHOTSCLASSIFICATIONANNOTATION => "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOOBJECTTRACKINGANNOTATION => "VIDEO_OBJECT_TRACKING_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOOBJECTDETECTIONANNOTATION => "VIDEO_OBJECT_DETECTION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOEVENTANNOTATION => "VIDEO_EVENT_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::TEXTCLASSIFICATIONANNOTATION => "TEXT_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::TEXTENTITYEXTRACTIONANNOTATION => "TEXT_ENTITY_EXTRACTION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::GENERALCLASSIFICATIONANNOTATION => "GENERAL_CLASSIFICATION_ANNOTATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::ANNOTATIONTYPEUNSPECIFIED),
           "IMAGE_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGECLASSIFICATIONANNOTATION),
           "IMAGE_BOUNDING_BOX_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEBOUNDINGBOXANNOTATION),
           "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEORIENTEDBOUNDINGBOXANNOTATION),
           "IMAGE_BOUNDING_POLY_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEBOUNDINGPOLYANNOTATION),
           "IMAGE_POLYLINE_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGEPOLYLINEANNOTATION),
           "IMAGE_SEGMENTATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::IMAGESEGMENTATIONANNOTATION),
           "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOSHOTSCLASSIFICATIONANNOTATION),
           "VIDEO_OBJECT_TRACKING_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOOBJECTTRACKINGANNOTATION),
           "VIDEO_OBJECT_DETECTION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOOBJECTDETECTIONANNOTATION),
           "VIDEO_EVENT_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::VIDEOEVENTANNOTATION),
           "TEXT_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::TEXTCLASSIFICATIONANNOTATION),
           "TEXT_ENTITY_EXTRACTION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::TEXTENTITYEXTRACTIONANNOTATION),
           "GENERAL_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum::GENERALCLASSIFICATIONANNOTATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1EvaluationAnnotationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Describes the current state of the job.
pub enum GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum {
    
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job is scheduled to run at the configured interval. You can pause or delete the job. When the job is in this state, it samples prediction input and output from your model version into your BigQuery table as predictions occur.
    ///
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
    

    /// The job is currently running. When the job runs, Data Labeling Service does several things: 1. If you have configured your job to use Data Labeling Service for ground truth labeling, the service creates a Dataset and a labeling task for all data sampled since the last time the job ran. Human labelers provide ground truth labels for your data. Human labeling may take hours, or even days, depending on how much data has been sampled. The job remains in the `RUNNING` state during this time, and it can even be running multiple times in parallel if it gets triggered again (for example 24 hours later) before the earlier run has completed. When human labelers have finished labeling the data, the next step occurs. If you have configured your job to provide your own ground truth labels, Data Labeling Service still creates a Dataset for newly sampled data, but it expects that you have already added ground truth labels to the BigQuery table by this time. The next step occurs immediately. 2. Data Labeling Service creates an Evaluation by comparing your model version's predictions with the ground truth labels. If the job remains in this state for a long time, it continues to sample prediction data into your BigQuery table and will run again at the next interval, even if it causes the job to run multiple times in parallel.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The job is not sampling prediction input and output into your BigQuery table and it will not run according to its schedule. You can resume the job.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The job has this state right before it is deleted.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::SCHEDULED => "SCHEDULED",
            GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::RUNNING => "RUNNING",
            GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::PAUSED => "PAUSED",
            GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::STOPPED => "STOPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::STATEUNSPECIFIED),
           "SCHEDULED" => Ok(GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::SCHEDULED),
           "RUNNING" => Ok(GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::RUNNING),
           "PAUSED" => Ok(GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::PAUSED),
           "STOPPED" => Ok(GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum::STOPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1EvaluationJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum {
    
    /// "FEEDBACK_THREAD_STATUS_UNSPECIFIED"
    #[serde(rename="FEEDBACK_THREAD_STATUS_UNSPECIFIED")]
    FEEDBACKTHREADSTATUSUNSPECIFIED,
    

    /// Feedback thread is created with no reply;
    ///
    /// "NEW"
    #[serde(rename="NEW")]
    NEW,
    

    /// Feedback thread is replied at least once;
    ///
    /// "REPLIED"
    #[serde(rename="REPLIED")]
    REPLIED,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum::FEEDBACKTHREADSTATUSUNSPECIFIED => "FEEDBACK_THREAD_STATUS_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum::NEW => "NEW",
            GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum::REPLIED => "REPLIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEEDBACK_THREAD_STATUS_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum::FEEDBACKTHREADSTATUSUNSPECIFIED),
           "NEW" => Ok(GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum::NEW),
           "REPLIED" => Ok(GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum::REPLIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1FeedbackThreadMetadataStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of how to aggregate answers.
pub enum GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum {
    
    /// "STRING_AGGREGATION_TYPE_UNSPECIFIED"
    #[serde(rename="STRING_AGGREGATION_TYPE_UNSPECIFIED")]
    STRINGAGGREGATIONTYPEUNSPECIFIED,
    

    /// Majority vote to aggregate answers.
    ///
    /// "MAJORITY_VOTE"
    #[serde(rename="MAJORITY_VOTE")]
    MAJORITYVOTE,
    

    /// Unanimous answers will be adopted.
    ///
    /// "UNANIMOUS_VOTE"
    #[serde(rename="UNANIMOUS_VOTE")]
    UNANIMOUSVOTE,
    

    /// Preserve all answers by crowd compute.
    ///
    /// "NO_AGGREGATION"
    #[serde(rename="NO_AGGREGATION")]
    NOAGGREGATION,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::STRINGAGGREGATIONTYPEUNSPECIFIED => "STRING_AGGREGATION_TYPE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::MAJORITYVOTE => "MAJORITY_VOTE",
            GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::UNANIMOUSVOTE => "UNANIMOUS_VOTE",
            GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::NOAGGREGATION => "NO_AGGREGATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STRING_AGGREGATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::STRINGAGGREGATIONTYPEUNSPECIFIED),
           "MAJORITY_VOTE" => Ok(GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::MAJORITYVOTE),
           "UNANIMOUS_VOTE" => Ok(GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::UNANIMOUSVOTE),
           "NO_AGGREGATION" => Ok(GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum::NOAGGREGATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1ImageClassificationConfigAnswerAggregationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of annotation to be performed on this data. You must specify this field if you are using this InputConfig in an EvaluationJob.
pub enum GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum {
    
    /// "ANNOTATION_TYPE_UNSPECIFIED"
    #[serde(rename="ANNOTATION_TYPE_UNSPECIFIED")]
    ANNOTATIONTYPEUNSPECIFIED,
    

    /// Classification annotations in an image. Allowed for continuous evaluation.
    ///
    /// "IMAGE_CLASSIFICATION_ANNOTATION"
    #[serde(rename="IMAGE_CLASSIFICATION_ANNOTATION")]
    IMAGECLASSIFICATIONANNOTATION,
    

    /// Bounding box annotations in an image. A form of image object detection. Allowed for continuous evaluation.
    ///
    /// "IMAGE_BOUNDING_BOX_ANNOTATION"
    #[serde(rename="IMAGE_BOUNDING_BOX_ANNOTATION")]
    IMAGEBOUNDINGBOXANNOTATION,
    

    /// Oriented bounding box. The box does not have to be parallel to horizontal line.
    ///
    /// "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION"
    #[serde(rename="IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION")]
    IMAGEORIENTEDBOUNDINGBOXANNOTATION,
    

    /// Bounding poly annotations in an image.
    ///
    /// "IMAGE_BOUNDING_POLY_ANNOTATION"
    #[serde(rename="IMAGE_BOUNDING_POLY_ANNOTATION")]
    IMAGEBOUNDINGPOLYANNOTATION,
    

    /// Polyline annotations in an image.
    ///
    /// "IMAGE_POLYLINE_ANNOTATION"
    #[serde(rename="IMAGE_POLYLINE_ANNOTATION")]
    IMAGEPOLYLINEANNOTATION,
    

    /// Segmentation annotations in an image.
    ///
    /// "IMAGE_SEGMENTATION_ANNOTATION"
    #[serde(rename="IMAGE_SEGMENTATION_ANNOTATION")]
    IMAGESEGMENTATIONANNOTATION,
    

    /// Classification annotations in video shots.
    ///
    /// "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION"
    #[serde(rename="VIDEO_SHOTS_CLASSIFICATION_ANNOTATION")]
    VIDEOSHOTSCLASSIFICATIONANNOTATION,
    

    /// Video object tracking annotation.
    ///
    /// "VIDEO_OBJECT_TRACKING_ANNOTATION"
    #[serde(rename="VIDEO_OBJECT_TRACKING_ANNOTATION")]
    VIDEOOBJECTTRACKINGANNOTATION,
    

    /// Video object detection annotation.
    ///
    /// "VIDEO_OBJECT_DETECTION_ANNOTATION"
    #[serde(rename="VIDEO_OBJECT_DETECTION_ANNOTATION")]
    VIDEOOBJECTDETECTIONANNOTATION,
    

    /// Video event annotation.
    ///
    /// "VIDEO_EVENT_ANNOTATION"
    #[serde(rename="VIDEO_EVENT_ANNOTATION")]
    VIDEOEVENTANNOTATION,
    

    /// Classification for text. Allowed for continuous evaluation.
    ///
    /// "TEXT_CLASSIFICATION_ANNOTATION"
    #[serde(rename="TEXT_CLASSIFICATION_ANNOTATION")]
    TEXTCLASSIFICATIONANNOTATION,
    

    /// Entity extraction for text.
    ///
    /// "TEXT_ENTITY_EXTRACTION_ANNOTATION"
    #[serde(rename="TEXT_ENTITY_EXTRACTION_ANNOTATION")]
    TEXTENTITYEXTRACTIONANNOTATION,
    

    /// General classification. Allowed for continuous evaluation.
    ///
    /// "GENERAL_CLASSIFICATION_ANNOTATION"
    #[serde(rename="GENERAL_CLASSIFICATION_ANNOTATION")]
    GENERALCLASSIFICATIONANNOTATION,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::ANNOTATIONTYPEUNSPECIFIED => "ANNOTATION_TYPE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGECLASSIFICATIONANNOTATION => "IMAGE_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEBOUNDINGBOXANNOTATION => "IMAGE_BOUNDING_BOX_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEORIENTEDBOUNDINGBOXANNOTATION => "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEBOUNDINGPOLYANNOTATION => "IMAGE_BOUNDING_POLY_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEPOLYLINEANNOTATION => "IMAGE_POLYLINE_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGESEGMENTATIONANNOTATION => "IMAGE_SEGMENTATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOSHOTSCLASSIFICATIONANNOTATION => "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOOBJECTTRACKINGANNOTATION => "VIDEO_OBJECT_TRACKING_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOOBJECTDETECTIONANNOTATION => "VIDEO_OBJECT_DETECTION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOEVENTANNOTATION => "VIDEO_EVENT_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::TEXTCLASSIFICATIONANNOTATION => "TEXT_CLASSIFICATION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::TEXTENTITYEXTRACTIONANNOTATION => "TEXT_ENTITY_EXTRACTION_ANNOTATION",
            GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::GENERALCLASSIFICATIONANNOTATION => "GENERAL_CLASSIFICATION_ANNOTATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOTATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::ANNOTATIONTYPEUNSPECIFIED),
           "IMAGE_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGECLASSIFICATIONANNOTATION),
           "IMAGE_BOUNDING_BOX_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEBOUNDINGBOXANNOTATION),
           "IMAGE_ORIENTED_BOUNDING_BOX_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEORIENTEDBOUNDINGBOXANNOTATION),
           "IMAGE_BOUNDING_POLY_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEBOUNDINGPOLYANNOTATION),
           "IMAGE_POLYLINE_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGEPOLYLINEANNOTATION),
           "IMAGE_SEGMENTATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::IMAGESEGMENTATIONANNOTATION),
           "VIDEO_SHOTS_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOSHOTSCLASSIFICATIONANNOTATION),
           "VIDEO_OBJECT_TRACKING_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOOBJECTTRACKINGANNOTATION),
           "VIDEO_OBJECT_DETECTION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOOBJECTDETECTIONANNOTATION),
           "VIDEO_EVENT_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::VIDEOEVENTANNOTATION),
           "TEXT_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::TEXTCLASSIFICATIONANNOTATION),
           "TEXT_ENTITY_EXTRACTION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::TEXTENTITYEXTRACTIONANNOTATION),
           "GENERAL_CLASSIFICATION_ANNOTATION" => Ok(GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum::GENERALCLASSIFICATIONANNOTATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1InputConfigAnnotationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Data type must be specifed when user tries to import data.
pub enum GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum {
    

    /// Data type is unspecified.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// Allowed for continuous evaluation.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// Video data type.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// Allowed for continuous evaluation.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Allowed for continuous evaluation.
    ///
    /// "GENERAL_DATA"
    #[serde(rename="GENERAL_DATA")]
    GENERALDATA,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::IMAGE => "IMAGE",
            GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::VIDEO => "VIDEO",
            GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::TEXT => "TEXT",
            GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::GENERALDATA => "GENERAL_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::DATATYPEUNSPECIFIED),
           "IMAGE" => Ok(GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::IMAGE),
           "VIDEO" => Ok(GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::VIDEO),
           "TEXT" => Ok(GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::TEXT),
           "GENERAL_DATA" => Ok(GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum::GENERALDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1InputConfigDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The data type of this instruction.
pub enum GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum {
    

    /// Data type is unspecified.
    ///
    /// "DATA_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_TYPE_UNSPECIFIED")]
    DATATYPEUNSPECIFIED,
    

    /// Allowed for continuous evaluation.
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// Video data type.
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// Allowed for continuous evaluation.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// Allowed for continuous evaluation.
    ///
    /// "GENERAL_DATA"
    #[serde(rename="GENERAL_DATA")]
    GENERALDATA,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::DATATYPEUNSPECIFIED => "DATA_TYPE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::IMAGE => "IMAGE",
            GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::VIDEO => "VIDEO",
            GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::TEXT => "TEXT",
            GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::GENERALDATA => "GENERAL_DATA",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::DATATYPEUNSPECIFIED),
           "IMAGE" => Ok(GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::IMAGE),
           "VIDEO" => Ok(GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::VIDEO),
           "TEXT" => Ok(GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::TEXT),
           "GENERAL_DATA" => Ok(GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum::GENERALDATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1InstructionDataTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of image labeling task.
pub enum GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum {
    
    /// "FEATURE_UNSPECIFIED"
    #[serde(rename="FEATURE_UNSPECIFIED")]
    FEATUREUNSPECIFIED,
    

    /// Label whole image with one or more of labels.
    ///
    /// "CLASSIFICATION"
    #[serde(rename="CLASSIFICATION")]
    CLASSIFICATION,
    

    /// Label image with bounding boxes for labels.
    ///
    /// "BOUNDING_BOX"
    #[serde(rename="BOUNDING_BOX")]
    BOUNDINGBOX,
    

    /// Label oriented bounding box. The box does not have to be parallel to horizontal line.
    ///
    /// "ORIENTED_BOUNDING_BOX"
    #[serde(rename="ORIENTED_BOUNDING_BOX")]
    ORIENTEDBOUNDINGBOX,
    

    /// Label images with bounding poly. A bounding poly is a plane figure that is bounded by a finite chain of straight line segments closing in a loop.
    ///
    /// "BOUNDING_POLY"
    #[serde(rename="BOUNDING_POLY")]
    BOUNDINGPOLY,
    

    /// Label images with polyline. Polyline is formed by connected line segments which are not in closed form.
    ///
    /// "POLYLINE"
    #[serde(rename="POLYLINE")]
    POLYLINE,
    

    /// Label images with segmentation. Segmentation is different from bounding poly since it is more fine-grained, pixel level annotation.
    ///
    /// "SEGMENTATION"
    #[serde(rename="SEGMENTATION")]
    SEGMENTATION,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::CLASSIFICATION => "CLASSIFICATION",
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::BOUNDINGBOX => "BOUNDING_BOX",
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::ORIENTEDBOUNDINGBOX => "ORIENTED_BOUNDING_BOX",
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::BOUNDINGPOLY => "BOUNDING_POLY",
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::POLYLINE => "POLYLINE",
            GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::SEGMENTATION => "SEGMENTATION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::FEATUREUNSPECIFIED),
           "CLASSIFICATION" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::CLASSIFICATION),
           "BOUNDING_BOX" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::BOUNDINGBOX),
           "ORIENTED_BOUNDING_BOX" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::ORIENTEDBOUNDINGBOX),
           "BOUNDING_POLY" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::BOUNDINGPOLY),
           "POLYLINE" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::POLYLINE),
           "SEGMENTATION" => Ok(GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum::SEGMENTATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1LabelImageRequestFeatureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of text labeling task.
pub enum GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum {
    
    /// "FEATURE_UNSPECIFIED"
    #[serde(rename="FEATURE_UNSPECIFIED")]
    FEATUREUNSPECIFIED,
    

    /// Label text content to one of more labels.
    ///
    /// "TEXT_CLASSIFICATION"
    #[serde(rename="TEXT_CLASSIFICATION")]
    TEXTCLASSIFICATION,
    

    /// Label entities and their span in text.
    ///
    /// "TEXT_ENTITY_EXTRACTION"
    #[serde(rename="TEXT_ENTITY_EXTRACTION")]
    TEXTENTITYEXTRACTION,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum::TEXTCLASSIFICATION => "TEXT_CLASSIFICATION",
            GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum::TEXTENTITYEXTRACTION => "TEXT_ENTITY_EXTRACTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum::FEATUREUNSPECIFIED),
           "TEXT_CLASSIFICATION" => Ok(GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum::TEXTCLASSIFICATION),
           "TEXT_ENTITY_EXTRACTION" => Ok(GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum::TEXTENTITYEXTRACTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1LabelTextRequestFeatureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of video labeling task.
pub enum GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum {
    
    /// "FEATURE_UNSPECIFIED"
    #[serde(rename="FEATURE_UNSPECIFIED")]
    FEATUREUNSPECIFIED,
    

    /// Label whole video or video segment with one or more labels.
    ///
    /// "CLASSIFICATION"
    #[serde(rename="CLASSIFICATION")]
    CLASSIFICATION,
    

    /// Label objects with bounding box on image frames extracted from the video.
    ///
    /// "OBJECT_DETECTION"
    #[serde(rename="OBJECT_DETECTION")]
    OBJECTDETECTION,
    

    /// Label and track objects in video.
    ///
    /// "OBJECT_TRACKING"
    #[serde(rename="OBJECT_TRACKING")]
    OBJECTTRACKING,
    

    /// Label the range of video for the specified events.
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
}

impl AsRef<str> for GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::CLASSIFICATION => "CLASSIFICATION",
            GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::OBJECTDETECTION => "OBJECT_DETECTION",
            GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::OBJECTTRACKING => "OBJECT_TRACKING",
            GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::EVENT => "EVENT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::FEATUREUNSPECIFIED),
           "CLASSIFICATION" => Ok(GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::CLASSIFICATION),
           "OBJECT_DETECTION" => Ok(GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::OBJECTDETECTION),
           "OBJECT_TRACKING" => Ok(GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::OBJECTTRACKING),
           "EVENT" => Ok(GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum::EVENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatalabelingV1beta1LabelVideoRequestFeatureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


