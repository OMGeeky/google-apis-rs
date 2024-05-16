use super::*;



// region CleanupPolicyActionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Policy action.
pub enum CleanupPolicyActionEnum {
    

    /// Action not specified.
    ///
    /// "ACTION_UNSPECIFIED"
    #[serde(rename="ACTION_UNSPECIFIED")]
    ACTIONUNSPECIFIED,
    

    /// Delete action.
    ///
    /// "DELETE"
    #[serde(rename="DELETE")]
    DELETE,
    

    /// Keep action.
    ///
    /// "KEEP"
    #[serde(rename="KEEP")]
    KEEP,
}

impl AsRef<str> for CleanupPolicyActionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CleanupPolicyActionEnum::ACTIONUNSPECIFIED => "ACTION_UNSPECIFIED",
            CleanupPolicyActionEnum::DELETE => "DELETE",
            CleanupPolicyActionEnum::KEEP => "KEEP",
        }
    }
}

impl std::convert::TryFrom< &str> for CleanupPolicyActionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACTION_UNSPECIFIED" => Ok(CleanupPolicyActionEnum::ACTIONUNSPECIFIED),
           "DELETE" => Ok(CleanupPolicyActionEnum::DELETE),
           "KEEP" => Ok(CleanupPolicyActionEnum::KEEP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CleanupPolicyActionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CleanupPolicyConditionTagStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Match versions by tag status.
pub enum CleanupPolicyConditionTagStateEnum {
    

    /// Tag status not specified.
    ///
    /// "TAG_STATE_UNSPECIFIED"
    #[serde(rename="TAG_STATE_UNSPECIFIED")]
    TAGSTATEUNSPECIFIED,
    

    /// Applies to tagged versions only.
    ///
    /// "TAGGED"
    #[serde(rename="TAGGED")]
    TAGGED,
    

    /// Applies to untagged versions only.
    ///
    /// "UNTAGGED"
    #[serde(rename="UNTAGGED")]
    UNTAGGED,
    

    /// Applies to all versions.
    ///
    /// "ANY"
    #[serde(rename="ANY")]
    ANY,
}

impl AsRef<str> for CleanupPolicyConditionTagStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CleanupPolicyConditionTagStateEnum::TAGSTATEUNSPECIFIED => "TAG_STATE_UNSPECIFIED",
            CleanupPolicyConditionTagStateEnum::TAGGED => "TAGGED",
            CleanupPolicyConditionTagStateEnum::UNTAGGED => "UNTAGGED",
            CleanupPolicyConditionTagStateEnum::ANY => "ANY",
        }
    }
}

impl std::convert::TryFrom< &str> for CleanupPolicyConditionTagStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TAG_STATE_UNSPECIFIED" => Ok(CleanupPolicyConditionTagStateEnum::TAGSTATEUNSPECIFIED),
           "TAGGED" => Ok(CleanupPolicyConditionTagStateEnum::TAGGED),
           "UNTAGGED" => Ok(CleanupPolicyConditionTagStateEnum::UNTAGGED),
           "ANY" => Ok(CleanupPolicyConditionTagStateEnum::ANY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CleanupPolicyConditionTagStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DockerRepositoryPublicRepositoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// One of the publicly available Docker repositories supported by Artifact Registry.
pub enum DockerRepositoryPublicRepositoryEnum {
    

    /// Unspecified repository.
    ///
    /// "PUBLIC_REPOSITORY_UNSPECIFIED"
    #[serde(rename="PUBLIC_REPOSITORY_UNSPECIFIED")]
    PUBLICREPOSITORYUNSPECIFIED,
    

    /// Docker Hub.
    ///
    /// "DOCKER_HUB"
    #[serde(rename="DOCKER_HUB")]
    DOCKERHUB,
}

impl AsRef<str> for DockerRepositoryPublicRepositoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DockerRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED => "PUBLIC_REPOSITORY_UNSPECIFIED",
            DockerRepositoryPublicRepositoryEnum::DOCKERHUB => "DOCKER_HUB",
        }
    }
}

impl std::convert::TryFrom< &str> for DockerRepositoryPublicRepositoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC_REPOSITORY_UNSPECIFIED" => Ok(DockerRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED),
           "DOCKER_HUB" => Ok(DockerRepositoryPublicRepositoryEnum::DOCKERHUB),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DockerRepositoryPublicRepositoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A common public repository base for Apt.
