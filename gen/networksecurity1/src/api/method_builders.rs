use super::*;
/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`NetworkSecurity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_networksecurity1 as networksecurity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use networksecurity1::{NetworkSecurity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = NetworkSecurity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_address_groups_add_items(...)`, `locations_address_groups_clone_items(...)`, `locations_address_groups_create(...)`, `locations_address_groups_delete(...)`, `locations_address_groups_get(...)`, `locations_address_groups_list(...)`, `locations_address_groups_list_references(...)`, `locations_address_groups_patch(...)`, `locations_address_groups_remove_items(...)`, `locations_firewall_endpoints_create(...)`, `locations_firewall_endpoints_delete(...)`, `locations_firewall_endpoints_get(...)`, `locations_firewall_endpoints_list(...)`, `locations_firewall_endpoints_patch(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_security_profile_groups_create(...)`, `locations_security_profile_groups_delete(...)`, `locations_security_profile_groups_get(...)`, `locations_security_profile_groups_list(...)`, `locations_security_profile_groups_patch(...)`, `locations_security_profiles_create(...)`, `locations_security_profiles_delete(...)`, `locations_security_profiles_get(...)`, `locations_security_profiles_list(...)` and `locations_security_profiles_patch(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a NetworkSecurity<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds items to an address group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `addressGroup` - Required. A name of the AddressGroup to add items to. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_add_items(&self, request: AddAddressGroupItemsRequest, address_group: &str) -> OrganizationLocationAddressGroupAddItemCall<'a, S> {
        OrganizationLocationAddressGroupAddItemCall {
            hub: self.hub,
            _request: request,
            _address_group: address_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clones items from one address group to another.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `addressGroup` - Required. A name of the AddressGroup to clone items to. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_clone_items(&self, request: CloneAddressGroupItemsRequest, address_group: &str) -> OrganizationLocationAddressGroupCloneItemCall<'a, S> {
        OrganizationLocationAddressGroupCloneItemCall {
            hub: self.hub,
            _request: request,
            _address_group: address_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new address group in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the AddressGroup. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_address_groups_create(&self, request: AddressGroup, parent: &str) -> OrganizationLocationAddressGroupCreateCall<'a, S> {
        OrganizationLocationAddressGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _address_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an address group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AddressGroup to delete. Must be in the format `projects/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_delete(&self, name: &str) -> OrganizationLocationAddressGroupDeleteCall<'a, S> {
        OrganizationLocationAddressGroupDeleteCall {
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
    /// Gets details of a single address group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AddressGroup to get. Must be in the format `projects/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_get(&self, name: &str) -> OrganizationLocationAddressGroupGetCall<'a, S> {
        OrganizationLocationAddressGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists address groups in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the AddressGroups should be listed, specified in the format `projects/*/locations/{location}`.
    pub fn locations_address_groups_list(&self, parent: &str) -> OrganizationLocationAddressGroupListCall<'a, S> {
        OrganizationLocationAddressGroupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists references of an address group.
    /// 
    /// # Arguments
    ///
    /// * `addressGroup` - Required. A name of the AddressGroup to clone items to. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_list_references(&self, address_group: &str) -> OrganizationLocationAddressGroupListReferenceCall<'a, S> {
        OrganizationLocationAddressGroupListReferenceCall {
            hub: self.hub,
            _address_group: address_group.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates parameters of an address group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`.
    pub fn locations_address_groups_patch(&self, request: AddressGroup, name: &str) -> OrganizationLocationAddressGroupPatchCall<'a, S> {
        OrganizationLocationAddressGroupPatchCall {
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
    /// Removes items from an address group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `addressGroup` - Required. A name of the AddressGroup to remove items from. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_remove_items(&self, request: RemoveAddressGroupItemsRequest, address_group: &str) -> OrganizationLocationAddressGroupRemoveItemCall<'a, S> {
        OrganizationLocationAddressGroupRemoveItemCall {
            hub: self.hub,
            _request: request,
            _address_group: address_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new FirewallEndpoint in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Value for parent.
    pub fn locations_firewall_endpoints_create(&self, request: FirewallEndpoint, parent: &str) -> OrganizationLocationFirewallEndpointCreateCall<'a, S> {
        OrganizationLocationFirewallEndpointCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _firewall_endpoint_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Endpoint.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource
    pub fn locations_firewall_endpoints_delete(&self, name: &str) -> OrganizationLocationFirewallEndpointDeleteCall<'a, S> {
        OrganizationLocationFirewallEndpointDeleteCall {
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
    /// Gets details of a single Endpoint.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource
    pub fn locations_firewall_endpoints_get(&self, name: &str) -> OrganizationLocationFirewallEndpointGetCall<'a, S> {
        OrganizationLocationFirewallEndpointGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists FirewallEndpoints in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListEndpointsRequest
    pub fn locations_firewall_endpoints_list(&self, parent: &str) -> OrganizationLocationFirewallEndpointListCall<'a, S> {
        OrganizationLocationFirewallEndpointListCall {
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
    /// Update a single Endpoint.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Identifier. name of resource
    pub fn locations_firewall_endpoints_patch(&self, request: FirewallEndpoint, name: &str) -> OrganizationLocationFirewallEndpointPatchCall<'a, S> {
        OrganizationLocationFirewallEndpointPatchCall {
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
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> OrganizationLocationOperationCancelCall<'a, S> {
        OrganizationLocationOperationCancelCall {
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
    pub fn locations_operations_delete(&self, name: &str) -> OrganizationLocationOperationDeleteCall<'a, S> {
        OrganizationLocationOperationDeleteCall {
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
    pub fn locations_operations_get(&self, name: &str) -> OrganizationLocationOperationGetCall<'a, S> {
        OrganizationLocationOperationGetCall {
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
    pub fn locations_operations_list(&self, name: &str) -> OrganizationLocationOperationListCall<'a, S> {
        OrganizationLocationOperationListCall {
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
    /// Creates a new SecurityProfileGroup in a given organization and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the SecurityProfileGroup. Must be in the format `projects|organizations/*/locations/{location}`.
    pub fn locations_security_profile_groups_create(&self, request: SecurityProfileGroup, parent: &str) -> OrganizationLocationSecurityProfileGroupCreateCall<'a, S> {
        OrganizationLocationSecurityProfileGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _security_profile_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single SecurityProfileGroup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the SecurityProfileGroup to delete. Must be in the format `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`.
    pub fn locations_security_profile_groups_delete(&self, name: &str) -> OrganizationLocationSecurityProfileGroupDeleteCall<'a, S> {
        OrganizationLocationSecurityProfileGroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single SecurityProfileGroup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the SecurityProfileGroup to get. Must be in the format `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`.
    pub fn locations_security_profile_groups_get(&self, name: &str) -> OrganizationLocationSecurityProfileGroupGetCall<'a, S> {
        OrganizationLocationSecurityProfileGroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists SecurityProfileGroups in a given organization and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project or organization and location from which the SecurityProfileGroups should be listed, specified in the format `projects|organizations/*/locations/{location}`.
    pub fn locations_security_profile_groups_list(&self, parent: &str) -> OrganizationLocationSecurityProfileGroupListCall<'a, S> {
        OrganizationLocationSecurityProfileGroupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single SecurityProfileGroup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Identifier. Name of the SecurityProfileGroup resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`.
    pub fn locations_security_profile_groups_patch(&self, request: SecurityProfileGroup, name: &str) -> OrganizationLocationSecurityProfileGroupPatchCall<'a, S> {
        OrganizationLocationSecurityProfileGroupPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new SecurityProfile in a given organization and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the SecurityProfile. Must be in the format `projects|organizations/*/locations/{location}`.
    pub fn locations_security_profiles_create(&self, request: SecurityProfile, parent: &str) -> OrganizationLocationSecurityProfileCreateCall<'a, S> {
        OrganizationLocationSecurityProfileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _security_profile_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single SecurityProfile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the SecurityProfile to delete. Must be in the format `projects|organizations/*/locations/{location}/securityProfiles/{security_profile_id}`.
    pub fn locations_security_profiles_delete(&self, name: &str) -> OrganizationLocationSecurityProfileDeleteCall<'a, S> {
        OrganizationLocationSecurityProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single SecurityProfile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the SecurityProfile to get. Must be in the format `projects|organizations/*/locations/{location}/securityProfiles/{security_profile_id}`.
    pub fn locations_security_profiles_get(&self, name: &str) -> OrganizationLocationSecurityProfileGetCall<'a, S> {
        OrganizationLocationSecurityProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists SecurityProfiles in a given organization and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project or organization and location from which the SecurityProfiles should be listed, specified in the format `projects|organizations/*/locations/{location}`.
    pub fn locations_security_profiles_list(&self, parent: &str) -> OrganizationLocationSecurityProfileListCall<'a, S> {
        OrganizationLocationSecurityProfileListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single SecurityProfile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Identifier. Name of the SecurityProfile resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfiles/{security_profile}`.
    pub fn locations_security_profiles_patch(&self, request: SecurityProfile, name: &str) -> OrganizationLocationSecurityProfilePatchCall<'a, S> {
        OrganizationLocationSecurityProfilePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`NetworkSecurity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_networksecurity1 as networksecurity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use networksecurity1::{NetworkSecurity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = NetworkSecurity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_address_groups_add_items(...)`, `locations_address_groups_clone_items(...)`, `locations_address_groups_create(...)`, `locations_address_groups_delete(...)`, `locations_address_groups_get(...)`, `locations_address_groups_get_iam_policy(...)`, `locations_address_groups_list(...)`, `locations_address_groups_list_references(...)`, `locations_address_groups_patch(...)`, `locations_address_groups_remove_items(...)`, `locations_address_groups_set_iam_policy(...)`, `locations_address_groups_test_iam_permissions(...)`, `locations_authorization_policies_create(...)`, `locations_authorization_policies_delete(...)`, `locations_authorization_policies_get(...)`, `locations_authorization_policies_get_iam_policy(...)`, `locations_authorization_policies_list(...)`, `locations_authorization_policies_patch(...)`, `locations_authorization_policies_set_iam_policy(...)`, `locations_authorization_policies_test_iam_permissions(...)`, `locations_client_tls_policies_create(...)`, `locations_client_tls_policies_delete(...)`, `locations_client_tls_policies_get(...)`, `locations_client_tls_policies_get_iam_policy(...)`, `locations_client_tls_policies_list(...)`, `locations_client_tls_policies_patch(...)`, `locations_client_tls_policies_set_iam_policy(...)`, `locations_client_tls_policies_test_iam_permissions(...)`, `locations_firewall_endpoint_associations_create(...)`, `locations_firewall_endpoint_associations_delete(...)`, `locations_firewall_endpoint_associations_get(...)`, `locations_firewall_endpoint_associations_list(...)`, `locations_firewall_endpoint_associations_patch(...)`, `locations_gateway_security_policies_create(...)`, `locations_gateway_security_policies_delete(...)`, `locations_gateway_security_policies_get(...)`, `locations_gateway_security_policies_list(...)`, `locations_gateway_security_policies_patch(...)`, `locations_gateway_security_policies_rules_create(...)`, `locations_gateway_security_policies_rules_delete(...)`, `locations_gateway_security_policies_rules_get(...)`, `locations_gateway_security_policies_rules_list(...)`, `locations_gateway_security_policies_rules_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_server_tls_policies_create(...)`, `locations_server_tls_policies_delete(...)`, `locations_server_tls_policies_get(...)`, `locations_server_tls_policies_get_iam_policy(...)`, `locations_server_tls_policies_list(...)`, `locations_server_tls_policies_patch(...)`, `locations_server_tls_policies_set_iam_policy(...)`, `locations_server_tls_policies_test_iam_permissions(...)`, `locations_tls_inspection_policies_create(...)`, `locations_tls_inspection_policies_delete(...)`, `locations_tls_inspection_policies_get(...)`, `locations_tls_inspection_policies_list(...)`, `locations_tls_inspection_policies_patch(...)`, `locations_url_lists_create(...)`, `locations_url_lists_delete(...)`, `locations_url_lists_get(...)`, `locations_url_lists_list(...)` and `locations_url_lists_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a NetworkSecurity<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds items to an address group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `addressGroup` - Required. A name of the AddressGroup to add items to. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_add_items(&self, request: AddAddressGroupItemsRequest, address_group: &str) -> ProjectLocationAddressGroupAddItemCall<'a, S> {
        ProjectLocationAddressGroupAddItemCall {
            hub: self.hub,
            _request: request,
            _address_group: address_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clones items from one address group to another.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `addressGroup` - Required. A name of the AddressGroup to clone items to. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_clone_items(&self, request: CloneAddressGroupItemsRequest, address_group: &str) -> ProjectLocationAddressGroupCloneItemCall<'a, S> {
        ProjectLocationAddressGroupCloneItemCall {
            hub: self.hub,
            _request: request,
            _address_group: address_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new address group in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the AddressGroup. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_address_groups_create(&self, request: AddressGroup, parent: &str) -> ProjectLocationAddressGroupCreateCall<'a, S> {
        ProjectLocationAddressGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _address_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single address group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AddressGroup to delete. Must be in the format `projects/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_delete(&self, name: &str) -> ProjectLocationAddressGroupDeleteCall<'a, S> {
        ProjectLocationAddressGroupDeleteCall {
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
    /// Gets details of a single address group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AddressGroup to get. Must be in the format `projects/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_get(&self, name: &str) -> ProjectLocationAddressGroupGetCall<'a, S> {
        ProjectLocationAddressGroupGetCall {
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
    pub fn locations_address_groups_get_iam_policy(&self, resource: &str) -> ProjectLocationAddressGroupGetIamPolicyCall<'a, S> {
        ProjectLocationAddressGroupGetIamPolicyCall {
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
    /// Lists address groups in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the AddressGroups should be listed, specified in the format `projects/*/locations/{location}`.
    pub fn locations_address_groups_list(&self, parent: &str) -> ProjectLocationAddressGroupListCall<'a, S> {
        ProjectLocationAddressGroupListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists references of an address group.
    /// 
    /// # Arguments
    ///
    /// * `addressGroup` - Required. A name of the AddressGroup to clone items to. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_list_references(&self, address_group: &str) -> ProjectLocationAddressGroupListReferenceCall<'a, S> {
        ProjectLocationAddressGroupListReferenceCall {
            hub: self.hub,
            _address_group: address_group.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single address group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`.
    pub fn locations_address_groups_patch(&self, request: AddressGroup, name: &str) -> ProjectLocationAddressGroupPatchCall<'a, S> {
        ProjectLocationAddressGroupPatchCall {
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
    /// Removes items from an address group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `addressGroup` - Required. A name of the AddressGroup to remove items from. Must be in the format `projects|organization/*/locations/{location}/addressGroups/*`.
    pub fn locations_address_groups_remove_items(&self, request: RemoveAddressGroupItemsRequest, address_group: &str) -> ProjectLocationAddressGroupRemoveItemCall<'a, S> {
        ProjectLocationAddressGroupRemoveItemCall {
            hub: self.hub,
            _request: request,
            _address_group: address_group.to_string(),
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
    pub fn locations_address_groups_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationAddressGroupSetIamPolicyCall<'a, S> {
        ProjectLocationAddressGroupSetIamPolicyCall {
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
    pub fn locations_address_groups_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationAddressGroupTestIamPermissionCall<'a, S> {
        ProjectLocationAddressGroupTestIamPermissionCall {
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
    /// Creates a new AuthorizationPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the AuthorizationPolicy. Must be in the format `projects/{project}/locations/{location}`.
    pub fn locations_authorization_policies_create(&self, request: AuthorizationPolicy, parent: &str) -> ProjectLocationAuthorizationPolicyCreateCall<'a, S> {
        ProjectLocationAuthorizationPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _authorization_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single AuthorizationPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AuthorizationPolicy to delete. Must be in the format `projects/{project}/locations/{location}/authorizationPolicies/*`.
    pub fn locations_authorization_policies_delete(&self, name: &str) -> ProjectLocationAuthorizationPolicyDeleteCall<'a, S> {
        ProjectLocationAuthorizationPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single AuthorizationPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the AuthorizationPolicy to get. Must be in the format `projects/{project}/locations/{location}/authorizationPolicies/*`.
    pub fn locations_authorization_policies_get(&self, name: &str) -> ProjectLocationAuthorizationPolicyGetCall<'a, S> {
        ProjectLocationAuthorizationPolicyGetCall {
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
    pub fn locations_authorization_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationAuthorizationPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationAuthorizationPolicyGetIamPolicyCall {
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
    /// Lists AuthorizationPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the AuthorizationPolicies should be listed, specified in the format `projects/{project}/locations/{location}`.
    pub fn locations_authorization_policies_list(&self, parent: &str) -> ProjectLocationAuthorizationPolicyListCall<'a, S> {
        ProjectLocationAuthorizationPolicyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single AuthorizationPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`.
    pub fn locations_authorization_policies_patch(&self, request: AuthorizationPolicy, name: &str) -> ProjectLocationAuthorizationPolicyPatchCall<'a, S> {
        ProjectLocationAuthorizationPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
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
    pub fn locations_authorization_policies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationAuthorizationPolicySetIamPolicyCall<'a, S> {
        ProjectLocationAuthorizationPolicySetIamPolicyCall {
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
    pub fn locations_authorization_policies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationAuthorizationPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationAuthorizationPolicyTestIamPermissionCall {
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
    /// Creates a new ClientTlsPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the ClientTlsPolicy. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_client_tls_policies_create(&self, request: ClientTlsPolicy, parent: &str) -> ProjectLocationClientTlsPolicyCreateCall<'a, S> {
        ProjectLocationClientTlsPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _client_tls_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ClientTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ClientTlsPolicy to delete. Must be in the format `projects/*/locations/{location}/clientTlsPolicies/*`.
    pub fn locations_client_tls_policies_delete(&self, name: &str) -> ProjectLocationClientTlsPolicyDeleteCall<'a, S> {
        ProjectLocationClientTlsPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ClientTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ClientTlsPolicy to get. Must be in the format `projects/*/locations/{location}/clientTlsPolicies/*`.
    pub fn locations_client_tls_policies_get(&self, name: &str) -> ProjectLocationClientTlsPolicyGetCall<'a, S> {
        ProjectLocationClientTlsPolicyGetCall {
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
    pub fn locations_client_tls_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationClientTlsPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationClientTlsPolicyGetIamPolicyCall {
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
    /// Lists ClientTlsPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the ClientTlsPolicies should be listed, specified in the format `projects/*/locations/{location}`.
    pub fn locations_client_tls_policies_list(&self, parent: &str) -> ProjectLocationClientTlsPolicyListCall<'a, S> {
        ProjectLocationClientTlsPolicyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single ClientTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/clientTlsPolicies/{client_tls_policy}`
    pub fn locations_client_tls_policies_patch(&self, request: ClientTlsPolicy, name: &str) -> ProjectLocationClientTlsPolicyPatchCall<'a, S> {
        ProjectLocationClientTlsPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
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
    pub fn locations_client_tls_policies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationClientTlsPolicySetIamPolicyCall<'a, S> {
        ProjectLocationClientTlsPolicySetIamPolicyCall {
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
    pub fn locations_client_tls_policies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationClientTlsPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationClientTlsPolicyTestIamPermissionCall {
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
    /// Creates a new FirewallEndpointAssociation in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Value for parent.
    pub fn locations_firewall_endpoint_associations_create(&self, request: FirewallEndpointAssociation, parent: &str) -> ProjectLocationFirewallEndpointAssociationCreateCall<'a, S> {
        ProjectLocationFirewallEndpointAssociationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _firewall_endpoint_association_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single FirewallEndpointAssociation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource
    pub fn locations_firewall_endpoint_associations_delete(&self, name: &str) -> ProjectLocationFirewallEndpointAssociationDeleteCall<'a, S> {
        ProjectLocationFirewallEndpointAssociationDeleteCall {
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
    /// Gets details of a single FirewallEndpointAssociation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource
    pub fn locations_firewall_endpoint_associations_get(&self, name: &str) -> ProjectLocationFirewallEndpointAssociationGetCall<'a, S> {
        ProjectLocationFirewallEndpointAssociationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Associations in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent value for ListAssociationsRequest
    pub fn locations_firewall_endpoint_associations_list(&self, parent: &str) -> ProjectLocationFirewallEndpointAssociationListCall<'a, S> {
        ProjectLocationFirewallEndpointAssociationListCall {
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
    /// Update a single FirewallEndpointAssociation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Identifier. name of resource
    pub fn locations_firewall_endpoint_associations_patch(&self, request: FirewallEndpointAssociation, name: &str) -> ProjectLocationFirewallEndpointAssociationPatchCall<'a, S> {
        ProjectLocationFirewallEndpointAssociationPatchCall {
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
    /// Creates a new GatewaySecurityPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent where this rule will be created. Format : projects/{project}/location/{location}/gatewaySecurityPolicies/*
    pub fn locations_gateway_security_policies_rules_create(&self, request: GatewaySecurityPolicyRule, parent: &str) -> ProjectLocationGatewaySecurityPolicyRuleCreateCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyRuleCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _gateway_security_policy_rule_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single GatewaySecurityPolicyRule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the GatewaySecurityPolicyRule to delete. Must be in the format `projects/{project}/locations/{location}/gatewaySecurityPolicies/{gatewaySecurityPolicy}/rules/*`.
    pub fn locations_gateway_security_policies_rules_delete(&self, name: &str) -> ProjectLocationGatewaySecurityPolicyRuleDeleteCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyRuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single GatewaySecurityPolicyRule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the GatewaySecurityPolicyRule to retrieve. Format: projects/{project}/location/{location}/gatewaySecurityPolicies/*/rules/*
    pub fn locations_gateway_security_policies_rules_get(&self, name: &str) -> ProjectLocationGatewaySecurityPolicyRuleGetCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyRuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists GatewaySecurityPolicyRules in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project, location and GatewaySecurityPolicy from which the GatewaySecurityPolicyRules should be listed, specified in the format `projects/{project}/locations/{location}/gatewaySecurityPolicies/{gatewaySecurityPolicy}`.
    pub fn locations_gateway_security_policies_rules_list(&self, parent: &str) -> ProjectLocationGatewaySecurityPolicyRuleListCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyRuleListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single GatewaySecurityPolicyRule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Immutable. Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule} rule should match the pattern: (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    pub fn locations_gateway_security_policies_rules_patch(&self, request: GatewaySecurityPolicyRule, name: &str) -> ProjectLocationGatewaySecurityPolicyRulePatchCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyRulePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new GatewaySecurityPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the GatewaySecurityPolicy. Must be in the format `projects/{project}/locations/{location}`.
    pub fn locations_gateway_security_policies_create(&self, request: GatewaySecurityPolicy, parent: &str) -> ProjectLocationGatewaySecurityPolicyCreateCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _gateway_security_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single GatewaySecurityPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the GatewaySecurityPolicy to delete. Must be in the format `projects/{project}/locations/{location}/gatewaySecurityPolicies/*`.
    pub fn locations_gateway_security_policies_delete(&self, name: &str) -> ProjectLocationGatewaySecurityPolicyDeleteCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single GatewaySecurityPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the GatewaySecurityPolicy to get. Must be in the format `projects/{project}/locations/{location}/gatewaySecurityPolicies/*`.
    pub fn locations_gateway_security_policies_get(&self, name: &str) -> ProjectLocationGatewaySecurityPolicyGetCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists GatewaySecurityPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the GatewaySecurityPolicies should be listed, specified in the format `projects/{project}/locations/{location}`.
    pub fn locations_gateway_security_policies_list(&self, parent: &str) -> ProjectLocationGatewaySecurityPolicyListCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single GatewaySecurityPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy} gateway_security_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    pub fn locations_gateway_security_policies_patch(&self, request: GatewaySecurityPolicy, name: &str) -> ProjectLocationGatewaySecurityPolicyPatchCall<'a, S> {
        ProjectLocationGatewaySecurityPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
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
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
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
    /// Creates a new ServerTlsPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the ServerTlsPolicy. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_server_tls_policies_create(&self, request: ServerTlsPolicy, parent: &str) -> ProjectLocationServerTlsPolicyCreateCall<'a, S> {
        ProjectLocationServerTlsPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _server_tls_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single ServerTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ServerTlsPolicy to delete. Must be in the format `projects/*/locations/{location}/serverTlsPolicies/*`.
    pub fn locations_server_tls_policies_delete(&self, name: &str) -> ProjectLocationServerTlsPolicyDeleteCall<'a, S> {
        ProjectLocationServerTlsPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single ServerTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the ServerTlsPolicy to get. Must be in the format `projects/*/locations/{location}/serverTlsPolicies/*`.
    pub fn locations_server_tls_policies_get(&self, name: &str) -> ProjectLocationServerTlsPolicyGetCall<'a, S> {
        ProjectLocationServerTlsPolicyGetCall {
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
    pub fn locations_server_tls_policies_get_iam_policy(&self, resource: &str) -> ProjectLocationServerTlsPolicyGetIamPolicyCall<'a, S> {
        ProjectLocationServerTlsPolicyGetIamPolicyCall {
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
    /// Lists ServerTlsPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the ServerTlsPolicies should be listed, specified in the format `projects/*/locations/{location}`.
    pub fn locations_server_tls_policies_list(&self, parent: &str) -> ProjectLocationServerTlsPolicyListCall<'a, S> {
        ProjectLocationServerTlsPolicyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single ServerTlsPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}`
    pub fn locations_server_tls_policies_patch(&self, request: ServerTlsPolicy, name: &str) -> ProjectLocationServerTlsPolicyPatchCall<'a, S> {
        ProjectLocationServerTlsPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
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
    pub fn locations_server_tls_policies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationServerTlsPolicySetIamPolicyCall<'a, S> {
        ProjectLocationServerTlsPolicySetIamPolicyCall {
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
    pub fn locations_server_tls_policies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationServerTlsPolicyTestIamPermissionCall<'a, S> {
        ProjectLocationServerTlsPolicyTestIamPermissionCall {
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
    /// Creates a new TlsInspectionPolicy in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the TlsInspectionPolicy. Must be in the format `projects/{project}/locations/{location}`.
    pub fn locations_tls_inspection_policies_create(&self, request: TlsInspectionPolicy, parent: &str) -> ProjectLocationTlsInspectionPolicyCreateCall<'a, S> {
        ProjectLocationTlsInspectionPolicyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _tls_inspection_policy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single TlsInspectionPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the TlsInspectionPolicy to delete. Must be in the format `projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy}`.
    pub fn locations_tls_inspection_policies_delete(&self, name: &str) -> ProjectLocationTlsInspectionPolicyDeleteCall<'a, S> {
        ProjectLocationTlsInspectionPolicyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single TlsInspectionPolicy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the TlsInspectionPolicy to get. Must be in the format `projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy}`.
    pub fn locations_tls_inspection_policies_get(&self, name: &str) -> ProjectLocationTlsInspectionPolicyGetCall<'a, S> {
        ProjectLocationTlsInspectionPolicyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists TlsInspectionPolicies in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the TlsInspectionPolicies should be listed, specified in the format `projects/{project}/locations/{location}`.
    pub fn locations_tls_inspection_policies_list(&self, parent: &str) -> ProjectLocationTlsInspectionPolicyListCall<'a, S> {
        ProjectLocationTlsInspectionPolicyListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single TlsInspectionPolicy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy} tls_inspection_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    pub fn locations_tls_inspection_policies_patch(&self, request: TlsInspectionPolicy, name: &str) -> ProjectLocationTlsInspectionPolicyPatchCall<'a, S> {
        ProjectLocationTlsInspectionPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new UrlList in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the UrlList. Must be in the format `projects/*/locations/{location}`.
    pub fn locations_url_lists_create(&self, request: UrlList, parent: &str) -> ProjectLocationUrlListCreateCall<'a, S> {
        ProjectLocationUrlListCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _url_list_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single UrlList.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the UrlList to delete. Must be in the format `projects/*/locations/{location}/urlLists/*`.
    pub fn locations_url_lists_delete(&self, name: &str) -> ProjectLocationUrlListDeleteCall<'a, S> {
        ProjectLocationUrlListDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single UrlList.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the UrlList to get. Must be in the format `projects/*/locations/{location}/urlLists/*`.
    pub fn locations_url_lists_get(&self, name: &str) -> ProjectLocationUrlListGetCall<'a, S> {
        ProjectLocationUrlListGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists UrlLists in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the UrlLists should be listed, specified in the format `projects/{project}/locations/{location}`.
    pub fn locations_url_lists_list(&self, parent: &str) -> ProjectLocationUrlListListCall<'a, S> {
        ProjectLocationUrlListListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the parameters of a single UrlList.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the resource provided by the user. Name is of the form projects/{project}/locations/{location}/urlLists/{url_list} url_list should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    pub fn locations_url_lists_patch(&self, request: UrlList, name: &str) -> ProjectLocationUrlListPatchCall<'a, S> {
        ProjectLocationUrlListPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
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



