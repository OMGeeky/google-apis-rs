use super::*;
/// Request message for DataprocMetastore.AlterMetadataResourceLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services alter location projects](ProjectLocationServiceAlterLocationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlterMetadataResourceLocationRequest {
    /// Required. The new location URI for the metadata resource.
    #[serde(rename="locationUri")]
    
    pub location_uri: Option<String>,
    /// Required. The relative metadata resource name in the following format.databases/{database_id} or databases/{database_id}/tables/{table_id} or databases/{database_id}/tables/{table_id}/partitions/{partition_id}
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::RequestValue for AlterMetadataResourceLocationRequest {}


/// Request message for DataprocMetastore.AlterTableProperties.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services alter table properties projects](ProjectLocationServiceAlterTablePropertyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlterTablePropertiesRequest {
    /// A map that describes the desired values to mutate. If update_mask is empty, the properties will not update. Otherwise, the properties only alters the value whose associated paths exist in the update mask
    
    pub properties: Option<HashMap<String, String>>,
    /// Required. The name of the table containing the properties you're altering in the following format.databases/{database_id}/tables/{table_id}
    #[serde(rename="tableName")]
    
    pub table_name: Option<String>,
    /// A field mask that specifies the metadata table properties that are overwritten by the update. Fields specified in the update_mask are relative to the resource (not to the full request). A field is overwritten if it is in the mask.For example, given the target properties: properties { a: 1 b: 2 } And an update properties: properties { a: 2 b: 3 c: 4 } then if the field mask is:paths: "properties.b", "properties.c"then the result will be: properties { a: 1 b: 3 c: 4 } 
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for AlterTablePropertiesRequest {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs.If there are AuditConfigs for both allServices and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted.Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, storage.googleapis.com, cloudsql.googleapis.com. allServices is a special value that covers all services.
    
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


/// Represents the autoscaling configuration of a metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoscalingConfig {
    /// Optional. Whether or not autoscaling is enabled for this service.
    #[serde(rename="autoscalingEnabled")]
    
    pub autoscaling_enabled: Option<bool>,
    /// Output only. The scaling factor of a service with autoscaling enabled.
    #[serde(rename="autoscalingFactor")]
    
    pub autoscaling_factor: Option<f32>,
    /// Optional. The LimitConfig of the service.
    #[serde(rename="limitConfig")]
    
    pub limit_config: Option<LimitConfig>,
}

impl client::Part for AutoscalingConfig {}


/// Configuration information for the auxiliary service versions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuxiliaryVersionConfig {
    /// A mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides. If keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence.
    #[serde(rename="configOverrides")]
    
    pub config_overrides: Option<HashMap<String, String>>,
    /// Output only. The network configuration contains the endpoint URI(s) of the auxiliary Hive metastore service.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NetworkConfig>,
    /// The Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version.
    
    pub version: Option<String>,
}

impl client::Part for AuxiliaryVersionConfig {}


/// Represents a backend metastore for the federation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BackendMetastore {
    /// The type of the backend metastore.
    #[serde(rename="metastoreType")]
    
    pub metastore_type: Option<BackendMetastoreMetastoreTypeEnum>,
    /// The relative resource name of the metastore that is being federated. The formats of the relative resource names for the currently supported metastores are listed below: BigQuery projects/{project_id} Dataproc Metastore projects/{project_id}/locations/{location}/services/{service_id}
    
    pub name: Option<String>,
}

impl client::Part for BackendMetastore {}


/// The details of a backup resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services backups create projects](ProjectLocationServiceBackupCreateCall) (request)
/// * [locations services backups get projects](ProjectLocationServiceBackupGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Backup {
    /// Output only. The time when the backup was started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The description of the backup.
    
    pub description: Option<String>,
    /// Output only. The time when the backup finished creating.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The relative resource name of the backup, in the following form:projects/{project_number}/locations/{location_id}/services/{service_id}/backups/{backup_id}
    
    pub name: Option<String>,
    /// Output only. Services that are restoring from the backup.
    #[serde(rename="restoringServices")]
    
    pub restoring_services: Option<Vec<String>>,
    /// Output only. The revision of the service at the time of backup.
    #[serde(rename="serviceRevision")]
    
    pub service_revision: Option<Service>,
    /// Output only. The current state of the backup.
    
    pub state: Option<BackupStateEnum>,
}

impl client::RequestValue for Backup {}
impl client::ResponseResult for Backup {}


