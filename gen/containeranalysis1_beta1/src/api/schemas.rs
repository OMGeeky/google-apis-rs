use super::*;
/// An alias to a repo revision.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AliasContext {
    /// The alias kind.
    
    pub kind: Option<AliasContextKindEnum>,
    /// The alias name.
    
    pub name: Option<String>,
}

impl client::Part for AliasContext {}


/// Indicates which analysis completed successfully. Multiple types of analysis can be performed on a single resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisCompleted {
    /// no description provided
    #[serde(rename="analysisType")]
    
    pub analysis_type: Option<Vec<String>>,
}

impl client::Part for AnalysisCompleted {}


/// Artifact describes a build product.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    /// Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container.
    
    pub checksum: Option<String>,
    /// Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`.
    
    pub id: Option<String>,
    /// Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image.
    
    pub names: Option<Vec<String>>,
}

impl client::Part for Artifact {}


/// Defines a hash object for use in Materials and Products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactHashes {
    /// no description provided
    
    pub sha256: Option<String>,
}

impl client::Part for ArtifactHashes {}


/// Defines an object to declare an in-toto artifact rule
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactRule {
    /// no description provided
    #[serde(rename="artifactRule")]
    
    pub artifact_rule: Option<Vec<String>>,
}

impl client::Part for ArtifactRule {}


/// Occurrence that represents a single "attestation". The authenticity of an attestation can be verified using the attached signature. If the verifier trusts the public key of the signer, then verifying the signature is sufficient to establish trust. In this circumstance, the authority to which this attestation is attached is primarily useful for look-up (how to find this attestation if you already know the authority and artifact to be verified) and intent (which authority was this attestation intended to sign for).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attestation {
    /// no description provided
    #[serde(rename="genericSignedAttestation")]
    
    pub generic_signed_attestation: Option<GenericSignedAttestation>,
    /// A PGP signed attestation.
    #[serde(rename="pgpSignedAttestation")]
    
    pub pgp_signed_attestation: Option<PgpSignedAttestation>,
}

impl client::Part for Attestation {}


/// Note kind that represents a logical attestation "role" or "authority". For example, an organization might have one `Authority` for "QA" and one for "build". This note is intended to act strictly as a grouping mechanism for the attached occurrences (Attestations). This grouping mechanism also provides a security boundary, since IAM ACLs gate the ability for a principle to attach an occurrence to a given note. It also provides a single point of lookup to find all attached attestation occurrences, even if they don't all live in the same project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Authority {
    /// Hint hints at the purpose of the attestation authority.
    
    pub hint: Option<Hint>,
}

impl client::Part for Authority {}


/// Basis describes the base image portion (Note) of the DockerImage relationship. Linked occurrences are derived from this or an equivalent image via: FROM Or an equivalent reference, e.g. a tag of the resource_url.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Basis {
    /// Required. Immutable. The fingerprint of the base image.
    
    pub fingerprint: Option<Fingerprint>,
    /// Required. Immutable. The resource_url for the resource representing the basis of associated occurrence images.
    #[serde(rename="resourceUrl")]
    
    pub resource_url: Option<String>,
}

impl client::Part for Basis {}


/// Request to create notes in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes batch create projects](ProjectNoteBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateNotesRequest {
    /// Required. The notes to create, the key is expected to be the note ID. Max allowed length is 1000.
    
    pub notes: Option<HashMap<String, Note>>,
}

impl client::RequestValue for BatchCreateNotesRequest {}


/// Response for creating notes in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes batch create projects](ProjectNoteBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateNotesResponse {
    /// The notes that were created.
    
    pub notes: Option<Vec<Note>>,
}

impl client::ResponseResult for BatchCreateNotesResponse {}


/// Request to create occurrences in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [occurrences batch create projects](ProjectOccurrenceBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateOccurrencesRequest {
    /// Required. The occurrences to create. Max allowed length is 1000.
    
    pub occurrences: Option<Vec<Occurrence>>,
}

impl client::RequestValue for BatchCreateOccurrencesRequest {}


/// Response for creating occurrences in batch.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [occurrences batch create projects](ProjectOccurrenceBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateOccurrencesResponse {
    /// The occurrences that were created.
    
    pub occurrences: Option<Vec<Occurrence>>,
}

impl client::ResponseResult for BatchCreateOccurrencesResponse {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// Note holding the version of the provider's builder and the signature of the provenance message in the build details occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    /// Required. Immutable. Version of the builder which produced this build.
    #[serde(rename="builderVersion")]
    
    pub builder_version: Option<String>,
    /// Signature of the build in occurrences pointing to this build note containing build details.
    
    pub signature: Option<BuildSignature>,
}

impl client::Part for Build {}


/// Provenance of a build. Contains all information needed to verify the full details about the build from source to completion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildProvenance {
    /// Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details.
    #[serde(rename="buildOptions")]
    
    pub build_options: Option<HashMap<String, String>>,
    /// Version string of the builder at the time this build was executed.
    #[serde(rename="builderVersion")]
    
    pub builder_version: Option<String>,
    /// Output of the build.
    #[serde(rename="builtArtifacts")]
    
    pub built_artifacts: Option<Vec<Artifact>>,
    /// Commands requested by the build.
    
    pub commands: Option<Vec<Command>>,
    /// Time at which the build was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time.
    
    pub creator: Option<String>,
    /// Time at which execution of the build was finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Unique identifier of the build.
    
    pub id: Option<String>,
    /// URI where any logs for this provenance were written.
    #[serde(rename="logsUri")]
    
    pub logs_uri: Option<String>,
    /// ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Details of the Source input to the build.
    #[serde(rename="sourceProvenance")]
    
    pub source_provenance: Option<Source>,
    /// Time at which execution of the build was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Trigger identifier if the build was triggered automatically; empty if not.
    #[serde(rename="triggerId")]
    
    pub trigger_id: Option<String>,
}

impl client::Part for BuildProvenance {}


/// Message encapsulating the signature of the verified build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildSignature {
    /// An ID for the key used to sign. This could be either an ID for the key stored in `public_key` (such as the ID or fingerprint for a PGP key, or the CN for a cert), or a reference to an external key (such as a reference to a key in Cloud Key Management Service).
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
    /// The type of the key, either stored in `public_key` or referenced in `key_id`.
    #[serde(rename="keyType")]
    
    pub key_type: Option<BuildSignatureKeyTypeEnum>,
    /// Public key of the builder which can be used to verify that the related findings are valid and unchanged. If `key_type` is empty, this defaults to PEM encoded public keys. This field may be empty if `key_id` references an external key. For Cloud Build based signatures, this is a PEM encoded public key. To verify the Cloud Build signature, place the contents of this field into a file (public.pem). The signature field is base64-decoded into its binary representation in signature.bin, and the provenance bytes from `BuildDetails` are base64-decoded into a binary representation in signed.bin. OpenSSL can then verify the signature: `openssl sha256 -verify public.pem -signature signature.bin signed.bin`
    #[serde(rename="publicKey")]
    
    pub public_key: Option<String>,
    /// Required. Signature of the related `BuildProvenance`. In JSON, this is base-64 encoded.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub signature: Option<Vec<u8>>,
}

impl client::Part for BuildSignature {}


/// Defines an object for the byproducts field in in-toto links. The suggested fields are "stderr", "stdout", and "return-value".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ByProducts {
    /// no description provided
    #[serde(rename="customValues")]
    
    pub custom_values: Option<HashMap<String, String>>,
}

impl client::Part for ByProducts {}


