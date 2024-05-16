use super::*;
/// Request used by the AddAddressGroupItems method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups add items organizations](OrganizationLocationAddressGroupAddItemCall) (request)
/// * [locations address groups add items projects](ProjectLocationAddressGroupAddItemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddAddressGroupItemsRequest {
    /// Required. List of items to add.
    
    pub items: Option<Vec<String>>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for AddAddressGroupItemsRequest {}


/// AddressGroup is a resource that specifies how a collection of IP/DNS used in Firewall Policy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups create organizations](OrganizationLocationAddressGroupCreateCall) (request)
/// * [locations address groups get organizations](OrganizationLocationAddressGroupGetCall) (response)
/// * [locations address groups patch organizations](OrganizationLocationAddressGroupPatchCall) (request)
/// * [locations address groups create projects](ProjectLocationAddressGroupCreateCall) (request)
/// * [locations address groups get projects](ProjectLocationAddressGroupGetCall) (response)
/// * [locations address groups patch projects](ProjectLocationAddressGroupPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddressGroup {
    /// Required. Capacity of the Address Group
    
    pub capacity: Option<i32>,
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Optional. List of items.
    
    pub items: Option<Vec<String>>,
    /// Optional. Set of label tags associated with the AddressGroup resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`.
    
    pub name: Option<String>,
    /// Output only. Server-defined fully-qualified URL for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Required. The type of the Address Group. Possible values are "IPv4" or "IPV6".
    #[serde(rename="type")]
    
    pub type_: Option<AddressGroupTypeEnum>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for AddressGroup {}
impl client::ResponseResult for AddressGroup {}


/// AuthorizationPolicy is a resource that specifies how a server should authorize incoming connections. This resource in itself does not change the configuration unless it’s attached to a target https proxy or endpoint config selector resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations authorization policies create projects](ProjectLocationAuthorizationPolicyCreateCall) (request)
/// * [locations authorization policies get projects](ProjectLocationAuthorizationPolicyGetCall) (response)
/// * [locations authorization policies patch projects](ProjectLocationAuthorizationPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    /// Required. The action to take when a rule match is found. Possible values are "ALLOW" or "DENY".
    
    pub action: Option<AuthorizationPolicyActionEnum>,
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Optional. Set of label tags associated with the AuthorizationPolicy resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`.
    
    pub name: Option<String>,
    /// Optional. List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken. A rule is a match if there is a matching source and destination. If left blank, the action specified in the `action` field will be applied on every request.
    
    pub rules: Option<Vec<Rule>>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for AuthorizationPolicy {}
impl client::ResponseResult for AuthorizationPolicy {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel organizations](OrganizationLocationOperationCancelCall) (request)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Specification of a TLS certificate provider instance. Workloads may have one or more CertificateProvider instances (plugins) and one of them is enabled and configured by specifying this message. Workloads use the values from this message to locate and load the CertificateProvider instance configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CertificateProviderInstance {
    /// Required. Plugin instance name, used to locate and load CertificateProvider instance configuration. Set to "google_cloud_private_spiffe" to use Certificate Authority Service certificate provider instance.
    #[serde(rename="pluginInstance")]
    
    pub plugin_instance: Option<String>,
}

impl client::Part for CertificateProviderInstance {}


/// ClientTlsPolicy is a resource that specifies how a client should authenticate connections to backends of a service. This resource itself does not affect configuration unless it is attached to a backend service resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations client tls policies create projects](ProjectLocationClientTlsPolicyCreateCall) (request)
/// * [locations client tls policies get projects](ProjectLocationClientTlsPolicyGetCall) (response)
/// * [locations client tls policies patch projects](ProjectLocationClientTlsPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientTlsPolicy {
    /// Optional. Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS.
    #[serde(rename="clientCertificate")]
    
    pub client_certificate: Option<GoogleCloudNetworksecurityV1CertificateProvider>,
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Optional. Set of label tags associated with the resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/clientTlsPolicies/{client_tls_policy}`
    
    pub name: Option<String>,
    /// Optional. Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate.
    #[serde(rename="serverValidationCa")]
    
    pub server_validation_ca: Option<Vec<ValidationCA>>,
    /// Optional. Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com".
    
    pub sni: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ClientTlsPolicy {}
impl client::ResponseResult for ClientTlsPolicy {}


/// Request used by the CloneAddressGroupItems method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups clone items organizations](OrganizationLocationAddressGroupCloneItemCall) (request)
/// * [locations address groups clone items projects](ProjectLocationAddressGroupCloneItemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloneAddressGroupItemsRequest {
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Required. Source address group to clone items from.
    #[serde(rename="sourceAddressGroup")]
    
    pub source_address_group: Option<String>,
}

impl client::RequestValue for CloneAddressGroupItemsRequest {}