pub enum GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum {
    

    /// Unspecified repository base.
    ///
    /// "REPOSITORY_BASE_UNSPECIFIED"
    #[serde(rename="REPOSITORY_BASE_UNSPECIFIED")]
    REPOSITORYBASEUNSPECIFIED,
    

    /// Debian.
    ///
    /// "DEBIAN"
    #[serde(rename="DEBIAN")]
    DEBIAN,
    

    /// Ubuntu LTS/Pro.
    ///
    /// "UBUNTU"
    #[serde(rename="UBUNTU")]
    UBUNTU,
    

    /// Archived Debian.
    ///
    /// "DEBIAN_SNAPSHOT"
    #[serde(rename="DEBIAN_SNAPSHOT")]
    DEBIANSNAPSHOT,
}

impl AsRef<str> for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::REPOSITORYBASEUNSPECIFIED => "REPOSITORY_BASE_UNSPECIFIED",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::DEBIAN => "DEBIAN",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::UBUNTU => "UBUNTU",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::DEBIANSNAPSHOT => "DEBIAN_SNAPSHOT",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPOSITORY_BASE_UNSPECIFIED" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::REPOSITORYBASEUNSPECIFIED),
           "DEBIAN" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::DEBIAN),
           "UBUNTU" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::UBUNTU),
           "DEBIAN_SNAPSHOT" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum::DEBIANSNAPSHOT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigAptRepositoryPublicRepositoryRepositoryBaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A common public repository base for Yum.
pub enum GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum {
    

    /// Unspecified repository base.
    ///
    /// "REPOSITORY_BASE_UNSPECIFIED"
    #[serde(rename="REPOSITORY_BASE_UNSPECIFIED")]
    REPOSITORYBASEUNSPECIFIED,
    

    /// CentOS.
    ///
    /// "CENTOS"
    #[serde(rename="CENTOS")]
    CENTOS,
    

    /// CentOS Debug.
    ///
    /// "CENTOS_DEBUG"
    #[serde(rename="CENTOS_DEBUG")]
    CENTOSDEBUG,
    

    /// CentOS Vault.
    ///
    /// "CENTOS_VAULT"
    #[serde(rename="CENTOS_VAULT")]
    CENTOSVAULT,
    

    /// CentOS Stream.
    ///
    /// "CENTOS_STREAM"
    #[serde(rename="CENTOS_STREAM")]
    CENTOSSTREAM,
    

    /// Rocky.
    ///
    /// "ROCKY"
    #[serde(rename="ROCKY")]
    ROCKY,
    

    /// Fedora Extra Packages for Enterprise Linux (EPEL).
    ///
    /// "EPEL"
    #[serde(rename="EPEL")]
    EPEL,
}

impl AsRef<str> for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::REPOSITORYBASEUNSPECIFIED => "REPOSITORY_BASE_UNSPECIFIED",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOS => "CENTOS",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOSDEBUG => "CENTOS_DEBUG",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOSVAULT => "CENTOS_VAULT",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOSSTREAM => "CENTOS_STREAM",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::ROCKY => "ROCKY",
            GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::EPEL => "EPEL",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REPOSITORY_BASE_UNSPECIFIED" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::REPOSITORYBASEUNSPECIFIED),
           "CENTOS" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOS),
           "CENTOS_DEBUG" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOSDEBUG),
           "CENTOS_VAULT" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOSVAULT),
           "CENTOS_STREAM" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::CENTOSSTREAM),
           "ROCKY" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::ROCKY),
           "EPEL" => Ok(GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum::EPEL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleDevtoolsArtifactregistryV1RemoteRepositoryConfigYumRepositoryPublicRepositoryRepositoryBaseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region HashTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The algorithm used to compute the hash value.
pub enum HashTypeEnum {
    

    /// Unspecified.
    ///
    /// "HASH_TYPE_UNSPECIFIED"
    #[serde(rename="HASH_TYPE_UNSPECIFIED")]
    HASHTYPEUNSPECIFIED,
    

    /// SHA256 hash.
    ///
    /// "SHA256"
    #[serde(rename="SHA256")]
    SHA256,
    

    /// MD5 hash.
    ///
    /// "MD5"
    #[serde(rename="MD5")]
    MD5,
}

impl AsRef<str> for HashTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            HashTypeEnum::HASHTYPEUNSPECIFIED => "HASH_TYPE_UNSPECIFIED",
            HashTypeEnum::SHA256 => "SHA256",
            HashTypeEnum::MD5 => "MD5",
        }
    }
}

