use super::*;



// region ApplicationReferenceTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reference type corresponding to this event.
pub enum ApplicationReferenceTypeEnum {
    

    /// The type is not available.
    ///
    /// "UNSPECIFIED_REFERENCE_TYPE"
    #[serde(rename="UNSPECIFIED_REFERENCE_TYPE")]
    UNSPECIFIEDREFERENCETYPE,
    

    /// The links of one or more Drive items were posted.
    ///
    /// "LINK"
    #[serde(rename="LINK")]
    LINK,
    

    /// Comments were made regarding a Drive item.
    ///
    /// "DISCUSS"
    #[serde(rename="DISCUSS")]
    DISCUSS,
}

impl AsRef<str> for ApplicationReferenceTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationReferenceTypeEnum::UNSPECIFIEDREFERENCETYPE => "UNSPECIFIED_REFERENCE_TYPE",
            ApplicationReferenceTypeEnum::LINK => "LINK",
            ApplicationReferenceTypeEnum::DISCUSS => "DISCUSS",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationReferenceTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNSPECIFIED_REFERENCE_TYPE" => Ok(ApplicationReferenceTypeEnum::UNSPECIFIEDREFERENCETYPE),
           "LINK" => Ok(ApplicationReferenceTypeEnum::LINK),
           "DISCUSS" => Ok(ApplicationReferenceTypeEnum::DISCUSS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationReferenceTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AppliedLabelChangeDetailTypesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of changes made to the Label on the Target.
pub enum AppliedLabelChangeDetailTypesEnum {
    

    /// The type of change to this Label is not available.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The identified Label was added to the Target.
    ///
    /// "LABEL_ADDED"
    #[serde(rename="LABEL_ADDED")]
    LABELADDED,
    

    /// The identified Label was removed from the Target.
    ///
    /// "LABEL_REMOVED"
    #[serde(rename="LABEL_REMOVED")]
    LABELREMOVED,
    

    /// Field values were changed on the Target.
    ///
    /// "LABEL_FIELD_VALUE_CHANGED"
    #[serde(rename="LABEL_FIELD_VALUE_CHANGED")]
    LABELFIELDVALUECHANGED,
    

    /// The Label was applied as a side-effect of Drive item creation.
    ///
    /// "LABEL_APPLIED_BY_ITEM_CREATE"
    #[serde(rename="LABEL_APPLIED_BY_ITEM_CREATE")]
    LABELAPPLIEDBYITEMCREATE,
}

impl AsRef<str> for AppliedLabelChangeDetailTypesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AppliedLabelChangeDetailTypesEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            AppliedLabelChangeDetailTypesEnum::LABELADDED => "LABEL_ADDED",
            AppliedLabelChangeDetailTypesEnum::LABELREMOVED => "LABEL_REMOVED",
            AppliedLabelChangeDetailTypesEnum::LABELFIELDVALUECHANGED => "LABEL_FIELD_VALUE_CHANGED",
            AppliedLabelChangeDetailTypesEnum::LABELAPPLIEDBYITEMCREATE => "LABEL_APPLIED_BY_ITEM_CREATE",
        }
    }
}

