use super::*;



// region AchievementDefinitionAchievementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the achievement.
pub enum AchievementDefinitionAchievementTypeEnum {
    

    /// Achievement is either locked or unlocked.
    ///
    /// "STANDARD"
    #[serde(rename="STANDARD")]
    STANDARD,
    

    /// Achievement is incremental.
    ///
    /// "INCREMENTAL"
    #[serde(rename="INCREMENTAL")]
    INCREMENTAL,
}

impl AsRef<str> for AchievementDefinitionAchievementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementDefinitionAchievementTypeEnum::STANDARD => "STANDARD",
            AchievementDefinitionAchievementTypeEnum::INCREMENTAL => "INCREMENTAL",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementDefinitionAchievementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "STANDARD" => Ok(AchievementDefinitionAchievementTypeEnum::STANDARD),
           "INCREMENTAL" => Ok(AchievementDefinitionAchievementTypeEnum::INCREMENTAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementDefinitionAchievementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AchievementDefinitionInitialStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The initial state of the achievement.
pub enum AchievementDefinitionInitialStateEnum {
    

    /// Achievement is hidden.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// Achievement is revealed.
    ///
    /// "REVEALED"
    #[serde(rename="REVEALED")]
    REVEALED,
    

    /// Achievement is unlocked.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
}

impl AsRef<str> for AchievementDefinitionInitialStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementDefinitionInitialStateEnum::HIDDEN => "HIDDEN",
            AchievementDefinitionInitialStateEnum::REVEALED => "REVEALED",
            AchievementDefinitionInitialStateEnum::UNLOCKED => "UNLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementDefinitionInitialStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HIDDEN" => Ok(AchievementDefinitionInitialStateEnum::HIDDEN),
           "REVEALED" => Ok(AchievementDefinitionInitialStateEnum::REVEALED),
           "UNLOCKED" => Ok(AchievementDefinitionInitialStateEnum::UNLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementDefinitionInitialStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AchievementRevealResponseCurrentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked.
pub enum AchievementRevealResponseCurrentStateEnum {
    

    /// Achievement is revealed.
    ///
    /// "REVEALED"
    #[serde(rename="REVEALED")]
    REVEALED,
    

    /// Achievement is unlocked.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
}

impl AsRef<str> for AchievementRevealResponseCurrentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementRevealResponseCurrentStateEnum::REVEALED => "REVEALED",
            AchievementRevealResponseCurrentStateEnum::UNLOCKED => "UNLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementRevealResponseCurrentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVEALED" => Ok(AchievementRevealResponseCurrentStateEnum::REVEALED),
           "UNLOCKED" => Ok(AchievementRevealResponseCurrentStateEnum::UNLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementRevealResponseCurrentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AchievementUpdateRequestUpdateTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of update being applied.
pub enum AchievementUpdateRequestUpdateTypeEnum {
    

    /// Achievement is revealed.
    ///
    /// "REVEAL"
    #[serde(rename="REVEAL")]
    REVEAL,
    

    /// Achievement is unlocked.
    ///
    /// "UNLOCK"
    #[serde(rename="UNLOCK")]
    UNLOCK,
    

    /// Achievement is incremented.
    ///
    /// "INCREMENT"
    #[serde(rename="INCREMENT")]
    INCREMENT,
    

    /// Achievement progress is set to at least the passed value.
    ///
    /// "SET_STEPS_AT_LEAST"
    #[serde(rename="SET_STEPS_AT_LEAST")]
    SETSTEPSATLEAST,
}

impl AsRef<str> for AchievementUpdateRequestUpdateTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementUpdateRequestUpdateTypeEnum::REVEAL => "REVEAL",
            AchievementUpdateRequestUpdateTypeEnum::UNLOCK => "UNLOCK",
            AchievementUpdateRequestUpdateTypeEnum::INCREMENT => "INCREMENT",
            AchievementUpdateRequestUpdateTypeEnum::SETSTEPSATLEAST => "SET_STEPS_AT_LEAST",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementUpdateRequestUpdateTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVEAL" => Ok(AchievementUpdateRequestUpdateTypeEnum::REVEAL),
           "UNLOCK" => Ok(AchievementUpdateRequestUpdateTypeEnum::UNLOCK),
           "INCREMENT" => Ok(AchievementUpdateRequestUpdateTypeEnum::INCREMENT),
           "SET_STEPS_AT_LEAST" => Ok(AchievementUpdateRequestUpdateTypeEnum::SETSTEPSATLEAST),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementUpdateRequestUpdateTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AchievementUpdateResponseCurrentStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The current state of the achievement.
pub enum AchievementUpdateResponseCurrentStateEnum {
    

    /// Achievement is hidden.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// Achievement is revealed.
    ///
    /// "REVEALED"
    #[serde(rename="REVEALED")]
    REVEALED,
    

    /// Achievement is unlocked.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
}

impl AsRef<str> for AchievementUpdateResponseCurrentStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementUpdateResponseCurrentStateEnum::HIDDEN => "HIDDEN",
            AchievementUpdateResponseCurrentStateEnum::REVEALED => "REVEALED",
            AchievementUpdateResponseCurrentStateEnum::UNLOCKED => "UNLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementUpdateResponseCurrentStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HIDDEN" => Ok(AchievementUpdateResponseCurrentStateEnum::HIDDEN),
           "REVEALED" => Ok(AchievementUpdateResponseCurrentStateEnum::REVEALED),
           "UNLOCKED" => Ok(AchievementUpdateResponseCurrentStateEnum::UNLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementUpdateResponseCurrentStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationEnabledFeaturesEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A list of features that have been enabled for the application.
pub enum ApplicationEnabledFeaturesEnum {
    

    /// Saved Games (snapshots).
    ///
    /// "SNAPSHOTS"
    #[serde(rename="SNAPSHOTS")]
    SNAPSHOTS,
}

impl AsRef<str> for ApplicationEnabledFeaturesEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationEnabledFeaturesEnum::SNAPSHOTS => "SNAPSHOTS",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationEnabledFeaturesEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SNAPSHOTS" => Ok(ApplicationEnabledFeaturesEnum::SNAPSHOTS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationEnabledFeaturesEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventBatchRecordFailureFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The cause for the update failure.
pub enum EventBatchRecordFailureFailureCauseEnum {
    

    /// A batch request was issued with more events than are allowed in a single batch.
    ///
    /// "TOO_LARGE"
    #[serde(rename="TOO_LARGE")]
    TOOLARGE,
    

    /// A batch was sent with data too far in the past to record.
    ///
    /// "TIME_PERIOD_EXPIRED"
    #[serde(rename="TIME_PERIOD_EXPIRED")]
    TIMEPERIODEXPIRED,
    

    /// A batch was sent with a time range that was too short.
    ///
    /// "TIME_PERIOD_SHORT"
    #[serde(rename="TIME_PERIOD_SHORT")]
    TIMEPERIODSHORT,
    

    /// A batch was sent with a time range that was too long.
    ///
    /// "TIME_PERIOD_LONG"
    #[serde(rename="TIME_PERIOD_LONG")]
    TIMEPERIODLONG,
    

    /// An attempt was made to record a batch of data which was already seen.
    ///
    /// "ALREADY_UPDATED"
    #[serde(rename="ALREADY_UPDATED")]
    ALREADYUPDATED,
    

    /// An attempt was made to record data faster than the server will apply updates.
    ///
    /// "RECORD_RATE_HIGH"
    #[serde(rename="RECORD_RATE_HIGH")]
    RECORDRATEHIGH,
}

impl AsRef<str> for EventBatchRecordFailureFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventBatchRecordFailureFailureCauseEnum::TOOLARGE => "TOO_LARGE",
            EventBatchRecordFailureFailureCauseEnum::TIMEPERIODEXPIRED => "TIME_PERIOD_EXPIRED",
            EventBatchRecordFailureFailureCauseEnum::TIMEPERIODSHORT => "TIME_PERIOD_SHORT",
            EventBatchRecordFailureFailureCauseEnum::TIMEPERIODLONG => "TIME_PERIOD_LONG",
            EventBatchRecordFailureFailureCauseEnum::ALREADYUPDATED => "ALREADY_UPDATED",
            EventBatchRecordFailureFailureCauseEnum::RECORDRATEHIGH => "RECORD_RATE_HIGH",
        }
    }
}

impl std::convert::TryFrom< &str> for EventBatchRecordFailureFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "TOO_LARGE" => Ok(EventBatchRecordFailureFailureCauseEnum::TOOLARGE),
           "TIME_PERIOD_EXPIRED" => Ok(EventBatchRecordFailureFailureCauseEnum::TIMEPERIODEXPIRED),
           "TIME_PERIOD_SHORT" => Ok(EventBatchRecordFailureFailureCauseEnum::TIMEPERIODSHORT),
           "TIME_PERIOD_LONG" => Ok(EventBatchRecordFailureFailureCauseEnum::TIMEPERIODLONG),
           "ALREADY_UPDATED" => Ok(EventBatchRecordFailureFailureCauseEnum::ALREADYUPDATED),
           "RECORD_RATE_HIGH" => Ok(EventBatchRecordFailureFailureCauseEnum::RECORDRATEHIGH),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventBatchRecordFailureFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventDefinitionVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The visibility of event being tracked in this definition.
pub enum EventDefinitionVisibilityEnum {
    

    /// This event should be visible to all users.
    ///
    /// "REVEALED"
    #[serde(rename="REVEALED")]
    REVEALED,
    

    /// This event should only be shown to users that have recorded this event at least once.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
}

impl AsRef<str> for EventDefinitionVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventDefinitionVisibilityEnum::REVEALED => "REVEALED",
            EventDefinitionVisibilityEnum::HIDDEN => "HIDDEN",
        }
    }
}