/// Associates members, or principals, with a role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding.If the condition evaluates to true, then this binding applies to the current request.If the condition evaluates to false, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. members can have the following values: allUsers: A special identifier that represents anyone who is on the internet; with or without a Google account. allAuthenticatedUsers: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. user:{emailid}: An email address that represents a specific Google account. For example, alice@example.com . serviceAccount:{emailid}: An email address that represents a Google service account. For example, my-other-app@appspot.gserviceaccount.com. serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]: An identifier for a Kubernetes service account (https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, my-project.svc.id.goog[my-namespace/my-kubernetes-sa]. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. domain:{domain}: The G Suite domain (primary) that represents all the users of that domain. For example, google.com or example.com. principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}: A single identity in a workforce identity pool. principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}: All workforce identities in a group. principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}: All workforce identities with a specific attribute value. principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*: All identities in a workforce identity pool. principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}: A single identity in a workload identity pool. principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}: A workload identity pool group. principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}: All identities in a workload identity pool with a certain attribute. principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*: All identities in a workload identity pool. deleted:user:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a user that has been recently deleted. For example, alice@example.com?uid=123456789012345678901. If the user is recovered, this value reverts to user:{emailid} and the recovered user retains the role in the binding. deleted:serviceAccount:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901. If the service account is undeleted, this value reverts to serviceAccount:{emailid} and the undeleted service account retains the role in the binding. deleted:group:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, admins@example.com?uid=123456789012345678901. If the group is recovered, this value reverts to group:{emailid} and the recovered group retains the role in the binding. deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}: Deleted single identity in a workforce identity pool. For example, deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of members, or principals. For example, roles/viewer, roles/editor, or roles/owner.For an overview of the IAM roles and permissions, see the IAM documentation (https://cloud.google.com/iam/docs/roles-overview). For a list of the available pre-defined roles, see here (https://cloud.google.com/iam/docs/understanding-roles).
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// Request message for DataprocMetastore.CancelMigration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services cancel migration projects](ProjectLocationServiceCancelMigrationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelMigrationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelMigrationRequest {}


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


/// Configuration information to start the Change Data Capture (CDC) streams from customer database to backend database of Dataproc Metastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CdcConfig {
    /// Optional. The bucket to write the intermediate stream event data in. The bucket name must be without any prefix like "gs://". See the bucket naming requirements (https://cloud.google.com/storage/docs/buckets#naming). This field is optional. If not set, the Artifacts Cloud Storage bucket will be used.
    
    pub bucket: Option<String>,
    /// Required. Input only. The password for the user that Datastream service should use for the MySQL connection. This field is not returned on request.
    
    pub password: Option<String>,
    /// Required. The URL of the subnetwork resource to create the VM instance hosting the reverse proxy in. More context in https://cloud.google.com/datastream/docs/private-connectivity#reverse-csql-proxy The subnetwork should reside in the network provided in the request that Datastream will peer to and should be in the same region as Datastream, in the following format. projects/{project_id}/regions/{region_id}/subnetworks/{subnetwork_id}
    #[serde(rename="reverseProxySubnet")]
    
    pub reverse_proxy_subnet: Option<String>,
    /// Optional. The root path inside the Cloud Storage bucket. The stream event data will be written to this path. The default value is /migration.
    #[serde(rename="rootPath")]
    
    pub root_path: Option<String>,
    /// Required. A /29 CIDR IP range for peering with datastream.
    #[serde(rename="subnetIpRange")]
    
    pub subnet_ip_range: Option<String>,
    /// Required. The username that the Datastream service should use for the MySQL connection.
    
    pub username: Option<String>,
    /// Required. Fully qualified name of the Cloud SQL instance's VPC network or the shared VPC network that Datastream will peer to, in the following format: projects/{project_id}/locations/global/networks/{network_id}. More context in https://cloud.google.com/datastream/docs/network-connectivity-options#privateconnectivity
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<String>,
}

impl client::Part for CdcConfig {}


/// Configuration information to establish customer database connection before the cutover phase of migration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSQLConnectionConfig {
    /// Required. The hive database name.
    #[serde(rename="hiveDatabaseName")]
    
    pub hive_database_name: Option<String>,
    /// Required. Cloud SQL database connection name (project_id:region:instance_name)
    #[serde(rename="instanceConnectionName")]
    
    pub instance_connection_name: Option<String>,
    /// Required. The private IP address of the Cloud SQL instance.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Required. The relative resource name of the subnetwork to be used for Private Service Connect. Note that this cannot be a regular subnet and is used only for NAT. (https://cloud.google.com/vpc/docs/about-vpc-hosted-services#psc-subnets) This subnet is used to publish the SOCKS5 proxy service. The subnet size must be at least /29 and it should reside in a network through which the Cloud SQL instance is accessible. The resource name should be in the format, projects/{project_id}/regions/{region_id}/subnetworks/{subnetwork_id}
    #[serde(rename="natSubnet")]
    
    pub nat_subnet: Option<String>,
    /// Required. Input only. The password for the user that Dataproc Metastore service will be using to connect to the database. This field is not returned on request.
    
    pub password: Option<String>,
    /// Required. The network port of the database.
    
    pub port: Option<i32>,
    /// Required. The relative resource name of the subnetwork to deploy the SOCKS5 proxy service in. The subnetwork should reside in a network through which the Cloud SQL instance is accessible. The resource name should be in the format, projects/{project_id}/regions/{region_id}/subnetworks/{subnetwork_id}
    #[serde(rename="proxySubnet")]
    
    pub proxy_subnet: Option<String>,
    /// Required. The username that Dataproc Metastore service will use to connect to the database.
    
    pub username: Option<String>,
}

