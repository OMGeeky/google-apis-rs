use super::*;
/// Autoscaling config for an instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingConfig {
    /// Required. Autoscaling limits for an instance.
    #[serde(rename="autoscalingLimits")]
    
    pub autoscaling_limits: Option<AutoscalingLimits>,
    /// Required. The autoscaling targets for an instance.
    #[serde(rename="autoscalingTargets")]
    
    pub autoscaling_targets: Option<AutoscalingTargets>,
}

impl client::Part for AutoscalingConfig {}


/// The autoscaling limits for the instance. Users can define the minimum and maximum compute capacity allocated to the instance, and the autoscaler will only scale within that range. Users can either use nodes or processing units to specify the limits, but should use the same unit to set both the min_limit and max_limit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingLimits {
    /// Maximum number of nodes allocated to the instance. If set, this number should be greater than or equal to min_nodes.
    #[serde(rename="maxNodes")]
    
    pub max_nodes: Option<i32>,
    /// Maximum number of processing units allocated to the instance. If set, this number should be multiples of 1000 and be greater than or equal to min_processing_units.
    #[serde(rename="maxProcessingUnits")]
    
    pub max_processing_units: Option<i32>,
    /// Minimum number of nodes allocated to the instance. If set, this number should be greater than or equal to 1.
    #[serde(rename="minNodes")]
    
    pub min_nodes: Option<i32>,
    /// Minimum number of processing units allocated to the instance. If set, this number should be multiples of 1000.
    #[serde(rename="minProcessingUnits")]
    
    pub min_processing_units: Option<i32>,
}

impl client::Part for AutoscalingLimits {}


/// The autoscaling targets for an instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingTargets {
    /// Required. The target high priority cpu utilization percentage that the autoscaler should be trying to achieve for the instance. This number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.
    #[serde(rename="highPriorityCpuUtilizationPercent")]
    
    pub high_priority_cpu_utilization_percent: Option<i32>,
    /// Required. The target storage utilization percentage that the autoscaler should be trying to achieve for the instance. This number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 99] inclusive.
    #[serde(rename="storageUtilizationPercent")]
    
    pub storage_utilization_percent: Option<i32>,
}

impl client::Part for AutoscalingTargets {}


/// A backup of a Cloud Spanner database.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups create projects](ProjectInstanceBackupCreateCall) (request)
/// * [instances backups get projects](ProjectInstanceBackupGetCall) (response)
/// * [instances backups patch projects](ProjectInstanceBackupPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Output only. The time the CreateBackup request is received. If the request does not specify `version_time`, the `version_time` of the backup will be equivalent to the `create_time`.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required for the CreateBackup operation. Name of the database from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects//instances//databases/`.
    
    pub database: Option<String>,
    /// Output only. The database dialect information for the backup.
    #[serde(rename="databaseDialect")]
    
    pub database_dialect: Option<BackupDatabaseDialectEnum>,
    /// Output only. The encryption information for the backup.
    #[serde(rename="encryptionInfo")]
    
    pub encryption_info: Option<EncryptionInfo>,
    /// Required for the CreateBackup operation. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 366 days from the time the CreateBackup request is processed. Once the `expire_time` has passed, the backup is eligible to be automatically deleted by Cloud Spanner to free the resources used by the backup.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The max allowed expiration time of the backup, with microseconds granularity. A backup's expiration time can be configured in multiple APIs: CreateBackup, UpdateBackup, CopyBackup. When updating or copying an existing backup, the expiration time specified must be less than `Backup.max_expire_time`.
    #[serde(rename="maxExpireTime")]
    
    pub max_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only for the CreateBackup operation. Required for the UpdateBackup operation. A globally unique identifier for the backup which cannot be changed. Values are of the form `projects//instances//backups/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. The backup is stored in the location(s) specified in the instance configuration of the instance containing the backup, identified by the prefix of the backup name of the form `projects//instances/`.
    
    pub name: Option<String>,
    /// Output only. The names of the destination backups being created by copying this source backup. The backup names are of the form `projects//instances//backups/`. Referencing backups may exist in different instances. The existence of any referencing backup prevents the backup from being deleted. When the copy operation is done (either successfully completed or cancelled or the destination backup is deleted), the reference to the backup is removed.
    #[serde(rename="referencingBackups")]
    
    pub referencing_backups: Option<Vec<String>>,
    /// Output only. The names of the restored databases that reference the backup. The database names are of the form `projects//instances//databases/`. Referencing databases may exist in different instances. The existence of any referencing database prevents the backup from being deleted. When a restored database from the backup enters the `READY` state, the reference to the backup is removed.
    #[serde(rename="referencingDatabases")]
    
    pub referencing_databases: Option<Vec<String>>,
    /// Output only. Size of the backup in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// Output only. The current state of the backup.
    
    pub state: Option<BackupStateEnum>,
    /// The backup will contain an externally consistent copy of the database at the timestamp specified by `version_time`. If `version_time` is not specified, the system will set `version_time` to the `create_time` of the backup.
    #[serde(rename="versionTime")]
    
    pub version_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Backup {}
impl client::ResponseResult for Backup {}


/// Information about a backup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Name of the backup.
    
    pub backup: Option<String>,
    /// The time the CreateBackup request was received.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name of the database the backup was created from.
    #[serde(rename="sourceDatabase")]
    
    pub source_database: Option<String>,
    /// The backup contains an externally consistent copy of `source_database` at the timestamp specified by `version_time`. If the CreateBackup request did not specify `version_time`, the `version_time` of the backup is equivalent to the `create_time`.
    #[serde(rename="versionTime")]
    
    pub version_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for BackupInfo {}


/// The request for BatchCreateSessions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions batch create projects](ProjectInstanceDatabaseSessionBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateSessionsRequest {
    /// Required. The number of sessions to be created in this batch call. The API may return fewer than the requested number of sessions. If a specific number of sessions are desired, the client can make additional calls to BatchCreateSessions (adjusting session_count as necessary).
    #[serde(rename="sessionCount")]
    
    pub session_count: Option<i32>,
    /// Parameters to be applied to each created session.
    #[serde(rename="sessionTemplate")]
    
    pub session_template: Option<Session>,
}

impl client::RequestValue for BatchCreateSessionsRequest {}


/// The response for BatchCreateSessions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions batch create projects](ProjectInstanceDatabaseSessionBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateSessionsResponse {
    /// The freshly created sessions.
    
    pub session: Option<Vec<Session>>,
}

impl client::ResponseResult for BatchCreateSessionsResponse {}


/// The request for BatchWrite.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions batch write projects](ProjectInstanceDatabaseSessionBatchWriteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchWriteRequest {
    /// Optional. When `exclude_txn_from_change_streams` is set to `true`: * Modifications from all transactions in this batch write operation will not be recorded in change streams with DDL option `allow_txn_exclusion=true` that are tracking columns modified by these transactions. * Modifications from all transactions in this batch write operation will be recorded in change streams with DDL option `allow_txn_exclusion=false or not set` that are tracking columns modified by these transactions. When `exclude_txn_from_change_streams` is set to `false` or not set, Modifications from all transactions in this batch write operation will be recorded in all change streams that are tracking columns modified by these transactions.
    #[serde(rename="excludeTxnFromChangeStreams")]
    
    pub exclude_txn_from_change_streams: Option<bool>,
    /// Required. The groups of mutations to be applied.
    #[serde(rename="mutationGroups")]
    
    pub mutation_groups: Option<Vec<MutationGroup>>,
    /// Common options for this request.
    #[serde(rename="requestOptions")]
    
    pub request_options: Option<RequestOptions>,
}

impl client::RequestValue for BatchWriteRequest {}


/// The result of applying a batch of mutations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions batch write projects](ProjectInstanceDatabaseSessionBatchWriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchWriteResponse {
    /// The commit timestamp of the transaction that applied this batch. Present if `status` is `OK`, absent otherwise.
    #[serde(rename="commitTimestamp")]
    
    pub commit_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The mutation groups applied in this batch. The values index into the `mutation_groups` field in the corresponding `BatchWriteRequest`.
    
    pub indexes: Option<Vec<i32>>,
    /// An `OK` status indicates success. Any other status indicates a failure.
    
    pub status: Option<Status>,
}

impl client::ResponseResult for BatchWriteResponse {}


/// The request for BeginTransaction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions begin transaction projects](ProjectInstanceDatabaseSessionBeginTransactionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BeginTransactionRequest {
    /// Required. Options for the new transaction.
    
    pub options: Option<TransactionOptions>,
    /// Common options for this request. Priority is ignored for this request. Setting the priority in this request_options struct will not do anything. To set the priority for a transaction, set it on the reads and writes that are part of this transaction instead.
    #[serde(rename="requestOptions")]
    
    pub request_options: Option<RequestOptions>,
}

impl client::RequestValue for BeginTransactionRequest {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workforce identity pool. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}`: All workforce identities in a group. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All workforce identities with a specific attribute value. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`: All identities in a workforce identity pool. * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workload identity pool. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}`: A workload identity pool group. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All identities in a workload identity pool with a certain attribute. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*`: All identities in a workload identity pool. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: Deleted single identity in a workforce identity pool. For example, `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value`.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-overview). For a list of the available pre-defined roles, see [here](https://cloud.google.com/iam/docs/understanding-roles).
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// Metadata associated with a parent-child relationship appearing in a PlanNode.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChildLink {
    /// The node to which the link points.
    #[serde(rename="childIndex")]
    
    pub child_index: Option<i32>,
    /// The type of the link. For example, in Hash Joins this could be used to distinguish between the build child and the probe child, or in the case of the child being an output variable, to represent the tag associated with the output variable.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Only present if the child node is SCALAR and corresponds to an output variable of the parent node. The field carries the name of the output variable. For example, a `TableScan` operator that reads rows from a table will have child links to the `SCALAR` nodes representing the output variables created for each column that is read by the operator. The corresponding `variable` fields will be set to the variable names assigned to the columns.
    
    pub variable: Option<String>,
}

impl client::Part for ChildLink {}


/// The request for Commit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions commit projects](ProjectInstanceDatabaseSessionCommitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitRequest {
    /// Optional. The amount of latency this request is configured to incur in order to improve throughput. If this field is not set, Spanner assumes requests are relatively latency sensitive and automatically determines an appropriate delay time. You can specify a commit delay value between 0 and 500 ms.
    #[serde(rename="maxCommitDelay")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_commit_delay: Option<client::chrono::Duration>,
    /// The mutations to be executed when this transaction commits. All mutations are applied atomically, in the order they appear in this list.
    
    pub mutations: Option<Vec<Mutation>>,
    /// Common options for this request.
    #[serde(rename="requestOptions")]
    
    pub request_options: Option<RequestOptions>,
    /// If `true`, then statistics related to the transaction will be included in the CommitResponse. Default value is `false`.
    #[serde(rename="returnCommitStats")]
    
    pub return_commit_stats: Option<bool>,
    /// Execute mutations in a temporary transaction. Note that unlike commit of a previously-started transaction, commit with a temporary transaction is non-idempotent. That is, if the `CommitRequest` is sent to Cloud Spanner more than once (for instance, due to retries in the application, or in the transport library), it is possible that the mutations are executed more than once. If this is undesirable, use BeginTransaction and Commit instead.
    #[serde(rename="singleUseTransaction")]
    
    pub single_use_transaction: Option<TransactionOptions>,
    /// Commit a previously-started transaction.
    #[serde(rename="transactionId")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub transaction_id: Option<Vec<u8>>,
}

impl client::RequestValue for CommitRequest {}


/// The response for Commit.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions commit projects](ProjectInstanceDatabaseSessionCommitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitResponse {
    /// The statistics about this Commit. Not returned by default. For more information, see CommitRequest.return_commit_stats.
    #[serde(rename="commitStats")]
    
    pub commit_stats: Option<CommitStats>,
    /// The Cloud Spanner timestamp at which the transaction committed.
    #[serde(rename="commitTimestamp")]
    
    pub commit_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for CommitResponse {}


/// Additional statistics about a commit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitStats {
    /// The total number of mutations for the transaction. Knowing the `mutation_count` value can help you maximize the number of mutations in a transaction and minimize the number of API round trips. You can also monitor this value to prevent transactions from exceeding the system [limit](https://cloud.google.com/spanner/quotas#limits_for_creating_reading_updating_and_deleting_data). If the number of mutations exceeds the limit, the server returns [INVALID_ARGUMENT](https://cloud.google.com/spanner/docs/reference/rest/v1/Code#ENUM_VALUES.INVALID_ARGUMENT).
    #[serde(rename="mutationCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub mutation_count: Option<i64>,
}

impl client::Part for CommitStats {}


/// A message representing context for a KeyRangeInfo, including a label, value, unit, and severity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContextValue {
    /// The label for the context value. e.g. "latency".
    
    pub label: Option<LocalizedString>,
    /// The severity of this context.
    
    pub severity: Option<ContextValueSeverityEnum>,
    /// The unit of the context value.
    
    pub unit: Option<String>,
    /// The value for the context.
    
    pub value: Option<f32>,
}

impl client::Part for ContextValue {}


/// Encryption configuration for the copied backup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyBackupEncryptionConfig {
    /// Required. The encryption type of the backup.
    #[serde(rename="encryptionType")]
    
    pub encryption_type: Option<CopyBackupEncryptionConfigEncryptionTypeEnum>,
    /// Optional. The Cloud KMS key that will be used to protect the backup. This field should be set only when encryption_type is `CUSTOMER_MANAGED_ENCRYPTION`. Values are of the form `projects//locations//keyRings//cryptoKeys/`.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for CopyBackupEncryptionConfig {}


/// The request for CopyBackup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups copy projects](ProjectInstanceBackupCopyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyBackupRequest {
    /// Required. The id of the backup copy. The `backup_id` appended to `parent` forms the full backup_uri of the form `projects//instances//backups/`.
    #[serde(rename="backupId")]
    
    pub backup_id: Option<String>,
    /// Optional. The encryption configuration used to encrypt the backup. If this field is not specified, the backup will use the same encryption configuration as the source backup by default, namely encryption_type = `USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION`.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<CopyBackupEncryptionConfig>,
    /// Required. The expiration time of the backup in microsecond granularity. The expiration time must be at least 6 hours and at most 366 days from the `create_time` of the source backup. Once the `expire_time` has passed, the backup is eligible to be automatically deleted by Cloud Spanner to free the resources used by the backup.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The source backup to be copied. The source backup needs to be in READY state for it to be copied. Once CopyBackup is in progress, the source backup cannot be deleted or cleaned up on expiration until CopyBackup is finished. Values are of the form: `projects//instances//backups/`.
    #[serde(rename="sourceBackup")]
    
    pub source_backup: Option<String>,
}