impl std::convert::TryFrom< &str> for EventDefinitionVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "REVEALED" => Ok(EventDefinitionVisibilityEnum::REVEALED),
           "HIDDEN" => Ok(EventDefinitionVisibilityEnum::HIDDEN),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventDefinitionVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region EventRecordFailureFailureCauseEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The cause for the update failure.
pub enum EventRecordFailureFailureCauseEnum {
    

    /// An attempt was made to set an event that was not defined.
    ///
    /// "NOT_FOUND"
    #[serde(rename="NOT_FOUND")]
    NOTFOUND,
    

    /// An attempt was made to increment an event by a non-positive value.
    ///
    /// "INVALID_UPDATE_VALUE"
    #[serde(rename="INVALID_UPDATE_VALUE")]
    INVALIDUPDATEVALUE,
}

impl AsRef<str> for EventRecordFailureFailureCauseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            EventRecordFailureFailureCauseEnum::NOTFOUND => "NOT_FOUND",
            EventRecordFailureFailureCauseEnum::INVALIDUPDATEVALUE => "INVALID_UPDATE_VALUE",
        }
    }
}

impl std::convert::TryFrom< &str> for EventRecordFailureFailureCauseEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NOT_FOUND" => Ok(EventRecordFailureFailureCauseEnum::NOTFOUND),
           "INVALID_UPDATE_VALUE" => Ok(EventRecordFailureFailureCauseEnum::INVALIDUPDATEVALUE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a EventRecordFailureFailureCauseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region InstancePlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The platform type.
pub enum InstancePlatformTypeEnum {
    

    /// Instance is for Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Instance is for iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Instance is for Web App.
    ///
    /// "WEB_APP"
    #[serde(rename="WEB_APP")]
    WEBAPP,
}

impl AsRef<str> for InstancePlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InstancePlatformTypeEnum::ANDROID => "ANDROID",
            InstancePlatformTypeEnum::IOS => "IOS",
            InstancePlatformTypeEnum::WEBAPP => "WEB_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for InstancePlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANDROID" => Ok(InstancePlatformTypeEnum::ANDROID),
           "IOS" => Ok(InstancePlatformTypeEnum::IOS),
           "WEB_APP" => Ok(InstancePlatformTypeEnum::WEBAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InstancePlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LeaderboardOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How scores are ordered.
pub enum LeaderboardOrderEnum {
    

    /// Larger values are better; scores are sorted in descending order
    ///
    /// "LARGER_IS_BETTER"
    #[serde(rename="LARGER_IS_BETTER")]
    LARGERISBETTER,
    

    /// Smaller values are better; scores are sorted in ascending order
    ///
    /// "SMALLER_IS_BETTER"
    #[serde(rename="SMALLER_IS_BETTER")]
    SMALLERISBETTER,
}

impl AsRef<str> for LeaderboardOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LeaderboardOrderEnum::LARGERISBETTER => "LARGER_IS_BETTER",
            LeaderboardOrderEnum::SMALLERISBETTER => "SMALLER_IS_BETTER",
        }
    }
}

