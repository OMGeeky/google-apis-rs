use super::*;



// region ActivityActivityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this activity.
pub enum ActivityActivityTypeEnum {
    

    /// ActivityType will never have this value in the response. Using this type in the request will result in an error.
    ///
    /// "ACTIVITY_TYPE_UNSPECIFIED"
    #[serde(rename="ACTIVITY_TYPE_UNSPECIFIED")]
    ACTIVITYTYPEUNSPECIFIED,
    

    /// Used when the activity resulted out of a visitor viewing a page.
    ///
    /// "PAGEVIEW"
    #[serde(rename="PAGEVIEW")]
    PAGEVIEW,
    

    /// Used when the activity resulted out of a visitor using an application on a mobile device.
    ///
    /// "SCREENVIEW"
    #[serde(rename="SCREENVIEW")]
    SCREENVIEW,
    

    /// Used to denote that a goal type activity.
    ///
    /// "GOAL"
    #[serde(rename="GOAL")]
    GOAL,
    

    /// An e-commerce transaction was performed by the visitor on the page.
    ///
    /// "ECOMMERCE"
    #[serde(rename="ECOMMERCE")]
    ECOMMERCE,
    

    /// Used when the activity is an event.
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
}

impl AsRef<str> for ActivityActivityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityActivityTypeEnum::ACTIVITYTYPEUNSPECIFIED => "ACTIVITY_TYPE_UNSPECIFIED",
            ActivityActivityTypeEnum::PAGEVIEW => "PAGEVIEW",
            ActivityActivityTypeEnum::SCREENVIEW => "SCREENVIEW",
            ActivityActivityTypeEnum::GOAL => "GOAL",
            ActivityActivityTypeEnum::ECOMMERCE => "ECOMMERCE",
            ActivityActivityTypeEnum::EVENT => "EVENT",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityActivityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVITY_TYPE_UNSPECIFIED" => Ok(ActivityActivityTypeEnum::ACTIVITYTYPEUNSPECIFIED),
           "PAGEVIEW" => Ok(ActivityActivityTypeEnum::PAGEVIEW),
           "SCREENVIEW" => Ok(ActivityActivityTypeEnum::SCREENVIEW),
           "GOAL" => Ok(ActivityActivityTypeEnum::GOAL),
           "ECOMMERCE" => Ok(ActivityActivityTypeEnum::ECOMMERCE),
           "EVENT" => Ok(ActivityActivityTypeEnum::EVENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityActivityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CohortTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the cohort. The only supported type as of now is `FIRST_VISIT_DATE`. If this field is unspecified the cohort is treated as `FIRST_VISIT_DATE` type cohort.
pub enum CohortTypeEnum {
    

    /// If unspecified it's treated as `FIRST_VISIT_DATE`.
    ///
    /// "UNSPECIFIED_COHORT_TYPE"
    #[serde(rename="UNSPECIFIED_COHORT_TYPE")]
    UNSPECIFIEDCOHORTTYPE,
    

    /// Cohorts that are selected based on first visit date.
    ///
    /// "FIRST_VISIT_DATE"
    #[serde(rename="FIRST_VISIT_DATE")]
    FIRSTVISITDATE,
}

impl AsRef<str> for CohortTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CohortTypeEnum::UNSPECIFIEDCOHORTTYPE => "UNSPECIFIED_COHORT_TYPE",
            CohortTypeEnum::FIRSTVISITDATE => "FIRST_VISIT_DATE",
        }
    }
}