impl client::RequestValue for CopyBackupRequest {}


/// The request for CreateDatabase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases create projects](ProjectInstanceDatabaseCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateDatabaseRequest {
    /// Required. A `CREATE DATABASE` statement, which specifies the ID of the new database. The database ID must conform to the regular expression `a-z*[a-z0-9]` and be between 2 and 30 characters in length. If the database ID is a reserved word or if it contains a hyphen, the database ID must be enclosed in backticks (`` ` ``).
    #[serde(rename="createStatement")]
    
    pub create_statement: Option<String>,
    /// Optional. The dialect of the Cloud Spanner Database.
    #[serde(rename="databaseDialect")]
    
    pub database_dialect: Option<CreateDatabaseRequestDatabaseDialectEnum>,
    /// Optional. The encryption configuration for the database. If this field is not specified, Cloud Spanner will encrypt/decrypt all data at rest using Google default encryption.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Optional. A list of DDL statements to run inside the newly created database. Statements can create tables, indexes, etc. These statements execute atomically with the creation of the database: if there is an error in any statement, the database is not created.
    #[serde(rename="extraStatements")]
    
    pub extra_statements: Option<Vec<String>>,
    /// Optional. Proto descriptors used by CREATE/ALTER PROTO BUNDLE statements in 'extra_statements' above. Contains a protobuf-serialized [google.protobuf.FileDescriptorSet](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto). To generate it, [install](https://grpc.io/docs/protoc-installation/) and run `protoc` with --include_imports and --descriptor_set_out. For example, to generate for moon/shot/app.proto, run ``` $protoc --proto_path=/app_path --proto_path=/lib_path \ --include_imports \ --descriptor_set_out=descriptors.data \ moon/shot/app.proto ``` For more details, see protobuffer [self description](https://developers.google.com/protocol-buffers/docs/techniques#self-description).
    #[serde(rename="protoDescriptors")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub proto_descriptors: Option<Vec<u8>>,
}

impl client::RequestValue for CreateDatabaseRequest {}


/// The request for CreateInstanceConfigRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs create projects](ProjectInstanceConfigCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateInstanceConfigRequest {
    /// Required. The InstanceConfig proto of the configuration to create. instance_config.name must be `/instanceConfigs/`. instance_config.base_config must be a Google managed configuration name, e.g. /instanceConfigs/us-east1, /instanceConfigs/nam3.
    #[serde(rename="instanceConfig")]
    
    pub instance_config: Option<InstanceConfig>,
    /// Required. The ID of the instance config to create. Valid identifiers are of the form `custom-[-a-z0-9]*[a-z0-9]` and must be between 2 and 64 characters in length. The `custom-` prefix is required to avoid name conflicts with Google managed configurations.
    #[serde(rename="instanceConfigId")]
    
    pub instance_config_id: Option<String>,
    /// An option to validate, but not actually execute, a request, and provide the same response.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for CreateInstanceConfigRequest {}


/// The request for CreateInstancePartition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances instance partitions create projects](ProjectInstanceInstancePartitionCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateInstancePartitionRequest {
    /// Required. The instance partition to create. The instance_partition.name may be omitted, but if specified must be `/instancePartitions/`.
    #[serde(rename="instancePartition")]
    
    pub instance_partition: Option<InstancePartition>,
    /// Required. The ID of the instance partition to create. Valid identifiers are of the form `a-z*[a-z0-9]` and must be between 2 and 64 characters in length.
    #[serde(rename="instancePartitionId")]
    
    pub instance_partition_id: Option<String>,
}

impl client::RequestValue for CreateInstancePartitionRequest {}


/// The request for CreateInstance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances create projects](ProjectInstanceCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    /// Required. The instance to create. The name may be omitted, but if specified must be `/instances/`.
    
    pub instance: Option<Instance>,
    /// Required. The ID of the instance to create. Valid identifiers are of the form `a-z*[a-z0-9]` and must be between 2 and 64 characters in length.
    #[serde(rename="instanceId")]
    
    pub instance_id: Option<String>,
}

impl client::RequestValue for CreateInstanceRequest {}


/// The request for CreateSession.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions create projects](ProjectInstanceDatabaseSessionCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    /// Required. The session to create.
    
    pub session: Option<Session>,
}

impl client::RequestValue for CreateSessionRequest {}


/// A Cloud Spanner database.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases get projects](ProjectInstanceDatabaseGetCall) (response)
/// * [instances databases patch projects](ProjectInstanceDatabasePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    /// Output only. If exists, the time at which the database creation started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The dialect of the Cloud Spanner Database.
    #[serde(rename="databaseDialect")]
    
    pub database_dialect: Option<DatabaseDatabaseDialectEnum>,
    /// Output only. The read-write region which contains the database's leader replicas. This is the same as the value of default_leader database option set using DatabaseAdmin.CreateDatabase or DatabaseAdmin.UpdateDatabaseDdl. If not explicitly set, this is empty.
    #[serde(rename="defaultLeader")]
    
    pub default_leader: Option<String>,
    /// Output only. Earliest timestamp at which older versions of the data can be read. This value is continuously updated by Cloud Spanner and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery.
    #[serde(rename="earliestVersionTime")]
    
    pub earliest_version_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether drop protection is enabled for this database. Defaults to false, if not set. For more details, please see how to [prevent accidental database deletion](https://cloud.google.com/spanner/docs/prevent-database-deletion).
    #[serde(rename="enableDropProtection")]
    
    pub enable_drop_protection: Option<bool>,
    /// Output only. For databases that are using customer managed encryption, this field contains the encryption configuration for the database. For databases that are using Google default or other types of encryption, this field is empty.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Output only. For databases that are using customer managed encryption, this field contains the encryption information for the database, such as all Cloud KMS key versions that are in use. The `encryption_status' field inside of each `EncryptionInfo` is not populated. For databases that are using Google default or other types of encryption, this field is empty. This field is propagated lazily from the backend. There might be a delay from when a key version is being used and when it appears in this field.
    #[serde(rename="encryptionInfo")]
    
    pub encryption_info: Option<Vec<EncryptionInfo>>,
    /// Required. The name of the database. Values are of the form `projects//instances//databases/`, where `` is as specified in the `CREATE DATABASE` statement. This name can be passed to other API methods to identify the database.
    
    pub name: Option<String>,
    /// Output only. If true, the database is being updated. If false, there are no ongoing update operations for the database.
    
    pub reconciling: Option<bool>,
    /// Output only. Applicable only for restored databases. Contains information about the restore source.
    #[serde(rename="restoreInfo")]
    
    pub restore_info: Option<RestoreInfo>,
    /// Output only. The current database state.
    
    pub state: Option<DatabaseStateEnum>,
    /// Output only. The period in which Cloud Spanner retains all versions of data for the database. This is the same as the value of version_retention_period database option set using UpdateDatabaseDdl. Defaults to 1 hour, if not set.
    #[serde(rename="versionRetentionPeriod")]
    
    pub version_retention_period: Option<String>,
}

impl client::RequestValue for Database {}
impl client::ResponseResult for Database {}


/// A Cloud Spanner database role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseRole {
    /// Required. The name of the database role. Values are of the form `projects//instances//databases//databaseRoles/` where `` is as specified in the `CREATE ROLE` DDL statement.
    
    pub name: Option<String>,
}

impl client::Part for DatabaseRole {}


/// Arguments to delete operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Delete {
    /// Required. The primary keys of the rows within table to delete. The primary keys must be specified in the order in which they appear in the `PRIMARY KEY()` clause of the table's equivalent DDL statement (the DDL statement used to create the table). Delete is idempotent. The transaction will succeed even if some or all rows do not exist.
    #[serde(rename="keySet")]
    
    pub key_set: Option<KeySet>,
    /// Required. The table whose rows will be deleted.
    
    pub table: Option<String>,
}

impl client::Part for Delete {}


/// A message representing a derived metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DerivedMetric {
    /// The name of the denominator metric. e.g. "rows".
    
    pub denominator: Option<LocalizedString>,
    /// The name of the numerator metric. e.g. "latency".
    
    pub numerator: Option<LocalizedString>,
}

impl client::Part for DerivedMetric {}


/// A message representing the key visualizer diagnostic messages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiagnosticMessage {
    /// Information about this diagnostic information.
    
    pub info: Option<LocalizedString>,
    /// The metric.
    
    pub metric: Option<LocalizedString>,
    /// Whether this message is specific only for the current metric. By default Diagnostics are shown for all metrics, regardless which metric is the currently selected metric in the UI. However occasionally a metric will generate so many messages that the resulting visual clutter becomes overwhelming. In this case setting this to true, will show the diagnostic messages for that metric only if it is the currently selected metric.
    #[serde(rename="metricSpecific")]
    
    pub metric_specific: Option<bool>,
    /// The severity of the diagnostic message.
    
    pub severity: Option<DiagnosticMessageSeverityEnum>,
    /// The short message.
    #[serde(rename="shortMessage")]
    
    pub short_message: Option<LocalizedString>,
}

impl client::Part for DiagnosticMessage {}


/// The DirectedReadOptions can be used to indicate which replicas or regions should be used for non-transactional reads or queries. DirectedReadOptions may only be specified for a read-only transaction, otherwise the API will return an `INVALID_ARGUMENT` error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DirectedReadOptions {
    /// Exclude_replicas indicates that specified replicas should be excluded from serving requests. Spanner will not route requests to the replicas in this list.
    #[serde(rename="excludeReplicas")]
    
    pub exclude_replicas: Option<ExcludeReplicas>,
    /// Include_replicas indicates the order of replicas (as they appear in this list) to process the request. If auto_failover_disabled is set to true and all replicas are exhausted without finding a healthy replica, Spanner will wait for a replica in the list to become available, requests may fail due to `DEADLINE_EXCEEDED` errors.
    #[serde(rename="includeReplicas")]
    
    pub include_replicas: Option<IncludeReplicas>,
}

impl client::Part for DirectedReadOptions {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs operations cancel projects](ProjectInstanceConfigOperationCancelCall) (response)
/// * [instance configs operations delete projects](ProjectInstanceConfigOperationDeleteCall) (response)
/// * [instance configs ssd caches operations cancel projects](ProjectInstanceConfigSsdCachOperationCancelCall) (response)
/// * [instance configs ssd caches operations delete projects](ProjectInstanceConfigSsdCachOperationDeleteCall) (response)
/// * [instance configs delete projects](ProjectInstanceConfigDeleteCall) (response)
/// * [instances backups operations cancel projects](ProjectInstanceBackupOperationCancelCall) (response)
/// * [instances backups operations delete projects](ProjectInstanceBackupOperationDeleteCall) (response)
/// * [instances backups delete projects](ProjectInstanceBackupDeleteCall) (response)
/// * [instances databases operations cancel projects](ProjectInstanceDatabaseOperationCancelCall) (response)
/// * [instances databases operations delete projects](ProjectInstanceDatabaseOperationDeleteCall) (response)
/// * [instances databases sessions delete projects](ProjectInstanceDatabaseSessionDeleteCall) (response)
/// * [instances databases sessions rollback projects](ProjectInstanceDatabaseSessionRollbackCall) (response)
/// * [instances databases drop database projects](ProjectInstanceDatabaseDropDatabaseCall) (response)
/// * [instances instance partitions operations cancel projects](ProjectInstanceInstancePartitionOperationCancelCall) (response)
/// * [instances instance partitions operations delete projects](ProjectInstanceInstancePartitionOperationDeleteCall) (response)
/// * [instances instance partitions delete projects](ProjectInstanceInstancePartitionDeleteCall) (response)
/// * [instances operations cancel projects](ProjectInstanceOperationCancelCall) (response)
/// * [instances operations delete projects](ProjectInstanceOperationDeleteCall) (response)
/// * [instances delete projects](ProjectInstanceDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Encryption configuration for a Cloud Spanner database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The Cloud KMS key to be used for encrypting and decrypting the database. Values are of the form `projects//locations//keyRings//cryptoKeys/`.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// Encryption information for a Cloud Spanner database or backup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// Output only. If present, the status of a recent encrypt/decrypt call on underlying data for this database or backup. Regardless of status, data is always encrypted at rest.
    #[serde(rename="encryptionStatus")]
    
    pub encryption_status: Option<Status>,
    /// Output only. The type of encryption.
    #[serde(rename="encryptionType")]
    
    pub encryption_type: Option<EncryptionInfoEncryptionTypeEnum>,
    /// Output only. A Cloud KMS key version that is being used to protect the database or backup.
    #[serde(rename="kmsKeyVersion")]
    
    pub kms_key_version: Option<String>,
}

impl client::Part for EncryptionInfo {}


/// An ExcludeReplicas contains a repeated set of ReplicaSelection that should be excluded from serving requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExcludeReplicas {
    /// The directed read replica selector.
    #[serde(rename="replicaSelections")]
    
    pub replica_selections: Option<Vec<ReplicaSelection>>,
}

impl client::Part for ExcludeReplicas {}


/// The request for ExecuteBatchDml.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions execute batch dml projects](ProjectInstanceDatabaseSessionExecuteBatchDmlCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteBatchDmlRequest {
    /// Common options for this request.
    #[serde(rename="requestOptions")]
    
    pub request_options: Option<RequestOptions>,
    /// Required. A per-transaction sequence number used to identify this request. This field makes each request idempotent such that if the request is received multiple times, at most one will succeed. The sequence number must be monotonically increasing within the transaction. If a request arrives for the first time with an out-of-order sequence number, the transaction may be aborted. Replays of previously handled requests will yield the same response as the first execution.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub seqno: Option<i64>,
    /// Required. The list of statements to execute in this batch. Statements are executed serially, such that the effects of statement `i` are visible to statement `i+1`. Each statement must be a DML statement. Execution stops at the first failed statement; the remaining statements are not executed. Callers must provide at least one statement.
    
    pub statements: Option<Vec<Statement>>,
    /// Required. The transaction to use. Must be a read-write transaction. To protect against replays, single-use transactions are not supported. The caller must either supply an existing transaction ID or begin a new transaction.
    
    pub transaction: Option<TransactionSelector>,
}

impl client::RequestValue for ExecuteBatchDmlRequest {}


