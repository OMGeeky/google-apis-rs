use super::*;



// region AcceleratorAcceleratorTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of an accelator for a CDF instance.
pub enum AcceleratorAcceleratorTypeEnum {
    

    /// Default value, if unspecified.
    ///
    /// "ACCELERATOR_TYPE_UNSPECIFIED"
    #[serde(rename="ACCELERATOR_TYPE_UNSPECIFIED")]
    ACCELERATORTYPEUNSPECIFIED,
    

    /// Change Data Capture accelerator for CDF.
    ///
    /// "CDC"
    #[serde(rename="CDC")]
    CDC,
    

    /// Cloud Healthcare accelerator for CDF. This accelerator is to enable Cloud Healthcare specific CDF plugins developed by Healthcare team.
    ///
    /// "HEALTHCARE"
    #[serde(rename="HEALTHCARE")]
    HEALTHCARE,
    

    /// Contact Center AI Insights This accelerator is used to enable import and export pipelines custom built to streamline CCAI Insights processing.
    ///
    /// "CCAI_INSIGHTS"
    #[serde(rename="CCAI_INSIGHTS")]
    CCAIINSIGHTS,
    

    /// Cloud search accelerator for CDF. This accelerator is to enable Cloud search specific CDF plugins developed by Cloudsearch team.
    ///
    /// "CLOUDSEARCH"
    #[serde(rename="CLOUDSEARCH")]
    CLOUDSEARCH,
}

impl AsRef<str> for AcceleratorAcceleratorTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AcceleratorAcceleratorTypeEnum::ACCELERATORTYPEUNSPECIFIED => "ACCELERATOR_TYPE_UNSPECIFIED",
            AcceleratorAcceleratorTypeEnum::CDC => "CDC",
            AcceleratorAcceleratorTypeEnum::HEALTHCARE => "HEALTHCARE",
            AcceleratorAcceleratorTypeEnum::CCAIINSIGHTS => "CCAI_INSIGHTS",
            AcceleratorAcceleratorTypeEnum::CLOUDSEARCH => "CLOUDSEARCH",
        }
    }
}

impl std::convert::TryFrom< &str> for AcceleratorAcceleratorTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCELERATOR_TYPE_UNSPECIFIED" => Ok(AcceleratorAcceleratorTypeEnum::ACCELERATORTYPEUNSPECIFIED),
           "CDC" => Ok(AcceleratorAcceleratorTypeEnum::CDC),
           "HEALTHCARE" => Ok(AcceleratorAcceleratorTypeEnum::HEALTHCARE),
           "CCAI_INSIGHTS" => Ok(AcceleratorAcceleratorTypeEnum::CCAIINSIGHTS),
           "CLOUDSEARCH" => Ok(AcceleratorAcceleratorTypeEnum::CLOUDSEARCH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AcceleratorAcceleratorTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AcceleratorStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the accelerator.
pub enum AcceleratorStateEnum {
    

    /// Default value, do not use.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Indicates that the accelerator is enabled and available to use.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// Indicates that the accelerator is disabled and not available to use.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
    

    /// Indicates that accelerator state is currently unknown. Requests for enable, disable could be retried while in this state.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
}

impl AsRef<str> for AcceleratorStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AcceleratorStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            AcceleratorStateEnum::ENABLED => "ENABLED",
            AcceleratorStateEnum::DISABLED => "DISABLED",
            AcceleratorStateEnum::UNKNOWN => "UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for AcceleratorStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(AcceleratorStateEnum::STATEUNSPECIFIED),
           "ENABLED" => Ok(AcceleratorStateEnum::ENABLED),
           "DISABLED" => Ok(AcceleratorStateEnum::DISABLED),
           "UNKNOWN" => Ok(AcceleratorStateEnum::UNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AcceleratorStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AuditLogConfigLogTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The log type that this config enables.
pub enum AuditLogConfigLogTypeEnum {
    

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

impl AsRef<str> for AuditLogConfigLogTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED => "LOG_TYPE_UNSPECIFIED",
            AuditLogConfigLogTypeEnum::ADMINREAD => "ADMIN_READ",
            AuditLogConfigLogTypeEnum::DATAWRITE => "DATA_WRITE",
            AuditLogConfigLogTypeEnum::DATAREAD => "DATA_READ",
        }
    }
}

impl std::convert::TryFrom< &str> for AuditLogConfigLogTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOG_TYPE_UNSPECIFIED" => Ok(AuditLogConfigLogTypeEnum::LOGTYPEUNSPECIFIED),
           "ADMIN_READ" => Ok(AuditLogConfigLogTypeEnum::ADMINREAD),
           "DATA_WRITE" => Ok(AuditLogConfigLogTypeEnum::DATAWRITE),
           "DATA_READ" => Ok(AuditLogConfigLogTypeEnum::DATAREAD),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AuditLogConfigLogTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceDisabledReasonEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. If the instance state is DISABLED, the reason for disabling the instance.