impl std::convert::TryFrom< &str> for CohortTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_COHORT_TYPE" => Ok(CohortTypeEnum::UNSPECIFIEDCOHORTTYPE),
           "FIRST_VISIT_DATE" => Ok(CohortTypeEnum::FIRSTVISITDATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CohortTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionFilterOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How to match the dimension to the expression. The default is REGEXP.
pub enum DimensionFilterOperatorEnum {
    

    /// If the match type is unspecified, it is treated as a `REGEXP`.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The match expression is treated as a regular expression. All match types are not treated as regular expressions.
    ///
    /// "REGEXP"
    #[serde(rename="REGEXP")]
    REGEXP,
    

    /// Matches the value which begin with the match expression provided.
    ///
    /// "BEGINS_WITH"
    #[serde(rename="BEGINS_WITH")]
    BEGINSWITH,
    

    /// Matches the values which end with the match expression provided.
    ///
    /// "ENDS_WITH"
    #[serde(rename="ENDS_WITH")]
    ENDSWITH,
    

    /// Substring match.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// The value should match the match expression entirely.
    ///
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    

    /// Integer comparison filters. case sensitivity is ignored for these and the expression is assumed to be a string representing an integer. Failure conditions: - If expression is not a valid int64, the client should expect an error. - Input dimensions that are not valid int64 values will never match the filter.
    ///
    /// "NUMERIC_EQUAL"
    #[serde(rename="NUMERIC_EQUAL")]
    NUMERICEQUAL,
    

    /// Checks if the dimension is numerically greater than the match expression. Read the description for `NUMERIC_EQUALS` for restrictions.
    ///
    /// "NUMERIC_GREATER_THAN"
    #[serde(rename="NUMERIC_GREATER_THAN")]
    NUMERICGREATERTHAN,
    

    /// Checks if the dimension is numerically less than the match expression. Read the description for `NUMERIC_EQUALS` for restrictions.
    ///
    /// "NUMERIC_LESS_THAN"
    #[serde(rename="NUMERIC_LESS_THAN")]
    NUMERICLESSTHAN,
    

    /// This option is used to specify a dimension filter whose expression can take any value from a selected list of values. This helps avoiding evaluating multiple exact match dimension filters which are OR'ed for every single response row. For example: expressions: ["A", "B", "C"] Any response row whose dimension has it is value as A, B or C, matches this DimensionFilter.
    ///
    /// "IN_LIST"
    #[serde(rename="IN_LIST")]
    INLIST,
}

impl AsRef<str> for DimensionFilterOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionFilterOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            DimensionFilterOperatorEnum::REGEXP => "REGEXP",
            DimensionFilterOperatorEnum::BEGINSWITH => "BEGINS_WITH",
            DimensionFilterOperatorEnum::ENDSWITH => "ENDS_WITH",
            DimensionFilterOperatorEnum::PARTIAL => "PARTIAL",
            DimensionFilterOperatorEnum::EXACT => "EXACT",
            DimensionFilterOperatorEnum::NUMERICEQUAL => "NUMERIC_EQUAL",
            DimensionFilterOperatorEnum::NUMERICGREATERTHAN => "NUMERIC_GREATER_THAN",
            DimensionFilterOperatorEnum::NUMERICLESSTHAN => "NUMERIC_LESS_THAN",
            DimensionFilterOperatorEnum::INLIST => "IN_LIST",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionFilterOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(DimensionFilterOperatorEnum::OPERATORUNSPECIFIED),
           "REGEXP" => Ok(DimensionFilterOperatorEnum::REGEXP),
           "BEGINS_WITH" => Ok(DimensionFilterOperatorEnum::BEGINSWITH),
           "ENDS_WITH" => Ok(DimensionFilterOperatorEnum::ENDSWITH),
           "PARTIAL" => Ok(DimensionFilterOperatorEnum::PARTIAL),
           "EXACT" => Ok(DimensionFilterOperatorEnum::EXACT),
           "NUMERIC_EQUAL" => Ok(DimensionFilterOperatorEnum::NUMERICEQUAL),
           "NUMERIC_GREATER_THAN" => Ok(DimensionFilterOperatorEnum::NUMERICGREATERTHAN),
           "NUMERIC_LESS_THAN" => Ok(DimensionFilterOperatorEnum::NUMERICLESSTHAN),
           "IN_LIST" => Ok(DimensionFilterOperatorEnum::INLIST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionFilterOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DimensionFilterClauseOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator for combining multiple dimension filters. If unspecified, it is treated as an `OR`.
pub enum DimensionFilterClauseOperatorEnum {
    

    /// Unspecified operator. It is treated as an `OR`.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The logical `OR` operator.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
    

    /// The logical `AND` operator.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
}

impl AsRef<str> for DimensionFilterClauseOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DimensionFilterClauseOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            DimensionFilterClauseOperatorEnum::OR => "OR",
            DimensionFilterClauseOperatorEnum::AND => "AND",
        }
    }
}

impl std::convert::TryFrom< &str> for DimensionFilterClauseOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(DimensionFilterClauseOperatorEnum::OPERATORUNSPECIFIED),
           "OR" => Ok(DimensionFilterClauseOperatorEnum::OR),
           "AND" => Ok(DimensionFilterClauseOperatorEnum::AND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DimensionFilterClauseOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EcommerceDataActionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Action associated with this e-commerce action.
pub enum EcommerceDataActionTypeEnum {
    

    /// Action type is not known.
    ///
    /// "UNKNOWN"
    #[serde(rename="UNKNOWN")]
    UNKNOWN,
    

    /// Click through of product lists.
    ///
    /// "CLICK"
    #[serde(rename="CLICK")]
    CLICK,
    

    /// Product detail views.
    ///
    /// "DETAILS_VIEW"
    #[serde(rename="DETAILS_VIEW")]
    DETAILSVIEW,
    

    /// Add product(s) to cart.
    ///
    /// "ADD_TO_CART"
    #[serde(rename="ADD_TO_CART")]
    ADDTOCART,
    

    /// Remove product(s) from cart.
    ///
    /// "REMOVE_FROM_CART"
    #[serde(rename="REMOVE_FROM_CART")]
    REMOVEFROMCART,
    

    /// Check out.
    ///
    /// "CHECKOUT"
    #[serde(rename="CHECKOUT")]
    CHECKOUT,
    

    /// Completed purchase.
    ///
    /// "PAYMENT"
    #[serde(rename="PAYMENT")]
    PAYMENT,
    

    /// Refund of purchase.
    ///
    /// "REFUND"
    #[serde(rename="REFUND")]
    REFUND,
    

    /// Checkout options.
    ///
    /// "CHECKOUT_OPTION"
    #[serde(rename="CHECKOUT_OPTION")]
    CHECKOUTOPTION,
}

impl AsRef<str> for EcommerceDataActionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EcommerceDataActionTypeEnum::UNKNOWN => "UNKNOWN",
            EcommerceDataActionTypeEnum::CLICK => "CLICK",
            EcommerceDataActionTypeEnum::DETAILSVIEW => "DETAILS_VIEW",
            EcommerceDataActionTypeEnum::ADDTOCART => "ADD_TO_CART",
            EcommerceDataActionTypeEnum::REMOVEFROMCART => "REMOVE_FROM_CART",
            EcommerceDataActionTypeEnum::CHECKOUT => "CHECKOUT",
            EcommerceDataActionTypeEnum::PAYMENT => "PAYMENT",
            EcommerceDataActionTypeEnum::REFUND => "REFUND",
            EcommerceDataActionTypeEnum::CHECKOUTOPTION => "CHECKOUT_OPTION",
        }
    }
}

