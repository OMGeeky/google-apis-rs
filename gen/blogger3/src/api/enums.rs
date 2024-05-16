use super::*;



// region BlogStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the blog.
pub enum BlogStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for BlogStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlogStatusEnum::LIVE => "LIVE",
            BlogStatusEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for BlogStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(BlogStatusEnum::LIVE),
           "DELETED" => Ok(BlogStatusEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlogStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlogPerUserInfoRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Access permissions that the user has for the blog (ADMIN, AUTHOR, or READER).
pub enum BlogPerUserInfoRoleEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for BlogPerUserInfoRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlogPerUserInfoRoleEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            BlogPerUserInfoRoleEnum::READER => "READER",
            BlogPerUserInfoRoleEnum::AUTHOR => "AUTHOR",
            BlogPerUserInfoRoleEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for BlogPerUserInfoRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(BlogPerUserInfoRoleEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(BlogPerUserInfoRoleEnum::READER),
           "AUTHOR" => Ok(BlogPerUserInfoRoleEnum::AUTHOR),
           "ADMIN" => Ok(BlogPerUserInfoRoleEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlogPerUserInfoRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommentStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the comment (only populated for admin users).
pub enum CommentStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "EMPTIED"
    #[serde(rename="EMPTIED")]
    EMPTIED,
    
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    
    /// "SPAM"
    #[serde(rename="SPAM")]
    SPAM,
}

impl AsRef<str> for CommentStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentStatusEnum::LIVE => "LIVE",
            CommentStatusEnum::EMPTIED => "EMPTIED",
            CommentStatusEnum::PENDING => "PENDING",
            CommentStatusEnum::SPAM => "SPAM",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(CommentStatusEnum::LIVE),
           "EMPTIED" => Ok(CommentStatusEnum::EMPTIED),
           "PENDING" => Ok(CommentStatusEnum::PENDING),
           "SPAM" => Ok(CommentStatusEnum::SPAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PageStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the page for admin resources (either LIVE or DRAFT).
pub enum PageStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    
    /// "SOFT_TRASHED"
    #[serde(rename="SOFT_TRASHED")]
    SOFTTRASHED,
}

impl AsRef<str> for PageStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PageStatusEnum::LIVE => "LIVE",
            PageStatusEnum::DRAFT => "DRAFT",
            PageStatusEnum::SOFTTRASHED => "SOFT_TRASHED",
        }
    }
}

impl std::convert::TryFrom< &str> for PageStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(PageStatusEnum::LIVE),
           "DRAFT" => Ok(PageStatusEnum::DRAFT),
           "SOFT_TRASHED" => Ok(PageStatusEnum::SOFTTRASHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PageStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostReaderCommentsEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Comment control and display setting for readers of this post.
pub enum PostReaderCommentsEnum {
    
    /// "ALLOW"
    #[serde(rename="ALLOW")]
    ALLOW,
    
    /// "DONT_ALLOW_SHOW_EXISTING"
    #[serde(rename="DONT_ALLOW_SHOW_EXISTING")]
    DONTALLOWSHOWEXISTING,
    
    /// "DONT_ALLOW_HIDE_EXISTING"
    #[serde(rename="DONT_ALLOW_HIDE_EXISTING")]
    DONTALLOWHIDEEXISTING,
}

impl AsRef<str> for PostReaderCommentsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostReaderCommentsEnum::ALLOW => "ALLOW",
            PostReaderCommentsEnum::DONTALLOWSHOWEXISTING => "DONT_ALLOW_SHOW_EXISTING",
            PostReaderCommentsEnum::DONTALLOWHIDEEXISTING => "DONT_ALLOW_HIDE_EXISTING",
        }
    }
}

impl std::convert::TryFrom< &str> for PostReaderCommentsEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALLOW" => Ok(PostReaderCommentsEnum::ALLOW),
           "DONT_ALLOW_SHOW_EXISTING" => Ok(PostReaderCommentsEnum::DONTALLOWSHOWEXISTING),
           "DONT_ALLOW_HIDE_EXISTING" => Ok(PostReaderCommentsEnum::DONTALLOWHIDEEXISTING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostReaderCommentsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of the post. Only set for admin-level requests.
pub enum PostStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
    
    /// "SOFT_TRASHED"
    #[serde(rename="SOFT_TRASHED")]
    SOFTTRASHED,
}

