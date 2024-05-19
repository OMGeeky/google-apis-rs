use super::*;
/// Configuration for an Apt remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AptRepository {
    /// Customer-specified remote repository.
    #[serde(rename="customRepository")]
    
    pub custom_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryCustomRepository>,
    /// One of the publicly available Apt repositories supported by Artifact Registry.
    #[serde(rename="publicRepository")]
    
    pub public_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepository>,
}

impl client::Part for AptRepository {}


/// The request to delete multiple versions across a repository.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages versions batch delete projects](ProjectLocationRepositoryPackageVersionBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteVersionsRequest {
    /// Required. The names of the versions to delete. A maximum of 10000 versions can be deleted in a batch.
    
    pub names: Option<Vec<String>>,
    /// If true, the request is performed without deleting data, following AIP-163.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for BatchDeleteVersionsRequest {}


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


/// Artifact policy configuration for repository cleanup policies.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CleanupPolicy {
    /// Policy action.
    
    pub action: Option<CleanupPolicyActionEnum>,
    /// Policy condition for matching versions.
    
    pub condition: Option<CleanupPolicyCondition>,
    /// The user-provided ID of the cleanup policy.
    
    pub id: Option<String>,
    /// Policy condition for retaining a minimum number of versions. May only be specified with a Keep action.
    #[serde(rename="mostRecentVersions")]
    
    pub most_recent_versions: Option<CleanupPolicyMostRecentVersions>,
}

impl client::Part for CleanupPolicy {}


/// CleanupPolicyCondition is a set of conditions attached to a CleanupPolicy. If multiple entries are set, all must be satisfied for the condition to be satisfied.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CleanupPolicyCondition {
    /// Match versions newer than a duration.
    #[serde(rename="newerThan")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub newer_than: Option<client::chrono::Duration>,
    /// Match versions older than a duration.
    #[serde(rename="olderThan")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub older_than: Option<client::chrono::Duration>,
    /// Match versions by package prefix. Applied on any prefix match.
    #[serde(rename="packageNamePrefixes")]
    
    pub package_name_prefixes: Option<Vec<String>>,
    /// Match versions by tag prefix. Applied on any prefix match.
    #[serde(rename="tagPrefixes")]
    
    pub tag_prefixes: Option<Vec<String>>,
    /// Match versions by tag status.
    #[serde(rename="tagState")]
    
    pub tag_state: Option<CleanupPolicyConditionTagStateEnum>,
    /// Match versions by version name prefix. Applied on any prefix match.
    #[serde(rename="versionNamePrefixes")]
    
    pub version_name_prefixes: Option<Vec<String>>,
}

impl client::Part for CleanupPolicyCondition {}


/// CleanupPolicyMostRecentVersions is an alternate condition of a CleanupPolicy for retaining a minimum number of versions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CleanupPolicyMostRecentVersions {
    /// Minimum number of versions to keep.
    #[serde(rename="keepCount")]
    
    pub keep_count: Option<i32>,
    /// List of package name prefixes that will apply this rule.
    #[serde(rename="packageNamePrefixes")]
    
    pub package_name_prefixes: Option<Vec<String>>,
}

impl client::Part for CleanupPolicyMostRecentVersions {}