/// The response for ExecuteBatchDml. Contains a list of ResultSet messages, one for each DML statement that has successfully executed, in the same order as the statements in the request. If a statement fails, the status in the response body identifies the cause of the failure. To check for DML statements that failed, use the following approach: 1. Check the status in the response message. The google.rpc.Code enum value `OK` indicates that all statements were executed successfully. 2. If the status was not `OK`, check the number of result sets in the response. If the response contains `N` ResultSet messages, then statement `N+1` in the request failed. Example 1: * Request: 5 DML statements, all executed successfully. * Response: 5 ResultSet messages, with the status `OK`. Example 2: * Request: 5 DML statements. The third statement has a syntax error. * Response: 2 ResultSet messages, and a syntax error (`INVALID_ARGUMENT`) status. The number of ResultSet messages indicates that the third statement failed, and the fourth and fifth statements were not executed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions execute batch dml projects](ProjectInstanceDatabaseSessionExecuteBatchDmlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteBatchDmlResponse {
    /// One ResultSet for each statement in the request that ran successfully, in the same order as the statements in the request. Each ResultSet does not contain any rows. The ResultSetStats in each ResultSet contain the number of rows modified by the statement. Only the first ResultSet in the response contains valid ResultSetMetadata.
    #[serde(rename="resultSets")]
    
    pub result_sets: Option<Vec<ResultSet>>,
    /// If all DML statements are executed successfully, the status is `OK`. Otherwise, the error status of the first failed statement.
    
    pub status: Option<Status>,
}

impl client::ResponseResult for ExecuteBatchDmlResponse {}


/// The request for ExecuteSql and ExecuteStreamingSql.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions execute sql projects](ProjectInstanceDatabaseSessionExecuteSqlCall) (request)
/// * [instances databases sessions execute streaming sql projects](ProjectInstanceDatabaseSessionExecuteStreamingSqlCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteSqlRequest {
    /// If this is for a partitioned query and this field is set to `true`, the request is executed with Spanner Data Boost independent compute resources. If the field is set to `true` but the request does not set `partition_token`, the API returns an `INVALID_ARGUMENT` error.
    #[serde(rename="dataBoostEnabled")]
    
    pub data_boost_enabled: Option<bool>,
    /// Directed read options for this request.
    #[serde(rename="directedReadOptions")]
    
    pub directed_read_options: Option<DirectedReadOptions>,
    /// It is not always possible for Cloud Spanner to infer the right SQL type from a JSON value. For example, values of type `BYTES` and values of type `STRING` both appear in params as JSON strings. In these cases, `param_types` can be used to specify the exact SQL type for some or all of the SQL statement parameters. See the definition of Type for more information about SQL types.
    #[serde(rename="paramTypes")]
    
    pub param_types: Option<HashMap<String, Type>>,
    /// Parameter names and values that bind to placeholders in the SQL string. A parameter placeholder consists of the `@` character followed by the parameter name (for example, `@firstName`). Parameter names must conform to the naming requirements of identifiers as specified at https://cloud.google.com/spanner/docs/lexical#identifiers. Parameters can appear anywhere that a literal value is expected. The same parameter name can be used more than once, for example: `"WHERE id > @msg_id AND id < @msg_id + 100"` It is an error to execute a SQL statement with unbound parameters.
    
    pub params: Option<HashMap<String, json::Value>>,
    /// If present, results will be restricted to the specified partition previously created using PartitionQuery(). There must be an exact match for the values of fields common to this message and the PartitionQueryRequest message used to create this partition_token.
    #[serde(rename="partitionToken")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub partition_token: Option<Vec<u8>>,
    /// Used to control the amount of debugging information returned in ResultSetStats. If partition_token is set, query_mode can only be set to QueryMode.NORMAL.
    #[serde(rename="queryMode")]
    
    pub query_mode: Option<ExecuteSqlRequestQueryModeEnum>,
    /// Query optimizer configuration to use for the given query.
    #[serde(rename="queryOptions")]
    
    pub query_options: Option<QueryOptions>,
    /// Common options for this request.
    #[serde(rename="requestOptions")]
    
    pub request_options: Option<RequestOptions>,
    /// If this request is resuming a previously interrupted SQL statement execution, `resume_token` should be copied from the last PartialResultSet yielded before the interruption. Doing this enables the new SQL statement execution to resume where the last one left off. The rest of the request parameters must exactly match the request that yielded this token.
    #[serde(rename="resumeToken")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub resume_token: Option<Vec<u8>>,
    /// A per-transaction sequence number used to identify this request. This field makes each request idempotent such that if the request is received multiple times, at most one will succeed. The sequence number must be monotonically increasing within the transaction. If a request arrives for the first time with an out-of-order sequence number, the transaction may be aborted. Replays of previously handled requests will yield the same response as the first execution. Required for DML statements. Ignored for queries.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub seqno: Option<i64>,
    /// Required. The SQL string.
    
    pub sql: Option<String>,
    /// The transaction to use. For queries, if none is provided, the default is a temporary read-only transaction with strong concurrency. Standard DML statements require a read-write transaction. To protect against replays, single-use transactions are not supported. The caller must either supply an existing transaction ID or begin a new transaction. Partitioned DML requires an existing Partitioned DML transaction ID.
    
    pub transaction: Option<TransactionSelector>,
}

impl client::RequestValue for ExecuteSqlRequest {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// Message representing a single field of a struct.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    /// The name of the field. For reads, this is the column name. For SQL queries, it is the column alias (e.g., `"Word"` in the query `"SELECT 'hello' AS Word"`), or the column name (e.g., `"ColName"` in the query `"SELECT ColName FROM Table"`). Some columns might have an empty name (e.g., `"SELECT UPPER(ColName)"`). Note that a query result can contain multiple fields with the same name.
    
    pub name: Option<String>,
    /// The type of the field.
    #[serde(rename="type")]
    
    pub type_: Option<Type>,
}

impl client::Part for Field {}


/// Free instance specific metadata that is kept even after an instance has been upgraded for tracking purposes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FreeInstanceMetadata {
    /// Specifies the expiration behavior of a free instance. The default of ExpireBehavior is `REMOVE_AFTER_GRACE_PERIOD`. This can be modified during or after creation, and before expiration.
    #[serde(rename="expireBehavior")]
    
    pub expire_behavior: Option<FreeInstanceMetadataExpireBehaviorEnum>,
    /// Output only. Timestamp after which the instance will either be upgraded or scheduled for deletion after a grace period. ExpireBehavior is used to choose between upgrading or scheduling the free instance for deletion. This timestamp is set during the creation of a free instance.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. If present, the timestamp at which the free instance was upgraded to a provisioned instance.
    #[serde(rename="upgradeTime")]
    
    pub upgrade_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for FreeInstanceMetadata {}


/// The response for GetDatabaseDdl.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases get ddl projects](ProjectInstanceDatabaseGetDdlCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetDatabaseDdlResponse {
    /// Proto descriptors stored in the database. Contains a protobuf-serialized [google.protobuf.FileDescriptorSet](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto). For more details, see protobuffer [self description](https://developers.google.com/protocol-buffers/docs/techniques#self-description).
    #[serde(rename="protoDescriptors")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub proto_descriptors: Option<Vec<u8>>,
    /// A list of formatted DDL statements defining the schema of the database specified in the request.
    
    pub statements: Option<Vec<String>>,
}

impl client::ResponseResult for GetDatabaseDdlResponse {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups get iam policy projects](ProjectInstanceBackupGetIamPolicyCall) (request)
/// * [instances databases get iam policy projects](ProjectInstanceDatabaseGetIamPolicyCall) (request)
/// * [instances get iam policy projects](ProjectInstanceGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// An IncludeReplicas contains a repeated set of ReplicaSelection which indicates the order in which replicas should be considered.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IncludeReplicas {
    /// If true, Spanner will not route requests to a replica outside the include_replicas list when all of the specified replicas are unavailable or unhealthy. Default value is `false`.
    #[serde(rename="autoFailoverDisabled")]
    
    pub auto_failover_disabled: Option<bool>,
    /// The directed read replica selector.
    #[serde(rename="replicaSelections")]
    
    pub replica_selections: Option<Vec<ReplicaSelection>>,
}

impl client::Part for IncludeReplicas {}


/// Recommendation to add new indexes to run queries more efficiently.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexAdvice {
    /// Optional. DDL statements to add new indexes that will improve the query.
    
    pub ddl: Option<Vec<String>>,
    /// Optional. Estimated latency improvement factor. For example if the query currently takes 500 ms to run and the estimated latency with new indexes is 100 ms this field will be 5.
    #[serde(rename="improvementFactor")]
    
    pub improvement_factor: Option<f64>,
}

impl client::Part for IndexAdvice {}


/// A message representing a (sparse) collection of hot keys for specific key buckets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexedHotKey {
    /// A (sparse) mapping from key bucket index to the index of the specific hot row key for that key bucket. The index of the hot row key can be translated to the actual row key via the ScanData.VisualizationData.indexed_keys repeated field.
    #[serde(rename="sparseHotKeys")]
    
    pub sparse_hot_keys: Option<HashMap<String, i32>>,
}

impl client::Part for IndexedHotKey {}


/// A message representing a (sparse) collection of KeyRangeInfos for specific key buckets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexedKeyRangeInfos {
    /// A (sparse) mapping from key bucket index to the KeyRangeInfos for that key bucket.
    #[serde(rename="keyRangeInfos")]
    
    pub key_range_infos: Option<HashMap<String, KeyRangeInfos>>,
}

impl client::Part for IndexedKeyRangeInfos {}


/// An isolated set of Cloud Spanner resources on which databases can be hosted.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances get projects](ProjectInstanceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// Optional. The autoscaling configuration. Autoscaling is enabled if this field is set. When autoscaling is enabled, node_count and processing_units are treated as OUTPUT_ONLY fields and reflect the current compute capacity allocated to the instance.
    #[serde(rename="autoscalingConfig")]
    
    pub autoscaling_config: Option<AutoscalingConfig>,
    /// Required. The name of the instance's configuration. Values are of the form `projects//instanceConfigs/`. See also InstanceConfig and ListInstanceConfigs.
    
    pub config: Option<String>,
    /// Output only. The time at which the instance was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The descriptive name for this instance as it appears in UIs. Must be unique per project and between 4 and 30 characters in length.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Deprecated. This field is not populated.
    #[serde(rename="endpointUris")]
    
    pub endpoint_uris: Option<Vec<String>>,
    /// Free instance metadata. Only populated for free instances.
    #[serde(rename="freeInstanceMetadata")]
    
    pub free_instance_metadata: Option<FreeInstanceMetadata>,
    /// The `InstanceType` of the current instance.
    #[serde(rename="instanceType")]
    
    pub instance_type: Option<InstanceInstanceTypeEnum>,
    /// Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. Cloud Labels can be used to filter collections of resources. They can be used to control how resource metrics are aggregated. And they can be used as arguments to policy management rules (e.g. route, firewall, load balancing, etc.). * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `[a-z0-9_-]{0,63}`. * No more than 64 labels can be associated with a given resource. See https://goo.gl/xmQnxf for more information on and examples of labels. If you plan to use labels in your own code, please note that additional characters may be allowed in the future. And so you are advised to use an internal label representation, such as JSON, which doesn't rely upon specific characters being disallowed. For example, representing labels as the string: name + "_" + value would prove problematic if we were to allow "_" in a future release.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. A unique identifier for the instance, which cannot be changed after the instance is created. Values are of the form `projects//instances/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length.
    
    pub name: Option<String>,
    /// The number of nodes allocated to this instance. At most one of either node_count or processing_units should be present in the message. Users can set the node_count field to specify the target number of nodes allocated to the instance. This may be zero in API responses for instances that are not yet in state `READY`. See [the documentation](https://cloud.google.com/spanner/docs/compute-capacity) for more information about nodes and processing units.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
    /// The number of processing units allocated to this instance. At most one of processing_units or node_count should be present in the message. Users can set the processing_units field to specify the target number of processing units allocated to the instance. This may be zero in API responses for instances that are not yet in state `READY`. See [the documentation](https://cloud.google.com/spanner/docs/compute-capacity) for more information about nodes and processing units.
    #[serde(rename="processingUnits")]
    
    pub processing_units: Option<i32>,
    /// Output only. The current instance state. For CreateInstance, the state must be either omitted or set to `CREATING`. For UpdateInstance, the state must be either omitted or set to `READY`.
    
    pub state: Option<InstanceStateEnum>,
    /// Output only. The time at which the instance was most recently updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Instance {}


/// A possible configuration for a Cloud Spanner instance. Configurations define the geographic placement of nodes and their replication.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs get projects](ProjectInstanceConfigGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceConfig {
    /// Base configuration name, e.g. projects//instanceConfigs/nam3, based on which this configuration is created. Only set for user managed configurations. `base_config` must refer to a configuration of type GOOGLE_MANAGED in the same project as this configuration.
    #[serde(rename="baseConfig")]
    
    pub base_config: Option<String>,
    /// Output only. Whether this instance config is a Google or User Managed Configuration.
    #[serde(rename="configType")]
    
    pub config_type: Option<InstanceConfigConfigTypeEnum>,
    /// The name of this instance configuration as it appears in UIs.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a instance config from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform instance config updates in order to avoid race conditions: An etag is returned in the response which contains instance configs, and systems are expected to put that etag in the request to update instance config to ensure that their change will be applied to the same version of the instance config. If no etag is provided in the call to update instance config, then the existing instance config is overwritten blindly.
    
    pub etag: Option<String>,
    /// Output only. Describes whether free instances are available to be created in this instance config.
    #[serde(rename="freeInstanceAvailability")]
    
    pub free_instance_availability: Option<InstanceConfigFreeInstanceAvailabilityEnum>,
    /// Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. Cloud Labels can be used to filter collections of resources. They can be used to control how resource metrics are aggregated. And they can be used as arguments to policy management rules (e.g. route, firewall, load balancing, etc.). * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `[a-z0-9_-]{0,63}`. * No more than 64 labels can be associated with a given resource. See https://goo.gl/xmQnxf for more information on and examples of labels. If you plan to use labels in your own code, please note that additional characters may be allowed in the future. Therefore, you are advised to use an internal label representation, such as JSON, which doesn't rely upon specific characters being disallowed. For example, representing labels as the string: name + "_" + value would prove problematic if we were to allow "_" in a future release.
    
    pub labels: Option<HashMap<String, String>>,
    /// Allowed values of the "default_leader" schema option for databases in instances that use this instance configuration.
    #[serde(rename="leaderOptions")]
    
    pub leader_options: Option<Vec<String>>,
    /// A unique identifier for the instance configuration. Values are of the form `projects//instanceConfigs/a-z*`.
    
    pub name: Option<String>,
    /// Output only. The available optional replicas to choose from for user managed configurations. Populated for Google managed configurations.
    #[serde(rename="optionalReplicas")]
    
    pub optional_replicas: Option<Vec<ReplicaInfo>>,
    /// Output only. If true, the instance config is being created or updated. If false, there are no ongoing operations for the instance config.
    
    pub reconciling: Option<bool>,
    /// The geographic placement of nodes in this instance configuration and their replication properties.
    
    pub replicas: Option<Vec<ReplicaInfo>>,
    /// Output only. The current instance config state. Applicable only for USER_MANAGED configs.
    
    pub state: Option<InstanceConfigStateEnum>,
    /// Output only. The storage limit in bytes per processing unit.
    #[serde(rename="storageLimitPerProcessingUnit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_limit_per_processing_unit: Option<i64>,
}