impl std::convert::TryFrom< &str> for EcommerceDataActionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN" => Ok(EcommerceDataActionTypeEnum::UNKNOWN),
           "CLICK" => Ok(EcommerceDataActionTypeEnum::CLICK),
           "DETAILS_VIEW" => Ok(EcommerceDataActionTypeEnum::DETAILSVIEW),
           "ADD_TO_CART" => Ok(EcommerceDataActionTypeEnum::ADDTOCART),
           "REMOVE_FROM_CART" => Ok(EcommerceDataActionTypeEnum::REMOVEFROMCART),
           "CHECKOUT" => Ok(EcommerceDataActionTypeEnum::CHECKOUT),
           "PAYMENT" => Ok(EcommerceDataActionTypeEnum::PAYMENT),
           "REFUND" => Ok(EcommerceDataActionTypeEnum::REFUND),
           "CHECKOUT_OPTION" => Ok(EcommerceDataActionTypeEnum::CHECKOUTOPTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EcommerceDataActionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EcommerceDataEcommerceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this e-commerce activity.
pub enum EcommerceDataEcommerceTypeEnum {
    

    /// Used when the e-commerce activity type is unspecified.
    ///
    /// "ECOMMERCE_TYPE_UNSPECIFIED"
    #[serde(rename="ECOMMERCE_TYPE_UNSPECIFIED")]
    ECOMMERCETYPEUNSPECIFIED,
    

    /// Used when activity has classic (non-enhanced) e-commerce information.
    ///
    /// "CLASSIC"
    #[serde(rename="CLASSIC")]
    CLASSIC,
    

    /// Used when activity has enhanced e-commerce information.
    ///
    /// "ENHANCED"
    #[serde(rename="ENHANCED")]
    ENHANCED,
}

impl AsRef<str> for EcommerceDataEcommerceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EcommerceDataEcommerceTypeEnum::ECOMMERCETYPEUNSPECIFIED => "ECOMMERCE_TYPE_UNSPECIFIED",
            EcommerceDataEcommerceTypeEnum::CLASSIC => "CLASSIC",
            EcommerceDataEcommerceTypeEnum::ENHANCED => "ENHANCED",
        }
    }
}

