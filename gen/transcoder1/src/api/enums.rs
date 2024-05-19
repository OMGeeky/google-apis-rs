use super::*;



// region AnimationFadeFadeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of fade animation: `FADE_IN` or `FADE_OUT`.
pub enum AnimationFadeFadeTypeEnum {
    

    /// The fade type is not specified.
    ///
    /// "FADE_TYPE_UNSPECIFIED"
    #[serde(rename="FADE_TYPE_UNSPECIFIED")]
    FADETYPEUNSPECIFIED,
    

    /// Fade the overlay object into view.
    ///
    /// "FADE_IN"
    #[serde(rename="FADE_IN")]
    FADEIN,
    

    /// Fade the overlay object out of view.
    ///
    /// "FADE_OUT"
    #[serde(rename="FADE_OUT")]
    FADEOUT,
}

impl AsRef<str> for AnimationFadeFadeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnimationFadeFadeTypeEnum::FADETYPEUNSPECIFIED => "FADE_TYPE_UNSPECIFIED",
            AnimationFadeFadeTypeEnum::FADEIN => "FADE_IN",
            AnimationFadeFadeTypeEnum::FADEOUT => "FADE_OUT",
        }
    }
}

impl std::convert::TryFrom< &str> for AnimationFadeFadeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FADE_TYPE_UNSPECIFIED" => Ok(AnimationFadeFadeTypeEnum::FADETYPEUNSPECIFIED),
           "FADE_IN" => Ok(AnimationFadeFadeTypeEnum::FADEIN),
           "FADE_OUT" => Ok(AnimationFadeFadeTypeEnum::FADEOUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnimationFadeFadeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DashConfigSegmentReferenceSchemeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The segment reference scheme for a `DASH` manifest. The default is `SEGMENT_LIST`.
pub enum DashConfigSegmentReferenceSchemeEnum {
    

    /// The segment reference scheme is not specified.
    ///
    /// "SEGMENT_REFERENCE_SCHEME_UNSPECIFIED"
    #[serde(rename="SEGMENT_REFERENCE_SCHEME_UNSPECIFIED")]
    SEGMENTREFERENCESCHEMEUNSPECIFIED,
    

    /// Explicitly lists the URLs of media files for each segment. For example, if SegmentSettings.individual_segments is `true`, then the manifest contains fields similar to the following: ```xml ... ```
    ///
    /// "SEGMENT_LIST"
    #[serde(rename="SEGMENT_LIST")]
    SEGMENTLIST,
    

    /// SegmentSettings.individual_segments must be set to `true` to use this segment reference scheme. Uses the DASH specification `` tag to determine the URLs of media files for each segment. For example: ```xml ... ```
    ///
    /// "SEGMENT_TEMPLATE_NUMBER"
    #[serde(rename="SEGMENT_TEMPLATE_NUMBER")]
    SEGMENTTEMPLATENUMBER,
}

impl AsRef<str> for DashConfigSegmentReferenceSchemeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DashConfigSegmentReferenceSchemeEnum::SEGMENTREFERENCESCHEMEUNSPECIFIED => "SEGMENT_REFERENCE_SCHEME_UNSPECIFIED",
            DashConfigSegmentReferenceSchemeEnum::SEGMENTLIST => "SEGMENT_LIST",
            DashConfigSegmentReferenceSchemeEnum::SEGMENTTEMPLATENUMBER => "SEGMENT_TEMPLATE_NUMBER",
        }
    }
}