/// DockerImage represents a docker artifact. The following fields are returned as untyped metadata in the Version resource, using camelcase keys (i.e. metadata.imageSizeBytes): * imageSizeBytes * mediaType * buildTime
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories docker images get projects](ProjectLocationRepositoryDockerImageGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DockerImage {
    /// The time this image was built. This field is returned as the 'metadata.buildTime' field in the Version resource. The build time is returned to the client as an RFC 3339 string, which can be easily used with the JavaScript Date constructor.
    #[serde(rename="buildTime")]
    
    pub build_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Calculated size of the image. This field is returned as the 'metadata.imageSizeBytes' field in the Version resource.
    #[serde(rename="imageSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub image_size_bytes: Option<i64>,
    /// Media type of this image, e.g. "application/vnd.docker.distribution.manifest.v2+json". This field is returned as the 'metadata.mediaType' field in the Version resource.
    #[serde(rename="mediaType")]
    
    pub media_type: Option<String>,
    /// Required. registry_location, project_id, repository_name and image id forms a unique image name:`projects//locations//repository//dockerImages/`. For example, "projects/test-project/locations/us-west4/repositories/test-repo/dockerImages/ nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and "nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf" is the image's digest.
    
    pub name: Option<String>,
    /// Tags attached to this image.
    
    pub tags: Option<Vec<String>>,
    /// Output only. The time when the docker image was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Time the image was uploaded.
    #[serde(rename="uploadTime")]
    
    pub upload_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. URL to access the image. Example: us-west4-docker.pkg.dev/test-project/test-repo/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
    
    pub uri: Option<String>,
}

impl client::ResponseResult for DockerImage {}


/// Configuration for a Docker remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DockerRepository {
    /// Customer-specified remote repository.
    #[serde(rename="customRepository")]
    
    pub custom_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigDockerRepositoryCustomRepository>,
    /// One of the publicly available Docker repositories supported by Artifact Registry.
    #[serde(rename="publicRepository")]
    
    pub public_repository: Option<DockerRepositoryPublicRepositoryEnum>,
}

impl client::Part for DockerRepository {}


/// DockerRepositoryConfig is docker related repository details. Provides additional configuration details for repositories of the docker format type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DockerRepositoryConfig {
    /// The repository which enabled this flag prevents all tags from being modified, moved or deleted. This does not prevent tags from being created.
    #[serde(rename="immutableTags")]
    
    pub immutable_tags: Option<bool>,
}

impl client::Part for DockerRepositoryConfig {}


/// The response to download a file.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [download media](MediaDownloadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadFileResponse { _never_set: Option<bool> }

impl client::ResponseResult for DownloadFileResponse {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages tags delete projects](ProjectLocationRepositoryPackageTagDeleteCall) (response)
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


/// Files store content that is potentially associated with Packages or Versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories files get projects](ProjectLocationRepositoryFileGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1File {
    /// Output only. The time when the File was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time when the last attempt to refresh the file's data was made. Only set when the repository is remote.
    #[serde(rename="fetchTime")]
    
    pub fetch_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The hashes of the file content.
    
    pub hashes: Option<Vec<Hash>>,
    /// The name of the file, for example: "projects/p1/locations/us-central1/repositories/repo1/files/a%2Fb%2Fc.txt". If the file ID part contains slashes, they are escaped.
    
    pub name: Option<String>,
    /// The name of the Package or Version that owns this file, if any.
    
    pub owner: Option<String>,
    /// The size of the File in bytes.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// Output only. The time when the File was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleDevtoolsArtifactregistryV1File {}


/// Customer-specified publicly available remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryCustomRepository {
    /// An http/https uri reference to the upstream remote repository, for ex: "https://my.apt.registry/".
    
    pub uri: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryCustomRepository {}


/// Publicly available Apt repositories constructed from a common repository base and a custom repository path.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepository {
    /// A common public repository base for Apt.
    #[serde(rename="repositoryBase")]
    
    pub repository_base: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum>,
    /// A custom field to define a path to a specific repository from the base.
    #[serde(rename="repositoryPath")]
    
    pub repository_path: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepository {}


/// Customer-specified publicly available remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigDockerRepositoryCustomRepository {
    /// An http/https uri reference to the custom remote repository, for ex: "https://registry-1.docker.io".
    
    pub uri: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigDockerRepositoryCustomRepository {}


/// Customer-specified publicly available remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigMavenRepositoryCustomRepository {
    /// An http/https uri reference to the upstream remote repository, for ex: "https://my.maven.registry/".
    
    pub uri: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigMavenRepositoryCustomRepository {}


/// Customer-specified publicly available remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigNpmRepositoryCustomRepository {
    /// An http/https uri reference to the upstream remote repository, for ex: "https://my.npm.registry/".
    
    pub uri: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigNpmRepositoryCustomRepository {}


/// Customer-specified publicly available remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigPythonRepositoryCustomRepository {
    /// An http/https uri reference to the upstream remote repository, for ex: "https://my.python.registry/".
    
    pub uri: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigPythonRepositoryCustomRepository {}


/// Customer-specified publicly available remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryCustomRepository {
    /// An http/https uri reference to the upstream remote repository, for ex: "https://my.yum.registry/".
    
    pub uri: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryCustomRepository {}


/// Publicly available Yum repositories constructed from a common repository base and a custom repository path.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepository {
    /// A common public repository base for Yum.
    #[serde(rename="repositoryBase")]
    
    pub repository_base: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum>,
    /// A custom field to define a path to a specific repository from the base.
    #[serde(rename="repositoryPath")]
    
    pub repository_path: Option<String>,
}

impl client::Part for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepository {}


/// A hash of file content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hash {
    /// The algorithm used to compute the hash value.
    #[serde(rename="type")]
    
    pub type_: Option<HashTypeEnum>,
    /// The hash value.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl client::Part for Hash {}


/// Google Cloud Storage location where the artifacts currently reside.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportAptArtifactsGcsSource {
    /// Cloud Storage paths URI (e.g., gs://my_bucket//my_object).
    
    pub uris: Option<Vec<String>>,
    /// Supports URI wildcards for matching multiple objects from a single URI.
    #[serde(rename="useWildcards")]
    
    pub use_wildcards: Option<bool>,
}

impl client::Part for ImportAptArtifactsGcsSource {}


/// The request to import new apt artifacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories apt artifacts import projects](ProjectLocationRepositoryAptArtifactImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportAptArtifactsRequest {
    /// Google Cloud Storage location where input content is located.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<ImportAptArtifactsGcsSource>,
}

impl client::RequestValue for ImportAptArtifactsRequest {}


/// Google Cloud Storage location where the artifacts currently reside.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportGoogetArtifactsGcsSource {
    /// Cloud Storage paths URI (e.g., `gs://my_bucket/my_object`).
    
    pub uris: Option<Vec<String>>,
    /// Supports URI wildcards for matching multiple objects from a single URI.
    #[serde(rename="useWildcards")]
    
    pub use_wildcards: Option<bool>,
}

impl client::Part for ImportGoogetArtifactsGcsSource {}


/// The request to import new googet artifacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories googet artifacts import projects](ProjectLocationRepositoryGoogetArtifactImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportGoogetArtifactsRequest {
    /// Google Cloud Storage location where input content is located.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<ImportGoogetArtifactsGcsSource>,
}

impl client::RequestValue for ImportGoogetArtifactsRequest {}


/// Google Cloud Storage location where the artifacts currently reside.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportYumArtifactsGcsSource {
    /// Cloud Storage paths URI (e.g., gs://my_bucket//my_object).
    
    pub uris: Option<Vec<String>>,
    /// Supports URI wildcards for matching multiple objects from a single URI.
    #[serde(rename="useWildcards")]
    
    pub use_wildcards: Option<bool>,
}

impl client::Part for ImportYumArtifactsGcsSource {}


/// The request to import new yum artifacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories yum artifacts import projects](ProjectLocationRepositoryYumArtifactImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportYumArtifactsRequest {
    /// Google Cloud Storage location where input content is located.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<ImportYumArtifactsGcsSource>,
}

impl client::RequestValue for ImportYumArtifactsRequest {}


/// The response from listing docker images.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories docker images list projects](ProjectLocationRepositoryDockerImageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDockerImagesResponse {
    /// The docker images returned.
    #[serde(rename="dockerImages")]
    
    pub docker_images: Option<Vec<DockerImage>>,
    /// The token to retrieve the next page of artifacts, or empty if there are no more artifacts to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDockerImagesResponse {}


/// The response from listing files.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories files list projects](ProjectLocationRepositoryFileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// The files returned.
    
    pub files: Option<Vec<GoogleDevtoolsArtifactregistryV1File>>,
    /// The token to retrieve the next page of files, or empty if there are no more files to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFilesResponse {}


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