impl std::convert::TryFrom< &str> for AppliedLabelChangeDetailTypesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(AppliedLabelChangeDetailTypesEnum::TYPEUNSPECIFIED),
           "LABEL_ADDED" => Ok(AppliedLabelChangeDetailTypesEnum::LABELADDED),
           "LABEL_REMOVED" => Ok(AppliedLabelChangeDetailTypesEnum::LABELREMOVED),
           "LABEL_FIELD_VALUE_CHANGED" => Ok(AppliedLabelChangeDetailTypesEnum::LABELFIELDVALUECHANGED),
           "LABEL_APPLIED_BY_ITEM_CREATE" => Ok(AppliedLabelChangeDetailTypesEnum::LABELAPPLIEDBYITEMCREATE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AppliedLabelChangeDetailTypesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AssignmentSubtypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The sub-type of this event.
pub enum AssignmentSubtypeEnum {
    

    /// Subtype not available.
    ///
    /// "SUBTYPE_UNSPECIFIED"
    #[serde(rename="SUBTYPE_UNSPECIFIED")]
    SUBTYPEUNSPECIFIED,
    

    /// An assignment was added.
    ///
    /// "ADDED"
    #[serde(rename="ADDED")]
    ADDED,
    

    /// An assignment was deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// An assignment reply was added.
    ///
    /// "REPLY_ADDED"
    #[serde(rename="REPLY_ADDED")]
    REPLYADDED,
    

    /// An assignment reply was deleted.
    ///
    /// "REPLY_DELETED"
    #[serde(rename="REPLY_DELETED")]
    REPLYDELETED,
    

    /// An assignment was resolved.
    ///
    /// "RESOLVED"
    #[serde(rename="RESOLVED")]
    RESOLVED,
    

    /// A resolved assignment was reopened.
    ///
    /// "REOPENED"
    #[serde(rename="REOPENED")]
    REOPENED,
    

    /// An assignment was reassigned.
    ///
    /// "REASSIGNED"
    #[serde(rename="REASSIGNED")]
    REASSIGNED,
}

impl AsRef<str> for AssignmentSubtypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AssignmentSubtypeEnum::SUBTYPEUNSPECIFIED => "SUBTYPE_UNSPECIFIED",
            AssignmentSubtypeEnum::ADDED => "ADDED",
            AssignmentSubtypeEnum::DELETED => "DELETED",
            AssignmentSubtypeEnum::REPLYADDED => "REPLY_ADDED",
            AssignmentSubtypeEnum::REPLYDELETED => "REPLY_DELETED",
            AssignmentSubtypeEnum::RESOLVED => "RESOLVED",
            AssignmentSubtypeEnum::REOPENED => "REOPENED",
            AssignmentSubtypeEnum::REASSIGNED => "REASSIGNED",
        }
    }
}

impl std::convert::TryFrom< &str> for AssignmentSubtypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBTYPE_UNSPECIFIED" => Ok(AssignmentSubtypeEnum::SUBTYPEUNSPECIFIED),
           "ADDED" => Ok(AssignmentSubtypeEnum::ADDED),
           "DELETED" => Ok(AssignmentSubtypeEnum::DELETED),
           "REPLY_ADDED" => Ok(AssignmentSubtypeEnum::REPLYADDED),
           "REPLY_DELETED" => Ok(AssignmentSubtypeEnum::REPLYDELETED),
           "RESOLVED" => Ok(AssignmentSubtypeEnum::RESOLVED),
           "REOPENED" => Ok(AssignmentSubtypeEnum::REOPENED),
           "REASSIGNED" => Ok(AssignmentSubtypeEnum::REASSIGNED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AssignmentSubtypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DataLeakPreventionChangeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of Data Leak Prevention (DLP) change.
pub enum DataLeakPreventionChangeTypeEnum {
    

    /// An update to the DLP state that is neither FLAGGED or CLEARED.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// Document has been flagged as containing sensitive content.
    ///
    /// "FLAGGED"
    #[serde(rename="FLAGGED")]
    FLAGGED,
    

    /// Document is no longer flagged as containing sensitive content.
    ///
    /// "CLEARED"
    #[serde(rename="CLEARED")]
    CLEARED,
}

impl AsRef<str> for DataLeakPreventionChangeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DataLeakPreventionChangeTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DataLeakPreventionChangeTypeEnum::FLAGGED => "FLAGGED",
            DataLeakPreventionChangeTypeEnum::CLEARED => "CLEARED",
        }
    }
}

