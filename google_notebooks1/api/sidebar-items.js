initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AIPlatformNotebooks","Central instance to access all AIPlatformNotebooks related resource activities"],["AcceleratorConfig","Definition of a hardware accelerator. Note that not all combinations of `type` and `core_count` are valid. Check GPUs on Compute Engine to find a valid combination. TPUs are not supported."],["Binding","Associates `members` with a `role`."],["CancelOperationRequest","The request message for Operations.CancelOperation."],["ContainerImage","Definition of a container image for starting a notebook instance with the environment installed in a container."],["Disk","An instance-attached disk resource."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["Environment","Definition of a software environment that is used to start a notebook instance."],["Execution","The definition of a single executed notebook."],["ExecutionTemplate","The description a notebook execution workload."],["Expr","Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."],["GetInstanceHealthResponse","Response for checking if a notebook instance is healthy."],["GuestOsFeature","Guest OS features for boot disk."],["Instance","The definition of a notebook instance."],["IsInstanceUpgradeableResponse","Response for checking if a notebook instance is upgradeable."],["ListEnvironmentsResponse","Response for listing environments."],["ListExecutionsResponse","Response for listing scheduled notebook executions"],["ListInstancesResponse","Response for listing notebook instances."],["ListLocationsResponse","The response message for Locations.ListLocations."],["ListOperationsResponse","The response message for Operations.ListOperations."],["ListSchedulesResponse","Response for listing scheduled notebook job."],["Location","A resource that represents Google Cloud Platform location."],["Operation","This resource represents a long-running operation that is the result of a network API call."],["Policy","An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation. JSON example: { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the IAM documentation."],["ProjectLocationEnvironmentCreateCall","Creates a new Environment."],["ProjectLocationEnvironmentDeleteCall","Deletes a single Environment."],["ProjectLocationEnvironmentGetCall","Gets details of a single Environment."],["ProjectLocationEnvironmentListCall","Lists environments in a project."],["ProjectLocationExecutionCreateCall","Creates a new Scheduled Notebook in a given project and location."],["ProjectLocationExecutionDeleteCall","Deletes execution"],["ProjectLocationExecutionGetCall","Gets details of executions"],["ProjectLocationExecutionListCall","Lists executions in a given project and location"],["ProjectLocationGetCall","Gets information about a location."],["ProjectLocationInstanceCreateCall","Creates a new Instance in a given project and location."],["ProjectLocationInstanceDeleteCall","Deletes a single Instance."],["ProjectLocationInstanceGetCall","Gets details of a single Instance."],["ProjectLocationInstanceGetIamPolicyCall","Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."],["ProjectLocationInstanceGetInstanceHealthCall","Check if a notebook instance is healthy."],["ProjectLocationInstanceIsUpgradeableCall","Check if a notebook instance is upgradable."],["ProjectLocationInstanceListCall","Lists instances in a given project and location."],["ProjectLocationInstanceRegisterCall","Registers an existing legacy notebook instance to the Notebooks API server. Legacy instances are instances created with the legacy Compute Engine calls. They are not manageable by the Notebooks API out of the box. This call makes these instances manageable by the Notebooks API."],["ProjectLocationInstanceReportCall","Allows notebook instances to report their latest instance information to the Notebooks API server. The server will merge the reported information to the instance metadata store. Do not use this method directly."],["ProjectLocationInstanceResetCall","Resets a notebook instance."],["ProjectLocationInstanceSetAcceleratorCall","Updates the guest accelerators of a single Instance."],["ProjectLocationInstanceSetIamPolicyCall","Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."],["ProjectLocationInstanceSetLabelCall","Replaces all the labels of an Instance."],["ProjectLocationInstanceSetMachineTypeCall","Updates the machine type of a single Instance."],["ProjectLocationInstanceStartCall","Starts a notebook instance."],["ProjectLocationInstanceStopCall","Stops a notebook instance."],["ProjectLocationInstanceTestIamPermissionCall","Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning."],["ProjectLocationInstanceUpgradeCall","Upgrades a notebook instance to the latest version."],["ProjectLocationInstanceUpgradeInternalCall","Allows notebook instances to call this endpoint to upgrade themselves. Do not use this method directly."],["ProjectLocationListCall","Lists information about the supported locations for this service."],["ProjectLocationOperationCancelCall","Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."],["ProjectLocationOperationDeleteCall","Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."],["ProjectLocationOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["ProjectLocationOperationListCall","Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."],["ProjectLocationScheduleCreateCall","Creates a new Scheduled Notebook in a given project and location."],["ProjectLocationScheduleDeleteCall","Deletes schedule and all underlying jobs"],["ProjectLocationScheduleGetCall","Gets details of schedule"],["ProjectLocationScheduleListCall","Lists schedules in a given project and location."],["ProjectLocationScheduleTriggerCall","Triggers execution of an existing schedule."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `AIPlatformNotebooks` hub."],["RegisterInstanceRequest","Request for registering a notebook instance."],["ReportInstanceInfoRequest","Request for notebook instances to report information to Notebooks API."],["ResetInstanceRequest","Request for reseting a notebook instance"],["Schedule","The definition of a schedule."],["SchedulerAcceleratorConfig","Definition of a hardware accelerator. Note that not all combinations of `type` and `core_count` are valid. Check GPUs on Compute Engine to find a valid combination. TPUs are not supported."],["SetIamPolicyRequest","Request message for `SetIamPolicy` method."],["SetInstanceAcceleratorRequest","Request for setting instance accelerator."],["SetInstanceLabelsRequest","Request for setting instance labels."],["SetInstanceMachineTypeRequest","Request for setting instance machine type."],["ShieldedInstanceConfig","A set of Shielded Instance options. Check [Images using supported Shielded VM features] Not all combinations are valid."],["StartInstanceRequest","Request for starting a notebook instance"],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["StopInstanceRequest","Request for stopping a notebook instance"],["TestIamPermissionsRequest","Request message for `TestIamPermissions` method."],["TestIamPermissionsResponse","Response message for `TestIamPermissions` method."],["TriggerScheduleRequest","Request for created scheduled notebooks"],["UpgradeHistoryEntry","The entry of VM image upgrade history."],["UpgradeInstanceInternalRequest","Request for upgrading a notebook instance from within the VM"],["UpgradeInstanceRequest","Request for upgrading a notebook instance"],["VmImage","Definition of a custom Compute Engine virtual machine image for starting a notebook instance with the environment installed directly on the VM."]]});