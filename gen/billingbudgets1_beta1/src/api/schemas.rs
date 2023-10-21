use super::*;
/// AllUpdatesRule defines notifications that are sent based on budget spend and thresholds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1AllUpdatesRule {
    /// Optional. When set to true, disables default notifications sent when a threshold is exceeded. Default notifications are sent to those with Billing Account Administrator and Billing Account User IAM roles for the target account.
    #[serde(rename="disableDefaultIamRecipients")]
    
    pub disable_default_iam_recipients: Option<bool>,
    /// Optional. Targets to send notifications to when a threshold is exceeded. This is in addition to default recipients who have billing account IAM roles. The value is the full REST resource name of a monitoring notification channel with the form `projects/{project_id}/notificationChannels/{channel_id}`. A maximum of 5 channels are allowed. See https://cloud.google.com/billing/docs/how-to/budgets-notification-recipients for more details.
    #[serde(rename="monitoringNotificationChannels")]
    
    pub monitoring_notification_channels: Option<Vec<String>>,
    /// Optional. The name of the Pub/Sub topic where budget related messages will be published, in the form `projects/{project_id}/topics/{topic_id}`. Updates are sent at regular intervals to the topic. The topic needs to be created before the budget is created; see https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications for more details. Caller is expected to have `pubsub.topics.setIamPolicy` permission on the topic when it's set for a budget, otherwise, the API call will fail with PERMISSION_DENIED. See https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#permissions_required_for_this_task for more details on Pub/Sub roles and permissions.
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
    /// Optional. Required when AllUpdatesRule.pubsub_topic is set. The schema version of the notification sent to AllUpdatesRule.pubsub_topic. Only "1.0" is accepted. It represents the JSON schema as defined in https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format.
    #[serde(rename="schemaVersion")]
    
    pub schema_version: Option<String>,
}

impl client::Part for GoogleCloudBillingBudgetsV1beta1AllUpdatesRule {}


/// A budget is a plan that describes what you expect to spend on Cloud projects, plus the rules to execute as spend is tracked against that plan, (for example, send an alert when 90% of the target spend is met). The budget time period is configurable, with options such as month (default), quarter, year, or custom time period.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [budgets create billing accounts](BillingAccountBudgetCreateCall) (response)
/// * [budgets get billing accounts](BillingAccountBudgetGetCall) (response)
/// * [budgets patch billing accounts](BillingAccountBudgetPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1Budget {
    /// Optional. Rules to apply to notifications sent based on budget spend and thresholds.
    #[serde(rename="allUpdatesRule")]
    
    pub all_updates_rule: Option<GoogleCloudBillingBudgetsV1beta1AllUpdatesRule>,
    /// Required. Budgeted amount.
    
    pub amount: Option<GoogleCloudBillingBudgetsV1beta1BudgetAmount>,
    /// Optional. Filters that define which resources are used to compute the actual spend against the budget amount, such as projects, services, and the budget's time period, as well as other filters.
    #[serde(rename="budgetFilter")]
    
    pub budget_filter: Option<GoogleCloudBillingBudgetsV1beta1Filter>,
    /// User data for display name in UI. Validation: <= 60 chars.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag will cause an update to overwrite other changes.
    
    pub etag: Option<String>,
    /// Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    
    pub name: Option<String>,
    /// Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget. Optional for `pubsubTopic` notifications. Required if using email notifications.
    #[serde(rename="thresholdRules")]
    
    pub threshold_rules: Option<Vec<GoogleCloudBillingBudgetsV1beta1ThresholdRule>>,
}

impl client::ResponseResult for GoogleCloudBillingBudgetsV1beta1Budget {}


/// The budgeted amount for each usage period.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1BudgetAmount {
    /// Use the last period's actual spend as the budget for the present period. LastPeriodAmount can only be set when the budget's time period is a Filter.calendar_period. It cannot be set in combination with Filter.custom_period.
    #[serde(rename="lastPeriodAmount")]
    
    pub last_period_amount: Option<GoogleCloudBillingBudgetsV1beta1LastPeriodAmount>,
    /// A specified amount to use as the budget. `currency_code` is optional. If specified when creating a budget, it must match the currency of the billing account. If specified when updating a budget, it must match the currency_code of the existing budget. The `currency_code` is provided on output.
    #[serde(rename="specifiedAmount")]
    
    pub specified_amount: Option<GoogleTypeMoney>,
}

impl client::Part for GoogleCloudBillingBudgetsV1beta1BudgetAmount {}


/// Request for CreateBudget
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [budgets create billing accounts](BillingAccountBudgetCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1CreateBudgetRequest {
    /// Required. Budget to create.
    
    pub budget: Option<GoogleCloudBillingBudgetsV1beta1Budget>,
}

impl client::RequestValue for GoogleCloudBillingBudgetsV1beta1CreateBudgetRequest {}


/// All date times begin at 12 AM US and Canadian Pacific Time (UTC-8).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1CustomPeriod {
    /// Optional. The end date of the time period. Budgets with elapsed end date won't be processed. If unset, specifies to track all usage incurred since the start_date.
    #[serde(rename="endDate")]
    
    pub end_date: Option<GoogleTypeDate>,
    /// Required. The start date must be after January 1, 2017.
    #[serde(rename="startDate")]
    
    pub start_date: Option<GoogleTypeDate>,
}

impl client::Part for GoogleCloudBillingBudgetsV1beta1CustomPeriod {}


