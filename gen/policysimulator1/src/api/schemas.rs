use super::*;
/// Similar to PolicySpec but with an extra 'launch' field for launch reference. The PolicySpec here is specific for dry-run/darklaunch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2AlternatePolicySpec {
    /// Reference to the launch that will be used while audit logging and to control the launch. Should be set only in the alternate policy.
    
    pub launch: Option<String>,
    /// Specify constraint for configurations of Google Cloud resources.
    
    pub spec: Option<GoogleCloudOrgpolicyV2PolicySpec>,
}

impl client::Part for GoogleCloudOrgpolicyV2AlternatePolicySpec {}


/// A custom constraint defined by customers which can *only* be applied to the given resource types and organization. By creating a custom constraint, customers can apply policies of this custom constraint. *Creating a custom constraint itself does NOT apply any policy enforcement*.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2CustomConstraint {
    /// Allow or deny type.
    #[serde(rename="actionType")]
    
    pub action_type: Option<GoogleCloudOrgpolicyV2CustomConstraintActionTypeEnum>,
    /// Org policy condition/expression. For example: `resource.instanceName.matches("[production|test]_.*_(\d)+")` or, `resource.management.auto_upgrade == true` The max length of the condition is 1000 characters.
    
    pub condition: Option<String>,
    /// Detailed information about this custom policy constraint. The max length of the description is 2000 characters.
    
    pub description: Option<String>,
    /// One line display name for the UI. The max length of the display_name is 200 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// All the operations being applied for this constraint.
    #[serde(rename="methodTypes")]
    
    pub method_types: Option<Vec<GoogleCloudOrgpolicyV2CustomConstraintMethodTypesEnum>>,
    /// Immutable. Name of the constraint. This is unique within the organization. Format of the name should be * `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms` The max length is 70 characters and the minimum length is 1. Note that the prefix `organizations/{organization_id}/customConstraints/` is not counted.
    
    pub name: Option<String>,
    /// Immutable. The resource instance type on which this policy applies. Format will be of the form : `/` Example: * `compute.googleapis.com/Instance`.
    #[serde(rename="resourceTypes")]
    
    pub resource_types: Option<Vec<String>>,
    /// Output only. The last time this custom constraint was updated. This represents the last time that the `CreateCustomConstraint` or `UpdateCustomConstraint` RPC was called
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudOrgpolicyV2CustomConstraint {}


/// Defines an organization policy which is used to specify constraints for configurations of Google Cloud resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2Policy {
    /// Deprecated.
    
    pub alternate: Option<GoogleCloudOrgpolicyV2AlternatePolicySpec>,
    /// Dry-run policy. Audit-only policy, can be used to monitor how the policy would have impacted the existing and future resources if it's enforced.
    #[serde(rename="dryRunSpec")]
    
    pub dry_run_spec: Option<GoogleCloudOrgpolicyV2PolicySpec>,
    /// Optional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Immutable. The resource name of the policy. Must be one of the following forms, where `constraint_name` is the name of the constraint which this policy configures: * `projects/{project_number}/policies/{constraint_name}` * `folders/{folder_id}/policies/{constraint_name}` * `organizations/{organization_id}/policies/{constraint_name}` For example, `projects/123/policies/compute.disableSerialPortAccess`. Note: `projects/{project_id}/policies/{constraint_name}` is also an acceptable name for API requests, but responses will return the name using the equivalent project number.
    
    pub name: Option<String>,
    /// Basic information about the Organization Policy.
    
    pub spec: Option<GoogleCloudOrgpolicyV2PolicySpec>,
}

impl client::Part for GoogleCloudOrgpolicyV2Policy {}