impl std::convert::TryFrom< &str> for LeaderboardOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LARGER_IS_BETTER" => Ok(LeaderboardOrderEnum::LARGERISBETTER),
           "SMALLER_IS_BETTER" => Ok(LeaderboardOrderEnum::SMALLERISBETTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LeaderboardOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LeaderboardEntryTimeSpanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time span of this high score.
pub enum LeaderboardEntryTimeSpanEnum {
    

    /// The score is an all-time score.
    ///
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    

    /// The score is a weekly score.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// The score is a daily score.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
}

impl AsRef<str> for LeaderboardEntryTimeSpanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LeaderboardEntryTimeSpanEnum::ALLTIME => "ALL_TIME",
            LeaderboardEntryTimeSpanEnum::WEEKLY => "WEEKLY",
            LeaderboardEntryTimeSpanEnum::DAILY => "DAILY",
        }
    }
}

impl std::convert::TryFrom< &str> for LeaderboardEntryTimeSpanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_TIME" => Ok(LeaderboardEntryTimeSpanEnum::ALLTIME),
           "WEEKLY" => Ok(LeaderboardEntryTimeSpanEnum::WEEKLY),
           "DAILY" => Ok(LeaderboardEntryTimeSpanEnum::DAILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LeaderboardEntryTimeSpanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkPersonaRequestCardinalityConstraintEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Cardinality constraint to observe when linking a persona to a player in the scope of a game.
pub enum LinkPersonaRequestCardinalityConstraintEnum {
    

    /// 1:1 cardinality between in-game personas and Play Games Services players. By the end of the linking operation only one entry for the player and the persona should remain in the scope of the application. Whether a new link is created or not when this constraint is specified is determined by the chosen `ConflictingLinksResolutionPolicy`: * If `KEEP_EXISTING_LINKS` is specified and the provided persona is already linked to a different player, or the player is already linked to a different persona, no new link will be created and the already existing link(s) will remain as is(are). * If `CREATE_NEW_LINK` is specified and the provided persona is already linked to a different player, or the player is already linked to another persona, the older link(s) will be removed in favour of the new link being created.
    ///
    /// "ONE_PERSONA_TO_ONE_PLAYER"
    #[serde(rename="ONE_PERSONA_TO_ONE_PLAYER")]
    ONEPERSONATOONEPLAYER,
}

impl AsRef<str> for LinkPersonaRequestCardinalityConstraintEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkPersonaRequestCardinalityConstraintEnum::ONEPERSONATOONEPLAYER => "ONE_PERSONA_TO_ONE_PLAYER",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkPersonaRequestCardinalityConstraintEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ONE_PERSONA_TO_ONE_PLAYER" => Ok(LinkPersonaRequestCardinalityConstraintEnum::ONEPERSONATOONEPLAYER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkPersonaRequestCardinalityConstraintEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkPersonaRequestConflictingLinksResolutionPolicyEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. Resolution policy to apply when the linking of a persona to a player would result in violating the specified cardinality constraint.
pub enum LinkPersonaRequestConflictingLinksResolutionPolicyEnum {
    

    /// If link(s) between a player and persona already exists which would result in violating the specified `RecallTokensCardinalityConstraint` if the new link was created, keep the already existing link(s). For example, if Persona1-Player1 is already linked in the scope of application1 and a new link Persona1-Player2 is attempted to be created in the scope of application1, then the old link will remain and no new link will be added. Note that if the already existing links do violate the specified policy (which could occur if not all `LinkPersona` calls use the same `RecallTokensCardinalityConstraint`) this policy will leave these violations unresolved; in order to resolve conflicts, the {@link `CREATE_NEW_LINK` policy needs to be used to rewrite links resolving conflicts.
    ///
    /// "KEEP_EXISTING_LINKS"
    #[serde(rename="KEEP_EXISTING_LINKS")]
    KEEPEXISTINGLINKS,
    

    /// If an existing link between a player and persona already exists which would result in violating the specified `RecallTokensCardinalityConstraint` if the new link was created, replace the already existing link(s) with the new link. For example, if Persona1-Player1 is already linked in the scope of application1 and a new link Persona1-Player2 is attempted to be created in the scope of application1, then the old link will be removed and the new link will be added to replace it.
    ///
    /// "CREATE_NEW_LINK"
    #[serde(rename="CREATE_NEW_LINK")]
    CREATENEWLINK,
}

impl AsRef<str> for LinkPersonaRequestConflictingLinksResolutionPolicyEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkPersonaRequestConflictingLinksResolutionPolicyEnum::KEEPEXISTINGLINKS => "KEEP_EXISTING_LINKS",
            LinkPersonaRequestConflictingLinksResolutionPolicyEnum::CREATENEWLINK => "CREATE_NEW_LINK",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkPersonaRequestConflictingLinksResolutionPolicyEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "KEEP_EXISTING_LINKS" => Ok(LinkPersonaRequestConflictingLinksResolutionPolicyEnum::KEEPEXISTINGLINKS),
           "CREATE_NEW_LINK" => Ok(LinkPersonaRequestConflictingLinksResolutionPolicyEnum::CREATENEWLINK),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkPersonaRequestConflictingLinksResolutionPolicyEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LinkPersonaResponseStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Output only. State of a persona linking attempt.
pub enum LinkPersonaResponseStateEnum {
    

    /// The link specified in the request was created.
    ///
    /// "LINK_CREATED"
    #[serde(rename="LINK_CREATED")]
    LINKCREATED,
    

    /// The link specified in the request was not created because already existing links would result in the new link violating the specified `RecallTokensCardinalityConstraint` if created.
    ///
    /// "PERSONA_OR_PLAYER_ALREADY_LINKED"
    #[serde(rename="PERSONA_OR_PLAYER_ALREADY_LINKED")]
    PERSONAORPLAYERALREADYLINKED,
}

impl AsRef<str> for LinkPersonaResponseStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LinkPersonaResponseStateEnum::LINKCREATED => "LINK_CREATED",
            LinkPersonaResponseStateEnum::PERSONAORPLAYERALREADYLINKED => "PERSONA_OR_PLAYER_ALREADY_LINKED",
        }
    }
}

