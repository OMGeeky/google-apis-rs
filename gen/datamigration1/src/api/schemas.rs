use super::*;
/// Specifies required connection parameters, and the parameters required to create an AlloyDB destination cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlloyDbConnectionProfile {
    /// Required. The AlloyDB cluster ID that this connection profile is associated with.
    #[serde(rename="clusterId")]
    
    pub cluster_id: Option<String>,
    /// Immutable. Metadata used to create the destination AlloyDB cluster.
    
    pub settings: Option<AlloyDbSettings>,
}

impl client::Part for AlloyDbConnectionProfile {}


/// Settings for creating an AlloyDB cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlloyDbSettings {
    /// Optional. The database engine major version. This is an optional field. If a database version is not supplied at cluster creation time, then a default database version will be used.
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<AlloyDbSettingDatabaseVersionEnum>,
    /// Optional. The encryption config can be specified to encrypt the data disks and other persistent data resources of a cluster with a customer-managed encryption key (CMEK). When this field is not specified, the cluster will then use default encryption scheme to protect the user data.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Required. Input only. Initial user to setup during cluster creation. Required.
    #[serde(rename="initialUser")]
    
    pub initial_user: Option<UserPassword>,
    /// Labels for the AlloyDB cluster created by DMS. An object containing a list of 'key', 'value' pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// no description provided
    #[serde(rename="primaryInstanceSettings")]
    
    pub primary_instance_settings: Option<PrimaryInstanceSettings>,
    /// Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster. It is specified in the form: "projects/{project_number}/global/networks/{network_id}". This is required to create a cluster.
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<String>,
}

impl client::Part for AlloyDbSettings {}


/// Request message for ‘ApplyConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces apply projects](ProjectLocationConversionWorkspaceApplyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplyConversionWorkspaceRequest {
    /// Optional. Specifies whether the conversion workspace is to be committed automatically after the apply.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// Optional. Fully qualified (Uri) name of the destination connection profile.
    #[serde(rename="connectionProfile")]
    
    pub connection_profile: Option<String>,
    /// Optional. Only validates the apply process, but doesn't change the destination database. Only works for PostgreSQL destination connection profile.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// Filter which entities to apply. Leaving this field empty will apply all of the entities. Supports Google AIP 160 based filtering.
    
    pub filter: Option<String>,
}

impl client::RequestValue for ApplyConversionWorkspaceRequest {}


/// Apply a hash function on the value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplyHash {
    /// Optional. Generate UUID from the data's byte array
    #[serde(rename="uuidFromBytes")]
    
    pub uuid_from_bytes: Option<Empty>,
}

impl client::Part for ApplyHash {}


/// Details regarding an Apply background job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplyJobDetails {
    /// Output only. The connection profile which was used for the apply job.
    #[serde(rename="connectionProfile")]
    
    pub connection_profile: Option<String>,
    /// Output only. AIP-160 based filter used to specify the entities to apply
    
    pub filter: Option<String>,
}

impl client::Part for ApplyJobDetails {}


/// Set to a specific value (value is converted to fit the target data type)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AssignSpecificValue {
    /// Required. Specific value to be assigned
    
    pub value: Option<String>,
}

impl client::Part for AssignSpecificValue {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<AuditLogConfigLogTypeEnum>,
}

impl client::Part for AuditLogConfig {}


/// Execution log of a background job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundJobLogEntry {
    /// Output only. Apply job details.
    #[serde(rename="applyJobDetails")]
    
    pub apply_job_details: Option<ApplyJobDetails>,
    /// Output only. Job completion comment, such as how many entities were seeded, how many warnings were found during conversion, and similar information.
    #[serde(rename="completionComment")]
    
    pub completion_comment: Option<String>,
    /// Output only. Job completion state, i.e. the final state after the job completed.
    #[serde(rename="completionState")]
    
    pub completion_state: Option<BackgroundJobLogEntryCompletionStateEnum>,
    /// Output only. Convert job details.
    #[serde(rename="convertJobDetails")]
    
    pub convert_job_details: Option<ConvertJobDetails>,
    /// The timestamp when the background job was finished.
    #[serde(rename="finishTime")]
    
    pub finish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The background job log entry ID.
    
    pub id: Option<String>,
    /// Output only. Import rules job details.
    #[serde(rename="importRulesJobDetails")]
    
    pub import_rules_job_details: Option<ImportRulesJobDetails>,
    /// The type of job that was executed.
    #[serde(rename="jobType")]
    
    pub job_type: Option<BackgroundJobLogEntryJobTypeEnum>,
    /// Output only. Whether the client requested the conversion workspace to be committed after a successful completion of the job.
    #[serde(rename="requestAutocommit")]
    
    pub request_autocommit: Option<bool>,
    /// Output only. Seed job details.
    #[serde(rename="seedJobDetails")]
    
    pub seed_job_details: Option<SeedJobDetails>,
    /// The timestamp when the background job was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for BackgroundJobLogEntry {}


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


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSqlConnectionProfile {
    /// Output only. The Cloud SQL database instance's additional (outgoing) public IP. Used when the Cloud SQL database availability type is REGIONAL (i.e. multiple zones / highly available).
    #[serde(rename="additionalPublicIp")]
    
    pub additional_public_ip: Option<String>,
    /// Output only. The Cloud SQL instance ID that this connection profile is associated with.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Output only. The Cloud SQL database instance's private IP.
    #[serde(rename="privateIp")]
    
    pub private_ip: Option<String>,
    /// Output only. The Cloud SQL database instance's public IP.
    #[serde(rename="publicIp")]
    
    pub public_ip: Option<String>,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    
    pub settings: Option<CloudSqlSettings>,
}

impl client::Part for CloudSqlConnectionProfile {}


/// Settings for creating a Cloud SQL database instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSqlSettings {
    /// The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Valid values: 'ALWAYS': The instance is on, and remains so even in the absence of connection requests. `NEVER`: The instance is off; it is not activated, even if a connection request arrives.
    #[serde(rename="activationPolicy")]
    
    pub activation_policy: Option<CloudSqlSettingActivationPolicyEnum>,
    /// [default: ON] If you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity. If the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB.
    #[serde(rename="autoStorageIncrease")]
    
    pub auto_storage_increase: Option<bool>,
    /// Optional. Availability type. Potential values: * `ZONAL`: The instance serves data from only one zone. Outages in that zone affect data availability. * `REGIONAL`: The instance can serve data from more than one zone in a region (it is highly available).
    #[serde(rename="availabilityType")]
    
    pub availability_type: Option<CloudSqlSettingAvailabilityTypeEnum>,
    /// The KMS key name used for the csql instance.
    #[serde(rename="cmekKeyName")]
    
    pub cmek_key_name: Option<String>,
    /// The Cloud SQL default instance level collation.
    
    pub collation: Option<String>,
    /// Optional. Data cache is an optional feature available for Cloud SQL for MySQL Enterprise Plus edition only. For more information on data cache, see [Data cache overview](https://cloud.google.com/sql/help/mysql-data-cache) in Cloud SQL documentation.
    #[serde(rename="dataCacheConfig")]
    
    pub data_cache_config: Option<DataCacheConfig>,
    /// The storage capacity available to the database, in GB. The minimum (and default) size is 10GB.
    #[serde(rename="dataDiskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_disk_size_gb: Option<i64>,
    /// The type of storage: `PD_SSD` (default) or `PD_HDD`.
    #[serde(rename="dataDiskType")]
    
    pub data_disk_type: Option<CloudSqlSettingDataDiskTypeEnum>,
    /// The database flags passed to the Cloud SQL instance at startup. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[serde(rename="databaseFlags")]
    
    pub database_flags: Option<HashMap<String, String>>,
    /// The database engine type and version.
    #[serde(rename="databaseVersion")]
    
    pub database_version: Option<CloudSqlSettingDatabaseVersionEnum>,
    /// Optional. The edition of the given Cloud SQL instance.
    
    pub edition: Option<CloudSqlSettingEditionEnum>,
    /// The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled.
    #[serde(rename="ipConfig")]
    
    pub ip_config: Option<SqlIpConfig>,
    /// Input only. Initial root password.
    #[serde(rename="rootPassword")]
    
    pub root_password: Option<String>,
    /// Output only. Indicates If this connection profile root password is stored.
    #[serde(rename="rootPasswordSet")]
    
    pub root_password_set: Option<bool>,
    /// Optional. The Google Cloud Platform zone where the failover Cloud SQL database instance is located. Used when the Cloud SQL database availability type is REGIONAL (i.e. multiple zones / highly available).
    #[serde(rename="secondaryZone")]
    
    pub secondary_zone: Option<String>,
    /// The Database Migration Service source connection profile ID, in the format: `projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID`
    #[serde(rename="sourceId")]
    
    pub source_id: Option<String>,
    /// The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit.
    #[serde(rename="storageAutoResizeLimit")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub storage_auto_resize_limit: Option<i64>,
    /// The tier (or machine type) for this instance, for example: `db-n1-standard-1` (MySQL instances) or `db-custom-1-3840` (PostgreSQL instances). For more information, see [Cloud SQL Instance Settings](https://cloud.google.com/sql/docs/mysql/instance-settings).
    
    pub tier: Option<String>,
    /// The resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "18kg", "count": "3" }`.
    #[serde(rename="userLabels")]
    
    pub user_labels: Option<HashMap<String, String>>,
    /// The Google Cloud Platform zone where your Cloud SQL database instance is located.
    
    pub zone: Option<String>,
}

impl client::Part for CloudSqlSettings {}


/// Column is not used as an independent entity, it is retrieved as part of a Table entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnEntity {
    /// Is the column of array type.
    
    pub array: Option<bool>,
    /// If the column is array, of which length.
    #[serde(rename="arrayLength")]
    
    pub array_length: Option<i32>,
    /// Is the column auto-generated/identity.
    #[serde(rename="autoGenerated")]
    
    pub auto_generated: Option<bool>,
    /// Charset override - instead of table level charset.
    
    pub charset: Option<String>,
    /// Collation override - instead of table level collation.
    
    pub collation: Option<String>,
    /// Comment associated with the column.
    
    pub comment: Option<String>,
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Column data type.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Default value of the column.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<String>,
    /// Column fractional second precision - used for timestamp based datatypes.
    #[serde(rename="fractionalSecondsPrecision")]
    
    pub fractional_seconds_precision: Option<i32>,
    /// Column length - e.g. varchar (50).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// Column name.
    
    pub name: Option<String>,
    /// Is the column nullable.
    
    pub nullable: Option<bool>,
    /// Column order in the table.
    #[serde(rename="ordinalPosition")]
    
    pub ordinal_position: Option<i32>,
    /// Column precision - when relevant.
    
    pub precision: Option<i32>,
    /// Column scale - when relevant.
    
    pub scale: Option<i32>,
    /// Specifies the list of values allowed in the column. Only used for set data type.
    #[serde(rename="setValues")]
    
    pub set_values: Option<Vec<String>>,
    /// Is the column a UDT.
    
    pub udt: Option<bool>,
}

