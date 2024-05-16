use super::*;
/// The request object used by `AbandonRelease`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases abandon projects](ProjectLocationDeliveryPipelineReleaseAbandonCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AbandonReleaseRequest { _never_set: Option<bool> }

impl client::RequestValue for AbandonReleaseRequest {}


/// The response object for `AbandonRelease`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases abandon projects](ProjectLocationDeliveryPipelineReleaseAbandonCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AbandonReleaseResponse { _never_set: Option<bool> }

impl client::ResponseResult for AbandonReleaseResponse {}


/// An advanceChildRollout Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvanceChildRolloutJob { _never_set: Option<bool> }

impl client::Part for AdvanceChildRolloutJob {}


/// AdvanceChildRolloutJobRun contains information specific to a advanceChildRollout `JobRun`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvanceChildRolloutJobRun {
    /// Output only. Name of the `ChildRollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/a-z{0,62}`.
    
    pub rollout: Option<String>,
    /// Output only. the ID of the ChildRollout's Phase.
    #[serde(rename="rolloutPhaseId")]
    
    pub rollout_phase_id: Option<String>,
}

impl client::Part for AdvanceChildRolloutJobRun {}


/// Contains the information of an automated advance-rollout operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvanceRolloutOperation {
    /// Output only. The phase the rollout will be advanced to.
    #[serde(rename="destinationPhase")]
    
    pub destination_phase: Option<String>,
    /// Output only. The name of the rollout that initiates the `AutomationRun`.
    
    pub rollout: Option<String>,
    /// Output only. The phase of a deployment that initiated the operation.
    #[serde(rename="sourcePhase")]
    
    pub source_phase: Option<String>,
    /// Output only. How long the operation will be paused.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub wait: Option<client::chrono::Duration>,
}

impl client::Part for AdvanceRolloutOperation {}


/// The request object used by `AdvanceRollout`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts advance projects](ProjectLocationDeliveryPipelineReleaseRolloutAdvanceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvanceRolloutRequest {
    /// Required. The phase ID to advance the `Rollout` to.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
}

impl client::RequestValue for AdvanceRolloutRequest {}


/// The response object from `AdvanceRollout`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts advance projects](ProjectLocationDeliveryPipelineReleaseRolloutAdvanceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvanceRolloutResponse { _never_set: Option<bool> }

impl client::ResponseResult for AdvanceRolloutResponse {}


/// The `AdvanceRollout` automation rule will automatically advance a successful Rollout to the next phase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdvanceRolloutRule {
    /// Output only. Information around the state of the Automation rule.
    
    pub condition: Option<AutomationRuleCondition>,
    /// Required. ID of the rule. This id must be unique in the `Automation` resource to which this rule belongs. The format is `a-z{0,62}`.
    
    pub id: Option<String>,
    /// Optional. Proceeds only after phase name matched any one in the list. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`.
    #[serde(rename="sourcePhases")]
    
    pub source_phases: Option<Vec<String>>,
    /// Optional. How long to wait after a rollout is finished.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub wait: Option<client::chrono::Duration>,
}

impl client::Part for AdvanceRolloutRule {}


/// Information specifying an Anthos Cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnthosCluster {
    /// Membership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`.
    
    pub membership: Option<String>,
}

impl client::Part for AnthosCluster {}


/// The request object used by `ApproveRollout`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts approve projects](ProjectLocationDeliveryPipelineReleaseRolloutApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApproveRolloutRequest {
    /// Required. True = approve; false = reject
    
    pub approved: Option<bool>,
}

impl client::RequestValue for ApproveRolloutRequest {}


/// The response object from `ApproveRollout`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts approve projects](ProjectLocationDeliveryPipelineReleaseRolloutApproveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApproveRolloutResponse { _never_set: Option<bool> }

impl client::ResponseResult for ApproveRolloutResponse {}


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


/// An `Automation` resource in the Cloud Deploy API. An `Automation` enables the automation of manually driven actions for a Delivery Pipeline, which includes Release promotion among Targets, Rollout repair and Rollout deployment strategy advancement. The intention of Automation is to reduce manual intervention in the continuous delivery process.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines automations create projects](ProjectLocationDeliveryPipelineAutomationCreateCall) (request)
/// * [locations delivery pipelines automations get projects](ProjectLocationDeliveryPipelineAutomationGetCall) (response)
/// * [locations delivery pipelines automations patch projects](ProjectLocationDeliveryPipelineAutomationPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Automation {
    /// Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (`/`). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character (`[a-z0-9A-Z]`) with dashes (`-`), underscores (`_`), dots (`.`), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots(`.`), not longer than 253 characters in total, followed by a slash (`/`). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. Time at which the automation was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the `Automation`. Max length is 255 characters.
    
    pub description: Option<String>,
    /// Optional. The weak etag of the `Automation` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Name of the `Automation`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automations/{automation}`.
    
    pub name: Option<String>,
    /// Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution.
    
    pub rules: Option<Vec<AutomationRule>>,
    /// Required. Selected resources to which the automation will be applied.
    
    pub selector: Option<AutomationResourceSelector>,
    /// Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Optional. When Suspended, automation is deactivated from execution.
    
    pub suspended: Option<bool>,
    /// Output only. Unique identifier of the `Automation`.
    
    pub uid: Option<String>,
    /// Output only. Time at which the automation was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Automation {}
impl client::ResponseResult for Automation {}


/// AutomationResourceSelector contains the information to select the resources to which an Automation is going to be applied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutomationResourceSelector {
    /// Contains attributes about a target.
    
    pub targets: Option<Vec<TargetAttribute>>,
}

impl client::Part for AutomationResourceSelector {}


/// AutomationRolloutMetadata contains Automation-related actions that were performed on a rollout.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutomationRolloutMetadata {
    /// Output only. The IDs of the AutomationRuns initiated by an advance rollout rule.
    #[serde(rename="advanceAutomationRuns")]
    
    pub advance_automation_runs: Option<Vec<String>>,
    /// Output only. The current AutomationRun repairing the rollout.
    #[serde(rename="currentRepairAutomationRun")]
    
    pub current_repair_automation_run: Option<String>,
    /// Output only. The ID of the AutomationRun initiated by a promote release rule.
    #[serde(rename="promoteAutomationRun")]
    
    pub promote_automation_run: Option<String>,
    /// Output only. The IDs of the AutomationRuns initiated by a repair rollout rule.
    #[serde(rename="repairAutomationRuns")]
    
    pub repair_automation_runs: Option<Vec<String>>,
}

impl client::Part for AutomationRolloutMetadata {}


/// `AutomationRule` defines the automation activities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutomationRule {
    /// Optional. The `AdvanceRolloutRule` will automatically advance a successful Rollout.
    #[serde(rename="advanceRolloutRule")]
    
    pub advance_rollout_rule: Option<AdvanceRolloutRule>,
    /// Optional. `PromoteReleaseRule` will automatically promote a release from the current target to a specified target.
    #[serde(rename="promoteReleaseRule")]
    
    pub promote_release_rule: Option<PromoteReleaseRule>,
    /// Optional. The `RepairRolloutRule` will automatically repair a failed rollout.
    #[serde(rename="repairRolloutRule")]
    
    pub repair_rollout_rule: Option<RepairRolloutRule>,
}

impl client::Part for AutomationRule {}


/// `AutomationRuleCondition` contains conditions relevant to an `Automation` rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutomationRuleCondition {
    /// Optional. Details around targets enumerated in the rule.
    #[serde(rename="targetsPresentCondition")]
    
    pub targets_present_condition: Option<TargetsPresentCondition>,
}

impl client::Part for AutomationRuleCondition {}


/// An `AutomationRun` resource in the Cloud Deploy API. An `AutomationRun` represents an execution instance of an automation rule.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines automation runs get projects](ProjectLocationDeliveryPipelineAutomationRunGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutomationRun {
    /// Output only. Advances a rollout to the next phase.
    #[serde(rename="advanceRolloutOperation")]
    
    pub advance_rollout_operation: Option<AdvanceRolloutOperation>,
    /// Output only. The ID of the automation that initiated the operation.
    #[serde(rename="automationId")]
    
    pub automation_id: Option<String>,
    /// Output only. Snapshot of the Automation taken at AutomationRun creation time.
    #[serde(rename="automationSnapshot")]
    
    pub automation_snapshot: Option<Automation>,
    /// Output only. Time at which the `AutomationRun` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The weak etag of the `AutomationRun` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. Time the `AutomationRun` expires. An `AutomationRun` expires after 14 days from its creation date.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Name of the `AutomationRun`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{delivery_pipeline}/automationRuns/{automation_run}`.
    
    pub name: Option<String>,
    /// Output only. Promotes a release to a specified 'Target'.
    #[serde(rename="promoteReleaseOperation")]
    
    pub promote_release_operation: Option<PromoteReleaseOperation>,
    /// Output only. Repairs a failed 'Rollout'.
    #[serde(rename="repairRolloutOperation")]
    
    pub repair_rollout_operation: Option<RepairRolloutOperation>,
    /// Output only. The ID of the automation rule that initiated the operation.
    #[serde(rename="ruleId")]
    
    pub rule_id: Option<String>,
    /// Output only. Email address of the user-managed IAM service account that performs the operations against Cloud Deploy resources.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Output only. Current state of the `AutomationRun`.
    
    pub state: Option<AutomationRunStateEnum>,
    /// Output only. Explains the current state of the `AutomationRun`. Present only when an explanation is needed.
    #[serde(rename="stateDescription")]
    
    pub state_description: Option<String>,
    /// Output only. The ID of the target that represents the promotion stage that initiates the `AutomationRun`. The value of this field is the last segment of a target name.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// Output only. Time at which the automationRun was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Earliest time the `AutomationRun` will attempt to resume. Wait-time is configured by `wait` in automation rule.
    #[serde(rename="waitUntilTime")]
    
    pub wait_until_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for AutomationRun {}


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