/// Defines a Google Cloud policy specification which is used to specify constraints for configurations of Google Cloud resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2PolicySpec {
    /// An opaque tag indicating the current version of the policySpec, used for concurrency control. This field is ignored if used in a `CreatePolicy` request. When the policy is returned from either a `GetPolicy` or a `ListPolicies` request, this `etag` indicates the version of the current policySpec to use when executing a read-modify-write loop. When the policy is returned from a `GetEffectivePolicy` request, the `etag` will be unset.
    
    pub etag: Option<String>,
    /// Determines the inheritance behavior for this policy. If `inherit_from_parent` is true, policy rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the new root for evaluation. This field can be set only for policies which configure list constraints.
    #[serde(rename="inheritFromParent")]
    
    pub inherit_from_parent: Option<bool>,
    /// Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, `rules` must be empty and `inherit_from_parent` must be set to false.
    
    pub reset: Option<bool>,
    /// In policies for boolean constraints, the following requirements apply: - There must be one and only one policy rule where condition is unset. - Boolean policy rules with conditions must set `enforced` to the opposite of the policy rule without a condition. - During policy evaluation, policy rules with conditions that are true for a target resource take precedence.
    
    pub rules: Option<Vec<GoogleCloudOrgpolicyV2PolicySpecPolicyRule>>,
    /// Output only. The time stamp this was previously updated. This represents the last time a call to `CreatePolicy` or `UpdatePolicy` was made for that policy.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudOrgpolicyV2PolicySpec {}


/// A rule used to express this policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2PolicySpecPolicyRule {
    /// Setting this to true means that all values are allowed. This field can be set only in policies for list constraints.
    #[serde(rename="allowAll")]
    
    pub allow_all: Option<bool>,
    /// A condition which determines whether this rule is used in the evaluation of the policy. When set, the `expression` field in the `Expr' must include from 1 to 10 subexpressions, joined by the "||" or "&&" operators. Each subexpression must be of the form "resource.matchTag('/tag_key_short_name, 'tag_value_short_name')". or "resource.matchTagId('tagKeys/key_id', 'tagValues/value_id')". where key_name and value_name are the resource names for Label Keys and Values. These names are available from the Tag Manager Service. An example expression is: "resource.matchTag('123456789/environment, 'prod')". or "resource.matchTagId('tagKeys/123', 'tagValues/456')".
    
    pub condition: Option<GoogleTypeExpr>,
    /// Setting this to true means that all values are denied. This field can be set only in policies for list constraints.
    #[serde(rename="denyAll")]
    
    pub deny_all: Option<bool>,
    /// If `true`, then the policy is enforced. If `false`, then any configuration is acceptable. This field can be set only in policies for boolean constraints.
    
    pub enforce: Option<bool>,
    /// List of values to be used for this policy rule. This field can be set only in policies for list constraints.
    
    pub values: Option<GoogleCloudOrgpolicyV2PolicySpecPolicyRuleStringValues>,
}

impl client::Part for GoogleCloudOrgpolicyV2PolicySpecPolicyRule {}


/// A message that holds specific allowed and denied values. This message can define specific values and subtrees of the Resource Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that are allowed or denied. This is achieved by using the `under:` and optional `is:` prefixes. The `under:` prefix is used to denote resource subtree values. The `is:` prefix is used to denote specific values, and is required only if the value contains a ":". Values prefixed with "is:" are treated the same as values with no prefix. Ancestry subtrees must be in one of the following formats: - `projects/` (for example, `projects/tokyo-rain-123`) - `folders/` (for example, `folders/1234`) - `organizations/` (for example, `organizations/1234`) The `supports_under` field of the associated `Constraint` defines whether ancestry prefixes can be used.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudOrgpolicyV2PolicySpecPolicyRuleStringValues {
    /// List of values allowed at this resource.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<String>>,
    /// List of values denied at this resource.
    #[serde(rename="deniedValues")]
    
    pub denied_values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudOrgpolicyV2PolicySpecPolicyRuleStringValues {}


/// A summary and comparison of the principal's access under the current (baseline) policies and the proposed (simulated) policies for a single access tuple.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1AccessStateDiff {
    /// How the principal's access, specified in the AccessState field, changed between the current (baseline) policies and proposed (simulated) policies.
    #[serde(rename="accessChange")]
    
    pub access_change: Option<GoogleCloudPolicysimulatorV1AccessStateDiffAccessChangeEnum>,
    /// The results of evaluating the access tuple under the current (baseline) policies. If the AccessState couldn't be fully evaluated, this field explains why.
    
    pub baseline: Option<GoogleCloudPolicysimulatorV1ExplainedAccess>,
    /// The results of evaluating the access tuple under the proposed (simulated) policies. If the AccessState couldn't be fully evaluated, this field explains why.
    
    pub simulated: Option<GoogleCloudPolicysimulatorV1ExplainedAccess>,
}

impl client::Part for GoogleCloudPolicysimulatorV1AccessStateDiff {}