impl std::convert::TryFrom< &str> for LinkPersonaResponseStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "LINK_CREATED" => Ok(LinkPersonaResponseStateEnum::LINKCREATED),
           "PERSONA_OR_PLAYER_ALREADY_LINKED" => Ok(LinkPersonaResponseStateEnum::PERSONAORPLAYERALREADYLINKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LinkPersonaResponseStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlayerFriendStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game.
pub enum PlayerFriendStatusEnum {
    

    /// There is no relationship between the players.
    ///
    /// "NO_RELATIONSHIP"
    #[serde(rename="NO_RELATIONSHIP")]
    NORELATIONSHIP,
    

    /// The player and requester are friends.
    ///
    /// "FRIEND"
    #[serde(rename="FRIEND")]
    FRIEND,
}

impl AsRef<str> for PlayerFriendStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerFriendStatusEnum::NORELATIONSHIP => "NO_RELATIONSHIP",
            PlayerFriendStatusEnum::FRIEND => "FRIEND",
        }
    }
}

impl std::convert::TryFrom< &str> for PlayerFriendStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NO_RELATIONSHIP" => Ok(PlayerFriendStatusEnum::NORELATIONSHIP),
           "FRIEND" => Ok(PlayerFriendStatusEnum::FRIEND),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlayerFriendStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlayerAchievementAchievementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The state of the achievement.