impl client::ResponseResult for InstanceConfig {}


/// An isolated set of Cloud Spanner resources that databases can define placements on.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances instance partitions get projects](ProjectInstanceInstancePartitionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstancePartition {
    /// Required. The name of the instance partition's configuration. Values are of the form `projects//instanceConfigs/`. See also InstanceConfig and ListInstanceConfigs.
    
    pub config: Option<String>,
    /// Output only. The time at which the instance partition was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The descriptive name for this instance partition as it appears in UIs. Must be unique per project and between 4 and 30 characters in length.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Used for optimistic concurrency control as a way to help prevent simultaneous updates of a instance partition from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform instance partition updates in order to avoid race conditions: An etag is returned in the response which contains instance partitions, and systems are expected to put that etag in the request to update instance partitions to ensure that their change will be applied to the same version of the instance partition. If no etag is provided in the call to update instance partition, then the existing instance partition is overwritten blindly.
    
    pub etag: Option<String>,
    /// Required. A unique identifier for the instance partition. Values are of the form `projects//instances//instancePartitions/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length. An instance partition's name cannot be changed after the instance partition is created.
    
    pub name: Option<String>,
    /// The number of nodes allocated to this instance partition. Users can set the node_count field to specify the target number of nodes allocated to the instance partition. This may be zero in API responses for instance partitions that are not yet in state `READY`.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
    /// The number of processing units allocated to this instance partition. Users can set the processing_units field to specify the target number of processing units allocated to the instance partition. This may be zero in API responses for instance partitions that are not yet in state `READY`.
    #[serde(rename="processingUnits")]
    
    pub processing_units: Option<i32>,
    /// Output only. The names of the backups that reference this instance partition. Referencing backups should share the parent instance. The existence of any referencing backup prevents the instance partition from being deleted.
    #[serde(rename="referencingBackups")]
    
    pub referencing_backups: Option<Vec<String>>,
    /// Output only. The names of the databases that reference this instance partition. Referencing databases should share the parent instance. The existence of any referencing database prevents the instance partition from being deleted.
    #[serde(rename="referencingDatabases")]
    
    pub referencing_databases: Option<Vec<String>>,
    /// Output only. The current instance partition state.
    
    pub state: Option<InstancePartitionStateEnum>,
    /// Output only. The time at which the instance partition was most recently updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for InstancePartition {}


/// KeyRange represents a range of rows in a table or index. A range has a start key and an end key. These keys can be open or closed, indicating if the range includes rows with that key. Keys are represented by lists, where the ith value in the list corresponds to the ith component of the table or index primary key. Individual values are encoded as described here. For example, consider the following table definition: CREATE TABLE UserEvents ( UserName STRING(MAX), EventDate STRING(10) ) PRIMARY KEY(UserName, EventDate); The following keys name rows in this table: "Bob", "2014-09-23" Since the `UserEvents` table's `PRIMARY KEY` clause names two columns, each `UserEvents` key has two elements; the first is the `UserName`, and the second is the `EventDate`. Key ranges with multiple components are interpreted lexicographically by component using the table or index key's declared sort order. For example, the following range returns all events for user `"Bob"` that occurred in the year 2015: "start_closed": ["Bob", "2015-01-01"] "end_closed": ["Bob", "2015-12-31"] Start and end keys can omit trailing key components. This affects the inclusion and exclusion of rows that exactly match the provided key components: if the key is closed, then rows that exactly match the provided components are included; if the key is open, then rows that exactly match are not included. For example, the following range includes all events for `"Bob"` that occurred during and after the year 2000: "start_closed": ["Bob", "2000-01-01"] "end_closed": ["Bob"] The next example retrieves all events for `"Bob"`: "start_closed": ["Bob"] "end_closed": ["Bob"] To retrieve events before the year 2000: "start_closed": ["Bob"] "end_open": ["Bob", "2000-01-01"] The following range includes all rows in the table: "start_closed": [] "end_closed": [] This range returns all users whose `UserName` begins with any character from A to C: "start_closed": ["A"] "end_open": ["D"] This range returns all users whose `UserName` begins with B: "start_closed": ["B"] "end_open": ["C"] Key ranges honor column sort order. For example, suppose a table is defined as follows: CREATE TABLE DescendingSortedTable { Key INT64, ... ) PRIMARY KEY(Key DESC); The following range retrieves all rows with key values between 1 and 100 inclusive: "start_closed": ["100"] "end_closed": ["1"] Note that 100 is passed as the start, and 1 is passed as the end, because `Key` is a descending column in the schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyRange {
    /// If the end is closed, then the range includes all rows whose first `len(end_closed)` key columns exactly match `end_closed`.
    #[serde(rename="endClosed")]
    
    pub end_closed: Option<Vec<json::Value>>,
    /// If the end is open, then the range excludes rows whose first `len(end_open)` key columns exactly match `end_open`.
    #[serde(rename="endOpen")]
    
    pub end_open: Option<Vec<json::Value>>,
    /// If the start is closed, then the range includes all rows whose first `len(start_closed)` key columns exactly match `start_closed`.
    #[serde(rename="startClosed")]
    
    pub start_closed: Option<Vec<json::Value>>,
    /// If the start is open, then the range excludes rows whose first `len(start_open)` key columns exactly match `start_open`.
    #[serde(rename="startOpen")]
    
    pub start_open: Option<Vec<json::Value>>,
}

impl client::Part for KeyRange {}


/// A message representing information for a key range (possibly one key).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyRangeInfo {
    /// The list of context values for this key range.
    #[serde(rename="contextValues")]
    
    pub context_values: Option<Vec<ContextValue>>,
    /// The index of the end key in indexed_keys.
    #[serde(rename="endKeyIndex")]
    
    pub end_key_index: Option<i32>,
    /// Information about this key range, for all metrics.
    
    pub info: Option<LocalizedString>,
    /// The number of keys this range covers.
    #[serde(rename="keysCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub keys_count: Option<i64>,
    /// The name of the metric. e.g. "latency".
    
    pub metric: Option<LocalizedString>,
    /// The index of the start key in indexed_keys.
    #[serde(rename="startKeyIndex")]
    
    pub start_key_index: Option<i32>,
    /// The time offset. This is the time since the start of the time interval.
    #[serde(rename="timeOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub time_offset: Option<client::chrono::Duration>,
    /// The unit of the metric. This is an unstructured field and will be mapped as is to the user.
    
    pub unit: Option<LocalizedString>,
    /// The value of the metric.
    
    pub value: Option<f32>,
}

impl client::Part for KeyRangeInfo {}


/// A message representing a list of specific information for multiple key ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyRangeInfos {
    /// The list individual KeyRangeInfos.
    
    pub infos: Option<Vec<KeyRangeInfo>>,
    /// The total size of the list of all KeyRangeInfos. This may be larger than the number of repeated messages above. If that is the case, this number may be used to determine how many are not being shown.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::Part for KeyRangeInfos {}


/// `KeySet` defines a collection of Cloud Spanner keys and/or key ranges. All the keys are expected to be in the same table or index. The keys need not be sorted in any particular way. If the same key is specified multiple times in the set (for example if two ranges, two keys, or a key and a range overlap), Cloud Spanner behaves as if the key were only specified once.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeySet {
    /// For convenience `all` can be set to `true` to indicate that this `KeySet` matches all keys in the table or index. Note that any keys specified in `keys` or `ranges` are only yielded once.
    
    pub all: Option<bool>,
    /// A list of specific keys. Entries in `keys` should have exactly as many elements as there are columns in the primary or index key with which this `KeySet` is used. Individual key values are encoded as described here.
    
    pub keys: Option<Vec<Vec<json::Value>>>,
    /// A list of key ranges. See KeyRange for more information about key range specifications.
    
    pub ranges: Option<Vec<KeyRange>>,
}

impl client::Part for KeySet {}


/// The response for ListBackupOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backup operations list projects](ProjectInstanceBackupOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupOperationsResponse {
    /// `next_page_token` can be sent in a subsequent ListBackupOperations call to fetch more of the matching metadata.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching backup long-running operations. Each operation's name will be prefixed by the backup's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that are pending or have completed/failed/canceled within the last 7 days. Operations returned are ordered by `operation.metadata.value.progress.start_time` in descending order starting from the most recently started operation.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListBackupOperationsResponse {}


/// The response for ListBackups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups list projects](ProjectInstanceBackupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    /// The list of matching backups. Backups returned are ordered by `create_time` in descending order, starting from the most recent `create_time`.
    
    pub backups: Option<Vec<Backup>>,
    /// `next_page_token` can be sent in a subsequent ListBackups call to fetch more of the matching backups.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBackupsResponse {}


/// The response for ListDatabaseOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances database operations list projects](ProjectInstanceDatabaseOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDatabaseOperationsResponse {
    /// `next_page_token` can be sent in a subsequent ListDatabaseOperations call to fetch more of the matching metadata.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching database long-running operations. Each operation's name will be prefixed by the database's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListDatabaseOperationsResponse {}


/// The response for ListDatabaseRoles.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases database roles list projects](ProjectInstanceDatabaseDatabaseRoleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDatabaseRolesResponse {
    /// Database roles that matched the request.
    #[serde(rename="databaseRoles")]
    
    pub database_roles: Option<Vec<DatabaseRole>>,
    /// `next_page_token` can be sent in a subsequent ListDatabaseRoles call to fetch more of the matching roles.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDatabaseRolesResponse {}


/// The response for ListDatabases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases list projects](ProjectInstanceDatabaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDatabasesResponse {
    /// Databases that matched the request.
    
    pub databases: Option<Vec<Database>>,
    /// `next_page_token` can be sent in a subsequent ListDatabases call to fetch more of the matching databases.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDatabasesResponse {}


/// The response for ListInstanceConfigOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance config operations list projects](ProjectInstanceConfigOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstanceConfigOperationsResponse {
    /// `next_page_token` can be sent in a subsequent ListInstanceConfigOperations call to fetch more of the matching metadata.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching instance config long-running operations. Each operation's name will be prefixed by the instance config's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListInstanceConfigOperationsResponse {}


/// The response for ListInstanceConfigs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs list projects](ProjectInstanceConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstanceConfigsResponse {
    /// The list of requested instance configurations.
    #[serde(rename="instanceConfigs")]
    
    pub instance_configs: Option<Vec<InstanceConfig>>,
    /// `next_page_token` can be sent in a subsequent ListInstanceConfigs call to fetch more of the matching instance configurations.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListInstanceConfigsResponse {}


/// The response for ListInstancePartitionOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances instance partition operations list projects](ProjectInstanceInstancePartitionOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancePartitionOperationsResponse {
    /// `next_page_token` can be sent in a subsequent ListInstancePartitionOperations call to fetch more of the matching metadata.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching instance partition long-running operations. Each operation's name will be prefixed by the instance partition's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata.
    
    pub operations: Option<Vec<Operation>>,
    /// The list of unreachable instance partitions. It includes the names of instance partitions whose operation metadata could not be retrieved within instance_partition_deadline.
    #[serde(rename="unreachableInstancePartitions")]
    
    pub unreachable_instance_partitions: Option<Vec<String>>,
}

impl client::ResponseResult for ListInstancePartitionOperationsResponse {}


/// The response for ListInstancePartitions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances instance partitions list projects](ProjectInstanceInstancePartitionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancePartitionsResponse {
    /// The list of requested instancePartitions.
    #[serde(rename="instancePartitions")]
    
    pub instance_partitions: Option<Vec<InstancePartition>>,
    /// `next_page_token` can be sent in a subsequent ListInstancePartitions call to fetch more of the matching instance partitions.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of unreachable instance partitions. It includes the names of instance partitions whose metadata could not be retrieved within instance_partition_deadline.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListInstancePartitionsResponse {}


/// The response for ListInstances.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances list projects](ProjectInstanceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInstancesResponse {
    /// The list of requested instances.
    
    pub instances: Option<Vec<Instance>>,
    /// `next_page_token` can be sent in a subsequent ListInstances call to fetch more of the matching instances.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of unreachable instances. It includes the names of instances whose metadata could not be retrieved within instance_deadline.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListInstancesResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs operations list projects](ProjectInstanceConfigOperationListCall1) (response)
/// * [instance configs ssd caches operations list projects](ProjectInstanceConfigSsdCachOperationListCall) (response)
/// * [instances backups operations list projects](ProjectInstanceBackupOperationListCall1) (response)
/// * [instances databases operations list projects](ProjectInstanceDatabaseOperationListCall1) (response)
/// * [instances instance partitions operations list projects](ProjectInstanceInstancePartitionOperationListCall1) (response)
/// * [instances operations list projects](ProjectInstanceOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response method from the ListScans method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list scans](ScanListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListScansResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Available scans based on the list query parameters.
    
    pub scans: Option<Vec<Scan>>,
}

impl client::ResponseResult for ListScansResponse {}


/// The response for ListSessions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions list projects](ProjectInstanceDatabaseSessionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSessionsResponse {
    /// `next_page_token` can be sent in a subsequent ListSessions call to fetch more of the matching sessions.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of requested sessions.
    
    pub sessions: Option<Vec<Session>>,
}

impl client::ResponseResult for ListSessionsResponse {}


/// A message representing a user-facing string whose value may need to be translated before being displayed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedString {
    /// A map of arguments used when creating the localized message. Keys represent parameter names which may be used by the localized version when substituting dynamic values.
    
    pub args: Option<HashMap<String, String>>,
    /// The canonical English version of this message. If no token is provided or the front-end has no message associated with the token, this text will be displayed as-is.
    
    pub message: Option<String>,
    /// The token identifying the message, e.g. 'METRIC_READ_CPU'. This should be unique within the service.
    
    pub token: Option<String>,
}

impl client::Part for LocalizedString {}


