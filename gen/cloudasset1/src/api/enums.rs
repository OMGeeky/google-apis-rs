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


// region ConditionEvaluationEvaluationValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The evaluation result.
pub enum ConditionEvaluationEvaluationValueEnum {
    

    /// Reserved for future use.
    ///
    /// "EVALUATION_VALUE_UNSPECIFIED"
    #[serde(rename="EVALUATION_VALUE_UNSPECIFIED")]
    EVALUATIONVALUEUNSPECIFIED,
    

    /// The evaluation result is `true`.
    ///
    /// "TRUE"
    #[serde(rename="TRUE")]
    TRUE,
    

    /// The evaluation result is `false`.
    ///
    /// "FALSE"
    #[serde(rename="FALSE")]
    FALSE,
    

    /// The evaluation result is `conditional` when the condition expression contains variables that are either missing input values or have not been supported by Policy Analyzer yet.
    ///
    /// "CONDITIONAL"
    #[serde(rename="CONDITIONAL")]
    CONDITIONAL,
}

impl AsRef<str> for ConditionEvaluationEvaluationValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ConditionEvaluationEvaluationValueEnum::EVALUATIONVALUEUNSPECIFIED => "EVALUATION_VALUE_UNSPECIFIED",
            ConditionEvaluationEvaluationValueEnum::TRUE => "TRUE",
            ConditionEvaluationEvaluationValueEnum::FALSE => "FALSE",
            ConditionEvaluationEvaluationValueEnum::CONDITIONAL => "CONDITIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ConditionEvaluationEvaluationValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "EVALUATION_VALUE_UNSPECIFIED" => Ok(ConditionEvaluationEvaluationValueEnum::EVALUATIONVALUEUNSPECIFIED),
           "TRUE" => Ok(ConditionEvaluationEvaluationValueEnum::TRUE),
           "FALSE" => Ok(ConditionEvaluationEvaluationValueEnum::FALSE),
           "CONDITIONAL" => Ok(ConditionEvaluationEvaluationValueEnum::CONDITIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ConditionEvaluationEvaluationValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ExportAssetsRequestContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Asset content type. If not specified, no content but the asset name will be returned.
pub enum ExportAssetsRequestContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
    

    /// The organization policy set on an asset.
    ///
    /// "ORG_POLICY"
    #[serde(rename="ORG_POLICY")]
    ORGPOLICY,
    

    /// The Access Context Manager policy set on an asset.
    ///
    /// "ACCESS_POLICY"
    #[serde(rename="ACCESS_POLICY")]
    ACCESSPOLICY,
    

    /// The runtime OS Inventory information.
    ///
    /// "OS_INVENTORY"
    #[serde(rename="OS_INVENTORY")]
    OSINVENTORY,
    

    /// The related resources.
    ///
    /// "RELATIONSHIP"
    #[serde(rename="RELATIONSHIP")]
    RELATIONSHIP,
}

impl AsRef<str> for ExportAssetsRequestContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ExportAssetsRequestContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            ExportAssetsRequestContentTypeEnum::RESOURCE => "RESOURCE",
            ExportAssetsRequestContentTypeEnum::IAMPOLICY => "IAM_POLICY",
            ExportAssetsRequestContentTypeEnum::ORGPOLICY => "ORG_POLICY",
            ExportAssetsRequestContentTypeEnum::ACCESSPOLICY => "ACCESS_POLICY",
            ExportAssetsRequestContentTypeEnum::OSINVENTORY => "OS_INVENTORY",
            ExportAssetsRequestContentTypeEnum::RELATIONSHIP => "RELATIONSHIP",
        }
    }
}

impl std::convert::TryFrom< &str> for ExportAssetsRequestContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(ExportAssetsRequestContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(ExportAssetsRequestContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(ExportAssetsRequestContentTypeEnum::IAMPOLICY),
           "ORG_POLICY" => Ok(ExportAssetsRequestContentTypeEnum::ORGPOLICY),
           "ACCESS_POLICY" => Ok(ExportAssetsRequestContentTypeEnum::ACCESSPOLICY),
           "OS_INVENTORY" => Ok(ExportAssetsRequestContentTypeEnum::OSINVENTORY),
           "RELATIONSHIP" => Ok(ExportAssetsRequestContentTypeEnum::RELATIONSHIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ExportAssetsRequestContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeedContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Asset content type. If not specified, no content but the asset name and type will be returned.
pub enum FeedContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
    

    /// The organization policy set on an asset.
    ///
    /// "ORG_POLICY"
    #[serde(rename="ORG_POLICY")]
    ORGPOLICY,
    

    /// The Access Context Manager policy set on an asset.
    ///
    /// "ACCESS_POLICY"
    #[serde(rename="ACCESS_POLICY")]
    ACCESSPOLICY,
    

    /// The runtime OS Inventory information.
    ///
    /// "OS_INVENTORY"
    #[serde(rename="OS_INVENTORY")]
    OSINVENTORY,
    

    /// The related resources.
    ///
    /// "RELATIONSHIP"
    #[serde(rename="RELATIONSHIP")]
    RELATIONSHIP,
}

impl AsRef<str> for FeedContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeedContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            FeedContentTypeEnum::RESOURCE => "RESOURCE",
            FeedContentTypeEnum::IAMPOLICY => "IAM_POLICY",
            FeedContentTypeEnum::ORGPOLICY => "ORG_POLICY",
            FeedContentTypeEnum::ACCESSPOLICY => "ACCESS_POLICY",
            FeedContentTypeEnum::OSINVENTORY => "OS_INVENTORY",
            FeedContentTypeEnum::RELATIONSHIP => "RELATIONSHIP",
        }
    }
}

