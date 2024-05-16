use super::*;
/// The request for HubService.AcceptHubSpoke.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs accept spoke projects](ProjectLocationGlobalHubAcceptSpokeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceptHubSpokeRequest {
    /// Optional. A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server knows to ignore the request if it has already been completed. The server guarantees that a request doesn't result in creation of duplicate commitments for at least 60 minutes. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check to see whether the original operation was received. If it was, the server ignores the second request. This behavior prevents clients from mistakenly creating duplicate commitments. The request ID must be a valid UUID, with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Required. The URI of the spoke to accept into the hub.
    #[serde(rename="spokeUri")]
    
    pub spoke_uri: Option<String>,
}

impl client::RequestValue for AcceptHubSpokeRequest {}


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


/// Allow the producer to specify which consumers can connect to it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsumerPscConfig {
    /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
    #[serde(rename="disableGlobalAccess")]
    
    pub disable_global_access: Option<bool>,
    /// The resource path of the consumer network where PSC connections are allowed to be created in. Note, this network does not need be in the ConsumerPscConfig.project in the case of SharedVPC. Example: projects/{projectNumOrId}/global/networks/{networkId}.
    
    pub network: Option<String>,
    /// The consumer project where PSC connections are allowed to be created in.
    
    pub project: Option<String>,
    /// Output only. Overall state of PSC Connections management for this consumer psc config.
    
    pub state: Option<ConsumerPscConfigStateEnum>,
}

impl client::Part for ConsumerPscConfig {}


/// PSC connection details on consumer side.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsumerPscConnection {
    /// The most recent error during operating this connection.
    
    pub error: Option<GoogleRpcStatus>,
    /// Output only. The error info for the latest error during operating this connection.
    #[serde(rename="errorInfo")]
    
    pub error_info: Option<GoogleRpcErrorInfo>,
    /// The error type indicates whether the error is consumer facing, producer facing or system internal.
    #[serde(rename="errorType")]
    
    pub error_type: Option<ConsumerPscConnectionErrorTypeEnum>,
    /// The URI of the consumer forwarding rule created. Example: projects/{projectNumOrId}/regions/us-east1/networks/{resourceId}.
    #[serde(rename="forwardingRule")]
    
    pub forwarding_rule: Option<String>,
    /// The last Compute Engine operation to setup PSC connection.
    #[serde(rename="gceOperation")]
    
    pub gce_operation: Option<String>,
    /// The IP literal allocated on the consumer network for the PSC forwarding rule that is created to connect to the producer service attachment in this service connection map.
    
    pub ip: Option<String>,
    /// The consumer network whose PSC forwarding rule is connected to the service attachments in this service connection map. Note that the network could be on a different project (shared VPC).
    
    pub network: Option<String>,
    /// The consumer project whose PSC forwarding rule is connected to the service attachments in this service connection map.
    
    pub project: Option<String>,
    /// The PSC connection id of the PSC forwarding rule connected to the service attachments in this service connection map.
    #[serde(rename="pscConnectionId")]
    
    pub psc_connection_id: Option<String>,
    /// Output only. The URI of the selected subnetwork selected to allocate IP address for this connection.
    #[serde(rename="selectedSubnetwork")]
    
    pub selected_subnetwork: Option<String>,
    /// The URI of a service attachment which is the target of the PSC connection.
    #[serde(rename="serviceAttachmentUri")]
    
    pub service_attachment_uri: Option<String>,
    /// The state of the PSC connection.
    
    pub state: Option<ConsumerPscConnectionStateEnum>,
}

impl client::Part for ConsumerPscConnection {}


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


/// Filter matches L4 traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// Optional. The destination IP range of outgoing packets that this policy-based route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
    #[serde(rename="destRange")]
    
    pub dest_range: Option<String>,
    /// Optional. The IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'.
    #[serde(rename="ipProtocol")]
    
    pub ip_protocol: Option<String>,
    /// Required. Internet protocol versions this policy-based route applies to. For this version, only IPV4 is supported.
    #[serde(rename="protocolVersion")]
    
    pub protocol_version: Option<FilterProtocolVersionEnum>,
    /// Optional. The source IP range of outgoing packets that this policy-based route applies to. Default is "0.0.0.0/0" if protocol version is IPv4.
    #[serde(rename="srcRange")]
    
    pub src_range: Option<String>,
}