impl client::Part for ColumnEntity {}


/// Request message for ‘CommitConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces commit projects](ProjectLocationConversionWorkspaceCommitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommitConversionWorkspaceRequest {
    /// Optional. Optional name of the commit.
    #[serde(rename="commitName")]
    
    pub commit_name: Option<String>,
}

impl client::RequestValue for CommitConversionWorkspaceRequest {}


/// Options to configure rule type ConditionalColumnSetValue. The rule is used to transform the data which is being replicated/migrated. The rule filter field can refer to one or more entities. The rule scope can be one of: Column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalColumnSetValue {
    /// Optional. Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Optional. Optional filter on source column precision and scale. Used for fixed point numbers such as NUMERIC/NUMBER data types.
    #[serde(rename="sourceNumericFilter")]
    
    pub source_numeric_filter: Option<SourceNumericFilter>,
    /// Optional. Optional filter on source column length. Used for text based data types like varchar.
    #[serde(rename="sourceTextFilter")]
    
    pub source_text_filter: Option<SourceTextFilter>,
    /// Required. Description of data transformation during migration.
    #[serde(rename="valueTransformation")]
    
    pub value_transformation: Option<ValueTransformation>,
}

impl client::Part for ConditionalColumnSetValue {}


/// A connection profile definition.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles create projects](ProjectLocationConnectionProfileCreateCall) (request)
/// * [locations connection profiles get projects](ProjectLocationConnectionProfileGetCall) (response)
/// * [locations connection profiles patch projects](ProjectLocationConnectionProfilePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionProfile {
    /// An AlloyDB cluster connection profile.
    
    pub alloydb: Option<AlloyDbConnectionProfile>,
    /// A CloudSQL database connection profile.
    
    pub cloudsql: Option<CloudSqlConnectionProfile>,
    /// Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The connection profile display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The error details in case of state FAILED.
    
    pub error: Option<Status>,
    /// The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    
    pub labels: Option<HashMap<String, String>>,
    /// A MySQL database connection profile.
    
    pub mysql: Option<MySqlConnectionProfile>,
    /// The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}.
    
    pub name: Option<String>,
    /// An Oracle database connection profile.
    
    pub oracle: Option<OracleConnectionProfile>,
    /// A PostgreSQL database connection profile.
    
    pub postgresql: Option<PostgreSqlConnectionProfile>,
    /// The database provider.
    
    pub provider: Option<ConnectionProfileProviderEnum>,
    /// Connection profile for a SQL Server data source.
    
    pub sqlserver: Option<SqlServerConnectionProfile>,
    /// The current connection profile state (e.g. DRAFT, READY, or FAILED).
    
    pub state: Option<ConnectionProfileStateEnum>,
    /// Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ConnectionProfile {}
impl client::ResponseResult for ConnectionProfile {}


/// Constraint is not used as an independent entity, it is retrieved as part of another entity such as Table or View.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConstraintEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the table constraint.
    
    pub name: Option<String>,
    /// Reference columns which may be associated with the constraint. For example, if the constraint is a FOREIGN_KEY, this represents the list of full names of referenced columns by the foreign key.
    #[serde(rename="referenceColumns")]
    
    pub reference_columns: Option<Vec<String>>,
    /// Reference table which may be associated with the constraint. For example, if the constraint is a FOREIGN_KEY, this represents the list of full name of the referenced table by the foreign key.
    #[serde(rename="referenceTable")]
    
    pub reference_table: Option<String>,
    /// Table columns used as part of the Constraint, for example primary key constraint should list the columns which constitutes the key.
    #[serde(rename="tableColumns")]
    
    pub table_columns: Option<Vec<String>>,
    /// Table which is associated with the constraint. In case the constraint is defined on a table, this field is left empty as this information is stored in parent_name. However, if constraint is defined on a view, this field stores the table name on which the view is defined.
    #[serde(rename="tableName")]
    
    pub table_name: Option<String>,
    /// Type of constraint, for example unique, primary key, foreign key (currently only primary key is supported).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ConstraintEntity {}


/// The main conversion workspace resource entity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces create projects](ProjectLocationConversionWorkspaceCreateCall) (request)
/// * [locations conversion workspaces get projects](ProjectLocationConversionWorkspaceGetCall) (response)
/// * [locations conversion workspaces patch projects](ProjectLocationConversionWorkspacePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionWorkspace {
    /// Output only. The timestamp when the workspace resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The destination engine details.
    
    pub destination: Option<DatabaseEngineInfo>,
    /// Optional. The display name for the workspace.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. A generic list of settings for the workspace. The settings are database pair dependant and can indicate default behavior for the mapping rules engine or turn on or off specific features. Such examples can be: convert_foreign_key_to_interleave=true, skip_triggers=false, ignore_non_table_synonyms=true
    #[serde(rename="globalSettings")]
    
    pub global_settings: Option<HashMap<String, String>>,
    /// Output only. Whether the workspace has uncommitted changes (changes which were made after the workspace was committed).
    #[serde(rename="hasUncommittedChanges")]
    
    pub has_uncommitted_changes: Option<bool>,
    /// Output only. The latest commit ID.
    #[serde(rename="latestCommitId")]
    
    pub latest_commit_id: Option<String>,
    /// Output only. The timestamp when the workspace was committed.
    #[serde(rename="latestCommitTime")]
    
    pub latest_commit_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Full name of the workspace resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{conversion_workspace}.
    
    pub name: Option<String>,
    /// Required. The source engine details.
    
    pub source: Option<DatabaseEngineInfo>,
    /// Output only. The timestamp when the workspace resource was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ConversionWorkspace {}
impl client::ResponseResult for ConversionWorkspace {}


/// A conversion workspace's version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConversionWorkspaceInfo {
    /// The commit ID of the conversion workspace.
    #[serde(rename="commitId")]
    
    pub commit_id: Option<String>,
    /// The resource name (URI) of the conversion workspace.
    
    pub name: Option<String>,
}

impl client::Part for ConversionWorkspaceInfo {}


/// Request message for ‘ConvertConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces convert projects](ProjectLocationConversionWorkspaceConvertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertConversionWorkspaceRequest {
    /// Optional. Specifies whether the conversion workspace is to be committed automatically after the conversion.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// Optional. Automatically convert the full entity path for each entity specified by the filter. For example, if the filter specifies a table, that table schema (and database if there is one) will also be converted.
    #[serde(rename="convertFullPath")]
    
    pub convert_full_path: Option<bool>,
    /// Optional. Filter the entities to convert. Leaving this field empty will convert all of the entities. Supports Google AIP-160 style filtering.
    
    pub filter: Option<String>,
}

impl client::RequestValue for ConvertConversionWorkspaceRequest {}


/// Details regarding a Convert background job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertJobDetails {
    /// Output only. AIP-160 based filter used to specify the entities to convert
    
    pub filter: Option<String>,
}

impl client::Part for ConvertJobDetails {}


/// Options to configure rule type ConvertROWIDToColumn. The rule is used to add column rowid to destination tables based on an Oracle rowid function/property. The rule filter field can refer to one or more entities. The rule scope can be one of: Table. This rule requires additional filter to be specified beyond the basic rule filter field, which is whether or not to work on tables which already have a primary key defined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConvertRowIdToColumn {
    /// Required. Only work on tables without primary key defined
    #[serde(rename="onlyIfNoPrimaryKey")]
    
    pub only_if_no_primary_key: Option<bool>,
}

impl client::Part for ConvertRowIdToColumn {}


/// Data cache is an optional feature available for Cloud SQL for MySQL Enterprise Plus edition only. For more information on data cache, see [Data cache overview](https://cloud.google.com/sql/help/mysql-data-cache) in Cloud SQL documentation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataCacheConfig {
    /// Optional. Whether data cache is enabled for the instance.
    #[serde(rename="dataCacheEnabled")]
    
    pub data_cache_enabled: Option<bool>,
}

impl client::Part for DataCacheConfig {}


/// The type and version of a source or destination database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseEngineInfo {
    /// Required. Engine type.
    
    pub engine: Option<DatabaseEngineInfoEngineEnum>,
    /// Required. Engine version, for example "12.c.1".
    
    pub version: Option<String>,
}

impl client::Part for DatabaseEngineInfo {}


/// The base entity type for all the database related entities. The message contains the entity name, the name of its parent, the entity type, and the specific details per entity type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseEntity {
    /// Database.
    
    pub database: Option<DatabaseInstanceEntity>,
    /// Function.
    #[serde(rename="databaseFunction")]
    
    pub database_function: Option<FunctionEntity>,
    /// Package.
    #[serde(rename="databasePackage")]
    
    pub database_package: Option<PackageEntity>,
    /// Details about the entity DDL script. Multiple DDL scripts are provided for child entities such as a table entity will have one DDL for the table with additional DDLs for each index, constraint and such.
    #[serde(rename="entityDdl")]
    
    pub entity_ddl: Option<Vec<EntityDdl>>,
    /// The type of the database entity (table, view, index, ...).
    #[serde(rename="entityType")]
    
    pub entity_type: Option<DatabaseEntityEntityTypeEnum>,
    /// Details about the various issues found for the entity.
    
    pub issues: Option<Vec<EntityIssue>>,
    /// Details about entity mappings. For source tree entities, this holds the draft entities which were generated by the mapping rules. For draft tree entities, this holds the source entities which were converted to form the draft entity. Destination entities will have no mapping details.
    
    pub mappings: Option<Vec<EntityMapping>>,
    /// Materialized view.
    #[serde(rename="materializedView")]
    
    pub materialized_view: Option<MaterializedViewEntity>,
    /// The full name of the parent entity (e.g. schema name).
    #[serde(rename="parentEntity")]
    
    pub parent_entity: Option<String>,
    /// Schema.
    
    pub schema: Option<SchemaEntity>,
    /// Sequence.
    
    pub sequence: Option<SequenceEntity>,
    /// The short name (e.g. table name) of the entity.
    #[serde(rename="shortName")]
    
    pub short_name: Option<String>,
    /// Stored procedure.
    #[serde(rename="storedProcedure")]
    
    pub stored_procedure: Option<StoredProcedureEntity>,
    /// Synonym.
    
    pub synonym: Option<SynonymEntity>,
    /// Table.
    
    pub table: Option<TableEntity>,
    /// The type of tree the entity belongs to.
    
    pub tree: Option<DatabaseEntityTreeEnum>,
    /// UDT.
    
    pub udt: Option<UDTEntity>,
    /// View.
    
    pub view: Option<ViewEntity>,
}

