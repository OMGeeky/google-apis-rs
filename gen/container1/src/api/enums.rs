use super::*;



// region AdvancedDatapathObservabilityConfigRelayModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Method used to make Relay available
pub enum AdvancedDatapathObservabilityConfigRelayModeEnum {
    

    /// Default value. This shouldn't be used.
    ///
    /// "RELAY_MODE_UNSPECIFIED"
    #[serde(rename="RELAY_MODE_UNSPECIFIED")]
    RELAYMODEUNSPECIFIED,
    

    /// disabled
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// exposed via internal load balancer
    ///
    /// "INTERNAL_VPC_LB"
    #[serde(rename="INTERNAL_VPC_LB")]
    INTERNALVPCLB,
    

    /// exposed via external load balancer
    ///
    /// "EXTERNAL_LB"
    #[serde(rename="EXTERNAL_LB")]
    EXTERNALLB,
}

impl AsRef<str> for AdvancedDatapathObservabilityConfigRelayModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AdvancedDatapathObservabilityConfigRelayModeEnum::RELAYMODEUNSPECIFIED => "RELAY_MODE_UNSPECIFIED",
            AdvancedDatapathObservabilityConfigRelayModeEnum::DISABLED => "DISABLED",
            AdvancedDatapathObservabilityConfigRelayModeEnum::INTERNALVPCLB => "INTERNAL_VPC_LB",
            AdvancedDatapathObservabilityConfigRelayModeEnum::EXTERNALLB => "EXTERNAL_LB",
        }
    }
}

impl std::convert::TryFrom< &str> for AdvancedDatapathObservabilityConfigRelayModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RELAY_MODE_UNSPECIFIED" => Ok(AdvancedDatapathObservabilityConfigRelayModeEnum::RELAYMODEUNSPECIFIED),
           "DISABLED" => Ok(AdvancedDatapathObservabilityConfigRelayModeEnum::DISABLED),
           "INTERNAL_VPC_LB" => Ok(AdvancedDatapathObservabilityConfigRelayModeEnum::INTERNALVPCLB),
           "EXTERNAL_LB" => Ok(AdvancedDatapathObservabilityConfigRelayModeEnum::EXTERNALLB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AdvancedDatapathObservabilityConfigRelayModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AutopilotCompatibilityIssueIncompatibilityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The incompatibility type of this issue.
pub enum AutopilotCompatibilityIssueIncompatibilityTypeEnum {
    

    /// Default value, should not be used.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Indicates that the issue is a known incompatibility between the cluster and Autopilot mode.
    ///
    /// "INCOMPATIBILITY"
    #[serde(rename="INCOMPATIBILITY")]
    INCOMPATIBILITY,
    

    /// Indicates the issue is an incompatibility if customers take no further action to resolve.
    ///
    /// "ADDITIONAL_CONFIG_REQUIRED"
    #[serde(rename="ADDITIONAL_CONFIG_REQUIRED")]
    ADDITIONALCONFIGREQUIRED,
    

    /// Indicates the issue is not an incompatibility, but depending on the workloads business logic, there is a potential that they won't work on Autopilot.
    ///
    /// "PASSED_WITH_OPTIONAL_CONFIG"
    #[serde(rename="PASSED_WITH_OPTIONAL_CONFIG")]
    PASSEDWITHOPTIONALCONFIG,
}

impl AsRef<str> for AutopilotCompatibilityIssueIncompatibilityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AutopilotCompatibilityIssueIncompatibilityTypeEnum::UNSPECIFIED => "UNSPECIFIED",
            AutopilotCompatibilityIssueIncompatibilityTypeEnum::INCOMPATIBILITY => "INCOMPATIBILITY",
            AutopilotCompatibilityIssueIncompatibilityTypeEnum::ADDITIONALCONFIGREQUIRED => "ADDITIONAL_CONFIG_REQUIRED",
            AutopilotCompatibilityIssueIncompatibilityTypeEnum::PASSEDWITHOPTIONALCONFIG => "PASSED_WITH_OPTIONAL_CONFIG",
        }
    }
}

impl std::convert::TryFrom< &str> for AutopilotCompatibilityIssueIncompatibilityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(AutopilotCompatibilityIssueIncompatibilityTypeEnum::UNSPECIFIED),
           "INCOMPATIBILITY" => Ok(AutopilotCompatibilityIssueIncompatibilityTypeEnum::INCOMPATIBILITY),
           "ADDITIONAL_CONFIG_REQUIRED" => Ok(AutopilotCompatibilityIssueIncompatibilityTypeEnum::ADDITIONALCONFIGREQUIRED),
           "PASSED_WITH_OPTIONAL_CONFIG" => Ok(AutopilotCompatibilityIssueIncompatibilityTypeEnum::PASSEDWITHOPTIONALCONFIG),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AutopilotCompatibilityIssueIncompatibilityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BinaryAuthorizationEvaluationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode of operation for binauthz policy evaluation. If unspecified, defaults to DISABLED.
pub enum BinaryAuthorizationEvaluationModeEnum {
    

    /// Default value
    ///
    /// "EVALUATION_MODE_UNSPECIFIED"
    #[serde(rename="EVALUATION_MODE_UNSPECIFIED")]
    EVALUATIONMODEUNSPECIFIED,
    

    /// Disable BinaryAuthorization
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Enforce Kubernetes admission requests with BinaryAuthorization using the project's singleton policy. This is equivalent to setting the enabled boolean to true.
    ///
    /// "PROJECT_SINGLETON_POLICY_ENFORCE"
    #[serde(rename="PROJECT_SINGLETON_POLICY_ENFORCE")]
    PROJECTSINGLETONPOLICYENFORCE,
}

impl AsRef<str> for BinaryAuthorizationEvaluationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BinaryAuthorizationEvaluationModeEnum::EVALUATIONMODEUNSPECIFIED => "EVALUATION_MODE_UNSPECIFIED",
            BinaryAuthorizationEvaluationModeEnum::DISABLED => "DISABLED",
            BinaryAuthorizationEvaluationModeEnum::PROJECTSINGLETONPOLICYENFORCE => "PROJECT_SINGLETON_POLICY_ENFORCE",
        }
    }
}

impl std::convert::TryFrom< &str> for BinaryAuthorizationEvaluationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVALUATION_MODE_UNSPECIFIED" => Ok(BinaryAuthorizationEvaluationModeEnum::EVALUATIONMODEUNSPECIFIED),
           "DISABLED" => Ok(BinaryAuthorizationEvaluationModeEnum::DISABLED),
           "PROJECT_SINGLETON_POLICY_ENFORCE" => Ok(BinaryAuthorizationEvaluationModeEnum::PROJECTSINGLETONPOLICYENFORCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BinaryAuthorizationEvaluationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlueGreenInfoPhaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current blue-green upgrade phase.
pub enum BlueGreenInfoPhaseEnum {
    

    /// Unspecified phase.
    ///
    /// "PHASE_UNSPECIFIED"
    #[serde(rename="PHASE_UNSPECIFIED")]
    PHASEUNSPECIFIED,
    

    /// blue-green upgrade has been initiated.
    ///
    /// "UPDATE_STARTED"
    #[serde(rename="UPDATE_STARTED")]
    UPDATESTARTED,
    

    /// Start creating green pool nodes.
    ///
    /// "CREATING_GREEN_POOL"
    #[serde(rename="CREATING_GREEN_POOL")]
    CREATINGGREENPOOL,
    

    /// Start cordoning blue pool nodes.
    ///
    /// "CORDONING_BLUE_POOL"
    #[serde(rename="CORDONING_BLUE_POOL")]
    CORDONINGBLUEPOOL,
    

    /// Start draining blue pool nodes.
    ///
    /// "DRAINING_BLUE_POOL"
    #[serde(rename="DRAINING_BLUE_POOL")]
    DRAININGBLUEPOOL,
    

    /// Start soaking time after draining entire blue pool.
    ///
    /// "NODE_POOL_SOAKING"
    #[serde(rename="NODE_POOL_SOAKING")]
    NODEPOOLSOAKING,
    

    /// Start deleting blue nodes.
    ///
    /// "DELETING_BLUE_POOL"
    #[serde(rename="DELETING_BLUE_POOL")]
    DELETINGBLUEPOOL,
    

    /// Rollback has been initiated.
    ///
    /// "ROLLBACK_STARTED"
    #[serde(rename="ROLLBACK_STARTED")]
    ROLLBACKSTARTED,
}

impl AsRef<str> for BlueGreenInfoPhaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlueGreenInfoPhaseEnum::PHASEUNSPECIFIED => "PHASE_UNSPECIFIED",
            BlueGreenInfoPhaseEnum::UPDATESTARTED => "UPDATE_STARTED",
            BlueGreenInfoPhaseEnum::CREATINGGREENPOOL => "CREATING_GREEN_POOL",
            BlueGreenInfoPhaseEnum::CORDONINGBLUEPOOL => "CORDONING_BLUE_POOL",
            BlueGreenInfoPhaseEnum::DRAININGBLUEPOOL => "DRAINING_BLUE_POOL",
            BlueGreenInfoPhaseEnum::NODEPOOLSOAKING => "NODE_POOL_SOAKING",
            BlueGreenInfoPhaseEnum::DELETINGBLUEPOOL => "DELETING_BLUE_POOL",
            BlueGreenInfoPhaseEnum::ROLLBACKSTARTED => "ROLLBACK_STARTED",
        }
    }
}