impl client::Part for CloudSQLConnectionConfig {}


/// Configuration information for migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudSQLMigrationConfig {
    /// Required. Configuration information to start the Change Data Capture (CDC) streams from customer database to backend database of Dataproc Metastore. Dataproc Metastore switches to using its backend database after the cutover phase of migration.
    #[serde(rename="cdcConfig")]
    
    pub cdc_config: Option<CdcConfig>,
    /// Required. Configuration information to establish customer database connection before the cutover phase of migration
    #[serde(rename="cloudSqlConnectionConfig")]
    
    pub cloud_sql_connection_config: Option<CloudSQLConnectionConfig>,
}

impl client::Part for CloudSQLMigrationConfig {}


/// Request message for DataprocMetastore.CompleteMigration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services complete migration projects](ProjectLocationServiceCompleteMigrationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteMigrationRequest { _never_set: Option<bool> }

impl client::RequestValue for CompleteMigrationRequest {}


/// Contains information of the customer's network configurations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Consumer {
    /// Output only. The location of the endpoint URI. Format: projects/{project}/locations/{location}.
    #[serde(rename="endpointLocation")]
    
    pub endpoint_location: Option<String>,
    /// Output only. The URI of the endpoint used to access the metastore service.
    #[serde(rename="endpointUri")]
    
    pub endpoint_uri: Option<String>,
    /// Immutable. The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint. It is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network. There must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:projects/{project_number}/regions/{region_id}/subnetworks/{subnetwork_id}
    
    pub subnetwork: Option<String>,
}

impl client::Part for Consumer {}


/// Specifies how metastore metadata should be integrated with the Data Catalog service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataCatalogConfig {
    /// Optional. Defines whether the metastore metadata should be synced to Data Catalog. The default value is to disable syncing metastore metadata to Data Catalog.
    
    pub enabled: Option<bool>,
}

impl client::Part for DataCatalogConfig {}


/// A specification of the location of and metadata about a database dump from a relational database management system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseDump {
    /// The type of the database.
    #[serde(rename="databaseType")]
    
    pub database_type: Option<DatabaseDumpDatabaseTypeEnum>,
    /// A Cloud Storage object or folder URI that specifies the source from which to import metadata. It must begin with gs://.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
    /// The name of the source database.
    #[serde(rename="sourceDatabase")]
    
    pub source_database: Option<String>,
    /// Optional. The type of the database dump. If unspecified, defaults to MYSQL.
    #[serde(rename="type")]
    
    pub type_: Option<DatabaseDumpTypeEnum>,
}

impl client::Part for DatabaseDump {}


/// Specifies how metastore metadata should be integrated with the Dataplex service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataplexConfig {
    /// A reference to the Lake resources that this metastore service is attached to. The key is the lake resource name. Example: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    #[serde(rename="lakeResources")]
    
    pub lake_resources: Option<HashMap<String, Lake>>,
}

impl client::Part for DataplexConfig {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Encryption settings for the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The fully qualified customer provided Cloud KMS key name to use for customer data encryption, in the following format:projects/{project_number}/locations/{location_id}/keyRings/{key_ring_id}/cryptoKeys/{crypto_key_id}.
    #[serde(rename="kmsKey")]
    
    pub kms_key: Option<String>,
}

impl client::Part for EncryptionConfig {}


/// Request message for DataprocMetastore.ExportMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services export metadata projects](ProjectLocationServiceExportMetadataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportMetadataRequest {
    /// Optional. The type of the database dump. If unspecified, defaults to MYSQL.
    #[serde(rename="databaseDumpType")]
    
    pub database_dump_type: Option<ExportMetadataRequestDatabaseDumpTypeEnum>,
    /// A Cloud Storage URI of a folder, in the format gs:///. A sub-folder containing exported files will be created below it.
    #[serde(rename="destinationGcsFolder")]
    
    pub destination_gcs_folder: Option<String>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the request if it has completed. The server will ignore subsequent requests that provide a duplicate request ID for at least 60 minutes after the first request.For example, if an initial request times out, followed by another request with the same request ID, the server ignores the second request to prevent the creation of duplicate commitments.The request ID must be a valid UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier#Format). A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for ExportMetadataRequest {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec.Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
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


/// Represents a federation of multiple backend metastores.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations create projects](ProjectLocationFederationCreateCall) (request)
/// * [locations federations get projects](ProjectLocationFederationGetCall) (response)
/// * [locations federations patch projects](ProjectLocationFederationPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Federation {
    /// A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number.
    #[serde(rename="backendMetastores")]
    
    pub backend_metastores: Option<HashMap<String, BackendMetastore>>,
    /// Output only. The time when the metastore federation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The federation endpoint.
    #[serde(rename="endpointUri")]
    
    pub endpoint_uri: Option<String>,
    /// User-defined labels for the metastore federation.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The relative resource name of the federation, of the form: projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
    
    pub name: Option<String>,
    /// Output only. The current state of the federation.
    
    pub state: Option<FederationStateEnum>,
    /// Output only. Additional information about the current state of the metastore federation, if available.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
    /// Output only. The globally unique resource identifier of the metastore federation.
    
    pub uid: Option<String>,
    /// Output only. The time when the metastore federation was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version.
    
    pub version: Option<String>,
}

