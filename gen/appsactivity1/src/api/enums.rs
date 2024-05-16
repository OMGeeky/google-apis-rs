use super::*;



// region EventAdditionalEventTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Additional event types. Some events may have multiple types when multiple actions are part of a single event. For example, creating a document, renaming it, and sharing it may be part of a single file-creation event.
pub enum EventAdditionalEventTypesEnum {
    
    /// "comment"
    #[serde(rename="comment")]
    Comment,
    
    /// "create"
    #[serde(rename="create")]
    Create,
    
    /// "edit"
    #[serde(rename="edit")]
    Edit,
    
    /// "emptyTrash"
    #[serde(rename="emptyTrash")]
    EmptyTrash,
    
    /// "move"
    #[serde(rename="move")]
    Move,
    
    /// "permissionChange"
    #[serde(rename="permissionChange")]
    PermissionChange,
    
    /// "rename"
    #[serde(rename="rename")]
    Rename,
    
    /// "trash"
    #[serde(rename="trash")]
    Trash,
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "untrash"
    #[serde(rename="untrash")]
    Untrash,
    
    /// "upload"
    #[serde(rename="upload")]
    Upload,
}

impl AsRef<str> for EventAdditionalEventTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventAdditionalEventTypesEnum::Comment => "comment",
            EventAdditionalEventTypesEnum::Create => "create",
            EventAdditionalEventTypesEnum::Edit => "edit",
            EventAdditionalEventTypesEnum::EmptyTrash => "emptyTrash",
            EventAdditionalEventTypesEnum::Move => "move",
            EventAdditionalEventTypesEnum::PermissionChange => "permissionChange",
            EventAdditionalEventTypesEnum::Rename => "rename",
            EventAdditionalEventTypesEnum::Trash => "trash",
            EventAdditionalEventTypesEnum::Unknown => "unknown",
            EventAdditionalEventTypesEnum::Untrash => "untrash",
            EventAdditionalEventTypesEnum::Upload => "upload",
        }
    }
}

impl std::convert::TryFrom< &str> for EventAdditionalEventTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "comment" => Ok(EventAdditionalEventTypesEnum::Comment),
           "create" => Ok(EventAdditionalEventTypesEnum::Create),
           "edit" => Ok(EventAdditionalEventTypesEnum::Edit),
           "emptyTrash" => Ok(EventAdditionalEventTypesEnum::EmptyTrash),
           "move" => Ok(EventAdditionalEventTypesEnum::Move),
           "permissionChange" => Ok(EventAdditionalEventTypesEnum::PermissionChange),
           "rename" => Ok(EventAdditionalEventTypesEnum::Rename),
           "trash" => Ok(EventAdditionalEventTypesEnum::Trash),
           "unknown" => Ok(EventAdditionalEventTypesEnum::Unknown),
           "untrash" => Ok(EventAdditionalEventTypesEnum::Untrash),
           "upload" => Ok(EventAdditionalEventTypesEnum::Upload),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventAdditionalEventTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventPrimaryEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The main type of event that occurred.
pub enum EventPrimaryEventTypeEnum {
    
    /// "comment"
    #[serde(rename="comment")]
    Comment,
    
    /// "create"
    #[serde(rename="create")]
    Create,
    
    /// "edit"
    #[serde(rename="edit")]
    Edit,
    
    /// "emptyTrash"
    #[serde(rename="emptyTrash")]
    EmptyTrash,
    
    /// "move"
    #[serde(rename="move")]
    Move,
    
    /// "permissionChange"
    #[serde(rename="permissionChange")]
    PermissionChange,
    
    /// "rename"
    #[serde(rename="rename")]
    Rename,
    
    /// "trash"
    #[serde(rename="trash")]
    Trash,
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "untrash"
    #[serde(rename="untrash")]
    Untrash,
    
    /// "upload"
    #[serde(rename="upload")]
    Upload,
}

impl AsRef<str> for EventPrimaryEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventPrimaryEventTypeEnum::Comment => "comment",
            EventPrimaryEventTypeEnum::Create => "create",
            EventPrimaryEventTypeEnum::Edit => "edit",
            EventPrimaryEventTypeEnum::EmptyTrash => "emptyTrash",
            EventPrimaryEventTypeEnum::Move => "move",
            EventPrimaryEventTypeEnum::PermissionChange => "permissionChange",
            EventPrimaryEventTypeEnum::Rename => "rename",
            EventPrimaryEventTypeEnum::Trash => "trash",
            EventPrimaryEventTypeEnum::Unknown => "unknown",
            EventPrimaryEventTypeEnum::Untrash => "untrash",
            EventPrimaryEventTypeEnum::Upload => "upload",
        }
    }
}

