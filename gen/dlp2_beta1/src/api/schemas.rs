use super::*;
/// The transformation to apply to the field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FieldTransformation {
    /// Only apply the transformation if the condition evaluates to true for the
    /// given `RecordCondition`. The conditions are allowed to reference fields
    /// that are not used in the actual transformation. [optional]
    /// 
    /// Example Use Cases:
    /// 
    /// - Apply a different bucket transformation to an age column if the zip code
    /// column for the same record is within a specific range.
    /// - Redact a field if the date of birth field is greater than 85.
    
    pub condition: Option<GooglePrivacyDlpV2beta1RecordCondition>,
    /// Treat the contents of the field as free text, and selectively
    /// transform content that matches an `InfoType`.
    #[serde(rename="infoTypeTransformations")]
    
    pub info_type_transformations: Option<GooglePrivacyDlpV2beta1InfoTypeTransformations>,
    /// Input field(s) to apply the transformation to. [required]
    
    pub fields: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
    /// Apply the transformation to the entire field.
    #[serde(rename="primitiveTransformation")]
    
    pub primitive_transformation: Option<GooglePrivacyDlpV2beta1PrimitiveTransformation>,
}

impl client::Part for GooglePrivacyDlpV2beta1FieldTransformation {}


/// Cloud repository for storing output.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1OutputStorageConfig {
    /// The path to a Google Cloud Storage location to store output.
    /// The bucket must already exist and
    /// the Google APIs service account for DLP must have write permission to
    /// write to the given bucket.
    /// Results are split over multiple csv files with each file name matching
    /// the pattern "[operation_id]_[count].csv", for example
    /// `3094877188788974909_1.csv`. The `operation_id` matches the
    /// identifier for the Operation, and the `count` is a counter used for
    /// tracking the number of files written.
    /// 
    /// The CSV file(s) contain the following columns regardless of storage type
    /// scanned:
    /// - id
    /// - info_type
    /// - likelihood
    /// - byte size of finding
    /// - quote
    /// - timestamp
    /// 
    /// For Cloud Storage the next columns are:
    /// 
    /// - file_path
    /// - start_offset
    /// 
    /// For Cloud Datastore the next columns are:
    /// 
    /// - project_id
    /// - namespace_id
    /// - path
    /// - column_name
    /// - offset
    /// 
    /// For BigQuery the next columns are:
    /// 
    /// - row_number
    /// - project_id
    /// - dataset_id
    /// - table_id
    #[serde(rename="storagePath")]
    
    pub storage_path: Option<GooglePrivacyDlpV2beta1CloudStoragePath>,
    /// Store findings in a new table in the dataset.
    
    pub table: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2beta1OutputStorageConfig {}


/// Generalization function that buckets values based on ranges. The ranges and
/// replacement values are dynamically provided by the user for custom behavior,
/// such as 1-30 -> LOW 31-65 -> MEDIUM 66-100 -> HIGH
/// This can be used on
/// data of type: number, long, string, timestamp.
/// If the bound `Value` type differs from the type of data being transformed, we
/// will first attempt converting the type of the data to be transformed to match
/// the type of the bound before comparing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1BucketingConfig {
    /// no description provided
    
    pub buckets: Option<Vec<GooglePrivacyDlpV2beta1Bucket>>,
}

impl client::Part for GooglePrivacyDlpV2beta1BucketingConfig {}


/// Results of de-identifying a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify content](ContentDeidentifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentifyContentResponse {
    /// no description provided
    
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// A review of the transformations that took place for each item.
    
    pub summaries: Option<Vec<GooglePrivacyDlpV2beta1DeidentificationSummary>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2beta1DeidentifyContentResponse {}


/// This resource represents a long-running operation that is the result of a
/// network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations get risk analysis](RiskAnalysiOperationGetCall) (response)
/// * [analyze data source](DataSourceAnalyzeCall) (response)
/// * [operations create inspect](InspectOperationCreateCall) (response)
/// * [operations get inspect](InspectOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    
    pub done: Option<bool>,
    /// This field will contain an InspectOperationResult object for `inspect.operations.create` or a RiskAnalysisOperationResult object for `dataSource.analyze`.
    
    pub response: Option<HashMap<String, json::Value>>,
    /// The server-assigned name. The `name` should have the format of `inspect/operations/<identifier>`.
    
    pub name: Option<String>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// This field will contain an InspectOperationMetadata object for `inspect.operations.create` or a RiskAnalysisOperationMetadata object for `dataSource.analyze`.  This will always be returned with the Operation.
    
    pub metadata: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// Include to use an existing data crypto key wrapped by KMS.
/// Authorization requires the following IAM permissions when sending a request
/// to perform a crypto transformation using a kms-wrapped crypto key:
/// dlp.kms.encrypt
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KmsWrappedCryptoKey {
    /// The wrapped data crypto key. [required]
    #[serde(rename="wrappedKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub wrapped_key: Option<Vec<u8>>,
    /// The resource name of the KMS CryptoKey to use for unwrapping. [required]
    #[serde(rename="cryptoKeyName")]
    
    pub crypto_key_name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1KmsWrappedCryptoKey {}


/// Type of information detected by the API.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoType {
    /// Name of the information type.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1InfoType {}


/// Characters to skip when doing deidentification of a value. These will be left
/// alone and skipped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CharsToIgnore {
    /// no description provided
    #[serde(rename="charactersToSkip")]
    
    pub characters_to_skip: Option<String>,
    /// no description provided
    #[serde(rename="commonCharactersToIgnore")]
    
    pub common_characters_to_ignore: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1CharsToIgnore {}


/// Buckets values based on fixed size ranges. The
/// Bucketing transformation can provide all of this functionality,
/// but requires more configuration. This message is provided as a convenience to
/// the user for simple bucketing strategies.
/// The resulting value will be a hyphenated string of
/// lower_bound-upper_bound.
/// This can be used on data of type: double, long.
/// If the bound Value type differs from the type of data
/// being transformed, we will first attempt converting the type of the data to
/// be transformed to match the type of the bound before comparing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FixedSizeBucketingConfig {
    /// Upper bound value of buckets. All values greater than upper_bound are
    /// grouped together into a single bucket; for example if `upper_bound` = 89,
    /// then all values greater than 89 are replaced with the value “89+”.
    /// [Required].
    #[serde(rename="upperBound")]
    
    pub upper_bound: Option<GooglePrivacyDlpV2beta1Value>,
    /// Lower bound value of buckets. All values less than `lower_bound` are
    /// grouped together into a single bucket; for example if `lower_bound` = 10,
    /// then all values less than 10 are replaced with the value “-10”. [Required].
    #[serde(rename="lowerBound")]
    
    pub lower_bound: Option<GooglePrivacyDlpV2beta1Value>,
    /// Size of each bucket (except for minimum and maximum buckets). So if
    /// `lower_bound` = 10, `upper_bound` = 89, and `bucket_size` = 10, then the
    /// following buckets would be used: -10, 10-20, 20-30, 30-40, 40-50, 50-60,
    /// 60-70, 70-80, 80-89, 89+. Precision up to 2 decimals works. [Required].
    #[serde(rename="bucketSize")]
    
    pub bucket_size: Option<f64>,
}

