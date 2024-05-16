use super::*;



// region CompositeFilterOpEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator for combining multiple filters.
pub enum CompositeFilterOpEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// Documents are required to satisfy all of the combined filters.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
    

    /// Documents are required to satisfy at least one of the combined filters.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
}

impl AsRef<str> for CompositeFilterOpEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CompositeFilterOpEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            CompositeFilterOpEnum::AND => "AND",
            CompositeFilterOpEnum::OR => "OR",
        }
    }
}

impl std::convert::TryFrom< &str> for CompositeFilterOpEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(CompositeFilterOpEnum::OPERATORUNSPECIFIED),
           "AND" => Ok(CompositeFilterOpEnum::AND),
           "OR" => Ok(CompositeFilterOpEnum::OR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CompositeFilterOpEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FieldFilterOpEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to filter by.
pub enum FieldFilterOpEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The given `field` is less than the given `value`. Requires: * That `field` come first in `order_by`.
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// The given `field` is less than or equal to the given `value`. Requires: * That `field` come first in `order_by`.
    ///
    /// "LESS_THAN_OR_EQUAL"
    #[serde(rename="LESS_THAN_OR_EQUAL")]
    LESSTHANOREQUAL,
    

    /// The given `field` is greater than the given `value`. Requires: * That `field` come first in `order_by`.
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// The given `field` is greater than or equal to the given `value`. Requires: * That `field` come first in `order_by`.
    ///
    /// "GREATER_THAN_OR_EQUAL"
    #[serde(rename="GREATER_THAN_OR_EQUAL")]
    GREATERTHANOREQUAL,
    

    /// The given `field` is equal to the given `value`.
    ///
    /// "EQUAL"
    #[serde(rename="EQUAL")]
    EQUAL,
    

    /// The given `field` is not equal to the given `value`. Requires: * No other `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`.
    ///
    /// "NOT_EQUAL"
    #[serde(rename="NOT_EQUAL")]
    NOTEQUAL,
    

    /// The given `field` is an array that contains the given `value`.
    ///
    /// "ARRAY_CONTAINS"
    #[serde(rename="ARRAY_CONTAINS")]
    ARRAYCONTAINS,
    

    /// The given `field` is equal to at least one value in the given array. Requires: * That `value` is a non-empty `ArrayValue`, subject to disjunction limits. * No `NOT_IN` filters in the same query.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// The given `field` is an array that contains any of the values in the given array. Requires: * That `value` is a non-empty `ArrayValue`, subject to disjunction limits. * No other `ARRAY_CONTAINS_ANY` filters within the same disjunction. * No `NOT_IN` filters in the same query.
    ///
    /// "ARRAY_CONTAINS_ANY"
    #[serde(rename="ARRAY_CONTAINS_ANY")]
    ARRAYCONTAINSANY,
    

    /// The value of the `field` is not in the given array. Requires: * That `value` is a non-empty `ArrayValue` with at most 10 values. * No other `OR`, `IN`, `ARRAY_CONTAINS_ANY`, `NOT_IN`, `NOT_EQUAL`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`.
    ///
    /// "NOT_IN"
    #[serde(rename="NOT_IN")]
    NOTIN,
}

impl AsRef<str> for FieldFilterOpEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FieldFilterOpEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            FieldFilterOpEnum::LESSTHAN => "LESS_THAN",
            FieldFilterOpEnum::LESSTHANOREQUAL => "LESS_THAN_OR_EQUAL",
            FieldFilterOpEnum::GREATERTHAN => "GREATER_THAN",
            FieldFilterOpEnum::GREATERTHANOREQUAL => "GREATER_THAN_OR_EQUAL",
            FieldFilterOpEnum::EQUAL => "EQUAL",
            FieldFilterOpEnum::NOTEQUAL => "NOT_EQUAL",
            FieldFilterOpEnum::ARRAYCONTAINS => "ARRAY_CONTAINS",
            FieldFilterOpEnum::IN => "IN",
            FieldFilterOpEnum::ARRAYCONTAINSANY => "ARRAY_CONTAINS_ANY",
            FieldFilterOpEnum::NOTIN => "NOT_IN",
        }
    }
}

