use super::*;



// region AnnouncementAssigneeModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`.
pub enum AnnouncementAssigneeModeEnum {
    

    /// No mode specified. This is never returned.
    ///
    /// "ASSIGNEE_MODE_UNSPECIFIED"
    #[serde(rename="ASSIGNEE_MODE_UNSPECIFIED")]
    ASSIGNEEMODEUNSPECIFIED,
    

    /// All students can see the item. This is the default state.
    ///
    /// "ALL_STUDENTS"
    #[serde(rename="ALL_STUDENTS")]
    ALLSTUDENTS,
    

    /// A subset of the students can see the item.
    ///
    /// "INDIVIDUAL_STUDENTS"
    #[serde(rename="INDIVIDUAL_STUDENTS")]
    INDIVIDUALSTUDENTS,
}

impl AsRef<str> for AnnouncementAssigneeModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnnouncementAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED => "ASSIGNEE_MODE_UNSPECIFIED",
            AnnouncementAssigneeModeEnum::ALLSTUDENTS => "ALL_STUDENTS",
            AnnouncementAssigneeModeEnum::INDIVIDUALSTUDENTS => "INDIVIDUAL_STUDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for AnnouncementAssigneeModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSIGNEE_MODE_UNSPECIFIED" => Ok(AnnouncementAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED),
           "ALL_STUDENTS" => Ok(AnnouncementAssigneeModeEnum::ALLSTUDENTS),
           "INDIVIDUAL_STUDENTS" => Ok(AnnouncementAssigneeModeEnum::INDIVIDUALSTUDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnnouncementAssigneeModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AnnouncementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of this announcement. If unspecified, the default state is `DRAFT`.
pub enum AnnouncementStateEnum {
    

    /// No state specified. This is never returned.
    ///
    /// "ANNOUNCEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ANNOUNCEMENT_STATE_UNSPECIFIED")]
    ANNOUNCEMENTSTATEUNSPECIFIED,
    

    /// Status for announcement that has been published. This is the default state.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Status for an announcement that is not yet published. Announcement in this state is visible only to course teachers and domain administrators.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Status for announcement that was published but is now deleted. Announcement in this state is visible only to course teachers and domain administrators. Announcement in this state is deleted after some time.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for AnnouncementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AnnouncementStateEnum::ANNOUNCEMENTSTATEUNSPECIFIED => "ANNOUNCEMENT_STATE_UNSPECIFIED",
            AnnouncementStateEnum::PUBLISHED => "PUBLISHED",
            AnnouncementStateEnum::DRAFT => "DRAFT",
            AnnouncementStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for AnnouncementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOUNCEMENT_STATE_UNSPECIFIED" => Ok(AnnouncementStateEnum::ANNOUNCEMENTSTATEUNSPECIFIED),
           "PUBLISHED" => Ok(AnnouncementStateEnum::PUBLISHED),
           "DRAFT" => Ok(AnnouncementStateEnum::DRAFT),
           "DELETED" => Ok(AnnouncementStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AnnouncementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseCourseStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of the course. If unspecified, the default state is `PROVISIONED`.
pub enum CourseCourseStateEnum {
    

    /// No course state. No returned Course message will use this value.
    ///
    /// "COURSE_STATE_UNSPECIFIED"
    #[serde(rename="COURSE_STATE_UNSPECIFIED")]
    COURSESTATEUNSPECIFIED,
    

    /// The course is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The course has been archived. You cannot modify it except to change it to a different state.
    ///
    /// "ARCHIVED"
    #[serde(rename="ARCHIVED")]
    ARCHIVED,
    

    /// The course has been created, but not yet activated. It is accessible by the primary teacher and domain administrators, who may modify it or change it to the `ACTIVE` or `DECLINED` states. A course may only be changed to `PROVISIONED` if it is in the `DECLINED` state.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
    

    /// The course has been created, but declined. It is accessible by the course owner and domain administrators, though it will not be displayed in the web UI. You cannot modify the course except to change it to the `PROVISIONED` state. A course may only be changed to `DECLINED` if it is in the `PROVISIONED` state.
    ///
    /// "DECLINED"
    #[serde(rename="DECLINED")]
    DECLINED,
    

    /// The course has been suspended. You cannot modify the course, and only the user identified by the `owner_id` can view the course. A course may be placed in this state if it potentially violates the Terms of Service.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for CourseCourseStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseCourseStateEnum::COURSESTATEUNSPECIFIED => "COURSE_STATE_UNSPECIFIED",
            CourseCourseStateEnum::ACTIVE => "ACTIVE",
            CourseCourseStateEnum::ARCHIVED => "ARCHIVED",
            CourseCourseStateEnum::PROVISIONED => "PROVISIONED",
            CourseCourseStateEnum::DECLINED => "DECLINED",
            CourseCourseStateEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseCourseStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_STATE_UNSPECIFIED" => Ok(CourseCourseStateEnum::COURSESTATEUNSPECIFIED),
           "ACTIVE" => Ok(CourseCourseStateEnum::ACTIVE),
           "ARCHIVED" => Ok(CourseCourseStateEnum::ARCHIVED),
           "PROVISIONED" => Ok(CourseCourseStateEnum::PROVISIONED),
           "DECLINED" => Ok(CourseCourseStateEnum::DECLINED),
           "SUSPENDED" => Ok(CourseCourseStateEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseCourseStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseWorkAssigneeModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`.
pub enum CourseWorkAssigneeModeEnum {
    

    /// No mode specified. This is never returned.
    ///
    /// "ASSIGNEE_MODE_UNSPECIFIED"
    #[serde(rename="ASSIGNEE_MODE_UNSPECIFIED")]
    ASSIGNEEMODEUNSPECIFIED,
    

    /// All students can see the item. This is the default state.
    ///
    /// "ALL_STUDENTS"
    #[serde(rename="ALL_STUDENTS")]
    ALLSTUDENTS,
    

    /// A subset of the students can see the item.
    ///
    /// "INDIVIDUAL_STUDENTS"
    #[serde(rename="INDIVIDUAL_STUDENTS")]
    INDIVIDUALSTUDENTS,
}

impl AsRef<str> for CourseWorkAssigneeModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseWorkAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED => "ASSIGNEE_MODE_UNSPECIFIED",
            CourseWorkAssigneeModeEnum::ALLSTUDENTS => "ALL_STUDENTS",
            CourseWorkAssigneeModeEnum::INDIVIDUALSTUDENTS => "INDIVIDUAL_STUDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseWorkAssigneeModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSIGNEE_MODE_UNSPECIFIED" => Ok(CourseWorkAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED),
           "ALL_STUDENTS" => Ok(CourseWorkAssigneeModeEnum::ALLSTUDENTS),
           "INDIVIDUAL_STUDENTS" => Ok(CourseWorkAssigneeModeEnum::INDIVIDUALSTUDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseWorkAssigneeModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseWorkStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of this course work. If unspecified, the default state is `DRAFT`.
pub enum CourseWorkStateEnum {
    

    /// No state specified. This is never returned.
    ///
    /// "COURSE_WORK_STATE_UNSPECIFIED"
    #[serde(rename="COURSE_WORK_STATE_UNSPECIFIED")]
    COURSEWORKSTATEUNSPECIFIED,
    

    /// Status for work that has been published. This is the default state.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Status for work that is not yet published. Work in this state is visible only to course teachers and domain administrators.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Status for work that was published but is now deleted. Work in this state is visible only to course teachers and domain administrators. Work in this state is deleted after some time.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CourseWorkStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseWorkStateEnum::COURSEWORKSTATEUNSPECIFIED => "COURSE_WORK_STATE_UNSPECIFIED",
            CourseWorkStateEnum::PUBLISHED => "PUBLISHED",
            CourseWorkStateEnum::DRAFT => "DRAFT",
            CourseWorkStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseWorkStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_WORK_STATE_UNSPECIFIED" => Ok(CourseWorkStateEnum::COURSEWORKSTATEUNSPECIFIED),
           "PUBLISHED" => Ok(CourseWorkStateEnum::PUBLISHED),
           "DRAFT" => Ok(CourseWorkStateEnum::DRAFT),
           "DELETED" => Ok(CourseWorkStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseWorkStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseWorkSubmissionModificationModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`.
pub enum CourseWorkSubmissionModificationModeEnum {
    

    /// No modification mode specified. This is never returned.
    ///
    /// "SUBMISSION_MODIFICATION_MODE_UNSPECIFIED"
    #[serde(rename="SUBMISSION_MODIFICATION_MODE_UNSPECIFIED")]
    SUBMISSIONMODIFICATIONMODEUNSPECIFIED,
    

    /// Submissions can be modified before being turned in.
    ///
    /// "MODIFIABLE_UNTIL_TURNED_IN"
    #[serde(rename="MODIFIABLE_UNTIL_TURNED_IN")]
    MODIFIABLEUNTILTURNEDIN,
    

    /// Submissions can be modified at any time.
    ///
    /// "MODIFIABLE"
    #[serde(rename="MODIFIABLE")]
    MODIFIABLE,
}

impl AsRef<str> for CourseWorkSubmissionModificationModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseWorkSubmissionModificationModeEnum::SUBMISSIONMODIFICATIONMODEUNSPECIFIED => "SUBMISSION_MODIFICATION_MODE_UNSPECIFIED",
            CourseWorkSubmissionModificationModeEnum::MODIFIABLEUNTILTURNEDIN => "MODIFIABLE_UNTIL_TURNED_IN",
            CourseWorkSubmissionModificationModeEnum::MODIFIABLE => "MODIFIABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseWorkSubmissionModificationModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBMISSION_MODIFICATION_MODE_UNSPECIFIED" => Ok(CourseWorkSubmissionModificationModeEnum::SUBMISSIONMODIFICATIONMODEUNSPECIFIED),
           "MODIFIABLE_UNTIL_TURNED_IN" => Ok(CourseWorkSubmissionModificationModeEnum::MODIFIABLEUNTILTURNEDIN),
           "MODIFIABLE" => Ok(CourseWorkSubmissionModificationModeEnum::MODIFIABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseWorkSubmissionModificationModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseWorkWorkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of this course work. The type is set when the course work is created and cannot be changed.
pub enum CourseWorkWorkTypeEnum {
    

    /// No work type specified. This is never returned.
    ///
    /// "COURSE_WORK_TYPE_UNSPECIFIED"
    #[serde(rename="COURSE_WORK_TYPE_UNSPECIFIED")]
    COURSEWORKTYPEUNSPECIFIED,
    

    /// An assignment.
    ///
    /// "ASSIGNMENT"
    #[serde(rename="ASSIGNMENT")]
    ASSIGNMENT,
    

    /// A short answer question.
    ///
    /// "SHORT_ANSWER_QUESTION"
    #[serde(rename="SHORT_ANSWER_QUESTION")]
    SHORTANSWERQUESTION,
    

    /// A multiple-choice question.
    ///
    /// "MULTIPLE_CHOICE_QUESTION"
    #[serde(rename="MULTIPLE_CHOICE_QUESTION")]
    MULTIPLECHOICEQUESTION,
}

impl AsRef<str> for CourseWorkWorkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseWorkWorkTypeEnum::COURSEWORKTYPEUNSPECIFIED => "COURSE_WORK_TYPE_UNSPECIFIED",
            CourseWorkWorkTypeEnum::ASSIGNMENT => "ASSIGNMENT",
            CourseWorkWorkTypeEnum::SHORTANSWERQUESTION => "SHORT_ANSWER_QUESTION",
            CourseWorkWorkTypeEnum::MULTIPLECHOICEQUESTION => "MULTIPLE_CHOICE_QUESTION",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseWorkWorkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_WORK_TYPE_UNSPECIFIED" => Ok(CourseWorkWorkTypeEnum::COURSEWORKTYPEUNSPECIFIED),
           "ASSIGNMENT" => Ok(CourseWorkWorkTypeEnum::ASSIGNMENT),
           "SHORT_ANSWER_QUESTION" => Ok(CourseWorkWorkTypeEnum::SHORTANSWERQUESTION),
           "MULTIPLE_CHOICE_QUESTION" => Ok(CourseWorkWorkTypeEnum::MULTIPLECHOICEQUESTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseWorkWorkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseWorkMaterialAssigneeModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`.
pub enum CourseWorkMaterialAssigneeModeEnum {
    

    /// No mode specified. This is never returned.
    ///
    /// "ASSIGNEE_MODE_UNSPECIFIED"
    #[serde(rename="ASSIGNEE_MODE_UNSPECIFIED")]
    ASSIGNEEMODEUNSPECIFIED,
    

    /// All students can see the item. This is the default state.
    ///
    /// "ALL_STUDENTS"
    #[serde(rename="ALL_STUDENTS")]
    ALLSTUDENTS,
    

    /// A subset of the students can see the item.
    ///
    /// "INDIVIDUAL_STUDENTS"
    #[serde(rename="INDIVIDUAL_STUDENTS")]
    INDIVIDUALSTUDENTS,
}

impl AsRef<str> for CourseWorkMaterialAssigneeModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseWorkMaterialAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED => "ASSIGNEE_MODE_UNSPECIFIED",
            CourseWorkMaterialAssigneeModeEnum::ALLSTUDENTS => "ALL_STUDENTS",
            CourseWorkMaterialAssigneeModeEnum::INDIVIDUALSTUDENTS => "INDIVIDUAL_STUDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseWorkMaterialAssigneeModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSIGNEE_MODE_UNSPECIFIED" => Ok(CourseWorkMaterialAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED),
           "ALL_STUDENTS" => Ok(CourseWorkMaterialAssigneeModeEnum::ALLSTUDENTS),
           "INDIVIDUAL_STUDENTS" => Ok(CourseWorkMaterialAssigneeModeEnum::INDIVIDUALSTUDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseWorkMaterialAssigneeModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseWorkMaterialStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Status of this course work material. If unspecified, the default state is `DRAFT`.
pub enum CourseWorkMaterialStateEnum {
    

    /// No state specified. This is never returned.
    ///
    /// "COURSEWORK_MATERIAL_STATE_UNSPECIFIED"
    #[serde(rename="COURSEWORK_MATERIAL_STATE_UNSPECIFIED")]
    COURSEWORKMATERIALSTATEUNSPECIFIED,
    

    /// Status for course work material that has been published. This is the default state.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Status for a course work material that is not yet published. Course work material in this state is visible only to course teachers and domain administrators.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Status for course work material that was published but is now deleted. Course work material in this state is visible only to course teachers and domain administrators. Course work material in this state is deleted after some time.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CourseWorkMaterialStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseWorkMaterialStateEnum::COURSEWORKMATERIALSTATEUNSPECIFIED => "COURSEWORK_MATERIAL_STATE_UNSPECIFIED",
            CourseWorkMaterialStateEnum::PUBLISHED => "PUBLISHED",
            CourseWorkMaterialStateEnum::DRAFT => "DRAFT",
            CourseWorkMaterialStateEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseWorkMaterialStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSEWORK_MATERIAL_STATE_UNSPECIFIED" => Ok(CourseWorkMaterialStateEnum::COURSEWORKMATERIALSTATEUNSPECIFIED),
           "PUBLISHED" => Ok(CourseWorkMaterialStateEnum::PUBLISHED),
           "DRAFT" => Ok(CourseWorkMaterialStateEnum::DRAFT),
           "DELETED" => Ok(CourseWorkMaterialStateEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseWorkMaterialStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region FeedFeedTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of feed.
pub enum FeedFeedTypeEnum {
    

    /// Should never be returned or provided.
    ///
    /// "FEED_TYPE_UNSPECIFIED"
    #[serde(rename="FEED_TYPE_UNSPECIFIED")]
    FEEDTYPEUNSPECIFIED,
    

    /// All roster changes for a particular domain. Notifications will be generated whenever a user joins or leaves a course. No notifications will be generated when an invitation is created or deleted, but notifications will be generated when a user joins a course by accepting an invitation.
    ///
    /// "DOMAIN_ROSTER_CHANGES"
    #[serde(rename="DOMAIN_ROSTER_CHANGES")]
    DOMAINROSTERCHANGES,
    

    /// All roster changes for a particular course. Notifications will be generated whenever a user joins or leaves a course. No notifications will be generated when an invitation is created or deleted, but notifications will be generated when a user joins a course by accepting an invitation.
    ///
    /// "COURSE_ROSTER_CHANGES"
    #[serde(rename="COURSE_ROSTER_CHANGES")]
    COURSEROSTERCHANGES,
    

    /// All course work activity for a particular course. Notifications will be generated when a CourseWork or StudentSubmission object is created or modified. No notification will be generated when a StudentSubmission object is created in connection with the creation or modification of its parent CourseWork object (but a notification will be generated for that CourseWork object's creation or modification).
    ///
    /// "COURSE_WORK_CHANGES"
    #[serde(rename="COURSE_WORK_CHANGES")]
    COURSEWORKCHANGES,
}

impl AsRef<str> for FeedFeedTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            FeedFeedTypeEnum::FEEDTYPEUNSPECIFIED => "FEED_TYPE_UNSPECIFIED",
            FeedFeedTypeEnum::DOMAINROSTERCHANGES => "DOMAIN_ROSTER_CHANGES",
            FeedFeedTypeEnum::COURSEROSTERCHANGES => "COURSE_ROSTER_CHANGES",
            FeedFeedTypeEnum::COURSEWORKCHANGES => "COURSE_WORK_CHANGES",
        }
    }
}

impl std::convert::TryFrom< &str> for FeedFeedTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "FEED_TYPE_UNSPECIFIED" => Ok(FeedFeedTypeEnum::FEEDTYPEUNSPECIFIED),
           "DOMAIN_ROSTER_CHANGES" => Ok(FeedFeedTypeEnum::DOMAINROSTERCHANGES),
           "COURSE_ROSTER_CHANGES" => Ok(FeedFeedTypeEnum::COURSEROSTERCHANGES),
           "COURSE_WORK_CHANGES" => Ok(FeedFeedTypeEnum::COURSEWORKCHANGES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a FeedFeedTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GlobalPermissionPermissionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Permission value.
pub enum GlobalPermissionPermissionEnum {
    

    /// No permission is specified. This is not returned and is not a valid value.
    ///
    /// "PERMISSION_UNSPECIFIED"
    #[serde(rename="PERMISSION_UNSPECIFIED")]
    PERMISSIONUNSPECIFIED,
    

    /// User is permitted to create a course.
    ///
    /// "CREATE_COURSE"
    #[serde(rename="CREATE_COURSE")]
    CREATECOURSE,
}

impl AsRef<str> for GlobalPermissionPermissionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GlobalPermissionPermissionEnum::PERMISSIONUNSPECIFIED => "PERMISSION_UNSPECIFIED",
            GlobalPermissionPermissionEnum::CREATECOURSE => "CREATE_COURSE",
        }
    }
}

impl std::convert::TryFrom< &str> for GlobalPermissionPermissionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PERMISSION_UNSPECIFIED" => Ok(GlobalPermissionPermissionEnum::PERMISSIONUNSPECIFIED),
           "CREATE_COURSE" => Ok(GlobalPermissionPermissionEnum::CREATECOURSE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GlobalPermissionPermissionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GradeHistoryGradeChangeTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of grade change at this time in the submission grade history.
pub enum GradeHistoryGradeChangeTypeEnum {
    

    /// No grade change type specified. This should never be returned.
    ///
    /// "UNKNOWN_GRADE_CHANGE_TYPE"
    #[serde(rename="UNKNOWN_GRADE_CHANGE_TYPE")]
    UNKNOWNGRADECHANGETYPE,
    

    /// A change in the numerator of the draft grade.
    ///
    /// "DRAFT_GRADE_POINTS_EARNED_CHANGE"
    #[serde(rename="DRAFT_GRADE_POINTS_EARNED_CHANGE")]
    DRAFTGRADEPOINTSEARNEDCHANGE,
    

    /// A change in the numerator of the assigned grade.
    ///
    /// "ASSIGNED_GRADE_POINTS_EARNED_CHANGE"
    #[serde(rename="ASSIGNED_GRADE_POINTS_EARNED_CHANGE")]
    ASSIGNEDGRADEPOINTSEARNEDCHANGE,
    

    /// A change in the denominator of the grade.
    ///
    /// "MAX_POINTS_CHANGE"
    #[serde(rename="MAX_POINTS_CHANGE")]
    MAXPOINTSCHANGE,
}

impl AsRef<str> for GradeHistoryGradeChangeTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GradeHistoryGradeChangeTypeEnum::UNKNOWNGRADECHANGETYPE => "UNKNOWN_GRADE_CHANGE_TYPE",
            GradeHistoryGradeChangeTypeEnum::DRAFTGRADEPOINTSEARNEDCHANGE => "DRAFT_GRADE_POINTS_EARNED_CHANGE",
            GradeHistoryGradeChangeTypeEnum::ASSIGNEDGRADEPOINTSEARNEDCHANGE => "ASSIGNED_GRADE_POINTS_EARNED_CHANGE",
            GradeHistoryGradeChangeTypeEnum::MAXPOINTSCHANGE => "MAX_POINTS_CHANGE",
        }
    }
}

impl std::convert::TryFrom< &str> for GradeHistoryGradeChangeTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_GRADE_CHANGE_TYPE" => Ok(GradeHistoryGradeChangeTypeEnum::UNKNOWNGRADECHANGETYPE),
           "DRAFT_GRADE_POINTS_EARNED_CHANGE" => Ok(GradeHistoryGradeChangeTypeEnum::DRAFTGRADEPOINTSEARNEDCHANGE),
           "ASSIGNED_GRADE_POINTS_EARNED_CHANGE" => Ok(GradeHistoryGradeChangeTypeEnum::ASSIGNEDGRADEPOINTSEARNEDCHANGE),
           "MAX_POINTS_CHANGE" => Ok(GradeHistoryGradeChangeTypeEnum::MAXPOINTSCHANGE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GradeHistoryGradeChangeTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GradebookSettingCalculationTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates how the overall grade is calculated.
pub enum GradebookSettingCalculationTypeEnum {
    

    /// No method specified. This is never returned.
    ///
    /// "CALCULATION_TYPE_UNSPECIFIED"
    #[serde(rename="CALCULATION_TYPE_UNSPECIFIED")]
    CALCULATIONTYPEUNSPECIFIED,
    

    /// Overall grade is the sum of grades divided by the sum of total points regardless of category.
    ///
    /// "TOTAL_POINTS"
    #[serde(rename="TOTAL_POINTS")]
    TOTALPOINTS,
    

    /// Overall grade is the weighted average by category.
    ///
    /// "WEIGHTED_CATEGORIES"
    #[serde(rename="WEIGHTED_CATEGORIES")]
    WEIGHTEDCATEGORIES,
}

impl AsRef<str> for GradebookSettingCalculationTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GradebookSettingCalculationTypeEnum::CALCULATIONTYPEUNSPECIFIED => "CALCULATION_TYPE_UNSPECIFIED",
            GradebookSettingCalculationTypeEnum::TOTALPOINTS => "TOTAL_POINTS",
            GradebookSettingCalculationTypeEnum::WEIGHTEDCATEGORIES => "WEIGHTED_CATEGORIES",
        }
    }
}

impl std::convert::TryFrom< &str> for GradebookSettingCalculationTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CALCULATION_TYPE_UNSPECIFIED" => Ok(GradebookSettingCalculationTypeEnum::CALCULATIONTYPEUNSPECIFIED),
           "TOTAL_POINTS" => Ok(GradebookSettingCalculationTypeEnum::TOTALPOINTS),
           "WEIGHTED_CATEGORIES" => Ok(GradebookSettingCalculationTypeEnum::WEIGHTEDCATEGORIES),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GradebookSettingCalculationTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GradebookSettingDisplaySettingEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates who can see the overall grade..
pub enum GradebookSettingDisplaySettingEnum {
    

    /// No setting specified. This is never returned.
    ///
    /// "DISPLAY_SETTING_UNSPECIFIED"
    #[serde(rename="DISPLAY_SETTING_UNSPECIFIED")]
    DISPLAYSETTINGUNSPECIFIED,
    

    /// Shows overall grade in the gradebook and student profile to both teachers and students.
    ///
    /// "SHOW_OVERALL_GRADE"
    #[serde(rename="SHOW_OVERALL_GRADE")]
    SHOWOVERALLGRADE,
    

    /// Does not show overall grade in the gradebook or student profile.
    ///
    /// "HIDE_OVERALL_GRADE"
    #[serde(rename="HIDE_OVERALL_GRADE")]
    HIDEOVERALLGRADE,
    

    /// Shows the overall grade to teachers in the gradebook and student profile. Hides from students in their student profile.
    ///
    /// "SHOW_TEACHERS_ONLY"
    #[serde(rename="SHOW_TEACHERS_ONLY")]
    SHOWTEACHERSONLY,
}

impl AsRef<str> for GradebookSettingDisplaySettingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GradebookSettingDisplaySettingEnum::DISPLAYSETTINGUNSPECIFIED => "DISPLAY_SETTING_UNSPECIFIED",
            GradebookSettingDisplaySettingEnum::SHOWOVERALLGRADE => "SHOW_OVERALL_GRADE",
            GradebookSettingDisplaySettingEnum::HIDEOVERALLGRADE => "HIDE_OVERALL_GRADE",
            GradebookSettingDisplaySettingEnum::SHOWTEACHERSONLY => "SHOW_TEACHERS_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for GradebookSettingDisplaySettingEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "DISPLAY_SETTING_UNSPECIFIED" => Ok(GradebookSettingDisplaySettingEnum::DISPLAYSETTINGUNSPECIFIED),
           "SHOW_OVERALL_GRADE" => Ok(GradebookSettingDisplaySettingEnum::SHOWOVERALLGRADE),
           "HIDE_OVERALL_GRADE" => Ok(GradebookSettingDisplaySettingEnum::HIDEOVERALLGRADE),
           "SHOW_TEACHERS_ONLY" => Ok(GradebookSettingDisplaySettingEnum::SHOWTEACHERSONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GradebookSettingDisplaySettingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GuardianInvitationStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state that this invitation is in.
pub enum GuardianInvitationStateEnum {
    

    /// Should never be returned.
    ///
    /// "GUARDIAN_INVITATION_STATE_UNSPECIFIED"
    #[serde(rename="GUARDIAN_INVITATION_STATE_UNSPECIFIED")]
    GUARDIANINVITATIONSTATEUNSPECIFIED,
    

    /// The invitation is active and awaiting a response.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The invitation is no longer active. It may have been accepted, declined, withdrawn or it may have expired.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
}

impl AsRef<str> for GuardianInvitationStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GuardianInvitationStateEnum::GUARDIANINVITATIONSTATEUNSPECIFIED => "GUARDIAN_INVITATION_STATE_UNSPECIFIED",
            GuardianInvitationStateEnum::PENDING => "PENDING",
            GuardianInvitationStateEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for GuardianInvitationStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GUARDIAN_INVITATION_STATE_UNSPECIFIED" => Ok(GuardianInvitationStateEnum::GUARDIANINVITATIONSTATEUNSPECIFIED),
           "PENDING" => Ok(GuardianInvitationStateEnum::PENDING),
           "COMPLETE" => Ok(GuardianInvitationStateEnum::COMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GuardianInvitationStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InvitationRoleEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`.
pub enum InvitationRoleEnum {
    

    /// No course role.
    ///
    /// "COURSE_ROLE_UNSPECIFIED"
    #[serde(rename="COURSE_ROLE_UNSPECIFIED")]
    COURSEROLEUNSPECIFIED,
    

    /// Student in the course.
    ///
    /// "STUDENT"
    #[serde(rename="STUDENT")]
    STUDENT,
    

    /// Teacher of the course.
    ///
    /// "TEACHER"
    #[serde(rename="TEACHER")]
    TEACHER,
    

    /// Owner of the course.
    ///
    /// "OWNER"
    #[serde(rename="OWNER")]
    OWNER,
}

impl AsRef<str> for InvitationRoleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvitationRoleEnum::COURSEROLEUNSPECIFIED => "COURSE_ROLE_UNSPECIFIED",
            InvitationRoleEnum::STUDENT => "STUDENT",
            InvitationRoleEnum::TEACHER => "TEACHER",
            InvitationRoleEnum::OWNER => "OWNER",
        }
    }
}

impl std::convert::TryFrom< &str> for InvitationRoleEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_ROLE_UNSPECIFIED" => Ok(InvitationRoleEnum::COURSEROLEUNSPECIFIED),
           "STUDENT" => Ok(InvitationRoleEnum::STUDENT),
           "TEACHER" => Ok(InvitationRoleEnum::TEACHER),
           "OWNER" => Ok(InvitationRoleEnum::OWNER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvitationRoleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ModifyAnnouncementAssigneesRequestAssigneeModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode of the announcement describing whether it is accessible by all students or specified individual students.
pub enum ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
    

    /// No mode specified. This is never returned.
    ///
    /// "ASSIGNEE_MODE_UNSPECIFIED"
    #[serde(rename="ASSIGNEE_MODE_UNSPECIFIED")]
    ASSIGNEEMODEUNSPECIFIED,
    

    /// All students can see the item. This is the default state.
    ///
    /// "ALL_STUDENTS"
    #[serde(rename="ALL_STUDENTS")]
    ALLSTUDENTS,
    

    /// A subset of the students can see the item.
    ///
    /// "INDIVIDUAL_STUDENTS"
    #[serde(rename="INDIVIDUAL_STUDENTS")]
    INDIVIDUALSTUDENTS,
}

impl AsRef<str> for ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ModifyAnnouncementAssigneesRequestAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED => "ASSIGNEE_MODE_UNSPECIFIED",
            ModifyAnnouncementAssigneesRequestAssigneeModeEnum::ALLSTUDENTS => "ALL_STUDENTS",
            ModifyAnnouncementAssigneesRequestAssigneeModeEnum::INDIVIDUALSTUDENTS => "INDIVIDUAL_STUDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSIGNEE_MODE_UNSPECIFIED" => Ok(ModifyAnnouncementAssigneesRequestAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED),
           "ALL_STUDENTS" => Ok(ModifyAnnouncementAssigneesRequestAssigneeModeEnum::ALLSTUDENTS),
           "INDIVIDUAL_STUDENTS" => Ok(ModifyAnnouncementAssigneesRequestAssigneeModeEnum::INDIVIDUALSTUDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ModifyAnnouncementAssigneesRequestAssigneeModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ModifyCourseWorkAssigneesRequestAssigneeModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mode of the coursework describing whether it will be assigned to all students or specified individual students.
pub enum ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
    

    /// No mode specified. This is never returned.
    ///
    /// "ASSIGNEE_MODE_UNSPECIFIED"
    #[serde(rename="ASSIGNEE_MODE_UNSPECIFIED")]
    ASSIGNEEMODEUNSPECIFIED,
    

    /// All students can see the item. This is the default state.
    ///
    /// "ALL_STUDENTS"
    #[serde(rename="ALL_STUDENTS")]
    ALLSTUDENTS,
    

    /// A subset of the students can see the item.
    ///
    /// "INDIVIDUAL_STUDENTS"
    #[serde(rename="INDIVIDUAL_STUDENTS")]
    INDIVIDUALSTUDENTS,
}

impl AsRef<str> for ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ModifyCourseWorkAssigneesRequestAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED => "ASSIGNEE_MODE_UNSPECIFIED",
            ModifyCourseWorkAssigneesRequestAssigneeModeEnum::ALLSTUDENTS => "ALL_STUDENTS",
            ModifyCourseWorkAssigneesRequestAssigneeModeEnum::INDIVIDUALSTUDENTS => "INDIVIDUAL_STUDENTS",
        }
    }
}

impl std::convert::TryFrom< &str> for ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ASSIGNEE_MODE_UNSPECIFIED" => Ok(ModifyCourseWorkAssigneesRequestAssigneeModeEnum::ASSIGNEEMODEUNSPECIFIED),
           "ALL_STUDENTS" => Ok(ModifyCourseWorkAssigneesRequestAssigneeModeEnum::ALLSTUDENTS),
           "INDIVIDUAL_STUDENTS" => Ok(ModifyCourseWorkAssigneesRequestAssigneeModeEnum::INDIVIDUALSTUDENTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ModifyCourseWorkAssigneesRequestAssigneeModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SharedDriveFileShareModeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Mechanism by which students access the Drive item.
pub enum SharedDriveFileShareModeEnum {
    

    /// No sharing mode specified. This should never be returned.
    ///
    /// "UNKNOWN_SHARE_MODE"
    #[serde(rename="UNKNOWN_SHARE_MODE")]
    UNKNOWNSHAREMODE,
    

    /// Students can view the shared file.
    ///
    /// "VIEW"
    #[serde(rename="VIEW")]
    VIEW,
    

    /// Students can edit the shared file.
    ///
    /// "EDIT"
    #[serde(rename="EDIT")]
    EDIT,
    

    /// Students have a personal copy of the shared file.
    ///
    /// "STUDENT_COPY"
    #[serde(rename="STUDENT_COPY")]
    STUDENTCOPY,
}

impl AsRef<str> for SharedDriveFileShareModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SharedDriveFileShareModeEnum::UNKNOWNSHAREMODE => "UNKNOWN_SHARE_MODE",
            SharedDriveFileShareModeEnum::VIEW => "VIEW",
            SharedDriveFileShareModeEnum::EDIT => "EDIT",
            SharedDriveFileShareModeEnum::STUDENTCOPY => "STUDENT_COPY",
        }
    }
}