impl client::Part for GooglePrivacyDlpV2beta1FixedSizeBucketingConfig {}


/// Info Type Category description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CategoryDescription {
    /// Internal name of the category.
    
    pub name: Option<String>,
    /// Human readable form of the category name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1CategoryDescription {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Conditions {
    /// no description provided
    
    pub conditions: Option<Vec<GooglePrivacyDlpV2beta1Condition>>,
}

impl client::Part for GooglePrivacyDlpV2beta1Conditions {}


/// A rule for transforming a value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PrimitiveTransformation {
    /// no description provided
    #[serde(rename="replaceWithInfoTypeConfig")]
    
    pub replace_with_info_type_config: Option<GooglePrivacyDlpV2beta1ReplaceWithInfoTypeConfig>,
    /// no description provided
    #[serde(rename="cryptoHashConfig")]
    
    pub crypto_hash_config: Option<GooglePrivacyDlpV2beta1CryptoHashConfig>,
    /// no description provided
    #[serde(rename="cryptoReplaceFfxFpeConfig")]
    
    pub crypto_replace_ffx_fpe_config: Option<GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfig>,
    /// no description provided
    #[serde(rename="replaceConfig")]
    
    pub replace_config: Option<GooglePrivacyDlpV2beta1ReplaceValueConfig>,
    /// no description provided
    #[serde(rename="timePartConfig")]
    
    pub time_part_config: Option<GooglePrivacyDlpV2beta1TimePartConfig>,
    /// no description provided
    #[serde(rename="fixedSizeBucketingConfig")]
    
    pub fixed_size_bucketing_config: Option<GooglePrivacyDlpV2beta1FixedSizeBucketingConfig>,
    /// no description provided
    #[serde(rename="characterMaskConfig")]
    
    pub character_mask_config: Option<GooglePrivacyDlpV2beta1CharacterMaskConfig>,
    /// no description provided
    #[serde(rename="bucketingConfig")]
    
    pub bucketing_config: Option<GooglePrivacyDlpV2beta1BucketingConfig>,
    /// no description provided
    #[serde(rename="redactConfig")]
    
    pub redact_config: Option<GooglePrivacyDlpV2beta1RedactConfig>,
}

impl client::Part for GooglePrivacyDlpV2beta1PrimitiveTransformation {}


/// Compute numerical stats over an individual column, including
/// number of distinct values and value count distribution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CategoricalStatsConfig {
    /// Field to compute categorical stats on. All column types are
    /// supported except for arrays and structs. However, it may be more
    /// informative to use NumericalStats when the field type is supported,
    /// depending on the data.
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl client::Part for GooglePrivacyDlpV2beta1CategoricalStatsConfig {}


/// Container structure describing a single finding within a string or image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Finding {
    /// The specific string that may be potentially sensitive info.
    
    pub quote: Option<String>,
    /// Location of the info found.
    
    pub location: Option<GooglePrivacyDlpV2beta1Location>,
    /// Estimate of how likely it is that the info_type is correct.
    
    pub likelihood: Option<String>,
    /// The specific type of info the string might be.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// Timestamp when finding was detected.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GooglePrivacyDlpV2beta1Finding {}


/// Reidentifiability metric. This corresponds to a risk model similar to what
/// is called "journalist risk" in the literature, except the attack dataset is
/// statistically modeled instead of being perfectly known. This can be done
/// using publicly available data (like the US Census), or using a custom
/// statistical model (indicated as one or several BigQuery tables), or by
/// extrapolating from the distribution of values in the input dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KMapEstimationConfig {
    /// Several auxiliary tables can be used in the analysis. Each custom_tag
    /// used to tag a quasi-identifiers column must appear in exactly one column
    /// of one auxiliary table.
    #[serde(rename="auxiliaryTables")]
    
    pub auxiliary_tables: Option<Vec<GooglePrivacyDlpV2beta1AuxiliaryTable>>,
    /// Fields considered to be quasi-identifiers. No two columns can have the
    /// same tag. [required]
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1TaggedField>>,
    /// ISO 3166-1 alpha-2 region code to use in the statistical modeling.
    /// Required if no column is tagged with a region-specific InfoType (like
    /// US_ZIP_5) or a region code.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1KMapEstimationConfig {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list risk analysis](RiskAnalysiOperationListCall) (response)
/// * [operations list inspect](InspectOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Row {
    /// no description provided
    
    pub values: Option<Vec<GooglePrivacyDlpV2beta1Value>>,
}

impl client::Part for GooglePrivacyDlpV2beta1Row {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel risk analysis](RiskAnalysiOperationCancelCall) (response)
/// * [operations delete risk analysis](RiskAnalysiOperationDeleteCall) (response)
/// * [operations cancel inspect](InspectOperationCancelCall) (response)
/// * [operations delete inspect](InspectOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// Response to the ListInspectFindings request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [results findings list inspect](InspectResultFindingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ListInspectFindingsResponse {
    /// The results.
    
    pub result: Option<GooglePrivacyDlpV2beta1InspectResult>,
    /// If not empty, indicates that there may be more results that match the
    /// request; this value should be passed in a new `ListInspectFindingsRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GooglePrivacyDlpV2beta1ListInspectFindingsResponse {}


/// A collection of expressions
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Expressions {
    /// The operator to apply to the result of conditions. Default and currently
    /// only supported value is `AND`.
    #[serde(rename="logicalOperator")]
    
    pub logical_operator: Option<String>,
    /// no description provided
    
    pub conditions: Option<GooglePrivacyDlpV2beta1Conditions>,
}

impl client::Part for GooglePrivacyDlpV2beta1Expressions {}


/// Options defining a file or a set of files (path ending with *) within
/// a Google Cloud Storage bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CloudStorageOptions {
    /// no description provided
    #[serde(rename="fileSet")]
    
    pub file_set: Option<GooglePrivacyDlpV2beta1FileSet>,
}

impl client::Part for GooglePrivacyDlpV2beta1CloudStorageOptions {}


/// A location in Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CloudStoragePath {
    /// The url, in the format of `gs://bucket/<path>`.
    
    pub path: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1CloudStoragePath {}


/// Generic half-open interval [start, end)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Range {
    /// Index of the first character of the range (inclusive).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start: Option<i64>,
    /// Index of the last character of the range (exclusive).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2beta1Range {}


/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeOfDay {
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
}

impl client::Part for GoogleTypeTimeOfDay {}


/// The configuration that controls how the data will change.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentifyConfig {
    /// Treat the dataset as structured. Transformations can be applied to
    /// specific locations within structured datasets, such as transforming
    /// a column within a table.
    #[serde(rename="recordTransformations")]
    
    pub record_transformations: Option<GooglePrivacyDlpV2beta1RecordTransformations>,
    /// Treat the dataset as free-form text and apply the same free text
    /// transformation everywhere.
    #[serde(rename="infoTypeTransformations")]
    
    pub info_type_transformations: Option<GooglePrivacyDlpV2beta1InfoTypeTransformations>,
}

impl client::Part for GooglePrivacyDlpV2beta1DeidentifyConfig {}