impl std::convert::TryFrom< &str> for FeedContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(FeedContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(FeedContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(FeedContentTypeEnum::IAMPOLICY),
           "ORG_POLICY" => Ok(FeedContentTypeEnum::ORGPOLICY),
           "ACCESS_POLICY" => Ok(FeedContentTypeEnum::ACCESSPOLICY),
           "OS_INVENTORY" => Ok(FeedContentTypeEnum::OSINVENTORY),
           "RELATIONSHIP" => Ok(FeedContentTypeEnum::RELATIONSHIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeedContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The partition key for BigQuery partitioned table.
pub enum GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum {
    

    /// Unspecified partition key. Tables won't be partitioned using this option.
    ///
    /// "PARTITION_KEY_UNSPECIFIED"
    #[serde(rename="PARTITION_KEY_UNSPECIFIED")]
    PARTITIONKEYUNSPECIFIED,
    

    /// The time when the request is received. If specified as partition key, the result table(s) is partitoned by the RequestTime column, an additional timestamp column representing when the request was received.
    ///
    /// "REQUEST_TIME"
    #[serde(rename="REQUEST_TIME")]
    REQUESTTIME,
}

impl AsRef<str> for GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum::PARTITIONKEYUNSPECIFIED => "PARTITION_KEY_UNSPECIFIED",
            GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum::REQUESTTIME => "REQUEST_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTITION_KEY_UNSPECIFIED" => Ok(GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum::PARTITIONKEYUNSPECIFIED),
           "REQUEST_TIME" => Ok(GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum::REQUESTTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssetV1ConstraintConstraintDefaultEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The evaluation behavior of this constraint in the absence of 'Policy'.
pub enum GoogleCloudAssetV1ConstraintConstraintDefaultEnum {
    

    /// This is only used for distinguishing unset values and should never be used.
    ///
    /// "CONSTRAINT_DEFAULT_UNSPECIFIED"
    #[serde(rename="CONSTRAINT_DEFAULT_UNSPECIFIED")]
    CONSTRAINTDEFAULTUNSPECIFIED,
    

    /// Indicate that all values are allowed for list constraints. Indicate that enforcement is off for boolean constraints.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Indicate that all values are denied for list constraints. Indicate that enforcement is on for boolean constraints.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GoogleCloudAssetV1ConstraintConstraintDefaultEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssetV1ConstraintConstraintDefaultEnum::CONSTRAINTDEFAULTUNSPECIFIED => "CONSTRAINT_DEFAULT_UNSPECIFIED",
            GoogleCloudAssetV1ConstraintConstraintDefaultEnum::ALLOW => "ALLOW",
            GoogleCloudAssetV1ConstraintConstraintDefaultEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssetV1ConstraintConstraintDefaultEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONSTRAINT_DEFAULT_UNSPECIFIED" => Ok(GoogleCloudAssetV1ConstraintConstraintDefaultEnum::CONSTRAINTDEFAULTUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudAssetV1ConstraintConstraintDefaultEnum::ALLOW),
           "DENY" => Ok(GoogleCloudAssetV1ConstraintConstraintDefaultEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssetV1ConstraintConstraintDefaultEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssetV1CustomConstraintActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allow or deny type.
pub enum GoogleCloudAssetV1CustomConstraintActionTypeEnum {
    

    /// Unspecified. Will results in user error.
    ///
    /// "ACTION_TYPE_UNSPECIFIED"
    #[serde(rename="ACTION_TYPE_UNSPECIFIED")]
    ACTIONTYPEUNSPECIFIED,
    

    /// Allowed action type.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// Deny action type.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GoogleCloudAssetV1CustomConstraintActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssetV1CustomConstraintActionTypeEnum::ACTIONTYPEUNSPECIFIED => "ACTION_TYPE_UNSPECIFIED",
            GoogleCloudAssetV1CustomConstraintActionTypeEnum::ALLOW => "ALLOW",
            GoogleCloudAssetV1CustomConstraintActionTypeEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssetV1CustomConstraintActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssetV1CustomConstraintActionTypeEnum::ACTIONTYPEUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudAssetV1CustomConstraintActionTypeEnum::ALLOW),
           "DENY" => Ok(GoogleCloudAssetV1CustomConstraintActionTypeEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssetV1CustomConstraintActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudAssetV1CustomConstraintMethodTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// All the operations being applied for this constraint.
pub enum GoogleCloudAssetV1CustomConstraintMethodTypesEnum {
    

    /// Unspecified. Will results in user error.
    ///
    /// "METHOD_TYPE_UNSPECIFIED"
    #[serde(rename="METHOD_TYPE_UNSPECIFIED")]
    METHODTYPEUNSPECIFIED,
    

    /// Constraint applied when creating the resource.
    ///
    /// "CREATE"
    #[serde(rename="CREATE")]
    CREATE,
    

    /// Constraint applied when updating the resource.
    ///
    /// "UPDATE"
    #[serde(rename="UPDATE")]
    UPDATE,
    

    /// Constraint applied when deleting the resource.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
}

impl AsRef<str> for GoogleCloudAssetV1CustomConstraintMethodTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudAssetV1CustomConstraintMethodTypesEnum::METHODTYPEUNSPECIFIED => "METHOD_TYPE_UNSPECIFIED",
            GoogleCloudAssetV1CustomConstraintMethodTypesEnum::CREATE => "CREATE",
            GoogleCloudAssetV1CustomConstraintMethodTypesEnum::UPDATE => "UPDATE",
            GoogleCloudAssetV1CustomConstraintMethodTypesEnum::DELETE => "DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudAssetV1CustomConstraintMethodTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METHOD_TYPE_UNSPECIFIED" => Ok(GoogleCloudAssetV1CustomConstraintMethodTypesEnum::METHODTYPEUNSPECIFIED),
           "CREATE" => Ok(GoogleCloudAssetV1CustomConstraintMethodTypesEnum::CREATE),
           "UPDATE" => Ok(GoogleCloudAssetV1CustomConstraintMethodTypesEnum::UPDATE),
           "DELETE" => Ok(GoogleCloudAssetV1CustomConstraintMethodTypesEnum::DELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudAssetV1CustomConstraintMethodTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The policy all_values state.
pub enum GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    

    /// Indicates that allowed_values or denied_values must be set.
    ///
    /// "ALL_VALUES_UNSPECIFIED"
    #[serde(rename="ALL_VALUES_UNSPECIFIED")]
    ALLVALUESUNSPECIFIED,
    

    /// A policy with this set allows all values.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    

    /// A policy with this set denies all values.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
}

impl AsRef<str> for GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLVALUESUNSPECIFIED => "ALL_VALUES_UNSPECIFIED",
            GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLOW => "ALLOW",
            GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::DENY => "DENY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_VALUES_UNSPECIFIED" => Ok(GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLVALUESUNSPECIFIED),
           "ALLOW" => Ok(GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::ALLOW),
           "DENY" => Ok(GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum::DENY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND.
pub enum GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    

    /// All `Conditions` must be true for the `BasicLevel` to be true.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
    

    /// If at least one `Condition` is true, then the `BasicLevel` is true.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::AND => "AND",
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::OR => "OR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "AND" => Ok(GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::AND),
           "OR" => Ok(GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum::OR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed device management levels, an empty list allows all management levels.
pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    

    /// The device's management level is not specified or not known.
    ///
    /// "MANAGEMENT_UNSPECIFIED"
    #[serde(rename="MANAGEMENT_UNSPECIFIED")]
    MANAGEMENTUNSPECIFIED,
    

    /// The device is not managed.
    ///
    /// "NONE"
    #[serde(rename="NONE")]
    NONE,
    

    /// Basic management is enabled, which is generally limited to monitoring and wiping the corporate account.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Complete device management. This includes more thorough monitoring and the ability to directly manage the device (such as remote wiping). This can be enabled through the Android Enterprise Platform.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::MANAGEMENTUNSPECIFIED => "MANAGEMENT_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::NONE => "NONE",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::BASIC => "BASIC",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MANAGEMENT_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::MANAGEMENTUNSPECIFIED),
           "NONE" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::NONE),
           "BASIC" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::BASIC),
           "COMPLETE" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum::COMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Allowed encryptions statuses, an empty list allows all statuses.
pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    

    /// The encryption status of the device is not specified or not known.
    ///
    /// "ENCRYPTION_UNSPECIFIED"
    #[serde(rename="ENCRYPTION_UNSPECIFIED")]
    ENCRYPTIONUNSPECIFIED,
    

    /// The device does not support encryption.
    ///
    /// "ENCRYPTION_UNSUPPORTED"
    #[serde(rename="ENCRYPTION_UNSUPPORTED")]
    ENCRYPTIONUNSUPPORTED,
    

    /// The device supports encryption, but is currently unencrypted.
    ///
    /// "UNENCRYPTED"
    #[serde(rename="UNENCRYPTED")]
    UNENCRYPTED,
    

    /// The device is encrypted.
    ///
    /// "ENCRYPTED"
    #[serde(rename="ENCRYPTED")]
    ENCRYPTED,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSPECIFIED => "ENCRYPTION_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSUPPORTED => "ENCRYPTION_UNSUPPORTED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::UNENCRYPTED => "UNENCRYPTED",
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTED => "ENCRYPTED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ENCRYPTION_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSPECIFIED),
           "ENCRYPTION_UNSUPPORTED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTIONUNSUPPORTED),
           "UNENCRYPTED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::UNENCRYPTED),
           "ENCRYPTED" => Ok(GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum::ENCRYPTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
pub enum GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    

    /// No blanket identity group specified.
    ///
    /// "IDENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="IDENTITY_TYPE_UNSPECIFIED")]
    IDENTITYTYPEUNSPECIFIED,
    

    /// Authorize access from all identities outside the perimeter.
    ///
    /// "ANY_IDENTITY"
    #[serde(rename="ANY_IDENTITY")]
    ANYIDENTITY,
    

    /// Authorize access from all human users outside the perimeter.
    ///
    /// "ANY_USER_ACCOUNT"
    #[serde(rename="ANY_USER_ACCOUNT")]
    ANYUSERACCOUNT,
    

    /// Authorize access from all service accounts outside the perimeter.
    ///
    /// "ANY_SERVICE_ACCOUNT"
    #[serde(rename="ANY_SERVICE_ACCOUNT")]
    ANYSERVICEACCOUNT,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED => "IDENTITY_TYPE_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYIDENTITY => "ANY_IDENTITY",
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYUSERACCOUNT => "ANY_USER_ACCOUNT",
            GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYSERVICEACCOUNT => "ANY_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY_TYPE_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED),
           "ANY_IDENTITY" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYIDENTITY),
           "ANY_USER_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYUSERACCOUNT),
           "ANY_SERVICE_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum::ANYSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether to enforce traffic restrictions based on `sources` field. If the `sources` fields is non-empty, then this field must be set to `SOURCE_RESTRICTION_ENABLED`.
pub enum GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    

    /// Enforcement preference unspecified, will not enforce traffic restrictions based on `sources` in EgressFrom.
    ///
    /// "SOURCE_RESTRICTION_UNSPECIFIED"
    #[serde(rename="SOURCE_RESTRICTION_UNSPECIFIED")]
    SOURCERESTRICTIONUNSPECIFIED,
    

    /// Enforcement preference enabled, traffic restrictions will be enforced based on `sources` in EgressFrom.
    ///
    /// "SOURCE_RESTRICTION_ENABLED"
    #[serde(rename="SOURCE_RESTRICTION_ENABLED")]
    SOURCERESTRICTIONENABLED,
    

    /// Enforcement preference disabled, will not enforce traffic restrictions based on `sources` in EgressFrom.
    ///
    /// "SOURCE_RESTRICTION_DISABLED"
    #[serde(rename="SOURCE_RESTRICTION_DISABLED")]
    SOURCERESTRICTIONDISABLED,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONUNSPECIFIED => "SOURCE_RESTRICTION_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONENABLED => "SOURCE_RESTRICTION_ENABLED",
            GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONDISABLED => "SOURCE_RESTRICTION_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SOURCE_RESTRICTION_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONUNSPECIFIED),
           "SOURCE_RESTRICTION_ENABLED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONENABLED),
           "SOURCE_RESTRICTION_DISABLED" => Ok(GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum::SOURCERESTRICTIONDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1EgressFromSourceRestrictionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access.
pub enum GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    

    /// No blanket identity group specified.
    ///
    /// "IDENTITY_TYPE_UNSPECIFIED"
    #[serde(rename="IDENTITY_TYPE_UNSPECIFIED")]
    IDENTITYTYPEUNSPECIFIED,
    

    /// Authorize access from all identities outside the perimeter.
    ///
    /// "ANY_IDENTITY"
    #[serde(rename="ANY_IDENTITY")]
    ANYIDENTITY,
    

    /// Authorize access from all human users outside the perimeter.
    ///
    /// "ANY_USER_ACCOUNT"
    #[serde(rename="ANY_USER_ACCOUNT")]
    ANYUSERACCOUNT,
    

    /// Authorize access from all service accounts outside the perimeter.
    ///
    /// "ANY_SERVICE_ACCOUNT"
    #[serde(rename="ANY_SERVICE_ACCOUNT")]
    ANYSERVICEACCOUNT,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED => "IDENTITY_TYPE_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYIDENTITY => "ANY_IDENTITY",
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYUSERACCOUNT => "ANY_USER_ACCOUNT",
            GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYSERVICEACCOUNT => "ANY_SERVICE_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "IDENTITY_TYPE_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::IDENTITYTYPEUNSPECIFIED),
           "ANY_IDENTITY" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYIDENTITY),
           "ANY_USER_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYUSERACCOUNT),
           "ANY_SERVICE_ACCOUNT" => Ok(GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum::ANYSERVICEACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The allowed OS type.
pub enum GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    

    /// The operating system of the device is not specified or not known.
    ///
    /// "OS_UNSPECIFIED"
    #[serde(rename="OS_UNSPECIFIED")]
    OSUNSPECIFIED,
    

    /// A desktop Mac operating system.
    ///
    /// "DESKTOP_MAC"
    #[serde(rename="DESKTOP_MAC")]
    DESKTOPMAC,
    

    /// A desktop Windows operating system.
    ///
    /// "DESKTOP_WINDOWS"
    #[serde(rename="DESKTOP_WINDOWS")]
    DESKTOPWINDOWS,
    

    /// A desktop Linux operating system.
    ///
    /// "DESKTOP_LINUX"
    #[serde(rename="DESKTOP_LINUX")]
    DESKTOPLINUX,
    

    /// A desktop ChromeOS operating system.
    ///
    /// "DESKTOP_CHROME_OS"
    #[serde(rename="DESKTOP_CHROME_OS")]
    DESKTOPCHROMEOS,
    

    /// An Android operating system.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// An iOS operating system.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::OSUNSPECIFIED => "OS_UNSPECIFIED",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPMAC => "DESKTOP_MAC",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPWINDOWS => "DESKTOP_WINDOWS",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPLINUX => "DESKTOP_LINUX",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPCHROMEOS => "DESKTOP_CHROME_OS",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::ANDROID => "ANDROID",
            GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::IOS => "IOS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OS_UNSPECIFIED" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::OSUNSPECIFIED),
           "DESKTOP_MAC" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPMAC),
           "DESKTOP_WINDOWS" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPWINDOWS),
           "DESKTOP_LINUX" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPLINUX),
           "DESKTOP_CHROME_OS" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::DESKTOPCHROMEOS),
           "ANDROID" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::ANDROID),
           "IOS" => Ok(GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum::IOS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Perimeter type indicator. A single project or VPC network is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty.
pub enum GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    

    /// Regular Perimeter. When no value is specified, the perimeter uses this type.
    ///
    /// "PERIMETER_TYPE_REGULAR"
    #[serde(rename="PERIMETER_TYPE_REGULAR")]
    PERIMETERTYPEREGULAR,
    

    /// Perimeter Bridge.
    ///
    /// "PERIMETER_TYPE_BRIDGE"
    #[serde(rename="PERIMETER_TYPE_BRIDGE")]
    PERIMETERTYPEBRIDGE,
}

impl AsRef<str> for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEREGULAR => "PERIMETER_TYPE_REGULAR",
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEBRIDGE => "PERIMETER_TYPE_BRIDGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERIMETER_TYPE_REGULAR" => Ok(GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEREGULAR),
           "PERIMETER_TYPE_BRIDGE" => Ok(GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum::PERIMETERTYPEBRIDGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region IamPolicyAnalysisStateCodeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;
pub enum IamPolicyAnalysisStateCodeEnum {
    

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

impl AsRef<str> for IamPolicyAnalysisStateCodeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            IamPolicyAnalysisStateCodeEnum::OK => "OK",
            IamPolicyAnalysisStateCodeEnum::CANCELLED => "CANCELLED",
            IamPolicyAnalysisStateCodeEnum::UNKNOWN => "UNKNOWN",
            IamPolicyAnalysisStateCodeEnum::INVALIDARGUMENT => "INVALID_ARGUMENT",
            IamPolicyAnalysisStateCodeEnum::DEADLINEEXCEEDED => "DEADLINE_EXCEEDED",
            IamPolicyAnalysisStateCodeEnum::NOTFOUND => "NOT_FOUND",
            IamPolicyAnalysisStateCodeEnum::ALREADYEXISTS => "ALREADY_EXISTS",
            IamPolicyAnalysisStateCodeEnum::PERMISSIONDENIED => "PERMISSION_DENIED",
            IamPolicyAnalysisStateCodeEnum::UNAUTHENTICATED => "UNAUTHENTICATED",
            IamPolicyAnalysisStateCodeEnum::RESOURCEEXHAUSTED => "RESOURCE_EXHAUSTED",
            IamPolicyAnalysisStateCodeEnum::FAILEDPRECONDITION => "FAILED_PRECONDITION",
            IamPolicyAnalysisStateCodeEnum::ABORTED => "ABORTED",
            IamPolicyAnalysisStateCodeEnum::OUTOFRANGE => "OUT_OF_RANGE",
            IamPolicyAnalysisStateCodeEnum::UNIMPLEMENTED => "UNIMPLEMENTED",
            IamPolicyAnalysisStateCodeEnum::INTERNAL => "INTERNAL",
            IamPolicyAnalysisStateCodeEnum::UNAVAILABLE => "UNAVAILABLE",
            IamPolicyAnalysisStateCodeEnum::DATALOSS => "DATA_LOSS",
        }
    }
}

