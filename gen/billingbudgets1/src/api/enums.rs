use super::*;



// region GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum {
    

    /// Unspecified ownership scope, same as ALL_USERS.
    ///
    /// "OWNERSHIP_SCOPE_UNSPECIFIED"
    #[serde(rename="OWNERSHIP_SCOPE_UNSPECIFIED")]
    OWNERSHIPSCOPEUNSPECIFIED,
    

    /// Both billing account-level users and project-level users have full access to the budget, if the users have the required IAM permissions.
    ///
    /// "ALL_USERS"
    #[serde(rename="ALL_USERS")]
    ALLUSERS,
    

    /// Only billing account-level users have full access to the budget. Project-level users have read-only access, even if they have the required IAM permissions.
    ///
    /// "BILLING_ACCOUNT"
    #[serde(rename="BILLING_ACCOUNT")]
    BILLINGACCOUNT,
}

impl AsRef<str> for GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum::OWNERSHIPSCOPEUNSPECIFIED => "OWNERSHIP_SCOPE_UNSPECIFIED",
            GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum::ALLUSERS => "ALL_USERS",
            GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum::BILLINGACCOUNT => "BILLING_ACCOUNT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OWNERSHIP_SCOPE_UNSPECIFIED" => Ok(GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum::OWNERSHIPSCOPEUNSPECIFIED),
           "ALL_USERS" => Ok(GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum::ALLUSERS),
           "BILLING_ACCOUNT" => Ok(GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum::BILLINGACCOUNT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudBillingBudgetsV1BudgetOwnershipScopeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. Specifies to track usage for recurring calendar period. For example, assume that CalendarPeriod.QUARTER is set. The budget tracks usage from April 1 to June 30, when the current calendar month is April, May, June. After that, it tracks usage from July 1 to September 30 when the current calendar month is July, August, September, so on.
pub enum GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum {
    

    /// Calendar period is unset. This is the default if the budget is for a custom time period (CustomPeriod).
    ///
    /// "CALENDAR_PERIOD_UNSPECIFIED"
    #[serde(rename="CALENDAR_PERIOD_UNSPECIFIED")]
    CALENDARPERIODUNSPECIFIED,
    

    /// A month. Month starts on the first day of each month, such as January 1, February 1, March 1, and so on.
    ///
    /// "MONTH"
    #[serde(rename="MONTH")]
    MONTH,
    

    /// A quarter. Quarters start on dates January 1, April 1, July 1, and October 1 of each year.
    ///
    /// "QUARTER"
    #[serde(rename="QUARTER")]
    QUARTER,
    

    /// A year. Year starts on January 1.
    ///
    /// "YEAR"
    #[serde(rename="YEAR")]
    YEAR,
}

impl AsRef<str> for GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::CALENDARPERIODUNSPECIFIED => "CALENDAR_PERIOD_UNSPECIFIED",
            GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::MONTH => "MONTH",
            GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::QUARTER => "QUARTER",
            GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::YEAR => "YEAR",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CALENDAR_PERIOD_UNSPECIFIED" => Ok(GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::CALENDARPERIODUNSPECIFIED),
           "MONTH" => Ok(GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::MONTH),
           "QUARTER" => Ok(GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::QUARTER),
           "YEAR" => Ok(GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum::YEAR),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudBillingBudgetsV1FilterCalendarPeriodEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`.
pub enum GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum {
    
    /// "CREDIT_TYPES_TREATMENT_UNSPECIFIED"
    #[serde(rename="CREDIT_TYPES_TREATMENT_UNSPECIFIED")]
    CREDITTYPESTREATMENTUNSPECIFIED,
    

    /// All types of credit are subtracted from the gross cost to determine the spend for threshold calculations.
    ///
    /// "INCLUDE_ALL_CREDITS"
    #[serde(rename="INCLUDE_ALL_CREDITS")]
    INCLUDEALLCREDITS,
    

    /// All types of credit are added to the net cost to determine the spend for threshold calculations.
    ///
    /// "EXCLUDE_ALL_CREDITS"
    #[serde(rename="EXCLUDE_ALL_CREDITS")]
    EXCLUDEALLCREDITS,
    

    /// [Credit types](https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type) specified in the credit_types field are subtracted from the gross cost to determine the spend for threshold calculations.
    ///
    /// "INCLUDE_SPECIFIED_CREDITS"
    #[serde(rename="INCLUDE_SPECIFIED_CREDITS")]
    INCLUDESPECIFIEDCREDITS,
}

impl AsRef<str> for GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::CREDITTYPESTREATMENTUNSPECIFIED => "CREDIT_TYPES_TREATMENT_UNSPECIFIED",
            GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::INCLUDEALLCREDITS => "INCLUDE_ALL_CREDITS",
            GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::EXCLUDEALLCREDITS => "EXCLUDE_ALL_CREDITS",
            GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::INCLUDESPECIFIEDCREDITS => "INCLUDE_SPECIFIED_CREDITS",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CREDIT_TYPES_TREATMENT_UNSPECIFIED" => Ok(GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::CREDITTYPESTREATMENTUNSPECIFIED),
           "INCLUDE_ALL_CREDITS" => Ok(GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::INCLUDEALLCREDITS),
           "EXCLUDE_ALL_CREDITS" => Ok(GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::EXCLUDEALLCREDITS),
           "INCLUDE_SPECIFIED_CREDITS" => Ok(GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum::INCLUDESPECIFIEDCREDITS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudBillingBudgetsV1FilterCreditTypesTreatmentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set.
pub enum GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum {
    

    /// Unspecified threshold basis.
    ///
    /// "BASIS_UNSPECIFIED"
    #[serde(rename="BASIS_UNSPECIFIED")]
    BASISUNSPECIFIED,
    

    /// Use current spend as the basis for comparison against the threshold.
    ///
    /// "CURRENT_SPEND"
    #[serde(rename="CURRENT_SPEND")]
    CURRENTSPEND,
    

    /// Use forecasted spend for the period as the basis for comparison against the threshold. FORECASTED_SPEND can only be set when the budget's time period is a Filter.calendar_period. It cannot be set in combination with Filter.custom_period.
    ///
    /// "FORECASTED_SPEND"
    #[serde(rename="FORECASTED_SPEND")]
    FORECASTEDSPEND,
}

impl AsRef<str> for GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum::BASISUNSPECIFIED => "BASIS_UNSPECIFIED",
            GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum::CURRENTSPEND => "CURRENT_SPEND",
            GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum::FORECASTEDSPEND => "FORECASTED_SPEND",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIS_UNSPECIFIED" => Ok(GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum::BASISUNSPECIFIED),
           "CURRENT_SPEND" => Ok(GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum::CURRENTSPEND),
           "FORECASTED_SPEND" => Ok(GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum::FORECASTEDSPEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleCloudBillingBudgetsV1ThresholdRuleSpendBasisEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


