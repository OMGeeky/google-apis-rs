use super::*;
/// Access related restrictions on the workforce pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessRestrictions {
    /// Optional. Immutable. Services allowed for web sign-in with the workforce pool. If not set by default there are no restrictions.
    #[serde(rename="allowedServices")]
    
    pub allowed_services: Option<Vec<ServiceConfig>>,
    /// Optional. Disable programmatic sign-in by disabling token issue via the Security Token API endpoint. See [Security Token Service API] (https://cloud.google.com/iam/docs/reference/sts/rest).
    #[serde(rename="disableProgrammaticSignin")]
    
    pub disable_programmatic_signin: Option<bool>,
}

impl client::Part for AccessRestrictions {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<AuditLogConfigLogTypeEnum>,
}

impl client::Part for AuditLogConfig {}


/// Contains information about an auditable service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditableService {
    /// Public name of the service. For example, the service name for Cloud IAM is 'iam.googleapis.com'.
    
    pub name: Option<String>,
}

impl client::Part for AuditableService {}


/// Represents an Amazon Web Services identity provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Aws {
    /// Required. The AWS account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
}

impl client::Part for Aws {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workforce identity pool. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}`: All workforce identities in a group. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All workforce identities with a specific attribute value. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`: All identities in a workforce identity pool. * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workload identity pool. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}`: A workload identity pool group. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All identities in a workload identity pool with a certain attribute. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*`: All identities in a workload identity pool. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: Deleted single identity in a workforce identity pool. For example, `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value`.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-overview). For a list of the available pre-defined roles, see [here](https://cloud.google.com/iam/docs/understanding-roles).
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// The request to create a new role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [roles create organizations](OrganizationRoleCreateCall) (request)
/// * [roles create projects](ProjectRoleCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    /// The Role resource to create.
    
    pub role: Option<Role>,
    /// The role ID to use for this role. A role ID may contain alphanumeric characters, underscores (`_`), and periods (`.`). It must contain a minimum of 3 characters and a maximum of 64 characters.
    #[serde(rename="roleId")]
    
    pub role_id: Option<String>,
}

impl client::RequestValue for CreateRoleRequest {}


/// The service account key create request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys create projects](ProjectServiceAccountKeyCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateServiceAccountKeyRequest {
    /// Which type of key and algorithm to use for the key. The default is currently a 2K RSA key. However this may change in the future.
    #[serde(rename="keyAlgorithm")]
    
    pub key_algorithm: Option<CreateServiceAccountKeyRequestKeyAlgorithmEnum>,
    /// The output format of the private key. The default value is `TYPE_GOOGLE_CREDENTIALS_FILE`, which is the Google Credentials File format.
    #[serde(rename="privateKeyType")]
    
    pub private_key_type: Option<CreateServiceAccountKeyRequestPrivateKeyTypeEnum>,
}

impl client::RequestValue for CreateServiceAccountKeyRequest {}


/// The service account create request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts create projects](ProjectServiceAccountCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateServiceAccountRequest {
    /// Required. The account id that is used to generate the service account email address and a stable unique id. It is unique within a project, must be 6-30 characters long, and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])` to comply with RFC1035.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The ServiceAccount resource to create. Currently, only the following values are user assignable: `display_name` and `description`.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<ServiceAccount>,
}

impl client::RequestValue for CreateServiceAccountRequest {}


/// The service account key disable request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys disable projects](ProjectServiceAccountKeyDisableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisableServiceAccountKeyRequest {
    /// Optional. Usable by internal google services only. An extended_status_message can be used to include additional information about the key, such as its private key data being exposed on a public repository like GitHub.
    #[serde(rename="extendedStatusMessage")]
    
    pub extended_status_message: Option<String>,
    /// Optional. Describes the reason this key is being disabled. If unspecified, the default value of SERVICE_ACCOUNT_KEY_DISABLE_REASON_USER_INITIATED will be used.
    #[serde(rename="serviceAccountKeyDisableReason")]
    
    pub service_account_key_disable_reason: Option<DisableServiceAccountKeyRequestServiceAccountKeyDisableReasonEnum>,
}

impl client::RequestValue for DisableServiceAccountKeyRequest {}


/// The service account disable request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts disable projects](ProjectServiceAccountDisableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisableServiceAccountRequest { _never_set: Option<bool> }

impl client::RequestValue for DisableServiceAccountRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations oauth clients credentials delete projects](ProjectLocationOauthClientCredentialDeleteCall) (response)
/// * [service accounts keys delete projects](ProjectServiceAccountKeyDeleteCall) (response)
/// * [service accounts keys disable projects](ProjectServiceAccountKeyDisableCall) (response)
/// * [service accounts keys enable projects](ProjectServiceAccountKeyEnableCall) (response)
/// * [service accounts delete projects](ProjectServiceAccountDeleteCall) (response)
/// * [service accounts disable projects](ProjectServiceAccountDisableCall) (response)
/// * [service accounts enable projects](ProjectServiceAccountEnableCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The service account key enable request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys enable projects](ProjectServiceAccountKeyEnableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnableServiceAccountKeyRequest { _never_set: Option<bool> }

impl client::RequestValue for EnableServiceAccountKeyRequest {}


/// The service account enable request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts enable projects](ProjectServiceAccountEnableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnableServiceAccountRequest { _never_set: Option<bool> }

impl client::RequestValue for EnableServiceAccountRequest {}


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


/// Extended status can store additional metadata. For example, for keys disabled due to their private key data being expoesed we may include a message with more information about the exposure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedStatus {
    /// The key for this extended status.
    
    pub key: Option<ExtendedStatusKeyEnum>,
    /// The value for the extended status.
    
    pub value: Option<String>,
}