impl std::convert::TryFrom< &str> for BlueGreenInfoPhaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PHASE_UNSPECIFIED" => Ok(BlueGreenInfoPhaseEnum::PHASEUNSPECIFIED),
           "UPDATE_STARTED" => Ok(BlueGreenInfoPhaseEnum::UPDATESTARTED),
           "CREATING_GREEN_POOL" => Ok(BlueGreenInfoPhaseEnum::CREATINGGREENPOOL),
           "CORDONING_BLUE_POOL" => Ok(BlueGreenInfoPhaseEnum::CORDONINGBLUEPOOL),
           "DRAINING_BLUE_POOL" => Ok(BlueGreenInfoPhaseEnum::DRAININGBLUEPOOL),
           "NODE_POOL_SOAKING" => Ok(BlueGreenInfoPhaseEnum::NODEPOOLSOAKING),
           "DELETING_BLUE_POOL" => Ok(BlueGreenInfoPhaseEnum::DELETINGBLUEPOOL),
           "ROLLBACK_STARTED" => Ok(BlueGreenInfoPhaseEnum::ROLLBACKSTARTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlueGreenInfoPhaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudRunConfigLoadBalancerTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Which load balancer type is installed for Cloud Run.
pub enum CloudRunConfigLoadBalancerTypeEnum {
    

    /// Load balancer type for Cloud Run is unspecified.
    ///
    /// "LOAD_BALANCER_TYPE_UNSPECIFIED"
    #[serde(rename="LOAD_BALANCER_TYPE_UNSPECIFIED")]
    LOADBALANCERTYPEUNSPECIFIED,
    

    /// Install external load balancer for Cloud Run.
    ///
    /// "LOAD_BALANCER_TYPE_EXTERNAL"
    #[serde(rename="LOAD_BALANCER_TYPE_EXTERNAL")]
    LOADBALANCERTYPEEXTERNAL,
    

    /// Install internal load balancer for Cloud Run.
    ///
    /// "LOAD_BALANCER_TYPE_INTERNAL"
    #[serde(rename="LOAD_BALANCER_TYPE_INTERNAL")]
    LOADBALANCERTYPEINTERNAL,
}

impl AsRef<str> for CloudRunConfigLoadBalancerTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudRunConfigLoadBalancerTypeEnum::LOADBALANCERTYPEUNSPECIFIED => "LOAD_BALANCER_TYPE_UNSPECIFIED",
            CloudRunConfigLoadBalancerTypeEnum::LOADBALANCERTYPEEXTERNAL => "LOAD_BALANCER_TYPE_EXTERNAL",
            CloudRunConfigLoadBalancerTypeEnum::LOADBALANCERTYPEINTERNAL => "LOAD_BALANCER_TYPE_INTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudRunConfigLoadBalancerTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOAD_BALANCER_TYPE_UNSPECIFIED" => Ok(CloudRunConfigLoadBalancerTypeEnum::LOADBALANCERTYPEUNSPECIFIED),
           "LOAD_BALANCER_TYPE_EXTERNAL" => Ok(CloudRunConfigLoadBalancerTypeEnum::LOADBALANCERTYPEEXTERNAL),
           "LOAD_BALANCER_TYPE_INTERNAL" => Ok(CloudRunConfigLoadBalancerTypeEnum::LOADBALANCERTYPEINTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudRunConfigLoadBalancerTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output only] The current status of this cluster.
pub enum ClusterStatusEnum {
    

    /// Not set.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The PROVISIONING state indicates the cluster is being created.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The RUNNING state indicates the cluster has been created and is fully usable.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The RECONCILING state indicates that some work is actively being done on the cluster, such as upgrading the master or node software. Details can be found in the `statusMessage` field.
    ///
    /// "RECONCILING"
    #[serde(rename="RECONCILING")]
    RECONCILING,
    

    /// The STOPPING state indicates the cluster is being deleted.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The ERROR state indicates the cluster is unusable. It will be automatically deleted. Details can be found in the `statusMessage` field.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// The DEGRADED state indicates the cluster requires user action to restore full functionality. Details can be found in the `statusMessage` field.
    ///
    /// "DEGRADED"
    #[serde(rename="DEGRADED")]
    DEGRADED,
}

impl AsRef<str> for ClusterStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            ClusterStatusEnum::PROVISIONING => "PROVISIONING",
            ClusterStatusEnum::RUNNING => "RUNNING",
            ClusterStatusEnum::RECONCILING => "RECONCILING",
            ClusterStatusEnum::STOPPING => "STOPPING",
            ClusterStatusEnum::ERROR => "ERROR",
            ClusterStatusEnum::DEGRADED => "DEGRADED",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(ClusterStatusEnum::STATUSUNSPECIFIED),
           "PROVISIONING" => Ok(ClusterStatusEnum::PROVISIONING),
           "RUNNING" => Ok(ClusterStatusEnum::RUNNING),
           "RECONCILING" => Ok(ClusterStatusEnum::RECONCILING),
           "STOPPING" => Ok(ClusterStatusEnum::STOPPING),
           "ERROR" => Ok(ClusterStatusEnum::ERROR),
           "DEGRADED" => Ok(ClusterStatusEnum::DEGRADED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterAutoscalingAutoscalingProfileEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines autoscaling behaviour.
pub enum ClusterAutoscalingAutoscalingProfileEnum {
    

    /// No change to autoscaling configuration.
    ///
    /// "PROFILE_UNSPECIFIED"
    #[serde(rename="PROFILE_UNSPECIFIED")]
    PROFILEUNSPECIFIED,
    

    /// Prioritize optimizing utilization of resources.
    ///
    /// "OPTIMIZE_UTILIZATION"
    #[serde(rename="OPTIMIZE_UTILIZATION")]
    OPTIMIZEUTILIZATION,
    

    /// Use default (balanced) autoscaling configuration.
    ///
    /// "BALANCED"
    #[serde(rename="BALANCED")]
    BALANCED,
}

impl AsRef<str> for ClusterAutoscalingAutoscalingProfileEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterAutoscalingAutoscalingProfileEnum::PROFILEUNSPECIFIED => "PROFILE_UNSPECIFIED",
            ClusterAutoscalingAutoscalingProfileEnum::OPTIMIZEUTILIZATION => "OPTIMIZE_UTILIZATION",
            ClusterAutoscalingAutoscalingProfileEnum::BALANCED => "BALANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterAutoscalingAutoscalingProfileEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_UNSPECIFIED" => Ok(ClusterAutoscalingAutoscalingProfileEnum::PROFILEUNSPECIFIED),
           "OPTIMIZE_UTILIZATION" => Ok(ClusterAutoscalingAutoscalingProfileEnum::OPTIMIZEUTILIZATION),
           "BALANCED" => Ok(ClusterAutoscalingAutoscalingProfileEnum::BALANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterAutoscalingAutoscalingProfileEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the total network bandwidth tier for NodePools in the cluster.
pub enum ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    

    /// Default value
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// Higher bandwidth, actual values based on VM size.
    ///
    /// "TIER_1"
    #[serde(rename="TIER_1")]
    TIER1,
}

impl AsRef<str> for ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIER1 => "TIER_1",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIERUNSPECIFIED),
           "TIER_1" => Ok(ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIER1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterNetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterUpdateDesiredDatapathProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired datapath provider for the cluster.
pub enum ClusterUpdateDesiredDatapathProviderEnum {
    

    /// Default value.
    ///
    /// "DATAPATH_PROVIDER_UNSPECIFIED"
    #[serde(rename="DATAPATH_PROVIDER_UNSPECIFIED")]
    DATAPATHPROVIDERUNSPECIFIED,
    

    /// Use the IPTables implementation based on kube-proxy.
    ///
    /// "LEGACY_DATAPATH"
    #[serde(rename="LEGACY_DATAPATH")]
    LEGACYDATAPATH,
    

    /// Use the eBPF based GKE Dataplane V2 with additional features. See the [GKE Dataplane V2 documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/dataplane-v2) for more.
    ///
    /// "ADVANCED_DATAPATH"
    #[serde(rename="ADVANCED_DATAPATH")]
    ADVANCEDDATAPATH,
}

impl AsRef<str> for ClusterUpdateDesiredDatapathProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterUpdateDesiredDatapathProviderEnum::DATAPATHPROVIDERUNSPECIFIED => "DATAPATH_PROVIDER_UNSPECIFIED",
            ClusterUpdateDesiredDatapathProviderEnum::LEGACYDATAPATH => "LEGACY_DATAPATH",
            ClusterUpdateDesiredDatapathProviderEnum::ADVANCEDDATAPATH => "ADVANCED_DATAPATH",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterUpdateDesiredDatapathProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATAPATH_PROVIDER_UNSPECIFIED" => Ok(ClusterUpdateDesiredDatapathProviderEnum::DATAPATHPROVIDERUNSPECIFIED),
           "LEGACY_DATAPATH" => Ok(ClusterUpdateDesiredDatapathProviderEnum::LEGACYDATAPATH),
           "ADVANCED_DATAPATH" => Ok(ClusterUpdateDesiredDatapathProviderEnum::ADVANCEDDATAPATH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterUpdateDesiredDatapathProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterUpdateDesiredInTransitEncryptionConfigEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specify the details of in-transit encryption.
pub enum ClusterUpdateDesiredInTransitEncryptionConfigEnum {
    

    /// Unspecified, will be inferred as default - IN_TRANSIT_ENCRYPTION_UNSPECIFIED.
    ///
    /// "IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED"
    #[serde(rename="IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED")]
    INTRANSITENCRYPTIONCONFIGUNSPECIFIED,
    

    /// In-transit encryption is disabled.
    ///
    /// "IN_TRANSIT_ENCRYPTION_DISABLED"
    #[serde(rename="IN_TRANSIT_ENCRYPTION_DISABLED")]
    INTRANSITENCRYPTIONDISABLED,
    

    /// Data in-transit is encrypted using inter-node transparent encryption.
    ///
    /// "IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT"
    #[serde(rename="IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT")]
    INTRANSITENCRYPTIONINTERNODETRANSPARENT,
}

impl AsRef<str> for ClusterUpdateDesiredInTransitEncryptionConfigEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterUpdateDesiredInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONCONFIGUNSPECIFIED => "IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED",
            ClusterUpdateDesiredInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONDISABLED => "IN_TRANSIT_ENCRYPTION_DISABLED",
            ClusterUpdateDesiredInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONINTERNODETRANSPARENT => "IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterUpdateDesiredInTransitEncryptionConfigEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED" => Ok(ClusterUpdateDesiredInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONCONFIGUNSPECIFIED),
           "IN_TRANSIT_ENCRYPTION_DISABLED" => Ok(ClusterUpdateDesiredInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONDISABLED),
           "IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT" => Ok(ClusterUpdateDesiredInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONINTERNODETRANSPARENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterUpdateDesiredInTransitEncryptionConfigEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired state of IPv6 connectivity to Google Services.
pub enum ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
    

    /// Default value. Same as DISABLED
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
    PRIVATEIPV6GOOGLEACCESSUNSPECIFIED,
    

    /// No private access to or from Google Services
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED")]
    PRIVATEIPV6GOOGLEACCESSDISABLED,
    

    /// Enables private IPv6 access to Google Services from GKE
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE")]
    PRIVATEIPV6GOOGLEACCESSTOGOOGLE,
    

    /// Enables private IPv6 access to and from Google Services
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL")]
    PRIVATEIPV6GOOGLEACCESSBIDIRECTIONAL,
}

impl AsRef<str> for ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSUNSPECIFIED => "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED",
            ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSDISABLED => "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED",
            ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSTOGOOGLE => "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE",
            ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSBIDIRECTIONAL => "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED" => Ok(ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSUNSPECIFIED),
           "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED" => Ok(ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSDISABLED),
           "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE" => Ok(ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSTOGOOGLE),
           "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL" => Ok(ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSBIDIRECTIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ClusterUpdateDesiredStackTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired stack type of the cluster. If a stack type is provided and does not match the current stack type of the cluster, update will attempt to change the stack type to the new type.
pub enum ClusterUpdateDesiredStackTypeEnum {
    

    /// Default value, will be defaulted as IPV4 only
    ///
    /// "STACK_TYPE_UNSPECIFIED"
    #[serde(rename="STACK_TYPE_UNSPECIFIED")]
    STACKTYPEUNSPECIFIED,
    

    /// Cluster is IPV4 only
    ///
    /// "IPV4"
    #[serde(rename="IPV4")]
    IPV4,
    

    /// Cluster can use both IPv4 and IPv6
    ///
    /// "IPV4_IPV6"
    #[serde(rename="IPV4_IPV6")]
    IPV4IPV6,
}

impl AsRef<str> for ClusterUpdateDesiredStackTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ClusterUpdateDesiredStackTypeEnum::STACKTYPEUNSPECIFIED => "STACK_TYPE_UNSPECIFIED",
            ClusterUpdateDesiredStackTypeEnum::IPV4 => "IPV4",
            ClusterUpdateDesiredStackTypeEnum::IPV4IPV6 => "IPV4_IPV6",
        }
    }
}

impl std::convert::TryFrom< &str> for ClusterUpdateDesiredStackTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STACK_TYPE_UNSPECIFIED" => Ok(ClusterUpdateDesiredStackTypeEnum::STACKTYPEUNSPECIFIED),
           "IPV4" => Ok(ClusterUpdateDesiredStackTypeEnum::IPV4),
           "IPV4_IPV6" => Ok(ClusterUpdateDesiredStackTypeEnum::IPV4IPV6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ClusterUpdateDesiredStackTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DNSConfigClusterDnsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// cluster_dns indicates which in-cluster DNS provider should be used.
pub enum DNSConfigClusterDnsEnum {
    

    /// Default value
    ///
    /// "PROVIDER_UNSPECIFIED"
    #[serde(rename="PROVIDER_UNSPECIFIED")]
    PROVIDERUNSPECIFIED,
    

    /// Use GKE default DNS provider(kube-dns) for DNS resolution.
    ///
    /// "PLATFORM_DEFAULT"
    #[serde(rename="PLATFORM_DEFAULT")]
    PLATFORMDEFAULT,
    

    /// Use CloudDNS for DNS resolution.
    ///
    /// "CLOUD_DNS"
    #[serde(rename="CLOUD_DNS")]
    CLOUDDNS,
    

    /// Use KubeDNS for DNS resolution.
    ///
    /// "KUBE_DNS"
    #[serde(rename="KUBE_DNS")]
    KUBEDNS,
}

impl AsRef<str> for DNSConfigClusterDnsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DNSConfigClusterDnsEnum::PROVIDERUNSPECIFIED => "PROVIDER_UNSPECIFIED",
            DNSConfigClusterDnsEnum::PLATFORMDEFAULT => "PLATFORM_DEFAULT",
            DNSConfigClusterDnsEnum::CLOUDDNS => "CLOUD_DNS",
            DNSConfigClusterDnsEnum::KUBEDNS => "KUBE_DNS",
        }
    }
}

impl std::convert::TryFrom< &str> for DNSConfigClusterDnsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROVIDER_UNSPECIFIED" => Ok(DNSConfigClusterDnsEnum::PROVIDERUNSPECIFIED),
           "PLATFORM_DEFAULT" => Ok(DNSConfigClusterDnsEnum::PLATFORMDEFAULT),
           "CLOUD_DNS" => Ok(DNSConfigClusterDnsEnum::CLOUDDNS),
           "KUBE_DNS" => Ok(DNSConfigClusterDnsEnum::KUBEDNS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DNSConfigClusterDnsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DNSConfigClusterDnsScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// cluster_dns_scope indicates the scope of access to cluster DNS records.
pub enum DNSConfigClusterDnsScopeEnum {
    

    /// Default value, will be inferred as cluster scope.
    ///
    /// "DNS_SCOPE_UNSPECIFIED"
    #[serde(rename="DNS_SCOPE_UNSPECIFIED")]
    DNSSCOPEUNSPECIFIED,
    

    /// DNS records are accessible from within the cluster.
    ///
    /// "CLUSTER_SCOPE"
    #[serde(rename="CLUSTER_SCOPE")]
    CLUSTERSCOPE,
    

    /// DNS records are accessible from within the VPC.
    ///
    /// "VPC_SCOPE"
    #[serde(rename="VPC_SCOPE")]
    VPCSCOPE,
}

impl AsRef<str> for DNSConfigClusterDnsScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DNSConfigClusterDnsScopeEnum::DNSSCOPEUNSPECIFIED => "DNS_SCOPE_UNSPECIFIED",
            DNSConfigClusterDnsScopeEnum::CLUSTERSCOPE => "CLUSTER_SCOPE",
            DNSConfigClusterDnsScopeEnum::VPCSCOPE => "VPC_SCOPE",
        }
    }
}