impl std::convert::TryFrom< &str> for DashConfigSegmentReferenceSchemeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SEGMENT_REFERENCE_SCHEME_UNSPECIFIED" => Ok(DashConfigSegmentReferenceSchemeEnum::SEGMENTREFERENCESCHEMEUNSPECIFIED),
           "SEGMENT_LIST" => Ok(DashConfigSegmentReferenceSchemeEnum::SEGMENTLIST),
           "SEGMENT_TEMPLATE_NUMBER" => Ok(DashConfigSegmentReferenceSchemeEnum::SEGMENTTEMPLATENUMBER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DashConfigSegmentReferenceSchemeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region H264CodecSettingFrameRateConversionStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Frame rate conversion strategy for desired frame rate. The default is `DOWNSAMPLE`.
pub enum H264CodecSettingFrameRateConversionStrategyEnum {
    

    /// Unspecified frame rate conversion strategy.
    ///
    /// "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED"
    #[serde(rename="FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED")]
    FRAMERATECONVERSIONSTRATEGYUNSPECIFIED,
    

    /// Selectively retain frames to reduce the output frame rate. Every _n_ th frame is kept, where `n = ceil(input frame rate / target frame rate)`. When _n_ = 1 (that is, the target frame rate is greater than the input frame rate), the output frame rate matches the input frame rate. When _n_ > 1, frames are dropped and the output frame rate is equal to `(input frame rate / n)`. For more information, see [Calculate frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate).
    ///
    /// "DOWNSAMPLE"
    #[serde(rename="DOWNSAMPLE")]
    DOWNSAMPLE,
    

    /// Drop or duplicate frames to match the specified frame rate.
    ///
    /// "DROP_DUPLICATE"
    #[serde(rename="DROP_DUPLICATE")]
    DROPDUPLICATE,
}

impl AsRef<str> for H264CodecSettingFrameRateConversionStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            H264CodecSettingFrameRateConversionStrategyEnum::FRAMERATECONVERSIONSTRATEGYUNSPECIFIED => "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED",
            H264CodecSettingFrameRateConversionStrategyEnum::DOWNSAMPLE => "DOWNSAMPLE",
            H264CodecSettingFrameRateConversionStrategyEnum::DROPDUPLICATE => "DROP_DUPLICATE",
        }
    }
}

impl std::convert::TryFrom< &str> for H264CodecSettingFrameRateConversionStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED" => Ok(H264CodecSettingFrameRateConversionStrategyEnum::FRAMERATECONVERSIONSTRATEGYUNSPECIFIED),
           "DOWNSAMPLE" => Ok(H264CodecSettingFrameRateConversionStrategyEnum::DOWNSAMPLE),
           "DROP_DUPLICATE" => Ok(H264CodecSettingFrameRateConversionStrategyEnum::DROPDUPLICATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a H264CodecSettingFrameRateConversionStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region H265CodecSettingFrameRateConversionStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Frame rate conversion strategy for desired frame rate. The default is `DOWNSAMPLE`.
pub enum H265CodecSettingFrameRateConversionStrategyEnum {
    

    /// Unspecified frame rate conversion strategy.
    ///
    /// "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED"
    #[serde(rename="FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED")]
    FRAMERATECONVERSIONSTRATEGYUNSPECIFIED,
    

    /// Selectively retain frames to reduce the output frame rate. Every _n_ th frame is kept, where `n = ceil(input frame rate / target frame rate)`. When _n_ = 1 (that is, the target frame rate is greater than the input frame rate), the output frame rate matches the input frame rate. When _n_ > 1, frames are dropped and the output frame rate is equal to `(input frame rate / n)`. For more information, see [Calculate frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate).
    ///
    /// "DOWNSAMPLE"
    #[serde(rename="DOWNSAMPLE")]
    DOWNSAMPLE,
    

    /// Drop or duplicate frames to match the specified frame rate.
    ///
    /// "DROP_DUPLICATE"
    #[serde(rename="DROP_DUPLICATE")]
    DROPDUPLICATE,
}

impl AsRef<str> for H265CodecSettingFrameRateConversionStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            H265CodecSettingFrameRateConversionStrategyEnum::FRAMERATECONVERSIONSTRATEGYUNSPECIFIED => "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED",
            H265CodecSettingFrameRateConversionStrategyEnum::DOWNSAMPLE => "DOWNSAMPLE",
            H265CodecSettingFrameRateConversionStrategyEnum::DROPDUPLICATE => "DROP_DUPLICATE",
        }
    }
}