impl std::convert::TryFrom< &str> for HashTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HASH_TYPE_UNSPECIFIED" => Ok(HashTypeEnum::HASHTYPEUNSPECIFIED),
           "SHA256" => Ok(HashTypeEnum::SHA256),
           "MD5" => Ok(HashTypeEnum::MD5),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a HashTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MavenRepositoryPublicRepositoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// One of the publicly available Maven repositories supported by Artifact Registry.
pub enum MavenRepositoryPublicRepositoryEnum {
    

    /// Unspecified repository.
    ///
    /// "PUBLIC_REPOSITORY_UNSPECIFIED"
    #[serde(rename="PUBLIC_REPOSITORY_UNSPECIFIED")]
    PUBLICREPOSITORYUNSPECIFIED,
    

    /// Maven Central.
    ///
    /// "MAVEN_CENTRAL"
    #[serde(rename="MAVEN_CENTRAL")]
    MAVENCENTRAL,
}

impl AsRef<str> for MavenRepositoryPublicRepositoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MavenRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED => "PUBLIC_REPOSITORY_UNSPECIFIED",
            MavenRepositoryPublicRepositoryEnum::MAVENCENTRAL => "MAVEN_CENTRAL",
        }
    }
}

impl std::convert::TryFrom< &str> for MavenRepositoryPublicRepositoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC_REPOSITORY_UNSPECIFIED" => Ok(MavenRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED),
           "MAVEN_CENTRAL" => Ok(MavenRepositoryPublicRepositoryEnum::MAVENCENTRAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MavenRepositoryPublicRepositoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MavenRepositoryConfigVersionPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Version policy defines the versions that the registry will accept.
pub enum MavenRepositoryConfigVersionPolicyEnum {
    

    /// VERSION_POLICY_UNSPECIFIED - the version policy is not defined. When the version policy is not defined, no validation is performed for the versions.
    ///
    /// "VERSION_POLICY_UNSPECIFIED"
    #[serde(rename="VERSION_POLICY_UNSPECIFIED")]
    VERSIONPOLICYUNSPECIFIED,
    

    /// RELEASE - repository will accept only Release versions.
    ///
    /// "RELEASE"
    #[serde(rename="RELEASE")]
    RELEASE,
    

    /// SNAPSHOT - repository will accept only Snapshot versions.
    ///
    /// "SNAPSHOT"
    #[serde(rename="SNAPSHOT")]
    SNAPSHOT,
}

impl AsRef<str> for MavenRepositoryConfigVersionPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MavenRepositoryConfigVersionPolicyEnum::VERSIONPOLICYUNSPECIFIED => "VERSION_POLICY_UNSPECIFIED",
            MavenRepositoryConfigVersionPolicyEnum::RELEASE => "RELEASE",
            MavenRepositoryConfigVersionPolicyEnum::SNAPSHOT => "SNAPSHOT",
        }
    }
}

impl std::convert::TryFrom< &str> for MavenRepositoryConfigVersionPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_POLICY_UNSPECIFIED" => Ok(MavenRepositoryConfigVersionPolicyEnum::VERSIONPOLICYUNSPECIFIED),
           "RELEASE" => Ok(MavenRepositoryConfigVersionPolicyEnum::RELEASE),
           "SNAPSHOT" => Ok(MavenRepositoryConfigVersionPolicyEnum::SNAPSHOT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MavenRepositoryConfigVersionPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region NpmRepositoryPublicRepositoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// One of the publicly available Npm repositories supported by Artifact Registry.
pub enum NpmRepositoryPublicRepositoryEnum {
    

    /// Unspecified repository.
    ///
    /// "PUBLIC_REPOSITORY_UNSPECIFIED"
    #[serde(rename="PUBLIC_REPOSITORY_UNSPECIFIED")]
    PUBLICREPOSITORYUNSPECIFIED,
    

    /// npmjs.
    ///
    /// "NPMJS"
    #[serde(rename="NPMJS")]
    NPMJS,
}

impl AsRef<str> for NpmRepositoryPublicRepositoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            NpmRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED => "PUBLIC_REPOSITORY_UNSPECIFIED",
            NpmRepositoryPublicRepositoryEnum::NPMJS => "NPMJS",
        }
    }
}

