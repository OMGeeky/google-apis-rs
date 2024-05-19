use super::*;



// region UrlNotificationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The URL life cycle event that Google is being notified about.
pub enum UrlNotificationTypeEnum {
    

    /// Unspecified.
    ///
    /// "URL_NOTIFICATION_TYPE_UNSPECIFIED"
    #[serde(rename="URL_NOTIFICATION_TYPE_UNSPECIFIED")]
    URLNOTIFICATIONTYPEUNSPECIFIED,
    

    /// The given URL (Web document) has been updated.
    ///
    /// "URL_UPDATED"
    #[serde(rename="URL_UPDATED")]
    URLUPDATED,
    

    /// The given URL (Web document) has been deleted.
    ///
    /// "URL_DELETED"
    #[serde(rename="URL_DELETED")]
    URLDELETED,
}

impl AsRef<str> for UrlNotificationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UrlNotificationTypeEnum::URLNOTIFICATIONTYPEUNSPECIFIED => "URL_NOTIFICATION_TYPE_UNSPECIFIED",
            UrlNotificationTypeEnum::URLUPDATED => "URL_UPDATED",
            UrlNotificationTypeEnum::URLDELETED => "URL_DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for UrlNotificationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "URL_NOTIFICATION_TYPE_UNSPECIFIED" => Ok(UrlNotificationTypeEnum::URLNOTIFICATIONTYPEUNSPECIFIED),
           "URL_UPDATED" => Ok(UrlNotificationTypeEnum::URLUPDATED),
           "URL_DELETED" => Ok(UrlNotificationTypeEnum::URLDELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UrlNotificationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