impl std::convert::TryFrom< &str> for EcommerceDataEcommerceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ECOMMERCE_TYPE_UNSPECIFIED" => Ok(EcommerceDataEcommerceTypeEnum::ECOMMERCETYPEUNSPECIFIED),
           "CLASSIC" => Ok(EcommerceDataEcommerceTypeEnum::CLASSIC),
           "ENHANCED" => Ok(EcommerceDataEcommerceTypeEnum::ENHANCED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EcommerceDataEcommerceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricFormattingTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how the metric expression should be formatted, for example `INTEGER`.
pub enum MetricFormattingTypeEnum {
    

    /// Metric type is unspecified.
    ///
    /// "METRIC_TYPE_UNSPECIFIED"
    #[serde(rename="METRIC_TYPE_UNSPECIFIED")]
    METRICTYPEUNSPECIFIED,
    

    /// Integer metric.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// Float metric.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// Currency metric.
    ///
    /// "CURRENCY"
    #[serde(rename="CURRENCY")]
    CURRENCY,
    

    /// Percentage metric.
    ///
    /// "PERCENT"
    #[serde(rename="PERCENT")]
    PERCENT,
    

    /// Time metric in `HH:MM:SS` format.
    ///
    /// "TIME"
    #[serde(rename="TIME")]
    TIME,
}

impl AsRef<str> for MetricFormattingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricFormattingTypeEnum::METRICTYPEUNSPECIFIED => "METRIC_TYPE_UNSPECIFIED",
            MetricFormattingTypeEnum::INTEGER => "INTEGER",
            MetricFormattingTypeEnum::FLOAT => "FLOAT",
            MetricFormattingTypeEnum::CURRENCY => "CURRENCY",
            MetricFormattingTypeEnum::PERCENT => "PERCENT",
            MetricFormattingTypeEnum::TIME => "TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricFormattingTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_TYPE_UNSPECIFIED" => Ok(MetricFormattingTypeEnum::METRICTYPEUNSPECIFIED),
           "INTEGER" => Ok(MetricFormattingTypeEnum::INTEGER),
           "FLOAT" => Ok(MetricFormattingTypeEnum::FLOAT),
           "CURRENCY" => Ok(MetricFormattingTypeEnum::CURRENCY),
           "PERCENT" => Ok(MetricFormattingTypeEnum::PERCENT),
           "TIME" => Ok(MetricFormattingTypeEnum::TIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricFormattingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricFilterOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Is the metric `EQUAL`, `LESS_THAN` or `GREATER_THAN` the comparisonValue, the default is `EQUAL`. If the operator is `IS_MISSING`, checks if the metric is missing and would ignore the comparisonValue.
pub enum MetricFilterOperatorEnum {
    

    /// If the operator is not specified, it is treated as `EQUAL`.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// Should the value of the metric be exactly equal to the comparison value.
    ///
    /// "EQUAL"
    #[serde(rename="EQUAL")]
    EQUAL,
    

    /// Should the value of the metric be less than to the comparison value.
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// Should the value of the metric be greater than to the comparison value.
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// Validates if the metric is missing. Doesn't take comparisonValue into account.
    ///
    /// "IS_MISSING"
    #[serde(rename="IS_MISSING")]
    ISMISSING,
}

impl AsRef<str> for MetricFilterOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricFilterOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            MetricFilterOperatorEnum::EQUAL => "EQUAL",
            MetricFilterOperatorEnum::LESSTHAN => "LESS_THAN",
            MetricFilterOperatorEnum::GREATERTHAN => "GREATER_THAN",
            MetricFilterOperatorEnum::ISMISSING => "IS_MISSING",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricFilterOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(MetricFilterOperatorEnum::OPERATORUNSPECIFIED),
           "EQUAL" => Ok(MetricFilterOperatorEnum::EQUAL),
           "LESS_THAN" => Ok(MetricFilterOperatorEnum::LESSTHAN),
           "GREATER_THAN" => Ok(MetricFilterOperatorEnum::GREATERTHAN),
           "IS_MISSING" => Ok(MetricFilterOperatorEnum::ISMISSING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricFilterOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricFilterClauseOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator for combining multiple metric filters. If unspecified, it is treated as an `OR`.
pub enum MetricFilterClauseOperatorEnum {
    

    /// Unspecified operator. It is treated as an `OR`.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The logical `OR` operator.
    ///
    /// "OR"
    #[serde(rename="OR")]
    OR,
    

    /// The logical `AND` operator.
    ///
    /// "AND"
    #[serde(rename="AND")]
    AND,
}

impl AsRef<str> for MetricFilterClauseOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricFilterClauseOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            MetricFilterClauseOperatorEnum::OR => "OR",
            MetricFilterClauseOperatorEnum::AND => "AND",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricFilterClauseOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(MetricFilterClauseOperatorEnum::OPERATORUNSPECIFIED),
           "OR" => Ok(MetricFilterClauseOperatorEnum::OR),
           "AND" => Ok(MetricFilterClauseOperatorEnum::AND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricFilterClauseOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetricHeaderEntryTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the metric, for example `INTEGER`.
pub enum MetricHeaderEntryTypeEnum {
    

    /// Metric type is unspecified.
    ///
    /// "METRIC_TYPE_UNSPECIFIED"
    #[serde(rename="METRIC_TYPE_UNSPECIFIED")]
    METRICTYPEUNSPECIFIED,
    

    /// Integer metric.
    ///
    /// "INTEGER"
    #[serde(rename="INTEGER")]
    INTEGER,
    

    /// Float metric.
    ///
    /// "FLOAT"
    #[serde(rename="FLOAT")]
    FLOAT,
    

    /// Currency metric.
    ///
    /// "CURRENCY"
    #[serde(rename="CURRENCY")]
    CURRENCY,
    

    /// Percentage metric.
    ///
    /// "PERCENT"
    #[serde(rename="PERCENT")]
    PERCENT,
    

    /// Time metric in `HH:MM:SS` format.
    ///
    /// "TIME"
    #[serde(rename="TIME")]
    TIME,
}

impl AsRef<str> for MetricHeaderEntryTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetricHeaderEntryTypeEnum::METRICTYPEUNSPECIFIED => "METRIC_TYPE_UNSPECIFIED",
            MetricHeaderEntryTypeEnum::INTEGER => "INTEGER",
            MetricHeaderEntryTypeEnum::FLOAT => "FLOAT",
            MetricHeaderEntryTypeEnum::CURRENCY => "CURRENCY",
            MetricHeaderEntryTypeEnum::PERCENT => "PERCENT",
            MetricHeaderEntryTypeEnum::TIME => "TIME",
        }
    }
}

impl std::convert::TryFrom< &str> for MetricHeaderEntryTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "METRIC_TYPE_UNSPECIFIED" => Ok(MetricHeaderEntryTypeEnum::METRICTYPEUNSPECIFIED),
           "INTEGER" => Ok(MetricHeaderEntryTypeEnum::INTEGER),
           "FLOAT" => Ok(MetricHeaderEntryTypeEnum::FLOAT),
           "CURRENCY" => Ok(MetricHeaderEntryTypeEnum::CURRENCY),
           "PERCENT" => Ok(MetricHeaderEntryTypeEnum::PERCENT),
           "TIME" => Ok(MetricHeaderEntryTypeEnum::TIME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetricHeaderEntryTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderByOrderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order type. The default orderType is `VALUE`.
pub enum OrderByOrderTypeEnum {
    

    /// Unspecified order type will be treated as sort based on value.
    ///
    /// "ORDER_TYPE_UNSPECIFIED"
    #[serde(rename="ORDER_TYPE_UNSPECIFIED")]
    ORDERTYPEUNSPECIFIED,
    

    /// The sort order is based on the value of the chosen column; looks only at the first date range.
    ///
    /// "VALUE"
    #[serde(rename="VALUE")]
    VALUE,
    

    /// The sort order is based on the difference of the values of the chosen column between the first two date ranges. Usable only if there are exactly two date ranges.
    ///
    /// "DELTA"
    #[serde(rename="DELTA")]
    DELTA,
    

    /// The sort order is based on weighted value of the chosen column. If column has n/d format, then weighted value of this ratio will be `(n + totals.n)/(d + totals.d)` Usable only for metrics that represent ratios.
    ///
    /// "SMART"
    #[serde(rename="SMART")]
    SMART,
    

    /// Histogram order type is applicable only to dimension columns with non-empty histogram-buckets.
    ///
    /// "HISTOGRAM_BUCKET"
    #[serde(rename="HISTOGRAM_BUCKET")]
    HISTOGRAMBUCKET,
    

    /// If the dimensions are fixed length numbers, ordinary sort would just work fine. `DIMENSION_AS_INTEGER` can be used if the dimensions are variable length numbers.
    ///
    /// "DIMENSION_AS_INTEGER"
    #[serde(rename="DIMENSION_AS_INTEGER")]
    DIMENSIONASINTEGER,
}

impl AsRef<str> for OrderByOrderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderByOrderTypeEnum::ORDERTYPEUNSPECIFIED => "ORDER_TYPE_UNSPECIFIED",
            OrderByOrderTypeEnum::VALUE => "VALUE",
            OrderByOrderTypeEnum::DELTA => "DELTA",
            OrderByOrderTypeEnum::SMART => "SMART",
            OrderByOrderTypeEnum::HISTOGRAMBUCKET => "HISTOGRAM_BUCKET",
            OrderByOrderTypeEnum::DIMENSIONASINTEGER => "DIMENSION_AS_INTEGER",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderByOrderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_TYPE_UNSPECIFIED" => Ok(OrderByOrderTypeEnum::ORDERTYPEUNSPECIFIED),
           "VALUE" => Ok(OrderByOrderTypeEnum::VALUE),
           "DELTA" => Ok(OrderByOrderTypeEnum::DELTA),
           "SMART" => Ok(OrderByOrderTypeEnum::SMART),
           "HISTOGRAM_BUCKET" => Ok(OrderByOrderTypeEnum::HISTOGRAMBUCKET),
           "DIMENSION_AS_INTEGER" => Ok(OrderByOrderTypeEnum::DIMENSIONASINTEGER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderByOrderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region OrderBySortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The sorting order for the field.
pub enum OrderBySortOrderEnum {
    

    /// If the sort order is unspecified, the default is ascending.
    ///
    /// "SORT_ORDER_UNSPECIFIED"
    #[serde(rename="SORT_ORDER_UNSPECIFIED")]
    SORTORDERUNSPECIFIED,
    

    /// Ascending sort. The field will be sorted in an ascending manner.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
    

    /// Descending sort. The field will be sorted in a descending manner.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
}

impl AsRef<str> for OrderBySortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            OrderBySortOrderEnum::SORTORDERUNSPECIFIED => "SORT_ORDER_UNSPECIFIED",
            OrderBySortOrderEnum::ASCENDING => "ASCENDING",
            OrderBySortOrderEnum::DESCENDING => "DESCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for OrderBySortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_ORDER_UNSPECIFIED" => Ok(OrderBySortOrderEnum::SORTORDERUNSPECIFIED),
           "ASCENDING" => Ok(OrderBySortOrderEnum::ASCENDING),
           "DESCENDING" => Ok(OrderBySortOrderEnum::DESCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a OrderBySortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ReportRequestSamplingLevelEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The desired report [sample](https://support.google.com/analytics/answer/2637192) size. If the the `samplingLevel` field is unspecified the `DEFAULT` sampling level is used. Every [ReportRequest](#ReportRequest) within a `batchGet` method must contain the same `samplingLevel` definition. See [developer guide](/analytics/devguides/reporting/core/v4/basics#sampling) for details.
pub enum ReportRequestSamplingLevelEnum {
    

    /// If the `samplingLevel` field is unspecified the `DEFAULT` sampling level is used.
    ///
    /// "SAMPLING_UNSPECIFIED"
    #[serde(rename="SAMPLING_UNSPECIFIED")]
    SAMPLINGUNSPECIFIED,
    

    /// Returns response with a sample size that balances speed and accuracy.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// It returns a fast response with a smaller sampling size.
    ///
    /// "SMALL"
    #[serde(rename="SMALL")]
    SMALL,
    

    /// Returns a more accurate response using a large sampling size. But this may result in response being slower.
    ///
    /// "LARGE"
    #[serde(rename="LARGE")]
    LARGE,
}

impl AsRef<str> for ReportRequestSamplingLevelEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ReportRequestSamplingLevelEnum::SAMPLINGUNSPECIFIED => "SAMPLING_UNSPECIFIED",
            ReportRequestSamplingLevelEnum::DEFAULT => "DEFAULT",
            ReportRequestSamplingLevelEnum::SMALL => "SMALL",
            ReportRequestSamplingLevelEnum::LARGE => "LARGE",
        }
    }
}

impl std::convert::TryFrom< &str> for ReportRequestSamplingLevelEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAMPLING_UNSPECIFIED" => Ok(ReportRequestSamplingLevelEnum::SAMPLINGUNSPECIFIED),
           "DEFAULT" => Ok(ReportRequestSamplingLevelEnum::DEFAULT),
           "SMALL" => Ok(ReportRequestSamplingLevelEnum::SMALL),
           "LARGE" => Ok(ReportRequestSamplingLevelEnum::LARGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ReportRequestSamplingLevelEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SearchUserActivityRequestActivityTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Set of all activity types being requested. Only acvities matching these types will be returned in the response. If empty, all activies will be returned.
pub enum SearchUserActivityRequestActivityTypesEnum {
    

    /// ActivityType will never have this value in the response. Using this type in the request will result in an error.
    ///
    /// "ACTIVITY_TYPE_UNSPECIFIED"
    #[serde(rename="ACTIVITY_TYPE_UNSPECIFIED")]
    ACTIVITYTYPEUNSPECIFIED,
    

    /// Used when the activity resulted out of a visitor viewing a page.
    ///
    /// "PAGEVIEW"
    #[serde(rename="PAGEVIEW")]
    PAGEVIEW,
    

    /// Used when the activity resulted out of a visitor using an application on a mobile device.
    ///
    /// "SCREENVIEW"
    #[serde(rename="SCREENVIEW")]
    SCREENVIEW,
    

    /// Used to denote that a goal type activity.
    ///
    /// "GOAL"
    #[serde(rename="GOAL")]
    GOAL,
    

    /// An e-commerce transaction was performed by the visitor on the page.
    ///
    /// "ECOMMERCE"
    #[serde(rename="ECOMMERCE")]
    ECOMMERCE,
    

    /// Used when the activity is an event.
    ///
    /// "EVENT"
    #[serde(rename="EVENT")]
    EVENT,
}

impl AsRef<str> for SearchUserActivityRequestActivityTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchUserActivityRequestActivityTypesEnum::ACTIVITYTYPEUNSPECIFIED => "ACTIVITY_TYPE_UNSPECIFIED",
            SearchUserActivityRequestActivityTypesEnum::PAGEVIEW => "PAGEVIEW",
            SearchUserActivityRequestActivityTypesEnum::SCREENVIEW => "SCREENVIEW",
            SearchUserActivityRequestActivityTypesEnum::GOAL => "GOAL",
            SearchUserActivityRequestActivityTypesEnum::ECOMMERCE => "ECOMMERCE",
            SearchUserActivityRequestActivityTypesEnum::EVENT => "EVENT",
        }
    }
}