/// A message representing the actual monitoring data, values for each key bucket over time, of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// The aggregation function used to aggregate each key bucket
    
    pub aggregation: Option<MetricAggregationEnum>,
    /// The category of the metric, e.g. "Activity", "Alerts", "Reads", etc.
    
    pub category: Option<LocalizedString>,
    /// The references to numerator and denominator metrics for a derived metric.
    
    pub derived: Option<DerivedMetric>,
    /// The displayed label of the metric.
    #[serde(rename="displayLabel")]
    
    pub display_label: Option<LocalizedString>,
    /// Whether the metric has any non-zero data.
    #[serde(rename="hasNonzeroData")]
    
    pub has_nonzero_data: Option<bool>,
    /// The value that is considered hot for the metric. On a per metric basis hotness signals high utilization and something that might potentially be a cause for concern by the end user. hot_value is used to calibrate and scale visual color scales.
    #[serde(rename="hotValue")]
    
    pub hot_value: Option<f32>,
    /// The (sparse) mapping from time index to an IndexedHotKey message, representing those time intervals for which there are hot keys.
    #[serde(rename="indexedHotKeys")]
    
    pub indexed_hot_keys: Option<HashMap<String, IndexedHotKey>>,
    /// The (sparse) mapping from time interval index to an IndexedKeyRangeInfos message, representing those time intervals for which there are informational messages concerning key ranges.
    #[serde(rename="indexedKeyRangeInfos")]
    
    pub indexed_key_range_infos: Option<HashMap<String, IndexedKeyRangeInfos>>,
    /// Information about the metric.
    
    pub info: Option<LocalizedString>,
    /// The data for the metric as a matrix.
    
    pub matrix: Option<MetricMatrix>,
    /// The unit of the metric.
    
    pub unit: Option<LocalizedString>,
    /// Whether the metric is visible to the end user.
    
    pub visible: Option<bool>,
}

impl client::Part for Metric {}


/// A message representing a matrix of floats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricMatrix {
    /// The rows of the matrix.
    
    pub rows: Option<Vec<MetricMatrixRow>>,
}

impl client::Part for MetricMatrix {}


/// A message representing a row of a matrix of floats.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricMatrixRow {
    /// The columns of the row.
    
    pub cols: Option<Vec<f32>>,
}

impl client::Part for MetricMatrixRow {}


/// The request for MoveInstance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances move projects](ProjectInstanceMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveInstanceRequest {
    /// Required. The target instance config for the instance to move. Values are of the form `projects//instanceConfigs/`.
    #[serde(rename="targetConfig")]
    
    pub target_config: Option<String>,
}

impl client::RequestValue for MoveInstanceRequest {}


/// A modification to one or more Cloud Spanner rows. Mutations can be applied to a Cloud Spanner database by sending them in a Commit call.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Mutation {
    /// Delete rows from a table. Succeeds whether or not the named rows were present.
    
    pub delete: Option<Delete>,
    /// Insert new rows in a table. If any of the rows already exist, the write or transaction fails with error `ALREADY_EXISTS`.
    
    pub insert: Option<Write>,
    /// Like insert, except that if the row already exists, then its column values are overwritten with the ones provided. Any column values not explicitly written are preserved. When using insert_or_update, just as when using insert, all `NOT NULL` columns in the table must be given a value. This holds true even when the row already exists and will therefore actually be updated.
    #[serde(rename="insertOrUpdate")]
    
    pub insert_or_update: Option<Write>,
    /// Like insert, except that if the row already exists, it is deleted, and the column values provided are inserted instead. Unlike insert_or_update, this means any values not explicitly written become `NULL`. In an interleaved table, if you create the child table with the `ON DELETE CASCADE` annotation, then replacing a parent row also deletes the child rows. Otherwise, you must delete the child rows before you replace the parent row.
    
    pub replace: Option<Write>,
    /// Update existing rows in a table. If any of the rows does not already exist, the transaction fails with error `NOT_FOUND`.
    
    pub update: Option<Write>,
}

impl client::Part for Mutation {}


/// A group of mutations to be committed together. Related mutations should be placed in a group. For example, two mutations inserting rows with the same primary key prefix in both parent and child tables are related.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MutationGroup {
    /// Required. The mutations in this group.
    
    pub mutations: Option<Vec<Mutation>>,
}

impl client::Part for MutationGroup {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs operations get projects](ProjectInstanceConfigOperationGetCall) (response)
/// * [instance configs ssd caches operations get projects](ProjectInstanceConfigSsdCachOperationGetCall) (response)
/// * [instance configs create projects](ProjectInstanceConfigCreateCall) (response)
/// * [instance configs patch projects](ProjectInstanceConfigPatchCall) (response)
/// * [instances backups operations get projects](ProjectInstanceBackupOperationGetCall) (response)
/// * [instances backups copy projects](ProjectInstanceBackupCopyCall) (response)
/// * [instances backups create projects](ProjectInstanceBackupCreateCall) (response)
/// * [instances databases operations get projects](ProjectInstanceDatabaseOperationGetCall) (response)
/// * [instances databases create projects](ProjectInstanceDatabaseCreateCall) (response)
/// * [instances databases patch projects](ProjectInstanceDatabasePatchCall) (response)
/// * [instances databases restore projects](ProjectInstanceDatabaseRestoreCall) (response)
/// * [instances databases update ddl projects](ProjectInstanceDatabaseUpdateDdlCall) (response)
/// * [instances instance partitions operations get projects](ProjectInstanceInstancePartitionOperationGetCall) (response)
/// * [instances instance partitions create projects](ProjectInstanceInstancePartitionCreateCall) (response)
/// * [instances instance partitions patch projects](ProjectInstanceInstancePartitionPatchCall) (response)
/// * [instances operations get projects](ProjectInstanceOperationGetCall) (response)
/// * [instances create projects](ProjectInstanceCreateCall) (response)
/// * [instances move projects](ProjectInstanceMoveCall) (response)
/// * [instances patch projects](ProjectInstancePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Partial results from a streaming read or SQL query. Streaming reads and SQL queries better tolerate large result sets, large rows, and large values, but are a little trickier to consume.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions execute streaming sql projects](ProjectInstanceDatabaseSessionExecuteStreamingSqlCall) (response)
/// * [instances databases sessions streaming read projects](ProjectInstanceDatabaseSessionStreamingReadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartialResultSet {
    /// If true, then the final value in values is chunked, and must be combined with more values from subsequent `PartialResultSet`s to obtain a complete field value.
    #[serde(rename="chunkedValue")]
    
    pub chunked_value: Option<bool>,
    /// Metadata about the result set, such as row type information. Only present in the first response.
    
    pub metadata: Option<ResultSetMetadata>,
    /// Streaming calls might be interrupted for a variety of reasons, such as TCP connection loss. If this occurs, the stream of results can be resumed by re-sending the original request and including `resume_token`. Note that executing any other transaction in the same session invalidates the token.
    #[serde(rename="resumeToken")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub resume_token: Option<Vec<u8>>,
    /// Query plan and execution statistics for the statement that produced this streaming result set. These can be requested by setting ExecuteSqlRequest.query_mode and are sent only once with the last response in the stream. This field will also be present in the last response for DML statements.
    
    pub stats: Option<ResultSetStats>,
    /// A streamed result set consists of a stream of values, which might be split into many `PartialResultSet` messages to accommodate large rows and/or large values. Every N complete values defines a row, where N is equal to the number of entries in metadata.row_type.fields. Most values are encoded based on type as described here. It is possible that the last value in values is "chunked", meaning that the rest of the value is sent in subsequent `PartialResultSet`(s). This is denoted by the chunked_value field. Two or more chunked values can be merged to form a complete value as follows: * `bool/number/null`: cannot be chunked * `string`: concatenate the strings * `list`: concatenate the lists. If the last element in a list is a `string`, `list`, or `object`, merge it with the first element in the next list by applying these rules recursively. * `object`: concatenate the (field name, field value) pairs. If a field name is duplicated, then apply these rules recursively to merge the field values. Some examples of merging: # Strings are concatenated. "foo", "bar" => "foobar" # Lists of non-strings are concatenated. [2, 3], [4] => [2, 3, 4] # Lists are concatenated, but the last and first elements are merged # because they are strings. ["a", "b"], ["c", "d"] => ["a", "bc", "d"] # Lists are concatenated, but the last and first elements are merged # because they are lists. Recursively, the last and first elements # of the inner lists are merged because they are strings. ["a", ["b", "c"]], [["d"], "e"] => ["a", ["b", "cd"], "e"] # Non-overlapping object fields are combined. {"a": "1"}, {"b": "2"} => {"a": "1", "b": 2"} # Overlapping object fields are merged. {"a": "1"}, {"a": "2"} => {"a": "12"} # Examples of merging objects containing lists of strings. {"a": ["1"]}, {"a": ["2"]} => {"a": ["12"]} For a more complete example, suppose a streaming SQL query is yielding a result set whose rows contain a single string field. The following `PartialResultSet`s might be yielded: { "metadata": { ... } "values": ["Hello", "W"] "chunked_value": true "resume_token": "Af65..." } { "values": ["orl"] "chunked_value": true } { "values": ["d"] "resume_token": "Zx1B..." } This sequence of `PartialResultSet`s encodes two rows, one containing the field value `"Hello"`, and a second containing the field value `"World" = "W" + "orl" + "d"`. Not all `PartialResultSet`s contain a `resume_token`. Execution can only be resumed from a previously yielded `resume_token`. For the above sequence of `PartialResultSet`s, resuming the query with `"resume_token": "Af65..."` will yield results from the `PartialResultSet` with value `["orl"]`.
    
    pub values: Option<Vec<json::Value>>,
}

impl client::ResponseResult for PartialResultSet {}


/// Information returned for each partition returned in a PartitionResponse.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Partition {
    /// This token can be passed to Read, StreamingRead, ExecuteSql, or ExecuteStreamingSql requests to restrict the results to those identified by this partition token.
    #[serde(rename="partitionToken")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub partition_token: Option<Vec<u8>>,
}

impl client::Part for Partition {}


/// Options for a PartitionQueryRequest and PartitionReadRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionOptions {
    /// **Note:** This hint is currently ignored by PartitionQuery and PartitionRead requests. The desired maximum number of partitions to return. For example, this may be set to the number of workers available. The default for this option is currently 10,000. The maximum value is currently 200,000. This is only a hint. The actual number of partitions returned may be smaller or larger than this maximum count request.
    #[serde(rename="maxPartitions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_partitions: Option<i64>,
    /// **Note:** This hint is currently ignored by PartitionQuery and PartitionRead requests. The desired data size for each partition generated. The default for this option is currently 1 GiB. This is only a hint. The actual size of each partition may be smaller or larger than this size request.
    #[serde(rename="partitionSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub partition_size_bytes: Option<i64>,
}

impl client::Part for PartitionOptions {}


/// The request for PartitionQuery
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions partition query projects](ProjectInstanceDatabaseSessionPartitionQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionQueryRequest {
    /// It is not always possible for Cloud Spanner to infer the right SQL type from a JSON value. For example, values of type `BYTES` and values of type `STRING` both appear in params as JSON strings. In these cases, `param_types` can be used to specify the exact SQL type for some or all of the SQL query parameters. See the definition of Type for more information about SQL types.
    #[serde(rename="paramTypes")]
    
    pub param_types: Option<HashMap<String, Type>>,
    /// Parameter names and values that bind to placeholders in the SQL string. A parameter placeholder consists of the `@` character followed by the parameter name (for example, `@firstName`). Parameter names can contain letters, numbers, and underscores. Parameters can appear anywhere that a literal value is expected. The same parameter name can be used more than once, for example: `"WHERE id > @msg_id AND id < @msg_id + 100"` It is an error to execute a SQL statement with unbound parameters.
    
    pub params: Option<HashMap<String, json::Value>>,
    /// Additional options that affect how many partitions are created.
    #[serde(rename="partitionOptions")]
    
    pub partition_options: Option<PartitionOptions>,
    /// Required. The query request to generate partitions for. The request will fail if the query is not root partitionable. For a query to be root partitionable, it needs to satisfy a few conditions. For example, if the query execution plan contains a distributed union operator, then it must be the first operator in the plan. For more information about other conditions, see [Read data in parallel](https://cloud.google.com/spanner/docs/reads#read_data_in_parallel). The query request must not contain DML commands, such as INSERT, UPDATE, or DELETE. Use ExecuteStreamingSql with a PartitionedDml transaction for large, partition-friendly DML operations.
    
    pub sql: Option<String>,
    /// Read only snapshot transactions are supported, read/write and single use transactions are not.
    
    pub transaction: Option<TransactionSelector>,
}

impl client::RequestValue for PartitionQueryRequest {}


/// The request for PartitionRead
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions partition read projects](ProjectInstanceDatabaseSessionPartitionReadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionReadRequest {
    /// The columns of table to be returned for each row matching this request.
    
    pub columns: Option<Vec<String>>,
    /// If non-empty, the name of an index on table. This index is used instead of the table primary key when interpreting key_set and sorting result rows. See key_set for further information.
    
    pub index: Option<String>,
    /// Required. `key_set` identifies the rows to be yielded. `key_set` names the primary keys of the rows in table to be yielded, unless index is present. If index is present, then key_set instead names index keys in index. It is not an error for the `key_set` to name rows that do not exist in the database. Read yields nothing for nonexistent rows.
    #[serde(rename="keySet")]
    
    pub key_set: Option<KeySet>,
    /// Additional options that affect how many partitions are created.
    #[serde(rename="partitionOptions")]
    
    pub partition_options: Option<PartitionOptions>,
    /// Required. The name of the table in the database to be read.
    
    pub table: Option<String>,
    /// Read only snapshot transactions are supported, read/write and single use transactions are not.
    
    pub transaction: Option<TransactionSelector>,
}

impl client::RequestValue for PartitionReadRequest {}


/// The response for PartitionQuery or PartitionRead
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions partition query projects](ProjectInstanceDatabaseSessionPartitionQueryCall) (response)
/// * [instances databases sessions partition read projects](ProjectInstanceDatabaseSessionPartitionReadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionResponse {
    /// Partitions created by this request.
    
    pub partitions: Option<Vec<Partition>>,
    /// Transaction created by this request.
    
    pub transaction: Option<Transaction>,
}

impl client::ResponseResult for PartitionResponse {}


/// Message type to initiate a Partitioned DML transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PartitionedDml { _never_set: Option<bool> }

impl client::Part for PartitionedDml {}


/// Node information for nodes appearing in a QueryPlan.plan_nodes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlanNode {
    /// List of child node `index`es and their relationship to this parent.
    #[serde(rename="childLinks")]
    
    pub child_links: Option<Vec<ChildLink>>,
    /// The display name for the node.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The execution statistics associated with the node, contained in a group of key-value pairs. Only present if the plan was returned as a result of a profile query. For example, number of executions, number of rows/time per execution etc.
    #[serde(rename="executionStats")]
    
    pub execution_stats: Option<HashMap<String, json::Value>>,
    /// The `PlanNode`'s index in node list.
    
    pub index: Option<i32>,
    /// Used to determine the type of node. May be needed for visualizing different kinds of nodes differently. For example, If the node is a SCALAR node, it will have a condensed representation which can be used to directly embed a description of the node in its parent.
    
    pub kind: Option<PlanNodeKindEnum>,
    /// Attributes relevant to the node contained in a group of key-value pairs. For example, a Parameter Reference node could have the following information in its metadata: { "parameter_reference": "param1", "parameter_type": "array" }
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Condensed representation for SCALAR nodes.
    #[serde(rename="shortRepresentation")]
    
    pub short_representation: Option<ShortRepresentation>,
}

