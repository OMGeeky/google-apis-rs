use super::*;



// region AggregationResultBatchMoreResultsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the query after the current batch. Only COUNT(*) aggregations are supported in the initial launch. Therefore, expected result type is limited to `NO_MORE_RESULTS`.
pub enum AggregationResultBatchMoreResultsEnum {
    

    /// Unspecified. This value is never used.
    ///
    /// "MORE_RESULTS_TYPE_UNSPECIFIED"
    #[serde(rename="MORE_RESULTS_TYPE_UNSPECIFIED")]
    MORERESULTSTYPEUNSPECIFIED,
    

    /// There may be additional batches to fetch from this query.
    ///
    /// "NOT_FINISHED"
    #[serde(rename="NOT_FINISHED")]
    NOTFINISHED,
    

    /// The query is finished, but there may be more results after the limit.
    ///
    /// "MORE_RESULTS_AFTER_LIMIT"
    #[serde(rename="MORE_RESULTS_AFTER_LIMIT")]
    MORERESULTSAFTERLIMIT,
    

    /// The query is finished, but there may be more results after the end cursor.
    ///
    /// "MORE_RESULTS_AFTER_CURSOR"
    #[serde(rename="MORE_RESULTS_AFTER_CURSOR")]
    MORERESULTSAFTERCURSOR,
    

    /// The query is finished, and there are no more results.
    ///
    /// "NO_MORE_RESULTS"
    #[serde(rename="NO_MORE_RESULTS")]
    NOMORERESULTS,
}

impl AsRef<str> for AggregationResultBatchMoreResultsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AggregationResultBatchMoreResultsEnum::MORERESULTSTYPEUNSPECIFIED => "MORE_RESULTS_TYPE_UNSPECIFIED",
            AggregationResultBatchMoreResultsEnum::NOTFINISHED => "NOT_FINISHED",
            AggregationResultBatchMoreResultsEnum::MORERESULTSAFTERLIMIT => "MORE_RESULTS_AFTER_LIMIT",
            AggregationResultBatchMoreResultsEnum::MORERESULTSAFTERCURSOR => "MORE_RESULTS_AFTER_CURSOR",
            AggregationResultBatchMoreResultsEnum::NOMORERESULTS => "NO_MORE_RESULTS",
        }
    }
}

impl std::convert::TryFrom< &str> for AggregationResultBatchMoreResultsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MORE_RESULTS_TYPE_UNSPECIFIED" => Ok(AggregationResultBatchMoreResultsEnum::MORERESULTSTYPEUNSPECIFIED),
           "NOT_FINISHED" => Ok(AggregationResultBatchMoreResultsEnum::NOTFINISHED),
           "MORE_RESULTS_AFTER_LIMIT" => Ok(AggregationResultBatchMoreResultsEnum::MORERESULTSAFTERLIMIT),
           "MORE_RESULTS_AFTER_CURSOR" => Ok(AggregationResultBatchMoreResultsEnum::MORERESULTSAFTERCURSOR),
           "NO_MORE_RESULTS" => Ok(AggregationResultBatchMoreResultsEnum::NOMORERESULTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AggregationResultBatchMoreResultsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommitRequestModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of commit to perform. Defaults to `TRANSACTIONAL`.
pub enum CommitRequestModeEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// Transactional: The mutations are either all applied, or none are applied. Learn about transactions [here](https://cloud.google.com/datastore/docs/concepts/transactions).
    ///
    /// "TRANSACTIONAL"
    #[serde(rename="TRANSACTIONAL")]
    TRANSACTIONAL,
    

    /// Non-transactional: The mutations may not apply as all or none.
    ///
    /// "NON_TRANSACTIONAL"
    #[serde(rename="NON_TRANSACTIONAL")]
    NONTRANSACTIONAL,
}

impl AsRef<str> for CommitRequestModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommitRequestModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            CommitRequestModeEnum::TRANSACTIONAL => "TRANSACTIONAL",
            CommitRequestModeEnum::NONTRANSACTIONAL => "NON_TRANSACTIONAL",
        }
    }
}