impl std::convert::TryFrom< &str> for SearchUserActivityRequestActivityTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTIVITY_TYPE_UNSPECIFIED" => Ok(SearchUserActivityRequestActivityTypesEnum::ACTIVITYTYPEUNSPECIFIED),
           "PAGEVIEW" => Ok(SearchUserActivityRequestActivityTypesEnum::PAGEVIEW),
           "SCREENVIEW" => Ok(SearchUserActivityRequestActivityTypesEnum::SCREENVIEW),
           "GOAL" => Ok(SearchUserActivityRequestActivityTypesEnum::GOAL),
           "ECOMMERCE" => Ok(SearchUserActivityRequestActivityTypesEnum::ECOMMERCE),
           "EVENT" => Ok(SearchUserActivityRequestActivityTypesEnum::EVENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchUserActivityRequestActivityTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SegmentDimensionFilterOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The operator to use to match the dimension with the expressions.
pub enum SegmentDimensionFilterOperatorEnum {
    

    /// If the match type is unspecified, it is treated as a REGEXP.
    ///
    /// "OPERATOR_UNSPECIFIED"
    #[serde(rename="OPERATOR_UNSPECIFIED")]
    OPERATORUNSPECIFIED,
    

    /// The match expression is treated as a regular expression. All other match types are not treated as regular expressions.
    ///
    /// "REGEXP"
    #[serde(rename="REGEXP")]
    REGEXP,
    

    /// Matches the values which begin with the match expression provided.
    ///
    /// "BEGINS_WITH"
    #[serde(rename="BEGINS_WITH")]
    BEGINSWITH,
    

    /// Matches the values which end with the match expression provided.
    ///
    /// "ENDS_WITH"
    #[serde(rename="ENDS_WITH")]
    ENDSWITH,
    

    /// Substring match.
    ///
    /// "PARTIAL"
    #[serde(rename="PARTIAL")]
    PARTIAL,
    

    /// The value should match the match expression entirely.
    ///
    /// "EXACT"
    #[serde(rename="EXACT")]
    EXACT,
    

    /// This option is used to specify a dimension filter whose expression can take any value from a selected list of values. This helps avoiding evaluating multiple exact match dimension filters which are OR'ed for every single response row. For example: expressions: ["A", "B", "C"] Any response row whose dimension has it is value as A, B or C, matches this DimensionFilter.
    ///
    /// "IN_LIST"
    #[serde(rename="IN_LIST")]
    INLIST,
    

    /// Integer comparison filters. case sensitivity is ignored for these and the expression is assumed to be a string representing an integer. Failure conditions: - if expression is not a valid int64, the client should expect an error. - input dimensions that are not valid int64 values will never match the filter. Checks if the dimension is numerically less than the match expression.
    ///
    /// "NUMERIC_LESS_THAN"
    #[serde(rename="NUMERIC_LESS_THAN")]
    NUMERICLESSTHAN,
    

    /// Checks if the dimension is numerically greater than the match expression.
    ///
    /// "NUMERIC_GREATER_THAN"
    #[serde(rename="NUMERIC_GREATER_THAN")]
    NUMERICGREATERTHAN,
    

    /// Checks if the dimension is numerically between the minimum and maximum of the match expression, boundaries excluded.
    ///
    /// "NUMERIC_BETWEEN"
    #[serde(rename="NUMERIC_BETWEEN")]
    NUMERICBETWEEN,
}

impl AsRef<str> for SegmentDimensionFilterOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SegmentDimensionFilterOperatorEnum::OPERATORUNSPECIFIED => "OPERATOR_UNSPECIFIED",
            SegmentDimensionFilterOperatorEnum::REGEXP => "REGEXP",
            SegmentDimensionFilterOperatorEnum::BEGINSWITH => "BEGINS_WITH",
            SegmentDimensionFilterOperatorEnum::ENDSWITH => "ENDS_WITH",
            SegmentDimensionFilterOperatorEnum::PARTIAL => "PARTIAL",
            SegmentDimensionFilterOperatorEnum::EXACT => "EXACT",
            SegmentDimensionFilterOperatorEnum::INLIST => "IN_LIST",
            SegmentDimensionFilterOperatorEnum::NUMERICLESSTHAN => "NUMERIC_LESS_THAN",
            SegmentDimensionFilterOperatorEnum::NUMERICGREATERTHAN => "NUMERIC_GREATER_THAN",
            SegmentDimensionFilterOperatorEnum::NUMERICBETWEEN => "NUMERIC_BETWEEN",
        }
    }
}