impl std::convert::TryFrom< &str> for SharedDriveFileShareModeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "UNKNOWN_SHARE_MODE" => Ok(SharedDriveFileShareModeEnum::UNKNOWNSHAREMODE),
           "VIEW" => Ok(SharedDriveFileShareModeEnum::VIEW),
           "EDIT" => Ok(SharedDriveFileShareModeEnum::EDIT),
           "STUDENT_COPY" => Ok(SharedDriveFileShareModeEnum::STUDENTCOPY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SharedDriveFileShareModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StateHistoryStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The workflow pipeline stage.
pub enum StateHistoryStateEnum {
    

    /// No state specified. This should never be returned.
    ///
    /// "STATE_UNSPECIFIED"
    #[serde(rename="STATE_UNSPECIFIED")]
    STATEUNSPECIFIED,
    

    /// The Submission has been created.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// The student has turned in an assigned document, which may or may not be a template.
    ///
    /// "TURNED_IN"
    #[serde(rename="TURNED_IN")]
    TURNEDIN,
    

    /// The teacher has returned the assigned document to the student.
    ///
    /// "RETURNED"
    #[serde(rename="RETURNED")]
    RETURNED,
    

    /// The student turned in the assigned document, and then chose to "unsubmit" the assignment, giving the student control again as the owner.
    ///
    /// "RECLAIMED_BY_STUDENT"
    #[serde(rename="RECLAIMED_BY_STUDENT")]
    RECLAIMEDBYSTUDENT,
    

    /// The student edited their submission after turning it in. Currently, only used by Questions, when the student edits their answer.
    ///
    /// "STUDENT_EDITED_AFTER_TURN_IN"
    #[serde(rename="STUDENT_EDITED_AFTER_TURN_IN")]
    STUDENTEDITEDAFTERTURNIN,
}

impl AsRef<str> for StateHistoryStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StateHistoryStateEnum::STATEUNSPECIFIED => "STATE_UNSPECIFIED",
            StateHistoryStateEnum::CREATED => "CREATED",
            StateHistoryStateEnum::TURNEDIN => "TURNED_IN",
            StateHistoryStateEnum::RETURNED => "RETURNED",
            StateHistoryStateEnum::RECLAIMEDBYSTUDENT => "RECLAIMED_BY_STUDENT",
            StateHistoryStateEnum::STUDENTEDITEDAFTERTURNIN => "STUDENT_EDITED_AFTER_TURN_IN",
        }
    }
}