impl client::Part for ExtendedStatus {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools get iam policy locations](LocationWorkforcePoolGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// Represents the OAuth 2.0 client credential configuration for retrieving additional user attributes that are not present in the initial authentication credentials from the identity provider, e.g. groups. See https://datatracker.ietf.org/doc/html/rfc6749#section-4.4 for more details on client credentials grant flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2Client {
    /// Required. Represents the IdP and type of claims that should be fetched.
    #[serde(rename="attributesType")]
    
    pub attributes_type: Option<GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientAttributesTypeEnum>,
    /// Required. The OAuth 2.0 client ID for retrieving extra attributes from the identity provider. Required to get the Access Token using client credentials grant flow.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Required. The OAuth 2.0 client secret for retrieving extra attributes from the identity provider. Required to get the Access Token using client credentials grant flow.
    #[serde(rename="clientSecret")]
    
    pub client_secret: Option<GoogleIamAdminV1WorkforcePoolProviderOidcClientSecret>,
    /// Required. The OIDC identity provider's issuer URI. Must be a valid URI using the `https` scheme. Required to get the OIDC discovery document.
    #[serde(rename="issuerUri")]
    
    pub issuer_uri: Option<String>,
    /// Optional. Represents the parameters to control which claims are fetched from an IdP.
    #[serde(rename="queryParameters")]
    
    pub query_parameters: Option<GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientQueryParameters>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2Client {}


/// Represents the parameters to control which claims are fetched from an IdP.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientQueryParameters {
    /// Optional. The filter used to request specific records from IdP. In case of attributes type as AZURE_AD_GROUPS_MAIL, it represents the filter used to request specific groups for users from IdP. By default all the groups associated with the user are fetched. The groups that are used should be mail enabled and security enabled. See https://learn.microsoft.com/en-us/graph/search-query-parameter for more details.
    
    pub filter: Option<String>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2ClientQueryParameters {}


/// Represents an OpenId Connect 1.0 identity provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderOidc {
    /// Required. The client ID. Must match the audience claim of the JWT issued by the identity provider.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// The optional client secret. Required to enable Authorization Code flow for web sign-in.
    #[serde(rename="clientSecret")]
    
    pub client_secret: Option<GoogleIamAdminV1WorkforcePoolProviderOidcClientSecret>,
    /// Required. The OIDC issuer URI. Must be a valid URI using the `https` scheme.
    #[serde(rename="issuerUri")]
    
    pub issuer_uri: Option<String>,
    /// OIDC JWKs in JSON String format. For details on the definition of a JWK, see https://tools.ietf.org/html/rfc7517. If not set, the `jwks_uri` from the discovery document(fetched from the .well-known path of the `issuer_uri`) will be used. Currently, RSA and EC asymmetric keys are supported. The JWK must use following format and include only the following fields: { "keys": [ { "kty": "RSA/EC", "alg": "", "use": "sig", "kid": "", "n": "", "e": "", "x": "", "y": "", "crv": "" } ] }
    #[serde(rename="jwksJson")]
    
    pub jwks_json: Option<String>,
    /// Required. Configuration for web single sign-on for the OIDC provider. Here, web sign-in refers to console sign-in and gcloud sign-in through the browser.
    #[serde(rename="webSsoConfig")]
    
    pub web_sso_config: Option<GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfig>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderOidc {}


/// Representation of a client secret configured for the OIDC provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderOidcClientSecret {
    /// The value of the client secret.
    
    pub value: Option<GoogleIamAdminV1WorkforcePoolProviderOidcClientSecretValue>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderOidcClientSecret {}


/// Representation of the value of the client secret.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderOidcClientSecretValue {
    /// Input only. The plain text of the client secret value. For security reasons, this field is only used for input and will never be populated in any response.
    #[serde(rename="plainText")]
    
    pub plain_text: Option<String>,
    /// Output only. A thumbprint to represent the current client secret value.
    
    pub thumbprint: Option<String>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderOidcClientSecretValue {}


/// Configuration for web single sign-on for the OIDC provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfig {
    /// Additional scopes to request for in the OIDC authentication request on top of scopes requested by default. By default, the `openid`, `profile` and `email` scopes that are supported by the identity provider are requested. Each additional scope may be at most 256 characters. A maximum of 10 additional scopes may be configured.
    #[serde(rename="additionalScopes")]
    
    pub additional_scopes: Option<Vec<String>>,
    /// Required. The behavior for how OIDC Claims are included in the `assertion` object used for attribute mapping and attribute condition.
    #[serde(rename="assertionClaimsBehavior")]
    
    pub assertion_claims_behavior: Option<GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigAssertionClaimsBehaviorEnum>,
    /// Required. The Response Type to request for in the OIDC Authorization Request for web sign-in. The `CODE` Response Type is recommended to avoid the Implicit Flow, for security reasons.
    #[serde(rename="responseType")]
    
    pub response_type: Option<GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfigResponseTypeEnum>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderOidcWebSsoConfig {}


/// Represents a SAML identity provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamAdminV1WorkforcePoolProviderSaml {
    /// Required. SAML Identity provider configuration metadata xml doc. The xml document should comply with [SAML 2.0 specification](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.pdf). The max size of the acceptable xml document will be bounded to 128k characters. The metadata xml document should satisfy the following constraints: 1) Must contain an Identity Provider Entity ID. 2) Must contain at least one non-expired signing key certificate. 3) For each signing key: a) Valid from should be no more than 7 days from now. b) Valid to should be no more than 15 years in the future. 4) Up to 3 IdP signing keys are allowed in the metadata xml. When updating the provider's metadata xml, at least one non-expired signing key must overlap with the existing metadata. This requirement is skipped if there are no non-expired signing keys present in the existing metadata.
    #[serde(rename="idpMetadataXml")]
    
    pub idp_metadata_xml: Option<String>,
}

impl client::Part for GoogleIamAdminV1WorkforcePoolProviderSaml {}


/// Represents a public key data along with its format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyData {
    /// Output only. The format of the key.
    
    pub format: Option<KeyDataFormatEnum>,
    /// Output only. The key data. The format of the key is represented by the format field.
    
    pub key: Option<String>,
    /// Required. The specifications for the key.
    #[serde(rename="keySpec")]
    
    pub key_spec: Option<KeyDataKeySpecEnum>,
    /// Output only. Latest timestamp when this key is valid. Attempts to use this key after this time will fail. Only present if the key data represents a X.509 certificate.
    #[serde(rename="notAfterTime")]
    
    pub not_after_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Earliest timestamp when this key is valid. Attempts to use this key before this time will fail. Only present if the key data represents a X.509 certificate.
    #[serde(rename="notBeforeTime")]
    
    pub not_before_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for KeyData {}


/// The request to lint a Cloud IAM policy object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lint policy iam policies](IamPolicyLintPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LintPolicyRequest {
    /// google.iam.v1.Binding.condition object to be linted.
    
    pub condition: Option<Expr>,
    /// The full resource name of the policy this lint request is about. The name follows the Google Cloud format for full resource names. For example, a Cloud project with ID `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`. The resource name is not used to read a policy from IAM. Only the data in the request object is linted.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
}

impl client::RequestValue for LintPolicyRequest {}


/// The response of a lint operation. An empty response indicates the operation was able to fully execute and no lint issue was found.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lint policy iam policies](IamPolicyLintPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LintPolicyResponse {
    /// List of lint results sorted by `severity` in descending order.
    #[serde(rename="lintResults")]
    
    pub lint_results: Option<Vec<LintResult>>,
}

impl client::ResponseResult for LintPolicyResponse {}


/// Structured response of a single validation unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LintResult {
    /// Human readable debug message associated with the issue.
    #[serde(rename="debugMessage")]
    
    pub debug_message: Option<String>,
    /// The name of the field for which this lint result is about. For nested messages `field_name` consists of names of the embedded fields separated by period character. The top-level qualifier is the input object to lint in the request. For example, the `field_name` value `condition.expression` identifies a lint result for the `expression` field of the provided condition.
    #[serde(rename="fieldName")]
    
    pub field_name: Option<String>,
    /// The validation unit level.
    
    pub level: Option<LintResultLevelEnum>,
    /// 0-based character position of problematic construct within the object identified by `field_name`. Currently, this is populated only for condition expression.
    #[serde(rename="locationOffset")]
    
    pub location_offset: Option<i32>,
    /// The validation unit severity.
    
    pub severity: Option<LintResultSeverityEnum>,
    /// The validation unit name, for instance "lintValidationUnits/ConditionComplexityCheck".
    #[serde(rename="validationUnitName")]
    
    pub validation_unit_name: Option<String>,
}

impl client::Part for LintResult {}


/// Response message for ListOauthClientCredentials.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations oauth clients credentials list projects](ProjectLocationOauthClientCredentialListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOauthClientCredentialsResponse {
    /// A list of oauth client credentials.
    #[serde(rename="oauthClientCredentials")]
    
    pub oauth_client_credentials: Option<Vec<OauthClientCredential>>,
}

impl client::ResponseResult for ListOauthClientCredentialsResponse {}


/// Response message for ListOauthClients.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations oauth clients list projects](ProjectLocationOauthClientListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOauthClientsResponse {
    /// Optional. A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of oauth clients.
    #[serde(rename="oauthClients")]
    
    pub oauth_clients: Option<Vec<OauthClient>>,
}

impl client::ResponseResult for ListOauthClientsResponse {}