/// l-diversity metric, used for analysis of reidentification risk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1LDiversityConfig {
    /// Sensitive field for computing the l-value.
    #[serde(rename="sensitiveAttribute")]
    
    pub sensitive_attribute: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// Set of quasi-identifiers indicating how equivalence classes are
    /// defined for the l-diversity computation. When multiple fields are
    /// specified, they are considered a single composite key.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
}

impl client::Part for GooglePrivacyDlpV2beta1LDiversityConfig {}


/// Shared message indicating Cloud storage type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1StorageConfig {
    /// BigQuery options specification.
    #[serde(rename="bigQueryOptions")]
    
    pub big_query_options: Option<GooglePrivacyDlpV2beta1BigQueryOptions>,
    /// Google Cloud Storage options specification.
    #[serde(rename="cloudStorageOptions")]
    
    pub cloud_storage_options: Option<GooglePrivacyDlpV2beta1CloudStorageOptions>,
    /// Google Cloud Datastore options specification.
    #[serde(rename="datastoreOptions")]
    
    pub datastore_options: Option<GooglePrivacyDlpV2beta1DatastoreOptions>,
}

impl client::Part for GooglePrivacyDlpV2beta1StorageConfig {}


/// Additional configuration for inspect long running operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1OperationConfig {
    /// Max number of findings per file, Datastore entity, or database row.
    #[serde(rename="maxItemFindings")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_item_findings: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2beta1OperationConfig {}


/// Options defining BigQuery table and row identifiers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1BigQueryOptions {
    /// References to fields uniquely identifying rows within the table.
    /// Nested fields in the format, like `person.birthdate.year`, are allowed.
    #[serde(rename="identifyingFields")]
    
    pub identifying_fields: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
    /// Complete BigQuery table reference.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
}

impl client::Part for GooglePrivacyDlpV2beta1BigQueryOptions {}


/// Privacy metric to compute for reidentification risk analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PrivacyMetric {
    /// no description provided
    #[serde(rename="kMapEstimationConfig")]
    
    pub k_map_estimation_config: Option<GooglePrivacyDlpV2beta1KMapEstimationConfig>,
    /// no description provided
    #[serde(rename="lDiversityConfig")]
    
    pub l_diversity_config: Option<GooglePrivacyDlpV2beta1LDiversityConfig>,
    /// no description provided
    #[serde(rename="numericalStatsConfig")]
    
    pub numerical_stats_config: Option<GooglePrivacyDlpV2beta1NumericalStatsConfig>,
    /// no description provided
    #[serde(rename="kAnonymityConfig")]
    
    pub k_anonymity_config: Option<GooglePrivacyDlpV2beta1KAnonymityConfig>,
    /// no description provided
    #[serde(rename="categoricalStatsConfig")]
    
    pub categorical_stats_config: Option<GooglePrivacyDlpV2beta1CategoricalStatsConfig>,
}

impl client::Part for GooglePrivacyDlpV2beta1PrivacyMetric {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ReplaceConfig {
    /// Content replacing sensitive information of given type. Max 256 chars.
    #[serde(rename="replaceWith")]
    
    pub replace_with: Option<String>,
    /// Type of information to replace. Only one ReplaceConfig per info_type
    /// should be provided. If ReplaceConfig does not have an info_type, the DLP
    /// API matches it against all info_types that are found but not specified in
    /// another ReplaceConfig.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
}

impl client::Part for GooglePrivacyDlpV2beta1ReplaceConfig {}


/// Compute numerical stats over an individual column, including
/// min, max, and quantiles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1NumericalStatsConfig {
    /// Field to compute numerical stats on. Supported types are
    /// integer, float, date, datetime, timestamp, time.
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl client::Part for GooglePrivacyDlpV2beta1NumericalStatsConfig {}


/// High level summary of deidentification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentificationSummary {
    /// Total size in bytes that were transformed in some way.
    #[serde(rename="transformedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transformed_bytes: Option<i64>,
    /// Transformations applied to the dataset.
    #[serde(rename="transformationSummaries")]
    
    pub transformation_summaries: Option<Vec<GooglePrivacyDlpV2beta1TransformationSummary>>,
}

impl client::Part for GooglePrivacyDlpV2beta1DeidentificationSummary {}


/// A condition for determining whether a transformation should be applied to
/// a field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordCondition {
    /// no description provided
    
    pub expressions: Option<GooglePrivacyDlpV2beta1Expressions>,
}

impl client::Part for GooglePrivacyDlpV2beta1RecordCondition {}


/// For use with `Date`, `Timestamp`, and `TimeOfDay`, extract or preserve a
/// portion of the value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TimePartConfig {
    /// no description provided
    #[serde(rename="partToExtract")]
    
    pub part_to_extract: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1TimePartConfig {}


/// Response to the ListInfoTypes request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [info types list root categories](RootCategoryInfoTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ListInfoTypesResponse {
    /// Set of sensitive info types belonging to a category.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2beta1InfoTypeDescription>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2beta1ListInfoTypesResponse {}


/// Record key for a finding in a Cloud Storage file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CloudStorageKey {
    /// Path to the file.
    #[serde(rename="filePath")]
    
    pub file_path: Option<String>,
    /// Byte offset of the referenced data in the file.
    #[serde(rename="startOffset")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_offset: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2beta1CloudStorageKey {}


/// Custom information type based on a dictionary of words or phrases. This can
/// be used to match sensitive information specific to the data, such as a list
/// of employee IDs or job titles.
/// 
/// Dictionary words are case-insensitive and all characters other than letters
/// and digits in the unicode [Basic Multilingual
/// Plane](https://en.wikipedia.org/wiki/Plane_%28Unicode%29#Basic_Multilingual_Plane)
/// will be replaced with whitespace when scanning for matches, so the
/// dictionary phrase "Sam Johnson" will match all three phrases "sam johnson",
/// "Sam, Johnson", and "Sam (Johnson)". Additionally, the characters
/// surrounding any match must be of a different type than the adjacent
/// characters within the word, so letters must be next to non-letters and
/// digits next to non-digits. For example, the dictionary word "jen" will
/// match the first three letters of the text "jen123" but will return no
/// matches for "jennifer".
/// 
/// Dictionary words containing a large number of characters that are not
/// letters or digits may result in unexpected findings because such characters
/// are treated as whitespace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Dictionary {
    /// List of words or phrases to search for.
    #[serde(rename="wordList")]
    
    pub word_list: Option<GooglePrivacyDlpV2beta1WordList>,
}

impl client::Part for GooglePrivacyDlpV2beta1Dictionary {}


/// Options defining a data set within Google Cloud Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DatastoreOptions {
    /// A partition ID identifies a grouping of entities. The grouping is always
    /// by project and namespace, however the namespace ID may be empty.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<GooglePrivacyDlpV2beta1PartitionId>,
    /// The kind to process.
    
    pub kind: Option<GooglePrivacyDlpV2beta1KindExpression>,
    /// Properties to scan. If none are specified, all properties will be scanned
    /// by default.
    
    pub projection: Option<Vec<GooglePrivacyDlpV2beta1Projection>>,
}

impl client::Part for GooglePrivacyDlpV2beta1DatastoreOptions {}