/// Specification of traffic destination attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Destination {
    /// Required. List of host names to match. Matched against the ":authority" header in http requests. At least one host should match. Each host can be an exact match, or a prefix match (example "mydomain.*") or a suffix match (example "*.myorg.com") or a presence (any) match "*".
    
    pub hosts: Option<Vec<String>>,
    /// Optional. Match against key:value pair in http header. Provides a flexible match based on HTTP headers, for potentially advanced use cases. At least one header should match. Avoid using header matches to make authorization decisions unless there is a strong guarantee that requests arrive through a trusted client or proxy.
    #[serde(rename="httpHeaderMatch")]
    
    pub http_header_match: Option<HttpHeaderMatch>,
    /// Optional. A list of HTTP methods to match. At least one method should match. Should not be set for gRPC services.
    
    pub methods: Option<Vec<String>>,
    /// Required. List of destination ports to match. At least one port should match.
    
    pub ports: Option<Vec<u32>>,
}

impl client::Part for Destination {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel organizations](OrganizationLocationOperationCancelCall) (response)
/// * [locations operations delete organizations](OrganizationLocationOperationDeleteCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// Message describing Endpoint object
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations firewall endpoints create organizations](OrganizationLocationFirewallEndpointCreateCall) (request)
/// * [locations firewall endpoints get organizations](OrganizationLocationFirewallEndpointGetCall) (response)
/// * [locations firewall endpoints patch organizations](OrganizationLocationFirewallEndpointPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallEndpoint {
    /// Output only. List of networks that are associated with this endpoint in the local zone. This is a projection of the FirewallEndpointAssociations pointing at this endpoint. A network will only appear in this list after traffic routing is fully configured. Format: projects/{project}/global/networks/{name}.
    #[serde(rename="associatedNetworks")]
    
    pub associated_networks: Option<Vec<String>>,
    /// Output only. List of FirewallEndpointAssociations that are associated to this endpoint. An association will only appear in this list after traffic routing is fully configured.
    
    pub associations: Option<Vec<FirewallEndpointAssociationReference>>,
    /// Required. Project to bill on endpoint uptime usage.
    #[serde(rename="billingProjectId")]
    
    pub billing_project_id: Option<String>,
    /// Output only. Create time stamp
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the firewall endpoint. Max length 2048 characters.
    
    pub description: Option<String>,
    /// Optional. Labels as key value pairs
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. Identifier. name of resource
    
    pub name: Option<String>,
    /// Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128.
    
    pub reconciling: Option<bool>,
    /// Output only. Current state of the endpoint.
    
    pub state: Option<FirewallEndpointStateEnum>,
    /// Output only. Update time stamp
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for FirewallEndpoint {}
impl client::ResponseResult for FirewallEndpoint {}


/// Message describing Association object
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations firewall endpoint associations create projects](ProjectLocationFirewallEndpointAssociationCreateCall) (request)
/// * [locations firewall endpoint associations get projects](ProjectLocationFirewallEndpointAssociationGetCall) (response)
/// * [locations firewall endpoint associations patch projects](ProjectLocationFirewallEndpointAssociationPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallEndpointAssociation {
    /// Output only. Create time stamp
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Whether the association is disabled. True indicates that traffic won't be intercepted
    
    pub disabled: Option<bool>,
    /// Required. The URL of the FirewallEndpoint that is being associated.
    #[serde(rename="firewallEndpoint")]
    
    pub firewall_endpoint: Option<String>,
    /// Optional. Labels as key value pairs
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. Identifier. name of resource
    
    pub name: Option<String>,
    /// Required. The URL of the network that is being associated.
    
    pub network: Option<String>,
    /// Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128.
    
    pub reconciling: Option<bool>,
    /// Output only. Current state of the association.
    
    pub state: Option<FirewallEndpointAssociationStateEnum>,
    /// Optional. The URL of the TlsInspectionPolicy that is being associated.
    #[serde(rename="tlsInspectionPolicy")]
    
    pub tls_inspection_policy: Option<String>,
    /// Output only. Update time stamp
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for FirewallEndpointAssociation {}
impl client::ResponseResult for FirewallEndpointAssociation {}


/// This is a subset of the FirewallEndpointAssociation message, containing fields to be used by the consumer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirewallEndpointAssociationReference {
    /// Output only. The resource name of the FirewallEndpointAssociation. Format: projects/{project}/locations/{location}/firewallEndpointAssociations/{id}
    
    pub name: Option<String>,
    /// Output only. The VPC network associated. Format: projects/{project}/global/networks/{name}.
    
    pub network: Option<String>,
}

impl client::Part for FirewallEndpointAssociationReference {}


/// The GatewaySecurityPolicy resource contains a collection of GatewaySecurityPolicyRules and associated metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations gateway security policies create projects](ProjectLocationGatewaySecurityPolicyCreateCall) (request)
/// * [locations gateway security policies get projects](ProjectLocationGatewaySecurityPolicyGetCall) (response)
/// * [locations gateway security policies patch projects](ProjectLocationGatewaySecurityPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GatewaySecurityPolicy {
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy} gateway_security_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    
    pub name: Option<String>,
    /// Optional. Name of a TLS Inspection Policy resource that defines how TLS inspection will be performed for any rule(s) which enables it.
    #[serde(rename="tlsInspectionPolicy")]
    
    pub tls_inspection_policy: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GatewaySecurityPolicy {}