/// Description of an a image to use during Skaffold rendering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildArtifact {
    /// Image name in Skaffold configuration.
    
    pub image: Option<String>,
    /// Image tag to use. This will generally be the full path to an image, such as "gcr.io/my-project/busybox:1.2.3" or "gcr.io/my-project/busybox@sha256:abc123".
    
    pub tag: Option<String>,
}

impl client::Part for BuildArtifact {}


/// Canary represents the canary deployment strategy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Canary {
    /// Configures the progressive based deployment for a Target.
    #[serde(rename="canaryDeployment")]
    
    pub canary_deployment: Option<CanaryDeployment>,
    /// Configures the progressive based deployment for a Target, but allows customizing at the phase level where a phase represents each of the percentage deployments.
    #[serde(rename="customCanaryDeployment")]
    
    pub custom_canary_deployment: Option<CustomCanaryDeployment>,
    /// Optional. Runtime specific configurations for the deployment strategy. The runtime configuration is used to determine how Cloud Deploy will split traffic to enable a progressive deployment.
    #[serde(rename="runtimeConfig")]
    
    pub runtime_config: Option<RuntimeConfig>,
}

impl client::Part for Canary {}


/// CanaryDeployment represents the canary deployment configuration
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CanaryDeployment {
    /// Required. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100.
    
    pub percentages: Option<Vec<i32>>,
    /// Optional. Configuration for the postdeploy job of the last phase. If this is not configured, there will be no postdeploy job for this phase.
    
    pub postdeploy: Option<Postdeploy>,
    /// Optional. Configuration for the predeploy job of the first phase. If this is not configured, there will be no predeploy job for this phase.
    
    pub predeploy: Option<Predeploy>,
    /// Whether to run verify tests after each percentage deployment.
    
    pub verify: Option<bool>,
}

impl client::Part for CanaryDeployment {}


/// The request object used by `CancelAutomationRun`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines automation runs cancel projects](ProjectLocationDeliveryPipelineAutomationRunCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelAutomationRunRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelAutomationRunRequest {}


/// The response object from `CancelAutomationRun`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines automation runs cancel projects](ProjectLocationDeliveryPipelineAutomationRunCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelAutomationRunResponse { _never_set: Option<bool> }

impl client::ResponseResult for CancelAutomationRunResponse {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// The request object used by `CancelRollout`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts cancel projects](ProjectLocationDeliveryPipelineReleaseRolloutCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelRolloutRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelRolloutRequest {}


/// The response object from `CancelRollout`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts cancel projects](ProjectLocationDeliveryPipelineReleaseRolloutCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelRolloutResponse { _never_set: Option<bool> }

impl client::ResponseResult for CancelRolloutResponse {}


/// ChildRollouts job composition
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChildRolloutJobs {
    /// Output only. List of AdvanceChildRolloutJobs
    #[serde(rename="advanceRolloutJobs")]
    
    pub advance_rollout_jobs: Option<Vec<Job>>,
    /// Output only. List of CreateChildRolloutJobs
    #[serde(rename="createRolloutJobs")]
    
    pub create_rollout_jobs: Option<Vec<Job>>,
}

impl client::Part for ChildRolloutJobs {}


/// CloudRunConfig contains the Cloud Run runtime configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunConfig {
    /// Whether Cloud Deploy should update the traffic stanza in a Cloud Run Service on the user's behalf to facilitate traffic splitting. This is required to be true for CanaryDeployments, but optional for CustomCanaryDeployments.
    #[serde(rename="automaticTrafficControl")]
    
    pub automatic_traffic_control: Option<bool>,
    /// Optional. A list of tags that are added to the canary revision while the canary phase is in progress.
    #[serde(rename="canaryRevisionTags")]
    
    pub canary_revision_tags: Option<Vec<String>>,
    /// Optional. A list of tags that are added to the prior revision while the canary phase is in progress.
    #[serde(rename="priorRevisionTags")]
    
    pub prior_revision_tags: Option<Vec<String>>,
    /// Optional. A list of tags that are added to the final stable revision when the stable phase is applied.
    #[serde(rename="stableRevisionTags")]
    
    pub stable_revision_tags: Option<Vec<String>>,
}

impl client::Part for CloudRunConfig {}


/// Information specifying where to deploy a Cloud Run Service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunLocation {
    /// Required. The location for the Cloud Run Service. Format must be `projects/{project}/locations/{location}`.
    
    pub location: Option<String>,
}

impl client::Part for CloudRunLocation {}


/// CloudRunMetadata contains information from a Cloud Run deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunMetadata {
    /// Output only. The name of the Cloud Run job that is associated with a `Rollout`. Format is `projects/{project}/locations/{location}/jobs/{job_name}`.
    
    pub job: Option<String>,
    /// Output only. The Cloud Run Revision id associated with a `Rollout`.
    
    pub revision: Option<String>,
    /// Output only. The name of the Cloud Run Service that is associated with a `Rollout`. Format is `projects/{project}/locations/{location}/services/{service}`.
    
    pub service: Option<String>,
    /// Output only. The Cloud Run Service urls that are associated with a `Rollout`.
    #[serde(rename="serviceUrls")]
    
    pub service_urls: Option<Vec<String>>,
}

impl client::Part for CloudRunMetadata {}


/// CloudRunRenderMetadata contains Cloud Run information associated with a `Release` render.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRunRenderMetadata {
    /// Output only. The name of the Cloud Run Service in the rendered manifest. Format is `projects/{project}/locations/{location}/services/{service}`.
    
    pub service: Option<String>,
}

impl client::Part for CloudRunRenderMetadata {}


/// Service-wide configuration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get config projects](ProjectLocationGetConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Default Skaffold version that is assigned when a Release is created without specifying a Skaffold version.
    #[serde(rename="defaultSkaffoldVersion")]
    
    pub default_skaffold_version: Option<String>,
    /// Name of the configuration.
    
    pub name: Option<String>,
    /// All supported versions of Skaffold.
    #[serde(rename="supportedVersions")]
    
    pub supported_versions: Option<Vec<SkaffoldVersion>>,
}

impl client::ResponseResult for Config {}


/// A createChildRollout Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateChildRolloutJob { _never_set: Option<bool> }

impl client::Part for CreateChildRolloutJob {}


/// CreateChildRolloutJobRun contains information specific to a createChildRollout `JobRun`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateChildRolloutJobRun {
    /// Output only. Name of the `ChildRollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/a-z{0,62}`.
    
    pub rollout: Option<String>,
    /// Output only. The ID of the childRollout Phase initiated by this JobRun.
    #[serde(rename="rolloutPhaseId")]
    
    pub rollout_phase_id: Option<String>,
}

impl client::Part for CreateChildRolloutJobRun {}


/// CustomCanaryDeployment represents the custom canary deployment configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomCanaryDeployment {
    /// Required. Configuration for each phase in the canary deployment in the order executed.
    #[serde(rename="phaseConfigs")]
    
    pub phase_configs: Option<Vec<PhaseConfig>>,
}

impl client::Part for CustomCanaryDeployment {}


/// CustomMetadata contains information from a user-defined operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomMetadata {
    /// Output only. Key-value pairs provided by the user-defined operation.
    
    pub values: Option<HashMap<String, String>>,
}

impl client::Part for CustomMetadata {}


/// Information specifying a Custom Target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomTarget {
    /// Required. The name of the CustomTargetType. Format must be `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`.
    #[serde(rename="customTargetType")]
    
    pub custom_target_type: Option<String>,
}

impl client::Part for CustomTarget {}


/// CustomTargetDeployMetadata contains information from a Custom Target deploy operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomTargetDeployMetadata {
    /// Output only. Skip message provided in the results of a custom deploy operation.
    #[serde(rename="skipMessage")]
    
    pub skip_message: Option<String>,
}

impl client::Part for CustomTargetDeployMetadata {}


/// CustomTargetSkaffoldActions represents the `CustomTargetType` configuration using Skaffold custom actions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomTargetSkaffoldActions {
    /// Required. The Skaffold custom action responsible for deploy operations.
    #[serde(rename="deployAction")]
    
    pub deploy_action: Option<String>,
    /// Optional. List of Skaffold modules Cloud Deploy will include in the Skaffold Config as required before performing diagnose.
    #[serde(rename="includeSkaffoldModules")]
    
    pub include_skaffold_modules: Option<Vec<SkaffoldModules>>,
    /// Optional. The Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via `skaffold render`.
    #[serde(rename="renderAction")]
    
    pub render_action: Option<String>,
}