/// A type of transformation that is applied over structured data such as a
/// table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordTransformations {
    /// Transform the record by applying various field transformations.
    #[serde(rename="fieldTransformations")]
    
    pub field_transformations: Option<Vec<GooglePrivacyDlpV2beta1FieldTransformation>>,
    /// Configuration defining which records get suppressed entirely. Records that
    /// match any suppression rule are omitted from the output [optional].
    #[serde(rename="recordSuppressions")]
    
    pub record_suppressions: Option<Vec<GooglePrivacyDlpV2beta1RecordSuppression>>,
}

impl client::Part for GooglePrivacyDlpV2beta1RecordTransformations {}


/// Configuration description of the scanning process.
/// When used with redactContent only info_types and min_likelihood are currently
/// used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectConfig {
    /// Configuration of findings limit given for specified info types.
    #[serde(rename="infoTypeLimits")]
    
    pub info_type_limits: Option<Vec<GooglePrivacyDlpV2beta1InfoTypeLimit>>,
    /// Limits the number of findings per content item or long running operation.
    #[serde(rename="maxFindings")]
    
    pub max_findings: Option<i32>,
    /// Restricts what info_types to look for. The values must correspond to
    /// InfoType values returned by ListInfoTypes or found in documentation.
    /// Empty info_types runs all enabled detectors.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2beta1InfoType>>,
    /// When true, a contextual quote from the data that triggered a finding is
    /// included in the response; see Finding.quote.
    #[serde(rename="includeQuote")]
    
    pub include_quote: Option<bool>,
    /// Custom info types provided by the user.
    #[serde(rename="customInfoTypes")]
    
    pub custom_info_types: Option<Vec<GooglePrivacyDlpV2beta1CustomInfoType>>,
    /// When true, excludes type information of the findings.
    #[serde(rename="excludeTypes")]
    
    pub exclude_types: Option<bool>,
    /// Only returns findings equal or above this threshold.
    #[serde(rename="minLikelihood")]
    
    pub min_likelihood: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1InspectConfig {}


/// A representation of a Datastore property in a projection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Projection {
    /// The property to project.
    
    pub property: Option<GooglePrivacyDlpV2beta1PropertyReference>,
}

impl client::Part for GooglePrivacyDlpV2beta1Projection {}


/// Redact a given value. For example, if used with an `InfoTypeTransformation`
/// transforming PHONE_NUMBER, and input 'My phone number is 206-555-0123', the
/// output would be 'My phone number is '.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RedactConfig { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2beta1RedactConfig {}


/// Pseudonymization method that generates surrogates via cryptographic hashing.
/// Uses SHA-256.
/// Outputs a 32 byte digest as an uppercase hex string
/// (for example, 41D1567F7F99F1DC2A5FAB886DEE5BEE).
/// Currently, only string and integer values can be hashed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CryptoHashConfig {
    /// The key used by the hash function.
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<GooglePrivacyDlpV2beta1CryptoKey>,
}

impl client::Part for GooglePrivacyDlpV2beta1CryptoHashConfig {}


/// A unique identifier for a Datastore entity.
/// If a key's partition ID or any of its path kinds or names are
/// reserved/read-only, the key is reserved/read-only.
/// A reserved/read-only key is forbidden in certain documented contexts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Key {
    /// The entity path.
    /// An entity path consists of one or more elements composed of a kind and a
    /// string or numerical identifier, which identify entities. The first
    /// element identifies a _root entity_, the second element identifies
    /// a _child_ of the root entity, the third element identifies a child of the
    /// second entity, and so forth. The entities identified by all prefixes of
    /// the path are called the element's _ancestors_.
    /// 
    /// A path can never be empty, and a path can have at most 100 elements.
    
    pub path: Option<Vec<GooglePrivacyDlpV2beta1PathElement>>,
    /// Entities are partitioned into subsets, currently identified by a project
    /// ID and namespace ID.
    /// Queries are scoped to a single partition.
    #[serde(rename="partitionId")]
    
    pub partition_id: Option<GooglePrivacyDlpV2beta1PartitionId>,
}

impl client::Part for GooglePrivacyDlpV2beta1Key {}


/// Request to search for potentially sensitive info in a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect content](ContentInspectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectContentRequest {
    /// The list of items to inspect. Items in a single request are
    /// considered "related" unless inspect_config.independent_inputs is true.
    /// Up to 100 are allowed per request.
    
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
}

impl client::RequestValue for GooglePrivacyDlpV2beta1InspectContentRequest {}


/// Represents a whole calendar date, e.g. date of birth. The time of day and
/// time zone are either specified elsewhere or are not significant. The date
/// is relative to the Proleptic Gregorian Calendar. The day may be 0 to
/// represent a year and month where the day is not significant, e.g. credit card
/// expiration date. The year may be 0 to represent a month and day independent
/// of year, e.g. anniversary date. Related types are google.type.TimeOfDay
/// and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Month of year. Must be from 1 to 12.
    
    pub month: Option<i32>,
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    
    pub year: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year/month where the day is not significant.
    
    pub day: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Configuration for determining how redaction of images should occur.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ImageRedactionConfig {
    /// The color to use when redacting content from an image. If not specified,
    /// the default is black.
    #[serde(rename="redactionColor")]
    
    pub redaction_color: Option<GooglePrivacyDlpV2beta1Color>,
    /// If true, all text found in the image, regardless whether it matches an
    /// info_type, is redacted.
    #[serde(rename="redactAllText")]
    
    pub redact_all_text: Option<bool>,
    /// Only one per info_type should be provided per request. If not
    /// specified, and redact_all_text is false, the DLP API will redact all
    /// text that it matches against all info_types that are found, but not
    /// specified in another ImageRedactionConfig.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
}

impl client::Part for GooglePrivacyDlpV2beta1ImageRedactionConfig {}


/// Replace each matching finding with the name of the info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ReplaceWithInfoTypeConfig { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2beta1ReplaceWithInfoTypeConfig {}


/// Specifies the location of a finding within its source item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Location {
    /// Key of the finding.
    #[serde(rename="recordKey")]
    
    pub record_key: Option<GooglePrivacyDlpV2beta1RecordKey>,
    /// Location within a `ContentItem.Table`.
    #[serde(rename="tableLocation")]
    
    pub table_location: Option<GooglePrivacyDlpV2beta1TableLocation>,
    /// Character offsets within a content item, included when content type
    /// is a text. Default charset assumed to be UTF-8.
    #[serde(rename="codepointRange")]
    
    pub codepoint_range: Option<GooglePrivacyDlpV2beta1Range>,
    /// Field id of the field containing the finding.
    #[serde(rename="fieldId")]
    
    pub field_id: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// Location within an image's pixels.
    #[serde(rename="imageBoxes")]
    
    pub image_boxes: Option<Vec<GooglePrivacyDlpV2beta1ImageLocation>>,
    /// Zero-based byte offsets within a content item.
    #[serde(rename="byteRange")]
    
    pub byte_range: Option<GooglePrivacyDlpV2beta1Range>,
}

impl client::Part for GooglePrivacyDlpV2beta1Location {}


/// Configuration to suppress records whose suppression conditions evaluate to
/// true.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordSuppression {
    /// no description provided
    
    pub condition: Option<GooglePrivacyDlpV2beta1RecordCondition>,
}

impl client::Part for GooglePrivacyDlpV2beta1RecordSuppression {}