impl client::Part for DatabaseEntity {}


/// DatabaseInstance acts as a parent entity to other database entities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseInstanceEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
}

impl client::Part for DatabaseInstanceEntity {}


/// A message defining the database engine and provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseType {
    /// The database engine.
    
    pub engine: Option<DatabaseTypeEngineEnum>,
    /// The database provider.
    
    pub provider: Option<DatabaseTypeProviderEnum>,
}

impl client::Part for DatabaseType {}


/// Request message for ‘DemoteDestination’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs demote destination projects](ProjectLocationMigrationJobDemoteDestinationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DemoteDestinationRequest { _never_set: Option<bool> }

impl client::RequestValue for DemoteDestinationRequest {}


/// Response message for ‘DescribeConversionWorkspaceRevisions’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces describe conversion workspace revisions projects](ProjectLocationConversionWorkspaceDescribeConversionWorkspaceRevisionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DescribeConversionWorkspaceRevisionsResponse {
    /// The list of conversion workspace revisions.
    
    pub revisions: Option<Vec<ConversionWorkspace>>,
}

impl client::ResponseResult for DescribeConversionWorkspaceRevisionsResponse {}


/// Response message for ‘DescribeDatabaseEntities’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces describe database entities projects](ProjectLocationConversionWorkspaceDescribeDatabaseEntityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DescribeDatabaseEntitiesResponse {
    /// The list of database entities for the conversion workspace.
    #[serde(rename="databaseEntities")]
    
    pub database_entities: Option<Vec<DatabaseEntity>>,
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for DescribeDatabaseEntitiesResponse {}


/// Filter based on relation between source value and compare value of type double in ConditionalColumnSetValue
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleComparisonFilter {
    /// Required. Double compare value to be used
    
    pub value: Option<f64>,
    /// Required. Relation between source value and compare value
    #[serde(rename="valueComparison")]
    
    pub value_comparison: Option<DoubleComparisonFilterValueComparisonEnum>,
}

impl client::Part for DoubleComparisonFilter {}


/// Dump flag definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DumpFlag {
    /// The name of the flag
    
    pub name: Option<String>,
    /// The value of the flag.
    
    pub value: Option<String>,
}

impl client::Part for DumpFlag {}


/// Dump flags definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DumpFlags {
    /// The flags for the initial dump.
    #[serde(rename="dumpFlags")]
    
    pub dump_flags: Option<Vec<DumpFlag>>,
}

impl client::Part for DumpFlags {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces mapping rules delete projects](ProjectLocationConversionWorkspaceMappingRuleDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// EncryptionConfig describes the encryption config of a cluster that is encrypted with a CMEK (customer-managed encryption key).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The fully-qualified resource name of the KMS key. Each Cloud KMS key is regionalized and has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// A single DDL statement for a specific entity
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityDdl {
    /// The actual ddl code.
    
    pub ddl: Option<String>,
    /// Type of DDL (Create, Alter).
    #[serde(rename="ddlType")]
    
    pub ddl_type: Option<String>,
    /// The name of the database entity the ddl refers to.
    
    pub entity: Option<String>,
    /// The entity type (if the DDL is for a sub entity).
    #[serde(rename="entityType")]
    
    pub entity_type: Option<EntityDdlEntityTypeEnum>,
    /// EntityIssues found for this ddl.
    #[serde(rename="issueId")]
    
    pub issue_id: Option<Vec<String>>,
}

impl client::Part for EntityDdl {}


/// Issue related to the entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityIssue {
    /// Error/Warning code
    
    pub code: Option<String>,
    /// The ddl which caused the issue, if relevant.
    
    pub ddl: Option<String>,
    /// The entity type (if the DDL is for a sub entity).
    #[serde(rename="entityType")]
    
    pub entity_type: Option<EntityIssueEntityTypeEnum>,
    /// Unique Issue ID.
    
    pub id: Option<String>,
    /// Issue detailed message
    
    pub message: Option<String>,
    /// The position of the issue found, if relevant.
    
    pub position: Option<Position>,
    /// Severity of the issue
    
    pub severity: Option<EntityIssueSeverityEnum>,
    /// The type of the issue.
    #[serde(rename="type")]
    
    pub type_: Option<EntityIssueTypeEnum>,
}

impl client::Part for EntityIssue {}


/// Details of the mappings of a database entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMapping {
    /// Target entity full name. The draft entity can also include a column, index or constraint using the same naming notation schema.table.column.
    #[serde(rename="draftEntity")]
    
    pub draft_entity: Option<String>,
    /// Type of draft entity.
    #[serde(rename="draftType")]
    
    pub draft_type: Option<EntityMappingDraftTypeEnum>,
    /// Entity mapping log entries. Multiple rules can be effective and contribute changes to a converted entity, such as a rule can handle the entity name, another rule can handle an entity type. In addition, rules which did not change the entity are also logged along with the reason preventing them to do so.
    #[serde(rename="mappingLog")]
    
    pub mapping_log: Option<Vec<EntityMappingLogEntry>>,
    /// Source entity full name. The source entity can also be a column, index or constraint using the same naming notation schema.table.column.
    #[serde(rename="sourceEntity")]
    
    pub source_entity: Option<String>,
    /// Type of source entity.
    #[serde(rename="sourceType")]
    
    pub source_type: Option<EntityMappingSourceTypeEnum>,
}

impl client::Part for EntityMapping {}


/// A single record of a rule which was used for a mapping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMappingLogEntry {
    /// Comment.
    #[serde(rename="mappingComment")]
    
    pub mapping_comment: Option<String>,
    /// Which rule caused this log entry.
    #[serde(rename="ruleId")]
    
    pub rule_id: Option<String>,
    /// Rule revision ID.
    #[serde(rename="ruleRevisionId")]
    
    pub rule_revision_id: Option<String>,
}

impl client::Part for EntityMappingLogEntry {}


/// Options to configure rule type EntityMove. The rule is used to move an entity to a new schema. The rule filter field can refer to one or more entities. The rule scope can be one of: Table, Column, Constraint, Index, View, Function, Stored Procedure, Materialized View, Sequence, UDT
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMove {
    /// Required. The new schema
    #[serde(rename="newSchema")]
    
    pub new_schema: Option<String>,
}

impl client::Part for EntityMove {}


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


/// Response message for a ‘FetchStaticIps’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations fetch static ips projects](ProjectLocationFetchStaticIpCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FetchStaticIpsResponse {
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of static IPs.
    #[serde(rename="staticIps")]
    
    pub static_ips: Option<Vec<String>>,
}

impl client::ResponseResult for FetchStaticIpsResponse {}


/// Options to configure rule type FilterTableColumns. The rule is used to filter the list of columns to include or exclude from a table. The rule filter field can refer to one entity. The rule scope can be: Table Only one of the two lists can be specified for the rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterTableColumns {
    /// Optional. List of columns to be excluded for a particular table.
    #[serde(rename="excludeColumns")]
    
    pub exclude_columns: Option<Vec<String>>,
    /// Optional. List of columns to be included for a particular table.
    #[serde(rename="includeColumns")]
    
    pub include_columns: Option<Vec<String>>,
}

impl client::Part for FilterTableColumns {}


/// Forward SSH Tunnel connectivity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ForwardSshTunnelConnectivity {
    /// Required. Hostname for the SSH tunnel.
    
    pub hostname: Option<String>,
    /// Input only. SSH password.
    
    pub password: Option<String>,
    /// Port for the SSH tunnel, default value is 22.
    
    pub port: Option<i32>,
    /// Input only. SSH private key.
    #[serde(rename="privateKey")]
    
    pub private_key: Option<String>,
    /// Required. Username for the SSH tunnel.
    
    pub username: Option<String>,
}

impl client::Part for ForwardSshTunnelConnectivity {}


/// Function's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FunctionEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the function.
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for FunctionEntity {}


/// Request message for ‘GenerateSshScript’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs generate ssh script projects](ProjectLocationMigrationJobGenerateSshScriptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateSshScriptRequest {
    /// Required. Bastion VM Instance name to use or to create.
    
    pub vm: Option<String>,
    /// The VM creation configuration
    #[serde(rename="vmCreationConfig")]
    
    pub vm_creation_config: Option<VmCreationConfig>,
    /// The port that will be open on the bastion host.
    #[serde(rename="vmPort")]
    
    pub vm_port: Option<i32>,
    /// The VM selection configuration
    #[serde(rename="vmSelectionConfig")]
    
    pub vm_selection_config: Option<VmSelectionConfig>,
}

impl client::RequestValue for GenerateSshScriptRequest {}


/// Request message for ‘GenerateTcpProxyScript’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs generate tcp proxy script projects](ProjectLocationMigrationJobGenerateTcpProxyScriptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTcpProxyScriptRequest {
    /// Required. The type of the Compute instance that will host the proxy.
    #[serde(rename="vmMachineType")]
    
    pub vm_machine_type: Option<String>,
    /// Required. The name of the Compute instance that will host the proxy.
    #[serde(rename="vmName")]
    
    pub vm_name: Option<String>,
    /// Required. The name of the subnet the Compute instance will use for private connectivity. Must be supplied in the form of projects/{project}/regions/{region}/subnetworks/{subnetwork}. Note: the region for the subnet must match the Compute instance region.
    #[serde(rename="vmSubnet")]
    
    pub vm_subnet: Option<String>,
    /// Optional. The Google Cloud Platform zone to create the VM in. The fully qualified name of the zone must be specified, including the region name, for example "us-central1-b". If not specified, uses the "-b" zone of the destination Connection Profile's region.
    #[serde(rename="vmZone")]
    
    pub vm_zone: Option<String>,
}