impl std::convert::TryFrom< &str> for DataLeakPreventionChangeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DataLeakPreventionChangeTypeEnum::TYPEUNSPECIFIED),
           "FLAGGED" => Ok(DataLeakPreventionChangeTypeEnum::FLAGGED),
           "CLEARED" => Ok(DataLeakPreventionChangeTypeEnum::CLEARED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DataLeakPreventionChangeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DeleteTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of delete action taken.
pub enum DeleteTypeEnum {
    

    /// Deletion type is not available.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// An object was put into the trash.
    ///
    /// "TRASH"
    #[serde(rename="TRASH")]
    TRASH,
    

    /// An object was deleted permanently.
    ///
    /// "PERMANENT_DELETE"
    #[serde(rename="PERMANENT_DELETE")]
    PERMANENTDELETE,
}

impl AsRef<str> for DeleteTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DeleteTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DeleteTypeEnum::TRASH => "TRASH",
            DeleteTypeEnum::PERMANENTDELETE => "PERMANENT_DELETE",
        }
    }
}

impl std::convert::TryFrom< &str> for DeleteTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DeleteTypeEnum::TYPEUNSPECIFIED),
           "TRASH" => Ok(DeleteTypeEnum::TRASH),
           "PERMANENT_DELETE" => Ok(DeleteTypeEnum::PERMANENTDELETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DeleteTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region DriveFolderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of Drive folder.
pub enum DriveFolderTypeEnum {
    

    /// The folder type is unknown.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The folder is the root of a user's MyDrive.
    ///
    /// "MY_DRIVE_ROOT"
    #[serde(rename="MY_DRIVE_ROOT")]
    MYDRIVEROOT,
    

    /// The folder is the root of a shared drive.
    ///
    /// "SHARED_DRIVE_ROOT"
    #[serde(rename="SHARED_DRIVE_ROOT")]
    SHAREDDRIVEROOT,
    

    /// The folder is a standard, non-root, folder.
    ///
    /// "STANDARD_FOLDER"
    #[serde(rename="STANDARD_FOLDER")]
    STANDARDFOLDER,
}

impl AsRef<str> for DriveFolderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            DriveFolderTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            DriveFolderTypeEnum::MYDRIVEROOT => "MY_DRIVE_ROOT",
            DriveFolderTypeEnum::SHAREDDRIVEROOT => "SHARED_DRIVE_ROOT",
            DriveFolderTypeEnum::STANDARDFOLDER => "STANDARD_FOLDER",
        }
    }
}

impl std::convert::TryFrom< &str> for DriveFolderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(DriveFolderTypeEnum::TYPEUNSPECIFIED),
           "MY_DRIVE_ROOT" => Ok(DriveFolderTypeEnum::MYDRIVEROOT),
           "SHARED_DRIVE_ROOT" => Ok(DriveFolderTypeEnum::SHAREDDRIVEROOT),
           "STANDARD_FOLDER" => Ok(DriveFolderTypeEnum::STANDARDFOLDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a DriveFolderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FolderTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This field is deprecated; please see `DriveFolder.type` instead.
pub enum FolderTypeEnum {
    

    /// This item is deprecated; please see `DriveFolder.Type` instead.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// This item is deprecated; please see `DriveFolder.Type` instead.
    ///
    /// "MY_DRIVE_ROOT"
    #[serde(rename="MY_DRIVE_ROOT")]
    MYDRIVEROOT,
    

    /// This item is deprecated; please see `DriveFolder.Type` instead.
    ///
    /// "TEAM_DRIVE_ROOT"
    #[serde(rename="TEAM_DRIVE_ROOT")]
    TEAMDRIVEROOT,
    

    /// This item is deprecated; please see `DriveFolder.Type` instead.
    ///
    /// "STANDARD_FOLDER"
    #[serde(rename="STANDARD_FOLDER")]
    STANDARDFOLDER,
}

impl AsRef<str> for FolderTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FolderTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            FolderTypeEnum::MYDRIVEROOT => "MY_DRIVE_ROOT",
            FolderTypeEnum::TEAMDRIVEROOT => "TEAM_DRIVE_ROOT",
            FolderTypeEnum::STANDARDFOLDER => "STANDARD_FOLDER",
        }
    }
}

