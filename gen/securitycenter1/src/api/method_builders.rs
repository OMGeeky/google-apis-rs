use super::*;
/// A builder providing access to all methods supported on *folder* resources.
/// It is not used directly, but through the [`SecurityCommandCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_securitycenter1 as securitycenter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_group(...)`, `assets_list(...)`, `assets_update_security_marks(...)`, `big_query_exports_create(...)`, `big_query_exports_delete(...)`, `big_query_exports_get(...)`, `big_query_exports_list(...)`, `big_query_exports_patch(...)`, `event_threat_detection_settings_custom_modules_create(...)`, `event_threat_detection_settings_custom_modules_delete(...)`, `event_threat_detection_settings_custom_modules_get(...)`, `event_threat_detection_settings_custom_modules_list(...)`, `event_threat_detection_settings_custom_modules_list_descendant(...)`, `event_threat_detection_settings_custom_modules_patch(...)`, `event_threat_detection_settings_effective_custom_modules_get(...)`, `event_threat_detection_settings_effective_custom_modules_list(...)`, `event_threat_detection_settings_validate_custom_module(...)`, `findings_bulk_mute(...)`, `locations_mute_configs_delete(...)`, `locations_mute_configs_get(...)`, `locations_mute_configs_patch(...)`, `mute_configs_create(...)`, `mute_configs_delete(...)`, `mute_configs_get(...)`, `mute_configs_list(...)`, `mute_configs_patch(...)`, `notification_configs_create(...)`, `notification_configs_delete(...)`, `notification_configs_get(...)`, `notification_configs_list(...)`, `notification_configs_patch(...)`, `security_health_analytics_settings_custom_modules_create(...)`, `security_health_analytics_settings_custom_modules_delete(...)`, `security_health_analytics_settings_custom_modules_get(...)`, `security_health_analytics_settings_custom_modules_list(...)`, `security_health_analytics_settings_custom_modules_list_descendant(...)`, `security_health_analytics_settings_custom_modules_patch(...)`, `security_health_analytics_settings_custom_modules_simulate(...)`, `security_health_analytics_settings_effective_custom_modules_get(...)`, `security_health_analytics_settings_effective_custom_modules_list(...)`, `sources_findings_external_systems_patch(...)`, `sources_findings_group(...)`, `sources_findings_list(...)`, `sources_findings_patch(...)`, `sources_findings_set_mute(...)`, `sources_findings_set_state(...)`, `sources_findings_update_security_marks(...)` and `sources_list(...)`
/// // to build up your call.
/// let rb = hub.folders();
/// # }
/// ```
pub struct FolderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecurityCommandCenter<S>,
}

impl<'a, S> client::MethodsBuilder for FolderMethods<'a, S> {}