/// The response from listing maven artifacts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories maven artifacts list projects](ProjectLocationRepositoryMavenArtifactListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMavenArtifactsResponse {
    /// The maven artifacts returned.
    #[serde(rename="mavenArtifacts")]
    
    pub maven_artifacts: Option<Vec<MavenArtifact>>,
    /// The token to retrieve the next page of artifacts, or empty if there are no more artifacts to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMavenArtifactsResponse {}


/// The response from listing npm packages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories npm packages list projects](ProjectLocationRepositoryNpmPackageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListNpmPackagesResponse {
    /// The token to retrieve the next page of artifacts, or empty if there are no more artifacts to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The npm packages returned.
    #[serde(rename="npmPackages")]
    
    pub npm_packages: Option<Vec<NpmPackage>>,
}

impl client::ResponseResult for ListNpmPackagesResponse {}


/// The response from listing packages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages list projects](ProjectLocationRepositoryPackageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPackagesResponse {
    /// The token to retrieve the next page of packages, or empty if there are no more packages to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The packages returned.
    
    pub packages: Option<Vec<Package>>,
}

impl client::ResponseResult for ListPackagesResponse {}


/// The response from listing python packages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories python packages list projects](ProjectLocationRepositoryPythonPackageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPythonPackagesResponse {
    /// The token to retrieve the next page of artifacts, or empty if there are no more artifacts to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The python packages returned.
    #[serde(rename="pythonPackages")]
    
    pub python_packages: Option<Vec<PythonPackage>>,
}

impl client::ResponseResult for ListPythonPackagesResponse {}


/// The response from listing repositories.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories list projects](ProjectLocationRepositoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRepositoriesResponse {
    /// The token to retrieve the next page of repositories, or empty if there are no more repositories to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The repositories returned.
    
    pub repositories: Option<Vec<Repository>>,
}

impl client::ResponseResult for ListRepositoriesResponse {}


/// The response from listing tags.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages tags list projects](ProjectLocationRepositoryPackageTagListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTagsResponse {
    /// The token to retrieve the next page of tags, or empty if there are no more tags to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The tags returned.
    
    pub tags: Option<Vec<Tag>>,
}

impl client::ResponseResult for ListTagsResponse {}


/// The response from listing versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages versions list projects](ProjectLocationRepositoryPackageVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVersionsResponse {
    /// The token to retrieve the next page of versions, or empty if there are no more versions to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The versions returned.
    
    pub versions: Option<Vec<Version>>,
}

impl client::ResponseResult for ListVersionsResponse {}


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


/// MavenArtifact represents a maven artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories maven artifacts get projects](ProjectLocationRepositoryMavenArtifactGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MavenArtifact {
    /// Artifact ID for the artifact.
    #[serde(rename="artifactId")]
    
    pub artifact_id: Option<String>,
    /// Output only. Time the artifact was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Group ID for the artifact. Example: com.google.guava
    #[serde(rename="groupId")]
    
    pub group_id: Option<String>,
    /// Required. registry_location, project_id, repository_name and maven_artifact forms a unique artifact For example, "projects/test-project/locations/us-west4/repositories/test-repo/mavenArtifacts/ com.google.guava:guava:31.0-jre", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and "com.google.guava:guava:31.0-jre" is the maven artifact.
    
    pub name: Option<String>,
    /// Required. URL to access the pom file of the artifact. Example: us-west4-maven.pkg.dev/test-project/test-repo/com/google/guava/guava/31.0/guava-31.0.pom
    #[serde(rename="pomUri")]
    
    pub pom_uri: Option<String>,
    /// Output only. Time the artifact was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Version of this artifact.
    
    pub version: Option<String>,
}

impl client::ResponseResult for MavenArtifact {}


/// Configuration for a Maven remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MavenRepository {
    /// Customer-specified remote repository.
    #[serde(rename="customRepository")]
    
    pub custom_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigMavenRepositoryCustomRepository>,
    /// One of the publicly available Maven repositories supported by Artifact Registry.
    #[serde(rename="publicRepository")]
    
    pub public_repository: Option<MavenRepositoryPublicRepositoryEnum>,
}

impl client::Part for MavenRepository {}


