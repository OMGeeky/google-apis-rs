use super::*;



// region AcceleratorConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this accelerator.
pub enum AcceleratorConfigTypeEnum {
    

    /// Accelerator type is not specified.
    ///
    /// "ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="ACCELERATOR_TYPE_UNSPECIFIED")]
    ACCELERATORTYPEUNSPECIFIED,
    

    /// Accelerator type is Nvidia Tesla K80.
    ///
    /// "NVIDIA_TESLA_K80"
    #[serde(rename="NVIDIA_TESLA_K80")]
    NVIDIATESLAK80,
    

    /// Accelerator type is Nvidia Tesla P100.
    ///
    /// "NVIDIA_TESLA_P100"
    #[serde(rename="NVIDIA_TESLA_P100")]
    NVIDIATESLAP100,
    

    /// Accelerator type is Nvidia Tesla V100.
    ///
    /// "NVIDIA_TESLA_V100"
    #[serde(rename="NVIDIA_TESLA_V100")]
    NVIDIATESLAV100,
    

    /// Accelerator type is Nvidia Tesla P4.
    ///
    /// "NVIDIA_TESLA_P4"
    #[serde(rename="NVIDIA_TESLA_P4")]
    NVIDIATESLAP4,
    

    /// Accelerator type is Nvidia Tesla T4.
    ///
    /// "NVIDIA_TESLA_T4"
    #[serde(rename="NVIDIA_TESLA_T4")]
    NVIDIATESLAT4,
    

    /// Accelerator type is Nvidia Tesla A100.
    ///
    /// "NVIDIA_TESLA_A100"
    #[serde(rename="NVIDIA_TESLA_A100")]
    NVIDIATESLAA100,
    

    /// Accelerator type is Nvidia Tesla L4.
    ///
    /// "NVIDIA_L4"
    #[serde(rename="NVIDIA_L4")]
    NVIDIAL4,
    

    /// Accelerator type is Nvidia Tesla A100 80GB.
    ///
    /// "NVIDIA_A100_80GB"
    #[serde(rename="NVIDIA_A100_80GB")]
    NVIDIAA10080GB,
    

    /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_T4_VWS"
    #[serde(rename="NVIDIA_TESLA_T4_VWS")]
    NVIDIATESLAT4VWS,
    

    /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_P100_VWS"
    #[serde(rename="NVIDIA_TESLA_P100_VWS")]
    NVIDIATESLAP100VWS,
    

    /// Accelerator type is NVIDIA Tesla P4 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_P4_VWS"
    #[serde(rename="NVIDIA_TESLA_P4_VWS")]
    NVIDIATESLAP4VWS,
    

    /// (Coming soon) Accelerator type is TPU V2.
    ///
    /// "TPU_V2"
    #[serde(rename="TPU_V2")]
    TPUV2,
    

    /// (Coming soon) Accelerator type is TPU V3.
    ///
    /// "TPU_V3"
    #[serde(rename="TPU_V3")]
    TPUV3,
}

impl AsRef<str> for AcceleratorConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AcceleratorConfigTypeEnum::ACCELERATORTYPEUNSPECIFIED => "ACCELERATOR_TYPE_UNSPECIFIED",
            AcceleratorConfigTypeEnum::NVIDIATESLAK80 => "NVIDIA_TESLA_K80",
            AcceleratorConfigTypeEnum::NVIDIATESLAP100 => "NVIDIA_TESLA_P100",
            AcceleratorConfigTypeEnum::NVIDIATESLAV100 => "NVIDIA_TESLA_V100",
            AcceleratorConfigTypeEnum::NVIDIATESLAP4 => "NVIDIA_TESLA_P4",
            AcceleratorConfigTypeEnum::NVIDIATESLAT4 => "NVIDIA_TESLA_T4",
            AcceleratorConfigTypeEnum::NVIDIATESLAA100 => "NVIDIA_TESLA_A100",
            AcceleratorConfigTypeEnum::NVIDIAL4 => "NVIDIA_L4",
            AcceleratorConfigTypeEnum::NVIDIAA10080GB => "NVIDIA_A100_80GB",
            AcceleratorConfigTypeEnum::NVIDIATESLAT4VWS => "NVIDIA_TESLA_T4_VWS",
            AcceleratorConfigTypeEnum::NVIDIATESLAP100VWS => "NVIDIA_TESLA_P100_VWS",
            AcceleratorConfigTypeEnum::NVIDIATESLAP4VWS => "NVIDIA_TESLA_P4_VWS",
            AcceleratorConfigTypeEnum::TPUV2 => "TPU_V2",
            AcceleratorConfigTypeEnum::TPUV3 => "TPU_V3",
        }
    }
}

impl std::convert::TryFrom< &str> for AcceleratorConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCELERATOR_TYPE_UNSPECIFIED" => Ok(AcceleratorConfigTypeEnum::ACCELERATORTYPEUNSPECIFIED),
           "NVIDIA_TESLA_K80" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAK80),
           "NVIDIA_TESLA_P100" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAP100),
           "NVIDIA_TESLA_V100" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAV100),
           "NVIDIA_TESLA_P4" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAP4),
           "NVIDIA_TESLA_T4" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAT4),
           "NVIDIA_TESLA_A100" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAA100),
           "NVIDIA_L4" => Ok(AcceleratorConfigTypeEnum::NVIDIAL4),
           "NVIDIA_A100_80GB" => Ok(AcceleratorConfigTypeEnum::NVIDIAA10080GB),
           "NVIDIA_TESLA_T4_VWS" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAT4VWS),
           "NVIDIA_TESLA_P100_VWS" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAP100VWS),
           "NVIDIA_TESLA_P4_VWS" => Ok(AcceleratorConfigTypeEnum::NVIDIATESLAP4VWS),
           "TPU_V2" => Ok(AcceleratorConfigTypeEnum::TPUV2),
           "TPU_V3" => Ok(AcceleratorConfigTypeEnum::TPUV3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AcceleratorConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event type.
pub enum EventTypeEnum {
    

    /// Event is not specified.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// The instance / runtime is idle
    ///
    /// "IDLE"
    #[serde(rename="IDLE")]
    IDLE,
    

    /// The instance / runtime is available. This event indicates that instance / runtime underlying compute is operational.
    ///
    /// "HEARTBEAT"
    #[serde(rename="HEARTBEAT")]
    HEARTBEAT,
    

    /// The instance / runtime health is available. This event indicates that instance / runtime health information.
    ///
    /// "HEALTH"
    #[serde(rename="HEALTH")]
    HEALTH,
    

    /// The instance / runtime is available. This event allows instance / runtime to send Host maintenance information to Control Plane. https://cloud.google.com/compute/docs/gpus/gpu-host-maintenance
    ///
    /// "MAINTENANCE"
    #[serde(rename="MAINTENANCE")]
    MAINTENANCE,
}

impl AsRef<str> for EventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventTypeEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            EventTypeEnum::IDLE => "IDLE",
            EventTypeEnum::HEARTBEAT => "HEARTBEAT",
            EventTypeEnum::HEALTH => "HEALTH",
            EventTypeEnum::MAINTENANCE => "MAINTENANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for EventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(EventTypeEnum::EVENTTYPEUNSPECIFIED),
           "IDLE" => Ok(EventTypeEnum::IDLE),
           "HEARTBEAT" => Ok(EventTypeEnum::HEARTBEAT),
           "HEALTH" => Ok(EventTypeEnum::HEALTH),
           "MAINTENANCE" => Ok(EventTypeEnum::MAINTENANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExecutionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of the underlying AI Platform job.
pub enum ExecutionStateEnum {
    

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
    

    /// The service is preparing to execution the job.
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
    

    /// The job has become expired (relevant to Vertex AI jobs) https://cloud.google.com/vertex-ai/docs/reference/rest/v1/JobState
    ///
    /// "EXPIRED"
    #[serde(rename="EXPIRED")]
    EXPIRED,
    

    /// The Execution is being created.
    ///
    /// "INITIALIZING"
    #[serde(rename="INITIALIZING")]
    INITIALIZING,
}

impl AsRef<str> for ExecutionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecutionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ExecutionStateEnum::QUEUED => "QUEUED",
            ExecutionStateEnum::PREPARING => "PREPARING",
            ExecutionStateEnum::RUNNING => "RUNNING",
            ExecutionStateEnum::SUCCEEDED => "SUCCEEDED",
            ExecutionStateEnum::FAILED => "FAILED",
            ExecutionStateEnum::CANCELLING => "CANCELLING",
            ExecutionStateEnum::CANCELLED => "CANCELLED",
            ExecutionStateEnum::EXPIRED => "EXPIRED",
            ExecutionStateEnum::INITIALIZING => "INITIALIZING",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecutionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ExecutionStateEnum::STATEUNSPECIFIED),
           "QUEUED" => Ok(ExecutionStateEnum::QUEUED),
           "PREPARING" => Ok(ExecutionStateEnum::PREPARING),
           "RUNNING" => Ok(ExecutionStateEnum::RUNNING),
           "SUCCEEDED" => Ok(ExecutionStateEnum::SUCCEEDED),
           "FAILED" => Ok(ExecutionStateEnum::FAILED),
           "CANCELLING" => Ok(ExecutionStateEnum::CANCELLING),
           "CANCELLED" => Ok(ExecutionStateEnum::CANCELLED),
           "EXPIRED" => Ok(ExecutionStateEnum::EXPIRED),
           "INITIALIZING" => Ok(ExecutionStateEnum::INITIALIZING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecutionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExecutionTemplateJobTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of Job to be used on this execution.
pub enum ExecutionTemplateJobTypeEnum {
    

    /// No type specified.
    ///
    /// "JOB_TYPE_UNSPECIFIED"
    #[serde(rename="JOB_TYPE_UNSPECIFIED")]
    JOBTYPEUNSPECIFIED,
    

    /// Custom Job in `aiplatform.googleapis.com`. Default value for an execution.
    ///
    /// "VERTEX_AI"
    #[serde(rename="VERTEX_AI")]
    VERTEXAI,
    

    /// Run execution on a cluster with Dataproc as a job. https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs
    ///
    /// "DATAPROC"
    #[serde(rename="DATAPROC")]
    DATAPROC,
}

impl AsRef<str> for ExecutionTemplateJobTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecutionTemplateJobTypeEnum::JOBTYPEUNSPECIFIED => "JOB_TYPE_UNSPECIFIED",
            ExecutionTemplateJobTypeEnum::VERTEXAI => "VERTEX_AI",
            ExecutionTemplateJobTypeEnum::DATAPROC => "DATAPROC",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecutionTemplateJobTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "JOB_TYPE_UNSPECIFIED" => Ok(ExecutionTemplateJobTypeEnum::JOBTYPEUNSPECIFIED),
           "VERTEX_AI" => Ok(ExecutionTemplateJobTypeEnum::VERTEXAI),
           "DATAPROC" => Ok(ExecutionTemplateJobTypeEnum::DATAPROC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecutionTemplateJobTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExecutionTemplateScaleTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Scale tier of the hardware used for notebook execution. DEPRECATED Will be discontinued. As right now only CUSTOM is supported.
pub enum ExecutionTemplateScaleTierEnum {
    

    /// Unspecified Scale Tier.
    ///
    /// "SCALE_TIER_UNSPECIFIED"
    #[serde(rename="SCALE_TIER_UNSPECIFIED")]
    SCALETIERUNSPECIFIED,
    

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
    

    /// A single worker instance with a K80 GPU.
    ///
    /// "BASIC_GPU"
    #[serde(rename="BASIC_GPU")]
    BASICGPU,
    

    /// A single worker instance with a Cloud TPU.
    ///
    /// "BASIC_TPU"
    #[serde(rename="BASIC_TPU")]
    BASICTPU,
    

    /// The CUSTOM tier is not a set tier, but rather enables you to use your own cluster specification. When you use this tier, set values to configure your processing cluster according to these guidelines: * You _must_ set `ExecutionTemplate.masterType` to specify the type of machine to use for your master node. This is the only required setting.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
}

impl AsRef<str> for ExecutionTemplateScaleTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExecutionTemplateScaleTierEnum::SCALETIERUNSPECIFIED => "SCALE_TIER_UNSPECIFIED",
            ExecutionTemplateScaleTierEnum::BASIC => "BASIC",
            ExecutionTemplateScaleTierEnum::STANDARD1 => "STANDARD_1",
            ExecutionTemplateScaleTierEnum::PREMIUM1 => "PREMIUM_1",
            ExecutionTemplateScaleTierEnum::BASICGPU => "BASIC_GPU",
            ExecutionTemplateScaleTierEnum::BASICTPU => "BASIC_TPU",
            ExecutionTemplateScaleTierEnum::CUSTOM => "CUSTOM",
        }
    }
}