impl std::convert::TryFrom< &str> for CommitRequestModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(CommitRequestModeEnum::MODEUNSPECIFIED),
           "TRANSACTIONAL" => Ok(CommitRequestModeEnum::TRANSACTIONAL),
           "NON_TRANSACTIONAL" => Ok(CommitRequestModeEnum::NONTRANSACTIONAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommitRequestModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CompositeFilterOpEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator for combining multiple filters.
pub enum CompositeFilterOpEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The results are required to satisfy each of the combined filters.
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


// region PropertyFilterOpEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to filter by.
pub enum PropertyFilterOpEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The given `property` is less than the given `value`. Requires: * That `property` comes first in `order_by`.
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// The given `property` is less than or equal to the given `value`. Requires: * That `property` comes first in `order_by`.
    ///
    /// "LESS_THAN_OR_EQUAL"
    #[serde(rename="LESS_THAN_OR_EQUAL")]
    LESSTHANOREQUAL,
    

    /// The given `property` is greater than the given `value`. Requires: * That `property` comes first in `order_by`.
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// The given `property` is greater than or equal to the given `value`. Requires: * That `property` comes first in `order_by`.
    ///
    /// "GREATER_THAN_OR_EQUAL"
    #[serde(rename="GREATER_THAN_OR_EQUAL")]
    GREATERTHANOREQUAL,
    

    /// The given `property` is equal to the given `value`.
    ///
    /// "EQUAL"
    #[serde(rename="EQUAL")]
    EQUAL,
    

    /// The given `property` is equal to at least one value in the given array. Requires: * That `value` is a non-empty `ArrayValue`, subject to disjunction limits. * No `NOT_IN` is in the same query.
    ///
    /// "IN"
    #[serde(rename="IN")]
    IN,
    

    /// The given `property` is not equal to the given `value`. Requires: * No other `NOT_EQUAL` or `NOT_IN` is in the same query. * That `property` comes first in the `order_by`.
    ///
    /// "NOT_EQUAL"
    #[serde(rename="NOT_EQUAL")]
    NOTEQUAL,
    

    /// Limit the result set to the given entity and its descendants. Requires: * That `value` is an entity key. * All evaluated disjunctions must have the same `HAS_ANCESTOR` filter.
    ///
    /// "HAS_ANCESTOR"
    #[serde(rename="HAS_ANCESTOR")]
    HASANCESTOR,
    

    /// The value of the `property` is not in the given array. Requires: * That `value` is a non-empty `ArrayValue` with at most 10 values. * No other `OR`, `IN`, `NOT_IN`, `NOT_EQUAL` is in the same query. * That `field` comes first in the `order_by`.
    ///
    /// "NOT_IN"
    #[serde(rename="NOT_IN")]
    NOTIN,
}

impl AsRef<str> for PropertyFilterOpEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PropertyFilterOpEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            PropertyFilterOpEnum::LESSTHAN => "LESS_THAN",
            PropertyFilterOpEnum::LESSTHANOREQUAL => "LESS_THAN_OR_EQUAL",
            PropertyFilterOpEnum::GREATERTHAN => "GREATER_THAN",
            PropertyFilterOpEnum::GREATERTHANOREQUAL => "GREATER_THAN_OR_EQUAL",
            PropertyFilterOpEnum::EQUAL => "EQUAL",
            PropertyFilterOpEnum::IN => "IN",
            PropertyFilterOpEnum::NOTEQUAL => "NOT_EQUAL",
            PropertyFilterOpEnum::HASANCESTOR => "HAS_ANCESTOR",
            PropertyFilterOpEnum::NOTIN => "NOT_IN",
        }
    }
}