impl client::Part for Filter {}


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
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunningCancelOperationRequest {}


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
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs accept spoke projects](ProjectLocationGlobalHubAcceptSpokeCall) (response)
/// * [locations global hubs create projects](ProjectLocationGlobalHubCreateCall) (response)
/// * [locations global hubs delete projects](ProjectLocationGlobalHubDeleteCall) (response)
/// * [locations global hubs patch projects](ProjectLocationGlobalHubPatchCall) (response)
/// * [locations global hubs reject spoke projects](ProjectLocationGlobalHubRejectSpokeCall) (response)
/// * [locations global policy based routes create projects](ProjectLocationGlobalPolicyBasedRouteCreateCall) (response)
/// * [locations global policy based routes delete projects](ProjectLocationGlobalPolicyBasedRouteDeleteCall) (response)
/// * [locations internal ranges create projects](ProjectLocationInternalRangeCreateCall) (response)
/// * [locations internal ranges delete projects](ProjectLocationInternalRangeDeleteCall) (response)
/// * [locations internal ranges patch projects](ProjectLocationInternalRangePatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations service classes delete projects](ProjectLocationServiceClassDeleteCall) (response)
/// * [locations service classes patch projects](ProjectLocationServiceClassPatchCall) (response)
/// * [locations service connection maps create projects](ProjectLocationServiceConnectionMapCreateCall) (response)
/// * [locations service connection maps delete projects](ProjectLocationServiceConnectionMapDeleteCall) (response)
/// * [locations service connection maps patch projects](ProjectLocationServiceConnectionMapPatchCall) (response)
/// * [locations service connection policies create projects](ProjectLocationServiceConnectionPolicyCreateCall) (response)
/// * [locations service connection policies delete projects](ProjectLocationServiceConnectionPolicyDeleteCall) (response)
/// * [locations service connection policies patch projects](ProjectLocationServiceConnectionPolicyPatchCall) (response)
/// * [locations service connection tokens create projects](ProjectLocationServiceConnectionTokenCreateCall) (response)
/// * [locations service connection tokens delete projects](ProjectLocationServiceConnectionTokenDeleteCall) (response)
/// * [locations spokes create projects](ProjectLocationSpokeCreateCall) (response)
/// * [locations spokes delete projects](ProjectLocationSpokeDeleteCall) (response)
/// * [locations spokes patch projects](ProjectLocationSpokePatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// Describes the cause of the error with structured details. Example of an error when contacting the "pubsub.googleapis.com" API when it is not enabled: { "reason": "API_DISABLED" "domain": "googleapis.com" "metadata": { "resource": "projects/123", "service": "pubsub.googleapis.com" } } This response indicates that the pubsub.googleapis.com API is not enabled. Example of an error that is returned when attempting to create a Spanner instance in a region that is out of stock: { "reason": "STOCKOUT" "domain": "spanner.googleapis.com", "metadata": { "availableRegions": "us-central1,us-east2" } }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcErrorInfo {
    /// The logical grouping to which the "reason" belongs. The error domain is typically the registered service name of the tool or product that generates the error. Example: "pubsub.googleapis.com". If the error is generated by some common infrastructure, the error domain must be a globally unique value that identifies the infrastructure. For Google API infrastructure, the error domain is "googleapis.com".
    
    pub domain: Option<String>,
    /// Additional structured details about this error. Keys should match /[a-zA-Z0-9-_]/ and be limited to 64 characters in length. When identifying the current value of an exceeded limit, the units should be contained in the key, not the value. For example, rather than {"instanceLimit": "100/request"}, should be returned as, {"instanceLimitPerRequest": "100"}, if the client exceeds the number of instances that can be created in a single (batch) request.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The reason of the error. This is a constant value that identifies the proximate cause of the error. Error reasons are unique within a particular domain of errors. This should be at most 63 characters and match a regular expression of `A-Z+[A-Z0-9]`, which represents UPPER_SNAKE_CASE.
    
    pub reason: Option<String>,
}

impl client::Part for GoogleRpcErrorInfo {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// A group represents a subset of spokes attached to a hub.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs groups get projects](ProjectLocationGlobalHubGroupGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    /// Output only. The time the group was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The description of the group.
    
    pub description: Option<String>,
    /// Optional. Labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of the group. Group names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub}/groups/{group_id}`
    
    pub name: Option<String>,
    /// Output only. The current lifecycle state of this group.
    
    pub state: Option<GroupStateEnum>,
    /// Output only. The Google-generated UUID for the group. This value is unique across all group resources. If a group is deleted and another with the same name is created, the new route table is assigned a different unique_id.
    
    pub uid: Option<String>,
    /// Output only. The time the group was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Group {}


/// A Network Connectivity Center hub is a global management resource to which you attach spokes. A single hub can contain spokes from multiple regions. However, if any of a hub’s spokes use the site-to-site data transfer feature, the resources associated with those spokes must all be in the same VPC network. Spokes that do not use site-to-site data transfer can be associated with any VPC network in your project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs create projects](ProjectLocationGlobalHubCreateCall) (request)
/// * [locations global hubs get projects](ProjectLocationGlobalHubGetCall) (response)
/// * [locations global hubs patch projects](ProjectLocationGlobalHubPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hub {
    /// Output only. The time the hub was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An optional description of the hub.
    
    pub description: Option<String>,
    /// Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of the hub. Hub names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}`
    
    pub name: Option<String>,
    /// Output only. The route tables that belong to this hub. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}/routeTables/{route_table_id}` This field is read-only. Network Connectivity Center automatically populates it based on the route tables nested under the hub.
    #[serde(rename="routeTables")]
    
    pub route_tables: Option<Vec<String>>,
    /// The VPC networks associated with this hub's spokes. This field is read-only. Network Connectivity Center automatically populates it based on the set of spokes attached to the hub.
    #[serde(rename="routingVpcs")]
    
    pub routing_vpcs: Option<Vec<RoutingVPC>>,
    /// Output only. A summary of the spokes associated with a hub. The summary includes a count of spokes according to type and according to state. If any spokes are inactive, the summary also lists the reasons they are inactive, including a count for each reason.
    #[serde(rename="spokeSummary")]
    
    pub spoke_summary: Option<SpokeSummary>,
    /// Output only. The current lifecycle state of this hub.
    
    pub state: Option<HubStateEnum>,
    /// Output only. The Google-generated UUID for the hub. This value is unique across all hub resources. If a hub is deleted and another with the same name is created, the new hub is assigned a different unique_id.
    #[serde(rename="uniqueId")]
    
    pub unique_id: Option<String>,
    /// Output only. The time the hub was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Hub {}
impl client::ResponseResult for Hub {}


/// InterconnectAttachment that this route applies to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InterconnectAttachment {
    /// Optional. Cloud region to install this policy-based route on interconnect attachment. Use `all` to install it on all interconnect attachments.
    
    pub region: Option<String>,
}

impl client::Part for InterconnectAttachment {}


