use super::*;



// region GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs
pub enum GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum {
    

    /// Run in the default mode.
    ///
    /// "FLEXRS_UNSPECIFIED"
    #[serde(rename="FLEXRS_UNSPECIFIED")]
    FLEXRSUNSPECIFIED,
    

    /// Optimize for lower execution time.
    ///
    /// "FLEXRS_SPEED_OPTIMIZED"
    #[serde(rename="FLEXRS_SPEED_OPTIMIZED")]
    FLEXRSSPEEDOPTIMIZED,
    

    /// Optimize for lower cost.
    ///
    /// "FLEXRS_COST_OPTIMIZED"
    #[serde(rename="FLEXRS_COST_OPTIMIZED")]
    FLEXRSCOSTOPTIMIZED,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum::FLEXRSUNSPECIFIED => "FLEXRS_UNSPECIFIED",
            GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum::FLEXRSSPEEDOPTIMIZED => "FLEXRS_SPEED_OPTIMIZED",
            GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum::FLEXRSCOSTOPTIMIZED => "FLEXRS_COST_OPTIMIZED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FLEXRS_UNSPECIFIED" => Ok(GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum::FLEXRSUNSPECIFIED),
           "FLEXRS_SPEED_OPTIMIZED" => Ok(GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum::FLEXRSSPEEDOPTIMIZED),
           "FLEXRS_COST_OPTIMIZED" => Ok(GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum::FLEXRSCOSTOPTIMIZED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentFlexrsGoalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Configuration for VM IPs.
pub enum GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum {
    

    /// The configuration is unknown, or unspecified.
    ///
    /// "WORKER_IP_UNSPECIFIED"
    #[serde(rename="WORKER_IP_UNSPECIFIED")]
    WORKERIPUNSPECIFIED,
    

    /// Workers should have public IP addresses.
    ///
    /// "WORKER_IP_PUBLIC"
    #[serde(rename="WORKER_IP_PUBLIC")]
    WORKERIPPUBLIC,
    

    /// Workers should have private IP addresses.
    ///
    /// "WORKER_IP_PRIVATE"
    #[serde(rename="WORKER_IP_PRIVATE")]
    WORKERIPPRIVATE,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum::WORKERIPUNSPECIFIED => "WORKER_IP_UNSPECIFIED",
            GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum::WORKERIPPUBLIC => "WORKER_IP_PUBLIC",
            GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum::WORKERIPPRIVATE => "WORKER_IP_PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WORKER_IP_UNSPECIFIED" => Ok(GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum::WORKERIPUNSPECIFIED),
           "WORKER_IP_PUBLIC" => Ok(GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum::WORKERIPPUBLIC),
           "WORKER_IP_PRIVATE" => Ok(GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum::WORKERIPPRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1FlexTemplateRuntimeEnvironmentIpConfigurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatapipelinesV1JobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the job.
pub enum GoogleCloudDatapipelinesV1JobStateEnum {
    

    /// The job state isn't specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job is waiting to start execution.
    ///
    /// "STATE_PENDING"
    #[serde(rename="STATE_PENDING")]
    STATEPENDING,
    

    /// The job is executing.
    ///
    /// "STATE_RUNNING"
    #[serde(rename="STATE_RUNNING")]
    STATERUNNING,
    

    /// The job has finished execution successfully.
    ///
    /// "STATE_DONE"
    #[serde(rename="STATE_DONE")]
    STATEDONE,
    

    /// The job has finished execution with a failure.
    ///
    /// "STATE_FAILED"
    #[serde(rename="STATE_FAILED")]
    STATEFAILED,
    

    /// The job has been terminated upon user request.
    ///
    /// "STATE_CANCELLED"
    #[serde(rename="STATE_CANCELLED")]
    STATECANCELLED,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1JobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1JobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDatapipelinesV1JobStateEnum::STATEPENDING => "STATE_PENDING",
            GoogleCloudDatapipelinesV1JobStateEnum::STATERUNNING => "STATE_RUNNING",
            GoogleCloudDatapipelinesV1JobStateEnum::STATEDONE => "STATE_DONE",
            GoogleCloudDatapipelinesV1JobStateEnum::STATEFAILED => "STATE_FAILED",
            GoogleCloudDatapipelinesV1JobStateEnum::STATECANCELLED => "STATE_CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1JobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDatapipelinesV1JobStateEnum::STATEUNSPECIFIED),
           "STATE_PENDING" => Ok(GoogleCloudDatapipelinesV1JobStateEnum::STATEPENDING),
           "STATE_RUNNING" => Ok(GoogleCloudDatapipelinesV1JobStateEnum::STATERUNNING),
           "STATE_DONE" => Ok(GoogleCloudDatapipelinesV1JobStateEnum::STATEDONE),
           "STATE_FAILED" => Ok(GoogleCloudDatapipelinesV1JobStateEnum::STATEFAILED),
           "STATE_CANCELLED" => Ok(GoogleCloudDatapipelinesV1JobStateEnum::STATECANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1JobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatapipelinesV1PipelineStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through UpdatePipeline requests.
pub enum GoogleCloudDatapipelinesV1PipelineStateEnum {
    

    /// The pipeline state isn't specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The pipeline is getting started or resumed. When finished, the pipeline state will be 'PIPELINE_STATE_ACTIVE'.
    ///
    /// "STATE_RESUMING"
    #[serde(rename="STATE_RESUMING")]
    STATERESUMING,
    

    /// The pipeline is actively running.
    ///
    /// "STATE_ACTIVE"
    #[serde(rename="STATE_ACTIVE")]
    STATEACTIVE,
    

    /// The pipeline is in the process of stopping. When finished, the pipeline state will be 'PIPELINE_STATE_ARCHIVED'.
    ///
    /// "STATE_STOPPING"
    #[serde(rename="STATE_STOPPING")]
    STATESTOPPING,
    

    /// The pipeline has been stopped. This is a terminal state and cannot be undone.
    ///
    /// "STATE_ARCHIVED"
    #[serde(rename="STATE_ARCHIVED")]
    STATEARCHIVED,
    

    /// The pipeline is paused. This is a non-terminal state. When the pipeline is paused, it will hold processing jobs, but can be resumed later. For a batch pipeline, this means pausing the scheduler job. For a streaming pipeline, creating a job snapshot to resume from will give the same effect.
    ///
    /// "STATE_PAUSED"
    #[serde(rename="STATE_PAUSED")]
    STATEPAUSED,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1PipelineStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1PipelineStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDatapipelinesV1PipelineStateEnum::STATERESUMING => "STATE_RESUMING",
            GoogleCloudDatapipelinesV1PipelineStateEnum::STATEACTIVE => "STATE_ACTIVE",
            GoogleCloudDatapipelinesV1PipelineStateEnum::STATESTOPPING => "STATE_STOPPING",
            GoogleCloudDatapipelinesV1PipelineStateEnum::STATEARCHIVED => "STATE_ARCHIVED",
            GoogleCloudDatapipelinesV1PipelineStateEnum::STATEPAUSED => "STATE_PAUSED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1PipelineStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDatapipelinesV1PipelineStateEnum::STATEUNSPECIFIED),
           "STATE_RESUMING" => Ok(GoogleCloudDatapipelinesV1PipelineStateEnum::STATERESUMING),
           "STATE_ACTIVE" => Ok(GoogleCloudDatapipelinesV1PipelineStateEnum::STATEACTIVE),
           "STATE_STOPPING" => Ok(GoogleCloudDatapipelinesV1PipelineStateEnum::STATESTOPPING),
           "STATE_ARCHIVED" => Ok(GoogleCloudDatapipelinesV1PipelineStateEnum::STATEARCHIVED),
           "STATE_PAUSED" => Ok(GoogleCloudDatapipelinesV1PipelineStateEnum::STATEPAUSED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1PipelineStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatapipelinesV1PipelineTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.
pub enum GoogleCloudDatapipelinesV1PipelineTypeEnum {
    

    /// The pipeline type isn't specified.
    ///
    /// "PIPELINE_TYPE_UNSPECIFIED"
    #[serde(rename="PIPELINE_TYPE_UNSPECIFIED")]
    PIPELINETYPEUNSPECIFIED,
    

    /// A batch pipeline. It runs jobs on a specific schedule, and each job will automatically terminate once execution is finished.
    ///
    /// "PIPELINE_TYPE_BATCH"
    #[serde(rename="PIPELINE_TYPE_BATCH")]
    PIPELINETYPEBATCH,
    

    /// A streaming pipeline. The underlying job is continuously running until it is manually terminated by the user. This type of pipeline doesn't have a schedule to run on, and the linked job gets created when the pipeline is created.
    ///
    /// "PIPELINE_TYPE_STREAMING"
    #[serde(rename="PIPELINE_TYPE_STREAMING")]
    PIPELINETYPESTREAMING,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1PipelineTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1PipelineTypeEnum::PIPELINETYPEUNSPECIFIED => "PIPELINE_TYPE_UNSPECIFIED",
            GoogleCloudDatapipelinesV1PipelineTypeEnum::PIPELINETYPEBATCH => "PIPELINE_TYPE_BATCH",
            GoogleCloudDatapipelinesV1PipelineTypeEnum::PIPELINETYPESTREAMING => "PIPELINE_TYPE_STREAMING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1PipelineTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PIPELINE_TYPE_UNSPECIFIED" => Ok(GoogleCloudDatapipelinesV1PipelineTypeEnum::PIPELINETYPEUNSPECIFIED),
           "PIPELINE_TYPE_BATCH" => Ok(GoogleCloudDatapipelinesV1PipelineTypeEnum::PIPELINETYPEBATCH),
           "PIPELINE_TYPE_STREAMING" => Ok(GoogleCloudDatapipelinesV1PipelineTypeEnum::PIPELINETYPESTREAMING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1PipelineTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Configuration for VM IPs.
pub enum GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum {
    

    /// The configuration is unknown, or unspecified.
    ///
    /// "WORKER_IP_UNSPECIFIED"
    #[serde(rename="WORKER_IP_UNSPECIFIED")]
    WORKERIPUNSPECIFIED,
    

    /// Workers should have public IP addresses.
    ///
    /// "WORKER_IP_PUBLIC"
    #[serde(rename="WORKER_IP_PUBLIC")]
    WORKERIPPUBLIC,
    

    /// Workers should have private IP addresses.
    ///
    /// "WORKER_IP_PRIVATE"
    #[serde(rename="WORKER_IP_PRIVATE")]
    WORKERIPPRIVATE,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum::WORKERIPUNSPECIFIED => "WORKER_IP_UNSPECIFIED",
            GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum::WORKERIPPUBLIC => "WORKER_IP_PUBLIC",
            GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum::WORKERIPPRIVATE => "WORKER_IP_PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WORKER_IP_UNSPECIFIED" => Ok(GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum::WORKERIPUNSPECIFIED),
           "WORKER_IP_PUBLIC" => Ok(GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum::WORKERIPPUBLIC),
           "WORKER_IP_PRIVATE" => Ok(GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum::WORKERIPPRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1RuntimeEnvironmentIpConfigurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The support status for this SDK version.
pub enum GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum {
    

    /// Dataflow is unaware of this version.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// This is a known version of an SDK, and is supported.
    ///
    /// "SUPPORTED"
    #[serde(rename="SUPPORTED")]
    SUPPORTED,
    

    /// A newer version of the SDK exists, and an update is recommended.
    ///
    /// "STALE"
    #[serde(rename="STALE")]
    STALE,
    

    /// This version of the SDK is deprecated and will eventually be unsupported.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// Support for this SDK version has ended and it should no longer be used.
    ///
    /// "UNSUPPORTED"
    #[serde(rename="UNSUPPORTED")]
    UNSUPPORTED,
}

impl AsRef<str> for GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::UNKNOWN => "UNKNOWN",
            GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::SUPPORTED => "SUPPORTED",
            GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::STALE => "STALE",
            GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::DEPRECATED => "DEPRECATED",
            GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::UNSUPPORTED => "UNSUPPORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::UNKNOWN),
           "SUPPORTED" => Ok(GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::SUPPORTED),
           "STALE" => Ok(GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::STALE),
           "DEPRECATED" => Ok(GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::DEPRECATED),
           "UNSUPPORTED" => Ok(GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum::UNSUPPORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDatapipelinesV1SdkVersionSdkSupportStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