/// The response containing the roles defined under a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [roles list organizations](OrganizationRoleListCall) (response)
/// * [roles list projects](ProjectRoleListCall) (response)
/// * [list roles](RoleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRolesResponse {
    /// To retrieve the next page of results, set `ListRolesRequest.page_token` to this value.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Roles defined on this resource.
    
    pub roles: Option<Vec<Role>>,
}

impl client::ResponseResult for ListRolesResponse {}


/// The service account keys list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys list projects](ProjectServiceAccountKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceAccountKeysResponse {
    /// The public keys for the service account.
    
    pub keys: Option<Vec<ServiceAccountKey>>,
}

impl client::ResponseResult for ListServiceAccountKeysResponse {}


/// The service account list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts list projects](ProjectServiceAccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServiceAccountsResponse {
    /// The list of matching service accounts.
    
    pub accounts: Option<Vec<ServiceAccount>>,
    /// To retrieve the next page of results, set ListServiceAccountsRequest.page_token to this value.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListServiceAccountsResponse {}


/// Response message for ListWorkforcePoolProviderKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools providers keys list locations](LocationWorkforcePoolProviderKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkforcePoolProviderKeysResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of WorkforcePoolProviderKeys.
    #[serde(rename="workforcePoolProviderKeys")]
    
    pub workforce_pool_provider_keys: Option<Vec<WorkforcePoolProviderKey>>,
}

impl client::ResponseResult for ListWorkforcePoolProviderKeysResponse {}


/// Response message for ListWorkforcePoolProviders.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools providers list locations](LocationWorkforcePoolProviderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkforcePoolProvidersResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of providers.
    #[serde(rename="workforcePoolProviders")]
    
    pub workforce_pool_providers: Option<Vec<WorkforcePoolProvider>>,
}

impl client::ResponseResult for ListWorkforcePoolProvidersResponse {}


/// Response message for ListWorkforcePools.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools list locations](LocationWorkforcePoolListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkforcePoolsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of pools.
    #[serde(rename="workforcePools")]
    
    pub workforce_pools: Option<Vec<WorkforcePool>>,
}

impl client::ResponseResult for ListWorkforcePoolsResponse {}


/// Response message for ListWorkloadIdentityPoolProviderKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools providers keys list projects](ProjectLocationWorkloadIdentityPoolProviderKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkloadIdentityPoolProviderKeysResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of WorkloadIdentityPoolProviderKey
    #[serde(rename="workloadIdentityPoolProviderKeys")]
    
    pub workload_identity_pool_provider_keys: Option<Vec<WorkloadIdentityPoolProviderKey>>,
}

impl client::ResponseResult for ListWorkloadIdentityPoolProviderKeysResponse {}


/// Response message for ListWorkloadIdentityPoolProviders.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools providers list projects](ProjectLocationWorkloadIdentityPoolProviderListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkloadIdentityPoolProvidersResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of providers.
    #[serde(rename="workloadIdentityPoolProviders")]
    
    pub workload_identity_pool_providers: Option<Vec<WorkloadIdentityPoolProvider>>,
}

impl client::ResponseResult for ListWorkloadIdentityPoolProvidersResponse {}


/// Response message for ListWorkloadIdentityPools.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools list projects](ProjectLocationWorkloadIdentityPoolListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkloadIdentityPoolsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of pools.
    #[serde(rename="workloadIdentityPools")]
    
    pub workload_identity_pools: Option<Vec<WorkloadIdentityPool>>,
}

impl client::ResponseResult for ListWorkloadIdentityPoolsResponse {}


/// Represents an oauth client. Used to access Google Cloud resources on behave of a user by using OAuth2 Protocol to obtain an access token from Google Cloud Platform.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations oauth clients create projects](ProjectLocationOauthClientCreateCall) (request|response)
/// * [locations oauth clients delete projects](ProjectLocationOauthClientDeleteCall) (response)
/// * [locations oauth clients get projects](ProjectLocationOauthClientGetCall) (response)
/// * [locations oauth clients patch projects](ProjectLocationOauthClientPatchCall) (request|response)
/// * [locations oauth clients undelete projects](ProjectLocationOauthClientUndeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OauthClient {
    /// Required. The list of OAuth grant type is allowed for the oauth client.
    #[serde(rename="allowedGrantTypes")]
    
    pub allowed_grant_types: Option<Vec<OauthClientAllowedGrantTypesEnum>>,
    /// Required. The list of redirect uris that is allowed to redirect back when authorization process is completed.
    #[serde(rename="allowedRedirectUris")]
    
    pub allowed_redirect_uris: Option<Vec<String>>,
    /// Required. The list of scopes that the oauth client is allowed to request during OAuth flows. The following scopes are supported: * `https://www.googleapis.com/auth/cloud-platform`: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account. * `openid`: Associate you with your personal info on Google Cloud. * `email`: See your Google Cloud Account email address.
    #[serde(rename="allowedScopes")]
    
    pub allowed_scopes: Option<Vec<String>>,
    /// Output only. The system-generated oauth client id.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Immutable. The type of oauth client. either public or private.
    #[serde(rename="clientType")]
    
    pub client_type: Option<OauthClientClientTypeEnum>,
    /// Optional. A user-specified description of the oauth client. Cannot exceed 256 characters.
    
    pub description: Option<String>,
    /// Optional. Whether the oauth client is disabled. You cannot use a disabled oauth client for login.
    
    pub disabled: Option<bool>,
    /// Optional. A user-specified display name of the oauth client. Cannot exceed 32 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Time after which the oauth client will be permanently purged and cannot be recovered.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. The resource name of the oauth client. Format:`projects/{project}/locations/{location}/oauthClients/{oauth_client}`.
    
    pub name: Option<String>,
    /// Output only. The state of the oauth client.
    
    pub state: Option<OauthClientStateEnum>,
}

impl client::RequestValue for OauthClient {}
impl client::ResponseResult for OauthClient {}


/// Represents an oauth client credential. Used to authenticate an oauth client while accessing Google Cloud resources on behalf of a user by using OAuth2 Protocol.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations oauth clients credentials create projects](ProjectLocationOauthClientCredentialCreateCall) (request|response)
/// * [locations oauth clients credentials get projects](ProjectLocationOauthClientCredentialGetCall) (response)
/// * [locations oauth clients credentials patch projects](ProjectLocationOauthClientCredentialPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OauthClientCredential {
    /// Output only. The system-generated oauth client secret.
    #[serde(rename="clientSecret")]
    
    pub client_secret: Option<String>,
    /// Optional. Whether the oauth client credential is disabled. You cannot use a disabled oauth client credential for OAuth.
    
    pub disabled: Option<bool>,
    /// Optional. A user-specified display name of the oauth client credential Cannot exceed 32 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Immutable. The resource name of the oauth client credential. Format: `projects/{project}/locations/{location}/oauthClients/{oauth_client}/credentials/{credential}`
    
    pub name: Option<String>,
}

impl client::RequestValue for OauthClientCredential {}
impl client::ResponseResult for OauthClientCredential {}


