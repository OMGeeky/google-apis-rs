use super::*;



// region NodeApiVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The API version that created this Node.
pub enum NodeApiVersionEnum {
    

    /// API version is unknown.
    ///
    /// "API_VERSION_UNSPECIFIED"
    #[serde(rename="API_VERSION_UNSPECIFIED")]
    APIVERSIONUNSPECIFIED,
    

    /// TPU API V1Alpha1 version.
    ///
    /// "V1_ALPHA1"
    #[serde(rename="V1_ALPHA1")]
    V1ALPHA1,
    

    /// TPU API V1 version.
    ///
    /// "V1"
    #[serde(rename="V1")]
    V1,
    

    /// TPU API V2Alpha1 version.
    ///
    /// "V2_ALPHA1"
    #[serde(rename="V2_ALPHA1")]
    V2ALPHA1,
}

impl AsRef<str> for NodeApiVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeApiVersionEnum::APIVERSIONUNSPECIFIED => "API_VERSION_UNSPECIFIED",
            NodeApiVersionEnum::V1ALPHA1 => "V1_ALPHA1",
            NodeApiVersionEnum::V1 => "V1",
            NodeApiVersionEnum::V2ALPHA1 => "V2_ALPHA1",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeApiVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "API_VERSION_UNSPECIFIED" => Ok(NodeApiVersionEnum::APIVERSIONUNSPECIFIED),
           "V1_ALPHA1" => Ok(NodeApiVersionEnum::V1ALPHA1),
           "V1" => Ok(NodeApiVersionEnum::V1),
           "V2_ALPHA1" => Ok(NodeApiVersionEnum::V2ALPHA1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeApiVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeHealthEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The health status of the TPU node.
pub enum NodeHealthEnum {
    

    /// Health status is unknown: not initialized or failed to retrieve.
    ///
    /// "HEALTH_UNSPECIFIED"
    #[serde(rename="HEALTH_UNSPECIFIED")]
    HEALTHUNSPECIFIED,
    

    /// The resource is healthy.
    ///
    /// "HEALTHY"
    #[serde(rename="HEALTHY")]
    HEALTHY,
    

    /// The resource is unhealthy.
    ///
    /// "DEPRECATED_UNHEALTHY"
    #[serde(rename="DEPRECATED_UNHEALTHY")]
    DEPRECATEDUNHEALTHY,
    

    /// The resource is unresponsive.
    ///
    /// "TIMEOUT"
    #[serde(rename="TIMEOUT")]
    TIMEOUT,
    

    /// The in-guest ML stack is unhealthy.
    ///
    /// "UNHEALTHY_TENSORFLOW"
    #[serde(rename="UNHEALTHY_TENSORFLOW")]
    UNHEALTHYTENSORFLOW,
    

    /// The node is under maintenance/priority boost caused rescheduling and will resume running once rescheduled.
    ///
    /// "UNHEALTHY_MAINTENANCE"
    #[serde(rename="UNHEALTHY_MAINTENANCE")]
    UNHEALTHYMAINTENANCE,
}

impl AsRef<str> for NodeHealthEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeHealthEnum::HEALTHUNSPECIFIED => "HEALTH_UNSPECIFIED",
            NodeHealthEnum::HEALTHY => "HEALTHY",
            NodeHealthEnum::DEPRECATEDUNHEALTHY => "DEPRECATED_UNHEALTHY",
            NodeHealthEnum::TIMEOUT => "TIMEOUT",
            NodeHealthEnum::UNHEALTHYTENSORFLOW => "UNHEALTHY_TENSORFLOW",
            NodeHealthEnum::UNHEALTHYMAINTENANCE => "UNHEALTHY_MAINTENANCE",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeHealthEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HEALTH_UNSPECIFIED" => Ok(NodeHealthEnum::HEALTHUNSPECIFIED),
           "HEALTHY" => Ok(NodeHealthEnum::HEALTHY),
           "DEPRECATED_UNHEALTHY" => Ok(NodeHealthEnum::DEPRECATEDUNHEALTHY),
           "TIMEOUT" => Ok(NodeHealthEnum::TIMEOUT),
           "UNHEALTHY_TENSORFLOW" => Ok(NodeHealthEnum::UNHEALTHYTENSORFLOW),
           "UNHEALTHY_MAINTENANCE" => Ok(NodeHealthEnum::UNHEALTHYMAINTENANCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeHealthEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state for the TPU Node.
pub enum NodeStateEnum {
    

    /// TPU node state is not known/set.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// TPU node is being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// TPU node has been created.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// TPU node is restarting.
    ///
    /// "RESTARTING"
    #[serde(rename="RESTARTING")]
    RESTARTING,
    

    /// TPU node is undergoing reimaging.
    ///
    /// "REIMAGING"
    #[serde(rename="REIMAGING")]
    REIMAGING,
    

    /// TPU node is being deleted.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// TPU node is being repaired and may be unusable. Details can be found in the `help_description` field.
    ///
    /// "REPAIRING"
    #[serde(rename="REPAIRING")]
    REPAIRING,
    

    /// TPU node is stopped.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// TPU node is currently stopping.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// TPU node is currently starting.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// TPU node has been preempted. Only applies to Preemptible TPU Nodes.
    ///
    /// "PREEMPTED"
    #[serde(rename="PREEMPTED")]
    PREEMPTED,
    

    /// TPU node has been terminated due to maintenance or has reached the end of its life cycle (for preemptible nodes).
    ///
    /// "TERMINATED"
    #[serde(rename="TERMINATED")]
    TERMINATED,
    

    /// TPU node is currently hiding.
    ///
    /// "HIDING"
    #[serde(rename="HIDING")]
    HIDING,
    

    /// TPU node has been hidden.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// TPU node is currently unhiding.
    ///
    /// "UNHIDING"
    #[serde(rename="UNHIDING")]
    UNHIDING,
}

impl AsRef<str> for NodeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            NodeStateEnum::CREATING => "CREATING",
            NodeStateEnum::READY => "READY",
            NodeStateEnum::RESTARTING => "RESTARTING",
            NodeStateEnum::REIMAGING => "REIMAGING",
            NodeStateEnum::DELETING => "DELETING",
            NodeStateEnum::REPAIRING => "REPAIRING",
            NodeStateEnum::STOPPED => "STOPPED",
            NodeStateEnum::STOPPING => "STOPPING",
            NodeStateEnum::STARTING => "STARTING",
            NodeStateEnum::PREEMPTED => "PREEMPTED",
            NodeStateEnum::TERMINATED => "TERMINATED",
            NodeStateEnum::HIDING => "HIDING",
            NodeStateEnum::HIDDEN => "HIDDEN",
            NodeStateEnum::UNHIDING => "UNHIDING",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(NodeStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(NodeStateEnum::CREATING),
           "READY" => Ok(NodeStateEnum::READY),
           "RESTARTING" => Ok(NodeStateEnum::RESTARTING),
           "REIMAGING" => Ok(NodeStateEnum::REIMAGING),
           "DELETING" => Ok(NodeStateEnum::DELETING),
           "REPAIRING" => Ok(NodeStateEnum::REPAIRING),
           "STOPPED" => Ok(NodeStateEnum::STOPPED),
           "STOPPING" => Ok(NodeStateEnum::STOPPING),
           "STARTING" => Ok(NodeStateEnum::STARTING),
           "PREEMPTED" => Ok(NodeStateEnum::PREEMPTED),
           "TERMINATED" => Ok(NodeStateEnum::TERMINATED),
           "HIDING" => Ok(NodeStateEnum::HIDING),
           "HIDDEN" => Ok(NodeStateEnum::HIDDEN),
           "UNHIDING" => Ok(NodeStateEnum::UNHIDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SymptomSymptomTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the Symptom.
pub enum SymptomSymptomTypeEnum {
    

    /// Unspecified symptom.
    ///
    /// "SYMPTOM_TYPE_UNSPECIFIED"
    #[serde(rename="SYMPTOM_TYPE_UNSPECIFIED")]
    SYMPTOMTYPEUNSPECIFIED,
    

    /// TPU VM memory is low.
    ///
    /// "LOW_MEMORY"
    #[serde(rename="LOW_MEMORY")]
    LOWMEMORY,
    

    /// TPU runtime is out of memory.
    ///
    /// "OUT_OF_MEMORY"
    #[serde(rename="OUT_OF_MEMORY")]
    OUTOFMEMORY,
    

    /// TPU runtime execution has timed out.
    ///
    /// "EXECUTE_TIMED_OUT"
    #[serde(rename="EXECUTE_TIMED_OUT")]
    EXECUTETIMEDOUT,
    

    /// TPU runtime fails to construct a mesh that recognizes each TPU device's neighbors.
    ///
    /// "MESH_BUILD_FAIL"
    #[serde(rename="MESH_BUILD_FAIL")]
    MESHBUILDFAIL,
    

    /// TPU HBM is out of memory.
    ///
    /// "HBM_OUT_OF_MEMORY"
    #[serde(rename="HBM_OUT_OF_MEMORY")]
    HBMOUTOFMEMORY,
    

    /// Abusive behaviors have been identified on the current project.
    ///
    /// "PROJECT_ABUSE"
    #[serde(rename="PROJECT_ABUSE")]
    PROJECTABUSE,
}

impl AsRef<str> for SymptomSymptomTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SymptomSymptomTypeEnum::SYMPTOMTYPEUNSPECIFIED => "SYMPTOM_TYPE_UNSPECIFIED",
            SymptomSymptomTypeEnum::LOWMEMORY => "LOW_MEMORY",
            SymptomSymptomTypeEnum::OUTOFMEMORY => "OUT_OF_MEMORY",
            SymptomSymptomTypeEnum::EXECUTETIMEDOUT => "EXECUTE_TIMED_OUT",
            SymptomSymptomTypeEnum::MESHBUILDFAIL => "MESH_BUILD_FAIL",
            SymptomSymptomTypeEnum::HBMOUTOFMEMORY => "HBM_OUT_OF_MEMORY",
            SymptomSymptomTypeEnum::PROJECTABUSE => "PROJECT_ABUSE",
        }
    }
}

impl std::convert::TryFrom< &str> for SymptomSymptomTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SYMPTOM_TYPE_UNSPECIFIED" => Ok(SymptomSymptomTypeEnum::SYMPTOMTYPEUNSPECIFIED),
           "LOW_MEMORY" => Ok(SymptomSymptomTypeEnum::LOWMEMORY),
           "OUT_OF_MEMORY" => Ok(SymptomSymptomTypeEnum::OUTOFMEMORY),
           "EXECUTE_TIMED_OUT" => Ok(SymptomSymptomTypeEnum::EXECUTETIMEDOUT),
           "MESH_BUILD_FAIL" => Ok(SymptomSymptomTypeEnum::MESHBUILDFAIL),
           "HBM_OUT_OF_MEMORY" => Ok(SymptomSymptomTypeEnum::HBMOUTOFMEMORY),
           "PROJECT_ABUSE" => Ok(SymptomSymptomTypeEnum::PROJECTABUSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SymptomSymptomTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