impl std::convert::TryFrom< &str> for FieldFilterOpEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(FieldFilterOpEnum::OPERATORUNSPECIFIED),
           "LESS_THAN" => Ok(FieldFilterOpEnum::LESSTHAN),
           "LESS_THAN_OR_EQUAL" => Ok(FieldFilterOpEnum::LESSTHANOREQUAL),
           "GREATER_THAN" => Ok(FieldFilterOpEnum::GREATERTHAN),
           "GREATER_THAN_OR_EQUAL" => Ok(FieldFilterOpEnum::GREATERTHANOREQUAL),
           "EQUAL" => Ok(FieldFilterOpEnum::EQUAL),
           "NOT_EQUAL" => Ok(FieldFilterOpEnum::NOTEQUAL),
           "ARRAY_CONTAINS" => Ok(FieldFilterOpEnum::ARRAYCONTAINS),
           "IN" => Ok(FieldFilterOpEnum::IN),
           "ARRAY_CONTAINS_ANY" => Ok(FieldFilterOpEnum::ARRAYCONTAINSANY),
           "NOT_IN" => Ok(FieldFilterOpEnum::NOTIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FieldFilterOpEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FieldTransformSetToServerValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sets the field to the given server value.
pub enum FieldTransformSetToServerValueEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "SERVER_VALUE_UNSPECIFIED"
    #[serde(rename="SERVER_VALUE_UNSPECIFIED")]
    SERVERVALUEUNSPECIFIED,
    

    /// The time at which the server processed the request, with millisecond precision. If used on multiple fields (same or different documents) in a transaction, all the fields will get the same server timestamp.
    ///
    /// "REQUEST_TIME"
    #[serde(rename="REQUEST_TIME")]
    REQUESTTIME,
}

impl AsRef<str> for FieldTransformSetToServerValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FieldTransformSetToServerValueEnum::SERVERVALUEUNSPECIFIED => "SERVER_VALUE_UNSPECIFIED",
            FieldTransformSetToServerValueEnum::REQUESTTIME => "REQUEST_TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for FieldTransformSetToServerValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SERVER_VALUE_UNSPECIFIED" => Ok(FieldTransformSetToServerValueEnum::SERVERVALUEUNSPECIFIED),
           "REQUEST_TIME" => Ok(FieldTransformSetToServerValueEnum::REQUESTTIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FieldTransformSetToServerValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FindNearestDistanceMeasureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The Distance Measure to use, required.
pub enum FindNearestDistanceMeasureEnum {
    

    /// Should not be set.
    ///
    /// "DISTANCE_MEASURE_UNSPECIFIED"
    #[serde(rename="DISTANCE_MEASURE_UNSPECIFIED")]
    DISTANCEMEASUREUNSPECIFIED,
    

    /// Measures the EUCLIDEAN distance between the vectors. See [Euclidean](https://en.wikipedia.org/wiki/Euclidean_distance) to learn more
    ///
    /// "EUCLIDEAN"
    #[serde(rename="EUCLIDEAN")]
    EUCLIDEAN,
    

    /// Compares vectors based on the angle between them, which allows you to measure similarity that isn't based on the vectors magnitude. We recommend using DOT_PRODUCT with unit normalized vectors instead of COSINE distance, which is mathematically equivalent with better performance. See [Cosine Similarity](https://en.wikipedia.org/wiki/Cosine_similarity) to learn more.
    ///
    /// "COSINE"
    #[serde(rename="COSINE")]
    COSINE,
    

    /// Similar to cosine but is affected by the magnitude of the vectors. See [Dot Product](https://en.wikipedia.org/wiki/Dot_product) to learn more.
    ///
    /// "DOT_PRODUCT"
    #[serde(rename="DOT_PRODUCT")]
    DOTPRODUCT,
}

impl AsRef<str> for FindNearestDistanceMeasureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FindNearestDistanceMeasureEnum::DISTANCEMEASUREUNSPECIFIED => "DISTANCE_MEASURE_UNSPECIFIED",
            FindNearestDistanceMeasureEnum::EUCLIDEAN => "EUCLIDEAN",
            FindNearestDistanceMeasureEnum::COSINE => "COSINE",
            FindNearestDistanceMeasureEnum::DOTPRODUCT => "DOT_PRODUCT",
        }
    }
}

