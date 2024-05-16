use super::*;



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


// region CloudFunctionDockerRegistryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Docker Registry to use for this deployment. If unspecified, it defaults to `ARTIFACT_REGISTRY`. If `docker_repository` field is specified, this field should either be left unspecified or set to `ARTIFACT_REGISTRY`.
pub enum CloudFunctionDockerRegistryEnum {
    

    /// Unspecified.
    ///
    /// "DOCKER_REGISTRY_UNSPECIFIED"
    #[serde(rename="DOCKER_REGISTRY_UNSPECIFIED")]
    DOCKERREGISTRYUNSPECIFIED,
    

    /// Docker images will be stored in multi-regional Container Registry repositories named `gcf`.
    ///
    /// "CONTAINER_REGISTRY"
    #[serde(rename="CONTAINER_REGISTRY")]
    CONTAINERREGISTRY,
    

    /// Docker images will be stored in regional Artifact Registry repositories. By default, GCF will create and use repositories named `gcf-artifacts` in every region in which a function is deployed. But the repository to use can also be specified by the user using the `docker_repository` field.
    ///
    /// "ARTIFACT_REGISTRY"
    #[serde(rename="ARTIFACT_REGISTRY")]
    ARTIFACTREGISTRY,
}

impl AsRef<str> for CloudFunctionDockerRegistryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudFunctionDockerRegistryEnum::DOCKERREGISTRYUNSPECIFIED => "DOCKER_REGISTRY_UNSPECIFIED",
            CloudFunctionDockerRegistryEnum::CONTAINERREGISTRY => "CONTAINER_REGISTRY",
            CloudFunctionDockerRegistryEnum::ARTIFACTREGISTRY => "ARTIFACT_REGISTRY",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudFunctionDockerRegistryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DOCKER_REGISTRY_UNSPECIFIED" => Ok(CloudFunctionDockerRegistryEnum::DOCKERREGISTRYUNSPECIFIED),
           "CONTAINER_REGISTRY" => Ok(CloudFunctionDockerRegistryEnum::CONTAINERREGISTRY),
           "ARTIFACT_REGISTRY" => Ok(CloudFunctionDockerRegistryEnum::ARTIFACTREGISTRY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudFunctionDockerRegistryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudFunctionIngressSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The ingress settings for the function, controlling what traffic can reach it.
pub enum CloudFunctionIngressSettingsEnum {
    

    /// Unspecified.
    ///
    /// "INGRESS_SETTINGS_UNSPECIFIED"
    #[serde(rename="INGRESS_SETTINGS_UNSPECIFIED")]
    INGRESSSETTINGSUNSPECIFIED,
    

    /// Allow HTTP traffic from public and private sources.
    ///
    /// "ALLOW_ALL"
    #[serde(rename="ALLOW_ALL")]
    ALLOWALL,
    

    /// Allow HTTP traffic from only private VPC sources.
    ///
    /// "ALLOW_INTERNAL_ONLY"
    #[serde(rename="ALLOW_INTERNAL_ONLY")]
    ALLOWINTERNALONLY,
    

    /// Allow HTTP traffic from private VPC sources and through GCLB.
    ///
    /// "ALLOW_INTERNAL_AND_GCLB"
    #[serde(rename="ALLOW_INTERNAL_AND_GCLB")]
    ALLOWINTERNALANDGCLB,
}

impl AsRef<str> for CloudFunctionIngressSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudFunctionIngressSettingsEnum::INGRESSSETTINGSUNSPECIFIED => "INGRESS_SETTINGS_UNSPECIFIED",
            CloudFunctionIngressSettingsEnum::ALLOWALL => "ALLOW_ALL",
            CloudFunctionIngressSettingsEnum::ALLOWINTERNALONLY => "ALLOW_INTERNAL_ONLY",
            CloudFunctionIngressSettingsEnum::ALLOWINTERNALANDGCLB => "ALLOW_INTERNAL_AND_GCLB",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudFunctionIngressSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INGRESS_SETTINGS_UNSPECIFIED" => Ok(CloudFunctionIngressSettingsEnum::INGRESSSETTINGSUNSPECIFIED),
           "ALLOW_ALL" => Ok(CloudFunctionIngressSettingsEnum::ALLOWALL),
           "ALLOW_INTERNAL_ONLY" => Ok(CloudFunctionIngressSettingsEnum::ALLOWINTERNALONLY),
           "ALLOW_INTERNAL_AND_GCLB" => Ok(CloudFunctionIngressSettingsEnum::ALLOWINTERNALANDGCLB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudFunctionIngressSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudFunctionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Status of the function deployment.
pub enum CloudFunctionStatusEnum {
    

    /// Not specified. Invalid state.
    ///
    /// "CLOUD_FUNCTION_STATUS_UNSPECIFIED"
    #[serde(rename="CLOUD_FUNCTION_STATUS_UNSPECIFIED")]
    CLOUDFUNCTIONSTATUSUNSPECIFIED,
    

    /// Function has been successfully deployed and is serving.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Function deployment failed and the function isn’t serving.
    ///
    /// "OFFLINE"
    #[serde(rename="OFFLINE")]
    OFFLINE,
    

    /// Function is being created or updated.
    ///
    /// "DEPLOY_IN_PROGRESS"
    #[serde(rename="DEPLOY_IN_PROGRESS")]
    DEPLOYINPROGRESS,
    

    /// Function is being deleted.
    ///
    /// "DELETE_IN_PROGRESS"
    #[serde(rename="DELETE_IN_PROGRESS")]
    DELETEINPROGRESS,
    

    /// Function deployment failed and the function serving state is undefined. The function should be updated or deleted to move it out of this state.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
}

impl AsRef<str> for CloudFunctionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudFunctionStatusEnum::CLOUDFUNCTIONSTATUSUNSPECIFIED => "CLOUD_FUNCTION_STATUS_UNSPECIFIED",
            CloudFunctionStatusEnum::ACTIVE => "ACTIVE",
            CloudFunctionStatusEnum::OFFLINE => "OFFLINE",
            CloudFunctionStatusEnum::DEPLOYINPROGRESS => "DEPLOY_IN_PROGRESS",
            CloudFunctionStatusEnum::DELETEINPROGRESS => "DELETE_IN_PROGRESS",
            CloudFunctionStatusEnum::UNKNOWN => "UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudFunctionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CLOUD_FUNCTION_STATUS_UNSPECIFIED" => Ok(CloudFunctionStatusEnum::CLOUDFUNCTIONSTATUSUNSPECIFIED),
           "ACTIVE" => Ok(CloudFunctionStatusEnum::ACTIVE),
           "OFFLINE" => Ok(CloudFunctionStatusEnum::OFFLINE),
           "DEPLOY_IN_PROGRESS" => Ok(CloudFunctionStatusEnum::DEPLOYINPROGRESS),
           "DELETE_IN_PROGRESS" => Ok(CloudFunctionStatusEnum::DELETEINPROGRESS),
           "UNKNOWN" => Ok(CloudFunctionStatusEnum::UNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudFunctionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CloudFunctionVpcConnectorEgressSettingsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The egress settings for the connector, controlling what traffic is diverted through it.
pub enum CloudFunctionVpcConnectorEgressSettingsEnum {
    

    /// Unspecified.
    ///
    /// "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED"
    #[serde(rename="VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED")]
    VPCCONNECTOREGRESSSETTINGSUNSPECIFIED,
    

    /// Use the VPC Access Connector only for private IP space from RFC1918.
    ///
    /// "PRIVATE_RANGES_ONLY"
    #[serde(rename="PRIVATE_RANGES_ONLY")]
    PRIVATERANGESONLY,
    

    /// Force the use of VPC Access Connector for all egress traffic from the function.
    ///
    /// "ALL_TRAFFIC"
    #[serde(rename="ALL_TRAFFIC")]
    ALLTRAFFIC,
}

impl AsRef<str> for CloudFunctionVpcConnectorEgressSettingsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CloudFunctionVpcConnectorEgressSettingsEnum::VPCCONNECTOREGRESSSETTINGSUNSPECIFIED => "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED",
            CloudFunctionVpcConnectorEgressSettingsEnum::PRIVATERANGESONLY => "PRIVATE_RANGES_ONLY",
            CloudFunctionVpcConnectorEgressSettingsEnum::ALLTRAFFIC => "ALL_TRAFFIC",
        }
    }
}