/// MavenRepositoryConfig is maven related repository details. Provides additional configuration details for repositories of the maven format type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MavenRepositoryConfig {
    /// The repository with this flag will allow publishing the same snapshot versions.
    #[serde(rename="allowSnapshotOverwrites")]
    
    pub allow_snapshot_overwrites: Option<bool>,
    /// Version policy defines the versions that the registry will accept.
    #[serde(rename="versionPolicy")]
    
    pub version_policy: Option<MavenRepositoryConfigVersionPolicyEnum>,
}

impl client::Part for MavenRepositoryConfig {}


/// NpmPackage represents an npm artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories npm packages get projects](ProjectLocationRepositoryNpmPackageGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NpmPackage {
    /// Output only. Time the package was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. registry_location, project_id, repository_name and npm_package forms a unique package For example, "projects/test-project/locations/us-west4/repositories/test-repo/npmPackages/ npm_test:1.0.0", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and npm_test:1.0.0" is the npm package.
    
    pub name: Option<String>,
    /// Package for the artifact.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Tags attached to this package.
    
    pub tags: Option<Vec<String>>,
    /// Output only. Time the package was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Version of this package.
    
    pub version: Option<String>,
}

impl client::ResponseResult for NpmPackage {}


/// Configuration for a Npm remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NpmRepository {
    /// Customer-specified remote repository.
    #[serde(rename="customRepository")]
    
    pub custom_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigNpmRepositoryCustomRepository>,
    /// One of the publicly available Npm repositories supported by Artifact Registry.
    #[serde(rename="publicRepository")]
    
    pub public_repository: Option<NpmRepositoryPublicRepositoryEnum>,
}

impl client::Part for NpmRepository {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations repositories apt artifacts import projects](ProjectLocationRepositoryAptArtifactImportCall) (response)
/// * [locations repositories googet artifacts import projects](ProjectLocationRepositoryGoogetArtifactImportCall) (response)
/// * [locations repositories packages versions batch delete projects](ProjectLocationRepositoryPackageVersionBatchDeleteCall) (response)
/// * [locations repositories packages versions delete projects](ProjectLocationRepositoryPackageVersionDeleteCall) (response)
/// * [locations repositories packages delete projects](ProjectLocationRepositoryPackageDeleteCall) (response)
/// * [locations repositories yum artifacts import projects](ProjectLocationRepositoryYumArtifactImportCall) (response)
/// * [locations repositories create projects](ProjectLocationRepositoryCreateCall) (response)
/// * [locations repositories delete projects](ProjectLocationRepositoryDeleteCall) (response)
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


/// Packages are named collections of versions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages get projects](ProjectLocationRepositoryPackageGetCall) (response)
/// * [locations repositories packages patch projects](ProjectLocationRepositoryPackagePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    /// Optional. Client specified annotations.
    
    pub annotations: Option<HashMap<String, String>>,
    /// The time when the package was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The display name of the package.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped.
    
    pub name: Option<String>,
    /// The time when the package was last updated. This includes publishing a new version of the package.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Package {}
impl client::ResponseResult for Package {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories get iam policy projects](ProjectLocationRepositoryGetIamPolicyCall) (response)
/// * [locations repositories set iam policy projects](ProjectLocationRepositorySetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// The Artifact Registry settings that apply to a Project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get project settings projects](ProjectGetProjectSettingCall) (response)
/// * [update project settings projects](ProjectUpdateProjectSettingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// The redirection state of the legacy repositories in this project.
    #[serde(rename="legacyRedirectionState")]
    
    pub legacy_redirection_state: Option<ProjectSettingLegacyRedirectionStateEnum>,
    /// The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set
    
    pub name: Option<String>,
}

impl client::RequestValue for ProjectSettings {}
impl client::ResponseResult for ProjectSettings {}