impl client::RequestValue for Federation {}
impl client::ResponseResult for Federation {}


/// Specifies configuration information specific to running Hive metastore software as the metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HiveMetastoreConfig {
    /// A mapping of Hive metastore version to the auxiliary version configuration. When specified, a secondary Hive metastore service is created along with the primary service. All auxiliary versions must be less than the service's primary version. The key is the auxiliary service name and it must match the regular expression a-z?. This means that the first character must be a lowercase letter, and all the following characters must be hyphens, lowercase letters, or digits, except the last character, which cannot be a hyphen.
    #[serde(rename="auxiliaryVersions")]
    
    pub auxiliary_versions: Option<HashMap<String, AuxiliaryVersionConfig>>,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml). The mappings override system defaults (some keys cannot be overridden). These overrides are also applied to auxiliary versions and can be further customized in the auxiliary version's AuxiliaryVersionConfig.
    #[serde(rename="configOverrides")]
    
    pub config_overrides: Option<HashMap<String, String>>,
    /// The protocol to use for the metastore service endpoint. If unspecified, defaults to THRIFT.
    #[serde(rename="endpointProtocol")]
    
    pub endpoint_protocol: Option<HiveMetastoreConfigEndpointProtocolEnum>,
    /// Information used to configure the Hive metastore service as a service principal in a Kerberos realm. To disable Kerberos, use the UpdateService method and specify this field's path (hive_metastore_config.kerberos_config) in the request's update_mask while omitting this field from the request's service.
    #[serde(rename="kerberosConfig")]
    
    pub kerberos_config: Option<KerberosConfig>,
    /// Immutable. The Hive metastore schema version.
    
    pub version: Option<String>,
}

impl client::Part for HiveMetastoreConfig {}


/// Configuration information for a Kerberos principal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KerberosConfig {
    /// A Kerberos keytab file that can be used to authenticate a service principal with a Kerberos Key Distribution Center (KDC).
    
    pub keytab: Option<Secret>,
    /// A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly.
    #[serde(rename="krb5ConfigGcsUri")]
    
    pub krb5_config_gcs_uri: Option<String>,
    /// A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form primary/instance@REALM, but there is no exact format.
    
    pub principal: Option<String>,
}

impl client::Part for KerberosConfig {}


/// Represents a Lake resource
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lake {
    /// The Lake resource name. Example: projects/{project_number}/locations/{location_id}/lakes/{lake_id}
    
    pub name: Option<String>,
}

impl client::Part for Lake {}


/// The details of the latest scheduled backup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatestBackup {
    /// Output only. The ID of an in-progress scheduled backup. Empty if no backup is in progress.
    #[serde(rename="backupId")]
    
    pub backup_id: Option<String>,
    /// Output only. The duration of the backup completion.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Output only. The time when the backup was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the backup.
    
    pub state: Option<LatestBackupStateEnum>,
}

impl client::Part for LatestBackup {}


/// Represents the autoscaling limit configuration of a metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LimitConfig {
    /// Optional. The highest scaling factor that the service should be autoscaled to.
    #[serde(rename="maxScalingFactor")]
    
    pub max_scaling_factor: Option<f32>,
    /// Optional. The lowest scaling factor that the service should be autoscaled to.
    #[serde(rename="minScalingFactor")]
    
    pub min_scaling_factor: Option<f32>,
}

impl client::Part for LimitConfig {}


/// Response message for DataprocMetastore.ListBackups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services backups list projects](ProjectLocationServiceBackupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBackupsResponse {
    /// The backups of the specified service.
    
    pub backups: Option<Vec<Backup>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListBackupsResponse {}


