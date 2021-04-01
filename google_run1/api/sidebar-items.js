initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["Addressable","Information for connecting over HTTP(s)."],["AuditConfig","Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."],["AuditLogConfig","Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."],["AuthorizedDomain","A domain that a user has been authorized to administer. To authorize use of a domain, verify ownership via Webmaster Central."],["Binding","Associates `members` with a `role`."],["CloudRun","Central instance to access all CloudRun related resource activities"],["ConfigMapEnvSource","Cloud Run fully managed: not supported Cloud Run for Anthos: supported ConfigMapEnvSource selects a ConfigMap to populate the environment variables with. The contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables."],["ConfigMapKeySelector","Cloud Run fully managed: not supported Cloud Run for Anthos: supported Selects a key from a ConfigMap."],["ConfigMapVolumeSource","Cloud Run fully managed: not supported Cloud Run for Anthos: supported Adapts a ConfigMap into a volume. The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths."],["Configuration","Configuration represents the \"floating HEAD\" of a linear history of Revisions, and optionally how the containers those revisions reference are built. Users create new Revisions by updating the Configuration's spec. The \"latest created\" revision's name is available under status, as is the \"latest ready\" revision's name. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#configuration"],["ConfigurationSpec","ConfigurationSpec holds the desired state of the Configuration (from the client)."],["ConfigurationStatus","ConfigurationStatus communicates the observed state of the Configuration (from the controller)."],["Container","A single application container. This specifies both the container to run, the command to run in the container and the arguments to supply to it. Note that additional arguments may be supplied by the system to the container at runtime."],["ContainerPort","ContainerPort represents a network port in a single container."],["DomainMapping","Resource to hold the state and status of a user's domain mapping. NOTE: This resource is currently in Beta."],["DomainMappingSpec","The desired state of the Domain Mapping."],["DomainMappingStatus","The current state of the Domain Mapping."],["EnvFromSource","Cloud Run fully managed: not supported Cloud Run for Anthos: supported EnvFromSource represents the source of a set of ConfigMaps"],["EnvVar","EnvVar represents an environment variable present in a Container."],["EnvVarSource","Cloud Run fully managed: not supported Cloud Run for Anthos: supported EnvVarSource represents a source for the value of an EnvVar."],["ExecAction","Cloud Run fully managed: not supported Cloud Run for Anthos: supported ExecAction describes a \"run in container\" action."],["Expr","Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."],["GoogleCloudRunV1Condition","Condition defines a generic condition for a Resource"],["HTTPGetAction","Cloud Run fully managed: not supported Cloud Run for Anthos: supported HTTPGetAction describes an action based on HTTP Get requests."],["HTTPHeader","Cloud Run fully managed: not supported Cloud Run for Anthos: supported HTTPHeader describes a custom header to be used in HTTP probes"],["KeyToPath","Cloud Run fully managed: supported Cloud Run for Anthos: supported Maps a string key to a path within a volume."],["ListAuthorizedDomainsResponse","A list of Authorized Domains."],["ListConfigurationsResponse","ListConfigurationsResponse is a list of Configuration resources."],["ListDomainMappingsResponse","ListDomainMappingsResponse is a list of DomainMapping resources."],["ListLocationsResponse","The response message for Locations.ListLocations."],["ListMeta","ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}."],["ListRevisionsResponse","ListRevisionsResponse is a list of Revision resources."],["ListRoutesResponse","ListRoutesResponse is a list of Route resources."],["ListServicesResponse","A list of Service resources."],["LocalObjectReference","Cloud Run fully managed: not supported Cloud Run for Anthos: supported LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace."],["Location","A resource that represents Google Cloud Platform location."],["NamespaceAuthorizeddomainListCall","List authorized domains."],["NamespaceConfigurationGetCall","Get information about a configuration."],["NamespaceConfigurationListCall","List configurations."],["NamespaceDomainmappingCreateCall","Create a new domain mapping."],["NamespaceDomainmappingDeleteCall","Delete a domain mapping."],["NamespaceDomainmappingGetCall","Get information about a domain mapping."],["NamespaceDomainmappingListCall","List domain mappings."],["NamespaceMethods","A builder providing access to all methods supported on namespace resources. It is not used directly, but through the `CloudRun` hub."],["NamespaceRevisionDeleteCall","Delete a revision."],["NamespaceRevisionGetCall","Get information about a revision."],["NamespaceRevisionListCall","List revisions."],["NamespaceRouteGetCall","Get information about a route."],["NamespaceRouteListCall","List routes."],["NamespaceServiceCreateCall","Create a service."],["NamespaceServiceDeleteCall","Delete a service. This will cause the Service to stop serving traffic and will delete the child entities like Routes, Configurations and Revisions."],["NamespaceServiceGetCall","Get information about a service."],["NamespaceServiceListCall","List services."],["NamespaceServiceReplaceServiceCall","Replace a service. Only the spec and metadata labels and annotations are modifiable. After the Update request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control."],["ObjectMeta","k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create."],["OwnerReference","OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field."],["Policy","An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation. JSON example: { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the IAM documentation."],["Probe","Cloud Run fully managed: not supported Cloud Run for Anthos: supported Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic."],["ProjectAuthorizeddomainListCall","List authorized domains."],["ProjectLocationAuthorizeddomainListCall","List authorized domains."],["ProjectLocationConfigurationGetCall","Get information about a configuration."],["ProjectLocationConfigurationListCall","List configurations."],["ProjectLocationDomainmappingCreateCall","Create a new domain mapping."],["ProjectLocationDomainmappingDeleteCall","Delete a domain mapping."],["ProjectLocationDomainmappingGetCall","Get information about a domain mapping."],["ProjectLocationDomainmappingListCall","List domain mappings."],["ProjectLocationListCall","Lists information about the supported locations for this service."],["ProjectLocationRevisionDeleteCall","Delete a revision."],["ProjectLocationRevisionGetCall","Get information about a revision."],["ProjectLocationRevisionListCall","List revisions."],["ProjectLocationRouteGetCall","Get information about a route."],["ProjectLocationRouteListCall","List routes."],["ProjectLocationServiceCreateCall","Create a service."],["ProjectLocationServiceDeleteCall","Delete a service. This will cause the Service to stop serving traffic and will delete the child entities like Routes, Configurations and Revisions."],["ProjectLocationServiceGetCall","Get information about a service."],["ProjectLocationServiceGetIamPolicyCall","Get the IAM Access Control policy currently in effect for the given Cloud Run service. This result does not include any inherited policies."],["ProjectLocationServiceListCall","List services."],["ProjectLocationServiceReplaceServiceCall","Replace a service. Only the spec and metadata labels and annotations are modifiable. After the Update request, Cloud Run will work to make the 'status' match the requested 'spec'. May provide metadata.resourceVersion to enforce update from last read for optimistic concurrency control."],["ProjectLocationServiceSetIamPolicyCall","Sets the IAM Access control policy for the specified Service. Overwrites any existing policy."],["ProjectLocationServiceTestIamPermissionCall","Returns permissions that a caller has on the specified Project. There are no permissions required for making this API call."],["ProjectMethods","A builder providing access to all methods supported on project resources. It is not used directly, but through the `CloudRun` hub."],["ResourceRecord","A DNS resource record."],["ResourceRequirements","ResourceRequirements describes the compute resource requirements."],["Revision","Revision is an immutable snapshot of code and configuration. A revision references a container image. Revisions are created by updates to a Configuration. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#revision"],["RevisionSpec","RevisionSpec holds the desired state of the Revision (from the client)."],["RevisionStatus","RevisionStatus communicates the observed state of the Revision (from the controller)."],["RevisionTemplate","RevisionTemplateSpec describes the data a revision should have when created from a template. Based on: https://github.com/kubernetes/api/blob/e771f807/core/v1/types.go#L3179-L3190"],["Route","Route is responsible for configuring ingress over a collection of Revisions. Some of the Revisions a Route distributes traffic over may be specified by referencing the Configuration responsible for creating them; in these cases the Route is additionally responsible for monitoring the Configuration for \"latest ready\" revision changes, and smoothly rolling out latest revisions. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#route Cloud Run currently supports referencing a single Configuration to automatically deploy the \"latest ready\" Revision from that Configuration."],["RouteSpec","RouteSpec holds the desired state of the Route (from the client)."],["RouteStatus","RouteStatus communicates the observed state of the Route (from the controller)."],["SecretEnvSource","Cloud Run fully managed: not supported Cloud Run for Anthos: supported SecretEnvSource selects a Secret to populate the environment variables with. The contents of the target Secret's Data field will represent the key-value pairs as environment variables."],["SecretKeySelector","Cloud Run fully managed: not supported Cloud Run for Anthos: supported SecretKeySelector selects a key of a Secret."],["SecretVolumeSource","Cloud Run fully managed: supported The secret's value will be presented as the content of a file whose name is defined in the item path. If no items are defined, the name of the file is the secret_name. Cloud Run for Anthos: supported The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names."],["SecurityContext","Cloud Run fully managed: not supported Cloud Run for Anthos: supported SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext. When both are set, the values in SecurityContext take precedence."],["Service","Service acts as a top-level container that manages a set of Routes and Configurations which implement a network service. Service exists to provide a singular abstraction which can be access controlled, reasoned about, and which encapsulates software lifecycle decisions such as rollout policy and team resource ownership. Service acts only as an orchestrator of the underlying Routes and Configurations (much as a kubernetes Deployment orchestrates ReplicaSets). The Service's controller will track the statuses of its owned Configuration and Route, reflecting their statuses and conditions as its own. See also: https://github.com/knative/serving/blob/master/docs/spec/overview.md#service"],["ServiceSpec","ServiceSpec holds the desired state of the Route (from the client), which is used to manipulate the underlying Route and Configuration(s)."],["ServiceStatus","The current state of the Service. Output only."],["SetIamPolicyRequest","Request message for `SetIamPolicy` method."],["Status","Status is a return value for calls that don't return other objects"],["StatusCause","StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered."],["StatusDetails","StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined."],["TCPSocketAction","Cloud Run fully managed: not supported Cloud Run for Anthos: supported TCPSocketAction describes an action based on opening a socket"],["TestIamPermissionsRequest","Request message for `TestIamPermissions` method."],["TestIamPermissionsResponse","Response message for `TestIamPermissions` method."],["TrafficTarget","TrafficTarget holds a single entry of the routing table for a Route."],["Volume","Cloud Run fully managed: not supported Cloud Run for Anthos: supported Volume represents a named volume in a container."],["VolumeMount","Cloud Run fully managed: not supported Cloud Run for Anthos: supported VolumeMount describes a mounting of a Volume within a container."]]});