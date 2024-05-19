use super::*;



// region BatchStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the batch.
pub enum BatchStateEnum {
    

    /// The batch state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The batch is created before running.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The batch is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The batch is cancelling.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The batch cancellation was successful.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The batch completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The batch is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for BatchStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BatchStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            BatchStateEnum::PENDING => "PENDING",
            BatchStateEnum::RUNNING => "RUNNING",
            BatchStateEnum::CANCELLING => "CANCELLING",
            BatchStateEnum::CANCELLED => "CANCELLED",
            BatchStateEnum::SUCCEEDED => "SUCCEEDED",
            BatchStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for BatchStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(BatchStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(BatchStateEnum::PENDING),
           "RUNNING" => Ok(BatchStateEnum::RUNNING),
           "CANCELLING" => Ok(BatchStateEnum::CANCELLING),
           "CANCELLED" => Ok(BatchStateEnum::CANCELLED),
           "SUCCEEDED" => Ok(BatchStateEnum::SUCCEEDED),
           "FAILED" => Ok(BatchStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BatchStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The cluster's state.
pub enum ClusterStatusStateEnum {
    

    /// The cluster state is unknown.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The cluster is being created and set up. It is not ready for use.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The cluster is currently running and healthy. It is ready for use.Note: The cluster state changes from "creating" to "running" status after the master node(s), first two primary worker nodes (and the last primary worker node if primary workers > 2) are running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The cluster encountered an error. It is not ready for use.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The cluster has encountered an error while being updated. Jobs can be submitted to the cluster, but the cluster cannot be updated.
    ///
    /// "ERROR_DUE_TO_UPDATE"
    #[serde(rename="ERROR_DUE_TO_UPDATE")]
    ERRORDUETOUPDATE,
    

    /// The cluster is being deleted. It cannot be used.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The cluster is being updated. It continues to accept and process jobs.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The cluster is being stopped. It cannot be used.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The cluster is currently stopped. It is not ready for use.
    ///
    /// "STOPPED"
    #[serde(rename="STOPPED")]
    STOPPED,
    

    /// The cluster is being started. It is not ready for use.
    ///
    /// "STARTING"
    #[serde(rename="STARTING")]
    STARTING,
    

    /// The cluster is being repaired. It is not ready for use.
    ///
    /// "REPAIRING"
    #[serde(rename="REPAIRING")]
    REPAIRING,
}

impl AsRef<str> for ClusterStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterStatusStateEnum::UNKNOWN => "UNKNOWN",
            ClusterStatusStateEnum::CREATING => "CREATING",
            ClusterStatusStateEnum::RUNNING => "RUNNING",
            ClusterStatusStateEnum::ERROR => "ERROR",
            ClusterStatusStateEnum::ERRORDUETOUPDATE => "ERROR_DUE_TO_UPDATE",
            ClusterStatusStateEnum::DELETING => "DELETING",
            ClusterStatusStateEnum::UPDATING => "UPDATING",
            ClusterStatusStateEnum::STOPPING => "STOPPING",
            ClusterStatusStateEnum::STOPPED => "STOPPED",
            ClusterStatusStateEnum::STARTING => "STARTING",
            ClusterStatusStateEnum::REPAIRING => "REPAIRING",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(ClusterStatusStateEnum::UNKNOWN),
           "CREATING" => Ok(ClusterStatusStateEnum::CREATING),
           "RUNNING" => Ok(ClusterStatusStateEnum::RUNNING),
           "ERROR" => Ok(ClusterStatusStateEnum::ERROR),
           "ERROR_DUE_TO_UPDATE" => Ok(ClusterStatusStateEnum::ERRORDUETOUPDATE),
           "DELETING" => Ok(ClusterStatusStateEnum::DELETING),
           "UPDATING" => Ok(ClusterStatusStateEnum::UPDATING),
           "STOPPING" => Ok(ClusterStatusStateEnum::STOPPING),
           "STOPPED" => Ok(ClusterStatusStateEnum::STOPPED),
           "STARTING" => Ok(ClusterStatusStateEnum::STARTING),
           "REPAIRING" => Ok(ClusterStatusStateEnum::REPAIRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterStatusSubstateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Additional state information that includes status reported by the agent.
pub enum ClusterStatusSubstateEnum {
    

    /// The cluster substate is unknown.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// The cluster is known to be in an unhealthy state (for example, critical daemons are not running or HDFS capacity is exhausted).Applies to RUNNING state.
    ///
    /// "UNHEALTHY"
    #[serde(rename="UNHEALTHY")]
    UNHEALTHY,
    

    /// The agent-reported status is out of date (may occur if Dataproc loses communication with Agent).Applies to RUNNING state.
    ///
    /// "STALE_STATUS"
    #[serde(rename="STALE_STATUS")]
    STALESTATUS,
}

impl AsRef<str> for ClusterStatusSubstateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterStatusSubstateEnum::UNSPECIFIED => "UNSPECIFIED",
            ClusterStatusSubstateEnum::UNHEALTHY => "UNHEALTHY",
            ClusterStatusSubstateEnum::STALESTATUS => "STALE_STATUS",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterStatusSubstateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(ClusterStatusSubstateEnum::UNSPECIFIED),
           "UNHEALTHY" => Ok(ClusterStatusSubstateEnum::UNHEALTHY),
           "STALE_STATUS" => Ok(ClusterStatusSubstateEnum::STALESTATUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterStatusSubstateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DiagnoseClusterRequestTarballAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. (Optional) The access type to the diagnostic tarball. If not specified, falls back to default access of the bucket
pub enum DiagnoseClusterRequestTarballAccessEnum {
    

    /// Tarball Access unspecified. Falls back to default access of the bucket
    ///
    /// "TARBALL_ACCESS_UNSPECIFIED"
    #[serde(rename="TARBALL_ACCESS_UNSPECIFIED")]
    TARBALLACCESSUNSPECIFIED,
    

    /// Google Cloud Support group has read access to the diagnostic tarball
    ///
    /// "GOOGLE_CLOUD_SUPPORT"
    #[serde(rename="GOOGLE_CLOUD_SUPPORT")]
    GOOGLECLOUDSUPPORT,
    

    /// Google Cloud Dataproc Diagnose service account has read access to the diagnostic tarball
    ///
    /// "GOOGLE_DATAPROC_DIAGNOSE"
    #[serde(rename="GOOGLE_DATAPROC_DIAGNOSE")]
    GOOGLEDATAPROCDIAGNOSE,
}

impl AsRef<str> for DiagnoseClusterRequestTarballAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DiagnoseClusterRequestTarballAccessEnum::TARBALLACCESSUNSPECIFIED => "TARBALL_ACCESS_UNSPECIFIED",
            DiagnoseClusterRequestTarballAccessEnum::GOOGLECLOUDSUPPORT => "GOOGLE_CLOUD_SUPPORT",
            DiagnoseClusterRequestTarballAccessEnum::GOOGLEDATAPROCDIAGNOSE => "GOOGLE_DATAPROC_DIAGNOSE",
        }
    }
}

impl std::convert::TryFrom< &str> for DiagnoseClusterRequestTarballAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TARBALL_ACCESS_UNSPECIFIED" => Ok(DiagnoseClusterRequestTarballAccessEnum::TARBALLACCESSUNSPECIFIED),
           "GOOGLE_CLOUD_SUPPORT" => Ok(DiagnoseClusterRequestTarballAccessEnum::GOOGLECLOUDSUPPORT),
           "GOOGLE_DATAPROC_DIAGNOSE" => Ok(DiagnoseClusterRequestTarballAccessEnum::GOOGLEDATAPROCDIAGNOSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DiagnoseClusterRequestTarballAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GceClusterConfigPrivateIpv6GoogleAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of IPv6 access for a cluster.
pub enum GceClusterConfigPrivateIpv6GoogleAccessEnum {
    

    /// If unspecified, Compute Engine default behavior will apply, which is the same as INHERIT_FROM_SUBNETWORK.
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
    PRIVATEIPV6GOOGLEACCESSUNSPECIFIED,
    

    /// Private access to and from Google Services configuration inherited from the subnetwork configuration. This is the default Compute Engine behavior.
    ///
    /// "INHERIT_FROM_SUBNETWORK"
    #[serde(rename="INHERIT_FROM_SUBNETWORK")]
    INHERITFROMSUBNETWORK,
    

    /// Enables outbound private IPv6 access to Google Services from the Dataproc cluster.
    ///
    /// "OUTBOUND"
    #[serde(rename="OUTBOUND")]
    OUTBOUND,
    

    /// Enables bidirectional private IPv6 access between Google Services and the Dataproc cluster.
    ///
    /// "BIDIRECTIONAL"
    #[serde(rename="BIDIRECTIONAL")]
    BIDIRECTIONAL,
}

impl AsRef<str> for GceClusterConfigPrivateIpv6GoogleAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GceClusterConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSUNSPECIFIED => "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED",
            GceClusterConfigPrivateIpv6GoogleAccessEnum::INHERITFROMSUBNETWORK => "INHERIT_FROM_SUBNETWORK",
            GceClusterConfigPrivateIpv6GoogleAccessEnum::OUTBOUND => "OUTBOUND",
            GceClusterConfigPrivateIpv6GoogleAccessEnum::BIDIRECTIONAL => "BIDIRECTIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for GceClusterConfigPrivateIpv6GoogleAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED" => Ok(GceClusterConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSUNSPECIFIED),
           "INHERIT_FROM_SUBNETWORK" => Ok(GceClusterConfigPrivateIpv6GoogleAccessEnum::INHERITFROMSUBNETWORK),
           "OUTBOUND" => Ok(GceClusterConfigPrivateIpv6GoogleAccessEnum::OUTBOUND),
           "BIDIRECTIONAL" => Ok(GceClusterConfigPrivateIpv6GoogleAccessEnum::BIDIRECTIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GceClusterConfigPrivateIpv6GoogleAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GkeNodePoolTargetRolesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The roles associated with the GKE node pool.
pub enum GkeNodePoolTargetRolesEnum {
    

    /// Role is unspecified.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// At least one node pool must have the DEFAULT role. Work assigned to a role that is not associated with a node pool is assigned to the node pool with the DEFAULT role. For example, work assigned to the CONTROLLER role will be assigned to the node pool with the DEFAULT role if no node pool has the CONTROLLER role.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Run work associated with the Dataproc control plane (for example, controllers and webhooks). Very low resource requirements.
    ///
    /// "CONTROLLER"
    #[serde(rename="CONTROLLER")]
    CONTROLLER,
    

    /// Run work associated with a Spark driver of a job.
    ///
    /// "SPARK_DRIVER"
    #[serde(rename="SPARK_DRIVER")]
    SPARKDRIVER,
    

    /// Run work associated with a Spark executor of a job.
    ///
    /// "SPARK_EXECUTOR"
    #[serde(rename="SPARK_EXECUTOR")]
    SPARKEXECUTOR,
}

impl AsRef<str> for GkeNodePoolTargetRolesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GkeNodePoolTargetRolesEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            GkeNodePoolTargetRolesEnum::DEFAULT => "DEFAULT",
            GkeNodePoolTargetRolesEnum::CONTROLLER => "CONTROLLER",
            GkeNodePoolTargetRolesEnum::SPARKDRIVER => "SPARK_DRIVER",
            GkeNodePoolTargetRolesEnum::SPARKEXECUTOR => "SPARK_EXECUTOR",
        }
    }
}

impl std::convert::TryFrom< &str> for GkeNodePoolTargetRolesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(GkeNodePoolTargetRolesEnum::ROLEUNSPECIFIED),
           "DEFAULT" => Ok(GkeNodePoolTargetRolesEnum::DEFAULT),
           "CONTROLLER" => Ok(GkeNodePoolTargetRolesEnum::CONTROLLER),
           "SPARK_DRIVER" => Ok(GkeNodePoolTargetRolesEnum::SPARKDRIVER),
           "SPARK_EXECUTOR" => Ok(GkeNodePoolTargetRolesEnum::SPARKEXECUTOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GkeNodePoolTargetRolesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceGroupConfigPreemptibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies the preemptibility of the instance group.The default value for master and worker groups is NON_PREEMPTIBLE. This default cannot be changed.The default value for secondary instances is PREEMPTIBLE.
pub enum InstanceGroupConfigPreemptibilityEnum {
    

    /// Preemptibility is unspecified, the system will choose the appropriate setting for each instance group.
    ///
    /// "PREEMPTIBILITY_UNSPECIFIED"
    #[serde(rename="PREEMPTIBILITY_UNSPECIFIED")]
    PREEMPTIBILITYUNSPECIFIED,
    

    /// Instances are non-preemptible.This option is allowed for all instance groups and is the only valid value for Master and Worker instance groups.
    ///
    /// "NON_PREEMPTIBLE"
    #[serde(rename="NON_PREEMPTIBLE")]
    NONPREEMPTIBLE,
    

    /// Instances are preemptible (https://cloud.google.com/compute/docs/instances/preemptible).This option is allowed only for secondary worker (https://cloud.google.com/dataproc/docs/concepts/compute/secondary-vms) groups.
    ///
    /// "PREEMPTIBLE"
    #[serde(rename="PREEMPTIBLE")]
    PREEMPTIBLE,
    

    /// Instances are Spot VMs (https://cloud.google.com/compute/docs/instances/spot).This option is allowed only for secondary worker (https://cloud.google.com/dataproc/docs/concepts/compute/secondary-vms) groups. Spot VMs are the latest version of preemptible VMs (https://cloud.google.com/compute/docs/instances/preemptible), and provide additional features.
    ///
    /// "SPOT"
    #[serde(rename="SPOT")]
    SPOT,
}

impl AsRef<str> for InstanceGroupConfigPreemptibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceGroupConfigPreemptibilityEnum::PREEMPTIBILITYUNSPECIFIED => "PREEMPTIBILITY_UNSPECIFIED",
            InstanceGroupConfigPreemptibilityEnum::NONPREEMPTIBLE => "NON_PREEMPTIBLE",
            InstanceGroupConfigPreemptibilityEnum::PREEMPTIBLE => "PREEMPTIBLE",
            InstanceGroupConfigPreemptibilityEnum::SPOT => "SPOT",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceGroupConfigPreemptibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PREEMPTIBILITY_UNSPECIFIED" => Ok(InstanceGroupConfigPreemptibilityEnum::PREEMPTIBILITYUNSPECIFIED),
           "NON_PREEMPTIBLE" => Ok(InstanceGroupConfigPreemptibilityEnum::NONPREEMPTIBLE),
           "PREEMPTIBLE" => Ok(InstanceGroupConfigPreemptibilityEnum::PREEMPTIBLE),
           "SPOT" => Ok(InstanceGroupConfigPreemptibilityEnum::SPOT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceGroupConfigPreemptibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A state message specifying the overall job state.
pub enum JobStatusStateEnum {
    

    /// The job state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job is pending; it has been submitted, but is not yet running.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Job has been received by the service and completed initial setup; it will soon be submitted to the cluster.
    ///
    /// "SETUP_DONE"
    #[serde(rename="SETUP_DONE")]
    SETUPDONE,
    

    /// The job is running on the cluster.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// A CancelJob request has been received, but is pending.
    ///
    /// "CANCEL_PENDING"
    #[serde(rename="CANCEL_PENDING")]
    CANCELPENDING,
    

    /// Transient in-flight resources have been canceled, and the request to cancel the running job has been issued to the cluster.
    ///
    /// "CANCEL_STARTED"
    #[serde(rename="CANCEL_STARTED")]
    CANCELSTARTED,
    

    /// The job cancellation was successful.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The job has completed successfully.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    

    /// The job has completed, but encountered an error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Job attempt has failed. The detail field contains failure details for this attempt.Applies to restartable jobs only.
    ///
    /// "ATTEMPT_FAILURE"
    #[serde(rename="ATTEMPT_FAILURE")]
    ATTEMPTFAILURE,
}

impl AsRef<str> for JobStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            JobStatusStateEnum::PENDING => "PENDING",
            JobStatusStateEnum::SETUPDONE => "SETUP_DONE",
            JobStatusStateEnum::RUNNING => "RUNNING",
            JobStatusStateEnum::CANCELPENDING => "CANCEL_PENDING",
            JobStatusStateEnum::CANCELSTARTED => "CANCEL_STARTED",
            JobStatusStateEnum::CANCELLED => "CANCELLED",
            JobStatusStateEnum::DONE => "DONE",
            JobStatusStateEnum::ERROR => "ERROR",
            JobStatusStateEnum::ATTEMPTFAILURE => "ATTEMPT_FAILURE",
        }
    }
}