impl std::convert::TryFrom< &str> for ExecutionTemplateScaleTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCALE_TIER_UNSPECIFIED" => Ok(ExecutionTemplateScaleTierEnum::SCALETIERUNSPECIFIED),
           "BASIC" => Ok(ExecutionTemplateScaleTierEnum::BASIC),
           "STANDARD_1" => Ok(ExecutionTemplateScaleTierEnum::STANDARD1),
           "PREMIUM_1" => Ok(ExecutionTemplateScaleTierEnum::PREMIUM1),
           "BASIC_GPU" => Ok(ExecutionTemplateScaleTierEnum::BASICGPU),
           "BASIC_TPU" => Ok(ExecutionTemplateScaleTierEnum::BASICTPU),
           "CUSTOM" => Ok(ExecutionTemplateScaleTierEnum::CUSTOM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExecutionTemplateScaleTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GetInstanceHealthResponseHealthStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Runtime health_state.
pub enum GetInstanceHealthResponseHealthStateEnum {
    

    /// The instance substate is unknown.
    ///
    /// "HEALTH_STATE_UNSPECIFIED"
    #[serde(rename="HEALTH_STATE_UNSPECIFIED")]
    HEALTHSTATEUNSPECIFIED,
    

    /// The instance is known to be in an healthy state (for example, critical daemons are running) Applies to ACTIVE state.
    ///
    /// "HEALTHY"
    #[serde(rename="HEALTHY")]
    HEALTHY,
    

    /// The instance is known to be in an unhealthy state (for example, critical daemons are not running) Applies to ACTIVE state.
    ///
    /// "UNHEALTHY"
    #[serde(rename="UNHEALTHY")]
    UNHEALTHY,
    

    /// The instance has not installed health monitoring agent. Applies to ACTIVE state.
    ///
    /// "AGENT_NOT_INSTALLED"
    #[serde(rename="AGENT_NOT_INSTALLED")]
    AGENTNOTINSTALLED,
    

    /// The instance health monitoring agent is not running. Applies to ACTIVE state.
    ///
    /// "AGENT_NOT_RUNNING"
    #[serde(rename="AGENT_NOT_RUNNING")]
    AGENTNOTRUNNING,
}

impl AsRef<str> for GetInstanceHealthResponseHealthStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GetInstanceHealthResponseHealthStateEnum::HEALTHSTATEUNSPECIFIED => "HEALTH_STATE_UNSPECIFIED",
            GetInstanceHealthResponseHealthStateEnum::HEALTHY => "HEALTHY",
            GetInstanceHealthResponseHealthStateEnum::UNHEALTHY => "UNHEALTHY",
            GetInstanceHealthResponseHealthStateEnum::AGENTNOTINSTALLED => "AGENT_NOT_INSTALLED",
            GetInstanceHealthResponseHealthStateEnum::AGENTNOTRUNNING => "AGENT_NOT_RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for GetInstanceHealthResponseHealthStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEALTH_STATE_UNSPECIFIED" => Ok(GetInstanceHealthResponseHealthStateEnum::HEALTHSTATEUNSPECIFIED),
           "HEALTHY" => Ok(GetInstanceHealthResponseHealthStateEnum::HEALTHY),
           "UNHEALTHY" => Ok(GetInstanceHealthResponseHealthStateEnum::UNHEALTHY),
           "AGENT_NOT_INSTALLED" => Ok(GetInstanceHealthResponseHealthStateEnum::AGENTNOTINSTALLED),
           "AGENT_NOT_RUNNING" => Ok(GetInstanceHealthResponseHealthStateEnum::AGENTNOTRUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GetInstanceHealthResponseHealthStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceBootDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Input only. The type of the boot disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`).
pub enum InstanceBootDiskTypeEnum {
    

    /// Disk type not set.
    ///
    /// "DISK_TYPE_UNSPECIFIED"
    #[serde(rename="DISK_TYPE_UNSPECIFIED")]
    DISKTYPEUNSPECIFIED,
    

    /// Standard persistent disk type.
    ///
    /// "PD_STANDARD"
    #[serde(rename="PD_STANDARD")]
    PDSTANDARD,
    

    /// SSD persistent disk type.
    ///
    /// "PD_SSD"
    #[serde(rename="PD_SSD")]
    PDSSD,
    

    /// Balanced persistent disk type.
    ///
    /// "PD_BALANCED"
    #[serde(rename="PD_BALANCED")]
    PDBALANCED,
    

    /// Extreme persistent disk type.
    ///
    /// "PD_EXTREME"
    #[serde(rename="PD_EXTREME")]
    PDEXTREME,
}

impl AsRef<str> for InstanceBootDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceBootDiskTypeEnum::DISKTYPEUNSPECIFIED => "DISK_TYPE_UNSPECIFIED",
            InstanceBootDiskTypeEnum::PDSTANDARD => "PD_STANDARD",
            InstanceBootDiskTypeEnum::PDSSD => "PD_SSD",
            InstanceBootDiskTypeEnum::PDBALANCED => "PD_BALANCED",
            InstanceBootDiskTypeEnum::PDEXTREME => "PD_EXTREME",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceBootDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISK_TYPE_UNSPECIFIED" => Ok(InstanceBootDiskTypeEnum::DISKTYPEUNSPECIFIED),
           "PD_STANDARD" => Ok(InstanceBootDiskTypeEnum::PDSTANDARD),
           "PD_SSD" => Ok(InstanceBootDiskTypeEnum::PDSSD),
           "PD_BALANCED" => Ok(InstanceBootDiskTypeEnum::PDBALANCED),
           "PD_EXTREME" => Ok(InstanceBootDiskTypeEnum::PDEXTREME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceBootDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceDataDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Input only. The type of the data disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`).
pub enum InstanceDataDiskTypeEnum {
    

    /// Disk type not set.
    ///
    /// "DISK_TYPE_UNSPECIFIED"
    #[serde(rename="DISK_TYPE_UNSPECIFIED")]
    DISKTYPEUNSPECIFIED,
    

    /// Standard persistent disk type.
    ///
    /// "PD_STANDARD"
    #[serde(rename="PD_STANDARD")]
    PDSTANDARD,
    

    /// SSD persistent disk type.
    ///
    /// "PD_SSD"
    #[serde(rename="PD_SSD")]
    PDSSD,
    

    /// Balanced persistent disk type.
    ///
    /// "PD_BALANCED"
    #[serde(rename="PD_BALANCED")]
    PDBALANCED,
    

    /// Extreme persistent disk type.
    ///
    /// "PD_EXTREME"
    #[serde(rename="PD_EXTREME")]
    PDEXTREME,
}

impl AsRef<str> for InstanceDataDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceDataDiskTypeEnum::DISKTYPEUNSPECIFIED => "DISK_TYPE_UNSPECIFIED",
            InstanceDataDiskTypeEnum::PDSTANDARD => "PD_STANDARD",
            InstanceDataDiskTypeEnum::PDSSD => "PD_SSD",
            InstanceDataDiskTypeEnum::PDBALANCED => "PD_BALANCED",
            InstanceDataDiskTypeEnum::PDEXTREME => "PD_EXTREME",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceDataDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISK_TYPE_UNSPECIFIED" => Ok(InstanceDataDiskTypeEnum::DISKTYPEUNSPECIFIED),
           "PD_STANDARD" => Ok(InstanceDataDiskTypeEnum::PDSTANDARD),
           "PD_SSD" => Ok(InstanceDataDiskTypeEnum::PDSSD),
           "PD_BALANCED" => Ok(InstanceDataDiskTypeEnum::PDBALANCED),
           "PD_EXTREME" => Ok(InstanceDataDiskTypeEnum::PDEXTREME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceDataDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceDiskEncryptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Input only. Disk encryption method used on the boot and data disks, defaults to GMEK.
pub enum InstanceDiskEncryptionEnum {
    

    /// Disk encryption is not specified.
    ///
    /// "DISK_ENCRYPTION_UNSPECIFIED"
    #[serde(rename="DISK_ENCRYPTION_UNSPECIFIED")]
    DISKENCRYPTIONUNSPECIFIED,
    

    /// Use Google managed encryption keys to encrypt the boot disk.
    ///
    /// "GMEK"
    #[serde(rename="GMEK")]
    GMEK,
    

    /// Use customer managed encryption keys to encrypt the boot disk.
    ///
    /// "CMEK"
    #[serde(rename="CMEK")]
    CMEK,
}

impl AsRef<str> for InstanceDiskEncryptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceDiskEncryptionEnum::DISKENCRYPTIONUNSPECIFIED => "DISK_ENCRYPTION_UNSPECIFIED",
            InstanceDiskEncryptionEnum::GMEK => "GMEK",
            InstanceDiskEncryptionEnum::CMEK => "CMEK",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceDiskEncryptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISK_ENCRYPTION_UNSPECIFIED" => Ok(InstanceDiskEncryptionEnum::DISKENCRYPTIONUNSPECIFIED),
           "GMEK" => Ok(InstanceDiskEncryptionEnum::GMEK),
           "CMEK" => Ok(InstanceDiskEncryptionEnum::CMEK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceDiskEncryptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceNicTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of vNIC to be used on this interface. This may be gVNIC or VirtioNet.
pub enum InstanceNicTypeEnum {
    

    /// No type specified.
    ///
    /// "UNSPECIFIED_NIC_TYPE"
    #[serde(rename="UNSPECIFIED_NIC_TYPE")]
    UNSPECIFIEDNICTYPE,
    

    /// VIRTIO
    ///
    /// "VIRTIO_NET"
    #[serde(rename="VIRTIO_NET")]
    VIRTIONET,
    

    /// GVNIC
    ///
    /// "GVNIC"
    #[serde(rename="GVNIC")]
    GVNIC,
}

impl AsRef<str> for InstanceNicTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceNicTypeEnum::UNSPECIFIEDNICTYPE => "UNSPECIFIED_NIC_TYPE",
            InstanceNicTypeEnum::VIRTIONET => "VIRTIO_NET",
            InstanceNicTypeEnum::GVNIC => "GVNIC",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceNicTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_NIC_TYPE" => Ok(InstanceNicTypeEnum::UNSPECIFIEDNICTYPE),
           "VIRTIO_NET" => Ok(InstanceNicTypeEnum::VIRTIONET),
           "GVNIC" => Ok(InstanceNicTypeEnum::GVNIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceNicTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of this instance.
pub enum InstanceStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The control logic is starting the instance.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The control logic is installing required frameworks and registering the instance with notebook proxy
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The instance is running.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The control logic is stopping the instance.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The instance is stopped.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// The instance is deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// The instance is upgrading.
    ///
    /// "UPGRADING"
    #[serde(rename="UPGRADING")]
    UPGRADING,
    

    /// The instance is being created.
    ///
    /// "INITIALIZING"
    #[serde(rename="INITIALIZING")]
    INITIALIZING,
    

    /// The instance is getting registered.
    ///
    /// "REGISTERING"
    #[serde(rename="REGISTERING")]
    REGISTERING,
    

    /// The instance is suspending.
    ///
    /// "SUSPENDING"
    #[serde(rename="SUSPENDING")]
    SUSPENDING,
    

    /// The instance is suspended.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::STARTING => "STARTING",
            InstanceStateEnum::PROVISIONING => "PROVISIONING",
            InstanceStateEnum::ACTIVE => "ACTIVE",
            InstanceStateEnum::STOPPING => "STOPPING",
            InstanceStateEnum::STOPPED => "STOPPED",
            InstanceStateEnum::DELETED => "DELETED",
            InstanceStateEnum::UPGRADING => "UPGRADING",
            InstanceStateEnum::INITIALIZING => "INITIALIZING",
            InstanceStateEnum::REGISTERING => "REGISTERING",
            InstanceStateEnum::SUSPENDING => "SUSPENDING",
            InstanceStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstanceStateEnum::STATEUNSPECIFIED),
           "STARTING" => Ok(InstanceStateEnum::STARTING),
           "PROVISIONING" => Ok(InstanceStateEnum::PROVISIONING),
           "ACTIVE" => Ok(InstanceStateEnum::ACTIVE),
           "STOPPING" => Ok(InstanceStateEnum::STOPPING),
           "STOPPED" => Ok(InstanceStateEnum::STOPPED),
           "DELETED" => Ok(InstanceStateEnum::DELETED),
           "UPGRADING" => Ok(InstanceStateEnum::UPGRADING),
           "INITIALIZING" => Ok(InstanceStateEnum::INITIALIZING),
           "REGISTERING" => Ok(InstanceStateEnum::REGISTERING),
           "SUSPENDING" => Ok(InstanceStateEnum::SUSPENDING),
           "SUSPENDED" => Ok(InstanceStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceMigrationEligibilityErrorsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Certain configurations make the UmN ineligible for an automatic migration. A manual migration is required.
pub enum InstanceMigrationEligibilityErrorsEnum {
    

    /// Default type.
    ///
    /// "ERROR_UNSPECIFIED"
    #[serde(rename="ERROR_UNSPECIFIED")]
    ERRORUNSPECIFIED,
    

    /// The UmN uses Dataproc Hub and cannot be migrated.
    ///
    /// "DATAPROC_HUB"
    #[serde(rename="DATAPROC_HUB")]
    DATAPROCHUB,
}

impl AsRef<str> for InstanceMigrationEligibilityErrorsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceMigrationEligibilityErrorsEnum::ERRORUNSPECIFIED => "ERROR_UNSPECIFIED",
            InstanceMigrationEligibilityErrorsEnum::DATAPROCHUB => "DATAPROC_HUB",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceMigrationEligibilityErrorsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_UNSPECIFIED" => Ok(InstanceMigrationEligibilityErrorsEnum::ERRORUNSPECIFIED),
           "DATAPROC_HUB" => Ok(InstanceMigrationEligibilityErrorsEnum::DATAPROCHUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceMigrationEligibilityErrorsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceMigrationEligibilityWarningsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Certain configurations will be defaulted during the migration.
pub enum InstanceMigrationEligibilityWarningsEnum {
    

    /// Default type.
    ///
    /// "WARNING_UNSPECIFIED"
    #[serde(rename="WARNING_UNSPECIFIED")]
    WARNINGUNSPECIFIED,
    

    /// The UmN uses an machine type that's unsupported in WbI. It will be migrated with the default machine type e2-standard-4. Users can change the machine type after the migration.
    ///
    /// "UNSUPPORTED_MACHINE_TYPE"
    #[serde(rename="UNSUPPORTED_MACHINE_TYPE")]
    UNSUPPORTEDMACHINETYPE,
    

    /// The UmN uses an accelerator type that's unsupported in WbI. It will be migrated without an accelerator. User can attach an accelerator after the migration.
    ///
    /// "UNSUPPORTED_ACCELERATOR_TYPE"
    #[serde(rename="UNSUPPORTED_ACCELERATOR_TYPE")]
    UNSUPPORTEDACCELERATORTYPE,
    

    /// The UmN uses an operating system that's unsupported in WbI (e.g. Debian 10, Ubuntu). It will be replaced with Debian 11 in WbI.
    ///
    /// "UNSUPPORTED_OS"
    #[serde(rename="UNSUPPORTED_OS")]
    UNSUPPORTEDOS,
    

    /// This UmN is configured with no_remove_data_disk, which is no longer available in WbI.
    ///
    /// "NO_REMOVE_DATA_DISK"
    #[serde(rename="NO_REMOVE_DATA_DISK")]
    NOREMOVEDATADISK,
    

    /// This UmN is configured with the Cloud Storage backup feature, which is no longer available in WbI.
    ///
    /// "GCS_BACKUP"
    #[serde(rename="GCS_BACKUP")]
    GCSBACKUP,
    

    /// This UmN is configured with a post startup script. Please optionally provide the `post_startup_script_option` for the migration.
    ///
    /// "POST_STARTUP_SCRIPT"
    #[serde(rename="POST_STARTUP_SCRIPT")]
    POSTSTARTUPSCRIPT,
}

impl AsRef<str> for InstanceMigrationEligibilityWarningsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceMigrationEligibilityWarningsEnum::WARNINGUNSPECIFIED => "WARNING_UNSPECIFIED",
            InstanceMigrationEligibilityWarningsEnum::UNSUPPORTEDMACHINETYPE => "UNSUPPORTED_MACHINE_TYPE",
            InstanceMigrationEligibilityWarningsEnum::UNSUPPORTEDACCELERATORTYPE => "UNSUPPORTED_ACCELERATOR_TYPE",
            InstanceMigrationEligibilityWarningsEnum::UNSUPPORTEDOS => "UNSUPPORTED_OS",
            InstanceMigrationEligibilityWarningsEnum::NOREMOVEDATADISK => "NO_REMOVE_DATA_DISK",
            InstanceMigrationEligibilityWarningsEnum::GCSBACKUP => "GCS_BACKUP",
            InstanceMigrationEligibilityWarningsEnum::POSTSTARTUPSCRIPT => "POST_STARTUP_SCRIPT",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceMigrationEligibilityWarningsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WARNING_UNSPECIFIED" => Ok(InstanceMigrationEligibilityWarningsEnum::WARNINGUNSPECIFIED),
           "UNSUPPORTED_MACHINE_TYPE" => Ok(InstanceMigrationEligibilityWarningsEnum::UNSUPPORTEDMACHINETYPE),
           "UNSUPPORTED_ACCELERATOR_TYPE" => Ok(InstanceMigrationEligibilityWarningsEnum::UNSUPPORTEDACCELERATORTYPE),
           "UNSUPPORTED_OS" => Ok(InstanceMigrationEligibilityWarningsEnum::UNSUPPORTEDOS),
           "NO_REMOVE_DATA_DISK" => Ok(InstanceMigrationEligibilityWarningsEnum::NOREMOVEDATADISK),
           "GCS_BACKUP" => Ok(InstanceMigrationEligibilityWarningsEnum::GCSBACKUP),
           "POST_STARTUP_SCRIPT" => Ok(InstanceMigrationEligibilityWarningsEnum::POSTSTARTUPSCRIPT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceMigrationEligibilityWarningsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LocalDiskInitializeParamDiskTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Input only. The type of the boot disk attached to this instance, defaults to standard persistent disk (`PD_STANDARD`).
pub enum LocalDiskInitializeParamDiskTypeEnum {
    

    /// Disk type not set.
    ///
    /// "DISK_TYPE_UNSPECIFIED"
    #[serde(rename="DISK_TYPE_UNSPECIFIED")]
    DISKTYPEUNSPECIFIED,
    

    /// Standard persistent disk type.
    ///
    /// "PD_STANDARD"
    #[serde(rename="PD_STANDARD")]
    PDSTANDARD,
    

    /// SSD persistent disk type.
    ///
    /// "PD_SSD"
    #[serde(rename="PD_SSD")]
    PDSSD,
    

    /// Balanced persistent disk type.
    ///
    /// "PD_BALANCED"
    #[serde(rename="PD_BALANCED")]
    PDBALANCED,
    

    /// Extreme persistent disk type.
    ///
    /// "PD_EXTREME"
    #[serde(rename="PD_EXTREME")]
    PDEXTREME,
}

impl AsRef<str> for LocalDiskInitializeParamDiskTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LocalDiskInitializeParamDiskTypeEnum::DISKTYPEUNSPECIFIED => "DISK_TYPE_UNSPECIFIED",
            LocalDiskInitializeParamDiskTypeEnum::PDSTANDARD => "PD_STANDARD",
            LocalDiskInitializeParamDiskTypeEnum::PDSSD => "PD_SSD",
            LocalDiskInitializeParamDiskTypeEnum::PDBALANCED => "PD_BALANCED",
            LocalDiskInitializeParamDiskTypeEnum::PDEXTREME => "PD_EXTREME",
        }
    }
}

impl std::convert::TryFrom< &str> for LocalDiskInitializeParamDiskTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISK_TYPE_UNSPECIFIED" => Ok(LocalDiskInitializeParamDiskTypeEnum::DISKTYPEUNSPECIFIED),
           "PD_STANDARD" => Ok(LocalDiskInitializeParamDiskTypeEnum::PDSTANDARD),
           "PD_SSD" => Ok(LocalDiskInitializeParamDiskTypeEnum::PDSSD),
           "PD_BALANCED" => Ok(LocalDiskInitializeParamDiskTypeEnum::PDBALANCED),
           "PD_EXTREME" => Ok(LocalDiskInitializeParamDiskTypeEnum::PDEXTREME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LocalDiskInitializeParamDiskTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrateInstanceRequestPostStartupScriptOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the behavior of post startup script during migration.
pub enum MigrateInstanceRequestPostStartupScriptOptionEnum {
    

    /// Post startup script option is not specified. Default is POST_STARTUP_SCRIPT_OPTION_SKIP.
    ///
    /// "POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED"
    #[serde(rename="POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED")]
    POSTSTARTUPSCRIPTOPTIONUNSPECIFIED,
    

    /// Not migrate the post startup script to the new Workbench Instance.
    ///
    /// "POST_STARTUP_SCRIPT_OPTION_SKIP"
    #[serde(rename="POST_STARTUP_SCRIPT_OPTION_SKIP")]
    POSTSTARTUPSCRIPTOPTIONSKIP,
    

    /// Redownload and rerun the same post startup script as the User-Managed Notebook.
    ///
    /// "POST_STARTUP_SCRIPT_OPTION_RERUN"
    #[serde(rename="POST_STARTUP_SCRIPT_OPTION_RERUN")]
    POSTSTARTUPSCRIPTOPTIONRERUN,
}

impl AsRef<str> for MigrateInstanceRequestPostStartupScriptOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrateInstanceRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONUNSPECIFIED => "POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED",
            MigrateInstanceRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONSKIP => "POST_STARTUP_SCRIPT_OPTION_SKIP",
            MigrateInstanceRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONRERUN => "POST_STARTUP_SCRIPT_OPTION_RERUN",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrateInstanceRequestPostStartupScriptOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED" => Ok(MigrateInstanceRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONUNSPECIFIED),
           "POST_STARTUP_SCRIPT_OPTION_SKIP" => Ok(MigrateInstanceRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONSKIP),
           "POST_STARTUP_SCRIPT_OPTION_RERUN" => Ok(MigrateInstanceRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONRERUN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrateInstanceRequestPostStartupScriptOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MigrateRuntimeRequestPostStartupScriptOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the behavior of post startup script during migration.
pub enum MigrateRuntimeRequestPostStartupScriptOptionEnum {
    

    /// Post startup script option is not specified. Default is POST_STARTUP_SCRIPT_OPTION_SKIP.
    ///
    /// "POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED"
    #[serde(rename="POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED")]
    POSTSTARTUPSCRIPTOPTIONUNSPECIFIED,
    

    /// Not migrate the post startup script to the new Workbench Instance.
    ///
    /// "POST_STARTUP_SCRIPT_OPTION_SKIP"
    #[serde(rename="POST_STARTUP_SCRIPT_OPTION_SKIP")]
    POSTSTARTUPSCRIPTOPTIONSKIP,
    

    /// Redownload and rerun the same post startup script as the Google-Managed Notebook.
    ///
    /// "POST_STARTUP_SCRIPT_OPTION_RERUN"
    #[serde(rename="POST_STARTUP_SCRIPT_OPTION_RERUN")]
    POSTSTARTUPSCRIPTOPTIONRERUN,
}

impl AsRef<str> for MigrateRuntimeRequestPostStartupScriptOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MigrateRuntimeRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONUNSPECIFIED => "POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED",
            MigrateRuntimeRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONSKIP => "POST_STARTUP_SCRIPT_OPTION_SKIP",
            MigrateRuntimeRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONRERUN => "POST_STARTUP_SCRIPT_OPTION_RERUN",
        }
    }
}

impl std::convert::TryFrom< &str> for MigrateRuntimeRequestPostStartupScriptOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POST_STARTUP_SCRIPT_OPTION_UNSPECIFIED" => Ok(MigrateRuntimeRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONUNSPECIFIED),
           "POST_STARTUP_SCRIPT_OPTION_SKIP" => Ok(MigrateRuntimeRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONSKIP),
           "POST_STARTUP_SCRIPT_OPTION_RERUN" => Ok(MigrateRuntimeRequestPostStartupScriptOptionEnum::POSTSTARTUPSCRIPTOPTIONRERUN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MigrateRuntimeRequestPostStartupScriptOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReservationAffinityConsumeReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of reservation to consume
pub enum ReservationAffinityConsumeReservationTypeEnum {
    

    /// Default type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Do not consume from any allocated capacity.
    ///
    /// "NO_RESERVATION"
    #[serde(rename="NO_RESERVATION")]
    NORESERVATION,
    

    /// Consume any reservation available.
    ///
    /// "ANY_RESERVATION"
    #[serde(rename="ANY_RESERVATION")]
    ANYRESERVATION,
    

    /// Must consume from a specific reservation. Must specify key value fields for specifying the reservations.
    ///
    /// "SPECIFIC_RESERVATION"
    #[serde(rename="SPECIFIC_RESERVATION")]
    SPECIFICRESERVATION,
}

impl AsRef<str> for ReservationAffinityConsumeReservationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReservationAffinityConsumeReservationTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ReservationAffinityConsumeReservationTypeEnum::NORESERVATION => "NO_RESERVATION",
            ReservationAffinityConsumeReservationTypeEnum::ANYRESERVATION => "ANY_RESERVATION",
            ReservationAffinityConsumeReservationTypeEnum::SPECIFICRESERVATION => "SPECIFIC_RESERVATION",
        }
    }
}