impl std::convert::TryFrom< &str> for FindNearestDistanceMeasureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISTANCE_MEASURE_UNSPECIFIED" => Ok(FindNearestDistanceMeasureEnum::DISTANCEMEASUREUNSPECIFIED),
           "EUCLIDEAN" => Ok(FindNearestDistanceMeasureEnum::EUCLIDEAN),
           "COSINE" => Ok(FindNearestDistanceMeasureEnum::COSINE),
           "DOT_PRODUCT" => Ok(FindNearestDistanceMeasureEnum::DOTPRODUCT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FindNearestDistanceMeasureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1BackupStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The current state of the backup.
pub enum GoogleFirestoreAdminV1BackupStateEnum {
    

    /// The state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The pending backup is still being created. Operations on the backup will be rejected in this state.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The backup is complete and ready to use.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The backup is not available at this moment.
    ///
    /// "NOT_AVAILABLE"
    #[serde(rename="NOT_AVAILABLE")]
    NOTAVAILABLE,
}

impl AsRef<str> for GoogleFirestoreAdminV1BackupStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1BackupStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleFirestoreAdminV1BackupStateEnum::CREATING => "CREATING",
            GoogleFirestoreAdminV1BackupStateEnum::READY => "READY",
            GoogleFirestoreAdminV1BackupStateEnum::NOTAVAILABLE => "NOT_AVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1BackupStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1BackupStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleFirestoreAdminV1BackupStateEnum::CREATING),
           "READY" => Ok(GoogleFirestoreAdminV1BackupStateEnum::READY),
           "NOT_AVAILABLE" => Ok(GoogleFirestoreAdminV1BackupStateEnum::NOTAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1BackupStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The App Engine integration mode to use for this database.
pub enum GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum {
    

    /// Not used.
    ///
    /// "APP_ENGINE_INTEGRATION_MODE_UNSPECIFIED"
    #[serde(rename="APP_ENGINE_INTEGRATION_MODE_UNSPECIFIED")]
    APPENGINEINTEGRATIONMODEUNSPECIFIED,
    

    /// If an App Engine application exists in the same region as this database, App Engine configuration will impact this database. This includes disabling of the application & database, as well as disabling writes to the database.
    ///
    /// "ENABLED"
    #[serde(rename="ENABLED")]
    ENABLED,
    

    /// App Engine has no effect on the ability of this database to serve requests. This is the default setting for databases created with the Firestore API.
    ///
    /// "DISABLED"
    #[serde(rename="DISABLED")]
    DISABLED,
}

impl AsRef<str> for GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum::APPENGINEINTEGRATIONMODEUNSPECIFIED => "APP_ENGINE_INTEGRATION_MODE_UNSPECIFIED",
            GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum::ENABLED => "ENABLED",
            GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum::DISABLED => "DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "APP_ENGINE_INTEGRATION_MODE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum::APPENGINEINTEGRATIONMODEUNSPECIFIED),
           "ENABLED" => Ok(GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum::ENABLED),
           "DISABLED" => Ok(GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum::DISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1DatabaseAppEngineIntegrationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The concurrency control mode to use for this database.
pub enum GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum {
    

    /// Not used.
    ///
    /// "CONCURRENCY_MODE_UNSPECIFIED"
    #[serde(rename="CONCURRENCY_MODE_UNSPECIFIED")]
    CONCURRENCYMODEUNSPECIFIED,
    

    /// Use optimistic concurrency control by default. This mode is available for Cloud Firestore databases.
    ///
    /// "OPTIMISTIC"
    #[serde(rename="OPTIMISTIC")]
    OPTIMISTIC,
    

    /// Use pessimistic concurrency control by default. This mode is available for Cloud Firestore databases. This is the default setting for Cloud Firestore.
    ///
    /// "PESSIMISTIC"
    #[serde(rename="PESSIMISTIC")]
    PESSIMISTIC,
    

    /// Use optimistic concurrency control with entity groups by default. This is the only available mode for Cloud Datastore. This mode is also available for Cloud Firestore with Datastore Mode but is not recommended.
    ///
    /// "OPTIMISTIC_WITH_ENTITY_GROUPS"
    #[serde(rename="OPTIMISTIC_WITH_ENTITY_GROUPS")]
    OPTIMISTICWITHENTITYGROUPS,
}

impl AsRef<str> for GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::CONCURRENCYMODEUNSPECIFIED => "CONCURRENCY_MODE_UNSPECIFIED",
            GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::OPTIMISTIC => "OPTIMISTIC",
            GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::PESSIMISTIC => "PESSIMISTIC",
            GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::OPTIMISTICWITHENTITYGROUPS => "OPTIMISTIC_WITH_ENTITY_GROUPS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONCURRENCY_MODE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::CONCURRENCYMODEUNSPECIFIED),
           "OPTIMISTIC" => Ok(GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::OPTIMISTIC),
           "PESSIMISTIC" => Ok(GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::PESSIMISTIC),
           "OPTIMISTIC_WITH_ENTITY_GROUPS" => Ok(GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum::OPTIMISTICWITHENTITYGROUPS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1DatabaseConcurrencyModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of delete protection for the database.
pub enum GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum {
    

    /// The default value. Delete protection type is not specified
    ///
    /// "DELETE_PROTECTION_STATE_UNSPECIFIED"
    #[serde(rename="DELETE_PROTECTION_STATE_UNSPECIFIED")]
    DELETEPROTECTIONSTATEUNSPECIFIED,
    

    /// Delete protection is disabled
    ///
    /// "DELETE_PROTECTION_DISABLED"
    #[serde(rename="DELETE_PROTECTION_DISABLED")]
    DELETEPROTECTIONDISABLED,
    

    /// Delete protection is enabled
    ///
    /// "DELETE_PROTECTION_ENABLED"
    #[serde(rename="DELETE_PROTECTION_ENABLED")]
    DELETEPROTECTIONENABLED,
}

impl AsRef<str> for GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum::DELETEPROTECTIONSTATEUNSPECIFIED => "DELETE_PROTECTION_STATE_UNSPECIFIED",
            GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum::DELETEPROTECTIONDISABLED => "DELETE_PROTECTION_DISABLED",
            GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum::DELETEPROTECTIONENABLED => "DELETE_PROTECTION_ENABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DELETE_PROTECTION_STATE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum::DELETEPROTECTIONSTATEUNSPECIFIED),
           "DELETE_PROTECTION_DISABLED" => Ok(GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum::DELETEPROTECTIONDISABLED),
           "DELETE_PROTECTION_ENABLED" => Ok(GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum::DELETEPROTECTIONENABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1DatabaseDeleteProtectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Whether to enable the PITR feature on this database.
pub enum GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum {
    

    /// Not used.
    ///
    /// "POINT_IN_TIME_RECOVERY_ENABLEMENT_UNSPECIFIED"
    #[serde(rename="POINT_IN_TIME_RECOVERY_ENABLEMENT_UNSPECIFIED")]
    POINTINTIMERECOVERYENABLEMENTUNSPECIFIED,
    

    /// Reads are supported on selected versions of the data from within the past 7 days: * Reads against any timestamp within the past hour * Reads against 1-minute snapshots beyond 1 hour and within 7 days `version_retention_period` and `earliest_version_time` can be used to determine the supported versions.
    ///
    /// "POINT_IN_TIME_RECOVERY_ENABLED"
    #[serde(rename="POINT_IN_TIME_RECOVERY_ENABLED")]
    POINTINTIMERECOVERYENABLED,
    

    /// Reads are supported on any version of the data from within the past 1 hour.
    ///
    /// "POINT_IN_TIME_RECOVERY_DISABLED"
    #[serde(rename="POINT_IN_TIME_RECOVERY_DISABLED")]
    POINTINTIMERECOVERYDISABLED,
}

impl AsRef<str> for GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum::POINTINTIMERECOVERYENABLEMENTUNSPECIFIED => "POINT_IN_TIME_RECOVERY_ENABLEMENT_UNSPECIFIED",
            GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum::POINTINTIMERECOVERYENABLED => "POINT_IN_TIME_RECOVERY_ENABLED",
            GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum::POINTINTIMERECOVERYDISABLED => "POINT_IN_TIME_RECOVERY_DISABLED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "POINT_IN_TIME_RECOVERY_ENABLEMENT_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum::POINTINTIMERECOVERYENABLEMENTUNSPECIFIED),
           "POINT_IN_TIME_RECOVERY_ENABLED" => Ok(GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum::POINTINTIMERECOVERYENABLED),
           "POINT_IN_TIME_RECOVERY_DISABLED" => Ok(GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum::POINTINTIMERECOVERYDISABLED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1DatabasePointInTimeRecoveryEnablementEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1DatabaseTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the database. See https://cloud.google.com/datastore/docs/firestore-or-datastore for information about how to choose.
pub enum GoogleFirestoreAdminV1DatabaseTypeEnum {
    

    /// The default value. This value is used if the database type is omitted.
    ///
    /// "DATABASE_TYPE_UNSPECIFIED"
    #[serde(rename="DATABASE_TYPE_UNSPECIFIED")]
    DATABASETYPEUNSPECIFIED,
    

    /// Firestore Native Mode
    ///
    /// "FIRESTORE_NATIVE"
    #[serde(rename="FIRESTORE_NATIVE")]
    FIRESTORENATIVE,
    

    /// Firestore in Datastore Mode.
    ///
    /// "DATASTORE_MODE"
    #[serde(rename="DATASTORE_MODE")]
    DATASTOREMODE,
}

impl AsRef<str> for GoogleFirestoreAdminV1DatabaseTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1DatabaseTypeEnum::DATABASETYPEUNSPECIFIED => "DATABASE_TYPE_UNSPECIFIED",
            GoogleFirestoreAdminV1DatabaseTypeEnum::FIRESTORENATIVE => "FIRESTORE_NATIVE",
            GoogleFirestoreAdminV1DatabaseTypeEnum::DATASTOREMODE => "DATASTORE_MODE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1DatabaseTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DATABASE_TYPE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1DatabaseTypeEnum::DATABASETYPEUNSPECIFIED),
           "FIRESTORE_NATIVE" => Ok(GoogleFirestoreAdminV1DatabaseTypeEnum::FIRESTORENATIVE),
           "DATASTORE_MODE" => Ok(GoogleFirestoreAdminV1DatabaseTypeEnum::DATASTOREMODE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1DatabaseTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1IndexApiScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The API scope supported by this index.
pub enum GoogleFirestoreAdminV1IndexApiScopeEnum {
    

    /// The index can only be used by the Firestore Native query API. This is the default.
    ///
    /// "ANY_API"
    #[serde(rename="ANY_API")]
    ANYAPI,
    

    /// The index can only be used by the Firestore in Datastore Mode query API.
    ///
    /// "DATASTORE_MODE_API"
    #[serde(rename="DATASTORE_MODE_API")]
    DATASTOREMODEAPI,
}

impl AsRef<str> for GoogleFirestoreAdminV1IndexApiScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1IndexApiScopeEnum::ANYAPI => "ANY_API",
            GoogleFirestoreAdminV1IndexApiScopeEnum::DATASTOREMODEAPI => "DATASTORE_MODE_API",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1IndexApiScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANY_API" => Ok(GoogleFirestoreAdminV1IndexApiScopeEnum::ANYAPI),
           "DATASTORE_MODE_API" => Ok(GoogleFirestoreAdminV1IndexApiScopeEnum::DATASTOREMODEAPI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1IndexApiScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1IndexQueryScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index.
pub enum GoogleFirestoreAdminV1IndexQueryScopeEnum {
    

    /// The query scope is unspecified. Not a valid option.
    ///
    /// "QUERY_SCOPE_UNSPECIFIED"
    #[serde(rename="QUERY_SCOPE_UNSPECIFIED")]
    QUERYSCOPEUNSPECIFIED,
    

    /// Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the collection id specified by the index.
    ///
    /// "COLLECTION"
    #[serde(rename="COLLECTION")]
    COLLECTION,
    

    /// Indexes with a collection group query scope specified allow queries against all collections that has the collection id specified by the index.
    ///
    /// "COLLECTION_GROUP"
    #[serde(rename="COLLECTION_GROUP")]
    COLLECTIONGROUP,
    

    /// Include all the collections's ancestor in the index. Only available for Datastore Mode databases.
    ///
    /// "COLLECTION_RECURSIVE"
    #[serde(rename="COLLECTION_RECURSIVE")]
    COLLECTIONRECURSIVE,
}

impl AsRef<str> for GoogleFirestoreAdminV1IndexQueryScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1IndexQueryScopeEnum::QUERYSCOPEUNSPECIFIED => "QUERY_SCOPE_UNSPECIFIED",
            GoogleFirestoreAdminV1IndexQueryScopeEnum::COLLECTION => "COLLECTION",
            GoogleFirestoreAdminV1IndexQueryScopeEnum::COLLECTIONGROUP => "COLLECTION_GROUP",
            GoogleFirestoreAdminV1IndexQueryScopeEnum::COLLECTIONRECURSIVE => "COLLECTION_RECURSIVE",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1IndexQueryScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "QUERY_SCOPE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1IndexQueryScopeEnum::QUERYSCOPEUNSPECIFIED),
           "COLLECTION" => Ok(GoogleFirestoreAdminV1IndexQueryScopeEnum::COLLECTION),
           "COLLECTION_GROUP" => Ok(GoogleFirestoreAdminV1IndexQueryScopeEnum::COLLECTIONGROUP),
           "COLLECTION_RECURSIVE" => Ok(GoogleFirestoreAdminV1IndexQueryScopeEnum::COLLECTIONRECURSIVE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1IndexQueryScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1IndexStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The serving state of the index.
pub enum GoogleFirestoreAdminV1IndexStateEnum {
    

    /// The state is unspecified.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The index is being created. There is an active long-running operation for the index. The index is updated when writing a document. Some index data may exist.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The index is ready to be used. The index is updated when writing a document. The index is fully populated from all stored documents it applies to.
    ///
    /// "READY"
    #[serde(rename="READY")]
    READY,
    

    /// The index was being created, but something went wrong. There is no active long-running operation for the index, and the most recently finished long-running operation failed. The index is not updated when writing a document. Some index data may exist. Use the google.longrunning.Operations API to determine why the operation that last attempted to create this index failed, then re-create the index.
    ///
    /// "NEEDS_REPAIR"
    #[serde(rename="NEEDS_REPAIR")]
    NEEDSREPAIR,
}

impl AsRef<str> for GoogleFirestoreAdminV1IndexStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1IndexStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleFirestoreAdminV1IndexStateEnum::CREATING => "CREATING",
            GoogleFirestoreAdminV1IndexStateEnum::READY => "READY",
            GoogleFirestoreAdminV1IndexStateEnum::NEEDSREPAIR => "NEEDS_REPAIR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1IndexStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1IndexStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleFirestoreAdminV1IndexStateEnum::CREATING),
           "READY" => Ok(GoogleFirestoreAdminV1IndexStateEnum::READY),
           "NEEDS_REPAIR" => Ok(GoogleFirestoreAdminV1IndexStateEnum::NEEDSREPAIR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1IndexStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1IndexFieldArrayConfigEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates that this field supports operations on `array_value`s.
pub enum GoogleFirestoreAdminV1IndexFieldArrayConfigEnum {
    

    /// The index does not support additional array queries.
    ///
    /// "ARRAY_CONFIG_UNSPECIFIED"
    #[serde(rename="ARRAY_CONFIG_UNSPECIFIED")]
    ARRAYCONFIGUNSPECIFIED,
    

    /// The index supports array containment queries.
    ///
    /// "CONTAINS"
    #[serde(rename="CONTAINS")]
    CONTAINS,
}

impl AsRef<str> for GoogleFirestoreAdminV1IndexFieldArrayConfigEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1IndexFieldArrayConfigEnum::ARRAYCONFIGUNSPECIFIED => "ARRAY_CONFIG_UNSPECIFIED",
            GoogleFirestoreAdminV1IndexFieldArrayConfigEnum::CONTAINS => "CONTAINS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1IndexFieldArrayConfigEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ARRAY_CONFIG_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1IndexFieldArrayConfigEnum::ARRAYCONFIGUNSPECIFIED),
           "CONTAINS" => Ok(GoogleFirestoreAdminV1IndexFieldArrayConfigEnum::CONTAINS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1IndexFieldArrayConfigEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1IndexFieldOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates that this field supports ordering by the specified order or comparing using =, !=, <, <=, >, >=.
pub enum GoogleFirestoreAdminV1IndexFieldOrderEnum {
    

    /// The ordering is unspecified. Not a valid option.
    ///
    /// "ORDER_UNSPECIFIED"
    #[serde(rename="ORDER_UNSPECIFIED")]
    ORDERUNSPECIFIED,
    

    /// The field is ordered by ascending field value.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// The field is ordered by descending field value.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for GoogleFirestoreAdminV1IndexFieldOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1IndexFieldOrderEnum::ORDERUNSPECIFIED => "ORDER_UNSPECIFIED",
            GoogleFirestoreAdminV1IndexFieldOrderEnum::ASCENDING => "ASCENDING",
            GoogleFirestoreAdminV1IndexFieldOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1IndexFieldOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1IndexFieldOrderEnum::ORDERUNSPECIFIED),
           "ASCENDING" => Ok(GoogleFirestoreAdminV1IndexFieldOrderEnum::ASCENDING),
           "DESCENDING" => Ok(GoogleFirestoreAdminV1IndexFieldOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1IndexFieldOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1TtlConfigStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. The state of the TTL configuration.
pub enum GoogleFirestoreAdminV1TtlConfigStateEnum {
    

    /// The state is unspecified or unknown.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The TTL is being applied. There is an active long-running operation to track the change. Newly written documents will have TTLs applied as requested. Requested TTLs on existing documents are still being processed. When TTLs on all existing documents have been processed, the state will move to 'ACTIVE'.
    ///
    /// "CREATING"
    #[serde(rename="CREATING")]
    CREATING,
    

    /// The TTL is active for all documents.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The TTL configuration could not be enabled for all existing documents. Newly written documents will continue to have their TTL applied. The LRO returned when last attempting to enable TTL for this `Field` has failed, and may have more details.
    ///
    /// "NEEDS_REPAIR"
    #[serde(rename="NEEDS_REPAIR")]
    NEEDSREPAIR,
}

impl AsRef<str> for GoogleFirestoreAdminV1TtlConfigStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1TtlConfigStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleFirestoreAdminV1TtlConfigStateEnum::CREATING => "CREATING",
            GoogleFirestoreAdminV1TtlConfigStateEnum::ACTIVE => "ACTIVE",
            GoogleFirestoreAdminV1TtlConfigStateEnum::NEEDSREPAIR => "NEEDS_REPAIR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1TtlConfigStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1TtlConfigStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleFirestoreAdminV1TtlConfigStateEnum::CREATING),
           "ACTIVE" => Ok(GoogleFirestoreAdminV1TtlConfigStateEnum::ACTIVE),
           "NEEDS_REPAIR" => Ok(GoogleFirestoreAdminV1TtlConfigStateEnum::NEEDSREPAIR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1TtlConfigStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The day of week to run. DAY_OF_WEEK_UNSPECIFIED is not allowed.
pub enum GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum {
    

    /// The day of the week is unspecified.
    ///
    /// "DAY_OF_WEEK_UNSPECIFIED"
    #[serde(rename="DAY_OF_WEEK_UNSPECIFIED")]
    DAYOFWEEKUNSPECIFIED,
    

    /// Monday
    ///
    /// "MONDAY"
    #[serde(rename="MONDAY")]
    MONDAY,
    

    /// Tuesday
    ///
    /// "TUESDAY"
    #[serde(rename="TUESDAY")]
    TUESDAY,
    

    /// Wednesday
    ///
    /// "WEDNESDAY"
    #[serde(rename="WEDNESDAY")]
    WEDNESDAY,
    

    /// Thursday
    ///
    /// "THURSDAY"
    #[serde(rename="THURSDAY")]
    THURSDAY,
    

    /// Friday
    ///
    /// "FRIDAY"
    #[serde(rename="FRIDAY")]
    FRIDAY,
    

    /// Saturday
    ///
    /// "SATURDAY"
    #[serde(rename="SATURDAY")]
    SATURDAY,
    

    /// Sunday
    ///
    /// "SUNDAY"
    #[serde(rename="SUNDAY")]
    SUNDAY,
}

impl AsRef<str> for GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::DAYOFWEEKUNSPECIFIED => "DAY_OF_WEEK_UNSPECIFIED",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::MONDAY => "MONDAY",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::TUESDAY => "TUESDAY",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::WEDNESDAY => "WEDNESDAY",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::THURSDAY => "THURSDAY",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::FRIDAY => "FRIDAY",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::SATURDAY => "SATURDAY",
            GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::SUNDAY => "SUNDAY",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DAY_OF_WEEK_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::DAYOFWEEKUNSPECIFIED),
           "MONDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::MONDAY),
           "TUESDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::TUESDAY),
           "WEDNESDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::WEDNESDAY),
           "THURSDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::THURSDAY),
           "FRIDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::FRIDAY),
           "SATURDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::SATURDAY),
           "SUNDAY" => Ok(GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum::SUNDAY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1WeeklyRecurrenceDayEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The direction to order by. Defaults to `ASCENDING`.
pub enum OrderDirectionEnum {
    

    /// Unspecified.
    ///
    /// "DIRECTION_UNSPECIFIED"
    #[serde(rename="DIRECTION_UNSPECIFIED")]
    DIRECTIONUNSPECIFIED,
    

    /// Ascending.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Descending.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for OrderDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderDirectionEnum::DIRECTIONUNSPECIFIED => "DIRECTION_UNSPECIFIED",
            OrderDirectionEnum::ASCENDING => "ASCENDING",
            OrderDirectionEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIRECTION_UNSPECIFIED" => Ok(OrderDirectionEnum::DIRECTIONUNSPECIFIED),
           "ASCENDING" => Ok(OrderDirectionEnum::ASCENDING),
           "DESCENDING" => Ok(OrderDirectionEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region TargetChangeTargetChangeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of change that occurred.
pub enum TargetChangeTargetChangeTypeEnum {
    

    /// No change has occurred. Used only to send an updated `resume_token`.
    ///
    /// "NO_CHANGE"
    #[serde(rename="NO_CHANGE")]
    NOCHANGE,
    

    /// The targets have been added.
    ///
    /// "ADD"
    #[serde(rename="ADD")]
    ADD,
    

    /// The targets have been removed.
    ///
    /// "REMOVE"
    #[serde(rename="REMOVE")]
    REMOVE,
    

    /// The targets reflect all changes committed before the targets were added to the stream. This will be sent after or with a `read_time` that is greater than or equal to the time at which the targets were added. Listeners can wait for this change if read-after-write semantics are desired.
    ///
    /// "CURRENT"
    #[serde(rename="CURRENT")]
    CURRENT,
    

    /// The targets have been reset, and a new initial state for the targets will be returned in subsequent changes. After the initial state is complete, `CURRENT` will be returned even if the target was previously indicated to be `CURRENT`.
    ///
    /// "RESET"
    #[serde(rename="RESET")]
    RESET,
}

impl AsRef<str> for TargetChangeTargetChangeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            TargetChangeTargetChangeTypeEnum::NOCHANGE => "NO_CHANGE",
            TargetChangeTargetChangeTypeEnum::ADD => "ADD",
            TargetChangeTargetChangeTypeEnum::REMOVE => "REMOVE",
            TargetChangeTargetChangeTypeEnum::CURRENT => "CURRENT",
            TargetChangeTargetChangeTypeEnum::RESET => "RESET",
        }
    }
}

impl std::convert::TryFrom< &str> for TargetChangeTargetChangeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_CHANGE" => Ok(TargetChangeTargetChangeTypeEnum::NOCHANGE),
           "ADD" => Ok(TargetChangeTargetChangeTypeEnum::ADD),
           "REMOVE" => Ok(TargetChangeTargetChangeTypeEnum::REMOVE),
           "CURRENT" => Ok(TargetChangeTargetChangeTypeEnum::CURRENT),
           "RESET" => Ok(TargetChangeTargetChangeTypeEnum::RESET),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a TargetChangeTargetChangeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UnaryFilterOpEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The unary operator to apply.
pub enum UnaryFilterOpEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The given `field` is equal to `NaN`.
    ///
    /// "IS_NAN"
    #[serde(rename="IS_NAN")]
    ISNAN,
    

    /// The given `field` is equal to `NULL`.
    ///
    /// "IS_NULL"
    #[serde(rename="IS_NULL")]
    ISNULL,
    

    /// The given `field` is not equal to `NaN`. Requires: * No other `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`.
    ///
    /// "IS_NOT_NAN"
    #[serde(rename="IS_NOT_NAN")]
    ISNOTNAN,
    

    /// The given `field` is not equal to `NULL`. Requires: * A single `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`.
    ///
    /// "IS_NOT_NULL"
    #[serde(rename="IS_NOT_NULL")]
    ISNOTNULL,
}

impl AsRef<str> for UnaryFilterOpEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UnaryFilterOpEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            UnaryFilterOpEnum::ISNAN => "IS_NAN",
            UnaryFilterOpEnum::ISNULL => "IS_NULL",
            UnaryFilterOpEnum::ISNOTNAN => "IS_NOT_NAN",
            UnaryFilterOpEnum::ISNOTNULL => "IS_NOT_NULL",
        }
    }
}

impl std::convert::TryFrom< &str> for UnaryFilterOpEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(UnaryFilterOpEnum::OPERATORUNSPECIFIED),
           "IS_NAN" => Ok(UnaryFilterOpEnum::ISNAN),
           "IS_NULL" => Ok(UnaryFilterOpEnum::ISNULL),
           "IS_NOT_NAN" => Ok(UnaryFilterOpEnum::ISNOTNAN),
           "IS_NOT_NULL" => Ok(UnaryFilterOpEnum::ISNOTNULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UnaryFilterOpEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ValueNullValueEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A null value.
pub enum ValueNullValueEnum {
    

    /// Null value.
    ///
    /// "NULL_VALUE"
    #[serde(rename="NULL_VALUE")]
    NULLVALUE,
}

impl AsRef<str> for ValueNullValueEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ValueNullValueEnum::NULLVALUE => "NULL_VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for ValueNullValueEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NULL_VALUE" => Ok(ValueNullValueEnum::NULLVALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ValueNullValueEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


