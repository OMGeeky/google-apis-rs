use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Spanner`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_spanner1 as spanner1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use spanner1::{Spanner, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Spanner::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `instance_config_operations_list(...)`, `instance_configs_create(...)`, `instance_configs_delete(...)`, `instance_configs_get(...)`, `instance_configs_list(...)`, `instance_configs_operations_cancel(...)`, `instance_configs_operations_delete(...)`, `instance_configs_operations_get(...)`, `instance_configs_operations_list(...)`, `instance_configs_patch(...)`, `instance_configs_ssd_caches_operations_cancel(...)`, `instance_configs_ssd_caches_operations_delete(...)`, `instance_configs_ssd_caches_operations_get(...)`, `instance_configs_ssd_caches_operations_list(...)`, `instances_backup_operations_list(...)`, `instances_backups_copy(...)`, `instances_backups_create(...)`, `instances_backups_delete(...)`, `instances_backups_get(...)`, `instances_backups_get_iam_policy(...)`, `instances_backups_list(...)`, `instances_backups_operations_cancel(...)`, `instances_backups_operations_delete(...)`, `instances_backups_operations_get(...)`, `instances_backups_operations_list(...)`, `instances_backups_patch(...)`, `instances_backups_set_iam_policy(...)`, `instances_backups_test_iam_permissions(...)`, `instances_create(...)`, `instances_database_operations_list(...)`, `instances_databases_create(...)`, `instances_databases_database_roles_list(...)`, `instances_databases_database_roles_test_iam_permissions(...)`, `instances_databases_drop_database(...)`, `instances_databases_get(...)`, `instances_databases_get_ddl(...)`, `instances_databases_get_iam_policy(...)`, `instances_databases_get_scans(...)`, `instances_databases_list(...)`, `instances_databases_operations_cancel(...)`, `instances_databases_operations_delete(...)`, `instances_databases_operations_get(...)`, `instances_databases_operations_list(...)`, `instances_databases_patch(...)`, `instances_databases_restore(...)`, `instances_databases_sessions_batch_create(...)`, `instances_databases_sessions_batch_write(...)`, `instances_databases_sessions_begin_transaction(...)`, `instances_databases_sessions_commit(...)`, `instances_databases_sessions_create(...)`, `instances_databases_sessions_delete(...)`, `instances_databases_sessions_execute_batch_dml(...)`, `instances_databases_sessions_execute_sql(...)`, `instances_databases_sessions_execute_streaming_sql(...)`, `instances_databases_sessions_get(...)`, `instances_databases_sessions_list(...)`, `instances_databases_sessions_partition_query(...)`, `instances_databases_sessions_partition_read(...)`, `instances_databases_sessions_read(...)`, `instances_databases_sessions_rollback(...)`, `instances_databases_sessions_streaming_read(...)`, `instances_databases_set_iam_policy(...)`, `instances_databases_test_iam_permissions(...)`, `instances_databases_update_ddl(...)`, `instances_delete(...)`, `instances_get(...)`, `instances_get_iam_policy(...)`, `instances_instance_partition_operations_list(...)`, `instances_instance_partitions_create(...)`, `instances_instance_partitions_delete(...)`, `instances_instance_partitions_get(...)`, `instances_instance_partitions_list(...)`, `instances_instance_partitions_operations_cancel(...)`, `instances_instance_partitions_operations_delete(...)`, `instances_instance_partitions_operations_get(...)`, `instances_instance_partitions_operations_list(...)`, `instances_instance_partitions_patch(...)`, `instances_list(...)`, `instances_move(...)`, `instances_operations_cancel(...)`, `instances_operations_delete(...)`, `instances_operations_get(...)`, `instances_operations_list(...)`, `instances_patch(...)`, `instances_set_iam_policy(...)` and `instances_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Spanner<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user-managed instance config long-running operations in the given project. An instance config operation has a name of the form `projects//instanceConfigs//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.start_time` in descending order starting from the most recently started operation.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project of the instance config operations. Values are of the form `projects/`.
    pub fn instance_config_operations_list(&self, parent: &str) -> ProjectInstanceConfigOperationListCall<'a, S> {
        ProjectInstanceConfigOperationListCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn instance_configs_operations_cancel(&self, name: &str) -> ProjectInstanceConfigOperationCancelCall<'a, S> {
        ProjectInstanceConfigOperationCancelCall {
            hub: self.hub,
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
    pub fn instance_configs_operations_delete(&self, name: &str) -> ProjectInstanceConfigOperationDeleteCall<'a, S> {
        ProjectInstanceConfigOperationDeleteCall {
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
    pub fn instance_configs_operations_get(&self, name: &str) -> ProjectInstanceConfigOperationGetCall<'a, S> {
        ProjectInstanceConfigOperationGetCall {
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
    pub fn instance_configs_operations_list(&self, name: &str) -> ProjectInstanceConfigOperationListCall1<'a, S> {
        ProjectInstanceConfigOperationListCall1 {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn instance_configs_ssd_caches_operations_cancel(&self, name: &str) -> ProjectInstanceConfigSsdCachOperationCancelCall<'a, S> {
        ProjectInstanceConfigSsdCachOperationCancelCall {
            hub: self.hub,
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
    pub fn instance_configs_ssd_caches_operations_delete(&self, name: &str) -> ProjectInstanceConfigSsdCachOperationDeleteCall<'a, S> {
        ProjectInstanceConfigSsdCachOperationDeleteCall {
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
    pub fn instance_configs_ssd_caches_operations_get(&self, name: &str) -> ProjectInstanceConfigSsdCachOperationGetCall<'a, S> {
        ProjectInstanceConfigSsdCachOperationGetCall {
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
    pub fn instance_configs_ssd_caches_operations_list(&self, name: &str) -> ProjectInstanceConfigSsdCachOperationListCall<'a, S> {
        ProjectInstanceConfigSsdCachOperationListCall {
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
    /// Creates an instance config and begins preparing it to be used. The returned long-running operation can be used to track the progress of preparing the new instance config. The instance config name is assigned by the caller. If the named instance config already exists, `CreateInstanceConfig` returns `ALREADY_EXISTS`. Immediately after the request returns: * The instance config is readable via the API, with all requested attributes. The instance config's reconciling field is set to true. Its state is `CREATING`. While the operation is pending: * Cancelling the operation renders the instance config immediately unreadable via the API. * Except for deleting the creating resource, all other attempts to modify the instance config are rejected. Upon completion of the returned operation: * Instances can be created using the instance configuration. * The instance config's reconciling field becomes false. Its state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance config. The metadata field type is CreateInstanceConfigMetadata. The response field type is InstanceConfig, if successful. Authorization requires `spanner.instanceConfigs.create` permission on the resource parent.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which to create the instance config. Values are of the form `projects/`.
    pub fn instance_configs_create(&self, request: CreateInstanceConfigRequest, parent: &str) -> ProjectInstanceConfigCreateCall<'a, S> {
        ProjectInstanceConfigCreateCall {
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
    /// Deletes the instance config. Deletion is only allowed when no instances are using the configuration. If any instances are using the config, returns `FAILED_PRECONDITION`. Only user managed configurations can be deleted. Authorization requires `spanner.instanceConfigs.delete` permission on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the instance configuration to be deleted. Values are of the form `projects//instanceConfigs/`
    pub fn instance_configs_delete(&self, name: &str) -> ProjectInstanceConfigDeleteCall<'a, S> {
        ProjectInstanceConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a particular instance configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the requested instance configuration. Values are of the form `projects//instanceConfigs/`.
    pub fn instance_configs_get(&self, name: &str) -> ProjectInstanceConfigGetCall<'a, S> {
        ProjectInstanceConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the supported instance configurations for a given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project for which a list of supported instance configurations is requested. Values are of the form `projects/`.
    pub fn instance_configs_list(&self, parent: &str) -> ProjectInstanceConfigListCall<'a, S> {
        ProjectInstanceConfigListCall {
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
    /// Updates an instance config. The returned long-running operation can be used to track the progress of updating the instance. If the named instance config does not exist, returns `NOT_FOUND`. Only user managed configurations can be updated. Immediately after the request returns: * The instance config's reconciling field is set to true. While the operation is pending: * Cancelling the operation sets its metadata's cancel_time. The operation is guaranteed to succeed at undoing all changes, after which point it terminates with a `CANCELLED` status. * All other attempts to modify the instance config are rejected. * Reading the instance config via the API continues to give the pre-request values. Upon completion of the returned operation: * Creating instances using the instance configuration uses the new values. * The instance config's new values are readable via the API. * The instance config's reconciling field becomes false. The returned long-running operation will have a name of the format `/operations/` and can be used to track the instance config modification. The metadata field type is UpdateInstanceConfigMetadata. The response field type is InstanceConfig, if successful. Authorization requires `spanner.instanceConfigs.update` permission on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A unique identifier for the instance configuration. Values are of the form `projects//instanceConfigs/a-z*`.
    pub fn instance_configs_patch(&self, request: UpdateInstanceConfigRequest, name: &str) -> ProjectInstanceConfigPatchCall<'a, S> {
        ProjectInstanceConfigPatchCall {
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
    /// Lists the backup long-running operations in the given instance. A backup operation has a name of the form `projects//instances//backups//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.progress.start_time` in descending order starting from the most recently started operation.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance of the backup operations. Values are of the form `projects//instances/`.
    pub fn instances_backup_operations_list(&self, parent: &str) -> ProjectInstanceBackupOperationListCall<'a, S> {
        ProjectInstanceBackupOperationListCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn instances_backups_operations_cancel(&self, name: &str) -> ProjectInstanceBackupOperationCancelCall<'a, S> {
        ProjectInstanceBackupOperationCancelCall {
            hub: self.hub,
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
    pub fn instances_backups_operations_delete(&self, name: &str) -> ProjectInstanceBackupOperationDeleteCall<'a, S> {
        ProjectInstanceBackupOperationDeleteCall {
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
    pub fn instances_backups_operations_get(&self, name: &str) -> ProjectInstanceBackupOperationGetCall<'a, S> {
        ProjectInstanceBackupOperationGetCall {
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
    pub fn instances_backups_operations_list(&self, name: &str) -> ProjectInstanceBackupOperationListCall1<'a, S> {
        ProjectInstanceBackupOperationListCall1 {
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
    /// Starts copying a Cloud Spanner Backup. The returned backup long-running operation will have a name of the format `projects//instances//backups//operations/` and can be used to track copying of the backup. The operation is associated with the destination backup. The metadata field type is CopyBackupMetadata. The response field type is Backup, if successful. Cancelling the returned operation will stop the copying and delete the destination backup. Concurrent CopyBackup requests can run on the same source backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the destination instance that will contain the backup copy. Values are of the form: `projects//instances/`.
    pub fn instances_backups_copy(&self, request: CopyBackupRequest, parent: &str) -> ProjectInstanceBackupCopyCall<'a, S> {
        ProjectInstanceBackupCopyCall {
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
    /// Starts creating a new Cloud Spanner Backup. The returned backup long-running operation will have a name of the format `projects//instances//backups//operations/` and can be used to track creation of the backup. The metadata field type is CreateBackupMetadata. The response field type is Backup, if successful. Cancelling the returned operation will stop the creation and delete the backup. There can be only one pending backup creation per database. Backup creation of different databases can run concurrently.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the instance in which the backup will be created. This must be the same instance that contains the database the backup will be created from. The backup will be stored in the location(s) specified in the instance configuration of this instance. Values are of the form `projects//instances/`.
    pub fn instances_backups_create(&self, request: Backup, parent: &str) -> ProjectInstanceBackupCreateCall<'a, S> {
        ProjectInstanceBackupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _encryption_config_kms_key_name: Default::default(),
            _encryption_config_encryption_type: Default::default(),
            _backup_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a pending or completed Backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the backup to delete. Values are of the form `projects//instances//backups/`.
    pub fn instances_backups_delete(&self, name: &str) -> ProjectInstanceBackupDeleteCall<'a, S> {
        ProjectInstanceBackupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets metadata on a pending or completed Backup.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the backup. Values are of the form `projects//instances//backups/`.
    pub fn instances_backups_get(&self, name: &str) -> ProjectInstanceBackupGetCall<'a, S> {
        ProjectInstanceBackupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a database or backup resource. Returns an empty policy if a database or backup exists but does not have a policy set. Authorization requires `spanner.databases.getIamPolicy` permission on resource. For backups, authorization requires `spanner.backups.getIamPolicy` permission on resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which the policy is being retrieved. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_backups_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectInstanceBackupGetIamPolicyCall<'a, S> {
        ProjectInstanceBackupGetIamPolicyCall {
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
    /// Lists completed and pending backups. Backups returned are ordered by `create_time` in descending order, starting from the most recent `create_time`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance to list backups from. Values are of the form `projects//instances/`.
    pub fn instances_backups_list(&self, parent: &str) -> ProjectInstanceBackupListCall<'a, S> {
        ProjectInstanceBackupListCall {
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
    /// Updates a pending or completed Backup.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only for the CreateBackup operation. Required for the UpdateBackup operation. A globally unique identifier for the backup which cannot be changed. Values are of the form `projects//instances//backups/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. The backup is stored in the location(s) specified in the instance configuration of the instance containing the backup, identified by the prefix of the backup name of the form `projects//instances/`.
    pub fn instances_backups_patch(&self, request: Backup, name: &str) -> ProjectInstanceBackupPatchCall<'a, S> {
        ProjectInstanceBackupPatchCall {
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
    /// Sets the access control policy on a database or backup resource. Replaces any existing policy. Authorization requires `spanner.databases.setIamPolicy` permission on resource. For backups, authorization requires `spanner.backups.setIamPolicy` permission on resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which the policy is being set. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for databases resources.
    pub fn instances_backups_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectInstanceBackupSetIamPolicyCall<'a, S> {
        ProjectInstanceBackupSetIamPolicyCall {
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
    /// Returns permissions that the caller has on the specified database or backup resource. Attempting this RPC on a non-existent Cloud Spanner database will result in a NOT_FOUND error if the user has `spanner.databases.list` permission on the containing Cloud Spanner instance. Otherwise returns an empty set of permissions. Calling this method on a backup that does not exist will result in a NOT_FOUND error if the user has `spanner.backups.list` permission on the containing instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which permissions are being tested. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_backups_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceBackupTestIamPermissionCall<'a, S> {
        ProjectInstanceBackupTestIamPermissionCall {
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
    /// Lists database longrunning-operations. A database operation has a name of the form `projects//instances//databases//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance of the database operations. Values are of the form `projects//instances/`.
    pub fn instances_database_operations_list(&self, parent: &str) -> ProjectInstanceDatabaseOperationListCall<'a, S> {
        ProjectInstanceDatabaseOperationListCall {
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
    /// Lists Cloud Spanner database roles.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The database whose roles should be listed. Values are of the form `projects//instances//databases/`.
    pub fn instances_databases_database_roles_list(&self, parent: &str) -> ProjectInstanceDatabaseDatabaseRoleListCall<'a, S> {
        ProjectInstanceDatabaseDatabaseRoleListCall {
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
    /// Returns permissions that the caller has on the specified database or backup resource. Attempting this RPC on a non-existent Cloud Spanner database will result in a NOT_FOUND error if the user has `spanner.databases.list` permission on the containing Cloud Spanner instance. Otherwise returns an empty set of permissions. Calling this method on a backup that does not exist will result in a NOT_FOUND error if the user has `spanner.backups.list` permission on the containing instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which permissions are being tested. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_databases_database_roles_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceDatabaseDatabaseRoleTestIamPermissionCall<'a, S> {
        ProjectInstanceDatabaseDatabaseRoleTestIamPermissionCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn instances_databases_operations_cancel(&self, name: &str) -> ProjectInstanceDatabaseOperationCancelCall<'a, S> {
        ProjectInstanceDatabaseOperationCancelCall {
            hub: self.hub,
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
    pub fn instances_databases_operations_delete(&self, name: &str) -> ProjectInstanceDatabaseOperationDeleteCall<'a, S> {
        ProjectInstanceDatabaseOperationDeleteCall {
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
    pub fn instances_databases_operations_get(&self, name: &str) -> ProjectInstanceDatabaseOperationGetCall<'a, S> {
        ProjectInstanceDatabaseOperationGetCall {
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
    pub fn instances_databases_operations_list(&self, name: &str) -> ProjectInstanceDatabaseOperationListCall1<'a, S> {
        ProjectInstanceDatabaseOperationListCall1 {
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
    /// Creates multiple new sessions. This API can be used to initialize a session cache on the clients. See https://goo.gl/TgSFN2 for best practices on session cache management.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database in which the new sessions are created.
    pub fn instances_databases_sessions_batch_create(&self, request: BatchCreateSessionsRequest, database: &str) -> ProjectInstanceDatabaseSessionBatchCreateCall<'a, S> {
        ProjectInstanceDatabaseSessionBatchCreateCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batches the supplied mutation groups in a collection of efficient transactions. All mutations in a group are committed atomically. However, mutations across groups can be committed non-atomically in an unspecified order and thus, they must be independent of each other. Partial failure is possible, i.e., some groups may have been committed successfully, while some may have failed. The results of individual batches are streamed into the response as the batches are applied. BatchWrite requests are not replay protected, meaning that each mutation group may be applied more than once. Replays of non-idempotent mutations may have undesirable effects. For example, replays of an insert mutation may produce an already exists error or if you use generated or commit timestamp-based keys, it may result in additional rows being added to the mutation's table. We recommend structuring your mutation groups to be idempotent to avoid this issue.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the batch request is to be run.
    pub fn instances_databases_sessions_batch_write(&self, request: BatchWriteRequest, session: &str) -> ProjectInstanceDatabaseSessionBatchWriteCall<'a, S> {
        ProjectInstanceDatabaseSessionBatchWriteCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Begins a new transaction. This step can often be skipped: Read, ExecuteSql and Commit can begin a new transaction as a side-effect.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the transaction runs.
    pub fn instances_databases_sessions_begin_transaction(&self, request: BeginTransactionRequest, session: &str) -> ProjectInstanceDatabaseSessionBeginTransactionCall<'a, S> {
        ProjectInstanceDatabaseSessionBeginTransactionCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commits a transaction. The request includes the mutations to be applied to rows in the database. `Commit` might return an `ABORTED` error. This can occur at any time; commonly, the cause is conflicts with concurrent transactions. However, it can also happen for a variety of other reasons. If `Commit` returns `ABORTED`, the caller should re-attempt the transaction from the beginning, re-using the same session. On very rare occasions, `Commit` might return `UNKNOWN`. This can happen, for example, if the client job experiences a 1+ hour networking failure. At that point, Cloud Spanner has lost track of the transaction outcome and we recommend that you perform another read from the database to see the state of things as they are now.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the transaction to be committed is running.
    pub fn instances_databases_sessions_commit(&self, request: CommitRequest, session: &str) -> ProjectInstanceDatabaseSessionCommitCall<'a, S> {
        ProjectInstanceDatabaseSessionCommitCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new session. A session can be used to perform transactions that read and/or modify data in a Cloud Spanner database. Sessions are meant to be reused for many consecutive transactions. Sessions can only execute one transaction at a time. To execute multiple concurrent read-write/write-only transactions, create multiple sessions. Note that standalone reads and queries use a transaction internally, and count toward the one transaction limit. Active sessions use additional server resources, so it is a good idea to delete idle and unneeded sessions. Aside from explicit deletes, Cloud Spanner may delete sessions for which no operations are sent for more than an hour. If a session is deleted, requests to it return `NOT_FOUND`. Idle sessions can be kept alive by sending a trivial SQL query periodically, e.g., `"SELECT 1"`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database in which the new session is created.
    pub fn instances_databases_sessions_create(&self, request: CreateSessionRequest, database: &str) -> ProjectInstanceDatabaseSessionCreateCall<'a, S> {
        ProjectInstanceDatabaseSessionCreateCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Ends a session, releasing server resources associated with it. This will asynchronously trigger cancellation of any operations that are running with this session.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session to delete.
    pub fn instances_databases_sessions_delete(&self, name: &str) -> ProjectInstanceDatabaseSessionDeleteCall<'a, S> {
        ProjectInstanceDatabaseSessionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes a batch of SQL DML statements. This method allows many statements to be run with lower latency than submitting them sequentially with ExecuteSql. Statements are executed in sequential order. A request can succeed even if a statement fails. The ExecuteBatchDmlResponse.status field in the response provides information about the statement that failed. Clients must inspect this field to determine whether an error occurred. Execution stops after the first failed statement; the remaining statements are not executed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the DML statements should be performed.
    pub fn instances_databases_sessions_execute_batch_dml(&self, request: ExecuteBatchDmlRequest, session: &str) -> ProjectInstanceDatabaseSessionExecuteBatchDmlCall<'a, S> {
        ProjectInstanceDatabaseSessionExecuteBatchDmlCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes an SQL statement, returning all results in a single reply. This method cannot be used to return a result set larger than 10 MiB; if the query yields more data than that, the query fails with a `FAILED_PRECONDITION` error. Operations inside read-write transactions might return `ABORTED`. If this occurs, the application should restart the transaction from the beginning. See Transaction for more details. Larger result sets can be fetched in streaming fashion by calling ExecuteStreamingSql instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the SQL query should be performed.
    pub fn instances_databases_sessions_execute_sql(&self, request: ExecuteSqlRequest, session: &str) -> ProjectInstanceDatabaseSessionExecuteSqlCall<'a, S> {
        ProjectInstanceDatabaseSessionExecuteSqlCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Like ExecuteSql, except returns the result set as a stream. Unlike ExecuteSql, there is no limit on the size of the returned result set. However, no individual row in the result set can exceed 100 MiB, and no column value can exceed 10 MiB.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the SQL query should be performed.
    pub fn instances_databases_sessions_execute_streaming_sql(&self, request: ExecuteSqlRequest, session: &str) -> ProjectInstanceDatabaseSessionExecuteStreamingSqlCall<'a, S> {
        ProjectInstanceDatabaseSessionExecuteStreamingSqlCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a session. Returns `NOT_FOUND` if the session does not exist. This is mainly useful for determining whether a session is still alive.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the session to retrieve.
    pub fn instances_databases_sessions_get(&self, name: &str) -> ProjectInstanceDatabaseSessionGetCall<'a, S> {
        ProjectInstanceDatabaseSessionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all sessions in a given database.
    /// 
    /// # Arguments
    ///
    /// * `database` - Required. The database in which to list sessions.
    pub fn instances_databases_sessions_list(&self, database: &str) -> ProjectInstanceDatabaseSessionListCall<'a, S> {
        ProjectInstanceDatabaseSessionListCall {
            hub: self.hub,
            _database: database.to_string(),
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
    /// Creates a set of partition tokens that can be used to execute a query operation in parallel. Each of the returned partition tokens can be used by ExecuteStreamingSql to specify a subset of the query result to read. The same session and read-only transaction must be used by the PartitionQueryRequest used to create the partition tokens and the ExecuteSqlRequests that use the partition tokens. Partition tokens become invalid when the session used to create them is deleted, is idle for too long, begins a new transaction, or becomes too old. When any of these happen, it is not possible to resume the query, and the whole operation must be restarted from the beginning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session used to create the partitions.
    pub fn instances_databases_sessions_partition_query(&self, request: PartitionQueryRequest, session: &str) -> ProjectInstanceDatabaseSessionPartitionQueryCall<'a, S> {
        ProjectInstanceDatabaseSessionPartitionQueryCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a set of partition tokens that can be used to execute a read operation in parallel. Each of the returned partition tokens can be used by StreamingRead to specify a subset of the read result to read. The same session and read-only transaction must be used by the PartitionReadRequest used to create the partition tokens and the ReadRequests that use the partition tokens. There are no ordering guarantees on rows returned among the returned partition tokens, or even within each individual StreamingRead call issued with a partition_token. Partition tokens become invalid when the session used to create them is deleted, is idle for too long, begins a new transaction, or becomes too old. When any of these happen, it is not possible to resume the read, and the whole operation must be restarted from the beginning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session used to create the partitions.
    pub fn instances_databases_sessions_partition_read(&self, request: PartitionReadRequest, session: &str) -> ProjectInstanceDatabaseSessionPartitionReadCall<'a, S> {
        ProjectInstanceDatabaseSessionPartitionReadCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reads rows from the database using key lookups and scans, as a simple key/value style alternative to ExecuteSql. This method cannot be used to return a result set larger than 10 MiB; if the read matches more data than that, the read fails with a `FAILED_PRECONDITION` error. Reads inside read-write transactions might return `ABORTED`. If this occurs, the application should restart the transaction from the beginning. See Transaction for more details. Larger result sets can be yielded in streaming fashion by calling StreamingRead instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the read should be performed.
    pub fn instances_databases_sessions_read(&self, request: ReadRequest, session: &str) -> ProjectInstanceDatabaseSessionReadCall<'a, S> {
        ProjectInstanceDatabaseSessionReadCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rolls back a transaction, releasing any locks it holds. It is a good idea to call this for any transaction that includes one or more Read or ExecuteSql requests and ultimately decides not to commit. `Rollback` returns `OK` if it successfully aborts the transaction, the transaction was already aborted, or the transaction is not found. `Rollback` never returns `ABORTED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the transaction to roll back is running.
    pub fn instances_databases_sessions_rollback(&self, request: RollbackRequest, session: &str) -> ProjectInstanceDatabaseSessionRollbackCall<'a, S> {
        ProjectInstanceDatabaseSessionRollbackCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Like Read, except returns the result set as a stream. Unlike Read, there is no limit on the size of the returned result set. However, no individual row in the result set can exceed 100 MiB, and no column value can exceed 10 MiB.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `session` - Required. The session in which the read should be performed.
    pub fn instances_databases_sessions_streaming_read(&self, request: ReadRequest, session: &str) -> ProjectInstanceDatabaseSessionStreamingReadCall<'a, S> {
        ProjectInstanceDatabaseSessionStreamingReadCall {
            hub: self.hub,
            _request: request,
            _session: session.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Cloud Spanner database and starts to prepare it for serving. The returned long-running operation will have a name of the format `/operations/` and can be used to track preparation of the database. The metadata field type is CreateDatabaseMetadata. The response field type is Database, if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the instance that will serve the new database. Values are of the form `projects//instances/`.
    pub fn instances_databases_create(&self, request: CreateDatabaseRequest, parent: &str) -> ProjectInstanceDatabaseCreateCall<'a, S> {
        ProjectInstanceDatabaseCreateCall {
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
    /// Drops (aka deletes) a Cloud Spanner database. Completed backups for the database will be retained according to their `expire_time`. Note: Cloud Spanner might continue to accept requests for a few seconds after the database has been deleted.
    /// 
    /// # Arguments
    ///
    /// * `database` - Required. The database to be dropped.
    pub fn instances_databases_drop_database(&self, database: &str) -> ProjectInstanceDatabaseDropDatabaseCall<'a, S> {
        ProjectInstanceDatabaseDropDatabaseCall {
            hub: self.hub,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the state of a Cloud Spanner database.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the requested database. Values are of the form `projects//instances//databases/`.
    pub fn instances_databases_get(&self, name: &str) -> ProjectInstanceDatabaseGetCall<'a, S> {
        ProjectInstanceDatabaseGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the schema of a Cloud Spanner database as a list of formatted DDL statements. This method does not show pending schema updates, those may be queried using the Operations API.
    /// 
    /// # Arguments
    ///
    /// * `database` - Required. The database whose schema we wish to get. Values are of the form `projects//instances//databases/`
    pub fn instances_databases_get_ddl(&self, database: &str) -> ProjectInstanceDatabaseGetDdlCall<'a, S> {
        ProjectInstanceDatabaseGetDdlCall {
            hub: self.hub,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a database or backup resource. Returns an empty policy if a database or backup exists but does not have a policy set. Authorization requires `spanner.databases.getIamPolicy` permission on resource. For backups, authorization requires `spanner.backups.getIamPolicy` permission on resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which the policy is being retrieved. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_databases_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectInstanceDatabaseGetIamPolicyCall<'a, S> {
        ProjectInstanceDatabaseGetIamPolicyCall {
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
    /// Request a specific scan with Database-specific data for Cloud Key Visualizer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The unique name of the scan containing the requested information, specific to the Database service implementing this interface.
    pub fn instances_databases_get_scans(&self, name: &str) -> ProjectInstanceDatabaseGetScanCall<'a, S> {
        ProjectInstanceDatabaseGetScanCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _start_time: Default::default(),
            _end_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Cloud Spanner databases.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance whose databases should be listed. Values are of the form `projects//instances/`.
    pub fn instances_databases_list(&self, parent: &str) -> ProjectInstanceDatabaseListCall<'a, S> {
        ProjectInstanceDatabaseListCall {
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
    /// Updates a Cloud Spanner database. The returned long-running operation can be used to track the progress of updating the database. If the named database does not exist, returns `NOT_FOUND`. While the operation is pending: * The database's reconciling field is set to true. * Cancelling the operation is best-effort. If the cancellation succeeds, the operation metadata's cancel_time is set, the updates are reverted, and the operation terminates with a `CANCELLED` status. * New UpdateDatabase requests will return a `FAILED_PRECONDITION` error until the pending operation is done (returns successfully or with error). * Reading the database via the API continues to give the pre-request values. Upon completion of the returned operation: * The new values are in effect and readable via the API. * The database's reconciling field becomes false. The returned long-running operation will have a name of the format `projects//instances//databases//operations/` and can be used to track the database modification. The metadata field type is UpdateDatabaseMetadata. The response field type is Database, if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the database. Values are of the form `projects//instances//databases/`, where `` is as specified in the `CREATE DATABASE` statement. This name can be passed to other API methods to identify the database.
    pub fn instances_databases_patch(&self, request: Database, name: &str) -> ProjectInstanceDatabasePatchCall<'a, S> {
        ProjectInstanceDatabasePatchCall {
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
    /// Create a new database by restoring from a completed backup. The new database must be in the same project and in an instance with the same instance configuration as the instance containing the backup. The returned database long-running operation has a name of the format `projects//instances//databases//operations/`, and can be used to track the progress of the operation, and to cancel it. The metadata field type is RestoreDatabaseMetadata. The response type is Database, if successful. Cancelling the returned operation will stop the restore and delete the database. There can be only one database being restored into an instance at a time. Once the restore operation completes, a new restore operation can be initiated, without waiting for the optimize operation associated with the first restore to complete.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the instance in which to create the restored database. This instance must be in the same project and have the same instance configuration as the instance containing the source backup. Values are of the form `projects//instances/`.
    pub fn instances_databases_restore(&self, request: RestoreDatabaseRequest, parent: &str) -> ProjectInstanceDatabaseRestoreCall<'a, S> {
        ProjectInstanceDatabaseRestoreCall {
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
    /// Sets the access control policy on a database or backup resource. Replaces any existing policy. Authorization requires `spanner.databases.setIamPolicy` permission on resource. For backups, authorization requires `spanner.backups.setIamPolicy` permission on resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which the policy is being set. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for databases resources.
    pub fn instances_databases_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectInstanceDatabaseSetIamPolicyCall<'a, S> {
        ProjectInstanceDatabaseSetIamPolicyCall {
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
    /// Returns permissions that the caller has on the specified database or backup resource. Attempting this RPC on a non-existent Cloud Spanner database will result in a NOT_FOUND error if the user has `spanner.databases.list` permission on the containing Cloud Spanner instance. Otherwise returns an empty set of permissions. Calling this method on a backup that does not exist will result in a NOT_FOUND error if the user has `spanner.backups.list` permission on the containing instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which permissions are being tested. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_databases_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceDatabaseTestIamPermissionCall<'a, S> {
        ProjectInstanceDatabaseTestIamPermissionCall {
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
    /// Updates the schema of a Cloud Spanner database by creating/altering/dropping tables, columns, indexes, etc. The returned long-running operation will have a name of the format `/operations/` and can be used to track execution of the schema change(s). The metadata field type is UpdateDatabaseDdlMetadata. The operation has no response.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `database` - Required. The database to update.
    pub fn instances_databases_update_ddl(&self, request: UpdateDatabaseDdlRequest, database: &str) -> ProjectInstanceDatabaseUpdateDdlCall<'a, S> {
        ProjectInstanceDatabaseUpdateDdlCall {
            hub: self.hub,
            _request: request,
            _database: database.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists instance partition long-running operations in the given instance. An instance partition operation has a name of the form `projects//instances//instancePartitions//operations/`. The long-running operation metadata field type `metadata.type_url` describes the type of the metadata. Operations returned include those that have completed/failed/canceled within the last 7 days, and pending operations. Operations returned are ordered by `operation.metadata.value.start_time` in descending order starting from the most recently started operation. Authorization requires `spanner.instancePartitionOperations.list` permission on the resource parent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent instance of the instance partition operations. Values are of the form `projects//instances/`.
    pub fn instances_instance_partition_operations_list(&self, parent: &str) -> ProjectInstanceInstancePartitionOperationListCall<'a, S> {
        ProjectInstanceInstancePartitionOperationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _instance_partition_deadline: Default::default(),
            _filter: Default::default(),
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
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn instances_instance_partitions_operations_cancel(&self, name: &str) -> ProjectInstanceInstancePartitionOperationCancelCall<'a, S> {
        ProjectInstanceInstancePartitionOperationCancelCall {
            hub: self.hub,
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
    pub fn instances_instance_partitions_operations_delete(&self, name: &str) -> ProjectInstanceInstancePartitionOperationDeleteCall<'a, S> {
        ProjectInstanceInstancePartitionOperationDeleteCall {
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
    pub fn instances_instance_partitions_operations_get(&self, name: &str) -> ProjectInstanceInstancePartitionOperationGetCall<'a, S> {
        ProjectInstanceInstancePartitionOperationGetCall {
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
    pub fn instances_instance_partitions_operations_list(&self, name: &str) -> ProjectInstanceInstancePartitionOperationListCall1<'a, S> {
        ProjectInstanceInstancePartitionOperationListCall1 {
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
    /// Creates an instance partition and begins preparing it to be used. The returned long-running operation can be used to track the progress of preparing the new instance partition. The instance partition name is assigned by the caller. If the named instance partition already exists, `CreateInstancePartition` returns `ALREADY_EXISTS`. Immediately upon completion of this request: * The instance partition is readable via the API, with all requested attributes but no allocated resources. Its state is `CREATING`. Until completion of the returned operation: * Cancelling the operation renders the instance partition immediately unreadable via the API. * The instance partition can be deleted. * All other attempts to modify the instance partition are rejected. Upon completion of the returned operation: * Billing for all successfully-allocated resources begins (some types may have lower than the requested levels). * Databases can start using this instance partition. * The instance partition's allocated resource levels are readable via the API. * The instance partition's state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance partition. The metadata field type is CreateInstancePartitionMetadata. The response field type is InstancePartition, if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the instance in which to create the instance partition. Values are of the form `projects//instances/`.
    pub fn instances_instance_partitions_create(&self, request: CreateInstancePartitionRequest, parent: &str) -> ProjectInstanceInstancePartitionCreateCall<'a, S> {
        ProjectInstanceInstancePartitionCreateCall {
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
    /// Deletes an existing instance partition. Requires that the instance partition is not used by any database or backup and is not the default instance partition of an instance. Authorization requires `spanner.instancePartitions.delete` permission on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the instance partition to be deleted. Values are of the form `projects/{project}/instances/{instance}/instancePartitions/{instance_partition}`
    pub fn instances_instance_partitions_delete(&self, name: &str) -> ProjectInstanceInstancePartitionDeleteCall<'a, S> {
        ProjectInstanceInstancePartitionDeleteCall {
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
    /// Gets information about a particular instance partition.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the requested instance partition. Values are of the form `projects/{project}/instances/{instance}/instancePartitions/{instance_partition}`.
    pub fn instances_instance_partitions_get(&self, name: &str) -> ProjectInstanceInstancePartitionGetCall<'a, S> {
        ProjectInstanceInstancePartitionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all instance partitions for the given instance.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The instance whose instance partitions should be listed. Values are of the form `projects//instances/`.
    pub fn instances_instance_partitions_list(&self, parent: &str) -> ProjectInstanceInstancePartitionListCall<'a, S> {
        ProjectInstanceInstancePartitionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _instance_partition_deadline: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an instance partition, and begins allocating or releasing resources as requested. The returned long-running operation can be used to track the progress of updating the instance partition. If the named instance partition does not exist, returns `NOT_FOUND`. Immediately upon completion of this request: * For resource types for which a decrease in the instance partition's allocation has been requested, billing is based on the newly-requested level. Until completion of the returned operation: * Cancelling the operation sets its metadata's cancel_time, and begins restoring resources to their pre-request values. The operation is guaranteed to succeed at undoing all resource changes, after which point it terminates with a `CANCELLED` status. * All other attempts to modify the instance partition are rejected. * Reading the instance partition via the API continues to give the pre-request resource levels. Upon completion of the returned operation: * Billing begins for all successfully-allocated resources (some types may have lower than the requested levels). * All newly-reserved resources are available for serving the instance partition's tables. * The instance partition's new resource levels are readable via the API. The returned long-running operation will have a name of the format `/operations/` and can be used to track the instance partition modification. The metadata field type is UpdateInstancePartitionMetadata. The response field type is InstancePartition, if successful. Authorization requires `spanner.instancePartitions.update` permission on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. A unique identifier for the instance partition. Values are of the form `projects//instances//instancePartitions/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length. An instance partition's name cannot be changed after the instance partition is created.
    pub fn instances_instance_partitions_patch(&self, request: UpdateInstancePartitionRequest, name: &str) -> ProjectInstanceInstancePartitionPatchCall<'a, S> {
        ProjectInstanceInstancePartitionPatchCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn instances_operations_cancel(&self, name: &str) -> ProjectInstanceOperationCancelCall<'a, S> {
        ProjectInstanceOperationCancelCall {
            hub: self.hub,
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
    pub fn instances_operations_delete(&self, name: &str) -> ProjectInstanceOperationDeleteCall<'a, S> {
        ProjectInstanceOperationDeleteCall {
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
    pub fn instances_operations_get(&self, name: &str) -> ProjectInstanceOperationGetCall<'a, S> {
        ProjectInstanceOperationGetCall {
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
    pub fn instances_operations_list(&self, name: &str) -> ProjectInstanceOperationListCall<'a, S> {
        ProjectInstanceOperationListCall {
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
    /// Creates an instance and begins preparing it to begin serving. The returned long-running operation can be used to track the progress of preparing the new instance. The instance name is assigned by the caller. If the named instance already exists, `CreateInstance` returns `ALREADY_EXISTS`. Immediately upon completion of this request: * The instance is readable via the API, with all requested attributes but no allocated resources. Its state is `CREATING`. Until completion of the returned operation: * Cancelling the operation renders the instance immediately unreadable via the API. * The instance can be deleted. * All other attempts to modify the instance are rejected. Upon completion of the returned operation: * Billing for all successfully-allocated resources begins (some types may have lower than the requested levels). * Databases can be created in the instance. * The instance's allocated resource levels are readable via the API. * The instance's state becomes `READY`. The returned long-running operation will have a name of the format `/operations/` and can be used to track creation of the instance. The metadata field type is CreateInstanceMetadata. The response field type is Instance, if successful.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which to create the instance. Values are of the form `projects/`.
    pub fn instances_create(&self, request: CreateInstanceRequest, parent: &str) -> ProjectInstanceCreateCall<'a, S> {
        ProjectInstanceCreateCall {
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
    /// Deletes an instance. Immediately upon completion of the request: * Billing ceases for all of the instance's reserved resources. Soon afterward: * The instance and *all of its databases* immediately and irrevocably disappear from the API. All data in the databases is permanently deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the instance to be deleted. Values are of the form `projects//instances/`
    pub fn instances_delete(&self, name: &str) -> ProjectInstanceDeleteCall<'a, S> {
        ProjectInstanceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a particular instance.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the requested instance. Values are of the form `projects//instances/`.
    pub fn instances_get(&self, name: &str) -> ProjectInstanceGetCall<'a, S> {
        ProjectInstanceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _field_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for an instance resource. Returns an empty policy if an instance exists but does not have a policy set. Authorization requires `spanner.instances.getIamPolicy` on resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which the policy is being retrieved. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> ProjectInstanceGetIamPolicyCall<'a, S> {
        ProjectInstanceGetIamPolicyCall {
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
    /// Lists all instances in the given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project for which a list of instances is requested. Values are of the form `projects/`.
    pub fn instances_list(&self, parent: &str) -> ProjectInstanceListCall<'a, S> {
        ProjectInstanceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _instance_deadline: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves the instance to the target instance config. The returned long-running operation can be used to track the progress of moving the instance. `MoveInstance` returns `FAILED_PRECONDITION` if the instance meets any of the following criteria: * Has an ongoing move to a different instance config * Has backups * Has an ongoing update * Is under free trial * Contains any CMEK-enabled databases While the operation is pending: * All other attempts to modify the instance, including changes to its compute capacity, are rejected. * The following database and backup admin operations are rejected: * DatabaseAdmin.CreateDatabase, * DatabaseAdmin.UpdateDatabaseDdl (Disabled if default_leader is specified in the request.) * DatabaseAdmin.RestoreDatabase * DatabaseAdmin.CreateBackup * DatabaseAdmin.CopyBackup * Both the source and target instance configs are subject to hourly compute and storage charges. * The instance may experience higher read-write latencies and a higher transaction abort rate. However, moving an instance does not cause any downtime. The returned long-running operation will have a name of the format `/operations/` and can be used to track the move instance operation. The metadata field type is MoveInstanceMetadata. The response field type is Instance, if successful. Cancelling the operation sets its metadata's cancel_time. Cancellation is not immediate since it involves moving any data previously moved to target instance config back to the original instance config. The same operation can be used to track the progress of the cancellation. Upon successful completion of the cancellation, the operation terminates with CANCELLED status. Upon completion(if not cancelled) of the returned operation: * Instance would be successfully moved to the target instance config. * You are billed for compute and storage in target instance config. Authorization requires `spanner.instances.update` permission on the resource instance. For more details, please see [documentation](https://cloud.google.com/spanner/docs/move-instance).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The instance to move. Values are of the form `projects//instances/`.
    pub fn instances_move(&self, request: MoveInstanceRequest, name: &str) -> ProjectInstanceMoveCall<'a, S> {
        ProjectInstanceMoveCall {
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
    /// Updates an instance, and begins allocating or releasing resources as requested. The returned long-running operation can be used to track the progress of updating the instance. If the named instance does not exist, returns `NOT_FOUND`. Immediately upon completion of this request: * For resource types for which a decrease in the instance's allocation has been requested, billing is based on the newly-requested level. Until completion of the returned operation: * Cancelling the operation sets its metadata's cancel_time, and begins restoring resources to their pre-request values. The operation is guaranteed to succeed at undoing all resource changes, after which point it terminates with a `CANCELLED` status. * All other attempts to modify the instance are rejected. * Reading the instance via the API continues to give the pre-request resource levels. Upon completion of the returned operation: * Billing begins for all successfully-allocated resources (some types may have lower than the requested levels). * All newly-reserved resources are available for serving the instance's tables. * The instance's new resource levels are readable via the API. The returned long-running operation will have a name of the format `/operations/` and can be used to track the instance modification. The metadata field type is UpdateInstanceMetadata. The response field type is Instance, if successful. Authorization requires `spanner.instances.update` permission on the resource name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. A unique identifier for the instance, which cannot be changed after the instance is created. Values are of the form `projects//instances/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length.
    pub fn instances_patch(&self, request: UpdateInstanceRequest, name: &str) -> ProjectInstancePatchCall<'a, S> {
        ProjectInstancePatchCall {
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
    /// Sets the access control policy on an instance resource. Replaces any existing policy. Authorization requires `spanner.instances.setIamPolicy` on resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which the policy is being set. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for databases resources.
    pub fn instances_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectInstanceSetIamPolicyCall<'a, S> {
        ProjectInstanceSetIamPolicyCall {
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
    /// Returns permissions that the caller has on the specified instance resource. Attempting this RPC on a non-existent Cloud Spanner instance resource will result in a NOT_FOUND error if the user has `spanner.instances.list` permission on the containing Google Cloud Project. Otherwise returns an empty set of permissions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The Cloud Spanner resource for which permissions are being tested. The format is `projects//instances/` for instance resources and `projects//instances//databases/` for database resources.
    pub fn instances_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectInstanceTestIamPermissionCall<'a, S> {
        ProjectInstanceTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *scan* resources.
/// It is not used directly, but through the [`Spanner`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_spanner1 as spanner1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use spanner1::{Spanner, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Spanner::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.scans();
/// # }
/// ```
pub struct ScanMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Spanner<S>,
}

impl<'a, S> client::MethodsBuilder for ScanMethods<'a, S> {}

impl<'a, S> ScanMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return available scans given a Database-specific resource name.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The unique name of the parent resource, specific to the Database service implementing this interface.
    pub fn list(&self, parent: &str) -> ScanListCall<'a, S> {
        ScanListCall {
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
}