impl std::convert::TryFrom< &str> for ReservationAffinityConsumeReservationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ReservationAffinityConsumeReservationTypeEnum::TYPEUNSPECIFIED),
           "NO_RESERVATION" => Ok(ReservationAffinityConsumeReservationTypeEnum::NORESERVATION),
           "ANY_RESERVATION" => Ok(ReservationAffinityConsumeReservationTypeEnum::ANYRESERVATION),
           "SPECIFIC_RESERVATION" => Ok(ReservationAffinityConsumeReservationTypeEnum::SPECIFICRESERVATION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReservationAffinityConsumeReservationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeHealthStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Runtime health_state.
pub enum RuntimeHealthStateEnum {
    

    /// The runtime substate is unknown.
    ///
    /// "HEALTH_STATE_UNSPECIFIED"
    #[serde(rename="HEALTH_STATE_UNSPECIFIED")]
    HEALTHSTATEUNSPECIFIED,
    

    /// The runtime is known to be in an healthy state (for example, critical daemons are running) Applies to ACTIVE state.
    ///
    /// "HEALTHY"
    #[serde(rename="HEALTHY")]
    HEALTHY,
    

    /// The runtime is known to be in an unhealthy state (for example, critical daemons are not running) Applies to ACTIVE state.
    ///
    /// "UNHEALTHY"
    #[serde(rename="UNHEALTHY")]
    UNHEALTHY,
    

    /// The runtime has not installed health monitoring agent. Applies to ACTIVE state.
    ///
    /// "AGENT_NOT_INSTALLED"
    #[serde(rename="AGENT_NOT_INSTALLED")]
    AGENTNOTINSTALLED,
    

    /// The runtime health monitoring agent is not running. Applies to ACTIVE state.
    ///
    /// "AGENT_NOT_RUNNING"
    #[serde(rename="AGENT_NOT_RUNNING")]
    AGENTNOTRUNNING,
}

impl AsRef<str> for RuntimeHealthStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeHealthStateEnum::HEALTHSTATEUNSPECIFIED => "HEALTH_STATE_UNSPECIFIED",
            RuntimeHealthStateEnum::HEALTHY => "HEALTHY",
            RuntimeHealthStateEnum::UNHEALTHY => "UNHEALTHY",
            RuntimeHealthStateEnum::AGENTNOTINSTALLED => "AGENT_NOT_INSTALLED",
            RuntimeHealthStateEnum::AGENTNOTRUNNING => "AGENT_NOT_RUNNING",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeHealthStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEALTH_STATE_UNSPECIFIED" => Ok(RuntimeHealthStateEnum::HEALTHSTATEUNSPECIFIED),
           "HEALTHY" => Ok(RuntimeHealthStateEnum::HEALTHY),
           "UNHEALTHY" => Ok(RuntimeHealthStateEnum::UNHEALTHY),
           "AGENT_NOT_INSTALLED" => Ok(RuntimeHealthStateEnum::AGENTNOTINSTALLED),
           "AGENT_NOT_RUNNING" => Ok(RuntimeHealthStateEnum::AGENTNOTRUNNING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeHealthStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Runtime state.
pub enum RuntimeStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The compute layer is starting the runtime. It is not ready for use.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The compute layer is installing required frameworks and registering the runtime with notebook proxy. It cannot be used.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The runtime is currently running. It is ready for use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The control logic is stopping the runtime. It cannot be used.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The runtime is stopped. It cannot be used.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// The runtime is being deleted. It cannot be used.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The runtime is upgrading. It cannot be used.
    ///
    /// "UPGRADING"
    #[serde(rename="UPGRADING")]
    UPGRADING,
    

    /// The runtime is being created and set up. It is not ready for use.
    ///
    /// "INITIALIZING"
    #[serde(rename="INITIALIZING")]
    INITIALIZING,
}

impl AsRef<str> for RuntimeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            RuntimeStateEnum::STARTING => "STARTING",
            RuntimeStateEnum::PROVISIONING => "PROVISIONING",
            RuntimeStateEnum::ACTIVE => "ACTIVE",
            RuntimeStateEnum::STOPPING => "STOPPING",
            RuntimeStateEnum::STOPPED => "STOPPED",
            RuntimeStateEnum::DELETING => "DELETING",
            RuntimeStateEnum::UPGRADING => "UPGRADING",
            RuntimeStateEnum::INITIALIZING => "INITIALIZING",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(RuntimeStateEnum::STATEUNSPECIFIED),
           "STARTING" => Ok(RuntimeStateEnum::STARTING),
           "PROVISIONING" => Ok(RuntimeStateEnum::PROVISIONING),
           "ACTIVE" => Ok(RuntimeStateEnum::ACTIVE),
           "STOPPING" => Ok(RuntimeStateEnum::STOPPING),
           "STOPPED" => Ok(RuntimeStateEnum::STOPPED),
           "DELETING" => Ok(RuntimeStateEnum::DELETING),
           "UPGRADING" => Ok(RuntimeStateEnum::UPGRADING),
           "INITIALIZING" => Ok(RuntimeStateEnum::INITIALIZING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeAcceleratorConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Accelerator model.
pub enum RuntimeAcceleratorConfigTypeEnum {
    

    /// Accelerator type is not specified.
    ///
    /// "ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="ACCELERATOR_TYPE_UNSPECIFIED")]
    ACCELERATORTYPEUNSPECIFIED,
    

    /// Accelerator type is Nvidia Tesla K80.
    ///
    /// "NVIDIA_TESLA_K80"
    #[serde(rename="NVIDIA_TESLA_K80")]
    NVIDIATESLAK80,
    

    /// Accelerator type is Nvidia Tesla P100.
    ///
    /// "NVIDIA_TESLA_P100"
    #[serde(rename="NVIDIA_TESLA_P100")]
    NVIDIATESLAP100,
    

    /// Accelerator type is Nvidia Tesla V100.
    ///
    /// "NVIDIA_TESLA_V100"
    #[serde(rename="NVIDIA_TESLA_V100")]
    NVIDIATESLAV100,
    

    /// Accelerator type is Nvidia Tesla P4.
    ///
    /// "NVIDIA_TESLA_P4"
    #[serde(rename="NVIDIA_TESLA_P4")]
    NVIDIATESLAP4,
    

    /// Accelerator type is Nvidia Tesla T4.
    ///
    /// "NVIDIA_TESLA_T4"
    #[serde(rename="NVIDIA_TESLA_T4")]
    NVIDIATESLAT4,
    

    /// Accelerator type is Nvidia Tesla A100 - 40GB.
    ///
    /// "NVIDIA_TESLA_A100"
    #[serde(rename="NVIDIA_TESLA_A100")]
    NVIDIATESLAA100,
    

    /// Accelerator type is Nvidia L4.
    ///
    /// "NVIDIA_L4"
    #[serde(rename="NVIDIA_L4")]
    NVIDIAL4,
    

    /// (Coming soon) Accelerator type is TPU V2.
    ///
    /// "TPU_V2"
    #[serde(rename="TPU_V2")]
    TPUV2,
    

    /// (Coming soon) Accelerator type is TPU V3.
    ///
    /// "TPU_V3"
    #[serde(rename="TPU_V3")]
    TPUV3,
    

    /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_T4_VWS"
    #[serde(rename="NVIDIA_TESLA_T4_VWS")]
    NVIDIATESLAT4VWS,
    

    /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_P100_VWS"
    #[serde(rename="NVIDIA_TESLA_P100_VWS")]
    NVIDIATESLAP100VWS,
    

    /// Accelerator type is NVIDIA Tesla P4 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_P4_VWS"
    #[serde(rename="NVIDIA_TESLA_P4_VWS")]
    NVIDIATESLAP4VWS,
}

impl AsRef<str> for RuntimeAcceleratorConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeAcceleratorConfigTypeEnum::ACCELERATORTYPEUNSPECIFIED => "ACCELERATOR_TYPE_UNSPECIFIED",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAK80 => "NVIDIA_TESLA_K80",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP100 => "NVIDIA_TESLA_P100",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAV100 => "NVIDIA_TESLA_V100",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP4 => "NVIDIA_TESLA_P4",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAT4 => "NVIDIA_TESLA_T4",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAA100 => "NVIDIA_TESLA_A100",
            RuntimeAcceleratorConfigTypeEnum::NVIDIAL4 => "NVIDIA_L4",
            RuntimeAcceleratorConfigTypeEnum::TPUV2 => "TPU_V2",
            RuntimeAcceleratorConfigTypeEnum::TPUV3 => "TPU_V3",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAT4VWS => "NVIDIA_TESLA_T4_VWS",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP100VWS => "NVIDIA_TESLA_P100_VWS",
            RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP4VWS => "NVIDIA_TESLA_P4_VWS",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeAcceleratorConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCELERATOR_TYPE_UNSPECIFIED" => Ok(RuntimeAcceleratorConfigTypeEnum::ACCELERATORTYPEUNSPECIFIED),
           "NVIDIA_TESLA_K80" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAK80),
           "NVIDIA_TESLA_P100" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP100),
           "NVIDIA_TESLA_V100" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAV100),
           "NVIDIA_TESLA_P4" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP4),
           "NVIDIA_TESLA_T4" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAT4),
           "NVIDIA_TESLA_A100" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAA100),
           "NVIDIA_L4" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIAL4),
           "TPU_V2" => Ok(RuntimeAcceleratorConfigTypeEnum::TPUV2),
           "TPU_V3" => Ok(RuntimeAcceleratorConfigTypeEnum::TPUV3),
           "NVIDIA_TESLA_T4_VWS" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAT4VWS),
           "NVIDIA_TESLA_P100_VWS" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP100VWS),
           "NVIDIA_TESLA_P4_VWS" => Ok(RuntimeAcceleratorConfigTypeEnum::NVIDIATESLAP4VWS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeAcceleratorConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeAccessConfigAccessTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of access mode this instance.
pub enum RuntimeAccessConfigAccessTypeEnum {
    

    /// Unspecified access.
    ///
    /// "RUNTIME_ACCESS_TYPE_UNSPECIFIED"
    #[serde(rename="RUNTIME_ACCESS_TYPE_UNSPECIFIED")]
    RUNTIMEACCESSTYPEUNSPECIFIED,
    

    /// Single user login.
    ///
    /// "SINGLE_USER"
    #[serde(rename="SINGLE_USER")]
    SINGLEUSER,
    

    /// Service Account mode. In Service Account mode, Runtime creator will specify a SA that exists in the consumer project. Using Runtime Service Account field. Users accessing the Runtime need ActAs (Service Account User) permission.
    ///
    /// "SERVICE_ACCOUNT"
    #[serde(rename="SERVICE_ACCOUNT")]
    SERVICEACCOUNT,
}

impl AsRef<str> for RuntimeAccessConfigAccessTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeAccessConfigAccessTypeEnum::RUNTIMEACCESSTYPEUNSPECIFIED => "RUNTIME_ACCESS_TYPE_UNSPECIFIED",
            RuntimeAccessConfigAccessTypeEnum::SINGLEUSER => "SINGLE_USER",
            RuntimeAccessConfigAccessTypeEnum::SERVICEACCOUNT => "SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeAccessConfigAccessTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RUNTIME_ACCESS_TYPE_UNSPECIFIED" => Ok(RuntimeAccessConfigAccessTypeEnum::RUNTIMEACCESSTYPEUNSPECIFIED),
           "SINGLE_USER" => Ok(RuntimeAccessConfigAccessTypeEnum::SINGLEUSER),
           "SERVICE_ACCOUNT" => Ok(RuntimeAccessConfigAccessTypeEnum::SERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeAccessConfigAccessTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeMigrationEligibilityErrorsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Certain configurations make the GmN ineligible for an automatic migration. A manual migration is required.
pub enum RuntimeMigrationEligibilityErrorsEnum {
    

    /// Default type.
    ///
    /// "ERROR_UNSPECIFIED"
    #[serde(rename="ERROR_UNSPECIFIED")]
    ERRORUNSPECIFIED,
    

    /// The GmN is configured with custom container(s) and cannot be migrated.
    ///
    /// "CUSTOM_CONTAINER"
    #[serde(rename="CUSTOM_CONTAINER")]
    CUSTOMCONTAINER,
}

impl AsRef<str> for RuntimeMigrationEligibilityErrorsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeMigrationEligibilityErrorsEnum::ERRORUNSPECIFIED => "ERROR_UNSPECIFIED",
            RuntimeMigrationEligibilityErrorsEnum::CUSTOMCONTAINER => "CUSTOM_CONTAINER",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeMigrationEligibilityErrorsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ERROR_UNSPECIFIED" => Ok(RuntimeMigrationEligibilityErrorsEnum::ERRORUNSPECIFIED),
           "CUSTOM_CONTAINER" => Ok(RuntimeMigrationEligibilityErrorsEnum::CUSTOMCONTAINER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeMigrationEligibilityErrorsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeMigrationEligibilityWarningsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Certain configurations will be defaulted during the migration.
pub enum RuntimeMigrationEligibilityWarningsEnum {
    

    /// Default type.
    ///
    /// "WARNING_UNSPECIFIED"
    #[serde(rename="WARNING_UNSPECIFIED")]
    WARNINGUNSPECIFIED,
    

    /// The GmN uses an accelerator type that's unsupported in WbI. It will be migrated without an accelerator. Users can attach an accelerator after the migration.
    ///
    /// "UNSUPPORTED_ACCELERATOR_TYPE"
    #[serde(rename="UNSUPPORTED_ACCELERATOR_TYPE")]
    UNSUPPORTEDACCELERATORTYPE,
    

    /// The GmN uses an operating system that's unsupported in WbI (e.g. Debian 10). It will be replaced with Debian 11 in WbI.
    ///
    /// "UNSUPPORTED_OS"
    #[serde(rename="UNSUPPORTED_OS")]
    UNSUPPORTEDOS,
    

    /// This GmN is configured with reserved IP range, which is no longer applicable in WbI.
    ///
    /// "RESERVED_IP_RANGE"
    #[serde(rename="RESERVED_IP_RANGE")]
    RESERVEDIPRANGE,
    

    /// This GmN is configured with a Google managed network. Please provide the `network` and `subnet` options for the migration.
    ///
    /// "GOOGLE_MANAGED_NETWORK"
    #[serde(rename="GOOGLE_MANAGED_NETWORK")]
    GOOGLEMANAGEDNETWORK,
    

    /// This GmN is configured with a post startup script. Please optionally provide the `post_startup_script_option` for the migration.
    ///
    /// "POST_STARTUP_SCRIPT"
    #[serde(rename="POST_STARTUP_SCRIPT")]
    POSTSTARTUPSCRIPT,
    

    /// This GmN is configured with single user mode. Please optionally provide the `service_account` option for the migration.
    ///
    /// "SINGLE_USER"
    #[serde(rename="SINGLE_USER")]
    SINGLEUSER,
}

impl AsRef<str> for RuntimeMigrationEligibilityWarningsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeMigrationEligibilityWarningsEnum::WARNINGUNSPECIFIED => "WARNING_UNSPECIFIED",
            RuntimeMigrationEligibilityWarningsEnum::UNSUPPORTEDACCELERATORTYPE => "UNSUPPORTED_ACCELERATOR_TYPE",
            RuntimeMigrationEligibilityWarningsEnum::UNSUPPORTEDOS => "UNSUPPORTED_OS",
            RuntimeMigrationEligibilityWarningsEnum::RESERVEDIPRANGE => "RESERVED_IP_RANGE",
            RuntimeMigrationEligibilityWarningsEnum::GOOGLEMANAGEDNETWORK => "GOOGLE_MANAGED_NETWORK",
            RuntimeMigrationEligibilityWarningsEnum::POSTSTARTUPSCRIPT => "POST_STARTUP_SCRIPT",
            RuntimeMigrationEligibilityWarningsEnum::SINGLEUSER => "SINGLE_USER",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeMigrationEligibilityWarningsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WARNING_UNSPECIFIED" => Ok(RuntimeMigrationEligibilityWarningsEnum::WARNINGUNSPECIFIED),
           "UNSUPPORTED_ACCELERATOR_TYPE" => Ok(RuntimeMigrationEligibilityWarningsEnum::UNSUPPORTEDACCELERATORTYPE),
           "UNSUPPORTED_OS" => Ok(RuntimeMigrationEligibilityWarningsEnum::UNSUPPORTEDOS),
           "RESERVED_IP_RANGE" => Ok(RuntimeMigrationEligibilityWarningsEnum::RESERVEDIPRANGE),
           "GOOGLE_MANAGED_NETWORK" => Ok(RuntimeMigrationEligibilityWarningsEnum::GOOGLEMANAGEDNETWORK),
           "POST_STARTUP_SCRIPT" => Ok(RuntimeMigrationEligibilityWarningsEnum::POSTSTARTUPSCRIPT),
           "SINGLE_USER" => Ok(RuntimeMigrationEligibilityWarningsEnum::SINGLEUSER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeMigrationEligibilityWarningsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RuntimeSoftwareConfigPostStartupScriptBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Behavior for the post startup script.
pub enum RuntimeSoftwareConfigPostStartupScriptBehaviorEnum {
    

    /// Unspecified post startup script behavior. Will run only once at creation.
    ///
    /// "POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED"
    #[serde(rename="POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED")]
    POSTSTARTUPSCRIPTBEHAVIORUNSPECIFIED,
    

    /// Runs the post startup script provided during creation at every start.
    ///
    /// "RUN_EVERY_START"
    #[serde(rename="RUN_EVERY_START")]
    RUNEVERYSTART,
    

    /// Downloads and runs the provided post startup script at every start.
    ///
    /// "DOWNLOAD_AND_RUN_EVERY_START"
    #[serde(rename="DOWNLOAD_AND_RUN_EVERY_START")]
    DOWNLOADANDRUNEVERYSTART,
}

impl AsRef<str> for RuntimeSoftwareConfigPostStartupScriptBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RuntimeSoftwareConfigPostStartupScriptBehaviorEnum::POSTSTARTUPSCRIPTBEHAVIORUNSPECIFIED => "POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED",
            RuntimeSoftwareConfigPostStartupScriptBehaviorEnum::RUNEVERYSTART => "RUN_EVERY_START",
            RuntimeSoftwareConfigPostStartupScriptBehaviorEnum::DOWNLOADANDRUNEVERYSTART => "DOWNLOAD_AND_RUN_EVERY_START",
        }
    }
}

impl std::convert::TryFrom< &str> for RuntimeSoftwareConfigPostStartupScriptBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED" => Ok(RuntimeSoftwareConfigPostStartupScriptBehaviorEnum::POSTSTARTUPSCRIPTBEHAVIORUNSPECIFIED),
           "RUN_EVERY_START" => Ok(RuntimeSoftwareConfigPostStartupScriptBehaviorEnum::RUNEVERYSTART),
           "DOWNLOAD_AND_RUN_EVERY_START" => Ok(RuntimeSoftwareConfigPostStartupScriptBehaviorEnum::DOWNLOADANDRUNEVERYSTART),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RuntimeSoftwareConfigPostStartupScriptBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScheduleStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScheduleStateEnum {
    

    /// Unspecified state.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job is executing normally.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// The job is paused by the user. It will not execute. A user can intentionally pause the job using PauseJobRequest.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// The job is disabled by the system due to error. The user cannot directly set a job to be disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// The job state resulting from a failed CloudScheduler.UpdateJob operation. To recover a job from this state, retry CloudScheduler.UpdateJob until a successful response is received.
    ///
    /// "UPDATE_FAILED"
    #[serde(rename="UPDATE_FAILED")]
    UPDATEFAILED,
    

    /// The schedule resource is being created.
    ///
    /// "INITIALIZING"
    #[serde(rename="INITIALIZING")]
    INITIALIZING,
    

    /// The schedule resource is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
}

impl AsRef<str> for ScheduleStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScheduleStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            ScheduleStateEnum::ENABLED => "ENABLED",
            ScheduleStateEnum::PAUSED => "PAUSED",
            ScheduleStateEnum::DISABLED => "DISABLED",
            ScheduleStateEnum::UPDATEFAILED => "UPDATE_FAILED",
            ScheduleStateEnum::INITIALIZING => "INITIALIZING",
            ScheduleStateEnum::DELETING => "DELETING",
        }
    }
}