/// Common Vulnerability Scoring System. This message is compatible with CVSS v2 and v3. For CVSS v2 details, see https://www.first.org/cvss/v2/guide CVSS v2 calculator: https://nvd.nist.gov/vuln-metrics/cvss/v2-calculator For CVSS v3 details, see https://www.first.org/cvss/specification-document CVSS v3 calculator: https://nvd.nist.gov/vuln-metrics/cvss/v3-calculator
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CVSS {
    /// Defined in CVSS v3, CVSS v2
    #[serde(rename="attackComplexity")]
    
    pub attack_complexity: Option<CVSAttackComplexityEnum>,
    /// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments. Defined in CVSS v3, CVSS v2
    #[serde(rename="attackVector")]
    
    pub attack_vector: Option<CVSAttackVectorEnum>,
    /// Defined in CVSS v2
    
    pub authentication: Option<CVSAuthenticationEnum>,
    /// Defined in CVSS v3, CVSS v2
    #[serde(rename="availabilityImpact")]
    
    pub availability_impact: Option<CVSAvailabilityImpactEnum>,
    /// The base score is a function of the base metric scores.
    #[serde(rename="baseScore")]
    
    pub base_score: Option<f32>,
    /// Defined in CVSS v3, CVSS v2
    #[serde(rename="confidentialityImpact")]
    
    pub confidentiality_impact: Option<CVSConfidentialityImpactEnum>,
    /// no description provided
    #[serde(rename="exploitabilityScore")]
    
    pub exploitability_score: Option<f32>,
    /// no description provided
    #[serde(rename="impactScore")]
    
    pub impact_score: Option<f32>,
    /// Defined in CVSS v3, CVSS v2
    #[serde(rename="integrityImpact")]
    
    pub integrity_impact: Option<CVSIntegrityImpactEnum>,
    /// Defined in CVSS v3
    #[serde(rename="privilegesRequired")]
    
    pub privileges_required: Option<CVSPrivilegesRequiredEnum>,
    /// Defined in CVSS v3
    
    pub scope: Option<CVSScopeEnum>,
    /// Defined in CVSS v3
    #[serde(rename="userInteraction")]
    
    pub user_interaction: Option<CVSUserInteractionEnum>,
}

impl client::Part for CVSS {}


/// Deprecated. Common Vulnerability Scoring System version 3. For details, see https://www.first.org/cvss/specification-document
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CVSSv3 {
    /// no description provided
    #[serde(rename="attackComplexity")]
    
    pub attack_complexity: Option<CVSSv3AttackComplexityEnum>,
    /// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments.
    #[serde(rename="attackVector")]
    
    pub attack_vector: Option<CVSSv3AttackVectorEnum>,
    /// no description provided
    #[serde(rename="availabilityImpact")]
    
    pub availability_impact: Option<CVSSv3AvailabilityImpactEnum>,
    /// The base score is a function of the base metric scores.
    #[serde(rename="baseScore")]
    
    pub base_score: Option<f32>,
    /// no description provided
    #[serde(rename="confidentialityImpact")]
    
    pub confidentiality_impact: Option<CVSSv3ConfidentialityImpactEnum>,
    /// no description provided
    #[serde(rename="exploitabilityScore")]
    
    pub exploitability_score: Option<f32>,
    /// no description provided
    #[serde(rename="impactScore")]
    
    pub impact_score: Option<f32>,
    /// no description provided
    #[serde(rename="integrityImpact")]
    
    pub integrity_impact: Option<CVSSv3IntegrityImpactEnum>,
    /// no description provided
    #[serde(rename="privilegesRequired")]
    
    pub privileges_required: Option<CVSSv3PrivilegesRequiredEnum>,
    /// no description provided
    
    pub scope: Option<CVSSv3ScopeEnum>,
    /// no description provided
    #[serde(rename="userInteraction")]
    
    pub user_interaction: Option<CVSSv3UserInteractionEnum>,
}

impl client::Part for CVSSv3 {}


/// A CloudRepoSourceContext denotes a particular revision in a Google Cloud Source Repo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRepoSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    
    pub alias_context: Option<AliasContext>,
    /// The ID of the repo.
    #[serde(rename="repoId")]
    
    pub repo_id: Option<RepoId>,
    /// A revision ID.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for CloudRepoSourceContext {}


/// Command describes a step performed as part of the build pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    /// Command-line arguments used when executing this command.
    
    pub args: Option<Vec<String>>,
    /// Working directory (relative to project source root) used when running this command.
    
    pub dir: Option<String>,
    /// Environment variables set before running this command.
    
    pub env: Option<Vec<String>>,
    /// Optional unique identifier for this command, used in wait_for to reference this command as a dependency.
    
    pub id: Option<String>,
    /// Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`.
    
    pub name: Option<String>,
    /// The ID(s) of the command(s) that this command depends on.
    #[serde(rename="waitFor")]
    
    pub wait_for: Option<Vec<String>>,
}

impl client::Part for Command {}


/// An artifact that can be deployed in some runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployable {
    /// Required. Resource URI for the artifact being deployed.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<Vec<String>>,
}

impl client::Part for Deployable {}


/// The period during which some deployable was active in a runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// Address of the runtime element hosting this deployment.
    
    pub address: Option<String>,
    /// Configuration used to create this deployment.
    
    pub config: Option<String>,
    /// Required. Beginning of the lifetime of this deployment.
    #[serde(rename="deployTime")]
    
    pub deploy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Platform hosting this deployment.
    
    pub platform: Option<DeploymentPlatformEnum>,
    /// Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<Vec<String>>,
    /// End of the lifetime of this deployment.
    #[serde(rename="undeployTime")]
    
    pub undeploy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identity of the user that triggered this deployment.
    #[serde(rename="userEmail")]
    
    pub user_email: Option<String>,
}

impl client::Part for Deployment {}


/// Derived describes the derived image portion (Occurrence) of the DockerImage relationship. This image would be produced from a Dockerfile with FROM .
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Derived {
    /// Output only. This contains the base image URL for the derived image occurrence.
    #[serde(rename="baseResourceUrl")]
    
    pub base_resource_url: Option<String>,
    /// Output only. The number of layers by which this image differs from the associated image basis.
    
    pub distance: Option<i32>,
    /// Required. The fingerprint of the derived image.
    
    pub fingerprint: Option<Fingerprint>,
    /// This contains layer-specific metadata, if populated it has length "distance" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer.
    #[serde(rename="layerInfo")]
    
    pub layer_info: Option<Vec<Layer>>,
}

impl client::Part for Derived {}


/// Identifies all appearances of this vulnerability in the package for a specific distro/location. For example: glibc in cpe:/o:debian:debian_linux:8 for versions 2.1 - 2.2
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Detail {
    /// Required. The CPE URI in [cpe format](https://cpe.mitre.org/specification/) in which the vulnerability manifests. Examples include distro or storage location for vulnerable jar.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// A vendor-specific description of this note.
    
    pub description: Option<String>,
    /// The fix for this specific package version.
    #[serde(rename="fixedLocation")]
    
    pub fixed_location: Option<VulnerabilityLocation>,
    /// Whether this detail is obsolete. Occurrences are expected not to point to obsolete details.
    #[serde(rename="isObsolete")]
    
    pub is_obsolete: Option<bool>,
    /// The max version of the package in which the vulnerability exists.
    #[serde(rename="maxAffectedVersion")]
    
    pub max_affected_version: Option<Version>,
    /// The min version of the package in which the vulnerability exists.
    #[serde(rename="minAffectedVersion")]
    
    pub min_affected_version: Option<Version>,
    /// Required. The name of the package where the vulnerability was found.
    
    pub package: Option<String>,
    /// The type of package; whether native or non native(ruby gems, node.js packages etc).
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// The severity (eg: distro assigned severity) for this vulnerability.
    #[serde(rename="severityName")]
    
    pub severity_name: Option<String>,
    /// The source from which the information in this Detail was obtained.
    
    pub source: Option<String>,
    /// The time this information was last changed at the source. This is an upstream timestamp from the underlying information source - e.g. Ubuntu security tracker.
    #[serde(rename="sourceUpdateTime")]
    
    pub source_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the vendor of the product.
    
    pub vendor: Option<String>,
}