impl std::convert::TryFrom< &str> for JobStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(JobStatusStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(JobStatusStateEnum::PENDING),
           "SETUP_DONE" => Ok(JobStatusStateEnum::SETUPDONE),
           "RUNNING" => Ok(JobStatusStateEnum::RUNNING),
           "CANCEL_PENDING" => Ok(JobStatusStateEnum::CANCELPENDING),
           "CANCEL_STARTED" => Ok(JobStatusStateEnum::CANCELSTARTED),
           "CANCELLED" => Ok(JobStatusStateEnum::CANCELLED),
           "DONE" => Ok(JobStatusStateEnum::DONE),
           "ERROR" => Ok(JobStatusStateEnum::ERROR),
           "ATTEMPT_FAILURE" => Ok(JobStatusStateEnum::ATTEMPTFAILURE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JobStatusSubstateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Additional state information, which includes status reported by the agent.
pub enum JobStatusSubstateEnum {
    

    /// The job substate is unknown.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// The Job is submitted to the agent.Applies to RUNNING state.
    ///
    /// "SUBMITTED"
    #[serde(rename="SUBMITTED")]
    SUBMITTED,
    

    /// The Job has been received and is awaiting execution (it might be waiting for a condition to be met). See the "details" field for the reason for the delay.Applies to RUNNING state.
    ///
    /// "QUEUED"
    #[serde(rename="QUEUED")]
    QUEUED,
    

    /// The agent-reported status is out of date, which can be caused by a loss of communication between the agent and Dataproc. If the agent does not send a timely update, the job will fail.Applies to RUNNING state.
    ///
    /// "STALE_STATUS"
    #[serde(rename="STALE_STATUS")]
    STALESTATUS,
}

impl AsRef<str> for JobStatusSubstateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JobStatusSubstateEnum::UNSPECIFIED => "UNSPECIFIED",
            JobStatusSubstateEnum::SUBMITTED => "SUBMITTED",
            JobStatusSubstateEnum::QUEUED => "QUEUED",
            JobStatusSubstateEnum::STALESTATUS => "STALE_STATUS",
        }
    }
}