/// Response message for ListFederations
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations list projects](ProjectLocationFederationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFederationsResponse {
    /// The services in the specified location.
    
    pub federations: Option<Vec<Federation>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListFederationsResponse {}


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


/// Response message for DataprocMetastore.ListMetadataImports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services metadata imports list projects](ProjectLocationServiceMetadataImportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMetadataImportsResponse {
    /// The imports in the specified service.
    #[serde(rename="metadataImports")]
    
    pub metadata_imports: Option<Vec<MetadataImport>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListMetadataImportsResponse {}


/// Response message for DataprocMetastore.ListMigrationExecutions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services migration executions list projects](ProjectLocationServiceMigrationExecutionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMigrationExecutionsResponse {
    /// The migration executions on the specified service.
    #[serde(rename="migrationExecutions")]
    
    pub migration_executions: Option<Vec<MigrationExecution>>,
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListMigrationExecutionsResponse {}


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


/// Response message for DataprocMetastore.ListServices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services list projects](ProjectLocationServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServicesResponse {
    /// A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The services in the specified location.
    
    pub services: Option<Vec<Service>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServicesResponse {}


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
    /// The canonical id for this location. For example: "us-east1".
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1"
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Maintenance window. This specifies when Dataproc Metastore may perform system maintenance operation to the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// The day of week, when the window starts.
    #[serde(rename="dayOfWeek")]
    
    pub day_of_week: Option<MaintenanceWindowDayOfWeekEnum>,
    /// The hour of day (0-23) when the window starts.
    #[serde(rename="hourOfDay")]
    
    pub hour_of_day: Option<i32>,
}

impl client::Part for MaintenanceWindow {}


/// The details of a metadata export operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataExport {
    /// Output only. The type of the database dump.
    #[serde(rename="databaseDumpType")]
    
    pub database_dump_type: Option<MetadataExportDatabaseDumpTypeEnum>,
    /// Output only. A Cloud Storage URI of a folder that metadata are exported to, in the form of gs:////, where is automatically generated.
    #[serde(rename="destinationGcsUri")]
    
    pub destination_gcs_uri: Option<String>,
    /// Output only. The time when the export ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the export started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the export.
    
    pub state: Option<MetadataExportStateEnum>,
}

impl client::Part for MetadataExport {}


/// A metastore resource that imports metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services metadata imports create projects](ProjectLocationServiceMetadataImportCreateCall) (request)
/// * [locations services metadata imports get projects](ProjectLocationServiceMetadataImportGetCall) (response)
/// * [locations services metadata imports patch projects](ProjectLocationServiceMetadataImportPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataImport {
    /// Output only. The time when the metadata import was started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. A database dump from a pre-existing metastore's database.
    #[serde(rename="databaseDump")]
    
    pub database_dump: Option<DatabaseDump>,
    /// The description of the metadata import.
    
    pub description: Option<String>,
    /// Output only. The time when the metadata import finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The relative resource name of the metadata import, of the form:projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}.
    
    pub name: Option<String>,
    /// Output only. The current state of the metadata import.
    
    pub state: Option<MetadataImportStateEnum>,
    /// Output only. The time when the metadata import was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for MetadataImport {}
impl client::ResponseResult for MetadataImport {}


/// Specifies how metastore metadata should be integrated with external services.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataIntegration {
    /// Optional. The integration config for the Data Catalog service.
    #[serde(rename="dataCatalogConfig")]
    
    pub data_catalog_config: Option<DataCatalogConfig>,
    /// The integration config for the Dataplex service.
    #[serde(rename="dataplexConfig")]
    
    pub dataplex_config: Option<DataplexConfig>,
}

impl client::Part for MetadataIntegration {}


/// The metadata management activities of the metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataManagementActivity {
    /// Output only. The latest metadata exports of the metastore service.
    #[serde(rename="metadataExports")]
    
    pub metadata_exports: Option<Vec<MetadataExport>>,
    /// Output only. The latest restores of the metastore service.
    
    pub restores: Option<Vec<Restore>>,
}

impl client::Part for MetadataManagementActivity {}


/// The details of a migration execution resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services migration executions get projects](ProjectLocationServiceMigrationExecutionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MigrationExecution {
    /// Configuration information specific to migrating from self-managed hive metastore on Google Cloud using Cloud SQL as the backend database to Dataproc Metastore.
    #[serde(rename="cloudSqlMigrationConfig")]
    
    pub cloud_sql_migration_config: Option<CloudSQLMigrationConfig>,
    /// Output only. The time when the migration execution was started.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the migration execution finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The relative resource name of the migration execution, in the following form: projects/{project_number}/locations/{location_id}/services/{service_id}/migrationExecutions/{migration_execution_id}
    
    pub name: Option<String>,
    /// Output only. The current phase of the migration execution.
    
    pub phase: Option<MigrationExecutionPhaseEnum>,
    /// Output only. The current state of the migration execution.
    
    pub state: Option<MigrationExecutionStateEnum>,
    /// Output only. Additional information about the current state of the migration execution.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
}

impl client::ResponseResult for MigrationExecution {}


/// Request message for DataprocMetastore.MoveTableToDatabase.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services move table to database projects](ProjectLocationServiceMoveTableToDatabaseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveTableToDatabaseRequest {
    /// Required. The name of the database where the table resides.
    #[serde(rename="dbName")]
    
    pub db_name: Option<String>,
    /// Required. The name of the database where the table should be moved.
    #[serde(rename="destinationDbName")]
    
    pub destination_db_name: Option<String>,
    /// Required. The name of the table to be moved.
    #[serde(rename="tableName")]
    
    pub table_name: Option<String>,
}

impl client::RequestValue for MoveTableToDatabaseRequest {}