impl std::convert::TryFrom< &str> for SegmentDimensionFilterOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OPERATOR_UNSPECIFIED" => Ok(SegmentDimensionFilterOperatorEnum::OPERATORUNSPECIFIED),
           "REGEXP" => Ok(SegmentDimensionFilterOperatorEnum::REGEXP),
           "BEGINS_WITH" => Ok(SegmentDimensionFilterOperatorEnum::BEGINSWITH),
           "ENDS_WITH" => Ok(SegmentDimensionFilterOperatorEnum::ENDSWITH),
           "PARTIAL" => Ok(SegmentDimensionFilterOperatorEnum::PARTIAL),
           "EXACT" => Ok(SegmentDimensionFilterOperatorEnum::EXACT),
           "IN_LIST" => Ok(SegmentDimensionFilterOperatorEnum::INLIST),
           "NUMERIC_LESS_THAN" => Ok(SegmentDimensionFilterOperatorEnum::NUMERICLESSTHAN),
           "NUMERIC_GREATER_THAN" => Ok(SegmentDimensionFilterOperatorEnum::NUMERICGREATERTHAN),
           "NUMERIC_BETWEEN" => Ok(SegmentDimensionFilterOperatorEnum::NUMERICBETWEEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SegmentDimensionFilterOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SegmentMetricFilterOperatorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies is the operation to perform to compare the metric. The default is `EQUAL`.
pub enum SegmentMetricFilterOperatorEnum {
    

    /// Unspecified operator is treated as `LESS_THAN` operator.
    ///
    /// "UNSPECIFIED_OPERATOR"
    #[serde(rename="UNSPECIFIED_OPERATOR")]
    UNSPECIFIEDOPERATOR,
    

    /// Checks if the metric value is less than comparison value.
    ///
    /// "LESS_THAN"
    #[serde(rename="LESS_THAN")]
    LESSTHAN,
    

    /// Checks if the metric value is greater than comparison value.
    ///
    /// "GREATER_THAN"
    #[serde(rename="GREATER_THAN")]
    GREATERTHAN,
    

    /// Equals operator.
    ///
    /// "EQUAL"
    #[serde(rename="EQUAL")]
    EQUAL,
    

    /// For between operator, both the minimum and maximum are exclusive. We will use `LT` and `GT` for comparison.
    ///
    /// "BETWEEN"
    #[serde(rename="BETWEEN")]
    BETWEEN,
}

impl AsRef<str> for SegmentMetricFilterOperatorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SegmentMetricFilterOperatorEnum::UNSPECIFIEDOPERATOR => "UNSPECIFIED_OPERATOR",
            SegmentMetricFilterOperatorEnum::LESSTHAN => "LESS_THAN",
            SegmentMetricFilterOperatorEnum::GREATERTHAN => "GREATER_THAN",
            SegmentMetricFilterOperatorEnum::EQUAL => "EQUAL",
            SegmentMetricFilterOperatorEnum::BETWEEN => "BETWEEN",
        }
    }
}

