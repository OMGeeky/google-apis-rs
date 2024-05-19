use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Networkconnectivity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_networkconnectivity1 as networkconnectivity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use networkconnectivity1::{Networkconnectivity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Networkconnectivity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_get(...)`, `locations_global_hubs_accept_spoke(...)`, `locations_global_hubs_create(...)`, `locations_global_hubs_delete(...)`, `locations_global_hubs_get(...)`, `locations_global_hubs_get_iam_policy(...)`, `locations_global_hubs_groups_get(...)`, `locations_global_hubs_groups_get_iam_policy(...)`, `locations_global_hubs_groups_list(...)`, `locations_global_hubs_groups_set_iam_policy(...)`, `locations_global_hubs_groups_test_iam_permissions(...)`, `locations_global_hubs_list(...)`, `locations_global_hubs_list_spokes(...)`, `locations_global_hubs_patch(...)`, `locations_global_hubs_reject_spoke(...)`, `locations_global_hubs_route_tables_get(...)`, `locations_global_hubs_route_tables_list(...)`, `locations_global_hubs_route_tables_routes_get(...)`, `locations_global_hubs_route_tables_routes_list(...)`, `locations_global_hubs_set_iam_policy(...)`, `locations_global_hubs_test_iam_permissions(...)`, `locations_global_policy_based_routes_create(...)`, `locations_global_policy_based_routes_delete(...)`, `locations_global_policy_based_routes_get(...)`, `locations_global_policy_based_routes_get_iam_policy(...)`, `locations_global_policy_based_routes_list(...)`, `locations_global_policy_based_routes_set_iam_policy(...)`, `locations_global_policy_based_routes_test_iam_permissions(...)`, `locations_internal_ranges_create(...)`, `locations_internal_ranges_delete(...)`, `locations_internal_ranges_get(...)`, `locations_internal_ranges_list(...)`, `locations_internal_ranges_patch(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_service_classes_delete(...)`, `locations_service_classes_get(...)`, `locations_service_classes_get_iam_policy(...)`, `locations_service_classes_list(...)`, `locations_service_classes_patch(...)`, `locations_service_classes_set_iam_policy(...)`, `locations_service_classes_test_iam_permissions(...)`, `locations_service_connection_maps_create(...)`, `locations_service_connection_maps_delete(...)`, `locations_service_connection_maps_get(...)`, `locations_service_connection_maps_get_iam_policy(...)`, `locations_service_connection_maps_list(...)`, `locations_service_connection_maps_patch(...)`, `locations_service_connection_maps_set_iam_policy(...)`, `locations_service_connection_maps_test_iam_permissions(...)`, `locations_service_connection_policies_create(...)`, `locations_service_connection_policies_delete(...)`, `locations_service_connection_policies_get(...)`, `locations_service_connection_policies_get_iam_policy(...)`, `locations_service_connection_policies_list(...)`, `locations_service_connection_policies_patch(...)`, `locations_service_connection_policies_set_iam_policy(...)`, `locations_service_connection_policies_test_iam_permissions(...)`, `locations_service_connection_tokens_create(...)`, `locations_service_connection_tokens_delete(...)`, `locations_service_connection_tokens_get(...)`, `locations_service_connection_tokens_list(...)`, `locations_spokes_create(...)`, `locations_spokes_delete(...)`, `locations_spokes_get(...)`, `locations_spokes_get_iam_policy(...)`, `locations_spokes_list(...)`, `locations_spokes_patch(...)`, `locations_spokes_set_iam_policy(...)` and `locations_spokes_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Networkconnectivity<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a Network Connectivity Center group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the route table resource.
    pub fn locations_global_hubs_groups_get(&self, name: &str) -> ProjectLocationGlobalHubGroupGetCall<'a, S> {
        ProjectLocationGlobalHubGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_hubs_groups_get_iam_policy(&self, resource: &str) -> ProjectLocationGlobalHubGroupGetIamPolicyCall<'a, S> {
        ProjectLocationGlobalHubGroupGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists groups in a given hub.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name.
    pub fn locations_global_hubs_groups_list(&self, parent: &str) -> ProjectLocationGlobalHubGroupListCall<'a, S> {
        ProjectLocationGlobalHubGroupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_hubs_groups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGlobalHubGroupSetIamPolicyCall<'a, S> {
        ProjectLocationGlobalHubGroupSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_hubs_groups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGlobalHubGroupTestIamPermissionCall<'a, S> {
        ProjectLocationGlobalHubGroupTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about the specified route.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the route resource.
    pub fn locations_global_hubs_route_tables_routes_get(&self, name: &str) -> ProjectLocationGlobalHubRouteTableRouteGetCall<'a, S> {
        ProjectLocationGlobalHubRouteTableRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists routes in a given route table.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name.
    pub fn locations_global_hubs_route_tables_routes_list(&self, parent: &str) -> ProjectLocationGlobalHubRouteTableRouteListCall<'a, S> {
        ProjectLocationGlobalHubRouteTableRouteListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a Network Connectivity Center route table.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the route table resource.
    pub fn locations_global_hubs_route_tables_get(&self, name: &str) -> ProjectLocationGlobalHubRouteTableGetCall<'a, S> {
        ProjectLocationGlobalHubRouteTableGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists route tables in a given hub.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name.
    pub fn locations_global_hubs_route_tables_list(&self, parent: &str) -> ProjectLocationGlobalHubRouteTableListCall<'a, S> {
        ProjectLocationGlobalHubRouteTableListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts a proposal to attach a Network Connectivity Center spoke to a hub.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the hub into which to accept the spoke.
    pub fn locations_global_hubs_accept_spoke(&self, request: AcceptHubSpokeRequest, name: &str) -> ProjectLocationGlobalHubAcceptSpokeCall<'a, S> {
        ProjectLocationGlobalHubAcceptSpokeCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Network Connectivity Center hub in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource.
    pub fn locations_global_hubs_create(&self, request: Hub, parent: &str) -> ProjectLocationGlobalHubCreateCall<'a, S> {
        ProjectLocationGlobalHubCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _hub_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Network Connectivity Center hub.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the hub to delete.
    pub fn locations_global_hubs_delete(&self, name: &str) -> ProjectLocationGlobalHubDeleteCall<'a, S> {
        ProjectLocationGlobalHubDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a Network Connectivity Center hub.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the hub resource to get.
    pub fn locations_global_hubs_get(&self, name: &str) -> ProjectLocationGlobalHubGetCall<'a, S> {
        ProjectLocationGlobalHubGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_hubs_get_iam_policy(&self, resource: &str) -> ProjectLocationGlobalHubGetIamPolicyCall<'a, S> {
        ProjectLocationGlobalHubGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Network Connectivity Center hubs associated with a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name.
    pub fn locations_global_hubs_list(&self, parent: &str) -> ProjectLocationGlobalHubListCall<'a, S> {
        ProjectLocationGlobalHubListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Network Connectivity Center spokes associated with a specified hub and location. The list includes both spokes that are attached to the hub and spokes that have been proposed but not yet accepted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the hub.
    pub fn locations_global_hubs_list_spokes(&self, name: &str) -> ProjectLocationGlobalHubListSpokeCall<'a, S> {
        ProjectLocationGlobalHubListSpokeCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _spoke_locations: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the description and/or labels of a Network Connectivity Center hub.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of the hub. Hub names must be unique. They use the following form: `projects/{project_number}/locations/global/hubs/{hub_id}`
    pub fn locations_global_hubs_patch(&self, request: Hub, name: &str) -> ProjectLocationGlobalHubPatchCall<'a, S> {
        ProjectLocationGlobalHubPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rejects a Network Connectivity Center spoke from being attached to a hub. If the spoke was previously in the `ACTIVE` state, it transitions to the `INACTIVE` state and is no longer able to connect to other spokes that are attached to the hub.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the hub from which to reject the spoke.
    pub fn locations_global_hubs_reject_spoke(&self, request: RejectHubSpokeRequest, name: &str) -> ProjectLocationGlobalHubRejectSpokeCall<'a, S> {
        ProjectLocationGlobalHubRejectSpokeCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_hubs_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGlobalHubSetIamPolicyCall<'a, S> {
        ProjectLocationGlobalHubSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_hubs_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGlobalHubTestIamPermissionCall<'a, S> {
        ProjectLocationGlobalHubTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new policy-based route in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource's name of the PolicyBasedRoute.
    pub fn locations_global_policy_based_routes_create(&self, request: PolicyBasedRoute, parent: &str) -> ProjectLocationGlobalPolicyBasedRouteCreateCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _policy_based_route_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single policy-based route.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the policy-based route resource to delete.
    pub fn locations_global_policy_based_routes_delete(&self, name: &str) -> ProjectLocationGlobalPolicyBasedRouteDeleteCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single policy-based route.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the PolicyBasedRoute resource to get.
    pub fn locations_global_policy_based_routes_get(&self, name: &str) -> ProjectLocationGlobalPolicyBasedRouteGetCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_policy_based_routes_get_iam_policy(&self, resource: &str) -> ProjectLocationGlobalPolicyBasedRouteGetIamPolicyCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists policy-based routes in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name.
    pub fn locations_global_policy_based_routes_list(&self, parent: &str) -> ProjectLocationGlobalPolicyBasedRouteListCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_policy_based_routes_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationGlobalPolicyBasedRouteSetIamPolicyCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_global_policy_based_routes_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationGlobalPolicyBasedRouteTestIamPermissionCall<'a, S> {
        ProjectLocationGlobalPolicyBasedRouteTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new internal range in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource's name of the internal range.
    pub fn locations_internal_ranges_create(&self, request: InternalRange, parent: &str) -> ProjectLocationInternalRangeCreateCall<'a, S> {
        ProjectLocationInternalRangeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _internal_range_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single internal range.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the internal range to delete.
    pub fn locations_internal_ranges_delete(&self, name: &str) -> ProjectLocationInternalRangeDeleteCall<'a, S> {
        ProjectLocationInternalRangeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single internal range.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the InternalRange to get.
    pub fn locations_internal_ranges_get(&self, name: &str) -> ProjectLocationInternalRangeGetCall<'a, S> {
        ProjectLocationInternalRangeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists internal ranges in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name.
    pub fn locations_internal_ranges_list(&self, parent: &str) -> ProjectLocationInternalRangeListCall<'a, S> {
        ProjectLocationInternalRangeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single internal range.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of an internal range. Format: projects/{project}/locations/{location}/internalRanges/{internal_range} See: https://google.aip.dev/122#fields-representing-resource-names
    pub fn locations_internal_ranges_patch(&self, request: InternalRange, name: &str) -> ProjectLocationInternalRangePatchCall<'a, S> {
        ProjectLocationInternalRangePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, S> {
        ProjectLocationOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServiceClass.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ServiceClass to delete.
    pub fn locations_service_classes_delete(&self, name: &str) -> ProjectLocationServiceClassDeleteCall<'a, S> {
        ProjectLocationServiceClassDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServiceClass.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ServiceClass to get.
    pub fn locations_service_classes_get(&self, name: &str) -> ProjectLocationServiceClassGetCall<'a, S> {
        ProjectLocationServiceClassGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_classes_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceClassGetIamPolicyCall<'a, S> {
        ProjectLocationServiceClassGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists ServiceClasses in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name. ex. projects/123/locations/us-east1
    pub fn locations_service_classes_list(&self, parent: &str) -> ProjectLocationServiceClassListCall<'a, S> {
        ProjectLocationServiceClassListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single ServiceClass.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of a ServiceClass resource. Format: projects/{project}/locations/{location}/serviceClasses/{service_class} See: https://google.aip.dev/122#fields-representing-resource-names
    pub fn locations_service_classes_patch(&self, request: ServiceClass, name: &str) -> ProjectLocationServiceClassPatchCall<'a, S> {
        ProjectLocationServiceClassPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_classes_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceClassSetIamPolicyCall<'a, S> {
        ProjectLocationServiceClassSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_classes_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceClassTestIamPermissionCall<'a, S> {
        ProjectLocationServiceClassTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ServiceConnectionMap in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource's name of the ServiceConnectionMap. ex. projects/123/locations/us-east1
    pub fn locations_service_connection_maps_create(&self, request: ServiceConnectionMap, parent: &str) -> ProjectLocationServiceConnectionMapCreateCall<'a, S> {
        ProjectLocationServiceConnectionMapCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_connection_map_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServiceConnectionMap.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ServiceConnectionMap to delete.
    pub fn locations_service_connection_maps_delete(&self, name: &str) -> ProjectLocationServiceConnectionMapDeleteCall<'a, S> {
        ProjectLocationServiceConnectionMapDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServiceConnectionMap.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ServiceConnectionMap to get.
    pub fn locations_service_connection_maps_get(&self, name: &str) -> ProjectLocationServiceConnectionMapGetCall<'a, S> {
        ProjectLocationServiceConnectionMapGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_connection_maps_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceConnectionMapGetIamPolicyCall<'a, S> {
        ProjectLocationServiceConnectionMapGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists ServiceConnectionMaps in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name. ex. projects/123/locations/us-east1
    pub fn locations_service_connection_maps_list(&self, parent: &str) -> ProjectLocationServiceConnectionMapListCall<'a, S> {
        ProjectLocationServiceConnectionMapListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single ServiceConnectionMap.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of a ServiceConnectionMap. Format: projects/{project}/locations/{location}/serviceConnectionMaps/{service_connection_map} See: https://google.aip.dev/122#fields-representing-resource-names
    pub fn locations_service_connection_maps_patch(&self, request: ServiceConnectionMap, name: &str) -> ProjectLocationServiceConnectionMapPatchCall<'a, S> {
        ProjectLocationServiceConnectionMapPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_connection_maps_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceConnectionMapSetIamPolicyCall<'a, S> {
        ProjectLocationServiceConnectionMapSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_connection_maps_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceConnectionMapTestIamPermissionCall<'a, S> {
        ProjectLocationServiceConnectionMapTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ServiceConnectionPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource's name of the ServiceConnectionPolicy. ex. projects/123/locations/us-east1
    pub fn locations_service_connection_policies_create(&self, request: ServiceConnectionPolicy, parent: &str) -> ProjectLocationServiceConnectionPolicyCreateCall<'a, S> {
        ProjectLocationServiceConnectionPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_connection_policy_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServiceConnectionPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ServiceConnectionPolicy to delete.
    pub fn locations_service_connection_policies_delete(&self, name: &str) -> ProjectLocationServiceConnectionPolicyDeleteCall<'a, S> {
        ProjectLocationServiceConnectionPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServiceConnectionPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ServiceConnectionPolicy to get.
    pub fn locations_service_connection_policies_get(&self, name: &str) -> ProjectLocationServiceConnectionPolicyGetCall<'a, S> {
        ProjectLocationServiceConnectionPolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_connection_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationServiceConnectionPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationServiceConnectionPolicyGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists ServiceConnectionPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name. ex. projects/123/locations/us-east1
    pub fn locations_service_connection_policies_list(&self, parent: &str) -> ProjectLocationServiceConnectionPolicyListCall<'a, S> {
        ProjectLocationServiceConnectionPolicyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single ServiceConnectionPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names
    pub fn locations_service_connection_policies_patch(&self, request: ServiceConnectionPolicy, name: &str) -> ProjectLocationServiceConnectionPolicyPatchCall<'a, S> {
        ProjectLocationServiceConnectionPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_connection_policies_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationServiceConnectionPolicySetIamPolicyCall<'a, S> {
        ProjectLocationServiceConnectionPolicySetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_service_connection_policies_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationServiceConnectionPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationServiceConnectionPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ServiceConnectionToken in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource's name of the ServiceConnectionToken. ex. projects/123/locations/us-east1
    pub fn locations_service_connection_tokens_create(&self, request: ServiceConnectionToken, parent: &str) -> ProjectLocationServiceConnectionTokenCreateCall<'a, S> {
        ProjectLocationServiceConnectionTokenCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _service_connection_token_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServiceConnectionToken.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the ServiceConnectionToken to delete.
    pub fn locations_service_connection_tokens_delete(&self, name: &str) -> ProjectLocationServiceConnectionTokenDeleteCall<'a, S> {
        ProjectLocationServiceConnectionTokenDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServiceConnectionToken.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ServiceConnectionToken to get.
    pub fn locations_service_connection_tokens_get(&self, name: &str) -> ProjectLocationServiceConnectionTokenGetCall<'a, S> {
        ProjectLocationServiceConnectionTokenGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists ServiceConnectionTokens in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource's name. ex. projects/123/locations/us-east1
    pub fn locations_service_connection_tokens_list(&self, parent: &str) -> ProjectLocationServiceConnectionTokenListCall<'a, S> {
        ProjectLocationServiceConnectionTokenListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Network Connectivity Center spoke.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource.
    pub fn locations_spokes_create(&self, request: Spoke, parent: &str) -> ProjectLocationSpokeCreateCall<'a, S> {
        ProjectLocationSpokeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _spoke_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Network Connectivity Center spoke.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the spoke to delete.
    pub fn locations_spokes_delete(&self, name: &str) -> ProjectLocationSpokeDeleteCall<'a, S> {
        ProjectLocationSpokeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a Network Connectivity Center spoke.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the spoke resource.
    pub fn locations_spokes_get(&self, name: &str) -> ProjectLocationSpokeGetCall<'a, S> {
        ProjectLocationSpokeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_spokes_get_iam_policy(&self, resource: &str) -> ProjectLocationSpokeGetIamPolicyCall<'a, S> {
        ProjectLocationSpokeGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Network Connectivity Center spokes in a specified project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource.
    pub fn locations_spokes_list(&self, parent: &str) -> ProjectLocationSpokeListCall<'a, S> {
        ProjectLocationSpokeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a Network Connectivity Center spoke.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The name of the spoke. Spoke names must be unique. They use the following form: `projects/{project_number}/locations/{region}/spokes/{spoke_id}`
    pub fn locations_spokes_patch(&self, request: Spoke, name: &str) -> ProjectLocationSpokePatchCall<'a, S> {
        ProjectLocationSpokePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_spokes_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectLocationSpokeSetIamPolicyCall<'a, S> {
        ProjectLocationSpokeSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_spokes_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectLocationSpokeTestIamPermissionCall<'a, S> {
        ProjectLocationSpokeTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