pub enum PlayerAchievementAchievementStateEnum {
    

    /// Achievement is hidden.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// Achievement is revealed.
    ///
    /// "REVEALED"
    #[serde(rename="REVEALED")]
    REVEALED,
    

    /// Achievement is unlocked.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
}

impl AsRef<str> for PlayerAchievementAchievementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerAchievementAchievementStateEnum::HIDDEN => "HIDDEN",
            PlayerAchievementAchievementStateEnum::REVEALED => "REVEALED",
            PlayerAchievementAchievementStateEnum::UNLOCKED => "UNLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for PlayerAchievementAchievementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "HIDDEN" => Ok(PlayerAchievementAchievementStateEnum::HIDDEN),
           "REVEALED" => Ok(PlayerAchievementAchievementStateEnum::REVEALED),
           "UNLOCKED" => Ok(PlayerAchievementAchievementStateEnum::UNLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlayerAchievementAchievementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlayerLeaderboardScoreTimeSpanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time span of this score.
pub enum PlayerLeaderboardScoreTimeSpanEnum {
    

    /// The score is an all-time score.
    ///
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    

    /// The score is a weekly score.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// The score is a daily score.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
}

impl AsRef<str> for PlayerLeaderboardScoreTimeSpanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerLeaderboardScoreTimeSpanEnum::ALLTIME => "ALL_TIME",
            PlayerLeaderboardScoreTimeSpanEnum::WEEKLY => "WEEKLY",
            PlayerLeaderboardScoreTimeSpanEnum::DAILY => "DAILY",
        }
    }
}

impl std::convert::TryFrom< &str> for PlayerLeaderboardScoreTimeSpanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_TIME" => Ok(PlayerLeaderboardScoreTimeSpanEnum::ALLTIME),
           "WEEKLY" => Ok(PlayerLeaderboardScoreTimeSpanEnum::WEEKLY),
           "DAILY" => Ok(PlayerLeaderboardScoreTimeSpanEnum::DAILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlayerLeaderboardScoreTimeSpanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlayerScoreTimeSpanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time span for this player score.
pub enum PlayerScoreTimeSpanEnum {
    

    /// The score is an all-time score.
    ///
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    

    /// The score is a weekly score.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// The score is a daily score.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
}

impl AsRef<str> for PlayerScoreTimeSpanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerScoreTimeSpanEnum::ALLTIME => "ALL_TIME",
            PlayerScoreTimeSpanEnum::WEEKLY => "WEEKLY",
            PlayerScoreTimeSpanEnum::DAILY => "DAILY",
        }
    }
}