impl std::convert::TryFrom< &str> for DNSConfigClusterDnsScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DNS_SCOPE_UNSPECIFIED" => Ok(DNSConfigClusterDnsScopeEnum::DNSSCOPEUNSPECIFIED),
           "CLUSTER_SCOPE" => Ok(DNSConfigClusterDnsScopeEnum::CLUSTERSCOPE),
           "VPC_SCOPE" => Ok(DNSConfigClusterDnsScopeEnum::VPCSCOPE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DNSConfigClusterDnsScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseEncryptionCurrentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of etcd encryption.
pub enum DatabaseEncryptionCurrentStateEnum {
    

    /// Should never be set
    ///
    /// "CURRENT_STATE_UNSPECIFIED"
    #[serde(rename="CURRENT_STATE_UNSPECIFIED")]
    CURRENTSTATEUNSPECIFIED,
    

    /// Secrets in etcd are encrypted.
    ///
    /// "CURRENT_STATE_ENCRYPTED"
    #[serde(rename="CURRENT_STATE_ENCRYPTED")]
    CURRENTSTATEENCRYPTED,
    

    /// Secrets in etcd are stored in plain text (at etcd level) - this is unrelated to Compute Engine level full disk encryption.
    ///
    /// "CURRENT_STATE_DECRYPTED"
    #[serde(rename="CURRENT_STATE_DECRYPTED")]
    CURRENTSTATEDECRYPTED,
    

    /// Encryption (or re-encryption with a different CloudKMS key) of Secrets is in progress.
    ///
    /// "CURRENT_STATE_ENCRYPTION_PENDING"
    #[serde(rename="CURRENT_STATE_ENCRYPTION_PENDING")]
    CURRENTSTATEENCRYPTIONPENDING,
    

    /// Encryption (or re-encryption with a different CloudKMS key) of Secrets in etcd encountered an error.
    ///
    /// "CURRENT_STATE_ENCRYPTION_ERROR"
    #[serde(rename="CURRENT_STATE_ENCRYPTION_ERROR")]
    CURRENTSTATEENCRYPTIONERROR,
    

    /// De-crypting Secrets to plain text in etcd is in progress.
    ///
    /// "CURRENT_STATE_DECRYPTION_PENDING"
    #[serde(rename="CURRENT_STATE_DECRYPTION_PENDING")]
    CURRENTSTATEDECRYPTIONPENDING,
    

    /// De-crypting Secrets to plain text in etcd encountered an error.
    ///
    /// "CURRENT_STATE_DECRYPTION_ERROR"
    #[serde(rename="CURRENT_STATE_DECRYPTION_ERROR")]
    CURRENTSTATEDECRYPTIONERROR,
}

impl AsRef<str> for DatabaseEncryptionCurrentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEUNSPECIFIED => "CURRENT_STATE_UNSPECIFIED",
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEENCRYPTED => "CURRENT_STATE_ENCRYPTED",
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEDECRYPTED => "CURRENT_STATE_DECRYPTED",
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEENCRYPTIONPENDING => "CURRENT_STATE_ENCRYPTION_PENDING",
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEENCRYPTIONERROR => "CURRENT_STATE_ENCRYPTION_ERROR",
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEDECRYPTIONPENDING => "CURRENT_STATE_DECRYPTION_PENDING",
            DatabaseEncryptionCurrentStateEnum::CURRENTSTATEDECRYPTIONERROR => "CURRENT_STATE_DECRYPTION_ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseEncryptionCurrentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CURRENT_STATE_UNSPECIFIED" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEUNSPECIFIED),
           "CURRENT_STATE_ENCRYPTED" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEENCRYPTED),
           "CURRENT_STATE_DECRYPTED" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEDECRYPTED),
           "CURRENT_STATE_ENCRYPTION_PENDING" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEENCRYPTIONPENDING),
           "CURRENT_STATE_ENCRYPTION_ERROR" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEENCRYPTIONERROR),
           "CURRENT_STATE_DECRYPTION_PENDING" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEDECRYPTIONPENDING),
           "CURRENT_STATE_DECRYPTION_ERROR" => Ok(DatabaseEncryptionCurrentStateEnum::CURRENTSTATEDECRYPTIONERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseEncryptionCurrentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DatabaseEncryptionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired state of etcd encryption.
pub enum DatabaseEncryptionStateEnum {
    

    /// Should never be set
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Secrets in etcd are encrypted.
    ///
    /// "ENCRYPTED"
    #[serde(rename="ENCRYPTED")]
    ENCRYPTED,
    

    /// Secrets in etcd are stored in plain text (at etcd level) - this is unrelated to Compute Engine level full disk encryption.
    ///
    /// "DECRYPTED"
    #[serde(rename="DECRYPTED")]
    DECRYPTED,
}

impl AsRef<str> for DatabaseEncryptionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DatabaseEncryptionStateEnum::UNKNOWN => "UNKNOWN",
            DatabaseEncryptionStateEnum::ENCRYPTED => "ENCRYPTED",
            DatabaseEncryptionStateEnum::DECRYPTED => "DECRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for DatabaseEncryptionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(DatabaseEncryptionStateEnum::UNKNOWN),
           "ENCRYPTED" => Ok(DatabaseEncryptionStateEnum::ENCRYPTED),
           "DECRYPTED" => Ok(DatabaseEncryptionStateEnum::DECRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DatabaseEncryptionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnterpriseConfigClusterTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. [Output only] cluster_tier specifies the premium tier of the cluster.
pub enum EnterpriseConfigClusterTierEnum {
    

    /// CLUSTER_TIER_UNSPECIFIED is when cluster_tier is not set.
    ///
    /// "CLUSTER_TIER_UNSPECIFIED"
    #[serde(rename="CLUSTER_TIER_UNSPECIFIED")]
    CLUSTERTIERUNSPECIFIED,
    

    /// STANDARD indicates a standard GKE cluster.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// ENTERPRISE indicates a GKE Enterprise cluster.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
}

impl AsRef<str> for EnterpriseConfigClusterTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnterpriseConfigClusterTierEnum::CLUSTERTIERUNSPECIFIED => "CLUSTER_TIER_UNSPECIFIED",
            EnterpriseConfigClusterTierEnum::STANDARD => "STANDARD",
            EnterpriseConfigClusterTierEnum::ENTERPRISE => "ENTERPRISE",
        }
    }
}

impl std::convert::TryFrom< &str> for EnterpriseConfigClusterTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLUSTER_TIER_UNSPECIFIED" => Ok(EnterpriseConfigClusterTierEnum::CLUSTERTIERUNSPECIFIED),
           "STANDARD" => Ok(EnterpriseConfigClusterTierEnum::STANDARD),
           "ENTERPRISE" => Ok(EnterpriseConfigClusterTierEnum::ENTERPRISE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnterpriseConfigClusterTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FilterEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Event types to allowlist.
pub enum FilterEventTypeEnum {
    

    /// Not set, will be ignored.
    ///
    /// "EVENT_TYPE_UNSPECIFIED"
    #[serde(rename="EVENT_TYPE_UNSPECIFIED")]
    EVENTTYPEUNSPECIFIED,
    

    /// Corresponds with UpgradeAvailableEvent.
    ///
    /// "UPGRADE_AVAILABLE_EVENT"
    #[serde(rename="UPGRADE_AVAILABLE_EVENT")]
    UPGRADEAVAILABLEEVENT,
    

    /// Corresponds with UpgradeEvent.
    ///
    /// "UPGRADE_EVENT"
    #[serde(rename="UPGRADE_EVENT")]
    UPGRADEEVENT,
    

    /// Corresponds with SecurityBulletinEvent.
    ///
    /// "SECURITY_BULLETIN_EVENT"
    #[serde(rename="SECURITY_BULLETIN_EVENT")]
    SECURITYBULLETINEVENT,
}

impl AsRef<str> for FilterEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FilterEventTypeEnum::EVENTTYPEUNSPECIFIED => "EVENT_TYPE_UNSPECIFIED",
            FilterEventTypeEnum::UPGRADEAVAILABLEEVENT => "UPGRADE_AVAILABLE_EVENT",
            FilterEventTypeEnum::UPGRADEEVENT => "UPGRADE_EVENT",
            FilterEventTypeEnum::SECURITYBULLETINEVENT => "SECURITY_BULLETIN_EVENT",
        }
    }
}

impl std::convert::TryFrom< &str> for FilterEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVENT_TYPE_UNSPECIFIED" => Ok(FilterEventTypeEnum::EVENTTYPEUNSPECIFIED),
           "UPGRADE_AVAILABLE_EVENT" => Ok(FilterEventTypeEnum::UPGRADEAVAILABLEEVENT),
           "UPGRADE_EVENT" => Ok(FilterEventTypeEnum::UPGRADEEVENT),
           "SECURITY_BULLETIN_EVENT" => Ok(FilterEventTypeEnum::SECURITYBULLETINEVENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FilterEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GPUDriverInstallationConfigGpuDriverVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode for how the GPU driver is installed.
pub enum GPUDriverInstallationConfigGpuDriverVersionEnum {
    

    /// Default value is to not install any GPU driver.
    ///
    /// "GPU_DRIVER_VERSION_UNSPECIFIED"
    #[serde(rename="GPU_DRIVER_VERSION_UNSPECIFIED")]
    GPUDRIVERVERSIONUNSPECIFIED,
    

    /// Disable GPU driver auto installation and needs manual installation
    ///
    /// "INSTALLATION_DISABLED"
    #[serde(rename="INSTALLATION_DISABLED")]
    INSTALLATIONDISABLED,
    

    /// "Default" GPU driver in COS and Ubuntu.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// "Latest" GPU driver in COS.
    ///
    /// "LATEST"
    #[serde(rename="LATEST")]
    LATEST,
}

impl AsRef<str> for GPUDriverInstallationConfigGpuDriverVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GPUDriverInstallationConfigGpuDriverVersionEnum::GPUDRIVERVERSIONUNSPECIFIED => "GPU_DRIVER_VERSION_UNSPECIFIED",
            GPUDriverInstallationConfigGpuDriverVersionEnum::INSTALLATIONDISABLED => "INSTALLATION_DISABLED",
            GPUDriverInstallationConfigGpuDriverVersionEnum::DEFAULT => "DEFAULT",
            GPUDriverInstallationConfigGpuDriverVersionEnum::LATEST => "LATEST",
        }
    }
}

impl std::convert::TryFrom< &str> for GPUDriverInstallationConfigGpuDriverVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GPU_DRIVER_VERSION_UNSPECIFIED" => Ok(GPUDriverInstallationConfigGpuDriverVersionEnum::GPUDRIVERVERSIONUNSPECIFIED),
           "INSTALLATION_DISABLED" => Ok(GPUDriverInstallationConfigGpuDriverVersionEnum::INSTALLATIONDISABLED),
           "DEFAULT" => Ok(GPUDriverInstallationConfigGpuDriverVersionEnum::DEFAULT),
           "LATEST" => Ok(GPUDriverInstallationConfigGpuDriverVersionEnum::LATEST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GPUDriverInstallationConfigGpuDriverVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GPUSharingConfigGpuSharingStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of GPU sharing strategy to enable on the GPU node.
pub enum GPUSharingConfigGpuSharingStrategyEnum {
    

    /// Default value.
    ///
    /// "GPU_SHARING_STRATEGY_UNSPECIFIED"
    #[serde(rename="GPU_SHARING_STRATEGY_UNSPECIFIED")]
    GPUSHARINGSTRATEGYUNSPECIFIED,
    

    /// GPUs are time-shared between containers.
    ///
    /// "TIME_SHARING"
    #[serde(rename="TIME_SHARING")]
    TIMESHARING,
    

    /// GPUs are shared between containers with NVIDIA MPS.
    ///
    /// "MPS"
    #[serde(rename="MPS")]
    MPS,
}

impl AsRef<str> for GPUSharingConfigGpuSharingStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GPUSharingConfigGpuSharingStrategyEnum::GPUSHARINGSTRATEGYUNSPECIFIED => "GPU_SHARING_STRATEGY_UNSPECIFIED",
            GPUSharingConfigGpuSharingStrategyEnum::TIMESHARING => "TIME_SHARING",
            GPUSharingConfigGpuSharingStrategyEnum::MPS => "MPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GPUSharingConfigGpuSharingStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GPU_SHARING_STRATEGY_UNSPECIFIED" => Ok(GPUSharingConfigGpuSharingStrategyEnum::GPUSHARINGSTRATEGYUNSPECIFIED),
           "TIME_SHARING" => Ok(GPUSharingConfigGpuSharingStrategyEnum::TIMESHARING),
           "MPS" => Ok(GPUSharingConfigGpuSharingStrategyEnum::MPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GPUSharingConfigGpuSharingStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GatewayAPIConfigChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Gateway API release channel to use for Gateway API.
pub enum GatewayAPIConfigChannelEnum {
    

    /// Default value.
    ///
    /// "CHANNEL_UNSPECIFIED"
    #[serde(rename="CHANNEL_UNSPECIFIED")]
    CHANNELUNSPECIFIED,
    

    /// Gateway API support is disabled
    ///
    /// "CHANNEL_DISABLED"
    #[serde(rename="CHANNEL_DISABLED")]
    CHANNELDISABLED,
    

    /// Gateway API support is enabled, experimental CRDs are installed
    ///
    /// "CHANNEL_EXPERIMENTAL"
    #[serde(rename="CHANNEL_EXPERIMENTAL")]
    CHANNELEXPERIMENTAL,
    

    /// Gateway API support is enabled, standard CRDs are installed
    ///
    /// "CHANNEL_STANDARD"
    #[serde(rename="CHANNEL_STANDARD")]
    CHANNELSTANDARD,
}

impl AsRef<str> for GatewayAPIConfigChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GatewayAPIConfigChannelEnum::CHANNELUNSPECIFIED => "CHANNEL_UNSPECIFIED",
            GatewayAPIConfigChannelEnum::CHANNELDISABLED => "CHANNEL_DISABLED",
            GatewayAPIConfigChannelEnum::CHANNELEXPERIMENTAL => "CHANNEL_EXPERIMENTAL",
            GatewayAPIConfigChannelEnum::CHANNELSTANDARD => "CHANNEL_STANDARD",
        }
    }
}

impl std::convert::TryFrom< &str> for GatewayAPIConfigChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CHANNEL_UNSPECIFIED" => Ok(GatewayAPIConfigChannelEnum::CHANNELUNSPECIFIED),
           "CHANNEL_DISABLED" => Ok(GatewayAPIConfigChannelEnum::CHANNELDISABLED),
           "CHANNEL_EXPERIMENTAL" => Ok(GatewayAPIConfigChannelEnum::CHANNELEXPERIMENTAL),
           "CHANNEL_STANDARD" => Ok(GatewayAPIConfigChannelEnum::CHANNELSTANDARD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GatewayAPIConfigChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IPAllocationPolicyIpv6AccessTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ipv6 access type (internal or external) when create_subnetwork is true
pub enum IPAllocationPolicyIpv6AccessTypeEnum {
    

    /// Default value, will be defaulted as type external.
    ///
    /// "IPV6_ACCESS_TYPE_UNSPECIFIED"
    #[serde(rename="IPV6_ACCESS_TYPE_UNSPECIFIED")]
    IPV6ACCESSTYPEUNSPECIFIED,
    

    /// Access type internal (all v6 addresses are internal IPs)
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    

    /// Access type external (all v6 addresses are external IPs)
    ///
    /// "EXTERNAL"
    #[serde(rename="EXTERNAL")]
    EXTERNAL,
}

impl AsRef<str> for IPAllocationPolicyIpv6AccessTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IPAllocationPolicyIpv6AccessTypeEnum::IPV6ACCESSTYPEUNSPECIFIED => "IPV6_ACCESS_TYPE_UNSPECIFIED",
            IPAllocationPolicyIpv6AccessTypeEnum::INTERNAL => "INTERNAL",
            IPAllocationPolicyIpv6AccessTypeEnum::EXTERNAL => "EXTERNAL",
        }
    }
}

