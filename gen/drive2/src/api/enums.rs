use super::*;



// region FileVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The visibility of the new file. This parameter is only relevant when convert=false.
pub enum FileVisibilityEnum {
    

    /// The visibility of the new file is determined by the user's default visibility/sharing policies.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// The new file will be visible to only the owner.
    ///
    /// "PRIVATE"
    #[serde(rename="PRIVATE")]
    PRIVATE,
}

impl AsRef<str> for FileVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileVisibilityEnum::DEFAULT => "DEFAULT",
            FileVisibilityEnum::PRIVATE => "PRIVATE",
        }
    }
}

impl std::convert::TryFrom< &str> for FileVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(FileVisibilityEnum::DEFAULT),
           "PRIVATE" => Ok(FileVisibilityEnum::PRIVATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for FileVisibilityEnum {
    fn default() -> FileVisibilityEnum {
        FileVisibilityEnum::DEFAULT
    }
}

// endregion


// region FileProjectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated: This parameter has no function.
pub enum FileProjectionEnum {
    

    /// Deprecated.
    ///
    /// "BASIC"
    #[serde(rename="BASIC")]
    BASIC,
    

    /// Deprecated.
    ///
    /// "FULL"
    #[serde(rename="FULL")]
    FULL,
}

impl AsRef<str> for FileProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileProjectionEnum::BASIC => "BASIC",
            FileProjectionEnum::FULL => "FULL",
        }
    }
}

impl std::convert::TryFrom< &str> for FileProjectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "BASIC" => Ok(FileProjectionEnum::BASIC),
           "FULL" => Ok(FileProjectionEnum::FULL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FileCorpusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated: The body of items (files/documents) to which the query applies. Use `corpora` instead.
pub enum FileCorpusEnum {
    

    /// The items that the user has accessed.
    ///
    /// "DEFAULT"
    #[serde(rename="DEFAULT")]
    DEFAULT,
    

    /// Items shared to the user's domain.
    ///
    /// "DOMAIN"
    #[serde(rename="DOMAIN")]
    DOMAIN,
}

impl AsRef<str> for FileCorpusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileCorpusEnum::DEFAULT => "DEFAULT",
            FileCorpusEnum::DOMAIN => "DOMAIN",
        }
    }
}

impl std::convert::TryFrom< &str> for FileCorpusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DEFAULT" => Ok(FileCorpusEnum::DEFAULT),
           "DOMAIN" => Ok(FileCorpusEnum::DOMAIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileCorpusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FileModifiedDateBehaviorEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Determines the behavior in which `modifiedDate` is updated. This overrides `setModifiedDate`.
pub enum FileModifiedDateBehaviorEnum {
    

    /// Set `modifiedDate` to the value provided in the body of the request. No change if no value was provided.
    ///
    /// "fromBody"
    #[serde(rename="fromBody")]
    FromBody,
    

    /// Set `modifiedDate` to the value provided in the body of the request depending on other contents of the update.
    ///
    /// "fromBodyIfNeeded"
    #[serde(rename="fromBodyIfNeeded")]
    FromBodyIfNeeded,
    

    /// Set modifiedDate to the value provided in the body of the request, or to the current time if no value was provided.
    ///
    /// "fromBodyOrNow"
    #[serde(rename="fromBodyOrNow")]
    FromBodyOrNow,
    

    /// Maintain the previous value of `modifiedDate`.
    ///
    /// "noChange"
    #[serde(rename="noChange")]
    NoChange,
    

    /// Set `modifiedDate` to the current time.
    ///
    /// "now"
    #[serde(rename="now")]
    Now,
    

    /// Set `modifiedDate` to the current time depending on contents of the update.
    ///
    /// "nowIfNeeded"
    #[serde(rename="nowIfNeeded")]
    NowIfNeeded,
}

impl AsRef<str> for FileModifiedDateBehaviorEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileModifiedDateBehaviorEnum::FromBody => "fromBody",
            FileModifiedDateBehaviorEnum::FromBodyIfNeeded => "fromBodyIfNeeded",
            FileModifiedDateBehaviorEnum::FromBodyOrNow => "fromBodyOrNow",
            FileModifiedDateBehaviorEnum::NoChange => "noChange",
            FileModifiedDateBehaviorEnum::Now => "now",
            FileModifiedDateBehaviorEnum::NowIfNeeded => "nowIfNeeded",
        }
    }
}

impl std::convert::TryFrom< &str> for FileModifiedDateBehaviorEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "fromBody" => Ok(FileModifiedDateBehaviorEnum::FromBody),
           "fromBodyIfNeeded" => Ok(FileModifiedDateBehaviorEnum::FromBodyIfNeeded),
           "fromBodyOrNow" => Ok(FileModifiedDateBehaviorEnum::FromBodyOrNow),
           "noChange" => Ok(FileModifiedDateBehaviorEnum::NoChange),
           "now" => Ok(FileModifiedDateBehaviorEnum::Now),
           "nowIfNeeded" => Ok(FileModifiedDateBehaviorEnum::NowIfNeeded),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FileModifiedDateBehaviorEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