impl std::convert::TryFrom< &str> for PlayerScoreTimeSpanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_TIME" => Ok(PlayerScoreTimeSpanEnum::ALLTIME),
           "WEEKLY" => Ok(PlayerScoreTimeSpanEnum::WEEKLY),
           "DAILY" => Ok(PlayerScoreTimeSpanEnum::DAILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlayerScoreTimeSpanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlayerScoreResponseBeatenScoreTimeSpansEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The time spans where the submitted score is better than the existing score for that time span.
pub enum PlayerScoreResponseBeatenScoreTimeSpansEnum {
    

    /// The score is an all-time score.
    ///
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    

    /// The score is a weekly score.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// The score is a daily score.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
}

impl AsRef<str> for PlayerScoreResponseBeatenScoreTimeSpansEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerScoreResponseBeatenScoreTimeSpansEnum::ALLTIME => "ALL_TIME",
            PlayerScoreResponseBeatenScoreTimeSpansEnum::WEEKLY => "WEEKLY",
            PlayerScoreResponseBeatenScoreTimeSpansEnum::DAILY => "DAILY",
        }
    }
}

impl std::convert::TryFrom< &str> for PlayerScoreResponseBeatenScoreTimeSpansEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_TIME" => Ok(PlayerScoreResponseBeatenScoreTimeSpansEnum::ALLTIME),
           "WEEKLY" => Ok(PlayerScoreResponseBeatenScoreTimeSpansEnum::WEEKLY),
           "DAILY" => Ok(PlayerScoreResponseBeatenScoreTimeSpansEnum::DAILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlayerScoreResponseBeatenScoreTimeSpansEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ProfileSettingFriendsListVisibilityEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProfileSettingFriendsListVisibilityEnum {
    

    /// The friends list is currently visible to the game.
    ///
    /// "VISIBLE"
    #[serde(rename="VISIBLE")]
    VISIBLE,
    

    /// The developer does not have access to the friends list, but can call the Android API to show a consent dialog.
    ///
    /// "REQUEST_REQUIRED"
    #[serde(rename="REQUEST_REQUIRED")]
    REQUESTREQUIRED,
    

    /// The friends list is currently unavailable for this user, and it is not possible to request access at this time, either because the user has permanently declined or the friends feature is not available to them. In this state, any attempts to request access to the friends list will be unsuccessful.
    ///
    /// "UNAVAILABLE"
    #[serde(rename="UNAVAILABLE")]
    UNAVAILABLE,
}

impl AsRef<str> for ProfileSettingFriendsListVisibilityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ProfileSettingFriendsListVisibilityEnum::VISIBLE => "VISIBLE",
            ProfileSettingFriendsListVisibilityEnum::REQUESTREQUIRED => "REQUEST_REQUIRED",
            ProfileSettingFriendsListVisibilityEnum::UNAVAILABLE => "UNAVAILABLE",
        }
    }
}

impl std::convert::TryFrom< &str> for ProfileSettingFriendsListVisibilityEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "VISIBLE" => Ok(ProfileSettingFriendsListVisibilityEnum::VISIBLE),
           "REQUEST_REQUIRED" => Ok(ProfileSettingFriendsListVisibilityEnum::REQUESTREQUIRED),
           "UNAVAILABLE" => Ok(ProfileSettingFriendsListVisibilityEnum::UNAVAILABLE),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ProfileSettingFriendsListVisibilityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region RevisionCheckResponseRevisionStatusEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The result of the revision check.
pub enum RevisionCheckResponseRevisionStatusEnum {
    

    /// The revision being used is current.
    ///
    /// "OK"
    #[serde(rename="OK")]
    OK,
    

    /// There is currently a newer version available, but the revision being used still works.
    ///
    /// "DEPRECATED"
    #[serde(rename="DEPRECATED")]
    DEPRECATED,
    

    /// The revision being used is not supported in any released version.
    ///
    /// "INVALID"
    #[serde(rename="INVALID")]
    INVALID,
}

impl AsRef<str> for RevisionCheckResponseRevisionStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            RevisionCheckResponseRevisionStatusEnum::OK => "OK",
            RevisionCheckResponseRevisionStatusEnum::DEPRECATED => "DEPRECATED",
            RevisionCheckResponseRevisionStatusEnum::INVALID => "INVALID",
        }
    }
}

impl std::convert::TryFrom< &str> for RevisionCheckResponseRevisionStatusEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "OK" => Ok(RevisionCheckResponseRevisionStatusEnum::OK),
           "DEPRECATED" => Ok(RevisionCheckResponseRevisionStatusEnum::DEPRECATED),
           "INVALID" => Ok(RevisionCheckResponseRevisionStatusEnum::INVALID),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a RevisionCheckResponseRevisionStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region SnapshotTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of this snapshot.
pub enum SnapshotTypeEnum {
    

    /// A snapshot representing a save game.
    ///
    /// "SAVE_GAME"
    #[serde(rename="SAVE_GAME")]
    SAVEGAME,
}