impl std::convert::TryFrom< &str> for ScheduleStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(ScheduleStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(ScheduleStateEnum::ENABLED),
           "PAUSED" => Ok(ScheduleStateEnum::PAUSED),
           "DISABLED" => Ok(ScheduleStateEnum::DISABLED),
           "UPDATE_FAILED" => Ok(ScheduleStateEnum::UPDATEFAILED),
           "INITIALIZING" => Ok(ScheduleStateEnum::INITIALIZING),
           "DELETING" => Ok(ScheduleStateEnum::DELETING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScheduleStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SchedulerAcceleratorConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this accelerator.
pub enum SchedulerAcceleratorConfigTypeEnum {
    

    /// Unspecified accelerator type. Default to no GPU.
    ///
    /// "SCHEDULER_ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="SCHEDULER_ACCELERATOR_TYPE_UNSPECIFIED")]
    SCHEDULERACCELERATORTYPEUNSPECIFIED,
    

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
    

    /// Nvidia Tesla V100 GPU.
    ///
    /// "NVIDIA_TESLA_V100"
    #[serde(rename="NVIDIA_TESLA_V100")]
    NVIDIATESLAV100,
    

    /// Nvidia Tesla P4 GPU.
    ///
    /// "NVIDIA_TESLA_P4"
    #[serde(rename="NVIDIA_TESLA_P4")]
    NVIDIATESLAP4,
    

    /// Nvidia Tesla T4 GPU.
    ///
    /// "NVIDIA_TESLA_T4"
    #[serde(rename="NVIDIA_TESLA_T4")]
    NVIDIATESLAT4,
    

    /// Nvidia Tesla A100 GPU.
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
}

impl AsRef<str> for SchedulerAcceleratorConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SchedulerAcceleratorConfigTypeEnum::SCHEDULERACCELERATORTYPEUNSPECIFIED => "SCHEDULER_ACCELERATOR_TYPE_UNSPECIFIED",
            SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAK80 => "NVIDIA_TESLA_K80",
            SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAP100 => "NVIDIA_TESLA_P100",
            SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAV100 => "NVIDIA_TESLA_V100",
            SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAP4 => "NVIDIA_TESLA_P4",
            SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAT4 => "NVIDIA_TESLA_T4",
            SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAA100 => "NVIDIA_TESLA_A100",
            SchedulerAcceleratorConfigTypeEnum::TPUV2 => "TPU_V2",
            SchedulerAcceleratorConfigTypeEnum::TPUV3 => "TPU_V3",
        }
    }
}