/// Network configuration for the Dataproc Metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Immutable. The consumer-side network configuration for the Dataproc Metastore instance.
    
    pub consumers: Option<Vec<Consumer>>,
    /// Enables custom routes to be imported and exported for the Dataproc Metastore service's peered VPC network.
    #[serde(rename="customRoutesEnabled")]
    
    pub custom_routes_enabled: Option<bool>,
}

impl client::Part for NetworkConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations create projects](ProjectLocationFederationCreateCall) (response)
/// * [locations federations delete projects](ProjectLocationFederationDeleteCall) (response)
/// * [locations federations patch projects](ProjectLocationFederationPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations services backups create projects](ProjectLocationServiceBackupCreateCall) (response)
/// * [locations services backups delete projects](ProjectLocationServiceBackupDeleteCall) (response)
/// * [locations services metadata imports create projects](ProjectLocationServiceMetadataImportCreateCall) (response)
/// * [locations services metadata imports patch projects](ProjectLocationServiceMetadataImportPatchCall) (response)
/// * [locations services migration executions delete projects](ProjectLocationServiceMigrationExecutionDeleteCall) (response)
/// * [locations services alter location projects](ProjectLocationServiceAlterLocationCall) (response)
/// * [locations services alter table properties projects](ProjectLocationServiceAlterTablePropertyCall) (response)
/// * [locations services cancel migration projects](ProjectLocationServiceCancelMigrationCall) (response)
/// * [locations services complete migration projects](ProjectLocationServiceCompleteMigrationCall) (response)
/// * [locations services create projects](ProjectLocationServiceCreateCall) (response)
/// * [locations services delete projects](ProjectLocationServiceDeleteCall) (response)
/// * [locations services export metadata projects](ProjectLocationServiceExportMetadataCall) (response)
/// * [locations services move table to database projects](ProjectLocationServiceMoveTableToDatabaseCall) (response)
/// * [locations services patch projects](ProjectLocationServicePatchCall) (response)
/// * [locations services query metadata projects](ProjectLocationServiceQueryMetadataCall) (response)
/// * [locations services restore projects](ProjectLocationServiceRestoreCall) (response)
/// * [locations services start migration projects](ProjectLocationServiceStartMigrationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}.
    
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources.A Policy is a collection of bindings. A binding binds one or more members, or principals, to a single role. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A role is a named list of permissions; each role can be an IAM predefined role or a user-created custom role.For some types of Google Cloud resources, a binding can also specify a condition, which is a logical expression that allows access to a resource only if the expression evaluates to true. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).JSON example: { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the IAM documentation (https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations get iam policy projects](ProjectLocationFederationGetIamPolicyCall) (response)
/// * [locations federations set iam policy projects](ProjectLocationFederationSetIamPolicyCall) (response)
/// * [locations services backups get iam policy projects](ProjectLocationServiceBackupGetIamPolicyCall) (response)
/// * [locations services backups set iam policy projects](ProjectLocationServiceBackupSetIamPolicyCall) (response)
/// * [locations services databases tables get iam policy projects](ProjectLocationServiceDatabaseTableGetIamPolicyCall) (response)
/// * [locations services databases tables set iam policy projects](ProjectLocationServiceDatabaseTableSetIamPolicyCall) (response)
/// * [locations services databases get iam policy projects](ProjectLocationServiceDatabaseGetIamPolicyCall) (response)
/// * [locations services databases set iam policy projects](ProjectLocationServiceDatabaseSetIamPolicyCall) (response)
/// * [locations services get iam policy projects](ProjectLocationServiceGetIamPolicyCall) (response)
/// * [locations services set iam policy projects](ProjectLocationServiceSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of members, or principals, with a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one principal.The bindings in a Policy can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the bindings grant 50 different roles to user:alice@example.com, and not to any other principal, then you can add another 1,450 principals to the bindings in the Policy.
    
    pub bindings: Option<Vec<Binding>>,
    /// etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Request message for DataprocMetastore.QueryMetadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services query metadata projects](ProjectLocationServiceQueryMetadataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryMetadataRequest {
    /// Required. A read-only SQL query to execute against the metadata database. The query cannot change or mutate the data.
    
    pub query: Option<String>,
}

impl client::RequestValue for QueryMetadataRequest {}


/// Request message for DataprocMetastore.RemoveIamPolicy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services remove iam policy projects](ProjectLocationServiceRemoveIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveIamPolicyRequest {
    /// Optional. Removes IAM policy attached to database or table asynchronously when it is set. The default is false.
    
    pub asynchronous: Option<bool>,
}

impl client::RequestValue for RemoveIamPolicyRequest {}


/// Response message for DataprocMetastore.RemoveIamPolicy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services remove iam policy projects](ProjectLocationServiceRemoveIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveIamPolicyResponse {
    /// True if the policy is successfully removed.
    
    pub success: Option<bool>,
}

impl client::ResponseResult for RemoveIamPolicyResponse {}