impl std::convert::TryFrom< &str> for CloudFunctionVpcConnectorEgressSettingsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED" => Ok(CloudFunctionVpcConnectorEgressSettingsEnum::VPCCONNECTOREGRESSSETTINGSUNSPECIFIED),
           "PRIVATE_RANGES_ONLY" => Ok(CloudFunctionVpcConnectorEgressSettingsEnum::PRIVATERANGESONLY),
           "ALL_TRAFFIC" => Ok(CloudFunctionVpcConnectorEgressSettingsEnum::ALLTRAFFIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CloudFunctionVpcConnectorEgressSettingsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HttpsTriggerSecurityLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The security level for the function.
pub enum HttpsTriggerSecurityLevelEnum {
    

    /// Unspecified.
    ///
    /// "SECURITY_LEVEL_UNSPECIFIED"
    #[serde(rename="SECURITY_LEVEL_UNSPECIFIED")]
    SECURITYLEVELUNSPECIFIED,
    

    /// Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect.
    ///
    /// "SECURE_ALWAYS"
    #[serde(rename="SECURE_ALWAYS")]
    SECUREALWAYS,
    

    /// Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly.
    ///
    /// "SECURE_OPTIONAL"
    #[serde(rename="SECURE_OPTIONAL")]
    SECUREOPTIONAL,
}

impl AsRef<str> for HttpsTriggerSecurityLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HttpsTriggerSecurityLevelEnum::SECURITYLEVELUNSPECIFIED => "SECURITY_LEVEL_UNSPECIFIED",
            HttpsTriggerSecurityLevelEnum::SECUREALWAYS => "SECURE_ALWAYS",
            HttpsTriggerSecurityLevelEnum::SECUREOPTIONAL => "SECURE_OPTIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for HttpsTriggerSecurityLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SECURITY_LEVEL_UNSPECIFIED" => Ok(HttpsTriggerSecurityLevelEnum::SECURITYLEVELUNSPECIFIED),
           "SECURE_ALWAYS" => Ok(HttpsTriggerSecurityLevelEnum::SECUREALWAYS),
           "SECURE_OPTIONAL" => Ok(HttpsTriggerSecurityLevelEnum::SECUREOPTIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HttpsTriggerSecurityLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