/// Represents an OpenId Connect 1.0 identity provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Oidc {
    /// Acceptable values for the `aud` field (audience) in the OIDC token. Token exchange requests are rejected if the token audience does not match one of the configured values. Each audience may be at most 256 characters. A maximum of 10 audiences may be configured. If this list is empty, the OIDC token audience must be equal to the full canonical resource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix. For example: ``` //iam.googleapis.com/projects//locations//workloadIdentityPools//providers/ https://iam.googleapis.com/projects//locations//workloadIdentityPools//providers/ ```
    #[serde(rename="allowedAudiences")]
    
    pub allowed_audiences: Option<Vec<String>>,
    /// Required. The OIDC issuer URL. Must be an HTTPS endpoint.
    #[serde(rename="issuerUri")]
    
    pub issuer_uri: Option<String>,
    /// Optional. OIDC JWKs in JSON String format. For details on the definition of a JWK, see https://tools.ietf.org/html/rfc7517. If not set, the `jwks_uri` from the discovery document(fetched from the .well-known path of the `issuer_uri`) will be used. Currently, RSA and EC asymmetric keys are supported. The JWK must use following format and include only the following fields: { "keys": [ { "kty": "RSA/EC", "alg": "", "use": "sig", "kid": "", "n": "", "e": "", "x": "", "y": "", "crv": "" } ] }
    #[serde(rename="jwksJson")]
    
    pub jwks_json: Option<String>,
}

impl client::Part for Oidc {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools operations get locations](LocationWorkforcePoolOperationGetCall) (response)
/// * [workforce pools providers keys operations get locations](LocationWorkforcePoolProviderKeyOperationGetCall) (response)
/// * [workforce pools providers keys create locations](LocationWorkforcePoolProviderKeyCreateCall) (response)
/// * [workforce pools providers keys delete locations](LocationWorkforcePoolProviderKeyDeleteCall) (response)
/// * [workforce pools providers keys undelete locations](LocationWorkforcePoolProviderKeyUndeleteCall) (response)
/// * [workforce pools providers operations get locations](LocationWorkforcePoolProviderOperationGetCall) (response)
/// * [workforce pools providers create locations](LocationWorkforcePoolProviderCreateCall) (response)
/// * [workforce pools providers delete locations](LocationWorkforcePoolProviderDeleteCall) (response)
/// * [workforce pools providers patch locations](LocationWorkforcePoolProviderPatchCall) (response)
/// * [workforce pools providers undelete locations](LocationWorkforcePoolProviderUndeleteCall) (response)
/// * [workforce pools subjects operations get locations](LocationWorkforcePoolSubjectOperationGetCall) (response)
/// * [workforce pools subjects delete locations](LocationWorkforcePoolSubjectDeleteCall) (response)
/// * [workforce pools subjects undelete locations](LocationWorkforcePoolSubjectUndeleteCall) (response)
/// * [workforce pools create locations](LocationWorkforcePoolCreateCall) (response)
/// * [workforce pools delete locations](LocationWorkforcePoolDeleteCall) (response)
/// * [workforce pools patch locations](LocationWorkforcePoolPatchCall) (response)
/// * [workforce pools undelete locations](LocationWorkforcePoolUndeleteCall) (response)
/// * [locations workload identity pools namespaces managed identities operations get projects](ProjectLocationWorkloadIdentityPoolNamespaceManagedIdentityOperationGetCall) (response)
/// * [locations workload identity pools namespaces managed identities workload sources operations get projects](ProjectLocationWorkloadIdentityPoolNamespaceManagedIdentityWorkloadSourceOperationGetCall) (response)
/// * [locations workload identity pools namespaces operations get projects](ProjectLocationWorkloadIdentityPoolNamespaceOperationGetCall) (response)
/// * [locations workload identity pools operations get projects](ProjectLocationWorkloadIdentityPoolOperationGetCall) (response)
/// * [locations workload identity pools providers keys operations get projects](ProjectLocationWorkloadIdentityPoolProviderKeyOperationGetCall) (response)
/// * [locations workload identity pools providers keys create projects](ProjectLocationWorkloadIdentityPoolProviderKeyCreateCall) (response)
/// * [locations workload identity pools providers keys delete projects](ProjectLocationWorkloadIdentityPoolProviderKeyDeleteCall) (response)
/// * [locations workload identity pools providers keys undelete projects](ProjectLocationWorkloadIdentityPoolProviderKeyUndeleteCall) (response)
/// * [locations workload identity pools providers operations get projects](ProjectLocationWorkloadIdentityPoolProviderOperationGetCall) (response)
/// * [locations workload identity pools providers create projects](ProjectLocationWorkloadIdentityPoolProviderCreateCall) (response)
/// * [locations workload identity pools providers delete projects](ProjectLocationWorkloadIdentityPoolProviderDeleteCall) (response)
/// * [locations workload identity pools providers patch projects](ProjectLocationWorkloadIdentityPoolProviderPatchCall) (response)
/// * [locations workload identity pools providers undelete projects](ProjectLocationWorkloadIdentityPoolProviderUndeleteCall) (response)
/// * [locations workload identity pools create projects](ProjectLocationWorkloadIdentityPoolCreateCall) (response)
/// * [locations workload identity pools delete projects](ProjectLocationWorkloadIdentityPoolDeleteCall) (response)
/// * [locations workload identity pools patch projects](ProjectLocationWorkloadIdentityPoolPatchCall) (response)
/// * [locations workload identity pools undelete projects](ProjectLocationWorkloadIdentityPoolUndeleteCall) (response)
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


/// The service account key patch request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys patch projects](ProjectServiceAccountKeyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PatchServiceAccountKeyRequest {
    /// Required. The service account key to update.
    #[serde(rename="serviceAccountKey")]
    
    pub service_account_key: Option<ServiceAccountKey>,
    /// Required. The update mask to apply to the service account key. Only the following fields are eligible for patching: - contact - description
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for PatchServiceAccountKeyRequest {}


/// The service account patch request. You can patch only the `display_name` and `description` fields. You must use the `update_mask` field to specify which of these fields you want to patch. Only the fields specified in the request are guaranteed to be returned in the response. Other fields may be empty in the response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts patch projects](ProjectServiceAccountPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PatchServiceAccountRequest {
    /// no description provided
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<ServiceAccount>,
    /// no description provided
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for PatchServiceAccountRequest {}


/// A permission which can be included by a role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query testable permissions permissions](PermissionQueryTestablePermissionCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// The service API associated with the permission is not enabled.
    #[serde(rename="apiDisabled")]
    
    pub api_disabled: Option<bool>,
    /// The current custom role support level.
    #[serde(rename="customRolesSupportLevel")]
    
    pub custom_roles_support_level: Option<PermissionCustomRolesSupportLevelEnum>,
    /// A brief description of what this Permission is used for. This permission can ONLY be used in predefined roles.
    
    pub description: Option<String>,
    /// The name of this Permission.
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="onlyInPredefinedRoles")]
    
    pub only_in_predefined_roles: Option<bool>,
    /// The preferred name for this permission. If present, then this permission is an alias of, and equivalent to, the listed primary_permission.
    #[serde(rename="primaryPermission")]
    
    pub primary_permission: Option<String>,
    /// The current launch stage of the permission.
    
    pub stage: Option<PermissionStageEnum>,
    /// The title of this Permission.
    
    pub title: Option<String>,
}

impl client::Resource for Permission {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools get iam policy locations](LocationWorkforcePoolGetIamPolicyCall) (response)
/// * [workforce pools set iam policy locations](LocationWorkforcePoolSetIamPolicyCall) (response)
/// * [service accounts get iam policy projects](ProjectServiceAccountGetIamPolicyCall) (response)
/// * [service accounts set iam policy projects](ProjectServiceAccountSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// A request to get the list of auditable services for a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query auditable services iam policies](IamPolicyQueryAuditableServiceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAuditableServicesRequest {
    /// Required. The full resource name to query from the list of auditable services. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
}

impl client::RequestValue for QueryAuditableServicesRequest {}


/// A response containing a list of auditable services for a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query auditable services iam policies](IamPolicyQueryAuditableServiceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAuditableServicesResponse {
    /// The auditable services for a resource.
    
    pub services: Option<Vec<AuditableService>>,
}