/// PythonPackage represents a python artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories python packages get projects](ProjectLocationRepositoryPythonPackageGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PythonPackage {
    /// Output only. Time the package was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. registry_location, project_id, repository_name and python_package forms a unique package name:`projects//locations//repository//pythonPackages/`. For example, "projects/test-project/locations/us-west4/repositories/test-repo/pythonPackages/ python_package:1.0.0", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and python_package:1.0.0" is the python package.
    
    pub name: Option<String>,
    /// Package for the artifact.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Output only. Time the package was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. URL to access the package. Example: us-west4-python.pkg.dev/test-project/test-repo/python_package/file-name-1.0.0.tar.gz
    
    pub uri: Option<String>,
    /// Version of this package.
    
    pub version: Option<String>,
}

impl client::ResponseResult for PythonPackage {}


/// Configuration for a Python remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PythonRepository {
    /// Customer-specified remote repository.
    #[serde(rename="customRepository")]
    
    pub custom_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigPythonRepositoryCustomRepository>,
    /// One of the publicly available Python repositories supported by Artifact Registry.
    #[serde(rename="publicRepository")]
    
    pub public_repository: Option<PythonRepositoryPublicRepositoryEnum>,
}

impl client::Part for PythonRepository {}


/// Remote repository configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteRepositoryConfig {
    /// Specific settings for an Apt remote repository.
    #[serde(rename="aptRepository")]
    
    pub apt_repository: Option<AptRepository>,
    /// The description of the remote source.
    
    pub description: Option<String>,
    /// Input only. A create/update remote repo option to avoid making a HEAD/GET request to validate a remote repo and any supplied upstream credentials.
    #[serde(rename="disableUpstreamValidation")]
    
    pub disable_upstream_validation: Option<bool>,
    /// Specific settings for a Docker remote repository.
    #[serde(rename="dockerRepository")]
    
    pub docker_repository: Option<DockerRepository>,
    /// Specific settings for a Maven remote repository.
    #[serde(rename="mavenRepository")]
    
    pub maven_repository: Option<MavenRepository>,
    /// Specific settings for an Npm remote repository.
    #[serde(rename="npmRepository")]
    
    pub npm_repository: Option<NpmRepository>,
    /// Specific settings for a Python remote repository.
    #[serde(rename="pythonRepository")]
    
    pub python_repository: Option<PythonRepository>,
    /// Optional. The credentials used to access the remote repository.
    #[serde(rename="upstreamCredentials")]
    
    pub upstream_credentials: Option<UpstreamCredentials>,
    /// Specific settings for a Yum remote repository.
    #[serde(rename="yumRepository")]
    
    pub yum_repository: Option<YumRepository>,
}

impl client::Part for RemoteRepositoryConfig {}


/// A Repository for storing artifacts with a specific format.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories create projects](ProjectLocationRepositoryCreateCall) (request)
/// * [locations repositories get projects](ProjectLocationRepositoryGetCall) (response)
/// * [locations repositories patch projects](ProjectLocationRepositoryPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    /// Optional. Cleanup policies for this repository. Cleanup policies indicate when certain package versions can be automatically deleted. Map keys are policy IDs supplied by users during policy creation. They must unique within a repository and be under 128 characters in length.
    #[serde(rename="cleanupPolicies")]
    
    pub cleanup_policies: Option<HashMap<String, CleanupPolicy>>,
    /// Optional. If true, the cleanup pipeline is prevented from deleting versions in this repository.
    #[serde(rename="cleanupPolicyDryRun")]
    
    pub cleanup_policy_dry_run: Option<bool>,
    /// Output only. The time when the repository was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The user-provided description of the repository.
    
    pub description: Option<String>,
    /// Optional. If this is true, aunspecified repo type will be treated as error. Is used for new repo types that don't have any specific fields. Right now is used by AOSS team when creating repos for customers.
    #[serde(rename="disallowUnspecifiedMode")]
    
    pub disallow_unspecified_mode: Option<bool>,
    /// Docker repository config contains repository level configuration for the repositories of docker type.
    #[serde(rename="dockerConfig")]
    
    pub docker_config: Option<DockerRepositoryConfig>,
    /// Optional. The format of packages that are stored in the repository.
    
    pub format: Option<RepositoryFormatEnum>,
    /// The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes.
    
    pub labels: Option<HashMap<String, String>>,
    /// Maven repository config contains repository level configuration for the repositories of maven type.
    #[serde(rename="mavenConfig")]
    
    pub maven_config: Option<MavenRepositoryConfig>,
    /// Optional. The mode of the repository.
    
    pub mode: Option<RepositoryModeEnum>,
    /// The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`.
    
    pub name: Option<String>,
    /// Configuration specific for a Remote Repository.
    #[serde(rename="remoteRepositoryConfig")]
    
    pub remote_repository_config: Option<RemoteRepositoryConfig>,
    /// Output only. If set, the repository satisfies physical zone separation.
    #[serde(rename="satisfiesPzs")]
    
    pub satisfies_pzs: Option<bool>,
    /// Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs.
    #[serde(rename="sizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// Output only. The time when the repository was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Configuration specific for a Virtual Repository.
    #[serde(rename="virtualRepositoryConfig")]
    
    pub virtual_repository_config: Option<VirtualRepositoryConfig>,
}

