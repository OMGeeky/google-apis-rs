use super::*;



// region AirflowMetadataRetentionPolicyConfigRetentionModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Retention can be either enabled or disabled.
pub enum AirflowMetadataRetentionPolicyConfigRetentionModeEnum {
    

    /// Default mode doesn't change environment parameters.
    ///
    /// "RETENTION_MODE_UNSPECIFIED"
    #[serde(rename="RETENTION_MODE_UNSPECIFIED")]
    RETENTIONMODEUNSPECIFIED,
    

    /// Retention policy is enabled.
    ///
    /// "RETENTION_MODE_ENABLED"
    #[serde(rename="RETENTION_MODE_ENABLED")]
    RETENTIONMODEENABLED,
    

    /// Retention policy is disabled.
    ///
    /// "RETENTION_MODE_DISABLED"
    #[serde(rename="RETENTION_MODE_DISABLED")]
    RETENTIONMODEDISABLED,
}

impl AsRef<str> for AirflowMetadataRetentionPolicyConfigRetentionModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AirflowMetadataRetentionPolicyConfigRetentionModeEnum::RETENTIONMODEUNSPECIFIED => "RETENTION_MODE_UNSPECIFIED",
            AirflowMetadataRetentionPolicyConfigRetentionModeEnum::RETENTIONMODEENABLED => "RETENTION_MODE_ENABLED",
            AirflowMetadataRetentionPolicyConfigRetentionModeEnum::RETENTIONMODEDISABLED => "RETENTION_MODE_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for AirflowMetadataRetentionPolicyConfigRetentionModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RETENTION_MODE_UNSPECIFIED" => Ok(AirflowMetadataRetentionPolicyConfigRetentionModeEnum::RETENTIONMODEUNSPECIFIED),
           "RETENTION_MODE_ENABLED" => Ok(AirflowMetadataRetentionPolicyConfigRetentionModeEnum::RETENTIONMODEENABLED),
           "RETENTION_MODE_DISABLED" => Ok(AirflowMetadataRetentionPolicyConfigRetentionModeEnum::RETENTIONMODEDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AirflowMetadataRetentionPolicyConfigRetentionModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComposerWorkloadTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of a workload.
pub enum ComposerWorkloadTypeEnum {
    

    /// Not able to determine the type of the workload.
    ///
    /// "COMPOSER_WORKLOAD_TYPE_UNSPECIFIED"
    #[serde(rename="COMPOSER_WORKLOAD_TYPE_UNSPECIFIED")]
    COMPOSERWORKLOADTYPEUNSPECIFIED,
    

    /// Celery worker.
    ///
    /// "CELERY_WORKER"
    #[serde(rename="CELERY_WORKER")]
    CELERYWORKER,
    

    /// Kubernetes worker.
    ///
    /// "KUBERNETES_WORKER"
    #[serde(rename="KUBERNETES_WORKER")]
    KUBERNETESWORKER,
    

    /// Workload created by Kubernetes Pod Operator.
    ///
    /// "KUBERNETES_OPERATOR_POD"
    #[serde(rename="KUBERNETES_OPERATOR_POD")]
    KUBERNETESOPERATORPOD,
    

    /// Airflow scheduler.
    ///
    /// "SCHEDULER"
    #[serde(rename="SCHEDULER")]
    SCHEDULER,
    

    /// Airflow Dag processor.
    ///
    /// "DAG_PROCESSOR"
    #[serde(rename="DAG_PROCESSOR")]
    DAGPROCESSOR,
    

    /// Airflow triggerer.
    ///
    /// "TRIGGERER"
    #[serde(rename="TRIGGERER")]
    TRIGGERER,
    

    /// Airflow web server UI.
    ///
    /// "WEB_SERVER"
    #[serde(rename="WEB_SERVER")]
    WEBSERVER,
    

    /// Redis.
    ///
    /// "REDIS"
    #[serde(rename="REDIS")]
    REDIS,
}

impl AsRef<str> for ComposerWorkloadTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComposerWorkloadTypeEnum::COMPOSERWORKLOADTYPEUNSPECIFIED => "COMPOSER_WORKLOAD_TYPE_UNSPECIFIED",
            ComposerWorkloadTypeEnum::CELERYWORKER => "CELERY_WORKER",
            ComposerWorkloadTypeEnum::KUBERNETESWORKER => "KUBERNETES_WORKER",
            ComposerWorkloadTypeEnum::KUBERNETESOPERATORPOD => "KUBERNETES_OPERATOR_POD",
            ComposerWorkloadTypeEnum::SCHEDULER => "SCHEDULER",
            ComposerWorkloadTypeEnum::DAGPROCESSOR => "DAG_PROCESSOR",
            ComposerWorkloadTypeEnum::TRIGGERER => "TRIGGERER",
            ComposerWorkloadTypeEnum::WEBSERVER => "WEB_SERVER",
            ComposerWorkloadTypeEnum::REDIS => "REDIS",
        }
    }
}