impl std::convert::TryFrom< &str> for JobStatusSubstateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(JobStatusSubstateEnum::UNSPECIFIED),
           "SUBMITTED" => Ok(JobStatusSubstateEnum::SUBMITTED),
           "QUEUED" => Ok(JobStatusSubstateEnum::QUEUED),
           "STALE_STATUS" => Ok(JobStatusSubstateEnum::STALESTATUS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JobStatusSubstateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region JupyterConfigKernelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Kernel
pub enum JupyterConfigKernelEnum {
    

    /// The kernel is unknown.
    ///
    /// "KERNEL_UNSPECIFIED"
    #[serde(rename="KERNEL_UNSPECIFIED")]
    KERNELUNSPECIFIED,
    

    /// Python kernel.
    ///
    /// "PYTHON"
    #[serde(rename="PYTHON")]
    PYTHON,
    

    /// Scala kernel.
    ///
    /// "SCALA"
    #[serde(rename="SCALA")]
    SCALA,
}

impl AsRef<str> for JupyterConfigKernelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            JupyterConfigKernelEnum::KERNELUNSPECIFIED => "KERNEL_UNSPECIFIED",
            JupyterConfigKernelEnum::PYTHON => "PYTHON",
            JupyterConfigKernelEnum::SCALA => "SCALA",
        }
    }
}