impl std::convert::TryFrom< &str> for H265CodecSettingFrameRateConversionStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED" => Ok(H265CodecSettingFrameRateConversionStrategyEnum::FRAMERATECONVERSIONSTRATEGYUNSPECIFIED),
           "DOWNSAMPLE" => Ok(H265CodecSettingFrameRateConversionStrategyEnum::DOWNSAMPLE),
           "DROP_DUPLICATE" => Ok(H265CodecSettingFrameRateConversionStrategyEnum::DROPDUPLICATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a H265CodecSettingFrameRateConversionStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The processing mode of the job. The default is `PROCESSING_MODE_INTERACTIVE`.
pub enum JobModeEnum {
    

    /// The job processing mode is not specified.
    ///
    /// "PROCESSING_MODE_UNSPECIFIED"
    #[serde(rename="PROCESSING_MODE_UNSPECIFIED")]
    PROCESSINGMODEUNSPECIFIED,
    

    /// The job processing mode is interactive mode. Interactive job will either be ran or rejected if quota does not allow for it.
    ///
    /// "PROCESSING_MODE_INTERACTIVE"
    #[serde(rename="PROCESSING_MODE_INTERACTIVE")]
    PROCESSINGMODEINTERACTIVE,
    

    /// The job processing mode is batch mode. Batch mode allows queuing of jobs.
    ///
    /// "PROCESSING_MODE_BATCH"
    #[serde(rename="PROCESSING_MODE_BATCH")]
    PROCESSINGMODEBATCH,
}

impl AsRef<str> for JobModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobModeEnum::PROCESSINGMODEUNSPECIFIED => "PROCESSING_MODE_UNSPECIFIED",
            JobModeEnum::PROCESSINGMODEINTERACTIVE => "PROCESSING_MODE_INTERACTIVE",
            JobModeEnum::PROCESSINGMODEBATCH => "PROCESSING_MODE_BATCH",
        }
    }
}