impl client::ResponseResult for QueryAuditableServicesResponse {}


/// The grantable role query request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query grantable roles roles](RoleQueryGrantableRoleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryGrantableRolesRequest {
    /// Required. The full resource name to query from the list of grantable roles. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// Optional limit on the number of roles to include in the response. The default is 300, and the maximum is 1,000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional pagination token returned in an earlier QueryGrantableRolesResponse.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// no description provided
    
    pub view: Option<QueryGrantableRolesRequestViewEnum>,
}

impl client::RequestValue for QueryGrantableRolesRequest {}


/// The grantable role query response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query grantable roles roles](RoleQueryGrantableRoleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryGrantableRolesResponse {
    /// To retrieve the next page of results, set `QueryGrantableRolesRequest.page_token` to this value.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching roles.
    
    pub roles: Option<Vec<Role>>,
}

impl client::ResponseResult for QueryGrantableRolesResponse {}


/// A request to get permissions which can be tested on a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query testable permissions permissions](PermissionQueryTestablePermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryTestablePermissionsRequest {
    /// Required. The full resource name to query from the list of testable permissions. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// Optional limit on the number of permissions to include in the response. The default is 100, and the maximum is 1,000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional pagination token returned in an earlier QueryTestablePermissionsRequest.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for QueryTestablePermissionsRequest {}


/// The response containing permissions which can be tested on a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query testable permissions permissions](PermissionQueryTestablePermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryTestablePermissionsResponse {
    /// To retrieve the next page of results, set `QueryTestableRolesRequest.page_token` to this value.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Permissions testable on the requested resource.
    
    pub permissions: Option<Vec<Permission>>,
}

impl client::ResponseResult for QueryTestablePermissionsResponse {}


/// A role in the Identity and Access Management API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [roles create organizations](OrganizationRoleCreateCall) (response)
/// * [roles delete organizations](OrganizationRoleDeleteCall) (response)
/// * [roles get organizations](OrganizationRoleGetCall) (response)
/// * [roles patch organizations](OrganizationRolePatchCall) (request|response)
/// * [roles undelete organizations](OrganizationRoleUndeleteCall) (response)
/// * [roles create projects](ProjectRoleCreateCall) (response)
/// * [roles delete projects](ProjectRoleDeleteCall) (response)
/// * [roles get projects](ProjectRoleGetCall) (response)
/// * [roles patch projects](ProjectRolePatchCall) (request|response)
/// * [roles undelete projects](ProjectRoleUndeleteCall) (response)
/// * [get roles](RoleGetCall) (response)
/// * [list roles](RoleListCall) (none)
/// * [query grantable roles roles](RoleQueryGrantableRoleCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    /// The current deleted state of the role. This field is read only. It will be ignored in calls to CreateRole and UpdateRole.
    
    pub deleted: Option<bool>,
    /// Optional. A human-readable description for the role.
    
    pub description: Option<String>,
    /// Used to perform a consistent read-modify-write.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// The names of the permissions this role grants when bound in an IAM policy.
    #[serde(rename="includedPermissions")]
    
    pub included_permissions: Option<Vec<String>>,
    /// The name of the role. When `Role` is used in `CreateRole`, the role name must not be set. When `Role` is used in output and other input such as `UpdateRole`, the role name is the complete path. For example, `roles/logging.viewer` for predefined roles, `organizations/{ORGANIZATION_ID}/roles/myRole` for organization-level custom roles, and `projects/{PROJECT_ID}/roles/myRole` for project-level custom roles.
    
    pub name: Option<String>,
    /// The current launch stage of the role. If the `ALPHA` launch stage has been selected for a role, the `stage` field will not be included in the returned definition for the role.
    
    pub stage: Option<RoleStageEnum>,
    /// Optional. A human-readable title for the role. Typically this is limited to 100 UTF-8 bytes.
    
    pub title: Option<String>,
}

impl client::RequestValue for Role {}
impl client::Resource for Role {}
impl client::ResponseResult for Role {}


/// Represents an SAML 2.0 identity provider.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Saml {
    /// Required. SAML identity provider (IdP) configuration metadata XML doc. The XML document must comply with the [SAML 2.0 specification](https://docs.oasis-open.org/security/saml/v2.0/saml-metadata-2.0-os.pdf). The maximum size of an acceptable XML document is 128K characters. The SAML metadata XML document must satisfy the following constraints: * Must contain an IdP Entity ID. * Must contain at least one non-expired signing certificate. * For each signing certificate, the expiration must be: * From no more than 7 days in the future. * To no more than 15 years in the future. * Up to three IdP signing keys are allowed. When updating the provider's metadata XML, at least one non-expired signing key must overlap with the existing metadata. This requirement is skipped if there are no non-expired signing keys present in the existing metadata.
    #[serde(rename="idpMetadataXml")]
    
    pub idp_metadata_xml: Option<String>,
}

impl client::Part for Saml {}


/// An IAM service account. A service account is an account for an application or a virtual machine (VM) instance, not a person. You can use a service account to call Google APIs. To learn more, read the [overview of service accounts](https://cloud.google.com/iam/help/service-accounts/overview). When you create a service account, you specify the project ID that owns the service account, as well as a name that must be unique within the project. IAM uses these values to create an email address that identifies the service account. //
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts create projects](ProjectServiceAccountCreateCall) (response)
/// * [service accounts get projects](ProjectServiceAccountGetCall) (response)
/// * [service accounts patch projects](ProjectServiceAccountPatchCall) (response)
/// * [service accounts update projects](ProjectServiceAccountUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// Optional. A user-specified, human-readable description of the service account. The maximum length is 256 UTF-8 bytes.
    
    pub description: Option<String>,
    /// Output only. Whether the service account is disabled.
    
    pub disabled: Option<bool>,
    /// Optional. A user-specified, human-readable name for the service account. The maximum length is 100 UTF-8 bytes.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The email address of the service account.
    
    pub email: Option<String>,
    /// Deprecated. Do not use.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAccounts/{EMAIL_ADDRESS}` * `projects/{PROJECT_ID}/serviceAccounts/{UNIQUE_ID}` As an alternative, you can use the `-` wildcard character instead of the project ID: * `projects/-/serviceAccounts/{EMAIL_ADDRESS}` * `projects/-/serviceAccounts/{UNIQUE_ID}` When possible, avoid using the `-` wildcard character, because it can cause response messages to contain misleading error codes. For example, if you try to access the service account `projects/-/serviceAccounts/fake@example.com`, which does not exist, the response contains an HTTP `403 Forbidden` error instead of a `404 Not Found` error.
    
    pub name: Option<String>,
    /// Output only. The OAuth 2.0 client ID for the service account.
    #[serde(rename="oauth2ClientId")]
    
    pub oauth2_client_id: Option<String>,
    /// Output only. The ID of the project that owns the service account.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. The unique, stable numeric ID for the service account. Each service account retains its unique ID even if you delete the service account. For example, if you delete a service account, then create a new service account with the same name, the new service account has a different unique ID than the deleted service account.
    #[serde(rename="uniqueId")]
    
    pub unique_id: Option<String>,
}

impl client::RequestValue for ServiceAccount {}
impl client::ResponseResult for ServiceAccount {}