impl AsRef<str> for SnapshotTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SnapshotTypeEnum::SAVEGAME => "SAVE_GAME",
        }
    }
}

impl std::convert::TryFrom< &str> for SnapshotTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SAVE_GAME" => Ok(SnapshotTypeEnum::SAVEGAME),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SnapshotTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AchievementStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Tells the server to return only achievements with the specified state. If this parameter isn't specified, all achievements are returned.
pub enum AchievementStateEnum {
    

    /// List all achievements. This is the default.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// List only hidden achievements.
    ///
    /// "HIDDEN"
    #[serde(rename="HIDDEN")]
    HIDDEN,
    

    /// List only revealed achievements.
    ///
    /// "REVEALED"
    #[serde(rename="REVEALED")]
    REVEALED,
    

    /// List only unlocked achievements.
    ///
    /// "UNLOCKED"
    #[serde(rename="UNLOCKED")]
    UNLOCKED,
}

impl AsRef<str> for AchievementStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementStateEnum::ALL => "ALL",
            AchievementStateEnum::HIDDEN => "HIDDEN",
            AchievementStateEnum::REVEALED => "REVEALED",
            AchievementStateEnum::UNLOCKED => "UNLOCKED",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(AchievementStateEnum::ALL),
           "HIDDEN" => Ok(AchievementStateEnum::HIDDEN),
           "REVEALED" => Ok(AchievementStateEnum::REVEALED),
           "UNLOCKED" => Ok(AchievementStateEnum::UNLOCKED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationPlatformTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Restrict application details returned to the specific platform.
pub enum ApplicationPlatformTypeEnum {
    

    /// Retrieve applications that can be played on Android.
    ///
    /// "ANDROID"
    #[serde(rename="ANDROID")]
    ANDROID,
    

    /// Retrieve applications that can be played on iOS.
    ///
    /// "IOS"
    #[serde(rename="IOS")]
    IOS,
    

    /// Retrieve applications that can be played on desktop web.
    ///
    /// "WEB_APP"
    #[serde(rename="WEB_APP")]
    WEBAPP,
}

impl AsRef<str> for ApplicationPlatformTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationPlatformTypeEnum::ANDROID => "ANDROID",
            ApplicationPlatformTypeEnum::IOS => "IOS",
            ApplicationPlatformTypeEnum::WEBAPP => "WEB_APP",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationPlatformTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ANDROID" => Ok(ApplicationPlatformTypeEnum::ANDROID),
           "IOS" => Ok(ApplicationPlatformTypeEnum::IOS),
           "WEB_APP" => Ok(ApplicationPlatformTypeEnum::WEBAPP),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationPlatformTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ApplicationEndPointTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of endpoint being requested.
pub enum ApplicationEndPointTypeEnum {
    

    /// Request a URL to create a new profile.
    ///
    /// "PROFILE_CREATION"
    #[serde(rename="PROFILE_CREATION")]
    PROFILECREATION,
    

    /// Request a URL for the Settings view.
    ///
    /// "PROFILE_SETTINGS"
    #[serde(rename="PROFILE_SETTINGS")]
    PROFILESETTINGS,
}

impl AsRef<str> for ApplicationEndPointTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ApplicationEndPointTypeEnum::PROFILECREATION => "PROFILE_CREATION",
            ApplicationEndPointTypeEnum::PROFILESETTINGS => "PROFILE_SETTINGS",
        }
    }
}

impl std::convert::TryFrom< &str> for ApplicationEndPointTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PROFILE_CREATION" => Ok(ApplicationEndPointTypeEnum::PROFILECREATION),
           "PROFILE_SETTINGS" => Ok(ApplicationEndPointTypeEnum::PROFILESETTINGS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ApplicationEndPointTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region MetagameCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The collection of categories for which data will be returned.
pub enum MetagameCollectionEnum {
    

    /// Retrieve data for all categories. This is the default.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
}

impl AsRef<str> for MetagameCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MetagameCollectionEnum::ALL => "ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for MetagameCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(MetagameCollectionEnum::ALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MetagameCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region PlayerCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Collection of players being retrieved
pub enum PlayerCollectionEnum {
    

    /// Retrieve a list of players that are also playing this game in reverse chronological order.
    ///
    /// "CONNECTED"
    #[serde(rename="CONNECTED")]
    CONNECTED,
    

    /// Retrieve a list of players in the user's social graph that are visible to this game.
    ///
    /// "VISIBLE"
    #[serde(rename="VISIBLE")]
    VISIBLE,
    

    /// Retrieve a list of players who are friends of the user in alphabetical order.
    ///
    /// "FRIENDS_ALL"
    #[serde(rename="FRIENDS_ALL")]
    FRIENDSALL,
}

impl AsRef<str> for PlayerCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlayerCollectionEnum::CONNECTED => "CONNECTED",
            PlayerCollectionEnum::VISIBLE => "VISIBLE",
            PlayerCollectionEnum::FRIENDSALL => "FRIENDS_ALL",
        }
    }
}