impl std::convert::TryFrom< &str> for ComposerWorkloadTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPOSER_WORKLOAD_TYPE_UNSPECIFIED" => Ok(ComposerWorkloadTypeEnum::COMPOSERWORKLOADTYPEUNSPECIFIED),
           "CELERY_WORKER" => Ok(ComposerWorkloadTypeEnum::CELERYWORKER),
           "KUBERNETES_WORKER" => Ok(ComposerWorkloadTypeEnum::KUBERNETESWORKER),
           "KUBERNETES_OPERATOR_POD" => Ok(ComposerWorkloadTypeEnum::KUBERNETESOPERATORPOD),
           "SCHEDULER" => Ok(ComposerWorkloadTypeEnum::SCHEDULER),
           "DAG_PROCESSOR" => Ok(ComposerWorkloadTypeEnum::DAGPROCESSOR),
           "TRIGGERER" => Ok(ComposerWorkloadTypeEnum::TRIGGERER),
           "WEB_SERVER" => Ok(ComposerWorkloadTypeEnum::WEBSERVER),
           "REDIS" => Ok(ComposerWorkloadTypeEnum::REDIS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComposerWorkloadTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ComposerWorkloadStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Workload state.
pub enum ComposerWorkloadStatusStateEnum {
    

    /// Not able to determine the status of the workload.
    ///
    /// "COMPOSER_WORKLOAD_STATE_UNSPECIFIED"
    #[serde(rename="COMPOSER_WORKLOAD_STATE_UNSPECIFIED")]
    COMPOSERWORKLOADSTATEUNSPECIFIED,
    

    /// Workload is in pending state and has not yet started.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// Workload is running fine.
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// Workload is running but there are some non-critical problems.
    ///
    /// "WARNING"
    #[serde(rename="WARNING")]
    WARNING,
    

    /// Workload is not running due to an error.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
    

    /// Workload has finished execution with success.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// Workload has finished execution with failure.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
}

impl AsRef<str> for ComposerWorkloadStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ComposerWorkloadStatusStateEnum::COMPOSERWORKLOADSTATEUNSPECIFIED => "COMPOSER_WORKLOAD_STATE_UNSPECIFIED",
            ComposerWorkloadStatusStateEnum::PENDING => "PENDING",
            ComposerWorkloadStatusStateEnum::OK => "OK",
            ComposerWorkloadStatusStateEnum::WARNING => "WARNING",
            ComposerWorkloadStatusStateEnum::ERROR => "ERROR",
            ComposerWorkloadStatusStateEnum::SUCCEEDED => "SUCCEEDED",
            ComposerWorkloadStatusStateEnum::FAILED => "FAILED",
        }
    }
}