impl std::convert::TryFrom< &str> for StateHistoryStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STATE_UNSPECIFIED" => Ok(StateHistoryStateEnum::STATEUNSPECIFIED),
           "CREATED" => Ok(StateHistoryStateEnum::CREATED),
           "TURNED_IN" => Ok(StateHistoryStateEnum::TURNEDIN),
           "RETURNED" => Ok(StateHistoryStateEnum::RETURNED),
           "RECLAIMED_BY_STUDENT" => Ok(StateHistoryStateEnum::RECLAIMEDBYSTUDENT),
           "STUDENT_EDITED_AFTER_TURN_IN" => Ok(StateHistoryStateEnum::STUDENTEDITEDAFTERTURNIN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StateHistoryStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StudentSubmissionCourseWorkTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of course work this submission is for. Read-only.
pub enum StudentSubmissionCourseWorkTypeEnum {
    

    /// No work type specified. This is never returned.
    ///
    /// "COURSE_WORK_TYPE_UNSPECIFIED"
    #[serde(rename="COURSE_WORK_TYPE_UNSPECIFIED")]
    COURSEWORKTYPEUNSPECIFIED,
    

    /// An assignment.
    ///
    /// "ASSIGNMENT"
    #[serde(rename="ASSIGNMENT")]
    ASSIGNMENT,
    

    /// A short answer question.
    ///
    /// "SHORT_ANSWER_QUESTION"
    #[serde(rename="SHORT_ANSWER_QUESTION")]
    SHORTANSWERQUESTION,
    

    /// A multiple-choice question.
    ///
    /// "MULTIPLE_CHOICE_QUESTION"
    #[serde(rename="MULTIPLE_CHOICE_QUESTION")]
    MULTIPLECHOICEQUESTION,
}

impl AsRef<str> for StudentSubmissionCourseWorkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StudentSubmissionCourseWorkTypeEnum::COURSEWORKTYPEUNSPECIFIED => "COURSE_WORK_TYPE_UNSPECIFIED",
            StudentSubmissionCourseWorkTypeEnum::ASSIGNMENT => "ASSIGNMENT",
            StudentSubmissionCourseWorkTypeEnum::SHORTANSWERQUESTION => "SHORT_ANSWER_QUESTION",
            StudentSubmissionCourseWorkTypeEnum::MULTIPLECHOICEQUESTION => "MULTIPLE_CHOICE_QUESTION",
        }
    }
}