impl std::convert::TryFrom< &str> for IPAllocationPolicyIpv6AccessTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IPV6_ACCESS_TYPE_UNSPECIFIED" => Ok(IPAllocationPolicyIpv6AccessTypeEnum::IPV6ACCESSTYPEUNSPECIFIED),
           "INTERNAL" => Ok(IPAllocationPolicyIpv6AccessTypeEnum::INTERNAL),
           "EXTERNAL" => Ok(IPAllocationPolicyIpv6AccessTypeEnum::EXTERNAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IPAllocationPolicyIpv6AccessTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IPAllocationPolicyStackTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The IP stack type of the cluster
pub enum IPAllocationPolicyStackTypeEnum {
    

    /// Default value, will be defaulted as IPV4 only
    ///
    /// "STACK_TYPE_UNSPECIFIED"
    #[serde(rename="STACK_TYPE_UNSPECIFIED")]
    STACKTYPEUNSPECIFIED,
    

    /// Cluster is IPV4 only
    ///
    /// "IPV4"
    #[serde(rename="IPV4")]
    IPV4,
    

    /// Cluster can use both IPv4 and IPv6
    ///
    /// "IPV4_IPV6"
    #[serde(rename="IPV4_IPV6")]
    IPV4IPV6,
}

impl AsRef<str> for IPAllocationPolicyStackTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IPAllocationPolicyStackTypeEnum::STACKTYPEUNSPECIFIED => "STACK_TYPE_UNSPECIFIED",
            IPAllocationPolicyStackTypeEnum::IPV4 => "IPV4",
            IPAllocationPolicyStackTypeEnum::IPV4IPV6 => "IPV4_IPV6",
        }
    }
}

impl std::convert::TryFrom< &str> for IPAllocationPolicyStackTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STACK_TYPE_UNSPECIFIED" => Ok(IPAllocationPolicyStackTypeEnum::STACKTYPEUNSPECIFIED),
           "IPV4" => Ok(IPAllocationPolicyStackTypeEnum::IPV4),
           "IPV4_IPV6" => Ok(IPAllocationPolicyStackTypeEnum::IPV4IPV6),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IPAllocationPolicyStackTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinuxNodeConfigCgroupModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// cgroup_mode specifies the cgroup mode to be used on the node.
pub enum LinuxNodeConfigCgroupModeEnum {
    

    /// CGROUP_MODE_UNSPECIFIED is when unspecified cgroup configuration is used. The default for the GKE node OS image will be used.
    ///
    /// "CGROUP_MODE_UNSPECIFIED"
    #[serde(rename="CGROUP_MODE_UNSPECIFIED")]
    CGROUPMODEUNSPECIFIED,
    

    /// CGROUP_MODE_V1 specifies to use cgroupv1 for the cgroup configuration on the node image.
    ///
    /// "CGROUP_MODE_V1"
    #[serde(rename="CGROUP_MODE_V1")]
    CGROUPMODEV1,
    

    /// CGROUP_MODE_V2 specifies to use cgroupv2 for the cgroup configuration on the node image.
    ///
    /// "CGROUP_MODE_V2"
    #[serde(rename="CGROUP_MODE_V2")]
    CGROUPMODEV2,
}

impl AsRef<str> for LinuxNodeConfigCgroupModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinuxNodeConfigCgroupModeEnum::CGROUPMODEUNSPECIFIED => "CGROUP_MODE_UNSPECIFIED",
            LinuxNodeConfigCgroupModeEnum::CGROUPMODEV1 => "CGROUP_MODE_V1",
            LinuxNodeConfigCgroupModeEnum::CGROUPMODEV2 => "CGROUP_MODE_V2",
        }
    }
}

impl std::convert::TryFrom< &str> for LinuxNodeConfigCgroupModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CGROUP_MODE_UNSPECIFIED" => Ok(LinuxNodeConfigCgroupModeEnum::CGROUPMODEUNSPECIFIED),
           "CGROUP_MODE_V1" => Ok(LinuxNodeConfigCgroupModeEnum::CGROUPMODEV1),
           "CGROUP_MODE_V2" => Ok(LinuxNodeConfigCgroupModeEnum::CGROUPMODEV2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinuxNodeConfigCgroupModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoggingComponentConfigEnableComponentsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select components to collect logs. An empty set would disable all logging.
pub enum LoggingComponentConfigEnableComponentsEnum {
    

    /// Default value. This shouldn't be used.
    ///
    /// "COMPONENT_UNSPECIFIED"
    #[serde(rename="COMPONENT_UNSPECIFIED")]
    COMPONENTUNSPECIFIED,
    

    /// system components
    ///
    /// "SYSTEM_COMPONENTS"
    #[serde(rename="SYSTEM_COMPONENTS")]
    SYSTEMCOMPONENTS,
    

    /// workloads
    ///
    /// "WORKLOADS"
    #[serde(rename="WORKLOADS")]
    WORKLOADS,
    

    /// kube-apiserver
    ///
    /// "APISERVER"
    #[serde(rename="APISERVER")]
    APISERVER,
    

    /// kube-scheduler
    ///
    /// "SCHEDULER"
    #[serde(rename="SCHEDULER")]
    SCHEDULER,
    

    /// kube-controller-manager
    ///
    /// "CONTROLLER_MANAGER"
    #[serde(rename="CONTROLLER_MANAGER")]
    CONTROLLERMANAGER,
}

impl AsRef<str> for LoggingComponentConfigEnableComponentsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoggingComponentConfigEnableComponentsEnum::COMPONENTUNSPECIFIED => "COMPONENT_UNSPECIFIED",
            LoggingComponentConfigEnableComponentsEnum::SYSTEMCOMPONENTS => "SYSTEM_COMPONENTS",
            LoggingComponentConfigEnableComponentsEnum::WORKLOADS => "WORKLOADS",
            LoggingComponentConfigEnableComponentsEnum::APISERVER => "APISERVER",
            LoggingComponentConfigEnableComponentsEnum::SCHEDULER => "SCHEDULER",
            LoggingComponentConfigEnableComponentsEnum::CONTROLLERMANAGER => "CONTROLLER_MANAGER",
        }
    }
}

impl std::convert::TryFrom< &str> for LoggingComponentConfigEnableComponentsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPONENT_UNSPECIFIED" => Ok(LoggingComponentConfigEnableComponentsEnum::COMPONENTUNSPECIFIED),
           "SYSTEM_COMPONENTS" => Ok(LoggingComponentConfigEnableComponentsEnum::SYSTEMCOMPONENTS),
           "WORKLOADS" => Ok(LoggingComponentConfigEnableComponentsEnum::WORKLOADS),
           "APISERVER" => Ok(LoggingComponentConfigEnableComponentsEnum::APISERVER),
           "SCHEDULER" => Ok(LoggingComponentConfigEnableComponentsEnum::SCHEDULER),
           "CONTROLLER_MANAGER" => Ok(LoggingComponentConfigEnableComponentsEnum::CONTROLLERMANAGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoggingComponentConfigEnableComponentsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LoggingVariantConfigVariantEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Logging variant deployed on nodes.
pub enum LoggingVariantConfigVariantEnum {
    

    /// Default value. This shouldn't be used.
    ///
    /// "VARIANT_UNSPECIFIED"
    #[serde(rename="VARIANT_UNSPECIFIED")]
    VARIANTUNSPECIFIED,
    

    /// default logging variant.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// maximum logging throughput variant.
    ///
    /// "MAX_THROUGHPUT"
    #[serde(rename="MAX_THROUGHPUT")]
    MAXTHROUGHPUT,
}

impl AsRef<str> for LoggingVariantConfigVariantEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LoggingVariantConfigVariantEnum::VARIANTUNSPECIFIED => "VARIANT_UNSPECIFIED",
            LoggingVariantConfigVariantEnum::DEFAULT => "DEFAULT",
            LoggingVariantConfigVariantEnum::MAXTHROUGHPUT => "MAX_THROUGHPUT",
        }
    }
}

impl std::convert::TryFrom< &str> for LoggingVariantConfigVariantEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VARIANT_UNSPECIFIED" => Ok(LoggingVariantConfigVariantEnum::VARIANTUNSPECIFIED),
           "DEFAULT" => Ok(LoggingVariantConfigVariantEnum::DEFAULT),
           "MAX_THROUGHPUT" => Ok(LoggingVariantConfigVariantEnum::MAXTHROUGHPUT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LoggingVariantConfigVariantEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MaintenanceExclusionOptionScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Scope specifies the upgrade scope which upgrades are blocked by the exclusion.
pub enum MaintenanceExclusionOptionScopeEnum {
    

    /// NO_UPGRADES excludes all upgrades, including patch upgrades and minor upgrades across control planes and nodes. This is the default exclusion behavior.
    ///
    /// "NO_UPGRADES"
    #[serde(rename="NO_UPGRADES")]
    NOUPGRADES,
    

    /// NO_MINOR_UPGRADES excludes all minor upgrades for the cluster, only patches are allowed.
    ///
    /// "NO_MINOR_UPGRADES"
    #[serde(rename="NO_MINOR_UPGRADES")]
    NOMINORUPGRADES,
    

    /// NO_MINOR_OR_NODE_UPGRADES excludes all minor upgrades for the cluster, and also exclude all node pool upgrades. Only control plane patches are allowed.
    ///
    /// "NO_MINOR_OR_NODE_UPGRADES"
    #[serde(rename="NO_MINOR_OR_NODE_UPGRADES")]
    NOMINORORNODEUPGRADES,
}

impl AsRef<str> for MaintenanceExclusionOptionScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MaintenanceExclusionOptionScopeEnum::NOUPGRADES => "NO_UPGRADES",
            MaintenanceExclusionOptionScopeEnum::NOMINORUPGRADES => "NO_MINOR_UPGRADES",
            MaintenanceExclusionOptionScopeEnum::NOMINORORNODEUPGRADES => "NO_MINOR_OR_NODE_UPGRADES",
        }
    }
}

impl std::convert::TryFrom< &str> for MaintenanceExclusionOptionScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_UPGRADES" => Ok(MaintenanceExclusionOptionScopeEnum::NOUPGRADES),
           "NO_MINOR_UPGRADES" => Ok(MaintenanceExclusionOptionScopeEnum::NOMINORUPGRADES),
           "NO_MINOR_OR_NODE_UPGRADES" => Ok(MaintenanceExclusionOptionScopeEnum::NOMINORORNODEUPGRADES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MaintenanceExclusionOptionScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MonitoringComponentConfigEnableComponentsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Select components to collect metrics. An empty set would disable all monitoring.
pub enum MonitoringComponentConfigEnableComponentsEnum {
    

    /// Default value. This shouldn't be used.
    ///
    /// "COMPONENT_UNSPECIFIED"
    #[serde(rename="COMPONENT_UNSPECIFIED")]
    COMPONENTUNSPECIFIED,
    

    /// system components
    ///
    /// "SYSTEM_COMPONENTS"
    #[serde(rename="SYSTEM_COMPONENTS")]
    SYSTEMCOMPONENTS,
    

    /// kube-apiserver
    ///
    /// "APISERVER"
    #[serde(rename="APISERVER")]
    APISERVER,
    

    /// kube-scheduler
    ///
    /// "SCHEDULER"
    #[serde(rename="SCHEDULER")]
    SCHEDULER,
    

    /// kube-controller-manager
    ///
    /// "CONTROLLER_MANAGER"
    #[serde(rename="CONTROLLER_MANAGER")]
    CONTROLLERMANAGER,
    

    /// Storage
    ///
    /// "STORAGE"
    #[serde(rename="STORAGE")]
    STORAGE,
    

    /// Horizontal Pod Autoscaling
    ///
    /// "HPA"
    #[serde(rename="HPA")]
    HPA,
    

    /// Pod
    ///
    /// "POD"
    #[serde(rename="POD")]
    POD,
    

    /// DaemonSet
    ///
    /// "DAEMONSET"
    #[serde(rename="DAEMONSET")]
    DAEMONSET,
    

    /// Deployment
    ///
    /// "DEPLOYMENT"
    #[serde(rename="DEPLOYMENT")]
    DEPLOYMENT,
    

    /// Statefulset
    ///
    /// "STATEFULSET"
    #[serde(rename="STATEFULSET")]
    STATEFULSET,
}

impl AsRef<str> for MonitoringComponentConfigEnableComponentsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MonitoringComponentConfigEnableComponentsEnum::COMPONENTUNSPECIFIED => "COMPONENT_UNSPECIFIED",
            MonitoringComponentConfigEnableComponentsEnum::SYSTEMCOMPONENTS => "SYSTEM_COMPONENTS",
            MonitoringComponentConfigEnableComponentsEnum::APISERVER => "APISERVER",
            MonitoringComponentConfigEnableComponentsEnum::SCHEDULER => "SCHEDULER",
            MonitoringComponentConfigEnableComponentsEnum::CONTROLLERMANAGER => "CONTROLLER_MANAGER",
            MonitoringComponentConfigEnableComponentsEnum::STORAGE => "STORAGE",
            MonitoringComponentConfigEnableComponentsEnum::HPA => "HPA",
            MonitoringComponentConfigEnableComponentsEnum::POD => "POD",
            MonitoringComponentConfigEnableComponentsEnum::DAEMONSET => "DAEMONSET",
            MonitoringComponentConfigEnableComponentsEnum::DEPLOYMENT => "DEPLOYMENT",
            MonitoringComponentConfigEnableComponentsEnum::STATEFULSET => "STATEFULSET",
        }
    }
}

