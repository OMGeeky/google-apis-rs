use super::*;
/// A builder providing access to all methods supported on *applicationDetailService* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_apk_details(...)`
/// // to build up your call.
/// let rb = hub.application_detail_service();
/// # }
/// ```
pub struct ApplicationDetailServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for ApplicationDetailServiceMethods<'a, S> {}

impl<'a, S> ApplicationDetailServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of an Android application APK.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_apk_details(&self, request: FileReference) -> ApplicationDetailServiceGetApkDetailCall<'a, S> {
        ApplicationDetailServiceGetApkDetailCall {
            hub: self.hub,
            _request: request,
            _bundle_location_gcs_path: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `device_sessions_cancel(...)`, `device_sessions_create(...)`, `device_sessions_get(...)`, `device_sessions_list(...)`, `device_sessions_patch(...)`, `test_matrices_cancel(...)`, `test_matrices_create(...)` and `test_matrices_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// POST /v1/projects/{project_id}/deviceSessions/{device_session_id}:cancel Changes the DeviceSession to state FINISHED and terminates all connections. Canceled sessions are not deleted and can be retrieved or listed by the user until they expire based on the 28 day deletion policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub fn device_sessions_cancel(&self, request: CancelDeviceSessionRequest, name: &str) -> ProjectDeviceSessionCancelCall<'a, S> {
        ProjectDeviceSessionCancelCall {
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
    /// POST /v1/projects/{project_id}/deviceSessions
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Compute Engine project under which this device will be allocated. "projects/{project_id}"
    pub fn device_sessions_create(&self, request: DeviceSession, parent: &str) -> ProjectDeviceSessionCreateCall<'a, S> {
        ProjectDeviceSessionCreateCall {
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
    /// GET /v1/projects/{project_id}/deviceSessions/{device_session_id} Return a DeviceSession, which documents the allocation status and whether the device is allocated. Clients making requests from this API must poll GetDeviceSession.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub fn device_sessions_get(&self, name: &str) -> ProjectDeviceSessionGetCall<'a, S> {
        ProjectDeviceSessionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// GET /v1/projects/{project_id}/deviceSessions Lists device Sessions owned by the project user.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent to request, e.g. "projects/{project_id}"
    pub fn device_sessions_list(&self, parent: &str) -> ProjectDeviceSessionListCall<'a, S> {
        ProjectDeviceSessionListCall {
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
    /// PATCH /v1/projects/{projectId}/deviceSessions/deviceSessionId}:updateDeviceSession Updates the current device session to the fields described by the update_mask.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub fn device_sessions_patch(&self, request: DeviceSession, name: &str) -> ProjectDeviceSessionPatchCall<'a, S> {
        ProjectDeviceSessionPatchCall {
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
    /// Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test.
    /// * `testMatrixId` - Test matrix that will be canceled.
    pub fn test_matrices_cancel(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceCancelCall<'a, S> {
        ProjectTestMatriceCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. The returned matrix will not yet contain the executions that will be created for this matrix. Execution creation happens later on and will require a call to GetTestMatrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The GCE project under which this job will run.
    pub fn test_matrices_create(&self, request: TestMatrix, project_id: &str) -> ProjectTestMatriceCreateCall<'a, S> {
        ProjectTestMatriceCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks the status of a test matrix and the executions once they are created. The test matrix will contain the list of test executions to run if and only if the resultStorage.toolResultsExecution fields have been populated. Note: Flaky test executions may be added to the matrix at a later stage. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test matrix.
    /// * `testMatrixId` - Unique test matrix id which was assigned by the service.
    pub fn test_matrices_get(&self, project_id: &str, test_matrix_id: &str) -> ProjectTestMatriceGetCall<'a, S> {
        ProjectTestMatriceGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *testEnvironmentCatalog* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use testing1::{Testing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Testing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.test_environment_catalog();
/// # }
/// ```
pub struct TestEnvironmentCatalogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Testing<S>,
}

impl<'a, S> client::MethodsBuilder for TestEnvironmentCatalogMethods<'a, S> {}

impl<'a, S> TestEnvironmentCatalogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred
    /// 
    /// # Arguments
    ///
    /// * `environmentType` - Required. The type of environment that should be listed.
    pub fn get(&self, environment_type: &TestEnvironmentCatalogEnvironmentTypeEnum) -> TestEnvironmentCatalogGetCall<'a, S> {
        TestEnvironmentCatalogGetCall {
            hub: self.hub,
            _environment_type: environment_type.clone(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