impl client::ResponseResult for GatewaySecurityPolicy {}


/// The GatewaySecurityPolicyRule resource is in a nested collection within a GatewaySecurityPolicy and represents a traffic matching condition and associated action to perform.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations gateway security policies rules create projects](ProjectLocationGatewaySecurityPolicyRuleCreateCall) (request)
/// * [locations gateway security policies rules get projects](ProjectLocationGatewaySecurityPolicyRuleGetCall) (response)
/// * [locations gateway security policies rules patch projects](ProjectLocationGatewaySecurityPolicyRulePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GatewaySecurityPolicyRule {
    /// Optional. CEL expression for matching on L7/application level criteria.
    #[serde(rename="applicationMatcher")]
    
    pub application_matcher: Option<String>,
    /// Required. Profile which tells what the primitive action should be.
    #[serde(rename="basicProfile")]
    
    pub basic_profile: Option<GatewaySecurityPolicyRuleBasicProfileEnum>,
    /// Output only. Time when the rule was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Required. Whether the rule is enforced.
    
    pub enabled: Option<bool>,
    /// Required. Immutable. Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule} rule should match the pattern: (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    
    pub name: Option<String>,
    /// Required. Priority of the rule. Lower number corresponds to higher precedence.
    
    pub priority: Option<i32>,
    /// Required. CEL expression for matching on session criteria.
    #[serde(rename="sessionMatcher")]
    
    pub session_matcher: Option<String>,
    /// Optional. Flag to enable TLS inspection of traffic matching on , can only be true if the parent GatewaySecurityPolicy references a TLSInspectionConfig.
    #[serde(rename="tlsInspectionEnabled")]
    
    pub tls_inspection_enabled: Option<bool>,
    /// Output only. Time when the rule was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GatewaySecurityPolicyRule {}
impl client::ResponseResult for GatewaySecurityPolicyRule {}


/// Specification of certificate provider. Defines the mechanism to obtain the certificate and private key for peer to peer authentication.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudNetworksecurityV1CertificateProvider {
    /// The certificate provider instance specification that will be passed to the data plane, which will be used to load necessary credential information.
    #[serde(rename="certificateProviderInstance")]
    
    pub certificate_provider_instance: Option<CertificateProviderInstance>,
    /// gRPC specific configuration to access the gRPC server to obtain the cert and private key.
    #[serde(rename="grpcEndpoint")]
    
    pub grpc_endpoint: Option<GoogleCloudNetworksecurityV1GrpcEndpoint>,
}

impl client::Part for GoogleCloudNetworksecurityV1CertificateProvider {}


/// Specification of the GRPC Endpoint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudNetworksecurityV1GrpcEndpoint {
    /// Required. The target URI of the gRPC endpoint. Only UDS path is supported, and should start with "unix:".
    #[serde(rename="targetUri")]
    
    pub target_uri: Option<String>,
}

impl client::Part for GoogleCloudNetworksecurityV1GrpcEndpoint {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<GoogleIamV1AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for GoogleIamV1AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<GoogleIamV1AuditLogConfigLogTypeEnum>,
}

impl client::Part for GoogleIamV1AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workforce identity pool. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}`: All workforce identities in a group. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All workforce identities with a specific attribute value. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`: All identities in a workforce identity pool. * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workload identity pool. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}`: A workload identity pool group. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All identities in a workload identity pool with a certain attribute. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*`: All identities in a workload identity pool. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: Deleted single identity in a workforce identity pool. For example, `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value`.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-overview). For a list of the available pre-defined roles, see [here](https://cloud.google.com/iam/docs/understanding-roles).
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1Binding {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups get iam policy projects](ProjectLocationAddressGroupGetIamPolicyCall) (response)
/// * [locations address groups set iam policy projects](ProjectLocationAddressGroupSetIamPolicyCall) (response)
/// * [locations authorization policies get iam policy projects](ProjectLocationAuthorizationPolicyGetIamPolicyCall) (response)
/// * [locations authorization policies set iam policy projects](ProjectLocationAuthorizationPolicySetIamPolicyCall) (response)
/// * [locations client tls policies get iam policy projects](ProjectLocationClientTlsPolicyGetIamPolicyCall) (response)
/// * [locations client tls policies set iam policy projects](ProjectLocationClientTlsPolicySetIamPolicyCall) (response)
/// * [locations server tls policies get iam policy projects](ProjectLocationServerTlsPolicyGetIamPolicyCall) (response)
/// * [locations server tls policies set iam policy projects](ProjectLocationServerTlsPolicySetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<GoogleIamV1AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<GoogleIamV1Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for GoogleIamV1Policy {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups set iam policy projects](ProjectLocationAddressGroupSetIamPolicyCall) (request)
/// * [locations authorization policies set iam policy projects](ProjectLocationAuthorizationPolicySetIamPolicyCall) (request)
/// * [locations client tls policies set iam policy projects](ProjectLocationClientTlsPolicySetIamPolicyCall) (request)
/// * [locations server tls policies set iam policy projects](ProjectLocationServerTlsPolicySetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<GoogleIamV1Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleIamV1SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups test iam permissions projects](ProjectLocationAddressGroupTestIamPermissionCall) (request)
/// * [locations authorization policies test iam permissions projects](ProjectLocationAuthorizationPolicyTestIamPermissionCall) (request)
/// * [locations client tls policies test iam permissions projects](ProjectLocationClientTlsPolicyTestIamPermissionCall) (request)
/// * [locations server tls policies test iam permissions projects](ProjectLocationServerTlsPolicyTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleIamV1TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups test iam permissions projects](ProjectLocationAddressGroupTestIamPermissionCall) (response)
/// * [locations authorization policies test iam permissions projects](ProjectLocationAuthorizationPolicyTestIamPermissionCall) (response)
/// * [locations client tls policies test iam permissions projects](ProjectLocationClientTlsPolicyTestIamPermissionCall) (response)
/// * [locations server tls policies test iam permissions projects](ProjectLocationServerTlsPolicyTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleIamV1TestIamPermissionsResponse {}


/// Specification of HTTP header match attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpHeaderMatch {
    /// Required. The name of the HTTP header to match. For matching against the HTTP request's authority, use a headerMatch with the header name ":authority". For matching a request's method, use the headerName ":method".
    #[serde(rename="headerName")]
    
    pub header_name: Option<String>,
    /// Required. The value of the header must match the regular expression specified in regexMatch. For regular expression grammar, please see: en.cppreference.com/w/cpp/regex/ecmascript For matching against a port specified in the HTTP request, use a headerMatch with headerName set to Host and a regular expression that satisfies the RFC2616 Host header's port specifier.
    #[serde(rename="regexMatch")]
    
    pub regex_match: Option<String>,
}