/// The internal range resource for IPAM operations within a VPC network. Used to represent a private address range along with behavioral characterstics of that range (its usage and peering behavior). Networking resources can link to this range if they are created as belonging to it.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations internal ranges create projects](ProjectLocationInternalRangeCreateCall) (request)
/// * [locations internal ranges get projects](ProjectLocationInternalRangeGetCall) (response)
/// * [locations internal ranges patch projects](ProjectLocationInternalRangePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InternalRange {
    /// Time when the internal range was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of this resource.
    
    pub description: Option<String>,
    /// The IP range that this internal range defines.
    #[serde(rename="ipCidrRange")]
    
    pub ip_cidr_range: Option<String>,
    /// User-defined labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of an internal range. Format: projects/{project}/locations/{location}/internalRanges/{internal_range} See: https://google.aip.dev/122#fields-representing-resource-names
    
    pub name: Option<String>,
    /// The URL or resource ID of the network in which to reserve the internal range. The network cannot be deleted if there are any reserved internal ranges referring to it. Legacy networks are not supported. This can only be specified for a global internal address. Example: - URL: /compute/v1/projects/{project}/global/networks/{resourceId} - ID: network123
    
    pub network: Option<String>,
    /// Optional. Types of resources that are allowed to overlap with the current internal range.
    
    pub overlaps: Option<Vec<InternalRangeOverlapsEnum>>,
    /// The type of peering set for this internal range.
    
    pub peering: Option<InternalRangePeeringEnum>,
    /// An alternate to ip_cidr_range. Can be set when trying to create a reservation that automatically finds a free range of the given size. If both ip_cidr_range and prefix_length are set, there is an error if the range sizes do not match. Can also be used during updates to change the range size.
    #[serde(rename="prefixLength")]
    
    pub prefix_length: Option<i32>,
    /// Optional. Can be set to narrow down or pick a different address space while searching for a free range. If not set, defaults to the "10.0.0.0/8" address space. This can be used to search in other rfc-1918 address spaces like "172.16.0.0/12" and "192.168.0.0/16" or non-rfc-1918 address spaces used in the VPC.
    #[serde(rename="targetCidrRange")]
    
    pub target_cidr_range: Option<Vec<String>>,
    /// Time when the internal range was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The type of usage set for this InternalRange.
    
    pub usage: Option<InternalRangeUsageEnum>,
    /// Output only. The list of resources that refer to this internal range. Resources that use the internal range for their range allocation are referred to as users of the range. Other resources mark themselves as users while doing so by creating a reference to this internal range. Having a user, based on this reference, prevents deletion of the internal range referred to. Can be empty.
    
    pub users: Option<Vec<String>>,
}

impl client::RequestValue for InternalRange {}
impl client::ResponseResult for InternalRange {}


/// A collection of VLAN attachment resources. These resources should be redundant attachments that all advertise the same prefixes to Google Cloud. Alternatively, in active/passive configurations, all attachments should be capable of advertising the same prefixes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedInterconnectAttachments {
    /// A value that controls whether site-to-site data transfer is enabled for these resources. Data transfer is available only in [supported locations](https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations).
    #[serde(rename="siteToSiteDataTransfer")]
    
    pub site_to_site_data_transfer: Option<bool>,
    /// The URIs of linked interconnect attachment resources
    
    pub uris: Option<Vec<String>>,
    /// Output only. The VPC network where these VLAN attachments are located.
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<String>,
}

impl client::Part for LinkedInterconnectAttachments {}


/// A collection of router appliance instances. If you configure multiple router appliance instances to receive data from the same set of sites outside of Google Cloud, we recommend that you associate those instances with the same spoke.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedRouterApplianceInstances {
    /// The list of router appliance instances.
    
    pub instances: Option<Vec<RouterApplianceInstance>>,
    /// A value that controls whether site-to-site data transfer is enabled for these resources. Data transfer is available only in [supported locations](https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations).
    #[serde(rename="siteToSiteDataTransfer")]
    
    pub site_to_site_data_transfer: Option<bool>,
    /// Output only. The VPC network where these router appliance instances are located.
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<String>,
}

impl client::Part for LinkedRouterApplianceInstances {}


/// An existing VPC network.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedVpcNetwork {
    /// Optional. IP ranges encompassing the subnets to be excluded from peering.
    #[serde(rename="excludeExportRanges")]
    
    pub exclude_export_ranges: Option<Vec<String>>,
    /// Required. The URI of the VPC network resource.
    
    pub uri: Option<String>,
}

impl client::Part for LinkedVpcNetwork {}


/// A collection of Cloud VPN tunnel resources. These resources should be redundant HA VPN tunnels that all advertise the same prefixes to Google Cloud. Alternatively, in a passive/active configuration, all tunnels should be capable of advertising the same prefixes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedVpnTunnels {
    /// A value that controls whether site-to-site data transfer is enabled for these resources. Data transfer is available only in [supported locations](https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/locations).
    #[serde(rename="siteToSiteDataTransfer")]
    
    pub site_to_site_data_transfer: Option<bool>,
    /// The URIs of linked VPN tunnel resources.
    
    pub uris: Option<Vec<String>>,
    /// Output only. The VPC network where these VPN tunnels are located.
    #[serde(rename="vpcNetwork")]
    
    pub vpc_network: Option<String>,
}

impl client::Part for LinkedVpnTunnels {}


/// Response for HubService.ListGroups method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs groups list projects](ProjectLocationGlobalHubGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    /// The requested groups.
    
    pub groups: Option<Vec<Group>>,
    /// The token for the next page of the response. To see more results, use this value as the page_token for your next request. If this value is empty, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Hubs that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListGroupsResponse {}