impl client::RequestValue for GenerateTcpProxyScriptRequest {}


/// Request message for ‘ImportMappingRules’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces mapping rules import projects](ProjectLocationConversionWorkspaceMappingRuleImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportMappingRulesRequest {
    /// Required. Should the conversion workspace be committed automatically after the import operation.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// Required. One or more rules files.
    #[serde(rename="rulesFiles")]
    
    pub rules_files: Option<Vec<RulesFile>>,
    /// Required. The format of the rules content file.
    #[serde(rename="rulesFormat")]
    
    pub rules_format: Option<ImportMappingRulesRequestRulesFormatEnum>,
}

impl client::RequestValue for ImportMappingRulesRequest {}


/// Details regarding an Import Rules background job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportRulesJobDetails {
    /// Output only. The requested file format.
    #[serde(rename="fileFormat")]
    
    pub file_format: Option<ImportRulesJobDetailFileFormatEnum>,
    /// Output only. File names used for the import rules job.
    
    pub files: Option<Vec<String>>,
}

impl client::Part for ImportRulesJobDetails {}


/// Index is not used as an independent entity, it is retrieved as part of a Table entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the index.
    
    pub name: Option<String>,
    /// Table columns used as part of the Index, for example B-TREE index should list the columns which constitutes the index.
    #[serde(rename="tableColumns")]
    
    pub table_columns: Option<Vec<String>>,
    /// Type of index, for example B-TREE.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Boolean value indicating whether the index is unique.
    
    pub unique: Option<bool>,
}

impl client::Part for IndexEntity {}


/// Filter based on relation between source value and compare value of type integer in ConditionalColumnSetValue
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntComparisonFilter {
    /// Required. Integer compare value to be used
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
    /// Required. Relation between source value and compare value
    #[serde(rename="valueComparison")]
    
    pub value_comparison: Option<IntComparisonFilterValueComparisonEnum>,
}

impl client::Part for IntComparisonFilter {}


/// Response message for ‘ListConnectionProfiles’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles list projects](ProjectLocationConnectionProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConnectionProfilesResponse {
    /// The response list of connection profiles.
    #[serde(rename="connectionProfiles")]
    
    pub connection_profiles: Option<Vec<ConnectionProfile>>,
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConnectionProfilesResponse {}


/// Response message for ‘ListConversionWorkspaces’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces list projects](ProjectLocationConversionWorkspaceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConversionWorkspacesResponse {
    /// The list of conversion workspace objects.
    #[serde(rename="conversionWorkspaces")]
    
    pub conversion_workspaces: Option<Vec<ConversionWorkspace>>,
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListConversionWorkspacesResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// Response message for ‘ListMappingRulesRequest’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces mapping rules list projects](ProjectLocationConversionWorkspaceMappingRuleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMappingRulesResponse {
    /// The list of conversion workspace mapping rules.
    #[serde(rename="mappingRules")]
    
    pub mapping_rules: Option<Vec<MappingRule>>,
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMappingRulesResponse {}


/// Response message for ‘ListMigrationJobs’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs list projects](ProjectLocationMigrationJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMigrationJobsResponse {
    /// The list of migration jobs objects.
    #[serde(rename="migrationJobs")]
    
    pub migration_jobs: Option<Vec<MigrationJob>>,
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListMigrationJobsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
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


/// Response message for ‘ListPrivateConnections’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations private connections list projects](ProjectLocationPrivateConnectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPrivateConnectionsResponse {
    /// A token which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of private connections.
    #[serde(rename="privateConnections")]
    
    pub private_connections: Option<Vec<PrivateConnection>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListPrivateConnectionsResponse {}


/// A resource that represents a Google Cloud location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// MachineConfig describes the configuration of a machine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MachineConfig {
    /// The number of CPU's in the VM instance.
    #[serde(rename="cpuCount")]
    
    pub cpu_count: Option<i32>,
}

impl client::Part for MachineConfig {}


/// Definition of a transformation that is to be applied to a group of entities in the source schema. Several such transformations can be applied to an entity sequentially to define the corresponding entity in the target schema.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces mapping rules create projects](ProjectLocationConversionWorkspaceMappingRuleCreateCall) (request|response)
/// * [locations conversion workspaces mapping rules get projects](ProjectLocationConversionWorkspaceMappingRuleGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MappingRule {
    /// Optional. Rule to specify how the data contained in a column should be transformed (such as trimmed, rounded, etc) provided that the data meets certain criteria.
    #[serde(rename="conditionalColumnSetValue")]
    
    pub conditional_column_set_value: Option<ConditionalColumnSetValue>,
    /// Optional. Rule to specify how multiple tables should be converted with an additional rowid column.
    #[serde(rename="convertRowidColumn")]
    
    pub convert_rowid_column: Option<ConvertRowIdToColumn>,
    /// Optional. A human readable name
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Optional. Rule to specify how multiple entities should be relocated into a different schema.
    #[serde(rename="entityMove")]
    
    pub entity_move: Option<EntityMove>,
    /// Required. The rule filter
    
    pub filter: Option<MappingRuleFilter>,
    /// Optional. Rule to specify the list of columns to include or exclude from a table.
    #[serde(rename="filterTableColumns")]
    
    pub filter_table_columns: Option<FilterTableColumns>,
    /// Optional. Rule to specify how multiple columns should be converted to a different data type.
    #[serde(rename="multiColumnDataTypeChange")]
    
    pub multi_column_data_type_change: Option<MultiColumnDatatypeChange>,
    /// Optional. Rule to specify how multiple entities should be renamed.
    #[serde(rename="multiEntityRename")]
    
    pub multi_entity_rename: Option<MultiEntityRename>,
    /// Full name of the mapping rule resource, in the form of: projects/{project}/locations/{location}/conversionWorkspaces/{set}/mappingRule/{rule}.
    
    pub name: Option<String>,
    /// Output only. The timestamp that the revision was created.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The revision ID of the mapping rule. A new revision is committed whenever the mapping rule is changed in any way. The format is an 8-character hexadecimal string.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Required. The order in which the rule is applied. Lower order rules are applied before higher value rules so they may end up being overridden.
    #[serde(rename="ruleOrder")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rule_order: Option<i64>,
    /// Required. The rule scope
    #[serde(rename="ruleScope")]
    
    pub rule_scope: Option<MappingRuleRuleScopeEnum>,
    /// Optional. Rule to specify the primary key for a table
    #[serde(rename="setTablePrimaryKey")]
    
    pub set_table_primary_key: Option<SetTablePrimaryKey>,
    /// Optional. Rule to specify how a single column is converted.
    #[serde(rename="singleColumnChange")]
    
    pub single_column_change: Option<SingleColumnChange>,
    /// Optional. Rule to specify how a single entity should be renamed.
    #[serde(rename="singleEntityRename")]
    
    pub single_entity_rename: Option<SingleEntityRename>,
    /// Optional. Rule to specify how a single package is converted.
    #[serde(rename="singlePackageChange")]
    
    pub single_package_change: Option<SinglePackageChange>,
    /// Optional. Rule to change the sql code for an entity, for example, function, procedure.
    #[serde(rename="sourceSqlChange")]
    
    pub source_sql_change: Option<SourceSqlChange>,
    /// Optional. The mapping rule state
    
    pub state: Option<MappingRuleStateEnum>,
}

impl client::RequestValue for MappingRule {}
impl client::ResponseResult for MappingRule {}


/// A filter defining the entities that a mapping rule should be applied to. When more than one field is specified, the rule is applied only to entities which match all the fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MappingRuleFilter {
    /// Optional. The rule should be applied to specific entities defined by their fully qualified names.
    
    pub entities: Option<Vec<String>>,
    /// Optional. The rule should be applied to entities whose non-qualified name contains the given string.
    #[serde(rename="entityNameContains")]
    
    pub entity_name_contains: Option<String>,
    /// Optional. The rule should be applied to entities whose non-qualified name starts with the given prefix.
    #[serde(rename="entityNamePrefix")]
    
    pub entity_name_prefix: Option<String>,
    /// Optional. The rule should be applied to entities whose non-qualified name ends with the given suffix.
    #[serde(rename="entityNameSuffix")]
    
    pub entity_name_suffix: Option<String>,
    /// Optional. The rule should be applied to entities whose parent entity (fully qualified name) matches the given value. For example, if the rule applies to a table entity, the expected value should be a schema (schema). If the rule applies to a column or index entity, the expected value can be either a schema (schema) or a table (schema.table)
    #[serde(rename="parentEntity")]
    
    pub parent_entity: Option<String>,
}

impl client::Part for MappingRuleFilter {}


/// MaterializedView's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedViewEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the view.
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for MaterializedViewEntity {}