impl std::convert::TryFrom< &str> for FolderTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(FolderTypeEnum::TYPEUNSPECIFIED),
           "MY_DRIVE_ROOT" => Ok(FolderTypeEnum::MYDRIVEROOT),
           "TEAM_DRIVE_ROOT" => Ok(FolderTypeEnum::TEAMDRIVEROOT),
           "STANDARD_FOLDER" => Ok(FolderTypeEnum::STANDARDFOLDER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FolderTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PermissionRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates the [Google Drive permissions role](https://developers.google.com/drive/web/manage-sharing#roles). The role determines a user's ability to read, write, and comment on items.
pub enum PermissionRoleEnum {
    

    /// The role is not available.
    ///
    /// "ROLE_UNSPECIFIED"
    #[serde(rename="ROLE_UNSPECIFIED")]
    ROLEUNSPECIFIED,
    

    /// A role granting full access.
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
    

    /// A role granting the ability to manage people and settings.
    ///
    /// "ORGANIZER"
    #[serde(rename="ORGANIZER")]
    ORGANIZER,
    

    /// A role granting the ability to contribute and manage content.
    ///
    /// "FILE_ORGANIZER"
    #[serde(rename="FILE_ORGANIZER")]
    FILEORGANIZER,
    

    /// A role granting the ability to contribute content. This role is sometimes also known as "writer".
    ///
    /// "EDITOR"
    #[serde(rename="EDITOR")]
    EDITOR,
    

    /// A role granting the ability to view and comment on content.
    ///
    /// "COMMENTER"
    #[serde(rename="COMMENTER")]
    COMMENTER,
    

    /// A role granting the ability to view content. This role is sometimes also known as "reader".
    ///
    /// "VIEWER"
    #[serde(rename="VIEWER")]
    VIEWER,
    

    /// A role granting the ability to view content only after it has been published to the web. This role is sometimes also known as "published reader". See https://support.google.com/sites/answer/6372880 for more information.
    ///
    /// "PUBLISHED_VIEWER"
    #[serde(rename="PUBLISHED_VIEWER")]
    PUBLISHEDVIEWER,
}

impl AsRef<str> for PermissionRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PermissionRoleEnum::ROLEUNSPECIFIED => "ROLE_UNSPECIFIED",
            PermissionRoleEnum::OWNER => "OWNER",
            PermissionRoleEnum::ORGANIZER => "ORGANIZER",
            PermissionRoleEnum::FILEORGANIZER => "FILE_ORGANIZER",
            PermissionRoleEnum::EDITOR => "EDITOR",
            PermissionRoleEnum::COMMENTER => "COMMENTER",
            PermissionRoleEnum::VIEWER => "VIEWER",
            PermissionRoleEnum::PUBLISHEDVIEWER => "PUBLISHED_VIEWER",
        }
    }
}

impl std::convert::TryFrom< &str> for PermissionRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ROLE_UNSPECIFIED" => Ok(PermissionRoleEnum::ROLEUNSPECIFIED),
           "OWNER" => Ok(PermissionRoleEnum::OWNER),
           "ORGANIZER" => Ok(PermissionRoleEnum::ORGANIZER),
           "FILE_ORGANIZER" => Ok(PermissionRoleEnum::FILEORGANIZER),
           "EDITOR" => Ok(PermissionRoleEnum::EDITOR),
           "COMMENTER" => Ok(PermissionRoleEnum::COMMENTER),
           "VIEWER" => Ok(PermissionRoleEnum::VIEWER),
           "PUBLISHED_VIEWER" => Ok(PermissionRoleEnum::PUBLISHEDVIEWER),
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


// region PostSubtypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The sub-type of this event.
pub enum PostSubtypeEnum {
    

    /// Subtype not available.
    ///
    /// "SUBTYPE_UNSPECIFIED"
    #[serde(rename="SUBTYPE_UNSPECIFIED")]
    SUBTYPEUNSPECIFIED,
    

    /// A post was added.
    ///
    /// "ADDED"
    #[serde(rename="ADDED")]
    ADDED,
    

    /// A post was deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// A reply was added.
    ///
    /// "REPLY_ADDED"
    #[serde(rename="REPLY_ADDED")]
    REPLYADDED,
    

    /// A reply was deleted.
    ///
    /// "REPLY_DELETED"
    #[serde(rename="REPLY_DELETED")]
    REPLYDELETED,
    