/// An entity in a dataset is a field or set of fields that correspond to a
/// single person. For example, in medical records the `EntityId` might be
/// a patient identifier, or for financial records it might be an account
/// identifier. This message is used when generalizations or analysis must be
/// consistent across multiple rows pertaining to the same entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1EntityId {
    /// Composite key indicating which field contains the entity identifier.
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl client::Part for GooglePrivacyDlpV2beta1EntityId {}


/// Using raw keys is prone to security risks due to accidentally
/// leaking the key. Choose another type of key if possible.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1UnwrappedCryptoKey {
    /// The AES 128/192/256 bit key. [required]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub key: Option<Vec<u8>>,
}

impl client::Part for GooglePrivacyDlpV2beta1UnwrappedCryptoKey {}


/// Custom information type provided by the user. Used to find domain-specific
/// sensitive information configurable to the data in question.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CustomInfoType {
    /// Info type configuration. All custom info types must have configurations
    /// that do not conflict with built-in info types or other custom info types.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// Dictionary-based custom info type.
    
    pub dictionary: Option<GooglePrivacyDlpV2beta1Dictionary>,
    /// Surrogate info type.
    #[serde(rename="surrogateType")]
    
    pub surrogate_type: Option<GooglePrivacyDlpV2beta1SurrogateType>,
}

impl client::Part for GooglePrivacyDlpV2beta1CustomInfoType {}


/// Description of the information type (infoType).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeDescription {
    /// Human readable form of the infoType name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// List of categories this infoType belongs to.
    
    pub categories: Option<Vec<GooglePrivacyDlpV2beta1CategoryDescription>>,
    /// Internal name of the infoType.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1InfoTypeDescription {}


/// An auxiliary table contains statistical information on the relative
/// frequency of different quasi-identifiers values. It has one or several
/// quasi-identifiers columns, and one column that indicates the relative
/// frequency of each quasi-identifier tuple.
/// If a tuple is present in the data but not in the auxiliary table, the
/// corresponding relative frequency is assumed to be zero (and thus, the
/// tuple is highly reidentifiable).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1AuxiliaryTable {
    /// Auxiliary table location. [required]
    
    pub table: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
    /// Quasi-identifier columns. [required]
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1QuasiIdField>>,
    /// The relative frequency column must contain a floating-point number
    /// between 0 and 1 (inclusive). Null values are assumed to be zero.
    /// [required]
    #[serde(rename="relativeFrequency")]
    
    pub relative_frequency: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl client::Part for GooglePrivacyDlpV2beta1AuxiliaryTable {}


/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// * Simple to use and understand for most users
/// * Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
/// 
/// # Language mapping
/// 
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
/// 
/// # Other uses
/// 
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
/// 
/// Example uses of this error model include:
/// 
/// * Partial errors. If a service needs to return partial errors to the client,
///   it may embed the `Status` in the normal response to indicate the partial
///   errors.
/// 
/// * Workflow errors. A typical workflow has multiple steps. Each step may
///   have a `Status` message for error reporting.
/// 
/// * Batch operations. If a client uses batch request and batch response, the
///   `Status` message should be used directly inside batch response, one for
///   each error sub-response.
/// 
/// * Asynchronous operations. If an API call embeds asynchronous operation
///   results in its response, the status of those operations should be
///   represented directly using the `Status` message.
/// 
/// * Logging. If some API errors are stored in logs, the message `Status` could
///   be used directly after any stripping needed for security/privacy reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::Part for GoogleRpcStatus {}


/// A (kind, ID/name) pair used to construct a key path.
/// 
/// If either name or ID is set, the element is complete.
/// If neither is set, the element is incomplete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PathElement {
    /// The auto-allocated ID of the entity.
    /// Never equal to zero. Values less than zero are discouraged and may not
    /// be supported in the future.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// The name of the entity.
    /// A name matching regex `__.*__` is reserved/read-only.
    /// A name must not be more than 1500 bytes when UTF-8 encoded.
    /// Cannot be `""`.
    
    pub name: Option<String>,
    /// The kind of the entity.
    /// A kind matching regex `__.*__` is reserved/read-only.
    /// A kind must not contain more than 1500 bytes when UTF-8 encoded.
    /// Cannot be `""`.
    
    pub kind: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1PathElement {}


/// A collection that informs the user the number of times a particular
/// `TransformationResultCode` and error details occurred.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1SummaryResult {
    /// A place for warnings or errors to show up if a transformation didn't
    /// work as expected.
    
    pub details: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// no description provided
    
    pub code: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1SummaryResult {}


/// Message defining the location of a BigQuery table. A table is uniquely
/// identified  by its project_id, dataset_id, and table_name. Within a query
/// a table is often referenced with a string in the format of:
/// `<project_id>:<dataset_id>.<table_id>` or
/// `<project_id>.<dataset_id>.<table_id>`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1BigQueryTable {
    /// Name of the table.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// The Google Cloud Platform project ID of the project containing the table.
    /// If omitted, project ID is inferred from the API call.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Dataset ID of the table.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1BigQueryTable {}


/// Response for ListRootCategories request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list root categories](RootCategoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ListRootCategoriesResponse {
    /// List of all into type categories supported by the API.
    
    pub categories: Option<Vec<GooglePrivacyDlpV2beta1CategoryDescription>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2beta1ListRootCategoriesResponse {}


/// A type of transformation that will scan unstructured text and
/// apply various `PrimitiveTransformation`s to each finding, where the
/// transformation is applied to only values that were identified as a specific
/// info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeTransformations {
    /// Transformation for each info type. Cannot specify more than one
    /// for a given info type. [required]
    
    pub transformations: Option<Vec<GooglePrivacyDlpV2beta1InfoTypeTransformation>>,
}

impl client::Part for GooglePrivacyDlpV2beta1InfoTypeTransformations {}


/// A representation of a Datastore kind.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KindExpression {
    /// The name of the kind.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1KindExpression {}


/// Set of files to scan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FileSet {
    /// The url, in the format `gs://<bucket>/<path>`. Trailing wildcard in the
    /// path is allowed.
    
    pub url: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1FileSet {}


/// A transformation to apply to text that is identified as a specific
/// info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeTransformation {
    /// Primitive transformation to apply to the info type. [required]
    #[serde(rename="primitiveTransformation")]
    
    pub primitive_transformation: Option<GooglePrivacyDlpV2beta1PrimitiveTransformation>,
    /// Info types to apply the transformation to. Empty list will match all
    /// available info types for this transformation.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<GooglePrivacyDlpV2beta1InfoType>>,
}

impl client::Part for GooglePrivacyDlpV2beta1InfoTypeTransformation {}


/// Request for creating a risk analysis operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze data source](DataSourceAnalyzeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest {
    /// Input dataset to compute metrics over.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<GooglePrivacyDlpV2beta1BigQueryTable>,
    /// Privacy metric to compute.
    #[serde(rename="privacyMetric")]
    
    pub privacy_metric: Option<GooglePrivacyDlpV2beta1PrivacyMetric>,
}

impl client::RequestValue for GooglePrivacyDlpV2beta1AnalyzeDataSourceRiskRequest {}