impl client::Part for HttpHeaderMatch {}


/// Response of the ListAddressGroupReferences method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups list references organizations](OrganizationLocationAddressGroupListReferenceCall) (response)
/// * [locations address groups list references projects](ProjectLocationAddressGroupListReferenceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAddressGroupReferencesResponse {
    /// A list of references that matches the specified filter in the request.
    #[serde(rename="addressGroupReferences")]
    
    pub address_group_references: Option<Vec<ListAddressGroupReferencesResponseAddressGroupReference>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAddressGroupReferencesResponse {}


/// The Reference of AddressGroup.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAddressGroupReferencesResponseAddressGroupReference {
    /// FirewallPolicy that is using the Address Group.
    #[serde(rename="firewallPolicy")]
    
    pub firewall_policy: Option<String>,
    /// Rule priority of the FirewallPolicy that is using the Address Group.
    #[serde(rename="rulePriority")]
    
    pub rule_priority: Option<i32>,
    /// Cloud Armor SecurityPolicy that is using the Address Group.
    #[serde(rename="securityPolicy")]
    
    pub security_policy: Option<String>,
}

impl client::Part for ListAddressGroupReferencesResponseAddressGroupReference {}


/// Response returned by the ListAddressGroups method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups list organizations](OrganizationLocationAddressGroupListCall) (response)
/// * [locations address groups list projects](ProjectLocationAddressGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAddressGroupsResponse {
    /// List of AddressGroups resources.
    #[serde(rename="addressGroups")]
    
    pub address_groups: Option<Vec<AddressGroup>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAddressGroupsResponse {}


/// Response returned by the ListAuthorizationPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations authorization policies list projects](ProjectLocationAuthorizationPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAuthorizationPoliciesResponse {
    /// List of AuthorizationPolicies resources.
    #[serde(rename="authorizationPolicies")]
    
    pub authorization_policies: Option<Vec<AuthorizationPolicy>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAuthorizationPoliciesResponse {}


/// Response returned by the ListClientTlsPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations client tls policies list projects](ProjectLocationClientTlsPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListClientTlsPoliciesResponse {
    /// List of ClientTlsPolicy resources.
    #[serde(rename="clientTlsPolicies")]
    
    pub client_tls_policies: Option<Vec<ClientTlsPolicy>>,
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListClientTlsPoliciesResponse {}


/// Message for response to listing Associations
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations firewall endpoint associations list projects](ProjectLocationFirewallEndpointAssociationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFirewallEndpointAssociationsResponse {
    /// The list of Association
    #[serde(rename="firewallEndpointAssociations")]
    
    pub firewall_endpoint_associations: Option<Vec<FirewallEndpointAssociation>>,
    /// A token identifying a page of results the server should return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListFirewallEndpointAssociationsResponse {}


/// Message for response to listing Endpoints
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations firewall endpoints list organizations](OrganizationLocationFirewallEndpointListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFirewallEndpointsResponse {
    /// The list of Endpoint
    #[serde(rename="firewallEndpoints")]
    
    pub firewall_endpoints: Option<Vec<FirewallEndpoint>>,
    /// A token identifying a page of results the server should return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListFirewallEndpointsResponse {}


/// Response returned by the ListGatewaySecurityPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations gateway security policies list projects](ProjectLocationGatewaySecurityPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGatewaySecurityPoliciesResponse {
    /// List of GatewaySecurityPolicies resources.
    #[serde(rename="gatewaySecurityPolicies")]
    
    pub gateway_security_policies: Option<Vec<GatewaySecurityPolicy>>,
    /// If there might be more results than those appearing in this response, then 'next_page_token' is included. To get the next set of results, call this method again using the value of 'next_page_token' as 'page_token'.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListGatewaySecurityPoliciesResponse {}


/// Response returned by the ListGatewaySecurityPolicyRules method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations gateway security policies rules list projects](ProjectLocationGatewaySecurityPolicyRuleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGatewaySecurityPolicyRulesResponse {
    /// List of GatewaySecurityPolicyRule resources.
    #[serde(rename="gatewaySecurityPolicyRules")]
    
    pub gateway_security_policy_rules: Option<Vec<GatewaySecurityPolicyRule>>,
    /// If there might be more results than those appearing in this response, then 'next_page_token' is included. To get the next set of results, call this method again using the value of 'next_page_token' as 'page_token'.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListGatewaySecurityPolicyRulesResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list organizations](OrganizationLocationOperationListCall) (response)
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response returned by the ListSecurityProfileGroups method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations security profile groups list organizations](OrganizationLocationSecurityProfileGroupListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSecurityProfileGroupsResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of SecurityProfileGroups resources.
    #[serde(rename="securityProfileGroups")]
    
    pub security_profile_groups: Option<Vec<SecurityProfileGroup>>,
}

impl client::ResponseResult for ListSecurityProfileGroupsResponse {}


/// Response returned by the ListSecurityProfiles method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations security profiles list organizations](OrganizationLocationSecurityProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSecurityProfilesResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of SecurityProfile resources.
    #[serde(rename="securityProfiles")]
    
    pub security_profiles: Option<Vec<SecurityProfile>>,
}

impl client::ResponseResult for ListSecurityProfilesResponse {}


/// Response returned by the ListServerTlsPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations server tls policies list projects](ProjectLocationServerTlsPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServerTlsPoliciesResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of ServerTlsPolicy resources.
    #[serde(rename="serverTlsPolicies")]
    
    pub server_tls_policies: Option<Vec<ServerTlsPolicy>>,
}

impl client::ResponseResult for ListServerTlsPoliciesResponse {}


/// Response returned by the ListTlsInspectionPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tls inspection policies list projects](ProjectLocationTlsInspectionPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTlsInspectionPoliciesResponse {
    /// If there might be more results than those appearing in this response, then 'next_page_token' is included. To get the next set of results, call this method again using the value of 'next_page_token' as 'page_token'.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of TlsInspectionPolicies resources.
    #[serde(rename="tlsInspectionPolicies")]
    
    pub tls_inspection_policies: Option<Vec<TlsInspectionPolicy>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListTlsInspectionPoliciesResponse {}


/// Response returned by the ListUrlLists method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations url lists list projects](ProjectLocationUrlListListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUrlListsResponse {
    /// If there might be more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
    /// List of UrlList resources.
    #[serde(rename="urlLists")]
    
    pub url_lists: Option<Vec<UrlList>>,
}

impl client::ResponseResult for ListUrlListsResponse {}


/// A resource that represents a Google Cloud location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Specification of the MTLSPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MTLSPolicy {
    /// Required if the policy is to be used with Traffic Director. For external HTTPS load balancers it must be empty. Defines the mechanism to obtain the Certificate Authority certificate to validate the client certificate.
    #[serde(rename="clientValidationCa")]
    
    pub client_validation_ca: Option<Vec<ValidationCA>>,
    /// When the client presents an invalid certificate or no certificate to the load balancer, the `client_validation_mode` specifies how the client connection is handled. Required if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty.
    #[serde(rename="clientValidationMode")]
    
    pub client_validation_mode: Option<MTLSPolicyClientValidationModeEnum>,
    /// Reference to the TrustConfig from certificatemanager.googleapis.com namespace. If specified, the chain validation will be performed against certificates configured in the given TrustConfig. Allowed only if the policy is to be used with external HTTPS load balancers.
    #[serde(rename="clientValidationTrustConfig")]
    
    pub client_validation_trust_config: Option<String>,
}