impl client::RequestValue for Repository {}
impl client::ResponseResult for Repository {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories set iam policy projects](ProjectLocationRepositorySetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


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


/// Tags point to a version and represent an alternative name that can be used to access the version.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages tags create projects](ProjectLocationRepositoryPackageTagCreateCall) (request|response)
/// * [locations repositories packages tags get projects](ProjectLocationRepositoryPackageTagGetCall) (response)
/// * [locations repositories packages tags patch projects](ProjectLocationRepositoryPackageTagPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    /// The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded.
    
    pub name: Option<String>,
    /// The name of the version the tag refers to, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811" If the package or version ID parts contain slashes, the slashes are escaped.
    
    pub version: Option<String>,
}

impl client::RequestValue for Tag {}
impl client::ResponseResult for Tag {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories test iam permissions projects](ProjectLocationRepositoryTestIamPermissionCall) (request)
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
/// * [locations repositories test iam permissions projects](ProjectLocationRepositoryTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// The response to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories apt artifacts upload projects](ProjectLocationRepositoryAptArtifactUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadAptArtifactMediaResponse {
    /// Operation to be returned to the user.
    
    pub operation: Option<Operation>,
}

impl client::ResponseResult for UploadAptArtifactMediaResponse {}


/// The request to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories apt artifacts upload projects](ProjectLocationRepositoryAptArtifactUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadAptArtifactRequest { _never_set: Option<bool> }

impl client::RequestValue for UploadAptArtifactRequest {}


/// The response to upload a Go module.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories go modules upload projects](ProjectLocationRepositoryGoModuleUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadGoModuleMediaResponse {
    /// Operation to be returned to the user.
    
    pub operation: Option<Operation>,
}

impl client::ResponseResult for UploadGoModuleMediaResponse {}


/// The request to upload a Go module.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories go modules upload projects](ProjectLocationRepositoryGoModuleUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadGoModuleRequest { _never_set: Option<bool> }

impl client::RequestValue for UploadGoModuleRequest {}


/// The response to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories googet artifacts upload projects](ProjectLocationRepositoryGoogetArtifactUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadGoogetArtifactMediaResponse {
    /// Operation to be returned to the user.
    
    pub operation: Option<Operation>,
}

impl client::ResponseResult for UploadGoogetArtifactMediaResponse {}


/// The request to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories googet artifacts upload projects](ProjectLocationRepositoryGoogetArtifactUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadGoogetArtifactRequest { _never_set: Option<bool> }

impl client::RequestValue for UploadGoogetArtifactRequest {}


/// The response to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories kfp artifacts upload projects](ProjectLocationRepositoryKfpArtifactUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadKfpArtifactMediaResponse {
    /// Operation that will be returned to the user.
    
    pub operation: Option<Operation>,
}

impl client::ResponseResult for UploadKfpArtifactMediaResponse {}


/// The request to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories kfp artifacts upload projects](ProjectLocationRepositoryKfpArtifactUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadKfpArtifactRequest {
    /// Description of the package version.
    
    pub description: Option<String>,
    /// Tags to be created with the version.
    
    pub tags: Option<Vec<String>>,
}

impl client::RequestValue for UploadKfpArtifactRequest {}


