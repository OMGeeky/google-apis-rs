use super::*;



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