impl std::convert::TryFrom< &str> for PlayerCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "CONNECTED" => Ok(PlayerCollectionEnum::CONNECTED),
           "VISIBLE" => Ok(PlayerCollectionEnum::VISIBLE),
           "FRIENDS_ALL" => Ok(PlayerCollectionEnum::FRIENDSALL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlayerCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScoreIncludeRankTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The types of ranks to return. If the parameter is omitted, no ranks will be returned.
pub enum ScoreIncludeRankTypeEnum {
    

    /// Retrieve all supported ranks. In HTTP, this parameter value can also be specified as `ALL`.
    ///
    /// "ALL"
    #[serde(rename="ALL")]
    ALL,
    

    /// Retrieve public ranks, if the player is sharing their gameplay activity publicly.
    ///
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
    

    /// (Obsolete) Retrieve the social rank.
    ///
    /// "SOCIAL"
    #[serde(rename="SOCIAL")]
    SOCIAL,
    

    /// Retrieve the rank on the friends collection.
    ///
    /// "FRIENDS"
    #[serde(rename="FRIENDS")]
    FRIENDS,
}

impl AsRef<str> for ScoreIncludeRankTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScoreIncludeRankTypeEnum::ALL => "ALL",
            ScoreIncludeRankTypeEnum::PUBLIC => "PUBLIC",
            ScoreIncludeRankTypeEnum::SOCIAL => "SOCIAL",
            ScoreIncludeRankTypeEnum::FRIENDS => "FRIENDS",
        }
    }
}

impl std::convert::TryFrom< &str> for ScoreIncludeRankTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL" => Ok(ScoreIncludeRankTypeEnum::ALL),
           "PUBLIC" => Ok(ScoreIncludeRankTypeEnum::PUBLIC),
           "SOCIAL" => Ok(ScoreIncludeRankTypeEnum::SOCIAL),
           "FRIENDS" => Ok(ScoreIncludeRankTypeEnum::FRIENDS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScoreIncludeRankTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScoreTimeSpanEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Required. The time span for the scores and ranks you're requesting.
pub enum ScoreTimeSpanEnum {
    

    /// The score is an all-time score.
    ///
    /// "ALL_TIME"
    #[serde(rename="ALL_TIME")]
    ALLTIME,
    

    /// The score is a weekly score.
    ///
    /// "WEEKLY"
    #[serde(rename="WEEKLY")]
    WEEKLY,
    

    /// The score is a daily score.
    ///
    /// "DAILY"
    #[serde(rename="DAILY")]
    DAILY,
}

impl AsRef<str> for ScoreTimeSpanEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScoreTimeSpanEnum::ALLTIME => "ALL_TIME",
            ScoreTimeSpanEnum::WEEKLY => "WEEKLY",
            ScoreTimeSpanEnum::DAILY => "DAILY",
        }
    }
}

impl std::convert::TryFrom< &str> for ScoreTimeSpanEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ALL_TIME" => Ok(ScoreTimeSpanEnum::ALLTIME),
           "WEEKLY" => Ok(ScoreTimeSpanEnum::WEEKLY),
           "DAILY" => Ok(ScoreTimeSpanEnum::DAILY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScoreTimeSpanEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region ScoreCollectionEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The collection of scores you're requesting.
pub enum ScoreCollectionEnum {
    

    /// List all scores in the public leaderboard.
    ///
    /// "PUBLIC"
    #[serde(rename="PUBLIC")]
    PUBLIC,
    

    /// (Obsolete) Legacy G+ social scores.
    ///
    /// "SOCIAL"
    #[serde(rename="SOCIAL")]
    SOCIAL,
    

    /// List only scores of friends.
    ///
    /// "FRIENDS"
    #[serde(rename="FRIENDS")]
    FRIENDS,
}

impl AsRef<str> for ScoreCollectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ScoreCollectionEnum::PUBLIC => "PUBLIC",
            ScoreCollectionEnum::SOCIAL => "SOCIAL",
            ScoreCollectionEnum::FRIENDS => "FRIENDS",
        }
    }
}

impl std::convert::TryFrom< &str> for ScoreCollectionEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "PUBLIC" => Ok(ScoreCollectionEnum::PUBLIC),
           "SOCIAL" => Ok(ScoreCollectionEnum::SOCIAL),
           "FRIENDS" => Ok(ScoreCollectionEnum::FRIENDS),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ScoreCollectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


