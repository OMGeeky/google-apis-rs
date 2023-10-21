use super::*;
/// The Label to be applied to the resource views.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    /// Key of the label.
    
    pub key: Option<String>,
    /// Value of the label.
    
    pub value: Option<String>,
}

impl client::Part for Label {}


/// The list response item that contains the resource and end points information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListResourceResponseItem {
    /// The list of service end points on the resource.
    
    pub endpoints: Option<HashMap<String, Vec<i32>>>,
    /// The full URL of the resource.
    
    pub resource: Option<String>,
}

impl client::Part for ListResourceResponseItem {}


/// An operation resource, used to manage asynchronous API requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get zone operations](ZoneOperationGetCall) (response)
/// * [add resources zone views](ZoneViewAddResourceCall) (response)
/// * [delete zone views](ZoneViewDeleteCall) (response)
/// * [insert zone views](ZoneViewInsertCall) (response)
/// * [remove resources zone views](ZoneViewRemoveResourceCall) (response)
/// * [set service zone views](ZoneViewSetServiceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// [Output only] An optional identifier specified by the client when the mutation was initiated. Must be unique for all operation resources in the project.
    #[serde(rename="clientOperationId")]
    
    pub client_operation_id: Option<String>,
    /// [Output Only] The time that this operation was requested, in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// [Output Only] The time that this operation was completed, in RFC3339 text format.
    #[serde(rename="endTime")]
    
    pub end_time: Option<String>,
    /// [Output Only] If errors occurred during processing of this operation, this field will be populated.
    
    pub error: Option<OperationError>,
    /// [Output only] If operation fails, the HTTP error message returned.
    #[serde(rename="httpErrorMessage")]
    
    pub http_error_message: Option<String>,
    /// [Output only] If operation fails, the HTTP error status code returned.
    #[serde(rename="httpErrorStatusCode")]
    
    pub http_error_status_code: Option<i32>,
    /// [Output Only] Unique identifier for the resource, generated by the server.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// [Output Only] The time that this operation was requested, in RFC3339 text format.
    #[serde(rename="insertTime")]
    
    pub insert_time: Option<String>,
    /// [Output only] Type of the resource.
    
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    
    pub name: Option<String>,
    /// [Output only] Type of the operation. Operations include insert, update, and delete.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// [Output only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the operation will be complete. This number should be monotonically increasing as the operation progresses.
    
    pub progress: Option<i32>,
    /// [Output Only] URL of the region where the operation resides. Only available when performing regional operations.
    
    pub region: Option<String>,
    /// [Output Only] Server-defined fully-qualified URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server, in RFC3339 text format.
    #[serde(rename="startTime")]
    
    pub start_time: Option<String>,
    /// [Output Only] Status of the operation.
    
    pub status: Option<String>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    
    pub status_message: Option<String>,
    /// [Output Only] Unique target ID which identifies a particular incarnation of the target.
    #[serde(rename="targetId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_id: Option<u64>,
    /// [Output only] URL of the resource the operation is mutating.
    #[serde(rename="targetLink")]
    
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: user@example.com.
    
    pub user: Option<String>,
    /// [Output Only] If there are issues with this operation, a warning is returned.
    
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] URL of the zone where the operation resides. Only available when performing per-zone operations.
    
    pub zone: Option<String>,
}

