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
    

    /// The job has failed. For additional information, see `failure_reason` and `failure_details`
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
/// Required. Type of the manifest, can be "HLS" or "DASH".
pub enum ManifestTypeEnum {
    

    /// The manifest type is not specified.
    ///
    /// "MANIFEST_TYPE_UNSPECIFIED"
    #[serde(rename="MANIFEST_TYPE_UNSPECIFIED")]
    MANIFESTTYPEUNSPECIFIED,
    

    /// Create `"HLS"` manifest. The corresponding file extension is `".m3u8"`.
    ///
    /// "HLS"
    #[serde(rename="HLS")]
    HLS,
    

    /// Create `"DASH"` manifest. The corresponding file extension is `".mpd"`.
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