impl client::Part for Detail {}


/// Details of an attestation occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Details {
    /// Required. Attestation for the resource.
    
    pub attestation: Option<Attestation>,
}

impl client::Part for Details {}


/// Digest information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Digest {
    /// `SHA1`, `SHA512` etc.
    
    pub algo: Option<String>,
    /// Value of the digest.
    #[serde(rename="digestBytes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub digest_bytes: Option<Vec<u8>>,
}

impl client::Part for Digest {}


/// Provides information about the analysis status of a discovered resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Discovered {
    /// no description provided
    #[serde(rename="analysisCompleted")]
    
    pub analysis_completed: Option<AnalysisCompleted>,
    /// Indicates any errors encountered during analysis of a resource. There could be 0 or more of these errors.
    #[serde(rename="analysisError")]
    
    pub analysis_error: Option<Vec<Status>>,
    /// The status of discovery for the resource.
    #[serde(rename="analysisStatus")]
    
    pub analysis_status: Option<DiscoveredAnalysisStatusEnum>,
    /// When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API.
    #[serde(rename="analysisStatusError")]
    
    pub analysis_status_error: Option<Status>,
    /// Whether the resource is continuously analyzed.
    #[serde(rename="continuousAnalysis")]
    
    pub continuous_analysis: Option<DiscoveredContinuousAnalysisEnum>,
    /// The last time continuous analysis was done for this resource. Deprecated, do not use.
    #[serde(rename="lastAnalysisTime")]
    
    pub last_analysis_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Discovered {}


/// A note that indicates a type of analysis a provider would perform. This note exists in a provider's project. A `Discovery` occurrence is created in a consumer's project at the start of analysis.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Discovery {
    /// Required. Immutable. The kind of analysis that is handled by this discovery.
    #[serde(rename="analysisKind")]
    
    pub analysis_kind: Option<DiscoveryAnalysisKindEnum>,
}

impl client::Part for Discovery {}


/// This represents a particular channel of distribution for a given package. E.g., Debian's jessie-backports dpkg mirror.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    /// The CPU architecture for which packages in this distribution channel were built.
    
    pub architecture: Option<DistributionArchitectureEnum>,
    /// Required. The cpe_uri in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The distribution channel-specific description of this package.
    
    pub description: Option<String>,
    /// The latest available version of this package in this distribution channel.
    #[serde(rename="latestVersion")]
    
    pub latest_version: Option<Version>,
    /// A freeform string denoting the maintainer of this package.
    
    pub maintainer: Option<String>,
    /// The distribution channel-specific homepage for this package.
    
    pub url: Option<String>,
}

impl client::Part for Distribution {}


/// DocumentNote represents an SPDX Document Creation Information section: https://spdx.github.io/spdx-spec/v2.3/document-creation-information/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentNote {
    /// Compliance with the SPDX specification includes populating the SPDX fields therein with data related to such fields ("SPDX-Metadata")
    #[serde(rename="dataLicence")]
    
    pub data_licence: Option<String>,
    /// Provide a reference number that can be used to understand how to parse and interpret the rest of the file
    #[serde(rename="spdxVersion")]
    
    pub spdx_version: Option<String>,
}

impl client::Part for DocumentNote {}


/// DocumentOccurrence represents an SPDX Document Creation Information section: https://spdx.github.io/spdx-spec/v2.3/document-creation-information/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DocumentOccurrence {
    /// Identify when the SPDX file was originally created. The date is to be specified according to combined date and time in UTC format as specified in ISO 8601 standard
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A field for creators of the SPDX file to provide general comments about the creation of the SPDX file or any other relevant comment not included in the other fields
    #[serde(rename="creatorComment")]
    
    pub creator_comment: Option<String>,
    /// Identify who (or what, in the case of a tool) created the SPDX file. If the SPDX file was created by an individual, indicate the person's name
    
    pub creators: Option<Vec<String>>,
    /// A field for creators of the SPDX file content to provide comments to the consumers of the SPDX document
    #[serde(rename="documentComment")]
    
    pub document_comment: Option<String>,
    /// Identify any external SPDX documents referenced within this SPDX document
    #[serde(rename="externalDocumentRefs")]
    
    pub external_document_refs: Option<Vec<String>>,
    /// Identify the current SPDX document which may be referenced in relationships by other files, packages internally and documents externally
    
    pub id: Option<String>,
    /// A field for creators of the SPDX file to provide the version of the SPDX License List used when the SPDX file was created
    #[serde(rename="licenseListVersion")]
    
    pub license_list_version: Option<String>,
    /// Provide an SPDX document specific namespace as a unique absolute Uniform Resource Identifier (URI) as specified in RFC-3986, with the exception of the ‘#’ delimiter
    
    pub namespace: Option<String>,
    /// Identify name of this document as designated by creator
    
    pub title: Option<String>,
}

impl client::Part for DocumentOccurrence {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes delete projects](ProjectNoteDeleteCall) (response)
/// * [occurrences delete projects](ProjectOccurrenceDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// MUST match https://github.com/secure-systems-lab/dsse/blob/master/envelope.proto. An authenticated message of arbitrary type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Envelope {
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub payload: Option<Vec<u8>>,
    /// no description provided
    #[serde(rename="payloadType")]
    
    pub payload_type: Option<String>,
    /// no description provided
    
    pub signatures: Option<Vec<EnvelopeSignature>>,
}

impl client::Part for Envelope {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeSignature {
    /// no description provided
    
    pub keyid: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sig: Option<Vec<u8>>,
}

impl client::Part for EnvelopeSignature {}


/// Defines an object for the environment field in in-toto links. The suggested fields are "variables", "filesystem", and "workdir".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    /// no description provided
    #[serde(rename="customValues")]
    
    pub custom_values: Option<HashMap<String, String>>,
}

impl client::Part for Environment {}


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


/// An External Reference allows a Package to reference an external source of additional information, metadata, enumerations, asset identifiers, or downloadable content believed to be relevant to the Package
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalRef {
    /// An External Reference allows a Package to reference an external source of additional information, metadata, enumerations, asset identifiers, or downloadable content believed to be relevant to the Package
    
    pub category: Option<ExternalRefCategoryEnum>,
    /// Human-readable information about the purpose and target of the reference
    
    pub comment: Option<String>,
    /// The unique string with no spaces necessary to access the package-specific information, metadata, or content within the target location
    
    pub locator: Option<String>,
    /// Type of category (e.g. 'npm' for the PACKAGE_MANAGER category)
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ExternalRef {}


/// Container message for hashes of byte content of files, used in source messages to verify integrity of source input to the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileHashes {
    /// Required. Collection of file hashes.
    #[serde(rename="fileHash")]
    
    pub file_hash: Option<Vec<Hash>>,
}

impl client::Part for FileHashes {}


/// FileNote represents an SPDX File Information section: https://spdx.github.io/spdx-spec/4-file-information/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileNote {
    /// Provide a unique identifier to match analysis information on each specific file in a package
    
    pub checksum: Option<Vec<String>>,
    /// This field provides information about the type of file identified
    #[serde(rename="fileType")]
    
    pub file_type: Option<FileNoteFileTypeEnum>,
    /// Identify the full path and filename that corresponds to the file information in this section
    
    pub title: Option<String>,
}

impl client::Part for FileNote {}