impl client::Part for MTLSPolicy {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups add items organizations](OrganizationLocationAddressGroupAddItemCall) (response)
/// * [locations address groups clone items organizations](OrganizationLocationAddressGroupCloneItemCall) (response)
/// * [locations address groups create organizations](OrganizationLocationAddressGroupCreateCall) (response)
/// * [locations address groups delete organizations](OrganizationLocationAddressGroupDeleteCall) (response)
/// * [locations address groups patch organizations](OrganizationLocationAddressGroupPatchCall) (response)
/// * [locations address groups remove items organizations](OrganizationLocationAddressGroupRemoveItemCall) (response)
/// * [locations firewall endpoints create organizations](OrganizationLocationFirewallEndpointCreateCall) (response)
/// * [locations firewall endpoints delete organizations](OrganizationLocationFirewallEndpointDeleteCall) (response)
/// * [locations firewall endpoints patch organizations](OrganizationLocationFirewallEndpointPatchCall) (response)
/// * [locations operations get organizations](OrganizationLocationOperationGetCall) (response)
/// * [locations security profile groups create organizations](OrganizationLocationSecurityProfileGroupCreateCall) (response)
/// * [locations security profile groups delete organizations](OrganizationLocationSecurityProfileGroupDeleteCall) (response)
/// * [locations security profile groups patch organizations](OrganizationLocationSecurityProfileGroupPatchCall) (response)
/// * [locations security profiles create organizations](OrganizationLocationSecurityProfileCreateCall) (response)
/// * [locations security profiles delete organizations](OrganizationLocationSecurityProfileDeleteCall) (response)
/// * [locations security profiles patch organizations](OrganizationLocationSecurityProfilePatchCall) (response)
/// * [locations address groups add items projects](ProjectLocationAddressGroupAddItemCall) (response)
/// * [locations address groups clone items projects](ProjectLocationAddressGroupCloneItemCall) (response)
/// * [locations address groups create projects](ProjectLocationAddressGroupCreateCall) (response)
/// * [locations address groups delete projects](ProjectLocationAddressGroupDeleteCall) (response)
/// * [locations address groups patch projects](ProjectLocationAddressGroupPatchCall) (response)
/// * [locations address groups remove items projects](ProjectLocationAddressGroupRemoveItemCall) (response)
/// * [locations authorization policies create projects](ProjectLocationAuthorizationPolicyCreateCall) (response)
/// * [locations authorization policies delete projects](ProjectLocationAuthorizationPolicyDeleteCall) (response)
/// * [locations authorization policies patch projects](ProjectLocationAuthorizationPolicyPatchCall) (response)
/// * [locations client tls policies create projects](ProjectLocationClientTlsPolicyCreateCall) (response)
/// * [locations client tls policies delete projects](ProjectLocationClientTlsPolicyDeleteCall) (response)
/// * [locations client tls policies patch projects](ProjectLocationClientTlsPolicyPatchCall) (response)
/// * [locations firewall endpoint associations create projects](ProjectLocationFirewallEndpointAssociationCreateCall) (response)
/// * [locations firewall endpoint associations delete projects](ProjectLocationFirewallEndpointAssociationDeleteCall) (response)
/// * [locations firewall endpoint associations patch projects](ProjectLocationFirewallEndpointAssociationPatchCall) (response)
/// * [locations gateway security policies rules create projects](ProjectLocationGatewaySecurityPolicyRuleCreateCall) (response)
/// * [locations gateway security policies rules delete projects](ProjectLocationGatewaySecurityPolicyRuleDeleteCall) (response)
/// * [locations gateway security policies rules patch projects](ProjectLocationGatewaySecurityPolicyRulePatchCall) (response)
/// * [locations gateway security policies create projects](ProjectLocationGatewaySecurityPolicyCreateCall) (response)
/// * [locations gateway security policies delete projects](ProjectLocationGatewaySecurityPolicyDeleteCall) (response)
/// * [locations gateway security policies patch projects](ProjectLocationGatewaySecurityPolicyPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations server tls policies create projects](ProjectLocationServerTlsPolicyCreateCall) (response)
/// * [locations server tls policies delete projects](ProjectLocationServerTlsPolicyDeleteCall) (response)
/// * [locations server tls policies patch projects](ProjectLocationServerTlsPolicyPatchCall) (response)
/// * [locations tls inspection policies create projects](ProjectLocationTlsInspectionPolicyCreateCall) (response)
/// * [locations tls inspection policies delete projects](ProjectLocationTlsInspectionPolicyDeleteCall) (response)
/// * [locations tls inspection policies patch projects](ProjectLocationTlsInspectionPolicyPatchCall) (response)
/// * [locations url lists create projects](ProjectLocationUrlListCreateCall) (response)
/// * [locations url lists delete projects](ProjectLocationUrlListDeleteCall) (response)
/// * [locations url lists patch projects](ProjectLocationUrlListPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// Request used by the RemoveAddressGroupItems method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations address groups remove items organizations](OrganizationLocationAddressGroupRemoveItemCall) (request)
/// * [locations address groups remove items projects](ProjectLocationAddressGroupRemoveItemCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveAddressGroupItemsRequest {
    /// Required. List of items to remove.
    
    pub items: Option<Vec<String>>,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for RemoveAddressGroupItemsRequest {}


/// Specification of rules.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// Optional. List of attributes for the traffic destination. All of the destinations must match. A destination is a match if a request matches all the specified hosts, ports, methods and headers. If not set, the action specified in the 'action' field will be applied without any rule checks for the destination.
    
    pub destinations: Option<Vec<Destination>>,
    /// Optional. List of attributes for the traffic source. All of the sources must match. A source is a match if both principals and ip_blocks match. If not set, the action specified in the 'action' field will be applied without any rule checks for the source.
    
    pub sources: Option<Vec<Source>>,
}