impl std::convert::TryFrom< &str> for SegmentMetricFilterOperatorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_OPERATOR" => Ok(SegmentMetricFilterOperatorEnum::UNSPECIFIEDOPERATOR),
           "LESS_THAN" => Ok(SegmentMetricFilterOperatorEnum::LESSTHAN),
           "GREATER_THAN" => Ok(SegmentMetricFilterOperatorEnum::GREATERTHAN),
           "EQUAL" => Ok(SegmentMetricFilterOperatorEnum::EQUAL),
           "BETWEEN" => Ok(SegmentMetricFilterOperatorEnum::BETWEEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SegmentMetricFilterOperatorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SegmentMetricFilterScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Scope for a metric defines the level at which that metric is defined. The specified metric scope must be equal to or greater than its primary scope as defined in the data model. The primary scope is defined by if the segment is selecting users or sessions.
pub enum SegmentMetricFilterScopeEnum {
    

    /// If the scope is unspecified, it defaults to the condition scope, `USER` or `SESSION` depending on if the segment is trying to choose users or sessions.
    ///
    /// "UNSPECIFIED_SCOPE"
    #[serde(rename="UNSPECIFIED_SCOPE")]
    UNSPECIFIEDSCOPE,
    

    /// Product scope.
    ///
    /// "PRODUCT"
    #[serde(rename="PRODUCT")]
    PRODUCT,
    

    /// Hit scope.
    ///
    /// "HIT"
    #[serde(rename="HIT")]
    HIT,
    

    /// Session scope.
    ///
    /// "SESSION"
    #[serde(rename="SESSION")]
    SESSION,
    

    /// User scope.
    ///
    /// "USER"
    #[serde(rename="USER")]
    USER,
}

impl AsRef<str> for SegmentMetricFilterScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SegmentMetricFilterScopeEnum::UNSPECIFIEDSCOPE => "UNSPECIFIED_SCOPE",
            SegmentMetricFilterScopeEnum::PRODUCT => "PRODUCT",
            SegmentMetricFilterScopeEnum::HIT => "HIT",
            SegmentMetricFilterScopeEnum::SESSION => "SESSION",
            SegmentMetricFilterScopeEnum::USER => "USER",
        }
    }
}