/// The details of a metadata restore operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Restore {
    /// Output only. The relative resource name of the metastore service backup to restore from, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}.
    
    pub backup: Option<String>,
    /// Optional. A Cloud Storage URI specifying where the backup artifacts are stored, in the format gs:///.
    #[serde(rename="backupLocation")]
    
    pub backup_location: Option<String>,
    /// Output only. The restore details containing the revision of the service to be restored to, in format of JSON.
    
    pub details: Option<String>,
    /// Output only. The time when the restore ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the restore started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the restore.
    
    pub state: Option<RestoreStateEnum>,
    /// Output only. The type of restore.
    #[serde(rename="type")]
    
    pub type_: Option<RestoreTypeEnum>,
}

impl client::Part for Restore {}


/// Request message for DataprocMetastore.Restore.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services restore projects](ProjectLocationServiceRestoreCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RestoreServiceRequest {
    /// Optional. The relative resource name of the metastore service backup to restore from, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}. Mutually exclusive with backup_location, and exactly one of the two must be set.
    
    pub backup: Option<String>,
    /// Optional. A Cloud Storage URI specifying the location of the backup artifacts, namely - backup avro files under "avro/", backup_metastore.json and service.json, in the following form:gs://. Mutually exclusive with backup, and exactly one of the two must be set.
    #[serde(rename="backupLocation")]
    
    pub backup_location: Option<String>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the request if it has completed. The server will ignore subsequent requests that provide a duplicate request ID for at least 60 minutes after the first request.For example, if an initial request times out, followed by another request with the same request ID, the server ignores the second request to prevent the creation of duplicate commitments.The request ID must be a valid UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier#Format). A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Optional. The type of restore. If unspecified, defaults to METADATA_ONLY.
    #[serde(rename="restoreType")]
    
    pub restore_type: Option<RestoreServiceRequestRestoreTypeEnum>,
}

impl client::RequestValue for RestoreServiceRequest {}


/// Represents the scaling configuration of a metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Optional. The autoscaling configuration.
    #[serde(rename="autoscalingConfig")]
    
    pub autoscaling_config: Option<AutoscalingConfig>,
    /// An enum of readable instance sizes, with each instance size mapping to a float value (e.g. InstanceSize.EXTRA_SMALL = scaling_factor(0.1))
    #[serde(rename="instanceSize")]
    
    pub instance_size: Option<ScalingConfigInstanceSizeEnum>,
    /// Scaling factor, increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0.
    #[serde(rename="scalingFactor")]
    
    pub scaling_factor: Option<f32>,
}

impl client::Part for ScalingConfig {}


/// This specifies the configuration of scheduled backup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScheduledBackup {
    /// Optional. A Cloud Storage URI of a folder, in the format gs:///. A sub-folder containing backup files will be stored below it.
    #[serde(rename="backupLocation")]
    
    pub backup_location: Option<String>,
    /// Optional. The scheduled interval in Cron format, see https://en.wikipedia.org/wiki/Cron The default is empty: scheduled backup is not enabled. Must be specified to enable scheduled backups.
    #[serde(rename="cronSchedule")]
    
    pub cron_schedule: Option<String>,
    /// Optional. Defines whether the scheduled backup is enabled. The default value is false.
    
    pub enabled: Option<bool>,
    /// Output only. The details of the latest scheduled backup.
    #[serde(rename="latestBackup")]
    
    pub latest_backup: Option<LatestBackup>,
    /// Output only. The time when the next backups execution is scheduled to start.
    #[serde(rename="nextScheduledTime")]
    
    pub next_scheduled_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Specifies the time zone to be used when interpreting cron_schedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones), e.g. America/Los_Angeles or Africa/Abidjan. If left unspecified, the default is UTC.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::Part for ScheduledBackup {}


/// A securely stored value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    /// The relative resource name of a Secret Manager secret version, in the following form:projects/{project_number}/secrets/{secret_id}/versions/{version_id}.
    #[serde(rename="cloudSecret")]
    
    pub cloud_secret: Option<String>,
}

impl client::Part for Secret {}