impl client::Part for Rule {}


/// SecurityProfile is a resource that defines the behavior for one of many ProfileTypes. Next ID: 9
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations security profiles create organizations](OrganizationLocationSecurityProfileCreateCall) (request)
/// * [locations security profiles get organizations](OrganizationLocationSecurityProfileGetCall) (response)
/// * [locations security profiles patch organizations](OrganizationLocationSecurityProfilePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityProfile {
    /// Output only. Resource creation timestamp.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. An optional description of the profile. Max length 512 characters.
    
    pub description: Option<String>,
    /// Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. Labels as key value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. Identifier. Name of the SecurityProfile resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfiles/{security_profile}`.
    
    pub name: Option<String>,
    /// The threat prevention configuration for the SecurityProfile.
    #[serde(rename="threatPreventionProfile")]
    
    pub threat_prevention_profile: Option<ThreatPreventionProfile>,
    /// Immutable. The single ProfileType that the SecurityProfile resource configures.
    #[serde(rename="type")]
    
    pub type_: Option<SecurityProfileTypeEnum>,
    /// Output only. Last resource update timestamp.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for SecurityProfile {}
impl client::ResponseResult for SecurityProfile {}


/// SecurityProfileGroup is a resource that defines the behavior for various ProfileTypes. Next ID: 8
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations security profile groups create organizations](OrganizationLocationSecurityProfileGroupCreateCall) (request)
/// * [locations security profile groups get organizations](OrganizationLocationSecurityProfileGroupGetCall) (response)
/// * [locations security profile groups patch organizations](OrganizationLocationSecurityProfileGroupPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityProfileGroup {
    /// Output only. Resource creation timestamp.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. An optional description of the profile group. Max length 2048 characters.
    
    pub description: Option<String>,
    /// Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. Labels as key value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// Immutable. Identifier. Name of the SecurityProfileGroup resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`.
    
    pub name: Option<String>,
    /// Optional. Reference to a SecurityProfile with the threat prevention configuration for the SecurityProfileGroup.
    #[serde(rename="threatPreventionProfile")]
    
    pub threat_prevention_profile: Option<String>,
    /// Output only. Last resource update timestamp.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for SecurityProfileGroup {}
impl client::ResponseResult for SecurityProfileGroup {}


/// ServerTlsPolicy is a resource that specifies how a server should authenticate incoming requests. This resource itself does not affect configuration unless it is attached to a target HTTPS proxy or endpoint config selector resource. ServerTlsPolicy in the form accepted by external HTTPS load balancers can be attached only to TargetHttpsProxy with an `EXTERNAL` or `EXTERNAL_MANAGED` load balancing scheme. Traffic Director compatible ServerTlsPolicies can be attached to EndpointPolicy and TargetHttpsProxy with Traffic Director `INTERNAL_SELF_MANAGED` load balancing scheme.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations server tls policies create projects](ProjectLocationServerTlsPolicyCreateCall) (request)
/// * [locations server tls policies get projects](ProjectLocationServerTlsPolicyGetCall) (response)
/// * [locations server tls policies patch projects](ProjectLocationServerTlsPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServerTlsPolicy {
    /// This field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies. Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility. Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80.
    #[serde(rename="allowOpen")]
    
    pub allow_open: Option<bool>,
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Free-text description of the resource.
    
    pub description: Option<String>,
    /// Set of label tags associated with the resource.
    
    pub labels: Option<HashMap<String, String>>,
    /// This field is required if the policy is used with external HTTPS load balancers. This field can be empty for Traffic Director. Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections.
    #[serde(rename="mtlsPolicy")]
    
    pub mtls_policy: Option<MTLSPolicy>,
    /// Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}`
    
    pub name: Option<String>,
    /// Optional if policy is to be used with Traffic Director. For external HTTPS load balancer must be empty. Defines a mechanism to provision server identity (public and private keys). Cannot be combined with `allow_open` as a permissive mode that allows both plain text and TLS is not supported.
    #[serde(rename="serverCertificate")]
    
    pub server_certificate: Option<GoogleCloudNetworksecurityV1CertificateProvider>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ServerTlsPolicy {}
impl client::ResponseResult for ServerTlsPolicy {}


/// Defines what action to take for a specific severity match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SeverityOverride {
    /// Required. Threat action override.
    
    pub action: Option<SeverityOverrideActionEnum>,
    /// Required. Severity level to match.
    
    pub severity: Option<SeverityOverrideSeverityEnum>,
}

impl client::Part for SeverityOverride {}


/// Specification of traffic source attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// Optional. List of CIDR ranges to match based on source IP address. At least one IP block should match. Single IP (e.g., "1.2.3.4") and CIDR (e.g., "1.2.3.0/24") are supported. Authorization based on source IP alone should be avoided. The IP addresses of any load balancers or proxies should be considered untrusted.
    #[serde(rename="ipBlocks")]
    
    pub ip_blocks: Option<Vec<String>>,
    /// Optional. List of peer identities to match for authorization. At least one principal should match. Each peer can be an exact match, or a prefix match (example, "namespace/*") or a suffix match (example, "*/service-account") or a presence match "*". Authorization based on the principal name without certificate validation (configured by ServerTlsPolicy resource) is considered insecure.
    
    pub principals: Option<Vec<String>>,
}