/// Buckets represented as ranges, along with replacement values. Ranges must
/// be non-overlapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Bucket {
    /// Upper bound of the range, exclusive; type must match min.
    
    pub max: Option<GooglePrivacyDlpV2beta1Value>,
    /// Replacement value for this bucket. If not provided
    /// the default behavior will be to hyphenate the min-max range.
    #[serde(rename="replacementValue")]
    
    pub replacement_value: Option<GooglePrivacyDlpV2beta1Value>,
    /// Lower bound of the range, inclusive. Type should be the same as max if
    /// used.
    
    pub min: Option<GooglePrivacyDlpV2beta1Value>,
}

impl client::Part for GooglePrivacyDlpV2beta1Bucket {}


/// Location of a finding within a `ContentItem.Table`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TableLocation {
    /// The zero-based index of the row where the finding is located.
    #[serde(rename="rowIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_index: Option<i64>,
}

impl client::Part for GooglePrivacyDlpV2beta1TableLocation {}


/// k-anonymity metric, used for analysis of reidentification risk.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1KAnonymityConfig {
    /// Optional message indicating that each distinct entity_id should not
    /// contribute to the k-anonymity count more than once per equivalence class.
    /// If an entity_id appears on several rows with different quasi-identifier
    /// tuples, it will contribute to each count exactly once.
    /// 
    /// This can lead to unexpected results. Consider a table where ID 1 is
    /// associated to quasi-identifier "foo", ID 2 to "bar", and ID 3 to *both*
    /// quasi-identifiers "foo" and "bar" (on separate rows), and where this ID
    /// is used as entity_id. Then, the anonymity value associated to ID 3 will
    /// be 2, even if it is the only ID to be associated to both values "foo" and
    /// "bar".
    #[serde(rename="entityId")]
    
    pub entity_id: Option<GooglePrivacyDlpV2beta1EntityId>,
    /// Set of fields to compute k-anonymity over. When multiple fields are
    /// specified, they are considered a single composite key. Structs and
    /// repeated data types are not supported; however, nested fields are
    /// supported so long as they are not structs themselves or nested within
    /// a repeated field.
    #[serde(rename="quasiIds")]
    
    pub quasi_ids: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
}

impl client::Part for GooglePrivacyDlpV2beta1KAnonymityConfig {}


/// Record key for a finding in Cloud Datastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DatastoreKey {
    /// Datastore entity key.
    #[serde(rename="entityKey")]
    
    pub entity_key: Option<GooglePrivacyDlpV2beta1Key>,
}

impl client::Part for GooglePrivacyDlpV2beta1DatastoreKey {}


/// Message for a unique key indicating a record that contains a finding.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RecordKey {
    /// no description provided
    #[serde(rename="cloudStorageKey")]
    
    pub cloud_storage_key: Option<GooglePrivacyDlpV2beta1CloudStorageKey>,
    /// no description provided
    #[serde(rename="datastoreKey")]
    
    pub datastore_key: Option<GooglePrivacyDlpV2beta1DatastoreKey>,
}

impl client::Part for GooglePrivacyDlpV2beta1RecordKey {}


/// Request to de-identify a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [deidentify content](ContentDeidentifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1DeidentifyContentRequest {
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
    /// The list of items to inspect. Up to 100 are allowed per request.
    /// All items will be treated as text/*.
    
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// Configuration for the de-identification of the list of content items.
    #[serde(rename="deidentifyConfig")]
    
    pub deidentify_config: Option<GooglePrivacyDlpV2beta1DeidentifyConfig>,
}

impl client::RequestValue for GooglePrivacyDlpV2beta1DeidentifyContentRequest {}


/// All the findings for a single scanned item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectResult {
    /// If true, then this item might have more findings than were returned,
    /// and the findings returned are an arbitrary subset of all findings.
    /// The findings list might be truncated because the input items were too
    /// large, or because the server reached the maximum amount of resources
    /// allowed for a single API call. For best results, divide the input into
    /// smaller batches.
    #[serde(rename="findingsTruncated")]
    
    pub findings_truncated: Option<bool>,
    /// List of findings for an item.
    
    pub findings: Option<Vec<GooglePrivacyDlpV2beta1Finding>>,
}

impl client::Part for GooglePrivacyDlpV2beta1InspectResult {}


/// A quasi-identifier column has a custom_tag, used to know which column
/// in the data corresponds to which column in the statistical model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1QuasiIdField {
    /// no description provided
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// no description provided
    #[serde(rename="customTag")]
    
    pub custom_tag: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1QuasiIdField {}


/// Bounding box encompassing detected text within an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ImageLocation {
    /// Height of the bounding box in pixels.
    
    pub height: Option<i32>,
    /// Top coordinate of the bounding box. (0,0) is upper left.
    
    pub top: Option<i32>,
    /// Left coordinate of the bounding box. (0,0) is upper left.
    
    pub left: Option<i32>,
    /// Width of the bounding box in pixels.
    
    pub width: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2beta1ImageLocation {}


/// Replace each input value with a given `Value`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ReplaceValueConfig {
    /// Value to replace it with.
    #[serde(rename="newValue")]
    
    pub new_value: Option<GooglePrivacyDlpV2beta1Value>,
}

impl client::Part for GooglePrivacyDlpV2beta1ReplaceValueConfig {}


/// Container structure for the content to inspect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1ContentItem {
    /// Structured content for inspection.
    
    pub table: Option<GooglePrivacyDlpV2beta1Table>,
    /// Content data to inspect or redact.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Type of the content, as defined in Content-Type HTTP header.
    /// Supported types are: all "text" types, octet streams, PNG images,
    /// JPEG images.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// String data to inspect or redact.
    
    pub value: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1ContentItem {}