impl client::Part for CustomTargetSkaffoldActions {}


/// A `CustomTargetType` resource in the Cloud Deploy API. A `CustomTargetType` defines a type of custom target that can be referenced in a `Target` in order to facilitate deploying to other systems besides the supported runtimes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom target types create projects](ProjectLocationCustomTargetTypeCreateCall) (request)
/// * [locations custom target types get projects](ProjectLocationCustomTargetTypeGetCall) (response)
/// * [locations custom target types patch projects](ProjectLocationCustomTargetTypePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomTargetType {
    /// Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. Time at which the `CustomTargetType` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Configures render and deploy for the `CustomTargetType` using Skaffold custom actions.
    #[serde(rename="customActions")]
    
    pub custom_actions: Option<CustomTargetSkaffoldActions>,
    /// Output only. Resource id of the `CustomTargetType`.
    #[serde(rename="customTargetTypeId")]
    
    pub custom_target_type_id: Option<String>,
    /// Optional. Description of the `CustomTargetType`. Max length is 255 characters.
    
    pub description: Option<String>,
    /// Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Name of the `CustomTargetType`. Format is `projects/{project}/locations/{location}/customTargetTypes/a-z{0,62}`.
    
    pub name: Option<String>,
    /// Output only. Unique identifier of the `CustomTargetType`.
    
    pub uid: Option<String>,
    /// Output only. Most recent time at which the `CustomTargetType` was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for CustomTargetType {}
impl client::ResponseResult for CustomTargetType {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for Date {}


/// Execution using the default Cloud Build pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DefaultPool {
    /// Optional. Cloud Storage location where execution outputs should be stored. This can either be a bucket ("gs://my-bucket") or a path within a bucket ("gs://my-bucket/my-dir"). If unspecified, a default bucket located in the same region will be used.
    #[serde(rename="artifactStorage")]
    
    pub artifact_storage: Option<String>,
    /// Optional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
}

impl client::Part for DefaultPool {}


/// A `DeliveryPipeline` resource in the Cloud Deploy API. A `DeliveryPipeline` defines a pipeline through which a Skaffold configuration can progress.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines create projects](ProjectLocationDeliveryPipelineCreateCall) (request)
/// * [locations delivery pipelines get projects](ProjectLocationDeliveryPipelineGetCall) (response)
/// * [locations delivery pipelines patch projects](ProjectLocationDeliveryPipelinePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryPipeline {
    /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. Information around the state of the Delivery Pipeline.
    
    pub condition: Option<PipelineCondition>,
    /// Output only. Time at which the pipeline was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Description of the `DeliveryPipeline`. Max length is 255 characters.
    
    pub description: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Name of the `DeliveryPipeline`. Format is `projects/{project}/locations/{location}/deliveryPipelines/a-z{0,62}`.
    
    pub name: Option<String>,
    /// SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`.
    #[serde(rename="serialPipeline")]
    
    pub serial_pipeline: Option<SerialPipeline>,
    /// When suspended, no new releases or rollouts can be created, but in-progress ones will complete.
    
    pub suspended: Option<bool>,
    /// Output only. Unique identifier of the `DeliveryPipeline`.
    
    pub uid: Option<String>,
    /// Output only. Most recent time at which the pipeline was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for DeliveryPipeline {}
impl client::ResponseResult for DeliveryPipeline {}


/// The artifacts produced by a deploy operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployArtifact {
    /// Output only. URI of a directory containing the artifacts. All paths are relative to this location.
    #[serde(rename="artifactUri")]
    
    pub artifact_uri: Option<String>,
    /// Output only. File paths of the manifests applied during the deploy operation relative to the URI.
    #[serde(rename="manifestPaths")]
    
    pub manifest_paths: Option<Vec<String>>,
}

impl client::Part for DeployArtifact {}


/// A deploy Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployJob { _never_set: Option<bool> }

impl client::Part for DeployJob {}


/// DeployJobRun contains information specific to a deploy `JobRun`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployJobRun {
    /// Output only. The artifact of a deploy job run, if available.
    
    pub artifact: Option<DeployArtifact>,
    /// Output only. The resource name of the Cloud Build `Build` object that is used to deploy. Format is `projects/{project}/locations/{location}/builds/{build}`.
    
    pub build: Option<String>,
    /// Output only. The reason the deploy failed. This will always be unspecified while the deploy is in progress or if it succeeded.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<DeployJobRunFailureCauseEnum>,
    /// Output only. Additional information about the deploy failure, if available.
    #[serde(rename="failureMessage")]
    
    pub failure_message: Option<String>,
    /// Output only. Metadata containing information about the deploy job run.
    
    pub metadata: Option<DeployJobRunMetadata>,
}

impl client::Part for DeployJobRun {}


/// DeployJobRunMetadata surfaces information associated with a `DeployJobRun` to the user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployJobRunMetadata {
    /// Output only. The name of the Cloud Run Service that is associated with a `DeployJobRun`.
    #[serde(rename="cloudRun")]
    
    pub cloud_run: Option<CloudRunMetadata>,
    /// Output only. Custom metadata provided by user-defined deploy operation.
    
    pub custom: Option<CustomMetadata>,
    /// Output only. Custom Target metadata associated with a `DeployJobRun`.
    #[serde(rename="customTarget")]
    
    pub custom_target: Option<CustomTargetDeployMetadata>,
}

impl client::Part for DeployJobRunMetadata {}


/// DeployParameters contains deploy parameters information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeployParameters {
    /// Optional. Deploy parameters are applied to targets with match labels. If unspecified, deploy parameters are applied to all targets (including child targets of a multi-target).
    #[serde(rename="matchTargetLabels")]
    
    pub match_target_labels: Option<HashMap<String, String>>,
    /// Required. Values are deploy parameters in key-value pairs.
    
    pub values: Option<HashMap<String, String>>,
}

impl client::Part for DeployParameters {}


/// Deployment job composition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentJobs {
    /// Output only. The deploy Job. This is the deploy job in the phase.
    #[serde(rename="deployJob")]
    
    pub deploy_job: Option<Job>,
    /// Output only. The postdeploy Job, which is the last job on the phase.
    #[serde(rename="postdeployJob")]
    
    pub postdeploy_job: Option<Job>,
    /// Output only. The predeploy Job, which is the first job on the phase.
    #[serde(rename="predeployJob")]
    
    pub predeploy_job: Option<Job>,
    /// Output only. The verify Job. Runs after a deploy if the deploy succeeds.
    #[serde(rename="verifyJob")]
    
    pub verify_job: Option<Job>,
}

impl client::Part for DeploymentJobs {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Configuration of the environment to use when calling Skaffold.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// Optional. Cloud Storage location in which to store execution outputs. This can either be a bucket ("gs://my-bucket") or a path within a bucket ("gs://my-bucket/my-dir"). If unspecified, a default bucket located in the same region will be used.
    #[serde(rename="artifactStorage")]
    
    pub artifact_storage: Option<String>,
    /// Optional. Use default Cloud Build pool.
    #[serde(rename="defaultPool")]
    
    pub default_pool: Option<DefaultPool>,
    /// Optional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used.
    #[serde(rename="executionTimeout")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub execution_timeout: Option<client::chrono::Duration>,
    /// Optional. Use private Cloud Build pool.
    #[serde(rename="privatePool")]
    
    pub private_pool: Option<PrivatePool>,
    /// Optional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Required. Usages when this configuration should be applied.
    
    pub usages: Option<Vec<ExecutionConfigUsagesEnum>>,
    /// Optional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used.
    #[serde(rename="workerPool")]
    
    pub worker_pool: Option<String>,
}

impl client::Part for ExecutionConfig {}


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


/// Information about the Kubernetes Gateway API service mesh configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GatewayServiceMesh {
    /// Required. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service.
    
    pub deployment: Option<String>,
    /// Required. Name of the Gateway API HTTPRoute.
    #[serde(rename="httpRoute")]
    
    pub http_route: Option<String>,
    /// Optional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time.
    #[serde(rename="routeUpdateWaitTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub route_update_wait_time: Option<client::chrono::Duration>,
    /// Required. Name of the Kubernetes Service.
    
    pub service: Option<String>,
    /// Optional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time.
    #[serde(rename="stableCutbackDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub stable_cutback_duration: Option<client::chrono::Duration>,
}

impl client::Part for GatewayServiceMesh {}


/// Information specifying a GKE Cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeCluster {
    /// Information specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`.
    
    pub cluster: Option<String>,
    /// Optional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept).
    #[serde(rename="internalIp")]
    
    pub internal_ip: Option<bool>,
}

impl client::Part for GkeCluster {}


/// The request object used by `IgnoreJob`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts ignore job projects](ProjectLocationDeliveryPipelineReleaseRolloutIgnoreJobCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IgnoreJobRequest {
    /// Required. The job ID for the Job to ignore.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Required. The phase ID the Job to ignore belongs to.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
}

impl client::RequestValue for IgnoreJobRequest {}