/// Information about the principal, resource, and permission to check.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1AccessTuple {
    /// Required. The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// Required. The IAM permission to check for the specified principal and resource. For a complete list of IAM permissions, see https://cloud.google.com/iam/help/permissions/reference. For a complete list of predefined IAM roles and the permissions in each role, see https://cloud.google.com/iam/help/roles/reference.
    
    pub permission: Option<String>,
    /// Required. The principal whose access you want to check, in the form of the email address that represents that principal. For example, `alice@example.com` or `my-service-account@my-project.iam.gserviceaccount.com`. The principal must be a Google Account or a service account. Other types of principals are not supported.
    
    pub principal: Option<String>,
}

impl client::Part for GoogleCloudPolicysimulatorV1AccessTuple {}


/// Details about how a binding in a policy affects a principal's ability to use a permission.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1BindingExplanation {
    /// Required. Indicates whether _this binding_ provides the specified permission to the specified principal for the specified resource. This field does _not_ indicate whether the principal actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse.
    
    pub access: Option<GoogleCloudPolicysimulatorV1BindingExplanationAccessEnum>,
    /// A condition expression that prevents this binding from granting access unless the expression evaluates to `true`. To learn about IAM Conditions, see https://cloud.google.com/iam/docs/conditions-overview.
    
    pub condition: Option<GoogleTypeExpr>,
    /// Indicates whether each principal in the binding includes the principal specified in the request, either directly or indirectly. Each key identifies a principal in the binding, and each value indicates whether the principal in the binding includes the principal in the request. For example, suppose that a binding includes the following principals: * `user:alice@example.com` * `group:product-eng@example.com` The principal in the replayed access tuple is `user:bob@example.com`. This user is a principal of the group `group:product-eng@example.com`. For the first principal in the binding, the key is `user:alice@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_NOT_INCLUDED`. For the second principal in the binding, the key is `group:product-eng@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_INCLUDED`.
    
    pub memberships: Option<HashMap<String, GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembership>>,
    /// The relevance of this binding to the overall determination for the entire policy.
    
    pub relevance: Option<GoogleCloudPolicysimulatorV1BindingExplanationRelevanceEnum>,
    /// The role that this binding grants. For example, `roles/compute.serviceAgent`. For a complete list of predefined IAM roles, as well as the permissions in each role, see https://cloud.google.com/iam/help/roles/reference.
    
    pub role: Option<String>,
    /// Indicates whether the role granted by this binding contains the specified permission.
    #[serde(rename="rolePermission")]
    
    pub role_permission: Option<GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionEnum>,
    /// The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy.
    #[serde(rename="rolePermissionRelevance")]
    
    pub role_permission_relevance: Option<GoogleCloudPolicysimulatorV1BindingExplanationRolePermissionRelevanceEnum>,
}

impl client::Part for GoogleCloudPolicysimulatorV1BindingExplanation {}


/// Details about whether the binding includes the principal.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembership {
    /// Indicates whether the binding includes the principal.
    
    pub membership: Option<GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipMembershipEnum>,
    /// The relevance of the principal's status to the overall determination for the binding.
    
    pub relevance: Option<GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembershipRelevanceEnum>,
}

impl client::Part for GoogleCloudPolicysimulatorV1BindingExplanationAnnotatedMembership {}


/// Details about how a set of policies, listed in ExplainedPolicy, resulted in a certain AccessState when replaying an access tuple.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ExplainedAccess {
    /// Whether the principal in the access tuple has permission to access the resource in the access tuple under the given policies.
    #[serde(rename="accessState")]
    
    pub access_state: Option<GoogleCloudPolicysimulatorV1ExplainedAccesAccessStateEnum>,
    /// If the AccessState is `UNKNOWN`, this field contains a list of errors explaining why the result is `UNKNOWN`. If the `AccessState` is `GRANTED` or `NOT_GRANTED`, this field is omitted.
    
    pub errors: Option<Vec<GoogleRpcStatus>>,
    /// If the AccessState is `UNKNOWN`, this field contains the policies that led to that result. If the `AccessState` is `GRANTED` or `NOT_GRANTED`, this field is omitted.
    
    pub policies: Option<Vec<GoogleCloudPolicysimulatorV1ExplainedPolicy>>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ExplainedAccess {}