impl std::convert::TryFrom< &str> for MonitoringComponentConfigEnableComponentsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPONENT_UNSPECIFIED" => Ok(MonitoringComponentConfigEnableComponentsEnum::COMPONENTUNSPECIFIED),
           "SYSTEM_COMPONENTS" => Ok(MonitoringComponentConfigEnableComponentsEnum::SYSTEMCOMPONENTS),
           "APISERVER" => Ok(MonitoringComponentConfigEnableComponentsEnum::APISERVER),
           "SCHEDULER" => Ok(MonitoringComponentConfigEnableComponentsEnum::SCHEDULER),
           "CONTROLLER_MANAGER" => Ok(MonitoringComponentConfigEnableComponentsEnum::CONTROLLERMANAGER),
           "STORAGE" => Ok(MonitoringComponentConfigEnableComponentsEnum::STORAGE),
           "HPA" => Ok(MonitoringComponentConfigEnableComponentsEnum::HPA),
           "POD" => Ok(MonitoringComponentConfigEnableComponentsEnum::POD),
           "DAEMONSET" => Ok(MonitoringComponentConfigEnableComponentsEnum::DAEMONSET),
           "DEPLOYMENT" => Ok(MonitoringComponentConfigEnableComponentsEnum::DEPLOYMENT),
           "STATEFULSET" => Ok(MonitoringComponentConfigEnableComponentsEnum::STATEFULSET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MonitoringComponentConfigEnableComponentsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigDatapathProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation.
pub enum NetworkConfigDatapathProviderEnum {
    

    /// Default value.
    ///
    /// "DATAPATH_PROVIDER_UNSPECIFIED"
    #[serde(rename="DATAPATH_PROVIDER_UNSPECIFIED")]
    DATAPATHPROVIDERUNSPECIFIED,
    

    /// Use the IPTables implementation based on kube-proxy.
    ///
    /// "LEGACY_DATAPATH"
    #[serde(rename="LEGACY_DATAPATH")]
    LEGACYDATAPATH,
    

    /// Use the eBPF based GKE Dataplane V2 with additional features. See the [GKE Dataplane V2 documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/dataplane-v2) for more.
    ///
    /// "ADVANCED_DATAPATH"
    #[serde(rename="ADVANCED_DATAPATH")]
    ADVANCEDDATAPATH,
}

impl AsRef<str> for NetworkConfigDatapathProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigDatapathProviderEnum::DATAPATHPROVIDERUNSPECIFIED => "DATAPATH_PROVIDER_UNSPECIFIED",
            NetworkConfigDatapathProviderEnum::LEGACYDATAPATH => "LEGACY_DATAPATH",
            NetworkConfigDatapathProviderEnum::ADVANCEDDATAPATH => "ADVANCED_DATAPATH",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigDatapathProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATAPATH_PROVIDER_UNSPECIFIED" => Ok(NetworkConfigDatapathProviderEnum::DATAPATHPROVIDERUNSPECIFIED),
           "LEGACY_DATAPATH" => Ok(NetworkConfigDatapathProviderEnum::LEGACYDATAPATH),
           "ADVANCED_DATAPATH" => Ok(NetworkConfigDatapathProviderEnum::ADVANCEDDATAPATH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigDatapathProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigInTransitEncryptionConfigEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specify the details of in-transit encryption.
pub enum NetworkConfigInTransitEncryptionConfigEnum {
    

    /// Unspecified, will be inferred as default - IN_TRANSIT_ENCRYPTION_UNSPECIFIED.
    ///
    /// "IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED"
    #[serde(rename="IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED")]
    INTRANSITENCRYPTIONCONFIGUNSPECIFIED,
    

    /// In-transit encryption is disabled.
    ///
    /// "IN_TRANSIT_ENCRYPTION_DISABLED"
    #[serde(rename="IN_TRANSIT_ENCRYPTION_DISABLED")]
    INTRANSITENCRYPTIONDISABLED,
    

    /// Data in-transit is encrypted using inter-node transparent encryption.
    ///
    /// "IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT"
    #[serde(rename="IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT")]
    INTRANSITENCRYPTIONINTERNODETRANSPARENT,
}

impl AsRef<str> for NetworkConfigInTransitEncryptionConfigEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONCONFIGUNSPECIFIED => "IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED",
            NetworkConfigInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONDISABLED => "IN_TRANSIT_ENCRYPTION_DISABLED",
            NetworkConfigInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONINTERNODETRANSPARENT => "IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigInTransitEncryptionConfigEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IN_TRANSIT_ENCRYPTION_CONFIG_UNSPECIFIED" => Ok(NetworkConfigInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONCONFIGUNSPECIFIED),
           "IN_TRANSIT_ENCRYPTION_DISABLED" => Ok(NetworkConfigInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONDISABLED),
           "IN_TRANSIT_ENCRYPTION_INTER_NODE_TRANSPARENT" => Ok(NetworkConfigInTransitEncryptionConfigEnum::INTRANSITENCRYPTIONINTERNODETRANSPARENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigInTransitEncryptionConfigEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigPrivateIpv6GoogleAccessEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)
pub enum NetworkConfigPrivateIpv6GoogleAccessEnum {
    

    /// Default value. Same as DISABLED
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
    PRIVATEIPV6GOOGLEACCESSUNSPECIFIED,
    

    /// No private access to or from Google Services
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED")]
    PRIVATEIPV6GOOGLEACCESSDISABLED,
    

    /// Enables private IPv6 access to Google Services from GKE
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE")]
    PRIVATEIPV6GOOGLEACCESSTOGOOGLE,
    

    /// Enables private IPv6 access to and from Google Services
    ///
    /// "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL"
    #[serde(rename="PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL")]
    PRIVATEIPV6GOOGLEACCESSBIDIRECTIONAL,
}

impl AsRef<str> for NetworkConfigPrivateIpv6GoogleAccessEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSUNSPECIFIED => "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED",
            NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSDISABLED => "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED",
            NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSTOGOOGLE => "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE",
            NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSBIDIRECTIONAL => "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigPrivateIpv6GoogleAccessEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED" => Ok(NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSUNSPECIFIED),
           "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED" => Ok(NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSDISABLED),
           "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE" => Ok(NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSTOGOOGLE),
           "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL" => Ok(NetworkConfigPrivateIpv6GoogleAccessEnum::PRIVATEIPV6GOOGLEACCESSBIDIRECTIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigPrivateIpv6GoogleAccessEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkPerformanceConfigTotalEgressBandwidthTierEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the total network bandwidth tier for the NodePool.
pub enum NetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    

    /// Default value
    ///
    /// "TIER_UNSPECIFIED"
    #[serde(rename="TIER_UNSPECIFIED")]
    TIERUNSPECIFIED,
    

    /// Higher bandwidth, actual values based on VM size.
    ///
    /// "TIER_1"
    #[serde(rename="TIER_1")]
    TIER1,
}

impl AsRef<str> for NetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIERUNSPECIFIED => "TIER_UNSPECIFIED",
            NetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIER1 => "TIER_1",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TIER_UNSPECIFIED" => Ok(NetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIERUNSPECIFIED),
           "TIER_1" => Ok(NetworkPerformanceConfigTotalEgressBandwidthTierEnum::TIER1),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkPerformanceConfigTotalEgressBandwidthTierEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkPolicyProviderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The selected network policy provider.
pub enum NetworkPolicyProviderEnum {
    

    /// Not set
    ///
    /// "PROVIDER_UNSPECIFIED"
    #[serde(rename="PROVIDER_UNSPECIFIED")]
    PROVIDERUNSPECIFIED,
    

    /// Tigera (Calico Felix).
    ///
    /// "CALICO"
    #[serde(rename="CALICO")]
    CALICO,
}

impl AsRef<str> for NetworkPolicyProviderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkPolicyProviderEnum::PROVIDERUNSPECIFIED => "PROVIDER_UNSPECIFIED",
            NetworkPolicyProviderEnum::CALICO => "CALICO",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkPolicyProviderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROVIDER_UNSPECIFIED" => Ok(NetworkPolicyProviderEnum::PROVIDERUNSPECIFIED),
           "CALICO" => Ok(NetworkPolicyProviderEnum::CALICO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkPolicyProviderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeAffinityOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Operator for NodeAffinity.
pub enum NodeAffinityOperatorEnum {
    

    /// Invalid or unspecified affinity operator.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// Affinity operator.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// Anti-affinity operator.
    ///
    /// "NOT_IN"
    #[serde(rename="NOT_IN")]
    NOTIN,
}

impl AsRef<str> for NodeAffinityOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeAffinityOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            NodeAffinityOperatorEnum::IN => "IN",
            NodeAffinityOperatorEnum::NOTIN => "NOT_IN",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeAffinityOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(NodeAffinityOperatorEnum::OPERATORUNSPECIFIED),
           "IN" => Ok(NodeAffinityOperatorEnum::IN),
           "NOT_IN" => Ok(NodeAffinityOperatorEnum::NOTIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeAffinityOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodePoolStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// [Output only] The status of the nodes in this pool instance.
pub enum NodePoolStatusEnum {
    

    /// Not set.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The PROVISIONING state indicates the node pool is being created.
    ///
    /// "PROVISIONING"
    #[serde(rename="PROVISIONING")]
    PROVISIONING,
    

    /// The RUNNING state indicates the node pool has been created and is fully usable.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The RUNNING_WITH_ERROR state indicates the node pool has been created and is partially usable. Some error state has occurred and some functionality may be impaired. Customer may need to reissue a request or trigger a new update.
    ///
    /// "RUNNING_WITH_ERROR"
    #[serde(rename="RUNNING_WITH_ERROR")]
    RUNNINGWITHERROR,
    

    /// The RECONCILING state indicates that some work is actively being done on the node pool, such as upgrading node software. Details can be found in the `statusMessage` field.
    ///
    /// "RECONCILING"
    #[serde(rename="RECONCILING")]
    RECONCILING,
    

    /// The STOPPING state indicates the node pool is being deleted.
    ///
    /// "STOPPING"
    #[serde(rename="STOPPING")]
    STOPPING,
    

    /// The ERROR state indicates the node pool may be unusable. Details can be found in the `statusMessage` field.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for NodePoolStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodePoolStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            NodePoolStatusEnum::PROVISIONING => "PROVISIONING",
            NodePoolStatusEnum::RUNNING => "RUNNING",
            NodePoolStatusEnum::RUNNINGWITHERROR => "RUNNING_WITH_ERROR",
            NodePoolStatusEnum::RECONCILING => "RECONCILING",
            NodePoolStatusEnum::STOPPING => "STOPPING",
            NodePoolStatusEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for NodePoolStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(NodePoolStatusEnum::STATUSUNSPECIFIED),
           "PROVISIONING" => Ok(NodePoolStatusEnum::PROVISIONING),
           "RUNNING" => Ok(NodePoolStatusEnum::RUNNING),
           "RUNNING_WITH_ERROR" => Ok(NodePoolStatusEnum::RUNNINGWITHERROR),
           "RECONCILING" => Ok(NodePoolStatusEnum::RECONCILING),
           "STOPPING" => Ok(NodePoolStatusEnum::STOPPING),
           "ERROR" => Ok(NodePoolStatusEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodePoolStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodePoolAutoscalingLocationPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Location policy used when scaling up a nodepool.
pub enum NodePoolAutoscalingLocationPolicyEnum {
    

    /// Not set.
    ///
    /// "LOCATION_POLICY_UNSPECIFIED"
    #[serde(rename="LOCATION_POLICY_UNSPECIFIED")]
    LOCATIONPOLICYUNSPECIFIED,
    

    /// BALANCED is a best effort policy that aims to balance the sizes of different zones.
    ///
    /// "BALANCED"
    #[serde(rename="BALANCED")]
    BALANCED,
    

    /// ANY policy picks zones that have the highest capacity available.
    ///
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
}

impl AsRef<str> for NodePoolAutoscalingLocationPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodePoolAutoscalingLocationPolicyEnum::LOCATIONPOLICYUNSPECIFIED => "LOCATION_POLICY_UNSPECIFIED",
            NodePoolAutoscalingLocationPolicyEnum::BALANCED => "BALANCED",
            NodePoolAutoscalingLocationPolicyEnum::ANY => "ANY",
        }
    }
}

impl std::convert::TryFrom< &str> for NodePoolAutoscalingLocationPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_POLICY_UNSPECIFIED" => Ok(NodePoolAutoscalingLocationPolicyEnum::LOCATIONPOLICYUNSPECIFIED),
           "BALANCED" => Ok(NodePoolAutoscalingLocationPolicyEnum::BALANCED),
           "ANY" => Ok(NodePoolAutoscalingLocationPolicyEnum::ANY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodePoolAutoscalingLocationPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NodeTaintEffectEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Effect for taint.
pub enum NodeTaintEffectEnum {
    

    /// Not set
    ///
    /// "EFFECT_UNSPECIFIED"
    #[serde(rename="EFFECT_UNSPECIFIED")]
    EFFECTUNSPECIFIED,
    

    /// NoSchedule
    ///
    /// "NO_SCHEDULE"
    #[serde(rename="NO_SCHEDULE")]
    NOSCHEDULE,
    

    /// PreferNoSchedule
    ///
    /// "PREFER_NO_SCHEDULE"
    #[serde(rename="PREFER_NO_SCHEDULE")]
    PREFERNOSCHEDULE,
    

    /// NoExecute
    ///
    /// "NO_EXECUTE"
    #[serde(rename="NO_EXECUTE")]
    NOEXECUTE,
}

impl AsRef<str> for NodeTaintEffectEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NodeTaintEffectEnum::EFFECTUNSPECIFIED => "EFFECT_UNSPECIFIED",
            NodeTaintEffectEnum::NOSCHEDULE => "NO_SCHEDULE",
            NodeTaintEffectEnum::PREFERNOSCHEDULE => "PREFER_NO_SCHEDULE",
            NodeTaintEffectEnum::NOEXECUTE => "NO_EXECUTE",
        }
    }
}

impl std::convert::TryFrom< &str> for NodeTaintEffectEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EFFECT_UNSPECIFIED" => Ok(NodeTaintEffectEnum::EFFECTUNSPECIFIED),
           "NO_SCHEDULE" => Ok(NodeTaintEffectEnum::NOSCHEDULE),
           "PREFER_NO_SCHEDULE" => Ok(NodeTaintEffectEnum::PREFERNOSCHEDULE),
           "NO_EXECUTE" => Ok(NodeTaintEffectEnum::NOEXECUTE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NodeTaintEffectEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationOperationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operation type.
pub enum OperationOperationTypeEnum {
    

    /// Not set.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The cluster is being created. The cluster should be assumed to be unusable until the operation finishes. In the event of the operation failing, the cluster will enter the ERROR state and eventually be deleted.
    ///
    /// "CREATE_CLUSTER"
    #[serde(rename="CREATE_CLUSTER")]
    CREATECLUSTER,
    

    /// The cluster is being deleted. The cluster should be assumed to be unusable as soon as this operation starts. In the event of the operation failing, the cluster will enter the ERROR state and the deletion will be automatically retried until completed.
    ///
    /// "DELETE_CLUSTER"
    #[serde(rename="DELETE_CLUSTER")]
    DELETECLUSTER,
    

    /// The cluster version is being updated. Note that this includes "upgrades" to the same version, which are simply a recreation. This also includes [auto-upgrades](https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-upgrades#upgrading_automatically). For more details, see [documentation on cluster upgrades](https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-upgrades#cluster_upgrades).
    ///
    /// "UPGRADE_MASTER"
    #[serde(rename="UPGRADE_MASTER")]
    UPGRADEMASTER,
    

    /// A node pool is being updated. Despite calling this an "upgrade", this includes most forms of updates to node pools. This also includes [auto-upgrades](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-upgrades). This operation sets the progress field and may be canceled. The upgrade strategy depends on [node pool configuration](https://cloud.google.com/kubernetes-engine/docs/concepts/node-pool-upgrade-strategies). The nodes are generally still usable during this operation.
    ///
    /// "UPGRADE_NODES"
    #[serde(rename="UPGRADE_NODES")]
    UPGRADENODES,
    

    /// A problem has been detected with the control plane and is being repaired. This operation type is initiated by GKE. For more details, see [documentation on repairs](https://cloud.google.com/kubernetes-engine/docs/concepts/maintenance-windows-and-exclusions#repairs).
    ///
    /// "REPAIR_CLUSTER"
    #[serde(rename="REPAIR_CLUSTER")]
    REPAIRCLUSTER,
    

    /// The cluster is being updated. This is a broad category of operations and includes operations that only change metadata as well as those that must recreate the entire cluster. If the control plane must be recreated, this will cause temporary downtime for zonal clusters. Some features require recreating the nodes as well. Those will be recreated as separate operations and the update may not be completely functional until the node pools recreations finish. Node recreations will generally follow [maintenance policies](https://cloud.google.com/kubernetes-engine/docs/concepts/maintenance-windows-and-exclusions). Some GKE-initiated operations use this type. This includes certain types of auto-upgrades and incident mitigations.
    ///
    /// "UPDATE_CLUSTER"
    #[serde(rename="UPDATE_CLUSTER")]
    UPDATECLUSTER,
    

    /// A node pool is being created. The node pool should be assumed to be unusable until this operation finishes. In the event of an error, the node pool may be partially created. If enabled, [node autoprovisioning](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning) may have automatically initiated such operations.
    ///
    /// "CREATE_NODE_POOL"
    #[serde(rename="CREATE_NODE_POOL")]
    CREATENODEPOOL,
    

    /// The node pool is being deleted. The node pool should be assumed to be unusable as soon as this operation starts.
    ///
    /// "DELETE_NODE_POOL"
    #[serde(rename="DELETE_NODE_POOL")]
    DELETENODEPOOL,
    

    /// The node pool's manamagent field is being updated. These operations only update metadata and may be concurrent with most other operations.
    ///
    /// "SET_NODE_POOL_MANAGEMENT"
    #[serde(rename="SET_NODE_POOL_MANAGEMENT")]
    SETNODEPOOLMANAGEMENT,
    

    /// A problem has been detected with nodes and [they are being repaired](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-repair). This operation type is initiated by GKE, typically automatically. This operation may be concurrent with other operations and there may be multiple repairs occurring on the same node pool.
    ///
    /// "AUTO_REPAIR_NODES"
    #[serde(rename="AUTO_REPAIR_NODES")]
    AUTOREPAIRNODES,
    

    /// Unused. Automatic node upgrade uses UPGRADE_NODES.
    ///
    /// "AUTO_UPGRADE_NODES"
    #[serde(rename="AUTO_UPGRADE_NODES")]
    AUTOUPGRADENODES,
    

    /// Unused. Updating labels uses UPDATE_CLUSTER.
    ///
    /// "SET_LABELS"
    #[serde(rename="SET_LABELS")]
    SETLABELS,
    

    /// Unused. Updating master auth uses UPDATE_CLUSTER.
    ///
    /// "SET_MASTER_AUTH"
    #[serde(rename="SET_MASTER_AUTH")]
    SETMASTERAUTH,
    

    /// The node pool is being resized. With the exception of resizing to or from size zero, the node pool is generally usable during this operation.
    ///
    /// "SET_NODE_POOL_SIZE"
    #[serde(rename="SET_NODE_POOL_SIZE")]
    SETNODEPOOLSIZE,
    

    /// Unused. Updating network policy uses UPDATE_CLUSTER.
    ///
    /// "SET_NETWORK_POLICY"
    #[serde(rename="SET_NETWORK_POLICY")]
    SETNETWORKPOLICY,
    

    /// Unused. Updating maintenance policy uses UPDATE_CLUSTER.
    ///
    /// "SET_MAINTENANCE_POLICY"
    #[serde(rename="SET_MAINTENANCE_POLICY")]
    SETMAINTENANCEPOLICY,
    

    /// The control plane is being resized. This operation type is initiated by GKE. These operations are often performed preemptively to ensure that the control plane has sufficient resources and is not typically an indication of issues. For more details, see [documentation on resizes](https://cloud.google.com/kubernetes-engine/docs/concepts/maintenance-windows-and-exclusions#repairs).
    ///
    /// "RESIZE_CLUSTER"
    #[serde(rename="RESIZE_CLUSTER")]
    RESIZECLUSTER,
    

    /// Fleet features of GKE Enterprise are being upgraded. The cluster should be assumed to be blocked for other upgrades until the operation finishes.
    ///
    /// "FLEET_FEATURE_UPGRADE"
    #[serde(rename="FLEET_FEATURE_UPGRADE")]
    FLEETFEATUREUPGRADE,
}

impl AsRef<str> for OperationOperationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationOperationTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            OperationOperationTypeEnum::CREATECLUSTER => "CREATE_CLUSTER",
            OperationOperationTypeEnum::DELETECLUSTER => "DELETE_CLUSTER",
            OperationOperationTypeEnum::UPGRADEMASTER => "UPGRADE_MASTER",
            OperationOperationTypeEnum::UPGRADENODES => "UPGRADE_NODES",
            OperationOperationTypeEnum::REPAIRCLUSTER => "REPAIR_CLUSTER",
            OperationOperationTypeEnum::UPDATECLUSTER => "UPDATE_CLUSTER",
            OperationOperationTypeEnum::CREATENODEPOOL => "CREATE_NODE_POOL",
            OperationOperationTypeEnum::DELETENODEPOOL => "DELETE_NODE_POOL",
            OperationOperationTypeEnum::SETNODEPOOLMANAGEMENT => "SET_NODE_POOL_MANAGEMENT",
            OperationOperationTypeEnum::AUTOREPAIRNODES => "AUTO_REPAIR_NODES",
            OperationOperationTypeEnum::AUTOUPGRADENODES => "AUTO_UPGRADE_NODES",
            OperationOperationTypeEnum::SETLABELS => "SET_LABELS",
            OperationOperationTypeEnum::SETMASTERAUTH => "SET_MASTER_AUTH",
            OperationOperationTypeEnum::SETNODEPOOLSIZE => "SET_NODE_POOL_SIZE",
            OperationOperationTypeEnum::SETNETWORKPOLICY => "SET_NETWORK_POLICY",
            OperationOperationTypeEnum::SETMAINTENANCEPOLICY => "SET_MAINTENANCE_POLICY",
            OperationOperationTypeEnum::RESIZECLUSTER => "RESIZE_CLUSTER",
            OperationOperationTypeEnum::FLEETFEATUREUPGRADE => "FLEET_FEATURE_UPGRADE",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationOperationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(OperationOperationTypeEnum::TYPEUNSPECIFIED),
           "CREATE_CLUSTER" => Ok(OperationOperationTypeEnum::CREATECLUSTER),
           "DELETE_CLUSTER" => Ok(OperationOperationTypeEnum::DELETECLUSTER),
           "UPGRADE_MASTER" => Ok(OperationOperationTypeEnum::UPGRADEMASTER),
           "UPGRADE_NODES" => Ok(OperationOperationTypeEnum::UPGRADENODES),
           "REPAIR_CLUSTER" => Ok(OperationOperationTypeEnum::REPAIRCLUSTER),
           "UPDATE_CLUSTER" => Ok(OperationOperationTypeEnum::UPDATECLUSTER),
           "CREATE_NODE_POOL" => Ok(OperationOperationTypeEnum::CREATENODEPOOL),
           "DELETE_NODE_POOL" => Ok(OperationOperationTypeEnum::DELETENODEPOOL),
           "SET_NODE_POOL_MANAGEMENT" => Ok(OperationOperationTypeEnum::SETNODEPOOLMANAGEMENT),
           "AUTO_REPAIR_NODES" => Ok(OperationOperationTypeEnum::AUTOREPAIRNODES),
           "AUTO_UPGRADE_NODES" => Ok(OperationOperationTypeEnum::AUTOUPGRADENODES),
           "SET_LABELS" => Ok(OperationOperationTypeEnum::SETLABELS),
           "SET_MASTER_AUTH" => Ok(OperationOperationTypeEnum::SETMASTERAUTH),
           "SET_NODE_POOL_SIZE" => Ok(OperationOperationTypeEnum::SETNODEPOOLSIZE),
           "SET_NETWORK_POLICY" => Ok(OperationOperationTypeEnum::SETNETWORKPOLICY),
           "SET_MAINTENANCE_POLICY" => Ok(OperationOperationTypeEnum::SETMAINTENANCEPOLICY),
           "RESIZE_CLUSTER" => Ok(OperationOperationTypeEnum::RESIZECLUSTER),
           "FLEET_FEATURE_UPGRADE" => Ok(OperationOperationTypeEnum::FLEETFEATUREUPGRADE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationOperationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current status of the operation.
pub enum OperationStatusEnum {
    

    /// Not set.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The operation has been created.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The operation is currently running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The operation is done, either cancelled or completed.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    

    /// The operation is aborting.
    ///
    /// "ABORTING"
    #[serde(rename="ABORTING")]
    ABORTING,
}

impl AsRef<str> for OperationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            OperationStatusEnum::PENDING => "PENDING",
            OperationStatusEnum::RUNNING => "RUNNING",
            OperationStatusEnum::DONE => "DONE",
            OperationStatusEnum::ABORTING => "ABORTING",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(OperationStatusEnum::STATUSUNSPECIFIED),
           "PENDING" => Ok(OperationStatusEnum::PENDING),
           "RUNNING" => Ok(OperationStatusEnum::RUNNING),
           "DONE" => Ok(OperationStatusEnum::DONE),
           "ABORTING" => Ok(OperationStatusEnum::ABORTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OperationProgresStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of an operation stage. Unset for single-stage operations.
pub enum OperationProgresStatusEnum {
    

    /// Not set.
    ///
    /// "STATUS_UNSPECIFIED"
    #[serde(rename="STATUS_UNSPECIFIED")]
    STATUSUNSPECIFIED,
    

    /// The operation has been created.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The operation is currently running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The operation is done, either cancelled or completed.
    ///
    /// "DONE"
    #[serde(rename="DONE")]
    DONE,
    

    /// The operation is aborting.
    ///
    /// "ABORTING"
    #[serde(rename="ABORTING")]
    ABORTING,
}

impl AsRef<str> for OperationProgresStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OperationProgresStatusEnum::STATUSUNSPECIFIED => "STATUS_UNSPECIFIED",
            OperationProgresStatusEnum::PENDING => "PENDING",
            OperationProgresStatusEnum::RUNNING => "RUNNING",
            OperationProgresStatusEnum::DONE => "DONE",
            OperationProgresStatusEnum::ABORTING => "ABORTING",
        }
    }
}

impl std::convert::TryFrom< &str> for OperationProgresStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATUS_UNSPECIFIED" => Ok(OperationProgresStatusEnum::STATUSUNSPECIFIED),
           "PENDING" => Ok(OperationProgresStatusEnum::PENDING),
           "RUNNING" => Ok(OperationProgresStatusEnum::RUNNING),
           "DONE" => Ok(OperationProgresStatusEnum::DONE),
           "ABORTING" => Ok(OperationProgresStatusEnum::ABORTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OperationProgresStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlacementPolicyTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of placement.
pub enum PlacementPolicyTypeEnum {
    

    /// TYPE_UNSPECIFIED specifies no requirements on nodes placement.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// COMPACT specifies node placement in the same availability domain to ensure low communication latency.
    ///
    /// "COMPACT"
    #[serde(rename="COMPACT")]
    COMPACT,
}

impl AsRef<str> for PlacementPolicyTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlacementPolicyTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            PlacementPolicyTypeEnum::COMPACT => "COMPACT",
        }
    }
}