/// Represents a Database Migration Service migration job object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs create projects](ProjectLocationMigrationJobCreateCall) (request)
/// * [locations migration jobs get projects](ProjectLocationMigrationJobGetCall) (response)
/// * [locations migration jobs patch projects](ProjectLocationMigrationJobPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MigrationJob {
    /// The CMEK (customer-managed encryption key) fully qualified key name used for the migration job. This field supports all migration jobs types except for: * Mysql to Mysql (use the cmek field in the cloudsql connection profile instead). * PostrgeSQL to PostgreSQL (use the cmek field in the cloudsql connection profile instead). * PostgreSQL to AlloyDB (use the kms_key_name field in the alloydb connection profile instead). Each Cloud CMEK key has the following format: projects/[PROJECT]/locations/[REGION]/keyRings/[RING]/cryptoKeys/[KEY_NAME]
    #[serde(rename="cmekKeyName")]
    
    pub cmek_key_name: Option<String>,
    /// The conversion workspace used by the migration.
    #[serde(rename="conversionWorkspace")]
    
    pub conversion_workspace: Option<ConversionWorkspaceInfo>,
    /// Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The resource name (URI) of the destination connection profile.
    
    pub destination: Option<String>,
    /// The database engine type and provider of the destination.
    #[serde(rename="destinationDatabase")]
    
    pub destination_database: Option<DatabaseType>,
    /// The migration job display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The initial dump flags. This field and the "dump_path" field are mutually exclusive.
    #[serde(rename="dumpFlags")]
    
    pub dump_flags: Option<DumpFlags>,
    /// The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]). This field and the "dump_flags" field are mutually exclusive.
    #[serde(rename="dumpPath")]
    
    pub dump_path: Option<String>,
    /// Optional. The type of the data dump. Supported for MySQL to CloudSQL for MySQL migrations only.
    #[serde(rename="dumpType")]
    
    pub dump_type: Option<MigrationJobDumpTypeEnum>,
    /// Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Output only. If the migration job is completed, the time when it was completed.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The error details in case of state FAILED.
    
    pub error: Option<Status>,
    /// This field can be used to select the entities to migrate as part of the migration job. It uses AIP-160 notation to select a subset of the entities configured on the associated conversion-workspace. This field should not be set on migration-jobs that are not associated with a conversion workspace.
    
    pub filter: Option<String>,
    /// The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    
    pub labels: Option<HashMap<String, String>>,
    /// The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/migrationJobs/{migrationJob}.
    
    pub name: Option<String>,
    /// Optional. Data dump parallelism settings used by the migration. Currently applicable only for MySQL to Cloud SQL for MySQL migrations only.
    #[serde(rename="performanceConfig")]
    
    pub performance_config: Option<PerformanceConfig>,
    /// Output only. The current migration job phase.
    
    pub phase: Option<MigrationJobPhaseEnum>,
    /// The details needed to communicate to the source over Reverse SSH tunnel connectivity.
    #[serde(rename="reverseSshConnectivity")]
    
    pub reverse_ssh_connectivity: Option<ReverseSshConnectivity>,
    /// Required. The resource name (URI) of the source connection profile.
    
    pub source: Option<String>,
    /// The database engine type and provider of the source.
    #[serde(rename="sourceDatabase")]
    
    pub source_database: Option<DatabaseType>,
    /// Optional. Configuration for SQL Server homogeneous migration.
    #[serde(rename="sqlserverHomogeneousMigrationJobConfig")]
    
    pub sqlserver_homogeneous_migration_job_config: Option<SqlServerHomogeneousMigrationJobConfig>,
    /// The current migration job state.
    
    pub state: Option<MigrationJobStateEnum>,
    /// static ip connectivity data (default, no additional details needed).
    #[serde(rename="staticIpConnectivity")]
    
    pub static_ip_connectivity: Option<StaticIpConnectivity>,
    /// Required. The migration job type.
    #[serde(rename="type")]
    
    pub type_: Option<MigrationJobTypeEnum>,
    /// Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The details of the VPC network that the source database is located in.
    #[serde(rename="vpcPeeringConnectivity")]
    
    pub vpc_peering_connectivity: Option<VpcPeeringConnectivity>,
}

impl client::RequestValue for MigrationJob {}
impl client::ResponseResult for MigrationJob {}


/// Options to configure rule type MultiColumnDatatypeChange. The rule is used to change the data type and associated properties of multiple columns at once. The rule filter field can refer to one or more entities. The rule scope can be one of:Column. This rule requires additional filters to be specified beyond the basic rule filter field, which is the source data type, but the rule supports additional filtering capabilities such as the minimum and maximum field length. All additional filters which are specified are required to be met in order for the rule to be applied (logical AND between the fields).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiColumnDatatypeChange {
    /// Optional. Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Required. New data type.
    #[serde(rename="newDataType")]
    
    pub new_data_type: Option<String>,
    /// Optional. Column fractional seconds precision - used only for timestamp based datatypes - if not specified and relevant uses the source column fractional seconds precision.
    #[serde(rename="overrideFractionalSecondsPrecision")]
    
    pub override_fractional_seconds_precision: Option<i32>,
    /// Optional. Column length - e.g. varchar (50) - if not specified and relevant uses the source column length.
    #[serde(rename="overrideLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub override_length: Option<i64>,
    /// Optional. Column precision - when relevant - if not specified and relevant uses the source column precision.
    #[serde(rename="overridePrecision")]
    
    pub override_precision: Option<i32>,
    /// Optional. Column scale - when relevant - if not specified and relevant uses the source column scale.
    #[serde(rename="overrideScale")]
    
    pub override_scale: Option<i32>,
    /// Required. Filter on source data type.
    #[serde(rename="sourceDataTypeFilter")]
    
    pub source_data_type_filter: Option<String>,
    /// Optional. Filter for fixed point number data types such as NUMERIC/NUMBER.
    #[serde(rename="sourceNumericFilter")]
    
    pub source_numeric_filter: Option<SourceNumericFilter>,
    /// Optional. Filter for text-based data types like varchar.
    #[serde(rename="sourceTextFilter")]
    
    pub source_text_filter: Option<SourceTextFilter>,
}

impl client::Part for MultiColumnDatatypeChange {}


/// Options to configure rule type MultiEntityRename. The rule is used to rename multiple entities. The rule filter field can refer to one or more entities. The rule scope can be one of: Database, Schema, Table, Column, Constraint, Index, View, Function, Stored Procedure, Materialized View, Sequence, UDT
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiEntityRename {
    /// Optional. The pattern used to generate the new entity's name. This pattern must include the characters '{name}', which will be replaced with the name of the original entity. For example, the pattern 't_{name}' for an entity name jobs would be converted to 't_jobs'. If unspecified, the default value for this field is '{name}'
    #[serde(rename="newNamePattern")]
    
    pub new_name_pattern: Option<String>,
    /// Optional. Additional transformation that can be done on the source entity name before it is being used by the new_name_pattern, for example lower case. If no transformation is desired, use NO_TRANSFORMATION
    #[serde(rename="sourceNameTransformation")]
    
    pub source_name_transformation: Option<MultiEntityRenameSourceNameTransformationEnum>,
}

impl client::Part for MultiEntityRename {}


/// Specifies connection parameters required specifically for MySQL databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MySqlConnectionProfile {
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Required. The IP or hostname of the source MySQL database.
    
    pub host: Option<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates If this connection profile password is stored.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source MySQL database.
    
    pub port: Option<i32>,
    /// SSL configuration for the destination to connect to the source database.
    
    pub ssl: Option<SslConfig>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for MySqlConnectionProfile {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles create projects](ProjectLocationConnectionProfileCreateCall) (response)
/// * [locations connection profiles delete projects](ProjectLocationConnectionProfileDeleteCall) (response)
/// * [locations connection profiles patch projects](ProjectLocationConnectionProfilePatchCall) (response)
/// * [locations conversion workspaces mapping rules import projects](ProjectLocationConversionWorkspaceMappingRuleImportCall) (response)
/// * [locations conversion workspaces apply projects](ProjectLocationConversionWorkspaceApplyCall) (response)
/// * [locations conversion workspaces commit projects](ProjectLocationConversionWorkspaceCommitCall) (response)
/// * [locations conversion workspaces convert projects](ProjectLocationConversionWorkspaceConvertCall) (response)
/// * [locations conversion workspaces create projects](ProjectLocationConversionWorkspaceCreateCall) (response)
/// * [locations conversion workspaces delete projects](ProjectLocationConversionWorkspaceDeleteCall) (response)
/// * [locations conversion workspaces patch projects](ProjectLocationConversionWorkspacePatchCall) (response)
/// * [locations conversion workspaces rollback projects](ProjectLocationConversionWorkspaceRollbackCall) (response)
/// * [locations conversion workspaces seed projects](ProjectLocationConversionWorkspaceSeedCall) (response)
/// * [locations migration jobs create projects](ProjectLocationMigrationJobCreateCall) (response)
/// * [locations migration jobs delete projects](ProjectLocationMigrationJobDeleteCall) (response)
/// * [locations migration jobs demote destination projects](ProjectLocationMigrationJobDemoteDestinationCall) (response)
/// * [locations migration jobs patch projects](ProjectLocationMigrationJobPatchCall) (response)
/// * [locations migration jobs promote projects](ProjectLocationMigrationJobPromoteCall) (response)
/// * [locations migration jobs restart projects](ProjectLocationMigrationJobRestartCall) (response)
/// * [locations migration jobs resume projects](ProjectLocationMigrationJobResumeCall) (response)
/// * [locations migration jobs start projects](ProjectLocationMigrationJobStartCall) (response)
/// * [locations migration jobs stop projects](ProjectLocationMigrationJobStopCall) (response)
/// * [locations migration jobs verify projects](ProjectLocationMigrationJobVerifyCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations private connections create projects](ProjectLocationPrivateConnectionCreateCall) (response)
/// * [locations private connections delete projects](ProjectLocationPrivateConnectionDeleteCall) (response)
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


/// Specifies connection parameters required specifically for Oracle databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OracleConnectionProfile {
    /// Required. Database service for the Oracle connection.
    #[serde(rename="databaseService")]
    
    pub database_service: Option<String>,
    /// Forward SSH tunnel connectivity.
    #[serde(rename="forwardSshConnectivity")]
    
    pub forward_ssh_connectivity: Option<ForwardSshTunnelConnectivity>,
    /// Required. The IP or hostname of the source Oracle database.
    
    pub host: Option<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates whether a new password is included in the request.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source Oracle database.
    
    pub port: Option<i32>,
    /// Private connectivity.
    #[serde(rename="privateConnectivity")]
    
    pub private_connectivity: Option<PrivateConnectivity>,
    /// SSL configuration for the connection to the source Oracle database. * Only `SERVER_ONLY` configuration is supported for Oracle SSL. * SSL is supported for Oracle versions 12 and above.
    
    pub ssl: Option<SslConfig>,
    /// Static Service IP connectivity.
    #[serde(rename="staticServiceIpConnectivity")]
    
    pub static_service_ip_connectivity: Option<StaticServiceIpConnectivity>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for OracleConnectionProfile {}


/// Package's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the package body. If the package specification has cursors or subprograms, then the package body is mandatory.
    #[serde(rename="packageBody")]
    
    pub package_body: Option<String>,
    /// The SQL code which creates the package.
    #[serde(rename="packageSqlCode")]
    
    pub package_sql_code: Option<String>,
}

impl client::Part for PackageEntity {}


/// Performance configuration definition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Initial dump parallelism level.
    #[serde(rename="dumpParallelLevel")]
    
    pub dump_parallel_level: Option<PerformanceConfigDumpParallelLevelEnum>,
}

