use super::*;



// region FileCorpusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Deprecated: The source of files to list. Use 'corpora' instead.
pub enum FileCorpusEnum {
    

    /// Files shared to the user's domain.
    ///
    /// "domain"
    #[serde(rename="domain")]
    Domain,
    

    /// Files owned by or shared to the user.
    ///
    /// "user"
    #[serde(rename="user")]
    User,
}

impl AsRef<str> for FileCorpusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FileCorpusEnum::Domain => "domain",
            FileCorpusEnum::User => "user",
        }
    }
}

impl std::convert::TryFrom< &str> for FileCorpusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "domain" => Ok(FileCorpusEnum::Domain),
           "user" => Ok(FileCorpusEnum::User),
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