/// The response for HubService.ListHubSpokes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs list spokes projects](ProjectLocationGlobalHubListSpokeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHubSpokesResponse {
    /// The token for the next page of the response. To see more results, use this value as the page_token for your next request. If this value is empty, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The requested spokes. The spoke fields can be partially populated based on the `view` field in the request message.
    
    pub spokes: Option<Vec<Spoke>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListHubSpokesResponse {}


/// Response for HubService.ListHubs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs list projects](ProjectLocationGlobalHubListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHubsResponse {
    /// The requested hubs.
    
    pub hubs: Option<Vec<Hub>>,
    /// The token for the next page of the response. To see more results, use this value as the page_token for your next request. If this value is empty, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListHubsResponse {}


/// Response for InternalRange.ListInternalRanges
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations internal ranges list projects](ProjectLocationInternalRangeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInternalRangesResponse {
    /// Internal ranges to be returned.
    #[serde(rename="internalRanges")]
    
    pub internal_ranges: Option<Vec<InternalRange>>,
    /// The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListInternalRangesResponse {}


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


/// Response for PolicyBasedRouting.ListPolicyBasedRoutes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global policy based routes list projects](ProjectLocationGlobalPolicyBasedRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPolicyBasedRoutesResponse {
    /// The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Policy-based routes to be returned.
    #[serde(rename="policyBasedRoutes")]
    
    pub policy_based_routes: Option<Vec<PolicyBasedRoute>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListPolicyBasedRoutesResponse {}


/// Response for HubService.ListRouteTables method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs route tables list projects](ProjectLocationGlobalHubRouteTableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRouteTablesResponse {
    /// The token for the next page of the response. To see more results, use this value as the page_token for your next request. If this value is empty, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The requested route tables.
    #[serde(rename="routeTables")]
    
    pub route_tables: Option<Vec<RouteTable>>,
    /// Hubs that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRouteTablesResponse {}


/// Response for HubService.ListRoutes method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs route tables routes list projects](ProjectLocationGlobalHubRouteTableRouteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRoutesResponse {
    /// The token for the next page of the response. To see more results, use this value as the page_token for your next request. If this value is empty, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The requested routes.
    
    pub routes: Option<Vec<Route>>,
    /// RouteTables that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRoutesResponse {}


/// Response for ListServiceClasses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service classes list projects](ProjectLocationServiceClassListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceClassesResponse {
    /// The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// ServiceClasses to be returned.
    #[serde(rename="serviceClasses")]
    
    pub service_classes: Option<Vec<ServiceClass>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServiceClassesResponse {}


/// Response for ListServiceConnectionMaps.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service connection maps list projects](ProjectLocationServiceConnectionMapListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceConnectionMapsResponse {
    /// The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// ServiceConnectionMaps to be returned.
    #[serde(rename="serviceConnectionMaps")]
    
    pub service_connection_maps: Option<Vec<ServiceConnectionMap>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServiceConnectionMapsResponse {}


/// Response for ListServiceConnectionPolicies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service connection policies list projects](ProjectLocationServiceConnectionPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceConnectionPoliciesResponse {
    /// The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// ServiceConnectionPolicies to be returned.
    #[serde(rename="serviceConnectionPolicies")]
    
    pub service_connection_policies: Option<Vec<ServiceConnectionPolicy>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServiceConnectionPoliciesResponse {}


/// Response for ListServiceConnectionTokens.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service connection tokens list projects](ProjectLocationServiceConnectionTokenListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceConnectionTokensResponse {
    /// The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// ServiceConnectionTokens to be returned.
    #[serde(rename="serviceConnectionTokens")]
    
    pub service_connection_tokens: Option<Vec<ServiceConnectionToken>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListServiceConnectionTokensResponse {}


/// The response for HubService.ListSpokes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations spokes list projects](ProjectLocationSpokeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSpokesResponse {
    /// The token for the next page of the response. To see more results, use this value as the page_token for your next request. If this value is empty, there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The requested spokes.
    
    pub spokes: Option<Vec<Spoke>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListSpokesResponse {}


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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NextHopVpcNetwork {
    /// The URI of the VPC network resource
    
    pub uri: Option<String>,
}

impl client::Part for NextHopVpcNetwork {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs groups get iam policy projects](ProjectLocationGlobalHubGroupGetIamPolicyCall) (response)
/// * [locations global hubs groups set iam policy projects](ProjectLocationGlobalHubGroupSetIamPolicyCall) (response)
/// * [locations global hubs get iam policy projects](ProjectLocationGlobalHubGetIamPolicyCall) (response)
/// * [locations global hubs set iam policy projects](ProjectLocationGlobalHubSetIamPolicyCall) (response)
/// * [locations global policy based routes get iam policy projects](ProjectLocationGlobalPolicyBasedRouteGetIamPolicyCall) (response)
/// * [locations global policy based routes set iam policy projects](ProjectLocationGlobalPolicyBasedRouteSetIamPolicyCall) (response)
/// * [locations service classes get iam policy projects](ProjectLocationServiceClassGetIamPolicyCall) (response)
/// * [locations service classes set iam policy projects](ProjectLocationServiceClassSetIamPolicyCall) (response)
/// * [locations service connection maps get iam policy projects](ProjectLocationServiceConnectionMapGetIamPolicyCall) (response)
/// * [locations service connection maps set iam policy projects](ProjectLocationServiceConnectionMapSetIamPolicyCall) (response)
/// * [locations service connection policies get iam policy projects](ProjectLocationServiceConnectionPolicyGetIamPolicyCall) (response)
/// * [locations service connection policies set iam policy projects](ProjectLocationServiceConnectionPolicySetIamPolicyCall) (response)
/// * [locations spokes get iam policy projects](ProjectLocationSpokeGetIamPolicyCall) (response)
/// * [locations spokes set iam policy projects](ProjectLocationSpokeSetIamPolicyCall) (response)
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


/// Policy-based routes route L4 network traffic based on not just destination IP address, but also source IP address, protocol, and more. If a policy-based route conflicts with other types of routes, the policy-based route always take precedence.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global policy based routes create projects](ProjectLocationGlobalPolicyBasedRouteCreateCall) (request)
/// * [locations global policy based routes get projects](ProjectLocationGlobalPolicyBasedRouteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyBasedRoute {
    /// Output only. Time when the policy-based route was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. An optional description of this resource. Provide this field when you create the resource.
    
    pub description: Option<String>,
    /// Required. The filter to match L4 traffic.
    
    pub filter: Option<Filter>,
    /// Optional. The interconnect attachments that this policy-based route applies to.
    #[serde(rename="interconnectAttachment")]
    
    pub interconnect_attachment: Option<InterconnectAttachment>,
    /// Output only. Type of this resource. Always networkconnectivity#policyBasedRoute for policy-based Route resources.
    
    pub kind: Option<String>,
    /// User-defined labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. A unique name of the resource in the form of `projects/{project_number}/locations/global/PolicyBasedRoutes/{policy_based_route_id}`
    
    pub name: Option<String>,
    /// Required. Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network.
    
    pub network: Option<String>,
    /// Optional. The IP address of a global-access-enabled L4 ILB that is the next hop for matching packets. For this version, only nextHopIlbIp is supported.
    #[serde(rename="nextHopIlbIp")]
    
    pub next_hop_ilb_ip: Option<String>,
    /// Optional. Other routes that will be referenced to determine the next hop of the packet.
    #[serde(rename="nextHopOtherRoutes")]
    
    pub next_hop_other_routes: Option<PolicyBasedRouteNextHopOtherRoutesEnum>,
    /// Optional. The priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive.
    
    pub priority: Option<i32>,
    /// Output only. Server-defined fully-qualified URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. Time when the policy-based route was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. VM instances to which this policy-based route applies to.
    #[serde(rename="virtualMachine")]
    
    pub virtual_machine: Option<VirtualMachine>,
    /// Output only. If potential misconfigurations are detected for this route, this field will be populated with warning messages.
    
    pub warnings: Option<Vec<Warnings>>,
}

impl client::RequestValue for PolicyBasedRoute {}
impl client::ResponseResult for PolicyBasedRoute {}


/// The PSC configurations on producer side.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProducerPscConfig {
    /// The resource path of a service attachment. Example: projects/{projectNumOrId}/regions/{region}/serviceAttachments/{resourceId}.
    #[serde(rename="serviceAttachmentUri")]
    
    pub service_attachment_uri: Option<String>,
}

