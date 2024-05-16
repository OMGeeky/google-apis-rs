use super::*;



// region ActivityCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The collection of activities to list.
pub enum ActivityCollectionEnum {
    

    /// All public activities created by the specified user.
    ///
    /// "public"
    #[serde(rename="public")]
    Public,
}

impl AsRef<str> for ActivityCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityCollectionEnum::Public => "public",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "public" => Ok(ActivityCollectionEnum::Public),
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


// region ActivityOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies how to order search results.
pub enum ActivityOrderByEnum {
    

    /// Sort activities by relevance to the user, most relevant first.
    ///
    /// "best"
    #[serde(rename="best")]
    Best,
    

    /// Sort activities by published date, most recent first.
    ///
    /// "recent"
    #[serde(rename="recent")]
    Recent,
}

impl AsRef<str> for ActivityOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityOrderByEnum::Best => "best",
            ActivityOrderByEnum::Recent => "recent",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "best" => Ok(ActivityOrderByEnum::Best),
           "recent" => Ok(ActivityOrderByEnum::Recent),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ActivityOrderByEnum {
    fn default() -> ActivityOrderByEnum {
        ActivityOrderByEnum::Recent
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
}

impl AsRef<str> for PersonCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PersonCollectionEnum::Plusoners => "plusoners",
            PersonCollectionEnum::Resharers => "resharers",
        }
    }
}

impl std::convert::TryFrom< &str> for PersonCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "plusoners" => Ok(PersonCollectionEnum::Plusoners),
           "resharers" => Ok(PersonCollectionEnum::Resharers),
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