impl client::Part for PlanNode {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups get iam policy projects](ProjectInstanceBackupGetIamPolicyCall) (response)
/// * [instances backups set iam policy projects](ProjectInstanceBackupSetIamPolicyCall) (response)
/// * [instances databases get iam policy projects](ProjectInstanceDatabaseGetIamPolicyCall) (response)
/// * [instances databases set iam policy projects](ProjectInstanceDatabaseSetIamPolicyCall) (response)
/// * [instances get iam policy projects](ProjectInstanceGetIamPolicyCall) (response)
/// * [instances set iam policy projects](ProjectInstanceSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// A message representing a key prefix node in the key prefix hierarchy. for eg. Bigtable keyspaces are lexicographically ordered mappings of keys to values. Keys often have a shared prefix structure where users use the keys to organize data. Eg ///employee In this case Keysight will possibly use one node for a company and reuse it for all employees that fall under the company. Doing so improves legibility in the UI.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrefixNode {
    /// Whether this corresponds to a data_source name.
    #[serde(rename="dataSourceNode")]
    
    pub data_source_node: Option<bool>,
    /// The depth in the prefix hierarchy.
    
    pub depth: Option<i32>,
    /// The index of the end key bucket of the range that this node spans.
    #[serde(rename="endIndex")]
    
    pub end_index: Option<i32>,
    /// The index of the start key bucket of the range that this node spans.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The string represented by the prefix node.
    
    pub word: Option<String>,
}

impl client::Part for PrefixNode {}


/// Output of query advisor analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAdvisorResult {
    /// Optional. Index Recommendation for a query. This is an optional field and the recommendation will only be available when the recommendation guarantees significant improvement in query performance.
    #[serde(rename="indexAdvice")]
    
    pub index_advice: Option<Vec<IndexAdvice>>,
}

impl client::Part for QueryAdvisorResult {}


/// Query optimizer configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryOptions {
    /// An option to control the selection of optimizer statistics package. This parameter allows individual queries to use a different query optimizer statistics package. Specifying `latest` as a value instructs Cloud Spanner to use the latest generated statistics package. If not specified, Cloud Spanner uses the statistics package set at the database level options, or the latest package if the database option is not set. The statistics package requested by the query has to be exempt from garbage collection. This can be achieved with the following DDL statement: ``` ALTER STATISTICS SET OPTIONS (allow_gc=false) ``` The list of available statistics packages can be queried from `INFORMATION_SCHEMA.SPANNER_STATISTICS`. Executing a SQL statement with an invalid optimizer statistics package or with a statistics package that allows garbage collection fails with an `INVALID_ARGUMENT` error.
    #[serde(rename="optimizerStatisticsPackage")]
    
    pub optimizer_statistics_package: Option<String>,
    /// An option to control the selection of optimizer version. This parameter allows individual queries to pick different query optimizer versions. Specifying `latest` as a value instructs Cloud Spanner to use the latest supported query optimizer version. If not specified, Cloud Spanner uses the optimizer version set at the database level options. Any other positive integer (from the list of supported optimizer versions) overrides the default optimizer version for query execution. The list of supported optimizer versions can be queried from SPANNER_SYS.SUPPORTED_OPTIMIZER_VERSIONS. Executing a SQL statement with an invalid optimizer version fails with an `INVALID_ARGUMENT` error. See https://cloud.google.com/spanner/docs/query-optimizer/manage-query-optimizer for more information on managing the query optimizer. The `optimizer_version` statement hint has precedence over this setting.
    #[serde(rename="optimizerVersion")]
    
    pub optimizer_version: Option<String>,
}

impl client::Part for QueryOptions {}


/// Contains an ordered list of nodes appearing in the query plan.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryPlan {
    /// The nodes in the query plan. Plan nodes are returned in pre-order starting with the plan root. Each PlanNode's `id` corresponds to its index in `plan_nodes`.
    #[serde(rename="planNodes")]
    
    pub plan_nodes: Option<Vec<PlanNode>>,
    /// Optional. The advices/recommendations for a query. Currently this field will be serving index recommendations for a query.
    #[serde(rename="queryAdvice")]
    
    pub query_advice: Option<QueryAdvisorResult>,
}

impl client::Part for QueryPlan {}


/// Message type to initiate a read-only transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadOnly {
    /// Executes all reads at a timestamp that is `exact_staleness` old. The timestamp is chosen soon after the read is started. Guarantees that all writes that have committed more than the specified number of seconds ago are visible. Because Cloud Spanner chooses the exact timestamp, this mode works even if the client's local clock is substantially skewed from Cloud Spanner commit timestamps. Useful for reading at nearby replicas without the distributed timestamp negotiation overhead of `max_staleness`.
    #[serde(rename="exactStaleness")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub exact_staleness: Option<client::chrono::Duration>,
    /// Read data at a timestamp >= `NOW - max_staleness` seconds. Guarantees that all writes that have committed more than the specified number of seconds ago are visible. Because Cloud Spanner chooses the exact timestamp, this mode works even if the client's local clock is substantially skewed from Cloud Spanner commit timestamps. Useful for reading the freshest data available at a nearby replica, while bounding the possible staleness if the local replica has fallen behind. Note that this option can only be used in single-use transactions.
    #[serde(rename="maxStaleness")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_staleness: Option<client::chrono::Duration>,
    /// Executes all reads at a timestamp >= `min_read_timestamp`. This is useful for requesting fresher data than some previous read, or data that is fresh enough to observe the effects of some previously committed transaction whose timestamp is known. Note that this option can only be used in single-use transactions. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: `"2014-10-02T15:01:23.045123456Z"`.
    #[serde(rename="minReadTimestamp")]
    
    pub min_read_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Executes all reads at the given timestamp. Unlike other modes, reads at a specific timestamp are repeatable; the same read at the same timestamp always returns the same data. If the timestamp is in the future, the read will block until the specified timestamp, modulo the read's deadline. Useful for large scale consistent reads such as mapreduces, or for coordinating many reads against a consistent snapshot of the data. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: `"2014-10-02T15:01:23.045123456Z"`.
    #[serde(rename="readTimestamp")]
    
    pub read_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If true, the Cloud Spanner-selected read timestamp is included in the Transaction message that describes the transaction.
    #[serde(rename="returnReadTimestamp")]
    
    pub return_read_timestamp: Option<bool>,
    /// Read at a timestamp where all previously committed transactions are visible.
    
    pub strong: Option<bool>,
}

impl client::Part for ReadOnly {}


/// The request for Read and StreamingRead.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions read projects](ProjectInstanceDatabaseSessionReadCall) (request)
/// * [instances databases sessions streaming read projects](ProjectInstanceDatabaseSessionStreamingReadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    /// Required. The columns of table to be returned for each row matching this request.
    
    pub columns: Option<Vec<String>>,
    /// If this is for a partitioned read and this field is set to `true`, the request is executed with Spanner Data Boost independent compute resources. If the field is set to `true` but the request does not set `partition_token`, the API returns an `INVALID_ARGUMENT` error.
    #[serde(rename="dataBoostEnabled")]
    
    pub data_boost_enabled: Option<bool>,
    /// Directed read options for this request.
    #[serde(rename="directedReadOptions")]
    
    pub directed_read_options: Option<DirectedReadOptions>,
    /// If non-empty, the name of an index on table. This index is used instead of the table primary key when interpreting key_set and sorting result rows. See key_set for further information.
    
    pub index: Option<String>,
    /// Required. `key_set` identifies the rows to be yielded. `key_set` names the primary keys of the rows in table to be yielded, unless index is present. If index is present, then key_set instead names index keys in index. If the partition_token field is empty, rows are yielded in table primary key order (if index is empty) or index key order (if index is non-empty). If the partition_token field is not empty, rows will be yielded in an unspecified order. It is not an error for the `key_set` to name rows that do not exist in the database. Read yields nothing for nonexistent rows.
    #[serde(rename="keySet")]
    
    pub key_set: Option<KeySet>,
    /// If greater than zero, only the first `limit` rows are yielded. If `limit` is zero, the default is no limit. A limit cannot be specified if `partition_token` is set.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// If present, results will be restricted to the specified partition previously created using PartitionRead(). There must be an exact match for the values of fields common to this message and the PartitionReadRequest message used to create this partition_token.
    #[serde(rename="partitionToken")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub partition_token: Option<Vec<u8>>,
    /// Common options for this request.
    #[serde(rename="requestOptions")]
    
    pub request_options: Option<RequestOptions>,
    /// If this request is resuming a previously interrupted read, `resume_token` should be copied from the last PartialResultSet yielded before the interruption. Doing this enables the new read to resume where the last read left off. The rest of the request parameters must exactly match the request that yielded this token.
    #[serde(rename="resumeToken")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub resume_token: Option<Vec<u8>>,
    /// Required. The name of the table in the database to be read.
    
    pub table: Option<String>,
    /// The transaction to use. If none is provided, the default is a temporary read-only transaction with strong concurrency.
    
    pub transaction: Option<TransactionSelector>,
}

impl client::RequestValue for ReadRequest {}


/// Message type to initiate a read-write transaction. Currently this transaction type has no options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadWrite {
    /// Read lock mode for the transaction.
    #[serde(rename="readLockMode")]
    
    pub read_lock_mode: Option<ReadWriteReadLockModeEnum>,
}

impl client::Part for ReadWrite {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaInfo {
    /// If true, this location is designated as the default leader location where leader replicas are placed. See the [region types documentation](https://cloud.google.com/spanner/docs/instances#region_types) for more details.
    #[serde(rename="defaultLeaderLocation")]
    
    pub default_leader_location: Option<bool>,
    /// The location of the serving resources, e.g., "us-central1".
    
    pub location: Option<String>,
    /// The type of replica.
    #[serde(rename="type")]
    
    pub type_: Option<ReplicaInfoTypeEnum>,
}

impl client::Part for ReplicaInfo {}


/// The directed read replica selector. Callers must provide one or more of the following fields for replica selection: * `location` - The location must be one of the regions within the multi-region configuration of your database. * `type` - The type of the replica. Some examples of using replica_selectors are: * `location:us-east1` --> The "us-east1" replica(s) of any available type will be used to process the request. * `type:READ_ONLY` --> The "READ_ONLY" type replica(s) in nearest available location will be used to process the request. * `location:us-east1 type:READ_ONLY` --> The "READ_ONLY" type replica(s) in location "us-east1" will be used to process the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaSelection {
    /// The location or region of the serving requests, e.g. "us-east1".
    
    pub location: Option<String>,
    /// The type of replica.
    #[serde(rename="type")]
    
    pub type_: Option<ReplicaSelectionTypeEnum>,
}

impl client::Part for ReplicaSelection {}


/// Common request options for various APIs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestOptions {
    /// Priority for the request.
    
    pub priority: Option<RequestOptionPriorityEnum>,
    /// A per-request tag which can be applied to queries or reads, used for statistics collection. Both request_tag and transaction_tag can be specified for a read or query that belongs to a transaction. This field is ignored for requests where it's not applicable (e.g. CommitRequest). Legal characters for `request_tag` values are all printable characters (ASCII 32 - 126) and the length of a request_tag is limited to 50 characters. Values that exceed this limit are truncated. Any leading underscore (_) characters will be removed from the string.
    #[serde(rename="requestTag")]
    
    pub request_tag: Option<String>,
    /// A tag used for statistics collection about this transaction. Both request_tag and transaction_tag can be specified for a read or query that belongs to a transaction. The value of transaction_tag should be the same for all requests belonging to the same transaction. If this request doesn't belong to any transaction, transaction_tag will be ignored. Legal characters for `transaction_tag` values are all printable characters (ASCII 32 - 126) and the length of a transaction_tag is limited to 50 characters. Values that exceed this limit are truncated. Any leading underscore (_) characters will be removed from the string.
    #[serde(rename="transactionTag")]
    
    pub transaction_tag: Option<String>,
}

impl client::Part for RequestOptions {}


/// Encryption configuration for the restored database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreDatabaseEncryptionConfig {
    /// Required. The encryption type of the restored database.
    #[serde(rename="encryptionType")]
    
    pub encryption_type: Option<RestoreDatabaseEncryptionConfigEncryptionTypeEnum>,
    /// Optional. The Cloud KMS key that will be used to encrypt/decrypt the restored database. This field should be set only when encryption_type is `CUSTOMER_MANAGED_ENCRYPTION`. Values are of the form `projects//locations//keyRings//cryptoKeys/`.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for RestoreDatabaseEncryptionConfig {}


/// The request for RestoreDatabase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases restore projects](ProjectInstanceDatabaseRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreDatabaseRequest {
    /// Name of the backup from which to restore. Values are of the form `projects//instances//backups/`.
    
    pub backup: Option<String>,
    /// Required. The id of the database to create and restore to. This database must not already exist. The `database_id` appended to `parent` forms the full database name of the form `projects//instances//databases/`.
    #[serde(rename="databaseId")]
    
    pub database_id: Option<String>,
    /// Optional. An encryption configuration describing the encryption type and key resources in Cloud KMS used to encrypt/decrypt the database to restore to. If this field is not specified, the restored database will use the same encryption configuration as the backup by default, namely encryption_type = `USE_CONFIG_DEFAULT_OR_BACKUP_ENCRYPTION`.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<RestoreDatabaseEncryptionConfig>,
}

impl client::RequestValue for RestoreDatabaseRequest {}


/// Information about the database restore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreInfo {
    /// Information about the backup used to restore the database. The backup may no longer exist.
    #[serde(rename="backupInfo")]
    
    pub backup_info: Option<BackupInfo>,
    /// The type of the restore source.
    #[serde(rename="sourceType")]
    
    pub source_type: Option<RestoreInfoSourceTypeEnum>,
}

impl client::Part for RestoreInfo {}


/// Results from Read or ExecuteSql.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions execute sql projects](ProjectInstanceDatabaseSessionExecuteSqlCall) (response)
/// * [instances databases sessions read projects](ProjectInstanceDatabaseSessionReadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultSet {
    /// Metadata about the result set, such as row type information.
    
    pub metadata: Option<ResultSetMetadata>,
    /// Each element in `rows` is a row whose format is defined by metadata.row_type. The ith element in each row matches the ith field in metadata.row_type. Elements are encoded based on type as described here.
    
    pub rows: Option<Vec<Vec<json::Value>>>,
    /// Query plan and execution statistics for the SQL statement that produced this result set. These can be requested by setting ExecuteSqlRequest.query_mode. DML statements always produce stats containing the number of rows modified, unless executed using the ExecuteSqlRequest.QueryMode.PLAN ExecuteSqlRequest.query_mode. Other fields may or may not be populated, based on the ExecuteSqlRequest.query_mode.
    
    pub stats: Option<ResultSetStats>,
}