impl std::convert::TryFrom< &str> for SchedulerAcceleratorConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEDULER_ACCELERATOR_TYPE_UNSPECIFIED" => Ok(SchedulerAcceleratorConfigTypeEnum::SCHEDULERACCELERATORTYPEUNSPECIFIED),
           "NVIDIA_TESLA_K80" => Ok(SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAK80),
           "NVIDIA_TESLA_P100" => Ok(SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAP100),
           "NVIDIA_TESLA_V100" => Ok(SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAV100),
           "NVIDIA_TESLA_P4" => Ok(SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAP4),
           "NVIDIA_TESLA_T4" => Ok(SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAT4),
           "NVIDIA_TESLA_A100" => Ok(SchedulerAcceleratorConfigTypeEnum::NVIDIATESLAA100),
           "TPU_V2" => Ok(SchedulerAcceleratorConfigTypeEnum::TPUV2),
           "TPU_V3" => Ok(SchedulerAcceleratorConfigTypeEnum::TPUV3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SchedulerAcceleratorConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SetInstanceAcceleratorRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Type of this accelerator.
pub enum SetInstanceAcceleratorRequestTypeEnum {
    

    /// Accelerator type is not specified.
    ///
    /// "ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="ACCELERATOR_TYPE_UNSPECIFIED")]
    ACCELERATORTYPEUNSPECIFIED,
    

    /// Accelerator type is Nvidia Tesla K80.
    ///
    /// "NVIDIA_TESLA_K80"
    #[serde(rename="NVIDIA_TESLA_K80")]
    NVIDIATESLAK80,
    

    /// Accelerator type is Nvidia Tesla P100.
    ///
    /// "NVIDIA_TESLA_P100"
    #[serde(rename="NVIDIA_TESLA_P100")]
    NVIDIATESLAP100,
    

    /// Accelerator type is Nvidia Tesla V100.
    ///
    /// "NVIDIA_TESLA_V100"
    #[serde(rename="NVIDIA_TESLA_V100")]
    NVIDIATESLAV100,
    

    /// Accelerator type is Nvidia Tesla P4.
    ///
    /// "NVIDIA_TESLA_P4"
    #[serde(rename="NVIDIA_TESLA_P4")]
    NVIDIATESLAP4,
    

    /// Accelerator type is Nvidia Tesla T4.
    ///
    /// "NVIDIA_TESLA_T4"
    #[serde(rename="NVIDIA_TESLA_T4")]
    NVIDIATESLAT4,
    

    /// Accelerator type is Nvidia Tesla A100.
    ///
    /// "NVIDIA_TESLA_A100"
    #[serde(rename="NVIDIA_TESLA_A100")]
    NVIDIATESLAA100,
    

    /// Accelerator type is Nvidia Tesla L4.
    ///
    /// "NVIDIA_L4"
    #[serde(rename="NVIDIA_L4")]
    NVIDIAL4,
    

    /// Accelerator type is Nvidia Tesla A100 80GB.
    ///
    /// "NVIDIA_A100_80GB"
    #[serde(rename="NVIDIA_A100_80GB")]
    NVIDIAA10080GB,
    

    /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_T4_VWS"
    #[serde(rename="NVIDIA_TESLA_T4_VWS")]
    NVIDIATESLAT4VWS,
    

    /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_P100_VWS"
    #[serde(rename="NVIDIA_TESLA_P100_VWS")]
    NVIDIATESLAP100VWS,
    

    /// Accelerator type is NVIDIA Tesla P4 Virtual Workstations.
    ///
    /// "NVIDIA_TESLA_P4_VWS"
    #[serde(rename="NVIDIA_TESLA_P4_VWS")]
    NVIDIATESLAP4VWS,
    

    /// (Coming soon) Accelerator type is TPU V2.
    ///
    /// "TPU_V2"
    #[serde(rename="TPU_V2")]
    TPUV2,
    

    /// (Coming soon) Accelerator type is TPU V3.
    ///
    /// "TPU_V3"
    #[serde(rename="TPU_V3")]
    TPUV3,
}

impl AsRef<str> for SetInstanceAcceleratorRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SetInstanceAcceleratorRequestTypeEnum::ACCELERATORTYPEUNSPECIFIED => "ACCELERATOR_TYPE_UNSPECIFIED",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAK80 => "NVIDIA_TESLA_K80",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP100 => "NVIDIA_TESLA_P100",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAV100 => "NVIDIA_TESLA_V100",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP4 => "NVIDIA_TESLA_P4",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAT4 => "NVIDIA_TESLA_T4",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAA100 => "NVIDIA_TESLA_A100",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIAL4 => "NVIDIA_L4",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIAA10080GB => "NVIDIA_A100_80GB",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAT4VWS => "NVIDIA_TESLA_T4_VWS",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP100VWS => "NVIDIA_TESLA_P100_VWS",
            SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP4VWS => "NVIDIA_TESLA_P4_VWS",
            SetInstanceAcceleratorRequestTypeEnum::TPUV2 => "TPU_V2",
            SetInstanceAcceleratorRequestTypeEnum::TPUV3 => "TPU_V3",
        }
    }
}