/// Details about how a specific IAM Policy contributed to the access check.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ExplainedPolicy {
    /// Indicates whether _this policy_ provides the specified permission to the specified principal for the specified resource. This field does _not_ indicate whether the principal actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the principal actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse.
    
    pub access: Option<GoogleCloudPolicysimulatorV1ExplainedPolicyAccessEnum>,
    /// Details about how each binding in the policy affects the principal's ability, or inability, to use the permission for the resource. If the user who created the Replay does not have access to the policy, this field is omitted.
    #[serde(rename="bindingExplanations")]
    
    pub binding_explanations: Option<Vec<GoogleCloudPolicysimulatorV1BindingExplanation>>,
    /// The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. If the user who created the Replay does not have access to the policy, this field is omitted. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names.
    #[serde(rename="fullResourceName")]
    
    pub full_resource_name: Option<String>,
    /// The IAM policy attached to the resource. If the user who created the Replay does not have access to the policy, this field is empty.
    
    pub policy: Option<GoogleIamV1Policy>,
    /// The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the user who created the Replay does not have access to the policy, this field is omitted.
    
    pub relevance: Option<GoogleCloudPolicysimulatorV1ExplainedPolicyRelevanceEnum>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ExplainedPolicy {}


/// ListOrgPolicyViolationsPreviewsResponse is the response message for OrgPolicyViolationsPreviewService.ListOrgPolicyViolationsPreviews.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations org policy violations previews list organizations](OrganizationLocationOrgPolicyViolationsPreviewListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ListOrgPolicyViolationsPreviewsResponse {
    /// A token that you can use to retrieve the next page of results. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of OrgPolicyViolationsPreview
    #[serde(rename="orgPolicyViolationsPreviews")]
    
    pub org_policy_violations_previews: Option<Vec<GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreview>>,
}

impl client::ResponseResult for GoogleCloudPolicysimulatorV1ListOrgPolicyViolationsPreviewsResponse {}


/// ListOrgPolicyViolationsResponse is the response message for OrgPolicyViolationsPreviewService.ListOrgPolicyViolations
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations org policy violations previews org policy violations list organizations](OrganizationLocationOrgPolicyViolationsPreviewOrgPolicyViolationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ListOrgPolicyViolationsResponse {
    /// A token that you can use to retrieve the next page of results. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of OrgPolicyViolations
    #[serde(rename="orgPolicyViolations")]
    
    pub org_policy_violations: Option<Vec<GoogleCloudPolicysimulatorV1OrgPolicyViolation>>,
}

impl client::ResponseResult for GoogleCloudPolicysimulatorV1ListOrgPolicyViolationsResponse {}


/// Response message for Simulator.ListReplayResults.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations replays results list folders](FolderLocationReplayResultListCall) (response)
/// * [locations replays results list organizations](OrganizationLocationReplayResultListCall) (response)
/// * [locations replays results list projects](ProjectLocationReplayResultListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ListReplayResultsResponse {
    /// A token that you can use to retrieve the next page of ReplayResult objects. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The results of running a Replay.
    #[serde(rename="replayResults")]
    
    pub replay_results: Option<Vec<GoogleCloudPolicysimulatorV1ReplayResult>>,
}

impl client::ResponseResult for GoogleCloudPolicysimulatorV1ListReplayResultsResponse {}


/// The proposed changes to OrgPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1OrgPolicyOverlay {
    /// Optional. The OrgPolicy CustomConstraint changes to preview violations for. Any existing CustomConstraints with the same name will be overridden in the simulation. That is, violations will be determined as if all custom constraints in the overlay were instantiated. Only a single custom_constraint is supported in the overlay at a time. For evaluating multiple constraints, multiple `GenerateOrgPolicyViolationsPreview` requests are made, where each request evaluates a single constraint.
    #[serde(rename="customConstraints")]
    
    pub custom_constraints: Option<Vec<GoogleCloudPolicysimulatorV1OrgPolicyOverlayCustomConstraintOverlay>>,
    /// Optional. The OrgPolicy changes to preview violations for. Any existing OrgPolicies with the same name will be overridden in the simulation. That is, violations will be determined as if all policies in the overlay were created or updated.
    
    pub policies: Option<Vec<GoogleCloudPolicysimulatorV1OrgPolicyOverlayPolicyOverlay>>,
}

impl client::Part for GoogleCloudPolicysimulatorV1OrgPolicyOverlay {}


