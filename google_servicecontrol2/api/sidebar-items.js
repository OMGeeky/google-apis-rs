initSidebarItems({"enum":[["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"struct":[["Api","This message defines attributes associated with API operations, such as a network API request. The terminology is based on the conventions used by Google APIs, Istio, and OpenAPI."],["AttributeContext","This message defines the standard attribute vocabulary for Google APIs. An attribute is a piece of metadata that describes an activity on a network service. For example, the size of an HTTP request, or the status code of an HTTP response. Each attribute has a type and a name, which is logically defined as a proto message field in `AttributeContext`. The field type becomes the attribute type, and the field path becomes the attribute name. For example, the attribute `source.ip` maps to field `AttributeContext.source.ip`. This message definition is guaranteed not to have any wire breaking change. So you can use it directly for passing attributes across different systems. NOTE: Different system may generate different subset of attributes. Please verify the system specification before relying on an attribute generated a system."],["Auth","This message defines request authentication attributes. Terminology is based on the JSON Web Token (JWT) standard, but the terms also correlate to concepts in other standards."],["CheckRequest","Request message for the Check method."],["CheckResponse","Response message for the Check method."],["Peer","This message defines attributes for a node that handles a network request. The node can be either a service or an application that sends, forwards, or receives the request. Service peers should fill in `principal` and `labels` as appropriate."],["ReportRequest","Request message for the Report method."],["ReportResponse","Response message for the Report method. If the request contains any invalid data, the server returns an RPC error."],["Request","This message defines attributes for an HTTP request. If the actual request is not an HTTP request, the runtime system should try to map the actual request to an equivalent HTTP request."],["Resource","This message defines core attributes for a resource. A resource is an addressable (named) entity provided by the destination service. For example, a file stored on a network storage service."],["ResourceInfo","Describes a resource referenced in the request."],["Response","This message defines attributes for a typical network response. It generally models semantics of an HTTP response."],["ServiceCheckCall","Private Preview. This feature is only available for approved services. This method provides admission control for services that are integrated with Service Infrastructure. It checks whether an operation should be allowed based on the service configuration and relevant policies. It must be called before the operation is executed. For more information, see Admission Control. NOTE: The admission control has an expected policy propagation delay of 60s. The caller must not depend on the most recent policy changes. NOTE: The admission control has a hard limit of 1 referenced resources per call. If an operation refers to more than 1 resources, the caller must call the Check method multiple times. This method requires the `servicemanagement.services.check` permission on the specified service. For more information, see Service Control API Access Control."],["ServiceControl","Central instance to access all ServiceControl related resource activities"],["ServiceMethods","A builder providing access to all methods supported on service resources. It is not used directly, but through the `ServiceControl` hub."],["ServiceReportCall","Private Preview. This feature is only available for approved services. This method provides telemetry reporting for services that are integrated with Service Infrastructure. It reports a list of operations that have occurred on a service. It must be called after the operations have been executed. For more information, see Telemetry Reporting. NOTE: The telemetry reporting has a hard limit of 1000 operations and 1MB per Report call. It is recommended to have no more than 100 operations per call. This method requires the `servicemanagement.services.report` permission on the specified service. For more information, see Service Control API Access Control."],["Status","The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC. Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the API Design Guide."]]});