impl std::convert::TryFrom< &str> for NpmRepositoryPublicRepositoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC_REPOSITORY_UNSPECIFIED" => Ok(NpmRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED),
           "NPMJS" => Ok(NpmRepositoryPublicRepositoryEnum::NPMJS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a NpmRepositoryPublicRepositoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectSettingLegacyRedirectionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The redirection state of the legacy repositories in this project.
pub enum ProjectSettingLegacyRedirectionStateEnum {
    

    /// No redirection status has been set.
    ///
    /// "REDIRECTION_STATE_UNSPECIFIED"
    #[serde(rename="REDIRECTION_STATE_UNSPECIFIED")]
    REDIRECTIONSTATEUNSPECIFIED,
    

    /// Redirection is disabled.
    ///
    /// "REDIRECTION_FROM_GCR_IO_DISABLED"
    #[serde(rename="REDIRECTION_FROM_GCR_IO_DISABLED")]
    REDIRECTIONFROMGCRIODISABLED,
    

    /// Redirection is enabled.
    ///
    /// "REDIRECTION_FROM_GCR_IO_ENABLED"
    #[serde(rename="REDIRECTION_FROM_GCR_IO_ENABLED")]
    REDIRECTIONFROMGCRIOENABLED,
    

    /// Redirection is enabled, and has been finalized so cannot be reverted.
    ///
    /// "REDIRECTION_FROM_GCR_IO_FINALIZED"
    #[serde(rename="REDIRECTION_FROM_GCR_IO_FINALIZED")]
    REDIRECTIONFROMGCRIOFINALIZED,
    

    /// Redirection is enabled and missing images are copied from GCR
    ///
    /// "REDIRECTION_FROM_GCR_IO_ENABLED_AND_COPYING"
    #[serde(rename="REDIRECTION_FROM_GCR_IO_ENABLED_AND_COPYING")]
    REDIRECTIONFROMGCRIOENABLEDANDCOPYING,
}

impl AsRef<str> for ProjectSettingLegacyRedirectionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONSTATEUNSPECIFIED => "REDIRECTION_STATE_UNSPECIFIED",
            ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIODISABLED => "REDIRECTION_FROM_GCR_IO_DISABLED",
            ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIOENABLED => "REDIRECTION_FROM_GCR_IO_ENABLED",
            ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIOFINALIZED => "REDIRECTION_FROM_GCR_IO_FINALIZED",
            ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIOENABLEDANDCOPYING => "REDIRECTION_FROM_GCR_IO_ENABLED_AND_COPYING",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectSettingLegacyRedirectionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REDIRECTION_STATE_UNSPECIFIED" => Ok(ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONSTATEUNSPECIFIED),
           "REDIRECTION_FROM_GCR_IO_DISABLED" => Ok(ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIODISABLED),
           "REDIRECTION_FROM_GCR_IO_ENABLED" => Ok(ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIOENABLED),
           "REDIRECTION_FROM_GCR_IO_FINALIZED" => Ok(ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIOFINALIZED),
           "REDIRECTION_FROM_GCR_IO_ENABLED_AND_COPYING" => Ok(ProjectSettingLegacyRedirectionStateEnum::REDIRECTIONFROMGCRIOENABLEDANDCOPYING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectSettingLegacyRedirectionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PythonRepositoryPublicRepositoryEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// One of the publicly available Python repositories supported by Artifact Registry.
pub enum PythonRepositoryPublicRepositoryEnum {
    

    /// Unspecified repository.
    ///
    /// "PUBLIC_REPOSITORY_UNSPECIFIED"
    #[serde(rename="PUBLIC_REPOSITORY_UNSPECIFIED")]
    PUBLICREPOSITORYUNSPECIFIED,
    

    /// PyPI.
    ///
    /// "PYPI"
    #[serde(rename="PYPI")]
    PYPI,
}

impl AsRef<str> for PythonRepositoryPublicRepositoryEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PythonRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED => "PUBLIC_REPOSITORY_UNSPECIFIED",
            PythonRepositoryPublicRepositoryEnum::PYPI => "PYPI",
        }
    }
}