impl std::convert::TryFrom< &str> for JupyterConfigKernelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KERNEL_UNSPECIFIED" => Ok(JupyterConfigKernelEnum::KERNELUNSPECIFIED),
           "PYTHON" => Ok(JupyterConfigKernelEnum::PYTHON),
           "SCALA" => Ok(JupyterConfigKernelEnum::SCALA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a JupyterConfigKernelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoggingConfigDriverLogLevelsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The per-package log levels for the driver. This can include "root" package name to configure rootLogger. Examples: - 'com.google = FATAL' - 'root = INFO' - 'org.apache = DEBUG'
pub enum LoggingConfigDriverLogLevelsEnum {
    

    /// Level is unspecified. Use default level for log4j.
    ///
    /// "LEVEL_UNSPECIFIED"
    #[serde(rename="LEVEL_UNSPECIFIED")]
    LEVELUNSPECIFIED,
    

    /// Use ALL level for log4j.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Use TRACE level for log4j.
    ///
    /// "TRACE"
    #[serde(rename="TRACE")]
    TRACE,
    

    /// Use DEBUG level for log4j.
    ///
    /// "DEBUG"
    #[serde(rename="DEBUG")]
    DEBUG,
    

    /// Use INFO level for log4j.
    ///
    /// "INFO"
    #[serde(rename="INFO")]
    INFO,
    

    /// Use WARN level for log4j.
    ///
    /// "WARN"
    #[serde(rename="WARN")]
    WARN,
    

    /// Use ERROR level for log4j.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Use FATAL level for log4j.
    ///
    /// "FATAL"
    #[serde(rename="FATAL")]
    FATAL,
    

    /// Turn off log4j.
    ///
    /// "OFF"
    #[serde(rename="OFF")]
    OFF,
}

impl AsRef<str> for LoggingConfigDriverLogLevelsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoggingConfigDriverLogLevelsEnum::LEVELUNSPECIFIED => "LEVEL_UNSPECIFIED",
            LoggingConfigDriverLogLevelsEnum::ALL => "ALL",
            LoggingConfigDriverLogLevelsEnum::TRACE => "TRACE",
            LoggingConfigDriverLogLevelsEnum::DEBUG => "DEBUG",
            LoggingConfigDriverLogLevelsEnum::INFO => "INFO",
            LoggingConfigDriverLogLevelsEnum::WARN => "WARN",
            LoggingConfigDriverLogLevelsEnum::ERROR => "ERROR",
            LoggingConfigDriverLogLevelsEnum::FATAL => "FATAL",
            LoggingConfigDriverLogLevelsEnum::OFF => "OFF",
        }
    }
}