/// A filter for a budget, limiting the scope of the cost to calculate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1Filter {
    /// Optional. Specifies to track usage for recurring calendar period. For example, assume that CalendarPeriod.QUARTER is set. The budget will track usage from April 1 to June 30, when the current calendar month is April, May, June. After that, it will track usage from July 1 to September 30 when the current calendar month is July, August, September, so on.
    #[serde(rename="calendarPeriod")]
    
    pub calendar_period: Option<String>,
    /// Optional. If Filter.credit_types_treatment is INCLUDE_SPECIFIED_CREDITS, this is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. See [a list of acceptable credit type values](https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type). If Filter.credit_types_treatment is **not** INCLUDE_SPECIFIED_CREDITS, this field must be empty.
    #[serde(rename="creditTypes")]
    
    pub credit_types: Option<Vec<String>>,
    /// Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`.
    #[serde(rename="creditTypesTreatment")]
    
    pub credit_types_treatment: Option<String>,
    /// Optional. Specifies to track usage from any start date (required) to any end date (optional). This time period is static, it does not recur.
    #[serde(rename="customPeriod")]
    
    pub custom_period: Option<GoogleCloudBillingBudgetsV1beta1CustomPeriod>,
    /// Optional. A single label and value pair specifying that usage from only this set of labeled resources should be included in the budget. If omitted, the report will include all labeled and unlabeled usage. An object containing a single `"key": value` pair. Example: `{ "name": "wrench" }`. _Currently, multiple entries or multiple values per entry are not allowed._
    
    pub labels: Option<HashMap<String, Vec<json::Value>>>,
    /// Optional. A set of projects of the form `projects/{project}`, specifying that usage from only this set of projects should be included in the budget. If omitted, the report will include all usage for the billing account, regardless of which project the usage occurred on.
    
    pub projects: Option<Vec<String>>,
    /// Optional. A set of services of the form `services/{service_id}`, specifying that usage from only this set of services should be included in the budget. If omitted, the report will include usage for all the services. The service names are available through the Catalog API: https://cloud.google.com/billing/v1/how-tos/catalog-api.
    
    pub services: Option<Vec<String>>,
    /// Optional. A set of subaccounts of the form `billingAccounts/{account_id}`, specifying that usage from only this set of subaccounts should be included in the budget. If a subaccount is set to the name of the parent account, usage from the parent account will be included. If omitted, the report will include usage from the parent account and all subaccounts, if they exist.
    
    pub subaccounts: Option<Vec<String>>,
}

impl client::Part for GoogleCloudBillingBudgetsV1beta1Filter {}


/// Describes a budget amount targeted to the last Filter.calendar_period spend. At this time, the amount is automatically 100% of the last calendar period's spend; that is, there are no other options yet. Future configuration options will be described here (for example, configuring a percentage of last period's spend). LastPeriodAmount cannot be set for a budget configured with a Filter.custom_period.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1LastPeriodAmount { _never_set: Option<bool> }

impl client::Part for GoogleCloudBillingBudgetsV1beta1LastPeriodAmount {}


/// Response for ListBudgets
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [budgets list billing accounts](BillingAccountBudgetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1ListBudgetsResponse {
    /// List of the budgets owned by the requested billing account.
    
    pub budgets: Option<Vec<GoogleCloudBillingBudgetsV1beta1Budget>>,
    /// If not empty, indicates that there may be more budgets that match the request; this value should be passed in a new `ListBudgetsRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudBillingBudgetsV1beta1ListBudgetsResponse {}


/// ThresholdRule contains the definition of a threshold. Threshold rules define the triggering events used to generate a budget notification email. When a threshold is crossed (spend exceeds the specified percentages of the budget), budget alert emails are sent to the email recipients you specify in the [NotificationsRule](#notificationsrule). Threshold rules also affect the fields included in the [JSON data object](https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format) sent to a Pub/Sub topic. Threshold rules are *required* if using email notifications. Threshold rules are *optional* if only setting a [`pubsubTopic` NotificationsRule](#NotificationsRule), unless you want your JSON data object to include data about the thresholds you set. For more information, see [set budget threshold rules and actions](https://cloud.google.com/billing/docs/how-to/budgets#budget-actions).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1ThresholdRule {
    /// Optional. The type of basis used to determine if spend has passed the threshold. Behavior defaults to CURRENT_SPEND if not set.
    #[serde(rename="spendBasis")]
    
    pub spend_basis: Option<String>,
    /// Required. Send an alert when this threshold is exceeded. This is a 1.0-based percentage, so 0.5 = 50%. Validation: non-negative number.
    #[serde(rename="thresholdPercent")]
    
    pub threshold_percent: Option<f64>,
}

impl client::Part for GoogleCloudBillingBudgetsV1beta1ThresholdRule {}


/// Request for UpdateBudget
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [budgets patch billing accounts](BillingAccountBudgetPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequest {
    /// Required. The updated budget object. The budget to update is specified by the budget name in the budget.
    
    pub budget: Option<GoogleCloudBillingBudgetsV1beta1Budget>,
    /// Optional. Indicates which fields in the provided budget to update. Read-only fields (such as `name`) cannot be changed. If this is not provided, then only fields with non-default values from the request are updated. See https://developers.google.com/protocol-buffers/docs/proto3#default for more details about default values.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudBillingBudgetsV1beta1UpdateBudgetRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [budgets delete billing accounts](BillingAccountBudgetDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeMoney {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for GoogleTypeMoney {}