/// A managed metastore service that serves metadata queries.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services create projects](ProjectLocationServiceCreateCall) (request)
/// * [locations services get projects](ProjectLocationServiceGetCall) (response)
/// * [locations services patch projects](ProjectLocationServicePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored.
    #[serde(rename="artifactGcsUri")]
    
    pub artifact_gcs_uri: Option<String>,
    /// Output only. The time when the metastore service was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The database type that the Metastore service stores its data.
    #[serde(rename="databaseType")]
    
    pub database_type: Option<ServiceDatabaseTypeEnum>,
    /// Immutable. Information used to configure the Dataproc Metastore service to encrypt customer data at rest. Cannot be updated.
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<EncryptionConfig>,
    /// Output only. The URI of the endpoint used to access the metastore service.
    #[serde(rename="endpointUri")]
    
    pub endpoint_uri: Option<String>,
    /// Configuration information specific to running Hive metastore software as the metastore service.
    #[serde(rename="hiveMetastoreConfig")]
    
    pub hive_metastore_config: Option<HiveMetastoreConfig>,
    /// User-defined labels for the metastore service.
    
    pub labels: Option<HashMap<String, String>>,
    /// The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time. Maintenance window is not needed for services with the SPANNER database type.
    #[serde(rename="maintenanceWindow")]
    
    pub maintenance_window: Option<MaintenanceWindow>,
    /// Optional. The setting that defines how metastore metadata should be integrated with external services and systems.
    #[serde(rename="metadataIntegration")]
    
    pub metadata_integration: Option<MetadataIntegration>,
    /// Output only. The metadata management activities of the metastore service.
    #[serde(rename="metadataManagementActivity")]
    
    pub metadata_management_activity: Option<MetadataManagementActivity>,
    /// Immutable. The relative resource name of the metastore service, in the following format:projects/{project_number}/locations/{location_id}/services/{service_id}.
    
    pub name: Option<String>,
    /// Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:projects/{project_number}/global/networks/{network_id}.
    
    pub network: Option<String>,
    /// The configuration specifying the network settings for the Dataproc Metastore service.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NetworkConfig>,
    /// The TCP port at which the metastore service is reached. Default: 9083.
    
    pub port: Option<i32>,
    /// Immutable. The release channel of the service. If unspecified, defaults to STABLE.
    #[serde(rename="releaseChannel")]
    
    pub release_channel: Option<ServiceReleaseChannelEnum>,
    /// Scaling configuration of the metastore service.
    #[serde(rename="scalingConfig")]
    
    pub scaling_config: Option<ScalingConfig>,
    /// Optional. The configuration of scheduled backup for the metastore service.
    #[serde(rename="scheduledBackup")]
    
    pub scheduled_backup: Option<ScheduledBackup>,
    /// Output only. The current state of the metastore service.
    
    pub state: Option<ServiceStateEnum>,
    /// Output only. Additional information about the current state of the metastore service, if available.
    #[serde(rename="stateMessage")]
    
    pub state_message: Option<String>,
    /// The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON.
    #[serde(rename="telemetryConfig")]
    
    pub telemetry_config: Option<TelemetryConfig>,
    /// The tier of the service.
    
    pub tier: Option<ServiceTierEnum>,
    /// Output only. The globally unique resource identifier of the metastore service.
    
    pub uid: Option<String>,
    /// Output only. The time when the metastore service was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Service {}
impl client::ResponseResult for Service {}


/// Request message for SetIamPolicy method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations set iam policy projects](ProjectLocationFederationSetIamPolicyCall) (request)
/// * [locations services backups set iam policy projects](ProjectLocationServiceBackupSetIamPolicyCall) (request)
/// * [locations services databases tables set iam policy projects](ProjectLocationServiceDatabaseTableSetIamPolicyCall) (request)
/// * [locations services databases set iam policy projects](ProjectLocationServiceDatabaseSetIamPolicyCall) (request)
/// * [locations services set iam policy projects](ProjectLocationServiceSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: "bindings, etag"
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Request message for DataprocMetastore.StartMigration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services start migration projects](ProjectLocationServiceStartMigrationCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StartMigrationRequest {
    /// Required. The configuration details for the migration.
    #[serde(rename="migrationExecution")]
    
    pub migration_execution: Option<MigrationExecution>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the request if it has completed. The server will ignore subsequent requests that provide a duplicate request ID for at least 60 minutes after the first request.For example, if an initial request times out, followed by another request with the same request ID, the server ignores the second request to prevent the creation of duplicate commitments.The request ID must be a valid UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier#Format) A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for StartMigrationRequest {}


/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors).
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


/// Telemetry Configuration for the Dataproc Metastore service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// The output format of the Dataproc Metastore service's logs.
    #[serde(rename="logFormat")]
    
    pub log_format: Option<TelemetryConfigLogFormatEnum>,
}

impl client::Part for TelemetryConfig {}


/// Request message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations test iam permissions projects](ProjectLocationFederationTestIamPermissionCall) (request)
/// * [locations services backups test iam permissions projects](ProjectLocationServiceBackupTestIamPermissionCall) (request)
/// * [locations services databases tables test iam permissions projects](ProjectLocationServiceDatabaseTableTestIamPermissionCall) (request)
/// * [locations services databases test iam permissions projects](ProjectLocationServiceDatabaseTestIamPermissionCall) (request)
/// * [locations services test iam permissions projects](ProjectLocationServiceTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the resource. Permissions with wildcards (such as * or storage.*) are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for TestIamPermissions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations federations test iam permissions projects](ProjectLocationFederationTestIamPermissionCall) (response)
/// * [locations services backups test iam permissions projects](ProjectLocationServiceBackupTestIamPermissionCall) (response)
/// * [locations services databases tables test iam permissions projects](ProjectLocationServiceDatabaseTableTestIamPermissionCall) (response)
/// * [locations services databases test iam permissions projects](ProjectLocationServiceDatabaseTestIamPermissionCall) (response)
/// * [locations services test iam permissions projects](ProjectLocationServiceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of TestPermissionsRequest.permissions that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