/// A change to an OrgPolicy custom constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1OrgPolicyOverlayCustomConstraintOverlay {
    /// Optional. The new or updated custom constraint.
    #[serde(rename="customConstraint")]
    
    pub custom_constraint: Option<GoogleCloudOrgpolicyV2CustomConstraint>,
    /// Optional. Resource the constraint is attached to. Example: "organization/987654"
    #[serde(rename="customConstraintParent")]
    
    pub custom_constraint_parent: Option<String>,
}

impl client::Part for GoogleCloudPolicysimulatorV1OrgPolicyOverlayCustomConstraintOverlay {}


/// A change to an OrgPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1OrgPolicyOverlayPolicyOverlay {
    /// Optional. The new or updated OrgPolicy.
    
    pub policy: Option<GoogleCloudOrgpolicyV2Policy>,
    /// Optional. The parent of the policy we are attaching to. Example: "projects/123456"
    #[serde(rename="policyParent")]
    
    pub policy_parent: Option<String>,
}

impl client::Part for GoogleCloudPolicysimulatorV1OrgPolicyOverlayPolicyOverlay {}


/// OrgPolicyViolation is a resource representing a single resource violating a single OrgPolicy constraint.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1OrgPolicyViolation {
    /// The custom constraint being violated.
    #[serde(rename="customConstraint")]
    
    pub custom_constraint: Option<GoogleCloudOrgpolicyV2CustomConstraint>,
    /// Any error encountered during the evaluation.
    
    pub error: Option<GoogleRpcStatus>,
    /// The name of the `OrgPolicyViolation`. Example: organizations/my-example-org/locations/global/orgPolicyViolationsPreviews/506a5f7f/orgPolicyViolations/38ce`
    
    pub name: Option<String>,
    /// The resource violating the constraint.
    
    pub resource: Option<GoogleCloudPolicysimulatorV1ResourceContext>,
}

impl client::Part for GoogleCloudPolicysimulatorV1OrgPolicyViolation {}


/// OrgPolicyViolationsPreview is a resource providing a preview of the violations that will exist if an OrgPolicy change is made. The list of violations are modeled as child resources and retrieved via a ListOrgPolicyViolations API call. There are potentially more OrgPolicyViolations than could fit in an embedded field. Thus, the use of a child resource instead of a field.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations org policy violations previews create organizations](OrganizationLocationOrgPolicyViolationsPreviewCreateCall) (request)
/// * [locations org policy violations previews get organizations](OrganizationLocationOrgPolicyViolationsPreviewGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreview {
    /// Output only. Time when this `OrgPolicyViolationsPreview` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The names of the constraints against which all `OrgPolicyViolations` were evaluated. If `OrgPolicyOverlay` only contains `PolicyOverlay` then it contains the name of the configured custom constraint, applicable to the specified policies. Otherwise it contains the name of the constraint specified in `CustomConstraintOverlay`. Format: `organizations/{organization_id}/customConstraints/{custom_constraint_id}` Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms`
    #[serde(rename="customConstraints")]
    
    pub custom_constraints: Option<Vec<String>>,
    /// Output only. The resource name of the `OrgPolicyViolationsPreview`. It has the following format: `organizations/{organization}/locations/{location}/orgPolicyViolationsPreviews/{orgPolicyViolationsPreview}` Example: `organizations/my-example-org/locations/global/orgPolicyViolationsPreviews/506a5f7f`
    
    pub name: Option<String>,
    /// Required. The proposed changes we are previewing violations for.
    
    pub overlay: Option<GoogleCloudPolicysimulatorV1OrgPolicyOverlay>,
    /// Output only. A summary of the state of all resources scanned for compliance with the changed OrgPolicy.
    #[serde(rename="resourceCounts")]
    
    pub resource_counts: Option<GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewResourceCounts>,
    /// Output only. The state of the `OrgPolicyViolationsPreview`.
    
    pub state: Option<GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewStateEnum>,
    /// Output only. The number of OrgPolicyViolations in this `OrgPolicyViolationsPreview`. This count may differ from `resource_summary.noncompliant_count` because each OrgPolicyViolation is specific to a resource **and** constraint. If there are multiple constraints being evaluated (i.e. multiple policies in the overlay), a single resource may violate multiple constraints.
    #[serde(rename="violationsCount")]
    
    pub violations_count: Option<i32>,
}