impl client::Part for PerformanceConfig {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles get iam policy projects](ProjectLocationConnectionProfileGetIamPolicyCall) (response)
/// * [locations connection profiles set iam policy projects](ProjectLocationConnectionProfileSetIamPolicyCall) (response)
/// * [locations conversion workspaces get iam policy projects](ProjectLocationConversionWorkspaceGetIamPolicyCall) (response)
/// * [locations conversion workspaces set iam policy projects](ProjectLocationConversionWorkspaceSetIamPolicyCall) (response)
/// * [locations migration jobs get iam policy projects](ProjectLocationMigrationJobGetIamPolicyCall) (response)
/// * [locations migration jobs set iam policy projects](ProjectLocationMigrationJobSetIamPolicyCall) (response)
/// * [locations private connections get iam policy projects](ProjectLocationPrivateConnectionGetIamPolicyCall) (response)
/// * [locations private connections set iam policy projects](ProjectLocationPrivateConnectionSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Issue position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    /// Issue column number
    
    pub column: Option<i32>,
    /// Issue length
    
    pub length: Option<i32>,
    /// Issue line number
    
    pub line: Option<i32>,
    /// Issue offset
    
    pub offset: Option<i32>,
}

impl client::Part for Position {}


/// Specifies connection parameters required specifically for PostgreSQL databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostgreSqlConnectionProfile {
    /// Optional. If the destination is an AlloyDB database, use this field to provide the AlloyDB cluster ID.
    #[serde(rename="alloydbClusterId")]
    
    pub alloydb_cluster_id: Option<String>,
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Required. The IP or hostname of the source PostgreSQL database.
    
    pub host: Option<String>,
    /// Output only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with.
    #[serde(rename="networkArchitecture")]
    
    pub network_architecture: Option<PostgreSqlConnectionProfileNetworkArchitectureEnum>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates If this connection profile password is stored.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source PostgreSQL database.
    
    pub port: Option<i32>,
    /// Private service connect connectivity.
    #[serde(rename="privateServiceConnectConnectivity")]
    
    pub private_service_connect_connectivity: Option<PrivateServiceConnectConnectivity>,
    /// SSL configuration for the destination to connect to the source database.
    
    pub ssl: Option<SslConfig>,
    /// Static ip connectivity data (default, no additional details needed).
    #[serde(rename="staticIpConnectivity")]
    
    pub static_ip_connectivity: Option<StaticIpConnectivity>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for PostgreSqlConnectionProfile {}


/// Settings for the cluster's primary instance
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrimaryInstanceSettings {
    /// Database flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used.
    #[serde(rename="databaseFlags")]
    
    pub database_flags: Option<HashMap<String, String>>,
    /// Required. The ID of the AlloyDB primary instance. The ID must satisfy the regex expression "[a-z0-9-]+".
    
    pub id: Option<String>,
    /// Labels for the AlloyDB primary instance created by DMS. An object containing a list of 'key', 'value' pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// Configuration for the machines that host the underlying database engine.
    #[serde(rename="machineConfig")]
    
    pub machine_config: Option<MachineConfig>,
    /// Output only. The private IP address for the Instance. This is the connection endpoint for an end-user application.
    #[serde(rename="privateIp")]
    
    pub private_ip: Option<String>,
}

impl client::Part for PrimaryInstanceSettings {}


/// The PrivateConnection resource is used to establish private connectivity with the customer’s network.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations private connections create projects](ProjectLocationPrivateConnectionCreateCall) (request)
/// * [locations private connections get projects](ProjectLocationPrivateConnectionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateConnection {
    /// Output only. The create time of the resource.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The private connection display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The error details in case of state FAILED.
    
    pub error: Option<Status>,
    /// The resource labels for private connections to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of "key": "value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    
    pub labels: Option<HashMap<String, String>>,
    /// The name of the resource.
    
    pub name: Option<String>,
    /// Output only. The state of the private connection.
    
    pub state: Option<PrivateConnectionStateEnum>,
    /// Output only. The last update time of the resource.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// VPC peering configuration.
    #[serde(rename="vpcPeeringConfig")]
    
    pub vpc_peering_config: Option<VpcPeeringConfig>,
}

impl client::RequestValue for PrivateConnection {}
impl client::ResponseResult for PrivateConnection {}


/// Private Connectivity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateConnectivity {
    /// Required. The resource name (URI) of the private connection.
    #[serde(rename="privateConnection")]
    
    pub private_connection: Option<String>,
}

impl client::Part for PrivateConnectivity {}


/// [Private Service Connect connectivity](https://cloud.google.com/vpc/docs/private-service-connect#service-attachments)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateServiceConnectConnectivity {
    /// Required. A service attachment that exposes a database, and has the following format: projects/{project}/regions/{region}/serviceAttachments/{service_attachment_name}
    #[serde(rename="serviceAttachment")]
    
    pub service_attachment: Option<String>,
}

impl client::Part for PrivateServiceConnectConnectivity {}


/// Request message for ‘PromoteMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs promote projects](ProjectLocationMigrationJobPromoteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromoteMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for PromoteMigrationJobRequest {}


/// Request message for ‘RestartMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs restart projects](ProjectLocationMigrationJobRestartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestartMigrationJobRequest {
    /// Optional. Restart the migration job without running prior configuration verification. Defaults to `false`.
    #[serde(rename="skipValidation")]
    
    pub skip_validation: Option<bool>,
}

impl client::RequestValue for RestartMigrationJobRequest {}


/// Request message for ‘ResumeMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs resume projects](ProjectLocationMigrationJobResumeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResumeMigrationJobRequest {
    /// Optional. Resume the migration job without running prior configuration verification. Defaults to `false`.
    #[serde(rename="skipValidation")]
    
    pub skip_validation: Option<bool>,
}

impl client::RequestValue for ResumeMigrationJobRequest {}


/// The details needed to configure a reverse SSH tunnel between the source and destination databases. These details will be used when calling the generateSshScript method (see https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs/generateSshScript) to produce the script that will help set up the reverse SSH tunnel, and to set up the VPC peering between the Cloud SQL private network and the VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReverseSshConnectivity {
    /// The name of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel.
    
    pub vm: Option<String>,
    /// Required. The IP of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel.
    #[serde(rename="vmIp")]
    
    pub vm_ip: Option<String>,
    /// Required. The forwarding port of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel.
    #[serde(rename="vmPort")]
    
    pub vm_port: Option<i32>,
    /// The name of the VPC to peer with the Cloud SQL private network.
    
    pub vpc: Option<String>,
}

impl client::Part for ReverseSshConnectivity {}


/// Request message for ‘RollbackConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces rollback projects](ProjectLocationConversionWorkspaceRollbackCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackConversionWorkspaceRequest { _never_set: Option<bool> }

impl client::RequestValue for RollbackConversionWorkspaceRequest {}


/// This allows the data to change scale, for example if the source is 2 digits after the decimal point, specify round to scale value = 2. If for example the value needs to be converted to an integer, use round to scale value = 0.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoundToScale {
    /// Required. Scale value to be used
    
    pub scale: Option<i32>,
}

impl client::Part for RoundToScale {}


/// Details of a single rules file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RulesFile {
    /// Required. The text content of the rules that needs to be converted.
    #[serde(rename="rulesContent")]
    
    pub rules_content: Option<String>,
    /// Required. The filename of the rules that needs to be converted. The filename is used mainly so that future logs of the import rules job contain it, and can therefore be searched by it.
    #[serde(rename="rulesSourceFilename")]
    
    pub rules_source_filename: Option<String>,
}

impl client::Part for RulesFile {}


/// Schema typically has no parent entity, but can have a parent entity DatabaseInstance (for database engines which support it). For some database engines, the terms schema and user can be used interchangeably when they refer to a namespace or a collection of other database entities. Can store additional information which is schema specific.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
}

impl client::Part for SchemaEntity {}


/// Response message for ‘SearchBackgroundJobs’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces search background jobs projects](ProjectLocationConversionWorkspaceSearchBackgroundJobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchBackgroundJobsResponse {
    /// The list of conversion workspace mapping rules.
    
    pub jobs: Option<Vec<BackgroundJobLogEntry>>,
}

impl client::ResponseResult for SearchBackgroundJobsResponse {}


/// Request message for ‘SeedConversionWorkspace’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations conversion workspaces seed projects](ProjectLocationConversionWorkspaceSeedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeedConversionWorkspaceRequest {
    /// Should the conversion workspace be committed automatically after the seed operation.
    #[serde(rename="autoCommit")]
    
    pub auto_commit: Option<bool>,
    /// Optional. Fully qualified (Uri) name of the destination connection profile.
    #[serde(rename="destinationConnectionProfile")]
    
    pub destination_connection_profile: Option<String>,
    /// Optional. Fully qualified (Uri) name of the source connection profile.
    #[serde(rename="sourceConnectionProfile")]
    
    pub source_connection_profile: Option<String>,
}

impl client::RequestValue for SeedConversionWorkspaceRequest {}


/// Details regarding a Seed background job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeedJobDetails {
    /// Output only. The connection profile which was used for the seed job.
    #[serde(rename="connectionProfile")]
    
    pub connection_profile: Option<String>,
}

impl client::Part for SeedJobDetails {}


/// Sequence's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SequenceEntity {
    /// Indicates number of entries to cache / precreate.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cache: Option<i64>,
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Indicates whether the sequence value should cycle through.
    
    pub cycle: Option<bool>,
    /// Increment value for the sequence.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub increment: Option<i64>,
    /// Maximum number for the sequence represented as bytes to accommodate large. numbers
    #[serde(rename="maxValue")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub max_value: Option<Vec<u8>>,
    /// Minimum number for the sequence represented as bytes to accommodate large. numbers
    #[serde(rename="minValue")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub min_value: Option<Vec<u8>>,
    /// Start number for the sequence represented as bytes to accommodate large. numbers
    #[serde(rename="startValue")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub start_value: Option<Vec<u8>>,
}

impl client::Part for SequenceEntity {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles set iam policy projects](ProjectLocationConnectionProfileSetIamPolicyCall) (request)
/// * [locations conversion workspaces set iam policy projects](ProjectLocationConversionWorkspaceSetIamPolicyCall) (request)
/// * [locations migration jobs set iam policy projects](ProjectLocationMigrationJobSetIamPolicyCall) (request)
/// * [locations private connections set iam policy projects](ProjectLocationPrivateConnectionSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Options to configure rule type SetTablePrimaryKey. The rule is used to specify the columns and name to configure/alter the primary key of a table. The rule filter field can refer to one entity. The rule scope can be one of: Table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetTablePrimaryKey {
    /// Optional. Name for the primary key
    #[serde(rename="primaryKey")]
    
    pub primary_key: Option<String>,
    /// Required. List of column names for the primary key
    #[serde(rename="primaryKeyColumns")]
    
    pub primary_key_columns: Option<Vec<String>>,
}