/// FileOccurrence represents an SPDX File Information section: https://spdx.github.io/spdx-spec/4-file-information/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileOccurrence {
    /// This field provides a place for the SPDX data creator to record, at the file level, acknowledgements that may be needed to be communicated in some contexts
    
    pub attributions: Option<Vec<String>>,
    /// This field provides a place for the SPDX file creator to record any general comments about the file
    
    pub comment: Option<String>,
    /// This field provides a place for the SPDX file creator to record file contributors
    
    pub contributors: Option<Vec<String>>,
    /// Identify the copyright holder of the file, as well as any dates present
    
    pub copyright: Option<String>,
    /// This field contains the license information actually found in the file, if any
    #[serde(rename="filesLicenseInfo")]
    
    pub files_license_info: Option<Vec<String>>,
    /// Uniquely identify any element in an SPDX document which may be referenced by other elements
    
    pub id: Option<String>,
    /// This field contains the license the SPDX file creator has concluded as governing the file or alternative values if the governing license cannot be determined
    #[serde(rename="licenseConcluded")]
    
    pub license_concluded: Option<License>,
    /// This field provides a place for the SPDX file creator to record license notices or other such related notices found in the file
    
    pub notice: Option<String>,
}

impl client::Part for FileOccurrence {}


/// A set of properties that uniquely identify a given Docker image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Fingerprint {
    /// Required. The layer ID of the final layer in the Docker image's v1 representation.
    #[serde(rename="v1Name")]
    
    pub v1_name: Option<String>,
    /// Required. The ordered list of v2 blobs that represent a given image.
    #[serde(rename="v2Blob")]
    
    pub v2_blob: Option<Vec<String>>,
    /// Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + " " + v2_name[N+1]) Only the name of the final blob is kept.
    #[serde(rename="v2Name")]
    
    pub v2_name: Option<String>,
}

impl client::Part for Fingerprint {}


/// Per resource and severity counts of fixable and total vulnerabilities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FixableTotalByDigest {
    /// The number of fixable vulnerabilities associated with this resource.
    #[serde(rename="fixableCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub fixable_count: Option<i64>,
    /// The affected resource.
    
    pub resource: Option<Resource>,
    /// The severity for this count. SEVERITY_UNSPECIFIED indicates total across all severities.
    
    pub severity: Option<FixableTotalByDigestSeverityEnum>,
    /// The total number of vulnerabilities associated with this resource.
    #[serde(rename="totalCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_count: Option<i64>,
}

impl client::Part for FixableTotalByDigest {}


/// An attestation wrapper that uses the Grafeas `Signature` message. This attestation must define the `serialized_payload` that the `signatures` verify and any metadata necessary to interpret that plaintext. The signatures should always be over the `serialized_payload` bytestring.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenericSignedAttestation {
    /// Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema).
    #[serde(rename="contentType")]
    
    pub content_type: Option<GenericSignedAttestationContentTypeEnum>,
    /// The serialized payload that is verified by one or more `signatures`. The encoding and semantic meaning of this payload must match what is set in `content_type`.
    #[serde(rename="serializedPayload")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub serialized_payload: Option<Vec<u8>>,
    /// One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification.
    
    pub signatures: Option<Vec<Signature>>,
}

impl client::Part for GenericSignedAttestation {}


/// A SourceContext referring to a Gerrit project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GerritSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    
    pub alias_context: Option<AliasContext>,
    /// The full project name within the host. Projects may be nested, so "project/subproject" is a valid project name. The "repo name" is the hostURI/project.
    #[serde(rename="gerritProject")]
    
    pub gerrit_project: Option<String>,
    /// The URI of a running Gerrit instance.
    #[serde(rename="hostUri")]
    
    pub host_uri: Option<String>,
    /// A revision (commit) ID.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for GerritSourceContext {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes get iam policy projects](ProjectNoteGetIamPolicyCall) (request)
/// * [occurrences get iam policy projects](ProjectOccurrenceGetIamPolicyCall) (request)
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


/// A GitSourceContext denotes a particular revision in a third party Git repository (e.g., GitHub).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitSourceContext {
    /// Git commit hash.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Git repository URL.
    
    pub url: Option<String>,
}

impl client::Part for GitSourceContext {}


/// Details of a build occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1BuildDetails {
    /// Required. The actual provenance for the build.
    
    pub provenance: Option<BuildProvenance>,
    /// Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes.
    #[serde(rename="provenanceBytes")]
    
    pub provenance_bytes: Option<String>,
}

impl client::Part for GrafeasV1beta1BuildDetails {}


/// Details of a deployment occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1DeploymentDetails {
    /// Required. Deployment history for the resource.
    
    pub deployment: Option<Deployment>,
}

impl client::Part for GrafeasV1beta1DeploymentDetails {}


/// Details of a discovery occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1DiscoveryDetails {
    /// Required. Analysis status for the discovered resource.
    
    pub discovered: Option<Discovered>,
}

impl client::Part for GrafeasV1beta1DiscoveryDetails {}


/// Details of an image occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1ImageDetails {
    /// Required. Immutable. The child image derived from the base image.
    #[serde(rename="derivedImage")]
    
    pub derived_image: Option<Derived>,
}

impl client::Part for GrafeasV1beta1ImageDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1IntotoArtifact {
    /// no description provided
    
    pub hashes: Option<ArtifactHashes>,
    /// no description provided
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
}

impl client::Part for GrafeasV1beta1IntotoArtifact {}


/// This corresponds to a signed in-toto link - it is made up of one or more signatures and the in-toto link itself. This is used for occurrences of a Grafeas in-toto note.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1IntotoDetails {
    /// no description provided
    
    pub signatures: Option<Vec<GrafeasV1beta1IntotoSignature>>,
    /// no description provided
    
    pub signed: Option<Link>,
}

impl client::Part for GrafeasV1beta1IntotoDetails {}


/// A signature object consists of the KeyID used and the signature itself.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1IntotoSignature {
    /// no description provided
    
    pub keyid: Option<String>,
    /// no description provided
    
    pub sig: Option<String>,
}

impl client::Part for GrafeasV1beta1IntotoSignature {}


/// Details of a package occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1PackageDetails {
    /// Required. Where the package was installed.
    
    pub installation: Option<Installation>,
}

impl client::Part for GrafeasV1beta1PackageDetails {}


/// Details of a vulnerability Occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1beta1VulnerabilityDetails {
    /// Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0-10 where 0 indicates low severity and 10 indicates high severity.
    #[serde(rename="cvssScore")]
    
    pub cvss_score: Option<f32>,
    /// Output only. CVSS version used to populate cvss_score and severity.
    #[serde(rename="cvssVersion")]
    
    pub cvss_version: Option<GrafeasV1beta1VulnerabilityDetailCvssVersionEnum>,
    /// The distro assigned severity for this vulnerability when it is available, and note provider assigned severity when distro has not yet assigned a severity for this vulnerability. When there are multiple PackageIssues for this vulnerability, they can have different effective severities because some might be provided by the distro while others are provided by the language ecosystem for a language pack. For this reason, it is advised to use the effective severity on the PackageIssue level. In the case where multiple PackageIssues have differing effective severities, this field should be the highest severity for any of the PackageIssues.
    #[serde(rename="effectiveSeverity")]
    
    pub effective_severity: Option<GrafeasV1beta1VulnerabilityDetailEffectiveSeverityEnum>,
    /// Output only. A detailed description of this vulnerability.
    #[serde(rename="longDescription")]
    
    pub long_description: Option<String>,
    /// Required. The set of affected locations and their fixes (if available) within the associated resource.
    #[serde(rename="packageIssue")]
    
    pub package_issue: Option<Vec<PackageIssue>>,
    /// Output only. URLs related to this vulnerability.
    #[serde(rename="relatedUrls")]
    
    pub related_urls: Option<Vec<RelatedUrl>>,
    /// Output only. The note provider assigned Severity of the vulnerability.
    
    pub severity: Option<GrafeasV1beta1VulnerabilityDetailSeverityEnum>,
    /// Output only. A one sentence description of this vulnerability.
    #[serde(rename="shortDescription")]
    
    pub short_description: Option<String>,
    /// The type of package; whether native or non native(ruby gems, node.js packages etc)
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GrafeasV1beta1VulnerabilityDetails {}