impl client::RequestValue for GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreview {}
impl client::ResponseResult for GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreview {}


/// A summary of the state of all resources scanned for compliance with the changed OrgPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewResourceCounts {
    /// Output only. Number of scanned resources with zero violations.
    
    pub compliant: Option<i32>,
    /// Output only. Number of resources that returned an error when scanned.
    
    pub errors: Option<i32>,
    /// Output only. Number of scanned resources with at least one violation.
    
    pub noncompliant: Option<i32>,
    /// Output only. Number of resources checked for compliance. Must equal: unenforced + noncompliant + compliant + error
    
    pub scanned: Option<i32>,
    /// Output only. Number of resources where the constraint was not enforced, i.e. the Policy set `enforced: false` for that resource.
    
    pub unenforced: Option<i32>,
}

impl client::Part for GoogleCloudPolicysimulatorV1OrgPolicyViolationsPreviewResourceCounts {}


/// A resource describing a `Replay`, or simulation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations replays create folders](FolderLocationReplayCreateCall) (request)
/// * [locations replays get folders](FolderLocationReplayGetCall) (response)
/// * [locations replays create organizations](OrganizationLocationReplayCreateCall) (request)
/// * [locations replays get organizations](OrganizationLocationReplayGetCall) (response)
/// * [locations replays create projects](ProjectLocationReplayCreateCall) (request)
/// * [locations replays get projects](ProjectLocationReplayGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1Replay {
    /// Required. The configuration used for the `Replay`.
    
    pub config: Option<GoogleCloudPolicysimulatorV1ReplayConfig>,
    /// Output only. The resource name of the `Replay`, which has the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36`
    
    pub name: Option<String>,
    /// Output only. Summary statistics about the replayed log entries.
    #[serde(rename="resultsSummary")]
    
    pub results_summary: Option<GoogleCloudPolicysimulatorV1ReplayResultsSummary>,
    /// Output only. The current state of the `Replay`.
    
    pub state: Option<GoogleCloudPolicysimulatorV1ReplayStateEnum>,
}

impl client::RequestValue for GoogleCloudPolicysimulatorV1Replay {}
impl client::ResponseResult for GoogleCloudPolicysimulatorV1Replay {}


/// The configuration used for a Replay.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ReplayConfig {
    /// The logs to use as input for the Replay.
    #[serde(rename="logSource")]
    
    pub log_source: Option<GoogleCloudPolicysimulatorV1ReplayConfigLogSourceEnum>,
    /// A mapping of the resources that you want to simulate policies for and the policies that you want to simulate. Keys are the full resource names for the resources. For example, `//cloudresourcemanager.googleapis.com/projects/my-project`. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names. Values are Policy objects representing the policies that you want to simulate. Replays automatically take into account any IAM policies inherited through the resource hierarchy, and any policies set on descendant resources. You do not need to include these policies in the policy overlay.
    #[serde(rename="policyOverlay")]
    
    pub policy_overlay: Option<HashMap<String, GoogleIamV1Policy>>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ReplayConfig {}


/// The difference between the results of evaluating an access tuple under the current (baseline) policies and under the proposed (simulated) policies. This difference explains how a principal's access could change if the proposed policies were applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ReplayDiff {
    /// A summary and comparison of the principal's access under the current (baseline) policies and the proposed (simulated) policies for a single access tuple. The evaluation of the principal's access is reported in the AccessState field.
    #[serde(rename="accessDiff")]
    
    pub access_diff: Option<GoogleCloudPolicysimulatorV1AccessStateDiff>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ReplayDiff {}


/// The result of replaying a single access tuple against a simulated state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ReplayResult {
    /// The access tuple that was replayed. This field includes information about the principal, resource, and permission that were involved in the access attempt.
    #[serde(rename="accessTuple")]
    
    pub access_tuple: Option<GoogleCloudPolicysimulatorV1AccessTuple>,
    /// The difference between the principal's access under the current (baseline) policies and the principal's access under the proposed (simulated) policies. This field is only included for access tuples that were successfully replayed and had different results under the current policies and the proposed policies.
    
    pub diff: Option<GoogleCloudPolicysimulatorV1ReplayDiff>,
    /// The error that caused the access tuple replay to fail. This field is only included for access tuples that were not replayed successfully.
    
    pub error: Option<GoogleRpcStatus>,
    /// The latest date this access tuple was seen in the logs.
    #[serde(rename="lastSeenDate")]
    
    pub last_seen_date: Option<GoogleTypeDate>,
    /// The resource name of the `ReplayResult`, in the following format: `{projects|folders|organizations}/{resource-id}/locations/global/replays/{replay-id}/results/{replay-result-id}`, where `{resource-id}` is the ID of the project, folder, or organization that owns the Replay. Example: `projects/my-example-project/locations/global/replays/506a5f7f-38ce-4d7d-8e03-479ce1833c36/results/1234`
    
    pub name: Option<String>,
    /// The Replay that the access tuple was included in.
    
    pub parent: Option<String>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ReplayResult {}


