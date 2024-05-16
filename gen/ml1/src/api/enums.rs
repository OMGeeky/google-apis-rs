use super::*;



// region GoogleCloudMlV1StudyConfigMetricSpecGoalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The optimization goal of the metric.
pub enum GoogleCloudMlV1StudyConfigMetricSpecGoalEnum {
    

    /// Goal Type will default to maximize.
    ///
    /// "GOAL_TYPE_UNSPECIFIED"
    #[serde(rename="GOAL_TYPE_UNSPECIFIED")]
    GOALTYPEUNSPECIFIED,
    

    /// Maximize the goal metric.
    ///
    /// "MAXIMIZE"
    #[serde(rename="MAXIMIZE")]
    MAXIMIZE,
    

    /// Minimize the goal metric.
    ///
    /// "MINIMIZE"
    #[serde(rename="MINIMIZE")]
    MINIMIZE,
}

impl AsRef<str> for GoogleCloudMlV1StudyConfigMetricSpecGoalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1StudyConfigMetricSpecGoalEnum::GOALTYPEUNSPECIFIED => "GOAL_TYPE_UNSPECIFIED",
            GoogleCloudMlV1StudyConfigMetricSpecGoalEnum::MAXIMIZE => "MAXIMIZE",
            GoogleCloudMlV1StudyConfigMetricSpecGoalEnum::MINIMIZE => "MINIMIZE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1StudyConfigMetricSpecGoalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GOAL_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1StudyConfigMetricSpecGoalEnum::GOALTYPEUNSPECIFIED),
           "MAXIMIZE" => Ok(GoogleCloudMlV1StudyConfigMetricSpecGoalEnum::MAXIMIZE),
           "MINIMIZE" => Ok(GoogleCloudMlV1StudyConfigMetricSpecGoalEnum::MINIMIZE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1StudyConfigMetricSpecGoalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the parameter should be scaled. Leave unset for categorical parameters.
pub enum GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum {
    

    /// By default, no scaling is applied.
    ///
    /// "SCALE_TYPE_UNSPECIFIED"
    #[serde(rename="SCALE_TYPE_UNSPECIFIED")]
    SCALETYPEUNSPECIFIED,
    

    /// Scales the feasible space to (0, 1) linearly.
    ///
    /// "UNIT_LINEAR_SCALE"
    #[serde(rename="UNIT_LINEAR_SCALE")]
    UNITLINEARSCALE,
    

    /// Scales the feasible space logarithmically to (0, 1). The entire feasible space must be strictly positive.
    ///
    /// "UNIT_LOG_SCALE"
    #[serde(rename="UNIT_LOG_SCALE")]
    UNITLOGSCALE,
    

    /// Scales the feasible space "reverse" logarithmically to (0, 1). The result is that values close to the top of the feasible space are spread out more than points near the bottom. The entire feasible space must be strictly positive.
    ///
    /// "UNIT_REVERSE_LOG_SCALE"
    #[serde(rename="UNIT_REVERSE_LOG_SCALE")]
    UNITREVERSELOGSCALE,
}

impl AsRef<str> for GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::SCALETYPEUNSPECIFIED => "SCALE_TYPE_UNSPECIFIED",
            GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::UNITLINEARSCALE => "UNIT_LINEAR_SCALE",
            GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::UNITLOGSCALE => "UNIT_LOG_SCALE",
            GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::UNITREVERSELOGSCALE => "UNIT_REVERSE_LOG_SCALE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCALE_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::SCALETYPEUNSPECIFIED),
           "UNIT_LINEAR_SCALE" => Ok(GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::UNITLINEARSCALE),
           "UNIT_LOG_SCALE" => Ok(GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::UNITLOGSCALE),
           "UNIT_REVERSE_LOG_SCALE" => Ok(GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum::UNITREVERSELOGSCALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1StudyConfigParameterSpecScaleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1StudyConfigParameterSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the parameter.
pub enum GoogleCloudMlV1StudyConfigParameterSpecTypeEnum {
    

    /// You must specify a valid type. Using this unspecified type will result in an error.
    ///
    /// "PARAMETER_TYPE_UNSPECIFIED"
    #[serde(rename="PARAMETER_TYPE_UNSPECIFIED")]
    PARAMETERTYPEUNSPECIFIED,
    

    /// Type for real-valued parameters.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// Type for integral parameters.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// The parameter is categorical, with a value chosen from the categories field.
    ///
    /// "CATEGORICAL"
    #[serde(rename="CATEGORICAL")]
    CATEGORICAL,
    

    /// The parameter is real valued, with a fixed set of feasible points. If `type==DISCRETE`, feasible_points must be provided, and {`min_value`, `max_value`} will be ignored.
    ///
    /// "DISCRETE"
    #[serde(rename="DISCRETE")]
    DISCRETE,
}

impl AsRef<str> for GoogleCloudMlV1StudyConfigParameterSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::PARAMETERTYPEUNSPECIFIED => "PARAMETER_TYPE_UNSPECIFIED",
            GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::INTEGER => "INTEGER",
            GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::CATEGORICAL => "CATEGORICAL",
            GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::DISCRETE => "DISCRETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1StudyConfigParameterSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARAMETER_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::PARAMETERTYPEUNSPECIFIED),
           "DOUBLE" => Ok(GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::DOUBLE),
           "INTEGER" => Ok(GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::INTEGER),
           "CATEGORICAL" => Ok(GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::CATEGORICAL),
           "DISCRETE" => Ok(GoogleCloudMlV1StudyConfigParameterSpecTypeEnum::DISCRETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1StudyConfigParameterSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1AcceleratorConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of accelerator to use.
pub enum GoogleCloudMlV1AcceleratorConfigTypeEnum {
    

    /// Unspecified accelerator type. Default to no GPU.
    ///
    /// "ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="ACCELERATOR_TYPE_UNSPECIFIED")]
    ACCELERATORTYPEUNSPECIFIED,
    

    /// Nvidia Tesla K80 GPU.
    ///
    /// "NVIDIA_TESLA_K80"
    #[serde(rename="NVIDIA_TESLA_K80")]
    NVIDIATESLAK80,
    

    /// Nvidia Tesla P100 GPU.
    ///
    /// "NVIDIA_TESLA_P100"
    #[serde(rename="NVIDIA_TESLA_P100")]
    NVIDIATESLAP100,
    

    /// Nvidia V100 GPU.
    ///
    /// "NVIDIA_TESLA_V100"
    #[serde(rename="NVIDIA_TESLA_V100")]
    NVIDIATESLAV100,
    

    /// Nvidia Tesla P4 GPU.
    ///
    /// "NVIDIA_TESLA_P4"
    #[serde(rename="NVIDIA_TESLA_P4")]
    NVIDIATESLAP4,
    

    /// Nvidia T4 GPU.
    ///
    /// "NVIDIA_TESLA_T4"
    #[serde(rename="NVIDIA_TESLA_T4")]
    NVIDIATESLAT4,
    

    /// Nvidia A100 GPU.
    ///
    /// "NVIDIA_TESLA_A100"
    #[serde(rename="NVIDIA_TESLA_A100")]
    NVIDIATESLAA100,
    

    /// TPU v2.
    ///
    /// "TPU_V2"
    #[serde(rename="TPU_V2")]
    TPUV2,
    

    /// TPU v3.
    ///
    /// "TPU_V3"
    #[serde(rename="TPU_V3")]
    TPUV3,
    

    /// TPU v2 POD.
    ///
    /// "TPU_V2_POD"
    #[serde(rename="TPU_V2_POD")]
    TPUV2POD,
    

    /// TPU v3 POD.
    ///
    /// "TPU_V3_POD"
    #[serde(rename="TPU_V3_POD")]
    TPUV3POD,
    

    /// TPU v4 POD.
    ///
    /// "TPU_V4_POD"
    #[serde(rename="TPU_V4_POD")]
    TPUV4POD,
}

impl AsRef<str> for GoogleCloudMlV1AcceleratorConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1AcceleratorConfigTypeEnum::ACCELERATORTYPEUNSPECIFIED => "ACCELERATOR_TYPE_UNSPECIFIED",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAK80 => "NVIDIA_TESLA_K80",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAP100 => "NVIDIA_TESLA_P100",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAV100 => "NVIDIA_TESLA_V100",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAP4 => "NVIDIA_TESLA_P4",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAT4 => "NVIDIA_TESLA_T4",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAA100 => "NVIDIA_TESLA_A100",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV2 => "TPU_V2",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV3 => "TPU_V3",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV2POD => "TPU_V2_POD",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV3POD => "TPU_V3_POD",
            GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV4POD => "TPU_V4_POD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1AcceleratorConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCELERATOR_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::ACCELERATORTYPEUNSPECIFIED),
           "NVIDIA_TESLA_K80" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAK80),
           "NVIDIA_TESLA_P100" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAP100),
           "NVIDIA_TESLA_V100" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAV100),
           "NVIDIA_TESLA_P4" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAP4),
           "NVIDIA_TESLA_T4" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAT4),
           "NVIDIA_TESLA_A100" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::NVIDIATESLAA100),
           "TPU_V2" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV2),
           "TPU_V3" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV3),
           "TPU_V2_POD" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV2POD),
           "TPU_V3_POD" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV3POD),
           "TPU_V4_POD" => Ok(GoogleCloudMlV1AcceleratorConfigTypeEnum::TPUV4POD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1AcceleratorConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Available accelerators for the capability.
pub enum GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum {
    

    /// Unspecified accelerator type. Default to no GPU.
    ///
    /// "ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="ACCELERATOR_TYPE_UNSPECIFIED")]
    ACCELERATORTYPEUNSPECIFIED,
    

    /// Nvidia Tesla K80 GPU.
    ///
    /// "NVIDIA_TESLA_K80"
    #[serde(rename="NVIDIA_TESLA_K80")]
    NVIDIATESLAK80,
    

    /// Nvidia Tesla P100 GPU.
    ///
    /// "NVIDIA_TESLA_P100"
    #[serde(rename="NVIDIA_TESLA_P100")]
    NVIDIATESLAP100,
    

    /// Nvidia V100 GPU.
    ///
    /// "NVIDIA_TESLA_V100"
    #[serde(rename="NVIDIA_TESLA_V100")]
    NVIDIATESLAV100,
    

    /// Nvidia Tesla P4 GPU.
    ///
    /// "NVIDIA_TESLA_P4"
    #[serde(rename="NVIDIA_TESLA_P4")]
    NVIDIATESLAP4,
    

    /// Nvidia T4 GPU.
    ///
    /// "NVIDIA_TESLA_T4"
    #[serde(rename="NVIDIA_TESLA_T4")]
    NVIDIATESLAT4,
    

    /// Nvidia A100 GPU.
    ///
    /// "NVIDIA_TESLA_A100"
    #[serde(rename="NVIDIA_TESLA_A100")]
    NVIDIATESLAA100,
    

    /// TPU v2.
    ///
    /// "TPU_V2"
    #[serde(rename="TPU_V2")]
    TPUV2,
    

    /// TPU v3.
    ///
    /// "TPU_V3"
    #[serde(rename="TPU_V3")]
    TPUV3,
    

    /// TPU v2 POD.
    ///
    /// "TPU_V2_POD"
    #[serde(rename="TPU_V2_POD")]
    TPUV2POD,
    

    /// TPU v3 POD.
    ///
    /// "TPU_V3_POD"
    #[serde(rename="TPU_V3_POD")]
    TPUV3POD,
    

    /// TPU v4 POD.
    ///
    /// "TPU_V4_POD"
    #[serde(rename="TPU_V4_POD")]
    TPUV4POD,
}

impl AsRef<str> for GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::ACCELERATORTYPEUNSPECIFIED => "ACCELERATOR_TYPE_UNSPECIFIED",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAK80 => "NVIDIA_TESLA_K80",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAP100 => "NVIDIA_TESLA_P100",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAV100 => "NVIDIA_TESLA_V100",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAP4 => "NVIDIA_TESLA_P4",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAT4 => "NVIDIA_TESLA_T4",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAA100 => "NVIDIA_TESLA_A100",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV2 => "TPU_V2",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV3 => "TPU_V3",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV2POD => "TPU_V2_POD",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV3POD => "TPU_V3_POD",
            GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV4POD => "TPU_V4_POD",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCELERATOR_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::ACCELERATORTYPEUNSPECIFIED),
           "NVIDIA_TESLA_K80" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAK80),
           "NVIDIA_TESLA_P100" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAP100),
           "NVIDIA_TESLA_V100" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAV100),
           "NVIDIA_TESLA_P4" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAP4),
           "NVIDIA_TESLA_T4" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAT4),
           "NVIDIA_TESLA_A100" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::NVIDIATESLAA100),
           "TPU_V2" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV2),
           "TPU_V3" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV3),
           "TPU_V2_POD" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV2POD),
           "TPU_V3_POD" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV3POD),
           "TPU_V4_POD" => Ok(GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum::TPUV4POD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1CapabilityAvailableAcceleratorsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1CapabilityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoogleCloudMlV1CapabilityTypeEnum {
    
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    
    /// "TRAINING"
    #[serde(rename="TRAINING")]
    TRAINING,
    
    /// "BATCH_PREDICTION"
    #[serde(rename="BATCH_PREDICTION")]
    BATCHPREDICTION,
    
    /// "ONLINE_PREDICTION"
    #[serde(rename="ONLINE_PREDICTION")]
    ONLINEPREDICTION,
}

impl AsRef<str> for GoogleCloudMlV1CapabilityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1CapabilityTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudMlV1CapabilityTypeEnum::TRAINING => "TRAINING",
            GoogleCloudMlV1CapabilityTypeEnum::BATCHPREDICTION => "BATCH_PREDICTION",
            GoogleCloudMlV1CapabilityTypeEnum::ONLINEPREDICTION => "ONLINE_PREDICTION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1CapabilityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1CapabilityTypeEnum::TYPEUNSPECIFIED),
           "TRAINING" => Ok(GoogleCloudMlV1CapabilityTypeEnum::TRAINING),
           "BATCH_PREDICTION" => Ok(GoogleCloudMlV1CapabilityTypeEnum::BATCHPREDICTION),
           "ONLINE_PREDICTION" => Ok(GoogleCloudMlV1CapabilityTypeEnum::ONLINEPREDICTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1CapabilityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1HyperparameterOutputStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The detailed state of the trial.
pub enum GoogleCloudMlV1HyperparameterOutputStateEnum {
    

    /// The job state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job has been just created and processing has not yet begun.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
    

    /// The service is preparing to run the job.
    ///
    /// "PREPARING"
    #[serde(rename="PREPARING")]
    PREPARING,
    

    /// The job is in progress.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The job completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The job failed. `error_message` should contain the details of the failure.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The job is being cancelled. `error_message` should describe the reason for the cancellation.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The job has been cancelled. `error_message` should describe the reason for the cancellation.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for GoogleCloudMlV1HyperparameterOutputStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1HyperparameterOutputStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudMlV1HyperparameterOutputStateEnum::QUEUED => "QUEUED",
            GoogleCloudMlV1HyperparameterOutputStateEnum::PREPARING => "PREPARING",
            GoogleCloudMlV1HyperparameterOutputStateEnum::RUNNING => "RUNNING",
            GoogleCloudMlV1HyperparameterOutputStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudMlV1HyperparameterOutputStateEnum::FAILED => "FAILED",
            GoogleCloudMlV1HyperparameterOutputStateEnum::CANCELLING => "CANCELLING",
            GoogleCloudMlV1HyperparameterOutputStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1HyperparameterOutputStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::STATEUNSPECIFIED),
           "QUEUED" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::QUEUED),
           "PREPARING" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::PREPARING),
           "RUNNING" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::RUNNING),
           "SUCCEEDED" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::FAILED),
           "CANCELLING" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::CANCELLING),
           "CANCELLED" => Ok(GoogleCloudMlV1HyperparameterOutputStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1HyperparameterOutputStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1HyperparameterSpecAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The search algorithm specified for the hyperparameter tuning job. Uses the default AI Platform hyperparameter tuning algorithm if unspecified.
pub enum GoogleCloudMlV1HyperparameterSpecAlgorithmEnum {
    

    /// The default algorithm used by the hyperparameter tuning service. This is a Bayesian optimization algorithm.
    ///
    /// "ALGORITHM_UNSPECIFIED"
    #[serde(rename="ALGORITHM_UNSPECIFIED")]
    ALGORITHMUNSPECIFIED,
    

    /// Simple grid search within the feasible space. To use grid search, all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`.
    ///
    /// "GRID_SEARCH"
    #[serde(rename="GRID_SEARCH")]
    GRIDSEARCH,
    

    /// Simple random search within the feasible space.
    ///
    /// "RANDOM_SEARCH"
    #[serde(rename="RANDOM_SEARCH")]
    RANDOMSEARCH,
}

impl AsRef<str> for GoogleCloudMlV1HyperparameterSpecAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1HyperparameterSpecAlgorithmEnum::ALGORITHMUNSPECIFIED => "ALGORITHM_UNSPECIFIED",
            GoogleCloudMlV1HyperparameterSpecAlgorithmEnum::GRIDSEARCH => "GRID_SEARCH",
            GoogleCloudMlV1HyperparameterSpecAlgorithmEnum::RANDOMSEARCH => "RANDOM_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1HyperparameterSpecAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALGORITHM_UNSPECIFIED" => Ok(GoogleCloudMlV1HyperparameterSpecAlgorithmEnum::ALGORITHMUNSPECIFIED),
           "GRID_SEARCH" => Ok(GoogleCloudMlV1HyperparameterSpecAlgorithmEnum::GRIDSEARCH),
           "RANDOM_SEARCH" => Ok(GoogleCloudMlV1HyperparameterSpecAlgorithmEnum::RANDOMSEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1HyperparameterSpecAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1HyperparameterSpecGoalEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of goal to use for tuning. Available types are `MAXIMIZE` and `MINIMIZE`. Defaults to `MAXIMIZE`.
pub enum GoogleCloudMlV1HyperparameterSpecGoalEnum {
    

    /// Goal Type will default to maximize.
    ///
    /// "GOAL_TYPE_UNSPECIFIED"
    #[serde(rename="GOAL_TYPE_UNSPECIFIED")]
    GOALTYPEUNSPECIFIED,
    

    /// Maximize the goal metric.
    ///
    /// "MAXIMIZE"
    #[serde(rename="MAXIMIZE")]
    MAXIMIZE,
    

    /// Minimize the goal metric.
    ///
    /// "MINIMIZE"
    #[serde(rename="MINIMIZE")]
    MINIMIZE,
}

impl AsRef<str> for GoogleCloudMlV1HyperparameterSpecGoalEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1HyperparameterSpecGoalEnum::GOALTYPEUNSPECIFIED => "GOAL_TYPE_UNSPECIFIED",
            GoogleCloudMlV1HyperparameterSpecGoalEnum::MAXIMIZE => "MAXIMIZE",
            GoogleCloudMlV1HyperparameterSpecGoalEnum::MINIMIZE => "MINIMIZE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1HyperparameterSpecGoalEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GOAL_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1HyperparameterSpecGoalEnum::GOALTYPEUNSPECIFIED),
           "MAXIMIZE" => Ok(GoogleCloudMlV1HyperparameterSpecGoalEnum::MAXIMIZE),
           "MINIMIZE" => Ok(GoogleCloudMlV1HyperparameterSpecGoalEnum::MINIMIZE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1HyperparameterSpecGoalEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1JobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The detailed state of a job.
pub enum GoogleCloudMlV1JobStateEnum {
    

    /// The job state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job has been just created and processing has not yet begun.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
    

    /// The service is preparing to run the job.
    ///
    /// "PREPARING"
    #[serde(rename="PREPARING")]
    PREPARING,
    

    /// The job is in progress.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The job completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The job failed. `error_message` should contain the details of the failure.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The job is being cancelled. `error_message` should describe the reason for the cancellation.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The job has been cancelled. `error_message` should describe the reason for the cancellation.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
}

impl AsRef<str> for GoogleCloudMlV1JobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1JobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudMlV1JobStateEnum::QUEUED => "QUEUED",
            GoogleCloudMlV1JobStateEnum::PREPARING => "PREPARING",
            GoogleCloudMlV1JobStateEnum::RUNNING => "RUNNING",
            GoogleCloudMlV1JobStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudMlV1JobStateEnum::FAILED => "FAILED",
            GoogleCloudMlV1JobStateEnum::CANCELLING => "CANCELLING",
            GoogleCloudMlV1JobStateEnum::CANCELLED => "CANCELLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1JobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudMlV1JobStateEnum::STATEUNSPECIFIED),
           "QUEUED" => Ok(GoogleCloudMlV1JobStateEnum::QUEUED),
           "PREPARING" => Ok(GoogleCloudMlV1JobStateEnum::PREPARING),
           "RUNNING" => Ok(GoogleCloudMlV1JobStateEnum::RUNNING),
           "SUCCEEDED" => Ok(GoogleCloudMlV1JobStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudMlV1JobStateEnum::FAILED),
           "CANCELLING" => Ok(GoogleCloudMlV1JobStateEnum::CANCELLING),
           "CANCELLED" => Ok(GoogleCloudMlV1JobStateEnum::CANCELLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1JobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1MetricSpecNameEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// metric name.
pub enum GoogleCloudMlV1MetricSpecNameEnum {
    

    /// Unspecified MetricName.
    ///
    /// "METRIC_NAME_UNSPECIFIED"
    #[serde(rename="METRIC_NAME_UNSPECIFIED")]
    METRICNAMEUNSPECIFIED,
    

    /// CPU usage.
    ///
    /// "CPU_USAGE"
    #[serde(rename="CPU_USAGE")]
    CPUUSAGE,
    

    /// GPU duty cycle.
    ///
    /// "GPU_DUTY_CYCLE"
    #[serde(rename="GPU_DUTY_CYCLE")]
    GPUDUTYCYCLE,
}

impl AsRef<str> for GoogleCloudMlV1MetricSpecNameEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1MetricSpecNameEnum::METRICNAMEUNSPECIFIED => "METRIC_NAME_UNSPECIFIED",
            GoogleCloudMlV1MetricSpecNameEnum::CPUUSAGE => "CPU_USAGE",
            GoogleCloudMlV1MetricSpecNameEnum::GPUDUTYCYCLE => "GPU_DUTY_CYCLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1MetricSpecNameEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_NAME_UNSPECIFIED" => Ok(GoogleCloudMlV1MetricSpecNameEnum::METRICNAMEUNSPECIFIED),
           "CPU_USAGE" => Ok(GoogleCloudMlV1MetricSpecNameEnum::CPUUSAGE),
           "GPU_DUTY_CYCLE" => Ok(GoogleCloudMlV1MetricSpecNameEnum::GPUDUTYCYCLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1MetricSpecNameEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1ParameterSpecScaleTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. How the parameter should be scaled to the hypercube. Leave unset for categorical parameters. Some kind of scaling is strongly recommended for real or integral parameters (e.g., `UNIT_LINEAR_SCALE`).
pub enum GoogleCloudMlV1ParameterSpecScaleTypeEnum {
    

    /// By default, no scaling is applied.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Scales the feasible space to (0, 1) linearly.
    ///
    /// "UNIT_LINEAR_SCALE"
    #[serde(rename="UNIT_LINEAR_SCALE")]
    UNITLINEARSCALE,
    

    /// Scales the feasible space logarithmically to (0, 1). The entire feasible space must be strictly positive.
    ///
    /// "UNIT_LOG_SCALE"
    #[serde(rename="UNIT_LOG_SCALE")]
    UNITLOGSCALE,
    

    /// Scales the feasible space "reverse" logarithmically to (0, 1). The result is that values close to the top of the feasible space are spread out more than points near the bottom. The entire feasible space must be strictly positive.
    ///
    /// "UNIT_REVERSE_LOG_SCALE"
    #[serde(rename="UNIT_REVERSE_LOG_SCALE")]
    UNITREVERSELOGSCALE,
}

impl AsRef<str> for GoogleCloudMlV1ParameterSpecScaleTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1ParameterSpecScaleTypeEnum::NONE => "NONE",
            GoogleCloudMlV1ParameterSpecScaleTypeEnum::UNITLINEARSCALE => "UNIT_LINEAR_SCALE",
            GoogleCloudMlV1ParameterSpecScaleTypeEnum::UNITLOGSCALE => "UNIT_LOG_SCALE",
            GoogleCloudMlV1ParameterSpecScaleTypeEnum::UNITREVERSELOGSCALE => "UNIT_REVERSE_LOG_SCALE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1ParameterSpecScaleTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NONE" => Ok(GoogleCloudMlV1ParameterSpecScaleTypeEnum::NONE),
           "UNIT_LINEAR_SCALE" => Ok(GoogleCloudMlV1ParameterSpecScaleTypeEnum::UNITLINEARSCALE),
           "UNIT_LOG_SCALE" => Ok(GoogleCloudMlV1ParameterSpecScaleTypeEnum::UNITLOGSCALE),
           "UNIT_REVERSE_LOG_SCALE" => Ok(GoogleCloudMlV1ParameterSpecScaleTypeEnum::UNITREVERSELOGSCALE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1ParameterSpecScaleTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1ParameterSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of the parameter.
pub enum GoogleCloudMlV1ParameterSpecTypeEnum {
    

    /// You must specify a valid type. Using this unspecified type will result in an error.
    ///
    /// "PARAMETER_TYPE_UNSPECIFIED"
    #[serde(rename="PARAMETER_TYPE_UNSPECIFIED")]
    PARAMETERTYPEUNSPECIFIED,
    

    /// Type for real-valued parameters.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// Type for integral parameters.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// The parameter is categorical, with a value chosen from the categories field.
    ///
    /// "CATEGORICAL"
    #[serde(rename="CATEGORICAL")]
    CATEGORICAL,
    

    /// The parameter is real valued, with a fixed set of feasible points. If `type==DISCRETE`, feasible_points must be provided, and {`min_value`, `max_value`} will be ignored.
    ///
    /// "DISCRETE"
    #[serde(rename="DISCRETE")]
    DISCRETE,
}

impl AsRef<str> for GoogleCloudMlV1ParameterSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1ParameterSpecTypeEnum::PARAMETERTYPEUNSPECIFIED => "PARAMETER_TYPE_UNSPECIFIED",
            GoogleCloudMlV1ParameterSpecTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudMlV1ParameterSpecTypeEnum::INTEGER => "INTEGER",
            GoogleCloudMlV1ParameterSpecTypeEnum::CATEGORICAL => "CATEGORICAL",
            GoogleCloudMlV1ParameterSpecTypeEnum::DISCRETE => "DISCRETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1ParameterSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARAMETER_TYPE_UNSPECIFIED" => Ok(GoogleCloudMlV1ParameterSpecTypeEnum::PARAMETERTYPEUNSPECIFIED),
           "DOUBLE" => Ok(GoogleCloudMlV1ParameterSpecTypeEnum::DOUBLE),
           "INTEGER" => Ok(GoogleCloudMlV1ParameterSpecTypeEnum::INTEGER),
           "CATEGORICAL" => Ok(GoogleCloudMlV1ParameterSpecTypeEnum::CATEGORICAL),
           "DISCRETE" => Ok(GoogleCloudMlV1ParameterSpecTypeEnum::DISCRETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1ParameterSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1PredictionInputDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The format of the input data files.
pub enum GoogleCloudMlV1PredictionInputDataFormatEnum {
    

    /// Unspecified format.
    ///
    /// "DATA_FORMAT_UNSPECIFIED"
    #[serde(rename="DATA_FORMAT_UNSPECIFIED")]
    DATAFORMATUNSPECIFIED,
    

    /// Each line of the file is a JSON dictionary representing one record.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Deprecated. Use JSON instead.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// The source file is a TFRecord file. Currently available only for input data.
    ///
    /// "TF_RECORD"
    #[serde(rename="TF_RECORD")]
    TFRECORD,
    

    /// The source file is a GZIP-compressed TFRecord file. Currently available only for input data.
    ///
    /// "TF_RECORD_GZIP"
    #[serde(rename="TF_RECORD_GZIP")]
    TFRECORDGZIP,
    

    /// Values are comma-separated rows, with keys in a separate file. Currently available only for output data.
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
}

impl AsRef<str> for GoogleCloudMlV1PredictionInputDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1PredictionInputDataFormatEnum::DATAFORMATUNSPECIFIED => "DATA_FORMAT_UNSPECIFIED",
            GoogleCloudMlV1PredictionInputDataFormatEnum::JSON => "JSON",
            GoogleCloudMlV1PredictionInputDataFormatEnum::TEXT => "TEXT",
            GoogleCloudMlV1PredictionInputDataFormatEnum::TFRECORD => "TF_RECORD",
            GoogleCloudMlV1PredictionInputDataFormatEnum::TFRECORDGZIP => "TF_RECORD_GZIP",
            GoogleCloudMlV1PredictionInputDataFormatEnum::CSV => "CSV",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1PredictionInputDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_FORMAT_UNSPECIFIED" => Ok(GoogleCloudMlV1PredictionInputDataFormatEnum::DATAFORMATUNSPECIFIED),
           "JSON" => Ok(GoogleCloudMlV1PredictionInputDataFormatEnum::JSON),
           "TEXT" => Ok(GoogleCloudMlV1PredictionInputDataFormatEnum::TEXT),
           "TF_RECORD" => Ok(GoogleCloudMlV1PredictionInputDataFormatEnum::TFRECORD),
           "TF_RECORD_GZIP" => Ok(GoogleCloudMlV1PredictionInputDataFormatEnum::TFRECORDGZIP),
           "CSV" => Ok(GoogleCloudMlV1PredictionInputDataFormatEnum::CSV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1PredictionInputDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1PredictionInputOutputDataFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Format of the output data files, defaults to JSON.
pub enum GoogleCloudMlV1PredictionInputOutputDataFormatEnum {
    

    /// Unspecified format.
    ///
    /// "DATA_FORMAT_UNSPECIFIED"
    #[serde(rename="DATA_FORMAT_UNSPECIFIED")]
    DATAFORMATUNSPECIFIED,
    

    /// Each line of the file is a JSON dictionary representing one record.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Deprecated. Use JSON instead.
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// The source file is a TFRecord file. Currently available only for input data.
    ///
    /// "TF_RECORD"
    #[serde(rename="TF_RECORD")]
    TFRECORD,
    

    /// The source file is a GZIP-compressed TFRecord file. Currently available only for input data.
    ///
    /// "TF_RECORD_GZIP"
    #[serde(rename="TF_RECORD_GZIP")]
    TFRECORDGZIP,
    

    /// Values are comma-separated rows, with keys in a separate file. Currently available only for output data.
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
}

impl AsRef<str> for GoogleCloudMlV1PredictionInputOutputDataFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1PredictionInputOutputDataFormatEnum::DATAFORMATUNSPECIFIED => "DATA_FORMAT_UNSPECIFIED",
            GoogleCloudMlV1PredictionInputOutputDataFormatEnum::JSON => "JSON",
            GoogleCloudMlV1PredictionInputOutputDataFormatEnum::TEXT => "TEXT",
            GoogleCloudMlV1PredictionInputOutputDataFormatEnum::TFRECORD => "TF_RECORD",
            GoogleCloudMlV1PredictionInputOutputDataFormatEnum::TFRECORDGZIP => "TF_RECORD_GZIP",
            GoogleCloudMlV1PredictionInputOutputDataFormatEnum::CSV => "CSV",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1PredictionInputOutputDataFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_FORMAT_UNSPECIFIED" => Ok(GoogleCloudMlV1PredictionInputOutputDataFormatEnum::DATAFORMATUNSPECIFIED),
           "JSON" => Ok(GoogleCloudMlV1PredictionInputOutputDataFormatEnum::JSON),
           "TEXT" => Ok(GoogleCloudMlV1PredictionInputOutputDataFormatEnum::TEXT),
           "TF_RECORD" => Ok(GoogleCloudMlV1PredictionInputOutputDataFormatEnum::TFRECORD),
           "TF_RECORD_GZIP" => Ok(GoogleCloudMlV1PredictionInputOutputDataFormatEnum::TFRECORDGZIP),
           "CSV" => Ok(GoogleCloudMlV1PredictionInputOutputDataFormatEnum::CSV),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1PredictionInputOutputDataFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1StudyStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The detailed state of a study.
pub enum GoogleCloudMlV1StudyStateEnum {
    

    /// The study state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The study is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The study is stopped due to an internal error.
    ///
    /// "INACTIVE"
    #[serde(rename="INACTIVE")]
    INACTIVE,
    

    /// The study is done when the service exhausts the parameter search space or max_trial_count is reached.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
}

impl AsRef<str> for GoogleCloudMlV1StudyStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1StudyStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudMlV1StudyStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudMlV1StudyStateEnum::INACTIVE => "INACTIVE",
            GoogleCloudMlV1StudyStateEnum::COMPLETED => "COMPLETED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1StudyStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudMlV1StudyStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudMlV1StudyStateEnum::ACTIVE),
           "INACTIVE" => Ok(GoogleCloudMlV1StudyStateEnum::INACTIVE),
           "COMPLETED" => Ok(GoogleCloudMlV1StudyStateEnum::COMPLETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1StudyStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1StudyConfigAlgorithmEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The search algorithm specified for the study.
pub enum GoogleCloudMlV1StudyConfigAlgorithmEnum {
    

    /// The default algorithm used by the Cloud AI Platform Vizier service.
    ///
    /// "ALGORITHM_UNSPECIFIED"
    #[serde(rename="ALGORITHM_UNSPECIFIED")]
    ALGORITHMUNSPECIFIED,
    

    /// Gaussian Process Bandit.
    ///
    /// "GAUSSIAN_PROCESS_BANDIT"
    #[serde(rename="GAUSSIAN_PROCESS_BANDIT")]
    GAUSSIANPROCESSBANDIT,
    

    /// Simple grid search within the feasible space. To use grid search, all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`.
    ///
    /// "GRID_SEARCH"
    #[serde(rename="GRID_SEARCH")]
    GRIDSEARCH,
    

    /// Simple random search within the feasible space.
    ///
    /// "RANDOM_SEARCH"
    #[serde(rename="RANDOM_SEARCH")]
    RANDOMSEARCH,
}

impl AsRef<str> for GoogleCloudMlV1StudyConfigAlgorithmEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1StudyConfigAlgorithmEnum::ALGORITHMUNSPECIFIED => "ALGORITHM_UNSPECIFIED",
            GoogleCloudMlV1StudyConfigAlgorithmEnum::GAUSSIANPROCESSBANDIT => "GAUSSIAN_PROCESS_BANDIT",
            GoogleCloudMlV1StudyConfigAlgorithmEnum::GRIDSEARCH => "GRID_SEARCH",
            GoogleCloudMlV1StudyConfigAlgorithmEnum::RANDOMSEARCH => "RANDOM_SEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1StudyConfigAlgorithmEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALGORITHM_UNSPECIFIED" => Ok(GoogleCloudMlV1StudyConfigAlgorithmEnum::ALGORITHMUNSPECIFIED),
           "GAUSSIAN_PROCESS_BANDIT" => Ok(GoogleCloudMlV1StudyConfigAlgorithmEnum::GAUSSIANPROCESSBANDIT),
           "GRID_SEARCH" => Ok(GoogleCloudMlV1StudyConfigAlgorithmEnum::GRIDSEARCH),
           "RANDOM_SEARCH" => Ok(GoogleCloudMlV1StudyConfigAlgorithmEnum::RANDOMSEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1StudyConfigAlgorithmEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1TrainingInputScaleTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Specifies the machine types, the number of replicas for workers and parameter servers.
pub enum GoogleCloudMlV1TrainingInputScaleTierEnum {
    

    /// A single worker instance. This tier is suitable for learning how to use Cloud ML, and for experimenting with new models using small datasets.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Many workers and a few parameter servers.
    ///
    /// "STANDARD_1"
    #[serde(rename="STANDARD_1")]
    STANDARD1,
    

    /// A large number of workers with many parameter servers.
    ///
    /// "PREMIUM_1"
    #[serde(rename="PREMIUM_1")]
    PREMIUM1,
    

    /// A single worker instance [with a GPU](/ai-platform/training/docs/using-gpus).
    ///
    /// "BASIC_GPU"
    #[serde(rename="BASIC_GPU")]
    BASICGPU,
    

    /// A single worker instance with a [Cloud TPU](/ml-engine/docs/tensorflow/using-tpus).
    ///
    /// "BASIC_TPU"
    #[serde(rename="BASIC_TPU")]
    BASICTPU,
    

    /// The CUSTOM tier is not a set tier, but rather enables you to use your own cluster specification. When you use this tier, set values to configure your processing cluster according to these guidelines: * You _must_ set `TrainingInput.masterType` to specify the type of machine to use for your master node. This is the only required setting. * You _may_ set `TrainingInput.workerCount` to specify the number of workers to use. If you specify one or more workers, you _must_ also set `TrainingInput.workerType` to specify the type of machine to use for your worker nodes. * You _may_ set `TrainingInput.parameterServerCount` to specify the number of parameter servers to use. If you specify one or more parameter servers, you _must_ also set `TrainingInput.parameterServerType` to specify the type of machine to use for your parameter servers. Note that all of your workers must use the same machine type, which can be different from your parameter server type and master type. Your parameter servers must likewise use the same machine type, which can be different from your worker type and master type.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for GoogleCloudMlV1TrainingInputScaleTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1TrainingInputScaleTierEnum::BASIC => "BASIC",
            GoogleCloudMlV1TrainingInputScaleTierEnum::STANDARD1 => "STANDARD_1",
            GoogleCloudMlV1TrainingInputScaleTierEnum::PREMIUM1 => "PREMIUM_1",
            GoogleCloudMlV1TrainingInputScaleTierEnum::BASICGPU => "BASIC_GPU",
            GoogleCloudMlV1TrainingInputScaleTierEnum::BASICTPU => "BASIC_TPU",
            GoogleCloudMlV1TrainingInputScaleTierEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1TrainingInputScaleTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(GoogleCloudMlV1TrainingInputScaleTierEnum::BASIC),
           "STANDARD_1" => Ok(GoogleCloudMlV1TrainingInputScaleTierEnum::STANDARD1),
           "PREMIUM_1" => Ok(GoogleCloudMlV1TrainingInputScaleTierEnum::PREMIUM1),
           "BASIC_GPU" => Ok(GoogleCloudMlV1TrainingInputScaleTierEnum::BASICGPU),
           "BASIC_TPU" => Ok(GoogleCloudMlV1TrainingInputScaleTierEnum::BASICTPU),
           "CUSTOM" => Ok(GoogleCloudMlV1TrainingInputScaleTierEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1TrainingInputScaleTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1TrialStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The detailed state of a trial.
pub enum GoogleCloudMlV1TrialStateEnum {
    

    /// The trial state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Indicates that a specific trial has been requested, but it has not yet been suggested by the service.
    ///
    /// "REQUESTED"
    #[serde(rename="REQUESTED")]
    REQUESTED,
    

    /// Indicates that the trial has been suggested.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Indicates that the trial is done, and either has a final_measurement set, or is marked as trial_infeasible.
    ///
    /// "COMPLETED"
    #[serde(rename="COMPLETED")]
    COMPLETED,
    

    /// Indicates that the trial should stop according to the service.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
}

impl AsRef<str> for GoogleCloudMlV1TrialStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1TrialStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudMlV1TrialStateEnum::REQUESTED => "REQUESTED",
            GoogleCloudMlV1TrialStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudMlV1TrialStateEnum::COMPLETED => "COMPLETED",
            GoogleCloudMlV1TrialStateEnum::STOPPING => "STOPPING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1TrialStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudMlV1TrialStateEnum::STATEUNSPECIFIED),
           "REQUESTED" => Ok(GoogleCloudMlV1TrialStateEnum::REQUESTED),
           "ACTIVE" => Ok(GoogleCloudMlV1TrialStateEnum::ACTIVE),
           "COMPLETED" => Ok(GoogleCloudMlV1TrialStateEnum::COMPLETED),
           "STOPPING" => Ok(GoogleCloudMlV1TrialStateEnum::STOPPING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1TrialStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1VersionFrameworkEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The machine learning framework AI Platform uses to train this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`, `XGBOOST`. If you do not specify a framework, AI Platform will analyze files in the deployment_uri to determine a framework. If you choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version of the model to 1.4 or greater. Do **not** specify a framework if you're deploying a [custom prediction routine](/ai-platform/prediction/docs/custom-prediction-routines) or if you're using a [custom container](/ai-platform/prediction/docs/use-custom-container).
pub enum GoogleCloudMlV1VersionFrameworkEnum {
    

    /// Unspecified framework. Assigns a value based on the file suffix.
    ///
    /// "FRAMEWORK_UNSPECIFIED"
    #[serde(rename="FRAMEWORK_UNSPECIFIED")]
    FRAMEWORKUNSPECIFIED,
    

    /// Tensorflow framework.
    ///
    /// "TENSORFLOW"
    #[serde(rename="TENSORFLOW")]
    TENSORFLOW,
    

    /// Scikit-learn framework.
    ///
    /// "SCIKIT_LEARN"
    #[serde(rename="SCIKIT_LEARN")]
    SCIKITLEARN,
    

    /// XGBoost framework.
    ///
    /// "XGBOOST"
    #[serde(rename="XGBOOST")]
    XGBOOST,
}

impl AsRef<str> for GoogleCloudMlV1VersionFrameworkEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1VersionFrameworkEnum::FRAMEWORKUNSPECIFIED => "FRAMEWORK_UNSPECIFIED",
            GoogleCloudMlV1VersionFrameworkEnum::TENSORFLOW => "TENSORFLOW",
            GoogleCloudMlV1VersionFrameworkEnum::SCIKITLEARN => "SCIKIT_LEARN",
            GoogleCloudMlV1VersionFrameworkEnum::XGBOOST => "XGBOOST",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1VersionFrameworkEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FRAMEWORK_UNSPECIFIED" => Ok(GoogleCloudMlV1VersionFrameworkEnum::FRAMEWORKUNSPECIFIED),
           "TENSORFLOW" => Ok(GoogleCloudMlV1VersionFrameworkEnum::TENSORFLOW),
           "SCIKIT_LEARN" => Ok(GoogleCloudMlV1VersionFrameworkEnum::SCIKITLEARN),
           "XGBOOST" => Ok(GoogleCloudMlV1VersionFrameworkEnum::XGBOOST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1VersionFrameworkEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudMlV1VersionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of a version.
pub enum GoogleCloudMlV1VersionStateEnum {
    

    /// The version state is unspecified.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The version is ready for prediction.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The version is being created. New UpdateVersion and DeleteVersion requests will fail if a version is in the CREATING state.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The version failed to be created, possibly cancelled. `error_message` should contain the details of the failure.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The version is being deleted. New UpdateVersion and DeleteVersion requests will fail if a version is in the DELETING state.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The version is being updated. New UpdateVersion and DeleteVersion requests will fail if a version is in the UPDATING state.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
}

impl AsRef<str> for GoogleCloudMlV1VersionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudMlV1VersionStateEnum::UNKNOWN => "UNKNOWN",
            GoogleCloudMlV1VersionStateEnum::READY => "READY",
            GoogleCloudMlV1VersionStateEnum::CREATING => "CREATING",
            GoogleCloudMlV1VersionStateEnum::FAILED => "FAILED",
            GoogleCloudMlV1VersionStateEnum::DELETING => "DELETING",
            GoogleCloudMlV1VersionStateEnum::UPDATING => "UPDATING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudMlV1VersionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(GoogleCloudMlV1VersionStateEnum::UNKNOWN),
           "READY" => Ok(GoogleCloudMlV1VersionStateEnum::READY),
           "CREATING" => Ok(GoogleCloudMlV1VersionStateEnum::CREATING),
           "FAILED" => Ok(GoogleCloudMlV1VersionStateEnum::FAILED),
           "DELETING" => Ok(GoogleCloudMlV1VersionStateEnum::DELETING),
           "UPDATING" => Ok(GoogleCloudMlV1VersionStateEnum::UPDATING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudMlV1VersionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIamV1AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
    

    /// Default case. Should never be this.
    ///
    /// "LOG_TYPE_UNSPECIFIED"
    #[serde(rename="LOG_TYPE_UNSPECIFIED")]
    LOGTYPEUNSPECIFIED,
    

    /// Admin reads. Example: CloudIAM getIamPolicy
    ///
    /// "ADMIN_READ"
    #[serde(rename="ADMIN_READ")]
    ADMINREAD,
    

    /// Data writes. Example: CloudSQL Users create
    ///
    /// "DATA_WRITE"
    #[serde(rename="DATA_WRITE")]
    DATAWRITE,
    

    /// Data reads. Example: CloudSQL Users list
    ///
    /// "DATA_READ"
    #[serde(rename="DATA_READ")]
    DATAREAD,
}

impl AsRef<str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIamV1AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(GoogleIamV1AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIamV1AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