impl client::Part for ProducerPscConfig {}


/// Configuration used for Private Service Connect connections. Used when Infrastructure is PSC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PscConfig {
    /// Optional. Max number of PSC connections for this policy.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub limit: Option<i64>,
    /// The resource paths of subnetworks to use for IP address management. Example: projects/{projectNumOrId}/regions/{region}/subnetworks/{resourceId}.
    
    pub subnetworks: Option<Vec<String>>,
}

impl client::Part for PscConfig {}


/// Information about a specific Private Service Connect connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PscConnection {
    /// The resource reference of the consumer address.
    #[serde(rename="consumerAddress")]
    
    pub consumer_address: Option<String>,
    /// The resource reference of the PSC Forwarding Rule within the consumer VPC.
    #[serde(rename="consumerForwardingRule")]
    
    pub consumer_forwarding_rule: Option<String>,
    /// The project where the PSC connection is created.
    #[serde(rename="consumerTargetProject")]
    
    pub consumer_target_project: Option<String>,
    /// The most recent error during operating this connection.
    
    pub error: Option<GoogleRpcStatus>,
    /// Output only. The error info for the latest error during operating this connection.
    #[serde(rename="errorInfo")]
    
    pub error_info: Option<GoogleRpcErrorInfo>,
    /// The error type indicates whether the error is consumer facing, producer facing or system internal.
    #[serde(rename="errorType")]
    
    pub error_type: Option<PscConnectionErrorTypeEnum>,
    /// The last Compute Engine operation to setup PSC connection.
    #[serde(rename="gceOperation")]
    
    pub gce_operation: Option<String>,
    /// The PSC connection id of the PSC forwarding rule.
    #[serde(rename="pscConnectionId")]
    
    pub psc_connection_id: Option<String>,
    /// Output only. The URI of the subnetwork selected to allocate IP address for this connection.
    #[serde(rename="selectedSubnetwork")]
    
    pub selected_subnetwork: Option<String>,
    /// State of the PSC Connection
    
    pub state: Option<PscConnectionStateEnum>,
}

impl client::Part for PscConnection {}


/// The request for HubService.RejectHubSpoke.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs reject spoke projects](ProjectLocationGlobalHubRejectSpokeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RejectHubSpokeRequest {
    /// Optional. Additional information provided by the hub administrator.
    
    pub details: Option<String>,
    /// Optional. A request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server knows to ignore the request if it has already been completed. The server guarantees that a request doesn't result in creation of duplicate commitments for at least 60 minutes. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check to see whether the original operation was received. If it was, the server ignores the second request. This behavior prevents clients from mistakenly creating duplicate commitments. The request ID must be a valid UUID, with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Required. The URI of the spoke to reject from the hub.
    #[serde(rename="spokeUri")]
    
    pub spoke_uri: Option<String>,
}

impl client::RequestValue for RejectHubSpokeRequest {}