/// The response object from `IgnoreJob`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts ignore job projects](ProjectLocationDeliveryPipelineReleaseRolloutIgnoreJobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IgnoreJobResponse { _never_set: Option<bool> }

impl client::ResponseResult for IgnoreJobResponse {}


/// Job represents an operation for a `Rollout`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Output only. An advanceChildRollout Job.
    #[serde(rename="advanceChildRolloutJob")]
    
    pub advance_child_rollout_job: Option<AdvanceChildRolloutJob>,
    /// Output only. A createChildRollout Job.
    #[serde(rename="createChildRolloutJob")]
    
    pub create_child_rollout_job: Option<CreateChildRolloutJob>,
    /// Output only. A deploy Job.
    #[serde(rename="deployJob")]
    
    pub deploy_job: Option<DeployJob>,
    /// Output only. The ID of the Job.
    
    pub id: Option<String>,
    /// Output only. The name of the `JobRun` responsible for the most recent invocation of this Job.
    #[serde(rename="jobRun")]
    
    pub job_run: Option<String>,
    /// Output only. A postdeploy Job.
    #[serde(rename="postdeployJob")]
    
    pub postdeploy_job: Option<PostdeployJob>,
    /// Output only. A predeploy Job.
    #[serde(rename="predeployJob")]
    
    pub predeploy_job: Option<PredeployJob>,
    /// Output only. Additional information on why the Job was skipped, if available.
    #[serde(rename="skipMessage")]
    
    pub skip_message: Option<String>,
    /// Output only. The current state of the Job.
    
    pub state: Option<JobStateEnum>,
    /// Output only. A verify Job.
    #[serde(rename="verifyJob")]
    
    pub verify_job: Option<VerifyJob>,
}

impl client::Part for Job {}


/// A `JobRun` resource in the Cloud Deploy API. A `JobRun` contains information of a single `Rollout` job evaluation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts job runs get projects](ProjectLocationDeliveryPipelineReleaseRolloutJobRunGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobRun {
    /// Output only. Information specific to an advanceChildRollout `JobRun`
    #[serde(rename="advanceChildRolloutJobRun")]
    
    pub advance_child_rollout_job_run: Option<AdvanceChildRolloutJobRun>,
    /// Output only. Information specific to a createChildRollout `JobRun`.
    #[serde(rename="createChildRolloutJobRun")]
    
    pub create_child_rollout_job_run: Option<CreateChildRolloutJobRun>,
    /// Output only. Time at which the `JobRun` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Information specific to a deploy `JobRun`.
    #[serde(rename="deployJobRun")]
    
    pub deploy_job_run: Option<DeployJobRun>,
    /// Output only. Time at which the `JobRun` ended.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. ID of the `Rollout` job this `JobRun` corresponds to.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Optional. Name of the `JobRun`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{releases}/rollouts/{rollouts}/jobRuns/{uuid}`.
    
    pub name: Option<String>,
    /// Output only. ID of the `Rollout` phase this `JobRun` belongs in.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
    /// Output only. Information specific to a postdeploy `JobRun`.
    #[serde(rename="postdeployJobRun")]
    
    pub postdeploy_job_run: Option<PostdeployJobRun>,
    /// Output only. Information specific to a predeploy `JobRun`.
    #[serde(rename="predeployJobRun")]
    
    pub predeploy_job_run: Option<PredeployJobRun>,
    /// Output only. Time at which the `JobRun` was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the `JobRun`.
    
    pub state: Option<JobRunStateEnum>,
    /// Output only. Unique identifier of the `JobRun`.
    
    pub uid: Option<String>,
    /// Output only. Information specific to a verify `JobRun`.
    #[serde(rename="verifyJobRun")]
    
    pub verify_job_run: Option<VerifyJobRun>,
}

impl client::ResponseResult for JobRun {}


/// KubernetesConfig contains the Kubernetes runtime configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KubernetesConfig {
    /// Kubernetes Gateway API service mesh configuration.
    #[serde(rename="gatewayServiceMesh")]
    
    pub gateway_service_mesh: Option<GatewayServiceMesh>,
    /// Kubernetes Service networking configuration.
    #[serde(rename="serviceNetworking")]
    
    pub service_networking: Option<ServiceNetworking>,
}

impl client::Part for KubernetesConfig {}


/// The response object from `ListAutomationRuns`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines automation runs list projects](ProjectLocationDeliveryPipelineAutomationRunListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAutomationRunsResponse {
    /// The `AutomationRuns` objects.
    #[serde(rename="automationRuns")]
    
    pub automation_runs: Option<Vec<AutomationRun>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListAutomationRunsResponse {}


/// The response object from `ListAutomations`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines automations list projects](ProjectLocationDeliveryPipelineAutomationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAutomationsResponse {
    /// The `Automation` objects.
    
    pub automations: Option<Vec<Automation>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListAutomationsResponse {}


/// The response object from `ListCustomTargetTypes.`
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom target types list projects](ProjectLocationCustomTargetTypeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCustomTargetTypesResponse {
    /// The `CustomTargetType` objects.
    #[serde(rename="customTargetTypes")]
    
    pub custom_target_types: Option<Vec<CustomTargetType>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListCustomTargetTypesResponse {}


/// The response object from `ListDeliveryPipelines`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines list projects](ProjectLocationDeliveryPipelineListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDeliveryPipelinesResponse {
    /// The `DeliveryPipeline` objects.
    #[serde(rename="deliveryPipelines")]
    
    pub delivery_pipelines: Option<Vec<DeliveryPipeline>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListDeliveryPipelinesResponse {}


/// ListJobRunsResponse is the response object returned by `ListJobRuns`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts job runs list projects](ProjectLocationDeliveryPipelineReleaseRolloutJobRunListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobRunsResponse {
    /// The `JobRun` objects.
    #[serde(rename="jobRuns")]
    
    pub job_runs: Option<Vec<JobRun>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Locations that could not be reached
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListJobRunsResponse {}


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


/// The response object from `ListReleases`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases list projects](ProjectLocationDeliveryPipelineReleaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReleasesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The `Release` objects.
    
    pub releases: Option<Vec<Release>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListReleasesResponse {}


/// ListRolloutsResponse is the response object reutrned by `ListRollouts`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts list projects](ProjectLocationDeliveryPipelineReleaseRolloutListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRolloutsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The `Rollout` objects.
    
    pub rollouts: Option<Vec<Rollout>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListRolloutsResponse {}


/// The response object from `ListTargets`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations targets list projects](ProjectLocationTargetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTargetsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The `Target` objects.
    
    pub targets: Option<Vec<Target>>,
    /// Locations that could not be reached.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListTargetsResponse {}


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


/// Metadata includes information associated with a `Rollout`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Output only. AutomationRolloutMetadata contains the information about the interactions between Automation service and this rollout.
    
    pub automation: Option<AutomationRolloutMetadata>,
    /// Output only. The name of the Cloud Run Service that is associated with a `Rollout`.
    #[serde(rename="cloudRun")]
    
    pub cloud_run: Option<CloudRunMetadata>,
    /// Output only. Custom metadata provided by user-defined `Rollout` operations.
    
    pub custom: Option<CustomMetadata>,
}

impl client::Part for Metadata {}


/// Information specifying a multiTarget.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiTarget {
    /// Required. The target_ids of this multiTarget.
    #[serde(rename="targetIds")]
    
    pub target_ids: Option<Vec<String>>,
}

impl client::Part for MultiTarget {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom target types create projects](ProjectLocationCustomTargetTypeCreateCall) (response)
/// * [locations custom target types delete projects](ProjectLocationCustomTargetTypeDeleteCall) (response)
/// * [locations custom target types patch projects](ProjectLocationCustomTargetTypePatchCall) (response)
/// * [locations delivery pipelines automations create projects](ProjectLocationDeliveryPipelineAutomationCreateCall) (response)
/// * [locations delivery pipelines automations delete projects](ProjectLocationDeliveryPipelineAutomationDeleteCall) (response)
/// * [locations delivery pipelines automations patch projects](ProjectLocationDeliveryPipelineAutomationPatchCall) (response)
/// * [locations delivery pipelines releases rollouts create projects](ProjectLocationDeliveryPipelineReleaseRolloutCreateCall) (response)
/// * [locations delivery pipelines releases create projects](ProjectLocationDeliveryPipelineReleaseCreateCall) (response)
/// * [locations delivery pipelines create projects](ProjectLocationDeliveryPipelineCreateCall) (response)
/// * [locations delivery pipelines delete projects](ProjectLocationDeliveryPipelineDeleteCall) (response)
/// * [locations delivery pipelines patch projects](ProjectLocationDeliveryPipelinePatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations targets create projects](ProjectLocationTargetCreateCall) (response)
/// * [locations targets delete projects](ProjectLocationTargetDeleteCall) (response)
/// * [locations targets patch projects](ProjectLocationTargetPatchCall) (response)
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