/// Represents a service account key. A service account has two sets of key-pairs: user-managed, and system-managed. User-managed key-pairs can be created and deleted by users. Users are responsible for rotating these keys periodically to ensure security of their service accounts. Users retain the private key of these key-pairs, and Google retains ONLY the public key. System-managed keys are automatically rotated by Google, and are used for signing for a maximum of two weeks. The rotation process is probabilistic, and usage of the new key will gradually ramp up and down over the key’s lifetime. If you cache the public key set for a service account, we recommend that you update the cache every 15 minutes. User-managed keys can be added and removed at any time, so it is important to update the cache frequently. For Google-managed keys, Google will publish a key at least 6 hours before it is first used for signing and will keep publishing it for at least 6 hours after it was last used for signing. Public keys for all service accounts are also published at the OAuth2 Service Account API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys create projects](ProjectServiceAccountKeyCreateCall) (response)
/// * [service accounts keys get projects](ProjectServiceAccountKeyGetCall) (response)
/// * [service accounts keys patch projects](ProjectServiceAccountKeyPatchCall) (response)
/// * [service accounts keys upload projects](ProjectServiceAccountKeyUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountKey {
    /// Optional. A user provided email address as the point of contact for this service account key. Must be an email address. Limit 64 characters.
    
    pub contact: Option<String>,
    /// Output only. The cloud identity that created this service account key. Populated automatically when the key is created and not editable by the user.
    
    pub creator: Option<String>,
    /// Optional. A user provided description of this service account key.
    
    pub description: Option<String>,
    /// optional. If the key is disabled, it may have a DisableReason describing why it was disabled.
    #[serde(rename="disableReason")]
    
    pub disable_reason: Option<ServiceAccountKeyDisableReasonEnum>,
    /// The key status.
    
    pub disabled: Option<bool>,
    /// Extended Status provides permanent information about a service account key. For example, if this key was detected as exposed or compromised, that information will remain for the lifetime of the key in the extended_status.
    #[serde(rename="extendedStatus")]
    
    pub extended_status: Option<Vec<ExtendedStatus>>,
    /// Specifies the algorithm (and possibly key size) for the key.
    #[serde(rename="keyAlgorithm")]
    
    pub key_algorithm: Option<ServiceAccountKeyKeyAlgorithmEnum>,
    /// The key origin.
    #[serde(rename="keyOrigin")]
    
    pub key_origin: Option<ServiceAccountKeyKeyOriginEnum>,
    /// The key type.
    #[serde(rename="keyType")]
    
    pub key_type: Option<ServiceAccountKeyKeyTypeEnum>,
    /// The resource name of the service account key in the following format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    
    pub name: Option<String>,
    /// The private key data. Only provided in `CreateServiceAccountKey` responses. Make sure to keep the private key data secure because it allows for the assertion of the service account identity. When base64 decoded, the private key data can be used to authenticate with Google API client libraries and with gcloud auth activate-service-account.
    #[serde(rename="privateKeyData")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub private_key_data: Option<Vec<u8>>,
    /// The output format for the private key. Only provided in `CreateServiceAccountKey` responses, not in `GetServiceAccountKey` or `ListServiceAccountKey` responses. Google never exposes system-managed private keys, and never retains user-managed private keys.
    #[serde(rename="privateKeyType")]
    
    pub private_key_type: Option<ServiceAccountKeyPrivateKeyTypeEnum>,
    /// The public key data. Only provided in `GetServiceAccountKey` responses.
    #[serde(rename="publicKeyData")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub public_key_data: Option<Vec<u8>>,
    /// The key can be used after this timestamp.
    #[serde(rename="validAfterTime")]
    
    pub valid_after_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The key can be used before this timestamp. For system-managed key pairs, this timestamp is the end time for the private key signing operation. The public key could still be used for verification for a few hours after this time.
    #[serde(rename="validBeforeTime")]
    
    pub valid_before_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ServiceAccountKey {}


/// Configuration for a service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Optional. Domain name of the service. Example: console.cloud.google
    
    pub domain: Option<String>,
}

impl client::Part for ServiceConfig {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools set iam policy locations](LocationWorkforcePoolSetIamPolicyCall) (request)
/// * [service accounts set iam policy projects](ProjectServiceAccountSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign blob request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign blob projects](ProjectServiceAccountSignBlobCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignBlobRequest {
    /// Required. Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The bytes to sign.
    #[serde(rename="bytesToSign")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub bytes_to_sign: Option<Vec<u8>>,
}

impl client::RequestValue for SignBlobRequest {}


/// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign blob response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign blob projects](ProjectServiceAccountSignBlobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignBlobResponse {
    /// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The id of the key used to sign the blob.
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
    /// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The signed blob.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub signature: Option<Vec<u8>>,
}

impl client::ResponseResult for SignBlobResponse {}


/// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign JWT request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign jwt projects](ProjectServiceAccountSignJwtCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignJwtRequest {
    /// Required. Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The JWT payload to sign. Must be a serialized JSON object that contains a JWT Claims Set. For example: `{"sub": "user@example.com", "iat": 313435}` If the JWT Claims Set contains an expiration time (`exp`) claim, it must be an integer timestamp that is not in the past and no more than 12 hours in the future. If the JWT Claims Set does not contain an expiration time (`exp`) claim, this claim is added automatically, with a timestamp that is 1 hour in the future.
    
    pub payload: Option<String>,
}

impl client::RequestValue for SignJwtRequest {}


/// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign JWT response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts sign jwt projects](ProjectServiceAccountSignJwtCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignJwtResponse {
    /// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The id of the key used to sign the JWT.
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
    /// Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The signed JWT.
    #[serde(rename="signedJwt")]
    
    pub signed_jwt: Option<String>,
}

impl client::ResponseResult for SignJwtResponse {}


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


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools test iam permissions locations](LocationWorkforcePoolTestIamPermissionCall) (request)
/// * [service accounts test iam permissions projects](ProjectServiceAccountTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools test iam permissions locations](LocationWorkforcePoolTestIamPermissionCall) (response)
/// * [service accounts test iam permissions projects](ProjectServiceAccountTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Request message for UndeleteOauthClient.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations oauth clients undelete projects](ProjectLocationOauthClientUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteOauthClientRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteOauthClientRequest {}


/// The request to undelete an existing role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [roles undelete organizations](OrganizationRoleUndeleteCall) (request)
/// * [roles undelete projects](ProjectRoleUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteRoleRequest {
    /// Used to perform a consistent read-modify-write.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
}

impl client::RequestValue for UndeleteRoleRequest {}


/// The service account undelete request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts undelete projects](ProjectServiceAccountUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteServiceAccountRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteServiceAccountRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts undelete projects](ProjectServiceAccountUndeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteServiceAccountResponse {
    /// Metadata for the restored service account.
    #[serde(rename="restoredAccount")]
    
    pub restored_account: Option<ServiceAccount>,
}

impl client::ResponseResult for UndeleteServiceAccountResponse {}


/// Request message for UndeleteWorkforcePoolProviderKey.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools providers keys undelete locations](LocationWorkforcePoolProviderKeyUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkforcePoolProviderKeyRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkforcePoolProviderKeyRequest {}


/// Request message for UndeleteWorkforcePoolProvider.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools providers undelete locations](LocationWorkforcePoolProviderUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkforcePoolProviderRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkforcePoolProviderRequest {}


/// Request message for UndeleteWorkforcePool.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools undelete locations](LocationWorkforcePoolUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkforcePoolRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkforcePoolRequest {}


/// Request message for UndeleteWorkforcePoolSubject.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools subjects undelete locations](LocationWorkforcePoolSubjectUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkforcePoolSubjectRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkforcePoolSubjectRequest {}


/// Request message for UndeleteWorkloadIdentityPoolProviderKey.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools providers keys undelete projects](ProjectLocationWorkloadIdentityPoolProviderKeyUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkloadIdentityPoolProviderKeyRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkloadIdentityPoolProviderKeyRequest {}