    /// A posted comment was resolved.
    ///
    /// "RESOLVED"
    #[serde(rename="RESOLVED")]
    RESOLVED,
    

    /// A posted comment was reopened.
    ///
    /// "REOPENED"
    #[serde(rename="REOPENED")]
    REOPENED,
}

impl AsRef<str> for PostSubtypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PostSubtypeEnum::SUBTYPEUNSPECIFIED => "SUBTYPE_UNSPECIFIED",
            PostSubtypeEnum::ADDED => "ADDED",
            PostSubtypeEnum::DELETED => "DELETED",
            PostSubtypeEnum::REPLYADDED => "REPLY_ADDED",
            PostSubtypeEnum::REPLYDELETED => "REPLY_DELETED",
            PostSubtypeEnum::RESOLVED => "RESOLVED",
            PostSubtypeEnum::REOPENED => "REOPENED",
        }
    }
}

impl std::convert::TryFrom< &str> for PostSubtypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBTYPE_UNSPECIFIED" => Ok(PostSubtypeEnum::SUBTYPEUNSPECIFIED),
           "ADDED" => Ok(PostSubtypeEnum::ADDED),
           "DELETED" => Ok(PostSubtypeEnum::DELETED),
           "REPLY_ADDED" => Ok(PostSubtypeEnum::REPLYADDED),
           "REPLY_DELETED" => Ok(PostSubtypeEnum::REPLYDELETED),
           "RESOLVED" => Ok(PostSubtypeEnum::RESOLVED),
           "REOPENED" => Ok(PostSubtypeEnum::REOPENED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PostSubtypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestoreTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of restore action taken.
pub enum RestoreTypeEnum {
    

    /// The type is not available.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// An object was restored from the trash.
    ///
    /// "UNTRASH"
    #[serde(rename="UNTRASH")]
    UNTRASH,
}

impl AsRef<str> for RestoreTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestoreTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            RestoreTypeEnum::UNTRASH => "UNTRASH",
        }
    }
}

impl std::convert::TryFrom< &str> for RestoreTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(RestoreTypeEnum::TYPEUNSPECIFIED),
           "UNTRASH" => Ok(RestoreTypeEnum::UNTRASH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestoreTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestrictionChangeFeatureEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The feature which had a change in restriction policy.
pub enum RestrictionChangeFeatureEnum {
    

    /// The feature which changed restriction settings was not available.
    ///
    /// "FEATURE_UNSPECIFIED"
    #[serde(rename="FEATURE_UNSPECIFIED")]
    FEATUREUNSPECIFIED,
    

    /// When restricted, this prevents items from being shared outside the domain.
    ///
    /// "SHARING_OUTSIDE_DOMAIN"
    #[serde(rename="SHARING_OUTSIDE_DOMAIN")]
    SHARINGOUTSIDEDOMAIN,
    

    /// When restricted, this prevents direct sharing of individual items.
    ///
    /// "DIRECT_SHARING"
    #[serde(rename="DIRECT_SHARING")]
    DIRECTSHARING,
    

    /// When restricted, this prevents actions like copy, download, and print that might result in uncontrolled duplicates of items.
    ///
    /// "ITEM_DUPLICATION"
    #[serde(rename="ITEM_DUPLICATION")]
    ITEMDUPLICATION,
    

    /// When restricted, this prevents use of Drive File Stream.
    ///
    /// "DRIVE_FILE_STREAM"
    #[serde(rename="DRIVE_FILE_STREAM")]
    DRIVEFILESTREAM,
    

    /// When restricted, this limits sharing of folders to managers only.
    ///
    /// "FILE_ORGANIZER_CAN_SHARE_FOLDERS"
    #[serde(rename="FILE_ORGANIZER_CAN_SHARE_FOLDERS")]
    FILEORGANIZERCANSHAREFOLDERS,
}

impl AsRef<str> for RestrictionChangeFeatureEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestrictionChangeFeatureEnum::FEATUREUNSPECIFIED => "FEATURE_UNSPECIFIED",
            RestrictionChangeFeatureEnum::SHARINGOUTSIDEDOMAIN => "SHARING_OUTSIDE_DOMAIN",
            RestrictionChangeFeatureEnum::DIRECTSHARING => "DIRECT_SHARING",
            RestrictionChangeFeatureEnum::ITEMDUPLICATION => "ITEM_DUPLICATION",
            RestrictionChangeFeatureEnum::DRIVEFILESTREAM => "DRIVE_FILE_STREAM",
            RestrictionChangeFeatureEnum::FILEORGANIZERCANSHAREFOLDERS => "FILE_ORGANIZER_CAN_SHARE_FOLDERS",
        }
    }
}