/// Phase represents a collection of jobs that are logically grouped together for a `Rollout`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Phase {
    /// Output only. ChildRollout job composition.
    #[serde(rename="childRolloutJobs")]
    
    pub child_rollout_jobs: Option<ChildRolloutJobs>,
    /// Output only. Deployment job composition.
    #[serde(rename="deploymentJobs")]
    
    pub deployment_jobs: Option<DeploymentJobs>,
    /// Output only. The ID of the Phase.
    
    pub id: Option<String>,
    /// Output only. Additional information on why the Phase was skipped, if available.
    #[serde(rename="skipMessage")]
    
    pub skip_message: Option<String>,
    /// Output only. Current state of the Phase.
    
    pub state: Option<PhaseStateEnum>,
}

impl client::Part for Phase {}


/// Contains the paths to the artifacts, relative to the URI, for a phase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhaseArtifact {
    /// Output only. File path of the directory of rendered job manifests relative to the URI. This is only set if it is applicable.
    #[serde(rename="jobManifestsPath")]
    
    pub job_manifests_path: Option<String>,
    /// Output only. File path of the rendered manifest relative to the URI.
    #[serde(rename="manifestPath")]
    
    pub manifest_path: Option<String>,
    /// Output only. File path of the resolved Skaffold configuration relative to the URI.
    #[serde(rename="skaffoldConfigPath")]
    
    pub skaffold_config_path: Option<String>,
}

impl client::Part for PhaseArtifact {}


/// PhaseConfig represents the configuration for a phase in the custom canary deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhaseConfig {
    /// Required. Percentage deployment for the phase.
    
    pub percentage: Option<i32>,
    /// Required. The ID to assign to the `Rollout` phase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
    /// Optional. Configuration for the postdeploy job of this phase. If this is not configured, there will be no postdeploy job for this phase.
    
    pub postdeploy: Option<Postdeploy>,
    /// Optional. Configuration for the predeploy job of this phase. If this is not configured, there will be no predeploy job for this phase.
    
    pub predeploy: Option<Predeploy>,
    /// Skaffold profiles to use when rendering the manifest for this phase. These are in addition to the profiles list specified in the `DeliveryPipeline` stage.
    
    pub profiles: Option<Vec<String>>,
    /// Whether to run verify tests after the deployment.
    
    pub verify: Option<bool>,
}

impl client::Part for PhaseConfig {}


/// PipelineCondition contains all conditions relevant to a Delivery Pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PipelineCondition {
    /// Details around the Pipeline's overall status.
    #[serde(rename="pipelineReadyCondition")]
    
    pub pipeline_ready_condition: Option<PipelineReadyCondition>,
    /// Details around targets enumerated in the pipeline.
    #[serde(rename="targetsPresentCondition")]
    
    pub targets_present_condition: Option<TargetsPresentCondition>,
    /// Details on the whether the targets enumerated in the pipeline are of the same type.
    #[serde(rename="targetsTypeCondition")]
    
    pub targets_type_condition: Option<TargetsTypeCondition>,
}

impl client::Part for PipelineCondition {}


/// PipelineReadyCondition contains information around the status of the Pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PipelineReadyCondition {
    /// True if the Pipeline is in a valid state. Otherwise at least one condition in `PipelineCondition` is in an invalid state. Iterate over those conditions and see which condition(s) has status = false to find out what is wrong with the Pipeline.
    
    pub status: Option<bool>,
    /// Last time the condition was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PipelineReadyCondition {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom target types get iam policy projects](ProjectLocationCustomTargetTypeGetIamPolicyCall) (response)
/// * [locations custom target types set iam policy projects](ProjectLocationCustomTargetTypeSetIamPolicyCall) (response)
/// * [locations delivery pipelines get iam policy projects](ProjectLocationDeliveryPipelineGetIamPolicyCall) (response)
/// * [locations delivery pipelines set iam policy projects](ProjectLocationDeliveryPipelineSetIamPolicyCall) (response)
/// * [locations targets get iam policy projects](ProjectLocationTargetGetIamPolicyCall) (response)
/// * [locations targets set iam policy projects](ProjectLocationTargetSetIamPolicyCall) (response)
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


/// Postdeploy contains the postdeploy job configuration information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Postdeploy {
    /// Optional. A sequence of Skaffold custom actions to invoke during execution of the postdeploy job.
    
    pub actions: Option<Vec<String>>,
}

impl client::Part for Postdeploy {}


/// A postdeploy Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostdeployJob {
    /// Output only. The custom actions that the postdeploy Job executes.
    
    pub actions: Option<Vec<String>>,
}

impl client::Part for PostdeployJob {}


/// PostdeployJobRun contains information specific to a postdeploy `JobRun`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostdeployJobRun {
    /// Output only. The resource name of the Cloud Build `Build` object that is used to execute the custom actions associated with the postdeploy Job. Format is `projects/{project}/locations/{location}/builds/{build}`.
    
    pub build: Option<String>,
    /// Output only. The reason the postdeploy failed. This will always be unspecified while the postdeploy is in progress or if it succeeded.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<PostdeployJobRunFailureCauseEnum>,
    /// Output only. Additional information about the postdeploy failure, if available.
    #[serde(rename="failureMessage")]
    
    pub failure_message: Option<String>,
}

impl client::Part for PostdeployJobRun {}


/// Predeploy contains the predeploy job configuration information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Predeploy {
    /// Optional. A sequence of Skaffold custom actions to invoke during execution of the predeploy job.
    
    pub actions: Option<Vec<String>>,
}

impl client::Part for Predeploy {}


/// A predeploy Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PredeployJob {
    /// Output only. The custom actions that the predeploy Job executes.
    
    pub actions: Option<Vec<String>>,
}

impl client::Part for PredeployJob {}


/// PredeployJobRun contains information specific to a predeploy `JobRun`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PredeployJobRun {
    /// Output only. The resource name of the Cloud Build `Build` object that is used to execute the custom actions associated with the predeploy Job. Format is `projects/{project}/locations/{location}/builds/{build}`.
    
    pub build: Option<String>,
    /// Output only. The reason the predeploy failed. This will always be unspecified while the predeploy is in progress or if it succeeded.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<PredeployJobRunFailureCauseEnum>,
    /// Output only. Additional information about the predeploy failure, if available.
    #[serde(rename="failureMessage")]
    
    pub failure_message: Option<String>,
}

impl client::Part for PredeployJobRun {}


/// Execution using a private Cloud Build pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivatePool {
    /// Optional. Cloud Storage location where execution outputs should be stored. This can either be a bucket ("gs://my-bucket") or a path within a bucket ("gs://my-bucket/my-dir"). If unspecified, a default bucket located in the same region will be used.
    #[serde(rename="artifactStorage")]
    
    pub artifact_storage: Option<String>,
    /// Optional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Required. Resource name of the Cloud Build worker pool to use. The format is `projects/{project}/locations/{location}/workerPools/{pool}`.
    #[serde(rename="workerPool")]
    
    pub worker_pool: Option<String>,
}

impl client::Part for PrivatePool {}


/// Contains the information of an automated promote-release operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromoteReleaseOperation {
    /// Output only. The starting phase of the rollout created by this operation.
    
    pub phase: Option<String>,
    /// Output only. The name of the rollout that initiates the `AutomationRun`.
    
    pub rollout: Option<String>,
    /// Output only. The ID of the target that represents the promotion stage to which the release will be promoted. The value of this field is the last segment of a target name.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// Output only. How long the operation will be paused.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub wait: Option<client::chrono::Duration>,
}

impl client::Part for PromoteReleaseOperation {}


/// `PromoteRelease` rule will automatically promote a release from the current target to a specified target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromoteReleaseRule {
    /// Output only. Information around the state of the Automation rule.
    
    pub condition: Option<AutomationRuleCondition>,
    /// Optional. The starting phase of the rollout created by this operation. Default to the first phase.
    #[serde(rename="destinationPhase")]
    
    pub destination_phase: Option<String>,
    /// Optional. The ID of the stage in the pipeline to which this `Release` is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine if the target is one of the stages in the promotion sequence defined in the pipeline. * "@next", the next target in the promotion sequence.
    #[serde(rename="destinationTargetId")]
    
    pub destination_target_id: Option<String>,
    /// Required. ID of the rule. This id must be unique in the `Automation` resource to which this rule belongs. The format is `a-z{0,62}`.
    
    pub id: Option<String>,
    /// Optional. How long the release need to be paused until being promoted to the next target.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub wait: Option<client::chrono::Duration>,
}

impl client::Part for PromoteReleaseRule {}


