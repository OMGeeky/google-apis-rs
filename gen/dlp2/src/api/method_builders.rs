use super::*;
/// A builder providing access to all methods supported on *infoType* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2 as dlp2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.info_types();
/// # }
/// ```
pub struct InfoTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for InfoTypeMethods<'a, S> {}

impl<'a, S> InfoTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of the sensitive information types that DLP API supports. See https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference to learn more.
    pub fn list(&self) -> InfoTypeListCall<'a, S> {
        InfoTypeListCall {
            hub: self.hub,
            _parent: Default::default(),
            _location_id: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2 as dlp2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `info_types_list(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of the sensitive information types that DLP API supports. See https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent resource name. The format of this value is as follows: locations/ LOCATION_ID
    pub fn info_types_list(&self, parent: &str) -> LocationInfoTypeListCall<'a, S> {
        LocationInfoTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _location_id: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2 as dlp2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `deidentify_templates_create(...)`, `deidentify_templates_delete(...)`, `deidentify_templates_get(...)`, `deidentify_templates_list(...)`, `deidentify_templates_patch(...)`, `inspect_templates_create(...)`, `inspect_templates_delete(...)`, `inspect_templates_get(...)`, `inspect_templates_list(...)`, `inspect_templates_patch(...)`, `locations_column_data_profiles_get(...)`, `locations_column_data_profiles_list(...)`, `locations_connections_search(...)`, `locations_deidentify_templates_create(...)`, `locations_deidentify_templates_delete(...)`, `locations_deidentify_templates_get(...)`, `locations_deidentify_templates_list(...)`, `locations_deidentify_templates_patch(...)`, `locations_discovery_configs_create(...)`, `locations_discovery_configs_delete(...)`, `locations_discovery_configs_get(...)`, `locations_discovery_configs_list(...)`, `locations_discovery_configs_patch(...)`, `locations_dlp_jobs_list(...)`, `locations_inspect_templates_create(...)`, `locations_inspect_templates_delete(...)`, `locations_inspect_templates_get(...)`, `locations_inspect_templates_list(...)`, `locations_inspect_templates_patch(...)`, `locations_job_triggers_create(...)`, `locations_job_triggers_delete(...)`, `locations_job_triggers_get(...)`, `locations_job_triggers_list(...)`, `locations_job_triggers_patch(...)`, `locations_project_data_profiles_get(...)`, `locations_project_data_profiles_list(...)`, `locations_stored_info_types_create(...)`, `locations_stored_info_types_delete(...)`, `locations_stored_info_types_get(...)`, `locations_stored_info_types_list(...)`, `locations_stored_info_types_patch(...)`, `locations_table_data_profiles_delete(...)`, `locations_table_data_profiles_get(...)`, `locations_table_data_profiles_list(...)`, `stored_info_types_create(...)`, `stored_info_types_delete(...)`, `stored_info_types_get(...)`, `stored_info_types_list(...)` and `stored_info_types_patch(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn deidentify_templates_create(&self, request: GooglePrivacyDlpV2CreateDeidentifyTemplateRequest, parent: &str) -> OrganizationDeidentifyTemplateCreateCall<'a, S> {
        OrganizationDeidentifyTemplateCreateCall {
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
    /// Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn deidentify_templates_delete(&self, name: &str) -> OrganizationDeidentifyTemplateDeleteCall<'a, S> {
        OrganizationDeidentifyTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn deidentify_templates_get(&self, name: &str) -> OrganizationDeidentifyTemplateGetCall<'a, S> {
        OrganizationDeidentifyTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn deidentify_templates_list(&self, parent: &str) -> OrganizationDeidentifyTemplateListCall<'a, S> {
        OrganizationDeidentifyTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn deidentify_templates_patch(&self, request: GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest, name: &str) -> OrganizationDeidentifyTemplatePatchCall<'a, S> {
        OrganizationDeidentifyTemplatePatchCall {
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
    /// Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn inspect_templates_create(&self, request: GooglePrivacyDlpV2CreateInspectTemplateRequest, parent: &str) -> OrganizationInspectTemplateCreateCall<'a, S> {
        OrganizationInspectTemplateCreateCall {
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
    /// Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn inspect_templates_delete(&self, name: &str) -> OrganizationInspectTemplateDeleteCall<'a, S> {
        OrganizationInspectTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn inspect_templates_get(&self, name: &str) -> OrganizationInspectTemplateGetCall<'a, S> {
        OrganizationInspectTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn inspect_templates_list(&self, parent: &str) -> OrganizationInspectTemplateListCall<'a, S> {
        OrganizationInspectTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn inspect_templates_patch(&self, request: GooglePrivacyDlpV2UpdateInspectTemplateRequest, name: &str) -> OrganizationInspectTemplatePatchCall<'a, S> {
        OrganizationInspectTemplatePatchCall {
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
    /// Gets a column data profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name, for example `organizations/12345/locations/us/columnDataProfiles/53234423`.
    pub fn locations_column_data_profiles_get(&self, name: &str) -> OrganizationLocationColumnDataProfileGetCall<'a, S> {
        OrganizationLocationColumnDataProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists column data profiles for an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`.
    pub fn locations_column_data_profiles_list(&self, parent: &str) -> OrganizationLocationColumnDataProfileListCall<'a, S> {
        OrganizationLocationColumnDataProfileListCall {
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
    /// Searches for Connections in a parent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent name, typically an organization, without location. For example: "organizations/12345678".
    pub fn locations_connections_search(&self, parent: &str) -> OrganizationLocationConnectionSearchCall<'a, S> {
        OrganizationLocationConnectionSearchCall {
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
    /// Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_deidentify_templates_create(&self, request: GooglePrivacyDlpV2CreateDeidentifyTemplateRequest, parent: &str) -> OrganizationLocationDeidentifyTemplateCreateCall<'a, S> {
        OrganizationLocationDeidentifyTemplateCreateCall {
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
    /// Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn locations_deidentify_templates_delete(&self, name: &str) -> OrganizationLocationDeidentifyTemplateDeleteCall<'a, S> {
        OrganizationLocationDeidentifyTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn locations_deidentify_templates_get(&self, name: &str) -> OrganizationLocationDeidentifyTemplateGetCall<'a, S> {
        OrganizationLocationDeidentifyTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_deidentify_templates_list(&self, parent: &str) -> OrganizationLocationDeidentifyTemplateListCall<'a, S> {
        OrganizationLocationDeidentifyTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn locations_deidentify_templates_patch(&self, request: GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest, name: &str) -> OrganizationLocationDeidentifyTemplatePatchCall<'a, S> {
        OrganizationLocationDeidentifyTemplatePatchCall {
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
    /// Creates a config for discovery to scan and profile storage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value is as follows: `projects/`PROJECT_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_discovery_configs_create(&self, request: GooglePrivacyDlpV2CreateDiscoveryConfigRequest, parent: &str) -> OrganizationLocationDiscoveryConfigCreateCall<'a, S> {
        OrganizationLocationDiscoveryConfigCreateCall {
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
    /// Deletes a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the config, for example `projects/dlp-test-project/discoveryConfigs/53234423`.
    pub fn locations_discovery_configs_delete(&self, name: &str) -> OrganizationLocationDiscoveryConfigDeleteCall<'a, S> {
        OrganizationLocationDiscoveryConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`.
    pub fn locations_discovery_configs_get(&self, name: &str) -> OrganizationLocationDiscoveryConfigGetCall<'a, S> {
        OrganizationLocationDiscoveryConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists discovery configurations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value is as follows: `projects/`PROJECT_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_discovery_configs_list(&self, parent: &str) -> OrganizationLocationDiscoveryConfigListCall<'a, S> {
        OrganizationLocationDiscoveryConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`.
    pub fn locations_discovery_configs_patch(&self, request: GooglePrivacyDlpV2UpdateDiscoveryConfigRequest, name: &str) -> OrganizationLocationDiscoveryConfigPatchCall<'a, S> {
        OrganizationLocationDiscoveryConfigPatchCall {
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
    /// Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_dlp_jobs_list(&self, parent: &str) -> OrganizationLocationDlpJobListCall<'a, S> {
        OrganizationLocationDlpJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_inspect_templates_create(&self, request: GooglePrivacyDlpV2CreateInspectTemplateRequest, parent: &str) -> OrganizationLocationInspectTemplateCreateCall<'a, S> {
        OrganizationLocationInspectTemplateCreateCall {
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
    /// Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn locations_inspect_templates_delete(&self, name: &str) -> OrganizationLocationInspectTemplateDeleteCall<'a, S> {
        OrganizationLocationInspectTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn locations_inspect_templates_get(&self, name: &str) -> OrganizationLocationInspectTemplateGetCall<'a, S> {
        OrganizationLocationInspectTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_inspect_templates_list(&self, parent: &str) -> OrganizationLocationInspectTemplateListCall<'a, S> {
        OrganizationLocationInspectTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn locations_inspect_templates_patch(&self, request: GooglePrivacyDlpV2UpdateInspectTemplateRequest, name: &str) -> OrganizationLocationInspectTemplatePatchCall<'a, S> {
        OrganizationLocationInspectTemplatePatchCall {
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
    /// Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_job_triggers_create(&self, request: GooglePrivacyDlpV2CreateJobTriggerRequest, parent: &str) -> OrganizationLocationJobTriggerCreateCall<'a, S> {
        OrganizationLocationJobTriggerCreateCall {
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
    /// Deletes a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_delete(&self, name: &str) -> OrganizationLocationJobTriggerDeleteCall<'a, S> {
        OrganizationLocationJobTriggerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_get(&self, name: &str) -> OrganizationLocationJobTriggerGetCall<'a, S> {
        OrganizationLocationJobTriggerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists job triggers. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_job_triggers_list(&self, parent: &str) -> OrganizationLocationJobTriggerListCall<'a, S> {
        OrganizationLocationJobTriggerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_patch(&self, request: GooglePrivacyDlpV2UpdateJobTriggerRequest, name: &str) -> OrganizationLocationJobTriggerPatchCall<'a, S> {
        OrganizationLocationJobTriggerPatchCall {
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
    /// Gets a project data profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name, for example `organizations/12345/locations/us/projectDataProfiles/53234423`.
    pub fn locations_project_data_profiles_get(&self, name: &str) -> OrganizationLocationProjectDataProfileGetCall<'a, S> {
        OrganizationLocationProjectDataProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists project data profiles for an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. organizations/{org_id}/locations/{loc_id}
    pub fn locations_project_data_profiles_list(&self, parent: &str) -> OrganizationLocationProjectDataProfileListCall<'a, S> {
        OrganizationLocationProjectDataProfileListCall {
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
    /// Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_stored_info_types_create(&self, request: GooglePrivacyDlpV2CreateStoredInfoTypeRequest, parent: &str) -> OrganizationLocationStoredInfoTypeCreateCall<'a, S> {
        OrganizationLocationStoredInfoTypeCreateCall {
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
    /// Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn locations_stored_info_types_delete(&self, name: &str) -> OrganizationLocationStoredInfoTypeDeleteCall<'a, S> {
        OrganizationLocationStoredInfoTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn locations_stored_info_types_get(&self, name: &str) -> OrganizationLocationStoredInfoTypeGetCall<'a, S> {
        OrganizationLocationStoredInfoTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_stored_info_types_list(&self, parent: &str) -> OrganizationLocationStoredInfoTypeListCall<'a, S> {
        OrganizationLocationStoredInfoTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn locations_stored_info_types_patch(&self, request: GooglePrivacyDlpV2UpdateStoredInfoTypeRequest, name: &str) -> OrganizationLocationStoredInfoTypePatchCall<'a, S> {
        OrganizationLocationStoredInfoTypePatchCall {
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
    /// Delete a TableDataProfile. Will not prevent the profile from being regenerated if the table is still included in a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the table data profile.
    pub fn locations_table_data_profiles_delete(&self, name: &str) -> OrganizationLocationTableDataProfileDeleteCall<'a, S> {
        OrganizationLocationTableDataProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a table data profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name, for example `organizations/12345/locations/us/tableDataProfiles/53234423`.
    pub fn locations_table_data_profiles_get(&self, name: &str) -> OrganizationLocationTableDataProfileGetCall<'a, S> {
        OrganizationLocationTableDataProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists table data profiles for an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`.
    pub fn locations_table_data_profiles_list(&self, parent: &str) -> OrganizationLocationTableDataProfileListCall<'a, S> {
        OrganizationLocationTableDataProfileListCall {
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
    /// Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn stored_info_types_create(&self, request: GooglePrivacyDlpV2CreateStoredInfoTypeRequest, parent: &str) -> OrganizationStoredInfoTypeCreateCall<'a, S> {
        OrganizationStoredInfoTypeCreateCall {
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
    /// Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn stored_info_types_delete(&self, name: &str) -> OrganizationStoredInfoTypeDeleteCall<'a, S> {
        OrganizationStoredInfoTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn stored_info_types_get(&self, name: &str) -> OrganizationStoredInfoTypeGetCall<'a, S> {
        OrganizationStoredInfoTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn stored_info_types_list(&self, parent: &str) -> OrganizationStoredInfoTypeListCall<'a, S> {
        OrganizationStoredInfoTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn stored_info_types_patch(&self, request: GooglePrivacyDlpV2UpdateStoredInfoTypeRequest, name: &str) -> OrganizationStoredInfoTypePatchCall<'a, S> {
        OrganizationStoredInfoTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`DLP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dlp2 as dlp2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dlp2::{DLP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DLP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `content_deidentify(...)`, `content_inspect(...)`, `content_reidentify(...)`, `deidentify_templates_create(...)`, `deidentify_templates_delete(...)`, `deidentify_templates_get(...)`, `deidentify_templates_list(...)`, `deidentify_templates_patch(...)`, `dlp_jobs_cancel(...)`, `dlp_jobs_create(...)`, `dlp_jobs_delete(...)`, `dlp_jobs_get(...)`, `dlp_jobs_list(...)`, `image_redact(...)`, `inspect_templates_create(...)`, `inspect_templates_delete(...)`, `inspect_templates_get(...)`, `inspect_templates_list(...)`, `inspect_templates_patch(...)`, `job_triggers_activate(...)`, `job_triggers_create(...)`, `job_triggers_delete(...)`, `job_triggers_get(...)`, `job_triggers_list(...)`, `job_triggers_patch(...)`, `locations_column_data_profiles_get(...)`, `locations_column_data_profiles_list(...)`, `locations_connections_create(...)`, `locations_connections_delete(...)`, `locations_connections_get(...)`, `locations_connections_list(...)`, `locations_connections_patch(...)`, `locations_connections_search(...)`, `locations_content_deidentify(...)`, `locations_content_inspect(...)`, `locations_content_reidentify(...)`, `locations_deidentify_templates_create(...)`, `locations_deidentify_templates_delete(...)`, `locations_deidentify_templates_get(...)`, `locations_deidentify_templates_list(...)`, `locations_deidentify_templates_patch(...)`, `locations_discovery_configs_create(...)`, `locations_discovery_configs_delete(...)`, `locations_discovery_configs_get(...)`, `locations_discovery_configs_list(...)`, `locations_discovery_configs_patch(...)`, `locations_dlp_jobs_cancel(...)`, `locations_dlp_jobs_create(...)`, `locations_dlp_jobs_delete(...)`, `locations_dlp_jobs_finish(...)`, `locations_dlp_jobs_get(...)`, `locations_dlp_jobs_hybrid_inspect(...)`, `locations_dlp_jobs_list(...)`, `locations_image_redact(...)`, `locations_inspect_templates_create(...)`, `locations_inspect_templates_delete(...)`, `locations_inspect_templates_get(...)`, `locations_inspect_templates_list(...)`, `locations_inspect_templates_patch(...)`, `locations_job_triggers_activate(...)`, `locations_job_triggers_create(...)`, `locations_job_triggers_delete(...)`, `locations_job_triggers_get(...)`, `locations_job_triggers_hybrid_inspect(...)`, `locations_job_triggers_list(...)`, `locations_job_triggers_patch(...)`, `locations_project_data_profiles_get(...)`, `locations_project_data_profiles_list(...)`, `locations_stored_info_types_create(...)`, `locations_stored_info_types_delete(...)`, `locations_stored_info_types_get(...)`, `locations_stored_info_types_list(...)`, `locations_stored_info_types_patch(...)`, `locations_table_data_profiles_delete(...)`, `locations_table_data_profiles_get(...)`, `locations_table_data_profiles_list(...)`, `stored_info_types_create(...)`, `stored_info_types_delete(...)`, `stored_info_types_get(...)`, `stored_info_types_list(...)` and `stored_info_types_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DLP<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/sensitive-data-protection/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn content_deidentify(&self, request: GooglePrivacyDlpV2DeidentifyContentRequest, parent: &str) -> ProjectContentDeidentifyCall<'a, S> {
        ProjectContentDeidentifyCall {
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
    /// Finds potentially sensitive info in content. This method has limits on input size, processing time, and output size. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. For how to guides, see https://cloud.google.com/sensitive-data-protection/docs/inspecting-images and https://cloud.google.com/sensitive-data-protection/docs/inspecting-text,
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn content_inspect(&self, request: GooglePrivacyDlpV2InspectContentRequest, parent: &str) -> ProjectContentInspectCall<'a, S> {
        ProjectContentInspectCall {
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
    /// Re-identifies content that has been de-identified. See https://cloud.google.com/sensitive-data-protection/docs/pseudonymization#re-identification_in_free_text_code_example to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn content_reidentify(&self, request: GooglePrivacyDlpV2ReidentifyContentRequest, parent: &str) -> ProjectContentReidentifyCall<'a, S> {
        ProjectContentReidentifyCall {
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
    /// Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn deidentify_templates_create(&self, request: GooglePrivacyDlpV2CreateDeidentifyTemplateRequest, parent: &str) -> ProjectDeidentifyTemplateCreateCall<'a, S> {
        ProjectDeidentifyTemplateCreateCall {
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
    /// Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn deidentify_templates_delete(&self, name: &str) -> ProjectDeidentifyTemplateDeleteCall<'a, S> {
        ProjectDeidentifyTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn deidentify_templates_get(&self, name: &str) -> ProjectDeidentifyTemplateGetCall<'a, S> {
        ProjectDeidentifyTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn deidentify_templates_list(&self, parent: &str) -> ProjectDeidentifyTemplateListCall<'a, S> {
        ProjectDeidentifyTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn deidentify_templates_patch(&self, request: GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest, name: &str) -> ProjectDeidentifyTemplatePatchCall<'a, S> {
        ProjectDeidentifyTemplatePatchCall {
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
    /// Starts asynchronous cancellation on a long-running DlpJob. The server makes a best effort to cancel the DlpJob, but success is not guaranteed. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the DlpJob resource to be cancelled.
    pub fn dlp_jobs_cancel(&self, request: GooglePrivacyDlpV2CancelDlpJobRequest, name: &str) -> ProjectDlpJobCancelCall<'a, S> {
        ProjectDlpJobCancelCall {
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
    /// Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn dlp_jobs_create(&self, request: GooglePrivacyDlpV2CreateDlpJobRequest, parent: &str) -> ProjectDlpJobCreateCall<'a, S> {
        ProjectDlpJobCreateCall {
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
    /// Deletes a long-running DlpJob. This method indicates that the client is no longer interested in the DlpJob result. The job will be canceled if possible. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DlpJob resource to be deleted.
    pub fn dlp_jobs_delete(&self, name: &str) -> ProjectDlpJobDeleteCall<'a, S> {
        ProjectDlpJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running DlpJob. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DlpJob resource.
    pub fn dlp_jobs_get(&self, name: &str) -> ProjectDlpJobGetCall<'a, S> {
        ProjectDlpJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn dlp_jobs_list(&self, parent: &str) -> ProjectDlpJobListCall<'a, S> {
        ProjectDlpJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/sensitive-data-protection/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn image_redact(&self, request: GooglePrivacyDlpV2RedactImageRequest, parent: &str) -> ProjectImageRedactCall<'a, S> {
        ProjectImageRedactCall {
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
    /// Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn inspect_templates_create(&self, request: GooglePrivacyDlpV2CreateInspectTemplateRequest, parent: &str) -> ProjectInspectTemplateCreateCall<'a, S> {
        ProjectInspectTemplateCreateCall {
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
    /// Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn inspect_templates_delete(&self, name: &str) -> ProjectInspectTemplateDeleteCall<'a, S> {
        ProjectInspectTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn inspect_templates_get(&self, name: &str) -> ProjectInspectTemplateGetCall<'a, S> {
        ProjectInspectTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn inspect_templates_list(&self, parent: &str) -> ProjectInspectTemplateListCall<'a, S> {
        ProjectInspectTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn inspect_templates_patch(&self, request: GooglePrivacyDlpV2UpdateInspectTemplateRequest, name: &str) -> ProjectInspectTemplatePatchCall<'a, S> {
        ProjectInspectTemplatePatchCall {
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
    /// Activate a job trigger. Causes the immediate execute of a trigger instead of waiting on the trigger event to occur.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the trigger to activate, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn job_triggers_activate(&self, request: GooglePrivacyDlpV2ActivateJobTriggerRequest, name: &str) -> ProjectJobTriggerActivateCall<'a, S> {
        ProjectJobTriggerActivateCall {
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
    /// Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn job_triggers_create(&self, request: GooglePrivacyDlpV2CreateJobTriggerRequest, parent: &str) -> ProjectJobTriggerCreateCall<'a, S> {
        ProjectJobTriggerCreateCall {
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
    /// Deletes a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn job_triggers_delete(&self, name: &str) -> ProjectJobTriggerDeleteCall<'a, S> {
        ProjectJobTriggerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn job_triggers_get(&self, name: &str) -> ProjectJobTriggerGetCall<'a, S> {
        ProjectJobTriggerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists job triggers. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn job_triggers_list(&self, parent: &str) -> ProjectJobTriggerListCall<'a, S> {
        ProjectJobTriggerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn job_triggers_patch(&self, request: GooglePrivacyDlpV2UpdateJobTriggerRequest, name: &str) -> ProjectJobTriggerPatchCall<'a, S> {
        ProjectJobTriggerPatchCall {
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
    /// Gets a column data profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name, for example `organizations/12345/locations/us/columnDataProfiles/53234423`.
    pub fn locations_column_data_profiles_get(&self, name: &str) -> ProjectLocationColumnDataProfileGetCall<'a, S> {
        ProjectLocationColumnDataProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists column data profiles for an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`.
    pub fn locations_column_data_profiles_list(&self, parent: &str) -> ProjectLocationColumnDataProfileListCall<'a, S> {
        ProjectLocationColumnDataProfileListCall {
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
    /// Create a Connection to an external data source.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name in the format: "projects/{project}/locations/{location}".
    pub fn locations_connections_create(&self, request: GooglePrivacyDlpV2CreateConnectionRequest, parent: &str) -> ProjectLocationConnectionCreateCall<'a, S> {
        ProjectLocationConnectionCreateCall {
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
    /// Delete a Connection.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the Connection to be deleted, in the format: "projects/{project}/locations/{location}/connections/{connection}".
    pub fn locations_connections_delete(&self, name: &str) -> ProjectLocationConnectionDeleteCall<'a, S> {
        ProjectLocationConnectionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a Connection by name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name in the format: "projects/{project}/locations/{location}/connections/{connection}".
    pub fn locations_connections_get(&self, name: &str) -> ProjectLocationConnectionGetCall<'a, S> {
        ProjectLocationConnectionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Connections in a parent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent name, for example: "projects/project-id/locations/global".
    pub fn locations_connections_list(&self, parent: &str) -> ProjectLocationConnectionListCall<'a, S> {
        ProjectLocationConnectionListCall {
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
    /// Update a Connection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name in the format: "projects/{project}/locations/{location}/connections/{connection}".
    pub fn locations_connections_patch(&self, request: GooglePrivacyDlpV2UpdateConnectionRequest, name: &str) -> ProjectLocationConnectionPatchCall<'a, S> {
        ProjectLocationConnectionPatchCall {
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
    /// Searches for Connections in a parent.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent name, typically an organization, without location. For example: "organizations/12345678".
    pub fn locations_connections_search(&self, parent: &str) -> ProjectLocationConnectionSearchCall<'a, S> {
        ProjectLocationConnectionSearchCall {
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
    /// De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/sensitive-data-protection/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_content_deidentify(&self, request: GooglePrivacyDlpV2DeidentifyContentRequest, parent: &str) -> ProjectLocationContentDeidentifyCall<'a, S> {
        ProjectLocationContentDeidentifyCall {
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
    /// Finds potentially sensitive info in content. This method has limits on input size, processing time, and output size. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. For how to guides, see https://cloud.google.com/sensitive-data-protection/docs/inspecting-images and https://cloud.google.com/sensitive-data-protection/docs/inspecting-text,
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_content_inspect(&self, request: GooglePrivacyDlpV2InspectContentRequest, parent: &str) -> ProjectLocationContentInspectCall<'a, S> {
        ProjectLocationContentInspectCall {
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
    /// Re-identifies content that has been de-identified. See https://cloud.google.com/sensitive-data-protection/docs/pseudonymization#re-identification_in_free_text_code_example to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_content_reidentify(&self, request: GooglePrivacyDlpV2ReidentifyContentRequest, parent: &str) -> ProjectLocationContentReidentifyCall<'a, S> {
        ProjectLocationContentReidentifyCall {
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
    /// Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_deidentify_templates_create(&self, request: GooglePrivacyDlpV2CreateDeidentifyTemplateRequest, parent: &str) -> ProjectLocationDeidentifyTemplateCreateCall<'a, S> {
        ProjectLocationDeidentifyTemplateCreateCall {
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
    /// Deletes a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be deleted, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn locations_deidentify_templates_delete(&self, name: &str) -> ProjectLocationDeidentifyTemplateDeleteCall<'a, S> {
        ProjectLocationDeidentifyTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and deidentify template to be read, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn locations_deidentify_templates_get(&self, name: &str) -> ProjectLocationDeidentifyTemplateGetCall<'a, S> {
        ProjectLocationDeidentifyTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DeidentifyTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_deidentify_templates_list(&self, parent: &str) -> ProjectLocationDeidentifyTemplateListCall<'a, S> {
        ProjectLocationDeidentifyTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the DeidentifyTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and deidentify template to be updated, for example `organizations/433245324/deidentifyTemplates/432452342` or projects/project-id/deidentifyTemplates/432452342.
    pub fn locations_deidentify_templates_patch(&self, request: GooglePrivacyDlpV2UpdateDeidentifyTemplateRequest, name: &str) -> ProjectLocationDeidentifyTemplatePatchCall<'a, S> {
        ProjectLocationDeidentifyTemplatePatchCall {
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
    /// Creates a config for discovery to scan and profile storage.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value is as follows: `projects/`PROJECT_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_discovery_configs_create(&self, request: GooglePrivacyDlpV2CreateDiscoveryConfigRequest, parent: &str) -> ProjectLocationDiscoveryConfigCreateCall<'a, S> {
        ProjectLocationDiscoveryConfigCreateCall {
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
    /// Deletes a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the config, for example `projects/dlp-test-project/discoveryConfigs/53234423`.
    pub fn locations_discovery_configs_delete(&self, name: &str) -> ProjectLocationDiscoveryConfigDeleteCall<'a, S> {
        ProjectLocationDiscoveryConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`.
    pub fn locations_discovery_configs_get(&self, name: &str) -> ProjectLocationDiscoveryConfigGetCall<'a, S> {
        ProjectLocationDiscoveryConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists discovery configurations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value is as follows: `projects/`PROJECT_ID`/locations/`LOCATION_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_discovery_configs_list(&self, parent: &str) -> ProjectLocationDiscoveryConfigListCall<'a, S> {
        ProjectLocationDiscoveryConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the project and the configuration, for example `projects/dlp-test-project/discoveryConfigs/53234423`.
    pub fn locations_discovery_configs_patch(&self, request: GooglePrivacyDlpV2UpdateDiscoveryConfigRequest, name: &str) -> ProjectLocationDiscoveryConfigPatchCall<'a, S> {
        ProjectLocationDiscoveryConfigPatchCall {
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
    /// Starts asynchronous cancellation on a long-running DlpJob. The server makes a best effort to cancel the DlpJob, but success is not guaranteed. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the DlpJob resource to be cancelled.
    pub fn locations_dlp_jobs_cancel(&self, request: GooglePrivacyDlpV2CancelDlpJobRequest, name: &str) -> ProjectLocationDlpJobCancelCall<'a, S> {
        ProjectLocationDlpJobCancelCall {
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
    /// Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_dlp_jobs_create(&self, request: GooglePrivacyDlpV2CreateDlpJobRequest, parent: &str) -> ProjectLocationDlpJobCreateCall<'a, S> {
        ProjectLocationDlpJobCreateCall {
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
    /// Deletes a long-running DlpJob. This method indicates that the client is no longer interested in the DlpJob result. The job will be canceled if possible. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DlpJob resource to be deleted.
    pub fn locations_dlp_jobs_delete(&self, name: &str) -> ProjectLocationDlpJobDeleteCall<'a, S> {
        ProjectLocationDlpJobDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finish a running hybrid DlpJob. Triggers the finalization steps and running of any enabled actions that have not yet run.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the DlpJob resource to be finished.
    pub fn locations_dlp_jobs_finish(&self, request: GooglePrivacyDlpV2FinishDlpJobRequest, name: &str) -> ProjectLocationDlpJobFinishCall<'a, S> {
        ProjectLocationDlpJobFinishCall {
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
    /// Gets the latest state of a long-running DlpJob. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the DlpJob resource.
    pub fn locations_dlp_jobs_get(&self, name: &str) -> ProjectLocationDlpJobGetCall<'a, S> {
        ProjectLocationDlpJobGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inspect hybrid content and store findings to a job. To review the findings, inspect the job. Inspection will occur asynchronously.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the job to execute a hybrid inspect on, for example `projects/dlp-test-project/dlpJob/53234423`.
    pub fn locations_dlp_jobs_hybrid_inspect(&self, request: GooglePrivacyDlpV2HybridInspectDlpJobRequest, name: &str) -> ProjectLocationDlpJobHybridInspectCall<'a, S> {
        ProjectLocationDlpJobHybridInspectCall {
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
    /// Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_dlp_jobs_list(&self, parent: &str) -> ProjectLocationDlpJobListCall<'a, S> {
        ProjectLocationDlpJobListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/sensitive-data-protection/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_image_redact(&self, request: GooglePrivacyDlpV2RedactImageRequest, parent: &str) -> ProjectLocationImageRedactCall<'a, S> {
        ProjectLocationImageRedactCall {
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
    /// Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_inspect_templates_create(&self, request: GooglePrivacyDlpV2CreateInspectTemplateRequest, parent: &str) -> ProjectLocationInspectTemplateCreateCall<'a, S> {
        ProjectLocationInspectTemplateCreateCall {
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
    /// Deletes an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be deleted, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn locations_inspect_templates_delete(&self, name: &str) -> ProjectLocationInspectTemplateDeleteCall<'a, S> {
        ProjectLocationInspectTemplateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and inspectTemplate to be read, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn locations_inspect_templates_get(&self, name: &str) -> ProjectLocationInspectTemplateGetCall<'a, S> {
        ProjectLocationInspectTemplateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists InspectTemplates. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_inspect_templates_list(&self, parent: &str) -> ProjectLocationInspectTemplateListCall<'a, S> {
        ProjectLocationInspectTemplateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the InspectTemplate. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and inspectTemplate to be updated, for example `organizations/433245324/inspectTemplates/432452342` or projects/project-id/inspectTemplates/432452342.
    pub fn locations_inspect_templates_patch(&self, request: GooglePrivacyDlpV2UpdateInspectTemplateRequest, name: &str) -> ProjectLocationInspectTemplatePatchCall<'a, S> {
        ProjectLocationInspectTemplatePatchCall {
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
    /// Activate a job trigger. Causes the immediate execute of a trigger instead of waiting on the trigger event to occur.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the trigger to activate, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_activate(&self, request: GooglePrivacyDlpV2ActivateJobTriggerRequest, name: &str) -> ProjectLocationJobTriggerActivateCall<'a, S> {
        ProjectLocationJobTriggerActivateCall {
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
    /// Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_job_triggers_create(&self, request: GooglePrivacyDlpV2CreateJobTriggerRequest, parent: &str) -> ProjectLocationJobTriggerCreateCall<'a, S> {
        ProjectLocationJobTriggerCreateCall {
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
    /// Deletes a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_delete(&self, name: &str) -> ProjectLocationJobTriggerDeleteCall<'a, S> {
        ProjectLocationJobTriggerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_get(&self, name: &str) -> ProjectLocationJobTriggerGetCall<'a, S> {
        ProjectLocationJobTriggerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inspect hybrid content and store findings to a trigger. The inspection will be processed asynchronously. To review the findings monitor the jobs within the trigger.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the trigger to execute a hybrid inspect on, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_hybrid_inspect(&self, request: GooglePrivacyDlpV2HybridInspectJobTriggerRequest, name: &str) -> ProjectLocationJobTriggerHybridInspectCall<'a, S> {
        ProjectLocationJobTriggerHybridInspectCall {
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
    /// Lists job triggers. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_job_triggers_list(&self, parent: &str) -> ProjectLocationJobTriggerListCall<'a, S> {
        ProjectLocationJobTriggerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a job trigger. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of the project and the triggeredJob, for example `projects/dlp-test-project/jobTriggers/53234423`.
    pub fn locations_job_triggers_patch(&self, request: GooglePrivacyDlpV2UpdateJobTriggerRequest, name: &str) -> ProjectLocationJobTriggerPatchCall<'a, S> {
        ProjectLocationJobTriggerPatchCall {
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
    /// Gets a project data profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name, for example `organizations/12345/locations/us/projectDataProfiles/53234423`.
    pub fn locations_project_data_profiles_get(&self, name: &str) -> ProjectLocationProjectDataProfileGetCall<'a, S> {
        ProjectLocationProjectDataProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists project data profiles for an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. organizations/{org_id}/locations/{loc_id}
    pub fn locations_project_data_profiles_list(&self, parent: &str) -> ProjectLocationProjectDataProfileListCall<'a, S> {
        ProjectLocationProjectDataProfileListCall {
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
    /// Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_stored_info_types_create(&self, request: GooglePrivacyDlpV2CreateStoredInfoTypeRequest, parent: &str) -> ProjectLocationStoredInfoTypeCreateCall<'a, S> {
        ProjectLocationStoredInfoTypeCreateCall {
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
    /// Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn locations_stored_info_types_delete(&self, name: &str) -> ProjectLocationStoredInfoTypeDeleteCall<'a, S> {
        ProjectLocationStoredInfoTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn locations_stored_info_types_get(&self, name: &str) -> ProjectLocationStoredInfoTypeGetCall<'a, S> {
        ProjectLocationStoredInfoTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn locations_stored_info_types_list(&self, parent: &str) -> ProjectLocationStoredInfoTypeListCall<'a, S> {
        ProjectLocationStoredInfoTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn locations_stored_info_types_patch(&self, request: GooglePrivacyDlpV2UpdateStoredInfoTypeRequest, name: &str) -> ProjectLocationStoredInfoTypePatchCall<'a, S> {
        ProjectLocationStoredInfoTypePatchCall {
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
    /// Delete a TableDataProfile. Will not prevent the profile from being regenerated if the table is still included in a discovery configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the table data profile.
    pub fn locations_table_data_profiles_delete(&self, name: &str) -> ProjectLocationTableDataProfileDeleteCall<'a, S> {
        ProjectLocationTableDataProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a table data profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name, for example `organizations/12345/locations/us/tableDataProfiles/53234423`.
    pub fn locations_table_data_profiles_get(&self, name: &str) -> ProjectLocationTableDataProfileGetCall<'a, S> {
        ProjectLocationTableDataProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists table data profiles for an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the organization or project, for example `organizations/433245324/locations/europe` or `projects/project-id/locations/asia`.
    pub fn locations_table_data_profiles_list(&self, parent: &str) -> ProjectLocationTableDataProfileListCall<'a, S> {
        ProjectLocationTableDataProfileListCall {
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
    /// Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID + Organizations scope, location specified: `organizations/`ORG_ID`/locations/`LOCATION_ID + Organizations scope, no location specified (defaults to global): `organizations/`ORG_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn stored_info_types_create(&self, request: GooglePrivacyDlpV2CreateStoredInfoTypeRequest, parent: &str) -> ProjectStoredInfoTypeCreateCall<'a, S> {
        ProjectStoredInfoTypeCreateCall {
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
    /// Deletes a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be deleted, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn stored_info_types_delete(&self, name: &str) -> ProjectStoredInfoTypeDeleteCall<'a, S> {
        ProjectStoredInfoTypeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a stored infoType. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the organization and storedInfoType to be read, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn stored_info_types_get(&self, name: &str) -> ProjectStoredInfoTypeGetCall<'a, S> {
        ProjectStoredInfoTypeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists stored infoTypes. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/`PROJECT_ID`/locations/`LOCATION_ID + Projects scope, no location specified (defaults to global): `projects/`PROJECT_ID The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
    pub fn stored_info_types_list(&self, parent: &str) -> ProjectStoredInfoTypeListCall<'a, S> {
        ProjectStoredInfoTypeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _location_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name of organization and storedInfoType to be updated, for example `organizations/433245324/storedInfoTypes/432452342` or projects/project-id/storedInfoTypes/432452342.
    pub fn stored_info_types_patch(&self, request: GooglePrivacyDlpV2UpdateStoredInfoTypeRequest, name: &str) -> ProjectStoredInfoTypePatchCall<'a, S> {
        ProjectStoredInfoTypePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