/// Container message for hash values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hash {
    /// Required. The type of hash that was performed.
    #[serde(rename="type")]
    
    pub type_: Option<HashTypeEnum>,
    /// Required. The hash value.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl client::Part for Hash {}


/// This submessage provides human-readable hints about the purpose of the authority. Because the name of a note acts as its resource reference, it is important to disambiguate the canonical name of the Note (which might be a UUID for security purposes) from "readable" names more suitable for debug output. Note that these hints should not be used to look up authorities in security sensitive contexts, such as when looking up attestations to verify.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hint {
    /// Required. The human readable name of this attestation authority, for example "qa".
    #[serde(rename="humanReadableName")]
    
    pub human_readable_name: Option<String>,
}

impl client::Part for Hint {}


/// This contains the fields corresponding to the definition of a software supply chain step in an in-toto layout. This information goes into a Grafeas note.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InToto {
    /// This field contains the expected command used to perform the step.
    #[serde(rename="expectedCommand")]
    
    pub expected_command: Option<Vec<String>>,
    /// The following fields contain in-toto artifact rules identifying the artifacts that enter this supply chain step, and exit the supply chain step, i.e. materials and products of the step.
    #[serde(rename="expectedMaterials")]
    
    pub expected_materials: Option<Vec<ArtifactRule>>,
    /// no description provided
    #[serde(rename="expectedProducts")]
    
    pub expected_products: Option<Vec<ArtifactRule>>,
    /// This field contains the public keys that can be used to verify the signatures on the step metadata.
    #[serde(rename="signingKeys")]
    
    pub signing_keys: Option<Vec<SigningKey>>,
    /// This field identifies the name of the step in the supply chain.
    #[serde(rename="stepName")]
    
    pub step_name: Option<String>,
    /// This field contains a value that indicates the minimum number of keys that need to be used to sign the step's in-toto link.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub threshold: Option<i64>,
}

impl client::Part for InToto {}


/// This represents how a particular software package may be installed on a system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Installation {
    /// Output only. The CPU architecture for which packages in this distribution channel were built. Architecture will be blank for language packages.
    
    pub architecture: Option<InstallationArchitectureEnum>,
    /// Output only. The cpe_uri in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package. The cpe_uri will be blank for language packages.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// Licenses that have been declared by the authors of the package.
    
    pub license: Option<License>,
    /// All of the places within the filesystem versions of this package have been found.
    
    pub location: Option<Vec<Location>>,
    /// Required. Output only. The name of the installed package.
    
    pub name: Option<String>,
    /// Output only. The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.).
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// Output only. The version of the package.
    
    pub version: Option<Version>,
}

impl client::Part for Installation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// The KB name (generally of the form KB[0-9]+ i.e. KB123456).
    
    pub name: Option<String>,
    /// A link to the KB in the Windows update catalog - https://www.catalog.update.microsoft.com/
    
    pub url: Option<String>,
}

impl client::Part for KnowledgeBase {}


/// Layer holds metadata specific to a layer of a Docker image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    /// The recovered arguments to the Dockerfile directive.
    
    pub arguments: Option<String>,
    /// Required. The recovered Dockerfile directive used to construct this layer.
    
    pub directive: Option<LayerDirectiveEnum>,
}

impl client::Part for Layer {}


/// License information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct License {
    /// Comments
    
    pub comments: Option<String>,
    /// Often a single license can be used to represent the licensing terms. Sometimes it is necessary to include a choice of one or more licenses or some combination of license identifiers. Examples: "LGPL-2.1-only OR MIT", "LGPL-2.1-only AND MIT", "GPL-2.0-or-later WITH Bison-exception-2.2".
    
    pub expression: Option<String>,
}

impl client::Part for License {}


/// This corresponds to an in-toto link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// ByProducts are data generated as part of a software supply chain step, but are not the actual result of the step.
    
    pub byproducts: Option<ByProducts>,
    /// This field contains the full command executed for the step. This can also be empty if links are generated for operations that aren't directly mapped to a specific command. Each term in the command is an independent string in the list. An example of a command in the in-toto metadata field is: "command": ["git", "clone", "https://github.com/in-toto/demo-project.git"]
    
    pub command: Option<Vec<String>>,
    /// This is a field that can be used to capture information about the environment. It is suggested for this field to contain information that details environment variables, filesystem information, and the present working directory. The recommended structure of this field is: "environment": { "custom_values": { "variables": "", "filesystem": "", "workdir": "", "": "..." } }
    
    pub environment: Option<Environment>,
    /// Materials are the supply chain artifacts that go into the step and are used for the operation performed. The key of the map is the path of the artifact and the structure contains the recorded hash information. An example is: "materials": [ { "resource_uri": "foo/bar", "hashes": { "sha256": "ebebf...", : } } ]
    
    pub materials: Option<Vec<GrafeasV1beta1IntotoArtifact>>,
    /// Products are the supply chain artifacts generated as a result of the step. The structure is identical to that of materials.
    
    pub products: Option<Vec<GrafeasV1beta1IntotoArtifact>>,
}

impl client::Part for Link {}


/// Response for listing occurrences for a note.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes occurrences list projects](ProjectNoteOccurrenceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNoteOccurrencesResponse {
    /// Token to provide to skip to a particular spot in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The occurrences attached to the specified note.
    
    pub occurrences: Option<Vec<Occurrence>>,
}

impl client::ResponseResult for ListNoteOccurrencesResponse {}


/// Response for listing notes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes list projects](ProjectNoteListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNotesResponse {
    /// The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The notes requested.
    
    pub notes: Option<Vec<Note>>,
}

impl client::ResponseResult for ListNotesResponse {}


/// Response for listing occurrences.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [occurrences list projects](ProjectOccurrenceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOccurrencesResponse {
    /// The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The occurrences requested.
    
    pub occurrences: Option<Vec<Occurrence>>,
}

impl client::ResponseResult for ListOccurrencesResponse {}


/// An occurrence of a particular package installation found within a system's filesystem. E.g., glibc was found in `/var/lib/dpkg/status`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Deprecated. The CPE URI in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The path from which we gathered that this package/version is installed.
    
    pub path: Option<String>,
    /// Deprecated. The version installed at this location.
    
    pub version: Option<Version>,
}

impl client::Part for Location {}