impl std::convert::TryFrom< &str> for StudentSubmissionCourseWorkTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_WORK_TYPE_UNSPECIFIED" => Ok(StudentSubmissionCourseWorkTypeEnum::COURSEWORKTYPEUNSPECIFIED),
           "ASSIGNMENT" => Ok(StudentSubmissionCourseWorkTypeEnum::ASSIGNMENT),
           "SHORT_ANSWER_QUESTION" => Ok(StudentSubmissionCourseWorkTypeEnum::SHORTANSWERQUESTION),
           "MULTIPLE_CHOICE_QUESTION" => Ok(StudentSubmissionCourseWorkTypeEnum::MULTIPLECHOICEQUESTION),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StudentSubmissionCourseWorkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region StudentSubmissionStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// State of this submission. Read-only.
pub enum StudentSubmissionStateEnum {
    

    /// No state specified. This should never be returned.
    ///
    /// "SUBMISSION_STATE_UNSPECIFIED"
    #[serde(rename="SUBMISSION_STATE_UNSPECIFIED")]
    SUBMISSIONSTATEUNSPECIFIED,
    

    /// The student has never accessed this submission. Attachments are not returned and timestamps is not set.
    ///
    /// "NEW"
    #[serde(rename="NEW")]
    NEW,
    

    /// Has been created.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// Has been turned in to the teacher.
    ///
    /// "TURNED_IN"
    #[serde(rename="TURNED_IN")]
    TURNEDIN,
    

