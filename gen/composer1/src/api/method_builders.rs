use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudComposer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_composer1 as composer1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use composer1::{CloudComposer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudComposer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_environments_create(...)`, `locations_environments_database_failover(...)`, `locations_environments_delete(...)`, `locations_environments_execute_airflow_command(...)`, `locations_environments_fetch_database_properties(...)`, `locations_environments_get(...)`, `locations_environments_list(...)`, `locations_environments_load_snapshot(...)`, `locations_environments_patch(...)`, `locations_environments_poll_airflow_command(...)`, `locations_environments_save_snapshot(...)`, `locations_environments_stop_airflow_command(...)`, `locations_environments_user_workloads_config_maps_create(...)`, `locations_environments_user_workloads_config_maps_delete(...)`, `locations_environments_user_workloads_config_maps_get(...)`, `locations_environments_user_workloads_config_maps_list(...)`, `locations_environments_user_workloads_config_maps_update(...)`, `locations_environments_user_workloads_secrets_create(...)`, `locations_environments_user_workloads_secrets_delete(...)`, `locations_environments_user_workloads_secrets_get(...)`, `locations_environments_user_workloads_secrets_list(...)`, `locations_environments_user_workloads_secrets_update(...)`, `locations_environments_workloads_list(...)`, `locations_image_versions_list(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)` and `locations_operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudComposer<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The environment name to create a ConfigMap for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_user_workloads_config_maps_create(&self, request: UserWorkloadsConfigMap, parent: &str) -> ProjectLocationEnvironmentUserWorkloadsConfigMapCreateCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsConfigMapCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The ConfigMap to delete, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}"
    pub fn locations_environments_user_workloads_config_maps_delete(&self, name: &str) -> ProjectLocationEnvironmentUserWorkloadsConfigMapDeleteCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsConfigMapDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an existing user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the ConfigMap to get, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}"
    pub fn locations_environments_user_workloads_config_maps_get(&self, name: &str) -> ProjectLocationEnvironmentUserWorkloadsConfigMapGetCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsConfigMapGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists user workloads ConfigMaps. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. List ConfigMaps in the given environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_user_workloads_config_maps_list(&self, parent: &str) -> ProjectLocationEnvironmentUserWorkloadsConfigMapListCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsConfigMapListCall {
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
    /// Updates a user workloads ConfigMap. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The resource name of the ConfigMap, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsConfigMaps/{userWorkloadsConfigMapId}"
    pub fn locations_environments_user_workloads_config_maps_update(&self, request: UserWorkloadsConfigMap, name: &str) -> ProjectLocationEnvironmentUserWorkloadsConfigMapUpdateCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsConfigMapUpdateCall {
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
    /// Creates a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The environment name to create a Secret for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_user_workloads_secrets_create(&self, request: UserWorkloadsSecret, parent: &str) -> ProjectLocationEnvironmentUserWorkloadsSecretCreateCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsSecretCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The Secret to delete, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}"
    pub fn locations_environments_user_workloads_secrets_delete(&self, name: &str) -> ProjectLocationEnvironmentUserWorkloadsSecretDeleteCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsSecretDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an existing user workloads Secret. Values of the "data" field in the response are cleared. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Secret to get, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}"
    pub fn locations_environments_user_workloads_secrets_get(&self, name: &str) -> ProjectLocationEnvironmentUserWorkloadsSecretGetCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsSecretGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists user workloads Secrets. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. List Secrets in the given environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_user_workloads_secrets_list(&self, parent: &str) -> ProjectLocationEnvironmentUserWorkloadsSecretListCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsSecretListCall {
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
    /// Updates a user workloads Secret. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Identifier. The resource name of the Secret, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}/userWorkloadsSecrets/{userWorkloadsSecretId}"
    pub fn locations_environments_user_workloads_secrets_update(&self, request: UserWorkloadsSecret, name: &str) -> ProjectLocationEnvironmentUserWorkloadsSecretUpdateCall<'a, S> {
        ProjectLocationEnvironmentUserWorkloadsSecretUpdateCall {
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
    /// Lists workloads in a Cloud Composer environment. Workload is a unit that runs a single Composer component. This method is supported for Cloud Composer environments in versions composer-3.*.*-airflow-*.*.* and newer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The environment name to get workloads for, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_workloads_list(&self, parent: &str) -> ProjectLocationEnvironmentWorkloadListCall<'a, S> {
        ProjectLocationEnvironmentWorkloadListCall {
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
    /// Create a new environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The parent must be of the form "projects/{projectId}/locations/{locationId}".
    pub fn locations_environments_create(&self, request: Environment, parent: &str) -> ProjectLocationEnvironmentCreateCall<'a, S> {
        ProjectLocationEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Triggers database failover (only for highly resilient environments).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - Target environment: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_database_failover(&self, request: DatabaseFailoverRequest, environment: &str) -> ProjectLocationEnvironmentDatabaseFailoverCall<'a, S> {
        ProjectLocationEnvironmentDatabaseFailoverCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - The environment to delete, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_delete(&self, name: &str) -> ProjectLocationEnvironmentDeleteCall<'a, S> {
        ProjectLocationEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes Airflow CLI command.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - The resource name of the environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}".
    pub fn locations_environments_execute_airflow_command(&self, request: ExecuteAirflowCommandRequest, environment: &str) -> ProjectLocationEnvironmentExecuteAirflowCommandCall<'a, S> {
        ProjectLocationEnvironmentExecuteAirflowCommandCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches database properties.
    /// 
    /// # Arguments
    ///
    /// * `environment` - Required. The resource name of the environment, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_fetch_database_properties(&self, environment: &str) -> ProjectLocationEnvironmentFetchDatabasePropertyCall<'a, S> {
        ProjectLocationEnvironmentFetchDatabasePropertyCall {
            hub: self.hub,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an existing environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the environment to get, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_get(&self, name: &str) -> ProjectLocationEnvironmentGetCall<'a, S> {
        ProjectLocationEnvironmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List environments.
    /// 
    /// # Arguments
    ///
    /// * `parent` - List environments in the given project and location, in the form: "projects/{projectId}/locations/{locationId}"
    pub fn locations_environments_list(&self, parent: &str) -> ProjectLocationEnvironmentListCall<'a, S> {
        ProjectLocationEnvironmentListCall {
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
    /// Loads a snapshot of a Cloud Composer environment. As a result of this operation, a snapshot of environment's specified in LoadSnapshotRequest is loaded into the environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - The resource name of the target environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_load_snapshot(&self, request: LoadSnapshotRequest, environment: &str) -> ProjectLocationEnvironmentLoadSnapshotCall<'a, S> {
        ProjectLocationEnvironmentLoadSnapshotCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the environment to update, in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_patch(&self, request: Environment, name: &str) -> ProjectLocationEnvironmentPatchCall<'a, S> {
        ProjectLocationEnvironmentPatchCall {
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
    /// Polls Airflow CLI command execution and fetches logs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - The resource name of the environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_poll_airflow_command(&self, request: PollAirflowCommandRequest, environment: &str) -> ProjectLocationEnvironmentPollAirflowCommandCall<'a, S> {
        ProjectLocationEnvironmentPollAirflowCommandCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a snapshots of a Cloud Composer environment. As a result of this operation, snapshot of environment's state is stored in a location specified in the SaveSnapshotRequest.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - The resource name of the source environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    pub fn locations_environments_save_snapshot(&self, request: SaveSnapshotRequest, environment: &str) -> ProjectLocationEnvironmentSaveSnapshotCall<'a, S> {
        ProjectLocationEnvironmentSaveSnapshotCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops Airflow CLI command execution.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `environment` - The resource name of the environment in the form: "projects/{projectId}/locations/{locationId}/environments/{environmentId}".
    pub fn locations_environments_stop_airflow_command(&self, request: StopAirflowCommandRequest, environment: &str) -> ProjectLocationEnvironmentStopAirflowCommandCall<'a, S> {
        ProjectLocationEnvironmentStopAirflowCommandCall {
            hub: self.hub,
            _request: request,
            _environment: environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List ImageVersions for provided location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - List ImageVersions in the given project and location, in the form: "projects/{projectId}/locations/{locationId}"
    pub fn locations_image_versions_list(&self, parent: &str) -> ProjectLocationImageVersionListCall<'a, S> {
        ProjectLocationImageVersionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include_past_releases: Default::default(),
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
}



