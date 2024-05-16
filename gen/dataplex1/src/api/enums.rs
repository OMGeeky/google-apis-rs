use super::*;



// region GoogleCloudDataplexV1ActionCategoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The category of issue associated with the action.
pub enum GoogleCloudDataplexV1ActionCategoryEnum {
    

    /// Unspecified category.
    ///
    /// "CATEGORY_UNSPECIFIED"
    #[serde(rename="CATEGORY_UNSPECIFIED")]
    CATEGORYUNSPECIFIED,
    

    /// Resource management related issues.
    ///
    /// "RESOURCE_MANAGEMENT"
    #[serde(rename="RESOURCE_MANAGEMENT")]
    RESOURCEMANAGEMENT,
    

    /// Security policy related issues.
    ///
    /// "SECURITY_POLICY"
    #[serde(rename="SECURITY_POLICY")]
    SECURITYPOLICY,
    

    /// Data and discovery related issues.
    ///
    /// "DATA_DISCOVERY"
    #[serde(rename="DATA_DISCOVERY")]
    DATADISCOVERY,
}

impl AsRef<str> for GoogleCloudDataplexV1ActionCategoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ActionCategoryEnum::CATEGORYUNSPECIFIED => "CATEGORY_UNSPECIFIED",
            GoogleCloudDataplexV1ActionCategoryEnum::RESOURCEMANAGEMENT => "RESOURCE_MANAGEMENT",
            GoogleCloudDataplexV1ActionCategoryEnum::SECURITYPOLICY => "SECURITY_POLICY",
            GoogleCloudDataplexV1ActionCategoryEnum::DATADISCOVERY => "DATA_DISCOVERY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ActionCategoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CATEGORY_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ActionCategoryEnum::CATEGORYUNSPECIFIED),
           "RESOURCE_MANAGEMENT" => Ok(GoogleCloudDataplexV1ActionCategoryEnum::RESOURCEMANAGEMENT),
           "SECURITY_POLICY" => Ok(GoogleCloudDataplexV1ActionCategoryEnum::SECURITYPOLICY),
           "DATA_DISCOVERY" => Ok(GoogleCloudDataplexV1ActionCategoryEnum::DATADISCOVERY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ActionCategoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether the action relates to a schema that is incompatible or modified.
pub enum GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum {
    

    /// Schema change unspecified.
    ///
    /// "SCHEMA_CHANGE_UNSPECIFIED"
    #[serde(rename="SCHEMA_CHANGE_UNSPECIFIED")]
    SCHEMACHANGEUNSPECIFIED,
    

    /// Newly discovered schema is incompatible with existing schema.
    ///
    /// "INCOMPATIBLE"
    #[serde(rename="INCOMPATIBLE")]
    INCOMPATIBLE,
    

    /// Newly discovered schema has changed from existing schema for data in a curated zone.
    ///
    /// "MODIFIED"
    #[serde(rename="MODIFIED")]
    MODIFIED,
}

impl AsRef<str> for GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum::SCHEMACHANGEUNSPECIFIED => "SCHEMA_CHANGE_UNSPECIFIED",
            GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum::INCOMPATIBLE => "INCOMPATIBLE",
            GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum::MODIFIED => "MODIFIED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCHEMA_CHANGE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum::SCHEMACHANGEUNSPECIFIED),
           "INCOMPATIBLE" => Ok(GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum::INCOMPATIBLE),
           "MODIFIED" => Ok(GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum::MODIFIED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ActionIncompatibleDataSchemaSchemaChangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The issue type of InvalidDataPartition.
pub enum GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum {
    

    /// PartitionStructure unspecified.
    ///
    /// "PARTITION_STRUCTURE_UNSPECIFIED"
    #[serde(rename="PARTITION_STRUCTURE_UNSPECIFIED")]
    PARTITIONSTRUCTUREUNSPECIFIED,
    

    /// Consistent hive-style partition definition (both raw and curated zone).
    ///
    /// "CONSISTENT_KEYS"
    #[serde(rename="CONSISTENT_KEYS")]
    CONSISTENTKEYS,
    

    /// Hive style partition definition (curated zone only).
    ///
    /// "HIVE_STYLE_KEYS"
    #[serde(rename="HIVE_STYLE_KEYS")]
    HIVESTYLEKEYS,
}

impl AsRef<str> for GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum::PARTITIONSTRUCTUREUNSPECIFIED => "PARTITION_STRUCTURE_UNSPECIFIED",
            GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum::CONSISTENTKEYS => "CONSISTENT_KEYS",
            GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum::HIVESTYLEKEYS => "HIVE_STYLE_KEYS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTITION_STRUCTURE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum::PARTITIONSTRUCTUREUNSPECIFIED),
           "CONSISTENT_KEYS" => Ok(GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum::CONSISTENTKEYS),
           "HIVE_STYLE_KEYS" => Ok(GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum::HIVESTYLEKEYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ActionInvalidDataPartitionExpectedStructureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AspectTypeTransferStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Denotes the transfer status of the Aspect Type. It is unspecified for Aspect Types created from Dataplex API.
pub enum GoogleCloudDataplexV1AspectTypeTransferStatusEnum {
    

    /// The default value. It is set for resources that were not subject for migration from Data Catalog service.
    ///
    /// "TRANSFER_STATUS_UNSPECIFIED"
    #[serde(rename="TRANSFER_STATUS_UNSPECIFIED")]
    TRANSFERSTATUSUNSPECIFIED,
    

    /// Indicates that a resource was migrated from Data Catalog service but it hasn't been transferred yet. In particular the resource cannot be updated from Dataplex API.
    ///
    /// "TRANSFER_STATUS_MIGRATED"
    #[serde(rename="TRANSFER_STATUS_MIGRATED")]
    TRANSFERSTATUSMIGRATED,
    

    /// Indicates that a resource was transferred from Data Catalog service. The resource can only be updated from Dataplex API.
    ///
    /// "TRANSFER_STATUS_TRANSFERRED"
    #[serde(rename="TRANSFER_STATUS_TRANSFERRED")]
    TRANSFERSTATUSTRANSFERRED,
}

impl AsRef<str> for GoogleCloudDataplexV1AspectTypeTransferStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AspectTypeTransferStatusEnum::TRANSFERSTATUSUNSPECIFIED => "TRANSFER_STATUS_UNSPECIFIED",
            GoogleCloudDataplexV1AspectTypeTransferStatusEnum::TRANSFERSTATUSMIGRATED => "TRANSFER_STATUS_MIGRATED",
            GoogleCloudDataplexV1AspectTypeTransferStatusEnum::TRANSFERSTATUSTRANSFERRED => "TRANSFER_STATUS_TRANSFERRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AspectTypeTransferStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_STATUS_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AspectTypeTransferStatusEnum::TRANSFERSTATUSUNSPECIFIED),
           "TRANSFER_STATUS_MIGRATED" => Ok(GoogleCloudDataplexV1AspectTypeTransferStatusEnum::TRANSFERSTATUSMIGRATED),
           "TRANSFER_STATUS_TRANSFERRED" => Ok(GoogleCloudDataplexV1AspectTypeTransferStatusEnum::TRANSFERSTATUSTRANSFERRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AspectTypeTransferStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AssetStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the asset.
pub enum GoogleCloudDataplexV1AssetStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1AssetStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AssetStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1AssetStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1AssetStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1AssetStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1AssetStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AssetStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AssetStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1AssetStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1AssetStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1AssetStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1AssetStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AssetStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current status of the discovery feature.
pub enum GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum {
    

    /// State is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Discovery for the asset is scheduled.
    ///
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
    

    /// Discovery for the asset is running.
    ///
    /// "IN_PROGRESS"
    #[serde(rename="IN_PROGRESS")]
    INPROGRESS,
    

    /// Discovery for the asset is currently paused (e.g. due to a lack of available resources). It will be automatically resumed.
    ///
    /// "PAUSED"
    #[serde(rename="PAUSED")]
    PAUSED,
    

    /// Discovery for the asset is disabled.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::SCHEDULED => "SCHEDULED",
            GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::INPROGRESS => "IN_PROGRESS",
            GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::PAUSED => "PAUSED",
            GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::STATEUNSPECIFIED),
           "SCHEDULED" => Ok(GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::SCHEDULED),
           "IN_PROGRESS" => Ok(GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::INPROGRESS),
           "PAUSED" => Ok(GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::PAUSED),
           "DISABLED" => Ok(GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AssetDiscoveryStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets.
pub enum GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum {
    

    /// Access mode unspecified.
    ///
    /// "ACCESS_MODE_UNSPECIFIED"
    #[serde(rename="ACCESS_MODE_UNSPECIFIED")]
    ACCESSMODEUNSPECIFIED,
    

    /// Default. Data is accessed directly using storage APIs.
    ///
    /// "DIRECT"
    #[serde(rename="DIRECT")]
    DIRECT,
    

    /// Data is accessed through a managed interface using BigQuery APIs.
    ///
    /// "MANAGED"
    #[serde(rename="MANAGED")]
    MANAGED,
}

impl AsRef<str> for GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum::ACCESSMODEUNSPECIFIED => "ACCESS_MODE_UNSPECIFIED",
            GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum::DIRECT => "DIRECT",
            GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum::MANAGED => "MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_MODE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum::ACCESSMODEUNSPECIFIED),
           "DIRECT" => Ok(GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum::DIRECT),
           "MANAGED" => Ok(GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum::MANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AssetResourceSpecReadAccessModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AssetResourceSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. Type of resource.
pub enum GoogleCloudDataplexV1AssetResourceSpecTypeEnum {
    

    /// Type not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Cloud Storage bucket.
    ///
    /// "STORAGE_BUCKET"
    #[serde(rename="STORAGE_BUCKET")]
    STORAGEBUCKET,
    

    /// BigQuery dataset.
    ///
    /// "BIGQUERY_DATASET"
    #[serde(rename="BIGQUERY_DATASET")]
    BIGQUERYDATASET,
}

impl AsRef<str> for GoogleCloudDataplexV1AssetResourceSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AssetResourceSpecTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1AssetResourceSpecTypeEnum::STORAGEBUCKET => "STORAGE_BUCKET",
            GoogleCloudDataplexV1AssetResourceSpecTypeEnum::BIGQUERYDATASET => "BIGQUERY_DATASET",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AssetResourceSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AssetResourceSpecTypeEnum::TYPEUNSPECIFIED),
           "STORAGE_BUCKET" => Ok(GoogleCloudDataplexV1AssetResourceSpecTypeEnum::STORAGEBUCKET),
           "BIGQUERY_DATASET" => Ok(GoogleCloudDataplexV1AssetResourceSpecTypeEnum::BIGQUERYDATASET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AssetResourceSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AssetResourceStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the managed resource.
pub enum GoogleCloudDataplexV1AssetResourceStatusStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource does not have any errors.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Resource has errors.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleCloudDataplexV1AssetResourceStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AssetResourceStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1AssetResourceStatusStateEnum::READY => "READY",
            GoogleCloudDataplexV1AssetResourceStatusStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AssetResourceStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AssetResourceStatusStateEnum::STATEUNSPECIFIED),
           "READY" => Ok(GoogleCloudDataplexV1AssetResourceStatusStateEnum::READY),
           "ERROR" => Ok(GoogleCloudDataplexV1AssetResourceStatusStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AssetResourceStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1AssetSecurityStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the security policy applied to the attached resource.
pub enum GoogleCloudDataplexV1AssetSecurityStatusStateEnum {
    

    /// State unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Security policy has been successfully applied to the attached resource.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Security policy is in the process of being applied to the attached resource.
    ///
    /// "APPLYING"
    #[serde(rename="APPLYING")]
    APPLYING,
    

    /// Security policy could not be applied to the attached resource due to errors.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleCloudDataplexV1AssetSecurityStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1AssetSecurityStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1AssetSecurityStatusStateEnum::READY => "READY",
            GoogleCloudDataplexV1AssetSecurityStatusStateEnum::APPLYING => "APPLYING",
            GoogleCloudDataplexV1AssetSecurityStatusStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1AssetSecurityStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1AssetSecurityStatusStateEnum::STATEUNSPECIFIED),
           "READY" => Ok(GoogleCloudDataplexV1AssetSecurityStatusStateEnum::READY),
           "APPLYING" => Ok(GoogleCloudDataplexV1AssetSecurityStatusStateEnum::APPLYING),
           "ERROR" => Ok(GoogleCloudDataplexV1AssetSecurityStatusStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1AssetSecurityStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ContentNotebookKernelTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Kernel Type of the notebook.
pub enum GoogleCloudDataplexV1ContentNotebookKernelTypeEnum {
    

    /// Kernel Type unspecified.
    ///
    /// "KERNEL_TYPE_UNSPECIFIED"
    #[serde(rename="KERNEL_TYPE_UNSPECIFIED")]
    KERNELTYPEUNSPECIFIED,
    

    /// Python 3 Kernel.
    ///
    /// "PYTHON3"
    #[serde(rename="PYTHON3")]
    PYTHON3,
}

impl AsRef<str> for GoogleCloudDataplexV1ContentNotebookKernelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ContentNotebookKernelTypeEnum::KERNELTYPEUNSPECIFIED => "KERNEL_TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1ContentNotebookKernelTypeEnum::PYTHON3 => "PYTHON3",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ContentNotebookKernelTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KERNEL_TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ContentNotebookKernelTypeEnum::KERNELTYPEUNSPECIFIED),
           "PYTHON3" => Ok(GoogleCloudDataplexV1ContentNotebookKernelTypeEnum::PYTHON3),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ContentNotebookKernelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ContentSqlScriptEngineEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Query Engine to be used for the Sql Query.
pub enum GoogleCloudDataplexV1ContentSqlScriptEngineEnum {
    

    /// Value was unspecified.
    ///
    /// "QUERY_ENGINE_UNSPECIFIED"
    #[serde(rename="QUERY_ENGINE_UNSPECIFIED")]
    QUERYENGINEUNSPECIFIED,
    

    /// Spark SQL Query.
    ///
    /// "SPARK"
    #[serde(rename="SPARK")]
    SPARK,
}

impl AsRef<str> for GoogleCloudDataplexV1ContentSqlScriptEngineEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ContentSqlScriptEngineEnum::QUERYENGINEUNSPECIFIED => "QUERY_ENGINE_UNSPECIFIED",
            GoogleCloudDataplexV1ContentSqlScriptEngineEnum::SPARK => "SPARK",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ContentSqlScriptEngineEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "QUERY_ENGINE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ContentSqlScriptEngineEnum::QUERYENGINEUNSPECIFIED),
           "SPARK" => Ok(GoogleCloudDataplexV1ContentSqlScriptEngineEnum::SPARK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ContentSqlScriptEngineEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Execution state for the BigQuery exporting.
pub enum GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum {
    

    /// The exporting state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The exporting completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The exporting is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The exporting is skipped due to no valid scan result to export (usually caused by scan failed).
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
}

impl AsRef<str> for GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::FAILED => "FAILED",
            GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::SKIPPED => "SKIPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::STATEUNSPECIFIED),
           "SUCCEEDED" => Ok(GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::FAILED),
           "SKIPPED" => Ok(GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum::SKIPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataProfileResultPostScanActionsResultBigQueryExportResultStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Execution state for the BigQuery exporting.
pub enum GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum {
    

    /// The exporting state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The exporting completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The exporting is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The exporting is skipped due to no valid scan result to export (usually caused by scan failed).
    ///
    /// "SKIPPED"
    #[serde(rename="SKIPPED")]
    SKIPPED,
}

impl AsRef<str> for GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::FAILED => "FAILED",
            GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::SKIPPED => "SKIPPED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::STATEUNSPECIFIED),
           "SUCCEEDED" => Ok(GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::FAILED),
           "SKIPPED" => Ok(GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum::SKIPPED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataQualityResultPostScanActionsResultBigQueryExportResultStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The aggregate metric to evaluate.
pub enum GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum {
    

    /// Unspecified statistic type
    ///
    /// "STATISTIC_UNDEFINED"
    #[serde(rename="STATISTIC_UNDEFINED")]
    STATISTICUNDEFINED,
    

    /// Evaluate the column mean
    ///
    /// "MEAN"
    #[serde(rename="MEAN")]
    MEAN,
    

    /// Evaluate the column min
    ///
    /// "MIN"
    #[serde(rename="MIN")]
    MIN,
    

    /// Evaluate the column max
    ///
    /// "MAX"
    #[serde(rename="MAX")]
    MAX,
}

impl AsRef<str> for GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::STATISTICUNDEFINED => "STATISTIC_UNDEFINED",
            GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::MEAN => "MEAN",
            GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::MIN => "MIN",
            GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::MAX => "MAX",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATISTIC_UNDEFINED" => Ok(GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::STATISTICUNDEFINED),
           "MEAN" => Ok(GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::MEAN),
           "MIN" => Ok(GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::MIN),
           "MAX" => Ok(GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum::MAX),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataQualityRuleStatisticRangeExpectationStatisticEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataScanStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the DataScan.
pub enum GoogleCloudDataplexV1DataScanStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1DataScanStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataScanStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1DataScanStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1DataScanStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1DataScanStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1DataScanStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataScanStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1DataScanStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1DataScanStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1DataScanStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1DataScanStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1DataScanStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataScanStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataScanTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of DataScan.
pub enum GoogleCloudDataplexV1DataScanTypeEnum {
    

    /// The DataScan type is unspecified.
    ///
    /// "DATA_SCAN_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_SCAN_TYPE_UNSPECIFIED")]
    DATASCANTYPEUNSPECIFIED,
    

    /// Data Quality scan.
    ///
    /// "DATA_QUALITY"
    #[serde(rename="DATA_QUALITY")]
    DATAQUALITY,
    

    /// Data Profile scan.
    ///
    /// "DATA_PROFILE"
    #[serde(rename="DATA_PROFILE")]
    DATAPROFILE,
}

impl AsRef<str> for GoogleCloudDataplexV1DataScanTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataScanTypeEnum::DATASCANTYPEUNSPECIFIED => "DATA_SCAN_TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1DataScanTypeEnum::DATAQUALITY => "DATA_QUALITY",
            GoogleCloudDataplexV1DataScanTypeEnum::DATAPROFILE => "DATA_PROFILE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataScanTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_SCAN_TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1DataScanTypeEnum::DATASCANTYPEUNSPECIFIED),
           "DATA_QUALITY" => Ok(GoogleCloudDataplexV1DataScanTypeEnum::DATAQUALITY),
           "DATA_PROFILE" => Ok(GoogleCloudDataplexV1DataScanTypeEnum::DATAPROFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataScanTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataScanJobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Execution state for the DataScanJob.
pub enum GoogleCloudDataplexV1DataScanJobStateEnum {
    

    /// The DataScanJob state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The DataScanJob is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The DataScanJob is canceling.
    ///
    /// "CANCELING"
    #[serde(rename="CANCELING")]
    CANCELING,
    

    /// The DataScanJob cancellation was successful.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The DataScanJob completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The DataScanJob is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The DataScanJob has been created but not started to run yet.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
}

impl AsRef<str> for GoogleCloudDataplexV1DataScanJobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataScanJobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1DataScanJobStateEnum::RUNNING => "RUNNING",
            GoogleCloudDataplexV1DataScanJobStateEnum::CANCELING => "CANCELING",
            GoogleCloudDataplexV1DataScanJobStateEnum::CANCELLED => "CANCELLED",
            GoogleCloudDataplexV1DataScanJobStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudDataplexV1DataScanJobStateEnum::FAILED => "FAILED",
            GoogleCloudDataplexV1DataScanJobStateEnum::PENDING => "PENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataScanJobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::RUNNING),
           "CANCELING" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::CANCELING),
           "CANCELLED" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::CANCELLED),
           "SUCCEEDED" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::FAILED),
           "PENDING" => Ok(GoogleCloudDataplexV1DataScanJobStateEnum::PENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataScanJobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1DataScanJobTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The type of the parent DataScan.
pub enum GoogleCloudDataplexV1DataScanJobTypeEnum {
    

    /// The DataScan type is unspecified.
    ///
    /// "DATA_SCAN_TYPE_UNSPECIFIED"
    #[serde(rename="DATA_SCAN_TYPE_UNSPECIFIED")]
    DATASCANTYPEUNSPECIFIED,
    

    /// Data Quality scan.
    ///
    /// "DATA_QUALITY"
    #[serde(rename="DATA_QUALITY")]
    DATAQUALITY,
    

    /// Data Profile scan.
    ///
    /// "DATA_PROFILE"
    #[serde(rename="DATA_PROFILE")]
    DATAPROFILE,
}

impl AsRef<str> for GoogleCloudDataplexV1DataScanJobTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1DataScanJobTypeEnum::DATASCANTYPEUNSPECIFIED => "DATA_SCAN_TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1DataScanJobTypeEnum::DATAQUALITY => "DATA_QUALITY",
            GoogleCloudDataplexV1DataScanJobTypeEnum::DATAPROFILE => "DATA_PROFILE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1DataScanJobTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATA_SCAN_TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1DataScanJobTypeEnum::DATASCANTYPEUNSPECIFIED),
           "DATA_QUALITY" => Ok(GoogleCloudDataplexV1DataScanJobTypeEnum::DATAQUALITY),
           "DATA_PROFILE" => Ok(GoogleCloudDataplexV1DataScanJobTypeEnum::DATAPROFILE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1DataScanJobTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1EntitySystemEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. Identifies the storage system of the entity data.
pub enum GoogleCloudDataplexV1EntitySystemEnum {
    

    /// Storage system unspecified.
    ///
    /// "STORAGE_SYSTEM_UNSPECIFIED"
    #[serde(rename="STORAGE_SYSTEM_UNSPECIFIED")]
    STORAGESYSTEMUNSPECIFIED,
    

    /// The entity data is contained within a Cloud Storage bucket.
    ///
    /// "CLOUD_STORAGE"
    #[serde(rename="CLOUD_STORAGE")]
    CLOUDSTORAGE,
    

    /// The entity data is contained within a BigQuery dataset.
    ///
    /// "BIGQUERY"
    #[serde(rename="BIGQUERY")]
    BIGQUERY,
}

impl AsRef<str> for GoogleCloudDataplexV1EntitySystemEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1EntitySystemEnum::STORAGESYSTEMUNSPECIFIED => "STORAGE_SYSTEM_UNSPECIFIED",
            GoogleCloudDataplexV1EntitySystemEnum::CLOUDSTORAGE => "CLOUD_STORAGE",
            GoogleCloudDataplexV1EntitySystemEnum::BIGQUERY => "BIGQUERY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1EntitySystemEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STORAGE_SYSTEM_UNSPECIFIED" => Ok(GoogleCloudDataplexV1EntitySystemEnum::STORAGESYSTEMUNSPECIFIED),
           "CLOUD_STORAGE" => Ok(GoogleCloudDataplexV1EntitySystemEnum::CLOUDSTORAGE),
           "BIGQUERY" => Ok(GoogleCloudDataplexV1EntitySystemEnum::BIGQUERY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1EntitySystemEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1EntityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of entity.
pub enum GoogleCloudDataplexV1EntityTypeEnum {
    

    /// Type unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Structured and semi-structured data.
    ///
    /// "TABLE"
    #[serde(rename="TABLE")]
    TABLE,
    

    /// Unstructured data.
    ///
    /// "FILESET"
    #[serde(rename="FILESET")]
    FILESET,
}

impl AsRef<str> for GoogleCloudDataplexV1EntityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1EntityTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1EntityTypeEnum::TABLE => "TABLE",
            GoogleCloudDataplexV1EntityTypeEnum::FILESET => "FILESET",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1EntityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1EntityTypeEnum::TYPEUNSPECIFIED),
           "TABLE" => Ok(GoogleCloudDataplexV1EntityTypeEnum::TABLE),
           "FILESET" => Ok(GoogleCloudDataplexV1EntityTypeEnum::FILESET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1EntityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1EntryGroupTransferStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Denotes the transfer status of the Entry Group. It is unspecified for Entry Group created from Dataplex API.
pub enum GoogleCloudDataplexV1EntryGroupTransferStatusEnum {
    

    /// The default value. It is set for resources that were not subject for migration from Data Catalog service.
    ///
    /// "TRANSFER_STATUS_UNSPECIFIED"
    #[serde(rename="TRANSFER_STATUS_UNSPECIFIED")]
    TRANSFERSTATUSUNSPECIFIED,
    

    /// Indicates that a resource was migrated from Data Catalog service but it hasn't been transferred yet. In particular the resource cannot be updated from Dataplex API.
    ///
    /// "TRANSFER_STATUS_MIGRATED"
    #[serde(rename="TRANSFER_STATUS_MIGRATED")]
    TRANSFERSTATUSMIGRATED,
    

    /// Indicates that a resource was transferred from Data Catalog service. The resource can only be updated from Dataplex API.
    ///
    /// "TRANSFER_STATUS_TRANSFERRED"
    #[serde(rename="TRANSFER_STATUS_TRANSFERRED")]
    TRANSFERSTATUSTRANSFERRED,
}

impl AsRef<str> for GoogleCloudDataplexV1EntryGroupTransferStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1EntryGroupTransferStatusEnum::TRANSFERSTATUSUNSPECIFIED => "TRANSFER_STATUS_UNSPECIFIED",
            GoogleCloudDataplexV1EntryGroupTransferStatusEnum::TRANSFERSTATUSMIGRATED => "TRANSFER_STATUS_MIGRATED",
            GoogleCloudDataplexV1EntryGroupTransferStatusEnum::TRANSFERSTATUSTRANSFERRED => "TRANSFER_STATUS_TRANSFERRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1EntryGroupTransferStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRANSFER_STATUS_UNSPECIFIED" => Ok(GoogleCloudDataplexV1EntryGroupTransferStatusEnum::TRANSFERSTATUSUNSPECIFIED),
           "TRANSFER_STATUS_MIGRATED" => Ok(GoogleCloudDataplexV1EntryGroupTransferStatusEnum::TRANSFERSTATUSMIGRATED),
           "TRANSFER_STATUS_TRANSFERRED" => Ok(GoogleCloudDataplexV1EntryGroupTransferStatusEnum::TRANSFERSTATUSTRANSFERRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1EntryGroupTransferStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1EnvironmentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the environment.
pub enum GoogleCloudDataplexV1EnvironmentStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1EnvironmentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1EnvironmentStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1EnvironmentStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1EnvironmentStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1EnvironmentStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1EnvironmentStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1EnvironmentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1EnvironmentStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1EnvironmentStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1EnvironmentStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1EnvironmentStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1EnvironmentStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1EnvironmentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1JobServiceEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The underlying service running a job.
pub enum GoogleCloudDataplexV1JobServiceEnum {
    

    /// Service used to run the job is unspecified.
    ///
    /// "SERVICE_UNSPECIFIED"
    #[serde(rename="SERVICE_UNSPECIFIED")]
    SERVICEUNSPECIFIED,
    

    /// Dataproc service is used to run this job.
    ///
    /// "DATAPROC"
    #[serde(rename="DATAPROC")]
    DATAPROC,
}

impl AsRef<str> for GoogleCloudDataplexV1JobServiceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1JobServiceEnum::SERVICEUNSPECIFIED => "SERVICE_UNSPECIFIED",
            GoogleCloudDataplexV1JobServiceEnum::DATAPROC => "DATAPROC",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1JobServiceEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVICE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1JobServiceEnum::SERVICEUNSPECIFIED),
           "DATAPROC" => Ok(GoogleCloudDataplexV1JobServiceEnum::DATAPROC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1JobServiceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1JobStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Execution state for the job.
pub enum GoogleCloudDataplexV1JobStateEnum {
    

    /// The job state is unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The job is running.
    ///
    /// "RUNNING"
    #[serde(rename="RUNNING")]
    RUNNING,
    

    /// The job is cancelling.
    ///
    /// "CANCELLING"
    #[serde(rename="CANCELLING")]
    CANCELLING,
    

    /// The job cancellation was successful.
    ///
    /// "CANCELLED"
    #[serde(rename="CANCELLED")]
    CANCELLED,
    

    /// The job completed successfully.
    ///
    /// "SUCCEEDED"
    #[serde(rename="SUCCEEDED")]
    SUCCEEDED,
    

    /// The job is no longer running due to an error.
    ///
    /// "FAILED"
    #[serde(rename="FAILED")]
    FAILED,
    

    /// The job was cancelled outside of Dataplex.
    ///
    /// "ABORTED"
    #[serde(rename="ABORTED")]
    ABORTED,
}

impl AsRef<str> for GoogleCloudDataplexV1JobStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1JobStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1JobStateEnum::RUNNING => "RUNNING",
            GoogleCloudDataplexV1JobStateEnum::CANCELLING => "CANCELLING",
            GoogleCloudDataplexV1JobStateEnum::CANCELLED => "CANCELLED",
            GoogleCloudDataplexV1JobStateEnum::SUCCEEDED => "SUCCEEDED",
            GoogleCloudDataplexV1JobStateEnum::FAILED => "FAILED",
            GoogleCloudDataplexV1JobStateEnum::ABORTED => "ABORTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1JobStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1JobStateEnum::STATEUNSPECIFIED),
           "RUNNING" => Ok(GoogleCloudDataplexV1JobStateEnum::RUNNING),
           "CANCELLING" => Ok(GoogleCloudDataplexV1JobStateEnum::CANCELLING),
           "CANCELLED" => Ok(GoogleCloudDataplexV1JobStateEnum::CANCELLED),
           "SUCCEEDED" => Ok(GoogleCloudDataplexV1JobStateEnum::SUCCEEDED),
           "FAILED" => Ok(GoogleCloudDataplexV1JobStateEnum::FAILED),
           "ABORTED" => Ok(GoogleCloudDataplexV1JobStateEnum::ABORTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1JobStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1JobTriggerEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Job execution trigger.
pub enum GoogleCloudDataplexV1JobTriggerEnum {
    

    /// The trigger is unspecified.
    ///
    /// "TRIGGER_UNSPECIFIED"
    #[serde(rename="TRIGGER_UNSPECIFIED")]
    TRIGGERUNSPECIFIED,
    

    /// The job was triggered by Dataplex based on trigger spec from task definition.
    ///
    /// "TASK_CONFIG"
    #[serde(rename="TASK_CONFIG")]
    TASKCONFIG,
    

    /// The job was triggered by the explicit call of Task API.
    ///
    /// "RUN_REQUEST"
    #[serde(rename="RUN_REQUEST")]
    RUNREQUEST,
}

impl AsRef<str> for GoogleCloudDataplexV1JobTriggerEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1JobTriggerEnum::TRIGGERUNSPECIFIED => "TRIGGER_UNSPECIFIED",
            GoogleCloudDataplexV1JobTriggerEnum::TASKCONFIG => "TASK_CONFIG",
            GoogleCloudDataplexV1JobTriggerEnum::RUNREQUEST => "RUN_REQUEST",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1JobTriggerEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TRIGGER_UNSPECIFIED" => Ok(GoogleCloudDataplexV1JobTriggerEnum::TRIGGERUNSPECIFIED),
           "TASK_CONFIG" => Ok(GoogleCloudDataplexV1JobTriggerEnum::TASKCONFIG),
           "RUN_REQUEST" => Ok(GoogleCloudDataplexV1JobTriggerEnum::RUNREQUEST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1JobTriggerEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1LakeStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the lake.
pub enum GoogleCloudDataplexV1LakeStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1LakeStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1LakeStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1LakeStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1LakeStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1LakeStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1LakeStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1LakeStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1LakeStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1LakeStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1LakeStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1LakeStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1LakeStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1LakeStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1LakeMetastoreStatusStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Current state of association.
pub enum GoogleCloudDataplexV1LakeMetastoreStatusStateEnum {
    

    /// Unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// A Metastore service instance is not associated with the lake.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// A Metastore service instance is attached to the lake.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// Attach/detach is in progress.
    ///
    /// "UPDATING"
    #[serde(rename="UPDATING")]
    UPDATING,
    

    /// Attach/detach could not be done due to errors.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleCloudDataplexV1LakeMetastoreStatusStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::NONE => "NONE",
            GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::READY => "READY",
            GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::UPDATING => "UPDATING",
            GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1LakeMetastoreStatusStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::STATEUNSPECIFIED),
           "NONE" => Ok(GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::NONE),
           "READY" => Ok(GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::READY),
           "UPDATING" => Ok(GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::UPDATING),
           "ERROR" => Ok(GoogleCloudDataplexV1LakeMetastoreStatusStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1LakeMetastoreStatusStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1SchemaPartitionStyleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The structure of paths containing partition data within the entity.
pub enum GoogleCloudDataplexV1SchemaPartitionStyleEnum {
    

    /// PartitionStyle unspecified
    ///
    /// "PARTITION_STYLE_UNSPECIFIED"
    #[serde(rename="PARTITION_STYLE_UNSPECIFIED")]
    PARTITIONSTYLEUNSPECIFIED,
    

    /// Partitions are hive-compatible. Examples: gs://bucket/path/to/table/dt=2019-10-31/lang=en, gs://bucket/path/to/table/dt=2019-10-31/lang=en/late.
    ///
    /// "HIVE_COMPATIBLE"
    #[serde(rename="HIVE_COMPATIBLE")]
    HIVECOMPATIBLE,
}

impl AsRef<str> for GoogleCloudDataplexV1SchemaPartitionStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1SchemaPartitionStyleEnum::PARTITIONSTYLEUNSPECIFIED => "PARTITION_STYLE_UNSPECIFIED",
            GoogleCloudDataplexV1SchemaPartitionStyleEnum::HIVECOMPATIBLE => "HIVE_COMPATIBLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1SchemaPartitionStyleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTITION_STYLE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1SchemaPartitionStyleEnum::PARTITIONSTYLEUNSPECIFIED),
           "HIVE_COMPATIBLE" => Ok(GoogleCloudDataplexV1SchemaPartitionStyleEnum::HIVECOMPATIBLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1SchemaPartitionStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of field.
pub enum GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum {
    

    /// SchemaType unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Boolean field.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// Single byte numeric field.
    ///
    /// "BYTE"
    #[serde(rename="BYTE")]
    BYTE,
    

    /// 16-bit numeric field.
    ///
    /// "INT16"
    #[serde(rename="INT16")]
    INT16,
    

    /// 32-bit numeric field.
    ///
    /// "INT32"
    #[serde(rename="INT32")]
    INT32,
    

    /// 64-bit numeric field.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// Floating point numeric field.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// Double precision numeric field.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// Real value numeric field.
    ///
    /// "DECIMAL"
    #[serde(rename="DECIMAL")]
    DECIMAL,
    

    /// Sequence of characters field.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Sequence of bytes field.
    ///
    /// "BINARY"
    #[serde(rename="BINARY")]
    BINARY,
    

    /// Date and time field.
    ///
    /// "TIMESTAMP"
    #[serde(rename="TIMESTAMP")]
    TIMESTAMP,
    

    /// Date field.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Time field.
    ///
    /// "TIME"
    #[serde(rename="TIME")]
    TIME,
    

    /// Structured field. Nested fields that define the structure of the map. If all nested fields are nullable, this field represents a union.
    ///
    /// "RECORD"
    #[serde(rename="RECORD")]
    RECORD,
    

    /// Null field that does not have values.
    ///
    /// "NULL"
    #[serde(rename="NULL")]
    NULL,
}

impl AsRef<str> for GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::BOOLEAN => "BOOLEAN",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::BYTE => "BYTE",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::INT16 => "INT16",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::INT32 => "INT32",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::INT64 => "INT64",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::FLOAT => "FLOAT",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::DECIMAL => "DECIMAL",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::STRING => "STRING",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::BINARY => "BINARY",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::TIMESTAMP => "TIMESTAMP",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::DATE => "DATE",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::TIME => "TIME",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::RECORD => "RECORD",
            GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::NULL => "NULL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::TYPEUNSPECIFIED),
           "BOOLEAN" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::BOOLEAN),
           "BYTE" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::BYTE),
           "INT16" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::INT16),
           "INT32" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::INT32),
           "INT64" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::INT64),
           "FLOAT" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::FLOAT),
           "DOUBLE" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::DOUBLE),
           "DECIMAL" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::DECIMAL),
           "STRING" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::STRING),
           "BINARY" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::BINARY),
           "TIMESTAMP" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::TIMESTAMP),
           "DATE" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::DATE),
           "TIME" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::TIME),
           "RECORD" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::RECORD),
           "NULL" => Ok(GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum::NULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1SchemaPartitionFieldTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1SchemaSchemaFieldModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Additional field semantics.
pub enum GoogleCloudDataplexV1SchemaSchemaFieldModeEnum {
    

    /// Mode unspecified.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// The field has required semantics.
    ///
    /// "REQUIRED"
    #[serde(rename="REQUIRED")]
    REQUIRED,
    

    /// The field has optional semantics, and may be null.
    ///
    /// "NULLABLE"
    #[serde(rename="NULLABLE")]
    NULLABLE,
    

    /// The field has repeated (0 or more) semantics, and is a list of values.
    ///
    /// "REPEATED"
    #[serde(rename="REPEATED")]
    REPEATED,
}

impl AsRef<str> for GoogleCloudDataplexV1SchemaSchemaFieldModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::REQUIRED => "REQUIRED",
            GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::NULLABLE => "NULLABLE",
            GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::REPEATED => "REPEATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1SchemaSchemaFieldModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::MODEUNSPECIFIED),
           "REQUIRED" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::REQUIRED),
           "NULLABLE" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::NULLABLE),
           "REPEATED" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldModeEnum::REPEATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1SchemaSchemaFieldModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The type of field.
pub enum GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum {
    

    /// SchemaType unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Boolean field.
    ///
    /// "BOOLEAN"
    #[serde(rename="BOOLEAN")]
    BOOLEAN,
    

    /// Single byte numeric field.
    ///
    /// "BYTE"
    #[serde(rename="BYTE")]
    BYTE,
    

    /// 16-bit numeric field.
    ///
    /// "INT16"
    #[serde(rename="INT16")]
    INT16,
    

    /// 32-bit numeric field.
    ///
    /// "INT32"
    #[serde(rename="INT32")]
    INT32,
    

    /// 64-bit numeric field.
    ///
    /// "INT64"
    #[serde(rename="INT64")]
    INT64,
    

    /// Floating point numeric field.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// Double precision numeric field.
    ///
    /// "DOUBLE"
    #[serde(rename="DOUBLE")]
    DOUBLE,
    

    /// Real value numeric field.
    ///
    /// "DECIMAL"
    #[serde(rename="DECIMAL")]
    DECIMAL,
    

    /// Sequence of characters field.
    ///
    /// "STRING"
    #[serde(rename="STRING")]
    STRING,
    

    /// Sequence of bytes field.
    ///
    /// "BINARY"
    #[serde(rename="BINARY")]
    BINARY,
    

    /// Date and time field.
    ///
    /// "TIMESTAMP"
    #[serde(rename="TIMESTAMP")]
    TIMESTAMP,
    

    /// Date field.
    ///
    /// "DATE"
    #[serde(rename="DATE")]
    DATE,
    

    /// Time field.
    ///
    /// "TIME"
    #[serde(rename="TIME")]
    TIME,
    

    /// Structured field. Nested fields that define the structure of the map. If all nested fields are nullable, this field represents a union.
    ///
    /// "RECORD"
    #[serde(rename="RECORD")]
    RECORD,
    

    /// Null field that does not have values.
    ///
    /// "NULL"
    #[serde(rename="NULL")]
    NULL,
}

impl AsRef<str> for GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::BOOLEAN => "BOOLEAN",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::BYTE => "BYTE",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::INT16 => "INT16",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::INT32 => "INT32",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::INT64 => "INT64",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::FLOAT => "FLOAT",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::DOUBLE => "DOUBLE",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::DECIMAL => "DECIMAL",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::STRING => "STRING",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::BINARY => "BINARY",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::TIMESTAMP => "TIMESTAMP",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::DATE => "DATE",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::TIME => "TIME",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::RECORD => "RECORD",
            GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::NULL => "NULL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::TYPEUNSPECIFIED),
           "BOOLEAN" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::BOOLEAN),
           "BYTE" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::BYTE),
           "INT16" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::INT16),
           "INT32" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::INT32),
           "INT64" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::INT64),
           "FLOAT" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::FLOAT),
           "DOUBLE" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::DOUBLE),
           "DECIMAL" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::DECIMAL),
           "STRING" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::STRING),
           "BINARY" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::BINARY),
           "TIMESTAMP" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::TIMESTAMP),
           "DATE" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::DATE),
           "TIME" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::TIME),
           "RECORD" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::RECORD),
           "NULL" => Ok(GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum::NULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1SchemaSchemaFieldTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1SessionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of Session
pub enum GoogleCloudDataplexV1SessionStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1SessionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1SessionStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1SessionStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1SessionStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1SessionStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1SessionStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1SessionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1SessionStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1SessionStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1SessionStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1SessionStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1SessionStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1SessionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1StorageAccesReadEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Describes the read access mechanism of the data. Not user settable.
pub enum GoogleCloudDataplexV1StorageAccesReadEnum {
    

    /// Access mode unspecified.
    ///
    /// "ACCESS_MODE_UNSPECIFIED"
    #[serde(rename="ACCESS_MODE_UNSPECIFIED")]
    ACCESSMODEUNSPECIFIED,
    

    /// Default. Data is accessed directly using storage APIs.
    ///
    /// "DIRECT"
    #[serde(rename="DIRECT")]
    DIRECT,
    

    /// Data is accessed through a managed interface using BigQuery APIs.
    ///
    /// "MANAGED"
    #[serde(rename="MANAGED")]
    MANAGED,
}

impl AsRef<str> for GoogleCloudDataplexV1StorageAccesReadEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1StorageAccesReadEnum::ACCESSMODEUNSPECIFIED => "ACCESS_MODE_UNSPECIFIED",
            GoogleCloudDataplexV1StorageAccesReadEnum::DIRECT => "DIRECT",
            GoogleCloudDataplexV1StorageAccesReadEnum::MANAGED => "MANAGED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1StorageAccesReadEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACCESS_MODE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1StorageAccesReadEnum::ACCESSMODEUNSPECIFIED),
           "DIRECT" => Ok(GoogleCloudDataplexV1StorageAccesReadEnum::DIRECT),
           "MANAGED" => Ok(GoogleCloudDataplexV1StorageAccesReadEnum::MANAGED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1StorageAccesReadEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1StorageFormatCompressionFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The compression type associated with the stored data. If unspecified, the data is uncompressed.
pub enum GoogleCloudDataplexV1StorageFormatCompressionFormatEnum {
    

    /// CompressionFormat unspecified. Implies uncompressed data.
    ///
    /// "COMPRESSION_FORMAT_UNSPECIFIED"
    #[serde(rename="COMPRESSION_FORMAT_UNSPECIFIED")]
    COMPRESSIONFORMATUNSPECIFIED,
    

    /// GZip compressed set of files.
    ///
    /// "GZIP"
    #[serde(rename="GZIP")]
    GZIP,
    

    /// BZip2 compressed set of files.
    ///
    /// "BZIP2"
    #[serde(rename="BZIP2")]
    BZIP2,
}

impl AsRef<str> for GoogleCloudDataplexV1StorageFormatCompressionFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1StorageFormatCompressionFormatEnum::COMPRESSIONFORMATUNSPECIFIED => "COMPRESSION_FORMAT_UNSPECIFIED",
            GoogleCloudDataplexV1StorageFormatCompressionFormatEnum::GZIP => "GZIP",
            GoogleCloudDataplexV1StorageFormatCompressionFormatEnum::BZIP2 => "BZIP2",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1StorageFormatCompressionFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COMPRESSION_FORMAT_UNSPECIFIED" => Ok(GoogleCloudDataplexV1StorageFormatCompressionFormatEnum::COMPRESSIONFORMATUNSPECIFIED),
           "GZIP" => Ok(GoogleCloudDataplexV1StorageFormatCompressionFormatEnum::GZIP),
           "BZIP2" => Ok(GoogleCloudDataplexV1StorageFormatCompressionFormatEnum::BZIP2),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1StorageFormatCompressionFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1StorageFormatFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The data format associated with the stored data, which represents content type values. The value is inferred from mime type.
pub enum GoogleCloudDataplexV1StorageFormatFormatEnum {
    

    /// Format unspecified.
    ///
    /// "FORMAT_UNSPECIFIED"
    #[serde(rename="FORMAT_UNSPECIFIED")]
    FORMATUNSPECIFIED,
    

    /// Parquet-formatted structured data.
    ///
    /// "PARQUET"
    #[serde(rename="PARQUET")]
    PARQUET,
    

    /// Avro-formatted structured data.
    ///
    /// "AVRO"
    #[serde(rename="AVRO")]
    AVRO,
    

    /// Orc-formatted structured data.
    ///
    /// "ORC"
    #[serde(rename="ORC")]
    ORC,
    

    /// Csv-formatted semi-structured data.
    ///
    /// "CSV"
    #[serde(rename="CSV")]
    CSV,
    

    /// Json-formatted semi-structured data.
    ///
    /// "JSON"
    #[serde(rename="JSON")]
    JSON,
    

    /// Image data formats (such as jpg and png).
    ///
    /// "IMAGE"
    #[serde(rename="IMAGE")]
    IMAGE,
    

    /// Audio data formats (such as mp3, and wav).
    ///
    /// "AUDIO"
    #[serde(rename="AUDIO")]
    AUDIO,
    

    /// Video data formats (such as mp4 and mpg).
    ///
    /// "VIDEO"
    #[serde(rename="VIDEO")]
    VIDEO,
    

    /// Textual data formats (such as txt and xml).
    ///
    /// "TEXT"
    #[serde(rename="TEXT")]
    TEXT,
    

    /// TensorFlow record format.
    ///
    /// "TFRECORD"
    #[serde(rename="TFRECORD")]
    TFRECORD,
    

    /// Data that doesn't match a specific format.
    ///
    /// "OTHER"
    #[serde(rename="OTHER")]
    OTHER,
    

    /// Data of an unknown format.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
}

impl AsRef<str> for GoogleCloudDataplexV1StorageFormatFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1StorageFormatFormatEnum::FORMATUNSPECIFIED => "FORMAT_UNSPECIFIED",
            GoogleCloudDataplexV1StorageFormatFormatEnum::PARQUET => "PARQUET",
            GoogleCloudDataplexV1StorageFormatFormatEnum::AVRO => "AVRO",
            GoogleCloudDataplexV1StorageFormatFormatEnum::ORC => "ORC",
            GoogleCloudDataplexV1StorageFormatFormatEnum::CSV => "CSV",
            GoogleCloudDataplexV1StorageFormatFormatEnum::JSON => "JSON",
            GoogleCloudDataplexV1StorageFormatFormatEnum::IMAGE => "IMAGE",
            GoogleCloudDataplexV1StorageFormatFormatEnum::AUDIO => "AUDIO",
            GoogleCloudDataplexV1StorageFormatFormatEnum::VIDEO => "VIDEO",
            GoogleCloudDataplexV1StorageFormatFormatEnum::TEXT => "TEXT",
            GoogleCloudDataplexV1StorageFormatFormatEnum::TFRECORD => "TFRECORD",
            GoogleCloudDataplexV1StorageFormatFormatEnum::OTHER => "OTHER",
            GoogleCloudDataplexV1StorageFormatFormatEnum::UNKNOWN => "UNKNOWN",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1StorageFormatFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMAT_UNSPECIFIED" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::FORMATUNSPECIFIED),
           "PARQUET" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::PARQUET),
           "AVRO" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::AVRO),
           "ORC" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::ORC),
           "CSV" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::CSV),
           "JSON" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::JSON),
           "IMAGE" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::IMAGE),
           "AUDIO" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::AUDIO),
           "VIDEO" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::VIDEO),
           "TEXT" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::TEXT),
           "TFRECORD" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::TFRECORD),
           "OTHER" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::OTHER),
           "UNKNOWN" => Ok(GoogleCloudDataplexV1StorageFormatFormatEnum::UNKNOWN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1StorageFormatFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1TaskStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the task.
pub enum GoogleCloudDataplexV1TaskStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1TaskStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1TaskStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1TaskStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1TaskStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1TaskStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1TaskStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1TaskStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1TaskStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1TaskStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1TaskStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1TaskStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1TaskStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1TaskStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1TaskTriggerSpecTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. Trigger type of the user-specified Task.
pub enum GoogleCloudDataplexV1TaskTriggerSpecTypeEnum {
    

    /// Unspecified trigger type.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The task runs one-time shortly after Task Creation.
    ///
    /// "ON_DEMAND"
    #[serde(rename="ON_DEMAND")]
    ONDEMAND,
    

    /// The task is scheduled to run periodically.
    ///
    /// "RECURRING"
    #[serde(rename="RECURRING")]
    RECURRING,
}

impl AsRef<str> for GoogleCloudDataplexV1TaskTriggerSpecTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1TaskTriggerSpecTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1TaskTriggerSpecTypeEnum::ONDEMAND => "ON_DEMAND",
            GoogleCloudDataplexV1TaskTriggerSpecTypeEnum::RECURRING => "RECURRING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1TaskTriggerSpecTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1TaskTriggerSpecTypeEnum::TYPEUNSPECIFIED),
           "ON_DEMAND" => Ok(GoogleCloudDataplexV1TaskTriggerSpecTypeEnum::ONDEMAND),
           "RECURRING" => Ok(GoogleCloudDataplexV1TaskTriggerSpecTypeEnum::RECURRING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1TaskTriggerSpecTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ZoneStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. Current state of the zone.
pub enum GoogleCloudDataplexV1ZoneStateEnum {
    

    /// State is not specified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// Resource is active, i.e., ready to use.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// Resource is under creation.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// Resource is under deletion.
    ///
    /// "DELETING"
    #[serde(rename="DELETING")]
    DELETING,
    

    /// Resource is active but has unresolved actions.
    ///
    /// "ACTION_REQUIRED"
    #[serde(rename="ACTION_REQUIRED")]
    ACTIONREQUIRED,
}

impl AsRef<str> for GoogleCloudDataplexV1ZoneStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ZoneStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleCloudDataplexV1ZoneStateEnum::ACTIVE => "ACTIVE",
            GoogleCloudDataplexV1ZoneStateEnum::CREATING => "CREATING",
            GoogleCloudDataplexV1ZoneStateEnum::DELETING => "DELETING",
            GoogleCloudDataplexV1ZoneStateEnum::ACTIONREQUIRED => "ACTION_REQUIRED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ZoneStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ZoneStateEnum::STATEUNSPECIFIED),
           "ACTIVE" => Ok(GoogleCloudDataplexV1ZoneStateEnum::ACTIVE),
           "CREATING" => Ok(GoogleCloudDataplexV1ZoneStateEnum::CREATING),
           "DELETING" => Ok(GoogleCloudDataplexV1ZoneStateEnum::DELETING),
           "ACTION_REQUIRED" => Ok(GoogleCloudDataplexV1ZoneStateEnum::ACTIONREQUIRED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ZoneStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ZoneTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The type of the zone.
pub enum GoogleCloudDataplexV1ZoneTypeEnum {
    

    /// Zone type not specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// A zone that contains data that needs further processing before it is considered generally ready for consumption and analytics workloads.
    ///
    /// "RAW"
    #[serde(rename="RAW")]
    RAW,
    

    /// A zone that contains data that is considered to be ready for broader consumption and analytics workloads. Curated structured data stored in Cloud Storage must conform to certain file formats (parquet, avro and orc) and organized in a hive-compatible directory layout.
    ///
    /// "CURATED"
    #[serde(rename="CURATED")]
    CURATED,
}

impl AsRef<str> for GoogleCloudDataplexV1ZoneTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ZoneTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1ZoneTypeEnum::RAW => "RAW",
            GoogleCloudDataplexV1ZoneTypeEnum::CURATED => "CURATED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ZoneTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ZoneTypeEnum::TYPEUNSPECIFIED),
           "RAW" => Ok(GoogleCloudDataplexV1ZoneTypeEnum::RAW),
           "CURATED" => Ok(GoogleCloudDataplexV1ZoneTypeEnum::CURATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ZoneTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Immutable. The location type of the resources that are allowed to be attached to the assets within this zone.
pub enum GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum {
    

    /// Unspecified location type.
    ///
    /// "LOCATION_TYPE_UNSPECIFIED"
    #[serde(rename="LOCATION_TYPE_UNSPECIFIED")]
    LOCATIONTYPEUNSPECIFIED,
    

    /// Resources that are associated with a single region.
    ///
    /// "SINGLE_REGION"
    #[serde(rename="SINGLE_REGION")]
    SINGLEREGION,
    

    /// Resources that are associated with a multi-region location.
    ///
    /// "MULTI_REGION"
    #[serde(rename="MULTI_REGION")]
    MULTIREGION,
}

impl AsRef<str> for GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum::LOCATIONTYPEUNSPECIFIED => "LOCATION_TYPE_UNSPECIFIED",
            GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum::SINGLEREGION => "SINGLE_REGION",
            GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum::MULTIREGION => "MULTI_REGION",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LOCATION_TYPE_UNSPECIFIED" => Ok(GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum::LOCATIONTYPEUNSPECIFIED),
           "SINGLE_REGION" => Ok(GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum::SINGLEREGION),
           "MULTI_REGION" => Ok(GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum::MULTIREGION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudDataplexV1ZoneResourceSpecLocationTypeEnum {
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


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. View for controlling which parts of an entry are to be returned.
pub enum ProjectViewEnum {
    

    /// Unspecified EntryView. Defaults to FULL.
    ///
    /// "ENTRY_VIEW_UNSPECIFIED"
    #[serde(rename="ENTRY_VIEW_UNSPECIFIED")]
    ENTRYVIEWUNSPECIFIED,
    

    /// Returns entry only, without aspects.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Returns all required aspects as well as the keys of all non-required aspects.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Returns aspects matching custom fields in GetEntryRequest. If the number of aspects would exceed 100, the first 100 will be returned.
    ///
    /// "CUSTOM"
    #[serde(rename="CUSTOM")]
    CUSTOM,
    

    /// Returns all aspects. If the number of aspects would exceed 100, the first 100 will be returned.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::ENTRYVIEWUNSPECIFIED => "ENTRY_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
            ProjectViewEnum::CUSTOM => "CUSTOM",
            ProjectViewEnum::ALL => "ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENTRY_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::ENTRYVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
           "FULL" => Ok(ProjectViewEnum::FULL),
           "CUSTOM" => Ok(ProjectViewEnum::CUSTOM),
           "ALL" => Ok(ProjectViewEnum::ALL),
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