/// A type of analysis that can be done for a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes create projects](ProjectNoteCreateCall) (request|response)
/// * [notes get projects](ProjectNoteGetCall) (response)
/// * [notes patch projects](ProjectNotePatchCall) (request|response)
/// * [occurrences get notes projects](ProjectOccurrenceGetNoteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    /// A note describing an attestation role.
    #[serde(rename="attestationAuthority")]
    
    pub attestation_authority: Option<Authority>,
    /// A note describing a base image.
    #[serde(rename="baseImage")]
    
    pub base_image: Option<Basis>,
    /// A note describing build provenance for a verifiable build.
    
    pub build: Option<Build>,
    /// Output only. The time this note was created. This field can be used as a filter in list requests.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A note describing something that can be deployed.
    
    pub deployable: Option<Deployable>,
    /// A note describing the initial analysis of a resource.
    
    pub discovery: Option<Discovery>,
    /// Time of expiration for this note. Empty if note does not expire.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A note describing an in-toto link.
    
    pub intoto: Option<InToto>,
    /// Output only. The type of analysis. This field can be used as a filter in list requests.
    
    pub kind: Option<NoteKindEnum>,
    /// A detailed description of this note.
    #[serde(rename="longDescription")]
    
    pub long_description: Option<String>,
    /// Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    
    pub name: Option<String>,
    /// A note describing a package hosted by various package managers.
    
    pub package: Option<Package>,
    /// Other notes related to this note.
    #[serde(rename="relatedNoteNames")]
    
    pub related_note_names: Option<Vec<String>>,
    /// URLs associated with this note.
    #[serde(rename="relatedUrl")]
    
    pub related_url: Option<Vec<RelatedUrl>>,
    /// A note describing a software bill of materials.
    
    pub sbom: Option<DocumentNote>,
    /// A one sentence description of this note.
    #[serde(rename="shortDescription")]
    
    pub short_description: Option<String>,
    /// A note describing an SPDX File.
    #[serde(rename="spdxFile")]
    
    pub spdx_file: Option<FileNote>,
    /// A note describing an SPDX Package.
    #[serde(rename="spdxPackage")]
    
    pub spdx_package: Option<PackageInfoNote>,
    /// A note describing an SPDX File.
    #[serde(rename="spdxRelationship")]
    
    pub spdx_relationship: Option<RelationshipNote>,
    /// Output only. The time this note was last updated. This field can be used as a filter in list requests.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A note describing a package vulnerability.
    
    pub vulnerability: Option<Vulnerability>,
}

impl client::RequestValue for Note {}
impl client::ResponseResult for Note {}


/// An instance of an analysis type that has been found on a resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [occurrences create projects](ProjectOccurrenceCreateCall) (request|response)
/// * [occurrences get projects](ProjectOccurrenceGetCall) (response)
/// * [occurrences patch projects](ProjectOccurrencePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Occurrence {
    /// Describes an attestation of an artifact.
    
    pub attestation: Option<Details>,
    /// Describes a verifiable build.
    
    pub build: Option<GrafeasV1beta1BuildDetails>,
    /// Output only. The time this occurrence was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes the deployment of an artifact on a runtime.
    
    pub deployment: Option<GrafeasV1beta1DeploymentDetails>,
    /// Describes how this resource derives from the basis in the associated note.
    #[serde(rename="derivedImage")]
    
    pub derived_image: Option<GrafeasV1beta1ImageDetails>,
    /// Describes when a resource was discovered.
    
    pub discovered: Option<GrafeasV1beta1DiscoveryDetails>,
    /// https://github.com/secure-systems-lab/dsse
    
    pub envelope: Option<Envelope>,
    /// Describes the installation of a package on the linked resource.
    
    pub installation: Option<GrafeasV1beta1PackageDetails>,
    /// Describes a specific in-toto link.
    
    pub intoto: Option<GrafeasV1beta1IntotoDetails>,
    /// Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests.
    
    pub kind: Option<OccurrenceKindEnum>,
    /// Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    
    pub name: Option<String>,
    /// Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests.
    #[serde(rename="noteName")]
    
    pub note_name: Option<String>,
    /// A description of actions that can be taken to remedy the note.
    
    pub remediation: Option<String>,
    /// Required. Immutable. The resource for which the occurrence applies.
    
    pub resource: Option<Resource>,
    /// Describes a specific software bill of materials document.
    
    pub sbom: Option<DocumentOccurrence>,
    /// Describes a specific SPDX File.
    #[serde(rename="spdxFile")]
    
    pub spdx_file: Option<FileOccurrence>,
    /// Describes a specific SPDX Package.
    #[serde(rename="spdxPackage")]
    
    pub spdx_package: Option<PackageInfoOccurrence>,
    /// Describes a specific SPDX Relationship.
    #[serde(rename="spdxRelationship")]
    
    pub spdx_relationship: Option<RelationshipOccurrence>,
    /// Output only. The time this occurrence was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes a security vulnerability.
    
    pub vulnerability: Option<GrafeasV1beta1VulnerabilityDetails>,
}

impl client::RequestValue for Occurrence {}
impl client::ResponseResult for Occurrence {}


/// Package represents a particular package version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    /// The CPU architecture for which packages in this distribution channel were built. Architecture will be blank for language packages.
    
    pub architecture: Option<PackageArchitectureEnum>,
    /// The cpe_uri in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package. The cpe_uri will be blank for language packages.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The description of this package.
    
    pub description: Option<String>,
    /// Hash value, typically a file digest, that allows unique identification a specific package.
    
    pub digest: Option<Vec<Digest>>,
    /// The various channels by which a package is distributed.
    
    pub distribution: Option<Vec<Distribution>>,
    /// Licenses that have been declared by the authors of the package.
    
    pub license: Option<License>,
    /// A freeform text denoting the maintainer of this package.
    
    pub maintainer: Option<String>,
    /// Required. Immutable. The name of the package.
    
    pub name: Option<String>,
    /// The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.).
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// The homepage for this package.
    
    pub url: Option<String>,
    /// The version of the package.
    
    pub version: Option<Version>,
}

impl client::Part for Package {}


/// PackageInfoNote represents an SPDX Package Information section: https://spdx.github.io/spdx-spec/3-package-information/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageInfoNote {
    /// Indicates whether the file content of this package has been available for or subjected to analysis when creating the SPDX document
    
    pub analyzed: Option<bool>,
    /// A place for the SPDX data creator to record, at the package level, acknowledgements that may be needed to be communicated in some contexts
    
    pub attribution: Option<String>,
    /// Provide an independently reproducible mechanism that permits unique identification of a specific package that correlates to the data in this SPDX file
    
    pub checksum: Option<String>,
    /// Identify the copyright holders of the package, as well as any dates present
    
    pub copyright: Option<String>,
    /// A more detailed description of the package
    #[serde(rename="detailedDescription")]
    
    pub detailed_description: Option<String>,
    /// This section identifies the download Universal Resource Locator (URL), or a specific location within a version control system (VCS) for the package at the time that the SPDX file was created
    #[serde(rename="downloadLocation")]
    
    pub download_location: Option<String>,
    /// ExternalRef
    #[serde(rename="externalRefs")]
    
    pub external_refs: Option<Vec<ExternalRef>>,
    /// Contain the license the SPDX file creator has concluded as governing the This field is to contain a list of all licenses found in the package. The relationship between licenses (i.e., conjunctive, disjunctive) is not specified in this field – it is simply a listing of all licenses found
    #[serde(rename="filesLicenseInfo")]
    
    pub files_license_info: Option<Vec<String>>,
    /// Provide a place for the SPDX file creator to record a web site that serves as the package's home page
    #[serde(rename="homePage")]
    
    pub home_page: Option<String>,
    /// List the licenses that have been declared by the authors of the package
    #[serde(rename="licenseDeclared")]
    
    pub license_declared: Option<License>,
    /// If the package identified in the SPDX file originated from a different person or organization than identified as Package Supplier, this field identifies from where or whom the package originally came
    
    pub originator: Option<String>,
    /// The type of package: OS, MAVEN, GO, GO_STDLIB, etc.
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// A short description of the package
    #[serde(rename="summaryDescription")]
    
    pub summary_description: Option<String>,
    /// Identify the actual distribution source for the package/directory identified in the SPDX file
    
    pub supplier: Option<String>,
    /// Identify the full name of the package as given by the Package Originator
    
    pub title: Option<String>,
    /// This field provides an independently reproducible mechanism identifying specific contents of a package based on the actual files (except the SPDX file itself, if it is included in the package) that make up each package and that correlates to the data in this SPDX file
    #[serde(rename="verificationCode")]
    
    pub verification_code: Option<String>,
    /// Identify the version of the package
    
    pub version: Option<String>,
}

