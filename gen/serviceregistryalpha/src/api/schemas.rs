use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete endpoints](EndpointDeleteCall) (none)
/// * [get endpoints](EndpointGetCall) (response)
/// * [insert endpoints](EndpointInsertCall) (request)
/// * [list endpoints](EndpointListCall) (none)
/// * [patch endpoints](EndpointPatchCall) (request)
/// * [update endpoints](EndpointUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    /// A user-provided address of the service represented by this endpoint. This can be an IPv4 or IPv6 address, or a hostname.
    
    pub address: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// An optional user-provided description of the endpoint.
    
    pub description: Option<String>,
    /// Supply the fingerprint value for update requests. The fingerprint value is generated by the server and ensures optimistic concurrency (so that only one update can be performed at a time). The fingerprint changes after each update.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub fingerprint: Option<Vec<u8>>,
    /// [Output Only] Unique identifier for the resource; defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// A user-provided name of the endpoint, which must be unique within the project. The name must comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    
    pub name: Option<String>,
    /// An optional user-provided port of the service represented by this endpoint.
    
    pub port: Option<i32>,
    /// [Output Only] Self link for the endpoint.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The current state of the endpoint, as determined by the system.
    
    pub state: Option<String>,
    /// The DNS Integration configuration for this endpoint. This must be a list of fully-qualified URLs to Compute Engine networks.
    
    pub visibility: Option<EndpointEndpointVisibility>,
}

impl client::RequestValue for Endpoint {}
impl client::Resource for Endpoint {}
impl client::ResponseResult for Endpoint {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointEndpointVisibility {
    /// Google Compute Engine networks for which the name of this endpoint should be resolvable through DNS.
    
    pub networks: Option<Vec<String>>,
}

impl client::Part for EndpointEndpointVisibility {}


/// A response containing a partial list of Endpoints and a page token used to build the next request if the request has been truncated. Next available tag: 6
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list endpoints](EndpointListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndpointsListResponse {
    /// The endpoints contained in this response.
    
    pub endpoints: Option<Vec<Endpoint>>,
    /// [Output Only] This token allows you to get the next page of results for list requests. If the number of results is larger than maxResults, use the nextPageToken as a value for the query parameter pageToken in the next list request. Subsequent list requests will have their own nextPageToken to continue paging through the results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for EndpointsListResponse {}


/// An Operation resource, used to manage asynchronous API requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete endpoints](EndpointDeleteCall) (response)
/// * [insert endpoints](EndpointInsertCall) (response)
/// * [patch endpoints](EndpointPatchCall) (response)
/// * [update endpoints](EndpointUpdateCall) (response)
/// * [get operations](OperationGetCall) (response)
/// * [list operations](OperationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// [Output Only] Reserved for future use.
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// [Output Only] A textual description of the operation, which is set when the operation is created.
    
    pub description: Option<String>,
    /// [Output Only] The time that this operation was completed. This value is in RFC3339 text format.
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// [Output Only] If errors are generated during processing of the operation, this field will be populated.
    
    pub error: Option<OperationError>,
    /// [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as NOT FOUND.
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a 404 means the resource was not found.
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] The time that this operation was requested. This value is in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output Only] Type of the resource. Always compute#operation for Operation resources.
    
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    
    pub name: Option<String>,
    /// [Output Only] The type of operation, such as insert, update, or delete, and so on.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses.
    
    pub progress: Option<i32>,
    /// [Output Only] The URL of the region where the operation resides. Only available when performing regional operations.
    
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// [Output Only] The status of the operation, which can be one of the following: PENDING, RUNNING, or DONE.
    
    pub status: Option<String>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] The unique target ID, which identifies a specific incarnation of the target resource.
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// [Output Only] The URL of the resource that the operation modifies.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: user@example.com.
    
    pub user: Option<String>,
    /// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] The URL of the zone where the operation resides. Only available when performing per-zone operations.
    
    pub zone: Option<String>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// A response containing a partial list of operations and a page token used to build the next request if the request has been truncated.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationsListResponse {
    /// [Output Only] A token used to continue a truncated list request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// [Output Only] Operations contained in this list response.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for OperationsListResponse {}


/// [Output Only] If errors are generated during processing of the operation, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationError {
    /// [Output Only] The array of errors encountered while processing this operation.
    
    pub errors: Option<Vec<OperationErrorErrors>>,
}

impl client::NestedType for OperationError {}
impl client::Part for OperationError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationErrorErrors {
    /// [Output Only] The error type identifier for this error.
    
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request that caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationErrorErrors {}
impl client::Part for OperationErrorErrors {}


/// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarnings {
    /// [Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response.
    
    pub code: Option<String>,
    /// [Output Only] Metadata about this warning in key: value format. For example:
    /// "data": [ { "key": "scope", "value": "zones/us-east1-d" }
    
    pub data: Option<Vec<OperationWarningsData>>,
    /// [Output Only] A human-readable description of the warning code.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// [Output Only] Metadata about this warning in key: value format. For example:
/// "data": [ { "key": "scope", "value": "zones/us-east1-d" }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarningsData {
    /// [Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding).
    
    pub key: Option<String>,
    /// [Output Only] A warning data value corresponding to the key.
    
    pub value: Option<String>,
}

impl client::NestedType for OperationWarningsData {}
impl client::Part for OperationWarningsData {}