impl std::convert::TryFrom< &str> for ComposerWorkloadStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPOSER_WORKLOAD_STATE_UNSPECIFIED" => Ok(ComposerWorkloadStatusStateEnum::COMPOSERWORKLOADSTATEUNSPECIFIED),
           "PENDING" => Ok(ComposerWorkloadStatusStateEnum::PENDING),
           "OK" => Ok(ComposerWorkloadStatusStateEnum::OK),
           "WARNING" => Ok(ComposerWorkloadStatusStateEnum::WARNING),
           "ERROR" => Ok(ComposerWorkloadStatusStateEnum::ERROR),
           "SUCCEEDED" => Ok(ComposerWorkloadStatusStateEnum::SUCCEEDED),
           "FAILED" => Ok(ComposerWorkloadStatusStateEnum::FAILED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ComposerWorkloadStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the environment.
pub enum EnvironmentStateEnum {
    

    /// The state of the environment is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The environment is in the process of being created.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The environment is currently running and healthy. It is ready for use.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The environment is being updated. It remains usable but cannot receive additional update requests or be deleted at this time.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// The environment is undergoing deletion. It cannot be used.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// The environment has encountered an error and cannot be used.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for EnvironmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            EnvironmentStateEnum::CREATING => "CREATING",
            EnvironmentStateEnum::RUNNING => "RUNNING",
            EnvironmentStateEnum::UPDATING => "UPDATING",
            EnvironmentStateEnum::DELETING => "DELETING",
            EnvironmentStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(EnvironmentStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(EnvironmentStateEnum::CREATING),
           "RUNNING" => Ok(EnvironmentStateEnum::RUNNING),
           "UPDATING" => Ok(EnvironmentStateEnum::UPDATING),
           "DELETING" => Ok(EnvironmentStateEnum::DELETING),
           "ERROR" => Ok(EnvironmentStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentConfigEnvironmentSizeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The size of the Cloud Composer environment. This field is supported for Cloud Composer environments in versions composer-2.*.*-airflow-*.*.* and newer.
pub enum EnvironmentConfigEnvironmentSizeEnum {
    

    /// The size of the environment is unspecified.
    ///
    /// "ENVIRONMENT_SIZE_UNSPECIFIED"
    #[serde(rename="ENVIRONMENT_SIZE_UNSPECIFIED")]
    ENVIRONMENTSIZEUNSPECIFIED,
    

    /// The environment size is small.
    ///
    /// "ENVIRONMENT_SIZE_SMALL"
    #[serde(rename="ENVIRONMENT_SIZE_SMALL")]
    ENVIRONMENTSIZESMALL,
    

    /// The environment size is medium.
    ///
    /// "ENVIRONMENT_SIZE_MEDIUM"
    #[serde(rename="ENVIRONMENT_SIZE_MEDIUM")]
    ENVIRONMENTSIZEMEDIUM,
    

    /// The environment size is large.
    ///
    /// "ENVIRONMENT_SIZE_LARGE"
    #[serde(rename="ENVIRONMENT_SIZE_LARGE")]
    ENVIRONMENTSIZELARGE,
}

impl AsRef<str> for EnvironmentConfigEnvironmentSizeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZEUNSPECIFIED => "ENVIRONMENT_SIZE_UNSPECIFIED",
            EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZESMALL => "ENVIRONMENT_SIZE_SMALL",
            EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZEMEDIUM => "ENVIRONMENT_SIZE_MEDIUM",
            EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZELARGE => "ENVIRONMENT_SIZE_LARGE",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentConfigEnvironmentSizeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENVIRONMENT_SIZE_UNSPECIFIED" => Ok(EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZEUNSPECIFIED),
           "ENVIRONMENT_SIZE_SMALL" => Ok(EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZESMALL),
           "ENVIRONMENT_SIZE_MEDIUM" => Ok(EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZEMEDIUM),
           "ENVIRONMENT_SIZE_LARGE" => Ok(EnvironmentConfigEnvironmentSizeEnum::ENVIRONMENTSIZELARGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentConfigEnvironmentSizeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EnvironmentConfigResilienceModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Resilience mode of the Cloud Composer Environment. This field is supported for Cloud Composer environments in versions composer-2.2.0-airflow-*.*.* and newer.
pub enum EnvironmentConfigResilienceModeEnum {
    

    /// Default mode doesn't change environment parameters.
    ///
    /// "RESILIENCE_MODE_UNSPECIFIED"
    #[serde(rename="RESILIENCE_MODE_UNSPECIFIED")]
    RESILIENCEMODEUNSPECIFIED,
    

    /// Enabled High Resilience mode, including Cloud SQL HA.
    ///
    /// "HIGH_RESILIENCE"
    #[serde(rename="HIGH_RESILIENCE")]
    HIGHRESILIENCE,
}

impl AsRef<str> for EnvironmentConfigResilienceModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EnvironmentConfigResilienceModeEnum::RESILIENCEMODEUNSPECIFIED => "RESILIENCE_MODE_UNSPECIFIED",
            EnvironmentConfigResilienceModeEnum::HIGHRESILIENCE => "HIGH_RESILIENCE",
        }
    }
}

impl std::convert::TryFrom< &str> for EnvironmentConfigResilienceModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESILIENCE_MODE_UNSPECIFIED" => Ok(EnvironmentConfigResilienceModeEnum::RESILIENCEMODEUNSPECIFIED),
           "HIGH_RESILIENCE" => Ok(EnvironmentConfigResilienceModeEnum::HIGHRESILIENCE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EnvironmentConfigResilienceModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NetworkingConfigConnectionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Indicates the user requested specifc connection type between Tenant and Customer projects. You cannot set networking connection type in public IP environment.
pub enum NetworkingConfigConnectionTypeEnum {
    

    /// No specific connection type was requested, so the environment uses the default value corresponding to the rest of its configuration.
    ///
    /// "CONNECTION_TYPE_UNSPECIFIED"
    #[serde(rename="CONNECTION_TYPE_UNSPECIFIED")]
    CONNECTIONTYPEUNSPECIFIED,
    

    /// Requests the use of VPC peerings for connecting the Customer and Tenant projects.
    ///
    /// "VPC_PEERING"
    #[serde(rename="VPC_PEERING")]
    VPCPEERING,
    

    /// Requests the use of Private Service Connect for connecting the Customer and Tenant projects.
    ///
    /// "PRIVATE_SERVICE_CONNECT"
    #[serde(rename="PRIVATE_SERVICE_CONNECT")]
    PRIVATESERVICECONNECT,
}

impl AsRef<str> for NetworkingConfigConnectionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NetworkingConfigConnectionTypeEnum::CONNECTIONTYPEUNSPECIFIED => "CONNECTION_TYPE_UNSPECIFIED",
            NetworkingConfigConnectionTypeEnum::VPCPEERING => "VPC_PEERING",
            NetworkingConfigConnectionTypeEnum::PRIVATESERVICECONNECT => "PRIVATE_SERVICE_CONNECT",
        }
    }
}