impl AsRef<str> for PostStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostStatusEnum::LIVE => "LIVE",
            PostStatusEnum::DRAFT => "DRAFT",
            PostStatusEnum::SCHEDULED => "SCHEDULED",
            PostStatusEnum::SOFTTRASHED => "SOFT_TRASHED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(PostStatusEnum::LIVE),
           "DRAFT" => Ok(PostStatusEnum::DRAFT),
           "SCHEDULED" => Ok(PostStatusEnum::SCHEDULED),
           "SOFT_TRASHED" => Ok(PostStatusEnum::SOFTTRASHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PageviewsCountTimeRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Time range the given count applies to.
pub enum PageviewsCountTimeRangeEnum {
    
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    
    /// "THIRTY_DAYS"
    #[serde(rename="THIRTY_DAYS")]
    THIRTYDAYS,
    
    /// "SEVEN_DAYS"
    #[serde(rename="SEVEN_DAYS")]
    SEVENDAYS,
}

impl AsRef<str> for PageviewsCountTimeRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PageviewsCountTimeRangeEnum::ALLTIME => "ALL_TIME",
            PageviewsCountTimeRangeEnum::THIRTYDAYS => "THIRTY_DAYS",
            PageviewsCountTimeRangeEnum::SEVENDAYS => "SEVEN_DAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for PageviewsCountTimeRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_TIME" => Ok(PageviewsCountTimeRangeEnum::ALLTIME),
           "THIRTY_DAYS" => Ok(PageviewsCountTimeRangeEnum::THIRTYDAYS),
           "SEVEN_DAYS" => Ok(PageviewsCountTimeRangeEnum::SEVENDAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PageviewsCountTimeRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlogViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlogViewEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for BlogViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlogViewEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            BlogViewEnum::READER => "READER",
            BlogViewEnum::AUTHOR => "AUTHOR",
            BlogViewEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for BlogViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(BlogViewEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(BlogViewEnum::READER),
           "AUTHOR" => Ok(BlogViewEnum::AUTHOR),
           "ADMIN" => Ok(BlogViewEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlogViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlogRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlogRoleEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for BlogRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlogRoleEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            BlogRoleEnum::READER => "READER",
            BlogRoleEnum::AUTHOR => "AUTHOR",
            BlogRoleEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for BlogRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(BlogRoleEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(BlogRoleEnum::READER),
           "AUTHOR" => Ok(BlogRoleEnum::AUTHOR),
           "ADMIN" => Ok(BlogRoleEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlogRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region BlogStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Default value of status is LIVE.
pub enum BlogStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for BlogStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            BlogStatusEnum::LIVE => "LIVE",
            BlogStatusEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for BlogStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(BlogStatusEnum::LIVE),
           "DELETED" => Ok(BlogStatusEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a BlogStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for BlogStatusEnum {
    fn default() -> BlogStatusEnum {
        BlogStatusEnum::LIVE
    }
}

// endregion


// region CommentViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommentViewEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for CommentViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentViewEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            CommentViewEnum::READER => "READER",
            CommentViewEnum::AUTHOR => "AUTHOR",
            CommentViewEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(CommentViewEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(CommentViewEnum::READER),
           "AUTHOR" => Ok(CommentViewEnum::AUTHOR),
           "ADMIN" => Ok(CommentViewEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CommentStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommentStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "EMPTIED"
    #[serde(rename="EMPTIED")]
    EMPTIED,
    
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    
    /// "SPAM"
    #[serde(rename="SPAM")]
    SPAM,
}

impl AsRef<str> for CommentStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentStatusEnum::LIVE => "LIVE",
            CommentStatusEnum::EMPTIED => "EMPTIED",
            CommentStatusEnum::PENDING => "PENDING",
            CommentStatusEnum::SPAM => "SPAM",
        }
    }
}

impl std::convert::TryFrom< &str> for CommentStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(CommentStatusEnum::LIVE),
           "EMPTIED" => Ok(CommentStatusEnum::EMPTIED),
           "PENDING" => Ok(CommentStatusEnum::PENDING),
           "SPAM" => Ok(CommentStatusEnum::SPAM),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PageViewRangeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PageViewRangeEnum {
    
    /// "all"
    #[serde(rename="all")]
    All,
    
    /// "30DAYS"
    #[serde(rename="30DAYS")]
    _30DAYS,
    
    /// "7DAYS"
    #[serde(rename="7DAYS")]
    _7DAYS,
}

impl AsRef<str> for PageViewRangeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PageViewRangeEnum::All => "all",
            PageViewRangeEnum::_30DAYS => "30DAYS",
            PageViewRangeEnum::_7DAYS => "7DAYS",
        }
    }
}

impl std::convert::TryFrom< &str> for PageViewRangeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "all" => Ok(PageViewRangeEnum::All),
           "30DAYS" => Ok(PageViewRangeEnum::_30DAYS),
           "7DAYS" => Ok(PageViewRangeEnum::_7DAYS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PageViewRangeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PageViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PageViewEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for PageViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PageViewEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            PageViewEnum::READER => "READER",
            PageViewEnum::AUTHOR => "AUTHOR",
            PageViewEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for PageViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(PageViewEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(PageViewEnum::READER),
           "AUTHOR" => Ok(PageViewEnum::AUTHOR),
           "ADMIN" => Ok(PageViewEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PageViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PageStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PageStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    
    /// "SOFT_TRASHED"
    #[serde(rename="SOFT_TRASHED")]
    SOFTTRASHED,
}

impl AsRef<str> for PageStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PageStatusEnum::LIVE => "LIVE",
            PageStatusEnum::DRAFT => "DRAFT",
            PageStatusEnum::SOFTTRASHED => "SOFT_TRASHED",
        }
    }
}

impl std::convert::TryFrom< &str> for PageStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(PageStatusEnum::LIVE),
           "DRAFT" => Ok(PageStatusEnum::DRAFT),
           "SOFT_TRASHED" => Ok(PageStatusEnum::SOFTTRASHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PageStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostUserInfoOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostUserInfoOrderByEnum {
    
    /// "ORDER_BY_UNSPECIFIED"
    #[serde(rename="ORDER_BY_UNSPECIFIED")]
    ORDERBYUNSPECIFIED,
    
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    
    /// "UPDATED"
    #[serde(rename="UPDATED")]
    UPDATED,
}

impl AsRef<str> for PostUserInfoOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostUserInfoOrderByEnum::ORDERBYUNSPECIFIED => "ORDER_BY_UNSPECIFIED",
            PostUserInfoOrderByEnum::PUBLISHED => "PUBLISHED",
            PostUserInfoOrderByEnum::UPDATED => "UPDATED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostUserInfoOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_BY_UNSPECIFIED" => Ok(PostUserInfoOrderByEnum::ORDERBYUNSPECIFIED),
           "PUBLISHED" => Ok(PostUserInfoOrderByEnum::PUBLISHED),
           "UPDATED" => Ok(PostUserInfoOrderByEnum::UPDATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostUserInfoOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PostUserInfoOrderByEnum {
    fn default() -> PostUserInfoOrderByEnum {
        PostUserInfoOrderByEnum::PUBLISHED
    }
}

// endregion


// region PostUserInfoStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostUserInfoStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
    
    /// "SOFT_TRASHED"
    #[serde(rename="SOFT_TRASHED")]
    SOFTTRASHED,
}

impl AsRef<str> for PostUserInfoStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostUserInfoStatusEnum::LIVE => "LIVE",
            PostUserInfoStatusEnum::DRAFT => "DRAFT",
            PostUserInfoStatusEnum::SCHEDULED => "SCHEDULED",
            PostUserInfoStatusEnum::SOFTTRASHED => "SOFT_TRASHED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostUserInfoStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(PostUserInfoStatusEnum::LIVE),
           "DRAFT" => Ok(PostUserInfoStatusEnum::DRAFT),
           "SCHEDULED" => Ok(PostUserInfoStatusEnum::SCHEDULED),
           "SOFT_TRASHED" => Ok(PostUserInfoStatusEnum::SOFTTRASHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostUserInfoStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostUserInfoViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostUserInfoViewEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for PostUserInfoViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostUserInfoViewEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            PostUserInfoViewEnum::READER => "READER",
            PostUserInfoViewEnum::AUTHOR => "AUTHOR",
            PostUserInfoViewEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for PostUserInfoViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(PostUserInfoViewEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(PostUserInfoViewEnum::READER),
           "AUTHOR" => Ok(PostUserInfoViewEnum::AUTHOR),
           "ADMIN" => Ok(PostUserInfoViewEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostUserInfoViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostViewEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostViewEnum {
    
    /// "VIEW_TYPE_UNSPECIFIED"
    #[serde(rename="VIEW_TYPE_UNSPECIFIED")]
    VIEWTYPEUNSPECIFIED,
    
    /// "READER"
    #[serde(rename="READER")]
    READER,
    
    /// "AUTHOR"
    #[serde(rename="AUTHOR")]
    AUTHOR,
    
    /// "ADMIN"
    #[serde(rename="ADMIN")]
    ADMIN,
}

impl AsRef<str> for PostViewEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostViewEnum::VIEWTYPEUNSPECIFIED => "VIEW_TYPE_UNSPECIFIED",
            PostViewEnum::READER => "READER",
            PostViewEnum::AUTHOR => "AUTHOR",
            PostViewEnum::ADMIN => "ADMIN",
        }
    }
}

impl std::convert::TryFrom< &str> for PostViewEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VIEW_TYPE_UNSPECIFIED" => Ok(PostViewEnum::VIEWTYPEUNSPECIFIED),
           "READER" => Ok(PostViewEnum::READER),
           "AUTHOR" => Ok(PostViewEnum::AUTHOR),
           "ADMIN" => Ok(PostViewEnum::ADMIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostViewEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PostOrderByEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostOrderByEnum {
    
    /// "ORDER_BY_UNSPECIFIED"
    #[serde(rename="ORDER_BY_UNSPECIFIED")]
    ORDERBYUNSPECIFIED,
    
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    
    /// "UPDATED"
    #[serde(rename="UPDATED")]
    UPDATED,
}

impl AsRef<str> for PostOrderByEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostOrderByEnum::ORDERBYUNSPECIFIED => "ORDER_BY_UNSPECIFIED",
            PostOrderByEnum::PUBLISHED => "PUBLISHED",
            PostOrderByEnum::UPDATED => "UPDATED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostOrderByEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ORDER_BY_UNSPECIFIED" => Ok(PostOrderByEnum::ORDERBYUNSPECIFIED),
           "PUBLISHED" => Ok(PostOrderByEnum::PUBLISHED),
           "UPDATED" => Ok(PostOrderByEnum::UPDATED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostOrderByEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PostOrderByEnum {
    fn default() -> PostOrderByEnum {
        PostOrderByEnum::PUBLISHED
    }
}

// endregion


// region PostSortOptionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort direction applied to post list.
pub enum PostSortOptionEnum {
    

    /// The unspecified sort option.
    ///
    /// "SORT_OPTION_UNSPECIFIED"
    #[serde(rename="SORT_OPTION_UNSPECIFIED")]
    SORTOPTIONUNSPECIFIED,
    

    /// The option to sort posts in descending order in time.
    ///
    /// "DESCENDING"
    #[serde(rename="DESCENDING")]
    DESCENDING,
    

    /// The option to sort posts in ascending order in time.
    ///
    /// "ASCENDING"
    #[serde(rename="ASCENDING")]
    ASCENDING,
}

impl AsRef<str> for PostSortOptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostSortOptionEnum::SORTOPTIONUNSPECIFIED => "SORT_OPTION_UNSPECIFIED",
            PostSortOptionEnum::DESCENDING => "DESCENDING",
            PostSortOptionEnum::ASCENDING => "ASCENDING",
        }
    }
}

impl std::convert::TryFrom< &str> for PostSortOptionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SORT_OPTION_UNSPECIFIED" => Ok(PostSortOptionEnum::SORTOPTIONUNSPECIFIED),
           "DESCENDING" => Ok(PostSortOptionEnum::DESCENDING),
           "ASCENDING" => Ok(PostSortOptionEnum::ASCENDING),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostSortOptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for PostSortOptionEnum {
    fn default() -> PostSortOptionEnum {
        PostSortOptionEnum::DESCENDING
    }
}

// endregion


// region PostStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum PostStatusEnum {
    
    /// "LIVE"
    #[serde(rename="LIVE")]
    LIVE,
    
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    
    /// "SCHEDULED"
    #[serde(rename="SCHEDULED")]
    SCHEDULED,
    
    /// "SOFT_TRASHED"
    #[serde(rename="SOFT_TRASHED")]
    SOFTTRASHED,
}

impl AsRef<str> for PostStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostStatusEnum::LIVE => "LIVE",
            PostStatusEnum::DRAFT => "DRAFT",
            PostStatusEnum::SCHEDULED => "SCHEDULED",
            PostStatusEnum::SOFTTRASHED => "SOFT_TRASHED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LIVE" => Ok(PostStatusEnum::LIVE),
           "DRAFT" => Ok(PostStatusEnum::DRAFT),
           "SCHEDULED" => Ok(PostStatusEnum::SCHEDULED),
           "SOFT_TRASHED" => Ok(PostStatusEnum::SOFTTRASHED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