impl client::Part for SetTablePrimaryKey {}


/// Options to configure rule type SingleColumnChange. The rule is used to change the properties of a column. The rule filter field can refer to one entity. The rule scope can be one of: Column. When using this rule, if a field is not specified than the destination column's configuration will be the same as the one in the source column..
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SingleColumnChange {
    /// Optional. Is the column of array type.
    
    pub array: Option<bool>,
    /// Optional. The length of the array, only relevant if the column type is an array.
    #[serde(rename="arrayLength")]
    
    pub array_length: Option<i32>,
    /// Optional. Is the column auto-generated/identity.
    #[serde(rename="autoGenerated")]
    
    pub auto_generated: Option<bool>,
    /// Optional. Charset override - instead of table level charset.
    
    pub charset: Option<String>,
    /// Optional. Collation override - instead of table level collation.
    
    pub collation: Option<String>,
    /// Optional. Comment associated with the column.
    
    pub comment: Option<String>,
    /// Optional. Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Optional. Column data type name.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Optional. Column fractional seconds precision - e.g. 2 as in timestamp (2) - when relevant.
    #[serde(rename="fractionalSecondsPrecision")]
    
    pub fractional_seconds_precision: Option<i32>,
    /// Optional. Column length - e.g. 50 as in varchar (50) - when relevant.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// Optional. Is the column nullable.
    
    pub nullable: Option<bool>,
    /// Optional. Column precision - e.g. 8 as in double (8,2) - when relevant.
    
    pub precision: Option<i32>,
    /// Optional. Column scale - e.g. 2 as in double (8,2) - when relevant.
    
    pub scale: Option<i32>,
    /// Optional. Specifies the list of values allowed in the column.
    #[serde(rename="setValues")]
    
    pub set_values: Option<Vec<String>>,
    /// Optional. Is the column a UDT (User-defined Type).
    
    pub udt: Option<bool>,
}

impl client::Part for SingleColumnChange {}


/// Options to configure rule type SingleEntityRename. The rule is used to rename an entity. The rule filter field can refer to only one entity. The rule scope can be one of: Database, Schema, Table, Column, Constraint, Index, View, Function, Stored Procedure, Materialized View, Sequence, UDT, Synonym
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SingleEntityRename {
    /// Required. The new name of the destination entity
    #[serde(rename="newName")]
    
    pub new_name: Option<String>,
}

impl client::Part for SingleEntityRename {}


/// Options to configure rule type SinglePackageChange. The rule is used to alter the sql code for a package entities. The rule filter field can refer to one entity. The rule scope can be: Package
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SinglePackageChange {
    /// Optional. Sql code for package body
    #[serde(rename="packageBody")]
    
    pub package_body: Option<String>,
    /// Optional. Sql code for package description
    #[serde(rename="packageDescription")]
    
    pub package_description: Option<String>,
}

impl client::Part for SinglePackageChange {}


/// Filter for fixed point number data types such as NUMERIC/NUMBER
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceNumericFilter {
    /// Required. Enum to set the option defining the datatypes numeric filter has to be applied to
    #[serde(rename="numericFilterOption")]
    
    pub numeric_filter_option: Option<SourceNumericFilterNumericFilterOptionEnum>,
    /// Optional. The filter will match columns with precision smaller than or equal to this number.
    #[serde(rename="sourceMaxPrecisionFilter")]
    
    pub source_max_precision_filter: Option<i32>,
    /// Optional. The filter will match columns with scale smaller than or equal to this number.
    #[serde(rename="sourceMaxScaleFilter")]
    
    pub source_max_scale_filter: Option<i32>,
    /// Optional. The filter will match columns with precision greater than or equal to this number.
    #[serde(rename="sourceMinPrecisionFilter")]
    
    pub source_min_precision_filter: Option<i32>,
    /// Optional. The filter will match columns with scale greater than or equal to this number.
    #[serde(rename="sourceMinScaleFilter")]
    
    pub source_min_scale_filter: Option<i32>,
}

impl client::Part for SourceNumericFilter {}


/// Options to configure rule type SourceSqlChange. The rule is used to alter the sql code for database entities. The rule filter field can refer to one entity. The rule scope can be: StoredProcedure, Function, Trigger, View
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceSqlChange {
    /// Required. Sql code for source (stored procedure, function, trigger or view)
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for SourceSqlChange {}


/// Filter for text-based data types like varchar.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceTextFilter {
    /// Optional. The filter will match columns with length smaller than or equal to this number.
    #[serde(rename="sourceMaxLengthFilter")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub source_max_length_filter: Option<i64>,
    /// Optional. The filter will match columns with length greater than or equal to this number.
    #[serde(rename="sourceMinLengthFilter")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub source_min_length_filter: Option<i64>,
}

impl client::Part for SourceTextFilter {}


/// An entry for an Access Control list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlAclEntry {
    /// The time when this access control entry expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example: `2012-11-15T16:19:00.094Z`.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A label to identify this entry.
    
    pub label: Option<String>,
    /// Input only. The time-to-leave of this access control entry.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
    /// The allowlisted value for the access control list.
    
    pub value: Option<String>,
}

impl client::Part for SqlAclEntry {}


/// IP Management configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlIpConfig {
    /// Optional. The name of the allocated IP address range for the private IP Cloud SQL instance. This name refers to an already allocated IP range address. If set, the instance IP address will be created in the allocated range. Note that this IP address range can't be modified after the instance is created. If you change the VPC when configuring connectivity settings for the migration job, this field is not relevant.
    #[serde(rename="allocatedIpRange")]
    
    pub allocated_ip_range: Option<String>,
    /// The list of external networks that are allowed to connect to the instance using the IP. See https://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation, also known as 'slash' notation (e.g. `192.168.100.0/24`).
    #[serde(rename="authorizedNetworks")]
    
    pub authorized_networks: Option<Vec<SqlAclEntry>>,
    /// Whether the instance should be assigned an IPv4 address or not.
    #[serde(rename="enableIpv4")]
    
    pub enable_ipv4: Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, `projects/myProject/global/networks/default`. This setting can be updated, but it cannot be removed after it is set.
    #[serde(rename="privateNetwork")]
    
    pub private_network: Option<String>,
    /// Whether SSL connections over IP should be enforced or not.
    #[serde(rename="requireSsl")]
    
    pub require_ssl: Option<bool>,
}

impl client::Part for SqlIpConfig {}


/// Specifies the backup details in Cloud Storage for homogeneous migration to Cloud SQL for SQL Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerBackups {
    /// Required. The Cloud Storage bucket that stores backups for all replicated databases.
    #[serde(rename="gcsBucket")]
    
    pub gcs_bucket: Option<String>,
    /// Optional. Cloud Storage path inside the bucket that stores backups.
    #[serde(rename="gcsPrefix")]
    
    pub gcs_prefix: Option<String>,
}

impl client::Part for SqlServerBackups {}


/// Specifies connection parameters required specifically for SQL Server databases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerConnectionProfile {
    /// The backup details in Cloud Storage for homogeneous migration to Cloud SQL for SQL Server.
    
    pub backups: Option<SqlServerBackups>,
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[serde(rename="cloudSqlId")]
    
    pub cloud_sql_id: Option<String>,
    /// Forward SSH tunnel connectivity.
    #[serde(rename="forwardSshConnectivity")]
    
    pub forward_ssh_connectivity: Option<ForwardSshTunnelConnectivity>,
    /// Required. The IP or hostname of the source SQL Server database.
    
    pub host: Option<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    
    pub password: Option<String>,
    /// Output only. Indicates whether a new password is included in the request.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// Required. The network port of the source SQL Server database.
    
    pub port: Option<i32>,
    /// Private connectivity.
    #[serde(rename="privateConnectivity")]
    
    pub private_connectivity: Option<PrivateConnectivity>,
    /// Private Service Connect connectivity.
    #[serde(rename="privateServiceConnectConnectivity")]
    
    pub private_service_connect_connectivity: Option<PrivateServiceConnectConnectivity>,
    /// SSL configuration for the destination to connect to the source database.
    
    pub ssl: Option<SslConfig>,
    /// Static IP connectivity data (default, no additional details needed).
    #[serde(rename="staticIpConnectivity")]
    
    pub static_ip_connectivity: Option<StaticIpConnectivity>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    
    pub username: Option<String>,
}

impl client::Part for SqlServerConnectionProfile {}


/// Specifies the backup details for a single database in Cloud Storage for homogeneous migration to Cloud SQL for SQL Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerDatabaseBackup {
    /// Required. Name of a SQL Server database for which to define backup configuration.
    
    pub database: Option<String>,
    /// Optional. Encryption settings for the database. Required if provided database backups are encrypted. Encryption settings include path to certificate, path to certificate private key, and key password.
    #[serde(rename="encryptionOptions")]
    
    pub encryption_options: Option<SqlServerEncryptionOptions>,
}

impl client::Part for SqlServerDatabaseBackup {}


/// Encryption settings for the SQL Server database.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerEncryptionOptions {
    /// Required. Path to certificate.
    #[serde(rename="certPath")]
    
    pub cert_path: Option<String>,
    /// Required. Input only. Private key password.
    #[serde(rename="pvkPassword")]
    
    pub pvk_password: Option<String>,
    /// Required. Path to certificate private key.
    #[serde(rename="pvkPath")]
    
    pub pvk_path: Option<String>,
}

impl client::Part for SqlServerEncryptionOptions {}


/// Configuration for homogeneous migration to Cloud SQL for SQL Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SqlServerHomogeneousMigrationJobConfig {
    /// Required. Pattern that describes the default backup naming strategy. The specified pattern should ensure lexicographical order of backups. The pattern must define one of the following capture group sets: Capture group set #1 yy/yyyy - year, 2 or 4 digits mm - month number, 1-12 dd - day of month, 1-31 hh - hour of day, 00-23 mi - minutes, 00-59 ss - seconds, 00-59 Example: For backup file TestDB_20230802_155400.trn, use pattern: (?.*)_backup_(?\d{4})(?\d{2})(?\d{2})_(?\d{2})(?\d{2})(?\d{2}).trn Capture group set #2 timestamp - unix timestamp Example: For backup file TestDB.1691448254.trn, use pattern: (?.*)\.(?\d*).trn or (?.*)\.(?\d*).trn
    #[serde(rename="backupFilePattern")]
    
    pub backup_file_pattern: Option<String>,
    /// Required. Backup details per database in Cloud Storage.
    #[serde(rename="databaseBackups")]
    
    pub database_backups: Option<Vec<SqlServerDatabaseBackup>>,
}