impl std::convert::TryFrom< &str> for RestrictionChangeFeatureEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEATURE_UNSPECIFIED" => Ok(RestrictionChangeFeatureEnum::FEATUREUNSPECIFIED),
           "SHARING_OUTSIDE_DOMAIN" => Ok(RestrictionChangeFeatureEnum::SHARINGOUTSIDEDOMAIN),
           "DIRECT_SHARING" => Ok(RestrictionChangeFeatureEnum::DIRECTSHARING),
           "ITEM_DUPLICATION" => Ok(RestrictionChangeFeatureEnum::ITEMDUPLICATION),
           "DRIVE_FILE_STREAM" => Ok(RestrictionChangeFeatureEnum::DRIVEFILESTREAM),
           "FILE_ORGANIZER_CAN_SHARE_FOLDERS" => Ok(RestrictionChangeFeatureEnum::FILEORGANIZERCANSHAREFOLDERS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestrictionChangeFeatureEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RestrictionChangeNewRestrictionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The restriction in place after the change.
pub enum RestrictionChangeNewRestrictionEnum {
    

    /// The type of restriction is not available.
    ///
    /// "RESTRICTION_UNSPECIFIED"
    #[serde(rename="RESTRICTION_UNSPECIFIED")]
    RESTRICTIONUNSPECIFIED,
    

    /// The feature is available without restriction.
    ///
    /// "UNRESTRICTED"
    #[serde(rename="UNRESTRICTED")]
    UNRESTRICTED,
    

    /// The use of this feature is fully restricted.
    ///
    /// "FULLY_RESTRICTED"
    #[serde(rename="FULLY_RESTRICTED")]
    FULLYRESTRICTED,
}

impl AsRef<str> for RestrictionChangeNewRestrictionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RestrictionChangeNewRestrictionEnum::RESTRICTIONUNSPECIFIED => "RESTRICTION_UNSPECIFIED",
            RestrictionChangeNewRestrictionEnum::UNRESTRICTED => "UNRESTRICTED",
            RestrictionChangeNewRestrictionEnum::FULLYRESTRICTED => "FULLY_RESTRICTED",
        }
    }
}