impl std::convert::TryFrom< &str> for EventPrimaryEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "comment" => Ok(EventPrimaryEventTypeEnum::Comment),
           "create" => Ok(EventPrimaryEventTypeEnum::Create),
           "edit" => Ok(EventPrimaryEventTypeEnum::Edit),
           "emptyTrash" => Ok(EventPrimaryEventTypeEnum::EmptyTrash),
           "move" => Ok(EventPrimaryEventTypeEnum::Move),
           "permissionChange" => Ok(EventPrimaryEventTypeEnum::PermissionChange),
           "rename" => Ok(EventPrimaryEventTypeEnum::Rename),
           "trash" => Ok(EventPrimaryEventTypeEnum::Trash),
           "unknown" => Ok(EventPrimaryEventTypeEnum::Unknown),
           "untrash" => Ok(EventPrimaryEventTypeEnum::Untrash),
           "upload" => Ok(EventPrimaryEventTypeEnum::Upload),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventPrimaryEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the Google Drive permissions role. The role determines a user's ability to read, write, or comment on the file.
pub enum PermissionRoleEnum {
    
    /// "commenter"
    #[serde(rename="commenter")]
    Commenter,
    
    /// "fileOrganizer"
    #[serde(rename="fileOrganizer")]
    FileOrganizer,
    
    /// "owner"
    #[serde(rename="owner")]
    Owner,
    
    /// "publishedReader"
    #[serde(rename="publishedReader")]
    PublishedReader,
    
    /// "reader"
    #[serde(rename="reader")]
    Reader,
    
    /// "writer"
    #[serde(rename="writer")]
    Writer,
}

impl AsRef<str> for PermissionRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionRoleEnum::Commenter => "commenter",
            PermissionRoleEnum::FileOrganizer => "fileOrganizer",
            PermissionRoleEnum::Owner => "owner",
            PermissionRoleEnum::PublishedReader => "publishedReader",
            PermissionRoleEnum::Reader => "reader",
            PermissionRoleEnum::Writer => "writer",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "commenter" => Ok(PermissionRoleEnum::Commenter),
           "fileOrganizer" => Ok(PermissionRoleEnum::FileOrganizer),
           "owner" => Ok(PermissionRoleEnum::Owner),
           "publishedReader" => Ok(PermissionRoleEnum::PublishedReader),
           "reader" => Ok(PermissionRoleEnum::Reader),
           "writer" => Ok(PermissionRoleEnum::Writer),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates how widely permissions are granted.
pub enum PermissionTypeEnum {
    
    /// "anyone"
    #[serde(rename="anyone")]
    Anyone,
    
    /// "domain"
    #[serde(rename="domain")]
    Domain,
    
    /// "group"
    #[serde(rename="group")]
    Group,
    
    /// "user"
    #[serde(rename="user")]
    User,
}

impl AsRef<str> for PermissionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionTypeEnum::Anyone => "anyone",
            PermissionTypeEnum::Domain => "domain",
            PermissionTypeEnum::Group => "group",
            PermissionTypeEnum::User => "user",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "anyone" => Ok(PermissionTypeEnum::Anyone),
           "domain" => Ok(PermissionTypeEnum::Domain),
           "group" => Ok(PermissionTypeEnum::Group),
           "user" => Ok(PermissionTypeEnum::User),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PermissionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ActivityGroupingStrategyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the strategy to use when grouping singleEvents items in the associated combinedEvent object.
pub enum ActivityGroupingStrategyEnum {
    
    /// "driveUi"
    #[serde(rename="driveUi")]
    DriveUi,
    
    /// "none"
    #[serde(rename="none")]
    None,
}

impl AsRef<str> for ActivityGroupingStrategyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityGroupingStrategyEnum::DriveUi => "driveUi",
            ActivityGroupingStrategyEnum::None => "none",
        }
    }
}

impl std::convert::TryFrom< &str> for ActivityGroupingStrategyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "driveUi" => Ok(ActivityGroupingStrategyEnum::DriveUi),
           "none" => Ok(ActivityGroupingStrategyEnum::None),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityGroupingStrategyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for ActivityGroupingStrategyEnum {
    fn default() -> ActivityGroupingStrategyEnum {
        ActivityGroupingStrategyEnum::DriveUi
    }
}

// endregion