impl std::convert::TryFrom< &str> for SegmentMetricFilterScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_SCOPE" => Ok(SegmentMetricFilterScopeEnum::UNSPECIFIEDSCOPE),
           "PRODUCT" => Ok(SegmentMetricFilterScopeEnum::PRODUCT),
           "HIT" => Ok(SegmentMetricFilterScopeEnum::HIT),
           "SESSION" => Ok(SegmentMetricFilterScopeEnum::SESSION),
           "USER" => Ok(SegmentMetricFilterScopeEnum::USER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SegmentMetricFilterScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SegmentSequenceStepMatchTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies if the step immediately precedes or can be any time before the next step.
pub enum SegmentSequenceStepMatchTypeEnum {
    

    /// Unspecified match type is treated as precedes.
    ///
    /// "UNSPECIFIED_MATCH_TYPE"
    #[serde(rename="UNSPECIFIED_MATCH_TYPE")]
    UNSPECIFIEDMATCHTYPE,
    

    /// Operator indicates that the previous step precedes the next step.
    ///
    /// "PRECEDES"
    #[serde(rename="PRECEDES")]
    PRECEDES,
    

    /// Operator indicates that the previous step immediately precedes the next step.
    ///
    /// "IMMEDIATELY_PRECEDES"
    #[serde(rename="IMMEDIATELY_PRECEDES")]
    IMMEDIATELYPRECEDES,
}

impl AsRef<str> for SegmentSequenceStepMatchTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SegmentSequenceStepMatchTypeEnum::UNSPECIFIEDMATCHTYPE => "UNSPECIFIED_MATCH_TYPE",
            SegmentSequenceStepMatchTypeEnum::PRECEDES => "PRECEDES",
            SegmentSequenceStepMatchTypeEnum::IMMEDIATELYPRECEDES => "IMMEDIATELY_PRECEDES",
        }
    }
}

impl std::convert::TryFrom< &str> for SegmentSequenceStepMatchTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_MATCH_TYPE" => Ok(SegmentSequenceStepMatchTypeEnum::UNSPECIFIEDMATCHTYPE),
           "PRECEDES" => Ok(SegmentSequenceStepMatchTypeEnum::PRECEDES),
           "IMMEDIATELY_PRECEDES" => Ok(SegmentSequenceStepMatchTypeEnum::IMMEDIATELYPRECEDES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SegmentSequenceStepMatchTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the user in the request. The field `userId` is associated with this type.
pub enum UserTypeEnum {
    

    /// When the User Id Type is not specified, the default type used will be CLIENT_ID.
    ///
    /// "USER_ID_TYPE_UNSPECIFIED"
    #[serde(rename="USER_ID_TYPE_UNSPECIFIED")]
    USERIDTYPEUNSPECIFIED,
    

    /// A single user, like a signed-in user account, that may interact with content across one or more devices and / or browser instances.
    ///
    /// "USER_ID"
    #[serde(rename="USER_ID")]
    USERID,
    

    /// Analytics assigned client_id.
    ///
    /// "CLIENT_ID"
    #[serde(rename="CLIENT_ID")]
    CLIENTID,
}

impl AsRef<str> for UserTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserTypeEnum::USERIDTYPEUNSPECIFIED => "USER_ID_TYPE_UNSPECIFIED",
            UserTypeEnum::USERID => "USER_ID",
            UserTypeEnum::CLIENTID => "CLIENT_ID",
        }
    }
}

impl std::convert::TryFrom< &str> for UserTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "USER_ID_TYPE_UNSPECIFIED" => Ok(UserTypeEnum::USERIDTYPEUNSPECIFIED),
           "USER_ID" => Ok(UserTypeEnum::USERID),
           "CLIENT_ID" => Ok(UserTypeEnum::CLIENTID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