/// A route defines a path from VM instances within a spoke to a specific destination resource. Only VPC spokes have routes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs route tables routes get projects](ProjectLocationGlobalHubRouteTableRouteGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    /// Output only. The time the route was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An optional description of the route.
    
    pub description: Option<String>,
    /// The destination IP address range.
    #[serde(rename="ipCidrRange")]
    
    pub ip_cidr_range: Option<String>,
    /// Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The origin location of the route. Uses the following form: "projects/{project}/locations/{location}" Example: projects/1234/locations/us-central1
    
    pub location: Option<String>,
    /// Immutable. The name of the route. Route names must be unique. Route names use the following form: `projects/{project_number}/locations/global/hubs/{hub}/routeTables/{route_table_id}/routes/{route_id}`
    
    pub name: Option<String>,
    /// Immutable. The destination VPC network for packets on this route.
    #[serde(rename="nextHopVpcNetwork")]
    
    pub next_hop_vpc_network: Option<NextHopVpcNetwork>,
    /// Immutable. The spoke that this route leads to. Example: projects/12345/locations/global/spokes/SPOKE
    
    pub spoke: Option<String>,
    /// Output only. The current lifecycle state of the route.
    
    pub state: Option<RouteStateEnum>,
    /// Output only. The route's type. Its type is determined by the properties of its IP address range.
    #[serde(rename="type")]
    
    pub type_: Option<RouteTypeEnum>,
    /// Output only. The Google-generated UUID for the route. This value is unique across all Network Connectivity Center route resources. If a route is deleted and another with the same name is created, the new route is assigned a different `uid`.
    
    pub uid: Option<String>,
    /// Output only. The time the route was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Route {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs route tables get projects](ProjectLocationGlobalHubRouteTableGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RouteTable {
    /// Output only. The time the route table was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An optional description of the route table.
    
    pub description: Option<String>,
    /// Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of the route table. Route table names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub}/routeTables/{route_table_id}`
    
    pub name: Option<String>,
    /// Output only. The current lifecycle state of this route table.
    
    pub state: Option<RouteTableStateEnum>,
    /// Output only. The Google-generated UUID for the route table. This value is unique across all route table resources. If a route table is deleted and another with the same name is created, the new route table is assigned a different `uid`.
    
    pub uid: Option<String>,
    /// Output only. The time the route table was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for RouteTable {}


/// A router appliance instance is a Compute Engine virtual machine (VM) instance that acts as a BGP speaker. A router appliance instance is specified by the URI of the VM and the internal IP address of one of the VM's network interfaces.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RouterApplianceInstance {
    /// The IP address on the VM to use for peering.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// The URI of the VM.
    #[serde(rename="virtualMachine")]
    
    pub virtual_machine: Option<String>,
}

impl client::Part for RouterApplianceInstance {}


/// RoutingVPC contains information about the VPC networks associated with the spokes of a Network Connectivity Center hub.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoutingVPC {
    /// Output only. If true, indicates that this VPC network is currently associated with spokes that use the data transfer feature (spokes where the site_to_site_data_transfer field is set to true). If you create new spokes that use data transfer, they must be associated with this VPC network. At most, one VPC network will have this field set to true.
    #[serde(rename="requiredForNewSiteToSiteDataTransferSpokes")]
    
    pub required_for_new_site_to_site_data_transfer_spokes: Option<bool>,
    /// The URI of the VPC network.
    
    pub uri: Option<String>,
}

impl client::Part for RoutingVPC {}


/// The ServiceClass resource. Next id: 9
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service classes get projects](ProjectLocationServiceClassGetCall) (response)
/// * [locations service classes patch projects](ProjectLocationServiceClassPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceClass {
    /// Output only. Time when the ServiceClass was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of this resource.
    
    pub description: Option<String>,
    /// Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// User-defined labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of a ServiceClass resource. Format: projects/{project}/locations/{location}/serviceClasses/{service_class} See: https://google.aip.dev/122#fields-representing-resource-names
    
    pub name: Option<String>,
    /// Output only. The generated service class name. Use this name to refer to the Service class in Service Connection Maps and Service Connection Policies.
    #[serde(rename="serviceClass")]
    
    pub service_class: Option<String>,
    /// Output only. Time when the ServiceClass was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ServiceClass {}
impl client::ResponseResult for ServiceClass {}


/// The ServiceConnectionMap resource. Next id: 15
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service connection maps create projects](ProjectLocationServiceConnectionMapCreateCall) (request)
/// * [locations service connection maps get projects](ProjectLocationServiceConnectionMapGetCall) (response)
/// * [locations service connection maps patch projects](ProjectLocationServiceConnectionMapPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceConnectionMap {
    /// The PSC configurations on consumer side.
    #[serde(rename="consumerPscConfigs")]
    
    pub consumer_psc_configs: Option<Vec<ConsumerPscConfig>>,
    /// Output only. PSC connection details on consumer side.
    #[serde(rename="consumerPscConnections")]
    
    pub consumer_psc_connections: Option<Vec<ConsumerPscConnection>>,
    /// Output only. Time when the ServiceConnectionMap was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of this resource.
    
    pub description: Option<String>,
    /// Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The infrastructure used for connections between consumers/producers.
    
    pub infrastructure: Option<ServiceConnectionMapInfrastructureEnum>,
    /// User-defined labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of a ServiceConnectionMap. Format: projects/{project}/locations/{location}/serviceConnectionMaps/{service_connection_map} See: https://google.aip.dev/122#fields-representing-resource-names
    
    pub name: Option<String>,
    /// The PSC configurations on producer side.
    #[serde(rename="producerPscConfigs")]
    
    pub producer_psc_configs: Option<Vec<ProducerPscConfig>>,
    /// The service class identifier this ServiceConnectionMap is for. The user of ServiceConnectionMap create API needs to have networkconnecitivty.serviceclasses.use iam permission for the service class.
    #[serde(rename="serviceClass")]
    
    pub service_class: Option<String>,
    /// Output only. The service class uri this ServiceConnectionMap is for.
    #[serde(rename="serviceClassUri")]
    
    pub service_class_uri: Option<String>,
    /// The token provided by the consumer. This token authenticates that the consumer can create a connecton within the specified project and network.
    
    pub token: Option<String>,
    /// Output only. Time when the ServiceConnectionMap was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ServiceConnectionMap {}