impl client::Part for Source {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Defines what action to take for a specific threat_id match.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThreatOverride {
    /// Required. Threat action override. For some threat types, only a subset of actions applies.
    
    pub action: Option<ThreatOverrideActionEnum>,
    /// Required. Vendor-specific ID of a threat to override.
    #[serde(rename="threatId")]
    
    pub threat_id: Option<String>,
    /// Output only. Type of the threat (read only).
    #[serde(rename="type")]
    
    pub type_: Option<ThreatOverrideTypeEnum>,
}

impl client::Part for ThreatOverride {}


/// ThreatPreventionProfile defines an action for specific threat signatures or severity levels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThreatPreventionProfile {
    /// Optional. Configuration for overriding threats actions by severity match.
    #[serde(rename="severityOverrides")]
    
    pub severity_overrides: Option<Vec<SeverityOverride>>,
    /// Optional. Configuration for overriding threats actions by threat_id match. If a threat is matched both by configuration provided in severity_overrides and threat_overrides, the threat_overrides action is applied.
    #[serde(rename="threatOverrides")]
    
    pub threat_overrides: Option<Vec<ThreatOverride>>,
}

impl client::Part for ThreatPreventionProfile {}


/// The TlsInspectionPolicy resource contains references to CA pools in Certificate Authority Service and associated metadata.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations tls inspection policies create projects](ProjectLocationTlsInspectionPolicyCreateCall) (request)
/// * [locations tls inspection policies get projects](ProjectLocationTlsInspectionPolicyGetCall) (response)
/// * [locations tls inspection policies patch projects](ProjectLocationTlsInspectionPolicyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TlsInspectionPolicy {
    /// Required. A CA pool resource used to issue interception certificates. The CA pool string has a relative resource path following the form "projects/{project}/locations/{location}/caPools/{ca_pool}".
    #[serde(rename="caPool")]
    
    pub ca_pool: Option<String>,
    /// Output only. The timestamp when the resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field.
    #[serde(rename="customTlsFeatures")]
    
    pub custom_tls_features: Option<Vec<String>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Optional. If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trust_config. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trust_config will be accepted. This defaults to FALSE (use public CAs in addition to trust_config) for backwards compatibility, but trusting public root CAs is *not recommended* unless the traffic in question is outbound to public web servers. When possible, prefer setting this to "false" and explicitly specifying trusted CAs and certificates in a TrustConfig. Note that Secure Web Proxy does not yet honor this field.
    #[serde(rename="excludePublicCaSet")]
    
    pub exclude_public_ca_set: Option<bool>,
    /// Optional. Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
    #[serde(rename="minTlsVersion")]
    
    pub min_tls_version: Option<TlsInspectionPolicyMinTlsVersionEnum>,
    /// Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy} tls_inspection_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    
    pub name: Option<String>,
    /// Optional. The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers ("PROFILE_COMPATIBLE"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
    #[serde(rename="tlsFeatureProfile")]
    
    pub tls_feature_profile: Option<TlsInspectionPolicyTlsFeatureProfileEnum>,
    /// Optional. A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Note that Secure Web Proxy does not yet honor this field.
    #[serde(rename="trustConfig")]
    
    pub trust_config: Option<String>,
    /// Output only. The timestamp when the resource was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for TlsInspectionPolicy {}
impl client::ResponseResult for TlsInspectionPolicy {}


/// UrlList proto helps users to set reusable, independently manageable lists of hosts, host patterns, URLs, URL patterns.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations url lists create projects](ProjectLocationUrlListCreateCall) (request)
/// * [locations url lists get projects](ProjectLocationUrlListGetCall) (response)
/// * [locations url lists patch projects](ProjectLocationUrlListPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UrlList {
    /// Output only. Time when the security policy was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Free-text description of the resource.
    
    pub description: Option<String>,
    /// Required. Name of the resource provided by the user. Name is of the form projects/{project}/locations/{location}/urlLists/{url_list} url_list should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$).
    
    pub name: Option<String>,
    /// Output only. Time when the security policy was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. FQDNs and URLs.
    
    pub values: Option<Vec<String>>,
}

impl client::RequestValue for UrlList {}
impl client::ResponseResult for UrlList {}


/// Specification of ValidationCA. Defines the mechanism to obtain the Certificate Authority certificate to validate the peer certificate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValidationCA {
    /// The certificate provider instance specification that will be passed to the data plane, which will be used to load necessary credential information.
    #[serde(rename="certificateProviderInstance")]
    
    pub certificate_provider_instance: Option<CertificateProviderInstance>,
    /// gRPC specific configuration to access the gRPC server to obtain the CA certificate.
    #[serde(rename="grpcEndpoint")]
    
    pub grpc_endpoint: Option<GoogleCloudNetworksecurityV1GrpcEndpoint>,
}

impl client::Part for ValidationCA {}


