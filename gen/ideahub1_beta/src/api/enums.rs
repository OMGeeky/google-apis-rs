use super::*;



// region GoogleSearchIdeahubV1betaIdeaActivityTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of activity performed.
pub enum GoogleSearchIdeahubV1betaIdeaActivityTypeEnum {
    

    /// An unspecified, unknown type of idea activity.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// An idea activity type indicating a post has been drafted.
    ///
    /// "POST_DRAFTED"
    #[serde(rename="POST_DRAFTED")]
    POSTDRAFTED,
    

    /// An idea activity type indicating a post has been published.
    ///
    /// "POST_PUBLISHED"
    #[serde(rename="POST_PUBLISHED")]
    POSTPUBLISHED,
    

    /// An idea activity type indicating a post has been deleted.
    ///
    /// "POST_DELETED"
    #[serde(rename="POST_DELETED")]
    POSTDELETED,
    

    /// An idea activity type indicating a post has been unpublished.
    ///
    /// "POST_UNPUBLISHED"
    #[serde(rename="POST_UNPUBLISHED")]
    POSTUNPUBLISHED,
}

impl AsRef<str> for GoogleSearchIdeahubV1betaIdeaActivityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTDRAFTED => "POST_DRAFTED",
            GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTPUBLISHED => "POST_PUBLISHED",
            GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTDELETED => "POST_DELETED",
            GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTUNPUBLISHED => "POST_UNPUBLISHED",
        }
    }
}

impl std::convert::TryFrom< &str> for GoogleSearchIdeahubV1betaIdeaActivityTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::TYPEUNSPECIFIED),
           "POST_DRAFTED" => Ok(GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTDRAFTED),
           "POST_PUBLISHED" => Ok(GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTPUBLISHED),
           "POST_DELETED" => Ok(GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTDELETED),
           "POST_UNPUBLISHED" => Ok(GoogleSearchIdeahubV1betaIdeaActivityTypeEnum::POSTUNPUBLISHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GoogleSearchIdeahubV1betaIdeaActivityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