impl std::convert::TryFrom< &str> for PropertyFilterOpEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(PropertyFilterOpEnum::OPERATORUNSPECIFIED),
           "LESS_THAN" => Ok(PropertyFilterOpEnum::LESSTHAN),
           "LESS_THAN_OR_EQUAL" => Ok(PropertyFilterOpEnum::LESSTHANOREQUAL),
           "GREATER_THAN" => Ok(PropertyFilterOpEnum::GREATERTHAN),
           "GREATER_THAN_OR_EQUAL" => Ok(PropertyFilterOpEnum::GREATERTHANOREQUAL),
           "EQUAL" => Ok(PropertyFilterOpEnum::EQUAL),
           "IN" => Ok(PropertyFilterOpEnum::IN),
           "NOT_EQUAL" => Ok(PropertyFilterOpEnum::NOTEQUAL),
           "HAS_ANCESTOR" => Ok(PropertyFilterOpEnum::HASANCESTOR),
           "NOT_IN" => Ok(PropertyFilterOpEnum::NOTIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PropertyFilterOpEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PropertyOrderDirectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The direction to order by. Defaults to `ASCENDING`.
pub enum PropertyOrderDirectionEnum {
    

    /// Unspecified. This value must not be used.
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

impl AsRef<str> for PropertyOrderDirectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PropertyOrderDirectionEnum::DIRECTIONUNSPECIFIED => "DIRECTION_UNSPECIFIED",
            PropertyOrderDirectionEnum::ASCENDING => "ASCENDING",
            PropertyOrderDirectionEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PropertyOrderDirectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DIRECTION_UNSPECIFIED" => Ok(PropertyOrderDirectionEnum::DIRECTIONUNSPECIFIED),
           "ASCENDING" => Ok(PropertyOrderDirectionEnum::ASCENDING),
           "DESCENDING" => Ok(PropertyOrderDirectionEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PropertyOrderDirectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryResultBatchEntityResultTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The result type for every entity in `entity_results`.
pub enum QueryResultBatchEntityResultTypeEnum {
    

    /// Unspecified. This value is never used.
    ///
    /// "RESULT_TYPE_UNSPECIFIED"
    #[serde(rename="RESULT_TYPE_UNSPECIFIED")]
    RESULTTYPEUNSPECIFIED,
    

    /// The key and properties.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
    

    /// A projected subset of properties. The entity may have no key.
    ///
    /// "PROJECTION"
    #[serde(rename="PROJECTION")]
    PROJECTION,
    

    /// Only the key.
    ///
    /// "KEY_ONLY"
    #[serde(rename="KEY_ONLY")]
    KEYONLY,
}

impl AsRef<str> for QueryResultBatchEntityResultTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryResultBatchEntityResultTypeEnum::RESULTTYPEUNSPECIFIED => "RESULT_TYPE_UNSPECIFIED",
            QueryResultBatchEntityResultTypeEnum::FULL => "FULL",
            QueryResultBatchEntityResultTypeEnum::PROJECTION => "PROJECTION",
            QueryResultBatchEntityResultTypeEnum::KEYONLY => "KEY_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryResultBatchEntityResultTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESULT_TYPE_UNSPECIFIED" => Ok(QueryResultBatchEntityResultTypeEnum::RESULTTYPEUNSPECIFIED),
           "FULL" => Ok(QueryResultBatchEntityResultTypeEnum::FULL),
           "PROJECTION" => Ok(QueryResultBatchEntityResultTypeEnum::PROJECTION),
           "KEY_ONLY" => Ok(QueryResultBatchEntityResultTypeEnum::KEYONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryResultBatchEntityResultTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region QueryResultBatchMoreResultsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the query after the current batch.
pub enum QueryResultBatchMoreResultsEnum {
    

    /// Unspecified. This value is never used.
    ///
    /// "MORE_RESULTS_TYPE_UNSPECIFIED"
    #[serde(rename="MORE_RESULTS_TYPE_UNSPECIFIED")]
    MORERESULTSTYPEUNSPECIFIED,
    

    /// There may be additional batches to fetch from this query.
    ///
    /// "NOT_FINISHED"
    #[serde(rename="NOT_FINISHED")]
    NOTFINISHED,
    

    /// The query is finished, but there may be more results after the limit.
    ///
    /// "MORE_RESULTS_AFTER_LIMIT"
    #[serde(rename="MORE_RESULTS_AFTER_LIMIT")]
    MORERESULTSAFTERLIMIT,
    

    /// The query is finished, but there may be more results after the end cursor.
    ///
    /// "MORE_RESULTS_AFTER_CURSOR"
    #[serde(rename="MORE_RESULTS_AFTER_CURSOR")]
    MORERESULTSAFTERCURSOR,
    

    /// The query is finished, and there are no more results.
    ///
    /// "NO_MORE_RESULTS"
    #[serde(rename="NO_MORE_RESULTS")]
    NOMORERESULTS,
}

impl AsRef<str> for QueryResultBatchMoreResultsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            QueryResultBatchMoreResultsEnum::MORERESULTSTYPEUNSPECIFIED => "MORE_RESULTS_TYPE_UNSPECIFIED",
            QueryResultBatchMoreResultsEnum::NOTFINISHED => "NOT_FINISHED",
            QueryResultBatchMoreResultsEnum::MORERESULTSAFTERLIMIT => "MORE_RESULTS_AFTER_LIMIT",
            QueryResultBatchMoreResultsEnum::MORERESULTSAFTERCURSOR => "MORE_RESULTS_AFTER_CURSOR",
            QueryResultBatchMoreResultsEnum::NOMORERESULTS => "NO_MORE_RESULTS",
        }
    }
}

impl std::convert::TryFrom< &str> for QueryResultBatchMoreResultsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MORE_RESULTS_TYPE_UNSPECIFIED" => Ok(QueryResultBatchMoreResultsEnum::MORERESULTSTYPEUNSPECIFIED),
           "NOT_FINISHED" => Ok(QueryResultBatchMoreResultsEnum::NOTFINISHED),
           "MORE_RESULTS_AFTER_LIMIT" => Ok(QueryResultBatchMoreResultsEnum::MORERESULTSAFTERLIMIT),
           "MORE_RESULTS_AFTER_CURSOR" => Ok(QueryResultBatchMoreResultsEnum::MORERESULTSAFTERCURSOR),
           "NO_MORE_RESULTS" => Ok(QueryResultBatchMoreResultsEnum::NOMORERESULTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a QueryResultBatchMoreResultsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReadOptionReadConsistencyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The non-transactional read consistency to use.
pub enum ReadOptionReadConsistencyEnum {
    

    /// Unspecified. This value must not be used.
    ///
    /// "READ_CONSISTENCY_UNSPECIFIED"
    #[serde(rename="READ_CONSISTENCY_UNSPECIFIED")]
    READCONSISTENCYUNSPECIFIED,
    

    /// Strong consistency.
    ///
    /// "STRONG"
    #[serde(rename="STRONG")]
    STRONG,
    

    /// Eventual consistency.
    ///
    /// "EVENTUAL"
    #[serde(rename="EVENTUAL")]
    EVENTUAL,
}

impl AsRef<str> for ReadOptionReadConsistencyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReadOptionReadConsistencyEnum::READCONSISTENCYUNSPECIFIED => "READ_CONSISTENCY_UNSPECIFIED",
            ReadOptionReadConsistencyEnum::STRONG => "STRONG",
            ReadOptionReadConsistencyEnum::EVENTUAL => "EVENTUAL",
        }
    }
}

impl std::convert::TryFrom< &str> for ReadOptionReadConsistencyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "READ_CONSISTENCY_UNSPECIFIED" => Ok(ReadOptionReadConsistencyEnum::READCONSISTENCYUNSPECIFIED),
           "STRONG" => Ok(ReadOptionReadConsistencyEnum::STRONG),
           "EVENTUAL" => Ok(ReadOptionReadConsistencyEnum::EVENTUAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReadOptionReadConsistencyEnum {
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