impl std::convert::TryFrom< &str> for SetInstanceAcceleratorRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCELERATOR_TYPE_UNSPECIFIED" => Ok(SetInstanceAcceleratorRequestTypeEnum::ACCELERATORTYPEUNSPECIFIED),
           "NVIDIA_TESLA_K80" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAK80),
           "NVIDIA_TESLA_P100" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP100),
           "NVIDIA_TESLA_V100" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAV100),
           "NVIDIA_TESLA_P4" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP4),
           "NVIDIA_TESLA_T4" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAT4),
           "NVIDIA_TESLA_A100" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAA100),
           "NVIDIA_L4" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIAL4),
           "NVIDIA_A100_80GB" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIAA10080GB),
           "NVIDIA_TESLA_T4_VWS" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAT4VWS),
           "NVIDIA_TESLA_P100_VWS" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP100VWS),
           "NVIDIA_TESLA_P4_VWS" => Ok(SetInstanceAcceleratorRequestTypeEnum::NVIDIATESLAP4VWS),
           "TPU_V2" => Ok(SetInstanceAcceleratorRequestTypeEnum::TPUV2),
           "TPU_V3" => Ok(SetInstanceAcceleratorRequestTypeEnum::TPUV3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SetInstanceAcceleratorRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpgradeHistoryEntryActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Action. Rolloback or Upgrade.
pub enum UpgradeHistoryEntryActionEnum {
    

    /// Operation is not specified.
    ///
    /// "ACTION_UNSPECIFIED"
    #[serde(rename="ACTION_UNSPECIFIED")]
    ACTIONUNSPECIFIED,
    

    /// Upgrade.
    ///
    /// "UPGRADE"
    #[serde(rename="UPGRADE")]
    UPGRADE,
    

    /// Rollback.
    ///
    /// "ROLLBACK"
    #[serde(rename="ROLLBACK")]
    ROLLBACK,
}

impl AsRef<str> for UpgradeHistoryEntryActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpgradeHistoryEntryActionEnum::ACTIONUNSPECIFIED => "ACTION_UNSPECIFIED",
            UpgradeHistoryEntryActionEnum::UPGRADE => "UPGRADE",
            UpgradeHistoryEntryActionEnum::ROLLBACK => "ROLLBACK",
        }
    }
}