impl client::ResponseResult for Operation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list zone operations](ZoneOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationList {
    /// Unique identifier for the resource; defined by the server (output only).
    
    pub id: Option<String>,
    /// The operation resources.
    
    pub items: Option<Vec<Operation>>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// A token used to continue a truncated list request (output only).
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for OperationList {}


/// The resource view object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get zone views](ZoneViewGetCall) (response)
/// * [insert zone views](ZoneViewInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceView {
    /// The creation time of the resource view.
    #[serde(rename="creationTimestamp")]
    
    pub creation_timestamp: Option<String>,
    /// The detailed description of the resource view.
    
    pub description: Option<String>,
    /// Services endpoint information.
    
    pub endpoints: Option<Vec<ServiceEndpoint>>,
    /// The fingerprint of the service endpoint information.
    
    pub fingerprint: Option<String>,
    /// [Output Only] The ID of the resource view.
    
    pub id: Option<String>,
    /// Type of the resource.
    
    pub kind: Option<String>,
    /// The labels for events.
    
    pub labels: Option<Vec<Label>>,
    /// The name of the resource view.
    
    pub name: Option<String>,
    /// The URL of a Compute Engine network to which the resources in the view belong.
    
    pub network: Option<String>,
    /// A list of all resources in the resource view.
    
    pub resources: Option<Vec<String>>,
    /// [Output Only] A self-link to the resource view.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The total number of resources in the resource view.
    
    pub size: Option<u32>,
}

impl client::RequestValue for ResourceView {}
impl client::ResponseResult for ResourceView {}


/// The service endpoint that may be started in a VM.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// The name of the service endpoint.
    
    pub name: Option<String>,
    /// The port of the service endpoint.
    
    pub port: Option<i32>,
}

impl client::Part for ServiceEndpoint {}


/// The request to add resources to the resource view.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add resources zone views](ZoneViewAddResourceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneViewsAddResourcesRequest {
    /// The list of resources to be added.
    
    pub resources: Option<Vec<String>>,
}

impl client::RequestValue for ZoneViewsAddResourcesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get service zone views](ZoneViewGetServiceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneViewsGetServiceResponse {
    /// The service information.
    
    pub endpoints: Option<Vec<ServiceEndpoint>>,
    /// The fingerprint of the service information.
    
    pub fingerprint: Option<String>,
}

impl client::ResponseResult for ZoneViewsGetServiceResponse {}


/// The response to a list request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list zone views](ZoneViewListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneViewsList {
    /// The result that contains all resource views that meet the criteria.
    
    pub items: Option<Vec<ResourceView>>,
    /// Type of resource.
    
    pub kind: Option<String>,
    /// A token used for pagination.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for ZoneViewsList {}


/// The response to a list resource request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list resources zone views](ZoneViewListResourceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneViewsListResourcesResponse {
    /// The formatted JSON that is requested by the user.
    
    pub items: Option<Vec<ListResourceResponseItem>>,
    /// The URL of a Compute Engine network to which the resources in the view belong.
    
    pub network: Option<String>,
    /// A token used for pagination.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ZoneViewsListResourcesResponse {}


/// The request to remove resources from the resource view.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remove resources zone views](ZoneViewRemoveResourceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneViewsRemoveResourcesRequest {
    /// The list of resources to be removed.
    
    pub resources: Option<Vec<String>>,
}

impl client::RequestValue for ZoneViewsRemoveResourcesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set service zone views](ZoneViewSetServiceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneViewsSetServiceRequest {
    /// The service information to be updated.
    
    pub endpoints: Option<Vec<ServiceEndpoint>>,
    /// Fingerprint of the service information; a hash of the contents. This field is used for optimistic locking when updating the service entries.
    
    pub fingerprint: Option<String>,
    /// The name of the resource if user wants to update the service information of the resource.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
}

impl client::RequestValue for ZoneViewsSetServiceRequest {}


/// [Output Only] If errors occurred during processing of this operation, this field will be populated.
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
    /// [Output Only] Indicates the field in the request which caused the error. This property is optional.
    
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationErrorErrors {}
impl client::Part for OperationErrorErrors {}


/// [Output Only] If there are issues with this operation, a warning is returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarnings {
    /// [Output only] The warning type identifier for this warning.
    
    pub code: Option<String>,
    /// [Output only] Metadata for this warning in key:value format.
    
    pub data: Option<Vec<OperationWarningsData>>,
    /// [Output only] Optional human-readable details for this warning.
    
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// [Output only] Metadata for this warning in key:value format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarningsData {
    /// [Output Only] Metadata key for this warning.
    
    pub key: Option<String>,
    /// [Output Only] Metadata value for this warning.
    
    pub value: Option<String>,
}

impl client::NestedType for OperationWarningsData {}
impl client::Part for OperationWarningsData {}