/// A `Release` resource in the Cloud Deploy API. A `Release` defines a specific Skaffold configuration instance that can be deployed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases create projects](ProjectLocationDeliveryPipelineReleaseCreateCall) (request)
/// * [locations delivery pipelines releases get projects](ProjectLocationDeliveryPipelineReleaseGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    /// Output only. Indicates whether this is an abandoned release.
    
    pub abandoned: Option<bool>,
    /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
    
    pub annotations: Option<HashMap<String, String>>,
    /// List of artifacts to pass through to Skaffold command.
    #[serde(rename="buildArtifacts")]
    
    pub build_artifacts: Option<Vec<BuildArtifact>>,
    /// Output only. Information around the state of the Release.
    
    pub condition: Option<ReleaseCondition>,
    /// Output only. Time at which the `Release` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Snapshot of the custom target types referenced by the targets taken at release creation time.
    #[serde(rename="customTargetTypeSnapshots")]
    
    pub custom_target_type_snapshots: Option<Vec<CustomTargetType>>,
    /// Output only. Snapshot of the parent pipeline taken at release creation time.
    #[serde(rename="deliveryPipelineSnapshot")]
    
    pub delivery_pipeline_snapshot: Option<DeliveryPipeline>,
    /// Optional. The deploy parameters to use for all targets in this release.
    #[serde(rename="deployParameters")]
    
    pub deploy_parameters: Option<HashMap<String, String>>,
    /// Description of the `Release`. Max length is 255 characters.
    
    pub description: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Name of the `Release`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/a-z{0,62}`.
    
    pub name: Option<String>,
    /// Output only. Time at which the render completed.
    #[serde(rename="renderEndTime")]
    
    pub render_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Time at which the render began.
    #[serde(rename="renderStartTime")]
    
    pub render_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Current state of the render operation.
    #[serde(rename="renderState")]
    
    pub render_state: Option<ReleaseRenderStateEnum>,
    /// Filepath of the Skaffold config inside of the config URI.
    #[serde(rename="skaffoldConfigPath")]
    
    pub skaffold_config_path: Option<String>,
    /// Cloud Storage URI of tar.gz archive containing Skaffold configuration.
    #[serde(rename="skaffoldConfigUri")]
    
    pub skaffold_config_uri: Option<String>,
    /// The Skaffold version to use when operating on this release, such as "1.20.0". Not all versions are valid; Cloud Deploy supports a specific set of versions. If unset, the most recent supported Skaffold version will be used.
    #[serde(rename="skaffoldVersion")]
    
    pub skaffold_version: Option<String>,
    /// Output only. Map from target ID to the target artifacts created during the render operation.
    #[serde(rename="targetArtifacts")]
    
    pub target_artifacts: Option<HashMap<String, TargetArtifact>>,
    /// Output only. Map from target ID to details of the render operation for that target.
    #[serde(rename="targetRenders")]
    
    pub target_renders: Option<HashMap<String, TargetRender>>,
    /// Output only. Snapshot of the targets taken at release creation time.
    #[serde(rename="targetSnapshots")]
    
    pub target_snapshots: Option<Vec<Target>>,
    /// Output only. Unique identifier of the `Release`.
    
    pub uid: Option<String>,
}

impl client::RequestValue for Release {}
impl client::ResponseResult for Release {}


/// ReleaseCondition contains all conditions relevant to a Release.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseCondition {
    /// Details around the Releases's overall status.
    #[serde(rename="releaseReadyCondition")]
    
    pub release_ready_condition: Option<ReleaseReadyCondition>,
    /// Details around the support state of the release's Skaffold version.
    #[serde(rename="skaffoldSupportedCondition")]
    
    pub skaffold_supported_condition: Option<SkaffoldSupportedCondition>,
}

impl client::Part for ReleaseCondition {}


/// ReleaseReadyCondition contains information around the status of the Release. If a release is not ready, you cannot create a rollout with the release.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseReadyCondition {
    /// True if the Release is in a valid state. Otherwise at least one condition in `ReleaseCondition` is in an invalid state. Iterate over those conditions and see which condition(s) has status = false to find out what is wrong with the Release.
    
    pub status: Option<bool>,
}

impl client::Part for ReleaseReadyCondition {}


/// RenderMetadata includes information associated with a `Release` render.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RenderMetadata {
    /// Output only. Metadata associated with rendering for Cloud Run.
    #[serde(rename="cloudRun")]
    
    pub cloud_run: Option<CloudRunRenderMetadata>,
    /// Output only. Custom metadata provided by user-defined render operation.
    
    pub custom: Option<CustomMetadata>,
}

impl client::Part for RenderMetadata {}


/// Configuration of the repair action.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepairMode {
    /// Optional. Retries a failed job.
    
    pub retry: Option<Retry>,
    /// Optional. Rolls back a `Rollout`.
    
    pub rollback: Option<Rollback>,
}

impl client::Part for RepairMode {}


/// RepairPhase tracks the repair attempts that have been made for each `RepairMode` specified in the `Automation` resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepairPhase {
    /// Output only. Records of the retry attempts for retry repair mode.
    
    pub retry: Option<RetryPhase>,
    /// Output only. Rollback attempt for rollback repair mode .
    
    pub rollback: Option<RollbackAttempt>,
}

impl client::Part for RepairPhase {}


/// Contains the information for an automated `repair rollout` operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepairRolloutOperation {
    /// Output only. The index of the current repair action in the repair sequence.
    #[serde(rename="currentRepairModeIndex")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_repair_mode_index: Option<i64>,
    /// Output only. The job ID for the Job to repair.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Output only. The phase ID of the phase that includes the job being repaired.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
    /// Output only. Records of the repair attempts. Each repair phase may have multiple retry attempts or single rollback attempt.
    #[serde(rename="repairPhases")]
    
    pub repair_phases: Option<Vec<RepairPhase>>,
    /// Output only. The name of the rollout that initiates the `AutomationRun`.
    
    pub rollout: Option<String>,
}

impl client::Part for RepairRolloutOperation {}


/// The `RepairRolloutRule` automation rule will automatically repair a failed `Rollout`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepairRolloutRule {
    /// Output only. Information around the state of the 'Automation' rule.
    
    pub condition: Option<AutomationRuleCondition>,
    /// Required. ID of the rule. This id must be unique in the `Automation` resource to which this rule belongs. The format is `a-z{0,62}`.
    
    pub id: Option<String>,
    /// Optional. Jobs to repair. Proceeds only after job name matched any one in the list, or for all jobs if unspecified or empty. The phase that includes the job must match the phase ID specified in `source_phase`. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`.
    
    pub jobs: Option<Vec<String>>,
    /// Required. Defines the types of automatic repair actions for failed jobs.
    #[serde(rename="repairModes")]
    
    pub repair_modes: Option<Vec<RepairMode>>,
    /// Optional. Phases within which jobs are subject to automatic repair actions on failure. Proceeds only after phase name matched any one in the list, or for all phases if unspecified. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$`.
    #[serde(rename="sourcePhases")]
    
    pub source_phases: Option<Vec<String>>,
}

impl client::Part for RepairRolloutRule {}


/// Retries the failed job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Retry {
    /// Required. Total number of retries. Retry is skipped if set to 0; The minimum value is 1, and the maximum value is 10.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub attempts: Option<i64>,
    /// Optional. The pattern of how wait time will be increased. Default is linear. Backoff mode will be ignored if `wait` is 0.
    #[serde(rename="backoffMode")]
    
    pub backoff_mode: Option<RetryBackoffModeEnum>,
    /// Optional. How long to wait for the first retry. Default is 0, and the maximum value is 14d.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub wait: Option<client::chrono::Duration>,
}

impl client::Part for Retry {}


/// RetryAttempt represents an action of retrying the failed Cloud Deploy job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RetryAttempt {
    /// Output only. The index of this retry attempt.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub attempt: Option<i64>,
    /// Output only. Valid state of this retry action.
    
    pub state: Option<RetryAttemptStateEnum>,
    /// Output only. Description of the state of the Retry.
    #[serde(rename="stateDesc")]
    
    pub state_desc: Option<String>,
    /// Output only. How long the operation will be paused.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub wait: Option<client::chrono::Duration>,
}

impl client::Part for RetryAttempt {}


/// RetryJobRequest is the request object used by `RetryJob`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts retry job projects](ProjectLocationDeliveryPipelineReleaseRolloutRetryJobCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RetryJobRequest {
    /// Required. The job ID for the Job to retry.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Required. The phase ID the Job to retry belongs to.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
}

impl client::RequestValue for RetryJobRequest {}


/// The response object from ‘RetryJob’.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts retry job projects](ProjectLocationDeliveryPipelineReleaseRolloutRetryJobCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RetryJobResponse { _never_set: Option<bool> }

impl client::ResponseResult for RetryJobResponse {}


/// RetryPhase contains the retry attempts and the metadata for initiating a new attempt.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RetryPhase {
    /// Output only. Detail of a retry action.
    
    pub attempts: Option<Vec<RetryAttempt>>,
    /// Output only. The pattern of how the wait time of the retry attempt is calculated.
    #[serde(rename="backoffMode")]
    
    pub backoff_mode: Option<RetryPhaseBackoffModeEnum>,
    /// Output only. The job ID for the Job to retry.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Output only. The phase ID of the phase that includes the job being retried.
    #[serde(rename="phaseId")]
    
    pub phase_id: Option<String>,
    /// Output only. The number of attempts that have been made.
    #[serde(rename="totalAttempts")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_attempts: Option<i64>,
}

impl client::Part for RetryPhase {}


/// Rolls back a `Rollout`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rollback {
    /// Optional. The starting phase ID for the `Rollout`. If unspecified, the `Rollout` will start in the stable phase.
    #[serde(rename="destinationPhase")]
    
    pub destination_phase: Option<String>,
}

impl client::Part for Rollback {}


