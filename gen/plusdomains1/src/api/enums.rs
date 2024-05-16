use super::*;



// region ActivityCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The collection of activities to list.
pub enum ActivityCollectionEnum {
    

    /// All activities created by the specified user that the authenticated user is authorized to view.
    ///
    /// "user"
    #[serde(rename="user")]
    User,
}

impl AsRef<str> for ActivityCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityCollectionEnum::User => "user",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "user" => Ok(ActivityCollectionEnum::User),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommentSortOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order in which to sort the list of comments.
pub enum CommentSortOrderEnum {
    

    /// Sort oldest comments first.
    ///
    /// "ascending"
    #[serde(rename="ascending")]
    Ascending,
    

    /// Sort newest comments first.
    ///
    /// "descending"
    #[serde(rename="descending")]
    Descending,
}

impl AsRef<str> for CommentSortOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentSortOrderEnum::Ascending => "ascending",
            CommentSortOrderEnum::Descending => "descending",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentSortOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ascending" => Ok(CommentSortOrderEnum::Ascending),
           "descending" => Ok(CommentSortOrderEnum::Descending),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentSortOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentSortOrderEnum {
    fn default() -> CommentSortOrderEnum {
        CommentSortOrderEnum::Ascending
    }
}

// endregion


// region MediaCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum MediaCollectionEnum {
    

    /// Upload the media to share on Google+.
    ///
    /// "cloud"
    #[serde(rename="cloud")]
    Cloud,
}

impl AsRef<str> for MediaCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MediaCollectionEnum::Cloud => "cloud",
        }
    }
}

impl std::convert::TryFrom< &str> for MediaCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "cloud" => Ok(MediaCollectionEnum::Cloud),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MediaCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The collection of people to list.
pub enum PersonCollectionEnum {
    

    /// List all people who have +1'd this activity.
    ///
    /// "plusoners"
    #[serde(rename="plusoners")]
    Plusoners,
    

    /// List all people who have reshared this activity.
    ///
    /// "resharers"
    #[serde(rename="resharers")]
    Resharers,
    

    /// List all people who this activity was shared to.
    ///
    /// "sharedto"
    #[serde(rename="sharedto")]
    Sharedto,
}

impl AsRef<str> for PersonCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonCollectionEnum::Plusoners => "plusoners",
            PersonCollectionEnum::Resharers => "resharers",
            PersonCollectionEnum::Sharedto => "sharedto",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "plusoners" => Ok(PersonCollectionEnum::Plusoners),
           "resharers" => Ok(PersonCollectionEnum::Resharers),
           "sharedto" => Ok(PersonCollectionEnum::Sharedto),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PersonOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order to return people in.
pub enum PersonOrderByEnum {
    

    /// Order the people by their display name.
    ///
    /// "alphabetical"
    #[serde(rename="alphabetical")]
    Alphabetical,
    

    /// Order people based on the relevence to the viewer.
    ///
    /// "best"
    #[serde(rename="best")]
    Best,
}

impl AsRef<str> for PersonOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonOrderByEnum::Alphabetical => "alphabetical",
            PersonOrderByEnum::Best => "best",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "alphabetical" => Ok(PersonOrderByEnum::Alphabetical),
           "best" => Ok(PersonOrderByEnum::Best),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PersonOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