impl std::convert::TryFrom< &str> for RestrictionChangeNewRestrictionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "RESTRICTION_UNSPECIFIED" => Ok(RestrictionChangeNewRestrictionEnum::RESTRICTIONUNSPECIFIED),
           "UNRESTRICTED" => Ok(RestrictionChangeNewRestrictionEnum::UNRESTRICTED),
           "FULLY_RESTRICTED" => Ok(RestrictionChangeNewRestrictionEnum::FULLYRESTRICTED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RestrictionChangeNewRestrictionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SuggestionSubtypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The sub-type of this event.
pub enum SuggestionSubtypeEnum {
    

    /// Subtype not available.
    ///
    /// "SUBTYPE_UNSPECIFIED"
    #[serde(rename="SUBTYPE_UNSPECIFIED")]
    SUBTYPEUNSPECIFIED,
    

    /// A suggestion was added.
    ///
    /// "ADDED"
    #[serde(rename="ADDED")]
    ADDED,
    

    /// A suggestion was deleted.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
    

    /// A suggestion reply was added.
    ///
    /// "REPLY_ADDED"
    #[serde(rename="REPLY_ADDED")]
    REPLYADDED,
    

    /// A suggestion reply was deleted.
    ///
    /// "REPLY_DELETED"
    #[serde(rename="REPLY_DELETED")]
    REPLYDELETED,
    

    /// A suggestion was accepted.
    ///
    /// "ACCEPTED"
    #[serde(rename="ACCEPTED")]
    ACCEPTED,
    

    /// A suggestion was rejected.
    ///
    /// "REJECTED"
    #[serde(rename="REJECTED")]
    REJECTED,
    

    /// An accepted suggestion was deleted.
    ///
    /// "ACCEPT_DELETED"
    #[serde(rename="ACCEPT_DELETED")]
    ACCEPTDELETED,
    

    /// A rejected suggestion was deleted.
    ///
    /// "REJECT_DELETED"
    #[serde(rename="REJECT_DELETED")]
    REJECTDELETED,
}

impl AsRef<str> for SuggestionSubtypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SuggestionSubtypeEnum::SUBTYPEUNSPECIFIED => "SUBTYPE_UNSPECIFIED",
            SuggestionSubtypeEnum::ADDED => "ADDED",
            SuggestionSubtypeEnum::DELETED => "DELETED",
            SuggestionSubtypeEnum::REPLYADDED => "REPLY_ADDED",
            SuggestionSubtypeEnum::REPLYDELETED => "REPLY_DELETED",
            SuggestionSubtypeEnum::ACCEPTED => "ACCEPTED",
            SuggestionSubtypeEnum::REJECTED => "REJECTED",
            SuggestionSubtypeEnum::ACCEPTDELETED => "ACCEPT_DELETED",
            SuggestionSubtypeEnum::REJECTDELETED => "REJECT_DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for SuggestionSubtypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBTYPE_UNSPECIFIED" => Ok(SuggestionSubtypeEnum::SUBTYPEUNSPECIFIED),
           "ADDED" => Ok(SuggestionSubtypeEnum::ADDED),
           "DELETED" => Ok(SuggestionSubtypeEnum::DELETED),
           "REPLY_ADDED" => Ok(SuggestionSubtypeEnum::REPLYADDED),
           "REPLY_DELETED" => Ok(SuggestionSubtypeEnum::REPLYDELETED),
           "ACCEPTED" => Ok(SuggestionSubtypeEnum::ACCEPTED),
           "REJECTED" => Ok(SuggestionSubtypeEnum::REJECTED),
           "ACCEPT_DELETED" => Ok(SuggestionSubtypeEnum::ACCEPTDELETED),
           "REJECT_DELETED" => Ok(SuggestionSubtypeEnum::REJECTDELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SuggestionSubtypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SystemEventTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the system event that may triggered activity.
pub enum SystemEventTypeEnum {
    

    /// The event type is unspecified.
    ///
    /// "TYPE_UNSPECIFIED"
    #[serde(rename="TYPE_UNSPECIFIED")]
    TYPEUNSPECIFIED,
    

    /// The event is a consequence of a user account being deleted.
    ///
    /// "USER_DELETION"
    #[serde(rename="USER_DELETION")]
    USERDELETION,
    

    /// The event is due to the system automatically purging trash.
    ///
    /// "TRASH_AUTO_PURGE"
    #[serde(rename="TRASH_AUTO_PURGE")]
    TRASHAUTOPURGE,
}

impl AsRef<str> for SystemEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SystemEventTypeEnum::TYPEUNSPECIFIED => "TYPE_UNSPECIFIED",
            SystemEventTypeEnum::USERDELETION => "USER_DELETION",
            SystemEventTypeEnum::TRASHAUTOPURGE => "TRASH_AUTO_PURGE",
        }
    }
}

impl std::convert::TryFrom< &str> for SystemEventTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TYPE_UNSPECIFIED" => Ok(SystemEventTypeEnum::TYPEUNSPECIFIED),
           "USER_DELETION" => Ok(SystemEventTypeEnum::USERDELETION),
           "TRASH_AUTO_PURGE" => Ok(SystemEventTypeEnum::TRASHAUTOPURGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SystemEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