impl std::convert::TryFrom< &str> for PlacementPolicyTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(PlacementPolicyTypeEnum::TYPEUNSPECIFIED),
           "COMPACT" => Ok(PlacementPolicyTypeEnum::COMPACT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlacementPolicyTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReleaseChannelChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// channel specifies which release channel the cluster is subscribed to.
pub enum ReleaseChannelChannelEnum {
    

    /// No channel specified.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// RAPID channel is offered on an early access basis for customers who want to test new releases. WARNING: Versions available in the RAPID Channel may be subject to unresolved issues with no known workaround and are not subject to any SLAs.
    ///
    /// "RAPID"
    #[serde(rename="RAPID")]
    RAPID,
    

    /// Clusters subscribed to REGULAR receive versions that are considered GA quality. REGULAR is intended for production users who want to take advantage of new features.
    ///
    /// "REGULAR"
    #[serde(rename="REGULAR")]
    REGULAR,
    

    /// Clusters subscribed to STABLE receive versions that are known to be stable and reliable in production.
    ///
    /// "STABLE"
    #[serde(rename="STABLE")]
    STABLE,
}

impl AsRef<str> for ReleaseChannelChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReleaseChannelChannelEnum::UNSPECIFIED => "UNSPECIFIED",
            ReleaseChannelChannelEnum::RAPID => "RAPID",
            ReleaseChannelChannelEnum::REGULAR => "REGULAR",
            ReleaseChannelChannelEnum::STABLE => "STABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReleaseChannelChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(ReleaseChannelChannelEnum::UNSPECIFIED),
           "RAPID" => Ok(ReleaseChannelChannelEnum::RAPID),
           "REGULAR" => Ok(ReleaseChannelChannelEnum::REGULAR),
           "STABLE" => Ok(ReleaseChannelChannelEnum::STABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReleaseChannelChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReleaseChannelConfigChannelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The release channel this configuration applies to.
pub enum ReleaseChannelConfigChannelEnum {
    

    /// No channel specified.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// RAPID channel is offered on an early access basis for customers who want to test new releases. WARNING: Versions available in the RAPID Channel may be subject to unresolved issues with no known workaround and are not subject to any SLAs.
    ///
    /// "RAPID"
    #[serde(rename="RAPID")]
    RAPID,
    

    /// Clusters subscribed to REGULAR receive versions that are considered GA quality. REGULAR is intended for production users who want to take advantage of new features.
    ///
    /// "REGULAR"
    #[serde(rename="REGULAR")]
    REGULAR,
    

    /// Clusters subscribed to STABLE receive versions that are known to be stable and reliable in production.
    ///
    /// "STABLE"
    #[serde(rename="STABLE")]
    STABLE,
}

impl AsRef<str> for ReleaseChannelConfigChannelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReleaseChannelConfigChannelEnum::UNSPECIFIED => "UNSPECIFIED",
            ReleaseChannelConfigChannelEnum::RAPID => "RAPID",
            ReleaseChannelConfigChannelEnum::REGULAR => "REGULAR",
            ReleaseChannelConfigChannelEnum::STABLE => "STABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReleaseChannelConfigChannelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(ReleaseChannelConfigChannelEnum::UNSPECIFIED),
           "RAPID" => Ok(ReleaseChannelConfigChannelEnum::RAPID),
           "REGULAR" => Ok(ReleaseChannelConfigChannelEnum::REGULAR),
           "STABLE" => Ok(ReleaseChannelConfigChannelEnum::STABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReleaseChannelConfigChannelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReservationAffinityConsumeReservationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Corresponds to the type of reservation consumption.
pub enum ReservationAffinityConsumeReservationTypeEnum {
    

    /// Default value. This should not be used.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Do not consume from any reserved capacity.
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
            ReservationAffinityConsumeReservationTypeEnum::UNSPECIFIED => "UNSPECIFIED",
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
           "UNSPECIFIED" => Ok(ReservationAffinityConsumeReservationTypeEnum::UNSPECIFIED),
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


// region SandboxConfigTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the sandbox to use for the node.
pub enum SandboxConfigTypeEnum {
    

    /// Default value. This should not be used.
    ///
    /// "UNSPECIFIED"
    #[serde(rename="UNSPECIFIED")]
    UNSPECIFIED,
    

    /// Run sandbox using gvisor.
    ///
    /// "GVISOR"
    #[serde(rename="GVISOR")]
    GVISOR,
}

impl AsRef<str> for SandboxConfigTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SandboxConfigTypeEnum::UNSPECIFIED => "UNSPECIFIED",
            SandboxConfigTypeEnum::GVISOR => "GVISOR",
        }
    }
}

impl std::convert::TryFrom< &str> for SandboxConfigTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED" => Ok(SandboxConfigTypeEnum::UNSPECIFIED),
           "GVISOR" => Ok(SandboxConfigTypeEnum::GVISOR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SandboxConfigTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SecondaryBootDiskModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Disk mode (container image cache, etc.)
pub enum SecondaryBootDiskModeEnum {
    

    /// MODE_UNSPECIFIED is when mode is not set.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// CONTAINER_IMAGE_CACHE is for using the secondary boot disk as a container image cache.
    ///
    /// "CONTAINER_IMAGE_CACHE"
    #[serde(rename="CONTAINER_IMAGE_CACHE")]
    CONTAINERIMAGECACHE,
}

impl AsRef<str> for SecondaryBootDiskModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecondaryBootDiskModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            SecondaryBootDiskModeEnum::CONTAINERIMAGECACHE => "CONTAINER_IMAGE_CACHE",
        }
    }
}

