initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["AuditConfig","Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."],["AuditLogConfig","Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."],["AuthorizationLoggingOptions","Authorization-related information used by Cloud Audit Logging."],["Binding","Associates `members` with a `role`."],["CancelOperationRequest","The request message for Operations.CancelOperation."],["CloudAuditOptions","Write a Cloud Audit log"],["Condition","A condition to be met."],["CounterOptions","Increment a streamz counter with the specified metric and field names. Metric names should start with a '/', generally be lowercase-only, and end in \"_count\". Field names should not contain an initial slash. The actual exported metric names will have \"/iam/policy\" prepended. Field names correspond to IAM request parameters and field values are their respective values. Supported field names: - \"authority\", which is \"[token]\" if IAMContext.token is present, otherwise the value of IAMContext.authority_selector if present, and otherwise a representation of IAMContext.principal; or - \"iam_principal\", a representation of IAMContext.principal even if a token or authority selector is present; or - \"\" (empty string), resulting in a counter with no fields. Examples: counter { metric: \"/debug_access_count\" field: \"iam_principal\" } ==> increment counter /iam/policy/debug_access_count {iam_principal=[value of IAMContext.principal]}"],["CustomField","Custom fields. These can be used to create a counter with arbitrary field/value pairs. See: go/rpcsp-custom-fields."],["DataAccessOptions","Write a Data Access (Gin) log"],["DeployedClusterState","The game server cluster changes made by the game server deployment."],["DeployedFleet","Agones fleet specification and details."],["DeployedFleetAutoscaler","Details about the Agones autoscaler."],["DeployedFleetDetails","Details of the deployed Agones fleet."],["DeployedFleetStatus","DeployedFleetStatus has details about the Agones fleets such as how many are running, how many allocated, and so on."],["Empty","A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."],["Expr","Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."],["FetchDeploymentStateRequest","Request message for GameServerDeploymentsService.FetchDeploymentState."],["FetchDeploymentStateResponse","Response message for GameServerDeploymentsService.FetchDeploymentState."],["FleetConfig","Fleet configs for Agones."],["GameServerCluster","A game server cluster resource."],["GameServerClusterConnectionInfo","The game server cluster connection information."],["GameServerConfig","A game server config resource."],["GameServerConfigOverride","A game server config override."],["GameServerDeployment","A game server deployment resource."],["GameServerDeploymentRollout","The game server deployment rollout which represents the desired rollout state."],["GameServices","Central instance to access all GameServices related resource activities"],["GkeClusterReference","A reference to a GKE cluster."],["LabelSelector","The label selector, used to group labels on the resources."],["ListGameServerClustersResponse","Response message for GameServerClustersService.ListGameServerClusters."],["ListGameServerConfigsResponse","Response message for GameServerConfigsService.ListGameServerConfigs."],["ListGameServerDeploymentsResponse","Response message for GameServerDeploymentsService.ListGameServerDeployments."],["ListLocationsResponse","The response message for Locations.ListLocations."],["ListOperationsResponse","The response message for Operations.ListOperations."],["ListRealmsResponse","Response message for RealmsService.ListRealms."],["Location","A resource that represents Google Cloud Platform location."],["LogConfig","Specifies what kind of log the caller must write"],["Operation","This resource represents a long-running operation that is the result of a network API call."],["Policy","An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation. JSON example: { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the IAM documentation."],["PreviewCreateGameServerClusterResponse","Response message for GameServerClustersService.PreviewCreateGameServerCluster."],["PreviewDeleteGameServerClusterResponse","Response message for GameServerClustersService.PreviewDeleteGameServerCluster."],["PreviewGameServerDeploymentRolloutResponse","Response message for PreviewGameServerDeploymentRollout. This has details about the Agones fleet and autoscaler to be actuated."],["PreviewRealmUpdateResponse","Response message for RealmsService.PreviewRealmUpdate."],["PreviewUpdateGameServerClusterResponse","Response message for GameServerClustersService.PreviewUpdateGameServerCluster"],["ProjectLocationGameServerDeploymentConfigCreateCall","Creates a new game server config in a given project, location, and game server deployment. Game server configs are immutable, and are not applied until referenced in the game server deployment rollout resource."],["ProjectLocationGameServerDeploymentConfigDeleteCall","Deletes a single game server config. The deletion will fail if the game server config is referenced in a game server deployment rollout."],["ProjectLocationGameServerDeploymentConfigGetCall","Gets details of a single game server config."],["ProjectLocationGameServerDeploymentConfigListCall","Lists game server configs in a given project, location, and game server deployment."],["ProjectLocationGameServerDeploymentCreateCall","Creates a new game server deployment in a given project and location."],["ProjectLocationGameServerDeploymentDeleteCall","Deletes a single game server deployment."],["ProjectLocationGameServerDeploymentFetchDeploymentStateCall","Retrieves information about the current state of the game server deployment. Gathers all the Agones fleets and Agones autoscalers, including fleets running an older version of the game server deployment."],["ProjectLocationGameServerDeploymentGetCall","Gets details of a single game server deployment."],["ProjectLocationGameServerDeploymentGetIamPolicyCall","Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set."],["ProjectLocationGameServerDeploymentGetRolloutCall","Gets details a single game server deployment rollout."],["ProjectLocationGameServerDeploymentListCall","Lists game server deployments in a given project and location."],["ProjectLocationGameServerDeploymentPatchCall","Patches a game server deployment."],["ProjectLocationGameServerDeploymentPreviewRolloutCall","Previews the game server deployment rollout. This API does not mutate the rollout resource."],["ProjectLocationGameServerDeploymentSetIamPolicyCall","Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."],["ProjectLocationGameServerDeploymentTestIamPermissionCall","Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning."],["ProjectLocationGameServerDeploymentUpdateRolloutCall","Patches a single game server deployment rollout. The method will not return an error if the update does not affect any existing realms. For example - if the default_game_server_config is changed but all existing realms use the override, that is valid. Similarly, if a non existing realm is explicitly called out in game_server_config_overrides field, that will also not result in an error."],["ProjectLocationGetCall","Gets information about a location."],["ProjectLocationListCall","Lists information about the supported locations for this service."],["ProjectLocationOperationCancelCall","Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."],["ProjectLocationOperationDeleteCall","Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."],["ProjectLocationOperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["ProjectLocationOperationListCall","Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."],["ProjectLocationRealmCreateCall","Creates a new realm in a given project and location."],["ProjectLocationRealmDeleteCall","Deletes a single realm."],["ProjectLocationRealmGameServerClusterCreateCall","Creates a new game server cluster in a given project and location."],["ProjectLocationRealmGameServerClusterDeleteCall","Deletes a single game server cluster."],["ProjectLocationRealmGameServerClusterGetCall","Gets details of a single game server cluster."],["ProjectLocationRealmGameServerClusterListCall","Lists game server clusters in a given project and location."],["ProjectLocationRealmGameServerClusterPatchCall","Patches a single game server cluster."],["ProjectLocationRealmGameServerClusterPreviewCreateCall","Previews creation of a new game server cluster in a given project and location."],["ProjectLocationRealmGameServerClusterPreviewDeleteCall","Previews deletion of a single game server cluster."],["ProjectLocationRealmGameServerClusterPreviewUpdateCall","Previews updating a GameServerCluster."],["ProjectLocationRealmGetCall","Gets details of a single realm."],["ProjectLocationRealmListCall","Lists realms in a given project and location."],["ProjectLocationRealmPatchCall","Patches a single realm."],["ProjectLocationRealmPreviewUpdateCall","Previews patches to a single realm."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `GameServices` hub."],["Realm","A realm resource."],["RealmSelector","The realm selector, used to match realm resources."],["Rule","A rule to be applied in a Policy."],["ScalingConfig","Autoscaling config for an Agones fleet."],["Schedule","The schedule of a recurring or one time event. The event's time span is specified by start_time and end_time. If the scheduled event's timespan is larger than the cron_spec + cron_job_duration, the event will be recurring. If only cron_spec + cron_job_duration are specified, the event is effective starting at the local time specified by cron_spec, and is recurring. start_time|-------[cron job]-------[cron job]-------[cron job]---|end_time cron job: cron spec start time + duration"],["SetIamPolicyRequest","Request message for `SetIamPolicy` method."],["SpecSource","Encapsulates Agones fleet spec and Agones autoscaler spec sources."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["TargetDetails","Details about the Agones resources."],["TargetFleet","Target Agones fleet specification."],["TargetFleetAutoscaler","Target Agones autoscaler policy reference."],["TargetFleetDetails","Details of the target Agones fleet."],["TargetState","Encapsulates the Target state."],["TestIamPermissionsRequest","Request message for `TestIamPermissions` method."],["TestIamPermissionsResponse","Response message for `TestIamPermissions` method."]]});