impl client::ResponseResult for ServiceConnectionMap {}


/// The ServiceConnectionPolicy resource. Next id: 12
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service connection policies create projects](ProjectLocationServiceConnectionPolicyCreateCall) (request)
/// * [locations service connection policies get projects](ProjectLocationServiceConnectionPolicyGetCall) (response)
/// * [locations service connection policies patch projects](ProjectLocationServiceConnectionPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceConnectionPolicy {
    /// Output only. Time when the ServiceConnectionMap was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of this resource.
    
    pub description: Option<String>,
    /// Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The type of underlying resources used to create the connection.
    
    pub infrastructure: Option<ServiceConnectionPolicyInfrastructureEnum>,
    /// User-defined labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names
    
    pub name: Option<String>,
    /// The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}.
    
    pub network: Option<String>,
    /// Configuration used for Private Service Connect connections. Used when Infrastructure is PSC.
    #[serde(rename="pscConfig")]
    
    pub psc_config: Option<PscConfig>,
    /// Output only. [Output only] Information about each Private Service Connect connection.
    #[serde(rename="pscConnections")]
    
    pub psc_connections: Option<Vec<PscConnection>>,
    /// The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass. It is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.
    #[serde(rename="serviceClass")]
    
    pub service_class: Option<String>,
    /// Output only. Time when the ServiceConnectionMap was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ServiceConnectionPolicy {}
impl client::ResponseResult for ServiceConnectionPolicy {}


/// The ServiceConnectionToken resource. Next id: 10
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations service connection tokens create projects](ProjectLocationServiceConnectionTokenCreateCall) (request)
/// * [locations service connection tokens get projects](ProjectLocationServiceConnectionTokenGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceConnectionToken {
    /// Output only. Time when the ServiceConnectionToken was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A description of this resource.
    
    pub description: Option<String>,
    /// Optional. The etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The time to which this token is valid.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// User-defined labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. The name of a ServiceConnectionToken. Format: projects/{project}/locations/{location}/ServiceConnectionTokens/{service_connection_token} See: https://google.aip.dev/122#fields-representing-resource-names
    
    pub name: Option<String>,
    /// The resource path of the network associated with this token. Example: projects/{projectNumOrId}/global/networks/{resourceId}.
    
    pub network: Option<String>,
    /// Output only. The token generated by Automation.
    
    pub token: Option<String>,
    /// Output only. Time when the ServiceConnectionToken was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ServiceConnectionToken {}
impl client::ResponseResult for ServiceConnectionToken {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs groups set iam policy projects](ProjectLocationGlobalHubGroupSetIamPolicyCall) (request)
/// * [locations global hubs set iam policy projects](ProjectLocationGlobalHubSetIamPolicyCall) (request)
/// * [locations global policy based routes set iam policy projects](ProjectLocationGlobalPolicyBasedRouteSetIamPolicyCall) (request)
/// * [locations service classes set iam policy projects](ProjectLocationServiceClassSetIamPolicyCall) (request)
/// * [locations service connection maps set iam policy projects](ProjectLocationServiceConnectionMapSetIamPolicyCall) (request)
/// * [locations service connection policies set iam policy projects](ProjectLocationServiceConnectionPolicySetIamPolicyCall) (request)
/// * [locations spokes set iam policy projects](ProjectLocationSpokeSetIamPolicyCall) (request)
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


/// A Network Connectivity Center spoke represents one or more network connectivity resources. When you create a spoke, you associate it with a hub. You must also identify a value for exactly one of the following fields: * linked_vpn_tunnels * linked_interconnect_attachments * linked_router_appliance_instances * linked_vpc_network
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations spokes create projects](ProjectLocationSpokeCreateCall) (request)
/// * [locations spokes get projects](ProjectLocationSpokeGetCall) (response)
/// * [locations spokes patch projects](ProjectLocationSpokePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Spoke {
    /// Output only. The time the spoke was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// An optional description of the spoke.
    
    pub description: Option<String>,
    /// Optional. The name of the group that this spoke is associated with.
    
    pub group: Option<String>,
    /// Immutable. The name of the hub that this spoke is attached to.
    
    pub hub: Option<String>,
    /// Optional labels in key-value pair format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
    
    pub labels: Option<HashMap<String, String>>,
    /// VLAN attachments that are associated with the spoke.
    #[serde(rename="linkedInterconnectAttachments")]
    
    pub linked_interconnect_attachments: Option<LinkedInterconnectAttachments>,
    /// Router appliance instances that are associated with the spoke.
    #[serde(rename="linkedRouterApplianceInstances")]
    
    pub linked_router_appliance_instances: Option<LinkedRouterApplianceInstances>,
    /// Optional. VPC network that is associated with the spoke.
    #[serde(rename="linkedVpcNetwork")]
    
    pub linked_vpc_network: Option<LinkedVpcNetwork>,
    /// VPN tunnels that are associated with the spoke.
    #[serde(rename="linkedVpnTunnels")]
    
    pub linked_vpn_tunnels: Option<LinkedVpnTunnels>,
    /// Immutable. The name of the spoke. Spoke names must be unique. They use the following form: `projects/{project_number}/locations/{region}/spokes/{spoke_id}`
    
    pub name: Option<String>,
    /// Output only. The reasons for current state of the spoke. Only present when the spoke is in the `INACTIVE` state.
    
    pub reasons: Option<Vec<StateReason>>,
    /// Output only. The type of resource associated with the spoke.
    #[serde(rename="spokeType")]
    
    pub spoke_type: Option<SpokeSpokeTypeEnum>,
    /// Output only. The current lifecycle state of this spoke.
    
    pub state: Option<SpokeStateEnum>,
    /// Output only. The Google-generated UUID for the spoke. This value is unique across all spoke resources. If a spoke is deleted and another with the same name is created, the new spoke is assigned a different `unique_id`.
    #[serde(rename="uniqueId")]
    
    pub unique_id: Option<String>,
    /// Output only. The time the spoke was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Spoke {}