impl std::convert::TryFrom< &str> for JobModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROCESSING_MODE_UNSPECIFIED" => Ok(JobModeEnum::PROCESSINGMODEUNSPECIFIED),
           "PROCESSING_MODE_INTERACTIVE" => Ok(JobModeEnum::PROCESSINGMODEINTERACTIVE),
           "PROCESSING_MODE_BATCH" => Ok(JobModeEnum::PROCESSINGMODEBATCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobOptimizationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The optimization strategy of the job. The default is `AUTODETECT`.
pub enum JobOptimizationEnum {
    

    /// The optimization strategy is not specified.
    ///
    /// "OPTIMIZATION_STRATEGY_UNSPECIFIED"
    #[serde(rename="OPTIMIZATION_STRATEGY_UNSPECIFIED")]
    OPTIMIZATIONSTRATEGYUNSPECIFIED,
    

    /// Prioritize job processing speed.
    ///
    /// "AUTODETECT"
    #[serde(rename="AUTODETECT")]
    AUTODETECT,
    

    /// Disable all optimizations.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for JobOptimizationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobOptimizationEnum::OPTIMIZATIONSTRATEGYUNSPECIFIED => "OPTIMIZATION_STRATEGY_UNSPECIFIED",
            JobOptimizationEnum::AUTODETECT => "AUTODETECT",
            JobOptimizationEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for JobOptimizationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPTIMIZATION_STRATEGY_UNSPECIFIED" => Ok(JobOptimizationEnum::OPTIMIZATIONSTRATEGYUNSPECIFIED),
           "AUTODETECT" => Ok(JobOptimizationEnum::AUTODETECT),
           "DISABLED" => Ok(JobOptimizationEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobOptimizationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the job.
pub enum JobStateEnum {
    

    /// The processing state is not specified.
    ///
    /// "PROCESSING_STATE_UNSPECIFIED"
    #[serde(rename="PROCESSING_STATE_UNSPECIFIED")]
    PROCESSINGSTATEUNSPECIFIED,
    

    /// The job is enqueued and will be picked up for processing soon.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The job is being processed.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The job has been completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The job has failed. For additional information, see [Troubleshooting](https://cloud.google.com/transcoder/docs/troubleshooting).
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for JobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobStateEnum::PROCESSINGSTATEUNSPECIFIED => "PROCESSING_STATE_UNSPECIFIED",
            JobStateEnum::PENDING => "PENDING",
            JobStateEnum::RUNNING => "RUNNING",
            JobStateEnum::SUCCEEDED => "SUCCEEDED",
            JobStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for JobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROCESSING_STATE_UNSPECIFIED" => Ok(JobStateEnum::PROCESSINGSTATEUNSPECIFIED),
           "PENDING" => Ok(JobStateEnum::PENDING),
           "RUNNING" => Ok(JobStateEnum::RUNNING),
           "SUCCEEDED" => Ok(JobStateEnum::SUCCEEDED),
           "FAILED" => Ok(JobStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ManifestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of the manifest.
pub enum ManifestTypeEnum {
    

    /// The manifest type is not specified.
    ///
    /// "MANIFEST_TYPE_UNSPECIFIED"
    #[serde(rename="MANIFEST_TYPE_UNSPECIFIED")]
    MANIFESTTYPEUNSPECIFIED,
    

    /// Create an HLS manifest. The corresponding file extension is `.m3u8`.
    ///
    /// "HLS"
    #[serde(rename="HLS")]
    HLS,
    

    /// Create an MPEG-DASH manifest. The corresponding file extension is `.mpd`.
    ///
    /// "DASH"
    #[serde(rename="DASH")]
    DASH,
}

impl AsRef<str> for ManifestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ManifestTypeEnum::MANIFESTTYPEUNSPECIFIED => "MANIFEST_TYPE_UNSPECIFIED",
            ManifestTypeEnum::HLS => "HLS",
            ManifestTypeEnum::DASH => "DASH",
        }
    }
}

impl std::convert::TryFrom< &str> for ManifestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANIFEST_TYPE_UNSPECIFIED" => Ok(ManifestTypeEnum::MANIFESTTYPEUNSPECIFIED),
           "HLS" => Ok(ManifestTypeEnum::HLS),
           "DASH" => Ok(ManifestTypeEnum::DASH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ManifestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region Vp9CodecSettingFrameRateConversionStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Frame rate conversion strategy for desired frame rate. The default is `DOWNSAMPLE`.
pub enum Vp9CodecSettingFrameRateConversionStrategyEnum {
    

    /// Unspecified frame rate conversion strategy.
    ///
    /// "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED"
    #[serde(rename="FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED")]
    FRAMERATECONVERSIONSTRATEGYUNSPECIFIED,
    

    /// Selectively retain frames to reduce the output frame rate. Every _n_ th frame is kept, where `n = ceil(input frame rate / target frame rate)`. When _n_ = 1 (that is, the target frame rate is greater than the input frame rate), the output frame rate matches the input frame rate. When _n_ > 1, frames are dropped and the output frame rate is equal to `(input frame rate / n)`. For more information, see [Calculate frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate).
    ///
    /// "DOWNSAMPLE"
    #[serde(rename="DOWNSAMPLE")]
    DOWNSAMPLE,
    

    /// Drop or duplicate frames to match the specified frame rate.
    ///
    /// "DROP_DUPLICATE"
    #[serde(rename="DROP_DUPLICATE")]
    DROPDUPLICATE,
}

impl AsRef<str> for Vp9CodecSettingFrameRateConversionStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            Vp9CodecSettingFrameRateConversionStrategyEnum::FRAMERATECONVERSIONSTRATEGYUNSPECIFIED => "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED",
            Vp9CodecSettingFrameRateConversionStrategyEnum::DOWNSAMPLE => "DOWNSAMPLE",
            Vp9CodecSettingFrameRateConversionStrategyEnum::DROPDUPLICATE => "DROP_DUPLICATE",
        }
    }
}

impl std::convert::TryFrom< &str> for Vp9CodecSettingFrameRateConversionStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FRAME_RATE_CONVERSION_STRATEGY_UNSPECIFIED" => Ok(Vp9CodecSettingFrameRateConversionStrategyEnum::FRAMERATECONVERSIONSTRATEGYUNSPECIFIED),
           "DOWNSAMPLE" => Ok(Vp9CodecSettingFrameRateConversionStrategyEnum::DOWNSAMPLE),
           "DROP_DUPLICATE" => Ok(Vp9CodecSettingFrameRateConversionStrategyEnum::DROPDUPLICATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a Vp9CodecSettingFrameRateConversionStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


