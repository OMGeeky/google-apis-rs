use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudDataplex`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dataplex1 as dataplex1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dataplex1::{CloudDataplex, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudDataplex::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_aspect_types_create(...)`, `locations_aspect_types_delete(...)`, `locations_aspect_types_get(...)`, `locations_aspect_types_get_iam_policy(...)`, `locations_aspect_types_list(...)`, `locations_aspect_types_patch(...)`, `locations_aspect_types_set_iam_policy(...)`, `locations_aspect_types_test_iam_permissions(...)`, `locations_data_attribute_bindings_create(...)`, `locations_data_attribute_bindings_delete(...)`, `locations_data_attribute_bindings_get(...)`, `locations_data_attribute_bindings_get_iam_policy(...)`, `locations_data_attribute_bindings_list(...)`, `locations_data_attribute_bindings_patch(...)`, `locations_data_attribute_bindings_set_iam_policy(...)`, `locations_data_attribute_bindings_test_iam_permissions(...)`, `locations_data_scans_create(...)`, `locations_data_scans_delete(...)`, `locations_data_scans_generate_data_quality_rules(...)`, `locations_data_scans_get(...)`, `locations_data_scans_get_iam_policy(...)`, `locations_data_scans_jobs_generate_data_quality_rules(...)`, `locations_data_scans_jobs_get(...)`, `locations_data_scans_jobs_list(...)`, `locations_data_scans_list(...)`, `locations_data_scans_patch(...)`, `locations_data_scans_run(...)`, `locations_data_scans_set_iam_policy(...)`, `locations_data_scans_test_iam_permissions(...)`, `locations_data_taxonomies_attributes_create(...)`, `locations_data_taxonomies_attributes_delete(...)`, `locations_data_taxonomies_attributes_get(...)`, `locations_data_taxonomies_attributes_get_iam_policy(...)`, `locations_data_taxonomies_attributes_list(...)`, `locations_data_taxonomies_attributes_patch(...)`, `locations_data_taxonomies_attributes_set_iam_policy(...)`, `locations_data_taxonomies_attributes_test_iam_permissions(...)`, `locations_data_taxonomies_create(...)`, `locations_data_taxonomies_delete(...)`, `locations_data_taxonomies_get(...)`, `locations_data_taxonomies_get_iam_policy(...)`, `locations_data_taxonomies_list(...)`, `locations_data_taxonomies_patch(...)`, `locations_data_taxonomies_set_iam_policy(...)`, `locations_data_taxonomies_test_iam_permissions(...)`, `locations_entry_groups_create(...)`, `locations_entry_groups_delete(...)`, `locations_entry_groups_entries_create(...)`, `locations_entry_groups_entries_delete(...)`, `locations_entry_groups_entries_get(...)`, `locations_entry_groups_entries_list(...)`, `locations_entry_groups_entries_patch(...)`, `locations_entry_groups_get(...)`, `locations_entry_groups_get_iam_policy(...)`, `locations_entry_groups_list(...)`, `locations_entry_groups_patch(...)`, `locations_entry_groups_set_iam_policy(...)`, `locations_entry_groups_test_iam_permissions(...)`, `locations_entry_types_create(...)`, `locations_entry_types_delete(...)`, `locations_entry_types_get(...)`, `locations_entry_types_get_iam_policy(...)`, `locations_entry_types_list(...)`, `locations_entry_types_patch(...)`, `locations_entry_types_set_iam_policy(...)`, `locations_entry_types_test_iam_permissions(...)`, `locations_get(...)`, `locations_governance_rules_get_iam_policy(...)`, `locations_governance_rules_set_iam_policy(...)`, `locations_governance_rules_test_iam_permissions(...)`, `locations_lakes_actions_list(...)`, `locations_lakes_content_create(...)`, `locations_lakes_content_delete(...)`, `locations_lakes_content_get(...)`, `locations_lakes_content_get_iam_policy(...)`, `locations_lakes_content_list(...)`, `locations_lakes_content_patch(...)`, `locations_lakes_content_set_iam_policy(...)`, `locations_lakes_content_test_iam_permissions(...)`, `locations_lakes_contentitems_create(...)`, `locations_lakes_contentitems_delete(...)`, `locations_lakes_contentitems_get(...)`, `locations_lakes_contentitems_get_iam_policy(...)`, `locations_lakes_contentitems_list(...)`, `locations_lakes_contentitems_patch(...)`, `locations_lakes_contentitems_set_iam_policy(...)`, `locations_lakes_contentitems_test_iam_permissions(...)`, `locations_lakes_create(...)`, `locations_lakes_delete(...)`, `locations_lakes_environments_create(...)`, `locations_lakes_environments_delete(...)`, `locations_lakes_environments_get(...)`, `locations_lakes_environments_get_iam_policy(...)`, `locations_lakes_environments_list(...)`, `locations_lakes_environments_patch(...)`, `locations_lakes_environments_sessions_list(...)`, `locations_lakes_environments_set_iam_policy(...)`, `locations_lakes_environments_test_iam_permissions(...)`, `locations_lakes_get(...)`, `locations_lakes_get_iam_policy(...)`, `locations_lakes_list(...)`, `locations_lakes_patch(...)`, `locations_lakes_set_iam_policy(...)`, `locations_lakes_tasks_create(...)`, `locations_lakes_tasks_delete(...)`, `locations_lakes_tasks_get(...)`, `locations_lakes_tasks_get_iam_policy(...)`, `locations_lakes_tasks_jobs_cancel(...)`, `locations_lakes_tasks_jobs_get(...)`, `locations_lakes_tasks_jobs_list(...)`, `locations_lakes_tasks_list(...)`, `locations_lakes_tasks_patch(...)`, `locations_lakes_tasks_run(...)`, `locations_lakes_tasks_set_iam_policy(...)`, `locations_lakes_tasks_test_iam_permissions(...)`, `locations_lakes_test_iam_permissions(...)`, `locations_lakes_zones_actions_list(...)`, `locations_lakes_zones_assets_actions_list(...)`, `locations_lakes_zones_assets_create(...)`, `locations_lakes_zones_assets_delete(...)`, `locations_lakes_zones_assets_get(...)`, `locations_lakes_zones_assets_get_iam_policy(...)`, `locations_lakes_zones_assets_list(...)`, `locations_lakes_zones_assets_patch(...)`, `locations_lakes_zones_assets_set_iam_policy(...)`, `locations_lakes_zones_assets_test_iam_permissions(...)`, `locations_lakes_zones_create(...)`, `locations_lakes_zones_delete(...)`, `locations_lakes_zones_entities_create(...)`, `locations_lakes_zones_entities_delete(...)`, `locations_lakes_zones_entities_get(...)`, `locations_lakes_zones_entities_list(...)`, `locations_lakes_zones_entities_partitions_create(...)`, `locations_lakes_zones_entities_partitions_delete(...)`, `locations_lakes_zones_entities_partitions_get(...)`, `locations_lakes_zones_entities_partitions_list(...)`, `locations_lakes_zones_entities_update(...)`, `locations_lakes_zones_get(...)`, `locations_lakes_zones_get_iam_policy(...)`, `locations_lakes_zones_list(...)`, `locations_lakes_zones_patch(...)`, `locations_lakes_zones_set_iam_policy(...)`, `locations_lakes_zones_test_iam_permissions(...)`, `locations_list(...)`, `locations_lookup_entry(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)` and `locations_search_entries(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudDataplex<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an AspectType
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_aspect_types_create(&self, request: GoogleCloudDataplexV1AspectType, parent: &str) -> ProjectLocationAspectTypeCreateCall<'a, S> {
        ProjectLocationAspectTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _aspect_type_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a AspectType resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the AspectType: projects/{project_number}/locations/{location_id}/aspectTypes/{aspect_type_id}.
    pub fn locations_aspect_types_delete(&self, name: &str) -> ProjectLocationAspectTypeDeleteCall<'a, S> {
        ProjectLocationAspectTypeDeleteCall {
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
    /// Retrieves a AspectType resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the AspectType: projects/{project_number}/locations/{location_id}/aspectTypes/{aspect_type_id}.
    pub fn locations_aspect_types_get(&self, name: &str) -> ProjectLocationAspectTypeGetCall<'a, S> {
        ProjectLocationAspectTypeGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_aspect_types_get_iam_policy(&self, resource: &str) -> ProjectLocationAspectTypeGetIamPolicyCall<'a, S> {
        ProjectLocationAspectTypeGetIamPolicyCall {
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
    /// Lists AspectType resources in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the AspectType location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_aspect_types_list(&self, parent: &str) -> ProjectLocationAspectTypeListCall<'a, S> {
        ProjectLocationAspectTypeListCall {
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
    /// Updates a AspectType resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the AspectType, of the form: projects/{project_number}/locations/{location_id}/aspectTypes/{aspect_type_id}.
    pub fn locations_aspect_types_patch(&self, request: GoogleCloudDataplexV1AspectType, name: &str) -> ProjectLocationAspectTypePatchCall<'a, S> {
        ProjectLocationAspectTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_aspect_types_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationAspectTypeSetIamPolicyCall<'a, S> {
        ProjectLocationAspectTypeSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_aspect_types_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationAspectTypeTestIamPermissionCall<'a, S> {
        ProjectLocationAspectTypeTestIamPermissionCall {
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
    /// Create a DataAttributeBinding resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent data taxonomy projects/{project_number}/locations/{location_id}
    pub fn locations_data_attribute_bindings_create(&self, request: GoogleCloudDataplexV1DataAttributeBinding, parent: &str) -> ProjectLocationDataAttributeBindingCreateCall<'a, S> {
        ProjectLocationDataAttributeBindingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _data_attribute_binding_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a DataAttributeBinding resource. All attributes within the DataAttributeBinding must be deleted before the DataAttributeBinding can be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the DataAttributeBinding: projects/{project_number}/locations/{location_id}/dataAttributeBindings/{data_attribute_binding_id}
    pub fn locations_data_attribute_bindings_delete(&self, name: &str) -> ProjectLocationDataAttributeBindingDeleteCall<'a, S> {
        ProjectLocationDataAttributeBindingDeleteCall {
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
    /// Retrieves a DataAttributeBinding resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the DataAttributeBinding: projects/{project_number}/locations/{location_id}/dataAttributeBindings/{data_attribute_binding_id}
    pub fn locations_data_attribute_bindings_get(&self, name: &str) -> ProjectLocationDataAttributeBindingGetCall<'a, S> {
        ProjectLocationDataAttributeBindingGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_attribute_bindings_get_iam_policy(&self, resource: &str) -> ProjectLocationDataAttributeBindingGetIamPolicyCall<'a, S> {
        ProjectLocationDataAttributeBindingGetIamPolicyCall {
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
    /// Lists DataAttributeBinding resources in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the Location: projects/{project_number}/locations/{location_id}
    pub fn locations_data_attribute_bindings_list(&self, parent: &str) -> ProjectLocationDataAttributeBindingListCall<'a, S> {
        ProjectLocationDataAttributeBindingListCall {
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
    /// Updates a DataAttributeBinding resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the Data Attribute Binding, of the form: projects/{project_number}/locations/{location}/dataAttributeBindings/{data_attribute_binding_id}
    pub fn locations_data_attribute_bindings_patch(&self, request: GoogleCloudDataplexV1DataAttributeBinding, name: &str) -> ProjectLocationDataAttributeBindingPatchCall<'a, S> {
        ProjectLocationDataAttributeBindingPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_attribute_bindings_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationDataAttributeBindingSetIamPolicyCall<'a, S> {
        ProjectLocationDataAttributeBindingSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_attribute_bindings_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationDataAttributeBindingTestIamPermissionCall<'a, S> {
        ProjectLocationDataAttributeBindingTestIamPermissionCall {
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
    /// Generates recommended DataQualityRule from a data profiling DataScan.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name should be either * the name of a datascan with at least one successful completed data profiling job, or * the name of a successful completed data profiling datascan job.
    pub fn locations_data_scans_jobs_generate_data_quality_rules(&self, request: GoogleCloudDataplexV1GenerateDataQualityRulesRequest, name: &str) -> ProjectLocationDataScanJobGenerateDataQualityRuleCall<'a, S> {
        ProjectLocationDataScanJobGenerateDataQualityRuleCall {
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
    /// Gets a DataScanJob resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the DataScanJob: projects/{project}/locations/{location_id}/dataScans/{data_scan_id}/jobs/{data_scan_job_id} where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_jobs_get(&self, name: &str) -> ProjectLocationDataScanJobGetCall<'a, S> {
        ProjectLocationDataScanJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DataScanJobs under the given DataScan.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent environment: projects/{project}/locations/{location_id}/dataScans/{data_scan_id} where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_jobs_list(&self, parent: &str) -> ProjectLocationDataScanJobListCall<'a, S> {
        ProjectLocationDataScanJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Creates a DataScan resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent location: projects/{project}/locations/{location_id} where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_create(&self, request: GoogleCloudDataplexV1DataScan, parent: &str) -> ProjectLocationDataScanCreateCall<'a, S> {
        ProjectLocationDataScanCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _data_scan_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a DataScan resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the dataScan: projects/{project}/locations/{location_id}/dataScans/{data_scan_id} where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_delete(&self, name: &str) -> ProjectLocationDataScanDeleteCall<'a, S> {
        ProjectLocationDataScanDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates recommended DataQualityRule from a data profiling DataScan.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name should be either * the name of a datascan with at least one successful completed data profiling job, or * the name of a successful completed data profiling datascan job.
    pub fn locations_data_scans_generate_data_quality_rules(&self, request: GoogleCloudDataplexV1GenerateDataQualityRulesRequest, name: &str) -> ProjectLocationDataScanGenerateDataQualityRuleCall<'a, S> {
        ProjectLocationDataScanGenerateDataQualityRuleCall {
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
    /// Gets a DataScan resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the dataScan: projects/{project}/locations/{location_id}/dataScans/{data_scan_id} where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_get(&self, name: &str) -> ProjectLocationDataScanGetCall<'a, S> {
        ProjectLocationDataScanGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_scans_get_iam_policy(&self, resource: &str) -> ProjectLocationDataScanGetIamPolicyCall<'a, S> {
        ProjectLocationDataScanGetIamPolicyCall {
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
    /// Lists DataScans.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent location: projects/{project}/locations/{location_id} where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_list(&self, parent: &str) -> ProjectLocationDataScanListCall<'a, S> {
        ProjectLocationDataScanListCall {
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
    /// Updates a DataScan resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the scan, of the form: projects/{project}/locations/{location_id}/dataScans/{datascan_id}, where project refers to a project_id or project_number and location_id refers to a GCP region.
    pub fn locations_data_scans_patch(&self, request: GoogleCloudDataplexV1DataScan, name: &str) -> ProjectLocationDataScanPatchCall<'a, S> {
        ProjectLocationDataScanPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs an on-demand execution of a DataScan
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the DataScan: projects/{project}/locations/{location_id}/dataScans/{data_scan_id}. where project refers to a project_id or project_number and location_id refers to a GCP region.Only OnDemand data scans are allowed.
    pub fn locations_data_scans_run(&self, request: GoogleCloudDataplexV1RunDataScanRequest, name: &str) -> ProjectLocationDataScanRunCall<'a, S> {
        ProjectLocationDataScanRunCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_scans_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationDataScanSetIamPolicyCall<'a, S> {
        ProjectLocationDataScanSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_scans_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationDataScanTestIamPermissionCall<'a, S> {
        ProjectLocationDataScanTestIamPermissionCall {
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
    /// Create a DataAttribute resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent data taxonomy projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}
    pub fn locations_data_taxonomies_attributes_create(&self, request: GoogleCloudDataplexV1DataAttribute, parent: &str) -> ProjectLocationDataTaxonomyAttributeCreateCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _data_attribute_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Data Attribute resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the DataAttribute: projects/{project_number}/locations/{location_id}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}
    pub fn locations_data_taxonomies_attributes_delete(&self, name: &str) -> ProjectLocationDataTaxonomyAttributeDeleteCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeDeleteCall {
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
    /// Retrieves a Data Attribute resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the dataAttribute: projects/{project_number}/locations/{location_id}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}
    pub fn locations_data_taxonomies_attributes_get(&self, name: &str) -> ProjectLocationDataTaxonomyAttributeGetCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_taxonomies_attributes_get_iam_policy(&self, resource: &str) -> ProjectLocationDataTaxonomyAttributeGetIamPolicyCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeGetIamPolicyCall {
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
    /// Lists Data Attribute resources in a DataTaxonomy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the DataTaxonomy: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}
    pub fn locations_data_taxonomies_attributes_list(&self, parent: &str) -> ProjectLocationDataTaxonomyAttributeListCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeListCall {
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
    /// Updates a DataAttribute resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the dataAttribute, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{dataTaxonomy}/attributes/{data_attribute_id}.
    pub fn locations_data_taxonomies_attributes_patch(&self, request: GoogleCloudDataplexV1DataAttribute, name: &str) -> ProjectLocationDataTaxonomyAttributePatchCall<'a, S> {
        ProjectLocationDataTaxonomyAttributePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_taxonomies_attributes_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationDataTaxonomyAttributeSetIamPolicyCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_taxonomies_attributes_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationDataTaxonomyAttributeTestIamPermissionCall<'a, S> {
        ProjectLocationDataTaxonomyAttributeTestIamPermissionCall {
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
    /// Create a DataTaxonomy resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the data taxonomy location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_data_taxonomies_create(&self, request: GoogleCloudDataplexV1DataTaxonomy, parent: &str) -> ProjectLocationDataTaxonomyCreateCall<'a, S> {
        ProjectLocationDataTaxonomyCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _data_taxonomy_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a DataTaxonomy resource. All attributes within the DataTaxonomy must be deleted before the DataTaxonomy can be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the DataTaxonomy: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}
    pub fn locations_data_taxonomies_delete(&self, name: &str) -> ProjectLocationDataTaxonomyDeleteCall<'a, S> {
        ProjectLocationDataTaxonomyDeleteCall {
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
    /// Retrieves a DataTaxonomy resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the DataTaxonomy: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}
    pub fn locations_data_taxonomies_get(&self, name: &str) -> ProjectLocationDataTaxonomyGetCall<'a, S> {
        ProjectLocationDataTaxonomyGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_taxonomies_get_iam_policy(&self, resource: &str) -> ProjectLocationDataTaxonomyGetIamPolicyCall<'a, S> {
        ProjectLocationDataTaxonomyGetIamPolicyCall {
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
    /// Lists DataTaxonomy resources in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the DataTaxonomy location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_data_taxonomies_list(&self, parent: &str) -> ProjectLocationDataTaxonomyListCall<'a, S> {
        ProjectLocationDataTaxonomyListCall {
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
    /// Updates a DataTaxonomy resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the DataTaxonomy, of the form: projects/{project_number}/locations/{location_id}/dataTaxonomies/{data_taxonomy_id}.
    pub fn locations_data_taxonomies_patch(&self, request: GoogleCloudDataplexV1DataTaxonomy, name: &str) -> ProjectLocationDataTaxonomyPatchCall<'a, S> {
        ProjectLocationDataTaxonomyPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_taxonomies_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationDataTaxonomySetIamPolicyCall<'a, S> {
        ProjectLocationDataTaxonomySetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_data_taxonomies_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationDataTaxonomyTestIamPermissionCall<'a, S> {
        ProjectLocationDataTaxonomyTestIamPermissionCall {
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
    /// Creates an Entry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent Entry Group: projects/{project}/locations/{location}/entryGroups/{entry_group}.
    pub fn locations_entry_groups_entries_create(&self, request: GoogleCloudDataplexV1Entry, parent: &str) -> ProjectLocationEntryGroupEntryCreateCall<'a, S> {
        ProjectLocationEntryGroupEntryCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _entry_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an Entry.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Entry: projects/{project}/locations/{location}/entryGroups/{entry_group}/entries/{entry}.
    pub fn locations_entry_groups_entries_delete(&self, name: &str) -> ProjectLocationEntryGroupEntryDeleteCall<'a, S> {
        ProjectLocationEntryGroupEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single entry.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Entry: projects/{project}/locations/{location}/entryGroups/{entry_group}/entries/{entry}.
    pub fn locations_entry_groups_entries_get(&self, name: &str) -> ProjectLocationEntryGroupEntryGetCall<'a, S> {
        ProjectLocationEntryGroupEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _paths: Default::default(),
            _aspect_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists entries within an entry group.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent Entry Group: projects/{project}/locations/{location}/entryGroups/{entry_group}.
    pub fn locations_entry_groups_entries_list(&self, parent: &str) -> ProjectLocationEntryGroupEntryListCall<'a, S> {
        ProjectLocationEntryGroupEntryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Updates an Entry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The relative resource name of the Entry, of the form: projects/{project}/locations/{location}/entryGroups/{entry_group}/entries/{entry}.
    pub fn locations_entry_groups_entries_patch(&self, request: GoogleCloudDataplexV1Entry, name: &str) -> ProjectLocationEntryGroupEntryPatchCall<'a, S> {
        ProjectLocationEntryGroupEntryPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delete_missing_aspects: Default::default(),
            _aspect_keys: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an EntryGroup
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the entryGroup, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_entry_groups_create(&self, request: GoogleCloudDataplexV1EntryGroup, parent: &str) -> ProjectLocationEntryGroupCreateCall<'a, S> {
        ProjectLocationEntryGroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _entry_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a EntryGroup resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the EntryGroup: projects/{project_number}/locations/{location_id}/entryGroups/{entry_group_id}.
    pub fn locations_entry_groups_delete(&self, name: &str) -> ProjectLocationEntryGroupDeleteCall<'a, S> {
        ProjectLocationEntryGroupDeleteCall {
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
    /// Retrieves a EntryGroup resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the EntryGroup: projects/{project_number}/locations/{location_id}/entryGroups/{entry_group_id}.
    pub fn locations_entry_groups_get(&self, name: &str) -> ProjectLocationEntryGroupGetCall<'a, S> {
        ProjectLocationEntryGroupGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_get_iam_policy(&self, resource: &str) -> ProjectLocationEntryGroupGetIamPolicyCall<'a, S> {
        ProjectLocationEntryGroupGetIamPolicyCall {
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
    /// Lists EntryGroup resources in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the entryGroup location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_entry_groups_list(&self, parent: &str) -> ProjectLocationEntryGroupListCall<'a, S> {
        ProjectLocationEntryGroupListCall {
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
    /// Updates a EntryGroup resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the EntryGroup, of the form: projects/{project_number}/locations/{location_id}/entryGroups/{entry_group_id}.
    pub fn locations_entry_groups_patch(&self, request: GoogleCloudDataplexV1EntryGroup, name: &str) -> ProjectLocationEntryGroupPatchCall<'a, S> {
        ProjectLocationEntryGroupPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationEntryGroupSetIamPolicyCall<'a, S> {
        ProjectLocationEntryGroupSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_groups_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationEntryGroupTestIamPermissionCall<'a, S> {
        ProjectLocationEntryGroupTestIamPermissionCall {
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
    /// Creates an EntryType
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_entry_types_create(&self, request: GoogleCloudDataplexV1EntryType, parent: &str) -> ProjectLocationEntryTypeCreateCall<'a, S> {
        ProjectLocationEntryTypeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _entry_type_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a EntryType resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the EntryType: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}.
    pub fn locations_entry_types_delete(&self, name: &str) -> ProjectLocationEntryTypeDeleteCall<'a, S> {
        ProjectLocationEntryTypeDeleteCall {
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
    /// Retrieves a EntryType resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the EntryType: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}.
    pub fn locations_entry_types_get(&self, name: &str) -> ProjectLocationEntryTypeGetCall<'a, S> {
        ProjectLocationEntryTypeGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_types_get_iam_policy(&self, resource: &str) -> ProjectLocationEntryTypeGetIamPolicyCall<'a, S> {
        ProjectLocationEntryTypeGetIamPolicyCall {
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
    /// Lists EntryType resources in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the EntryType location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_entry_types_list(&self, parent: &str) -> ProjectLocationEntryTypeListCall<'a, S> {
        ProjectLocationEntryTypeListCall {
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
    /// Updates a EntryType resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the EntryType, of the form: projects/{project_number}/locations/{location_id}/entryTypes/{entry_type_id}.
    pub fn locations_entry_types_patch(&self, request: GoogleCloudDataplexV1EntryType, name: &str) -> ProjectLocationEntryTypePatchCall<'a, S> {
        ProjectLocationEntryTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_types_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationEntryTypeSetIamPolicyCall<'a, S> {
        ProjectLocationEntryTypeSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_entry_types_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationEntryTypeTestIamPermissionCall<'a, S> {
        ProjectLocationEntryTypeTestIamPermissionCall {
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
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_governance_rules_get_iam_policy(&self, resource: &str) -> ProjectLocationGovernanceRuleGetIamPolicyCall<'a, S> {
        ProjectLocationGovernanceRuleGetIamPolicyCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_governance_rules_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationGovernanceRuleSetIamPolicyCall<'a, S> {
        ProjectLocationGovernanceRuleSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_governance_rules_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationGovernanceRuleTestIamPermissionCall<'a, S> {
        ProjectLocationGovernanceRuleTestIamPermissionCall {
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
    /// Lists action resources in a lake.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_actions_list(&self, parent: &str) -> ProjectLocationLakeActionListCall<'a, S> {
        ProjectLocationLakeActionListCall {
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
    /// Create a content.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}
    pub fn locations_lakes_content_create(&self, request: GoogleCloudDataplexV1Content, parent: &str) -> ProjectLocationLakeContentCreateCall<'a, S> {
        ProjectLocationLakeContentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a content.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the content: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    pub fn locations_lakes_content_delete(&self, name: &str) -> ProjectLocationLakeContentDeleteCall<'a, S> {
        ProjectLocationLakeContentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a content resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the content: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    pub fn locations_lakes_content_get(&self, name: &str) -> ProjectLocationLakeContentGetCall<'a, S> {
        ProjectLocationLakeContentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a contentitem resource. A NOT_FOUND error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it.Caller must have Google IAM dataplex.content.getIamPolicy permission on the resource.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_content_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeContentGetIamPolicyCall<'a, S> {
        ProjectLocationLakeContentGetIamPolicyCall {
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
    /// List content.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}
    pub fn locations_lakes_content_list(&self, parent: &str) -> ProjectLocationLakeContentListCall<'a, S> {
        ProjectLocationLakeContentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Update a content. Only supports full resource update.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    pub fn locations_lakes_content_patch(&self, request: GoogleCloudDataplexV1Content, name: &str) -> ProjectLocationLakeContentPatchCall<'a, S> {
        ProjectLocationLakeContentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified contentitem resource. Replaces any existing policy.Caller must have Google IAM dataplex.content.setIamPolicy permission on the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_content_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeContentSetIamPolicyCall<'a, S> {
        ProjectLocationLakeContentSetIamPolicyCall {
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
    /// Returns the caller's permissions on a resource. If the resource does not exist, an empty set of permissions is returned (a NOT_FOUND error is not returned).A caller is not required to have Google IAM permission to make this request.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_content_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeContentTestIamPermissionCall<'a, S> {
        ProjectLocationLakeContentTestIamPermissionCall {
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
    /// Create a content.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}
    pub fn locations_lakes_contentitems_create(&self, request: GoogleCloudDataplexV1Content, parent: &str) -> ProjectLocationLakeContentitemCreateCall<'a, S> {
        ProjectLocationLakeContentitemCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a content.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the content: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    pub fn locations_lakes_contentitems_delete(&self, name: &str) -> ProjectLocationLakeContentitemDeleteCall<'a, S> {
        ProjectLocationLakeContentitemDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a content resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the content: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    pub fn locations_lakes_contentitems_get(&self, name: &str) -> ProjectLocationLakeContentitemGetCall<'a, S> {
        ProjectLocationLakeContentitemGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a contentitem resource. A NOT_FOUND error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it.Caller must have Google IAM dataplex.content.getIamPolicy permission on the resource.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_contentitems_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeContentitemGetIamPolicyCall<'a, S> {
        ProjectLocationLakeContentitemGetIamPolicyCall {
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
    /// List content.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}
    pub fn locations_lakes_contentitems_list(&self, parent: &str) -> ProjectLocationLakeContentitemListCall<'a, S> {
        ProjectLocationLakeContentitemListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Update a content. Only supports full resource update.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the content, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/content/{content_id}
    pub fn locations_lakes_contentitems_patch(&self, request: GoogleCloudDataplexV1Content, name: &str) -> ProjectLocationLakeContentitemPatchCall<'a, S> {
        ProjectLocationLakeContentitemPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified contentitem resource. Replaces any existing policy.Caller must have Google IAM dataplex.content.setIamPolicy permission on the resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_contentitems_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeContentitemSetIamPolicyCall<'a, S> {
        ProjectLocationLakeContentitemSetIamPolicyCall {
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
    /// Returns the caller's permissions on a resource. If the resource does not exist, an empty set of permissions is returned (a NOT_FOUND error is not returned).A caller is not required to have Google IAM permission to make this request.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_contentitems_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeContentitemTestIamPermissionCall<'a, S> {
        ProjectLocationLakeContentitemTestIamPermissionCall {
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
    /// Lists session resources in an environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent environment: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}.
    pub fn locations_lakes_environments_sessions_list(&self, parent: &str) -> ProjectLocationLakeEnvironmentSessionListCall<'a, S> {
        ProjectLocationLakeEnvironmentSessionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Create an environment resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_environments_create(&self, request: GoogleCloudDataplexV1Environment, parent: &str) -> ProjectLocationLakeEnvironmentCreateCall<'a, S> {
        ProjectLocationLakeEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _environment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete the environment resource. All the child resources must have been deleted before environment deletion can be initiated.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the environment: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environments/{environment_id}.
    pub fn locations_lakes_environments_delete(&self, name: &str) -> ProjectLocationLakeEnvironmentDeleteCall<'a, S> {
        ProjectLocationLakeEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get environment resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the environment: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environments/{environment_id}.
    pub fn locations_lakes_environments_get(&self, name: &str) -> ProjectLocationLakeEnvironmentGetCall<'a, S> {
        ProjectLocationLakeEnvironmentGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_environments_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeEnvironmentGetIamPolicyCall<'a, S> {
        ProjectLocationLakeEnvironmentGetIamPolicyCall {
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
    /// Lists environments under the given lake.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_id}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_environments_list(&self, parent: &str) -> ProjectLocationLakeEnvironmentListCall<'a, S> {
        ProjectLocationLakeEnvironmentListCall {
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
    /// Update the environment resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the environment, of the form: projects/{project_id}/locations/{location_id}/lakes/{lake_id}/environment/{environment_id}
    pub fn locations_lakes_environments_patch(&self, request: GoogleCloudDataplexV1Environment, name: &str) -> ProjectLocationLakeEnvironmentPatchCall<'a, S> {
        ProjectLocationLakeEnvironmentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_environments_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeEnvironmentSetIamPolicyCall<'a, S> {
        ProjectLocationLakeEnvironmentSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_environments_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeEnvironmentTestIamPermissionCall<'a, S> {
        ProjectLocationLakeEnvironmentTestIamPermissionCall {
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
    /// Cancel jobs running for the task resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the job: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/task/{task_id}/job/{job_id}.
    pub fn locations_lakes_tasks_jobs_cancel(&self, request: GoogleCloudDataplexV1CancelJobRequest, name: &str) -> ProjectLocationLakeTaskJobCancelCall<'a, S> {
        ProjectLocationLakeTaskJobCancelCall {
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
    /// Get job resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the job: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}/jobs/{job_id}.
    pub fn locations_lakes_tasks_jobs_get(&self, name: &str) -> ProjectLocationLakeTaskJobGetCall<'a, S> {
        ProjectLocationLakeTaskJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Jobs under the given task.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent environment: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}.
    pub fn locations_lakes_tasks_jobs_list(&self, parent: &str) -> ProjectLocationLakeTaskJobListCall<'a, S> {
        ProjectLocationLakeTaskJobListCall {
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
    /// Creates a task resource within a lake.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_tasks_create(&self, request: GoogleCloudDataplexV1Task, parent: &str) -> ProjectLocationLakeTaskCreateCall<'a, S> {
        ProjectLocationLakeTaskCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _task_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete the task resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the task: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/task/{task_id}.
    pub fn locations_lakes_tasks_delete(&self, name: &str) -> ProjectLocationLakeTaskDeleteCall<'a, S> {
        ProjectLocationLakeTaskDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get task resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the task: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{tasks_id}.
    pub fn locations_lakes_tasks_get(&self, name: &str) -> ProjectLocationLakeTaskGetCall<'a, S> {
        ProjectLocationLakeTaskGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_tasks_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeTaskGetIamPolicyCall<'a, S> {
        ProjectLocationLakeTaskGetIamPolicyCall {
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
    /// Lists tasks under the given lake.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_tasks_list(&self, parent: &str) -> ProjectLocationLakeTaskListCall<'a, S> {
        ProjectLocationLakeTaskListCall {
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
    /// Update the task resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the task, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/ tasks/{task_id}.
    pub fn locations_lakes_tasks_patch(&self, request: GoogleCloudDataplexV1Task, name: &str) -> ProjectLocationLakeTaskPatchCall<'a, S> {
        ProjectLocationLakeTaskPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Run an on demand execution of a Task.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the task: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/tasks/{task_id}.
    pub fn locations_lakes_tasks_run(&self, request: GoogleCloudDataplexV1RunTaskRequest, name: &str) -> ProjectLocationLakeTaskRunCall<'a, S> {
        ProjectLocationLakeTaskRunCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_tasks_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeTaskSetIamPolicyCall<'a, S> {
        ProjectLocationLakeTaskSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_tasks_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeTaskTestIamPermissionCall<'a, S> {
        ProjectLocationLakeTaskTestIamPermissionCall {
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
    /// Lists action resources in a zone.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_actions_list(&self, parent: &str) -> ProjectLocationLakeZoneActionListCall<'a, S> {
        ProjectLocationLakeZoneActionListCall {
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
    /// Lists action resources in an asset.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent asset: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}.
    pub fn locations_lakes_zones_assets_actions_list(&self, parent: &str) -> ProjectLocationLakeZoneAssetActionListCall<'a, S> {
        ProjectLocationLakeZoneAssetActionListCall {
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
    /// Creates an asset resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_assets_create(&self, request: GoogleCloudDataplexV1Asset, parent: &str) -> ProjectLocationLakeZoneAssetCreateCall<'a, S> {
        ProjectLocationLakeZoneAssetCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _asset_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an asset resource. The referenced storage resource is detached (default) or deleted based on the associated Lifecycle policy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the asset: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}.
    pub fn locations_lakes_zones_assets_delete(&self, name: &str) -> ProjectLocationLakeZoneAssetDeleteCall<'a, S> {
        ProjectLocationLakeZoneAssetDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an asset resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the asset: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}.
    pub fn locations_lakes_zones_assets_get(&self, name: &str) -> ProjectLocationLakeZoneAssetGetCall<'a, S> {
        ProjectLocationLakeZoneAssetGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_zones_assets_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeZoneAssetGetIamPolicyCall<'a, S> {
        ProjectLocationLakeZoneAssetGetIamPolicyCall {
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
    /// Lists asset resources in a zone.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_assets_list(&self, parent: &str) -> ProjectLocationLakeZoneAssetListCall<'a, S> {
        ProjectLocationLakeZoneAssetListCall {
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
    /// Updates an asset resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the asset, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/assets/{asset_id}.
    pub fn locations_lakes_zones_assets_patch(&self, request: GoogleCloudDataplexV1Asset, name: &str) -> ProjectLocationLakeZoneAssetPatchCall<'a, S> {
        ProjectLocationLakeZoneAssetPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_zones_assets_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeZoneAssetSetIamPolicyCall<'a, S> {
        ProjectLocationLakeZoneAssetSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_zones_assets_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeZoneAssetTestIamPermissionCall<'a, S> {
        ProjectLocationLakeZoneAssetTestIamPermissionCall {
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
    /// Create a metadata partition.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.
    pub fn locations_lakes_zones_entities_partitions_create(&self, request: GoogleCloudDataplexV1Partition, parent: &str) -> ProjectLocationLakeZoneEntityPartitionCreateCall<'a, S> {
        ProjectLocationLakeZoneEntityPartitionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a metadata partition.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the partition. format: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}/partitions/{partition_value_path}. The {partition_value_path} segment consists of an ordered sequence of partition values separated by "/". All values must be provided.
    pub fn locations_lakes_zones_entities_partitions_delete(&self, name: &str) -> ProjectLocationLakeZoneEntityPartitionDeleteCall<'a, S> {
        ProjectLocationLakeZoneEntityPartitionDeleteCall {
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
    /// Get a metadata partition of an entity.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the partition: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}/partitions/{partition_value_path}. The {partition_value_path} segment consists of an ordered sequence of partition values separated by "/". All values must be provided.
    pub fn locations_lakes_zones_entities_partitions_get(&self, name: &str) -> ProjectLocationLakeZoneEntityPartitionGetCall<'a, S> {
        ProjectLocationLakeZoneEntityPartitionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List metadata partitions of an entity.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent entity: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.
    pub fn locations_lakes_zones_entities_partitions_list(&self, parent: &str) -> ProjectLocationLakeZoneEntityPartitionListCall<'a, S> {
        ProjectLocationLakeZoneEntityPartitionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Create a metadata entity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_entities_create(&self, request: GoogleCloudDataplexV1Entity, parent: &str) -> ProjectLocationLakeZoneEntityCreateCall<'a, S> {
        ProjectLocationLakeZoneEntityCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a metadata entity.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the entity: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.
    pub fn locations_lakes_zones_entities_delete(&self, name: &str) -> ProjectLocationLakeZoneEntityDeleteCall<'a, S> {
        ProjectLocationLakeZoneEntityDeleteCall {
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
    /// Get a metadata entity.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the entity: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{entity_id}.
    pub fn locations_lakes_zones_entities_get(&self, name: &str) -> ProjectLocationLakeZoneEntityGetCall<'a, S> {
        ProjectLocationLakeZoneEntityGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List metadata entities in a zone.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_entities_list(&self, parent: &str) -> ProjectLocationLakeZoneEntityListCall<'a, S> {
        ProjectLocationLakeZoneEntityListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
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
    /// Update a metadata entity. Only supports full resource update.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the entity, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}/entities/{id}.
    pub fn locations_lakes_zones_entities_update(&self, request: GoogleCloudDataplexV1Entity, name: &str) -> ProjectLocationLakeZoneEntityUpdateCall<'a, S> {
        ProjectLocationLakeZoneEntityUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a zone resource within a lake.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_zones_create(&self, request: GoogleCloudDataplexV1Zone, parent: &str) -> ProjectLocationLakeZoneCreateCall<'a, S> {
        ProjectLocationLakeZoneCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _zone_id: Default::default(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a zone resource. All assets within a zone must be deleted before the zone can be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_delete(&self, name: &str) -> ProjectLocationLakeZoneDeleteCall<'a, S> {
        ProjectLocationLakeZoneDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a zone resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the zone: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_get(&self, name: &str) -> ProjectLocationLakeZoneGetCall<'a, S> {
        ProjectLocationLakeZoneGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_zones_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeZoneGetIamPolicyCall<'a, S> {
        ProjectLocationLakeZoneGetIamPolicyCall {
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
    /// Lists zone resources in a lake.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the parent lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_zones_list(&self, parent: &str) -> ProjectLocationLakeZoneListCall<'a, S> {
        ProjectLocationLakeZoneListCall {
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
    /// Updates a zone resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the zone, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}/zones/{zone_id}.
    pub fn locations_lakes_zones_patch(&self, request: GoogleCloudDataplexV1Zone, name: &str) -> ProjectLocationLakeZonePatchCall<'a, S> {
        ProjectLocationLakeZonePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_zones_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeZoneSetIamPolicyCall<'a, S> {
        ProjectLocationLakeZoneSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_zones_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeZoneTestIamPermissionCall<'a, S> {
        ProjectLocationLakeZoneTestIamPermissionCall {
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
    /// Creates a lake resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the lake location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_lakes_create(&self, request: GoogleCloudDataplexV1Lake, parent: &str) -> ProjectLocationLakeCreateCall<'a, S> {
        ProjectLocationLakeCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _lake_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a lake resource. All zones within the lake must be deleted before the lake can be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_delete(&self, name: &str) -> ProjectLocationLakeDeleteCall<'a, S> {
        ProjectLocationLakeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a lake resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the lake: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_get(&self, name: &str) -> ProjectLocationLakeGetCall<'a, S> {
        ProjectLocationLakeGetCall {
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
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_get_iam_policy(&self, resource: &str) -> ProjectLocationLakeGetIamPolicyCall<'a, S> {
        ProjectLocationLakeGetIamPolicyCall {
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
    /// Lists lake resources in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the lake location, of the form: projects/{project_number}/locations/{location_id} where location_id refers to a GCP region.
    pub fn locations_lakes_list(&self, parent: &str) -> ProjectLocationLakeListCall<'a, S> {
        ProjectLocationLakeListCall {
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
    /// Updates a lake resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The relative resource name of the lake, of the form: projects/{project_number}/locations/{location_id}/lakes/{lake_id}.
    pub fn locations_lakes_patch(&self, request: GoogleCloudDataplexV1Lake, name: &str) -> ProjectLocationLakePatchCall<'a, S> {
        ProjectLocationLakePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> ProjectLocationLakeSetIamPolicyCall<'a, S> {
        ProjectLocationLakeSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See Resource names (https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn locations_lakes_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> ProjectLocationLakeTestIamPermissionCall<'a, S> {
        ProjectLocationLakeTestIamPermissionCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.
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
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED.
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
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.
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
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up a single entry.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project to which the request should be attributed in the following form: projects/{project}/locations/{location}.
    pub fn locations_lookup_entry(&self, name: &str) -> ProjectLocationLookupEntryCall<'a, S> {
        ProjectLocationLookupEntryCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _paths: Default::default(),
            _entry: Default::default(),
            _aspect_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for entries matching given query and scope.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The project to which the request should be attributed in the following form: projects/{project}/locations/{location}.
    pub fn locations_search_entries(&self, name: &str) -> ProjectLocationSearchEntryCall<'a, S> {
        ProjectLocationSearchEntryCall {
            hub: self.hub,
            _name: name.to_string(),
            _scope: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