impl client::Part for SqlServerHomogeneousMigrationJobConfig {}


/// Response message for ‘GenerateSshScript’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs generate ssh script projects](ProjectLocationMigrationJobGenerateSshScriptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SshScript {
    /// The ssh configuration script.
    
    pub script: Option<String>,
}

impl client::ResponseResult for SshScript {}


/// SSL configuration information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SslConfig {
    /// Required. Input only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate. The replica will use this certificate to verify it's connecting to the right host.
    #[serde(rename="caCertificate")]
    
    pub ca_certificate: Option<String>,
    /// Input only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.If this field is used then the 'client_key' field is mandatory.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<String>,
    /// Input only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate. If this field is used then the 'client_certificate' field is mandatory.
    #[serde(rename="clientKey")]
    
    pub client_key: Option<String>,
    /// Output only. The ssl config type according to 'client_key', 'client_certificate' and 'ca_certificate'.
    #[serde(rename="type")]
    
    pub type_: Option<SslConfigTypeEnum>,
}

impl client::Part for SslConfig {}


/// Request message for ‘StartMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs start projects](ProjectLocationMigrationJobStartCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartMigrationJobRequest {
    /// Optional. Start the migration job without running prior configuration verification. Defaults to `false`.
    #[serde(rename="skipValidation")]
    
    pub skip_validation: Option<bool>,
}

impl client::RequestValue for StartMigrationJobRequest {}


/// The source database will allow incoming connections from the public IP of the destination database. You can retrieve the public IP of the Cloud SQL instance from the Cloud SQL console or using Cloud SQL APIs. No additional configuration is required.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticIpConnectivity { _never_set: Option<bool> }

impl client::Part for StaticIpConnectivity {}


/// Static IP address connectivity configured on service project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StaticServiceIpConnectivity { _never_set: Option<bool> }

impl client::Part for StaticServiceIpConnectivity {}


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


/// Request message for ‘StopMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs stop projects](ProjectLocationMigrationJobStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StopMigrationJobRequest { _never_set: Option<bool> }

impl client::RequestValue for StopMigrationJobRequest {}


/// Stored procedure's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StoredProcedureEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the stored procedure.
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for StoredProcedureEntity {}


/// Synonym's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SynonymEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the entity for which the synonym is being created (the source).
    #[serde(rename="sourceEntity")]
    
    pub source_entity: Option<String>,
    /// The type of the entity for which the synonym is being created (usually a table or a sequence).
    #[serde(rename="sourceType")]
    
    pub source_type: Option<SynonymEntitySourceTypeEnum>,
}

impl client::Part for SynonymEntity {}


/// Table's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableEntity {
    /// Table columns.
    
    pub columns: Option<Vec<ColumnEntity>>,
    /// Comment associated with the table.
    
    pub comment: Option<String>,
    /// Table constraints.
    
    pub constraints: Option<Vec<ConstraintEntity>>,
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// Table indices.
    
    pub indices: Option<Vec<IndexEntity>>,
    /// Table triggers.
    
    pub triggers: Option<Vec<TriggerEntity>>,
}

impl client::Part for TableEntity {}


/// Response message for ‘GenerateTcpProxyScript’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs generate tcp proxy script projects](ProjectLocationMigrationJobGenerateTcpProxyScriptCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TcpProxyScript {
    /// The TCP Proxy configuration script.
    
    pub script: Option<String>,
}

impl client::ResponseResult for TcpProxyScript {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations connection profiles test iam permissions projects](ProjectLocationConnectionProfileTestIamPermissionCall) (request)
/// * [locations conversion workspaces test iam permissions projects](ProjectLocationConversionWorkspaceTestIamPermissionCall) (request)
/// * [locations migration jobs test iam permissions projects](ProjectLocationMigrationJobTestIamPermissionCall) (request)
/// * [locations private connections test iam permissions projects](ProjectLocationPrivateConnectionTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
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
/// * [locations connection profiles test iam permissions projects](ProjectLocationConnectionProfileTestIamPermissionCall) (response)
/// * [locations conversion workspaces test iam permissions projects](ProjectLocationConversionWorkspaceTestIamPermissionCall) (response)
/// * [locations migration jobs test iam permissions projects](ProjectLocationMigrationJobTestIamPermissionCall) (response)
/// * [locations private connections test iam permissions projects](ProjectLocationPrivateConnectionTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Trigger is not used as an independent entity, it is retrieved as part of a Table entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TriggerEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The name of the trigger.
    
    pub name: Option<String>,
    /// The SQL code which creates the trigger.
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
    /// Indicates when the trigger fires, for example BEFORE STATEMENT, AFTER EACH ROW.
    #[serde(rename="triggerType")]
    
    pub trigger_type: Option<String>,
    /// The DML, DDL, or database events that fire the trigger, for example INSERT, UPDATE.
    #[serde(rename="triggeringEvents")]
    
    pub triggering_events: Option<Vec<String>>,
}

impl client::Part for TriggerEntity {}


/// UDT's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UDTEntity {
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the udt body.
    #[serde(rename="udtBody")]
    
    pub udt_body: Option<String>,
    /// The SQL code which creates the udt.
    #[serde(rename="udtSqlCode")]
    
    pub udt_sql_code: Option<String>,
}

impl client::Part for UDTEntity {}


/// The username/password for a database user. Used for specifying initial users at cluster creation time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserPassword {
    /// The initial password for the user.
    
    pub password: Option<String>,
    /// Output only. Indicates if the initial_user.password field has been set.
    #[serde(rename="passwordSet")]
    
    pub password_set: Option<bool>,
    /// The database username.
    
    pub user: Option<String>,
}

impl client::Part for UserPassword {}


/// A list of values to filter by in ConditionalColumnSetValue
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueListFilter {
    /// Required. Whether to ignore case when filtering by values. Defaults to false
    #[serde(rename="ignoreCase")]
    
    pub ignore_case: Option<bool>,
    /// Required. Indicates whether the filter matches rows with values that are present in the list or those with values not present in it.
    #[serde(rename="valuePresentList")]
    
    pub value_present_list: Option<ValueListFilterValuePresentListEnum>,
    /// Required. The list to be used to filter by
    
    pub values: Option<Vec<String>>,
}

impl client::Part for ValueListFilter {}


/// Description of data transformation during migration as part of the ConditionalColumnSetValue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueTransformation {
    /// Optional. Applies a hash function on the data
    #[serde(rename="applyHash")]
    
    pub apply_hash: Option<ApplyHash>,
    /// Optional. Set to max_value - if integer or numeric, will use int.maxvalue, etc
    #[serde(rename="assignMaxValue")]
    
    pub assign_max_value: Option<Empty>,
    /// Optional. Set to min_value - if integer or numeric, will use int.minvalue, etc
    #[serde(rename="assignMinValue")]
    
    pub assign_min_value: Option<Empty>,
    /// Optional. Set to null
    #[serde(rename="assignNull")]
    
    pub assign_null: Option<Empty>,
    /// Optional. Set to a specific value (value is converted to fit the target data type)
    #[serde(rename="assignSpecificValue")]
    
    pub assign_specific_value: Option<AssignSpecificValue>,
    /// Optional. Filter on relation between source value and compare value of type double.
    #[serde(rename="doubleComparison")]
    
    pub double_comparison: Option<DoubleComparisonFilter>,
    /// Optional. Filter on relation between source value and compare value of type integer.
    #[serde(rename="intComparison")]
    
    pub int_comparison: Option<IntComparisonFilter>,
    /// Optional. Value is null
    #[serde(rename="isNull")]
    
    pub is_null: Option<Empty>,
    /// Optional. Allows the data to change scale
    #[serde(rename="roundScale")]
    
    pub round_scale: Option<RoundToScale>,
    /// Optional. Value is found in the specified list.
    #[serde(rename="valueList")]
    
    pub value_list: Option<ValueListFilter>,
}

impl client::Part for ValueTransformation {}


/// Request message for ‘VerifyMigrationJob’ request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations migration jobs verify projects](ProjectLocationMigrationJobVerifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyMigrationJobRequest {
    /// Optional. The changed migration job parameters to verify. It will not update the migration job.
    #[serde(rename="migrationJob")]
    
    pub migration_job: Option<MigrationJob>,
    /// Optional. Field mask is used to specify the changed fields to be verified. It will not update the migration job.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for VerifyMigrationJobRequest {}


/// View's parent is a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewEntity {
    /// View constraints.
    
    pub constraints: Option<Vec<ConstraintEntity>>,
    /// Custom engine specific features.
    #[serde(rename="customFeatures")]
    
    pub custom_features: Option<HashMap<String, json::Value>>,
    /// The SQL code which creates the view.
    #[serde(rename="sqlCode")]
    
    pub sql_code: Option<String>,
}

impl client::Part for ViewEntity {}


/// VM creation configuration message
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VmCreationConfig {
    /// The subnet name the vm needs to be created in.
    
    pub subnet: Option<String>,
    /// Required. VM instance machine type to create.
    #[serde(rename="vmMachineType")]
    
    pub vm_machine_type: Option<String>,
    /// The Google Cloud Platform zone to create the VM in.
    #[serde(rename="vmZone")]
    
    pub vm_zone: Option<String>,
}

impl client::Part for VmCreationConfig {}


/// VM selection configuration message
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VmSelectionConfig {
    /// Required. The Google Cloud Platform zone the VM is located.
    #[serde(rename="vmZone")]
    
    pub vm_zone: Option<String>,
}

impl client::Part for VmSelectionConfig {}


/// The VPC peering configuration is used to create VPC peering with the consumer's VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcPeeringConfig {
    /// Required. A free subnet for peering. (CIDR of /29)
    
    pub subnet: Option<String>,
    /// Required. Fully qualified name of the VPC that Database Migration Service will peer to.
    #[serde(rename="vpcName")]
    
    pub vpc_name: Option<String>,
}

impl client::Part for VpcPeeringConfig {}


/// The details of the VPC where the source database is located in Google Cloud. We will use this information to set up the VPC peering connection between Cloud SQL and this VPC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VpcPeeringConnectivity {
    /// The name of the VPC network to peer with the Cloud SQL private network.
    
    pub vpc: Option<String>,
}

impl client::Part for VpcPeeringConnectivity {}