impl std::convert::TryFrom< &str> for NetworkingConfigConnectionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTION_TYPE_UNSPECIFIED" => Ok(NetworkingConfigConnectionTypeEnum::CONNECTIONTYPEUNSPECIFIED),
           "VPC_PEERING" => Ok(NetworkingConfigConnectionTypeEnum::VPCPEERING),
           "PRIVATE_SERVICE_CONNECT" => Ok(NetworkingConfigConnectionTypeEnum::PRIVATESERVICECONNECT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NetworkingConfigConnectionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SoftwareConfigWebServerPluginsModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Whether or not the web server uses custom plugins. If unspecified, the field defaults to `PLUGINS_ENABLED`. This field is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
pub enum SoftwareConfigWebServerPluginsModeEnum {
    

    /// Default mode.
    ///
    /// "WEB_SERVER_PLUGINS_MODE_UNSPECIFIED"
    #[serde(rename="WEB_SERVER_PLUGINS_MODE_UNSPECIFIED")]
    WEBSERVERPLUGINSMODEUNSPECIFIED,
    

    /// Web server plugins are not supported.
    ///
    /// "PLUGINS_DISABLED"
    #[serde(rename="PLUGINS_DISABLED")]
    PLUGINSDISABLED,
    

    /// Web server plugins are supported.
    ///
    /// "PLUGINS_ENABLED"
    #[serde(rename="PLUGINS_ENABLED")]
    PLUGINSENABLED,
}

impl AsRef<str> for SoftwareConfigWebServerPluginsModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SoftwareConfigWebServerPluginsModeEnum::WEBSERVERPLUGINSMODEUNSPECIFIED => "WEB_SERVER_PLUGINS_MODE_UNSPECIFIED",
            SoftwareConfigWebServerPluginsModeEnum::PLUGINSDISABLED => "PLUGINS_DISABLED",
            SoftwareConfigWebServerPluginsModeEnum::PLUGINSENABLED => "PLUGINS_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for SoftwareConfigWebServerPluginsModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "WEB_SERVER_PLUGINS_MODE_UNSPECIFIED" => Ok(SoftwareConfigWebServerPluginsModeEnum::WEBSERVERPLUGINSMODEUNSPECIFIED),
           "PLUGINS_DISABLED" => Ok(SoftwareConfigWebServerPluginsModeEnum::PLUGINSDISABLED),
           "PLUGINS_ENABLED" => Ok(SoftwareConfigWebServerPluginsModeEnum::PLUGINSENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SoftwareConfigWebServerPluginsModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TaskLogsRetentionConfigStorageModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The mode of storage for Airflow workers task logs.
pub enum TaskLogsRetentionConfigStorageModeEnum {
    

    /// This configuration is not specified by the user.
    ///
    /// "TASK_LOGS_STORAGE_MODE_UNSPECIFIED"
    #[serde(rename="TASK_LOGS_STORAGE_MODE_UNSPECIFIED")]
    TASKLOGSSTORAGEMODEUNSPECIFIED,
    

    /// Store task logs in Cloud Logging and in the environment's Cloud Storage bucket.
    ///
    /// "CLOUD_LOGGING_AND_CLOUD_STORAGE"
    #[serde(rename="CLOUD_LOGGING_AND_CLOUD_STORAGE")]
    CLOUDLOGGINGANDCLOUDSTORAGE,
    

    /// Store task logs in Cloud Logging only.
    ///
    /// "CLOUD_LOGGING_ONLY"
    #[serde(rename="CLOUD_LOGGING_ONLY")]
    CLOUDLOGGINGONLY,
}

impl AsRef<str> for TaskLogsRetentionConfigStorageModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TaskLogsRetentionConfigStorageModeEnum::TASKLOGSSTORAGEMODEUNSPECIFIED => "TASK_LOGS_STORAGE_MODE_UNSPECIFIED",
            TaskLogsRetentionConfigStorageModeEnum::CLOUDLOGGINGANDCLOUDSTORAGE => "CLOUD_LOGGING_AND_CLOUD_STORAGE",
            TaskLogsRetentionConfigStorageModeEnum::CLOUDLOGGINGONLY => "CLOUD_LOGGING_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for TaskLogsRetentionConfigStorageModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TASK_LOGS_STORAGE_MODE_UNSPECIFIED" => Ok(TaskLogsRetentionConfigStorageModeEnum::TASKLOGSSTORAGEMODEUNSPECIFIED),
           "CLOUD_LOGGING_AND_CLOUD_STORAGE" => Ok(TaskLogsRetentionConfigStorageModeEnum::CLOUDLOGGINGANDCLOUDSTORAGE),
           "CLOUD_LOGGING_ONLY" => Ok(TaskLogsRetentionConfigStorageModeEnum::CLOUDLOGGINGONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TaskLogsRetentionConfigStorageModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