/// The response to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories yum artifacts upload projects](ProjectLocationRepositoryYumArtifactUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadYumArtifactMediaResponse {
    /// Operation to be returned to the user.
    
    pub operation: Option<Operation>,
}

impl client::ResponseResult for UploadYumArtifactMediaResponse {}


/// The request to upload an artifact.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories yum artifacts upload projects](ProjectLocationRepositoryYumArtifactUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadYumArtifactRequest { _never_set: Option<bool> }

impl client::RequestValue for UploadYumArtifactRequest {}


/// The credentials to access the remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpstreamCredentials {
    /// Use username and password to access the remote repository.
    #[serde(rename="usernamePasswordCredentials")]
    
    pub username_password_credentials: Option<UsernamePasswordCredentials>,
}

impl client::Part for UpstreamCredentials {}


/// Artifact policy configuration for the repository contents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpstreamPolicy {
    /// The user-provided ID of the upstream policy.
    
    pub id: Option<String>,
    /// Entries with a greater priority value take precedence in the pull order.
    
    pub priority: Option<i32>,
    /// A reference to the repository resource, for example: `projects/p1/locations/us-central1/repositories/repo1`.
    
    pub repository: Option<String>,
}

impl client::Part for UpstreamPolicy {}


/// Username and password credentials.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UsernamePasswordCredentials {
    /// The Secret Manager key version that holds the password to access the remote repository. Must be in the format of `projects/{project}/secrets/{secret}/versions/{version}`.
    #[serde(rename="passwordSecretVersion")]
    
    pub password_secret_version: Option<String>,
    /// The username to access the remote repository.
    
    pub username: Option<String>,
}

impl client::Part for UsernamePasswordCredentials {}


/// The Artifact Registry VPC SC config that apply to a Project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get vpcsc config projects](ProjectLocationGetVpcscConfigCall) (response)
/// * [locations update vpcsc config projects](ProjectLocationUpdateVpcscConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VPCSCConfig {
    /// The name of the project's VPC SC Config. Always of the form: projects/{projectID}/locations/{location}/vpcscConfig In update request: never set In response: always set
    
    pub name: Option<String>,
    /// The project per location VPC SC policy that defines the VPC SC behavior for the Remote Repository (Allow/Deny).
    #[serde(rename="vpcscPolicy")]
    
    pub vpcsc_policy: Option<VPCSCConfigVpcscPolicyEnum>,
}

impl client::RequestValue for VPCSCConfig {}
impl client::ResponseResult for VPCSCConfig {}


/// The body of a version resource. A version resource represents a collection of components, such as files and other data. This may correspond to a version in many package management schemes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations repositories packages versions get projects](ProjectLocationRepositoryPackageVersionGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// The time when the version was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Description of the version, as specified in its metadata.
    
    pub description: Option<String>,
    /// Output only. Repository-specific Metadata stored against this version. The fields returned are defined by the underlying repository-specific resource. Currently, the resources could be: DockerImage MavenArtifact
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The name of the version, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/art1". If the package or version ID parts contain slashes, the slashes are escaped.
    
    pub name: Option<String>,
    /// Output only. A list of related tags. Will contain up to 100 tags that reference this version.
    #[serde(rename="relatedTags")]
    
    pub related_tags: Option<Vec<Tag>>,
    /// The time when the version was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Version {}


/// Virtual repository configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VirtualRepositoryConfig {
    /// Policies that configure the upstream artifacts distributed by the Virtual Repository. Upstream policies cannot be set on a standard repository.
    #[serde(rename="upstreamPolicies")]
    
    pub upstream_policies: Option<Vec<UpstreamPolicy>>,
}

impl client::Part for VirtualRepositoryConfig {}


/// Configuration for a Yum remote repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct YumRepository {
    /// Customer-specified remote repository.
    #[serde(rename="customRepository")]
    
    pub custom_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryCustomRepository>,
    /// One of the publicly available Yum repositories supported by Artifact Registry.
    #[serde(rename="publicRepository")]
    
    pub public_repository: Option<GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepository>,
}

impl client::Part for YumRepository {}