impl std::convert::TryFrom< &str> for PythonRepositoryPublicRepositoryEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC_REPOSITORY_UNSPECIFIED" => Ok(PythonRepositoryPublicRepositoryEnum::PUBLICREPOSITORYUNSPECIFIED),
           "PYPI" => Ok(PythonRepositoryPublicRepositoryEnum::PYPI),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PythonRepositoryPublicRepositoryEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RepositoryFormatEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The format of packages that are stored in the repository.
pub enum RepositoryFormatEnum {
    

    /// Unspecified package format.
    ///
    /// "FORMAT_UNSPECIFIED"
    #[serde(rename="FORMAT_UNSPECIFIED")]
    FORMATUNSPECIFIED,
    

    /// Docker package format.
    ///
    /// "DOCKER"
    #[serde(rename="DOCKER")]
    DOCKER,
    

    /// Maven package format.
    ///
    /// "MAVEN"
    #[serde(rename="MAVEN")]
    MAVEN,
    

    /// NPM package format.
    ///
    /// "NPM"
    #[serde(rename="NPM")]
    NPM,
    

    /// APT package format.
    ///
    /// "APT"
    #[serde(rename="APT")]
    APT,
    

    /// YUM package format.
    ///
    /// "YUM"
    #[serde(rename="YUM")]
    YUM,
    

    /// GooGet package format.
    ///
    /// "GOOGET"
    #[serde(rename="GOOGET")]
    GOOGET,
    

    /// Python package format.
    ///
    /// "PYTHON"
    #[serde(rename="PYTHON")]
    PYTHON,
    

    /// Kubeflow Pipelines package format.
    ///
    /// "KFP"
    #[serde(rename="KFP")]
    KFP,
    

    /// Go package format.
    ///
    /// "GO"
    #[serde(rename="GO")]
    GO,
}

impl AsRef<str> for RepositoryFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RepositoryFormatEnum::FORMATUNSPECIFIED => "FORMAT_UNSPECIFIED",
            RepositoryFormatEnum::DOCKER => "DOCKER",
            RepositoryFormatEnum::MAVEN => "MAVEN",
            RepositoryFormatEnum::NPM => "NPM",
            RepositoryFormatEnum::APT => "APT",
            RepositoryFormatEnum::YUM => "YUM",
            RepositoryFormatEnum::GOOGET => "GOOGET",
            RepositoryFormatEnum::PYTHON => "PYTHON",
            RepositoryFormatEnum::KFP => "KFP",
            RepositoryFormatEnum::GO => "GO",
        }
    }
}

impl std::convert::TryFrom< &str> for RepositoryFormatEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FORMAT_UNSPECIFIED" => Ok(RepositoryFormatEnum::FORMATUNSPECIFIED),
           "DOCKER" => Ok(RepositoryFormatEnum::DOCKER),
           "MAVEN" => Ok(RepositoryFormatEnum::MAVEN),
           "NPM" => Ok(RepositoryFormatEnum::NPM),
           "APT" => Ok(RepositoryFormatEnum::APT),
           "YUM" => Ok(RepositoryFormatEnum::YUM),
           "GOOGET" => Ok(RepositoryFormatEnum::GOOGET),
           "PYTHON" => Ok(RepositoryFormatEnum::PYTHON),
           "KFP" => Ok(RepositoryFormatEnum::KFP),
           "GO" => Ok(RepositoryFormatEnum::GO),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RepositoryFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RepositoryModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Optional. The mode of the repository.
pub enum RepositoryModeEnum {
    

    /// Unspecified mode.
    ///
    /// "MODE_UNSPECIFIED"
    #[serde(rename="MODE_UNSPECIFIED")]
    MODEUNSPECIFIED,
    

    /// A standard repository storing artifacts.
    ///
    /// "STANDARD_REPOSITORY"
    #[serde(rename="STANDARD_REPOSITORY")]
    STANDARDREPOSITORY,
    

    /// A virtual repository to serve artifacts from one or more sources.
    ///
    /// "VIRTUAL_REPOSITORY"
    #[serde(rename="VIRTUAL_REPOSITORY")]
    VIRTUALREPOSITORY,
    

    /// A remote repository to serve artifacts from a remote source.
    ///
    /// "REMOTE_REPOSITORY"
    #[serde(rename="REMOTE_REPOSITORY")]
    REMOTEREPOSITORY,
    