/// Request message for UndeleteWorkloadIdentityPoolProvider.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools providers undelete projects](ProjectLocationWorkloadIdentityPoolProviderUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkloadIdentityPoolProviderRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkloadIdentityPoolProviderRequest {}


/// Request message for UndeleteWorkloadIdentityPool.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools undelete projects](ProjectLocationWorkloadIdentityPoolUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWorkloadIdentityPoolRequest { _never_set: Option<bool> }

impl client::RequestValue for UndeleteWorkloadIdentityPoolRequest {}


/// The service account key upload request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service accounts keys upload projects](ProjectServiceAccountKeyUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadServiceAccountKeyRequest {
    /// The public key to associate with the service account. Must be an RSA public key that is wrapped in an X.509 v3 certificate. Include the first line, `-----BEGIN CERTIFICATE-----`, and the last line, `-----END CERTIFICATE-----`.
    #[serde(rename="publicKeyData")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub public_key_data: Option<Vec<u8>>,
}

impl client::RequestValue for UploadServiceAccountKeyRequest {}


/// Represents a collection of external workforces. Provides namespaces for federated users that can be referenced in IAM policies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools create locations](LocationWorkforcePoolCreateCall) (request)
/// * [workforce pools get locations](LocationWorkforcePoolGetCall) (response)
/// * [workforce pools patch locations](LocationWorkforcePoolPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkforcePool {
    /// Optional. Configure access restrictions on the workforce pool users. This is an optional field. If specified web sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users.
    #[serde(rename="accessRestrictions")]
    
    pub access_restrictions: Option<AccessRestrictions>,
    /// A user-specified description of the pool. Cannot exceed 256 characters.
    
    pub description: Option<String>,
    /// Disables the workforce pool. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again.
    
    pub disabled: Option<bool>,
    /// A user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Time after which the workforce pool will be permanently purged and cannot be recovered.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name of the pool. Format: `locations/{location}/workforcePools/{workforce_pool_id}`
    
    pub name: Option<String>,
    /// Immutable. The resource name of the parent. Format: `organizations/{org-id}`.
    
    pub parent: Option<String>,
    /// Duration that the Google Cloud access tokens, console sign-in sessions, and `gcloud` sign-in sessions from this pool are valid. Must be greater than 15 minutes (900s) and less than 12 hours (43200s). If `session_duration` is not configured, minted credentials have a default duration of one hour (3600s). For SAML providers, the lifetime of the token is the minimum of the `session_duration` and the `SessionNotOnOrAfter` claim in the SAML assertion.
    #[serde(rename="sessionDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub session_duration: Option<client::chrono::Duration>,
    /// Output only. The state of the pool.
    
    pub state: Option<WorkforcePoolStateEnum>,
}

impl client::RequestValue for WorkforcePool {}
impl client::ResponseResult for WorkforcePool {}


/// A configuration for an external identity provider.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools providers create locations](LocationWorkforcePoolProviderCreateCall) (request)
/// * [workforce pools providers get locations](LocationWorkforcePoolProviderGetCall) (response)
/// * [workforce pools providers patch locations](LocationWorkforcePoolProviderPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkforcePoolProvider {
    /// A [Common Expression Language](https://opensource.google/projects/cel) expression, in plain text, to restrict what otherwise valid authentication credentials issued by the provider should not be accepted. The expression must output a boolean representing whether to allow the federation. The following keywords may be referenced in the expressions: * `assertion`: JSON representing the authentication credential issued by the provider. * `google`: The Google attributes mapped from the assertion in the `attribute_mappings`. `google.profile_photo`, `google.display_name` and `google.posix_username` are not supported. * `attribute`: The custom attributes mapped from the assertion in the `attribute_mappings`. The maximum length of the attribute condition expression is 4096 characters. If unspecified, all valid authentication credentials will be accepted. The following example shows how to only allow credentials with a mapped `google.groups` value of `admins`: ``` "'admins' in google.groups" ```
    #[serde(rename="attributeCondition")]
    
    pub attribute_condition: Option<String>,
    /// Required. Maps attributes from the authentication credentials issued by an external identity provider to Google Cloud attributes, such as `subject` and `segment`. Each key must be a string specifying the Google Cloud IAM attribute to map to. The following keys are supported: * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings. This is also the subject that appears in Cloud Logging logs. This is a required field and the mapped subject cannot exceed 127 bytes. * `google.groups`: Groups the authenticating user belongs to. You can grant groups access to resources using an IAM `principalSet` binding; access applies to all members of the group. * `google.display_name`: The name of the authenticated user. This is an optional field and the mapped display name cannot exceed 100 bytes. If not set, `google.subject` will be displayed instead. This attribute cannot be referenced in IAM bindings. * `google.profile_photo`: The URL that specifies the authenticated user's thumbnail photo. This is an optional field. When set, the image will be visible as the user's profile picture. If not set, a generic user icon will be displayed instead. This attribute cannot be referenced in IAM bindings. * `google.posix_username`: The Linux username used by OS Login. This is an optional field and the mapped POSIX username cannot exceed 32 characters, The key must match the regex "^a-zA-Z0-9._{0,31}$". This attribute cannot be referenced in IAM bindings. You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where {custom_attribute} is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes. The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_]. You can reference these attributes in IAM policies to define fine-grained access for a workforce pool to Google Cloud resources. For example: * `google.subject`: `principal://iam.googleapis.com/locations/global/workforcePools/{pool}/subject/{value}` * `google.groups`: `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool}/group/{value}` * `attribute.{custom_attribute}`: `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool}/attribute.{custom_attribute}/{value}` Each value must be a [Common Expression Language] (https://opensource.google/projects/cel) function that maps an identity provider credential to the normalized attribute specified by the corresponding map key. You can use the `assertion` keyword in the expression to access a JSON representation of the authentication credential issued by the provider. The maximum length of an attribute mapping expression is 2048 characters. When evaluated, the total size of all mapped attributes must not exceed 4KB. For OIDC providers, you must supply a custom mapping that includes the `google.subject` attribute. For example, the following maps the `sub` claim of the incoming credential to the `subject` attribute on a Google token: ``` {"google.subject": "assertion.sub"} ```
    #[serde(rename="attributeMapping")]
    
    pub attribute_mapping: Option<HashMap<String, String>>,
    /// A user-specified description of the provider. Cannot exceed 256 characters.
    
    pub description: Option<String>,
    /// Disables the workforce pool provider. You cannot use a disabled provider to exchange tokens. However, existing tokens still grant access.
    
    pub disabled: Option<bool>,
    /// A user-specified display name for the provider. Cannot exceed 32 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Time after which the workload pool provider will be permanently purged and cannot be recovered.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The configuration for OAuth 2.0 client used to get the additional user attributes. This should be used when users can't get the desired claims in authentication credentials. Currently this configuration is only supported with OIDC protocol.
    #[serde(rename="extraAttributesOauth2Client")]
    
    pub extra_attributes_oauth2_client: Option<GoogleIamAdminV1WorkforcePoolProviderExtraAttributesOAuth2Client>,
    /// Output only. The resource name of the provider. Format: `locations/{location}/workforcePools/{workforce_pool_id}/providers/{provider_id}`
    
    pub name: Option<String>,
    /// An OpenId Connect 1.0 identity provider configuration.
    
    pub oidc: Option<GoogleIamAdminV1WorkforcePoolProviderOidc>,
    /// A SAML identity provider configuration.
    
    pub saml: Option<GoogleIamAdminV1WorkforcePoolProviderSaml>,
    /// Output only. The state of the provider.
    
    pub state: Option<WorkforcePoolProviderStateEnum>,
}