impl std::convert::TryFrom< &str> for UpgradeHistoryEntryActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNSPECIFIED" => Ok(UpgradeHistoryEntryActionEnum::ACTIONUNSPECIFIED),
           "UPGRADE" => Ok(UpgradeHistoryEntryActionEnum::UPGRADE),
           "ROLLBACK" => Ok(UpgradeHistoryEntryActionEnum::ROLLBACK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpgradeHistoryEntryActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpgradeHistoryEntryStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of this instance upgrade history entry.
pub enum UpgradeHistoryEntryStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The instance upgrade is started.
    ///
    /// "STARTED"
    #[serde(rename="STARTED")]
    STARTED,
    

    /// The instance upgrade is succeeded.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The instance upgrade is failed.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for UpgradeHistoryEntryStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpgradeHistoryEntryStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            UpgradeHistoryEntryStateEnum::STARTED => "STARTED",
            UpgradeHistoryEntryStateEnum::SUCCEEDED => "SUCCEEDED",
            UpgradeHistoryEntryStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for UpgradeHistoryEntryStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(UpgradeHistoryEntryStateEnum::STATEUNSPECIFIED),
           "STARTED" => Ok(UpgradeHistoryEntryStateEnum::STARTED),
           "SUCCEEDED" => Ok(UpgradeHistoryEntryStateEnum::SUCCEEDED),
           "FAILED" => Ok(UpgradeHistoryEntryStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpgradeHistoryEntryStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpgradeInstanceInternalRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The optional UpgradeType. Setting this field will search for additional compute images to upgrade this instance.
pub enum UpgradeInstanceInternalRequestTypeEnum {
    

    /// Upgrade type is not specified.
    ///
    /// "UPGRADE_TYPE_UNSPECIFIED"
    #[serde(rename="UPGRADE_TYPE_UNSPECIFIED")]
    UPGRADETYPEUNSPECIFIED,
    

    /// Upgrade ML framework.
    ///
    /// "UPGRADE_FRAMEWORK"
    #[serde(rename="UPGRADE_FRAMEWORK")]
    UPGRADEFRAMEWORK,
    

    /// Upgrade Operating System.
    ///
    /// "UPGRADE_OS"
    #[serde(rename="UPGRADE_OS")]
    UPGRADEOS,
    

    /// Upgrade CUDA.
    ///
    /// "UPGRADE_CUDA"
    #[serde(rename="UPGRADE_CUDA")]
    UPGRADECUDA,
    

    /// Upgrade All (OS, Framework and CUDA).
    ///
    /// "UPGRADE_ALL"
    #[serde(rename="UPGRADE_ALL")]
    UPGRADEALL,
}

impl AsRef<str> for UpgradeInstanceInternalRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpgradeInstanceInternalRequestTypeEnum::UPGRADETYPEUNSPECIFIED => "UPGRADE_TYPE_UNSPECIFIED",
            UpgradeInstanceInternalRequestTypeEnum::UPGRADEFRAMEWORK => "UPGRADE_FRAMEWORK",
            UpgradeInstanceInternalRequestTypeEnum::UPGRADEOS => "UPGRADE_OS",
            UpgradeInstanceInternalRequestTypeEnum::UPGRADECUDA => "UPGRADE_CUDA",
            UpgradeInstanceInternalRequestTypeEnum::UPGRADEALL => "UPGRADE_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for UpgradeInstanceInternalRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPGRADE_TYPE_UNSPECIFIED" => Ok(UpgradeInstanceInternalRequestTypeEnum::UPGRADETYPEUNSPECIFIED),
           "UPGRADE_FRAMEWORK" => Ok(UpgradeInstanceInternalRequestTypeEnum::UPGRADEFRAMEWORK),
           "UPGRADE_OS" => Ok(UpgradeInstanceInternalRequestTypeEnum::UPGRADEOS),
           "UPGRADE_CUDA" => Ok(UpgradeInstanceInternalRequestTypeEnum::UPGRADECUDA),
           "UPGRADE_ALL" => Ok(UpgradeInstanceInternalRequestTypeEnum::UPGRADEALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpgradeInstanceInternalRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpgradeInstanceRequestTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The optional UpgradeType. Setting this field will search for additional compute images to upgrade this instance.
pub enum UpgradeInstanceRequestTypeEnum {
    

    /// Upgrade type is not specified.
    ///
    /// "UPGRADE_TYPE_UNSPECIFIED"
    #[serde(rename="UPGRADE_TYPE_UNSPECIFIED")]
    UPGRADETYPEUNSPECIFIED,
    

    /// Upgrade ML framework.
    ///
    /// "UPGRADE_FRAMEWORK"
    #[serde(rename="UPGRADE_FRAMEWORK")]
    UPGRADEFRAMEWORK,
    

    /// Upgrade Operating System.
    ///
    /// "UPGRADE_OS"
    #[serde(rename="UPGRADE_OS")]
    UPGRADEOS,
    

    /// Upgrade CUDA.
    ///
    /// "UPGRADE_CUDA"
    #[serde(rename="UPGRADE_CUDA")]
    UPGRADECUDA,
    

    /// Upgrade All (OS, Framework and CUDA).
    ///
    /// "UPGRADE_ALL"
    #[serde(rename="UPGRADE_ALL")]
    UPGRADEALL,
}

impl AsRef<str> for UpgradeInstanceRequestTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpgradeInstanceRequestTypeEnum::UPGRADETYPEUNSPECIFIED => "UPGRADE_TYPE_UNSPECIFIED",
            UpgradeInstanceRequestTypeEnum::UPGRADEFRAMEWORK => "UPGRADE_FRAMEWORK",
            UpgradeInstanceRequestTypeEnum::UPGRADEOS => "UPGRADE_OS",
            UpgradeInstanceRequestTypeEnum::UPGRADECUDA => "UPGRADE_CUDA",
            UpgradeInstanceRequestTypeEnum::UPGRADEALL => "UPGRADE_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for UpgradeInstanceRequestTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPGRADE_TYPE_UNSPECIFIED" => Ok(UpgradeInstanceRequestTypeEnum::UPGRADETYPEUNSPECIFIED),
           "UPGRADE_FRAMEWORK" => Ok(UpgradeInstanceRequestTypeEnum::UPGRADEFRAMEWORK),
           "UPGRADE_OS" => Ok(UpgradeInstanceRequestTypeEnum::UPGRADEOS),
           "UPGRADE_CUDA" => Ok(UpgradeInstanceRequestTypeEnum::UPGRADECUDA),
           "UPGRADE_ALL" => Ok(UpgradeInstanceRequestTypeEnum::UPGRADEALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpgradeInstanceRequestTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VirtualMachineConfigNicTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of vNIC to be used on this interface. This may be gVNIC or VirtioNet.
pub enum VirtualMachineConfigNicTypeEnum {
    

    /// No type specified.
    ///
    /// "UNSPECIFIED_NIC_TYPE"
    #[serde(rename="UNSPECIFIED_NIC_TYPE")]
    UNSPECIFIEDNICTYPE,
    

    /// VIRTIO
    ///
    /// "VIRTIO_NET"
    #[serde(rename="VIRTIO_NET")]
    VIRTIONET,
    

    /// GVNIC
    ///
    /// "GVNIC"
    #[serde(rename="GVNIC")]
    GVNIC,
}

impl AsRef<str> for VirtualMachineConfigNicTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VirtualMachineConfigNicTypeEnum::UNSPECIFIEDNICTYPE => "UNSPECIFIED_NIC_TYPE",
            VirtualMachineConfigNicTypeEnum::VIRTIONET => "VIRTIO_NET",
            VirtualMachineConfigNicTypeEnum::GVNIC => "GVNIC",
        }
    }
}

impl std::convert::TryFrom< &str> for VirtualMachineConfigNicTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_NIC_TYPE" => Ok(VirtualMachineConfigNicTypeEnum::UNSPECIFIEDNICTYPE),
           "VIRTIO_NET" => Ok(VirtualMachineConfigNicTypeEnum::VIRTIONET),
           "GVNIC" => Ok(VirtualMachineConfigNicTypeEnum::GVNIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VirtualMachineConfigNicTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The optional UpgradeType. Setting this field will search for additional compute images to upgrade this instance.
pub enum ProjectTypeEnum {
    

    /// Upgrade type is not specified.
    ///
    /// "UPGRADE_TYPE_UNSPECIFIED"
    #[serde(rename="UPGRADE_TYPE_UNSPECIFIED")]
    UPGRADETYPEUNSPECIFIED,
    

    /// Upgrade ML framework.
    ///
    /// "UPGRADE_FRAMEWORK"
    #[serde(rename="UPGRADE_FRAMEWORK")]
    UPGRADEFRAMEWORK,
    

    /// Upgrade Operating System.
    ///
    /// "UPGRADE_OS"
    #[serde(rename="UPGRADE_OS")]
    UPGRADEOS,
    

    /// Upgrade CUDA.
    ///
    /// "UPGRADE_CUDA"
    #[serde(rename="UPGRADE_CUDA")]
    UPGRADECUDA,
    

    /// Upgrade All (OS, Framework and CUDA).
    ///
    /// "UPGRADE_ALL"
    #[serde(rename="UPGRADE_ALL")]
    UPGRADEALL,
}

impl AsRef<str> for ProjectTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectTypeEnum::UPGRADETYPEUNSPECIFIED => "UPGRADE_TYPE_UNSPECIFIED",
            ProjectTypeEnum::UPGRADEFRAMEWORK => "UPGRADE_FRAMEWORK",
            ProjectTypeEnum::UPGRADEOS => "UPGRADE_OS",
            ProjectTypeEnum::UPGRADECUDA => "UPGRADE_CUDA",
            ProjectTypeEnum::UPGRADEALL => "UPGRADE_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UPGRADE_TYPE_UNSPECIFIED" => Ok(ProjectTypeEnum::UPGRADETYPEUNSPECIFIED),
           "UPGRADE_FRAMEWORK" => Ok(ProjectTypeEnum::UPGRADEFRAMEWORK),
           "UPGRADE_OS" => Ok(ProjectTypeEnum::UPGRADEOS),
           "UPGRADE_CUDA" => Ok(ProjectTypeEnum::UPGRADECUDA),
           "UPGRADE_ALL" => Ok(ProjectTypeEnum::UPGRADEALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