impl client::ResponseResult for ResultSet {}


/// Metadata about a ResultSet or PartialResultSet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultSetMetadata {
    /// Indicates the field names and types for the rows in the result set. For example, a SQL query like `"SELECT UserId, UserName FROM Users"` could return a `row_type` value like: "fields": [ { "name": "UserId", "type": { "code": "INT64" } }, { "name": "UserName", "type": { "code": "STRING" } }, ]
    #[serde(rename="rowType")]
    
    pub row_type: Option<StructType>,
    /// If the read or SQL query began a transaction as a side-effect, the information about the new transaction is yielded here.
    
    pub transaction: Option<Transaction>,
    /// A SQL query can be parameterized. In PLAN mode, these parameters can be undeclared. This indicates the field names and types for those undeclared parameters in the SQL query. For example, a SQL query like `"SELECT * FROM Users where UserId = @userId and UserName = @userName "` could return a `undeclared_parameters` value like: "fields": [ { "name": "UserId", "type": { "code": "INT64" } }, { "name": "UserName", "type": { "code": "STRING" } }, ]
    #[serde(rename="undeclaredParameters")]
    
    pub undeclared_parameters: Option<StructType>,
}

impl client::Part for ResultSetMetadata {}


/// Additional statistics about a ResultSet or PartialResultSet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultSetStats {
    /// QueryPlan for the query associated with this result.
    #[serde(rename="queryPlan")]
    
    pub query_plan: Option<QueryPlan>,
    /// Aggregated statistics from the execution of the query. Only present when the query is profiled. For example, a query could return the statistics as follows: { "rows_returned": "3", "elapsed_time": "1.22 secs", "cpu_time": "1.19 secs" }
    #[serde(rename="queryStats")]
    
    pub query_stats: Option<HashMap<String, json::Value>>,
    /// Standard DML returns an exact count of rows that were modified.
    #[serde(rename="rowCountExact")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count_exact: Option<i64>,
    /// Partitioned DML does not offer exactly-once semantics, so it returns a lower bound of the rows modified.
    #[serde(rename="rowCountLowerBound")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count_lower_bound: Option<i64>,
}

impl client::Part for ResultSetStats {}


/// The request for Rollback.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions rollback projects](ProjectInstanceDatabaseSessionRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackRequest {
    /// Required. The transaction to roll back.
    #[serde(rename="transactionId")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub transaction_id: Option<Vec<u8>>,
}

impl client::RequestValue for RollbackRequest {}


/// Scan is a structure which describes Cloud Key Visualizer scan information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases get scans projects](ProjectInstanceDatabaseGetScanCall) (response)
/// * [list scans](ScanListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Scan {
    /// Additional information provided by the implementer.
    
    pub details: Option<HashMap<String, json::Value>>,
    /// The upper bound for when the scan is defined.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The unique name of the scan, specific to the Database service implementing this interface.
    
    pub name: Option<String>,
    /// Output only. Cloud Key Visualizer scan data. Note, this field is not available to the ListScans method.
    #[serde(rename="scanData")]
    
    pub scan_data: Option<ScanData>,
    /// A range of time (inclusive) for when the scan is defined. The lower bound for when the scan is defined.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Scan {}
impl client::ResponseResult for Scan {}


/// ScanData contains Cloud Key Visualizer scan data used by the caller to construct a visualization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScanData {
    /// Cloud Key Visualizer scan data. The range of time this information covers is captured via the above time range fields. Note, this field is not available to the ListScans method.
    
    pub data: Option<VisualizationData>,
    /// The upper bound for when the contained data is defined.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A range of time (inclusive) for when the contained data is defined. The lower bound for when the contained data is defined.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ScanData {}


/// A session in the Cloud Spanner API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions create projects](ProjectInstanceDatabaseSessionCreateCall) (response)
/// * [instances databases sessions get projects](ProjectInstanceDatabaseSessionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    /// Output only. The approximate timestamp when the session is last used. It is typically earlier than the actual last use time.
    #[serde(rename="approximateLastUseTime")]
    
    pub approximate_last_use_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The timestamp when the session is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The database role which created this session.
    #[serde(rename="creatorRole")]
    
    pub creator_role: Option<String>,
    /// The labels for the session. * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. * No more than 64 labels can be associated with a given session. See https://goo.gl/xmQnxf for more information on and examples of labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. If true, specifies a multiplexed session. A multiplexed session may be used for multiple, concurrent read-only operations but can not be used for read-write transactions, partitioned reads, or partitioned queries. Multiplexed sessions can be created via CreateSession but not via BatchCreateSessions. Multiplexed sessions may not be deleted nor listed.
    
    pub multiplexed: Option<bool>,
    /// Output only. The name of the session. This is always system-assigned.
    
    pub name: Option<String>,
}

impl client::ResponseResult for Session {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups set iam policy projects](ProjectInstanceBackupSetIamPolicyCall) (request)
/// * [instances databases set iam policy projects](ProjectInstanceDatabaseSetIamPolicyCall) (request)
/// * [instances set iam policy projects](ProjectInstanceSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Condensed representation of a node and its subtree. Only present for `SCALAR` PlanNode(s).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShortRepresentation {
    /// A string representation of the expression subtree rooted at this node.
    
    pub description: Option<String>,
    /// A mapping of (subquery variable name) -> (subquery node id) for cases where the `description` string of this node references a `SCALAR` subquery contained in the expression subtree rooted at this node. The referenced `SCALAR` subquery may not necessarily be a direct child of this node.
    
    pub subqueries: Option<HashMap<String, i32>>,
}

impl client::Part for ShortRepresentation {}


/// A single DML statement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Statement {
    /// It is not always possible for Cloud Spanner to infer the right SQL type from a JSON value. For example, values of type `BYTES` and values of type `STRING` both appear in params as JSON strings. In these cases, `param_types` can be used to specify the exact SQL type for some or all of the SQL statement parameters. See the definition of Type for more information about SQL types.
    #[serde(rename="paramTypes")]
    
    pub param_types: Option<HashMap<String, Type>>,
    /// Parameter names and values that bind to placeholders in the DML string. A parameter placeholder consists of the `@` character followed by the parameter name (for example, `@firstName`). Parameter names can contain letters, numbers, and underscores. Parameters can appear anywhere that a literal value is expected. The same parameter name can be used more than once, for example: `"WHERE id > @msg_id AND id < @msg_id + 100"` It is an error to execute a SQL statement with unbound parameters.
    
    pub params: Option<HashMap<String, json::Value>>,
    /// Required. The DML string.
    
    pub sql: Option<String>,
}

impl client::Part for Statement {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// `StructType` defines the fields of a STRUCT type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructType {
    /// The list of fields that make up this struct. Order is significant, because values of this struct type are represented as lists, where the order of field values matches the order of fields in the StructType. In turn, the order of fields matches the order of columns in a read request, or the order of fields in the `SELECT` clause of a query.
    
    pub fields: Option<Vec<Field>>,
}

impl client::Part for StructType {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups test iam permissions projects](ProjectInstanceBackupTestIamPermissionCall) (request)
/// * [instances databases database roles test iam permissions projects](ProjectInstanceDatabaseDatabaseRoleTestIamPermissionCall) (request)
/// * [instances databases test iam permissions projects](ProjectInstanceDatabaseTestIamPermissionCall) (request)
/// * [instances test iam permissions projects](ProjectInstanceTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// REQUIRED: The set of permissions to check for 'resource'. Permissions with wildcards (such as '*', 'spanner.*', 'spanner.instances.*') are not allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances backups test iam permissions projects](ProjectInstanceBackupTestIamPermissionCall) (response)
/// * [instances databases database roles test iam permissions projects](ProjectInstanceDatabaseDatabaseRoleTestIamPermissionCall) (response)
/// * [instances databases test iam permissions projects](ProjectInstanceDatabaseTestIamPermissionCall) (response)
/// * [instances test iam permissions projects](ProjectInstanceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// A transaction.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases sessions begin transaction projects](ProjectInstanceDatabaseSessionBeginTransactionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// `id` may be used to identify the transaction in subsequent Read, ExecuteSql, Commit, or Rollback calls. Single-use read-only transactions do not have IDs, because single-use transactions do not support multiple requests.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub id: Option<Vec<u8>>,
    /// For snapshot read-only transactions, the read timestamp chosen for the transaction. Not returned by default: see TransactionOptions.ReadOnly.return_read_timestamp. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: `"2014-10-02T15:01:23.045123456Z"`.
    #[serde(rename="readTimestamp")]
    
    pub read_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Transaction {}


/// Transactions: Each session can have at most one active transaction at a time (note that standalone reads and queries use a transaction internally and do count towards the one transaction limit). After the active transaction is completed, the session can immediately be re-used for the next transaction. It is not necessary to create a new session for each transaction. Transaction modes: Cloud Spanner supports three transaction modes: 1. Locking read-write. This type of transaction is the only way to write data into Cloud Spanner. These transactions rely on pessimistic locking and, if necessary, two-phase commit. Locking read-write transactions may abort, requiring the application to retry. 2. Snapshot read-only. Snapshot read-only transactions provide guaranteed consistency across several reads, but do not allow writes. Snapshot read-only transactions can be configured to read at timestamps in the past, or configured to perform a strong read (where Spanner will select a timestamp such that the read is guaranteed to see the effects of all transactions that have committed before the start of the read). Snapshot read-only transactions do not need to be committed. Queries on change streams must be performed with the snapshot read-only transaction mode, specifying a strong read. Please see TransactionOptions.ReadOnly.strong for more details. 3. Partitioned DML. This type of transaction is used to execute a single Partitioned DML statement. Partitioned DML partitions the key space and runs the DML statement over each partition in parallel using separate, internal transactions that commit independently. Partitioned DML transactions do not need to be committed. For transactions that only read, snapshot read-only transactions provide simpler semantics and are almost always faster. In particular, read-only transactions do not take locks, so they do not conflict with read-write transactions. As a consequence of not taking locks, they also do not abort, so retry loops are not needed. Transactions may only read-write data in a single database. They may, however, read-write data in different tables within that database. Locking read-write transactions: Locking transactions may be used to atomically read-modify-write data anywhere in a database. This type of transaction is externally consistent. Clients should attempt to minimize the amount of time a transaction is active. Faster transactions commit with higher probability and cause less contention. Cloud Spanner attempts to keep read locks active as long as the transaction continues to do reads, and the transaction has not been terminated by Commit or Rollback. Long periods of inactivity at the client may cause Cloud Spanner to release a transaction's locks and abort it. Conceptually, a read-write transaction consists of zero or more reads or SQL statements followed by Commit. At any time before Commit, the client can send a Rollback request to abort the transaction. Semantics: Cloud Spanner can commit the transaction if all read locks it acquired are still valid at commit time, and it is able to acquire write locks for all writes. Cloud Spanner can abort the transaction for any reason. If a commit attempt returns `ABORTED`, Cloud Spanner guarantees that the transaction has not modified any user data in Cloud Spanner. Unless the transaction commits, Cloud Spanner makes no guarantees about how long the transaction's locks were held for. It is an error to use Cloud Spanner locks for any sort of mutual exclusion other than between Cloud Spanner transactions themselves. Retrying aborted transactions: When a transaction aborts, the application can choose to retry the whole transaction again. To maximize the chances of successfully committing the retry, the client should execute the retry in the same session as the original attempt. The original session's lock priority increases with each consecutive abort, meaning that each attempt has a slightly better chance of success than the previous. Under some circumstances (for example, many transactions attempting to modify the same row(s)), a transaction can abort many times in a short period before successfully committing. Thus, it is not a good idea to cap the number of retries a transaction can attempt; instead, it is better to limit the total amount of time spent retrying. Idle transactions: A transaction is considered idle if it has no outstanding reads or SQL queries and has not started a read or SQL query within the last 10 seconds. Idle transactions can be aborted by Cloud Spanner so that they don't hold on to locks indefinitely. If an idle transaction is aborted, the commit will fail with error `ABORTED`. If this behavior is undesirable, periodically executing a simple SQL query in the transaction (for example, `SELECT 1`) prevents the transaction from becoming idle. Snapshot read-only transactions: Snapshot read-only transactions provides a simpler method than locking read-write transactions for doing several consistent reads. However, this type of transaction does not support writes. Snapshot transactions do not take locks. Instead, they work by choosing a Cloud Spanner timestamp, then executing all reads at that timestamp. Since they do not acquire locks, they do not block concurrent read-write transactions. Unlike locking read-write transactions, snapshot read-only transactions never abort. They can fail if the chosen read timestamp is garbage collected; however, the default garbage collection policy is generous enough that most applications do not need to worry about this in practice. Snapshot read-only transactions do not need to call Commit or Rollback (and in fact are not permitted to do so). To execute a snapshot transaction, the client specifies a timestamp bound, which tells Cloud Spanner how to choose a read timestamp. The types of timestamp bound are: - Strong (the default). - Bounded staleness. - Exact staleness. If the Cloud Spanner database to be read is geographically distributed, stale read-only transactions can execute more quickly than strong or read-write transactions, because they are able to execute far from the leader replica. Each type of timestamp bound is discussed in detail below. Strong: Strong reads are guaranteed to see the effects of all transactions that have committed before the start of the read. Furthermore, all rows yielded by a single read are consistent with each other -- if any part of the read observes a transaction, all parts of the read see the transaction. Strong reads are not repeatable: two consecutive strong read-only transactions might return inconsistent results if there are concurrent writes. If consistency across reads is required, the reads should be executed within a transaction or at an exact read timestamp. Queries on change streams (see below for more details) must also specify the strong read timestamp bound. See TransactionOptions.ReadOnly.strong. Exact staleness: These timestamp bounds execute reads at a user-specified timestamp. Reads at a timestamp are guaranteed to see a consistent prefix of the global transaction history: they observe modifications done by all transactions with a commit timestamp less than or equal to the read timestamp, and observe none of the modifications done by transactions with a larger commit timestamp. They will block until all conflicting transactions that may be assigned commit timestamps <= the read timestamp have finished. The timestamp can either be expressed as an absolute Cloud Spanner commit timestamp or a staleness relative to the current time. These modes do not require a "negotiation phase" to pick a timestamp. As a result, they execute slightly faster than the equivalent boundedly stale concurrency modes. On the other hand, boundedly stale reads usually return fresher results. See TransactionOptions.ReadOnly.read_timestamp and TransactionOptions.ReadOnly.exact_staleness. Bounded staleness: Bounded staleness modes allow Cloud Spanner to pick the read timestamp, subject to a user-provided staleness bound. Cloud Spanner chooses the newest timestamp within the staleness bound that allows execution of the reads at the closest available replica without blocking. All rows yielded are consistent with each other -- if any part of the read observes a transaction, all parts of the read see the transaction. Boundedly stale reads are not repeatable: two stale reads, even if they use the same staleness bound, can execute at different timestamps and thus return inconsistent results. Boundedly stale reads execute in two phases: the first phase negotiates a timestamp among all replicas needed to serve the read. In the second phase, reads are executed at the negotiated timestamp. As a result of the two phase execution, bounded staleness reads are usually a little slower than comparable exact staleness reads. However, they are typically able to return fresher results, and are more likely to execute at the closest replica. Because the timestamp negotiation requires up-front knowledge of which rows will be read, it can only be used with single-use read-only transactions. See TransactionOptions.ReadOnly.max_staleness and TransactionOptions.ReadOnly.min_read_timestamp. Old read timestamps and garbage collection: Cloud Spanner continuously garbage collects deleted and overwritten data in the background to reclaim storage space. This process is known as "version GC". By default, version GC reclaims versions after they are one hour old. Because of this, Cloud Spanner cannot perform reads at read timestamps more than one hour in the past. This restriction also applies to in-progress reads and/or SQL queries whose timestamp become too old while executing. Reads and SQL queries with too-old read timestamps fail with the error `FAILED_PRECONDITION`. You can configure and extend the `VERSION_RETENTION_PERIOD` of a database up to a period as long as one week, which allows Cloud Spanner to perform reads up to one week in the past. Querying change Streams: A Change Stream is a schema object that can be configured to watch data changes on the entire database, a set of tables, or a set of columns in a database. When a change stream is created, Spanner automatically defines a corresponding SQL Table-Valued Function (TVF) that can be used to query the change records in the associated change stream using the ExecuteStreamingSql API. The name of the TVF for a change stream is generated from the name of the change stream: READ_. All queries on change stream TVFs must be executed using the ExecuteStreamingSql API with a single-use read-only transaction with a strong read-only timestamp_bound. The change stream TVF allows users to specify the start_timestamp and end_timestamp for the time range of interest. All change records within the retention period is accessible using the strong read-only timestamp_bound. All other TransactionOptions are invalid for change stream queries. In addition, if TransactionOptions.read_only.return_read_timestamp is set to true, a special value of 2^63 - 2 will be returned in the Transaction message that describes the transaction, instead of a valid read timestamp. This special value should be discarded and not used for any subsequent queries. Please see https://cloud.google.com/spanner/docs/change-streams for more details on how to query the change stream TVFs. Partitioned DML transactions: Partitioned DML transactions are used to execute DML statements with a different execution strategy that provides different, and often better, scalability properties for large, table-wide operations than DML in a ReadWrite transaction. Smaller scoped statements, such as an OLTP workload, should prefer using ReadWrite transactions. Partitioned DML partitions the keyspace and runs the DML statement on each partition in separate, internal transactions. These transactions commit automatically when complete, and run independently from one another. To reduce lock contention, this execution strategy only acquires read locks on rows that match the WHERE clause of the statement. Additionally, the smaller per-partition transactions hold locks for less time. That said, Partitioned DML is not a drop-in replacement for standard DML used in ReadWrite transactions. - The DML statement must be fully-partitionable. Specifically, the statement must be expressible as the union of many statements which each access only a single row of the table. - The statement is not applied atomically to all rows of the table. Rather, the statement is applied atomically to partitions of the table, in independent transactions. Secondary index rows are updated atomically with the base table rows. - Partitioned DML does not guarantee exactly-once execution semantics against a partition. The statement will be applied at least once to each partition. It is strongly recommended that the DML statement should be idempotent to avoid unexpected results. For instance, it is potentially dangerous to run a statement such as `UPDATE table SET column = column + 1` as it could be run multiple times against some rows. - The partitions are committed automatically - there is no support for Commit or Rollback. If the call returns an error, or if the client issuing the ExecuteSql call dies, it is possible that some rows had the statement executed on them successfully. It is also possible that statement was never executed against other rows. - Partitioned DML transactions may only contain the execution of a single DML statement via ExecuteSql or ExecuteStreamingSql. - If any error is encountered during the execution of the partitioned DML operation (for instance, a UNIQUE INDEX violation, division by zero, or a value that cannot be stored due to schema constraints), then the operation is stopped at that point and an error is returned. It is possible that at this point, some partitions have been committed (or even committed multiple times), and other partitions have not been run at all. Given the above, Partitioned DML is good fit for large, database-wide, operations that are idempotent, such as deleting old rows from a very large table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionOptions {
    /// When `exclude_txn_from_change_streams` is set to `true`: * Modifications from this transaction will not be recorded in change streams with DDL option `allow_txn_exclusion=true` that are tracking columns modified by these transactions. * Modifications from this transaction will be recorded in change streams with DDL option `allow_txn_exclusion=false or not set` that are tracking columns modified by these transactions. When `exclude_txn_from_change_streams` is set to `false` or not set, Modifications from this transaction will be recorded in all change streams that are tracking columns modified by these transactions. `exclude_txn_from_change_streams` may only be specified for read-write or partitioned-dml transactions, otherwise the API will return an `INVALID_ARGUMENT` error.
    #[serde(rename="excludeTxnFromChangeStreams")]
    
    pub exclude_txn_from_change_streams: Option<bool>,
    /// Partitioned DML transaction. Authorization to begin a Partitioned DML transaction requires `spanner.databases.beginPartitionedDmlTransaction` permission on the `session` resource.
    #[serde(rename="partitionedDml")]
    
    pub partitioned_dml: Option<PartitionedDml>,
    /// Transaction will not write. Authorization to begin a read-only transaction requires `spanner.databases.beginReadOnlyTransaction` permission on the `session` resource.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<ReadOnly>,
    /// Transaction may write. Authorization to begin a read-write transaction requires `spanner.databases.beginOrRollbackReadWriteTransaction` permission on the `session` resource.
    #[serde(rename="readWrite")]
    
    pub read_write: Option<ReadWrite>,
}