impl client::ResponseResult for Spoke {}


/// The number of spokes that are in a particular state and associated with a given hub.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpokeStateCount {
    /// Output only. The total number of spokes that are in this state and associated with a given hub.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Output only. The state of the spokes.
    
    pub state: Option<SpokeStateCountStateEnum>,
}

impl client::Part for SpokeStateCount {}


/// The number of spokes in the hub that are inactive for this reason.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpokeStateReasonCount {
    /// Output only. The total number of spokes that are inactive for a particular reason and associated with a given hub.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Output only. The reason that a spoke is inactive.
    #[serde(rename="stateReasonCode")]
    
    pub state_reason_code: Option<SpokeStateReasonCountStateReasonCodeEnum>,
}

impl client::Part for SpokeStateReasonCount {}


/// Summarizes information about the spokes associated with a hub. The summary includes a count of spokes according to type and according to state. If any spokes are inactive, the summary also lists the reasons they are inactive, including a count for each reason.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpokeSummary {
    /// Output only. Counts the number of spokes that are in each state and associated with a given hub.
    #[serde(rename="spokeStateCounts")]
    
    pub spoke_state_counts: Option<Vec<SpokeStateCount>>,
    /// Output only. Counts the number of spokes that are inactive for each possible reason and associated with a given hub.
    #[serde(rename="spokeStateReasonCounts")]
    
    pub spoke_state_reason_counts: Option<Vec<SpokeStateReasonCount>>,
    /// Output only. Counts the number of spokes of each type that are associated with a specific hub.
    #[serde(rename="spokeTypeCounts")]
    
    pub spoke_type_counts: Option<Vec<SpokeTypeCount>>,
}

impl client::Part for SpokeSummary {}


/// The number of spokes of a given type that are associated with a specific hub. The type indicates what kind of resource is associated with the spoke.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpokeTypeCount {
    /// Output only. The total number of spokes of this type that are associated with the hub.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Output only. The type of the spokes.
    #[serde(rename="spokeType")]
    
    pub spoke_type: Option<SpokeTypeCountSpokeTypeEnum>,
}

impl client::Part for SpokeTypeCount {}


/// The reason a spoke is inactive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StateReason {
    /// The code associated with this reason.
    
    pub code: Option<StateReasonCodeEnum>,
    /// Human-readable details about this reason.
    
    pub message: Option<String>,
    /// Additional information provided by the user in the RejectSpoke call.
    #[serde(rename="userDetails")]
    
    pub user_details: Option<String>,
}

impl client::Part for StateReason {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations global hubs groups test iam permissions projects](ProjectLocationGlobalHubGroupTestIamPermissionCall) (request)
/// * [locations global hubs test iam permissions projects](ProjectLocationGlobalHubTestIamPermissionCall) (request)
/// * [locations global policy based routes test iam permissions projects](ProjectLocationGlobalPolicyBasedRouteTestIamPermissionCall) (request)
/// * [locations service classes test iam permissions projects](ProjectLocationServiceClassTestIamPermissionCall) (request)
/// * [locations service connection maps test iam permissions projects](ProjectLocationServiceConnectionMapTestIamPermissionCall) (request)
/// * [locations service connection policies test iam permissions projects](ProjectLocationServiceConnectionPolicyTestIamPermissionCall) (request)
/// * [locations spokes test iam permissions projects](ProjectLocationSpokeTestIamPermissionCall) (request)
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
/// * [locations global hubs groups test iam permissions projects](ProjectLocationGlobalHubGroupTestIamPermissionCall) (response)
/// * [locations global hubs test iam permissions projects](ProjectLocationGlobalHubTestIamPermissionCall) (response)
/// * [locations global policy based routes test iam permissions projects](ProjectLocationGlobalPolicyBasedRouteTestIamPermissionCall) (response)
/// * [locations service classes test iam permissions projects](ProjectLocationServiceClassTestIamPermissionCall) (response)
/// * [locations service connection maps test iam permissions projects](ProjectLocationServiceConnectionMapTestIamPermissionCall) (response)
/// * [locations service connection policies test iam permissions projects](ProjectLocationServiceConnectionPolicyTestIamPermissionCall) (response)
/// * [locations spokes test iam permissions projects](ProjectLocationSpokeTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// VM instances to which this policy-based route applies to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualMachine {
    /// Optional. A list of VM instance tags the this policy-based route applies to. VM instances that have ANY of tags specified here will install this PBR.
    
    pub tags: Option<Vec<String>>,
}

impl client::Part for VirtualMachine {}


/// Informational warning message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Warnings {
    /// Output only. A warning code, if applicable.
    
    pub code: Option<WarningCodeEnum>,
    /// Output only. Metadata about this warning in key: value format. The key should provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement.
    
    pub data: Option<HashMap<String, String>>,
    /// Output only. A human-readable description of the warning code.
    #[serde(rename="warningMessage")]
    
    pub warning_message: Option<String>,
}

impl client::Part for Warnings {}