pub enum InstanceDisabledReasonEnum {
    

    /// This is an unknown reason for disabling.
    ///
    /// "DISABLED_REASON_UNSPECIFIED"
    #[serde(rename="DISABLED_REASON_UNSPECIFIED")]
    DISABLEDREASONUNSPECIFIED,
    

    /// The KMS key used by the instance is either revoked or denied access to
    ///
    /// "KMS_KEY_ISSUE"
    #[serde(rename="KMS_KEY_ISSUE")]
    KMSKEYISSUE,
}

impl AsRef<str> for InstanceDisabledReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceDisabledReasonEnum::DISABLEDREASONUNSPECIFIED => "DISABLED_REASON_UNSPECIFIED",
            InstanceDisabledReasonEnum::KMSKEYISSUE => "KMS_KEY_ISSUE",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceDisabledReasonEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISABLED_REASON_UNSPECIFIED" => Ok(InstanceDisabledReasonEnum::DISABLEDREASONUNSPECIFIED),
           "KMS_KEY_ISSUE" => Ok(InstanceDisabledReasonEnum::KMSKEYISSUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceDisabledReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstanceStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of this Data Fusion instance.
pub enum InstanceStateEnum {
    

    /// Instance does not have a state yet
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Instance is being created
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Instance is running and ready for requests
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// Instance creation failed
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// Instance is being deleted
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Instance is being upgraded
    ///
    /// "UPGRADING"
    #[serde(rename="UPGRADING")]
    UPGRADING,
    

    /// Instance is being restarted
    ///
    /// "RESTARTING"
    #[serde(rename="RESTARTING")]
    RESTARTING,
    

    /// Instance is being updated on customer request
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// Instance is being auto-updated
    ///
    /// "AUTO_UPDATING"
    #[serde(rename="AUTO_UPDATING")]
    AUTOUPDATING,
    

    /// Instance is being auto-upgraded
    ///
    /// "AUTO_UPGRADING"
    #[serde(rename="AUTO_UPGRADING")]
    AUTOUPGRADING,
    

    /// Instance is disabled
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for InstanceStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            InstanceStateEnum::CREATING => "CREATING",
            InstanceStateEnum::RUNNING => "RUNNING",
            InstanceStateEnum::FAILED => "FAILED",
            InstanceStateEnum::DELETING => "DELETING",
            InstanceStateEnum::UPGRADING => "UPGRADING",
            InstanceStateEnum::RESTARTING => "RESTARTING",
            InstanceStateEnum::UPDATING => "UPDATING",
            InstanceStateEnum::AUTOUPDATING => "AUTO_UPDATING",
            InstanceStateEnum::AUTOUPGRADING => "AUTO_UPGRADING",
            InstanceStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(InstanceStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(InstanceStateEnum::CREATING),
           "RUNNING" => Ok(InstanceStateEnum::RUNNING),
           "FAILED" => Ok(InstanceStateEnum::FAILED),
           "DELETING" => Ok(InstanceStateEnum::DELETING),
           "UPGRADING" => Ok(InstanceStateEnum::UPGRADING),
           "RESTARTING" => Ok(InstanceStateEnum::RESTARTING),
           "UPDATING" => Ok(InstanceStateEnum::UPDATING),
           "AUTO_UPDATING" => Ok(InstanceStateEnum::AUTOUPDATING),
           "AUTO_UPGRADING" => Ok(InstanceStateEnum::AUTOUPGRADING),
           "DISABLED" => Ok(InstanceStateEnum::DISABLED),
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


// region InstanceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Instance type.
pub enum InstanceTypeEnum {
    

    /// No type specified. The instance creation will fail.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines using point and click UI. However, there are certain limitations, such as fewer number of concurrent pipelines, no support for streaming pipelines, etc.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Enterprise Data Fusion instance. In Enterprise type, the user will have all features available, such as support for streaming pipelines, unlimited number of concurrent pipelines, etc.
    ///
    /// "ENTERPRISE"
    #[serde(rename="ENTERPRISE")]
    ENTERPRISE,
    

    /// Developer Data Fusion instance. In Developer type, the user will have all features available but with restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration pipelines at low cost.
    ///
    /// "DEVELOPER"
    #[serde(rename="DEVELOPER")]
    DEVELOPER,
}

impl AsRef<str> for InstanceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstanceTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            InstanceTypeEnum::BASIC => "BASIC",
            InstanceTypeEnum::ENTERPRISE => "ENTERPRISE",
            InstanceTypeEnum::DEVELOPER => "DEVELOPER",
        }
    }
}