    /// An AOSS repository provides artifacts from AOSS upstreams.
    ///
    /// "AOSS_REPOSITORY"
    #[serde(rename="AOSS_REPOSITORY")]
    AOSSREPOSITORY,
}

impl AsRef<str> for RepositoryModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RepositoryModeEnum::MODEUNSPECIFIED => "MODE_UNSPECIFIED",
            RepositoryModeEnum::STANDARDREPOSITORY => "STANDARD_REPOSITORY",
            RepositoryModeEnum::VIRTUALREPOSITORY => "VIRTUAL_REPOSITORY",
            RepositoryModeEnum::REMOTEREPOSITORY => "REMOTE_REPOSITORY",
            RepositoryModeEnum::AOSSREPOSITORY => "AOSS_REPOSITORY",
        }
    }
}

impl std::convert::TryFrom< &str> for RepositoryModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "MODE_UNSPECIFIED" => Ok(RepositoryModeEnum::MODEUNSPECIFIED),
           "STANDARD_REPOSITORY" => Ok(RepositoryModeEnum::STANDARDREPOSITORY),
           "VIRTUAL_REPOSITORY" => Ok(RepositoryModeEnum::VIRTUALREPOSITORY),
           "REMOTE_REPOSITORY" => Ok(RepositoryModeEnum::REMOTEREPOSITORY),
           "AOSS_REPOSITORY" => Ok(RepositoryModeEnum::AOSSREPOSITORY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RepositoryModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region VPCSCConfigVpcscPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The project per location VPC SC policy that defines the VPC SC behavior for the Remote Repository (Allow/Deny).
pub enum VPCSCConfigVpcscPolicyEnum {
    

    /// VPCSC_POLICY_UNSPECIFIED - the VPS SC policy is not defined. When VPS SC policy is not defined - the Service will use the default behavior (VPCSC_DENY).
    ///
    /// "VPCSC_POLICY_UNSPECIFIED"
    #[serde(rename="VPCSC_POLICY_UNSPECIFIED")]
    VPCSCPOLICYUNSPECIFIED,
    

    /// VPCSC_DENY - repository will block the requests to the Upstreams for the Remote Repositories if the resource is in the perimeter.
    ///
    /// "DENY"
    #[serde(rename="DENY")]
    DENY,
    

    /// VPCSC_ALLOW - repository will allow the requests to the Upstreams for the Remote Repositories if the resource is in the perimeter.
    ///
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
}

impl AsRef<str> for VPCSCConfigVpcscPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VPCSCConfigVpcscPolicyEnum::VPCSCPOLICYUNSPECIFIED => "VPCSC_POLICY_UNSPECIFIED",
            VPCSCConfigVpcscPolicyEnum::DENY => "DENY",
            VPCSCConfigVpcscPolicyEnum::ALLOW => "ALLOW",
        }
    }
}

impl std::convert::TryFrom< &str> for VPCSCConfigVpcscPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VPCSC_POLICY_UNSPECIFIED" => Ok(VPCSCConfigVpcscPolicyEnum::VPCSCPOLICYUNSPECIFIED),
           "DENY" => Ok(VPCSCConfigVpcscPolicyEnum::DENY),
           "ALLOW" => Ok(VPCSCConfigVpcscPolicyEnum::ALLOW),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VPCSCConfigVpcscPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProjectViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The view that should be returned in the response.
pub enum ProjectViewEnum {
    

    /// The default / unset value. The API will default to the BASIC view.
    ///
    /// "VERSION_VIEW_UNSPECIFIED"
    #[serde(rename="VERSION_VIEW_UNSPECIFIED")]
    VERSIONVIEWUNSPECIFIED,
    

    /// Includes basic information about the version, but not any related tags.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Include everything.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for ProjectViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProjectViewEnum::VERSIONVIEWUNSPECIFIED => "VERSION_VIEW_UNSPECIFIED",
            ProjectViewEnum::BASIC => "BASIC",
            ProjectViewEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for ProjectViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VERSION_VIEW_UNSPECIFIED" => Ok(ProjectViewEnum::VERSIONVIEWUNSPECIFIED),
           "BASIC" => Ok(ProjectViewEnum::BASIC),
           "FULL" => Ok(ProjectViewEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProjectViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