/// RollbackAttempt represents an action of rolling back a Cloud Deploy 'Target'.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackAttempt {
    /// Output only. The phase to which the rollout will be rolled back to.
    #[serde(rename="destinationPhase")]
    
    pub destination_phase: Option<String>,
    /// Output only. ID of the rollback `Rollout` to create.
    #[serde(rename="rolloutId")]
    
    pub rollout_id: Option<String>,
    /// Output only. Valid state of this rollback action.
    
    pub state: Option<RollbackAttemptStateEnum>,
    /// Output only. Description of the state of the Rollback.
    #[serde(rename="stateDesc")]
    
    pub state_desc: Option<String>,
}

impl client::Part for RollbackAttempt {}


/// Configs for the Rollback rollout.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackTargetConfig {
    /// Optional. The rollback `Rollout` to create.
    
    pub rollout: Option<Rollout>,
    /// Optional. The starting phase ID for the `Rollout`. If unspecified, the `Rollout` will start in the stable phase.
    #[serde(rename="startingPhaseId")]
    
    pub starting_phase_id: Option<String>,
}

impl client::Part for RollbackTargetConfig {}


/// The request object for `RollbackTarget`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines rollback target projects](ProjectLocationDeliveryPipelineRollbackTargetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackTargetRequest {
    /// Optional. ID of the `Release` to roll back to. If this isn't specified, the previous successful `Rollout` to the specified target will be used to determine the `Release`.
    #[serde(rename="releaseId")]
    
    pub release_id: Option<String>,
    /// Optional. Configs for the rollback `Rollout`.
    #[serde(rename="rollbackConfig")]
    
    pub rollback_config: Option<RollbackTargetConfig>,
    /// Required. ID of the rollback `Rollout` to create.
    #[serde(rename="rolloutId")]
    
    pub rollout_id: Option<String>,
    /// Optional. If provided, this must be the latest `Rollout` that is on the `Target`.
    #[serde(rename="rolloutToRollBack")]
    
    pub rollout_to_roll_back: Option<String>,
    /// Required. ID of the `Target` that is being rolled back.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// Optional. If set to true, the request is validated and the user is provided with a `RollbackTargetResponse`.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for RollbackTargetRequest {}


/// The response object from `RollbackTarget`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines rollback target projects](ProjectLocationDeliveryPipelineRollbackTargetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RollbackTargetResponse {
    /// The config of the rollback `Rollout` created or will be created.
    #[serde(rename="rollbackConfig")]
    
    pub rollback_config: Option<RollbackTargetConfig>,
}

impl client::ResponseResult for RollbackTargetResponse {}


/// A `Rollout` resource in the Cloud Deploy API. A `Rollout` contains information around a specific deployment to a `Target`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts create projects](ProjectLocationDeliveryPipelineReleaseRolloutCreateCall) (request)
/// * [locations delivery pipelines releases rollouts get projects](ProjectLocationDeliveryPipelineReleaseRolloutGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rollout {
    /// User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. Approval state of the `Rollout`.
    #[serde(rename="approvalState")]
    
    pub approval_state: Option<RolloutApprovalStateEnum>,
    /// Output only. Time at which the `Rollout` was approved.
    #[serde(rename="approveTime")]
    
    pub approve_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Name of the `ControllerRollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/a-z{0,62}`.
    #[serde(rename="controllerRollout")]
    
    pub controller_rollout: Option<String>,
    /// Output only. Time at which the `Rollout` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Time at which the `Rollout` finished deploying.
    #[serde(rename="deployEndTime")]
    
    pub deploy_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The reason this rollout failed. This will always be unspecified while the rollout is in progress.
    #[serde(rename="deployFailureCause")]
    
    pub deploy_failure_cause: Option<RolloutDeployFailureCauseEnum>,
    /// Output only. Time at which the `Rollout` started deploying.
    #[serde(rename="deployStartTime")]
    
    pub deploy_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The resource name of the Cloud Build `Build` object that is used to deploy the Rollout. Format is `projects/{project}/locations/{location}/builds/{build}`.
    #[serde(rename="deployingBuild")]
    
    pub deploying_build: Option<String>,
    /// Description of the `Rollout` for user purposes. Max length is 255 characters.
    
    pub description: Option<String>,
    /// Output only. Time at which the `Rollout` was enqueued.
    #[serde(rename="enqueueTime")]
    
    pub enqueue_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. Additional information about the rollout failure, if available.
    #[serde(rename="failureReason")]
    
    pub failure_reason: Option<String>,
    /// Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Metadata contains information about the rollout.
    
    pub metadata: Option<Metadata>,
    /// Optional. Name of the `Rollout`. Format is `projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/releases/{release}/rollouts/a-z{0,62}`.
    
    pub name: Option<String>,
    /// Output only. The phases that represent the workflows of this `Rollout`.
    
    pub phases: Option<Vec<Phase>>,
    /// Output only. Name of the `Rollout` that is rolled back by this `Rollout`. Empty if this `Rollout` wasn't created as a rollback.
    #[serde(rename="rollbackOfRollout")]
    
    pub rollback_of_rollout: Option<String>,
    /// Output only. Names of `Rollouts` that rolled back this `Rollout`.
    #[serde(rename="rolledBackByRollouts")]
    
    pub rolled_back_by_rollouts: Option<Vec<String>>,
    /// Output only. Current state of the `Rollout`.
    
    pub state: Option<RolloutStateEnum>,
    /// Required. The ID of Target to which this `Rollout` is deploying.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// Output only. Unique identifier of the `Rollout`.
    
    pub uid: Option<String>,
}

impl client::RequestValue for Rollout {}
impl client::ResponseResult for Rollout {}


/// RuntimeConfig contains the runtime specific configurations for a deployment strategy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Cloud Run runtime configuration.
    #[serde(rename="cloudRun")]
    
    pub cloud_run: Option<CloudRunConfig>,
    /// Kubernetes runtime configuration.
    
    pub kubernetes: Option<KubernetesConfig>,
}

impl client::Part for RuntimeConfig {}


/// SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SerialPipeline {
    /// Each stage specifies configuration for a `Target`. The ordering of this list defines the promotion flow.
    
    pub stages: Option<Vec<Stage>>,
}

impl client::Part for SerialPipeline {}


/// Information about the Kubernetes Service networking configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceNetworking {
    /// Required. Name of the Kubernetes Deployment whose traffic is managed by the specified Service.
    
    pub deployment: Option<String>,
    /// Optional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster.
    #[serde(rename="disablePodOverprovisioning")]
    
    pub disable_pod_overprovisioning: Option<bool>,
    /// Required. Name of the Kubernetes Service.
    
    pub service: Option<String>,
}

impl client::Part for ServiceNetworking {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations custom target types set iam policy projects](ProjectLocationCustomTargetTypeSetIamPolicyCall) (request)
/// * [locations delivery pipelines set iam policy projects](ProjectLocationDeliveryPipelineSetIamPolicyCall) (request)
/// * [locations targets set iam policy projects](ProjectLocationTargetSetIamPolicyCall) (request)
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


/// Cloud Storage bucket containing Skaffold Config modules.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SkaffoldGCSSource {
    /// Optional. Relative path from the source to the Skaffold file.
    
    pub path: Option<String>,
    /// Required. Cloud Storage source paths to copy recursively. For example, providing "gs://my-bucket/dir/configs/*" will result in Skaffold copying all files within the "dir/configs" directory in the bucket "my-bucket".
    
    pub source: Option<String>,
}

impl client::Part for SkaffoldGCSSource {}


/// Git repository containing Skaffold Config modules.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SkaffoldGitSource {
    /// Optional. Relative path from the repository root to the Skaffold file.
    
    pub path: Option<String>,
    /// Optional. Git ref the package should be cloned from.
    #[serde(rename="ref")]
    
    pub ref_: Option<String>,
    /// Required. Git repository the package should be cloned from.
    
    pub repo: Option<String>,
}

impl client::Part for SkaffoldGitSource {}


/// Skaffold Config modules and their remote source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SkaffoldModules {
    /// Optional. The Skaffold Config modules to use from the specified source.
    
    pub configs: Option<Vec<String>>,
    /// Remote git repository containing the Skaffold Config modules.
    
    pub git: Option<SkaffoldGitSource>,
    /// Cloud Storage bucket containing the Skaffold Config modules.
    #[serde(rename="googleCloudStorage")]
    
    pub google_cloud_storage: Option<SkaffoldGCSSource>,
}

impl client::Part for SkaffoldModules {}


/// SkaffoldSupportedCondition contains information about when support for the release's version of Skaffold ends.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SkaffoldSupportedCondition {
    /// The time at which this release's version of Skaffold will enter maintenance mode.
    #[serde(rename="maintenanceModeTime")]
    
    pub maintenance_mode_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The Skaffold support state for this release's version of Skaffold.
    #[serde(rename="skaffoldSupportState")]
    
    pub skaffold_support_state: Option<SkaffoldSupportedConditionSkaffoldSupportStateEnum>,
    /// True if the version of Skaffold used by this release is supported.
    
    pub status: Option<bool>,
    /// The time at which this release's version of Skaffold will no longer be supported.
    #[serde(rename="supportExpirationTime")]
    
    pub support_expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for SkaffoldSupportedCondition {}