impl client::Part for TransactionOptions {}


/// This message is used to select the transaction in which a Read or ExecuteSql call runs. See TransactionOptions for more information about transactions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionSelector {
    /// Begin a new transaction and execute this read or SQL query in it. The transaction ID of the new transaction is returned in ResultSetMetadata.transaction, which is a Transaction.
    
    pub begin: Option<TransactionOptions>,
    /// Execute the read or SQL query in a previously-started transaction.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub id: Option<Vec<u8>>,
    /// Execute the read or SQL query in a temporary transaction. This is the most efficient way to execute a transaction that consists of a single SQL query.
    #[serde(rename="singleUse")]
    
    pub single_use: Option<TransactionOptions>,
}

impl client::Part for TransactionSelector {}


/// `Type` indicates the type of a Cloud Spanner value, as might be stored in a table cell or returned from an SQL query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Type {
    /// If code == ARRAY, then `array_element_type` is the type of the array elements.
    #[serde(rename="arrayElementType")]
    
    pub array_element_type: Option<Option<Box<Type>>>,
    /// Required. The TypeCode for this type.
    
    pub code: Option<TypeCodeEnum>,
    /// If code == PROTO or code == ENUM, then `proto_type_fqn` is the fully qualified name of the proto type representing the proto/enum definition.
    #[serde(rename="protoTypeFqn")]
    
    pub proto_type_fqn: Option<String>,
    /// If code == STRUCT, then `struct_type` provides type information for the struct's fields.
    #[serde(rename="structType")]
    
    pub struct_type: Option<StructType>,
    /// The TypeAnnotationCode that disambiguates SQL type that Spanner will use to represent values of this type during query processing. This is necessary for some type codes because a single TypeCode can be mapped to different SQL types depending on the SQL dialect. type_annotation typically is not needed to process the content of a value (it doesn't affect serialization) and clients can ignore it on the read path.
    #[serde(rename="typeAnnotation")]
    
    pub type_annotation: Option<TypeTypeAnnotationEnum>,
}

impl client::Part for Type {}


/// Enqueues the given DDL statements to be applied, in order but not necessarily all at once, to the database schema at some point (or points) in the future. The server checks that the statements are executable (syntactically valid, name tables that exist, etc.) before enqueueing them, but they may still fail upon later execution (e.g., if a statement from another batch of statements is applied first and it conflicts in some way, or if there is some data-related problem like a `NULL` value in a column to which `NOT NULL` would be added). If a statement fails, all subsequent statements in the batch are automatically cancelled. Each batch of statements is assigned a name which can be used with the Operations API to monitor progress. See the operation_id field for more details.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances databases update ddl projects](ProjectInstanceDatabaseUpdateDdlCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDatabaseDdlRequest {
    /// If empty, the new update request is assigned an automatically-generated operation ID. Otherwise, `operation_id` is used to construct the name of the resulting Operation. Specifying an explicit operation ID simplifies determining whether the statements were executed in the event that the UpdateDatabaseDdl call is replayed, or the return value is otherwise lost: the database and `operation_id` fields can be combined to form the name of the resulting longrunning.Operation: `/operations/`. `operation_id` should be unique within the database, and must be a valid identifier: `a-z*`. Note that automatically-generated operation IDs always begin with an underscore. If the named operation already exists, UpdateDatabaseDdl returns `ALREADY_EXISTS`.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// Optional. Proto descriptors used by CREATE/ALTER PROTO BUNDLE statements. Contains a protobuf-serialized [google.protobuf.FileDescriptorSet](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto). To generate it, [install](https://grpc.io/docs/protoc-installation/) and run `protoc` with --include_imports and --descriptor_set_out. For example, to generate for moon/shot/app.proto, run ``` $protoc --proto_path=/app_path --proto_path=/lib_path \ --include_imports \ --descriptor_set_out=descriptors.data \ moon/shot/app.proto ``` For more details, see protobuffer [self description](https://developers.google.com/protocol-buffers/docs/techniques#self-description).
    #[serde(rename="protoDescriptors")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub proto_descriptors: Option<Vec<u8>>,
    /// Required. DDL statements to be applied to the database.
    
    pub statements: Option<Vec<String>>,
}

impl client::RequestValue for UpdateDatabaseDdlRequest {}


/// The request for UpdateInstanceConfigRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instance configs patch projects](ProjectInstanceConfigPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInstanceConfigRequest {
    /// Required. The user instance config to update, which must always include the instance config name. Otherwise, only fields mentioned in update_mask need be included. To prevent conflicts of concurrent updates, etag can be used.
    #[serde(rename="instanceConfig")]
    
    pub instance_config: Option<InstanceConfig>,
    /// Required. A mask specifying which fields in InstanceConfig should be updated. The field mask must always be specified; this prevents any future fields in InstanceConfig from being erased accidentally by clients that do not know about them. Only display_name and labels can be updated.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
    /// An option to validate, but not actually execute, a request, and provide the same response.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for UpdateInstanceConfigRequest {}


/// The request for UpdateInstancePartition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances instance partitions patch projects](ProjectInstanceInstancePartitionPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInstancePartitionRequest {
    /// Required. A mask specifying which fields in InstancePartition should be updated. The field mask must always be specified; this prevents any future fields in InstancePartition from being erased accidentally by clients that do not know about them.
    #[serde(rename="fieldMask")]
    
    pub field_mask: Option<client::FieldMask>,
    /// Required. The instance partition to update, which must always include the instance partition name. Otherwise, only fields mentioned in field_mask need be included.
    #[serde(rename="instancePartition")]
    
    pub instance_partition: Option<InstancePartition>,
}

impl client::RequestValue for UpdateInstancePartitionRequest {}


/// The request for UpdateInstance.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [instances patch projects](ProjectInstancePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInstanceRequest {
    /// Required. A mask specifying which fields in Instance should be updated. The field mask must always be specified; this prevents any future fields in Instance from being erased accidentally by clients that do not know about them.
    #[serde(rename="fieldMask")]
    
    pub field_mask: Option<client::FieldMask>,
    /// Required. The instance to update, which must always include the instance name. Otherwise, only fields mentioned in field_mask need be included.
    
    pub instance: Option<Instance>,
}

impl client::RequestValue for UpdateInstanceRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VisualizationData {
    /// The token signifying the end of a data_source.
    #[serde(rename="dataSourceEndToken")]
    
    pub data_source_end_token: Option<String>,
    /// The token delimiting a datasource name from the rest of a key in a data_source.
    #[serde(rename="dataSourceSeparatorToken")]
    
    pub data_source_separator_token: Option<String>,
    /// The list of messages (info, alerts, ...)
    #[serde(rename="diagnosticMessages")]
    
    pub diagnostic_messages: Option<Vec<DiagnosticMessage>>,
    /// We discretize the entire keyspace into buckets. Assuming each bucket has an inclusive keyrange and covers keys from k(i) ... k(n). In this case k(n) would be an end key for a given range. end_key_string is the collection of all such end keys
    #[serde(rename="endKeyStrings")]
    
    pub end_key_strings: Option<Vec<String>>,
    /// Whether this scan contains PII.
    #[serde(rename="hasPii")]
    
    pub has_pii: Option<bool>,
    /// Keys of key ranges that contribute significantly to a given metric Can be thought of as heavy hitters.
    #[serde(rename="indexedKeys")]
    
    pub indexed_keys: Option<Vec<String>>,
    /// The token delimiting the key prefixes.
    #[serde(rename="keySeparator")]
    
    pub key_separator: Option<String>,
    /// The unit for the key: e.g. 'key' or 'chunk'.
    #[serde(rename="keyUnit")]
    
    pub key_unit: Option<VisualizationDataKeyUnitEnum>,
    /// The list of data objects for each metric.
    
    pub metrics: Option<Vec<Metric>>,
    /// The list of extracted key prefix nodes used in the key prefix hierarchy.
    #[serde(rename="prefixNodes")]
    
    pub prefix_nodes: Option<Vec<PrefixNode>>,
}

impl client::Part for VisualizationData {}


/// Arguments to insert, update, insert_or_update, and replace operations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Write {
    /// The names of the columns in table to be written. The list of columns must contain enough columns to allow Cloud Spanner to derive values for all primary key columns in the row(s) to be modified.
    
    pub columns: Option<Vec<String>>,
    /// Required. The table whose rows will be written.
    
    pub table: Option<String>,
    /// The values to be written. `values` can contain more than one list of values. If it does, then multiple rows are written, one for each entry in `values`. Each list in `values` must have exactly as many entries as there are entries in columns above. Sending multiple lists is equivalent to sending multiple `Mutation`s, each containing one `values` entry and repeating table and columns. Individual values in each list are encoded as described here.
    
    pub values: Option<Vec<Vec<json::Value>>>,
}

impl client::Part for Write {}