impl client::RequestValue for WorkforcePoolProvider {}
impl client::ResponseResult for WorkforcePoolProvider {}


/// Represents a public key configuration for a Workforce Pool Provider. The key can be configured in your identity provider to encrypt SAML assertions. Google holds the corresponding private key, which it uses to decrypt encrypted tokens.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [workforce pools providers keys create locations](LocationWorkforcePoolProviderKeyCreateCall) (request)
/// * [workforce pools providers keys get locations](LocationWorkforcePoolProviderKeyGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkforcePoolProviderKey {
    /// Output only. The time after which the key will be permanently deleted and cannot be recovered. Note that the key may get purged before this time if the total limit of keys per provider is exceeded.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. Public half of the asymmetric key.
    #[serde(rename="keyData")]
    
    pub key_data: Option<KeyData>,
    /// Output only. The resource name of the key.
    
    pub name: Option<String>,
    /// Output only. The state of the key.
    
    pub state: Option<WorkforcePoolProviderKeyStateEnum>,
    /// Required. The purpose of the key.
    #[serde(rename="use")]
    
    pub use_: Option<WorkforcePoolProviderKeyUseEnum>,
}

impl client::RequestValue for WorkforcePoolProviderKey {}
impl client::ResponseResult for WorkforcePoolProviderKey {}


/// Represents a collection of workload identities. You can define IAM policies to grant these identities access to Google Cloud resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools create projects](ProjectLocationWorkloadIdentityPoolCreateCall) (request)
/// * [locations workload identity pools get projects](ProjectLocationWorkloadIdentityPoolGetCall) (response)
/// * [locations workload identity pools patch projects](ProjectLocationWorkloadIdentityPoolPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadIdentityPool {
    /// A description of the pool. Cannot exceed 256 characters.
    
    pub description: Option<String>,
    /// Whether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again.
    
    pub disabled: Option<bool>,
    /// A display name for the pool. Cannot exceed 32 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Time after which the workload identity pool will be permanently purged and cannot be recovered.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name of the pool.
    
    pub name: Option<String>,
    /// Output only. The state of the pool.
    
    pub state: Option<WorkloadIdentityPoolStateEnum>,
}

impl client::RequestValue for WorkloadIdentityPool {}
impl client::ResponseResult for WorkloadIdentityPool {}


/// A configuration for an external identity provider.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools providers create projects](ProjectLocationWorkloadIdentityPoolProviderCreateCall) (request)
/// * [locations workload identity pools providers get projects](ProjectLocationWorkloadIdentityPoolProviderGetCall) (response)
/// * [locations workload identity pools providers patch projects](ProjectLocationWorkloadIdentityPoolProviderPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadIdentityPoolProvider {
    /// [A Common Expression Language](https://opensource.google/projects/cel) expression, in plain text, to restrict what otherwise valid authentication credentials issued by the provider should not be accepted. The expression must output a boolean representing whether to allow the federation. The following keywords may be referenced in the expressions: * `assertion`: JSON representing the authentication credential issued by the provider. * `google`: The Google attributes mapped from the assertion in the `attribute_mappings`. * `attribute`: The custom attributes mapped from the assertion in the `attribute_mappings`. The maximum length of the attribute condition expression is 4096 characters. If unspecified, all valid authentication credential are accepted. The following example shows how to only allow credentials with a mapped `google.groups` value of `admins`: ``` "'admins' in google.groups" ```
    #[serde(rename="attributeCondition")]
    
    pub attribute_condition: Option<String>,
    ///  Maps attributes from authentication credentials issued by an external identity provider to Google Cloud attributes, such as `subject` and `segment`. Each key must be a string specifying the Google Cloud IAM attribute to map to. The following keys are supported: * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings. This is also the subject that appears in Cloud Logging logs. Cannot exceed 127 bytes. * `google.groups`: Groups the external identity belongs to. You can grant groups access to resources using an IAM `principalSet` binding; access applies to all members of the group. You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where `{custom_attribute}` is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes. The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_]. You can reference these attributes in IAM policies to define fine-grained access for a workload to Google Cloud resources. For example: * `google.subject`: `principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}` * `google.groups`: `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}` * `attribute.{custom_attribute}`: `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}` Each value must be a [Common Expression Language] (https://opensource.google/projects/cel) function that maps an identity provider credential to the normalized attribute specified by the corresponding map key. You can use the `assertion` keyword in the expression to access a JSON representation of the authentication credential issued by the provider. The maximum length of an attribute mapping expression is 2048 characters. When evaluated, the total size of all mapped attributes must not exceed 8KB. For AWS providers, if no attribute mapping is defined, the following default mapping applies: ``` { "google.subject":"assertion.arn", "attribute.aws_role": "assertion.arn.contains('assumed-role')" " ? assertion.arn.extract('{account_arn}assumed-role/')" " + 'assumed-role/'" " + assertion.arn.extract('assumed-role/{role_name}/')" " : assertion.arn", } ``` If any custom attribute mappings are defined, they must include a mapping to the `google.subject` attribute. For OIDC providers, you must supply a custom mapping, which must include the `google.subject` attribute. For example, the following maps the `sub` claim of the incoming credential to the `subject` attribute on a Google token: ``` {"google.subject": "assertion.sub"} ```
    #[serde(rename="attributeMapping")]
    
    pub attribute_mapping: Option<HashMap<String, String>>,
    /// An Amazon Web Services identity provider.
    
    pub aws: Option<Aws>,
    /// A description for the provider. Cannot exceed 256 characters.
    
    pub description: Option<String>,
    /// Whether the provider is disabled. You cannot use a disabled provider to exchange tokens. However, existing tokens still grant access.
    
    pub disabled: Option<bool>,
    /// A display name for the provider. Cannot exceed 32 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Time after which the workload identity pool provider will be permanently purged and cannot be recovered.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name of the provider.
    
    pub name: Option<String>,
    /// An OpenId Connect 1.0 identity provider.
    
    pub oidc: Option<Oidc>,
    /// An SAML 2.0 identity provider.
    
    pub saml: Option<Saml>,
    /// Output only. The state of the provider.
    
    pub state: Option<WorkloadIdentityPoolProviderStateEnum>,
}

impl client::RequestValue for WorkloadIdentityPoolProvider {}
impl client::ResponseResult for WorkloadIdentityPoolProvider {}


/// Represents a public key configuration for your workload identity pool provider. The key can be configured in your identity provider to encrypt the SAML assertions. Google holds the corresponding private key which it uses to decrypt encrypted tokens.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations workload identity pools providers keys create projects](ProjectLocationWorkloadIdentityPoolProviderKeyCreateCall) (request)
/// * [locations workload identity pools providers keys get projects](ProjectLocationWorkloadIdentityPoolProviderKeyGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkloadIdentityPoolProviderKey {
    /// Output only. Time after which the key will be permanently purged and cannot be recovered. Note that the key may get purged before this timestamp if the total limit of keys per provider is crossed.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Immutable. Public half of the asymmetric key.
    #[serde(rename="keyData")]
    
    pub key_data: Option<KeyData>,
    /// Output only. The resource name of the key.
    
    pub name: Option<String>,
    /// Output only. The state of the key.
    
    pub state: Option<WorkloadIdentityPoolProviderKeyStateEnum>,
    /// Required. The purpose of the key.
    #[serde(rename="use")]
    
    pub use_: Option<WorkloadIdentityPoolProviderKeyUseEnum>,
}

impl client::RequestValue for WorkloadIdentityPoolProviderKey {}
impl client::ResponseResult for WorkloadIdentityPoolProviderKey {}