/// Replaces an identifier with a surrogate using FPE with the FFX
/// mode of operation.
/// The identifier must be representable by the US-ASCII character set.
/// For a given crypto key and context, the same identifier will be
/// replaced with the same surrogate.
/// Identifiers must be at least two characters long.
/// In the case that the identifier is the empty string, it will be skipped.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfig {
    /// The key used by the encryption algorithm. [required]
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<GooglePrivacyDlpV2beta1CryptoKey>,
    /// A context may be used for higher security since the same
    /// identifier in two different contexts likely will be given a distinct
    /// surrogate. The principle is that the likeliness is inversely related
    /// to the ratio of the number of distinct identifiers per context over the
    /// number of possible surrogates: As long as this ratio is small, the
    /// likehood is large.
    /// 
    /// If the context is not set, a default tweak will be used.
    /// If the context is set but:
    /// 
    /// 1. there is no record present when transforming a given value or
    /// 1. the field is not present when transforming a given value,
    /// 
    /// a default tweak will be used.
    /// 
    /// Note that case (1) is expected when an `InfoTypeTransformation` is
    /// applied to both structured and non-structured `ContentItem`s.
    /// Currently, the referenced field may be of value type integer or string.
    /// 
    /// The tweak is constructed as a sequence of bytes in big endian byte order
    /// such that:
    /// 
    /// - a 64 bit integer is encoded followed by a single byte of value 1
    /// - a string is encoded in UTF-8 format followed by a single byte of value 2
    /// 
    /// This is also known as the 'tweak', as in tweakable encryption.
    
    pub context: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// The custom info type to annotate the surrogate with.
    /// This annotation will be applied to the surrogate by prefixing it with
    /// the name of the custom info type followed by the number of
    /// characters comprising the surrogate. The following scheme defines the
    /// format: info_type_name(surrogate_character_count):surrogate
    /// 
    /// For example, if the name of custom info type is ‘MY_TOKEN_INFO_TYPE’ and
    /// the surrogate is ‘abc’, the full replacement value
    /// will be: ‘MY_TOKEN_INFO_TYPE(3):abc’
    /// 
    /// This annotation identifies the surrogate when inspecting content using the
    /// custom info type
    /// [`SurrogateType`](https://cloud.google.com/dlp/docs/reference/rest/v2beta1/InspectConfig#surrogatetype).
    /// This facilitates reversal of the surrogate when it occurs in free text.
    /// 
    /// In order for inspection to work properly, the name of this info type must
    /// not occur naturally anywhere in your data; otherwise, inspection may
    /// find a surrogate that does not correspond to an actual identifier.
    /// Therefore, choose your custom info type name carefully after considering
    /// what your data looks like. One way to select a name that has a high chance
    /// of yielding reliable detection is to include one or more unicode characters
    /// that are highly improbable to exist in your data.
    /// For example, assuming your data is entered from a regular ASCII keyboard,
    /// the symbol with the hex code point 29DD might be used like so:
    /// ⧝MY_TOKEN_TYPE
    #[serde(rename="surrogateInfoType")]
    
    pub surrogate_info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// no description provided
    #[serde(rename="commonAlphabet")]
    
    pub common_alphabet: Option<String>,
    /// The native way to select the alphabet. Must be in the range [2, 62].
    
    pub radix: Option<i32>,
    /// This is supported by mapping these to the alphanumeric characters
    /// that the FFX mode natively supports. This happens before/after
    /// encryption/decryption.
    /// Each character listed must appear only once.
    /// Number of characters must be in the range [2, 62].
    /// This must be encoded as ASCII.
    /// The order of characters does not matter.
    #[serde(rename="customAlphabet")]
    
    pub custom_alphabet: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1CryptoReplaceFfxFpeConfig {}


/// Represents a color in the RGB color space.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Color {
    /// The amount of red in the color as a value in the interval [0, 1].
    
    pub red: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    
    pub green: Option<f32>,
    /// The amount of blue in the color as a value in the interval [0, 1].
    
    pub blue: Option<f32>,
}

impl client::Part for GooglePrivacyDlpV2beta1Color {}


/// Structured content to inspect. Up to 50,000 `Value`s per request allowed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Table {
    /// no description provided
    
    pub rows: Option<Vec<GooglePrivacyDlpV2beta1Row>>,
    /// no description provided
    
    pub headers: Option<Vec<GooglePrivacyDlpV2beta1FieldId>>,
}

impl client::Part for GooglePrivacyDlpV2beta1Table {}


/// Message for detecting output from deidentification transformations
/// such as
/// [`CryptoReplaceFfxFpeConfig`](https://cloud.google.com/dlp/docs/reference/rest/v2beta1/content/deidentify#CryptoReplaceFfxFpeConfig).
/// These types of transformations are
/// those that perform pseudonymization, thereby producing a “surrogate” as
/// output. This should be used in conjunction with a field on the
/// transformation such as `surrogate_info_type`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1SurrogateType { _never_set: Option<bool> }

impl client::Part for GooglePrivacyDlpV2beta1SurrogateType {}


/// This is a data encryption key (DEK) (as opposed to
/// a key encryption key (KEK) stored by KMS).
/// When using KMS to wrap/unwrap DEKs, be sure to set an appropriate
/// IAM policy on the KMS CryptoKey (KEK) to ensure an attacker cannot
/// unwrap the data crypto key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CryptoKey {
    /// no description provided
    #[serde(rename="kmsWrapped")]
    
    pub kms_wrapped: Option<GooglePrivacyDlpV2beta1KmsWrappedCryptoKey>,
    /// no description provided
    
    pub unwrapped: Option<GooglePrivacyDlpV2beta1UnwrappedCryptoKey>,
    /// no description provided
    
    pub transient: Option<GooglePrivacyDlpV2beta1TransientCryptoKey>,
}

impl client::Part for GooglePrivacyDlpV2beta1CryptoKey {}


/// Max findings configuration per info type, per content item or long running
/// operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InfoTypeLimit {
    /// Type of information the findings limit applies to. Only one limit per
    /// info_type should be provided. If InfoTypeLimit does not have an
    /// info_type, the DLP API applies the limit against all info_types that are
    /// found but not specified in another InfoTypeLimit.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// Max findings limit for the given infoType.
    #[serde(rename="maxFindings")]
    
    pub max_findings: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2beta1InfoTypeLimit {}


/// Set of primitive values supported by the system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Value {
    /// no description provided
    #[serde(rename="floatValue")]
    
    pub float_value: Option<f64>,
    /// no description provided
    #[serde(rename="timeValue")]
    
    pub time_value: Option<GoogleTypeTimeOfDay>,
    /// no description provided
    #[serde(rename="integerValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub integer_value: Option<i64>,
    /// no description provided
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
    /// no description provided
    #[serde(rename="dateValue")]
    
    pub date_value: Option<GoogleTypeDate>,
    /// no description provided
    #[serde(rename="timestampValue")]
    
    pub timestamp_value: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// no description provided
    #[serde(rename="booleanValue")]
    
    pub boolean_value: Option<bool>,
}

impl client::Part for GooglePrivacyDlpV2beta1Value {}


/// The field type of `value` and `field` do not need to match to be
/// considered equal, but not all comparisons are possible.
/// 
/// A `value` of type:
/// 
/// - `string` can be compared against all other types
/// - `boolean` can only be compared against other booleans
/// - `integer` can be compared against doubles or a string if the string value
/// can be parsed as an integer.
/// - `double` can be compared against integers or a string if the string can
/// be parsed as a double.
/// - `Timestamp` can be compared against strings in RFC 3339 date string
/// format.
/// - `TimeOfDay` can be compared against timestamps and strings in the format
/// of 'HH:mm:ss'.
/// 
/// If we fail to compare do to type mismatch, a warning will be given and
/// the condition will evaluate to false.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1Condition {
    /// Field within the record this condition is evaluated against. [required]
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
    /// Operator used to compare the field or info type to the value. [required]
    
    pub operator: Option<String>,
    /// Value to compare against. [Required, except for `EXISTS` tests.]
    
    pub value: Option<GooglePrivacyDlpV2beta1Value>,
}

impl client::Part for GooglePrivacyDlpV2beta1Condition {}


/// Datastore partition ID.
/// A partition ID identifies a grouping of entities. The grouping is always
/// by project and namespace, however the namespace ID may be empty.
/// 
/// A partition ID contains several dimensions:
/// project ID and namespace ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PartitionId {
    /// If not empty, the ID of the namespace to which the entities belong.
    #[serde(rename="namespaceId")]
    
    pub namespace_id: Option<String>,
    /// The ID of the project to which the entities belong.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1PartitionId {}


/// Results of inspecting a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inspect content](ContentInspectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1InspectContentResponse {
    /// Each content_item from the request has a result in this list, in the
    /// same order as the request.
    
    pub results: Option<Vec<GooglePrivacyDlpV2beta1InspectResult>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2beta1InspectContentResponse {}


