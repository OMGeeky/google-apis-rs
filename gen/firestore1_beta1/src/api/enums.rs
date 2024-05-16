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


// region GoogleFirestoreAdminV1beta1IndexStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the index. Output only.
pub enum GoogleFirestoreAdminV1beta1IndexStateEnum {
    

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
    

    /// The index was being created, but something went wrong. There is no active long-running operation for the index, and the most recently finished long-running operation failed. The index is not updated when writing a document. Some index data may exist.
    ///
    /// "ERROR"
    #[serde(rename="ERROR")]
    ERROR,
}

impl AsRef<str> for GoogleFirestoreAdminV1beta1IndexStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1beta1IndexStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            GoogleFirestoreAdminV1beta1IndexStateEnum::CREATING => "CREATING",
            GoogleFirestoreAdminV1beta1IndexStateEnum::READY => "READY",
            GoogleFirestoreAdminV1beta1IndexStateEnum::ERROR => "ERROR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1beta1IndexStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1beta1IndexStateEnum::STATEUNSPECIFIED),
           "CREATING" => Ok(GoogleFirestoreAdminV1beta1IndexStateEnum::CREATING),
           "READY" => Ok(GoogleFirestoreAdminV1beta1IndexStateEnum::READY),
           "ERROR" => Ok(GoogleFirestoreAdminV1beta1IndexStateEnum::ERROR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1beta1IndexStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleFirestoreAdminV1beta1IndexFieldModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The field's mode.
pub enum GoogleFirestoreAdminV1beta1IndexFieldModeEnum {
    

    /// The mode is unspecified.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// The field's values are indexed so as to support sequencing in ascending order and also query by <, >, <=, >=, and =.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// The field's values are indexed so as to support sequencing in descending order and also query by <, >, <=, >=, and =.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
    

    /// The field's array values are indexed so as to support membership using ARRAY_CONTAINS queries.
    ///
    /// "ARRAY_CONTAINS"
    #[serde(rename="ARRAY_CONTAINS")]
    ARRAYCONTAINS,
}

impl AsRef<str> for GoogleFirestoreAdminV1beta1IndexFieldModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleFirestoreAdminV1beta1IndexFieldModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            GoogleFirestoreAdminV1beta1IndexFieldModeEnum::ASCENDING => "ASCENDING",
            GoogleFirestoreAdminV1beta1IndexFieldModeEnum::DESCENDING => "DESCENDING",
            GoogleFirestoreAdminV1beta1IndexFieldModeEnum::ARRAYCONTAINS => "ARRAY_CONTAINS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleFirestoreAdminV1beta1IndexFieldModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(GoogleFirestoreAdminV1beta1IndexFieldModeEnum::MODEUNSPECIFIED),
           "ASCENDING" => Ok(GoogleFirestoreAdminV1beta1IndexFieldModeEnum::ASCENDING),
           "DESCENDING" => Ok(GoogleFirestoreAdminV1beta1IndexFieldModeEnum::DESCENDING),
           "ARRAY_CONTAINS" => Ok(GoogleFirestoreAdminV1beta1IndexFieldModeEnum::ARRAYCONTAINS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleFirestoreAdminV1beta1IndexFieldModeEnum {
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