impl client::Part for PackageInfoNote {}


/// PackageInfoOccurrence represents an SPDX Package Information section: https://spdx.github.io/spdx-spec/3-package-information/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageInfoOccurrence {
    /// A place for the SPDX file creator to record any general comments about the package being described
    
    pub comment: Option<String>,
    /// Provide the actual file name of the package, or path of the directory being treated as a package
    
    pub filename: Option<String>,
    /// Output only. Provide a place for the SPDX file creator to record a web site that serves as the package's home page
    #[serde(rename="homePage")]
    
    pub home_page: Option<String>,
    /// Uniquely identify any element in an SPDX document which may be referenced by other elements
    
    pub id: Option<String>,
    /// package or alternative values, if the governing license cannot be determined
    #[serde(rename="licenseConcluded")]
    
    pub license_concluded: Option<License>,
    /// Output only. The type of package: OS, MAVEN, GO, GO_STDLIB, etc.
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// Provide a place for the SPDX file creator to record any relevant background information or additional comments about the origin of the package
    #[serde(rename="sourceInfo")]
    
    pub source_info: Option<String>,
    /// Output only. A short description of the package
    #[serde(rename="summaryDescription")]
    
    pub summary_description: Option<String>,
    /// Output only. Identify the full name of the package as given by the Package Originator
    
    pub title: Option<String>,
    /// Output only. Identify the version of the package
    
    pub version: Option<String>,
}

impl client::Part for PackageInfoOccurrence {}


/// This message wraps a location affected by a vulnerability and its associated fix (if one is available).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageIssue {
    /// Required. The location of the vulnerability.
    #[serde(rename="affectedLocation")]
    
    pub affected_location: Option<VulnerabilityLocation>,
    /// Output only. The distro or language system assigned severity for this vulnerability when that is available and note provider assigned severity when it is not available.
    #[serde(rename="effectiveSeverity")]
    
    pub effective_severity: Option<PackageIssueEffectiveSeverityEnum>,
    /// The location of the available fix for vulnerability.
    #[serde(rename="fixedLocation")]
    
    pub fixed_location: Option<VulnerabilityLocation>,
    /// The type of package (e.g. OS, MAVEN, GO).
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// Deprecated, use Details.effective_severity instead The severity (e.g., distro assigned severity) for this vulnerability.
    #[serde(rename="severityName")]
    
    pub severity_name: Option<String>,
}

impl client::Part for PackageIssue {}


/// An attestation wrapper with a PGP-compatible signature. This message only supports `ATTACHED` signatures, where the payload that is signed is included alongside the signature itself in the same file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PgpSignedAttestation {
    /// Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema).
    #[serde(rename="contentType")]
    
    pub content_type: Option<PgpSignedAttestationContentTypeEnum>,
    /// The cryptographic fingerprint of the key used to generate the signature, as output by, e.g. `gpg --list-keys`. This should be the version 4, full 160-bit fingerprint, expressed as a 40 character hexadecimal string. See https://tools.ietf.org/html/rfc4880#section-12.2 for details. Implementations may choose to acknowledge "LONG", "SHORT", or other abbreviated key IDs, but only the full fingerprint is guaranteed to work. In gpg, the full fingerprint can be retrieved from the `fpr` field returned when calling --list-keys with --with-colons. For example: ``` gpg --with-colons --with-fingerprint --force-v4-certs \ --list-keys attester@example.com tru::1:1513631572:0:3:1:5 pub:...... fpr:::::::::24FF6481B76AC91E66A00AC657A93A81EF3AE6FB: ``` Above, the fingerprint is `24FF6481B76AC91E66A00AC657A93A81EF3AE6FB`.
    #[serde(rename="pgpKeyId")]
    
    pub pgp_key_id: Option<String>,
    /// Required. The raw content of the signature, as output by GNU Privacy Guard (GPG) or equivalent. Since this message only supports attached signatures, the payload that was signed must be attached. While the signature format supported is dependent on the verification implementation, currently only ASCII-armored (`--armor` to gpg), non-clearsigned (`--sign` rather than `--clearsign` to gpg) are supported. Concretely, `gpg --sign --armor --output=signature.gpg payload.json` will create the signature content expected in this field in `signature.gpg` for the `payload.json` attestation payload.
    
    pub signature: Option<String>,
}

impl client::Part for PgpSignedAttestation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes get iam policy projects](ProjectNoteGetIamPolicyCall) (response)
/// * [notes set iam policy projects](ProjectNoteSetIamPolicyCall) (response)
/// * [occurrences get iam policy projects](ProjectOccurrenceGetIamPolicyCall) (response)
/// * [occurrences set iam policy projects](ProjectOccurrenceSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Selects a repo using a Google Cloud Platform project ID (e.g., winged-cargo-31) and a repo name within that project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The name of the repo. Leave empty for the default repo.
    #[serde(rename="repoName")]
    
    pub repo_name: Option<String>,
}

impl client::Part for ProjectRepoId {}


/// Metadata for any related URL information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedUrl {
    /// Label to describe usage of the URL.
    
    pub label: Option<String>,
    /// Specific URL associated with the resource.
    
    pub url: Option<String>,
}

impl client::Part for RelatedUrl {}


/// RelationshipNote represents an SPDX Relationship section: https://spdx.github.io/spdx-spec/7-relationships-between-SPDX-elements/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipNote {
    /// The type of relationship between the source and target SPDX elements
    #[serde(rename="type")]
    
    pub type_: Option<RelationshipNoteTypeEnum>,
}

impl client::Part for RelationshipNote {}


/// RelationshipOccurrence represents an SPDX Relationship section: https://spdx.github.io/spdx-spec/7-relationships-between-SPDX-elements/
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipOccurrence {
    /// A place for the SPDX file creator to record any general comments about the relationship
    
    pub comment: Option<String>,
    /// Also referred to as SPDXRef-A The source SPDX element (file, package, etc)
    
    pub source: Option<String>,
    /// Also referred to as SPDXRef-B The target SPDC element (file, package, etc) In cases where there are "known unknowns", the use of the keyword NOASSERTION can be used The keywords NONE can be used to indicate that an SPDX element (package/file/snippet) has no other elements connected by some relationship to it
    
    pub target: Option<String>,
    /// Output only. The type of relationship between the source and target SPDX elements
    #[serde(rename="type")]
    
    pub type_: Option<RelationshipOccurrenceTypeEnum>,
}

impl client::Part for RelationshipOccurrence {}


/// A unique identifier for a Cloud Repo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepoId {
    /// A combination of a project ID and a repo name.
    #[serde(rename="projectRepoId")]
    
    pub project_repo_id: Option<ProjectRepoId>,
    /// A server-assigned, globally unique identifier.
    
    pub uid: Option<String>,
}

impl client::Part for RepoId {}


/// An entity that can have metadata. For example, a Docker image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// Deprecated, do not use. Use uri instead. The hash of the resource content. For example, the Docker digest.
    #[serde(rename="contentHash")]
    
    pub content_hash: Option<Hash>,
    /// Deprecated, do not use. Use uri instead. The name of the resource. For example, the name of a Docker image - "Debian".
    
    pub name: Option<String>,
    /// Required. The unique URI of the resource. For example, `https://gcr.io/project/image@sha256:foo` for a Docker image.
    
    pub uri: Option<String>,
}