impl std::convert::TryFrom< &str> for IamPolicyAnalysisStateCodeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OK" => Ok(IamPolicyAnalysisStateCodeEnum::OK),
           "CANCELLED" => Ok(IamPolicyAnalysisStateCodeEnum::CANCELLED),
           "UNKNOWN" => Ok(IamPolicyAnalysisStateCodeEnum::UNKNOWN),
           "INVALID_ARGUMENT" => Ok(IamPolicyAnalysisStateCodeEnum::INVALIDARGUMENT),
           "DEADLINE_EXCEEDED" => Ok(IamPolicyAnalysisStateCodeEnum::DEADLINEEXCEEDED),
           "NOT_FOUND" => Ok(IamPolicyAnalysisStateCodeEnum::NOTFOUND),
           "ALREADY_EXISTS" => Ok(IamPolicyAnalysisStateCodeEnum::ALREADYEXISTS),
           "PERMISSION_DENIED" => Ok(IamPolicyAnalysisStateCodeEnum::PERMISSIONDENIED),
           "UNAUTHENTICATED" => Ok(IamPolicyAnalysisStateCodeEnum::UNAUTHENTICATED),
           "RESOURCE_EXHAUSTED" => Ok(IamPolicyAnalysisStateCodeEnum::RESOURCEEXHAUSTED),
           "FAILED_PRECONDITION" => Ok(IamPolicyAnalysisStateCodeEnum::FAILEDPRECONDITION),
           "ABORTED" => Ok(IamPolicyAnalysisStateCodeEnum::ABORTED),
           "OUT_OF_RANGE" => Ok(IamPolicyAnalysisStateCodeEnum::OUTOFRANGE),
           "UNIMPLEMENTED" => Ok(IamPolicyAnalysisStateCodeEnum::UNIMPLEMENTED),
           "INTERNAL" => Ok(IamPolicyAnalysisStateCodeEnum::INTERNAL),
           "UNAVAILABLE" => Ok(IamPolicyAnalysisStateCodeEnum::UNAVAILABLE),
           "DATA_LOSS" => Ok(IamPolicyAnalysisStateCodeEnum::DATALOSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a IamPolicyAnalysisStateCodeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ItemOriginTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The origin of this inventory item.
pub enum ItemOriginTypeEnum {
    

    /// Invalid. An origin type must be specified.
    ///
    /// "ORIGIN_TYPE_UNSPECIFIED"
    #[serde(rename="ORIGIN_TYPE_UNSPECIFIED")]
    ORIGINTYPEUNSPECIFIED,
    

    /// This inventory item was discovered as the result of the agent reporting inventory via the reporting API.
    ///
    /// "INVENTORY_REPORT"
    #[serde(rename="INVENTORY_REPORT")]
    INVENTORYREPORT,
}

impl AsRef<str> for ItemOriginTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ItemOriginTypeEnum::ORIGINTYPEUNSPECIFIED => "ORIGIN_TYPE_UNSPECIFIED",
            ItemOriginTypeEnum::INVENTORYREPORT => "INVENTORY_REPORT",
        }
    }
}