    /// Has been returned to the student.
    ///
    /// "RETURNED"
    #[serde(rename="RETURNED")]
    RETURNED,
    

    /// Student chose to "unsubmit" the assignment.
    ///
    /// "RECLAIMED_BY_STUDENT"
    #[serde(rename="RECLAIMED_BY_STUDENT")]
    RECLAIMEDBYSTUDENT,
}

impl AsRef<str> for StudentSubmissionStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            StudentSubmissionStateEnum::SUBMISSIONSTATEUNSPECIFIED => "SUBMISSION_STATE_UNSPECIFIED",
            StudentSubmissionStateEnum::NEW => "NEW",
            StudentSubmissionStateEnum::CREATED => "CREATED",
            StudentSubmissionStateEnum::TURNEDIN => "TURNED_IN",
            StudentSubmissionStateEnum::RETURNED => "RETURNED",
            StudentSubmissionStateEnum::RECLAIMEDBYSTUDENT => "RECLAIMED_BY_STUDENT",
        }
    }
}

impl std::convert::TryFrom< &str> for StudentSubmissionStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBMISSION_STATE_UNSPECIFIED" => Ok(StudentSubmissionStateEnum::SUBMISSIONSTATEUNSPECIFIED),
           "NEW" => Ok(StudentSubmissionStateEnum::NEW),
           "CREATED" => Ok(StudentSubmissionStateEnum::CREATED),
           "TURNED_IN" => Ok(StudentSubmissionStateEnum::TURNEDIN),
           "RETURNED" => Ok(StudentSubmissionStateEnum::RETURNED),
           "RECLAIMED_BY_STUDENT" => Ok(StudentSubmissionStateEnum::RECLAIMEDBYSTUDENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a StudentSubmissionStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseAnnouncementStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restriction on the `state` of announcements returned. If this argument is left unspecified, the default value is `PUBLISHED`.
pub enum CourseAnnouncementStatesEnum {
    

    /// No state specified. This is never returned.
    ///
    /// "ANNOUNCEMENT_STATE_UNSPECIFIED"
    #[serde(rename="ANNOUNCEMENT_STATE_UNSPECIFIED")]
    ANNOUNCEMENTSTATEUNSPECIFIED,
    

    /// Status for announcement that has been published. This is the default state.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Status for an announcement that is not yet published. Announcement in this state is visible only to course teachers and domain administrators.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Status for announcement that was published but is now deleted. Announcement in this state is visible only to course teachers and domain administrators. Announcement in this state is deleted after some time.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CourseAnnouncementStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseAnnouncementStatesEnum::ANNOUNCEMENTSTATEUNSPECIFIED => "ANNOUNCEMENT_STATE_UNSPECIFIED",
            CourseAnnouncementStatesEnum::PUBLISHED => "PUBLISHED",
            CourseAnnouncementStatesEnum::DRAFT => "DRAFT",
            CourseAnnouncementStatesEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseAnnouncementStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANNOUNCEMENT_STATE_UNSPECIFIED" => Ok(CourseAnnouncementStatesEnum::ANNOUNCEMENTSTATEUNSPECIFIED),
           "PUBLISHED" => Ok(CourseAnnouncementStatesEnum::PUBLISHED),
           "DRAFT" => Ok(CourseAnnouncementStatesEnum::DRAFT),
           "DELETED" => Ok(CourseAnnouncementStatesEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseAnnouncementStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseLateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requested lateness value. If specified, returned student submissions are restricted by the requested value. If unspecified, submissions are returned regardless of `late` value.
pub enum CourseLateEnum {
    

    /// No restriction on submission late values specified.
    ///
    /// "LATE_VALUES_UNSPECIFIED"
    #[serde(rename="LATE_VALUES_UNSPECIFIED")]
    LATEVALUESUNSPECIFIED,
    

    /// Return StudentSubmissions where late is true.
    ///
    /// "LATE_ONLY"
    #[serde(rename="LATE_ONLY")]
    LATEONLY,
    

    /// Return StudentSubmissions where late is false.
    ///
    /// "NOT_LATE_ONLY"
    #[serde(rename="NOT_LATE_ONLY")]
    NOTLATEONLY,
}

impl AsRef<str> for CourseLateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseLateEnum::LATEVALUESUNSPECIFIED => "LATE_VALUES_UNSPECIFIED",
            CourseLateEnum::LATEONLY => "LATE_ONLY",
            CourseLateEnum::NOTLATEONLY => "NOT_LATE_ONLY",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseLateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LATE_VALUES_UNSPECIFIED" => Ok(CourseLateEnum::LATEVALUESUNSPECIFIED),
           "LATE_ONLY" => Ok(CourseLateEnum::LATEONLY),
           "NOT_LATE_ONLY" => Ok(CourseLateEnum::NOTLATEONLY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseLateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Requested submission states. If specified, returned student submissions match one of the specified submission states.
pub enum CourseStatesEnum {
    

    /// No state specified. This should never be returned.
    ///
    /// "SUBMISSION_STATE_UNSPECIFIED"
    #[serde(rename="SUBMISSION_STATE_UNSPECIFIED")]
    SUBMISSIONSTATEUNSPECIFIED,
    

    /// The student has never accessed this submission. Attachments are not returned and timestamps is not set.
    ///
    /// "NEW"
    #[serde(rename="NEW")]
    NEW,
    

    /// Has been created.
    ///
    /// "CREATED"
    #[serde(rename="CREATED")]
    CREATED,
    

    /// Has been turned in to the teacher.
    ///
    /// "TURNED_IN"
    #[serde(rename="TURNED_IN")]
    TURNEDIN,
    

    /// Has been returned to the student.
    ///
    /// "RETURNED"
    #[serde(rename="RETURNED")]
    RETURNED,
    

    /// Student chose to "unsubmit" the assignment.
    ///
    /// "RECLAIMED_BY_STUDENT"
    #[serde(rename="RECLAIMED_BY_STUDENT")]
    RECLAIMEDBYSTUDENT,
}

impl AsRef<str> for CourseStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseStatesEnum::SUBMISSIONSTATEUNSPECIFIED => "SUBMISSION_STATE_UNSPECIFIED",
            CourseStatesEnum::NEW => "NEW",
            CourseStatesEnum::CREATED => "CREATED",
            CourseStatesEnum::TURNEDIN => "TURNED_IN",
            CourseStatesEnum::RETURNED => "RETURNED",
            CourseStatesEnum::RECLAIMEDBYSTUDENT => "RECLAIMED_BY_STUDENT",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SUBMISSION_STATE_UNSPECIFIED" => Ok(CourseStatesEnum::SUBMISSIONSTATEUNSPECIFIED),
           "NEW" => Ok(CourseStatesEnum::NEW),
           "CREATED" => Ok(CourseStatesEnum::CREATED),
           "TURNED_IN" => Ok(CourseStatesEnum::TURNEDIN),
           "RETURNED" => Ok(CourseStatesEnum::RETURNED),
           "RECLAIMED_BY_STUDENT" => Ok(CourseStatesEnum::RECLAIMEDBYSTUDENT),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseCourseWorkStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restriction on the work status to return. Only courseWork that matches is returned. If unspecified, items with a work status of `PUBLISHED` is returned.
pub enum CourseCourseWorkStatesEnum {
    

    /// No state specified. This is never returned.
    ///
    /// "COURSE_WORK_STATE_UNSPECIFIED"
    #[serde(rename="COURSE_WORK_STATE_UNSPECIFIED")]
    COURSEWORKSTATEUNSPECIFIED,
    

    /// Status for work that has been published. This is the default state.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Status for work that is not yet published. Work in this state is visible only to course teachers and domain administrators.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Status for work that was published but is now deleted. Work in this state is visible only to course teachers and domain administrators. Work in this state is deleted after some time.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CourseCourseWorkStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseCourseWorkStatesEnum::COURSEWORKSTATEUNSPECIFIED => "COURSE_WORK_STATE_UNSPECIFIED",
            CourseCourseWorkStatesEnum::PUBLISHED => "PUBLISHED",
            CourseCourseWorkStatesEnum::DRAFT => "DRAFT",
            CourseCourseWorkStatesEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseCourseWorkStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_WORK_STATE_UNSPECIFIED" => Ok(CourseCourseWorkStatesEnum::COURSEWORKSTATEUNSPECIFIED),
           "PUBLISHED" => Ok(CourseCourseWorkStatesEnum::PUBLISHED),
           "DRAFT" => Ok(CourseCourseWorkStatesEnum::DRAFT),
           "DELETED" => Ok(CourseCourseWorkStatesEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseCourseWorkStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseCourseWorkMaterialStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restriction on the work status to return. Only course work material that matches is returned. If unspecified, items with a work status of `PUBLISHED` is returned.
pub enum CourseCourseWorkMaterialStatesEnum {
    

    /// No state specified. This is never returned.
    ///
    /// "COURSEWORK_MATERIAL_STATE_UNSPECIFIED"
    #[serde(rename="COURSEWORK_MATERIAL_STATE_UNSPECIFIED")]
    COURSEWORKMATERIALSTATEUNSPECIFIED,
    

    /// Status for course work material that has been published. This is the default state.
    ///
    /// "PUBLISHED"
    #[serde(rename="PUBLISHED")]
    PUBLISHED,
    

    /// Status for a course work material that is not yet published. Course work material in this state is visible only to course teachers and domain administrators.
    ///
    /// "DRAFT"
    #[serde(rename="DRAFT")]
    DRAFT,
    

    /// Status for course work material that was published but is now deleted. Course work material in this state is visible only to course teachers and domain administrators. Course work material in this state is deleted after some time.
    ///
    /// "DELETED"
    #[serde(rename="DELETED")]
    DELETED,
}

impl AsRef<str> for CourseCourseWorkMaterialStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseCourseWorkMaterialStatesEnum::COURSEWORKMATERIALSTATEUNSPECIFIED => "COURSEWORK_MATERIAL_STATE_UNSPECIFIED",
            CourseCourseWorkMaterialStatesEnum::PUBLISHED => "PUBLISHED",
            CourseCourseWorkMaterialStatesEnum::DRAFT => "DRAFT",
            CourseCourseWorkMaterialStatesEnum::DELETED => "DELETED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseCourseWorkMaterialStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSEWORK_MATERIAL_STATE_UNSPECIFIED" => Ok(CourseCourseWorkMaterialStatesEnum::COURSEWORKMATERIALSTATEUNSPECIFIED),
           "PUBLISHED" => Ok(CourseCourseWorkMaterialStatesEnum::PUBLISHED),
           "DRAFT" => Ok(CourseCourseWorkMaterialStatesEnum::DRAFT),
           "DELETED" => Ok(CourseCourseWorkMaterialStatesEnum::DELETED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseCourseWorkMaterialStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region CourseCourseStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restricts returned courses to those in one of the specified states The default value is ACTIVE, ARCHIVED, PROVISIONED, DECLINED.
pub enum CourseCourseStatesEnum {
    

    /// No course state. No returned Course message will use this value.
    ///
    /// "COURSE_STATE_UNSPECIFIED"
    #[serde(rename="COURSE_STATE_UNSPECIFIED")]
    COURSESTATEUNSPECIFIED,
    

    /// The course is active.
    ///
    /// "ACTIVE"
    #[serde(rename="ACTIVE")]
    ACTIVE,
    

    /// The course has been archived. You cannot modify it except to change it to a different state.
    ///
    /// "ARCHIVED"
    #[serde(rename="ARCHIVED")]
    ARCHIVED,
    

    /// The course has been created, but not yet activated. It is accessible by the primary teacher and domain administrators, who may modify it or change it to the `ACTIVE` or `DECLINED` states. A course may only be changed to `PROVISIONED` if it is in the `DECLINED` state.
    ///
    /// "PROVISIONED"
    #[serde(rename="PROVISIONED")]
    PROVISIONED,
    

    /// The course has been created, but declined. It is accessible by the course owner and domain administrators, though it will not be displayed in the web UI. You cannot modify the course except to change it to the `PROVISIONED` state. A course may only be changed to `DECLINED` if it is in the `PROVISIONED` state.
    ///
    /// "DECLINED"
    #[serde(rename="DECLINED")]
    DECLINED,
    

    /// The course has been suspended. You cannot modify the course, and only the user identified by the `owner_id` can view the course. A course may be placed in this state if it potentially violates the Terms of Service.
    ///
    /// "SUSPENDED"
    #[serde(rename="SUSPENDED")]
    SUSPENDED,
}

impl AsRef<str> for CourseCourseStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CourseCourseStatesEnum::COURSESTATEUNSPECIFIED => "COURSE_STATE_UNSPECIFIED",
            CourseCourseStatesEnum::ACTIVE => "ACTIVE",
            CourseCourseStatesEnum::ARCHIVED => "ARCHIVED",
            CourseCourseStatesEnum::PROVISIONED => "PROVISIONED",
            CourseCourseStatesEnum::DECLINED => "DECLINED",
            CourseCourseStatesEnum::SUSPENDED => "SUSPENDED",
        }
    }
}

impl std::convert::TryFrom< &str> for CourseCourseStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "COURSE_STATE_UNSPECIFIED" => Ok(CourseCourseStatesEnum::COURSESTATEUNSPECIFIED),
           "ACTIVE" => Ok(CourseCourseStatesEnum::ACTIVE),
           "ARCHIVED" => Ok(CourseCourseStatesEnum::ARCHIVED),
           "PROVISIONED" => Ok(CourseCourseStatesEnum::PROVISIONED),
           "DECLINED" => Ok(CourseCourseStatesEnum::DECLINED),
           "SUSPENDED" => Ok(CourseCourseStatesEnum::SUSPENDED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CourseCourseStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region UserProfileStatesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If specified, only results with the specified `state` values are returned. Otherwise, results with a `state` of `PENDING` are returned.
pub enum UserProfileStatesEnum {
    

    /// Should never be returned.
    ///
    /// "GUARDIAN_INVITATION_STATE_UNSPECIFIED"
    #[serde(rename="GUARDIAN_INVITATION_STATE_UNSPECIFIED")]
    GUARDIANINVITATIONSTATEUNSPECIFIED,
    

    /// The invitation is active and awaiting a response.
    ///
    /// "PENDING"
    #[serde(rename="PENDING")]
    PENDING,
    

    /// The invitation is no longer active. It may have been accepted, declined, withdrawn or it may have expired.
    ///
    /// "COMPLETE"
    #[serde(rename="COMPLETE")]
    COMPLETE,
}

impl AsRef<str> for UserProfileStatesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            UserProfileStatesEnum::GUARDIANINVITATIONSTATEUNSPECIFIED => "GUARDIAN_INVITATION_STATE_UNSPECIFIED",
            UserProfileStatesEnum::PENDING => "PENDING",
            UserProfileStatesEnum::COMPLETE => "COMPLETE",
        }
    }
}

impl std::convert::TryFrom< &str> for UserProfileStatesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "GUARDIAN_INVITATION_STATE_UNSPECIFIED" => Ok(UserProfileStatesEnum::GUARDIANINVITATIONSTATEUNSPECIFIED),
           "PENDING" => Ok(UserProfileStatesEnum::PENDING),
           "COMPLETE" => Ok(UserProfileStatesEnum::COMPLETE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a UserProfileStatesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