impl std::convert::TryFrom< &str> for SecondaryBootDiskModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(SecondaryBootDiskModeEnum::MODEUNSPECIFIED),
           "CONTAINER_IMAGE_CACHE" => Ok(SecondaryBootDiskModeEnum::CONTAINERIMAGECACHE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecondaryBootDiskModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SecurityPostureConfigModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets which mode to use for Security Posture features.
pub enum SecurityPostureConfigModeEnum {
    

    /// Default value not specified.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Disables Security Posture features on the cluster.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Applies Security Posture features on the cluster.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for SecurityPostureConfigModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecurityPostureConfigModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            SecurityPostureConfigModeEnum::DISABLED => "DISABLED",
            SecurityPostureConfigModeEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for SecurityPostureConfigModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(SecurityPostureConfigModeEnum::MODEUNSPECIFIED),
           "DISABLED" => Ok(SecurityPostureConfigModeEnum::DISABLED),
           "BASIC" => Ok(SecurityPostureConfigModeEnum::BASIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecurityPostureConfigModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SecurityPostureConfigVulnerabilityModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets which mode to use for vulnerability scanning.
pub enum SecurityPostureConfigVulnerabilityModeEnum {
    

    /// Default value not specified.
    ///
    /// "VULNERABILITY_MODE_UNSPECIFIED"
    #[serde(rename="VULNERABILITY_MODE_UNSPECIFIED")]
    VULNERABILITYMODEUNSPECIFIED,
    

    /// Disables vulnerability scanning on the cluster.
    ///
    /// "VULNERABILITY_DISABLED"
    #[serde(rename="VULNERABILITY_DISABLED")]
    VULNERABILITYDISABLED,
    

    /// Applies basic vulnerability scanning on the cluster.
    ///
    /// "VULNERABILITY_BASIC"
    #[serde(rename="VULNERABILITY_BASIC")]
    VULNERABILITYBASIC,
    

    /// Applies the Security Posture's vulnerability on cluster Enterprise level features.
    ///
    /// "VULNERABILITY_ENTERPRISE"
    #[serde(rename="VULNERABILITY_ENTERPRISE")]
    VULNERABILITYENTERPRISE,
}

impl AsRef<str> for SecurityPostureConfigVulnerabilityModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYMODEUNSPECIFIED => "VULNERABILITY_MODE_UNSPECIFIED",
            SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYDISABLED => "VULNERABILITY_DISABLED",
            SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYBASIC => "VULNERABILITY_BASIC",
            SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYENTERPRISE => "VULNERABILITY_ENTERPRISE",
        }
    }
}

impl std::convert::TryFrom< &str> for SecurityPostureConfigVulnerabilityModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VULNERABILITY_MODE_UNSPECIFIED" => Ok(SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYMODEUNSPECIFIED),
           "VULNERABILITY_DISABLED" => Ok(SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYDISABLED),
           "VULNERABILITY_BASIC" => Ok(SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYBASIC),
           "VULNERABILITY_ENTERPRISE" => Ok(SecurityPostureConfigVulnerabilityModeEnum::VULNERABILITYENTERPRISE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SecurityPostureConfigVulnerabilityModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SetMasterAuthRequestActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The exact form of action to be taken on the master auth.
pub enum SetMasterAuthRequestActionEnum {
    

    /// Operation is unknown and will error out.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Set the password to a user generated value.
    ///
    /// "SET_PASSWORD"
    #[serde(rename="SET_PASSWORD")]
    SETPASSWORD,
    

    /// Generate a new password and set it to that.
    ///
    /// "GENERATE_PASSWORD"
    #[serde(rename="GENERATE_PASSWORD")]
    GENERATEPASSWORD,
    

    /// Set the username. If an empty username is provided, basic authentication is disabled for the cluster. If a non-empty username is provided, basic authentication is enabled, with either a provided password or a generated one.
    ///
    /// "SET_USERNAME"
    #[serde(rename="SET_USERNAME")]
    SETUSERNAME,
}

impl AsRef<str> for SetMasterAuthRequestActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SetMasterAuthRequestActionEnum::UNKNOWN => "UNKNOWN",
            SetMasterAuthRequestActionEnum::SETPASSWORD => "SET_PASSWORD",
            SetMasterAuthRequestActionEnum::GENERATEPASSWORD => "GENERATE_PASSWORD",
            SetMasterAuthRequestActionEnum::SETUSERNAME => "SET_USERNAME",
        }
    }
}

impl std::convert::TryFrom< &str> for SetMasterAuthRequestActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(SetMasterAuthRequestActionEnum::UNKNOWN),
           "SET_PASSWORD" => Ok(SetMasterAuthRequestActionEnum::SETPASSWORD),
           "GENERATE_PASSWORD" => Ok(SetMasterAuthRequestActionEnum::GENERATEPASSWORD),
           "SET_USERNAME" => Ok(SetMasterAuthRequestActionEnum::SETUSERNAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SetMasterAuthRequestActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StatusConditionCanonicalCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Canonical code of the condition.
pub enum StatusConditionCanonicalCodeEnum {
    

    /// Not an error; returned on success. HTTP Mapping: 200 OK
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request
    ///
    /// "INVALID_ARGUMENT"
    #[serde(rename="INVALID_ARGUMENT")]
    INVALIDARGUMENT,
    

    /// The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout
    ///
    /// "DEADLINE_EXCEEDED"
    #[serde(rename="DEADLINE_EXCEEDED")]
    DEADLINEEXCEEDED,
    

    /// Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict
    ///
    /// "ALREADY_EXISTS"
    #[serde(rename="ALREADY_EXISTS")]
    ALREADYEXISTS,
    

    /// The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden
    ///
    /// "PERMISSION_DENIED"
    #[serde(rename="PERMISSION_DENIED")]
    PERMISSIONDENIED,
    

    /// The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized
    ///
    /// "UNAUTHENTICATED"
    #[serde(rename="UNAUTHENTICATED")]
    UNAUTHENTICATED,
    

    /// Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests
    ///
    /// "RESOURCE_EXHAUSTED"
    #[serde(rename="RESOURCE_EXHAUSTED")]
    RESOURCEEXHAUSTED,
    

    /// The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level. For example, when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence. (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. For example, if an "rmdir" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request
    ///
    /// "FAILED_PRECONDITION"
    #[serde(rename="FAILED_PRECONDITION")]
    FAILEDPRECONDITION,
    

    /// The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
    

    /// The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request
    ///
    /// "OUT_OF_RANGE"
    #[serde(rename="OUT_OF_RANGE")]
    OUTOFRANGE,
    

    /// The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented
    ///
    /// "UNIMPLEMENTED"
    #[serde(rename="UNIMPLEMENTED")]
    UNIMPLEMENTED,
    

    /// Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error
    ///
    /// "INTERNAL"
    #[serde(rename="INTERNAL")]
    INTERNAL,
    

    /// The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
    

    /// Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error
    ///
    /// "DATA_LOSS"
    #[serde(rename="DATA_LOSS")]
    DATALOSS,
}

impl AsRef<str> for StatusConditionCanonicalCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StatusConditionCanonicalCodeEnum::OK => "OK",
            StatusConditionCanonicalCodeEnum::CANCELLED => "CANCELLED",
            StatusConditionCanonicalCodeEnum::UNKNOWN => "UNKNOWN",
            StatusConditionCanonicalCodeEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            StatusConditionCanonicalCodeEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            StatusConditionCanonicalCodeEnum::NOTFOUND => "NOT_FOUND",
            StatusConditionCanonicalCodeEnum::ALREADYEXISTS => "ALREADY_EXISTS",
            StatusConditionCanonicalCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            StatusConditionCanonicalCodeEnum::UNAUTHENTICATED => "UNAUTHENTICATED",
            StatusConditionCanonicalCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            StatusConditionCanonicalCodeEnum::FAILEDPRECONDITION => "FAILED_PRECONDITION",
            StatusConditionCanonicalCodeEnum::ABORTED => "ABORTED",
            StatusConditionCanonicalCodeEnum::OUTOFRANGE => "OUT_OF_RANGE",
            StatusConditionCanonicalCodeEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            StatusConditionCanonicalCodeEnum::INTERNAL => "INTERNAL",
            StatusConditionCanonicalCodeEnum::UNAVAILABLE => "UNAVAILABLE",
            StatusConditionCanonicalCodeEnum::DATALOSS => "DATA_LOSS",
        }
    }
}