impl<'a, S> FolderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization's assets and groups them by their specified properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent to group the assets by. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_group(&self, request: GroupAssetsRequest, parent: &str) -> FolderAssetGroupCall<'a, S> {
        FolderAssetGroupCall {
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
    /// Lists an organization's assets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource that contains the assets. The value that you can specify on parent depends on the method in which you specify parent. You can specify one of the following values: "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_list(&self, parent: &str) -> FolderAssetListCall<'a, S> {
        FolderAssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn assets_update_security_marks(&self, request: SecurityMarks, name: &str) -> FolderAssetUpdateSecurityMarkCall<'a, S> {
        FolderAssetUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource of the new BigQuery export. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn big_query_exports_create(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, parent: &str) -> FolderBigQueryExportCreateCall<'a, S> {
        FolderBigQueryExportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _big_query_export_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the BigQuery export to delete. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_delete(&self, name: &str) -> FolderBigQueryExportDeleteCall<'a, S> {
        FolderBigQueryExportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the BigQuery export to retrieve. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_get(&self, name: &str) -> FolderBigQueryExportGetCall<'a, S> {
        FolderBigQueryExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists BigQuery exports. Note that when requesting BigQuery exports at a given level all exports under that level are also returned e.g. if requesting BigQuery exports under a folder, then all BigQuery exports immediately under the folder plus the ones created under the projects within the folder are returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of BigQuery exports. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn big_query_exports_list(&self, parent: &str) -> FolderBigQueryExportListCall<'a, S> {
        FolderBigQueryExportListCall {
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
    /// Updates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    pub fn big_query_exports_patch(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, name: &str) -> FolderBigQueryExportPatchCall<'a, S> {
        FolderBigQueryExportPatchCall {
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
    /// Creates a resident Event Threat Detection custom module at the scope of the given Resource Manager parent, and also creates inherited custom modules for all descendants of the given parent. These modules are enabled by default.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The new custom module's parent. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_create(&self, request: EventThreatDetectionCustomModule, parent: &str) -> FolderEventThreatDetectionSettingCustomModuleCreateCall<'a, S> {
        FolderEventThreatDetectionSettingCustomModuleCreateCall {
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
    /// Deletes the specified Event Threat Detection custom module and all of its descendants in the Resource Manager hierarchy. This method is only supported for resident custom modules.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to delete. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_delete(&self, name: &str) -> FolderEventThreatDetectionSettingCustomModuleDeleteCall<'a, S> {
        FolderEventThreatDetectionSettingCustomModuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an Event Threat Detection custom module.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to get. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_get(&self, name: &str) -> FolderEventThreatDetectionSettingCustomModuleGetCall<'a, S> {
        FolderEventThreatDetectionSettingCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Event Threat Detection custom modules for the given Resource Manager parent. This includes resident modules defined at the scope of the parent along with modules inherited from ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_list(&self, parent: &str) -> FolderEventThreatDetectionSettingCustomModuleListCall<'a, S> {
        FolderEventThreatDetectionSettingCustomModuleListCall {
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
    /// Lists all resident Event Threat Detection custom modules under the given Resource Manager parent and its descendants.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_list_descendant(&self, parent: &str) -> FolderEventThreatDetectionSettingCustomModuleListDescendantCall<'a, S> {
        FolderEventThreatDetectionSettingCustomModuleListDescendantCall {
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
    /// Updates the Event Threat Detection custom module with the given name based on the given update mask. Updating the enablement state is supported for both resident and inherited modules (though resident modules cannot have an enablement state of "inherited"). Updating the display name or configuration of a module is supported for resident modules only. The type of a module cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the Event Threat Detection custom module. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_patch(&self, request: EventThreatDetectionCustomModule, name: &str) -> FolderEventThreatDetectionSettingCustomModulePatchCall<'a, S> {
        FolderEventThreatDetectionSettingCustomModulePatchCall {
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
    /// Gets an effective Event Threat Detection custom module at the given level.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the effective Event Threat Detection custom module. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/effectiveCustomModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/effectiveCustomModules/{module}". * "projects/{project}/eventThreatDetectionSettings/effectiveCustomModules/{module}".
    pub fn event_threat_detection_settings_effective_custom_modules_get(&self, name: &str) -> FolderEventThreatDetectionSettingEffectiveCustomModuleGetCall<'a, S> {
        FolderEventThreatDetectionSettingEffectiveCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all effective Event Threat Detection custom modules for the given parent. This includes resident modules defined at the scope of the parent along with modules inherited from its ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules for. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_effective_custom_modules_list(&self, parent: &str) -> FolderEventThreatDetectionSettingEffectiveCustomModuleListCall<'a, S> {
        FolderEventThreatDetectionSettingEffectiveCustomModuleListCall {
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
    /// Validates the given Event Threat Detection custom module.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the parent to validate the Custom Module under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_validate_custom_module(&self, request: ValidateEventThreatDetectionCustomModuleRequest, parent: &str) -> FolderEventThreatDetectionSettingValidateCustomModuleCall<'a, S> {
        FolderEventThreatDetectionSettingValidateCustomModuleCall {
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
    /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The parent can be either an organization, folder or project. The findings matched by the filter will be muted after the LRO is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, at which bulk action needs to be applied. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn findings_bulk_mute(&self, request: BulkMuteFindingsRequest, parent: &str) -> FolderFindingBulkMuteCall<'a, S> {
        FolderFindingBulkMuteCall {
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
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn locations_mute_configs_delete(&self, name: &str) -> FolderLocationMuteConfigDeleteCall<'a, S> {
        FolderLocationMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn locations_mute_configs_get(&self, name: &str) -> FolderLocationMuteConfigGetCall<'a, S> {
        FolderLocationMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}" "organizations/{organization}/locations/global/muteConfigs/{mute_config}" "folders/{folder}/locations/global/muteConfigs/{mute_config}" "projects/{project}/locations/global/muteConfigs/{mute_config}"
    pub fn locations_mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> FolderLocationMuteConfigPatchCall<'a, S> {
        FolderLocationMuteConfigPatchCall {
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
    /// Creates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new mute configs's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn mute_configs_create(&self, request: GoogleCloudSecuritycenterV1MuteConfig, parent: &str) -> FolderMuteConfigCreateCall<'a, S> {
        FolderMuteConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mute_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn mute_configs_delete(&self, name: &str) -> FolderMuteConfigDeleteCall<'a, S> {
        FolderMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn mute_configs_get(&self, name: &str) -> FolderMuteConfigGetCall<'a, S> {
        FolderMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists mute configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of mute configs. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn mute_configs_list(&self, parent: &str) -> FolderMuteConfigListCall<'a, S> {
        FolderMuteConfigListCall {
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
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}" "organizations/{organization}/locations/global/muteConfigs/{mute_config}" "folders/{folder}/locations/global/muteConfigs/{mute_config}" "projects/{project}/locations/global/muteConfigs/{mute_config}"
    pub fn mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> FolderMuteConfigPatchCall<'a, S> {
        FolderMuteConfigPatchCall {
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
    /// Creates a notification config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_create(&self, request: NotificationConfig, parent: &str) -> FolderNotificationConfigCreateCall<'a, S> {
        FolderNotificationConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to delete. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_delete(&self, name: &str) -> FolderNotificationConfigDeleteCall<'a, S> {
        FolderNotificationConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to get. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_get(&self, name: &str) -> FolderNotificationConfigGetCall<'a, S> {
        FolderNotificationConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists notification configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent in which to list the notification configurations. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_list(&self, parent: &str) -> FolderNotificationConfigListCall<'a, S> {
        FolderNotificationConfigListCall {
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
    ///  Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    pub fn notification_configs_patch(&self, request: NotificationConfig, name: &str) -> FolderNotificationConfigPatchCall<'a, S> {
        FolderNotificationConfigPatchCall {
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
    /// Creates a resident SecurityHealthAnalyticsCustomModule at the scope of the given CRM parent, and also creates inherited SecurityHealthAnalyticsCustomModules for all CRM descendants of the given parent. These modules are enabled by default.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new custom module's parent. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_create(&self, request: GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModule, parent: &str) -> FolderSecurityHealthAnalyticsSettingCustomModuleCreateCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModuleCreateCall {
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
    /// Deletes the specified SecurityHealthAnalyticsCustomModule and all of its descendants in the CRM hierarchy. This method is only supported for resident custom modules.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to delete. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}"
    pub fn security_health_analytics_settings_custom_modules_delete(&self, name: &str) -> FolderSecurityHealthAnalyticsSettingCustomModuleDeleteCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a SecurityHealthAnalyticsCustomModule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to get. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}"
    pub fn security_health_analytics_settings_custom_modules_get(&self, name: &str) -> FolderSecurityHealthAnalyticsSettingCustomModuleGetCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all SecurityHealthAnalyticsCustomModules for the given parent. This includes resident modules defined at the scope of the parent, and inherited modules, inherited from CRM ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_list(&self, parent: &str) -> FolderSecurityHealthAnalyticsSettingCustomModuleListCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModuleListCall {
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
    /// Returns a list of all resident SecurityHealthAnalyticsCustomModules under the given CRM parent and all of the parent’s CRM descendants.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list descendant custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_list_descendant(&self, parent: &str) -> FolderSecurityHealthAnalyticsSettingCustomModuleListDescendantCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModuleListDescendantCall {
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
    /// Updates the SecurityHealthAnalyticsCustomModule under the given name based on the given update mask. Updating the enablement state is supported on both resident and inherited modules (though resident modules cannot have an enablement state of "inherited"). Updating the display name and custom config of a module is supported on resident modules only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the custom module. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", or "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}" The id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits.
    pub fn security_health_analytics_settings_custom_modules_patch(&self, request: GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModule, name: &str) -> FolderSecurityHealthAnalyticsSettingCustomModulePatchCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModulePatchCall {
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
    /// Simulates a given SecurityHealthAnalyticsCustomModule and Resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the organization, project, or folder. For more information about relative resource names, see [Relative Resource Name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) Example: `organizations/{organization_id}`
    pub fn security_health_analytics_settings_custom_modules_simulate(&self, request: SimulateSecurityHealthAnalyticsCustomModuleRequest, parent: &str) -> FolderSecurityHealthAnalyticsSettingCustomModuleSimulateCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingCustomModuleSimulateCall {
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
    /// Retrieves an EffectiveSecurityHealthAnalyticsCustomModule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the effective custom module to get. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}"
    pub fn security_health_analytics_settings_effective_custom_modules_get(&self, name: &str) -> FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all EffectiveSecurityHealthAnalyticsCustomModules for the given parent. This includes resident modules defined at the scope of the parent, and inherited modules, inherited from CRM ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list effective custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_effective_custom_modules_list(&self, parent: &str) -> FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall<'a, S> {
        FolderSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall {
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
    /// Updates external system. This is for a given finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    pub fn sources_findings_external_systems_patch(&self, request: GoogleCloudSecuritycenterV1ExternalSystem, name: &str) -> FolderSourceFindingExternalSystemPatchCall<'a, S> {
        FolderSourceFindingExternalSystemPatchCall {
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
    /// Filters an organization or source's findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the source to groupBy. Its format is "organizations/[organization_id]/sources/[source_id]", folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]. To groupBy across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-, or projects/{project_id}/sources/-
    pub fn sources_findings_group(&self, request: GroupFindingsRequest, parent: &str) -> FolderSourceFindingGroupCall<'a, S> {
        FolderSourceFindingGroupCall {
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
    /// Lists an organization or source's findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the source the findings belong to. Its format is "organizations/[organization_id]/sources/[source_id], folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]". To list across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or projects/{projects_id}/sources/-
    pub fn sources_findings_list(&self, parent: &str) -> FolderSourceFindingListCall<'a, S> {
        FolderSourceFindingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_patch(&self, request: Finding, name: &str) -> FolderSourceFindingPatchCall<'a, S> {
        FolderSourceFindingPatchCall {
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
    /// Updates the mute state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_mute(&self, request: SetMuteRequest, name: &str) -> FolderSourceFindingSetMuteCall<'a, S> {
        FolderSourceFindingSetMuteCall {
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
    /// Updates the state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_state(&self, request: SetFindingStateRequest, name: &str) -> FolderSourceFindingSetStateCall<'a, S> {
        FolderSourceFindingSetStateCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn sources_findings_update_security_marks(&self, request: SecurityMarks, name: &str) -> FolderSourceFindingUpdateSecurityMarkCall<'a, S> {
        FolderSourceFindingUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all sources belonging to an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent of sources to list. Its format should be "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn sources_list(&self, parent: &str) -> FolderSourceListCall<'a, S> {
        FolderSourceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`SecurityCommandCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_securitycenter1 as securitycenter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_group(...)`, `assets_list(...)`, `assets_run_discovery(...)`, `assets_update_security_marks(...)`, `big_query_exports_create(...)`, `big_query_exports_delete(...)`, `big_query_exports_get(...)`, `big_query_exports_list(...)`, `big_query_exports_patch(...)`, `event_threat_detection_settings_custom_modules_create(...)`, `event_threat_detection_settings_custom_modules_delete(...)`, `event_threat_detection_settings_custom_modules_get(...)`, `event_threat_detection_settings_custom_modules_list(...)`, `event_threat_detection_settings_custom_modules_list_descendant(...)`, `event_threat_detection_settings_custom_modules_patch(...)`, `event_threat_detection_settings_effective_custom_modules_get(...)`, `event_threat_detection_settings_effective_custom_modules_list(...)`, `event_threat_detection_settings_validate_custom_module(...)`, `findings_bulk_mute(...)`, `get_organization_settings(...)`, `locations_mute_configs_delete(...)`, `locations_mute_configs_get(...)`, `locations_mute_configs_patch(...)`, `mute_configs_create(...)`, `mute_configs_delete(...)`, `mute_configs_get(...)`, `mute_configs_list(...)`, `mute_configs_patch(...)`, `notification_configs_create(...)`, `notification_configs_delete(...)`, `notification_configs_get(...)`, `notification_configs_list(...)`, `notification_configs_patch(...)`, `operations_cancel(...)`, `operations_delete(...)`, `operations_get(...)`, `operations_list(...)`, `resource_value_configs_batch_create(...)`, `resource_value_configs_delete(...)`, `resource_value_configs_get(...)`, `resource_value_configs_list(...)`, `resource_value_configs_patch(...)`, `security_health_analytics_settings_custom_modules_create(...)`, `security_health_analytics_settings_custom_modules_delete(...)`, `security_health_analytics_settings_custom_modules_get(...)`, `security_health_analytics_settings_custom_modules_list(...)`, `security_health_analytics_settings_custom_modules_list_descendant(...)`, `security_health_analytics_settings_custom_modules_patch(...)`, `security_health_analytics_settings_custom_modules_simulate(...)`, `security_health_analytics_settings_effective_custom_modules_get(...)`, `security_health_analytics_settings_effective_custom_modules_list(...)`, `simulations_attack_exposure_results_attack_paths_list(...)`, `simulations_attack_exposure_results_valued_resources_list(...)`, `simulations_attack_paths_list(...)`, `simulations_get(...)`, `simulations_valued_resources_attack_paths_list(...)`, `simulations_valued_resources_get(...)`, `simulations_valued_resources_list(...)`, `sources_create(...)`, `sources_findings_create(...)`, `sources_findings_external_systems_patch(...)`, `sources_findings_group(...)`, `sources_findings_list(...)`, `sources_findings_patch(...)`, `sources_findings_set_mute(...)`, `sources_findings_set_state(...)`, `sources_findings_update_security_marks(...)`, `sources_get(...)`, `sources_get_iam_policy(...)`, `sources_list(...)`, `sources_patch(...)`, `sources_set_iam_policy(...)`, `sources_test_iam_permissions(...)` and `update_organization_settings(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecurityCommandCenter<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization's assets and groups them by their specified properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent to group the assets by. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_group(&self, request: GroupAssetsRequest, parent: &str) -> OrganizationAssetGroupCall<'a, S> {
        OrganizationAssetGroupCall {
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
    /// Lists an organization's assets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource that contains the assets. The value that you can specify on parent depends on the method in which you specify parent. You can specify one of the following values: "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_list(&self, parent: &str) -> OrganizationAssetListCall<'a, S> {
        OrganizationAssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs asset discovery. The discovery is tracked with a long-running operation. This API can only be called with limited frequency for an organization. If it is called too frequently the caller will receive a TOO_MANY_REQUESTS error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization to run asset discovery for. Its format is "organizations/[organization_id]".
    pub fn assets_run_discovery(&self, request: RunAssetDiscoveryRequest, parent: &str) -> OrganizationAssetRunDiscoveryCall<'a, S> {
        OrganizationAssetRunDiscoveryCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn assets_update_security_marks(&self, request: SecurityMarks, name: &str) -> OrganizationAssetUpdateSecurityMarkCall<'a, S> {
        OrganizationAssetUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource of the new BigQuery export. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn big_query_exports_create(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, parent: &str) -> OrganizationBigQueryExportCreateCall<'a, S> {
        OrganizationBigQueryExportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _big_query_export_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the BigQuery export to delete. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_delete(&self, name: &str) -> OrganizationBigQueryExportDeleteCall<'a, S> {
        OrganizationBigQueryExportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the BigQuery export to retrieve. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_get(&self, name: &str) -> OrganizationBigQueryExportGetCall<'a, S> {
        OrganizationBigQueryExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists BigQuery exports. Note that when requesting BigQuery exports at a given level all exports under that level are also returned e.g. if requesting BigQuery exports under a folder, then all BigQuery exports immediately under the folder plus the ones created under the projects within the folder are returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of BigQuery exports. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn big_query_exports_list(&self, parent: &str) -> OrganizationBigQueryExportListCall<'a, S> {
        OrganizationBigQueryExportListCall {
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
    /// Updates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    pub fn big_query_exports_patch(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, name: &str) -> OrganizationBigQueryExportPatchCall<'a, S> {
        OrganizationBigQueryExportPatchCall {
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
    /// Creates a resident Event Threat Detection custom module at the scope of the given Resource Manager parent, and also creates inherited custom modules for all descendants of the given parent. These modules are enabled by default.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The new custom module's parent. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_create(&self, request: EventThreatDetectionCustomModule, parent: &str) -> OrganizationEventThreatDetectionSettingCustomModuleCreateCall<'a, S> {
        OrganizationEventThreatDetectionSettingCustomModuleCreateCall {
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
    /// Deletes the specified Event Threat Detection custom module and all of its descendants in the Resource Manager hierarchy. This method is only supported for resident custom modules.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to delete. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_delete(&self, name: &str) -> OrganizationEventThreatDetectionSettingCustomModuleDeleteCall<'a, S> {
        OrganizationEventThreatDetectionSettingCustomModuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an Event Threat Detection custom module.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to get. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_get(&self, name: &str) -> OrganizationEventThreatDetectionSettingCustomModuleGetCall<'a, S> {
        OrganizationEventThreatDetectionSettingCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Event Threat Detection custom modules for the given Resource Manager parent. This includes resident modules defined at the scope of the parent along with modules inherited from ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_list(&self, parent: &str) -> OrganizationEventThreatDetectionSettingCustomModuleListCall<'a, S> {
        OrganizationEventThreatDetectionSettingCustomModuleListCall {
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
    /// Lists all resident Event Threat Detection custom modules under the given Resource Manager parent and its descendants.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_list_descendant(&self, parent: &str) -> OrganizationEventThreatDetectionSettingCustomModuleListDescendantCall<'a, S> {
        OrganizationEventThreatDetectionSettingCustomModuleListDescendantCall {
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
    /// Updates the Event Threat Detection custom module with the given name based on the given update mask. Updating the enablement state is supported for both resident and inherited modules (though resident modules cannot have an enablement state of "inherited"). Updating the display name or configuration of a module is supported for resident modules only. The type of a module cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the Event Threat Detection custom module. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_patch(&self, request: EventThreatDetectionCustomModule, name: &str) -> OrganizationEventThreatDetectionSettingCustomModulePatchCall<'a, S> {
        OrganizationEventThreatDetectionSettingCustomModulePatchCall {
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
    /// Gets an effective Event Threat Detection custom module at the given level.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the effective Event Threat Detection custom module. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/effectiveCustomModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/effectiveCustomModules/{module}". * "projects/{project}/eventThreatDetectionSettings/effectiveCustomModules/{module}".
    pub fn event_threat_detection_settings_effective_custom_modules_get(&self, name: &str) -> OrganizationEventThreatDetectionSettingEffectiveCustomModuleGetCall<'a, S> {
        OrganizationEventThreatDetectionSettingEffectiveCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all effective Event Threat Detection custom modules for the given parent. This includes resident modules defined at the scope of the parent along with modules inherited from its ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules for. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_effective_custom_modules_list(&self, parent: &str) -> OrganizationEventThreatDetectionSettingEffectiveCustomModuleListCall<'a, S> {
        OrganizationEventThreatDetectionSettingEffectiveCustomModuleListCall {
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
    /// Validates the given Event Threat Detection custom module.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the parent to validate the Custom Module under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_validate_custom_module(&self, request: ValidateEventThreatDetectionCustomModuleRequest, parent: &str) -> OrganizationEventThreatDetectionSettingValidateCustomModuleCall<'a, S> {
        OrganizationEventThreatDetectionSettingValidateCustomModuleCall {
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
    /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The parent can be either an organization, folder or project. The findings matched by the filter will be muted after the LRO is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, at which bulk action needs to be applied. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn findings_bulk_mute(&self, request: BulkMuteFindingsRequest, parent: &str) -> OrganizationFindingBulkMuteCall<'a, S> {
        OrganizationFindingBulkMuteCall {
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
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn locations_mute_configs_delete(&self, name: &str) -> OrganizationLocationMuteConfigDeleteCall<'a, S> {
        OrganizationLocationMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn locations_mute_configs_get(&self, name: &str) -> OrganizationLocationMuteConfigGetCall<'a, S> {
        OrganizationLocationMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}" "organizations/{organization}/locations/global/muteConfigs/{mute_config}" "folders/{folder}/locations/global/muteConfigs/{mute_config}" "projects/{project}/locations/global/muteConfigs/{mute_config}"
    pub fn locations_mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> OrganizationLocationMuteConfigPatchCall<'a, S> {
        OrganizationLocationMuteConfigPatchCall {
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
    /// Creates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new mute configs's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn mute_configs_create(&self, request: GoogleCloudSecuritycenterV1MuteConfig, parent: &str) -> OrganizationMuteConfigCreateCall<'a, S> {
        OrganizationMuteConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mute_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn mute_configs_delete(&self, name: &str) -> OrganizationMuteConfigDeleteCall<'a, S> {
        OrganizationMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn mute_configs_get(&self, name: &str) -> OrganizationMuteConfigGetCall<'a, S> {
        OrganizationMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists mute configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of mute configs. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn mute_configs_list(&self, parent: &str) -> OrganizationMuteConfigListCall<'a, S> {
        OrganizationMuteConfigListCall {
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
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}" "organizations/{organization}/locations/global/muteConfigs/{mute_config}" "folders/{folder}/locations/global/muteConfigs/{mute_config}" "projects/{project}/locations/global/muteConfigs/{mute_config}"
    pub fn mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> OrganizationMuteConfigPatchCall<'a, S> {
        OrganizationMuteConfigPatchCall {
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
    /// Creates a notification config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_create(&self, request: NotificationConfig, parent: &str) -> OrganizationNotificationConfigCreateCall<'a, S> {
        OrganizationNotificationConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to delete. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_delete(&self, name: &str) -> OrganizationNotificationConfigDeleteCall<'a, S> {
        OrganizationNotificationConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to get. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_get(&self, name: &str) -> OrganizationNotificationConfigGetCall<'a, S> {
        OrganizationNotificationConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists notification configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent in which to list the notification configurations. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_list(&self, parent: &str) -> OrganizationNotificationConfigListCall<'a, S> {
        OrganizationNotificationConfigListCall {
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
    ///  Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    pub fn notification_configs_patch(&self, request: NotificationConfig, name: &str) -> OrganizationNotificationConfigPatchCall<'a, S> {
        OrganizationNotificationConfigPatchCall {
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
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn operations_cancel(&self, name: &str) -> OrganizationOperationCancelCall<'a, S> {
        OrganizationOperationCancelCall {
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
    pub fn operations_delete(&self, name: &str) -> OrganizationOperationDeleteCall<'a, S> {
        OrganizationOperationDeleteCall {
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
    pub fn operations_get(&self, name: &str) -> OrganizationOperationGetCall<'a, S> {
        OrganizationOperationGetCall {
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
    pub fn operations_list(&self, name: &str) -> OrganizationOperationListCall<'a, S> {
        OrganizationOperationListCall {
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
    /// Creates a ResourceValueConfig for an organization. Maps user's tags to difference resource values for use by the attack path simulation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new ResourceValueConfig's parent. The parent field in the CreateResourceValueConfigRequest messages must either be empty or match this field.
    pub fn resource_value_configs_batch_create(&self, request: BatchCreateResourceValueConfigsRequest, parent: &str) -> OrganizationResourceValueConfigBatchCreateCall<'a, S> {
        OrganizationResourceValueConfigBatchCreateCall {
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
    /// Deletes a ResourceValueConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the ResourceValueConfig to delete
    pub fn resource_value_configs_delete(&self, name: &str) -> OrganizationResourceValueConfigDeleteCall<'a, S> {
        OrganizationResourceValueConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a ResourceValueConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource value config to retrieve. Its format is organizations/{organization}/resourceValueConfigs/{config_id}.
    pub fn resource_value_configs_get(&self, name: &str) -> OrganizationResourceValueConfigGetCall<'a, S> {
        OrganizationResourceValueConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all ResourceValueConfigs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of resource value configs. Its format is "organizations/[organization_id]"
    pub fn resource_value_configs_list(&self, parent: &str) -> OrganizationResourceValueConfigListCall<'a, S> {
        OrganizationResourceValueConfigListCall {
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
    /// Updates an existing ResourceValueConfigs with new rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name for the resource value config
    pub fn resource_value_configs_patch(&self, request: GoogleCloudSecuritycenterV1ResourceValueConfig, name: &str) -> OrganizationResourceValueConfigPatchCall<'a, S> {
        OrganizationResourceValueConfigPatchCall {
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
    /// Creates a resident SecurityHealthAnalyticsCustomModule at the scope of the given CRM parent, and also creates inherited SecurityHealthAnalyticsCustomModules for all CRM descendants of the given parent. These modules are enabled by default.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new custom module's parent. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_create(&self, request: GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModule, parent: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModuleCreateCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModuleCreateCall {
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
    /// Deletes the specified SecurityHealthAnalyticsCustomModule and all of its descendants in the CRM hierarchy. This method is only supported for resident custom modules.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to delete. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}"
    pub fn security_health_analytics_settings_custom_modules_delete(&self, name: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModuleDeleteCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a SecurityHealthAnalyticsCustomModule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to get. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}"
    pub fn security_health_analytics_settings_custom_modules_get(&self, name: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModuleGetCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all SecurityHealthAnalyticsCustomModules for the given parent. This includes resident modules defined at the scope of the parent, and inherited modules, inherited from CRM ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_list(&self, parent: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModuleListCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModuleListCall {
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
    /// Returns a list of all resident SecurityHealthAnalyticsCustomModules under the given CRM parent and all of the parent’s CRM descendants.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list descendant custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_list_descendant(&self, parent: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModuleListDescendantCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModuleListDescendantCall {
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
    /// Updates the SecurityHealthAnalyticsCustomModule under the given name based on the given update mask. Updating the enablement state is supported on both resident and inherited modules (though resident modules cannot have an enablement state of "inherited"). Updating the display name and custom config of a module is supported on resident modules only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the custom module. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", or "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}" The id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits.
    pub fn security_health_analytics_settings_custom_modules_patch(&self, request: GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModule, name: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModulePatchCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModulePatchCall {
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
    /// Simulates a given SecurityHealthAnalyticsCustomModule and Resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the organization, project, or folder. For more information about relative resource names, see [Relative Resource Name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) Example: `organizations/{organization_id}`
    pub fn security_health_analytics_settings_custom_modules_simulate(&self, request: SimulateSecurityHealthAnalyticsCustomModuleRequest, parent: &str) -> OrganizationSecurityHealthAnalyticsSettingCustomModuleSimulateCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingCustomModuleSimulateCall {
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
    /// Retrieves an EffectiveSecurityHealthAnalyticsCustomModule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the effective custom module to get. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}"
    pub fn security_health_analytics_settings_effective_custom_modules_get(&self, name: &str) -> OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all EffectiveSecurityHealthAnalyticsCustomModules for the given parent. This includes resident modules defined at the scope of the parent, and inherited modules, inherited from CRM ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list effective custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_effective_custom_modules_list(&self, parent: &str) -> OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall<'a, S> {
        OrganizationSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall {
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
    /// Lists the attack paths for a set of simulation results or valued resources and filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list attack paths. Valid formats: "organizations/{organization}", "organizations/{organization}/simulations/{simulation}" "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}" "organizations/{organization}/simulations/{simulation}/valuedResources/{valued_resource}"
    pub fn simulations_attack_exposure_results_attack_paths_list(&self, parent: &str) -> OrganizationSimulationAttackExposureResultAttackPathListCall<'a, S> {
        OrganizationSimulationAttackExposureResultAttackPathListCall {
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
    /// Lists the valued resources for a set of simulation results and filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list valued resources. Valid formats: "organizations/{organization}", "organizations/{organization}/simulations/{simulation}" "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}"
    pub fn simulations_attack_exposure_results_valued_resources_list(&self, parent: &str) -> OrganizationSimulationAttackExposureResultValuedResourceListCall<'a, S> {
        OrganizationSimulationAttackExposureResultValuedResourceListCall {
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
    /// Lists the attack paths for a set of simulation results or valued resources and filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list attack paths. Valid formats: "organizations/{organization}", "organizations/{organization}/simulations/{simulation}" "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}" "organizations/{organization}/simulations/{simulation}/valuedResources/{valued_resource}"
    pub fn simulations_attack_paths_list(&self, parent: &str) -> OrganizationSimulationAttackPathListCall<'a, S> {
        OrganizationSimulationAttackPathListCall {
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
    /// Lists the attack paths for a set of simulation results or valued resources and filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list attack paths. Valid formats: "organizations/{organization}", "organizations/{organization}/simulations/{simulation}" "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}" "organizations/{organization}/simulations/{simulation}/valuedResources/{valued_resource}"
    pub fn simulations_valued_resources_attack_paths_list(&self, parent: &str) -> OrganizationSimulationValuedResourceAttackPathListCall<'a, S> {
        OrganizationSimulationValuedResourceAttackPathListCall {
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
    /// Get the valued resource by name
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of this valued resource Valid format: "organizations/{organization}/simulations/{simulation}/valuedResources/{valued_resource}"
    pub fn simulations_valued_resources_get(&self, name: &str) -> OrganizationSimulationValuedResourceGetCall<'a, S> {
        OrganizationSimulationValuedResourceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the valued resources for a set of simulation results and filter.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list valued resources. Valid formats: "organizations/{organization}", "organizations/{organization}/simulations/{simulation}" "organizations/{organization}/simulations/{simulation}/attackExposureResults/{attack_exposure_result_v2}"
    pub fn simulations_valued_resources_list(&self, parent: &str) -> OrganizationSimulationValuedResourceListCall<'a, S> {
        OrganizationSimulationValuedResourceListCall {
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
    /// Get the simulation by name or the latest simulation for the given organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The organization name or simulation name of this simulation Valid format: "organizations/{organization}/simulations/latest" "organizations/{organization}/simulations/{simulation}"
    pub fn simulations_get(&self, name: &str) -> OrganizationSimulationGetCall<'a, S> {
        OrganizationSimulationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates external system. This is for a given finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    pub fn sources_findings_external_systems_patch(&self, request: GoogleCloudSecuritycenterV1ExternalSystem, name: &str) -> OrganizationSourceFindingExternalSystemPatchCall<'a, S> {
        OrganizationSourceFindingExternalSystemPatchCall {
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
    /// Creates a finding. The corresponding source must exist for finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new finding's parent. Its format should be "organizations/[organization_id]/sources/[source_id]".
    pub fn sources_findings_create(&self, request: Finding, parent: &str) -> OrganizationSourceFindingCreateCall<'a, S> {
        OrganizationSourceFindingCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _finding_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization or source's findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the source to groupBy. Its format is "organizations/[organization_id]/sources/[source_id]", folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]. To groupBy across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-, or projects/{project_id}/sources/-
    pub fn sources_findings_group(&self, request: GroupFindingsRequest, parent: &str) -> OrganizationSourceFindingGroupCall<'a, S> {
        OrganizationSourceFindingGroupCall {
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
    /// Lists an organization or source's findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the source the findings belong to. Its format is "organizations/[organization_id]/sources/[source_id], folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]". To list across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or projects/{projects_id}/sources/-
    pub fn sources_findings_list(&self, parent: &str) -> OrganizationSourceFindingListCall<'a, S> {
        OrganizationSourceFindingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_patch(&self, request: Finding, name: &str) -> OrganizationSourceFindingPatchCall<'a, S> {
        OrganizationSourceFindingPatchCall {
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
    /// Updates the mute state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_mute(&self, request: SetMuteRequest, name: &str) -> OrganizationSourceFindingSetMuteCall<'a, S> {
        OrganizationSourceFindingSetMuteCall {
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
    /// Updates the state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_state(&self, request: SetFindingStateRequest, name: &str) -> OrganizationSourceFindingSetStateCall<'a, S> {
        OrganizationSourceFindingSetStateCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn sources_findings_update_security_marks(&self, request: SecurityMarks, name: &str) -> OrganizationSourceFindingUpdateSecurityMarkCall<'a, S> {
        OrganizationSourceFindingUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new source's parent. Its format should be "organizations/[organization_id]".
    pub fn sources_create(&self, request: Source, parent: &str) -> OrganizationSourceCreateCall<'a, S> {
        OrganizationSourceCreateCall {
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
    /// Gets a source.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Relative resource name of the source. Its format is "organizations/[organization_id]/source/[source_id]".
    pub fn sources_get(&self, name: &str) -> OrganizationSourceGetCall<'a, S> {
        OrganizationSourceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy on the specified Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn sources_get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> OrganizationSourceGetIamPolicyCall<'a, S> {
        OrganizationSourceGetIamPolicyCall {
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
    /// Lists all sources belonging to an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent of sources to list. Its format should be "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn sources_list(&self, parent: &str) -> OrganizationSourceListCall<'a, S> {
        OrganizationSourceListCall {
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
    /// Updates a source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this source. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/sources/{source_id}"
    pub fn sources_patch(&self, request: Source, name: &str) -> OrganizationSourcePatchCall<'a, S> {
        OrganizationSourcePatchCall {
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
    /// Sets the access control policy on the specified Source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn sources_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> OrganizationSourceSetIamPolicyCall<'a, S> {
        OrganizationSourceSetIamPolicyCall {
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
    /// Returns the permissions that a caller has on the specified source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn sources_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> OrganizationSourceTestIamPermissionCall<'a, S> {
        OrganizationSourceTestIamPermissionCall {
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
    /// Gets the settings for an organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the organization to get organization settings for. Its format is "organizations/[organization_id]/organizationSettings".
    pub fn get_organization_settings(&self, name: &str) -> OrganizationGetOrganizationSettingCall<'a, S> {
        OrganizationGetOrganizationSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an organization's settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the settings. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/organizationSettings".
    pub fn update_organization_settings(&self, request: OrganizationSettings, name: &str) -> OrganizationUpdateOrganizationSettingCall<'a, S> {
        OrganizationUpdateOrganizationSettingCall {
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
/// It is not used directly, but through the [`SecurityCommandCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_securitycenter1 as securitycenter1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use securitycenter1::{SecurityCommandCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SecurityCommandCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_group(...)`, `assets_list(...)`, `assets_update_security_marks(...)`, `big_query_exports_create(...)`, `big_query_exports_delete(...)`, `big_query_exports_get(...)`, `big_query_exports_list(...)`, `big_query_exports_patch(...)`, `event_threat_detection_settings_custom_modules_create(...)`, `event_threat_detection_settings_custom_modules_delete(...)`, `event_threat_detection_settings_custom_modules_get(...)`, `event_threat_detection_settings_custom_modules_list(...)`, `event_threat_detection_settings_custom_modules_list_descendant(...)`, `event_threat_detection_settings_custom_modules_patch(...)`, `event_threat_detection_settings_effective_custom_modules_get(...)`, `event_threat_detection_settings_effective_custom_modules_list(...)`, `event_threat_detection_settings_validate_custom_module(...)`, `findings_bulk_mute(...)`, `locations_mute_configs_delete(...)`, `locations_mute_configs_get(...)`, `locations_mute_configs_patch(...)`, `mute_configs_create(...)`, `mute_configs_delete(...)`, `mute_configs_get(...)`, `mute_configs_list(...)`, `mute_configs_patch(...)`, `notification_configs_create(...)`, `notification_configs_delete(...)`, `notification_configs_get(...)`, `notification_configs_list(...)`, `notification_configs_patch(...)`, `security_health_analytics_settings_custom_modules_create(...)`, `security_health_analytics_settings_custom_modules_delete(...)`, `security_health_analytics_settings_custom_modules_get(...)`, `security_health_analytics_settings_custom_modules_list(...)`, `security_health_analytics_settings_custom_modules_list_descendant(...)`, `security_health_analytics_settings_custom_modules_patch(...)`, `security_health_analytics_settings_custom_modules_simulate(...)`, `security_health_analytics_settings_effective_custom_modules_get(...)`, `security_health_analytics_settings_effective_custom_modules_list(...)`, `sources_findings_external_systems_patch(...)`, `sources_findings_group(...)`, `sources_findings_list(...)`, `sources_findings_patch(...)`, `sources_findings_set_mute(...)`, `sources_findings_set_state(...)`, `sources_findings_update_security_marks(...)` and `sources_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SecurityCommandCenter<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Filters an organization's assets and groups them by their specified properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent to group the assets by. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_group(&self, request: GroupAssetsRequest, parent: &str) -> ProjectAssetGroupCall<'a, S> {
        ProjectAssetGroupCall {
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
    /// Lists an organization's assets.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent resource that contains the assets. The value that you can specify on parent depends on the method in which you specify parent. You can specify one of the following values: "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn assets_list(&self, parent: &str) -> ProjectAssetListCall<'a, S> {
        ProjectAssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn assets_update_security_marks(&self, request: SecurityMarks, name: &str) -> ProjectAssetUpdateSecurityMarkCall<'a, S> {
        ProjectAssetUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent resource of the new BigQuery export. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn big_query_exports_create(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, parent: &str) -> ProjectBigQueryExportCreateCall<'a, S> {
        ProjectBigQueryExportCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _big_query_export_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the BigQuery export to delete. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_delete(&self, name: &str) -> ProjectBigQueryExportDeleteCall<'a, S> {
        ProjectBigQueryExportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the BigQuery export to retrieve. Its format is organizations/{organization}/bigQueryExports/{export_id}, folders/{folder}/bigQueryExports/{export_id}, or projects/{project}/bigQueryExports/{export_id}
    pub fn big_query_exports_get(&self, name: &str) -> ProjectBigQueryExportGetCall<'a, S> {
        ProjectBigQueryExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists BigQuery exports. Note that when requesting BigQuery exports at a given level all exports under that level are also returned e.g. if requesting BigQuery exports under a folder, then all BigQuery exports immediately under the folder plus the ones created under the projects within the folder are returned.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of BigQuery exports. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn big_query_exports_list(&self, parent: &str) -> ProjectBigQueryExportListCall<'a, S> {
        ProjectBigQueryExportListCall {
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
    /// Updates a BigQuery export.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this export. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name. Example format: "organizations/{organization_id}/bigQueryExports/{export_id}" Example format: "folders/{folder_id}/bigQueryExports/{export_id}" Example format: "projects/{project_id}/bigQueryExports/{export_id}" This field is provided in responses, and is ignored when provided in create requests.
    pub fn big_query_exports_patch(&self, request: GoogleCloudSecuritycenterV1BigQueryExport, name: &str) -> ProjectBigQueryExportPatchCall<'a, S> {
        ProjectBigQueryExportPatchCall {
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
    /// Creates a resident Event Threat Detection custom module at the scope of the given Resource Manager parent, and also creates inherited custom modules for all descendants of the given parent. These modules are enabled by default.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The new custom module's parent. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_create(&self, request: EventThreatDetectionCustomModule, parent: &str) -> ProjectEventThreatDetectionSettingCustomModuleCreateCall<'a, S> {
        ProjectEventThreatDetectionSettingCustomModuleCreateCall {
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
    /// Deletes the specified Event Threat Detection custom module and all of its descendants in the Resource Manager hierarchy. This method is only supported for resident custom modules.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to delete. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_delete(&self, name: &str) -> ProjectEventThreatDetectionSettingCustomModuleDeleteCall<'a, S> {
        ProjectEventThreatDetectionSettingCustomModuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an Event Threat Detection custom module.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to get. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_get(&self, name: &str) -> ProjectEventThreatDetectionSettingCustomModuleGetCall<'a, S> {
        ProjectEventThreatDetectionSettingCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Event Threat Detection custom modules for the given Resource Manager parent. This includes resident modules defined at the scope of the parent along with modules inherited from ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_list(&self, parent: &str) -> ProjectEventThreatDetectionSettingCustomModuleListCall<'a, S> {
        ProjectEventThreatDetectionSettingCustomModuleListCall {
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
    /// Lists all resident Event Threat Detection custom modules under the given Resource Manager parent and its descendants.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_custom_modules_list_descendant(&self, parent: &str) -> ProjectEventThreatDetectionSettingCustomModuleListDescendantCall<'a, S> {
        ProjectEventThreatDetectionSettingCustomModuleListDescendantCall {
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
    /// Updates the Event Threat Detection custom module with the given name based on the given update mask. Updating the enablement state is supported for both resident and inherited modules (though resident modules cannot have an enablement state of "inherited"). Updating the display name or configuration of a module is supported for resident modules only. The type of a module cannot be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the Event Threat Detection custom module. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/customModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/customModules/{module}". * "projects/{project}/eventThreatDetectionSettings/customModules/{module}".
    pub fn event_threat_detection_settings_custom_modules_patch(&self, request: EventThreatDetectionCustomModule, name: &str) -> ProjectEventThreatDetectionSettingCustomModulePatchCall<'a, S> {
        ProjectEventThreatDetectionSettingCustomModulePatchCall {
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
    /// Gets an effective Event Threat Detection custom module at the given level.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the effective Event Threat Detection custom module. Its format is: * "organizations/{organization}/eventThreatDetectionSettings/effectiveCustomModules/{module}". * "folders/{folder}/eventThreatDetectionSettings/effectiveCustomModules/{module}". * "projects/{project}/eventThreatDetectionSettings/effectiveCustomModules/{module}".
    pub fn event_threat_detection_settings_effective_custom_modules_get(&self, name: &str) -> ProjectEventThreatDetectionSettingEffectiveCustomModuleGetCall<'a, S> {
        ProjectEventThreatDetectionSettingEffectiveCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all effective Event Threat Detection custom modules for the given parent. This includes resident modules defined at the scope of the parent along with modules inherited from its ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the parent to list custom modules for. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_effective_custom_modules_list(&self, parent: &str) -> ProjectEventThreatDetectionSettingEffectiveCustomModuleListCall<'a, S> {
        ProjectEventThreatDetectionSettingEffectiveCustomModuleListCall {
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
    /// Validates the given Event Threat Detection custom module.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the parent to validate the Custom Module under. Its format is: * "organizations/{organization}/eventThreatDetectionSettings". * "folders/{folder}/eventThreatDetectionSettings". * "projects/{project}/eventThreatDetectionSettings".
    pub fn event_threat_detection_settings_validate_custom_module(&self, request: ValidateEventThreatDetectionCustomModuleRequest, parent: &str) -> ProjectEventThreatDetectionSettingValidateCustomModuleCall<'a, S> {
        ProjectEventThreatDetectionSettingValidateCustomModuleCall {
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
    /// Kicks off an LRO to bulk mute findings for a parent based on a filter. The parent can be either an organization, folder or project. The findings matched by the filter will be muted after the LRO is done.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, at which bulk action needs to be applied. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn findings_bulk_mute(&self, request: BulkMuteFindingsRequest, parent: &str) -> ProjectFindingBulkMuteCall<'a, S> {
        ProjectFindingBulkMuteCall {
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
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn locations_mute_configs_delete(&self, name: &str) -> ProjectLocationMuteConfigDeleteCall<'a, S> {
        ProjectLocationMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn locations_mute_configs_get(&self, name: &str) -> ProjectLocationMuteConfigGetCall<'a, S> {
        ProjectLocationMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}" "organizations/{organization}/locations/global/muteConfigs/{mute_config}" "folders/{folder}/locations/global/muteConfigs/{mute_config}" "projects/{project}/locations/global/muteConfigs/{mute_config}"
    pub fn locations_mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> ProjectLocationMuteConfigPatchCall<'a, S> {
        ProjectLocationMuteConfigPatchCall {
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
    /// Creates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new mute configs's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn mute_configs_create(&self, request: GoogleCloudSecuritycenterV1MuteConfig, parent: &str) -> ProjectMuteConfigCreateCall<'a, S> {
        ProjectMuteConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _mute_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to delete. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn mute_configs_delete(&self, name: &str) -> ProjectMuteConfigDeleteCall<'a, S> {
        ProjectMuteConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a mute config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the mute config to retrieve. Its format is organizations/{organization}/muteConfigs/{config_id}, folders/{folder}/muteConfigs/{config_id}, projects/{project}/muteConfigs/{config_id}, organizations/{organization}/locations/global/muteConfigs/{config_id}, folders/{folder}/locations/global/muteConfigs/{config_id}, or projects/{project}/locations/global/muteConfigs/{config_id}.
    pub fn mute_configs_get(&self, name: &str) -> ProjectMuteConfigGetCall<'a, S> {
        ProjectMuteConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists mute configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns the collection of mute configs. Its format is "organizations/[organization_id]", "folders/[folder_id]", "projects/[project_id]".
    pub fn mute_configs_list(&self, parent: &str) -> ProjectMuteConfigListCall<'a, S> {
        ProjectMuteConfigListCall {
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
    /// Updates a mute config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - This field will be ignored if provided on config creation. Format "organizations/{organization}/muteConfigs/{mute_config}" "folders/{folder}/muteConfigs/{mute_config}" "projects/{project}/muteConfigs/{mute_config}" "organizations/{organization}/locations/global/muteConfigs/{mute_config}" "folders/{folder}/locations/global/muteConfigs/{mute_config}" "projects/{project}/locations/global/muteConfigs/{mute_config}"
    pub fn mute_configs_patch(&self, request: GoogleCloudSecuritycenterV1MuteConfig, name: &str) -> ProjectMuteConfigPatchCall<'a, S> {
        ProjectMuteConfigPatchCall {
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
    /// Creates a notification config.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new notification config's parent. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_create(&self, request: NotificationConfig, parent: &str) -> ProjectNotificationConfigCreateCall<'a, S> {
        ProjectNotificationConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to delete. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_delete(&self, name: &str) -> ProjectNotificationConfigDeleteCall<'a, S> {
        ProjectNotificationConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a notification config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the notification config to get. Its format is "organizations/[organization_id]/notificationConfigs/[config_id]", "folders/[folder_id]/notificationConfigs/[config_id]", or "projects/[project_id]/notificationConfigs/[config_id]".
    pub fn notification_configs_get(&self, name: &str) -> ProjectNotificationConfigGetCall<'a, S> {
        ProjectNotificationConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists notification configs.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent in which to list the notification configurations. Its format is "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn notification_configs_list(&self, parent: &str) -> ProjectNotificationConfigListCall<'a, S> {
        ProjectNotificationConfigListCall {
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
    ///  Updates a notification config. The following update fields are allowed: description, pubsub_topic, streaming_config.filter
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of this notification config. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Example: "organizations/{organization_id}/notificationConfigs/notify_public_bucket", "folders/{folder_id}/notificationConfigs/notify_public_bucket", or "projects/{project_id}/notificationConfigs/notify_public_bucket".
    pub fn notification_configs_patch(&self, request: NotificationConfig, name: &str) -> ProjectNotificationConfigPatchCall<'a, S> {
        ProjectNotificationConfigPatchCall {
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
    /// Creates a resident SecurityHealthAnalyticsCustomModule at the scope of the given CRM parent, and also creates inherited SecurityHealthAnalyticsCustomModules for all CRM descendants of the given parent. These modules are enabled by default.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the new custom module's parent. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_create(&self, request: GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModule, parent: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModuleCreateCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModuleCreateCall {
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
    /// Deletes the specified SecurityHealthAnalyticsCustomModule and all of its descendants in the CRM hierarchy. This method is only supported for resident custom modules.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to delete. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}"
    pub fn security_health_analytics_settings_custom_modules_delete(&self, name: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModuleDeleteCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModuleDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a SecurityHealthAnalyticsCustomModule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the custom module to get. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}"
    pub fn security_health_analytics_settings_custom_modules_get(&self, name: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModuleGetCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all SecurityHealthAnalyticsCustomModules for the given parent. This includes resident modules defined at the scope of the parent, and inherited modules, inherited from CRM ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_list(&self, parent: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModuleListCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModuleListCall {
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
    /// Returns a list of all resident SecurityHealthAnalyticsCustomModules under the given CRM parent and all of the parent’s CRM descendants.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list descendant custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_custom_modules_list_descendant(&self, parent: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModuleListDescendantCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModuleListDescendantCall {
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
    /// Updates the SecurityHealthAnalyticsCustomModule under the given name based on the given update mask. Updating the enablement state is supported on both resident and inherited modules (though resident modules cannot have an enablement state of "inherited"). Updating the display name and custom config of a module is supported on resident modules only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. The resource name of the custom module. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/customModules/{customModule}", or "folders/{folder}/securityHealthAnalyticsSettings/customModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/customModules/{customModule}" The id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits.
    pub fn security_health_analytics_settings_custom_modules_patch(&self, request: GoogleCloudSecuritycenterV1SecurityHealthAnalyticsCustomModule, name: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModulePatchCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModulePatchCall {
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
    /// Simulates a given SecurityHealthAnalyticsCustomModule and Resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative resource name of the organization, project, or folder. For more information about relative resource names, see [Relative Resource Name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) Example: `organizations/{organization_id}`
    pub fn security_health_analytics_settings_custom_modules_simulate(&self, request: SimulateSecurityHealthAnalyticsCustomModuleRequest, parent: &str) -> ProjectSecurityHealthAnalyticsSettingCustomModuleSimulateCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingCustomModuleSimulateCall {
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
    /// Retrieves an EffectiveSecurityHealthAnalyticsCustomModule.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the effective custom module to get. Its format is "organizations/{organization}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", "folders/{folder}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}", or "projects/{project}/securityHealthAnalyticsSettings/effectiveCustomModules/{customModule}"
    pub fn security_health_analytics_settings_effective_custom_modules_get(&self, name: &str) -> ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all EffectiveSecurityHealthAnalyticsCustomModules for the given parent. This includes resident modules defined at the scope of the parent, and inherited modules, inherited from CRM ancestors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of parent to list effective custom modules. Its format is "organizations/{organization}/securityHealthAnalyticsSettings", "folders/{folder}/securityHealthAnalyticsSettings", or "projects/{project}/securityHealthAnalyticsSettings"
    pub fn security_health_analytics_settings_effective_custom_modules_list(&self, parent: &str) -> ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall<'a, S> {
        ProjectSecurityHealthAnalyticsSettingEffectiveCustomModuleListCall {
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
    /// Updates external system. This is for a given finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Full resource name of the external system, for example: "organizations/1234/sources/5678/findings/123456/externalSystems/jira", "folders/1234/sources/5678/findings/123456/externalSystems/jira", "projects/1234/sources/5678/findings/123456/externalSystems/jira"
    pub fn sources_findings_external_systems_patch(&self, request: GoogleCloudSecuritycenterV1ExternalSystem, name: &str) -> ProjectSourceFindingExternalSystemPatchCall<'a, S> {
        ProjectSourceFindingExternalSystemPatchCall {
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
    /// Filters an organization or source's findings and groups them by their specified properties. To group across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings, /v1/folders/{folder_id}/sources/-/findings, /v1/projects/{project_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the source to groupBy. Its format is "organizations/[organization_id]/sources/[source_id]", folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]. To groupBy across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/-, or projects/{project_id}/sources/-
    pub fn sources_findings_group(&self, request: GroupFindingsRequest, parent: &str) -> ProjectSourceFindingGroupCall<'a, S> {
        ProjectSourceFindingGroupCall {
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
    /// Lists an organization or source's findings. To list across all sources provide a `-` as the source id. Example: /v1/organizations/{organization_id}/sources/-/findings
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the source the findings belong to. Its format is "organizations/[organization_id]/sources/[source_id], folders/[folder_id]/sources/[source_id], or projects/[project_id]/sources/[source_id]". To list across all sources provide a source_id of `-`. For example: organizations/{organization_id}/sources/-, folders/{folder_id}/sources/- or projects/{projects_id}/sources/-
    pub fn sources_findings_list(&self, parent: &str) -> ProjectSourceFindingListCall<'a, S> {
        ProjectSourceFindingListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _field_mask: Default::default(),
            _compare_duration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a finding. The corresponding source must exist for a finding creation to succeed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_patch(&self, request: Finding, name: &str) -> ProjectSourceFindingPatchCall<'a, S> {
        ProjectSourceFindingPatchCall {
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
    /// Updates the mute state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_mute(&self, request: SetMuteRequest, name: &str) -> ProjectSourceFindingSetMuteCall<'a, S> {
        ProjectSourceFindingSetMuteCall {
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
    /// Updates the state of a finding.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [relative resource name](https://cloud.google.com/apis/design/resource_names#relative_resource_name) of the finding. Example: "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}", "folders/{folder_id}/sources/{source_id}/findings/{finding_id}", "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    pub fn sources_findings_set_state(&self, request: SetFindingStateRequest, name: &str) -> ProjectSourceFindingSetStateCall<'a, S> {
        ProjectSourceFindingSetStateCall {
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
    /// Updates security marks.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The relative resource name of the SecurityMarks. See: https://cloud.google.com/apis/design/resource_names#relative_resource_name Examples: "organizations/{organization_id}/assets/{asset_id}/securityMarks" "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    pub fn sources_findings_update_security_marks(&self, request: SecurityMarks, name: &str) -> ProjectSourceFindingUpdateSecurityMarkCall<'a, S> {
        ProjectSourceFindingUpdateSecurityMarkCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _start_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all sources belonging to an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent of sources to list. Its format should be "organizations/[organization_id]", "folders/[folder_id]", or "projects/[project_id]".
    pub fn sources_list(&self, parent: &str) -> ProjectSourceListCall<'a, S> {
        ProjectSourceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



