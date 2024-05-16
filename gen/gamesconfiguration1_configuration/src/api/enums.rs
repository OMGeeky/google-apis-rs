use super::*;



// region AchievementConfigurationAchievementTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the achievement.
pub enum AchievementConfigurationAchievementTypeEnum {
    

    /// Default value. This value is unused.
    ///
    /// "ACHIEVEMENT_TYPE_UNSPECIFIED"
    #[serde(rename="ACHIEVEMENT_TYPE_UNSPECIFIED")]
    ACHIEVEMENTTYPEUNSPECIFIED,
    

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

impl AsRef<str> for AchievementConfigurationAchievementTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementConfigurationAchievementTypeEnum::ACHIEVEMENTTYPEUNSPECIFIED => "ACHIEVEMENT_TYPE_UNSPECIFIED",
            AchievementConfigurationAchievementTypeEnum::STANDARD => "STANDARD",
            AchievementConfigurationAchievementTypeEnum::INCREMENTAL => "INCREMENTAL",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementConfigurationAchievementTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "ACHIEVEMENT_TYPE_UNSPECIFIED" => Ok(AchievementConfigurationAchievementTypeEnum::ACHIEVEMENTTYPEUNSPECIFIED),
           "STANDARD" => Ok(AchievementConfigurationAchievementTypeEnum::STANDARD),
           "INCREMENTAL" => Ok(AchievementConfigurationAchievementTypeEnum::INCREMENTAL),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementConfigurationAchievementTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region AchievementConfigurationInitialStateEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The initial state of the achievement.
pub enum AchievementConfigurationInitialStateEnum {
    

    /// Default value. This value is unused.
    ///
    /// "INITIAL_STATE_UNSPECIFIED"
    #[serde(rename="INITIAL_STATE_UNSPECIFIED")]
    INITIALSTATEUNSPECIFIED,
    

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
}

impl AsRef<str> for AchievementConfigurationInitialStateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            AchievementConfigurationInitialStateEnum::INITIALSTATEUNSPECIFIED => "INITIAL_STATE_UNSPECIFIED",
            AchievementConfigurationInitialStateEnum::HIDDEN => "HIDDEN",
            AchievementConfigurationInitialStateEnum::REVEALED => "REVEALED",
        }
    }
}

impl std::convert::TryFrom< &str> for AchievementConfigurationInitialStateEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "INITIAL_STATE_UNSPECIFIED" => Ok(AchievementConfigurationInitialStateEnum::INITIALSTATEUNSPECIFIED),
           "HIDDEN" => Ok(AchievementConfigurationInitialStateEnum::HIDDEN),
           "REVEALED" => Ok(AchievementConfigurationInitialStateEnum::REVEALED),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a AchievementConfigurationInitialStateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region GamesNumberFormatConfigurationNumberFormatTypeEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The formatting for the number.
pub enum GamesNumberFormatConfigurationNumberFormatTypeEnum {
    

    /// Default value. This value is unused.
    ///
    /// "NUMBER_FORMAT_TYPE_UNSPECIFIED"
    #[serde(rename="NUMBER_FORMAT_TYPE_UNSPECIFIED")]
    NUMBERFORMATTYPEUNSPECIFIED,
    

    /// Numbers are formatted to have no digits or fixed number of digits after the decimal point according to locale. An optional custom unit can be added.
    ///
    /// "NUMERIC"
    #[serde(rename="NUMERIC")]
    NUMERIC,
    

    /// Numbers are formatted to hours, minutes and seconds.
    ///
    /// "TIME_DURATION"
    #[serde(rename="TIME_DURATION")]
    TIMEDURATION,
    

    /// Numbers are formatted to currency according to locale.
    ///
    /// "CURRENCY"
    #[serde(rename="CURRENCY")]
    CURRENCY,
}

impl AsRef<str> for GamesNumberFormatConfigurationNumberFormatTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            GamesNumberFormatConfigurationNumberFormatTypeEnum::NUMBERFORMATTYPEUNSPECIFIED => "NUMBER_FORMAT_TYPE_UNSPECIFIED",
            GamesNumberFormatConfigurationNumberFormatTypeEnum::NUMERIC => "NUMERIC",
            GamesNumberFormatConfigurationNumberFormatTypeEnum::TIMEDURATION => "TIME_DURATION",
            GamesNumberFormatConfigurationNumberFormatTypeEnum::CURRENCY => "CURRENCY",
        }
    }
}

impl std::convert::TryFrom< &str> for GamesNumberFormatConfigurationNumberFormatTypeEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "NUMBER_FORMAT_TYPE_UNSPECIFIED" => Ok(GamesNumberFormatConfigurationNumberFormatTypeEnum::NUMBERFORMATTYPEUNSPECIFIED),
           "NUMERIC" => Ok(GamesNumberFormatConfigurationNumberFormatTypeEnum::NUMERIC),
           "TIME_DURATION" => Ok(GamesNumberFormatConfigurationNumberFormatTypeEnum::TIMEDURATION),
           "CURRENCY" => Ok(GamesNumberFormatConfigurationNumberFormatTypeEnum::CURRENCY),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a GamesNumberFormatConfigurationNumberFormatTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


// region LeaderboardConfigurationScoreOrderEnum
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaderboardConfigurationScoreOrderEnum {
    

    /// Default value. This value is unused.
    ///
    /// "SCORE_ORDER_UNSPECIFIED"
    #[serde(rename="SCORE_ORDER_UNSPECIFIED")]
    SCOREORDERUNSPECIFIED,
    

    /// Larger scores posted are ranked higher.
    ///
    /// "LARGER_IS_BETTER"
    #[serde(rename="LARGER_IS_BETTER")]
    LARGERISBETTER,
    

    /// Smaller scores posted are ranked higher.
    ///
    /// "SMALLER_IS_BETTER"
    #[serde(rename="SMALLER_IS_BETTER")]
    SMALLERISBETTER,
}

impl AsRef<str> for LeaderboardConfigurationScoreOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LeaderboardConfigurationScoreOrderEnum::SCOREORDERUNSPECIFIED => "SCORE_ORDER_UNSPECIFIED",
            LeaderboardConfigurationScoreOrderEnum::LARGERISBETTER => "LARGER_IS_BETTER",
            LeaderboardConfigurationScoreOrderEnum::SMALLERISBETTER => "SMALLER_IS_BETTER",
        }
    }
}

impl std::convert::TryFrom< &str> for LeaderboardConfigurationScoreOrderEnum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
           "SCORE_ORDER_UNSPECIFIED" => Ok(LeaderboardConfigurationScoreOrderEnum::SCOREORDERUNSPECIFIED),
           "LARGER_IS_BETTER" => Ok(LeaderboardConfigurationScoreOrderEnum::LARGERISBETTER),
           "SMALLER_IS_BETTER" => Ok(LeaderboardConfigurationScoreOrderEnum::SMALLERISBETTER),
            _=> Err(()),
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LeaderboardConfigurationScoreOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


// endregion


