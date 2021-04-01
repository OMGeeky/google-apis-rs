initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["FolderLocationReplayCreateCall","Creates and starts a Replay using the given ReplayConfig."],["FolderLocationReplayGetCall","Gets the specified Replay. Each `Replay` is available for at least 7 days."],["FolderLocationReplayResultListCall","Lists the results of running a Replay."],["FolderMethods","A builder providing access to all methods supported on folder resources. It is not used directly, but through the `PolicySimulator` hub."],["GoogleCloudPolicysimulatorV1AccessStateDiff","A summary and comparison of the member's access under the current (baseline) policies and the proposed (simulated) policies for a single access tuple."],["GoogleCloudPolicysimulatorV1AccessTuple","Information about the member, resource, and permission to check."],["GoogleCloudPolicysimulatorV1BindingExplanation","Details about how a binding in a policy affects a member's ability to use a permission."],["GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembership","Details about whether the binding includes the member."],["GoogleCloudPolicysimulatorV1ExplainedAccess","Details about how a set of policies, listed in ExplainedPolicy, resulted in a certain AccessState when replaying an access tuple."],["GoogleCloudPolicysimulatorV1ExplainedPolicy","Details about how a specific IAM Policy contributed to the access check."],["GoogleCloudPolicysimulatorV1ListReplayResultsResponse","Response message for Simulator.ListReplayResults."],["GoogleCloudPolicysimulatorV1Replay","A resource describing a `Replay`, or simulation."],["GoogleCloudPolicysimulatorV1ReplayConfig","The configuration used for a Replay."],["GoogleCloudPolicysimulatorV1ReplayDiff","The difference between the results of evaluating an access tuple under the current (baseline) policies and under the proposed (simulated) policies. This difference explains how a member's access could change if the proposed policies were applied."],["GoogleCloudPolicysimulatorV1ReplayResult","The result of replaying a single access tuple against a simulated state."],["GoogleCloudPolicysimulatorV1ReplayResultsSummary","Summary statistics about the replayed log entries."],["GoogleIamV1AuditConfig","Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."],["GoogleIamV1AuditLogConfig","Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."],["GoogleIamV1Binding","Associates `members` with a `role`."],["GoogleIamV1Policy","An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation. JSON example: { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the IAM documentation."],["GoogleLongrunningListOperationsResponse","The response message for Operations.ListOperations."],["GoogleLongrunningOperation","This resource represents a long-running operation that is the result of a network API call."],["GoogleRpcStatus","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."],["GoogleTypeDate","Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."],["GoogleTypeExpr","Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."],["OperationGetCall","Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."],["OperationListCall","Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."],["OperationMethods","A builder providing access to all methods supported on operation resources. It is not used directly, but through the `PolicySimulator` hub."],["OrganizationLocationReplayCreateCall","Creates and starts a Replay using the given ReplayConfig."],["OrganizationLocationReplayGetCall","Gets the specified Replay. Each `Replay` is available for at least 7 days."],["OrganizationLocationReplayResultListCall","Lists the results of running a Replay."],["OrganizationMethods","A builder providing access to all methods supported on organization resources. It is not used directly, but through the `PolicySimulator` hub."],["PolicySimulator","Central instance to access all PolicySimulator related resource activities"],["ProjectLocationReplayCreateCall","Creates and starts a Replay using the given ReplayConfig."],["ProjectLocationReplayGetCall","Gets the specified Replay. Each `Replay` is available for at least 7 days."],["ProjectLocationReplayResultListCall","Lists the results of running a Replay."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `PolicySimulator` hub."]]});