impl std::convert::TryFrom< &str> for InstanceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(InstanceTypeEnum::TYPEUNSPECIFIED),
           "BASIC" => Ok(InstanceTypeEnum::BASIC),
           "ENTERPRISE" => Ok(InstanceTypeEnum::ENTERPRISE),
           "DEVELOPER" => Ok(InstanceTypeEnum::DEVELOPER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstanceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkConfigConnectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and the corresponding tenant project from a predefined list of available connection modes. If this field is unspecified for a private instance, VPC peering is used.
pub enum NetworkConfigConnectionTypeEnum {
    

    /// No specific connection type was requested, the default value of VPC_PEERING is chosen.
    ///
    /// "CONNECTION_TYPE_UNSPECIFIED"
    #[serde(rename="CONNECTION_TYPE_UNSPECIFIED")]
    CONNECTIONTYPEUNSPECIFIED,
    

    /// Requests the use of VPC peerings for connecting the consumer and tenant projects.
    ///
    /// "VPC_PEERING"
    #[serde(rename="VPC_PEERING")]
    VPCPEERING,
    

    /// Requests the use of Private Service Connect Interfaces for connecting the consumer and tenant projects.
    ///
    /// "PRIVATE_SERVICE_CONNECT_INTERFACES"
    #[serde(rename="PRIVATE_SERVICE_CONNECT_INTERFACES")]
    PRIVATESERVICECONNECTINTERFACES,
}

impl AsRef<str> for NetworkConfigConnectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkConfigConnectionTypeEnum::CONNECTIONTYPEUNSPECIFIED => "CONNECTION_TYPE_UNSPECIFIED",
            NetworkConfigConnectionTypeEnum::VPCPEERING => "VPC_PEERING",
            NetworkConfigConnectionTypeEnum::PRIVATESERVICECONNECTINTERFACES => "PRIVATE_SERVICE_CONNECT_INTERFACES",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkConfigConnectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_TYPE_UNSPECIFIED" => Ok(NetworkConfigConnectionTypeEnum::CONNECTIONTYPEUNSPECIFIED),
           "VPC_PEERING" => Ok(NetworkConfigConnectionTypeEnum::VPCPEERING),
           "PRIVATE_SERVICE_CONNECT_INTERFACES" => Ok(NetworkConfigConnectionTypeEnum::PRIVATESERVICECONNECTINTERFACES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkConfigConnectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VersionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type represents the release availability of the version
pub enum VersionTypeEnum {
    

    /// Version does not have availability yet
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Version is under development and not considered stable
    ///
    /// "TYPE_PREVIEW"
    #[serde(rename="TYPE_PREVIEW")]
    TYPEPREVIEW,
    

    /// Version is available for public use Version is under development and not considered stable
    ///
    /// "TYPE_GENERAL_AVAILABILITY"
    #[serde(rename="TYPE_GENERAL_AVAILABILITY")]
    TYPEGENERALAVAILABILITY,
}

impl AsRef<str> for VersionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VersionTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            VersionTypeEnum::TYPEPREVIEW => "TYPE_PREVIEW",
            VersionTypeEnum::TYPEGENERALAVAILABILITY => "TYPE_GENERAL_AVAILABILITY",
        }
    }
}

impl std::convert::TryFrom< &str> for VersionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(VersionTypeEnum::TYPEUNSPECIFIED),
           "TYPE_PREVIEW" => Ok(VersionTypeEnum::TYPEPREVIEW),
           "TYPE_GENERAL_AVAILABILITY" => Ok(VersionTypeEnum::TYPEGENERALAVAILABILITY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VersionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// By default, only basic information about a namespace is returned (e.g. name). When `NAMESPACE_VIEW_FULL` is specified, additional information associated with a namespace gets returned (e.g. IAM policy set on the namespace)
pub enum ProjectViewEnum {
    

    /// Default/unset value, which will use BASIC view.
    ///
    /// "NAMESPACE_VIEW_UNSPECIFIED"
    #[serde(rename="NAMESPACE_VIEW_UNSPECIFIED")]
    NAMESPACEVIEWUNSPECIFIED,
    

    /// Show the most basic metadata of a namespace
    ///
    /// "NAMESPACE_VIEW_BASIC"
    #[serde(rename="NAMESPACE_VIEW_BASIC")]
    NAMESPACEVIEWBASIC,
    

    /// Returns all metadata of a namespace
    ///
    /// "NAMESPACE_VIEW_FULL"
    #[serde(rename="NAMESPACE_VIEW_FULL")]
    NAMESPACEVIEWFULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::NAMESPACEVIEWUNSPECIFIED => "NAMESPACE_VIEW_UNSPECIFIED",
            ProjectViewEnum::NAMESPACEVIEWBASIC => "NAMESPACE_VIEW_BASIC",
            ProjectViewEnum::NAMESPACEVIEWFULL => "NAMESPACE_VIEW_FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NAMESPACE_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::NAMESPACEVIEWUNSPECIFIED),
           "NAMESPACE_VIEW_BASIC" => Ok(ProjectViewEnum::NAMESPACEVIEWBASIC),
           "NAMESPACE_VIEW_FULL" => Ok(ProjectViewEnum::NAMESPACEVIEWFULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