/// Request to search for potentially sensitive info in a list of items
/// and replace it with a default or provided content.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [redact content](ContentRedactCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RedactContentRequest {
    /// The list of items to inspect. Up to 100 are allowed per request.
    
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
    /// The strings to replace findings text findings with. Must specify at least
    /// one of these or one ImageRedactionConfig if redacting images.
    #[serde(rename="replaceConfigs")]
    
    pub replace_configs: Option<Vec<GooglePrivacyDlpV2beta1ReplaceConfig>>,
    /// The configuration for specifying what content to redact from images.
    #[serde(rename="imageRedactionConfigs")]
    
    pub image_redaction_configs: Option<Vec<GooglePrivacyDlpV2beta1ImageRedactionConfig>>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
}

impl client::RequestValue for GooglePrivacyDlpV2beta1RedactContentRequest {}


/// Message defining a list of words or phrases to search for in the data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1WordList {
    /// Words or phrases defining the dictionary. The dictionary must contain
    /// at least one phrase and every phrase must contain at least 2 characters
    /// that are letters or digits. [required]
    
    pub words: Option<Vec<String>>,
}

impl client::Part for GooglePrivacyDlpV2beta1WordList {}


/// General identifier of a data field in a storage service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1FieldId {
    /// Name describing the field.
    #[serde(rename="columnName")]
    
    pub column_name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1FieldId {}


/// Summary of a single tranformation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TransformationSummary {
    /// The field transformation that was applied. This list will contain
    /// multiple only in the case of errors.
    #[serde(rename="fieldTransformations")]
    
    pub field_transformations: Option<Vec<GooglePrivacyDlpV2beta1FieldTransformation>>,
    /// The specific suppression option these stats apply to.
    #[serde(rename="recordSuppress")]
    
    pub record_suppress: Option<GooglePrivacyDlpV2beta1RecordSuppression>,
    /// Set if the transformation was limited to a specific info_type.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// The specific transformation these stats apply to.
    
    pub transformation: Option<GooglePrivacyDlpV2beta1PrimitiveTransformation>,
    /// no description provided
    
    pub results: Option<Vec<GooglePrivacyDlpV2beta1SummaryResult>>,
    /// Set if the transformation was limited to a specific FieldId.
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl client::Part for GooglePrivacyDlpV2beta1TransformationSummary {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations cancel risk analysis](RiskAnalysiOperationCancelCall) (request)
/// * [operations cancel inspect](InspectOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunningCancelOperationRequest {}


/// Partially mask a string by replacing a given number of characters with a
/// fixed character. Masking can start from the beginning or end of the string.
/// This can be used on data of any type (numbers, longs, and so on) and when
/// de-identifying structured data we'll attempt to preserve the original data's
/// type. (This allows you to take a long like 123 and modify it to a string like
/// **3.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CharacterMaskConfig {
    /// When masking a string, items in this list will be skipped when replacing.
    /// For example, if your string is 555-555-5555 and you ask us to skip `-` and
    /// mask 5 chars with * we would produce ***-*55-5555.
    #[serde(rename="charactersToIgnore")]
    
    pub characters_to_ignore: Option<Vec<GooglePrivacyDlpV2beta1CharsToIgnore>>,
    /// Character to mask the sensitive values&mdash;for example, "*" for an
    /// alphabetic string such as name, or "0" for a numeric string such as ZIP
    /// code or credit card number. String must have length 1. If not supplied, we
    /// will default to "*" for strings, 0 for digits.
    #[serde(rename="maskingCharacter")]
    
    pub masking_character: Option<String>,
    /// Mask characters in reverse order. For example, if `masking_character` is
    /// '0', number_to_mask is 14, and `reverse_order` is false, then
    /// 1234-5678-9012-3456 -> 00000000000000-3456
    /// If `masking_character` is '*', `number_to_mask` is 3, and `reverse_order`
    /// is true, then 12345 -> 12***
    #[serde(rename="reverseOrder")]
    
    pub reverse_order: Option<bool>,
    /// Number of characters to mask. If not set, all matching chars will be
    /// masked. Skipped characters do not count towards this tally.
    #[serde(rename="numberToMask")]
    
    pub number_to_mask: Option<i32>,
}

impl client::Part for GooglePrivacyDlpV2beta1CharacterMaskConfig {}


/// Use this to have a random data crypto key generated.
/// It will be discarded after the operation/request finishes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TransientCryptoKey {
    /// Name of the key. [required]
    /// This is an arbitrary string used to differentiate different keys.
    /// A unique key is generated per name: two separate `TransientCryptoKey`
    /// protos share the same generated key if their names are the same.
    /// When the data crypto key is generated, this name is not used in any way
    /// (repeating the api call will result in a different key being generated).
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1TransientCryptoKey {}


/// Request for scheduling a scan of a data subset from a Google Platform data
/// repository.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations create inspect](InspectOperationCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1CreateInspectOperationRequest {
    /// Specification of the data set to process.
    #[serde(rename="storageConfig")]
    
    pub storage_config: Option<GooglePrivacyDlpV2beta1StorageConfig>,
    /// Optional location to store findings.
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<GooglePrivacyDlpV2beta1OutputStorageConfig>,
    /// Additional configuration settings for long running operations.
    #[serde(rename="operationConfig")]
    
    pub operation_config: Option<GooglePrivacyDlpV2beta1OperationConfig>,
    /// Configuration for the inspector.
    #[serde(rename="inspectConfig")]
    
    pub inspect_config: Option<GooglePrivacyDlpV2beta1InspectConfig>,
}

impl client::RequestValue for GooglePrivacyDlpV2beta1CreateInspectOperationRequest {}


/// A column with a semantic tag attached.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1TaggedField {
    /// A column can be tagged with a custom tag. In this case, the user must
    /// indicate an auxiliary table that contains statistical information on
    /// the possible values of this column (below).
    #[serde(rename="customTag")]
    
    pub custom_tag: Option<String>,
    /// A column can be tagged with a InfoType to use the relevant public
    /// dataset as a statistical model of population, if available. We
    /// currently support US ZIP codes, region codes, ages and genders.
    #[serde(rename="infoType")]
    
    pub info_type: Option<GooglePrivacyDlpV2beta1InfoType>,
    /// If no semantic tag is indicated, we infer the statistical model from
    /// the distribution of values in the input data
    
    pub inferred: Option<GoogleProtobufEmpty>,
    /// Identifies the column. [required]
    
    pub field: Option<GooglePrivacyDlpV2beta1FieldId>,
}

impl client::Part for GooglePrivacyDlpV2beta1TaggedField {}


/// Results of redacting a list of items.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [redact content](ContentRedactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1RedactContentResponse {
    /// The redacted content.
    
    pub items: Option<Vec<GooglePrivacyDlpV2beta1ContentItem>>,
}

impl client::ResponseResult for GooglePrivacyDlpV2beta1RedactContentResponse {}


/// A reference to a property relative to the Datastore kind expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GooglePrivacyDlpV2beta1PropertyReference {
    /// The name of the property.
    /// If name includes "."s, it may be interpreted as a property name path.
    
    pub name: Option<String>,
}

impl client::Part for GooglePrivacyDlpV2beta1PropertyReference {}