impl std::convert::TryFrom< &str> for StatusConditionCanonicalCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OK" => Ok(StatusConditionCanonicalCodeEnum::OK),
           "CANCELLED" => Ok(StatusConditionCanonicalCodeEnum::CANCELLED),
           "UNKNOWN" => Ok(StatusConditionCanonicalCodeEnum::UNKNOWN),
           "INVALID_ARGUMENT" => Ok(StatusConditionCanonicalCodeEnum::INVALIDARGUMENT),
           "DEADLINE_EXCEEDED" => Ok(StatusConditionCanonicalCodeEnum::DEADLINEEXCEEDED),
           "NOT_FOUND" => Ok(StatusConditionCanonicalCodeEnum::NOTFOUND),
           "ALREADY_EXISTS" => Ok(StatusConditionCanonicalCodeEnum::ALREADYEXISTS),
           "PERMISSION_DENIED" => Ok(StatusConditionCanonicalCodeEnum::PERMISSIONDENIED),
           "UNAUTHENTICATED" => Ok(StatusConditionCanonicalCodeEnum::UNAUTHENTICATED),
           "RESOURCE_EXHAUSTED" => Ok(StatusConditionCanonicalCodeEnum::RESOURCEEXHAUSTED),
           "FAILED_PRECONDITION" => Ok(StatusConditionCanonicalCodeEnum::FAILEDPRECONDITION),
           "ABORTED" => Ok(StatusConditionCanonicalCodeEnum::ABORTED),
           "OUT_OF_RANGE" => Ok(StatusConditionCanonicalCodeEnum::OUTOFRANGE),
           "UNIMPLEMENTED" => Ok(StatusConditionCanonicalCodeEnum::UNIMPLEMENTED),
           "INTERNAL" => Ok(StatusConditionCanonicalCodeEnum::INTERNAL),
           "UNAVAILABLE" => Ok(StatusConditionCanonicalCodeEnum::UNAVAILABLE),
           "DATA_LOSS" => Ok(StatusConditionCanonicalCodeEnum::DATALOSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StatusConditionCanonicalCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StatusConditionCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Machine-friendly representation of the condition Deprecated. Use canonical_code instead.
pub enum StatusConditionCodeEnum {
    

    /// UNKNOWN indicates a generic condition.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// GCE_STOCKOUT indicates that Google Compute Engine resources are temporarily unavailable.
    ///
    /// "GCE_STOCKOUT"
    #[serde(rename="GCE_STOCKOUT")]
    GCESTOCKOUT,
    

    /// GKE_SERVICE_ACCOUNT_DELETED indicates that the user deleted their robot service account.
    ///
    /// "GKE_SERVICE_ACCOUNT_DELETED"
    #[serde(rename="GKE_SERVICE_ACCOUNT_DELETED")]
    GKESERVICEACCOUNTDELETED,
    

    /// Google Compute Engine quota was exceeded.
    ///
    /// "GCE_QUOTA_EXCEEDED"
    #[serde(rename="GCE_QUOTA_EXCEEDED")]
    GCEQUOTAEXCEEDED,
    

    /// Cluster state was manually changed by an SRE due to a system logic error.
    ///
    /// "SET_BY_OPERATOR"
    #[serde(rename="SET_BY_OPERATOR")]
    SETBYOPERATOR,
    

    /// Unable to perform an encrypt operation against the CloudKMS key used for etcd level encryption.
    ///
    /// "CLOUD_KMS_KEY_ERROR"
    #[serde(rename="CLOUD_KMS_KEY_ERROR")]
    CLOUDKMSKEYERROR,
    

    /// Cluster CA is expiring soon.
    ///
    /// "CA_EXPIRING"
    #[serde(rename="CA_EXPIRING")]
    CAEXPIRING,
}

impl AsRef<str> for StatusConditionCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StatusConditionCodeEnum::UNKNOWN => "UNKNOWN",
            StatusConditionCodeEnum::GCESTOCKOUT => "GCE_STOCKOUT",
            StatusConditionCodeEnum::GKESERVICEACCOUNTDELETED => "GKE_SERVICE_ACCOUNT_DELETED",
            StatusConditionCodeEnum::GCEQUOTAEXCEEDED => "GCE_QUOTA_EXCEEDED",
            StatusConditionCodeEnum::SETBYOPERATOR => "SET_BY_OPERATOR",
            StatusConditionCodeEnum::CLOUDKMSKEYERROR => "CLOUD_KMS_KEY_ERROR",
            StatusConditionCodeEnum::CAEXPIRING => "CA_EXPIRING",
        }
    }
}

impl std::convert::TryFrom< &str> for StatusConditionCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(StatusConditionCodeEnum::UNKNOWN),
           "GCE_STOCKOUT" => Ok(StatusConditionCodeEnum::GCESTOCKOUT),
           "GKE_SERVICE_ACCOUNT_DELETED" => Ok(StatusConditionCodeEnum::GKESERVICEACCOUNTDELETED),
           "GCE_QUOTA_EXCEEDED" => Ok(StatusConditionCodeEnum::GCEQUOTAEXCEEDED),
           "SET_BY_OPERATOR" => Ok(StatusConditionCodeEnum::SETBYOPERATOR),
           "CLOUD_KMS_KEY_ERROR" => Ok(StatusConditionCodeEnum::CLOUDKMSKEYERROR),
           "CA_EXPIRING" => Ok(StatusConditionCodeEnum::CAEXPIRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StatusConditionCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UpgradeSettingStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Update strategy of the node pool.
pub enum UpgradeSettingStrategyEnum {
    

    /// Default value if unset. GKE internally defaults the update strategy to SURGE for unspecified strategies.
    ///
    /// "NODE_POOL_UPDATE_STRATEGY_UNSPECIFIED"
    #[serde(rename="NODE_POOL_UPDATE_STRATEGY_UNSPECIFIED")]
    NODEPOOLUPDATESTRATEGYUNSPECIFIED,
    

    /// blue-green upgrade.
    ///
    /// "BLUE_GREEN"
    #[serde(rename="BLUE_GREEN")]
    BLUEGREEN,
    

    /// SURGE is the traditional way of upgrade a node pool. max_surge and max_unavailable determines the level of upgrade parallelism.
    ///
    /// "SURGE"
    #[serde(rename="SURGE")]
    SURGE,
}

impl AsRef<str> for UpgradeSettingStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UpgradeSettingStrategyEnum::NODEPOOLUPDATESTRATEGYUNSPECIFIED => "NODE_POOL_UPDATE_STRATEGY_UNSPECIFIED",
            UpgradeSettingStrategyEnum::BLUEGREEN => "BLUE_GREEN",
            UpgradeSettingStrategyEnum::SURGE => "SURGE",
        }
    }
}

impl std::convert::TryFrom< &str> for UpgradeSettingStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NODE_POOL_UPDATE_STRATEGY_UNSPECIFIED" => Ok(UpgradeSettingStrategyEnum::NODEPOOLUPDATESTRATEGYUNSPECIFIED),
           "BLUE_GREEN" => Ok(UpgradeSettingStrategyEnum::BLUEGREEN),
           "SURGE" => Ok(UpgradeSettingStrategyEnum::SURGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UpgradeSettingStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UsableSubnetworkSecondaryRangeStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This field is to determine the status of the secondary range programmably.
pub enum UsableSubnetworkSecondaryRangeStatusEnum {
    

    /// UNKNOWN is the zero value of the Status enum. It's not a valid status.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// UNUSED denotes that this range is unclaimed by any cluster.
    ///
    /// "UNUSED"
    #[serde(rename="UNUSED")]
    UNUSED,
    

    /// IN_USE_SERVICE denotes that this range is claimed by cluster(s) for services. User-managed services range can be shared between clusters within the same subnetwork.
    ///
    /// "IN_USE_SERVICE"
    #[serde(rename="IN_USE_SERVICE")]
    INUSESERVICE,
    

    /// IN_USE_SHAREABLE_POD denotes this range was created by the network admin and is currently claimed by a cluster for pods. It can only be used by other clusters as a pod range.
    ///
    /// "IN_USE_SHAREABLE_POD"
    #[serde(rename="IN_USE_SHAREABLE_POD")]
    INUSESHAREABLEPOD,
    

    /// IN_USE_MANAGED_POD denotes this range was created by GKE and is claimed for pods. It cannot be used for other clusters.
    ///
    /// "IN_USE_MANAGED_POD"
    #[serde(rename="IN_USE_MANAGED_POD")]
    INUSEMANAGEDPOD,
}

impl AsRef<str> for UsableSubnetworkSecondaryRangeStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UsableSubnetworkSecondaryRangeStatusEnum::UNKNOWN => "UNKNOWN",
            UsableSubnetworkSecondaryRangeStatusEnum::UNUSED => "UNUSED",
            UsableSubnetworkSecondaryRangeStatusEnum::INUSESERVICE => "IN_USE_SERVICE",
            UsableSubnetworkSecondaryRangeStatusEnum::INUSESHAREABLEPOD => "IN_USE_SHAREABLE_POD",
            UsableSubnetworkSecondaryRangeStatusEnum::INUSEMANAGEDPOD => "IN_USE_MANAGED_POD",
        }
    }
}

impl std::convert::TryFrom< &str> for UsableSubnetworkSecondaryRangeStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(UsableSubnetworkSecondaryRangeStatusEnum::UNKNOWN),
           "UNUSED" => Ok(UsableSubnetworkSecondaryRangeStatusEnum::UNUSED),
           "IN_USE_SERVICE" => Ok(UsableSubnetworkSecondaryRangeStatusEnum::INUSESERVICE),
           "IN_USE_SHAREABLE_POD" => Ok(UsableSubnetworkSecondaryRangeStatusEnum::INUSESHAREABLEPOD),
           "IN_USE_MANAGED_POD" => Ok(UsableSubnetworkSecondaryRangeStatusEnum::INUSEMANAGEDPOD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UsableSubnetworkSecondaryRangeStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WindowsNodeConfigOsVersionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// OSVersion specifies the Windows node config to be used on the node
pub enum WindowsNodeConfigOsVersionEnum {
    

    /// When OSVersion is not specified
    ///
    /// "OS_VERSION_UNSPECIFIED"
    #[serde(rename="OS_VERSION_UNSPECIFIED")]
    OSVERSIONUNSPECIFIED,
    

    /// LTSC2019 specifies to use LTSC2019 as the Windows Servercore Base Image
    ///
    /// "OS_VERSION_LTSC2019"
    #[serde(rename="OS_VERSION_LTSC2019")]
    OSVERSIONLTSC2019,
    

    /// LTSC2022 specifies to use LTSC2022 as the Windows Servercore Base Image
    ///
    /// "OS_VERSION_LTSC2022"
    #[serde(rename="OS_VERSION_LTSC2022")]
    OSVERSIONLTSC2022,
}

impl AsRef<str> for WindowsNodeConfigOsVersionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WindowsNodeConfigOsVersionEnum::OSVERSIONUNSPECIFIED => "OS_VERSION_UNSPECIFIED",
            WindowsNodeConfigOsVersionEnum::OSVERSIONLTSC2019 => "OS_VERSION_LTSC2019",
            WindowsNodeConfigOsVersionEnum::OSVERSIONLTSC2022 => "OS_VERSION_LTSC2022",
        }
    }
}

impl std::convert::TryFrom< &str> for WindowsNodeConfigOsVersionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OS_VERSION_UNSPECIFIED" => Ok(WindowsNodeConfigOsVersionEnum::OSVERSIONUNSPECIFIED),
           "OS_VERSION_LTSC2019" => Ok(WindowsNodeConfigOsVersionEnum::OSVERSIONLTSC2019),
           "OS_VERSION_LTSC2022" => Ok(WindowsNodeConfigOsVersionEnum::OSVERSIONLTSC2022),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WindowsNodeConfigOsVersionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region WorkloadMetadataConfigModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode is the configuration for how to expose metadata to workloads running on the node pool.
pub enum WorkloadMetadataConfigModeEnum {
    

    /// Not set.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Expose all Compute Engine metadata to pods.
    ///
    /// "GCE_METADATA"
    #[serde(rename="GCE_METADATA")]
    GCEMETADATA,
    

    /// Run the GKE Metadata Server on this node. The GKE Metadata Server exposes a metadata API to workloads that is compatible with the V1 Compute Metadata APIs exposed by the Compute Engine and App Engine Metadata Servers. This feature can only be enabled if Workload Identity is enabled at the cluster level.
    ///
    /// "GKE_METADATA"
    #[serde(rename="GKE_METADATA")]
    GKEMETADATA,
}

impl AsRef<str> for WorkloadMetadataConfigModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            WorkloadMetadataConfigModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            WorkloadMetadataConfigModeEnum::GCEMETADATA => "GCE_METADATA",
            WorkloadMetadataConfigModeEnum::GKEMETADATA => "GKE_METADATA",
        }
    }
}

impl std::convert::TryFrom< &str> for WorkloadMetadataConfigModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(WorkloadMetadataConfigModeEnum::MODEUNSPECIFIED),
           "GCE_METADATA" => Ok(WorkloadMetadataConfigModeEnum::GCEMETADATA),
           "GKE_METADATA" => Ok(WorkloadMetadataConfigModeEnum::GKEMETADATA),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a WorkloadMetadataConfigModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