impl std::convert::TryFrom< &str> for LoggingConfigDriverLogLevelsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LEVEL_UNSPECIFIED" => Ok(LoggingConfigDriverLogLevelsEnum::LEVELUNSPECIFIED),
           "ALL" => Ok(LoggingConfigDriverLogLevelsEnum::ALL),
           "TRACE" => Ok(LoggingConfigDriverLogLevelsEnum::TRACE),
           "DEBUG" => Ok(LoggingConfigDriverLogLevelsEnum::DEBUG),
           "INFO" => Ok(LoggingConfigDriverLogLevelsEnum::INFO),
           "WARN" => Ok(LoggingConfigDriverLogLevelsEnum::WARN),
           "ERROR" => Ok(LoggingConfigDriverLogLevelsEnum::ERROR),
           "FATAL" => Ok(LoggingConfigDriverLogLevelsEnum::FATAL),
           "OFF" => Ok(LoggingConfigDriverLogLevelsEnum::OFF),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoggingConfigDriverLogLevelsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricMetricSourceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. A standard set of metrics is collected unless metricOverrides are specified for the metric source (see Custom metrics (https://cloud.google.com/dataproc/docs/guides/dataproc-metrics#custom_metrics) for more information).
pub enum MetricMetricSourceEnum {
    

    /// Required unspecified metric source.
    ///
    /// "METRIC_SOURCE_UNSPECIFIED"
    #[serde(rename="METRIC_SOURCE_UNSPECIFIED")]
    METRICSOURCEUNSPECIFIED,
    

    /// Monitoring agent metrics. If this source is enabled, Dataproc enables the monitoring agent in Compute Engine, and collects monitoring agent metrics, which are published with an agent.googleapis.com prefix.
    ///
    /// "MONITORING_AGENT_DEFAULTS"
    #[serde(rename="MONITORING_AGENT_DEFAULTS")]
    MONITORINGAGENTDEFAULTS,
    

    /// HDFS metric source.
    ///
    /// "HDFS"
    #[serde(rename="HDFS")]
    HDFS,
    

    /// Spark metric source.
    ///
    /// "SPARK"
    #[serde(rename="SPARK")]
    SPARK,
    

    /// YARN metric source.
    ///
    /// "YARN"
    #[serde(rename="YARN")]
    YARN,
    

    /// Spark History Server metric source.
    ///
    /// "SPARK_HISTORY_SERVER"
    #[serde(rename="SPARK_HISTORY_SERVER")]
    SPARKHISTORYSERVER,
    

    /// Hiveserver2 metric source.
    ///
    /// "HIVESERVER2"
    #[serde(rename="HIVESERVER2")]
    HIVESERVER2,
    

    /// hivemetastore metric source
    ///
    /// "HIVEMETASTORE"
    #[serde(rename="HIVEMETASTORE")]
    HIVEMETASTORE,
    

    /// flink metric source
    ///
    /// "FLINK"
    #[serde(rename="FLINK")]
    FLINK,
}

impl AsRef<str> for MetricMetricSourceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricMetricSourceEnum::METRICSOURCEUNSPECIFIED => "METRIC_SOURCE_UNSPECIFIED",
            MetricMetricSourceEnum::MONITORINGAGENTDEFAULTS => "MONITORING_AGENT_DEFAULTS",
            MetricMetricSourceEnum::HDFS => "HDFS",
            MetricMetricSourceEnum::SPARK => "SPARK",
            MetricMetricSourceEnum::YARN => "YARN",
            MetricMetricSourceEnum::SPARKHISTORYSERVER => "SPARK_HISTORY_SERVER",
            MetricMetricSourceEnum::HIVESERVER2 => "HIVESERVER2",
            MetricMetricSourceEnum::HIVEMETASTORE => "HIVEMETASTORE",
            MetricMetricSourceEnum::FLINK => "FLINK",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricMetricSourceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_SOURCE_UNSPECIFIED" => Ok(MetricMetricSourceEnum::METRICSOURCEUNSPECIFIED),
           "MONITORING_AGENT_DEFAULTS" => Ok(MetricMetricSourceEnum::MONITORINGAGENTDEFAULTS),
           "HDFS" => Ok(MetricMetricSourceEnum::HDFS),
           "SPARK" => Ok(MetricMetricSourceEnum::SPARK),
           "YARN" => Ok(MetricMetricSourceEnum::YARN),
           "SPARK_HISTORY_SERVER" => Ok(MetricMetricSourceEnum::SPARKHISTORYSERVER),
           "HIVESERVER2" => Ok(MetricMetricSourceEnum::HIVESERVER2),
           "HIVEMETASTORE" => Ok(MetricMetricSourceEnum::HIVEMETASTORE),
           "FLINK" => Ok(MetricMetricSourceEnum::FLINK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricMetricSourceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeGroupRolesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Node group roles.
pub enum NodeGroupRolesEnum {
    

    /// Required unspecified role.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// Job drivers run on the node pool.
    ///
    /// "DRIVER"
    #[serde(rename="DRIVER")]
    DRIVER,
}

impl AsRef<str> for NodeGroupRolesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeGroupRolesEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            NodeGroupRolesEnum::DRIVER => "DRIVER",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeGroupRolesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(NodeGroupRolesEnum::ROLEUNSPECIFIED),
           "DRIVER" => Ok(NodeGroupRolesEnum::DRIVER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeGroupRolesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodePoolRepairActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Repair action to take on specified resources of the node pool.
pub enum NodePoolRepairActionEnum {
    

    /// No action will be taken by default.
    ///
    /// "REPAIR_ACTION_UNSPECIFIED"
    #[serde(rename="REPAIR_ACTION_UNSPECIFIED")]
    REPAIRACTIONUNSPECIFIED,
    

    /// delete the specified list of nodes.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for NodePoolRepairActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodePoolRepairActionEnum::REPAIRACTIONUNSPECIFIED => "REPAIR_ACTION_UNSPECIFIED",
            NodePoolRepairActionEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for NodePoolRepairActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPAIR_ACTION_UNSPECIFIED" => Ok(NodePoolRepairActionEnum::REPAIRACTIONUNSPECIFIED),
           "DELETE" => Ok(NodePoolRepairActionEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodePoolRepairActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RepairNodeGroupRequestRepairActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Repair action to take on specified resources of the node pool.
pub enum RepairNodeGroupRequestRepairActionEnum {
    

    /// No action will be taken by default.
    ///
    /// "REPAIR_ACTION_UNSPECIFIED"
    #[serde(rename="REPAIR_ACTION_UNSPECIFIED")]
    REPAIRACTIONUNSPECIFIED,
    

    /// replace the specified list of nodes.
    ///
    /// "REPLACE"
    #[serde(rename="REPLACE")]
    REPLACE,
}

impl AsRef<str> for RepairNodeGroupRequestRepairActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RepairNodeGroupRequestRepairActionEnum::REPAIRACTIONUNSPECIFIED => "REPAIR_ACTION_UNSPECIFIED",
            RepairNodeGroupRequestRepairActionEnum::REPLACE => "REPLACE",
        }
    }
}

impl std::convert::TryFrom< &str> for RepairNodeGroupRequestRepairActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPAIR_ACTION_UNSPECIFIED" => Ok(RepairNodeGroupRequestRepairActionEnum::REPAIRACTIONUNSPECIFIED),
           "REPLACE" => Ok(RepairNodeGroupRequestRepairActionEnum::REPLACE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RepairNodeGroupRequestRepairActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReservationAffinityConsumeReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of reservation to consume
pub enum ReservationAffinityConsumeReservationTypeEnum {
    
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


// region SessionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. A state of the session.
pub enum SessionStateEnum {
    

    /// The session state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The session is created prior to running.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The session is running.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The session is terminating.
    ///
    /// "TERMINATING"
    #[serde(rename="TERMINATING")]
    TERMINATING,
    

    /// The session is terminated successfully.
    ///
    /// "TERMINATED"
    #[serde(rename="TERMINATED")]
    TERMINATED,
    

    /// The session is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for SessionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SessionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SessionStateEnum::CREATING => "CREATING",
            SessionStateEnum::ACTIVE => "ACTIVE",
            SessionStateEnum::TERMINATING => "TERMINATING",
            SessionStateEnum::TERMINATED => "TERMINATED",
            SessionStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for SessionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SessionStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(SessionStateEnum::CREATING),
           "ACTIVE" => Ok(SessionStateEnum::ACTIVE),
           "TERMINATING" => Ok(SessionStateEnum::TERMINATING),
           "TERMINATED" => Ok(SessionStateEnum::TERMINATED),
           "FAILED" => Ok(SessionStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SessionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SessionStateHistoryStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the session at this point in the session history.
pub enum SessionStateHistoryStateEnum {
    

    /// The session state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The session is created prior to running.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The session is running.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The session is terminating.
    ///
    /// "TERMINATING"
    #[serde(rename="TERMINATING")]
    TERMINATING,
    

    /// The session is terminated successfully.
    ///
    /// "TERMINATED"
    #[serde(rename="TERMINATED")]
    TERMINATED,
    

    /// The session is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for SessionStateHistoryStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SessionStateHistoryStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            SessionStateHistoryStateEnum::CREATING => "CREATING",
            SessionStateHistoryStateEnum::ACTIVE => "ACTIVE",
            SessionStateHistoryStateEnum::TERMINATING => "TERMINATING",
            SessionStateHistoryStateEnum::TERMINATED => "TERMINATED",
            SessionStateHistoryStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for SessionStateHistoryStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(SessionStateHistoryStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(SessionStateHistoryStateEnum::CREATING),
           "ACTIVE" => Ok(SessionStateHistoryStateEnum::ACTIVE),
           "TERMINATING" => Ok(SessionStateHistoryStateEnum::TERMINATING),
           "TERMINATED" => Ok(SessionStateHistoryStateEnum::TERMINATED),
           "FAILED" => Ok(SessionStateHistoryStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SessionStateHistoryStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SoftwareConfigOptionalComponentsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The set of components to activate on the cluster.
pub enum SoftwareConfigOptionalComponentsEnum {
    

    /// Unspecified component. Specifying this will cause Cluster creation to fail.
    ///
    /// "COMPONENT_UNSPECIFIED"
    #[serde(rename="COMPONENT_UNSPECIFIED")]
    COMPONENTUNSPECIFIED,
    

    /// The Anaconda python distribution. The Anaconda component is not supported in the Dataproc 2.0 image. The 2.0 image is pre-installed with Miniconda.
    ///
    /// "ANACONDA"
    #[serde(rename="ANACONDA")]
    ANACONDA,
    

    /// Docker
    ///
    /// "DOCKER"
    #[serde(rename="DOCKER")]
    DOCKER,
    

    /// The Druid query engine. (alpha)
    ///
    /// "DRUID"
    #[serde(rename="DRUID")]
    DRUID,
    

    /// Flink
    ///
    /// "FLINK"
    #[serde(rename="FLINK")]
    FLINK,
    

    /// HBase. (beta)
    ///
    /// "HBASE"
    #[serde(rename="HBASE")]
    HBASE,
    

    /// The Hive Web HCatalog (the REST service for accessing HCatalog).
    ///
    /// "HIVE_WEBHCAT"
    #[serde(rename="HIVE_WEBHCAT")]
    HIVEWEBHCAT,
    

    /// Hudi.
    ///
    /// "HUDI"
    #[serde(rename="HUDI")]
    HUDI,
    

    /// The Jupyter Notebook.
    ///
    /// "JUPYTER"
    #[serde(rename="JUPYTER")]
    JUPYTER,
    

    /// The Presto query engine.
    ///
    /// "PRESTO"
    #[serde(rename="PRESTO")]
    PRESTO,
    

    /// The Trino query engine.
    ///
    /// "TRINO"
    #[serde(rename="TRINO")]
    TRINO,
    

    /// The Ranger service.
    ///
    /// "RANGER"
    #[serde(rename="RANGER")]
    RANGER,
    

    /// The Solr service.
    ///
    /// "SOLR"
    #[serde(rename="SOLR")]
    SOLR,
    

    /// The Zeppelin notebook.
    ///
    /// "ZEPPELIN"
    #[serde(rename="ZEPPELIN")]
    ZEPPELIN,
    

    /// The Zookeeper service.
    ///
    /// "ZOOKEEPER"
    #[serde(rename="ZOOKEEPER")]
    ZOOKEEPER,
}

impl AsRef<str> for SoftwareConfigOptionalComponentsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SoftwareConfigOptionalComponentsEnum::COMPONENTUNSPECIFIED => "COMPONENT_UNSPECIFIED",
            SoftwareConfigOptionalComponentsEnum::ANACONDA => "ANACONDA",
            SoftwareConfigOptionalComponentsEnum::DOCKER => "DOCKER",
            SoftwareConfigOptionalComponentsEnum::DRUID => "DRUID",
            SoftwareConfigOptionalComponentsEnum::FLINK => "FLINK",
            SoftwareConfigOptionalComponentsEnum::HBASE => "HBASE",
            SoftwareConfigOptionalComponentsEnum::HIVEWEBHCAT => "HIVE_WEBHCAT",
            SoftwareConfigOptionalComponentsEnum::HUDI => "HUDI",
            SoftwareConfigOptionalComponentsEnum::JUPYTER => "JUPYTER",
            SoftwareConfigOptionalComponentsEnum::PRESTO => "PRESTO",
            SoftwareConfigOptionalComponentsEnum::TRINO => "TRINO",
            SoftwareConfigOptionalComponentsEnum::RANGER => "RANGER",
            SoftwareConfigOptionalComponentsEnum::SOLR => "SOLR",
            SoftwareConfigOptionalComponentsEnum::ZEPPELIN => "ZEPPELIN",
            SoftwareConfigOptionalComponentsEnum::ZOOKEEPER => "ZOOKEEPER",
        }
    }
}

impl std::convert::TryFrom< &str> for SoftwareConfigOptionalComponentsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPONENT_UNSPECIFIED" => Ok(SoftwareConfigOptionalComponentsEnum::COMPONENTUNSPECIFIED),
           "ANACONDA" => Ok(SoftwareConfigOptionalComponentsEnum::ANACONDA),
           "DOCKER" => Ok(SoftwareConfigOptionalComponentsEnum::DOCKER),
           "DRUID" => Ok(SoftwareConfigOptionalComponentsEnum::DRUID),
           "FLINK" => Ok(SoftwareConfigOptionalComponentsEnum::FLINK),
           "HBASE" => Ok(SoftwareConfigOptionalComponentsEnum::HBASE),
           "HIVE_WEBHCAT" => Ok(SoftwareConfigOptionalComponentsEnum::HIVEWEBHCAT),
           "HUDI" => Ok(SoftwareConfigOptionalComponentsEnum::HUDI),
           "JUPYTER" => Ok(SoftwareConfigOptionalComponentsEnum::JUPYTER),
           "PRESTO" => Ok(SoftwareConfigOptionalComponentsEnum::PRESTO),
           "TRINO" => Ok(SoftwareConfigOptionalComponentsEnum::TRINO),
           "RANGER" => Ok(SoftwareConfigOptionalComponentsEnum::RANGER),
           "SOLR" => Ok(SoftwareConfigOptionalComponentsEnum::SOLR),
           "ZEPPELIN" => Ok(SoftwareConfigOptionalComponentsEnum::ZEPPELIN),
           "ZOOKEEPER" => Ok(SoftwareConfigOptionalComponentsEnum::ZOOKEEPER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SoftwareConfigOptionalComponentsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StateHistoryStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the batch at this point in history.
pub enum StateHistoryStateEnum {
    

    /// The batch state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The batch is created before running.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The batch is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The batch is cancelling.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The batch cancellation was successful.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The batch completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The batch is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for StateHistoryStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StateHistoryStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            StateHistoryStateEnum::PENDING => "PENDING",
            StateHistoryStateEnum::RUNNING => "RUNNING",
            StateHistoryStateEnum::CANCELLING => "CANCELLING",
            StateHistoryStateEnum::CANCELLED => "CANCELLED",
            StateHistoryStateEnum::SUCCEEDED => "SUCCEEDED",
            StateHistoryStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for StateHistoryStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(StateHistoryStateEnum::STATEUNSPECIFIED),
           "PENDING" => Ok(StateHistoryStateEnum::PENDING),
           "RUNNING" => Ok(StateHistoryStateEnum::RUNNING),
           "CANCELLING" => Ok(StateHistoryStateEnum::CANCELLING),
           "CANCELLED" => Ok(StateHistoryStateEnum::CANCELLED),
           "SUCCEEDED" => Ok(StateHistoryStateEnum::SUCCEEDED),
           "FAILED" => Ok(StateHistoryStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StateHistoryStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region YarnApplicationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The application state.
pub enum YarnApplicationStateEnum {
    

    /// Status is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Status is NEW.
    ///
    /// "NEW"
    #[serde(rename="NEW")]
    NEW,
    

    /// Status is NEW_SAVING.
    ///
    /// "NEW_SAVING"
    #[serde(rename="NEW_SAVING")]
    NEWSAVING,
    

    /// Status is SUBMITTED.
    ///
    /// "SUBMITTED"
    #[serde(rename="SUBMITTED")]
    SUBMITTED,
    

    /// Status is ACCEPTED.
    ///
    /// "ACCEPTED"
    #[serde(rename="ACCEPTED")]
    ACCEPTED,
    

    /// Status is RUNNING.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Status is FINISHED.
    ///
    /// "FINISHED"
    #[serde(rename="FINISHED")]
    FINISHED,
    

    /// Status is FAILED.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Status is KILLED.
    ///
    /// "KILLED"
    #[serde(rename="KILLED")]
    KILLED,
}

impl AsRef<str> for YarnApplicationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            YarnApplicationStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            YarnApplicationStateEnum::NEW => "NEW",
            YarnApplicationStateEnum::NEWSAVING => "NEW_SAVING",
            YarnApplicationStateEnum::SUBMITTED => "SUBMITTED",
            YarnApplicationStateEnum::ACCEPTED => "ACCEPTED",
            YarnApplicationStateEnum::RUNNING => "RUNNING",
            YarnApplicationStateEnum::FINISHED => "FINISHED",
            YarnApplicationStateEnum::FAILED => "FAILED",
            YarnApplicationStateEnum::KILLED => "KILLED",
        }
    }
}

impl std::convert::TryFrom< &str> for YarnApplicationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(YarnApplicationStateEnum::STATEUNSPECIFIED),
           "NEW" => Ok(YarnApplicationStateEnum::NEW),
           "NEW_SAVING" => Ok(YarnApplicationStateEnum::NEWSAVING),
           "SUBMITTED" => Ok(YarnApplicationStateEnum::SUBMITTED),
           "ACCEPTED" => Ok(YarnApplicationStateEnum::ACCEPTED),
           "RUNNING" => Ok(YarnApplicationStateEnum::RUNNING),
           "FINISHED" => Ok(YarnApplicationStateEnum::FINISHED),
           "FAILED" => Ok(YarnApplicationStateEnum::FAILED),
           "KILLED" => Ok(YarnApplicationStateEnum::KILLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a YarnApplicationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectActionOnFailedPrimaryWorkersEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Failure action when primary worker creation fails.
pub enum ProjectActionOnFailedPrimaryWorkersEnum {
    

    /// When FailureAction is unspecified, failure action defaults to NO_ACTION.
    ///
    /// "FAILURE_ACTION_UNSPECIFIED"
    #[serde(rename="FAILURE_ACTION_UNSPECIFIED")]
    FAILUREACTIONUNSPECIFIED,
    

    /// Take no action on failure to create a cluster resource. NO_ACTION is the default.
    ///
    /// "NO_ACTION"
    #[serde(rename="NO_ACTION")]
    NOACTION,
    

    /// Delete the failed cluster resource.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for ProjectActionOnFailedPrimaryWorkersEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectActionOnFailedPrimaryWorkersEnum::FAILUREACTIONUNSPECIFIED => "FAILURE_ACTION_UNSPECIFIED",
            ProjectActionOnFailedPrimaryWorkersEnum::NOACTION => "NO_ACTION",
            ProjectActionOnFailedPrimaryWorkersEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectActionOnFailedPrimaryWorkersEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FAILURE_ACTION_UNSPECIFIED" => Ok(ProjectActionOnFailedPrimaryWorkersEnum::FAILUREACTIONUNSPECIFIED),
           "NO_ACTION" => Ok(ProjectActionOnFailedPrimaryWorkersEnum::NOACTION),
           "DELETE" => Ok(ProjectActionOnFailedPrimaryWorkersEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectActionOnFailedPrimaryWorkersEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectJobStateMatcherEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies enumerated categories of jobs to list. (default = match ALL jobs).If filter is provided, jobStateMatcher will be ignored.
pub enum ProjectJobStateMatcherEnum {
    

    /// Match all jobs, regardless of state.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Only match jobs in non-terminal states: PENDING, RUNNING, or CANCEL_PENDING.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Only match jobs in terminal states: CANCELLED, DONE, or ERROR.
    ///
    /// "NON_ACTIVE"
    #[serde(rename="NON_ACTIVE")]
    NONACTIVE,
}

impl AsRef<str> for ProjectJobStateMatcherEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectJobStateMatcherEnum::ALL => "ALL",
            ProjectJobStateMatcherEnum::ACTIVE => "ACTIVE",
            ProjectJobStateMatcherEnum::NONACTIVE => "NON_ACTIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectJobStateMatcherEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(ProjectJobStateMatcherEnum::ALL),
           "ACTIVE" => Ok(ProjectJobStateMatcherEnum::ACTIVE),
           "NON_ACTIVE" => Ok(ProjectJobStateMatcherEnum::NONACTIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectJobStateMatcherEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