impl std::convert::TryFrom< &str> for ItemOriginTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORIGIN_TYPE_UNSPECIFIED" => Ok(ItemOriginTypeEnum::ORIGINTYPEUNSPECIFIED),
           "INVENTORY_REPORT" => Ok(ItemOriginTypeEnum::INVENTORYREPORT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ItemOriginTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ItemTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The specific type of inventory, correlating to its specific details.
pub enum ItemTypeEnum {
    

    /// Invalid. An type must be specified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// This represents a package that is installed on the VM.
    ///
    /// "INSTALLED_PACKAGE"
    #[serde(rename="INSTALLED_PACKAGE")]
    INSTALLEDPACKAGE,
    

    /// This represents an update that is available for a package.
    ///
    /// "AVAILABLE_PACKAGE"
    #[serde(rename="AVAILABLE_PACKAGE")]
    AVAILABLEPACKAGE,
}

impl AsRef<str> for ItemTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ItemTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            ItemTypeEnum::INSTALLEDPACKAGE => "INSTALLED_PACKAGE",
            ItemTypeEnum::AVAILABLEPACKAGE => "AVAILABLE_PACKAGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ItemTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(ItemTypeEnum::TYPEUNSPECIFIED),
           "INSTALLED_PACKAGE" => Ok(ItemTypeEnum::INSTALLEDPACKAGE),
           "AVAILABLE_PACKAGE" => Ok(ItemTypeEnum::AVAILABLEPACKAGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ItemTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PartitionSpecPartitionKeyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The partition key for BigQuery partitioned table.
pub enum PartitionSpecPartitionKeyEnum {
    

    /// Unspecified partition key. If used, it means using non-partitioned table.
    ///
    /// "PARTITION_KEY_UNSPECIFIED"
    #[serde(rename="PARTITION_KEY_UNSPECIFIED")]
    PARTITIONKEYUNSPECIFIED,
    

    /// The time when the snapshot is taken. If specified as partition key, the result table(s) is partitoned by the additional timestamp column, readTime. If [read_time] in ExportAssetsRequest is specified, the readTime column's value will be the same as it. Otherwise, its value will be the current time that is used to take the snapshot.
    ///
    /// "READ_TIME"
    #[serde(rename="READ_TIME")]
    READTIME,
    

    /// The time when the request is received and started to be processed. If specified as partition key, the result table(s) is partitoned by the requestTime column, an additional timestamp column representing when the request was received.
    ///
    /// "REQUEST_TIME"
    #[serde(rename="REQUEST_TIME")]
    REQUESTTIME,
}

impl AsRef<str> for PartitionSpecPartitionKeyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PartitionSpecPartitionKeyEnum::PARTITIONKEYUNSPECIFIED => "PARTITION_KEY_UNSPECIFIED",
            PartitionSpecPartitionKeyEnum::READTIME => "READ_TIME",
            PartitionSpecPartitionKeyEnum::REQUESTTIME => "REQUEST_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for PartitionSpecPartitionKeyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PARTITION_KEY_UNSPECIFIED" => Ok(PartitionSpecPartitionKeyEnum::PARTITIONKEYUNSPECIFIED),
           "READ_TIME" => Ok(PartitionSpecPartitionKeyEnum::READTIME),
           "REQUEST_TIME" => Ok(PartitionSpecPartitionKeyEnum::REQUESTTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PartitionSpecPartitionKeyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TemporalAssetPriorAssetStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of prior_asset.
pub enum TemporalAssetPriorAssetStateEnum {
    

    /// prior_asset is not applicable for the current asset.
    ///
    /// "PRIOR_ASSET_STATE_UNSPECIFIED"
    #[serde(rename="PRIOR_ASSET_STATE_UNSPECIFIED")]
    PRIORASSETSTATEUNSPECIFIED,
    

    /// prior_asset is populated correctly.
    ///
    /// "PRESENT"
    #[serde(rename="PRESENT")]
    PRESENT,
    

    /// Failed to set prior_asset.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
    

    /// Current asset is the first known state.
    ///
    /// "DOES_NOT_EXIST"
    #[serde(rename="DOES_NOT_EXIST")]
    DOESNOTEXIST,
    

    /// prior_asset is a deletion.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for TemporalAssetPriorAssetStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TemporalAssetPriorAssetStateEnum::PRIORASSETSTATEUNSPECIFIED => "PRIOR_ASSET_STATE_UNSPECIFIED",
            TemporalAssetPriorAssetStateEnum::PRESENT => "PRESENT",
            TemporalAssetPriorAssetStateEnum::INVALID => "INVALID",
            TemporalAssetPriorAssetStateEnum::DOESNOTEXIST => "DOES_NOT_EXIST",
            TemporalAssetPriorAssetStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for TemporalAssetPriorAssetStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PRIOR_ASSET_STATE_UNSPECIFIED" => Ok(TemporalAssetPriorAssetStateEnum::PRIORASSETSTATEUNSPECIFIED),
           "PRESENT" => Ok(TemporalAssetPriorAssetStateEnum::PRESENT),
           "INVALID" => Ok(TemporalAssetPriorAssetStateEnum::INVALID),
           "DOES_NOT_EXIST" => Ok(TemporalAssetPriorAssetStateEnum::DOESNOTEXIST),
           "DELETED" => Ok(TemporalAssetPriorAssetStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TemporalAssetPriorAssetStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssetContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Asset content type. If not specified, no content but the asset name will be returned.
pub enum AssetContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
    

    /// The organization policy set on an asset.
    ///
    /// "ORG_POLICY"
    #[serde(rename="ORG_POLICY")]
    ORGPOLICY,
    

    /// The Access Context Manager policy set on an asset.
    ///
    /// "ACCESS_POLICY"
    #[serde(rename="ACCESS_POLICY")]
    ACCESSPOLICY,
    

    /// The runtime OS Inventory information.
    ///
    /// "OS_INVENTORY"
    #[serde(rename="OS_INVENTORY")]
    OSINVENTORY,
    

    /// The related resources.
    ///
    /// "RELATIONSHIP"
    #[serde(rename="RELATIONSHIP")]
    RELATIONSHIP,
}

impl AsRef<str> for AssetContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssetContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            AssetContentTypeEnum::RESOURCE => "RESOURCE",
            AssetContentTypeEnum::IAMPOLICY => "IAM_POLICY",
            AssetContentTypeEnum::ORGPOLICY => "ORG_POLICY",
            AssetContentTypeEnum::ACCESSPOLICY => "ACCESS_POLICY",
            AssetContentTypeEnum::OSINVENTORY => "OS_INVENTORY",
            AssetContentTypeEnum::RELATIONSHIP => "RELATIONSHIP",
        }
    }
}

impl std::convert::TryFrom< &str> for AssetContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(AssetContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(AssetContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(AssetContentTypeEnum::IAMPOLICY),
           "ORG_POLICY" => Ok(AssetContentTypeEnum::ORGPOLICY),
           "ACCESS_POLICY" => Ok(AssetContentTypeEnum::ACCESSPOLICY),
           "OS_INVENTORY" => Ok(AssetContentTypeEnum::OSINVENTORY),
           "RELATIONSHIP" => Ok(AssetContentTypeEnum::RELATIONSHIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssetContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MethodViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Analysis view indicating what information should be included in the analysis response. If unspecified, the default view is FULL.
pub enum MethodViewEnum {
    

    /// The default/unset value. The API will default to the FULL view.
    ///
    /// "ANALYSIS_VIEW_UNSPECIFIED"
    #[serde(rename="ANALYSIS_VIEW_UNSPECIFIED")]
    ANALYSISVIEWUNSPECIFIED,
    

    /// Full analysis including all level of impacts of the specified resource move.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// Basic analysis only including blockers which will prevent the specified resource move at runtime.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
}

impl AsRef<str> for MethodViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MethodViewEnum::ANALYSISVIEWUNSPECIFIED => "ANALYSIS_VIEW_UNSPECIFIED",
            MethodViewEnum::FULL => "FULL",
            MethodViewEnum::BASIC => "BASIC",
        }
    }
}

impl std::convert::TryFrom< &str> for MethodViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANALYSIS_VIEW_UNSPECIFIED" => Ok(MethodViewEnum::ANALYSISVIEWUNSPECIFIED),
           "FULL" => Ok(MethodViewEnum::FULL),
           "BASIC" => Ok(MethodViewEnum::BASIC),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MethodViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MethodContentTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The content type.
pub enum MethodContentTypeEnum {
    

    /// Unspecified content type.
    ///
    /// "CONTENT_TYPE_UNSPECIFIED"
    #[serde(rename="CONTENT_TYPE_UNSPECIFIED")]
    CONTENTTYPEUNSPECIFIED,
    

    /// Resource metadata.
    ///
    /// "RESOURCE"
    #[serde(rename="RESOURCE")]
    RESOURCE,
    

    /// The actual IAM policy set on a resource.
    ///
    /// "IAM_POLICY"
    #[serde(rename="IAM_POLICY")]
    IAMPOLICY,
    

    /// The organization policy set on an asset.
    ///
    /// "ORG_POLICY"
    #[serde(rename="ORG_POLICY")]
    ORGPOLICY,
    

    /// The Access Context Manager policy set on an asset.
    ///
    /// "ACCESS_POLICY"
    #[serde(rename="ACCESS_POLICY")]
    ACCESSPOLICY,
    

    /// The runtime OS Inventory information.
    ///
    /// "OS_INVENTORY"
    #[serde(rename="OS_INVENTORY")]
    OSINVENTORY,
    

    /// The related resources.
    ///
    /// "RELATIONSHIP"
    #[serde(rename="RELATIONSHIP")]
    RELATIONSHIP,
}

impl AsRef<str> for MethodContentTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MethodContentTypeEnum::CONTENTTYPEUNSPECIFIED => "CONTENT_TYPE_UNSPECIFIED",
            MethodContentTypeEnum::RESOURCE => "RESOURCE",
            MethodContentTypeEnum::IAMPOLICY => "IAM_POLICY",
            MethodContentTypeEnum::ORGPOLICY => "ORG_POLICY",
            MethodContentTypeEnum::ACCESSPOLICY => "ACCESS_POLICY",
            MethodContentTypeEnum::OSINVENTORY => "OS_INVENTORY",
            MethodContentTypeEnum::RELATIONSHIP => "RELATIONSHIP",
        }
    }
}

impl std::convert::TryFrom< &str> for MethodContentTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONTENT_TYPE_UNSPECIFIED" => Ok(MethodContentTypeEnum::CONTENTTYPEUNSPECIFIED),
           "RESOURCE" => Ok(MethodContentTypeEnum::RESOURCE),
           "IAM_POLICY" => Ok(MethodContentTypeEnum::IAMPOLICY),
           "ORG_POLICY" => Ok(MethodContentTypeEnum::ORGPOLICY),
           "ACCESS_POLICY" => Ok(MethodContentTypeEnum::ACCESSPOLICY),
           "OS_INVENTORY" => Ok(MethodContentTypeEnum::OSINVENTORY),
           "RELATIONSHIP" => Ok(MethodContentTypeEnum::RELATIONSHIP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MethodContentTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