/// Summary statistics about the replayed log entries.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ReplayResultsSummary {
    /// The number of replayed log entries with a difference between baseline and simulated policies.
    #[serde(rename="differenceCount")]
    
    pub difference_count: Option<i32>,
    /// The number of log entries that could not be replayed.
    #[serde(rename="errorCount")]
    
    pub error_count: Option<i32>,
    /// The total number of log entries replayed.
    #[serde(rename="logCount")]
    
    pub log_count: Option<i32>,
    /// The date of the newest log entry replayed.
    #[serde(rename="newestDate")]
    
    pub newest_date: Option<GoogleTypeDate>,
    /// The date of the oldest log entry replayed.
    #[serde(rename="oldestDate")]
    
    pub oldest_date: Option<GoogleTypeDate>,
    /// The number of replayed log entries with no difference between baseline and simulated policies.
    #[serde(rename="unchangedCount")]
    
    pub unchanged_count: Option<i32>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ReplayResultsSummary {}


/// ResourceContext provides the context we know about a resource. It is similar in concept to google.cloud.asset.v1.Resource, but focuses on the information specifically used by Simulator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPolicysimulatorV1ResourceContext {
    /// The ancestry path of the resource in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the resource is a project, folder, or organization, the ancestry path starts from the resource itself. Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    
    pub ancestors: Option<Vec<String>>,
    /// The asset type of the resource as defined by CAIS. Example: `compute.googleapis.com/Firewall` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information.
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// The full name of the resource. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    
    pub resource: Option<String>,
}

impl client::Part for GoogleCloudPolicysimulatorV1ResourceContext {}


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
    
    pub condition: Option<GoogleTypeExpr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workforce identity pool. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}`: All workforce identities in a group. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All workforce identities with a specific attribute value. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`: All identities in a workforce identity pool. * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workload identity pool. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}`: A workload identity pool group. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All identities in a workload identity pool with a certain attribute. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*`: All identities in a workload identity pool. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: Deleted single identity in a workforce identity pool. For example, `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value`.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-overview). For a list of the available pre-defined roles, see [here](https://cloud.google.com/iam/docs/understanding-roles).
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1Binding {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** ``` { "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 } ``` **YAML example:** ``` bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3 ``` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl client::Part for GoogleIamV1Policy {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations replays operations list folders](FolderLocationReplayOperationListCall) (response)
/// * [list operations](OperationListCall) (response)
/// * [locations replays operations list organizations](OrganizationLocationReplayOperationListCall) (response)
/// * [locations replays operations list projects](ProjectLocationReplayOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations org policy violations previews operations get folders](FolderLocationOrgPolicyViolationsPreviewOperationGetCall) (response)
/// * [locations replays operations get folders](FolderLocationReplayOperationGetCall) (response)
/// * [locations replays create folders](FolderLocationReplayCreateCall) (response)
/// * [get operations](OperationGetCall) (response)
/// * [locations org policy violations previews operations get organizations](OrganizationLocationOrgPolicyViolationsPreviewOperationGetCall) (response)
/// * [locations org policy violations previews create organizations](OrganizationLocationOrgPolicyViolationsPreviewCreateCall) (response)
/// * [locations replays operations get organizations](OrganizationLocationReplayOperationGetCall) (response)
/// * [locations replays create organizations](OrganizationLocationReplayCreateCall) (response)
/// * [locations org policy violations previews operations get projects](ProjectLocationOrgPolicyViolationsPreviewOperationGetCall) (response)
/// * [locations replays operations get projects](ProjectLocationReplayOperationGetCall) (response)
/// * [locations replays create projects](ProjectLocationReplayCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeExpr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for GoogleTypeExpr {}