impl client::Part for Resource {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [notes set iam policy projects](ProjectNoteSetIamPolicyCall) (request)
/// * [occurrences set iam policy projects](ProjectOccurrenceSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Verifiers (e.g. Kritis implementations) MUST verify signatures with respect to the trust anchors defined in policy (e.g. a Kritis policy). Typically this means that the verifier has been configured with a map from `public_key_id` to public key material (and any required parameters, e.g. signing algorithm). In particular, verification implementations MUST NOT treat the signature `public_key_id` as anything more than a key lookup hint. The `public_key_id` DOES NOT validate or authenticate a public key; it only provides a mechanism for quickly selecting a public key ALREADY CONFIGURED on the verifier through a trusted channel. Verification implementations MUST reject signatures in any of the following circumstances: * The `public_key_id` is not recognized by the verifier. * The public key that `public_key_id` refers to does not verify the signature with respect to the payload. The `signature` contents SHOULD NOT be "attached" (where the payload is included with the serialized `signature` bytes). Verifiers MUST ignore any "attached" payload and only verify signatures with respect to explicitly provided payload (e.g. a `payload` field on the proto message that holds this Signature, or the canonical serialization of the proto message that holds this signature).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    /// The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * "openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA" See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * "ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU" * "nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5"
    #[serde(rename="publicKeyId")]
    
    pub public_key_id: Option<String>,
    /// The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub signature: Option<Vec<u8>>,
}

impl client::Part for Signature {}


/// This defines the format used to record keys used in the software supply chain. An in-toto link is attested using one or more keys defined in the in-toto layout. An example of this is: { "key_id": "776a00e29f3559e0141b3b096f696abc6cfb0c657ab40f441132b345b0...", "key_type": "rsa", "public_key_value": "-----BEGIN PUBLIC KEY-----\nMIIBojANBgkqhkiG9w0B...", "key_scheme": "rsassa-pss-sha256" } The format for in-toto's key definition can be found in section 4.2 of the in-toto specification.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SigningKey {
    /// key_id is an identifier for the signing key.
    #[serde(rename="keyId")]
    
    pub key_id: Option<String>,
    /// This field contains the corresponding signature scheme. Eg: "rsassa-pss-sha256".
    #[serde(rename="keyScheme")]
    
    pub key_scheme: Option<String>,
    /// This field identifies the specific signing method. Eg: "rsa", "ed25519", and "ecdsa".
    #[serde(rename="keyType")]
    
    pub key_type: Option<String>,
    /// This field contains the actual public key.
    #[serde(rename="publicKeyValue")]
    
    pub public_key_value: Option<String>,
}

impl client::Part for SigningKey {}


/// Source describes the location of the source used for the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field.
    #[serde(rename="additionalContexts")]
    
    pub additional_contexts: Option<Vec<SourceContext>>,
    /// If provided, the input binary artifacts for the build came from this location.
    #[serde(rename="artifactStorageSourceUri")]
    
    pub artifact_storage_source_uri: Option<String>,
    /// If provided, the source code used for the build came from this location.
    
    pub context: Option<SourceContext>,
    /// Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file.
    #[serde(rename="fileHashes")]
    
    pub file_hashes: Option<HashMap<String, FileHashes>>,
}

impl client::Part for Source {}


/// A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceContext {
    /// A SourceContext referring to a revision in a Google Cloud Source Repo.
    #[serde(rename="cloudRepo")]
    
    pub cloud_repo: Option<CloudRepoSourceContext>,
    /// A SourceContext referring to a Gerrit project.
    
    pub gerrit: Option<GerritSourceContext>,
    /// A SourceContext referring to any third party Git repo (e.g., GitHub).
    
    pub git: Option<GitSourceContext>,
    /// Labels with user defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for SourceContext {}


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
/// * [notes test iam permissions projects](ProjectNoteTestIamPermissionCall) (request)
/// * [occurrences test iam permissions projects](ProjectOccurrenceTestIamPermissionCall) (request)
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
/// * [notes test iam permissions projects](ProjectNoteTestIamPermissionCall) (response)
/// * [occurrences test iam permissions projects](ProjectOccurrenceTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Version contains structured information about the version of a package.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// Used to correct mistakes in the version numbering scheme.
    
    pub epoch: Option<i32>,
    /// Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range.
    
    pub inclusive: Option<bool>,
    /// Required. Distinguishes between sentinel MIN/MAX versions and normal versions.
    
    pub kind: Option<VersionKindEnum>,
    /// Required only when version kind is NORMAL. The main part of the version name.
    
    pub name: Option<String>,
    /// The iteration of the package build from the above version.
    
    pub revision: Option<String>,
}

impl client::Part for Version {}


/// Vulnerability provides metadata about a security vulnerability in a Note.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    /// The CVSS score for this vulnerability.
    #[serde(rename="cvssScore")]
    
    pub cvss_score: Option<f32>,
    /// The full description of the CVSS for version 2.
    #[serde(rename="cvssV2")]
    
    pub cvss_v2: Option<CVSS>,
    /// The full description of the CVSS for version 3.
    #[serde(rename="cvssV3")]
    
    pub cvss_v3: Option<CVSSv3>,
    /// CVSS version used to populate cvss_score and severity.
    #[serde(rename="cvssVersion")]
    
    pub cvss_version: Option<VulnerabilityCvssVersionEnum>,
    /// A list of CWE for this vulnerability. For details, see: https://cwe.mitre.org/index.html
    
    pub cwe: Option<Vec<String>>,
    /// All information about the package to specifically identify this vulnerability. One entry per (version range and cpe_uri) the package vulnerability has manifested in.
    
    pub details: Option<Vec<Detail>>,
    /// Note provider assigned impact of the vulnerability.
    
    pub severity: Option<VulnerabilitySeverityEnum>,
    /// The time this information was last changed at the source. This is an upstream timestamp from the underlying information source - e.g. Ubuntu security tracker.
    #[serde(rename="sourceUpdateTime")]
    
    pub source_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Windows details get their own format because the information format and model don't match a normal detail. Specifically Windows updates are done as patches, thus Windows vulnerabilities really are a missing package, rather than a package being at an incorrect version.
    #[serde(rename="windowsDetails")]
    
    pub windows_details: Option<Vec<WindowsDetail>>,
}

impl client::Part for Vulnerability {}


/// The location of the vulnerability.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VulnerabilityLocation {
    /// Required. The CPE URI in [cpe format](https://cpe.mitre.org/specification/) format. Examples include distro or storage location for vulnerable jar.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// Required. The package being described.
    
    pub package: Option<String>,
    /// Required. The version of the package being described.
    
    pub version: Option<Version>,
}

impl client::Part for VulnerabilityLocation {}


/// A summary of how many vulnerability occurrences there are per resource and severity type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [occurrences get vulnerability summary projects](ProjectOccurrenceGetVulnerabilitySummaryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VulnerabilityOccurrencesSummary {
    /// A listing by resource of the number of fixable and total vulnerabilities.
    
    pub counts: Option<Vec<FixableTotalByDigest>>,
}

impl client::ResponseResult for VulnerabilityOccurrencesSummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsDetail {
    /// Required. The CPE URI in [cpe format](https://cpe.mitre.org/specification/) in which the vulnerability manifests. Examples include distro or storage location for vulnerable jar.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The description of the vulnerability.
    
    pub description: Option<String>,
    /// Required. The names of the KBs which have hotfixes to mitigate this vulnerability. Note that there may be multiple hotfixes (and thus multiple KBs) that mitigate a given vulnerability. Currently any listed kb's presence is considered a fix.
    #[serde(rename="fixingKbs")]
    
    pub fixing_kbs: Option<Vec<KnowledgeBase>>,
    /// Required. The name of the vulnerability.
    
    pub name: Option<String>,
}

impl client::Part for WindowsDetail {}