/// Details of a supported Skaffold version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SkaffoldVersion {
    /// The time at which this version of Skaffold will enter maintenance mode.
    #[serde(rename="maintenanceModeTime")]
    
    pub maintenance_mode_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Date when this version is expected to no longer be supported.
    #[serde(rename="supportEndDate")]
    
    pub support_end_date: Option<Date>,
    /// The time at which this version of Skaffold will no longer be supported.
    #[serde(rename="supportExpirationTime")]
    
    pub support_expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Release version number. For example, "1.20.3".
    
    pub version: Option<String>,
}

impl client::Part for SkaffoldVersion {}


/// Stage specifies a location to which to deploy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Stage {
    /// Optional. The deploy parameters to use for the target in this stage.
    #[serde(rename="deployParameters")]
    
    pub deploy_parameters: Option<Vec<DeployParameters>>,
    /// Skaffold profiles to use when rendering the manifest for this stage's `Target`.
    
    pub profiles: Option<Vec<String>>,
    /// Optional. The strategy to use for a `Rollout` to this stage.
    
    pub strategy: Option<Strategy>,
    /// The target_id to which this stage points. This field refers exclusively to the last segment of a target name. For example, this field would just be `my-target` (rather than `projects/project/locations/location/targets/my-target`). The location of the `Target` is inferred to be the same as the location of the `DeliveryPipeline` that contains this `Stage`.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
}

impl client::Part for Stage {}


/// Standard represents the standard deployment strategy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Standard {
    /// Optional. Configuration for the postdeploy job. If this is not configured, postdeploy job will not be present.
    
    pub postdeploy: Option<Postdeploy>,
    /// Optional. Configuration for the predeploy job. If this is not configured, predeploy job will not be present.
    
    pub predeploy: Option<Predeploy>,
    /// Whether to verify a deployment.
    
    pub verify: Option<bool>,
}

impl client::Part for Standard {}


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


/// Strategy contains deployment strategy information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Strategy {
    /// Canary deployment strategy provides progressive percentage based deployments to a Target.
    
    pub canary: Option<Canary>,
    /// Standard deployment strategy executes a single deploy and allows verifying the deployment.
    
    pub standard: Option<Standard>,
}

impl client::Part for Strategy {}


/// A `Target` resource in the Cloud Deploy API. A `Target` defines a location to which a Skaffold configuration can be deployed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations targets create projects](ProjectLocationTargetCreateCall) (request)
/// * [locations targets get projects](ProjectLocationTargetGetCall) (response)
/// * [locations targets patch projects](ProjectLocationTargetPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Target {
    /// Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Optional. Information specifying an Anthos Cluster.
    #[serde(rename="anthosCluster")]
    
    pub anthos_cluster: Option<AnthosCluster>,
    /// Output only. Time at which the `Target` was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Information specifying a Custom Target.
    #[serde(rename="customTarget")]
    
    pub custom_target: Option<CustomTarget>,
    /// Optional. The deploy parameters to use for this target.
    #[serde(rename="deployParameters")]
    
    pub deploy_parameters: Option<HashMap<String, String>>,
    /// Optional. Description of the `Target`. Max length is 255 characters.
    
    pub description: Option<String>,
    /// Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`.
    #[serde(rename="executionConfigs")]
    
    pub execution_configs: Option<Vec<ExecutionConfig>>,
    /// Optional. Information specifying a GKE Cluster.
    
    pub gke: Option<GkeCluster>,
    /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Information specifying a multiTarget.
    #[serde(rename="multiTarget")]
    
    pub multi_target: Option<MultiTarget>,
    /// Optional. Name of the `Target`. Format is `projects/{project}/locations/{location}/targets/a-z{0,62}`.
    
    pub name: Option<String>,
    /// Optional. Whether or not the `Target` requires approval.
    #[serde(rename="requireApproval")]
    
    pub require_approval: Option<bool>,
    /// Optional. Information specifying a Cloud Run deployment target.
    
    pub run: Option<CloudRunLocation>,
    /// Output only. Resource id of the `Target`.
    #[serde(rename="targetId")]
    
    pub target_id: Option<String>,
    /// Output only. Unique identifier of the `Target`.
    
    pub uid: Option<String>,
    /// Output only. Most recent time at which the `Target` was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Target {}
impl client::ResponseResult for Target {}


/// The artifacts produced by a target render operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetArtifact {
    /// Output only. URI of a directory containing the artifacts. This contains deployment configuration used by Skaffold during a rollout, and all paths are relative to this location.
    #[serde(rename="artifactUri")]
    
    pub artifact_uri: Option<String>,
    /// Output only. File path of the rendered manifest relative to the URI.
    #[serde(rename="manifestPath")]
    
    pub manifest_path: Option<String>,
    /// Output only. Map from the phase ID to the phase artifacts for the `Target`.
    #[serde(rename="phaseArtifacts")]
    
    pub phase_artifacts: Option<HashMap<String, PhaseArtifact>>,
    /// Output only. File path of the resolved Skaffold configuration relative to the URI.
    #[serde(rename="skaffoldConfigPath")]
    
    pub skaffold_config_path: Option<String>,
}

impl client::Part for TargetArtifact {}


/// Contains criteria for selecting Targets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetAttribute {
    /// ID of the `Target`. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine which target is being referred to * "*", all targets in a location.
    
    pub id: Option<String>,
    /// Target labels.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for TargetAttribute {}


/// Details of rendering for a single target.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetRender {
    /// Output only. Reason this render failed. This will always be unspecified while the render in progress.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<TargetRenderFailureCauseEnum>,
    /// Output only. Additional information about the render failure, if available.
    #[serde(rename="failureMessage")]
    
    pub failure_message: Option<String>,
    /// Output only. Metadata related to the `Release` render for this Target.
    
    pub metadata: Option<RenderMetadata>,
    /// Output only. The resource name of the Cloud Build `Build` object that is used to render the manifest for this target. Format is `projects/{project}/locations/{location}/builds/{build}`.
    #[serde(rename="renderingBuild")]
    
    pub rendering_build: Option<String>,
    /// Output only. Current state of the render operation for this Target.
    #[serde(rename="renderingState")]
    
    pub rendering_state: Option<TargetRenderRenderingStateEnum>,
}

impl client::Part for TargetRender {}


/// `TargetsPresentCondition` contains information on any Targets referenced in the Delivery Pipeline that do not actually exist.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetsPresentCondition {
    /// The list of Target names that do not exist. For example, `projects/{project_id}/locations/{location_name}/targets/{target_name}`.
    #[serde(rename="missingTargets")]
    
    pub missing_targets: Option<Vec<String>>,
    /// True if there aren't any missing Targets.
    
    pub status: Option<bool>,
    /// Last time the condition was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TargetsPresentCondition {}


/// TargetsTypeCondition contains information on whether the Targets defined in the Delivery Pipeline are of the same type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetsTypeCondition {
    /// Human readable error message.
    #[serde(rename="errorDetails")]
    
    pub error_details: Option<String>,
    /// True if the targets are all a comparable type. For example this is true if all targets are GKE clusters. This is false if some targets are Cloud Run targets and others are GKE clusters.
    
    pub status: Option<bool>,
}

impl client::Part for TargetsTypeCondition {}


/// The request object used by `TerminateJobRun`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts job runs terminate projects](ProjectLocationDeliveryPipelineReleaseRolloutJobRunTerminateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TerminateJobRunRequest { _never_set: Option<bool> }

impl client::RequestValue for TerminateJobRunRequest {}


/// The response object from `TerminateJobRun`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines releases rollouts job runs terminate projects](ProjectLocationDeliveryPipelineReleaseRolloutJobRunTerminateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TerminateJobRunResponse { _never_set: Option<bool> }

impl client::ResponseResult for TerminateJobRunResponse {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations delivery pipelines test iam permissions projects](ProjectLocationDeliveryPipelineTestIamPermissionCall) (request)
/// * [locations targets test iam permissions projects](ProjectLocationTargetTestIamPermissionCall) (request)
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
/// * [locations delivery pipelines test iam permissions projects](ProjectLocationDeliveryPipelineTestIamPermissionCall) (response)
/// * [locations targets test iam permissions projects](ProjectLocationTargetTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// A verify Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyJob { _never_set: Option<bool> }

impl client::Part for VerifyJob {}


/// VerifyJobRun contains information specific to a verify `JobRun`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VerifyJobRun {
    /// Output only. URI of a directory containing the verify artifacts. This contains the Skaffold event log.
    #[serde(rename="artifactUri")]
    
    pub artifact_uri: Option<String>,
    /// Output only. The resource name of the Cloud Build `Build` object that is used to verify. Format is `projects/{project}/locations/{location}/builds/{build}`.
    
    pub build: Option<String>,
    /// Output only. File path of the Skaffold event log relative to the artifact URI.
    #[serde(rename="eventLogPath")]
    
    pub event_log_path: Option<String>,
    /// Output only. The reason the verify failed. This will always be unspecified while the verify is in progress or if it succeeded.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<VerifyJobRunFailureCauseEnum>,
    /// Output only. Additional information about the verify failure, if available.
    #[serde(rename="failureMessage")]
    
    pub failure_message: Option<String>,
}

impl client::Part for VerifyJobRun {}


